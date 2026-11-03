import importlib
m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
s = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
for c in ("vacuum", "kettle", "shower"):
    g = s.staffage("physics-fundamentals-" + c, unit=1.0)
    print(c, "probe", g.get_left()[0], g.get_bottom()[1], g.get_right()[0], g.get_top()[1])
    g = s.staffage_motion("physics-fundamentals-" + c, unit=1.0, rate=0.0)
    print(c, "motion", g.get_left()[0], g.get_bottom()[1], g.get_right()[0], g.get_top()[1])
