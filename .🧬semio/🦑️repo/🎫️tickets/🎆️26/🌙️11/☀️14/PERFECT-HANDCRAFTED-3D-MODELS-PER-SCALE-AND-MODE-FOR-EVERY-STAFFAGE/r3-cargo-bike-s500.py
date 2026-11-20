"""🛺 Round 3 vehicles/cargo-bike at 1:500 (smallest feature 75 mm): the long-john as the mass model this scale can print, authored anew. Wheels as 80 mm rings with hub drums, every tube generalised to a 75 mm bar on its real centre line (beam with gooseneck, both head tubes, down tube, seat tube to the saddle, one chain- and one seat stay per side, fork blades, column and handlebar), motor, battery, saddle pad, the carrier as a deck, crank bars with pedal blocks, the box as one block with the upper crate standing out of it, and the two stand legs."""
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
PENS = {"body": 0.15, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25}
PEN = 0.15
G = 0.0375
bw = fine.bw


def bar(S, i, bone, a, b, region, tone=TUBE, views=ALL, depth=None, r=G, mat=AL):
    S.put(S.cap(i, bone, a, b, r, mat, tone), region)
    S.struct(S.cap(i, bone, a, b, r, mat, 0.4), bone)
    S.member(i, bone, a, b, 2 * r, "body", tone, region, mat, pen=PEN, views=views, depth=depth)


def wheel(S, bone):
    w = pt.WHEELS[bone]
    c, tag = w["c"], bone[-1]
    ro, ri = w["out"], w["out"] - 0.08
    S.put(S.rev(f"wheel-{tag}", bone, [(-G, ri), (G, ri), (G, ro - 0.02), (0.02, ro), (-0.02, ro), (-G, ro - 0.02), (-G, ri)], c, [0, 0, 1], RUBBER, DARK), "tyres")
    S.struct(S.rev(f"rim-{tag}", bone, [(-G, ri), (G, ri), (G, ri + 0.04), (-G, ri + 0.04), (-G, ri)], c, [0, 0, 1], RIM, 0.4), bone)
    outer, inner = pt.ring(c, ro, ri, 20)
    S.swept(f"wheel-{tag}", bone, outer, -G, G, "body", DARK, "tyres", RUBBER, holes=[inner], tol=0.003, views=SIDE)
    for k, h in enumerate(pt.half_rings(c, ro, ri, 10)):
        S.swept(f"wheel-{tag}-{k + 1}", bone, h, -G, G, "body", DARK, "tyres", RUBBER, tol=0.003, views=("front", "plan"))
    S.put(S.cyl(f"hub-{tag}", bone, [c[0], c[1], -0.04], [c[0], c[1], 0.04], 0.045, RIM, METAL))
    S.therm(S.body[-1], "brakes")
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], -0.04], [c[0], c[1], 0.04], 0.038, RIM, 0.5), bone, "joint")
    S.disc(f"hub-{tag}", bone, c, 0.09, -0.04, 0.04, "body", METAL, "brakes", RIM, pen=PEN, views=SIDE)
    for j in range(3):
        q = math.radians(120 * j + 90)
        S.struct(S.cyl(f"spokes-{tag}-{j + 1}", bone, kit.add(c, [0.04 * math.cos(q), 0.04 * math.sin(q), 0]), kit.add(c, [ri * math.cos(q), ri * math.sin(q), 0]), G, RIM, 0.4), bone)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.04, 0.04, 0.04], c, RIM, 0.5), bone, "mass")


