import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
d = {**m.orientation_study()["design"], "overhang": .6}
for th in (353, 354, 355, 356, 366, 367):
    r = m.evaluate({**d, "orientation": float(th)})
    print(th, "E", round(r["electricity"], 1), "cool", round(r["cooling"], 1), "shade_s", round(r["shading_summer"], 3), "solar_s", round(r["solar_summer"], 1))
    for s in m.sun_samples(float(th), "summer"):
        print("   h", s["hour"], "bearing", round(s["bearing"], 1), "alt", round(s["altitude"], 1), "profile", round(s["profile"], 1), "across", round(s["across"], 1), "w", round(s["weight"], 3))
