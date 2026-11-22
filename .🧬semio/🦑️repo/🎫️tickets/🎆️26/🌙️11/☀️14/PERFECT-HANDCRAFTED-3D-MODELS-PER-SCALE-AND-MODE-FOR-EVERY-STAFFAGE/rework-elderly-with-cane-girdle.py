"""🦴 elderly-with-cane, second round: shoulder and pelvic girdle on the lateral rest placement and the long bones at their true shaft at every scale.

Hand-chosen numbers (world rest coordinates [ahead, up, left], metres):
- clavicle: unchanged axis (sternoclavicular joint [0.146, 1.3256, ±0.025] to the acromion [0.0961, 1.3656, ±0.170]); new joint axis back (left) / ahead (right) so that `angleDeg` lifts the shoulder; the upper arm hangs from it at the unchanged glenohumeral centre [0.09, 1.3356, ±0.18].
- hip bone (os coxae): from the sacroiliac joint [-0.05, 0.9446, ±0.045] to the centre of the hip socket [0, 0.8696, ±0.09], 0.1008 m; `angleDeg` hikes the hip; the thigh hangs from its tip in its unchanged sagittal frame.
- long bones (humerus, ulna, radius, femur, tibia, fibula): the 1:50 shaft sections at 1:100, 1:200 and 1:500, `slender: true`, reaching the joint stations of the coarse list.

Usage: python3 rework-elderly-with-cane-girdle.py            writes the candidate 🗑️generated/rework-elderly-with-cane/elderly-with-cane/🔣️.json
       python3 rework-elderly-with-cane-girdle.py --apply    re-reads catalog and source and writes `rig` and `volumes` (catalog) and the source
"""
import copy
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
K = importlib.util.module_from_spec(spec)
ARGV = sys.argv
spec.loader.exec_module(K)
CATALOG = os.path.join(K.REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "elderly-with-cane", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "elderly-with-cane.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-elderly-with-cane", "elderly-with-cane", "🔣️.json")

SACROILIAC = (-0.05, 0.9446, 0.045)
BACK = {"l": (-1.0, 0.0, 0.0), "r": (1.0, 0.0, 0.0)}
CLAVICLE_LIMITS = {"minDeg": -5, "maxDeg": 25}
HIP_LIMITS = {"minDeg": -5, "maxDeg": 5}
HIP_TITLE = {"l": {"en": "Left hip bone", "de": "Linkes Hüftbein"}, "r": {"en": "Right hip bone", "de": "Rechtes Hüftbein"}}
HIP_SOLIDS = ("ilium-{s}", "ilium-body-{s}", "pubis-ischium-{s}")
LONG = {"humerus": "arm", "ulna": "forearm", "radius": "forearm", "femur": "thigh", "tibia": "shank", "fibula": "shank"}


def ordered(bone):
    keys = ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint")
    out = {key: bone[key] for key in keys if key in bone}
    out.update({key: value for key, value in bone.items() if key not in out})
    return out


def clean(placement):
    return {key: value for key, value in placement.items() if key not in ("abductionDeg", "twistDeg") or abs(value) > 1e-9}


def moved(solid, turn, shift):
    K.rebind(solid, turn)
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = K.r5([solid["p0"][i] + shift[i] for i in range(3)]), K.r5([solid["p1"][i] + shift[i] for i in range(3)])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = K.r5([section["at"][i] + shift[i] for i in range(3)])
    else:
        translation = solid["transform"]["translation"]
        solid["transform"]["translation"] = K.r5([translation[i] + shift[i] for i in range(3)])


