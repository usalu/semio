#!/usr/bin/env python3
"""Independent checker of the raw-process-spawn law (Python standard library only, cross-platform).

It finds every place a source file starts a process, whatever the program, resolves the program word where it is static, and judges the sites against the tool registry and the allowlist. It shares only JSON data with the TypeScript law (the registry, the allowlist document and the fixtures), never code.
Site: path, line, language, callee, form (spawn | shell | funnel), program (a word, `<self>` or `<dynamic>`), scope (prod | test).
usage:
  python3 oracle.py check     --root ROOT [--files LIST | --git] [--json] [--strict]
  python3 oracle.py inventory --root ROOT [--files LIST | --git] [--outside]
  python3 oracle.py sites     --root ROOT --files LIST      (JSON of every site of the listed files, any scope)
  python3 oracle.py fixtures  FIXTURES.json
check: exit 0 when no finding (and, with --strict, no stale row), 1 otherwise. inventory prints every production site with its disposition."""
import json, os, re, sys

IDENT = re.compile(r"^[A-Za-z_$][\w$]*$")


def mask(text, lang, blank_strings=False):
    out = list(text)
    n = len(text)
    i = 0

    def blank(a, b):
        for k in range(a, b):
            if out[k] != "\n":
                out[k] = " "

    def skip_str(start):
        q = text[start]
        k = start + 1
        while k < n:
            c = text[k]
            if c == "\\":
                k += 2
                continue
            if c == q or (c == "\n" and q != '"'):
                return k + 1
            k += 1
        return n

    def skip_tpl(start):
        k = start + 1
        while k < n:
            c = text[k]
            if c == "\\":
                k += 2
                continue
            if c == "`":
                return k + 1
            if c == "$" and text[k + 1:k + 2] == "{":
                depth = 1
                k += 2
                while k < n and depth > 0:
                    d = text[k]
                    if d == "{":
                        depth += 1
                    elif d == "}":
                        depth -= 1
                    elif d in "\"'":
                        k = skip_str(k)
                        continue
                    elif d == "`":
                        k = skip_tpl(k)
                        continue
                    k += 1
                continue
            k += 1
        return n

    prev = ""
    while i < n:
        c = text[i]
        nx = text[i + 1] if i + 1 < n else ""
        if lang == "python":
            if c == "#":
                e = text.find("\n", i)
                e = n if e < 0 else e
                blank(i, e)
                i = e
                continue
            if c in "\"'":
                if text.startswith(c * 3, i):
                    e = text.find(c * 3, i + 3)
                    stop = n if e < 0 else e + 3
                    blank(i, stop)
                    i = stop
                    continue
                stop = skip_str(i)
                if blank_strings:
                    blank(i + 1, stop - 1)
                i = stop
                continue
            i += 1
            continue
        if c == "/" and nx == "/":
            e = text.find("\n", i)
            e = n if e < 0 else e
            blank(i, e)
            i = e
            continue
        if c == "/" and nx == "*":
            depth, e = 1, i + 2
            while e < n and depth > 0:
                if text.startswith("*/", e):
                    depth -= 1
                    e += 2
                elif lang == "rust" and text.startswith("/*", e):
                    depth += 1
                    e += 2
                else:
                    e += 1
            blank(i, e)
            i = e
            continue
        if lang == "rust" and c == "r" and not re.match(r"\w", prev or " ") and re.match(r'r#*"', text[i:i + 8]):
            h = re.match(r'r(#*)"', text[i:i + 8]).group(1)
            close = '"' + h
            e = text.find(close, i + 2 + len(h))
            stop = n if e < 0 else e + len(close)
            if blank_strings:
                blank(i + 1, stop - 1)
            i = stop
            prev = '"'
            continue
        if lang == "go" and c == "`":
            e = text.find("`", i + 1)
            stop = n if e < 0 else e + 1
            if blank_strings:
                blank(i + 1, stop - 1)
            i = stop
            prev = "`"
            continue
        if lang == "ts" and c == "`":
            stop = skip_tpl(i)
            if blank_strings:
                blank(i + 1, stop - 1)
            i = stop
            prev = "`"
            continue
        if lang in ("rust", "go") and c == "'":
            m = re.match(r"'(?:\\.[^']*|[^'\\])'", text[i:i + 12])
            i += len(m.group(0)) if m else 1
            prev = "'"
            continue
        if c in "\"'":
            if lang in ("rust", "go") and c == "'":
                i += 1
                continue
            stop = skip_str(i)
            if blank_strings:
                blank(i + 1, stop - 1)
            i = stop
            prev = c
            continue
        if lang == "ts" and c == "/":
            sig = prev
            if sig == "" or sig in "(,=:[!&|?{};":
                e, in_class = i + 1, False
                while e < n and text[e] != "\n":
                    d = text[e]
                    if d == "\\":
                        e += 1
                    elif d == "[":
                        in_class = True
                    elif d == "]":
                        in_class = False
                    elif d == "/" and not in_class:
                        break
                    e += 1
                if e < n and text[e] == "/":
                    i = e + 1
                    prev = "/"
                    continue
        if not c.isspace():
            prev = c
        i += 1
    return "".join(out)


