import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_34_lib import *

A = math.radians(38.853374)
CA, SA = math.cos(A), math.sin(A)
STEP = 0.38523
CUT = 0.19866 / 0.16003
VT = -0.03115
DEPTH = 0.16003


def w2l(x, y):
    return [r6(x * CA + y * SA), r6(-x * SA + y * CA)]


def strip(v0, v1, end="A"):
    u0 = {"A": 0.03867, "B": 4.59768}[end]
    f = lambda v: u0 + (VT - v) * CUT
    return [f(v0), v0], [f(v1), v1]


def beam_part(v0, v1):
    a0, a1 = strip(v0, v1, "A")
    b0, b1 = strip(v0, v1, "B")
    return [[r6(a0[0]), r6(a0[1])], [r6(b0[0]), r6(b0[1])], [r6(b1[0]), r6(b1[1])], [r6(a1[0]), r6(a1[1])]]


TREAD = [[0.10315, 0.17587], [0.36014, -0.03115], [0.38523, 0], [0.12824, 0.20702]]
GUSSET = [[0.26669, 0.04413], [0.33678, -0.01233], [0.32161, -0.03115], [0.20605, -0.03115]]
RP = {"count": 12, "translation": [STEP, 0, 0]}


def chamfer(poly, idxs, c):
    out = []
    n = len(poly)
    for i, p_ in enumerate(poly):
        if i not in idxs:
            out.append([r6(p_[0]), r6(p_[1])])
            continue
        a, b = poly[i - 1], poly[(i + 1) % n]
        for q in (a, b):
            d = math.hypot(q[0] - p_[0], q[1] - p_[1])
            out.append([r6(p_[0] + (q[0] - p_[0]) * c / d), r6(p_[1] + (q[1] - p_[1]) * c / d)])
    return out


def skin(t):
    up = (SA, CA)
    p0, p1 = TREAD[3], TREAD[2]
    return [[r6(p0[0]), r6(p0[1])], [r6(p1[0]), r6(p1[1])], [r6(p1[0] - t * up[0]), r6(p1[1] - t * up[1])], [r6(p0[0] - t * up[0]), r6(p0[1] - t * up[1])]]


def common(s, tf, tw, plank_thick_check=None):
    s.b(ext("hea-top-flange", "stringer", "steel", 0.5, "xy", beam_part(VT, VT - tf), -0.08, 0.08))
    s.b(ext("hea-bottom-flange", "stringer", "steel", 0.5, "xy", beam_part(VT - DEPTH + tf, VT - DEPTH), -0.08, 0.08))
    s.b(ext("hea-web", "stringer", "steel", 0.5, "xy", beam_part(VT - tf + 0.001, VT - DEPTH + tf - 0.001), -tw / 2, tw / 2))


