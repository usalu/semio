"""🧱 Round 3 bodies of humans/child-on-kick-scooter at 1:100, 1:200 and 1:500: each scale is its own abstraction (merged masses, fewer sections, assemblies generalised to what 15 / 30 / 75 mm can show), stated by hand; nothing is filtered from the 1:50 list."""
import importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-child-on-kick-scooter-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
K = mod("kit")
SKIN, HAIR, HELMET, JACKET, LEGGINGS, SNEAKER, PACK = "skin-child", "hair-blond", "helmet-eps", "zip-jacket", "leggings-knit", "sneaker-textile", "nylon-backpack"
DECK, STEM, GRIP, WHEEL = "alu-deck", "alu-stem", "rubber-grip", "pu-wheel"
AX = [R.AXIS[0], R.AXIS[1], 0.0]
O = [R.HT_BOTTOM[0], R.HT_BOTTOM[1], 0.0]
BAR = [R.BAR_X, R.BAR_Y, 0.0]
SIDES = ((1, "l"), (-1, "r"))


def tag(s, region, g):
    s["_region"] = region
    s["_g"] = g
    return s


def helmet(rows, tilt=12.0, c0=(0.040, -0.005)):
    t = math.radians(tilt)
    n = [math.cos(t), math.sin(t), 0.0]
    d = [math.sin(t), -math.cos(t), 0.0]
    q = K.quat(K.W, d)
    return [K.section([c0[0] + s * n[0], c0[1] + s * n[1], 0], hw, hd, e, q) for s, hd, hw, e in rows]


def shoe(rows, side):
    a = math.radians(R.FOOT_DIR)
    q = K.quat(K.W, [math.sin(a), math.cos(a), 0.0])
    secs = []
    for x, y0, y1, hw, wc, e in rows:
        u, v = R.foot_local(x, (y0 + y1) / 2)
        secs.append(K.section([u, v, side * wc], hw, (y1 - y0) / 2, e, q))
    return secs


def grip_roll(i, bone, w0, w1, r, end, material, tone):
    cu, cv = R.GRIP_LOCAL
    rr = R.GRIP_R + r
    secs = []
    for ph in (0, 45, 90, 135, end):
        a = math.radians(ph)
        secs.append(K.section([cu + rr * math.sin(a), cv + rr * math.cos(a), (w0 + w1) / 2], abs(w1 - w0) / 2, r, 3.0, K.quat(K.W, [math.sin(a), math.cos(a), 0.0])))
    return K.loft(i, bone, secs, material, tone)


