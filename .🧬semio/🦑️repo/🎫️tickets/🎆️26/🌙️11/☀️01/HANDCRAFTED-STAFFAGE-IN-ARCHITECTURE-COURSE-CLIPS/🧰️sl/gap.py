import importlib
import sys

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
for src in sys.argv[1:]:
    box = V.math_text(src, font_size=24)
    print(src, [(round(float(b.get_left()[0]), 3), round(float(b.get_right()[0]), 3)) for b in box[:-1]])
