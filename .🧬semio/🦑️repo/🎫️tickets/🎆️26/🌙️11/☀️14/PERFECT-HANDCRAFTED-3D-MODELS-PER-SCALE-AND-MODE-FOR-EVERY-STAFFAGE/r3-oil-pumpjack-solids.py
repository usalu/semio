"""🧱 Round 3 solids of `machinery/oil-pumpjack`, authored anew for each scale (body, thermal, structure, decisions) from the dimension table of `r3-oil-pumpjack-kit.py`; every section is stated here by hand."""

import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))


def _load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


K = _load("pjkit", "r3-oil-pumpjack-kit.py")
DR = _load("pjdraw", "r3-oil-pumpjack-draw.py")

STEEL, CONC, CAST, MOT = "painted-steel", "concrete", "cast-iron", "motor-steel"
REGION = {}


def tag(region, *items):
    for d in items:
        REGION[d["id"]] = region


def beam_frame_to_head(pts):
    return [K.L2("horsehead", K.W2("beam", p)) for p in pts]


def base_parts(B, scale):
    sx0, sx1 = K.SLAB[0], K.SLAB[1]
    tag("base", B.box("slab", "base", CONC, 0.82, ((sx0 + sx1) / 2, 0.15, 0), (sx1 - sx0, 0.3, 2 * K.SLAB[4])))
    sl = K.SKID_X1 - K.SKID_X0
    if scale in (50, 100):
        tf = K.IB_TF if scale == 50 else 0.015
        tw = K.IB_TW if scale == 50 else 0.015
        for side, sg in (("l", 1), ("r", -1)):
            sec = list(K.ibeam(0.0, 0.24, K.IB_BF / 2, tf, tw))
            sec = [(v, sg * K.IB_Z + w) for v, w in sec]
            tag("base", B.along("skid-beam-" + side, "skid", STEEL, 0.5, sec, 0.0, sl))
        if scale == 50:
            ch = [(0, 0), (0.075, 0), (0.075, 0.012), (0.012, 0.012), (0.012, 0.198), (0.075, 0.198), (0.075, 0.21), (0, 0.21)]
            for i, x in enumerate(K.CROSS_X):
                pts = [(x - 0.04 + a, 0.315 + b) for a, b in ch]
                tag("base", B.side("skid-cross-%d" % (i + 1), "skid", STEEL, 0.5, pts, -(K.IB_Z - K.IB_TW / 2), K.IB_Z - K.IB_TW / 2))
            tag("base", B.box("skid-deck", "skid", STEEL, 0.5, ((K.SKID_X0 + K.SKID_X1) / 2, 0.545, 0), (sl, 0.01, 2 * K.SKID_Z)))
        else:
            for i, x in enumerate(K.CROSS_X):
                tag("base", B.box("skid-cross-%d" % (i + 1), "skid", STEEL, 0.5, (x, 0.42, 0), (0.12, 0.21, 2 * (K.IB_Z - 0.0075))))
            tag("base", B.box("skid-deck", "skid", STEEL, 0.5, ((K.SKID_X0 + K.SKID_X1) / 2, 0.5425, 0), (sl, 0.015, 2 * K.SKID_Z)))
    elif scale == 200:
        for side, sg in (("l", 1), ("r", -1)):
            tag("base", B.box("skid-beam-" + side, "skid", STEEL, 0.5, ((K.SKID_X0 + K.SKID_X1) / 2, 0.41, sg * K.IB_Z), (sl, 0.22, K.IB_BF)))
        for i, x in enumerate(K.CROSS_X[::2]):
            tag("base", B.box("skid-cross-%d" % (i + 1), "skid", STEEL, 0.5, (x, 0.41, 0), (0.15, 0.22, 2 * K.IB_Z - K.IB_BF)))
        tag("base", B.box("skid-deck", "skid", STEEL, 0.5, ((K.SKID_X0 + K.SKID_X1) / 2, 0.535, 0), (sl, 0.03, 2 * K.SKID_Z)))
    else:
        for side, sg in (("l", 1), ("r", -1)):
            tag("base", B.box("skid-beam-" + side, "skid", STEEL, 0.5, ((K.SKID_X0 + K.SKID_X1) / 2, 0.3875, sg * K.IB_Z), (sl, 0.175, K.IB_BF)))
        tag("base", B.box("skid-deck", "skid", STEEL, 0.5, ((K.SKID_X0 + K.SKID_X1) / 2, 0.5125, 0), (sl, 0.075, 2 * K.SKID_Z)))


