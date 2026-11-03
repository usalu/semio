import importlib
import sys
from pathlib import Path

import numpy as np

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
g = S.staffage("excel-program-occupant", unit=1.0)
print("unrotated w,h", g.width, g.height, "centre", g.get_center())
print("height_m", S.staffage_height("excel-program-occupant"))
r = g.copy().rotate(-np.pi / 2)
print("rotated w,h", r.width, r.height)
