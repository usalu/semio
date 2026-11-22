"""✏️ Second-round 2D drawings of vehicles/delivery-van: the front bumper, grille and headlamps that the 1:200 and 1:500 models gained, drawn by hand in elevation and front. `python3 rework-delivery-van-drawings.py <asset 🔣️.json>`; publish the volumes with `volumes-insert.py` afterwards."""
import json, sys

path = sys.argv[1]
document = json.loads(open(path, encoding="utf8").read())
R = document["asset"]["representations"]
TRIM = {"bone": "chassis", "role": "body", "filled": True, "tone": 0.2, "material": "plastic-trim"}
LENS = {"bone": "chassis", "role": "glazing", "z": 7, "filled": True, "tone": 0.9, "kind": "ellipse", "material": "glass"}


def put(scale, view, shape):
    rows = R[scale]["geometry"][view]
    R[scale]["geometry"][view] = [row for row in rows if row["id"] != shape["id"]] + [shape]


put("200", "elevation", {"id": "bumper-front", **TRIM, "z": 2.3, "kind": "poly", "points": [[2.12, 0.3], [2.12, 0.56], [2.67, 0.56], [2.67, 0.42], [2.645, 0.295], [2.55, 0.255], [2.2, 0.263]], "closed": True})
put("200", "front", {"id": "bumper", **TRIM, "z": 2.4, "kind": "poly", "points": [[-0.985, 0.56], [-0.985, 0.37], [-0.955, 0.31], [-0.9, 0.275], [-0.8, 0.262], [0.8, 0.262], [0.9, 0.275], [0.955, 0.31], [0.985, 0.37], [0.985, 0.56]], "closed": True})
put("200", "front", {"id": "grille", "bone": "chassis", "role": "detail", "z": 2.5, "filled": True, "tone": 0.15, "kind": "rect", "region": "engine-bay", "material": "engine-metal", "x": -0.45, "y": 0.7, "w": 0.9, "h": 0.2})
put("500", "elevation", {"id": "bumper-front", **TRIM, "z": 2.3, "kind": "poly", "points": [[2.12, 0.3], [2.12, 0.56], [2.67, 0.56], [2.67, 0.42], [2.6, 0.27], [2.2, 0.263]], "closed": True})
put("500", "elevation", {"id": "headlamp", **LENS, "cx": 2.48, "cy": 0.86, "rx": 0.15, "ry": 0.085})
put("500", "front", {"id": "bumper", **TRIM, "z": 2.4, "kind": "rect", "x": -0.992, "y": 0.26, "w": 1.984, "h": 0.3, "r": 0.08})
for side, sign in (("l", 1), ("r", -1)):
    put("500", "front", {"id": f"headlamp-{side}", **LENS, "cx": sign * 0.84, "cy": 0.86, "rx": 0.13, "ry": 0.085})
open(path, "w", encoding="utf8").write(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("written")
