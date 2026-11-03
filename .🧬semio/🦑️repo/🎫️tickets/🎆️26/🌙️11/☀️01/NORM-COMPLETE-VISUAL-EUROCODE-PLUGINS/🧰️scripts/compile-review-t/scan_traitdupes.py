"""🔎 E0119 class: the same `impl Trait for Type` (no generics) written twice in one crate; includes derive(Trait) plus a manual impl of the same trait."""
import os, re, sys
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
root = ART + "/" + CODE[code]
def crate_of(f):
    d = os.path.dirname(f)
    while d.startswith(root):
        if os.path.exists(d + "/Cargo.toml"): return d
        d = os.path.dirname(d)
    return root
seen = {}
derives = {}
ITEM = re.compile(r'((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum)\s+(\w+)')
for f in rs_files(root):
    try: s = strip(open(f, encoding="utf-8").read())
    except Exception: continue
    c = crate_of(f)
    for m in re.finditer(r'\bimpl\s*(?:<[^>]*>)?\s*([A-Z][\w:]*(?:<[^>{]*>)?)\s+for\s+([A-Z][\w:]*)\s*(?:<[^>{]*>)?\s*(?:where[^{]*)?\{', s):
        tr, ty = m.group(1), m.group(2).split("::")[-1]
        key = (c, tr.split("::")[-1], ty)
        seen.setdefault(key, []).append(f"{rel(f)}:{s.count(chr(10), 0, m.start()) + 1}")
    for m in ITEM.finditer(s):
        for d in re.findall(r'derive\(([^)]*)\)', m.group(1)):
            for t in re.findall(r'\w+', d):
                derives.setdefault((c, t, m.group(2)), []).append(f"{rel(f)}")
for k, v in seen.items():
    if len(v) > 1 and "<" not in k[1]: print("dup-impl", k[1], k[2], v)
    if k[1] in ("Default", "Clone", "Debug", "PartialEq", "Eq", "Hash", "Copy", "PartialOrd", "Ord") and (k[0], k[1], k[2]) in derives: print("derive+impl", k[1], k[2], v, derives[k])
