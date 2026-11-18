"""Tower crane 1:200 elevation and front: the skip gate (0.4 m by 0.15 m below the skip) is in the volume and in the 1:100 drawings but not in the 1:200 ones."""
from mvlib import *
d = D("🏗️machinery/tower-crane")
src = D("🏗️machinery/tower-crane")
for view in ("elevation", "front"):
    last = d.at("asset", "representations", "200", "geometry", view)["items"][-1]
    o = src.obj(src.shape(100, view, "skip-gate")); o["z"] = 1.9
    d.insert_after(last, o)
d.commit()
