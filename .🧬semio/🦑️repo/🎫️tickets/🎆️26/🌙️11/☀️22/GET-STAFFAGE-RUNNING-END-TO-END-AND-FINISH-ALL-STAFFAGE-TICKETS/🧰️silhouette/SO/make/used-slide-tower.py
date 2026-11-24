"""🛝️ Writes the repair ops of `used-slide-tower`: at 1:200 the two lifting eyes on the ridge (64 mm rings of 16 mm rod at 1:50 and 1:100) are kept as lugs at the 30 mm feature, 30 x 62 x 30 mm, in the volume, its thermal list and the three views, so the body is 4.417 m high like the declared 4.427 m instead of ending 72 mm lower at the ridge."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
Y0, Y1 = 0.955342, 1.017342
ops = []
for side, z in (("left", 0.6), ("right", -0.6)):
    sid = f"eye-{side}"
    look = {"bone": "roof", "role": "detail", "filled": True, "tone": 0.62, "region": "galvanised-steel", "material": "steel-galvanised-weathered", "kind": "poly", "closed": True}
    ops += [
        {"op": "solid-add", "scale": "200", "after": "ridge", "solid": {"id": sid, "bone": "roof", "material": "steel-galvanised-weathered", "role": "detail", "tone": 0.62, "kind": "box", "size": [0.03, round(Y1 - Y0, 6), 0.03], "transform": {"translation": [0.8, round((Y0 + Y1) / 2, 6), z]}}},
        {"op": "solid-add", "scale": "200", "list": "thermal", "solid": {"id": f"th-{sid}", "of": sid, "region": "galvanised-steel"}},
        {"op": "shape-add", "scale": "200", "view": "elevation", "after": "ridge", "shape": {"id": sid, **look, "z": 2.81, "points": [[0.785, Y0], [0.815, Y0], [0.815, Y1], [0.785, Y1]]}},
        {"op": "shape-add", "scale": "200", "view": "front", "after": "ridge", "shape": {"id": sid, **look, "z": 2.09, "points": [[z - 0.015, 2.6 + Y0], [z + 0.015, 2.6 + Y0], [z + 0.015, 2.6 + Y1], [z - 0.015, 2.6 + Y1]]}},
        {"op": "shape-add", "scale": "200", "view": "plan", "after": "ridge", "shape": {"id": sid, **look, "z": 5.6173, "points": [[-0.015, z - 0.015], [0.015, z - 0.015], [0.015, z + 0.015], [-0.015, z + 0.015]]}},
    ]
write("used-slide-tower", ops)
