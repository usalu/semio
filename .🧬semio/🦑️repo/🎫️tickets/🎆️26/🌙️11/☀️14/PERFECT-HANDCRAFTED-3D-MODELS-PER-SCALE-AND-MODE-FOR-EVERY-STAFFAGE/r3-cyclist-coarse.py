"""🔭 Round 3 bodies of humans/cyclist at 1:100, 1:200 and 1:500 (smallest feature 15, 30, 75 mm): each scale authored anew with its own sections, merged masses and simplified assemblies, so the woman on the low-step city bicycle stays right in proportion at a glance.

Every function returns solids with the private keys `_region` and `_g` (stripped by the assembler).
"""
import importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-cyclist-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


B = mod("body50")
R, K = B.R, B.K
tag, SIDES, TONE = B.tag, B.SIDES, B.TONE
SKIN, HAIR, JACKET, TROUSER, GLOVE, SHOE, HELMET = B.SKIN, B.HAIR, B.JACKET, B.TROUSER, B.GLOVE, B.SHOE, B.HELMET
STEEL, RUBBER, SADDLE = B.STEEL, B.RUBBER, B.SADDLE


def helmet(rows, c0, tilt):
    t = math.radians(tilt)
    n = [math.cos(t), math.sin(t), 0.0]
    d = [math.sin(t), -math.cos(t), 0.0]
    q = K.quat(K.W, d)
    return [K.section([c0[0] + s * n[0], c0[1] + s * n[1], 0], hw, hd, e, q) for s, hd, hw, e in rows]


