"""🚐 The 1:100 lists of the round 3 delivery van (smallest feature 15 mm): the same hollow panel construction with real door apertures, drawn for the scale: arches of 16 chords, one-piece corner posts, sill and step as one section, seats as base and back, wheels as tyre, dished wheel and hub; no handles, strips, wipers, plates or wheel nuts."""
import math

from r3_delivery_van_kit import QX, RA, SLIDER, T, W, XF, XR, X_B, X_BULK, X_HINGE, X_QUARTER, YA, Y_BELT, Y_CANT, Y_FLOOR, Y_SILL, ZW, Z_LEAF, arc, arch_points, box, capsule, ellipsoid, extrude, loft, member, nose, nose_patch, nose_top, pillar_x, plate, post, ref, revolve, roof, sec, side_z, sill_step, sweep, torus, wall, windscreen

SIDES = (("l", 1), ("r", -1))
PAINT = dict(material="paint", role="body", tone=0.65)
DOOR = dict(material="paint", role="body", tone=0.6)
TRIM = dict(material="plastic-trim", role="detail", tone=0.2)
SEAT = dict(material="plastic-trim", role="detail", tone=0.3)
STEEL = dict(material="cast-iron", role="detail", tone=0.4)
GLASS = dict(material="glass", role="glazing", tone=0.88)
ARCH = 16
ROOF = [(-2.64, 1.945, 0.035, 0.88), (-2.61, 1.945, 0.045, 0.90), (-2.55, 1.945, 0.045, 0.905), (-2.49, 1.90, 0.09, 0.905), (0.40, 1.90, 0.09, 0.905), (0.85, 1.89, 0.085, 0.89), (1.05, 1.865, 0.065, 0.865), (1.12, 1.845, 0.04, 0.845)]
TYRE = [[-0.085, 0.356], [0.085, 0.356], [0.11, 0.34], [0.1175, 0.295], [0.105, 0.23], [0.092, 0.206], [-0.092, 0.206], [-0.105, 0.23], [-0.1175, 0.295], [-0.11, 0.34], [-0.085, 0.356]]
WHEEL = [[0.042, 0.0], [0.042, 0.17], [-0.045, 0.17], [-0.06, 0.19], [-0.094, 0.19], [-0.094, 0.221], [-0.08, 0.221], [-0.08, 0.206], [-0.055, 0.206], [-0.04, 0.185], [0.03, 0.185], [0.05, 0.206], [0.08, 0.206], [0.08, 0.221], [0.094, 0.221], [0.094, 0.2], [0.058, 0.176], [0.058, 0.0]]


def quarter_outline(x_front):
    return [[-2.60, 0.36], [-2.32, 0.36], [-2.12, Y_SILL]] + arch_points(XR, ARCH) + [[x_front, Y_SILL], [x_front, Y_BELT], [-2.60, Y_BELT]]


