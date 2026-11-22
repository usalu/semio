"""🪜 Rework 18 spiral stair (landing platform, column post, landing guards, slab fixing): the handcrafted solids of every scale (body, thermal, structure), stated part by part, written to the volumes source."""
import importlib.util
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "rework-18-spiral-stair-kit.py"))
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
    return K.prism("column-core", CO, ST, T_ST, K.circle(K.R_CORE, 48), "yz", K.BASE_T, K.POST_TOP)


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


def cap(r=K.R_CORE):
    return K.revolve("column-cap", CO, ST, T_ST, [[0, r], [K.CAP_T - 0.002, r], [K.CAP_T, r - 0.002], [K.CAP_T, 0]], [1, 0, 0], [K.POST_TOP, 0, 0])


def column(foot=None):
    """🏛️ The merged column of the coarser scales: the 177.8 mm sleeve stack up to the landing and the 168.3 mm core running on as the landing post."""
    lead = [[0, K.BASE_R], [foot, K.BASE_R], [foot, K.R_HUB]] if foot else [[K.BASE_T, K.R_HUB]]
    return K.revolve("column", CO, ST, T_ST, lead + [[K.TOP, K.R_HUB], [K.TOP, K.R_CORE], [K.POST_TOP + K.CAP_T, K.R_CORE]], [1, 0, 0], [0, 0, 0])


def landing_fix(detail, t):
    """🔩 The landing bolted to the edge of the upper slab: two packer plates 160 x 70 mm on the exit flange, at 1:50 each with its M12 bolt (head inside the pan, shank 60 mm into the slab)."""
    c = K.LAND_C
    step = K.land(K.FIX_R[1] - K.FIX_R[0], 0)
    r0 = K.FIX_R[0]
    plate = [K.land(r0 - 0.08, K.LAND), K.land(r0 + 0.08, K.LAND), K.land(r0 + 0.08, K.LAND + t), K.land(r0 - 0.08, K.LAND + t)]
    out = [K.prism("tread-fix-packer", tb(15), ST, T_DK, K.zx(plate), "zx", -max(K.PAN, t), 0, role="detail")]
    if detail:
        x, z = K.land(r0, K.LAND - K.PLATE_T - 0.008)
        out.append(K.revolve("tread-fix-bolt", tb(15), ST, T_DK, [[0, 0.0095], [0.008, 0.0095], [0.008, K.BOLT_R], [0.008 + K.PLATE_T + t + 0.06, K.BOLT_R]], [c, 0, c], [x, -K.PAN / 2, z], role="detail"))
    for o in out:
        o["repeat"] = {"count": 2, "translation": [K.r5(step[0]), 0, K.r5(step[1])]}
    return out


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


def landing_pan(wall):
    """⬛ The landing of 1:100 as a pan with walls of the feature size, open underneath like the real one."""
    outer = K.tread_outline(15, step=4.5, r_in=K.R_HUB - 0.002, inner=False)
    hole = K.tread_outline(15, inset=wall, r_in=K.R_HUB + wall, step=4.5)
    return [K.prism("tread-15", tb(15), ST, T_ST, K.zx(outer), "zx", -wall, 0), K.prism("tread-15-flange", tb(15), ST, T_ST, K.zx(outer), "zx", -K.PAN, -wall, holes=[K.zx(hole)])]


def wedge(k, thick, lap, step, i=None):
    return K.prism(i or "tread-%d" % k, tb(k), ST, T_ST, K.zx(K.tread_outline(k, lap=lap, r_in=K.R_HUB - 0.002, step=step, inner=False)), "zx", -thick, 0)


GAUGE = {
    "50": {"bal": K.BAL_R, "tube": K.RAIL_R, "coarse": False},
    "100": {"bal": 0.0075, "tube": K.RAIL_R, "coarse": False},
    "200": {"bal": 0.015, "tube": K.RAIL_R, "coarse": False},
    "500": {"bal": 0.0375, "tube": 0.0375, "coarse": False},
}
G = dict(GAUGE["50"])