STR = re.compile(r"""^(?:"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'|`([^`$]*)`)$""")


def literal(expr):
    m = STR.match(expr.strip())
    return None if m is None else (m.group(1) if m.group(1) is not None else m.group(2) if m.group(2) is not None else m.group(3))


def read_arg(text, start):
    depth, i, n = 0, start, len(text)
    while i < n:
        c = text[i]
        if c in "\"'":
            k = i + 1
            while k < n and text[k] != c and text[k] != "\n":
                if text[k] == "\\":
                    k += 1
                k += 1
            i = k + 1
            continue
        if c == "`":
            k = i + 1
            while k < n and text[k] != "`":
                if text[k] == "\\":
                    k += 1
                k += 1
            i = k + 1
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                break
            depth -= 1
        elif c == "," and depth == 0:
            break
        i += 1
    return text[start:i].strip(), i


def shell_words(line):
    wrappers = {"sudo", "exec", "env", "time", "nice", "command", "nohup", "xargs", "then", "do", "else", "!"}
    out = []
    for seg in re.split(r"&&|\|\||;|\||\$\(|`|\(|\)|\n", line):
        words = seg.strip().split()
        p = 0
        while p < len(words):
            w = words[p]
            if w in wrappers or re.match(r"^[A-Za-z_][A-Za-z0-9_]*=", w):
                p += 1
            elif w.startswith("-") and p > 0 and (words[p - 1] in wrappers or (p > 1 and words[p - 2] == "nice")):
                p += 2 if (w == "-n" and words[p - 1] == "nice") else 1
            else:
                break
        if p < len(words):
            out.append(re.sub(r"^\./(?=[^/]+$)", "", re.sub(r"^[\"']|[\"']$", "", words[p])))
    return out


def resolve(expr, code_masked, lang, selfs):
    e = expr.strip()
    e = re.sub(r"^&", "", e)
    e = re.sub(r"\.to_string\(\)$|\.as_str\(\)$|\.into\(\)$", "", e)
    lit = literal(e)
    if lit is not None:
        return lit
    if e in selfs:
        return "<self>"
    if IDENT.match(e):
        pats = {
            "rust": r"\b(?:const|static|let)\s+(?:mut\s+)?%s\s*(?::[^=]+)?=\s*([^;\n]+)",
            "go": r"\b(?:const|var)?\s*%s\s*(?::=|=)\s*([^\n]+)",
            "ts": r"\b(?:const|let|var)\s+%s\s*(?::[^=]+)?=\s*([^;\n]+)",
            "python": r"^\s*%s\s*=\s*([^\n]+)",
        }
        m = re.search(pats[lang] % re.escape(e), code_masked, re.M)
        if m:
            right = m.group(1).strip()
            if right in selfs:
                return "<self>"
            whole = literal(re.sub(r"\s+as\s+const$", "", right.rstrip(";, ")))
            if whole is not None:
                return whole
    return None


