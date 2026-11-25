"""🔥️ Ops writer of `used-forced-draught-oil-burner`: the air shutter turns by 35 degrees when the burner runs and the two oil hoses hang down by 85 degrees once the burner is lifted, but their front and plan shapes were flat outlines of the rest pose (shutter 26 to 29 mm, hoses 176 to 187 mm off the side elevation). The shutter becomes its side elevation profile swept laterally, the hoses and their caps round bars (`member`) along the hose bone: `python3 make/used-forced-draught-oil-burner.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("used-forced-draught-oil-burner")
ops = solidify(a, {"air-damper"})
z0 = lateral_rest(a, "hoses")
for scale in scales(a):
    for view in ("front", "plan"):
        for s in views(a, scale)[view]:
            if s["bone"] != "hoses":
                continue
            if s["kind"] == "member":
                ops.append(op(scale, view, s["id"], s))
                continue
            twin = shape(a, scale, "elevation", s["id"])
            b = box(s)
            z = round(((b[0] + b[1]) / 2 if view == "front" else (b[2] + b[3]) / 2) - z0, 6)
            e = box(twin)
            ops.append(op(scale, view, s["id"], member(s, (e[0], 0, z), (e[1], 0, z), round(e[3] - e[2], 6))))
write("used-forced-draught-oil-burner", ops)
