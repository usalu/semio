"""🔭 The four individually authored scales of the used kitchen base cabinet: 1:50 (smallest feature 7.5 mm), 1:100 (15 mm),
1:200 (30 mm) and 1:500 (75 mm). Each builder states its own solids and its own 2D shapes; coarser scales merge and generalise
against their own feature instead of deleting 1:50 parts.

@see ./author_parts.py — the hand-chosen numbers
"""
import math
from author_geom import T, R6, solid_box, solid_extrude, solid_revolve, shape, g_rect, g_poly, g_ellipse, g_path_rings, rect_ring, poly_area, circle_pts
from author_parts import (Scale, FR, BL, LEG, CB, CT, WT, WTT, CX, FT, FX, BD, HALF, MARGIN, FRONT_TOP, DOOR_B_TOP, DRAWER_BOTTOM, RIM, BOWL_OPEN, BOWL_OUT, BOWL_FLOOR, CUTOUT, WASTE, TAP, TONE, DENSITY,
                          blanket_profile, worktop_end, nose_profile, swelling_profile, legs, tap_parts)

FRONTS = "fronts"
CARC = "carcass"


def blanket(sc, detail):
    """🛏 The moving blanket folded to the footprint: rounded folds at 1:50, a declared 12 mm fabric sheet coarser."""
    if detail == "50":
        prof = blanket_profile()
        s = solid_extrude("blanket", "blanket", "xy", prof, -HALF, HALF, "moving-blanket", TONE["blanket"])
        sc.solid(s, "blanket", mass=(poly_area(prof) * 2 * HALF * DENSITY["moving-blanket"], (0.29, BL / 2, 0)))
        sc.draw("elevation", "blanket", "blanket", "body", 7.6, True, TONE["blanket"], g_poly(prof), region="blanket", material="moving-blanket")
    else:
        sc.add("blanket", "blanket", (0, 0, -HALF), (0.580, BL, HALF), "moving-blanket", "blanket", TONE["blanket"], views=("elevation",), solid_extra={"sheet": True}, shape_extra={"penMm": 0.2 if detail == "500" else None})
        sc.views["elevation"][-1]["z"] = 7.3 if detail == "500" else 7.6
    if detail != "500":
        sc.draw("front", "blanket", "blanket", "body", 2.4, True, TONE["blanket"], g_rect(-HALF, 0, 2 * HALF, BL), region="blanket", material="moving-blanket")
    sc.draw("plan", "blanket", "blanket", "body", 0.05, True, TONE["blanket"], g_rect(0, -HALF, 0.580, 2 * HALF), region="blanket", material="moving-blanket")


def plinth(sc, detail):
    """🦶 Clip-on plinth panel on the four front legs (1:50, 1:100 with its clips; 1:200 thickened to 30 mm against the legs)."""
    x0 = 0.500 if detail in ("50", "100") else 0.486
    lo, hi = ((x0, BL + 0.002, -0.598), (0.516, CB - 0.002, 0.598)) if detail in ("50", "100") else ((x0, BL, -HALF), (0.516, CB, HALF))
    sc.add("plinth", "carcass", lo, hi, "melamine-chipboard-aged", "plinth", TONE["plinth"], views=("elevation", "front"), member=False)
    if detail in ("50", "100"):
        for name, z in (("r", -0.565), ("l", 0.035)):
            s = solid_box(f"plinth-clip-{name}", "carcass", FR["carcass"], (0.484, 0.080, z - 0.012), (0.500, 0.110, z + 0.012), "pp-plastic", TONE["leg"], repeat={"count": 2, "translation": [0, 0, 0.53]})
            sc.solid(s, "plinth", member=False, mass=(2 * 0.016 * 0.03 * 0.024 * DENSITY["pp-plastic"], (0.492, 0.095, z + 0.265)))
        sc.draw("elevation", "plinth-clip", "carcass", "body", 7.1, True, TONE["leg"], g_rect(0.484, 0.080 - BL, 0.016, 0.030), region="plinth", material="pp-plastic")


