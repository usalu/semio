"""🏗️ The model of the used TV sideboard: materials, rig, anchors, volumes and drawings per scale, poses, clips, thermal and structural model.

Executed inside `author-used-tv-sideboard.py` with `g` (the geometry helpers) and `TXT` (the texts) in scope; `build()` returns the asset document.
"""

DENSITY = {"chipboard-white-foil": 650, "chipboard-yellowed-foil": 650, "hdf-back": 850, "pp-feet": 905, "aluminium-handle": 2700, "steel-runner": 7850, "beech-knob": 720, "packing-tape": 900, "paper-label": 800}
REGIONS = ["top-foil", "carcass-foil", "fronts", "back-panel", "feet", "handles", "knob"]
AREA = {"top-foil": 1.05, "carcass-foil": 1.54, "fronts": 0.64, "back-panel": 0.935, "feet": 0.0255, "handles": 0.0148, "knob": 0.0026}
FEET = [("foot-front-left", g.FOOT_XF, g.FOOT_Z), ("foot-front-middle", g.FOOT_XF, 0.0), ("foot-front-right", g.FOOT_XF, -g.FOOT_Z), ("foot-back-left", g.FOOT_XB, g.FOOT_Z), ("foot-back-middle", g.FOOT_XB, 0.0), ("foot-back-right", g.FOOT_XB, -g.FOOT_Z)]
GRIPS = [("grip-left", g.GRIP[0], g.GRIP[1]), ("grip-right", g.GRIP[0], -g.GRIP[1])]


def t(key):
    """🌐 The bilingual text `key` of `author-texts.py`."""
    return TXT[key]


def ref(sid, of, **label):
    """🔗 A reference solid of the thermal or structure list."""
    out = {"id": sid, "of": of}
    out.update(label)
    return out


