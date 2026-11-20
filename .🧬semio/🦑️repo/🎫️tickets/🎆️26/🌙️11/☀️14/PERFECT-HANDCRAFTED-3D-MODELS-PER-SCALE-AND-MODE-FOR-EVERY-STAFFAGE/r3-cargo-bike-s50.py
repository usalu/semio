"""🛺 Round 3 vehicles/cargo-bike at 1:50 (smallest feature 7.5 mm): every assembly of the real long-john cargo bike stated by hand, as body solid, thermal entry, structural part and drawn shape in one place."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-cargo-bike-parts")
kit = pt.kit
bp, fp = kit.bar_pt, kit.fork_pt
R, F, B, P = pt.R3, pt.F3, pt.B3, kit.P
ALL, SIDE = pt.ALL, pt.SIDE
AL, PLY, RUBBER, RIM, DISC, MOTOR, BATT, PLASTIC = "aluminium", "plywood", "rubber", "alloy-rim", "steel-disc", "motor-case", "battery-case", "plastic"
TUBE, WOOD, DARK, METAL, LIGHT, CRATE = 0.32, 0.72, 0.1, 0.7, 0.88, 0.55
PENS = {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}
THIN = 0.13


def bw(bone, u, v, z=0.0):
    """🧭 The world rest point of a bone-local (u, v, z)."""
    (ox, oy), deg = kit.BONES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [ox + u * c - v * s, oy + u * s + v * c, z]


def rounded(points, r, n=4):
    """⭕ A closed polygon with every corner rounded over the tangent length r (a pressed or cast outline)."""
    m = len(points)
    ring = [points[-1]] + points + [points[0]]
    out = []
    for k in range(1, m + 1):
        a, c, b = ring[k - 1], ring[k], ring[k + 1]
        ta = kit.add(c + [0.0] * (3 - len(c)), kit.unit(kit.sub(a + [0.0] * (3 - len(a)), c + [0.0] * (3 - len(c)))), r)
        tb = kit.add(c + [0.0] * (3 - len(c)), kit.unit(kit.sub(b + [0.0] * (3 - len(b)), c + [0.0] * (3 - len(c)))), r)
        out.extend([q[:2] for q in kit.bezier(ta, c + [0.0] * (3 - len(c)), tb, n)])
    clean = []
    for q in out:
        if not clean or math.hypot(q[0] - clean[-1][0], q[1] - clean[-1][1]) > 1e-6:
            clean.append(q)
    if math.hypot(clean[0][0] - clean[-1][0], clean[0][1] - clean[-1][1]) < 1e-6:
        clean.pop()
    return clean


def bar(S, i, bone, a, b, r, region, mat=AL, tone=TUBE, role="body", wall=0.0018, views=ALL, depth=None, pen=0.18, capped=False):
    """📏 A straight round tube: body cylinder, its hollow structural member and its member shape in every view."""
    S.put((S.cap if capped else S.cyl)(i, bone, a, b, r, mat, tone, role=role), region)
    S.struct(S.pipe(i, bone, a, b, r, wall, mat, 0.4), bone)
    S.member(i, bone, a, b, 2 * r, role, tone, region, mat, pen=pen, views=views, depth=depth)


def wheel(S, bone):
    w = pt.WHEELS[bone]
    c, tag = w["c"], bone[-1]
    centre, half, height = w["tyre"]
    S.put(S.rev(f"tyre-{tag}", bone, pt.tyre_profile(centre, half, height), c, [0, 0, 1], RUBBER, DARK), "tyres")
    outer, inner = pt.ring(c, w["out"], centre - height + 0.002)
    S.swept(f"tyre-{tag}", bone, outer, -half, half, "body", DARK, "tyres", RUBBER, holes=[inner], tol=0.0003, views=SIDE)
    for k, h in enumerate(pt.half_rings(c, w["out"], w["inner"], 24)):
        S.swept(f"tyre-{tag}-{k + 1}", bone, h, -half, half, "body", DARK, "tyres", RUBBER, tol=0.0003, views=("front", "plan"))
    S.put(S.rev(f"rim-{tag}", bone, w["rim"], c, [0, 0, 1], RIM, LIGHT))
    S.struct_of(f"rim-{tag}", bone)
    outer, inner = pt.ring(c, centre - height + 0.002, w["rim_r"][0])
    S.swept(f"rim-{tag}", bone, outer, -0.015, 0.015, "body", LIGHT, None, RIM, pen=THIN, holes=[inner], tol=0.0003, views=SIDE)
    S.put(S.rev(f"hub-{tag}", bone, w["hub"], c, [0, 0, 1], RIM, METAL))
    S.struct_of(f"hub-{tag}", bone)
    hz = w["hub"][-1][0]
    S.disc(f"hub-{tag}", bone, c, 0.066 if tag == "r" else 0.046, -hz, hz, "detail", METAL, None, RIM, pen=THIN)
    z0, z1, r0, r1 = w["rotor"]
    S.put(S.rev(f"rotor-{tag}", bone, [(z0, r0), (z0, r1), (z1, r1), (z1, r0), (z0, r0)], c, [0, 0, 1], DISC, 0.6, role="detail", texture=True), "brakes")
    S.disc(f"rotor-{tag}", bone, c, 2 * r1, z0, z1, "detail", 0.6, "brakes", DISC, pen=0.18, filled=False, views=SIDE)
    S.disc(f"rotor-{tag}-track", bone, c, 2 * r1 - 0.036, z0, z1, "texture", 0.6, "brakes", DISC, pen=0.1, filled=False, views=SIDE)
    a = w["axle"]
    S.put(S.cyl(f"axle-{tag}", bone, [c[0], c[1], -a], [c[0], c[1], a], 0.005, RIM, METAL, role="detail"))
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], -hz], [c[0], c[1], hz], 0.011, RIM, 0.5), bone, "joint")
    nut = w["nut"]
    for side, name in ((1, "l"), (-1, "r")):
        S.put(S.cyl(f"nut-{tag}-{name}", bone, [c[0], c[1], side * (nut - 0.004)], [c[0], c[1], side * (nut + 0.004)], 0.009, RIM, METAL, role="detail"))
    S.disc(f"axle-{tag}", bone, c, 0.018, -nut - 0.004, nut + 0.004, "detail", METAL, None, RIM, pen=THIN)
    k = 0
    for fz, fr, off in w["flanges"]:
        for lead in (True, False):
            k += 1
            sa, sb = pt.spoke(fr, fz, off + (0 if lead else 20), lead, w["spoke_r"])
            S.put(S.cap(f"spoke-{tag}-{k}", bone, kit.add(c, sa), kit.add(c, sb), 0.001, RIM, 0.6, role="texture", slender=True, texture=True, repeat={"count": 9, "rotationAxis": [0, 0, 1], "rotationRad": round(math.radians(40), 6), "origin": [0, 0, 0]}))
            S.struct_of(f"spoke-{tag}-{k}", bone)
    fz, fr, off = w["flanges"][1]
    for j in range(18):
        lead = j % 2 == 0
        sa, sb = pt.spoke(fr, fz, off + 20 * j, lead, w["spoke_r"])
        S.sliver(f"spoke-{tag}-{j + 1}", bone, kit.add(c, sa), kit.add(c, sb), "texture", 0.6, None, RIM, 0.1)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.03, 0.03, 0.03], c, RIM, 0.5), bone, "mass")
    if tag == "r":
        S.put(S.rev("sprocket", bone, [(-0.0505, 0.016), (-0.0505, pt.SPROCKET + 0.003), (-0.0485, pt.SPROCKET + 0.003), (-0.0485, 0.016)], c, [0, 0, 1], RIM, 0.5, role="detail", texture=True))
        S.disc("sprocket", bone, c, 2 * pt.SPROCKET + 0.006, -0.0505, -0.0485, "texture", 0.5, None, RIM, pen=0.1, filled=False, views=SIDE)


def frame(S):
    reg = "frame"
    beam = pt.beam_path()
    rows = [(p, pt.BEAM_HALF[0], pt.BEAM_HALF[1], 4.0) for p in beam]
    S.put(S.tube("beam", "frame", rows, AL, TUBE), reg)
    S.swept("beam", "frame", kit.ribbon([q[:2] for q in beam], pt.BEAM_HALF[1]), -pt.BEAM_HALF[0], pt.BEAM_HALF[0], "body", TUBE, reg, AL, pen=0.18, tol=0.0004)
    hy, hz, wall = pt.BEAM_HALF[1], pt.BEAM_HALF[0], 0.0025
    sect = [[pt.BEAM_Y - hy, -hz], [pt.BEAM_Y + hy, -hz], [pt.BEAM_Y + hy, hz], [pt.BEAM_Y - hy, hz]]
    hole = [[pt.BEAM_Y - hy + wall, -hz + wall], [pt.BEAM_Y + hy - wall, -hz + wall], [pt.BEAM_Y + hy - wall, hz - wall], [pt.BEAM_Y - hy + wall, hz - wall]]
    S.struct(S.solid("beam-box", "extrude", "frame", AL, 0.4, profile=sect, holes=[hole], plane="yz", **{"from": pt.BEAM_X0, "to": pt.BEAM_X1 - 0.08}), "frame")
    gs = beam[1:]
    S.struct(S.tube("gooseneck", "frame", [(p, pt.BEAM_HALF[0] - 0.003, pt.BEAM_HALF[1] - 0.003, 4.0) for p in gs], AL, 0.4), "frame")
    bar(S, "head-tube-f", "frame", fp(pt.HEAD_F[0]), fp(pt.HEAD_F[1]), 0.0225, reg, wall=0.003)
    bar(S, "head-tube-r", "frame", bp(pt.HEAD_R[0]), bp(pt.HEAD_R[1]), 0.0225, reg, wall=0.003)
    for i, a, b in (("cup-f-low", fp(pt.HEAD_F[0] - 0.004), fp(pt.HEAD_F[0] + 0.01)), ("cup-f-top", fp(pt.HEAD_F[1] - 0.01), fp(pt.HEAD_F[1] + 0.004)), ("cup-r-low", bp(pt.HEAD_R[0] - 0.004), bp(pt.HEAD_R[0] + 0.01)), ("cup-r-top", bp(pt.HEAD_R[1] - 0.01), bp(pt.HEAD_R[1] + 0.004))):
        S.put(S.cyl(i, "frame", a, b, 0.0255, AL, 0.25, role="detail"), reg)
        S.plank(i, "frame", a, b, 0.051, 0.0, "detail", 0.25, reg, AL, pen=THIN)
    bar(S, "head-strut", "frame", bp(0.13, -0.004), [0.70, 0.284, 0.0], 0.016, reg)
    down = [(kit.lerp(list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, t), 0.02, 0.025, 2.6) for t in (0.0, 0.5, 1.0)]
    S.put(S.tube("down-tube", "frame", down, AL, TUBE), reg)
    S.struct(S.pipe("down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, 0.022, 0.002, AL, 0.4), "frame")
    S.member("down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, 0.05, "body", TUBE, reg, AL, pen=0.18)
    bar(S, "seat-tube", "frame", kit.lerp(B, pt.SEAT_TOP, 0.05), pt.SEAT_TOP, 0.0175, reg)
    clamp = kit.add(pt.SEAT_TOP, [pt.ST[0], pt.ST[1], 0], 0.012)
    S.put(S.cyl("seat-clamp", "frame", kit.add(pt.SEAT_TOP, [pt.ST[0], pt.ST[1], 0], -0.008), clamp, 0.0205, AL, 0.25, role="detail"), reg)
    S.plank("seat-clamp", "frame", kit.add(pt.SEAT_TOP, [pt.ST[0], pt.ST[1], 0], -0.008), clamp, 0.041, 0.0, "detail", 0.25, reg, AL, pen=THIN)
    bar(S, "seat-post", "frame", kit.lerp(B, pt.SEAT_TOP, 0.85), pt.POST_TOP, 0.0158, reg, tone=METAL, pen=THIN, views=("elevation", "front"))
    for side, name in ((1, "l"), (-1, "r")):
        drop = [R[0], R[1], side * pt.STAY_Z]
        a, b = [B[0] - 0.035, B[1] - 0.004, side * 0.026], kit.add(drop, [0.012, 0.0, 0.0])
        S.put(S.cyl(f"chainstay-{name}", "frame", a, b, 0.0115, AL, TUBE, r1=0.0085), reg)
        S.struct_of(f"chainstay-{name}", "frame")
        S.member(f"chainstay-{name}", "frame", a, b, 0.02, "body", TUBE, reg, AL, pen=THIN, depth=[0.45, 0.31, side * 0.04])
        a, b = kit.add(pt.SEAT_TOP, [-0.012, -0.03, side * 0.022]), kit.add(drop, [0.006, 0.014, 0.0])
        S.put(S.cyl(f"seatstay-{name}", "frame", a, b, 0.0095, AL, TUBE, r1=0.0075), reg)
        S.struct_of(f"seatstay-{name}", "frame")
        S.member(f"seatstay-{name}", "frame", a, b, 0.017, "body", TUBE, reg, AL, pen=THIN)
        z0, z1 = sorted((side * 0.0675, side * 0.0755))
        plate = [[R[0] + dx, R[1] + dy] for dx, dy in ((0.03, -0.012), (0.03, 0.01), (0.018, 0.03), (-0.006, 0.032), (-0.02, 0.014), (-0.02, -0.012), (-0.006, -0.022), (0.014, -0.022))]
        S.put(S.prism(f"dropout-{name}", "frame", plate, z0, z1, AL, TUBE, role="detail"), reg)
        S.struct_of(f"dropout-{name}", "frame")
        S.swept(f"dropout-{name}", "frame", plate, z0, z1, "detail", TUBE, reg, AL, pen=THIN)
    cast = rounded([[0.395, 0.325], [0.425, 0.385], [0.50, 0.408], [0.56, 0.40], [0.605, 0.345], [0.618, 0.29], [0.618, 0.218], [0.572, 0.218], [0.574, 0.29], [0.552, 0.35], [0.50, 0.372], [0.44, 0.355], [0.42, 0.318]], 0.012, 3)
    S.put(S.prism("motor-bracket", "frame", cast, -0.045, 0.045, AL, 0.28), reg)
    S.struct_of("motor-bracket", "frame")
    S.swept("motor-bracket", "frame", cast, -0.045, 0.045, "body", 0.28, reg, AL, pen=0.18, tol=0.0006)
    bridge = [0.395, 0.318]
    S.put(S.cyl("chainstay-bridge", "frame", [bridge[0], bridge[1], -0.045], [bridge[0], bridge[1], 0.045], 0.007, AL, TUBE, role="detail"), reg)
    S.disc("chainstay-bridge", "frame", bridge, 0.014, -0.045, 0.045, "detail", TUBE, reg, AL, pen=THIN, views=("front", "plan"))
    for k, x in enumerate((0.95, 1.27, 1.57)):
        prof = [[x - 0.02, 0.255], [x + 0.02, 0.255], [x + 0.02, 0.29], [x - 0.02, 0.29]]
        S.put(S.prism(f"box-rail-{k + 1}", "frame", prof, -0.29, 0.29, AL, 0.35, role="detail"), reg)
        S.struct_of(f"box-rail-{k + 1}", "frame")
        S.swept(f"box-rail-{k + 1}", "frame", prof, -0.29, 0.29, "detail", 0.35, reg, AL, pen=THIN, views=ALL if k == 2 else ("elevation", "plan"))
    brk = [[P[0] - 0.045, 0.222], [P[0] + 0.045, 0.222], [P[0] + 0.022, 0.19], [P[0] - 0.022, 0.19]]
    S.put(S.prism("stand-bracket", "frame", brk, -0.028, 0.028, AL, TUBE, role="detail"), reg)
    S.struct_of("stand-bracket", "frame")
    S.swept("stand-bracket", "frame", brk, -0.028, 0.028, "detail", TUBE, reg, AL, pen=THIN)
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.045, 0.045, 0.045], [fx, fy, 0], AL, 0.5), "frame", "mass")
    S.struct(S.box("contact-pad-r", "frame", [0.04, 0.0075, 0.04], [0.0, 0.00375, 0.0], RUBBER, 0.1), "frame", "support", anchor="contact-r")


def carrier(S):
    reg = "frame"
    x0, x1 = pt.CARRIER_X
    y, z = pt.CARRIER_Y, pt.CARRIER_Z
    rails = {"carrier-rail-l": ([x0, y, z], [x1, y, z]), "carrier-rail-r": ([x0, y, -z], [x1, y, -z])}
    for side, name in ((1, "l"), (-1, "r")):
        eye = [R[0] - 0.004, R[1] + 0.016, side * 0.081]
        rails[f"carrier-leg-a-{name}"] = ([-0.04, y, side * z], eye)
        rails[f"carrier-leg-b-{name}"] = ([-0.24, y, side * z], eye)
        rails[f"carrier-strut-{name}"] = ([x1, y, side * 0.05], kit.add(pt.SEAT_TOP, [-0.03, -0.06, side * 0.026]))
    for i, (a, b) in rails.items():
        S.put(S.cap(i, "frame", a, b, 0.006, AL, TUBE), reg)
        S.struct_of(i, "frame")
        S.member(i, "frame", a, b, 0.012, "detail", TUBE, reg, AL, pen=THIN, views=ALL)
    for k, x in enumerate((x0, -0.17, -0.04, 0.09, x1)):
        i = f"carrier-bar-{k + 1}"
        S.put(S.cap(i, "frame", [x, y, -z], [x, y, z], 0.005, AL, TUBE, role="detail"), reg)
        S.disc(i, "frame", [x, y], 0.01, -z, z, "detail", TUBE, reg, AL, pen=THIN, filled=False, views=("plan",))
    plate = [[x0 - 0.012, y - 0.06], [x0 + 0.006, y - 0.06], [x0 + 0.006, y + 0.004], [x0 - 0.012, y + 0.004]]
    S.put(S.prism("carrier-tail", "frame", plate, -0.06, 0.06, AL, TUBE, role="detail"), reg)
    S.swept("carrier-tail", "frame", plate, -0.06, 0.06, "detail", TUBE, reg, AL, pen=THIN)
    c = [x0 - 0.022, y - 0.035, 0.0]
    S.put(S.box("tail-lamp", "frame", [0.022, 0.035, 0.09], c, PLASTIC, 0.2, role="detail", corner=0.005))
    S.swept("tail-lamp", "frame", [[c[0] - 0.011, c[1] - 0.0175], [c[0] + 0.011, c[1] - 0.0175], [c[0] + 0.011, c[1] + 0.0175], [c[0] - 0.011, c[1] + 0.0175]], -0.045, 0.045, "detail", 0.2, None, PLASTIC, pen=THIN)


def saddle(S):
    rows = [([x, yc, 0.0], w, t, 2.6) for x, yc, w, t in pt.SADDLE]
    S.put(S.tube("saddle", "frame", rows, PLASTIC, 0.15), "touch-parts")
    top = [[x, yc + t] for x, yc, w, t in pt.SADDLE]
    bottom = [[x, yc - t] for x, yc, w, t in pt.SADDLE]
    S.flat("elevation", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, top + bottom[::-1], [0.34, 1.0, -0.107])
    S.flat("plan", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[x, w] for x, yc, w, t in pt.SADDLE] + [[x, -w] for x, yc, w, t in pt.SADDLE][::-1], [0.34, 1.0, -0.107])
    x, yc, w, t = pt.SADDLE[6]
    front = [[w * math.copysign(abs(math.cos(a)) ** (2 / 2.6), math.cos(a)), yc + t * math.copysign(abs(math.sin(a)) ** (2 / 2.6), math.sin(a))] for a in [2 * math.pi * k / 28 for k in range(28)]]
    S.flat("front", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, front, [0.34, 1.0, -0.107])
    reg = "frame"
    c = [pt.POST_TOP[0], pt.POST_TOP[1] + 0.006, 0.0]
    S.put(S.box("saddle-clamp", "frame", [0.05, 0.032, 0.05], c, AL, METAL, role="detail", corner=0.006), reg)
    S.swept("saddle-clamp", "frame", [[c[0] - 0.025, c[1] - 0.016], [c[0] + 0.025, c[1] - 0.016], [c[0] + 0.025, c[1] + 0.016], [c[0] - 0.025, c[1] + 0.016]], -0.025, 0.025, "detail", METAL, reg, AL, pen=THIN)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = [0.30, 0.968, side * 0.012], [0.105, 0.966, side * 0.04]
        S.put(S.cap(f"saddle-rail-{name}", "frame", a, b, 0.0035, AL, METAL, role="detail", slender=True), reg)
        S.wire(f"saddle-rail-{name}", a, b, "detail", METAL, reg, AL, THIN)


def guards(S):
    reg = "fenders"
    for i, bone, c, a0, a1, r in (("fender-r", "frame", R, 18.0, 196.0, pt.FENDER_R[0]), ("fender-f", "fork", F, 18.0, 150.0, pt.FENDER_R[1])):
        deg = kit.BONES[bone][1]
        o = kit.loc(bone, c)
        s = S.solid(i, "revolve", bone, PLASTIC, 0.22, profile=[[kit.r5(z), kit.r5(q)] for z, q in pt.fender_profile(r)], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec(turn_offset(o, a0 - deg))}
        S.put(s, reg)
        band = kit.arc(c, r + 0.0013, a0, a1, 40) + kit.arc(c, r - 0.0122, a1, a0, 40)
        S.swept(i, bone, band, -pt.FENDER_HALF, pt.FENDER_HALF, "body", 0.22, reg, PLASTIC, pen=0.18, tol=0.0004)
    for side, name in ((1, "l"), (-1, "r")):
        for k, deg in enumerate((150.0, 105.0)):
            a = [R[0] - 0.014, R[1] + 0.006, side * 0.08]
            b = kit.polar(R, pt.FENDER_R[0] - 0.012, deg, side * 0.03)
            i = f"fender-stay-r{k + 1}-{name}"
            S.put(S.cap(i, "frame", a, b, 0.002, AL, METAL, role="detail", slender=True), reg)
            S.wire(i, a, b, "detail", METAL, reg, AL, 0.1)
        a = [F[0] - 0.004, F[1] + 0.006, side * 0.058]
        b = kit.polar(F, pt.FENDER_R[1] - 0.012, 40.0, side * 0.03)
        i = f"fender-stay-f-{name}"
        S.put(S.cap(i, "fork", a, b, 0.002, AL, METAL, role="detail", slender=True), reg)
        S.member(i, "fork", a, b, 0.004, "detail", METAL, reg, AL, pen=0.1, filled=False)


def turn_offset(o, deg):
    """↩️ The translation that keeps the point o fixed when a solid is turned by deg about the bone origin's w axis."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [o[0] - (o[0] * c - o[1] * s), o[1] - (o[0] * s + o[1] * c), 0.0]


