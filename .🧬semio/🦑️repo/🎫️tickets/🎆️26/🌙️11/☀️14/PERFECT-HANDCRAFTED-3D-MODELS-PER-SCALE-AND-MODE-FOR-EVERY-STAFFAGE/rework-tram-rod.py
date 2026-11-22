"""🚋 Tram 1:50: the lower coupling rod runs close beside the lower arm (60 mm off the centre line instead of 100 mm), on two short pins."""
import json

PATH = "volumes/vehicles/tram.json"
doc = json.load(open(PATH))
body = doc["volumes"]["50"]["body"]
get = lambda sid: next(s for s in body if s.get("id") == sid)
rod = get("pan-lower-rod")
rod["p0"][2] = rod["p1"][2] = 0.06
for pin in ("pan-rod-pin-base", "pan-rod-pin-knee"):
    r = get(pin)["profile"][1][1]
    get(pin)["profile"] = [[0, 0.0], [0, r], [0.075, r], [0.075, 0.0]]
json.dump(doc, open(PATH, "w"), ensure_ascii=False)
