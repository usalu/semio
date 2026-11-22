"""🏗️ Hand-chosen numbers of the used bicycle shelter: rig and the parts of every authored scale.

A municipal mono-pitch cantilever shelter of hot-dip galvanised steel (catalogue type of the 2000s): three frames at 2.40 m centres, each a
rear post RHS 100 x 100 x 4 on a dowelled base plate, a cantilever roof beam RHS 120 x 60 x 4 at 5 degrees and a knee brace CHS 60.3 x 3.2;
three purlins RHS 60 x 40 x 3, four 16 mm multiwall polycarbonate sheets of 1.25 m bays under aluminium glazing bars, a half-round eaves
gutter with a downpipe at the rear; four galvanised Sheffield stands CHS 48.3 x 3.2 (0.75 x 0.80 m) at 1.20 m centres under the roof and one
abandoned city bike chained to the second stand. See `../../reports/used-bicycle-shelter.md` for the reference table.
"""
import math

from author_geometry import C5, S5, T5, Part, Rig, t

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
ZF = {"a": -2.40, "b": 0.0, "c": 2.40}
ZP = {"1": -1.875, "2": -0.625, "3": 0.625, "4": 1.875}
PANEL_ON = {"1": "a", "2": "b", "3": "b", "4": "c"}
ZS = {"1": -1.80, "2": -0.60, "3": 0.60, "4": 1.80}
ZBAR = [-2.49, -1.25, 0.0, 1.25, 2.49]
POST_X, POST_TOP = -0.80, 2.30
ROOF_X0, ROOF_X1 = -1.05, 1.05
L_B = (ROOF_X1 - ROOF_X0) / C5
PANEL_V = 0.12
PANEL_T = 0.016
PANEL_HALF = 0.615
PANEL_U0, PANEL_U1 = -0.03, L_B + 0.03
PURLIN_U = [0.12, L_B / 2, L_B - 0.12]
GUTTER_X, GUTTER_Y = -1.116, 2.33
STAND_X0, STAND_X1, STAND_TOP, STAND_RO = -0.325, 0.425, 0.80, 0.20415
BIKE_O = (-0.55, 0.351, -0.38)
BRACE_A, BRACE_B = (-0.75, 1.70), None


def yc(x):
    """📈️ Height of the roof-beam centre line at x (passes the post axis at 2.30 m, rises 5 degrees to the front)."""
    return POST_TOP + (x - POST_X) * T5


BRACE_B = (0.20, yc(0.20) - 0.06 / C5)
BRACE_L = math.dist(BRACE_A, BRACE_B)
BRACE_ANG = math.degrees(math.atan2(BRACE_B[1] - BRACE_A[1], BRACE_B[0] - BRACE_A[0]))
STAND_R50 = 0.02415
STAND_ORIGIN_X = STAND_X0 + STAND_R50

