"""🌱 Root fixes of humans/parent-with-stroller outside `volumes`, stated as old -> new values: canopy hinge of the rig, toddler centre of mass, the fold angle of the canopy in `checking-child`, breath anchors on the modelled faces, and (when `r3-stroller-2d.json` exists) the redrawn 2D geometry and corrected region areas.

Usage: python3 r3-stroller-roots.py [--apply]   (without --apply the candidate goes to 🗑️generated/r3-parent-with-stroller/🔣️.json)
Only the members that change are rewritten, as the same indent-2 blocks the file uses; the file is re-read right before writing.
"""
import json, os, sys

T = os.path.dirname(os.path.abspath(__file__))
CAT = os.path.join(T, *[".."] * 7, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "parent-with-stroller", "🔣️.json")
OUT = os.path.join(T, "🗑️generated", "r3-parent-with-stroller", "🔣️.json")
EXTRA = os.path.join(T, "r3-stroller-2d.json")

BONES = {"stroller-hood": {"x": (0.18, 0.19), "y": (-0.12, -0.225), "lengthM": (0.46, 0.29)}, "stroller": {"angleDeg": (0, 90)}}
JOINTS = {"stroller": {"minDeg": (-60, -150), "maxDeg": (60, -30)}}
REST_SHIFT = {"stroller": -90}
MASS = {"stroller-hood": {"comX": (0.22, 0.085), "comY": (0.08, 0.18)}, "child": {"comX": (0.18, 0.025), "comY": (0, 0.11)}}
ANCHORS = {"mouth": {"x": (0.1134, 0.083), "y": (-0.0945, -0.092)}, "nose": {"x": (0.1547, 0.107), "y": (-0.1123, -0.1)}, "child-mouth": {"x": (0.06, 0.045), "y": (0.312, 0.288)}, "child-nose": {"x": (0.068, 0.056), "y": (0.337, 0.313)}}
POSES = {"checking-child": {"stroller-hood": (95, 40)}}


def block(key, value):
    return '    "%s": %s' % (key, json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n    "))


def put(obj, field, pair, log, where):
    old, new = pair
    if obj[field] == new:
        return
    assert obj[field] == old, (where, field, obj[field], old)
    obj[field] = new
    log.append("%s.%s: %s -> %s" % (where, field, old, new))


def main():
    text = open(CAT, encoding="utf-8").read()
    asset = json.loads(text)["asset"]
    before = {k: block(k, v) for k, v in asset.items() if k != "volumes"}
    log = []
    shifted = any(b["id"] in REST_SHIFT and b["angleDeg"] == BONES[b["id"]]["angleDeg"][1] for b in asset["rig"]["bones"])
    if not shifted:
        for p in asset["poses"]:
            for bone, d in REST_SHIFT.items():
                if bone in p.get("bones", {}):
                    old = p["bones"][bone]["angleDeg"]
                    p["bones"][bone]["angleDeg"] = round(old + d, 4)
                    log.append("poses.%s.%s.angleDeg: %s -> %s" % (p["id"], bone, old, p["bones"][bone]["angleDeg"]))
        for c in asset["clips"]:
            for k in c["keys"]:
                for bone, d in REST_SHIFT.items():
                    assert bone not in k.get("bones", {}), (c["id"], bone)
    for b in asset["rig"]["bones"]:
        for f, p in JOINTS.get(b["id"], {}).items():
            put(b["joint"], f, p, log, "rig.%s.joint" % b["id"])
        for f, p in BONES.get(b["id"], {}).items():
            put(b, f, p, log, "rig." + b["id"])
        for f, p in MASS.get(b["id"], {}).items():
            put(b["mass"], f, p, log, "rig.%s.mass" % b["id"])
    for a in asset["anchors"]:
        for f, p in ANCHORS.get(a["id"], {}).items():
            put(a, f, p, log, "anchors." + a["id"])
    for p in asset["poses"]:
        for bone, pair in POSES.get(p["id"], {}).items():
            put(p["bones"][bone], "angleDeg", pair, log, "poses.%s.%s" % (p["id"], bone))
    if os.path.exists(EXTRA):
        extra = json.load(open(EXTRA, encoding="utf-8"))
        for scale, rep in extra.get("representations", {}).items():
            for k, v in rep.items():
                if asset["representations"][scale].get(k) != v:
                    asset["representations"][scale][k] = v
                    log.append("representations.%s.%s redrawn" % (scale, k))
        for rid, pair in extra.get("areas", {}).items():
            for r in asset["thermal"]["regions"]:
                if r["id"] == rid:
                    put(r, "areaM2", tuple(pair), log, "thermal.regions." + rid)
        for k, pair in extra.get("dimensions", {}).items():
            put(asset["dimensions"], k, tuple(pair), log, "dimensions")
    for k, old in before.items():
        new = block(k, asset[k])
        if new != old:
            assert text.count(old) == 1, k
            text = text.replace(old, new)
    json.loads(text)
    if "--apply" in sys.argv:
        now = open(CAT, encoding="utf-8").read()
        cur = json.loads(now)["asset"]
        for k, old in before.items():
            new = block(k, asset[k])
            if new != old:
                o = block(k, cur[k])
                assert now.count(o) == 1, k
                now = now.replace(o, new)
        json.loads(now)
        tmp = CAT + ".r3tmp"
        open(tmp, "w", encoding="utf-8").write(now)
        os.replace(tmp, CAT)
        print("applied to catalog")
    else:
        os.makedirs(os.path.dirname(OUT), exist_ok=True)
        open(OUT, "w", encoding="utf-8").write(text)
        print("candidate", OUT)
    print("\n".join(log) if log else "nothing to change")


main()
