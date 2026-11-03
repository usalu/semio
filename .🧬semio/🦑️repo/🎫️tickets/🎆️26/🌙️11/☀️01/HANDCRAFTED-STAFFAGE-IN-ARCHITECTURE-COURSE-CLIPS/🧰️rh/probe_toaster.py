import importlib.util
import sys
from pathlib import Path

root = Path("🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py").resolve()
spec = importlib.util.spec_from_file_location("conduction_clip", root)
module = importlib.util.module_from_spec(spec)
sys.modules["conduction_clip"] = module
spec.loader.exec_module(module)
body = module.staffage_motion("heating-conduction-toaster", unit=0.74 / module.staffage_height("heating-conduction-toaster"), rate=0.0)
print("body w h", body.width, body.height, "bottom", body.get_bottom(), "left", body.get_left())
vis = [s for s in body.slots if s.get_fill_opacity() > 0 or s.get_stroke_opacity() > 0]
xs = [p for s in vis for p in s.points]
import numpy as np
a = np.array(xs)
print("visible x", a[:, 0].min(), a[:, 0].max(), "y", a[:, 1].min(), a[:, 1].max())
