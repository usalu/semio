import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-44-lib.py")).read())
SLUG = "vintage-car-1929"
P, FR, GL, RU, RM, NI, CI, EX, EM, UP = "paint", "fabric-roof", "glass", "rubber", "steel-rim", "nickel-plate", "cast-iron", "exhaust-steel", "engine-metal", "upholstery"
CH, AF, AR, WF, WR = "chassis", "axle-f", "axle-r", "wheel-f", "wheel-r"
ZC = 0.6705
R_ = lambda s, e: shape(SLUG, s, "elevation", e)
F_ = lambda s, e: shape(SLUG, s, "front", e)
PL_ = lambda s, e: shape(SLUG, s, "plan", e)


def Fk(y):
    return 0.6 - (y - 1.14) * 0.1 / 0.48


def Rk(y):
    return -1.84 + (y - 1.14) * 0.14 / 0.44


def band(ring, x, lo=0.03):
    a, b = span(ring, x)
    if b - a < lo:
        m = (a + b) / 2
        a, b = m - lo / 2, m + lo / 2
    return a, b


def fender_loft(id, sc, side, which, tone=0.22):
    ring = R_(sc, "fender-front" if which == "f" else "fender-rear")
    pr = PL_(sc, ("fender-f" if which == "f" else "fender-r") + ("l" if side > 0 else "r"))
    xs = [0.962, 0.98, 1.0, 1.05, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.05, 2.1, 2.136] if which == "f" else [-2.026, -2.0, -1.95, -1.9, -1.8, -1.7, -1.6, -1.5, -1.4, -1.3, -1.2, -1.1, -1.0, -0.982]
    rows = []
    for x in xs:
        y0, y1 = band(ring, x)
        z0, z1 = span(pr, x)
        if side < 0 and which == "f" and x <= 1.17:
            z1 = -0.62
        rows.append((x, y0, y1, z0, z1, 5))
    return xloft(id, CH, P, tone, rows)


def wheel_solids(bone, tag, sgn, lod=50):
    sx = lambda prof: [[r6(sgn * a), r] for a, r in prof]
    o = [0, 0, sgn * ZC]
    out = []
    out.append(revolve(f"tyre-{tag}", bone, RU, 0.06, "z", o, sx(tyre_profile(0.25, 0.37, 0.0575))))
    out.append(revolve(f"rim-{tag}", bone, RM, 0.55, "z", o, sx(cap_prof([[-0.0425, 0.215], [-0.0425, 0.252], [0.0425, 0.252], [0.0425, 0.215]])), role="detail"))
    out.append(revolve(f"disc-{tag}", bone, RM, 0.5, "z", o, sx(cap_prof([[-0.01, 0], [-0.01, 0.215], [0.01, 0.215], [0.01, 0]])), role="detail"))
    rib = box(f"spokes-{tag}", bone, RM, 0.62, [0.15, 0.022, 0.01], [0.14, 0, sgn * (ZC + 0.015)], role="detail")
    rib["repeat"] = {"count": 12, "rotationRad": r6(2 * math.pi / 12), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}
    out.append(rib)
    out.append(revolve(f"hub-{tag}", bone, NI, 0.85, "z", o, sx(cap_prof([[0.008, 0], [0.008, 0.07], [0.085, 0.066], [0.0965, 0.04], [0.0965, 0]])), role="detail"))
    out.append(revolve(f"drum-{tag}", bone, CI, 0.3, "z", o, sx(cap_prof([[-0.0575, 0], [-0.0575, 0.15], [-0.01, 0.15], [-0.01, 0]])), role="detail"))
    return out


def axle_solids(bone, tag):
    out = []
    for nm, a, b in (("l", 0.41, 0.6), ("r", -0.6, -0.41), ("mid", -0.35, 0.35)):
        out.append(revolve(f"axle-{tag}-{nm}", bone, CI, 0.3, "z", [0, 0, a], cap_prof([[0, 0], [0, 0.03], [b - a, 0.03], [b - a, 0]]), role="detail"))
    return out


