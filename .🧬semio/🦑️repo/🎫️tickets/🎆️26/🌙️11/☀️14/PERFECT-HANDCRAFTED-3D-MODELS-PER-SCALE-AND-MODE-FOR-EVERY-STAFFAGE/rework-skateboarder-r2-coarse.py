"""🛹️ Second round, pass 1 of `humans/skateboarder`: coarse scales read as the same body.

- Limb segments of 1:100, 1:200 and 1:500 ended in flat cuts that showed as boxes at knee, elbow and shoulder: the ends that lie
  inside a joint get a rounded end inside their own length (hems and cuffs stay open cuts, as cloth is). The 1:500 thigh
  ran 74 mm past the knee and stood out behind the bent knee: it ends 50 mm past the knee centre, one knee radius.
- Structure 1:200: five true rib pairs (1, 3, 5, 7, 9) on the paths of the 1:100 skeleton and a sternum at the 30 mm feature
  instead of three loose hoops. Structure 1:500: one thoracic cage (ribs cannot be shown at 75 mm) around the spine.
- Decisions of 1:200 and 1:500 say what the lists hold.

Usage: python3 rework-skateboarder-r2-coarse.py   (edits T/volumes/humans/skateboarder.json in place; publish with volumes-insert.py)
"""
import copy
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "skateboarder.json")
QUARTER = [0, -0.7071068, 0, 0.7071068]
ROUNDED = {"sleeve": ("start", "end"), "forearm": ("start",), "thigh": ("start", "end"), "shank": ("start",)}
ARC = (0.4, 0.7, 0.9, 0.97)
KNEE_END_500 = 0.468
RIBS_200 = (1, 3, 5, 7, 9)
RIB_RADIUS = 0.0045
STERNUM_RADIUS = 0.0152
CAGE = (
    (0.186, -0.045, 0.060, 0.030, 2.0),
    (0.200, -0.046, 0.110, 0.052, 2.3),
    (0.235, -0.049, 0.127, 0.064, 2.3),
    (0.285, -0.052, 0.134, 0.074, 2.3),
    (0.345, -0.044, 0.118, 0.068, 2.3),
    (0.395, -0.033, 0.082, 0.054, 2.2),
    (0.425, -0.026, 0.050, 0.040, 2.0),
    (0.437, -0.024, 0.025, 0.020, 2.0),
)
DECISIONS = {
    "200": {2: {
        "en": "1:200 structure: skull with facial skeleton, cervical and thoracolumbar column in their curvature, five true rib pairs (1, 3, 5, 7, 9) from the column round to a sternum at the 30 mm feature, clavicles from sternum to acromion, hip bones with sacrum and pubic arch, and the long bones at their true shafts as slender members. The board carries the rider through deck, two trucks and four wheels.",
        "de": "1:200-Tragwerk: Schädel mit Gesichtsschädel, Hals- und Brust-Lenden-Wirbelsäule in ihrer Krümmung, fünf Rippenpaare (1, 3, 5, 7, 9) im wahren Querschnitt von der Wirbelsäule bis zum Brustbein an der 30-mm-Grenze, Schlüsselbeine vom Brustbein zum Schulterdach, Hüftbeine mit Kreuzbein und Schambogen sowie die Röhrenknochen mit wahrem Schaft als schlanke Stäbe. Das Brett trägt den Fahrer über Deck, zwei Achsen und vier Rollen."}},
    "500": {
        0: {
            "en": "1:500 body: two to five sections per mass at the 75 mm floor: trunk with the hood merged, head and beanie two forms, one form per arm and leg segment with rounded ends inside elbow, knee, shoulder and hip, the hands blunt mittens, each shoe one form. Deck, trucks and wheels are one board form 0.81 m long, 0.2 m wide and 0.09 m high with both kicks, because an 11.5 mm deck and 54 mm wheels do not print at this scale.",
            "de": "1:500-Körper: zwei bis fünf Querschnitte je Masse an der 75-mm-Grenze: Rumpf mit aufgegangener Kapuze, Kopf und Mütze zwei Formen, eine Form je Arm- und Beinsegment mit gerundeten Enden in Ellbogen, Knie, Schulter und Hüfte, die Hände stumpfe Fäustlinge, jeder Schuh eine Form. Deck, Achsen und Rollen sind eine Brettform von 0,81 m Länge, 0,2 m Breite und 0,09 m Höhe mit Nose und Tail, weil ein 11,5-mm-Deck und 54-mm-Rollen in diesem Maßstab nicht druckbar sind."},
        2: {
            "en": "1:500 structure: skull, one column from sacrum to atlas at the feature, one thoracic cage around the column in place of ribs and sternum, clavicles at their true section, one pelvis, and one member per limb segment with joints at shoulder, elbow, hip, knee and ankle. The board form carries the rider to the ground as one member.",
            "de": "1:500-Tragwerk: Schädel, eine Wirbelsäule vom Kreuzbein bis zum Atlas an der Darstellungsgrenze, ein Brustkorb um die Wirbelsäule anstelle von Rippen und Brustbein, Schlüsselbeine im wahren Querschnitt, ein Becken und ein Stab je Gliedmaßensegment mit Gelenken an Schulter, Ellbogen, Hüfte, Knie und Sprunggelenk. Die Brettform leitet die Last des Fahrers als ein Stab in den Boden."}},
}


