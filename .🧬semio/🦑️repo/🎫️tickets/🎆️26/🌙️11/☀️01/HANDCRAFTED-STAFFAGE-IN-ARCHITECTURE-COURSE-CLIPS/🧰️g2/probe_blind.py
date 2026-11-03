import importlib, sys
import numpy as np
sys.path.insert(0, ".")
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
parts = V.staffage_parts(sys.argv[1], "")
for k, v in parts.items():
    if k.startswith("slat-") and k not in ("slat-0", "slat-1", "slat-2", "slat-13"):
        continue
    print(k, np.round(v.min(axis=0), 3), np.round(v.max(axis=0), 3), len(v))
print(len(parts))
