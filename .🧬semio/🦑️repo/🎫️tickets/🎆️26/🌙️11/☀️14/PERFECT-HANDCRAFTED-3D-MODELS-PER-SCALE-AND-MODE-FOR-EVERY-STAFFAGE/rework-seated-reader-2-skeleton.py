"""🦴 Stage 2 of the seated reader: the skeleton is one connected load path at every scale.

Hand-chosen numbers (metres). Shoulder: the acromion runs on to the acromioclavicular joint, the spine of the scapula ties blade and acromion, the glenoid (neck 24 mm, socket 34 mm) carries the humeral head. Hip: the acetabulum (42 mm) joins ilium, pubis and ischium around the femoral head. Hand: metacarpals reach their knuckles (head 11 mm, shaft 8 mm), phalanges swell at the joints, the thumbs lie on the notebook board they hold. Ischial tuberosities bear on the seat at 1:100 (7 mm lower), 1:200 and 1:500 (bar from the hip joint down to the seat). Body 1:100 and 1:200: the shank runs into the shoe collar. 1:500: the knee end of the thigh is 100 mm, the smallest part an office laser prints at that scale, in the solid and in the drawing.

Run: `python3 rework-seated-reader-2-skeleton.py` writes the candidate, `--apply` the volumes source and `representations` of the catalog.
"""
import copy
import importlib.util
import json
import os
import sys

from rework_03_lib import T, catalog_path, source_path

spec = importlib.util.spec_from_file_location("stage1", os.path.join(T, "rework-seated-reader-1-rig.py"))
stage1 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stage1)
rig, rod = stage1.rig, stage1.rod

A = "humans/seated-reader"
SIDES = {"l": 1, "r": -1}
ACROMION = ((0.406, 0.059, 0.102), (0.434, 0.019, 0.147), (0.44, -0.028, 0.17))
GLENOID = ((0.38, 0.047, 0.122, 0.013), (0.403, -0.005, 0.15, 0.015), (0.406, -0.028, 0.158, 0.017))
SCAPULAR_SPINE = ((0.385, 0.078, 0.06, 0.006), (0.395, 0.066, 0.085, 0.006), (0.406, 0.059, 0.102, 0.0066))
THUMB = ((0.016, -0.004, 0.024), (0.04, -0.008, 0.043), (0.06, -0.022, 0.05), (0.076, -0.036, 0.044), (0.088, -0.038, 0.034))
PHALANX_RADII = (0.0056, 0.004, 0.005, 0.005, 0.0046, 0.0046, 0.004)
BOARD_HALF = 0.006
ILIUM_BODY = ((0.05118, 0.04807, 0.00702, 0.012), (0.065, 0.02, 0.003, 0.014), (0.0727, 0, 0, 0.016))
AURICULAR = ((0, 0, 0, 0.012), (0.03, 0.03, 0.005, 0.011), (0.05118, 0.04807, 0.00702, 0.011))


def turned(q, v):
    """🌀 A vector turned by the quaternion [x, y, z, w]."""
    x, y, z, w = q
    m = [[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)], [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)], [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]]
    return rig.apply(m, v)


def find(solids, solid_id):
    """🔎 The solid of a list by id."""
    return next(s for s in solids if s["id"] == solid_id)


def after(solids, solid_id, *fresh):
    """📌 Inserts solids behind the named one."""
    at = solids.index(find(solids, solid_id)) + 1
    solids[at:at] = list(fresh)


def to_world(frames, bone, p):
    """🌍 A point of a bone frame in world coordinates."""
    return [frames[bone][0][i] + rig.apply(frames[bone][1], p)[i] for i in range(3)]


def to_local(frames, bone, p):
    """🦴 A world point in a bone frame."""
    return [round(c, 5) for c in rig.apply(rig.transpose(frames[bone][1]), [p[i] - frames[bone][0][i] for i in range(3)])]


def on_board(posed, hand, tip, radius):
    """📓 The point where a thumb tip of that radius lies on the face of the notebook board."""
    p = rig.apply(rig.transpose(posed["pad"][1]), [to_world(posed, hand, tip)[i] - posed["pad"][0][i] for i in range(3)])
    p[1] = (1 if p[1] > 0 else -1) * (BOARD_HALF + radius - 0.002)
    return to_local(posed, hand, to_world(posed, "pad", p))


