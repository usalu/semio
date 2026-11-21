"""🧩 Part writers of the excavator's three coarser scales (1:100, 1:200, 1:500). Each writer states one assembly as the scale script asks for it: which parts stay separate, which merge into one mass and with which thickness; nothing is filtered from the 1:50 list. Every writer adds the body solid, its thermal and structural use and the drawn shape of the three views from the same numbers (ruling E1)."""
import math
import sys

F = sys.modules["r3_excavator_form"]
L, K = F.L, F.K
PS, TS, WS, CI, GL, CH, EX, EM = "paint-steel", "track-steel", "wear-steel", "cast-iron", "glass", "chrome", "exhaust-steel", "engine-metal"
U, UP, CW, EN = "undercarriage", "upper", "counterweight", "engine"
G = F.GAUGE
TWO = {"count": 2, "translation": [0, 0, -2 * G]}
H = F.HALF
TOP = F.SPROCKET[1] + F.WRAP


def stadium(length, r):
    return L.Path().M(0, -r).L(length, -r).A(r, length, r).L(0, r).A(r, 0, -r).Z()


def loop(offset, chord):
    return F.track_path(offset).pts(chord)


def ring_path(outer, inner=None):
    """One outline of a ring: the outer loop and, as its hole, the inner loop."""
    q = L.Path()
    for ring in (outer, inner):
        if ring:
            q.M(*ring[0])
            for pt in ring[1:]:
                q.L(*pt)
            q.cmd.append(("Z",))
    return q


def disc(S, id, centre, r, half, tone, material=TS, hub=None):
    """A wheel of the track as one disc on both sides, drawn as its circle in the elevation."""
    S.cyl(id, U, (centre[0], centre[1], G - half), (centre[0], centre[1], G + half), r, material=material, tone=tone, region="tracks", repeat=TWO)
    S.flat("elevation", id, U, {"kind": "ellipse", "cx": centre[0], "cy": centre[1], "rx": r, "ry": r}, [(0, 0, -G - 0.3)], role="detail", tone=tone, material=material, parent="track-belt", sub=hub or 1)


