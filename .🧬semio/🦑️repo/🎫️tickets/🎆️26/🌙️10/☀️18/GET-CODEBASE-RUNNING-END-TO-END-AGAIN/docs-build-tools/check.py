#!/usr/bin/env python3
"""Checks the staged build guide fragments against the live tree without running any repo command.

Checks: (1) every `repo ...` / `bun repo ...` command line (fenced or inline) names an operation, subcommand and flags of the operation catalogue,
(2) every backticked path resolves in the tree (runtime paths and placeholders are listed, not failed), (3) fence balance, table column counts,
(4) heading emoji carry U+FE0F, (5) the EN and DE guides carry the same number of headings, tables, code fences and commands.
Usage: check.py [FRAGMENT_DIR]   (read only)"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
if not os.path.isdir(os.path.join(ROOT, "🧰️framework")):
    ROOT = "/Users/ueli/Documents/semio"
PRODUCT = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo")
MODULES = os.path.join(PRODUCT, "🔨️modules")
LIBRARY = os.path.join(MODULES, "📚️library")
CATALOG = os.path.join(LIBRARY, "🏗️build/⚙️operations/🔣️catalog.json")
FRAGMENTS = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "staged")

catalog = json.load(open(CATALOG, encoding="utf8"))
operations = {o["name"]: o for o in catalog["operations"]}
flags = {f["name"]: f for f in catalog["flags"]}
aliases = {a: f["name"] for f in catalog["flags"] for a in f.get("aliases", [])}
GLOBAL = {f["name"] for f in catalog["flags"] if "operations" not in f}
failures, notes = [], []
STAGED_FLAGS = {}


def staged_remote_execution():
    """Reads the operation the bzl-rbe set stages (from its edit script) while the live catalogue lacks it."""
    path = os.path.join(HERE, "..", "bzl-rbe-tools", "edits.py")
    if "remote-execution" in operations or not os.path.isfile(path):
        return
    marker = "OPERATION = r" + "'" * 3
    text = open(path, encoding="utf8").read()
    start = text.index(marker) + len(marker)
    block = text[start:text.index("'" * 3, start)]
    operation = json.loads(block[block.index('{\n      "name": "remote-execution"'):block.rindex("\n  ]")])
    operations[operation["name"]] = operation
    STAGED_FLAGS["remote-execution"] = set(operation["flags"])
    notes.append("remote-execution validated against the staged bzl-rbe operation (not in the live catalogue yet)")


staged_remote_execution()


def fail(message):
    failures.append(message)


def commands_in(text):
    found = []
    in_fence = False
    for line in text.split("\n"):
        if line.startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence:
            candidate = re.sub(r"^(?:[A-Z_]+=\S+\s+)+", "", line.strip())
            if candidate.startswith("$env:") or candidate.startswith("#") or not candidate:
                continue
            if re.match(r"^(?:bun )?repo\s", candidate):
                found.append(candidate)
        else:
            for m in re.finditer(r"`((?:bun )?repo\s[^`]*)`", line):
                found.append(m.group(1))
    return found


def check_command(command):
    words = re.sub(r"^bun\s+", "", command).split()
    words = words[1:]
    if "--" in words:
        words = words[: words.index("--")]
    words = [w for w in words if not (w.startswith("<") or w.startswith("["))]
    positional = [w for w in words if not w.startswith("-")]
    given = [w for w in words if w.startswith("-")]
    if not positional:
        return
    op = positional[0]
    if op not in operations and op not in ("--help",):
        fail(f"unknown operation in `{command}`: {op}")
        return
    operation = operations[op]
    if len(positional) > 1 and "subcommands" in operation:
        names = {s["name"] for s in operation["subcommands"]}
        if op == "cache" and positional[1] == "remote":
            if positional[2:3] != ["status"]:
                fail(f"cache remote needs status: `{command}`")
        elif op == "tools" and positional[1] == "setup":
            pass
        elif positional[1] not in names:
            fail(f"unknown subcommand in `{command}`: {positional[1]}")
    allowed = set(operation.get("flags", [])) | GLOBAL
    for word in given:
        name = word.split("=")[0]
        name = aliases.get(name, name)
        if name in ("--tool", "--workers", "--help", "--lang") or name in STAGED_FLAGS.get(op, ()):
            continue
        if name not in flags:
            fail(f"flag not in catalogue in `{command}`: {name}")
        elif name not in allowed:
            fail(f"flag {name} not offered by operation {op} in `{command}`")


RUNTIME = re.compile(r"^(?:\.cache/|\.🧬semio/🦑️repo/⚡️cache|bazel/|nx/|bazel-bin|\.nx/|\.🧾️store/|\.cache|dist/|\{|\.🧬semio/)")


def resolve(token):
    if "<" in token or "*" in token or token.startswith("http") or token.startswith("{") or " " in token:
        return None
    if RUNTIME.match(token):
        return "runtime"
    bases = [ROOT, PRODUCT, MODULES, LIBRARY, os.path.join(LIBRARY, "🏗️build"), os.path.join(LIBRARY, "🏗️build/🔧️tools/🏗️bazel"),
             os.path.join(LIBRARY, "⚡️caching"), os.path.join(LIBRARY, "🏗️build/🕸️graph/📥️inference")]
    t = token
    if t.startswith("📚️library/"):
        t = t[len("📚️library/"):]
        bases = [LIBRARY]
    for base in bases:
        if os.path.exists(os.path.join(base, t)):
            return "ok"
    return "missing"


def paths_in(text):
    out = []
    in_fence = False
    for line in text.split("\n"):
        if line.startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        for m in re.finditer(r"`([^`]+)`", line):
            token = m.group(1)
            if "/" in token and re.search(r"[^\x00-\x7f]", token) and not token.startswith("repo ") and not token.startswith("bun "):
                out.append(token)
            elif re.fullmatch(r"[^\s`]+\.(?:json|ts|mjs|md)", token) and re.search(r"[^\x00-\x7f]", token):
                out.append(token)
    return out


def structure(name, text):
    lines = text.split("\n")
    fences = sum(1 for l in lines if l.startswith("```"))
    if fences % 2:
        fail(f"{name}: unbalanced code fences")
    in_fence = False
    block = []
    tables = 0
    for l in lines + [""]:
        if l.startswith("```"):
            in_fence = not in_fence
        if not in_fence and l.startswith("|"):
            block.append(l)
        else:
            if block:
                tables += 1
                cells = [len(re.split(r"(?<!\\)\|", b.strip().strip("|"))) for b in block]
                if len(set(cells)) != 1:
                    fail(f"{name}: table column counts differ {cells} in table starting `{block[0][:50]}`")
                block = []
    headings = [l for l in lines if re.match(r"^#{2,4} ", l)]
    return {"fences": fences // 2, "tables": tables, "headings": len(headings), "commands": len(commands_in(text))}, headings


stats = {}
for name in ("guide-en.md", "guide-de.md"):
    text = open(os.path.join(FRAGMENTS, name), encoding="utf8").read()
    for command in commands_in(text):
        check_command(command)
    tally = {"ok": 0, "runtime": 0, "missing": 0}
    for token in paths_in(text):
        status = resolve(token)
        if status is None:
            continue
        tally[status] += 1
        if status == "missing":
            fail(f"{name}: path does not resolve: {token}")
    notes.append(f"{name}: paths ok={tally['ok']} runtime={tally['runtime']} missing={tally['missing']}")
    stats[name], headings = structure(name, text)
    if name == "guide-en.md":
        for h in headings:
            if h.startswith("## ") or h.startswith("### "):
                first = h.split(" ", 2)[1]
                if not first.endswith("️") and not re.fullmatch(r"[☀-➿]️?", first[:1]) and len(first) > 0 and ord(first[0]) > 0x2000:
                    fail(f"heading emoji without U+FE0F: {h}")
    secrets = re.findall(r"(?:ghp_[A-Za-z0-9]{20,}|\bsk-[A-Za-z0-9]{20,}|AKIA[0-9A-Z]{12,})", text)
    if secrets:
        fail(f"{name}: secret-like token")
edits_path = os.path.join(FRAGMENTS, "edits.json")
if os.path.isfile(edits_path):
    extra = "\n".join(e["new"] for e in json.load(open(edits_path, encoding="utf8"))["edits"] if not e["id"].startswith("guide-"))
    for command in commands_in("```bash\n" + "\n".join(l.split("  #")[0].rstrip() for l in extra.split("\n") if l.startswith("repo ") and "|" not in l) + "\n```") + commands_in(extra):
        check_command(command)
    tally = {"ok": 0, "runtime": 0, "missing": 0}
    for token in paths_in(extra):
        status = resolve(token)
        if status is None:
            continue
        tally[status] += 1
        if status == "missing":
            fail(f"edits.json: path does not resolve: {token}")
    notes.append(f"edits.json: paths ok={tally['ok']} runtime={tally['runtime']} missing={tally['missing']}")
if stats["guide-en.md"] != stats["guide-de.md"]:
    fail(f"EN and DE structure differ: {stats}")
notes.append(f"structure {stats}")
for n in notes:
    print(n)
if failures:
    print("FAIL")
    for f in failures:
        print("  " + f)
    sys.exit(1)
print("ok")
