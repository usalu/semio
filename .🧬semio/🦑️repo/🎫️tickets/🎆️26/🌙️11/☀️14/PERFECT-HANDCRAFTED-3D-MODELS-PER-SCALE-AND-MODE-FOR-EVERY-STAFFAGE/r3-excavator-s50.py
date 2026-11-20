"""🏗️ Excavator at 1:50 (smallest feature 7.5 mm): every assembly of the real machine, part by part, as body solid and as the drawn shape of the three views; the thermal list references the surfaces by region and adds the engine internals and the exhaust outlet; the structure list is the welded load path (track frames, X carbody, revolving frame, box girders of boom and stick as four plates each, cylinder tubes and rods, links, pins)."""
import math
import sys

F = sys.modules["r3_excavator_form"]
L, K = F.L, F.K
PS, TS, WS, CI, GL, CH, EX, EM = "paint-steel", "track-steel", "wear-steel", "cast-iron", "glass", "chrome", "exhaust-steel", "engine-metal"
U, UP, CW, EN = "undercarriage", "upper", "counterweight", "engine"
G2 = 2 * F.GAUGE
TWO = {"count": 2, "translation": [0, 0, -G2]}
PITCH = 0.19 / 3


def stadium(length, r):
    return L.Path().M(0, -r).L(length, -r).A(r, length, r).L(0, r).A(r, 0, -r).Z()


def ticks(points):
    q = L.Path()
    for a, b in points:
        q.M(*a).L(*b)
    return q


