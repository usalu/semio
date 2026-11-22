"""🧭 Round 3 stair flight: root values (rig on the centroid axis of the real stringer, anchors, loads of a centre stringer, poses, clips, dimensions, masses, areas, provenance, summary) and the 2D drawings of every scale and of the symbol, redrawn as true views of each scale's body (ruling E1).

Without flag: writes the candidate document to 🗑️generated/r3-stair-flight/🔣️.json for `r3-validate.ts --file` and `r3-2d.ts --file`.
With --apply: re-reads the catalog file and replaces only the members listed in MEMBERS, keeping the file's layout.
"""
import copy
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "rework-18-stair-flight-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/stair-flight/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "rework-stair-flight", "stair-flight", "🔣️.json")
SRC = os.path.join(T, "volumes", "components", "stair-flight.json")
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "cx", "cy", "rx", "ry")
r5, text = K.r5, K.text
AREAS = {"stringer-steel": 6.18, "tread-top": 5.95, "balustrade-steel": 5.94}
MASS = K.masses()
LOAD_N = 3000.0 * 2 * K.HALF_W * K.RUN
W_MEMBER = LOAD_N / K.BONE_L
PERSON_N = 800.0


def pts(p):
    return [[r5(a), r5(b)] for a, b in p]


def loc(p):
    return [K.local(q) for q in p]


def shape(base, i, z, geom, **over):
    s = {k: v for k, v in base.items() if k not in GEOM and k != "solid"}
    s.update({"id": i, "z": r5(z)})
    s.update(over)
    s.update(geom)
    return s


def poly(p, closed=True):
    return {"kind": "poly", "points": pts(p), "closed": closed}


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r5(x0), "y": r5(y0), "w": r5(x1 - x0), "h": r5(y1 - y0)}


def dot(cx, cy, r):
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(r), "ry": r5(r)}


def style(role, tone, region, material=None, hatch=None, pen=None, filled=True):
    s = {"bone": "stringer", "role": role, "filled": filled, "tone": tone, "region": region}
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    return s


CFG = {
    "50": {"foot": K.BASE_T, "head": K.HEAD_T, "plate": K.BASE_T, "fin_top": K.DROP, "carrier": True, "nuts": True, "round": True, "t": K.T_TREAD, "fins": True, "plates": True, "hatch": True},
    "100": {"foot": K.BASE_T, "head": K.HEAD_T, "plate": K.BASE_T, "fin_top": K.T_TREAD, "carrier": False, "nuts": False, "round": False, "t": K.T_TREAD, "fins": True, "plates": True, "hatch": True},
    "200": {"foot": 0.03, "head": 0.03, "plate": 0.03, "fin_top": K.T_TREAD, "carrier": False, "nuts": False, "round": False, "t": K.T_TREAD, "fins": True, "plates": True, "hatch": False},
    "500": {"foot": 0.0, "head": 0.0, "plate": None, "fin_top": None, "carrier": False, "nuts": False, "round": False, "t": 0.075, "fins": False, "plates": False, "hatch": False},
}

STEEL_S = style("body", 0.45, "stringer-steel", "steel")
PLATE_S = style("body", 0.35, "stringer-steel", "steel")
FIN_S = style("body", 0.3, "stringer-steel", "steel")
NUT_S = style("detail", 0.2, "stringer-steel", "steel")
TREAD_S = style("body", 0.6, "tread-top", "timber")
EDGE_S = style("detail", 0.5, "tread-top", filled=False)
WALK_S = style("detail", 0.35, "tread-top", filled=False)
RAIL_PENS = {"50": (0.85, 0.43, 0.24), "100": (0.42, 0.21, 0.15), "200": (0.21, 0.15, 0.15), "500": (0.15, 0.15, None)}
RAIL_EVERY = {"50": 1, "100": 1, "200": 1, "500": 1}


def rail_style(pen, tone=0.3):
    return style("detail", tone, "balustrade-steel", "steel", pen=pen, filled=False)


def draw_rails(sc, E, F, P):
    """🚧 The balustrades as strokes whose pen is the real width: side view once (both sides coincide), front one line per side, plan the two handrails."""
    post_pen, low_pen, bal_pen = RAIL_PENS[sc]
    x0, x1 = K.rail_span()
    a, b = K.post_x(K.POST_KS[0]), K.post_x(K.POST_KS[-1])
    if bal_pen:
        for bay in range(len(K.POST_KS) - 1):
            for j in range(RAIL_EVERY[sc], K.BAL_PER_BAY + 1, RAIL_EVERY[sc]):
                x = K.baluster_x(bay, j)
                E.append(shape(rail_style(bal_pen), "baluster-%d-%d" % (bay + 1, j), 4, poly(loc([(x, K.rail_y(x, K.LOW_H)), (x, K.rail_y(x))]), False)))
    if low_pen:
        E.append(shape(rail_style(low_pen), "bottom-rail", 4.1, poly(loc([(a, K.rail_y(a, K.LOW_H)), (b, K.rail_y(b, K.LOW_H))]), False)))
    for n, k in enumerate(K.POST_KS, 1):
        x = K.post_x(k)
        E.append(shape(rail_style(post_pen), "post-%d" % n, 4.2, poly(loc([(x, K.tread_y(k)), (x, K.rail_y(x))]), False)))
    E.append(shape(rail_style(post_pen), "handrail", 4.3, poly(loc([(x0, K.rail_y(x0)), (x1, K.rail_y(x1))]), False)))
    for side, z in (("r", -K.RAIL_Z), ("l", K.RAIL_Z)):
        F.append(shape(rail_style(post_pen), "balustrade-" + side, 5, poly([(z, K.tread_y(K.POST_KS[0])), (z, K.rail_y(x1) + K.RAIL_R)], False)))
        P.append(shape(rail_style(post_pen), "handrail-" + side, 7, poly([(x0, z), (x1, z)], False)))


