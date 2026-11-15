"""Author 60: humans/cyclist 1:100, the 40 mm tyre ring (tyre and rim merged in the drawing) becomes a real 18 mm tyre plus a 22 mm rim ring as at 1:50, so the tyres region is a tyre. Patches volumes/humans/cyclist.json."""
import json, copy
F = "volumes/humans/cyclist.json"
src = json.load(open(F))
s = src["volumes"]["100"]
hub = {x["id"]: x for x in s["body"]}["rear-hub"]
out = []
for x in s["body"]:
    if x["id"] in ("rear-tyre", "front-tyre"):
        w = x["bone"]
        x["p0"][1] = x["p1"][1] = 0.331
        x["r0"] = x["r1"] = 0.009
        x["lateralScale"] = 2.67
        out.append(x)
        rim = copy.deepcopy(x)
        rim["id"] = x["id"].replace("tyre", "rim")
        rim["material"], rim["tone"] = hub["material"], 0.7
        rim["p0"][1] = rim["p1"][1] = 0.311
        rim["r0"] = rim["r1"] = 0.011
        rim["lateralScale"] = 1.4
        out.append(rim)
    else:
        out.append(x)
s["body"] = out
th = [t for t in s["thermal"]]
for side in ("rear", "front"):
    th.append({"id": f"th-{side}-rim", "of": f"{side}-rim", "region": "bike-metal"})
s["thermal"] = th
json.dump(src, open(F, "w"), ensure_ascii=False, indent=2)
print("ok")
