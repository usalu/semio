import importlib, os
import numpy as np
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
B = "🎓️teaching.🏛️architecture.⚡️energy."
V = importlib.import_module(B + "🏗️authoring.🎨️visuals.🐍️")
icon = V.staffage_motion("physics-fundamentals-shower", unit=1.0, rate=0.0)
for t in (0.0, 5.0):
    icon.seek(t)
    print("t", t)
    for m in icon.family_members_with_points():
        p = m.get_all_points()
        if len(p) > 2:
            print("  ", len(p), round(float(m.get_fill_opacity()), 2), round(float(m.get_stroke_width()), 2), np.round(p[:, :2].min(axis=0), 3), np.round(p[:, :2].max(axis=0), 3))
V2 = importlib.import_module(B + "🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
print("nozzle", V2.SHOWER_NOZZLE, V2.SHOWER_UNIT)