RHS100 = {"areaM2": 1.49e-3, "secondMomentM4": 2.26e-6, "extremeFibreM": 0.05, "material": "galv-weathered", "title": t("RHS 100 x 100 x 4, S235, hot-dip galvanised", "RR 100 x 100 x 4, S235, feuerverzinkt")}
RHS120 = {"areaM2": 1.34e-3, "secondMomentM4": 2.45e-6, "extremeFibreM": 0.06, "material": "galv-weathered", "title": t("RHS 120 x 60 x 4, S235, hot-dip galvanised", "RR 120 x 60 x 4, S235, feuerverzinkt")}
CHS60 = {"areaM2": 5.74e-4, "secondMomentM4": 2.35e-7, "extremeFibreM": 0.03015, "material": "galv-weathered", "title": t("CHS 60.3 x 3.2, S235, hot-dip galvanised", "RO 60,3 x 3,2, S235, feuerverzinkt")}
CHS48 = {"areaM2": 4.53e-4, "secondMomentM4": 1.16e-7, "extremeFibreM": 0.02415, "material": "galv-worn", "title": t("CHS 48.3 x 3.2, S235, hot-dip galvanised", "RO 48,3 x 3,2, S235, feuerverzinkt")}
PIPE76 = {"areaM2": 1.43e-4, "secondMomentM4": 1.03e-7, "extremeFibreM": 0.038, "material": "galv-sheet", "title": t("Downpipe 76 x 0.6 mm, galvanised sheet", "Fallrohr 76 x 0,6 mm, verzinktes Blech")}
PC16 = {"areaM2": 2.77e-3, "secondMomentM4": 1.14e-7, "extremeFibreM": 0.008, "material": "pc-yellowed", "title": t("Multiwall polycarbonate 16 mm, 1.23 m wide (2.7 kg/m²)", "Stegplatte Polycarbonat 16 mm, 1,23 m breit (2,7 kg/m²)")}
BIKE35 = {"areaM2": 1.578e-4, "secondMomentM4": 2.22e-8, "extremeFibreM": 0.0175, "material": "bike-steel", "title": t("Steel down tube 35 x 1.5 mm (idealised axle to axle)", "Stahl-Unterrohr 35 x 1,5 mm (idealisiert Achse zu Achse)")}


def build_rig():
    """🦴️ The rig: ground, three frames (post, beam, brace), four panels, gutter, downpipe, four stands and the bike."""
    rig = Rig()
    rig.add("ground", None, 0, 0, 0, 0, 0, {"type": "fixed"}, t("Ground reference: the paving and the concrete point footings that stay behind", "Bodenbezug: das Pflaster und die Punktfundamente aus Beton, die zurückbleiben"))
    for f, z in ZF.items():
        rig.add(f"post-{f}", "ground", POST_X, 0, z, 90, POST_TOP, {"type": "fixed"}, t(f"Rear post of frame {f.upper()} from its base plate up into the roof beam", f"Rückwärtige Stütze von Rahmen {f.upper()} von der Fußplatte bis in den Dachträger"), RHS100)
        rig.add(f"beam-{f}", f"post-{f}", ROOF_X0, yc(ROOF_X0), z, 5, L_B, {"type": "fixed"}, t(f"Cantilever roof beam of frame {f.upper()} from the gutter end, rising 5 degrees to the front", f"Kragträger des Dachs von Rahmen {f.upper()} vom Rinnenende, nach vorn um 5 Grad steigend"), RHS120)
        rig.add(f"brace-{f}", f"post-{f}", BRACE_A[0], BRACE_A[1], z, BRACE_ANG, BRACE_L, {"type": "fixed"}, t(f"Knee brace of frame {f.upper()} from the post face to the beam underside", f"Kopfband von Rahmen {f.upper()} von der Stützenfläche zur Trägerunterseite"), CHS60)
    for p, z in ZP.items():
        f = PANEL_ON[p]
        joint = {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.2} if p == "3" else {"type": "fixed"}
        title = t("Replacement roof sheet (bay 3), slides along the glazing bars", "Ersatz-Dachplatte (Feld 3), gleitet längs der Sprossen") if p == "3" else t(f"Original roof sheet, bay {p}", f"Ursprüngliche Dachplatte, Feld {p}")
        x, y = ROOF_X0 - PANEL_V * S5, yc(ROOF_X0) + PANEL_V * C5
        rig.add(f"panel-{p}", f"beam-{f}", x, y, z, 5, L_B, joint, title, PC16)
    rig.add("gutter", "beam-b", GUTTER_X, GUTTER_Y, 0, 0, 0.1, {"type": "fixed"}, t("Half-round eaves gutter hung at the beam ends, along the whole rear edge", "Halbrunde Dachrinne an den Trägerenden, über die ganze Hinterkante"))
    rig.add("downpipe", "post-c", GUTTER_X, 0, 2.40, 90, 2.27, {"type": "fixed"}, t("Downpipe behind post C into the gully", "Fallrohr hinter Stütze C in den Ablauf"), PIPE76)
    for s, z in ZS.items():
        rig.add(f"stand-{s}", "ground", STAND_ORIGIN_X, 0, z, 90, STAND_TOP, {"type": "fixed"}, t(f"Sheffield stand {s}, from the base of its rear leg", f"Anlehnbügel {s}, vom Fuß seines hinteren Beins"), CHS48)
    rig.add("bike", "ground", BIKE_O[0], BIKE_O[1], BIKE_O[2], 0, 1.10, {"type": "fixed"}, t("Abandoned city bike chained to stand 2, from the rear axle to the front axle", "Zurückgelassenes Stadtrad, an Bügel 2 angeschlossen, von der Hinter- zur Vorderachse"), BIKE35)
    return rig