def carcass_panels(sc, detail):
    """🗄 The two 18 mm melamine carcasses (1:50 every board, 1:100 with the two middle sides as one 36 mm partition)."""
    m = "melamine-chipboard-aged"
    t = TONE["carcass"]
    sc.add("side-b-out", "carcass", (0.003, CB, -HALF), (CX, CT, -HALF + BD), m, CARC, t, views=("elevation",))
    if detail == "50":
        sc.add("side-b-in", "carcass", (0.003, CB, -BD), (CX, CT, 0), m, CARC, TONE["inner"], views=())
        sc.add("side-a-in", "carcass", (0.003, CB, 0), (CX, CT, BD), m, CARC, TONE["inner"], views=())
    else:
        sc.add("partition", "carcass", (0.003, CB, -BD), (CX, CT, BD), m, CARC, TONE["inner"], views=())
    sc.add("side-a-out", "carcass", (0.003, CB, HALF - BD), (CX, CT, HALF), m, CARC, t, views=())
    sc.add("bottom-b", "carcass", (0.003, CB, -HALF + BD), (CX, CB + BD, -BD), m, CARC, TONE["inner"], views=())
    sc.add("bottom-a", "carcass", (0.003, CB, BD), (CX, CB + BD, HALF - BD), m, CARC, TONE["inner"], views=())
    if detail == "50":
        prof = [(0.488, CB + BD - BL), (CX, CB + BD - BL), (CX, CB + BD - BL + 0.008), (0.548, CB + BD - BL + 0.009), (0.525, CB + BD - BL + 0.007), (0.500, CB + BD - BL + 0.003)]
        s = solid_extrude("swelling-bottom-a", "carcass", "xy", prof, BD, HALF - BD, "swollen-chipboard", TONE["swollen"])
        sc.solid(s, CARC, mass=(poly_area(prof) * (HALF - 2 * BD) * DENSITY["swollen-chipboard"], (0.53, CB + BD + 0.004, 0.3)))
    if detail == "50":
        sc.add("back-b", "carcass", (0, CB, -HALF), (0.003, CT, 0), "hdf-back", CARC, TONE["back"], views=("elevation",), solid_extra={"sheet": True})
        sc.add("back-a", "carcass", (0, CB, 0), (0.003, CT, HALF), "hdf-back", CARC, TONE["back"], views=(), solid_extra={"sheet": True})
    else:
        sc.add("back", "carcass", (0, CB, -HALF), (0.003, CT, HALF), "hdf-back", CARC, TONE["back"], views=("elevation",), solid_extra={"sheet": True})
    sc.add("rail-b-front", "carcass", (0.460, CT - BD, -HALF + BD), (CX, CT, -BD), m, CARC, TONE["inner"], views=())
    sc.add("rail-b-back", "carcass", (0.003, CT - BD, -HALF + BD), (0.103, CT, -BD), m, CARC, TONE["inner"], views=())
    sc.add("rail-a-front", "carcass", (CX - BD, CT - 0.100, BD), (CX, CT, HALF - BD), m, CARC, TONE["inner"], views=())
    sc.add("rail-a-back", "carcass", (0.003, CT - BD, BD), (0.103, CT, HALF - BD), m, CARC, TONE["inner"], views=())
    sc.add("shelf-b", "carcass", (0.020, 0.470, -HALF + BD + 0.0005), (0.540, 0.488, -BD - 0.0005), m, CARC, TONE["inner"], views=())
    sc.draw("front", "carcass-face", "carcass", "body", 0.5, True, t, g_rect(-HALF, CB, 2 * HALF, CT - CB), region=CARC, material=m)
    sc.draw("plan", "carcass-top", "carcass", "body", 0.4, True, TONE["inner"], g_rect(0, -HALF, CX, 2 * HALF), region=CARC, material=m)


