#!/usr/bin/env python3
"""Moves the two inline test modules of the 1.4 base sqlite snapshot into file modules under 🧪️tests."""
import re, sys, os
B = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot"
root = f"{B}/🦀️.rs"
text = open(root).read()
mods = [("tests", "🔬️unit", "Unit tests of the PDF 1.4 sqlite snapshot backing, limits and erased round trips."), ("own14_public_page_domain", "🔬️page-domain", "Page-domain observations of the PDF 1.4 sqlite snapshot against the resolved page corpus.")]
def fix(m):
    macro, path = m.group(1), m.group(2)
    new = "../" + path[len("🧪️tests/"):] if path.startswith("🧪️tests/") else "../../" + path
    return f'{macro}!("{new}")'
out = {}
head_cut = None
for name, folder, doc in mods:
    start = text.index(f"#[cfg(test)]\nmod {name} {{\n")
    open_at = text.index("{\n", start) + 2
    depth = 1; i = open_at
    while depth:
        c = text[i]
        if c == "{": depth += 1
        elif c == "}": depth -= 1
        i += 1
    body = text[open_at:i - 1]
    lines = [l[4:] if l.startswith("    ") else l for l in body.split("\n")]
    b = re.sub(r'(include_str|include_bytes)!\("([^"]+)"\)', fix, "\n".join(lines))
    out[name] = (start, i, f"//! 🧪️ {doc}\n" + b.strip("\n") + "\n", folder)
new_root = text
for name, (s, e, body, folder) in sorted(out.items(), key=lambda kv: -kv[1][0]):
    new_root = new_root[:s] + f'#[cfg(test)]\n#[path = "🧪️tests/{folder}/🦀️.rs"]\nmod {name};' + new_root[e:]
for name, (s, e, body, folder) in out.items():
    r = __import__("subprocess").run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=body, capture_output=True, text=True)
    print(name, "parse", r.returncode)
if len(sys.argv) > 1 and sys.argv[1] == "apply":
    od = os.environ["F"] + "/r5b-tools/orig/sqlite14-root.rs"
    if not os.path.exists(od): open(od, "w").write(text)
    for name, (s, e, body, folder) in out.items():
        p = f"{B}/🧪️tests/{folder}/🦀️.rs"
        os.makedirs(os.path.dirname(p), exist_ok=True)
        assert not os.path.exists(p)
        open(p, "w").write(body)
    open(root, "w").write(new_root)
    print("applied")