def P(pid, bone, kind, geo, material, region, tone, **kw):
    """🧩️ Shorthand for a part."""
    return Part(pid, bone, kind, geo, material, region, tone, **kw)


def frame_parts(rig, s):
    """🏗️ Posts with base plates and nuts, beams, knee braces of the three frames at scale 1:s."""
    f_ = FEATURE[s]
    out = []
    plate_t = {50: 0.015, 100: 0.015, 200: 0.03, 500: 0.10}[s]
    beam_w = 0.06 if s < 500 else 0.12
    brace_r = 0.03015 if s < 500 else 0.0375
    for f in ZF:
        pb, bb, rb = f"post-{f}", f"beam-{f}", f"brace-{f}"
        out.append(P(f"post-{f}", pb, "box", {"c": ((plate_t + POST_TOP) / 2, 0, 0), "s": (POST_TOP - plate_t, 0.10, 0.10)}, "galv-weathered", "steel-frame", 0.55, kg=11.7 * (POST_TOP - 0.015), struct="member"))
        out.append(P(f"plate-{f}", pb, "box", {"c": (plate_t / 2, 0, 0), "s": (plate_t, 0.25, 0.25)}, "steel-rusted", "base-fixings", 0.4, kg=7.36, struct="joint", views=("elevation", "front", "plan") if s < 500 else ("elevation", "front")))
        if s == 50:
            for i, (dv, dw) in enumerate(((0.09, 0.09), (0.09, -0.09), (-0.09, 0.09), (-0.09, -0.09))):
                out.append(P(f"nut-{f}{i + 1}", pb, "cyl", {"axis": "u", "at": (dv, dw), "a0": 0.015, "a1": 0.028, "r": 0.012}, "steel-rusted", "base-fixings", 0.3, role="detail", kg=0.06, views=("elevation", "front")))
        out.append(P(f"beam-{f}", bb, "box", {"c": (L_B / 2, 0, 0) if s < 500 else (L_B / 2, 0.03, 0), "s": (L_B, 0.12, beam_w) if s < 500 else (L_B, 0.18, beam_w)}, "galv-weathered", "steel-frame", 0.55, kg=10.5 * L_B, struct="member", views=("elevation", "front", "plan") if s < 500 else ("elevation", "front")))
        out.append(P(f"brace-{f}", rb, "cyl", {"axis": "u", "at": (0, 0), "a0": -0.045, "a1": BRACE_L + 0.045, "r": brace_r}, "galv-weathered", "steel-frame", 0.5, kg=4.51 * BRACE_L, struct="member", views=("elevation", "front", "plan") if s < 500 else ("elevation", "front")))
    return out


