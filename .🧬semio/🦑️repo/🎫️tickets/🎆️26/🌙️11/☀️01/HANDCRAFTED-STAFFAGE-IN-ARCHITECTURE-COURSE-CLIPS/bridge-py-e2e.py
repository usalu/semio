import importlib, sys, time
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
t = time.time()
ids = ["standing", "walking", "radiator", "seated-reader"]
V.ensure_casts(ids)
print("[DEBUG] ensure", time.time() - t)
for i in ids:
    g = V.staffage(i, unit=0.38)
    print("[DEBUG]", i, V.staffage_height(i), g.width, g.height, g.get_bottom())
m = V.staffage_motion("walking", unit=0.38, color="#FFFFFF")
print("[DEBUG] frames", len(m.frames), m.fps, m.loops, m.width, m.height)
for k in range(30): m.update(1/15)
print("[DEBUG] frame", m.frame)
