"""⚡️ Repair ops of `diesel-generator`, first part (the traces follow in diesel-generator.sh).
- Dimensions: door handles, hinges and the louvre frames stand 8 and 6 mm proud of the 6.060 × 2.440 m container (body of 1:50: 6.076 m long, 2.452 m wide; 1:100: 6.076, 2.440 m). Declared 6.076 m and 2.452 m (1:50 missed the old values by 16 and 12 mm against 7.5 mm, 1:100 by 16 against 15 mm).
- Corner castings at 1:50, side and front elevation: the drawn castings had their aperture as a hole through the drawing; in the volume one does not see through a casting (the end rail and the opposite casting stand behind it). The casting is drawn full and the aperture as a dark shape on it.
- Base frame at 1:50: the bottom end rails and side rails run under the corner posts to the outer faces of the container (solids `bottom-end-rail`, `bottom-rail`); the drawn base stopped at the castings and left an 18 × 82 mm (side) and 12 × 82 mm (front) notch between casting, post and wall (178 and 163 mm in the tiles of the guard). The drawn base runs through, behind castings and posts.
- Plan at 1:50: the four inlet louvres stand out of the side walls under the roof edge (frames to 1.2202 m, inclined blades beyond) and were not drawn (24.4 mm). Each is drawn as one strip of its frame length from the wall face to 1.226 m, in the material of the blades, so the guard compares the slender blades at the width of this strip (see `U/requests.md`: the centre line the guard finds for a thin inclined blade is crooked at its ends and 51 mm wide for a blade of 31 mm in plan).
"""
from oplib import load, write

asset = load("diesel-generator")
ops = [{"op": "dims", "set": {"widthM": 6.076, "depthM": 2.452}}]
for view in ("elevation", "front"):
    for s in asset["representations"]["50"]["geometry"][view]:
        if "casting" in s["id"] and not s["id"].endswith("-aperture") and s.get("d", "").count("M") == 2:
            outer, hole = ["M" + part for part in s["d"].split("M")[1:]]
            ops.append({"op": "shape-set", "scale": "50", "view": view, "id": s["id"], "set": {"d": outer.strip()}})
            ops.append({"op": "shape-add", "scale": "50", "view": view, "after": s["id"], "shape": {**{k: v for k, v in s.items() if k != "d"}, "id": s["id"] + "-aperture", "z": round(s["z"] + 0.01, 3), "tone": 0.08, "d": hole.strip()}})
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "skid-tank", "set": {"points": [[0.0, 0.013], [6.06, 0.013], [6.06, 0.2], [0.0, 0.2]]}})
ops.append({"op": "shape-set", "scale": "50", "view": "front", "id": "skid-tank", "set": {"x": -1.22, "w": 2.44}})
for k, (x0, x1) in enumerate(((-2.85, -1.45), (-0.25, 1.15)), 1):
    for side, sign in (("r", -1), ("l", 1)):
        ya, yb = sorted((sign * 1.226, sign * 1.19))
        ops.append({"op": "shape-add", "scale": "50", "view": "plan", "after": "roof", "shape": {"id": f"inlet-louvre-{k}-{side}", "bone": "container", "role": "detail", "z": 1.9, "filled": True, "tone": 0.3, "kind": "rect", "x": x0, "y": ya, "w": round(x1 - x0, 4), "h": round(yb - ya, 4), "region": "roof", "material": "aluminium"}})
if len(ops) > 3:
    write("diesel-generator", ops)
