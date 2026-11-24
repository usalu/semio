"""♿️ Hand-written part of the repair of `wheelchair-user` 1:50, written as ops/wheelchair-user.hand.json (`make/wheelchair-user.sh` joins it with the outlines).
Drawing: the six tabs that carry each hand rim on its wheel were not drawn; in the body they bridge the slit of 3.7 mm between tyre and hand rim, so the slit was a row of closed pockets in the body and one open slit in the front view and the plan (22.5 mm). Each set of tabs is a prism swept along the wheel axis like the tyre, rim and hand rim beside it, in all three views; the six tabs are two shapes of three, chosen so that no two tabs of one shape lie behind each other in the front view or the plan (a shape is filled even-odd, two rings that cover each other cancel).
Volume: the two hoodie drawstrings were bowed lofts of 4.8 mm marked `slender`; the guard has no centre line for a bowed loft and compared each as one straight stroke 33.6 mm wide in the side elevation (36.4 mm). At 1:50 a cord is typified to the smallest feature: 7.5 mm thick, no longer `slender`, compared by its own outline.
`python3 make/wheelchair-user.py`"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import lib

asset = lib.asset_of("wheelchair-user")
ops = []
body = {s["id"]: s for s in asset["volumes"]["50"]["body"]}
for side in "lr":
    cord = dict(body[f"drawstring-{side}"])
    cord.pop("slender", None)
    cord["sections"] = [{**section, "rx": 0.00375, "ry": 0.00375} for section in cord["sections"]]
    ops.append({"op": "solid", "scale": "50", "id": cord["id"], "solid": cord})

RADIUS, NEAR, FAR, PIN = 0.2605, 0.259, 0.262, 0.0042


def tab(turn):
    """🔩️ The section of one tab in the wheel plane (u ahead, v up), turned about the axle."""
    ring = []
    for k in range(12):
        phi = 2 * math.pi * k / 12
        u, v = PIN * math.cos(phi), (FAR if math.sin(phi) > 1e-9 else NEAR if math.sin(phi) < -1e-9 else RADIUS) + PIN * math.sin(phi)
        ring.append((u * math.cos(turn) - v * math.sin(turn), u * math.sin(turn) + v * math.cos(turn)))
    return "M " + " L ".join(f"{u:.5f} {v:.5f}" for u, v in ring) + " Z"


HALVES = {"1": (0, 1, 4), "2": (3, 2, 5)}
for view in ("elevation", "front", "plan"):
    shapes = {s["id"]: s for s in asset["representations"]["50"]["geometry"][view]}
    for side, sign in (("l", 1), ("r", -1)):
        like = shapes[f"hand-rim-{side}-2"]
        solid = body[f"hand-rim-tabs-{side}"]
        z = sorted(sign * (value + pad) for value, pad in ((abs(solid["p0"][2]), -PIN), (abs(solid["p1"][2]), PIN)))
        for half, turns in HALVES.items():
            shape = {"id": f"hand-rim-tabs-{side}-{half}", "bone": "rear-wheel", "role": "detail", "z": round(like["z"] + 0.05, 4), "filled": True, "tone": solid["tone"], "region": like["region"], "material": solid["material"], "kind": "path", "d": " ".join(tab(k * math.pi / 3) for k in turns), "solid": {"kind": "extruded", "sweep": "w", "z0": round(z[0], 5), "z1": round(z[1], 5)}}
            ops.append({"op": "shape-add", "scale": "50", "view": view, "after": like["id"], "shape": shape})
lib.write_ops("wheelchair-user", ops, "hand")
