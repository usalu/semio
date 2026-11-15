import importlib, os
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
B = "🎓️teaching.🏛️architecture.⚡️energy."
shared = importlib.import_module(B + "🏗️authoring.🎨️visuals.🐍️")
m = importlib.import_module(B + "🔥️heating.🎬️clip.🧲️fundamentals.🐍️")
metre = (m.CEIL_Y - m.FLOOR_Y) / 2.5
print("metre", metre)
print("staffage_height person", shared.staffage_height("shared-person-winter"))
st = m._stage()
o = st["occupant"]
print("occupant height", o.height, o.height / metre)
for t in (0.0, 0.5, 1.0, 2.0):
    o.seek(t)
    print(t, o.height / metre)
