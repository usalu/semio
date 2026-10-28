import importlib, sys
sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
for th in (270., 300., 315., 330., 360.):
    for season in ("winter", "summer"):
        s = m.sun_state(th, season)
        print(th, season, {k: round(v, 1) if isinstance(v, float) else v for k, v in s.items()}, "samples", len(m.sun_samples(th, season)), "sun", round(m.evaluate({"orientation": th, "wwr": .6, "overhang": 0, "insulation": .16})["sun_" + season], 2))
