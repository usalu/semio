"""🦴 tour-guide: shoulder and pelvic girdle on the lateral rest placement of the rig (see `rig-lateral-rest.md`), and the coarse skeletons back on their true shaft sections.

Rig: `clavicle-l/-r` run from the sternoclavicular to the acromioclavicular joint and carry the upper arms; `ilium-l/-r` run from the sacroiliac joint to the hip joint and carry the thighs. Arms and thighs keep their world rest frames, so their solids, 2D shapes, anchors and masses stay valid.
Poses: a raised arm lifts its clavicle (scapulohumeral rhythm); the arm offsets are solved again so that every posed arm keeps its direction.
Structure: the girdle bones are bound to the new bones (same world geometry), the clavicle becomes the S-curved bone, and 1:200 / 1:500 carry the long bones at the shaft section of 1:50 (`slender`) instead of rods thickened to the feature.
Every number is metres in the world of the rest pose (x ahead, y up from the hip joint, z left), left side; the right side is the mirror image.
Run after `rework-tour-guide-skeleton.py` (that script rebuilds the structure lists on the old frames).
Usage: python3 rework-tour-guide-girdle.py            candidate `🗑️generated/rework-tour-guide/tour-guide/🔣️.json`
       python3 rework-tour-guide-girdle.py --apply    writes `rig` and `poses` of the catalog file and the volumes source (publish it with `volumes-insert.py`)
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rl = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(rl)

REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "tour-guide", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "tour-guide.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-tour-guide", "tour-guide", "🔣️.json")

STERNOCLAVICULAR = (0.060, 0.5276, 0.022)
ACROMIOCLAVICULAR = (-0.004, 0.5473, 0.170)
SACROILIAC = (-0.060, 0.070, 0.050)
BACK = {"l": (-1.0, 0.0, 0.0), "r": (1.0, 0.0, 0.0)}
CLAVICLE_JOINT = {"l": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5}, "r": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -5, "twistMaxDeg": 40}}
ILIUM_JOINT = {"type": "revolute", "minDeg": -3, "maxDeg": 3}
TITLES = {
    "clavicle-l": {"en": "Left clavicle", "de": "Linkes Schlüsselbein"}, "clavicle-r": {"en": "Right clavicle", "de": "Rechtes Schlüsselbein"},
    "ilium-l": {"en": "Left hip bone", "de": "Linkes Hüftbein"}, "ilium-r": {"en": "Right hip bone", "de": "Rechtes Hüftbein"},
}
LIFT = {"standing-pointing": {"r": 8}, "waving": {"r": 12}, "wave-swing-a": {"r": 12}, "wave-swing-b": {"r": 12}, "holding-flag": {"l": 4}, "gesturing-to-group": {"r": 3}}
CLAVICLE_CURVE = [(0.060, 0.5276, 0.022, 0.0110), (0.065, 0.5320, 0.060, 0.0080), (0.051, 0.5390, 0.100, 0.0066), (0.015, 0.5450, 0.136, 0.0068), (-0.006, 0.5475, 0.156, 0.0076), (-0.004, 0.5473, 0.170, 0.0082)]
CLAVICLE_ROD = (0.0095, 0.0080)
GIRDLE = {"clavicle": ("clavicle", "scapula", "acromion"), "ilium": ("ilium", "pubis", "ischium")}
SHAFTS = {"200": ("tibia", "fibula", "ulna", "radius"), "500": ("femur", "tibia", "humerus", "ulna", "radius")}
REPLACED = {"200": ("shoulder-girdle", "tibia-l", "tibia-r", "forearm-bones-l", "forearm-bones-r"), "500": ("shoulder-girdle", "femur-l", "femur-r", "tibia-l", "tibia-r", "humerus-l", "humerus-r", "forearm-bones-l", "forearm-bones-r")}
PELVIS_MASS = {"200": "pelvic-ring", "500": "pelvic-girdle"}
SPINE_500 = [(-0.040, 0.045, 0.020), (0.01068, 0.03495, 0.021), (0.23468, 0.05992, 0.0175), (0.43547, 0.04993, 0.015)]
CERVICAL_500 = 0.0115
JOINT_500 = 0.0375
ALONG_U = [0.5, 0.5, 0.5, 0.5]
FEMUR_NECK_500 = ((0.0, 0.016), (0.55, 0.014))
WORLD = ([0.0, 0.0, 0.0], [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
ILIAC_WING = {"centre": (-0.025, 0.075, 0.100), "radii": (0.062, 0.078, 0.011), "along": (0.15, 0.0, 0.07), "flareDeg": 20.0}
ILIAC_CREST = [(-0.098, 0.088, 0.048), (-0.075, 0.125, 0.085), (-0.030, 0.137, 0.128), (0.020, 0.118, 0.138), (0.052, 0.082, 0.122)]
ILIAC_CREST_R = 0.008
ACETABULUM = ((-0.004, 0.004, 0.072), (0.034, 0.034, 0.022))
ISCHIOPUBIC_RAMUS = ((-0.045, -0.053, 0.060), (0.048, -0.035, 0.012), 0.0075)

STRUCTURE_TEXT = {
    "50": {
        "en": "1:50 structure: the skeleton carries the body: skull with jaw, cervical and thoracolumbar spine with their curves, twelve pairs of descending ribs with a costal margin and floating ends, and the sternum. The shoulder girdle hangs from the sternum on the clavicle bones of the rig: the S-curved clavicle from the sternoclavicular to the acromioclavicular joint, the scapula and the acromion over the humeral head, so a raised arm lifts its shoulder. The pelvic girdle sits on the hip bones of the rig, from the sacroiliac joint to the hip joint: flared iliac wing with its crest from the posterior to the anterior superior spine, acetabulum, pubic and ischial rami and the ischiopubic ramus round the obturator foramen, beside the sacrum. Femur with head, neck, trochanter and condyles, patella, tibia, fibula, malleolus, calcaneus, midfoot and toes, humerus, ulna, radius, carpus, metacarpals and finger rays; joints at the rig joints and masses at the segment centres; the pointer is carried by its grip and sections, the flag by the pole and the cloth on its hoist.",
        "de": "1:50 Tragwerk: Das Skelett trägt den Körper: Schädel mit Unterkiefer, Hals- und Brust-Lenden-Wirbelsäule mit ihren Krümmungen, zwölf absteigende Rippenpaare mit Rippenbogen und freien Enden und das Brustbein. Der Schultergürtel hängt am Brustbein an den Schlüsselbein-Knochen des Rigs: das S-förmig geschwungene Schlüsselbein vom Brustbein-Schlüsselbein-Gelenk zum Schultereckgelenk, das Schulterblatt und die Schulterhöhe über dem Oberarmkopf, sodass ein gehobener Arm seine Schulter hebt. Der Beckengürtel sitzt auf den Hüftbein-Knochen des Rigs vom Kreuz-Darmbein-Gelenk zum Hüftgelenk: ausladende Darmbeinschaufel mit ihrem Kamm vom hinteren zum vorderen oberen Darmbeinstachel, Hüftpfanne, Scham- und Sitzbeinast und der Sitz-Schambein-Ast um das Hüftloch, neben dem Kreuzbein. Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Kniescheibe, Schien- und Wadenbein, Knöchel, Fersenbein, Mittelfuß und Zehen, Oberarmknochen, Elle, Speiche, Handwurzel, Mittelhand und Fingerstrahlen; Gelenke an den Rig-Gelenken und Massen in den Segmentschwerpunkten; der Zeigestab trägt über Griff und Auszüge, die Fahne über die Stange und das Tuch an seinem Hohlsaum.",
    },
    "100": {
        "en": "1:100 structure: a skeleton of merged bones: skull, face and jaw, cervical and thoracolumbar spine, six bilateral rib courses including a floating pair, sternum; on the clavicle bones of the rig the curved clavicle, the scapula and the acromion that ties it to the clavicle; on the hip bones of the rig the iliac wing with its crest, the acetabulum and the pubic, ischial and ischiopubic rami beside the sacrum; femur with neck and condyles, patella, tibia, fibula, one foot and one toe form, humerus, ulna, radius, metacarpus, finger and thumb rays; joints, masses; the pointer by its grip and rod, the pole and the flag cloth by reference.",
        "de": "1:100 Tragwerk: ein Skelett aus zusammengefassten Knochen: Schädel, Gesicht und Kiefer, Hals- und Brust-Lenden-Wirbelsäule, sechs beidseitige Rippenzüge mit einem freien Paar, Brustbein; an den Schlüsselbein-Knochen des Rigs das geschwungene Schlüsselbein, das Schulterblatt und die Schulterhöhe, die es an das Schlüsselbein bindet; an den Hüftbein-Knochen des Rigs die Darmbeinschaufel mit ihrem Kamm, die Hüftpfanne und Scham-, Sitz- und Sitz-Schambein-Ast neben dem Kreuzbein; Oberschenkelknochen mit Hals und Kondylen, Kniescheibe, Schien- und Wadenbein, je eine Fuß- und Zehenform, Oberarmknochen, Elle, Speiche, Mittelhand, Finger- und Daumenstrahlen; Gelenke, Massen; der Zeigestab über Griff und Stab, Stange und Fahnentuch per Verweis.",
    },
    "200": {
        "en": "1:200 structure: skull and face, cervical bar, spine, three bilateral rib courses with the lower costal arch, sternum, the two clavicles as straight rods at their true section from the sternum to the shoulder, a pelvic ring that stands for both hip bones, the long bones at the shaft section of 1:50 (femur with head, neck, trochanter and condyles, tibia and fibula, humerus with head and condyles, ulna and radius), foot and toe forms, metacarpal plates; joints at hips, knees, shoulders, trunk and neck, masses of trunk, pelvis, head, thighs, pointer and pole; pointer, pole and flag by reference. The scapula is thinner than the feature and left out.",
        "de": "1:200 Tragwerk: Schädel und Gesicht, Halsstab, Wirbelsäule, drei beidseitige Rippenzüge mit dem unteren Rippenbogen, Brustbein, die beiden Schlüsselbeine als gerade Stäbe im wahren Querschnitt vom Brustbein zur Schulter, ein Beckenring für beide Hüftbeine, die langen Knochen im Schaftquerschnitt von 1:50 (Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Schien- und Wadenbein, Oberarmknochen mit Kopf und Kondylen, Elle und Speiche), Fuß- und Zehenformen, Mittelhandplatten; Gelenke an Hüften, Knien, Schultern, Rumpf und Hals, Massen von Rumpf, Becken, Kopf, Oberschenkeln, Zeigestab und Stange; Zeigestab, Stange und Fahne per Verweis. Das Schulterblatt ist dünner als das Merkmal und entfällt.",
    },
    "500": {
        "en": "1:500 structure: the long bones keep their true shaft section and are not thickened to the 75 mm feature: spine from the sacrum to the neck, cervical column, the two clavicles, femur bent through its neck into the hip joint, tibia, humerus, ulna and radius; skull, a pelvic bar that stands for both hip bones, foot and hand masses and the joint markers at shoulders, hips, knees, ankles, trunk, neck and the flag hoist are compact forms of the feature size; two open paired rib courses and a sternal bar; the trunk mass; pointer, pole and flag by reference.",
        "de": "1:500 Tragwerk: Die langen Knochen behalten ihren wahren Schaftquerschnitt und werden nicht auf das Merkmal von 75 mm verdickt: Wirbelsäule vom Kreuzbein bis zum Hals, Halswirbelsäule, die beiden Schlüsselbeine, der Oberschenkelknochen über seinen Hals ins Hüftgelenk gebogen, Schienbein, Oberarmknochen, Elle und Speiche; Schädel, ein Beckenstab für beide Hüftbeine, Fuß- und Handmassen und die Gelenkmarken an Schultern, Hüften, Knien, Sprunggelenken, Rumpf, Hals und am Fahnensaum sind kompakte Formen in Merkmalsgröße; zwei offene Rippenpaare und ein Brustbeinstab; die Rumpfmasse; Zeigestab, Stange und Fahne per Verweis.",
    },
}


def side_point(point, sign):
    return [point[0], point[1], sign * point[2]]


def local(frame, point):
    return rl.r5(rl.apply(rl.transpose(frame[1]), [point[i] - frame[0][i] for i in range(3)]))


def moved(solid, old, new):
    """The same world solid written in the frame of another bone."""
    turn = rl.mul(rl.transpose(new[1]), old[1])
    q = rl.quaternion(turn)

    def point(p):
        return local(new, [old[0][i] + rl.apply(old[1], p)[i] for i in range(3)])

    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = point(transform["translation"])
        transform["rotation"] = rl.r5(rl.product(q, transform.get("rotation", [0, 0, 0, 1])))
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            if "rotation" in section:
                section["rotation"] = rl.r5(rl.product(q, section["rotation"]))
            elif section["rx"] != section["ry"]:
                raise SystemExit(f"loft {solid['id']} has a flat section without rotation")
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")


def bone_solid(solid_id, bone, **fields):
    return {"id": solid_id, "bone": bone, "material": "bone", "role": "body", "tone": 0.9, **fields, "member": bone, "part": "member"}


def hip_bone(side, sign, frame):
    """The hip bone of one side: flared iliac wing, iliac crest from the posterior to the anterior superior spine, acetabulum and the ischiopubic ramus that closes the obturator foramen."""
    bone = f"ilium-{side}"
    n = math.hypot(ILIAC_WING["along"][0], ILIAC_WING["along"][2])
    a2 = [ILIAC_WING["along"][0] / n, 0.0, sign * ILIAC_WING["along"][2] / n]
    out = [-ILIAC_WING["along"][2] / n, 0.0, sign * ILIAC_WING["along"][0] / n]
    c, s = math.cos(ILIAC_WING["flareDeg"] / rl.DEG), math.sin(ILIAC_WING["flareDeg"] / rl.DEG)
    a1 = [s * out[0], c, s * out[2]]
    a3 = [a1[1] * a2[2] - a1[2] * a2[1], a1[2] * a2[0] - a1[0] * a2[2], a1[0] * a2[1] - a1[1] * a2[0]]
    wing = bone_solid(f"ilium-{side}", bone, kind="ellipsoid", radii=list(ILIAC_WING["radii"]), transform={"translation": side_point(ILIAC_WING["centre"], sign), "rotation": rl.r5(rl.quaternion([[a1[i], a2[i], a3[i]] for i in range(3)]))})
    wing = {**wing, "kind": "ellipsoid"}
    socket = bone_solid(f"acetabulum-{side}", bone, kind="ellipsoid", radii=list(ACETABULUM[1]), transform={"translation": side_point(ACETABULUM[0], sign)})
    ramus = bone_solid(f"ischiopubic-ramus-{side}", bone, kind="capsule", p0=side_point(ISCHIOPUBIC_RAMUS[0], sign), p1=side_point(ISCHIOPUBIC_RAMUS[1], sign), r0=ISCHIOPUBIC_RAMUS[2], r1=ISCHIOPUBIC_RAMUS[2])
    for solid in (wing, socket, ramus):
        moved(solid, WORLD, frame)
    crest = bone_solid(f"iliac-crest-{side}", bone, kind="loft", sections=[{"at": local(frame, side_point(point, sign)), "rx": ILIAC_CREST_R, "ry": ILIAC_CREST_R} for point in ILIAC_CREST], capStart=True, capEnd=True)
    return [wing, crest, socket, ramus]


def joint_marker(solid_id, bone):
    return {"id": solid_id, "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": 0.9, "radii": [JOINT_500] * 3, "transform": {"translation": [0, 0, 0]}, "member": bone, "part": "joint"}


def bone_entry(bone_id, parent, placement, length, joint):
    return {"id": bone_id, "parent": parent, "title": TITLES[bone_id], **placement, "lengthM": round(length, 5), "joint": joint}


def convert_rig(asset):
    bones = asset["rig"]["bones"]
    if any(bone["id"] == "clavicle-l" for bone in bones):
        return None
    old_bones = copy.deepcopy(bones)
    old = rl.frames(old_bones)
    for side, sign in (("l", 1), ("r", -1)):
        for girdle, parent, limb, start, joint in (("clavicle", "torso", "upper-arm", STERNOCLAVICULAR, CLAVICLE_JOINT[side]), ("ilium", "pelvis", "thigh", SACROILIAC, ILIUM_JOINT)):
            origin = side_point(start, sign)
            tip = side_point(ACROMIOCLAVICULAR, sign) if girdle == "clavicle" else old[f"thigh-{side}"][0]
            u = [tip[i] - origin[i] for i in range(3)]
            entry = bone_entry(f"{girdle}-{side}", parent, rl.placed(old[parent], origin, u, BACK[side]), math.sqrt(sum(c * c for c in u)), joint)
            index = next(i for i, bone in enumerate(bones) if bone["id"] == f"{limb}-{side}")
            bones.insert(index, entry)
            child = bones[index + 1]
            placement = rl.fields(rl.frames(bones)[entry["id"]], old[child["id"]][0], old[child["id"]][1])
            child["parent"] = entry["id"]
            child.update({key: value for key, value in placement.items() if key not in ("abductionDeg", "twistDeg") or abs(value) > 1e-9})
    new = rl.frames(bones)
    for bone in old_bones:
        drift = max(abs(new[bone["id"]][0][i] - old[bone["id"]][0][i]) for i in range(3))
        turned = max(abs(new[bone["id"]][1][i][j] - old[bone["id"]][1][i][j]) for i in range(3) for j in range(3))
        if drift > 2e-5 or turned > 2e-5:
            raise SystemExit(f"{bone['id']} moved by {drift} m / {turned}")
    for pose in asset["poses"]:
        offsets = pose["bones"]
        before = rl.frames(old_bones, copy.deepcopy(offsets))
        for side, lift in LIFT.get(pose["id"], {}).items():
            offsets[f"clavicle-{side}"] = {"angleDeg": lift}
            arm = f"upper-arm-{side}"
            parent = rl.frames(bones, {key: value for key, value in offsets.items() if key != arm})[arm][1]
            a, b, t = rl.angles(rl.mul(rl.transpose(parent), before[arm][1]))
            offsets[arm] = {key: value for key, value in (("angleDeg", round(a, 2) + 0.0), ("abductionDeg", round(b, 2) + 0.0), ("twistDeg", round(t, 2) + 0.0)) if key == "angleDeg" or value != 0}
        after = rl.frames(bones, offsets)
        for side in ("l", "r"):
            turned = max(abs(after[f"hand-{side}"][1][i][j] - before[f"hand-{side}"][1][i][j]) for i in range(3) for j in range(3))
            if turned > 4e-4:
                raise SystemExit(f"{pose['id']}: hand-{side} turned by {turned}")
    return old, new


def clavicle_loft(side, sign, frame):
    return {"id": f"clavicle-{side}", "kind": "loft", "bone": f"clavicle-{side}", "material": "bone", "role": "body", "tone": 0.9, "sections": [{"at": local(frame, [x, y, sign * z]), "rx": r, "ry": r} for x, y, z, r in CLAVICLE_CURVE], "capStart": True, "capEnd": True, "member": f"clavicle-{side}", "part": "member"}


def clavicle_rod(side, length):
    return {"id": f"clavicle-{side}", "kind": "capsule", "bone": f"clavicle-{side}", "material": "bone", "role": "body", "tone": 0.9, "p0": [0, 0, 0], "p1": [round(length, 5), 0, 0], "r0": CLAVICLE_ROD[0], "r1": CLAVICLE_ROD[1], "slender": True, "member": f"clavicle-{side}", "part": "member"}


def convert_volumes(volumes, old, new, lengths):
    for scale, text in STRUCTURE_TEXT.items():
        volumes[scale]["decisions"][2] = text
    if any(solid.get("bone") == "clavicle-l" for solid in volumes["50"]["structure"]):
        return
    fine = {solid["id"]: solid for solid in volumes["50"]["structure"]}
    for scale in ("50", "100", "200", "500"):
        for solid in volumes[scale]["structure"]:
            if solid["id"] in ("tibia-l", "tibia-r") and solid.get("kind") == "loft":
                for section in solid["sections"]:
                    section["rotation"] = ALONG_U
    for scale in ("50", "100"):
        out = []
        for solid in volumes[scale]["structure"]:
            name, _, side = solid["id"].rpartition("-")
            target = next((girdle for girdle, names in GIRDLE.items() if name in names and side in ("l", "r")), None)
            if target:
                parent, bone = ("torso", f"clavicle-{side}") if target == "clavicle" else ("pelvis", f"ilium-{side}")
                if name == "clavicle":
                    solid = clavicle_loft(side, 1 if side == "l" else -1, new[bone])
                elif name == "ilium":
                    out.extend(hip_bone(side, 1 if side == "l" else -1, new[bone]))
                    continue
                else:
                    moved(solid, old[parent], new[bone])
                    solid["bone"] = solid["member"] = bone
            out.append(solid)
            if scale == "100" and name == "scapula" and side in ("l", "r"):
                out.append(copy.deepcopy(fine[f"acromion-{side}"]))
        volumes[scale]["structure"] = out
    for scale in ("200", "500"):
        out = []
        for solid in volumes[scale]["structure"]:
            if solid["id"] == "shoulder-girdle":
                out.extend(clavicle_rod(side, lengths[f"clavicle-{side}"]) for side in ("l", "r"))
            elif solid["id"] in REPLACED[scale]:
                name, _, side = solid["id"].rpartition("-")
                for bone in (("ulna", "radius") if name == "forearm-bones" else (name, "fibula") if name == "tibia" and "fibula" in SHAFTS[scale] else (name,)):
                    shaft = copy.deepcopy(fine[f"{bone}-{side}"])
                    shaft["slender"] = True
                    if scale == "500" and bone == "femur":
                        neck = fine[f"femur-neck-{side}"]["p1"]
                        shaft["sections"] = [{"at": rl.r5([k * c for c in neck]), "rx": r, "ry": r} for k, r in FEMUR_NECK_500] + shaft["sections"]
                    out.append(shaft)
            else:
                if solid["id"] == PELVIS_MASS[scale]:
                    solid["members"] = ["ilium-l", "ilium-r"]
                if scale == "500" and solid["id"] == "spine":
                    solid["sections"] = [{"at": [x, y, 0], "rx": r, "ry": r} for x, y, r in SPINE_500]
                    solid["slender"] = True
                if scale == "500" and solid["id"] == "cervical-spine":
                    for section in solid["sections"]:
                        section["rx"] = section["ry"] = CERVICAL_500
                    solid["slender"] = True
                out.append(solid)
        if scale == "500":
            out.extend(joint_marker(f"joint-{bone}-{side}", f"{bone}-{side}") for bone in ("upper-arm", "foot") for side in ("l", "r"))
        volumes[scale]["structure"] = out


def main():
    text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    frames = convert_rig(document["asset"])
    if frames is None:
        old = new = rl.frames(document["asset"]["rig"]["bones"])
    else:
        old, new = frames
    lengths = {bone["id"]: bone["lengthM"] for bone in document["asset"]["rig"]["bones"]}
    convert_volumes(source["volumes"], old, new, lengths)
    if "--apply" not in sys.argv:
        document["asset"]["volumes"] = source["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("clavicle", "ilium", "upper-arm", "thigh"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        for pose in document["asset"]["poses"]:
            print(pose["id"], json.dumps({key: value for key, value in pose["bones"].items() if key[:-2] in ("clavicle", "upper-arm")}))
        print(CANDIDATE)
        return
    if frames is not None:
        for key in ("rig", "poses"):
            text = rl.splice(text, key, before[key], document["asset"][key])
        if json.loads(text)["asset"]["rig"] != document["asset"]["rig"] or json.loads(text)["asset"]["poses"] != document["asset"]["poses"]:
            raise SystemExit("splice did not round-trip")
        if open(CATALOG, encoding="utf8").read() != json.dumps(None) and json.loads(open(CATALOG, encoding="utf8").read())["asset"] != before:
            raise SystemExit("the catalog file changed while converting: run again")
        open(CATALOG, "w", encoding="utf8").write(text)
    if open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("the volumes source changed while converting: run again")
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
