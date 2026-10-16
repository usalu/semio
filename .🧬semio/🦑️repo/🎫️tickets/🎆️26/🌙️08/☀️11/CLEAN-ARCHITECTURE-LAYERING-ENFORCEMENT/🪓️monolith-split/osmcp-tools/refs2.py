import sys, re, collections
sys.path.insert(0, ".")
from prod import *
files = walk(LIBROOT, ["crate"])
def topmod(f):
    mp = f.modpath
    return mp[1] if len(mp) > 1 else "(root-glue)"
MODS = set(topmod(f) for f in files.values())
def refs(codefn):
    rows = collections.defaultdict(lambda: collections.Counter())
    for p, f in files.items():
        t = topmod(f)
        code = codefn(f)
        for m in re.finditer(r'\bcrate::(\w+)', code):
            rows[t][m.group(1)] += 1
        for m in re.finditer(r'\bcrate::\{', code):
            paths,_ = parse_use_tree(code, m.start())
            for pp in paths:
                if len(pp) > 1: rows[t][pp[1]] += 1
    return rows
rows = refs(lambda f: prod_code(f, files))
for t in sorted(rows):
    mods = {k: v for k, v in rows[t].items() if k in MODS and k != t}
    flat = {k: v for k, v in rows[t].items() if k not in MODS}
    if not mods and not flat: continue
    print("==", t)
    print("  mods:", dict(sorted(mods.items())))
    print("  flat:", dict(sorted(flat.items())))
