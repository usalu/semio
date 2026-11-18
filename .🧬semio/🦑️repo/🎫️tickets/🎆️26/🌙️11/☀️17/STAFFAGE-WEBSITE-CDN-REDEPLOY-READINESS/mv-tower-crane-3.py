"""Tower crane 1:200 and 1:100 elevation, counter-jib: the bone is turned by 180 degrees about the truss middle, so the volume chord centre lines are at local y = +-0.7 m (world 43.70 and 45.10 m); the drawn rectangles of 0.12 m are at y = 0.64 (upper pair) and -0.76 (lower pair) and the zigzag of both diagonal outlines stays centred on the truss middle."""
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc in (200, 100):
    for sid in ("counter-chord-top-l", "counter-chord-top-r"):
        d.set(d.shape(sc, "elevation", sid), "y", 0.64)
    for sid in ("counter-chord-bottom-l", "counter-chord-bottom-r"):
        d.set(d.shape(sc, "elevation", sid), "y", -0.76)
    for sid in ("counter-diagonals-l", "counter-diagonals-r"):
        n = d.shape(sc, "elevation", sid); o = d.obj(n)
        o["points"] = [[x, round(y - 0.06, 4)] for x, y in o["points"]]
        d.replace_obj(n, o)
d.commit()