def rider100(out, F):
    out.append(tag(K.stack("pelvis-trousers", "pelvis", [(-0.110, 0.034, 0.054, 0.064, 2.1), (-0.075, 0.062, 0.104, 0.142, 2.3), (-0.025, 0.074, 0.116, 0.168, 2.3), (0.035, 0.079, 0.104, 0.164, 2.3), (0.090, 0.076, 0.080, 0.144, 2.2)], TROUSER, 0.2), "trousers", "pelvis-trousers"))
    out.append(tag(K.stack("jacket", "torso", [(-0.124, 0.130, 0.090, 0.174, 2.3), (-0.112, 0.146, 0.104, 0.190, 2.3), (-0.05, 0.130, 0.092, 0.177, 2.3), (0.06, 0.112, 0.050, 0.136, 2.3), (0.20, 0.148, 0.060, 0.150, 2.3), (0.27, 0.167, 0.071, 0.159, 2.35), (0.36, 0.126, 0.077, 0.186, 2.4), (0.408, 0.092, 0.070, 0.198, 2.5), (0.430, 0.074, 0.062, 0.182, 2.4), (0.446, 0.060, 0.055, 0.122, 2.2), (0.466, 0.045, 0.043, 0.062, 2.0)], JACKET, 0.3), "jacket", "jacket"))
    out.append(tag(K.stack("neck", "neck", [(-0.03, 0.068, 0.020, 0.050), (0.06, 0.079, 0.005, 0.053), (0.13, 0.070, 0.020, 0.050)], SKIN, 0.84), "face-skin", "neck"))
    out.append(tag(K.stack("collar", "neck", [(-0.035, 0.078, 0.060, 0.080, 2.2), (0.0, 0.085, 0.030, 0.064, 2.2), (0.056, 0.085, 0.019, 0.061, 2.1)], JACKET, 0.3), "jacket", "collar"))
    out.append(tag(K.stack("head", "head", [(-0.080, 0.092, -0.058, 0.018, 2.0), (-0.055, 0.106, -0.012, 0.048, 2.1), (-0.015, 0.112, 0.048, 0.063, 2.1), (0.035, 0.110, 0.088, 0.072, 2.0), (0.085, 0.100, 0.094, 0.073, 2.0), (0.130, 0.062, 0.066, 0.055, 2.0), (0.158, 0.008, 0.010, 0.009, 2.0)], SKIN, 0.86), "face-skin", "head"))
    out.append(tag(K.stack("nose", "head", [(0.040, 0.108, -0.100, 0.0075), (0.012, 0.125, -0.100, 0.0085), (-0.002, 0.116, -0.100, 0.0085)], SKIN, 0.86, role="detail"), "face-skin", "head"))
    for s, n in SIDES:
        out.append(tag(K.ellipsoid(f"ear-{n}", "head", [0.030, 0.017, 0.0085], [0.016, 0.010, s * 0.072], SKIN, 0.8, role="detail", rotation=K.rot_w(-12)), "face-skin", "ear"))
    out.append(tag(K.tube("hair-tail", "head", [([0.045, 0.090, 0], 0.024, 0.024), ([0.012, 0.126, 0], 0.026, 0.024), ([-0.050, 0.134, 0], 0.023, 0.021), ([-0.140, 0.118, 0], 0.010, 0.010)], HAIR, 0.25), "hair", "hair-tail"))
    out.append(tag(K.loft("helmet", "head", helmet([(0.0, 0.127, 0.099, 2.3), (0.018, 0.134, 0.106, 2.2), (0.060, 0.124, 0.100, 2.1), (0.100, 0.090, 0.075, 2.0), (0.124, 0.044, 0.037, 2.0), (0.131, 0.012, 0.010, 2.0)], B.HELMET_C0, B.HELMET_TILT), HELMET, 0.82, caps=(False, True)), "helmet", "helmet"))
    for s, n in SIDES:
        tone = TONE[f"jacket-{n}"]
        out.append(tag(K.limb(f"sleeve-{n}", f"arm-{n}", [(-0.040, 0.0, -0.020, 0.030, 0.028), (0.0, 0.003, 0.002, 0.048, 0.045), (0.08, 0.004, 0.005, 0.049, 0.046), (0.22, 0.002, 0.0, 0.043, 0.041), (0.32, -0.002, 0.0, 0.039, 0.042), (0.355, -0.002, 0.0, 0.028, 0.030)], JACKET, tone, s), "jacket", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", [(-0.045, 0.0, 0.0, 0.030, 0.032), (0.03, 0.002, 0.002, 0.042, 0.044), (0.13, 0.0, 0.0, 0.034, 0.039), (0.225, 0.0, 0.0, 0.027, 0.035), (0.258, 0.0, 0.0, 0.026, 0.034)], JACKET, tone, s), "jacket", f"forearm-{n}"))
        g = TONE[f"glove-{n}"]
        out.append(tag(K.limb(f"glove-{n}", f"hand-{n}", [(-0.022, 0.002, 0.0, 0.023, 0.033, 2.2), (0.02, 0.002, 0.0, 0.018, 0.040, 2.5), (0.07, 0.0, 0.0, 0.016, 0.043, 2.6), (0.100, -0.004, 0.0, 0.011, 0.039, 2.4)], GLOVE, g, s), "gloves", f"glove-{n}"))
        c, d, up, ahead = B.grip_frame(F, s)
        roll = []
        for ph in (-12, 45, 105, 165, 205):
            a = math.radians(ph)
            radial = [up[q] * math.cos(a) + ahead[q] * math.sin(a) for q in range(3)]
            tangent = [-up[q] * math.sin(a) + ahead[q] * math.cos(a) for q in range(3)]
            p = [c[q] + radial[q] * (R.GRIP_R + 0.0112) for q in range(3)]
            roll.append(K.section(p, 0.036, 0.0105, 3.0, K.quat(K.unit(d), K.cross(tangent, K.unit(d)))))
        out.append(tag(K.loft(f"fingers-{n}", f"hand-{n}", roll, GLOVE, round(g * 0.55, 4), role="detail"), "gloves", f"glove-{n}"))
        m = -s
        out.append(tag(K.tube(f"glove-thumb-{n}", f"hand-{n}", [([0.004, -0.010, m * 0.030], 0.013, 0.012), ([0.045, -0.036, m * 0.041], 0.0105, 0.0098), ([0.090, -0.052, m * 0.022], 0.0085, 0.0080)], GLOVE, g, hint=K.U, role="detail"), "gloves", f"glove-thumb-{n}"))
        t = TONE[f"trouser-{n}"]
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", [(-0.075, 0.0, -0.01, 0.060, 0.058), (-0.02, 0.008, 0.011, 0.086, 0.084), (0.10, 0.012, 0.007, 0.084, 0.078), (0.25, 0.010, 0.0, 0.070, 0.064), (0.37, 0.012, -0.002, 0.055, 0.053), (0.432, 0.008, 0.0, 0.050, 0.049), (0.470, 0.0, 0.0, 0.026, 0.026)], TROUSER, t, s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", [(-0.050, 0.004, 0.0, 0.034, 0.034), (0.020, 0.0, 0.0, 0.050, 0.048), (0.12, -0.016, -0.002, 0.057, 0.050), (0.27, -0.007, 0.0, 0.044, 0.040), (0.40, 0.002, 0.0, 0.037, 0.035), (0.445, 0.002, 0.0, 0.028, 0.028)], TROUSER, t, s), "trousers", f"shank-{n}"))
        out.append(tag(K.limb(f"trouser-clip-{n}", f"shank-{n}", [(0.300, -0.004, 0.0, 0.0450, 0.0420), (0.346, -0.002, 0.0, 0.0430, 0.0400)], TROUSER, 0.75, s, role="detail"), "trousers", f"trouser-clip-{n}"))
        tone = TONE[f"shoe-{n}"]
        out.append(tag(K.loft(f"shoe-rear-{n}", f"foot-{n}", low_shoe([(-0.066, -0.046, 2.2), (-0.050, -0.004, 2.5), (0.010, -0.010, 2.5), (0.070, -0.036, 2.5), (0.132, -0.052, 2.5)], B.SOLE, s, B.foot_local, 0.0), SHOE, tone), "shoes", f"shoe-rear-{n}"))
        out.append(tag(K.loft(f"shoe-toe-{n}", f"toe-{n}", low_shoe([(0.120, -0.051, 2.5), (0.160, -0.059, 2.4), (0.198, -0.077, 2.0)], B.SOLE_TOE, s, B.toe_local, 0.0), SHOE, tone), "shoes", f"shoe-toe-{n}"))


def low_shoe(rows, table, side, conv, inset, bottom=None):
    """👞 Shoe sections whose bottom is the sole bottom (sole merged into the upper)."""
    up = conv(0.0, 1.0)
    o = conv(0.0, 0.0)
    q = K.quat(K.W, [up[0] - o[0], up[1] - o[1], 0.0])
    lo = -R.ANKLE_SHOE if bottom is None else bottom
    secs = []
    for x, top, e in rows:
        L, M = B.lm(x, table)
        u, v = conv(x, (top + lo) / 2)
        secs.append(K.section([u, v, side * (L - M) / 2], max((L + M) / 2 - inset, 0.006), (top - lo) / 2, e, q))
    return secs


