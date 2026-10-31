import sys, json
sys.path.insert(0, '/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-V')
from lib import *
G = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-V"
a = json.load(open(f"{G}/orig/{sys.argv[1]}.json"))["asset"]; r = Rig(a)
for b in a["rig"]["bones"]:
    s = r.to_world(b["id"], 0, 0); e = r.to_world(b["id"], b["lengthM"], 0)
    print(f'{b["id"]:16s} par={str(b["parent"]):14s} ({s[0]:6.2f},{s[1]:6.2f})->({e[0]:6.2f},{e[1]:6.2f}) L={b["lengthM"]:5.2f} lim={b.get("limits") or b.get("limitDeg") or ""} {b.get("mass") or b.get("massKg") or ""}')
print({k: v for k, v in a["rig"]["bones"][0].items()})
print("poses", [p["id"] for p in a["poses"]], "clips", [c["id"] for c in a["clips"]])
print("anchors", [x["id"] if isinstance(x,dict) else x for x in a["anchors"]])
print("dims", a["dimensions"])
