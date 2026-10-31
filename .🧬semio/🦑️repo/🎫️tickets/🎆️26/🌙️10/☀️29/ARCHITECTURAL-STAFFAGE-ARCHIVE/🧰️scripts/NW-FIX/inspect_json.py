"""Print the JSON fragments the semantic fixes depend on."""
import json
import os

root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"

def find(asset_id):
    for cat in os.listdir(root):
        catp = os.path.join(root, cat)
        if not os.path.isdir(catp):
            continue
        p = os.path.join(catp, asset_id)
        if os.path.isdir(p):
            docs = [f for f in os.listdir(p) if f.endswith(".json")]
            return os.path.join(p, docs[0])
    raise SystemExit(asset_id)

def load(asset_id):
    return json.load(open(find(asset_id)))

# bookshelf tracks
bs = load("bookshelf")
clip = bs["asset"]["clips"][0]
print("BOOKSHELF clip", clip["id"], "dur", clip["durationS"], "ntracks", len(clip["tracks"]))
for i in (8, 9):
    tr = clip["tracks"][i]
    print(" track", i, tr["target"], [k["t"] for k in tr["keys"]])

# lime epicormic
lime = load("small-leaved-lime")
for den, rep in lime["asset"]["representations"].items():
    if rep.get("status") != "authored":
        print("lime", den, "unavail")
        continue
    hits = []
    for sh in rep["geometry"]["elevation"]:
        regs = [sh.get("region")] + list(sh.get("regions") or [])
        if "epicormic-stem" in regs:
            hits.append(sh["id"])
    print("lime", den, "draws", hits, "n", len(rep["geometry"]["elevation"]))

# heat pump regions and hatch
hp = load("heat-pump-outdoor-unit")
print("HP regions:")
for r in hp["asset"]["thermal"]["regions"]:
    op = r.get("operating")
    print(" ", r["id"], "model", r["model"], "area", r["areaM2"], "heat", (op or {}).get("heatW"), "exp", r.get("exposedFraction"))
rep = hp["asset"]["representations"]["200"]
print("HP 200 hatches", [h["id"] for h in rep.get("hatches", [])])
sh = rep["geometry"]["elevation"][2]
print("HP elev[2]", sh.get("id"), sh.get("hatch"), sh.get("kind"))

# panel radiator thermal env and structure
pr = load("panel-radiator")
env = pr["asset"]["thermal"].get("environment")
print("RAD env keys", None if env is None else {k: env[k] for k in env if "Temp" in k or k in ("skyTempC", "groundTempC")})
print("RAD supports", pr["asset"]["structure"]["supports"])
print("RAD dynamics", pr["asset"]["structure"].get("dynamics"))
for pose in pr["asset"]["poses"]:
    if pose.get("key"):
        continue
    print(" pose", pose["id"], "contacts", pose["contacts"], "loads", [(l["id"], l.get("fxN"), l.get("fyN"), l.get("anchor"), l.get("bone")) for l in pose["loads"]])
print("RAD clips", [(c["id"], c["durationS"], c.get("dynamics"), len(c["contacts"])) for c in pr["asset"]["clips"]])

# vehicles dynamics
for aid in ("delivery-van", "motor-scooter"):
    a = load(aid)["asset"]
    print(aid, "dynamics", a["structure"].get("dynamics"), "supports", a["structure"]["supports"])
    for c in a["clips"]:
        print(" clip", c["id"], "dur", c["durationS"], "phases", [(p["id"], p.get("ballistic"), p["fromS"], p["toS"]) for p in c["phases"]], "contacts", [(s["anchor"], s["fromS"], s["toS"]) for s in c["contacts"]])
