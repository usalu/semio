"""🛴 Hand-chosen solids of `vehicles/electric-kick-scooter` (second round, pass 1): 1:50 joins and per-scale wheels, supports and joints.

Run from the ticket folder: `python3 rework-electric-kick-scooter-volumes.py`; it edits `volumes/vehicles/electric-kick-scooter.json` in place and is idempotent.
"""
import json
from pathlib import Path

SOURCE = Path(__file__).parent / "volumes/vehicles/electric-kick-scooter.json"
document = json.loads(SOURCE.read_text(encoding="utf-8"))
volumes = document["volumes"]
Z = [0, 0, 1]


def turned(identifier, bone, material, profile, origin_z, role, tone, **extra):
    """🌀 A part turned about the wheel axle (bone w)."""
    return {"id": identifier, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile, "axis": Z, "origin": [0, 0, origin_z], **extra}


def ring(points):
    """⭕ A closed profile loop (a ring, not closed through the axis)."""
    return points + [points[0]]


def put(entries, solid, after=None):
    """📌 Replace the solid of the same id or insert it after `after` (else append)."""
    for index, entry in enumerate(entries):
        if entry["id"] == solid["id"]:
            entries[index] = solid
            return
    if after is not None:
        for index, entry in enumerate(entries):
            if entry["id"] == after:
                entries.insert(index + 1, solid)
                return
    entries.append(solid)


def drop(entries, identifiers):
    """🧹 Remove the named solids."""
    entries[:] = [entry for entry in entries if entry["id"] not in identifiers]


def drop_bones(entries, body, bones):
    """🧹 Remove every solid that sits on one of the bones (directly or through `of`)."""
    by_id = {entry["id"]: entry for entry in body}
    entries[:] = [entry for entry in entries if (entry.get("bone") or by_id.get(entry.get("of"), {}).get("bone")) not in bones]


WHEELS = ("wheel-r", "wheel-f")
ROTOR = ring([[0, 0.035], [0.003, 0.035], [0.003, 0.055], [0, 0.055]])
ROTOR_FLANGE = ring([[0, 0.016], [0.0075, 0.016], [0.0075, 0.038], [0, 0.038]])

fine = volumes["50"]
put(fine["body"], turned("disc-r", "wheel-r", "steel-disc", ROTOR, 0.031, "texture", 0.7))
put(fine["body"], turned("disc-flange-r", "wheel-r", "anodised", ROTOR_FLANGE, 0.0235, "detail", 0.55), after="disc-r")
put(fine["thermal"], {"id": "th-disc-flange-r", "of": "disc-flange-r", "region": "rear-rim"}, after="th-disc-r")
put(fine["body"], {"id": "stand-pivot", "kind": "box", "bone": "deck", "material": "steel-disc", "role": "detail", "tone": 0.4, "size": [0.026, 0.014, 0.026], "cornerRadius": 0.003, "transform": {"translation": [0.105, 0.1, 0.075]}}, after="side-stand")
put(fine["thermal"], {"id": "th-stand-pivot", "of": "stand-pivot", "region": "deck-battery"}, after="th-side-stand")
drop(fine["structure"], {"st-disc-r", "support-r", "support-f"})
put(fine["structure"], {"id": "hinge-base", "of": "hinge-base", "member": "fork", "part": "member"}, after="hinge-pin")

for scale in ("100", "200", "500"):
    drop(volumes[scale]["structure"], {"support-r", "support-f"})

medium = volumes["100"]
for entry in medium["structure"]:
    if entry["id"] == "deck-girder":
        entry["from"] = -0.305
TYRE_100 = ring([[-0.019, 0.068], [-0.025, 0.082], [-0.022, 0.1], [-0.01, 0.108], [0.01, 0.108], [0.022, 0.1], [0.025, 0.082], [0.019, 0.068]])
DISC_WHEEL_100 = ring([[-0.0225, 0.058], [-0.0225, 0.077], [0.0225, 0.077], [0.0225, 0.058], [0.008, 0.05], [0.008, 0.019], [-0.008, 0.019], [-0.008, 0.05]])
MOTOR_100 = [[-0.033, 0], [-0.033, 0.058], [-0.0225, 0.068], [-0.0225, 0.077], [0.0225, 0.077], [0.0225, 0.068], [0.033, 0.058], [0.033, 0]]
for name in ("thermal", "structure", "body"):
    drop_bones(medium[name], medium["body"], WHEELS)
