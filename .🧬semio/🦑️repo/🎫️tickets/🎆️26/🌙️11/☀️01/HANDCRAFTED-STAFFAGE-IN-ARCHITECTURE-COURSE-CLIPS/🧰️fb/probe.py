import importlib
import numpy as np

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
cast = "shared-person-pathlines"
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
idx = S._index(S.ensure_casts([cast])[cast])
print("figure", idx.get("figureBoundsM"), "bounds", idx["boundsM"], "h", idx["heightM"])
print("extent", V.staffage_extent(cast))
for rate in (0.0, None):
    g = V.fitted_staffage(cast, height=1.2, rate=rate)
    print(rate, type(g).__name__, g.height, g.width, g.get_bottom())
    g.align_to(np.array([0.0, -0.7, 0.0]), [0, -1, 0])
    print(" after", g.height, g.get_bottom())
