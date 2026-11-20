"""🚗️ The 1:50 lists of the round 3 vintage saloon, solid by solid: body, thermal and structure. Executed by r3-vintage-car-1929.py after the kit."""


def wheel50(bone):
    o = [0, 0, ZW]
    out = [revolve("tyre-l", bone, RU, 0.08, "z", o, TYRE), revolve("rim-l", bone, RM, 0.55, "z", o, RIM, role="detail")]
    sp = capsule("spokes-l", bone, RM, 0.7, [0.066, 0, ZW + 0.018], [0.232, 0, ZW], 0.014, 0.01, role="detail")
    sp["repeat"] = {"count": 12, "rotationRad": r6(math.pi / 6), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}
    out.append(sp)
    out.append(revolve("hub-l", bone, RM, 0.5, "z", o, [[-0.03, 0], [-0.03, 0.05], [-0.005, 0.068], [0.03, 0.068], [0.038, 0.055], [0.038, 0]], role="detail"))
    out.append(revolve("hub-cap-l", bone, NI, 0.8, "z", o, [[0.036, 0], [0.036, 0.05], [0.058, 0.046], [0.07, 0.03], [0.075, 0]], role="detail"))
    out.append(revolve("drum-l", bone, CI, 0.3, "z", o, [[-0.10, 0], [-0.10, 0.145], [-0.095, 0.16], [-0.055, 0.16], [-0.05, 0.15], [-0.05, 0]], role="detail"))
    out.append(capsule("valve-l", bone, RM, 0.9, [0.2188, 0.0884, ZW + 0.03], [0.2462, 0.0995, ZW + 0.03], 0.006, role="detail"))
    res = []
    for d in out:
        res += pair(d)
    tag = "f" if bone == WF else "r"
    for d in res:
        d["id"] = d["id"][:-2] + "-" + tag + d["id"][-1]
    return res


