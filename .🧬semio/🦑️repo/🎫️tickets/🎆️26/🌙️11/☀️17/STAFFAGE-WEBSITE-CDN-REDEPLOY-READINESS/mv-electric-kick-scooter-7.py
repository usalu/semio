"""Electric kick scooter 1:50 front: the fork crown as an extruded solid of the fork bone (profile in the bone plane, swept over z -0.015 to 0.069 m) with a tone, like the plan shape of the crown, so it follows the bone in every pose."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
n = d.shape(50, "front", "fork-crown")
d.replace_obj(n, {"kind": "rect", "x": -0.045315, "y": -0.011315, "w": 0.031, "h": 0.03, "r": 0.004, "id": "fork-crown", "bone": "fork", "role": "body", "filled": True, "z": 3.1, "tone": 0.35, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.015, "z1": 0.069}})
d.commit()
