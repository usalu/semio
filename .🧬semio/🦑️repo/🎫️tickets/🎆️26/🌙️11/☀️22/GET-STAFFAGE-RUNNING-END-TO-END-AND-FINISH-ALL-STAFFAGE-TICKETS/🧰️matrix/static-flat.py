"""🧱 Counts, from the asset documents alone, per category: the front and plan shapes by construction (flat, solid, member, followTip) and the assets in which a flat front or plan shape sits on a bone that a pose turns (angleDeg, abductionDeg or twistDeg on the bone or an ancestor); also the contact, attachment and emitter anchors without z. usage: python3 static-flat.py <catalog-dir>"""
import collections
import glob
import json
import sys

TURNS = ("angleDeg", "abductionDeg", "twistDeg")
kinds = collections.defaultdict(collections.Counter)
at_risk = collections.defaultdict(list)
totals = collections.Counter()
no_z = collections.defaultdict(list)
all_flat = collections.Counter()
for path in sorted(glob.glob(sys.argv[1] + "/*/*/🔣️.json")):
    asset = json.load(open(path, encoding="utf-8"))["asset"]
    category = asset["category"]
    totals[category] += 1
    parent = {bone["id"]: bone.get("parent") for bone in asset["rig"]["bones"]}
    turned = collections.defaultdict(float)
    for pose in asset["poses"]:
        for bone, offset in (pose.get("bones") or {}).items():
            turned[bone] = max(turned[bone], max((abs(offset.get(key) or 0) for key in TURNS), default=0))
    def turn_of(bone):
        worst = 0.0
        while bone is not None:
            worst = max(worst, turned.get(bone, 0.0))
            bone = parent.get(bone)
        return worst
    flat_on_turning = 0
    worst = 0.0
    solids = 0
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        for view in ("front", "plan"):
            for shape in rep["geometry"][view]:
                kind = "solid" if "solid" in shape else "member" if shape.get("kind") == "member" else "followTip" if shape.get("followTip") else "flat"
                kinds[category][kind] += 1
                solids += kind != "flat"
                if kind == "flat" and shape.get("role") != "ground" and turn_of(shape["bone"]) >= 5:
                    flat_on_turning += 1
                    worst = max(worst, turn_of(shape["bone"]))
    if solids == 0:
        all_flat[category] += 1
    if flat_on_turning:
        at_risk[category].append((asset["id"], flat_on_turning, worst))
    missing = [anchor["id"] for anchor in asset["anchors"] if anchor["kind"] in ("contact", "attachment", "emitter") and anchor.get("z") is None]
    if missing:
        no_z[category].append((asset["id"], len(missing)))
print("category\tassets\tflat\tsolid\tmember\tfollowTip\tassets without any front/plan solid\tassets with a flat front/plan shape on a bone turned >= 5 deg\tassets with landmark anchors without z")
for category in sorted(totals):
    k = kinds[category]
    print(f"{category}\t{totals[category]}\t{k['flat']}\t{k['solid']}\t{k['member']}\t{k['followTip']}\t{all_flat[category]}\t{len(at_risk[category])}\t{len(no_z[category])}")
print("total\t%d\t\t\t\t\t%d\t%d\t%d" % (sum(totals.values()), sum(all_flat.values()), sum(len(v) for v in at_risk.values()), sum(len(v) for v in no_z.values())))