def frame(S):
    reg = "frame"
    beam = pt.beam_path(2)
    S.put(S.tube("beam", "frame", [(p, G, G, 4.0) for p in beam], AL, TUBE), reg)
    S.swept("beam", "frame", kit.ribbon([q[:2] for q in beam], G), -G, G, "body", TUBE, reg, AL, pen=PEN, tol=0.003)
    knots = [beam[0], beam[1], beam[-4], beam[-1]]
    for k in range(len(knots) - 1):
        S.struct(S.cyl(f"beam-{k + 1}", "frame", knots[k], knots[k + 1], G, AL, 0.4), "frame")
    bar(S, "head-tube-f", "frame", fp(pt.HEAD_F[0] - 0.004), fp(0.5), reg)
    bar(S, "head-tube-r", "frame", bp(pt.DRUM_R[0]), bp(pt.HEAD_R[1] + 0.004), reg)
    bar(S, "down-tube", "frame", list(kit.DOWN_LO) + [0.0], kit.DOWN_HI, reg)
    bar(S, "seat-tube", "frame", kit.lerp(B, pt.SEAT_TOP, 0.05), pt.POST_TOP, reg)
    for side, name in ((1, "l"), (-1, "r")):
        z = side * 0.0755
        bar(S, f"chainstay-{name}", "frame", [B[0] - 0.06, B[1] - 0.01, side * 0.045], [R[0], R[1], z], reg, views=ALL if side < 0 else ("front", "plan"), depth=[0.45, 0.31, z])
        bar(S, f"seatstay-{name}", "frame", kit.add(pt.SEAT_TOP, [0.0, -0.03, side * 0.045]), [R[0], R[1], z], reg, views=ALL if side < 0 else ("front", "plan"))
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.05, 0.05, 0.05], [fx, fy, 0], AL, 0.5), "frame", "mass")
    S.struct(S.box("contact-pad-r", "frame", [0.08, 0.075, 0.08], [0.0, 0.0375, 0.0], RUBBER, 0.1), "frame", "support", anchor="contact-r")


def carrier(S):
    reg = "frame"
    x0, x1 = pt.CARRIER_X
    y = pt.CARRIER_Y
    c = [(x0 + x1) / 2, y, 0.0]
    S.put(S.box("carrier", "frame", [x1 - x0, 0.075, 0.15], c, AL, TUBE), reg)
    S.struct(S.box("carrier-rails", "frame", [x1 - x0, 0.075, 0.15], c, AL, 0.4), "frame")
    S.swept("carrier", "frame", [[x0, y - G], [x1, y - G], [x1, y + G], [x0, y + G]], -0.075, 0.075, "body", TUBE, reg, AL, pen=PEN, tol=0.0)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = [-0.14, y, side * 0.075], [R[0] - 0.004, R[1] + 0.016, side * 0.081]
        S.put(S.cap(f"carrier-leg-{name}", "frame", a, b, 0.006, AL, TUBE, role="detail", slender=True), reg)
        S.struct_of(f"carrier-leg-{name}", "frame")
        if side < 0:
            S.wire(f"carrier-leg-{name}", a, b, "detail", TUBE, reg, AL, 0.13, views=SIDE)


def saddle(S):
    rows = [([0.338, 0.985, 0.0], 0.04, G, 2.6), ([0.205, 0.982, 0.0], 0.085, G, 2.6), ([0.075, 0.985, 0.0], 0.09, G, 2.6)]
    S.put(S.tube("saddle", "frame", rows, PLASTIC, 0.15), "touch-parts")
    S.flat("elevation", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[0.338, 0.985 - G], [0.338, 0.985 + G], [0.075, 0.985 + G], [0.075, 0.985 - G]], [0.34, 1.0, -0.107])
    S.flat("plan", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[0.338, -0.04], [0.205, -0.085], [0.075, -0.09], [0.075, 0.09], [0.205, 0.085], [0.338, 0.04]], [0.34, 1.0, -0.107])
    S.flat("front", "saddle", "frame", "body", 0.15, "touch-parts", PLASTIC, [[-0.09, 0.985 - G], [0.09, 0.985 - G], [0.09, 0.985 + G], [-0.09, 0.985 + G]], [0.34, 1.0, -0.107])


