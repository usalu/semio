import json
import sys

sys.stdout.reconfigure(encoding="utf8")
root = sys.argv[1]
for n in sys.argv[2:]:
    a = json.load(open(f"{root}/{n}/🔣️.json", encoding="utf8"))["asset"]
    print(n, [(i, p["id"], p["rateBpm"], p["tidalVolumeL"], p["route"]) for i, p in enumerate(a["thermal"]["respiration"]["patterns"])])
    print(" poses", [(p["id"], p["thermal"]["activityMet"], p["thermal"]["breathing"]) for p in a["poses"]])
    for c in a["clips"]:
        for t in c["tracks"]:
            if t["target"] == "breathing":
                print(" clip", c["id"], t)
