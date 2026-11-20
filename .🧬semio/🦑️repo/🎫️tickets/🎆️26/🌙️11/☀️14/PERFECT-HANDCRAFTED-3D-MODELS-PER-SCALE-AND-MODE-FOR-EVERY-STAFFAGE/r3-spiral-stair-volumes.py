"""🪜 Round 3 spiral stair: the handcrafted solids of every scale (body, thermal, structure), stated part by part, written to the volumes source."""
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-spiral-stair-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
SRC = os.path.join(T, "volumes", "components", "spiral-stair.json")

CO, BA = "column", "balustrade"
ST, SB = "steel", "steel-bright"
T_ST, T_SB, T_DK = 0.5, 0.35, 0.25
R, P = "treads", "column-steel"
B = "balustrade-steel"
PLATE_50 = 0.0075


def tb(k):
    return "tread-%d" % k


def column_core():
    return K.prism("column-core", CO, ST, T_ST, K.circle(K.R_CORE, 48), "yz", K.BASE_T, K.TOP)


def base_plate(bolts):
    holes = [[(ANCHOR[0] + 0.009 * math.cos(t), ANCHOR[1] + 0.009 * math.sin(t)) for t in [2 * math.pi * i / 12 for i in range(12)]] for ANCHOR in [K.polar(K.ANCHOR_R, 45 + 90 * j) for j in range(4)]] if bolts else None
    plate = K.circle(K.BASE_R, 64)
    return K.prism("base-plate", CO, ST, T_ST, [(-x, z) for x, z in plate], "yz", 0, K.BASE_T, holes=[[(-x, z) for x, z in h] for h in holes] if holes else None)


def anchors():
    x, z = K.polar(K.ANCHOR_R, 45)
    v = -x
    out = []
    stud = K.revolve("anchor-stud", CO, ST, T_DK, [[0, K.STUD_R], [K.BASE_T + K.WASHER_T + K.NUT_H + 0.006, K.STUD_R], [K.BASE_T + K.WASHER_T + K.NUT_H + 0.008, K.STUD_R - 0.002]], [1, 0, 0], [0, v, z], role="detail")
    nut = K.prism("anchor-nut", CO, ST, T_DK, [(v + a, z + b) for a, b in K.hexagon(K.NUT_AF)], "yz", K.BASE_T + K.WASHER_T, K.BASE_T + K.WASHER_T + K.NUT_H, role="detail")
    for s in (stud, nut):
        s["repeat"] = {"count": 4, "rotationAxis": [1, 0, 0], "rotationRad": K.r5(math.pi / 2)}
        out.append(s)
    return out


def cap(r=K.R_HUB):
    return K.revolve("column-cap", CO, ST, T_ST, [[0, r], [K.CAP_T - 0.002, r], [K.CAP_T, r - 0.002], [K.CAP_T, 0]], [1, 0, 0], [K.TOP, 0, 0])


def hub(k):
    y0, y1 = K.hub_span(k)
    L = y1 - y0
    return K.revolve("hub-%d" % k, tb(k), ST, T_ST, [[0, K.R_HUB_IN], [L, K.R_HUB_IN], [L, K.R_HUB], [0, K.R_HUB], [0, K.R_HUB_IN]], [0, 1, 0], [0, y0 - K.RISE * k, 0])


def pan(k):
    outer = K.tread_outline(k, step=3.0)
    hole = K.tread_outline(k, inset=K.PLATE_T, r_in=K.R_HUB, step=3.0)
    plate = K.prism("tread-plate-%d" % k, tb(k), ST, T_ST, K.zx(outer), "zx", -PLATE_50, 0)
    flange = K.prism("tread-flange-%d" % k, tb(k), ST, T_ST, K.zx(outer), "zx", -K.PAN, -PLATE_50, holes=[K.zx(hole)])
    return plate, flange


def wedge(k, thick, lap, step, i=None):
    return K.prism(i or "tread-%d" % k, tb(k), ST, T_ST, K.zx(K.tread_outline(k, lap=lap, r_in=K.R_HUB - 0.002, step=step, inner=False)), "zx", -thick, 0)