def shell():
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
        out.append(plate(f"a-pillar-{side}", s, [(y, pillar_x(y) - 0.055, pillar_x(y) + 0.055) for y in (1.15, Y_CANT)], thickness=0.09, n=6, **PAINT))
        out.append(plate(f"door-glass-{side}", s, [(1.165, 0.365, 1.535), (1.805, 0.365, 0.975)], thickness=0.005, sunk=0.012, n=10, **GLASS))
        out.append(sill_step(f"sill-{side}", s, material="plastic-trim", role="body", tone=0.25))
        out.append(post(f"corner-post-{side}", s, (0.52, Y_BELT, 1.945), 0.0525, 0.0625, **PAINT))
        out.append(box(f"tail-lamp-{side}", [0.024, 0.42, 0.085], [-2.628, 1.40, s * 0.895], radius=0.01, material="glass", role="detail", tone=0.35))
    out.append(roof("roof", ROOF, **{**PAINT, "tone": 0.7}))
    out.append(nose("front-end", ARCH, **PAINT))
    out.append(nose("bumper-front", ARCH, x0=2.115, grow=0.015, top=0.56, material="plastic-trim", role="body", tone=0.2))
    out.append(loft("engine-undertray", [sec([x, (b + t) / 2, 0], h, (t - b) / 2, 6, QX) for x, b, t, h in [(1.24, 0.24, 0.80, 0.66), (2.25, 0.22, 0.80, 0.64)]], material="plastic-trim", role="body", tone=0.12))
    out.append(loft("bumper-rear", [sec([x, 0.44, 0], h, r, 8, QX) for x, r, h in [(-2.67, 0.08, 0.94), (-2.63, 0.1, 0.99), (-2.36, 0.1, 0.995)]], material="plastic-trim", role="body", tone=0.2))
    floor = [[0.95, 1.25], [0.95, -1.15], [0.67, -1.15], [0.67, -2.05], [0.95, -2.05], [0.95, -2.61], [-0.95, -2.61], [-0.95, -2.05], [-0.67, -2.05], [-0.67, -1.15], [-0.95, -1.15], [-0.95, 1.25]]
    out.append(extrude("floor", floor, "zx", 0.44, Y_FLOOR, material="paint", role="body", tone=0.45))
    low = math.degrees(math.asin((Y_SILL - YA) / RA))
    for side, s in SIDES:
        out.append(extrude(f"wheelhouse-{side}", arc(XR, YA, 0.465, 180 - low, low, ARCH) + arc(XR, YA, RA, low, 180 - low, ARCH), "xy", s * 0.66, s * 0.95, material="paint", role="body", tone=0.4))
        out.append(extrude(f"wheelhouse-wall-{side}", arc(XR, YA, 0.465, 180 - low, low, ARCH), "xy", s * 0.66, s * 0.69, material="paint", role="body", tone=0.4))
    out.append(extrude("bulkhead", [[Y_FLOOR, -0.945], [Y_BELT, -0.945], [1.86, -0.87], [1.86, 0.87], [Y_BELT, 0.945], [Y_FLOOR, 0.945]], "yz", 0.29, X_BULK, material="paint", role="body", tone=0.55))
    out.append(box("rear-threshold", [0.11, 0.16, 1.72], [-2.555, 0.48, 0], radius=0.01, material="paint", role="body", tone=0.5))
    return out


def cab():
    out = [windscreen("windscreen", [-0.895, -0.845, -0.6, -0.3, 0.0, 0.3, 0.6, 0.845, 0.895])]
    out.append(loft("cowl", [sec([1.775 - sweep(z), 1.205, z], 0.05, 0.016, 3) for z in (-0.89, -0.45, 0.0, 0.45, 0.89)], material="plastic-trim", role="detail", tone=0.15))
    out.append(box("grille", [0.04, 0.2, 0.9], [2.646, 0.80, 0], radius=0.015, material="engine-metal", role="detail", tone=0.15))
    out.append(loft("bonnet-skin", [sec([x, nose_top(x) + 0.001, 0], 0.0085, h, 3) for x, h in ((1.80, 0.86), (2.2, 0.855), (2.54, 0.77))], **{**PAINT, "tone": 0.62}))
    for side, s in SIDES:
        lamp = []
        for x, ry in ((2.30, 0.04), (2.40, 0.085), (2.52, 0.085), (2.60, 0.065), (2.625, 0.04)):
            at, _ = nose_patch(x, 0.86, s, 0.006)
            lamp.append(sec(at, 0.02, ry, 4))
        out.append(loft(f"headlamp-{side}", lamp, material="glass", role="glazing", tone=0.9))
        out.append(capsule(f"mirror-arm-{side}", [1.36, 1.26, s * 0.975], [1.40, 1.33, s * 1.03], 0.022, **{**TRIM, "tone": 0.25}))
        out.append(ellipsoid(f"mirror-{side}", [0.06, 0.135, 0.095], [1.415, 1.385, s * 1.04], **{**TRIM, "tone": 0.25}))
    out.append(loft("dashboard", [sec([1.40, 1.215, z], 0.2, 0.05, 3) for z in (-0.90, 0.0, 0.90)], **{**SEAT, "tone": 0.2}))
    tilt = math.radians(38)
    out.append(torus("steering-wheel", [-math.cos(tilt), math.sin(tilt), 0], [1.02, 1.21, 0.5], 0.185, 0.016, 6, **{**SEAT, "tone": 0.15}))
    for name, zc, half in (("driver", 0.5, 0.24), ("bench", -0.37, 0.43)):
        out.append(box(f"seat-base-{name}", [0.5, 0.4, 2 * half], [0.69, 0.76, zc], radius=0.03, **SEAT))
        out.append(loft(f"seat-back-{name}", [sec([0.50, 0.93, zc], 0.06, half, 4), sec([0.395, 1.56, zc], 0.045, half - 0.03, 4)], **SEAT))
    for name, zc in (("driver", 0.5), ("bench-outer", -0.59), ("bench-inner", -0.15)):
        out.append(ellipsoid(f"headrest-{name}", [0.05, 0.095, 0.125], [0.385, 1.66, zc], **SEAT))
    return out


