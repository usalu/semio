"""🛵 Round 3 root fixes of vehicles/motor-scooter outside `volumes`: the rig becomes the real scooter (steering axis raked 25 degrees with 25 mm axle offset, engine pivot under the cylinder, seat hinge at the seat nose, centre stand that swings over centre), with the anchors, poses, masses, texts, the rim thermal region, the thermal areas the honest model shows, the symbol and the three drawings of every authored scale with their decisions.

Usage: python3 r3-motor-scooter-roots.py [--no-2d] [--apply]
Writes the candidate document to 🗑️generated/r3-motor-scooter/🔣️.json (with the volumes of the source); `--apply` re-reads the catalog file and writes it in the file's own indent-1 style (the `volumes` member keeps its text and is spliced by volumes-insert.py)."""
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/motor-scooter/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "r3-motor-scooter")


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-motor-scooter-kit")
AF, AR, H, PIVOT, HINGE = kit.AF, kit.AR, kit.H, kit.PIVOT, kit.HINGE


def r4(x):
    return round(x, 4) + 0.0


def en_de(en, de):
    return {"en": en, "de": de}


def rot(p, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (p[0] * c - p[1] * s, p[0] * s + p[1] * c)


STAND_FOOT = (kit.STAND_LENGTH, 0.0)


def parked_angle():
    """🅿️ The root rotation that sets the front tyre and the lowered stand foot on one level ground."""
    a = math.radians(kit.STAND_REST + kit.STAND_DOWN)
    foot = (kit.STAND_PIVOT[0] + STAND_FOOT[0] * math.cos(a), kit.STAND_PIVOT[1] + STAND_FOOT[0] * math.sin(a))
    lo, hi = -10.0, 10.0
    for _ in range(60):
        mid = (lo + hi) / 2
        gap = (rot(AF, mid)[1] - kit.TYRE_R) - rot(foot, mid)[1]
        lo, hi = (lo, mid) if gap > 0 else (mid, hi)
    return round((lo + hi) / 2, 3)


BRAKE_PITCH = -1.5


def braking_swing():
    """🛑 The swing angle that keeps the rear tyre on the ground when the body pitches about the front contact."""
    front = rot(AF, BRAKE_PITCH)[1]
    pivot = rot(PIVOT, BRAKE_PITCH)
    arm = (AR[0] - PIVOT[0], AR[1] - PIVOT[1])
    lo, hi = -10.0, 10.0
    for _ in range(60):
        mid = (lo + hi) / 2
        y = pivot[1] + rot(arm, BRAKE_PITCH + mid)[1]
        lo, hi = (mid, hi) if y > front else (lo, mid)
    return round((lo + hi) / 2, 2)


PARKED = parked_angle()
SWING = braking_swing()
CONTACT_F = kit.loc("steer", (AF[0], 0.0))

BONES = {
    "frame": dict(mass_com=(-0.25, 0.5)),
    "steer": dict(x=r4(H[0]), y=r4(H[1]), angleDeg=-90.0 + kit.RAKE, lengthM=r4(kit.STEER_LENGTH), mass_com=(0.42, 0.0), title=en_de("Headset, steering column, fork leg, link, shock and mudguard", "Lenkkopf, Lenksäule, Gabelrohr, Schwinge, Federbein und Schutzblech")),
    "wheel-f": dict(x=r4(kit.STEER_LENGTH), y=r4(kit.OFFSET), angleDeg=90.0 - kit.RAKE),
    "swing": dict(x=PIVOT[0], y=PIVOT[1], lengthM=r4(math.hypot(AR[0] - PIVOT[0], AR[1] - PIVOT[1])), mass_com=(-0.2, 0.053), title=en_de("Engine unit with belt case, right arm and silencer", "Triebsatz mit Riemengehäuse, rechtem Arm und Schalldämpfer")),
    "wheel-r": dict(x=r4(AR[0] - PIVOT[0]), y=r4(AR[1] - PIVOT[1])),
    "seat": dict(x=HINGE[0], y=HINGE[1], lengthM=kit.SEAT_LENGTH, mass_com=(0.34, -0.04)),
    "stand": dict(x=kit.STAND_PIVOT[0], y=kit.STAND_PIVOT[1], angleDeg=kit.STAND_REST, lengthM=kit.STAND_LENGTH, mass_com=(0.12, 0.0), joint={"type": "revolute", "minDeg": 0, "maxDeg": 95}),
}

ANCHORS = {
    "contact-f": (r4(CONTACT_F[0]), r4(CONTACT_F[1]), None),
    "contact-r": (r4(AR[0] - PIVOT[0]), r4(-PIVOT[1]), None),
    "stand-foot": (r4(STAND_FOOT[0]), 0.0, None),
    "tailpipe": (r4(-0.914 - PIVOT[0]), r4(0.262 - PIVOT[1]), -0.15),
    "seat-storage": (-0.45, 0.62, None),
    "rider-seat": (0.3, -0.068, None),
}

AREAS = {"body-panels":3.25,"steer-assembly":.4,"headlamp":.069,"engine":.46,"exhaust-system":.26,"tyres":.79,"brake-disc":.129,"rims":.6}

RIM_REGION = {
    "id": "rims",
    "title": en_de("Cast wheel rims, spokes and hubs", "Gussfelgen, Speichen und Naben"),
    "kind": "surface",
    "model": "passive",
    "material": "alloy",
    "areaM2": 0.5,
    "characteristicLengthM": 0.3,
    "geometry": "cylinder",
    "skyViewFactor": 0.3,
    "layers": [],
    "heatCapacityJM2K": 9000,
    "source": "authored-estimate",
    "reference": "ashrae-2021",
}

REFERENCE = {"id": "piaggio-primavera-125", "citation": "Piaggio & C. SpA. Vespa Primavera 125 technical data as listed by 1000PS and UltimateSpecs (model years 2018 to 2026): length 1852 to 1870 mm, width 735 mm, height 1150 mm, wheelbase 1340 mm, seat height 790 mm, kerb mass 126 to 132 kg, tyres 110/70-12 front and 120/70-12 rear, front disc 200 mm, rear drum 140 mm, steel monocoque, single-sided trailing-link front suspension, swinging engine unit with CVT."}

NOTES = en_de(
    "Typical 125 cc four-stroke step-through scooter with a steel monocoque of the Vespa Primavera class (1.85 x 0.74 x 1.15 m to the mirrors, wheelbase 1.34 m, seat 0.79 m, kerb mass 130 kg, 12 inch wheels): single-sided fork leg with trailing link on the right, engine unit swinging about a pivot under the cylinder with the belt case on the left and the silencer on the right, steering axis raked 25 degrees with 80 mm trail. Dimensions follow the manufacturer's figures; rake, trail and heat data are authored class values (authored-estimate). Both wheels roll with theta = -x / r (r 0.23 m, the 110/70-12 radius; the 6 mm larger 120/70-12 rear radius is not modelled so that the ride loop closes). The rider is excluded.",
    "Typischer 125-cm3-Viertakt-Roller mit Durchstieg und Stahl-Monocoque der Klasse Vespa Primavera (1,85 x 0,74 x 1,15 m bis zu den Spiegeln, Radstand 1,34 m, Sitzhöhe 0,79 m, Leermasse 130 kg, 12-Zoll-Räder): einseitiges Gabelrohr mit gezogener Schwinge rechts, Triebsatzschwinge um ein Lager unter dem Zylinder mit Riemengehäuse links und Schalldämpfer rechts, Lenkachse um 25 Grad geneigt mit 80 mm Nachlauf. Die Abmessungen folgen den Herstellerangaben; Lenkkopfwinkel, Nachlauf und Wärmedaten sind selbst gesetzte Klassenwerte (authored-estimate). Beide Räder rollen mit theta = -x / r (r 0,23 m, der Halbmesser von 110/70-12; der um 6 mm größere Halbmesser des hinteren 120/70-12 ist nicht modelliert, damit die Fahrschleife schließt). Der Fahrer ist ausgeschlossen.",
)

ASSUMPTIONS = [
    en_de(
        "Rigid bodies on two unilateral ground contacts or on the front tyre and the centre stand; the engine unit is a small rotation about its pivot under the cylinder, and the front trailing link is rigid (braking dive is a pitch about the front contact with the engine unit swinging down).",
        "Starre Körper auf zwei einseitigen Bodenkontakten oder auf Vorderreifen und Hauptständer; der Triebsatz ist eine kleine Drehung um sein Lager unter dem Zylinder, die vordere gezogene Schwinge ist starr (das Bremsnicken ist eine Drehung um den vorderen Kontakt, der Triebsatz schwingt nach unten).",
    ),
    en_de(
        "Steering is a rotation of headset, column, fork leg, mudguard and front wheel about the steering axis, which is raked 25 degrees; lateral balance is not modelled in the elevation.",
        "Lenken ist eine Drehung von Lenkkopf, Lenksäule, Gabelrohr, Schutzblech und Vorderrad um die um 25 Grad geneigte Lenkachse; das seitliche Gleichgewicht wird in der Ansicht nicht modelliert.",
    ),
]

BRAKING = en_de(
    "Hard braking attitude: body pitched 1.5 degrees nose down about the front contact, the engine unit swung down so that the rear tyre stays on the ground, front disc and rear drum hot.",
    "Haltung bei kräftiger Bremsung: Aufbau um den vorderen Kontakt 1,5 Grad nach vorn geneigt, der Triebsatz nach unten geschwungen, sodass der Hinterreifen am Boden bleibt, vordere Scheibe und hintere Trommel heiß.",
)


def symbol(sym):
    """🔣 The true-extent symbol follows the corrected body."""
    g = sym["geometry"]
    for s in g["elevation"]:
        if s["id"] == "elevation-body":
            s["points"] = [[-0.95, 0.62], [-0.9, 0.7], [-0.15, 0.72], [-0.07, 0.56], [-0.02, 0.3], [0.37, 0.3], [0.45, 0.55], [0.31, 0.94], [0.25, 0.94], [0.39, 0.5], [0.34, 0.18], [-0.16, 0.18], [-0.3, 0.39], [-0.8, 0.51]]
        if s["id"] == "elevation-seat":
            s.update(x=-0.815, y=0.715, w=0.68, h=0.11)
        if s["id"] == "elevation-fork":
            s["points"] = [[0.67, 0.23], [r4(kit.FOOT[0]), r4(kit.FOOT[1])], [r4(H[0]), r4(H[1])]]
        if s["id"] == "elevation-cowl":
            s["points"] = [[0.2, 0.95], [0.41, 0.95], [0.4, 1.07], [0.3, 1.15], [0.21, 1.07]]
    for s in g["front"]:
        if s["id"] == "front-wheel":
            s.update(x=-0.055, w=0.11)
        if s["id"] == "front-body":
            s.update(x=-0.235, y=0.18, w=0.47, h=0.76)
        if s["id"] == "front-fork":
            s.update(x=-0.1, y=0.2, w=0.2, h=0.07)
        if s["id"] == "front-cowl-bar":
            s.update(x=-0.37, y=0.95, w=0.74, h=0.2)
    for s in g["plan"]:
        if s["id"] == "plan-wheel-r":
            s.update(x=-0.9, y=-0.06, w=0.46, h=0.12)
        if s["id"] == "plan-wheel-f":
            s.update(x=0.44, y=-0.055, w=0.46, h=0.11)
        if s["id"] == "plan-body":
            s["points"] = [[-0.95, -0.085], [-0.8, -0.19], [-0.52, -0.233], [-0.2, -0.2], [0.3, -0.21], [0.41, -0.235], [0.5, -0.06], [0.5, 0.06], [0.41, 0.235], [0.3, 0.21], [-0.2, 0.2], [-0.52, 0.233], [-0.8, 0.19], [-0.95, 0.085]]
        if s["id"] == "plan-seat":
            s.update(x=-0.815, y=-0.17, w=0.68, h=0.34)
        if s["id"] == "plan-handlebar":
            s.update(x=0.255, y=-0.37, w=0.065, h=0.74)


def roots(asset, drawings):
    order = list(asset.keys())
    prov = asset["provenance"]
    if REFERENCE["id"] not in [r["id"] for r in prov["references"]]:
        prov["references"].insert(0, REFERENCE)
    prov["notes"] = NOTES
    asset["summary"] = en_de(
        "A riderless 125 cc four-stroke step-through scooter with a steel monocoque (130 kg, 12 inch wheels, mirrors) with hinged seat, centre stand, exhaust jet, hot engine, silencer, tyres and brakes.",
        "Ein fahrerloser Motorroller mit 125-cm3-Viertaktmotor, Durchstieg und Stahl-Monocoque (130 kg, 12-Zoll-Räder, Spiegel) mit klappbarer Sitzbank, Hauptständer, Abgasstrahl, heißem Motor, Schalldämpfer, Reifen und Bremsen.",
    )
    for bone in asset["rig"]["bones"]:
        for k, v in BONES.get(bone["id"], {}).items():
            if k == "mass_com":
                bone["mass"]["comX"], bone["mass"]["comY"] = v
            else:
                bone[k] = v
    for a in asset["anchors"]:
        if a["id"] in ANCHORS:
            x, y, z = ANCHORS[a["id"]]
            a["x"], a["y"] = x, y
            if z is not None:
                a["z"] = z
    for pose in asset["poses"]:
        if "stand" in pose["bones"]:
            pose["bones"]["stand"] = {"angleDeg": kit.STAND_DOWN}
            pose["root"]["angleDeg"] = PARKED
        if pose["id"] == "braking-nose-down":
            pose["bones"]["swing"] = {"angleDeg": SWING}
            pose["root"]["angleDeg"] = BRAKE_PITCH
            pose["description"] = BRAKING
        regions = pose["thermal"]["regions"]
        if "rims" not in regions:
            regions["rims"] = {"exposedFraction": 1, "sunlitFraction": regions["tyres"]["sunlitFraction"]}
    th = asset["thermal"]
    if "rims" not in [r["id"] for r in th["regions"]]:
        th["regions"].append(json.loads(json.dumps(RIM_REGION)))
    for region in th["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "steer-assembly":
            region["title"] = en_de("Headset, steering column, fork leg and front suspension", "Lenkkopf, Lenksäule, Gabelrohr und Vorderradaufhängung")
    asset["structure"]["assumptions"] = ASSUMPTIONS
    for sym in asset["symbols"]:
        symbol(sym)
    if drawings is not None:
        for key, d in drawings.items():
            rep = asset["representations"][key]
            if not all(d["geometry"][v] for v in ("elevation", "front", "plan")):
                continue
            rep["geometry"] = d["geometry"]
            if d.get("decisions") is not None:
                rep["decisions"] = d["decisions"]
            rep["pens"] = d["pens"]
    return {k: asset[k] for k in order}


if __name__ == "__main__":
    args = sys.argv[1:]
    doc = json.load(open(ASSET, encoding="utf8"))
    drawings = None
    path2d = os.path.join(OUT, "drawings.json")
    if "--no-2d" not in args and os.path.exists(path2d):
        drawings = json.load(open(path2d, encoding="utf8"))
    doc["asset"] = roots(doc["asset"], drawings)
    src = os.path.join(T, "volumes", "vehicles", "motor-scooter.json")
    cand = json.loads(json.dumps(doc))
    cand["asset"]["volumes"] = json.load(open(src, encoding="utf8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(cand, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print("candidate", os.path.join(OUT, "🔣️.json"), "parked", PARKED, "swing", SWING)
    if "--apply" in args:
        text = open(ASSET, encoding="utf8").read()
        now = json.loads(text)
        now["asset"] = roots(now["asset"], drawings)
        start = text.index('\n  "volumes": {') + 1
        end = text.index('\n  "presentation": {') + 1
        block = text[start:end]
        marker = "@@VOLUMES@@"
        vol = now["asset"]["volumes"]
        now["asset"]["volumes"] = marker
        out = json.dumps(now, ensure_ascii=False, indent=1) + "\n"
        out = out.replace('  "volumes": "' + marker + '",\n', block)
        assert json.loads(out)["asset"]["volumes"] == vol
        with open(ASSET, "w", encoding="utf8") as fh:
            fh.write(out)
        print("applied", ASSET)