def girdles(asset, volumes_sets):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "hip-l" in by_id:
        return
    for view in ("elevation", "front", "plan"):
        for representation in asset["representations"].values():
            for shape in representation["geometry"][view]:
                if str(shape.get("bone", "")).startswith("clavicle-"):
                    raise SystemExit(f"2D shape {shape['id']} on a clavicle: redraw first")
    for pose in asset["poses"]:
        if any(key.startswith("clavicle-") for key in pose.get("bones", {})):
            raise SystemExit("a pose moves a clavicle: re-solve first")
    old = K.frames(copy.deepcopy(bones))
    turn_of, shift_of = {}, {}
    for side, sign in (("l", 1), ("r", -1)):
        clavicle, arm, thigh = by_id[f"clavicle-{side}"], by_id[f"arm-{side}"], by_id[f"thigh-{side}"]
        u = [old[clavicle["id"]][1][i][0] for i in range(3)]
        clavicle.update(K.placed(old["chest"], old[clavicle["id"]][0], u, BACK[side]))
        clavicle["joint"] = {**clavicle["joint"], **CLAVICLE_LIMITS}
        now = K.frames(bones)
        arm.update({"parent": clavicle["id"], **clean(K.fields(now[clavicle["id"]], old[arm["id"]][0], old[arm["id"]][1]))})
        turn_of[clavicle["id"]] = K.mul(K.transpose(now[clavicle["id"]][1]), old[clavicle["id"]][1])
        shift_of[clavicle["id"]] = [0.0, 0.0, 0.0]
        origin = [SACROILIAC[0], SACROILIAC[1], sign * SACROILIAC[2]]
        socket = old[thigh["id"]][0]
        axis = [socket[i] - origin[i] for i in range(3)]
        hip = {"id": f"hip-{side}", "parent": "pelvis", "title": HIP_TITLE[side], **K.placed(old["pelvis"], origin, axis, BACK[side]), "lengthM": round(math.sqrt(sum(c * c for c in axis)), 5), "joint": {**{key: value for key, value in by_id["pelvis"]["joint"].items() if key not in ("minDeg", "maxDeg")}, "type": "revolute", **HIP_LIMITS}}
        bones.insert(bones.index(thigh), ordered(hip))
        now = K.frames(bones)
        thigh.update({"parent": hip["id"], **clean(K.fields(now[hip["id"]], socket, old[thigh["id"]][1]))})
        for bone in (clavicle, arm, thigh):
            fresh = ordered(bone)
            bone.clear()
            bone.update(fresh)
        back = K.transpose(now[hip["id"]][1])
        turn_of[hip["id"]] = K.mul(back, old["pelvis"][1])
        shift_of[hip["id"]] = K.apply(back, [old["pelvis"][0][i] - origin[i] for i in range(3)])
    new = K.frames(bones)
    for bone_id, (origin, basis) in old.items():
        if bone_id.startswith("clavicle-"):
            continue
        drift = max(abs(new[bone_id][0][i] - origin[i]) for i in range(3))
        turned = max(abs(new[bone_id][1][i][j] - basis[i][j]) for i in range(3) for j in range(3))
        if drift > 3e-5 or turned > 3e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")
    for volumes in volumes_sets:
        for scale, volume in volumes.items():
            for name in ("body", "thermal", "structure"):
                for solid in volume.get(name, []):
                    if solid.get("bone", "").startswith("clavicle-"):
                        moved(solid, turn_of[solid["bone"]], shift_of[solid["bone"]])
                    for side in ("l", "r"):
                        if solid["id"] in [pattern.format(s=side) for pattern in HIP_SOLIDS] and solid.get("bone") == "pelvis":
                            moved(solid, turn_of[f"hip-{side}"], shift_of[f"hip-{side}"])
                            solid["bone"] = f"hip-{side}"
                            if solid.get("part") == "member":
                                solid["member"] = f"hip-{side}"
                    if name == "structure" and solid["id"] == "pelvic-ring":
                        solid["members"] = ["hip-l", "hip-r"]


def long_bones(volumes_sets):
    for volumes in volumes_sets:
        fine = {solid["id"]: solid for solid in volumes["50"]["structure"]}
        for scale in ("100", "200", "500"):
            listed = volumes[scale]["structure"]
            index = {solid["id"]: position for position, solid in enumerate(listed)}
            for name in LONG:
                for side in ("l", "r"):
                    key = f"{name}-{side}"
                    true = copy.deepcopy(fine[key])
                    true["slender"] = True
                    if key in index:
                        coarse = listed[index[key]]
                        first, last = coarse["sections"][0]["at"], coarse["sections"][-1]["at"]
                        if first[0] < true["sections"][0]["at"][0]:
                            true["sections"][0]["at"] = first
                        if last[0] > true["sections"][-1]["at"][0]:
                            true["sections"][-1]["at"] = last
                        listed[index[key]] = true
                    else:
                        partner = {"radius": "ulna", "fibula": "tibia"}[name]
                        at = next(position for position, solid in enumerate(listed) if solid["id"] == f"{partner}-{side}")
                        listed.insert(at + 1, true)
                        index = {solid["id"]: position for position, solid in enumerate(listed)}


def both(half):
    return [[u, -r] for u, r in half] + [[u, r] for u, r in reversed(half)]


