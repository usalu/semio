"""🧩 Round 5 coarse lists (round 3 lists on the corrected 1.40 m track; 1:500 underbody band down to the axle line). 🚙️ The 1:100, 1:200 and 1:500 lists of the round 3 vintage saloon: each scale restated by hand with merged masses and simplified sections. Executed by r3-vintage-car-1929.py after the kit and the 1:50 part."""


def both(d, tag_):
    res = pair(d)
    for x in res:
        x["id"] = x["id"][:-2] + "-" + tag_ + x["id"][-1]
    return res


def tref(s, i, region, faces_=None, layer=None):
    s.thermal.append(ref("th-" + i, i, region, layer, faces_))


def tilt_side(d):
    return turn(d, Qx(-A_TUM), [0, YBELT, ZB])


def greenhouse(den, glass_t):
    """Frames with openings and glass behind them (1:100, 1:200): side frames, windscreen frame, rear wall."""
    b = []
    fr = ext("cabin-side-l", CH, P, 0.58, "xy", CABIN_LOW, 0.59, ZB, holes=[[list(p) for p in w] for w in WINS])
    b += pair(tilt_side(fr))
    pane = [[0.46, 1.21], [0.43, 1.58], [-1.70, 1.58], [-1.78, 1.21]]
    b += pair(tilt_side(ext("window-band-l", CH, GL, 0.85, "xy", pane, 0.60, 0.60 + glass_t, role="glazing")))
    if den == 200:
        b[0]["id"], b[1]["id"] = "cabin-frame-l", "cabin-frame-r"
    k = (YHEAD - YBELT) / 0.46
    outer = [[YBELT - 0.05, -0.606], [YBELT - 0.05, 0.606], [YHEAD, 0.606 - 0.04 * k], [YHEAD, -0.606 + 0.04 * k]]
    wsh = [[1.215, -0.54], [1.215, 0.54], [1.578, 0.51], [1.578, -0.51]]
    b.append(turn(ext("windscreen-frame", CH, P, 0.58, "yz", outer, 0.485, 0.52, holes=[wsh]), Qz(A_TUM), [0.52, YBELT, 0]))
    b.append(turn(ext("windscreen", CH, GL, 0.85, "yz", grow(wsh, 0.01), 0.497, 0.497 + glass_t, role="glazing"), Qz(A_TUM), [0.52, YBELT, 0]))
    kr = (YHEAD - YBELT) / math.cos(A_REAR)
    outer_r = [[YBELT - 0.02, -0.606], [YBELT - 0.02, 0.606], [YBELT + kr, 0.606 - 0.04 * k], [YBELT + kr, -0.606 + 0.04 * k]]
    rwh = [[1.30, -0.28], [1.30, 0.28], [1.52, 0.28], [1.52, -0.28]]
    b.append(turn(ext("cabin-rear-wall", CH, P, 0.58, "yz", outer_r, -1.90, -1.87, holes=[rwh]), Qz(-A_REAR), [-1.90, YBELT, 0]))
    b.append(turn(ext("window-back", CH, GL, 0.85, "yz", grow(rwh, 0.01), -1.892, -1.892 + glass_t, role="glazing"), Qz(-A_REAR), [-1.90, YBELT, 0]))
    return b


TAIL_IN_N = {100: [[-1.90, 1.135], [-1.94, 1.12], [-1.965, 1.06], [-1.965, 0.72], [-1.93, 0.52]], 200: [[-1.89, 1.13], [-1.96, 1.07], [-1.96, 0.70], [-1.93, 0.52]]}


def tub_shell(den, t):
    """The open tub of 1:100 and 1:200: two side skins with the wheel arches, the turned-under tail panel, the arch closing plates, the floor and the two benches."""
    b = pair(ext("tub-side-l", CH, P, 0.6, "xy", tub_of(den), ZB - t, ZB))
    b.append(ext("tub-rear", CH, P, 0.6, "xy", TAILS[den] + TAIL_IN_N[den], -(ZB - t + 0.002), ZB - t + 0.002))
    wh = arc(XR, RW, ARCH_R + 0.006, 25.5, 154.5, ARCHN[den])
    b += pair(ext("wheelhouse-l", CH, P, 0.3, "xy", wh, ZB - 2 * t, ZB - t + 0.002))
    b.append(rbox("floor", CH, P, 0.3, [2.47, 0.03, 2 * (ZB - t) + 0.004], [-0.725, 0.535, 0], None))
    r = 0.06 if den == 100 else None
    b.append(rbox("seat-front", CH, UP, 0.3, [0.52, 0.56, 1.10], [-0.08, 0.90, 0], r, role="detail"))
    b.append(rbox("seat-rear", CH, UP, 0.3, [0.56, 0.60, 1.10], [-1.42, 0.92, 0], r, role="detail"))
    return b


