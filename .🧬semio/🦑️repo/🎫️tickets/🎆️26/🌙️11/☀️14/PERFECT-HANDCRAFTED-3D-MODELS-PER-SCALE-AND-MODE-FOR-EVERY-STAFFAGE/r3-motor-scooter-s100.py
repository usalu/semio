"""🛵 Round 3, 1:100 of vehicles/motor-scooter, authored for 15 mm features: the same scooter in plain parts (floor, leg shield of a centre body with the nose merged and two swept wings, rear body with side cowls, headset, bars with grips, mirrors, column, fork leg, link, shock, mudguard, dished wheel discs in tyres, engine block, belt case, air box, silencer, seat, stand legs); every section is stated anew for this scale."""
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
PENS = {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.35}
S18, C18 = math.sin(math.radians(18)), math.cos(math.radians(18))

FLOOR = [(-0.165, 0.185, 0.3, 0.19, 5), (0.25, 0.178, 0.302, 0.208, 5), (0.345, 0.2, 0.308, 0.195, 4), (0.385, 0.262, 0.318, 0.15, 3)]
CORE = [(0.288, 0.34, 0.39, 0.075, 4), (0.42, 0.39, 0.45, 0.09, 3.5), (0.52, 0.37, 0.53, 0.12, 3), (0.58, 0.355, 0.545, 0.125, 3), (0.7, 0.305, 0.49, 0.125, 3), (0.86, 0.254, 0.41, 0.09, 3), (0.965, 0.25, 0.36, 0.06, 3)]
WING = [(0.9, 0.325, 0.045), (0.72, 0.41, 0.1), (0.5, 0.45, 0.105), (0.34, 0.412, 0.1), (0.288, 0.378, 0.094)]
TAIL = [(-0.025, 0.297, 0.32, 0.112, 2.3), (-0.06, 0.297, 0.52, 0.125, 2.5), (-0.11, 0.3, 0.7, 0.148, 2.7), (-0.17, 0.33, 0.72, 0.172, 3), (-0.3, 0.39, 0.725, 0.197, 3), (-0.5, 0.445, 0.725, 0.213, 3), (-0.72, 0.505, 0.716, 0.2, 3), (-0.9, 0.538, 0.695, 0.14, 2.6), (-0.948, 0.575, 0.668, 0.085, 2.4)]
CHEEK = [(-0.24, 0.415, 0.505, 0.012), (-0.36, 0.354, 0.572, 0.035), (-0.53, 0.345, 0.602, 0.045), (-0.72, 0.383, 0.6, 0.04), (-0.86, 0.462, 0.58, 0.023), (-0.905, 0.51, 0.562, 0.011)]
SEAT = [(-0.135, 0.725, 0.775, 0.085, 2.6), (-0.25, 0.713, 0.798, 0.145, 3), (-0.44, 0.714, 0.794, 0.17, 3), (-0.6, 0.722, 0.828, 0.158, 3), (-0.76, 0.731, 0.826, 0.126, 2.8), (-0.815, 0.742, 0.8, 0.078, 2.4)]
HEADSET = [(-0.172, 0.29, 1.008, 0.03, 0.027), (-0.1, 0.298, 1.011, 0.075, 0.05), (0.0, 0.305, 1.012, 0.1, 0.062), (0.1, 0.298, 1.011, 0.075, 0.05), (0.172, 0.29, 1.008, 0.03, 0.027)]
CVT = [(-0.272, 0.222, 0.3, 0.035), (-0.36, 0.178, 0.348, 0.05), (-0.55, 0.166, 0.33, 0.05), (-0.7, 0.152, 0.315, 0.05), (-0.787, 0.2, 0.265, 0.03)]
GUARD = [[-0.098, 0.216], [-0.098, 0.246], [-0.078, 0.262], [0.078, 0.262], [0.098, 0.246], [0.098, 0.216], [0.093, 0.216], [0.093, 0.243], [0.075, 0.257], [-0.075, 0.257], [-0.093, 0.243], [-0.093, 0.216], [-0.098, 0.216]]
TYRE = {0.11: [[-0.047, 0.166], [-0.055, 0.19], [-0.045, 0.22], [-0.015, 0.23], [0.015, 0.23], [0.045, 0.22], [0.055, 0.19], [0.047, 0.166], [0.04, 0.15], [-0.04, 0.15], [-0.047, 0.166]], 0.12: [[-0.047, 0.166], [-0.06, 0.19], [-0.05, 0.22], [-0.017, 0.23], [0.017, 0.23], [0.05, 0.22], [0.06, 0.19], [0.047, 0.166], [0.04, 0.15], [-0.04, 0.15], [-0.047, 0.166]]}