def roof_parts(rig, s):
    """🏠️ Purlins, roof sheets with moss, glazing bars, gutter with litter, downpipe and its clamps at scale 1:s."""
    out = []
    purlin = (0.04, 0.06, 0.06) if s < 500 else (0.10, 0.045, 0.10)
    for i, u in enumerate(PURLIN_U):
        out.append(P(f"purlin-{i + 1}", "beam-b", "box", {"c": (u, (purlin[1] + 0.12) / 2, 0), "s": (purlin[0], 0.12 - purlin[1], 5.0)}, "galv-weathered", "steel-frame", 0.5, kg=4.25 * 5.0, struct="joint", views=("elevation", "front", "plan") if s < 500 else ()))
    for p in ZP:
        new = p == "3"
        u0, hw = (PANEL_U0, PANEL_HALF) if s < 500 else (0.0, 0.625)
        out.append(P(f"panel-{p}", f"panel-{p}", "box", {"c": ((u0 + PANEL_U1) / 2, PANEL_T / 2, 0), "s": (PANEL_U1 - u0, PANEL_T, 2 * hw)}, "pc-clear" if new else "pc-yellowed", "roof-new" if new else "roof-old", 0.9 if new else 0.76, flags={"sheet": True}, kg=2.7 * (PANEL_U1 - PANEL_U0) * 2 * PANEL_HALF, struct="member", extra={"hatch": "flutes"} if s <= 100 else {}))
    moss = {"1": [(0.0, 0.42, -0.58, 0.25), (1.2, 1.5, -0.5, -0.2)], "2": [(0.0, 0.36, -0.55, 0.55)], "4": [(0.0, 0.50, -0.3, 0.58), (0.8, 1.05, 0.1, 0.45)]}
    mt = {50: 0.010, 100: 0.010, 200: 0.010, 500: 0.010}[s]
    keep = moss if s <= 100 else {k: v[:1] for k, v in moss.items()}
    for p, patches in keep.items():
        for j, (u0, u1, w0, w1) in enumerate(patches):
            out.append(P(f"moss-{p}{j + 1}", f"panel-{p}", "box", {"c": ((u0 + u1) / 2, PANEL_T + mt / 2, (w0 + w1) / 2), "s": (u1 - u0, mt, w1 - w0)}, "moss-litter", "moss", 0.32, role="detail", flags={"texture": True}, kg=1.5 * (u1 - u0) * (w1 - w0), views=("elevation", "plan") if s <= 100 else ()))
    if s <= 100:
        prof = [(0.12, -0.01), (0.136, -0.01), (0.136, -0.03), (0.151, -0.03), (0.151, 0.03), (0.136, 0.03), (0.136, 0.01), (0.12, 0.01)]
    elif s == 200:
        prof = [(0.136, -0.03), (0.166, -0.03), (0.166, 0.03), (0.136, 0.03)]
    else:
        prof = [(0.136, -0.03), (0.146, -0.03), (0.146, 0.03), (0.136, 0.03)]
    for i, z in enumerate(ZBAR):
        out.append(P(f"bar-{i + 1}", "beam-b", "extx", {"profile": [(v, w + z) for v, w in prof], "u0": PANEL_U0, "u1": PANEL_U1}, "alu-weathered", "glazing-bars", 0.62, kg=1.0 * (PANEL_U1 - PANEL_U0), role="detail" if s == 500 else "body", flags={"texture": True} if s == 500 else {}, views=() if s == 500 else ("elevation", "front", "plan")))
    gr = 0.0635 if s < 500 else 0.085
    gx = 0.0 if s < 500 else -0.019
    half = [(gx + gr * math.cos(math.radians(a)), -gr * math.sin(math.radians(a))) for a in range(0, 181, 15 if s <= 100 else 30)] if s < 500 else [(gx + gr, 0.0), (gx + gr, -0.12), (gx - gr, -0.12), (gx - gr, 0.0)]
    out.append(P("gutter", "gutter", "ext", {"profile": [(a, b) for a, b in reversed(half)], "w0": -2.52, "w1": 2.52}, "galv-sheet", "rainwater-goods", 0.5, kg=1.3 * 5.04))
    lt = 0.010
    out.append(P("litter", "gutter", "box", {"c": (gx, lt / 2, 0), "s": (2 * gr - 0.004, lt, 5.0)}, "moss-litter", "moss", 0.3, role="detail", flags={"texture": True}, kg=0.7 * 5.04, views=("elevation", "plan") if s == 50 else ()))
    pr = 0.038
    out.append(P("downpipe", "downpipe", "cyl", {"axis": "u", "at": (0, 0), "a0": 0.0, "a1": 2.27, "r": pr}, "galv-sheet", "rainwater-goods", 0.48, kg=1.12 * 2.27, struct="member", flags={"slender": True} if s == 500 else {}))
    ct = {50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075}[s]
    for i, u in enumerate((0.70, 1.80) if s < 500 else (1.80,)):
        out.append(P(f"clamp-{i + 1}", "downpipe", "box", {"c": (u, -0.141, 0), "s": (ct, 0.25, 0.03 if s < 500 else 0.075)}, "galv-sheet", "rainwater-goods", 0.45, kg=0.25, struct="joint"))
    return out