def body50():
    b = []
    tub = dedup(R_(50, "tub"))
    pl_tub = PL_(50, "tub")
    # frame
    fr = dedup(R_(50, "frame"))
    b.append(ext("frame-rail-l", CH, P, 0.15, "xy", fr, 0.35, 0.41))
    b.append(ext("frame-rail-r", CH, P, 0.15, "xy", fr, -0.41, -0.35))
    for nm, x in (("front", 1.75), ("mid", 0.2), ("rear", -1.9)):
        b.append(rbox(f"crossmember-{nm}", CH, P, 0.15, [0.05, 0.06, 0.70], [x, 0.38, 0], 0.005))
    for nm, x in (("a", -1.45), ("b", -0.5), ("c", 0.4)):
        for s, sg in (("l", 1), ("r", -1)):
            b.append(rbox(f"body-mount-{nm}-{s}", CH, P, 0.15, [0.1, 0.11, 0.06], [x, 0.465, sg * 0.38], 0.004))
    # lower body (hollow tub)
    for s, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"tub-side-{s}", CH, P, 0.25, "xy", tub, 0.54 if sg > 0 else -0.62, 0.62 if sg > 0 else -0.54))
    rear = clip_box(tub, -1.99, -1.80, 0.0, 2.0)
    b.append(ext("tub-rear", CH, P, 0.25, "xy", dedup(rear), -0.54, 0.54))
    b.append(xloft("tub-floor", CH, P, 0.2, [(-1.8, 0.52, 0.6, -0.54, 0.54), (-1.0, 0.52, 0.6, -0.54, 0.54), (0.52, 0.52, 0.6, -0.54, 0.54)], e=10))
    b.append(rbox("tub-front", CH, P, 0.25, [0.08, 0.62, 1.08], [0.56, 0.83, 0], 0.006))
    # cabin side frames with window openings
    cab = dedup(clip_box(R_(50, "cabin"), -1.9, 0.7, 1.14, 1.64))
    holes = [[[-0.27, 1.21], [0.50, 1.21], [0.50, 1.58], [-0.27, 1.58]], [[-1.07, 1.21], [-0.41, 1.21], [-0.41, 1.58], [-1.07, 1.58]], [[-1.17, 1.21], [-1.17, 1.58], [-1.62, 1.58], [-1.74, 1.21]]]
    for s, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"cabin-side-{s}", CH, P, 0.25, "xy", cab, 0.52 if sg > 0 else -0.6, 0.6 if sg > 0 else -0.52, holes=holes))
        z0, z1 = (0.594, 0.6) if sg > 0 else (-0.6, -0.594)
        for nm, h in (("front", holes[0]), ("rear", holes[1]), ("quarter", holes[2])):
            b.append(ext(f"window-{nm}-{s}", CH, GL, 0.85, "xy", h, z0, z1, role="glazing"))
    b.append(rbox("scuttle", CH, P, 0.25, [0.07, 0.07, 1.04], [0.5585, 1.175, 0], 0.004))
    rw = [[Rk(1.14), 1.14], [Rk(1.14) + 0.07, 1.14], [Rk(1.6) + 0.07, 1.6], [Rk(1.6), 1.6]]
    b.append(ext("cabin-rear-wall", CH, P, 0.25, "xy", rw, -0.52, 0.52))
    ws_len = math.hypot(Fk(1.6) - Fk(1.21), 1.6 - 1.21)
    ws_tilt = math.atan2(Fk(1.21) - Fk(1.6), 1.6 - 1.21)
    b.append(box("windscreen", CH, GL, 0.85, [0.012, r6(ws_len), 1.04], [r6((Fk(1.21) + Fk(1.6)) / 2 - 0.012), 1.405, 0], None, "glazing", Qz(ws_tilt)))
    # roof
    rr, rp = R_(50, "roof-slab"), PL_(50, "roof-slab")
    rows = []
    for x in (-1.698, -1.68, -1.65, -1.6, -1.55, -1.5, -1.0, 0.0, 0.5, 0.58, 0.618):
        a, c = band(rr, x)
        h = span(rp, x)[1]
        rows.append((x, a, c, -h, h, 6))
    b.append(xloft("roof-slab", CH, FR, 0.78, rows))
    # hood
    hr, hp = dedup(R_(50, "hood")), PL_(50, "hood")
    rows = []
    for x in (0.622, 0.64, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.76, 1.778):
        yt = band(hr, x)[1]
        h = span(hp, x)[1]
        rows.append((x, yt - 0.025, yt, -h, h, 8))
    b.append(xloft("hood-top", CH, P, 0.2, rows))
    for s, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"hood-side-{s}", CH, P, 0.2, "xy", hr, 0.32 if sg > 0 else -0.34, 0.34 if sg > 0 else -0.32, holes=louvre_holes()))
        b.append(rbox(f"hood-front-{s}", CH, P, 0.2, [0.02, 0.56, 0.12], [1.77, 0.86, sg * 0.28], 0.004))
        v = box(f"louvre-vanes-{s}", CH, P, 0.3, [0.04, 0.174, 0.008], [0.92, 1.045, sg * 0.325], None, "detail", Qy(sg * math.pi / 4))
        v["repeat"] = {"count": 10, "translation": [0.07, 0, 0]}
        b.append(v)
    b.append(rbox("radiator-shell", CH, NI, 0.85, [0.1, 0.7, 0.44], [1.83, 0.91, 0], 0.03))
    b.append(capsule("radiator-neck", CH, NI, 0.85, [1.83, 1.26, 0], [1.83, 1.30, 0], 0.018, role="detail"))
    b.append(ellipsoid("radiator-cap", CH, NI, 0.85, [0.05, 0.032, 0.05], [1.83, 1.335, 0], role="detail"))
    b.append(rbox("radiator-mount", CH, P, 0.15, [0.06, 0.15, 0.2], [1.79, 0.485, 0], 0.004))
    # engine
    b.append(rbox("engine-block", CH, EM, 0.35, [0.62, 0.30, 0.28], [1.23, 0.73, 0], 0.02, role="detail"))
    b.append(rbox("engine-head", CH, EM, 0.3, [0.46, 0.12, 0.24], [1.23, 0.94, 0], 0.015, role="detail"))
    b.append(rbox("engine-cover", CH, EM, 0.4, [0.40, 0.12, 0.20], [1.23, 1.06, 0], 0.015, role="detail"))
    b.append(rbox("gearbox", CH, EM, 0.3, [0.27, 0.34, 0.30], [0.735, 0.75, 0], 0.02, role="detail"))
    for k, x in enumerate((0.95, 1.45)):
        for s, sg in (("l", 1), ("r", -1)):
            b.append(rbox(f"engine-bearer-{k + 1}-{s}", CH, P, 0.15, [0.1, 0.17, 0.22], [x, 0.495, sg * 0.28], 0.004, role="detail"))
    b.append(capsule("exhaust-manifold", CH, EX, 0.3, [0.95, 0.84, -0.18], [1.5, 0.84, -0.18], 0.03, role="detail"))
    b.append(capsule("exhaust-downpipe", CH, EX, 0.3, [0.95, 0.84, -0.18], [0.93, 0.28, -0.3], 0.027, role="detail"))
    # exhaust
    b.append(capsule("exhaust-pipe", CH, EX, 0.3, [-1.05, 0.27, -0.3], [0.9, 0.27, -0.3], 0.03, role="detail"))
    b.append(capsule("silencer", CH, EX, 0.4, [-1.75, 0.27, -0.3], [-1.10, 0.27, -0.3], 0.075, role="detail"))
    b.append(capsule("tail-pipe", CH, EX, 0.3, [-1.75, 0.27, -0.35], [-2.08, 0.25, -0.35], 0.032, 0.0275, role="detail"))
    for k, x in enumerate((-1.5, -0.6, 0.5)):
        b.append(rbox(f"exhaust-hanger-{k + 1}", CH, EX, 0.3, [0.03, 0.08, 0.05], [x, 0.325, -0.325], 0.004, role="detail"))
    # fenders and running boards
    for s, sg in (("l", 1), ("r", -1)):
        b.append(fender_loft(f"fender-front-{s}", 50, sg, "f"))
        b.append(fender_loft(f"fender-rear-{s}", 50, sg, "r"))
        b.append(box(f"running-board-{s}", CH, P, 0.4, [2.0, 0.06, 0.18], [0, 0.43, sg * 0.71], 0.015))
        for k, x in enumerate((-0.8, 0.0, 0.8)):
            b.append(rbox(f"running-bracket-{k + 1}-{s}", CH, P, 0.15, [0.05, 0.08, 0.27], [x, 0.37, sg * 0.545], 0.004, role="detail"))
    # lamps
    for s, sg in (("l", 1), ("r", -1)):
        b.append(ellipsoid(f"headlamp-bowl-{s}", CH, NI, 0.85, [0.105, 0.11, 0.11], [1.78, 0.99, sg * 0.55]))
        b.append(revolve(f"headlamp-lens-{s}", CH, GL, 0.9, "x", [1.848, 0.99, sg * 0.55], cap_prof([[0, 0], [0, 0.085], [0.018, 0.08], [0.036, 0.045], [0.036, 0]]), role="glazing"))
        b.append(capsule(f"lamp-stalk-{s}", CH, NI, 0.85, [1.76, 0.80, sg * 0.55], [1.76, 0.90, sg * 0.55], 0.015, role="detail"))
        c = capsule(f"bracket-front-{s}", CH, EX, 0.2, [2.2, 0.5, sg * 0.3], [2.05, 0.43, sg * 0.3], 0.012, role="detail")
        c["lateralScale"] = 1.67
        b.append(c)
        c = capsule(f"bracket-rear-{s}", CH, EX, 0.2, [-2.1, 0.5, sg * 0.3], [-1.95, 0.43, sg * 0.3], 0.012, role="detail")
        c["lateralScale"] = 1.67
        b.append(c)
    b.append(capsule("fender-lamp", CH, NI, 0.85, [2.0, 0.66, 0.62], [2.1, 0.58, 0.62], 0.02, role="detail"))
    # bumpers
    for nm, x in (("front", 2.22), ("rear", -2.12)):
        b.append(rbox(f"bumper-{nm}-upper", CH, NI, 0.8, [0.044, 0.044, 1.4], [x, 0.662, 0], 0.015))
        b.append(rbox(f"bumper-{nm}-lower", CH, NI, 0.8, [0.044, 0.044, 1.4], [x, 0.4, 0], 0.015))
        for s, sg in (("l", 1), ("r", -1)):
            b.append(rbox(f"bumper-{nm}-post-{s}", CH, NI, 0.8, [0.044, 0.30, 0.044], [x, 0.531, sg * 0.678], 0.012, role="detail"))
    b.append(rbox("number-plate", CH, P, 0.7, [0.03, 0.12, 0.4], [-2.0, 0.64, 0], 0.004, role="detail"))
    b.append(rbox("tail-lamp", CH, P, 0.5, [0.05, 0.1, 0.1], [-2.005, 0.95, 0.4], 0.008, role="detail"))
    for k, (nm, x) in enumerate((("front", -0.15), ("rear", -0.45))):
        for s, sg in (("l", 1), ("r", -1)):
            b.append(rbox(f"handle-{nm}-{s}", CH, NI, 0.85, [0.124, 0.024, 0.012], [x, 1.04, sg * 0.626], 0.005, role="detail"))
    # spare wheel (right side)
    so = [0.86, 0.78, -0.5575]
    b.append(revolve("spare-tyre", CH, RU, 0.06, "z", so, tyre_profile(0.23, 0.31, 0.0575, 0.01)))
    b.append(revolve("spare-cover", CH, P, 0.4, "z", [0.86, 0.78, -0.6], cap_prof([[0, 0], [0, 0.23], [0.015, 0.23], [0.015, 0]]), role="detail"))
    b.append(revolve("spare-boss", CH, NI, 0.85, "z", [0.86, 0.78, -0.615], cap_prof([[0, 0], [0, 0.04], [0.012, 0.04], [0.012, 0]]), role="detail"))
    b.append(capsule("spare-mount", CH, EX, 0.2, [0.86, 0.78, -0.34], [0.86, 0.78, -0.5], 0.02, role="detail"))
    # axles and wheels
    for bone, tag in ((AF, "f"), (AR, "r")):
        b.extend(axle_solids(bone, tag))
    for bone, tag, wb in ((WF, "f", WF), (WR, "r", WR)):
        for s, sg in (("l", 1), ("r", -1)):
            b.extend(wheel_solids(wb, f"{tag}{s}", sg))
    return b


def louvre_holes():
    return [[[0.908 + 0.07 * k, 0.958], [0.932 + 0.07 * k, 0.958], [0.932 + 0.07 * k, 1.132], [0.908 + 0.07 * k, 1.132]] for k in range(10)]


def interior50():
    b = []
    for nm, x in (("front", 0.08), ("rear", -0.87)):
        b.append(rbox(f"seat-{nm}-back", CH, UP, 0.45, [0.2, 0.72, 0.96], [x, 1.14, 0], 0.03, role="detail"))
        b.append(rbox(f"seat-{nm}-cushion", CH, UP, 0.4, [0.44, 0.18, 0.96], [x + 0.22, 0.69, 0], 0.03, role="detail"))
    b.append(capsule("steering-column", CH, RM, 0.2, [0.54, 1.08, 0.28], [0.32, 1.37, 0.28], 0.018, role="detail"))
    b.append(revolve("steering-wheel", CH, RM, 0.15, "x", [0.31, 1.39, 0.28], cap_prof([[-0.016, 0.082], [0.016, 0.082], [0.016, 0.1], [-0.016, 0.1]]), role="detail"))
    return b



def by(lst):
    return {d["id"]: d for d in lst}


def thermal50(s):
    B = by(s.body)
    th = s.thermal

    def layered(ids, region, outer, inner, fx=None):
        for i in ids:
            f = (fx or {}).get(i)
            th.append(ref(f"th-{i}", i, region, outer, f))
            c = inset(B[i], 0.00008, f"th-{inner}-{i}")
            c["region"], c["layer"] = region, inner
            if c["kind"] == "loft" or c["kind"] == "box" or c["kind"] == "extrude":
                pass
            th.append(c)

    layered(["tub-side-l", "tub-side-r", "tub-rear", "tub-front", "cabin-side-l", "cabin-side-r", "scuttle", "cabin-rear-wall"], "body-panels", "paint", "sheet", {"tub-side-l": ["-w"], "tub-side-r": ["+w"], "tub-floor": ["+v"], "tub-front": ["-u"], "cabin-side-l": ["-w"], "cabin-side-r": ["+w"], "cabin-rear-wall": ["+u"], "tub-rear": ["+u"]})
    th.append(ref("th-roof-slab", "roof-slab", "roof", None, ["-v"]))
    layered(["hood-top", "hood-side-l", "hood-side-r", "hood-front-l", "hood-front-r"], "hood", "paint", "sheet", {"hood-top": ["-v"], "hood-side-l": ["-w"], "hood-side-r": ["+w"], "hood-front-l": ["-u"], "hood-front-r": ["-u"]})
    layered(["fender-front-l", "fender-front-r", "fender-rear-l", "fender-rear-r", "running-board-l", "running-board-r"], "fenders", "paint", "sheet")
    for i in ("window-front-l", "window-rear-l", "window-quarter-l"):
        th.append(ref(f"th-{i}", i, "glazing", None, ["-w"]))
    for i in ("window-front-r", "window-rear-r", "window-quarter-r"):
        th.append(ref(f"th-{i}", i, "glazing", None, ["+w"]))
    th.append(ref("th-windscreen", "windscreen", "glazing", None, ["-u"]))
    for sd in ("l", "r"):
        th.append(ref(f"th-headlamp-lens-{sd}", f"headlamp-lens-{sd}", "glazing", None, ["-u"]))
    for i in ("seat-front-back", "seat-front-cushion", "seat-rear-back", "seat-rear-cushion", "steering-column", "steering-wheel"):
        c = json.loads(json.dumps(B[i]))
        c["id"], c["region"] = f"th-{i}", "interior"
        th.append(c)
    for nm in ("front", "rear"):
        for sd in ("l", "r"):
            c = json.loads(json.dumps(B[f"handle-{nm}-{sd}"]))
            c["id"], c["region"] = f"th-handle-{nm}-{sd}", "interior"
            th.append(c)
    for i in ("radiator-shell",):
        th.append(ref(f"th-{i}", i, "radiator", None, ["-u"]))
    for i in ("radiator-neck", "radiator-cap"):
        th.append(ref(f"th-{i}", i, "radiator"))
    for i in ("engine-block", "engine-head", "engine-cover"):
        th.append(ref(f"th-{i}", i, "engine-bay"))
    for i in ("exhaust-manifold", "exhaust-downpipe", "exhaust-pipe", "silencer", "tail-pipe"):
        th.append(ref(f"th-{i}", i, "exhaust-system"))
    jet = revolve("tailpipe-jet", CH, EX, 0.9, [-1, 0, 0], [-2.1, 0.25, -0.35], cap_prof([[0, 0], [0, 0.0225], [0.008, 0.0225], [0.008, 0]]), role="detail")
    tag(jet, "tailpipe")
    faces(jet, "-u", "+u")
    jet["region"] = "exhaust-system"
    th.append(jet)
    for tg in ("fl", "fr", "rl", "rr"):
        th.append(ref(f"th-tyre-{tg}", f"tyre-{tg}", "tyres"))
        th.append(ref(f"th-drum-{tg}", f"drum-{tg}", "brakes"))
    th.append(ref("th-spare-tyre", "spare-tyre", "tyres"))


