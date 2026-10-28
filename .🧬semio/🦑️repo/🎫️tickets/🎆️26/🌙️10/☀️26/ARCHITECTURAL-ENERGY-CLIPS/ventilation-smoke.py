"""🎬 Smoke render the actual natural ventilation adjustable-window scene."""

import importlib
import sys
from pathlib import Path
from manim import tempconfig

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
OUTPUT=Path(__file__).resolve().parent/"🗑️generated"/"ventilation-smoke"
module=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🪟️natural.🐍️")
with tempconfig({"media_dir":str(OUTPUT),"pixel_width":854,"pixel_height":480,"frame_rate":5,
                 "skip_animations":True,"save_last_frame":True,"disable_caching":True,"verbosity":"ERROR"}):
    module.Beat1_Fensterregeln().render()