def frustum(id, tone=0.58):
    """The cabin as one mass between belt and cant rail: raked front and rear, tumblehome sides (1:200, 1:500)."""
    q = Qx(-math.pi / 2)
    x0f, x0r, x1f, x1r = 0.52, -1.90, 0.52 - 0.04 * (YHEAD - YBELT) / 0.46, -1.90 + 0.10 * (YHEAD - YBELT) / 0.46
    k = 0.04 * (YHEAD - YBELT) / 0.46
    return loft(id, CH, P, tone, [sec([(x0f + x0r) / 2, YBELT - 0.02, 0], (x0f - x0r) / 2, ZB, 14, rotation=q), sec([(x0f + x0r) / 2, YBELT, 0], (x0f - x0r) / 2, ZB, 14, rotation=q), sec([(x1f + x1r) / 2, YHEAD, 0], (x1f - x1r) / 2, ZB - k, 14, rotation=q)])


def bumpers(b, zs, dxs, t, clamps=False):
    for end, sg, zi in (("front", 1, 0.34), ("rear", -1, 0.42)):
        x0 = sg * (XBUMP - t)
        for nm, y in zip(("lower", "upper"), BUMP_Y):
            b.append(loft(f"bumper-{end}-{nm}", CH, NI, 0.8, [sec([x0 + sg * dx, y, z], t, 0.0225, 4) for z, dx in zip(zs, dxs)]))
        tip = (2.07, 0.505) if sg > 0 else (-1.95, 0.56)
        b += pair(capsule(f"bracket-{end}-l", CH, EX, 0.6, [tip[0], tip[1], zi], [x0 - sg * 0.004, 0.52, zi], max(0.014, t), role="detail"))


# ---------------------------------------------------------------- 1:100
def wheel100(bone, t):
    o = [0, 0, ZW]
    out = [revolve("tyre-l", bone, RU, 0.08, "z", o, TYRES[100]), revolve("rim-l", bone, RM, 0.55, "z", o, [[-0.05, 0.222], [-0.05, 0.256], [0.05, 0.256], [0.05, 0.222], [-0.05, 0.222]], role="detail")]
    sp = capsule("spokes-l", bone, RM, 0.7, [0.06, 0, ZW + 0.015], [0.232, 0, ZW], 0.016, role="detail")
    sp["repeat"] = {"count": 8, "rotationRad": r6(math.pi / 4), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}
    out.append(sp)
    out.append(revolve("hub-l", bone, NI, 0.8, "z", o, [[-0.03, 0], [-0.03, 0.068], [0.04, 0.068], [0.07, 0.035], [0.075, 0]], role="detail"))
    out.append(revolve("drum-l", bone, CI, 0.3, "z", o, [[-0.10, 0], [-0.10, 0.16], [-0.05, 0.16], [-0.05, 0]], role="detail"))
    out.append(capsule("valve-l", bone, RM, 0.9, [0.2188, 0.0884, ZW + 0.03], [0.2462, 0.0995, ZW + 0.03], 0.0075, role="detail"))
    res = []
    for d in out:
        res += both(d, t)
    return res


