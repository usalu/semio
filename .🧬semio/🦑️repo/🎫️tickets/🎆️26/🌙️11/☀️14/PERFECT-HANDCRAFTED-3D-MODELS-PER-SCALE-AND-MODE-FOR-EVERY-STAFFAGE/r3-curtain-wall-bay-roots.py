"""🏢 Round 3 curtain-wall bay: the root fixes outside `volumes` (summary, references, depth and mass, materials, frame section and masses, the four supports, load anchors, the 2D drawings of every scale with their decisions, seismic loads, thermal layer thickness and frame areas, structural assumptions), applied to the catalog asset.

Usage: python3 r3-curtain-wall-bay-roots.py                  writes the candidate document (with the volumes source) to 🗑️generated/r3-curtain-wall-bay/🔣️.json
       python3 r3-curtain-wall-bay-roots.py --apply          writes the changed members into the catalog file (re-read right before, only these members change)
       --area=<region>:<m2>                                  sets the areaM2 of a frame region (measured on the 1:50 model)
"""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("kit", "r3-curtain-wall-bay-kit.py")
V = load("vol", "r3-curtain-wall-bay-volumes.py")
L = load("layout", "r3-curtain-wall-bay-layout.py")
text, r5 = K.text, K.r5

MEMBERS = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "poses", "thermal", "structure"]
AREAS = {"mullion": 0.477, "transom": 0.285}
FR, IG, SP = "frame", "igu", "spandrel"
MO, MC = K.tube(K.Z_MULLION_BACK)
TO, TC = K.tube(K.Z_TRANSOM_BACK)
A_M, CY_M, I_M, C_M = K.section([MO, MC])
G = 9.81


def shape(i, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, pen=None):
    out = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if pen:
        out["penMm"] = pen
    out.update(geometry)
    return out


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def line(x0, y0, x1, y1):
    return poly([[x0, y0], [x1, y1]], False)


def elevation(scale):
    """🏢 The face of the bay seen from outside: caps of mullions and transoms over the glass and the spandrel; identical at 1:50 to 1:200 apart from pens and reflections, merged at 1:500."""
    if scale == 500:
        return [
            shape("ground", FR, "ground", 0, False, 0.5, line(0, 0, K.W, 0)),
            shape("spandrel", SP, "body", 1, True, 0.65, rect(-0.05, -0.05, 1.35, 0.8), regions=["spandrel"], material="spandrel-glass"),
            shape("vision", IG, "body", 1, True, 0.15, rect(-0.05, -0.05, 1.35, 2.4), regions=["glass-centre", "glass-edge", "mullion", "transom"], material="glass"),
        ]
    member_role = "detail" if scale == 200 else "body"
    out = [
        shape("ground", FR, "ground", 0, False, 0.5, line(0, 0, K.W, 0)),
        shape("spandrel", SP, "body", 1, True, 0.65, rect(0, 0, 1.3, 0.8), region="spandrel", material="spandrel-glass"),
        shape("glass", IG, "glazing", 1, True, 0.15, rect(0, 0, 1.3, 2.35), regions=["glass-centre", "glass-edge"], material="glass"),
        shape("mullion-left", FR, member_role, 3, True, 0.4, rect(0, 0, 0.05, K.H), region="mullion", material="aluminium"),
        shape("mullion-right", FR, member_role, 3, True, 0.4, rect(1.35, 0, 1.4, K.H), region="mullion", material="aluminium"),
    ]
    out += [shape(f"transom-{n}", FR, member_role, 3, True, 0.4, rect(0.05, yc - 0.025, 1.35, yc + 0.025), region="transom", material="aluminium") for n, yc in K.TRANSOM_Y]
    if scale in (50, 100):
        out += [
            shape("reflex-a", IG, "glazing", 3 if scale == 50 else 2, False, 0.2, line(0.2, 1.75, 0.4, 2.25), region="glass-centre"),
            shape("reflex-b", IG, "glazing", 3 if scale == 50 else 2, False, 0.2, line(0.3, 1.75, 0.5, 2.25), region="glass-centre"),
        ]
    return out


def ad(points, map_):
    return [map_(a, d) for a, d in points]


