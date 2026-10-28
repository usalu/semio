import importlib.util, itertools
from pathlib import Path
import numpy as np
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
v = m._frozen()[1]
W = np.linspace(.1, .8, 41); D = np.linspace(0, 1.2, 41)
def ev(w, d, key="electricity", t=.16): return m._evaluate(180.0, float(w), float(d), t)[key]
def run(ds, dw, loss, half):
    v["climate"]["direct_share_summer"] = ds; v["climate"]["direct_share_winter"] = dw
    v["daylight"]["sky_angle_loss"] = loss; v["daylight"]["overhang_half_depth"] = half
    m._evaluate.cache_clear()
    g = np.array([[ev(w, d) for d in D] for w in W])
    i, j = np.unravel_index(g.argmin(), g.shape)
    valley = D[g.argmin(axis=1)]
    pad = np.pad(g, 1, constant_values=np.inf)
    locs = sum(1 for a in range(41) for b in range(41) if g[a, b] <= pad[a:a+3, b:b+3].min())
    c = lambda w, d: ev(w, d, "cooling")
    r1, r2, r3, r4 = c(.3, 0), c(.3, 1), c(.7, 0), c(.7, 1)
    pred = (r1 + r2 + r3 + r4) / 4; act = c(.5, .5)
    return dict(opt=(round(W[i], 2), round(D[j], 2), round(g.min())), locs=locs, vr=(round(valley[8], 2), round(valley[30], 2)), curv=round((act - pred) / pred, 3),
                sav=(round(r1 - r2), round(r3 - r4)), share=round((c(.5, 0) - c(.5, .5)) / (c(.5, 0) - c(.5, 1)), 2), base=round(ev(.45, .6)), corner=round(g[-1, -1]))
for ds, dw, loss, half in itertools.product((.6, .8, 1.0), (.3, .5), (.3, .5, .7), (.4, .6, .9)):
    print(ds, dw, loss, half, run(ds, dw, loss, half))