def materials():
    """🧱 The materials with their thermal and mechanical data."""
    def m(mid, key, eps, alpha, rho, k, c, E=None, f=None, src="literature", refid="iso-10456-2007"):
        out = {"id": mid, "title": t(key), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
        if E is not None:
            out["elasticModulusPa"] = E
            out["strengthPa"] = f
        out["source"] = src
        out["reference"] = refid
        return out
    return [
        m("chipboard-white-foil", "mat-foil", 0.9, 0.25, 650, 0.13, 1700, 1.6e9, 11e6, "literature", "en-312-2010"),
        m("chipboard-yellowed-foil", "mat-foil-yellowed", 0.9, 0.34, 650, 0.13, 1700, 1.6e9, 11e6, "authored-estimate", "en-312-2010"),
        m("hdf-back", "mat-hdf", 0.9, 0.55, 850, 0.14, 1700, 3.0e9, 30e6, "authored-estimate", "en-622-5-2009"),
        m("pp-feet", "mat-pp", 0.92, 0.6, 905, 0.22, 1900, 1.3e9, 30e6, "literature", "iso-10456-2007"),
        m("aluminium-handle", "mat-alu", 0.77, 0.4, 2700, 160, 900, 70e9, 150e6, "literature", "incropera-2018"),
        m("steel-runner", "mat-steel", 0.28, 0.5, 7850, 50, 460, 210e9, 235e6, "literature", "iso-10456-2007"),
        m("beech-knob", "mat-beech", 0.9, 0.6, 720, 0.16, 1600, 14e9, 50e6, "literature", "iso-10456-2007"),
        m("packing-tape", "mat-tape", 0.9, 0.4, 900, 0.17, 1700, None, None, "authored-estimate", "iso-10456-2007"),
        m("paper-label", "mat-label", 0.9, 0.3, 800, 0.1, 1300, None, None, "authored-estimate", "iso-10456-2007"),
    ]


def poly_area(pts):
    """📐 The unsigned area of a polygon."""
    return abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2


def solid_volume(s):
    """⚖️ The volume of one 1:50 body solid with its repeat count (boxes, extrudes, revolves by Pappus, capsules, convex polyhedra)."""
    n = s.get("repeat", {}).get("count", 1)
    k = s["kind"]
    if k == "box":
        return g.box_volume(s)
    if k == "extrude":
        a = poly_area(s["profile"]) - sum(poly_area(h) for h in s.get("holes", []))
        return n * a * (s["to"] - s["from"])
    if k == "revolve":
        p = s["profile"] + [[s["profile"][-1][0], 0.0], [s["profile"][0][0], 0.0]]
        a, ry = 0.0, 0.0
        for i in range(len(p)):
            x0, y0 = p[i]
            x1, y1 = p[(i + 1) % len(p)]
            cr = x0 * y1 - x1 * y0
            a += cr / 2
            ry += (y0 + y1) * cr / 6
        return n * abs(2 * math.pi * ry)
    if k == "capsule":
        ln = math.dist(s["p0"], s["p1"])
        return n * (math.pi * s["r0"] ** 2 * ln + 4 / 3 * math.pi * s["r0"] ** 3)
    if k == "polyhedron":
        v = s["vertices"]
        vol = 0.0
        for a, b, c in s["triangles"]:
            A, B, C = v[a], v[b], v[c]
            vol += (A[0] * (B[1] * C[2] - B[2] * C[1]) - A[1] * (B[0] * C[2] - B[2] * C[0]) + A[2] * (B[0] * C[1] - B[1] * C[0])) / 6
        return n * abs(vol)
    raise ValueError(k)


def solid_centre(s):
    """🎯 The approximate centre of one solid in its bone frame (u, v)."""
    k = s["kind"]
    if k == "box":
        c = s["transform"]["translation"]
        if "repeat" in s:
            c = [c[i] + s["repeat"]["translation"][i] * (s["repeat"]["count"] - 1) / 2 for i in range(3)]
        return c
    if k == "extrude":
        xs = [p[0] for p in s["profile"]]
        ys = [p[1] for p in s["profile"]]
        mid = (s["from"] + s["to"]) / 2
        a, b = (min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2
        return {"xy": [a, b, mid], "yz": [mid, a, b], "zx": [b, mid, a]}[s["plane"]]
    if k == "revolve":
        o = s["origin"]
        h = s["profile"][-1][0] / 2
        c = [o[i] + s["axis"][i] * h for i in range(3)]
        if "repeat" in s:
            c = [c[i] + s["repeat"]["translation"][i] * (s["repeat"]["count"] - 1) / 2 for i in range(3)]
        return c
    if k == "capsule":
        return [(s["p0"][i] + s["p1"][i]) / 2 for i in range(3)]
    if k == "polyhedron":
        v = s["vertices"]
        return [(min(p[i] for p in v) + max(p[i] for p in v)) / 2 for i in range(3)]
    raise ValueError(k)


def body50():
    """🔍 The 1:50 body: every board, fitting, foot, the drawer box, the chipped corner, tape and label."""
    B = carcass_boards()
    B.append(g.back_panel("back", True))
    B.append(g.feet_row("feet-front", g.FOOT_XF, False))
    B.append(g.feet_row("feet-back", g.FOOT_XB, False))
    B.append(g.box("runner-l", "carcass", "steel-runner", (g.RUN_X0, g.XF), (g.RUN_Y0, g.RUN_Y1), (g.BOX_ZO, g.ZDI), "detail", 0.45))
    B.append(g.box("runner-r", "carcass", "steel-runner", (g.RUN_X0, g.XF), (g.RUN_Y0, g.RUN_Y1), (-g.ZDI, -g.BOX_ZO), "detail", 0.45))
    B += tape_carcass()
    B.append(g.box("price-label", "carcass", "paper-label", g.LABEL[0], (g.YT1, g.YT1 + 0.0002), g.LABEL[1], "texture", 0.97, texture=True))
    B.append(g.box("door-l", "flap-l", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (g.DOOR_Z0, g.DOOR_Z1), tone=0.92))
    B += g.bar_handle("handle-l", "flap-l", g.HANDLE_Y_DOOR, g.DOOR_ZC)
    B.append(g.box("tape-door-l", "flap-l", "packing-tape", (g.XD, g.XD + g.TAPE), (0.33, g.DOOR_Y1), g.TAPE_L, "texture", 0.72, texture=True))
    B.append(g.chipped_door("door-r", "flap-r"))
    B += g.bar_handle("handle-r", "flap-r", g.HANDLE_Y_DOOR, -g.DOOR_ZC)
    B.append(g.box("tape-door-r", "flap-r", "packing-tape", (g.XD, g.XD + g.TAPE), (0.33, g.DOOR_Y1), g.TAPE_R, "texture", 0.72, texture=True))
    B += drawer_parts(True)
    B.append(g.knob("knob-d", False))
    B.append(tape_drawer())
    return B


def tape_carcass():
    """📼 The tape pieces that stay on the carcass when the cut strips are opened: over the top front edge above each door and over the edge of the niche shelf above the drawer."""
    top = lambda Z: [(0.30, g.YT1), (g.XD, g.YT1), (g.XD, g.DOOR_Y1 + 0.001), (g.XD + g.TAPE, g.DOOR_Y1 + 0.001), (g.XD + g.TAPE, g.YT1 + g.TAPE), (0.30, g.YT1 + g.TAPE)]
    shelf = [(0.33, g.YM1), (g.XF, g.YM1), (g.XF, g.YM0), (g.XF + g.TAPE, g.YM0), (g.XF + g.TAPE, g.YM1 + g.TAPE), (0.33, g.YM1 + g.TAPE)]
    return [g.l_extrude("tape-top-l", "carcass", "packing-tape", top(g.TAPE_L), g.TAPE_L, texture=True), g.l_extrude("tape-top-r", "carcass", "packing-tape", top(g.TAPE_R), g.TAPE_R, texture=True), g.l_extrude("tape-shelf", "carcass", "packing-tape", shelf, g.TAPE_D, texture=True)]


def tape_drawer():
    """📼 The tape piece on the drawer front, bent over its top edge."""
    pts = [(g.XD, 0.13), (g.XD + g.TAPE, 0.13), (g.XD + g.TAPE, g.DRW_Y1 + g.TAPE), (g.XF, g.DRW_Y1 + g.TAPE), (g.XF, g.DRW_Y1), (g.XD, g.DRW_Y1)]
    return g.l_extrude("tape-drawer", "drawer", "packing-tape", pts, g.TAPE_D, texture=True)


def drawer_parts(fine):
    """🗄️ The drawer front and box: at 1:50 its 12 mm sides, back and 3 mm bottom; coarser one block for the box."""
    P = [g.box("drawer-front", "drawer", "chipboard-white-foil", (g.XF, g.XD), (g.DRW_Y0, g.DRW_Y1), (-g.DRW_Z, g.DRW_Z), tone=0.92)]
    if fine:
        P.append(g.box("drawer-side-l", "drawer", "chipboard-white-foil", (g.DRW_X0, g.XF), (g.BOX_Y0, g.BOX_Y1), (g.BOX_ZI, g.BOX_ZO), tone=0.84))
        P.append(g.box("drawer-side-r", "drawer", "chipboard-white-foil", (g.DRW_X0, g.XF), (g.BOX_Y0, g.BOX_Y1), (-g.BOX_ZO, -g.BOX_ZI), tone=0.84))
        P.append(g.box("drawer-back", "drawer", "chipboard-white-foil", (g.DRW_X0, g.DRW_X0 + g.BOX_T), (g.BOX_Y0, g.BOX_Y1), (-g.BOX_ZI, g.BOX_ZI), tone=0.84))
        P.append(g.box("drawer-bottom", "drawer", "hdf-back", (g.DRW_X0 + g.BOX_T, g.XF), (g.BOX_Y0, g.BOX_Y0 + g.TB), (-g.BOX_ZI, g.BOX_ZI), tone=0.8, sheet=True))
    else:
        P.append(g.box("drawer-box", "drawer", "chipboard-white-foil", (g.DRW_X0, g.XF), (g.BOX_Y0, g.BOX_Y1), (-g.BOX_ZO, g.BOX_ZO), tone=0.84))
    return P


def carcass_boards(prefix=""):
    """🪵 Top, sides, bottom, dividers and niche shelf at their true 16 mm."""
    return [
        g.box("top", "carcass", "chipboard-yellowed-foil", (g.X0, g.XD), (g.YT0, g.YT1), (-g.ZS, g.ZS), tone=0.88),
        g.box("side-l", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YF, g.YT0), (g.ZSI, g.ZS), tone=0.9),
        g.box("side-r", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YF, g.YT0), (-g.ZS, -g.ZSI), tone=0.9),
        g.box("bottom", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YF, g.YB1), (-g.ZSI, g.ZSI), tone=0.86),
        g.box("divider-l", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YB1, g.YT0), (g.ZDI, g.ZDO), tone=0.86),
        g.box("divider-r", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YB1, g.YT0), (-g.ZDO, -g.ZDI), tone=0.86),
        g.box("mid-shelf", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YM0, g.YM1), (-g.ZDI, g.ZDI), tone=0.86),
    ]