def r5(value):
    return round(value, 5)


def rounded(sections, end):
    """🥚️ The loft with one end replaced by a rounded end inside its own length."""
    ordered = sections if end == "start" else sections[::-1]
    tip, inner = ordered[0], ordered[1]
    span = [b - a for a, b in zip(tip["at"], inner["at"])]
    length = math.sqrt(sum(c * c for c in span))
    radius = min(tip["rx"], tip["ry"])
    if radius < 0.02 or length < 0.5 * radius or min(inner["rx"], inner["ry"]) < 0.6 * radius:
        return None
    depth = min(0.9 * radius, 0.6 * length)
    share = depth / length
    base = {"at": [a + s * share for a, s in zip(tip["at"], span)], "rx": tip["rx"] + (inner["rx"] - tip["rx"]) * share, "ry": tip["ry"] + (inner["ry"] - tip["ry"]) * share}
    dome = []
    for t in (0.0,) + ARC:
        section = copy.deepcopy(tip)
        scale = math.sqrt(1 - t * t)
        section["at"] = [r5(b - s * share * t) for b, s in zip(base["at"], span)]
        section["rx"], section["ry"] = r5(base["rx"] * scale), r5(base["ry"] * scale)
        dome.append(section)
    out = dome[::-1] + ordered[1:]
    return out if end == "start" else out[::-1]


def round_limbs(volumes, log):
    for solid in volumes["500"]["body"]:
        if solid["id"] in ("thigh-l", "thigh-r"):
            solid["sections"][-1]["at"][0] = KNEE_END_500
    for scale in ("100", "200", "500"):
        for solid in volumes[scale]["body"]:
            stem = solid["id"].rsplit("-", 1)[0]
            if solid.get("kind") != "loft" or stem not in ROUNDED or not solid.get("bone", "").endswith(("-l", "-r")):
                continue
            for end in ROUNDED[stem]:
                sections = rounded(solid["sections"], end)
                if sections is not None:
                    solid["sections"] = sections
                    log.append(f"1:{scale} {solid['id']} {end}")


def mirrored(solid):
    twin = copy.deepcopy(solid)
    twin["id"] = solid["id"][:-2] + "-r"
    for section in twin["sections"]:
        section["at"] = [section["at"][0], section["at"][1], -section["at"][2]]
    return twin


def ribs_200(volumes):
    fine = {s["id"]: s for s in volumes["100"]["structure"]}
    kept = [s for s in volumes["200"]["structure"] if not s["id"].startswith("open-")]
    cage = []
    for number in RIBS_200:
        rib = copy.deepcopy(fine[f"costal-{number}-l"])
        for section in rib["sections"]:
            section["rx"] = section["ry"] = RIB_RADIUS
        rib["slender"] = True
        cage += [rib, mirrored(rib)]
    sternum = copy.deepcopy(fine["sternum"])
    for section in sternum["sections"]:
        section["rx"] = section["ry"] = STERNUM_RADIUS
    at = next(i for i, s in enumerate(kept) if s["id"] == "spine") + 1
    volumes["200"]["structure"] = kept[:at] + cage + [sternum] + kept[at:]


def cage_500(volumes):
    kept = [s for s in volumes["500"]["structure"] if not s["id"].startswith("open-")]
    spine = next(s for s in kept if s["id"] == "spine")
    cage = {"id": "ribcage", "kind": "loft", "bone": "torso", "material": spine["material"], "role": "body", "tone": spine["tone"],
            "sections": [{"at": [u, v, 0], "rx": rx, "ry": ry, "exponent": e, "rotation": QUARTER} for u, v, rx, ry, e in CAGE],
            "capStart": True, "capEnd": True, "member": "torso", "part": "member"}
    at = kept.index(spine) + 1
    volumes["500"]["structure"] = kept[:at] + [cage] + kept[at:]


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    volumes = document["volumes"]
    log = []
    round_limbs(volumes, log)
    ribs_200(volumes)
    cage_500(volumes)
    for scale, entries in DECISIONS.items():
        for index, text in entries.items():
            volumes[scale]["decisions"][index] = text
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("\n".join(log))
    for scale in ("200", "500"):
        print(scale, [s["id"] for s in volumes[scale]["structure"] if s.get("bone") == "torso"])


main()