def tracks(S, chord, *, band, links, wheels, rollers, carriers, frame_low, skirt=0, solid_loop=False):
    """The undercarriage: `band` is the thickness of the shoe band, `links` the depth of the link chain under it (0: merged into the band), `wheels` the half width of sprocket and idler discs (0: merged into the track block), `rollers` and `carriers` the roller counts (0: merged into the frame), `frame_low` the underside of the track frame, `skirt` the wall of its two side skirts when the frame is an open channel over the rollers (0: a closed beam)."""
    outer = loop(0, chord)
    inner = loop(band + links, chord)
    near = [(0, 0, -G - 0.3)]
    if solid_loop:
        S.ext("track", U, outer, "w", G - 0.30, G + 0.30, material=TS, tone=0.2, region="tracks", repeat=TWO)
        S.flat("elevation", "track-belt", U, L.Path_of(outer), near, tone=0.2, region="tracks", material=TS)
    else:
        S.ext("track-band", U, outer, "w", G - 0.30, G + 0.30, holes=[loop(band, chord)], material=TS, tone=0.2, region="tracks", repeat=TWO)
        if links:
            S.ext("track-links", U, loop(band, chord), "w", G - 0.09, G + 0.09, holes=[inner], material=TS, role="detail", tone=0.25, region="tracks", repeat=TWO)
        S.flat("elevation", "track-belt", U, ring_path(outer, inner), near, tone=0.2, region="tracks", material=TS)
        r = F.WRAP - band - links
        if wheels:
            disc(S, "idler", F.IDLER, r, wheels, 0.5)
            disc(S, "sprocket", F.SPROCKET, r, wheels, 0.5)
            S.cyl("sprocket-hub", U, (F.SPROCKET[0], F.SPROCKET[1], G - 0.30), (F.SPROCKET[0], F.SPROCKET[1], G + 0.19), 0.20, material=PS, tone=0.4, region="tracks", repeat={"count": 2, "translation": [0, 0, -(2 * G - 0.11)]})
            S.flat("elevation", "sprocket-hub", U, {"kind": "ellipse", "cx": F.SPROCKET[0], "cy": F.SPROCKET[1], "rx": 0.20, "ry": 0.20}, near, role="detail", tone=0.4, material=PS, parent="track-belt", sub=3)
        step = 2 * F.RUN / 7
        if rollers:
            S.cyl("roller", U, (-F.RUN, 0.215, G - 0.10), (-F.RUN, 0.215, G + 0.10), 0.09, material=TS, tone=0.55, region="tracks", repeat={"count": 8, "translation": [L.r4(step), 0, 0]})
            S.cyl("roller-far", U, (-F.RUN, 0.215, -G - 0.10), (-F.RUN, 0.215, -G + 0.10), 0.09, material=TS, tone=0.55, region="tracks", repeat={"count": 8, "translation": [L.r4(step), 0, 0]})
            for i in range(8):
                S.flat("elevation", "roller-%d" % (i + 1), U, {"kind": "ellipse", "cx": L.r4(-F.RUN + i * step), "cy": 0.215, "rx": 0.09, "ry": 0.09}, near, role="detail", tone=0.55, material=TS, parent="track-belt", sub=1)
        if carriers:
            cy = F.top_run_y(-0.62, band + links) - 0.06
            for side, zc in (("", G), ("-far", -G)):
                S.cyl("carrier-roller" + side, U, (-0.62, cy, zc - 0.085), (-0.62, cy, zc + 0.085), 0.06, material=TS, tone=0.55, region="tracks", repeat={"count": 2, "translation": [1.24, -0.0068, 0]})
            for i, x in enumerate((-0.62, 0.62)):
                S.flat("elevation", "carrier-roller-%d" % (i + 1), U, {"kind": "ellipse", "cx": x, "cy": L.r4(F.top_run_y(x, band + links) - 0.06), "rx": 0.06, "ry": 0.06}, near, role="detail", tone=0.55, material=TS, parent="track-belt", sub=1)
        prof = [(frame_low, -0.15), (0.52, -0.15), (0.62, 0.0), (0.52, 0.15), (frame_low, 0.15)]
        if skirt:
            prof = [(frame_low, -0.15), (0.52, -0.15), (0.62, 0.0), (0.52, 0.15), (frame_low, 0.15), (frame_low, 0.15 - skirt), (0.34, 0.15 - skirt), (0.34, skirt - 0.15), (frame_low, skirt - 0.15)]
        S.ext("track-frame", U, [(v, w + G) for v, w in prof], "u", -1.70, 1.50, material=PS, tone=0.4, region="tracks", member=U, repeat=TWO)
        S.flat("elevation", "track-frame", U, L.rect(-1.70, frame_low, 1.50, 0.62), near, role="detail", tone=0.4, material=PS, parent="track-belt", sub=2)
    for side, sg in (("left", 1), ("right", -1)):
        zc = sg * G
        S.flat("front", "track-belt-" + side, U, L.rect(zc - 0.30, 0, zc + 0.30, TOP), [(2.2, 0, zc)], tone=0.25, region="tracks", material=TS)
        S.flat("plan", "track-belt-" + side, U, L.rect(-2.225, zc - 0.30, 2.225, zc + 0.30), [(0, TOP, zc)], tone=0.25, region="tracks", material=TS)