def tracks(S):
    g, p, r = F.GROUSER, F.PLATE, F.RAIL
    T = F.track(0)
    S.ext("track-shoes", U, F.track_path(g).pts(), "w", F.GAUGE - 0.30, F.GAUGE + 0.30, holes=[F.track_path(g + p).pts()], material=TS, tone=0.2, region="tracks", repeat=TWO)
    for name, z in (("outer", 0.07), ("inner", -0.07)):
        S.ext("track-rail-" + name, U, F.track_path(g + p).pts(), "w", F.GAUGE + z - 0.0225, F.GAUGE + z + 0.0225, holes=[F.track_path(g + p + r).pts()], material=TS, role="detail", tone=0.25, region="tracks", repeat=TWO)
    rib = dict(material=TS, role="detail", tone=0.3)
    runs = []
    n = int(2 * F.RUN / PITCH)
    runs.append(("bottom", (-F.RUN + (2 * F.RUN - n * PITCH) / 2, 0), 0.0, n + 1))
    for name, a, b in (("ramp-front", T["p1"], T["ib"]), ("top", T["it"], T["st"]), ("ramp-rear", T["sb"], T["p0"])):
        d = math.dist(a, b)
        n = int(d / PITCH)
        ang = math.atan2(b[1] - a[1], b[0] - a[0])
        off = (d - n * PITCH) / 2 + (PITCH / 2 if name != "top" else 0)
        runs.append((name, (a[0] + off * math.cos(ang), a[1] + off * math.sin(ang)), math.degrees(ang), n + (1 if name == "top" else 0)))
    marks = {"bottom": [], "top": [], "idler": [], "sprocket": []}
    for side, zc in (("l", F.GAUGE), ("r", -F.GAUGE)):
        for name, start, deg, count in runs:
            c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
            centre = (start[0] - s * g / 2, start[1] + c * g / 2, zc)
            S.box("grouser-%s-%s" % (name, side), U, centre, (0.018, g, F.SHOE), rz=deg, repeat={"count": count, "translation": [L.r4(PITCH * c), L.r4(PITCH * s), 0]}, **rib)
            if side == "r":
                key = "bottom" if name == "bottom" else "top" if name == "top" else "idler" if name == "ramp-front" else "sprocket"
                for k in range(count):
                    x, y = start[0] + k * PITCH * c, start[1] + k * PITCH * s
                    marks[key].append(((x, y), (x - s * g, y + c * g)))
        for name, centre, (a0, a1) in (("idler", F.IDLER, T["idler"]), ("sprocket", F.SPROCKET, T["sprocket"])):
            rr = F.WRAP - g / 2
            da = PITCH / rr
            n = int((a1 - a0) / da)
            a = a0 + ((a1 - a0) - n * da) / 2 + da / 2
            S.box("grouser-%s-%s" % (name, side), U, (centre[0] + rr * math.cos(a), centre[1] + rr * math.sin(a), zc), (0.018, g, F.SHOE), rz=math.degrees(a) - 90, repeat={"count": n, "rotationRad": L.r4(da), "rotationAxis": [0, 0, 1], "origin": [centre[0], centre[1], zc]}, **rib)
            if side == "r":
                for k in range(n):
                    b = a + k * da
                    marks[name].append(((centre[0] + F.WRAP * math.cos(b), centre[1] + F.WRAP * math.sin(b)), (centre[0] + (F.WRAP - g) * math.cos(b), centre[1] + (F.WRAP - g) * math.sin(b))))
    wheel = dict(material=TS, role="detail", region="tracks", repeat=TWO)
    S.rev("idler", U, (F.IDLER[0], F.IDLER[1], F.GAUGE), (0, 0, 1), [(-0.095, 0), (-0.095, 0.275), (-0.03, 0.275), (-0.03, 0.305), (0.03, 0.305), (0.03, 0.275), (0.095, 0.275), (0.095, 0)], tone=0.5, **wheel)
    S.cyl("idler-hub", U, (F.IDLER[0], F.IDLER[1], F.GAUGE - 0.145), (F.IDLER[0], F.IDLER[1], F.GAUGE + 0.145), 0.085, tone=0.25, **wheel)
    S.cyl("sprocket", U, (F.SPROCKET[0], F.SPROCKET[1], F.GAUGE - 0.035), (F.SPROCKET[0], F.SPROCKET[1], F.GAUGE + 0.035), 0.30, tone=0.5, **wheel)
    for side, zc in (("l", F.GAUGE), ("r", -F.GAUGE)):
        S.box("sprocket-teeth-" + side, U, (F.SPROCKET[0] + 0.32, F.SPROCKET[1], zc), (0.07, 0.045, 0.06), material=TS, role="detail", tone=0.5, region="tracks", repeat={"count": 23, "rotationRad": L.r4(2 * math.pi / 23), "rotationAxis": [0, 0, 1], "origin": [F.SPROCKET[0], F.SPROCKET[1], zc]})
    S.cyl("sprocket-hub", U, (F.SPROCKET[0], F.SPROCKET[1], F.GAUGE - 0.30), (F.SPROCKET[0], F.SPROCKET[1], F.GAUGE + 0.19), 0.20, material=PS, role="detail", tone=0.4, region="tracks", repeat={"count": 2, "translation": [0, 0, -(2 * F.GAUGE - 0.11)]})
    S.cyl("sprocket-cap", U, (F.SPROCKET[0], F.SPROCKET[1], F.GAUGE + 0.19), (F.SPROCKET[0], F.SPROCKET[1], F.GAUGE + 0.23), 0.12, material=PS, role="detail", tone=0.25, region="tracks", repeat={"count": 2, "translation": [0, 0, -(2 * F.GAUGE + 0.42)]})
    x0 = -F.RUN
    step = 2 * F.RUN / 7
    roller = [(-0.10, 0), (-0.10, 0.09), (-0.03, 0.09), (-0.03, 0.115), (0.03, 0.115), (0.03, 0.09), (0.10, 0.09), (0.10, 0)]
    cy = top = None
    for side, zc in (("l", F.GAUGE), ("r", -F.GAUGE)):
        S.rev("roller-" + side, U, (x0, 0.215, zc), (0, 0, 1), roller, material=TS, role="detail", tone=0.55, region="tracks", repeat={"count": 8, "translation": [L.r4(step), 0, 0]})
        S.cyl("roller-shaft-" + side, U, (x0, 0.215, zc - 0.15), (x0, 0.215, zc + 0.15), 0.035, material=TS, role="detail", tone=0.25, region="tracks", repeat={"count": 8, "translation": [L.r4(step), 0, 0]})
        cy = F.top_run_y(-0.62, g + p + r) - 0.06
        S.cyl("carrier-roller-" + side, U, (-0.62, cy, zc - 0.085), (-0.62, cy, zc + 0.085), 0.06, material=TS, role="detail", tone=0.55, region="tracks", repeat={"count": 2, "translation": [1.24, -0.0068, 0]})
        S.box("carrier-post-" + side, U, (-0.62, (cy + 0.60) / 2 - 0.02, zc), (0.07, cy - 0.60 + 0.04, 0.06), material=PS, role="detail", tone=0.4, region="tracks", repeat={"count": 2, "translation": [1.24, 0, 0]})
    frame = [(0.215, -0.15), (0.215, -0.115), (0.34, -0.115), (0.34, 0.115), (0.215, 0.115), (0.215, 0.15), (0.52, 0.15), (0.62, 0.0), (0.52, -0.15)]
    S.ext("track-frame", U, [(v, w + F.GAUGE) for v, w in frame], "u", -1.70, 1.50, material=PS, role="detail", tone=0.4, region="tracks", member=U, repeat=TWO)
    for name, z in (("outer", 0.125), ("inner", -0.125)):
        S.box("idler-yoke-" + name, U, (1.62, 0.45, F.GAUGE + z), (0.56, 0.20, 0.03), material=PS, role="detail", tone=0.4, region="tracks", repeat=TWO)
    X = [(1.05, 1.04), (0.55, 1.04), (0.30, 0.55), (-0.30, 0.55), (-0.55, 1.04), (-1.05, 1.04), (-0.72, 0.30), (-0.72, -0.30), (-1.05, -1.04), (-0.55, -1.04), (-0.30, -0.55), (0.30, -0.55), (0.55, -1.04), (1.05, -1.04), (0.72, -0.30), (0.72, 0.30)]
    S.ext("carbody", U, X, "v", 0.47, 0.86, material=PS, tone=0.4, region="tracks")
    S.cyl("turret", U, (0, 0.86, 0), (0, 0.94, 0), 0.70, material=PS, role="detail", tone=0.35, region="tracks")
    S.cyl("slew-bearing", "slew-ring", (0, 0, 0), (0.12, 0, 0), 0.66, material=TS, role="detail", tone=0.3, region="tracks")
    near = -F.GAUGE - 0.30
    belt = F.track_path(g)
    pb = [(x, y, near) for x, y in belt.pts()]
    belt.cmd += [("Z",)] + F.track_path(g + p + r).cmd
    S.flat("elevation", "carbody", U, L.rect(-1.05, 0.47, 1.05, 0.86), [(-1.05, 0.47, 1.04), (1.05, 0.86, 1.04)], tone=0.4, region="tracks", material=PS)
    S.flat("elevation", "track-belt", U, belt, pb, tone=0.2, region="tracks", material=TS)
    dots = dict(role="detail", material=TS, parent="track-belt")
    for i in range(8):
        S.flat("elevation", "roller-%d" % (i + 1), U, {"kind": "ellipse", "cx": L.r4(x0 + i * step), "cy": 0.215, "rx": 0.115, "ry": 0.115}, pb, tone=0.55, sub=1, **dots)
    for i, x in enumerate((-0.62, 0.62)):
        S.flat("elevation", "carrier-roller" + ("" if i == 0 else "-2"), U, {"kind": "ellipse", "cx": x, "cy": L.r4(F.top_run_y(x, g + p + r) - 0.06), "rx": 0.06, "ry": 0.06}, pb, tone=0.55, sub=1, **dots)
    S.flat("elevation", "idler", U, {"kind": "ellipse", "cx": F.IDLER[0], "cy": F.IDLER[1], "rx": 0.305, "ry": 0.305}, pb, tone=0.5, sub=1, **dots)
    S.flat("elevation", "sprocket", U, {"kind": "ellipse", "cx": F.SPROCKET[0], "cy": F.SPROCKET[1], "rx": 0.335, "ry": 0.335}, pb, tone=0.5, sub=1, **dots)
    S.flat("elevation", "track-frame", U, L.poly([(-1.70, 0.215), (1.50, 0.215), (1.50, 0.35), (1.90, 0.35), (1.90, 0.55), (1.50, 0.55), (1.50, 0.62), (-1.70, 0.62)]), pb, tone=0.4, sub=2, role="detail", material=PS, parent="track-belt")
    S.flat("elevation", "idler-hub", U, {"kind": "ellipse", "cx": F.IDLER[0], "cy": F.IDLER[1], "rx": 0.085, "ry": 0.085}, pb, tone=0.25, sub=3, **dots)
    S.flat("elevation", "sprocket-hub", U, {"kind": "ellipse", "cx": F.SPROCKET[0], "cy": F.SPROCKET[1], "rx": 0.20, "ry": 0.20}, pb, tone=0.4, sub=3, role="detail", material=PS, parent="track-belt")
    S.flat("elevation", "sprocket-cap", U, {"kind": "ellipse", "cx": F.SPROCKET[0], "cy": F.SPROCKET[1], "rx": 0.12, "ry": 0.12}, pb, tone=0.25, sub=4, role="detail", material=PS, parent="track-belt")
    for key in ("bottom", "top", "idler", "sprocket"):
        S.flat("elevation", "grousers-" + key, U, L.bands(marks[key], 0.018), pb, role="detail", tone=0.3, material=TS, parent="track-belt", sub=5)
    ring = L.poly([(-0.70, 0.86), (0.70, 0.86), (0.70, 0.94), (0.66, 0.94), (0.66, 1.06), (-0.66, 1.06), (-0.66, 0.94), (-0.70, 0.94)])
    S.flat("elevation", "slew-ring", U, ring, [(-0.7, 0.86, -0.7), (0.7, 1.06, 0.7)], role="detail", tone=0.3, material=PS)
    lo, hi = 0, F.SPROCKET[1] + F.WRAP
    ys = [F.IDLER[1] + (F.WRAP - g) * math.sin(math.radians(a)) for a in (-62, -34, -6, 22, 50, 76)]
    for side, sg in (("left", 1), ("right", -1)):
        zc = sg * F.GAUGE
        fpts = [(2.2, lo, zc - 0.3), (2.2, hi, zc + 0.3)]
        S.flat("front", "track-belt-" + side, U, L.rect(zc - 0.30, lo, zc + 0.30, hi), fpts, tone=0.25, region="tracks", material=TS)
        S.flat("front", "grousers-front-" + side, U, ticks([((zc - 0.30, y), (zc + 0.30, y)) for y in ys]), fpts, role="texture", tone=0.4, filled=False, parent="track-belt-" + side, sub=1)
        ppts = [(-2.2, hi, zc - 0.3), (2.2, hi, zc + 0.3)]
        S.flat("plan", "track-belt-" + side, U, L.rect(-2.225, zc - 0.30, 2.225, zc + 0.30), ppts, tone=0.25, region="tracks", material=TS)
        S.flat("plan", "grousers-plan-" + side, U, ticks([((-1.80 + k * 0.19, zc - 0.30), (-1.80 + k * 0.19, zc + 0.30)) for k in range(20)]), ppts, role="texture", tone=0.4, filled=False, parent="track-belt-" + side, sub=1)
    S.flat("front", "carbody", U, L.rect(-1.04, 0.47, 1.04, 0.86), [(1.05, 0.47, -1.04), (1.05, 0.86, 1.04)], tone=0.4, region="tracks", material=PS)
    S.flat("front", "slew-ring", U, ring, [(0.7, 0.86, -0.7), (0.7, 1.06, 0.7)], role="detail", tone=0.3, material=PS)
    S.flat("plan", "carbody", U, L.poly(X), [(x, 0.86, z) for x, z in X], tone=0.4, region="tracks", material=PS)
    S.cyl("carbody-ring", U, (0, 0.47, 0), (0, 0.94, 0), 0.70, material=PS, bore=0.62, tone=0.35, to="structure", member=U)
    for k, (sx, sz) in enumerate(((1, 1), (-1, 1), (-1, -1), (1, -1))):
        S.box("carbody-leg-%d" % (k + 1), U, (sx * 0.64, 0.66, sz * 0.78), (0.80, 0.34, 0.26), material=PS, tone=0.4, rot=L.quat_y(math.degrees(math.atan2(-sz * 0.60, sx * 0.36))), to="structure", member=U)
    for name in ("r2", "r1", "f1", "f2"):
        x = {"r2": -1.35, "r1": -0.45, "f1": 0.45, "f2": 1.35}[name]
        for side, zc in (("l", F.GAUGE), ("r", -F.GAUGE)):
            S.box("support-track-%s-%s" % (name, side), U, (x, 0.005, zc), (0.9, 0.01, 0.6), material=TS, role="ground", tone=0.2, to="structure", member=U, part="support", anchor="track-%s-%s" % (name, side))
    S.ell("mass-undercarriage", U, (0, 0.48, 0), (0.45, 0.22, 0.45), material=PS, tone=0.2, to="structure", member=U, part="mass")
    S.cyl("joint-slew", "slew-ring", (0.02, 0, 0), (0.10, 0, 0), 0.66, material=TS, bore=0.54, tone=0.3, to="structure", member=UP, part="joint")


