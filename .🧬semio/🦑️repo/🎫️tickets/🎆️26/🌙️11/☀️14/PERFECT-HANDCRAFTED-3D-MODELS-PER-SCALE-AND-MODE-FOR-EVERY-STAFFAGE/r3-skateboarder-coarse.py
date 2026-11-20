"""🧱 Round 3 bodies of humans/skateboarder at 1:100 (feature 15 mm), 1:200 (30 mm) and 1:500 (75 mm), each authored for its scale: fewer, generalised sections, parts below the feature merged into their mass (sole into shoe, cuff into sleeve, grip and concave into one deck, fingers into a mitten, hood into the back of the trunk, beanie cuff into the beanie, at 1:500 the whole board one side profile).

`body100()`, `body200()`, `body500()` return solids with the private keys `_region` and `_g` like `r3-skateboarder-body50.build()`.
"""
import importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-skateboarder-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


B = mod("body50")
R, K = B.R, B.K
tag, SIDES, TONE = B.tag, B.SIDES, B.TONE


def pick(rows, us, grow=0.0, floor=0.0):
    """📐 Sections of a hand-stated table at the stations `us`, every radius grown by `grow` and held at least `floor`."""
    out = []
    for u in us:
        r = list(B.row_at(rows, u))
        for k in (3, 4):
            if k < len(r):
                r[k] = max(r[k] + grow, floor)
        out.append(tuple(r))
    return out


def trunk_rows(us, hood_back, floor):
    """👕 Trunk stations with the hood merged into the back reach (`hood_back`: extra reach behind the shoulder blades, peaking at u 0.42)."""
    out = []
    for u in us:
        r = list(B.row_at(B.HOODIE, min(u, B.HOODIE[-1][0])))
        r[0] = u
        r[2] += hood_back * max(0.0, 1 - abs(u - 0.42) / 0.14)
        r[1], r[2], r[3] = max(r[1], floor), max(r[2], floor), max(r[3], floor)
        out.append(tuple(r))
    return out


def head_rows(ys, grow=0.0):
    return [(y, *[q for q in (B.head_row(y)[1] + grow, B.head_row(y)[2] - grow, B.head_row(y)[3] + grow, 2.1)]) for y in ys]


def shoe(out, s, n, den, rows_rear, rows_toe, sole_merge):
    st = TONE["shoe"]
    if sole_merge:
        bottom = -R.ANKLE
        q = K.quat(K.W, [math.sin(math.radians(R.FOOT_DROP)), math.cos(math.radians(R.FOOT_DROP)), 0.0])
        secs = []
        for x, top, e in rows_rear:
            L, M = B.lm(x, B.SOLE)
            hw, wc = (L + M) / 2, s * (L - M) / 2
            u, v = B.foot_local(x, (top + bottom) / 2)
            secs.append(K.section([u, v, wc], hw, (top - bottom) / 2, e, q))
        out.append(tag(K.loft(f"shoe-{n}", f"foot-{n}", secs, B.SHOE, st), "shoes", f"shoe-{n}"))
        if rows_toe:
            secs = []
            for x, top, e in rows_toe:
                L, M = B.lm(x, B.SOLE_TOE)
                hw, wc = (L + M) / 2, s * (L - M) / 2
                u, v = B.toe_local(x, (top + bottom) / 2)
                secs.append(K.section([u, v, wc], hw, (top - bottom) / 2, e, K.ACROSS))
            out.append(tag(K.loft(f"shoe-toe-{n}", f"toe-{n}", secs, B.SHOE, st), "shoes", f"shoe-toe-{n}"))


def deck_flat(i, thick, n=14):
    """🛹 The deck as one plate along the kicked centre curve, `thick` metres thick (grip and concave merged)."""
    pts = B.deck_path(n)
    secs = []
    for k, (x, y) in enumerate(pts):
        a = pts[max(0, k - 1)]
        b = pts[min(len(pts) - 1, k + 1)]
        t = K.unit([b[0] - a[0], b[1] - a[1], 0.0])
        nrm = [-t[1], t[0], 0.0]
        hw = max(R.deck_half_width(x), 0.004) if 0 < k < len(pts) - 1 else 0.004
        c = [x + nrm[0] * (thick / 2 - R.DECK_T / 2 + R.CONCAVE / 4), y + nrm[1] * (thick / 2 - R.DECK_T / 2 + R.CONCAVE / 4), 0.0]
        secs.append(K.section(c, hw, thick / 2, 5.0, K.quat(K.W, nrm)))
    return K.loft(i, "board", secs, B.MAPLE, TONE["maple"])


