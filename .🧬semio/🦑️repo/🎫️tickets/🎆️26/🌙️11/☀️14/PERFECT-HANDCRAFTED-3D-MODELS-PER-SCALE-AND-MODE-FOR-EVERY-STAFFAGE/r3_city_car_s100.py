"""🚗 The 1:100 lists of the round 3 city-car, authored for 15 mm features: hull with arches, sills, glasshouse on plate pillars, one-piece seats behind the glass, wheels with tyre, dished rim, five spokes and brake disc, one exhaust run; thermal as single surfaces with real hidden internals; structure as the unibody frame reduced to its main members."""
import math

from r3_city_car_kit import QX, ZW, bonnet_y, box, capsule, ellipsoid, hull, hull_patch, loft, member, plan_loft, ref, revolve, roof_loft, screen_x, sec, side_plate, torus, windscreen

SIDES = (("l", 1), ("r", -1))
PAINT = dict(material="paint", role="body", tone=0.62)
GLASS = dict(material="glass", role="glazing", tone=0.85)
SEAT = dict(material="interior-trim", role="detail", tone=0.3)
ROOF = [(0.335, 1.405, 1.432, 0.2, 5), (0.3, 1.403, 1.446, 0.4, 5), (0.25, 1.401, 1.46, 0.575, 5), (0.14, 1.4, 1.478, 0.622, 5), (-0.3, 1.4, 1.499, 0.622, 4.5), (-0.9, 1.4, 1.498, 0.622, 4.5), (-1.35, 1.4, 1.476, 0.612, 5), (-1.525, 1.406, 1.438, 0.575, 6)]
TYRE = [[-0.07, 0.3], [0.07, 0.3], [0.09, 0.28], [0.0925, 0.245], [0.08, 0.195], [-0.08, 0.195], [-0.0925, 0.245], [-0.09, 0.28], [-0.07, 0.3]]
RIM = [[0.08, 0.2], [0.08, 0.175], [0.03, 0.17], [-0.08, 0.17], [-0.08, 0.2], [0.08, 0.2]]


