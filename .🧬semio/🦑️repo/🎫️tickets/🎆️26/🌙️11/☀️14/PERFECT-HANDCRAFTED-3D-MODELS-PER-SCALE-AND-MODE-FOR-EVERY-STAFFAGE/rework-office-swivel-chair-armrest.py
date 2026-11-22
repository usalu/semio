"""🪑 Office swivel chair: the armrest straps are screwed to the underside of the seat (no air gap), every scale.

Hand-chosen stations in the seat frame (x up from the seat joint, y fore-aft, z lateral, metres).
"""
import json, pathlib

SOURCE = pathlib.Path(__file__).parent / "volumes/furniture/office-swivel-chair.json"
FLAT = [0.0, 0.0, -0.707107, 0.707107]
INNER = {
    "50": [(0.0415, 0.13, 0.006), (0.0415, 0.19, 0.006)],
    "200": [(0.040, 0.17, 0.008), (0.040, 0.22, 0.008)],
    "500": [(0.0415, 0.2, 0.009)],
}
KEEP_FROM = {"50": 1, "200": 1, "500": 1}

document = json.loads(SOURCE.read_text())
for scale, stations in INNER.items():
    for side, sign in (("left", 1), ("right", -1)):
        solid = next(s for s in document["volumes"][scale]["body"] if s["id"] == f"armrest-strap-{side}")
        inner = [{"at": [x, 0.02, sign * z], "rx": 0.03, "ry": ry, "exponent": 4.0, "rotation": FLAT} for x, z, ry in stations]
        solid["sections"] = inner + solid["sections"][KEEP_FROM[scale]:]
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