def chassis50():
    b = []
    rail = loft("frame-rail-l", CH, P, 0.18, [xsec(x, yc - h / 2, yc + h / 2, 0.0225, 10, z) for x, yc, z, h in RAIL])
    b += pair(rail)
    for nm, x, y, hw in (("front", 1.97, 0.50, 0.34), ("radiator", 1.74, 0.475, 0.34), ("gearbox", 0.62, 0.455, 0.41), ("mid", -0.45, 0.455, 0.42), ("kick", -1.10, 0.47, 0.42), ("rear", -1.93, 0.565, 0.42)):
        b.append(rbox(f"crossmember-{nm}", CH, P, 0.18, [0.05, 0.06, 2 * hw], [x, y, 0], 0.008))
    sf = sheet("spring-front-l", CH, EX, 0.25, SPRING_F, [0.34] * 7, [0.0225] * 7, SPRING_T, e=6, role="detail")
    sr = sheet("spring-rear-l", CH, EX, 0.25, SPRING_R, [0.47] * 7, [0.025] * 7, SPRING_T, e=6, role="detail")
    b += pair(sf) + pair(sr)
    b += pair(capsule("shackle-front-l", CH, EX, 0.25, [1.31, 0.405, 0.34], [1.30, 0.44, 0.345], 0.012, role="detail"))
    b += pair(capsule("shackle-rear-l", CH, EX, 0.25, [-1.96, 0.48, 0.47], [-1.955, 0.54, 0.445], 0.012, role="detail"))
    b += pair(capsule("spring-hanger-l", CH, EX, 0.25, [-0.95, 0.46, 0.47], [-0.95, 0.47, 0.44], 0.014, role="detail"))
    # front axle (bone axle-f, origin at the wheel centre line)
    zs = [-0.56, -0.50, -0.44, -0.38, 0.38, 0.44, 0.50, 0.56]
    ys = [0.0, -0.005, -0.035, -0.06, -0.06, -0.035, -0.005, 0.0]
    b.append(loft("axle-beam", AF, CI, 0.3, [sec([0, y, z], 0.022, 0.03, 6) for y, z in zip(ys, zs)], role="detail"))
    b += pair(capsule("kingpin-l", AF, CI, 0.3, [0, -0.055, 0.565], [0, 0.055, 0.565], 0.018, role="detail"))
    b += pair(revolve("stub-axle-l", AF, CI, 0.3, "z", [0, 0, 0.565], [[0, 0], [0, 0.024], [0.07, 0.02], [0.07, 0]], role="detail"))
    b += pair(revolve("backplate-f-l", AF, CI, 0.25, "z", [0, 0, 0.553], [[0.005, 0], [0.005, 0.155], [0.017, 0.155], [0.017, 0]], role="detail"))
    b.append(slender(capsule("tie-rod", AF, EX, 0.25, [-0.12, -0.03, -0.52], [-0.12, -0.03, 0.52], 0.011, role="detail")))
    b += pair(capsule("steering-arm-l", AF, CI, 0.3, [-0.01, -0.03, 0.55], [-0.12, -0.03, 0.52], 0.012, role="detail"))
    # rear axle (bone axle-r)
    b.append(revolve("axle-tube", AR, CI, 0.3, "z", [0, 0, 0], [[-0.56, 0], [-0.56, 0.035], [-0.16, 0.042], [-0.12, 0.085], [-0.06, 0.13], [0, 0.14], [0.06, 0.13], [0.12, 0.085], [0.16, 0.042], [0.56, 0.035], [0.56, 0]], role="detail"))
    b.append(revolve("differential-nose", AR, CI, 0.3, "x", [0.0, 0, 0], [[0, 0], [0, 0.10], [0.13, 0.085], [0.20, 0.05], [0.22, 0]], role="detail"))
    b += pair(revolve("backplate-r-l", AR, CI, 0.25, "z", [0, 0, 0.553], [[0.005, 0], [0.005, 0.155], [0.017, 0.155], [0.017, 0]], role="detail"))
    b += pair(rbox("spring-seat-l", AR, EX, 0.25, [0.11, 0.03, 0.06], [0, 0.045, 0.47], 0.006, role="detail"))
    b.append(capsule("propeller-shaft", CH, EX, 0.25, [XR + 0.2, RW, 0], [0.50, 0.46, 0], 0.03, role="detail"))
    # engine and drive
    b.append(rbox("engine-block", CH, EM, 0.35, [0.70, 0.30, 0.22], [1.23, 0.71, 0], 0.02, role="detail"))
    b.append(rbox("engine-head", CH, EM, 0.3, [0.68, 0.12, 0.20], [1.23, 0.915, 0], 0.02, role="detail"))
    b.append(loft("engine-sump", CH, EM, 0.3, [xsec(0.90, 0.43, 0.58, 0.10, 4), xsec(1.10, 0.43, 0.58, 0.10, 4), xsec(1.40, 0.47, 0.58, 0.095, 4), xsec(1.57, 0.50, 0.58, 0.09, 4)], role="detail"))
    b.append(revolve("bell-housing", CH, EM, 0.3, "x", [0.78, 0.60, 0], [[0, 0], [0, 0.12], [0.06, 0.185], [0.11, 0.185], [0.11, 0]], role="detail"))
    b.append(loft("gearbox", CH, EM, 0.3, [xsec(0.50, 0.435, 0.545, 0.07, 3), xsec(0.60, 0.42, 0.60, 0.10, 4), xsec(0.80, 0.42, 0.62, 0.11, 4)], role="detail"))
    b.append(revolve("fan", CH, EM, 0.25, "x", [1.60, 0.80, 0], [[0, 0], [0, 0.04], [0.02, 0.04], [0.02, 0.17], [0.035, 0.17], [0.035, 0]], role="detail"))
    b.append(capsule("dynamo", CH, EM, 0.3, [1.30, 0.66, 0.16], [1.50, 0.66, 0.16], 0.045, role="detail"))
    b.append(capsule("inlet-manifold", CH, EM, 0.4, [1.02, 0.82, 0.135], [1.44, 0.82, 0.135], 0.022, role="detail"))
    b.append(rbox("carburettor", CH, NI, 0.7, [0.07, 0.11, 0.06], [1.23, 0.76, 0.165], 0.012, role="detail"))
    for k, x in enumerate((0.98, 1.50)):
        b += pair(rbox(f"engine-bearer-{k + 1}-l", CH, P, 0.18, [0.07, 0.04, 0.27], [x, 0.56, 0.235], 0.006, role="detail"))
    b.append(capsule("exhaust-manifold", CH, EX, 0.35, [0.98, 0.78, -0.15], [1.50, 0.78, -0.15], 0.03, role="detail"))
    b += [dict(d, id=f"exhaust-downpipe-{i + 1}") for i, d in enumerate(tube("x", CH, EX, 0.35, EXH[:3], 0.027))]
    b.append(capsule("exhaust-pipe", CH, EX, 0.35, EXH[2], EXH[3], 0.025, role="detail"))
    b.append(revolve("silencer", CH, EX, 0.4, [-1, 0, 0], [-0.13, 0.36, -0.30], [[0, 0], [0, 0.03], [0.03, 0.07], [0.70, 0.07], [0.73, 0.03], [0.73, 0]], role="detail"))
    b += [dict(d, id=f"tail-pipe-{i + 1}") for i, d in enumerate(tube("x", CH, EX, 0.45, EXH_TAIL, 0.025))]
    for k, (x, y0, y1, z) in enumerate(((0.45, 0.385, 0.42, -0.30), (-0.50, 0.42, 0.435, -0.30), (-1.55, 0.52, 0.55, -0.37))):
        b.append(rbox(f"exhaust-hanger-{k + 1}", CH, EX, 0.18, [0.03, y1 - y0 + 0.02, 0.03], [x, (y0 + y1) / 2, z], 0.004, role="detail"))
    b.append(rbox("fuel-tank", CH, P, 0.2, [0.20, 0.19, 0.58], [-1.845, 0.53, 0], 0.06, role="detail"))
    return b