def imports_process(text, cfg):
    names = {}
    for mod in cfg["processModules"]:
        pat = r"""(?:from\s+["']%s["']|require\(\s*["']%s["']\s*\))""" % (re.escape(mod), re.escape(mod))
        for m in re.finditer(r"import\s+(?:type\s+)?([^;]*?)\s+" + pat, text, re.S):
            clause = m.group(1)
            ns = re.search(r"\*\s+as\s+([\w$]+)", clause)
            if ns:
                names[ns.group(1)] = "*"
            default = re.match(r"\s*([\w$]+)\s*(?:,|$)", clause)
            if default and not clause.strip().startswith("{") and not clause.strip().startswith("*"):
                names[default.group(1)] = "*"
            brace = re.search(r"\{([^}]*)\}", clause)
            if brace:
                for part in brace.group(1).split(","):
                    bits = re.split(r"\s+as\s+", part.strip())
                    if bits[0]:
                        names[bits[-1].strip()] = bits[0].strip()
        for m in re.finditer(r"(?:const|let|var)\s+(\{[^}]*\}|[\w$]+)\s*=\s*require\(\s*[\"']%s[\"']\s*\)" % re.escape(mod), text):
            target = m.group(1)
            if target.startswith("{"):
                for part in target.strip("{}").split(","):
                    bits = re.split(r"\s*:\s*", part.strip())
                    if bits[0]:
                        names[bits[-1].strip()] = bits[0].strip()
            else:
                names[target] = "*"
        for m in re.finditer(r"await\s+import\(\s*[\"']%s[\"']\s*\)" % re.escape(mod), text):
            names["<dynamic-import>"] = "*"
    return names


def is_definition(code, start):
    return re.search(r"\b(?:function\*?|func|fn|def)\s+$", code[max(0, start - 16):start]) is not None


def line_starts(text):
    s = [0]
    for i, c in enumerate(text):
        if c == "\n":
            s.append(i + 1)
    return s


def line_of(starts, off):
    lo, hi = 0, len(starts) - 1
    while lo < hi:
        mid = (lo + hi + 1) >> 1
        if starts[mid] <= off:
            lo = mid
        else:
            hi = mid - 1
    return lo + 1


def language_of(path, cfg):
    name = path.rsplit("/", 1)[-1]
    ext = "." + name.rsplit(".", 1)[-1] if "." in name else ""
    for lang, exts in cfg["extensions"].items():
        if ext in exts:
            return lang
    return None


def scope_of(path, cfg):
    segs = path.split("/")
    if any(s in cfg["testSegments"] for s in segs) or re.search(r"\.(?:test|spec)\.[a-z]+$", path) or path.endswith("_test.go"):
        return "test"
    return "prod"