def s50():
    s = Scale(50)
    common(s, 0.009, 0.0075)
    g = ext("gusset", "stringer", "steel", 0.4, "xy", GUSSET, -0.03, 0.03, repeat=RP)
    s.b(g)
    s.b(ext("tread", "stringer", "timber", 0.7, "xy", chamfer(TREAD, {1, 2}, 0.0075), -0.6, 0.6, repeat=RP))
    up = [SA, CA, 0]
    for j, (sdist, lat) in enumerate([(0.05, -0.02), (0.05, 0.02), (0.28, -0.02), (0.28, 0.02)]):
        f = sdist / 0.33
        px = TREAD[3][0] + (TREAD[2][0] - TREAD[3][0]) * f
        py = TREAD[3][1] + (TREAD[2][1] - TREAD[3][1]) * f
        o = [px + up[0] * 0.001, py + up[1] * 0.001, lat]
        d = revolve("screw-%d" % (j + 1), "stringer", "steel", 0.3, [-up[0], -up[1], 0], o, [[0, 0.009], [0.0085, 0.009]], role="detail", repeat=RP)
        s.b(d)
    s.thof("th-hea-top-flange", "hea-top-flange", "stringer-steel")
    s.thof("th-hea-web", "hea-web", "stringer-steel")
    s.thof("th-hea-bottom-flange", "hea-bottom-flange", "stringer-steel")
    s.thof("th-gusset", "gusset", "stringer-steel")
    p = ext("th-tread-top", "stringer", "timber", 0.7, "xy", skin(0.0075), -0.6, 0.6, repeat=RP)
    p["sectionFaces"] = ["-v", "+u", "-u", "-w", "+w"]
    s.th(p, "tread-top")
    s.stof("st-hea-top-flange", "hea-top-flange", "stringer", "member")
    s.stof("st-hea-web", "hea-web", "stringer", "member")
    s.stof("st-hea-bottom-flange", "hea-bottom-flange", "stringer", "member")
    s.stof("st-gusset", "gusset", "stringer", "joint")
    s.stof("st-tread", "tread", "stringer", "member")
    wedge = [(-0.0, 0), (0.06, 0), (0.06, 0.0075), (0.04, 0.0075), (0.03, 0.04), (0.02, 0.0075), (0.0, 0.0075)]
    s.st(ext("support-foot-pin", "stringer", "steel", 0.15, "xy", [w2l(*q) for q in wedge], -0.05, 0.05, role="detail"), "stringer", "support")
    s.st(cyl("support-landing-roller", "stringer", "steel", 0.15, "z", w2l(3.62, 2.882) + [0], 0.1, 0.02, role="detail"), "stringer", "support")
    s.st(ellipsoid("mass-stringer", "stringer", "steel", 0.1, [0.06, 0.06, 0.06], at=[2.31, -0.11, 0], role="detail"), "stringer", "mass")
    s.dec('The 1:50 model is the flight as built: an HEA 160 centre stringer as three extrusions along the slope (two 9 mm flanges 160 mm wide and a web of 7.5 mm, the real 6 mm lifted to the feature) with horizontal end cuts that bear on the landings, twelve 60 mm wide gusset blocks welded to the top flange, twelve 1.2 m wide timber treads of 40 mm with 7.5 mm eased nosing edges and four countersunk steel screws per tread, flush with the tread top (heads 18 mm).', 'Das 1:50-Modell ist der Lauf wie gebaut: eine HEA-160-Mittelwange als drei Strangkörper entlang der Neigung (zwei Flansche von 9 mm und 160 mm Breite und ein Steg von 7,5 mm, die wirklichen 6 mm auf das Merkmal angehoben) mit waagrechten Endschnitten, die auf den Podesten auflagern, zwölf 60 mm breite Knaggenblöcke am Obergurt, zwölf 1,2 m breite Holzstufen von 40 mm mit 7,5 mm gebrochenen Vorderkanten und vier Senkkopfschrauben aus Stahl je Stufe, bündig mit der Stufenoberseite (Köpfe 18 mm).')
    s.dec('Thermal: stringer-steel is the outer surface of flanges, web and gussets by reference; tread-top is a 7.5 mm skin patch inside the top of each tread whose back, edge and end faces are declared section faces, so only the walking surface counts (the declared 4.75 m2 is the sum of the twelve tread tops, the undersides belong to no region).', 'Thermik: Wangenstahl ist die Außenfläche von Flanschen, Steg und Knaggen per Verweis; Stufenoberseite ist je Stufe ein 7,5 mm dickes Hautstück im oberen Teil der Stufe, dessen Rück-, Rand- und Stirnflächen als Schnittflächen erklärt sind, so zählt nur die Trittfläche (die angegebenen 4,75 m2 sind die Summe der zwölf Stufenoberseiten, die Unterseiten gehören zu keiner Region).')
    s.dec('Structure: flanges and web are the member (the HEA 160 section), the treads carry the load to twelve gussets that are the joints; a 60 mm pin wedge on a 7.5 mm pad at the foot anchor, a 40 mm roller at the landing anchor, and the stringer mass as one 120 mm point at mid-span.', 'Tragwerk: Flansche und Steg sind das Bauteil (der Querschnitt HEA 160), die Stufen tragen die Last in zwölf Knaggen, die die Knoten sind; ein 60-mm-Keil als Festlager auf einer 7,5-mm-Platte am Fußanker, eine 40-mm-Rolle am Podestanker und die Wangenmasse als ein 120-mm-Punkt in Feldmitte.')
    return s


def supports(s, pad, wedge_h, roller_r, mass_r, W=0.06):
    k = W / 0.06
    wedge = [(0.0, 0), (W, 0), (W, pad), (0.04 * k, pad), (0.03 * k, wedge_h), (0.02 * k, pad), (0.0, pad)]
    s.st(ext("support-foot-pin", "stringer", "steel", 0.15, "xy", [w2l(*q) for q in wedge], -0.05, 0.05, role="detail"), "stringer", "support")
    s.st(cyl("support-landing-roller", "stringer", "steel", 0.15, "z", w2l(3.6 + roller_r, 2.862 + roller_r) + [0], 0.1, roller_r, role="detail"), "stringer", "support")
    s.st(ellipsoid("mass-stringer", "stringer", "steel", 0.1, [mass_r] * 3, at=[2.31, -0.11, 0], role="detail"), "stringer", "mass")


