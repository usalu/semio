import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[7] / "tutorial"
sys.path.insert(0, str(ROOT))
import importlib.util
from manim import VGroup
from manim_visuals import caption_bar
for tag, rel in (("m1", "energy/demand/Heating/1_introduction/scene_1.py"),
                 ("m4", "energy/demand/Heating/4_internal_heat_gain/scene_4.py")):
    spec = importlib.util.spec_from_file_location(f"scene_{tag}", ROOT / rel)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    for name in sorted(dir(mod)):
        cls = getattr(mod, name)
        if name.startswith("Beat") and hasattr(cls, "NARRATION"):
            for key, _en, de in cls.NARRATION:
                label = caption_bar(de)[1]
                lines = len(label.submobjects) if isinstance(label, VGroup) else 1
                flag = "  !!" if lines > 2 else ""
                print(f"[DEBUG] {tag} {name:34s} {key:10s} {lines} lines {len(de):3d} chars{flag}")
