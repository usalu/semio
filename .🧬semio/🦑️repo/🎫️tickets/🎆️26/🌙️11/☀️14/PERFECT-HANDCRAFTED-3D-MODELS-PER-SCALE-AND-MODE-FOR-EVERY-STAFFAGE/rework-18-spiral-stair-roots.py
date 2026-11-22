"""🧭 Rework 18 spiral stair (landing, column post, landing guards, slab fixing): root values (rig masses, dimensions, anchor, areas, provenance, summary), the 2D drawings of every scale and the symbol, redrawn as true views of each scale's body (ruling E1).

Without flag: writes the candidate document to 🗑️generated/rework-18/spiral-stair/🔣️.json for `r3-validate.ts --file` and `r3-2d.ts --file`.
With --apply: re-reads the catalog file and replaces only provenance, summary, dimensions, rig, anchors, representations, symbols, thermal.
"""
import copy
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "rework-18-spiral-stair-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/spiral-stair/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "rework-18", "spiral-stair", "🔣️.json")
SRC = os.path.join(T, "volumes", "components", "spiral-stair.json")
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "cx", "cy", "rx", "ry")
COL_TOP = K.POST_TOP + K.CAP_T
COL_SIDE = [(K.TOP, K.R_HUB), (K.TOP, K.R_CORE), (COL_TOP, K.R_CORE)]
AREAS = {"treads": 9.24, "column-steel": 2.33, "balustrade-steel": 3.26}
r5, text = K.r5, K.text


def pts(p):
    return [[r5(a), r5(b)] for a, b in p]


def restyle(old, **geom):
    s = {k: v for k, v in old.items() if k not in GEOM}
    s.update(geom)
    return s


def poly(p, closed):
    return {"kind": "poly", "points": pts(p), "closed": closed}