def body100():
    b = []
    b += pair(loft("frame-rail-l", CH, P, 0.18, [xsec(x, yc - h / 2, yc + h / 2, 0.0225, 10, z) for x, yc, z, h in RAILS[100]]))
    for nm, x, y, hw in (("front", 1.97, 0.50, 0.34), ("mid", -0.45, 0.455, 0.42), ("rear", -1.93, 0.565, 0.42)):
        b.append(rbox(f"crossmember-{nm}", CH, P, 0.18, [0.05, 0.06, 2 * hw], [x, y, 0], 0.008))
    b += pair(sheet("spring-front-l", CH, EX, 0.25, [SPRING_F[0], SPRING_F[3], SPRING_F[6]], [0.34] * 3, [0.0225] * 3, [0.01, 0.03, 0.01], e=6, role="detail"))
    b += pair(sheet("spring-rear-l", CH, EX, 0.25, [SPRING_R[0], SPRING_R[3], SPRING_R[6]], [0.47] * 3, [0.025] * 3, [0.01, 0.03, 0.01], e=6, role="detail"))
    b.append(loft("axle-beam", AF, CI, 0.3, [sec([0, y, z], 0.022, 0.03, 6) for y, z in ((0, -0.59), (-0.06, -0.40), (-0.06, 0.40), (0, 0.59))], role="detail"))
    b += pair(revolve("stub-axle-l", AF, CI, 0.3, "z", [0, 0, 0.58], [[0, 0], [0, 0.03], [0.085, 0.022], [0.085, 0]], role="detail"))
    b.append(revolve("axle-tube", AR, CI, 0.3, "z", [0, 0, 0], [[-0.59, 0], [-0.59, 0.035], [-0.15, 0.042], [-0.07, 0.13], [0.07, 0.13], [0.15, 0.042], [0.59, 0.035], [0.59, 0]], role="detail"))
    b.append(capsule("propeller-shaft", CH, EX, 0.25, [XR + 0.1, RW, 0], [0.50, 0.46, 0], 0.03, role="detail"))
    b.append(rbox("engine-block", CH, EM, 0.35, [0.70, 0.42, 0.22], [1.23, 0.77, 0], 0.03, role="detail"))
    b.append(loft("gearbox", CH, EM, 0.3, [xsec(0.50, 0.435, 0.545, 0.07, 3), xsec(0.72, 0.42, 0.76, 0.16, 3), xsec(0.90, 0.43, 0.76, 0.14, 4)], role="detail"))
    b.append(capsule("exhaust-downpipe", CH, EX, 0.35, [0.98, 0.78, -0.15], EXH[2], 0.027, role="detail"))
    b.append(capsule("exhaust-pipe", CH, EX, 0.35, EXH[2], EXH[3], 0.025, role="detail"))
    b.append(revolve("silencer", CH, EX, 0.4, [-1, 0, 0], [-0.13, 0.36, -0.30], [[0, 0], [0, 0.07], [0.73, 0.07], [0.73, 0]], role="detail"))
    b += [dict(d, id=f"tail-pipe-{i + 1}") for i, d in enumerate(tube("x", CH, EX, 0.45, [EXH_TAIL[0], EXH_TAIL[2], EXH_TAIL[3], EXH_TAIL[4]], 0.025))]
    b.append(rbox("fuel-tank", CH, P, 0.2, [0.20, 0.19, 0.58], [-1.845, 0.53, 0], 0.06, role="detail"))
    # body: closed tub core with two skins that carry the wheel arches, cowl, glazed cabin frames, roof
    b += tub_shell(100, 0.025)
    c = SW_C
    b.append(capsule("steering-column", CH, EX, 0.3, list(c), [0.62, 0.84, c[2]], 0.015, role="detail"))
    ring = [[r6(0.015 * math.cos(math.radians(a))), r6(0.20 + 0.015 * math.sin(math.radians(a)))] for a in range(0, 360, 60)]
    b.append(revolve("steering-wheel", CH, EX, 0.3, list(SW_D), list(c), ring + [ring[0]], role="detail"))
    b.append(loft("cowl", CH, P, 0.6, [xsec(x, YSILL, top, hw, e) for x, top, hw, e in COWLS[100]]))
    b += greenhouse(100, 0.008)
    b.append(loft("roof-slab", CH, FB, 0.78, [sec([x, yc, 0], rx, ry, ROOF_E, rotation=Qy(math.pi / 2)) for x, yc, rx, ry in ROOFS[100]]))
    b.append(loft("hood-top", CH, P, 0.62, [sec([x, top - HOOD_RY, 0], hw, HOOD_RY, e, rotation=Qy(math.pi / 2)) for x, top, hw, e in (HOOD[0], HOOD[2])]))
    L = lambda x: 0.72 + (x - 0.72) / math.cos(HOOD_B)
    lh = [[[L(x) - 0.015, 0.76], [L(x) + 0.015, 0.76], [L(x) + 0.015, 0.98], [L(x) - 0.015, 0.98]] for x in LOUVRES100]
    hs = ext("hood-side-l", CH, P, 0.62, "xy", [[0.72, YSILL], [L(1.69), YSILL], [L(1.69), 1.09], [0.72, 1.09]], HOOD_ZR - 0.017, HOOD_ZR, holes=lh)
    b += pair(turn(hs, Qy(HOOD_B), [0.72, 0, HOOD_ZR]))
    b.append(ext("radiator-shell", CH, NI, 0.78, "yz", full(SHELLS[100]), 1.68, 1.80, holes=[full(CORE100)]))
    b.append(ext("radiator-core", CH, EM, 0.15, "yz", [[y + (0.004 if y > 0.9 else -0.004), z * 1.02] for y, z in full(CORE100)], 1.70, 1.775, role="detail"))
    b.append(revolve("radiator-cap", CH, NI, 0.78, "y", [1.74, 1.19, 0], [[0, 0], [0, 0.03], [0.035, 0.03], [0.035, 0.045], [0.06, 0.03], [0.065, 0]], role="detail"))
    # wings, boards, lamps, bumpers, spare
    wf, wb = WINGF[100], WINGB[100]
    b += pair(sheet("fender-front-l", CH, P, 0.5, *wf, e=2.4))
    b += pair(sheet("fender-rear-l", CH, P, 0.5, *wb, e=2.4))
    b += pair(rbox("running-board-l", CH, P, 0.42, [2.05, 0.035, 0.21], [0.005, 0.4375, 0.725], 0.012))
    b += pair(rbox("sill-apron-l", CH, P, 0.5, [1.60, 0.085, 0.03], [-0.27, 0.4825, 0.607], None))
    for k, x in enumerate((-0.55, 0.55)):
        b += pair(rbox(f"running-bracket-{k + 1}-l", CH, P, 0.18, [0.04, 0.03, 0.40], [x, 0.405, 0.62], None, role="detail"))
    lo = [LAMP[0], LAMP[1], LAMP[2]]
    b += pair(revolve("headlamp-bowl-l", CH, NI, 0.74, "x", lo, [[0, 0], [0.03, 0.08], [0.10, 0.112], [0.165, 0.118], [0.165, 0]]))
    b += pair(revolve("headlamp-lens-l", CH, GL, 0.88, "x", lo, [[0.163, 0], [0.163, 0.105], [0.18, 0.06], [0.184, 0]], role="glazing"))
    b += pair(capsule("lamp-stalk-l", CH, NI, 0.5, [1.83, 0.862, 0.43], [1.845, 0.895, 0.43], 0.016, role="detail"))
    b.append(capsule("lamp-bar", CH, NI, 0.74, [1.83, 0.862, -0.62], [1.83, 0.862, 0.62], 0.015, role="detail"))
    b.append(revolve("tail-lamp", CH, NI, 0.3, [-1, 0, 0], [-1.985, 0.95, 0.42], [[0, 0], [0, 0.032], [0.045, 0.032], [0.045, 0]], role="detail"))
    bumpers(b, BUMP100[0], BUMP100[1], 0.0075)
    arch_y = RW + math.sqrt(ARCH_R ** 2 - (X_C - XR) ** 2)
    for nm, x, y0 in (("hinge-gap", X_A, 0.56), ("gap-b", X_B, 0.56), ("gap-rear", X_C, arch_y + 0.01)):
        b += pair(texture(rbox(f"door-{nm}-l", CH, P, 0.12, [0.015, YBELT - y0, 0.004], [x, (YBELT + y0) / 2, ZB + 0.001], None, role="detail")))
    b += pair(texture(rbox("belt-line-l", CH, P, 0.4, [2.41, 0.015, 0.004], [-0.69, YBELT, ZB + 0.001], None, role="detail")))
    so = [SPARE[0], SPARE[1], SPARE[2]]
    b.append(revolve("spare-tyre", CH, RU, 0.08, "z", so, TYRES[100]))
    b.append(revolve("spare-cover", CH, P, 0.62, [0, 0, -1], so, [[-0.03, 0], [-0.03, 0.256], [0.03, 0.256], [0.045, 0.16], [0.05, 0]], role="detail"))
    b.append(capsule("spare-mount", CH, EX, 0.18, [0.80, 0.90, -0.41], [SPARE[0], SPARE[1], SPARE[2] + 0.02], 0.02, role="detail"))
    return b + wheel100(WF, "f") + wheel100(WR, "r")