def hollow_tube(id, bone, mat, tone, y0, y1, z0, z1, x0, x1, wall):
    out = [[y0, z0], [y1, z0], [y1, z1], [y0, z1]]
    hole = [[y0 + wall, z0 + wall], [y1 - wall, z0 + wall], [y1 - wall, z1 - wall], [y0 + wall, z1 - wall]]
    return ext(id, bone, mat, tone, "yz", out, x0, x1, holes=[hole])


def structure50(s):
    B = by(s.body)
    st = s.structure
    for sd, sg in (("l", 1), ("r", -1)):
        z0, z1 = (0.355, 0.405) if sg > 0 else (-0.405, -0.355)
        s.st(hollow_tube(f"frame-tube-{sd}", CH, EX, 0.2, 0.355, 0.405, z0, z1, -1.95, 2.05, 0.005), CH, "member")
    for nm in ("front", "mid", "rear"):
        s.stof(f"crossmember-{nm}-member", f"crossmember-{nm}", CH, "member")
    s.stof("tub-floor-member", "tub-floor", CH, "member")
    for tg in ("f", "r"):
        bone = AF if tg == "f" else AR
        for nm in ("l", "r", "mid"):
            s.stof(f"axle-{tg}-{nm}-member", f"axle-{tg}-{nm}", bone, "member")
    for tg, wb in (("f", WF), ("r", WR)):
        for sd in ("l", "r"):
            s.stof(f"wheel-disc-{tg}{sd}-member", f"disc-{tg}{sd}", wb, "member")
            s.stof(f"wheel-rim-{tg}{sd}-member", f"rim-{tg}{sd}", wb, "member")
            sg = 1 if sd == "l" else -1
            s.st(revolve(f"wheel-bearing-{tg}{sd}", wb, CI, 0.2, "z", [0, 0, sg * ZC], [[r6(sg * -0.07), 0], [r6(sg * -0.07), 0.045], [r6(sg * 0.01), 0.045], [r6(sg * 0.01), 0]], role="detail"), wb, "joint")
    for tg, bone, x in (("f", AF, 1.5), ("r", AR, -1.5)):
        for sd, sg in (("l", 1), ("r", -1)):
            s.st(rbox(f"axle-clamp-{tg}{sd}", bone, EX, 0.2, [0.12, 0.06, 0.07], [0, 0, sg * 0.38], 0.005, role="detail"), bone, "joint")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.18, 0.14, 0.26], [-0.1, 0.72, 0], role="detail"), CH, "mass")
    for tg, wb in (("f", WF), ("r", WR)):
        s.st(ellipsoid(f"mass-wheels-{tg}", wb, EX, 0.2, [0.06, 0.03, 0.2], [0, 0, 0], role="detail"), wb, "mass")


def build50():
    s = Scale(50)
    s.body = body50() + interior50()
    return s


def interior50_noop():
    return []


def decisions50(s):
    s.dec("The car is built as a real body, not as a block. A ladder frame of two rails with cross members carries a hollow tub (floor, two 80 mm side walls, rear wall, firewall), a hollow cabin (two side frames whose window openings leave the pillars, a rear wall, a scuttle, a 70 mm fabric roof slab) and its own glazing: six side panes and a raked windscreen are separate glazing solids. The bonnet is a top and two side panels with ten real louvre slots; behind them stand the engine block, head and cover on engine bearers, an exhaust manifold and down pipe. Separate crescent wings follow the drawn arch band over each wheel in elevation and the drawn plan outline; running boards on brackets; headlamps with domed lenses on stalks; nickel radiator on a mount; two-bar bumpers on brackets; a side-mounted spare. Wheels are balloon tyre, rim, pressed disc with twelve spoke ribs, hub cap, brake drum and hub barrel, on a dropped axle that is cut where the frame rails pass. Seats, steering column and wheel stand inside the cabin.", "Der Wagen ist als echter Körper gebaut, nicht als Block. Ein Leiterrahmen aus zwei Längsträgern mit Querträgern trägt eine hohle Wanne (Boden, zwei 80 mm dicke Seitenwände, Heckwand, Spritzwand), eine hohle Kabine (zwei Seitenrahmen, deren Fensteröffnungen die Säulen stehen lassen, Rückwand, Windlauf, 70 mm Stoffdachplatte) und eigene Verglasung: sechs Seitenscheiben und eine geneigte Windschutzscheibe sind getrennte Glaskörper. Die Haube besteht aus Deckel und zwei Seitenblechen mit zehn echten Luftschlitzen; dahinter stehen Motorblock, Kopf und Deckel auf Motorlagern, Krümmer und Fallrohr. Getrennte Sichelkotflügel folgen dem gezeichneten Bogenband über jedem Rad in der Ansicht und dem Grundrissumriss; Trittbretter auf Konsolen; Scheinwerfer mit gewölbten Gläsern auf Stielen; vernickelter Kühler auf Halterung; zweistangige Stoßfänger auf Konsolen; ein seitlich montiertes Reserverad. Die Räder bestehen aus Ballonreifen, Felge, gepresster Scheibe mit zwölf Speichenrippen, Radkappe, Bremstrommel und Nabenhülse an einer abgesetzten Achse, die dort unterbrochen ist, wo die Längsträger verlaufen. Sitze, Lenksäule und Lenkrad stehen in der Kabine.")
    s.dec("Thermal: every body panel region is a nested shell pair (paint outside, steel sheet inside) on the same solid, with the inner faces of walls, floor, roof and panes declared as section faces because they belong to the cabin air, not to the exposed skin. Engine, exhaust parts and brake drums are real solids behind louvres, under the bonnet and inside the wheels; the interior region is a set of real hidden solids (seats, steering, door handles) and the tail pipe carries a thin jet disc tagged with its emitter.", "Thermisch: jede Blechregion ist ein verschachteltes Schalenpaar (Lack außen, Stahlblech innen) am selben Körper; die Innenflächen von Wänden, Boden, Dach und Scheiben sind als Schnittflächen deklariert, weil sie zur Kabinenluft gehören und nicht zur freien Haut. Motor, Abgasteile und Bremstrommeln sind echte Körper hinter den Schlitzen, unter der Haube und in den Rädern; die Region Innenraum besteht aus echten verdeckten Körpern (Sitze, Lenkung, Türgriffe), und das Auspuffende trägt eine dünne Strahlscheibe mit der Emitter-Kennung.")
    s.dec("Structure: the two frame rails are hollow 50 mm tubes with 5 mm walls inside the body rails, cross members, floor and the axle pieces are the body solids themselves, each wheel carries its disc and rim as member, a bearing as revolute joint and an axle clamp as fixed joint at the rails; compact masses sit at the body centre of mass and at the wheel axes.", "Struktur: die beiden Rahmenlängsträger sind hohle 50-mm-Rohre mit 5 mm Wand innerhalb der Körper-Längsträger; Querträger, Boden und Achsstücke sind die Körper selbst, jedes Rad trägt Scheibe und Felge als Stab, ein Lager als Drehgelenk und eine Achsklemme als festes Gelenk an den Längsträgern; kompakte Massen sitzen im Schwerpunkt des Aufbaus und auf den Radachsen.")



def wheel100(bone, tag, sgn):
    sx = lambda prof: [[r6(sgn * a), r] for a, r in prof]
    o = [0, 0, sgn * ZC]
    out = [revolve(f"tyre-{tag}", bone, RU, 0.06, "z", o, sx(tyre_profile(0.25, 0.37, 0.0575, 0.02)))]
    out.append(revolve(f"wheel-{tag}", bone, RM, 0.52, "z", o, sx(cap_prof([[-0.0425, 0.252], [0.0425, 0.252], [0.0425, 0.215], [0.012, 0.215], [0.012, 0], [-0.012, 0], [-0.012, 0.215], [-0.0425, 0.215]])), role="detail"))
    rib = box(f"spokes-{tag}", bone, RM, 0.62, [0.17, 0.032, 0.016], [0.15, 0, sgn * (ZC + 0.02)], role="detail")
    rib["repeat"] = {"count": 8, "rotationRad": r6(2 * math.pi / 8), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}
    out.append(rib)
    out.append(revolve(f"hub-{tag}", bone, NI, 0.85, "z", o, sx(cap_prof([[0.008, 0], [0.008, 0.07], [0.085, 0.066], [0.0965, 0.04], [0.0965, 0]])), role="detail"))
    out.append(revolve(f"drum-{tag}", bone, CI, 0.3, "z", o, sx(cap_prof([[-0.0575, 0], [-0.0575, 0.15], [-0.01, 0.15], [-0.01, 0.05], [0.012, 0.05], [0.012, 0]])), role="detail"))
    return out


