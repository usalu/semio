"""🛺 Round 3 vehicles/cargo-bike at 1:100 (smallest feature 15 mm): the long-john authored anew for this scale. Hub, flanges and axle nuts are one shell, rotor and chainring are drawn rings, twelve spokes per wheel, cups merged into the head tubes, the bracket, rails and the stand bracket as plain plates, the carrier as rails, legs and struts without cross bars, saddle without rails, the lamp on the crown without bracket, levers and bell merged into the grips, box capping merged into the walls, the bench one block, each crate one tub with its lid."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


fine = load("r3-cargo-bike-s50")
pt = fine.pt
kit = pt.kit
bp, fp = kit.bar_pt, kit.fork_pt
R, F, B, P = pt.R3, pt.F3, pt.B3, kit.P
ALL, SIDE = pt.ALL, pt.SIDE
AL, PLY, RUBBER, RIM, DISC, MOTOR, BATT, PLASTIC = "aluminium", "plywood", "rubber", "alloy-rim", "steel-disc", "motor-case", "battery-case", "plastic"
TUBE, WOOD, DARK, METAL, LIGHT, CRATE = 0.32, 0.72, 0.1, 0.7, 0.88, 0.55
PENS = {"body": 0.18, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}
PEN = 0.13
bw, rounded = fine.bw, fine.rounded


def bar(S, i, bone, a, b, r, region, tone=TUBE, role="body", views=ALL, depth=None, capped=False):
    S.put((S.cap if capped else S.cyl)(i, bone, a, b, r, AL, tone, role=role), region)
    S.struct(S.pipe(i, bone, a, b, r, 0.002, AL, 0.4), bone)
    S.member(i, bone, a, b, 2 * r, role, tone, region, AL, pen=PEN, views=views, depth=depth)


def wheel(S, bone):
    w = pt.WHEELS[bone]
    c, tag = w["c"], bone[-1]
    centre, half, height = w["tyre"]
    S.put(S.rev(f"tyre-{tag}", bone, pt.tyre_profile(centre, half, height, 16), c, [0, 0, 1], RUBBER, DARK), "tyres")
    outer, inner = pt.ring(c, w["out"], centre - height + 0.002, 36)
    S.swept(f"tyre-{tag}", bone, outer, -half, half, "body", DARK, "tyres", RUBBER, holes=[inner], tol=0.0006, views=SIDE)
    for k, h in enumerate(pt.half_rings(c, w["out"], w["rim_r"][0], 18)):
        S.swept(f"tyre-{tag}-{k + 1}", bone, h, -half, half, "body", DARK, "tyres", RUBBER, tol=0.0006, views=("front", "plan"))
    r0, r1 = w["rim_r"]
    S.put(S.rev(f"rim-{tag}", bone, [(-0.015, r0), (0.015, r0), (0.015, r1), (-0.015, r1), (-0.015, r0)], c, [0, 0, 1], RIM, LIGHT))
    S.struct_of(f"rim-{tag}", bone)
    outer, inner = pt.ring(c, centre - height + 0.002, r0, 36)
    S.swept(f"rim-{tag}", bone, outer, -0.015, 0.015, "body", LIGHT, None, RIM, pen=PEN, holes=[inner], tol=0.0006, views=SIDE)
    hz = w["nut"] + 0.004
    rh = 0.04 if tag == "r" else 0.03
    S.put(S.rev(f"hub-{tag}", bone, [(-hz, 0.009), (-hz, 0.012), (-0.034, 0.012), (-0.034, rh), (0.034, rh), (0.034, 0.012), (hz, 0.012), (hz, 0.009)], c, [0, 0, 1], RIM, METAL))
    S.struct_of(f"hub-{tag}", bone)
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], -0.034], [c[0], c[1], 0.034], 0.011, RIM, 0.5), bone, "joint")
    S.disc(f"hub-{tag}", bone, c, 2 * rh, -0.034, 0.034, "detail", METAL, None, RIM, pen=PEN)
    z0, z1, q0, q1 = w["rotor"]
    S.put(S.rev(f"rotor-{tag}", bone, [(z0, q0), (z0, q1), (z1, q1), (z1, q0), (z0, q0)], c, [0, 0, 1], DISC, 0.6, role="detail", texture=True), "brakes")
    S.disc(f"rotor-{tag}", bone, c, 2 * q1, z0, z1, "detail", 0.6, "brakes", DISC, pen=PEN, filled=False, views=SIDE)
    for k, (fz, off) in enumerate(((0.03, 0.0), (-0.03, 15.0))):
        a, b = [rh * 0.9, 0.0, fz], [w["spoke_r"], 0.0, 0.0]
        rot = lambda p, d: [p[0] * math.cos(math.radians(d)) - p[1] * math.sin(math.radians(d)), p[0] * math.sin(math.radians(d)) + p[1] * math.cos(math.radians(d)), p[2]]
        S.put(S.cap(f"spoke-{tag}-{k + 1}", bone, kit.add(c, rot(a, off)), kit.add(c, rot(b, off + 30)), 0.001, RIM, 0.6, role="texture", slender=True, texture=True, repeat={"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": round(math.radians(60), 6), "origin": [0, 0, 0]}))
        S.struct_of(f"spoke-{tag}-{k + 1}", bone)
        for j in range(6):
            S.sliver(f"spoke-{tag}-{k + 1}-{j + 1}", bone, kit.add(c, rot(a, off + 60 * j)), kit.add(c, rot(b, off + 30 + 60 * j)), "texture", 0.6, None, RIM, 0.1)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.03, 0.03, 0.03], c, RIM, 0.5), bone, "mass")


def frame(S):
    reg = "frame"
    beam = pt.beam_path(5)
    S.put(S.tube("beam", "frame", [(p, pt.BEAM_HALF[0], pt.BEAM_HALF[1], 4.0) for p in beam], AL, TUBE), reg)
    S.swept("beam", "frame", kit.ribbon([q[:2] for q in beam], pt.BEAM_HALF[1]), -pt.BEAM_HALF[0], pt.BEAM_HALF[0], "body", TUBE, reg, AL, pen=PEN, tol=0.001)
    hy, hz, wall = pt.BEAM_HALF[1], pt.BEAM_HALF[0], 0.0025
    sect = [[pt.BEAM_Y - hy, -hz], [pt.BEAM_Y + hy, -hz], [pt.BEAM_Y + hy, hz], [pt.BEAM_Y - hy, hz]]
    hole = [[pt.BEAM_Y - hy + wall, -hz + wall], [pt.BEAM_Y + hy - wall, -hz + wall], [pt.BEAM_Y + hy - wall, hz - wall], [pt.BEAM_Y - hy + wall, hz - wall]]
    S.struct(S.solid("beam-box", "extrude", "frame", AL, 0.4, profile=sect, holes=[hole], plane="yz", **{"from": pt.BEAM_X0, "to": pt.BEAM_X1 - 0.08}), "frame")
    gs = beam[1:]
    S.struct(S.tube("gooseneck", "frame", [(p, pt.BEAM_HALF[0] - 0.003, pt.BEAM_HALF[1] - 0.003, 4.0) for p in gs], AL, 0.4), "frame")
    bar(S, "head-tube-f", "frame", fp(pt.HEAD_F[0] - 0.004), fp(pt.HEAD_F[1] + 0.004), 0.025, reg)
    bar(S, "head-tube-r", "frame", bp(pt.HEAD_R[0] - 0.004), bp(pt.HEAD_R[1] + 0.004), 0.025, reg)
    bar(S, "head-strut", "frame", bp(0.13, -0.004), [0.70, 0.284, 0.0], 0.016, reg)
    S.put(S.tube("down-tube", "frame", [(kit.lerp(list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, t), 0.02, 0.025, 2.6) for t in (0.0, 1.0)], AL, TUBE), reg)
    S.struct(S.pipe("down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, 0.022, 0.002, AL, 0.4), "frame")
    S.member("down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, 0.05, "body", TUBE, reg, AL, pen=PEN)
    bar(S, "seat-tube", "frame", kit.lerp(B, pt.SEAT_TOP, 0.05), kit.add(pt.SEAT_TOP, [pt.ST[0], pt.ST[1], 0], 0.012), 0.0185, reg)
    bar(S, "seat-post", "frame", kit.lerp(B, pt.SEAT_TOP, 0.85), pt.POST_TOP, 0.0158, reg, tone=METAL, views=("elevation", "front"))
    for side, name in ((1, "l"), (-1, "r")):
        drop = [R[0], R[1], side * pt.STAY_Z]
        a, b = [B[0] - 0.035, B[1] - 0.004, side * 0.026], kit.add(drop, [0.012, 0.0, 0.0])
        bar(S, f"chainstay-{name}", "frame", a, b, 0.01, reg, depth=[0.45, 0.31, side * 0.04], capped=True)
        a, b = kit.add(pt.SEAT_TOP, [-0.012, -0.03, side * 0.022]), kit.add(drop, [0.006, 0.014, 0.0])
        bar(S, f"seatstay-{name}", "frame", a, b, 0.0085, reg, capped=True)
    cast = rounded([[0.395, 0.325], [0.425, 0.385], [0.50, 0.408], [0.56, 0.40], [0.605, 0.345], [0.618, 0.29], [0.618, 0.218], [0.572, 0.218], [0.574, 0.29], [0.552, 0.35], [0.50, 0.372], [0.44, 0.355], [0.42, 0.318]], 0.012, 2)
    S.put(S.prism("motor-bracket", "frame", cast, -0.045, 0.045, AL, 0.28), reg)
    S.struct_of("motor-bracket", "frame")
    S.swept("motor-bracket", "frame", cast, -0.045, 0.045, "body", 0.28, reg, AL, pen=PEN, tol=0.0015)
    for k, x in enumerate((0.95, 1.27, 1.57)):
        prof = [[x - 0.02, 0.255], [x + 0.02, 0.255], [x + 0.02, 0.29], [x - 0.02, 0.29]]
        S.put(S.prism(f"box-rail-{k + 1}", "frame", prof, -0.29, 0.29, AL, 0.35, role="detail"), reg)
        S.struct_of(f"box-rail-{k + 1}", "frame")
        S.swept(f"box-rail-{k + 1}", "frame", prof, -0.29, 0.29, "detail", 0.35, reg, AL, pen=PEN, views=ALL if k == 2 else ("elevation", "plan"))
    brk = [[P[0] - 0.045, 0.222], [P[0] + 0.045, 0.222], [P[0] + 0.022, 0.19], [P[0] - 0.022, 0.19]]
    S.put(S.prism("stand-bracket", "frame", brk, -0.028, 0.028, AL, TUBE, role="detail"), reg)
    S.swept("stand-bracket", "frame", brk, -0.028, 0.028, "detail", TUBE, reg, AL, pen=PEN, views=("elevation", "front"))
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.045, 0.045, 0.045], [fx, fy, 0], AL, 0.5), "frame", "mass")
    S.struct(S.box("contact-pad-r", "frame", [0.04, 0.015, 0.04], [0.0, 0.0075, 0.0], RUBBER, 0.1), "frame", "support", anchor="contact-r")


def carrier(S):
    reg = "frame"
    x0, x1 = pt.CARRIER_X
    y, z = pt.CARRIER_Y, pt.CARRIER_Z
    hoop = [[x1, y, z], [x0, y, z], [x0, y, -z], [x1, y, -z]]
    path = kit.bent(hoop, [0.03, 0.03], 2)
    S.put(S.tube("carrier-hoop", "frame", [(p, 0.006, 0.006) for p in path], AL, TUBE, hint=[0.0, 1.0, 0.0], slender=True), reg)
    S.struct_of("carrier-hoop", "frame")
    S.flat("elevation", "carrier-hoop", "frame", "detail", TUBE, reg, AL, [[x1, y - 0.006], [x0 - 0.006, y - 0.006], [x0 - 0.006, y + 0.006], [x1, y + 0.006]], [x1, y, -z], pen=PEN)
    S.flat("front", "carrier-hoop", "frame", "detail", TUBE, reg, AL, [[-z - 0.006, y - 0.006], [z + 0.006, y - 0.006], [z + 0.006, y + 0.006], [-z - 0.006, y + 0.006]], [x1, y, -z], pen=PEN)
    S.flat("plan", "carrier-hoop", "frame", "detail", TUBE, reg, AL, kit.ribbon([[p[0], p[2]] for p in path], 0.006), [x1, y, -z], pen=PEN, tol=0.0)
    for side, name in ((1, "l"), (-1, "r")):
        eye = [R[0] - 0.004, R[1] + 0.016, side * 0.081]
        a, b = [-0.14, y, side * z], eye
        S.put(S.cap(f"carrier-leg-{name}", "frame", a, b, 0.006, AL, TUBE, role="detail", slender=True), reg)
        S.struct_of(f"carrier-leg-{name}", "frame")
        S.wire(f"carrier-leg-{name}", a, b, "detail", TUBE, reg, AL, PEN)
        a, b = [x1, y, side * 0.05], kit.add(pt.SEAT_TOP, [-0.03, -0.06, side * 0.026])
        S.put(S.cap(f"carrier-strut-{name}", "frame", a, b, 0.0075, AL, TUBE, role="detail"), reg)
        S.struct_of(f"carrier-strut-{name}", "frame")
        S.member(f"carrier-strut-{name}", "frame", a, b, 0.015, "detail", TUBE, reg, AL, pen=PEN)
    plate = [[x0 - 0.03, y - 0.06], [x0 + 0.006, y - 0.06], [x0 + 0.006, y + 0.004], [x0 - 0.03, y + 0.004]]
    S.put(S.prism("carrier-tail", "frame", plate, -0.06, 0.06, AL, 0.2, role="detail"), reg)
    S.swept("carrier-tail", "frame", plate, -0.06, 0.06, "detail", 0.2, reg, AL, pen=PEN)


SADDLE = [(0.338, 0.988, 0.016, 0.01), (0.295, 0.986, 0.04, 0.02), (0.205, 0.982, 0.083, 0.024), (0.115, 0.987, 0.106, 0.024), (0.07, 0.993, 0.062, 0.009)]


def saddle(S):
    S.put(S.tube("saddle", "frame", [([x, yc, 0.0], w, t, 2.6) for x, yc, w, t in SADDLE], PLASTIC, 0.15), "touch-parts")
    top = [[x, yc + t] for x, yc, w, t in SADDLE]
    bottom = [[x, yc - t] for x, yc, w, t in SADDLE]
    S.flat("elevation", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, top + bottom[::-1], [0.34, 1.0, -0.107])
    S.flat("plan", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[x, w] for x, yc, w, t in SADDLE] + [[x, -w] for x, yc, w, t in SADDLE][::-1], [0.34, 1.0, -0.107])
    x, yc, w, t = SADDLE[3]
    S.flat("front", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[w * math.copysign(abs(math.cos(a)) ** (2 / 2.6), math.cos(a)), yc + t * math.copysign(abs(math.sin(a)) ** (2 / 2.6), math.sin(a))] for a in [2 * math.pi * k / 20 for k in range(20)]], [0.34, 1.0, -0.107])
    c = [pt.POST_TOP[0], pt.POST_TOP[1] + 0.006, 0.0]
    S.put(S.box("saddle-clamp", "frame", [0.05, 0.032, 0.05], c, AL, METAL, role="detail", corner=0.006), "frame")


def guards(S):
    reg = "fenders"
    for i, bone, c, a0, a1, r in (("fender-r", "frame", R, 18.0, 196.0, pt.FENDER_R[0]), ("fender-f", "fork", F, 18.0, 150.0, pt.FENDER_R[1])):
        deg = kit.BONES[bone][1]
        o = kit.loc(bone, c)
        s = S.solid(i, "revolve", bone, PLASTIC, 0.22, profile=[[kit.r5(z), kit.r5(q)] for z, q in pt.fender_profile(r, n=4)], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec(fine.turn_offset(o, a0 - deg))}
        S.put(s, reg)
        band = kit.arc(c, r + 0.0013, a0, a1, 24) + kit.arc(c, r - 0.0122, a1, a0, 24)
        S.swept(i, bone, band, -pt.FENDER_HALF, pt.FENDER_HALF, "body", 0.22, reg, PLASTIC, pen=PEN, tol=0.0008)


def motor(S):
    out = [[B[0] + x, B[1] + y] for x, y in rounded(fine.MOTOR, 0.02, 2)]
    S.put(S.prism("motor-housing", "motor", out, -0.0425, 0.0425, MOTOR, 0.3), "drive-motor")
    S.struct_of("motor-housing", "motor")
    S.swept("motor-housing", "motor", out, -0.0425, 0.0425, "body", 0.3, "drive-motor", MOTOR, pen=PEN, tol=0.0015)
    S.struct(S.ball("mass-motor", "motor", [0.04, 0.04, 0.04], [B[0] + 0.02, B[1] - 0.01, 0.0], MOTOR, 0.5), "motor", "mass")


def battery(S):
    c = bw("battery", 0.15, 0.0665)
    S.put(S.box("battery-case", "battery", [0.36, 0.085, 0.092], c, BATT, 0.18, corner=0.014), "battery-pack")
    S.struct_of("battery-case", "battery")
    q = [bw("battery", u, v)[:2] for u, v in ((-0.03, 0.024), (0.33, 0.024), (0.33, 0.109), (-0.03, 0.109))]
    S.swept("battery-case", "battery", q, -0.046, 0.046, "body", 0.18, "battery-pack", BATT, pen=PEN, tol=0.0)
    S.struct(S.ball("mass-battery", "battery", [0.03, 0.03, 0.03], c, BATT, 0.5), "battery", "mass")


def drive(S):
    for n, (a, b) in enumerate(pt.chain_runs()):
        i = f"chain-{n + 1}"
        a3, b3 = [a[0], a[1], pt.CHAINRING[1]], [b[0], b[1], pt.CHAINRING[1]]
        S.put(S.cap(i, "frame", a3, b3, 0.0038, RIM, 0.25, role="detail", slender=True))
        S.wire(i, a3, b3, "detail", 0.25, None, RIM, 0.1, views=SIDE)
    z = pt.CHAINRING[1]
    S.put(S.rev("chainring", "crank", [(z - 0.006, 0.03), (z - 0.006, 0.089), (z + 0.002, 0.089), (z + 0.002, 0.03)], B, [0, 0, 1], PLASTIC, 0.3, role="detail", texture=True))
    S.disc("chainring", "crank", B, 0.178, z - 0.006, z + 0.002, "detail", 0.3, None, PLASTIC, pen=PEN, filled=False, views=SIDE)
    S.put(S.cyl("crank-axle", "crank", [B[0], B[1], -0.072], [B[0], B[1], 0.072], 0.009, RIM, METAL, role="detail"))
    S.struct_of("crank-axle", "crank")
    S.struct(S.cyl("bb-bearing", "crank", [B[0], B[1], -0.05], [B[0], B[1], 0.05], 0.014, RIM, 0.5), "crank", "joint")
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        zc = side * 0.064
        a, b = [B[0] - sign * 0.02, B[1], zc], [B[0] + sign * 0.18, B[1], zc]
        S.put(S.cap(f"crank-arm-{name}", "crank", a, b, 0.0075, AL, 0.3), None)
        S.struct_of(f"crank-arm-{name}", "crank")
        S.plank(f"crank-arm-{name}", "crank", a, b, 0.03, zc, "body", 0.3, None, AL, pen=PEN, wide=0.015, d1=0.02)
        e = [B[0] + sign * kit.CRANK, B[1], 0.0]
        zc = side * 0.12
        S.put(S.box(f"pedal-{name}", "crank", [0.07, 0.024, 0.11], [e[0], e[1], zc], PLASTIC, DARK, corner=0.006))
        S.struct_of(f"pedal-{name}", "crank")
        S.swept(f"pedal-{name}", "crank", [[e[0] - 0.035, e[1] - 0.012], [e[0] + 0.035, e[1] - 0.012], [e[0] + 0.035, e[1] + 0.012], [e[0] - 0.035, e[1] + 0.012]], zc - 0.055, zc + 0.055, "body", DARK, None, PLASTIC, pen=PEN)


def fork(S):
    reg = "frame"
    S.struct(S.pipe("steerer-f", "fork", fp(pt.CROWN_F[1]), fp(0.5), 0.0143, 0.002, AL, 0.4), "fork")
    S.put(S.cyl("steerer-cap-f", "fork", fp(pt.HEAD_F[1] + 0.004), fp(0.5), 0.0165, AL, 0.25, role="detail"), reg)
    S.plank("steerer-cap-f", "fork", fp(pt.HEAD_F[1] + 0.004), fp(0.5), 0.033, 0.0, "detail", 0.25, reg, AL, pen=PEN)
    for u, name in ((pt.HEAD_F[0], "low"), (pt.HEAD_F[1], "top")):
        S.struct(S.pipe(f"bearing-f-{name}", "fork", fp(u - 0.008), fp(u + 0.008), 0.022, 0.007, AL, 0.5), "fork", "joint")
    crown = [(fp(sum(pt.CROWN_F) / 2, 0.004, z), ru, rv, 3.2) for z, ru, rv in ((-0.068, 0.011, 0.018), (-0.04, 0.012, 0.024), (0.04, 0.012, 0.024), (0.068, 0.011, 0.018))]
    S.put(S.tube("fork-crown", "fork", crown, AL, TUBE, hint=[kit.UP_F[0], kit.UP_F[1], 0.0]), reg)
    S.struct_of("fork-crown", "fork")
    S.plank("fork-crown", "fork", fp(pt.CROWN_F[0], 0.004), fp(pt.CROWN_F[1], 0.004), 0.048, 0.0, "body", TUBE, reg, AL, pen=PEN, wide=0.136)
    S.put(S.rev("drum-f", "fork", [(0.0, 0.01), (0.0, 0.034), (pt.DRUM_F[1] - pt.DRUM_F[0], 0.034), (pt.DRUM_F[1] - pt.DRUM_F[0], 0.01)], fp(pt.DRUM_F[0]), [kit.UP_F[0], kit.UP_F[1], 0.0], AL, 0.3, role="detail"), reg)
    S.plank("drum-f", "fork", fp(pt.DRUM_F[0]), fp(pt.DRUM_F[1]), 0.068, 0.0, "detail", 0.3, reg, AL, pen=PEN, wide=0.068)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = fp(pt.CROWN_F[0] + 0.005, 0.002, side * 0.052), fp(0.0, kit.OFFSET_F, side * 0.05)
        S.put(S.cap(f"fork-blade-{name}", "fork", a, b, 0.012, AL, TUBE, r1=0.0095), reg)
        S.struct(S.pipe(f"fork-blade-{name}", "fork", a, b, 0.012, 0.0018, AL, 0.4), "fork")
        S.member(f"fork-blade-{name}", "fork", a, b, 0.024, "body", TUBE, reg, AL, pen=PEN)
    ax = [math.cos(math.radians(4)), math.sin(math.radians(4)), 0.0]
    lamp = fp(sum(pt.CROWN_F) / 2 + 0.01, 0.07)
    S.put(S.rev("head-lamp", "fork", [(-0.032, 0.012), (-0.022, 0.024), (0.0, 0.031), (0.035, 0.031), (0.038, 0.001)], lamp, ax, PLASTIC, 0.85, role="detail"))
    S.member("head-lamp", "fork", kit.add(lamp, ax, -0.025), kit.add(lamp, ax, 0.035), 0.062, "detail", 0.85, None, PLASTIC, pen=PEN)
    S.struct(S.ball("mass-fork", "fork", [0.03, 0.03, 0.03], fp(0.2, 0.02), AL, 0.5), "fork", "mass")


def cockpit(S):
    reg = "frame"
    S.struct(S.pipe("steerer-r", "bar", bp(0.04), bp(pt.RISER_TOP), 0.0143, 0.002, AL, 0.4), "bar")
    for u, name in ((pt.HEAD_R[0], "low"), (pt.HEAD_R[1], "top")):
        S.struct(S.pipe(f"bearing-r-{name}", "bar", bp(u - 0.008), bp(u + 0.008), 0.022, 0.007, AL, 0.5), "bar", "joint")
    S.put(S.rev("drum-r", "bar", [(0.0, 0.01), (0.0, 0.034), (pt.HEAD_R[0] - 0.004 - pt.DRUM_R[0], 0.034), (pt.HEAD_R[0] - 0.004 - pt.DRUM_R[0], 0.01)], bp(pt.DRUM_R[0]), [kit.UP_R[0], kit.UP_R[1], 0.0], AL, 0.3, role="detail"), reg)
    S.plank("drum-r", "bar", bp(pt.DRUM_R[0]), bp(pt.HEAD_R[0] - 0.004), 0.068, 0.0, "detail", 0.3, reg, AL, pen=PEN, wide=0.068)
    S.put(S.cyl("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.0175, AL, TUBE), reg)
    S.struct(S.pipe("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.0175, 0.002, AL, 0.4), "bar")
    S.member("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.035, "body", TUBE, reg, AL, pen=PEN)
    S.put(S.cap("stem", "bar", bp(pt.CLAMP_S - 0.004, 0.0), pt.CLAMP, 0.019, AL, TUBE), reg)
    S.struct_of("stem", "bar")
    S.plank("stem", "bar", bp(pt.CLAMP_S - 0.004, 0.0), pt.CLAMP, 0.038, 0.0, "body", TUBE, reg, AL, pen=PEN, wide=0.05)
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.bar_path(side, 3)
        g0 = pt.grip_start(side)
        end = pt.mirror(pt.BAR_END, side)
        S.put(S.tube(f"handlebar-{name}", "bar", [(p, 0.0127, 0.0127) for p in path[:-1] + [kit.lerp(g0, end, 0.5)]], AL, METAL, hint=[0.0, 1.0, 0.0]), reg)
        knots = [path[0], path[2], g0, end]
        for k in range(len(knots) - 1):
            S.struct(S.pipe(f"handlebar-{name}-{k + 1}", "bar", knots[k], knots[k + 1], 0.0127, 0.0018, AL, 0.5), "bar")
            if k < 2:
                S.member(f"handlebar-{name}-{k + 1}", "bar", knots[k], knots[k + 1], 0.0254, "body", METAL, reg, AL, pen=PEN)
        d = kit.unit(kit.sub(end, g0))
        S.put(S.cyl(f"grip-{name}", "bar", kit.add(g0, d, -0.03), end, 0.0175, RUBBER, DARK), "touch-parts")
        S.member(f"grip-{name}", "bar", kit.add(g0, d, -0.03), end, 0.035, "body", DARK, "touch-parts", RUBBER, pen=PEN)
    disp = kit.add(pt.CLAMP, [0.006, 0.024, 0.0])
    S.put(S.box("display", "bar", [0.05, 0.016, 0.075], disp, PLASTIC, DARK, role="detail", corner=0.004, rotation=kit.rot_w(-15.0)))
    S.plank("display", "bar", kit.add(disp, [-0.024, -0.006, 0]), kit.add(disp, [0.024, 0.006, 0]), 0.016, 0.0, "detail", DARK, None, PLASTIC, pen=PEN, wide=0.075)
    S.struct(S.ball("mass-bar", "bar", [0.025, 0.025, 0.025], bp(0.72, 0.0), AL, 0.5), "bar", "mass")


def cables(S):
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.cable_path(side)[::2] + [pt.cable_path(side)[-1]]
        S.put(S.tube(f"steer-cable-{name}", "frame", [(p, 0.003, 0.003) for p in path], RUBBER, 0.12, role="detail", slender=True))
        S.struct(S.tube(f"steer-cable-{name}", "frame", [(p, 0.002, 0.002) for p in path], AL, 0.4, slender=True), "frame")
        if side < 0:
            S.flat("elevation", f"steer-cable-{name}", "frame", "detail", 0.12, None, RUBBER, [q[:2] for q in path], [path[0][0], path[0][1], -0.034], pen=0.1, filled=False, closed=False, tol=0.002)


def box(S):
    reg = "box-wood"
    top = pt.BOX_YT + pt.CAP[1]
    side = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_XT + 0.004, top], [pt.BOX_X0, top]]
    for s, name in ((1, "l"), (-1, "r")):
        z0, z1 = sorted((s * pt.BOX_HW, s * (pt.BOX_HW - 0.015)))
        S.put(S.prism(f"box-side-{name}", "box", side, z0, z1, PLY, WOOD), reg)
        S.struct_of(f"box-side-{name}", "box")
        S.swept(f"box-side-{name}", "box", side, z0, z1, "body", WOOD, reg, PLY, pen=PEN, tol=0.0)
    hw = pt.BOX_HW - 0.015
    floor = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_X1 + 0.004, pt.BOX_Y0 + 0.015], [pt.BOX_X0, pt.BOX_Y0 + 0.015]]
    rear = [[pt.BOX_X0, pt.BOX_Y0 + 0.015], [pt.BOX_X0 + 0.015, pt.BOX_Y0 + 0.015], [pt.BOX_X0 + 0.015, top], [pt.BOX_X0, top]]
    k = (pt.BOX_XT - pt.BOX_X1) / (pt.BOX_YT - pt.BOX_Y0)
    xf = pt.BOX_X1 + k * 0.015
    d = 0.015 * math.sqrt(1 + k * k)
    front = [[xf - d, pt.BOX_Y0 + 0.015], [xf, pt.BOX_Y0 + 0.015], [pt.BOX_XT + 0.004, top], [pt.BOX_XT + 0.004 - d, top]]
    bench = [[pt.BOX_X0 + 0.015, pt.BOX_Y0 + 0.015], [pt.BENCH_X1, pt.BOX_Y0 + 0.015], [pt.BENCH_X1, pt.BENCH_Y], [pt.BOX_X0 + 0.015, pt.BENCH_Y]]
    for i, prof, role, z in (("box-floor", floor, "body", pt.BOX_HW), ("box-rear", rear, "body", hw), ("box-front", front, "body", hw), ("bench", bench, "detail", hw)):
        S.put(S.prism(i, "box", prof, -z, z, PLY, WOOD if role == "body" else 0.8, role=role), reg)
        S.struct_of(i, "box")
        S.swept(i, "box", prof, -z, z, role, WOOD if role == "body" else 0.8, reg, PLY, pen=PEN, tol=0.0)
    S.struct(S.ball("mass-box", "box", [0.04, 0.04, 0.04], [1.24, 0.42, 0.0], PLY, 0.5), "box", "mass")


def crates(S):
    reg = "cargo"
    L, W, H = pt.CRATE
    x0, y0 = kit.CRATE_O
    for k, name in ((0, "lower"), (1, "upper")):
        yb = y0 + k * H
        rows = [([x0 + L / 2, yb + 0.002, 0.0], W / 2 - 0.018, L / 2 - 0.018, 6.0), ([x0 + L / 2, yb + H - 0.045, 0.0], W / 2 - 0.006, L / 2 - 0.006, 6.0), ([x0 + L / 2, yb + H - 0.044, 0.0], W / 2, L / 2, 6.0), ([x0 + L / 2, yb + H - 0.001, 0.0], W / 2, L / 2, 6.0)]
        S.put(S.tube(f"crate-{name}", "crate", rows, PLASTIC, CRATE + 0.07 * k, hint=[0.0, 0.0, 1.0]), reg)
        S.struct_of(f"crate-{name}", "crate")
        side = [[x0 + 0.018, yb + 0.002], [x0 + L - 0.018, yb + 0.002], [x0 + L - 0.006, yb + H - 0.045], [x0 + L, yb + H - 0.044], [x0 + L, yb + H - 0.001], [x0, yb + H - 0.001], [x0, yb + H - 0.044], [x0 + 0.006, yb + H - 0.045]]
        depth = [x0 + L, yb + H, -W / 2]
        S.flat("elevation", f"crate-{name}", "crate", "body", CRATE + 0.07 * k, reg, PLASTIC, [kit.loc("crate", q)[:2] for q in side], depth, pen=PEN, tol=0.0)
        front = [[-(W / 2 - 0.018), yb + 0.002], [W / 2 - 0.018, yb + 0.002], [W / 2 - 0.006, yb + H - 0.045], [W / 2, yb + H - 0.044], [W / 2, yb + H - 0.001], [-W / 2, yb + H - 0.001], [-W / 2, yb + H - 0.044], [-(W / 2 - 0.006), yb + H - 0.045]]
        S.flat("front", f"crate-{name}", "crate", "body", CRATE + 0.07 * k, reg, PLASTIC, front, depth, pen=PEN, tol=0.0)
    S.flat("plan", "crate-upper", "crate", "body", CRATE + 0.07, reg, PLASTIC, rounded([[x0, -W / 2], [x0 + L, -W / 2], [x0 + L, W / 2], [x0, W / 2]], 0.012, 2), [x0 + L, y0 + 2 * H, -W / 2], pen=PEN, tol=0.0)
    S.struct(S.box("crate-post", "crate-y", [0.02, 2 * H - 0.02, 0.02], [x0 + 0.02, y0 + H, W / 2 - 0.02], PLASTIC, 0.4), "crate-y")
    S.struct(S.box("crate-base", "crate-x", [L - 0.04, 0.015, W - 0.04], [x0 + L / 2, y0 + 0.009, 0.0], PLASTIC, 0.4), "crate-x")
    S.struct(S.ball("mass-crates", "crate", [0.04, 0.04, 0.04], [x0 + L / 2, y0 + H, 0.0], PLASTIC, 0.5), "crate", "mass")


def stand(S):
    reg = "frame"
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    S.put(S.cyl("stand-pivot", "stand", at(0, -0.046), at(0, 0.046), 0.011, AL, TUBE, role="detail"), reg)
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.04), at(0, 0.04), 0.008, AL, 0.5), "stand", "joint")
    S.disc("stand-pivot", "stand", P, 0.022, -0.046, 0.046, "detail", TUBE, reg, AL, pen=PEN)
    L = kit.STAND_LENGTH
    for side, name in ((1, "l"), (-1, "r")):
        a, b = at(0.0, side * 0.04), at(L - 0.012, side * 0.205)
        S.put(S.cap(f"stand-leg-{name}", "stand", a, b, 0.0095, AL, TUBE), reg)
        S.struct(S.pipe(f"stand-leg-{name}", "stand", a, b, 0.0095, 0.002, AL, 0.4), "stand")
        S.member(f"stand-leg-{name}", "stand", a, b, 0.019, "body", TUBE, reg, AL, pen=PEN)
        a, b = at(L - 0.012, side * 0.18), at(L - 0.012, side * 0.232)
        S.put(S.cap(f"stand-foot-{name}", "stand", a, b, 0.012, RUBBER, DARK, role="detail"), reg)
    a, b = at(0.15, -0.14), at(0.15, 0.14)
    S.put(S.cyl("stand-cross", "stand", a, b, 0.008, AL, TUBE, role="detail"), reg)
    S.struct(S.pipe("stand-cross", "stand", a, b, 0.008, 0.002, AL, 0.4), "stand")


def build():
    S = kit.Scale(100, PENS)
    wheel(S, "wheel-r")
    wheel(S, "wheel-f")
    frame(S)
    carrier(S)
    saddle(S)
    guards(S)
    motor(S)
    battery(S)
    drive(S)
    fork(S)
    cockpit(S)
    cables(S)
    box(S)
    crates(S)
    stand(S)
    return S


DECISIONS = [
    {
        "en": "1:100 is the long-john authored anew at 15 mm: the same frame lines (beam with gooseneck, both head tubes with their cups merged, strut, oval down tube, seat tube with clamp merged, post, stays) and the cast motor bracket, three box rails and the stand bracket; wheels as tyre, rim and one hub shell (flanges, cones and nuts merged), the brake rotors and the chainring with its guard as drawn rings, twelve spokes each; carrier as one hoop on two legs and two struts, saddle pad with its clamp, mudguards, motor, battery with its lock merged, chain, straight crank bars, pedal blocks; fork with crown, drum and two blades, the head lamp on the crown; handlebar column with drum, riser, stem, swept bar in three bends and grips with levers merged; the box walls with the capping merged, the bench as one block, each crate one tub with its lid; the stand with straight legs.",
        "de": "1:100 ist das Long-John neu für 15 mm: dieselben Rahmenlinien (Träger mit Schwanenhals, beide Steuerrohre mit eingeschlossenen Schalen, Strebe, ovales Unterrohr, Sattelrohr mit eingeschlossener Klemme, Stütze, Streben) und die gegossene Motorkonsole, drei Boxschienen und die Ständerkonsole; Räder als Reifen, Felge und ein Nabenkörper (Flansche, Konen und Muttern eingeschlossen), Bremsscheiben und Kettenblatt mit Schutzring als gezeichnete Ringe, je zwölf Speichen; Gepäckträger als ein Bügel auf zwei Beinen und zwei Streben, Sattelpolster mit Klemme, Schutzbleche, Motor, Akku mit eingeschlossenem Schloss, Kette, gerade Kurbelstäbe, Pedalblöcke; Gabel mit Krone, Trommel und zwei Scheiden, der Scheinwerfer an der Krone; Lenksäule mit Trommel, Steigrohr, Vorbau, geschwungenem Lenker in drei Biegungen und Griffen mit eingeschlossenen Hebeln; die Boxwände mit eingeschlossenen Kantenleisten, die Bank als ein Block, jede Kiste eine Wanne mit Deckel; der Ständer mit geraden Beinen.",
    },
    {
        "en": "Thermal: every region references its body solids (frame, box-wood, cargo, fenders, touch-parts, battery-pack, drive-motor, tyres, brakes); the paint and the single layers are merged into the body surface at this scale.",
        "de": "Thermik: jede Region verweist auf ihre Körper (Rahmen, Boxholz, Ladung, Schutzbleche, Berührteile, Akku, Motor, Reifen, Bremsen); Lack und einzelne Schichten sind bei diesem Maßstab in die Körperoberfläche eingeschlossen.",
    },
    {
        "en": "Structure: the same load path with merged parts: beam box section and gooseneck, both head tubes, strut, down tube, seat tube, post and stays as hollow sections, bracket, rails, carrier hoop, legs and struts; steerers with their bearings, crown, blades, riser, stem and handlebar as four tubes per side; cables; rims, twelve spokes and hub per wheel with bearing; crank axle, bearing, arms and pedals; motor housing, battery case; box floor, walls and bench; crate base and corner post; stand legs, cross tube and hinge; the rear tyre patch as support; masses at their centres.",
        "de": "Tragwerk: derselbe Lastpfad mit zusammengefassten Teilen: Kastenprofil des Trägers und Schwanenhals, beide Steuerrohre, Strebe, Unterrohr, Sattelrohr, Stütze und Streben als Hohlprofile, Konsole, Schienen, Gepäckträgerbügel, Beine und Streben; Gabelschäfte mit ihren Lagern, Krone, Scheiden, Steigrohr, Vorbau und Lenker als vier Rohre je Seite; Seile; Felgen, zwölf Speichen und Nabe je Rad mit Lager; Kurbelachse, Lager, Arme und Pedale; Motorgehäuse, Akkugehäuse; Boxboden, Wände und Bank; Kistenboden und Eckpfosten; Ständerbeine, Querrohr und Gelenk; die hintere Reifenaufstandsfläche als Auflager; Massen in ihren Schwerpunkten.",
    },
]

DRAWN = {
    "silhouette": {"en": "All three views are the true orthographic views of the 1:100 body in the parked pose: wheels with twelve spokes, rotor rings and hubs, the frame with beam and gooseneck, column, battery, motor with bracket, saddle, carrier, mudguards, fork with lamp, handlebar, the box with bench, the two crates and the stand under the box.", "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:100-Körpers in der Parkstellung: Räder mit zwölf Speichen, Scheibenringen und Naben, der Rahmen mit Träger und Schwanenhals, Lenksäule, Akku, Motor mit Konsole, Sattel, Gepäckträger, Schutzbleche, Gabel mit Scheinwerfer, Lenker, die Box mit Bank, die zwei Kisten und der Ständer unter der Box."},
    "retained": {"en": "Every part of at least 15 mm, the carrier tubes, chain and steering cable as strokes.", "de": "Jedes Teil ab 15 mm, die Gepäckträgerrohre, die Kette und das Lenkseil als Striche."},
    "merged": {"en": "Headset cups into the head tubes, hub flanges and nuts into one shell, levers into the grips, box capping into the walls, bench seat and panel into one block, crate lids into the tubs.", "de": "Steuersatzschalen in die Steuerrohre, Nabenflansche und Muttern in einen Körper, Hebel in die Griffe, Kantenleisten in die Wände, Banksitz und -front in einen Block, Kistendeckel in die Wannen."},
    "omitted": {"en": "Dropouts, saddle rails, mudguard stays, the carrier cross bars, lamp bracket, tail lamp, bell, chain guard as a separate part, crate handles.", "de": "Ausfallenden, Sattelgestell, Schutzblechstreben, Querstäbe des Gepäckträgers, Scheinwerferhalter, Rücklicht, Klingel, der Kettenschutz als eigenes Teil, Kistengriffe."},
    "thinElements": {"en": "Tubes are filled bars at their real width (beam 0.7 mm, stays 0.17 to 0.2 mm on paper) with a 0.13 mm outline; carrier, chain and cable are 0.1 to 0.13 mm strokes; rotors and chainring are open rings.", "de": "Rohre sind gefüllte Stäbe in ihrer wirklichen Breite (Träger 0,7 mm, Streben 0,17 bis 0,2 mm auf dem Papier) mit 0,13 mm Kontur; Gepäckträger, Kette und Seil sind Striche von 0,1 bis 0,13 mm; Bremsscheiben und Kettenblatt sind offene Ringe."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "As at 1:50: near black tyres, grips and pedals, light rims, mid-tone frame, light box, grey crates, dark battery and motor.", "de": "Wie bei 1:50: fast schwarze Reifen, Griffe und Pedale, helle Felgen, mitteltoniger Rahmen, helle Box, graue Kisten, dunkler Akku und Motor."},
    "overlays": {"en": "Every shape carries the thermal region of its part where it has one; straight tubes are members, plates and round parts extruded profiles, so steering, rolling, the crank, the stand and the crate stack move the drawing with the body.", "de": "Jede Form trägt die thermische Region ihres Teils, wo es eine gibt; gerade Rohre sind Stäbe, Platten und runde Teile extrudierte Profile, so bewegen Lenken, Rollen, Kurbel, Ständer und Kistenstapel die Zeichnung mit dem Körper."},
}