def benches(s, t=0.06):
    s.th(rbox("th-seat-front", CH, UP, 0.3, [0.52, 0.56, 1.10], [-0.08, 0.90, 0], t, role="detail"), "interior")
    s.th(rbox("th-seat-rear", CH, UP, 0.3, [0.56, 0.60, 1.10], [-1.42, 0.92, 0], t, role="detail"), "interior")


def thermal100(s):
    for i, f in (("tub-side-l", ["-w"]), ("tub-side-r", ["+w"]), ("tub-rear", ["+u"]), ("cowl", ["-u"]), ("cabin-side-l", ["-w"]), ("cabin-side-r", ["+w"]), ("windscreen-frame", ["-u"]), ("cabin-rear-wall", ["+u"])):
        tref(s, i, "body-panels", f, "paint")
    tref(s, "roof-slab", "roof", ["-v"])
    for i, f in (("hood-top", ["-v"]), ("hood-side-l", ["-w"]), ("hood-side-r", ["+w"])):
        tref(s, i, "hood", f, "paint")
    for i in ("fender-front-l", "fender-front-r", "fender-rear-l", "fender-rear-r", "running-board-l", "running-board-r"):
        tref(s, i, "fenders", None, "paint")
    for i, f in (("window-band-l", ["-w"]), ("window-band-r", ["+w"]), ("windscreen", ["-u"]), ("window-back", ["+u"]), ("headlamp-lens-l", None), ("headlamp-lens-r", None)):
        tref(s, i, "glazing", f)
    B = {d["id"]: d for d in s.body}
    for i in ("seat-front", "seat-rear", "steering-column", "steering-wheel"):
        s.th(dict(B[i], id="th-" + i), "interior")
    tref(s, "radiator-shell", "radiator")
    tref(s, "radiator-cap", "radiator")
    tref(s, "engine-block", "engine-bay")
    for i in [d["id"] for d in s.body if d["id"].startswith(("exhaust-", "silencer", "tail-pipe"))]:
        tref(s, i, "exhaust-system")
    s.th(jet(0.015), "exhaust-system")
    for i in [d["id"] for d in s.body if d["id"].startswith("tyre-") or d["id"] == "spare-tyre"]:
        tref(s, i, "tyres")
    for i in [d["id"] for d in s.body if d["id"].startswith("drum-")]:
        tref(s, i, "brakes")


