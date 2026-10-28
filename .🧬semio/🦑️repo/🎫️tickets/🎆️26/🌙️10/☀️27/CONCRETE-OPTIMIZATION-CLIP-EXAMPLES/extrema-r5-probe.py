import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
d = m.orientation_study()["design"]
for th in (180, 282):
    r = m.evaluate({**d, "orientation": float(th)})
    print(th, {k: r[k] for k in ("cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed", "cooling_demand")})
