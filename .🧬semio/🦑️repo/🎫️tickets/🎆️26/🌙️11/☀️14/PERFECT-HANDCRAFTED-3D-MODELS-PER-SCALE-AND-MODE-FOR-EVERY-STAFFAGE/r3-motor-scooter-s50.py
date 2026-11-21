"""🛵 Round 3, 1:50 of vehicles/motor-scooter, authored for 7.5 mm features: the steel monocoque (floor, leg shield with nose, wings and glovebox core, rear body with side cowls), the steering assembly (headset, lamp, grips, levers, mirrors, column, single-sided fork leg, trailing link, coil-over shock, wide mudguard), cast five-spoke wheels with tyres, disc and drum, the swinging engine unit with belt case, cylinder, air box, exhaust and rear shock, the stepped seat and the centre stand; every section is stated by hand."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-motor-scooter-parts")
kit = pt.kit
AF, AR, H, PIVOT, HINGE = pt.AF, pt.AR, pt.H, pt.PIVOT, pt.HINGE
RG, TONE = pt.RG, pt.TONE
PAINT, RUBBER, ALLOY, VINYL, IRON, EXHAUST, ENGINE, GLASS = pt.PAINT, pt.RUBBER, pt.ALLOY, pt.VINYL, pt.IRON, pt.EXHAUST, pt.ENGINE, pt.GLASS
sp, ax = kit.steer_pt, kit.axis_x
PENS = {"body": 0.35, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.5}
S18, C18 = math.sin(math.radians(18)), math.cos(math.radians(18))

FLOOR = [(-0.165, 0.185, 0.3, 0.185, 5), (-0.05, 0.178, 0.302, 0.2, 5), (0.2, 0.178, 0.302, 0.208, 5), (0.31, 0.19, 0.304, 0.205, 5), (0.36, 0.225, 0.312, 0.185, 4), (0.385, 0.262, 0.318, 0.15, 3)]
TAIL = [(-0.022, 0.297, 0.312, 0.112, 2.3), (-0.04, 0.297, 0.43, 0.12, 2.4), (-0.07, 0.297, 0.565, 0.13, 2.5), (-0.11, 0.299, 0.7, 0.148, 2.7), (-0.15, 0.31, 0.718, 0.165, 3), (-0.2, 0.372, 0.723, 0.182, 3), (-0.3, 0.39, 0.725, 0.197, 3), (-0.44, 0.425, 0.725, 0.21, 3), (-0.56, 0.47, 0.725, 0.215, 3), (-0.67, 0.5, 0.72, 0.21, 3), (-0.8, 0.512, 0.71, 0.185, 3), (-0.9, 0.538, 0.695, 0.14, 2.6), (-0.948, 0.575, 0.668, 0.085, 2.4)]
CHEEK = [(-0.235, 0.42, 0.5, 0.01), (-0.29, 0.378, 0.54, 0.024), (-0.36, 0.354, 0.572, 0.035), (-0.44, 0.345, 0.592, 0.042), (-0.53, 0.345, 0.602, 0.045), (-0.62, 0.356, 0.605, 0.045), (-0.71, 0.38, 0.602, 0.041), (-0.79, 0.415, 0.594, 0.034), (-0.86, 0.462, 0.58, 0.023), (-0.905, 0.51, 0.562, 0.011)]
CORE = [(0.288, 0.34, 0.39, 0.075, 4), (0.4, 0.39, 0.415, 0.085, 4), (0.5, 0.375, 0.424, 0.118, 4), (0.62, 0.34, 0.408, 0.13, 4), (0.76, 0.283, 0.364, 0.122, 4), (0.86, 0.254, 0.328, 0.088, 3.5), (0.94, 0.25, 0.312, 0.062, 3)]
NOSE = [(0.975, 0.058, 0.046), (0.8, 0.08, 0.055), (0.66, 0.078, 0.055), (0.56, 0.062, 0.05), (0.515, 0.04, 0.034)]
WING = [(0.9, 0.325, 0.045), (0.8, 0.372, 0.085), (0.68, 0.425, 0.103), (0.55, 0.452, 0.105), (0.43, 0.44, 0.105), (0.34, 0.412, 0.1), (0.288, 0.378, 0.094)]
SEAT = [(-0.135, 0.725, 0.775, 0.085, 2.6), (-0.2, 0.715, 0.795, 0.125, 3), (-0.32, 0.712, 0.8, 0.16, 3), (-0.44, 0.714, 0.794, 0.17, 3), (-0.52, 0.718, 0.812, 0.166, 3), (-0.6, 0.722, 0.828, 0.158, 3), (-0.7, 0.727, 0.83, 0.142, 3), (-0.77, 0.732, 0.824, 0.122, 2.8), (-0.815, 0.742, 0.8, 0.078, 2.4)]
HEADSET = [(-0.172, 0.29, 1.008, 0.03, 0.027), (-0.135, 0.293, 1.01, 0.052, 0.042), (-0.075, 0.302, 1.012, 0.09, 0.057), (0.0, 0.305, 1.012, 0.1, 0.062), (0.075, 0.302, 1.012, 0.09, 0.057), (0.135, 0.293, 1.01, 0.052, 0.042), (0.172, 0.29, 1.008, 0.03, 0.027)]
CVT = [(-0.272, 0.222, 0.3, 0.03), (-0.33, 0.183, 0.345, 0.045), (-0.42, 0.176, 0.35, 0.045), (-0.55, 0.166, 0.33, 0.045), (-0.67, 0.152, 0.32, 0.045), (-0.75, 0.165, 0.3, 0.04), (-0.787, 0.2, 0.265, 0.022)]
GUARD = [[-0.098, 0.216], [-0.098, 0.246], [-0.078, 0.262], [0.0, 0.267], [0.078, 0.262], [0.098, 0.246], [0.098, 0.216], [0.093, 0.216], [0.093, 0.243], [0.075, 0.257], [0.0, 0.262], [-0.075, 0.257], [-0.093, 0.243], [-0.093, 0.216], [-0.098, 0.216]]
LEG = [((0.5306, 0.4698, 0.0), 0.019), ((0.565, 0.415, -0.058), 0.018), ((0.602, 0.374, pt.FORK_Z), 0.017), ((0.665, 0.315, pt.FORK_Z), 0.017), ((0.752, 0.26, pt.FORK_Z), 0.019)]


def wheel(S, name, c, width, face, drum):
    """🛞 One wheel: tyre, rim barrel with flanges, the cast five-spoke centre on the side away from its single-sided mount (face = +1 left, -1 right) and the hub (a brake drum at the rear)."""
    bone = "wheel-" + name
    h = width / 2
    S.put(S.rev("tyre-" + name, bone, pt.tyre(width), c + (0.0,), [0, 0, 1], RUBBER, TONE["tyre"]), RG["tyre"])
    pt.ring2d(S, "tyre-" + name, bone, c, 0.23, 0.166, -h, h, "body", TONE["tyre"], RG["tyre"], RUBBER)
    S.put(S.rev("rim-" + name, bone, pt.RIM, c + (0.0,), [0, 0, 1], ALLOY, TONE["rim"]), RG["rim"])
    S.struct_of("rim-" + name, bone)
    pt.ring2d(S, "rim-" + name, bone, c, 0.166, 0.134, -0.049, 0.049, "body", TONE["rim"], RG["rim"], ALLOY)
    z0, z1 = sorted((face * 0.008, face * 0.022))
    holes = [pt.window(c, k) for k in range(5)]
    S.put(S.prism("spokes-" + name, bone, pt.arc(c, 0.139, 0, 360, 48)[:-1], z0, z1, ALLOY, TONE["rim"] - 0.08, holes=holes), RG["rim"])
    S.struct_of("spokes-" + name, bone)
    pt.ring2d(S, "spokes-" + name, bone, c, 0.139, None, z0, z1, "detail", TONE["rim"] - 0.14, RG["rim"], ALLOY, holes=holes, views=("elevation",))
    if drum:
        a, b = sorted((-face * 0.008, -face * 0.052))
        hub = S.rev("hub-" + name, bone, [[a, 0.0], [a, 0.079], [a + 0.006, 0.083], [b - 0.006, 0.083], [b, 0.079], [b, 0.0]], c + (0.0,), [0, 0, 1], IRON, TONE["iron"])
        S.put(hub, RG["brake"])
        pt.ring2d(S, "hub-" + name, bone, c, 0.083, None, a, b, "detail", TONE["iron"], RG["brake"], IRON)
        cap = S.rev("cap-" + name, bone, [[-0.04, 0.0], [-0.034, 0.03], [-0.022, 0.036], [-0.022, 0.0]], c + (0.0,), [0, 0, 1], ALLOY, TONE["rim"])
        S.put(cap, RG["rim"])
        pt.ring2d(S, "cap-" + name, bone, c, 0.036, None, -0.04, -0.022, "detail", TONE["rim"], RG["rim"], ALLOY, views=("elevation",))
    else:
        a, b = sorted((-face * 0.062, face * 0.03))
        hub = S.rev("hub-" + name, bone, [[a, 0.0], [a, 0.034], [b - 0.008, 0.034], [b, 0.026], [b, 0.0]], c + (0.0,), [0, 0, 1], ALLOY, TONE["rim"])
        S.put(hub, RG["rim"])
        pt.ring2d(S, "hub-" + name, bone, c, 0.034, None, a, b, "detail", TONE["rim"], RG["rim"], ALLOY)
    S.struct_of("hub-" + name, bone)
    S.struct(S.cyl("axle-" + name, bone, c + (-0.07,), c + (0.07,), 0.0085, IRON, 0.3, role="detail"), bone, "joint")


def monocoque(S):
    """🧱 The frame bone: floor, leg shield, rear body with side cowls, lamps, plate holder, grab rail and the engine hanger."""
    S.put(S.lofted("floor", "frame", FLOOR, PAINT, TONE["paint"]), RG["panels"])
    for k, z in enumerate((-0.15, -0.1, -0.05, 0.05, 0.1, 0.15)):
        rail = S.box(f"floor-rail-{k + 1}", "frame", [0.4, 0.006, 0.014], [0.08, 0.304, z], RUBBER, TONE["dark"], role="texture", corner=0.002, texture=True)
        S.put(rail, RG["panels"])
        S.stroke("plan", f"floor-rail-{k + 1}", "frame", "texture", TONE["dark"], RG["panels"], RUBBER, [[-0.12, z], [0.28, z]], [0.28, 0.33, z])
    S.put(S.lofted("shield-core", "frame", CORE, PAINT, TONE["paint"], along="y"), RG["panels"])
    S.put(S.tube("shield-nose", "frame", [([ax(y) + 0.012, y, 0.0], rz, d, 2.6) for y, rz, d in NOSE], PAINT, TONE["paint"]), RG["panels"])
    for side, sign in (("l", 1), ("r", -1)):
        pts = [([x - hs * S18, y, sign * (0.03 + hs * C18)], hs, 0.011) for y, x, hs in WING]
        S.put(S.tube("shield-wing-" + side, "frame", pts, PAINT, TONE["paint"], hint=(-S18, 0.0, sign * C18)), RG["panels"])
        S.put(S.ball("turn-f" + side, "frame", [0.008, 0.05, 0.015], [0.405, 0.6, sign * 0.175], GLASS, TONE["glass"], role="glazing", rotation=kit.rot_w(12)), RG["lamp"])
        S.put(S.lofted("cheek-" + side, "frame", [(x, lo, hi, t, 2.4, sign * 0.188) for x, lo, hi, t in CHEEK], PAINT, TONE["paint"] + 0.04), RG["panels"])
        S.put(S.ball("turn-r" + side, "frame", [0.038, 0.015, 0.012], [-0.84, 0.512, sign * 0.212], GLASS, TONE["glass"], role="glazing"), RG["lamp"])
    S.put(S.tube("tie", "frame", [([ax(y) + 0.067 + dx, y, 0.0], 0.016, 0.006) for y, dx in ((0.9, -0.008), (0.8, 0.004), (0.68, 0.005), (0.58, 0.0))], ALLOY, TONE["trim"], role="detail"), RG["panels"])
    S.put(S.lofted("tail", "frame", TAIL, PAINT, TONE["paint"]), RG["panels"])
    S.put(S.box("tail-lamp", "frame", [0.034, 0.07, 0.13], [-0.95, 0.622, 0.0], GLASS, 0.3, role="glazing", corner=0.012), RG["lamp"])
    S.put(S.box("plate-holder", "frame", [0.014, 0.2, 0.2], [-0.932, 0.49, 0.0], PAINT, TONE["dark"], corner=0.005, rotation=kit.rot_w(-8)), RG["panels"])
    rail = [[-0.6, 0.738, 0.168], [-0.84, 0.762, 0.168], [-0.935, 0.766, 0.09], [-0.935, 0.766, -0.09], [-0.84, 0.762, -0.168], [-0.6, 0.738, -0.168]]
    S.put(S.tube("grab-rail", "frame", [(p, 0.009, 0.009) for p in kit.bent(rail, [0.05, 0.06, 0.06, 0.05], 4)], ALLOY, TONE["trim"], role="detail"), RG["panels"])
    for side, sign in (("l", 1), ("r", -1)):
        S.put(S.cap("rail-post-" + side, "frame", [-0.62, 0.7, sign * 0.165], [-0.62, 0.742, sign * 0.168], 0.009, ALLOY, TONE["trim"], role="detail"), RG["panels"])
    hanger = [[-0.165, 0.185], [-0.165, 0.25], [-0.25, 0.228], [-0.288, 0.212], [-0.288, 0.19], [-0.262, 0.182]]
    for side, z0, z1 in (("l", 0.052, 0.06), ("r", -0.06, -0.052)):
        S.put(S.prism("hanger-" + side, "frame", hanger, z0, z1, PAINT, TONE["steel"]), RG["panels"])
        S.struct_of("hanger-" + side, "frame")
    S.put(S.cap("shock-r-body", "frame", [-0.643, 0.39, 0.118], [-0.625, 0.535, 0.118], 0.02, IRON, TONE["steel"], role="detail"), RG["panels"])
    S.struct_of("shock-r-body", "frame")
    S.put(S.cap("shock-r-rod", "swing", [-0.66, 0.292, 0.118], [-0.643, 0.45, 0.118], 0.012, IRON, TONE["steel"], role="detail"), RG["engine"])
    S.put(S.cyl("shock-r-spring", "swing", [-0.655, 0.33, 0.118], [-0.641, 0.455, 0.118], 0.026, IRON, TONE["dark"], role="detail"), RG["engine"])
    S.struct_of("shock-r-rod", "swing")
    for side, sign in (("l", 1), ("r", -1)):
        S.put(S.box("footrest-" + side, "frame", [0.095, 0.02, 0.013], [-0.108, 0.262, sign * 0.197], ALLOY, TONE["trim"], role="detail", corner=0.004), RG["panels"])
    S.put(S.box("number-plate", "frame", [0.004, 0.17, 0.17], [-0.9415, 0.485, 0.0], ALLOY, 0.9, role="texture", rotation=kit.rot_w(-8), texture=True), RG["panels"])


def steering(S):
    """🧭 The steer bone: headset with lamp, instruments, grips, levers and mirrors, the column, the single-sided fork leg with trailing link, shock and caliper, and the mudguard."""
    secs = [([x, y, z], rx, ry, 2.6) for z, x, y, rx, ry in HEADSET]
    S.put(S.tube("headset", "steer", secs, PAINT, TONE["paint"] + 0.03, hint=(1.0, 0.0, 0.0)), RG["steer"])
    S.put(S.ball("headlamp", "steer", [0.03, 0.05, 0.056], [0.379, 1.012, 0.0], GLASS, TONE["glass"], role="glazing", rotation=kit.rot_w(65)), RG["lamp"])
    S.put(S.ball("dash", "steer", [0.055, 0.012, 0.06], [0.262, 1.068, 0.0], GLASS, 0.25, role="detail", rotation=kit.rot_w(65 + 14)), RG["steer"])
    bar = S.cyl("handlebar", "steer", [pt.BAR[0], pt.BAR[1], -0.3], [pt.BAR[0], pt.BAR[1], 0.3], 0.011, IRON, TONE["steel"])
    S.struct(bar, "steer")
    for side, sign in (("l", 1), ("r", -1)):
        a, b = sign * 0.16, sign * pt.GRIP[0]
        S.put(S.cyl("switch-" + side, "steer", [pt.BAR[0], pt.BAR[1], a], [pt.BAR[0], pt.BAR[1], b], 0.023, PAINT, TONE["dark"]), RG["steer"])
        g0, g1 = sign * (pt.GRIP[0] - 0.004), sign * pt.GRIP[1]
        prof = [[0.0, 0.0], [0.0, 0.019], [0.006, 0.019], [0.01, 0.0165], [0.108, 0.0165], [0.114, 0.0195], [0.131, 0.0195], [0.131, 0.0]]
        S.put(S.rev("grip-" + side, "steer", prof, [pt.BAR[0], pt.BAR[1], g0], [0, 0, sign], RUBBER, TONE["tyre"] + 0.04), RG["steer"])
        lever = [([0.318, 1.006, sign * 0.176], 0.009, 0.008), ([0.334, 1.004, sign * 0.2], 0.008, 0.007), ([0.338, 1.002, sign * 0.27], 0.007, 0.006), ([0.332, 1.001, sign * 0.332], 0.008, 0.007)]
        S.put(S.tube("lever-" + side, "steer", lever, ALLOY, TONE["trim"], hint=(0.0, 1.0, 0.0), role="detail"), RG["steer"])
        stalk = kit.bent([[c[0], c[1], sign * c[2]] for c in pt.STALK], [0.03], 4)
        S.put(S.tube("stalk-" + side, "steer", [(p, 0.004, 0.004) for p in stalk], ALLOY, TONE["trim"], role="detail"), RG["steer"])
        S.put(S.ball("mirror-" + side, "steer", [0.013, 0.042, 0.056], [pt.MIRROR[0], pt.MIRROR[1], sign * pt.MIRROR[2]], ALLOY, TONE["trim"], role="detail", rotation=kit.rot_w(65)), RG["steer"])
    col = S.cyl("column", "steer", sp(0.02), sp(0.6), 0.02, IRON, TONE["steel"])
    S.put(col, RG["steer"])
    S.struct_of("column", "steer")
    S.put(S.tube("fork-leg", "steer", [(list(p), r, r) for p, r in LEG], IRON, TONE["steel"]), RG["steer"])
    S.struct_of("fork-leg", "steer")
    LP = pt.LINK_PIVOT
    link = [[LP[0] + 0.02, LP[1] + 0.012], [LP[0] + 0.022, LP[1] - 0.012], [AF[0] - 0.01, AF[1] - 0.03], [AF[0] - 0.032, AF[1] - 0.012], [AF[0] - 0.03, AF[1] + 0.022], [AF[0] - 0.005, AF[1] + 0.034]]
    S.put(S.prism("link", "steer", link, pt.FORK_Z - 0.012, pt.FORK_Z + 0.012, ALLOY, TONE["engine"] + 0.1), RG["steer"])
    S.struct_of("link", "steer")
    S.put(S.cyl("stub-axle", "steer", [AF[0], AF[1], pt.FORK_Z - 0.014], [AF[0], AF[1], -0.058], 0.018, IRON, TONE["steel"], role="detail"), RG["steer"])
    S.struct_of("stub-axle", "steer")
    top, low = [0.577, 0.432, pt.SHOCK_Z], [0.664, 0.243, pt.SHOCK_Z]
    S.put(S.cap("shock-f-body", "steer", low, top, 0.012, IRON, TONE["steel"], role="detail"), RG["steer"])
    S.struct_of("shock-f-body", "steer")
    S.put(S.cyl("shock-f-spring", "steer", kit.lerp(low, top, 0.2), kit.lerp(low, top, 0.92), 0.024, IRON, TONE["dark"], role="detail"), RG["steer"])
    for i, p in (("shock-f-top", top), ("shock-f-foot", low)):
        S.put(S.cyl(i, "steer", [p[0], p[1], pt.FORK_Z], [p[0], p[1], pt.SHOCK_Z - 0.014], 0.008, IRON, TONE["steel"], role="detail"), RG["steer"])
    hose = kit.bent([sp(0.1, 0.02, -0.03), sp(0.5, 0.03, -0.03), [0.556, 0.462, -0.04], [0.59, 0.418, -0.07], [0.627, 0.372, -0.078], [0.636, 0.33, -0.07], [0.62, 0.292, -0.058]], [0.05, 0.04, 0.03, 0.03, 0.03], 4)
    S.put(S.tube("brake-hose", "steer", [(p, 0.004, 0.004) for p in hose], RUBBER, TONE["dark"], role="detail", slender=True))
    S.put(S.box("caliper", "steer", [0.078, 0.044, 0.04], [0.597, 0.273, -0.047], ALLOY, TONE["dark"], role="detail", corner=0.008, rotation=kit.rot_w(125)), RG["brake"])
    S.put(pt.channel(S, "mudguard", "steer", AF, GUARD, 24.0, 160.0, PAINT, TONE["paint"]), RG["panels"])
    S.put(S.rev("disc-f", "wheel-f", [[-0.05, 0.03], [-0.05, 0.1], [-0.046, 0.1], [-0.046, 0.03], [-0.05, 0.03]], AF + (0.0,), [0, 0, 1], IRON, TONE["iron"] + 0.2, role="texture"), RG["brake"])
    pt.ring2d(S, "disc-f", "wheel-f", AF, 0.1, 0.03, -0.05, -0.046, "detail", TONE["iron"] + 0.2, RG["brake"], IRON)


def engine(S):
    """🔧 The swing bone: crankcase with fan cover, forward cylinder, belt case with cover on the left, air box, right arm, header pipe and silencer on the right."""
    C = pt.CRANK
    case = S.rev("crankcase", "swing", [[-0.103, 0.0], [-0.103, 0.06], [-0.092, 0.074], [-0.07, 0.078], [-0.062, 0.088], [0.066, 0.088], [0.066, 0.0]], C + (0.0,), [0, 0, 1], ENGINE, TONE["engine"])
    S.put(case, RG["engine"])
    S.struct_of("crankcase", "swing")
    a, b = [-0.335, 0.268, -0.005], [-0.182, 0.3, -0.005]
    S.put(S.rev("cylinder", "swing", [[0.0, 0.0], [0.0, 0.05], [0.085, 0.05], [0.085, 0.056], [0.148, 0.056], [0.156, 0.045], [0.156, 0.0]], a, kit.sub(b, a), ENGINE, TONE["engine"] + 0.06), RG["engine"])
    S.put(S.lofted("cvt-case", "swing", [(x, lo, hi, t, 3, 0.11) for x, lo, hi, t in CVT], ENGINE, TONE["engine"]), RG["engine"])
    S.struct_of("cvt-case", "swing")
    cover = pt.arc((C[0], 0.262), 0.066, -98, 80, 10) + pt.arc((AR[0], 0.236), 0.07, 80, 262, 10)
    S.put(S.prism("cvt-cover", "swing", cover, 0.15, 0.168, ENGINE, TONE["engine"] + 0.08), RG["engine"])
    S.put(S.box("air-box", "swing", [0.23, 0.07, 0.068], [-0.475, 0.372, 0.1], ENGINE, TONE["dark"], corner=0.02, rotation=kit.rot_w(4)), RG["engine"])
    S.put(S.tube("arm-r", "swing", [([-0.43, 0.25, -0.078], 0.034, 0.01, 3), ([-0.55, 0.243, -0.078], 0.028, 0.01, 3), ([-0.67, 0.23, -0.078], 0.03, 0.01, 3), ([-0.7, 0.228, -0.078], 0.02, 0.01, 3)], ENGINE, TONE["engine"], hint=(0.0, 1.0, 0.0)), RG["engine"])
    S.struct_of("arm-r", "swing")
    (m0, m1, mz, mr) = pt.MUFFLER
    a, b = [m0[0], m0[1], mz], [m1[0], m1[1], mz]
    L = kit.norm(kit.sub(b, a))
    S.put(S.rev("muffler", "swing", [[0.0, 0.0], [0.0, 0.03], [0.012, 0.047], [0.03, mr], [L - 0.03, mr], [L - 0.012, 0.047], [L, 0.03], [L, 0.0]], a, kit.sub(b, a), EXHAUST, TONE["exhaust"]), RG["exhaust"])
    S.put(S.cyl("tail-pipe", "swing", [m1[0] + 0.006, m1[1] - 0.008, mz], list(pt.TAILPIPE), 0.0145, EXHAUST, TONE["exhaust"] - 0.1), RG["exhaust"])
    header = kit.bent([[-0.2, 0.262, -0.02], [-0.215, 0.2, -0.06], [-0.36, 0.178, -0.135], [m0[0] + 0.01, m0[1], mz]], [0.035, 0.05], 4)
    S.put(S.tube("header", "swing", [(p, 0.014, 0.014) for p in header], EXHAUST, TONE["exhaust"] - 0.06), RG["exhaust"])
    shield = [[0.045, mr + 0.004], [L - 0.06, mr + 0.004], [L - 0.06, mr + 0.008], [0.045, mr + 0.008], [0.045, mr + 0.004]]
    hs = S.rev("heat-shield", "swing", shield, a, kit.sub(b, a), EXHAUST, TONE["exhaust"] + 0.14, role="detail")
    hs["sweepRad"] = round(math.radians(150), 6)
    hs["origin"] = [0.0, 0.0, 0.0]
    axis = kit.unit(kit.locd("swing", kit.sub(b, a)))
    hs["axis"] = [1.0, 0.0, 0.0]
    hs["transform"] = {"translation": kit.vec(kit.loc("swing", a)), "rotation": kit.quat(axis, kit.unit(kit.add(kit.cross([0.0, 0.0, -1.0], axis), [0.0, 0.0, -1.0], 1.0)))}
    S.put(hs, RG["exhaust"])
    S.thermal.append(dict(S.cyl("tailpipe-outlet", "swing", [pt.TAILPIPE[0] + 0.0066, pt.TAILPIPE[1] + 0.0003, pt.TAILPIPE[2]], [pt.TAILPIPE[0] - 0.001, pt.TAILPIPE[1], pt.TAILPIPE[2]], 0.0145, EXHAUST, 0.2), region=RG["exhaust"], tags=["tailpipe"]))


def seat(S):
    S.put(S.lofted("seat", "seat", SEAT, VINYL, TONE["seat"]), RG["seat"])
    pan = [(-0.15,.730,.741,.075,3),(-0.32,.718,.729,.137,4),(-0.52,.724,.735,.142,4),(-0.7,.735,.746,.122,3),(-0.79,.748,.759,.069,3)]
    S.struct(S.lofted("seat-pan", "seat", pan, PAINT, TONE["steel"]), "seat")
    S.put(S.cyl("hinge-knuckle", "seat", [HINGE[0], HINGE[1], -.064], [HINGE[0], HINGE[1], .064], .0105, IRON, .32, role="detail"), RG["seat"])
    S.struct(S.cyl("seat-hinge", "seat", [HINGE[0], HINGE[1], -0.06], [HINGE[0], HINGE[1], 0.06], 0.008, IRON, 0.3, role="detail"), "seat", "joint")


def stand(S):
    """🦿 The stand bone at rest (folded back under the engine): two splayed legs with feet, the cross bar and the pivot tube; drawn as true bars so that the 93 degree swing carries them in every view."""
    P = kit.STAND_PIVOT

    def w(u, v, z):
        return kit.world("stand", [u, v, z])

    for side, sign in (("l", 1), ("r", -1)):
        pts = [w(0.0, 0.0, sign * 0.062), w(0.1, 0.0, sign * 0.098), w(0.19, 0.0, sign * 0.132), w(0.204, 0.0, sign * 0.138)]
        S.put(S.tube("stand-leg-" + side, "stand", [(p, 0.011, 0.011) for p in pts], IRON, TONE["steel"]), RG["panels"])
        S.struct_of("stand-leg-" + side, "stand")
        zc = sign * 0.062
        quad = [w(0.0, 0.011, 0)[:2], w(0.0, -0.011, 0)[:2], w(0.204, -0.011, 0)[:2], w(0.204, 0.011, 0)[:2]]
        S.swept("stand-leg-" + side, "stand", quad, zc - 0.011, zc + 0.011, "body", TONE["steel"], RG["panels"], IRON, tol=0.0, tip=(sign * 0.142 - 0.011, sign * 0.142 + 0.011))
        S.put(S.cap("stand-foot-" + side, "stand", w(0.204, 0.0, sign * 0.128), w(0.204, 0.0, sign * 0.158), 0.011, IRON, TONE["steel"]), RG["panels"])
        S.disc("stand-foot-" + side, "stand", w(0.204, 0.0, 0.0), 0.022, sign * 0.117, sign * 0.169, "body", TONE["steel"], RG["panels"], IRON)
    S.put(S.cyl("stand-bar", "stand", w(0.1, 0.0, -0.098), w(0.1, 0.0, 0.098), 0.009, IRON, TONE["steel"], role="detail"), RG["panels"])
    S.struct_of("stand-bar", "stand")
    S.disc("stand-bar", "stand", w(0.1, 0.0, 0.0), 0.018, -0.098, 0.098, "detail", TONE["steel"], RG["panels"], IRON)
    S.disc("stand-pivot", "stand", [P[0], P[1], 0.0], 0.026, -0.075, 0.075, "detail", TONE["steel"], RG["panels"], IRON)
    S.put(S.cyl("stand-pivot", "stand", [P[0], P[1], -0.075], [P[0], P[1], 0.075], 0.013, IRON, TONE["steel"], role="detail"), RG["panels"])
    S.struct(S.cyl("stand-pin", "stand", [P[0], P[1], -0.08], [P[0], P[1], 0.08], 0.008, IRON, 0.3, role="detail"), "stand", "joint")


def frame_structure(S):
    """🏗️ The load path of the monocoque: head tube, the box spine down the leg shield, the floor tunnel, two rear shell rails with their cross member, bearings, pivots, the front contact pad and the masses."""
    S.struct(S.pipe("head-tube", "frame", sp(0.085), sp(0.5), 0.027, 0.003, PAINT, 0.5), "frame")
    spine = [([.458,.6,0],.032,.019,5),([.436,.53,0],.035,.019,5),([.417,.44,0],.04,.019,5),([.383,.33,0],.045,.022,5),([.353,.267,0],.05,.026,5),([.30,.235,0],.055,.030,5)]
    S.struct(S.tube("spine", "frame", spine, PAINT, 0.5), "frame")
    box = [[0.192, -0.06], [0.192, 0.06], [0.272, 0.06], [0.272, -0.06]]
    S.struct(S.solid("tunnel", "extrude", "frame", PAINT, 0.5, profile=box, holes=[kit.inset(box, 0.003)], plane="yz", **{"from": -0.17, "to": 0.31}), "frame")
    for side, sign in (("l", 1), ("r", -1)):
        rail = [([-0.15, 0.24, sign * 0.07], 0.03, 0.02, 4), ([-0.2, 0.42, sign * 0.13], 0.03, 0.02, 4), ([-0.3, 0.6, sign * 0.165], 0.03, 0.02, 4), ([-0.6, 0.66, sign * 0.185], 0.03, 0.02, 4), ([-0.86, 0.64, sign * 0.13], 0.026, 0.018, 4)]
        S.struct(S.tube("rail-" + side, "frame", rail, PAINT, 0.5, hint=(0.0, 1.0, 0.0)), "frame")
    S.struct(S.box("cross-rear", "frame", [0.03, 0.05, 0.27], [-0.86, 0.64, 0.0], PAINT, 0.5), "frame")
    S.struct(S.box("cross-seat", "frame", [0.03, 0.05, 0.33], [-0.3, 0.6, 0.0], PAINT, 0.5), "frame")
    S.struct(S.box("cross-floor", "frame", [0.03, 0.06, 0.16], [-0.155, 0.232, 0.0], PAINT, 0.5), "frame")
    S.struct(S.box("cross-shock", "frame", [0.03, 0.05, 0.37], [-0.6, 0.658, 0.0], PAINT, 0.5), "frame")
    S.struct(S.tube("shock-mount", "frame", [([-0.625, 0.525, 0.118], 0.016, 0.012, 4), ([-0.612, 0.6, 0.13], 0.016, 0.012, 4), ([-0.603, 0.66, 0.15], 0.018, 0.012, 4)], PAINT, 0.5, hint=(1.0, 0.0, 0.0)), "frame")
    for i, u in (("bearing-upper", 0.07), ("bearing-lower", 0.5)):
        S.struct(S.cyl(i, "steer", sp(u), sp(u + 0.018), 0.031, IRON, 0.3, role="detail"), "steer", "joint")
    S.struct(S.cyl("swing-pivot", "swing", [PIVOT[0], PIVOT[1], -0.066], [PIVOT[0], PIVOT[1], 0.066], 0.011, IRON, 0.3, role="detail"), "swing", "joint")
    S.struct(S.cyl("link-pivot", "steer", [pt.LINK_PIVOT[0], pt.LINK_PIVOT[1], pt.FORK_Z - 0.022], [pt.LINK_PIVOT[0], pt.LINK_PIVOT[1], pt.FORK_Z + 0.022], 0.009, IRON, 0.3, role="detail"), "steer", "joint")
    c = kit.loc("steer", (AF[0], 0.00375, 0.0))
    S.struct(S.solid("contact-pad-f", "box", "steer", RUBBER, 0.1, size=[0.09, 0.0075, 0.11], transform={"translation": kit.vec(c), "rotation": kit.rot_w(90 - kit.RAKE)}), "steer", "support", anchor="contact-f")
    S.struct(S.ball("mass-frame", "frame", [0.09, 0.06, 0.08], [-0.25, 0.5, 0.0], IRON, 0.2, role="detail"), "frame", "mass")
    S.struct(S.ball("mass-steer", "steer", [0.05, 0.016, 0.016], sp(0.42, 0.0), IRON, 0.2, role="detail", rotation=None), "steer", "mass")
    S.struct(S.ball("mass-engine", "swing", [0.08, 0.05, 0.05], [-0.47, 0.258, 0.02], IRON, 0.2, role="detail"), "swing", "mass")


def drawing(S):
    """✏️ The three views drawn from the body: every curved part is the true outline of its own solids."""
    P, D = RG["panels"], TONE
    S.proj("floor", ["floor"], "body", D["paint"], P, PAINT)
    S.proj("shield", ["shield-core", "shield-nose", "shield-wing-l", "shield-wing-r"], "body", D["paint"], P, PAINT)
    S.proj("tie", ["tie"], "detail", D["trim"], P, ALLOY, views=("front",))
    S.proj("body", ["tail", "cheek-l", "cheek-r"], "body", D["paint"], P, PAINT)
    S.proj("cowl-r", ["cheek-r"], "detail", D["paint"] + 0.06, P, PAINT, views=("elevation",))
    S.proj("tail-lamp", ["tail-lamp"], "glazing", 0.3, RG["lamp"], GLASS, views=("elevation", "plan"))
    S.proj("plate-holder", ["plate-holder"], "detail", D["dark"], P, PAINT, views=("elevation",))
    S.proj("turn-rr", ["turn-rr"], "glazing", D["glass"], RG["lamp"], GLASS, views=("elevation",))
    for side in ("l", "r"):
        S.proj("turn-f" + side, ["turn-f" + side], "glazing", D["glass"], RG["lamp"], GLASS, views=("front",))
    S.proj("grab-rail", ["grab-rail", "rail-post-l", "rail-post-r"], "detail", D["trim"], P, ALLOY, views=("elevation", "plan"))
    S.proj("hanger", ["hanger-r"], "detail", D["steel"], P, PAINT, views=("elevation",))
    ST = RG["steer"]
    S.proj("headset", ["headset"], "body", D["paint"] + 0.03, ST, PAINT, span=(-0.172, 0.172), cards=("front", "plan"))
    S.proj("headlamp", ["headlamp"], "glazing", D["glass"], RG["lamp"], GLASS, span=(-0.056, 0.056), cards=("front", "plan"))
    S.proj("dash", ["dash"], "detail", 0.25, ST, GLASS, views=("plan",), cards=("plan",))
    for side, sign in (("l", 1), ("r", -1)):
        S.disc("switch-" + side, "steer", pt.BAR + (0.0,), 0.046, sign * 0.16, sign * pt.GRIP[0], "body", D["dark"], ST, PAINT)
        S.disc("grip-" + side, "steer", pt.BAR + (0.0,), 0.036, sign * (pt.GRIP[0] - 0.004), sign * pt.GRIP[1], "body", D["tyre"] + 0.04, ST, RUBBER)
        S.proj("lever-" + side, ["lever-" + side], "detail", D["trim"], ST, ALLOY, views=("front", "plan"), span=tuple(sorted((sign * 0.176, sign * 0.332))))
        S.proj("mirror-" + side, ["stalk-" + side, "mirror-" + side], "detail", D["trim"], ST, ALLOY, span=tuple(sorted((sign * 0.135, sign * 0.37))), cards=("front", "plan"))
    S.proj("column", ["column"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(-0.02, 0.02))
    S.proj("fork-leg", ["fork-leg"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(pt.FORK_Z - 0.019, pt.FORK_Z + 0.019))
    S.proj("link", ["link"], "detail", D["engine"] + 0.1, ST, ALLOY, views=("elevation", "front"), span=(pt.FORK_Z - 0.012, pt.FORK_Z + 0.012))
    S.disc("stub-axle", "steer", AF + (0.0,), 0.036, pt.FORK_Z - 0.014, -0.058, "detail", D["steel"], ST, IRON, views=("elevation", "front"))
    S.proj("shock-f", ["shock-f-body", "shock-f-spring"], "detail", D["dark"], ST, IRON, views=("elevation", "front"), span=(pt.SHOCK_Z - 0.024, pt.SHOCK_Z + 0.024))
    S.proj("caliper", ["caliper"], "detail", D["dark"], RG["brake"], ALLOY, views=("elevation",), span=(-0.067, -0.027))
    S.swept("mudguard-crown", "steer", pt.band(AF, 0.24, 0.267, 24.0, 160.0, 24), -0.08, 0.08, "body", D["paint"], P, PAINT, tol=0.0006)
    S.swept("mudguard", "steer", pt.band(AF, 0.216, 0.25, 24.0, 160.0, 24), -0.098, 0.098, "body", D["paint"], P, PAINT, tol=0.0006)
    S.proj("engine", ["crankcase", "cylinder"], "body", D["engine"], RG["engine"], ENGINE, views=("elevation",))
    S.proj("cvt", ["cvt-case", "cvt-cover", "air-box"], "body", D["engine"], RG["engine"], ENGINE, views=("elevation",))
    S.proj("muffler", ["muffler", "tail-pipe"], "body", D["exhaust"], RG["exhaust"], EXHAUST, views=("elevation",))
    S.proj("heat-shield", ["heat-shield"], "detail", D["exhaust"] + 0.14, RG["exhaust"], EXHAUST, views=("elevation",))
    S.proj("header", ["header"], "detail", D["exhaust"] - 0.06, RG["exhaust"], EXHAUST, views=("elevation",))
    S.proj("seat", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("elevation", "plan"))
    S.proj("seat-front", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("front",), span=(-0.168, 0.168))


def build():
    S = kit.Sheet(50, PENS)
    monocoque(S)
    steering(S)
    wheel(S, "f", AF, 0.11, 1, False)
    wheel(S, "r", AR, 0.12, -1, True)
    engine(S)
    seat(S)
    stand(S)
    frame_structure(S)
    drawing(S)
    return S


DECISIONS = [
    {"en": "Body at 1:50, authored for 7.5 mm features: a steel-monocoque step-through scooter of the Vespa Primavera class. Frame: floor with six rubber rails, leg shield (nose around the steering column, two swept wings, glovebox core, chrome tie), rear body with two side cowls, tail lamp, plate holder, four indicators, grab rail, engine hanger plates and the rear shock (damper body on the frame, rod and spring on the engine unit, so the shock telescopes when the unit swings). Steering: headset with headlamp, instrument dome, switch housings, flanged grips, two brake levers and two mirrors, the column, the single-sided fork leg bent to the right, the trailing link with stub axle, the coil-over shock outside the wide mudguard and the brake caliper. Wheels: tyre sections 110 and 120 mm on 12 inch rims with flanges, a cast five-spoke centre with five windows, hub and 200 mm disc in front, 166 mm drum hub and cap at the rear. Engine unit: crankcase with fan cover, forward cylinder, belt case with raised cover on the left, air box, right arm, header pipe, silencer with heat shield and tail pipe on the right. Stepped two-up seat; centre stand with splayed legs, feet, cross bar and pivot tube; folded passenger footrests on the floor edges, the number plate on its holder and the front brake hose from the lever down the fork leg to the caliper. The rear body sweeps down from the seat nose to the floor in a raked front with a fillet, as the pressed rear shell of the real scooter does.", "de": "Körper bei 1:50, für 7,5-mm-Einzelheiten entworfen: ein Stahl-Monocoque-Roller mit Durchstieg der Klasse Vespa Primavera. Rahmen: Trittbrett mit sechs Gummileisten, Beinschild (Nase um die Lenksäule, zwei gepfeilte Flügel, Handschuhfachkern, Chromleiste), Heckkarosserie mit zwei Seitenbacken, Rücklicht, Kennzeichenträger, vier Blinkern, Haltebügel, Motoraufhängungsblechen und hinterem Federbein (Dämpferkörper am Rahmen, Kolbenstange und Feder am Triebsatz, so dass das Federbein beim Schwingen des Triebsatzes teleskopiert). Lenkung: Lenkkopf mit Scheinwerfer, Instrumentenkuppel, Schaltergehäusen, Griffen mit Bund, zwei Bremshebeln und zwei Spiegeln, die Lenksäule, das einseitige, nach rechts gekröpfte Gabelrohr, die gezogene Schwinge mit Achsstummel, das Federbein außerhalb des breiten Schutzblechs und der Bremssattel. Räder: Reifenquerschnitte 110 und 120 mm auf 12-Zoll-Felgen mit Hörnern, ein gegossener Fünfspeichenstern mit fünf Fenstern, Nabe und 200-mm-Scheibe vorn, 166-mm-Trommelnabe und Kappe hinten. Triebsatz: Kurbelgehäuse mit Lüfterdeckel, liegender Zylinder, Riemengehäuse mit erhabenem Deckel links, Luftfilterkasten, rechter Arm, Krümmer, Schalldämpfer mit Hitzeschild und Endrohr rechts. Gestufte Zweiersitzbank; Hauptständer mit gespreizten Beinen, Füßen, Querstrebe und Lagerrohr; eingeklappte Soziusfußrasten an den Trittbrettkanten, das Kennzeichen auf seinem Träger und die vordere Bremsleitung vom Hebel am Gabelrohr hinab zum Sattel. Die Heckkarosserie fällt von der Sitzbanknase in einer geneigten Front mit Hohlkehle zum Trittbrett ab, wie die gepresste Heckschale des echten Rollers."},
    {"en": "Thermal: every painted panel is the paint film by reference with the 0.8 mm steel wall as a nested solid drawn in by the film; lamps, seat cover, exhaust and tyres are their single layers; engine, rims and brake parts are whole regions. The floor rails count with the panels, the brake caliper with the disc and drum. The exhaust jet leaves from an outlet patch on the tail pipe mouth.", "de": "Thermik: Jedes lackierte Blech ist per Verweis die Lackschicht, die 0,8-mm-Stahlwand liegt als um die Schicht eingezogener Körper darin; Leuchten, Sitzbezug, Auspuff und Reifen sind ihre einzelnen Schichten; Motor, Felgen und Bremsteile sind ganze Regionen. Die Trittleisten zählen zu den Blechen, der Bremssattel zu Scheibe und Trommel. Der Abgasstrahl tritt aus einem Auslassfleck an der Endrohrmündung aus."},
    {"en": "Structure: the load path of the monocoque is the head tube, the box spine down the leg shield, the hollow floor tunnel, two shell rails rising to the seat and running to the tail with four cross members, the shock mount hanging from the left rail, and the engine hanger plates. Steering: handlebar tube, column, fork leg, link, stub axle and shock. The engine unit swings as one arm (crankcase, belt case, right arm); the rear shock carries it. Wheels: rim, spoke centre and hub. Seat pan and stand legs with cross bar. Joints: two head bearings, link pivot, swing pivot, seat hinge, stand pin, two axles. One contact pad under the front tyre; masses at the centres of frame, steering and engine unit.", "de": "Tragwerk: Der Lastpfad des Monocoques ist das Steuerrohr, der Kastenholm im Beinschild, der hohle Bodentunnel, zwei Schalenholme, die zur Sitzbank aufsteigen und zum Heck laufen, mit vier Querträgern, die Federbeinaufnahme am linken Holm und die Motoraufhängungsbleche. Lenkung: Lenkerrohr, Lenksäule, Gabelrohr, Schwinge, Achsstummel und Federbein. Der Triebsatz schwingt als ein Arm (Kurbelgehäuse, Riemengehäuse, rechter Arm); das hintere Federbein stützt ihn. Räder: Felge, Speichenstern und Nabe. Sitzwanne und Ständerbeine mit Querstrebe. Gelenke: zwei Steuerlager, Schwingenlager vorn, Triebsatzlager, Sitzscharnier, Ständerbolzen, zwei Achsen. Ein Kontaktpolster unter dem Vorderreifen; Massen in den Schwerpunkten von Rahmen, Lenkung und Triebsatz."},
]
