"""🚐 The 1:50 lists of the round 3 delivery van, solid by solid: body (hollow panel construction with real door apertures, front end with wheel arches, roof, cab with glazing and interior, doors on their bones, underbody, wheels), thermal (layer shells, hidden engine and exhaust, outlet patch) and structure (unibody frame, door frames, suspension, wheels)."""
import math

from r3_delivery_van_kit import BETA, DISC_SOLID, DISC_VENTED, H, QH, QX, RA, RHO, RIM, RT, SLIDER, T, TYRE, W, XF, XR, X_B, X_BULK, X_HINGE, X_QUARTER, X_SKIN, YA, Y_BELT, Y_CANT, Y_DOOR, Y_FLOOR, Y_SILL, ZW, Z_LEAF, arc, arch_points, base, box, capsule, circle, ellipsoid, extrude, loft, member, nose, nose_patch, nose_top, pillar_x, plate, q_axis, q_screen, ref, revolve, roof, rr, screen_point, sec, shrunk, side_z, sweep, torus, wall, windscreen

SIDES = (("l", 1), ("r", -1))
PAINT = dict(material="paint", role="body", tone=0.65)
DOOR = dict(material="paint", role="body", tone=0.6)
TRIM = dict(material="plastic-trim", role="detail", tone=0.2)
SEAT = dict(material="plastic-trim", role="detail", tone=0.3)
STEEL = dict(material="cast-iron", role="detail", tone=0.4)
GLASS = dict(material="glass", role="glazing", tone=0.88)
ARCH = 28


def quarter_outline(x_front):
    return [[-2.60, 0.36], [-2.32, 0.36], [-2.12, Y_SILL]] + arch_points(XR, ARCH) + [[x_front, Y_SILL], [x_front, Y_BELT], [-2.60, Y_BELT]]


def side_panels():
    out = [wall("side-l", 1, quarter_outline(X_BULK), **PAINT)]
    out.append(plate("side-upper-l", 1, [(1.16, -2.60, 0.37), (Y_CANT, -2.60, 0.37)], **PAINT))
    out.append(wall("quarter-r", -1, quarter_outline(X_QUARTER), **PAINT))
    out.append(plate("quarter-upper-r", -1, [(1.16, -2.60, X_QUARTER), (Y_CANT, -2.60, X_QUARTER)], **PAINT))
    out.append(wall("b-post-r", -1, [[X_B, Y_SILL], [X_BULK, Y_SILL], [X_BULK, Y_BELT], [X_B, Y_BELT]], **PAINT))
    out.append(plate("b-post-upper-r", -1, [(1.16, X_B, 0.37), (Y_CANT, X_B, 0.37)], **PAINT))
    out.append(plate("slider-lintel", -1, [(1.84, X_QUARTER, X_B), (Y_CANT, X_QUARTER, X_B)], **PAINT))
    for side, s in SIDES:
        out.append(wall(f"front-door-{side}", s, [[X_BULK, 0.36], [X_HINGE, 0.36], [X_HINGE, Y_BELT], [X_BULK, Y_BELT]], **DOOR))
        out.append(plate(f"cant-rail-{side}", s, [(1.80, 0.37, 1.02), (Y_CANT, 0.37, 0.93)], **DOOR))
        out.append(plate(f"a-pillar-{side}", s, [(y, pillar_x(y) - 0.055, pillar_x(y) + 0.055) for y in (1.15, 1.40, 1.65, Y_CANT)], thickness=0.09, n=6, **PAINT))
        out.append(plate(f"door-frame-front-{side}", s, [(1.16, 1.19, 1.23), (1.47, 1.19, 1.23)], thickness=0.03, proud=0.003, **DOOR))
        out.append(plate(f"door-glass-{side}", s, [(1.165, 0.365, 1.535), (1.805, 0.365, 0.975)], thickness=0.005, sunk=0.012, n=10, **GLASS))
        out.append(box(f"sill-{side}", [2.475, 0.1, 0.066], [0.0475, 0.35, s * 0.955], radius=0.012, material="plastic-trim", role="body", tone=0.25))
        out.append(box(f"door-step-{side}", [2.475, 0.08, 0.07], [0.0475, 0.43, s * 0.915], radius=0.008, material="paint", role="body", tone=0.45))
        out.append(box(f"corner-post-{side}", [0.105, 0.66, 0.125], [-2.5725, 0.85, s * 0.9225], radius=0.03, **PAINT))
        out.append(plate(f"corner-post-upper-{side}", s, [(1.16, -2.625, -2.52), (1.945, -2.625, -2.52)], thickness=0.125, n=6, **PAINT))
        out.append(box(f"tail-lamp-{side}", [0.024, 0.42, 0.085], [-2.628, 1.40, s * 0.895], radius=0.01, material="glass", role="detail", tone=0.35))
        for k, y in enumerate((0.78, 1.66)):
            out.append(capsule(f"rear-hinge-{side}-{k + 1}", [-2.632, y - 0.04, s * 0.868], [-2.632, y + 0.04, s * 0.868], 0.013, **TRIM))
    return out


