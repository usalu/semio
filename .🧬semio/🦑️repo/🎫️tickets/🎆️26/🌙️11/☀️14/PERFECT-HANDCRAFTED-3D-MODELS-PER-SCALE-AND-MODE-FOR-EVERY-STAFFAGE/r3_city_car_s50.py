"""🚙 The 1:50 lists of the round 3 city-car, solid by solid: body (hull with real wheel arches, sills, hollow glasshouse on pillars, interior, wheels with tyres, rims, discs and calipers, exhaust), thermal (layer shells, hidden engine and cabin internals, outlet patch) and structure (unibody frame, suspension, wheels)."""
import math

from r3_city_car_kit import BETA, DISC_SOLID, DISC_VENTED, QH, QX, RA, RHO, RIM, RT, TYRE, XF, XR, YA, ZW, base, bonnet_y, box, capsule, ellipsoid, extrude, hatch_y, hull, hull_patch, hull_z, loft, plan_loft, q_axis, revolve, roof_loft, rr, screen_point, screen_x, sec, shrunk, side_plate, side_z, torus, windscreen

SIDES = (("l", 1), ("r", -1))
PAINT = dict(material="paint", role="body", tone=0.62)
TRIM = dict(material="plastic-trim", role="detail", tone=0.2)
SEAT = dict(material="interior-trim", role="detail", tone=0.3)
STEEL = dict(material="cast-iron", role="detail", tone=0.5)

WAIST = [(0.90, -1.70, 0.90, 0.775), (1.0, -1.775, 0.925, 0.775), (1.045, -1.74, 0.878, 0.758)]
ROOF = [(0.34, 1.406, 1.428, 0.12, 4), (0.33, 1.405, 1.434, 0.2, 5), (0.317, 1.404, 1.44, 0.3, 5), (0.299, 1.403, 1.447, 0.4, 5), (0.275, 1.402, 1.454, 0.5, 5), (0.25, 1.401, 1.46, 0.575, 5), (0.21, 1.4, 1.468, 0.61, 5), (0.14, 1.4, 1.478, 0.622, 5), (0.0, 1.4, 1.49, 0.622, 4.5), (-0.3, 1.4, 1.499, 0.622, 4.5), (-0.7, 1.4, 1.5, 0.622, 4.5), (-1.1, 1.4, 1.493, 0.62, 4.5), (-1.35, 1.4, 1.476, 0.612, 5), (-1.47, 1.402, 1.456, 0.598, 6), (-1.525, 1.406, 1.438, 0.575, 6)]


def shell():
    out = [hull("hull", 12, **PAINT)]
    out.append(loft("underbody", [sec([x, (b + t) / 2, 0], h, (t - b) / 2, 6, QX) for x, b, t, h in [(-1.90, 0.36, 0.70, 0.50), (-1.70, 0.33, 0.72, 0.60), (-0.95, 0.30, 0.72, 0.60), (-0.90, 0.245, 0.72, 0.60), (0.75, 0.245, 0.72, 0.60), (0.80, 0.25, 0.72, 0.60), (1.50, 0.25, 0.72, 0.60), (1.56, 0.275, 0.72, 0.60), (1.88, 0.30, 0.66, 0.50)]], material="plastic-trim", role="body", tone=0.1))
    for side, s in SIDES:
        out.append(loft(f"sill-{side}", [sec([x, 0.255, s * 0.755], 0.07, 0.062, 5, QX) for x in (-0.931, 0.781)], **PAINT))
    out.append(plan_loft("waist", WAIST, 6, **PAINT))
    out.append(roof_loft("roof", ROOF, **PAINT))
    for side, s in SIDES:
        out.append(side_plate(f"a-pillar-{side}", s, [(0.985, 0.715, 0.835), (1.2, 0.445, 0.55), (1.425, 0.16, 0.262)], 0.03, -0.022, 4, **PAINT))
        out.append(side_plate(f"b-pillar-{side}", s, [(0.99, -0.37, -0.23), (1.425, -0.37, -0.23)], 0.012, -0.004, 4, **PAINT))
        out.append(side_plate(f"c-pillar-{side}", s, [(0.99, -1.76, -1.28), (1.425, -1.5, -1.16)], 0.02, -0.012, 4, **PAINT))
        out.append(loft(f"d-pillar-{side}", [sec([-1.735, 0.985, s * 0.655], 0.045, 0.115, 4), sec([-1.6, 1.2, s * 0.61], 0.04, 0.085, 4), sec([-1.475, 1.42, s * 0.572], 0.035, 0.05, 4)], **PAINT))
    return out