def stand_parts(rig, s):
    """🅿️ The four Sheffield stands as tube centre lines (legs, bends, top rail, cross rail); stand 4 dented by a vehicle at 1:50 and 1:100."""
    r = STAND_R50 if s < 500 else 0.0376
    nb = {50: 3, 100: 2, 200: 2, 500: 1}[s]
    xr, xf, tc, rb = STAND_X0 + r, STAND_X1 - r, STAND_TOP - r, STAND_RO - r
    out, lines = [], {}

    def L(x, y):
        return (y, -(x - STAND_ORIGIN_X), 0.0)

    for k in ZS:
        b = f"stand-{k}"
        legtop = tc - rb
        arc_r = [(xr + rb - rb * math.cos(math.radians(90 * i / nb)), legtop + rb * math.sin(math.radians(90 * i / nb))) for i in range(nb + 1)]
        arc_f = [(xf - rb + rb * math.cos(math.radians(90 * i / nb)), legtop + rb * math.sin(math.radians(90 * i / nb))) for i in reversed(range(nb + 1))]
        dent = k == "4" and s <= 100
        top = [arc_r[-1]] + ([((xr + xf) / 2 + 0.03, tc - 0.025)] if dent else []) + [arc_f[0]]
        cl = [(xr, 0.0)] + arc_r + top[1:-1] + arc_f + [(xf, 0.0)]
        lines[b] = [L(x, y)[:2] for x, y in cl]
        mper = 3.56
        out.append(P(f"{b}-leg-r", b, "cyl", {"axis": "u", "at": (L(xr, 0)[1], 0), "a0": 0.0, "a1": legtop, "r": r}, "galv-worn", "bike-stands", 0.55, kg=mper * legtop, struct="member", views=("front", "plan"), group={"front": b, "plan": b}))
        out.append(P(f"{b}-leg-f", b, "cyl", {"axis": "u", "at": (L(xf, 0)[1], 0), "a0": 0.0, "a1": legtop, "r": r}, "galv-worn", "bike-stands", 0.55, kg=mper * legtop, struct="member", views=("front", "plan"), group={"front": b, "plan": b}))
        seq = arc_r + top[1:-1] + arc_f
        for i in range(len(seq) - 1):
            a, c = L(*seq[i]), L(*seq[i + 1])
            out.append(P(f"{b}-tube-{i + 1}", b, "cap", {"p0": a, "p1": c, "r": r}, "galv-worn", "bike-stands", 0.55, kg=mper * math.dist(a, c), struct="member", views=("front", "plan"), group={"front": b, "plan": b}))
        yb = 0.40
        v0, v1 = L(xr, 0)[1], L(xf, 0)[1]
        out.append(P(f"{b}-cross", b, "cyl", {"axis": "v", "at": (yb, 0), "a0": v1 + r * 0.9, "a1": v0 - r * 0.9, "r": r}, "galv-worn", "bike-stands", 0.55, kg=mper * (abs(v1 - v0) - 2 * r), struct="member", views=("elevation", "front", "plan"), group={"front": b, "plan": b}))
    return out, lines, r


