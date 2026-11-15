import re, json, sys
sys.path.insert(0, ".")
from author_43_root import *
slug = "cargo-bike"
txt = load(slug)
n = 0
for scale in ("50", "100", "200"):
    a, b = view_span(txt, scale, "elevation")
    blk = txt[a:b]
    i = blk.index('"id": "tyre-f"')
    j = blk.index('"d":', i)
    seg = blk[i:j]
    m = re.search(r'\n\s*"solid": \{[^}]*\},?', seg)
    if m:
        seg2 = seg[:m.start()] + seg[m.end():]
        blk = blk[:i] + seg2 + blk[j:]
        json.loads(blk)
        txt = txt[:a] + blk + txt[b:]
        n += 1
        print("removed solid hint at", scale)
json.loads(txt)
save(slug, txt)
print(n)