def glazing():
    out = [windscreen("windscreen", [-0.74, -0.70, -0.64, -0.585, -0.4, -0.2, 0.0, 0.2, 0.4, 0.585, 0.64, 0.70, 0.74])]
    glass = dict(material="glass", role="glazing", tone=0.85)
    for side, s in SIDES:
        out.append(side_plate(f"side-glass-front-{side}", s, [(1.035, -0.24, 0.70), (1.42, -0.24, 0.21)], 0.002, 0.002, 8, **glass))
        out.append(side_plate(f"side-glass-rear-{side}", s, [(1.035, -1.30, -0.36), (1.42, -1.18, -0.36)], 0.002, 0.002, 8, **glass))
    out.append(loft("cowl", [sec([0.93 - screen_x(z), 1.018, z], 0.055, 0.03, 3) for z in (-0.735, -0.6, -0.4, -0.2, 0.0, 0.2, 0.4, 0.6, 0.735)], material="plastic-trim", role="detail", tone=0.15))
    for name, z0, z1 in (("driver", 0.64, 0.08), ("passenger", -0.02, -0.56)):
        out.append(capsule(f"wiper-{name}", screen_point(0.085, z0, 0.012), screen_point(0.125, z1, 0.012), 0.007, material="plastic-trim", role="detail", tone=0.1, slender=True))
    return out


def front_end():
    out = [loft("engine-cover", [sec([x, bonnet_y(x) - 0.004, 0], 0.016, h, 2.5) for x, h in ((1.02, 0.62), (1.4, 0.61), (1.80, 0.56))], material="engine-metal", role="detail", tone=0.12)]
    out.append(box("grille", [0.03, 0.05, 0.62], [1.94, 0.665, 0], radius=0.012, material="engine-metal", role="detail", tone=0.15))
    out.append(box("intake", [0.03, 0.1, 0.9], [1.94, 0.41, 0], radius=0.014, material="plastic-trim", role="detail", tone=0.15))
    out.append(box("plate-front", [0.012, 0.11, 0.52], [1.955, 0.53, 0], radius=0.004, material="plastic-trim", role="detail", tone=0.9))
    for side, s in SIDES:
        at, q = hull_patch(1.84, 0.70, s, 0.02)
        out.append(ellipsoid(f"headlamp-{side}", [0.13, 0.06, 0.035], at, rotation=q, material="glass", role="glazing", tone=0.9))
        out.append(box(f"tail-lamp-{side}", [0.05, 0.15, 0.1], [-1.912, 0.705, s * 0.65], radius=0.02, material="glass", role="detail", tone=0.7))
        out.append(ellipsoid(f"mirror-{side}", [0.055, 0.065, 0.095], [0.76, 1.085, s * 0.855], **{**TRIM, "tone": 0.3}))
        out.append(capsule(f"mirror-foot-{side}", [0.78, 1.035, s * 0.755], [0.765, 1.075, s * 0.80], 0.024, **{**TRIM, "tone": 0.3}))
        out.append(ellipsoid(f"mirror-glass-{side}", [0.008, 0.05, 0.078], [0.712, 1.085, s * 0.857], material="glass", role="glazing", tone=0.9))
        for name, x0, x1 in (("front", -0.26, -0.14), ("rear", -0.96, -0.84)):
            z = hull_z((x0 + x1) / 2, 0.90) + 0.006
            out.append(capsule(f"door-handle-{name}-{side}", [x0, 0.90, s * z], [x1, 0.90, s * z], 0.012, **{**TRIM, "tone": 0.3}))
    return out