def body_frame(s, pillars, bows, bearers, t):
    for sd, sg in (("l", 1), ("r", -1)):
        s.st(rbox(f"body-sill-{sd}", CH, P, 0.4, [2.42, 0.06, 2 * t], [-0.71, 0.55, sg * 0.57], None), CH, "member")
        s.st(rbox(f"cant-rail-{sd}", CH, P, 0.4, [2.26, 2 * t, 2 * t], [-0.67, 1.615, sg * 0.555], None), CH, "member")
        for nm, xb, xt, yb in pillars:
            s.st(post(f"pillar-{nm}-{sd}", xb, xt, yb, 1.615, sg * 0.575, sg * 0.555, t), CH, "member")
    for k_, x in enumerate(bearers):
        s.st(rbox(f"floor-bearer-{k_ + 1}", CH, P, 0.4, [2 * t, 0.04 if t < 0.03 else 2 * t, 1.14], [x, 0.54, 0], None), CH, "member")
    for k_, x in enumerate(bows):
        s.st(rbox(f"roof-bow-{k_ + 1}", CH, P, 0.4, [2 * t, 2 * t, 1.11], [x, 1.625, 0], None), CH, "member")


def structure100(s):
    for i in ("frame-rail-l", "frame-rail-r", "crossmember-front", "crossmember-mid", "crossmember-rear", "spring-front-l", "spring-front-r", "spring-rear-l", "spring-rear-r"):
        s.stof("st-" + i, i, CH, "member")
    body_frame(s, (("a", 0.49, 0.455, 0.55), ("b", -0.33, -0.33, 0.55), ("c", -1.105, -1.105, 0.63), ("d", -1.875, -1.79, 0.55)), (0.30, -0.65, -1.60), (0.30, -0.10, -0.80), 0.025)
    s.stof("st-axle-beam", "axle-beam", AF, "member")
    s.stof("st-axle-tube", "axle-tube", AR, "member")
    for bone, ab, t in ((WF, AF, "f"), (WR, AR, "r")):
        for sd, sg in (("l", 1), ("r", -1)):
            s.st(rbox(f"spring-clamp-{t}{sd}", ab, EX, 0.25, [0.10, 0.08, 0.06], [0, 0.0 if t == "f" else 0.03, sg * (0.34 if t == "f" else 0.47)], None, role="detail"), ab, "joint")
            for part in ("hub", "spokes", "rim"):
                s.stof(f"st-{part}-{t}{sd}", f"{part}-{t}{sd}", bone, "member")
            s.st(revolve(f"wheel-bearing-{t}{sd}", bone, CI, 0.2, [0, 0, sg], [0, 0, sg * (ZW - 0.07)], [[0, 0], [0, 0.045], [0.08, 0.045], [0.08, 0]], role="detail"), bone, "joint")
            s.st(ellipsoid(f"mass-wheel-{t}{sd}", bone, EX, 0.2, [0.10, 0.10, 0.04], [0, 0, sg * ZW], role="detail"), bone, "mass")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.22, 0.16, 0.30], [-0.1, 0.72, 0], role="detail"), CH, "mass")


# ---------------------------------------------------------------- 1:200
def wheel200(bone, t):
    o = [0, 0, ZW]
    out = [revolve("tyre-l", bone, RU, 0.08, "z", o, TYRES[200]), revolve("rim-l", bone, RM, 0.55, "z", o, [[-0.045, 0], [-0.045, 0.256], [0.0, 0.256], [0.0, 0]], role="detail")]
    sp = rbox("spokes-l", bone, RM, 0.2, [0.44, 0.06, 0.03], [0, 0, ZW + 0.012], None, role="detail")
    sp["repeat"] = {"count": 2, "rotationRad": r6(math.pi / 2), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}
    out.append(sp)
    out.append(revolve("hub-l", bone, NI, 0.8, "z", o, [[0, 0], [0, 0.065], [0.05, 0.06], [0.075, 0]], role="detail"))
    res = []
    for d in out:
        res += both(d, t)
    return res