def scan(path, text, cfg):
    lang = language_of(path, cfg)
    if lang is None or lang in ("shell",):
        return scan_shell(path, text, cfg) if lang == "shell" else []
    needles = [c["name"].split(".")[-1].split("::")[0] if c["shape"] != "template" else "$`" for c in cfg["callees"] if c["language"] == lang] + [f["name"] for f in cfg["funnels"] if f["language"] == lang]
    if not any(x in text for x in needles):
        return []
    scope = scope_of(path, cfg)
    masked = mask(text, lang)
    code = mask(text, lang, True)
    starts = line_starts(text)
    test_at = text.find("#[cfg(test)]") if lang == "rust" else -1
    rows = []
    selfs = set(cfg["selfExpressions"])

    def add(callee, form, program, off):
        sc = "test" if test_at >= 0 and off >= test_at else scope
        rows.append({"path": path, "line": line_of(starts, off), "language": lang, "callee": callee, "form": form, "program": program, "scope": sc})

    imported = imports_process(text, cfg) if lang == "ts" else {}
    rust_process = lang == "rust" and re.search(r"\b(?:std|tokio|async_std)::process\b|use\s+std::process::\{?[^;]*\bCommand\b", text) is not None
    rust_clap = lang == "rust" and re.search(r"\bclap\b", text) is not None and not rust_process
    for callee in [c for c in cfg["callees"] if c["language"] == lang]:
        name = callee["name"]
        shape = callee["shape"]
        esc = re.escape(name)
        if shape == "template":
            bare = re.search(r"import\s*\{[^}]*(?<![\w$])\$(?![\w$])[^}]*\}\s*from\s*[\"']bun[\"']", text) is not None
            pat = re.compile(r"(?<![\w$])(?:Bun\.)?\$`" if bare else r"(?<![\w$])Bun\.\$`")
            for m in pat.finditer(code):
                end = masked.find("`", m.end())
                body = masked[m.end(): end if end >= 0 else None]
                words = shell_words(body)
                for w in words:
                    add(name, "shell", w, m.start())
                if not words:
                    add(name, "shell", "<dynamic>", m.start())
            continue
        qualified = "." in name or "::" in name
        if lang == "ts" and not qualified and callee.get("imported", True):
            if imported:
                locals_ = [loc for loc, orig in imported.items() if orig == name or orig == "*"]
                if "<dynamic-import>" in imported:
                    locals_.append("<dynamic-import>")
                direct = [loc for loc, orig in imported.items() if orig == name]
                if not direct and not any(o == "*" for o in imported.values()):
                    continue
                pat = re.compile(r"(?<![\w$.])(?:%s)\s*\(" % "|".join(re.escape(x) for x in direct)) if direct else None
                nspat = re.compile(r"(?<![\w$])(?:%s)\.%s\s*\(" % ("|".join(re.escape(x) for x, o in imported.items() if o == "*" and x != "<dynamic-import>") or "(?!)", esc))
                matches = list(nspat.finditer(code)) + (list(pat.finditer(code)) if pat else [])
                if "<dynamic-import>" in imported:
                    matches += list(re.finditer(r"(?<![\w$])[\w$]+\.%s\s*\(" % esc, code))
            else:
                matches = []
        elif lang == "rust" and name == "Command::new":
            if not rust_process:
                matches = list(re.finditer(r"(?<![\w$:])(?:std::process|tokio::process)::Command::new\s*\(", code))
            else:
                matches = list(re.finditer(r"(?<![\w$])(?:(?:std|tokio)::process::)?Command::new\s*\(", code))
            if rust_clap:
                matches = []
        else:
            matches = list(re.finditer(r"(?<![\w$])%s\s*\(" % esc, code)) if qualified else list(re.finditer(r"(?<![\w$.])%s\s*\(" % esc, code))
            if not qualified:
                matches += [m for m in re.finditer(r"(?<![\w$])[\w$]+\.%s\s*\(" % esc, code) if lang != "ts"]
        for m in matches:
            if is_definition(code, m.start()):
                continue
            open_ = m.end()
            first_text, first_end = read_arg(masked, open_)
            if lang == "rust" and masked[first_end:first_end + 1] != ")":
                continue
            expr, rest = first_text, masked[first_end:first_end + 400]
            if shape in ("array", "optionsCmd"):
                opts = re.match(r"^\{[\s\S]*?\bcmd\s*:\s*\[", first_text)
                if opts:
                    expr, _ = read_arg(first_text, opts.end())
                elif first_text.startswith("["):
                    expr, _ = read_arg(first_text, 1)
                elif lang == "python":
                    d = literal(first_text)
                    if d is not None:
                        words = shell_words(d)
                        for w in words:
                            add(name, "shell", w, m.start())
                    else:
                        add(name, "spawn", "<dynamic>", m.start())
                    continue
                else:
                    add(name, "spawn", "<dynamic>", m.start())
                    continue
            if lang == "go" and name == "exec.CommandContext":
                second_text, second_end = read_arg(masked, first_end + 1)
                expr, rest = second_text, masked[second_end:second_end + 400]
            if shape == "shell-string":
                lit = literal(first_text)
                if lit is None and first_text.startswith("`"):
                    cut = first_text.find("${")
                    lit = first_text[1: len(first_text) - 1 if cut < 0 else cut]
                if lit is not None:
                    words = shell_words(lit)
                    for w in words:
                        add(name, "shell", w, m.start())
                    if not words:
                        add(name, "shell", "<dynamic>", m.start())
                else:
                    add(name, "shell", "<dynamic>", m.start())
                continue
            value = resolve(expr, masked, lang, selfs)
            if value is None:
                add(name, "spawn", "<dynamic>", m.start())
                continue
            if value.split("/")[-1].split("\\")[-1] in cfg["shellWrappers"]:
                flag = re.search(r"""["'](?:-c|-lc|\/c|\/k|-Command|-command)["']\s*,\s*(?:"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'|`([^`]*)`)""", rest)
                if flag:
                    inner = flag.group(1) or flag.group(2) or flag.group(3) or ""
                    words = shell_words(inner)
                    for w in words:
                        add(name, "shell", w, m.start())
                    if words:
                        continue
            add(name, "spawn", value, m.start())
    for funnel in cfg["funnels"]:
        if funnel["language"] != lang:
            continue
        pat = re.compile(r"(?<![\w$.])%s\s*\(" % re.escape(funnel["name"]))
        for m in pat.finditer(code):
            if is_definition(code, m.start()):
                continue
            args, pos = [], m.end()
            for _ in range(funnel["argument"] + 1):
                a, e = read_arg(masked, pos)
                args.append(a)
                pos = e + 1
            if len(args) <= funnel["argument"]:
                continue
            value = resolve(args[funnel["argument"]], masked, lang, selfs)
            add(funnel["name"], "funnel", "<dynamic>" if value is None else value, m.start())
    rows.sort(key=lambda r: (r["line"], r["callee"]))
    return rows