def body100():
    """🔎 The 1:100 body: boards, back with cable hole, plain turned feet, doors without the chip, the knob as a plain cylinder, the drawer box as one block, tape and label as texture; the 12 mm bar handles are below the feature and left out."""
    B = carcass_boards()
    B.append(g.back_panel("back", True))
    B.append(g.feet_row("feet-front", g.FOOT_XF, True))
    B.append(g.feet_row("feet-back", g.FOOT_XB, True))
    B += tape_carcass()
    B.append(g.box("price-label", "carcass", "paper-label", g.LABEL[0], (g.YT1, g.YT1 + 0.0002), g.LABEL[1], "texture", 0.97, texture=True))
    B.append(g.box("door-l", "flap-l", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (g.DOOR_Z0, g.DOOR_Z1), tone=0.92))
    B.append(g.box("tape-door-l", "flap-l", "packing-tape", (g.XD, g.XD + g.TAPE), (0.33, g.DOOR_Y1), g.TAPE_L, "texture", 0.72, texture=True))
    B.append(g.box("door-r", "flap-r", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (-g.DOOR_Z1, -g.DOOR_Z0), tone=0.92))
    B.append(g.box("tape-door-r", "flap-r", "packing-tape", (g.XD, g.XD + g.TAPE), (0.33, g.DOOR_Y1), g.TAPE_R, "texture", 0.72, texture=True))
    B += drawer_parts(False)
    B.append(g.knob("knob-d", True))
    B.append(tape_drawer())
    return B


