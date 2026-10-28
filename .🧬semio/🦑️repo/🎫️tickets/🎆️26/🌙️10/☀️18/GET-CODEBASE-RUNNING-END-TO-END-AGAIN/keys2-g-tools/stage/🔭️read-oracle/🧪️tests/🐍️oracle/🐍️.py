#!/usr/bin/env python3
"""🐍️ Independent oracle of the read oracle: recomputes the policy, the log parsing and the Landlock grants with plain Python.
usage: 🐍️.py <corpus.json> <probe.json>
The probe holds what the TypeScript side rendered (Seatbelt profile text per policy case, Landlock grants); this script reads the RENDERED TEXT and evaluates it as the kernel would, and parses the log vectors with its own parser. Prints one JSON document."""
import ast, json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
probe = json.load(open(sys.argv[2], encoding="utf-8"))
AMBIENT = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"]


def direct_readable(case):
    """What the policy says in words: the declared files, everything below an ambient tree, the files named like a declared name."""
    trees = AMBIENT + case["ambient"]
    out = []
    for path in corpus["tree"]:
        if path in case["allowed"] or any(path == t or path.startswith(t + "/") for t in trees) or path.rsplit("/", 1)[-1] in case["names"]:
            out.append(path)
    return sorted(out)


def seatbelt_readable(profile, root):
    """Evaluates the rendered Seatbelt text: a deny of the whole root, then allow rules; a file is readable when an allow rule matches it."""
    matchers = []
    for line in profile.splitlines():
        sub = re.fullmatch(r'\(allow file-read-data \(subpath "(.*)"\)\)', line)
        lit = re.fullmatch(r'\(allow file-read-data \(literal "(.*)"\)\)', line)
        rex = re.fullmatch(r'\(allow file-read-data \(regex #"(.*)"\)\)', line)
        if sub:
            base = sub.group(1).replace('\\"', '"').replace("\\\\", "\\")
            matchers.append(lambda p, base=base: p == base or p.startswith(base + "/"))
        elif lit:
            base = lit.group(1).replace('\\"', '"').replace("\\\\", "\\")
            matchers.append(lambda p, base=base: p == base)
        elif rex:
            expression = re.compile(rex.group(1))
            matchers.append(lambda p, expression=expression: expression.fullmatch(p) is not None)
    return sorted(path for path in corpus["tree"] if any(m(root + "/" + path) for m in matchers))


def grants_readable(grants, paths):
    return sorted(path for path in paths if any(path == grant or path.startswith(grant.rstrip("/") + "/") for grant in grants))


def decode(literal):
    out, i, raw = bytearray(), 0, literal
    named = {"n": 10, "t": 9, "r": 13, '"': 34, "\\": 92, "a": 7, "b": 8, "f": 12, "v": 11, "e": 27}
    while i < len(raw):
        c = raw[i]
        if c != "\\":
            out += c.encode("utf-8")
            i += 1
            continue
        i += 1
        if raw[i] == "x":
            out.append(int(raw[i + 1:i + 3], 16))
            i += 3
        elif raw[i] in "01234567":
            j = i
            while j < len(raw) and j < i + 3 and raw[j] in "01234567":
                j += 1
            out.append(int(raw[i:j], 8))
            i = j
        else:
            out.append(named.get(raw[i], ord(raw[i])))
            i += 1
    return out.decode("utf-8")


def seatbelt_denials(log, root):
    found = set()
    for line in log.split("\n"):
        m = re.search(r"deny\(1\) file-read-data (.*)$", line)
        if m and m.group(1).strip().startswith(root + "/"):
            found.add(m.group(1).strip()[len(root) + 1:])
    return sorted(found)


def strace_reads(lines, root, cwd):
    """Second implementation: first collect (pid, call, path, flags, directory) with their result in one pass over a pending table, then classify."""
    pending, calls = {}, []
    for line in lines:
        resumed = re.match(r"^(?:\[pid\s+(\d+)\]\s+|(\d+)\s+)?<\.\.\.\s+(\w+)\s+resumed>.*\)\s+=\s+(-?\d+|\?)(?:\s+(E[A-Z]+))?", line)
        if resumed:
            key = (resumed.group(1) or resumed.group(2) or "", resumed.group(3))
            if key in pending:
                calls.append((pending.pop(key), resumed.group(5), resumed.group(4)))
            continue
        call = re.match(r"^(?:\[pid\s+(\d+)\]\s+|(\d+)\s+)?(openat2|openat|open|execveat|execve)\((.*)$", line)
        if not call:
            continue
        name, rest = call.group(3), call.group(4)
        literal = re.search(r'"((?:[^"\\]|\\.)*)"', rest)
        if not literal:
            continue
        path = decode(literal.group(1))
        base = cwd
        d = re.match(r"^(?:AT_FDCWD|\d+)(?:<([^>]*)>)?", rest)
        if (name.startswith("openat") or name == "execveat") and d and d.group(1):
            base = d.group(1)
        absolute = path if path.startswith("/") else base.rstrip("/") + "/" + path
        flags = rest[rest.index('",') + 2:] if '",' in rest else ""
        reads = name.startswith("exec") or (("O_RDONLY" in flags or "O_RDWR" in flags) and "O_PATH" not in flags and "O_DIRECTORY" not in flags)
        entry = (absolute, reads)
        if "<unfinished ...>" in rest:
            pending[(call.group(1) or call.group(2) or "", name)] = entry
            continue
        done = re.search(r"\)\s+=\s+(-?\d+|\?)(?:\s+(E[A-Z]+))?", rest)
        if done:
            calls.append((entry, done.group(2), done.group(1)))
    opened, denied = set(), set()
    for (absolute, reads), error, result in calls:
        if not reads or not absolute.startswith(root + "/"):
            continue
        relative = absolute[len(root) + 1:]
        if error in ("EACCES", "EPERM"):
            denied.add(relative)
        elif result not in ("-1", "?"):
            opened.add(relative)
    return {"opened": sorted(opened), "denied": sorted(denied)}


answer = {"policy": [], "seatbeltLog": [], "strace": [], "straceStrings": [decode(row["literal"]) for row in corpus["straceStrings"]], "linuxGrants": []}
for case in corpus["policy"]:
    entry = probe["policy"][case["id"]]
    answer["policy"].append({"id": case["id"], "direct": direct_readable(case), "seatbelt": seatbelt_readable(entry["profile"], probe["root"]), "rules": sorted(path for path in corpus["tree"] if any((rule["kind"] == "literal" and path == rule["path"]) or (rule["kind"] == "subpath" and (path == rule["path"] or path.startswith(rule["path"] + "/"))) for rule in entry["rules"]) or any(path == t or path.startswith(t + "/") for t in AMBIENT + case["ambient"]) or path.rsplit("/", 1)[-1] in case["names"])})
for row in corpus["seatbeltLog"]:
    answer["seatbeltLog"].append(seatbelt_denials(row["log"], row["root"]))
for row in corpus["strace"]:
    answer["strace"].append(strace_reads(row["log"], row["root"], row["cwd"]))
for case in corpus["linuxGrants"]:
    files = [f"{directory.rstrip('/')}/{name}" for directory, names in case["tree"].items() for name in names if f"{directory.rstrip('/')}/{name}" not in case["tree"]]
    answer["linuxGrants"].append({"id": case["id"], "readable": grants_readable(probe["linuxGrants"][case["id"]], files), "files": sorted(files)})
print(json.dumps(answer, ensure_ascii=False))
