"""🛞 Charcoal grill, second round: the wheel pair as a bone on its axle with spoke ribs, a tilted wheeling pose, lid straps that end on the dome instead of inside it, and grates and briquettes typified per scale.

Usage: python3 rework-charcoal-grill-wheels-grate.py [--apply]
Without flags it writes the candidate `🗑️generated/rework-charcoal-grill/🔣️.json`; `--apply` also writes the source and the changed catalog root members.
"""
import copy
import math
import os
import shutil
import sys

from rework_15_kit import Kit, SCALES, T, text

AXLE = (-0.14278, 0.076)
WHEEL_Z = {"50": 0.282, "100": 0.282, "200": 0.282, "500": 0.2955}
WHEEL_R = 0.076
TILT = 20.0
DOME_R = (0.2925 ** 2 + 0.242 ** 2) / (2 * 0.242)
CROWN = (0.2925, 0.242)
GRATE_KEPT = {"50": range(1, 28), "100": range(1, 28), "200": range(2, 27, 4), "500": (4, 14, 24)}
FUEL_KEPT = {"50": range(1, 10), "100": range(1, 10), "200": (1, 3, 5, 7, 9), "500": (1, 5, 9)}
BRIQUETTES_KEPT = {"50": range(1, 15), "100": range(1, 15), "200": range(1, 15, 2), "500": ()}
SHEETS = {"50": (), "100": ("damper-plate",), "200": ("damper-plate",), "500": ("damper-plate", "vent-dial")}
RIB_COUNT, RIB_RADIUS, RIB_SIZE, RIB_Z = 5, 0.039, [0.03, 0.009, 0.008], 0.2965


def dome_y(dx, z):
    return CROWN[1] - (DOME_R - math.sqrt(DOME_R ** 2 - dx ** 2 - z ** 2))


def shift(point):
    return [point[0] - AXLE[0], point[1] - AXLE[1], point[2]]


def rig(kit):
    bowl = kit.bone("bowl")
    bowl["mass"]["kg"], bowl["mass"]["comX"], bowl["mass"]["comY"] = 9.4, -0.009, 0.511
    kit.add_bone("ash-damper", {
        "id": "wheels", "parent": "bowl",
        "title": text("Wheel pair on the fixed axle of the rear legs", "Radpaar auf der festen Achse der hinteren Beine"),
        "x": AXLE[0], "y": AXLE[1], "angleDeg": 0, "lengthM": WHEEL_R, "joint": {"type": "revolute"},
        "mass": {"kg": 0.6, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"},
    })
    for anchor in kit.asset["anchors"]:
        if anchor["id"] in ("wheel-left", "wheel-right"):
            anchor["bone"], anchor["x"], anchor["y"] = "wheels", 0, -WHEEL_R


def pose(kit):
    poses = kit.asset["poses"]
    poses[:] = [p for p in poses if p["id"] != "wheeled-tilted"]
    cold = next(p for p in poses if p["id"] == "cold-closed")
    a = math.radians(TILT)
    turned = (AXLE[0] * math.cos(a) - AXLE[1] * math.sin(a), AXLE[0] * math.sin(a) + AXLE[1] * math.cos(a))
    poses.append({
        "id": "wheeled-tilted",
        "title": text("Tilted on its wheels", "Auf die Räder gekippt"),
        "description": text(
            "The cold grill lifted at the front handle and tilted 20 degrees about the axle: the front foot is 0.15 m off the ground and the grill rolls on its two wheels.",
            "Der kalte Grill am vorderen Griff angehoben und 20 Grad um die Achse gekippt: der vordere Fuss steht 0,15 m über dem Boden und der Grill rollt auf seinen zwei Rädern."),
        "bones": {"wheels": {"angleDeg": -TILT}},
        "root": {"x": round(AXLE[0] - turned[0], 5), "y": round(AXLE[1] - turned[1], 5), "angleDeg": TILT},
        "ground": "authored",
        "contacts": ["wheel-left", "wheel-right"],
        "thermal": copy.deepcopy(cold["thermal"]),
        "loads": [],
    })


def clip(kit):
    clips = kit.asset["clips"]
    clips[:] = [c for c in clips if c["id"] != "tilt-on-wheels"]
    clips.append({
        "id": "tilt-on-wheels",
        "title": text("Tilt onto the wheels and set down", "Auf die Räder kippen und abstellen"),
        "description": text(
            "The cold grill is lifted at the front handle in two seconds, held tilted on its two wheels for two seconds and set down again.",
            "Der kalte Grill wird in zwei Sekunden am vorderen Griff angehoben, zwei Sekunden auf seinen zwei Rädern gekippt gehalten und wieder abgestellt."),
        "durationS": 6, "fps": 12, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "cold-closed", "ease": "smooth"}, {"t": 2, "pose": "wheeled-tilted", "ease": "smooth"}, {"t": 4, "pose": "wheeled-tilted", "ease": "smooth"}, {"t": 6, "pose": "cold-closed", "ease": "smooth"}],
        "contacts": [{"anchor": "wheel-left", "fromS": 0, "toS": 6}, {"anchor": "wheel-right", "fromS": 0, "toS": 6}],
        "events": [{"t": 2, "kind": "motion", "title": text("Front foot 0.15 m up", "Vorderer Fuss 0,15 m angehoben")}, {"t": 4, "kind": "motion", "title": text("Lowering begins", "Absenken beginnt")}],
        "tracks": [],
        "phases": [
            {"id": "lift", "fromS": 0, "toS": 2, "title": text("Lifting the front foot", "Vorderen Fuss anheben")},
            {"id": "hold", "fromS": 2, "toS": 4, "title": text("Held on the wheels", "Auf den Rädern gehalten")},
            {"id": "lower", "fromS": 4, "toS": 6, "title": text("Setting down", "Abstellen")},
        ],
    })


