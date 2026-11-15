import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-44-lib.py")).read())
SLUG = "tram"
DUMP = dump_of(SLUG)
P_, GL, IT, SW, CI, RU, MS, RS, CA, PA = "paint", "glass", "interior-trim", "steel-wheel", "cast-iron", "rubber", "motor-steel", "resistor-steel", "carbon", "pan-alloy"
BE, GW, BM, BF, BR = [Fr(DUMP, n) for n in ("body-end", "gangway", "body-mid", "bogie-f", "bogie-r")]
WSF = {n: Fr(DUMP, n) for n in ("ws-f1", "ws-f2", "ws-r1", "ws-r2")}
DRF = {n: Fr(DUMP, n) for n in ("door-a1", "door-a2", "door-b1", "door-b2")}
PLo, PUp, PHd = Fr(DUMP, "pan-lower"), Fr(DUMP, "pan-upper"), Fr(DUMP, "pan-head")
ELE = lambda sc, i: shape(SLUG, sc, "elevation", i)
PLN = lambda sc, i: shape(SLUG, sc, "plan", i)
WS_C = {"ws-r1": 1.0, "ws-r2": 2.8, "ws-f1": 6.3, "ws-f2": 8.1}
ZW = 0.715


def rect4(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


ROOF_O = [(1.2, 3.0), (1.18, 3.15), (1.14, 3.28), (1.08, 3.36), (1.0, 3.4)]
ROOF_I = [(1.12, 3.0), (1.10, 3.12), (1.06, 3.22), (1.0, 3.29), (0.95, 3.32)]
SECT = [(-1.2, 0.28), (1.2, 0.28)] + [(z, y) for z, y in ROOF_O[:-1]] + [(1.0, 3.4), (-1.0, 3.4)] + [(-z, y) for z, y in reversed(ROOF_O[:-1])]


def roof_profile():
    outer = [(-z, y) for z, y in ROOF_O] + [(z, y) for z, y in reversed(ROOF_O)]
    inner = [(z, y) for z, y in ROOF_I] + [(-z, y) for z, y in reversed(ROOF_I)]
    pts = outer + inner
    return [[y, z] for z, y in pts]


def tube_yz(y0, y1, hw, wall, chamfer=0.0):
    out = [[y0, -hw], [y0, hw], [y1, hw], [y1, -hw]]
    hole = [[y0 + wall, -hw + wall], [y0 + wall, hw - wall], [y1 - wall, hw - wall], [y1 - wall, -hw + wall]]
    return out, hole


def gangway():
    out, hole = tube_yz(0.45, 3.25, 1.0, 0.08)
    return [Ex("gangway-bellows", GW, RU, 0.2, "yz", out, -0.45, 0.45, holes=[hole])]


def wheel_profile():
    return cap_prof([[-0.065, 0], [-0.065, 0.07], [-0.015, 0.07], [-0.015, 0.27], [-0.065, 0.27], [-0.065, 0.3], [0.065, 0.3], [0.065, 0.27], [0.015, 0.27], [0.015, 0.07], [0.065, 0.07], [0.065, 0]])


def bogies():
    b = []
    for nm, fr, bx in (("f", BF, 7.2), ("r", BR, 1.9)):
        for sd, sg in (("l", 1), ("r", -1)):
            b.append(Bx(f"bogie-side-beam-{nm}-{sd}", fr, CI, 0.25, [1.8, 0.14, 0.1], (bx, 0.38, sg * 0.5), 0.01))
        for k, dx in (("a", -0.5), ("b", 0.5)):
            b.append(Bx(f"bogie-cross-beam-{nm}-{k}", fr, CI, 0.25, [0.1, 0.14, 1.0], (bx + dx, 0.38, 0), 0.01))
        b.append(Bx(f"motor-{nm}", fr, MS, 0.35, [0.9, 0.32, 0.7], (bx, 0.44, 0), 0.03, "detail"))
    return b


def pantograph():
    b = []
    b.append(capsule("pan-lower-arm", "pan-lower", PA, 0.5, [0, 0, 0], [1.9, 0, 0], 0.03, role="body"))
    b.append(capsule("pan-upper-arm", "pan-upper", PA, 0.5, [0, 0, 0], [1.9, 0, 0], 0.03, role="body"))
    b.append(box("pan-head-plate", "pan-head", CA, 0.1, [0.62, 0.06, 1.2], [0, 0, 0], 0.03, "detail"))
    for d in b[:2]:
        d["slender"] = True
    return b


def body50():
    c = {"sc": 50, "end_l": list(WIN50_E.items()), "end_ra": [("e1", WIN50_E["e1"])], "end_rb": [(k, WIN50_E[k]) for k in ("e2", "e3", "cab")], "mid_l": list(WIN50_M.items()), "mid_ra": [("m1", WIN50_M["m1"])], "mid_rb": [("m2", WIN50_M["m2"])], "deck": (0.66, 0.75), "wall_x": 0.53, "roof": roof_profile(), "nose_rows": NOSE, "lamp": [0.08, 0.25, 0.3]}
    b = end_c(c) + mid_c(c) + doors_c(True) + gangway() + wheelsets_c(50) + bogies() + pantograph()
    for nm, x in (("a", 3.27), ("b", 5.83), ("c", 8.53)):
        b.append(Bx(f"deck-riser-{nm}", BE, IT, 0.5, [0.04, 0.3, 2.24], (x, 0.6, 0), 0.0))
    b.append(Bx("destination-sign", BE, GL, 0.6, [0.03, 0.24, 1.4], (8.65 + 0.015 * 0.85, 2.841 + 0.015 * 0.53, 0), None, "glazing", math.atan2(0.85, 1.35)))
    return b




def sides(fr, prefix, ring_or_rect, xr, win_l, win_ra, win_rb, door, b, sec_x):
    xa0, xa1, xb0, xb1 = xr
    rect_of = (lambda x0, x1, y0, y1: dedup(clip_box(ring_or_rect, x0, x1, y0, y1))) if isinstance(ring_or_rect, list) and len(ring_or_rect) > 8 else (lambda x0, x1, y0, y1: rect4(x0, y0, x1, y1))
    b.append(Ex(f"{prefix}-side-l", fr, P_, 0.3, "xy", rect_of(xa0, xb1, 0.28, 3.0), 1.12, 1.2, holes=[h for _, h in win_l]))
    b.append(Ex(f"{prefix}-side-r-a", fr, P_, 0.3, "xy", rect_of(xa0, door[0], 0.28, 3.0), -1.2, -1.12, holes=[h for _, h in win_ra]))
    b.append(Ex(f"{prefix}-side-r-door-header", fr, P_, 0.3, "xy", rect_of(door[0], door[1], 2.7, 3.0), -1.2, -1.12))
    b.append(Ex(f"{prefix}-side-r-b", fr, P_, 0.3, "xy", rect_of(door[1], xb1, 0.28, 3.0), -1.2, -1.12, holes=[h for _, h in win_rb]))
    for nm, h in win_l:
        b.append(Ex(f"pane-{nm}-l", fr, GL, 0.85, "xy", h, 1.176, 1.2, role="glazing"))
    for nm, h in win_ra + win_rb:
        b.append(Ex(f"pane-{nm}-r", fr, GL, 0.85, "xy", h, -1.2, -1.176, role="glazing"))


def end_c(c):
    b = []
    ring = dedup(ELE(c["sc"], "shell-end"))
    sides(BE, "end", ring, (0.45, 3.5, 4.8, 8.9), c["end_l"], c["end_ra"], c["end_rb"], (3.5, 4.8), b, 0)
    b.append(Ex("end-floor-mid", BE, IT, 0.5, "zx", [(-1.12, 3.25), (1.12, 3.25), (1.12, 5.85), (-1.12, 5.85)], 0.28, 0.45))
    b.append(Ex("end-floor-nose", BE, IT, 0.5, "zx", [(-1.12, 8.55), (1.12, 8.55), (1.12, 8.9), (-1.12, 8.9)], 0.28, 0.45))
    b.append(Ex("bogie-deck-r", BE, IT, 0.5, "zx", [(-1.12, 0.45), (1.12, 0.45), (1.12, 3.25), (-1.12, 3.25)], c["deck"][0], c["deck"][1]))
    b.append(Ex("bogie-deck-f", BE, IT, 0.5, "zx", [(-1.12, 5.85), (1.12, 5.85), (1.12, 8.55), (-1.12, 8.55)], c["deck"][0], c["deck"][1]))
    wall_prof = [[y, z] for z, y in SECT]
    hole = [[0.53, -0.92], [0.53, 0.92], [3.17, 0.92], [3.17, -0.92]]
    b.append(Ex("end-rear-wall", BE, P_, 0.3, "yz", wall_prof, 0.45, c["wall_x"], holes=[hole]))
    b.append(Ex("end-roof", BE, P_, 0.8, "yz", c["roof"], c["wall_x"], 8.35))
    b.append(Lf("cab-nose", BE, P_, 0.3, c["nose_rows"], e=6))
    ang = math.atan2(0.85, 1.35)
    b.append(Bx("windscreen-slope", BE, GL, 0.85, [0.03, 1.6, 2.0], (8.975 + 0.015 * 0.85, 2.325 + 0.015 * 0.53, 0), None, "glazing", ang))
    b.append(Bx("windscreen-lower", BE, GL, 0.85, [0.016, 0.25, 1.56], (9.442, 1.375, 0), None, "glazing"))
    for sd, sg in (("l", 1), ("r", -1)):
        b.append(Bx(f"headlamp-{sd}", BE, GL, 0.9, c["lamp"], (9.41, 0.975, sg * 0.85), 0.02, "glazing"))
    b.append(Bx("pan-base", BE, PA, 0.5, [0.6, 0.15, 0.8], (2.6, 3.475, 0), 0.02, "detail"))
    b.append(Bx("resistor-box", BE, RS, 0.35, [2.0, 0.24, 1.8], (5.5, 3.52, 0), 0.05))
    return b


def mid_c(c):
    b = []
    sides(BM, "mid", rect4(-5.9, 0.28, -0.53, 3.0), (-5.9, -4.0, -2.7, -0.53), c["mid_l"], c["mid_ra"], c["mid_rb"], (-4.0, -2.7), b, 0)
    b.append(Ex("mid-floor", BM, IT, 0.5, "zx", [(-1.12, -5.9), (1.12, -5.9), (1.12, -0.53), (-1.12, -0.53)], 0.28, 0.45))
    wall_prof = [[y, z] for z, y in SECT]
    hole = [[0.53, -0.92], [0.53, 0.92], [3.17, 0.92], [3.17, -0.92]]
    b.append(Ex("mid-front-wall", BM, P_, 0.3, "yz", wall_prof, -c["wall_x"], -0.45, holes=[hole]))
    b.append(Ex("mid-roof", BM, P_, 0.8, "yz", c["roof"], -5.5, -c["wall_x"]))
    band = [(1.2, -5.5), (1.1, -5.85), (0.8, -6.0), (-0.8, -6.0), (-1.1, -5.85), (-1.2, -5.5), (-1.12, -5.5), (-1.03, -5.82), (-0.78, -5.92), (0.78, -5.92), (1.03, -5.82), (1.12, -5.5)]
    b.append(Ex("mid-end-wall", BM, P_, 0.3, "zx", band, 0.28, 3.32))
    cap = [(1.2, -5.5), (1.1, -5.85), (0.8, -6.0), (-0.8, -6.0), (-1.1, -5.85), (-1.2, -5.5)]
    b.append(Ex("mid-end-roof", BM, P_, 0.8, "zx", cap, 3.32, 3.4))
    b.append(Bx("hvac-box", BM, P_, 0.45, [3.8, 0.26, 1.8], (-3.3, 3.53, 0), 0.06))
    return b


def doors_c(glass=True, thick=0.06):
    b = []
    for nm, x0, x1 in (("a1", 3.5, 4.15), ("a2", 4.15, 4.8), ("b1", -4.0, -3.35), ("b2", -3.35, -2.7)):
        fr = DRF[f"door-{nm}"]
        g0, g1 = (x0 + 0.12, x1 - 0.12)
        b.append(Ex(f"leaf-{nm}", fr, P_, 0.35, "xy", rect4(x0, 0.28, x1, 2.7), -1.2, -1.2 + thick, holes=[rect4(g0, 1.25, g1, 2.55)] if glass else None))
        if glass:
            b.append(Ex(f"leaf-glass-{nm}", fr, GL, 0.85, "xy", rect4(g0, 1.25, g1, 2.55), -1.2, -1.176, role="glazing"))
    return b


def wheel_prof(detail):
    if detail == 50:
        return wheel_profile()
    if detail == 100:
        return cap_prof([[-0.065, 0], [-0.065, 0.08], [-0.02, 0.08], [-0.02, 0.27], [-0.065, 0.27], [-0.065, 0.3], [0.065, 0.3], [0.065, 0.27], [0.02, 0.27], [0.02, 0.08], [0.065, 0.08], [0.065, 0]])
    return cap_prof([[-0.065, 0], [-0.065, 0.3], [0.065, 0.3], [0.065, 0]])


def wheelsets_c(detail):
    b = []
    for ws, cx in WS_C.items():
        fr = WSF[ws]
        for sd, sg in (("l", 1), ("r", -1)):
            b.append(Rv(f"wheel-{ws}-{sd}", fr, SW, 0.4, (0, 0, 1), (cx, 0.3, sg * ZW), wheel_prof(detail)))
            if detail <= 100:
                b.append(Rv(f"brake-disc-{ws}-{sd}", fr, CI, 0.3, (0, 0, 1), (cx, 0.3, sg * (ZW - 0.09)), cap_prof([[-0.015, 0.1], [-0.015, 0.2], [0.015, 0.2], [0.015, 0.1]]), role="detail"))
        b.append(Rv(f"axle-{ws}", fr, SW, 0.4, (0, 0, 1), (cx, 0.3, -0.65), cap_prof([[0, 0], [0, 0.075], [1.3, 0.075], [1.3, 0]]), role="detail"))
    return b


def bogies_c(cross):
    b = []
    for nm, fr, bx in (("f", BF, 7.2), ("r", BR, 1.9)):
        for sd, sg in (("l", 1), ("r", -1)):
            b.append(Bx(f"bogie-side-beam-{nm}-{sd}", fr, CI, 0.25, [1.8, 0.16, 0.12], (bx, 0.38, sg * 0.5), 0.01))
        for k, dx in cross:
            b.append(Bx(f"bogie-cross-beam-{nm}-{k}", fr, CI, 0.25, [0.12, 0.16, 1.0], (bx + dx, 0.38, 0), 0.01))
        b.append(Bx(f"motor-{nm}", fr, MS, 0.35, [0.9, 0.32, 0.7], (bx, 0.44, 0), 0.03, "detail"))
    return b


def pantograph_c(r):
    b = [capsule("pan-lower-arm", "pan-lower", PA, 0.5, [0, 0, 0], [1.9, 0, 0], r), capsule("pan-upper-arm", "pan-upper", PA, 0.5, [0, 0, 0], [1.9, 0, 0], r), box("pan-head-plate", "pan-head", CA, 0.1, [0.62, 0.06 if r < 0.05 else 0.075, 1.2], [0, 0, 0], 0.03, "detail")]
    for d in b[:2]:
        if r < 0.05:
            d["slender"] = True
    return b


def gangway_c(wall):
    out, hole = tube_yz(0.45, 3.25, 1.0, wall)
    return [Ex("gangway-bellows", GW, RU, 0.2, "yz", out, -0.45, 0.45, holes=[hole])]


def roof_c(thick):
    outer = [(-z, y) for z, y in ROOF_O] + [(z, y) for z, y in reversed(ROOF_O)]
    inner = [(z - thick + 0.08, y - thick + 0.08) for z, y in ROOF_I] + [(-z + thick - 0.08, y - thick + 0.08) for z, y in reversed(ROOF_I)]
    return [[y, z] for z, y in outer + inner]


WIN50_E = {"e1": rect4(0.9, 1.25, 3.3, 2.75), "e2": rect4(5.0, 1.25, 6.55, 2.75), "e3": rect4(6.7, 1.25, 8.0, 2.75), "cab": [[8.15, 1.25], [8.4, 1.25], [8.5, 2.75], [8.15, 2.75]]}
WIN50_M = {"m1": rect4(-5.85, 1.25, -4.25, 2.75), "m2": rect4(-2.55, 1.25, -0.85, 2.75)}
NOSE = [(8.9, 0.288, 2.444, -1.2, 1.2), (9.1, 0.283, 2.126, -1.15, 1.15), (9.3, 0.34, 1.809, -1.1, 1.1), (9.4, 0.4, 1.65, -0.95, 0.95), (9.45, 0.55, 1.5, -0.8, 0.8)]


def cfg(sc):
    E, M = WIN50_E, WIN50_M
    if sc == 100:
        e2 = rect4(5.0, 1.25, 8.0, 2.75)
        return {"sc": 100, "end_l": [("e1", E["e1"]), ("e2", e2), ("cab", E["cab"])], "end_ra": [("e1", E["e1"])], "end_rb": [("e2", e2), ("cab", E["cab"])], "mid_l": list(M.items()), "mid_ra": [("m1", M["m1"])], "mid_rb": [("m2", M["m2"])], "deck": (0.66, 0.75), "wall_x": 0.53, "roof": roof_profile(), "nose_rows": [NOSE[0], NOSE[2], NOSE[3], NOSE[4]], "lamp": [0.08, 0.25, 0.3]}
    if sc == 200:
        return {"sc": 200, "end_l": list(E.items()), "end_ra": [("e1", E["e1"])], "end_rb": [("e2", E["e2"]), ("e3", E["e3"]), ("cab", E["cab"])], "mid_l": list(M.items()), "mid_ra": [("m1", M["m1"])], "mid_rb": [("m2", M["m2"])], "deck": (0.64, 0.76), "wall_x": 0.55, "roof": roof_c(0.1), "nose_rows": [NOSE[0], NOSE[2], NOSE[4]], "lamp": [0.08, 0.25, 0.3]}
    return {"sc": 500, "end_l": [("band", rect4(0.9, 1.25, 8.45, 2.75))], "end_ra": [("band-a", rect4(0.9, 1.25, 3.45, 2.75))], "end_rb": [("band-b", rect4(4.85, 1.25, 8.45, 2.75))], "mid_l": [("band-m", rect4(-5.85, 1.25, -0.7, 2.75))], "mid_ra": [("band-ma", rect4(-5.85, 1.25, -4.05, 2.75))], "mid_rb": [("band-mb", rect4(-2.65, 1.25, -0.7, 2.75))], "deck": (0.62, 0.78), "wall_x": 0.6, "roof": roof_c(0.15), "nose_rows": [NOSE[0], NOSE[4]], "lamp": [0.08, 0.25, 0.3]}


def by(lst):
    return {d["id"]: d for d in lst}


def one_sided(d, t, id, region, layer):
    c = json.loads(json.dumps(d))
    c["id"], c["region"], c["layer"] = id, region, layer
    if c["kind"] == "extrude" and c["plane"] == "xy":
        z0, z1 = c["from"], c["to"]
        if z1 > 0 and z0 > 0:
            c["to"] = r6(z1 - t)
        elif z1 < 0 and z0 < 0:
            c["from"] = r6(z0 + t)
        else:
            c["from"], c["to"] = r6(z0 + t), r6(z1 - t)
    elif c["kind"] == "extrude":
        c["from"], c["to"] = r6(c["from"] + t), r6(c["to"] - t) if c["to"] - c["from"] > 2 * t + 0.001 else r6(c["to"])
    elif c["kind"] == "loft":
        for sc_ in c["sections"]:
            sc_["rx"], sc_["ry"] = r6(sc_["rx"] - t), r6(sc_["ry"] - t)
    return c


def skin_roof(d, id):
    c = json.loads(json.dumps(d))
    c["id"], c["region"], c["layer"] = id, "roof", "skin"
    out = [[y - 0.0, z] for y, z in c["profile"]]
    c["profile"] = [[r6(y - 0.045), z * 0.97] for y, z in out]
    return c


def thermal50(s):
    B = by(s.body)
    th = s.thermal
    wall_faces = {"end-side-l": ["-w"], "end-side-r-a": ["+w"], "end-side-r-door-header": ["+w"], "end-side-r-b": ["+w"], "mid-side-l": ["-w"], "mid-side-r-a": ["+w"], "mid-side-r-door-header": ["+w"], "mid-side-r-b": ["+w"], "end-rear-wall": ["+u"], "mid-front-wall": ["-u"]}
    wall_faces["mid-end-wall"] = ["-u"]
    for i in list(wall_faces) + ["cab-nose"]:
        th.append(ref(f"th-{i}", i, "body-panels", "insulation", wall_faces.get(i)))
        if i not in ("mid-end-wall", "end-rear-wall", "mid-front-wall"):
            th.append(one_sided(B[i], 0.05, f"th-skin-{i}", "body-panels", "skin"))
    for i in ("end-rear-wall", "mid-front-wall", "mid-end-wall"):
        c = json.loads(json.dumps(B[i]))
        c["id"], c["region"], c["layer"] = f"th-skin-{i}", "body-panels", "skin"
        c["from"], c["to"] = (r6(c["from"] + 0.04), c["to"]) if i == "end-rear-wall" else (c["from"], r6(c["to"] - 0.04)) if i == "mid-front-wall" else (r6(c["from"] + 0.04), r6(c["to"] - 0.04))
        th.append(c)
    for i in ("end-roof", "mid-roof"):
        th.append(ref(f"th-{i}", i, "roof", "insulation", ["-v"]))
        th.append(skin_roof(B[i], f"th-skin-{i}"))
    th.append(ref("th-mid-end-roof", "mid-end-roof", "roof", "insulation", ["-v"]))
    c = json.loads(json.dumps(B["mid-end-roof"]))
    c["id"], c["region"], c["layer"] = "th-skin-mid-end-roof", "roof", "skin"
    c["to"] = r6(c["to"] - 0.045)
    th.append(c)
    gl = ["pane-e1", "pane-e2", "pane-e3", "pane-cab", "pane-m1", "pane-m2"]
    for sd in ("l", "r"):
        for k in gl:
            i = f"{k}-{sd}"
            th.append(ref(f"th-{i}", i, "glazing", "pane-in", ["-w"] if sd == "l" else ["+w"]))
            for ly, t in (("gap", 0.006), ("pane-out", 0.018)):
                c = json.loads(json.dumps(B[i]))
                c["id"], c["region"], c["layer"] = f"th-{ly}-{i}", "glazing", ly
                if sd == "l":
                    c["to"] = r6(c["to"] - t)
                else:
                    c["from"] = r6(c["from"] + t)
                th.append(c)
    for i, f in (("windscreen-slope", ["-u"]), ("windscreen-lower", ["-u"]), ("destination-sign", ["-u"])):
        th.append(ref(f"th-{i}", i, "glazing", "pane-in", f))
        for ly, t in (("gap", 0.006), ("pane-out", 0.012)):
            c = json.loads(json.dumps(B[i]))
            c["id"], c["region"], c["layer"] = f"th-{ly}-{i}", "glazing", ly
            c["size"] = [r6(c["size"][0] - t), c["size"][1], c["size"][2]]
            th.append(c)
    for nm in ("a1", "a2", "b1", "b2"):
        th.append(ref(f"th-glass-{nm}", f"leaf-glass-{nm}", "glazing", "pane-in", ["+w"]))
        for ly, t in (("gap", 0.006), ("pane-out", 0.012)):
            c = json.loads(json.dumps(B[f"leaf-glass-{nm}"]))
            c["id"], c["region"], c["layer"] = f"th-{ly}-glass-{nm}", "glazing", ly
            c["from"] = r6(c["from"] + t)
            th.append(c)
        th.append(ref(f"th-leaf-{nm}", f"leaf-{nm}", "doors", "insulation"))
        c = one_sided(B[f"leaf-{nm}"], 0.05, f"th-skin-leaf-{nm}", "doors", "skin")
        th.append(c)
    th.append(ref("th-gangway-bellows", "gangway-bellows", "gangway", None))
    for n in ("f", "r"):
        th.append(ref(f"th-motor-{n}", f"motor-{n}", "traction-motors"))
    for ws in WS_C:
        for sd in ("l", "r"):
            th.append(ref(f"th-wheel-{ws}-{sd}", f"wheel-{ws}-{sd}", "wheelsets"))
            th.append(ref(f"th-brake-disc-{ws}-{sd}", f"brake-disc-{ws}-{sd}", "brake-discs"))
    th.append(ref("th-resistor-box", "resistor-box", "brake-resistor"))
    th.append(ref("th-hvac-box", "hvac-box", "hvac-unit"))
    for i in ("pan-base", "pan-lower-arm", "pan-upper-arm", "pan-head-plate"):
        th.append(ref(f"th-{i}", i, "pantograph"))
    d = revolve("resistor-fan-outlet", "body-end", RS, 0.9, "y", [5.5, 3.64, 0], cap_prof([[0, 0], [0, 0.35], [0.008, 0.35], [0.008, 0]]), role="detail")
    tag(d, "resistor-fan")
    faces(d, "-v")
    d["region"] = "brake-resistor"
    th.append(d)
    d = Rv("hvac-vent-outlet", BM, P_, 0.9, (0, 1, 0), (-3.5, 3.66, 0), cap_prof([[0, 0], [0, 0.45], [0.008, 0.45], [0.008, 0]]), role="detail")
    tag(d, "hvac-vent")
    faces(d, "-v")
    d["region"] = "hvac-unit"
    th.append(d)


def hollow(id, bone, mat, tone, y0, y1, z0, z1, x0, x1, wall):
    return ext(id, bone, mat, tone, "yz", [[y0, z0], [y1, z0], [y1, z1], [y0, z1]], x0, x1, holes=[[[y0 + wall, z0 + wall], [y1 - wall, z0 + wall], [y1 - wall, z1 - wall], [y0 + wall, z1 - wall]]])


def structure50(s):
    st = s.structure
    for sd, sg in (("l", 1), ("r", -1)):
        z0, z1 = (0.85, 1.05) if sg > 0 else (-1.05, -0.85)
        s.st(hollow(f"end-sill-{sd}", "body-end", P_, 0.2, 0.68, 0.88, z0, z1, 0.5, 8.85, 0.012), "body-end", "member")
        d = Ex(f"mid-sill-{sd}", BM, P_, 0.2, "yz", [[0.3, z0], [0.5, z0], [0.5, z1], [0.3, z1]], -5.9, -0.53, holes=[[[0.312, z0 + 0.012], [0.488, z0 + 0.012], [0.488, z1 - 0.012], [0.312, z1 - 0.012]]])
        s.st(d, "body-mid", "member")
    s.stof("gangway-member", "gangway-bellows", "gangway", "member")
    for n, fr in (("f", "bogie-f"), ("r", "bogie-r")):
        for sd in ("l", "r"):
            s.stof(f"bogie-{n}-beam-{sd}", f"bogie-side-beam-{n}-{sd}", fr, "member")
        s.stof(f"bogie-{n}-cross-a", f"bogie-cross-beam-{n}-a", fr, "member")
        s.stof(f"bogie-{n}-cross-b", f"bogie-cross-beam-{n}-b", fr, "member")
        cx = 7.2 if n == "f" else 1.9
        s.st(Rv(f"bogie-pivot-{n}", BF if n == "f" else BR, CI, 0.2, (0, 1, 0), (cx, 0.43, 0), cap_prof([[0, 0], [0, 0.1], [0.23, 0.1], [0.23, 0]]), role="detail"), fr, "joint")
    for ws in WS_C:
        s.stof(f"axle-{ws}-member", f"axle-{ws}", ws, "member")
        for sd, sg in (("l", 1), ("r", -1)):
            s.st(Rv(f"bearing-{ws}-{sd}", WSF[ws], CI, 0.2, (0, 0, 1), (WS_C[ws], 0.3, sg * ZW), cap_prof([[-0.05, 0], [-0.05, 0.055], [0.05, 0.055], [0.05, 0]]), role="detail"), ws, "joint")
    s.stof("pan-lower-member", "pan-lower-arm", "pan-lower", "member")
    s.stof("pan-upper-member", "pan-upper-arm", "pan-upper", "member")
    s.stof("pan-head-member", "pan-head-plate", "pan-head", "member")
    s.st(ellipsoid("pan-foot-joint", "pan-lower", PA, 0.5, [0.03, 0.03, 0.03], [0, 0, 0], role="detail"), "pan-lower", "joint")
    s.st(ellipsoid("pan-elbow-joint", "pan-upper", PA, 0.5, [0.03, 0.03, 0.03], [0, 0, 0], role="detail"), "pan-upper", "joint")
    s.st(ellipsoid("pan-head-joint", "pan-head", PA, 0.5, [0.03, 0.03, 0.03], [0, 0, 0], role="detail"), "pan-head", "joint")
    for nm in ("a1", "a2", "b1", "b2"):
        s.stof(f"door-{nm}-member", f"leaf-{nm}", f"door-{nm}", "member")
        fr = DRF[f"door-{nm}"]
        x0 = {"a1": 3.5, "a2": 4.15, "b1": -4.0, "b2": -3.35}[nm]
        s.st(Bx(f"door-{nm}-hanger", fr, CI, 0.2, [0.1, 0.06, 0.07], (x0 + 0.325, 2.74, -1.17), 0.005, "detail"), f"door-{nm}", "joint")
    s.st(ellipsoid("mass-end", "body-end", CI, 0.2, [0.5, 0.3, 0.6], [5.0, 1.5, 0], role="detail"), "body-end", "mass")
    s.st(Fix(ellipsoid("mass-mid", "body-mid", CI, 0.2, [0.4, 0.25, 0.6], BM.P(-2.8, 1.5, 0), role="detail")), "body-mid", "mass")
    s.st(ellipsoid("mass-bogie-f", "bogie-f", CI, 0.2, [0.2, 0.045, 0.3], [0, 0.05, 0], role="detail"), "bogie-f", "mass")


def Fix(d):
    if "transform" not in d:
        return d
    return d


def decisions50(s):
    s.dec("The tram is built from its real parts. Each module is a hollow car body: a floor (middle floor, nose floor, raised bogie decks with risers), two 80 mm side walls cut from the drawn outline with the window openings left as holes (so pillars and sills stand), the right-hand wall opened at the two door openings with a header above, a rear wall with the 1.84 m gangway opening, and a roof strip with a real roof radius extruded along the car (the 2 x 0.4 m edge fillet of the front view). Windows are their own 24 mm glazing solids flush in the openings, the cab has a slanted windscreen and a lower vertical glass strip on a solid nose cap, headlamps in the corners. Doors are four 60 mm leaves with glass in the opening of the right wall (the plan draws doors on one side only). The mid module has the plan-tapered end wall and a flat end roof. Gangway: a hollow 80 mm bellows tube. Bogies: two side beams, two cross beams, a traction motor under the raised deck; four wheelsets of two I-section steel wheels, an axle tube and two brake discs each. The pantograph is a base, two slender 60 mm arms (lower, upper) and a carbon head plate; roof equipment is the resistor box and the air-conditioning box.", "Die Strassenbahn ist aus ihren wirklichen Teilen gebaut. Jedes Modul ist ein hohler Wagenkasten: Boden (Mittelboden, Nasenboden, erhöhte Drehgestellpodeste mit Stufen), zwei 80 mm dicke Seitenwände aus dem gezeichneten Umriss mit den Fensteröffnungen als Löchern (Säulen und Brüstungen bleiben stehen), die rechte Wand an den zwei Türöffnungen mit Sturz geöffnet, eine Stirnwand mit der 1,84 m großen Übergangsöffnung und ein Dachstreifen mit echtem Dachradius, entlang des Wagens extrudiert (die 2 x 0,4 m Randfase der Vorderansicht). Die Fenster sind eigene 24-mm-Glaskörper bündig in den Öffnungen, die Kabine hat eine geneigte Frontscheibe und einen unteren senkrechten Glasstreifen auf einer massiven Nasenkappe, Scheinwerfer in den Ecken. Die Türen sind vier 60 mm dicke Flügel mit Glas in der Öffnung der rechten Wand (der Grundriss zeichnet Türen nur einer Seite). Das Mittelmodul hat die im Grundriss verjüngte Stirnwand und ein flaches Enddach. Übergang: ein hohler 80-mm-Balgschlauch. Drehgestelle: zwei Längsträger, zwei Querträger, ein Fahrmotor unter dem Podest; vier Radsätze aus je zwei I-förmigen Stahlrädern, Achsrohr und zwei Bremsscheiben. Der Stromabnehmer besteht aus Fuß, zwei schlanken 60-mm-Armen (unten, oben) und einer Kohle-Schleifleiste; Dachgeräte sind der Bremswiderstand und die Klimaanlage.")
    s.dec("Thermal: every wall, door and roof region is a nested shell pair on the same solid (insulation = outer layer = reference to the body solid, skin = real solid on the inner side of the wall); glass is three nested layers (pane-in outside, 12 mm gap, pane-out inside) on every pane; the inner faces of walls, roof and panes are section faces. Motors, wheels, brake discs, resistor box, air-conditioning box, pantograph parts by reference; the two vent outlets are thin discs on the roof boxes tagged with the emitter ids.", "Thermik: jede Wand-, Tür- und Dachregion ist ein verschachteltes Schalenpaar am selben Körper (Dämmung = Außenschicht = Verweis auf den Körper, Blech = echter Körper auf der Innenseite der Wand); Glas hat drei verschachtelte Schichten (pane-in außen, 12 mm Spalt, pane-out innen) an jeder Scheibe; die Innenflächen von Wänden, Dach und Scheiben sind Schnittflächen. Motoren, Räder, Bremsscheiben, Widerstandskasten, Klimakasten und Stromabnehmerteile sind Verweise; die beiden Auslässe sind dünne Scheiben auf den Dachkästen mit den Emitter-Kennungen.")
    s.dec("Structure: each carbody carries two hollow 200 x 200 mm sills with 12 mm walls along the lower edge, the gangway is its own member, each bogie its beams with a kingpin joint, every wheelset its axle with two bearings, the pantograph its three members with three joints, each door leaf its member with a hanger; compact masses at the centres of the two modules and of the front bogie.", "Struktur: jeder Wagenkasten trägt zwei hohle 200-x-200-mm-Schweller mit 12 mm Wand an der Unterkante, der Übergang ist ein eigener Stab, jedes Drehgestell seine Träger mit Drehzapfengelenk, jeder Radsatz seine Achse mit zwei Lagern, der Stromabnehmer seine drei Stäbe mit drei Gelenken, jeder Türflügel sein Stab mit einem Aufhänger; kompakte Massen in den Schwerpunkten der beiden Module und des vorderen Drehgestells.")



def thermal_c(s, sc):
    B = by(s.body)
    th = s.thermal
    fs = {"-side-l": ["-w"], "-side-r": ["+w"], "end-rear-wall": ["+u"], "mid-front-wall": ["-u"], "mid-end-wall": ["-u"]}
    for i in B:
        f = None
        for k, v in fs.items():
            if k in i and (k != "-side-r" or i.split("-side-r")[0] in ("end", "mid")):
                f = v
        if i.startswith(("end-side", "mid-side", "end-rear-wall", "mid-front-wall", "mid-end-wall", "cab-nose")):
            th.append(ref(f"th-{i}", i, "body-panels", None, f))
        elif i in ("end-roof", "mid-roof", "mid-end-roof"):
            th.append(ref(f"th-{i}", i, "roof", None, ["-v"]))
        elif i.startswith("pane-"):
            th.append(ref(f"th-{i}", i, "glazing", None, ["-w"] if i.endswith("-l") else ["+w"]))
        elif i.startswith("windscreen") or i == "destination-sign":
            th.append(ref(f"th-{i}", i, "glazing", None, ["-u"]))
        elif i.startswith("leaf-glass"):
            th.append(ref(f"th-{i}", i, "glazing", None, ["+w"]))
        elif i.startswith("leaf-"):
            th.append(ref(f"th-{i}", i, "doors"))
        elif i == "gangway-bellows":
            th.append(ref(f"th-{i}", i, "gangway"))
        elif i.startswith("motor-"):
            th.append(ref(f"th-{i}", i, "traction-motors"))
        elif i.startswith("wheel-"):
            th.append(ref(f"th-{i}", i, "wheelsets"))
        elif i.startswith("brake-disc"):
            th.append(ref(f"th-{i}", i, "brake-discs"))
        elif i == "resistor-box":
            th.append(ref("th-resistor-box", i, "brake-resistor"))
        elif i == "hvac-box":
            th.append(ref("th-hvac-box", i, "hvac-unit"))
        elif i in ("pan-base", "pan-lower-arm", "pan-upper-arm", "pan-head-plate"):
            th.append(ref(f"th-{i}", i, "pantograph"))
    if not any(i.startswith("motor-") for i in B):
        for n, bx, fr in (("f", 7.2, BF), ("r", 1.9, BR)):
            d = Bx(f"th-motor-{n}", fr, MS, 0.35, [0.9, 0.32, 0.7], (bx, 0.44, 0), 0.03, "detail")
            d["region"] = "traction-motors"
            th.append(d)
    if not any(i.startswith("brake-disc") for i in B):
        t = 0.015 if sc == 200 else 0.04
        for ws, cx in WS_C.items():
            for sd, sg in (("l", 1), ("r", -1)):
                d = Rv(f"th-brake-disc-{ws}-{sd}", WSF[ws], CI, 0.3, (0, 0, 1), (cx, 0.3, sg * (ZW - 0.09)), cap_prof([[-t, 0.1], [-t, 0.2], [t, 0.2], [t, 0.1]]), role="detail")
                d["region"] = "brake-discs"
                th.append(d)
    thick = 0.03 if sc == 200 else 0.075
    d = revolve("resistor-fan-outlet", "body-end", RS, 0.9, "y", [5.5, 3.64, 0], cap_prof([[0, 0], [0, 0.35], [thick, 0.35], [thick, 0]]), role="detail")
    tag(d, "resistor-fan")
    faces(d, "-v")
    d["region"] = "brake-resistor"
    th.append(d)
    d = Rv("hvac-vent-outlet", BM, P_, 0.9, (0, 1, 0), (-3.5, 3.66, 0), cap_prof([[0, 0], [0, 0.45], [thick, 0.45], [thick, 0]]), role="detail")
    tag(d, "hvac-vent")
    faces(d, "-v")
    d["region"] = "hvac-unit"
    th.append(d)


def structure_c(s, sc):
    B = by(s.body)
    wall = {100: 0.016, 200: None, 500: None}[sc]
    for sd, sg in (("l", 1), ("r", -1)):
        z0, z1 = (0.85, 1.05) if sg > 0 else (-1.05, -0.85)
        if wall:
            s.st(hollow(f"end-sill-{sd}", "body-end", P_, 0.2, 0.68, 0.88, z0, z1, 0.55, 8.8, wall), "body-end", "member")
            d = Ex(f"mid-sill-{sd}", BM, P_, 0.2, "yz", [[0.3, z0], [0.5, z0], [0.5, z1], [0.3, z1]], -5.9, -0.55, holes=[[[0.3 + wall, z0 + wall], [0.5 - wall, z0 + wall], [0.5 - wall, z1 - wall], [0.3 + wall, z1 - wall]]])
            s.st(d, "body-mid", "member")
        elif sc == 200:
            s.st(Bx(f"end-sill-{sd}", BE, P_, 0.2, [8.25, 0.2, 0.2], (4.675, 0.78, (z0 + z1) / 2), 0.01), "body-end", "member")
            s.st(Bx(f"mid-sill-{sd}", BM, P_, 0.2, [5.35, 0.2, 0.2], (-3.225, 0.4, (z0 + z1) / 2), 0.01), "body-mid", "member")
    if sc == 500:
        s.st(Bx("end-girder", BE, P_, 0.2, [8.25, 0.3, 1.2], (4.675, 0.78, 0), 0.02), "body-end", "member")
        s.st(Bx("mid-girder", BM, P_, 0.2, [5.35, 0.3, 1.2], (-3.225, 0.4, 0), 0.02), "body-mid", "member")
    s.stof("gangway-member", "gangway-bellows", "gangway", "member")
    for n, fr, cx in (("f", "bogie-f", 7.2), ("r", "bogie-r", 1.9)):
        if f"bogie-side-beam-{n}-l" in B:
            for sd in ("l", "r"):
                s.stof(f"bogie-{n}-beam-{sd}", f"bogie-side-beam-{n}-{sd}", fr, "member")
            for k in ("a", "b"):
                if f"bogie-cross-beam-{n}-{k}" in B:
                    s.stof(f"bogie-{n}-cross-{k}", f"bogie-cross-beam-{n}-{k}", fr, "member")
        else:
            frm = BF if n == "f" else BR
            s.st(Bx(f"bogie-{n}-frame", frm, CI, 0.25, [1.9, 0.18, 1.3], (cx, 0.38, 0), 0.02), fr, "member")
        if sc == 100:
            s.st(Rv(f"bogie-pivot-{n}", BF if n == "f" else BR, CI, 0.2, (0, 1, 0), (cx, 0.43, 0), cap_prof([[0, 0], [0, 0.1], [0.23, 0.1], [0.23, 0]]), role="detail"), fr, "joint")
    for ws, cx in WS_C.items():
        if f"axle-{ws}" in B:
            s.stof(f"axle-{ws}-member", f"axle-{ws}", ws, "member")
        else:
            s.st(Rv(f"axle-{ws}", WSF[ws], CI, 0.3, (0, 0, 1), (cx, 0.3, -0.65), cap_prof([[0, 0], [0, 0.075], [1.3, 0.075], [1.3, 0]]), role="detail"), ws, "member")
        if sc == 100:
            for sd, sg in (("l", 1), ("r", -1)):
                s.st(Rv(f"bearing-{ws}-{sd}", WSF[ws], CI, 0.2, (0, 0, 1), (cx, 0.3, sg * ZW), cap_prof([[-0.05, 0], [-0.05, 0.065], [0.05, 0.065], [0.05, 0]]), role="detail"), ws, "joint")
    s.stof("pan-lower-member", "pan-lower-arm", "pan-lower", "member")
    s.stof("pan-upper-member", "pan-upper-arm", "pan-upper", "member")
    s.stof("pan-head-member", "pan-head-plate", "pan-head", "member")
    if sc == 100:
        for bn in ("pan-lower", "pan-upper", "pan-head"):
            s.st(ellipsoid(f"{bn}-joint", bn, PA, 0.5, [0.03, 0.03, 0.03], [0, 0, 0], role="detail"), bn, "joint")
    for nm in ("a1", "a2", "b1", "b2"):
        s.stof(f"door-{nm}-member", f"leaf-{nm}", f"door-{nm}", "member")
    s.st(ellipsoid("mass-end", "body-end", CI, 0.2, [0.5, 0.3, 0.6], [5.0, 1.5, 0], role="detail"), "body-end", "mass")
    s.st(ellipsoid("mass-mid", "body-mid", CI, 0.2, [0.4, 0.25, 0.6], BM.P(-2.8, 1.5, 0), role="detail"), "body-mid", "mass")
    s.st(ellipsoid("mass-bogie-f", "bogie-f", CI, 0.2, [0.2, 0.045, 0.3], [0, 0.05, 0], role="detail"), "bogie-f", "mass")


def decisions_c(s, sc):
    t = {100: ("The 1:100 tram keeps the real-body architecture at 15 mm: hollow modules with the windows the 1:100 drawing shows (the middle and the third end window merge into one opening), 80 mm side walls, floor, bogie decks, rear and front walls, roof strips; four door leaves with glass; a cab nose of four stations; wheels as I-sections with web 40 mm, eight brake rings, four axle tubes; bogies with two beams, one cross beam and a motor; slender pantograph arms; the headlamp strip is two corner lenses.", "Die Strassenbahn 1:100 behält den Aufbau als echter Körper bei 15 mm: hohle Module mit den Fenstern der Zeichnung 1:100 (das mittlere und das dritte Endfenster fließen zu einer Öffnung zusammen), 80 mm dicke Seitenwände, Boden, Drehgestellpodeste, Stirnwände und Dachstreifen; vier Türflügel mit Glas; eine Kabinennase aus vier Stationen; Räder als I-Profile mit 40 mm Steg, acht Bremsringe, vier Achsrohre; Drehgestelle mit zwei Trägern, einem Querträger und einem Motor; schlanke Stromabnehmerarme; der Scheinwerferstreifen besteht aus zwei Eckgläsern."),
         200: ("At 1:200 the modules keep their hollow shells (80 mm walls, 100 mm roof strips) with the six drawn window openings and the doors as plain leaves; wheels are solid 130 mm discs; bogie frames, motors, brake discs and the nose lens details are left out of the body (they are not drawn) and live in the thermal and structure lists as real solids under the skirts.", "Bei 1:200 behalten die Module ihre hohlen Hüllen (80 mm Wände, 100 mm Dachstreifen) mit den sechs gezeichneten Fensteröffnungen und den Türen als glatte Flügel; die Räder sind massive 130-mm-Scheiben; Drehgestellrahmen, Motoren, Bremsscheiben und die Linsendetails der Nase werden nicht gezeichnet und stehen als echte Körper unter den Schürzen in den Thermik- und Strukturlisten."),
         500: ("At 1:500 each module is a hollow shell with 80 mm walls, 150 mm roof strips and one window band as drawn; the cab nose is two stations; wheels are discs; pantograph arms 120 mm slender; every other part is generalised away and kept as real solids in the thermal and structure lists.", "Bei 1:500 ist jedes Modul eine hohle Hülle mit 80 mm Wänden, 150 mm Dachstreifen und einem Fensterband wie gezeichnet; die Kabinennase besteht aus zwei Stationen; die Räder sind Scheiben; die Stromabnehmerarme sind 120 mm schlank; alle anderen Teile sind weggeneralisiert und stehen als echte Körper in den Thermik- und Strukturlisten.")}[sc]
    s.dec(*t)
    s.dec("Thermal: one reference per drawn part, section faces for the inner faces of walls, roof and panes; motors, brake discs and the two outlet discs are real solids (tagged with the emitter ids) where the body does not draw them.", "Thermik: ein Verweis je gezeichnetem Teil, Schnittflächen für die Innenseiten von Wänden, Dach und Scheiben; Motoren, Bremsscheiben und die zwei Auslassscheiben sind echte Körper (mit den Emitter-Kennungen), wo der Körper sie nicht zeichnet.")
    s.dec("Structure: sills along both modules, the gangway, bogie frames, axles, pantograph members and door leaves as members, compact masses at the module centres; joints only at 1:100.", "Struktur: Schweller entlang beider Module, der Übergang, Drehgestellrahmen, Achsen, Stromabnehmerstäbe und Türflügel als Stäbe, kompakte Massen in den Modulschwerpunkten; Gelenke nur bei 1:100.")


def build_c(sc):
    c = cfg(sc)
    s = Scale(sc)
    glass = sc <= 100
    s.body = end_c(c) + mid_c(c) + doors_c(glass, 0.06 if sc < 500 else 0.08) + gangway_c(0.08) + wheelsets_c(sc) + (bogies_c((("a", -0.5),)) if sc == 100 else []) + pantograph_c(0.03 if sc <= 200 else 0.06)
    thermal_c(s, sc)
    structure_c(s, sc)
    decisions_c(s, sc)
    chk(s)
    return s


def chk(s):
    for lst in (s.body, s.thermal, s.structure):
        ids = [d["id"] for d in lst]
        assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]


if __name__ == "__main__":
    only = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
    sc = []
    if 50 in only:
        s = Scale(50)
        s.body = body50()
        thermal50(s)
        structure50(s)
        decisions50(s)
        chk(s)
        sc.append(s)
    for k in (100, 200, 500):
        if k in only:
            sc.append(build_c(k))
    save_v(SLUG, sc)
