"""🚲 Round 3 vehicles/city-bicycle at 1:100 (smallest feature 15 mm): the Dutch city bicycle authored anew for this scale. Masses are merged (headset into the head tube, lug into the seat tube, saddle springs into one yoke, hub and brake unit into one shell, pedal into one block, lamp and bracket into one housing), sections are simplified and everything thinner than the carrier rods is left out."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-city-bicycle-parts")
kit = pt.kit
R, B, F, P = pt.R3, pt.B3, pt.F3, kit.P
sp = kit.steer_pt
ALL, SIDE = pt.ALL, pt.SIDE
PAINT, CHROME, ALLOY, RUBBER, PLASTIC, SKIN = "paint", "chrome", "alloy", "rubber", "plastic", "saddle-skin"
TUBE, GUARD, CASE, STEEL, DARK = 0.3, 0.55, 0.6, 0.7, 0.1
PENS = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}
PEN = 0.13


def wheel(S, bone, c, shell):
    tag = bone[-1]
    S.put(S.rev(f"tyre-{tag}", bone, pt.tyre_profile(12), c, [0, 0, 1], RUBBER, DARK), "tyres")
    outer, inner = pt.ring(c, 0.345, 0.308, 40)
    S.swept(f"tyre-{tag}", bone, outer, -0.0185, 0.0185, "body", DARK, "tyres", RUBBER, holes=[inner], tol=0.0006, views=SIDE)
    for k, half in enumerate(pt.half_rings(c, 0.345, 0.308, 20)):
        S.swept(f"tyre-{tag}-{k + 1}", bone, half, -0.0185, 0.0185, "body", DARK, "tyres", RUBBER, tol=0.0006, views=("front", "plan"))
    rim = [(-0.0125, 0.3175), (-0.0125, 0.3), (0.0, 0.296), (0.0125, 0.3), (0.0125, 0.3175), (-0.0125, 0.3175)]
    S.put(S.rev(f"rim-{tag}", bone, rim, c, [0, 0, 1], ALLOY, 0.78), "rims-spokes")
    S.struct_of(f"rim-{tag}", bone)
    outer, inner = pt.ring(c, 0.3175, 0.296, 40)
    S.swept(f"rim-{tag}", bone, outer, -0.0125, 0.0125, "body", 0.78, "rims-spokes", ALLOY, pen=PEN, holes=[inner], tol=0.0006, views=SIDE)
    z0, z1, r = shell
    S.put(S.annulus(f"hub-{tag}", bone, [(z0, 0.0075), (z0, r - 0.008), (z0 + 0.006, r), (z1 - 0.006, r), (z1, r - 0.008), (z1, 0.0075)], c, [0, 0, 1], CHROME, STEEL), "hub-brakes")
    S.struct_of(f"hub-{tag}", bone)
    S.disc(f"hub-{tag}", bone, c, 2 * r, z0, z1, "detail", STEEL, "hub-brakes", CHROME, pen=PEN)
    half = max(abs(z0), abs(z1)) + 0.022
    S.put(S.cyl(f"axle-{tag}", bone, [c[0], c[1], -half], [c[0], c[1], half], 0.0075, CHROME, STEEL, role="detail"), "hub-brakes")
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], z0], [c[0], c[1], z1], 0.012, CHROME, 0.5), bone, "joint")
    S.disc(f"axle-{tag}", bone, c, 0.015, -half, half, "detail", STEEL, "hub-brakes", CHROME, pen=PEN, views=("front", "plan"))
    for k, (fz, off, turn) in enumerate(((0.026, 0.0, 15.0), (-0.026, 30.0, 15.0))):
        a = [0.032 * math.cos(math.radians(off)), 0.032 * math.sin(math.radians(off)), fz]
        b = [0.297 * math.cos(math.radians(off + turn)), 0.297 * math.sin(math.radians(off + turn)), 0.0]
        s = S.cap(f"spoke-{tag}-{k + 1}", bone, kit.add(c, a), kit.add(c, b), 0.001, ALLOY, 0.6, role="texture", slender=True, texture=True, repeat={"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": round(math.radians(60), 6), "origin": [0, 0, 0]})
        S.put(s, "rims-spokes")
        S.struct_of(f"spoke-{tag}-{k + 1}", bone)
        for j in range(6):
            q = math.radians(60 * j)
            ra = [a[0] * math.cos(q) - a[1] * math.sin(q), a[0] * math.sin(q) + a[1] * math.cos(q), 0]
            rb = [b[0] * math.cos(q) - b[1] * math.sin(q), b[0] * math.sin(q) + b[1] * math.cos(q), 0]
            S.sliver(f"spoke-{tag}-{k * 6 + j + 1}", bone, kit.add(c, ra), kit.add(c, rb), "texture", 0.6, "rims-spokes", ALLOY, PEN)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.03, 0.03, 0.03], c, CHROME, 0.5), bone, "mass")


def bar(S, i, bone, a, b, r, region, mat=PAINT, tone=TUBE, role="body", member=True, views=ALL, depth=None, pen=PEN):
    S.put(S.cyl(i, bone, a, b, r, mat, tone, role=role), region)
    if member:
        S.struct_of(i, bone)
    S.member(i, bone, a, b, 2 * r, role, tone, region, mat, pen=pen, views=views, depth=depth)


def frame(S):
    reg = "frame-tubes"
    S.put(S.cyl("bb-shell", "frame", [B[0], B[1], -0.034], [B[0], B[1], 0.034], 0.02, PAINT, TUBE), reg)
    S.struct_of("bb-shell", "frame")
    S.disc("bb-shell", "frame", B, 0.04, -0.034, 0.034, "body", TUBE, reg, PAINT, pen=PEN)
    bar(S, "seat-tube", "frame", B, pt.SEAT_TOP, 0.0148, reg)
    bar(S, "seat-post", "frame", pt.SEAT_TOP, kit.add(pt.POST_TOP, [pt.ST[0], pt.ST[1], 0], 0.03), 0.0127, reg, mat=CHROME, tone=STEEL)
    bar(S, "down-tube", "frame", pt.DOWN_HEAD, B, 0.0159, reg)
    bar(S, "head-tube", "frame", sp(-0.004, 0), sp(0.246, 0), 0.0185, reg)
    loop = [pt.LOOP_HEAD] + kit.bezier(pt.LOOP_Q1, pt.LOOP_C, pt.LOOP_END, 5)
    S.put(S.tube("loop-tube", "frame", [(p, 0.0143, 0.0143) for p in loop], PAINT, TUBE), reg)
    S.struct_of("loop-tube", "frame")
    S.swept("loop-tube", "frame", kit.ribbon(loop, 0.0143), -0.0143, 0.0143, "body", TUBE, reg, PAINT, pen=PEN, tol=0.0015)
    for side, name in ((1, "l"), (-1, "r")):
        for key, (a, b, r0, r1) in pt.stay_points().items():
            a, b = pt.mirror(a, side), pt.mirror(b, side)
            bar(S, f"{key}-{name}", "frame", a, b, max((r0 + r1) / 2, 0.008), reg, pen=0.15, depth=[max(a[0], b[0]), max(a[1], b[1]), side * 0.03] if key == "chainstay" else None)
    bracket = [[P[0] - 0.015, P[1] - 0.008], [P[0] + 0.015, P[1] - 0.008], [P[0] + 0.03, 0.3], [P[0] - 0.01, 0.3]]
    S.put(S.prism("stand-bracket", "frame", bracket, -0.026, 0.026, PAINT, TUBE, role="detail"), reg)
    S.swept("stand-bracket", "frame", bracket, -0.026, 0.026, "detail", TUBE, reg, PAINT, pen=PEN)
    x0, x1 = pt.RACK_X
    y, z = pt.RACK_Y, pt.RACK_Z
    xf = pt.SEAT_TOP[0] - 0.012
    hoop = kit.bent([[xf, y, z], [x1, y, z], [x1, y, -z], [xf, y, -z]], [0.03, 0.03], 3)
    S.put(S.tube("rack-hoop", "frame", [(p, 0.005, 0.005) for p in hoop], PAINT, TUBE, hint=[0.0, 1.0, 0.0], role="detail"), reg)
    S.struct_of("rack-hoop", "frame")
    S.flat("elevation", "rack-hoop", "frame", "detail", TUBE, reg, PAINT, [[xf, y - 0.005], [x1 - 0.005, y - 0.005], [x1 - 0.005, y + 0.005], [xf, y + 0.005]], [xf, y, -z], pen=PEN)
    S.flat("front", "rack-hoop", "frame", "detail", TUBE, reg, PAINT, [[-z - 0.005, y - 0.005], [z + 0.005, y - 0.005], [z + 0.005, y + 0.005], [-z - 0.005, y + 0.005]], [xf, y, -z], pen=PEN)
    outer = [[p[0], p[2]] for p in hoop]
    S.flat("plan", "rack-hoop", "frame", "detail", TUBE, reg, PAINT, kit.ribbon(outer, 0.005), [xf, y, -z], pen=PEN, tol=0.0015)
    for side, name in ((1, "l"), (-1, "r")):
        eye = [R[0], R[1] + 0.014, side * 0.073]
        bar(S, f"rack-leg-a-{name}", "frame", [-0.6, y, side * z], eye, 0.005, reg, role="detail")
        bar(S, f"rack-leg-b-{name}", "frame", [-0.8, y, side * z], eye, 0.005, reg, role="detail")
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.045, 0.045, 0.045], [fx, fy, 0], PAINT, 0.5), "frame", "mass")


SADDLE = [(-0.222, 0.984, 0.015, 0.012), (-0.26, 0.983, 0.03, 0.017), (-0.33, 0.98, 0.058, 0.02), (-0.4, 0.982, 0.099, 0.022), (-0.455, 0.988, 0.106, 0.021), (-0.49, 0.995, 0.085, 0.012)]


def saddle(S):
    S.put(S.tube("saddle", "frame", [([x, yc, 0.0], w, t, 2.6) for x, yc, w, t in SADDLE], SKIN, 0.2), "saddle")
    S.flat("elevation", "saddle", "frame", "body", 0.2, "saddle", SKIN, [[x, yc + t] for x, yc, w, t in SADDLE] + [[x, yc - t] for x, yc, w, t in SADDLE][::-1], [-0.22, 1.01, -0.106], tol=0.0015)
    S.flat("plan", "saddle", "frame", "body", 0.2, "saddle", SKIN, [[x, w] for x, yc, w, t in SADDLE] + [[x, -w] for x, yc, w, t in SADDLE][::-1], [-0.22, 1.01, -0.106], tol=0.0015)
    front = [[0.106 * math.copysign(abs(math.cos(a)) ** (2 / 2.6), math.cos(a)), 0.988 + 0.022 * math.copysign(abs(math.sin(a)) ** (2 / 2.6), math.sin(a))] for a in [2 * math.pi * k / 16 for k in range(16)]]
    S.flat("front", "saddle", "frame", "body", 0.2, "saddle", SKIN, front, [-0.22, 1.01, -0.106], tol=0.0015)
    c = [-0.452, 0.946, 0.0]
    S.put(S.box("saddle-yoke", "frame", [0.03, 0.044, 0.152], c, CHROME, STEEL, role="detail", corner=0.006), "frame-tubes")
    S.swept("saddle-yoke", "frame", [[c[0] - 0.015, c[1] - 0.022], [c[0] + 0.015, c[1] - 0.022], [c[0] + 0.015, c[1] + 0.022], [c[0] - 0.015, c[1] + 0.022]], -0.076, 0.076, "detail", STEEL, "frame-tubes", CHROME, pen=PEN)


def guards(S):
    reg = "mudguards"
    for i, bone, c, a0, a1 in (("fender-r", "frame", R, 0.0, 200.0), ("fender-f", "steer", F, 62.0, 215.0)):
        deg = kit.BONES[bone][1]
        o = kit.loc(bone, c)
        cs, sn = math.cos(math.radians(a0 - deg)), math.sin(math.radians(a0 - deg))
        s = S.solid(i, "revolve", bone, PAINT, GUARD, profile=[[kit.r5(z), kit.r5(r)] for z, r in pt.fender_profile(n=4)], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec([o[0] - (o[0] * cs - o[1] * sn), o[1] - (o[0] * sn + o[1] * cs), 0.0])}
        S.put(s, reg)
        band = kit.arc(c, kit.FENDER_R + 0.0004, a0, a1, 24) + kit.arc(c, kit.FENDER_R - 0.0104, a1, a0, 24)
        S.swept(i, bone, band, -0.024, 0.024, "body", GUARD, reg, PAINT, pen=PEN, tol=0.0008)


def drive(S):
    reg = "drivetrain"
    out = pt.case_outline(5)
    S.put(S.prism("chaincase", "frame", out, pt.CASE_Z[0], pt.CASE_Z[1], PAINT, CASE), reg)
    S.swept("chaincase", "frame", out, pt.CASE_Z[0], pt.CASE_Z[1], "body", CASE, reg, PAINT, pen=0.18, tol=0.001)
    S.put(S.cyl("crank-axle", "crank", [B[0], B[1], -0.09], [B[0], B[1], 0.09], 0.0125, CHROME, STEEL, role="detail"), reg)
    S.struct_of("crank-axle", "crank")
    S.disc("crank-axle", "crank", B, 0.025, -0.09, 0.09, "detail", STEEL, reg, CHROME, pen=PEN)
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        z = side * 0.084
        rows = [(-0.014, 0.012), (0.0, 0.016), (0.09, 0.011), (0.17, 0.011), (0.182, 0.008)]
        S.put(S.tube(f"crank-arm-{name}", "crank", [([B[0] + sign * x, B[1], z], 0.0075, h, 3.0) for x, h in rows], PAINT, 0.35), reg)
        S.struct_of(f"crank-arm-{name}", "crank")
        S.swept(f"crank-arm-{name}", "crank", [[B[0] + sign * x, B[1] + h] for x, h in rows] + [[B[0] + sign * x, B[1] - h] for x, h in rows][::-1], z - 0.0075, z + 0.0075, "body", 0.35, reg, PAINT, pen=PEN)
        bone = f"pedal-{name}"
        e = [B[0] + sign * kit.CRANK, B[1], 0.0]
        zc = side * 0.147
        S.put(S.box(f"pedal-{name}", bone, [0.08, 0.024, 0.098], [e[0], e[1], zc], RUBBER, 0.15, corner=0.005), reg)
        S.swept(f"pedal-{name}", bone, [[e[0] - 0.04, e[1] - 0.012], [e[0] + 0.04, e[1] - 0.012], [e[0] + 0.04, e[1] + 0.012], [e[0] - 0.04, e[1] + 0.012]], zc - 0.049, zc + 0.049, "body", 0.15, reg, RUBBER, pen=PEN)
        S.struct(S.cyl(f"pedal-spindle-{name}", bone, [e[0], e[1], side * 0.09], [e[0], e[1], side * 0.196], 0.0075, CHROME, 0.5), bone)
    S.struct(S.cyl("bb-bearing", "crank", [B[0], B[1], -0.034], [B[0], B[1], 0.034], 0.0165, CHROME, 0.5), "crank", "joint")


def fork(S):
    reg = "steer-assembly"
    S.struct(S.pipe("steerer", "steer", sp(-0.012, 0), sp(pt.CROWN_U, 0), 0.0127, 0.0016, PAINT, 0.4), "steer")
    S.struct(S.pipe("head-bearing", "steer", sp(-0.004, 0), sp(0.246, 0), 0.0182, 0.004, CHROME, 0.5), "steer", "joint")
    a, b = sp(pt.CROWN_U, 0, -0.058), sp(pt.CROWN_U, 0, 0.058)
    S.put(S.cap("fork-crown", "steer", a, b, 0.013, PAINT, TUBE), reg)
    S.struct_of("fork-crown", "steer")
    S.disc("fork-crown", "steer", sp(pt.CROWN_U, 0), 0.026, -0.071, 0.071, "body", TUBE, reg, PAINT, pen=PEN)
    blade = [pt.BLADE[k] for k in (0, 2, 4, 6, 8, 9)]
    for side, name in ((1, "l"), (-1, "r")):
        S.put(S.tube(f"fork-{name}", "steer", [(sp(u, v, side * z), rz, rp) for u, v, z, rz, rp in blade], PAINT, TUBE), reg)
        S.struct_of(f"fork-{name}", "steer")
        span = sorted((side * 0.0316, side * 0.0523))
        tip = sorted((side * 0.0447, side * 0.0588))
        S.swept(f"fork-{name}", "steer", kit.ribbon([sp(u, v)[:2] for u, v, z, rz, rp in blade], [rp for u, v, z, rz, rp in blade]), span[0], span[1], "body", TUBE, reg, PAINT, pen=PEN, tip=tip, tol=0.001)


def cockpit(S):
    reg = "steer-assembly"
    S.put(S.cyl("stem-quill", "steer", sp(-0.004, 0), pt.QUILL_TOP, 0.0111, CHROME, STEEL), reg)
    S.struct_of("stem-quill", "steer")
    S.plank("stem-quill", "steer", sp(-0.004, 0), pt.QUILL_TOP, 0.0222, 0.0, "body", STEEL, reg, CHROME, pen=PEN)
    S.put(S.cap("stem-head", "steer", pt.QUILL_TOP, pt.CLAMP, 0.014, CHROME, STEEL), reg)
    S.struct_of("stem-head", "steer")
    S.plank("stem-head", "steer", kit.add(pt.QUILL_TOP, kit.unit(kit.sub(pt.QUILL_TOP, pt.CLAMP)), 0.014), kit.add(pt.CLAMP, kit.unit(kit.sub(pt.CLAMP, pt.QUILL_TOP)), 0.014), 0.028, 0.0, "body", STEEL, reg, CHROME, pen=PEN)
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.bar_path(side, 3)
        g0 = pt.grip_start(side)
        end = pt.mirror(pt.BAR_END, side)
        S.put(S.tube(f"handlebar-{name}", "steer", [(p, 0.0111, 0.0111) for p in path[:-1] + [kit.lerp(g0, end, 0.5)]], CHROME, STEEL, hint=[0.0, 1.0, 0.0]), reg)
        knots = [path[0], path[1], path[2], path[3], path[4], g0, end]
        for k in range(len(knots) - 1):
            S.struct(S.cyl(f"handlebar-{name}-{k + 1}", "steer", knots[k], knots[k + 1], 0.0111, CHROME, 0.5), "steer")
            if k < 5:
                S.member(f"handlebar-{name}-{k + 1}", "steer", knots[k], knots[k + 1], 0.0222, "body", STEEL, reg, CHROME, pen=PEN)
        S.put(S.cap(f"grip-{name}", "steer", g0, kit.add(end, kit.unit(kit.sub(g0, end)), 0.016), 0.016, RUBBER, DARK), reg)
        S.member(f"grip-{name}", "steer", g0, end, 0.032, "body", DARK, reg, RUBBER, pen=PEN)
    g0 = pt.grip_start(1)
    d = kit.unit(kit.sub(pt.mirror(pt.BAR_END, 1), g0))
    base = kit.add(kit.add(g0, d, -0.075), [0.0, 0.009, 0.0])
    S.put(S.rev("bell", "steer", [(0.0, 0.026), (0.016, 0.026), (0.03, 0.012)], base, [0, 1, 0], CHROME, 0.8, role="detail"), reg)
    S.member("bell", "steer", base, kit.add(base, [0, 0.028, 0]), 0.052, "detail", 0.8, reg, CHROME, pen=PEN)
    S.struct(S.ball("mass-steer", "steer", [0.03, 0.03, 0.03], sp(0.3161, -0.0086), PAINT, 0.5), "steer", "mass")
    S.struct(S.box("contact-pad-f", "steer", [0.04, 0.015, 0.037], [F[0], 0.0075, 0.0], RUBBER, 0.1, rotation=kit.rot_w(70)), "steer", "support", anchor="contact-f")


def lamps(S):
    reg = "lamps"
    ax = pt.LAMP_AXIS
    S.put(S.rev("head-lamp", "steer", [(-0.062, 0.009), (-0.03, 0.016), (-0.012, 0.033), (0.02, 0.039), (0.03, 0.036), (0.036, 0.016)], pt.LAMP, ax, PLASTIC, 0.3), reg)
    S.member("head-lamp", "steer", kit.add(pt.LAMP, ax, -0.012), kit.add(pt.LAMP, ax, 0.034), 0.076, "body", 0.3, reg, PLASTIC, pen=PEN)
    S.plank("head-lamp-neck", "steer", kit.add(pt.LAMP, ax, -0.062), kit.add(pt.LAMP, ax, -0.012), 0.02, 0.0, "body", 0.3, reg, PLASTIC, pen=PEN, d1=0.06, wide=0.03)
    c = kit.polar(R, kit.FENDER_R + 0.018, pt.TAIL_DEG)
    S.put(S.box("tail-lamp", "frame", [0.036, 0.05, 0.045], c, PLASTIC, 0.3, corner=0.006, rotation=kit.rot_w(pt.TAIL_DEG)), reg)
    e = [math.cos(math.radians(pt.TAIL_DEG)), math.sin(math.radians(pt.TAIL_DEG))]
    t = [-e[1], e[0]]
    S.swept("tail-lamp", "frame", [[c[0] + e[0] * a + t[0] * b, c[1] + e[1] * a + t[1] * b] for a, b in ((-0.018, -0.025), (0.018, -0.025), (0.018, 0.025), (-0.018, 0.025))], -0.0225, 0.0225, "body", 0.3, reg, PLASTIC, pen=PEN)


def stand(S):
    reg = "frame-tubes"
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    S.put(S.cyl("stand-pivot", "stand", at(0, -0.036), at(0, 0.036), 0.011, PAINT, TUBE, role="detail"), reg)
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.03), at(0, 0.03), 0.0075, CHROME, 0.5), "stand", "joint")
    S.disc("stand-pivot", "stand", P, 0.022, -0.036, 0.036, "detail", TUBE, reg, PAINT, pen=PEN)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = at(0.004, side * 0.03), at(kit.STAND_LENGTH - 0.009, side * 0.09)
        S.put(S.cap(f"stand-leg-{name}", "stand", a, b, 0.009, PAINT, TUBE), reg)
        S.struct_of(f"stand-leg-{name}", "stand")
        S.member(f"stand-leg-{name}", "stand", a, at(kit.STAND_LENGTH, side * 0.09), 0.018, "body", TUBE, reg, PAINT, pen=PEN)


def build():
    S = kit.Scale(100, PENS)
    wheel(S, "wheel-r", R, (-0.034, 0.056, 0.04))
    wheel(S, "wheel-f", F, (-0.038, 0.042, 0.045))
    frame(S)
    saddle(S)
    guards(S)
    drive(S)
    fork(S)
    cockpit(S)
    lamps(S)
    stand(S)
    return S


DECISIONS = [
    {
        "en": "1:100 is authored anew for the 15 mm feature: the frame keeps every tube on its real centre line (seat tube with the lug merged, post, down tube, loop tube with six sections, head tube with the headset cups merged, bottom bracket shell, one even section per stay), the carrier is one hoop of round rod that runs from the seat lug around the rear and back, on four legs, the saddle has six sections and its springs are one yoke block, each wheel is a tyre, a plain rim ring, one hub shell that includes the brake unit, an axle with its nuts and twelve spokes, the mudguards keep their sheet section without stays, chain case, crank axle with both bosses, crank arms and one block per pedal, fork crown as a round bar with six-section blades, quill stem with head and clamp in one, the swept-back handlebar with grips and a plain bell, the head lamp with its bracket as one housing, the tail lamp, and the stand as a pivot and two straight legs. Dropouts, bridges, saddle rails, mudguard stays, brake levers, fork ends and lamp lenses are below the feature and left out.",
        "de": "1:100 ist für das 15-mm-Merkmal neu aufgebaut: der Rahmen behält jedes Rohr auf seiner wirklichen Mittellinie (Sattelrohr mit eingeschmolzener Muffe, Stütze, Unterrohr, Bügelrohr mit sechs Schnitten, Steuerrohr mit eingeschmolzenen Steuersatzschalen, Tretlagergehäuse, je Strebe ein gleichmäßiger Querschnitt), der Gepäckträger besteht aus einem Rundstabbügel, der von der Sattelmuffe um das Heck und zurück läuft, auf vier Beinen, der Sattel hat sechs Schnitte und seine Federn sind ein Jochblock, jedes Rad ist ein Reifen, ein schlichter Felgenring, ein Nabenkörper samt Bremseinheit, eine Achse mit ihren Muttern und zwölf Speichen, die Schutzbleche behalten ihren Blechquerschnitt ohne Streben, Kettenkasten, Kurbelachse mit beiden Naben, Kurbelarme und ein Block je Pedal, Gabelkrone als Rundstab mit Scheiden aus sechs Schnitten, Schaftvorbau mit Kopf und Klemme in einem, der zurückgeschwungene Lenker mit Griffen und schlichter Klingel, der Scheinwerfer mit seinem Halter als ein Gehäuse, das Rücklicht und der Ständer als Gelenk mit zwei geraden Beinen. Ausfallenden, Stege, Sattelgestell, Schutzblechstreben, Bremshebel, Gabelenden und Lampengläser liegen unter dem Merkmal und entfallen.",
    },
    {
        "en": "Thermal: one reference per body solid in its region; the 0.06 mm paint film and the steel walls of 1:50 are not separate at this scale. All nine regions are covered.",
        "de": "Thermik: ein Verweis je Körper in seiner Region; der 0,06-mm-Lackfilm und die Stahlwände von 1:50 sind in diesem Maßstab nicht getrennt. Alle neun Regionen sind abgedeckt.",
    },
    {
        "en": "Structure: the frame tubes, stays and carrier bars themselves, the hollow steerer in the head tube with one headset bearing sleeve, fork crown and blades, quill and stem head, the handlebar as six round bars per side, rim, hub shell and twelve spokes per wheel with the hub bearing, crank axle with arms and the bottom bracket bearing, a 15 mm spindle per pedal, both stand legs with the hinge, the front tyre patch as support and the masses of frame, steer assembly and wheels.",
        "de": "Tragwerk: die Rahmenrohre, Streben und Gepäckträgerstäbe selbst, der hohle Gabelschaft im Steuerrohr mit einer Steuersatzhülse, Gabelkrone und Scheiden, Schaft und Vorbaukopf, der Lenker als sechs Rundstäbe je Seite, Felge, Nabenkörper und zwölf Speichen je Rad mit dem Nabenlager, Kurbelachse mit Armen und dem Tretlager, eine 15-mm-Achse je Pedal, beide Ständerbeine mit dem Gelenk, die vordere Reifenaufstandsfläche als Auflager und die Massen von Rahmen, Lenkeinheit und Rädern.",
    },
]

DRAWN = {
    "silhouette": {
        "en": "The true orthographic views of the 1:100 body. Side: tyre and rim rings with twelve spokes and one hub shell, the frame tubes at their real width, carrier rails and legs, saddle on its spring yoke, chain case, cranks with pedal blocks, mudguards, fork, stem, swept-back handlebar with grips and bell, both lamps and the folded centre stand. Front and top show the same parts.",
        "de": "Die wahren orthogonalen Ansichten des 1:100-Körpers. Seite: Reifen- und Felgenringe mit zwölf Speichen und einem Nabenkörper, die Rahmenrohre in wirklicher Breite, Schienen und Beine des Gepäckträgers, Sattel auf seinem Federjoch, Kettenkasten, Kurbeln mit Pedalblöcken, Schutzbleche, Gabel, Vorbau, zurückgeschwungener Lenker mit Griffen und Klingel, beide Leuchten und der hochgeklappte Zweibeinständer. Vorder- und Draufsicht zeigen dieselben Teile.",
    },
    "retained": {"en": "Every part of the body: all frame tubes, stays, carrier, saddle, wheels with hubs, mudguards, chain case, cranks, pedals, fork, stem, handlebar, grips, bell, lamps and stand.", "de": "Jedes Teil des Körpers: alle Rahmenrohre, Streben, Gepäckträger, Sattel, Räder mit Naben, Schutzbleche, Kettenkasten, Kurbeln, Pedale, Gabel, Vorbau, Lenker, Griffe, Klingel, Leuchten und Ständer."},
    "merged": {"en": "Headset cups into the head tube, seat lug into the seat tube, saddle springs into one yoke, hub and brake unit into one shell, axle and nuts into one bar, pedal blocks and plates into one block, lamp, lens and bracket into one housing, stand leg and foot into one bar; the 36 real spokes are twelve.", "de": "Steuersatzschalen im Steuerrohr, Sattelmuffe im Sattelrohr, Sattelfedern in einem Joch, Nabe und Bremseinheit in einem Körper, Achse und Muttern in einem Stab, Pedalblöcke und -platten in einem Block, Lampe, Glas und Halter in einem Gehäuse, Ständerbein und Fuß in einem Stab; aus den 36 wirklichen Speichen werden zwölf."},
    "omitted": {"en": "Dropouts, bridges, saddle rails, mudguard stays, brake levers, fork ends, chain, cables, tread, reflectors.", "de": "Ausfallenden, Stege, Sattelgestell, Schutzblechstreben, Bremshebel, Gabelenden, Kette, Züge, Profil, Reflektoren."},
    "thinElements": {"en": "Tubes are filled bars at their real width (0.29 to 0.32 mm on paper, stays 0.15 to 0.19 mm, carrier rods 0.1 mm) with a 0.13 mm outline; spokes are 0.13 mm strokes; mudguards are 0.1 mm bands.", "de": "Rohre sind gefüllte Stäbe in wirklicher Breite (0,29 bis 0,32 mm auf dem Papier, Streben 0,15 bis 0,19 mm, Gepäckträgerstäbe 0,1 mm) mit 0,13 mm Kontur; Speichen sind 0,13-mm-Striche; Schutzbleche sind 0,1-mm-Bänder."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Near black tyres, grips and pedals, light rims and chrome, mid-tone tubes, lighter mudguards and chain case, dark saddle.", "de": "Fast schwarze Reifen, Griffe und Pedale, helle Felgen und Chrom, mitteltonige Rohre, hellere Schutzbleche und Kettenkasten, dunkler Sattel."},
    "overlays": {"en": "Every shape carries the thermal region of its part. Straight tubes are members, plates and bent tubes extruded profiles, so one definition gives side, front and top and follows steering (a twist about the head tube axis), rolling wheels, turning cranks and the folding stand; only the saddle has an outline per view.", "de": "Jede Form trägt die thermische Region ihres Teils. Gerade Rohre sind Stäbe, Platten und gebogene Rohre extrudierte Profile; so liefert eine Definition Seite, Vorderansicht und Draufsicht und folgt dem Lenken (einer Drehung um die Steuerrohrachse), den rollenden Rädern, den drehenden Kurbeln und dem klappbaren Ständer; nur der Sattel hat je Ansicht eine Kontur."},
}