def wheel_profile(steps):
    return [(-R.WHEEL_W / 2, 0.0), (-R.WHEEL_W / 2, 0.019), (-0.010, 0.0262), (-0.004, R.WHEEL_R), (0.009, R.WHEEL_R), (0.014, 0.024), (R.WHEEL_W / 2, 0.018), (R.WHEEL_W / 2, 0.0)] if steps else [(-R.WHEEL_W / 2, 0.0), (-R.WHEEL_W / 2, 0.0235), (-0.008, R.WHEEL_R), (0.008, R.WHEEL_R), (R.WHEEL_W / 2, 0.0235), (R.WHEEL_W / 2, 0.0)]


def body100():
    out = []
    out.append(tag(K.stack("trousers-seat", "pelvis", pick(B.SEAT, (-0.114, -0.098, -0.060, -0.010, 0.050, 0.100)), B.TWILL, TONE["trousers"]), "trousers", "trousers-seat"))
    out.append(tag(K.stack("hoodie", "torso", pick(B.HOODIE, (-0.088, -0.040, 0.040, 0.160, 0.290, 0.395, 0.440, 0.478, 0.508)), B.FLEECE, TONE["hoodie"]), "hoodie-torso", "hoodie"))
    out.append(tag(K.onto_girdle(K.stack("hood", "torso", [(0.310, -0.092, 0.132, 0.100, 2.4), (0.390, -0.090, 0.144, 0.140, 2.5), (0.470, -0.066, 0.124, 0.128, 2.4), (0.530, -0.020, 0.094, 0.084, 2.0)], B.FLEECE, 0.4), R.GIRDLE, R.GIRDLE_X), "hoodie-torso", "hood"))
    collar = [(0.0195 * math.sin(2 * math.pi * k / 10), 0.073 + 0.0195 * math.cos(2 * math.pi * k / 10)) for k in range(10)]
    out.append(tag(K.onto_girdle(K.revolve("hoodie-collar", "torso", collar + [collar[0]], [R.TORSO - 0.002, 0.004, 0.0], [1.0, 0.0, 0.0], B.FLEECE, 0.4), R.GIRDLE, R.GIRDLE_X), "hoodie-torso", "hood"))
    out.append(tag(B.head_stack("neck", [B.NECK[0], B.NECK[2], B.NECK[4]], B.SKIN, TONE["skin"]), "face", "head"))
    out.append(tag(B.head_stack("head", head_rows((0.016, 0.030, 0.062, 0.100, 0.148, 0.190, 0.222, 0.240, 0.2474)), B.SKIN, TONE["skin"]), "face", "head"))
    out.append(tag(K.onto_girdle(K.tube("nose", "head", [(B.hp(0.142, 0.160), 0.009, 0.0080), (B.hp(0.110, 0.178), 0.0110, 0.0090), (B.hp(0.092, 0.172), 0.0130, 0.0080)], B.SKIN, TONE["skin"], hint=K.W, role="detail"), R.GAZE, R.GAZE_X), "face", "head"))
    rows = B.beanie_rows()
    out.append(tag(K.loft("beanie", "head", B.beanie_sections([rows[k] for k in (0, 2, 4, 6, 8, 10, 11)], 0.003), B.KNIT, TONE["beanie"]), "beanie", "beanie"))
    for s, n in SIDES:
        out.append(tag(K.limb(f"sleeve-{n}", f"upper-arm-{n}", pick(B.UPPER, (-0.078, -0.052, 0.010, 0.120, 0.260, 0.352, 0.372)), B.FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"sleeve-{n}"))
        fore = pick(B.FORE, (-0.045, -0.015, 0.060, 0.150, 0.216)) + [(0.236, 0.0, 0.0, 0.034, 0.029), (0.250, 0.0, 0.0, 0.031, 0.026)]
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", fore, B.FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"forearm-{n}"))
        medial = -s
        mitt = [(-0.020, 0.002, 0.0, 0.027, 0.018, 2.2), (0.030, 0.004, 0.0, 0.040, 0.016, 2.6), (0.090, 0.002, medial * 0.004, 0.043, 0.0155, 2.6), (0.140, 0.0, medial * 0.020, 0.038, 0.0135, 2.4), (0.170, -0.002, medial * 0.034, 0.028, 0.010, 2.2), (0.182, -0.002, medial * 0.040, 0.014, 0.008, 2.0)]
        out.append(tag(K.limb(f"hand-{n}", f"hand-{n}", [(u, v, w * s, rv, rw, e) for u, v, w, rv, rw, e in mitt], B.SKIN, TONE["skin"], s), "hands", f"hand-{n}"))
        out.append(tag(K.tube(f"thumb-{n}", f"hand-{n}", [([0.012, 0.028, medial * 0.004], 0.0115, 0.0110), ([0.060, 0.044, medial * 0.016], 0.0095, 0.0090), ([0.100, 0.036, medial * 0.026], 0.0080, 0.0076)], B.SKIN, TONE["skin"], hint=K.W, role="detail"), "hands", f"hand-{n}"))
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", pick(B.THIGH, (-0.110, -0.050, 0.030, 0.200, 0.350, 0.440, 0.470, 0.492)), B.TWILL, TONE["trousers"], s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", pick(B.SHANK, (-0.050, -0.010, 0.120, 0.280, 0.404, 0.428, 0.438)), B.TWILL, TONE["trousers"], s), "trousers", f"shank-{n}"))
        shoe(out, s, n, 100, [B.UPPER_SHOE[k] for k in (0, 1, 3, 4, 6, 8)], [B.TOE_BOX[k] for k in (0, 2, 4, 5)], True)
    out.append(tag(deck_flat("deck", 0.015), "deck", "deck"))
    for tx, tn in ((R.AXLE_X, "f"), (-R.AXLE_X, "r")):
        o = 1 if tx > 0 else -1
        ay = -R.DECK_Y + R.WHEEL_R
        y0 = -R.DECK_T / 2
        hanger = [K.section([xc, yc, z], rx, ry, 2.4, K.quat(K.U, K.V)) for z, rx, ry, yc, xc in ((-R.AXLE_HALF, 0.0076, 0.0076, ay, tx), (-0.066, 0.0085, 0.0085, ay, tx), (-0.030, 0.0125, 0.0135, ay + 0.006, tx + o * 0.002), (0.0, 0.016, 0.0175, ay + 0.0095, tx + o * 0.003), (0.030, 0.0125, 0.0135, ay + 0.006, tx + o * 0.002), (0.066, 0.0085, 0.0085, ay, tx), (R.AXLE_HALF, 0.0076, 0.0076, ay, tx))]
        out.append(tag(K.loft(f"hanger-{tn}", "board", hanger, B.ALU, TONE["alu"]), "trucks", f"truck-{tn}"))
        out.append(tag(K.box(f"baseplate-{tn}", "board", [0.068, 0.0155, 0.056], [tx + o * 0.006, y0 - 0.0077, 0.0], B.ALU, TONE["alu"], corner=0.004), "trucks", f"truck-{tn}"))
        out.append(tag(K.capsule(f"bushings-{tn}", "board", [tx - o * 0.016, y0 - 0.012, 0.0], [tx - o * 0.006, ay + 0.004, 0.0], 0.0135, 0.0125, B.PU, TONE["bushing"]), "trucks", f"truck-{tn}"))
    for b, tn in (("wheel-f", "f"), ("wheel-r", "r")):
        for s, n in SIDES:
            out.append(tag(K.revolve(f"wheel-{tn}-{n}", b, [(a if s > 0 else -a, r) for a, r in wheel_profile(True)], [0.0, 0.0, s * R.WHEEL_Z], [0.0, 0.0, 1.0], B.PU, TONE["wheel"]), "wheels", f"wheel-{tn}"))
    return out