def scan_shell(path, text, cfg):
    rows = []
    scope = scope_of(path, cfg)
    for i, raw in enumerate(re.split(r"\r?\n", text)):
        line = re.sub(r"(^|\s)#.*$", r"\1", raw)
        if not line.strip() or re.match(r"^\s*(?:echo|printf|Write-Host|Write-Output|log|die|warn)\b", line):
            continue
        for w in shell_words(re.sub(r"^\s*(?:if|elif|while|until)\s+", "", line)):
            if re.match(r"^[A-Za-z]", w):
                rows.append({"path": path, "line": i + 1, "language": "shell", "callee": "shell", "form": "shell", "program": w, "scope": scope})
    return rows



REGISTRY = "🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json"
LAW = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn"
ALLOWLIST = LAW + "/📒️allowlist/🔣️.json"
TOOLS_MODULE = "🧰️framework/🔨️modules/🏃️process/🧰️tools/**"


def glob_regex(pattern):
    out, i = "", 0
    while i < len(pattern):
        if pattern[i:i + 3] == "**/":
            out += "(?:.*/)?"
            i += 3
        elif pattern[i:i + 2] == "**":
            out += ".*"
            i += 2
        elif pattern[i] == "*":
            out += "[^/]*"
            i += 1
        else:
            out += re.escape(pattern[i])
            i += 1
    return re.compile("^" + out + "$")


def build_cfg(registry, law):
    scan_ = registry["scan"]
    return {
        "callees": scan_["callees"] + law["extraCallees"],
        "funnels": law["commandFunnels"],
        "shellWrappers": scan_["shellWrappers"],
        "selfExpressions": scan_["selfExpressions"],
        "testSegments": scan_["testSegments"],
        "ignoredSegments": scan_["ignoredSegments"],
        "extensions": scan_["extensions"],
        "processModules": law["processModules"],
    }


def command_name(program):
    return re.sub(r"\.(?:exe|cmd|bat)$", "", program.replace("\\", "/").split("/")[-1], flags=re.I)