def front(scale):
    """✂️ Section A-A through the bay centre (x = world z, exterior left; y = height), seen towards the left mullion."""
    if scale == 500:
        return [shape("bay-section", FR, "body", 1, True, 0.3, rect(-0.1, 0, 0.1, K.H), regions=["spandrel", "glass-centre", "glass-edge", "mullion", "transom"], material="aluminium")]
    out = [shape("mullion-side", FR, "body", 1, True, 0.75, rect(K.Z_CAP, 0, K.Z_MULLION_BACK, K.H), region="mullion", material="aluminium")]
    if scale == 50:
        out += [
            shape("mullion-side-cap", FR, "detail", 1.5, False, 0.4, line(K.Z_PLATE, 0, K.Z_PLATE, K.H), region="mullion"),
            shape("mullion-side-face", FR, "detail", 1.5, False, 0.4, line(K.Z_LIP, 0, K.Z_LIP, K.H), region="mullion"),
            shape("spandrel-enamel", SP, "glazing", 2, False, 0.65, line(-0.081, 0.05, -0.081, 0.85), region="spandrel", material="spandrel-glass", pen=0.13),
            shape("spandrel-wool", SP, "body", 2, True, 0.9, rect(K.Z_WOOL, 0.05, K.Z_PAN, 0.85), region="spandrel", material="wool"),
            shape("spandrel-pan", SP, "detail", 2.5, False, 0.3, line(K.Z_PAN + 0.00075, 0.05, K.Z_PAN + 0.00075, 0.85), region="spandrel", material="steel", pen=0.1),
            shape("pane-out", IG, "glazing", 2, False, 0.15, line(-0.081, 0.9, -0.081, 3.25), region="glass-centre", material="glass", pen=0.13),
            shape("pane-in", IG, "glazing", 2, False, 0.15, line(-0.059, 0.9, -0.059, 3.25), region="glass-centre", material="glass", pen=0.13),
        ]
        for name, (y0, y1) in (("base", (K.VISION_Y[0], K.VISION_Y[0] + K.SEAL_W)), ("head", (K.VISION_Y[1] - K.SEAL_W, K.VISION_Y[1]))):
            out.append(shape(f"spacer-{name}", IG, "detail", 3.5, True, 0.4, rect(K.Z_PANE_OUT, y0, K.Z_PANE_IN, y1), region="glass-edge", material="glass"))
        for name, yc in K.TRANSOM_Y:
            m = lambda a, d, yc=yc: [d, yc + a]
            out += [
                shape(f"transom-{name}", FR, "body", 3, True, 0.08, poly(ad(K.section_outline(K.Z_TRANSOM_BACK), m)), region="transom", material="aluminium"),
                shape(f"transom-{name}-break", FR, "detail", 3.2, True, 0.85, rect(K.Z_PLATE, yc - K.A_ISOLATOR, K.Z_CHANNEL, yc + K.A_ISOLATOR), region="transom", material="aluminium"),
                shape(f"transom-{name}-cap", FR, "detail", 3.2, False, 0.85, line(K.Z_PLATE, yc - K.A_CAP, K.Z_PLATE, yc + K.A_CAP), region="transom"),
                shape(f"transom-{name}-chamber", FR, "detail", 3.2, False, 0.85, poly(ad(TC, m)), region="transom"),
            ]
        return out
    if scale == 100:
        g0, g1, s0, s1 = K.Z_GLASS_OUT, K.Z_GLASS_IN, K.Z_GLASS_OUT, K.Z_PAN + K.PAN_T
    else:
        (g0, g1), (s0, s1) = V.GLASS200, V.SPAN200
    out += [
        shape("spandrel", SP, "body", 2, True, 0.65, rect(s0, 0.05, s1, 0.85), region="spandrel", material="spandrel-glass"),
        shape("glass", IG, "glazing", 2, True, 0.15, rect(g0, 0.9, g1, 3.25), regions=["glass-centre", "glass-edge"], material="glass"),
    ]
    role = "body" if scale == 100 else "detail"
    for name, yc in K.TRANSOM_Y:
        geometry = poly(ad(K.section_outline(K.Z_TRANSOM_BACK), lambda a, d, yc=yc: [d, yc + a])) if scale == 100 else rect(K.Z_CAP, yc - 0.025, K.Z_TRANSOM_BACK, yc + 0.025)
        out.append(shape(f"transom-{name}", FR, role, 3, True, 0.08, geometry, region="transom", material="aluminium"))
    return out