MOTOR = [[0.11, -0.035], [0.085, 0.05], [0.04, 0.072], [-0.04, 0.068], [-0.072, 0.02], [-0.065, -0.05], [-0.02, -0.088], [0.07, -0.082]]


def motor(S):
    out = [[B[0] + x, B[1] + y] for x, y in rounded(MOTOR, 0.02, 4)]
    S.put(S.prism("motor-housing", "motor", out, -0.0425, 0.0425, MOTOR_MAT, 0.3), "drive-motor")
    S.struct_of("motor-housing", "motor")
    S.swept("motor-housing", "motor", out, -0.0425, 0.0425, "body", 0.3, "drive-motor", MOTOR_MAT, pen=0.18, tol=0.0006)
    cov = [[B[0] + 0.045 * math.cos(2 * math.pi * k / 32), B[1] + 0.045 * math.sin(2 * math.pi * k / 32)] for k in range(32)]
    S.flat("elevation", "motor-cover", "motor", "detail", 0.22, "drive-motor", MOTOR_MAT, [kit.loc("motor", q)[:2] for q in cov], [B[0] + 0.11, B[1] + 0.07, -0.0425], pen=THIN, filled=False)
    S.struct(S.ball("mass-motor", "motor", [0.04, 0.04, 0.04], [B[0] + 0.02, B[1] - 0.01, 0.0], MOTOR_MAT, 0.5), "motor", "mass")