def carcass_tube(sc, detail):
    """🗄 1:200 and 1:500: the carcass as one hollow tube from back to front (bottom, sides, rails), open where the fronts close it; 1:200 with the middle partition, 1:500 one compartment."""
    m = "melamine-chipboard-aged"
    top = CT if detail == "200" else 0.845
    v = lambda y: y - BL
    outer = [(v(CB), -HALF), (v(top), -HALF), (v(top), HALF), (v(CB), HALF)]
    if detail == "200":
        holes = [[(v(CB + BD), -HALF + BD), (v(CT - BD), -HALF + BD), (v(CT - BD), -BD), (v(CB + BD), -BD)], [(v(CB + BD), BD), (v(CT - 0.0005), BD), (v(CT - 0.0005), HALF - BD), (v(CB + BD), HALF - BD)]]
    else:
        holes = [[(v(CB + BD), -HALF + BD), (v(top - BD), -HALF + BD), (v(top - BD), HALF - BD), (v(CB + BD), HALF - BD)]]
    s = solid_extrude("carcass-shell", "carcass", "yz", outer, 0.003, CX, m, TONE["carcass"], holes=holes)
    area = poly_area(outer) - sum(poly_area(h) for h in holes)
    sc.solid(s, CARC, mass=(area * (CX - 0.003) * DENSITY[m], (0.28, (CB + top) / 2, 0)))
    sc.draw("elevation", "carcass-shell", "carcass", "body", 7.4, True, TONE["carcass"], g_rect(0, v(CB), CX, top - CB), region=CARC, material=m)
    sc.add("back", "carcass", (0, CB, -HALF), (0.003, top, HALF), "hdf-back", CARC, TONE["back"], views=(), solid_extra={"sheet": True})
    sc.draw("front", "carcass-face", "carcass", "body", 0.5, True, TONE["carcass"], g_rect(-HALF, CB, 2 * HALF, top - CB), region=CARC, material=m)
    sc.draw("plan", "carcass-top", "carcass", "body", 0.4, True, TONE["inner"], g_rect(0, -HALF, CX, 2 * HALF), region=CARC, material=m)


def doors(sc, detail):
    """🚪 The overlay fronts: door A of the sink unit (hinged left, 718 mm), door B under the drawer (hinged right, 578.5 mm); declared 19 mm sheets from 1:200 on."""
    f = "foil-mdf-front"
    extra = {"sheet": True} if detail in ("200", "500") else None
    pen = {"penMm": 0.2} if detail == "500" else None
    top_a = FRONT_TOP if detail != "500" else 0.8435
    sc.add("door-a", "door-a", (CX, CB, MARGIN), (FX, top_a, HALF - MARGIN), f, FRONTS, TONE["front"], solid_extra=extra, zbias=0.05, shape_extra=pen)
    sc.add("door-b", "door-b", (CX, CB, -(HALF - MARGIN)), (FX, DOOR_B_TOP, -MARGIN), f, FRONTS, TONE["front"], solid_extra=extra, zbias=0.05, shape_extra=pen)
    for v in ("front", "plan"):
        for s in sc.views[v][-2:]:
            s.pop("penMm", None)


def handles(sc, detail):
    """🔩 Handles: the replaced stainless bar on door A (160 mm centres), the original 128 mm aluminium bows on door B and the drawer; one 15 mm block each at 1:100."""
    if detail == "50":
        a = "stainless-bar-handle"
        sc.add("handle-a-post-low", "door-a", (FX, 0.676, 0.039), (FX + 0.020, 0.688, 0.051), a, FRONTS, TONE["handle"], member=False)
        sc.add("handle-a-post-high", "door-a", (FX, 0.836, 0.039), (FX + 0.020, 0.848, 0.051), a, FRONTS, TONE["handle"], member=False)
        sc.add("handle-a-bar", "door-a", (FX + 0.020, 0.652, 0.039), (FX + 0.030, 0.872, 0.051), a, FRONTS, TONE["handle"], member=False)
        b = "aluminium-handle"
        sc.add("handle-b-post-low", "door-b", (FX, 0.591, -0.050), (FX + 0.015, 0.601, -0.040), b, FRONTS, TONE["handle"], member=False)
        sc.add("handle-b-post-high", "door-b", (FX, 0.719, -0.050), (FX + 0.015, 0.729, -0.040), b, FRONTS, TONE["handle"], member=False)
        sc.add("handle-b-bar", "door-b", (FX + 0.015, 0.590, -0.050), (FX + 0.025, 0.730, -0.040), b, FRONTS, TONE["handle"], member=False)
        sc.add("handle-d-post-r", "drawer", (FX, 0.806, -0.369), (FX + 0.015, 0.816, -0.359), b, FRONTS, TONE["handle"], member=False)
        sc.add("handle-d-post-l", "drawer", (FX, 0.806, -0.241), (FX + 0.015, 0.816, -0.231), b, FRONTS, TONE["handle"], member=False)
        sc.add("handle-d-bar", "drawer", (FX + 0.015, 0.806, -0.370), (FX + 0.025, 0.816, -0.230), b, FRONTS, TONE["handle"], member=False)
    elif detail == "100":
        sc.add("handle-a", "door-a", (FX, 0.652, 0.0375), (FX + 0.030, 0.872, 0.0525), "stainless-bar-handle", FRONTS, TONE["handle"], member=False)
        sc.add("handle-b", "door-b", (FX, 0.590, -0.0525), (FX + 0.025, 0.730, -0.0375), "aluminium-handle", FRONTS, TONE["handle"], member=False)
        sc.add("handle-d", "drawer", (FX, 0.8035, -0.370), (FX + 0.025, 0.8185, -0.230), "aluminium-handle", FRONTS, TONE["handle"], member=False)


