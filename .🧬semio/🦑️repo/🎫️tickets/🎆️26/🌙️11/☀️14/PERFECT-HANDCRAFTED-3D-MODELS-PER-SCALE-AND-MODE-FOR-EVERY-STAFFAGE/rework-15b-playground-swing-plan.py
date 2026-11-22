"""🗺️ Playground swing: plan drawing of the leaning chains as tip-following capsules.

A rigid plan rectangle under the beam cannot follow the swing (every vertex sits at one forward position), so each
chain is a plan capsule from the pin (lateral +-0.25 m) to the seat eye (lateral +-0.20 m) that follows the bone tip:
at rest it is the 50 mm of chain seen from above, in a swung pose it reaches forward to the seat.
Radius as drawn before: 13.6 mm at 1:50 (link width), 9 mm coarser.

Usage: python3 rework-15b-playground-swing-plan.py   (re-reads the catalog file, swaps the eight plan shapes in its text)
"""
import json

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/playground-swing/🔣️.json"
BEAM_X = 1.0
PIN_Z = 0.25
SEAT_Z = 0.20
RADIUS = {"50": 0.0136, "100": 0.009, "200": 0.009, "500": 0.009}


def block(shape):
    return json.dumps(shape, ensure_ascii=False, indent=2).replace("\n", "\n" + " " * 12)


with open(CATALOG, encoding="utf-8") as handle:
    text = handle.read()
document = json.loads(text)
for scale, representation in document["asset"]["representations"].items():
    for shape in representation["geometry"]["plan"]:
        if shape["id"] in ("chain-left", "chain-right"):
            side = 1 if shape["id"] == "chain-left" else -1
            new = {key: value for key, value in shape.items() if key not in ("kind", "points", "closed")}
            new.update({"kind": "capsule", "x0": BEAM_X, "y0": side * PIN_Z, "x1": BEAM_X, "y1": side * SEAT_Z, "r0": RADIUS[scale], "r1": RADIUS[scale], "followTip": True})
            if block(shape) not in text:
                raise SystemExit("shape text not found: " + scale + " " + shape["id"])
            text = text.replace(block(shape), block(new), 1)
with open(CATALOG, "w", encoding="utf-8") as handle:
    handle.write(text)