def interface_globs(registry, law):
    globs = [TOOLS_MODULE]
    for tool in registry["tools"]:
        globs += tool["adapterScope"] + [tool["adapter"]]
        if tool["provision"].get("installer"):
            globs.append(tool["provision"]["installer"])
    globs += [f["path"] for f in registry.get("funnels", [])]
    globs += [row["glob"] for row in law["interfaces"]]
    return globs


def evaluate(sites, registry, law):
    """Judges the sites: returns (rows, findings, stale). rows carry the disposition of every production site under the roots."""
    registered = {c for t in registry["tools"] for c in t["commands"]}
    owner = {c: t for t in registry["tools"] for c in t["commands"]}
    tool_scope = {t["id"]: [glob_regex(g) for g in t["adapterScope"] + [t["adapter"], TOOLS_MODULE] + ([t["provision"]["installer"]] if t["provision"].get("installer") else [])] for t in registry["tools"]}
    interface = [glob_regex(g) for g in interface_globs(registry, law)]
    budget = {}
    for row in law["exceptions"]:
        budget[(row["path"], row["program"])] = budget.get((row["path"], row["program"]), 0) + row["count"]
    classes = {(row["path"], row["program"]): row["class"] for row in law["exceptions"]}
    left = dict(budget)
    rows, findings = [], []
    roots = tuple(r.rstrip("/") + "/" for r in law["roots"])
    for site in sites:
        if site["scope"] != "prod" or not site["path"].startswith(roots):
            continue
        program = site["program"]
        literal = program not in ("<self>", "<dynamic>")
        name = command_name(program) if literal else ""
        known = literal and name in registered
        inside_interface = any(g.match(site["path"]) for g in interface)
        disposition, rule = None, None
        if literal and not known:
            rule = "unregistered-command"
        elif site["form"] != "funnel":
            if known and any(g.match(site["path"]) for g in tool_scope[owner[name]["id"]]):
                disposition = "adapter"
            elif not known and inside_interface:
                disposition = "interface"
            else:
                rule = "raw-spawn"
        else:
            disposition = "funnel"
        if rule is not None:
            key = (site["path"], program)
            if left.get(key, 0) > 0:
                left[key] -= 1
                disposition = "exception:" + classes[key]
            else:
                findings.append({"rule": rule, "path": site["path"], "line": site["line"], "program": program, "detail": "%s %s of %s outside the interface modules and without an allowlist row" % (site["callee"], site["form"], program)})
                disposition = "FINDING:" + rule
        rows.append({**site, "disposition": disposition})
    stale = ["%s %s %d/%d" % (path, program, budget[(path, program)] - remaining, budget[(path, program)]) for (path, program), remaining in sorted(left.items()) if remaining > 0]
    return rows, findings, stale


def load(root):
    registry = json.load(open(os.path.join(root, REGISTRY), encoding="utf8"))
    law = json.load(open(os.path.join(root, ALLOWLIST), encoding="utf8"))
    return registry, law


def list_files(root, args, law):
    if "--files" in args:
        return [l.rstrip("\n") for l in open(args[args.index("--files") + 1], encoding="utf8")]
    import subprocess
    out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--"] + law["roots"], cwd=root, capture_output=True, check=True).stdout.decode("utf8")
    return [p for p in out.split("\0") if p]


def scan_files(root, files, cfg):
    sites = []
    for path in files:
        if language_of(path, cfg) in (None, "shell") or any(s in cfg["ignoredSegments"] for s in path.split("/")) or path.rsplit("/", 1)[-1].startswith("."):
            continue
        try:
            text = open(os.path.join(root, path), encoding="utf8", errors="replace").read()
        except OSError:
            continue
        if len(text) > 4_000_000:
            continue
        sites.extend(scan(path, text, cfg))
    return sites


