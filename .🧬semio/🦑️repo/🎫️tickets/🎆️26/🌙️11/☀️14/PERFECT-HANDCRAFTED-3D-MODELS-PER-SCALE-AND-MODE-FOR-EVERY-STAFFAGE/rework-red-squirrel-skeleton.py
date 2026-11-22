"""💀 Red squirrel, 1:50 skeleton: radius and ulna, tibia and fibula, carpus with metacarpals, tarsus with metatarsals,
skull with zygomatic arches, a mandible of two rami and incisors; the sacrum at its true size at every scale.

All numbers are hand-set in metres in the frame of the bone (u along the bone, v its normal, w lateral; near side is -w).

Usage: python3 rework-red-squirrel-skeleton.py candidate|publish
"""
import copy
import json
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-squirrel/🔣️.json"
SOURCE = "volumes/animals/red-squirrel.json"
CANDIDATE = "🗑️generated/r3-red-squirrel/🔣️.json"
ALONG = [0, -0.7071068, 0, 0.7071068]
SIDES = (("n", "near", -1), ("f", "far", 1))

RADIUS = [(0.0, 0.0012, 0.0019, 0.0016), (0.004, 0.0013, 0.0013, 0.0012), (0.02, 0.0012, 0.0013, 0.0011), (0.034, 0.0009, 0.0017, 0.0013), (0.0395, 0.0006, 0.0024, 0.0017), (0.0412, 0.0005, 0.0016, 0.0012)]
ULNA = [(-0.0058, -0.0016, 0.0012, 0.0014), (-0.003, -0.0014, 0.0014, 0.002), (0.0, -0.0017, 0.0015, 0.0017), (0.004, -0.0018, 0.0012, 0.0014), (0.02, -0.0016, 0.0009, 0.0009), (0.034, -0.0014, 0.0008, 0.0008), (0.0395, -0.0012, 0.0009, 0.0009), (0.041, -0.001, 0.0006, 0.0006)]
FIBULA = [(0.002, -0.0006, 0.0028, 0.0009, 0.001), (0.006, -0.0006, 0.0029, 0.0006, 0.0006), (0.03, -0.0004, 0.0021, 0.00055, 0.00055), (0.052, -0.0003, 0.0021, 0.0006, 0.0006), (0.058, -0.0002, 0.0024, 0.0009, 0.001), (0.0595, -0.0002, 0.0022, 0.0006, 0.0006)]
CARPUS = [(-0.0015, 0.0, 0.0028, 0.0027), (0.00125, -0.0001, 0.00323, 0.00232), (0.004, -0.0002, 0.0036, 0.002), (0.0056, -0.0003, 0.003, 0.0014)]
TARSUS = [(-0.0074, -0.0032, 0.0016, 0.002), (-0.0052, -0.00283, 0.00198, 0.00245), (-0.003, -0.0024, 0.0024, 0.0028), (-0.0015, -0.00181, 0.00287, 0.00294), (0.0, -0.0014, 0.0032, 0.003), (0.006, -0.0018, 0.00332, 0.00244), (0.0095, -0.002, 0.0032, 0.0019), (0.0115, -0.0021, 0.0026, 0.0013)]
CRANIUM_BREADTH = {0.0225: 0.01, 0.026: 0.0086, 0.029: 0.0078, 0.032: 0.0068, 0.036: 0.006}
ARCH = [(0.016, -0.0008, 0.0116, 0.0009, 0.0015), (0.0195, -0.0022, 0.0142, 0.0007, 0.0013), (0.0245, -0.003, 0.015, 0.0007, 0.0012), (0.0295, -0.003, 0.0128, 0.0007, 0.0013), (0.0345, -0.0024, 0.0066, 0.0009, 0.0016)]
RAMUS = [(0.006, -0.0005, 0.0104, 0.001, 0.0016), (0.0085, -0.0042, 0.0098, 0.0009, 0.0042), (0.013, -0.0062, 0.0088, 0.0009, 0.0036), (0.02, -0.007, 0.0068, 0.0009, 0.003), (0.027, -0.0073, 0.0046, 0.0009, 0.0025), (0.034, -0.0068, 0.0026, 0.0009, 0.0018), (0.0405, -0.0058, 0.0011, 0.001, 0.0014)]
LOWER_INCISORS = [(0.0385, -0.0058, 0.0, 0.0013, 0.0008), (0.043, -0.0046, 0.0, 0.0012, 0.0007), (0.0462, -0.003, 0.0, 0.0011, 0.0006), (0.0474, -0.0016, 0.0, 0.0009, 0.0004)]
UPPER_INCISORS = [(0.0485, 0.0002, 0.0, 0.0013, 0.0008), (0.0506, -0.001, 0.0, 0.0013, 0.0007), (0.0506, -0.0026, 0.0, 0.0012, 0.0006), (0.0494, -0.0038, 0.0, 0.001, 0.0004)]
SACRUM_START, SACRUM_OLD_START, SACRUM_END, SACRUM_BREADTH = 0.019, 0.013, 0.0385, 0.82