BRACKET = [[0.26669, 0.04413], [0.36014, -0.03115], [0.20605, -0.03115]]
BEAM200 = [[0.09345, -0.07528], [0.23733, -0.19118], [4.63693, -0.19118], [4.73361, -0.26906], [4.79634, -0.19118], [4.59768, -0.03115]]
BOARD200 = [[0.0906, 0.16029], [0.34759, -0.04673], [0.38523, 0], [0.12824, 0.20702]]


def s100():
    s = Scale(100)
    common(s, 0.0152, 0.0152)
    s.b(ext("gusset", "stringer", "steel", 0.4, "xy", BRACKET, -0.03, 0.03, repeat=RP, role="detail"))
    s.b(ext("tread", "stringer", "timber", 0.7, "xy", TREAD, -0.6, 0.6, repeat=RP))
    for n_, o_ in (("hea-top-flange", "top-flange"), ("hea-web", "web"), ("hea-bottom-flange", "bottom-flange")):
        s.thof("th-" + n_, n_, "stringer-steel")
    s.thof("th-gusset", "gusset", "stringer-steel")
    p = ext("th-tread-top", "stringer", "timber", 0.7, "xy", skin(0.0152), -0.6, 0.6, repeat=RP)
    p["sectionFaces"] = ["-v", "+u", "-u", "-w", "+w"]
    s.th(p, "tread-top")
    for n_ in ("hea-top-flange", "hea-web", "hea-bottom-flange"):
        s.stof("st-" + n_, n_, "stringer", "member")
    s.stof("st-gusset", "gusset", "stringer", "joint")
    s.stof("st-tread", "tread", "stringer", "member")
    supports(s, 0.015, 0.05, 0.02, 0.0775)
    s.dec('At 1:100 the stringer keeps its three parts with 15.2 mm flanges and web (the 15 mm feature), the gussets are the drawn triangular brackets of 60 mm width, the treads are plain 40 mm timber boards 1.2 m wide; the screws are below the feature and left out, the eased nosing edges vanish.', 'Bei 1:100 behält die Wange ihre drei Teile mit 15,2 mm Flanschen und Steg (das 15-mm-Merkmal), die Knaggen sind die gezeichneten dreieckigen Konsolen von 60 mm Breite, die Stufen sind schlichte 40-mm-Holzbohlen von 1,2 m Breite; die Schrauben liegen unter dem Merkmal und entfallen, die gebrochenen Kanten verschwinden.')
    s.dec('Thermal: stringer parts and brackets by reference, tread-top as a 15.2 mm skin patch under the top face with the same section faces as at 1:50.', 'Thermik: Wangenteile und Konsolen per Verweis, Stufenoberseite als 15,2 mm dickes Hautstück unter der Oberfläche mit denselben Schnittflächen wie bei 1:50.')
    s.dec('Structure: the same member parts, brackets as joints and treads; a 15 mm pin pad with a 50 mm wedge, a 40 mm roller and a 155 mm mass point.', 'Tragwerk: dieselben Bauteile, Konsolen als Knoten und die Stufen; eine 15-mm-Auflagerplatte mit 50-mm-Keil, eine 40-mm-Rolle und ein 155-mm-Massenpunkt.')
    return s