def shell50():
    b = []
    side = ext("tub-side-l", CH, P, 0.6, "xy", TUB, 0.595, ZB)
    b += pair(side)
    b.append(ext("tub-rear", CH, P, 0.6, "xy", TAIL + TAIL_IN, -0.597, 0.597))
    wh = arc(XR, RW, ARCH_R + 0.006, 25.5, 154.5, 10)
    b += pair(ext("wheelhouse-l", CH, P, 0.3, "xy", wh, 0.575, 0.597))
    b.append(rbox("floor-front", CH, P, 0.3, [1.57, 0.025, 1.192], [-0.275, 0.5325, 0], 0.004))
    b.append(rbox("heel-board", CH, P, 0.3, [0.025, 0.165, 1.192], [-1.05, 0.6025, 0], 0.004))
    b.append(rbox("seat-pan", CH, P, 0.3, [0.93, 0.025, 1.192], [-1.505, 0.6725, 0], 0.004))
    b.append(loft("cowl", CH, P, 0.6, [xsec(x, YSILL, top, hw, e) for x, top, hw, e in COWL]))
    # greenhouse: two tilted side frames with their openings, windscreen frame, rear panel, glass
    holes = [fillet(w, 0.03) for w in (WIN_F, WIN_R, WIN_Q)]
    fr = ext("cabin-side-l", CH, P, 0.58, "xy", CABIN_LOW, 0.59, ZB, holes=holes)
    b += pair(turn(fr, Qx(-A_TUM), [0, YBELT, ZB]))
    for nm, w in (("front", WIN_F), ("rear", WIN_R), ("quarter", WIN_Q)):
        g = ext(f"window-{nm}-l", CH, GL, 0.85, "xy", grow(w, 0.008), 0.602, 0.608, role="glazing")
        b += pair(turn(g, Qx(-A_TUM), [0, YBELT, ZB]))
    k = (YHEAD - YBELT) / 0.46
    outer = [[YBELT - 0.05, -0.606], [YBELT - 0.05, 0.606], [YHEAD, 0.606 - 0.04 * k], [YHEAD, -0.606 + 0.04 * k]]
    wsh = [[1.215, -0.54], [1.215, 0.54], [1.578, 0.51], [1.578, -0.51]]
    f = ext("windscreen-frame", CH, P, 0.58, "yz", outer, 0.485, 0.52, holes=[fillet(wsh, 0.03)])
    b.append(turn(f, Qz(A_TUM), [0.52, YBELT, 0]))
    g = ext("windscreen", CH, GL, 0.85, "yz", grow(wsh, 0.008), 0.499, 0.505, role="glazing")
    b.append(turn(g, Qz(A_TUM), [0.52, YBELT, 0]))
    kr = (YHEAD - YBELT) / math.cos(A_REAR)
    outer_r = [[YBELT - 0.02, -0.606], [YBELT - 0.02, 0.606], [YBELT + kr, 0.606 - 0.04 * k], [YBELT + kr, -0.606 + 0.04 * k]]
    rwh = fillet([[1.30, -0.28], [1.30, 0.28], [1.52, 0.28], [1.52, -0.28]], 0.05)
    f = ext("cabin-rear-wall", CH, P, 0.58, "yz", outer_r, -1.90, -1.87, holes=[rwh])
    b.append(turn(f, Qz(-A_REAR), [-1.90, YBELT, 0]))
    g = ext("window-back", CH, GL, 0.85, "yz", [[1.29, -0.29], [1.29, 0.29], [1.53, 0.29], [1.53, -0.29]], -1.888, -1.882, role="glazing")
    b.append(turn(g, Qz(-A_REAR), [-1.90, YBELT, 0]))
    b.append(loft("roof-slab", CH, FB, 0.78, [sec([x, yc, 0], rx, ry, ROOF_E, rotation=Qy(math.pi / 2)) for x, yc, rx, ry in ROOF]))
    # bonnet: crowned top, two louvred side panels turned to the plan taper, hinge
    b.append(loft("hood-top", CH, P, 0.62, [sec([x, top - HOOD_RY, 0], hw, HOOD_RY, e, rotation=Qy(math.pi / 2)) for x, top, hw, e in HOOD]))
    L = lambda x: 0.72 + (x - 0.72) / math.cos(HOOD_B)
    prof = [[0.72, YSILL], [L(1.69), YSILL], [L(1.69), 1.09], [0.72, 1.09]]
    lh = [[[L(x) - 0.007, 0.76], [L(x) + 0.007, 0.76], [L(x) + 0.007, 0.98], [L(x) - 0.007, 0.98]] for x in LOUVRES]
    hs = ext("hood-side-l", CH, P, 0.62, "xy", prof, HOOD_ZR - 0.012, HOOD_ZR, holes=lh)
    b += pair(turn(hs, Qy(HOOD_B), [0.72, 0, HOOD_ZR]))
    b.append(rbox("hood-hinge", CH, NI, 0.8, [0.96, 0.01, 0.016], [1.205, 1.187, 0], None, role="detail"))
    for k_, x in enumerate((0.84, 1.56)):
        c = capsule(f"hood-catch-{k_ + 1}-l", CH, NI, 0.8, [x, 0.60, 0.0], [x, 0.66, 0.0], 0.009, role="detail")
        zz = hood_z(x) + 0.006
        c["p0"][2] = c["p1"][2] = r6(zz)
        b += pair(c)
    # radiator
    full = lambda half: [[y, -z] for y, z in half] + [[y, z] for y, z in half[::-1][1:]]
    b.append(ext("radiator-shell", CH, NI, 0.78, "yz", full(SHELL), 1.68, 1.80, holes=[full(CORE)]))
    b.append(ext("radiator-core", CH, EM, 0.15, "yz", [[y + (0.004 if y > 0.9 else -0.004), z * 1.02] for y, z in full(CORE)], 1.70, 1.775, role="detail"))
    b.append(revolve("radiator-neck", CH, NI, 0.78, "y", [1.74, 1.19, 0], [[0, 0], [0, 0.028], [0.04, 0.028], [0.04, 0]], role="detail"))
    b.append(revolve("radiator-cap", CH, NI, 0.78, "y", [1.74, 1.225, 0], [[0, 0], [0, 0.042], [0.014, 0.045], [0.026, 0.03], [0.032, 0]], role="detail"))
    return b