def seams():
    out = []
    lines = {"door-front": [(0.745, 0.33), (0.745, 0.6), (0.745, 0.85), (0.75, 0.95)], "door-b": [(-0.30, 0.33), (-0.30, 0.6), (-0.30, 0.85), (-0.30, 0.95)], "door-rear": [(-1.03, 0.58), (-1.02, 0.72), (-1.01, 0.86), (-1.0, 0.95)]}
    for side, s in SIDES:
        for name, points in lines.items():
            for k, ((x0, y0), (x1, y1)) in enumerate(zip(points, points[1:])):
                out.append(capsule(f"seam-{name}-{side}-{k + 1}", [x0, y0, s * (hull_z(x0, y0) + 0.001)], [x1, y1, s * (hull_z(x1, y1) + 0.001)], 0.004, material="plastic-trim", role="texture", tone=0.25, texture=True))
    z = hull_z(-1.52, 0.78)
    out.append(revolve("fuel-flap", [[-0.004, 0.0], [-0.004, 0.068], [0.004, 0.068], [0.004, 0.0]], [0, 0, -1], [-1.52, 0.78, -z], material="paint", role="texture", tone=0.5, texture=True))
    return out


def interior():
    out = [plan_loft("cabin-deck", [(1.03, -1.73, 0.87, 0.75), (1.052, -1.72, 0.86, 0.745)], 6, **{**SEAT, "tone": 0.22})]
    out.append(ellipsoid("binnacle", [0.11, 0.04, 0.17], [0.60, 1.055, 0.36], **{**SEAT, "tone": 0.2}))
    axis = [-math.cos(math.radians(25)), math.sin(math.radians(25)), 0]
    centre = [0.36, 1.0, 0.36]
    out.append(torus("steering-wheel", axis, centre, 0.171, 0.014, **{**SEAT, "tone": 0.2}))
    out.append(capsule("steering-column", [centre[0] - 0.02 * axis[0], centre[1] - 0.02 * axis[1], 0.36], [centre[0] - 0.3 * axis[0], centre[1] - 0.3 * axis[1], 0.36], 0.03, 0.022, **{**SEAT, "tone": 0.2}))
    up = [math.sin(math.radians(25)), math.cos(math.radians(25)), 0]
    for k, deg in enumerate((90, 210, 330)):
        c, sn = math.cos(math.radians(deg)), math.sin(math.radians(deg))
        tip = [centre[0] + 0.165 * sn * up[0], centre[1] + 0.165 * sn * up[1], centre[2] + 0.165 * c]
        out.append(capsule(f"steering-spoke-{k + 1}", centre, tip, 0.012, **{**SEAT, "tone": 0.2}))
    for side, s in SIDES:
        zc = s * 0.36
        out.append(loft(f"seat-front-back-{side}", [sec([-0.17, 0.80, zc], 0.055, 0.24, 3), sec([-0.30, 1.08, zc], 0.055, 0.235, 3), sec([-0.355, 1.20, zc], 0.045, 0.20, 3)], **SEAT))
        out.append(ellipsoid(f"seat-front-headrest-{side}", [0.05, 0.085, 0.12], [-0.40, 1.30, zc], **SEAT))
        for k, dz in enumerate((-0.06, 0.06)):
            out.append(capsule(f"headrest-stem-{side}-{k + 1}", [-0.355, 1.18, zc + dz], [-0.385, 1.25, zc + dz], 0.006, material="cast-iron", role="detail", tone=0.6))
        out.append(ellipsoid(f"seat-rear-headrest-{side}", [0.045, 0.07, 0.11], [-1.165, 1.215, s * 0.34], **SEAT))
    out.append(loft("seat-rear-back", [sec([-0.98, 0.80, 0], 0.055, 0.63, 4), sec([-1.09, 1.05, 0], 0.055, 0.625, 4), sec([-1.13, 1.14, 0], 0.045, 0.60, 4)], **SEAT))
    out.append(box("interior-mirror", [0.02, 0.055, 0.2], [0.20, 1.385, 0], radius=0.008, **{**SEAT, "tone": 0.2}))
    return out