MOTOR_MAT = "motor-case"


def battery(S):
    c = bw("battery", 0.16, 0.0665)
    S.put(S.box("battery-case", "battery", [0.34, 0.085, 0.092], c, BATT, 0.18, corner=0.014, rotation=kit.rot_w(0.0)), "battery-pack")
    S.struct_of("battery-case", "battery")
    q = [bw("battery", u, v)[:2] for u, v in ((-0.01, 0.024), (0.33, 0.024), (0.33, 0.109), (-0.01, 0.109))]
    S.swept("battery-case", "battery", q, -0.046, 0.046, "body", 0.18, "battery-pack", BATT, pen=0.18, tol=0.0)
    lock = bw("battery", -0.022, 0.05)
    S.put(S.box("battery-lock", "battery", [0.024, 0.06, 0.06], lock, BATT, 0.1, role="detail", corner=0.006))
    S.struct(S.ball("mass-battery", "battery", [0.03, 0.03, 0.03], c, BATT, 0.5), "battery", "mass")
    q = [bw("battery", u, v)[:2] for u, v in ((-0.034, 0.02), (-0.01, 0.02), (-0.01, 0.08), (-0.034, 0.08))]
    S.swept("battery-lock", "battery", q, -0.03, 0.03, "detail", 0.1, None, BATT, pen=THIN, tol=0.0)


