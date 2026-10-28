import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
s = m.orientation_study(); d = s["design"]
for th in (90, 180, 282, 315, 360):
    r = m.evaluate({**d, "orientation": float(th)})
    print(th, "light check", 9*48*1500/1000*(1-r["autonomy"]), r["lighting"], "heat", r["heating_demand"]/3.5, r["heating"], "cool", r["cooling_demand"]/3, r["cooling"])
    print("  sun hours w/s", len(m.sun_samples(float(th),"winter")), len(m.sun_samples(float(th),"summer")), "u", r["u_wall"], r["window_area"], r["autonomy"])
best = {}
for lab, dd in (("base", d), ("d1.2", {**d, "overhang": 1.2}), ("w.3", {**d, "wwr": .3}), ("w.8", {**d, "wwr": .8}), ("d.6", {**d, "overhang": .6})):
    es = {t: m.evaluate({**dd, "orientation": float(t)})["electricity"] for t in range(270, 451)}
    lo = min(es, key=es.get); print(lab, lo, es[lo])
