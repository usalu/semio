"""🎸 Seated guitarist, second round: true long-bone shafts at every scale, closed limb ends at 1:100 and 1:200, the sound box outline of 1:100 through fifteen stations and the shirt yoke carried out to the acromion. Idempotent: every solid is stated whole."""
import json
import math
import pathlib

T = pathlib.Path(__file__).parent
SOURCE = T / "volumes/humans/seated-guitarist.json"
ROT = [0, -0.70711, 0, 0.70711]
ALONG = [0.5, 0.5, 0.5, 0.5]
BONE = {"material": "bone", "role": "body", "tone": 0.62, "capStart": True, "capEnd": True, "part": "member"}

SHAFTS = {
    "humerus": ("upper-arm", [([0, 0, 0], 0.0125), ([0.158, 0.002, 0], 0.011), ([0.311, 0, 0], 0.0115)]),
    "radius": ("forearm", [([0.012, 0.004, 0.018], 0.008), ([0.133, 0.002, 0.022], 0.0075), ([0.253, 0, 0.016], 0.011)]),
    "ulna": ("forearm", [([-0.022, -0.014, -0.008], 0.011), ([0.02, -0.004, -0.014], 0.0105), ([0.251, 0, -0.019], 0.0075)]),
    "tibia": ("shank", [([0.02, 0.01, -0.004], 0.019), ([0.12, 0.014, -0.006], 0.015), ([0.3, 0.01, -0.004], 0.0125), ([0.425, 0.002, -0.004], 0.017)]),
    "fibula": ("shank", [([0.024, -0.01, 0.03], 0.0085), ([0.22, -0.012, 0.034], 0.006), ([0.438, -0.008, 0.03], 0.0095)]),
}
FEMUR_COARSE = [([0, 0, 0.03], 0.017), ([0.2, 0.006, 0.02], 0.0145), ([0.42, 0, 0], 0.02)]
TIBIA_COARSE = [([0, 0.006, 0], 0.019), ([0.2, 0.012, 0], 0.0125), ([0.428, 0.002, 0], 0.017)]

GUITAR_100 = [(-0.29, 0.06), (-0.282, 0.115), (-0.262, 0.16), (-0.225, 0.187), (-0.175, 0.198), (-0.125, 0.196), (-0.075, 0.177), (-0.028, 0.1495), (0.02, 0.1365), (0.065, 0.1395), (0.115, 0.145), (0.162, 0.1445), (0.195, 0.129), (0.21, 0.105), (0.218, 0.075)]

YOKE = {
    "50": {0.456: (0.19, 0.074), 0.472: (0.14, 0.066)},
    "100": {0.462: (0.178, 0.07)},
    "200": {0.465: (0.168, 0.07)},
    "500": {0.465: (0.168, 0.072)},
}

DOME = [(0.45, 0.8), (0.75, 0.5), (0.88, 0.22)]
CLOSE = {
    "100": {"sleeve": "end", "thigh": "end", "shank": "end"},
    "200": {"sleeve": "end", "forearm": "start", "thigh": "both", "shank": "both"},
}


def shaft(name, side, bone, stations, slender, turned=False):
    sign = 1 if side == "l" else -1
    solid = {"id": f"{name}-{side}", "kind": "loft", "bone": f"{bone}-{side}", **BONE, "member": f"{bone}-{side}",
             "sections": [{"at": [p[0], p[1], sign * p[2]], "rx": r, "ry": r, **({"rotation": ALONG} if turned else {})} for p, r in stations]}
    if slender:
        solid["slender"] = True
    return solid


def put(solids, solid, after=None):
    for i, s in enumerate(solids):
        if s["id"] == solid["id"]:
            solids[i] = solid
            return
    at = next((i + 1 for i, s in enumerate(solids) if s["id"] == after), len(solids))
    solids.insert(at, solid)


def drop(solids, ident):
    solids[:] = [s for s in solids if s["id"] != ident]


def dome(sections, end):
    a, b = (sections[-1], sections[-2]) if end else (sections[0], sections[1])
    d = [a["at"][i] - b["at"][i] for i in range(3)]
    n = math.sqrt(sum(c * c for c in d))
    d = [c / n for c in d]
    radius = max(a["rx"], a["ry"])
    if radius < 0.03 or a["rx"] < 0.45 * max(s["rx"] for s in sections):
        return
    extra = [{**a, "at": [round(a["at"][i] + d[i] * f * radius, 5) for i in range(3)], "rx": round(a["rx"] * k, 5), "ry": round(a["ry"] * k, 5)} for f, k in DOME]
    if end:
        sections.extend(extra)
    else:
        sections[:0] = extra[::-1]


document = json.loads(SOURCE.read_text())
volumes = document["volumes"]

for scale in ("100", "200", "500"):
    structure = volumes[scale]["structure"]
    for side in "lr":
        for name in ("humerus", "radius", "ulna"):
            bone, stations = SHAFTS[name]
            put(structure, shaft(name, side, bone, stations, True), after=f"forearm-bones-{side}" if name != "humerus" else None)
        drop(structure, f"forearm-bones-{side}")
        if scale == "100":
            put(structure, shaft("fibula", side, *SHAFTS["fibula"], True))
        else:
            put(structure, shaft("femur", side, "thigh", FEMUR_COARSE, True, True))
            put(structure, shaft("tibia", side, "shank", TIBIA_COARSE, True, True))