def plan(scale):
    """✂️ Section B-B at 2.075 m through the vision glass (x = world x, y = world z, exterior at the bottom), seen down onto the mid transom."""
    if scale == 500:
        m = 0.075
        notch = [[0, K.Z_CAP], [K.W, K.Z_CAP], [K.W, K.Z_MULLION_BACK], [K.W - m, K.Z_MULLION_BACK], [K.W - m, V.SPAN_BACK500], [m, V.SPAN_BACK500], [m, K.Z_MULLION_BACK], [0, K.Z_MULLION_BACK]]
        return [shape("bay-section", FR, "body", 1, True, 0.3, poly(notch), regions=["glass-centre", "glass-edge", "mullion", "transom"], material="aluminium")]
    out = [shape("transom-top", FR, "detail", 1, True, 0.75, rect(0.05, K.Z_CAP, 1.35, K.Z_TRANSOM_BACK), region="transom", material="aluminium")]
    role = "detail" if scale == 200 else "body"
    for side, x0 in (("left", K.MULLION_X[0]), ("right", K.MULLION_X[1])):
        m = lambda a, d, x0=x0: [x0 + a, d]
        if scale == 200:
            out.append(shape(f"mullion-{side}", FR, role, 3, True, 0.08, rect(x0 - 0.025, K.Z_CAP, x0 + 0.025, K.Z_MULLION_BACK), region="mullion", material="aluminium"))
            continue
        out.append(shape(f"mullion-{side}", FR, role, 3, True, 0.08, poly(ad(K.section_outline(K.Z_MULLION_BACK), m)), region="mullion", material="aluminium"))
        if scale == 50:
            out += [
                shape(f"mullion-{side}-break", FR, "detail", 3.2, True, 0.85, rect(x0 - K.A_ISOLATOR, K.Z_PLATE, x0 + K.A_ISOLATOR, K.Z_CHANNEL), region="mullion", material="aluminium"),
                shape(f"mullion-{side}-cap", FR, "detail", 3.2, False, 0.85, line(x0 - K.A_CAP, K.Z_PLATE, x0 + K.A_CAP, K.Z_PLATE), region="mullion"),
                shape(f"mullion-{side}-chamber", FR, "detail", 3.2, False, 0.85, poly(ad(MC, m)), region="mullion"),
            ]
    if scale == 50:
        out += [
            shape("transom-top-break", FR, "texture", 1.5, False, 0.4, line(0.05, K.Z_PLATE, 1.35, K.Z_PLATE), region="transom"),
            shape("transom-top-break-b", FR, "texture", 1.5, False, 0.4, line(0.05, K.Z_FACE, 1.35, K.Z_FACE), region="transom"),
            shape("pane-out", IG, "glazing", 2, False, 0.15, line(0.05, -0.081, 1.35, -0.081), region="glass-centre", material="glass", pen=0.13),
            shape("pane-in", IG, "glazing", 2, False, 0.15, line(0.05, -0.059, 1.35, -0.059), region="glass-centre", material="glass", pen=0.13),
            shape("spacer-left", IG, "detail", 3.5, True, 0.4, rect(K.GLASS_X[0], K.Z_PANE_OUT, K.GLASS_X[0] + K.SEAL_W, K.Z_PANE_IN), region="glass-edge", material="glass"),
            shape("spacer-right", IG, "detail", 3.5, True, 0.4, rect(K.GLASS_X[1] - K.SEAL_W, K.Z_PANE_OUT, K.GLASS_X[1], K.Z_PANE_IN), region="glass-edge", material="glass"),
        ]
    else:
        g0, g1 = (K.Z_GLASS_OUT, K.Z_GLASS_IN) if scale == 100 else V.GLASS200
        out.append(shape("glass", IG, "glazing", 2, True, 0.15, rect(0.05, g0, 1.35, g1), regions=["glass-centre", "glass-edge"], material="glass"))
    return out


CONVENTION = text(
    "Convention: the elevation is the outside face of the bay; the front view is the vertical section A-A through the bay centre seen from ahead (lateral x is the asset's z: exterior at the left, x = -0.10 m at the cover caps, interior at +0.10 m at the backs of the mullions) and the plan view is the horizontal section B-B through the middle of the vision glass seen from above (lateral y is z, exterior at the bottom); the left mullion lies behind the vertical cut, the mid transom below the horizontal cut.",
    "Konvention: die Ansicht ist die Außenfläche des Feldes; die Vorderansicht ist der Vertikalschnitt A-A durch die Feldmitte von vorn gesehen (seitliches x ist z des Objekts: außen links, x = -0,10 m an den Deckschalen, innen bei +0,10 m an den Pfostenrücken) und die Draufsicht ist der Horizontalschnitt B-B in halber Höhe der Verglasung von oben gesehen (seitliches y ist z, außen unten); der linke Pfosten liegt hinter dem Vertikalschnitt, der mittlere Riegel unter dem Horizontalschnitt.")


def joined(a, b):
    return text(a["en"] + " " + b["en"], a["de"] + " " + b["de"])