def doors():
    b = dict(bone="slider", origin=SLIDER)
    out = [wall("slider-panel", -1, [[X_QUARTER, 0.40], [X_B, 0.40], [X_B, Y_BELT], [X_QUARTER, Y_BELT]], proud=0.015, **b, **DOOR)]
    out.append(plate("slider-upper", -1, [(1.16, X_QUARTER, X_B), (1.84, X_QUARTER, X_B)], proud=0.015, **b, **DOOR))
    for side, s in SIDES:
        out.append(box(f"rear-leaf-{side}", [1.35, Z_LEAF, T], [0.675, Z_LEAF / 2, -s * T / 2], radius=0.012, bone=f"rear-door-{side}", **DOOR))
    return out


def underbody():
    out = []
    for side, s in SIDES:
        out.append(box(f"chassis-rail-{side}", [3.85, 0.12, 0.08], [-0.63, 0.38, s * 0.5], **STEEL))
    for k, x in enumerate((-2.45, -0.3, 1.15)):
        out.append(box(f"crossmember-{k + 1}", [0.08, 0.08, 0.92], [x, 0.40, 0], **STEEL))
    out.append(box("fuel-tank", [0.85, 0.14, 0.5], [0.0, 0.37, 0.2], radius=0.05, material="plastic-trim", role="detail", tone=0.15))
    out.append(revolve("spare-wheel", [[0.0, 0.0], [0.0, 0.33], [0.17, 0.33], [0.17, 0.0]], [0, 1, 0], [-2.25, 0.27, 0.12], material="rubber", role="detail", tone=0.1))
    out.append(capsule("tail-pipe", [-2.36, 0.30, -0.31], [-2.60, 0.30, -0.31], 0.03, material="exhaust-steel", role="detail", tone=0.4))
    return out


def wheel(tag, bone, s):
    w = s * ZW
    axis = [0, 0, s]
    out = [revolve(f"{tag}-tyre", TYRE, axis, [0, 0, w], bone=bone, material="rubber", role="body", tone=0.1)]
    out.append(revolve(f"{tag}-wheel", WHEEL, axis, [0, 0, w], bone=bone, material="steel-rim", role="detail", tone=0.62))
    out.append(revolve(f"{tag}-hub", [[0.0, 0.0], [0.0, 0.085], [0.07, 0.085], [0.095, 0.05], [0.1, 0.0]], axis, [0, 0, w], bone=bone, material="steel-rim", role="detail", tone=0.72))
    out.append(revolve(f"{tag}-brake-disc", [[-0.06, 0.0], [-0.06, 0.15], [-0.03, 0.15], [-0.03, 0.09], [-0.005, 0.082], [-0.005, 0.0]], axis, [0, 0, w], bone=bone, material="cast-iron", role="detail", tone=0.42))
    return out