def shell():
    out = side_panels()
    out.append(roof("roof", **{**PAINT, "tone": 0.7}))
    out.append(nose("front-end", ARCH, **PAINT))
    out.append(nose("bumper-front", ARCH, x0=2.115, grow=0.007, top=0.56, material="plastic-trim", role="body", tone=0.2))
    out.append(loft("engine-undertray", [sec([x, (b + t) / 2, 0], h, (t - b) / 2, 6, QX) for x, b, t, h in [(1.24, 0.24, 0.80, 0.66), (2.15, 0.22, 0.80, 0.66), (2.30, 0.24, 0.60, 0.60)]], material="plastic-trim", role="body", tone=0.12))
    out.append(loft("bumper-rear", [sec([x, 0.44, 0], h, r, 8, QX) for x, r, h in [(-2.67, 0.075, 0.93), (-2.655, 0.095, 0.965), (-2.62, 0.1, 0.99), (-2.38, 0.1, 0.992), (-2.36, 0.085, 0.985)]], material="plastic-trim", role="body", tone=0.2))
    floor = [[0.95, 1.25], [0.95, -1.15], [0.67, -1.15], [0.67, -2.05], [0.95, -2.05], [0.95, -2.61], [-0.95, -2.61], [-0.95, -2.05], [-0.67, -2.05], [-0.67, -1.15], [-0.95, -1.15], [-0.95, 1.25]]
    out.append(extrude("floor", floor, "zx", 0.44, Y_FLOOR, material="paint", role="body", tone=0.45))
    for side, s in SIDES:
        low = math.degrees(math.asin((Y_SILL - YA) / RA))
        ring = arc(XR, YA, 0.465, 180 - low, low, ARCH) + arc(XR, YA, RA, low, 180 - low, ARCH)
        out.append(extrude(f"wheelhouse-{side}", ring, "xy", s * 0.66, s * 0.95, material="paint", role="body", tone=0.4))
        out.append(extrude(f"wheelhouse-wall-{side}", arc(XR, YA, 0.465, 180 - low, low, ARCH), "xy", s * 0.66, s * 0.69, material="paint", role="body", tone=0.4))
    bulk = [[Y_FLOOR, -0.945], [Y_BELT, -0.945], [1.86, -0.87], [1.86, 0.87], [Y_BELT, 0.945], [Y_FLOOR, 0.945]]
    out.append(extrude("bulkhead", bulk, "yz", 0.29, X_BULK, material="paint", role="body", tone=0.55))
    out.append(box("rear-threshold", [0.11, 0.16, 1.72], [-2.555, 0.48, 0], radius=0.01, material="paint", role="body", tone=0.5))
    return out


def glazing():
    out = [windscreen("windscreen", [-0.895, -0.875, -0.845, -0.7, -0.5, -0.25, 0.0, 0.25, 0.5, 0.7, 0.845, 0.875, 0.895])]
    out.append(loft("cowl", [sec([cowl_x(z), 1.205, z], 0.05, 0.016, 3) for z in (-0.89, -0.7, -0.45, -0.2, 0.0, 0.2, 0.45, 0.7, 0.89)], material="plastic-trim", role="detail", tone=0.15))
    for name, z0, z1 in (("driver", 0.74, 0.12), ("passenger", 0.02, -0.60)):
        out.append(capsule(f"wiper-{name}", screen_point(0.05, z0, 0.012), screen_point(0.11, z1, 0.012), 0.008, material="plastic-trim", role="detail", tone=0.1, slender=True))
    return out


def cowl_x(z):
    return 1.775 - sweep(z)


