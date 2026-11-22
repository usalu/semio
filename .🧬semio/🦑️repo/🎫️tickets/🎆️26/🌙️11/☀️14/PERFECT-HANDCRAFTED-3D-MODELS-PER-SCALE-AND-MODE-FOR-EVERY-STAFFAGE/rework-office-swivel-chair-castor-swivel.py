"""🛞 Office swivel chair: every castor gets its real second pivot, the vertical swivel stem under the arm boss.

Usage: python3 rework-office-swivel-chair-castor-swivel.py <asset document in> <asset document out> [<volumes source out>]
The catalog file is written with indent 2; run volumes-insert.py afterwards so the volumes member gets its compact form again.
Rig: arm -> swivel-<k> (points down the stem from the boss underside, axial twist = castor swivel) -> castor-<k> (wheel axle, rolls).
2D: every castor shape becomes a placed solid (wheels and hubs extruded along the axle, web and hood extruded, swivel head revolved about the stem), the same profile in elevation, front and plan.
The housing (neck, swivel head, web, hood, stem joint) is rebound from the wheel bone to the stem bone; wheels, axle and floor contact stay on the wheel bone.
Wheel frame to stem frame at rest (trail 20 mm, axle 20 mm under the stem joint): (a, b, c) -> (0.02 - b, a - 0.02, c).
"""
import json, pathlib, sys

argv = sys.argv
source, target = pathlib.Path(argv[1]), pathlib.Path(argv[2])

TRAIL, DROP, JOINT_Y = 0.02, 0.02, 0.045
CASTORS = {
    "front": ("arm-front", "Front", "Vorderer", 0.3, -1, 0.0, -90),
    "side-a": ("arm-side-a", "Side A", "Seitlicher A", 0.0927, -1, 0.2853, -90),
    "side-b": ("arm-side-b", "Side B", "Seitlicher B", 0.0927, -1, -0.2853, -90),
    "back-a": ("arm-back-a", "Back A", "Hinterer A", 0.2427, 1, 0.1763, 90),
    "back-b": ("arm-back-b", "Back B", "Hinterer B", 0.2427, 1, -0.1763, 90),
}
HOUSING = ("neck", "head", "web", "hood")
SCATTER = {"front": 120, "side-a": -60, "side-b": 150, "back-a": 35, "back-b": -140}
ROLLED = {"front": 90, "side-a": 90, "side-b": 90, "back-a": 90, "back-b": 90}


def point(p):
    return [round(TRAIL - p[1], 5), round(p[0] - TRAIL, 5)] + [round(v, 5) for v in p[2:]]


def vector(v):
    return [-v[1] + 0.0, v[0], v[2]]


def rebind(solid, bone):
    solid["bone"] = bone
    if solid["kind"] == "revolve":
        solid["origin"] = point(solid["origin"])
        solid["axis"] = vector(solid["axis"])
    elif solid["kind"] == "extrude":
        assert solid["plane"] == "xy" and "transform" not in solid, solid["id"]
        solid["profile"] = [point(p) for p in solid["profile"]]
    else:
        raise SystemExit("unexpected kind " + solid["id"])


WHEEL_SPAN = {"50": 0.012, "100": 0.010}
HEAD = {
    "50": [[0.011, 0.0], [0.011, 0.011], [0.007, 0.013], [-0.005, 0.013], [-0.005, 0.008], [-0.013, 0.008], [-0.013, 0.0]],
    "100": [[0.011, 0.0], [0.011, 0.013], [-0.013, 0.013], [-0.013, 0.0]],
}


def extruded(z0, z1, side=0.0):
    return {"kind": "extruded", "sweep": "w", "z0": round(side + z0, 5), "z1": round(side + z1, 5)}


def drawn(template, shape_id, bone, geometry, solid, tone=None):
    keep = {k: template[k] for k in ("role", "z", "filled") if k in template}
    tail = {k: template[k] for k in ("region", "material", "penMm") if k in template}
    return {"id": shape_id, "bone": bone, **keep, "tone": template["tone"] if tone is None else tone, **geometry, **tail, "solid": solid}