def trim50():
    b = []
    b += pair(sheet("fender-front-l", CH, P, 0.5, WING_F, WING_F_Z, WING_F_RX, WING_F_RY, e=2.4))
    b += pair(sheet("fender-rear-l", CH, P, 0.5, WING_B, WING_B_Z, WING_B_RX, WING_B_RY, e=2.4))
    b += pair(rbox("running-board-l", CH, P, 0.42, [2.05, 0.035, 0.21], [0.005, 0.4375, 0.725], 0.012))
    b += pair(rbox("sill-apron-l", CH, P, 0.5, [1.60, 0.085, 0.03], [-0.27, 0.4825, 0.607], 0.004))
    for k, x in enumerate((-0.75, 0.0, 0.75)):
        b += pair(rbox(f"running-bracket-{k + 1}-l", CH, P, 0.18, [0.04, 0.03, 0.40], [x, 0.405, 0.62], 0.004, role="detail"))
    b += pair(slender(capsule("wing-stay-l", CH, P, 0.18, [1.96, 0.50, 0.35], [1.96, 0.755, 0.62], 0.009, role="detail")))
    # head lamps on a tie bar, side lamps, tail lamp, plate
    lo = [LAMP[0], LAMP[1], LAMP[2]]
    b += pair(revolve("headlamp-bowl-l", CH, NI, 0.74, "x", lo, BOWL))
    b += pair(revolve("headlamp-lens-l", CH, GL, 0.88, "x", lo, LENS, role="glazing"))
    b += pair(capsule("lamp-stalk-l", CH, NI, 0.5, [1.83, 0.862, 0.43], [1.845, 0.895, 0.43], 0.016, role="detail"))
    b.append(capsule("lamp-bar", CH, NI, 0.74, [1.83, 0.862, -0.60], [1.83, 0.862, 0.60], 0.014, role="detail"))
    b += pair(capsule("lamp-bar-foot-l", CH, NI, 0.74, [1.83, 0.862, 0.60], [1.815, 0.84, 0.64], 0.014, role="detail"))
    b += pair(revolve("fender-lamp-l", CH, NI, 0.85, "x", [1.60, 0.888, 0.72], [[0, 0], [0.02, 0.022], [0.06, 0.03], [0.085, 0.03], [0.085, 0]], role="detail"))
    b += pair(revolve("fender-lamp-lens-l", CH, GL, 0.88, "x", [1.60, 0.888, 0.72], [[0.083, 0], [0.083, 0.026], [0.092, 0.015], [0.094, 0]], role="glazing"))
    b += pair(capsule("fender-lamp-foot-l", CH, NI, 0.85, [1.65, 0.842, 0.72], [1.65, 0.865, 0.72], 0.009, role="detail"))
    b.append(revolve("tail-lamp", CH, NI, 0.3, [-1, 0, 0], [-1.985, 0.95, 0.42], [[0, 0], [0, 0.032], [0.03, 0.032], [0.042, 0.02], [0.045, 0]], role="detail"))
    b.append(rbox("number-plate", CH, P, 0.9, [0.012, 0.11, 0.40], [-1.996, 0.82, 0], None, role="detail"))
    # bumpers: two sprung blades with swept ends on irons, clamped
    for end, sg, zi in (("front", 1, 0.34), ("rear", -1, 0.42)):
        x0 = sg * (XBUMP - 0.006)
        for nm, y in zip(("lower", "upper"), BUMP_Y):
            b.append(loft(f"bumper-{end}-{nm}", CH, NI, 0.8, [sec([x0 + sg * dx, y, z], 0.006, 0.0225, 4) for z, dx in zip(BUMP_Z, BUMP_DX)]))
        for cz, cn in ((-zi, "r"), (0.0, "c"), (zi, "l")):
            b.append(rbox(f"bumper-{end}-clamp-{cn}", CH, NI, 0.8, [0.02, 0.15, 0.03], [x0 - sg * 0.006, 0.52, cz], 0.006, role="detail"))
        tip = (2.07, 0.505) if sg > 0 else (-1.95, 0.56)
        b += pair(capsule(f"bracket-{end}-l", CH, EX, 0.6, [tip[0], tip[1], zi], [x0 - sg * 0.01, 0.52, zi], 0.014, role="detail"))
    # doors: shut lines, hinges, handles, belt moulding
    arch_y = RW + math.sqrt(ARCH_R ** 2 - (X_C - XR) ** 2)
    for nm, x, y0 in (("hinge-gap", X_A, 0.56), ("gap-b", X_B, 0.56), ("gap-rear", X_C, arch_y + 0.01)):
        b += pair(texture(rbox(f"door-{nm}-l", CH, P, 0.12, [0.008, YBELT - y0, 0.004], [x, (YBELT + y0) / 2, ZB + 0.001], None, role="detail")))
    b += pair(rbox("belt-moulding-l", CH, P, 0.5, [2.41, 0.022, 0.009], [-0.69, YBELT - 0.006, ZB + 0.004], 0.003, role="detail"))
    for nm, x in (("front", X_B + 0.11), ("rear", X_B - 0.11)):
        b += pair(capsule(f"handle-{nm}-l", CH, NI, 0.8, [x - 0.045, 1.06, ZB + 0.022], [x + 0.045, 1.06, ZB + 0.022], 0.009, role="detail"))
        b += pair(capsule(f"handle-{nm}-stem-l", CH, NI, 0.8, [x - (0.045 if nm == "front" else -0.045), 1.06, ZB - 0.003], [x - (0.045 if nm == "front" else -0.045), 1.06, ZB + 0.022], 0.009, role="detail"))
    for nm, x in (("front", X_A - 0.012), ("rear", X_C - 0.012)):
        h = capsule(f"hinge-{nm}-l", CH, P, 0.5, [x, 0.66, ZB + 0.005], [x, 0.71, ZB + 0.005], 0.008, role="detail")
        h["repeat"] = {"count": 2, "translation": [0, 0.36, 0]}
        b += pair(h)
    # spare wheel in the right front wing
    so = [SPARE[0], SPARE[1], SPARE[2]]
    b.append(revolve("spare-tyre", CH, RU, 0.08, "z", so, TYRE))
    b.append(revolve("spare-rim", CH, RM, 0.55, "z", so, RIM, role="detail"))
    b.append(revolve("spare-cover", CH, P, 0.62, [0, 0, -1], so, [[-0.02, 0], [-0.02, 0.24], [0.02, 0.24], [0.04, 0.16], [0.048, 0]], role="detail"))
    b.append(revolve("spare-boss", CH, NI, 0.8, [0, 0, -1], so, [[0.046, 0], [0.046, 0.04], [0.058, 0.03], [0.062, 0]], role="detail"))
    b.append(capsule("spare-mount", CH, EX, 0.18, [0.80, 0.90, -0.41], [SPARE[0], SPARE[1], SPARE[2] + 0.02], 0.02, role="detail"))
    return b


