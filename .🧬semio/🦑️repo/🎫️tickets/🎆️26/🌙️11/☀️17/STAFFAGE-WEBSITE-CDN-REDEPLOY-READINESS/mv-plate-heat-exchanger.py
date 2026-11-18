"""Plate heat exchanger: the 1:50 front hinge barrel is the 0.42 m by 0.02 m bar the volume capsule projects (it runs along the lateral axis, 0.4 m long plus its 10 mm caps), at its world position."""
from mvlib import *
d = D("🏗️machinery/plate-heat-exchanger")
n = d.shape(50, "front", "damper-hinge")
o = d.obj(n)
for k in ("cx", "cy", "rx", "ry"): o.pop(k)
o.update({"kind": "rect", "x": -0.21, "y": 0.6, "w": 0.42, "h": 0.02})
o = {k: o[k] for k in ("id", "bone", "role", "z", "filled", "tone", "kind", "x", "y", "w", "h", "region", "material")}
d.replace_obj(n, o)
d.commit()