def front_details():
    out = [box("grille", [0.04, 0.2, 0.9], [2.646, 0.80, 0], radius=0.015, material="engine-metal", role="detail", tone=0.15)]
    for k, y in enumerate((0.75, 0.80, 0.85)):
        out.append(box(f"grille-bar-{k + 1}", [0.012, 0.016, 0.86], [2.668, y, 0], radius=0.004, material="plastic-trim", role="texture", tone=0.35, texture=True))
    out.append(box("intake", [0.03, 0.1, 1.0], [2.668, 0.37, 0], radius=0.014, material="plastic-trim", role="detail", tone=0.08))
    out.append(box("plate-front", [0.012, 0.11, 0.52], [2.682, 0.50, 0], radius=0.004, material="plastic-trim", role="detail", tone=0.9))
    out.append(loft("bonnet-skin", [sec([x, nose_top(x) + 0.001, 0], 0.007, h, 3) for x, h in ((1.80, 0.86), (2.0, 0.86), (2.2, 0.855), (2.44, 0.82), (2.54, 0.77))], **{**PAINT, "tone": 0.62}))
    for side, s in SIDES:
        lamp = []
        for x, ry in ((2.30, 0.03), (2.34, 0.07), (2.42, 0.085), (2.50, 0.085), (2.56, 0.08), (2.60, 0.065), (2.625, 0.04)):
            at, _ = nose_patch(x, 0.86, s, 0.008)
            lamp.append(sec(at, 0.02, ry, 4))
        out.append(loft(f"headlamp-{side}", lamp, material="glass", role="glazing", tone=0.9))
        out.append(capsule(f"mirror-arm-{side}", [1.36, 1.26, s * 0.975], [1.40, 1.33, s * 1.03], 0.022, **{**TRIM, "tone": 0.25}))
        out.append(ellipsoid(f"mirror-{side}", [0.06, 0.135, 0.095], [1.415, 1.385, s * 1.04], **{**TRIM, "tone": 0.25}))
        out.append(ellipsoid(f"mirror-glass-{side}", [0.008, 0.115, 0.08], [1.362, 1.385, s * 1.042], **{**GLASS, "tone": 0.9}))
        out.append(capsule(f"door-handle-front-{side}", [0.42, 1.06, s * (W + 0.008)], [0.56, 1.06, s * (W + 0.008)], 0.014, **TRIM))
        out.append(box(f"rub-strip-front-door-{side}", [0.83, 0.05, 0.012], [0.775, 0.72, s * (W + 0.004)], radius=0.005, material="plastic-trim", role="texture", tone=0.2, texture=True))
    out.append(box("rub-strip-side-l", [1.07, 0.05, 0.012], [-0.275, 0.72, W + 0.004], radius=0.005, material="plastic-trim", role="texture", tone=0.2, texture=True))
    out.append(box("slider-track", [1.12, 0.03, 0.012], [-1.36, 1.10, -(W + 0.003)], radius=0.005, material="plastic-trim", role="texture", tone=0.12, texture=True))
    out.append(revolve("fuel-flap", [[-0.003, 0.0], [-0.003, 0.085], [0.003, 0.085], [0.003, 0.0]], [0, 0, 1], [0.05, 0.92, W + 0.0005], material="paint", role="texture", tone=0.55, texture=True))
    return out


def interior():
    out = [loft("dashboard", [sec([1.40, 1.215, z], 0.2, 0.05, 3) for z in (-0.90, -0.5, 0.0, 0.5, 0.90)], **{**SEAT, "tone": 0.2})]
    out.append(ellipsoid("binnacle", [0.13, 0.05, 0.2], [1.30, 1.265, 0.5], **{**SEAT, "tone": 0.2}))
    tilt = math.radians(38)
    axis = [-math.cos(tilt), math.sin(tilt), 0]
    centre = [1.02, 1.21, 0.5]
    out.append(torus("steering-wheel", axis, centre, 0.185, 0.015, **{**SEAT, "tone": 0.15}))
    out.append(capsule("steering-column", [centre[0] - 0.02 * axis[0], centre[1] - 0.02 * axis[1], 0.5], [centre[0] - 0.34 * axis[0], centre[1] - 0.34 * axis[1], 0.5], 0.032, 0.026, **{**SEAT, "tone": 0.15}))
    up = [math.sin(tilt), math.cos(tilt), 0]
    for k, deg in enumerate((90, 210, 330)):
        c, sn = math.cos(math.radians(deg)), math.sin(math.radians(deg))
        out.append(capsule(f"steering-spoke-{k + 1}", centre, [centre[0] + 0.18 * sn * up[0], centre[1] + 0.18 * sn * up[1], centre[2] + 0.18 * c], 0.012, **{**SEAT, "tone": 0.15}))
    for name, zc, half in (("driver", 0.5, 0.24), ("bench", -0.37, 0.43)):
        out.append(box(f"seat-base-{name}", [0.44, 0.3, 2 * half - 0.06], [0.68, 0.71, zc], radius=0.02, **{**SEAT, "tone": 0.2}))
        out.append(loft(f"seat-cushion-{name}", [sec([0.44, 0.905, zc], half, 0.05, 4), sec([0.68, 0.915, zc], half, 0.065, 4), sec([0.94, 0.94, zc], half - 0.01, 0.05, 4)], **SEAT))
        out.append(loft(f"seat-back-{name}", [sec([0.50, 0.93, zc], 0.06, half, 4), sec([0.44, 1.25, zc], 0.06, half - 0.005, 4), sec([0.395, 1.56, zc], 0.045, half - 0.03, 4)], **SEAT))
    for name, zc in (("driver", 0.5), ("bench-outer", -0.59), ("bench-inner", -0.15)):
        out.append(ellipsoid(f"headrest-{name}", [0.05, 0.095, 0.125], [0.385, 1.68, zc], **SEAT))
        for k, dz in enumerate((-0.06, 0.06)):
            out.append(capsule(f"headrest-stem-{name}-{k + 1}", [0.395, 1.55, zc + dz], [0.385, 1.63, zc + dz], 0.006, material="cast-iron", role="detail", tone=0.6))
    out.append(box("interior-mirror", [0.02, 0.06, 0.22], [1.0, 1.76, 0], radius=0.008, **{**SEAT, "tone": 0.2}))
    return out