def carbody(S, *, cross, turret):
    """The centre frame between the tracks: the X of the real carbody (`cross` False) or one cross beam, and the turret with the slew bearing as one drum."""
    X = [(1.05, 1.04), (0.55, 1.04), (0.30, 0.55), (-0.30, 0.55), (-0.55, 1.04), (-1.05, 1.04), (-0.72, 0.30), (-0.72, -0.30), (-1.05, -1.04), (-0.55, -1.04), (-0.30, -0.55), (0.30, -0.55), (0.55, -1.04), (1.05, -1.04), (0.72, -0.30), (0.72, 0.30)]
    B = [(-0.72, -1.04), (0.72, -1.04), (0.72, 1.04), (-0.72, 1.04)]
    plan = B if cross else X
    half = 0.72 if cross else 1.05
    S.ext("carbody", U, plan, "v", 0.47, 0.86, material=PS, tone=0.4, region="tracks")
    S.cyl("turret", U, (0, 0.86, 0), (0, 1.06, 0), turret, material=PS, tone=0.35, region="tracks")
    S.flat("elevation", "carbody", U, L.rect(-half, 0.47, half, 0.86), [(0, 0, 1.04)], tone=0.4, region="tracks", material=PS)
    S.flat("front", "carbody", U, L.rect(-1.04, 0.47, 1.04, 0.86), [(half, 0, 0)], tone=0.4, region="tracks", material=PS)
    S.flat("plan", "carbody", U, L.Path_of(plan), [(0, 0.86, 0)], tone=0.4, region="tracks", material=PS)
    for view, pts in (("elevation", [(0, 0, -turret)]), ("front", [(turret, 0, 0)])):
        S.flat(view, "slew-ring", U, L.rect(-turret, 0.86, turret, 1.06), pts, role="body", tone=0.3, material=PS)


def supports(S):
    for name, x in (("r2", -1.35), ("r1", -0.45), ("f1", 0.45), ("f2", 1.35)):
        for side, zc in (("l", G), ("r", -G)):
            S.box("support-track-%s-%s" % (name, side), U, (x, 0.005, zc), (0.9, 0.01, 0.6), material=TS, role="ground", tone=0.2, to="structure", member=U, part="support", anchor="track-%s-%s" % (name, side))