def balusters(feet, slender):
    """🦯 The balusters at the gauge of the scale: 12 mm as built at 1:50, 15 mm at 1:100, 30 mm at 1:200, 75 mm at 1:500 (where the 101 mm centres leave 26 mm and the infill reads as a band)."""
    out = []
    if G["bal"] is None:
        return out
    for tag, d in zip("abc", K.BAL_OFF):
        if G["coarse"] and tag != "b":
            continue
        ks = [k for _, k, p in K.baluster_feet() if (k, tag) == (k, tag) and _.endswith(tag)]
        if not ks or tag not in feet:
            continue
        k0 = ks[0]
        p = K.phi_mid(k0) + d
        x, z = K.polar(K.R_RAIL, p)
        L = K.rail_y(p) - K.RISE * k0
        prof = [[0, 0.011], [0.004, 0.011], [0.009, K.BAL_R], [L, K.BAL_R]] if feet[tag] else [[0, G["bal"]], [L, G["bal"]]]
        s = K.revolve("baluster-%s" % tag, BA, SB, T_SB, prof, [0, 1, 0], [x, K.RISE * k0, z], role="detail")
        if slender:
            s["slender"] = True
        out.append(K.spin(s, len(ks), K.STEP, K.RISE))
    L = K.RAIL_LEVEL - K.TOP
    n_out, n_in = K.land_counts(G["coarse"])
    for tag, r, t1, t2, n in (("o", K.R_RAIL, K.outer_t(1, n_out), K.outer_t(2, n_out), n_out), ("i", K.INNER_R, K.inner_t(1, n_in), K.inner_t(2, n_in), n_in)):
        x, z = K.land(r, t1)
        dx, dz = K.land(0, t2 - t1)
        prof = [[0, 0.011], [0.004, 0.011], [0.009, K.BAL_R], [L, K.BAL_R]] if feet["b"] else [[0, G["bal"]], [L, G["bal"]]]
        s = K.revolve("baluster-%s" % tag, BA, SB, T_SB, prof, [0, 1, 0], [x, K.TOP, z], role="detail")
        if slender:
            s["slender"] = True
        s["repeat"] = {"count": n, "translation": [K.r5(dx), 0, K.r5(dz)]}
        out.append(s)
    return out


def true_balusters():
    """🌡️ The balusters of the 1:500 thermal list at their real 12 mm as slender rods: the 75 mm rods of the body would count six times their surface."""
    out = []
    for s in balusters({"a": False, "b": False, "c": False}, False):
        t = dict(s, id="th-" + s["id"], region=B, slender=True)
        t["profile"] = [[a, K.BAL_R] for a, _ in s["profile"]]
        out.append(t)
    return out


def posts(flange, slender):
    out = []
    for name, x, z, y0, y1 in K.posts():
        L = y1 - y0
        lo = K.FLANGE_T if flange else 0.0
        s = K.revolve(name, BA, SB, T_SB, [[lo, G["tube"]], [L, G["tube"]]], [0, 1, 0], [x, y0, z], role="detail")
        if slender:
            s["slender"] = True
        out.append(s)
        if flange and name == "end-post":
            ix, _, iz = K.inner_rail()[-1]
            out.append(K.revolve("end-post-inner-flange", BA, SB, T_SB, [[0, K.FLANGE_R - 0.002], [0.002, K.FLANGE_R], [K.FLANGE_T, K.FLANGE_R], [K.FLANGE_T + 0.004, K.POST_R]], [0, 1, 0], [ix, y0, iz], role="detail"))
        if flange:
            out.append(K.revolve(name + "-flange", BA, SB, T_SB, [[0, K.FLANGE_R - 0.002], [0.002, K.FLANGE_R], [K.FLANGE_T, K.FLANGE_R], [K.FLANGE_T + 0.004, K.POST_R]], [0, 1, 0], [x, y0, z], role="detail"))
    return out


def rail(step, dome, slender):
    f = G["tube"] / K.RAIL_R
    s = K.loft("handrail", BA, SB, T_SB, [(p, r * f) for p, r in K.rail_path(step=step, dome=dome)], role="detail")
    inner = K.loft("handrail-inner", BA, SB, T_SB, [(p, G["tube"]) for p in K.inner_rail(K.FLANGE_T if dome else 0.0)], role="detail")
    if slender:
        s["slender"] = True
        inner["slender"] = True
    return [s, inner]


def slab_fix():
    return K.st_of("support-landing-fix", "tread-fix-packer", tb(15), "support", "top-tie")


