"""🦴 Dairy cow girdles on the 3D rest placement: the shoulder blades lie on the ribcage beside the withers and run outwards to the shoulder joints, the hip bones run from the sacroiliac joints to the hip joints, and the limbs hang from them at their true breadth.

Usage: python3 rework-dairy-cow-girdle.py <asset document> (in place, after the rig, body and structure chisels)
Hand-chosen numbers: dorsal border of the scapula 0.09 m beside the median plane, shoulder joint 0.198 m, sacroiliac joint 0.07 m beside the plane and 0.235 m up the pelvis bone, hip joint 0.15 m (0.30 m between the acetabula).
The scapula bone is inclined outwards by its true 14 degrees (`INCLINED_SCAPULA = True`): it runs from the dorsal border 0.09 m beside the plane to the shoulder joint at 0.198 m and swings in its own plane, so a protracted limb circumducts slightly as in the animal; the poses are not compensated (`UPRIGHT = False`), which keeps the flat limb drawings within the print minimum. The walk advance is the planted travel measured with this placement.
Every solid keeps its world place: its coordinates are re-expressed in the new frame of its bone (see `rig-lateral-rest.md`, 1.6).
"""
import copy
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCAPULA_TOP, SHOULDER, SACROILIAC, SACROILIAC_UP, HIP = 0.09, 0.198, 0.07, 0.235, 0.15
INCLINED_SCAPULA = True
UPRIGHT = False
helper = {"math": math}
text = open(os.path.join(HERE, "rig-lateral-standing-adult.py"), encoding="utf-8").read()
exec(text[text.index("\nDEG = "): text.index("\ndef convert(")], helper)
globals().update({key: value for key, value in helper.items() if not key.startswith("__")})


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf-8"))
    asset = document["asset"]
    bones = asset["rig"]["bones"]
    by = {bone["id"]: bone for bone in bones}
    before = frames(bones)
    sagittal = copy.deepcopy(bones)
    for tag, sign in (("n", -1), ("f", 1)):
        scapula, humerus = by["scapula-f" + tag], by["humerus-f" + tag]
        scapula["z"] = sign * SCAPULA_TOP
        if INCLINED_SCAPULA:
            scapula["abductionDeg"] = round(sign * math.degrees(math.atan((SHOULDER - SCAPULA_TOP) / scapula["lengthM"])), 4)
            scapula["lengthM"] = round(math.hypot(scapula["lengthM"], SHOULDER - SCAPULA_TOP), 5)
        now = frames(bones)
        blade = now["scapula-f" + tag]
        joint = [blade[0][i] + apply(blade[1], [scapula["lengthM"], 0, 0])[i] for i in range(3)]
        joint[2] = sign * SHOULDER
        humerus.update(fields(blade, joint, before["humerus-f" + tag][1]))
        thigh = by["thigh-h" + tag]
        pelvis = now["pelvis"]
        start = [pelvis[0][i] + apply(pelvis[1], [SACROILIAC_UP, 0, sign * SACROILIAC])[i] for i in range(3)]
        socket = [before["thigh-h" + tag][0][0], before["thigh-h" + tag][0][1], sign * HIP]
        along = [socket[i] - start[i] for i in range(3)]
        side = "Near" if tag == "n" else "Far"
        seite = "Nahes" if tag == "n" else "Fernes"
        ilium = {"id": "ilium-h" + tag, "parent": "pelvis", "title": {"en": side + " hip bone (sacroiliac joint to hip joint)", "de": seite + " Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)"}}
        ilium.update(placed(pelvis, start, along, (0.0, 0.0, 1.0)))
        ilium["lengthM"] = round(math.sqrt(sum(c * c for c in along)), 5)
        ilium["joint"] = {"type": "revolute", "minDeg": -2, "maxDeg": 2}
        bones.insert(bones.index(thigh), ilium)
        now = frames(bones)
        thigh["parent"] = ilium["id"]
        thigh.update(fields(now[ilium["id"]], socket, before["thigh-h" + tag][1]))
    after = frames(bones)

    def carry(entry, source, target):
        (old_origin, old_basis), (new_origin, new_basis) = source, target
        turned = mul(transpose(new_basis), old_basis)
        spin = quaternion(turned)
        same = all(abs(turned[i][j] - (1 if i == j else 0)) < 1e-9 for i in range(3) for j in range(3))

        def point(p):
            world = [old_origin[i] + apply(old_basis, p)[i] for i in range(3)]
            return r5(apply(transpose(new_basis), [world[i] - new_origin[i] for i in range(3)]))

        if entry["kind"] == "capsule":
            entry["p0"], entry["p1"] = point(entry["p0"]), point(entry["p1"])
        elif entry["kind"] == "loft":
            for section in entry["sections"]:
                section["at"] = point(section["at"])
                if not same:
                    section["rotation"] = r5(product(spin, section.get("rotation", [0, 0, 0, 1])))
        elif entry["kind"] == "ellipsoid":
            transform = entry.setdefault("transform", {})
            transform["translation"] = point(transform.get("translation", [0, 0, 0]))
            if not same:
                transform["rotation"] = r5(product(spin, transform.get("rotation", [0, 0, 0, 1])))
        else:
            raise SystemExit("solid %s of kind %s on %s is not carried" % (entry["id"], entry["kind"], entry["bone"]))

    moved = [bone for bone in before if max(abs(before[bone][0][i] - after[bone][0][i]) for i in range(3)) > 1e-7 or max(abs(before[bone][1][i][j] - after[bone][1][i][j]) for i in range(3) for j in range(3)) > 1e-7]
    print("[DEBUG] moved frames", moved)
    for scale in asset["volumes"].values():
        for name in ("body", "thermal", "structure"):
            for entry in scale[name]:
                if "kind" not in entry:
                    continue
                bone = entry["bone"]
                ident = entry["id"]
                if bone == "pelvis" and name == "structure" and ident.split("-")[0] in ("near", "far") and (ident.endswith("-ilium") or ident.endswith("-hip-bone")):
                    target = "ilium-h" + ("n" if ident.startswith("near") else "f")
                    carry(entry, before["pelvis"], after[target])
                    entry["bone"] = entry["member"] = target
                elif bone in moved:
                    carry(entry, before[bone], after[bone])
    def upright(table):
        was, now = frames(sagittal, table), frames(bones, table)
        for tag in ("n", "f"):
            if abs(table.get("scapula-f" + tag, {}).get("angleDeg", 0)) < 1e-9:
                continue
            name = "humerus-f" + tag
            rest = turn(by[name]["angleDeg"], by[name].get("abductionDeg", 0.0), by[name].get("twistDeg", 0.0))
            local = mul(transpose(now["scapula-f" + tag][1]), was[name][1])
            a, b, t = angles(mul(transpose(rest), local))
            entry = dict(table.get(name, {}))
            entry.update({"angleDeg": round(a, 5) + 0.0, "abductionDeg": round(b, 5) + 0.0, "twistDeg": round(t, 5) + 0.0})
            table[name] = entry
        return table

    poses = {}
    for pose in asset["poses"]:
        poses[pose["id"]] = copy.deepcopy(pose["bones"])
        if INCLINED_SCAPULA and UPRIGHT:
            upright(pose["bones"])
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if not (INCLINED_SCAPULA and UPRIGHT) or not key.get("bones") or not any(name.startswith(("scapula-", "humerus-")) for name in key["bones"]):
                continue
            merged = upright({**copy.deepcopy(poses[key["pose"]]), **copy.deepcopy(key["bones"])})
            for name in ("humerus-fn", "humerus-ff"):
                if name in merged:
                    key["bones"][name] = merged[name]
    for clip in asset["clips"]:
        if INCLINED_SCAPULA and clip["id"] == "walk-cycle":
            clip["advanceM"] = 1.0259
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    for name in ("scapula-fn", "humerus-fn", "ilium-hn", "thigh-hn", "hoof-hn", "hoof-ff"):
        print("[DEBUG]", name, "origin", r5(after[name][0]), "u", r5([after[name][1][i][0] for i in range(3)]), "w", r5([after[name][1][i][2] for i in range(3)]))


main()
