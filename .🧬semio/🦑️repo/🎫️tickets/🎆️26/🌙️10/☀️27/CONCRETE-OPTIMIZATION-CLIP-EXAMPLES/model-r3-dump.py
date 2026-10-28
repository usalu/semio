import sys, json, itertools, importlib.util
CLIP = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
spec = importlib.util.spec_from_file_location("m", sys.argv[2] if len(sys.argv) > 2 else CLIP + "/🧮️model/🐍️.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
seen = {}
original = m._evaluate
def recorder(*args):
    seen[args] = True
    return original(*args)
m._evaluate = recorder
m.evaluate()
m.orientation_study(); m.surface(41)
for kind in ("grid", "random", "lhs"):
    m.samples(kind)
m.factorial(); m.fractional(); m.variants(True); m.pareto(); m.gradient_path(); m.pattern_path(); m.genetic(); m.convergence()
study = m.load_study()
base = study["baseline"]
south = {**base, "orientation": 180.0}
for wwr in (0.3, 0.45, 0.5, 0.7):
    for d in (0, 0.5, 0.6, 1.0):
        m.evaluate({**south, "wwr": wwr, "overhang": d})
for wwr in (0.3, 0.6, 0.8):
    for theta in range(90, 451):
        m.evaluate({**base, "orientation": float(theta), "wwr": wwr, "overhang": 0.0})
scene = sorted(seen)
for theta in range(90, 451, 5):
    for wwr in (0.1, 0.3, 0.45, 0.6, 0.8):
        for d in (0, 0.3, 0.6, 1.0, 1.2):
            m.evaluate({**base, "orientation": float(theta), "wwr": wwr, "overhang": float(d)})
for theta in (354.0, 354.5, 355.0, 5.0, 6.0, 365.0, 366.0):
    for d in (0.3, 0.6, 1.0):
        m.evaluate({**base, "orientation": theta, "overhang": d})
out = {"designs": {}, "sun": {}, "scene": [" ".join(f"{v:.6g}" for v in key) for key in scene]}
for key in sorted(seen):
    out["designs"][" ".join(f"{v:.6g}" for v in key)] = m._evaluate.__wrapped__(*key) if hasattr(m._evaluate, "__wrapped__") else original(*key)
for theta in range(90, 451, 1):
    for season in ("winter", "summer"):
        out["sun"][f"{theta} {season}"] = [[s["hour"], round(s["across"], 9), round(s["weight"], 9)] for s in m.sun_samples(float(theta), season)]
out["state"] = {f"{theta} {season}": m.sun_state(theta, season) for theta in (90, 180, 270, 360) for season in ("winter", "summer")}
json.dump(out, open(sys.argv[1], "w", encoding="utf-8"), indent=0, sort_keys=True)
print(len(out["designs"]), "designs")