def section(x, y, z, rx, ry):
    return {"at": [round(x, 5), round(y, 5), round(z, 5)], "rx": rx, "ry": ry, "exponent": 2, "rotation": ALONG}


def member(template, name, bone, sections, slender):
    solid = {k: v for k, v in template.items() if k not in ("sections", "slender", "members")}
    solid.update(id=name, bone=bone, member=bone, sections=sections)
    if slender:
        solid["slender"] = True
    return solid


def start_of(structure, name):
    return next(s for s in structure if s["id"] == name)["sections"][0]["at"]


def skeleton(structure):
    if any(s["id"] == "ulna-n" for s in structure):
        raise SystemExit("skeleton already rebuilt")
    template = next(s for s in structure if s["id"] == "m-forearm-n")
    out = []
    for solid in structure:
        name = solid["id"]
        for letter, word, sign in SIDES:
            if name == "m-forearm-" + letter:
                bone = "forearm-" + letter
                solid = member(template, name, bone, [section(x, y, 0, rx, ry) for x, y, rx, ry in RADIUS], True)
                out.append(solid)
                solid = member(template, "ulna-" + letter, bone, [section(x, y, 0, rx, ry) for x, y, rx, ry in ULNA], True)
            elif name == "m-shank-" + letter:
                out.append(solid)
                solid = member(template, "fibula-" + letter, "shank-" + letter, [section(x, y, sign * z, rx, ry) for x, y, z, rx, ry in FIBULA], True)
            elif name == "m-hand-" + letter:
                bone = "hand-" + letter
                out.append(member(template, name, bone, [section(x, y, 0, rx, ry) for x, y, rx, ry in CARPUS], False))
                for digit in range(1, 5):
                    x, y, z = start_of(structure, f"phalanx-{word}-hand-digit-{digit}")
                    out.append(member(template, f"metacarpal-{word}-{digit}", bone, [section(0.0045, -0.0003, 0.7 * z, 0.0009, 0.0008), section(0.009, (y - 0.0003) / 2, 0.87 * z, 0.0007, 0.0007), section(x + 0.0006, y, z, 0.0009, 0.0008)], False))
                solid = None
            elif name == "m-foot-" + letter:
                bone = "foot-" + letter
                out.append(member(template, name, bone, [section(x, y, 0, rx, ry) for x, y, rx, ry in TARSUS], False))
                for digit in range(1, 6):
                    x, y, z = start_of(structure, f"phalanx-{word}-foot-digit-{digit}")
                    out.append(member(template, f"metatarsal-{word}-{digit}", bone, [section(0.0105, -0.0021, 0.72 * z, 0.0009, 0.0009), section(0.0195, -0.0019, 0.9 * z, 0.0007, 0.0007), section(x + 0.0006, y, z, 0.001, 0.0009)], x + 0.0006 - 0.0105 >= 0.0155))
                solid = None
        if solid is None:
            continue
        if name == "m-head":
            for s in solid["sections"]:
                if s["at"][0] in CRANIUM_BREADTH:
                    s["rx"] = CRANIUM_BREADTH[s["at"][0]]
            out.append(solid)
            for letter, word, sign in SIDES:
                out.append(member(template, "zygomatic-arch-" + letter, "head", [section(x, y, sign * z, rx, ry) for x, y, z, rx, ry in ARCH], True))
            out.append(member(template, "incisors-upper", "head", [section(*row) for row in UPPER_INCISORS], False))
            continue
        if name == "mandible":
            for letter, word, sign in SIDES:
                out.append(member(template, "mandible-" + letter, "head", [section(x, y, sign * z, rx, ry) for x, y, z, rx, ry in RAMUS], False))
            out.append(member(template, "incisors-lower", "head", [section(*row) for row in LOWER_INCISORS], False))
            continue
        out.append(solid)
    return out


def sacrum(structure):
    for solid in structure:
        if solid["id"] == "sacrum":
            for s in solid["sections"]:
                x = s["at"][0]
                s["at"][0] = round(SACRUM_END - (SACRUM_END - x) * (SACRUM_END - SACRUM_START) / (SACRUM_END - SACRUM_OLD_START), 5)
                if s["rx"] > 0.0045:
                    s["rx"] = round(max(0.0045, s["rx"] * SACRUM_BREADTH), 5)


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = json.loads(text)
    source = json.load(open(SOURCE, encoding="utf-8"))
    if source["volumes"] != document["asset"]["volumes"]:
        raise SystemExit("source and catalog volumes differ")
    volumes = source["volumes"]
    volumes["50"]["structure"] = skeleton(volumes["50"]["structure"])
    for scale in volumes:
        sacrum(volumes[scale]["structure"])
    print("[DEBUG] 1:50 structure solids", len(volumes["50"]["structure"]))
    if sys.argv[1] == "candidate":
        document["asset"]["volumes"] = volumes
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        return
    json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("source written; run volumes-insert.py")


main()
