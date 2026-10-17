#!/usr/bin/env python3
import re, os, glob
B = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot"
root = open(f"{B}/🦀️.rs").read()
mods = re.findall(r'#\[path = "([^"]+)/🦀️\.rs"\]\nmod (\w+);', root)
modmap = {m: p for p, m in mods}
print("mods", list(modmap))
items = re.findall(r'^(?:pub )?(?:fn|struct|const|static) (\w+)', root, re.M)
print("root items", items)
imports = re.findall(r'^use ([^;]+);', root, re.M)
for i in imports: print("use", i[:150])
def idents(text):
    text = re.sub(r'//[^\n]*', '', text)
    text = re.sub(r'"(?:[^"\\]|\\.)*"', '""', text)
    return set(re.findall(r'\b[A-Za-z_][A-Za-z0-9_]*\b', text))
for m, p in modmap.items():
    f = f"{B}/{p}/🦀️.rs"
    t = open(f).read()
    ids = idents(t)
    sib = [s for s in modmap if s in ids and s != m]
    helpers = [i for i in items if i in ids]
    print(f"{m}: size={len(t)} siblings={sib} helpers={helpers}")
