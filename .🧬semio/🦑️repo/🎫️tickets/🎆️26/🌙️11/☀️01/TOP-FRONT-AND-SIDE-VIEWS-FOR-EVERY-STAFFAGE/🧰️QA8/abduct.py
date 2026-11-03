import sys, os
A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
sys.path.insert(0, A2)
import lib, jsonfmt
# mallard-duck wing-stretch: wings spread out of the sagittal plane (near wing towards the right side = negative)
A = lib.Asset("mallard-duck")
doc = jsonfmt.read(A.path)
for p in doc["asset"]["poses"]:
    if p["id"] == "wing-stretch":
        p["bones"]["wing-n"]["abductionDeg"] = -55
        p["bones"]["wing-f"]["abductionDeg"] = 55
jsonfmt.write(A.path, doc)
print("ok")

# pigeon: wings spread out of the sagittal plane in the half-open, take-off and airborne poses (near wing negative, far positive)
SPREAD = {"wings-half-open": 40, "takeoff-crouch": 20, "takeoff-wings-up": 55, "takeoff-push": 45, "takeoff-liftoff": 55, "airborne-wings-up": 60, "airborne-wings-down": 60, "airborne-climb": 60}
A = lib.Asset("pigeon")
doc = jsonfmt.read(A.path)
for p in doc["asset"]["poses"]:
    if p["id"] in SPREAD:
        p["bones"]["humerus-n"]["abductionDeg"] = -SPREAD[p["id"]]
        p["bones"]["humerus-f"]["abductionDeg"] = SPREAD[p["id"]]
jsonfmt.write(A.path, doc)

# red-fox: head cock in the mousing-pounce clip (twist about the head axis)
A = lib.Asset("red-fox")
doc = jsonfmt.read(A.path)
for c in doc["asset"]["clips"]:
    if c["id"] == "mousing-pounce":
        for k in c["keys"]:
            if k["t"] == 1:
                k["bones"]["head"]["twistDeg"] = 28
            if k["t"] == 3:
                k["bones"]["head"]["twistDeg"] = 12
jsonfmt.write(A.path, doc)
print("ok2")
