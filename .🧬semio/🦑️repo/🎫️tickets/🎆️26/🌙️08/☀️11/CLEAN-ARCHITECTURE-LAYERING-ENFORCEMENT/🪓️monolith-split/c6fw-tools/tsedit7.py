"""📝 c6fw source-text reader re-points for the split roots of census 6 (3d, actor, job): a text read of a cut root whose asserted text moved into the new children becomes `readModuleTreeSource(<same path>)` (the module tree is the text the unsplit file held); the two publication reads that slice the bridge item read the child that holds it. Each edit asserts its expected match count; the module-tree import is added with a repo-relative path where the file lacks it. usage: tsedit7.py [--apply]"""
import os, re, sys
R = "/Users/ueli/Documents/semio/"
M = "🧰️framework/🔨️modules/"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
TREE = R + "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"
NEWURL = r'(new URL\("%s",import\.meta\.url\))'
def rs(prefix, name="🦀️.rs"):
    return re.escape(prefix + name)
RD = r'readFileSync\(' + NEWURL
EDITS = [
    (M + "🎭️actor/🎟️retained-turn/🎟️supplier/🧪️tests/🟦️.ts", RD % rs("../../../") + r',"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (M + "🎭️actor/🎟️retained-turn/📃️policy/🧪️tests/🟦️.ts", RD % rs("../../../") + r',"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (M + "🎭️actor/🎟️retained-turn/📃️policy/🧪️tests/🟦️.ts", RD % rs("../../../📖️vocabulary/") + r',"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (M + "🎭️actor/🎟️retained-turn/🫴️receiving/🧪️tests/🟦️.ts", RD % rs("../../../") + r',"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (M + "🎭️actor/🧪️tests/📄️checkpoint/🟦️.ts", RD % rs("../../") + r',"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (M + "🎭️actor/🧪️tests/📄️checkpoint/🟦️.ts", re.escape('new URL("../../💼️job/📨️publication/🦀️.rs"'), 'new URL("../../💼️job/📨️publication/🌉️publication-error-and-bridge/🦀️.rs"', 2),
    (M + "🧵️job/♻️retirement/📄️payload/🧪️tests/🟦️.ts", r'source=' + RD % rs("../../../") + r',"utf8"\)', r'source=readModuleTreeSource(\1)', 1),
    (M + "🧵️job/👷️worker/🎟️turn/🤝️receipt/🧪️tests/🟦️.ts", r'source=' + RD % rs("../../../") + r',"utf8"\)', r'source=readModuleTreeSource(\1)', 1),
    (M + "🧵️job/👷️worker/🔀️erasure/🧪️tests/🟦️.ts", r'producer=' + RD % rs("../../../") + r',"utf8"\)', r'producer=readModuleTreeSource(\1)', 1),
    (M + "🧵️job/📬️outcome/🎟️descriptor/🧪️tests/🟦️.ts", r'job=' + RD % rs("../../../") + r',"utf8"\)', r'job=readModuleTreeSource(\1)', 1),
    (M + "🧵️job/📬️outcome/📦️builder/📏️extent/🧪️tests/🟦️.ts", r'job=' + RD % rs("../../../../") + r',"utf8"\)', r'job=readModuleTreeSource(\1)', 1),
    (M + "🧵️job/📬️outcome/🤝️loan/🧪️tests/🟦️.ts", RD % rs("../../../") + r',"utf8"\)(\.includes\("fn borrow_outcome"\))', r'readModuleTreeSource(\1)\2', 1),
    (M + "🧵️job/⏱️context/📦️owner/🧪️tests/🟦️.ts", r'const source=' + RD % rs("../../../") + r',"utf8"\);\n  for\(const method of \["pub retained', r'const source=readModuleTreeSource(\1);\n  for(const method of ["pub retained', 1),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧪️tests/🎟️supplier/🟦️.ts", RD % rs("../../../../../../../../../🔨️modules/🎭️actor/") + r',"utf8"\)', r'readModuleTreeSource(\1)', 1),
    (LIB + "⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts", r'const actor = policyReadFileSafe\(root, ("🧰️framework/🔨️modules/🎭️actor/🦀️\.rs")\);', r'const actor = policyReadModuleTreeSource(root, \1);', 1),
    (LIB + "⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts", r'const jobRuntime = policyReadFileSafe\(root, ("🧰️framework/🔨️modules/🧵️job/🦀️\.rs")\);', r'const jobRuntime = policyReadModuleTreeSource(root, \1);', 1),
    (LIB + "🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts", r'const actor = policyReadFileSafe\((this\.root), ("🧰️framework/🔨️modules/🎭️actor/🦀️\.rs")\);', r'const actor = policyReadModuleTreeSource(\1, \2);', 1),
    (LIB + "🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts", r'const jobRuntime = policyReadFileSafe\((this\.root), ("🧰️framework/🔨️modules/🧵️job/🦀️\.rs")\);', r'const jobRuntime = policyReadModuleTreeSource(\1, \2);', 1),
    (LIB + "🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts", r'const source = `\$\{policyReadFileSafe\((this\.root), ("🧰️framework/🔨️modules/🧵️job/🦀️\.rs")\)\}', r'const source = `${policyReadModuleTreeSource(\1, \2)}', 1),
]
apply = "--apply" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]
if only:
    EDITS = [e for e in EDITS if any(o in e[0] for o in only)]
bad = 0
by_path = {}
for path, rx, rep, n in EDITS:
    by_path.setdefault(path, []).append((rx, rep, n))
for path, edits in by_path.items():
    p = R + path
    text = open(p, encoding="utf-8").read()
    for rx, rep, n in edits:
        found = len(re.findall(rx, text))
        if found != n:
            print("COUNT", found, "!=", n, path.split("/")[-4:-1], rx[:90]); bad += 1; continue
        text = re.sub(rx, rep, text)
        print("ok", n, "x", "/".join(path.split("/")[-4:-1]), rx[:50])
    if "readModuleTreeSource(" in text and "policyReadModuleTreeSource" not in text and not re.search(r"^\s*import\b[^\n]*readModuleTreeSource", text, flags=re.M):
        rel = os.path.relpath(TREE, os.path.dirname(p))
        imp = f'import {{ readModuleTreeSource }} from "{rel}";'
        lines = text.split("\n")
        idx = max((i for i, l in enumerate(lines) if re.match(r"^import\b", l) and l.rstrip().endswith(";")), default=-1)
        lines.insert(idx + 1, imp)
        text = "\n".join(lines)
        print("   + import", rel[-40:])
    if path.startswith(LIB):
        if not re.search(r"policyReadModuleTreeSource[^\n]*from|^import[^\n]*policyReadModuleTreeSource", text, flags=re.M) and "policyReadModuleTreeSource," not in text:
            print("   ! check policyReadModuleTreeSource import in", path.split("/")[-3])
    if apply and not bad:
        open(p, "w", encoding="utf-8").write(text)
print("bad", bad, "applied" if apply and not bad else "dry")