def body100():
    o = []
    for s, n in SIDES:
        o.append(tag(K.limb(f"thigh-g-{n}", f"thigh-{n}", [(-0.055, -0.004, 0.0, 0.030, 0.026), (-0.015, -0.006, 0.001, 0.063, 0.050), (0.040, -0.003, 0.002, 0.066, 0.056), (0.150, 0.003, 0.0, 0.055, 0.051), (0.270, 0.005, 0.0, 0.045, 0.045), (0.310, 0.003, 0.0, 0.043, 0.044), (0.338, 0.0, 0.0, 0.024, 0.024)], LEGGINGS, 0.25, s), "leggings", f"thigh-{n}"))
        o.append(tag(K.limb(f"shank-g-{n}", f"shank-{n}", [(-0.034, 0.002, 0.0, 0.019, 0.019), (0.0, 0.004, 0.0, 0.0395, 0.0405), (0.030, 0.002, 0.0, 0.0405, 0.0415), (0.085, -0.005, 0.0, 0.043, 0.040), (0.180, -0.002, 0.0, 0.033, 0.031), (0.262, 0.0, 0.0, 0.027, 0.0265), (0.2934, 0.0, 0.0, 0.0285, 0.029), (0.308, 0.0, 0.0, 0.022, 0.022)], LEGGINGS, 0.25, s), "leggings", f"shank-{n}"))
        o.append(tag(K.loft(f"shoe-rear-{n}", f"foot-{n}", shoe([(-0.061, -0.060, -0.020, 0.014, 0.0, 2.6), (-0.048, -0.063, -0.004, 0.027, 0.001, 3.0), (-0.010, -0.063, 0.005, 0.031, 0.0015, 3.0), (0.030, -0.063, 0.007, 0.033, 0.0035, 3.0), (0.065, -0.063, -0.007, 0.038, 0.0045, 3.0), (0.100, -0.063, -0.021, 0.0415, 0.0035, 3.0)], s), SNEAKER, 0.85), "sneakers", f"shoe-rear-{n}"))
        o.append(tag(K.loft(f"shoe-front-{n}", f"toe-{n}", [K.section([u, (y0 + y1) / 2, s * wc], hw, (y1 - y0) / 2, e, K.quat(K.W, K.V)) for u, y0, y1, hw, wc, e in [(-0.008, 0.0, 0.042, 0.0415, 0.0035, 3.0), (0.022, 0.0, 0.039, 0.039, 0.002, 3.0), (0.046, 0.0, 0.031, 0.031, -0.001, 2.6), (0.060, 0.001, 0.022, 0.019, -0.003, 2.2), (0.065, 0.003, 0.016, 0.009, -0.003, 2.0)]], SNEAKER, 0.85), "sneakers", f"shoe-front-{n}"))
        o.append(tag(K.limb(f"sleeve-up-{n}", f"arm-{n}", [(-0.050, 0.0, -0.003, 0.020, 0.018), (-0.020, 0.0, 0.004, 0.044, 0.040), (0.015, 0.0, 0.006, 0.047, 0.044), (0.110, 0.002, 0.002, 0.040, 0.038), (0.2286, -0.002, 0.0, 0.036, 0.0355), (0.258, -0.002, 0.0, 0.022, 0.022)], JACKET, 0.66, s), "jacket", f"sleeve-up-{n}"))
        o.append(tag(K.limb(f"sleeve-fore-{n}", f"forearm-{n}", [(-0.030, 0.0, 0.0, 0.017, 0.017), (0.0, -0.002, 0.0, 0.0325, 0.0315), (0.045, 0.002, 0.0, 0.0355, 0.0345), (0.130, 0.0, 0.0, 0.028, 0.030), (0.165, 0.0, 0.0, 0.026, 0.030), (0.176, 0.0, 0.0, 0.025, 0.029)], JACKET, 0.66, s), "jacket", f"sleeve-fore-{n}"))
        o.append(tag(K.limb(f"palm-{n}", f"hand-{n}", [(-0.010, 0.001, 0.0, 0.0115, 0.022, 2.4), (0.030, 0.0005, 0.0, 0.0125, 0.030, 2.6), (0.064, 0.0, 0.0, 0.0115, 0.0305, 2.6), (0.072, -0.001, 0.0, 0.008, 0.029, 2.6)], SKIN, 0.8, s), "hands", f"hand-{n}"))
        o.append(tag(grip_roll(f"fingers-{n}", f"hand-{n}", -0.029, 0.027, 0.0078, 172, SKIN, 0.8), "hands", f"hand-{n}"))
        o.append(tag(K.tube(f"thumb-{n}", f"hand-{n}", [([0.016, -0.004, -s * 0.026], 0.0095, 0.0095), ([0.036, -0.022, -s * 0.027], 0.0085, 0.0085), ([0.058, -0.047, -s * 0.014], 0.0076, 0.0076)], SKIN, 0.8, hint=K.U), "hands", f"thumb-{n}"))
        o.append(tag(K.ellipsoid(f"ear-{n}", "head", [0.027, 0.015, 0.0078], [0.013, 0.012, s * 0.065], SKIN, 0.78, role="detail"), "face-skin", f"ear-{n}"))
        o.append(tag(K.tube(f"backpack-strap-{n}", "torso", [([u, v, s * w], 0.017, 0.005, 3.0) for u, v, w in [(0.322, 0.094, 0.068), (0.3595, 0.006, 0.074), (0.296, -0.066, 0.081), (0.150, -0.089, 0.096), (0.070, 0.004, 0.113), (0.058, 0.078, 0.108)]], PACK, 0.3, role="detail", texture=True), "backpack", f"backpack-strap-{n}"))
    o.append(tag(K.stack("seat", "pelvis", [(-0.076, 0.026, 0.034, 0.036, 2.2), (-0.045, 0.060, 0.076, 0.098, 2.4), (-0.010, 0.069, 0.084, 0.111, 2.4), (0.050, 0.074, 0.066, 0.100, 2.3)], LEGGINGS, 0.25), "leggings", "seat"))
    o.append(tag(K.stack("skirt", "pelvis", [(-0.016, 0.083, 0.094, 0.122, 2.4), (0.004, 0.084, 0.092, 0.121, 2.4), (0.048, 0.082, 0.074, 0.108, 2.4), (0.078, 0.058, 0.050, 0.080, 2.3)], JACKET, 0.66), "jacket", "hip"))
    o.append(tag(K.stack("torso", "torso", [(-0.045, 0.060, 0.050, 0.085, 2.3), (-0.018, 0.086, 0.077, 0.109, 2.35), (0.060, 0.088, 0.070, 0.104, 2.3), (0.160, 0.083, 0.075, 0.109, 2.3), (0.250, 0.076, 0.082, 0.119, 2.3), (0.296, 0.062, 0.079, 0.122, 2.2), (0.330, 0.047, 0.063, 0.086, 2.1), (0.356, 0.036, 0.048, 0.052, 2.0)], JACKET, 0.66), "jacket", "torso"))
    o.append(tag(K.stack("neck", "neck", [(-0.025, 0.052, 0.036, 0.044), (0.040, 0.057, 0.031, 0.040), (0.100, 0.060, 0.034, 0.041), (0.120, 0.050, 0.030, 0.036)], SKIN, 0.85), "face-skin", "neck"))
    o.append(tag(K.stack("collar", "neck", [(-0.030, 0.052, 0.058, 0.066), (-0.002, 0.064, 0.047, 0.056), (0.030, 0.062, 0.040, 0.049)], JACKET, 0.6), "jacket", "collar"))
    o.append(tag(K.stack("head", "head", [(-0.073, 0.074, -0.046, 0.016, 2.0), (-0.060, 0.086, -0.020, 0.036, 2.2), (-0.032, 0.095, 0.020, 0.052, 2.3), (-0.008, 0.099, 0.052, 0.058, 2.2), (0.030, 0.094, 0.085, 0.066, 2.0), (0.060, 0.096, 0.088, 0.070, 2.0), (0.092, 0.080, 0.076, 0.064, 2.0), (0.114, 0.046, 0.044, 0.040, 2.0), (0.124, 0.010, 0.010, 0.009, 2.0)], SKIN, 0.85), "face-skin", "head"))
    o.append(tag(K.stack("nose", "head", [(0.018, 0.099, -0.082, 0.009), (-0.008, 0.110, -0.084, 0.012), (-0.022, 0.100, -0.084, 0.011)], SKIN, 0.85, role="detail"), "face-skin", "nose"))
    o.append(tag(K.stack("hair-nape", "head", [(-0.020, -0.030, 0.062, 0.050), (0.005, -0.020, 0.086, 0.064), (0.040, -0.018, 0.094, 0.071), (0.055, -0.018, 0.094, 0.072)], HAIR, 0.3), "hair", "hair-nape"))
    o.append(tag(K.tube("ponytail", "head", [([0.022, 0.088, 0], 0.017, 0.017), ([-0.005, 0.110, 0], 0.022, 0.022), ([-0.070, 0.113, 0], 0.020, 0.020), ([-0.125, 0.099, 0], 0.011, 0.011)], HAIR, 0.3), "hair", "hair-tail"))
    o.append(tag(K.loft("helmet", "head", helmet([(0.0, 0.118, 0.093, 2.15), (0.016, 0.121, 0.097, 2.1), (0.050, 0.114, 0.093, 2.0), (0.082, 0.086, 0.071, 2.0), (0.102, 0.044, 0.037, 2.0), (0.109, 0.012, 0.011, 2.0)]), HELMET, 0.55), "helmet", "helmet"))
    o.append(tag(K.stack("backpack", "torso", [(0.028, -0.090, 0.150, 0.088, 3.0), (0.055, -0.074, 0.186, 0.116, 3.3), (0.180, -0.074, 0.194, 0.122, 3.4), (0.290, -0.077, 0.180, 0.114, 3.2), (0.332, -0.086, 0.138, 0.078, 2.6)], PACK, 0.4), "backpack", "backpack"))
    o.append(tag(K.stack("backpack-pocket", "torso", [(0.052, -0.180, 0.204, 0.072, 3.0), (0.070, -0.176, 0.222, 0.085, 3.4), (0.170, -0.176, 0.222, 0.085, 3.4), (0.194, -0.180, 0.204, 0.070, 2.8)], PACK, 0.36), "backpack", "backpack-pocket"))
    o.append(tag(K.extrude("deck", "frame", [(R.DECK_TOP, -R.DECK_HALF), (R.DECK_TOP, R.DECK_HALF), (0.070, R.DECK_HALF), (R.DECK_BOTTOM, 0.047), (R.DECK_BOTTOM, -0.047), (0.070, -R.DECK_HALF)], "yz", R.DECK_X0, R.DECK_X1, DECK, 0.3), "deck", "deck"))
    plate = [(0.110, 0.058), (0.110, 0.084), (0.004, 0.094), (-0.022, 0.082), (-0.022, 0.064), (0.010, 0.056)]
    o.append(tag(K.extrude("strut-l", "frame", plate, "xy", 0.0195, 0.0275, DECK, 0.7), "deck", "strut-l"))
    o.append(tag(K.extrude("strut-r", "frame", plate, "xy", -0.0275, -0.0195, DECK, 0.7), "deck", "strut-r"))
    c = (0.0, R.WHEEL_R)
    o.append(tag(K.extrude("fender", "frame", K.arc(c[0], c[1], R.FENDER_R1, 140, 30, 10) + [(0.096, 0.097), (0.094, 0.088), (0.086, 0.092)] + K.arc(c[0], c[1], R.FENDER_R0, 32, 140, 10), "xy", -0.022, 0.022, DECK, 0.2), "deck", "fender"))
    o.append(tag(K.box("hinge-block", "frame", [0.050, 0.046, 0.058], [0.428, 0.078, 0], STEM, 0.7), "stem", "head-tube"))
    top = [R.stem_point(0.070)[0] + 0.012 * R.AXIS[1], R.stem_point(0.070)[1] - 0.012 * R.AXIS[0]]
    o.append(tag(K.tube("neck-arm", "frame", [([0.430, 0.094, 0], 0.019, 0.017, 3.0), ([0.482, 0.159, 0], 0.018, 0.016, 3.0), ([top[0], top[1], 0], 0.018, 0.017, 3.0)], STEM, 0.7), "stem", "head-tube"))
    o.append(tag(K.revolve("head-tube", "frame", [(R.CROWN_T - 0.012, 0.0), (R.CROWN_T - 0.012, 0.022), (R.HT_T1, 0.022), (R.HT_T1, 0.0)], O, AX, STEM, 0.7), "stem", "head-tube"))
    a0 = R.stem_point(R.CROWN_T)
    blade = [(a0[0] - 0.014, a0[1]), (a0[0] + 0.014, a0[1] + 0.004), (R.WB + 0.013, R.WHEEL_R), (R.WB, R.WHEEL_R - 0.014), (R.WB - 0.014, R.WHEEL_R)]
    o.append(tag(K.extrude("fork-l", "frame", blade, "xy", 0.0195, 0.0275, STEM, 0.7), "stem", "fork-l"))
    o.append(tag(K.extrude("fork-r", "frame", blade, "xy", -0.0275, -0.0195, STEM, 0.7), "stem", "fork-r"))
    o.append(tag(K.revolve("stem-lower", "frame", [(R.HT_T1, 0.0), (R.HT_T1, 0.017), (0.372, 0.017), (0.372, 0.0)], O, AX, STEM, 0.7), "stem", "stem"))
    o.append(tag(K.revolve("clamp", "frame", [(0.362, 0.0), (0.362, 0.0225), (0.394, 0.0225), (0.394, 0.0)], O, AX, STEM, 0.5, role="detail"), "stem", "clamp"))
    o.append(tag(K.revolve("stem-upper", "frame", [(0.394, 0.0), (0.394, 0.0145), (R.BAR_T + 0.012, 0.0145), (R.BAR_T + 0.012, 0.0)], O, AX, STEM, 0.7), "stem", "stem"))
    o.append(tag(K.revolve("handlebar", "frame", [(-0.168, 0.0), (-0.168, 0.011), (0.168, 0.011), (0.168, 0.0)], BAR, K.W, STEM, 0.7), "stem", "handlebar"))
    for s, n in SIDES:
        prof = [(s * 0.062, 0.0), (s * 0.062, 0.0165), (s * 0.172, 0.0165), (s * 0.172, 0.0)]
        o.append(tag(K.revolve(f"grip-{n}", "frame", prof if s > 0 else prof[::-1], BAR, K.W, GRIP, 0.12), "grips", f"grip-{n}"))
    for n in ("rear", "front"):
        b = f"wheel-{n}"
        o.append(tag(K.revolve(f"tyre-{n}", b, [(-0.015, 0.045), (-0.015, 0.064), (-0.009, 0.071), (0.0, R.WHEEL_R), (0.009, 0.071), (0.015, 0.064), (0.015, 0.045), (-0.015, 0.045)], [0, 0, 0], K.W, WHEEL, 0.14), "wheels", f"tyre-{n}"))
        o.append(tag(K.revolve(f"rim-{n}", b, [(-0.010, 0.0), (-0.010, 0.046), (0.010, 0.046), (0.010, 0.0)], [0, 0, 0], K.W, WHEEL, 0.8), "wheels", f"rim-{n}"))
        o.append(tag(K.revolve(f"hub-{n}", b, [(-0.016, 0.0), (-0.016, 0.014), (0.016, 0.014), (0.016, 0.0)], [0, 0, 0], K.W, WHEEL, 0.3, role="detail"), "wheels", f"hub-{n}"))
    return o