def convert(a):
    """🛠️ The whole stage on an asset whose `volumes` are the source."""
    bones = a["rig"]["bones"]
    rest = rig.frames(copy.deepcopy(bones))
    posed = rig.frames(copy.deepcopy(bones), a["poses"][0]["bones"])
    length = {b["id"]: b["lengthM"] for b in bones}
    volumes = a["volumes"]
    if any(s["id"] == "glenoid-l" for s in volumes["50"]["structure"]):
        raise SystemExit("already converted")

    def girdle(bone, p, sign):
        return to_local(rest, bone, to_world(rest, "lumbar", [p[0], p[1], sign * p[2]]))

    for scale in ("50", "100"):
        st = volumes[scale]["structure"]
        after(st, "sternum", {"id": "manubrium", "kind": "ellipsoid", "bone": "torso", "material": "bone", "role": "body", "tone": 0.62, "radii": [0.018, 0.008, 0.026], "transform": {"translation": [0.25, -0.101, 0]}, "member": "torso", "part": "member"})
        for side, sign in SIDES.items():
            cl, il = f"clavicle-{side}", f"ilium-{side}"
            tip = [round(length[cl] - 0.002, 5), 0, 0]
            if scale == "50":
                find(st, f"acromion-{side}")["sections"].append({"at": tip, "rx": 0.0075, "ry": 0.0075})
                after(st, f"scapula-{side}", rod(f"scapular-spine-{side}", cl, [(*girdle(cl, p, sign), p[3]) for p in SCAPULAR_SPINE]))
            else:
                after(st, f"scapula-{side}", rod(f"acromion-{side}", cl, [(*girdle(cl, p, sign), 0.0077) for p in ACROMION] + [(*tip, 0.0077)]))
            blade = find(st, f"scapula-{side}")
            inside = [blade["transform"]["translation"][k] + turned(blade["transform"]["rotation"], blade["sections"][1]["at"])[k] for k in range(3)]
            after(st, f"acromion-{side}", rod(f"glenoid-{side}", cl, [(*inside, 0.008)] + [(*girdle(cl, p, sign), p[3]) for p in GLENOID]))

            def hip(p):
                w = to_world(rest, "ilium-l", list(p[:3]))
                return (*to_local(rest, il, [w[0], w[1], sign * w[2]]), p[3])

            after(st, f"ilium-{side}", rod(f"ilium-body-{side}", il, [hip(p) for p in ILIUM_BODY]), rod(f"auricular-{side}", il, [hip(p) for p in AURICULAR]))
            after(st, f"ilium-{side}", {"id": f"acetabulum-{side}", "kind": "ellipsoid", "bone": il, "material": "bone", "role": "body", "tone": 0.62, "radii": [0.021, 0.021, 0.021], "transform": {"translation": [round(length[il] - 0.02, 5), 0, 0]}, "member": il, "part": "member"})
            hand = f"hand-{side}"
            if scale == "50":
                for i in range(1, 5):
                    mc, ph = find(st, f"metacarpal-{i}-{side}"), find(st, f"phalanges-{i}-{side}")
                    base, head = mc["sections"]
                    waist = copy.deepcopy(base)
                    waist["at"] = [round((base["at"][k] + head["at"][k]) / 2, 5) for k in range(3)]
                    knuckle = copy.deepcopy(ph["sections"][0])
                    for section, r in ((base, 0.0052), (waist, 0.004), (head, 0.0046), (knuckle, 0.0056)):
                        section["rx"] = section["ry"] = r
                    mc["sections"] = [base, waist, head, knuckle]
                    if len(ph["sections"]) == len(PHALANX_RADII):
                        for section, r in zip(ph["sections"], PHALANX_RADII):
                            section["rx"] = section["ry"] = r
                thumb = find(st, f"thumb-bones-{side}")
                thumb["sections"].append({"at": on_board(posed, hand, thumb["sections"][-1]["at"], 0.0042), "rx": 0.0042, "ry": 0.0042})
            else:
                points = [(p[0], p[1], -sign * p[2], 0.0077) for p in THUMB]
                points.append((*on_board(posed, hand, list(points[-1][:3]), 0.0077), 0.0077))
                after(st, f"hand-bones-{side}", rod(f"thumb-bones-{side}", hand, points))
                seat_bone = find(st, f"ischium-{side}")["sections"][2]
                w = to_world(posed, il, seat_bone["at"])
                seat_bone["at"] = to_local(posed, il, [w[0], w[1] - 0.007, w[2]])
    for scale, drop, r in (("200", 0.0165, None), ("500", None, 0.0377)):
        for side, sign in SIDES.items():
            il = f"ilium-{side}"
            bar = find(volumes[scale]["structure"], f"hip-bar-{side}")
            last = bar["sections"][-1]
            if drop:
                w = to_world(posed, il, last["at"])
                target = [w[0], w[1] - drop, w[2]]
            else:
                target = [-0.019, 0.44, sign * 0.06]
            fresh = copy.deepcopy(last)
            fresh["at"] = to_local(posed, il, target)
            bar["sections"].append(fresh)
    for scale in ("100", "200"):
        for side in SIDES:
            shank = find(volumes[scale]["body"], f"shank-{side}")
            ankle = copy.deepcopy(shank["sections"][-1])
            ankle["at"] = [0.425, ankle["at"][1], ankle["at"][2]]
            ankle["rx"], ankle["ry"] = 0.036, 0.04
            shank["sections"].append(ankle)
    for side in SIDES:
        thigh = find(volumes["500"]["body"], f"thigh-{side}")
        thigh["sections"][-2]["rx"], thigh["sections"][-2]["ry"] = 0.058, 0.058
        thigh["sections"][-1]["rx"], thigh["sections"][-1]["ry"] = 0.05, 0.05
        for shape in [s for shapes in a["representations"]["500"]["geometry"].values() for s in shapes]:
            if shape["id"] == f"thigh-{side}":
                for point in shape["points"]:
                    if point == [0.43, 0.0577]:
                        point[1] = 0.0612
                    if point == [0.47, 0.0407]:
                        point[1] = 0.0532
    return a


def main():
    """🚀 Candidate, or with `--apply` the source and the catalog member."""
    catalog = catalog_path(A)
    original = text = open(catalog, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(source_path(A), encoding="utf8").read()
    document["asset"]["volumes"] = json.loads(source_text)["volumes"]
    convert(document["asset"])
    if "--apply" not in sys.argv:
        json.dump(document, open(stage1.CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(stage1.CANDIDATE)
        return
    text = stage1.spliced(text, "representations", before["representations"], document["asset"]["representations"])
    if open(catalog, encoding="utf8").read() != original or open(source_path(A), encoding="utf8").read() != source_text:
        raise SystemExit("a file changed while converting: run again")
    open(catalog, "w", encoding="utf8").write(text)
    json.dump({"volumes": document["asset"]["volumes"]}, open(source_path(A), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(source_path(A), "a", encoding="utf8").write("\n")
    print("applied")


if __name__ == "__main__":
    main()