def balusters(feet, slender):
    out = []
    for tag, d in zip("abc", K.BAL_OFF):
        ks = [k for _, k, p in K.baluster_feet() if (k, tag) == (k, tag) and _.endswith(tag)]
        if not ks or tag not in feet:
            continue
        k0 = ks[0]
        p = K.phi_mid(k0) + d
        x, z = K.polar(K.R_RAIL, p)
        L = K.rail_y(p) - K.RISE * k0
        prof = [[0, 0.011], [0.004, 0.011], [0.009, K.BAL_R], [L, K.BAL_R]] if feet[tag] else [[0, K.BAL_R], [L, K.BAL_R]]
        s = K.revolve("baluster-%s" % tag, BA, SB, T_SB, prof, [0, 1, 0], [x, K.RISE * k0, z], role="detail")
        if slender:
            s["slender"] = True
        out.append(K.spin(s, len(ks), K.STEP, K.RISE))
    return out


def posts(flange, slender):
    out = []
    for name, p, k in (("newel", K.NEWEL_PHI, 1), ("end-post", K.END_PHI, 15)):
        x, z = K.polar(K.R_RAIL, p)
        y0 = K.RISE * k
        L = K.rail_y(p) - y0
        lo = K.FLANGE_T if flange else 0.0
        s = K.revolve(name, BA, SB, T_SB, [[lo, K.POST_R], [L, K.POST_R]], [0, 1, 0], [x, y0, z], role="detail")
        if slender:
            s["slender"] = True
        out.append(s)
        if flange:
            out.append(K.revolve(name + "-flange", BA, SB, T_SB, [[0, K.FLANGE_R - 0.002], [0.002, K.FLANGE_R], [K.FLANGE_T, K.FLANGE_R], [K.FLANGE_T + 0.004, K.POST_R]], [0, 1, 0], [x, y0, z], role="detail"))
    return out


def rail(step, dome, slender):
    s = K.loft("handrail", BA, SB, T_SB, K.rail_path(step=step, dome=dome), role="detail")
    if slender:
        s["slender"] = True
    return s


def roller(r):
    return K.st(K.revolve("support-top-tie", CO, ST, 0.15, [[0, r], [0.12, r]], [0, 0, 1], [K.TOP + K.CAP_T + r - 0.002, 0, -0.06], role="detail"), CO, "support", "top-tie")


def tread_masses(M, r):
    return [K.st(K.ellipsoid("mass-tread-%d" % k, tb(k), ST, 0.1, [r, r, r], [M["treads"][k]["x"], M["treads"][k]["v"], M["treads"][k]["z"]]), tb(k), "mass") for k in range(1, 16)]


def main_masses(M, rc, rb):
    col_kg, col_com = M["column"]
    _, bcom = M["balustrade"]
    return [K.st(K.ellipsoid("mass-column", CO, ST, 0.1, [rc, rc, rc], [col_com, 0, 0]), CO, "mass"),
            K.st(K.ellipsoid("mass-balustrade", BA, ST, 0.1, [rb, rb, rb], bcom), BA, "mass")]


def core_rod(r, i="st-column-core"):
    return K.st(K.revolve(i, CO, ST, 0.4, [[K.BASE_T, r], [K.TOP, r]], [1, 0, 0], [0, 0, 0]), CO)


def collar(k, wall, depth):
    return K.st(K.revolve("joint-hub-%d" % k, tb(k), ST, 0.3, [[-depth, K.R_HUB - wall], [0, K.R_HUB - wall], [0, K.R_HUB], [-depth, K.R_HUB], [-depth, K.R_HUB - wall]], [0, 1, 0], [0, 0, 0], role="detail"), tb(k), "joint")


ALL = {"a": True, "b": True, "c": True}