def body200():
    o = []
    for s, n in SIDES:
        o.append(tag(K.limb(f"thigh-g-{n}", f"thigh-{n}", [(-0.050, -0.004, 0.0, 0.034, 0.030), (0.010, -0.005, 0.0, 0.062, 0.050), (0.160, 0.003, 0.0, 0.054, 0.050), (0.300, 0.004, 0.0, 0.044, 0.045), (0.336, 0.0, 0.0, 0.026, 0.026)], LEGGINGS, 0.25, s), "leggings", f"thigh-{n}"))
        o.append(tag(K.limb(f"shank-g-{n}", f"shank-{n}", [(-0.032, 0.002, 0.0, 0.021, 0.021), (0.0, 0.004, 0.0, 0.0395, 0.0405), (0.090, -0.005, 0.0, 0.043, 0.040), (0.250, 0.0, 0.0, 0.028, 0.027), (0.305, 0.0, 0.0, 0.024, 0.024)], LEGGINGS, 0.25, s), "leggings", f"shank-{n}"))
        o.append(tag(K.loft(f"shoe-{n}", f"foot-{n}", shoe([(-0.061, -0.060, -0.018, 0.016, 0.0, 2.6), (-0.040, -0.063, 0.003, 0.030, 0.001, 3.0), (0.025, -0.063, 0.007, 0.033, 0.003, 3.0), (0.100, -0.063, -0.021, 0.041, 0.0035, 3.0)], s), SNEAKER, 0.85), "sneakers", f"shoe-{n}"))
        o.append(tag(K.loft(f"toe-{n}", f"toe-{n}", [K.section([u, y1 / 2, s * 0.002], hw, y1 / 2, e, K.quat(K.W, K.V)) for u, y1, hw, e in [(-0.008, 0.042, 0.041, 3.0), (0.030, 0.037, 0.037, 3.0), (0.056, 0.026, 0.024, 2.4), (0.065, 0.016, 0.012, 2.0)]], SNEAKER, 0.85), "sneakers", f"toe-{n}"))
        o.append(tag(K.limb(f"sleeve-up-{n}", f"arm-{n}", [(-0.048, 0.0, -0.002, 0.022, 0.020), (0.0, 0.0, 0.005, 0.047, 0.043), (0.120, 0.002, 0.002, 0.040, 0.038), (0.2286, -0.002, 0.0, 0.036, 0.0355), (0.256, -0.002, 0.0, 0.024, 0.024)], JACKET, 0.66, s), "jacket", f"sleeve-up-{n}"))
        o.append(tag(K.limb(f"sleeve-fore-{n}", f"forearm-{n}", [(-0.028, 0.0, 0.0, 0.018, 0.018), (0.012, 0.0, 0.0, 0.034, 0.033), (0.120, 0.0, 0.0, 0.029, 0.030), (0.176, 0.0, 0.0, 0.025, 0.029)], JACKET, 0.66, s), "jacket", f"sleeve-fore-{n}"))
        o.append(tag(K.limb(f"hand-{n}", f"hand-{n}", [(-0.010, -0.004, 0.0, 0.016, 0.024, 2.4), (0.030, -0.014, 0.0, 0.027, 0.029, 2.6), (0.066, -0.022, 0.0, 0.028, 0.029, 2.6), (0.084, -0.026, 0.0, 0.016, 0.026, 2.4)], SKIN, 0.8, s), "hands", f"hand-{n}"))
    o.append(tag(K.stack("hip", "pelvis", [(-0.076, 0.026, 0.034, 0.036, 2.2), (-0.045, 0.056, 0.074, 0.096, 2.4), (-0.014, 0.080, 0.093, 0.121, 2.4), (-0.002, 0.085, 0.094, 0.123, 2.4), (0.040, 0.084, 0.078, 0.111, 2.4), (0.078, 0.058, 0.050, 0.080, 2.3)], JACKET, 0.66), "jacket", "hip"))
    o.append(tag(K.stack("torso", "torso", [(-0.045, 0.060, 0.050, 0.085, 2.3), (-0.010, 0.087, 0.077, 0.109, 2.35), (0.160, 0.083, 0.075, 0.109, 2.3), (0.270, 0.072, 0.083, 0.121, 2.3), (0.310, 0.056, 0.074, 0.112, 2.2), (0.356, 0.036, 0.048, 0.052, 2.0)], JACKET, 0.66), "jacket", "torso"))
    o.append(tag(K.stack("neck", "neck", [(-0.030, 0.052, 0.058, 0.066), (0.0, 0.064, 0.046, 0.055), (0.035, 0.058, 0.034, 0.042), (0.115, 0.056, 0.032, 0.039)], SKIN, 0.75), "face-skin", "neck"))
    o.append(tag(K.stack("head", "head", [(-0.073, 0.074, -0.046, 0.016, 2.0), (-0.050, 0.092, -0.006, 0.043, 2.3), (-0.010, 0.102, 0.052, 0.058, 2.2), (0.035, 0.095, 0.092, 0.068, 2.0), (0.070, 0.094, 0.090, 0.070, 2.0), (0.110, 0.056, 0.052, 0.046, 2.0), (0.124, 0.010, 0.010, 0.009, 2.0)], SKIN, 0.85), "face-skin", "head"))
    o.append(tag(K.tube("hair-tail", "head", [([0.030, 0.082, 0], 0.020, 0.020), ([-0.005, 0.110, 0], 0.023, 0.023), ([-0.070, 0.113, 0], 0.021, 0.021), ([-0.120, 0.100, 0], 0.015, 0.015)], HAIR, 0.3), "hair", "hair-tail"))
    o.append(tag(K.loft("helmet", "head", helmet([(0.0, 0.119, 0.094, 2.1), (0.020, 0.121, 0.097, 2.1), (0.060, 0.108, 0.089, 2.0), (0.095, 0.062, 0.052, 2.0), (0.109, 0.014, 0.012, 2.0)]), HELMET, 0.55), "helmet", "helmet"))
    o.append(tag(K.stack("backpack", "torso", [(0.028, -0.090, 0.150, 0.088, 3.0), (0.060, -0.074, 0.214, 0.118, 3.3), (0.190, -0.074, 0.218, 0.122, 3.3), (0.290, -0.077, 0.180, 0.114, 3.2), (0.332, -0.086, 0.138, 0.078, 2.6)], PACK, 0.4), "backpack", "backpack"))
    o.append(tag(K.box("deck", "frame", [R.DECK_X1 - 0.02 + 0.025, 0.030, 0.120], [(R.DECK_X1 + 0.02 - 0.025) / 2 + 0.01, 0.070, 0], DECK, 0.3), "deck", "deck"))
    c = (0.0, R.WHEEL_R)
    o.append(tag(K.extrude("fender", "frame", K.arc(c[0], c[1], R.FENDER_R1, 140, 30, 6) + [(0.096, 0.095), (0.088, 0.090)] + K.arc(c[0], c[1], R.FENDER_R0, 32, 140, 6), "xy", -0.022, 0.022, DECK, 0.2), "deck", "fender"))
    o.append(tag(K.revolve("stem", "frame", [(R.CROWN_T - 0.012, 0.0), (R.CROWN_T - 0.012, 0.020), (R.HT_T1, 0.020), (R.HT_T1, 0.017), (R.BAR_T + 0.012, 0.017), (R.BAR_T + 0.012, 0.0)], O, AX, STEM, 0.7), "stem", "stem"))
    a0 = R.stem_point(R.CROWN_T)
    blade = [(a0[0] - 0.016, a0[1]), (a0[0] + 0.016, a0[1] + 0.005), (R.WB + 0.015, R.WHEEL_R), (R.WB, R.WHEEL_R - 0.015), (R.WB - 0.015, R.WHEEL_R)]
    o.append(tag(K.extrude("fork-l", "frame", blade, "xy", 0.016, 0.034, STEM, 0.7), "stem", "fork-l"))
    o.append(tag(K.extrude("fork-r", "frame", blade, "xy", -0.034, -0.016, STEM, 0.7), "stem", "fork-r"))
    top = [R.stem_point(0.070)[0] + 0.012 * R.AXIS[1], R.stem_point(0.070)[1] - 0.012 * R.AXIS[0]]
    o.append(tag(K.tube("neck-arm", "frame", [([0.405, 0.074, 0], 0.024, 0.017, 3.0), ([0.445, 0.106, 0], 0.020, 0.017, 3.0), ([top[0], top[1], 0], 0.018, 0.017, 3.0)], STEM, 0.7), "stem", "head-tube"))
    o.append(tag(K.revolve("handlebar", "frame", [(-0.064, 0.0), (-0.064, 0.015), (0.064, 0.015), (0.064, 0.0)], BAR, K.W, STEM, 0.7), "stem", "handlebar"))
    for s, n in SIDES:
        prof = [(s * 0.060, 0.0), (s * 0.060, 0.0165), (s * 0.176, 0.0165), (s * 0.176, 0.0)]
        o.append(tag(K.revolve(f"grip-{n}", "frame", prof if s > 0 else prof[::-1], BAR, K.W, GRIP, 0.12), "grips", f"grip-{n}"))
    for n in ("rear", "front"):
        o.append(tag(K.revolve(f"tyre-{n}", f"wheel-{n}", [(-0.015, 0.0), (-0.015, 0.066), (-0.007, R.WHEEL_R), (0.007, R.WHEEL_R), (0.015, 0.066), (0.015, 0.0)], [0, 0, 0], K.W, WHEEL, 0.14), "wheels", f"tyre-{n}"))
    return o


