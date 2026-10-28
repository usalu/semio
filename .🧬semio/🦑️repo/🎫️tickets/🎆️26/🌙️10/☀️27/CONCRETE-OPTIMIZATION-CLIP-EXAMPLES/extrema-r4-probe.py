import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
s = m.orientation_study()
d = s["design"]
r = m.evaluate({**d, "orientation": 180.0})
print({k: round(v, 3) for k, v in r.items() if k.startswith(("heating", "cooling", "lamp", "lighting", "cop", "eer", "beam", "direct"))})
print("orientation study", s["local_minima"], s["global_minimum"], s["feasible_minimum"], round(min(e for _, e in s["samples"]), 1))
for lab, dd in (("base", d), ("d0.6", {**d, "overhang": .6}), ("d1.2", {**d, "overhang": 1.2}), ("w.3", {**d, "wwr": .3}), ("w.8", {**d, "wwr": .8})):
    es = {t: m.evaluate({**dd, "orientation": float(t)})["electricity"] for t in range(90, 451)}
    feas = {t: e for t, e in es.items() if 270 <= t <= 450}
    g = min(es, key=es.get); f = min(feas, key=feas.get)
    steps = max(abs(es[t + 1] - es[t]) for t in range(90, 450))
    print(lab, "global", g, round(es[g], 1), "feasible", f, round(feas[f], 1), "E(360)", round(es[360], 1), "maxstep/deg", round(steps, 1), "locals", [t for t in range(91, 450) if es[t] < es[t-1] and es[t] <= es[t+1]])
