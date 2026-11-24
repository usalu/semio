"""🚜️ Repair ops of the volume of `forklift`: seven pins, eyes and rollers that lie along the axle (a revolve of length t that starts at +z0) were copied to the other side with a translation of -2·z0, which puts the second copy inside the truck instead of mirrored; the translation becomes -(2·z0 + t), at 1:50 and 1:100."""
import oplib

IDS = ["stub-axle", "tilt-eye", "tilt-rod-eye", "pivot-pin", "inner-roller", "carriage-roller-1", "carriage-roller-2"]
asset = oplib.load("forklift")
ops = []
for scale in ("50", "100"):
    for s in asset["volumes"][scale]["body"]:
        if s["id"] in IDS:
            z0, t = s["origin"][2], max(p[0] for p in s["profile"])
            ops.append({"op": "solid-set", "scale": scale, "id": s["id"], "set": {"repeat": {"count": 2, "translation": [0, 0, round(-(2 * z0 + t), 6)]}}})
oplib.write("forklift", ops)