def scale50(M):
    body = [column_core(), base_plate(True)] + anchors() + [cap()]
    for k in range(1, 16):
        body.append(hub(k))
        body.extend(pan(k))
    body += balusters(ALL, True) + posts(True, True) + [rail(3.75, True, True)]
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [K.st_of("st-column-core", "column-core", CO)]
    for k in range(1, 16):
        structure += [K.st_of("st-tread-plate-%d" % k, "tread-plate-%d" % k, tb(k)), K.st_of("st-tread-flange-%d" % k, "tread-flange-%d" % k, tb(k)), K.st_of("joint-hub-%d" % k, "hub-%d" % k, tb(k), "joint")]
    structure += [K.st_of("st-handrail", "handrail", BA), K.st_of("st-newel", "newel", BA), K.st_of("st-end-post", "end-post", BA)]
    structure += [K.st_of("st-baluster-%s" % t, "baluster-%s" % t, BA) for t in "abc"]
    structure += [K.st_of("joint-newel-flange", "newel-flange", BA, "joint"), K.st_of("joint-end-post-flange", "end-post-flange", BA, "joint")]
    structure += [K.st_of("support-base-fix", "base-plate", CO, "support", "base-fix"), roller(0.0125)]
    structure += main_masses(M, 0.04, 0.035) + tread_masses(M, 0.02)
    dec = [
        K.text("The 1:50 model is the spiral stair as a manufacturer builds it: a CHS 168.3 x 6.3 core column (its bore is a sealed cavity and is modelled filled, so the closed tube is not counted as open) on a 400 x 20 mm base plate with four M16 anchors (washer, nut), closed by a 6 mm cap; fifteen tread units stacked on the column, each a 177.8 x 4.0 hub sleeve one riser (0.187 m) long welded under a pressed pan of 5 mm plate with a 70 mm deep folded flange, its nosing edge lapping 30 mm over the tread below (DIN 18065, open risers), the pan open underneath; 43 balusters of 12 mm with welded feet, three per tread (89 mm clear, MBO section 38), a newel post on the first and an end post on the last tread, both 42.4 mm on a 70 mm flange, and the 42.4 mm handrail swept along the true helix 1.0 m above the tread centre (0.91 m over every nosing) with domed ends beyond the posts.",
               "Das 1:50-Modell ist die Wendeltreppe, wie sie ein Hersteller baut: eine Kernsäule CHS 168,3 x 6,3 (ihre Bohrung ist ein geschlossener Hohlraum und wird gefüllt modelliert, damit das geschlossene Rohr nicht als offen zählt) auf einer Fußplatte 400 x 20 mm mit vier M16-Ankern (Scheibe, Mutter), oben mit 6 mm Kopfplatte geschlossen; fünfzehn auf die Säule gesteckte Stufeneinheiten, jede eine Nabenhülse 177,8 x 4,0 von einer Steigung (0,187 m) Länge unter einer gekanteten Wanne aus 5-mm-Blech mit 70 mm tiefem Bord, deren Vorderkante 30 mm über die untere Stufe greift (DIN 18065, offene Stufen), unten offen; 43 Stäbe 12 mm mit angeschweißtem Fuß, drei je Stufe (89 mm licht, MBO § 38), ein Antrittspfosten auf der ersten und ein Austrittspfosten auf der letzten Stufe, beide 42,4 mm auf 70-mm-Flansch, und der 42,4-mm-Handlauf entlang der wahren Wendel 1,0 m über der Stufenmitte (0,91 m über jeder Vorderkante) mit gewölbten Enden hinter den Pfosten."),
        K.text("Thermal: every region is the outer surface of its body solids by reference. treads = plate and flange of the fifteen pans (top, the open underside and both faces of the flange); column-steel = hub sleeves, cap, base plate and anchors (the core bore is a sealed cavity and the core face lies under the sleeves); balustrade-steel = handrail, posts with flanges and balusters. One steel layer per region, so no nested shells.",
               "Thermik: Jede Region ist die Außenfläche ihrer Körper per Verweis. treads = Platte und Bord der fünfzehn Wannen (Oberseite, offene Unterseite und beide Bordseiten); column-steel = Nabenhülsen, Kopfplatte, Fußplatte und Anker (die Kernbohrung ist ein geschlossener Hohlraum, die Kernoberfläche liegt unter den Hülsen); balustrade-steel = Handlauf, Pfosten mit Flanschen und Stäbe. Eine Stahlschicht je Region, daher keine verschachtelten Schalen."),
        K.text("Structure: the core tube is the column member; each pan (plate and flange as one folded section) is the cantilever member of its tread and its hub sleeve is the joint that clamps it to the column; handrail, posts and balusters are the balustrade members with the post flanges as joints; the base plate with its anchors is the fixed support, a roller pin at the column head the horizontal tie to the upper slab; mass points sit at the centres of mass of the column, of every tread unit and of the balustrade.",
               "Tragwerk: Das Kernrohr ist das Stützenbauteil; jede Wanne (Platte und Bord als ein gekanteter Querschnitt) ist der Kragarm ihrer Stufe, ihre Nabenhülse der Knoten, der sie an der Säule hält; Handlauf, Pfosten und Stäbe sind die Geländerbauteile mit den Pfostenflanschen als Knoten; die Fußplatte mit Ankern ist das feste Auflager, ein Rollenbolzen am Säulenkopf die waagrechte Halterung an der oberen Decke; Massenpunkte liegen in den Schwerpunkten der Säule, jeder Stufeneinheit und des Geländers."),
    ]
    return body, thermal, structure, dec


