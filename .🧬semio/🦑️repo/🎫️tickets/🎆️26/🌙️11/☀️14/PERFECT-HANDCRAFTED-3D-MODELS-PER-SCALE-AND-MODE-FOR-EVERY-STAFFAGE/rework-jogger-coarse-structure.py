"""🦴️ Jogger, second round pass 2: the skeletons of 1:200 and 1:500 modelled for their feature size.

Usage: python3 rework-jogger-coarse-structure.py <candidate out> [--publish]
Reads the catalog document as it is. Long bones keep their measured shaft (femur 32 to 36 mm, tibia 24 to 29 mm, humerus 22 to
25 mm, radius and ulna 12 to 22 mm, clavicle 12 to 13 mm) and are declared slender instead of being thickened to the feature;
compact parts below the feature are merged into the solid they belong to (`members`) or left out. All numbers by hand.
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    sys.argv, keep = [sys.argv[0]], sys.argv
    spec.loader.exec_module(m)
    sys.argv = keep
    return m


lib = module("rework-02-lib")
ASSET = "humans/jogger"
SIDES = (("l", 1), ("r", -1))
FEMUR = {"p0": [0.04, 0.002, 0.054], "p1": [0.4447, 0.0, 0.0], "r0": 0.016, "r1": 0.018}
FEMUR_AXIS = {"p0": [0.0, 0.0, 0.0], "p1": [0.4447, 0.0, 0.0], "r0": 0.016, "r1": 0.018}
FEMUR_NECK = {"p0": [0.0, 0.0, 0.0], "p1": [0.045, 0.002, 0.056], "r0": 0.024, "r1": 0.017}
FEMUR_CONDYLES = {"radii": [0.02, 0.03, 0.04], "at": [0.435, -0.002, 0.0]}
TIBIA = {"p0": [0.0, 0.002, 0.0], "p1": [0.4343, 0.0, -0.002], "r0": 0.0145, "r1": 0.012}
TIBIA_PLATEAU = {"radii": [0.016, 0.027, 0.037], "at": [0.006, 0.002, 0.0]}
FIBULA = {"p0": [0.03, -0.012, 0.034], "p1": [0.4383, -0.006, 0.027], "r0": 0.007, "r1": 0.009}
HUMERUS = {"p0": [0.0, 0.0, 0.0], "p1": [0.322, 0.0, 0.0], "r0": 0.0125, "r1": 0.011}
HUMERUS_HEAD = {"radii": [0.023, 0.023, 0.023], "at": [0.0, 0.0, -0.002]}
ULNA = {"p0": [-0.012, -0.01, 0.0], "p1": [0.26416, -0.016, 0.0], "r0": 0.011, "r1": 0.006}
RADIUS = {"p0": [0.00576, 0.011, 0.0], "p1": [0.268, 0.012, 0.0], "r0": 0.008, "r1": 0.011}
CLAVICLE = {"p0": [0.0, 0.0, 0.0], "p1": [0.1494, 0.0, 0.0], "r0": 0.0065, "r1": 0.006}
HIP_STRUT = {"p0": [0.0, 0.0, 0.0], "p1": [0.1084, 0.0, 0.0], "r0": 0.018, "r1": 0.018}
SCAPULA_500 = {"p0": [0.13, 0.03, 0.09], "p1": [0.2222, -0.025, 0.165], "r0": 0.0376, "r1": 0.0376}
FOOT_500 = {"p0": [-0.045, -0.0447, 0.0], "p1": [0.1344, -0.0507, 0.0], "r0": 0.0376, "r1": 0.0376}
SKULL_500 = {"radii": [0.105, 0.092, 0.071], "at": [0.04, -0.028, 0.0]}
NECK_ON_CHEST = [([0.3175, 0.0, 0.0], 0.0184, 0.016), ([0.3649, 0.008, 0.0], 0.0184, 0.0155), ([0.4124, 0.016, 0.0], 0.019, 0.0155)]
DECISIONS = {
    "200": {
        "en": "Structure 1:200: the load path for a smallest feature of 30 mm. Skull with jaw, cervical, thoracic and lumbar column on their bones, the ribcage as one basket, the scapula as one strut from the blade to the glenoid, the pelvis as one basin with a hip strut from the sacroiliac joint to each socket. The long bones keep their measured shafts and are declared slender instead of being thickened: clavicle 12 to 13 mm, femur 32 to 36 mm with head, neck and condyles, tibia 24 to 29 mm with its plateau and the fibula beside it, humerus 22 to 25 mm with its head, radius and ulna as two bones. Hand and fingers are one member, the foot is heel, midfoot and toes.",
        "de": "Tragwerk 1:200: der Lastpfad für ein kleinstes Merkmal von 30 mm. Schädel mit Kiefer, Hals-, Brust- und Lendenwirbelsäule auf ihren Knochen, der Brustkorb als ein Korb, das Schulterblatt als eine Strebe vom Blatt zur Pfanne, das Becken als eine Schale mit einer Hüftstrebe vom Iliosakralgelenk zu jeder Pfanne. Die langen Knochen behalten ihre gemessenen Schäfte und sind als schlank deklariert statt verdickt: Schlüsselbein 12 bis 13 mm, Oberschenkelknochen 32 bis 36 mm mit Kopf, Hals und Kondylen, Schienbein 24 bis 29 mm mit seinem Plateau und dem Wadenbein daneben, Oberarmknochen 22 bis 25 mm mit Kopf, Speiche und Elle als zwei Knochen. Hand und Finger sind ein Glied, der Fuss besteht aus Ferse, Mittelfuss und Zehen.",
    },
    "500": {
        "en": "Structure 1:500: the load path for a smallest feature of 75 mm. What is smaller is merged: the skull takes the jaw in, the ribcage basket carries the clavicles, the pelvic basin the hip bones, one foot member the heel, midfoot and toes, one hand member the fingers, and the fibula is left out. The thoracic and cervical column is one slender rod at its true section, the lumbar column is the 75 mm envelope of its vertebrae with their processes. Femur, tibia, humerus, radius and ulna keep their measured shafts (32 to 36, 24 to 29, 22 to 25 and 12 to 22 mm) and are declared slender; the femur runs on its mechanical axis from the hip joint to the knee because head and neck are below the feature.",
        "de": "Tragwerk 1:500: der Lastpfad für ein kleinstes Merkmal von 75 mm. Was kleiner ist, wird zusammengefasst: Der Schädel nimmt den Kiefer auf, der Brustkorb trägt die Schlüsselbeine, die Beckenschale die Hüftbeine, ein Fussglied Ferse, Mittelfuss und Zehen, ein Handglied die Finger, und das Wadenbein entfällt. Brust- und Halswirbelsäule sind ein schlanker Stab mit wahrem Querschnitt, die Lendenwirbelsäule ist die 75-mm-Hülle ihrer Wirbel samt Fortsätzen. Oberschenkelknochen, Schienbein, Oberarmknochen, Speiche und Elle behalten ihre gemessenen Schäfte (32 bis 36, 24 bis 29, 22 bis 25 und 12 bis 22 mm) und sind als schlank deklariert; der Oberschenkelknochen läuft auf seiner mechanischen Achse vom Hüftgelenk zum Knie, weil Kopf und Hals unter dem Merkmal liegen.",
    },
}


def sided(values, sign):
    return [values[0], values[1], values[2] * sign]


def capsule(like, id, bone, spec, sign, slender=False, **extra):
    """💊️ A member capsule with the common members of an existing one."""
    s = {k: v for k, v in like.items() if k not in ("p0", "p1", "r0", "r1", "lateralScale", "members", "slender", "kind", "sections", "radii", "transform", "capStart", "capEnd")}
    s.update(id=id, kind="capsule", bone=bone, p0=sided(spec["p0"], sign), p1=sided(spec["p1"], sign), r0=spec["r0"], r1=spec["r1"], member=bone, part="member")
    if slender:
        s["slender"] = True
    s.update(extra)
    return s


def ellipsoid(like, id, bone, spec, sign):
    s = {k: v for k, v in like.items() if k not in ("p0", "p1", "r0", "r1", "lateralScale", "members", "slender", "kind", "sections", "capStart", "capEnd")}
    s.update(id=id, kind="ellipsoid", bone=bone, radii=list(spec["radii"]), transform={"translation": sided(spec["at"], sign)}, member=bone, part="member")
    return s


def rebuild(structure, scale):
    """🔨️ The member list of one coarse scale; joints, masses and everything not named stay as they are."""
    by = {s["id"]: s for s in structure}
    like = by["sk-femur-l"]
    fine = scale == "200"
    out = []
    for s in structure:
        id = s["id"]
        side = id[-1] if id[-2:] in ("-l", "-r") else ""
        sign = 1 if side == "l" else -1
        stem = id[:-2] if side else id
        if stem == "sk-femur":
            if fine:
                out += [capsule(like, f"sk-femur-neck-{side}", f"thigh-{side}", FEMUR_NECK, sign), capsule(like, id, f"thigh-{side}", FEMUR, sign, True), ellipsoid(like, f"sk-femur-condyles-{side}", f"thigh-{side}", FEMUR_CONDYLES, sign)]
            else:
                out.append(capsule(like, id, f"thigh-{side}", FEMUR_AXIS, sign, True))
        elif stem == "sk-tibia":
            out.append(capsule(like, id, f"shank-{side}", TIBIA, sign, True))
            if fine:
                out += [ellipsoid(like, f"sk-tibia-plateau-{side}", f"shank-{side}", TIBIA_PLATEAU, sign), capsule(like, f"sk-fibula-{side}", f"shank-{side}", FIBULA, sign, True)]
        elif stem == "sk-humerus":
            out.append(capsule(like, id, f"upper-arm-{side}", HUMERUS, sign, True))
            if fine:
                out.append(ellipsoid(like, f"sk-humerus-head-{side}", f"upper-arm-{side}", HUMERUS_HEAD, sign))
        elif stem == "sk-forearm":
            out += [capsule(like, f"sk-ulna-{side}", f"forearm-{side}", ULNA, sign, True), capsule(like, f"sk-radius-{side}", f"forearm-{side}", RADIUS, sign, True)]
        elif stem == "sk-clavicle":
            if fine:
                out.append(capsule(like, id, f"clavicle-{side}", CLAVICLE, sign, True))
        elif stem == "sk-hip":
            if fine:
                out.append(capsule(like, id, f"ilium-{side}", HIP_STRUT, sign))
        elif fine:
            out.append(s)
        elif stem == "sk-mandible" or stem == "sk-glenoid" or stem == "sk-calcaneus" or stem == "sk-phalanges" or id == "sk-cervical":
            continue
        elif stem == "sk-cranium":
            out.append(ellipsoid(like, id, "head", SKULL_500, 1))
        elif id == "sk-thoracic-spine":
            rod = copy.deepcopy(by["sk-thoracic-spine"])
            for q in rod["sections"]:
                q["rx"], q["ry"] = FINE_THORACIC[min(FINE_THORACIC, key=lambda u: abs(u - q["at"][0]))]
            rod["sections"] = [q for q in rod["sections"] if q["at"][0] < NECK_ON_CHEST[0][0][0] - 0.005]
            for at, rx, ry in NECK_ON_CHEST:
                q = copy.deepcopy(rod["sections"][-1])
                q["at"], q["rx"], q["ry"] = list(at), rx, ry
                rod["sections"].append(q)
            rod["slender"] = True
            rod["members"] = ["neck"]
            out.append(rod)
        elif id == "sk-ribcage":
            out.append({**s, "members": ["clavicle-l", "clavicle-r"]})
        elif id == "sk-pelvis":
            out.append({**s, "members": ["ilium-l", "ilium-r"]})
        elif stem == "sk-scapula":
            out.append(capsule(like, id, "chest", SCAPULA_500, sign))
        elif stem == "sk-foot":
            out.append(capsule(like, id, f"foot-{side}", FOOT_500, sign, False, lateralScale=1.3, members=[f"toe-{side}"]))
        else:
            out.append(s)
    return out


FINE_THORACIC = {}


def main():
    out = sys.argv[1]
    base = lib.load_catalog(ASSET)
    document = copy.deepcopy(base)
    asset = document["asset"]
    for q in next(s for s in asset["volumes"]["200"]["structure"] if s["id"] == "sk-thoracic-spine")["sections"]:
        FINE_THORACIC[q["at"][0]] = (q["rx"], q["ry"])
    for scale in ("200", "500"):
        v = asset["volumes"][scale]
        v["structure"] = rebuild(v["structure"], scale)
        v["decisions"][2] = dict(DECISIONS[scale])
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
        f.write("\n")
    if "--publish" in sys.argv:
        assert lib.load_catalog(ASSET) == base, "the catalog document changed since it was read"
        lib.save_source(ASSET, {"volumes": asset["volumes"]})
        lib.save_catalog(ASSET, document)
    print(json.dumps({s: [x["id"] for x in asset["volumes"][s]["structure"] if x.get("part") == "member"] for s in ("200", "500")}))


if __name__ == "__main__":
    main()
