"""🛺 Round 3 vehicles/cargo-bike at 1:200 (smallest feature 30 mm): the long-john reduced to what this scale shows, authored anew. Each wheel is one ring (tyre and rim) with a hub that carries the rotor and six spokes; the frame is the beam with a three-step gooseneck, two head tubes, strut, down tube, seat tube with post and the stays at their true width; motor with its bracket, battery, saddle pad, carrier hoop on two legs, mudguards, crank bars with pedal blocks, fork of crown and two blades with the lamp, one column bar with the swept handlebar, the box as 20 mm walls with the bench block, each crate one block, the stand as two legs."""
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
AL, PLY, RUBBER, RIM, MOTOR, BATT, PLASTIC = "aluminium", "plywood", "rubber", "alloy-rim", "motor-case", "battery-case", "plastic"
TUBE, WOOD, DARK, METAL, CRATE = 0.32, 0.72, 0.12, 0.7, 0.55
PENS = {"body": 0.13, "detail": 0.1, "texture": 0.1, "glazing": 0.1, "ground": 0.25}
PEN = 0.13
bw, rounded = fine.bw, fine.rounded


def bar(S, i, bone, a, b, r, region, tone=TUBE, role="body", views=ALL, depth=None, capped=False, wall=0.002):
    S.put((S.cap if capped else S.cyl)(i, bone, a, b, r, AL, tone, role=role), region)
    S.struct(S.pipe(i, bone, a, b, r, wall, AL, 0.4), bone)
    S.member(i, bone, a, b, 2 * r, role, tone, region, AL, pen=PEN, views=views, depth=depth)


def wheel(S, bone):
    w = pt.WHEELS[bone]
    c, tag = w["c"], bone[-1]
    ro, ri = w["out"], w["rim_r"][0]
    ring = [(-0.015, ri), (0.015, ri), (0.015, ro - 0.035), (0.025, ro - 0.022), (0.016, ro - 0.004), (0.0, ro), (-0.016, ro - 0.004), (-0.025, ro - 0.022), (-0.015, ro - 0.035), (-0.015, ri)]
    S.put(S.rev(f"wheel-{tag}", bone, ring, c, [0, 0, 1], RUBBER, DARK), "tyres")
    S.struct_of(f"wheel-{tag}", bone)
    outer, inner = pt.ring(c, ro, ri, 28)
    S.swept(f"wheel-{tag}", bone, outer, -0.025, 0.025, "body", DARK, "tyres", RUBBER, holes=[inner], tol=0.0015, views=SIDE)
    for k, h in enumerate(pt.half_rings(c, ro, ri, 14)):
        S.swept(f"wheel-{tag}-{k + 1}", bone, h, -0.025, 0.025, "body", DARK, "tyres", RUBBER, tol=0.0015, views=("front", "plan"))
    hz = w["drop"][0]
    S.put(S.cyl(f"hub-{tag}", bone, [c[0], c[1], -hz], [c[0], c[1], hz], 0.035, RIM, METAL))
    S.therm(S.body[-1], "brakes")
    S.struct_of(f"hub-{tag}", bone)
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], -hz], [c[0], c[1], hz], 0.015, RIM, 0.5), bone, "joint")
    S.disc(f"hub-{tag}", bone, c, 0.07, -hz, hz, "detail", METAL, "brakes", RIM, pen=PEN)
    a, b = [0.034, 0.0, 0.0], [ri, 0.0, 0.0]
    S.put(S.cap(f"spoke-{tag}", bone, kit.add(c, a), kit.add(c, b), 0.001, RIM, 0.6, role="texture", slender=True, texture=True, repeat={"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": round(math.radians(60), 6), "origin": [0, 0, 0]}))
    S.struct_of(f"spoke-{tag}", bone)
    for j in range(6):
        q = math.radians(60 * j + 30)
        S.sliver(f"spoke-{tag}-{j + 1}", bone, kit.add(c, [0.034 * math.cos(q), 0.034 * math.sin(q), 0]), kit.add(c, [ri * math.cos(q), ri * math.sin(q), 0]), "texture", 0.6, None, RIM, 0.1)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.03, 0.03, 0.03], c, RIM, 0.5), bone, "mass")