def wheel(S, name, c, width, face, drum):
    """🛞 One wheel at 1:100: the tyre section and one dished wheel disc (rim barrel with its centre web on the open side); a hub in front, the drum at the rear."""
    bone = "wheel-" + name
    h = width / 2
    S.put(S.rev("tyre-" + name, bone, TYRE[width], c + (0.0,), [0, 0, 1], RUBBER, TONE["tyre"]), RG["tyre"])
    pt.ring2d(S, "tyre-" + name, bone, c, 0.23, 0.166, -h, h, "body", TONE["tyre"], RG["tyre"], RUBBER)
    a, b = sorted((face * 0.007, face * 0.023))
    S.put(S.rev("rim-" + name, bone, [[a, 0.0], [a, 0.136], [-0.049, 0.136], [-0.049, 0.166], [0.049, 0.166], [0.049, 0.136], [b, 0.136], [b, 0.0]], c + (0.0,), [0, 0, 1], ALLOY, TONE["rim"]), RG["rim"])
    S.struct_of("rim-" + name, bone)
    pt.ring2d(S, "rim-" + name, bone, c, 0.166, None, -0.049, 0.049, "body", TONE["rim"], RG["rim"], ALLOY)
    if drum:
        hub = S.cyl("hub-" + name, bone, c + (0.023,), c + (0.055,), 0.083, IRON, TONE["iron"])
        S.put(hub, RG["brake"])
        pt.ring2d(S, "hub-" + name, bone, c, 0.083, None, 0.023, 0.055, "detail", TONE["iron"], RG["brake"], IRON)
    else:
        hub = S.cyl("hub-" + name, bone, c + (-0.062,), c + (0.007,), 0.034, ALLOY, TONE["rim"])
        S.put(hub, RG["rim"])
        pt.ring2d(S, "hub-" + name, bone, c, 0.034, None, -0.062, 0.007, "detail", TONE["rim"], RG["rim"], ALLOY)
    S.struct_of("hub-" + name, bone)
    S.struct(S.cyl("axle-" + name, bone, c + (-0.07,), c + (0.07,), 0.0085, IRON, 0.3, role="detail"), bone, "joint")


