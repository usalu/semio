"""🚐 Second-round body rework of vehicles/delivery-van (source `volumes/vehicles/delivery-van.json`): deeper, sharper headlamp lenses, a high-mount stop lamp, lamps, grille and a front bumper at the coarse scales, rear brake discs at the smallest feature of each scale. Reference numbers: `rework/assets/vehicles--delivery-van.md`."""
import json

PATH = "volumes/vehicles/delivery-van.json"
document = json.load(open(PATH, encoding="utf8"))
V = document["volumes"]
ACROSS = [0.70711, 0, 0.70711, 0]
GLASS = {"material": "glass", "role": "detail", "tone": 0.35}
TRIM = {"material": "plastic-trim", "role": "body", "tone": 0.2}


def find(scale, name, part="body"):
    return next((solid for solid in V[scale][part] if solid["id"] == name), None)


def put(scale, solid, after, part="body"):
    rows = V[scale][part]
    old = find(scale, solid["id"], part)
    if old is not None:
        rows[rows.index(old)] = solid
        return
    rows.insert(rows.index(find(scale, after, part)) + 1, solid)


def drop(scale, name, part="body"):
    V[scale][part] = [solid for solid in V[scale][part] if solid["id"] != name]


def decide(scale, en, de):
    rows = [row for row in V[scale]["decisions"] if not row["en"].startswith(en[:40])]
    V[scale]["decisions"] = rows + [{"en": en, "de": de}]


def bumper(scale, stations):
    return {"id": "bumper-front", "kind": "loft", "bone": "chassis", **TRIM, "sections": [{"at": [x, y, 0], "rx": half, "ry": rise, "exponent": power, "rotation": ACROSS} for x, y, half, rise, power in stations], "capStart": True, "capEnd": True}


for scale in ("50", "100"):
    for side in ("l", "r"):
        for section in find(scale, f"headlamp-{side}")["sections"]:
            section["rx"], section["exponent"] = 0.03, 8

put("50", {"id": "stop-lamp-high", "kind": "box", "bone": "chassis", **GLASS, "size": [0.02, 0.03, 0.24], "transform": {"translation": [-2.648, 1.945, 0]}, "cornerRadius": 0.006}, "roof")
put("100", {"id": "stop-lamp-high", "kind": "box", "bone": "chassis", **GLASS, "size": [0.02, 0.03, 0.24], "transform": {"translation": [-2.648, 1.945, 0]}}, "roof")

DISC = {"100": (-0.0525, -0.0375), "200": (-0.06, -0.03), "500": (-0.075, 0)}
for scale, (inner, outer) in DISC.items():
    for wheel in ("rl", "rr"):
        find(scale, f"{wheel}-brake-disc")["profile"] = [[inner, 0.14], [outer, 0.14], [outer, 0.08], [inner, 0.08], [inner, 0.14]]

put("200", bumper("200", [(2.12, 0.429, 0.999, 0.138, 7.97), (2.3, 0.4135, 0.983, 0.154, 7), (2.45, 0.4115, 0.945, 0.156, 6.5), (2.56, 0.412, 0.885, 0.155, 6.13), (2.63, 0.43, 0.795, 0.1375, 6), (2.67, 0.49, 0.675, 0.0775, 6)]), "front-end")
put("200", {"id": "grille", "kind": "box", "bone": "chassis", "material": "engine-metal", "role": "detail", "tone": 0.15, "size": [0.04, 0.2, 0.9], "transform": {"translation": [2.646, 0.8, 0]}}, "bumper-front")
put("500", bumper("500", [(2.12, 0.429, 0.999, 0.138, 8), (2.4, 0.412, 0.96, 0.156, 8), (2.6, 0.42, 0.845, 0.145, 8), (2.67, 0.49, 0.675, 0.0775, 8)]), "front-end")
for side, sign in (("l", 1), ("r", -1)):
    put("200", {"id": f"tail-lamp-{side}", "kind": "box", "bone": "chassis", **GLASS, "size": [0.03, 0.42, 0.085], "transform": {"translation": [-2.627, 1.4, sign * 0.895]}}, f"corner-post-{side}")
    put("500", {"id": f"tail-lamp-{side}", "kind": "box", "bone": "chassis", **GLASS, "size": [0.075, 0.42, 0.1], "transform": {"translation": [-2.6045, 1.4, sign * 0.9]}}, "bumper-rear")
    lamp = dict(find("200", f"headlamp-{side}"))
    lamp["radii"] = [0.15, 0.085, 0.04]
    put("500", lamp, "front-end")

decide("50", "Second round: the headlamp lenses are 60 mm deep bodies with crisp edges that stand proud of the wing along their whole outline, and a high-mount stop lamp sits on the rear roof header above the doors.", "Zweite Runde: Die Scheinwerfergläser sind 60 mm tiefe Körper mit scharfen Kanten, die auf ihrem ganzen Umriss aus dem Kotflügel vorstehen, und eine dritte Bremsleuchte sitzt auf dem hinteren Dachquerträger über den Türen.")
decide("100", "Second round: headlamp lenses as at 1:50, the high-mount stop lamp is kept (240 mm wide) and the solid rear brake discs are 15 mm thick, the smallest feature of this scale.", "Zweite Runde: Scheinwerfergläser wie in 1:50, die dritte Bremsleuchte bleibt (240 mm breit) und die massiven hinteren Bremsscheiben sind 15 mm stark, das kleinste Merkmal dieses Maßstabs.")
decide("200", "Second round: the dark front bumper, the radiator grille and the tall tail lamps on the corner posts are modelled because each is far larger than 30 mm and they give the van its face and its back; the rear brake discs are 30 mm thick.", "Zweite Runde: Der dunkle vordere Stoßfänger, der Kühlergrill und die hohen Rückleuchten an den Eckpfosten sind modelliert, weil jedes Teil weit größer als 30 mm ist und sie dem Wagen Gesicht und Heck geben; die hinteren Bremsscheiben sind 30 mm stark.")
decide("500", "Second round: front bumper, headlamps and tail lamps stay as single solids of at least 75 mm, so the van still reads front from back at this scale; each rear brake is one 75 mm ring.", "Zweite Runde: Vorderer Stoßfänger, Scheinwerfer und Rückleuchten bleiben als einzelne Körper von mindestens 75 mm, damit der Wagen auch in diesem Maßstab vorn von hinten unterscheidbar ist; jede hintere Bremse ist ein 75-mm-Ring.")

json.dump(document, open(PATH, "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("written", {scale: len(V[scale]["body"]) for scale in V})