RAILS = [("st-handrail", "handrail"), ("st-handrail-inner", "handrail-inner"), ("st-newel", "newel"), ("st-newel-turn", "newel-turn"), ("st-end-post", "end-post")]
FLANGES = ["newel", "newel-turn", "end-post", "end-post-inner"]


def rail_members():
    tags = "" if G["bal"] is None else ("boi" if G["coarse"] else "abcoi")
    return [K.st_of(i, of, BA) for i, of in RAILS + [("st-baluster-%s" % t, "baluster-%s" % t) for t in tags]]


def tread_masses(M, r):
    return [K.st(K.ellipsoid("mass-tread-%d" % k, tb(k), ST, 0.1, [r, r, r], [M["treads"][k]["x"], M["treads"][k]["v"], M["treads"][k]["z"]]), tb(k), "mass") for k in range(1, 16)]


def main_masses(M, rc, rb):
    col_kg, col_com = M["column"]
    _, bcom = M["balustrade"]
    return [K.st(K.ellipsoid("mass-column", CO, ST, 0.1, [rc, rc, rc], [col_com, 0, 0]), CO, "mass"),
            K.st(K.ellipsoid("mass-balustrade", BA, ST, 0.1, [rb, rb, rb], bcom), BA, "mass")]


def core_rod(r, i="st-column-core"):
    return K.st(K.revolve(i, CO, ST, 0.4, [[K.BASE_T, r], [K.POST_TOP, r]], [1, 0, 0], [0, 0, 0]), CO)


def collar(k, wall, depth):
    return K.st(K.revolve("joint-hub-%d" % k, tb(k), ST, 0.3, [[-depth, K.R_HUB - wall], [0, K.R_HUB - wall], [0, K.R_HUB], [-depth, K.R_HUB], [-depth, K.R_HUB - wall]], [0, 1, 0], [0, 0, 0], role="detail"), tb(k), "joint")


ALL = {"a": True, "b": True, "c": True}


