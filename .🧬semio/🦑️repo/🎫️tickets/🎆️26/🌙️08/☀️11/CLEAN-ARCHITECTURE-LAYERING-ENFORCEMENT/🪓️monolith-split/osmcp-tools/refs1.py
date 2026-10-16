import sys, re, collections
sys.path.insert(0, ".")
from uses import *
files = walk(LIBROOT, ["crate"])
def topmod(f):
    mp = f.modpath
    return mp[1] if len(mp) > 1 else "(root-glue)"
MODS = set(topmod(f) for f in files.values())
print(sorted(MODS))
# explicit crate::X occurrences
rows = collections.defaultdict(lambda: collections.Counter())
for p, f in files.items():
    t = topmod(f)
    for m in re.finditer(r'\bcrate::(\w+)', f.code):
        rows[t][m.group(1)] += 1
    # grouped use crate::{...}
    for m in re.finditer(r'\bcrate::\{', f.code):
        paths,_ = parse_use_tree(f.code, m.start())
        for pp,_ in paths:
            if len(pp) > 1: rows[t][pp[1]] += 1
for t in sorted(rows):
    mods = {k: v for k, v in rows[t].items() if k in MODS and k != t}
    flat = {k: v for k, v in rows[t].items() if k not in MODS}
    print("==", t)
    print("  mods:", dict(sorted(mods.items())))
    print("  flat:", dict(sorted(flat.items())))