def frame(S):
    reg = "frame"
    beam = pt.beam_path(3)
    S.put(S.tube("beam", "frame", [(p, pt.BEAM_HALF[0], pt.BEAM_HALF[1], 4.0) for p in beam], AL, TUBE), reg)
    S.swept("beam", "frame", kit.ribbon([q[:2] for q in beam], pt.BEAM_HALF[1]), -pt.BEAM_HALF[0], pt.BEAM_HALF[0], "body", TUBE, reg, AL, pen=PEN, tol=0.002)
    S.struct(S.tube("beam", "frame", [(p, pt.BEAM_HALF[0] - 0.003, pt.BEAM_HALF[1] - 0.003, 4.0) for p in beam], AL, 0.4), "frame")
    bar(S, "head-tube-f", "frame", fp(pt.HEAD_F[0] - 0.004), fp(0.5), 0.025, reg)
    bar(S, "head-tube-r", "frame", bp(pt.DRUM_R[0]), bp(pt.HEAD_R[1] + 0.004), 0.025, reg)
    bar(S, "head-strut", "frame", bp(0.13, -0.004), [0.70, 0.284, 0.0], 0.016, reg)
    S.put(S.tube("down-tube", "frame", [(kit.lerp(list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, t), 0.02, 0.025, 2.6) for t in (0.0, 1.0)], AL, TUBE), reg)
    S.struct(S.pipe("down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, 0.022, 0.002, AL, 0.4), "frame")
    S.member("down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, 0.05, "body", TUBE, reg, AL, pen=PEN)
    bar(S, "seat-tube", "frame", kit.lerp(B, pt.SEAT_TOP, 0.05), pt.POST_TOP, 0.0175, reg)
    for side, name in ((1, "l"), (-1, "r")):
        drop = [R[0], R[1], side * pt.STAY_Z]
        a, b = [B[0] - 0.035, B[1] - 0.004, side * 0.026], kit.add(drop, [0.012, 0.0, 0.0])
        bar(S, f"chainstay-{name}", "frame", a, b, 0.015, reg, depth=[0.45, 0.31, side * 0.04], capped=True, views=ALL if side < 0 else ("front", "plan"))
        a, b = kit.add(pt.SEAT_TOP, [-0.012, -0.03, side * 0.022]), kit.add(drop, [0.006, 0.014, 0.0])
        bar(S, f"seatstay-{name}", "frame", a, b, 0.015, reg, capped=True, views=ALL if side < 0 else ("front", "plan"))
    cast = [[0.40, 0.33], [0.46, 0.40], [0.57, 0.40], [0.618, 0.30], [0.618, 0.218], [0.57, 0.218], [0.55, 0.34], [0.44, 0.33]]
    S.put(S.prism("motor-bracket", "frame", cast, -0.045, 0.045, AL, 0.28), reg)
    S.struct_of("motor-bracket", "frame")
    S.swept("motor-bracket", "frame", cast, -0.045, 0.045, "body", 0.28, reg, AL, pen=PEN, tol=0.0)
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.05, 0.05, 0.05], [fx, fy, 0], AL, 0.5), "frame", "mass")
    S.struct(S.box("contact-pad-r", "frame", [0.05, 0.03, 0.05], [0.0, 0.015, 0.0], RUBBER, 0.1), "frame", "support", anchor="contact-r")


def carrier(S):
    reg = "frame"
    x0, x1 = pt.CARRIER_X
    y, z = pt.CARRIER_Y, pt.CARRIER_Z
    hoop = [[x1, y, z], [x0, y, z], [x0, y, -z], [x1, y, -z]]
    path = kit.bent(hoop, [0.03, 0.03], 1)
    S.put(S.tube("carrier-hoop", "frame", [(p, 0.006, 0.006) for p in path], AL, TUBE, hint=[0.0, 1.0, 0.0], slender=True), reg)
    S.struct_of("carrier-hoop", "frame")
    S.flat("elevation", "carrier-hoop", "frame", "detail", TUBE, reg, AL, [[x1, y], [x0, y]], [x1, y, -z], pen=PEN, filled=False, closed=False)
    S.flat("front", "carrier-hoop", "frame", "detail", TUBE, reg, AL, [[-z, y], [z, y]], [x1, y, -z], pen=PEN, filled=False, closed=False)
    S.flat("plan", "carrier-hoop", "frame", "detail", TUBE, reg, AL, [[p[0], p[2]] for p in path], [x1, y, -z], pen=PEN, filled=False, closed=False, tol=0.0)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = [-0.14, y, side * z], [R[0] - 0.004, R[1] + 0.016, side * 0.081]
        S.put(S.cap(f"carrier-leg-{name}", "frame", a, b, 0.006, AL, TUBE, role="detail", slender=True), reg)
        S.struct_of(f"carrier-leg-{name}", "frame")
        S.wire(f"carrier-leg-{name}", a, b, "detail", TUBE, reg, AL, PEN)