def body100():
    sc = 100
    b = []
    tub = dedup(R_(sc, "tub"))
    fr = dedup(R_(sc, "frame"))
    b.append(ext("frame-rail-l", CH, P, 0.15, "xy", fr, 0.35, 0.41))
    b.append(ext("frame-rail-r", CH, P, 0.15, "xy", fr, -0.41, -0.35))
    for nm, x in (("front", 1.75), ("rear", -1.9)):
        b.append(rbox(f"crossmember-{nm}", CH, P, 0.15, [0.05, 0.06, 0.70], [x, 0.38, 0], 0.005))
    for nm, x in (("a", -1.0), ("b", 0.3)):
        for s_, sg in (("l", 1), ("r", -1)):
            b.append(rbox(f"body-mount-{nm}-{s_}", CH, P, 0.15, [0.12, 0.11, 0.07], [x, 0.465, sg * 0.38], 0.004))
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"tub-side-{s_}", CH, P, 0.25, "xy", tub, 0.54 if sg > 0 else -0.62, 0.62 if sg > 0 else -0.54))
    b.append(ext("tub-rear", CH, P, 0.25, "xy", dedup(clip_box(tub, -1.99, -1.80, 0.0, 2.0)), -0.54, 0.54))
    b.append(xloft("tub-floor", CH, P, 0.2, [(-1.8, 0.52, 0.6, -0.54, 0.54), (0.52, 0.52, 0.6, -0.54, 0.54)], e=10))
    b.append(rbox("tub-front", CH, P, 0.25, [0.08, 0.62, 1.08], [0.56, 0.83, 0], 0.006))
    cab = dedup(clip_box(R_(sc, "cabin"), -1.9, 0.7, 1.14, 1.64))
    holes = [[[-0.27, 1.21], [0.50, 1.21], [0.50, 1.58], [-0.27, 1.58]], [[-1.07, 1.21], [-0.41, 1.21], [-0.41, 1.58], [-1.07, 1.58]], [[-1.17, 1.21], [-1.17, 1.58], [-1.62, 1.58], [-1.74, 1.21]]]
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"cabin-side-{s_}", CH, P, 0.25, "xy", cab, 0.52 if sg > 0 else -0.6, 0.6 if sg > 0 else -0.52, holes=holes))
        z0, z1 = (0.594, 0.6) if sg > 0 else (-0.6, -0.594)
        for nm, h in (("front", holes[0]), ("rear", holes[1]), ("quarter", holes[2])):
            b.append(ext(f"window-{nm}-{s_}", CH, GL, 0.85, "xy", h, z0, z1, role="glazing"))
    b.append(rbox("scuttle", CH, P, 0.25, [0.07, 0.07, 1.04], [0.5585, 1.175, 0], 0.004))
    b.append(ext("cabin-rear-wall", CH, P, 0.25, "xy", [[Rk(1.14), 1.14], [Rk(1.14) + 0.07, 1.14], [Rk(1.6) + 0.07, 1.6], [Rk(1.6), 1.6]], -0.52, 0.52))
    ws_len = math.hypot(Fk(1.6) - Fk(1.21), 1.6 - 1.21)
    b.append(box("windscreen", CH, GL, 0.85, [0.012, r6(ws_len), 1.04], [r6((Fk(1.21) + Fk(1.6)) / 2 - 0.012), 1.405, 0], None, "glazing", Qz(math.atan2(Fk(1.21) - Fk(1.6), 0.39))))
    rr, rp = R_(sc, "roof-slab"), PL_(sc, "roof-slab")
    rows = []
    for x in (-1.698, -1.65, -1.55, -1.5, 0.0, 0.58, 0.618):
        a, c = band(rr, x)
        h = span(rp, x)[1]
        rows.append((x, a, c, -h, h, 6))
    b.append(xloft("roof-slab", CH, FR, 0.78, rows))
    hr, hp = dedup(R_(sc, "hood")), PL_(sc, "hood")
    rows = []
    for x in (0.622, 0.7, 0.9, 1.2, 1.5, 1.7, 1.778):
        yt = band(hr, x)[1]
        h = span(hp, x)[1]
        rows.append((x, yt - 0.025, yt, -h, h, 8))
    b.append(xloft("hood-top", CH, P, 0.2, rows))
    holes6 = [[[0.93 + 0.12 * k, 0.95], [0.97 + 0.12 * k, 0.95], [0.97 + 0.12 * k, 1.14], [0.93 + 0.12 * k, 1.14]] for k in range(6)]
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"hood-side-{s_}", CH, P, 0.2, "xy", hr, 0.32 if sg > 0 else -0.34, 0.34 if sg > 0 else -0.32, holes=holes6))
        v = box(f"louvre-vanes-{s_}", CH, P, 0.3, [0.06, 0.19, 0.016], [0.95, 1.045, sg * 0.325], None, "detail", Qy(sg * math.pi / 4))
        v["repeat"] = {"count": 6, "translation": [0.12, 0, 0]}
        b.append(v)
        b.append(rbox(f"hood-front-{s_}", CH, P, 0.2, [0.02, 0.56, 0.12], [1.77, 0.86, sg * 0.28], 0.004))
    b.append(rbox("radiator-shell", CH, NI, 0.85, [0.1, 0.7, 0.44], [1.83, 0.91, 0], 0.03))
    b.append(capsule("radiator-neck", CH, NI, 0.85, [1.83, 1.26, 0], [1.83, 1.30, 0], 0.018, role="detail"))
    b.append(ellipsoid("radiator-cap", CH, NI, 0.85, [0.05, 0.032, 0.05], [1.83, 1.335, 0], role="detail"))
    b.append(rbox("radiator-mount", CH, P, 0.15, [0.06, 0.15, 0.2], [1.79, 0.485, 0], 0.004))
    b.append(rbox("engine-block", CH, EM, 0.35, [0.62, 0.30, 0.28], [1.23, 0.73, 0], 0.02, role="detail"))
    b.append(rbox("engine-head", CH, EM, 0.3, [0.46, 0.24, 0.24], [1.23, 0.97, 0], 0.015, role="detail"))
    b.append(rbox("gearbox", CH, EM, 0.3, [0.27, 0.34, 0.30], [0.735, 0.75, 0], 0.02, role="detail"))
    for k, x in enumerate((0.95, 1.45)):
        for s_, sg in (("l", 1), ("r", -1)):
            b.append(rbox(f"engine-bearer-{k + 1}-{s_}", CH, P, 0.15, [0.1, 0.17, 0.22], [x, 0.495, sg * 0.28], 0.004, role="detail"))
    b.append(capsule("exhaust-manifold", CH, EX, 0.3, [0.95, 0.84, -0.18], [1.5, 0.84, -0.18], 0.03, role="detail"))
    b.append(capsule("exhaust-downpipe", CH, EX, 0.3, [0.95, 0.84, -0.18], [0.93, 0.28, -0.3], 0.027, role="detail"))
    b.append(capsule("exhaust-pipe", CH, EX, 0.3, [-1.05, 0.27, -0.3], [0.9, 0.27, -0.3], 0.03, role="detail"))
    b.append(capsule("silencer", CH, EX, 0.4, [-1.75, 0.27, -0.3], [-1.10, 0.27, -0.3], 0.075, role="detail"))
    b.append(capsule("tail-pipe", CH, EX, 0.3, [-1.75, 0.27, -0.35], [-2.08, 0.25, -0.35], 0.032, 0.0275, role="detail"))
    for k, x in enumerate((-1.5, 0.5)):
        b.append(rbox(f"exhaust-hanger-{k + 1}", CH, EX, 0.3, [0.03, 0.08, 0.05], [x, 0.325, -0.325], 0.004, role="detail"))
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(fender_loft100(f"fender-front-{s_}", sg, "f"))
        b.append(fender_loft100(f"fender-rear-{s_}", sg, "r"))
        b.append(box(f"running-board-{s_}", CH, P, 0.4, [2.0, 0.06, 0.18], [0, 0.43, sg * 0.71], 0.015))
        for k, x in enumerate((-0.8, 0.8)):
            b.append(rbox(f"running-bracket-{k + 1}-{s_}", CH, P, 0.15, [0.06, 0.08, 0.27], [x, 0.37, sg * 0.545], 0.004, role="detail"))
        b.append(ellipsoid(f"headlamp-bowl-{s_}", CH, NI, 0.85, [0.105, 0.11, 0.11], [1.78, 0.99, sg * 0.55]))
        b.append(revolve(f"headlamp-lens-{s_}", CH, GL, 0.9, "x", [1.848, 0.99, sg * 0.55], cap_prof([[0, 0], [0, 0.085], [0.02, 0.07], [0.036, 0]]), role="glazing"))
        b.append(capsule(f"lamp-stalk-{s_}", CH, NI, 0.85, [1.76, 0.80, sg * 0.55], [1.76, 0.90, sg * 0.55], 0.0175, role="detail"))
        c = capsule(f"bracket-front-{s_}", CH, EX, 0.2, [2.2, 0.5, sg * 0.3], [2.05, 0.43, sg * 0.3], 0.012, role="detail")
        c["lateralScale"] = 1.67
        b.append(c)
        c = capsule(f"bracket-rear-{s_}", CH, EX, 0.2, [-2.1, 0.5, sg * 0.3], [-1.95, 0.43, sg * 0.3], 0.012, role="detail")
        c["lateralScale"] = 1.67
        b.append(c)
    b.append(capsule("fender-lamp", CH, NI, 0.85, [2.0, 0.66, 0.62], [2.1, 0.58, 0.62], 0.02, role="detail"))
    for nm, x in (("front", 2.22), ("rear", -2.12)):
        b.append(rbox(f"bumper-{nm}", CH, NI, 0.8, [0.044, 0.306, 1.4], [x, 0.531, 0], 0.02))
    b.append(rbox("tail-lamp", CH, P, 0.5, [0.05, 0.1, 0.1], [-2.005, 0.95, 0.4], 0.008, role="detail"))
    so = [0.86, 0.78, -0.5575]
    b.append(revolve("spare-tyre", CH, RU, 0.06, "z", so, tyre_profile(0.23, 0.31, 0.0575, 0.02)))
    b.append(revolve("spare-cover", CH, P, 0.4, "z", [0.86, 0.78, -0.6], cap_prof([[0, 0], [0, 0.23], [0.015, 0.23], [0.015, 0]]), role="detail"))
    b.append(capsule("spare-mount", CH, EX, 0.2, [0.86, 0.78, -0.34], [0.86, 0.78, -0.5], 0.02, role="detail"))
    for bone, tag in ((AF, "f"), (AR, "r")):
        b.extend(axle_solids(bone, tag))
    for bone, tag in ((WF, "f"), (WR, "r")):
        for s_, sg in (("l", 1), ("r", -1)):
            b.extend(wheel100(bone, f"{tag}{s_}", sg))
    return b