def build():
    S = kit.Sheet(100, PENS)
    P, ST, D = RG["panels"], RG["steer"], TONE
    S.put(S.lofted("floor", "frame", FLOOR, PAINT, D["paint"]), P)
    S.put(S.lofted("shield-core", "frame", CORE, PAINT, D["paint"], along="y"), P)
    for side, sign in (("l", 1), ("r", -1)):
        pts = [([x - hs * S18, y, sign * (0.03 + hs * C18)], hs, 0.011) for y, x, hs in WING]
        S.put(S.tube("shield-wing-" + side, "frame", pts, PAINT, D["paint"], hint=(-S18, 0.0, sign * C18)), P)
        S.put(S.lofted("cheek-" + side, "frame", [(x, lo, hi, t, 2.4, sign * 0.188) for x, lo, hi, t in CHEEK], PAINT, D["paint"] + 0.04), P)
    S.put(S.lofted("tail", "frame", TAIL, PAINT, D["paint"]), P)
    S.put(S.box("tail-lamp", "frame", [0.03, 0.07, 0.13], [-0.94, 0.622, 0.0], GLASS, 0.3, role="glazing", corner=0.012), RG["lamp"])
    S.put(S.box("plate-holder", "frame", [0.016, 0.2, 0.2], [-0.932, 0.49, 0.0], PAINT, D["dark"], rotation=kit.rot_w(-8)), P)
    rail = [[-0.6, 0.738, 0.168], [-0.84, 0.762, 0.168], [-0.935, 0.766, 0.09], [-0.935, 0.766, -0.09], [-0.84, 0.762, -0.168], [-0.6, 0.738, -0.168]]
    S.put(S.tube("grab-rail", "frame", [(p, 0.009, 0.009) for p in kit.bent(rail, [0.05, 0.06, 0.06, 0.05], 2)], ALLOY, D["trim"], role="detail"), P)
    S.proj("floor", ["floor"], "body", D["paint"], P, PAINT)
    S.proj("shield", ["shield-core", "shield-wing-l", "shield-wing-r"], "body", D["paint"], P, PAINT)
    S.proj("body", ["tail", "cheek-l", "cheek-r"], "body", D["paint"], P, PAINT)
    S.proj("cowl-r", ["cheek-r"], "detail", D["paint"] + 0.06, P, PAINT, views=("elevation",))
    S.proj("tail-lamp", ["tail-lamp"], "glazing", 0.3, RG["lamp"], GLASS, views=("elevation", "plan"))
    S.proj("plate-holder", ["plate-holder"], "detail", D["dark"], P, PAINT, views=("elevation",))
    S.proj("grab-rail", ["grab-rail"], "detail", D["trim"], P, ALLOY, views=("elevation", "plan"))

    S.put(S.tube("headset", "steer", [([x, y, z], rx, ry, 2.6) for z, x, y, rx, ry in HEADSET], PAINT, D["paint"] + 0.03, hint=(1.0, 0.0, 0.0)), ST)
    S.put(S.ball("headlamp", "steer", [0.03, 0.05, 0.056], [0.379, 1.012, 0.0], GLASS, D["glass"], role="glazing", rotation=kit.rot_w(65)), RG["lamp"])
    S.struct(S.cyl("handlebar", "steer", [pt.BAR[0], pt.BAR[1], -0.3], [pt.BAR[0], pt.BAR[1], 0.3], 0.011, IRON, D["steel"]), "steer")
    S.proj("headset", ["headset"], "body", D["paint"] + 0.03, ST, PAINT, span=(-0.172, 0.172), cards=("front", "plan"))
    S.proj("headlamp", ["headlamp"], "glazing", D["glass"], RG["lamp"], GLASS, span=(-0.056, 0.056), cards=("front", "plan"))
    for side, sign in (("l", 1), ("r", -1)):
        S.put(S.cyl("grip-" + side, "steer", [pt.BAR[0], pt.BAR[1], sign * 0.16], [pt.BAR[0], pt.BAR[1], sign * pt.GRIP[1]], 0.018, RUBBER, D["tyre"] + 0.04), ST)
        S.disc("grip-" + side, "steer", pt.BAR + (0.0,), 0.036, sign * 0.16, sign * pt.GRIP[1], "body", D["tyre"] + 0.04, ST, RUBBER)
        stalk = [[c[0], c[1], sign * c[2]] for c in pt.STALK]
        S.put(S.tube("stalk-" + side, "steer", [(p, 0.004, 0.004) for p in stalk], ALLOY, D["trim"], role="detail", slender=True), ST)
        S.put(S.ball("mirror-" + side, "steer", [0.013, 0.042, 0.056], [pt.MIRROR[0], pt.MIRROR[1], sign * pt.MIRROR[2]], ALLOY, D["trim"], role="detail", rotation=kit.rot_w(65)), ST)
        S.proj("mirror-" + side, ["stalk-" + side, "mirror-" + side], "detail", D["trim"], ST, ALLOY, span=tuple(sorted((sign * 0.135, sign * 0.37))), cards=("front", "plan"))
    S.put(S.cyl("column", "steer", sp(0.02), sp(0.6), 0.02, IRON, D["steel"]), ST)
    S.struct_of("column", "steer")
    leg = [((0.5306, 0.4698, 0.0), 0.019), ((0.585, 0.392, -0.07), 0.018), ((0.665, 0.315, pt.FORK_Z), 0.018), ((0.752, 0.26, pt.FORK_Z), 0.019)]
    S.put(S.tube("fork-leg", "steer", [(list(p), r, r) for p, r in leg], IRON, D["steel"]), ST)
    S.struct_of("fork-leg", "steer")
    LP = pt.LINK_PIVOT
    link = [[LP[0] + 0.02, LP[1] + 0.012], [LP[0] + 0.022, LP[1] - 0.012], [AF[0] - 0.01, AF[1] - 0.03], [AF[0] - 0.032, AF[1] - 0.01], [AF[0] - 0.02, AF[1] + 0.03]]
    S.put(S.prism("link", "steer", link, pt.FORK_Z - 0.014, -0.058, ALLOY, D["engine"] + 0.1), ST)
    S.struct_of("link", "steer")
    top, low = [0.577, 0.432, pt.SHOCK_Z], [0.664, 0.243, pt.SHOCK_Z]
    S.put(S.cyl("shock-f", "steer", low, top, 0.022, IRON, D["dark"], role="detail"), ST)
    S.struct_of("shock-f", "steer")
    S.put(pt.channel(S, "mudguard", "steer", AF, GUARD, 24.0, 160.0, PAINT, D["paint"]), P)
    S.put(S.rev("disc-f", "wheel-f", [[-0.05, 0.03], [-0.05, 0.1], [-0.046, 0.1], [-0.046, 0.03], [-0.05, 0.03]], AF + (0.0,), [0, 0, 1], IRON, D["iron"] + 0.2, role="texture"), RG["brake"])
    pt.ring2d(S, "disc-f", "wheel-f", AF, 0.1, 0.034, -0.05, -0.046, "detail", D["iron"] + 0.2, RG["brake"], IRON)
    S.proj("column", ["column"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(-0.02, 0.02))
    S.proj("fork-leg", ["fork-leg"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(pt.FORK_Z - 0.019, pt.FORK_Z + 0.019))
    S.proj("link", ["link"], "detail", D["engine"] + 0.1, ST, ALLOY, views=("elevation", "front"), span=(pt.FORK_Z - 0.014, -0.058))
    S.proj("shock-f", ["shock-f"], "detail", D["dark"], ST, IRON, views=("elevation", "front"), span=(pt.SHOCK_Z - 0.022, pt.SHOCK_Z + 0.022))
    S.swept("mudguard", "steer", pt.band(AF, 0.216, 0.262, 24.0, 160.0, 16), -0.098, 0.098, "body", D["paint"], P, PAINT, tol=0.001)
    wheel(S, "f", AF, 0.11, 1, False)
    wheel(S, "r", AR, 0.12, -1, True)

    C = pt.CRANK
    S.put(S.rev("crankcase", "swing", [[-0.1, 0.0], [-0.1, 0.07], [-0.065, 0.088], [0.066, 0.088], [0.066, 0.0]], C + (0.0,), [0, 0, 1], ENGINE, D["engine"]), RG["engine"])
    S.struct_of("crankcase", "swing")
    a, b = [-0.335, 0.268, -0.005], [-0.182, 0.3, -0.005]
    S.put(S.cyl("cylinder", "swing", a, b, 0.054, ENGINE, D["engine"] + 0.06), RG["engine"])
    S.put(S.lofted("cvt-case", "swing", [(x, lo, hi, t, 3, 0.115) for x, lo, hi, t in CVT], ENGINE, D["engine"]), RG["engine"])
    S.struct_of("cvt-case", "swing")
    S.put(S.box("air-box", "swing", [0.23, 0.07, 0.068], [-0.475, 0.372, 0.1], ENGINE, D["dark"], corner=0.02), RG["engine"])
    (m0, m1, mz, mr) = pt.MUFFLER
    a, b = [m0[0], m0[1], mz], [m1[0], m1[1], mz]
    L = kit.norm(kit.sub(b, a))
    S.put(S.rev("muffler", "swing", [[0.0, 0.0], [0.0, 0.035], [0.025, mr], [L - 0.025, mr], [L, 0.035], [L, 0.0]], a, kit.sub(b, a), EXHAUST, D["exhaust"]), RG["exhaust"])
    S.put(S.cyl("tail-pipe", "swing", [m1[0] + 0.006, m1[1] - 0.008, mz], list(pt.TAILPIPE), 0.0145, EXHAUST, D["exhaust"] - 0.1), RG["exhaust"])
    header = kit.bent([[-0.2, 0.262, -0.02], [-0.215, 0.2, -0.06], [-0.36, 0.178, -0.135], [m0[0] + 0.01, m0[1], mz]], [0.035, 0.05], 3)
    S.put(S.tube("header", "swing", [(q, 0.014, 0.014) for q in header], EXHAUST, D["exhaust"] - 0.06), RG["exhaust"])
    guard = S.rev("silencer-guard", "swing", [[.045,.059],[L-.055,.059],[L-.055,.065],[.045,.065],[.045,.059]], a, kit.sub(b,a), EXHAUST, D["exhaust"]+.12, role="texture")
    guard.update(axis=[1,0,0],origin=[0,0,0],sweepRad=2.5,texture=True)
    axis = kit.unit(kit.locd("swing",kit.sub(b,a)))
    guard["transform"]={"translation":kit.vec(kit.loc("swing",a)),"rotation":kit.quat(axis,kit.unit(kit.add(kit.cross([0,0,-1],axis),[0,0,-1],1)))}
    S.put(guard,RG["exhaust"])
    S.thermal.append(dict(S.cyl("tailpipe-outlet", "swing", [pt.TAILPIPE[0] + 0.0142, pt.TAILPIPE[1] + 0.0006, pt.TAILPIPE[2]], [pt.TAILPIPE[0] - 0.001, pt.TAILPIPE[1], pt.TAILPIPE[2]], 0.0145, EXHAUST, 0.2), region=RG["exhaust"], tags=["tailpipe"]))
    S.proj("engine", ["crankcase", "cylinder"], "body", D["engine"], RG["engine"], ENGINE, views=("elevation",))
    S.proj("cvt", ["cvt-case", "air-box"], "body", D["engine"], RG["engine"], ENGINE, views=("elevation",))
    S.proj("muffler", ["muffler", "tail-pipe"], "body", D["exhaust"], RG["exhaust"], EXHAUST, views=("elevation",))
    S.proj("header", ["header"], "detail", D["exhaust"] - 0.06, RG["exhaust"], EXHAUST, views=("elevation",))

    S.put(S.lofted("seat", "seat", SEAT, VINYL, D["seat"]), RG["seat"])
    S.proj("seat", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("elevation", "plan"))
    S.proj("seat-front", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("front",), span=(-0.168, 0.168))
    pan = [[-0.14, 0.085], [-0.3, 0.15], [-0.6, 0.15], [-0.8, 0.1], [-0.8, -0.1], [-0.6, -0.15], [-0.3, -0.15], [-0.14, -0.085]]
    S.struct(S.solid("seat-pan", "extrude", "seat", PAINT, D["steel"], profile=[[kit.r5(z), kit.r5(kit.loc("seat", [x, 0.72])[0])] for x, z in pan], plane="zx", **{"from": kit.r5(kit.loc("seat", [0, 0.73])[1]), "to": kit.r5(kit.loc("seat", [0, 0.714])[1])}), "seat")
    S.put(S.cyl("hinge-knuckle", "seat", [HINGE[0], HINGE[1], -0.064], [HINGE[0], HINGE[1], 0.064], 0.0105, IRON, 0.32, role="detail"), RG["seat"])
    S.struct(S.cyl("seat-hinge", "seat", [HINGE[0], HINGE[1], -0.06], [HINGE[0], HINGE[1], 0.06], 0.008, IRON, 0.3, role="detail"), "seat", "joint")

    def w(u, v, z):
        return kit.world("stand", [u, v, z])

    for side, sign in (("l", 1), ("r", -1)):
        S.put(S.cyl("stand-leg-" + side, "stand", w(0.0, 0.0, sign * 0.062), w(0.204, 0.0, sign * 0.142), 0.011, IRON, D["steel"]), P)
        S.struct_of("stand-leg-" + side, "stand")
        zc = sign * 0.062
        quad = [w(0.0, 0.011, 0)[:2], w(0.0, -0.011, 0)[:2], w(0.204, -0.011, 0)[:2], w(0.204, 0.011, 0)[:2]]
        S.swept("stand-leg-" + side, "stand", quad, zc - 0.011, zc + 0.011, "body", D["steel"], P, IRON, tol=0.0, tip=(sign * 0.142 - 0.011, sign * 0.142 + 0.011))
    SP = kit.STAND_PIVOT
    S.struct(S.cyl("stand-pin", "stand", [SP[0], SP[1], -0.08], [SP[0], SP[1], 0.08], 0.008, IRON, 0.3, role="detail"), "stand", "joint")

    S.struct(S.pipe("head-tube", "frame", sp(0.085), sp(0.5), 0.027, 0.003, PAINT, 0.5), "frame")
    spine = [([ax(0.56) - 0.012, 0.56, 0.0], 0.04, 0.028, 5), ([0.44, 0.4, 0.0], 0.047, 0.029, 5), ([0.36, 0.255, 0.0], 0.055, 0.034, 5), ([0.3, 0.232, 0.0], 0.06, 0.038, 5)]
    S.struct(S.tube("spine", "frame", spine, PAINT, 0.5), "frame")
    box = [[0.192, -0.06], [0.192, 0.06], [0.272, 0.06], [0.272, -0.06]]
    S.struct(S.solid("tunnel", "extrude", "frame", PAINT, 0.5, profile=box, holes=[kit.inset(box, 0.003)], plane="yz", **{"from": -0.17, "to": 0.31}), "frame")
    for side, sign in (("l", 1), ("r", -1)):
        rail = [([-0.15, 0.24, sign * 0.07], 0.03, 0.02, 4), ([-0.25, 0.52, sign * 0.15], 0.03, 0.02, 4), ([-0.4, 0.64, sign * 0.18], 0.03, 0.02, 4), ([-0.86, 0.64, sign * 0.13], 0.026, 0.018, 4)]
        S.struct(S.tube("rail-" + side, "frame", rail, PAINT, 0.5, hint=(0.0, 1.0, 0.0)), "frame")
    S.struct(S.box("cross-rear", "frame", [0.03, 0.05, 0.27], [-0.86, 0.64, 0.0], PAINT, 0.5), "frame")
    S.struct(S.box("cross-floor", "frame", [0.03, 0.06, 0.16], [-0.155, 0.232, 0.0], PAINT, 0.5), "frame")
    S.struct(S.cap("shock-r", "frame", [-0.66, 0.292, 0.118], [-0.625, 0.535, 0.118], 0.017, IRON, D["steel"], role="detail"), "frame")
    S.struct(S.tube("shock-mount", "frame", [([-0.625, 0.525, 0.118], 0.016, 0.014, 4), ([-0.61, 0.64, 0.155], 0.018, 0.014, 4)], PAINT, 0.5, hint=(1.0, 0.0, 0.0)), "frame")
    S.struct(S.tube("arm-r", "swing", [([-0.43, 0.25, -0.078], 0.034, 0.01, 3), ([-0.7, 0.228, -0.078], 0.024, 0.01, 3)], ENGINE, D["engine"], hint=(0.0, 1.0, 0.0)), "swing")
    for i, u in (("bearing-upper", 0.07), ("bearing-lower", 0.5)):
        S.struct(S.cyl(i, "steer", sp(u), sp(u + 0.018), 0.031, IRON, 0.3, role="detail"), "steer", "joint")
    S.struct(S.cyl("swing-pivot", "swing", [PIVOT[0], PIVOT[1], -0.066], [PIVOT[0], PIVOT[1], 0.066], 0.011, IRON, 0.3, role="detail"), "swing", "joint")
    S.struct(S.cyl("link-pivot", "steer", [LP[0], LP[1], pt.FORK_Z - 0.022], [LP[0], LP[1], pt.FORK_Z + 0.022], 0.009, IRON, 0.3, role="detail"), "steer", "joint")
    c = kit.loc("steer", (AF[0], 0.0075, 0.0))
    S.struct(S.solid("contact-pad-f", "box", "steer", RUBBER, 0.1, size=[0.09, 0.015, 0.11], transform={"translation": kit.vec(c), "rotation": kit.rot_w(90 - kit.RAKE)}), "steer", "support", anchor="contact-f")
    S.struct(S.ball("mass-frame", "frame", [0.09, 0.06, 0.08], [-0.25, 0.5, 0.0], IRON, 0.2, role="detail"), "frame", "mass")
    S.struct(S.ball("mass-steer", "steer", [0.05, 0.016, 0.016], sp(0.42, 0.0), IRON, 0.2, role="detail"), "steer", "mass")
    S.struct(S.ball("mass-engine", "swing", [0.08, 0.05, 0.05], [-0.47, 0.258, 0.02], IRON, 0.2, role="detail"), "swing", "mass")
    return S


DECISIONS = [
    {"en": "Body at 1:100, authored for 15 mm features: the leg shield is one centre body with the nose merged into it and two swept wings; the rear body keeps its two side cowls; floor rails, tie, indicators, levers, switch housings, caliper, heat shield, stand feet and cross bar are left out. Each wheel is a tyre section on one dished wheel disc (rim barrel with its centre web) with a hub; the 200 mm brake disc stays as a thin ring, the rear drum as a plain cylinder. Grip and bar end are one cylinder per side; the mirror stalks are true 8 mm rods (slender). Engine block and cylinder are two plain turned bodies beside the belt case with its air box; the 28 mm header pipe runs from the cylinder under the engine to the silencer.", "de": "Körper bei 1:100, für 15-mm-Einzelheiten entworfen: Das Beinschild ist ein Mittelkörper mit eingeschmolzener Nase und zwei gepfeilten Flügeln; die Heckkarosserie behält ihre zwei Seitenbacken; Trittleisten, Zierleiste, Blinker, Hebel, Schaltergehäuse, Bremssattel, Hitzeschild, Ständerfüße und Querstrebe entfallen. Jedes Rad ist ein Reifenquerschnitt auf einer geschüsselten Radscheibe (Felgenbett mit Mittelsteg) mit Nabe; die 200-mm-Bremsscheibe bleibt ein dünner Ring, die hintere Trommel ein glatter Zylinder. Griff und Lenkerende sind ein Zylinder je Seite; die Spiegelarme sind echte 8-mm-Stäbe (schlank). Motorblock und Zylinder sind zwei glatte Drehkörper neben dem Riemengehäuse mit Luftfilterkasten; der 28-mm-Krümmer läuft vom Zylinder unter dem Motor zum Schalldämpfer."},
    {"en": "Thermal: every region is the surface of its own parts by reference (layers are merged at this scale); the exhaust jet leaves from an outlet patch on the tail pipe mouth.", "de": "Thermik: Jede Region ist per Verweis die Oberfläche ihrer eigenen Teile (die Schichten sind in diesem Maßstab zusammengefasst); der Abgasstrahl tritt aus einem Auslassfleck an der Endrohrmündung aus."},
    {"en": "Structure: head tube, box spine, hollow floor tunnel, two shell rails with a floor and a rear cross member; handlebar tube, column, fork leg, link and shock; engine block, belt case and right arm as the swinging arm with the rear shock and its mount under the left rail; wheel discs and hubs; seat pan; stand legs. Joints: two head bearings, link pivot, swing pivot, seat hinge, stand pin, two axles. Contact pad under the front tyre; masses at the centres of frame, steering and engine unit.", "de": "Tragwerk: Steuerrohr, Kastenholm, hohler Bodentunnel, zwei Schalenholme mit einem Boden- und einem Heckquerträger; Lenkerrohr, Lenksäule, Gabelrohr, Schwinge und Federbein; Motorblock, Riemengehäuse und rechter Arm als Triebsatzschwinge mit dem hinteren Federbein und seiner Aufnahme unter dem linken Holm; Radscheiben und Naben; Sitzwanne; Ständerbeine. Gelenke: zwei Steuerlager, Schwingenlager vorn, Triebsatzlager, Sitzscharnier, Ständerbolzen, zwei Achsen. Kontaktpolster unter dem Vorderreifen; Massen in den Schwerpunkten von Rahmen, Lenkung und Triebsatz."},
]
