import sys
from pathlib import Path
M3 = Path(__file__).resolve().parents[7] / "tutorial" / "energy" / "demand" / "Heating" / "3_convection"
sys.path.insert(0, str(M3))
import scene_3
from manim import VGroup
from manim_visuals import caption_bar
for name in dir(scene_3):
    cls = getattr(scene_3, name)
    if name.startswith("Beat") and hasattr(cls, "NARRATION"):
        for key, en, de in cls.NARRATION:
            bar = caption_bar(de)
            label = bar[1]
            lines = len(label.submobjects) if isinstance(label, VGroup) else 1
            flag = "  !!" if lines > 2 or "_" in de or "_" in en else ""
            print(f"{name:28s} {key:12s} {lines} lines {len(de):3d} chars{flag}")