def house(S):
    hp, dp, tp = L.poly(F.HOUSE_PLAN), L.poly(F.DECK_PLAN), L.poly(F.TOOLBOX_PLAN)
    H = F.HALF
    S.ext("deck", UP, dp.pts(), "v", 0, F.DECK_TOP, material=PS, tone=0.5, region="house")
    S.box("house", UP, (-1.38, (F.DECK_TOP + F.HOOD) / 2, 0), (2.0, F.HOOD - F.DECK_TOP, 2 * H), material=PS, tone=0.62, corner=0.04, region="house")
    S.box("house-tank", UP, (0.15, (F.DECK_TOP + F.HOOD) / 2, (F.SLOT[0] - H) / 2), (1.20, F.HOOD - F.DECK_TOP, H + F.SLOT[0]), material=PS, tone=0.62, corner=0.04, region="house")
    S.box("toolbox", UP, (1.085, (F.DECK_TOP + 0.92) / 2, (F.SLOT[0] - H) / 2), (0.67, 0.92 - F.DECK_TOP, H + F.SLOT[0]), material=PS, tone=0.58, corner=0.03, region="house")
    S.draw("deck", UP, L.rect(-2.38, 0, 1.44, F.DECK_TOP), "w", -H, H, tone=0.5, region="house", material=PS, views="ef")
    S.draw("deck", UP, dp, "v", 0, F.DECK_TOP, tone=0.5, region="house", material=PS, views="p")
    S.draw("house", UP, L.rect(-2.38, F.DECK_TOP, 0.75, F.HOOD), "w", -H, F.SLOT[0], tone=0.62, region="house", material=PS, views="ef")
    S.draw("house-rear", UP, L.rect(-2.38, F.DECK_TOP, -0.38, F.HOOD), "w", F.SLOT[0], H, tone=0.62, region="house", material=PS, views="f")
    S.draw("house", UP, hp, "v", F.DECK_TOP, F.HOOD, tone=0.62, region="house", material=PS, views="p")
    S.draw("toolbox", UP, L.rect(0.75, F.DECK_TOP, 1.42, 0.92), "w", -H, F.SLOT[0], tone=0.58, region="house", material=PS, views="ef")
    S.draw("toolbox", UP, tp, "v", F.DECK_TOP, 0.92, tone=0.58, region="house", material=PS, views="p")
    hood = L.rect(-2.28, -0.80, -1.30, 0.90, 0.06)
    S.box("hood-cover", UP, (-1.79, F.HOOD + 0.02, 0.05), (0.98, 0.06, 1.70), material=PS, role="detail", tone=0.5, corner=0.02, region="house")
    S.draw("hood-grille-top", UP, L.rect(-2.28, F.HOOD, -1.30, F.HOOD + 0.05), "w", -0.80, 0.90, role="detail", tone=0.45, material=PS, region="house", views="ef")
    S.draw("hood-grille-top", UP, hood, "v", F.HOOD, F.HOOD + 0.05, role="detail", tone=0.45, material=PS, region="house", views="p")
    S.box("vent-panel", UP, (-1.825, 0.775, -H - 0.006), (0.75, 0.55, 0.012), material=EM, role="detail", tone=0.15, region="engine-compartment", thfaces=["+w"])
    S.box("vent-slat", UP, (-1.825, 0.55, -H - 0.016), (0.69, 0.022, 0.008), material=EM, role="texture", tone=0.3, texture=True, repeat={"count": 6, "translation": [0, 0.09, 0]})
    S.draw("hood-grille", UP, L.rect(-2.20, 0.50, -1.45, 1.05), "w", -H - 0.012, -H, role="detail", tone=0.15, material=EM, region="engine-compartment")
    S.draw("radiator-grille", UP, L.rect(-2.25, 0.45, -1.30, 1.12), "w", H, H + 0.012, role="detail", tone=0.15, material=EM, region="engine-compartment", views="fp")
    S.draw("hood-louvres", UP, L.bands([((-2.17, 0.55 + 0.09 * k), (-1.48, 0.55 + 0.09 * k)) for k in range(6)], 0.022), "w", -H - 0.02, -H - 0.012, role="texture", tone=0.3, views="e", parent="hood-grille")
    S.box("radiator-grille", UP, (-1.775, 0.785, H + 0.006), (0.95, 0.67, 0.012), material=EM, role="detail", tone=0.15, region="engine-compartment", thfaces=["-w"])
    S.box("radiator-slat", UP, (-1.775, 0.52, H + 0.016), (0.89, 0.022, 0.008), material=EM, role="texture", tone=0.3, texture=True, repeat={"count": 7, "translation": [0, 0.09, 0]})
    for k, u in enumerate((-1.40, -0.40)):
        S.draw("door-seam-%d" % (k + 1), UP, L.band([(u, 0.34), (u, 1.18)], 0.004), "w", -H - 0.002, -H, role="detail", tone=0.3, views="e", parent="house")
    S.draw("skirt-line", UP, L.band([(-2.38, F.DECK_TOP), (1.44, F.DECK_TOP)], 0.004), "w", -H - 0.002, -H, role="detail", tone=0.4, views="e", parent="house")
    sx, sz = -1.80, -1.02
    S.cyl("stack", UP, (sx, F.HOOD, sz), (sx, 1.74, sz), 0.06, material=EX, bore=0.054, tone=0.25)
    S.cyl("stack-base", UP, (sx, F.HOOD, sz), (sx, F.HOOD + 0.07, sz), 0.085, material=EX, tone=0.2)
    S.cyl("stack-surface", UP, (sx, F.HOOD + 0.07, sz), (sx, 1.74, sz), 0.06, material=EX, tone=0.25, to="thermal", region="exhaust-stack")
    S.cyl("stack-base-surface", UP, (sx, F.HOOD, sz), (sx, F.HOOD + 0.07, sz), 0.085, material=EX, tone=0.2, to="thermal", region="exhaust-stack", thfaces=["-v"])
    S.draw("stack", UP, L.rect(sx - 0.06, F.HOOD, sx + 0.06, 1.74), "w", sz - 0.06, sz + 0.06, role="detail", tone=0.25, material=EX, region="exhaust-stack", views="ef")
    S.draw("stack", UP, {"kind": "ellipse", "cx": sx, "cy": sz, "rx": 0.06, "ry": 0.06}, "v", F.HOOD, 1.74, role="detail", tone=0.25, material=EX, region="exhaust-stack", views="p")
    S.draw("stack-cap", UP, L.rect(sx - 0.085, F.HOOD, sx + 0.085, F.HOOD + 0.07), "w", sz - 0.085, sz + 0.085, role="detail", tone=0.2, material=EX, views="ef", parent="stack")
    S.rev("exhaust-outlet", UP, (sx, 1.732, sz), (0, 1, 0), [(0, 0.054), (0.008, 0.054)], material=EX, tone=0.1, to="thermal", region="exhaust-stack", tags=["exhaust-stack", "emitter"])
    rail = [(-0.30, F.HOOD), (-0.30, 1.62), (0.70, 1.62), (0.70, F.HOOD)]
    S.tube("hood-rail", UP, [(u, v, -1.30) for u, v in rail], 0.016, material=PS, role="detail", tone=0.3, region="house")
    S.draw("hood-rail", UP, L.band(rail, 0.032), "w", -1.316, -1.284, role="detail", tone=0.3, material=PS)
    S.box("mirror-right", UP, (0.95, 1.50, -1.40), (0.02, 0.22, 0.13), material=PS, role="detail", tone=0.2, corner=0.008)
    S.tube("mirror-arm-right", UP, [(0.70, 1.60, -1.30), (0.86, 1.60, -1.36), (0.94, 1.56, -1.40)], 0.012, material=PS, role="detail", tone=0.3)
    S.draw("mirror-right", UP, L.rect(0.94, 1.39, 0.96, 1.61), "w", -1.465, -1.335, role="detail", tone=0.2, material=PS)
    S.draw("mirror-arm-right", UP, L.band([(0.70, 1.60), (0.86, 1.60), (0.94, 1.56)], 0.024), "w", -1.40, -1.30, role="detail", tone=0.3, material=PS, views="e")
    S.draw("mirror-arm-right", UP, L.band([(0.70, -1.30), (0.86, -1.36), (0.94, -1.40)], 0.024), "v", 1.55, 1.61, role="detail", tone=0.3, material=PS, views="fp")
    S.box("work-light", UP, (1.36, 0.975, -0.98), (0.09, 0.11, 0.16), material=PS, role="detail", tone=0.25, corner=0.015)
    S.draw("work-light", UP, L.rect(1.315, 0.92, 1.405, 1.03), "w", -1.06, -0.90, role="detail", tone=0.25, material=PS)
    foot = L.Path().M(-0.30, F.DECK_TOP).L(0.55, F.DECK_TOP).L(0.285, 0.897).A(0.14, 0.015, 0.897).Z()
    for name, z in (("right", F.ZC - 0.34), ("left", F.ZC + 0.27)):
        S.ext("boom-foot-" + name, UP, foot.pts(), "w", z, z + 0.07, material=PS, tone=0.4, region="house", member=UP)
    S.draw("boom-foot", UP, foot, "w", F.ZC - 0.34, F.ZC + 0.34, tone=0.4, region="house", material=PS)
    A, C = K.PIN["A"], K.PIN["C"]
    S.cyl("boom-pin", UP, (A[0], A[1], F.ZC - 0.37), (A[0], A[1], F.ZC + 0.37), 0.05, material=TS, tone=0.12, member="boom", part="joint")
    S.draw("boom-pin", UP, {"kind": "ellipse", "cx": A[0], "cy": A[1], "rx": 0.05, "ry": 0.05}, "w", F.ZC - 0.37, F.ZC + 0.37, role="detail", tone=0.12, material=TS, parent="boom-foot")
    lug = L.Path().M(0.42, 0.02).L(0.66, 0.02).L(0.60, 0.19).A(0.07, 0.50, 0.19).Z()
    for k, zc in enumerate(F.CYL_Z):
        for name, dz in (("a", -0.09), ("b", 0.06)):
            S.ext("boom-cyl-lug-%d%s" % (k + 1, name), UP, lug.pts(), "w", zc + dz, zc + dz + 0.03, material=PS, role="detail", tone=0.4, region="house", member=UP)
        S.cyl("boom-cyl-pin-%d" % (k + 1), UP, (C[0], C[1], zc - 0.10), (C[0], C[1], zc + 0.10), 0.04, material=TS, tone=0.12, member="boom-cyl", part="joint")
    S.draw("boom-cyl-foot", UP, {"kind": "ellipse", "cx": C[0], "cy": C[1], "rx": 0.07, "ry": 0.07}, "w", F.CYL_Z[1] - 0.10, F.CYL_Z[0] + 0.10, role="detail", tone=0.12, material=TS)