def rect(x, y, w, h):
    return {"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)}


def ellipse(r):
    return {"kind": "ellipse", "cx": 0, "cy": 0, "rx": r5(r), "ry": r5(r)}


def tread_xz(k, lap, step=3.0):
    return K.tread_outline(k, lap=lap, r_in=K.R_HUB, step=step)


def tread_extents(k, lap):
    d = K.tread_outline(k, lap=lap, r_in=K.R_HUB, step=0.25)
    return K.extent(d, 0), K.extent(d, 1)


def rail_segments(step):
    """✂️ The handrail centre line cut into pieces of at most 30 degrees, each with its mid azimuth for the depth order."""
    cuts = [K.NEWEL_PHI] + [a for a in range(30, 301, 30)] + [K.TURN_PHI, K.END_PHI]
    out = []
    for i in range(len(cuts) - 1):
        a0, a1 = cuts[i], cuts[i + 1]
        n = max(2, int(math.ceil((a1 - a0) / step)))
        out.append((azimuth(*K.rail_xz((a0 + a1) / 2)), [a0 + (a1 - a0) * j / n for j in range(n + 1)]))
    return out


def azimuth(x, z):
    return math.degrees(math.atan2(z, x))


def rail_xyz(p):
    return K.rail_point(p)


def walking():
    """🚶 The walking line of DIN 18065 at 0.6 m radius: from the middle of tread 1 round the column, then straight across the landing to its exit edge; with the tip and the arrow barbs there."""
    line = [K.polar(0.6, K.phi_mid(1) + (K.TURN_PHI - K.phi_mid(1)) * j / 54) for j in range(55)] + [K.land(0.6, 0.72)]
    tip = line[-1]
    barbs = [K.land(0.54, 0.60), tip, K.land(0.66, 0.60)]
    return line, tip, barbs


def elev_z_tread(phi):
    return 2 - 0.5 * math.sin(math.radians(phi))


def front_z_tread(phi):
    return 5 + math.cos(math.radians(phi))


def sawtooth(h_tread, lap, axis, sign_v):
    """🦷 The 1:500 merged outline (height, lateral) of column, foot and fifteen tread bars: right side up, top, left side down."""
    rng = []
    for k in range(1, 16):
        e = tread_extents(k, lap)[axis]
        lo, hi = (-e[1], -e[0]) if sign_v else e
        rng.append((K.RISE * k - h_tread, K.RISE * k, lo, hi))
    R = K.R_HUB
    right = [(0, K.BASE_R), (0.075, K.BASE_R), (0.075, R)]
    for h0, h1, lo, hi in rng:
        if hi > R + 1e-6:
            right += [(h0, R), (h0, hi), (h1, hi), (h1, R)]
    right += COL_SIDE
    left = [(hh, -r) for hh, r in reversed(COL_SIDE)]
    for h0, h1, lo, hi in reversed(rng):
        if lo < -R - 1e-6:
            left += [(h1, -R), (h1, lo), (h0, lo), (h0, -R)]
    left += [(0.075, -R), (0.075, -K.BASE_R), (0, -K.BASE_R)]
    return right + left


STYLE = {
    "50": {"pen": 0.25, "bal": 0.24, "post": 0.85, "rail": 0.85, "h": K.PAN, "lap": K.LAP, "bal_set": "abc", "sleeves": True, "step": 3.75},
    "100": {"pen": 0.18, "bal": 0.15, "post": 0.42, "rail": 0.42, "h": K.PAN, "lap": K.LAP, "bal_set": "abc", "sleeves": False, "step": 7.5},
    "200": {"pen": 0.13, "bal": 0.15, "post": 0.21, "rail": 0.21, "h": K.PAN, "lap": K.LAP, "bal_set": "abc", "sleeves": False, "step": 7.5},
    "500": {"pen": None, "bal": 0.15, "post": 0.15, "rail": 0.15, "h": 0.075, "lap": 0.0, "bal_set": "abc", "sleeves": False, "step": 7.5},
}


def stroke_base(old_rail, role, tone, pen, region):
    s = {k: v for k, v in old_rail.items() if k not in GEOM and k not in ("id", "z")}
    s.update({"role": role, "tone": tone, "region": region})
    if pen is not None:
        s["penMm"] = pen
    return s


def draw_scale(sc, rep):
    cfg = STYLE[sc]
    g = rep["geometry"]
    old = {v: {s["id"]: s for s in g[v]} for v in g}
    new = {"elevation": [], "front": [], "plan": []}
    E, F, P = new["elevation"], new["front"], new["plan"]
    lap, h = cfg["lap"], cfg["h"]
    oe, of, op = old["elevation"], old["front"], old["plan"]
    rail_e, rail_f, rail_p = (o.get("handrail") or o["handrail-1"] for o in (oe, of, op))
    if "ground" in oe:
        E.append(oe["ground"])
    if sc == "500":
        E.append(restyle(oe["stair-mass"], **poly(sawtooth(h, lap, 0, True), True)))
    else:
        E.append(restyle(oe["column"], z=2, **poly([(0, K.R_HUB)] + COL_SIDE + [(hh, -r) for hh, r in reversed(COL_SIDE)] + [(0, -K.R_HUB)], True)))
        E.append(oe["base-plate"])
        if cfg["sleeves"]:
            for k in range(1, 15):
                E.append(restyle(oe["sleeve-1"], id="sleeve-%d" % k, z=2.01, **poly([(K.RISE * k, K.R_HUB), (K.RISE * k, -K.R_HUB)], False)))
        for k in range(1, 16):
            (x0, x1), _ = tread_extents(k, lap)
            E.append(restyle(oe["tread-%d" % k], z=r5(elev_z_tread(K.phi_mid(k))), **rect(x0, -h, x1 - x0, h)))
    feet = K.bars()
    ip = K.inner_rail()
    inner_phi = azimuth(ip[1][0], ip[1][2])
    bal_e = stroke_base(rail_e, "detail", 0.3, cfg["bal"], "balustrade-steel")
    bal_f = stroke_base(rail_f, "detail", 0.35, cfg["bal"], "balustrade-steel")
    for i, x, z, y0, y1 in feet:
        E.append({"id": i, **bal_e, "z": r5(elev_z_tread(azimuth(x, z)) + 0.05), **poly([(x, y0), (x, y1)], False)})
    for name, x, z, y0, y1 in K.posts():
        E.append({"id": name, **stroke_base(rail_e, "detail", 0.3, cfg["post"], "balustrade-steel"), "z": r5(elev_z_tread(azimuth(x, z)) + 0.06), **poly([(x, y0), (x, y1)], False)})
    E.append(restyle(rail_e, id="handrail-inner", z=r5(elev_z_tread(inner_phi) + 0.07), penMm=cfg["rail"], **poly([(p[0], p[1]) for p in ip], False)))
    segs = rail_segments(cfg["step"])
    for j, (pm, ps) in enumerate(segs, 1):
        E.append(restyle(rail_e, id="handrail-%d" % j, z=r5(elev_z_tread(pm) + 0.07), penMm=cfg["rail"], **poly([(rail_xyz(p)[0], rail_xyz(p)[1]) for p in ps], False)))

    F.append(of["ground"])
    if sc == "500":
        F.append(restyle(of["stair-mass"], **poly([(s, hh) for hh, s in sawtooth(h, lap, 1, False)], True)))
    else:
        F.append(restyle(of["column"], **poly([(K.R_HUB, K.BASE_T)] + [(r, hh) for hh, r in COL_SIDE] + [(-r, hh) for hh, r in reversed(COL_SIDE)] + [(-K.R_HUB, K.BASE_T)], True)))
        F.append(of["base-plate"])
        if cfg["sleeves"]:
            for k in range(1, 15):
                F.append(restyle(of["sleeve-1"], id="sleeve-%d" % k, **poly([(-K.R_HUB, K.RISE * k), (K.R_HUB, K.RISE * k)], False)))
        for k in range(1, 16):
            _, (z0, z1) = tread_extents(k, lap)
            F.append(restyle(of["tread-%d" % k], z=r5(front_z_tread(K.phi_mid(k))), **rect(z0, K.RISE * k - h, z1 - z0, h)))
    for i, x, z, y0, y1 in feet:
        F.append({"id": i, **bal_f, "z": r5(front_z_tread(azimuth(x, z)) + 0.05), **poly([(z, y0), (z, y1)], False)})
    for name, x, z, y0, y1 in K.posts():
        F.append({"id": name, **stroke_base(rail_f, "detail", 0.35, cfg["post"], "balustrade-steel"), "z": r5(front_z_tread(azimuth(x, z)) + 0.06), **poly([(z, y0), (z, y1)], False)})
    F.append(restyle(rail_f, id="handrail-inner", z=r5(front_z_tread(inner_phi) + 0.07), penMm=cfg["rail"], **poly([(p[2], p[1]) for p in ip], False)))
    for j, (pm, ps) in enumerate(segs, 1):
        F.append(restyle(rail_f, id="handrail-%d" % j, z=r5(front_z_tread(pm) + 0.07), penMm=cfg["rail"], **poly([(rail_xyz(p)[2], rail_xyz(p)[1]) for p in ps], False)))

    if "base-plate" in op:
        P.append(op["base-plate"])
    for k in range(1, 16):
        P.append(restyle(op["tread-%d" % k], z=r5(2 + 0.02 * k), **poly(tread_xz(k, lap, 3.0 if sc != "500" else 7.5), True)))
    P.append(restyle(op["column"], **ellipse(K.R_CORE)))
    P.append(restyle(rail_p, id="handrail-inner", penMm=cfg["rail"], **poly([(ip[0][0], ip[0][2]), (ip[-1][0], ip[-1][2])], False)))
    for j, (pm, ps) in enumerate(segs, 1):
        P.append(restyle(rail_p, id="handrail-%d" % j, penMm=cfg["rail"], **poly([(rail_xyz(p)[0], rail_xyz(p)[2]) for p in ps], False)))
    line, tip, barbs = walking()
    if "walking-line" in op:
        P.append(restyle(op["walking-line"], **poly(line, False)))
    if "walking-arrow" in op:
        P.append(restyle(op["walking-arrow"], **poly(barbs, False)))
    if "walking-start" in op:
        P.append(restyle(op["walking-start"], kind="ellipse", cx=r5(line[0][0]), cy=r5(line[0][1]), rx=op["walking-start"]["rx"], ry=op["walking-start"]["ry"]))
    rep["geometry"] = new
    rep["decisions"] = DECISIONS[sc]


DECISIONS = {
    "50": {
        "silhouette": text("The true side view of the stair: the 177.8 mm column of stacked hub sleeves (a joint line at every tread top) on its base plate, fifteen 70 mm tread pans whose bar length is the projection of the lapped sector, 52 balusters, the newel and end posts and the helical handrail; parts behind the column are hidden by it. Front and plan: the front is the same body seen from ahead; the plan shows the fifteen sectors each lapping 30 mm over the one below (the visible nosing edges count the steps), the capped column, the handrail circle from newel to end post and the walking line with start dot and arrow.", "Die wahre Seitenansicht der Treppe: die 177,8-mm-Säule aus gestapelten Nabenhülsen (eine Fuge an jeder Stufenoberkante) auf ihrer Fußplatte, fünfzehn 70-mm-Stufenwannen, deren Balkenlänge die Projektion des übergreifenden Sektors ist, 52 Stäbe, Antritts- und Austrittspfosten und der Wendelhandlauf; Teile hinter der Säule werden von ihr verdeckt. Vorderansicht und Grundriss: die Vorderansicht ist derselbe Körper von vorn; der Grundriss zeigt die fünfzehn Sektoren, jeder 30 mm über die untere Stufe greifend (die sichtbaren Vorderkanten zählen die Stufen), die geschlossene Säule, den Handlaufkreis vom Antritts- zum Austrittspfosten und die Lauflinie mit Startpunkt und Pfeil."),
        "retained": text("Hub sleeve joints, the 30 mm nosing lap, all 52 balusters at 0.24 mm (true 12 mm), posts and rail at 0.85 mm (true 42.4 mm).", "Fugen der Nabenhülsen, der 30-mm-Übergriff, alle 52 Stäbe mit 0,24 mm (wahre 12 mm), Pfosten und Handlauf mit 0,85 mm (wahre 42,4 mm)."),
        "merged": text("Plate and flange of each pan are one bar; in plan the balusters and posts lie under the rail and are covered by it.", "Platte und Bord jeder Wanne sind ein Balken; im Grundriss liegen Stäbe und Pfosten unter dem Handlauf und werden von ihm verdeckt."),
        "omitted": text("Anchor nuts (inside the plate outline), baluster feet, post flanges, the chequer pattern of the plates.", "Ankermuttern (innerhalb der Plattenkontur), Stabfüße, Pfostenflansche, das Riffelmuster der Bleche."),
        "thinElements": text("Balusters, posts and rail are lines whose pen equals their real width.", "Stäbe, Pfosten und Handlauf sind Linien, deren Stift ihrer wahren Breite entspricht."),
        "hatching": text("None.", "Keine."),
        "styles": text("Bars 0.25 mm, balusters 0.24 mm, posts and rail 0.85 mm, sleeve joints 0.1 mm.", "Balken 0,25 mm, Stäbe 0,24 mm, Pfosten und Handlauf 0,85 mm, Hülsenfugen 0,1 mm."),
        "overlays": text("One cell per region; each tread is a cell of the treads region.", "Eine Zelle je Region; jede Stufe ist eine Zelle der Stufenregion."),
    },
    "100": {
        "silhouette": text("Column, fifteen 70 mm tread bars with the projected lapped sectors, 52 balusters, two posts and the helical rail, depth-ordered against the column. Front and plan: the front is the same body from ahead; the plan shows the lapped sectors, the column, the rail from newel to end post and the walking line.", "Säule, fünfzehn 70-mm-Stufenbalken mit den projizierten übergreifenden Sektoren, 52 Stäbe, zwei Pfosten und der Wendelhandlauf, nach der Tiefe gegen die Säule geordnet. Vorderansicht und Grundriss: die Vorderansicht ist derselbe Körper von vorn; der Grundriss zeigt die übergreifenden Sektoren, die Säule, den Handlauf vom Antritts- zum Austrittspfosten und die Lauflinie."),
        "retained": text("All 52 balusters at 0.15 mm (the 15 mm feature for the real 12 mm), posts and rail at 0.42 mm (true 42.4 mm), the 30 mm lap (0.3 mm).", "Alle 52 Stäbe mit 0,15 mm (das Merkmal 15 mm für die wahren 12 mm), Pfosten und Handlauf mit 0,42 mm (wahre 42,4 mm), der 30-mm-Übergriff (0,3 mm)."),
        "merged": text("Hub sleeves and core are one column; each pan is one bar.", "Nabenhülsen und Kern sind eine Säule; jede Wanne ist ein Balken."),
        "omitted": text("Sleeve joints, anchors, feet and flanges.", "Hülsenfugen, Anker, Füße und Flansche."),
        "thinElements": text("Balusters, posts and rail are lines whose pen equals their real width.", "Stäbe, Pfosten und Handlauf sind Linien, deren Stift ihrer wahren Breite entspricht."),
        "hatching": text("None.", "Keine."),
        "styles": text("Bars 0.18 mm outline, balusters 0.15 mm, posts and rail 0.42 mm.", "Balken 0,18 mm Umriss, Stäbe 0,15 mm, Pfosten und Handlauf 0,42 mm."),
        "overlays": text("Cells: treads, column, balustrade.", "Zellen: Stufen, Säule, Geländer."),
    },
    "200": {
        "silhouette": text("The column on its plate, fifteen tread bars with their projected lengths, all 52 balusters as 0.15 mm lines (the 30 mm feature), the posts and the helical rail. Front and plan: the front shows the same body from ahead; the plan shows the fifteen lapped sectors, the column, the rail and the walking line at 0.6 m radius with arrow upward (DIN 1356-1).", "Die Säule auf ihrer Platte, fünfzehn Stufenbalken mit ihren projizierten Längen, alle 52 Stäbe als 0,15-mm-Linien (das Merkmal 30 mm), die Pfosten und der Wendelhandlauf. Vorderansicht und Grundriss: die Vorderansicht zeigt denselben Körper von vorn; der Grundriss zeigt die fünfzehn übergreifenden Sektoren, die Säule, den Handlauf und die Lauflinie bei 0,6 m Radius mit Pfeil nach oben (DIN 1356-1)."),
        "retained": text("Every tread bar (0.07 m = 0.35 mm) with its true projected extent, column 0.178 m, base plate 0.4 m, posts and rail 0.0424 m.", "Jeder Stufenbalken (0,07 m = 0,35 mm) mit seiner wahren Projektion, Säule 0,178 m, Fußplatte 0,4 m, Pfosten und Handlauf 0,0424 m."),
        "merged": text("Hub sleeves and core are one column; each pan is one bar.", "Nabenhülsen und Kern sind eine Säule; jede Wanne ist ein Balken."),
        "omitted": text("Sleeve joints, anchors, baluster feet and post flanges.", "Hülsenfugen, Anker, Stabfüße und Pfostenflansche."),
        "thinElements": text("Balusters are 0.15 mm lines (the 30 mm feature; the real 12 mm would be 0.06 mm), posts and rail 0.21 mm (true width).", "Stäbe sind 0,15-mm-Linien (das Merkmal 30 mm; die wahren 12 mm wären 0,06 mm), Pfosten und Handlauf 0,21 mm (wahre Breite)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Bars and column 0.13 mm outline, balusters 0.15 mm, posts and rail 0.21 mm.", "Balken und Säule 0,13 mm Umriss, Stäbe 0,15 mm, Pfosten und Handlauf 0,21 mm."),
        "overlays": text("Cells: the 15 treads (one region), the column with its plate, the balustrade.", "Zellen: die 15 Stufen (eine Region), die Säule mit Platte, das Geländer."),
    },
    "500": {
        "silhouette": text("One saw-tooth outline: the column with a 0.075 m foot for the base plate and fifteen tread bars of 0.075 m whose projected lengths follow the sectors, left and right, the posts, the handrail and the 52 balusters as 0.15 mm lines (the 75 mm feature; the balusters read as the infill band). Front and plan: the front is the same saw-tooth seen from ahead, the plan the fifteen sectors around the column with the handrail; the free sector is the landing opening.", "Eine Sägezahnkontur: die Säule mit einem 0,075-m-Fuß für die Fußplatte und fünfzehn Stufenbalken von 0,075 m, deren projizierte Längen den Sektoren folgen, links und rechts, die Pfosten, der Handlauf und die 52 Stäbe als 0,15-mm-Linien (das Merkmal 75 mm; die Stäbe lesen sich als Füllband). Vorderansicht und Grundriss: die Vorderansicht ist derselbe Sägezahn von vorn, der Grundriss die fünfzehn Sektoren um die Säule mit dem Handlauf; der freie Sektor ist die Austrittsöffnung."),
        "retained": text("Total height 3.89 m (rail), width 1.60 m (tread tips), 15 risers of 0.1867 m, 22.5 degrees per tread, the balusters and posts that carry the rail.", "Gesamthöhe 3,89 m (Handlauf), Breite 1,60 m (Stufenspitzen), 15 Steigungen von 0,1867 m, 22,5 Grad je Stufe, die Stäbe und Pfosten, die den Handlauf tragen."),
        "merged": text("The 15 treads (0.07 m, drawn at the 0.075 m feature = 0.15 mm, with 0.11 m gaps = 0.22 mm) and the column are one filled outline; the 30 mm nosing lap is below the feature and the sectors meet edge to edge.", "Die 15 Stufen (0,07 m, gezeichnet mit dem Merkmal 0,075 m = 0,15 mm, mit 0,11 m Lücken = 0,22 mm) und die Säule sind eine gefüllte Kontur; der 30-mm-Übergriff liegt unter dem Merkmal, die Sektoren stoßen Kante an Kante."),
        "omitted": text("Sleeve joints, anchors, feet and flanges.", "Hülsenfugen, Anker, Füße und Flansche."),
        "thinElements": text("Posts, rail and balusters are 0.15 mm lines (0.075 m, the feature, wider than the real 0.0424 m and 0.012 m).", "Pfosten, Handlauf und Stäbe sind 0,15-mm-Linien (0,075 m, das Merkmal, breiter als die wahren 0,0424 m und 0,012 m)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Mid-grey mass, black post, rail and baluster lines.", "Mittelgraue Masse, schwarze Pfosten-, Handlauf- und Stablinien."),
        "overlays": text("Cells: the stair mass (treads and column merged) and the balustrade lines.", "Zellen: die Treppenmasse (Stufen und Säule zusammengefasst) und die Geländerlinien."),
    },
}


def draw_symbol(sym):
    g = sym["geometry"]
    oe = {s["id"]: s for s in g["elevation"]}
    of = {s["id"]: s for s in g["front"]}
    op = {s["id"]: s for s in g["plan"]}
    E = [restyle(oe["column"], **poly([(-K.R_HUB, 0), (K.R_HUB, 0), (K.R_HUB, COL_TOP), (-K.R_HUB, COL_TOP)], True)), restyle(oe["base-plate"], **poly([(-K.BASE_R, 0), (K.BASE_R, 0), (K.BASE_R, K.BASE_T), (-K.BASE_R, K.BASE_T)], True))]
    F = [restyle(of["column"], **rect(-K.R_HUB, K.BASE_T, 2 * K.R_HUB, COL_TOP - K.BASE_T)), of["base-plate"]]
    for k in range(1, 16):
        (x0, x1), (z0, z1) = tread_extents(k, K.LAP)
        y0 = K.RISE * k - K.PAN
        E.append(restyle(oe["tread-%d" % k], **poly([(x0, y0), (x1, y0), (x1, K.RISE * k), (x0, K.RISE * k)], True)))
        F.append(restyle(of["tread-%d" % k], **rect(z0, y0, z1 - z0, K.PAN)))
    path = sorted(set([K.NEWEL_PHI + (K.END_PHI - K.NEWEL_PHI) * j / 88 for j in range(89)] + [K.TURN_PHI]))
    E.append(restyle(oe["handrail"], **poly([(rail_xyz(p)[0], rail_xyz(p)[1]) for p in path], False)))
    F.append(restyle(of["handrail"], **poly([(rail_xyz(p)[2], rail_xyz(p)[1]) for p in path], False)))
    P = [restyle(op["tread-plan-%d" % k], **poly(tread_xz(k, K.LAP), True)) for k in range(1, 16)]
    P += [restyle(op["column-plan"], **ellipse(K.R_CORE)), restyle(op["walking-line"], **poly(walking()[0], False)), restyle(op["balustrade-circle"], **poly([(rail_xyz(p)[0], rail_xyz(p)[2]) for p in path], False))]
    sym["geometry"] = {"elevation": E, "plan": P, "front": F}
    sym["dimensionalNote"] = text("Model metres at real size in every view: the 1.6 m diameter, the 22.5 degree tread angle with its 30 mm nosing lap and the 3.89 m height are true.", "Modellmeter in wahrer Größe in jeder Ansicht: der Durchmesser 1,6 m, der Stufenwinkel 22,5 Grad mit 30 mm Übergriff und die Höhe 3,89 m sind echt.")


def roots(a):
    M = K.masses()
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    col_kg, col_com = M["column"]
    bones["column"]["mass"].update({"kg": r5(col_kg), "comX": r5(col_com)})
    bones["column"]["title"] = text("Central column: CHS 168.3 x 6.3 core on its base plate, running on above the landing as its post, capped (fixed root)", "Mittelsäule: Kernrohr CHS 168,3 x 6,3 auf der Fußplatte, über dem Podest als dessen Pfosten weitergeführt, oben geschlossen (feste Wurzel)")
    for k in range(1, 16):
        t = M["treads"][k]
        b = bones["tread-%d" % k]
        b["mass"].update({"kg": r5(t["kg"]), "comX": r5(t["x"]), "comY": r5(t["v"])})
        b["title"] = text("Tread %d (pressed pan on its hub sleeve)" % k, "Stufe %d (gekantete Wanne auf ihrer Nabenhülse)" % k)
    bones["column"]["lengthM"] = r5(K.POST_TOP)
    bones["tread-15"]["title"] = text("Landing (unit 15): pressed pan 0.8 x 0.8 m on its hub sleeve, bolted to the slab edge", "Podest (Einheit 15): gekantete Wanne 0,8 x 0,8 m auf ihrer Nabenhülse, an die Deckenstirn geschraubt")
    bones["tread-15"]["lengthM"] = r5(K.LAND / K.LAND_C)
    bkg, bcom = M["balustrade"]
    bones["balustrade"]["mass"].update({"kg": r5(bkg), "comX": r5(bcom[0]), "comY": r5(bcom[1])})
    bones["balustrade"]["title"] = text("Balustrade: helical handrail with the two level landing rails, newel, turn post and end posts, 52 balusters", "Geländer: Wendelhandlauf mit den zwei waagrechten Podesthandläufen, Antritts-, Wende- und Austrittspfosten, 52 Stäbe")
    total = col_kg + sum(t["kg"] for t in M["treads"].values()) + bkg
    a["dimensions"]["heightM"] = round(COL_TOP, 3)
    a["dimensions"]["widthM"] = round(K.R_OUT + K.LAND / K.LAND_C, 3)
    a["dimensions"]["massKg"] = round(total, 3)
    x, z = K.land(K.R_RAIL, K.END_T)
    fx, fz = K.land(sum(K.FIX_R) / 2, K.LAND)
    for an in a["anchors"]:
        if an["id"] == "rail-point":
            an.update({"x": r5(x), "y": r5(K.RAIL_LEVEL + K.RAIL_R), "z": r5(z)})
            an["title"] = text("Handrail at the end post of the landing", "Handlauf am Austrittspfosten des Podests")
        if an["id"] == "top-tie":
            an.update({"bone": "tread-15", "x": r5(fx), "y": r5(-K.PAN / 2), "z": r5(fz)})
            an["title"] = text("Landing bolted to the edge of the upper slab, horizontal hold", "Podest an die Stirn der oberen Decke geschraubt, waagrechte Halterung")
    for rg in a["thermal"]["regions"]:
        if rg["id"] in AREAS:
            rg["areaM2"] = AREAS[rg["id"]]
        if rg["id"] == "column-steel":
            rg["title"] = text("Column: hub sleeves, cap and base plate", "Säule: Nabenhülsen, Kopfplatte und Fußplatte")
            rg["characteristicLengthM"] = 0.178
        if rg["id"] == "treads":
            rg["title"] = text("Tread pans: top, open underside and flanges", "Stufenwannen: Oberseite, offene Unterseite und Borde")
        if rg["id"] == "balustrade-steel":
            rg["title"] = text("Handrail, posts and balusters", "Handlauf, Pfosten und Stäbe")
    a["summary"] = text(
        "A 1.6 m diameter steel spiral stair as a manufacturer builds it: a CHS 168.3 x 6.3 core column on a 400 mm base plate, fourteen tread units and the 0.8 x 0.8 m landing stacked on it (hub sleeve 177.8 x 4.0 and a pressed 5 mm pan with 70 mm flange), the landing bolted to the edge of the upper slab and guarded on both free sides, 15 risers of 0.187 m (2.80 m floor to floor), 22.5 degrees per tread with a 30 mm nosing lap, three 12 mm balusters per tread, newel, turn post and end posts, and a 42.4 mm helical handrail that runs level over the landing 1.0 m above the tread centre (0.91 m over every nosing); drawn in elevation where every tread projects to a bar whose length follows the helix and the rail reads as a sine curve; a plan symbol gives the true spiral.",
        "Eine Stahl-Wendeltreppe mit 1,6 m Durchmesser, wie sie ein Hersteller baut: ein Kernrohr CHS 168,3 x 6,3 auf einer 400-mm-Fußplatte, vierzehn darauf gesteckte Stufeneinheiten und das Podest 0,8 x 0,8 m (Nabenhülse 177,8 x 4,0 und gekantete 5-mm-Wanne mit 70 mm Bord), das Podest an die Stirn der oberen Decke geschraubt und an beiden freien Seiten umwehrt, 15 Steigungen von 0,187 m (2,80 m Geschosshöhe), 22,5 Grad je Stufe mit 30 mm Übergriff, drei 12-mm-Stäbe je Stufe, Antritts-, Wende- und Austrittspfosten und ein 42,4-mm-Wendelhandlauf, der über dem Podest waagrecht weiterläuft, 1,0 m über der Stufenmitte (0,91 m über jeder Vorderkante); in der Ansicht gezeichnet, in der jede Stufe als Balken mit wendelfolgender Länge erscheint und der Handlauf als Sinuskurve; ein Grundrisssymbol zeigt die echte Spirale.")
    pv = a["provenance"]
    refs = {r["id"]: r for r in pv["references"]}
    refs["din-18065"]["citation"] = "DIN 18065:2020-08 Stairs in buildings, terms, measuring rules, main dimensions: spiral stairs measured on the walking line, 2 s + g about 0.59 to 0.65 m; stairs without risers need a nosing overlap of at least 30 mm; handrail and guard height measured vertically above the nosing."
    refs["en-10219"]["citation"] = "EN 10219-2:2019 Cold formed welded structural hollow sections: CHS 168.3 x 6.3 (25.2 kg/m, A 32.1 cm2, I 1068 cm4), CHS 177.8 x 4.0 (17.1 kg/m, hub sleeves), CHS 42.4 x 2.6 (2.55 kg/m, handrail and posts)."
    refs["spiral-catalogues"]["citation"] = "Manufacturer data for steel spiral stairs of 1.6 m diameter: core column 168 mm with tread units stacked on it (each a hub sleeve one riser long welded to a pressed 5 mm pan with a folded flange, 12 to 14 kg), nosing lap 30 mm, three 12 mm balusters per tread for openings under 120 mm, newel post at the start, 42.4 mm handrail. Typical values consulted."
    if "mbo-38" not in refs:
        pv["references"].append({"id": "mbo-38", "citation": "Musterbauordnung (MBO 2002, as amended) section 38 Umwehrungen and the state building codes: guard height at least 0.90 m (1.10 m above 12 m fall height); where children are to be expected, openings in guards and between treads of open stairs at most 0.12 m."})
    pv["notes"] = text(
        "Tread k is centred at 11.25 + 22.5 (k-1) degrees; its leading edge (nosing) is the radial edge at 22.5 (k-1) degrees shifted back by 30 mm, so it laps over tread k-1; the projected tread bar extent is the true minimum and maximum of that outline. Balusters stand at -7.5, 0 and +7.5 degrees of each tread centre on radius 0.775 m (89 mm clear), the newel replaces the first of tread 1; unit 15 is the landing, the square 0 <= r, t <= 0.8 m in the frame of the ray at 315 degrees (r) and the walking direction there (t), where the rail leaves the helix at the turn post (r = 0.775, t = 0) and runs level at y = 3.707 m to the end post (t = 0.775), with the inner rail at r = 0.025 m from the column post; on the helix the rail centre is y = 0.187 + 1.0 + 0.187 (phi - 11.25) / 22.5 at x = 0.775 cos(phi), z = 0.775 sin(phi). The open riser between pans is 117 mm.",
        "Stufe k ist bei 11,25 + 22,5 (k-1) Grad zentriert; ihre Vorderkante ist die radiale Kante bei 22,5 (k-1) Grad, um 30 mm zurückversetzt, sodass sie über Stufe k-1 greift; die projizierte Balkenlänge ist das wahre Minimum und Maximum dieser Kontur. Die Stäbe stehen bei -7,5, 0 und +7,5 Grad jeder Stufenmitte auf 0,775 m Radius (89 mm licht), der Antrittspfosten ersetzt den ersten der Stufe 1; Einheit 15 ist das Podest, das Quadrat 0 <= r, t <= 0,8 m im System des Strahls bei 315 Grad (r) und der Laufrichtung dort (t), wo der Handlauf die Wendel am Wendepfosten (r = 0,775, t = 0) verlässt und waagrecht auf y = 3,707 m zum Austrittspfosten (t = 0,775) läuft, mit dem inneren Handlauf bei r = 0,025 m ab dem Säulenpfosten; auf der Wendel ist die Handlaufachse y = 0,187 + 1,0 + 0,187 (phi - 11,25) / 22,5 bei x = 0,775 cos(phi), z = 0,775 sin(phi). Die offene Steigung zwischen den Wannen ist 117 mm.")
    for sc in ("50", "100", "200", "500"):
        draw_scale(sc, a["representations"][sc])
        d = a["representations"][sc]["decisions"]
        d["silhouette"] = text(d["silhouette"]["en"] + " " + LANDING_2D[0], d["silhouette"]["de"] + " " + LANDING_2D[1])
    draw_symbol(a["symbols"][0])


LANDING_2D = ("The top unit is the 0.8 x 0.8 m landing level with the upper floor: a longer bar in the views and a square in plan, with the column running on as its post and the two level landing rails on their posts and balusters.",
              "Die oberste Einheit ist das Podest 0,8 x 0,8 m auf Höhe der oberen Decke: in den Ansichten ein längerer Balken, im Grundriss ein Quadrat, mit der als Pfosten weiterlaufenden Säule und den zwei waagrechten Podesthandläufen auf ihren Pfosten und Stäben.")
MEMBERS = ("provenance", "summary", "dimensions", "rig", "anchors", "representations", "symbols", "thermal")


def js_number(text):
    from decimal import Decimal
    v = float(text)
    return format(Decimal(repr(v)), "f") if -7 < math.floor(math.log10(abs(v))) else text


def dumped(value):
    out = re.sub(r"(-?\d+(?:\.\d+)?e-0?\d+)", lambda m: js_number(m.group(1)), json.dumps(value, indent=2, ensure_ascii=False))
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(out.split("\n")))


