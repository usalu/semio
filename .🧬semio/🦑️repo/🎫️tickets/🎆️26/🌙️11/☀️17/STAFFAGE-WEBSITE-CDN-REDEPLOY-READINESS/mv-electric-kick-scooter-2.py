"""Electric kick scooter 1:50 front: the fork crown (0.084 m by 0.034 m, z -0.015 to 0.069 m, height 0.253 to 0.287 m) was in the volume and the plan but not in the front drawing."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
n = d.shape(50, "front", "fork-blade")
d.insert_after(n, {"kind": "rect", "x": -0.015, "y": 0.253, "w": 0.084, "h": 0.034, "r": 0.004, "id": "fork-crown", "bone": "fork", "role": "body", "filled": True, "z": 3.1})
d.commit()
