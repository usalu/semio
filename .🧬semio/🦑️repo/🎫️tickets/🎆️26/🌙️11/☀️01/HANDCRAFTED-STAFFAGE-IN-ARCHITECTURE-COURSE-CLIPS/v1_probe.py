"""V1: print staffage_height vs group extents for casts: v1_probe.py cast..."""
import importlib
import sys

import numpy as np

shared = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
for cast in sys.argv[1:]:
    group = shared.staffage_motion(cast, unit=1.0, anchor=np.zeros(3)) if "person" in cast else shared.staffage(cast, unit=1.0, anchor=np.zeros(3))
    print(cast, "heightM", round(shared.staffage_height(cast), 4), "group h", round(group.height, 4), "w", round(group.width, 4),
          "bottom", round(group.get_bottom()[1], 4), "cx", round(group.get_center()[0], 4))