def wheels(kit, scale):
    z = WHEEL_Z[scale]
    for side, sign in (("left", 1), ("right", -1)):
        wheel = kit.solid(scale, "wheel-" + side)
        wheel["bone"], wheel["origin"] = "wheels", [0, 0, sign * z]
        for solid in kit.solids(scale, "structure"):
            if solid["id"] == "member-wheel-" + side:
                solid["member"] = "wheels"
            if solid["id"] in ("joint-wheel-bearing-" + side, "support-wheel-" + side):
                solid["bone"], solid["member"] = "wheels", "wheels"
                if "origin" in solid:
                    solid["origin"] = shift(solid["origin"])
                else:
                    solid["transform"]["translation"] = [round(v, 5) for v in shift(solid["transform"]["translation"])]
    if scale != "50":
        return
    for side, sign in (("left", 1), ("right", -1)):
        for k in range(RIB_COUNT):
            a = math.radians(90 + k * 360 / RIB_COUNT)
            kit.put(scale, {
                "id": "wheel-rib-%s-%d" % (side, k + 1), "kind": "box", "bone": "wheels", "material": "steel-black", "role": "detail", "tone": 0.25,
                "size": RIB_SIZE, "cornerRadius": 0.002,
                "transform": {"translation": [RIB_RADIUS * math.cos(a), RIB_RADIUS * math.sin(a), sign * RIB_Z], "rotation": [0, 0, math.sin(a / 2), math.cos(a / 2)]},
            }, region="legs-steel", after="wheel-" + side if k == 0 else "wheel-rib-%s-%d" % (side, k))


def straps(kit, scale):
    for side in ("left", "right"):
        box = kit.solid(scale, "lid-bracket-" + side)
        if box is None:
            continue
        centre, size = box["transform"]["translation"], box["size"]
        top = centre[1] + size[1] / 2
        bottom = dome_y(0, abs(centre[2]) - size[2] / 2) - 0.0005
        size[1], centre[1] = round(top - bottom, 5), round((top + bottom) / 2, 5)


def grates(kit, scale):
    kept = set(GRATE_KEPT[scale])
    gone = ["grate-bar-%d" % n for n in range(1, 28) if n not in kept]
    kit.drop(scale, gone + ["th-" + g for g in gone])
    thermal = kit.solids(scale, "thermal")
    template = next(s for s in thermal if s["id"].startswith("th-grate-cross"))
    thermal[:] = [s for s in thermal if not s["id"].startswith("th-grate-bar-")]
    for n in sorted(kept):
        bar = dict(kit.solid(scale, "grate-bar-%d" % n))
        bar.update(id="th-grate-bar-%d" % n, material=template["material"], role=template["role"], tone=template["tone"], region=template["region"], layer=template["layer"])
        thermal.append(bar)
    kit.drop(scale, ["fuel-bar-%d" % n for n in range(1, 10) if n not in set(FUEL_KEPT[scale])])
    kit.drop(scale, ["briquette-%d" % n for n in range(1, 15) if n not in set(BRIQUETTES_KEPT[scale])])
    if scale == "200":
        for n in BRIQUETTES_KEPT[scale]:
            kit.solid(scale, "briquette-%d" % n)["radii"] = [0.027, 0.015, 0.021]


def sheets(kit, scale):
    for id in SHEETS[scale]:
        kit.solid(scale, id)["sheet"] = True