SADDLE = [(0.338, 0.99, 0.02, 0.015), (0.205, 0.982, 0.083, 0.024), (0.07, 0.99, 0.08, 0.016)]


def saddle(S):
    S.put(S.tube("saddle", "frame", [([x, yc, 0.0], w, t, 2.6) for x, yc, w, t in SADDLE], PLASTIC, 0.15), "touch-parts")
    top = [[x, yc + t] for x, yc, w, t in SADDLE]
    bottom = [[x, yc - t] for x, yc, w, t in SADDLE]
    S.flat("elevation", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, top + bottom[::-1], [0.34, 1.0, -0.107])
    S.flat("plan", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[x, w] for x, yc, w, t in SADDLE] + [[x, -w] for x, yc, w, t in SADDLE][::-1], [0.34, 1.0, -0.107])
    S.flat("front", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[-0.083, 0.958], [0.083, 0.958], [0.083, 1.006], [-0.083, 1.006]], [0.34, 1.0, -0.107])


def guards(S):
    reg = "fenders"
    for i, bone, c, a0, a1, r in (("fender-r", "frame", R, 18.0, 196.0, pt.FENDER_R[0]), ("fender-f", "fork", F, 18.0, 150.0, pt.FENDER_R[1])):
        deg = kit.BONES[bone][1]
        o = kit.loc(bone, c)
        s = S.solid(i, "revolve", bone, PLASTIC, 0.22, profile=[[kit.r5(z), kit.r5(q)] for z, q in pt.fender_profile(r, n=2)], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec(fine.turn_offset(o, a0 - deg))}
        S.put(s, reg)
        band = kit.arc(c, r + 0.0013, a0, a1, 16) + kit.arc(c, r - 0.0122, a1, a0, 16)
        S.swept(i, bone, band, -pt.FENDER_HALF, pt.FENDER_HALF, "body", 0.22, reg, PLASTIC, pen=PEN, tol=0.0015)


def motor(S):
    out = [[B[0] + x, B[1] + y] for x, y in fine.MOTOR]
    S.put(S.prism("motor-housing", "motor", out, -0.0425, 0.0425, MOTOR, 0.3), "drive-motor")
    S.struct_of("motor-housing", "motor")
    S.swept("motor-housing", "motor", out, -0.0425, 0.0425, "body", 0.3, "drive-motor", MOTOR, pen=PEN, tol=0.0)
    S.struct(S.ball("mass-motor", "motor", [0.04, 0.04, 0.04], [B[0] + 0.02, B[1] - 0.01, 0.0], MOTOR, 0.5), "motor", "mass")


def battery(S):
    c = bw("battery", 0.15, 0.0665)
    S.put(S.box("battery-case", "battery", [0.36, 0.085, 0.092], c, BATT, 0.18, corner=0.01), "battery-pack")
    S.struct_of("battery-case", "battery")
    q = [bw("battery", u, v)[:2] for u, v in ((-0.03, 0.024), (0.33, 0.024), (0.33, 0.109), (-0.03, 0.109))]
    S.swept("battery-case", "battery", q, -0.046, 0.046, "body", 0.18, "battery-pack", BATT, pen=PEN, tol=0.0)
    S.struct(S.ball("mass-battery", "battery", [0.03, 0.03, 0.03], c, BATT, 0.5), "battery", "mass")


