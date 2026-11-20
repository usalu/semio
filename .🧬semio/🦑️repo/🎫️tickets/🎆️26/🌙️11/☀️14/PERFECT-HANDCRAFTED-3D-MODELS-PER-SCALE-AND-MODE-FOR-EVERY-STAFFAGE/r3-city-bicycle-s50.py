"""🚲 Round 3 vehicles/city-bicycle at 1:50 (smallest feature 7.5 mm): every assembly of the real Dutch city bicycle stated by hand, as body solid, thermal entry, structural part and drawn shape in one place."""
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
PENS = {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}


def wheel(S, bone, c, hub, flanges, shell, brake, axle_half, nut_half):
    tag = bone[-1]
    S.put(S.rev(f"tyre-{tag}", bone, pt.tyre_profile(), c, [0, 0, 1], RUBBER, DARK), "tyres")
    outer, inner = pt.ring(c, 0.345, 0.308)
    S.swept(f"tyre-{tag}", bone, outer, -0.0185, 0.0185, "body", DARK, "tyres", RUBBER, holes=[inner], tol=0.0003, views=SIDE)
    for k, half in enumerate(pt.half_rings(c, 0.345, 0.308, 24)):
        S.swept(f"tyre-{tag}-{k + 1}", bone, half, -0.0185, 0.0185, "body", DARK, "tyres", RUBBER, tol=0.0003, views=("front", "plan"))
    S.put(S.rev(f"rim-{tag}", bone, pt.RIM, c, [0, 0, 1], ALLOY, 0.78), "rims-spokes")
    S.struct_of(f"rim-{tag}", bone)
    outer, inner = pt.ring(c, 0.3175, 0.297)
    S.swept(f"rim-{tag}", bone, outer, -0.0125, 0.0125, "body", 0.78, "rims-spokes", ALLOY, pen=0.13, holes=[inner], tol=0.0003, views=SIDE)
    S.put(S.rev(f"hub-{tag}", bone, hub, c, [0, 0, 1], CHROME, STEEL), "hub-brakes")
    S.struct_of(f"hub-{tag}", bone)
    lo, hi = hub[0][0], hub[-1][0]
    S.disc(f"hub-{tag}", bone, c, shell[2], shell[0], shell[1], "detail", STEEL, "hub-brakes", CHROME, pen=0.13)
    S.disc(f"brake-{tag}", bone, c, brake[2], brake[0], brake[1], "detail", 0.45, "hub-brakes", CHROME, pen=0.13)
    S.put(S.cyl(f"axle-{tag}", bone, [c[0], c[1], -axle_half], [c[0], c[1], axle_half], 0.005, CHROME, STEEL, role="detail"), "hub-brakes")
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], lo], [c[0], c[1], hi], 0.011, CHROME, STEEL), bone, "joint")
    for side, name in ((1, "l"), (-1, "r")):
        S.put(S.cyl(f"nut-{tag}-{name}", bone, [c[0], c[1], side * (nut_half - 0.004)], [c[0], c[1], side * (nut_half + 0.004)], 0.009, CHROME, STEEL, role="detail"), "hub-brakes")
    S.disc(f"axle-{tag}", bone, c, 0.018, -nut_half - 0.004, nut_half + 0.004, "detail", STEEL, "hub-brakes", CHROME, pen=0.13)
    k = 0
    for fz, fr, off in flanges:
        for lead in (True, False):
            k += 1
            a, b = pt.spoke(fr, fz, off + (0 if lead else 20), lead)
            s = S.cap(f"spoke-{tag}-{k}", bone, kit.add(c, a), kit.add(c, b), 0.001, ALLOY, 0.6, role="texture", slender=True, texture=True, repeat={"count": 9, "rotationAxis": [0, 0, 1], "rotationRad": round(math.radians(40), 6), "origin": [0, 0, 0]})
            S.put(s, "rims-spokes")
            S.struct_of(f"spoke-{tag}-{k}", bone)
    fz, fr, off = flanges[1]
    for j in range(18):
        lead = j % 2 == 0
        a, b = pt.spoke(fr, fz, off + 20 * j, lead)
        S.sliver(f"spoke-{tag}-{j + 1}", bone, kit.add(c, a), kit.add(c, b), "texture", 0.6, "rims-spokes", ALLOY, 0.13)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.03, 0.03, 0.03], c, CHROME, 0.5), bone, "mass")


def tube2d(S, i, bone, a, b, d, region, tone=TUBE, mat=PAINT, role="body", pen=0.18, views=ALL):
    S.member(i, bone, a, b, d, role, tone, region, mat, pen=pen, views=views)


