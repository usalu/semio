import importlib, os
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
B = "🎓️teaching.🏛️architecture.⚡️energy."
shared = importlib.import_module(B + "🏗️authoring.🎨️visuals.🐍️")
for c in ("shared-radiator-winter", "shared-radiator-thermostat-front", "heating-calculation-radiator"):
    try:
        print(c, shared.staffage_height(c))
    except Exception as e:
        print(c, "ERR", e)
m = importlib.import_module(B + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
p = m._plant_house()
print("room_height", p["room_height"], "person", p["person"].height, [r.height for r in p["radiators"]], [r.width for r in p["radiators"]])
print("scale", p["room_height"] / 2.5)