def cabin50():
    b = []
    b.append(rbox("seat-front-base", CH, UP, 0.25, [0.44, 0.15, 1.06], [0.0, 0.62, 0], 0.01, role="detail"))
    b.append(rbox("seat-front-cushion", CH, UP, 0.3, [0.50, 0.17, 1.10], [0.0, 0.775, 0], 0.06, role="detail"))
    b.append(rbox("seat-front-back", CH, UP, 0.3, [0.13, 0.60, 1.10], [-0.25, 1.08, 0], 0.055, role="detail", rot=Qz(math.radians(8))))
    b.append(rbox("seat-rear-cushion", CH, UP, 0.3, [0.52, 0.18, 1.10], [-1.36, 0.775, 0], 0.06, role="detail"))
    b.append(rbox("seat-rear-back", CH, UP, 0.3, [0.14, 0.62, 1.10], [-1.67, 1.08, 0], 0.055, role="detail", rot=Qz(math.radians(10))))
    c, d = SW_C, SW_D
    b.append(capsule("steering-column", CH, EX, 0.3, [c[0], c[1], c[2]], [1.02, 0.539, c[2]], 0.014, role="detail"))
    ring = [[r6(0.013 * math.cos(math.radians(a))), r6(0.20 + 0.013 * math.sin(math.radians(a)))] for a in range(0, 360, 45)]
    b.append(revolve("steering-wheel", CH, EX, 0.3, list(d), list(c), ring + [ring[0]], role="detail"))
    e2 = (0.658, 0.753, 0.0)
    for i, (u, v) in enumerate(((0, 1), (0, -1), (1, 0), (-1, 0))):
        tip = [c[0] + 0.19 * u * e2[0], c[1] + 0.19 * u * e2[1], c[2] + 0.19 * v]
        b.append(capsule(f"steering-spoke-{i + 1}", CH, EX, 0.3, list(c), tip, 0.008, role="detail"))
    b.append(rbox("steering-box", CH, EM, 0.3, [0.10, 0.10, 0.07], [1.04, 0.56, c[2]], 0.015, role="detail"))
    b.append(capsule("drop-arm", CH, CI, 0.3, [1.04, 0.56, -0.345], [1.07, 0.40, -0.345], 0.014, role="detail"))
    b.append(slender(capsule("drag-link", CH, EX, 0.25, [1.07, 0.40, -0.345], [XF - 0.10, RW - 0.03, -0.50], 0.011, role="detail")))
    b.append(loft("dashboard", CH, P, 0.35, [xsec(0.40, 0.98, 1.16, 0.59, 6), xsec(0.46, 0.96, 1.16, 0.59, 6)], role="detail"))
    for i, z in enumerate((-0.10, 0.10)):
        b.append(revolve(f"dial-{i + 1}", CH, NI, 0.85, [-1, 0, 0], [0.402, 1.08, z], [[0, 0], [0, 0.045], [0.008, 0.045], [0.008, 0]], role="detail"))
    b.append(revolve("mirror", CH, NI, 0.8, [-1, 0, 0], [0.44, 1.545, 0], [[0, 0], [0, 0.012], [0.03, 0.012], [0.03, 0.05], [0.04, 0.05], [0.04, 0]], role="detail"))
    return b


