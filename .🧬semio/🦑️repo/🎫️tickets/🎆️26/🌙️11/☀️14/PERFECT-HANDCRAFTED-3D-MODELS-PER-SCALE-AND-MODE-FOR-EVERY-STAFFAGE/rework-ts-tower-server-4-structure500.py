"""🏗️ Tower server, second round pass 1, stage 4: the 1:500 chassis frame over the true chassis length (0.2 m ahead to 0.213 m behind the middle) instead of a centred 0.3 m box, the power supply mass at its real size and place; rear slab of the 1:500 body in the case tone. Writes the candidate document from catalog and source."""
import json, os
T = os.path.dirname(os.path.abspath(__file__))
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"
SRC = T + "/volumes/appliances/tower-server.json"
OUT = T + "/🗑️generated/r3-tower-server/🔣️.json"
doc = json.load(open(A)); V = json.load(open(SRC))["volumes"]; doc["asset"]["volumes"] = V
s = {x["id"]: x for x in V["500"]["structure"]}
s["member-case-floor"]["size"], s["member-case-floor"]["transform"] = [0.03, 0.413, 0.2], {"translation": [0.027, 0.0065, 0.0]}
s["member-case-board-wall"]["size"], s["member-case-board-wall"]["transform"] = [0.378, 0.413, 0.03], {"translation": [0.231, 0.0065, 0.085]}
s["member-case-posts"]["transform"], s["member-case-posts"]["repeat"] = {"translation": [0.231, 0.198, -0.085]}, {"count": 2, "translation": [0.0, -0.383, 0.0]}
s["member-case-top-frame"]["profile"] = [[0.213, -0.1], [-0.2, -0.1], [-0.2, 0.1], [0.213, 0.1]]
s["member-case-top-frame"]["holes"] = [[[0.183, 0.07], [-0.17, 0.07], [-0.17, -0.07], [0.183, -0.07]]]
s["mass-case-psu"]["size"], s["mass-case-psu"]["transform"] = [0.086, 0.14, 0.15], {"translation": [0.377, 0.143, 0.0]}
s["mass-case-internals"]["size"], s["mass-case-internals"]["transform"] = [0.278, 0.353, 0.13], {"translation": [0.181, 0.0065, 0.005]}
next(x for x in V["500"]["body"] if x["id"] == "rear-panel")["tone"] = 0.55
json.dump(doc, open(OUT, "w"), ensure_ascii=False, indent=2)
print("written", OUT)