RAIL_2D = {
    "50": ("Both balustrades are drawn: in the side view (the two sides coincide) the six posts and the handrail at 0.85 mm, the bottom rail at 0.43 mm and the 35 balusters at 0.24 mm, each pen the real width; in the front one line per side, in the plan the two handrails.",
           "Beide Geländer sind gezeichnet: in der Seitenansicht (die zwei Seiten decken sich) die sechs Pfosten und der Handlauf mit 0,85 mm, der Untergurt mit 0,43 mm und die 35 Stäbe mit 0,24 mm, jeder Stift die wahre Breite; in der Vorderansicht eine Linie je Seite, im Grundriss die zwei Handläufe."),
    "100": ("Both balustrades: posts and handrail at 0.42 mm, bottom rail 0.21 mm (the real widths), balusters 0.15 mm (the 15 mm feature); front one line per side, plan the two handrails.",
            "Beide Geländer: Pfosten und Handlauf mit 0,42 mm, Untergurt 0,21 mm (die wahren Breiten), Stäbe 0,15 mm (das Merkmal 15 mm); Vorderansicht eine Linie je Seite, Grundriss die zwei Handläufe."),
    "200": ("Both balustrades: posts and handrail at 0.21 mm, bottom rail and balusters at 0.15 mm (the 30 mm feature); front one line per side, plan the two handrails.",
            "Beide Geländer: Pfosten und Handlauf mit 0,21 mm, Untergurt und Stäbe mit 0,15 mm (das Merkmal 30 mm); Vorderansicht eine Linie je Seite, Grundriss die zwei Handläufe."),
    "500": ("The balustrades are drawn with 0.15 mm lines (the 75 mm feature): handrail, six posts and bottom rail; the balusters are left out as in the model.",
            "Die Geländer sind mit 0,15-mm-Linien (das Merkmal 75 mm) gezeichnet: Handlauf, sechs Pfosten und Untergurt; die Stäbe entfallen wie im Modell."),
}

GROUND_S = {"bone": "stringer", "role": "ground", "filled": False, "tone": 0.5}


def nut_side(x, y0, y1):
    rr = K.NUT_AF / math.sqrt(3)
    return [(x - rr, y0), (x + rr, y0), (x + rr, y1), (x - rr, y1)]


