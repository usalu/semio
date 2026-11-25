#!/usr/bin/env python3
"""🗒️ Writes the loft plan of a walker: `limbs-plan.py <file.json> <out.json> [--only <regex of bones>] [--scales 50,100] [--coat] [--views front,plan]` lists for every closed drawn part of an arm, hand, thigh, shank, foot and toe in the named views the body solids of the band that make it: the loft of the same id, the palm with every finger of the hand, the sole with its heel, the sock as the ankle; with --coat the plan `coat` as the trunk, waist, centre and both skirts."""
import json, re, sys
a = json.load(open(sys.argv[1]))["asset"]
opt = lambda k, d: sys.argv[sys.argv.index(k) + 1] if k in sys.argv else d
only = re.compile(opt("--only", r"^(arm|forearm|hand|thigh|shank|foot|toe)-[lr]$")); views = opt("--views", "front,plan").split(",")
parent = {b["id"]: b.get("parent") for b in a["rig"]["bones"]}
def under(b, top):
    while b is not None:
        if b == top: return True
        b = parent.get(b)
    return False
out = []
for s in opt("--scales", "50,100,200,500").split(","):
    body = a["volumes"][s]["body"]; g = a["representations"][s]["geometry"]
    ids = {x["id"] for x in body}
    for bone in [b for b in parent if only.match(b)]:
        for id in dict.fromkeys(x["id"] for v in views for x in g[v] if x["bone"] == bone and x.get("closed", True)):
            side = id[-1]
            if bone.startswith("hand-"): solids = [x["id"] for x in body if under(x["bone"], bone) and x["id"].startswith("hand-")]
            elif id in ("shoe-sole-l", "shoe-sole-r"): solids = [x for x in (id, "shoe-heel-" + side) if x in ids]
            elif id.startswith("ankle-"): solids = ["sock-" + side]
            else: solids = [id]
            vs = [v for v in views if any(x["id"] == id and x["bone"] == bone for x in g[v])]
            if solids and all(x in ids for x in solids): out.append({"scale": int(s), "bone": bone, "id": id, "views": vs, "solids": solids, **({"stations": 14} if len(solids) > 3 else {})})
            else: print("no solid for", s, bone, id)
    if "--coat" in sys.argv: out.append({"scale": int(s), "bone": "torso", "id": "coat", "views": ["plan"], "solids": [x for x in ("coat", "coat-waist", "coat-centre", "coat-skirt-l", "coat-skirt-r") if x in ids], "stations": 16})
open(sys.argv[2], "w").write("[\n" + ",\n".join(json.dumps(x) for x in out) + "\n]\n")
print(len(out), "lofts")