def body200():
    """🗺️ The 1:200 body: the boards, the side panels and the bottom brought down to the floor as a plinth (feet and the 25 mm floor gap merged), doors, knob, drawer front and box block."""
    B = [
        g.box("top", "carcass", "chipboard-yellowed-foil", (g.X0, g.XD), (g.YT0, g.YT1), (-g.ZS, g.ZS), tone=0.88),
        g.box("side-l", "carcass", "chipboard-white-foil", (g.XB, g.XF), (0.0, g.YT0), (g.ZSI, g.ZS), tone=0.9),
        g.box("side-r", "carcass", "chipboard-white-foil", (g.XB, g.XF), (0.0, g.YT0), (-g.ZS, -g.ZSI), tone=0.9),
        g.box("plinth", "carcass", "chipboard-white-foil", (g.XB, g.XF), (0.0, g.YB1), (-g.ZSI, g.ZSI), tone=0.6),
        g.box("divider-l", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YB1, g.YT0), (g.ZDI, g.ZDO), tone=0.86),
        g.box("divider-r", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YB1, g.YT0), (-g.ZDO, -g.ZDI), tone=0.86),
        g.box("mid-shelf", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YM0, g.YM1), (-g.ZDI, g.ZDI), tone=0.86),
        g.back_panel("back", True),
        g.box("door-l", "flap-l", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (g.DOOR_Z0, g.DOOR_Z1), tone=0.92),
        g.box("door-r", "flap-r", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (-g.DOOR_Z1, -g.DOOR_Z0), tone=0.92),
    ]
    B += drawer_parts(False)
    B.append(g.knob("knob-d", True))
    return B