def wellhead(B, scale):
    o = (K.WELL_X, 0, 0)
    up = (0, 1, 0)
    W = "wellhead"
    if scale == 50:
        tag(W, B.rev("wellhead-base", "base", STEEL, 0.55, o, up, [[0, 0.17], [0.5, 0.17], [0.5, 0.2], [0.53, 0.2]]))
        tag(W, B.rev("wellhead-valve", "base", STEEL, 0.7, (K.WELL_X, 0.53, 0), up, [[0, 0.1], [0.47, 0.1]]))
        tag(W, B.rev("stuffing-box", "base", STEEL, 0.45, (K.WELL_X, 1.0, 0), up, [[0, 0.08], [0.22, 0.08], [0.28, 0.06]]))
        tag(W, B.cyl("flowline", "base", STEEL, 0.55, (K.WELL_X + 0.05, 0.68, 0), (-2.0, 0.68, 0), 0.06))
    elif scale == 100:
        tag(W, B.rev("wellhead-base", "base", STEEL, 0.55, o, up, [[0, 0.17], [0.5, 0.17], [0.5, 0.2], [0.53, 0.2]]))
        tag(W, B.rev("wellhead-top", "base", STEEL, 0.6, (K.WELL_X, 0.53, 0), up, [[0, 0.1], [0.47, 0.1], [0.75, 0.08]]))
    else:
        tag(W, B.rev("wellhead", "base", STEEL, 0.6, o, up, [[0, 0.2], [0.53, 0.2], [0.53, 0.1], [1.28, 0.1]]))
    w = 0.05 if scale < 500 else 0.1
    tag("steel", B.rev("polished-rod", "base", STEEL, 0.9, (K.WELL_X, 1.28, 0), up, [[0, w / 2], [K.PIVOT[1] + 0.025 - 1.28, w / 2]], slender=True))


def post(B, scale):
    ex = {50: 8, 100: 6, 200: 4, 500: 4}[scale]
    h = K.LEG / 2
    for id, sx, sz in (("leg-fl", -1, 1), ("leg-fr", -1, -1), ("leg-rl", 1, 1), ("leg-rr", 1, -1)):
        p0, p1 = K.leg_axis(sx, sz)
        d = [b - a for a, b in zip(p0, p1)]
        n = sum(c * c for c in d) ** 0.5
        t0 = -0.08 / n
        t1 = 1 + 0.03 / n
        a = [p + t0 * c for p, c in zip(p0, d)]
        b = [p + t1 * c for p, c in zip(p0, d)]
        tag("steel", B.loft(id, "post", STEEL, 0.62, [(a, h, h, ex), (b, h, h, ex)]))
    if scale < 500:
        f = (K.BRACE_Y - 0.55) / (K.TOP_Y - 0.55)
        xb = K.FOOT_X - (K.FOOT_X - K.TOP_X) * f
        zb = K.FOOT_Z - (K.FOOT_Z - K.TOP_Z) * f
        t = K.BRACE_T
        for side, sg in (("l", 1), ("r", -1)):
            tag("steel", B.box("brace-x-" + side, "post", STEEL, 0.55, (0, K.BRACE_Y, sg * zb), (2 * xb, t, t)))
        for side, sg in (("f", -1), ("r", 1)):
            tag("steel", B.box("brace-z-" + side, "post", STEEL, 0.55, (sg * xb, K.BRACE_Y, 0), (t, t, 2 * zb - t)))
    tag("steel", B.box("post-cap", "post", STEEL, 0.4, (0, (K.cap_y0(scale) + K.CAP_Y1) / 2, 0), (2 * K.CAP_HW, K.CAP_Y1 - K.cap_y0(scale), 2 * K.CAP_HW)))
    zc = (K.BLOCK_Z0 + K.BLOCK_Z1) / 2
    for side, sg in (("l", 1), ("r", -1)):
        tag("steel", B.box("saddle-" + side, "post", STEEL, 0.3, (0, (K.CAP_Y1 + K.BLOCK_Y1) / 2, sg * zc), (2 * K.BLOCK_HX, K.BLOCK_Y1 - K.CAP_Y1, K.BLOCK_Z1 - K.BLOCK_Z0)))
    if scale <= 100:
        tag("steel", B.cyl("saddle-shaft", "post", STEEL, 0.25, (0, K.PIVOT[1], -K.BLOCK_Z1 - 0.02), (0, K.PIVOT[1], K.BLOCK_Z1 + 0.02), 0.075))