def scale100(M):
    body = [K.revolve("column", CO, ST, T_ST, [[K.BASE_T, K.R_HUB], [K.TOP + K.CAP_T, K.R_HUB]], [1, 0, 0], [0, 0, 0]), base_plate(False)]
    body += [wedge(k, K.PAN, K.LAP, 4.5) for k in range(1, 16)]
    body += balusters({"a": False, "b": False, "c": False}, True) + posts(False, True) + [rail(7.5, True, True)]
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [core_rod(K.R_CORE)]
    for k in range(1, 16):
        structure += [K.st_of("st-tread-%d" % k, "tread-%d" % k, tb(k)), collar(k, 0.015, K.PAN)]
    structure += [K.st_of("st-handrail", "handrail", BA), K.st_of("st-newel", "newel", BA), K.st_of("st-end-post", "end-post", BA)]
    structure += [K.st_of("st-baluster-%s" % t, "baluster-%s" % t, BA) for t in "abc"]
    structure += [K.st_of("support-base-fix", "base-plate", CO, "support", "base-fix"), roller(0.015)]
    structure += main_masses(M, 0.05, 0.04) + tread_masses(M, 0.025)
    dec = [
        K.text("At 1:100 (15 mm feature) the core, the fifteen hub sleeves and the cap merge into one 177.8 mm column on the 20 mm base plate (anchors below the feature are left out); each pan becomes one solid 70 mm tread with its 30 mm nosing lap, because its 5 mm walls and the open underside are below what the scale shows; the 43 balusters keep their true 12 mm as slender rods without feet, the posts their 42.4 mm without flanges, and the handrail follows the helix through sections every 7.5 degrees.",
               "Bei 1:100 (Merkmal 15 mm) verschmelzen Kern, die fünfzehn Nabenhülsen und die Kopfplatte zu einer Säule von 177,8 mm auf der 20-mm-Fußplatte (die Anker liegen unter dem Merkmal und entfallen); jede Wanne wird eine volle 70-mm-Stufe mit ihrem 30-mm-Übergriff, weil ihre 5-mm-Wände und die offene Unterseite unter dem liegen, was der Maßstab zeigt; die 43 Stäbe behalten ihre wahren 12 mm als schlanke Stäbe ohne Fuß, die Pfosten ihre 42,4 mm ohne Flansch, und der Handlauf folgt der Wendel mit Schnitten alle 7,5 Grad."),
        K.text("Thermal: by reference as at 1:50; the treads region is now the outside of fifteen solid treads (the inside of the pans is merged), the column region the outside of the column and plate.",
               "Thermik: per Verweis wie bei 1:50; die Stufenregion ist nun die Außenseite von fünfzehn vollen Stufen (das Wanneninnere ist zusammengefasst), die Säulenregion die Außenseite von Säule und Platte."),
        K.text("Structure: the 168.3 mm core as the column member inside the merged column, the fifteen treads as members with a 15 mm hub collar as the joint at the column, handrail, posts and balusters as members, base plate and head roller as supports, mass points for column, treads and balustrade.",
               "Tragwerk: der 168,3-mm-Kern als Stützenbauteil in der zusammengefassten Säule, die fünfzehn Stufen als Bauteile mit einem 15-mm-Nabenring als Knoten an der Säule, Handlauf, Pfosten und Stäbe als Bauteile, Fußplatte und Rolle am Kopf als Auflager, Massenpunkte für Säule, Stufen und Geländer."),
    ]
    return body, thermal, structure, dec