def bonnet():
    return [loft("bonnet-skin", [sec([u, -0.004, 0], h, t, 2.5) for u, h, t in ((0.0, 0.705, 0.02), (0.25, 0.70, 0.03), (0.6, 0.685, 0.03), (0.85, 0.655, 0.022), (0.915, 0.62, 0.012))], bone="bonnet", **PAINT)]


def hatch():
    steel = dict(bone="hatch", **PAINT)
    out = [loft("hatch-panel", [sec([0.47, 0.022, 0], 0.625, 0.022, 6), sec([0.745, 0.022, 0], 0.655, 0.022, 6)], **steel)]
    out.append(loft("hatch-lip", [sec([0.735, 0.018, 0], 0.022, 0.60, 6), sec([0.911, 0.144, 0], 0.022, 0.60, 6)], **steel))
    out.append(loft("hatch-header", [sec([0.0, 0.022, 0], 0.55, 0.02, 6), sec([0.06, 0.022, 0], 0.56, 0.02, 6)], **steel))
    for side, s in SIDES:
        out.append(loft(f"hatch-rail-{side}", [sec([0.04, 0.03, s * 0.525], 0.035, 0.018, 4), sec([0.50, 0.03, s * 0.595], 0.035, 0.018, 4)], bone="hatch", material="paint", role="body", tone=0.35))
    out.append(loft("hatch-glass", [sec([0.03, 0.004, 0], 0.548, 0.002, 8), sec([0.50, 0.004, 0], 0.618, 0.002, 8)], bone="hatch", material="glass", role="glazing", tone=0.85))
    out.append(box("plate-rear", [0.11, 0.012, 0.52], [0.8406, 0.0619, 0], radius=0.004, rotation=q_axis([0, 0, 1], 35.6), bone="hatch", material="plastic-trim", role="detail", tone=0.9))
    out.append(capsule("hatch-handle", [0.784, 0.024, -0.09], [0.784, 0.024, 0.09], 0.012, bone="hatch", **{**TRIM, "tone": 0.3}))
    return out


def exhaust():
    pipe = dict(material="exhaust-steel", role="detail", tone=0.35)
    out = [capsule("exhaust-front-pipe", [0.95, 0.26, -0.10], [0.20, 0.215, -0.10], 0.024, **pipe)]
    out.append(capsule("exhaust-resonator", [0.10, 0.235, -0.10], [-0.25, 0.235, -0.10], 0.045, **{**pipe, "tone": 0.4}))
    out.append(capsule("exhaust-mid-pipe", [-0.30, 0.215, -0.10], [-0.92, 0.215, -0.10], 0.024, **pipe))
    out.append(capsule("exhaust-axle-pipe", [-0.92, 0.215, -0.10], [-1.50, 0.33, -0.20], 0.024, **pipe))
    out.append(capsule("silencer", [-1.66, 0.315, -0.44], [-1.66, 0.315, 0.0], 0.085, lateralScale=0.8, **{**pipe, "tone": 0.4}))
    out.append(capsule("tail-pipe", [-1.70, 0.305, -0.30], [-1.925, 0.31, -0.30], 0.025, **{**pipe, "tone": 0.45}))
    return out