def drawer(sc, detail):
    """🗃 The drawer of unit B on its roller runners: 1:50 front, box and runners apart; 1:100 runners merged into 28 mm box sides; 1:200 and 1:500 one block behind the front line."""
    c = "melamine-chipboard-aged"
    if detail in ("50", "100"):
        sc.add("drawer-front", "drawer", (CX, DRAWER_BOTTOM, -(HALF - MARGIN)), (FX, FRONT_TOP, -MARGIN), "foil-mdf-front", FRONTS, TONE["front"], zbias=0.05)
        sr = -0.570 if detail == "50" else -0.582
        sl = -0.030 if detail == "50" else -0.018
        sc.add("drawer-side-r", "drawer", (0.060, 0.755, sr), (0.544, 0.855, -0.554), c, CARC, TONE["inner"], views=("elevation", "plan"), zbias=-3.0)
        sc.add("drawer-side-l", "drawer", (0.060, 0.755, -0.046), (0.544, 0.855, sl), c, CARC, TONE["inner"], views=("plan",))
        sc.add("drawer-box-front", "drawer", (0.544, 0.755, -0.554), (CX, 0.855, -0.046), c, CARC, TONE["inner"], views=("plan",))
        sc.add("drawer-back", "drawer", (0.060, 0.755, -0.554), (0.076, 0.855, -0.046), c, CARC, TONE["inner"], views=("plan",))
        sc.add("drawer-bottom", "drawer", (0.076, 0.758, -0.554), (0.544, 0.761, -0.046), "hdf-back", CARC, TONE["back"], views=("plan",), solid_extra={"sheet": True}, zbias=-1.0)
        if detail == "50":
            sc.add("runner-r", "drawer", (0.080, 0.755, -0.582), (0.540, 0.772, -0.570), "zinc-steel-runner", CARC, TONE["runner"], views=("elevation",), zbias=-3.5, mass_kg=0.45)
            sc.add("runner-l", "drawer", (0.080, 0.755, -0.030), (0.540, 0.772, -0.018), "zinc-steel-runner", CARC, TONE["runner"], views=(), mass_kg=0.45)
    else:
        top = 0.864 if detail == "200" else 0.827
        sc.add("drawer-block", "drawer", (0.076, DRAWER_BOTTOM, -HALF + BD), (FX, top, -BD), "foil-mdf-front", FRONTS, TONE["front"], zbias=0.12)