def fender_loft100(id, side, which):
    ring = R_(100, "fender-front" if which == "f" else "fender-rear")
    pr = PL_(100, ("fender-f" if which == "f" else "fender-r") + ("l" if side > 0 else "r"))
    xs = [0.962, 1.0, 1.1, 1.3, 1.5, 1.7, 1.9, 2.05, 2.136] if which == "f" else [-2.026, -1.95, -1.8, -1.6, -1.4, -1.2, -1.05, -0.982]
    rows = []
    for x in xs:
        y0, y1 = band(ring, x)
        z0, z1 = span(pr, x)
        if side < 0 and which == "f" and x <= 1.17:
            z1 = -0.62
        rows.append((x, y0, y1, z0, z1, 5))
    return xloft(id, CH, P, 0.22, rows)


def interior100():
    b = []
    for nm, pts in (("front", [(-0.02, 0.6), (0.52, 0.6), (0.52, 0.78), (0.18, 0.78), (0.18, 1.5), (-0.02, 1.5)]), ("rear", [(-0.97, 0.6), (-0.33, 0.6), (-0.33, 0.78), (-0.77, 0.78), (-0.77, 1.5), (-0.97, 1.5)])):
        b.append(ext(f"seat-{nm}", CH, UP, 0.42, "xy", [list(p) for p in pts], -0.48, 0.48, role="detail"))
    b.append(capsule("steering-column", CH, RM, 0.2, [0.54, 1.08, 0.28], [0.32, 1.37, 0.28], 0.018, role="detail"))
    return b


def thermal100(s):
    B = by(s.body)
    th = s.thermal
    for i in ["tub-side-l", "tub-side-r", "tub-rear", "tub-front", "cabin-side-l", "cabin-side-r", "scuttle", "cabin-rear-wall"]:
        th.append(ref(f"th-{i}", i, "body-panels", None, {"tub-side-l": ["-w"], "tub-side-r": ["+w"], "tub-front": ["-u"], "cabin-side-l": ["-w"], "cabin-side-r": ["+w"], "cabin-rear-wall": ["+u"], "tub-rear": ["+u"]}.get(i)))
    th.append(ref("th-roof-slab", "roof-slab", "roof", None, ["-v"]))
    for i in ["hood-top", "hood-side-l", "hood-side-r", "hood-front-l", "hood-front-r"]:
        th.append(ref(f"th-{i}", i, "hood", None, {"hood-top": ["-v"], "hood-side-l": ["-w"], "hood-side-r": ["+w"], "hood-front-l": ["-u"], "hood-front-r": ["-u"]}.get(i)))
    for i in ["fender-front-l", "fender-front-r", "fender-rear-l", "fender-rear-r", "running-board-l", "running-board-r"]:
        th.append(ref(f"th-{i}", i, "fenders"))
    for i in ("window-front-l", "window-rear-l", "window-quarter-l"):
        th.append(ref(f"th-{i}", i, "glazing", None, ["-w"]))
    for i in ("window-front-r", "window-rear-r", "window-quarter-r"):
        th.append(ref(f"th-{i}", i, "glazing", None, ["+w"]))
    th.append(ref("th-windscreen", "windscreen", "glazing", None, ["-u"]))
    for sd in ("l", "r"):
        th.append(ref(f"th-headlamp-lens-{sd}", f"headlamp-lens-{sd}", "glazing", None, ["-u"]))
    for i in ("seat-front", "seat-rear", "steering-column"):
        c = json.loads(json.dumps(B[i]))
        c["id"], c["region"] = f"th-{i}", "interior"
        th.append(c)
    wheel = revolve("th-steering-wheel", CH, RM, 0.15, "x", [0.31, 1.39, 0.28], cap_prof([[-0.016, 0.082], [0.016, 0.082], [0.016, 0.1], [-0.016, 0.1]]), role="detail")
    wheel["region"] = "interior"
    th.append(wheel)
    th.append(ref("th-radiator-shell", "radiator-shell", "radiator", None, ["-u"]))
    for i in ("radiator-neck", "radiator-cap"):
        th.append(ref(f"th-{i}", i, "radiator"))
    for i in ("engine-block", "engine-head"):
        th.append(ref(f"th-{i}", i, "engine-bay"))
    for i in ("exhaust-manifold", "exhaust-downpipe", "exhaust-pipe", "silencer", "tail-pipe"):
        th.append(ref(f"th-{i}", i, "exhaust-system"))
    jet = revolve("tailpipe-jet", CH, EX, 0.9, [-1, 0, 0], [-2.1, 0.25, -0.35], cap_prof([[0, 0], [0, 0.0225], [0.015, 0.0225], [0.015, 0]]), role="detail")
    tag(jet, "tailpipe")
    faces(jet, "-u", "+u")
    jet["region"] = "exhaust-system"
    th.append(jet)
    for tg in ("fl", "fr", "rl", "rr"):
        th.append(ref(f"th-tyre-{tg}", f"tyre-{tg}", "tyres"))
        th.append(ref(f"th-drum-{tg}", f"drum-{tg}", "brakes"))
    th.append(ref("th-spare-tyre", "spare-tyre", "tyres"))


def structure100(s):
    st = s.structure
    for sd, sg in (("l", 1), ("r", -1)):
        z0, z1 = (0.355, 0.405) if sg > 0 else (-0.405, -0.355)
        s.st(hollow_tube(f"frame-tube-{sd}", CH, EX, 0.2, 0.352, 0.408, z0 - 0.003, z1 + 0.003, -1.96, 2.06, 0.008), CH, "member")
    for nm in ("front", "rear"):
        s.stof(f"crossmember-{nm}-member", f"crossmember-{nm}", CH, "member")
    for tg in ("f", "r"):
        bone = AF if tg == "f" else AR
        for nm in ("l", "r", "mid"):
            s.stof(f"axle-{tg}-{nm}-member", f"axle-{tg}-{nm}", bone, "member")
    for tg, wb in (("f", WF), ("r", WR)):
        for sd in ("l", "r"):
            s.stof(f"wheel-{tg}{sd}-member", f"wheel-{tg}{sd}", wb, "member")
    for tg, bone in (("f", AF), ("r", AR)):
        for sd, sg in (("l", 1), ("r", -1)):
            s.st(rbox(f"axle-clamp-{tg}{sd}", bone, EX, 0.2, [0.12, 0.06, 0.07], [0, 0, sg * 0.38], 0.005, role="detail"), bone, "joint")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.18, 0.14, 0.26], [-0.1, 0.72, 0], role="detail"), CH, "mass")
    for tg, wb in (("f", WF), ("r", WR)):
        s.st(ellipsoid(f"mass-wheels-{tg}", wb, EX, 0.2, [0.06, 0.03, 0.2], [0, 0, 0], role="detail"), wb, "mass")


def decisions100(s):
    s.dec("The 1:100 car keeps the real-body architecture but at 15 mm detail: frame rails with two cross members and two body mounts per side, hollow tub and cabin with the three window openings and glazing, roof slab, bonnet top and sides with the six louvre slots the 1:100 drawing shows, each slot closed in elevation by a tilted vane, engine block and one head block behind them; wings from nine stations; each wheel is a tyre, one T-section rim and disc solid with eight spoke ribs, a hub cap and a drum with barrel; door handles, number plate, seats as separate parts, steering wheel and the valve are not drawn at this scale and are left out of the body (the hidden interior keeps two seat solids and the column in thermal).", "Der Wagen 1:100 behält den Aufbau als echter Körper bei 15 mm Detail: Rahmenlängsträger mit zwei Querträgern und je zwei Karosserielagern, hohle Wanne und Kabine mit den drei Fensteröffnungen und Verglasung, Dachplatte, Haubendeckel und -seiten mit den sechs Luftschlitzen der Zeichnung 1:100, jeder Schlitz in der Ansicht durch eine geneigte Lamelle geschlossen, dahinter Motorblock und ein Kopfblock; Kotflügel aus neun Stationen; jedes Rad ist ein Reifen, ein T-förmiger Felgen-Scheiben-Körper mit acht Speichenrippen, eine Radkappe und eine Trommel mit Hülse; Türgriffe, Nummernschild, Sitze als eigene Teile, Lenkrad und Ventil werden bei diesem Maßstab nicht gezeichnet und fehlen im Körper (der verdeckte Innenraum behält zwei Sitzkörper und die Säule im Thermikmodell).")
    s.dec("Thermal at 1:100 is a single layer of references per region (nested layers belong to 1:50); the inner faces of walls, roof and panes are section faces; engine, exhaust, drums are real bodies; the interior keeps two L-shaped seat solids, the column and the wheel ring as hidden real solids; the tail pipe disc carries the emitter id.", "Die Thermik bei 1:100 ist je Region eine Schicht aus Verweisen (verschachtelte Schichten gehören zu 1:50); die Innenflächen von Wänden, Dach und Scheiben sind Schnittflächen; Motor, Abgas und Trommeln sind echte Körper; der Innenraum behält zwei L-förmige Sitzkörper, die Säule und den Lenkradring als verdeckte echte Körper; die Auspuffscheibe trägt die Emitter-Kennung.")
    s.dec("Structure at 1:100: two hollow frame tubes, two cross members, the three axle pieces per axle, one wheel solid per wheel as member, axle clamps as joints and compact masses; wheel bearings, the floor and the third cross member of 1:50 are merged away.", "Struktur bei 1:100: zwei hohle Rahmenrohre, zwei Querträger, die drei Achsstücke je Achse, ein Radkörper je Rad als Stab, Achsklemmen als Gelenke und kompakte Massen; Radlager, Boden und der dritte Querträger von 1:50 sind zusammengefasst.")