CANE_Z = (-0.2267, -0.1997)
HAND_Z = (-0.192, -0.15)
SHAFT = {
    "50": both([(0.01, 0.0133), (0.0324, 0.0132), (0.0349, 0.0112), (0.7317, 0.0092), (0.735, 0.0089)]),
    "100": both([(0.009, 0.0115), (0.0183, 0.0111), (0.7299, 0.0093), (0.735, 0.009)]),
}
FERRULE = {
    "50": both([(0.73, 0.0119), (0.7692, 0.0152), (0.772, 0.014)]),
    "100": both([(0.73, 0.0118), (0.772, 0.0142)]),
}
HANDLE = {
    "50": [[0.0134, 0.0133], [0.0118, 0.0333], [0.0072, 0.0364], [0.0025, 0.0371], [-0.0024, 0.036], [-0.0093, 0.0221], [-0.0156, -0.0171], [-0.0171, -0.0446], [-0.0117, -0.0734], [-0.0013, -0.0929], [0.0058, -0.1023], [0.0098, -0.1044], [0.0172, -0.1026], [0.0205, -0.0987], [0.0208, -0.0957], [0.0135, -0.0671], [0.0109, -0.0449], [0.0124, -0.0134], [0.0134, -0.0133]],
    "100": [[0.0125, 0.012], [0.012, 0.033], [0.002, 0.037], [-0.005, 0.033], [-0.0096, 0.022], [-0.0172, -0.046], [-0.0117, -0.0737], [0.002, -0.0977], [0.011, -0.1045], [0.0185, -0.1015], [0.021, -0.094], [0.011, -0.047], [0.0125, -0.012]],
}
HAND = {
    "50": [[-0.02, -0.027], [-0.02, 0.027], [0, 0.03], [0.02, 0.04], [0.035, 0.052], [0.06, 0.056], [0.072, 0.048], [0.08, 0.042], [0.094, 0.04], [0.1, 0.03], [0.1, -0.02], [0.094, -0.034], [0.08, -0.042], [0.05, -0.04], [0, -0.03]],
    "100": [[-0.02, -0.027], [-0.02, 0.027], [0.02, 0.04], [0.035, 0.052], [0.06, 0.056], [0.08, 0.042], [0.1, 0.034], [0.1, -0.022], [0.08, -0.042], [0, -0.03]],
    "200": [[-0.02, -0.028], [-0.02, 0.03], [0.04, 0.055], [0.08, 0.045], [0.1, 0.035], [0.1, -0.02], [0.08, -0.042], [0, -0.032]],
}
REVOLVED = {"kind": "revolved", "axisBone": "cane", "axis": "u", "axisX": 0}


def shapes2d(asset):
    for scale, representation in asset["representations"].items():
        geometry = representation["geometry"]
        if any("solid" in shape for shape in geometry["front"] if shape.get("bone") == "cane"):
            continue
        elevation = {shape["id"]: shape for shape in geometry["elevation"]}
        if scale in SHAFT:
            shaft = elevation["cane-shaft"]
            shaft["points"] = SHAFT[scale]
            elevation["cane-ferrule"]["points"] = FERRULE[scale]
            handle = {**copy.deepcopy(shaft), "id": "cane-handle", "z": shaft["z"] + 0.05, "points": HANDLE[scale]}
            geometry["elevation"].insert(geometry["elevation"].index(shaft) + 1, handle)
            elevation["cane-handle"] = handle
        for view in ("front", "plan"):
            shapes = geometry[view]
            for shape in list(shapes):
                if shape.get("bone") == "cane" and shape["id"] in ("cane-shaft", "cane-ferrule"):
                    shape["points"] = copy.deepcopy(elevation[shape["id"]]["points"])
                    shape["solid"] = dict(REVOLVED)
                    if shape["id"] == "cane-shaft" and scale in HANDLE:
                        handle = {**copy.deepcopy(shape), "id": "cane-handle", "z": shape["z"] + 0.05, "points": HANDLE[scale], "solid": {"kind": "extruded", "sweep": "w", "z0": CANE_Z[0], "z1": CANE_Z[1]}}
                        shapes.insert(shapes.index(shape) + 1, handle)
                if shape.get("bone") == "hand-r" and shape["id"] == "hand-r" and scale in HAND:
                    shape["points"] = copy.deepcopy(HAND[scale])
                    shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": HAND_Z[0], "z1": HAND_Z[1]}