def drive(S):
    for a, b, k in ((p, q, n + 1) for n, (p, q) in enumerate(pt.chain_runs())):
        i = f"chain-{k}"
        a3, b3 = [a[0], a[1], pt.CHAINRING[1]], [b[0], b[1], pt.CHAINRING[1]]
        S.put(S.cap(i, "frame", a3, b3, 0.0038, RIM, 0.25, role="detail", slender=True))
        S.member(i, "frame", a3, b3, 0.0076, "detail", 0.25, None, RIM, pen=THIN)
    z = pt.CHAINRING[1]
    S.put(S.rev("chainring", "crank", [(z - 0.002, 0.03), (z - 0.002, pt.CHAINRING[0] + 0.003), (z + 0.002, pt.CHAINRING[0] + 0.003), (z + 0.002, 0.03)], B, [0, 0, 1], RIM, 0.5, role="detail", texture=True))
    S.disc("chainring", "crank", B, 2 * pt.CHAINRING[0] + 0.006, z - 0.002, z + 0.002, "texture", 0.5, None, RIM, pen=0.1, filled=False, views=SIDE)
    S.put(S.rev("chain-guard", "crank", [(z - 0.006, 0.07), (z - 0.006, 0.089), (z - 0.002, 0.089), (z - 0.002, 0.07)], B, [0, 0, 1], PLASTIC, 0.2, role="detail", texture=True))
    S.disc("chain-guard", "crank", B, 0.178, z - 0.006, z - 0.002, "detail", 0.2, None, PLASTIC, pen=0.18, filled=False, views=SIDE)
    S.put(S.cyl("crank-axle", "crank", [B[0], B[1], -0.072], [B[0], B[1], 0.072], 0.009, RIM, METAL, role="detail"))
    S.struct_of("crank-axle", "crank")
    S.struct(S.cyl("bb-bearing", "crank", [B[0], B[1], -0.05], [B[0], B[1], 0.05], 0.014, RIM, 0.5), "crank", "joint")
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        zc = side * 0.064
        rows = [(-0.022, 0.013), (-0.012, 0.018), (0.0, 0.019), (0.02, 0.016), (0.08, 0.012), (0.15, 0.0105), (0.17, 0.012), (0.182, 0.008)]
        S.put(S.tube(f"crank-arm-{name}", "crank", [([B[0] + sign * x, B[1], zc], 0.0065, h, 3.0) for x, h in rows], AL, 0.3), None)
        S.struct_of(f"crank-arm-{name}", "crank")
        S.swept(f"crank-arm-{name}", "crank", [[B[0] + sign * x, B[1] + h] for x, h in rows] + [[B[0] + sign * x, B[1] - h] for x, h in rows][::-1], zc - 0.0065, zc + 0.0065, "body", 0.3, None, AL, pen=THIN)
        e = [B[0] + sign * kit.CRANK, B[1], 0.0]
        S.put(S.cyl(f"pedal-spindle-{name}", "crank", [e[0], e[1], side * 0.069], [e[0], e[1], side * 0.088], 0.0065, RIM, METAL, role="detail"))
        S.struct_of(f"pedal-spindle-{name}", "crank")
        zc = side * 0.13
        S.put(S.box(f"pedal-{name}", "crank", [0.07, 0.024, 0.09], [e[0], e[1], zc], PLASTIC, DARK, corner=0.006))
        S.swept(f"pedal-{name}", "crank", [[e[0] - 0.035, e[1] - 0.012], [e[0] + 0.035, e[1] - 0.012], [e[0] + 0.035, e[1] + 0.012], [e[0] - 0.035, e[1] + 0.012]], zc - 0.045, zc + 0.045, "body", DARK, None, PLASTIC, pen=THIN)


def fork(S):
    reg = "frame"
    S.struct(S.pipe("steerer-f", "fork", fp(pt.CROWN_F[1]), fp(0.5), 0.0143, 0.002, AL, 0.4), "fork")
    S.put(S.cyl("steerer-cap-f", "fork", fp(pt.HEAD_F[1] + 0.004), fp(0.5), 0.0155, AL, 0.25, role="detail"), reg)
    S.plank("steerer-cap-f", "fork", fp(pt.HEAD_F[1] + 0.004), fp(0.5), 0.031, 0.0, "detail", 0.25, reg, AL, pen=THIN)
    S.put(S.cyl("steerer-f", "fork", fp(pt.CROWN_F[1]), fp(pt.HEAD_F[0] - 0.004), 0.0143, AL, 0.25, role="detail"), reg)
    for u, name in ((pt.HEAD_F[0], "low"), (pt.HEAD_F[1], "top")):
        S.struct(S.pipe(f"bearing-f-{name}", "fork", fp(u - 0.006), fp(u + 0.006), 0.022, 0.006, AL, 0.5), "fork", "joint")
    crown = [(fp(sum(pt.CROWN_F) / 2, 0.004, z), ru, rv, 3.2) for z, ru, rv in ((-0.068, 0.011, 0.016), (-0.056, 0.012, 0.02), (-0.025, 0.012, 0.024), (0.025, 0.012, 0.024), (0.056, 0.012, 0.02), (0.068, 0.011, 0.016))]
    S.put(S.tube("fork-crown", "fork", crown, AL, TUBE, hint=[kit.UP_F[0], kit.UP_F[1], 0.0]), reg)
    S.struct_of("fork-crown", "fork")
    S.plank("fork-crown", "fork", fp(pt.CROWN_F[0], 0.004), fp(pt.CROWN_F[1], 0.004), 0.048, 0.0, "body", TUBE, reg, AL, pen=THIN, wide=0.136)
    for side, name in ((1, "l"), (-1, "r")):
        rows = [(fp(pt.CROWN_F[0] + 0.005, 0.002, side * 0.052), 0.012, 0.016), (fp(0.15, 0.019, side * 0.051), 0.011, 0.014), (fp(0.02, kit.OFFSET_F, side * 0.05), 0.009, 0.011)]
        S.put(S.tube(f"fork-blade-{name}", "fork", rows, AL, TUBE), reg)
        S.struct(S.pipe(f"fork-blade-{name}", "fork", rows[0][0], rows[-1][0], 0.012, 0.0018, AL, 0.4), "fork")
        S.member(f"fork-blade-{name}", "fork", rows[0][0], rows[-1][0], 0.028, "body", TUBE, reg, AL, pen=THIN)
        d0, d1 = pt.WHEELS["wheel-f"]["drop"]
        a, b = [F[0], F[1], side * d0], [F[0], F[1], side * d1]
        S.put(S.cyl(f"fork-end-{name}", "fork", a, b, 0.013, AL, TUBE, role="detail"), reg)
        S.struct_of(f"fork-end-{name}", "fork")
    S.put(S.rev("drum-f", "fork", [(0.0, 0.01), (0.0, 0.034), (pt.DRUM_F[1] - pt.DRUM_F[0], 0.034), (pt.DRUM_F[1] - pt.DRUM_F[0], 0.01)], fp(pt.DRUM_F[0]), [kit.UP_F[0], kit.UP_F[1], 0.0], AL, 0.3, role="detail"), reg)
    S.plank("drum-f", "fork", fp(pt.DRUM_F[0]), fp(pt.DRUM_F[1]), 0.068, 0.0, "detail", 0.3, reg, AL, pen=THIN, wide=0.068)
    cal = kit.polar(F, 0.075, 128.0, 0.0345)
    S.put(S.box("caliper-f", "fork", [0.06, 0.03, 0.04], cal, AL, 0.2, role="detail", corner=0.006, rotation=kit.rot_w(128.0 - 90.0 + 70.0)))
    S.plank("caliper-f", "fork", kit.polar(F, 0.075, 150.0), kit.polar(F, 0.075, 106.0), 0.03, 0.0345, "detail", 0.2, None, AL, pen=THIN, wide=0.04, views=("elevation", "plan"))
    ax = [math.cos(math.radians(4)), math.sin(math.radians(4)), 0.0]
    lamp = fp(sum(pt.CROWN_F) / 2 + 0.03, 0.095)
    S.put(S.rev("head-lamp", "fork", [(-0.032, 0.01), (-0.026, 0.02), (-0.01, 0.028), (0.02, 0.031), (0.03, 0.031)], lamp, ax, PLASTIC, 0.2, role="detail"))
    S.put(S.rev("head-lamp-lens", "fork", [(0.029, 0.027), (0.034, 0.02), (0.037, 0.01), (0.038, 0.001)], lamp, ax, PLASTIC, 0.92, role="detail"))
    S.member("head-lamp", "fork", kit.add(lamp, ax, -0.026), kit.add(lamp, ax, 0.03), 0.062, "detail", 0.2, None, PLASTIC, pen=THIN)
    S.member("head-lamp-lens", "fork", kit.add(lamp, ax, 0.03), kit.add(lamp, ax, 0.037), 0.054, "detail", 0.92, None, PLASTIC, pen=THIN, views=("elevation", "front"))
    a, b = fp(pt.CROWN_F[1] - 0.006, 0.02), kit.add(lamp, ax, -0.028)
    S.put(S.cap("lamp-bracket", "fork", a, b, 0.0045, AL, METAL, role="detail"), reg)
    S.plank("lamp-bracket", "fork", a, b, 0.009, 0.0, "detail", METAL, reg, AL, pen=THIN)
    S.struct(S.ball("mass-fork", "fork", [0.03, 0.03, 0.03], fp(0.2, 0.02), AL, 0.5), "fork", "mass")