def bike_parts(rig, s):
    """🚲️ The abandoned city bike (28-inch wheels 40-622, 1.10 m wheelbase) in the frame of its bone: origin at the rear axle."""
    out = []
    Af, BB, S, H1, H2 = (1.10, 0.0), (0.43, -0.071), (0.25, 0.479), (0.91, 0.509), (0.968, 0.349)
    SP, ST = (0.225, 0.579), (0.895, 0.639)
    b = "bike"

    def cap(pid, p0, p1, r, mat, reg, tone, kg, struct=None, role="body", views=("elevation", "front", "plan"), flags=None):
        out.append(P(pid, b, "cap", {"p0": p0, "p1": p1, "r": r}, mat, reg, tone, kg=kg, struct=struct, role=role, views=views, flags=flags))

    fr = {50: None, 100: None, 200: 0.015, 500: 0.0375}[s]

    def rr(x):
        return x if fr is None else max(x, fr)

    for wid, cx in (("rear", 0.0), ("front", 1.10)):
        if s == 50:
            out.append(P(f"tyre-{wid}", b, "torus", {"c": (cx, 0, 0), "R": 0.331, "r": 0.020}, "rubber-weathered", "bike-rubber", 0.15, kg=0.75, struct="member"))
            out.append(P(f"rim-{wid}", b, "ring", {"c": (cx, 0, 0), "ri": 0.288, "ro": 0.3115, "hw": 0.011}, "bike-steel", "bike-frame", 0.45, kg=0.6, struct="member"))
            for side, w in (("l", 0.03), ("r", -0.03)):
                out.append(P(f"spokes-{wid}-{side}", b, "cap", {"p0": (cx, 0.03, w), "p1": (cx, 0.289, w * 0.2), "r": 0.001, "repeat": {"count": 16, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 16, 6), "origin": [cx, 0, 0]}}, "bike-steel", "bike-frame", 0.5, role="detail", kg=0.15, struct="member", flags={"texture": True}, views=()))
        else:
            band = {100: 0.061, 200: 0.061, 500: 0.075}[s]
            hw = {100: 0.020, 200: 0.020, 500: 0.0375}[s]
            if s == 100:
                out.append(P(f"tyre-{wid}", b, "ring", {"c": (cx, 0, 0), "ri": 0.311, "ro": 0.351, "hw": 0.020}, "rubber-weathered", "bike-rubber", 0.15, kg=0.75, struct="member"))
                out.append(P(f"rim-{wid}", b, "ring", {"c": (cx, 0, 0), "ri": 0.29, "ro": 0.3115, "hw": 0.011}, "bike-steel", "bike-frame", 0.45, kg=0.6, struct="member"))
            else:
                out.append(P(f"tyre-{wid}", b, "ring", {"c": (cx, 0, 0), "ri": 0.351 - band, "ro": 0.351, "hw": hw}, "rubber-weathered", "bike-rubber", 0.15, kg=1.35, struct="member"))
            n = {100: 8, 200: 4, 500: 4}[s]
            ri = 0.29 if s == 100 else 0.351 - band
            out.append(P(f"spokes-{wid}", b, "cap", {"p0": (cx, 0.0, 0.0), "p1": (cx + (ri + 0.002) * math.cos(0.2), (ri + 0.002) * math.sin(0.2), 0.0), "r": 0.002, "repeat": {"count": n, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / n, 6), "origin": [cx, 0, 0]}}, "bike-steel", "bike-frame", 0.6, role="detail", kg=0.15, struct="member", flags={"texture": True}, views=()))
        if s <= 100:
            hr, hw2 = (0.035, 0.065) if wid == "rear" else (0.03, 0.05)
            out.append(P(f"hub-{wid}", b, "cyl", {"axis": "w", "at": (cx, 0), "a0": -hw2, "a1": hw2, "r": hr}, "bike-steel", "bike-frame", 0.4, kg=1.4 if wid == "rear" else 0.6, struct="member"))
    if s == 500:
        plate = [(0.0, 0.0), (0.20, 0.595), (0.26, 0.595), (0.30, 0.50), (0.92, 0.53), (0.90, 0.66), (0.96, 0.66), (0.985, 0.35), (1.10, 0.0), (0.43, -0.09)]
        out.append(P("frame-plate", b, "ext", {"profile": plate, "w0": -0.0375, "w1": 0.0375}, "bike-steel", "bike-frame", 0.3, kg=6.15, struct="member"))
        out.append(P("saddle", b, "ell", {"c": (0.21, 0.63, 0), "radii": (0.135, 0.0375, 0.08)}, "rubber-weathered", "bike-rubber", 0.2, kg=0.55))
        cap("bar", (0.93, 0.66, -0.29), (0.93, 0.66, 0.29), 0.0375, "bike-steel", "bike-frame", 0.5, 0.4)
        return out
    cap("top-tube", (*S, 0), (*H1, 0), rr(0.0143), "bike-steel", "bike-frame", 0.3, 0.6, "member")
    cap("down-tube", (*BB, 0), (*H2, 0), rr(0.0175), "bike-steel", "bike-frame", 0.3, 0.85, "member")
    cap("seat-tube", (*BB, 0), (*S, 0), rr(0.0143), "bike-steel", "bike-frame", 0.3, 0.55, "member")
    hu = ((H1[0] - H2[0]) / 9, (H1[1] - H2[1]) / 9)
    cap("head-tube", (H2[0] - hu[0], H2[1] - hu[1], 0), (H1[0] + hu[0], H1[1] + hu[1], 0), rr(0.018), "bike-steel", "bike-frame", 0.3, 0.3, "member")
    if s <= 200:
        for side, sg in (("l", 1), ("r", -1)):
            cap(f"chainstay-{side}", (*BB, sg * 0.025), (0.0, 0.0, sg * 0.062), rr(0.0095), "bike-steel", "bike-frame", 0.3, 0.25, "member")
            cap(f"seatstay-{side}", (S[0] - 0.01, S[1] - 0.03, sg * 0.02), (0.0, 0.0, sg * 0.058), rr(0.008), "bike-steel", "bike-frame", 0.3, 0.2, "member")
            cap(f"fork-{side}", (*H2, sg * 0.03), (Af[0], 0.0, sg * 0.048), rr(0.0125), "bike-steel", "bike-frame", 0.3, 0.35, "member")
    else:
        cap("chainstay", (*BB, 0), (0.0, 0.0, 0), rr(0.0095), "bike-steel", "bike-frame", 0.3, 0.5, "member")
        cap("seatstay", (S[0] - 0.01, S[1] - 0.03, 0), (0.0, 0.0, 0), rr(0.008), "bike-steel", "bike-frame", 0.3, 0.4, "member")
        cap("fork", (*H2, 0), (Af[0], 0.0, 0), rr(0.0125), "bike-steel", "bike-frame", 0.3, 0.7, "member")
    cap("seatpost", (*S, 0), (*SP, 0), rr(0.0136), "bike-steel", "bike-frame", 0.5, 0.3)
    out.append(P("saddle", b, "ell", {"c": (0.21, 0.614, 0), "radii": (0.135, 0.035 if s < 500 else 0.0375, 0.08)}, "rubber-weathered", "bike-rubber", 0.2, kg=0.55))
    cap("stem", (*H1, 0), (*ST, 0), rr(0.0125), "bike-steel", "bike-frame", 0.5, 0.35)
    cap("bar-centre", (*ST, -0.10), (*ST, 0.10), rr(0.011), "bike-steel", "bike-frame", 0.5, 0.1)
    for side, sg in (("l", 1), ("r", -1)):
        cap(f"bar-{side}", (*ST, sg * 0.10), (0.78, 0.649, sg * 0.29), rr(0.011), "bike-steel", "bike-frame", 0.5, 0.15)
        if s <= 100:
            cap(f"grip-{side}", (0.80, 0.648, sg * 0.255), (0.765, 0.650, sg * 0.31), 0.016, "rubber-weathered", "bike-rubber", 0.15, 0.08)
    if s <= 100:
        rail = 0.005 if s == 50 else 0.0075
        for side, sg in (("l", 1), ("r", -1)):
            cap(f"rack-rail-{side}", (-0.27, 0.429, sg * 0.07), (0.22, 0.429, sg * 0.07), rail, "bike-steel", "bike-frame", 0.35, 0.3)
            cap(f"rack-strut-{side}", (-0.25, 0.429, sg * 0.07), (0.0, 0.0, sg * 0.07), rail, "bike-steel", "bike-frame", 0.35, 0.15)
        for i, u in enumerate((-0.25, -0.05, 0.15)):
            cap(f"rack-rod-{i + 1}", (u, 0.429, -0.07), (u, 0.429, 0.07), rail, "bike-steel", "bike-frame", 0.35, 0.1)
    elif s == 200:
        out.append(P("rack", b, "box", {"c": (-0.025, 0.444, 0), "s": (0.49, 0.03, 0.14)}, "bike-steel", "bike-frame", 0.35, kg=1.0))
        cap("rack-strut", (-0.25, 0.429, 0.0), (0.0, 0.0, 0.0), 0.015, "bike-steel", "bike-frame", 0.35, 0.2)
    ct = {50: 0.008, 100: 0.015, 200: 0.03, 500: None}[s]
    if ct:
        out.append(P("chainring", b, "cyl", {"axis": "w", "at": BB, "a0": -0.047 - ct, "a1": -0.047, "r": 0.095}, "bike-steel", "bike-frame", 0.25, kg=0.7))
    if s <= 100:
        out.append(P("bb-shell", b, "cyl", {"axis": "w", "at": BB, "a0": -0.056, "a1": 0.056, "r": 0.02}, "bike-steel", "bike-frame", 0.3, kg=0.3, struct="member"))
        cw = 0.012 if s == 50 else 0.015
        out.append(P("crank-r", b, "box", {"c": (BB[0], BB[1] - 0.085, -0.062), "s": (0.03, 0.2, cw)}, "bike-steel", "bike-frame", 0.4, kg=0.3))
        out.append(P("crank-l", b, "box", {"c": (BB[0], BB[1] + 0.085, 0.062), "s": (0.03, 0.2, cw)}, "bike-steel", "bike-frame", 0.4, kg=0.3))
        out.append(P("pedal-r", b, "box", {"c": (BB[0], -0.241, -0.105), "s": (0.07, 0.025, 0.075)}, "rubber-weathered", "bike-rubber", 0.2, kg=0.2))
        out.append(P("pedal-l", b, "box", {"c": (BB[0], 0.099, 0.105), "s": (0.07, 0.025, 0.075)}, "rubber-weathered", "bike-rubber", 0.2, kg=0.2))
    kr = rr(0.009)
    cap("kickstand", (0.47, -0.05, 0.03), (0.29, -0.351 + kr, 0.18), kr, "bike-steel", "bike-frame", 0.35, 0.45, "member")
    if s == 50:
        cap("lock-upper", (0.378, 0.089, 0.0), (0.249, 0.089, -0.22), 0.008, "steel-rusted", "bike-frame", 0.2, 0.6)
        cap("lock-lower", (0.388, 0.059, 0.0), (0.249, 0.059, -0.22), 0.008, "steel-rusted", "bike-frame", 0.2, 0.6)
    return out


def parts_of(rig, s):
    """📦️ Every part of scale 1:s with the stand centre lines and tube radius."""
    st, lines, r = stand_parts(rig, s)
    return frame_parts(rig, s) + roof_parts(rig, s) + st + bike_parts(rig, s), lines, r