def guards(S):
    reg = "fenders"
    for i, bone, c, a0, a1, r in (("fender-r", "frame", R, 18.0, 196.0, pt.WHEELS["wheel-r"]["out"] + 0.004), ("fender-f", "fork", F, 18.0, 150.0, pt.WHEELS["wheel-f"]["out"] + 0.004)):
        deg = kit.BONES[bone][1]
        o = kit.loc(bone, c)
        s = S.solid(i, "revolve", bone, PLASTIC, 0.22, profile=[[-G, r], [G, r], [G, r + 0.03], [-G, r + 0.03], [-G, r]], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec(fine.turn_offset(o, a0 - deg))}
        S.put(s, reg)
        band = kit.arc(c, r + 0.03, a0, a1, 12) + kit.arc(c, r, a1, a0, 12)
        S.swept(i, bone, band, -G, G, "body", 0.22, reg, PLASTIC, pen=PEN, tol=0.003)


def motor(S):
    out = [[B[0] + x, B[1] + y] for x, y in ((0.12, -0.03), (0.08, 0.075), (-0.045, 0.075), (-0.08, 0.0), (-0.04, -0.09), (0.09, -0.09))]
    S.put(S.prism("motor-housing", "motor", out, -0.045, 0.045, MOTOR, 0.3), "drive-motor")
    S.struct_of("motor-housing", "motor")
    S.swept("motor-housing", "motor", out, -0.045, 0.045, "body", 0.3, "drive-motor", MOTOR, pen=PEN, tol=0.0)
    S.struct(S.ball("mass-motor", "motor", [0.04, 0.04, 0.04], [B[0] + 0.02, B[1] - 0.01, 0.0], MOTOR, 0.5), "motor", "mass")


def battery(S):
    c = bw("battery", 0.15, 0.0665)
    S.put(S.box("battery-case", "battery", [0.36, 0.085, 0.092], c, BATT, 0.18), "battery-pack")
    S.struct_of("battery-case", "battery")
    q = [bw("battery", u, v)[:2] for u, v in ((-0.03, 0.024), (0.33, 0.024), (0.33, 0.109), (-0.03, 0.109))]
    S.swept("battery-case", "battery", q, -0.046, 0.046, "body", 0.18, "battery-pack", BATT, pen=PEN, tol=0.0)
    S.struct(S.ball("mass-battery", "battery", [0.04, 0.04, 0.04], c, BATT, 0.5), "battery", "mass")


def drive(S):
    S.struct(S.cyl("bb-bearing", "crank", [B[0], B[1], -0.05], [B[0], B[1], 0.05], 0.038, RIM, 0.5), "crank", "joint")
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        zc = side * 0.085
        a, b = [B[0], B[1], zc], [B[0] + sign * 0.19, B[1], zc]
        S.put(S.box(f"crank-arm-{name}", "crank", [0.19, 0.075, 0.075], kit.lerp(a, b, 0.5), AL, 0.3))
        S.struct_of(f"crank-arm-{name}", "crank")
        S.swept(f"crank-arm-{name}", "crank", [[min(a[0], b[0]), B[1] - G], [max(a[0], b[0]), B[1] - G], [max(a[0], b[0]), B[1] + G], [min(a[0], b[0]), B[1] + G]], zc - G, zc + G, "body", 0.3, None, AL, pen=PEN)


def fork(S):
    reg = "frame"
    S.struct(S.cap("steerer-f", "fork", fp(pt.CROWN_F[1]), fp(0.5), G, AL, 0.4), "fork")
    S.put(S.box("fork-crown", "fork", [0.075, 0.075, 0.226], fp(pt.CROWN_F[0] + 0.005, 0.004), AL, TUBE), reg)
    S.struct_of("fork-crown", "fork")
    S.plank("fork-crown", "fork", fp(pt.CROWN_F[0] - 0.0325, 0.004), fp(pt.CROWN_F[0] + 0.0425, 0.004), 0.075, 0.0, "body", TUBE, reg, AL, pen=PEN, wide=0.226)
    for side, name in ((1, "l"), (-1, "r")):
        a, b = fp(pt.CROWN_F[0], 0.002, side * 0.0755), fp(0.0, kit.OFFSET_F, side * 0.0755)
        bar(S, f"fork-blade-{name}", "fork", a, b, reg, views=ALL if side < 0 else ("front", "plan"))
    S.struct(S.ball("mass-fork", "fork", [0.04, 0.04, 0.04], fp(0.2, 0.02), AL, 0.5), "fork", "mass")


