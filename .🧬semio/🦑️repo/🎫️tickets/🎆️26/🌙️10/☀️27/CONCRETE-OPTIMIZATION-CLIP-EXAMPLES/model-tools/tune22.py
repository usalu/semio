import importlib.util, itertools, sys, time
from pathlib import Path
import numpy as np
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
v = m._frozen()[1]
W = np.linspace(.1, .8, 36); D = np.linspace(0, 1.2, 25)
def ev(w, d, key="electricity", t=.16, o=180.0): return m._evaluate(o, float(w), float(d), t)[key]
def setv(**kw):
    for k, val in kw.items():
        sec, name = k.split("__"); v[sec][name] = val
    m._evaluate.cache_clear()
def check():
    out = {}
    sw = np.arange(90, 451)
    E = np.array([m._evaluate(float(a), .6, 0.0, .16)["electricity"] for a in sw])
    mins = [int(sw[i]) for i in range(1, len(sw) - 1) if E[i] < E[i-1] and E[i] <= E[i+1]]
    maxs = [int(sw[i]) for i in range(1, len(sw) - 1) if E[i] > E[i-1] and E[i] >= E[i+1]]
    out["or"] = (mins, maxs, round(E[270 - 90] / E[180 - 90] if False else E[360-90] / E[180-90] - 1, 3))
    g = np.array([[ev(w, d) for d in D] for w in W])
    i, j = np.unravel_index(g.argmin(), g.shape)
    out["opt"] = (round(W[i], 2), round(D[j], 2), round(g.min()))
    pad = np.pad(g, 1, constant_values=np.inf)
    out["locs"] = sum(1 for a in range(len(W)) for b in range(len(D)) if g[a, b] <= pad[a:a+3, b:b+3].min())
    c = lambda w, d: ev(w, d, "cooling")
    r1, r2, r3, r4 = c(.3, 0), c(.3, 1), c(.7, 0), c(.7, 1)
    pred = (r1 + r2 + r3 + r4) / 4; act = c(.5, .5)
    out["rmin"] = round(min(r1, r2, r3, r4)); out["curv"] = round((act - pred) / pred, 3); out["sav"] = (round(r1 - r2), round(r3 - r4)); out["share"] = round((c(.5, 0) - c(.5, .5)) / (c(.5, 0) - c(.5, 1)), 2)
    out["base"] = round(ev(.45, .6)); out["min"] = round(g.min()); out["max"] = round(g.max())
    out["valley"] = (round(float(D[g.argmin(axis=1)][5]), 2), round(float(D[g.argmin(axis=1)][28]), 2))
    return out
if __name__ == "__main__":
    t0 = time.time(); print(check(), time.time() - t0)