def house(S, chord, *, toolbox, cover, grille, rail, foot, merged_deck=False, regions=None):
    """The upper structure without the cab: deck, the L-shaped engine house, the tool box, the hood cover (thickness, 0: merged), the two grilles as plates `grille` proud of the walls (0: thermal patches only), the handrail and the boom foot plates of thickness `foot`."""
    hp, dp = L.poly(F.HOUSE_PLAN), L.poly(F.DECK_PLAN)
    low = 0 if merged_deck else F.DECK_TOP
    extra = {"regions": regions} if regions else {"region": "house"}
    if not merged_deck:
        S.ext("deck", UP, dp.pts(chord), "v", 0, F.DECK_TOP, material=PS, tone=0.5, region="house")
        S.draw("deck", UP, L.rect(-2.26, 0, 1.44, F.DECK_TOP), "w", -H, H, tone=0.5, region="house", material=PS, views="ef")
        S.draw("deck", UP, L.Path_of(dp.pts(chord)), "v", 0, F.DECK_TOP, tone=0.5, region="house", material=PS, views="p")
    S.ext("house", UP, hp.pts(chord), "v", low, F.HOOD, material=PS, tone=0.62, region="house")
    S.draw("house", UP, L.rect(-2.26, low, 0.75, F.HOOD), "w", -H, F.SLOT[0], tone=0.62, material=PS, views="ef", **extra)
    S.draw("house-rear", UP, L.rect(-2.26, low, -0.38, F.HOOD), "w", F.SLOT[0], H, tone=0.62, region="house", material=PS, views="f")
    S.draw("house", UP, L.Path_of(hp.pts(chord)), "v", low, F.HOOD, tone=0.62, material=PS, views="p", **extra)
    if toolbox:
        S.box("toolbox", UP, (1.085, (low + 0.92) / 2, (F.SLOT[0] - H) / 2), (0.67, 0.92 - low, H + F.SLOT[0]), material=PS, tone=0.58, region="house")
        S.draw("toolbox", UP, L.rect(0.75, low, 1.42, 0.92), "w", -H, F.SLOT[0], tone=0.58, region="house", material=PS)
    if cover:
        S.box("hood-cover", UP, (-1.67, F.HOOD + cover / 2, 0.05), (0.98, cover, 1.70), material=PS, role="detail", tone=0.5, region="house")
        S.draw("hood-cover", UP, L.rect(-2.16, F.HOOD, -1.18, F.HOOD + cover), "w", -0.80, 0.90, role="detail", tone=0.45, material=PS, region="house")
    faces = ["+u", "-u", "+v", "-v"]
    if grille:
        S.box("vent-panel", UP, (-1.705, 0.775, -H - grille / 2), (0.75, 0.55, grille), material=EM, role="detail", tone=0.15, region="engine-compartment", thfaces=["+w"])
        S.box("radiator-grille", UP, (-1.655, 0.785, H + grille / 2), (0.95, 0.67, grille), material=EM, role="detail", tone=0.15, region="engine-compartment", thfaces=["-w"])
        S.draw("hood-grille", UP, L.rect(-2.08, 0.50, -1.33, 1.05), "w", -H - grille, -H, role="detail", tone=0.15, material=EM, region="engine-compartment")
        S.draw("radiator-grille", UP, L.rect(-2.13, 0.45, -1.18, 1.12), "w", H, H + grille, role="detail", tone=0.15, material=EM, region="engine-compartment", views="fp")
    else:
        t = 2 * S.feature
        S.box("vent-patch", UP, (-1.705, 0.775, -H + t / 2), (0.75, 0.55, t), material=EM, role="detail", tone=0.15, to="thermal", region="engine-compartment", thfaces=faces + ["+w"])
        S.box("radiator-patch", UP, (-1.655, 0.785, H - t / 2), (0.95, 0.67, t), material=EM, role="detail", tone=0.15, to="thermal", region="engine-compartment", thfaces=faces + ["-w"])
    sx, sz = -1.68, -1.02
    S.cyl("stack", UP, (sx, F.HOOD, sz), (sx, 1.74, sz), 0.06, material=EX, tone=0.25, region="exhaust-stack")
    if not regions:
        S.draw("stack", UP, L.rect(sx - 0.06, F.HOOD, sx + 0.06, 1.74), "w", sz - 0.06, sz + 0.06, role="detail", tone=0.25, material=EX, region="exhaust-stack", views="ef")
        S.draw("stack", UP, {"kind": "ellipse", "cx": sx, "cy": sz, "rx": 0.06, "ry": 0.06}, "v", F.HOOD, 1.74, role="detail", tone=0.25, material=EX, region="exhaust-stack", views="p")
    else:
        S.draw("stack", UP, L.rect(sx - 0.06, F.HOOD, sx + 0.06, 1.74), "w", sz - 0.06, sz + 0.06, role="body", tone=0.25, material=EX, views="ef")
        S.draw("stack", UP, {"kind": "ellipse", "cx": sx, "cy": sz, "rx": 0.06, "ry": 0.06}, "v", F.HOOD, 1.74, role="body", tone=0.25, material=EX, views="p")
    S.cyl("exhaust-outlet", UP, (sx, 1.74, sz), (sx, 1.74 + S.feature, sz), 0.054, material=EX, tone=0.1, to="thermal", region="exhaust-stack", tags=["exhaust-stack", "emitter"])
    if rail:
        pts = [(-0.30, F.HOOD), (-0.30, 1.62), (0.70, 1.62), (0.70, F.HOOD)]
        S.tube("hood-rail", UP, [(u, v, -1.30) for u, v in pts], 0.016, material=PS, role="detail", tone=0.3, region="house")
        S.draw("hood-rail", UP, L.band(pts, 0.032), "w", -1.316, -1.284, role="detail", tone=0.3, material=PS)
    plate = L.Path().M(-0.30, low).L(0.55, low).L(0.285, 0.897).A(0.14, 0.015, 0.897).Z()
    for name, z in (("right", F.ZC - 0.27 - foot), ("left", F.ZC + 0.27)):
        S.ext("boom-foot-" + name, UP, plate.pts(chord), "w", z, z + foot, material=PS, tone=0.4, region="house", member=UP)
    S.draw("boom-foot", UP, L.Path_of(plate.pts(chord)), "w", F.ZC - 0.27 - foot, F.ZC + 0.27 + foot, tone=0.4, region="house", material=PS)