def body200():
    b = []
    b += pair(loft("frame-rail-l", CH, P, 0.18, [xsec(x, yc - h / 2, yc + h / 2, 0.0225, 10, z) for x, yc, z, h in RAILS[200]]))
    b.append(rbox("crossmember-front", CH, P, 0.18, [0.05, 0.06, 0.68], [1.97, 0.50, 0], None))
    b.append(rbox("crossmember-rear", CH, P, 0.18, [0.05, 0.06, 0.84], [-1.93, 0.565, 0], None))
    b.append(revolve("axle-beam", AF, CI, 0.3, "z", [0, -0.045, -0.63], [[0, 0], [0, 0.03], [1.26, 0.03], [1.26, 0]], role="detail"))
    b.append(revolve("axle-tube", AR, CI, 0.3, "z", [0, 0, 0], [[-0.63, 0], [-0.63, 0.035], [-0.14, 0.04], [-0.06, 0.13], [0.06, 0.13], [0.14, 0.04], [0.63, 0.035], [0.63, 0]], role="detail"))
    b.append(capsule("propeller-shaft", CH, EX, 0.25, [XR + 0.1, RW, 0], [0.72, 0.47, 0], 0.03, role="detail"))
    b.append(capsule("exhaust-pipe", CH, EX, 0.35, [0.95, 0.40, -0.30], EXH[3], 0.025, role="detail"))
    b.append(capsule("silencer", CH, EX, 0.4, [-0.20, 0.36, -0.30], [-0.80, 0.36, -0.30], 0.07, role="detail"))
    b += [dict(d, id=f"tail-pipe-{i + 1}") for i, d in enumerate(tube("x", CH, EX, 0.45, [EXH_TAIL[0], EXH_TAIL[2], EXH_TAIL[3], EXH_TAIL[4]], 0.025))]
    b += tub_shell(200, 0.03)
    b.append(loft("cowl", CH, P, 0.6, [xsec(x, YSILL, top, hw, e) for x, top, hw, e in COWLS[200]]))
    b += greenhouse(200, 0.01)
    b.append(loft("roof-slab", CH, FB, 0.78, [sec([x, yc, 0], rx, ry, 3, rotation=Qy(math.pi / 2)) for x, yc, rx, ry in ROOFS[200]]))
    b.append(loft("hood", CH, P, 0.62, [xsec(x, YSILL, top, hw, e) for x, top, hw, e in ((0.72, 1.185, HOOD_ZR, 8), (1.69, 1.185, HOOD_ZF, 5))]))
    band_ = rbox("louvre-band-l", CH, EM, 0.12, [0.60, 0.22, 0.006], [1.20, 0.87, hood_z(1.20) + 0.002], None, role="detail", rot=Qy(HOOD_B))
    b += pair(texture(band_))
    b.append(ext("radiator-shell", CH, NI, 0.78, "yz", full(SHELLS[200]), 1.68, 1.80))
    b.append(revolve("radiator-cap", CH, NI, 0.78, "y", [1.74, 1.19, 0], [[0, 0], [0, 0.035], [0.05, 0.035], [0.065, 0]], role="detail"))
    b += pair(sheet("fender-front-l", CH, P, 0.5, *WINGF[200], e=2.4))
    b += pair(sheet("fender-rear-l", CH, P, 0.5, *WINGB[200], e=2.4))
    b += pair(rbox("running-board-l", CH, P, 0.42, [2.05, 0.035, 0.21], [0.005, 0.4375, 0.725], None))
    b += pair(rbox("sill-apron-l", CH, P, 0.5, [1.60, 0.085, 0.03], [-0.27, 0.4825, 0.607], None))
    b += pair(rbox("running-bracket-l", CH, P, 0.18, [0.06, 0.03, 0.40], [0.0, 0.405, 0.62], None, role="detail"))
    lo = [LAMP[0], LAMP[1], LAMP[2]]
    b += pair(revolve("headlamp-l", CH, NI, 0.74, "x", lo, [[0, 0], [0.04, 0.09], [0.165, 0.118], [0.184, 0]]))
    b += pair(capsule("lamp-stalk-l", CH, NI, 0.5, [1.86, 0.53, 0.36], [1.85, 0.88, 0.43], 0.016, role="detail"))
    b.append(rbox("tail-lamp", CH, NI, 0.3, [0.05, 0.07, 0.07], [-2.005, 0.95, 0.42], None, role="detail"))
    bumpers(b, BUMP100[0], BUMP100[1], 0.015)
    so = [SPARE[0], SPARE[1], SPARE[2]]
    b.append(revolve("spare-tyre", CH, RU, 0.08, "z", so, TYRES[200]))
    b.append(revolve("spare-cover", CH, P, 0.62, [0, 0, -1], so, [[-0.03, 0], [-0.03, 0.256], [0.03, 0.256], [0.05, 0]], role="detail"))
    b.append(capsule("spare-mount", CH, EX, 0.18, [0.80, 0.90, -0.41], [SPARE[0], SPARE[1], SPARE[2] + 0.02], 0.02, role="detail"))
    return b + wheel200(WF, "f") + wheel200(WR, "r")


