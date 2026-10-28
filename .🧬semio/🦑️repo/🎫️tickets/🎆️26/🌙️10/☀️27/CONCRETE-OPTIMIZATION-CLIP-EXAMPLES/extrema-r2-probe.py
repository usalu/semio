import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
s = m.orientation_study()
d = s["design"]
print(d, s["local_minima"], s["local_maxima"], s["local_maxima_electricity"])
for th in (90, 135, 150, 180, 210, 225, 270, 282, 315, 330, 340, 350, 355, 360, 365, 370, 380, 405, 438, 450):
    r = m.evaluate({**d, "orientation": float(th)})
    print(th, *(f"{k[:4]}={r[k]:.1f}" for k in ("heating_demand","cooling_demand","heating","cooling","lighting","electricity","autonomy","daylight","solar_winter","solar_summer","sun_winter","sun_summer")))
print(len(m.sun_samples(180.0,"winter")), len(m.sun_samples(180.0,"summer")))