def member(source, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def main():
    with open(CAT, encoding="utf-8") as f:
        source = f.read()
    doc = json.loads(source)
    before = json.loads(source)
    roots(doc["asset"])
    if "--apply" not in sys.argv:
        cand = copy.deepcopy(doc)
        cand["asset"]["volumes"] = json.load(open(SRC))["volumes"]
        os.makedirs(os.path.dirname(OUT), exist_ok=True)
        with open(OUT, "w") as f:
            json.dump(cand, f, indent=2, ensure_ascii=False)
            f.write("\n")
        print("[DEBUG] candidate", OUT, doc["asset"]["dimensions"])
        return
    for key in MEMBERS:
        start, end = member(source, key)
        if dumped(before["asset"][key]) != source[start:end]:
            raise SystemExit("member %s is not in the plain layout, not written" % key)
        source = source[:start] + dumped(doc["asset"][key]) + source[end:]
    check = json.loads(source)
    assert list(check["asset"]) == list(before["asset"])
    for key in check["asset"]:
        if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
            raise SystemExit("member %s changed unexpectedly" % key)
        if key in MEMBERS:
            assert check["asset"][key] == doc["asset"][key], key
    with open(CAT, "w", encoding="utf-8") as f:
        f.write(source)
    print("[DEBUG] applied", CAT, doc["asset"]["dimensions"])


if __name__ == "__main__":
    main()