def body50():
    return chassis50() + shell50() + trim50() + cabin50() + wheel50(WF) + wheel50(WR)


PANELS = ["tub-side-l", "tub-side-r", "tub-rear", "cowl", "cabin-side-l", "cabin-side-r", "windscreen-frame", "cabin-rear-wall"]
HOODS = ["hood-top", "hood-side-l", "hood-side-r"]
INNER = {"tub-side-l": ["-w"], "tub-side-r": ["+w"], "tub-rear": ["+u"], "cowl": ["-u"], "cabin-side-l": ["-w"], "cabin-side-r": ["+w"], "windscreen-frame": ["-u"], "cabin-rear-wall": ["+u"], "hood-top": ["-v"], "hood-side-l": ["-w"], "hood-side-r": ["+w"], "window-front-l": ["-w"], "window-rear-l": ["-w"], "window-quarter-l": ["-w"], "window-front-r": ["+w"], "window-rear-r": ["+w"], "window-quarter-r": ["+w"], "windscreen": ["-u"], "window-back": ["+u"]}
WINGS = ["fender-front-l", "fender-front-r", "fender-rear-l", "fender-rear-r", "running-board-l", "running-board-r"]


def jet(length, r=0.0225):
    j = revolve("tailpipe-jet", CH, EX, 0.9, [-1, 0, 0], [-2.118, 0.42, -0.35], cap_prof([[0, 0], [0, r], [length, r], [length, 0]]), role="detail")
    tag(j, "tailpipe")
    return faces(j, "-u", "+u")