def cab(S):
    c = F.CAB
    side = L.poly(F.CAB_SIDE)
    z0, z1 = c["right"], c["left"]
    plan = L.rect(c["rear"], z0, c["front"], z1, 0.07)
    shell = dict(material=PS, tone=0.62, region="house")
    S.ext("cab-floor", UP, plan.pts(), "v", c["floor"], c["floor"] + 0.06, thfaces=["+v"], **shell)
    S.ext("cab-roof", UP, L.above(side.pts(), 1.84), "w", z0, z1, thfaces=["-v"], **shell)
    for name, a, b in (("right", z0, z0 + 0.06), ("left", z1 - 0.06, z1)):
        S.box("cab-cant-rail-" + name, UP, (0.47, 1.81, (a + b) / 2), (1.64, 0.08, 0.06), **shell)
    front = [(1.44, 0.36), (1.44, 1.02), (1.313, 1.84)]
    post = L.strip(front, 0.07)
    for name, a, b in (("right", z0, z0 + 0.06), ("left", z1 - 0.06, z1)):
        S.ext("cab-a-pillar-" + name, UP, post, "w", a, b, **shell)
        S.box("cab-c-pillar-" + name, UP, (-0.345, 1.10, (a + b) / 2), (0.07, 1.48, 0.06), **shell)
    S.box("cab-b-pillar", UP, (0.50, 1.10, z1 - 0.03), (0.07, 1.48, 0.06), **shell)
    S.box("cab-panel-right", UP, (0.53, 0.63, z0 + 0.02), (1.76, 0.54, 0.04), thfaces=["+w"], **shell)
    S.box("cab-panel-left", UP, (0.53, 0.52, z1 - 0.02), (1.76, 0.32, 0.04), thfaces=["-w"], **shell)
    S.box("cab-panel-rear", UP, (-0.36, 0.71, (z0 + z1) / 2), (0.04, 0.70, z1 - z0 - 0.12), thfaces=["+u"], **shell)
    S.box("cab-panel-front", UP, (1.42, 0.41, (z0 + z1) / 2), (0.04, 0.10, z1 - z0 - 0.12), thfaces=["-u"], **shell)
    S.box("cab-sill", UP, (1.405, 1.02, (z0 + z1) / 2), (0.05, 0.05, z1 - z0 - 0.12), **shell)
    glass = dict(material=GL, role="glazing", tone=0.85, region="cab-glazing")
    t = 0.008
    lower = L.strip([(1.44, 0.46), (1.44, 0.995)], t)
    upper = L.strip([(1.436, 1.045), (1.313, 1.84)], t)
    S.ext("cab-glass-front-lower", UP, lower, "w", z0 + 0.06, z1 - 0.06, thfaces=["-u"], **glass)
    S.ext("cab-glass-front", UP, upper, "w", z0 + 0.06, z1 - 0.06, thfaces=["-u"], **glass)
    right = L.poly([(-0.31, 0.90), (1.40, 0.90), (1.40, 1.02), (1.283, 1.78, 0.05), (-0.31, 1.78, 0.05)])
    S.ext("cab-glass-side", UP, right.pts(), "w", z0, z0 + t, thfaces=["+w"], **glass)
    door = L.poly([(0.535, 0.68), (1.40, 0.68), (1.40, 1.02), (1.283, 1.78, 0.05), (0.535, 1.78)])
    quarter = L.poly([(-0.31, 0.68), (0.465, 0.68), (0.465, 1.78), (-0.31, 1.78, 0.05)])
    S.ext("cab-glass-door", UP, door.pts(), "w", z1 - t, z1, thfaces=["-w"], **glass)
    S.ext("cab-glass-quarter", UP, quarter.pts(), "w", z1 - t, z1, thfaces=["-w"], **glass)
    S.box("cab-glass-rear", UP, (-0.38 + t / 2, 1.45, (z0 + z1) / 2), (t, 0.78, z1 - z0 - 0.12), thfaces=["+u"], **glass)
    S.box("cab-glass-roof", UP, (0.62, 1.9035, (z0 + z1) / 2), (0.60, t, 0.50), rz=-0.68, thfaces=["-v"], **glass)
    inside = dict(material=PS, role="detail", tone=0.25)
    zs = (z0 + z1) / 2 + 0.02
    S.box("seat-cushion", UP, (0.36, 0.78, zs), (0.48, 0.12, 0.48), corner=0.04, **inside)
    S.box("seat-back", UP, (0.11, 1.14, zs), (0.12, 0.62, 0.46), corner=0.04, rz=8, **inside)
    S.box("seat-headrest", UP, (0.05, 1.53, zs), (0.09, 0.18, 0.24), corner=0.03, rz=8, **inside)
    S.box("seat-base", UP, (0.36, 0.54, zs), (0.40, 0.36, 0.40), **inside)
    for name, dz in (("right", -0.31), ("left", 0.31)):
        S.box("console-" + name, UP, (0.48, 0.80, zs + dz), (0.62, 0.20, 0.13), corner=0.03, **inside)
        S.cap("joystick-" + name, UP, (0.74, 0.90, zs + dz), (0.76, 1.06, zs + dz), 0.02, **inside)
        S.cap("travel-lever-" + name, UP, (1.12, 0.36, zs + dz * 0.3), (1.08, 0.92, zs + dz * 0.3), 0.012, **inside)
    S.box("monitor", UP, (1.25, 1.15, z0 + 0.16), (0.04, 0.20, 0.16), rz=-20, **inside)
    S.draw("cab", UP, side, "w", z0, z1, tone=0.62, region="house", material=PS, views="ef", ids={"f": "cab-block"})
    S.draw("cab-roof", UP, plan, "v", c["floor"], c["top"], tone=0.6, region="house", material=PS, views="p")
    seat = L.poly([(0.12, 0.72, 0.04), (0.60, 0.72, 0.04), (0.60, 0.84, 0.04), (0.22, 0.86), (0.13, 1.44, 0.04), (0.02, 1.43, 0.04), (0.10, 0.86)])
    S.draw("cab-seat", UP, seat, "w", zs - 0.24, zs + 0.24, role="detail", tone=0.25, material=PS, views="e", parent="cab", sub=1)
    S.draw("cab-glass-side", UP, right, "w", z0, z0 + t, role="glazing", tone=0.85, region="cab-glazing", material=GL, views="e", parent="cab", sub=2)
    S.draw("cab-glass", UP, L.poly(L.strip([(1.44, 0.46), (1.44, 1.02), (1.313, 1.84)], 0.02)), "w", z0 + 0.06, z1 - 0.06, role="glazing", tone=0.85, region="cab-glazing", material=GL, views="f", parent="cab-block", sub=2)
    S.draw("cab-pillar", UP, L.rect(1.38, 0.995, 1.43, 1.045), "w", z0 + 0.06, z1 - 0.06, role="detail", tone=0.3, views="f", parent="cab-block", sub=3)
    S.draw("cab-glass", UP, L.rect(0.32, zs - 0.27, 0.92, zs + 0.23, 0.04), "v", c["top"] - t, c["top"], role="glazing", tone=0.85, region="cab-glazing", material=GL, views="p", parent="cab-roof", sub=2)
    S.box("mirror-left", UP, (1.30, 1.45, 1.43), (0.02, 0.24, 0.10), material=PS, role="detail", tone=0.2, corner=0.008)
    S.draw("mirror-left", UP, L.rect(1.29, 1.33, 1.31, 1.57), "w", 1.38, 1.48, role="detail", tone=0.2, material=PS, views="fp")
    for name, zc in (("right", z0 + 0.03), ("left", z1 - 0.03)):
        S.ext("rops-post-front-" + name, UP, L.strip(front, 0.06), "w", zc - 0.025, zc + 0.025, material=PS, tone=0.5, to="structure", member=UP)
        S.box("rops-post-rear-" + name, UP, (-0.335, 1.08, zc), (0.06, 1.50, 0.05), material=PS, tone=0.5, to="structure", member=UP)
        S.box("rops-rail-" + name, UP, (0.45, 1.845, zc), (1.62, 0.05, 0.05), material=PS, tone=0.5, to="structure", member=UP)


