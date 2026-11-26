"""🏖️ Ops writer of `sunbather-on-towel`: the roll from prone to supine twists the pelvis by 180 degrees, and the side elevation drew thighs, shanks, feet, toes and the head as flat outlines that shrink to a line when the body lies on its side. Each keeps its outline and is swept laterally: foot and toe over the sweep of their front twin, thigh and shank about the middle of the foot over 85 % of the diameter the front draws (the box that stands for a round limb), head and hair over 0.128 m. Torso and pelvis were boxes over the full width, 47 to 51 mm higher than the sliced body of the front when the body is a quarter turned: they keep their outline over 65 % of the width and nest a core (the middle 55 % of the thickness) over the full width, the stepped section of an ellipse. The side elevation draws the hand with its fingers (0.18 m); the front swept the section of the palm over the palm only (0.106 m at 1:50 and 1:100) and the plan drew the lateral sweep of the side outline, 40 to 72 mm apart on the side-lying body. The plan takes the bodies of the front, and the sweep of the front runs 10 mm (1:50) and 8 mm (1:100) further into the fingers, as far as the silhouette guard of the front at rest allows (to the fingertips it measures 0.915 against 0.92); in the plan, where the hand lies flat on the towel, the sweep runs to the middle joints of the fingers (0.148 m): `python3 make/sunbather-on-towel.py [--palm]` writes its ops (`--palm` leaves the sweep of the front hand as it is)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *
a, ops = asset("sunbather-on-towel"), []
LONGER = {"50": 0.01, "100": 0.008}
REACH = {} if "--palm" in sys.argv else {("50", "hand-l"): 0.11588, ("50", "hand-r"): 0.11588, ("100", "hand-l"): 0.11378, ("100", "hand-r"): 0.11378}
sweep = lambda z0, z1: {"kind": "extruded", "sweep": "w", "z0": round(z0, 5), "z1": round(z1, 5)}
for scale in scales(a):
    g = a["representations"][scale]["geometry"]
    front, side = {s["id"]: s for s in g["front"]}, {s["id"]: s for s in g["elevation"]}
    for sid, s in side.items():
        part, twin = s["bone"].split("-")[0], front.get(sid)
        if part in ("foot", "toe") and twin is not None and twin.get("solid", {}).get("sweep") == "w":
            ops.append({"op": "shape-set", "scale": scale, "view": "elevation", "id": sid, "set": {"solid": twin["solid"]}})
        elif part in ("thigh", "shank") and twin is not None and twin.get("solid", {}).get("kind") == "revolved":
            left = front["foot-l"]["solid"]
            mid, half = (left["z0"] + left["z1"]) / 2 * (1 if sid.endswith("-l") else -1), 0.85 * max(p[1] for p in twin["points"])
            ops.append({"op": "shape-set", "scale": scale, "view": "elevation", "id": sid, "set": {"solid": sweep(mid - half, mid + half)}})
        elif sid in ("head", "head-hair", "head-hair-2"):
            ops.append({"op": "shape-set", "scale": scale, "view": "elevation", "id": sid, "set": {"solid": sweep(-0.064, 0.064)}})
    for sid in ("torso", "pelvis"):
        s, core = side[sid], side.get(f"{sid}-core")
        half = round(max(abs((core or s)["solid"]["z0"]), abs((core or s)["solid"]["z1"])), 5)
        vs = [p[1] for p in s["points"]]
        mid, b = (min(vs) + max(vs)) / 2, (max(vs) - min(vs)) / 2
        ops.append({"op": "nest", "scale": scale, "view": "elevation", "id": sid, "axis": 1, "range": [round(-0.65 * half, 5), round(0.65 * half, 5)], "pieces": [{"id": f"{sid}-core", "clip": [round(mid - 0.55 * b, 5), round(mid + 0.55 * b, 5)], "range": [-half, half]}]})
    for sid in ("hand-l", "hand-r"):
        if sid in side and sid in front and front[sid].get("solid", {}).get("sweep") == "u":
            reach = REACH.get((scale, sid), round(min(max(p[0] for p in side[sid]["points"]), front[sid]["solid"]["z1"]), 5))
            ops.append({"op": "shape-set", "scale": scale, "view": "front", "id": sid, "set": {"solid": {**front[sid]["solid"], "z1": reach}}})
    for s in g["front"]:
        if s["bone"] in ("hand-l", "hand-r") and "solid" in s:
            ops.append({"op": "solid-from", "scale": scale, "view": "plan", "id": s["id"], "from": "front", "paint": s["bone"], "after": s["bone"]})
            if s["id"] == s["bone"] and (scale, s["id"]) in REACH and s["solid"].get("sweep") == "u":
                ops.append({"op": "shape-set", "scale": scale, "view": "plan", "id": s["id"], "set": {"solid": {**s["solid"], "z1": 0.148}}})
write("sunbather-on-towel", ops)