def wheel200(bone, tag, sgn):
    sx = lambda prof: [[r6(sgn * a), r] for a, r in prof]
    o = [0, 0, sgn * ZC]
    out = [revolve(f"tyre-{tag}", bone, RU, 0.06, "z", o, sx(cap_prof([[-0.0575, 0.27], [-0.0575, 0.34], [-0.03, 0.37], [0.03, 0.37], [0.0575, 0.34], [0.0575, 0.27], [0.04, 0.25], [-0.04, 0.25]])))]
    out.append(revolve(f"wheel-{tag}", bone, RM, 0.52, "z", o, sx(cap_prof([[-0.0425, 0.252], [0.0425, 0.252], [0.0425, 0.215], [0.015, 0.215], [0.015, 0], [-0.015, 0], [-0.015, 0.215], [-0.0425, 0.215]])), role="detail"))
    rib = box(f"spokes-{tag}", bone, RM, 0.62, [0.43, 0.06, 0.03], [0, 0, sgn * (ZC + 0.03)], role="detail")
    rib["repeat"] = {"count": 2, "rotationRad": r6(math.pi / 2), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}
    out.append(rib)
    out.append(revolve(f"hub-{tag}", bone, NI, 0.85, "z", o, sx(cap_prof([[0.015, 0], [0.015, 0.07], [0.085, 0.066], [0.0965, 0.04], [0.0965, 0]])), role="detail"))
    return out


def body200():
    sc = 200
    b = []
    tub = dedup(R_(sc, "tub"))
    fr = dedup(R_(sc, "frame"))
    b.append(ext("frame-slab", CH, P, 0.15, "xy", fr, -0.41, 0.41))
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(rbox(f"body-mount-{s_}", CH, P, 0.15, [0.3, 0.11, 0.07], [-0.4, 0.465, sg * 0.38], 0.004))
        b.append(ext(f"tub-side-{s_}", CH, P, 0.25, "xy", tub, 0.54 if sg > 0 else -0.62, 0.62 if sg > 0 else -0.54))
    b.append(ext("tub-rear", CH, P, 0.25, "xy", dedup(clip_box(tub, -1.99, -1.80, 0.0, 2.0)), -0.54, 0.54))
    b.append(xloft("tub-floor", CH, P, 0.2, [(-1.8, 0.52, 0.6, -0.54, 0.54), (0.52, 0.52, 0.6, -0.54, 0.54)], e=10))
    b.append(rbox("tub-front", CH, P, 0.25, [0.08, 0.62, 1.08], [0.56, 0.83, 0], 0.006))
    cab = dedup(clip_box(R_(sc, "cabin"), -1.9, 0.7, 1.14, 1.64))
    holes = [[[-0.27, 1.21], [0.50, 1.21], [0.50, 1.58], [-0.27, 1.58]], [[-1.07, 1.21], [-0.41, 1.21], [-0.41, 1.58], [-1.07, 1.58]], [[-1.17, 1.21], [-1.17, 1.58], [-1.62, 1.58], [-1.74, 1.21]]]
    pane = [[-1.74, 1.21], [0.5, 1.21], [0.5, 1.58], [-1.62, 1.58]]
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"cabin-side-{s_}", CH, P, 0.25, "xy", cab, 0.52 if sg > 0 else -0.6, 0.6 if sg > 0 else -0.52, holes=holes))
        b.append(ext(f"window-{s_}", CH, GL, 0.85, "xy", pane, 0.55 if sg > 0 else -0.56, 0.56 if sg > 0 else -0.55, role="glazing"))
    b.append(rbox("scuttle", CH, P, 0.25, [0.07, 0.07, 1.04], [0.5585, 1.175, 0], 0.004))
    b.append(ext("cabin-rear-wall", CH, P, 0.25, "xy", [[Rk(1.14), 1.14], [Rk(1.14) + 0.07, 1.14], [Rk(1.6) + 0.07, 1.6], [Rk(1.6), 1.6]], -0.52, 0.52))
    ws_len = math.hypot(Fk(1.6) - Fk(1.21), 1.6 - 1.21)
    b.append(box("windscreen", CH, GL, 0.85, [0.03, r6(ws_len), 1.04], [r6((Fk(1.21) + Fk(1.6)) / 2 - 0.02), 1.405, 0], None, "glazing", Qz(math.atan2(Fk(1.21) - Fk(1.6), 0.39))))
    rr, rp = R_(sc, "roof-slab"), PL_(sc, "roof-slab")
    rows = []
    for x in (-1.698, -1.55, -1.5, 0.0, 0.618):
        a, c = band(rr, x)
        h = span(rp, x)[1]
        rows.append((x, a, c, -h, h, 6))
    b.append(xloft("roof-slab", CH, FR, 0.78, rows))
    hr, hp = dedup(R_(sc, "hood")), PL_(sc, "hood")
    rows = []
    for x in (0.622, 0.9, 1.5, 1.778):
        yt = band(hr, x)[1]
        h = span(hp, x)[1]
        rows.append((x, yt - 0.03, yt, -h, min(h, 0.34), 8))
    b.append(xloft("hood-top", CH, P, 0.2, rows))
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"hood-side-{s_}", CH, P, 0.2, "xy", hr, 0.31 if sg > 0 else -0.34, 0.34 if sg > 0 else -0.31))
        b.append(rbox(f"hood-front-{s_}", CH, P, 0.2, [0.03, 0.56, 0.12], [1.765, 0.86, sg * 0.28], 0.004))
    b.append(rbox("radiator-shell", CH, NI, 0.85, [0.1, 0.7, 0.44], [1.83, 0.91, 0], 0.03))
    b.append(capsule("radiator-neck", CH, NI, 0.85, [1.83, 1.26, 0], [1.83, 1.30, 0], 0.0185, role="detail"))
    b.append(ellipsoid("radiator-cap", CH, NI, 0.85, [0.05, 0.032, 0.05], [1.83, 1.335, 0], role="detail"))
    b.append(capsule("exhaust-pipe", CH, EX, 0.3, [-1.05, 0.27, -0.3], [0.9, 0.27, -0.3], 0.03, role="detail"))
    b.append(capsule("silencer", CH, EX, 0.4, [-1.75, 0.27, -0.3], [-1.10, 0.27, -0.3], 0.075, role="detail"))
    b.append(capsule("tail-pipe", CH, EX, 0.3, [-1.75, 0.27, -0.35], [-2.08, 0.25, -0.35], 0.032, 0.0275, role="detail"))
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(fender_loft200(f"fender-front-{s_}", sg, "f"))
        b.append(fender_loft200(f"fender-rear-{s_}", sg, "r"))
        b.append(box(f"running-board-{s_}", CH, P, 0.4, [2.0, 0.06, 0.18], [0, 0.43, sg * 0.71], 0.015))
        b.append(rbox(f"running-bracket-{s_}", CH, P, 0.15, [0.1, 0.08, 0.27], [0.0, 0.37, sg * 0.545], 0.004, role="detail"))
        b.append(ellipsoid(f"headlamp-bowl-{s_}", CH, NI, 0.85, [0.105, 0.11, 0.11], [1.78, 0.99, sg * 0.55]))
        b.append(revolve(f"headlamp-lens-{s_}", CH, GL, 0.9, "x", [1.848, 0.99, sg * 0.55], cap_prof([[0, 0], [0, 0.085], [0.036, 0]]), role="glazing"))
        b.append(capsule(f"lamp-stalk-{s_}", CH, NI, 0.85, [1.76, 0.80, sg * 0.55], [1.76, 0.90, sg * 0.55], 0.0175, role="detail"))
        b.append(rbox(f"bracket-front-{s_}", CH, EX, 0.2, [0.17, 0.05, 0.05], [2.125, 0.465, sg * 0.3], 0.01, role="detail"))
        b.append(rbox(f"bracket-rear-{s_}", CH, EX, 0.2, [0.17, 0.05, 0.05], [-2.025, 0.465, sg * 0.3], 0.01, role="detail"))
    for nm, x in (("front", 2.22), ("rear", -2.12)):
        b.append(rbox(f"bumper-{nm}", CH, NI, 0.8, [0.044, 0.306, 1.4], [x, 0.531, 0], 0.02))
    b.append(rbox("tail-lamp", CH, P, 0.5, [0.05, 0.1, 0.1], [-2.005, 0.95, 0.4], 0.008, role="detail"))
    so = [0.86, 0.78, -0.5575]
    b.append(revolve("spare-tyre", CH, RU, 0.06, "z", so, cap_prof([[-0.0575, 0.25], [-0.0575, 0.28], [-0.03, 0.31], [0.03, 0.31], [0.0575, 0.28], [0.0575, 0.25]])))
    b.append(revolve("spare-cover", CH, P, 0.4, "z", [0.86, 0.78, -0.6], cap_prof([[0, 0], [0, 0.23], [0.03, 0.23], [0.03, 0]]), role="detail"))
    b.append(capsule("spare-mount", CH, EX, 0.2, [0.86, 0.78, -0.34], [0.86, 0.78, -0.5], 0.02, role="detail"))
    for bone, tag in ((AF, "f"), (AR, "r")):
        b.extend(axle_solids(bone, tag))
    for bone, tag in ((WF, "f"), (WR, "r")):
        for s_, sg in (("l", 1), ("r", -1)):
            b.extend(wheel200(bone, f"{tag}{s_}", sg))
    return b


