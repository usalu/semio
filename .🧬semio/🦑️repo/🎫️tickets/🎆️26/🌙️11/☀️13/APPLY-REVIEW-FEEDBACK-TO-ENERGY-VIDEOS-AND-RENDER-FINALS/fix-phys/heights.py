import sys; sys.path.insert(0, ".")
import importlib, os
os.environ.setdefault("SEMIO_TEACHING_PROFILE","preview"); os.environ.setdefault("SEMIO_TEACHING_LANGUAGE","en")
m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
for c in ("shared-radiator-winter","shared-radiator-front-winter","physics-fundamentals-shower"):
    print(c, m.staffage_height(c), m.staffage_width(c))
