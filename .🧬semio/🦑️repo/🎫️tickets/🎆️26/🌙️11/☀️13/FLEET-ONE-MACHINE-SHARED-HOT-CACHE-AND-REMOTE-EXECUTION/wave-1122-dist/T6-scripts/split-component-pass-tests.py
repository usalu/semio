"""🪓️ Splits the component-pass driver test file along its taxonomy (T6, wave 1122-dist): laws stay, driver runs move into one file per topic, shared stubs into a helper."""
import re, sys, os, hashlib

TESTS = sys.argv[1]
SOURCE = os.path.join(TESTS, "🟦️.ts")
EXPECTED = sys.argv[2]
text = open(SOURCE, encoding="utf8").read()
if hashlib.sha256(text.encode()).hexdigest() != EXPECTED: sys.exit("pre-image changed")
lines = text.split("\n")
imports = [l for l in lines[:40] if l.startswith("import")]
starts = [41, 45, 59, 68, 137, 147, 183, 190, 201, 219, 226, 231, 257, 270, 284, 302, 319, 332, 438, 477, 555, 620, 622, 655, 678, 733, 766, 775, 789, 797, 810, 819, 835, 844, 856, 862, 897, 905, 922, 941, 963, 971, 984, 1019, 1079, 1092, 1101, 1111, 1139, 1158, 1172, 1182, 1184, 1199, 1233, 1246, 1274, 1286, 1291]
assert lines[40].startswith("const artifacts") and lines[1290].startswith("/** 🧩️ Proves")
blocks = {s: "\n".join(lines[s - 1:(starts[i + 1] - 1 if i + 1 < len(starts) else len(lines))]).rstrip("\n") for i, s in enumerate(starts)}
plan = {
    "🎭️stub": [41, 68, 137, 147, 897],
    "": [45, 59, 183, 190, 201, 219, 226, 438, 819, 1101, 1111, 1139, 1182, 1184, 1274, 1286],
    "🌊️streaming": [231, 257, 270, 284, 302, 319],
    "🔁️retry": [332, 477],
    "🔏️seal": [620, 622, 655, 678, 733],
    "🧩️closure": [1291],
    "💥️failures": [766, 775, 789, 810, 835, 844, 856],
    "♻️reclaim": [797],
    "🦀️real": [555, 862],
    "🏃️jobs": [905, 922, 941, 963, 971, 984, 1019, 1079, 1092],
    "🛑️cancellation": [1158, 1172, 1199, 1233, 1246],
}
assert sorted(sum(plan.values(), [])) == starts
stub_exports = ["artifacts", "header", "stubCargo", "stubWorkspace", "runStub", "stubJob"]
pattern = re.compile(r'^import\s*(?:(\w+)\s*)?,?\s*(?:\{([^}]*)\})?\s*from\s*"([^"]+)"(.*)$')

def deeper(spec):
    return "../" + spec if spec.startswith("../") else "../" + spec[2:] if spec.startswith("./") else spec

def code(body):
    out, depth, i, stack = [], 0, 0, []
    while i < len(body):
        c = body[i]
        if stack and stack[-1] == "`":
            if c == "\\": i += 2; continue
            if c == "`": stack.pop()
            elif body.startswith("${", i): stack.append("{"); i += 2; continue
            i += 1; continue
        if c == '"':
            end = body.find('"', i + 1)
            while body[end - 1] == "\\": end = body.find('"', end + 1)
            i = end + 1; continue
        if c == "`": stack.append("`")
        elif c == "{" and stack: stack.append("{")
        elif c == "}" and stack and stack[-1] == "{": stack.pop()
        out.append(c); i += 1
    return "".join(out)

def used(name, body):
    return re.search(r"(?<![\w$])" + re.escape(name) + r"(?![\w$])", code(body)) is not None

def header_for(body, moved):
    out = []
    for line in imports:
        m = pattern.match(line)
        assert m, line
        default, named, spec, rest = m.groups()
        spec = deeper(spec) if moved else spec
        keep_default = default if default and used(default, body) else None
        kept = [item.strip() for item in (named or "").split(",") if item.strip() and used(re.split(r"\s+as\s+", item.strip().removeprefix("type "))[-1], body)]
        if not keep_default and not kept: continue
        clause = ", ".join(([keep_default] if keep_default else []) + (["{ " + ", ".join(kept) + " }"] if kept else []))
        out.append(f'import {clause} from "{spec}"{rest}')
    return out

for folder, ids in plan.items():
    body = "\n\n".join(blocks[s] for s in ids)
    moved = folder != ""
    if moved: body = body.replace('new URL("../', 'new URL("../../')
    if folder == "🎭️stub":
        body = re.sub(r"^(const artifacts|const header|function stubCargo|function stubWorkspace|async function runStub|function stubJob)", r"export \1", body, flags=re.M)
        body = body.replace("mkdirSync(artifacts, { recursive: true });\nexport const header", "mkdirSync(artifacts, { recursive: true });\n/** 🧱️ The component header every stub links. */\nexport const header")
        body = "/** 📂️ The artifact directory every stub run writes below. */\n" + body
        head = header_for(body, True)
    else:
        head = header_for(body, moved)
        needed = [name for name in stub_exports if used(name, body)]
        if needed: head.append("import { " + ", ".join(needed) + ' } from "' + ("../🎭️stub/🟦️.ts" if moved else "./🎭️stub/🟦️.ts") + '";')
    target = os.path.join(TESTS, folder, "🟦️.ts") if moved else SOURCE
    if moved: os.makedirs(os.path.dirname(target), exist_ok=True)
    content = "\n".join(head) + "\n\n" + body + "\n"
    open(target + ".t6tmp", "w", encoding="utf8").write(content)
for folder in plan:
    target = os.path.join(TESTS, folder, "🟦️.ts") if folder else SOURCE
    os.replace(target + ".t6tmp", target)
    print(folder or "(laws)", hashlib.sha256(open(target, "rb").read()).hexdigest()[:12], sum(1 for _ in open(target, encoding="utf8")))
