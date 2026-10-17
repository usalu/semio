"""🔬 c4p13 method outline: for every depth-0 impl/trait block of a Rust file lists the depth-1 items with their 1-based spans (docs and attributes included). Read only.
usage: meth.py <file> [min-impl-lines]"""
import re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/frs-tools")
from rslex import lex, outline

text = open(sys.argv[1], encoding="utf-8").read()
mn = int(sys.argv[2]) if len(sys.argv) > 2 else 100
L = text.split("\n")
depths, inlit = lex(text)
ITEM = re.compile(r"^\s{4}(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(fn|const|type|struct|enum|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)")
for it in outline(text):
    s, e = it["start"], it["end"]
    if it["kind"] not in ("impl", "trait") or e - s + 1 < mn:
        continue
    hdr = L[s].strip()
    j = s
    while "{" not in L[j]:
        j += 1
    print(f"== {s+1}-{e+1} ({e-s+1}) {hdr[:140]}  [header ends at {j+1}]")
    items = []
    for i in range(j + 1, e):
        if depths[i] == 1 and not inlit[i]:
            m = ITEM.match(L[i])
            if m:
                a = i
                while a - 1 > j and re.match(r"^\s{4}(///|#\[|//)", L[a - 1]):
                    a -= 1
                items.append((a, i, m.group(1), m.group(2)))
    for k, (a, i, kind, name) in enumerate(items):
        end = (items[k + 1][0] - 1) if k + 1 < len(items) else e - 1
        while end > i and L[end].strip() == "":
            end -= 1
        print(f"   {a+1:5}-{end+1:5} {end-a+1:4} {kind:5} {name}")