def draw_scale(sc, rep):
    c = CFG[sc]
    d = K.flange_drops()
    E, F, P = [], [], []
    hatch_steel = "steel" if c["hatch"] else None
    head_x = K.RUN - c["head"]
    beam = K.band(d[0], d[3], c["foot"], head_x)
    sid = "stringer"
    E.append(shape(dict(STEEL_S, hatch=hatch_steel) if hatch_steel else STEEL_S, sid, 1, poly(loc(beam))))
    F.append(shape(GROUND_S, "ground", 0, poly([(-0.8, 0), (0.8, 0)], False)))
    F.append(shape(dict(STEEL_S, hatch=hatch_steel) if hatch_steel else STEEL_S, sid, 3, rect(-K.HEA_B / 2, c["foot"], K.HEA_B / 2, K.line_y(head_x, d[0]))))
    P.append(shape(STEEL_S, sid, 1, rect(K.line_x(c["foot"], d[0]), -K.HEA_B / 2, head_x, K.HEA_B / 2)))
    if c["plates"]:
        t = c["plate"]
        E.append(shape(PLATE_S, "base-plate", 2.2, poly(loc(K.rect(K.BASE_X[0], 0.0, K.BASE_X[1], t)))))
        E.append(shape(PLATE_S, "head-plate", 2.2, poly(loc(K.rect(K.RUN - t, K.HEAD_Y[0], K.RUN, K.HEAD_Y[1])))))
        F.append(shape(PLATE_S, "base-plate", 3.1, rect(-K.BASE_HALF, 0.0, K.BASE_HALF, t)))
        F.append(shape(PLATE_S, "head-plate", 4, rect(-K.HEAD_HALF, K.HEAD_Y[0], K.HEAD_HALF, K.HEAD_Y[1])))
    if c["nuts"]:
        y0, y1 = K.BASE_T, K.BASE_T + K.NUT_H
        for j, x in enumerate(K.ANCHOR_X, 1):
            E.append(shape(NUT_S, "base-nut-%d" % j, 2.4, poly(loc(nut_side(x, y0, y1)))))
            E.append(shape(NUT_S, "base-bolt-%d" % j, 2.45, poly(loc(K.rect(x - K.BOLT_R, y1, x + K.BOLT_R, y1 + K.BOLT_OUT)))))
        for j, z in enumerate((-K.ANCHOR_Z, K.ANCHOR_Z), 1):
            F.append(shape(NUT_S, "base-nut-%d" % j, 3.2, rect(z - K.NUT_AF / 2, y0, z + K.NUT_AF / 2, y1)))
            F.append(shape(NUT_S, "base-bolt-%d" % j, 3.25, rect(z - K.BOLT_R, y1, z + K.BOLT_R, y1 + K.BOLT_OUT)))
        xf = K.RUN - K.HEAD_T
        rr = K.NUT_AF / math.sqrt(3)
        for j, y in enumerate(K.HEAD_BOLT_Y, 1):
            E.append(shape(NUT_S, "head-nut-%d" % j, 2.4, poly(loc(K.rect(xf - K.NUT_H, y - rr, xf, y + rr)))))
            E.append(shape(NUT_S, "head-bolt-%d" % j, 2.45, poly(loc(K.rect(xf - K.NUT_H - K.HEAD_BOLT_OUT, y - K.BOLT_R, xf - K.NUT_H, y + K.BOLT_R)))))
    for k in range(1, K.N + 1):
        kk = "%02d" % k
        if c["fins"]:
            E.append(shape(FIN_S, "fin-" + kk, 1.5, poly(loc(K.fin_outline(k, top_drop=c["fin_top"], foot_y=c["foot"])))))
        if c["carrier"]:
            co = K.carrier_outline(k)
            E.append(shape(PLATE_S, "carrier-" + kk, 2, poly(loc(co))))
            F.append(shape(PLATE_S, "carrier-" + kk, r5(1.5 + 0.01 * k), rect(-K.CARRIER_HALF, co[0][1], K.CARRIER_HALF, co[2][1])))
        if sc == "500":
            continue
        tr = K.tread_outline(k, rounded=c["round"], t=c["t"])
        tid = "tread-" + kk
        ts = dict(TREAD_S, hatch="grain") if sc == "50" else TREAD_S
        E.append(shape(ts, tid, 3, poly(loc(tr))))
        F.append(shape(ts, tid, r5(2 + 0.01 * k), rect(-K.HALF_W, K.tread_y(k) - c["t"], K.HALF_W, K.tread_y(k))))
        P.append(shape(TREAD_S, tid, r5(2 + 0.01 * k), rect(K.tread_back(k) - K.DEPTH, -K.HALF_W, K.tread_back(k), K.HALF_W)))
        go = K.guard_outline(k)
        E.append(shape(TREAD_S, "riser-guard-" + kk, 2.9, poly(loc(go))))
        F.append(shape(TREAD_S, "riser-guard-" + kk, r5(1.995 + 0.01 * k), rect(-K.HALF_W, go[0][1], K.HALF_W, go[2][1])))
        if k > 1:
            P.append(shape(EDGE_S, "tread-edge-" + kk, 6, poly([(K.tread_back(k) - K.DEPTH, -K.HALF_W), (K.tread_back(k) - K.DEPTH, K.HALF_W)], False)))
    x_start = K.tread_back(1) - K.DEPTH / 2
    x_end = K.tread_back(K.N) - K.DEPTH / 2
    if sc == "500":
        E.append(shape(TREAD_S, "flight", 3, poly(loc(K.folded_band(c["t"])))))
        F.append(shape(TREAD_S, "flight", 2, rect(-K.HALF_W, 0.0, K.HALF_W, K.H)))
        P = [shape(dict(TREAD_S, regions=["tread-top", "stringer-steel"]), "flight", 1, rect(K.tread_back(1) - K.DEPTH, -K.HALF_W, K.RUN, K.HALF_W))]
        P[0].pop("region", None)
        walk = style("body", 0.2, "tread-top", pen=0.13, filled=False)
        P.append(shape(walk, "walking-line", 2, poly([(x_start, 0), (x_end, 0), (x_end - 0.3, 0.25), (x_end, 0), (x_end - 0.3, -0.25)], False)))
    else:
        P.append(shape(WALK_S, "walking-line", 5, poly([(x_start, 0), (x_end, 0)], False)))
        P.append(shape(WALK_S, "walking-arrow", 5, poly([(x_end - 0.25, 0.1), (x_end, 0), (x_end - 0.25, -0.1)], False)))
        P.append(shape(dict(WALK_S, filled=True), "walking-start", 5, dot(x_start, 0, 0.04)))
    draw_rails(sc, E, F, P)
    rep["geometry"] = {"elevation": E, "front": F, "plan": P}
    rep["decisions"] = dict(DECISIONS[sc])
    sil = rep["decisions"]["silhouette"]
    rep["decisions"]["silhouette"] = text(sil["en"] + " " + RAIL_2D[sc][0], sil["de"] + " " + RAIL_2D[sc][1])


def t2(en, de):
    return text(en, de)


