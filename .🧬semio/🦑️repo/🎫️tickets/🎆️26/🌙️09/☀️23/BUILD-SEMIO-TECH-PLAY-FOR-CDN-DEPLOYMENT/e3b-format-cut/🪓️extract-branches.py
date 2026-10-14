#!/usr/bin/env python3
"""🪓️ Extracts every branch of one `if … else if …` dispatch chain inside a function into its own function.

The chain is found by a line regex inside the named function. Each branch body becomes
`fn <name>(<the chain's free variables it uses>) -> Result<(), String>` placed right after the function, and the
branch becomes one call `<name>(<arguments>)?;`. Free variables come from the spec's parameter table (name, type,
argument spelling); `&mut x` inside a body whose parameter is already a `&mut` reference is rewritten to `x`. Any other
local of the enclosing function that a body uses is reported and refuses the extraction.

usage: 🪓️extract-branches.py plan|apply <spec.json>
spec: {"file": repo-relative file, "function": name, "chain": line regex of the chain's first `if`,
       "params": [[name, type, argument]...], "names": {branch index: function name}, "doc": {branch index: "emoji text"},
       "default_doc": "emoji text {name}"}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CUT = os.path.join(HERE, "✂️crate-cut.py")
cut = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CUT, "exec"), cut)
strip = cut["strip"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


def match_brace(text, open_at):
    depth = 0
    for j in range(open_at, len(text)):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                return j
    raise SystemExit("unbalanced braces")


def snake(text):
    return re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")


def analyse(spec):
    path = os.path.join(REPO, spec["file"])
    raw = open(path, encoding="utf-8").read()
    text = strip(raw)
    fn = re.search(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?fn\s+" + re.escape(spec["function"]) + r"\b", text)
    fn_open = text.index("{", fn.end())
    fn_close = match_brace(text, fn_open)
    chain = re.compile(spec["chain"], re.M).search(text, fn_open, fn_close)
    if not chain:
        raise SystemExit("chain not found")
    start = chain.start() + len(re.match(r"[ \t]*", text[chain.start():]).group(0))
    branches, pos = [], start
    while True:
        head = re.compile(r"if\s").match(text, pos)
        if not head:
            raise SystemExit("unexpected chain continuation")
        open_at = text.index("{", head.end())
        while True:
            candidate = text[head.end():open_at]
            if candidate.count("(") == candidate.count(")") and candidate.count("[") == candidate.count("]"):
                break
            open_at = text.index("{", open_at + 1)
        close_at = match_brace(text, open_at)
        branches.append({"condition": raw[head.end():open_at].strip(), "body": (open_at + 1, close_at)})
        rest = re.compile(r"\s*else\s+").match(text, close_at + 1)
        if rest and text.startswith("if", rest.end()):
            pos = rest.end()
            continue
        if rest:
            raise SystemExit("trailing else branch is not supported")
        end = close_at + 1
        break
    stack, visible_scopes, scoped = [], None, []
    for j in range(fn_open, start + 1):
        if j == start:
            visible_scopes = set(stack)
            break
        c = text[j]
        if c == "{":
            stack.append(j)
        elif c == "}":
            stack.pop()
        elif text.startswith("let", j) and not (text[j - 1].isalnum() or text[j - 1] == "_") and not (text[j + 3].isalnum() or text[j + 3] == "_"):
            scoped.append((j, stack[-1] if stack else None))
    locals_ = set()
    for j, scope in scoped:
        if scope not in visible_scopes:
            continue
        pattern = re.match(r"let\s+(?:mut\s+)?(\([^=]*\)|" + IDENT + r")", text[j:])
        if pattern:
            locals_ |= set(re.findall(IDENT, pattern.group(1))) - {"mut"}
    params_sig = re.search(r"\(([^)]*)\)", text[fn.end():fn_open])
    if params_sig:
        locals_ |= set(re.findall(r"(" + IDENT + r")\s*:", params_sig.group(1)))
    for loop in re.finditer(r"\bfor\s+\(?([^)]*?)\)?\s+in\b", text[fn_open:start]):
        if fn_open + loop.start() in [k for k in visible_scopes] or True:
            locals_ |= set(re.findall(IDENT, loop.group(1)))
    return path, raw, text, (fn_open, fn_close), start, end, branches, locals_


def plan(spec, quiet=False):
    path, raw, text, (fn_open, fn_close), start, end, branches, locals_ = analyse(spec)
    params = {p[0]: p for p in spec["params"]}
    problems = []
    for index, branch in enumerate(branches):
        a, b = branch["body"]
        used = {m.group(0) for m in re.finditer(r"(?<![\w:.])" + IDENT + r"(?!\s*::)(?!\w)", text[a:b])}
        branch["params"] = [p for p in spec["params"] if p[0] in used]
        body_text = text[a:b]
        bound = set()
        for pattern in re.findall(r"\blet\s+(?:mut\s+)?(\([^=]*?\)|" + IDENT + r")\s*[:=]", body_text):
            bound |= set(re.findall(IDENT, pattern))
        for pattern in re.findall(r"\bfor\s+(\([^)]*\)|" + IDENT + r")\s+in\b", body_text):
            bound |= set(re.findall(IDENT, pattern))
        for pattern in re.findall(r"\|([^|]*)\|", body_text):
            bound |= set(re.findall(IDENT, pattern))
        stray = sorted((used & locals_) - set(params) - bound - {"_", "mut"})
        literal = re.search(r'"([A-Z0-9_]+)"', branch["condition"])
        branch["name"] = spec.get("names", {}).get(str(index)) or (f"decode_{snake(literal.group(1))}_object" if literal else None)
        if stray:
            problems.append(f"branch {index} ({branch['name']}) uses unlisted locals {stray}")
        if not branch["name"]:
            problems.append(f"branch {index} has no name: {branch['condition'][:80]}")
        if not quiet:
            print(f"{index:2} {branch['name']}: {raw[a:b].count(chr(10))} lines, params {[p[0] for p in branch['params']]}")
    for problem in problems:
        print("PROBLEM " + problem)
    return path, raw, text, (fn_open, fn_close), start, end, branches, problems


def apply(spec):
    path, raw, text, (fn_open, fn_close), start, end, branches, problems = plan(spec, quiet=True)
    if problems:
        plan(spec)
        raise SystemExit("refused")
    refs = {p[0] for p in spec["params"] if p[1].startswith("&mut")}
    indent = re.match(r"[ \t]*", raw[raw.rfind("\n", 0, start) + 1:]).group(0)
    calls, functions = [], []
    for index, branch in enumerate(branches):
        a, b = branch["body"]
        body = raw[a:b]
        for name in refs:
            body = re.sub(r"&mut\s+" + re.escape(name) + r"\b(?!\s*[\.\[])", name, body)
        lines = body.strip("\n").split("\n")
        common = min((len(l) - len(l.lstrip()) for l in lines if l.strip()), default=0)
        body = "\n".join(("    " + l[common:]) if l.strip() else "" for l in lines)
        signature = ", ".join(f"{p[0]}: {p[1]}" for p in branch["params"])
        arguments = ", ".join(p[2] for p in branch["params"])
        doc = spec.get("doc", {}).get(str(index)) or spec["default_doc"].format(name=branch["name"], condition=branch["condition"])
        functions.append(f"/// {doc}\nfn {branch['name']}({signature}) -> Result<(), String> {{\n{body}\n    Ok(())\n}}")
        keyword = "if" if index == 0 else "} else if"
        calls.append(f"{keyword} {branch['condition']} {{\n{indent}    {branch['name']}({arguments})?;\n{indent}")
    replacement = "".join(calls) + "}"
    after = fn_close + 1
    new = raw[:start] + replacement + raw[end:after] + "\n\n" + "\n\n".join(functions) + raw[after:]
    open(path, "w", encoding="utf-8").write(new)
    print(f"extracted {len(branches)} branches from {spec['function']}")


if __name__ == "__main__":
    mode, spec_path = sys.argv[1], sys.argv[2]
    spec = json.load(open(spec_path, encoding="utf-8"))
    (plan if mode == "plan" else apply)(spec)