def cockpit(S):
    reg = "frame"
    S.struct(S.pipe("steerer-r", "bar", bp(0.04), bp(pt.RISER_TOP), 0.0143, 0.002, AL, 0.4), "bar")
    for u, name in ((pt.HEAD_R[0], "low"), (pt.HEAD_R[1], "top")):
        S.struct(S.pipe(f"bearing-r-{name}", "bar", bp(u - 0.006), bp(u + 0.006), 0.022, 0.006, AL, 0.5), "bar", "joint")
    S.put(S.cyl("steerer-r", "bar", bp(0.04), bp(pt.HEAD_R[0] - 0.004), 0.0143, AL, 0.25, role="detail"), reg)
    S.plank("steerer-r", "bar", bp(0.04), bp(pt.HEAD_R[0] - 0.004), 0.0286, 0.0, "detail", 0.25, reg, AL, pen=THIN)
    S.put(S.rev("drum-r", "bar", [(0.0, 0.01), (0.0, 0.034), (pt.DRUM_R[1] - pt.DRUM_R[0], 0.034), (pt.DRUM_R[1] - pt.DRUM_R[0], 0.01)], bp(pt.DRUM_R[0]), [kit.UP_R[0], kit.UP_R[1], 0.0], AL, 0.3, role="detail"), reg)
    S.plank("drum-r", "bar", bp(pt.DRUM_R[0]), bp(pt.DRUM_R[1]), 0.068, 0.0, "detail", 0.3, reg, AL, pen=THIN, wide=0.068)
    S.put(S.cyl("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.0175, AL, TUBE), reg)
    S.struct(S.pipe("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.0175, 0.002, AL, 0.4), "bar")
    S.member("riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP), 0.035, "body", TUBE, reg, AL, pen=0.18)
    S.put(S.cap("stem", "bar", bp(pt.CLAMP_S - 0.004, 0.0), pt.CLAMP, 0.016, AL, TUBE), reg)
    S.struct_of("stem", "bar")
    S.plank("stem", "bar", bp(pt.CLAMP_S - 0.004, 0.0), pt.CLAMP, 0.032, 0.0, "body", TUBE, reg, AL, pen=THIN, wide=0.032)
    a, b = [pt.CLAMP[0], pt.CLAMP[1], -0.025], [pt.CLAMP[0], pt.CLAMP[1], 0.025]
    S.put(S.cyl("bar-clamp", "bar", a, b, 0.019, AL, 0.25, role="detail"), reg)
    S.struct_of("bar-clamp", "bar")
    S.disc("bar-clamp", "bar", pt.CLAMP, 0.038, -0.025, 0.025, "detail", 0.25, reg, AL, pen=THIN)
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.bar_path(side)
        g0 = pt.grip_start(side)
        end = pt.mirror(pt.BAR_END, side)
        S.put(S.tube(f"handlebar-{name}", "bar", [(p, 0.0127, 0.0127) for p in path[:-1] + [kit.lerp(g0, end, 0.5)]], AL, METAL, hint=[0.0, 1.0, 0.0]), reg)
        knots = [path[0], path[2], path[4], path[6], g0, end]
        for k in range(len(knots) - 1):
            if k < 4:
                S.member(f"handlebar-{name}-{k + 1}", "bar", knots[k], knots[k + 1], 0.0254, "body", METAL, reg, AL, pen=THIN)
        knots = [q for q in path[:-1] if kit.norm(kit.sub(q, g0)) > 0.02] + [g0, end]
        for k in range(len(knots) - 1):
            S.struct(S.pipe(f"handlebar-{name}-{k + 1}", "bar", knots[k], knots[k + 1], 0.0127, 0.0018, AL, 0.5), "bar")
        S.put(S.cyl(f"grip-{name}", "bar", g0, end, 0.0165, RUBBER, DARK), "touch-parts")
        S.member(f"grip-{name}", "bar", g0, end, 0.033, "body", DARK, "touch-parts", RUBBER, pen=THIN)
        d = kit.unit(kit.sub(end, g0))
        hinge = kit.add(kit.add(g0, d, -0.025), [0.012, 0.012, 0.0])
        S.put(S.cap(f"lever-body-{name}", "bar", kit.add(g0, d, -0.035), hinge, 0.0105, AL, 0.25, role="detail"))
        tip = kit.add(kit.add(g0, d, 0.08), [0.035, -0.008, 0.0])
        S.put(S.cap(f"lever-{name}", "bar", hinge, tip, 0.005, AL, 0.75, role="detail"))
        S.plank(f"lever-{name}", "bar", hinge, tip, 0.01, side * 0.27, "detail", 0.75, None, AL, pen=THIN, wide=0.01)
    disp = kit.add(pt.CLAMP, [0.006, 0.024, 0.0])
    S.put(S.box("display", "bar", [0.05, 0.016, 0.075], disp, PLASTIC, DARK, role="detail", corner=0.004, rotation=kit.rot_w(-15.0)))
    S.plank("display", "bar", kit.add(disp, [-0.024, -0.006, 0]), kit.add(disp, [0.024, 0.006, 0]), 0.016, 0.0, "detail", DARK, None, PLASTIC, pen=THIN, wide=0.075)
    g0 = pt.grip_start(1)
    d = kit.unit(kit.sub(pt.mirror(pt.BAR_END, 1), g0))
    base = kit.add(kit.add(g0, d, -0.06), [0.0, 0.0127, 0.0])
    S.put(S.rev("bell", "bar", [(0.0, 0.026), (0.012, 0.026), (0.022, 0.02), (0.028, 0.01), (0.03, 0.001)], base, [0, 1, 0], AL, 0.8, role="detail"))
    S.plank("bell", "bar", base, kit.add(base, [0, 0.026, 0]), 0.052, base[2], "detail", 0.8, None, AL, pen=THIN, wide=0.052, d1=0.02)
    S.struct(S.ball("mass-bar", "bar", [0.025, 0.025, 0.025], bp(0.72, 0.0), AL, 0.5), "bar", "mass")


def cables(S):
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.cable_path(side)
        S.put(S.tube(f"steer-cable-{name}", "frame", [(p, 0.003, 0.003) for p in path], RUBBER, 0.12, role="detail", slender=True))
        S.struct(S.tube(f"steer-cable-{name}", "frame", [(p, 0.002, 0.002) for p in path], AL, 0.4, slender=True), "frame")
        if side < 0:
            S.flat("elevation", f"steer-cable-{name}", "frame", "detail", 0.12, None, RUBBER, [q[:2] for q in path], [path[0][0], path[0][1], -0.034], pen=0.13, filled=False, closed=False, tol=0.001)
        S.flat("plan", f"steer-cable-{name}", "frame", "detail", 0.12, None, RUBBER, [[q[0], q[2]] for q in path], [path[0][0], 0.27, side * 0.034], pen=0.13, filled=False, closed=False, tol=0.001)


def box(S):
    reg = "box-wood"
    side = [[x, y] for x, y in pt.box_side()]
    for s, name in ((1, "l"), (-1, "r")):
        z0, z1 = sorted((s * pt.BOX_HW, s * (pt.BOX_HW - pt.PLY)))
        S.put(S.prism(f"box-side-{name}", "box", side, z0, z1, PLY, WOOD), reg)
        S.struct_of(f"box-side-{name}", "box")
        S.swept(f"box-side-{name}", "box", side, z0, z1, "body", WOOD, reg, PLY, pen=0.18, tol=0.0)
    hw = pt.BOX_HW - pt.PLY
    floor = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_X1 + 0.003, pt.BOX_YF], [pt.BOX_X0, pt.BOX_YF]]
    rear = [[pt.BOX_X0, pt.BOX_YF], [pt.BOX_X0 + pt.PLY, pt.BOX_YF], [pt.BOX_X0 + pt.PLY, pt.BOX_YT], [pt.BOX_X0, pt.BOX_YT]]
    seat = [[pt.BOX_X0 + pt.PLY, pt.BENCH_Y - pt.PLY], [pt.BENCH_X1, pt.BENCH_Y - pt.PLY], [pt.BENCH_X1, pt.BENCH_Y], [pt.BOX_X0 + pt.PLY, pt.BENCH_Y]]
    panel = [[pt.BENCH_X1 - pt.PLY, pt.BOX_YF], [pt.BENCH_X1, pt.BOX_YF], [pt.BENCH_X1, pt.BENCH_Y - pt.PLY], [pt.BENCH_X1 - pt.PLY, pt.BENCH_Y - pt.PLY]]
    for i, prof, role, z in (("box-floor", floor, "body", pt.BOX_HW), ("box-rear", rear, "body", hw), ("box-front", pt.front_wall(), "body", hw), ("bench-seat", seat, "detail", hw), ("bench-panel", panel, "detail", hw)):
        S.put(S.prism(i, "box", prof, -z, z, PLY, WOOD if role == "body" else 0.8, role=role), reg)
        S.struct_of(i, "box")
        S.swept(i, "box", prof, -z, z, role, WOOD if role == "body" else 0.8, reg, PLY, pen=0.18 if role == "body" else THIN, tol=0.0)
    cw, ch = pt.CAP
    y0, y1 = pt.BOX_YT, pt.BOX_YT + ch
    for s, name in ((1, "l"), (-1, "r")):
        prof = [[pt.BOX_X0 - 0.004, y0], [pt.BOX_XT + 0.006, y0], [pt.BOX_XT + 0.006, y1], [pt.BOX_X0 - 0.004, y1]]
        z0, z1 = sorted((s * (pt.BOX_HW + 0.004), s * (pt.BOX_HW + 0.004 - cw)))
        S.put(S.prism(f"box-cap-{name}", "box", prof, z0, z1, PLY, 0.5, role="detail"), reg)
        S.swept(f"box-cap-{name}", "box", prof, z0, z1, "detail", 0.5, reg, PLY, pen=THIN, tol=0.0)
    for i, x in (("box-cap-rear", pt.BOX_X0 - 0.004), ("box-cap-front", pt.BOX_XT + 0.006 - cw)):
        prof = [[x, y0], [x + cw, y0], [x + cw, y1], [x, y1]]
        S.put(S.prism(i, "box", prof, -pt.BOX_HW + cw, pt.BOX_HW - cw, PLY, 0.5, role="detail"), reg)
        S.swept(i, "box", prof, -pt.BOX_HW + cw, pt.BOX_HW - cw, "detail", 0.5, reg, PLY, pen=THIN, tol=0.0)
    S.struct(S.ball("mass-box", "box", [0.04, 0.04, 0.04], [1.24, 0.42, 0.0], PLY, 0.5), "box", "mass")


def crates(S):
    reg = "cargo"
    L, W, H = pt.CRATE
    x0, y0 = kit.CRATE_O
    for k, name in ((0, "lower"), (1, "upper")):
        yb = y0 + k * H
        rows = [([x0 + L / 2, yb + 0.002, 0.0], W / 2 - 0.018, L / 2 - 0.018, 6.0), ([x0 + L / 2, yb + 0.012, 0.0], W / 2 - 0.016, L / 2 - 0.016, 6.0), ([x0 + L / 2, yb + H - 0.045, 0.0], W / 2 - 0.006, L / 2 - 0.006, 6.0), ([x0 + L / 2, yb + H - 0.044, 0.0], W / 2, L / 2, 6.0), ([x0 + L / 2, yb + H - 0.014, 0.0], W / 2, L / 2, 6.0), ([x0 + L / 2, yb + H - 0.013, 0.0], W / 2 - 0.004, L / 2 - 0.004, 6.0), ([x0 + L / 2, yb + H - 0.001, 0.0], W / 2 - 0.006, L / 2 - 0.006, 6.0)]
        S.put(S.tube(f"crate-{name}", "crate", rows, PLASTIC, CRATE + 0.07 * k, hint=[0.0, 0.0, 1.0]), reg)
        S.struct_of(f"crate-{name}", "crate")
        side = [[x0 + 0.018, yb + 0.002], [x0 + L - 0.018, yb + 0.002], [x0 + L - 0.006, yb + H - 0.045], [x0 + L, yb + H - 0.044], [x0 + L, yb + H - 0.014], [x0 + L - 0.006, yb + H - 0.001], [x0 + 0.006, yb + H - 0.001], [x0, yb + H - 0.014], [x0, yb + H - 0.044], [x0 + 0.006, yb + H - 0.045]]
        depth = [x0 + L, yb + H, -W / 2]
        S.flat("elevation", f"crate-{name}", "crate", "body", CRATE + 0.07 * k, reg, PLASTIC, [kit.loc("crate", q)[:2] for q in side], depth, pen=0.18, tol=0.0)
        front = [[-(W / 2 - 0.018), yb + 0.002], [W / 2 - 0.018, yb + 0.002], [W / 2 - 0.006, yb + H - 0.045], [W / 2, yb + H - 0.044], [W / 2, yb + H - 0.014], [W / 2 - 0.006, yb + H - 0.001], [-(W / 2 - 0.006), yb + H - 0.001], [-W / 2, yb + H - 0.014], [-W / 2, yb + H - 0.044], [-(W / 2 - 0.006), yb + H - 0.045]]
        S.flat("front", f"crate-{name}", "crate", "body", CRATE + 0.07 * k, reg, PLASTIC, front, depth, pen=0.18, tol=0.0)
        hx = x0 + L / 2
        S.flat("elevation", f"crate-{name}-handle", "crate", "texture", 0.3, None, PLASTIC, [kit.loc("crate", q)[:2] for q in ([hx - 0.06, yb + H - 0.07], [hx + 0.06, yb + H - 0.07], [hx + 0.06, yb + H - 0.05], [hx - 0.06, yb + H - 0.05])], depth, pen=0.1, filled=False)
        S.flat("elevation", f"crate-{name}-lid", "crate", "texture", 0.3, None, PLASTIC, [kit.loc("crate", q)[:2] for q in ([x0, yb + H - 0.014], [x0 + L, yb + H - 0.014])], depth, pen=0.1, filled=False, closed=False)
    plan = rounded([[x0, -W / 2], [x0 + L, -W / 2], [x0 + L, W / 2], [x0, W / 2]], 0.012, 3)
    S.flat("plan", "crate-upper", "crate", "body", CRATE + 0.07, reg, PLASTIC, plan, [x0 + L, y0 + 2 * H, -W / 2], pen=0.18, tol=0.0)
    for k, zz in enumerate((-0.15, 0.15)):
        S.flat("plan", f"crate-rib-{k + 1}", "crate", "texture", 0.35, None, PLASTIC, [[x0 + 0.03, zz], [x0 + L - 0.03, zz]], [x0 + L, y0 + 2 * H, -W / 2], pen=0.1, filled=False, closed=False)
    S.struct(S.box("crate-post", "crate-y", [0.02, 2 * H - 0.02, 0.02], [x0 + 0.02, y0 + H, W / 2 - 0.02], PLASTIC, 0.4), "crate-y")
    S.struct(S.box("crate-base", "crate-x", [L - 0.04, 0.012, W - 0.04], [x0 + L / 2, y0 + 0.008, 0.0], PLASTIC, 0.4), "crate-x")
    S.struct(S.ball("mass-crates", "crate", [0.04, 0.04, 0.04], [x0 + L / 2, y0 + H, 0.0], PLASTIC, 0.5), "crate", "mass")


def stand(S):
    reg = "frame"
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    S.put(S.cyl("stand-pivot", "stand", at(0, -0.046), at(0, 0.046), 0.011, AL, TUBE, role="detail"), reg)
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.04), at(0, 0.04), 0.007, AL, 0.5), "stand", "joint")
    S.disc("stand-pivot", "stand", P, 0.022, -0.046, 0.046, "detail", TUBE, reg, AL, pen=THIN)
    L = kit.STAND_LENGTH
    for side, name in ((1, "l"), (-1, "r")):
        path = kit.bent([at(0.0, side * 0.038), at(0.045, side * 0.042), at(L - 0.012, side * 0.205)], [0.03], 4)
        S.put(S.tube(f"stand-leg-{name}", "stand", [(p, 0.0095, 0.0095) for p in path], AL, TUBE, hint=[-u[1], u[0], 0.0]), reg)
        for k, (a, b) in enumerate(((path[0], path[3]), (path[3], path[-1]))):
            S.struct(S.pipe(f"stand-leg-{name}-{k + 1}", "stand", a, b, 0.0095, 0.002, AL, 0.4), "stand")
            S.member(f"stand-leg-{name}-{k + 1}", "stand", a, b, 0.019, "body", TUBE, reg, AL, pen=THIN)
        a, b = at(L - 0.012, side * 0.18), at(L - 0.012, side * 0.232)
        S.put(S.cap(f"stand-foot-{name}", "stand", a, b, 0.012, RUBBER, DARK, role="detail"), reg)
        S.plank(f"stand-foot-{name}", "stand", at(L - 0.024, 0), at(L, 0), 0.052, side * 0.206, "detail", DARK, reg, RUBBER, pen=THIN)
    a, b = at(0.15, -0.14), at(0.15, 0.14)
    S.put(S.cyl("stand-cross", "stand", a, b, 0.008, AL, TUBE, role="detail"), reg)
    S.struct(S.pipe("stand-cross", "stand", a, b, 0.008, 0.002, AL, 0.4), "stand")
    S.disc("stand-cross", "stand", at(0.15, 0)[:2], 0.016, -0.14, 0.14, "detail", TUBE, reg, AL, pen=THIN, views=("front", "plan"))


