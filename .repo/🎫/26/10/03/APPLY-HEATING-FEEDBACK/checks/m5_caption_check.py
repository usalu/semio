import sys, warnings
from pathlib import Path
M5 = Path(__file__).resolve().parents[7] / "tutorial" / "energy" / "demand" / "Heating" / "5_solar_heat_gain"
sys.path.insert(0, str(M5))
import scene_5
from manim import VGroup
from manim_visuals import caption_bar
bad = 0
for name in sorted(dir(scene_5)):
    cls = getattr(scene_5, name)
    if name.startswith("Beat") and hasattr(cls, "NARRATION"):
        for key, _en, de in cls.NARRATION:
            with warnings.catch_warnings():
                warnings.simplefilter("ignore")
                bar = caption_bar(de)
            label = bar[1]
            lines = len(label.submobjects) if isinstance(label, VGroup) else 1
            flag = "  !!" if lines > 2 else ""
            bad += lines > 2
            print(f"{name:28s} {key:10s} {lines} lines {len(de):3d} chars{flag}")
print(f"over two lines: {bad}")