DECISIONS = {
    "50": {
        "silhouette": t2("The true side view of the flight: the HEA 160 centre stringer with steel hatch, cut horizontally on its 15 mm base plate with two of its four anchor nuts and plumb against the 15 mm head plate with two of its four bolt nuts, under every one of the 16 treads a bracket (fin and 8 mm carrier plate), and the 16 timber treads (40 mm, grain hatch, rounded nosing) lapping 30 mm, risers open with a 30 x 30 mm guard batten under each nosing. Front and plan: the front is the flight seen from the landing, the treads stacked at their true heights behind the stringer band, the carrier plates showing beside it, the base plate with its nuts at the foot and the head plate in front at the top; the plan shows the 16 treads, each lapping over the one below, and the walking line with start dot and arrow (DIN 1356-1); stringer, brackets and plates lie under the treads.", "Die wahre Seitenansicht des Laufs: die Mittelwange HEA 160 mit Stahlschraffur, waagrecht auf ihrer 15-mm-Fußplatte geschnitten mit zwei ihrer vier Ankermuttern und lotrecht an der 15-mm-Kopfplatte mit zwei ihrer vier Mutternpaare, unter jeder der 16 Stufen eine Konsole (Steg und 8-mm-Auflagerplatte) und die 16 Holzstufen (40 mm, Faserschraffur, gerundete Vorderkante) mit 30 mm Unterschneidung, Setzstufen offen mit einer Kinderschutzleiste 30 x 30 mm unter jeder Vorderkante. Vorderansicht und Grundriss: die Vorderansicht zeigt den Lauf vom Podest aus, die Stufen in wahren Höhen hinter dem Wangenband, daneben die Auflagerplatten, am Fuß die Fußplatte mit Muttern und oben vorn die Kopfplatte; der Grundriss zeigt die 16 Stufen, jede über die untere greifend, und die Lauflinie mit Startpunkt und Pfeil (DIN 1356-1); Wange, Konsolen und Platten liegen unter den Stufen."),
        "retained": t2("Stringer 152 mm (3.0 mm), treads 310 x 40 mm (6.2 x 0.8 mm) with the 8 mm nosing radius, open risers 141 mm with guard battens 30 x 30 mm (111 mm clear), fins 10 mm and carrier plates 170 x 8 mm, base plate 300 x 15 mm, head plate 280 x 15 mm, M12 nuts (AF 19, 10 mm high) and bolt ends.", "Wange 152 mm (3,0 mm), Stufen 310 x 40 mm (6,2 x 0,8 mm) mit 8 mm Kantenrundung, offene Setzstufen 141 mm mit Kinderschutzleisten 30 x 30 mm (111 mm licht), Stege 10 mm und Auflagerplatten 170 x 8 mm, Fußplatte 300 x 15 mm, Kopfplatte 280 x 15 mm, Muttern M12 (SW 19, 10 mm hoch) und Bolzenenden."),
        "merged": t2("Flanges and web of the HEA are one outline in the side view; the far nuts lie behind the near ones.", "Flansche und Steg der HEA sind in der Seitenansicht eine Kontur; die hinteren Muttern liegen hinter den vorderen."),
        "omitted": t2("Weld seams, root radii, the flange edge lines (9 mm = 0.18 mm from the outline), the slotted holes and the wood screws that fix the treads from below.", "Schweißnähte, Ausrundungen, die Flanschkantenlinien (9 mm = 0,18 mm von der Kontur), die Langlöcher und die Holzschrauben, die die Stufen von unten halten."),
        "thinElements": t2("The 8 mm carrier plates are filled bars of 0.16 mm; nuts and bolt ends are filled at their true size (0.2 to 0.44 mm).", "Die 8-mm-Auflagerplatten sind gefüllte Balken von 0,16 mm; Muttern und Bolzenenden sind in wahrer Größe gefüllt (0,2 bis 0,44 mm)."),
        "hatching": t2("Timber longitudinal hatch on the treads in side and front view, steel hatch at 45 degrees on the stringer (material indication, not a cut; DIN 1356-1).", "Holz-Längsschraffur auf den Stufen in Seiten- und Vorderansicht, Stahlschraffur 45 Grad auf der Wange (Materialandeutung, kein Schnitt; DIN 1356-1)."),
        "styles": t2("Body outline 0.35 mm, nuts and bolts 0.18 mm; mid-grey steel, light timber.", "Körperkontur 0,35 mm, Muttern und Bolzen 0,18 mm; mittelgrauer Stahl, helles Holz."),
        "overlays": t2("One cell per region: the steel (stringer, plates, brackets, anchors) and the treads.", "Eine Zelle je Region: der Stahl (Wange, Platten, Konsolen, Anker) und die Stufen."),
    },
    "100": {
        "silhouette": t2("The HEA stringer with steel hatch on its base plate and against its head plate, one bracket fin under every tread and the 16 plain 40 mm treads with 30 mm lap, risers open with a 30 x 30 mm guard batten under each nosing. Front and plan: the front from the landing shows the treads behind the stringer band, base plate and head plate; the plan the 16 lapping treads and the walking line with start dot and arrow.", "Die HEA-Wange mit Stahlschraffur auf ihrer Fußplatte und an ihrer Kopfplatte, unter jeder Stufe ein Konsolsteg und die 16 glatten 40-mm-Stufen mit 30 mm Unterschneidung, Setzstufen offen mit einer Kinderschutzleiste 30 x 30 mm unter jeder Vorderkante. Vorderansicht und Grundriss: die Vorderansicht vom Podest zeigt die Stufen hinter dem Wangenband, Fuß- und Kopfplatte; der Grundriss die 16 übergreifenden Stufen und die Lauflinie mit Startpunkt und Pfeil."),
        "retained": t2("Stringer 152 mm (1.5 mm), treads 3.1 x 0.4 mm, fins, base plate and head plate (0.15 mm thick), open risers 1.4 mm with the guard battens 0.3 mm (1.1 mm clear).", "Wange 152 mm (1,5 mm), Stufen 3,1 x 0,4 mm, Stege, Fuß- und Kopfplatte (0,15 mm dick), offene Setzstufen 1,4 mm mit den Kinderschutzleisten 0,3 mm (1,1 mm licht)."),
        "merged": t2("The carrier plate is part of the fin, which reaches the tread.", "Die Auflagerplatte ist Teil des Stegs, der bis zur Stufe reicht."),
        "omitted": t2("Anchors and bolts, the nosing radius, welds, grain.", "Anker und Bolzen, die Kantenrundung, Schweißnähte, Maserung."),
        "thinElements": t2("The 15 mm plates are filled bars of 0.15 mm.", "Die 15-mm-Platten sind gefüllte Balken von 0,15 mm."),
        "hatching": t2("Steel hatch at 45 degrees on the stringer (material indication; DIN 1356-1).", "Stahlschraffur 45 Grad auf der Wange (Materialandeutung; DIN 1356-1)."),
        "styles": t2("Outline 0.25 mm; hatched stringer, darker fins, lighter treads.", "Kontur 0,25 mm; schraffierte Wange, dunklere Stege, hellere Stufen."),
        "overlays": t2("Cells: steel and treads.", "Zellen: Stahl und Stufen."),
    },
    "200": {
        "silhouette": t2("The stringer bar on a base plate and against a head plate (both drawn at the 30 mm feature), a fin under every tread and 16 tread boards of 40 mm with the 30 mm lap; open risers keep light between the boards, the guard batten under each nosing is a 0.15 mm square. Front and plan: the front stacks the 16 tread edges at their true heights behind the 0.16 m stringer; the plan shows the lapping treads with the walking line (start dot, arrow upward, DIN 1356-1).", "Der Wangenbalken auf einer Fußplatte und an einer Kopfplatte (beide im Merkmal 30 mm gezeichnet), unter jeder Stufe ein Steg und 16 Stufenbohlen von 40 mm mit 30 mm Unterschneidung; die offenen Setzstufen lassen Licht zwischen den Bohlen, die Kinderschutzleiste unter jeder Vorderkante ist ein Quadrat von 0,15 mm. Vorderansicht und Grundriss: die Vorderansicht stapelt die 16 Stufenkanten in wahren Höhen hinter der Wange 0,16 m; der Grundriss zeigt die übergreifenden Stufen mit der Lauflinie (Startpunkt, Pfeil nach oben, DIN 1356-1)."),
        "retained": t2("Stringer (0.76 mm deep), 16 treads (1.55 x 0.2 mm), 0.9 mm rise, fins and plates.", "Wange (0,76 mm tief), 16 Stufen (1,55 x 0,2 mm), 0,9 mm Steigung, Stege und Platten."),
        "merged": t2("Flanges and web are one bar; carrier plate and fin are one fin.", "Flansche und Steg sind ein Balken; Auflagerplatte und Steg sind ein Steg."),
        "omitted": t2("Anchors, nosing radius, flange lines, grain, hatch.", "Anker, Kantenrundung, Flanschlinien, Maserung, Schraffur."),
        "thinElements": t2("The 40 mm tread is a filled bar of 0.2 mm, the plates 0.15 mm (30 mm, thicker than the real 15 mm).", "Die 40-mm-Stufe ist ein gefüllter Balken von 0,2 mm, die Platten 0,15 mm (30 mm, dicker als die wirklichen 15 mm)."),
        "hatching": t2("None: the stringer is 0.76 mm deep.", "Keine: die Wange ist 0,76 mm tief."),
        "styles": t2("Dark bar reads as steel, lighter boards as timber; outline 0.18 mm.", "Der dunkle Balken liest sich als Stahl, die helleren Bohlen als Holz; Kontur 0,18 mm."),
        "overlays": t2("Stringer with plates and fins, and each tread board, are separate overlay cells.", "Wange mit Platten und Stegen und jede Stufenbohle sind getrennte Überlagerungszellen."),
    },
    "500": {
        "silhouette": t2("A dark sloping stringer band (152 mm, 0.3 mm) from the floor to the landing face under one folded band of the 75 mm feature that carries the 16 steps as a saw-tooth (teeth 0.56 x 0.36 mm). Front and plan: the front from the landing is the 1.2 m wide flight band over the full 2.90 m height; the plan is the tread footprint 4.51 x 1.2 m with the walking line and arrow.", "Ein dunkles geneigtes Wangenband (152 mm, 0,3 mm) vom Boden bis zur Podeststirn unter einem gefalteten Band im Merkmal 75 mm, das die 16 Stufen als Sägezahn trägt (Zähne 0,56 x 0,36 mm). Vorderansicht und Grundriss: die Vorderansicht vom Podest ist das 1,2 m breite Laufband über die ganze Höhe von 2,90 m; der Grundriss ist die Stufenfläche 4,51 x 1,2 m mit Lauflinie und Pfeil."),
        "retained": t2("Height 2.90 m, run 4.48 m, the 16 rises and goings of the saw-tooth, the stringer foot on the floor and its head at the landing face; overall 4.51 x 2.90 m.", "Höhe 2,90 m, Lauflänge 4,48 m, die 16 Steigungen und Auftritte des Sägezahns, der Wangenfuß auf dem Boden und ihr Kopf an der Podeststirn; Gesamtmaß 4,51 x 2,90 m."),
        "merged": t2("Flanges, web, plates and brackets are one bar; the treads are one folded band.", "Flansche, Steg, Platten und Konsolen sind ein Balken; die Stufen sind ein gefaltetes Band."),
        "omitted": t2("The open risers: 111 mm clear under the guard battens, between 75 mm bars 106 mm = 0.21 mm, leave 0.08 mm of white between the pens, below the 0.15 mm printable gap, so they are closed; the 30 mm nosing lap, anchors, the true tread thickness.", "Die offenen Setzstufen: 111 mm licht unter den Kinderschutzleisten, zwischen 75-mm-Balken 106 mm = 0,21 mm, lassen 0,08 mm Weiß zwischen den Stiften, unter der druckbaren Lücke von 0,15 mm, sie sind daher geschlossen; die 30-mm-Unterschneidung, Anker, die wahre Stufendicke."),
        "thinElements": t2("The band is drawn at the 75 mm feature (0.15 mm); the walking line is a 0.13 mm stroke.", "Das Band ist im Merkmal 75 mm gezeichnet (0,15 mm); die Lauflinie ist ein 0,13-mm-Strich."),
        "hatching": t2("None.", "Keine."),
        "styles": t2("Dark stringer, mid-grey band, outline 0.13 mm.", "Dunkle Wange, mittelgraues Band, Kontur 0,13 mm."),
        "overlays": t2("Cells: stringer and flight band; the plan footprint carries both regions.", "Zellen: Wange und Laufband; die Grundrissfläche trägt beide Regionen."),
    },
}


