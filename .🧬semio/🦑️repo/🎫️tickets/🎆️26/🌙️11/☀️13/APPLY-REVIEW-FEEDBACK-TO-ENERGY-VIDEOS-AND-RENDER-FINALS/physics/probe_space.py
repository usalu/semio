import importlib
import os

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
os.environ.setdefault("SEMIO_TEACHING_PROFILE", "preview")
v = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
variants = [
    r"P\;\text{in}\;\mathrm{kW}",
    r"P\text{ in }\mathrm{kW}",
    r"P\text{ in}\;\mathrm{kW}",
    r"P\;\text{in}\;\;\mathrm{kW}",
    r"P\ \text{in}\ \mathrm{kW}",
    r"P\quad\text{in}\quad\mathrm{kW}",
    r"P\text{ in }\,\mathrm{kW}",
    r"P\text{ in }\;\mathrm{kW}",
    r"P\mathrm{kW}",
    r"P\;\mathrm{kW}",
    r"P\text{ in kW}",
]
for src in variants:
    try:
        print(round(v.math_text(src, font_size=24).width, 3), src)
    except Exception as exc:
        print("ERR", src, str(exc)[:60])