def slider():
    b = dict(bone="slider", origin=SLIDER)
    out = [wall("slider-panel", -1, [[X_QUARTER, 0.40], [X_B, 0.40], [X_B, Y_BELT], [X_QUARTER, Y_BELT]], proud=0.008, **b, **DOOR)]
    out.append(plate("slider-upper", -1, [(1.16, X_QUARTER, X_B), (1.84, X_QUARTER, X_B)], proud=0.008, **b, **DOOR))
    out.append(capsule("handle-slider", [X_B - 0.21 - SLIDER[0], 1.06 - SLIDER[1], -(W + 0.016)], [X_B - 0.07 - SLIDER[0], 1.06 - SLIDER[1], -(W + 0.016)], 0.014, bone="slider", **TRIM))
    out.append(box("rub-strip-slider", [0.99, 0.05, 0.012], [0.525, 0.72 - SLIDER[1], -(W + 0.012)], radius=0.005, bone="slider", material="plastic-trim", role="texture", tone=0.2, texture=True))
    out.append(box("slider-roller-arm", [0.07, 0.03, 0.04], [0.03, 1.10 - SLIDER[1], -(W + 0.012)], radius=0.008, bone="slider", **TRIM))
    return out


def rear_doors():
    out = []
    for side, s in SIDES:
        bone = f"rear-door-{side}"
        out.append(box(f"rear-leaf-{side}", [1.35, Z_LEAF, T], [0.675, Z_LEAF / 2, -s * T / 2], radius=0.012, bone=bone, **DOOR))
        out.append(box(f"rear-leaf-recess-{side}", [0.5, 0.62, 0.006], [0.36, Z_LEAF / 2, s * 0.002], radius=0.002, bone=bone, material="paint", role="texture", tone=0.57, texture=True))
    out.append(capsule("rear-handle", [0.52, 0.79, -0.014], [0.66, 0.79, -0.014], 0.013, bone="rear-door-r", **TRIM))
    out.append(box("plate-rear", [0.11, 0.52, 0.012], [0.36, 0.43, 0.006], radius=0.004, bone="rear-door-l", material="plastic-trim", role="detail", tone=0.9))
    return out


def underbody():
    out = []
    for side, s in SIDES:
        out.append(box(f"chassis-rail-{side}", [3.85, 0.12, 0.08], [-0.63, 0.38, s * 0.5], radius=0.012, **STEEL))
    for k, x in enumerate((-2.45, -1.05, -0.3, 0.45, 1.15)):
        out.append(box(f"crossmember-{k + 1}", [0.08, 0.08, 0.92], [x, 0.40, 0], radius=0.01, **STEEL))
    out.append(box("fuel-tank", [0.85, 0.14, 0.5], [0.0, 0.37, 0.2], radius=0.05, material="plastic-trim", role="detail", tone=0.15))
    out.append(revolve("spare-wheel", [[0.0, 0.0], [0.0, 0.3], [0.03, 0.33], [0.14, 0.33], [0.17, 0.3], [0.17, 0.0]], [0, 1, 0], [-2.25, 0.27, 0.12], material="rubber", role="detail", tone=0.1))
    out.append(capsule("tail-pipe", [-2.36, 0.30, -0.31], [-2.60, 0.30, -0.31], 0.03, material="exhaust-steel", role="detail", tone=0.4))
    return out


def wheel(tag, bone, s, front):
    w = s * ZW
    axis = [0, 0, s]
    out = [revolve(f"{tag}-tyre", TYRE, axis, [0, 0, w], bone=bone, material="rubber", role="body", tone=0.1)]
    out.append(revolve(f"{tag}-rim", RIM, axis, [0, 0, w], bone=bone, material="steel-rim", role="detail", tone=0.6))
    holes = [circle(0.118 * math.cos(2 * math.pi * k / 8), 0.118 * math.sin(2 * math.pi * k / 8), 0.021, 10) for k in range(8)]
    out.append(extrude(f"{tag}-disc-wheel", circle(0, 0, 0.176, 40), "xy", w + s * 0.05, w + s * 0.058, holes=holes, bone=bone, material="steel-rim", role="detail", tone=0.65))
    out.append(revolve(f"{tag}-hub", [[0.0, 0.0], [0.0, 0.085], [0.062, 0.085], [0.075, 0.07], [0.09, 0.05], [0.098, 0.0]], axis, [0, 0, w], bone=bone, material="steel-rim", role="detail", tone=0.72))
    out.append(capsule(f"{tag}-nut", [0.067, 0, w + s * 0.066], [0.067, 0, w + s * 0.08], 0.011, bone=bone, material="steel-rim", role="detail", tone=0.8, repeat={"count": 5, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 5, 6)}))
    profile = DISC_VENTED if front else DISC_SOLID
    out.append(revolve(f"{tag}-brake-disc", [[a - 0.045, r] for a, r in profile], axis, [0, 0, w], bone=bone, material="cast-iron", role="detail", tone=0.42))
    out.append(revolve(f"{tag}-brake-hat", [[-0.045, 0.0], [-0.045, 0.09], [-0.005, 0.082], [-0.005, 0.0]], axis, [0, 0, w], bone=bone, material="cast-iron", role="detail", tone=0.42))
    return out