def body500():
    """🌍 The 1:500 body: the carcass as one mass with the top overhang over the fronts, the three fronts, the knob and the drawer box."""
    prof = [[0.0, 0.0], [g.XF, 0.0], [g.XF, g.YT0], [g.XD, g.YT0], [g.XD, g.YT1], [0.0, g.YT1]]
    B = [g.solid("carcass-mass", "carcass", "chipboard-white-foil", "body", 0.88, kind="extrude", profile=g.r6(prof), plane="xy", **{"from": -g.ZS, "to": g.ZS})]
    B.append(g.box("door-l", "flap-l", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (g.DOOR_Z0, g.DOOR_Z1), tone=0.92))
    B.append(g.box("door-r", "flap-r", "chipboard-white-foil", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1), (-g.DOOR_Z1, -g.DOOR_Z0), tone=0.92))
    B += drawer_parts(False)
    B.append(g.knob("knob-d", True))
    return B


def handle_bars(limit):
    """🔩 Thermal-only stand-ins of the two bar handles at a coarse scale: a square bar of the scale's limit, 120 mm long, carrying the handles region."""
    out = []
    for bone, y, zc, sid in (("flap-l", g.HANDLE_Y_DOOR, g.DOOR_ZC, "th-handle-l"), ("flap-r", g.HANDLE_Y_DOOR, -g.DOOR_ZC, "th-handle-r")):
        s = g.box(sid, bone, "aluminium-handle", (0.428 - limit, 0.428), (y - limit / 2, y + limit / 2), (zc - 0.06, zc + 0.06), "detail", 0.5)
        s["region"] = "handles"
        out.append(s)
    return out


def thermal(scale, body):
    """🌡️ The thermal list of a scale: references to the board surfaces, own solids where a coarse body merges two regions."""
    ids = {s["id"] for s in body}
    th = []
    region_of = {"top": "top-foil", "side-l": "carcass-foil", "side-r": "carcass-foil", "bottom": "carcass-foil", "divider-l": "carcass-foil", "divider-r": "carcass-foil", "mid-shelf": "carcass-foil", "back": "back-panel", "feet-front": "feet", "feet-back": "feet", "door-l": "fronts", "door-r": "fronts", "drawer-front": "fronts", "handle-l-bar": "handles", "handle-l-legs": "handles", "handle-r-bar": "handles", "handle-r-legs": "handles", "knob-d": "knob"}
    for sid, region in region_of.items():
        if sid in ids:
            th.append(ref(f"th-{sid}", sid, region=region))
    if scale == "100":
        th += handle_bars(0.015)
    if scale in ("200", "500"):
        for aid, x, z in FEET:
            th.append({**g.box(f"th-{aid}", "carcass", "pp-feet", (x - g.FOOT_R, x + g.FOOT_R), (0.0, g.YF), (z - g.FOOT_R, z + g.FOOT_R), "body", 0.35), "region": "feet"})
        th += handle_bars(0.016)
    if scale == "200":
        th.append({**g.box("th-plinth-board", "carcass", "chipboard-white-foil", (g.XB, g.XF), (g.YF, g.YB1), (-g.ZSI, g.ZSI), "body", 0.86), "region": "carcass-foil"})
    if scale == "500":
        u = [[g.YF, -g.ZS], [g.YT0, -g.ZS], [g.YT0, -g.ZSI], [g.YB1, -g.ZSI], [g.YB1, g.ZSI], [g.YT0, g.ZSI], [g.YT0, g.ZS], [g.YF, g.ZS]]
        h = [[g.YB1, -g.ZDO], [g.YT0, -g.ZDO], [g.YT0, -g.ZDI], [g.YM1, -g.ZDI], [g.YM1, g.ZDI], [g.YT0, g.ZDI], [g.YT0, g.ZDO], [g.YB1, g.ZDO], [g.YB1, g.ZDI], [g.YM0, g.ZDI], [g.YM0, -g.ZDI], [g.YB1, -g.ZDI]]
        th.append({**g.box("th-top", "carcass", "chipboard-yellowed-foil", (g.X0, g.XD), (g.YT0, g.YT1), (-g.ZS, g.ZS), "body", 0.88), "region": "top-foil"})
        th.append({**g.solid("th-carcass-u", "carcass", "chipboard-white-foil", "body", 0.88, kind="extrude", profile=g.r6(u), plane="yz", **{"from": g.XB, "to": g.XF}), "region": "carcass-foil"})
        th.append({**g.solid("th-dividers-h", "carcass", "chipboard-white-foil", "body", 0.86, kind="extrude", profile=g.r6(h), plane="yz", **{"from": g.XB, "to": g.XF}), "region": "carcass-foil"})
        th.append({**g.back_panel("th-back", True), "region": "back-panel"})
    return th


