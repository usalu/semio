import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_34_lib import *

B = "column"


def rectp(v0, v1, w0, w1):
    return [[v0, w0], [v1, w0], [v1, w1], [v0, w1]]


def rectc(v0, v1, w0, w1, c):
    return [[v0 + c, w0], [v1 - c, w0], [v1, w0 + c], [v1, w1 - c], [v1 - c, w1], [v0 + c, w1], [v0, w1 - c], [v0, w0 + c]]


def arc(cv, cw, r, a0, a1, n):
    return [[cv + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cw + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))] for i in range(n + 1)]


def web_profile(tw, hv, rf, n=5):
    hw = tw / 2
    p = [[-hv, rf + hw], [-hv, -(rf + hw)]]
    p += arc(-hv + rf, -hw - rf, rf, 180, 90, n)
    p += arc(hv - rf, -hw - rf, rf, 90, 0, n)
    p += [[hv, rf + hw]]
    p += arc(hv - rf, hw + rf, rf, 0, -90, n)
    p += arc(-hv + rf, hw + rf, rf, -90, -180, n)
    out = []
    for a, b in p:
        q = [r6(a), r6(b)]
        if not out or max(abs(out[-1][0] - q[0]), abs(out[-1][1] - q[1])) > 1e-5:
            out.append(q)
    if max(abs(out[0][0] - out[-1][0]), abs(out[0][1] - out[-1][1])) < 1e-5:
        out.pop()
    return out


def hexagon(cv, cw, R):
    return [[r6(cv + R * math.cos(math.radians(60 * i))), r6(cw + R * math.sin(math.radians(60 * i)))] for i in range(6)]