def wheel(tag, bone, s, vented):
    w = s * ZW
    axis = [0, 0, s]
    out = [revolve(f"{tag}-tyre", TYRE, axis, [0, 0, w], bone=bone, material="rubber", role="body", tone=0.1)]
    out.append(revolve(f"{tag}-rim", RIM, axis, [0, 0, w], bone=bone, material="alloy", role="detail", tone=0.55))
    out.append(revolve(f"{tag}-hub", [[0.02, 0.0], [0.02, 0.062], [0.064, 0.062], [0.074, 0.045], [0.078, 0.0]], axis, [0, 0, w], bone=bone, material="alloy", role="detail", tone=0.7))
    out.append(capsule(f"{tag}-spoke", [0, 0.045, s * (ZW + 0.058)], [0, 0.187, s * (ZW + 0.062)], 0.024, 0.019, bone=bone, material="alloy", role="detail", tone=0.8, lateralScale=0.6, repeat={"count": 5, "rotationAxis": [0, 0, 1], "rotationRad": r72()}))
    out.append(revolve(f"{tag}-disc", [[a - 0.006, r] for a, r in (DISC_VENTED if vented else DISC_SOLID)], axis, [0, 0, w], bone=bone, material="cast-iron", role="detail", tone=0.42))
    out.append(revolve(f"{tag}-disc-hat", [[-0.006, 0.0], [-0.006, 0.082], [0.03, 0.074], [0.03, 0.0]], axis, [0, 0, w], bone=bone, material="cast-iron", role="detail", tone=0.42))
    return out


def r72():
    return round(2 * math.pi / 5, 6)