def s200():
    s = Scale(200)
    s.b(ext("stringer", "stringer", "steel", 0.5, "xy", [[r6(a), r6(b)] for a, b in BEAM200], -0.08, 0.08))
    s.b(ext("tread", "stringer", "timber", 0.7, "xy", BOARD200, -0.6, 0.6, repeat=RP))
    s.thof("th-stringer", "stringer", "stringer-steel")
    p = ext("th-tread-top", "stringer", "timber", 0.7, "xy", skin(0.03), -0.6, 0.6, repeat=RP)
    p["sectionFaces"] = ["-v", "+u", "-u", "-w", "+w"]
    s.th(p, "tread-top")
    s.stof("st-stringer", "stringer", "stringer", "member")
    s.stof("st-tread", "tread", "stringer", "member")
    supports(s, 0.03, 0.06, 0.03, 0.0775)
    s.dec('At 1:200 the stringer is one extrusion of the drawn outline (flanges and web merged, 160 mm wide, with the bearing seat under the head), the treads are 60 mm boards (the drawn thickness, the real 40 mm is below the 30 mm feature once eased), and the gussets are merged into the beam and the treads.', 'Bei 1:200 ist die Wange ein Strangkörper der gezeichneten Kontur (Flansche und Steg zusammengefasst, 160 mm breit, mit der Auflagernase unter dem Kopf), die Stufen sind 60-mm-Bohlen (die gezeichnete Dicke), und die Knaggen sind in Wange und Stufen aufgegangen.')
    s.dec('Thermal: the beam by reference, tread-top as a 30 mm skin patch with the section faces of 1:50; the beam exposes less than the HEA (flange inner faces are lost), the total stays within 30 %.', 'Thermik: die Wange per Verweis, Stufenoberseite als 30 mm dickes Hautstück mit den Schnittflächen von 1:50; die Wange zeigt weniger Fläche als die HEA (die Flanschinnenseiten fehlen), die Summe bleibt im 30-%-Band.')
    s.dec('Structure: the merged beam and the treads as members, a 30 mm pin pad with a 60 mm wedge, a 60 mm roller and a 155 mm mass point; no joints at this scale.', 'Tragwerk: die zusammengefasste Wange und die Stufen als Bauteile, eine 30-mm-Auflagerplatte mit 60-mm-Keil, eine 60-mm-Rolle und ein 155-mm-Massenpunkt; keine Knoten in diesem Maßstab.')
    return s


def s500():
    s = Scale(500)
    s.b(ext("stringer", "stringer", "steel", 0.5, "xy", [[r6(a), r6(b)] for a, b in BEAM200 if (a, b) != (4.73361, -0.26906)], -0.08, 0.08))
    up = (SA, CA)
    top = (TREAD[3], TREAD[2])
    t75 = [[r6(top[0][0]), r6(top[0][1])], [r6(top[1][0]), r6(top[1][1])], [r6(top[1][0] - 0.075 * up[0]), r6(top[1][1] - 0.075 * up[1])], [r6(top[0][0] - 0.075 * up[0]), r6(top[0][1] - 0.075 * up[1])]]
    s.b(ext("tread", "stringer", "timber", 0.7, "xy", t75, -0.6, 0.6, repeat=RP))
    s.thof("th-stringer", "stringer", "stringer-steel")
    p = ext("th-tread-top", "stringer", "timber", 0.7, "xy", skin(0.076), -0.6, 0.6, repeat=RP)
    p["sectionFaces"] = ["-v", "+u", "-u", "-w", "+w"]
    s.th(p, "tread-top")
    s.stof("st-stringer", "stringer", "stringer", "member")
    s.stof("st-tread", "tread", "stringer", "member")
    supports(s, 0.076, 0.1, 0.04, 0.0775, W=0.12)
    s.dec('At 1:500 the stringer is one block of the drawn outline without the 75 mm bearing nose, and the treads are 75 mm boards (the smallest feature, drawn the same way in elevation and front); the 2D head bearing stroke is the beam end.', 'Bei 1:500 ist die Wange ein Block der gezeichneten Kontur ohne die 75-mm-Auflagernase, und die Stufen sind 75-mm-Bohlen (das kleinste Merkmal, in Seiten- und Vorderansicht gleich gezeichnet); der gezeichnete Kopflagerstrich ist das Wangenende.')
    s.dec('Thermal: the beam by reference, tread-top as a 76 mm skin patch with the section faces of 1:50; only the total is compared at this scale.', 'Thermik: die Wange per Verweis, Stufenoberseite als 76 mm dickes Hautstück mit den Schnittflächen von 1:50; in diesem Maßstab zählt nur die Summe.')
    s.dec('Structure: beam and treads as members, a 120 mm pin pad with a 100 mm wedge, an 80 mm roller and a 155 mm mass point.', 'Tragwerk: Wange und Stufen als Bauteile, eine 120-mm-Auflagerplatte mit 100-mm-Keil, eine 80-mm-Rolle und ein 155-mm-Massenpunkt.')
    return s


SCALES = {"50": s50, "100": s100, "200": s200, "500": s500}
if __name__ == "__main__":
    want = sys.argv[1:] or list(SCALES)
    save("components", "stair-flight", [SCALES[w]() for w in want])