def body():
    out = [hull("hull", 6, **PAINT)]
    out.append(loft("underbody", [sec([x, (b + t) / 2, 0], h, (t - b) / 2, 6, QX) for x, b, t, h in [(-1.88, 0.35, 0.7, 0.5), (-1.7, 0.32, 0.72, 0.6), (-0.92, 0.3, 0.72, 0.6), (-0.9, 0.245, 0.72, 0.6), (0.78, 0.245, 0.72, 0.6), (1.55, 0.26, 0.72, 0.6), (1.88, 0.3, 0.66, 0.5)]], material="plastic-trim", role="body", tone=0.1))
    for side, s in SIDES:
        out.append(loft(f"sill-{side}", [sec([x, 0.26, s * 0.755], 0.07, 0.065, 4, QX) for x in (-0.93, 0.78)], **PAINT))
    out.append(plan_loft("waist", [(0.9, -1.7, 0.9, 0.775), (1.04, -1.75, 0.9, 0.76)], 6, **PAINT))
    out.append(roof_loft("roof", ROOF, **PAINT))
    for side, s in SIDES:
        out.append(side_plate(f"a-pillar-{side}", s, [(0.985, 0.715, 0.835), (1.425, 0.16, 0.262)], 0.03, -0.022, 4, **PAINT))
        out.append(side_plate(f"b-pillar-{side}", s, [(0.99, -0.375, -0.225), (1.425, -0.375, -0.225)], 0.014, -0.004, 4, **PAINT))
        out.append(side_plate(f"c-pillar-{side}", s, [(0.99, -1.76, -1.28), (1.425, -1.5, -1.16)], 0.02, -0.012, 4, **PAINT))
        out.append(loft(f"d-pillar-{side}", [sec([-1.735, 0.985, s * 0.655], 0.045, 0.115, 4), sec([-1.475, 1.42, s * 0.572], 0.035, 0.05, 4)], **PAINT))
        out.append(side_plate(f"side-glass-front-{side}", s, [(1.035, -0.24, 0.7), (1.42, -0.24, 0.21)], 0.002, 0.002, 8, **GLASS))
        out.append(side_plate(f"side-glass-rear-{side}", s, [(1.035, -1.3, -0.36), (1.42, -1.18, -0.36)], 0.002, 0.002, 8, **GLASS))
    out.append(windscreen("windscreen", [-0.74, -0.585, -0.3, 0.0, 0.3, 0.585, 0.74]))
    out.append(loft("cowl", [sec([0.93 - screen_x(z), 1.018, z], 0.055, 0.03, 3) for z in (-0.735, -0.4, 0.0, 0.4, 0.735)], material="plastic-trim", role="detail", tone=0.15))
    out.append(box("grille", [0.03, 0.13, 0.9], [1.94, 0.42, 0], radius=0.014, material="engine-metal", role="detail", tone=0.15))
    for side, s in SIDES:
        at, q = hull_patch(1.84, 0.7, s, 0.02)
        out.append(ellipsoid(f"headlamp-{side}", [0.13, 0.06, 0.035], at, rotation=q, material="glass", role="glazing", tone=0.9))
        out.append(box(f"tail-lamp-{side}", [0.05, 0.15, 0.1], [-1.912, 0.705, s * 0.65], radius=0.02, material="glass", role="detail", tone=0.7))
        out.append(ellipsoid(f"mirror-{side}", [0.055, 0.06, 0.105], [0.76, 1.08, s * 0.845], material="plastic-trim", role="detail", tone=0.3))
    out.append(plan_loft("cabin-deck", [(1.03, -1.73, 0.87, 0.75), (1.052, -1.72, 0.86, 0.745)], 6, **{**SEAT, "tone": 0.22}))
    for side, s in SIDES:
        out.append(loft(f"seat-front-{side}", [sec([-0.17, 0.8, s * 0.36], 0.055, 0.24, 3), sec([-0.32, 1.12, s * 0.36], 0.055, 0.23, 3), sec([-0.365, 1.22, s * 0.36], 0.045, 0.12, 3), sec([-0.41, 1.36, s * 0.36], 0.045, 0.11, 3)], **SEAT))
    out.append(loft("seat-rear", [sec([-0.98, 0.8, 0], 0.055, 0.63, 4), sec([-1.1, 1.08, 0], 0.055, 0.62, 4), sec([-1.15, 1.2, 0], 0.045, 0.58, 4)], **SEAT))
    axis = [-math.cos(math.radians(25)), math.sin(math.radians(25)), 0]
    out.append(torus("steering-wheel", axis, [0.36, 1.0, 0.36], 0.171, 0.0125, 6, **{**SEAT, "tone": 0.2}))
    out.append(loft("bonnet-skin", [sec([u, -0.004, 0], h, t, 2.5) for u, h, t in ((0.0, 0.705, 0.02), (0.5, 0.69, 0.03), (0.915, 0.62, 0.014))], bone="bonnet", **PAINT))
    out.append(loft("hatch-panel", [sec([0.47, 0.022, 0], 0.625, 0.022, 6), sec([0.745, 0.022, 0], 0.655, 0.022, 6)], bone="hatch", **PAINT))
    out.append(loft("hatch-frame", [sec([0.0, 0.026, 0], 0.55, 0.014, 6), sec([0.49, 0.026, 0], 0.625, 0.014, 6)], bone="hatch", material="paint", role="body", tone=0.35))
    out.append(loft("hatch-lip", [sec([0.735, 0.018, 0], 0.022, 0.6, 6), sec([0.911, 0.144, 0], 0.022, 0.6, 6)], bone="hatch", **PAINT))
    out.append(loft("hatch-glass", [sec([0.04, 0.006, 0], 0.535, 0.002, 8), sec([0.48, 0.006, 0], 0.6, 0.002, 8)], bone="hatch", **GLASS))
    pipe = dict(material="exhaust-steel", role="detail", tone=0.35)
    out.append(capsule("exhaust-pipe", [0.95, 0.25, -0.1], [-1.5, 0.3, -0.16], 0.025, **pipe))
    out.append(capsule("silencer", [-1.66, 0.315, -0.44], [-1.66, 0.315, 0.0], 0.085, lateralScale=0.8, **{**pipe, "tone": 0.4}))
    out.append(capsule("tail-pipe", [-1.7, 0.305, -0.3], [-1.925, 0.31, -0.3], 0.025, **{**pipe, "tone": 0.45}))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        axis, origin = [0, 0, s], [0, 0, s * ZW]
        out.append(revolve(f"{tag}-tyre", TYRE, axis, origin, bone=bone, material="rubber", role="body", tone=0.1))
        out.append(revolve(f"{tag}-rim", RIM, axis, origin, bone=bone, material="alloy", role="detail", tone=0.55))
        out.append(revolve(f"{tag}-hub", [[0.01, 0.0], [0.01, 0.06], [0.07, 0.05], [0.078, 0.0]], axis, origin, bone=bone, material="alloy", role="detail", tone=0.7))
        out.append(capsule(f"{tag}-spoke", [0, 0.04, s * (ZW + 0.058)], [0, 0.18, s * (ZW + 0.062)], 0.026, 0.02, bone=bone, material="alloy", role="detail", tone=0.8, lateralScale=0.6, repeat={"count": 5, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 5, 6)}))
        out.append(revolve(f"{tag}-disc", [[-0.016, 0.0], [-0.016, 0.15], [0.0, 0.15], [0.0, 0.08], [0.03, 0.075], [0.03, 0.0]], axis, origin, bone=bone, material="cast-iron", role="detail", tone=0.42))
    steel = dict(material="cast-iron", role="detail", tone=0.3)
    out.append(capsule("drive-shafts", [0, 0, -0.66], [0, 0, 0.66], 0.02, bone="axle-f", **steel))
    out.append(box("subframe", [0.36, 0.05, 0.7], [0.05, -0.07, 0], radius=0.015, bone="axle-f", **steel))
    out.append(capsule("twist-beam", [0.2, 0.0, -0.56], [0.2, 0.0, 0.56], 0.035, bone="axle-r", **steel))
    for side, s in SIDES:
        out.append(capsule(f"lower-arm-{side}", [0.0, -0.085, s * 0.64], [0.03, -0.06, s * 0.33], 0.02, bone="axle-f", **steel))
        out.append(capsule(f"trailing-arm-{side}", [0.42, 0.02, s * 0.56], [0.0, 0.0, s * 0.63], 0.03, bone="axle-r", **steel))
    return out


