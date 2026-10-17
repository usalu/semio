#!/usr/bin/env python3
"""Moves the inline `mod tests { .. }` of the pdf 1.7 base sqlite snapshot root into the file module 🧪️tests/🔬️unit/🦀️.rs."""
import re, sys, os
B = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot"
root = f"{B}/🦀️.rs"
target = f"{B}/🧪️tests/🔬️unit/🦀️.rs"
text = open(root).read()
head = "#[cfg(test)]\nmod tests {\n"
assert text.count(head) == 1
start = text.index(head)
assert text.rstrip().endswith("}")
body = text[start + len(head): text.rstrip().rindex("}")]
assert text[:start].rstrip().endswith("}")
def fix(m):
    macro, path = m.group(1), m.group(2)
    if path.startswith("🧪️tests/"): new = "../" + path[len("🧪️tests/"):]
    else: new = "../../" + path
    return f'{macro}!("{new}")'
lines = []
for line in body.split("\n"):
    lines.append(line[4:] if line.startswith("    ") else line)
out = "\n".join(lines)
out = re.sub(r'(include_str|include_bytes)!\("([^"]+)"\)', fix, out)
out = "//! 🧪️ Unit tests of the PDF 1.7 sqlite snapshot projection, row admission and conformance refusals.\n" + out.rstrip() + "\n"
new_root = text[:start] + '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
if len(sys.argv) > 1 and sys.argv[1] == "apply":
    os.makedirs(os.path.dirname(target), exist_ok=True)
    assert not os.path.exists(target)
    orig = os.environ["F"] + "/r5b-tools/orig/sqlite-snapshot-root.rs"
    if not os.path.exists(orig): open(orig, "w").write(text)
    open(target, "w").write(out)
    open(root, "w").write(new_root)
    print("applied")
else:
    print(len(text), "->", len(new_root), "and", len(out))