medium["body"] += [
    turned("tyre-r", "wheel-r", "rubber", TYRE_100, 0, "body", 0.07),
    turned("rim-r", "wheel-r", "anodised", DISC_WHEEL_100, 0, "body", 0.6),
    turned("hub-r", "wheel-r", "anodised", [[0, 0.02], [0.056, 0.02], [0.056, 0.038], [0.071, 0.038], [0.071, 0.02], [0.08, 0.02]], -0.04, "body", 0.55),
    turned("axle-r", "wheel-r", "steel-disc", [[0, 0.009], [0.115, 0.009]], -0.0575, "detail", 0.4),
    turned("disc-r", "wheel-r", "steel-disc", ROTOR, 0.031, "texture", 0.7),
    turned("tyre-f", "wheel-f", "rubber", TYRE_100, 0, "body", 0.07),
    turned("motor-f", "wheel-f", "motor-alloy", MOTOR_100, 0, "body", 0.5),
    turned("axle-f", "wheel-f", "steel-disc", [[0, 0.009], [0.104, 0.009]], -0.052, "detail", 0.4),
]
medium["thermal"] += [
    {"id": "th-tyre-r", "of": "tyre-r", "region": "tyres"},
    {"id": "th-rim-r", "of": "rim-r", "region": "rear-rim"},
    {"id": "th-hub-r", "of": "hub-r", "region": "rear-rim"},
    {"id": "th-axle-r", "of": "axle-r", "region": "rear-rim"},
    {"id": "th-disc-r", "of": "disc-r", "region": "brake-disc"},
    {"id": "th-tyre-f", "of": "tyre-f", "region": "tyres"},
    {"id": "th-motor-f", "of": "motor-f", "region": "hub-motor"},
    {"id": "th-axle-f", "of": "axle-f", "region": "hub-motor"},
]
medium["structure"] += [
    {"id": "st-tyre-r", "of": "tyre-r", "member": "wheel-r", "part": "member"},
    {"id": "rim-r", "of": "rim-r", "member": "wheel-r", "part": "member"},
    {"id": "hub-r", "of": "hub-r", "member": "wheel-r", "part": "member"},
    {"id": "axle-r", "of": "axle-r", "member": "wheel-r", "part": "joint"},
    {"id": "st-tyre-f", "of": "tyre-f", "member": "wheel-f", "part": "member"},
    {"id": "motor-f", "of": "motor-f", "member": "wheel-f", "part": "member"},
    {"id": "axle-f", "of": "axle-f", "member": "wheel-f", "part": "joint"},
    turned("mass-motor", "wheel-f", "steel-disc", [[0, 0.05], [0.04, 0.05]], -0.02, "detail", 0.2, member="wheel-f", part="mass"),
]

coarse = volumes["200"]
TYRE_200 = ring([[-0.02, 0.07], [-0.025, 0.09], [-0.014, 0.108], [0.014, 0.108], [0.025, 0.09], [0.02, 0.07]])
WHEEL_200 = [[-0.04, 0], [-0.04, 0.02], [-0.0225, 0.02], [-0.0225, 0.077], [0.0225, 0.077], [0.0225, 0.038], [0.031, 0.038], [0.031, 0.02], [0.04, 0.02], [0.04, 0]]
MOTOR_200 = [[-0.047, 0], [-0.047, 0.015], [-0.033, 0.015], [-0.033, 0.058], [-0.0225, 0.077], [0.0225, 0.077], [0.033, 0.058], [0.033, 0.015], [0.047, 0.015], [0.047, 0]]
for name in ("thermal", "structure", "body"):
    drop_bones(coarse[name], coarse["body"], WHEELS)