def frame(S):
    reg = "frame-tubes"
    S.put(S.cyl("bb-shell", "frame", [B[0], B[1], -0.034], [B[0], B[1], 0.034], 0.02, PAINT, TUBE), reg)
    S.struct(S.pipe("bb-shell", "frame", [B[0], B[1], -0.034], [B[0], B[1], 0.034], 0.02, 0.003, PAINT, 0.4), "frame")
    S.disc("bb-shell", "frame", B, 0.04, -0.034, 0.034, "body", TUBE, reg, PAINT, pen=0.18)
    for i, a, b, r, wall in (("seat-tube", B, pt.SEAT_TOP, 0.0143, 0.0012), ("down-tube", pt.DOWN_HEAD, B, 0.0159, 0.0012), ("head-tube", pt.HEAD_TOP, pt.HEAD_BOTTOM, 0.017, 0.0015)):
        S.put(S.cyl(i, "frame", a, b, r, PAINT, TUBE), reg)
        S.struct(S.pipe(i, "frame", a, b, r, wall, PAINT, 0.4), "frame")
        tube2d(S, i, "frame", a, b, 2 * r, reg)
    post0 = kit.lerp(B, pt.SEAT_TOP, 0.9)
    S.put(S.cyl("seat-post", "frame", post0, pt.POST_TOP, 0.0127, CHROME, STEEL), reg)
    S.struct(S.pipe("seat-post", "frame", post0, pt.POST_TOP, 0.0127, 0.0015, CHROME, 0.5), "frame")
    tube2d(S, "seat-post", "frame", pt.SEAT_TOP, pt.POST_TOP, 0.0254, reg, tone=STEEL, mat=CHROME, pen=0.13)
    lug0 = kit.lerp(B, pt.SEAT_TOP, 0.945)
    lug1 = kit.add(pt.SEAT_TOP, [pt.ST[0], pt.ST[1], 0], 0.004)
    S.put(S.cyl("seat-lug", "frame", lug0, lug1, 0.0172, PAINT, TUBE, role="detail"), reg)
    S.plank("seat-lug", "frame", lug0, lug1, 0.0344, 0.0, "detail", TUBE, reg, PAINT, pen=0.13)
    S.put(S.tube("loop-tube", "frame", [(p, 0.0143, 0.0143) for p in pt.LOOP], PAINT, TUBE), reg)
    S.struct_of("loop-tube", "frame")
    S.swept("loop-tube", "frame", kit.ribbon(pt.LOOP, 0.0143), -0.0143, 0.0143, "body", TUBE, reg, PAINT, pen=0.18)
    for i, u0, u1 in (("headset-top", -0.004, 0.012), ("headset-bottom", 0.232, 0.246)):
        S.put(S.cyl(i, "frame", sp(u0, 0), sp(u1, 0), 0.0198, CHROME, STEEL, role="detail"), reg)
        S.plank(i, "frame", sp(u0, 0), sp(u1, 0), 0.0396, 0.0, "detail", STEEL, reg, CHROME, pen=0.13)
    stays = pt.stay_points()
    for side, name in ((1, "l"), (-1, "r")):
        for key, (a, b, r0, r1) in stays.items():
            i = f"{key}-{name}"
            a, b = pt.mirror(a, side), pt.mirror(b, side)
            S.put(S.cyl(i, "frame", a, b, r0, PAINT, TUBE, r1=r1), reg)
            S.struct_of(i, "frame")
            S.member(i, "frame", a, b, r0 + r1, "body", TUBE, reg, PAINT, pen=0.13, depth=[max(a[0], b[0]), max(a[1], b[1]), side * 0.03] if key == "chainstay" else None)
        z0, z1 = sorted((side * 0.060, side * 0.068))
        plate = [[R[0] + dx, R[1] + dy] for dx, dy in ((0.024, -0.01), (0.026, 0.008), (0.016, 0.026), (-0.006, 0.028), (-0.018, 0.012), (-0.018, -0.01), (-0.006, -0.02), (0.012, -0.02))]
        S.put(S.prism(f"dropout-{name}", "frame", plate, z0, z1, PAINT, TUBE, role="detail"), reg)
        S.struct_of(f"dropout-{name}", "frame")
        S.swept(f"dropout-{name}", "frame", plate, z0, z1, "detail", TUBE, reg, PAINT, pen=0.13)
    cb = [P[0] + 0.012, 0.3075]
    S.put(S.cyl("chainstay-bridge", "frame", [cb[0], cb[1], -0.036], [cb[0], cb[1], 0.036], 0.006, PAINT, TUBE, role="detail"), reg)
    S.disc("chainstay-bridge", "frame", cb, 0.012, -0.036, 0.036, "detail", TUBE, reg, PAINT, pen=0.13, views=("front", "plan"))
    a, b = stays["seatstay"][0], stays["seatstay"][1]
    sb = kit.lerp(a, b, 0.24)
    S.put(S.cyl("seatstay-bridge", "frame", [sb[0], sb[1], -sb[2]], [sb[0], sb[1], sb[2]], 0.006, PAINT, TUBE, role="detail"), reg)
    S.disc("seatstay-bridge", "frame", sb, 0.012, -sb[2], sb[2], "detail", TUBE, reg, PAINT, pen=0.13, views=("front", "plan"))
    bracket = [[P[0] - 0.015, P[1] - 0.006], [P[0] + 0.015, P[1] - 0.006], [P[0] + 0.032, 0.298], [P[0] - 0.01, 0.298]]
    S.put(S.prism("stand-bracket", "frame", bracket, -0.026, 0.026, PAINT, TUBE, role="detail"), reg)
    S.swept("stand-bracket", "frame", bracket, -0.026, 0.026, "detail", TUBE, reg, PAINT, pen=0.13)
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.045, 0.045, 0.045], [fx, fy, 0], PAINT, 0.5), "frame", "mass")


