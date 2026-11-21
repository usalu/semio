"""🛏️ Round 5 dairy cow: in the pose `lying-resting` the rigid udder hung 0.12 m below the plane the cow lies on (its teats 0.14 m); the udder is drawn up into the belly by a translation offset of the udder bone so that its floor sits on that plane, as the real udder is pressed up and aside under a lying cow.

Usage: python3 r5-dairy-cow-lying.py [--candidate | --apply]
"""
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig", os.path.join(HERE, "r5-dairy-cow-rig.py"))
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)
FLOOR = [(0.3, 0.492), (0.1, 0.492), (0.26, 0.54), (0.05, 0.53)]
CLEAR = 0.005


def change(asset, log):
    poses = json.loads(json.dumps(asset["poses"]))
    bones = asset["rig"]["bones"]
    rest = R.fk(bones)
    anchors = {x["id"]: x for x in asset["anchors"]}
    p = next(x for x in poses if x["id"] == "lying-resting")
    f = R.fk(bones, p["bones"], p.get("root"))
    ground = min(R.to_world(f[anchors[c]["bone"]], anchors[c]["x"], anchors[c]["y"])[1] for c in p["contacts"])
    low = min(R.to_world(f["udder"], *R.to_local(rest["udder"], x, y - R.HIP))[1] for x, y in FLOOR)
    lift = ground + CLEAR - low
    pa = math.radians(f["pelvis"][2])
    dx, dy = lift * math.sin(pa), lift * math.cos(pa)
    old = p["bones"].get("udder")
    p["bones"]["udder"] = {**(old or {}), "dx": R.r5(dx), "dy": R.r5(dy)}
    log.append("pose lying-resting udder: %s -> %s (lift %.3f m)" % (old, p["bones"]["udder"], lift))
    return {"poses": poses}


def main():
    text = open(R.FILE, encoding="utf-8").read()
    doc = json.loads(text)
    log = []
    members = change(doc["asset"], log)
    print("\n".join(log))
    if "--candidate" in sys.argv:
        doc["asset"].update(members)
        json.dump(doc, open(R.CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        print("candidate", R.CANDIDATE)
    if "--apply" in sys.argv:
        for key, new in members.items():
            old = R.block(key, doc["asset"][key])
            if text.count(old) != 1:
                raise SystemExit("member %s not found exactly once" % key)
            text = text.replace(old, R.block(key, new))
        json.loads(text)
        open(R.FILE, "w", encoding="utf-8").write(text)
        print("written", R.FILE)


if __name__ == "__main__":
    main()
