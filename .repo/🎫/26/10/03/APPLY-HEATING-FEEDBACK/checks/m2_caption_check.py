import sys
from pathlib import Path
M2 = Path(__file__).resolve().parents[7] / "tutorial" / "energy" / "demand" / "Heating" / "2_conduction"
sys.path.insert(0, str(M2))
import scene_2
from manim import VGroup
from manim_visuals import caption_bar
for name in dir(scene_2):
    cls = getattr(scene_2, name)
    if name.startswith("Beat") and hasattr(cls, "NARRATION"):
        for key, _en, de in cls.NARRATION:
            label = caption_bar(de)[1]
            lines = len(label.submobjects) if isinstance(label, VGroup) else 1
            flag = "  !!" if lines > 2 or "_" in de else ""
            print(f"{name:26s} {key:12s} {lines} lines {len(de):3d} chars{flag}")
print(f"U gedämmt={scene_2.U_INSULATED} R={scene_2.R_INSULATED} d={scene_2.D_INSULATED}")
for title, layers in scene_2.WALLS:
    u = scene_2._u_value(layers)
    print(f"{title:28s} R={scene_2._r_total(layers):.3f} U={u:.2f} Q={u * 500:.0f} W")
print(f"A={scene_2.ENVELOPE_AREA} PHI_T={scene_2.PHI_T:.1f} H_T={scene_2.H_T:.2f}")