def thermal():
    parts = {s["id"]: s for s in body()}
    out = [ref("th-hull", "hull", "body-panels", faces=["+v"]), ref("th-underbody", "underbody", "body-panels"), ref("th-waist", "waist", "body-panels", faces=["+v"])]
    for side, face in (("l", "-w"), ("r", "+w")):
        out.append(ref(f"th-sill-{side}", f"sill-{side}", "body-panels"))
        for p in "abc":
            out.append(ref(f"th-{p}-pillar-{side}", f"{p}-pillar-{side}", "body-panels", faces=[face]))
        out.append(ref(f"th-d-pillar-{side}", f"d-pillar-{side}", "body-panels"))
        out.append(ref(f"th-side-glass-front-{side}", f"side-glass-front-{side}", "glazing", faces=[face]))
        out.append(ref(f"th-side-glass-rear-{side}", f"side-glass-rear-{side}", "glazing", faces=[face]))
        out.append(ref(f"th-headlamp-{side}", f"headlamp-{side}", "glazing"))
    out.append(ref("th-roof", "roof", "roof", faces=["-v"]))
    out.append(ref("th-bonnet", "bonnet-skin", "bonnet", faces=["-v"]))
    out.append(ref("th-hatch-panel", "hatch-panel", "tailgate", faces=["+v"]))
    out.append(ref("th-hatch-lip", "hatch-lip", "tailgate", faces=["+v"]))
    out.append(ref("th-windscreen", "windscreen", "glazing", faces=["-v"]))
    out.append(ref("th-hatch-glass", "hatch-glass", "glazing", faces=["+v"]))
    foam = dict(material="interior-trim", role="detail", tone=0.3, region="interior")
    for i in ("seat-front-l", "seat-front-r", "seat-rear"):
        out.append({**parts[i], "id": f"th-{i}", "region": "interior"})
    for side, s in SIDES:
        out.append(box(f"th-seat-cushion-{side}", [0.5, 0.12, 0.48], [0.07, 0.6, s * 0.36], radius=0.04, **foam))
    out.append(box("th-bench-cushion", [0.48, 0.12, 1.22], [-0.74, 0.62, 0], radius=0.04, **foam))
    out.append(box("th-dashboard", [0.2, 0.22, 1.3], [0.7, 0.86, 0], radius=0.06, **foam))
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine-block", [0.3, 0.38, 0.38], [1.36, 0.56, 0.13], radius=0.03, **engine))
    out.append(capsule("th-gearbox", [1.33, 0.48, -0.14], [1.33, 0.48, -0.36], 0.13, **engine))
    out.append(capsule("th-exhaust-manifold", [1.17, 0.62, 0.02], [1.12, 0.34, -0.08], 0.045, **engine))
    for i in ("exhaust-pipe", "silencer", "tail-pipe"):
        out.append(ref(f"th-{i}", i, "exhaust-system"))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.0225], [0.015, 0.0225], [0.015, 0.0]], [-1, 0, 0], [-1.94, 0.31, -0.3], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for w in ("fl", "fr", "rl", "rr"):
        out.append(ref(f"th-{w}-tyre", f"{w}-tyre", "tyres"))
        out.append(ref(f"th-{w}-disc", f"{w}-disc", "brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for side, s in SIDES:
        m(box(f"sill-{side}", [1.7, 0.1, 0.09], [-0.075, 0.265, s * 0.745], radius=0.02, **frame))
        m(loft(f"front-rail-{side}", [sec([1.86, 0.47, s * 0.42], 0.05, 0.035, 4), sec([0.9, 0.47, s * 0.42], 0.05, 0.035, 4), sec([0.6, 0.29, s * 0.52], 0.04, 0.04, 4)], **frame))
        m(loft(f"rear-rail-{side}", [sec([-0.84, 0.28, s * 0.5], 0.04, 0.035, 4), sec([-1.05, 0.52, s * 0.5], 0.04, 0.035, 4), sec([-1.86, 0.52, s * 0.5], 0.04, 0.035, 4)], **frame))
        m(capsule(f"hinge-pillar-{side}", [0.76, 0.3, s * 0.72], [0.8, 0.96, s * 0.72], 0.04, **frame))
        m(capsule(f"a-pillar-{side}", [0.8, 0.98, s * 0.74], [0.235, 1.42, s * 0.585], 0.03, **frame))
        m(capsule(f"b-pillar-{side}", [-0.3, 0.3, s * 0.745], [-0.3, 1.42, s * 0.6], 0.04, 0.03, **frame))
        m(capsule(f"c-pillar-{side}", [-1.0, 0.72, s * 0.74], [-1.3, 1.42, s * 0.6], 0.045, 0.035, **frame))
        m(capsule(f"roof-rail-{side}", [0.235, 1.425, s * 0.585], [-1.46, 1.425, s * 0.575], 0.03, **frame))
    for name, x, y, w, h in (("bumper-beam-front", 1.87, 0.47, 1.0, 0.1), ("firewall-crossmember", 0.86, 0.45, 1.0, 0.1), ("seat-crossmember", -0.15, 0.26, 1.4, 0.06), ("heel-kick", -0.86, 0.3, 1.4, 0.1), ("bumper-beam-rear", -1.87, 0.52, 1.1, 0.1)):
        m(box(name, [0.07, h, w], [x, y, 0], radius=0.015, **frame))
    for name, x, y, half in (("cowl-crossmember", 0.82, 0.97, 0.72), ("windscreen-header", 0.235, 1.425, 0.585), ("roof-bow", -0.3, 1.44, 0.6), ("rear-header", -1.46, 1.425, 0.575)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.028, **frame))
    out.append(member(ellipsoid("com-chassis", [0.13, 0.13, 0.13], [0.28, 0.55, 0], **frame), "chassis", "mass"))
    for side, s in SIDES:
        out.append(member(box(f"bonnet-frame-{side}", [0.86, 0.03, 0.05], [0.46, -0.02, s * 0.6], radius=0.01, bone="bonnet", **frame), "bonnet"))
    for name, u in (("rear", 0.04), ("front", 0.88)):
        out.append(member(box(f"bonnet-frame-{name}", [0.05, 0.03, 1.2], [u, -0.02, 0], radius=0.01, bone="bonnet", **frame), "bonnet"))
    for side, s in SIDES:
        out.append(member(capsule(f"hatch-frame-{side}", [0.03, 0.03, s * 0.52], [0.72, 0.03, s * 0.6], 0.022, bone="hatch", **frame), "hatch"))
    for name, u, half in (("top", 0.03, 0.52), ("waist", 0.5, 0.58), ("bottom", 0.72, 0.6)):
        out.append(member(capsule(f"hatch-frame-{name}", [u, 0.03, -half], [u, 0.03, half], 0.022, bone="hatch", **frame), "hatch"))
    out.append({"id": "st-drive-shafts", "of": "drive-shafts", "member": "axle-f", "part": "member"})
    out.append({"id": "st-subframe", "of": "subframe", "member": "axle-f", "part": "member"})
    out.append({"id": "st-twist-beam", "of": "twist-beam", "member": "axle-r", "part": "member"})
    for side, s in SIDES:
        out.append(member(capsule(f"strut-{side}", [-0.005, 0.07, s * 0.635], [-0.04, 0.42, s * 0.585], 0.03, bone="axle-f", **frame), "axle-f"))
        out.append({"id": f"st-lower-arm-{side}", "of": f"lower-arm-{side}", "member": "axle-f", "part": "member"})
        out.append({"id": f"st-trailing-arm-{side}", "of": f"trailing-arm-{side}", "member": "axle-r", "part": "member"})
        out.append(member(capsule(f"spring-{side}", [0.08, -0.02, s * 0.5], [0.08, 0.2, s * 0.5], 0.05, bone="axle-r", **frame), "axle-r"))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        for part in ("tyre", "rim", "spoke"):
            out.append({"id": f"st-{tag}-{part}", "of": f"{tag}-{part}", "member": bone, "part": "member"})
        out.append(member(ellipsoid(f"{tag}-bearing", [0.05, 0.05, 0.05], [0, 0, s * 0.68], bone=bone, material="alloy", role="detail", tone=0.6), bone, "joint"))
    return out


def decisions():
    return [
        {"en": "At 1:100 the body is the same design drawn for 15 mm features: one hull loft with both wheel arches, the sills, the waist rail, a roof with its crown and swept header, plate pillars in the tumblehome plane and thin glass between them; panel joints, door handles, number plates, wipers and brake calipers are left to the drawing.", "de": "Im Maßstab 1:100 ist die Karosserie derselbe Entwurf für 15-mm-Merkmale: ein Rumpf-Loft mit beiden Radhäusern, die Schweller, die Brüstung, ein Dach mit Wölbung und gepfeiltem Windlauf, Plattenpfosten in der Ebene der Seitenscheiben und dünnes Glas dazwischen; Fugen, Türgriffe, Kennzeichen, Wischer und Bremssättel bleiben der Zeichnung überlassen."},
        {"en": "Each seat is one loft from cushion to headrest, the exhaust is one pipe with silencer and tail pipe, each wheel is tyre, dished rim, hub, five spokes and one plain brake disc; thermal surfaces reference these solids, the layers of 1:50 are merged into single surfaces, and the engine, gearbox, manifold, seat cushions and dashboard are real hidden solids.", "de": "Jeder Sitz ist ein Loft vom Polster bis zur Kopfstütze, die Abgasanlage ein Rohr mit Schalldämpfer und Endrohr, jedes Rad besteht aus Reifen, geschüsselter Felge, Nabe, fünf Speichen und einer glatten Bremsscheibe; die thermischen Flächen verweisen auf diese Körper, die Schichten von 1:50 sind zu einfachen Flächen zusammengefasst, und Motor, Getriebe, Krümmer, Sitzpolster und Armaturenbrett sind echte verdeckte Körper."},
        {"en": "The structure is the unibody frame without its secondary members: sills, front and rear rails with their kick-ups, hinge, A, B and C pillars, roof rails, five floor crossmembers and four roof bows, the bonnet and tailgate frames, struts, subframe, twist beam and the wheels on their bearings.", "de": "Das Tragwerk ist der selbsttragende Rahmen ohne Nebenträger: Schweller, vordere und hintere Längsträger mit ihren Kröpfungen, Scharnier-, A-, B- und C-Säulen, Dachholme, fünf Bodenquerträger und vier Dachspriegel, die Rahmen von Haube und Heckklappe, Federbeine, Hilfsrahmen, Verbundlenkerachse und die Räder auf ihren Lagern."},
    ]
