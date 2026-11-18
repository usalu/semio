"""Electric kick scooter 1:100: the fork blade is 28 mm wide in the volume; front and plan drew 17 mm."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
for view in ("front", "plan"):
    s = d.shape(100, view, "fork-blade")
    for k in ("r0", "r1"): d.set(s, k, 0.014)
    sol = d.field(s, "solid"); d.set(sol, "z0", 0.04); d.set(sol, "z1", 0.068)
d.commit()