def draw_symbol(sym):
    g = sym["geometry"]
    oe = {s["id"]: s for s in g["elevation"]}
    of = {s["id"]: s for s in g["front"]}
    op = {s["id"]: s for s in g["plan"]}
    d = K.flange_drops()
    E = [shape(oe["stringer-beam"], "stringer-beam", 1, poly(K.band(d[0], d[3], 0.0, K.RUN)))]
    F = [shape(of["stringer"], "stringer", 1, rect(-K.HEA_B / 2, 0.0, K.HEA_B / 2, K.line_y(K.RUN, d[0])))]
    P = [shape(op["flight-outline"], "flight-outline", 1, poly(K.rect(K.tread_back(1) - K.DEPTH, -K.HALF_W, K.RUN, K.HALF_W))),
         shape(op["stringer-outline"], "stringer-outline", 1, poly(K.rect(K.line_x(0.0, d[0]), -K.HEA_B / 2, K.RUN, K.HEA_B / 2)))]
    for k in range(1, K.N + 1):
        kk = "%02d" % k
        x1 = K.tread_back(k)
        y1 = K.tread_y(k)
        E.append(shape(oe["tread-board-01"], "tread-board-" + kk, 2, poly(K.rect(x1 - K.DEPTH, y1 - K.T_TREAD, x1, y1))))
        F.append(shape(of["tread-01"], "tread-" + kk, r5(2 + 0.01 * k), rect(-K.HALF_W, y1 - K.T_TREAD, K.HALF_W, y1)))
        if k > 1:
            P.append(shape(op["tread-edge-02"], "tread-edge-" + kk, 1, poly([(x1 - K.DEPTH, -K.HALF_W), (x1 - K.DEPTH, K.HALF_W)], False)))
    xs, xe = K.tread_back(1) - K.DEPTH / 2, K.tread_back(K.N) - K.DEPTH / 2
    P.append(shape(op["walking-line"], "walking-line", 1, poly([(xs, 0), (xe, 0)], False)))
    P.append(shape(op["walking-arrow"], "walking-arrow", 1, poly([(xe - 0.25, 0.1), (xe, 0), (xe - 0.25, -0.1)], False)))
    P.append(shape(op["walking-start"], "walking-start", 1, dot(xs, 0, 0.04)))
    line = {k: v for k, v in op["walking-line"].items() if k != "penMm"}
    x0, x1 = K.rail_span()
    E.append(shape(line, "handrail", 3, poly([(x0, K.rail_y(x0)), (x1, K.rail_y(x1) + K.RAIL_R)], False)))
    for n, k in enumerate(K.POST_KS, 1):
        x = K.post_x(k)
        E.append(shape(line, "post-%d" % n, 3, poly([(x, K.tread_y(k)), (x, K.rail_y(x))], False)))
    for side, z in (("r", -K.RAIL_Z), ("l", K.RAIL_Z)):
        F.append(shape(line, "balustrade-" + side, 3, poly([(z, K.tread_y(K.POST_KS[0])), (z, K.rail_y(x1) + K.RAIL_R)], False)))
        P.append(shape(line, "handrail-" + side, 1, poly([(x0, z), (x1, z)], False)))
    sym["geometry"] = {"elevation": E, "plan": P, "front": F}
    sym["description"] = text(
        "The conventional sign of a straight flight in all three views at real size: in plan the outline, one line per riser, the stringer as an outline and the walking line with a start dot and an arrow pointing up (DIN 1356-1); in the side view the stringer and the sixteen treads as outlines with the handrail and its six posts; in the front view the stringer and the treads seen from the landing between the two balustrades; in plan the two handrails as lines.",
        "Das Zeichen eines geraden Laufs in allen drei Ansichten in wahrer Größe: im Grundriss der Umriss, eine Linie je Steigung, der Wangenbalken als Umriss und die Lauflinie mit Startpunkt und Pfeil nach oben (DIN 1356-1); in der Seitenansicht der Wangenbalken und die sechzehn Stufen als Umriss mit Handlauf und sechs Pfosten; in der Vorderansicht Wangenbalken und Stufen vom Podest aus gesehen zwischen den zwei Geländern; im Grundriss die zwei Handläufe als Linien.")
    sym["dimensionalNote"] = text(
        "Real size: 4.48 m run (16 goings of 0.28 m) plus the 30 mm nosing lap of the first tread, 1.20 m width, 2.90 m storey height and 3.99 m to the top of the handrail at the landing; the cut line at 1 m above the floor is not drawn. At 1:200 the sign is 22.6 x 6 mm.",
        "Echte Größe: 4,48 m Lauflänge (16 Auftritte zu 0,28 m) zuzüglich 30 mm Unterschneidung der ersten Stufe, 1,20 m Breite, 2,90 m Geschosshöhe und 3,99 m bis Oberkante Handlauf am Podest; die Schnittlinie 1 m über dem Boden ist nicht gezeichnet. Bei 1:200 ist das Zeichen 22,6 x 6 mm groß.")


