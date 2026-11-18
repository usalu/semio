"""Scissor lift: the lift cylinder and its rod eye, drawn in the front at 1:200, 1:100 and 1:50, exist in the volume (they are hidden in the elevation by the arm stack and in the plan by the deck)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/scissor-lift")
for sc in (200, 100, 50):
    n = d.solid(sc, "motor-vent-outlet")
    cyl = {"id": "lift-cylinder", "kind": "box", "bone": "chassis", "material": "scissor-steel", "role": "body", "tone": 0.3, "size": [0.8, 0.1, 0.1], "transform": {"translation": [0.0, 0.505, 0.0]}}
    eye = {"id": "lift-cylinder-eye", "kind": "box", "bone": "chassis", "material": "scissor-steel", "role": "body", "tone": 0.3, "size": [0.04, 0.07, 0.44], "transform": {"translation": [0.0, 0.513, 0.0]}}
    d.insert_after(n, eye); d.insert_after(n, cyl)
d.commit()