def cockpit(S):
    reg = "frame"
    S.struct(S.cap("headset-r", "bar", bp(pt.HEAD_R[0]), bp(pt.HEAD_R[1]), 0.04, AL, 0.5), "bar", "joint")
    bar(S, "riser", "bar", bp(pt.HEAD_R[1] + 0.004), bp(pt.RISER_TOP - 0.02), reg)
    end = [pt.BAR_END[0], pt.BAR_END[1], 0.28]
    hb = [pt.CLAMP, [pt.BAR_CORNER[0], pt.BAR_CORNER[1], 0.16], end]
    for side, name in ((1, "l"), (-1, "r")):
        path = [pt.mirror(p, side) for p in hb]
        S.put(S.tube(f"handlebar-{name}", "bar", [(p, G, G) for p in kit.bent(path, [0.06], 2)], AL, METAL, hint=[0.0, 1.0, 0.0]), reg)
        for k in range(2):
            S.struct(S.cap(f"handlebar-{name}-{k + 1}", "bar", path[k], path[k + 1], G, AL, 0.4), "bar")
            S.member(f"handlebar-{name}-{k + 1}", "bar", path[k], path[k + 1], 2 * G, "body", METAL, reg, AL, pen=PEN)
    S.struct(S.ball("mass-bar", "bar", [0.04, 0.04, 0.04], bp(0.72, 0.0), AL, 0.5), "bar", "mass")
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.cable_path(side)[::6] + [pt.cable_path(side)[-1]]
        S.struct(S.tube(f"steer-cable-{name}", "frame", [(p, 0.002, 0.002) for p in path], AL, 0.4, slender=True), "frame")


def box(S):
    top = pt.BOX_YT + pt.CAP[1]
    side = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_XT + 0.004, top], [pt.BOX_X0, top]]
    S.put(S.prism("box", "box", side, -pt.BOX_HW, pt.BOX_HW, PLY, WOOD), "box-wood")
    S.swept("box", "box", side, -pt.BOX_HW, pt.BOX_HW, "body", WOOD, "box-wood", PLY, pen=PEN, tol=0.0)
    t = 2 * G
    floor = [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_X1 + 0.01, pt.BOX_Y0 + t], [pt.BOX_X0, pt.BOX_Y0 + t]]
    S.struct(S.prism("box-floor", "box", floor, -pt.BOX_HW + 0.002, pt.BOX_HW - 0.002, PLY, 0.4), "box")
    for s, name in ((1, "l"), (-1, "r")):
        z0, z1 = sorted((s * (pt.BOX_HW - 0.002), s * (pt.BOX_HW - 0.002 - t)))
        S.struct(S.prism(f"box-side-{name}", "box", [[pt.BOX_X0, pt.BOX_Y0], [pt.BOX_X1, pt.BOX_Y0], [pt.BOX_XT, pt.BOX_YT], [pt.BOX_X0, pt.BOX_YT]], z0, z1, PLY, 0.4), "box")
    S.struct(S.ball("mass-box", "box", [0.04, 0.04, 0.04], [1.24, 0.42, 0.0], PLY, 0.5), "box", "mass")