def bike100(out):
    fl = R.FORK_LEN
    out.append(tag(B.revolve_axis("tube-head", "frame", [(0.0, 0.0), (0.0, 0.0225), (R.HT_LEN, 0.0225), (R.HT_LEN, 0.0)], B.steer(fl), B.steer(fl + 0.01), STEEL, 0.2), "bike-metal", "tube-head"))
    st = R.STEERER_TOP
    out.append(tag(B.oval_tube("stem", "frame", [B.steer(fl + 0.012 + R.HT_LEN - 0.01), [st[0], st[1], 0.0], [st[0] + 0.030, st[1] + 0.046, 0.0], [R.CLAMP[0], R.CLAMP[1], 0.0]], [0.0165, 0.0165, 0.0155, 0.0165], [0.0165, 0.0165, 0.0155, 0.0165], STEEL, 0.2), "bike-metal", "stem"))
    for s, n in SIDES:
        ax = R.grip_axis(s)
        g = R.grip_centre(s)
        gi = [g[q] - ax[q] * R.GRIP_LEN / 2 for q in range(3)]
        out.append(tag(B.path_tube(f"handlebar-{n}", "frame", [[R.CLAMP[0], R.CLAMP[1], 0.0], [R.CLAMP[0], R.CLAMP[1], s * 0.080], [R.CLAMP[0] - 0.012, R.CLAMP[1] - 0.003, s * 0.135], gi], 0.0125, 0.0115, STEEL, 0.2, hint=K.V), "bike-metal", "handlebar"))
        out.append(tag(B.revolve_axis(f"grip-{n}", "frame", [(0.0, 0.0), (0.0, 0.0185), (R.GRIP_LEN + 0.004, 0.0185), (R.GRIP_LEN + 0.004, 0.0)], gi, [gi[q] + ax[q] for q in range(3)], RUBBER, 0.1), "bike-metal", "grip"))
    crown = R.CROWN
    out.append(tag(K.box("fork-crown", "frame", [0.030, 0.026, 0.118], [crown[0], crown[1] - 0.004, 0.0], STEEL, 0.2, corner=0.009, rotation=K.rot_w(90 - R.HEAD_ANGLE)), "bike-metal", "fork"))
    for s, n in SIDES:
        out.append(tag(B.oval_tube(f"fork-{n}", "frame", [B.steer(fl - 0.010, s * 0.050), [0.927, 0.560, s * 0.051], [1.008, 0.385, s * 0.051], [R.FRONT[0], R.FRONT[1], s * 0.051]], 0.0125, 0.0095, STEEL, 0.2, hint=K.W), "bike-metal", "fork"))
    head_lo = B.steer(fl + 0.045)
    out.append(tag(B.oval_tube("tube-down", "frame", [[head_lo[0] + 0.006, head_lo[1], 0.0], [0.800, 0.600, 0.0], [0.680, 0.458, 0.0], [0.540, 0.395, 0.0], B.seat(0.085)], [0.020, 0.021, 0.022, 0.021, 0.020], [0.026, 0.028, 0.028, 0.026, 0.022], STEEL, 0.2), "bike-metal", "tube-down"))
    out.append(tag(B.oval_tube("tube-seat", "frame", [B.seat(-0.015), B.seat(R.ST_LEN)], 0.0175, 0.0175, STEEL, 0.2), "bike-metal", "tube-seat"))
    out.append(tag(B.revolve_axis("seatpost", "frame", [(0.0, 0.0), (0.0, 0.0136), (R.POST_LEN - R.ST_LEN + 0.010, 0.0136), (R.POST_LEN - R.ST_LEN + 0.010, 0.0)], B.seat(R.ST_LEN - 0.006), B.seat(R.ST_LEN + 0.3), STEEL, 0.6), "bike-metal", "seatpost"))
    out.append(tag(K.revolve("bottom-bracket", "frame", [(-0.040, 0.0), (-0.040, 0.0215), (0.040, 0.0215), (0.040, 0.0)], [R.BB[0], R.BB[1], 0.0], K.W, STEEL, 0.2), "bike-metal", "bottom-bracket"))
    for s, n in SIDES:
        out.append(tag(B.oval_tube(f"tube-chainstay-{n}", "frame", [[R.BB[0] - 0.012, R.BB[1] + 0.004, s * 0.026], [0.20, 0.322, s * 0.058], [0.0, 0.340, s * 0.066]], [0.0120, 0.0105, 0.0090], [0.0095, 0.0085, 0.0080], STEEL, 0.2, hint=K.V), "bike-metal", "tube-chainstay"))
        out.append(tag(B.oval_tube(f"tube-seatstay-{n}", "frame", [B.seat(R.ST_LEN - 0.045, s * 0.018), [0.08, 0.455, s * 0.060], [0.0, 0.345, s * 0.066]], 0.0080, 0.0080, STEEL, 0.2), "bike-metal", "tube-seatstay"))
    for n, c, a0, sweep, w in (("front", R.FRONT, -38.0, 205.0, 0.028), ("rear", R.REAR, 18.0, 207.0, 0.030)):
        r = R.FENDER_R
        prof = [(-w, r - 0.010), (0.0, r + 0.005), (w, r - 0.010), (w - 0.002, r - 0.010), (0.0, r + 0.003), (-w + 0.002, r - 0.010), (-w, r - 0.010)]
        sl = K.revolve(f"{n}-fender", "frame", prof, [c[0], c[1], 0.0], K.W, STEEL, 0.3, sweepRad=round(math.radians(sweep), 6))
        ca, sa = math.cos(math.radians(a0)), math.sin(math.radians(a0))
        sl["transform"] = {"rotation": K.rot_w(a0), "translation": [K.r5(c[0] - (c[0] * ca - c[1] * sa)), K.r5(c[1] - (c[0] * sa + c[1] * ca)), 0.0]}
        out.append(tag(sl, "bike-metal", f"{n}-fender"))
    y = R.CARRIER_Y
    for s, n in SIDES:
        out.append(tag(B.path_tube(f"carrier-side-{n}", "frame", [B.seat(R.ST_LEN - 0.060, s * 0.016), [0.250, 0.770, s * 0.030], [0.215, y, s * 0.068], [-0.115, y, s * 0.068], [-0.030, 0.560, s * 0.074], [0.008, 0.352, s * 0.072]], 0.0055, 0.0055, STEEL, 0.25, slender=True), "bike-metal", "carrier"))
    out.append(tag(B.path_tube("carrier-bars", "frame", [[0.215, y, 0.064], [0.215, y, -0.064], [-0.115, y, -0.064], [-0.115, y, 0.064]], 0.0050, 0.0050, STEEL, 0.25, slender=True), "bike-metal", "carrier"))
    out.append(tag(K.box("tail-lamp", "frame", [0.030, 0.040, 0.070], [-0.128, y - 0.022, 0.0], STEEL, 0.55, corner=0.006, role="detail"), "bike-metal", "carrier"))
    bx, by = R.BB
    guard = K.arc(bx, by, 0.104, 95, -20, 8) + K.arc(bx, by, 0.089, -20, 95, 8) + [(0.070, 0.388), (0.058, 0.398), (0.070, 0.406)]
    out.append(tag(K.extrude("chain-guard", "frame", guard, "xy", -0.072, -0.057, STEEL, 0.25), "bike-metal", "chain-guard"))
    out.append(tag(K.revolve("chainring", "crank-r", [(-0.0615, 0.0), (-0.0615, 0.083), (-0.046, 0.083), (-0.046, 0.0)], [0, 0, 0], K.W, STEEL, 0.55), "bike-metal", "chainring"))
    for s, n in SIDES:
        out.append(tag(K.extrude(f"crank-arm-{n}", f"crank-{n}", [(-0.015, 0.0), (0.0, 0.020), (0.170, 0.013), (0.184, 0.0), (0.170, -0.013), (0.0, -0.020)], "xy", s * R.CRANK_Z - 0.0075, s * R.CRANK_Z + 0.0075, STEEL, 0.35 if n == "r" else 0.3), "bike-metal", f"crank-arm-{n}"))
        z0, z1 = sorted((s * (R.CRANK_Z + 0.004), s * R.PEDAL_Z1))
        out.append(tag(K.extrude(f"pedal-{n}", f"pedal-{n}", [(-0.052, -0.0125), (-0.052, 0.0125), (0.052, 0.0125), (0.052, -0.0125)], "xy", z0, z1, STEEL, 0.15), "bike-metal", f"pedal-{n}"))
    out.append(tag(K.loft("saddle", "frame", [K.section([x, top - h, 0.0], hw, h, 2.4, K.ACROSS) for x, hw, top, h in ((0.160, 0.074, 0.904, 0.020), (0.200, 0.102, 0.924, 0.030), (0.270, 0.074, 0.919, 0.026), (0.360, 0.032, 0.913, 0.020), (0.425, 0.013, 0.903, 0.011))], SADDLE, 0.15), "saddle", "saddle"))
    p = [R.CROWN[0] + 0.044, R.CROWN[1] + 0.018, 0.0]
    out.append(tag(K.revolve("headlamp", "frame", [(-0.034, 0.0), (-0.034, 0.020), (0.008, 0.025), (0.008, 0.0)], p, K.U, STEEL, 0.7, role="detail"), "bike-metal", "headlamp"))
    out.append(tag(B.path_tube("kickstand", "frame", [[0.392, 0.296, 0.052], [0.140, 0.322, 0.084]], 0.0085, 0.0080, STEEL, 0.25, role="detail"), "bike-metal", "kickstand"))
    rc = R.WHEEL_R - R.TYRE_H / 2
    tyre = [(R.TYRE_W / 2 * math.sin(2 * math.pi * k / 12), rc + R.TYRE_H / 2 * math.cos(2 * math.pi * k / 12)) for k in range(12)]
    tyre.append(tyre[0])
    rim = [(-0.012, R.RIM_OUT), (-0.012, R.RIM_IN + 0.004), (0.0, R.RIM_IN), (0.012, R.RIM_IN + 0.004), (0.012, R.RIM_OUT), (-0.012, R.RIM_OUT)]
    for n, hl, hr in (("rear", 0.066, 0.040), ("front", 0.052, 0.034)):
        b = f"{n}-wheel"
        out.append(tag(K.revolve(f"{n}-tyre", b, tyre, [0, 0, 0], K.W, RUBBER, 0.12), "tyres", f"{n}-tyre"))
        out.append(tag(K.revolve(f"{n}-rim", b, rim, [0, 0, 0], K.W, STEEL, 0.75), "bike-metal", f"{n}-rim"))
        out.append(tag(K.revolve(f"{n}-hub", b, [(-hl, 0.0), (-hl, 0.008), (-0.040, 0.008), (-0.040, hr), (0.040, hr), (0.040, 0.008), (hl, 0.008), (hl, 0.0)], [0, 0, 0], K.W, STEEL, 0.6), "bike-metal", f"{n}-hub"))
        for k, z in enumerate((0.030, -0.030)):
            a0 = math.radians(15 * k)
            a1 = a0 + math.radians(28 if k == 0 else -28)
            sp = K.capsule(f"{n}-spokes-{k}", b, [0.032 * math.cos(a0), 0.032 * math.sin(a0), z], [(R.RIM_IN + 0.002) * math.cos(a1), (R.RIM_IN + 0.002) * math.sin(a1), 0.0], 0.001, 0.001, STEEL, 0.6, role="texture", slender=True, texture=True)
            sp["repeat"] = {"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 6, 7)}
            out.append(tag(sp, "bike-metal", f"{n}-spoke"))
    out.append(tag(K.revolve("rear-cog", "rear-wheel", [(-0.062, 0.0), (-0.062, 0.043), (-0.047, 0.043), (-0.047, 0.0)], [0, 0, 0], K.W, STEEL, 0.5, role="detail"), "bike-metal", "rear-cog"))


def body100():
    F = B.posed()
    out = []
    rider100(out, F)
    bike100(out)
    return out


def rider200(out, F):
    out.append(tag(K.stack("pelvis-trousers", "pelvis", [(-0.108, 0.036, 0.058, 0.070, 2.1), (-0.060, 0.068, 0.110, 0.158, 2.3), (0.020, 0.078, 0.110, 0.166, 2.3), (0.090, 0.076, 0.080, 0.144, 2.2)], TROUSER, 0.2), "trousers", "pelvis-trousers"))
    out.append(tag(K.stack("jacket", "torso", [(-0.115, 0.146, 0.104, 0.190, 2.3), (0.06, 0.112, 0.050, 0.136, 2.3), (0.25, 0.168, 0.068, 0.156, 2.35), (0.40, 0.100, 0.072, 0.198, 2.5), (0.432, 0.072, 0.062, 0.178, 2.4), (0.450, 0.058, 0.052, 0.110, 2.2), (0.466, 0.045, 0.043, 0.062, 2.0)], JACKET, 0.3), "jacket", "jacket"))
    out.append(tag(K.stack("neck", "neck", [(-0.03, 0.080, 0.050, 0.070), (0.04, 0.084, 0.020, 0.058), (0.13, 0.070, 0.020, 0.050)], SKIN, 0.84), "face-skin", "neck"))
    out.append(tag(K.stack("head", "head", [(-0.080, 0.094, -0.054, 0.020, 2.0), (-0.040, 0.110, 0.012, 0.054, 2.1), (0.020, 0.114, 0.080, 0.070, 2.0), (0.090, 0.100, 0.094, 0.073, 2.0), (0.150, 0.030, 0.034, 0.030, 2.0)], SKIN, 0.86), "face-skin", "head"))
    out.append(tag(K.tube("hair-tail", "head", [([0.045, 0.090, 0], 0.028, 0.026), ([0.0, 0.130, 0], 0.028, 0.026), ([-0.120, 0.122, 0], 0.016, 0.016)], HAIR, 0.25), "hair", "hair-tail"))
    out.append(tag(K.loft("helmet", "head", helmet([(0.0, 0.128, 0.100, 2.3), (0.040, 0.130, 0.104, 2.1), (0.100, 0.090, 0.075, 2.0), (0.131, 0.016, 0.014, 2.0)], B.HELMET_C0, B.HELMET_TILT), HELMET, 0.82, caps=(False, True)), "helmet", "helmet"))
    for s, n in SIDES:
        tone = TONE[f"jacket-{n}"]
        out.append(tag(K.limb(f"sleeve-{n}", f"arm-{n}", [(-0.040, 0.0, -0.020, 0.032, 0.030), (0.02, 0.004, 0.004, 0.050, 0.047), (0.20, 0.002, 0.0, 0.044, 0.042), (0.355, -0.002, 0.0, 0.032, 0.034)], JACKET, tone, s), "jacket", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", [(-0.045, 0.0, 0.0, 0.032, 0.034), (0.04, 0.002, 0.002, 0.042, 0.044), (0.258, 0.0, 0.0, 0.027, 0.035)], JACKET, tone, s), "jacket", f"forearm-{n}"))
        g = TONE[f"glove-{n}"]
        c, d, up, ahead = B.grip_frame(F, s)
        ring = [c[q] + ahead[q] * 0.010 for q in range(3)]
        out.append(tag(K.limb(f"glove-{n}", f"hand-{n}", [(-0.022, 0.002, 0.0, 0.024, 0.034, 2.2), (0.04, 0.0, 0.0, 0.020, 0.042, 2.5), (ring[0], ring[1] - 0.004, 0.0, 0.034, 0.044, 2.4), (ring[0] + 0.026, ring[1] - 0.008, 0.0, 0.020, 0.038, 2.2)], GLOVE, g, s), "gloves", f"glove-{n}"))
        t = TONE[f"trouser-{n}"]
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", [(-0.075, 0.0, -0.01, 0.060, 0.058), (0.0, 0.010, 0.011, 0.088, 0.084), (0.20, 0.010, 0.002, 0.076, 0.069), (0.404, 0.012, 0.0, 0.053, 0.051), (0.470, 0.0, 0.0, 0.032, 0.032)], TROUSER, t, s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", [(-0.050, 0.004, 0.0, 0.036, 0.036), (0.10, -0.014, -0.002, 0.057, 0.051), (0.30, -0.004, 0.0, 0.041, 0.038), (0.445, 0.002, 0.0, 0.032, 0.032)], TROUSER, t, s), "trousers", f"shank-{n}"))
        tone = TONE[f"shoe-{n}"]
        out.append(tag(K.loft(f"shoe-rear-{n}", f"foot-{n}", low_shoe([(-0.066, -0.040, 2.2), (-0.030, 0.000, 2.5), (0.060, -0.032, 2.5), (0.135, -0.052, 2.5)], B.SOLE, s, B.foot_local, 0.0), SHOE, tone), "shoes", f"shoe-rear-{n}"))
        out.append(tag(K.loft(f"shoe-toe-{n}", f"toe-{n}", low_shoe([(0.120, -0.051, 2.5), (0.198, -0.068, 2.0)], B.SOLE_TOE, s, B.toe_local, 0.0), SHOE, tone), "shoes", f"shoe-toe-{n}"))


