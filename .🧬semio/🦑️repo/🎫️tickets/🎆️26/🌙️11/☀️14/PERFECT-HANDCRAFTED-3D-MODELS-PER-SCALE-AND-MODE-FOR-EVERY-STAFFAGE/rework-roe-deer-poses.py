"""🦌 roe-deer poses: `grazing` stands on four straight legs, the forelegs set one ahead and one back to lower the withers, the neck reaching down, instead of crouching on folded forelegs with the upper arm pointing backwards.

Usage: python3 rework-roe-deer-poses.py            writes the candidate `🗑️generated/rework-roe-deer/roe-deer/🔣️.json`
       python3 rework-roe-deer-poses.py --apply    re-reads the catalog file and writes `poses`
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("roe_rig", os.path.join(T, "rework-roe-deer-rig.py"))
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)

NEAR_FORELEG_AHEAD_DEG = 27.0
SPINE = {"lumbar": -4.0, "thorax": -6.0}
HEAD_BELOW_HORIZON_DEG = 82.0
MUZZLE_ABOVE_GROUND_M = 0.06
FORE = ("scapula", "humerus", "forearm", "cannon", "hoof")


def at(asset, offsets, anchor_id):
    anchor = next(anchor for anchor in asset["anchors"] if anchor["id"] == anchor_id)
    origin, basis = R.K.frames(asset["rig"]["bones"], {bone: {"angleDeg": angle} for bone, angle in offsets.items()})[anchor["bone"]]
    return R.add(origin, R.K.apply(basis, [anchor["x"], anchor["y"], 0.0]))


def bisect(f, low, high):
    if (f(low) > 0) == (f(high) > 0):
        return low if abs(f(low)) < abs(f(high)) else high
    for _ in range(60):
        mid = (low + high) / 2
        if (f(low) > 0) == (f(mid) > 0):
            low = mid
        else:
            high = mid
    return (low + high) / 2


def convert(asset):
    """Solves the pose on the rig: hind legs as in `alert`, root pitch and far foreleg so that all four soles stand on one ground, neck so that the muzzle hovers above it."""
    poses = {pose["id"]: pose for pose in asset["poses"]}
    alert, grazing = poses["alert"]["bones"], poses["grazing"]["bones"]
    base = {bone: offset["angleDeg"] for bone, offset in alert.items() if bone[:4] not in ("neck", "head", "ear-")}
    ground = at(asset, base, "contact-hn")[1]

    def stand(pitch, far):
        offsets = {**base, **SPINE, "pelvis": pitch}
        for side in ("n", "f"):
            offsets[f"thigh-h{side}"] = base.get(f"thigh-h{side}", 0.0) - pitch
        for side, swing in (("n", NEAR_FORELEG_AHEAD_DEG), ("f", far)):
            offsets[f"scapula-f{side}"] = swing
            offsets[f"hoof-f{side}"] = -(pitch + sum(SPINE.values()) + swing)
        return offsets

    pitch = bisect(lambda value: at(asset, stand(value, 0.0), "contact-fn")[1] - ground, -25.0, 5.0)
    far = bisect(lambda value: at(asset, stand(pitch, value), "contact-ff")[1] - ground, -45.0, 0.0)
    offsets = stand(pitch, far)
    bones = {bone["id"]: bone for bone in asset["rig"]["bones"]}
    trunk = bones["pelvis"]["angleDeg"] + bones["lumbar"]["angleDeg"] + bones["thorax"]["angleDeg"] + pitch + sum(SPINE.values())

    def reach(neck):
        head = -HEAD_BELOW_HORIZON_DEG - (trunk + bones["neck"]["angleDeg"] + neck + bones["head"]["angleDeg"])
        return {**offsets, "neck": neck, "head": head, "jaw": -3.0}

    neck = bisect(lambda value: at(asset, reach(value), "nose")[1] - ground - MUZZLE_ABOVE_GROUND_M, -128.0, -40.0)
    solved = reach(neck)
    root = solved.pop("pelvis")
    poses["grazing"]["root"] = {"angleDeg": round(root, 4)}
    poses["grazing"]["bones"] = {**{bone: {"angleDeg": round(angle, 4)} for bone, angle in solved.items() if abs(angle) > 1e-6}, **{bone: offset for bone, offset in grazing.items() if bone.startswith("ear-")}}
    poses["grazing"]["description"] = {
        "en": "Standing on four straight legs with one foreleg set ahead and the other back, which lowers the withers; back slightly rounded, neck stretched forward and down so the muzzle hovers just above the ground vegetation; ears relaxed sideways.",
        "de": "Stand auf vier gestreckten Läufen, ein Vorderlauf vorgestellt, der andere zurückgestellt, wodurch der Widerrist sinkt; Rücken leicht gerundet, Hals nach vorn unten gestreckt, sodass der Äser dicht über der Bodenvegetation schwebt; Lauscher entspannt zur Seite.",
    }
    print("grazing", json.dumps({"root": poses["grazing"]["root"], **poses["grazing"]["bones"]}), [round(at(asset, {**solved, "pelvis": root}, anchor)[1] - ground, 4) for anchor in ("contact-hn", "contact-hf", "contact-fn", "contact-ff", "nose")])


def main():
    text = open(R.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    convert(document["asset"])
    if "--apply" in sys.argv:
        open(R.CATALOG, "w", encoding="utf8").write(R.splice(text, "poses", document["asset"]["poses"]))
    else:
        os.makedirs(os.path.dirname(R.CANDIDATE), exist_ok=True)
        json.dump(document, open(R.CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