def drive(S):
    S.struct(S.cyl("bb-bearing", "crank", [B[0], B[1], -0.05], [B[0], B[1], 0.05], 0.015, RIM, 0.5), "crank", "joint")
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        zc = side * 0.065
        a, b = [B[0] - sign * 0.015, B[1], zc], [B[0] + sign * 0.185, B[1], zc]
        S.put(S.box(f"crank-arm-{name}", "crank", [0.2, 0.03, 0.03], kit.lerp(a, b, 0.5), AL, 0.3), None)
        S.struct_of(f"crank-arm-{name}", "crank")
        S.swept(f"crank-arm-{name}", "crank", [[min(a[0], b[0]), B[1] - 0.015], [max(a[0], b[0]), B[1] - 0.015], [max(a[0], b[0]), B[1] + 0.015], [min(a[0], b[0]), B[1] + 0.015]], zc - 0.015, zc + 0.015, "body", 0.3, None, AL, pen=PEN)
        e = [B[0] + sign * kit.CRANK, B[1], 0.0]
        zc = side * 0.12
        S.put(S.box(f"pedal-{name}", "crank", [0.07, 0.03, 0.11], [e[0], e[1], zc], PLASTIC, DARK))
        S.struct_of(f"pedal-{name}", "crank")
        S.swept(f"pedal-{name}", "crank", [[e[0] - 0.035, e[1] - 0.015], [e[0] + 0.035, e[1] - 0.015], [e[0] + 0.035, e[1] + 0.015], [e[0] - 0.035, e[1] + 0.015]], zc - 0.055, zc + 0.055, "body", DARK, None, PLASTIC, pen=PEN)


def fork(S):
    reg = "frame"
    S.struct(S.pipe("steerer-f", "fork", fp(pt.CROWN_F[1]), fp(0.5), 0.015, 0.002, AL, 0.4), "fork")
    S.struct(S.cyl("headset-f", "fork", fp(pt.HEAD_F[0]), fp(pt.HEAD_F[1]), 0.02, AL, 0.5), "fork", "joint")
    S.put(S.box("fork-crown", "fork", [0.034, 0.048, 0.136], fp(pt.CROWN_F[1] - 0.008, 0.004), AL, TUBE, rotation=kit.rot_w(0.0)), reg)
    S.struct_of("fork-crown", "fork")
    S.plank("fork-crown", "fork", fp(pt.CROWN_F[0] - 0.004, 0.004), fp(pt.CROWN_F[1] + 0.03, 0.004), 0.048, 0.0, "body", TUBE, reg, AL, pen=PEN, wide=0.136)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = fp(pt.CROWN_F[0] + 0.005, 0.002, side * 0.052), fp(0.0, kit.OFFSET_F, side * 0.05)
        S.put(S.cap(f"fork-blade-{name}", "fork", a, b, 0.015, AL, TUBE), reg)
        S.struct(S.pipe(f"fork-blade-{name}", "fork", a, b, 0.015, 0.002, AL, 0.4), "fork")
        S.member(f"fork-blade-{name}", "fork", a, b, 0.03, "body", TUBE, reg, AL, pen=PEN)
    ax = [1.0, 0.0, 0.0]
    lamp = fp(sum(pt.CROWN_F) / 2 + 0.01, 0.07)
    S.put(S.cyl("head-lamp", "fork", kit.add(lamp, ax, -0.03), kit.add(lamp, ax, 0.035), 0.031, PLASTIC, 0.85, role="detail"))
    S.member("head-lamp", "fork", kit.add(lamp, ax, -0.03), kit.add(lamp, ax, 0.035), 0.062, "detail", 0.85, None, PLASTIC, pen=PEN)
    S.struct(S.ball("mass-fork", "fork", [0.03, 0.03, 0.03], fp(0.2, 0.02), AL, 0.5), "fork", "mass")