def scale50(M):
    body = [column_core(), base_plate(True)] + anchors() + [cap()]
    for k in range(1, 16):
        body.append(hub(k))
        body.extend(pan(k))
    body += landing_fix(True, K.PACK_T)
    body += balusters(ALL, True) + posts(True, True) + rail(3.75, True, True)
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [K.st_of("st-column-core", "column-core", CO)]
    for k in range(1, 16):
        structure += [K.st_of("st-tread-plate-%d" % k, "tread-plate-%d" % k, tb(k)), K.st_of("st-tread-flange-%d" % k, "tread-flange-%d" % k, tb(k)), K.st_of("joint-hub-%d" % k, "hub-%d" % k, tb(k), "joint")]
    structure += rail_members()
    structure += [K.st_of("joint-%s-flange" % n, "%s-flange" % n, BA, "joint") for n in FLANGES] + [K.st_of("joint-landing-bolt", "tread-fix-bolt", tb(15), "joint")]
    structure += [K.st_of("support-base-fix", "base-plate", CO, "support", "base-fix"), slab_fix()]
    structure += main_masses(M, 0.04, 0.012) + tread_masses(M, 0.02)
    dec = [
        K.text("The 1:50 model is the spiral stair as a manufacturer builds it: a CHS 168.3 x 6.3 core column (its bore is a sealed cavity and is modelled filled, so the closed tube is not counted as open) on a 400 x 20 mm base plate with four M16 anchors (washer, nut), running on above the landing as its 0.93 m post and closed by an 8 mm cap; fifteen units stacked on the column, each a 177.8 x 4.0 hub sleeve one riser (0.187 m) long welded under a pressed pan of 5 mm plate with a 70 mm deep folded flange, its nosing edge lapping 30 mm over the tread below (DIN 18065, open risers), the pan open underneath; 52 balusters of 12 mm with welded feet, three per tread (89 mm clear, MBO section 38), a newel post on the first tread, a turn post and two end posts on the landing, all 42.4 mm on a 70 mm flange, and the 42.4 mm handrail swept along the true helix 1.0 m above the tread centre (0.91 m over every nosing) with domed ends beyond the posts.",
               "Das 1:50-Modell ist die Wendeltreppe, wie sie ein Hersteller baut: eine Kernsäule CHS 168,3 x 6,3 (ihre Bohrung ist ein geschlossener Hohlraum und wird gefüllt modelliert, damit das geschlossene Rohr nicht als offen zählt) auf einer Fußplatte 400 x 20 mm mit vier M16-Ankern (Scheibe, Mutter), über dem Podest als dessen 0,93 m hoher Pfosten weitergeführt und mit 8 mm Kopfplatte geschlossen; fünfzehn auf die Säule gesteckte Einheiten, jede eine Nabenhülse 177,8 x 4,0 von einer Steigung (0,187 m) Länge unter einer gekanteten Wanne aus 5-mm-Blech mit 70 mm tiefem Bord, deren Vorderkante 30 mm über die untere Stufe greift (DIN 18065, offene Stufen), unten offen; 52 Stäbe 12 mm mit angeschweißtem Fuß, drei je Stufe (89 mm licht, MBO § 38), ein Antrittspfosten auf der ersten Stufe, ein Wendepfosten und zwei Austrittspfosten auf dem Podest, alle 42,4 mm auf 70-mm-Flansch, und der 42,4-mm-Handlauf entlang der wahren Wendel 1,0 m über der Stufenmitte (0,91 m über jeder Vorderkante) mit gewölbten Enden hinter den Pfosten."),
        K.text("Thermal: every region is the outer surface of its body solids by reference. treads = plate and flange of the fifteen pans (top, the open underside and both faces of the flange); column-steel = hub sleeves, cap, base plate and anchors (the core bore is a sealed cavity and the core face lies under the sleeves); balustrade-steel = handrail, posts with flanges and balusters. One steel layer per region, so no nested shells.",
               "Thermik: Jede Region ist die Außenfläche ihrer Körper per Verweis. treads = Platte und Bord der fünfzehn Wannen (Oberseite, offene Unterseite und beide Bordseiten); column-steel = Nabenhülsen, Kopfplatte, Fußplatte und Anker (die Kernbohrung ist ein geschlossener Hohlraum, die Kernoberfläche liegt unter den Hülsen); balustrade-steel = Handlauf, Pfosten mit Flanschen und Stäbe. Eine Stahlschicht je Region, daher keine verschachtelten Schalen."),
        K.text("Structure: the core tube is the column member; each pan (plate and flange as one folded section) is the cantilever member of its tread and its hub sleeve is the joint that clamps it to the column; handrail, posts and balusters are the balustrade members with the post flanges as joints; the base plate with its anchors is the fixed support, the two packer plates on the exit flange of the landing, bolted to the edge of the upper slab, the horizontal tie; mass points sit at the centres of mass of the column, of every tread unit and of the balustrade.",
               "Tragwerk: Das Kernrohr ist das Stützenbauteil; jede Wanne (Platte und Bord als ein gekanteter Querschnitt) ist der Kragarm ihrer Stufe, ihre Nabenhülse der Knoten, der sie an der Säule hält; Handlauf, Pfosten und Stäbe sind die Geländerbauteile mit den Pfostenflanschen als Knoten; die Fußplatte mit Ankern ist das feste Auflager, die zwei Futterplatten am Austrittsbord des Podests, an die Stirn der oberen Decke geschraubt, die waagrechte Halterung; Massenpunkte liegen in den Schwerpunkten der Säule, jeder Stufeneinheit und des Geländers."),
    ]
    return body, thermal, structure, dec