GIRDLE_TEXT = {
    "50": {"en": "Girdles: each upper arm hangs from the acromial end of its clavicle and each thigh from the socket of its hip bone, which runs from the sacroiliac joint down and out to the hip joint; iliac blade, body of the ilium, pubic and ischial rami are bound to that hip bone, clavicle and scapula to the clavicle, so a lifted shoulder or a hiked hip carries the limb with it.", "de": "Gürtel: jeder Oberarm hängt am Schulterhöhenende seines Schlüsselbeins und jeder Oberschenkel an der Pfanne seines Hüftbeins, das vom Kreuzdarmbeingelenk nach unten außen zum Hüftgelenk läuft; Darmbeinschaufel, Darmbeinkörper, Scham- und Sitzbeinäste sind an dieses Hüftbein gebunden, Schlüsselbein und Schulterblatt an das Schlüsselbein, sodass eine gehobene Schulter oder eine angehobene Hüfte das Glied mitnimmt."},
    "100": {"en": "Girdles and long bones: the hip bones and clavicles carry the limbs as at 1:50; humerus, ulna, radius, femur, tibia and fibula keep the shaft section of 1:50 and are declared slender instead of being thickened to 15 mm.", "de": "Gürtel und Röhrenknochen: Hüftbeine und Schlüsselbeine tragen die Glieder wie in 1:50; Oberarmknochen, Elle, Speiche, Oberschenkelknochen, Schienbein und Wadenbein behalten den Schaftquerschnitt von 1:50 und sind als schlank erklärt, statt auf 15 mm verdickt zu werden."},
    "200": {"en": "Girdles and long bones: the pelvic ring is the member of both hip bones, which the scale cannot show apart from it; humerus, ulna, radius, femur, tibia and fibula are six true shafts per side at the section of 1:50, declared slender, so the forearm and the shank keep their two bones.", "de": "Gürtel und Röhrenknochen: der Beckenring ist das Glied beider Hüftbeine, die der Maßstab nicht von ihm trennen kann; Oberarmknochen, Elle, Speiche, Oberschenkelknochen, Schienbein und Wadenbein sind je Seite sechs wahre Schäfte im Querschnitt von 1:50, als schlank erklärt, sodass Unterarm und Unterschenkel ihre zwei Knochen behalten."},
}
GIRDLE_TEXT["500"] = GIRDLE_TEXT["200"]


def texts(volumes_sets):
    for volumes in volumes_sets:
        for scale, text in GIRDLE_TEXT.items():
            if text not in volumes[scale]["decisions"]:
                volumes[scale]["decisions"].append(dict(text))
            for decision in volumes[scale]["decisions"]:
                decision["en"] = decision["en"].replace("one bone per limb segment in its true thickness", "the long bones as true slender shafts")
                decision["de"] = decision["de"].replace("ein Knochen je Gliedabschnitt in wahrer Stärke", "die Röhrenknochen als wahre schlanke Schäfte")


def convert(asset, volumes_sets):
    girdles(asset, volumes_sets)
    long_bones(volumes_sets)
    shapes2d(asset)
    texts(volumes_sets)


def spliced(text, key, before, after):
    """Like `splice` of the rig tool, and for a member in the plain two-space layout too."""
    insert = K.insert_module()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    start, end = insert.member_range(text, indent, key)
    plain = lambda value: indent + json.dumps(key) + ": " + json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n" + indent)
    if plain(before) == text[start:end]:
        return text[:start] + plain(after) + text[end:]
    return K.splice(text, key, before, after)


def main():
    if "--apply" not in ARGV:
        document = json.load(open(CATALOG, encoding="utf8"))
        convert(document["asset"], [document["asset"]["volumes"]])
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("clavicle", "arm", "hip", "thigh"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        print(CANDIDATE)
        return
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    if source["volumes"] != document["asset"]["volumes"]:
        raise SystemExit("source and catalog volumes differ: publish first")
    convert(document["asset"], [document["asset"]["volumes"], source["volumes"]])
    for key in ("rig", "representations", "volumes"):
        text = spliced(text, key, before[key], document["asset"][key])
    if json.loads(text) != document:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original or open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("a file changed while converting: run again")
    open(SOURCE, "w", encoding="utf8").write(json.dumps(source, ensure_ascii=False) + ("\n" if source_text.endswith("\n") else ""))
    open(CATALOG, "w", encoding="utf8").write(text)
    print("written", CATALOG, SOURCE)


main()