def cockpit(S):
    reg = "frame"
    S.struct(S.pipe("steerer-r", "bar", bp(0.04), bp(pt.RISER_TOP), 0.015, 0.002, AL, 0.4), "bar")
    S.struct(S.cyl("headset-r", "bar", bp(pt.HEAD_R[0]), bp(pt.HEAD_R[1]), 0.02, AL, 0.5), "bar", "joint")
    S.put(S.cyl("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.0175, AL, TUBE), reg)
    S.struct(S.pipe("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.0175, 0.002, AL, 0.4), "bar")
    S.member("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.035, "body", TUBE, reg, AL, pen=PEN)
    S.put(S.box("display", "bar", [0.05, 0.03, 0.075], kit.add(pt.CLAMP, [0.0, 0.017, 0.0]), PLASTIC, DARK, role="detail", rotation=kit.rot_w(-15.0)))
    S.plank("display", "bar", kit.add(pt.CLAMP, [-0.024, 0.01, 0]), kit.add(pt.CLAMP, [0.024, 0.024, 0]), 0.03, 0.0, "detail", DARK, None, PLASTIC, pen=PEN, wide=0.075)
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.bar_path(side, 2)
        end = pt.mirror(pt.BAR_END, side)
        S.put(S.tube(f"handlebar-{name}", "bar", [(p, 0.015, 0.015) for p in path[:-1]] + [(end, 0.016, 0.016)], AL, METAL, hint=[0.0, 1.0, 0.0]), reg)
        knots = [path[0], path[2], end]
        for k in range(2):
            S.struct(S.pipe(f"handlebar-{name}-{k + 1}", "bar", knots[k], knots[k + 1], 0.015, 0.002, AL, 0.5), "bar")
            S.member(f"handlebar-{name}-{k + 1}", "bar", knots[k], knots[k + 1], 0.03, "body", METAL, reg, AL, pen=PEN)
    S.struct(S.ball("mass-bar", "bar", [0.025, 0.025, 0.025], bp(0.72, 0.0), AL, 0.5), "bar", "mass")
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.cable_path(side)[::4] + [pt.cable_path(side)[-1]]
        S.struct(S.tube(f"steer-cable-{name}", "frame", [(p, 0.002, 0.002) for p in path], AL, 0.4, slender=True), "frame")


def box(S):
    reg = "box-wood"
    t = 0.02
    top = pt.BOX_YT + pt.CAP[1]
    side = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_XT + 0.004, top], [pt.BOX_X0, top]]
    for s, name in ((1, "l"), (-1, "r")):
        z0, z1 = sorted((s * pt.BOX_HW, s * (pt.BOX_HW - t)))
        S.put(S.prism(f"box-side-{name}", "box", side, z0, z1, PLY, WOOD), reg)
        S.struct_of(f"box-side-{name}", "box")
        S.swept(f"box-side-{name}", "box", side, z0, z1, "body", WOOD, reg, PLY, pen=PEN, tol=0.0)
    hw = pt.BOX_HW - t
    k = (pt.BOX_XT - pt.BOX_X1) / (pt.BOX_YT - pt.BOX_Y0)
    d = t * math.sqrt(1 + k * k)
    xf = pt.BOX_X1 + k * t
    floor = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [xf, pt.BOX_Y0 + t], [pt.BOX_X0, pt.BOX_Y0 + t]]
    rear = [[pt.BOX_X0, pt.BOX_Y0 + t], [pt.BOX_X0 + t, pt.BOX_Y0 + t], [pt.BOX_X0 + t, top], [pt.BOX_X0, top]]
    front = [[xf - d, pt.BOX_Y0 + t], [xf, pt.BOX_Y0 + t], [pt.BOX_XT + 0.004, top], [pt.BOX_XT + 0.004 - d, top]]
    bench = [[pt.BOX_X0 + t, pt.BOX_Y0 + t], [pt.BENCH_X1, pt.BOX_Y0 + t], [pt.BENCH_X1, pt.BENCH_Y], [pt.BOX_X0 + t, pt.BENCH_Y]]
    for i, prof, role, z in (("box-floor", floor, "body", pt.BOX_HW), ("box-rear", rear, "body", hw), ("box-front", front, "body", hw), ("bench", bench, "detail", hw)):
        S.put(S.prism(i, "box", prof, -z, z, PLY, WOOD if role == "body" else 0.8, role=role), reg)
        S.struct_of(i, "box")
        S.swept(i, "box", prof, -z, z, role, WOOD if role == "body" else 0.8, reg, PLY, pen=PEN, tol=0.0, views={"bench": ("elevation", "plan"), "box-floor": ("plan",)}.get(i, ALL))
    S.struct(S.ball("mass-box", "box", [0.04, 0.04, 0.04], [1.24, 0.42, 0.0], PLY, 0.5), "box", "mass")


def crates(S):
    reg = "cargo"
    L, W, H = pt.CRATE
    x0, y0 = kit.CRATE_O
    for k, name in ((0, "lower"), (1, "upper")):
        yb = y0 + k * H
        c = [x0 + L / 2, yb + H / 2, 0.0]
        S.put(S.box(f"crate-{name}", "crate", [L, H - 0.002, W], c, PLASTIC, CRATE + 0.07 * k, corner=0.01), reg)
        S.struct_of(f"crate-{name}", "crate")
        depth = [x0 + L, yb + H, -W / 2]
        S.flat("elevation", f"crate-{name}", "crate", "body", CRATE + 0.07 * k, reg, PLASTIC, [kit.loc("crate", q)[:2] for q in ([x0, yb + 0.001], [x0 + L, yb + 0.001], [x0 + L, yb + H - 0.001], [x0, yb + H - 0.001])], depth, pen=PEN, tol=0.0)
        S.flat("front", f"crate-{name}", "crate", "body", CRATE + 0.07 * k, reg, PLASTIC, [[-W / 2, yb + 0.001], [W / 2, yb + 0.001], [W / 2, yb + H - 0.001], [-W / 2, yb + H - 0.001]], depth, pen=PEN, tol=0.0)
    S.flat("plan", "crate-upper", "crate", "body", CRATE + 0.07, reg, PLASTIC, [[x0, -W / 2], [x0 + L, -W / 2], [x0 + L, W / 2], [x0, W / 2]], [x0 + L, y0 + 2 * H, -W / 2], pen=PEN, tol=0.0)
    S.struct(S.box("crate-post", "crate-y", [0.03, 2 * H - 0.03, 0.03], [x0 + 0.02, y0 + H, W / 2 - 0.02], PLASTIC, 0.4), "crate-y")
    S.struct(S.box("crate-base", "crate-x", [L - 0.04, 0.03, W - 0.04], [x0 + L / 2, y0 + 0.016, 0.0], PLASTIC, 0.4), "crate-x")
    S.struct(S.ball("mass-crates", "crate", [0.04, 0.04, 0.04], [x0 + L / 2, y0 + H, 0.0], PLASTIC, 0.5), "crate", "mass")


def stand(S):
    reg = "frame"
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.05), at(0, 0.05), 0.015, AL, 0.5), "stand", "joint")
    L = kit.STAND_LENGTH
    for side, name in ((1, "l"), (-1, "r")):
        a, b = at(0.0, side * 0.04), at(L, side * 0.205)
        S.put(S.cap(f"stand-leg-{name}", "stand", a, b, 0.015, AL, TUBE), reg)
        S.struct(S.pipe(f"stand-leg-{name}", "stand", a, b, 0.015, 0.002, AL, 0.4), "stand")
        S.member(f"stand-leg-{name}", "stand", a, b, 0.03, "body", TUBE, reg, AL, pen=PEN)