def scale100(M):
    body = [column(), base_plate(False)]
    body += [wedge(k, K.PAN, K.LAP, 4.5) for k in range(1, 15)] + landing_pan(0.015) + landing_fix(False, 0.015)
    body += balusters({"a": False, "b": False, "c": False}, True) + posts(False, True) + rail(7.5, True, True)
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [core_rod(K.R_CORE)]
    for k in range(1, 16):
        structure += [K.st_of("st-tread-%d" % k, "tread-%d" % k, tb(k)), collar(k, 0.015, K.PAN)]
    structure.append(K.st_of("st-tread-15-flange", "tread-15-flange", tb(15)))
    structure += rail_members()
    structure += [K.st_of("support-base-fix", "base-plate", CO, "support", "base-fix"), slab_fix()]
    structure += main_masses(M, 0.05, 0.02) + tread_masses(M, 0.025)
    dec = [
        K.text("At 1:100 (15 mm feature) the core, the fifteen hub sleeves and the cap merge into one 177.8 mm column on the 20 mm base plate (anchors below the feature are left out); each pan becomes one solid 70 mm tread with its 30 mm nosing lap, because its 5 mm walls and the open underside are below what the scale shows; the 52 balusters are rods of the 15 mm feature (real 12 mm) without feet, the posts their 42.4 mm without flanges, and the handrail follows the helix through sections every 7.5 degrees.",
               "Bei 1:100 (Merkmal 15 mm) verschmelzen Kern, die fünfzehn Nabenhülsen und die Kopfplatte zu einer Säule von 177,8 mm auf der 20-mm-Fußplatte (die Anker liegen unter dem Merkmal und entfallen); jede Wanne wird eine volle 70-mm-Stufe mit ihrem 30-mm-Übergriff, weil ihre 5-mm-Wände und die offene Unterseite unter dem liegen, was der Maßstab zeigt; die 52 Stäbe sind Stäbe im Merkmal 15 mm (wahr 12 mm) ohne Fuß, die Pfosten ihre 42,4 mm ohne Flansch, und der Handlauf folgt der Wendel mit Schnitten alle 7,5 Grad."),
        K.text("Thermal: by reference as at 1:50; the treads region is now the outside of fifteen solid treads (the inside of the pans is merged), the column region the outside of the column and plate.",
               "Thermik: per Verweis wie bei 1:50; die Stufenregion ist nun die Außenseite von fünfzehn vollen Stufen (das Wanneninnere ist zusammengefasst), die Säulenregion die Außenseite von Säule und Platte."),
        K.text("Structure: the 168.3 mm core as the column member inside the merged column, the fifteen treads as members with a 15 mm hub collar as the joint at the column, handrail, posts and balusters as members, base plate and the landing fixing to the slab edge as supports, mass points for column, treads and balustrade.",
               "Tragwerk: der 168,3-mm-Kern als Stützenbauteil in der zusammengefassten Säule, die fünfzehn Stufen als Bauteile mit einem 15-mm-Nabenring als Knoten an der Säule, Handlauf, Pfosten und Stäbe als Bauteile, Fußplatte und Podestbefestigung an der Deckenstirn als Auflager, Massenpunkte für Säule, Stufen und Geländer."),
    ]
    return body, thermal, structure, dec


def scale200(M):
    body = [column(), K.revolve("base-plate", CO, ST, T_ST, [[0, K.BASE_R], [K.BASE_T, K.BASE_R]], [1, 0, 0], [0, 0, 0])]
    body += [wedge(k, K.PAN, K.LAP, 7.5) for k in range(1, 16)] + landing_fix(False, 0.03)
    body += balusters({"a": False, "b": False, "c": False}, True) + posts(False, True) + rail(7.5, False, True)
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    structure = [core_rod(K.R_CORE)]
    for k in range(1, 16):
        structure += [K.st_of("st-tread-%d" % k, "tread-%d" % k, tb(k)), collar(k, 0.03, K.PAN)]
    structure += rail_members()
    structure += [K.st_of("support-base-fix", "base-plate", CO, "support", "base-fix"), slab_fix()]
    structure += main_masses(M, 0.06, 0.035)
    dec = [
        K.text("At 1:200 (30 mm feature) the stair is the column of 177.8 mm on its plate, fifteen solid 70 mm treads with the 30 mm nosing lap in coarser arcs, the handrail through sections every 7.5 degrees without domed ends, the posts at their true 42.4 mm, and all 52 balusters as rods of the 30 mm feature (real 12 mm, 71 mm clear between them); the drawing shows them as 0.15 mm lines.",
               "Bei 1:200 (Merkmal 30 mm) ist die Treppe die Säule von 177,8 mm auf ihrer Platte, fünfzehn volle 70-mm-Stufen mit dem 30-mm-Übergriff in gröberen Bögen, der Handlauf mit Schnitten alle 7,5 Grad ohne gewölbte Enden, die Pfosten in ihren wahren 42,4 mm und alle 52 Stäbe als Stäbe im Merkmal 30 mm (wahr 12 mm, 71 mm licht dazwischen); die Zeichnung zeigt sie als 0,15-mm-Linien."),
        K.text("Thermal: by reference to the body solids; at this scale only the total area of all regions is compared.",
               "Thermik: per Verweis auf die Körper; in diesem Maßstab wird nur die Gesamtfläche aller Regionen verglichen."),
        K.text("Structure: core member, fifteen tread members with a 30 mm hub collar as joint, handrail, posts and balusters as members, plate and the landing fixing to the slab edge as supports, mass points for column and balustrade (the tread masses are part of the tread members).",
               "Tragwerk: Kernbauteil, fünfzehn Stufenbauteile mit einem 30-mm-Nabenring als Knoten, Handlauf, Pfosten und Stäbe als Bauteile, Platte und Podestbefestigung an der Deckenstirn als Auflager, Massenpunkte für Säule und Geländer (die Stufenmassen gehören zu den Stufenbauteilen)."),
    ]
    return body, thermal, structure, dec