def body200():
    out = []
    out.append(tag(K.stack("trousers-seat", "pelvis", pick(B.SEAT, (-0.114, -0.075, 0.000, 0.100)), B.TWILL, TONE["trousers"]), "trousers", "trousers-seat"))
    out.append(tag(K.stack("hoodie", "torso", trunk_rows((-0.088, 0.040, 0.230, 0.345, 0.430, 0.478, 0.530), 0.030, 0.030), B.FLEECE, TONE["hoodie"]), "hoodie-torso", "hoodie"))
    neckhead = [(-0.040, 0.078, -0.044, 0.064, 2.0), (0.040, 0.100, -0.030, 0.060, 2.0), (0.060, 0.160, -0.026, 0.064, 2.1), (0.120, 0.172, -0.046, 0.074, 2.1), (0.176, 0.162, -0.051, 0.078, 2.1), (0.222, 0.126, -0.036, 0.066, 2.0), (0.2474, 0.052, 0.016, 0.016, 2.0)]
    out.append(tag(B.head_stack("head", neckhead, B.SKIN, TONE["skin"]), "face", "head"))
    rows = B.beanie_rows()
    out.append(tag(K.loft("beanie", "head", B.beanie_sections([rows[k] for k in (0, 3, 6, 9, 11)], 0.004), B.KNIT, TONE["beanie"]), "beanie", "beanie"))
    for s, n in SIDES:
        out.append(tag(K.limb(f"sleeve-{n}", f"upper-arm-{n}", pick(B.UPPER, (-0.078, -0.030, 0.120, 0.320, 0.372)), B.FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", pick(B.FORE, (-0.045, 0.030, 0.150)) + [(0.250, 0.0, 0.0, 0.034, 0.029)], B.FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"forearm-{n}"))
        medial = -s
        mitt = [(-0.020, 0.004, 0.0, 0.028, 0.0165, 2.2), (0.060, 0.008, medial * 0.002, 0.046, 0.0165, 2.6), (0.150, 0.004, medial * 0.024, 0.036, 0.0155, 2.4), (0.180, 0.0, medial * 0.036, 0.018, 0.0155, 2.0)]
        out.append(tag(K.limb(f"hand-{n}", f"hand-{n}", [(u, v, w * s, rv, rw, e) for u, v, w, rv, rw, e in mitt], B.SKIN, TONE["skin"], s), "hands", f"hand-{n}"))
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", pick(B.THIGH, (-0.110, 0.030, 0.280, 0.440, 0.492)), B.TWILL, TONE["trousers"], s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", pick(B.SHANK, (-0.050, 0.120, 0.350, 0.438)), B.TWILL, TONE["trousers"], s), "trousers", f"shank-{n}"))
        shoe(out, s, n, 200, [(-0.064, -0.016, 2.2), (-0.010, 0.002, 2.6), (0.080, -0.008, 2.6), (0.150, -0.026, 2.6)], [(0.135, -0.024, 2.6), (0.200, -0.038, 2.3), (0.226, -0.050, 2.0)], True)
    out.append(tag(deck_flat("deck", 0.030, 10), "deck", "deck"))
    for tx, tn in ((R.AXLE_X, "f"), (-R.AXLE_X, "r")):
        o = 1 if tx > 0 else -1
        ay = -R.DECK_Y + R.WHEEL_R
        out.append(tag(K.extrude(f"truck-{tn}", "board", [(tx - o * 0.024, -0.018), (tx + o * 0.040, -0.018), (tx + o * 0.012, ay + 0.004), (tx - o * 0.010, ay + 0.004)][:: o], "xy", -0.066, 0.066, B.ALU, TONE["alu"]), "trucks", f"truck-{tn}"))
    for b, tn in (("wheel-f", "f"), ("wheel-r", "r")):
        for s, n in SIDES:
            out.append(tag(K.revolve(f"wheel-{tn}-{n}", b, [(a if s > 0 else -a, r) for a, r in wheel_profile(False)], [0.0, 0.0, s * R.WHEEL_Z], [0.0, 0.0, 1.0], B.PU, TONE["wheel"]), "wheels", f"wheel-{tn}"))
    return out