def running_gear():
    out = []
    for side, s in SIDES:
        out += wheel(f"f{side}", "wheel-f", s, True) + wheel(f"r{side}", "wheel-r", s, False)
        out.append(box(f"caliper-f{side}", [0.13, 0.06, 0.075], [-0.09, 0.095, s * (ZW - 0.045)], bone="axle-f", material="cast-iron", role="detail", tone=0.32, radius=0.012, rotation=q_axis([0, 0, 1], 43)))
        out.append(box(f"caliper-r{side}", [0.11, 0.05, 0.05], [0.085, 0.09, s * (ZW - 0.045)], bone="axle-r", material="cast-iron", role="detail", tone=0.32, radius=0.012, rotation=q_axis([0, 0, 1], -43)))
        out.append(ellipsoid(f"knuckle-f{side}", [0.06, 0.1, 0.035], [0, 0, s * 0.735], bone="axle-f", **STEEL))
        out.append(capsule(f"lower-arm-f{side}", [0.0, -0.1, s * 0.72], [0.04, -0.075, s * 0.36], 0.024, bone="axle-f", **{**STEEL, "tone": 0.3}))
        out.append(capsule(f"drive-shaft-{side}", [0, 0, s * 0.22], [0, 0, s * 0.71], 0.018, bone="axle-f", **{**STEEL, "tone": 0.3}))
        out.append(loft(f"leaf-spring-{side}", [sec([x, y, s * 0.56], 0.035, 0.014, 6, QX) for x, y in ((-0.66, 0.085), (-0.33, -0.03), (0.0, -0.07), (0.33, -0.03), (0.66, 0.085))], bone="axle-r", **{**STEEL, "tone": 0.3}))
        out.append(capsule(f"damper-r{side}", [-0.07, -0.05, s * 0.6], [-0.17, 0.05, s * 0.6], 0.024, bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(capsule("rear-axle-beam", [0, -0.01, -0.74], [0, -0.01, 0.74], 0.042, bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(box("subframe", [0.5, 0.05, 0.78], [0.02, -0.105, 0], radius=0.015, bone="axle-f", **{**STEEL, "tone": 0.3}))
    return out


def body():
    return shell() + glazing() + front_details() + interior() + slider() + rear_doors() + underbody() + running_gear()


def shells(solids, region, faces=None):
    """🧅 Paint film as the body skin itself and the steel sheet as the same solid 0.08 mm inside it."""
    out = []
    for solid in solids:
        sheet = shrunk(solid, 0.00008)
        sheet.update(id=f"th-{solid['id']}-wall", region=region, layer="wall")
        paint = ref(f"th-{solid['id']}-paint", solid["id"], region, "paint", faces)
        if faces:
            sheet["sectionFaces"] = faces
        out += [sheet, paint]
    return out


INWARD = {"l": "-w", "r": "+w"}
PANELS = {
    "side-l": ["-w"], "side-upper-l": ["-w"], "quarter-r": ["+w"], "quarter-upper-r": ["+w"], "b-post-r": ["+w"], "b-post-upper-r": ["+w"], "slider-lintel": ["+w"],
    "front-door-l": ["-w"], "front-door-r": ["+w"], "cant-rail-l": ["-w"], "cant-rail-r": ["+w"], "a-pillar-l": ["-w"], "a-pillar-r": ["+w"], "door-frame-front-l": ["-w"], "door-frame-front-r": ["+w"],
    "corner-post-l": ["-w", "+u"], "corner-post-r": ["+w", "+u"], "corner-post-upper-l": ["-w", "+u"], "corner-post-upper-r": ["+w", "+u"],
    "front-end": ["-u"], "floor": ["+v"], "wheelhouse-l": ["+v"], "wheelhouse-r": ["+v"], "wheelhouse-wall-l": ["-w"], "wheelhouse-wall-r": ["+w"], "rear-threshold": ["+v", "+u"], "door-step-l": ["+v", "-w"], "door-step-r": ["+v", "+w"],
}


def exhaust_run():
    pipe = dict(material="exhaust-steel", role="detail", tone=0.35, region="exhaust-system", layer="pipe")
    z = -0.31
    out = [capsule("th-downpipe", [1.92, 0.62, -0.1], [1.55, 0.33, z], 0.032, **pipe)]
    out.append(capsule("th-dpf-can", [1.42, 0.33, z], [1.05, 0.33, z], 0.08, **{**pipe, "tone": 0.4}))
    out.append(capsule("th-front-pipe", [0.95, 0.33, z], [-0.72, 0.33, z], 0.03, **pipe))
    out.append(capsule("th-silencer", [-0.80, 0.31, z], [-1.22, 0.31, z], 0.07, lateralScale=1.3, **{**pipe, "tone": 0.4}))
    out.append(capsule("th-axle-pipe-rise", [-1.25, 0.33, z], [-1.48, 0.44, z], 0.03, **pipe))
    out.append(capsule("th-axle-pipe", [-1.48, 0.44, z], [-1.80, 0.44, z], 0.03, **pipe))
    out.append(capsule("th-axle-pipe-fall", [-1.80, 0.44, z], [-2.36, 0.30, z], 0.03, **pipe))
    out.append(capsule("th-tail-pipe", [-2.36, 0.30, z], [-2.60, 0.30, z], 0.03, **pipe))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.03], [0.004, 0.03], [0.004, 0.0]], [-1, 0, 0], [-2.629, 0.30, z], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", layer="pipe", tags=["tailpipe"]))
    return out


def thermal():
    parts = {s["id"]: s for s in body()}
    out = []
    for i, faces in PANELS.items():
        out += shells([parts[i]], "body-panels", faces)
    out += shells([parts["roof"]], "roof", ["-v"])
    out += shells([parts["bonnet-skin"]], "bonnet", ["-v"])
    out += shells([parts["slider-panel"], parts["slider-upper"]], "slider-door", ["+w"])
    out += shells([parts["rear-leaf-l"]], "rear-doors", ["-w"]) + shells([parts["rear-leaf-r"]], "rear-doors", ["+w"])
    out.append(ref("th-windscreen", "windscreen", "cab-glazing", "glass", ["-u", "-v"]))
    for side in "lr":
        out.append(ref(f"th-door-glass-{side}", f"door-glass-{side}", "cab-glazing", "glass", [INWARD[side]]))
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine-block", [0.4, 0.38, 0.46], [2.12, 0.72, 0.16], radius=0.03, **engine))
    out.append(capsule("th-gearbox", [2.10, 0.62, -0.18], [2.10, 0.62, -0.40], 0.11, **engine))
    out.append(capsule("th-exhaust-manifold", [1.90, 0.78, -0.02], [1.90, 0.78, 0.34], 0.035, **engine))
    out += exhaust_run()
    for w in ("fl", "fr", "rl", "rr"):
        out.append(ref(f"th-{w}-tyre", f"{w}-tyre", "tyres", "tread"))
        out.append(ref(f"th-{w}-brake-disc", f"{w}-brake-disc", "brakes"))
        out.append(ref(f"th-{w}-brake-hat", f"{w}-brake-hat", "brakes"))
        out.append(ref(f"th-caliper-{w}", f"caliper-{w}", "brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for i in ("chassis-rail-l", "chassis-rail-r", "crossmember-1", "crossmember-2", "crossmember-3", "crossmember-4", "crossmember-5"):
        out.append({"id": f"st-{i}", "of": i, "member": "chassis", "part": "member"})
    for side, s in SIDES:
        m(box(f"sill-{side}", [2.47, 0.12, 0.09], [0.045, 0.37, s * 0.93], radius=0.02, **frame))
        m(box(f"front-rail-{side}", [1.25, 0.12, 0.08], [1.92, 0.50, s * 0.46], radius=0.015, **frame))
        m(capsule(f"front-rail-kick-{side}", [1.30, 0.48, s * 0.46], [1.10, 0.40, s * 0.5], 0.045, **frame))
        m(box(f"rear-sill-{side}", [1.42, 0.1, 0.07], [-1.89, 0.50, s * 0.62], radius=0.015, **frame))
        m(capsule(f"hinge-pillar-{side}", [1.25, 0.36, s * 0.94], [1.25, 1.15, s * 0.94], 0.04, **frame))
        m(capsule(f"a-pillar-{side}", [pillar_x(1.17), 1.17, s * 0.945], [pillar_x(1.88), 1.88, s * (side_z(1.88) - 0.045)], 0.03, **frame))
        m(capsule(f"b-pillar-{side}", [0.30, 0.36, s * 0.945], [0.30, 1.88, s * (side_z(1.88) - 0.035)], 0.04, 0.03, **frame))
        m(capsule(f"c-pillar-{side}", [-0.84, 0.36, s * 0.945], [-0.84, 1.88, s * (side_z(1.88) - 0.035)], 0.04, 0.03, **frame))
        m(capsule(f"d-pillar-{side}", [-2.57, 0.52, s * 0.925], [-2.57, 1.90, s * (side_z(1.90) - 0.06)], 0.045, 0.035, **frame))
        m(capsule(f"cant-rail-{side}", [-2.57, 1.88, s * (side_z(1.88) - 0.04)], [pillar_x(1.88), 1.88, s * (side_z(1.88) - 0.045)], 0.032, **frame))
        m(capsule(f"waist-rail-{side}", [-2.57, 1.15, s * 0.95], [-0.84, 1.15, s * 0.95], 0.022, **frame))
        m(capsule(f"strut-tower-{side}", [1.66, 0.52, s * 0.52], [1.64, 0.98, s * 0.64], 0.055, **frame))
        m(capsule(f"upper-rail-{side}", [1.25, 1.05, s * 0.92], [2.25, 0.95, s * 0.84], 0.03, **frame))
        m(capsule(f"wheelhouse-hoop-{side}", [-2.03, 0.50, s * 0.67], [-1.17, 0.50, s * 0.67], 0.025, **frame))
    for name, x, y, w, h in (("bumper-beam-front", 2.58, 0.52, 1.3, 0.12), ("firewall-crossmember", 1.26, 0.52, 1.0, 0.1), ("bumper-beam-rear", -2.58, 0.46, 1.5, 0.1)):
        m(box(name, [0.07, h, w], [x, y, 0], radius=0.015, **frame))
    for name, x, y, half in (("cowl-crossmember", 1.56, 1.14, 0.92), ("windscreen-header", 1.0, 1.86, 0.84), ("roof-bow-b", 0.30, 1.90, 0.86), ("roof-bow-2", -0.30, 1.90, 0.86), ("roof-bow-c", -0.84, 1.90, 0.86), ("roof-bow-4", -1.70, 1.90, 0.86), ("rear-header", -2.57, 1.92, 0.85), ("bulkhead-rail", 0.31, 1.15, 0.92)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.028, **frame))
    out.append(member(ellipsoid("com-chassis", [0.15, 0.15, 0.15], [0.48, 0.85, 0], **frame), "chassis", "mass"))
    b = dict(bone="slider", **frame)
    u0, u1, v0, v1, wz = 0.04, 1.01, 0.08, 1.45, -(W - 0.02)
    for name, p0, p1 in (("rear", (u0, v0), (u0, 0.80)), ("front", (u1, v0), (u1, 0.80)), ("bottom", (u0, v0), (u1, v0)), ("waist", (u0, 0.80), (u1, 0.80))):
        out.append(member(capsule(f"slider-frame-{name}", [p0[0], p0[1], wz], [p1[0], p1[1], wz], 0.02, **b), "slider"))
    lean = -(side_z(1.80) - 0.02)
    for name, u in (("rear", u0), ("front", u1)):
        out.append(member(capsule(f"slider-frame-upper-{name}", [u, 0.80, wz], [u, 1.44, lean], 0.02, **b), "slider"))
    out.append(member(capsule("slider-frame-top", [u0, 1.44, lean], [u1, 1.44, lean], 0.02, **b), "slider"))
    for name, u, v, w in (("low", 0.95, 0.06, -0.93), ("mid", 0.03, 0.745, -(W - 0.01)), ("high", 0.95, 1.47, -(side_z(1.83) - 0.03))):
        out.append(member(ellipsoid(f"slider-roller-{name}", [0.03, 0.03, 0.02], [u, v, w], **b), "slider", "joint"))
    for side, s in SIDES:
        bone = f"rear-door-{side}"
        d = dict(bone=bone, **frame)
        w = -s * 0.0225
        for name, p0, p1 in (("hinge-stile", (0.04, 0.04), (1.31, 0.04)), ("lock-stile", (0.04, 0.82), (1.31, 0.82)), ("bottom-rail", (0.04, 0.04), (0.04, 0.82)), ("waist-rail", (0.62, 0.04), (0.62, 0.82)), ("top-rail", (1.31, 0.04), (1.31, 0.82))):
            out.append(member(capsule(f"{bone}-{name}", [p0[0], p0[1], w], [p1[0], p1[1], w], 0.018, **d), bone))
        for k, u in enumerate((0.23, 1.11)):
            out.append(member(ellipsoid(f"{bone}-hinge-{k + 1}", [0.04, 0.02, 0.02], [u, 0.0, w], **d), bone, "joint"))
    a = dict(bone="axle-f", **frame)
    out.append({"id": "st-subframe", "of": "subframe", "member": "axle-f", "part": "member"})
    for side, s in SIDES:
        out.append({"id": f"st-lower-arm-f{side}", "of": f"lower-arm-f{side}", "member": "axle-f", "part": "member"})
        out.append(member(capsule(f"axle-f-strut-{side}", [-0.01, 0.08, s * 0.72], [-0.06, 0.60, s * 0.64], 0.03, **a), "axle-f"))
        out.append(member(capsule(f"axle-f-stub-{side}", [0, 0, s * 0.68], [0, 0, s * 0.78], 0.032, **a), "axle-f"))
        out.append(member(ellipsoid(f"axle-f-top-mount-{side}", [0.045, 0.045, 0.045], [-0.06, 0.62, s * 0.64], **a), "axle-f", "joint"))
    out.append(member(ellipsoid("axle-f-mass", [0.07, 0.07, 0.07], [0, 0, 0], **a), "axle-f", "mass"))
    r = dict(bone="axle-r", **frame)
    out.append({"id": "st-rear-axle-beam", "of": "rear-axle-beam", "member": "axle-r", "part": "member"})
    for side, s in SIDES:
        out.append({"id": f"st-leaf-spring-{side}", "of": f"leaf-spring-{side}", "member": "axle-r", "part": "member"})
        for name, x in (("front", 0.66), ("rear", -0.66)):
            out.append(member(ellipsoid(f"axle-r-spring-eye-{name}-{side}", [0.03, 0.03, 0.04], [x, 0.085, s * 0.56], **r), "axle-r", "joint"))
    out.append(member(ellipsoid("axle-r-mass", [0.07, 0.07, 0.07], [0, -0.01, 0], **r), "axle-r", "mass"))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        for part in ("tyre", "rim", "disc-wheel"):
            out.append({"id": f"st-{tag}-{part}", "of": f"{tag}-{part}", "member": bone, "part": "member"})
        w = dict(bone=bone, material="steel-rim", role="detail", tone=0.6)
        out.append(member(ellipsoid(f"{tag}-bearing", [0.05, 0.05, 0.05], [0, 0, s * 0.78], **w), bone, "joint"))
        out.append(member(ellipsoid(f"{tag}-mass", [0.07, 0.07, 0.04], [0, 0, s * ZW], **w), bone, "mass"))
    return out


def decisions():
    return [
        {"en": "Body: a hollow panel van built like the real one. Side panels, doors and posts are 45 mm slabs with the wheel arches cut as circles of 0.42 m around the 0.356 m tyres; above the belt at 1.17 m they lean in by 80 mm to the roof line; the sliding door and both rear leaves fill real apertures and move on their bones, so the open poses show the cargo hold with floor, wheel houses and bulkhead. The front end is one loft with the front arches, a separate bumper, bonnet skin, grille, wrapped headlamps; the cab has a swept windscreen, A-pillars, door glass, quarter-light bar, mirrors, dashboard, steering wheel, driver seat and double bench.", "de": "Körper: ein hohler Kastenwagen, gebaut wie der echte. Seitenwände, Türen und Pfosten sind 45-mm-Platten mit den Radausschnitten als Kreise von 0,42 m um die 0,356-m-Reifen; über der Brüstung bei 1,17 m neigen sie sich um 80 mm zur Dachlinie; die Schiebetür und beide Heckflügel füllen echte Öffnungen und bewegen sich an ihren Knochen, sodass die offenen Posen den Laderaum mit Boden, Radkästen und Trennwand zeigen. Der Vorderwagen ist ein Loft mit den vorderen Radhäusern, eigenem Stoßfänger, Haubenblech, Grill und umlaufenden Scheinwerfern; das Fahrerhaus hat eine gepfeilte Windschutzscheibe, A-Säulen, Türscheiben, Dreiecksfenstersteg, Spiegel, Armaturenbrett, Lenkrad, Fahrersitz und Doppelsitzbank."},
        {"en": "Thermal: every painted panel is a pair of nested shells (0.08 mm paint film over the 0.8 mm steel sheet), the glazing is the 6 mm glass, engine block, gearbox and manifold sit as hidden solids in the front end, the exhaust run (down pipe, particulate filter, pipe, silencer, pipe over the rear axle) hangs under the floor with the tail pipe below the rear bumper and its mouth as the outlet patch; tyres, brake discs, hats and calipers are the body solids.", "de": "Thermik: jedes lackierte Blech ist ein Paar verschachtelter Schalen (0,08 mm Lackfilm über 0,8 mm Stahlblech), die Verglasung ist das 6-mm-Glas, Motorblock, Getriebe und Krümmer sitzen als verborgene Körper im Vorderwagen, der Abgasstrang (Hosenrohr, Partikelfilter, Rohr, Schalldämpfer, Rohr über der Hinterachse) hängt unter dem Boden mit dem Endrohr unter dem hinteren Stoßfänger und seiner Mündung als Auslassfläche; Reifen, Bremsscheiben, Töpfe und Sättel sind die Körper des Aufbaus."},
        {"en": "Structure: the unibody frame (two chassis rails with five crossmembers, sills, front rails with kick-up, hinge, A, B, C and D pillars, cant rails, waist rails, roof bows, headers, cowl and bulkhead rails, strut towers, bumper beams), the frames of the sliding door and of both rear leaves with rollers and hinges as joints, front subframe with lower arms and struts, rear beam axle on leaf springs, wheels on bearings, and the masses at the centres of mass.", "de": "Tragwerk: der selbsttragende Rahmen (zwei Längsträger mit fünf Querträgern, Schweller, vordere Längsträger mit Kröpfung, Scharnier-, A-, B-, C- und D-Säulen, Dachlängsträger, Brüstungsträger, Dachspriegel, Querträger an Scheibe, Windlauf und Trennwand, Federbeindome, Stoßfängerträger), die Rahmen der Schiebetür und beider Heckflügel mit Rollen und Scharnieren als Gelenke, vorderer Hilfsrahmen mit Querlenkern und Federbeinen, hintere Starrachse an Blattfedern, Räder auf Lagern und die Massen in den Schwerpunkten."},
    ]