def worktop(sc, detail):
    """🪵 The 38 mm postformed laminate worktop with the cut-out for the inset sink; 1:50 adds the rounded nose and the swollen lip over the sink unit; 1:500 one 75 mm slab under the sink plate."""
    hp = "hpl-worktop"
    if detail == "500":
        sc.add("worktop", "worktop", (0, 0.845, -HALF), (0.600, WTT, HALF), hp, "worktop", TONE["worktop"], views=("elevation", "front", "plan"))
        return
    reach = 0.590 if detail == "50" else 0.600
    outer = [(-HALF, 0), (HALF, 0), (HALF, reach), (-HALF, reach)]
    hole = [(CUTOUT[2], CUTOUT[0]), (CUTOUT[3], CUTOUT[0]), (CUTOUT[3], CUTOUT[1]), (CUTOUT[2], CUTOUT[1])]
    s = solid_extrude("worktop-slab", "worktop", "zx", outer, 0, WT, hp, TONE["worktop"], holes=[hole])
    sc.solid(s, "worktop", mass=((poly_area(outer) - poly_area(hole)) * WT * DENSITY[hp], (0.30, CT + WT / 2, -0.06)))
    if detail == "50":
        nose = nose_profile()
        s = solid_extrude("worktop-nose", "worktop", "xy", nose, -HALF, HALF, hp, TONE["worktop"])
        sc.solid(s, "worktop", mass=(poly_area(nose) * 2 * HALF * DENSITY[hp], (0.595, CT + WT / 2, 0)))
        sw = swelling_profile()
        s = solid_extrude("worktop-swelling", "worktop", "xy", sw, 0.10, 0.56, "swollen-chipboard", TONE["swollen"])
        sc.solid(s, "worktop", member=False, mass=(poly_area(sw) * 0.46 * DENSITY["swollen-chipboard"], (0.592, CT - 0.004, 0.33)))
        sc.draw("elevation", "worktop-swelling", "worktop", "body", 8.1, True, TONE["swollen"], g_poly(sw), region="worktop", material="swollen-chipboard")
        sc.draw("front", "worktop-swelling", "worktop", "body", 2.45, True, TONE["swollen"], g_rect(0.10, CT - 0.0085, 0.46, 0.0085), region="worktop", material="swollen-chipboard")
    sc.draw("elevation", "worktop", "worktop", "body", 8.0, True, TONE["worktop"], g_poly(worktop_end(0.010 if detail == "50" else 0)), region="worktop", material=hp)
    sc.draw("front", "worktop", "worktop", "body", 2.42, True, TONE["worktop"], g_rect(-HALF, CT, 2 * HALF, WT), region="worktop", material=hp)
    sc.draw("plan", "worktop", "worktop", "body", 4.0, True, TONE["worktop"], g_rect(0, -HALF, 0.600, 2 * HALF), region="worktop", material=hp)


