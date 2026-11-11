"""🌀 Inspect the authored AHU assembly and air paths."""
import importlib
import os
import sys
from pathlib import Path
from manim import Camera

sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.⚖️heating-comparison.🐍️")
house = m.house_section(m.HOUSE_CENTER)
ahu = m._build_ahu(house)
camera = Camera(pixel_width=1280, pixel_height=720, background_color=m.P_DEEP_DARK)
camera.capture_mobjects([house["group"], ahu["unit"], ahu["supply_paths"], ahu["extract_paths"], ahu["labels"], ahu["unit_label"]])
camera.get_image().save(Path(__file__).parent / "🗑️generated" / "ahu-layout.png")