def body500():
    o = []
    for s, n in SIDES:
        o.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", [(-0.045, -0.004, 0.0, 0.040, 0.036), (0.020, -0.004, 0.0, 0.061, 0.050), (0.300, 0.004, 0.0, 0.045, 0.045), (0.332, 0.0, 0.0, 0.030, 0.030)], LEGGINGS, 0.25, s), "leggings", f"thigh-{n}"))
        o.append(tag(K.limb(f"shank-{n}", f"shank-{n}", [(-0.030, 0.002, 0.0, 0.030, 0.030), (0.040, 0.0, 0.0, 0.046, 0.046), (0.260, 0.0, 0.0, 0.029, 0.028), (0.300, 0.0, 0.0, 0.026, 0.026)], LEGGINGS, 0.25, s), "leggings", f"shank-{n}"))
        o.append(tag(K.loft(f"shoe-{n}", f"foot-{n}", shoe([(-0.061, -0.062, -0.012, 0.020, 0.0, 2.6), (-0.020, -0.063, 0.0135, 0.032, 0.001, 3.0), (0.095, -0.063, -0.020, 0.041, 0.003, 3.0), (0.140, -0.063, -0.027, 0.036, 0.002, 2.8), (0.158, -0.061, -0.042, 0.018, 0.0, 2.2)], s), SNEAKER, 0.85), "sneakers", f"shoe-{n}"))
        o.append(tag(K.limb(f"upper-arm-{n}", f"arm-{n}", [(-0.045, 0.0, 0.0, 0.026, 0.024), (0.005, 0.0, 0.005, 0.047, 0.043), (0.2286, -0.002, 0.0, 0.036, 0.0355), (0.254, -0.002, 0.0, 0.026, 0.026)], JACKET, 0.66, s), "jacket", f"upper-arm-{n}"))
        o.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", [(-0.026, 0.0, 0.0, 0.028, 0.028), (0.015, 0.0, 0.0, 0.038, 0.038), (0.176, 0.0, 0.0, 0.030, 0.031)], JACKET, 0.66, s), "jacket", f"forearm-{n}"))
        o.append(tag(K.limb(f"hand-{n}", f"hand-{n}", [(-0.010, -0.004, 0.0, 0.022, 0.028, 2.4), (0.045, -0.016, 0.0, 0.038, 0.038, 2.6), (0.084, -0.026, 0.0, 0.022, 0.030, 2.4)], SKIN, 0.8, s), "hands", f"hand-{n}"))
    o.append(tag(K.stack("hip", "pelvis", [(-0.074, 0.030, 0.038, 0.042, 2.2), (-0.042, 0.056, 0.074, 0.096, 2.4), (-0.010, 0.084, 0.094, 0.122, 2.4), (0.078, 0.060, 0.052, 0.082, 2.3)], JACKET, 0.66), "jacket", "hip"))
    o.append(tag(K.stack("torso", "torso", [(-0.045, 0.060, 0.050, 0.085, 2.3), (-0.005, 0.088, 0.077, 0.109, 2.3), (0.270, 0.072, 0.083, 0.121, 2.3), (0.325, 0.050, 0.068, 0.100, 2.1)], JACKET, 0.66), "jacket", "torso"))
    o.append(tag(K.stack("shoulders", "neck", [(-0.040, 0.054, 0.066, 0.100, 2.1), (0.000, 0.064, 0.050, 0.068), (0.040, 0.059, 0.036, 0.045), (0.115, 0.056, 0.034, 0.041)], JACKET, 0.66), "jacket", "shoulders"))
    o.append(tag(K.stack("head", "head", [(-0.073, 0.076, -0.040, 0.026, 2.0), (-0.040, 0.094, 0.014, 0.052, 2.1), (0.035, 0.096, 0.092, 0.068, 2.0), (0.105, 0.064, 0.060, 0.052, 2.0), (0.124, 0.012, 0.012, 0.010, 2.0)], SKIN, 0.85), "face-skin", "head"))
    o.append(tag(K.loft("helmet", "head", helmet([(0.0, 0.120, 0.095, 2.1), (0.035, 0.119, 0.096, 2.0), (0.085, 0.080, 0.066, 2.0), (0.109, 0.016, 0.014, 2.0)]), HELMET, 0.55), "helmet", "helmet"))
    o.append(tag(K.stack("backpack", "torso", [(0.028, -0.086, 0.160, 0.092, 3.0), (0.070, -0.074, 0.216, 0.120, 3.2), (0.280, -0.076, 0.186, 0.116, 3.2), (0.332, -0.086, 0.138, 0.080, 2.6)], PACK, 0.4), "backpack", "backpack"))
    o.append(tag(K.box("deck", "frame", [0.43, 0.030, 0.120], [0.225, 0.070, 0], DECK, 0.3), "deck", "deck"))
    o.append(tag(K.revolve("stem", "frame", [(R.CROWN_T - 0.012, 0.0), (R.CROWN_T - 0.012, 0.016), (R.BAR_T + 0.012, 0.016), (R.BAR_T + 0.012, 0.0)], O, AX, STEM, 0.7, slender=True), "stem", "stem"))
    a0 = R.stem_point(R.CROWN_T)
    o.append(tag(K.extrude("fork", "frame", [(0.405, 0.058), (0.440, 0.085), (a0[0] - 0.016, a0[1] + 0.02), (a0[0] + 0.016, a0[1] + 0.02), (R.WB + 0.015, R.WHEEL_R), (R.WB, R.WHEEL_R - 0.015), (R.WB - 0.015, R.WHEEL_R), (a0[0] - 0.004, a0[1] - 0.032)], "xy", -0.0275, 0.0275, STEM, 0.7), "stem", "fork"))
    o.append(tag(K.revolve("handlebar", "frame", [(-0.172, 0.0), (-0.172, 0.0165), (0.172, 0.0165), (0.172, 0.0)], BAR, K.W, GRIP, 0.3), "grips", "handlebar"))
    for n in ("rear", "front"):
        o.append(tag(K.revolve(f"tyre-{n}", f"wheel-{n}", [(-0.018, 0.0), (-0.018, R.WHEEL_R), (0.018, R.WHEEL_R), (0.018, 0.0)], [0, 0, 0], K.W, WHEEL, 0.14), "wheels", f"tyre-{n}"))
    return o