def build():
    S = kit.Scale(200, PENS)
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
    box(S)
    crates(S)
    stand(S)
    return S


DECISIONS = [
    {
        "en": "1:200 shows the long-john at 30 mm: each wheel one ring of tyre and rim with a plain hub and six spokes; the beam with a three-step gooseneck, both head tubes (the front one with its steerer cap, the rear one down to its drum), strut, oval down tube, one seat tube up to the saddle, the four stays generalised to 30 mm and the cast motor bracket; motor, battery, saddle pad, carrier hoop on two legs, mudguards, 30 mm crank bars and pedal blocks; fork crown block, two blades and the head lamp; riser, display and the swept handlebar with its grips; the box with 20 mm walls, floor and the bench block; each crate one block; the stand as two straight legs. The rotors are merged into the hubs, the box rails into the floor, the cables are left to the structure.",
        "de": "1:200 zeigt das Long-John bei 30 mm: jedes Rad ein Ring aus Reifen und Felge mit schlichter Nabe und sechs Speichen; der Träger mit dreistufigem Schwanenhals, beide Steuerrohre (das vordere mit seiner Schaftkappe, das hintere bis zur Trommel), Strebe, ovales Unterrohr, ein Sattelrohr bis zum Sattel, die vier Streben auf 30 mm verallgemeinert und die gegossene Motorkonsole; Motor, Akku, Sattelpolster, Gepäckträgerbügel auf zwei Beinen, Schutzbleche, 30-mm-Kurbelstäbe und Pedalblöcke; Gabelkronenblock, zwei Scheiden und der Scheinwerfer; Steigrohr, Display und der geschwungene Lenker mit seinen Griffen; die Box mit 20-mm-Wänden, Boden und dem Bankblock; jede Kiste ein Block; der Ständer als zwei gerade Beine. Die Bremsscheiben sind in die Naben eingeschlossen, die Boxschienen in den Boden, die Seile bleiben dem Tragwerk.",
    },
    {
        "en": "Thermal: every region references its body solids; the brake rotors are represented by the hub shells (region brakes), the grips by the handlebar ends (region frame, the saddle alone carries touch-parts). Only the total exposed area is compared at this scale.",
        "de": "Thermik: jede Region verweist auf ihre Körper; die Bremsscheiben werden durch die Nabenkörper vertreten (Region Bremsen), die Griffe durch die Lenkerenden (Region Rahmen, der Sattel allein trägt die Berührteile). Bei diesem Maßstab wird nur die gesamte freie Fläche verglichen.",
    },
    {
        "en": "Structure: beam and gooseneck as one hollow loft, head tubes, strut, down tube, seat tube and stays as hollow sections, bracket, carrier hoop and legs, steering cables; fork steerer, crown and blades, column steerer, riser and handlebar; wheel rings, six spokes, hubs and bearings; crank arms and pedals with the bearing; motor and battery; box floor, walls and bench; crate base and corner post; the stand legs with the hinge; the rear tyre patch as support; masses at their centres.",
        "de": "Tragwerk: Träger und Schwanenhals als ein hohler Loft, Steuerrohre, Strebe, Unterrohr, Sattelrohr und Streben als Hohlprofile, Konsole, Gepäckträgerbügel und Beine, Lenkseile; Gabelschaft, Krone und Scheiden, Lenksäulenschaft, Steigrohr und Lenker; Radringe, sechs Speichen, Naben und Lager; Kurbelarme und Pedale mit dem Lager; Motor und Akku; Boxboden, Wände und Bank; Kistenboden und Eckpfosten; die Ständerbeine mit dem Gelenk; die hintere Reifenaufstandsfläche als Auflager; Massen in ihren Schwerpunkten.",
    },
]