def bike200(out):
    fl = R.FORK_LEN
    st = R.STEERER_TOP
    out.append(tag(B.oval_tube("tube-head", "frame", [B.steer(fl - 0.004), B.steer(fl + 0.012 + R.HT_LEN), [st[0], st[1], 0.0], [st[0] + 0.030, st[1] + 0.046, 0.0], [R.CLAMP[0], R.CLAMP[1], 0.0]], [0.0230, 0.0230, 0.0160, 0.0160, 0.0165], [0.0230, 0.0230, 0.0160, 0.0160, 0.0165], STEEL, 0.2), "bike-metal", "tube-head"))
    for s, n in SIDES:
        ax = R.grip_axis(s)
        g = R.grip_centre(s)
        gi = [g[q] - ax[q] * R.GRIP_LEN / 2 for q in range(3)]
        go = [g[q] + ax[q] * (R.GRIP_LEN / 2 + 0.004) for q in range(3)]
        out.append(tag(B.oval_tube(f"handlebar-{n}", "frame", [[R.CLAMP[0], R.CLAMP[1], 0.0], [R.CLAMP[0] - 0.006, R.CLAMP[1] - 0.002, s * 0.110], gi, go], [0.0155, 0.0155, 0.0185, 0.0185], [0.0155, 0.0155, 0.0185, 0.0185], STEEL, 0.15, hint=K.V), "bike-metal", "handlebar"))
    out.append(tag(K.loft("fork", "frame", [K.section(p, 0.062, 0.016, 3.0, q) for p, q in fork_sections()], STEEL, 0.2), "bike-metal", "fork"))
    head_lo = B.steer(fl + 0.045)
    out.append(tag(B.oval_tube("tube-down", "frame", [[head_lo[0] + 0.006, head_lo[1], 0.0], [0.790, 0.585, 0.0], [0.640, 0.430, 0.0], B.seat(0.085)], [0.020, 0.022, 0.022, 0.020], [0.027, 0.028, 0.027, 0.023], STEEL, 0.2), "bike-metal", "tube-down"))
    out.append(tag(B.oval_tube("tube-seat", "frame", [B.seat(-0.020), B.seat(R.POST_LEN + 0.004)], 0.0170, 0.0170, STEEL, 0.2), "bike-metal", "tube-seat"))
    out.append(tag(K.loft("tube-chainstay", "frame", [K.section(p, hw, 0.016, 3.0, K.quat(K.W, K.V)) for p, hw in (([R.BB[0] + 0.010, R.BB[1], 0.0], 0.042), ([0.22, 0.322, 0.0], 0.060), ([-0.008, 0.340, 0.0], 0.074))], STEEL, 0.2), "bike-metal", "tube-chainstay"))
    out.append(tag(K.loft("tube-seatstay", "frame", [K.section(p, hw, 0.016, 3.0, q) for p, hw, q in stay_sections()], STEEL, 0.2), "bike-metal", "tube-seatstay"))
    for n, c, a0, sweep in (("front", R.FRONT, -38.0, 205.0), ("rear", R.REAR, 18.0, 207.0)):
        r = R.FENDER_R
        prof = [(-0.030, r - 0.018), (0.0, r + 0.012), (0.030, r - 0.018), (-0.030, r - 0.018)]
        sl = K.revolve(f"{n}-fender", "frame", prof, [c[0], c[1], 0.0], K.W, STEEL, 0.3, sweepRad=round(math.radians(sweep), 6))
        ca, sa = math.cos(math.radians(a0)), math.sin(math.radians(a0))
        sl["transform"] = {"rotation": K.rot_w(a0), "translation": [K.r5(c[0] - (c[0] * ca - c[1] * sa)), K.r5(c[1] - (c[0] * sa + c[1] * ca)), 0.0]}
        out.append(tag(sl, "bike-metal", f"{n}-fender"))
    y = R.CARRIER_Y - 0.025
    out.append(tag(K.box("carrier", "frame", [0.398, 0.030, 0.150], [0.084, y - 0.010, 0.0], STEEL, 0.25, corner=0.008), "bike-metal", "carrier"))
    for s, n in SIDES:
        out.append(tag(B.path_tube(f"carrier-leg-{n}", "frame", [[-0.060, y - 0.020, s * 0.068], [-0.020, 0.560, s * 0.073], [0.006, 0.352, s * 0.071]], 0.007, 0.007, STEEL, 0.25, slender=True), "bike-metal", "carrier"))
    out.append(tag(K.revolve("chainring", "crank-r", [(-0.075, 0.0), (-0.075, 0.104), (-0.045, 0.104), (-0.045, 0.0)], [0, 0, 0], K.W, STEEL, 0.4), "bike-metal", "chainring"))
    for s, n in SIDES:
        out.append(tag(K.extrude(f"crank-arm-{n}", f"crank-{n}", [(-0.015, -0.015), (-0.015, 0.015), (0.185, 0.015), (0.185, -0.015)], "xy", s * R.CRANK_Z - 0.015, s * R.CRANK_Z + 0.015, STEEL, 0.35 if n == "r" else 0.3), "bike-metal", f"crank-arm-{n}"))
        z0, z1 = sorted((s * (R.CRANK_Z + 0.010), s * R.PEDAL_Z1))
        out.append(tag(K.extrude(f"pedal-{n}", f"pedal-{n}", [(-0.052, -0.015), (-0.052, 0.015), (0.052, 0.015), (0.052, -0.015)], "xy", z0, z1, STEEL, 0.15), "bike-metal", f"pedal-{n}"))
    out.append(tag(K.loft("saddle", "frame", [K.section([x, top - h, 0.0], hw, h, 2.4, K.ACROSS) for x, hw, top, h in ((0.160, 0.080, 0.904, 0.022), (0.210, 0.102, 0.924, 0.032), (0.330, 0.042, 0.916, 0.024), (0.425, 0.016, 0.903, 0.016))], SADDLE, 0.15), "saddle", "saddle"))
    out.append(tag(K.ellipsoid("headlamp", "frame", [0.032, 0.024, 0.024], [R.CROWN[0] + 0.044, R.CROWN[1] + 0.018, 0.0], STEEL, 0.7, role="detail"), "bike-metal", "headlamp"))
    rc = (R.WHEEL_R + R.RIM_IN) / 2
    hh = (R.WHEEL_R - R.RIM_IN) / 2
    wheel = [(-0.020, rc - hh), (-0.020, rc + hh - 0.012), (-0.010, R.WHEEL_R), (0.010, R.WHEEL_R), (0.020, rc + hh - 0.012), (0.020, rc - hh), (-0.020, rc - hh)]
    for n in ("rear", "front"):
        b = f"{n}-wheel"
        out.append(tag(K.revolve(f"{n}-tyre", b, wheel, [0, 0, 0], K.W, RUBBER, 0.12), "tyres", f"{n}-tyre"))
        out.append(tag(K.revolve(f"{n}-hub", b, [(-0.060, 0.0), (-0.060, 0.016), (-0.040, 0.016), (-0.040, 0.038), (0.040, 0.038), (0.040, 0.016), (0.060, 0.016), (0.060, 0.0)], [0, 0, 0], K.W, STEEL, 0.6), "bike-metal", f"{n}-hub"))
        sp = K.capsule(f"{n}-spokes", b, [0.036, 0.0, 0.0], [R.RIM_IN + 0.002, 0.0, 0.0], 0.001, 0.001, STEEL, 0.6, role="texture", slender=True, texture=True)
        sp["repeat"] = {"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 6, 7)}
        out.append(tag(sp, "bike-metal", f"{n}-spoke"))


def fork_sections():
    fl = R.FORK_LEN
    pts = [B.steer(fl + 0.004), [0.927, 0.560, 0.0], [0.966, 0.452, 0.0], [R.FRONT[0], R.FRONT[1] - 0.010, 0.0]]
    out = []
    for k, p in enumerate(pts):
        a, b = pts[max(0, k - 1)], pts[min(len(pts) - 1, k + 1)]
        t = K.unit([b[q] - a[q] for q in range(3)])
        out.append((p, K.quat(K.W, [-t[1], t[0], 0.0])))
    return out


def stay_sections():
    top = B.seat(R.ST_LEN - 0.040)
    pts = [(top, 0.024), ([0.12, 0.520, 0.0], 0.052), ([-0.004, 0.348, 0.0], 0.074)]
    out = []
    for k, (p, hw) in enumerate(pts):
        a, b = pts[max(0, k - 1)][0], pts[min(len(pts) - 1, k + 1)][0]
        t = K.unit([b[q] - a[q] for q in range(3)])
        out.append((p, hw, K.quat(K.W, [-t[1], t[0], 0.0])))
    return out


def body200():
    F = B.posed()
    out = []
    rider200(out, F)
    bike200(out)
    return out


def rider500(out):
    out.append(tag(K.stack("pelvis-trousers", "pelvis", [(-0.105, 0.040, 0.060, 0.080, 2.1), (-0.030, 0.076, 0.114, 0.166, 2.3), (0.090, 0.076, 0.080, 0.144, 2.2)], TROUSER, 0.2), "trousers", "pelvis-trousers"))
    out.append(tag(K.stack("jacket", "torso", [(-0.115, 0.146, 0.104, 0.190, 2.3), (0.08, 0.112, 0.050, 0.136, 2.3), (0.26, 0.168, 0.070, 0.158, 2.35), (0.410, 0.094, 0.070, 0.198, 2.5), (0.438, 0.070, 0.060, 0.170, 2.4), (0.466, 0.048, 0.045, 0.070, 2.0)], JACKET, 0.3), "jacket", "jacket"))
    out.append(tag(K.stack("neck", "neck", [(-0.03, 0.084, 0.054, 0.070), (0.13, 0.072, 0.022, 0.052)], SKIN, 0.84), "face-skin", "neck"))
    out.append(tag(K.loft("head-helmet", "head", [K.section([u, (b - f) / 2, 0.0], hw, (f + b) / 2, e, K.ACROSS) for u, f, b, hw, e in ((-0.080, 0.096, -0.040, 0.038, 2.0), (-0.030, 0.112, 0.040, 0.064, 2.1), (0.050, 0.126, 0.118, 0.100, 2.2), (0.120, 0.098, 0.100, 0.086, 2.0), (0.190, 0.030, 0.030, 0.028, 2.0))], HELMET, 0.82), "helmet", "head-helmet"))
    for s, n in SIDES:
        tone = TONE[f"jacket-{n}"]
        out.append(tag(K.limb(f"sleeve-{n}", f"arm-{n}", [(-0.040, 0.0, -0.020, 0.038, 0.038), (0.04, 0.004, 0.004, 0.050, 0.047), (0.355, 0.0, 0.0, 0.040, 0.040)], JACKET, tone, s), "jacket", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", [(-0.045, 0.0, 0.0, 0.039, 0.039), (0.258, 0.0, 0.0, 0.038, 0.038)], JACKET, tone, s), "jacket", f"forearm-{n}"))
        out.append(tag(K.limb(f"glove-{n}", f"hand-{n}", [(-0.020, 0.0, 0.0, 0.038, 0.038, 2.2), (0.110, -0.020, 0.0, 0.038, 0.040, 2.2)], GLOVE, TONE[f"glove-{n}"], s), "gloves", f"glove-{n}"))
        t = TONE[f"trouser-{n}"]
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", [(-0.075, 0.0, -0.01, 0.060, 0.058), (0.02, 0.010, 0.010, 0.088, 0.084), (0.470, 0.004, 0.0, 0.050, 0.050)], TROUSER, t, s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", [(-0.050, 0.004, 0.0, 0.042, 0.042), (0.11, -0.014, -0.002, 0.057, 0.051), (0.445, 0.002, 0.0, 0.038, 0.038)], TROUSER, t, s), "trousers", f"shank-{n}"))
        out.append(tag(K.loft(f"shoe-rear-{n}", f"foot-{n}", low_shoe([(-0.066, -0.012, 2.2), (0.060, -0.012, 2.5), (0.205, -0.014, 2.0)], [(-0.066, 0.038, 0.038), (0.205, 0.040, 0.040)], s, B.foot_local, 0.0), SHOE, TONE[f"shoe-{n}"]), "shoes", f"shoe-rear-{n}"))


