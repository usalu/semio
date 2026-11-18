"""Scissor lift 1:50: the plan chassis body follows the wheel wells of the deck (z 0.245 m between the wheels) instead of a full rectangle."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/scissor-lift")
pts = [[-0.925, -0.405], [-0.82, -0.405], [-0.82, -0.245], [-0.28, -0.245], [-0.28, -0.405], [0.28, -0.405], [0.28, -0.245], [0.82, -0.245], [0.82, -0.405], [0.925, -0.405], [0.925, 0.405], [0.82, 0.405], [0.82, 0.245], [0.28, 0.245], [0.28, 0.405], [-0.28, 0.405], [-0.28, 0.245], [-0.82, 0.245], [-0.82, 0.405], [-0.925, 0.405]]
n = d.shape(50, "plan", "chassis-body"); o = d.obj(n)
out = {}
for k, v in o.items():
    if k == "kind": out[k] = "poly"; out["points"] = pts; out["closed"] = True
    elif k not in ("x", "y", "w", "h", "r"): out[k] = v
d.replace_obj(n, out)
d.commit()