def running_gear():
    out = []
    for side, s in SIDES:
        out += wheel(f"f{side}", "wheel-f", s, True) + wheel(f"r{side}", "wheel-r", s, False)
        out.append(box(f"caliper-f{side}", [0.11, 0.05, 0.06], [0.085, 0.118, s * (ZW - 0.006)], bone="axle-f", material="cast-iron", role="detail", tone=0.35, radius=0.012, rotation=q_axis([0, 0, 1], -36)))
        out.append(box(f"caliper-r{side}", [0.1, 0.045, 0.045], [-0.085, -0.118, s * (ZW - 0.006)], bone="axle-r", material="cast-iron", role="detail", tone=0.35, radius=0.012, rotation=q_axis([0, 0, 1], -36)))
        out.append(ellipsoid(f"knuckle-f{side}", [0.06, 0.09, 0.035], [0, 0, s * 0.645], bone="axle-f", **STEEL))
        out.append(capsule(f"strut-f{side}", [-0.005, 0.07, s * 0.635], [-0.04, 0.42, s * 0.585], 0.026, bone="axle-f", **{**STEEL, "tone": 0.3}))
        out.append(capsule(f"arm-f{side}", [0.0, -0.085, s * 0.64], [0.03, -0.06, s * 0.33], 0.02, bone="axle-f", **{**STEEL, "tone": 0.3}))
        out.append(ellipsoid(f"knuckle-r{side}", [0.055, 0.07, 0.03], [0, 0, s * 0.645], bone="axle-r", **STEEL))
        out.append(capsule(f"trailing-arm-r{side}", [0.42, 0.02, s * 0.56], [0.0, -0.03, s * 0.62], 0.03, bone="axle-r", **{**STEEL, "tone": 0.3}))
        out.append(capsule(f"damper-r{side}", [-0.06, -0.02, s * 0.63], [-0.11, 0.36, s * 0.60], 0.022, bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(capsule("twist-beam", [0.2, 0.0, -0.56], [0.2, 0.0, 0.56], 0.035, bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(box("subframe", [0.36, 0.05, 0.7], [0.05, -0.07, 0], radius=0.015, bone="axle-f", **{**STEEL, "tone": 0.3}))
    for side, s in SIDES:
        out.append(capsule(f"drive-shaft-{side}", [0, 0, s * 0.2], [0, 0, s * 0.62], 0.016, bone="axle-f", **{**STEEL, "tone": 0.3}))
    return out


def body():
    return shell() + glazing() + front_end() + seams() + interior() + bonnet() + hatch() + exhaust() + running_gear()


def ref(id, of, region, layer=None, faces=None):
    out = {"id": id, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    if faces:
        out["sectionFaces"] = faces
    return out


def shells(solids, region, faces=None):
    """🧅 Paint film as the body skin itself and the steel sheet as the same solid 0.08 mm inside it."""
    out = []
    for solid in solids:
        sheet = shrunk(solid, 0.00008)
        sheet.update(id=f"th-{solid['id']}-sheet", region=region, layer="sheet")
        paint = ref(f"th-{solid['id']}-paint", solid["id"], region, "paint", faces)
        if faces:
            sheet["sectionFaces"] = faces
        out += [sheet, paint]
    return out


def thermal():
    parts = {s["id"]: s for s in body()}
    panels = ["hull", "underbody", "sill-l", "sill-r", "waist"] + [f"{p}-pillar-{side}" for p in "abcd" for side in "lr"]
    flat = [f"{p}-pillar-{side}" for p in "abc" for side in "lr"]
    out = shells([parts[i] for i in panels if i not in ("waist", "hull") and i not in flat], "body-panels")
    out += shells([parts["waist"], parts["hull"]], "body-panels", ["+v"])
    out += shells([parts[f"{p}-pillar-l"] for p in "abc"], "body-panels", ["-w"]) + shells([parts[f"{p}-pillar-r"] for p in "abc"], "body-panels", ["+w"])
    out += shells([parts["roof"]], "roof", ["-v"])
    out += shells([parts["bonnet-skin"]], "bonnet", ["-v"])
    out += shells([parts[i] for i in ("hatch-panel", "hatch-header", "hatch-lip")], "tailgate", ["+v"])
    out.append(ref("th-windscreen", "windscreen", "glazing", "glass", ["-v"]))
    for side, face in (("l", "-w"), ("r", "+w")):
        out.append(ref(f"th-side-glass-front-{side}", f"side-glass-front-{side}", "glazing", "glass", [face]))
        out.append(ref(f"th-side-glass-rear-{side}", f"side-glass-rear-{side}", "glazing", "glass", [face]))
        out.append(ref(f"th-headlamp-{side}", f"headlamp-{side}", "glazing", "glass"))
    out.append(ref("th-hatch-glass", "hatch-glass", "glazing", "glass", ["+v"]))
    for i in ("seat-front-back-l", "seat-front-back-r", "seat-front-headrest-l", "seat-front-headrest-r", "seat-rear-back", "seat-rear-headrest-l", "seat-rear-headrest-r", "binnacle", "steering-wheel"):
        out.append({**parts[i], "id": f"th-{i}", "region": "interior", "layer": "foam"})
    foam = dict(material="interior-trim", role="detail", tone=0.3, region="interior", layer="foam")
    for side, s in SIDES:
        out.append(box(f"th-seat-front-cushion-{side}", [0.5, 0.12, 0.48], [0.07, 0.6, s * 0.36], radius=0.04, **foam))
    out.append(box("th-seat-rear-cushion", [0.48, 0.12, 1.22], [-0.74, 0.62, 0], radius=0.04, **foam))
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine-block", [0.3, 0.38, 0.38], [1.36, 0.56, 0.13], radius=0.03, **engine))
    out.append(capsule("th-gearbox", [1.33, 0.48, -0.14], [1.33, 0.48, -0.36], 0.13, **engine))
    out.append(capsule("th-exhaust-manifold", [1.17, 0.62, 0.02], [1.12, 0.34, -0.08], 0.045, **engine))
    for i in ("exhaust-front-pipe", "exhaust-resonator", "exhaust-mid-pipe", "exhaust-axle-pipe", "silencer", "tail-pipe"):
        out.append(ref(f"th-{i}", i, "exhaust-system", "pipe"))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.0225], [0.004, 0.0225], [0.004, 0.0]], [-1, 0, 0], [-1.948, 0.31, -0.30], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", layer="pipe", tags=["tailpipe"]))
    for w in ("fl", "fr", "rl", "rr"):
        out.append(ref(f"th-{w}-tyre", f"{w}-tyre", "tyres", "tread"))
        out.append(ref(f"th-{w}-disc", f"{w}-disc", "brakes"))
        out.append(ref(f"th-{w}-disc-hat", f"{w}-disc-hat", "brakes"))
        out.append(ref(f"th-caliper-{w}", f"caliper-{w}", "brakes"))
    return out


def member(solid, bone_member, part="member"):
    solid.update(member=bone_member, part=part)
    return solid


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for side, s in SIDES:
        m(box(f"sill-{side}", [1.7, 0.1, 0.09], [-0.075, 0.262, s * 0.745], radius=0.02, **frame))
        m(box(f"front-rail-{side}", [1.0, 0.1, 0.07], [1.36, 0.47, s * 0.42], radius=0.015, **frame))
        m(capsule(f"front-rail-kick-{side}", [0.88, 0.45, s * 0.42], [0.62, 0.27, s * 0.52], 0.04, **frame))
        m(box(f"floor-rail-{side}", [1.5, 0.06, 0.08], [-0.13, 0.25, s * 0.5], radius=0.012, **frame))
        m(capsule(f"rear-rail-kick-{side}", [-0.86, 0.26, s * 0.5], [-1.05, 0.5, s * 0.5], 0.04, **frame))
        m(box(f"rear-rail-{side}", [0.84, 0.08, 0.07], [-1.46, 0.52, s * 0.5], radius=0.015, **frame))
        m(capsule(f"hinge-pillar-{side}", [0.76, 0.3, s * 0.72], [0.8, 0.95, s * 0.72], 0.04, **frame))
        m(capsule(f"a-pillar-{side}", [0.8, 0.98, s * 0.74], [0.235, 1.43, s * 0.585], 0.03, **frame))
        m(capsule(f"b-pillar-{side}", [-0.3, 0.3, s * 0.745], [-0.3, 1.425, s * 0.585], 0.04, 0.028, **frame))
        m(capsule(f"c-pillar-{side}", [-1.0, 0.72, s * 0.74], [-1.24, 1.425, s * 0.585], 0.045, 0.028, **frame))
        m(capsule(f"d-pillar-{side}", [-1.74, 0.9, s * 0.7], [-1.455, 1.42, s * 0.575], 0.04, 0.03, **frame))
        m(capsule(f"roof-rail-{side}", [0.235, 1.43, s * 0.585], [-1.455, 1.42, s * 0.585], 0.03, **frame))
        m(capsule(f"wheelhouse-rear-{side}", [-1.0, 0.72, s * 0.62], [-1.6, 0.72, s * 0.62], 0.03, **frame))
        m(capsule(f"strut-tower-{side}", [1.11, 0.47, s * 0.47], [1.11, 0.74, s * 0.585], 0.05, **frame))
        m(capsule(f"upper-rail-{side}", [0.8, 0.9, s * 0.72], [1.6, 0.74, s * 0.66], 0.03, **frame))
    for name, x, y, w, h in (("bumper-beam-front", 1.87, 0.47, 1.1, 0.1), ("firewall-crossmember", 0.86, 0.45, 1.0, 0.1), ("seat-crossmember-front", 0.1, 0.25, 1.4, 0.06), ("seat-crossmember-rear", -0.45, 0.25, 1.4, 0.06), ("heel-kick", -0.86, 0.3, 1.4, 0.1), ("rear-crossmember", -1.55, 0.52, 1.0, 0.08), ("bumper-beam-rear", -1.88, 0.5, 1.2, 0.1)):
        m(box(name, [0.07, h, w], [x, y, 0], radius=0.015, **frame))
    for name, x, y, half in (("cowl-crossmember", 0.82, 0.98, 0.72), ("windscreen-header", 0.235, 1.43, 0.585), ("roof-bow-b", -0.3, 1.44, 0.585), ("roof-bow-c", -1.0, 1.44, 0.585), ("rear-header", -1.455, 1.42, 0.575)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.028, **frame))
    out.append(member(ellipsoid("com-chassis", [0.13, 0.13, 0.13], [0.28, 0.55, 0], material="cast-iron", role="detail", tone=0.5), "chassis", "mass"))
    b = dict(bone="bonnet", **frame)
    for side, s in SIDES:
        out.append(member(box(f"bonnet-frame-{side}", [0.86, 0.03, 0.05], [0.46, -0.02, s * 0.6], radius=0.01, **b), "bonnet"))
        out.append(member(ellipsoid(f"bonnet-hinge-{side}", [0.03, 0.03, 0.03], [0.0, -0.01, s * 0.6], **b), "bonnet", "joint"))
    for name, u in (("rear", 0.04), ("front", 0.88)):
        out.append(member(box(f"bonnet-frame-{name}", [0.05, 0.03, 1.2], [u, -0.02, 0], radius=0.01, **b), "bonnet"))
    h = dict(bone="hatch", **frame)
    for side, s in SIDES:
        out.append(member(capsule(f"hatch-frame-{side}", [0.03, 0.03, s * 0.52], [0.72, 0.03, s * 0.6], 0.022, **h), "hatch"))
        out.append(member(ellipsoid(f"hatch-hinge-{side}", [0.03, 0.03, 0.03], [0.0, 0.02, s * 0.42], **h), "hatch", "joint"))
    for name, u, half in (("top", 0.03, 0.52), ("waist", 0.5, 0.58), ("bottom", 0.72, 0.6)):
        out.append(member(capsule(f"hatch-frame-{name}", [u, 0.03, -half], [u, 0.03, half], 0.022, **h), "hatch"))
    for axle, front in (("axle-f", True), ("axle-r", False)):
        a = dict(bone=axle, **frame)
        for side, s in SIDES:
            out.append(member(capsule(f"{axle}-stub-{side}", [0, 0, s * 0.6], [0, 0, s * 0.7], 0.03, **a), axle))
            if front:
                out.append(member(capsule(f"{axle}-lower-arm-{side}", [0.0, -0.085, s * 0.64], [0.03, -0.06, s * 0.33], 0.02, **a), axle))
                out.append(member(capsule(f"{axle}-strut-{side}", [-0.005, 0.07, s * 0.635], [-0.04, 0.42, s * 0.585], 0.026, **a), axle))
                out.append(member(ellipsoid(f"{axle}-top-mount-{side}", [0.04, 0.04, 0.04], [-0.04, 0.44, s * 0.585], **a), axle, "joint"))
            else:
                out.append(member(capsule(f"{axle}-trailing-arm-{side}", [0.42, 0.02, s * 0.56], [0.0, -0.03, s * 0.62], 0.03, **a), axle))
                out.append(member(capsule(f"{axle}-spring-{side}", [0.08, -0.02, s * 0.5], [0.08, 0.2, s * 0.5], 0.05, **a), axle))
                out.append(member(ellipsoid(f"{axle}-bush-{side}", [0.04, 0.04, 0.04], [0.42, 0.02, s * 0.56], **a), axle, "joint"))
        if front:
            out.append({"id": "axle-f-subframe", "of": "subframe", "member": axle, "part": "member"})
        else:
            out.append(member(capsule("axle-r-twist-beam", [0.2, 0.0, -0.56], [0.2, 0.0, 0.56], 0.035, **a), axle))
        out.append(member(ellipsoid(f"{axle}-mass", [0.06, 0.06, 0.06], [0, 0, 0], **a), axle, "mass"))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        for part in ("tyre", "rim", "spoke"):
            out.append({"id": f"{tag}-{part}", "of": f"{tag}-{part}", "member": bone, "part": "member"})
        w = dict(bone=bone, material="alloy", role="detail", tone=0.6)
        out.append(member(ellipsoid(f"{tag}-bearing", [0.05, 0.05, 0.05], [0, 0, s * 0.7], **w), bone, "joint"))
        out.append(member(ellipsoid(f"{tag}-mass", [0.07, 0.07, 0.04], [0, 0, s * 0.72], **w), bone, "mass"))
    return out
