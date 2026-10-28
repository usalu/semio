import importlib.util, itertools
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
cache = {}
def ev(w, t, d):
    k = (w, t, d)
    if k not in cache: cache[k] = m.evaluate({"wwr": w, "insulation": t, "overhang": d})
    return cache[k]
good = []
for wwrs in ([.2,.35,.5,.65,.8], [.2,.3,.4,.5,.6,.7,.8], [.15,.3,.45,.6,.75], [.3,.4,.5,.6,.7,.8], [.1,.25,.4,.55,.7]):
  for ts in ([.12,.2,.3], [.1,.2,.3], [.1,.3]):
    for ds in ([0,.6], [0,.4], [0,.8], [0,.3,.6], [0,.5,1.0]):
      for cwin, cwall, cins, cov in itertools.product(range(450, 951, 50), (150,200,250,300,350,400), (2,3,4,5,6,8), (40,60,100,150,250,350,450)):
        if cwin < cwall + cins*16 + 40: continue
        pts = []
        for w, t, d in itertools.product(wwrs, ts, ds):
            e = ev(w, t, d)
            cost = e["wall_area"] * (cwall + cins*100*t) + e["window_area"]*cwin + cov*8*d
            pts.append((cost, e["electricity"], (w,t,d)))
        front = sorted(p for p in pts if not any((q[0] <= p[0] and q[1] <= p[1] and (q[0] < p[0] or q[1] < p[1])) for q in pts))
        if not 6 <= len(front) <= 9: continue
        ratios = [(b[0]-a[0])/(a[1]-b[1]) for a, b in itertools.pairwise(front)]
        if any(r <= 0 for r in ratios): continue
        if not all(x < y for x, y in itertools.pairwise(ratios)): continue
        life = 30
        if ratios[0] > 0.5*life*.25 or ratios[-1] < 2*life*.25: continue
        c1 = [r for r in ratios if r/.25 <= life]; c2 = [r for r in ratios if r/.4 <= life]
        if len(c1) == len(c2) or len(c1) < 1 or len(c2) >= len(ratios): continue
        good.append((len(front), wwrs, ts, ds, cwin, cwall, cins, cov, [round(r,1) for r in ratios]))
print(len(good))
for g in [x for x in good if len(x[1]) == 5 and x[7] >= 40][:40]: print(g)