def beam(B, scale):
    if scale == 50:
        sec = K.ibeam(K.BEAM_V0, K.BEAM_V1, K.BEAM_BF / 2, K.BEAM_TF, K.BEAM_TW)
        tag("steel", B.along("beam", "beam", STEEL, 0.7, sec, K.BEAM_X0, K.BEAM_X1))
        for i, x in enumerate((0.0, K.C_TAIL)):
            for side, sg in (("l", 1), ("r", -1)):
                zc = sg * (K.BEAM_TW / 2 + (K.BEAM_BF / 2 - K.BEAM_TW / 2) / 2)
                tag("steel", B.box("stiffener-%d-%s" % (i + 1, side), "beam", STEEL, 0.6, (x, K.PIVOT[1] + (K.BEAM_V0 + K.BEAM_V1) / 2, zc), (0.012, K.BEAM_V1 - K.BEAM_V0 - 2 * K.BEAM_TF, K.BEAM_BF / 2 - K.BEAM_TW / 2)))
    elif scale in (100, 200):
        sec = K.ibeam(K.BEAM_V0, K.BEAM_V1, K.BEAM_BF / 2, 0.02 if scale == 100 else 0.03, 0.016 if scale == 100 else 0.03)
        tag("steel", B.along("beam", "beam", STEEL, 0.7, sec, K.BEAM_X0, K.BEAM_X1))
    else:
        bw = K.BEAM_BF
        tag("steel", B.along("beam", "beam", STEEL, 0.7, [(K.BEAM_V0, -bw / 2), (K.BEAM_V0, bw / 2), (K.BEAM_V1, bw / 2), (K.BEAM_V1, -bw / 2)], K.BEAM_X0, K.BEAM_X1))
    s = DR.LEVEL[scale]
    tag("steel", B.side("centre-bearing", "beam", STEEL, 0.3, DR.centre_housing(s), -K.HOUSING_R, K.HOUSING_R, local=True))
    if scale <= 100:
        top = K.BEAM_V0 - K.HOUSING_PLATE
        cheek = [(K.C_TAIL - K.HANGER_R, top), (K.C_TAIL + K.HANGER_R, top), (K.C_TAIL + K.HANGER_R, 0.0)] + K.arc(K.C_TAIL, 0, K.HANGER_R, 0, -180, max(3, s["arc"] // 2))[1:-1] + [(K.C_TAIL - K.HANGER_R, 0.0)]
        for side, sg in (("l", 1), ("r", -1)):
            tag("steel", B.side("tail-pin-" + side, "beam", STEEL, 0.3, cheek, sg * 0.12, sg * 0.15, local=True))
        tag("steel", B.box("tail-plate", "beam", STEEL, 0.3, (K.C_TAIL, K.PIVOT[1] + K.BEAM_V0 - K.HOUSING_PLATE / 2, 0), (2 * K.HANGER_HW, K.HOUSING_PLATE, 0.30)))
    elif scale == 200:
        tag("steel", B.side("tail-pin", "beam", STEEL, 0.3, DR.tail_hanger(s), -0.15, 0.15, local=True))


def horsehead(B, scale):
    s = DR.LEVEL[scale]
    out = beam_frame_to_head(K.head_outline(s["head"]))
    if scale == 50:
        inner = K.arc(0, 0, K.A_HEAD - K.FACE_T, K.HEAD_A1, K.HEAD_A0, s["head"]) + [(K.HEAD_REAR_X, K.HEAD_REAR_TOP), (K.HEAD_REAR_X, K.HEAD_REAR_BOT)]
        tag("steel", B.side("horsehead", "horsehead", STEEL, 0.78, beam_frame_to_head(inner), -K.HEAD_HW, K.HEAD_HW, local=True))
        band = beam_frame_to_head(K.face_band(s["head"]))
        tag("steel", B.side("horsehead-face", "horsehead", STEEL, 0.3, band, -K.FACE_HW, K.FACE_HW, local=True))
    else:
        tag("steel", B.side("horsehead", "horsehead", STEEL, 0.78, out, -K.FACE_HW, K.FACE_HW, local=True))


def pitman(B, scale):
    s = DR.LEVEL[scale]
    hv = K.PIT_DEPTH / 2
    for side, sg in (("l", 1), ("r", -1)):
        zc = sg * (K.PIT_Z0 + K.PIT_Z1) / 2
        if scale <= 100:
            tag("steel", B.side("pitman-" + side, "pitman", STEEL, 0.6, [(0.2, -hv), (2.56, -hv), (2.56, hv), (0.2, hv)], sg * K.PIT_Z0, sg * K.PIT_Z1, local=True))
            tag("steel", B.side("pitman-head-" + side, "pitman", STEEL, 0.55, [(-0.09, -0.13), (0.26, -0.13), (0.26, 0.13), (-0.09, 0.13)], sg * K.PIT_Z0, sg * K.PIT_Z1, local=True))
            tag("steel", B.side("pitman-eye-" + side, "pitman", STEEL, 0.3, K.circle(K.P_PITMAN, 0, K.EYE_R, 24 if scale == 50 else 16), sg * K.EYE_Z0, sg * K.EYE_Z1, local=True))
        else:
            tag("steel", B.side("pitman-" + side, "pitman", STEEL, 0.6, DR.pitman_local(s), sg * K.PIT_Z0, sg * K.PIT_Z1, local=True))
    eq = [(K.eq_u0(scale), -K.EQ_V), (K.EQ_U1, -K.EQ_V), (K.EQ_U1, K.EQ_V), (K.eq_u0(scale), K.EQ_V)]
    if scale <= 100:
        for side, sg in (("l", 1), ("r", -1)):
            tag("steel", B.side("equalizer-" + side, "pitman", STEEL, 0.45, eq, sg * 0.155, sg * (K.EQ_HW if scale == 50 else K.EQ_HW), local=True))
        tag("steel", B.side("equalizer-bearing", "pitman", STEEL, 0.3, K.circle(0, 0, 0.10, 20), -0.12, 0.12, local=True))
    else:
        tag("steel", B.side("equalizer", "pitman", STEEL, 0.45, eq, -K.EQ_HW, K.EQ_HW, local=True))


def crank(B, scale):
    s = DR.LEVEL[scale]
    arm = DR.crank_arm(s)
    cw = DR.counterweight(s)
    for side, sg in (("l", 1), ("r", -1)):
        tag("crank", B.side("crank-arm-" + side, "crank", CAST, 0.58, arm, sg * K.ARM_Z0, sg * K.ARM_Z1, local=True))
        tag("crank", B.side("counterweight-" + side, "crank", CAST, 0.4, cw, sg * K.CW_Z0, sg * K.CW_Z1, local=True))
        if scale <= 200:
            tag("crank", B.side("crank-hub-" + side, "crank", CAST, 0.3, K.circle(0, 0, K.HUB_R, 24 if scale == 50 else 16), sg * K.HUB_Z0, sg * K.HUB_Z1, local=True))
        if scale <= 100:
            tag("crank", B.side("crank-pin-" + side, "crank", CAST, 0.25, K.circle(K.R_CRANK, 0, K.PIN_R, 16), sg * K.ARM_Z0, sg * K.EYE_Z1, local=True))


def gearbox(B, scale):
    s = DR.LEVEL[scale]
    tag("gearbox", B.side("gearbox", "gearbox", CAST, 0.62, DR.gearbox_pts(s), -K.GB_HW, K.GB_HW, local=True))
    tag("gearbox", B.box("pedestal", "gearbox", CAST, 0.5, (K.CRANK[0] + 0.49, K.CRANK[1] - 1.125, 0), (2.22, 0.15, 0.80)))
    if scale <= 500:
        tag("gearbox", B.side("input-shaft", "gearbox", CAST, 0.35, K.circle(K.INPUT[0], K.INPUT[1], 0.07, 12), K.GUARD_Z1 - 0.02, -K.GB_HW + 0.01, local=True))
    if scale <= 100:
        for side, sg in (("l", 1), ("r", -1)):
            tag("gearbox", B.side("bearing-cap-" + side, "gearbox", CAST, 0.55, K.circle(0, 0, 0.24, 24 if scale == 50 else 16), sg * K.GB_HW, sg * K.HUB_Z0, local=True))
        tag("gearbox", B.side("input-cap", "gearbox", CAST, 0.55, K.circle(K.INPUT[0], K.INPUT[1], 0.16, 16), K.GB_HW, K.GB_HW + 0.05, local=True))
    tag("steel", B.side("belt-guard", "gearbox", STEEL, 0.45, DR.guard_pts(s), K.GUARD_Z0, K.GUARD_Z1, local=True))
    if scale == 50:
        mb = K.L2("gearbox", K.MOTOR)
        for i, (gx, top) in enumerate(((K.INPUT[0], K.CRANK[1] + K.INPUT[1] - K.GUARD_RA + 0.03), (mb[0], K.CRANK[1] + mb[1] - K.GUARD_RB + 0.03))):
            tag("steel", B.box("guard-bracket-%d" % (i + 1), "gearbox", STEEL, 0.4, (K.CRANK[0] + gx, (0.55 + top) / 2, (K.GUARD_Z0 + K.GUARD_Z1) / 2), (0.08, top - 0.55, 0.16)))


def motor(B, scale):
    s = DR.LEVEL[scale]
    my = K.MOTOR
    z = (0, 0, 1)
    if scale == 50:
        tag("motor", B.side("motor", "motor", MOT, 0.68, K.motor_fins(36, K.MOTOR_CORE, K.MOTOR_R, 0.005), K.MOTOR_Z0, K.MOTOR_Z1, local=True))
        tag("motor", B.rev("motor-endshield", "motor", MOT, 0.6, (my[0], my[1], K.MOTOR_Z0), (0, 0, -1), [[-0.01, 0.235], [0.03, 0.225], [0.07, 0.15], [0.085, 0.07]]))
        tag("motor", B.rev("motor-cowl", "motor", MOT, 0.5, (my[0], my[1], K.MOTOR_Z1 - 0.01), z, [[0, K.COWL_R], [0.17, K.COWL_R], [0.21, 0.2]]))
        tag("motor", B.cyl("motor-shaft", "motor", MOT, 0.35, (my[0], my[1], K.GUARD_Z1 - 0.03), (my[0], my[1], K.MOTOR_Z0 - 0.08), 0.035))
        tag("motor", B.box("terminal-box", "motor", MOT, 0.5, (my[0], my[1] + 0.31, -0.20), (0.22, 0.14, 0.22)))
        for i, zc in enumerate((-0.38, 0.0)):
            tag("motor", B.box("motor-foot-%d" % (i + 1), "motor", MOT, 0.5, (my[0], my[1] - 0.265, zc), (0.44, 0.09, 0.08)))
        tag("motor", B.box("motor-feet", "motor", MOT, 0.5, (my[0], my[1] - 0.33, -0.21), (0.64, 0.04, 0.68)))
    elif scale == 100:
        tag("motor", B.side("motor", "motor", MOT, 0.68, K.motor_fins(18, K.MOTOR_CORE, K.MOTOR_R, 0.0075), K.MOTOR_Z0, K.MOTOR_Z1, local=True))
        tag("motor", B.rev("motor-endshield", "motor", MOT, 0.6, (my[0], my[1], K.MOTOR_Z0), (0, 0, -1), [[-0.01, 0.235], [0.03, 0.225], [0.085, 0.07]]))
        tag("motor", B.rev("motor-cowl", "motor", MOT, 0.5, (my[0], my[1], K.MOTOR_Z1 - 0.01), z, [[0, K.COWL_R], [0.17, K.COWL_R], [0.21, 0.2]]))
        tag("motor", B.cyl("motor-shaft", "motor", MOT, 0.35, (my[0], my[1], K.GUARD_Z1 - 0.03), (my[0], my[1], K.MOTOR_Z0 - 0.08), 0.035))
        tag("motor", B.box("terminal-box", "motor", MOT, 0.5, (my[0], my[1] + 0.31, -0.20), (0.22, 0.14, 0.22)))
        tag("motor", B.box("motor-feet", "motor", MOT, 0.5, (my[0], my[1] - 0.285, -0.21), (0.64, 0.13, 0.68)))
    else:
        tag("motor", B.rev("motor", "motor", MOT, 0.68, (my[0], my[1], K.MOTOR_Z0 - 0.02), z, [[0, K.MOTOR_R - 0.005], [K.COWL_Z1 - K.MOTOR_Z0 + 0.02, K.MOTOR_R - 0.005]]))
        tag("motor", B.cyl("motor-shaft", "motor", MOT, 0.35, (my[0], my[1], K.GUARD_Z1 - 0.03), (my[0], my[1], K.MOTOR_Z0 - 0.02), 0.035 if scale == 200 else 0.04))
        tag("motor", B.box("motor-feet", "motor", MOT, 0.5, (my[0], my[1] - 0.285, -0.21), (0.64, 0.13, 0.68)))


def carrier(B, scale):
    c = (K.WELL_X, K.ROD_Y, 0)
    hu, hv = K.carrier_h(scale)
    tag("steel", B.box("carrier", "rod", STEEL, 0.3, c, (2 * hv, 2 * hu, 2 * K.CARRIER_HZ)))
    if scale <= 100:
        tag("steel", B.box("rod-clamp", "rod", STEEL, 0.25, (K.WELL_X, K.ROD_Y + K.CARRIER_HU + 0.05, 0), (0.12, 0.10, 0.12)))


def body(scale):
    REGION.clear()
    B = K.Solids()
    base_parts(B, scale)
    wellhead(B, scale)
    post(B, scale)
    beam(B, scale)
    horsehead(B, scale)
    pitman(B, scale)
    crank(B, scale)
    gearbox(B, scale)
    motor(B, scale)
    carrier(B, scale)
    return B.items, dict(REGION)


def structure(scale, body_items):
    """Load path: foundation, skid beams, the four legs with braces and cap, the walking beam split at the saddle into its tail arm (bone `beam`) and front arm with horsehead (bone `horsehead`), the pitman arms with the equalizer, the crank arms, the reducer housing and the motor frame; joints at the saddle, the equalizer bearing, the wrist pins and the crank hubs; masses at the centres of mass."""
    S = K.Solids()
    ids = {d["id"] for d in body_items}
    out = []

    def ref(id, of, member, part):
        if of in ids:
            out.append({"id": id, "of": of, "member": member, "part": part})

    ref("member-slab", "slab", "base", "member")
    for d in body_items:
        if d["bone"] == "skid" and "deck" not in d["id"]:
            ref("member-" + d["id"], d["id"], "skid", "member")
    for d in body_items:
        if d["bone"] == "post" and not d["id"].startswith("saddle"):
            ref("member-" + d["id"], d["id"], "post", "member")
    ref("member-polished-rod", "polished-rod", "base", "member")
    ref("member-carrier", "carrier", "rod", "member")
    if scale == 50:
        sec = K.ibeam(K.BEAM_V0, K.BEAM_V1, K.BEAM_BF / 2, K.BEAM_TF, K.BEAM_TW)
        hsec = [(-v, w) for v, w in sec]
    elif scale in (100, 200):
        sec = K.ibeam(K.BEAM_V0, K.BEAM_V1, K.BEAM_BF / 2, 0.02 if scale == 100 else 0.03, 0.016 if scale == 100 else 0.03)
        hsec = [(-v, w) for v, w in sec]
    else:
        bw = K.BEAM_BF / 2
        sec = [(K.BEAM_V0, -bw), (K.BEAM_V0, bw), (K.BEAM_V1, bw), (K.BEAM_V1, -bw)]
        hsec = [(-v, w) for v, w in sec]
    S.along("member-beam-tail", "beam", STEEL, 0.4, sec, 0.0, K.BEAM_X1, member="beam", part="member")
    S.along("member-beam-front", "horsehead", STEEL, 0.4, hsec, 0.0, -K.BEAM_X0, member="horsehead", part="member")
    ref("member-horsehead", "horsehead", "horsehead", "member")
    for d in body_items:
        if d["bone"] == "pitman" and (d["id"].startswith("pitman-") and not d["id"].startswith("pitman-eye") or d["id"].startswith("equalizer") and d["id"] != "equalizer-bearing"):
            ref("member-" + d["id"], d["id"], "pitman", "member")
    for side in ("l", "r"):
        ref("member-crank-arm-" + side, "crank-arm-" + side, "crank", "member")
    ref("member-gearbox", "gearbox", "gearbox", "member")
    ref("member-pedestal", "pedestal", "gearbox", "member")
    ref("member-motor", "motor", "motor", "member")
    ref("joint-saddle", "saddle-shaft", "post", "joint")
    if "saddle-shaft" not in ids:
        S.cyl("joint-saddle", "post", STEEL, 0.25, (0, K.PIVOT[1], -K.BLOCK_Z1), (0, K.PIVOT[1], K.BLOCK_Z1), 0.09 if scale == 200 else 0.12, member="post", part="joint")
    ref("joint-centre-bearing", "centre-bearing", "beam", "joint")
    if "equalizer-bearing" in ids:
        ref("joint-equalizer", "equalizer-bearing", "pitman", "joint")
    else:
        S.cyl("joint-equalizer", "pitman", STEEL, 0.25, (K.C_TAIL, K.PIVOT[1], -0.15), (K.C_TAIL, K.PIVOT[1], 0.15), 0.10, member="pitman", part="joint")
    for side, sg in (("l", 1), ("r", -1)):
        if "crank-pin-" + side in ids:
            ref("joint-wrist-pin-" + side, "crank-pin-" + side, "crank", "joint")
        else:
            S.cyl("joint-wrist-pin-" + side, "crank", CAST, 0.25, (K.CRANK[0] + K.R_CRANK, K.CRANK[1], sg * K.ARM_Z0), (K.CRANK[0] + K.R_CRANK, K.CRANK[1], sg * K.EYE_Z1), 0.09, member="crank", part="joint")
        if "crank-hub-" + side in ids:
            ref("joint-crank-hub-" + side, "crank-hub-" + side, "crank", "joint")
        else:
            S.cyl("joint-crank-hub-" + side, "crank", CAST, 0.25, (K.CRANK[0], K.CRANK[1], sg * K.HUB_Z0), (K.CRANK[0], K.CRANK[1], sg * K.HUB_Z1), K.HUB_R, member="crank", part="joint")
    for side in ("l", "r"):
        ref("mass-counterweight-" + side, "counterweight-" + side, "crank", "mass")
    f = {50: 1.0, 100: 1.15, 200: 1.4, 500: 2.0}[scale]
    S.box("mass-base", "base", CONC, 0.4, (1.9, 0.15, 0), (0.3 * f, 0.15, 0.3 * f), member="base", part="mass")
    S.ell("mass-gearbox", "gearbox", CAST, 0.4, (K.CRANK[0] + 0.5, K.CRANK[1] - 0.4, 0), (0.12 * f, 0.12 * f, 0.12 * f), member="gearbox", part="mass")
    S.ell("mass-motor", "motor", MOT, 0.4, (K.MOTOR[0], K.MOTOR[1], -0.21), (0.08 * f, 0.08 * f, 0.08 * f), member="motor", part="mass")
    S.ell("mass-beam", "beam", STEEL, 0.4, (0.81, K.PIVOT[1] + 0.52, 0), (0.08 * f, 0.08 * f, 0.08 * f), member="beam", part="mass")
    hx, hy = K.W2("horsehead", (2.3, -0.2))
    S.ell("mass-horsehead", "horsehead", STEEL, 0.4, (hx, hy, 0), (0.1 * f, 0.1 * f, 0.1 * f), member="horsehead", part="mass")
    return out + S.items


DECISIONS = {
    50: [
        ("Full construction at 1:50 (smallest feature 7.5 mm): concrete foundation; skid of two rolled I-beams (240 mm, 15 mm flanges, 12 mm web) with five channel cross members and a 10 mm deck; wellhead as casing head with flange, pumping tee, stuffing box and flowline; the polished rod and the vertical bridle run are one 50 mm slender line from the stuffing box to the tangent point of the horsehead, as drawn.",
         "Vollständige Konstruktion bei 1:50 (kleinstes Detail 7,5 mm): Betonfundament; Rahmen aus zwei gewalzten Doppel-T-Trägern (240 mm, Flansche 15 mm, Steg 12 mm) mit fünf U-Querträgern und einem Deckblech von 10 mm; Bohrlochkopf als Futterrohrkopf mit Flansch, Pumpen-T-Stück, Stopfbuchse und Flowline; Polierstange und senkrechter Seilstrang sind wie gezeichnet eine schlanke 50-mm-Linie von der Stopfbuchse zum Tangentenpunkt des Pferdekopfs."),
        ("Samson post of four 170 mm legs splayed in both directions from the skid to a cap plate, braced by a square frame at 1.68 m, with two saddle blocks holding the 150 mm saddle shaft; walking beam as a W24 rolled section (611 mm deep, 325 mm wide and 19 mm thick flanges, 12.7 mm web) with web stiffeners over both bearings, the centre bearing housing and the equalizer hanger bolted under its bottom flange.",
         "Lagerbock aus vier 170-mm-Beinen, in beiden Richtungen gespreizt vom Rahmen zu einer Kopfplatte, ausgesteift durch einen quadratischen Rahmen auf 1,68 m, mit zwei Sattelböcken für die 150-mm-Sattelwelle; Balancier als gewalztes W24-Profil (611 mm hoch, Flansche 325 mm breit und 19 mm dick, Steg 12,7 mm) mit Stegsteifen über beiden Lagern, Mittellagergehäuse und Ausgleichslagerbock unter dem Untergurt verschraubt."),
        ("Horsehead as a closed side-plate box with a 30 mm curved face plate of radius A = 3.0 m about the saddle that overhangs the side plates; two pitman arms outboard of the crank arms, each with its upper fitting on the transverse equalizer and its wrist-pin bearing housing; the equalizer turns on its bearing between the hanger plates. Cranks in phase with the wrist pin (the weights hang down at the top of the stroke): per side a cast arm, a counterweight block on its inner face, the hub on the low-speed shaft and the wrist pin.",
         "Pferdekopf als geschlossener Seitenblechkasten mit einem 30-mm-Bogenblech vom Radius A = 3,0 m um den Sattel, das über die Seitenbleche vorsteht; zwei Schubstangen außen an den Kurbelarmen, jede mit ihrem Kopfstück auf dem querliegenden Ausgleichsbalken und ihrem Kurbelzapfen-Lagergehäuse; der Ausgleichsbalken dreht in seinem Lager zwischen den Hängeblechen. Kurbeln in Phase mit dem Kurbelzapfen (die Gewichte hängen am oberen Hubende unten): je Seite ein gegossener Arm, ein Gegengewicht auf seiner Innenseite, die Nabe auf der langsamen Welle und der Kurbelzapfen."),
        ("Gear reducer as its cast housing profile (arch over the bull gear, sloping top to the input end beyond the crank circle, base flange) on a sub-base, with bearing caps, input cap and input shaft into the belt guard; belt guard as the closed sheet enclosure around both sheaves on two brackets; 30 kW TEFC motor (frame 324T) with its axis across the unit: 24-fin frame, drive-end shield, fan cowl, shaft into the guard, terminal box, feet and slide base. Thermal: every region a reference to its body solids (one layer each, no hidden regions, no emitters).",
         "Getriebe als Profil seines Gussgehäuses (Bogen über dem großen Zahnrad, schräge Oberseite zum Antriebsende jenseits des Kurbelkreises, Fußflansch) auf einem Untersatz, mit Lagerdeckeln, Antriebsdeckel und Antriebswelle in den Riemenschutz; Riemenschutz als geschlossenes Blechgehäuse um beide Scheiben auf zwei Konsolen; 30-kW-Motor IC 411 (Baugröße 324T) mit der Achse quer zur Anlage: Rippengehäuse mit 24 Rippen, antriebsseitiger Lagerschild, Lüfterhaube, Welle in den Schutz, Klemmenkasten, Füße und Spannschiene. Thermik: jede Region als Verweis auf ihre Körper (je eine Schicht, keine verdeckten Regionen, keine Emitter)."),
    ],
    100: [
        ("At 1:100 (15 mm): the same assemblies with merged details: skid I-beams with 15 mm webs and flanges, cross members as bars, 15 mm deck; wellhead as two stepped cylinders as drawn; beam as an I-section with 20 mm flanges and a 16 mm web, no stiffeners; horsehead one extruded box with the face; pitman arms, upper fittings, eyes and a one-piece equalizer on its bearing; crank arms, counterweights, hubs and wrist pins.",
         "Bei 1:100 (15 mm): dieselben Baugruppen mit zusammengefassten Details: Rahmenträger mit 15-mm-Stegen und -Flanschen, Querträger als Stäbe, Deck 15 mm; Bohrlochkopf als zwei gestufte Zylinder wie gezeichnet; Balancier als Doppel-T mit 20-mm-Flanschen und 16-mm-Steg ohne Steifen; Pferdekopf ein extrudierter Kasten mit Bogenblech; Schubstangen mit Kopfstücken, Augen und einteiligem Ausgleichsbalken auf seinem Lager; Kurbelarme, Gegengewichte, Naben und Kurbelzapfen."),
        ("Reducer housing with fewer arc segments, bearing caps, input cap and input shaft; belt guard without brackets; motor as a smooth frame over the fin tips with shield, cowl, shaft, terminal box and a merged foot and slide base. Thermal by reference; structure as at 1:50 with its own beam arms split at the saddle.",
         "Getriebegehäuse mit weniger Bogensegmenten, Lagerdeckeln, Antriebsdeckel und Antriebswelle; Riemenschutz ohne Konsolen; Motor als glattes Gehäuse über den Rippenspitzen mit Schild, Haube, Welle, Klemmenkasten und zusammengefasstem Fuß mit Spannschiene. Thermik als Verweis; Tragwerk wie bei 1:50 mit eigenen, am Sattel geteilten Balancierarmen."),
    ],
    200: [
        ("At 1:200 (30 mm): the walking beam keeps its I-section with flanges and web generalised to 30 mm; the skid is two box beams, three cross bars and a 30 mm deck; the wellhead is one stepped cylinder; the bearing housings, saddle blocks, equalizer, pitman arms with their eyes, crank arms, counterweights and hubs stay as separate masses; the saddle shaft, wrist pins, bearing caps, input shaft, terminal box and guard brackets are dropped.",
         "Bei 1:200 (30 mm): der Balancier behält seinen Doppel-T-Querschnitt mit auf 30 mm verallgemeinerten Flanschen und Steg; der Rahmen besteht aus zwei Kastenträgern, drei Querstäben und einem 30-mm-Deck; der Bohrlochkopf ist ein gestufter Zylinder; Lagergehäuse, Sattelböcke, Ausgleichsbalken, Schubstangen mit Augen, Kurbelarme, Gegengewichte und Naben bleiben eigene Massen; Sattelwelle, Kurbelzapfen, Lagerdeckel, Antriebswelle, Klemmenkasten und Schutzkonsolen entfallen."),
        ("Motor one cylinder over frame and cowl on its foot block; reducer housing with coarse arcs. Thermal by reference; structure with its own joint cylinders where the body has no pin.",
         "Motor ein Zylinder über Gehäuse und Haube auf seinem Fußblock; Getriebegehäuse mit groben Bögen. Thermik als Verweis; Tragwerk mit eigenen Gelenkzylindern, wo der Körper keinen Zapfen hat."),
    ],
    500: [
        ("At 1:500 (75 mm), where flanges and web of 30 mm cannot be shown, the walking beam is its 611 x 325 mm envelope; a composed mark that keeps every assembly in its true proportion: slab, skid as two beams under a 75 mm deck, wellhead cylinder, polished rod widened to 0.1 m as drawn, four legs without braces, cap and saddle blocks, the beam envelope with simplified housings, horsehead, the two pitman arms with their eyes merged, a one-piece equalizer, crank arms with counterweights, a polygonal reducer housing, the belt guard and the motor as one cylinder on its foot block.",
         "Bei 1:500 (75 mm), wo Flansche und Steg von 30 mm nicht darstellbar sind, ist der Balancier seine Hülle von 611 x 325 mm; ein zusammengesetztes Zeichen, das jede Baugruppe in ihrem wahren Verhältnis behält: Platte, Rahmen aus zwei Trägern unter einem 75-mm-Deck, Bohrlochkopfzylinder, Polierstange wie gezeichnet auf 0,1 m verbreitert, vier Beine ohne Streben, Kopfplatte und Sattelböcke, die Balancierhülle mit vereinfachten Lagergehäusen, Pferdekopf, die zwei Schubstangen mit eingeschlossenen Augen, ein einteiliger Ausgleichsbalken, Kurbelarme mit Gegengewichten, ein polygonales Getriebegehäuse, der Riemenschutz und der Motor als ein Zylinder auf seinem Fußblock."),
    ],
}


def volume(scale):
    items, reg = body(scale)
    thermal = [{"id": "th-" + d["id"], "of": d["id"], "region": reg[d["id"]]} for d in items]
    return {"status": "authored", "denominator": scale, "body": items, "thermal": thermal, "structure": structure(scale, items), "decisions": [{"en": e, "de": g} for e, g in DECISIONS[scale]]}
