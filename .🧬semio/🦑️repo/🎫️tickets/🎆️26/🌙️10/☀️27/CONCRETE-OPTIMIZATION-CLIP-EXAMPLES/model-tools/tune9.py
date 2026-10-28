import importlib.util, itertools
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
bal = {"electricity": .4, "cost": .3, "daylight": .2, "carbon": .1}
ch = m.reweigh(bal, "cost", .6)
keys = {"electricity": ("intensity", 1), "cost": ("cost", 1), "daylight": ("daylight", -1), "carbon": ("carbon", 1)}
def rank(designs, w):
    ev = {k: m.evaluate(d) for k, d in designs.items()}
    sc = {k: 0 for k in designs}
    for c, (key, sign) in keys.items():
        vals = {k: sign * ev[k][key] for k in designs}
        lo, hi = min(vals.values()), max(vals.values())
        for k in designs: sc[k] += w[c] * (vals[k] - lo) / (hi - lo)
    return sorted(sc.items(), key=lambda x: x[1]), ev
base = {"orientation": 180}
res = []
D = lambda w, o, t: {**base, "wwr": w, "overhang": o, "insulation": t}
for pw, rw, rd, ot, bw, ib in itertools.product((.2,.25,.3), (.4,.45,.5,.55,.6), (.3,.4,.5,.6), (.2,.25,.3), (.7,.75,.8), (.8,)):
    ow = pw
    d = {"punched": D(pw,0,.16), "ribbon": D(rw,rd,.16), "glass_shaded": D(bw,1.2,.16), "punched_insulated": D(ow,0,ot), "glass_bare": D(bw,0,.16)}
    for pr in (D(bw,1.2,.3),):
        d6 = {**d, "premium": pr}
        r5b, ev = rank(d, bal); r5c, _ = rank(d, ch); r6b, _ = rank(d6, bal); r6c, _ = rank(d6, ch)
        if r5b[0][0] == r5c[0][0]: continue
        gaps = min(b[1] - a[1] for a, b in itertools.pairwise(r5b))
        if gaps < .02: continue
        o5 = [k for k, _ in r5b]; o6 = [k for k, _ in r6b if k != "premium"]
        if o5 == o6 or r6b[0][0] == "premium" or r6c[0][0] == "premium": continue
        bests = {min(ev, key=lambda k: ev[k]["intensity"]), min(ev, key=lambda k: ev[k]["cost"]), max(ev, key=lambda k: ev[k]["daylight"]), min(ev, key=lambda k: ev[k]["carbon"])}
        if len(bests) < 3: continue
        res.append((round(gaps,3), pw, rw, rd, ow, ot, bw, pr["wwr"], pr["overhang"], r5b[0][0], r5c[0][0], o5, o6))
res.sort(reverse=True)
for r in res[:10]: print(r)
print(len(res))