def carrier(S):
    reg = "frame-tubes"
    x0, x1 = pt.RACK_X
    y, z = pt.RACK_Y, pt.RACK_Z
    bars = {"rack-rail-l": ([x0, y, z], [x1, y, z]), "rack-rail-r": ([x0, y, -z], [x1, y, -z]), "rack-spine": ([x0, y, 0], [x1, y, 0])}
    for k, x in enumerate((x0, -0.58, -0.72, x1)):
        bars[f"rack-bar-{k + 1}"] = ([x, y, -z], [x, y, z])
    for side, name in ((1, "l"), (-1, "r")):
        eye = [R[0], R[1] + 0.014, side * 0.073]
        bars[f"rack-leg-a-{name}"] = ([-0.6, y, side * z], eye)
        bars[f"rack-leg-b-{name}"] = ([-0.8, y, side * z], eye)
        bars[f"rack-strut-{name}"] = ([x0, y, side * 0.05], [pt.SEAT_TOP[0] - 0.012, pt.SEAT_TOP[1] - 0.02, side * 0.018])
    for i, (a, b) in bars.items():
        S.put(S.cap(i, "frame", a, b, 0.005, PAINT, TUBE, role="detail" if "bar" in i or "spine" in i else "body"), reg)
        if "bar" not in i:
            S.struct_of(i, "frame")
            tube2d(S, i, "frame", a, b, 0.01, reg, role="detail", pen=0.13, views=("plan",) if "spine" in i else ALL)
        else:
            S.disc(i, "frame", a, 0.01, a[2], b[2], "detail", TUBE, reg, PAINT, pen=0.13, filled=False, views=("plan",))


def saddle(S):
    rows = [([x, yc, 0.0], w, t, 2.6) for x, yc, w, t in pt.SADDLE]
    S.put(S.tube("saddle", "frame", rows, SKIN, 0.2), "saddle")
    top = [[x, yc + t] for x, yc, w, t in pt.SADDLE]
    bottom = [[x, yc - t] for x, yc, w, t in pt.SADDLE]
    S.flat("elevation", "saddle", "frame", "body", 0.2, "saddle", SKIN, top + bottom[::-1], [-0.22, 1.01, -0.107])
    S.flat("plan", "saddle", "frame", "body", 0.2, "saddle", SKIN, [[x, w] for x, yc, w, t in pt.SADDLE] + [[x, -w] for x, yc, w, t in pt.SADDLE][::-1], [-0.22, 1.01, -0.107])
    x, yc, w, t = pt.SADDLE[6]
    front = [[w * math.copysign(abs(math.cos(a)) ** (2 / 2.6), math.cos(a)), 0.988 + 0.024 * math.copysign(abs(math.sin(a)) ** (2 / 2.6), math.sin(a))] for a in [2 * math.pi * k / 28 for k in range(28)]]
    S.flat("front", "saddle", "frame", "body", 0.2, "saddle", SKIN, front, [-0.22, 1.01, -0.107])
    reg = "frame-tubes"
    for side, name in ((1, "l"), (-1, "r")):
        a, b = [-0.455, 0.925, side * 0.062], [-0.455, 0.969, side * 0.062]
        S.put(S.cyl(f"saddle-spring-{name}", "frame", a, b, 0.014, CHROME, STEEL, role="detail"), reg)
        S.plank(f"saddle-spring-{name}", "frame", a, b, 0.028, side * 0.062, "detail", STEEL, reg, CHROME, pen=0.13)
        a, b = [-0.236, 0.974, side * 0.012], [-0.455, 0.925, side * 0.062]
        S.put(S.cap(f"saddle-rail-{name}", "frame", a, b, 0.004, CHROME, STEEL, role="detail"), reg)
        S.wire(f"saddle-rail-{name}", a, b, "detail", STEEL, reg, CHROME, 0.13)
    a, b = [-0.455, 0.925, -0.062], [-0.455, 0.925, 0.062]
    S.put(S.cap("saddle-yoke", "frame", a, b, 0.004, CHROME, STEEL, role="detail"), reg)
    S.wire("saddle-yoke", a, b, "detail", STEEL, reg, CHROME, 0.13, views=("front", "plan"))
    c = [pt.POST_TOP[0] + 0.004, pt.POST_TOP[1] + 0.016, 0]
    S.put(S.box("saddle-clamp", "frame", [0.04, 0.03, 0.052], c, CHROME, STEEL, role="detail", corner=0.006), reg)
    S.swept("saddle-clamp", "frame", [[c[0] - 0.02, c[1] - 0.015], [c[0] + 0.02, c[1] - 0.015], [c[0] + 0.02, c[1] + 0.015], [c[0] - 0.02, c[1] + 0.015]], -0.026, 0.026, "detail", STEEL, reg, CHROME, pen=0.13)


def guards(S):
    reg = "mudguards"
    for i, bone, c, a0, a1 in (("fender-r", "frame", R, 0.0, 200.0), ("fender-f", "steer", F, 62.0, 215.0)):
        deg = BONE_DEG[bone]
        o = kit.loc(bone, c)
        s = S.solid(i, "revolve", bone, PAINT, GUARD, profile=[[kit.r5(z), kit.r5(r)] for z, r in pt.fender_profile()], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec(turn_offset(o, a0 - deg))}
        w = dict(s)
        w["profile"] = [[kit.r5(z), kit.r5(r)] for z, r in pt.fender_profile(half=0.02394, t=0.0006)]
        S.put(s, reg, inner=w)
        band = kit.arc(c, kit.FENDER_R + 0.0004, a0, a1, 40) + kit.arc(c, kit.FENDER_R - 0.0104, a1, a0, 40)
        S.swept(i, bone, band, -0.024, 0.024, "body", GUARD, reg, PAINT, pen=0.18, tol=0.0004)
    for side, name in ((1, "l"), (-1, "r")):
        for k, deg in enumerate((150.0, 195.0)):
            a = [R[0] - 0.012, R[1] + 0.004, side * 0.071]
            b = kit.polar(R, kit.FENDER_R - 0.009, deg, side * 0.024)
            i = f"fender-stay-r{k + 1}-{name}"
            S.put(S.cap(i, "frame", a, b, 0.002, CHROME, STEEL, role="detail", slender=True), reg)
            S.wire(i, a, b, "detail", STEEL, reg, CHROME, 0.13)
        a = [F[0], F[1], side * 0.058]
        b = kit.polar(F, kit.FENDER_R - 0.009, 200.0, side * 0.024)
        i = f"fender-stay-f-{name}"
        S.put(S.cap(i, "steer", a, b, 0.002, CHROME, STEEL, role="detail", slender=True), reg)
        S.member(i, "steer", a, b, 0.004, "detail", STEEL, reg, CHROME, pen=0.13, filled=False)