coarse["body"] += [
    turned("tyre-r", "wheel-r", "rubber", TYRE_200, 0, "body", 0.07),
    turned("rim-r", "wheel-r", "anodised", WHEEL_200, 0, "body", 0.6),
    turned("tyre-f", "wheel-f", "rubber", TYRE_200, 0, "body", 0.07),
    turned("motor-f", "wheel-f", "motor-alloy", MOTOR_200, 0, "body", 0.5),
]
coarse["thermal"] += [
    {"id": "th-tyre-r", "of": "tyre-r", "region": "tyres"},
    {"id": "th-rim-r", "of": "rim-r", "region": "rear-rim"},
    turned("th-disc-r", "wheel-r", "steel-disc", ROTOR, 0.031, "texture", 0.7, region="brake-disc"),
    {"id": "th-tyre-f", "of": "tyre-f", "region": "tyres"},
    {"id": "th-motor-f", "of": "motor-f", "region": "hub-motor"},
]
coarse["structure"] += [
    {"id": "st-tyre-r", "of": "tyre-r", "member": "wheel-r", "part": "member"},
    {"id": "rim-r", "of": "rim-r", "member": "wheel-r", "part": "member"},
    turned("axle-r", "wheel-r", "steel-disc", [[0, 0.015], [0.12, 0.015]], -0.06, "detail", 0.3, member="wheel-r", part="joint"),
    {"id": "st-tyre-f", "of": "tyre-f", "member": "wheel-f", "part": "member"},
    {"id": "motor-f", "of": "motor-f", "member": "wheel-f", "part": "member"},
    turned("axle-f", "wheel-f", "steel-disc", [[0, 0.015], [0.094, 0.015]], -0.047, "detail", 0.3, member="wheel-f", part="joint"),
    turned("mass-motor", "wheel-f", "steel-disc", [[0, 0.05], [0.04, 0.05]], -0.02, "detail", 0.2, member="wheel-f", part="mass"),
]

block = volumes["500"]
for entry in block["body"]:
    if entry["id"] == "deck":
        entry["profile"] = [[-0.075, 0.266], [0.075, 0.266], [0.075, -0.3], [0.06, -0.452], [0.03, -0.452], [0.03, -0.316], [-0.03, -0.316], [-0.03, -0.452], [-0.06, -0.452], [-0.075, -0.3]]
TYRE_500 = ring([[-0.025, 0.07], [-0.025, 0.095], [-0.012, 0.108], [0.012, 0.108], [0.025, 0.095], [0.025, 0.07]])
WHEEL_500 = [[-0.04, 0], [-0.04, 0.02], [-0.0225, 0.03], [-0.0225, 0.077], [0.0225, 0.077], [0.0225, 0.04], [0.031, 0.036], [0.04, 0.02], [0.04, 0]]
MOTOR_500 = [[-0.033, 0], [-0.033, 0.06], [-0.0225, 0.077], [0.0225, 0.077], [0.033, 0.06], [0.033, 0]]
for name in ("thermal", "structure", "body"):
    drop_bones(block[name], block["body"], WHEELS)
block["body"] += [
    turned("tyre-r", "wheel-r", "rubber", TYRE_500, 0, "body", 0.07),
    turned("rim-r", "wheel-r", "anodised", WHEEL_500, 0, "body", 0.6),
    turned("tyre-f", "wheel-f", "rubber", TYRE_500, 0, "body", 0.07),
    turned("motor-f", "wheel-f", "motor-alloy", MOTOR_500, 0, "body", 0.5),
]
block["thermal"] += [
    {"id": "th-tyre-r", "of": "tyre-r", "region": "tyres"},
    {"id": "th-rim-r", "of": "rim-r", "region": "rear-rim"},
    turned("th-disc-r", "wheel-r", "steel-disc", ROTOR, 0.031, "texture", 0.7, region="brake-disc"),
    {"id": "th-tyre-f", "of": "tyre-f", "region": "tyres"},
    {"id": "th-motor-f", "of": "motor-f", "region": "hub-motor"},
]
block["structure"] += [
    {"id": "st-tyre-r", "of": "tyre-r", "member": "wheel-r", "part": "member"},
    {"id": "rim-r", "of": "rim-r", "member": "wheel-r", "part": "member"},
    {"id": "st-tyre-f", "of": "tyre-f", "member": "wheel-f", "part": "member"},
    {"id": "motor-f", "of": "motor-f", "member": "wheel-f", "part": "member"},
    turned("mass-motor", "wheel-f", "steel-disc", [[0, 0.05], [0.04, 0.05]], -0.02, "detail", 0.2, member="wheel-f", part="mass"),
]