def thermal_coarse(s, den):
    B = {d["id"] for d in s.body}
    for i, f in (("tub-core", ["+v", "-v"]), ("tub-side-l", ["-w"]), ("tub-side-r", ["+w"]), ("tub-rear", ["+u"]), ("cowl", ["-u"]), ("cabin", ["-v", "+v"]), ("cabin-frame-l", ["-w"]), ("cabin-frame-r", ["+w"]), ("windscreen-frame", ["-u"]), ("cabin-rear-wall", ["+u"])):
        if i in B:
            tref(s, i, "body-panels", f, "paint")
    tref(s, "roof-slab", "roof", ["-v"])
    tref(s, "hood", "hood", ["-v", "-u"], "paint")
    for i in ("fender-front-l", "fender-front-r", "fender-rear-l", "fender-rear-r", "running-board-l", "running-board-r", "wing-line-l", "wing-line-r"):
        if i in B:
            tref(s, i, "fenders", None, "paint")
    for d in s.body:
        if d["role"] == "glazing":
            f = ["-w"] if d["id"].endswith("-l") else ["+w"] if d["id"].endswith("-r") else ["+u"] if d["id"] == "window-back" else ["-u"]
            tref(s, d["id"], "glazing", f)
    if den == 200:
        for d in s.body:
            if d["id"] in ("seat-front", "seat-rear"):
                s.th(dict(d, id="th-" + d["id"]), "interior")
    else:
        benches(s, None)
    tref(s, "radiator-shell", "radiator", ["-u"])
    s.th(rbox("th-engine", CH, EM, 0.35, [0.70, 0.42, 0.22], [1.23, 0.77, 0], 0.03 if den == 200 else None, role="detail"), "engine-bay")
    if den == 200:
        for i in [d["id"] for d in s.body if d["id"].startswith(("exhaust-", "silencer", "tail-pipe"))]:
            tref(s, i, "exhaust-system")
    else:
        s.th(capsule("th-silencer", CH, EX, 0.4, [-0.20, 0.375, -0.30], [-0.80, 0.375, -0.30], 0.07, role="detail"), "exhaust-system")
        s.th(capsule("th-exhaust-pipe", CH, EX, 0.35, [0.95, 0.40, -0.30], [-2.06, 0.42, -0.33], 0.0375, role="detail"), "exhaust-system")
    s.th(jet(0.03 if den == 200 else 0.075, 0.0225 if den == 200 else 0.0375), "exhaust-system")
    for i in [d["id"] for d in s.body if d["id"].startswith("tyre-") or d["id"] == "spare-tyre"]:
        tref(s, i, "tyres")
    w = 0.05 if den == 200 else 0.075
    for bone, t in ((WF, "f"), (WR, "r")):
        for sd, sg in (("l", 1), ("r", -1)):
            s.th(revolve(f"th-drum-{t}{sd}", bone, CI, 0.3, [0, 0, sg], [0, 0, sg * (ZW - 0.05 - w)], [[0, 0], [0, 0.16], [w, 0.16], [w, 0]], role="detail"), "brakes")


def thermal200(s):
    thermal_coarse(s, 200)


def structure200(s):
    for i in ("frame-rail-l", "frame-rail-r", "crossmember-front", "crossmember-rear"):
        s.stof("st-" + i, i, CH, "member")
    body_frame(s, (("a", 0.49, 0.455, 0.55), ("b", -0.33, -0.33, 0.55), ("d", -1.875, -1.79, 0.55)), (-0.65,), (-0.10,), 0.03)
    s.stof("st-axle-beam", "axle-beam", AF, "member")
    s.stof("st-axle-tube", "axle-tube", AR, "member")
    for bone, t in ((WF, "f"), (WR, "r")):
        for sd, sg in (("l", 1), ("r", -1)):
            s.stof(f"st-rim-{t}{sd}", f"rim-{t}{sd}", bone, "member")
            s.st(revolve(f"wheel-bearing-{t}{sd}", bone, CI, 0.2, [0, 0, sg], [0, 0, sg * (ZW - 0.09)], [[0, 0], [0, 0.05], [0.09, 0.05], [0.09, 0]], role="detail"), bone, "joint")
        s.st(ellipsoid(f"mass-wheels-{t}", bone, EX, 0.2, [0.11, 0.11, 0.11], [0, 0, 0], role="detail"), bone, "mass")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.22, 0.16, 0.30], [-0.1, 0.72, 0], role="detail"), CH, "mass")