def build():
    S = kit.Scale(50, PENS)
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
        "en": "1:50 is the complete two-wheeled long-john cargo bike (Bakfiets.nl Cargobike Long class, 2.00 m wheelbase), every assembly stated by hand: aluminium frame with the 50 x 70 mm main beam that runs under the box and bends up around the front wheel as a gooseneck into the front head tube, the long rear steering column with headset cups and its strut, the oval down tube carrying the battery, seat tube with clamp and post, chain and seat stays to the dropouts, motor mount plates, three box rails and the stand bracket; 26 inch rear and 20 inch front wheels with 50 mm tyres, double-wall rims, 36 crossed spokes, gear hub and front hub with 180 mm brake rotors, axles and nuts; mid-drive motor with covers, frame battery with lock, crank arms, chainring with guard, chain and block pedals; sprung comfort saddle on rails, rear carrier with legs and struts and the tail lamp; 65 mm plastic mudguards with stays; front fork with steerer, crown, straight blades, ends, caliper, cable drum and head lamp on its bracket; handlebar column with riser, stem, swept-back bar, grips, levers, display and bell, the cable drum at its foot and the two housed steering cables along the beam; the plywood box (floor, sides, rear wall, raked front wall, capping, child bench), two stacked Euro containers 600 x 400 x 270 mm with lids, and the folded two-leg stand under the box.",
        "de": "1:50 ist das vollständige zweirädrige Long-John-Lastenrad (Klasse Bakfiets.nl Cargobike Long, 2,00 m Radstand), jede Baugruppe von Hand angegeben: Aluminiumrahmen mit dem 50 x 70 mm Hauptträger, der unter der Box läuft und sich als Schwanenhals um das Vorderrad in das vordere Steuerrohr hochbiegt, die lange hintere Lenksäule mit Steuersatzschalen und ihrer Strebe, das ovale Unterrohr mit dem Akku, Sattelrohr mit Klemme und Stütze, Ketten- und Sitzstreben zu den Ausfallenden, Motorhalterplatten, drei Boxschienen und die Ständerkonsole; 26-Zoll-Hinterrad und 20-Zoll-Vorderrad mit 50-mm-Reifen, Hohlkammerfelgen, 36 gekreuzten Speichen, Getriebenabe und Vorderradnabe mit 180-mm-Bremsscheiben, Achsen und Muttern; Mittelmotor mit Deckeln, Rahmenakku mit Schloss, Kurbelarme, Kettenblatt mit Schutzring, Kette und Blockpedale; gefederter Komfortsattel auf Streben, Gepäckträger mit Beinen und Streben und dem Rücklicht; 65 mm breite Kunststoffschutzbleche mit Streben; Vorderradgabel mit Schaft, Krone, geraden Scheiden, Enden, Bremssattel, Seiltrommel und Scheinwerfer an seinem Halter; Lenksäule mit Steigrohr, Vorbau, zurückgeschwungenem Lenker, Griffen, Hebeln, Display und Klingel, der Seiltrommel an ihrem Fuß und den zwei ummantelten Lenkseilen entlang des Trägers; die Sperrholzbox (Boden, Seiten, Rückwand, schräge Stirnwand, Kantenleisten, Kinderbank), zwei gestapelte Eurobehälter 600 x 400 x 270 mm mit Deckeln und der hochgeklappte Zweibeinständer unter der Box.",
    },
    {
        "en": "Thermal: every region lies on the body surface itself and references its body solids (frame tubes, fork, column, carrier, stand and rails; box panels and bench; crates; mudguards; saddle and grips; battery case; motor housing with covers; tyres; brake rotors). Each region has at most one layer, so there are no nested shells; there are no hidden regions and no emitters. Rims, spokes, hubs, chain, cranks, pedals, lamps, calipers and the display belong to no thermal region of the asset.",
        "de": "Thermik: jede Region liegt auf der Körperoberfläche selbst und verweist auf ihre Körper (Rahmenrohre, Gabel, Lenksäule, Gepäckträger, Ständer und Schienen; Boxplatten und Bank; Kisten; Schutzbleche; Sattel und Griffe; Akkugehäuse; Motorgehäuse mit Deckeln; Reifen; Bremsscheiben). Jede Region hat höchstens eine Schicht, daher gibt es keine verschachtelten Schalen; es gibt keine verdeckten Regionen und keine Emitter. Felgen, Speichen, Naben, Kette, Kurbeln, Pedale, Leuchten, Bremssättel und das Display gehören zu keiner thermischen Region des Assets.",
    },
    {
        "en": "Structure: the real load path. The main beam as a 50 x 70 x 2.5 mm box section and the gooseneck as round segments, both head tubes, strut, down tube, seat tube and post as hollow sections, stays, dropouts, motor mount plates, box rails and carrier tubes; the hollow steerers of fork and handlebar column with their headset bearings, fork crown and blades, riser, stem and the handlebar as five tubes per side; the steering cables; rims, all 36 spokes and the hub per wheel with the hub bearing; crank axle with its bearing, arms and pedal spindles; motor housing and battery case; box floor, walls and bench; the crate stack with its base and corner post; stand legs, cross tube and hinge; the rear tyre patch as support; masses of frame, fork, column, box, motor, battery, wheels and crates at their centres of mass.",
        "de": "Tragwerk: der wirkliche Lastpfad. Der Hauptträger als Kastenprofil 50 x 70 x 2,5 mm und der Schwanenhals als runde Abschnitte, beide Steuerrohre, Strebe, Unterrohr, Sattelrohr und Stütze als Hohlprofile, Streben, Ausfallenden, Motorhalterplatten, Boxschienen und Gepäckträgerrohre; die hohlen Gabelschäfte von Gabel und Lenksäule mit ihren Steuersatzlagern, Gabelkrone und Scheiden, Steigrohr, Vorbau und der Lenker als fünf Rohre je Seite; die Lenkseile; Felgen, alle 36 Speichen und die Nabe je Rad mit dem Nabenlager; Kurbelachse mit ihrem Lager, Arme und Pedalachsen; Motorgehäuse und Akkugehäuse; Boxboden, Wände und Bank; der Kistenstapel mit Boden und Eckpfosten; Ständerbeine, Querrohr und Gelenk; die hintere Reifenaufstandsfläche als Auflager; Massen von Rahmen, Gabel, Lenksäule, Box, Motor, Akku, Rädern und Kisten in ihren Schwerpunkten.",
    },
]

