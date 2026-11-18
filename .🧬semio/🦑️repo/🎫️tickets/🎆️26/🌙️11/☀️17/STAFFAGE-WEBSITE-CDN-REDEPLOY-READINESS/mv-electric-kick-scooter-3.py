"""Electric kick scooter 1:50: the fork blade is 28 mm wide in the volume (capsule radius 0.014 m, z 0.0395 to 0.0675 m); the front and plan drawing used 17 mm."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
for view in ("front", "plan"):
    s = d.shape(50, view, "fork-blade")
    for k in ("r0", "r1"): d.set(s, k, 0.014)
    sol = d.field(s, "solid"); d.set(sol, "z0", 0.04); d.set(sol, "z1", 0.068)
d.commit()
