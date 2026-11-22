"""🪑 Café chair, second round pass 1: back board and rail as 12 mm formed beech ply (15 mm at 1:100) lying on the post fronts, dome rivets through board and tube, coach bolts through the tubes into the coped apron corners (1:50)."""
import json

SOURCE = "volumes/furniture/cafe-chair.json"
POST_RADIUS = 0.011
PLY = {"50": 0.012, "100": 0.015}
RIVET = [[0.0, 0.0], [0.0, 0.006], [0.0015, 0.0052], [0.003, 0.0]]
BOLT = [[0.0, 0.0], [0.0, 0.0065], [0.003, 0.0055], [0.005, 0.0]]
RIVET_STATIONS = {"board-upper": 0.375, "board-lower": 0.285, "rail": 0.14}
BOLTS = {"rear": ("leg-rear", 0.4137, 0.18583), "front": ("leg-front", 0.423, 0.19112)}

document = json.load(open(SOURCE))
volumes = document["volumes"]
for scale, thickness in PLY.items():
    for solid in volumes[scale]["body"]:
        if solid["id"] in ("back-board", "back-rail"):
            shift = solid["sections"][0]["ry"] - thickness / 2
            for section in solid["sections"]:
                section["ry"] = thickness / 2
                section["at"][1] = round(section["at"][1] + shift, 5)
body = [solid for solid in volumes["50"]["body"] if not solid["id"].startswith(("rivet-", "bolt-"))]
for name, station in RIVET_STATIONS.items():
    for side, sign in (("l", 1), ("r", -1)):
        body.append({"id": f"rivet-{name}-{side}", "kind": "revolve", "bone": "back", "material": "steel-tube", "role": "detail", "tone": 0.2, "texture": True, "profile": RIVET, "axis": [0.0, -0.964, 0.265 * sign], "origin": [station, -(POST_RADIUS + PLY["50"]) + 0.001, 0.185 * sign]})
for name, (bone, station, lateral) in BOLTS.items():
    for side, sign in (("l", 1), ("r", -1)):
        body.append({"id": f"bolt-seat-{name}-{side}", "kind": "revolve", "bone": bone, "material": "steel-tube", "role": "detail", "tone": 0.2, "texture": True, "profile": BOLT, "axis": [0.0, 0.0, float(sign)], "origin": [station, 0.0, round((lateral + POST_RADIUS - 0.001) * sign, 5)]})
volumes["50"]["body"] = body
json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)