def roots(a):
    a["summary"] = text(
        "A straight steel centre-stringer flight of 16 risers of 181.25 mm and goings of 280 mm (2 r + g = 0.64 m, 32.9 degrees, DIN 18065) from floor to landing, 2.90 m high and 1.20 m wide: an HEA 160 stringer standing on a base plate with four anchors and bolted to the landing face through a head plate with slotted holes, a welded bracket (fin and carrier plate) under each of the 16 C24 timber treads of 40 mm with a 30 mm nosing lap, open risers narrowed to 111 mm by a 30 x 30 mm guard batten under each nosing; the stringer is pinned at the foot and on a roller at the landing.",
        "Ein gerader Stahltreppenlauf mit Mittelwange, 16 Steigungen von 181,25 mm und Auftritten von 280 mm (2 s + a = 0,64 m, 32,9 Grad, DIN 18065) vom Boden bis zum Podest, 2,90 m hoch und 1,20 m breit: eine Wange HEA 160 auf einer Fußplatte mit vier Ankern, über eine Kopfplatte mit Langlöchern an die Podeststirn geschraubt, unter jeder der 16 Holzstufen aus C24 (40 mm, 30 mm Unterschneidung) eine angeschweißte Konsole aus Steg und Auflagerplatte, Setzstufen offen, durch eine Kinderschutzleiste 30 x 30 mm unter jeder Vorderkante auf 111 mm verengt; die Wange ist am Fuß fest und am Podest verschieblich gelagert.")
    pv = a["provenance"]
    refs = {r["id"]: r for r in pv["references"]}
    refs["en-10365"]["citation"] = "EN 10365:2017 Hot rolled steel channels, I and H sections: HEA 160 h = 152 mm, b = 160 mm, t_w = 6 mm, t_f = 9 mm, r = 15 mm, A = 38.8 cm2, 30.4 kg/m, I_y = 1673 cm4, W_el,y = 220 cm3, surface 0.906 m2/m."
    refs["en-1991-1-1"]["citation"] = "EN 1991-1-1:2002 Eurocode 1, Part 1-1: imposed loads on stairs, q_k = 3.0 kN/m2 (an authored value inside the recommended range; the value depends on the use category); a person on the flight is taken as 0.8 kN."
    if "din-18065" not in refs:
        pv["references"].append({"id": "din-18065", "citation": "DIN 18065:2020-08 Stairs in buildings, terms, measuring rules, main dimensions: step rule 2 r + g = 0.59 to 0.65 m, rise and going limits for buildings in general (r at most 0.19 m, g at least 0.26 m), stairs without risers need a nosing lap of at least 30 mm."})
    pv["notes"] = text(
        "A single centre stringer carries the full 1.2 m width. Tread k (k = 1 to 16) has its top at 0.18125 k m and its back edge at 0.28 k m, its nosing laps 30 mm over tread k-1, tread 16 is flush with the landing floor and abuts the landing face at x = 4.48 m. The top of the top flange runs 50 mm (tread and carrier plate) below the line through the tread back edges, so the stringer centroid axis lies 116.3 mm below it: it meets the floor at x = 0.214 m (pin) and the landing face at 2.761 m (roller); the member is 5.082 m long. Masses from EN 10365 (30.4 kg/m over the 5.05 m steel length), S235 plates and brackets at 7850 kg/m3, C24 treads at 420 kg/m3.",
        "Eine einzige Mittelwange trägt die ganze Breite von 1,2 m. Stufe k (k = 1 bis 16) hat ihre Oberkante bei 0,18125 k m und ihre Hinterkante bei 0,28 k m, ihre Vorderkante greift 30 mm über Stufe k-1, Stufe 16 liegt bündig mit dem Podestboden und stößt bei x = 4,48 m an die Podeststirn. Die Oberkante des Obergurts liegt 50 mm (Stufe und Auflagerplatte) unter der Linie durch die Stufenhinterkanten, die Schwerachse der Wange daher 116,3 mm darunter: sie trifft den Boden bei x = 0,214 m (Festlager) und die Podeststirn in 2,761 m Höhe (Rollenlager); das Bauteil ist 5,082 m lang. Massen nach EN 10365 (30,4 kg/m über 5,05 m Stahllänge), Platten und Konsolen aus S235 mit 7850 kg/m3, Stufen aus C24 mit 420 kg/m3.")
    a["dimensions"]["widthM"] = r5(K.RUN - (K.tread_back(1) - K.DEPTH))
    a["dimensions"]["massKg"] = round(MASS["kg"], 1)
    a["dimensions"]["heightM"] = round(K.rail_y(K.rail_span()[1]) + K.RAIL_R, 3)
    a["summary"] = text(a["summary"]["en"] + " Both open sides carry a balustrade of six posts on the treads, a 42.4 mm handrail 0.95 m above the nosing line, a bottom rail and 12 mm balusters at 105 mm centres.", a["summary"]["de"] + " Beide offenen Seiten tragen ein Geländer aus sechs Pfosten auf den Stufen, einem 42,4-mm-Handlauf 0,95 m über der Vorderkantenlinie, einem Untergurt und 12-mm-Stäben im Abstand von 105 mm.")
    if "mbo-38" not in {r["id"] for r in a["provenance"]["references"]}:
        a["provenance"]["references"].append({"id": "mbo-38", "citation": "Musterbauordnung (MBO 2002, as amended) section 38 Umwehrungen and DIN 18065:2020-08 section 6.9: guards on the open sides of stairs at least 0.90 m high above the nosing line, openings at most 0.12 m where children are to be expected; handrail and posts CHS 42.4 x 2.6 (2.55 kg/m), bottom rail CHS 21.3 x 2.3 (1.08 kg/m) to EN 10219-2, balusters round bar 12 mm (0.888 kg/m)."})
    b = {x["id"]: x for x in a["rig"]["bones"]}
    s = b["stringer"]
    s.update({"x": r5(K.X0), "y": 0, "angleDeg": round(math.degrees(K.TH), 6), "lengthM": r5(K.BONE_L)})
    s["title"] = text("Stringer (HEA 160 centroid axis, foot bearing to landing bearing)", "Wange (Schwerachse der HEA 160, Fußlager bis Podestlager)")
    cu, cv = MASS["comLocal"]
    s["mass"].update({"kg": round(MASS["kg"], 1), "comX": r5(cu), "comY": r5(cv)})
    s["section"]["extremeFibreM"] = K.HEA_H / 2
    w = b["walker-carrier"]
    w.update({"x": r5(K.WALK_U), "y": r5(K.WALK_V)})
    w["title"] = text("Walker load carrier on the tread line (massless)", "Lastträger Person auf der Stufenlinie (masselos)")
    w["joint"]["maxM"] = r5(K.PITCH * K.N)
    for an in a["anchors"]:
        if an["id"] == "landing-roller":
            an["x"] = r5(K.BONE_L)
            an["title"] = text("Landing face, head plate with slotted holes (roller)", "Podeststirn, Kopfplatte mit Langlöchern (Rollenlager)")
        if an["id"] == "foot-pin":
            an["title"] = text("Lower floor, base plate (pin)", "Unterer Boden, Fußplatte (Festlager)")
    lp = K.PITCH * K.N
    for p in a["poses"]:
        if p["id"] == "crowd-live-load":
            p["description"] = text("Imposed load of 3 kN/m2 over the full 1.2 m width on the 4.48 m run, 16.13 kN on the centre stringer, %.3f kN/m along the 5.082 m member." % (W_MEMBER / 1000), "Nutzlast 3 kN/m² über die volle Breite von 1,2 m auf 4,48 m Lauflänge, 16,13 kN auf der Mittelwange, %s kN/m entlang des 5,082 m langen Bauteils." % ("%.3f" % (W_MEMBER / 1000)).replace(".", ","))
            ld = p["loads"][0]
            ld["distributed"].update({"toM": r5(K.BONE_L), "wyNPerM": round(-W_MEMBER, 1)})
        if p["id"] == "point-load-tread":
            p["description"] = text("A person of 0.8 kN at mid-flight; the centre stringer carries all of it.", "Eine Person von 0,8 kN in Laufmitte; die Mittelwange trägt sie ganz.")
            p["bones"]["walker-carrier"]["dx"] = r5(lp / 2)
            ld = p["loads"][0]
            ld["fyN"] = -PERSON_N
            ld["title"] = text("Person 800 N on the stringer", "Person 800 N auf der Wange")
    for cl in a["clips"]:
        if cl["id"] == "moving-point-load":
            cl["keys"][0]["bones"]["walker-carrier"]["dx"] = r5(0.3)
            cl["keys"][1]["bones"]["walker-carrier"]["dx"] = r5(lp - 0.3)
    th = a["thermal"]
    if "balustrade-steel" not in {rg["id"] for rg in th["regions"]}:
        rail = copy.deepcopy(next(rg for rg in th["regions"] if rg["id"] == "stringer-steel"))
        rail.update({"id": "balustrade-steel", "title": text("Balustrades: handrails, posts, bottom rails and balusters", "Geländer: Handläufe, Pfosten, Untergurte und Stäbe"), "areaM2": AREAS["balustrade-steel"], "characteristicLengthM": 0.0424, "geometry": "cylinder", "heatCapacityJM2K": 9400, "skyViewFactor": 0.8})
        th["regions"].append(rail)
    for p in a["poses"]:
        p["thermal"]["regions"].setdefault("balustrade-steel", dict(p["thermal"]["regions"]["stringer-steel"]))
    for rg in th["regions"]:
        if rg["id"] == "tread-top":
            rg["areaM2"] = AREAS["tread-top"]
            rg["characteristicLengthM"] = K.DEPTH
        if rg["id"] == "stringer-steel" and AREAS["stringer-steel"]:
            rg["areaM2"] = AREAS["stringer-steel"]
        if rg["id"] == "stringer-steel":
            rg["title"] = text("Stringer, plates and brackets steel", "Wange, Platten und Konsolen Stahl")
    th["assumptions"][0] = text(
        "The stringer is an HEA 160 (0.906 m2 of surface per metre, about 5.05 m long) with base plate, head plate and 16 welded brackets, average steel thickness 7.5 mm; the tread tops are 16 timber boards of 0.31 x 1.2 m.",
        "Die Wange ist eine HEA 160 (0,906 m² Oberfläche je Meter, etwa 5,05 m lang) mit Fußplatte, Kopfplatte und 16 angeschweißten Konsolen, mittlere Stahldicke 7,5 mm; die Stufenoberseiten sind 16 Holzbohlen 0,31 x 1,2 m.")
    st = a["structure"]
    st["assumptions"] = [
        text("Inclined simply supported centre stringer along its centroid axis: pin at the base plate on the lower floor, roller at the head plate on the landing face (horizontal slotted holes, vertical reaction only); slope 32.92 deg, horizontal span 4.27 m, member 5.082 m, flight 2.90 m high on a 4.48 m run.", "Geneigte, frei aufliegende Mittelwange entlang ihrer Schwerachse: Festlager an der Fußplatte auf dem unteren Boden, Rollenlager an der Kopfplatte an der Podeststirn (waagrechte Langlöcher, nur senkrechte Auflagerkraft); Neigung 32,92 Grad, waagrechte Stützweite 4,27 m, Bauteil 5,082 m, Lauf 2,90 m hoch auf 4,48 m Lauflänge."),
        text("Vertical loads are converted to line loads per metre of member; the single stringer carries the whole 1.2 m width. Loads on one side of the treads twist the stringer; the torsion is taken by the four anchors of the base plate and the four bolts of the head plate and is not evaluated here.", "Vertikale Lasten sind in Linienlasten je Meter Bauteil umgerechnet; die einzige Wange trägt die ganze Breite von 1,2 m. Lasten auf einer Stufenseite verdrehen die Wange; die Torsion nehmen die vier Anker der Fußplatte und die vier Bolzen der Kopfplatte auf, sie wird hier nicht ausgewertet."),
    ]
    for sc in ("50", "100", "200", "500"):
        draw_scale(sc, a["representations"][sc])
    draw_symbol(a["symbols"][0])


MEMBERS = ("summary", "provenance", "dimensions", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure")


def js_number(t):
    from decimal import Decimal
    v = float(t)
    return format(Decimal(repr(v)), "f") if -7 < math.floor(math.log10(abs(v))) else t


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
    if "--area" in sys.argv:
        AREAS["stringer-steel"] = float(sys.argv[sys.argv.index("--area") + 1])
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
