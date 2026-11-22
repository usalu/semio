"""🪽 Herring gull, second round pass 1: shoulder girdle and pelvis halves on the lateral rest placement, humeral feathers (tertials) on their own bones, true thin vanes and membranes at 1:100 and 1:200, closed load paths.

Usage: python3 rework-herring-gull-girdle.py            writes the candidate document to 🗑️generated/r3-herring-gull/🔣️.json
       python3 rework-herring-gull-girdle.py --apply    writes rig and poses of the catalog asset and the volumes source (publish the volumes with volumes-insert.py afterwards)
Every number below is chosen by hand against the reference table of rework/assets/animals--herring-gull.md.
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/herring-gull/🔣️.json"
SOURCE = os.path.join(T, "volumes/animals/herring-gull.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-herring-gull/🔣️.json")
SCALES = ("50", "100", "200", "500")
LIMB = [0.5, 0.5, 0.5, 0.5]
VANE = [0, 0.70710678, 0, 0.70710678]

STERNAL_FACET = [0.124, -0.028, -0.013]
GLENOID = [0.095, 0.016, -0.036]
SHIELD_EDGE = [0.004, 0.014, -0.016]
ACETABULUM = [0.0, 0.0, -0.022]
TERTIALS = [
    {"k": 1, "x": 0.110, "length": 0.090, "rest": -3.0, "open": -74.0, "width": 0.034},
    {"k": 2, "x": 0.086, "length": 0.112, "rest": -3.5, "open": -80.0, "width": 0.036},
    {"k": 3, "x": 0.062, "length": 0.122, "rest": -4.0, "open": -86.0, "width": 0.037},
    {"k": 4, "x": 0.038, "length": 0.122, "rest": -4.5, "open": -92.0, "width": 0.036},
    {"k": 5, "x": 0.014, "length": 0.112, "rest": -5.0, "open": -100.0, "width": 0.034},
]
VANE_PROFILE = {
    "50": [(0, 0.05), (0.08, 0.44), (0.25, 0.78), (0.48, 1), (0.67, 0.98), (0.83, 0.78), (0.94, 0.48), (0.98, 0.29), (1, 0.05)],
    "100": [(0, 0.05), (0.08, 0.44), (0.25, 0.78), (0.48, 1), (0.83, 0.78), (0.94, 0.48), (1, 0.05)],
    "200": [(0, 0.05), (0.25, 0.78), (0.48, 1), (0.83, 0.78), (1, 0.05)],
    "500": [(0, 0.05), (0.25, 0.78), (0.67, 0.98), (1, 0.05)],
}
VANE_THICKNESS = 0.0012
SIDES = (("n", "near", 1), ("f", "far", -1))


def rz(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def ry(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rx(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def transpose(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def apply(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def rest(angle, abduction=0, twist=0):
    """🧭 Rest rotation of a bone in its parent: Rw(angle) · Rv(abduction) · Ru(twist) with the senses of rig-lateral-rest.md."""
    return mul(mul(rz(angle), ry(-abduction)), rx(-twist))


def angles(matrix):
    """🧮 Inverse of rest(): angleDeg, abductionDeg, twistDeg."""
    beta = -math.asin(max(-1, min(1, matrix[2][0])))
    alpha = math.atan2(matrix[1][0], matrix[0][0])
    gamma = math.atan2(matrix[2][1], matrix[2][2])
    return [round(math.degrees(alpha), 4), round(-math.degrees(beta), 4), round(-math.degrees(gamma), 4)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def norm(a):
    length = math.sqrt(sum(x * x for x in a))
    return [x / length for x in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def frame(origin, tip, axis):
    """🦴 Columns u, v, w of a bone from origin to tip whose joint axis is the given direction made perpendicular."""
    u = norm(sub(tip, origin))
    dot = sum(axis[i] * u[i] for i in range(3))
    w = norm([axis[i] - dot * u[i] for i in range(3)])
    v = cross(w, u)
    return [[u[i], v[i], w[i]] for i in range(3)]


def r5(values):
    return [round(v, 5) for v in values]


def mirror_z(point, sign):
    return [point[0], point[1], point[2] * sign]


def girdle(parent_frame_of_limb, origin, tip):
    """🔩 Near-side girdle bone under the root and the limb re-parented to its tip without moving."""
    matrix = frame(origin, tip, [1, 0, 0])
    length = math.sqrt(sum(x * x for x in sub(tip, origin)))
    local = mul(transpose(matrix), parent_frame_of_limb)
    return matrix, round(length, 5), angles(matrix), angles(local)


def bone_entry(identifier, parent, title, position, placement, length, joint, sign, mass=None):
    entry = {"id": identifier, "parent": parent, "title": title, "x": round(position[0], 5), "y": round(position[1], 5), "z": round(position[2] * sign, 5), "angleDeg": placement[0]}
    if placement[1]:
        entry["abductionDeg"] = round(placement[1] * sign, 4)
    if placement[2]:
        entry["twistDeg"] = round(placement[2] * sign, 4)
    entry["lengthM"] = length
    entry["joint"] = joint
    if mass:
        entry["mass"] = mass
    return entry


def loft(identifier, bone, material, tone, stations, rotation, extra):
    solid = {"id": identifier, "kind": "loft", "bone": bone, "material": material, "role": "body", "tone": tone, "sections": [{"at": r5(at), "rx": a, "ry": b, "exponent": 2, "rotation": rotation} for at, a, b in stations], "capStart": True, "capEnd": True}
    solid.update(extra)
    return solid


def vane(scale, side, name, sign, feather, width):
    half = width / 2
    stations = [([feather["length"] * f, 0.004, 0], VANE_THICKNESS / 2, max(half * g, VANE_THICKNESS / 2)) for f, g in VANE_PROFILE[scale]]
    return loft(f"{name}-tertial-{feather['k']}", f"t{feather['k']}-{side}", "wing-feathers", 0.62, stations, VANE, {"sheet": True})


def rachis(scale, side, name, feather):
    stations = [([feather["length"] * f, 0.004, 0], 0.0009 - 0.0007 * f, 0.001 - 0.0008 * f) for f, _ in VANE_PROFILE[scale]]
    return loft(f"{name}-tertial-{feather['k']}-rachis", f"t{feather['k']}-{side}", "wing-feathers", 0.58, stations, VANE, {"slender": True, "part": "member", "member": f"t{feather['k']}-{side}"})


def convert(asset, volume_sets):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    if "coracoid-n" in by_id:
        return False
    humerus_frame = rest(by_id["humerus-n"]["angleDeg"])
    thigh_frame = rest(by_id["thigh-n"]["angleDeg"])
    coracoid_matrix, coracoid_length, coracoid_rest, humerus_local = girdle(humerus_frame, STERNAL_FACET, GLENOID)
    ilium_matrix, ilium_length, ilium_rest, thigh_local = girdle(thigh_frame, SHIELD_EDGE, ACETABULUM)
    titles = {
        "coracoid": {"en": "{0} coracoid: strut of the shoulder girdle from the sternal facet up, back and out to the shoulder socket; scapula and furcula meet at its head", "de": "{1} Rabenbein: Strebe des Schultergürtels von der Brustbeinfacette nach oben, hinten und außen zur Schultergelenkpfanne; Schulterblatt und Gabelbein treffen an seinem Kopf zusammen"},
        "ilium": {"en": "{0} pelvis half: acetabular wall of ilium and ischium from the edge of the fused dorsal shield down and out to the hip socket (fused, no motion)", "de": "{1} Beckenhälfte: Pfannenwand aus Darm- und Sitzbein vom Rand des verwachsenen Rückenschilds nach unten und außen zur Hüftpfanne (verwachsen, unbeweglich)"},
        "tertial": {"en": "Humeral feather T{2} ({3} wing)", "de": "Schirmfeder T{2} ({4} Flügel)"},
    }
    words = {"n": ("Near", "Nahes", "near", "naher"), "f": ("Far", "Fernes", "far", "ferner")}
    fresh = {}
    for side, name, sign in SIDES:
        near, nahes, low, nah = words[side]
        fresh[f"humerus-{side}"] = [
            bone_entry(f"coracoid-{side}", "trunk", {key: text.format(near, nahes) for key, text in titles["coracoid"].items()}, STERNAL_FACET, coracoid_rest, coracoid_length, {"type": "revolute", "minDeg": -4, "maxDeg": 4}, sign)
        ]
        fresh[f"thigh-{side}"] = [
            bone_entry(f"ilium-{side}", "trunk", {key: text.format(near, "Nahe" if side == "n" else "Ferne") for key, text in titles["ilium"].items()}, SHIELD_EDGE, ilium_rest, ilium_length, {"type": "fixed"}, sign)
        ]
        for limb, parent, length, local in ((f"humerus-{side}", f"coracoid-{side}", coracoid_length, humerus_local), (f"thigh-{side}", f"ilium-{side}", ilium_length, thigh_local)):
            bone = by_id[limb]
            placed = bone_entry(limb, parent, bone["title"], [length, 0, 0], local, bone["lengthM"], bone["joint"], sign, bone.get("mass"))
            bone.clear()
            bone.update(placed)
        by_id[f"humerus-{side}"]["mass"]["kg"] = 0.029
    ordered = []
    for bone in bones:
        ordered.extend(fresh.get(bone["id"], []))
        ordered.append(bone)
    for side, name, sign in SIDES:
        near, nahes, low, nah = words[side]
        for feather in TERTIALS:
            title = {key: text.format(near, nahes, feather["k"], low, "naher" if side == "n" else "ferner") for key, text in titles["tertial"].items()}
            mass = {"kg": 0.0002, "comX": round(feather["length"] * 0.41, 5), "comY": 0, "distribution": "uniform", "source": "authored-estimate"}
            joint = {"type": "revolute", "minDeg": -180, "maxDeg": 180, "twistMinDeg": -100, "twistMaxDeg": 100}
            ordered.append(bone_entry(f"t{feather['k']}-{side}", f"humerus-{side}", title, [feather["x"], 0, -0.004 - 0.0013 * feather["k"]], [feather["rest"], 0, 0], feather["length"], joint, sign, mass))
    asset["rig"]["bones"] = ordered

    for pose in asset["poses"]:
        opening = abs(pose["bones"].get("s12-n", {}).get("angleDeg", 0))
        if not opening:
            continue
        factor = min(1.0, opening / 70.0)
        for side, name, sign in SIDES:
            for feather in TERTIALS:
                pose["bones"][f"t{feather['k']}-{side}"] = {"angleDeg": round((feather["open"] - feather["rest"]) * factor, 2)}

    coracoid_turn = transpose(coracoid_matrix)
    ilium_turn = transpose(ilium_matrix)
    for volumes in volume_sets:
        for scale in SCALES:
            lists = volumes[scale]
            body, thermal, structure = lists["body"], lists["thermal"], lists["structure"]
            for side, name, sign in SIDES:
                length = coracoid_length
                if scale == "50":
                    stations = [([-0.003, 0, 0], 0.0058, 0.0058), ([0.012, 0, 0], 0.0044, 0.0044), ([0.036, 0, 0], 0.0038, 0.0038), ([length - 0.006, 0, 0], 0.0042, 0.0042), ([length + 0.001, 0, 0], 0.0046, 0.0046)]
                    extra = {}
                elif scale == "100":
                    stations = [([-0.018, 0, 0], 0.00136, 0.00136), ([length, 0, 0], 0.00136, 0.00136)]
                    extra = {"slender": True}
                else:
                    stations = [([-0.02, 0, 0], 0.0014, 0.0014), ([length, 0, 0], 0.0014, 0.0014)]
                    extra = {"slender": True}
                extra.update({"member": f"coracoid-{side}", "part": "member"})
                fresh_coracoid = loft(f"{name}-coracoid", f"coracoid-{side}", "bone", 0.88, stations, LIMB, extra)
                index = next(i for i, solid in enumerate(structure) if solid["id"] == f"{name}-coracoid")
                structure[index] = fresh_coracoid
                for solid in structure:
                    if solid["id"] == f"{name}-scapula":
                        solid["sections"][0]["at"] = [0.094, 0.019, -0.033 * sign]
                        solid["sections"][1]["at"] = [0.07, 0.033, -0.027 * sign]
                    if solid["id"] == f"{name}-furcula" and scale in ("50", "100"):
                        solid["sections"][0]["at"] = [0.098, 0.014, -0.033 * sign]
                    if solid["id"] == f"{name}-hand-bones" and scale in ("100", "200"):
                        solid["slender"] = True
                if scale == "50":
                    stations = [([-0.004, 0, 0], 0.0042, 0.0052), ([0.006, 0, 0], 0.0042, 0.0048), ([ilium_length + 0.001, 0, 0], 0.0044, 0.0044)]
                    position = next(i for i, solid in enumerate(structure) if solid["id"] == f"{name}-pubis")
                    structure.insert(position, loft(f"{name}-ilium", f"ilium-{side}", "bone", 0.88, stations, LIMB, {"member": f"ilium-{side}", "part": "member"}))
                if scale == "200":
                    rib = [([0.07, 0.03, -0.003 * sign], 0.0012, 0.0012), ([0.074, 0.012, -0.03 * sign], 0.0012, 0.0012), ([0.08, -0.03, -0.032 * sign], 0.0012, 0.0012), ([0.085, -0.052, -0.02 * sign], 0.0012, 0.0012)]
                    position = next(i for i, solid in enumerate(structure) if solid["id"] == "near-coracoid")
                    structure.insert(position, loft(f"{name}-rib", "trunk", "bone", 0.88, rib, [0, -0.70711, 0, 0.70711], {"slender": True, "member": "trunk", "part": "member"}))
                thermal_pattern = next(entry for entry in thermal if entry["id"] == f"th-{name}-secondary-12")
                feathers = TERTIALS if scale != "500" else [dict(TERTIALS[2], width=0.08)]
                for feather in feathers:
                    body.append(vane(scale, side, name, sign, feather, feather["width"]))
                    entry = copy.deepcopy(thermal_pattern)
                    entry["id"], entry["of"] = f"th-{name}-tertial-{feather['k']}", f"{name}-tertial-{feather['k']}"
                    thermal.append(entry)
                for feather in TERTIALS:
                    structure.append(rachis(scale, side, name, feather))
            if scale in ("100", "200"):
                for solid in body:
                    parts = solid["id"].split("-")
                    feather = len(parts) == 3 and parts[1] in ("primary", "secondary") and parts[2].isdigit()
                    web = scale == "200" and "-".join(parts[1:]) in ("propatagium", "arm-web", "hand-web")
                    if not feather and not web:
                        continue
                    thickest = max(section["rx"] for section in solid["sections"])
                    target = 0.0005 if feather else 0.0007
                    for section in solid["sections"]:
                        section["rx"] = round(section["rx"] * target / thickest, 6)
                    widest = max(section["ry"] for section in solid["sections"])
                    floor = {"100": 0.0105, "200": 0.0155}[scale]
                    if widest < floor:
                        for section in solid["sections"]:
                            section["ry"] = round(section["ry"] * floor / widest, 6)
                    solid["sheet"] = True
            notes = DECISIONS[scale]
            for index, note in notes.items():
                for language in ("en", "de"):
                    lists["decisions"][index][language] += " " + note[language]
    return True


DECISIONS = {
    "50": {
        0: {"en": "Five humeral feathers (tertials, 90 to 122 mm, vanes 34 to 37 mm wide and 1.2 mm thick) sit on their own bones on the humerus: folded they lie inside the covert panel, in flight they fan behind the upper arm and close the inner wing between the body and the secondaries.", "de": "Fünf Schirmfedern (90 bis 122 mm, Fahnen 34 bis 37 mm breit und 1,2 mm dick) sitzen auf eigenen Knochen am Oberarm: angelegt liegen sie in der Deckenfläche, im Flug fächern sie hinter dem Oberarm auf und schließen den Innenflügel zwischen Rumpf und Armschwingen."},
        2: {"en": "The coracoid is a bone of the rig of its own (57 mm, from the sternal facet up, back and out to the shoulder socket 36 mm beside the plane) and carries the wing; the scapula now ends at the same socket. The pelvis half is a fused bone of the rig from the edge of the dorsal shield to the hip socket 22 mm beside the plane, modelled as the acetabular wall that carries the femur. One rachis per humeral feather.", "de": "Das Rabenbein ist ein eigener Knochen des Rigs (57 mm, von der Brustbeinfacette nach oben, hinten und außen zur Schulterpfanne 36 mm neben der Ebene) und trägt den Flügel; das Schulterblatt endet jetzt an derselben Pfanne. Die Beckenhälfte ist ein verwachsener Knochen des Rigs vom Rand des Rückenschilds zur Hüftpfanne 22 mm neben der Ebene, modelliert als Pfannenwand, die den Oberschenkel trägt. Ein Schaft je Schirmfeder."},
    },
    "100": {
        0: {"en": "Flight feathers are true vanes 1 mm thick, declared as sheets; five humeral feathers on their own bones close the inner wing in flight.", "de": "Die Schwungfedern sind echte Fahnen von 1 mm Dicke, als Blätter deklariert; fünf Schirmfedern auf eigenen Knochen schließen im Flug den Innenflügel."},
        2: {"en": "The coracoid bar is bound to its own bone and runs from the sternum to the shoulder socket; the scapula ends there; the hand bones are a slender bar of their true section.", "de": "Der Rabenbeinstab hängt an seinem eigenen Knochen und läuft vom Brustbein zur Schulterpfanne; das Schulterblatt endet dort; die Handknochen sind ein schlanker Stab im wahren Querschnitt."},
    },
    "200": {
        0: {"en": "Flight feathers and wing membranes are true sheets (vanes 1 mm, membranes 1.4 mm); the outermost primary is widened to 31 mm so that it stays a feature of the scale; five humeral feathers close the inner wing in flight.", "de": "Schwungfedern und Flughäute sind echte Blätter (Fahnen 1 mm, Häute 1,4 mm); die äußerste Handschwinge ist auf 31 mm verbreitert, damit sie ein Merkmal des Maßstabs bleibt; fünf Schirmfedern schließen im Flug den Innenflügel."},
        2: {"en": "The coracoid bar on its own bone is set into the sternum, and one rib per side joins spine and sternum, so the wire skeleton is one connected load path.", "de": "Der Rabenbeinstab an seinem eigenen Knochen ist in das Brustbein eingebunden, und eine Rippe je Seite verbindet Wirbelsäule und Brustbein, so ist das Drahtskelett ein zusammenhängender Lastpfad."},
    },
    "500": {
        0: {"en": "One humeral blade 80 mm wide per wing on the middle humeral feather bone closes the inner wing in flight.", "de": "Eine 80 mm breite Schirmfederfläche je Flügel auf dem mittleren Schirmfederknochen schließt im Flug den Innenflügel."},
        2: {"en": "The coracoid bar is bound to its own bone and set into the sternum; one rachis per humeral feather.", "de": "Der Rabenbeinstab hängt an seinem eigenen Knochen und ist in das Brustbein eingebunden; ein Schaft je Schirmfeder."},
    },
}


def main():
    probe = rest(0, 90, 90)
    assert [round(probe[i][0]) for i in range(3)] == [0, 0, 1] and [round(probe[i][1]) for i in range(3)] == [1, 0, 0], probe
    assert angles(rest(31, -47, 112)) == [31, -47, 112]
    text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    if "--apply" not in sys.argv:
        document["asset"]["volumes"] = source["volumes"]
        if not convert(document["asset"], [document["asset"]["volumes"]]):
            raise SystemExit("already converted")
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        for bone in document["asset"]["rig"]["bones"]:
            if bone["id"][:-2] in ("coracoid", "ilium", "humerus", "thigh", "t1", "t5"):
                print(json.dumps({key: value for key, value in bone.items() if key not in ("title", "mass")}))
        print(CANDIDATE)
        return
    before = copy.deepcopy(document["asset"])
    if not convert(document["asset"], [document["asset"]["volumes"], source["volumes"]]):
        raise SystemExit("already converted")
    spec = importlib.util.spec_from_file_location("lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
    lateral = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(lateral)
    for key in ("rig", "poses"):
        text = lateral.splice(text, key, before[key], document["asset"][key])
    written = json.loads(text)["asset"]
    if written["rig"] != document["asset"]["rig"] or written["poses"] != document["asset"]["poses"]:
        raise SystemExit("splice did not round-trip")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
