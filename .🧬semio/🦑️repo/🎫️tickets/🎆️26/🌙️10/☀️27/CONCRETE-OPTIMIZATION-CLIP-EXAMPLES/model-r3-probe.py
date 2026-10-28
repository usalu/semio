import sys, math, importlib.util
C = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"
spec = importlib.util.spec_from_file_location("m", C); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
for season in ("winter", "summer"):
    print(season)
    for th in (90, 120, 150, 180, 210, 240, 270, 300, 330, 345, 350, 354, 354.5, 355, 360, 5, 6, 10, 20, 30, 60, 85, 88, 89, 90.5):
        s = m.sun_samples(th % 360 if th < 360 else th, season)
        print(th, len(s), round(sum(i["weight"] for i in s), 4), [round(i["across"], 1) for i in s][:3])