DECISIONS = {
    50: {
        "silhouette": joined(text("The face of a stick-system bay: 50 mm caps of the two mullions running through, three transom caps between them, the vision glass with two reflections and the enamelled spandrel.", "Die Außenfläche eines Pfosten-Riegel-Feldes: 50-mm-Deckschalen der beiden durchlaufenden Pfosten, drei Riegelkappen dazwischen, die Sichtverglasung mit zwei Reflexen und die emaillierte Brüstung."), joined(CONVENTION, text("At 1:50 section A-A shows each transom cut as its profile (cap, rebate zone with the polyamide isolator, tube with its chamber), the two panes as lines with the edge seal at their edges in the rebates, and the spandrel as enamel line, 100 mm wool and back pan; section B-B shows both mullions cut the same way, the panes with the edge seals and the mid transom below.", "Bei 1:50 zeigt der Schnitt A-A jeden Riegel geschnitten als Profil (Deckschale, Falzzone mit dem Polyamid-Isolator, Rohr mit seiner Kammer), die beiden Scheiben als Linien mit dem Randverbund an ihren Kanten im Falz und die Brüstung als Emaillinie, 100 mm Wolle und Rückwand; der Schnitt B-B zeigt beide Pfosten ebenso geschnitten, die Scheiben mit dem Randverbund und den mittleren Riegel darunter."))),
        "retained": text("Caps 50 mm (1 mm), the 14 mm isolators, the 12 mm edge seals, the tube chambers and the cap line in the sections; the zones of the glass (centre and 65 mm edge) are one drawn pane in the elevation, as they are one pane.", "Deckschalen 50 mm (1 mm), die 14-mm-Isolatoren, die 12-mm-Randverbunde, die Rohrkammern und die Kappenlinie in den Schnitten; die Glaszonen (Mitte und 65-mm-Rand) sind in der Ansicht eine gezeichnete Scheibe, wie sie eine Scheibe sind."),
        "merged": text("In the sections cap, pressure plate, gaskets, glass edges and tube walls are one dark profile (3 mm walls would print 0.06 mm); the chamber and the cap line are light lines in it.", "In den Schnitten sind Deckschale, Pressleiste, Dichtungen, Glasränder und Rohrwände ein dunkles Profil (3-mm-Wände würden 0,06 mm drucken); Kammer und Kappenlinie sind helle Linien darin."),
        "omitted": text("Gaskets, setting blocks, screws and transom connectors (below 0.15 mm on paper or hidden behind the caps); the shadow-box outline and the edge-zone rectangle of the earlier drawing, which no one sees on the real wall.", "Dichtungen, Klotzungen, Schrauben und Riegelverbinder (unter 0,15 mm auf dem Papier oder hinter den Kappen verdeckt); der Schattenkastenumriss und das Randzonenrechteck der früheren Zeichnung, die an der wirklichen Wand niemand sieht."),
        "thinElements": text("The 6 mm panes, the 6 mm enamel and the 1.5 mm back pan are pen lines (0.13 and 0.1 mm); the 16 mm cavity and the 20 mm air layer stay open.", "Die 6-mm-Scheiben, das 6-mm-Email und die 1,5-mm-Rückwand sind Strichlinien (0,13 und 0,1 mm); der 16-mm-Zwischenraum und die 20-mm-Luftschicht bleiben offen."),
        "hatching": text("Cut aluminium is filled solid dark, the drafting convention for thin metal sections; the isolators are light; the mullion side beyond the cut and the transom below it stay light grey. Glass, spandrel and wool are shown by lines and tone, not hatched.", "Geschnittenes Aluminium ist dunkel gefüllt, die Zeichenkonvention für dünne Metallschnitte; die Isolatoren sind hell; die Pfostenseite hinter dem Schnitt und der Riegel darunter bleiben hellgrau. Glas, Brüstung und Wolle sind durch Linien und Ton gezeigt, nicht schraffiert."),
        "styles": text("Frame outline 0.35 mm, details 0.18 mm, glass 0.13 mm.", "Rahmenumriss 0,35 mm, Details 0,18 mm, Glas 0,13 mm."),
        "overlays": text("One cell per region: spandrel, glass (centre and edge zone), mullions, transoms.", "Eine Zelle je Region: Brüstung, Glas (Mitte und Randzone), Pfosten, Riegel."),
    },
    100: {
        "silhouette": joined(text("Caps of mullions and transoms of true width around the glass pane with two reflection strokes and the dark spandrel.", "Deckschalen der Pfosten und Riegel in wahrer Breite um die Glasscheibe mit zwei Reflexstrichen und die dunkle Brüstung."), joined(CONVENTION, text("At 1:100 the cut members are dark profiles of their outline (cap, rebate zone and tube, 200 mm mullions and 150 mm transoms), the cut glass unit a 28 mm strip, the spandrel a strip from the glass to the back pan; the mid transom top is light below the horizontal cut.", "Bei 1:100 sind die geschnittenen Stäbe dunkle Profile ihres Umrisses (Kappe, Falzzone und Rohr, Pfosten 200 mm und Riegel 150 mm), die geschnittene Isolierverglasung ein 28-mm-Streifen, die Brüstung ein Streifen vom Glas bis zur Rückwand; die Oberseite des mittleren Riegels ist hell unter dem Horizontalschnitt."))),
        "retained": text("Mullions and transoms 0.5 mm, reflection strokes, spandrel 0.8 m, the true depth of the members.", "Pfosten und Riegel 0,5 mm, Reflexstriche, Brüstung 0,8 m, die wahre Tiefe der Stäbe."),
        "merged": text("Glass centre and edge zone are one pane; panes, cavity and edge seal one strip; enamel, air, wool and pan of the spandrel one strip.", "Glasmitte und Randzone sind eine Scheibe; Scheiben, Zwischenraum und Randverbund ein Streifen; Email, Luft, Wolle und Rückwand der Brüstung ein Streifen."),
        "omitted": text("Isolators, chambers, edge seals, gaskets and fixings.", "Isolatoren, Kammern, Randverbunde, Dichtungen und Befestigungen."),
        "thinElements": text("None thickened: the 28 mm unit prints 0.28 mm.", "Nichts verdickt: die 28-mm-Einheit druckt 0,28 mm."),
        "hatching": text("Cut aluminium filled solid dark; the mullion side beyond the cut and the transom below it light. Glass and spandrel by tone.", "Geschnittenes Aluminium dunkel gefüllt; die Pfostenseite hinter dem Schnitt und der Riegel darunter hell. Glas und Brüstung durch Ton."),
        "styles": text("Frame outline 0.25 mm, glass strokes 0.1 mm.", "Rahmenumriss 0,25 mm, Glasstriche 0,1 mm."),
        "overlays": text("Cells: spandrel, glass, mullions, transoms.", "Zellen: Brüstung, Glas, Pfosten, Riegel."),
    },
    200: {
        "silhouette": joined(text("The grid of a stick-system bay: two mullion bars, three transom bars, a filled spandrel and a light glass pane.", "Das Raster eines Pfosten-Riegel-Feldes: zwei Pfostenbalken, drei Riegelbalken, eine gefüllte Brüstung und eine helle Glasscheibe."), joined(CONVENTION, text("At 1:200 section A-A shows the mullion side as the ground mass with spandrel, glass strip and three transom blocks over it; section B-B shows the two mullion blocks, the glass strip and the transom below.", "Bei 1:200 zeigt der Schnitt A-A die Pfostenseite als Grundfläche mit Brüstung, Glasstreifen und drei Riegelblöcken darüber; der Schnitt B-B zeigt die beiden Pfostenblöcke, den Glasstreifen und den Riegel darunter."))),
        "retained": text("Mullions and transoms as filled bars of their real 0.05 m width (0.25 mm) and depth, spandrel 0.8 m, glass 2.35 m.", "Pfosten und Riegel als gefüllte Balken ihrer wahren Breite von 0,05 m (0,25 mm) und Tiefe, Brüstung 0,8 m, Glas 2,35 m."),
        "merged": text("Both glass zones are one pane; cap, rebate and tube one bar.", "Beide Glaszonen sind eine Scheibe; Kappe, Falz und Rohr ein Balken."),
        "omitted": text("Reflections, isolators, chambers, cap outline.", "Reflexe, Isolatoren, Kammern, Kappenumriss."),
        "thinElements": text("The 28 mm glass unit is drawn 32 mm (0.16 mm) in both sections so that it prints; the frame members are filled at their real width.", "Die 28-mm-Isolierverglasung ist in beiden Schnitten 32 mm (0,16 mm) gezeichnet, damit sie druckt; die Rahmenstäbe sind in wahrer Breite gefüllt."),
        "hatching": text("Cut aluminium filled solid dark; the mullion side beyond the cut and the transom below it light.", "Geschnittenes Aluminium dunkel gefüllt; die Pfostenseite hinter dem Schnitt und der Riegel darunter hell."),
        "styles": text("Frame bars 0.1 mm outline, dark; glass 0.1 mm; spandrel dark.", "Rahmenbalken mit 0,1-mm-Umriss, dunkel; Glas 0,1 mm; Brüstung dunkel."),
        "overlays": text("Cells: spandrel, glass (centre and edge merged), mullions, transoms.", "Zellen: Brüstung, Glas (Mitte und Rand zusammengefasst), Pfosten, Riegel."),
    },
    500: {
        "silhouette": joined(text("Two filled blocks: a dark spandrel and a light vision field above it, outlined by the frame.", "Zwei gefüllte Blöcke: eine dunkle Brüstung und darüber das helle Sichtfeld, vom Rahmen umrissen."), joined(CONVENTION, text("At 1:500 section A-A is one filled strip 0.4 x 6.6 mm; section B-B is the vision layer between the two full-depth mullions, which reach 80 mm further into the room than the spandrel zone below.", "Bei 1:500 ist der Schnitt A-A ein gefüllter Streifen 0,4 x 6,6 mm; der Schnitt B-B ist die Sichtschicht zwischen den beiden Pfosten voller Tiefe, die 80 mm weiter in den Raum reichen als die Brüstungszone darunter."))),
        "retained": text("Bay 1.40 x 3.30 m, depth 0.20 m, spandrel 0.85 m high (1.7 mm), vision field 2.45 m, the step of the mullion backs in plan.", "Feld 1,40 x 3,30 m, Tiefe 0,20 m, Brüstung 0,85 m hoch (1,7 mm), Sichtfeld 2,45 m, die Stufe der Pfostenrücken im Grundriss."),
        "merged": text("Mullions (0.05 m = 0.1 mm), transoms and both glass zones are one block listing their regions; the 0.13 mm outline stands for the frame.", "Pfosten (0,05 m = 0,1 mm), Riegel und beide Glaszonen sind ein Block, der ihre Regionen nennt; der 0,13-mm-Umriss steht für den Rahmen."),
        "omitted": text("Reflections, individual members, glass layers, spandrel build-up, isolators and profile parts.", "Reflexe, einzelne Stäbe, Glasschichten, Brüstungsaufbau, Isolatoren und Profilteile."),
        "thinElements": text("The frame is the outline (0.13 mm = 0.065 m, wider than the real 0.05 m); the mullion backs are drawn 75 mm wide and the step behind the spandrel zone 80 mm deep so that both print.", "Der Rahmen ist der Umriss (0,13 mm = 0,065 m, breiter als die wirklichen 0,05 m); die Pfostenrücken sind 75 mm breit und die Stufe hinter der Brüstungszone 80 mm tief gezeichnet, damit beide drucken."),
        "hatching": text("None: at 1:500 the cut bay is one filled shape.", "Keine: bei 1:500 ist das geschnittene Feld eine gefüllte Form."),
        "styles": text("Light glass, dark spandrel, black outline.", "Helles Glas, dunkle Brüstung, schwarzer Umriss."),
        "overlays": text("Cells: spandrel, vision block (glass, frame merged).", "Zellen: Brüstung, Sichtblock (Glas, Rahmen zusammengefasst)."),
    },
}