def scale200(M):
    body = [K.revolve("column", CO, ST, T_ST, [[K.BASE_T, K.R_HUB], [K.TOP + K.CAP_T, K.R_HUB]], [1, 0, 0], [0, 0, 0]), K.revolve("base-plate", CO, ST, T_ST, [[0, K.BASE_R], [K.BASE_T, K.BASE_R]], [1, 0, 0], [0, 0, 0])]
    body += [wedge(k, K.PAN, K.LAP, 7.5) for k in range(1, 16)]
    body += balusters({"a": False, "b": False, "c": False}, True) + posts(False, True) + [rail(7.5, False, True)]
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [core_rod(K.R_CORE)]
    for k in range(1, 16):
        structure += [K.st_of("st-tread-%d" % k, "tread-%d" % k, tb(k)), collar(k, 0.03, K.PAN)]
    structure += [K.st_of("st-handrail", "handrail", BA), K.st_of("st-newel", "newel", BA), K.st_of("st-end-post", "end-post", BA)] + [K.st_of("st-baluster-%s" % t, "baluster-%s" % t, BA) for t in "abc"]
    structure += [K.st_of("support-base-fix", "base-plate", CO, "support", "base-fix"), roller(0.03)]
    structure += main_masses(M, 0.06, 0.05)
    dec = [
        K.text("At 1:200 (30 mm feature) the stair is the column of 177.8 mm on its plate, fifteen solid 70 mm treads with the 30 mm nosing lap in coarser arcs, the handrail through sections every 7.5 degrees without domed ends, the two posts and all 43 balusters without feet. Posts, balusters and rail keep their true diameters as slender parts; the drawing shows the balusters as 0.1 mm lines that read as the infill band.",
               "Bei 1:200 (Merkmal 30 mm) ist die Treppe die Säule von 177,8 mm auf ihrer Platte, fünfzehn volle 70-mm-Stufen mit dem 30-mm-Übergriff in gröberen Bögen, der Handlauf mit Schnitten alle 7,5 Grad ohne gewölbte Enden, die beiden Pfosten und alle 43 Stäbe ohne Fuß. Pfosten, Stäbe und Handlauf behalten als schlanke Teile ihren wahren Durchmesser; die Zeichnung zeigt die Stäbe als 0,1-mm-Linien, die als Füllband gelesen werden."),
        K.text("Thermal: by reference to the body solids; at this scale only the total area of all regions is compared.",
               "Thermik: per Verweis auf die Körper; in diesem Maßstab wird nur die Gesamtfläche aller Regionen verglichen."),
        K.text("Structure: core member, fifteen tread members with a 30 mm hub collar as joint, handrail, posts and balusters as members, plate and head roller as supports, mass points for column and balustrade (the tread masses are part of the tread members).",
               "Tragwerk: Kernbauteil, fünfzehn Stufenbauteile mit einem 30-mm-Nabenring als Knoten, Handlauf, Pfosten und Stäbe als Bauteile, Platte und Kopfrolle als Auflager, Massenpunkte für Säule und Geländer (die Stufenmassen gehören zu den Stufenbauteilen)."),
    ]
    return body, thermal, structure, dec