BONE_DEG = {k: v[1] for k, v in kit.BONES.items()}


def turn_offset(o, deg):
    """↩️ The translation that keeps the point o fixed when a solid is turned by deg about the bone origin's w axis."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [o[0] - (o[0] * c - o[1] * s), o[1] - (o[0] * s + o[1] * c), 0.0]


def drive(S):
    reg = "drivetrain"
    out = pt.case_outline()
    S.put(S.prism("chaincase", "frame", out, pt.CASE_Z[0], pt.CASE_Z[1], PAINT, CASE), reg)
    S.swept("chaincase", "frame", out, pt.CASE_Z[0], pt.CASE_Z[1], "body", CASE, reg, PAINT, pen=0.18, tol=0.0005)
    S.put(S.cyl("crank-axle", "crank", [B[0], B[1], -0.078], [B[0], B[1], 0.078], 0.0085, CHROME, STEEL, role="detail"), reg)
    S.struct_of("crank-axle", "crank")
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        z = side * pt.ARM_Z
        rows = [(-0.02, 0.012), (-0.012, 0.017), (0.0, 0.018), (0.02, 0.015), (0.08, 0.0115), (0.15, 0.0095), (0.17, 0.0115), (0.181, 0.008)]
        S.put(S.tube(f"crank-arm-{name}", "crank", [([B[0] + sign * x, B[1], z], 0.006, h, 3.0) for x, h in rows], PAINT, 0.35), reg)
        S.struct_of(f"crank-arm-{name}", "crank")
        S.swept(f"crank-arm-{name}", "crank", [[B[0] + sign * x, B[1] + h] for x, h in rows] + [[B[0] + sign * x, B[1] - h] for x, h in rows][::-1], z - 0.006, z + 0.006, "body", 0.35, reg, PAINT, pen=0.13)
        a, b = [B[0], B[1], z - side * 0.006], [B[0], B[1], side * 0.066 if side < 0 else side * 0.034]
        S.put(S.cyl(f"crank-boss-{name}", "crank", a, b, 0.0125, CHROME, STEEL, role="detail"), reg)
        S.disc(f"crank-boss-{name}", "crank", B, 0.025, a[2], b[2], "detail", STEEL, reg, CHROME, pen=0.13)
        bone = f"pedal-{name}"
        e = [B[0] + sign * kit.CRANK, B[1], 0.0]
        zi, zo = side * 0.09, side * 0.198
        S.put(S.cyl(f"pedal-spindle-{name}", bone, [e[0], e[1], zi], [e[0], e[1], zo], 0.006, CHROME, STEEL, role="detail"), reg)
        S.struct_of(f"pedal-spindle-{name}", bone)
        S.struct(S.ball(f"pedal-bearing-{name}", bone, [0.009, 0.009, 0.009], [e[0], e[1], side * 0.1], CHROME, 0.5), bone, "joint")
        zc = side * 0.149
        for k, dx in enumerate((-0.027, 0.027)):
            S.put(S.box(f"pedal-block-{name}-{k + 1}", bone, [0.024, 0.024, 0.086], [e[0] + dx, e[1], zc], RUBBER, 0.15, corner=0.004), reg)
            S.swept(f"pedal-block-{name}-{k + 1}", bone, [[e[0] + dx - 0.012, e[1] - 0.012], [e[0] + dx + 0.012, e[1] - 0.012], [e[0] + dx + 0.012, e[1] + 0.012], [e[0] + dx - 0.012, e[1] + 0.012]], zc - 0.043, zc + 0.043, "body", 0.15, reg, RUBBER, pen=0.13)
        for k, zz in enumerate((side * 0.102, side * 0.196)):
            S.put(S.box(f"pedal-plate-{name}-{k + 1}", bone, [0.08, 0.02, 0.008], [e[0], e[1], zz], CHROME, STEEL, role="detail", corner=0.003), reg)
            S.swept(f"pedal-plate-{name}-{k + 1}", bone, [[e[0] - 0.04, e[1] - 0.01], [e[0] + 0.04, e[1] - 0.01], [e[0] + 0.04, e[1] + 0.01], [e[0] - 0.04, e[1] + 0.01]], zz - 0.004, zz + 0.004, "detail", STEEL, reg, CHROME, pen=0.13)
    for side in (-1, 1):
        S.struct(S.cyl(f"bb-bearing-{'r' if side < 0 else 'l'}", "crank", [B[0], B[1], side * 0.022], [B[0], B[1], side * 0.034], 0.0165, CHROME, 0.5), "crank", "joint")


def fork(S):
    reg = "steer-assembly"
    S.struct(S.pipe("steerer", "steer", sp(-0.012, 0), sp(pt.CROWN_U, 0), 0.0127, 0.0016, PAINT, 0.4), "steer")
    for u, name in ((0.004, "top"), (0.239, "bottom")):
        S.struct(S.pipe(f"head-bearing-{name}", "steer", sp(u - 0.006, 0), sp(u + 0.006, 0), 0.0185, 0.005, CHROME, 0.5), "steer", "joint")
    crown = [(sp(pt.CROWN_U, 0, z), ru, rv, 3.0) for z, ru, rv in ((-0.06, 0.009, 0.011), (-0.05, 0.012, 0.015), (-0.02, 0.012, 0.017), (0.02, 0.012, 0.017), (0.05, 0.012, 0.015), (0.06, 0.009, 0.011))]
    S.put(S.tube("fork-crown", "steer", crown, PAINT, TUBE, hint=[kit.COS70, -kit.SIN70, 0.0]), reg)
    S.struct_of("fork-crown", "steer")
    S.plank("fork-crown", "steer", sp(pt.CROWN_U - 0.012, 0), sp(pt.CROWN_U + 0.012, 0), 0.032, 0.0, "body", TUBE, reg, PAINT, pen=0.13, wide=0.12)
    for side, name in ((1, "l"), (-1, "r")):
        rows = [(sp(u, v, side * z), rz, rp) for u, v, z, rz, rp in pt.BLADE]
        S.put(S.tube(f"fork-{name}", "steer", rows, PAINT, TUBE), reg)
        S.struct_of(f"fork-{name}", "steer")
        path = [sp(u, v)[:2] for u, v, z, rz, rp in pt.BLADE]
        span = sorted((side * 0.0316, side * 0.0523))
        tip = sorted((side * 0.0447, side * 0.0588))
        S.swept(f"fork-{name}", "steer", kit.ribbon(path, [rp for u, v, z, rz, rp in pt.BLADE]), span[0], span[1], "body", TUBE, reg, PAINT, pen=0.13, tip=tip, tol=0.0004)
        a, b = [F[0], F[1], side * 0.046], [F[0], F[1], side * 0.058]
        S.put(S.cyl(f"fork-end-{name}", "steer", a, b, 0.011, PAINT, TUBE, role="detail"), reg)
        S.struct_of(f"fork-end-{name}", "steer")


def cockpit(S):
    reg = "steer-assembly"
    S.put(S.cyl("stem-quill", "steer", sp(-0.004, 0), pt.QUILL_TOP, 0.0111, CHROME, STEEL), reg)
    S.struct_of("stem-quill", "steer")
    S.plank("stem-quill", "steer", sp(-0.004, 0), pt.QUILL_TOP, 0.0222, 0.0, "body", STEEL, reg, CHROME, pen=0.13)
    S.put(S.cap("stem-head", "steer", pt.QUILL_TOP, pt.CLAMP, 0.0125, CHROME, STEEL), reg)
    S.struct_of("stem-head", "steer")
    S.plank("stem-head", "steer", kit.add(pt.QUILL_TOP, kit.unit(kit.sub(pt.QUILL_TOP, pt.CLAMP)), 0.012), pt.CLAMP, 0.025, 0.0, "body", STEEL, reg, CHROME, pen=0.13)
    a, b = [pt.CLAMP[0], pt.CLAMP[1], -0.019], [pt.CLAMP[0], pt.CLAMP[1], 0.019]
    S.put(S.cyl("stem-clamp", "steer", a, b, 0.0165, CHROME, STEEL, role="detail"), reg)
    S.struct_of("stem-clamp", "steer")
    S.disc("stem-clamp", "steer", pt.CLAMP, 0.033, -0.019, 0.019, "detail", STEEL, reg, CHROME, pen=0.13)
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.bar_path(side)
        g0 = pt.grip_start(side)
        end = pt.mirror(pt.BAR_END, side)
        S.put(S.tube(f"handlebar-{name}", "steer", [(p, 0.0111, 0.0111) for p in path[:-1] + [kit.lerp(g0, end, 0.5)]], CHROME, STEEL, hint=[0.0, 1.0, 0.0]), reg)
        knots = [path[0], path[1], path[3], path[5], path[7], g0, end]
        for k in range(len(knots) - 1):
            S.struct(S.pipe(f"handlebar-{name}-{k + 1}", "steer", knots[k], knots[k + 1], 0.0111, 0.0015, CHROME, 0.5), "steer")
            if k < 5:
                S.member(f"handlebar-{name}-{k + 1}", "steer", knots[k], knots[k + 1], 0.0222, "body", STEEL, reg, CHROME, pen=0.13)
        S.put(S.cyl(f"grip-{name}", "steer", g0, end, 0.016, RUBBER, DARK), reg)
        S.member(f"grip-{name}", "steer", g0, end, 0.032, "body", DARK, reg, RUBBER, pen=0.13)
        d = kit.unit(kit.sub(end, g0))
        S.put(S.cyl(f"grip-flange-{name}", "steer", kit.add(end, d, -0.008), kit.add(end, d, 0.002), 0.019, RUBBER, DARK, role="detail"), reg)
        hinge = kit.add(kit.add(g0, d, -0.03), [0.0, -0.012, side * 0.012])
        S.put(S.cap(f"lever-body-{name}", "steer", kit.add(g0, d, -0.034), hinge, 0.009, PLASTIC, 0.25, role="detail"), reg)
        tip = kit.add(kit.add(g0, d, 0.085), [0.0, -0.03, side * 0.026])
        S.put(S.cap(f"lever-{name}", "steer", hinge, tip, 0.0045, ALLOY, 0.75, role="detail"), reg)
        S.member(f"lever-{name}", "steer", hinge, tip, 0.009, "detail", 0.75, reg, ALLOY, pen=0.13)
    g0 = pt.grip_start(1)
    d = kit.unit(kit.sub(pt.mirror(pt.BAR_END, 1), g0))
    base = kit.add(kit.add(g0, d, -0.075), [0.0, 0.011, 0.0])
    S.put(S.rev("bell", "steer", [(0.0, 0.027), (0.012, 0.027), (0.024, 0.021), (0.03, 0.011), (0.032, 0.001)], base, [0, 1, 0], CHROME, 0.8, role="detail"), reg)
    S.member("bell", "steer", base, kit.add(base, [0, 0.026, 0]), 0.052, "detail", 0.8, reg, CHROME, pen=0.13)
    S.struct(S.ball("mass-steer", "steer", [0.03, 0.03, 0.03], sp(0.3161, -0.0086), PAINT, 0.5), "steer", "mass")
    S.struct(S.box("contact-pad-f", "steer", [0.04, 0.0075, 0.037], [F[0], 0.00375, 0.0], RUBBER, 0.1, rotation=kit.rot_w(70)), "steer", "support", anchor="contact-f")


def lamps(S):
    reg = "lamps"
    ax = pt.LAMP_AXIS
    S.put(S.rev("head-lamp", "steer", [(-0.034, 0.01), (-0.028, 0.022), (-0.012, 0.033), (0.012, 0.037), (0.022, 0.039), (0.027, 0.039)], pt.LAMP, ax, PLASTIC, 0.25), reg)
    S.put(S.rev("head-lamp-lens", "steer", [(0.0265, 0.0345), (0.032, 0.024), (0.035, 0.012), (0.036, 0.001)], pt.LAMP, ax, PLASTIC, 0.9, role="detail"), reg)
    S.member("head-lamp", "steer", kit.add(pt.LAMP, ax, -0.012), kit.add(pt.LAMP, ax, 0.027), 0.078, "body", 0.25, reg, PLASTIC, pen=0.13)
    S.member("head-lamp-back", "steer", kit.add(pt.LAMP, ax, -0.034), kit.add(pt.LAMP, ax, -0.012), 0.044, "body", 0.25, reg, PLASTIC, pen=0.13)
    S.plank("head-lamp-lens", "steer", kit.add(pt.LAMP, ax, 0.027), kit.add(pt.LAMP, ax, 0.0345), 0.066, 0.0, "detail", 0.9, reg, PLASTIC, pen=0.13, d1=0.03, wide=0.05, views=("elevation",))
    S.member("head-lamp-lens", "steer", kit.add(pt.LAMP, ax, 0.027), kit.add(pt.LAMP, ax, 0.034), 0.066, "detail", 0.9, reg, PLASTIC, pen=0.13, views=("front",))
    a, b = sp(pt.CROWN_U - 0.004, 0.014), kit.add(pt.LAMP, [-0.022, -0.026, 0.0])
    S.put(S.cap("lamp-bracket", "steer", a, b, 0.004, CHROME, STEEL, role="detail"), "steer-assembly")
    S.plank("lamp-bracket", "steer", a, b, 0.008, 0.0, "detail", STEEL, "steer-assembly", CHROME, pen=0.13)
    c = kit.polar(R, kit.FENDER_R + 0.016, pt.TAIL_DEG)
    S.put(S.box("tail-lamp", "frame", [0.03, 0.05, 0.045], c, PLASTIC, 0.25, corner=0.006, rotation=kit.rot_w(pt.TAIL_DEG)), reg)
    e = [math.cos(math.radians(pt.TAIL_DEG)), math.sin(math.radians(pt.TAIL_DEG))]
    t = [-e[1], e[0]]
    box = [[c[0] + e[0] * a + t[0] * b, c[1] + e[1] * a + t[1] * b] for a, b in ((-0.015, -0.025), (0.015, -0.025), (0.015, 0.025), (-0.015, 0.025))]
    S.swept("tail-lamp", "frame", box, -0.0225, 0.0225, "body", 0.25, reg, PLASTIC, pen=0.13)
    lens = kit.polar(R, kit.FENDER_R + 0.03, pt.TAIL_DEG)
    S.put(S.ball("tail-lamp-lens", "frame", [0.008, 0.02, 0.018], lens, PLASTIC, 0.85, role="detail", rotation=kit.rot_w(pt.TAIL_DEG)), reg)
    S.plank("tail-lamp-lens", "frame", lens, kit.polar(R, kit.FENDER_R + 0.038, pt.TAIL_DEG), 0.038, 0.0, "detail", 0.85, reg, PLASTIC, pen=0.13, d1=0.02, wide=0.034)


def stand(S):
    reg = "frame-tubes"
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    S.put(S.cyl("stand-pivot", "stand", at(0, -0.034), at(0, 0.034), 0.011, PAINT, TUBE, role="detail"), reg)
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.03), at(0, 0.03), 0.006, CHROME, 0.5), "stand", "joint")
    S.disc("stand-pivot", "stand", P, 0.022, -0.034, 0.034, "detail", TUBE, reg, PAINT, pen=0.13)
    for side, name in ((1, "l"), (-1, "r")):
        path = kit.bent([at(0.0, side * 0.03), at(0.05, side * 0.034), at(kit.STAND_LENGTH - 0.02, side * 0.09)], [0.03], 4)
        S.put(S.tube(f"stand-leg-{name}", "stand", [(p, 0.008, 0.008) for p in path], PAINT, TUBE, hint=[-u[1], u[0], 0.0]), reg)
        for k, (a, b) in enumerate(((path[0], path[3]), (path[3], path[-1]))):
            S.struct(S.pipe(f"stand-leg-{name}-{k + 1}", "stand", a, b, 0.008, 0.002, PAINT, 0.4), "stand")
            S.member(f"stand-leg-{name}-{k + 1}", "stand", a, b, 0.016, "body", TUBE, reg, PAINT, pen=0.13)
        a, b = at(kit.STAND_LENGTH - 0.03, side * 0.0875), at(kit.STAND_LENGTH - 0.011, side * 0.0925)
        S.put(S.cap(f"stand-foot-{name}", "stand", a, b, 0.011, RUBBER, DARK, role="detail"), reg)
        S.plank(f"stand-foot-{name}", "stand", at(kit.STAND_LENGTH - 0.041, 0), at(kit.STAND_LENGTH, 0), 0.022, side * 0.09, "detail", DARK, reg, RUBBER, pen=0.13)


def build():
    S = kit.Scale(50, PENS)
    wheel(S, "wheel-r", R, pt.REAR_HUB, ((0.029, 0.0345, 0.0), (-0.029, 0.0345, 10.0)), (-0.034, 0.036, 0.074), (0.036, 0.056, 0.09), 0.074, 0.072)
    wheel(S, "wheel-f", F, pt.FRONT_HUB, ((0.022, 0.032, 0.0), (-0.032, 0.032, 10.0)), (-0.038, 0.026, 0.09), (0.026, 0.042, 0.084), 0.064, 0.062)
    frame(S)
    carrier(S)
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
        "en": "1:50 is the complete Dutch city bicycle, every assembly stated by hand: frame of round steel tubes on their real centre lines (seat tube with lug and post, straight down tube, loop tube that runs parallel above it and bends back into the seat tube, 220 mm head tube with headset cups, bottom bracket shell, tapered chain and seat stays to forged dropouts, two bridges, the stand bracket), tube carrier with spine, cross bars, four legs to the axle and two struts to the seat lug, sprung saddle on rails, wheels with 37 mm tyres in box rims, 36 crossed spokes each, a three-speed hub and a dynamo hub with their roller brakes, axles and nuts, sheet mudguards with wire stays, the closed chain case, cranks with bosses and block pedals, fork with crown, curved blades and ends, quill stem, swept-back handlebar with grips, brake levers and bell, head lamp on its bracket, tail lamp on the rear mudguard and the folded two-leg centre stand.",
        "de": "1:50 ist das vollständige Holland-Stadtrad, jede Baugruppe von Hand angegeben: Rahmen aus runden Stahlrohren auf ihren wirklichen Mittellinien (Sattelrohr mit Muffe und Stütze, gerades Unterrohr, Bügelrohr, das parallel darüber läuft und zum Sattelrohr zurückbiegt, 220 mm langes Steuerrohr mit Steuersatzschalen, Tretlagergehäuse, konische Ketten- und Sitzstreben zu geschmiedeten Ausfallenden, zwei Stege, die Ständerkonsole), Rohrgepäckträger mit Mittelstab, Querstäben, vier Beinen zur Achse und zwei Streben zur Sattelmuffe, gefederter Sattel auf Gestell, Räder mit 37-mm-Reifen in Kastenfelgen, je 36 gekreuzte Speichen, Dreigangnabe und Nabendynamo mit ihren Rollenbremsen, Achsen und Muttern, Blechschutzbleche mit Drahtstreben, geschlossener Kettenkasten, Kurbeln mit Naben und Blockpedalen, Gabel mit Krone, gebogenen Scheiden und Gabelenden, Schaftvorbau, zurückgeschwungener Lenker mit Griffen, Bremshebeln und Klingel, Scheinwerfer an seinem Halter, Rücklicht auf dem hinteren Schutzblech und der hochgeklappte Zweibeinständer.",
    },
    {
        "en": "Thermal: every region lies on the body surface itself; the four regions with a steel wall under a paint film (frame tubes, steer assembly, mudguards, drivetrain) carry the wall as a second solid drawn in by the 0.06 mm film, the mudguard wall as its own 0.6 mm sheet. Tyres, rims and spokes, saddle, hubs with brake units and lamps have one layer and reference their body solids. There are no hidden regions and no emitters.",
        "de": "Thermik: jede Region liegt auf der Körperoberfläche selbst; die vier Regionen mit Stahlwand unter einem Lackfilm (Rahmenrohre, Lenkeinheit, Schutzbleche, Antrieb) tragen die Wand als zweiten, um den 0,06-mm-Film eingerückten Körper, die Schutzblechwand als eigenes 0,6-mm-Blech. Reifen, Felgen und Speichen, Sattel, Naben mit Bremseinheiten und Leuchten haben eine Schicht und verweisen auf ihre Körper. Es gibt keine verdeckten Regionen und keine Emitter.",
    },
    {
        "en": "Structure: the real load path. Frame tubes as hollow sections with their walls (seat tube, post, down tube, head tube, bottom bracket shell), the bent loop tube, stays, dropouts and the carrier rails, legs and struts; the hollow steerer inside the head tube with both headset bearings, fork crown and blades, quill stem and handlebar; rims, all 36 spokes and the hub shell per wheel with the hub bearing; crank axle, arms and bottom bracket bearings, pedal spindles with their bearings, stand legs with the hinge; the front tyre patch as support; masses of frame, steer assembly and wheels at their centres of mass.",
        "de": "Tragwerk: der wirkliche Lastpfad. Rahmenrohre als Hohlprofile mit ihren Wandstärken (Sattelrohr, Stütze, Unterrohr, Steuerrohr, Tretlagergehäuse), das gebogene Bügelrohr, Streben, Ausfallenden sowie Schienen, Beine und Streben des Gepäckträgers; der hohle Gabelschaft im Steuerrohr mit beiden Steuersatzlagern, Gabelkrone und Scheiden, Schaftvorbau und Lenker; Felgen, alle 36 Speichen und der Nabenkörper je Rad mit dem Nabenlager; Kurbelachse, Arme und Tretlager, Pedalachsen mit ihren Lagern, Ständerbeine mit dem Gelenk; die vordere Reifenaufstandsfläche als Auflager; Massen von Rahmen, Lenkeinheit und Rädern in ihren Schwerpunkten.",
    },
]

DRAWN = {
    "silhouette": {
        "en": "All three views are the true orthographic views of the 1:50 body. Side: tyre and rim rings with 18 of the 36 spokes, both hubs with their brake units, the frame with seat tube, down tube and loop tube, head tube with headset, stays and dropouts, carrier with legs and struts, sprung saddle, chain case, cranks and block pedals, mudguards with stays, fork with curved blades, quill stem, swept-back handlebar with grips, levers and bell, head and tail lamp and the folded centre stand. Front and top show the same parts from ahead and from above.",
        "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers. Seite: Reifen- und Felgenringe mit 18 der 36 Speichen, beide Naben mit ihren Bremseinheiten, der Rahmen mit Sattelrohr, Unterrohr und Bügelrohr, Steuerrohr mit Steuersatz, Streben und Ausfallenden, Gepäckträger mit Beinen und Streben, gefederter Sattel, Kettenkasten, Kurbeln und Blockpedale, Schutzbleche mit Streben, Gabel mit gebogenen Scheiden, Schaftvorbau, zurückgeschwungener Lenker mit Griffen, Hebeln und Klingel, Scheinwerfer und Rücklicht und der hochgeklappte Zweibeinständer. Vorder- und Draufsicht zeigen dieselben Teile von vorn und von oben.",
    },
    "retained": {"en": "Every assembly of the body that is at least 7.5 mm thick, and the wires (spokes, mudguard stays, saddle rails) as strokes.", "de": "Jede Baugruppe des Körpers ab 7,5 mm Dicke und die Drähte (Speichen, Schutzblechstreben, Sattelgestell) als Striche."},
    "merged": {"en": "Each handlebar half is five straight bars along its bend, each stand leg two; the hub shell and its flanges are one cylinder, the brake unit a second one.", "de": "Jede Lenkerhälfte besteht aus fünf geraden Stäben entlang ihrer Biegung, jedes Ständerbein aus zwei; Nabenkörper und Flansche sind ein Zylinder, die Bremseinheit ein zweiter."},
    "omitted": {"en": "Chain and chainring inside the closed case, brake and gear cables, tyre tread, reflectors, valve, the cross bars of the carrier in side and front view, the second half of the spokes.", "de": "Kette und Kettenblatt im geschlossenen Kasten, Brems- und Schaltzüge, Reifenprofil, Reflektoren, Ventil, die Querstäbe des Gepäckträgers in Seiten- und Vorderansicht, die zweite Hälfte der Speichen."},
    "thinElements": {"en": "Tubes are filled bars at their real width (seat tube 0.57 mm, stays 0.3 to 0.4 mm on paper) with a 0.13 to 0.18 mm outline; carrier rods 0.2 mm; spokes, mudguard stays and saddle rails are 0.13 mm strokes; mudguards are 0.2 mm bands.", "de": "Rohre sind gefüllte Stäbe in ihrer wirklichen Breite (Sattelrohr 0,57 mm, Streben 0,3 bis 0,4 mm auf dem Papier) mit 0,13 bis 0,18 mm Kontur; Gepäckträgerstäbe 0,2 mm; Speichen, Schutzblechstreben und Sattelgestell sind 0,13-mm-Striche; Schutzbleche sind 0,2-mm-Bänder."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Near black tyres, grips and pedal blocks, light rims and chrome parts, mid-tone painted tubes, lighter mudguards and chain case, dark saddle, white lamp lenses.", "de": "Fast schwarze Reifen, Griffe und Pedalblöcke, helle Felgen und Chromteile, mitteltonige lackierte Rohre, hellere Schutzbleche und Kettenkasten, dunkler Sattel, weiße Lampengläser."},
    "overlays": {"en": "Every shape carries the thermal region of its part. Straight tubes are members (bars between two points of their bone), plates and bent tubes are profiles extruded across the bicycle, so every part is one definition for side, front and top and follows steering (a twist of the steer bone about the head tube axis), rolling wheels, turning cranks with levelled pedals and the folding stand. Only the saddle has a separate outline per view.", "de": "Jede Form trägt die thermische Region ihres Teils. Gerade Rohre sind Stäbe zwischen zwei Punkten ihres Knochens, Platten und gebogene Rohre quer zum Rad extrudierte Profile; so ist jedes Teil eine Definition für Seite, Vorderansicht und Draufsicht und folgt dem Lenken (einer Drehung des Lenkknochens um die Steuerrohrachse), den rollenden Rädern, den drehenden Kurbeln mit waagerecht gehaltenen Pedalen und dem klappbaren Ständer. Nur der Sattel hat je Ansicht eine eigene Kontur."},
}