def crates(S):
    L, W, H = pt.CRATE
    x0, y0 = kit.CRATE_O
    yb = pt.BOX_YT + pt.CAP[1]
    yt = y0 + 2 * H
    c = [x0 + L / 2, (yb + yt) / 2, 0.0]
    S.put(S.box("crates", "crate", [L, yt - yb, W], c, PLASTIC, CRATE), "cargo")
    S.struct(S.box("crate-stack", "crate", [L - 0.01, 2 * H - 0.01, W - 0.01], [x0 + L / 2, y0 + H, 0.0], PLASTIC, 0.4), "crate")
    depth = [x0 + L, yt, -W / 2]
    S.flat("elevation", "crates", "crate", "body", CRATE, "cargo", PLASTIC, [kit.loc("crate", q)[:2] for q in ([x0, yb], [x0 + L, yb], [x0 + L, yt], [x0, yt])], depth, pen=PEN, tol=0.0)
    S.flat("front", "crates", "crate", "body", CRATE, "cargo", PLASTIC, [[-W / 2, yb], [W / 2, yb], [W / 2, yt], [-W / 2, yt]], depth, pen=PEN, tol=0.0)
    S.flat("plan", "crates", "crate", "body", CRATE, "cargo", PLASTIC, [[x0, -W / 2], [x0 + L, -W / 2], [x0 + L, W / 2], [x0, W / 2]], depth, pen=PEN, tol=0.0)
    S.struct(S.box("crate-post", "crate-y", [0.075, 2 * H - 0.08, 0.075], [x0 + 0.04, y0 + H, W / 2 - 0.04], PLASTIC, 0.4), "crate-y")
    S.struct(S.box("crate-base", "crate-x", [L - 0.02, 0.075, W - 0.02], [x0 + L / 2, y0 + 0.04, 0.0], PLASTIC, 0.4), "crate-x")
    S.struct(S.ball("mass-crates", "crate", [0.04, 0.04, 0.04], [x0 + L / 2, y0 + H, 0.0], PLASTIC, 0.5), "crate", "mass")


def stand(S):
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.05), at(0, 0.05), 0.038, AL, 0.5), "stand", "joint")
    for side, name in ((1, "l"), (-1, "r")):
        bar(S, f"stand-leg-{name}", "stand", at(0.0, side * 0.04), at(kit.STAND_LENGTH - G, side * 0.19), "frame")


def build():
    S = kit.Scale(500, PENS)
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
        "en": "1:500 is the mass model at the 75 mm floor of this scale: each wheel an 80 mm ring with a hub drum; every tube a 75 mm bar on its real centre line (beam with a two-step gooseneck, both head tubes, down tube, seat tube up to the saddle, one chain stay and one seat stay per side, two fork blades under a crown block, column riser and the swept handlebar narrowed to 0.64 m overall); the motor, the battery, the saddle pad, the carrier as one deck on two strokes, the crank bars; the box as one block (walls, floor, bench and the lower crate inside it) with the upper crate standing out of it; the stand as two legs. The mudguards are 30 mm bands over the tyres; pedals, chain, cables, lamps and all small parts are omitted.",
        "de": "1:500 ist das Massenmodell an der 75-mm-Grenze dieses Maßstabs: jedes Rad ein 80-mm-Ring mit Nabentrommel; jedes Rohr ein 75-mm-Stab auf seiner wirklichen Mittellinie (Träger mit zweistufigem Schwanenhals, beide Steuerrohre, Unterrohr, Sattelrohr bis zum Sattel, je Seite eine Ketten- und eine Sitzstrebe, zwei Gabelscheiden unter einem Kronenblock, Steigrohr und der geschwungene Lenker, auf 0,64 m Gesamtbreite verengt); der Motor, der Akku, das Sattelpolster, der Gepäckträger als ein Deck auf zwei Strichen, die Kurbelstäbe; die Box als ein Block (Wände, Boden, Bank und die untere Kiste darin) mit der oberen Kiste, die aus ihr herausragt; der Ständer als zwei Beine. Die Schutzbleche sind 30-mm-Bänder über den Reifen; Pedale, Kette, Seile, Leuchten und alle kleinen Teile entfallen.",
    },
    {
        "en": "Thermal: the regions reference their body solids (tyres on the wheel rings, brakes on the hub drums, frame, box-wood on the box block, cargo on the upper crate, touch-parts on the saddle, battery-pack, drive-motor); the mudguards are their 30 mm bands (region fenders). Only the total exposed area is compared at this scale.",
        "de": "Thermik: die Regionen verweisen auf ihre Körper (Reifen auf den Radringen, Bremsen auf den Nabentrommeln, Rahmen, Boxholz auf dem Boxblock, Ladung auf der oberen Kiste, Berührteile auf dem Sattel, Akku, Motor); die Schutzbleche sind ihre 30-mm-Bänder (Region Schutzbleche). Bei diesem Maßstab wird nur die gesamte freie Fläche verglichen.",
    },
    {
        "en": "Structure: the load path at the 75 mm floor and its own: beam segments, head tubes, down tube, seat tube, stays and carrier deck as bars inside the body bars; fork steerer, crown and blades; column bearing, riser and handlebar; rims with three spoke bars and the hub bearings; crank bars with the bearing; motor and battery; box floor and side walls as plates inside the box block; the crate stack with base and corner post; stand legs and hinge; the steering cables; the rear tyre patch as support; masses at their centres.",
        "de": "Tragwerk: der Lastpfad an der 75-mm-Grenze als eigene Liste: Trägerabschnitte, Steuerrohre, Unterrohr, Sattelrohr, Streben und Gepäckträgerdeck als Stäbe innerhalb der Körperstäbe; Gabelschaft, Krone und Scheiden; Lenksäulenlager, Steigrohr und Lenker; Felgen mit drei Speichenstäben und die Nabenlager; Kurbelstäbe mit dem Lager; Motor und Akku; Boxboden und Seitenwände als Platten im Boxblock; der Kistenstapel mit Boden und Eckpfosten; Ständerbeine und Gelenk; die Lenkseile; die hintere Reifenaufstandsfläche als Auflager; Massen in ihren Schwerpunkten.",
    },
]