def structure(scale, body):
    """🏗️ The structure list of a scale: boards, feet, fronts and drawer as members, hinge pins and runners as joints and the four bone masses; the load reaches the floor through the feet (or the hands at the grip anchors), which are switched pose contacts, not permanent supports."""
    ids = {s["id"] for s in body}
    st = []
    members = {"top": "carcass", "side-l": "carcass", "side-r": "carcass", "bottom": "carcass", "plinth": "carcass", "divider-l": "carcass", "divider-r": "carcass", "mid-shelf": "carcass", "back": "carcass", "feet-front": "carcass", "feet-back": "carcass", "carcass-mass": "carcass", "door-l": "flap-l", "door-r": "flap-r", "drawer-front": "drawer", "drawer-side-l": "drawer", "drawer-side-r": "drawer", "drawer-back": "drawer", "drawer-bottom": "drawer", "drawer-box": "drawer"}
    for sid, member in members.items():
        if sid in ids:
            st.append(ref(f"m-{sid}", sid, member=member, part="member"))
    feature = {"50": 0.0075, "100": 0.015, "200": 0.016, "500": 0.02}[scale]
    marker = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    r = max(0.005, feature / 2)
    for bone in ("flap-l", "flap-r"):
        st.append(g.solid(f"j-hinge-{bone[-1]}", bone, "steel-runner", "detail", 0.3, kind="capsule", p0=[0.0, 0.0, -0.28], p1=[0.0, 0.0, 0.28], r0=r, r1=r, member=bone, part="joint"))
    if scale == "50":
        st.append(ref("j-runner-l", "runner-l", member="drawer", part="joint"))
        st.append(ref("j-runner-r", "runner-r", member="drawer", part="joint"))
    else:
        w = max(feature, 0.0125)
        zc = (g.BOX_ZO + g.ZDI) / 2
        for side, sgn in (("l", 1), ("r", -1)):
            z0, z1 = sorted((sgn * (zc - w / 2), sgn * (zc + w / 2)))
            st.append({**g.box(f"j-runner-{side}", "carcass", "steel-runner", (g.RUN_X0, g.XF), (g.RUN_Y0, g.RUN_Y0 + max(feature, 0.02)), (z0, z1), "detail", 0.45), "member": "drawer", "part": "joint"})
    rm = marker / 2 if scale != "50" else 0.0075
    for bone, (kg, cu, cv) in MASSES.items():
        wz = 0.0
        st.append(g.solid(f"mass-{bone}", bone, "steel-runner", "detail", 0.3, kind="ellipsoid", radii=[rm, rm, rm], transform={"translation": g.r6([cu, cv, wz])}, member=bone, part="mass"))
    return st


def compute_masses():
    """⚖️ Bone masses and centres from the 1:50 solids: volume times density, summed per bone."""
    acc = {}
    for s in body50():
        v = solid_volume(s)
        kg = v * DENSITY[s["material"]]
        c = solid_centre(s)
        a = acc.setdefault(s["bone"], [0.0, 0.0, 0.0])
        a[0] += kg
        a[1] += kg * c[0]
        a[2] += kg * c[1]
    return {b: (round(a[0], 3), round(a[1] / a[0], 4), round(a[2] / a[0], 4)) for b, a in acc.items()}