def counterweight(S):
    plan = F.cw_plan()
    ring = plan.pts()
    S.ext("counterweight-body", CW, ring, "v", 0.08, F.CW_TOP - 0.08, material=CI, tone=0.35, region="counterweight")
    for k, z in enumerate((-0.60, 0.60)):
        S.box("counterweight-carrier-%d" % (k + 1), CW, (0.345, 0.22, z), (0.45, 0.28, 0.10), material=PS, tone=0.45, to="structure", member=CW)
    S.ext("counterweight-top", CW, ring, "v", F.CW_TOP - 0.08, F.CW_TOP, taper=(1, 0.94), material=CI, tone=0.35, region="counterweight")
    S.ext("counterweight-bottom", CW, ring, "v", 0, 0.08, taper=(0.94, 1), material=CI, tone=0.35, region="counterweight")
    S.cyl("counterweight-eye", CW, (0.34, F.CW_TOP, -0.70), (0.34, F.CW_TOP + 0.03, -0.70), 0.05, material=CI, tone=0.2, repeat={"count": 2, "translation": [0, 0, 1.40]})
    area = sum(ring[i - 1][0] * ring[i][1] - ring[i][0] * ring[i - 1][1] for i in range(len(ring))) / 2
    cu = sum((ring[i - 1][0] + ring[i][0]) * (ring[i - 1][0] * ring[i][1] - ring[i][0] * ring[i - 1][1]) for i in range(len(ring))) / (6 * area)
    ur, uf, ct = cu * 0.06, cu + (F.CW_FRONT - cu) * 0.94, F.CW_TOP
    side = L.poly([(0, 0.08), (ur, 0), (uf, 0), (F.CW_FRONT, 0.08), (F.CW_FRONT, ct - 0.08), (uf, ct), (ur, ct), (0, ct - 0.08)])
    S.draw("counterweight-eye", CW, L.rect(0.29, ct, 0.39, ct + 0.03), "w", -0.75, 0.75, role="detail", tone=0.2, material=CI, views="e", parent="counterweight-body")
    S.draw("counterweight-body", CW, side, "w", -F.HALF, F.HALF, tone=0.35, region="counterweight", material=CI, views="ef")
    S.draw("counterweight-body", CW, plan, "v", 0, F.CW_TOP, tone=0.35, region="counterweight", material=CI, views="p")
    S.draw("counterweight-seam", CW, L.band([(0.33, 0.58), (F.CW_FRONT, 0.58)], 0.008), "w", -F.HALF - 0.002, -F.HALF, role="texture", tone=0.2, views="e", parent="counterweight-body")
    S.ell("mass-counterweight", CW, (0.32, 0.58, 0), (0.16, 0.30, 0.55), material=CI, tone=0.2, to="structure", member=CW, part="mass")


def engine(S):
    S.box("engine-block", EN, (0.36, 0.33, -0.15), (0.72, 0.80, 1.05), material=EM, tone=0.2, corner=0.03, to="thermal", region="engine-compartment")
    S.box("engine-cooler", EN, (0.36, 0.36, 0.95), (0.80, 0.90, 0.14), material=EM, tone=0.25, to="thermal", region="engine-compartment")
    for k, z in enumerate((-0.45, 0.15)):
        S.box("engine-bed-%d" % (k + 1), EN, (0.36, -0.07, z), (0.90, 0.10, 0.08), material=PS, tone=0.4, to="structure", member=EN)
    S.ell("mass-engine", EN, (0.36, 0.33, -0.15), (0.22, 0.25, 0.32), material=EM, tone=0.2, to="structure", member=EN, part="mass")


def frame(S):
    """The revolving frame under the cladding: two centre rails, the cross members, the side beams and the counterweight carrier."""
    m = dict(material=PS, tone=0.45, to="structure", member=UP)
    for k, z in enumerate((F.SLOT[0] + 0.04, F.SLOT[1] - 0.04)):
        S.box("frame-rail-%d" % (k + 1), UP, (-0.80, 0.14, z), (3.16, 0.28, 0.08), **m)
    for k, x in enumerate((-2.30, -1.20, -0.40, 0.38)):
        S.box("frame-cross-%d" % (k + 1), UP, (x, 0.12, 0), (0.10, 0.20, 2 * F.HALF - 0.08), **m)
    for k, z in enumerate((-F.HALF + 0.05, F.HALF - 0.05)):
        S.box("frame-side-%d" % (k + 1), UP, (-0.47, 0.10, z), (3.82, 0.16, 0.08), **m)
    S.cyl("frame-ring", UP, (0, 0, 0), (0, 0.06, 0), 0.70, material=PS, bore=0.56, tone=0.4, to="structure", member=UP)
    S.ell("mass-upper", UP, (-0.35, 0.62, 0), (0.42, 0.25, 0.50), material=PS, tone=0.2, to="structure", member=UP, part="mass")


