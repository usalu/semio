"""Hand-authored volume of the sample figure (`🧫️fixtures/👤️sample-figure/🔣️.json`), inserted after `rig` (idempotent).

Bone-local rest frames (u along the bone, v its normal, w lateral, left +): pelvis, torso and head point up (v is backward), arm and leg bones point down (v is forward), a foot points forward (v is up).
Lateral positions come from the plan shapes: arms at w = +-0.225, legs and feet at w = +-0.09.
"""
import json
import sys

path = sys.argv[1]
document = json.loads(open(path, encoding="utf8").read())
asset = document["asset"]


def solid(id, kind, bone, material, role, tone, region, **fields):
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, "region": region, **fields}


def side(name, sign):
    w = round(0.09 * sign, 4)
    arm = round(0.225 * sign, 4)
    lo, hi = sorted((round(0.045 * sign, 4), round(0.135 * sign, 4)))
    return [
        solid(f"arm-{name}", "capsule", f"arm-{name}", "cotton-clothing", "body", 0.5, "arms", p0=[0, 0, arm], p1=[0.6, 0, arm], r0=0.04365, r1=0.03395),
        solid(f"thigh-{name}", "capsule", f"thigh-{name}", "cotton-clothing", "body", 0.3, "legs", p0=[0, 0, w], p1=[0.45, 0, w], r0=0.08245, r1=0.06305),
        solid(f"shank-{name}", "capsule", f"shank-{name}", "cotton-clothing", "body", 0.3, "legs", p0=[0, 0, w], p1=[0.43, 0, w], r0=0.0582, r1=0.0388),
        solid(
            f"shoe-{name}",
            "extrude",
            f"foot-{name}",
            "leather-shoe",
            "body",
            0.15,
            "feet",
            profile=[[-0.06, -0.065], [0.2, -0.065], [0.21, -0.055], [0.21, -0.045], [0.14, -0.02], [0.03, -0.01], [-0.04, -0.01], [-0.06, -0.03]],
            plane="xy",
            **{"from": lo, "to": hi},
        ),
        solid(f"sole-{name}", "box", f"foot-{name}", "leather-shoe", "detail", 0.08, "feet", size=[0.27, 0.015, 0.09], cornerRadius=0.005, transform={"translation": [0.075, -0.0725, w]}),
    ]


solids = [
    solid("head", "ellipsoid", "head", "skin", "body", 0.85, "head", radii=[0.115, 0.095, 0.085], transform={"translation": [0.1, 0, 0]}),
    solid(
        "hair",
        "revolve",
        "head",
        "hair",
        "body",
        0.2,
        "head",
        profile=[[0.1, 0.1007], [0.16095, 0.0872], [0.2056, 0.0504], [0.22005, 0.01749], [0.2219, 0]],
        axis=[1, 0, 0],
        origin=[0, 0, 0],
        lateralScale=0.895,
    ),
    solid("neck", "capsule", "head", "skin", "body", 0.85, "head", p0=[-0.06, 0, 0], p1=[0.06, 0, 0], r0=0.0485, r1=0.0485, weights=[{"bone": "head", "weight": 0.7}, {"bone": "torso", "weight": 0.3}]),
    solid("nose", "ellipsoid", "head", "skin", "detail", 0.85, "head", radii=[0.016, 0.01, 0.012], transform={"translation": [0.1, -0.092, 0]}, maxDenominator=50),
    solid(
        "torso",
        "loft",
        "torso",
        "cotton-clothing",
        "body",
        0.45,
        "torso",
        sections=[
            {"at": [0, 0, 0], "rx": 0.16, "ry": 0.1261, "exponent": 2.4},
            {"at": [0.22, 0, 0], "rx": 0.18, "ry": 0.14, "exponent": 2.4},
            {"at": [0.45, 0, 0], "rx": 0.19, "ry": 0.1552, "exponent": 2.2},
        ],
        capStart=True,
        capEnd=True,
    ),
    solid("buttons", "ellipsoid", "torso", "cotton-clothing", "detail", 0.7, "torso", radii=[0.008, 0.004, 0.008], transform={"translation": [0.15, -0.138, 0]}, repeat={"count": 3, "translation": [0.08, 0, 0]}, maxDenominator=50),
    solid("pelvis", "ellipsoid", "pelvis", "cotton-clothing", "body", 0.3, "torso", radii=[0.12, 0.1358, 0.17], transform={"translation": [-0.02, 0, 0]}),
    *side("l", 1),
    *side("r", -1),
]

volume = {
    "status": "authored",
    "solids": solids,
    "decisions": [
        {
            "en": "Every solid is read off the authored side, front and plan shapes of the test figure: the capsules and the ellipsoids repeat the elevation radii, the lateral positions come from the plan, so the three silhouettes of the volume agree with the three drawings.",
            "de": "Jeder Körper ist den gezeichneten Seiten-, Vorder- und Grundrissformen der Testfigur entnommen: die Kapseln und Ellipsoide wiederholen die Radien der Seitenansicht, die seitlichen Lagen stammen aus dem Grundriss, sodass die drei Silhouetten des Volumens mit den drei Zeichnungen übereinstimmen.",
        },
        {
            "en": "The torso is a loft of three superelliptic sections (hip, waist, shoulder), the hair a revolve about the head axis, the shoe an extrusion of its side profile and the sole a rounded box; the neck is blended 70 % to the head and 30 % to the torso.",
            "de": "Der Rumpf ist ein Loft aus drei superelliptischen Schnitten (Hüfte, Taille, Schulter), das Haar ein Rotationskörper um die Kopfachse, der Schuh eine Extrusion seines Seitenprofils und die Sohle ein abgerundeter Quader; der Hals ist zu 70 % an den Kopf und zu 30 % an den Rumpf gebunden.",
        },
        {
            "en": "Nose and the three coat buttons exist only at 1:50 (maxDenominator 50); the buttons are one solid repeated three times along the torso.",
            "de": "Nase und die drei Mantelknöpfe existieren nur im Maßstab 1:50 (maxDenominator 50); die Knöpfe sind ein Körper, der dreimal entlang des Rumpfes wiederholt wird.",
        },
    ],
}

out = {}
for key, value in asset.items():
    if key == "volume":
        continue
    out[key] = value
    if key == "rig":
        out["volume"] = volume
document["asset"] = out
open(path, "w", encoding="utf8").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
