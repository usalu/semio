"""📏 [DEBUG] Which label font sizes pass the strict 11 px cap-height rule of layout_strict.py at 1280 px width?"""
import importlib
import os
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/toolkit-r2/fonts")
manim = importlib.import_module("manim")
room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
strict = importlib.import_module("layout_strict")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
samples = ["WWR 45 %", "Lighting", "d 0.6 m", "522 kWh/a", "1", "kWh/m²a", "Heating demand 204 kWh/a heat"]
with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720}):
    for size in (14, 15, 16, 17, 18, 19):
        row = []
        for sample in samples:
            parts = [room.Number(p) if p[0].isdigit() else p for p in sample.split(" ")]
            label = strict.Label(room.text(parts, size=size), 0)
            row.append(f"{label.cap_height * 1280 / 14.222:.1f}")
        print(f"[DEBUG] size {size}: " + " | ".join(f"{s}={v}" for s, v in zip(samples, row)))
    body = fonts.body_text("Lighting", font_size=19.2, color="#ffffff")
    print("[DEBUG] energy label 19.2: Lighting", f"{strict.Label(body, 0).cap_height * 1280 / 14.222:.1f}")