def circ(cv, cw, r, n=16):
    return [[r6(cv + r * math.cos(2 * math.pi * i / n)), r6(cw + r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def flange_th(s, vin, u0, u1):
    a = ext("th-flange-a", B, "steel", 0.5, "yz", rectp(vin, 0.1, -0.1, 0.1), u0, u1)
    a["sectionFaces"] = ["-v"]
    b = ext("th-flange-b", B, "steel", 0.5, "yz", rectp(-0.1, -vin, -0.1, 0.1), u0, u1)
    b["sectionFaces"] = ["+v"]
    s.th(a, "column-flange")
    s.th(b, "column-flange")
    ia = ext("th-inner-a", B, "steel", 0.5, "yz", rectp(vin, 0.1, -0.1, 0.1), u0, u1)
    ia["sectionFaces"] = ["+v", "+w", "-w", "+u", "-u"]
    ib = ext("th-inner-b", B, "steel", 0.5, "yz", rectp(-0.1, -vin, -0.1, 0.1), u0, u1)
    ib["sectionFaces"] = ["-v", "+w", "-w", "+u", "-u"]
    s.th(ia, "column-web")
    s.th(ib, "column-web")


def support_mass(s, pad, mass_r, mass_u):
    s.st(box("support-footing", B, "steel", 0.15, [pad, 0.3, 0.26], at=[pad / 2, 0, 0], role="detail"), B, "support")
    s.st(ellipsoid("mass-column", B, "steel", 0.1, [mass_r] * 3, at=[mass_u, 0, 0], role="detail"), B, "mass")


def s50():
    s = Scale(50)
    s.b(ext("flange-a", B, "steel", 0.5, "yz", rectp(0.085, 0.1, -0.1, 0.1), 0.02, 2.98))
    s.b(ext("flange-b", B, "steel", 0.5, "yz", rectp(-0.1, -0.085, -0.1, 0.1), 0.02, 2.98))
    s.b(ext("web", B, "steel", 0.5, "yz", web_profile(0.009, 0.085, 0.018), 0.02, 2.98))
    holes = [circ(vv, ww, 0.008) for vv in (-0.125, 0.125) for ww in (-0.1, 0.1)]
    s.b(ext("base-plate", B, "steel", 0.4, "yz", rectc(-0.15, 0.15, -0.13, 0.13, 0.015), 0, 0.02, holes=holes))
    s.b(ext("stiffener-a", B, "steel", 0.4, "xy", [[0.02, 0.1], [0.02, 0.15], [0.11, 0.13], [0.11, 0.1]], -0.008, 0.008))
    s.b(ext("stiffener-b", B, "steel", 0.4, "xy", [[0.02, -0.1], [0.02, -0.15], [0.11, -0.13], [0.11, -0.1]], -0.008, 0.008))
    k = 0
    for vv in (-0.125, 0.125):
        for ww in (-0.1, 0.1):
            k += 1
            s.b(ext("anchor-nut-%d" % k, B, "steel", 0.3, "yz", hexagon(vv, ww, 0.012), 0.02, 0.038, role="detail"))
            s.b(cyl("anchor-rod-%d" % k, B, "steel", 0.3, "x", [0.0375, vv, ww], 0.075, 0.005, role="detail"))
    s.b(ext("cap-plate", B, "steel", 0.4, "yz", rectc(-0.12, 0.12, -0.11, 0.11, 0.015), 2.98, 3.0))
    flange_th(s, 0.085, 0.02, 2.98)
    s.thof("th-web", "web", "column-web")
    for i in ("base-plate", "stiffener-a", "stiffener-b") + tuple("anchor-nut-%d" % j for j in range(1, 5)) + tuple("anchor-rod-%d" % j for j in range(1, 5)):
        s.thof("th-" + i, i, "base-plate")
    s.thof("th-cap-plate", "cap-plate", "cap-plate")
    for i in ("flange-a", "flange-b", "web"):
        s.stof("st-" + i, i, B, "member")
    for i in ("base-plate", "stiffener-a", "stiffener-b", "cap-plate"):
        s.stof("st-" + i, i, B, "joint")
    for sg in (-1, 1):
        s.st(box("joint-splice-%s" % ("a" if sg > 0 else "b"), B, "steel", 0.3, [0.3, 0.12, 0.008], at=[1.5, 0, sg * 0.0085], role="detail"), B, "joint")
    s.st(box("joint-load-carrier", B, "steel", 0.3, [0.02, 0.04, 0.04], at=[2.99, 0, 0], role="detail"), "eccentric-carrier", "joint")
    support_mass(s, 0.008, 0.05, 1.45)
    s.dec("The 1:50 model is the HEB 200 column as built: two 15 mm flanges 200 mm wide and a 9 mm web with the 18 mm root fillets as separate extrusions along the 2.96 m shaft, a 20 mm base plate 300 x 260 mm with 15 mm clipped corners and four 16 mm anchor holes, two trapezoidal stiffeners of 16 mm, four hexagonal nuts (24 mm across corners) and 10 mm anchor rods, and a 20 mm head plate 240 x 220 mm with clipped corners. The splice at 1.5 m is not modelled in the body (the 2D draws only its line).", "Das 1:50-Modell ist die HEB-200-Stütze wie gebaut: zwei 15-mm-Flansche von 200 mm Breite und ein 9-mm-Steg mit den 18-mm-Ausrundungen als getrennte Strangkörper entlang des 2,96 m langen Schafts, eine 20-mm-Fußplatte 300 x 260 mm mit 15 mm abgeschnittenen Ecken und vier 16-mm-Ankerlöchern, zwei trapezförmige Rippen von 16 mm, vier Sechskantmuttern (24 mm über Ecken) und 10-mm-Ankerstangen sowie eine 20-mm-Kopfplatte 240 x 220 mm mit abgeschnittenen Ecken. Der Stoß bei 1,5 m ist im Körper nicht modelliert (die 2D zeichnet nur seine Linie).")
    s.dec("Thermal follows the declared split of the surface: column-flange is the outer face and the two edges of each flange (copies of the flange with the inner face declared a section face), column-web the web, its fillets and the inner flange faces (web by reference, two flange copies that keep only the inner face), base-plate the plate, stiffeners, nuts and rods by reference, cap-plate the head plate.", "Die Thermik folgt der angegebenen Flächenaufteilung: Flansch ist die Außenfläche und die beiden Kanten jedes Flanschs (Kopien des Flanschs, deren Innenseite als Schnittfläche erklärt ist), Steg ist der Steg mit Ausrundungen und den inneren Flanschflächen (Steg per Verweis, zwei Flanschkopien, die nur die Innenseite behalten), Fußplatte sind Platte, Rippen, Muttern und Stangen per Verweis, Kopfplatte die Kopfplatte.")
    s.dec("Structure: the two flanges and the web are the member (the HEB 200 section), plates and stiffeners are the joints at both ends, two 8 mm web splice plates at 1.5 m and a load pad in the head plate for the eccentric carrier; a fixed 8 mm footing pad under the base plate and the column mass as one 100 mm point.", "Tragwerk: die beiden Flansche und der Steg sind das Bauteil (der Querschnitt HEB 200), Platten und Rippen sind die Knoten an beiden Enden, zwei 8-mm-Stegstoßlaschen bei 1,5 m und ein Lastblock in der Kopfplatte für den außermittigen Träger; eine feste 8-mm-Fußauflage unter der Fußplatte und die Stützenmasse als ein 100-mm-Punkt.")
    return s


def s100():
    s = Scale(100)
    tf = 0.0152
    s.b(ext("flange-a", B, "steel", 0.5, "yz", rectp(0.1 - tf, 0.1, -0.1, 0.1), 0.035, 2.965))
    s.b(ext("flange-b", B, "steel", 0.5, "yz", rectp(-0.1, -0.1 + tf, -0.1, 0.1), 0.035, 2.965))
    s.b(ext("web", B, "steel", 0.5, "yz", rectp(-0.1 + tf - 0.001, 0.1 - tf + 0.001, -0.0076, 0.0076), 0.035, 2.965))
    s.b(ext("base-plate", B, "steel", 0.4, "yz", rectc(-0.15, 0.15, -0.13, 0.13, 0.015), 0, 0.035))
    s.b(ext("stiffener-a", B, "steel", 0.4, "xy", [[0.035, 0.1], [0.035, 0.15], [0.2, 0.13], [0.2, 0.1]], -0.01, 0.01))
    s.b(ext("stiffener-b", B, "steel", 0.4, "xy", [[0.035, -0.1], [0.035, -0.15], [0.2, -0.13], [0.2, -0.1]], -0.01, 0.01))
    s.b(ext("cap-plate", B, "steel", 0.4, "yz", rectc(-0.12, 0.12, -0.11, 0.11, 0.015), 2.965, 3.0))
    flange_th(s, 0.1 - tf, 0.035, 2.965)
    s.thof("th-web", "web", "column-web")
    for i in ("base-plate", "stiffener-a", "stiffener-b"):
        s.thof("th-" + i, i, "base-plate")
    s.thof("th-cap-plate", "cap-plate", "cap-plate")
    for i in ("flange-a", "flange-b", "web"):
        s.stof("st-" + i, i, B, "member")
    for i in ("base-plate", "stiffener-a", "stiffener-b", "cap-plate"):
        s.stof("st-" + i, i, B, "joint")
    for sg in (-1, 1):
        s.st(box("joint-splice-%s" % ("a" if sg > 0 else "b"), B, "steel", 0.3, [0.3, 0.12, 0.016], at=[1.5, 0, sg * 0.0116], role="detail"), B, "joint")
    s.st(box("joint-load-carrier", B, "steel", 0.3, [0.035, 0.04, 0.04], at=[2.9825, 0, 0], role="detail"), "eccentric-carrier", "joint")
    support_mass(s, 0.016, 0.0775, 1.45)
    s.dec("At 1:100 the shaft keeps its three parts, the flanges at their real 15.2 mm and the web raised from 9 to 15.2 mm (the feature) without fillets; the base and head plates are the drawn 35 mm blocks with 15 mm clipped corners, the stiffeners 20 mm thick trapezoids up to 0.2 m; anchor holes, nuts and rods are below the feature and omitted.", "Bei 1:100 behält der Schaft seine drei Teile, die Flansche mit den wirklichen 15,2 mm und der Steg von 9 auf 15,2 mm (das Merkmal) angehoben ohne Ausrundungen; Fuß- und Kopfplatte sind die gezeichneten 35-mm-Blöcke mit 15 mm abgeschnittenen Ecken, die Rippen 20 mm dicke Trapeze bis 0,2 m; Ankerlöcher, Muttern und Stangen liegen unter dem Merkmal und entfallen.")
    s.dec("Thermal: flange outer faces and edges as flange copies with the inner face declared a section face, the web and the inner flange faces as in the 1:50 split, plate and stiffeners by reference, head plate by reference.", "Thermik: Flanschaußenseiten und -kanten als Flanschkopien, deren Innenseite als Schnittfläche erklärt ist, Steg und innere Flanschflächen wie bei der 1:50-Aufteilung, Platte und Rippen sowie Kopfplatte per Verweis.")
    s.dec("Structure: the same member parts, the plates and stiffeners as joints, two 16 mm splice plates, a load pad in the head plate, a 16 mm footing pad and the mass as one 155 mm point.", "Tragwerk: dieselben Bauteile, die Platten und Rippen als Knoten, zwei 16-mm-Stoßlaschen, ein Lastblock in der Kopfplatte, eine 16-mm-Fußauflage und die Masse als ein 155-mm-Punkt.")
    return s


def s200():
    s = Scale(200)
    tf = 0.03
    prof = [[0.1, -0.1], [0.1, 0.1], [0.1 - tf, 0.1], [0.1 - tf, 0.015], [-0.1 + tf, 0.015], [-0.1 + tf, 0.1], [-0.1, 0.1], [-0.1, -0.1], [-0.1 + tf, -0.1], [-0.1 + tf, -0.015], [0.1 - tf, -0.015], [0.1 - tf, -0.1]]
    s.b(ext("column", B, "steel", 0.5, "yz", [[r6(a), r6(b)] for a, b in prof], 0.05, 2.95))
    s.b(ext("base-plate", B, "steel", 0.4, "yz", rectc(-0.15, 0.15, -0.13, 0.13, 0.015), 0, 0.05))
    s.b(ext("cap-plate", B, "steel", 0.4, "yz", rectc(-0.12, 0.12, -0.11, 0.11, 0.015), 2.95, 3.0))
    flange_th(s, 0.1 - tf, 0.05, 2.95)
    w = ext("th-web", B, "steel", 0.5, "yz", rectp(-0.1 + tf, 0.1 - tf, -0.015, 0.015), 0.05, 2.95)
    w["sectionFaces"] = ["+v", "-v", "+u", "-u"]
    s.th(w, "column-web")
    s.thof("th-base-plate", "base-plate", "base-plate")
    s.thof("th-cap-plate", "cap-plate", "cap-plate")
    s.stof("st-column", "column", B, "member")
    support_mass(s, 0.031, 0.0775, 1.45)
    s.dec("At 1:200 the shaft is one H extrusion with 30 mm flanges and a 30 mm web (the feature; the real 15 and 9 mm are below it), 0.05 m base and head plates as drawn with 15 mm clipped corners; the stiffeners and the splice are merged away.", "Bei 1:200 ist der Schaft ein H-Strangkörper mit 30-mm-Flanschen und 30-mm-Steg (das Merkmal; die wirklichen 15 und 9 mm liegen darunter), Fuß- und Kopfplatte wie gezeichnet 0,05 m mit 15 mm abgeschnittenen Ecken; Rippen und Stoß sind weggefallen.")
    s.dec("Thermal: flange and web as skins inside the merged shaft (flange copies keep the outer face and the edges, web copy keeps its two faces, two flange copies the inner faces), plates by reference; only the total is compared at this scale.", "Thermik: Flansch und Steg als Hautstücke im zusammengefassten Schaft (Flanschkopien behalten Außenseite und Kanten, die Stegkopie ihre zwei Flächen, zwei Flanschkopien die Innenseiten), Platten per Verweis; in diesem Maßstab zählt nur die Summe.")
    s.dec("Structure: the merged shaft is the member, a 31 mm footing pad and one 155 mm mass point; no joints at this scale.", "Tragwerk: der zusammengefasste Schaft ist das Bauteil, eine 31-mm-Fußauflage und ein 155-mm-Massenpunkt; keine Knoten in diesem Maßstab.")
    return s


def s500():
    s = Scale(500)
    tf = 0.075
    prof = [[0.1, -0.1], [0.1, 0.1], [0.1 - tf, 0.1], [0.1 - tf, tf / 2], [-0.1 + tf, tf / 2], [-0.1 + tf, 0.1], [-0.1, 0.1], [-0.1, -0.1], [-0.1 + tf, -0.1], [-0.1 + tf, -tf / 2], [0.1 - tf, -tf / 2], [0.1 - tf, -0.1]]
    s.b(ext("column", B, "steel", 0.5, "yz", [[r6(a), r6(b)] for a, b in prof], 0.075, 2.925))
    s.b(box("base-plate", B, "steel", 0.4, [0.075, 0.3, 0.26], at=[0.0375, 0, 0]))
    s.b(box("cap-plate", B, "steel", 0.4, [0.075, 0.24, 0.22], at=[2.9625, 0, 0]))
    s.thof("th-column", "column", "column-flange")
    w = ext("th-web", B, "steel", 0.5, "yz", rectp(-0.0375, 0.0375, -tf / 2, tf / 2), 0.075, 2.925)
    w["sectionFaces"] = ["+v", "-v", "+u", "-u"]
    s.th(w, "column-web")
    s.thof("th-base-plate", "base-plate", "base-plate")
    s.thof("th-cap-plate", "cap-plate", "cap-plate")
    s.stof("st-column", "column", B, "member")
    s.st(box("support-footing", B, "steel", 0.15, [0.076, 0.3, 0.26], at=[0.038, 0, 0], role="detail"), B, "support")
    s.st(box("mass-column", B, "steel", 0.1, [0.155, 0.155, 0.155], at=[1.45, 0, 0], role="detail"), B, "mass")
    s.dec("At 1:500 the shaft is still an H extrusion, now with 75 mm flanges and a 75 mm web (the smallest feature, the real 15 and 9 mm are far below it) so that its exposed surface stays near the real section; the plates are plain 75 mm blocks (the drawn step-outs are 20 mm, the clipped corners are below the feature) with the drawn 0.3 / 0.24 m widths.", "Bei 1:500 ist der Schaft weiter ein H-Strangkörper, nun mit 75-mm-Flanschen und 75-mm-Steg (das kleinste Merkmal, die wirklichen 15 und 9 mm liegen weit darunter), damit seine freie Fläche nahe am wirklichen Querschnitt bleibt; die Platten sind schlichte 75-mm-Blöcke (die gezeichneten Absätze sind 20 mm, die abgeschnittenen Ecken liegen unter dem Merkmal) mit den gezeichneten Breiten 0,3 / 0,24 m.")
    s.dec("Thermal: flange and web are merged in the H extrusion, so the column region keeps the whole shaft by reference (flange region) and the web region the 75 mm skin between the flanges (two side faces only), plates by reference; the split between flange and web area is no longer kept, only the total is compared at this scale.", "Thermik: Flansch und Steg sind im H-Strangkörper zusammengefasst, daher behält die Flanschregion den ganzen Schaft per Verweis und die Stegregion das 75 mm dicke Hautstück zwischen den Flanschen (nur zwei Seitenflächen), Platten per Verweis; die Aufteilung zwischen Flansch- und Stegfläche bleibt nicht erhalten, in diesem Maßstab zählt nur die Summe.")
    s.dec("Structure: the shaft is the member, a 76 mm footing pad and the column mass as one 155 mm cube.", "Tragwerk: der Schaft ist das Bauteil, eine 76-mm-Fußauflage und die Stützenmasse als ein 155-mm-Würfel.")
    return s


SCALES = {"50": s50, "100": s100, "200": s200, "500": s500}
if __name__ == "__main__":
    want = sys.argv[1:] or list(SCALES)
    save("components", "steel-column", [SCALES[w]() for w in want])