def scale500(M):
    body = [column(0.075)]
    body += [wedge(k, 0.075, 0.0, 11.25) for k in range(1, 16)] + landing_fix(False, 0.075)
    body += balusters({"a": False, "b": False, "c": False}, True) + posts(False, True) + rail(11.25, False, True)
    ids = [s["id"] for s in body]
    region = lambda i: R if i.startswith("tread-") else (B if i.split("-")[0] in ("baluster", "newel", "end", "handrail") else P)
    thermal = [K.th_of("th-" + i, i, region(i)) for i in ids]
    thermal = [t for t in thermal if not t["of"].startswith("baluster-")] + true_balusters()
    structure = [core_rod(K.R_HUB - 0.0005)]
    structure += [K.st_of("st-tread-%d" % k, "tread-%d" % k, tb(k)) for k in range(1, 16)]
    structure += rail_members()
    structure += [K.st(K.revolve("support-base-fix", CO, ST, 0.15, [[0, K.BASE_R], [0.075, K.BASE_R]], [1, 0, 0], [0, 0, 0], role="detail"), CO, "support", "base-fix"), slab_fix()]
    structure += main_masses(M, 0.075, 0.075)
    dec = [
        K.text("At 1:500 (75 mm feature) the stair is the saw-tooth of the drawing: one column of 177.8 mm with a 75 mm foot for the base plate, fifteen solid tread sectors of 75 mm (the real pan is 70 mm, its 30 mm lap is below the feature and the sectors meet edge to edge), and the handrail as a tube of the 75 mm feature (real 42.4 mm) through sections every 11.25 degrees, carried by the newel, the turn post and the end posts at the same 75 mm; the infill (12 mm at 101 mm centres, 0.2 mm on paper) is kept as 52 rods of the 75 mm feature that leave 26 mm between them and read as the infill band (dropping any of them moves the outline by more than two features against 1:200), and the drawing shows rail, posts and balusters as 0.15 mm lines.",
               "Bei 1:500 (Merkmal 75 mm) ist die Treppe der Sägezahn der Zeichnung: eine Säule von 177,8 mm mit einem 75-mm-Fuß für die Fußplatte, fünfzehn volle Stufensektoren von 75 mm (die wirkliche Wanne ist 70 mm, ihr 30-mm-Übergriff liegt unter dem Merkmal, die Sektoren stoßen Kante an Kante) und der Handlauf als Rohr im Merkmal 75 mm (wahr 42,4 mm) mit Schnitten alle 11,25 Grad, getragen vom Antritts-, vom Wende- und von den Austrittspfosten in denselben 75 mm; die Füllung (12 mm im Abstand von 101 mm, 0,2 mm auf dem Papier) bleibt als 52 Stäbe im Merkmal 75 mm, die 26 mm zwischen sich lassen und als Füllband gelesen werden (das Weglassen einzelner verschiebt den Umriss gegenüber 1:200 um mehr als zwei Merkmale), die Zeichnung zeigt Handlauf, Pfosten und Stäbe als 0,15-mm-Linien."),
        K.text("Thermal: column, treads, posts and handrail by reference; the balusters are their own slender 12 mm rods, because the 75 mm rods of the body would count six times their real surface; only the total area is compared at this scale.",
               "Thermik: Säule, Stufen, Pfosten und Handlauf per Verweis; die Stäbe sind eigene schlanke 12-mm-Stäbe, weil die 75-mm-Stäbe des Körpers das Sechsfache ihrer wahren Oberfläche zählen würden; verglichen wird in diesem Maßstab nur die Gesamtfläche."),
        K.text("Structure: the column member, fifteen tread members, handrail, posts and balusters, a 75 mm fixed disc at the foot and the landing fixing at the slab edge, mass points for column and balustrade; joints are below the feature and left out.",
               "Tragwerk: das Stützenbauteil, fünfzehn Stufenbauteile, Handlauf, Pfosten und Stäbe, eine feste 75-mm-Scheibe am Fuß und die Podestbefestigung an der Deckenstirn, Massenpunkte für Säule und Geländer; Knoten liegen unter dem Merkmal und entfallen."),
    ]
    return body, thermal, structure, dec