for scale, stations in YOKE.items():
    shirt = next(s for s in volumes[scale]["body"] if s["id"] == "shirt")
    for height, (rx, ry) in stations.items():
        found = next((s for s in shirt["sections"] if abs(s["at"][0] - height) < 1e-6), None)
        if found:
            found.update(rx=rx, ry=ry)
        else:
            shirt["sections"].append({"at": [height, 0.011, 0], "rx": rx, "ry": ry, "exponent": 2.4, "rotation": ROT})
    shirt["sections"].sort(key=lambda s: s["at"][0])

for scale, parts in CLOSE.items():
    for solid in volumes[scale]["body"]:
        stem = solid["id"].rsplit("-", 1)[0]
        if stem in parts and solid["kind"] == "loft":
            if parts[stem] in ("end", "both"):
                dome(solid["sections"], True)
            if parts[stem] in ("start", "both"):
                dome(solid["sections"], False)

body = volumes["100"]["body"]
box = next(s for s in body if s["id"] == "guitar-body")
box["sections"] = [{"at": [x, round(0.007 - 0.0256 * (x + 0.29), 5), -0.1365], "rx": rx, "ry": round(0.062 - 0.0256 * (x + 0.29), 5), "exponent": 8, "rotation": ROT} for x, rx in GUITAR_100]
top = next(s for s in body if s["id"] == "guitar-top")
top["sections"] = [{"at": [x, -0.0555, -0.1365], "rx": round(rx - 0.004, 5), "ry": 0.0025, "exponent": 8, "rotation": ROT} for x, rx in GUITAR_100]

SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("[DEBUG] written", SOURCE)

DECISIONS = {
    "50": ("Shirt yoke: the shoulder line runs from the collar out to the acromion (half breadth 190 mm at 0.456 m, 140 mm at 0.472 m above L5/S1), so the sleeve head sits under the yoke instead of standing above it.", "Hemdpasse: Die Schulterlinie läuft vom Kragen bis zum Akromion (halbe Breite 190 mm auf 0,456 m, 140 mm auf 0,472 m über L5/S1), der Ärmelkopf sitzt damit unter der Passe und steht nicht darüber."),
    "100": ("Second round: the sound box runs through fifteen stations of the dreadnought outline (lower bout 397 mm, waist 273 mm, upper bout 292 mm) instead of eight; sleeves, thighs and shanks end in a dome at elbow and knee so no flat disc shows when the joint bends; radius, ulna and fibula keep their true shafts of 15 mm and 12 mm as slender members.", "Zweite Runde: Der Korpus läuft durch fünfzehn Stationen des Dreadnought-Umrisses (Unterbug 397 mm, Taille 273 mm, Oberbug 292 mm) statt durch acht; Ärmel, Oberschenkel und Unterschenkel enden an Ellbogen und Knie in einer Kuppe, damit beim Beugen keine flache Scheibe sichtbar wird; Speiche, Elle und Wadenbein behalten ihre wahren Schäfte von 15 mm und 12 mm als schlanke Stäbe."),
    "200": ("Second round, structure: humerus, radius, ulna, femur and tibia keep the shaft sections of 1:50 (22, 15, 15, 29 and 25 mm at the narrowest) as slender members between the joint markers; the merged forearm rod of 30 mm is gone and the fibula is left out. Body: sleeves, forearms, thighs and shanks end in a dome at elbow and knee.", "Zweite Runde, Tragwerk: Oberarmknochen, Speiche, Elle, Oberschenkelknochen und Schienbein behalten die Schaftquerschnitte von 1:50 (22, 15, 15, 29 und 25 mm an der engsten Stelle) als schlanke Stäbe zwischen den Gelenkmarken; der zusammengefasste Unterarmstab von 30 mm entfällt, das Wadenbein bleibt weg. Körper: Ärmel, Unterarme, Oberschenkel und Unterschenkel enden an Ellbogen und Knie in einer Kuppe."),
    "500": ("Second round, structure: the long bones are no longer thickened to 75 mm. Humerus, radius, ulna, femur and tibia keep the shaft sections of 1:50 as slender members between joint markers of 78 mm; spine, shoulder bar, pelvic bar, hands and feet stay at the feature size.", "Zweite Runde, Tragwerk: Die Röhrenknochen sind nicht mehr auf 75 mm verdickt. Oberarmknochen, Speiche, Elle, Oberschenkelknochen und Schienbein behalten die Schaftquerschnitte von 1:50 als schlanke Stäbe zwischen Gelenkmarken von 78 mm; Wirbelsäule, Schulterstab, Beckenstab, Hände und Füsse bleiben in Merkmalsgrösse."),
}
for scale, (en, de) in DECISIONS.items():
    entries = document["volumes"][scale]["decisions"]
    entries[:] = [e for e in entries if not e["en"].startswith(en[:24])] + [{"en": en, "de": de}]
    for entry in entries:
        if scale == "100":
            entry["en"] = entry["en"].replace("sound box through eight stations", "sound box through fifteen stations")
            entry["de"] = entry["de"].replace("Korpus durch acht Stationen", "Korpus durch fünfzehn Stationen")
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("[DEBUG] decisions written")

for scale in ("200", "500"):
    entries = document["volumes"][scale]["decisions"]
    entries[:] = [e for e in entries if not e["en"].startswith(("This scale retains separate lower cervical", "The fretting hand has separate knuckles"))]
    for entry in entries:
        entry["en"] = entry["en"].replace("Shirt of six sections without collar", "Shirt of seven sections without collar")
        entry["de"] = entry["de"].replace("Hemd aus sechs Schnitten ohne Kragen", "Hemd aus sieben Schnitten ohne Kragen")
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