def drawings(kit, scale):
    for view in ("elevation", "front", "plan"):
        for shape in kit.shapes(scale, view):
            if shape["id"] in ("wheel", "wheel-hub", "wheel-l", "wheel-r", "wheel-hub-l", "wheel-hub-r"):
                shape["bone"] = "wheels"
                if view == "elevation":
                    shape["cx"], shape["cy"] = 0, 0
    front = kit.shape(scale, "front", "axle")
    if front is not None:
        twin = {k: front[k] for k in ("id", "bone", "role", "filled", "tone", "region") if k in front}
        twin["z"] = 3.0
        if front["kind"] == "capsule":
            twin.update(kind="ellipse", cx=AXLE[0], cy=AXLE[1], rx=front["r0"], ry=front["r0"], material=front["material"])
        else:
            twin.update(kind="poly", points=[[AXLE[0] - 0.005, AXLE[1]], [AXLE[0] + 0.005, AXLE[1]]], closed=False)
        kit.put_shape(scale, "elevation", twin, after="leg-socket-wheel")
    if scale != "50":
        return
    for k in range(RIB_COUNT):
        a = math.radians(90 + k * 360 / RIB_COUNT)
        kit.put_shape(scale, "elevation", {
            "id": "wheel-rib-%d" % (k + 1), "bone": "wheels", "role": "detail", "z": 3.12, "filled": True, "tone": 0.25, "kind": "capsule",
            "x0": 0.028 * math.cos(a), "y0": 0.028 * math.sin(a), "x1": 0.05 * math.cos(a), "y1": 0.05 * math.sin(a), "r0": 0.0045, "r1": 0.0045,
            "region": "legs-steel", "material": "steel-black",
        }, after="wheel" if k == 0 else "wheel-rib-%d" % k)


DECISIONS = {
    "50": (
        "Second round: the two 152 mm wheels are bound to the bone `wheels` on the fixed 10 mm axle and carry five 30 x 9 mm spoke ribs on the outer web, so a turn of the wheels is visible; the two lid straps end 0.5 mm in the dome instead of 5 mm inside the lid.",
        "Zweite Runde: die beiden 152-mm-Räder hängen am Knochen `wheels` auf der festen 10-mm-Achse und tragen fünf Speichenrippen 30 x 9 mm auf dem äusseren Steg, so dass eine Raddrehung sichtbar ist; die beiden Deckelbügel enden 0,5 mm in der Kuppel statt 5 mm im Deckelinnern."),
    "100": (
        "Second round: wheels on the bone `wheels` without ribs (8 mm is below 15 mm); the cooking grate keeps its 27 true 6 mm bars at 19.5 mm pitch, because the pitch is above the feature and the thermal surface of the rods (0.3 m2) depends on them; the lid straps end on the dome.",
        "Zweite Runde: Räder am Knochen `wheels` ohne Rippen (8 mm liegt unter 15 mm); der Grillrost behält seine 27 wahren 6-mm-Stäbe im Abstand 19,5 mm, weil der Abstand über dem Merkmal liegt und die thermische Oberfläche der Stäbe (0,3 m2) von ihnen abhängt; die Deckelbügel enden auf der Kuppel."),
    "200": (
        "Second round: wheels on the bone `wheels`; cooking grate typified to ring and seven bars at 78 mm pitch (as the plan draws it), fuel grate to five bars at 84 mm, the charcoal to seven briquettes of 54 x 30 x 42 mm on the mound; the lid posts end on the dome.",
        "Zweite Runde: Räder am Knochen `wheels`; Grillrost auf Ring und sieben Stäbe im Abstand 78 mm typisiert (wie im Grundriss gezeichnet), Kohlerost auf fünf Stäbe im Abstand 84 mm, die Kohle auf sieben Briketts 54 x 30 x 42 mm auf dem Hügel; die Deckelpfosten enden auf der Kuppel."),
    "500": (
        "Second round: wheels on the bone `wheels`; cooking grate typified to its ring, three bars at 195 mm pitch and the two cross rods, fuel grate to three bars at 168 mm, single briquettes left out (the 76 mm mound is the charcoal).",
        "Zweite Runde: Räder am Knochen `wheels`; Grillrost auf Ring, drei Stäbe im Abstand 195 mm und die zwei Querstäbe typisiert, Kohlerost auf drei Stäbe im Abstand 168 mm, einzelne Briketts weggelassen (der 76-mm-Hügel ist die Kohle)."),
}


def main():
    kit = Kit("objects/charcoal-grill")
    rig(kit)
    pose(kit)
    clip(kit)
    for scale in SCALES:
        wheels(kit, scale)
        straps(kit, scale)
        grates(kit, scale)
        sheets(kit, scale)
        drawings(kit, scale)
        kit.decide(scale, *DECISIONS[scale])
    kit.finish(sys.argv)
    made, out = os.path.join(T, "🗑️generated", "rework-15-charcoal-grill"), os.path.join(T, "🗑️generated", "rework-charcoal-grill")
    os.makedirs(out, exist_ok=True)
    shutil.move(os.path.join(made, "🔣️.json"), os.path.join(out, "🔣️.json"))
    os.rmdir(made)
    print("candidate moved to", os.path.join(out, "🔣️.json"))


if __name__ == "__main__":
    main()
