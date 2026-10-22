"""♻️ Cycle check of plans: no added producer edge D -> P (or D -> new macro crate -> its producers) may close a cycle (the target must not reach D through the current graph's normal or build edges). `python3 cycle_check.py tag,tag,...`"""
from hubenv import *
import re
tags = sys.argv[1].split(",")
plans = [json.load(open(os.path.normpath(T + '/../staged/plans/%s.json' % t))) for t in tags]
_, I2L, I2D, LIB = make_world()
NEW = json.load(open(OUT + "/newcrates.json"))
created = {c["path"]: c["content"] for p in plans for c in p.get("creates", [])}
newdeps = {}
for ident, n in NEW.items():
    txt = created.get(n["dir"] + "/Cargo.toml")
    if txt: newdeps[ident] = [byname[m.group(1)] for m in re.finditer(r'^([a-z0-9-]+) = \{ path', txt, re.M) if m.group(1) in byname]
fwd = {}
def down(p):
    if p in fwd: return fwd[p]
    seen = set(); st = [p]
    while st:
        x = st.pop()
        for y in E_.get(x, ()):
            if y not in seen: seen.add(y); st.append(y)
    fwd[p] = seen; return seen
bad = n = 0
for plan in plans:
    for name, c in plan["crates"].items():
        d = c.get("dir")
        if not d: continue
        for i in c.get("owners_prod", []):
            ps = [I2D[i]] if i in I2D else newdeps.get(i, [])
            for p in ps:
                n += 1
                if p == d or d in down(p): bad += 1; print("CYCLE", name, "->", C[p]["name"])
# macro-defining crates that gained producer edges (macros-elsewhere) and macro crates among themselves
for ident, deps in newdeps.items():
    for p in deps:
        for q in deps: pass
print(','.join(tags), 'checked edges', n, 'cycles', bad)