def bike500(out):
    fl = R.FORK_LEN
    st = R.STEERER_TOP
    out.append(tag(B.path_tube("fork", "frame", [[R.FRONT[0], R.FRONT[1], 0.0], [0.966, 0.452, 0.0], [0.927, 0.560, 0.0], B.steer(fl), B.steer(fl + 0.012 + R.HT_LEN), [st[0], st[1], 0.0], [R.CLAMP[0], R.CLAMP[1], 0.0]], 0.0165, 0.0165, STEEL, 0.2, slender=True), "bike-metal", "fork-bars"))
    out.append(tag(B.path_tube("handlebar", "frame", [[R.GRIP[0] - 0.052, R.GRIP[1], 0.316], [R.GRIP[0], R.GRIP[1], 0.250], [R.CLAMP[0], R.CLAMP[1], 0.110], [R.CLAMP[0], R.CLAMP[1], -0.110], [R.GRIP[0], R.GRIP[1], -0.250], [R.GRIP[0] - 0.052, R.GRIP[1], -0.316]], 0.0150, 0.0150, STEEL, 0.2, hint=K.V, slender=True), "bike-metal", "fork-bars"))
    head_lo = B.steer(fl + 0.045)
    out.append(tag(B.path_tube("tube-down", "frame", [[head_lo[0], head_lo[1], 0.0], [0.790, 0.585, 0.0], [0.640, 0.430, 0.0], [0.520, 0.380, 0.0], B.seat(0.020), B.seat(0.300), B.seat(R.POST_LEN + 0.004)], 0.021, 0.016, STEEL, 0.2, slender=True), "bike-metal", "main-triangle"))
    out.append(tag(B.path_tube("rear-stays", "frame", [B.seat(-0.010), [0.0, R.REAR[1], 0.0], B.seat(R.ST_LEN - 0.040)], 0.014, 0.014, STEEL, 0.2, slender=True), "bike-metal", "rear-triangle"))
    out.append(tag(K.loft("saddle", "frame", [K.section([x, 0.920 - 0.0375, 0.0], hw, 0.0375, 2.4, K.ACROSS) for x, hw in ((0.165, 0.080), (0.250, 0.090), (0.425, 0.040))], SADDLE, 0.15), "saddle", "saddle"))
    for n, c, a0, sweep in (("front", R.FRONT, -38.0, 205.0), ("rear", R.REAR, 18.0, 207.0)):
        r = R.FENDER_R
        sl = K.revolve(f"{n}-fender", "frame", [(-0.008, r - 0.006), (0.008, r - 0.006), (0.008, r + 0.006), (-0.008, r + 0.006), (-0.008, r - 0.006)], [c[0], c[1], 0.0], K.W, STEEL, 0.3, sweepRad=round(math.radians(sweep), 6), slender=True)
        ca, sa = math.cos(math.radians(a0)), math.sin(math.radians(a0))
        sl["transform"] = {"rotation": K.rot_w(a0), "translation": [K.r5(c[0] - (c[0] * ca - c[1] * sa)), K.r5(c[1] - (c[0] * sa + c[1] * ca)), 0.0]}
        out.append(tag(sl, "bike-metal", f"{n}-fender"))
    for n in ("rear", "front"):
        tyre = [(0.020 * math.sin(2 * math.pi * k / 8), R.WHEEL_R - 0.020 + 0.020 * math.cos(2 * math.pi * k / 8)) for k in range(8)]
        tyre.append(tyre[0])
        out.append(tag(K.revolve(f"{n}-tyre", f"{n}-wheel", tyre, [0, 0, 0], K.W, RUBBER, 0.12, slender=True), "tyres", f"{n}-tyre"))


def body500():
    out = []
    rider500(out)
    bike500(out)
    return out