def disc(radius):
    return {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": radius, "ry": radius}


def outline(points):
    return {"kind": "poly", "points": [[round(v, 5) for v in p] for p in points], "closed": True}


def redraw(shapes, key, scale, body, side):
    """✏️ Castor shapes as placed solids (spans are lateral positions of the rest pose, so the castor's own `side` is added), the same profile in all three views, so a swivelled castor is drawn right."""
    wheel_bone, stem_bone = f"castor-{key}", f"swivel-{key}"
    inner = WHEEL_SPAN.get(scale)
    for index in range(len(shapes) - 1, -1, -1):
        shape = shapes[index]
        if shape.get("bone") != wheel_bone:
            continue
        name = shape["id"]
        if name in (wheel_bone, f"{wheel_bone}-wheel"):
            if shape["kind"] == "ellipse" or inner is None:
                new = [drawn(shape, name, wheel_bone, disc(0.025), extruded(-0.025, 0.025, side))]
            else:
                new = [drawn(shape, f"{name}-l", wheel_bone, disc(0.025), extruded(inner, 0.025, side)),
                       drawn(shape, f"{name}-r", wheel_bone, disc(0.025), extruded(-0.025, -inner, side))]
        elif name == f"{wheel_bone}-hub":
            new = [drawn(shape, name, wheel_bone, disc(0.0172), extruded(-0.0263, 0.0263, side))]
        elif name == f"{wheel_bone}-hood":
            hood = body[f"{wheel_bone}-hood"]
            new = [drawn(shape, f"{wheel_bone}-head", stem_bone, outline(HEAD[scale]),
                         {"kind": "revolved", "axisBone": stem_bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True})]
            if f"{wheel_bone}-web" in body:
                web = body[f"{wheel_bone}-web"]
                new.append(drawn(shape, f"{wheel_bone}-web", stem_bone, outline(web["profile"]), extruded(web["from"], web["to"], side)))
            new.append(drawn(shape, name, stem_bone, outline(hood["profile"]), extruded(hood["from"], hood["to"], side), tone=hood["tone"]))
        else:
            raise SystemExit("unexpected castor shape " + name)
        shapes[index:index + 1] = new



text = source.read_text()
document = json.loads(text)
asset = document["asset"]
bones = asset["rig"]["bones"]
assert not any(b["id"].startswith("swivel-") for b in bones), "already applied"

for key, (arm, en, de, x, sign, z, angle) in CASTORS.items():
    wheel = next(b for b in bones if b["id"] == f"castor-{key}")
    stem = {"id": f"swivel-{key}", "parent": arm,
            "title": {"en": f"{en} castor swivel stem", "de": f"{de} Rollen-Schwenkzapfen"},
            "x": x, "y": sign * JOINT_Y, **({"z": z} if z else {}), "angleDeg": angle, "lengthM": DROP,
            "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0},
            "mass": {"kg": 0.06, "comX": 0.005, "comY": 0, "distribution": "point", "source": "authored-estimate"}}
    bones.insert(bones.index(wheel), stem)
    wheel["parent"] = stem["id"]
    wheel["x"], wheel["y"], wheel["angleDeg"] = DROP, -TRAIL, 90
    wheel.pop("z", None)
    wheel["mass"]["kg"] = 0.09
    wheel["title"] = {"en": wheel["title"]["en"] + " wheels", "de": wheel["title"]["de"] + ", Räder"}
    for scale in asset["volumes"].values():
        for name in ("body", "thermal", "structure"):
            for solid in scale.get(name, []):
                if solid.get("bone") != f"castor-{key}" or "of" in solid:
                    continue
                part = solid["id"].removeprefix(f"castor-{key}-")
                if part in HOUSING or solid["id"] == f"joint-castor-{key}-stem":
                    rebind(solid, stem["id"])
    for scale, representation in asset["representations"].items():
        body = {s["id"]: s for s in asset["volumes"][scale]["body"]}
        for view, shapes in representation["geometry"].items():
            redraw(shapes, key, scale, body, z)

low = next(p for p in asset["poses"] if p["id"] == "seat-low")
for pose_id, twists, en, de, den, dde in (
    ("castors-scattered", SCATTER, "Castors scattered", "Rollen verstreut",
     "Seat low; the five castors have swivelled about their stems into the scattered directions they take after the sitter has shuffled the chair about.",
     "Sitz niedrig; die fünf Rollen sind um ihre Zapfen in die verstreuten Richtungen geschwenkt, die sie nach dem Hin- und Herrücken des Stuhls einnehmen."),
    ("rolled-sideways", ROLLED, "Rolled sideways", "Seitlich gerollt",
     "Seat low; all five castors have swivelled a quarter turn and trail across the chair, as after rolling it sideways along a desk.",
     "Sitz niedrig; alle fünf Rollen sind eine Vierteldrehung geschwenkt und laufen quer zum Stuhl nach, wie nach dem seitlichen Rollen an einem Tisch entlang."),
):
    pose = json.loads(json.dumps(low))
    pose["id"], pose["title"], pose["description"] = pose_id, {"en": en, "de": de}, {"en": den, "de": dde}
    pose["bones"] = {"column": {"dy": 0}, **{f"swivel-{k}": {"twistDeg": t} for k, t in twists.items()}}
    asset["poses"].append(pose)

target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
if len(argv) > 3:
    pathlib.Path(argv[3]).write_text(json.dumps({"volumes": asset["volumes"]}, ensure_ascii=False, indent=1) + "\n")
print("written", target)
