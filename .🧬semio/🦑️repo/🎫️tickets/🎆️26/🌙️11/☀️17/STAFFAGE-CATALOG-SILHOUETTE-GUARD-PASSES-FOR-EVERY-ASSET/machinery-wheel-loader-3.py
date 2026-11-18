"""Wheel loader: the lift pivot shapes of 1:50 bound to the front frame bone."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/wheel-loader")
for v in ("front", "plan"):
    n = d.shape(50, v, "lift-pivot"); o = d.obj(n)
    d.replace_obj(n, {"id": "lift-pivot", "bone": "front-frame", "role": "body", "z": 2.1, "filled": True, "tone": 0.45, "kind": "rect", "x": o["x"], "y": o["y"], "w": o["w"], "h": o["h"], "region": "front-frame", "material": "paint-steel"})
d.commit()