def cylinder(S, name, bone, rod, zs, spec, rodx, tip, *, foot_pin=True):
    """One hydraulic cylinder (or the boom pair): barrel tube with cap eye and gland on the barrel bone, chrome rod with its eye on the rod bone."""
    lb = spec["closed"] - 0.23
    rb, rr = spec["barrel"] / 2, spec["rod"] / 2
    rep = {"count": len(zs), "translation": [0, 0, L.r4(zs[1] - zs[0])]} if len(zs) > 1 else None
    z = zs[0]
    hyd = dict(material=PS, region="hydraulics", repeat=rep)
    S.cyl(name + "-barrel", bone, (0.06, 0, z), (lb - 0.07, 0, z), rb, tone=0.35, **hyd)
    S.cyl(name + "-gland", bone, (lb - 0.07, 0, z), (lb, 0, z), rb + 0.012, tone=0.3, **hyd)
    S.cyl(name + "-eye", bone, (0, 0, z - 0.05), (0, 0, z + 0.05), 0.075, tone=0.3, **hyd)
    S.cyl(name + "-rod", rod, (0, 0, z), (tip - 0.05, 0, z), rr, material=CH, tone=0.85, region="hydraulics", repeat=rep)
    S.cyl(name + "-rod-eye", rod, (tip, 0, z - 0.05), (tip, 0, z + 0.05), 0.07, tone=0.3, **hyd)
    for k, zz in enumerate(zs):
        tag = "" if len(zs) == 1 else ("-left" if zz > F.ZC else "-right")
        S.cyl("tube-%s%s" % (name, tag), bone, (0.06, 0, zz), (lb, 0, zz), rb, material=PS, bore=spec["bore"] / 2, tone=0.35, to="structure", member=bone)
        S.cyl("shaft-%s%s" % (name, tag), rod, (0, 0, zz), (tip, 0, zz), rr, material=CH, tone=0.8, to="structure", member=rod)
        views = "efp" if zz == min(zs) else "fp"
        ids = {"e": name + "-rod", "f": name + "-rod" + tag, "p": name + "-rod" + tag}
        S.bar(name + "-rod" + tag, rod, (0, 0, zz), (tip, 0, zz), spec["rod"], tone=0.85, material=CH, region="hydraulics", views=views, ids=ids)
        ids = {"e": name + "-barrel", "f": name + "-barrel" + tag, "p": name + "-barrel" + tag}
        S.bar(name + "-barrel" + tag, bone, (0, 0, zz), (lb - 0.07, 0, zz), spec["barrel"], tone=0.35, material=PS, region="hydraulics", views=views, ids=ids, parent=name + "-rod" + tag, sub=1)
        for part, pb, shape, tone in (("gland", bone, L.rect(lb - 0.07, -rb - 0.012, lb, rb + 0.012), 0.3), ("eye", bone, L.circle(0, 0, 0.075), 0.3), ("rod-eye", rod, L.circle(tip, 0, 0.07), 0.3)):
            w = rb + 0.012 if part == "gland" else 0.05
            S.draw("%s-%s%s" % (name, part, tag), pb, shape, "w", zz - w, zz + w, role="detail", tone=tone, material=PS, views=views, ids={"e": "%s-%s" % (name, part), "f": "%s-%s%s" % (name, part, tag), "p": "%s-%s%s" % (name, part, tag)}, parent=name + "-rod" + tag, sub=2)