def problems(law, registry, files):
    out = []
    for row in law["exceptions"]:
        if not row["reason"].get("en") or not row["reason"].get("de"):
            out.append("exception %s %s has no EN and DE reason" % (row["path"], row["program"]))
    for row in law["interfaces"]:
        if not row["reason"].get("en") or not row["reason"].get("de"):
            out.append("interface %s has no EN and DE reason" % row["glob"])
        if "**" not in row["glob"] and row["glob"] not in files:
            out.append("interface %s matches no file" % row["glob"])
    return out


def fixtures(path):
    document = json.load(open(path, encoding="utf8"))
    failures = []
    cfg = build_cfg(document["registry"], document["law"])
    for case in document["scanCases"]:
        got = [[r["line"], r["callee"], r["form"], r["program"], r["scope"]] for r in scan(case["path"], case["text"], cfg)]
        if got != case["expect"]:
            failures.append("scan %s: expected %s got %s" % (case["name"], case["expect"], got))
    for case in document["globCases"]:
        if bool(glob_regex(case["pattern"]).match(case["path"])) != case["matches"]:
            failures.append("glob %s %s" % (case["pattern"], case["path"]))
    for case in document["lawCases"]:
        sites = [s for f in case["files"] for s in scan(f["path"], f["text"], cfg)]
        law = {**document["law"], "exceptions": case.get("exceptions", []), "interfaces": case.get("interfaces", document["law"]["interfaces"]), "roots": case.get("roots", document["law"]["roots"])}
        rows, findings, stale = evaluate(sites, document["registry"], law)
        got = {"dispositions": [r["disposition"] for r in rows], "findings": sorted("%s %s %s" % (f["rule"], f["path"], f["program"]) for f in findings), "stale": stale}
        if got != case["expect"]:
            failures.append("law %s: expected %s got %s" % (case["name"], case["expect"], got))
    return failures


def main(argv):
    command = argv[0] if argv else "check"
    if command == "fixtures":
        failures = fixtures(argv[1])
        for failure in failures:
            print("FAIL", failure)
        print("fixtures: %s" % ("ok" if not failures else "%d failed" % len(failures)))
        return 1 if failures else 0
    root = os.path.abspath(argv[argv.index("--root") + 1]) if "--root" in argv else os.getcwd()
    registry, law = load(root)
    cfg = build_cfg(registry, law)
    files = list_files(root, argv, law)
    sites = scan_files(root, files, cfg)
    if command == "sites":
        print(json.dumps(sites, ensure_ascii=False))
        return 0
    rows, findings, stale = evaluate(sites, registry, law)
    bad = problems(law, registry, set(files))
    if command == "inventory":
        counts = {}
        for r in rows:
            key = r["disposition"].split(":")[0] if not r["disposition"].startswith("exception") else r["disposition"]
            counts[key] = counts.get(key, 0) + 1
        for key, n in sorted(counts.items(), key=lambda kv: -kv[1]):
            print("%6d %s" % (n, key))
        print("%6d total production sites under the roots, %d files scanned" % (len(rows), len(files)))
        if "--sites" in argv:
            for r in rows:
                print("%s\t%s\t%d\t%s\t%s\t%s" % (r["disposition"], r["path"], r["line"], r["callee"], r["form"], r["program"]))
        return 0
    if "--json" in argv:
        print(json.dumps({"findings": findings, "stale": stale, "problems": bad, "sites": len(rows)}, ensure_ascii=False, indent=1))
    else:
        for f in findings:
            print("[raw-process-spawn] %s %s:%d %s: %s" % (f["rule"], f["path"], f["line"], f["program"], f["detail"]))
        for item in bad:
            print("[raw-process-spawn] allowlist %s" % item)
        for item in stale[:50]:
            print("[raw-process-spawn] stale allowlist row %s" % item)
        print("[raw-process-spawn] %d production sites, %d finding(s), %d stale row(s), %d allowlist problem(s)" % (len(rows), len(findings), len(stale), len(bad)))
    return 1 if findings or bad or ("--strict" in argv and stale) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