def thermal50(s):
    B = {d["id"]: d for d in s.body}
    for ids, region in ((PANELS, "body-panels"), (HOODS, "hood"), (WINGS, "fenders")):
        for i in ids:
            s.th(inset(B[i], 0.00008, "th-sheet-" + i), region, "sheet")
            s.thermal.append(ref("th-" + i, i, region, "paint", INNER.get(i)))
    s.thermal.append(ref("th-roof-slab", "roof-slab", "roof", None, ["-v"]))
    for i in [d["id"] for d in s.body if d["role"] == "glazing"]:
        s.thermal.append(ref("th-" + i, i, "glazing", None, INNER.get(i)))
    for i in ("seat-front-cushion", "seat-front-back", "seat-rear-cushion", "seat-rear-back", "steering-column", "steering-wheel", "handle-front-l", "handle-front-r", "handle-rear-l", "handle-rear-r"):
        s.th(dict(B[i], id="th-" + i), "interior")
    for i in ("radiator-shell", "radiator-neck", "radiator-cap"):
        s.thof("th-" + i, i, "radiator")
    for i in ("engine-block", "engine-head", "engine-sump"):
        s.thof("th-" + i, i, "engine-bay")
    for i in [d["id"] for d in s.body if d["id"].startswith(("exhaust-manifold", "exhaust-downpipe", "exhaust-pipe", "silencer", "tail-pipe"))]:
        s.thof("th-" + i, i, "exhaust-system")
    s.th(jet(0.008), "exhaust-system")
    for i in [d["id"] for d in s.body if d["id"].startswith("tyre-") or d["id"] == "spare-tyre"]:
        s.thof("th-" + i, i, "tyres")
    for i in [d["id"] for d in s.body if d["id"].startswith("drum-")]:
        s.thof("th-" + i, i, "brakes")


def post(id, x0, x1, y0, y1, z0, z1, t=0.025):
    return loft(id, CH, P, 0.4, [sec([x0, y0, z0], t, t, 8, rotation=Qx(-math.pi / 2)), sec([x1, y1, z1], t, t, 8, rotation=Qx(-math.pi / 2))])