MASSES = compute_masses()


def rig():
    """🦴 The rig: floor reference, carcass, two drop-down flaps hinged at the bottom front edge, the drawer on its runners."""
    def mass(bone):
        kg, cu, cv = MASSES[bone]
        return {"kg": kg, "comX": cu, "comY": cv, "distribution": "point", "source": "authored-estimate", "reference": "en-312-2010"}
    door_section = {"areaM2": g.r6(0.598 * g.T), "secondMomentM4": float(f"{0.598 * g.T ** 3 / 12:.4e}"), "extremeFibreM": g.T / 2, "material": "chipboard-white-foil", "title": t("section-door")}
    return {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": t("bone-ground"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "carcass", "parent": "ground", "title": t("bone-carcass"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": g.XD, "joint": {"type": "fixed"}, "mass": mass("carcass")},
        {"id": "flap-l", "parent": "carcass", "title": t("bone-flap-l"), "x": g.XF, "y": g.YB1, "z": g.DOOR_ZC, "angleDeg": 90, "lengthM": g.r6(g.DOOR_Y1 - g.DOOR_Y0), "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 0}, "mass": mass("flap-l"), "section": door_section},
        {"id": "flap-r", "parent": "carcass", "title": t("bone-flap-r"), "x": g.XF, "y": g.YB1, "z": -g.DOOR_ZC, "angleDeg": 90, "lengthM": g.r6(g.DOOR_Y1 - g.DOOR_Y0), "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 0}, "mass": mass("flap-r"), "section": door_section},
        {"id": "drawer", "parent": "carcass", "title": t("bone-drawer"), "x": g.DRW_X0, "y": g.DRW_Y0, "angleDeg": 0, "lengthM": g.r6(g.XD - g.DRW_X0), "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.3}, "mass": mass("drawer")},
    ]}


def anchors():
    """📍 Six glide feet on the floor, two grip points under the bottom board at the ends."""
    out = []
    for aid, x, z in FEET:
        out.append({"id": aid, "bone": "carcass", "x": x, "y": 0.0, "z": z, "kind": "contact", "title": t(f"anchor-{aid}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.4})
    for aid, x, z in GRIPS:
        out.append({"id": aid, "bone": "carcass", "x": x, "y": g.YF, "z": z, "kind": "support", "title": t(f"anchor-{aid}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.08, "frictionCoefficient": 0.6})
    return out


def volumes():
    """🧊 The four handcrafted volumes."""
    bodies = {"50": body50(), "100": body100(), "200": body200(), "500": body500()}
    out = {}
    for sc, body in bodies.items():
        out[sc] = {"status": "authored", "denominator": int(sc), "body": body, "thermal": thermal(sc, body), "structure": structure(sc, body), "decisions": [t(f"vol{sc}-body"), t(f"vol{sc}-lists")]}
    return out


exec(compile((HERE / "author-drawings.py").read_text(encoding="utf-8"), str(HERE / "author-drawings.py"), "exec"))
exec(compile((HERE / "author-scenario.py").read_text(encoding="utf-8"), str(HERE / "author-scenario.py"), "exec"))


def build():
    """📄 The whole asset document."""
    total = round(sum(v[0] for v in MASSES.values()), 1)
    asset = {
        "id": "used-tv-sideboard",
        "category": "furniture",
        "title": t("title"),
        "summary": t("summary"),
        "tags": ["used", "household", "second-hand", "living", "tv-sideboard", "lowboard", "storage", "chipboard", "flap-door", "drawer", "interior"],
        "provenance": provenance(),
        "dimensions": {"heightM": round(g.YT1, 3), "widthM": round(g.BAR_X + g.BAR_R, 3), "depthM": round(2 * g.ZS, 3), "massKg": total},
        "materials": materials(),
        "rig": rig(),
        "anchors": anchors(),
        "representations": representations(),
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(),
        "structure": structure_model(),
        "volumes": volumes(),
        "presentation": {"defaultPose": "stored", "defaultClip": "flap-and-drawer", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