def running_gear():
    out = []
    for side, s in SIDES:
        out += wheel(f"f{side}", "wheel-f", s) + wheel(f"r{side}", "wheel-r", s)
        out.append(capsule(f"lower-arm-f{side}", [0.0, -0.1, s * 0.72], [0.04, -0.075, s * 0.36], 0.024, bone="axle-f", **{**STEEL, "tone": 0.3}))
        out.append(capsule(f"drive-shaft-{side}", [0, 0, s * 0.22], [0, 0, s * 0.71], 0.018, bone="axle-f", **{**STEEL, "tone": 0.3}))
        out.append(loft(f"leaf-spring-{side}", [sec([x, y, s * 0.56], 0.035, 0.014, 6, QX) for x, y in ((-0.66, 0.085), (0.0, -0.07), (0.66, 0.085))], bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(capsule("rear-axle-beam", [0, -0.01, -0.74], [0, -0.01, 0.74], 0.042, bone="axle-r", **{**STEEL, "tone": 0.3}))
    out.append(box("subframe", [0.5, 0.05, 0.78], [0.02, -0.105, 0], bone="axle-f", **{**STEEL, "tone": 0.3}))
    return out


def body():
    return shell() + cab() + doors() + underbody() + running_gear()


INWARD = {"l": "-w", "r": "+w"}
PANELS = {
    "side-l": ["-w"], "side-upper-l": ["-w"], "quarter-r": ["+w"], "quarter-upper-r": ["+w"], "b-post-r": ["+w"], "b-post-upper-r": ["+w"], "slider-lintel": ["+w"],
    "front-door-l": ["-w"], "front-door-r": ["+w"], "cant-rail-l": ["-w"], "cant-rail-r": ["+w"], "a-pillar-l": ["-w"], "a-pillar-r": ["+w"], "corner-post-l": ["-w", "+u"], "corner-post-r": ["+w", "+u"],
    "front-end": ["-u"], "floor": ["+v"], "wheelhouse-l": ["+v"], "wheelhouse-r": ["+v"], "wheelhouse-wall-l": ["-w"], "wheelhouse-wall-r": ["+w"], "rear-threshold": ["+v", "+u"],
}


def thermal():
    out = [ref(f"th-{i}", i, "body-panels", None, faces) for i, faces in PANELS.items()]
    out.append(ref("th-roof", "roof", "roof", None, ["-v"]))
    out.append(ref("th-bonnet-skin", "bonnet-skin", "bonnet", None, ["-v"]))
    out += [ref("th-slider-panel", "slider-panel", "slider-door", None, ["+w"]), ref("th-slider-upper", "slider-upper", "slider-door", None, ["+w"])]
    out += [ref("th-rear-leaf-l", "rear-leaf-l", "rear-doors", None, ["-w"]), ref("th-rear-leaf-r", "rear-leaf-r", "rear-doors", None, ["+w"])]
    out.append(ref("th-windscreen", "windscreen", "cab-glazing", None, ["-u", "-v"]))
    for side in "lr":
        out.append(ref(f"th-door-glass-{side}", f"door-glass-{side}", "cab-glazing", None, [INWARD[side]]))
    engine = dict(material="engine-metal", role="detail", tone=0.2, region="engine-bay")
    out.append(box("th-engine", [0.42, 0.4, 0.5], [2.12, 0.71, 0.14], radius=0.03, **engine))
    out.append(capsule("th-gearbox", [2.10, 0.62, -0.20], [2.10, 0.62, -0.40], 0.11, **engine))
    pipe = dict(material="exhaust-steel", role="detail", tone=0.35, region="exhaust-system")
    z = -0.31
    out.append(capsule("th-downpipe", [1.92, 0.62, -0.1], [1.50, 0.33, z], 0.032, **pipe))
    out.append(capsule("th-dpf-can", [1.42, 0.33, z], [1.05, 0.33, z], 0.08, **pipe))
    out.append(capsule("th-pipe", [0.95, 0.33, z], [-0.72, 0.33, z], 0.03, **pipe))
    out.append(capsule("th-silencer", [-0.80, 0.31, z], [-1.22, 0.31, z], 0.08, **pipe))
    out.append(capsule("th-axle-pipe", [-1.25, 0.33, z], [-1.64, 0.44, z], 0.03, **pipe))
    out.append(capsule("th-rear-pipe", [-1.64, 0.44, z], [-2.36, 0.30, z], 0.03, **pipe))
    out.append(capsule("th-tail-pipe", [-2.36, 0.30, z], [-2.60, 0.30, z], 0.03, **pipe))
    out.append(revolve("th-tailpipe-outlet", [[0.0, 0.0], [0.0, 0.03], [0.015, 0.03], [0.015, 0.0]], [-1, 0, 0], [-2.618, 0.30, z], material="exhaust-steel", role="detail", tone=0.5, region="exhaust-system", tags=["tailpipe"]))
    for w in ("fl", "fr", "rl", "rr"):
        out.append(ref(f"th-{w}-tyre", f"{w}-tyre", "tyres"))
        out.append(ref(f"th-{w}-brake-disc", f"{w}-brake-disc", "brakes"))
    return out


def structure():
    frame = dict(material="cast-iron", role="detail", tone=0.5)
    out = []
    m = lambda s: out.append(member(s, "chassis"))
    for i in ("chassis-rail-l", "chassis-rail-r", "crossmember-1", "crossmember-2", "crossmember-3"):
        out.append({"id": f"st-{i}", "of": i, "member": "chassis", "part": "member"})
    for side, s in SIDES:
        m(box(f"sill-{side}", [2.47, 0.12, 0.09], [0.045, 0.37, s * 0.93], **frame))
        m(box(f"front-rail-{side}", [1.45, 0.12, 0.08], [1.82, 0.50, s * 0.46], **frame))
        m(capsule(f"a-pillar-{side}", [1.25, 0.36, s * 0.94], [pillar_x(1.17), 1.17, s * 0.945], 0.04, **frame))
        m(capsule(f"a-pillar-upper-{side}", [pillar_x(1.17), 1.17, s * 0.945], [pillar_x(1.88), 1.88, s * (side_z(1.88) - 0.045)], 0.032, **frame))
        m(capsule(f"b-pillar-{side}", [0.30, 0.36, s * 0.945], [0.30, 1.88, s * (side_z(1.88) - 0.035)], 0.04, 0.03, **frame))
        m(capsule(f"c-pillar-{side}", [-0.84, 0.36, s * 0.945], [-0.84, 1.88, s * (side_z(1.88) - 0.035)], 0.04, 0.03, **frame))
        m(capsule(f"d-pillar-{side}", [-2.57, 0.52, s * 0.925], [-2.57, 1.90, s * (side_z(1.90) - 0.06)], 0.045, 0.035, **frame))
        m(capsule(f"cant-rail-{side}", [-2.57, 1.88, s * (side_z(1.88) - 0.04)], [pillar_x(1.88), 1.88, s * (side_z(1.88) - 0.045)], 0.032, **frame))
        m(capsule(f"strut-tower-{side}", [1.66, 0.52, s * 0.52], [1.64, 0.98, s * 0.64], 0.055, **frame))
    for name, x, y, w, h in (("bumper-beam-front", 2.58, 0.52, 1.3, 0.12), ("bumper-beam-rear", -2.58, 0.46, 1.5, 0.1)):
        m(box(name, [0.07, h, w], [x, y, 0], **frame))
    for name, x, y, half in (("cowl-crossmember", 1.56, 1.14, 0.92), ("windscreen-header", 1.0, 1.86, 0.84), ("roof-bow-b", 0.30, 1.90, 0.86), ("roof-bow-c", -0.84, 1.90, 0.86), ("rear-header", -2.57, 1.92, 0.85)):
        m(capsule(name, [x, y, -half], [x, y, half], 0.03, **frame))
    out.append(member(ellipsoid("com-chassis", [0.15, 0.15, 0.15], [0.48, 0.85, 0], **frame), "chassis", "mass"))
    b = dict(bone="slider", **frame)
    wz, lean = -(W - 0.02), -(side_z(1.80) - 0.02)
    for name, u in (("rear", 0.04), ("front", 1.01)):
        out.append(member(capsule(f"slider-stile-{name}", [u, 0.08, wz], [u, 0.80, wz], 0.022, **b), "slider"))
        out.append(member(capsule(f"slider-stile-upper-{name}", [u, 0.80, wz], [u, 1.44, lean], 0.022, **b), "slider"))
    for name, v, w in (("bottom", 0.08, wz), ("waist", 0.80, wz), ("top", 1.44, lean)):
        out.append(member(capsule(f"slider-rail-{name}", [0.04, v, w], [1.01, v, w], 0.022, **b), "slider"))
    out.append(member(ellipsoid("slider-roller", [0.03, 0.03, 0.02], [0.03, 0.745, -(W - 0.01)], **b), "slider", "joint"))
    for side, s in SIDES:
        bone = f"rear-door-{side}"
        d = dict(bone=bone, **frame)
        w = -s * 0.0225
        for name, p0, p1 in (("hinge-stile", (0.04, 0.04), (1.31, 0.04)), ("lock-stile", (0.04, 0.82), (1.31, 0.82)), ("bottom-rail", (0.04, 0.04), (0.04, 0.82)), ("top-rail", (1.31, 0.04), (1.31, 0.82))):
            out.append(member(capsule(f"{bone}-{name}", [p0[0], p0[1], w], [p1[0], p1[1], w], 0.02, **d), bone))
        out.append(member(ellipsoid(f"{bone}-hinge", [0.04, 0.02, 0.02], [0.675, 0.0, w], **d), bone, "joint"))
    a = dict(bone="axle-f", **frame)
    out.append({"id": "st-subframe", "of": "subframe", "member": "axle-f", "part": "member"})
    for side, s in SIDES:
        out.append({"id": f"st-lower-arm-f{side}", "of": f"lower-arm-f{side}", "member": "axle-f", "part": "member"})
        out.append(member(capsule(f"axle-f-strut-{side}", [-0.01, 0.08, s * 0.72], [-0.06, 0.60, s * 0.64], 0.03, **a), "axle-f"))
        out.append({"id": f"st-leaf-spring-{side}", "of": f"leaf-spring-{side}", "member": "axle-r", "part": "member"})
    out.append(member(ellipsoid("axle-f-mass", [0.07, 0.07, 0.07], [0, 0, 0], **a), "axle-f", "mass"))
    out.append({"id": "st-rear-axle-beam", "of": "rear-axle-beam", "member": "axle-r", "part": "member"})
    out.append(member(ellipsoid("axle-r-mass", [0.07, 0.07, 0.07], [0, -0.01, 0], bone="axle-r", **frame), "axle-r", "mass"))
    for tag, bone, s in (("fl", "wheel-f", 1), ("fr", "wheel-f", -1), ("rl", "wheel-r", 1), ("rr", "wheel-r", -1)):
        for part in ("tyre", "wheel"):
            out.append({"id": f"st-{tag}-{part}", "of": f"{tag}-{part}", "member": bone, "part": "member"})
        out.append(member(ellipsoid(f"{tag}-bearing", [0.05, 0.05, 0.05], [0, 0, s * 0.78], bone=bone, material="steel-rim", role="detail", tone=0.6), bone, "joint"))
    return out


def decisions():
    return [
        {"en": "Body: the hollow panel van of 1:50 drawn for a 15 mm feature: side slabs with arches of 16 chords, leaning upper panels, doors in real apertures on their bones, one-piece rear corner posts, sill and door step as one section, front end with bumper, bonnet skin, grille and wrapped headlamps, swept windscreen, door glass, mirrors; seats are a base and a back with headrests, the steering wheel is a plain ring; wheels are tyre, one dished wheel, hub and brake disc.", "de": "Körper: der hohle Kastenwagen von 1:50, gezeichnet für 15 mm Mindestmaß: Seitenplatten mit Radausschnitten aus 16 Sehnen, geneigte obere Bleche, Türen in echten Öffnungen an ihren Knochen, einteilige hintere Eckpfosten, Schweller und Trittstufe als ein Profil, Vorderwagen mit Stoßfänger, Haubenblech, Grill und umlaufenden Scheinwerfern, gepfeilte Windschutzscheibe, Türscheiben, Spiegel; Sitze sind Sockel und Lehne mit Kopfstützen, das Lenkrad ein glatter Ring; Räder sind Reifen, eine geschüsselte Felge, Nabe und Bremsscheibe."},
        {"en": "Merged or omitted: paint film and steel sheet are one shell per panel; handles, rub strips, wipers, number plates, grille bars, hinges, the quarter-light bar, wheel nuts and holes, calipers, dampers and the steering column are left out (below 15 mm or 0.15 mm on paper).", "de": "Zusammengefasst oder weggelassen: Lackfilm und Stahlblech sind eine Schale je Blech; Griffe, Scheuerleisten, Wischer, Kennzeichen, Grillstäbe, Scharniere, der Dreiecksfenstersteg, Radmuttern und Felgenlöcher, Bremssättel, Dämpfer und die Lenksäule entfallen (unter 15 mm bzw. 0,15 mm auf dem Papier)."},
        {"en": "Thermal and structure: engine and gearbox are two hidden solids, the exhaust run keeps filter, silencer, the pipe over the axle and the outlet patch below the rear bumper; the frame keeps rails with three crossmembers, sills, front rails, the four pillar pairs, cant rails, two roof bows, headers, strut towers, bumper beams, the door frames, subframe, struts, beam axle on leaf springs and the wheels.", "de": "Thermik und Tragwerk: Motor und Getriebe sind zwei verborgene Körper, der Abgasstrang behält Filter, Schalldämpfer, das Rohr über der Achse und die Auslassfläche unter dem hinteren Stoßfänger; der Rahmen behält Längsträger mit drei Querträgern, Schweller, vordere Längsträger, die vier Säulenpaare, Dachlängsträger, zwei Dachspriegel, Querträger, Federbeindome, Stoßfängerträger, die Türrahmen, Hilfsrahmen, Federbeine, Starrachse an Blattfedern und die Räder."},
    ]