def cab(S, chord, *, low, panes, wall):
    """The cab from `low` to the roof as a closed hollow shell of thickness `wall`: one ring of floor, front, roof and rear wall, and two side plates with their window openings; 8 mm panes close the side openings and lie on the front and rear wall: `panes` names them (front, right, left, rear)."""
    c = F.CAB
    z0, z1 = c["right"], c["left"]
    ring = L.poly([(-0.38, low), (1.44, low), (1.44, 1.02), (1.30, 1.90, 0.10), (-0.38, 1.92, 0.10)]).pts(chord)
    t = 0.008
    inner = F.band(ring, 0.0, wall)[1]
    right = L.poly([(-0.31, 0.90), (1.40, 0.90), (1.40, 1.02), (1.283, 1.78), (-0.31, 1.78)]).pts(chord)
    left = L.poly([(-0.31, 0.68), (1.40, 0.68), (1.40, 1.02), (1.283, 1.78), (-0.31, 1.78)]).pts(chord)
    S.ext("cab", UP, ring, "w", z0 + wall, z1 - wall, holes=[inner], material=PS, tone=0.62)
    S.ext("cab-side-right", UP, ring, "w", z0, z0 + wall, holes=[right] if "right" in panes else None, material=PS, tone=0.62)
    S.ext("cab-side-left", UP, ring, "w", z1 - wall, z1, holes=[left] if "left" in panes else None, material=PS, tone=0.62)
    S.ext("cab-surface", UP, ring, "w", z0, z1, material=PS, tone=0.62, to="thermal", region="house")
    S.draw("cab", UP, L.Path_of(ring), "w", z0, z1, tone=0.62, region="house", material=PS, views="ef", ids={"f": "cab-block"})
    S.draw("cab-roof", UP, L.rect(c["rear"], z0, c["front"], z1), "v", low, c["top"], tone=0.6, region="house", material=PS, views="p")
    glass = dict(material=GL, role="glazing", tone=0.85, region="cab-glazing")
    if "front" in panes:
        front = L.strip([(1.44, 0.46), (1.44, 1.02), (1.319, 1.80)], -t)
        S.ext("cab-glass-front", UP, front, "w", z0 + 0.06, z1 - 0.06, thfaces=["-u"], **glass)
        S.draw("cab-glass", UP, L.Path_of(front), "w", z0 + 0.06, z1 - 0.06, role="glazing", tone=0.85, region="cab-glazing", material=GL, views="f", parent="cab-block", sub=2)
    if "right" in panes:
        S.ext("cab-glass-side", UP, right, "w", z0, z0 + t, thfaces=["+w"], **glass)
        S.draw("cab-glass-side", UP, L.Path_of(right), "w", z0, z0 + t, role="glazing", tone=0.85, region="cab-glazing", material=GL, views="e", parent="cab", sub=2)
    if "left" in panes:
        S.ext("cab-glass-left", UP, left, "w", z1 - t, z1, thfaces=["-w"], **glass)
    if "rear" in panes:
        S.box("cab-glass-rear", UP, (-0.38 - t / 2, 1.39, (z0 + z1) / 2), (t, 0.78, z1 - z0 - 0.12), thfaces=["+u"], **glass)
    S.draw("cab-glass", UP, L.rect(1.20, z0 + 0.06, 1.43, z1 - 0.06), "v", c["top"] - 0.02, c["top"], role="glazing", tone=0.85, region="cab-glazing", material=GL, views="p", parent="cab-roof", sub=2)
    for name, zc in (("right", z0 + 0.04), ("left", z1 - 0.04)):
        S.ext("rops-post-front-" + name, UP, L.strip([(1.44, low), (1.44, 1.02), (1.313, 1.84)], 0.08), "w", zc - 0.04, zc + 0.04, material=PS, tone=0.5, to="structure", member=UP)
        S.box("rops-post-rear-" + name, UP, (-0.34, (low + 1.84) / 2, zc), (0.08, 1.84 - low, 0.08), material=PS, tone=0.5, to="structure", member=UP)
        S.box("rops-rail-" + name, UP, (0.45, 1.83, zc), (1.62, 0.08, 0.08), material=PS, tone=0.5, to="structure", member=UP)