DRAWN = {
    "silhouette": {"en": "All three views are the true orthographic views of the 1:500 mass model in the parked pose: two wheel rings with hubs and mudguard bands, the frame bars with beam and gooseneck, motor, battery, saddle, carrier deck, fork, column and handlebar, the box block with the upper crate and the stand legs.", "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:500-Massenmodells in der Parkstellung: zwei Radringe mit Naben und Schutzblechbändern, die Rahmenstäbe mit Träger und Schwanenhals, Motor, Akku, Sattel, Gepäckträgerdeck, Gabel, Lenksäule und Lenker, der Boxblock mit der oberen Kiste und die Ständerbeine."},
    "retained": {"en": "The wheels, the frame lines, the box with the crates, saddle, handlebar and stand.", "de": "Die Räder, die Rahmenlinien, die Box mit den Kisten, Sattel, Lenker und Ständer."},
    "merged": {"en": "Tyre and rim into one ring, rotors into the hubs, all tubes to 75 mm bars, box walls, floor, bench and lower crate into one block, the carrier into one deck.", "de": "Reifen und Felge in einen Ring, Bremsscheiben in die Naben, alle Rohre zu 75-mm-Stäben, Boxwände, Boden, Bank und untere Kiste in einen Block, der Gepäckträger in ein Deck."},
    "omitted": {"en": "Spokes, pedals, chain, cables, lamps, display, levers, strut, motor bracket, stand feet.", "de": "Speichen, Pedale, Kette, Seile, Leuchten, Display, Hebel, Strebe, Motorkonsole, Ständerfüße."},
    "thinElements": {"en": "Every bar is 0.15 mm on paper, drawn filled with a 0.15 mm outline; the carrier legs are one 0.13 mm stroke.", "de": "Jeder Stab ist auf dem Papier 0,15 mm breit, gefüllt mit 0,15 mm Kontur; die Gepäckträgerbeine sind ein 0,13-mm-Strich."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Dark wheels, mid-tone frame, light box, grey crate.", "de": "Dunkle Räder, mitteltoniger Rahmen, helle Box, graue Kiste."},
    "overlays": {"en": "Shapes carry the thermal regions of their parts; members and extruded profiles follow steering, rolling, crank, stand and crate.", "de": "Die Formen tragen die thermischen Regionen ihrer Teile; Stäbe und extrudierte Profile folgen Lenken, Rollen, Kurbel, Ständer und Kiste."},
}
