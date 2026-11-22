"""💀 person-with-umbrella: hand-modelled skull, segmented vertebral column, ankle mortise and foot rays of the 1:50 structure list.

Runs after `rework-05-person-with-umbrella-girdles.py` on what that script wrote. Every number is a hand-chosen station in the frame of its bone (head: x up, y back, z left, origin at the atlanto-occipital joint; torso and neck: x up, y back; shank: x down, y ahead; foot and toe: x ahead, y up; z left everywhere).

Usage: python3 rework-05-person-with-umbrella-skeleton.py            candidate `🗑️generated/rework-person-with-umbrella/pwu-cand/🔣️.json`
       python3 rework-05-person-with-umbrella-skeleton.py --apply    volumes source and catalog `asset.volumes`
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "person-with-umbrella", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "person-with-umbrella.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-person-with-umbrella", "pwu-cand", "🔣️.json")

SPINE = [(0.0076, 0.036, 0.024, 0.018), (0.050, 0.026, 0.023, 0.017), (0.0926, 0.022, 0.022, 0.017), (0.150, 0.032, 0.020, 0.016), (0.2026, 0.048, 0.018, 0.015), (0.265, 0.060, 0.017, 0.014), (0.3226, 0.064, 0.016, 0.013), (0.380, 0.056, 0.015, 0.012), (0.4276, 0.044, 0.014, 0.012)]
NECK = [(-0.0219, 0.044, 0.013, 0.011), (0.020, 0.030, 0.013, 0.011), (0.0631, 0.020, 0.012, 0.010), (0.1061, 0.014, 0.013, 0.010)]
LUMBAR = [("l5", 0.000, 0.036), ("l4", 0.036, 0.072), ("l3", 0.072, 0.108), ("l2", 0.108, 0.144), ("l1", 0.144, 0.180)]
THORACIC = [(f"t{12 - i}", round(0.180 + 0.0225 * i, 4), round(0.2025 + 0.0225 * i, 4)) for i in range(12)]
CERVICAL = [(f"c{7 - i}", round(-0.0219 + 0.01829 * i, 5), round(-0.0219 + 0.01829 * (i + 1), 5)) for i in range(7)]
CRANIUM = [((-0.004, 0.004, 0), 0.050, 0.045), ((0.012, -0.004, 0), 0.058, 0.064), ((0.034, -0.008, 0), 0.0615, 0.072), ((0.052, -0.019, 0), 0.064, 0.085), ((0.075, -0.019, 0), 0.066, 0.085), ((0.100, -0.017, 0), 0.061, 0.077), ((0.114, -0.016, 0), 0.052, 0.065), ((0.124, -0.015, 0), 0.039, 0.049), ((0.130, -0.014, 0), 0.024, 0.030), ((0.133, -0.014, 0), 0.008, 0.010)]
JAW = [((-0.047, -0.024, 0.044), 0.011, 0.0055), ((-0.060, -0.050, 0.037), 0.012, 0.006), ((-0.068, -0.080, 0.019), 0.012, 0.006), ((-0.070, -0.088, 0.0), 0.012, 0.0065)]
RAMUS = [((0.012, -0.018, 0.050), 0.007, 0.005), ((-0.012, -0.022, 0.048), 0.015, 0.0045), ((-0.036, -0.026, 0.045), 0.016, 0.005), ((-0.049, -0.024, 0.044), 0.010, 0.005)]
TARSUS = [((0.020, -0.026, 0.0), 0.024, 0.017), ((0.044, -0.030, 0.001), 0.029, 0.015), ((0.066, -0.036, 0.002), 0.032, 0.012)]
METATARSALS = [((0.064, -0.034, -0.022), (0.128, -0.058, -0.028), 0.0075, 0.009), ((0.066, -0.032, -0.010), (0.134, -0.060, -0.012), 0.005, 0.006), ((0.066, -0.034, 0.002), (0.130, -0.061, 0.003), 0.005, 0.006), ((0.064, -0.037, 0.013), (0.123, -0.062, 0.017), 0.005, 0.006), ((0.058, -0.040, 0.024), (0.113, -0.062, 0.031), 0.005, 0.0065)]
TOES = [((0.004, 0.027, -0.029), (0.054, 0.021, -0.024), 0.008, 0.007), ((0.010, 0.025, -0.012), (0.056, 0.018, -0.013), 0.0055, 0.0045), ((0.006, 0.024, 0.003), (0.049, 0.018, 0.003), 0.0055, 0.0045), ((-0.001, 0.023, 0.017), (0.040, 0.018, 0.018), 0.0055, 0.0045), ((-0.011, 0.023, 0.031), (0.026, 0.018, 0.031), 0.0055, 0.0045)]
OWNED = ("cranium", "face-bones", "maxilla", "mandible", "spine", "cervical-spine", "joint-umbrella")
OWNED_PREFIX = ("mandible-", "nasal-process-", "zygoma-", "zygomatic-arch-", "spinous-", "transverse-", "talus-", "tarsus-", "midfoot-", "metatarsal-", "phalanges-l", "phalanges-r", "toe-ray-", "fibula-", "fibular-")


def norm(a):
    n = math.sqrt(sum(c * c for c in a))
    return [c / n for c in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def quaternion(m):
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [round(c, 5) for c in q]


def sections(rows, hint):
    """📐 Loft sections through `rows` (point, half extent along the hint, half extent across it)."""
    points = [list(row[0]) for row in rows]
    out = []
    for i, row in enumerate(rows):
        a, b = points[min(i + 1, len(points) - 1)], points[max(i - 1, 0)]
        tangent = norm([a[k] - b[k] for k in range(3)])
        dot = sum(hint[k] * tangent[k] for k in range(3))
        wide = norm([hint[k] - dot * tangent[k] for k in range(3)])
        other = cross(tangent, wide)
        out.append({"at": [round(c, 5) for c in points[i]], "rx": row[1], "ry": row[2], "exponent": 2, "rotation": quaternion([[wide[k], other[k], tangent[k]] for k in range(3)])})
    return out


def bone(sid, bone_id, part="member"):
    return {"id": sid, "bone": bone_id, "material": "bone", "role": "body", "tone": 0.9 if part == "member" else 0.4, "member": bone_id, "part": part}


def loft(sid, bone_id, rows, hint):
    return {**bone(sid, bone_id), "kind": "loft", "sections": sections(rows, hint), "capStart": True, "capEnd": True}


def capsule(sid, bone_id, p0, p1, r0, r1):
    return {**bone(sid, bone_id), "kind": "capsule", "p0": list(p0), "p1": list(p1), "r0": r0, "r1": r1}


def ellipsoid(sid, bone_id, centre, radii):
    return {**bone(sid, bone_id), "kind": "ellipsoid", "radii": list(radii), "transform": {"translation": list(centre)}}


def mirror(p, sign):
    return (p[0], p[1], sign * p[2])


def station(rows, x):
    """📏 Centre line and half extents of a column at height x, between its hand-stated stations."""
    if x <= rows[0][0]:
        return rows[0][1:]
    for a, b in zip(rows, rows[1:]):
        if x <= b[0]:
            t = (x - a[0]) / (b[0] - a[0])
            return tuple(a[k] + t * (b[k] - a[k]) for k in (1, 2, 3))
    return rows[-1][1:]


def column(sid, bone_id, rows, vertebrae, processes):
    """🦴 Vertebral bodies waisted between their end plates as one member, a spinous process per vertebra, transverse processes where they are long."""
    body = []
    out = []
    for name, low, high in vertebrae:
        pitch = high - low
        for x, scale in ((low + 0.10 * pitch, 1.0), (low + 0.5 * pitch, 0.84), (high - 0.10 * pitch, 1.0)):
            y, wide, deep = station(rows, x)
            body.append(((x, y, 0), round(wide * scale, 5), round(deep * scale, 5)))
        middle = low + 0.5 * pitch
        y, wide, deep = station(rows, middle)
        drop, reach, radius, lateral = processes(name)
        out.append(capsule(f"spinous-{name}", bone_id, (round(middle, 5), round(y + 0.7 * deep, 5), 0), (round(middle - drop, 5), round(y + deep + reach, 5), 0), radius, radius))
        if lateral:
            for side, sign in (("l", 1), ("r", -1)):
                out.append(capsule(f"transverse-{name}-{side}", bone_id, (round(middle, 5), round(y + 0.5 * deep, 5), sign * round(0.7 * wide, 5)), (round(middle, 5), round(y + 0.5 * deep + 0.006, 5), sign * round(wide + lateral, 5)), 0.0045, 0.004))
    return [loft(sid, bone_id, body, (0, 0, 1))] + out


def trunk(name):
    if name.startswith("l"):
        return 0.006, 0.026, 0.005, 0.024
    return 0.020, 0.022, 0.004, 0


def nape(name):
    return (0.008, 0.024, 0.0045, 0) if name == "c7" else (0.005, 0.014, 0.004, 0)


def skull():
    """💀 Neurocranium narrowed at the orbits, maxilla on its frontal processes, cheek bones with their arches, mandible as one arch with two rami."""
    out = [loft("cranium", "head", CRANIUM, (0, 0, 1)), ellipsoid("maxilla", "head", (-0.016, -0.080, 0), (0.026, 0.022, 0.038))]
    arch = JAW + [(mirror(row[0], -1), row[1], row[2]) for row in reversed(JAW[:-1])]
    out.append(loft("mandible", "head", arch, (1, 0, 0)))
    for side, sign in (("l", 1), ("r", -1)):
        out.append(capsule(f"nasal-process-{side}", "head", mirror((0.046, -0.092, 0.012), sign), mirror((-0.002, -0.094, 0.015), sign), 0.006, 0.006))
        out.append(capsule(f"zygoma-{side}", "head", mirror((0.046, -0.066, 0.050), sign), mirror((0.003, -0.073, 0.039), sign), 0.006, 0.012))
        out.append(capsule(f"zygomatic-arch-{side}", "head", mirror((0.006, -0.066, 0.047), sign), mirror((0.008, -0.014, 0.057), sign), 0.004, 0.004))
        out.append(loft(f"mandible-ramus-{side}", "head", [(mirror(row[0], sign), row[1], row[2]) for row in RAMUS], (0, 1, 0)))
    return out


def leg(side, sign):
    """🦵 Fibula with head and lateral malleolus in the tibial notch, talus in the mortise, tarsus, five metatarsals and five toes."""
    shank, foot, toe = f"shank-{side}", f"foot-{side}", f"toe-{side}"
    out = [
        ellipsoid(f"fibular-head-{side}", shank, mirror((0.040, -0.014, 0.030), sign), (0.014, 0.011, 0.011)),
        capsule(f"fibula-{side}", shank, mirror((0.040, -0.014, 0.030), sign), mirror((0.415, -0.008, 0.022), sign), 0.0068, 0.0072),
        ellipsoid(f"fibular-malleolus-{side}", shank, mirror((0.424, -0.006, 0.021), sign), (0.017, 0.011, 0.0105)),
        ellipsoid(f"talus-{side}", foot, (0.008, -0.016, 0), (0.028, 0.016, 0.020)),
        loft(f"tarsus-{side}", foot, [(mirror(row[0], sign), row[1], row[2]) for row in TARSUS], (0, 0, 1)),
    ]
    for ray, (p0, p1, r0, r1) in enumerate(METATARSALS, 1):
        out.append(capsule(f"metatarsal-{side}-{ray}", foot, mirror(p0, sign), mirror(p1, sign), r0, r1))
    for ray, (p0, p1, r0, r1) in enumerate(TOES, 1):
        out.append(capsule(f"toe-ray-{side}-{ray}", toe, mirror(p0, sign), mirror(p1, sign), r0, r1))
    return out


COARSE_CRANIUM = {"100": (0, 1, 2, 3, 4, 5, 7, 8), "200": (0, 2, 3, 4, 5, 7)}
STRUCTURE_50 = {
    "en": "1:50 structure: the skeleton carries the body. Skull: a neurocranium narrowed at the orbits, the maxilla hung on its two frontal processes beside the nasal opening, cheek bones with their arches, the mandible as one arch with two rami in the jaw joints. Column: seven cervical, twelve thoracic and five lumbar bodies waisted between their end plates along the neck lordosis, thoracic kyphosis and lumbar lordosis, a spinous process on each, transverse processes on the lumbar ones, a broad sacrum with coccyx. Twelve pairs of descending ribs with costal margin and floating ends, sternum. Shoulder girdle on its own lateral bones: S-shaped clavicle from the sternoclavicular to the acromioclavicular joint, scapula with blade, spine, acromion, glenoid neck and coracoid. Pelvic girdle on its own lateral bones from the sacroiliac joint to the hip joint: iliac wing, acetabulum, pubis to the symphysis, ischium and ischiopubic ramus. Femur with head, neck, trochanter and condyles, patella, tibia with plateau and medial malleolus, fibula with its head under the lateral condyle and the lateral malleolus in the tibial notch, talus held in that mortise, calcaneus, tarsus, five metatarsals and five toes; humerus, ulna, radius, carpus, metacarpals and finger rays of both hands. Joints at the rig joints, masses at the segment centres. The umbrella carries the wind through its real frame: the cloth loads the eight ribs, each propped by its stretcher from the runner, into notch and shaft and through the handle into the fist, where a grip joint ties it to the hand bones.",
    "de": "1:50 Tragwerk: Das Skelett trägt den Körper. Schädel: ein an den Augenhöhlen eingezogener Hirnschädel, der Oberkiefer an seinen zwei Stirnfortsätzen neben der Nasenöffnung, Jochbeine mit ihren Bögen, der Unterkiefer als ein Bogen mit zwei Ästen in den Kiefergelenken. Wirbelsäule: sieben Hals-, zwölf Brust- und fünf Lendenwirbelkörper, zwischen ihren Deckplatten tailliert, entlang Halslordose, Brustkyphose und Lendenlordose, je ein Dornfortsatz, Querfortsätze an den Lendenwirbeln, ein breites Kreuzbein mit Steißbein. Zwölf absteigende Rippenpaare mit Rippenbogen und freien Enden, Brustbein. Schultergürtel auf eigenen seitlichen Knochen: S-förmiges Schlüsselbein vom Brustbein-Schlüsselbein- zum Schultereckgelenk, Schulterblatt mit Blatt, Gräte, Schulterhöhe, Pfannenhals und Rabenschnabelfortsatz. Beckengürtel auf eigenen seitlichen Knochen vom Kreuzdarmbein- zum Hüftgelenk: Darmbeinschaufel, Hüftpfanne, Schambein bis zur Schamfuge, Sitzbein und Sitz-Scham-Ast. Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Kniescheibe, Schienbein mit Plateau und Innenknöchel, Wadenbein mit seinem Kopf unter dem äußeren Kondylus und dem Außenknöchel in der Schienbeinkerbe, das Sprungbein in dieser Gabel, Fersenbein, Fußwurzel, fünf Mittelfußknochen und fünf Zehen; Oberarmknochen, Elle, Speiche, Handwurzel, Mittelhand und Fingerstrahlen beider Hände. Gelenke an den Rig-Gelenken, Massen in den Segmentschwerpunkten. Der Schirm trägt den Wind über sein echtes Gestell: der Stoff belastet die acht Rippen, jede vom Schieber aus durch ihre Strebe gestützt, in Krone und Stock und über den Griff in die Faust, wo ein Griffgelenk ihn an die Handknochen bindet.",
}
REWORDED = {
    "100": [("skull, face and jaw", "skull narrowed at the orbits, face and jaw"), ("clavicles, scapulae, ilia, sacrum and ischiopubic bars", "clavicles and hip bones on their own lateral bones (sternoclavicular to acromioclavicular joint, sacroiliac joint to hip joint), scapulae with blade, spine, acromion and neck, iliac wings, acetabula, pubic and ischial bars, a broad sacrum"), ("Schädel, Gesicht und Kiefer", "an den Augenhöhlen eingezogener Schädel, Gesicht und Kiefer"), ("Schlüsselbeine, Schulterblätter, Darmbeine, Kreuzbein und Sitz-Scham-Stege", "Schlüsselbeine und Hüftbeine auf eigenen seitlichen Knochen (Brustbein-Schlüsselbein- bis Schultereckgelenk, Kreuzdarmbein- bis Hüftgelenk), Schulterblätter mit Blatt, Gräte, Schulterhöhe und Hals, Darmbeinschaufeln, Hüftpfannen, Scham- und Sitzbeinstege, ein breites Kreuzbein")],
    "200": [("a shoulder girdle bar, a pelvic ring", "the clavicles as slender rods on their own lateral bones, scapula plates, a pelvic ring with a hip bone on each side"), ("ein Schultergürtelstab, ein Beckenring", "die Schlüsselbeine als schlanke Stäbe auf ihren eigenen seitlichen Knochen, Schulterblattplatten, ein Beckenring mit je einem Hüftbein")],
    "500": [("shoulder and pelvic bars", "the clavicles as slender rods on their own lateral bones and a pelvic bar"), ("one stick per limb segment and foot", "the long bones at their true shaft section as slender rods, one stick per foot"), ("joints at hips, knees, trunk and neck", "joints at hips, knees, shoulders, trunk and neck"), ("Schulter- und Beckenstab", "die Schlüsselbeine als schlanke Stäbe auf ihren eigenen seitlichen Knochen und ein Beckenstab"), ("ein Stab je Gliedabschnitt und Fuß", "die langen Knochen mit ihrem wahren Schaftquerschnitt als schlanke Stäbe, ein Stab je Fuß"), ("Gelenke an Hüften, Knien, Rumpf und Hals", "Gelenke an Hüften, Knien, Schultern, Rumpf und Hals")],
}


def coarse(volumes):
    """💀 Coarser scales keep the cranium of 1:50 in fewer sections; the decisions say what each structure list holds."""
    for scale, rows in COARSE_CRANIUM.items():
        lists = volumes[scale]["structure"]
        at = next(i for i, solid in enumerate(lists) if solid["id"] == "cranium")
        lists[at] = loft("cranium", "head", [CRANIUM[i] for i in rows], (0, 0, 1))
    volumes["50"]["decisions"][2] = dict(STRUCTURE_50)
    for scale, pairs in REWORDED.items():
        text = volumes[scale]["decisions"][2]
        for old, fresh in pairs:
            for language in ("en", "de"):
                text[language] = text[language].replace(old, fresh)


def rebuild(volumes):
    coarse(volumes)
    lists = volumes["50"]
    kept = [solid for solid in lists["structure"] if solid["id"] not in OWNED and not solid["id"].startswith(OWNED_PREFIX)]
    grip = {**bone("joint-umbrella", "umbrella", "joint"), "kind": "ellipsoid", "radii": [0.018, 0.018, 0.018], "transform": {"translation": [0, 0, 0]}}
    made = [grip] + column("spine", "torso", SPINE, LUMBAR + THORACIC, trunk) + column("cervical-spine", "neck", NECK, CERVICAL, nape) + skull() + leg("l", 1) + leg("r", -1)
    at = next((i for i, solid in enumerate(kept) if solid["id"] == "joint-torso"), len(kept))
    lists["structure"] = kept[:at] + made + kept[at:]
    print(f"[DEBUG] 1:50 structure {len(lists['structure'])} solids")


def main():
    if "--apply" in sys.argv:
        source = json.load(open(SOURCE, encoding="utf-8"))
        rebuild(source["volumes"])
        document = json.load(open(CATALOG, encoding="utf-8"))
        document["asset"]["volumes"] = source["volumes"]
        for path, value in ((SOURCE, source), (CATALOG, document)):
            with open(path, "w", encoding="utf-8") as handle:
                json.dump(value, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
        print("[DEBUG] applied to catalog and source")
        return
    document = json.load(open(CANDIDATE, encoding="utf-8"))
    rebuild(document["asset"]["volumes"])
    with open(CANDIDATE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
    print("[DEBUG] candidate", CANDIDATE)


if __name__ == "__main__":
    main()