def counterweight(S, chord):
    ring = F.cw_plan().pts(chord)
    S.ext("counterweight-body", CW, ring, "v", 0, F.CW_TOP, material=CI, tone=0.35, region="counterweight")
    S.draw("counterweight-body", CW, L.rect(0, 0, F.CW_FRONT, F.CW_TOP), "w", -H, H, tone=0.35, region="counterweight", material=CI, views="ef")
    S.draw("counterweight-body", CW, L.Path_of(ring), "v", 0, F.CW_TOP, tone=0.35, region="counterweight", material=CI, views="p")
    for k, z in enumerate((-0.60, 0.60)):
        S.box("counterweight-carrier-%d" % (k + 1), CW, (0.345, 0.22, z), (0.45, 0.28, 0.12), material=PS, tone=0.45, to="structure", member=CW)
    S.ell("mass-counterweight", CW, (0.32, 0.58, 0), (0.16, 0.30, 0.55), material=CI, tone=0.2, to="structure", member=CW, part="mass")


def engine(S, *, bed):
    S.box("engine-block", EN, (0.36, 0.33, 0.12), (0.72, 0.84, 1.70), material=EM, tone=0.2, to="thermal", region="engine-compartment")
    S.box("engine-bed", EN, (0.36, -0.07, -0.15), (0.90, 0.10, bed), material=PS, tone=0.4, to="structure", member=EN)
    S.ell("mass-engine", EN, (0.36, 0.33, -0.15), (0.22, 0.25, 0.32), material=EM, tone=0.2, to="structure", member=EN, part="mass")


def frame(S, *, cross, sides):
    """The revolving frame: two centre rails, `cross` cross members and (with `sides`) the two side beams, the slew flange and the mass of the upper structure."""
    m = dict(material=PS, tone=0.45, to="structure", member=UP)
    for k, z in enumerate((F.SLOT[0] + 0.05, F.SLOT[1] - 0.05)):
        S.box("frame-rail-%d" % (k + 1), UP, (-0.80, 0.14, z), (3.16, 0.28, 0.10), **m)
    for k, x in enumerate(cross):
        S.box("frame-cross-%d" % (k + 1), UP, (x, 0.12, 0), (0.12, 0.20, 2 * H - 0.08), **m)
    if sides:
        for k, z in enumerate((-H + 0.06, H - 0.06)):
            S.box("frame-side-%d" % (k + 1), UP, (-0.47, 0.10, z), (3.82, 0.16, 0.10), **m)
    S.cyl("frame-ring", UP, (0, 0, 0), (0, 0.08, 0), 0.66, material=PS, bore=0.50, tone=0.4, to="structure", member=UP)
    S.ell("mass-upper", UP, (-0.35, 0.62, 0), (0.42, 0.25, 0.50), material=PS, tone=0.2, to="structure", member=UP, part="mass")
    S.ell("mass-undercarriage", U, (0, 0.48, 0), (0.45, 0.16, 0.45), material=PS, tone=0.2, to="structure", member=U, part="mass")
    S.cyl("joint-slew", "slew-ring", (0.0, 0, 0), (0.12, 0, 0), 0.62, material=TS, bore=0.50, tone=0.3, to="structure", member=UP, part="joint")