def fender_loft200(id, side, which):
    ring = R_(200, "fender-front" if which == "f" else "fender-rear")
    pr = PL_(200, ("fender-f" if which == "f" else "fender-r") + ("l" if side > 0 else "r"))
    xs = [0.97, 1.1, 1.5, 1.9, 2.1, 2.13] if which == "f" else [-2.02, -1.9, -1.5, -1.1, -0.99]
    rows = []
    for x in xs:
        y0, y1 = band(ring, x, 0.06)
        z0, z1 = span(pr, x)
        if side < 0 and which == "f" and x <= 1.17:
            z1 = -0.62
        rows.append((x, y0, y1, z0, z1, 5))
    return xloft(id, CH, P, 0.22, rows)


def interior200():
    return []


def thermal200(s):
    B = by(s.body)
    th = s.thermal
    for i in ["tub-side-l", "tub-side-r", "tub-rear", "tub-front", "cabin-side-l", "cabin-side-r", "scuttle", "cabin-rear-wall"]:
        th.append(ref(f"th-{i}", i, "body-panels", None, {"tub-side-l": ["-w"], "tub-side-r": ["+w"], "tub-front": ["-u"], "cabin-side-l": ["-w"], "cabin-side-r": ["+w"], "cabin-rear-wall": ["+u"], "tub-rear": ["+u"]}.get(i)))
    th.append(ref("th-roof-slab", "roof-slab", "roof", None, ["-v"]))
    for i in ["hood-top", "hood-side-l", "hood-side-r"]:
        th.append(ref(f"th-{i}", i, "hood", None, {"hood-top": ["-v"], "hood-side-l": ["-w"], "hood-side-r": ["+w"]}.get(i)))
    for i in ["fender-front-l", "fender-front-r", "fender-rear-l", "fender-rear-r", "running-board-l", "running-board-r"]:
        th.append(ref(f"th-{i}", i, "fenders"))
    th.append(ref("th-window-l", "window-l", "glazing", None, ["-w"]))
    th.append(ref("th-window-r", "window-r", "glazing", None, ["+w"]))
    th.append(ref("th-windscreen", "windscreen", "glazing", None, ["-u"]))
    for sd in ("l", "r"):
        th.append(ref(f"th-headlamp-lens-{sd}", f"headlamp-lens-{sd}", "glazing", None, ["-u"]))
    for nm, x0, x1 in (("front", -0.02, 0.52), ("rear", -0.97, -0.33)):
        d = ext(f"th-seat-{nm}", CH, UP, 0.42, "xy", [[x0, 0.6], [x1, 0.6], [x1, 0.78], [x0 + 0.2, 0.78], [x0 + 0.2, 1.5], [x0, 1.5]], -0.48, 0.48, role="detail")
        d["region"] = "interior"
        th.append(d)
    d = capsule("th-steering", CH, RM, 0.2, [0.54, 1.08, 0.28], [0.31, 1.39, 0.28], 0.05, role="detail")
    d["region"] = "interior"
    th.append(d)
    th.append(ref("th-radiator-shell", "radiator-shell", "radiator", None, ["-u"]))
    for i in ("radiator-neck", "radiator-cap"):
        th.append(ref(f"th-{i}", i, "radiator"))
    d = rbox("th-engine", CH, EM, 0.35, [0.62, 0.42, 0.28], [1.23, 0.79, 0], 0.02, role="detail")
    d["region"] = "engine-bay"
    th.append(d)
    for i in ("exhaust-pipe", "silencer", "tail-pipe"):
        th.append(ref(f"th-{i}", i, "exhaust-system"))
    d = capsule("th-exhaust-manifold", CH, EX, 0.3, [0.93, 0.8, -0.18], [1.5, 0.84, -0.18], 0.03, role="detail")
    d["region"] = "exhaust-system"
    th.append(d)
    jet = revolve("tailpipe-jet", CH, EX, 0.9, [-1, 0, 0], [-2.1, 0.25, -0.35], cap_prof([[0, 0], [0, 0.0225], [0.03, 0.0225], [0.03, 0]]), role="detail")
    tag(jet, "tailpipe")
    faces(jet, "-u", "+u")
    jet["region"] = "exhaust-system"
    th.append(jet)
    for tg, wb in (("fl", WF), ("fr", WF), ("rl", WR), ("rr", WR)):
        th.append(ref(f"th-tyre-{tg}", f"tyre-{tg}", "tyres"))
        sg = 1 if tg[1] == "l" else -1
        d = revolve(f"th-drum-{tg}", wb, CI, 0.3, "z", [0, 0, sg * ZC], [[r6(sg * -0.0575), 0], [r6(sg * -0.0575), 0.15], [r6(sg * -0.01), 0.15], [r6(sg * -0.01), 0], [r6(sg * -0.0575), 0]], role="detail")
        d["region"] = "brakes"
        th.append(d)
    th.append(ref("th-spare-tyre", "spare-tyre", "tyres"))


def structure200(s):
    for sd, sg in (("l", 1), ("r", -1)):
        d = rbox(f"frame-rail-{sd}", CH, EX, 0.2, [3.96, 0.05, 0.05], [0.05, 0.38, sg * 0.38], 0.005, role="detail")
        s.st(d, CH, "member")
    for tg in ("f", "r"):
        bone = AF if tg == "f" else AR
        for nm in ("l", "r", "mid"):
            s.stof(f"axle-{tg}-{nm}-member", f"axle-{tg}-{nm}", bone, "member")
    for tg, wb in (("f", WF), ("r", WR)):
        for sd in ("l", "r"):
            s.stof(f"wheel-{tg}{sd}-member", f"wheel-{tg}{sd}", wb, "member")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.2, 0.15, 0.28], [-0.1, 0.72, 0], role="detail"), CH, "mass")
    for tg, wb in (("f", WF), ("r", WR)):
        s.st(ellipsoid(f"mass-wheels-{tg}", wb, EX, 0.2, [0.06, 0.03, 0.2], [0, 0, 0], role="detail"), wb, "mass")


def decisions200(s):
    s.dec("The 1:200 car is the same body at 30 mm: the frame is one 60 mm slab of the drawn frame width, the hollow tub and cabin keep their 80 mm walls, but each side has one merged glass pane behind the pillars, the bonnet is a closed top with plain side panels (the louvre band is only a drawn band, no slots), the wings come from six stations, the bumpers are one blade each as drawn, the wheels are a tyre, a T-section rim and disc, two crossed 60 mm spoke bars and a hub cap; drums, engine, manifold and seats are not visible at this scale and live in the thermal list as real solids inside the shells.", "Der Wagen 1:200 ist derselbe Aufbau bei 30 mm: der Rahmen ist eine 60-mm-Platte in der gezeichneten Rahmenbreite, die hohle Wanne und Kabine behalten ihre 80-mm-Wände, jede Seite hat aber eine zusammengefasste Scheibe hinter den Säulen, die Haube ist ein geschlossener Deckel mit glatten Seitenblechen (das Schlitzband ist nur ein gezeichnetes Band, keine Schlitze), die Kotflügel entstehen aus sechs Stationen, die Stoßfänger sind je ein Blatt wie gezeichnet, die Räder bestehen aus Reifen, T-förmiger Felge mit Scheibe, zwei gekreuzten 60-mm-Speichenbalken und einer Radkappe; Trommeln, Motor, Krümmer und Sitze sind bei diesem Maßstab nicht sichtbar und stehen als echte Körper in den Hüllen im Thermikmodell.")
    s.dec("Thermal at 1:200 is one reference per drawn part; the hidden engine block, manifold, drums, seats and column are real solids inside the shells; only the total exposed area is compared at this scale.", "Die Thermik bei 1:200 ist ein Verweis je gezeichnetem Teil; Motorblock, Krümmer, Trommeln, Sitze und Säule sind echte verdeckte Körper in den Hüllen; bei diesem Maßstab wird nur die Gesamtfläche verglichen.")
    s.dec("Structure at 1:200: two solid 50 mm frame rails of equal stiffness, the axle pieces and one solid wheel per wheel as members, one body mass and two wheel masses; joints are merged into the members.", "Struktur bei 1:200: zwei massive 50-mm-Längsträger gleicher Steifigkeit, die Achsstücke und je ein massiver Radkörper als Stäbe, eine Aufbaumasse und zwei Radmassen; Gelenke sind in die Stäbe aufgenommen.")


