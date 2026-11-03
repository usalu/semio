import importlib
import sys

v = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
for cast in sys.argv[1:]:
    g = v.staffage(cast, unit=1.0)
    m = v.staffage_motion(cast, unit=1.0)
    print(cast, "pose bottom", round(float(g.get_bottom()[1]), 3), "top", round(float(g.get_top()[1]), 3), "w", round(g.width, 3),
          "| motion bottom", round(float(m.get_bottom()[1]), 3), "top", round(float(m.get_top()[1]), 3), "h", round(v.staffage_height(cast), 3))
