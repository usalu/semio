"""Electric kick scooter 1:50: grips of 35 mm diameter (radius 0.0175 m instead of 0.028 m) so the handlebar height is the declared 1.11 m, in the volume and in all three drawings."""
from mvlib import *
d = D("🚜️vehicles/electric-kick-scooter")
r = 0.0175
for gid in ("grip-l", "grip-r"):
    n = d.solid(50, gid); d.set(n, "profile", [[0.0, 0.0], [0.0, r], [0.095, r], [0.095, 0.0], [0.0, 0.0]])
    for view in ("front", "plan"):
        s = d.shape(50, view, gid); d.set(s, "rx", r); d.set(s, "ry", r)
s = d.shape(50, "elevation", "grip-end"); d.set(s, "rx", r); d.set(s, "ry", r)
d.commit()
