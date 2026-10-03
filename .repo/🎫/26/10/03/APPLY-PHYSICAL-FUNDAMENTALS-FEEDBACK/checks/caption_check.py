import sys, warnings
from pathlib import Path
PF = Path(__file__).resolve().parents[7] / "tutorial" / "energy" / "demand" / "1_physical_fundamentals"
sys.path.insert(0, str(PF))
import scene_1
from manim import VGroup
from manim_visuals import caption_bar
for name in dir(scene_1):
    cls = getattr(scene_1, name)
    if name.startswith("Beat") and hasattr(cls, "NARRATION"):
        for key, _en, de in cls.NARRATION:
            bar = caption_bar(de)
            label = bar[1]
            lines = len(label.submobjects) if isinstance(label, VGroup) else 1
            flag = "  !!" if lines > 2 else ""
            print(f"{name:28s} {key:12s} {lines} lines {len(de):3d} chars{flag}")