WHEEL_DECISIONS = {
    "50": {
        "en": "Joins and bearings (rework 2026-11-22): the brake rotor reaches in to the hub flange it is bolted to, the side stand hangs on a pivot block under the left deck edge, the hinge base is a member of the fork so that the stem meets the steerer through the hinge pin; the rotor is no member of the structure, which ends in the two tyre contact patches.",
        "de": "Anschlüsse und Lager (Überarbeitung 2026-11-22): Die Bremsscheibe reicht bis an den Nabenflansch, an dem sie verschraubt ist, der Seitenständer hängt an einem Lagerbock unter der linken Trittbrettkante, der Gelenksockel ist ein Stab der Gabel, sodass die Lenkstange über den Gelenkbolzen am Gabelschaft anschliesst; die Bremsscheibe ist kein Stab des Tragwerks, das in den beiden Reifenaufstandsflächen endet.",
    },
    "100": {
        "en": "Wheels at 1:100, modelled for 15 mm: the tyre is a turned ring of eight profile points, the cast rear wheel is one dished disc wheel (rim band and a 16 mm web; the five spoke windows are below what this scale shows and agree with the disc of the drawing), the hub is a plain barrel, the axles are drawn nut to nut (18 mm) and the brake rotor stays as the declared thin plate on the hub flange; the hub motor is one turned shell with its rim. The bearings are left out.",
        "de": "Räder im Massstab 1:100, für 15 mm modelliert: Der Reifen ist ein gedrehter Ring aus acht Profilpunkten, das gegossene Hinterrad ein geschüsseltes Scheibenrad (Felgenband und 16 mm Steg; die fünf Speichenfenster liegen unter dem, was dieser Massstab zeigt, und stimmen mit der Scheibe der Zeichnung überein), die Nabe ist eine einfache Trommel, die Achsen sind von Mutter zu Mutter gezeichnet (18 mm) und die Bremsscheibe bleibt als ausgewiesene dünne Platte am Nabenflansch; der Nabenmotor ist eine gedrehte Schale mit ihrer Felge. Die Lager entfallen.",
    },
    "200": {
        "en": "Wheels at 1:200, modelled for 30 mm: each wheel is two turned parts, the tyre ring of six profile points and one wheel body (rear: rim, web and hub barrel reaching into both deck arms; front: the motor shell with its axle bosses reaching into both fork legs). Spokes, bearings, axles and the rotor are below 30 mm and are not in the body; the rotor remains in the thermal list as its own region and the axles in the structure list as the two wheel joints.",
        "de": "Räder im Massstab 1:200, für 30 mm modelliert: Jedes Rad besteht aus zwei Drehteilen, dem Reifenring aus sechs Profilpunkten und einem Radkörper (hinten: Felge, Steg und Nabentrommel, die in beide Trittbrettarme greift; vorn: die Motorschale mit ihren Achsnaben, die in beide Gabelschenkel greifen). Speichen, Lager, Achsen und Bremsscheibe liegen unter 30 mm und fehlen im Körper; die Bremsscheibe bleibt als eigene Region in der thermischen Liste und die Achsen als die beiden Radgelenke in der Tragwerksliste.",
    },
    "500": {
        "en": "Wheels at 1:500, modelled as masses: a tyre ring with a flat tread and one wheel body per wheel (the rear with a conical hub boss into the deck arms, the front the motor drum); nothing smaller is modelled in the body. The brake rotor is kept only in the thermal list, because it is a heat source of its own.",
        "de": "Räder im Massstab 1:500, als Massen modelliert: je Rad ein Reifenring mit flacher Lauffläche und ein Radkörper (hinten mit kegeliger Nabe in die Trittbrettarme, vorn die Motortrommel); Kleineres wird im Körper nicht modelliert. Die Bremsscheibe bleibt nur in der thermischen Liste, weil sie eine eigene Wärmequelle ist.",
    },
}
for scale, text in WHEEL_DECISIONS.items():
    decisions = volumes[scale]["decisions"]
    decisions[:] = [entry for entry in decisions if "rework 2026-11-22" not in entry["en"] and not entry["en"].startswith("Wheels at 1:")]
    if scale != "50":
        decisions[:] = [entry for entry in decisions if "cartridges in shouldered bearing seats" not in entry["en"]]
    decisions.append(text)

SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] written", {scale: {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")} for scale in volumes})
