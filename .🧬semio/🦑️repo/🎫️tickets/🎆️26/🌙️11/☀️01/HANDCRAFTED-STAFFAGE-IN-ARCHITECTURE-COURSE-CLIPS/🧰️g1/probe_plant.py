import importlib, sys
sys.path.insert(0, ".")
B = "🎓️teaching.🏛️architecture.⚡️energy."
m = importlib.import_module(B + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
p = m._plant_house()
metre = p["room_height"] / 2.5
for k, box in (("boiler", (0.6, 1.0)), ("tank", (0.65, 1.5))):
    g = p[k] if k == "boiler" else p["tank"]["frame"]
    print(k, g.width / metre, g.height / metre, box, g.get_bottom()[1], p["cellar"].get_bottom()[1])