# ---------------------------------------------------------------- 1:500
def body500():
    b = []
    b.append(rbox("underbody", CH, EX, 0.25, [4.30, 0.12, 0.70], [0.0, 0.46, 0], None))
    b.append(ext("tub-core", CH, P, 0.55, "xy", core_of(500), -0.545, 0.545))
    b += pair(ext("tub-side-l", CH, P, 0.55, "xy", tub_of(500), 0.545, ZB))
    b.append(frustum("cabin", 0.55))
    b.append(loft("roof-slab", CH, FB, 0.78, [sec([x, yc, 0], rx, ry, 4, rotation=Qy(math.pi / 2)) for x, yc, rx, ry in ROOFS[500]]))
    b.append(loft("hood", CH, P, 0.55, [xsec(0.47, YSILL, 1.16, 0.62, 20), xsec(0.60, YSILL, 1.174, 0.58, 10), xsec(0.74, YSILL, 1.185, HOOD_ZR, 8), xsec(1.69, YSILL, 1.185, HOOD_ZF, 5)]))
    b.append(ext("radiator-shell", CH, NI, 0.78, "yz", full([[0.52, 0.285], [1.08, 0.297], [1.20, 0.0]]), 1.68, 1.80))
    for sd, sg in (("l", 1), ("r", -1)):
        z0, z1 = (0.61, 0.63) if sg > 0 else (-0.63, -0.61)
        g = ext(f"window-{sd}", CH, GL, 0.85, "xy", [[0.444, 1.225], [0.415, 1.565], [-1.692, 1.565], [-1.766, 1.225]], 0.61, 0.63, role="glazing")
        b.append(tilt_side(g) if sg > 0 else mir(tilt_side(g), f"window-{sd}"))
    b.append(turn(ext("windscreen", CH, GL, 0.85, "yz", [[1.215, -0.54], [1.215, 0.54], [1.578, 0.51], [1.578, -0.51]], 0.51, 0.53, role="glazing"), Qz(A_TUM), [0.52, YBELT, 0]))
    b += pair(sheet("wing-line-l", CH, P, 0.55, *WINGLINE, e=4))
    for end, sg in (("front", 1), ("rear", -1)):
        b.append(rbox(f"bumper-{end}", CH, NI, 0.8, [0.075, 0.135, 1.48], [sg * (XBUMP - 0.0375), 0.52, 0], None))
    for bone, nm in ((AF, "axle-f"), (AR, "axle-r")):
        b.append(rbox(nm, bone, CI, 0.3, [0.075, 0.075, 1.27], [0, 0, 0], None, role="detail"))
    for bone, t in ((WF, "f"), (WR, "r")):
        b += both(revolve("tyre-l", bone, RU, 0.08, "z", [0, 0, ZW], [[-0.065, 0], [-0.065, 0.33], [-0.035, 0.37], [0.035, 0.37], [0.065, 0.33], [0.065, 0]]), t)
    return b


def thermal500(s):
    thermal_coarse(s, 500)


def structure500(s):
    for sd, sg in (("l", 1), ("r", -1)):
        s.st(loft(f"frame-rail-{sd}", CH, P, 0.18, [xsec(x, yc - 0.0375, yc + 0.0375, 0.0375, 8, sg * z) for x, yc, z in ((2.06, 0.495, 0.30), (0.50, 0.475, 0.31), (-1.05, 0.475, 0.31), (-1.50, 0.585, 0.31), (-1.96, 0.565, 0.31))]), CH, "member")
        s.st(rbox(f"body-sill-{sd}", CH, P, 0.4, [2.42, 0.075, 0.075], [-0.71, 0.56, sg * 0.56], None), CH, "member")
        s.st(rbox(f"cant-rail-{sd}", CH, P, 0.4, [2.26, 0.075, 0.075], [-0.67, 1.60, sg * 0.54], None), CH, "member")
        for nm, xb, xt in (("a", 0.47, 0.44), ("d", -1.85, -1.77)):
            s.st(post(f"pillar-{nm}-{sd}", xb, xt, 0.56, 1.60, sg * 0.56, sg * 0.54, 0.0375), CH, "member")
    s.st(rbox("crossmember-mid", CH, P, 0.18, [0.075, 0.075, 0.62], [-0.30, 0.475, 0], None), CH, "member")
    s.stof("st-axle-f", "axle-f", AF, "member")
    s.stof("st-axle-r", "axle-r", AR, "member")
    s.st(rbox("differential", AR, CI, 0.3, [0.20, 0.20, 0.16], [0, 0, 0], None, role="detail"), AR, "joint")
    for bone, t in ((WF, "f"), (WR, "r")):
        for sd, sg in (("l", 1), ("r", -1)):
            s.st(revolve(f"wheel-{t}{sd}", bone, RM, 0.55, [0, 0, sg], [0, 0, sg * (ZW - 0.0375)], [[0, 0], [0, 0.25], [0.075, 0.25], [0.075, 0]], role="detail"), bone, "member")
            s.st(rbox(f"mass-wheel-{t}{sd}", bone, EX, 0.2, [0.15, 0.15, 0.075], [0, 0, sg * (ZW + 0.04)], None, role="detail"), bone, "mass")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.22, 0.16, 0.30], [-0.1, 0.72, 0], role="detail"), CH, "mass")