def sink(sc, detail):
    """🥣 The 860 x 500 inset stainless sink: 1:50 rolled 8 mm rim, deck, bowl walls and floor with the strainer waste left in its hole (the trap below is cut off); 1:100 a flat deck on the worktop with bowl, floor and strainer; 1:200 the floor closed over the merged strainer; 1:500 one flat sink plate."""
    st = "stainless-steel-sink"
    g = 0.0008 * DENSITY[st]
    x0, x1, z0, z1 = RIM
    u = lambda x: x - RIM[0]
    v = lambda y: y - WTT
    outer = [(z0, u(x0)), (z1, u(x0)), (z1, u(x1)), (z0, u(x1))]
    if detail == "500":
        s = solid_box("sink-plate", "sink", FR["sink"], (x0, WTT, z0), (x1, WTT + 0.001, z1), st, TONE["sink"], sheet=True)
        sc.solid(s, "sink", mass=(((x1 - x0) * (z1 - z0) + 0.3) * g, (0.30, WTT, 0.14)))
        sc.draw("plan", "sink-plate", "sink", "body", 9.0, True, TONE["sink"], g_rect(x0, z0, x1 - x0, z1 - z0, 0.02), region="sink", material=st)
        sc.draw("elevation", "sink-hidden", "sink", "body", 0.1, True, TONE["sink"], g_rect(u(x0), v(0.80), x1 - x0, 0.10), region="sink", material=st)
        return
    deck_top = WTT + 0.008 if detail == "50" else WTT + 0.001
    bo = BOWL_OPEN
    bw = BOWL_OUT
    open_ring = [(bo[2], u(bo[0])), (bo[3], u(bo[0])), (bo[3], u(bo[1])), (bo[2], u(bo[1]))]
    wall_ring = [(bw[2], u(bw[0])), (bw[3], u(bw[0])), (bw[3], u(bw[1])), (bw[2], u(bw[1]))]
    if detail == "50":
        lip_hole = [(z0 + 0.008, u(x0 + 0.008)), (z1 - 0.008, u(x0 + 0.008)), (z1 - 0.008, u(x1 - 0.008)), (z0 + 0.008, u(x1 - 0.008))]
        s = solid_extrude("sink-lip", "sink", "zx", outer, 0, 0.008, st, TONE["sink"], holes=[lip_hole])
        sc.solid(s, "sink", mass=(2 * ((x1 - x0) + (z1 - z0)) * 0.025 * g, (0.30, WTT + 0.004, 0.14)))
        sc.draw("elevation", "sink-lip", "sink", "body", 8.2, True, TONE["sink"], g_rect(0, 0, x1 - x0, 0.008), region="sink", material=st)
        sc.draw("front", "sink-lip", "sink", "body", 2.2, True, TONE["sink"], g_rect(z0, WTT, z1 - z0, 0.008), region="sink", material=st)
    s = solid_extrude("sink-deck", "sink", "zx", outer, v(deck_top) - 0.001, v(deck_top), st, TONE["sink"], holes=[open_ring], sheet=True)
    sc.solid(s, "sink", mass=((poly_area(outer) - poly_area(open_ring)) * g, (0.30, deck_top, 0.10)))
    s = solid_extrude("bowl-walls", "sink", "zx", wall_ring, v(0.762), v(deck_top) - 0.001, st, TONE["sink"], holes=[open_ring])
    sc.solid(s, "sink", mass=(2 * ((bw[1] - bw[0]) + (bw[3] - bw[2])) * (deck_top - 0.762) * g, (0.33, 0.845, 0.34)))
    if detail == "200":
        s = solid_extrude("bowl-floor", "sink", "zx", wall_ring, v(BOWL_FLOOR), v(0.762), st, TONE["sink"], sheet=True)
        sc.solid(s, "sink", mass=(poly_area(wall_ring) * g, (0.33, BOWL_FLOOR, 0.34)))
    else:
        waste = [(WASTE[1] + WASTE[2] * math.cos(2 * math.pi * k / 24), u(WASTE[0]) + WASTE[2] * math.sin(2 * math.pi * k / 24)) for k in range(24)]
        s = solid_extrude("bowl-floor", "sink", "zx", wall_ring, v(BOWL_FLOOR), v(0.762), st, TONE["sink"], holes=[waste], sheet=True)
        sc.solid(s, "sink", mass=((poly_area(wall_ring) - poly_area(waste)) * g, (0.33, BOWL_FLOOR, 0.34)))
        prof = [(v(0.762) + 0.0015, 0), (v(0.762) + 0.0015, 0.047), (v(0.762), 0.047), (v(0.762), 0.040), (v(0.762) - 0.058, 0.040), (v(0.762) - 0.064, 0.020), (v(0.762) - 0.100, 0.020), (v(0.762) - 0.100, 0)]
        if detail == "100":
            prof = [(v(0.762) + 0.0015, 0), (v(0.762) + 0.0015, 0.047), (v(0.762), 0.047), (v(0.762), 0.040), (v(0.762) - 0.064, 0.040), (v(0.762) - 0.064, 0)]
        s = solid_revolve("waste-strainer", "sink", prof, [0, 1, 0], [u(WASTE[0]), 0, WASTE[1]], st, 0.3)
        sc.solid(s, "sink", member=False, mass=(0.35, (WASTE[0], 0.72, WASTE[1])))
    sc.draw("elevation", "bowl-hidden", "sink", "body", 0.1, True, TONE["sink"], g_rect(u(bw[0]), v(BOWL_FLOOR), bw[1] - bw[0], deck_top - BOWL_FLOOR), region="sink", material=st)
    rim_rings = [rect_ring(x0, z0, x1, z1), rect_ring(bo[0], bo[2], bo[1], bo[3])]
    sc.draw("plan", "sink-rim", "sink", "body", 9.0, True, TONE["sink"], g_path_rings(rim_rings), region="sink", material=st)
    sc.draw("plan", "bowl-floor", "sink", "body", 8.5, True, 0.5, g_rect(bo[0], bo[2], bo[1] - bo[0], bo[3] - bo[2]), region="sink", material=st)
    if detail != "200":
        sc.draw("plan", "waste-strainer", "sink", "detail", 8.6, True, 0.35, g_ellipse(WASTE[0], WASTE[1], 0.047, 0.047), material=st)