DRAWN = {
    "silhouette": {"en": "All three views are the true orthographic views of the 1:200 body in the parked pose: wheel rings with hubs and six spokes, frame, motor, battery, saddle, carrier, mudguards, fork with lamp, handlebar, box with bench, two crates and the stand.", "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:200-Körpers in der Parkstellung: Radringe mit Naben und sechs Speichen, Rahmen, Motor, Akku, Sattel, Gepäckträger, Schutzbleche, Gabel mit Scheinwerfer, Lenker, Box mit Bank, zwei Kisten und der Ständer."},
    "retained": {"en": "Every part of at least 30 mm; stays, fork blades, steerers, handlebar and stand legs generalised to 30 mm; the carrier as strokes.", "de": "Jedes Teil ab 30 mm; Streben, Gabelscheiden, Gabelschäfte, Lenker und Ständerbeine auf 30 mm verallgemeinert; der Gepäckträger als Striche."},
    "merged": {"en": "Tyre and rim into one ring, rotors into the hubs, post into the seat tube, grips into the handlebar ends, box rails into the floor, capping into the walls, each crate with its lid into one block.", "de": "Reifen und Felge in einen Ring, Bremsscheiben in die Naben, Stütze ins Sattelrohr, Griffe in die Lenkerenden, Boxschienen in den Boden, Kantenleisten in die Wände, jede Kiste mit Deckel in einen Block."},
    "omitted": {"en": "Chain, chainring, cables, dropouts, stand bracket and feet, mudguard stays, carrier struts and tail lamp, bell, levers, crate handles; the bench in front view (hidden by the box front).", "de": "Kette, Kettenblatt, Seile, Ausfallenden, Ständerkonsole und -füße, Schutzblechstreben, Gepäckträgerstreben und Rücklicht, Klingel, Hebel, Kistengriffe; die Bank in der Vorderansicht (von der Boxfront verdeckt)."},
    "thinElements": {"en": "Stays, fork blades, handlebar and stand legs are filled bars of 0.15 mm with a 0.13 mm outline; the carrier is a 0.13 mm stroke; spokes are 0.1 mm strokes.", "de": "Streben, Gabelscheiden, Lenker und Ständerbeine sind gefüllte Stäbe von 0,15 mm mit 0,13 mm Kontur; der Gepäckträger ist ein 0,13-mm-Strich; Speichen sind 0,1-mm-Striche."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Dark wheels, mid-tone frame, light box, grey crates, dark motor and battery.", "de": "Dunkle Räder, mitteltoniger Rahmen, helle Box, graue Kisten, dunkler Motor und Akku."},
    "overlays": {"en": "Shapes carry the thermal regions of their parts; members and extruded profiles follow steering, rolling, crank, stand and crate stack.", "de": "Die Formen tragen die thermischen Regionen ihrer Teile; Stäbe und extrudierte Profile folgen Lenken, Rollen, Kurbel, Ständer und Kistenstapel."},
}
