import sys, os
A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
sys.path.insert(0, A2)
import lib, jsonfmt

# red-fox curled-sleeping: the curl lies in the horizontal plane (lateral bend of spine, neck, head and tail towards the left, cumulative down each chain)
CURL = {"lumbar": 22, "thorax": 28, "neck": 40, "head": 25, "tail-1": 45, "tail-2": 50, "tail-3": 40}
if len(sys.argv) > 1:
    CURL = {k: float(v) for k, v in (a.split("=") for a in sys.argv[1:])}
path = os.path.join(lib.CAT, "red-fox", "🔣️.json")
doc = jsonfmt.read(path)
for p in doc["asset"]["poses"]:
    if p["id"] == "curled-sleeping":
        for b, v in CURL.items():
            p["bones"].setdefault(b, {})["abductionDeg"] = v
jsonfmt.write(path, doc)
print("curl", CURL)