def traces(sc, detail):
    """🩹 Traces of use below the smallest feature, as detail and texture strokes of the drawing: scuffs, the chipped worktop edge, the swollen and peeled door edge, the two screw holes of the lost handle, knife cuts, ring marks and limescale; fewer and coarser at 1:100, none coarser."""
    if detail not in ("50", "100"):
        return
    vb = lambda y: y - BL
    sc.draw("elevation", "scuff-side", "carcass", "texture", 7.45, False, 0.6, g_poly([(0.42, vb(0.20)), (0.47, vb(0.215)), (0.52, vb(0.205))], closed=False))
    sc.draw("front", "plinth-scuffs", "carcass", "texture", 2.1, False, 0.2, g_poly([(-0.52, 0.06), (-0.40, 0.075), (-0.31, 0.058), (0.05, 0.09), (0.21, 0.07)], closed=False))
    sc.draw("front", "worktop-chip", "worktop", "detail", 2.5, True, 0.35, g_poly([(-0.262, CT + 0.026), (-0.250, CT + 0.038), (-0.232, CT + 0.038), (-0.240, CT + 0.030)]), material="hpl-worktop")
    if detail == "50":
        sc.draw("elevation", "chip-side", "carcass", "detail", 7.45, True, 0.55, g_poly([(CX - 0.012, vb(CB)), (CX, vb(CB)), (CX, vb(CB + 0.010))]), material="melamine-chipboard-aged")
        sc.draw("front", "drawer-scratch", "drawer", "texture", 2.36, False, 0.6, g_poly([(-0.52, 0.775), (-0.44, 0.792), (-0.39, 0.788)], closed=False))
        u = lambda y: y - CB
        for name, y in (("low", 0.736), ("high", 0.864)):
            sc.draw("front", f"old-screw-hole-{name}", "door-a", "detail", 2.4, True, 0.15, g_rect(u(y) - 0.0025, -FT, 0.005, 0.005), solid={"kind": "extruded", "sweep": "w", "z0": 0.0425, "z1": 0.0475})
        sc.draw("front", "door-a-swollen-edge", "door-a", "detail", 2.39, True, 0.6, g_rect(0, -FT - 0.0025, 0.045, 0.0025), solid={"kind": "extruded", "sweep": "w", "z0": MARGIN, "z1": HALF - MARGIN}, region=FRONTS, material="foil-mdf-front")
        for k, (a, b) in enumerate((((0.30, -0.47), (0.42, -0.43)), ((0.33, -0.52), (0.40, -0.50)), ((0.21, -0.41), (0.36, -0.37)), ((0.45, -0.55), (0.52, -0.51)))):
            sc.draw("plan", f"knife-cut-{k}", "worktop", "texture", 4.1, False, 0.3, g_poly([a, b], closed=False))
        sc.draw("plan", "ring-mark-1", "worktop", "texture", 4.1, False, 0.3, g_ellipse(0.40, -0.40, 0.040, 0.040))
        sc.draw("plan", "ring-mark-2", "worktop", "texture", 4.1, False, 0.3, g_ellipse(0.20, -0.48, 0.045, 0.045))
        sc.draw("plan", "limescale-ring", "sink", "texture", 8.7, False, 0.8, g_ellipse(WASTE[0], WASTE[1], 0.085, 0.085))
    ribs = range(7) if detail == "50" else range(0, 7, 2)
    for k in ribs:
        z = -0.25 + 0.05 * k
        sc.draw("plan", f"drainer-rib-{k}", "sink", "texture", 9.1, False, 0.4, g_poly([(0.09, z), (0.51, z)], closed=False))


def build(detail):
    """🏗 One scale: every part in its own generalisation for the smallest feature of `detail`."""
    sc = Scale(detail)
    blanket(sc, detail)
    if detail == "500":
        sc.add("plinth-zone", "carcass", (0.030, BL, -HALF), (0.516, CB, HALF), "pp-plastic", "plinth", TONE["plinth"], views=("elevation", "front", "plan"))
        sc.views["plan"][-1]["z"] = 0.3
        sc.views["front"][-1]["z"] = 2.5
    else:
        legs(sc, "50" if detail == "50" else "100")
        plinth(sc, detail)
    if detail in ("50", "100"):
        carcass_panels(sc, detail)
    else:
        carcass_tube(sc, detail)
    doors(sc, detail)
    handles(sc, detail)
    drawer(sc, detail)
    worktop(sc, detail)
    sink(sc, detail)
    tap_parts(sc, detail)
    traces(sc, detail)
    r = {"50": 0.02, "100": 0.025, "200": 0.035, "500": 0.05}[detail]
    sc.mass_marker("carcass", (0.28, 0.52, 0.0), r)
    sc.mass_marker("worktop", (0.30, CT + WT / 2, 0.0), r)
    return sc