def scale500(M):
    body = [K.revolve("column", CO, ST, T_ST, [[0, K.BASE_R], [0.075, K.BASE_R], [0.075, K.R_HUB], [K.TOP + K.CAP_T, K.R_HUB]], [1, 0, 0], [0, 0, 0])]
    body += [wedge(k, 0.075, 0.0, 11.25) for k in range(1, 16)]
    body += balusters({"a": False, "b": False, "c": False}, True) + posts(False, True) + [rail(11.25, False, True)]
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [core_rod(K.R_HUB - 0.0005)]
    structure += [K.st_of("st-tread-%d" % k, "tread-%d" % k, tb(k)) for k in range(1, 16)]
    structure += [K.st_of("st-handrail", "handrail", BA), K.st_of("st-newel", "newel", BA), K.st_of("st-end-post", "end-post", BA)] + [K.st_of("st-baluster-%s" % t, "baluster-%s" % t, BA) for t in "abc"]
    structure += [K.st(K.revolve("support-base-fix", CO, ST, 0.15, [[0, K.BASE_R], [0.075, K.BASE_R]], [1, 0, 0], [0, 0, 0], role="detail"), CO, "support", "base-fix"), roller(0.0375)]
    structure += main_masses(M, 0.075, 0.075)
    dec = [
        K.text("At 1:500 (75 mm feature) the stair is the saw-tooth of the drawing: one column of 177.8 mm with a 75 mm foot for the base plate, fifteen solid tread sectors of 75 mm (the real pan is 70 mm, its 30 mm lap is below the feature and the sectors meet edge to edge), and the handrail as a slender 42.4 mm tube through sections every 11.25 degrees, carried by the newel, the end post and the 43 balusters, all at their true diameters as slender parts; the drawing shows them as the thinnest line (0.13 mm), so the infill reads as a band.",
               "Bei 1:500 (Merkmal 75 mm) ist die Treppe der Sägezahn der Zeichnung: eine Säule von 177,8 mm mit einem 75-mm-Fuß für die Fußplatte, fünfzehn volle Stufensektoren von 75 mm (die wirkliche Wanne ist 70 mm, ihr 30-mm-Übergriff liegt unter dem Merkmal, die Sektoren stoßen Kante an Kante) und der Handlauf als schlankes 42,4-mm-Rohr mit Schnitten alle 11,25 Grad, getragen vom Antritts- und vom Austrittspfosten und den 43 Stäben, alle in wahrem Durchmesser als schlanke Teile; die Zeichnung zeigt sie mit der dünnsten Linie (0,13 mm), sodass die Füllung als Band gelesen wird."),
        K.text("Thermal: column, treads, posts, balusters and handrail by reference; only the total area is compared at this scale.",
               "Thermik: Säule, Stufen, Pfosten, Stäbe und Handlauf per Verweis; verglichen wird in diesem Maßstab nur die Gesamtfläche."),
        K.text("Structure: the column member, fifteen tread members, handrail, posts and balusters, a 75 mm fixed disc at the foot and a roller at the head, mass points for column and balustrade; joints are below the feature and left out.",
               "Tragwerk: das Stützenbauteil, fünfzehn Stufenbauteile, Handlauf, Pfosten und Stäbe, eine feste 75-mm-Scheibe am Fuß und eine Rolle am Kopf, Massenpunkte für Säule und Geländer; Knoten liegen unter dem Merkmal und entfallen."),
    ]
    return body, thermal, structure, dec


def main():
    M = K.masses()
    vol = {}
    for sc, fn in (("50", scale50), ("100", scale100), ("200", scale200), ("500", scale500)):
        body, thermal, structure, dec = fn(M)
        vol[sc] = {"status": "authored", "denominator": int(sc), "body": body, "thermal": thermal, "structure": structure, "decisions": dec}
    with open(SRC, "w") as f:
        json.dump({"volumes": vol}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("[DEBUG] wrote", SRC, {k: [len(v[x]) for x in ("body", "thermal", "structure")] for k, v in vol.items()})


if __name__ == "__main__":
    main()