def pin(S, id, bone, at, half, member, r=0.05):
    S.cyl(id, bone, (at[0], at[1], F.ZC - half), (at[0], at[1], F.ZC + half), r, material=TS, tone=0.12, to="structure", member=member, part="joint")


def cylinder(S, name, zs, spec, tip, role="detail", bore=True):
    """A hydraulic cylinder (or the boom pair) as barrel and rod, drawn as two bars."""
    bone, rod = name, name.replace("-cyl", "-rod")
    lb = spec["closed"] - 0.23
    rep = {"count": 2, "translation": [0, 0, L.r4(zs[1] - zs[0])]} if len(zs) > 1 else None
    S.cyl(name + "-barrel", bone, (0, 0, zs[0]), (lb, 0, zs[0]), spec["barrel"] / 2, material=PS, role=role, tone=0.35, region="hydraulics", repeat=rep)
    S.cyl(name + "-rod", rod, (0, 0, zs[0]), (tip, 0, zs[0]), spec["rod"] / 2, material=CH, role=role, tone=0.85, region="hydraulics", repeat=rep)
    for zz in zs:
        tag = "" if len(zs) == 1 else ("-left" if zz > F.ZC else "-right")
        S.cyl("tube-%s%s" % (name, tag), bone, (0, 0, zz), (lb, 0, zz), spec["barrel"] / 2, material=PS, bore=spec["bore"] / 2 if bore else 0, tone=0.35, to="structure", member=bone)
        S.cyl("shaft-%s%s" % (name, tag), rod, (0, 0, zz), (tip, 0, zz), spec["rod"] / 2, material=CH, tone=0.8, to="structure", member=rod)
        views = "efp" if zz == min(zs) else "fp"
        S.bar(name + "-rod" + tag, rod, (0, 0, zz), (tip, 0, zz), spec["rod"], role=role, tone=0.85, material=CH, region="hydraulics", views=views, ids={"e": name + "-rod", "f": name + "-rod" + tag, "p": name + "-rod" + tag})
        S.bar(name + "-barrel" + tag, bone, (0, 0, zz), (lb, 0, zz), spec["barrel"], role=role, tone=0.35, material=PS, region="hydraulics", views=views, ids={"e": name + "-barrel", "f": name + "-barrel" + tag, "p": name + "-barrel" + tag}, parent=name + "-rod" + tag, sub=1)


def girder(S, id, bone, ring, span, *, webs, wall):
    """The box girder of boom or stick in the structure list: the flange band around the side outline and, with `webs`, the two side plates."""
    bo, bi = F.band(ring, 0.0, wall)
    m = dict(material=PS, tone=0.5, to="structure", member=bone)
    if webs:
        S.ext(id + "-flanges", bone, bo, "w", span[0] + wall, span[1] - wall, holes=[bi], **m)
        for name, a in (("right", span[0]), ("left", span[1] - wall)):
            S.ext("%s-web-%s" % (id, name), bone, bo, "w", a, a + wall, **m)
    else:
        S.ext(id + "-box", bone, bo, "w", span[0], span[1], holes=[bi], **m)


def bars(S, id, bone, points, section, member):
    """A member as straight box bars through the points (u, v) on the centre plane."""
    for k, (a, b) in enumerate(zip(points, points[1:])):
        l = math.dist(a, b)
        S.box("%s-%d" % (id, k + 1), bone, ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2, F.ZC), (l, section[0], section[1]), rz=math.degrees(math.atan2(b[1] - a[1], b[0] - a[0])), material=PS, tone=0.5, to="structure", member=member)