REFERENCES = [
    {"id": "schueco-fws-50", "citation": "Schüco FWS 50 mullion-transom facade system, product data: face width of mullion, transom and pressure plate / cover cap 50 mm, mullion basic depth 50 to 250 mm, glass and panel thickness 4 to 62 mm (schueco.com, mullion-transom facades, FWS 50)."},
    {"id": "en-1279", "citation": "EN 1279-1:2018 Glass in building, insulating glass units: edge seal of spacer, butyl primary and polysulfide or silicone secondary seal."},
    {"id": "din-18545", "citation": "DIN 18545:2015 Sealing of glazing with sealants and gaskets, and the glass setting guideline it refers to: glass bite about 15 mm, setting blocks 5 mm under the full unit thickness near the quarter points on glass carriers."},
]

MATERIALS = [
    {"id": "epdm", "title": text("EPDM glazing gaskets, setting blocks and packers", "EPDM-Glasdichtungen, Klotzungen und Ausgleichsprofile"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10077-2"},
    {"id": "polyamide", "title": text("Polyamide 6.6 with 25 % glass fibre, isolator", "Polyamid 6.6 mit 25 % Glasfaser, Isolator"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1300, "conductivityWmK": 0.3, "specificHeatJKgK": 1600, "elasticModulusPa": 4500000000, "source": "literature", "reference": "en-iso-10077-2"},
    {"id": "edge-seal", "title": text("Warm-edge spacer with butyl and silicone secondary seal", "Warme-Kante-Abstandhalter mit Butyl und Silikon-Zweitdichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1200, "conductivityWmK": 0.35, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10077-2"},
]


def anchors():
    """📌 The two slab brackets (pins at the mullion heads) and the two spigots (lateral rollers at the mullion feet), and the load anchors at the depth of what they load."""
    out = []
    for side, x in (("left", K.MULLION_X[0]), ("right", K.MULLION_X[1])):
        out.append({"id": f"head-anchor-{side}", "bone": FR, "x": x, "y": V.HEAD_Y, "z": V.ANCHOR_Z, "kind": "support",
                    "title": text(f"Slab bracket bolt at the head of the {side} mullion, pin (hangs the mullion)", f"Konsolbolzen am Kopf des {'linken' if side == 'left' else 'rechten'} Pfostens, Festlager (hängt den Pfosten auf)"), "support": "pin"})
    for side, x in (("left", K.MULLION_X[0]), ("right", K.MULLION_X[1])):
        out.append({"id": f"base-guide-{side}", "bone": FR, "x": x, "y": 0, "z": 0.027, "kind": "support",
                    "title": text(f"Spigot in the foot of the {side} mullion, lateral roller (holds the wind, slides vertically)", f"Stoßdorn im Fuß des {'linken' if side == 'left' else 'rechten'} Pfostens, seitliches Rollenlager (hält den Wind, gleitet senkrecht)"),
                    "support": "roller", "normalDeg": 0, "tiltDeg": 90})
    out += [
        {"id": "cradle-hook", "bone": "cradle", "x": 0, "y": 0, "z": -0.094, "kind": "load", "title": text("Hook of the maintenance cradle on the mid transom cap", "Haken der Wartungsgondel an der Kappe des mittleren Riegels")},
        {"id": "igu-centre", "bone": IG, "x": 0.65, "y": 1.175, "z": -0.07, "kind": "load", "title": text("Centre of the glazing unit", "Mitte der Glaseinheit")},
        {"id": "spandrel-centre", "bone": SP, "x": 0.65, "y": 0.4, "z": r5(V.SPANDREL_COM_Z), "kind": "load", "title": text("Centre of the spandrel", "Mitte der Brüstung")},
    ]
    return out


def apply(a):
    a["summary"] = text(
        "One 1.35 m wide, 3.30 m high bay of a stick-system curtain wall: thermally broken aluminium mullions 50 x 150 mm and transoms 50 x 100 mm with polyamide isolators, pressure plates and 12 mm cover caps (U_f 2.0 and 2.2), a 6-16-6 argon glass unit (U_g about 1.08) clamped 15 mm deep with a warm-edge seal, and an enamelled spandrel with an insulated steel back pan (U about 0.31); the mullions hang from slab brackets and are guided by spigots. Every element is a conditioned layered region; sun and shade follow the pose sunlit fractions.",
        "Ein 1,35 m breites, 3,30 m hohes Feld einer Pfosten-Riegel-Fassade: thermisch getrennte Aluminiumpfosten 50 x 150 mm und -riegel 50 x 100 mm mit Polyamid-Isolatoren, Pressleisten und 12-mm-Deckschalen (U_f 2,0 und 2,2), eine Argon-Isolierverglasung 6-16-6 (U_g etwa 1,08), 15 mm tief eingespannt, mit Warme-Kante-Randverbund, und eine emaillierte Brüstung mit gedämmter Stahlrückwand (U etwa 0,31); die Pfosten hängen an Deckenkonsolen und sind über Stoßdorne geführt. Jedes Element ist eine konditionierte, geschichtete Region; Sonne und Schatten folgen den Besonnungsanteilen der Posen.")
    refs = a["provenance"]["references"]
    for ref in REFERENCES:
        if ref["id"] not in [r["id"] for r in refs]:
            refs.append(ref)
    for r in refs:
        if r["id"] == "profile-catalogues":
            r["citation"] = "Manufacturer data for mullion-transom facade systems: mullion 50 x 150 mm and transom 50 x 100 mm aluminium tubes of about 3 mm wall, polyamide isolator in the glass joint, pressure plate and snap-on cover cap 50 x 12 mm, spandrel as enamelled 6 mm glass with an insulated 1.5 mm steel back pan. Typical values consulted; the masses are computed from the modelled sections."
    a["provenance"]["notes"] = text(
        "Bay width 1.35 m centre to centre, extents 1.40 m including both mullion faces; storey height 3.30 m; depth 0.20 m from the cover caps to the backs of the mullions (12 mm cap and plate, 4 mm gasket, 28 mm glass, 6 mm gasket, 150 mm mullion). A 50 x 150 mm mullion (I = 359 cm4) spanning 3.3 m under 1.08 kPa on 1.35 m deflects about 9 mm (L/370). Layer resistances follow the EN ISO 12631 component method.",
        "Feldbreite 1,35 m von Mitte zu Mitte, Gesamtbreite 1,40 m einschließlich beider Pfostenansichten; Geschosshöhe 3,30 m; Tiefe 0,20 m von den Deckschalen bis zu den Pfostenrücken (12 mm Kappe und Pressleiste, 4 mm Dichtung, 28 mm Glas, 6 mm Dichtung, 150 mm Pfosten). Ein Pfosten 50 x 150 mm (I = 359 cm4) über 3,3 m unter 1,08 kPa auf 1,35 m biegt sich etwa 9 mm durch (L/370). Die Schichtwiderstände folgen dem Komponentenverfahren nach EN ISO 12631.")
    total = V.FRAME_KG + V.IGU_KG + V.SPANDREL_KG
    a["dimensions"] = {"heightM": 3.3, "widthM": 1.4, "depthM": 0.2, "massKg": round(total, 2)}
    for m in MATERIALS:
        if m["id"] not in [x["id"] for x in a["materials"]]:
            a["materials"].append(m)
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones[FR]["title"] = text("Mullion-transom frame (fixed root, hung from the slab brackets)", "Pfosten-Riegel-Rahmen (feste Wurzel, an den Deckenkonsolen aufgehängt)")
    bones[FR]["mass"].update({"kg": round(V.FRAME_KG, 2), "comX": 0.7, "comY": round(V.FRAME_COM_Y, 3), "source": "authored-estimate"})
    bones[FR]["section"] = {"areaM2": round(A_M, 7), "secondMomentM4": float("%.4g" % I_M), "extremeFibreM": round(C_M, 4), "material": "aluminium",
                            "title": text("Mullion 50 x 150 mm, aluminium tube, 3 mm walls, with screw channel", "Pfosten 50 x 150 mm, Aluminiumrohr, 3-mm-Wände, mit Schraubkanal")}
    bones[IG]["mass"].update({"kg": round(V.IGU_KG, 2), "comX": 0.65, "comY": 1.175})
    bones[SP]["mass"].update({"kg": round(V.SPANDREL_KG, 2), "comX": 0.65, "comY": 0.4})
    bones["cradle"]["title"] = text("Maintenance cradle hook on the mid transom cap (massless)", "Wartungsgondel-Haken an der Kappe des mittleren Riegels (masselos)")
    a["anchors"] = anchors()
    for s in (50, 100, 200, 500):
        rep = a["representations"][str(s)]
        rep["geometry"] = {"elevation": elevation(s), "front": front(s), "plan": plan(s)}
        rep["decisions"] = DECISIONS[s]
    import runpy
    runpy.run_path(os.path.join(os.path.dirname(__file__), "equipment-fine-ink.py"))["apply"](a)
    for pose in a["poses"]:
        if pose["id"] == "dead-load":
            pose["description"] = text("The bay hangs from the slab brackets at the heads of its mullions under its own weight, sunlit.", "Das Feld hängt unter Eigengewicht an den Deckenkonsolen an den Köpfen seiner Pfosten, besonnt.")
        for load in pose["loads"]:
            if load["id"] == "seismic-igu":
                load["fxN"] = round(0.12 * V.IGU_KG * G, 3)
            if load["id"] == "seismic-spandrel":
                load["fxN"] = round(0.12 * V.SPANDREL_KG * G, 3)
    for region in a["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
            for layer in region["layers"]:
                if layer["id"] == "break":
                    layer["thicknessM"] = 0.038
                    layer["title"] = text("Polyamide isolator, gaskets and glass edges in the 38 mm rebate zone (U_f %s W/m2K)" % ("2.0" if region["id"] == "mullion" else "2.2"),
                                          "Polyamid-Isolator, Dichtungen und Glasränder in der 38-mm-Falzzone (U_f %s W/m2K)" % ("2,0" if region["id"] == "mullion" else "2,2"))
                if layer["id"] == "cap":
                    layer["title"] = text("Pressure plate and cover cap, 2 mm aluminium", "Pressleiste und Deckschale, 2 mm Aluminium")
    a["thermal"]["assumptions"][1] = text(
        "Centre-of-glass U 1.08, edge zone 1.72, spandrel 0.31, mullion 2.00, transom 2.20 W/m2K; the glass regions are the visible glass (centre and 65 mm edge zone, EN ISO 10077-1), the frame regions the outer surface of the cover caps, face and sides (EN ISO 12631 frame zone: the glass edges in the rebate belong to the frame).",
        "U Scheibenmitte 1,08, Randzone 1,72, Brüstung 0,31, Pfosten 2,00, Riegel 2,20 W/m2K; die Glasregionen sind das sichtbare Glas (Mitte und 65-mm-Randzone, EN ISO 10077-1), die Rahmenregionen die Außenfläche der Deckschalen, Ansicht und Seiten (Rahmenzone nach EN ISO 12631: die Glasränder im Falz gehören zum Rahmen).")
    a["structure"]["supports"] = ["head-anchor-left", "head-anchor-right", "base-guide-left", "base-guide-right"]
    a["structure"]["assumptions"][0] = text(
        "Each mullion hangs from a slab bracket at its head (pin) and is held across the wall by the spigot at its foot (lateral roller, free to slide vertically for the storey movement); frame, glass and spandrel are rigid parts, the glass rests on two setting blocks and is clamped by the pressure plates. In-plane actions (seismic) and the out-of-plane wind suction are both represented; the suction is applied as two point resultants on the tributary zones (authored estimate guided by EN 1991-1-4, not a code evaluation). The two brackets share the in-plane loads by the equal-stiffness assumption of the solver.",
        "Jeder Pfosten hängt an seinem Kopf an einer Deckenkonsole (Festlager) und wird an seinem Fuß vom Stoßdorn quer zur Wand gehalten (seitliches Rollenlager, senkrecht verschieblich für die Geschossbewegung); Rahmen, Glas und Brüstung sind starre Teile, das Glas steht auf zwei Klotzungen und wird von den Pressleisten gehalten. Einwirkungen in der Ebene (Erdbeben) und der Windsog senkrecht zur Ebene sind beide abgebildet; der Sog ist als zwei Punktresultierende auf den Einzugszonen angesetzt (Schätzwert nach dem Vorbild von EN 1991-1-4, keine normgemäße Auswertung). Die beiden Konsolen teilen sich die Lasten in der Ebene nach der Annahme gleicher Steifigkeit des Lösers.")


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        if arg.startswith("--area="):
            rid, value = arg[7:].split(":")
            AREAS[rid] = float(value)
    source = open(L.CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = copy.deepcopy(document)
    apply(document["asset"])
    print("mullion A %.7f I %.4e c %.4f | frame %.2f kg com y %.3f | igu %.2f | spandrel %.2f | total %.2f" % (A_M, I_M, C_M, V.FRAME_KG, V.FRAME_COM_Y, V.IGU_KG, V.SPANDREL_KG, V.FRAME_KG + V.IGU_KG + V.SPANDREL_KG))
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = L.member(source, key)
            source = source[:start] + L.dumped(document["asset"][key]) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
            if key in MEMBERS and check["asset"][key] != document["asset"][key]:
                raise SystemExit("member %s not written as intended" % key)
        with open(L.CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "curtain-wall-bay.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-curtain-wall-bay")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written")