def arm(S, R):
    """Boom, stick, bucket, links and cylinders; R holds the rig numbers the build computed (rod bone positions, lengths)."""
    bw, sw, kw = F.BOOM_W, F.STICK_W, F.BUCKET_W
    att = dict(material=PS, region="attachment")
    pin = dict(material=TS, tone=0.12)
    boom = L.poly(F.BOOM)
    D, E = K.PIN["D"], K.PIN["E"]
    S.ext("boom-body", "boom", L.poly(F.BOOM_BOX).pts(), "w", bw[0], bw[1], tone=0.58, **att)
    for name, a, b in (("right", bw[0], bw[0] + 0.07), ("left", bw[1] - 0.07, bw[1])):
        S.ext("boom-cheek-" + name, "boom", F.BOOM_CHEEK, "w", a, b, tone=0.58, **att)
        S.cyl("boom-tip-boss-" + name, "boom", (5.7, 0, a), (5.7, 0, b), 0.20, tone=0.5, **att)
    S.cyl("boom-root-boss", "boom", (0, 0, bw[0] - 0.01), (0, 0, bw[1] + 0.01), 0.20, tone=0.5, **att)
    S.cyl("boom-rod-boss", "boom", (D[0], D[1], F.CYL_Z[1] - 0.10), (D[0], D[1], F.CYL_Z[0] + 0.10), 0.07, tone=0.4, **att)
    S.cyl("boom-rod-pin", "boom", (D[0], D[1], F.CYL_Z[1] - 0.12), (D[0], D[1], F.CYL_Z[0] + 0.12), 0.04, member="boom-rod", part="joint", **pin)
    elug = L.Path().M(2.74, 0.80).L(3.26, 0.72).L(3.07, 0.92).A(0.07, 2.93, 0.92).Z()
    for name, z in (("right", F.ZC - 0.10), ("left", F.ZC + 0.07)):
        S.ext("stick-cyl-lug-" + name, "boom", elug.pts(), "w", z, z + 0.03, role="detail", tone=0.45, **att)
    S.cyl("stick-cyl-pin", "boom", (E[0], E[1], F.ZC - 0.12), (E[0], E[1], F.ZC + 0.12), 0.04, member="stick-cyl", part="joint", **pin)
    S.cyl("stick-pin", "boom", (5.7, 0, bw[0] - 0.02), (5.7, 0, bw[1] + 0.02), 0.05, member="stick", part="joint", **pin)
    top = [(u, v - 0.01) for u, v in L.thin(L.chain(boom.pts(), 0.55, 4.9), 0.003)]
    for name, z in (("right", bw[0] + 0.07), ("left", bw[1] - 0.07)):
        S.tube("boom-pipe-" + name, "boom", [(u, v + 0.03, z) for u, v in top], 0.017, material=PS, role="detail", tone=0.25, region="attachment")
        S.draw("boom-pipe-" + name, "boom", L.band([(u, v + 0.03) for u, v in top], 0.034), "w", z - 0.017, z + 0.017, role="detail", tone=0.25, material=PS, views="efp" if name == "right" else "fp", parent="boom-body", sub=3)
    S.box("boom-light", "boom", (1.95, 0.50, bw[0] - 0.05), (0.14, 0.10, 0.10), material=PS, role="detail", tone=0.25, corner=0.015)
    S.draw("boom-body", "boom", boom, "w", bw[0], bw[1], tone=0.58, views="e", **att)
    S.draw("boom-body", "boom", L.poly(F.BOOM_BOX), "w", bw[0], bw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", bw[0], bw[0] + 0.07), ("left", bw[1] - 0.07, bw[1])):
        S.draw("boom-cheek-" + name, "boom", L.poly(F.BOOM_CHEEK), "w", a, b, tone=0.58, views="fp", **att)
    S.draw("boom-root-boss", "boom", L.circle(0, 0, 0.20), "w", bw[0] - 0.01, bw[1] + 0.01, role="detail", tone=0.5, material=PS, parent="boom-body", sub=1)
    S.draw("boom-tip-boss", "boom", L.circle(5.7, 0, 0.20), "w", bw[0], bw[1], role="detail", tone=0.5, material=PS, parent="boom-body", sub=1)
    S.draw("boom-rod-eye", "boom", L.circle(D[0], D[1], 0.07), "w", F.CYL_Z[1] - 0.10, F.CYL_Z[0] + 0.10, role="detail", tone=0.12, material=TS, parent="boom-body", sub=2)
    S.draw("stick-cyl-foot", "boom", elug, "w", F.ZC - 0.10, F.ZC + 0.10, role="detail", tone=0.45, material=PS, parent="boom-body", sub=1)
    for k, (u, a, b) in enumerate(((1.25, -0.03, 0.50), (2.60, 0.20, 0.86), (4.20, 0.02, 0.53))):
        S.draw("boom-weld-%d" % (k + 1), "boom", L.band([(u, a), (u, b)], 0.004), "w", bw[0] - 0.002, bw[0], role="detail", tone=0.3, views="e", parent="boom-body", sub=3)
    S.draw("boom-light", "boom", L.rect(1.88, 0.45, 2.02, 0.55), "w", bw[0] - 0.10, bw[0], role="detail", tone=0.25, material=PS, parent="boom-body", sub=4)
    stick = L.poly(F.STICK)
    Fp, Hp, Ip = K.PIN["F"], K.PIN["H"], K.PIN["I"]
    S.ext("stick-body", "stick", L.poly(F.STICK_BOX).pts(), "w", sw[0], sw[1], tone=0.58, **att)
    for name, a, b in (("right", sw[0], sw[0] + 0.05), ("left", sw[1] - 0.05, sw[1])):
        S.ext("stick-tail-" + name, "stick", L.poly(F.STICK_TAIL).pts(), "w", a, b, tone=0.58, **att)
        S.cyl("stick-rod-boss-" + name, "stick", (Fp[0], Fp[1], a - 0.01), (Fp[0], Fp[1], b + 0.01), 0.11, tone=0.5, **att)
    S.cyl("stick-root-boss", "stick", (0, 0, bw[0] + 0.07), (0, 0, bw[1] - 0.07), 0.19, tone=0.5, **att)
    S.cyl("stick-tip-boss", "stick", (2.9, 0, sw[0]), (2.9, 0, sw[1]), 0.13, tone=0.5, **att)
    S.cyl("stick-rod-pin", "stick", (Fp[0], Fp[1], sw[0] - 0.03), (Fp[0], Fp[1], sw[1] + 0.03), 0.045, member="stick-rod", part="joint", **pin)
    hlug = L.Path().M(0.02, 0.46).L(0.50, 0.40).L(0.32, 0.56).A(0.07, 0.18, 0.56).Z()
    for name, z in (("right", F.ZC - 0.10), ("left", F.ZC + 0.07)):
        S.ext("bucket-cyl-lug-" + name, "stick", hlug.pts(), "w", z, z + 0.03, role="detail", tone=0.45, **att)
    S.cyl("bucket-cyl-pin", "stick", (Hp[0], Hp[1], F.ZC - 0.12), (Hp[0], Hp[1], F.ZC + 0.12), 0.04, member="bucket-cyl", part="joint", **pin)
    S.cyl("idler-pin", "stick", (Ip[0], Ip[1], F.ZC - 0.23), (Ip[0], Ip[1], F.ZC + 0.23), 0.04, member="idler-link", part="joint", **pin)
    S.cyl("bucket-pin", "stick", (2.9, 0, F.ZC - 0.25), (2.9, 0, F.ZC + 0.25), 0.045, member="bucket", part="joint", **pin)
    S.draw("stick-body", "stick", stick, "w", sw[0], sw[1], tone=0.58, views="e", **att)
    S.draw("stick-body", "stick", L.poly(F.STICK_BOX), "w", sw[0], sw[1], tone=0.58, views="fp", **att)
    for name, a, b in (("right", sw[0], sw[0] + 0.05), ("left", sw[1] - 0.05, sw[1])):
        S.draw("stick-tail-" + name, "stick", L.poly(F.STICK_TAIL), "w", a, b, tone=0.58, views="fp", **att)
    S.draw("stick-root-boss", "stick", L.circle(0, 0, 0.19), "w", sw[0], sw[1], role="detail", tone=0.5, material=PS, parent="stick-body", sub=1)
    S.draw("stick-tip-boss", "stick", L.circle(2.9, 0, 0.13), "w", sw[0], sw[1], role="detail", tone=0.5, material=PS, parent="stick-body", sub=1)
    S.draw("stick-rod-eye", "stick", L.circle(Fp[0], Fp[1], 0.11), "w", sw[0] - 0.01, sw[1] + 0.01, role="detail", tone=0.5, material=PS, parent="stick-body", sub=1)
    S.draw("bucket-cyl-foot", "stick", hlug, "w", F.ZC - 0.10, F.ZC + 0.10, role="detail", tone=0.45, material=PS, parent="stick-body", sub=1)
    S.draw("idler-pin", "stick", L.circle(Ip[0], Ip[1], 0.04), "w", F.ZC - 0.23, F.ZC + 0.23, role="detail", tone=0.12, material=TS, views="e", parent="stick-body", sub=2)
    S.draw("stick-seam", "stick", L.band([(1.45, -0.17), (1.45, 0.30)], 0.004), "w", sw[0] - 0.002, sw[0], role="detail", tone=0.3, views="e", parent="stick-body", sub=3)
    wear = dict(material=WS, region="attachment")
    shell = L.poly(F.BUCKET_SHELL)
    wall = L.strip(shell.pts(), 0.016)
    S.ext("bucket-shell", "bucket", wall, "w", kw[0] + 0.016, kw[1] - 0.016, tone=0.4, member="bucket", **wear)
    side = L.poly(F.BUCKET)
    for name, a, b in (("right", kw[0], kw[0] + 0.016), ("left", kw[1] - 0.016, kw[1])):
        S.ext("bucket-cheek-" + name, "bucket", side.pts(), "w", a, b, tone=0.42, member="bucket", **wear)
    S.ext("bucket-lip", "bucket", F.LIP, "w", kw[0], kw[1], tone=0.3, **wear)
    S.ext("bucket-tooth", "bucket", F.TOOTH, "w", kw[0] + 0.03, kw[0] + 0.12, role="detail", tone=0.2, repeat={"count": 5, "translation": [0, 0, L.r4((kw[1] - kw[0] - 0.15) / 4)]}, **wear)
    S.box("bucket-beam", "bucket", (0.03, -0.04, F.ZC), (0.16, 0.12, kw[1] - kw[0] - 0.02), tone=0.4, corner=0.02, member="bucket", **wear)
    ear = L.poly([(-0.17, 0.62, 0.06), (-0.22, 0.20, 0.05), (-0.12, -0.10, 0.05), (0.12, -0.10, 0.05), (0.12, 0.30), (0.04, 0.62, 0.06)])
    for name, z in (("right", F.ZC - 0.215), ("left", F.ZC + 0.17)):
        S.ext("bucket-ear-" + name, "bucket", ear.pts(), "w", z, z + 0.045, tone=0.38, member="bucket", **wear)
    J = K.PIN["J"]
    S.cyl("bucket-link-pin", "bucket", (J[0], J[1], F.ZC - 0.25), (J[0], J[1], F.ZC + 0.25), 0.04, member="bucket-link", part="joint", **pin)
    for k, run in enumerate(([(1.08, 0.342), (0.96, 0.447)], [(0.72, 0.59), (0.56, 0.67)])):
        S.ext("bucket-wear-%d" % (k + 1), "bucket", L.strip(run, -0.012), "w", kw[0] + 0.25, kw[0] + 0.33, role="detail", tone=0.3, repeat={"count": 3, "translation": [0, 0, 0.31]}, **wear)
    S.draw("bucket-body", "bucket", side, "w", kw[0], kw[1], tone=0.4, region="attachment", material=WS)
    S.draw("bucket-teeth", "bucket", L.poly(F.TOOTH), "w", kw[0] + 0.03, kw[1] - 0.03, role="detail", tone=0.2, region="attachment", material=WS, views="e", parent="bucket-body", sub=1)
    step = (kw[1] - kw[0] - 0.15) / 4
    for k in range(5):
        S.draw("bucket-tooth-%d" % k, "bucket", L.poly(F.TOOTH), "w", kw[0] + 0.03 + k * step, kw[0] + 0.12 + k * step, role="detail", tone=0.2, region="attachment", material=WS, views="fp", parent="bucket-body", sub=1)
    S.draw("bucket-ear", "bucket", ear, "w", F.ZC - 0.215, F.ZC + 0.215, role="detail", tone=0.38, material=WS, parent="bucket-body", sub=2)
    for k, run in enumerate(([(1.08, 0.342), (0.96, 0.447)], [(0.72, 0.59), (0.56, 0.67)])):
        S.draw("bucket-wear-%d" % (k + 1), "bucket", L.Path_of(L.strip(run, -0.012)), "w", kw[0] + 0.25, kw[0] + 0.95, role="detail", tone=0.3, material=WS, views="e", parent="bucket-body", sub=1)
    S.draw("bucket-pin", "bucket", L.circle(0, 0, 0.045), "w", F.ZC - 0.25, F.ZC + 0.25, role="detail", tone=0.12, material=TS, views="e", parent="bucket-body", sub=3)
    S.draw("bucket-plate", "bucket", L.band([(1.18, 0.10), (0.20, 0.03)], 0.004), "w", kw[0] - 0.002, kw[0], role="detail", tone=0.3, views="e", parent="bucket-body", sub=3)
    il = stadium(K.IDLER, 0.055)
    for name, z in (("right", F.ZC - 0.21), ("left", F.ZC + 0.175)):
        S.ext("idler-link-" + name, "idler-link", il.pts(), "w", z, z + 0.035, role="detail", tone=0.45, member="idler-link", **att)
    S.cyl("link-pin", "idler-link", (K.IDLER, 0, F.ZC - 0.23), (K.IDLER, 0, F.ZC + 0.23), 0.04, member="bucket-rod", part="joint", **pin)
    S.draw("idler-link", "idler-link", il, "w", F.ZC - 0.21, F.ZC + 0.21, role="detail", tone=0.45, region="attachment", material=PS, views="e")
    for name, z in (("right", F.ZC - 0.21), ("left", F.ZC + 0.175)):
        S.draw("idler-link-" + name, "idler-link", il, "w", z, z + 0.035, role="detail", tone=0.45, region="attachment", material=PS, views="fp")
    bl = stadium(K.LINK, 0.06)
    for name, z in (("right", F.ZC - 0.12), ("left", F.ZC + 0.075)):
        S.ext("bucket-link-" + name, "bucket-link", bl.pts(), "w", z, z + 0.045, role="detail", tone=0.45, member="bucket-link", **att)
    S.box("bucket-link-web", "bucket-link", (0.42, 0, F.ZC), (0.26, 0.09, 0.15), role="detail", tone=0.45, **att)
    S.draw("bucket-link", "bucket-link", bl, "w", F.ZC - 0.12, F.ZC + 0.12, role="detail", tone=0.45, region="attachment", material=PS, views="e")
    for name, z in (("right", F.ZC - 0.12), ("left", F.ZC + 0.075)):
        S.draw("bucket-link-" + name, "bucket-link", bl, "w", z, z + 0.045, role="detail", tone=0.45, region="attachment", material=PS, views="fp")
    S.draw("bucket-link-web", "bucket-link", L.rect(0.29, -0.045, 0.55, 0.045), "w", F.ZC - 0.075, F.ZC + 0.075, role="detail", tone=0.45, region="attachment", material=PS, views="fp")
    cylinder(S, "boom-cyl", "boom-cyl", "boom-rod", F.CYL_Z, K.CYL["boom"], R["boom-rod"]["x"], R["boom-rod"]["lengthM"])
    cylinder(S, "stick-cyl", "stick-cyl", "stick-rod", (F.ZC,), K.CYL["stick"], R["stick-rod"]["x"], R["stick-rod"]["lengthM"])
    cylinder(S, "bucket-cyl", "bucket-cyl", "bucket-rod", (F.ZC,), K.CYL["bucket"], R["bucket-rod"]["x"], R["bucket-rod"]["lengthM"])
    box = dict(material=PS, tone=0.5, to="structure")
    bo, bi = F.band(L.poly(F.BOOM_BOX).pts(0.004), 0.0, 0.012)
    S.ext("boom-flanges", "boom", bo, "w", bw[0] + 0.012, bw[1] - 0.012, holes=[bi], member="boom", **box)
    for name, a in (("right", bw[0]), ("left", bw[1] - 0.012)):
        S.ext("boom-web-" + name, "boom", bo, "w", a, a + 0.012, member="boom", **box)
        S.ext("boom-fork-" + name, "boom", F.BOOM_CHEEK, "w", bw[0] if name == "right" else bw[1] - 0.07, bw[0] + 0.07 if name == "right" else bw[1], member="boom", **box)
    S.cyl("boom-foot-tube", "boom", (0, 0, bw[0]), (0, 0, bw[1]), 0.12, bore=0.05, member="boom", **box)
    so, si = F.band(L.poly(F.STICK_BOX).pts(0.004), 0.0, 0.012)
    S.ext("stick-flanges", "stick", so, "w", sw[0] + 0.012, sw[1] - 0.012, holes=[si], member="stick", **box)
    for name, a in (("right", sw[0]), ("left", sw[1] - 0.012)):
        S.ext("stick-web-" + name, "stick", so, "w", a, a + 0.012, member="stick", **box)
        S.ext("stick-fork-" + name, "stick", L.poly(F.STICK_TAIL).pts(), "w", sw[0] if name == "right" else sw[1] - 0.05, sw[0] + 0.05 if name == "right" else sw[1], member="stick", **box)


def build(S, R):
    tracks(S)
    house(S)
    cab(S)
    counterweight(S)
    engine(S)
    frame(S)
    arm(S, R)
    S.decide("Modelled as the real 22 t machine, part by part on the corrected rig. Undercarriage: per track a 14 mm shoe band on two link rails, 150 grouser ribs, toothed sprocket with final drive, flanged idler in its yoke, eight flanged track rollers on shafts and two carrier rollers under an inverted channel frame with a ridge roof, an X carbody with turret and slew bearing. Upper structure: U-shaped deck with the boom slot, L-shaped engine house with a chamfered roof edge, hood cover, vent and radiator grilles, tool box, tubular exhaust stack, bent handrail, mirrors, work light, a counterweight whose rear face is the 2.95 m tail swing arc, and a hollow cab of floor, roof, six pillars and sill panels closed by seven 8 mm panes with seat, consoles, joysticks, travel levers and monitor inside. Attachment: banana boom as a closed box with fork cheeks, foot boss, cylinder bosses and two steel lines, stick with forked tail, bucket of 16 mm shell and cheeks with lip, five teeth, cross beam, hinge ears and wear strips, twin idler links and the H link, two boom cylinders, one stick and one bucket cylinder, each a barrel with cap eye and gland plus chrome rod and rod eye, and every pin.", "Als reale 22-t-Maschine Teil für Teil auf dem berichtigten Skelett modelliert. Unterwagen: je Kette ein 14-mm-Bodenplattenband auf zwei Kettenschienen, 150 Stollenrippen, verzahntes Antriebsrad mit Fahrgetriebe, Leitrad mit Bund in seiner Gabel, acht Laufrollen mit Bund auf Achsen und zwei Stützrollen unter einem umgekehrten U-Rahmen mit Firstdach, X-Mittelrahmen mit Turm und Drehkranzlager. Oberwagen: U-förmige Plattform mit Auslegerschlitz, L-förmiges Maschinenhaus mit gefaster Dachkante, Haubendeckel, Lüftungs- und Kühlergitter, Werkzeugkasten, Auspuffrohr, gebogener Handlauf, Spiegel, Arbeitsscheinwerfer, ein Gegengewicht, dessen Rückseite der Heckschwenkradius von 2,95 m ist, und eine hohle Kabine aus Boden, Dach, sechs Säulen und Brüstungsblechen, geschlossen durch sieben 8-mm-Scheiben, mit Sitz, Konsolen, Joysticks, Fahrhebeln und Monitor. Ausrüstung: Knickausleger als geschlossener Kasten mit Gabelwangen, Fußnabe, Zylindernaben und zwei Stahlleitungen, Stiel mit Gabelende, Löffel aus 16-mm-Schale und Wangen mit Schneide, fünf Zähnen, Querträger, Lagerlaschen und Schleißleisten, zwei Umlenkhebel und die H-Lasche, zwei Auslegerzylinder, ein Stiel- und ein Löffelzylinder, jeder ein Rohr mit Bodenauge und Führungsbuchse sowie Chromstange mit Stangenauge, dazu jeder Bolzen.")
    S.decide("Thermal: every region references the body solids that are its surface; the faces of the cab shell and of the panes that look into the closed cabin and the backs of the two grille plates are marked as not outer (C31, C25 b). The engine compartment is the engine block and the cooler pack as real solids inside the house plus the two grille plates; the exhaust emitter is the outlet patch in the mouth of the stack.", "Thermik: Jede Region verweist auf die Körper, die ihre Oberfläche sind; die Flächen der Kabinenschale und der Scheiben, die in die geschlossene Kabine blicken, und die Rückseiten der beiden Gitterbleche sind als nicht außenliegend markiert (C31, C25 b). Der Motorraum besteht aus Motorblock und Kühlerpaket als echten Körpern im Maschinenhaus sowie den beiden Gitterblechen; der Abgasemitter ist die Austrittsfläche in der Mündung des Auspuffrohrs.")
    S.decide("Structure: the welded load path, not the cladding. Track frames, the hollow turret ring and four carbody legs, eight ground pads at the support anchors; the revolving frame of two centre rails, four cross members and two side beams with the slew ring flange, the boom foot plates and cylinder lugs, the ROPS frame of the cab, engine bed rails; boom and stick as box girders of two 12 mm webs and a flange band with their forks, the bucket shell, cheeks, beam and ears, both links, cylinder tubes with their real bore and solid rods, a pin at every joint, two carrier beams for the counterweight, and masses at the centres of undercarriage, upper structure, counterweight and engine.", "Tragwerk: der geschweißte Lastpfad, nicht die Verkleidung. Laufwerksrahmen, der hohle Turmring und vier Mittelrahmenarme, acht Standflächen an den Auflagerankern; der Oberwagenrahmen aus zwei Mittelholmen, vier Querträgern und zwei Seitenträgern mit dem Drehkranzflansch, die Auslegerfußbleche und Zylinderlaschen, der ROPS-Rahmen der Kabine, Motorlagerholme; Ausleger und Stiel als Kastenträger aus zwei 12-mm-Stegen und einem Gurtband mit ihren Gabeln, Löffelschale, Wangen, Querträger und Laschen, beide Hebel, Zylinderrohre mit ihrer wirklichen Bohrung und massive Kolbenstangen, ein Bolzen an jedem Gelenk zwei Trägerarme für das Gegengewicht und Massen in den Schwerpunkten von Unterwagen, Oberwagen, Gegengewicht und Motor.")
