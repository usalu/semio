"""sm2 probe: drawn bounds (cast metres) of the paths kept by a shower window at running time, for several windows."""
import importlib

import numpy as np

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
ANCHOR = np.zeros(3)
for t in (60.0, 62.0, 66.0, 90.0):
    for window in (None, (-0.04, 0.3, 0.28, 0.92), (-0.04, 0.45, 0.28, 0.92), (-0.1, 0.4, 0.35, 1.0)):
        m = V.staffage_motion("physics-fundamentals-shower", unit=1.0, anchor=ANCHOR, rate=1.0, window=window)
        m.seek(t)
        pts = np.vstack([s.get_all_points() for s in m.slots])
        probe = m.probes[0].get_center()
        live = pts[np.abs(pts - probe).sum(axis=1) > 1e-6]
        print("T", t, "W", window, "min", np.round(live.min(axis=0)[:2], 3) if len(live) else None, "max", np.round(live.max(axis=0)[:2], 3) if len(live) else None, "n", len(live))