def body500():
    out = []
    out.append(tag(K.stack("trousers-seat", "pelvis", [(-0.114, 0.045, 0.060, 0.090, 2.0), (-0.040, 0.088, 0.134, 0.180, 2.4), (0.100, 0.100, 0.112, 0.168, 2.4)], B.TWILL, TONE["trousers"]), "trousers", "trousers-seat"))
    out.append(tag(K.stack("hoodie", "torso", [(-0.088, 0.100, 0.082, 0.178, 2.5), (0.160, 0.136, 0.098, 0.178, 2.4), (0.360, 0.134, 0.150, 0.200, 2.6), (0.470, 0.084, 0.110, 0.150, 2.3), (0.530, 0.050, 0.060, 0.070, 2.0)], B.FLEECE, TONE["hoodie"]), "hoodie-torso", "hoodie"))
    out.append(tag(B.head_stack("head", [(-0.040, 0.080, -0.044, 0.064, 2.0), (0.050, 0.160, -0.030, 0.064, 2.0), (0.130, 0.172, -0.046, 0.074, 2.1), (0.190, 0.156, -0.050, 0.076, 2.1)], B.SKIN, TONE["skin"]), "face", "head"))
    rows = B.beanie_rows()
    out.append(tag(K.loft("beanie", "head", B.beanie_sections([rows[k] for k in (0, 5, 11)], 0.006), B.KNIT, TONE["beanie"]), "beanie", "beanie"))
    for s, n in SIDES:
        out.append(tag(K.limb(f"sleeve-{n}", f"upper-arm-{n}", [(-0.078, 0.0, -0.016, 0.040, 0.040), (0.010, 0.002, 0.004, 0.058, 0.060), (0.372, -0.004, 0.0, 0.040, 0.040)], B.FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"sleeve-{n}"))
        out.append(tag(K.limb(f"forearm-{n}", f"forearm-{n}", [(-0.045, -0.004, 0.0, 0.040, 0.040), (0.030, -0.002, 0.002, 0.049, 0.050), (0.262, 0.0, 0.0, 0.0385, 0.0385)], B.FLEECE, TONE["hoodie"], s), "hoodie-sleeves", f"forearm-{n}"))
        out.append(tag(K.ellipsoid(f"hand-{n}", f"hand-{n}", [0.075, 0.046, 0.038], [0.065, 0.004, -s * 0.010], B.SKIN, TONE["skin"]), "hands", f"hand-{n}"))
        out.append(tag(K.limb(f"thigh-{n}", f"thigh-{n}", [(-0.110, 0.0, 0.0, 0.072, 0.072), (0.030, 0.012, 0.012, 0.094, 0.095), (0.492, 0.0, 0.0, 0.050, 0.050)], B.TWILL, TONE["trousers"], s), "trousers", f"thigh-{n}"))
        out.append(tag(K.limb(f"shank-{n}", f"shank-{n}", [(-0.050, 0.004, 0.0, 0.052, 0.052), (0.120, -0.010, -0.002, 0.066, 0.059), (0.438, 0.008, 0.0, 0.062, 0.060)], B.TWILL, TONE["trousers"], s), "trousers", f"shank-{n}"))
        shoe(out, s, n, 500, [(-0.062, -0.010, 2.2), (0.030, 0.000, 2.6), (0.150, -0.020, 2.6), (0.205, -0.030, 2.2)], [], True)
    slab = [(-0.401, 0.050, 0.030, 0.030), (-0.385, 0.045, 0.012, 0.062), (-0.340, 0.030, -0.012, 0.094), (-0.290, 0.012, -0.050, 0.100), (-0.235, 0.005, -0.085, 0.1015), (0.235, 0.005, -0.085, 0.1015), (0.290, 0.012, -0.050, 0.100), (0.340, 0.032, -0.012, 0.094), (0.388, 0.049, 0.014, 0.062), (0.405, 0.055, 0.034, 0.030)]
    out.append(tag(K.loft("board", "board", [K.section([x, (top + bottom) / 2, 0.0], hw, (top - bottom) / 2, 5.0, K.quat(K.W, K.V)) for x, top, bottom, hw in slab], B.MAPLE, TONE["maple"]), "deck", "deck"))
    return out


if __name__ == "__main__":
    for f in (body100, body200, body500):
        print(f.__name__, len(f()))