def structure50(s):
    for i in ("engine-bearer-1-l", "engine-bearer-1-r", "engine-bearer-2-l", "engine-bearer-2-r"):
        s.stof("st-" + i, i, CH, "member")
    for i in ("frame-rail-l", "frame-rail-r", "crossmember-front", "crossmember-radiator", "crossmember-gearbox", "crossmember-mid", "crossmember-kick", "crossmember-rear", "spring-front-l", "spring-front-r", "spring-rear-l", "spring-rear-r"):
        s.stof("st-" + i, i, CH, "member")
    for i in ("shackle-front-l", "shackle-front-r", "shackle-rear-l", "shackle-rear-r", "spring-hanger-l", "spring-hanger-r"):
        s.stof("st-" + i, i, CH, "joint")
    # coachbuilt body frame standing on the chassis: sills, pillars, cant rails, roof bows, floor bearers
    k = lambda y: 0.04 * (y - YBELT) / 0.46
    for sd, sg in (("l", 1), ("r", -1)):
        s.st(rbox(f"body-sill-{sd}", CH, P, 0.4, [2.42, 0.06, 0.05], [-0.71, 0.55, sg * 0.57], 0.005), CH, "member")
        s.st(rbox(f"cant-rail-{sd}", CH, P, 0.4, [2.26, 0.05, 0.05], [-0.67, 1.615, sg * 0.555], 0.005), CH, "member")
        for nm, xb, xt, yb in (("a", 0.49, 0.455, 0.55), ("b", -0.33, -0.33, 0.55), ("c", -1.105, -1.105, 0.63), ("d", -1.875, -1.79, 0.55)):
            s.st(post(f"pillar-{nm}-{sd}", xb, xt, yb, 1.615, sg * 0.575, sg * 0.555), CH, "member")
    for k_, x in enumerate((0.40, -0.10, -0.60, -1.00)):
        s.st(rbox(f"floor-bearer-{k_ + 1}", CH, P, 0.4, [0.05, 0.04, 1.14], [x, 0.54, 0], 0.005), CH, "member")
    for k_, x in enumerate((0.40, -0.10, -0.60, -1.10, -1.60)):
        s.st(rbox(f"roof-bow-{k_ + 1}", CH, P, 0.4, [0.035, 0.035, 1.11], [x, 1.63, 0], 0.005), CH, "member")
    s.stof("st-axle-beam", "axle-beam", AF, "member")
    s.stof("st-kingpin-l", "kingpin-l", AF, "joint")
    s.stof("st-kingpin-r", "kingpin-r", AF, "joint")
    s.stof("st-stub-axle-l", "stub-axle-l", AF, "member")
    s.stof("st-stub-axle-r", "stub-axle-r", AF, "member")
    s.stof("st-axle-tube", "axle-tube", AR, "member")
    s.stof("st-spring-seat-l", "spring-seat-l", AR, "joint")
    s.stof("st-spring-seat-r", "spring-seat-r", AR, "joint")
    for sd, sg in (("l", 1), ("r", -1)):
        s.st(rbox(f"spring-clamp-f-{sd}", AF, EX, 0.25, [0.10, 0.07, 0.06], [0, -0.035, sg * 0.34], 0.006, role="detail"), AF, "joint")
    for bone, t in ((WF, "f"), (WR, "r")):
        for sd, sg in (("l", 1), ("r", -1)):
            for part in ("hub", "spokes", "rim"):
                s.stof(f"st-{part}-{t}{sd}", f"{part}-{t}{sd}", bone, "member")
            s.st(revolve(f"wheel-bearing-{t}{sd}", bone, CI, 0.2, [0, 0, sg], [0, 0, sg * (ZW - 0.07)], [[0, 0], [0, 0.04], [0.08, 0.04], [0.08, 0]], role="detail"), bone, "joint")
            s.st(ellipsoid(f"mass-wheel-{t}{sd}", bone, EX, 0.2, [0.095, 0.095, 0.04], [0, 0, sg * ZW], role="detail"), bone, "mass")
    s.st(ellipsoid("mass-body", CH, EX, 0.2, [0.22, 0.16, 0.30], [-0.1, 0.72, 0], role="detail"), CH, "mass")
