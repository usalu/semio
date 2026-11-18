"""Diesel generator 1:50: the two lifting eye plates (30 mm thick at z +-1.0 m) drawn in the front as two rectangles, not a full-width slab."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/diesel-generator")
n = d.shape(50, "front", "lifting-eye"); o = d.obj(n)
out = {}
for k, v in o.items():
    if k == "kind": out["kind"] = "path"; out["d"] = "M -1.015 2.59 L -0.985 2.59 L -0.985 2.66 L -1.015 2.66 Z M 0.985 2.59 L 1.015 2.59 L 1.015 2.66 L 0.985 2.66 Z"
    elif k not in ("x", "y", "w", "h"): out[k] = v
d.replace_obj(n, out)
d.commit()