LANDING = {
    "50": ("Unit 15 is the landing: a pressed pan 0.8 x 0.8 m between the rays at 315 and 45 degrees, level with the upper floor (2.80 m), its exit flange bolted through two 160 x 70 x 8 mm packers with M12 bolts into the slab edge; the handrail leaves the helix at the turn post and runs level, 0.907 m above the landing, along the outer edge to its end post, and a second level rail runs from the column post to the exit and bends down there as its own post, five and six 12 mm balusters under them (99 and 102 mm clear).",
           "Einheit 15 ist das Podest: eine gekantete Wanne 0,8 x 0,8 m zwischen den Strahlen bei 315 und 45 Grad, bündig mit der oberen Decke (2,80 m), ihr Austrittsbord über zwei Futterplatten 160 x 70 x 8 mm mit M12-Schrauben in die Deckenstirn geschraubt; der Handlauf verlässt die Wendel am Wendepfosten und läuft waagrecht, 0,907 m über dem Podest, an der Außenkante bis zu seinem Austrittspfosten, ein zweiter waagrechter Handlauf läuft vom Säulenpfosten zum Austritt und biegt dort als eigener Pfosten nach unten, darunter fünf und sechs 12-mm-Stäbe (99 und 102 mm licht)."),
    "100": ("Unit 15 is the 0.8 x 0.8 m landing, a pan with 15 mm walls open underneath (its hollow is larger than the feature), with two 15 mm packer blocks for the slab fixing (the bolts are below the feature); the column runs on as the 168.3 mm landing post, and both level landing rails with their posts and eleven balusters are kept.",
            "Einheit 15 ist das Podest 0,8 x 0,8 m, eine unten offene Wanne mit 15-mm-Wänden (ihr Hohlraum ist größer als das Merkmal), mit zwei 15-mm-Futterklötzen für die Deckenbefestigung (die Schrauben liegen unter dem Merkmal); die Säule läuft als Podestpfosten von 168,3 mm weiter, beide waagrechten Podesthandläufe mit ihren Pfosten und elf Stäben bleiben."),
    "200": ("Unit 15 is the 0.8 x 0.8 m landing with two 30 mm blocks for the slab fixing, the column runs on as the landing post, and both level landing rails with posts and balusters are kept as slender parts.",
            "Einheit 15 ist das Podest 0,8 x 0,8 m mit zwei 30-mm-Klötzen für die Deckenbefestigung, die Säule läuft als Podestpfosten weiter, beide waagrechten Podesthandläufe mit Pfosten und Stäben bleiben als schlanke Teile."),
    "500": ("Unit 15 is the 0.8 x 0.8 m landing square with two 75 mm blocks where it is fixed to the slab edge; the column runs on as the landing post and carries the inner landing rail.",
            "Einheit 15 ist das Podestquadrat 0,8 x 0,8 m mit zwei 75-mm-Klötzen an der Befestigung zur Deckenstirn; die Säule läuft als Podestpfosten weiter und trägt den inneren Podesthandlauf."),
}


def main():
    M = K.masses()
    vol = {}
    for sc, fn in (("50", scale50), ("100", scale100), ("200", scale200), ("500", scale500)):
        G.update(GAUGE[sc])
        body, thermal, structure, dec = fn(M)
        for s in body:
            s.pop("slender", None)
        dec[0] = K.text(dec[0]["en"] + " " + LANDING[sc][0], dec[0]["de"] + " " + LANDING[sc][1])
        vol[sc] = {"status": "authored", "denominator": int(sc), "body": body, "thermal": thermal, "structure": structure, "decisions": dec}
    with open(SRC, "w") as f:
        json.dump({"volumes": vol}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("[DEBUG] wrote", SRC, {k: [len(v[x]) for x in ("body", "thermal", "structure")] for k, v in vol.items()})


if __name__ == "__main__":
    main()