DRAWN = {
    "silhouette": {
        "en": "All three views are the true orthographic views of the 1:50 body in the parked pose. Side: tyre and rim rings with 18 of the 36 spokes, hubs with their brake rotors, the main beam with its gooseneck, both head tubes with their headset cups, strut, down tube with the battery, seat tube with clamp and post, stays and dropouts, motor with mount plates, chainring with guard, chain, cranks and block pedals, sprung saddle, rear carrier with tail lamp, mudguards with stays, the fork with crown, blades, cable drum, caliper and head lamp, the handlebar column with riser, stem, swept-back bar, grips, levers, display and bell, the steering cable, the plywood box with raked front wall, capping and bench, the two crates and the stand under the box. Front and top show the same parts from ahead and from above.",
        "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers in der Parkstellung. Seite: Reifen- und Felgenringe mit 18 der 36 Speichen, Naben mit ihren Bremsscheiben, der Hauptträger mit seinem Schwanenhals, beide Steuerrohre mit Steuersatzschalen, Strebe, Unterrohr mit dem Akku, Sattelrohr mit Klemme und Stütze, Streben und Ausfallenden, Motor mit Halterplatten, Kettenblatt mit Schutzring, Kette, Kurbeln und Blockpedale, gefederter Sattel, Gepäckträger mit Rücklicht, Schutzbleche mit Streben, die Gabel mit Krone, Scheiden, Seiltrommel, Bremssattel und Scheinwerfer, die Lenksäule mit Steigrohr, Vorbau, zurückgeschwungenem Lenker, Griffen, Hebeln, Display und Klingel, das Lenkseil, die Sperrholzbox mit schräger Stirnwand, Kantenleisten und Bank, die zwei Kisten und der Ständer unter der Box. Vorder- und Draufsicht zeigen dieselben Teile von vorn und von oben.",
    },
    "retained": {"en": "Every assembly of the body that is at least 7.5 mm thick, the thin sheet parts (rotors, chainring, motor plates) at their true thickness, and the wires (spokes, mudguard stays, saddle rails, steering cables) as strokes.", "de": "Jede Baugruppe des Körpers ab 7,5 mm Dicke, die dünnen Blechteile (Bremsscheiben, Kettenblatt, Motorplatten) in ihrer wirklichen Dicke und die Drähte (Speichen, Schutzblechstreben, Sattelgestell, Lenkseile) als Striche."},
    "merged": {"en": "Each handlebar half is four straight bars along its bend, each stand leg two; hub shell and flanges are one body.", "de": "Jede Lenkerhälfte besteht aus vier geraden Stäben entlang ihrer Biegung, jedes Ständerbein aus zwei; Nabenkörper und Flansche sind ein Körper."},
    "omitted": {"en": "Brake and gear cables, cable housings at the drums, tyre tread, valves, reflectors, the second half of the spokes, the cross bars of the carrier in side and front view, the left steering cable in the side view.", "de": "Brems- und Schaltzüge, Seilhüllen an den Trommeln, Reifenprofil, Ventile, Reflektoren, die zweite Hälfte der Speichen, die Querstäbe des Gepäckträgers in Seiten- und Vorderansicht, das linke Lenkseil in der Seitenansicht."},
    "thinElements": {"en": "Tubes are filled bars at their real width (beam 1.4 mm, head tubes 0.9 mm, stays 0.34 to 0.4 mm on paper) with a 0.13 to 0.18 mm outline; carrier tubes 0.24 mm; spokes, stays, rails and cables are 0.1 to 0.13 mm strokes; rotors, chainring and mudguards are drawn at their real thickness with a 0.18 mm outline.", "de": "Rohre sind gefüllte Stäbe in ihrer wirklichen Breite (Träger 1,4 mm, Steuerrohre 0,9 mm, Streben 0,34 bis 0,4 mm auf dem Papier) mit 0,13 bis 0,18 mm Kontur; Gepäckträgerrohre 0,24 mm; Speichen, Streben, Gestell und Seile sind Striche von 0,1 bis 0,13 mm; Bremsscheiben, Kettenblatt und Schutzbleche sind in ihrer wirklichen Dicke mit 0,18 mm Kontur gezeichnet."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Near black tyres, grips and pedals, light rims, mid-tone aluminium frame, light plywood box with darker capping, mid-grey crates, dark battery and motor.", "de": "Fast schwarze Reifen, Griffe und Pedale, helle Felgen, mitteltoniger Aluminiumrahmen, helle Sperrholzbox mit dunkleren Kantenleisten, mittelgraue Kisten, dunkler Akku und Motor."},
    "overlays": {"en": "Every shape carries the thermal region of its part where it has one. Straight tubes are members (bars between two points of their bone), plates, bent tubes and round parts are profiles extruded across the bike, so every part is one definition for side, front and top and follows steering (a twist of the column and of the fork about their own head tube axes), rolling wheels, the turning crank, the folding stand and the moving crate stack. Only the saddle and the crates have a separate outline per view.", "de": "Jede Form trägt die thermische Region ihres Teils, wo es eine gibt. Gerade Rohre sind Stäbe zwischen zwei Punkten ihres Knochens, Platten, gebogene Rohre und runde Teile quer zum Rad extrudierte Profile; so ist jedes Teil eine Definition für Seite, Vorderansicht und Draufsicht und folgt dem Lenken (einer Drehung von Lenksäule und Gabel um ihre eigene Steuerrohrachse), den rollenden Rädern, der drehenden Kurbel, dem klappbaren Ständer und dem bewegten Kistenstapel. Nur der Sattel und die Kisten haben je Ansicht eine eigene Kontur."},
}
