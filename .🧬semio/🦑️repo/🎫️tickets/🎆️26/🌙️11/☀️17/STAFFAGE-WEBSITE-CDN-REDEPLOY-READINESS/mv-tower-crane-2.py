"""Tower crane 1:200 and 1:100 elevation: the lattice diagonals are drawn as the unfilled zigzag outline they are and carry the member width as pen (60 mm), because a filled closed zigzag counts as one strip as wide as the whole truss (1.56 m) and the slender volume diagonals that follow it were widened to that."""
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc, pen in ((200, 0.3), (100, 0.6)):
    for sid in ("jib-diagonals-l", "jib-diagonals-r", "counter-diagonals-l", "counter-diagonals-r"):
        n = d.shape(sc, "elevation", sid); o = d.obj(n); out = {}
        for k, v in o.items():
            out[k] = False if k == "filled" else v
            if k == "filled": out["penMm"] = pen
        d.replace_obj(n, out)
d.commit()