def body500():
    sc = 500
    b = []
    b.append(rbox("underbody", CH, P, 0.15, [4.39, 0.25, 1.4], [0.05, 0.515, 0], 0.03))
    top = [(-1.985, 1.1), (-1.9, 1.12), (-1.82, 1.14), (-1.75, 1.4), (-1.7, 1.58), (-1.6, 1.67), (-1.5, 1.7), (0.46, 1.7), (0.5, 1.64), (0.55, 1.42), (0.62, 1.18)]
    b.append(xloft("tub-cabin", CH, P, 0.25, [(x, 0.5, y, -0.62, 0.62, 8) for x, y in top], e=8))
    pane = dedup(shape(SLUG, 500, "elevation", "windows"))
    for s_, sg in (("l", 1), ("r", -1)):
        b.append(ext(f"window-{s_}", CH, GL, 0.85, "xy", pane, 0.62 if sg > 0 else -0.64, 0.64 if sg > 0 else -0.62, role="glazing"))
    b.append(box("windscreen", CH, GL, 0.85, [0.03, 0.4, 1.04], [0.53, 1.38, 0], None, "glazing", Qz(0.2)))
    ring = R_(500, "silhouette")
    pr = PL_(500, "silhouette")
    hood_rows = []
    for x, yt in ((0.622, 1.18), (0.8, 1.2), (0.94, 1.21), (0.97, 1.213), (1.0, 1.215), (1.05, 1.215), (1.3, 1.215), (1.6, 1.21), (1.78, 1.2)):
        lo = 0.5 if x <= 0.95 else span(ring, x)[0]
        hood_rows.append((x, lo, yt, -0.28, 0.28, 8))
    b.append(xloft("hood", CH, P, 0.2, hood_rows, e=8))
    rows = []
    for x in (0.965, 1.0, 1.1, 1.3, 1.5, 1.7, 1.88, 1.93, 2.05, 2.14):
        ys = slc(ring, x)
        lo, hi = (ys[0], ys[0] + 0.1) if x <= 1.88 else (0.5, max(ys[-1], 0.58))
        h = span(pr, x)[1]
        rows.append((x, lo, hi, -h, h, 5))
    b.append(xloft("wing-deck", CH, P, 0.22, rows))
    for s_, sg in (("l", 1), ("r", -1)):
        zz = lambda a, c: (min(sg * a, sg * c), max(sg * a, sg * c))
        b.append(xloft(f"wing-rear-{s_}", CH, P, 0.22, [(-2.025, 0.5, 0.78, *zz(0.5, 0.6), 5), (-1.95, 0.5, 0.82, *zz(0.5, 0.8), 5), (-1.8, 0.5, 0.85, *zz(0.5, 0.85), 5), (-1.1, 0.5, 0.85, *zz(0.5, 0.85), 5), (-0.985, 0.5, 0.75, *zz(0.5, 0.64), 5)], e=5))
    b.append(rbox("radiator-shell", CH, NI, 0.85, [0.12, 0.7, 0.44], [1.84, 0.9, 0], 0.03))
    b.append(rbox("tail-lamp", CH, P, 0.5, [0.075, 0.1, 0.1], [-2.0, 0.95, 0.4], 0.008, role="detail"))
    for bone, tag in ((WF, "f"), (WR, "r")):
        for s_, sg in (("l", 1), ("r", -1)):
            b.append(revolve(f"tyre-{tag}{s_}", bone, RU, 0.06, "z", [0, 0, sg * ZC], cap_prof([[-0.0575, 0], [-0.0575, 0.34], [-0.03, 0.37], [0.03, 0.37], [0.0575, 0.34], [0.0575, 0]])))
    return b


def thermal500(s):
    B = by(s.body)
    th = s.thermal
    th.append(ref("th-tub-cabin", "tub-cabin", "body-panels"))
    roof = xloft("th-roof", CH, FR, 0.78, [(-1.5, 1.625, 1.705, -0.6, 0.6, 6), (0.46, 1.625, 1.705, -0.6, 0.6, 6)])
    roof["region"] = "roof"
    faces(roof, "-v")
    th.append(roof)
    th.append(ref("th-hood", "hood", "hood"))
    for i in ("wing-deck", "wing-rear-l", "wing-rear-r"):
        th.append(ref(f"th-{i}", i, "fenders"))
    for i, f in (("window-l", ["-w"]), ("window-r", ["+w"]), ("windscreen", ["-u"])):
        th.append(ref(f"th-{i}", i, "glazing", None, f))
    for nm, x0, x1 in (("front", -0.02, 0.52), ("rear", -0.97, -0.33)):
        d = ext(f"th-seat-{nm}", CH, UP, 0.42, "xy", [[x0, 0.6], [x1, 0.6], [x1, 0.78], [x0 + 0.2, 0.78], [x0 + 0.2, 1.5], [x0, 1.5]], -0.48, 0.48, role="detail")
        d["region"] = "interior"
        th.append(d)
    th.append(ref("th-radiator-shell", "radiator-shell", "radiator", None, ["-u"]))
    d = rbox("th-engine", CH, EM, 0.35, [0.62, 0.22, 0.28], [1.23, 1.0, 0], 0.02, role="detail")
    d["region"] = "engine-bay"
    th.append(d)
    d = slender(capsule("th-exhaust-pipe", CH, EX, 0.3, [-1.75, 0.27, -0.3], [0.9, 0.27, -0.3], 0.03, role="detail"))
    d["region"] = "exhaust-system"
    th.append(d)
    d = capsule("th-silencer", CH, EX, 0.4, [-1.75, 0.27, -0.3], [-1.10, 0.27, -0.3], 0.075, role="detail")
    d["region"] = "exhaust-system"
    th.append(d)
    jet = revolve("tailpipe-jet", CH, EX, 0.9, [-1, 0, 0], [-2.1, 0.25, -0.35], cap_prof([[0, 0], [0, 0.0375], [0.075, 0.0375], [0.075, 0]]), role="detail")
    tag(jet, "tailpipe")
    faces(jet, "-u", "+u")
    jet["region"] = "exhaust-system"
    th.append(jet)
    for tg, wb in (("fl", WF), ("fr", WF), ("rl", WR), ("rr", WR)):
        th.append(ref(f"th-tyre-{tg}", f"tyre-{tg}", "tyres"))
        sg = 1 if tg[1] == "l" else -1
        d = revolve(f"th-drum-{tg}", wb, CI, 0.3, "z", [0, 0, sg * ZC], [[r6(sg * -0.0875), 0], [r6(sg * -0.0875), 0.15], [r6(sg * -0.01), 0.15], [r6(sg * -0.01), 0], [r6(sg * -0.0875), 0]], role="detail")
        d["region"] = "brakes"
        th.append(d)


def structure500(s):
    for sd, sg in (("l", 1), ("r", -1)):
        s.st(rbox(f"frame-rail-{sd}", CH, EX, 0.2, [3.96, 0.075, 0.075], [0.05, 0.38, sg * 0.38], 0.01, role="detail"), CH, "member")
    for tg, bone in (("f", AF), ("r", AR)):
        d = revolve(f"axle-{tg}", bone, CI, 0.3, "z", [0, 0, -0.6], cap_prof([[0, 0], [0, 0.04], [1.2, 0.04], [1.2, 0]]), role="detail")
        s.st(d, bone, "member")
    for tg, wb in (("f", WF), ("r", WR)):
        for sd in ("l", "r"):
            s.stof(f"wheel-{tg}{sd}-member", f"tyre-{tg}{sd}", wb, "member")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.3, 0.2, 0.4], [-0.1, 0.72, 0], role="detail"), CH, "mass")
    for tg, wb in (("f", WF), ("r", WR)):
        s.st(ellipsoid(f"mass-wheels-{tg}", wb, EX, 0.2, [0.09, 0.04, 0.3], [0, 0, 0], role="detail"), wb, "mass")


def decisions500(s):
    s.dec("At 1:500 the car is a handful of masses at 75 mm: an underbody slab for frame, bumpers and exhaust zone, one hull loft for tub, cabin and roof on the drawn silhouette, a hood block standing on a full-width wing deck whose lower edge is the drawn arch curve, two rear wing masses, the radiator block, two glass panes and a windscreen, and four solid tyre discs. Frame, axles, engine, drums and seats are not drawn; they exist in the thermal and structure lists as real solids.", "Bei 1:500 besteht der Wagen aus wenigen Massen mit 75 mm: eine Unterbauplatte für Rahmen, Stoßfänger und Abgaszone, ein Rumpfkörper für Wanne, Kabine und Dach nach der gezeichneten Silhouette, ein Haubenblock auf einem durchgehenden Kotflügeldeck, dessen Unterkante die gezeichnete Bogenkurve ist, zwei hintere Kotflügelmassen, der Kühlerblock, zwei Scheiben und eine Windschutzscheibe sowie vier massive Reifenscheiben. Rahmen, Achsen, Motor, Trommeln und Sitze werden nicht gezeichnet; sie stehen als echte Körper in den Thermik- und Strukturlisten.")
    s.dec("Thermal at 1:500: a reference per drawn mass; roof plate, hidden engine, seats, exhaust pipe with silencer and drums are real solids; only the total exposed area is compared.", "Die Thermik bei 1:500: ein Verweis je gezeichneter Masse; Dachplatte, verdeckter Motor, Sitze, Abgasrohr mit Schalldämpfer und Trommeln sind echte Körper; verglichen wird nur die Gesamtfläche.")
    s.dec("Structure at 1:500: two solid 75 mm frame rails, one axle tube per axle, one tyre disc per wheel as member, one body mass and two wheel masses.", "Struktur bei 1:500: zwei massive 75-mm-Längsträger, ein Achsrohr je Achse, eine Reifenscheibe je Rad als Stab, eine Aufbaumasse und zwei Radmassen.")

def chk(s):
    for lst in (s.body, s.thermal, s.structure):
        ids = [d["id"] for d in lst]
        assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]


if __name__ == "__main__":
    only = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
    sc = []
    if 50 in only:
        s = Scale(50)
        s.body = body50() + interior50()
        thermal50(s)
        structure50(s)
        decisions50(s)
        chk(s)
        sc.append(s)
    if 100 in only:
        s = Scale(100)
        s.body = body100() + interior100()
        thermal100(s)
        structure100(s)
        decisions100(s)
        chk(s)
        sc.append(s)
    if 200 in only:
        s = Scale(200)
        s.body = body200()
        thermal200(s)
        structure200(s)
        decisions200(s)
        chk(s)
        sc.append(s)
    if 500 in only:
        s = Scale(500)
        s.body = body500()
        thermal500(s)
        structure500(s)
        decisions500(s)
        chk(s)
        sc.append(s)
    save_v(SLUG, sc)
