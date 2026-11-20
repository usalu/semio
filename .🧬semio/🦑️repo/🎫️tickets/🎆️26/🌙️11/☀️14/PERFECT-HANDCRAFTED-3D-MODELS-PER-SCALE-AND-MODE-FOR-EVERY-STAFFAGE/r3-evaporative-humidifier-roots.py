"""🌱 Round 3, machinery/evaporative-humidifier: the root corrections outside `volumes` (rig, anchors, texts, references, thermal data, structure text) and the 2D drawings of 1:50, 1:100 and 1:200 (elevation section A-A, front section B-B, plan section C-C), drawn by hand from the measures of `r3-evaporative-humidifier-geo.py` (ruling E1).

Usage: imported by `r3-evaporative-humidifier-build.py` (`apply_roots(asset, areas)`).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("humidifier_geo", os.path.join(HERE, "r3-evaporative-humidifier-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STEEL = "galvanised-steel"
PP = "pp-plastic"
R6 = lambda v: round(v, 6)


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def shape(id, bone, role, z, tone, geom, region=None, material=None, filled=True, **kw):
    """✏️ One 2D shape with the common members in the catalog's order."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    if region:
        out["region"] = region
    if material:
        out["material"] = material
    out.update(kw)
    return out


def rect(x0, y0, x1, y1):
    """▭ Rect geometry from two corners."""
    return {"kind": "rect", "x": R6(x0), "y": R6(y0), "w": R6(x1 - x0), "h": R6(y1 - y0)}


def erect(bone, x0, y0, x1, y1):
    """▭ World rectangle as a rect of the elevation frame of a bone."""
    ox, oy = G.RIG[bone]
    return rect(x0 - ox, y0 - oy, x1 - ox, y1 - oy)


def poly(points, closed=True):
    """📐 Poly geometry."""
    return {"kind": "poly", "points": [[R6(x), R6(y)] for x, y in points], "closed": closed}


def epoly(bone, points, closed=True):
    """📐 World points as a poly of the elevation frame of a bone."""
    return poly([G.loc(bone, x, y) for x, y in points], closed)


def rects_path(rects):
    """🧩 Path geometry of several separate rectangles (x0, y0, x1, y1)."""
    return {"kind": "path", "d": " ".join(f"M {R6(a)} {R6(b)} L {R6(c)} {R6(b)} L {R6(c)} {R6(d)} L {R6(a)} {R6(d)} Z" for a, b, c, d in rects)}


def ellipse(cx, cy, rx, ry=None):
    """⭕ Ellipse geometry."""
    return {"kind": "ellipse", "cx": R6(cx), "cy": R6(cy), "rx": R6(rx), "ry": R6(ry if ry else rx)}


CAS = dict(region="casing", material=STEEL)
PEN = {50: 0.0045, 100: 0.009, 200: 0.013}
DROP_R = {50: (0.01, 0.013), 100: (0.014, 0.0182)}
HOSE_PEN = {50: 0.32, 100: 0.16}


def elevation(s):
    """🖼️ Side elevation drawn as section A-A on the lateral mid plane (z = 0), seen from the service side: the far side panel behind everything, the connection frames with the tank end walls cut, runners, tank water with pump, riser hose and float ball, rails, the bulkhead frame and its reveal, the pad cut along its sheets, the trough with its film and the three drops of the section plane."""
    hx, top = G.HX, G.TOP
    out = []
    if s == 200:
        out.append(shape("casing", "casing", "body", 3.0, 0.9, erect("casing", -hx, 0.0, hx, top), **CAS))
        out.append(shape("water", "water", "body", 3.2, 0.45, erect("water", -0.22, 0.03, 0.22, 0.07), region="sump", material="water"))
        out.append(shape("bulkhead", "casing", "body", 3.25, 0.3, epoly("casing", [[-0.135, 0.07], [-0.135, top - PEN[200]]], closed=False), filled=False, **CAS))
        out.append(shape("pad", "pad", "body", 3.3, 0.7, erect("pad", -0.12, 0.07, 0.1, 0.48), regions=["pad"], material="cellulose"))
        out.append(shape("trough", "trough", "body", 3.5, 0.4, erect("trough", -G.TROUGH_X, 0.48, G.TROUGH_X, 0.54), region="trough", material=PP))
        return out
    f = {50: 0.01, 100: 0.015}[s]
    out.append(shape("casing", "casing", "body", 3.0, 0.9, erect("casing", -hx, G.BASE, hx, top), **CAS))
    for name, x in (("inlet", -G.RUNNER_X), ("outlet", G.RUNNER_X)):
        out.append(shape(f"runner-{name}", "casing", "body", 3.1, 0.25, erect("casing", x - 0.02, 0.0, x + 0.02, G.BASE), **CAS))
    if s == 50:
        out.append(shape("end-frame-inlet", "casing", "body", 3.15, 0.55, erect("casing", -hx, 0.09, -hx + f, top), **CAS))
        out.append(shape("end-frame-outlet", "casing", "body", 3.15, 0.55, erect("casing", hx - f, 0.09, hx, top), **CAS))
        ox, oy = G.RIG["casing"]
        out.append(shape("sump-tray", "casing", "body", 3.4, 0.3, rects_path([(-hx - ox, G.BASE, -hx + f - ox, 0.09), (hx - f - ox, G.BASE, hx - ox, 0.09)]), **CAS))
    pump = G.pump_outline() if s == 50 else [[0.137, 0.021], [0.203, 0.021], [0.203, 0.106], [0.137, 0.106]]
    out.append(shape("pump", "casing", "body", 3.16, 0.25, epoly("casing", pump), material=PP))
    out.append(shape("float-ball", "casing", "body", 3.17, 0.6, ellipse(*G.loc("casing", G.FLOAT[0], G.FLOAT[1]), G.FLOAT_R), material=PP))
    out.append(shape("water", "water", "body", 3.2, 0.45, erect("water", -hx + f, G.BASE, hx - f, G.LEVEL), region="sump", material="water"))
    out.append(shape("riser-hose", "casing", "detail", 3.6, 0.3, epoly("casing", G.hose_path(s), closed=False), filled=False, material=PP, region="trough", penMm=HOSE_PEN[s]))
    for name, x in (("inlet", -G.RAIL_X), ("outlet", G.RAIL_X)):
        out.append(shape(f"pad-rail-{name}", "casing", "body", 3.22, 0.3, erect("casing", x - 0.01, 0.05, x + 0.01, 0.08), **CAS))
    if s == 50:
        out.append(shape("bulkhead", "casing", "body", 3.25, 0.3, epoly("casing", [[-0.12375, 0.045], [-0.12375, top - PEN[50]]], closed=False), filled=False, **CAS))
        out.append(shape("pad-reveal", "casing", "body", 3.26, 0.5, erect("casing", G.BULKHEAD_X, 0.0875, -G.PAD_X, 0.4775), **CAS))
    else:
        out.append(shape("bulkhead", "casing", "body", 3.25, 0.3, epoly("casing", [[-0.1275, 0.04], [-0.1275, top - PEN[100]]], closed=False), filled=False, **CAS))
        out.append(shape("pad-reveal", "casing", "body", 3.26, 0.5, erect("casing", G.BULKHEAD_X, 0.08, -G.PAD_X, 0.48), **CAS))
    out.append(shape("pad", "pad", "body", 3.3, 0.7, erect("pad", -G.PAD_X, G.PAD_Y[0], G.PAD_X, G.PAD_Y[1]), region="pad", material="cellulose", hatch="flute-cut"))
    out.append(shape("trough", "trough", "body", 3.5, 0.4, erect("trough", -G.TROUGH_X, 0.48, G.TROUGH_X, 0.53), region="trough", material=PP))
    if s == 50:
        out.append(shape("film", "pad", "body", 3.6, 0.85, erect("pad", -0.117, 0.483, 0.117, 0.493), region="pad", material="water"))
    rx, ry = DROP_R[s]
    xs = (-G.DROP_X, 0.0, G.DROP_X) if s == 50 else (-G.DROP_X - 0.004, 0.0, G.DROP_X + 0.004)
    for k, x in enumerate(xs):
        out.append(shape(f"drop-{k + 1}", "drip", "body", 3.9, 0.85, ellipse(x, 0.0, rx, ry), region="pad", material="water"))
    return out


def front(s):
    """🖼️ Front elevation drawn as section B-B across the pad (x = 0), seen from the outlet: the bulkhead behind everything inside the cut casing, the inlet runner, the tank water, the inlet rail, the pad cut across its sheets (vertical hatch), the trough with its film and the three drops of the section plane."""
    hz, top = G.HZ, G.TOP
    out = []
    if s == 200:
        out.append(shape("casing", "casing", "body", 3.0, 0.9, rect(-hz, 0.0, hz, top), **CAS))
        out.append(shape("water", "water", "body", 3.2, 0.45, rect(-hz, 0.03, hz, 0.07), region="sump", material="water"))
        out.append(shape("pad", "pad", "body", 3.3, 0.7, rect(-hz, 0.07, hz, 0.48), regions=["pad", "casing"], material="cellulose"))
        out.append(shape("trough", "trough", "body", 3.5, 0.4, rect(-hz, 0.48, hz, 0.54), region="trough", material=PP))
        return out
    out.append(shape("casing", "casing", "body", 3.0, 0.9, rect(-hz, G.BASE, hz, top), **CAS))
    out.append(shape("runner-inlet", "casing", "body", 3.1, 0.25, rect(-hz, 0.0, hz, G.BASE), **CAS))
    out.append(shape("water", "water", "body", 3.2, 0.45, rect(-hz, G.BASE, hz, G.LEVEL), region="sump", material="water"))
    out.append(shape("pad-rail-inlet", "casing", "body", 3.22, 0.3, rect(-hz, G.LEVEL, hz, 0.08), **CAS))
    out.append(shape("pad", "pad", "body", 3.3, 0.7, rect(-G.PAD_Z, G.PAD_Y[0], G.PAD_Z, G.PAD_Y[1]), region="pad", material="cellulose", hatch="flutes"))
    out.append(shape("trough", "trough", "body", 3.5, 0.4, rect(-G.TROUGH_Z, 0.48, G.TROUGH_Z, 0.53), region="trough", material=PP))
    if s == 50:
        out.append(shape("film", "pad", "body", 3.6, 0.85, rect(-0.187, 0.483, 0.187, 0.493), region="pad", material="water"))
    rx, ry = DROP_R[s]
    for k, z in enumerate((-G.DROP_Z, 0.0, G.DROP_Z)):
        out.append(shape(f"drop-{k + 1}", "drip", "body", 3.9, 0.85, ellipse(z, 0.5, rx, ry), region="pad", material="water"))
    return out


def plan(s):
    """🖼️ Plan drawn as section C-C under the top panel (y = 0.56), seen from above: the cut casing and connection frames, the open water of the tank upstream and downstream, the bulkhead, the trough with its film and the drops at their drip holes, the pump with the hose and the float valve."""
    hx, hz = G.HX, G.HZ
    out = [shape("casing", "casing", "body", 3.0, 0.9, rect(-hx, -hz, hx, hz), **CAS)]
    if s == 200:
        out.append(shape("water", "water", "body", 3.2, 0.45, rects_path([(-0.22, -hz, -0.15, hz), (0.12, -hz, 0.22, hz)]), region="sump", material="water"))
        out.append(shape("bulkhead", "casing", "body", 3.25, 0.3, poly([[-0.135, -hz + PEN[200]], [-0.135, hz - PEN[200]]], closed=False), filled=False, **CAS))
        out.append(shape("trough", "trough", "body", 3.5, 0.4, rect(-G.TROUGH_X, -hz, G.TROUGH_X, hz), regions=["trough", "pad"], material=PP))
        return out
    f = {50: 0.01, 100: 0.015}[s]
    t = G.FEATURE[s]
    if s == 50:
        out.append(shape("end-frame-inlet", "casing", "body", 3.15, 0.55, rect(-hx, -hz, -hx + f, hz), **CAS))
        out.append(shape("end-frame-outlet", "casing", "body", 3.15, 0.55, rect(hx - f, -hz, hx, hz), **CAS))
    out.append(shape("water", "water", "body", 3.2, 0.45, rects_path([(-hx + f, -hz, G.BULKHEAD_X - t, hz), (G.TROUGH_X, -hz, hx - f, hz)]), region="sump", material="water"))
    px, pz = G.PUMP
    out.append(shape("pump", "casing", "body", 3.3, 0.25, ellipse(px, pz, 0.034 if s == 50 else 0.033), material=PP))
    out.append(shape("riser-hose", "casing", "detail", 3.6, 0.3, poly([[px, pz], [0.09, pz]], closed=False), filled=False, material=PP, region="trough", penMm=HOSE_PEN[s]))
    fx, _, fz = G.FLOAT
    if s == 50:
        out.append(shape("float-valve", "casing", "body", 3.28, 0.3, rect(fx - 0.011, 0.09, fx + 0.011, hz), material=PP))
    out.append(shape("float-ball", "casing", "body", 3.3, 0.6, ellipse(fx, fz, G.FLOAT_R), material=PP))
    out.append(shape("bulkhead", "casing", "body", 3.25, 0.3, poly([[G.BULKHEAD_X - t / 2, -hz + PEN[s]], [G.BULKHEAD_X - t / 2, hz - PEN[s]]], closed=False), filled=False, **CAS))
    out.append(shape("trough", "trough", "body", 3.5, 0.4, rect(-G.TROUGH_X, -G.TROUGH_Z, G.TROUGH_X, G.TROUGH_Z), region="trough", material=PP))
    if s == 50:
        out.append(shape("film", "pad", "body", 3.6, 0.85, rect(-0.111, -0.178, 0.111, 0.178), region="pad", material="water"))
        rx = DROP_R[50][0]
        k = 0
        for z in (-G.DROP_Z, 0.0, G.DROP_Z):
            for x in (-G.DROP_X, 0.0, G.DROP_X):
                k += 1
                out.append(shape(f"drop-{k}", "drip", "body", 3.9, 0.85, ellipse(x, z, rx), region="pad", material="water"))
    else:
        out.append(shape("film", "pad", "body", 3.6, 0.85, rect(-0.117, -0.187, 0.117, 0.187), region="pad", material="water"))
        rx = DROP_R[100][0]
        for k, (x, z) in enumerate(((0.0, -G.DROP_Z), (-G.DROP_X - 0.004, 0.0), (0.0, 0.0), (G.DROP_X + 0.004, 0.0), (0.0, G.DROP_Z))):
            out.append(shape(f"drop-{k + 1}", "drip", "body", 3.9, 0.85, ellipse(x, z, rx), region="pad", material="water"))
    return out


SECTIONS = {
    "elevation": {"plane": 0, "label": T("A-A", "A-A")},
    "front": {"plane": 0, "label": T("B-B", "B-B")},
    "plan": {"plane": 0.56, "label": T("C-C", "C-C")},
}

FLUTES = {"id": "flutes", "title": T("Pad cut across its sheets", "Matte quer zu den Lagen geschnitten"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.5, "penMm": 0.1}
FLUTE_CUT = {"id": "flute-cut", "title": T("Pad cut along its sheets: crossing flutes", "Matte längs der Lagen geschnitten: gekreuzte Wellen"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.7, "penMm": 0.1}
HATCH = {50: [FLUTES, FLUTE_CUT], 100: [FLUTES, FLUTE_CUT], 200: []}

DECISIONS = {
    50: {
        "silhouette": T("A 0.5 m by 0.6 m evaporative humidifier section, 10 mm by 12 mm on paper, drawn in three sections. A-A (side) cuts the casing on its lateral mid plane: the air passes from left to right through the 0.2 m deep pad, which stands on two rails over the tank; the trough with its water film lies on the pad, the bulkhead frame with its reveal stands on the inlet side and dips into the water, the pump with its riser hose stands downstream and the float ball upstream; three drops fall on the two pad faces and inside the pad. B-B (front) cuts across the pad: the hatched pad between water and trough inside the casing. C-C (plan) cuts under the top panel: the trough with film and drip points between the two open water surfaces, with pump, hose and float valve.", "Ein 0,5 m mal 0,6 m großer Verdunstungsbefeuchterteil, auf Papier 10 mm mal 12 mm, in drei Schnitten gezeichnet. A-A (Seite) schneidet das Gehäuse in der Mittelebene: die Luft strömt von links nach rechts durch die 0,2 m tiefe Matte, die auf zwei Schienen über der Wanne steht; die Rinne mit ihrem Wasserfilm liegt auf der Matte, der Schottrahmen mit Laibung steht auf der Eintrittsseite und taucht ins Wasser, die Pumpe mit Steigschlauch steht stromab und die Schwimmerkugel stromauf; drei Tropfen fallen an den beiden Mattenflächen und in der Matte. B-B (vorn) schneidet quer durch die Matte: die schraffierte Matte zwischen Wasser und Rinne im Gehäuse. C-C (Grundriss) schneidet unter dem Deckel: die Rinne mit Film und Tropfstellen zwischen den beiden offenen Wasserflächen, mit Pumpe, Schlauch und Schwimmerventil."),
        "retained": T("Casing with the far panel, connection frames and tank end walls, base runners, tank water, pump, riser hose, float ball and valve, rails, bulkhead and reveal, pad with its hatches, trough, film and drops.", "Gehäuse mit hinterer Wand, Anschlussrahmen und Wannenstirnwände, Kufen, Wannenwasser, Pumpe, Steigschlauch, Schwimmerkugel und -ventil, Schienen, Schott und Laibung, Matte mit ihren Schraffuren, Rinne, Film und Tropfen."),
        "merged": T("The 1 mm casing sheet is the casing outline; the cassette channels are the pad outline; the two legs of each rail are one block; the 2 mm runner tube with its end plug is one 40 by 20 mm block.", "Das 1-mm-Gehäuseblech ist der Gehäuseumriss; die Kassettenprofile sind der Mattenumriss; die zwei Schenkel jeder Schiene sind ein Block; das 2-mm-Kufenrohr mit seinem Stopfen ist ein Block von 40 mal 20 mm."),
        "omitted": T("The cover fasteners and the wall fittings (4 mm proud), the float lever in the side section, the drip holes and the pump cable.", "Die Deckelverschlüsse und die Wandverschraubungen (4 mm vorstehend), der Schwimmerhebel im Seitenschnitt, die Tropflöcher und das Pumpenkabel."),
        "thinElements": T("The 7.5 mm bulkhead is a body stroke; the 16 mm riser hose is a stroke of 0.32 mm; the 7 mm sheets of the pad are a hatch; the drops are 20 mm.", "Das 7,5 mm starke Schott ist ein Körperstrich; der 16 mm dicke Steigschlauch ist ein Strich von 0,32 mm; die 7-mm-Lagen der Matte sind eine Schraffur; die Tropfen haben 20 mm."),
        "hatching": T("In B-B a vertical line hatch of 0.5 mm paper pitch fills the pad cut across its sheets; in A-A a cross hatch at 45 degrees of 0.7 mm pitch fills the pad cut along its sheets (the crossing flutes); both with a 0.1 mm pen.", "In B-B füllt eine senkrechte Linienschraffur mit 0,5 mm Papierabstand die quer zu den Lagen geschnittene Matte; in A-A füllt eine Kreuzschraffur unter 45 Grad mit 0,7 mm Abstand die längs der Lagen geschnittene Matte (die gekreuzten Wellen); beide mit 0,1-mm-Stift."),
        "styles": T("Darker tones carry the steel and plastic parts, the light tone the far casing panel, mid tones pad and water; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Dunklere Töne tragen die Stahl- und Kunststoffteile, der helle Ton die hintere Gehäusewand, mittlere Töne Matte und Wasser; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": T("Casing, wet pad, trough and sump are four passive regions; the cooled moist air leaves the outlet frame as a vent emitter that scales with the water key.", "Gehäuse, nasse Matte, Rinne und Wanne sind vier passive Regionen; die gekühlte feuchte Luft verlässt den Austrittsrahmen als Lüftungsemitter, der mit der Wasserstufe skaliert."),
    },
    100: {
        "silhouette": T("A 5 mm by 6 mm humidifier in the same three sections: in A-A the casing on its runners, the tank water with pump, hose and float ball, the pad on its rails behind the bulkhead and its reveal, the trough and three drops; in B-B the hatched pad between water and trough; in C-C the trough with film and five drip points between the two water surfaces.", "Ein 5 mm mal 6 mm großer Befeuchter in denselben drei Schnitten: in A-A das Gehäuse auf seinen Kufen, das Wannenwasser mit Pumpe, Schlauch und Schwimmerkugel, die Matte auf ihren Schienen hinter Schott und Laibung, die Rinne und drei Tropfen; in B-B die schraffierte Matte zwischen Wasser und Rinne; in C-C die Rinne mit Film und fünf Tropfstellen zwischen den beiden Wasserflächen."),
        "retained": T("Casing, runners, tank water, pump, hose, float ball, rails, bulkhead and reveal, pad with hatch, trough, film in plan, drops.", "Gehäuse, Kufen, Wannenwasser, Pumpe, Schlauch, Schwimmerkugel, Schienen, Schott und Laibung, Matte mit Schraffur, Rinne, Film im Grundriss, Tropfen."),
        "merged": T("The connection frames and the tank end walls join the casing outline; the cassette joins reveal and pad; the film joins the trough in both elevations.", "Anschlussrahmen und Wannenstirnwände gehen im Gehäuseumriss auf; die Kassette geht in Laibung und Matte auf; der Film geht in beiden Ansichten in der Rinne auf."),
        "omitted": T("Fasteners, fittings, float valve and lever, drip holes.", "Verschlüsse, Verschraubungen, Schwimmerventil und Hebel, Tropflöcher."),
        "thinElements": T("The 15 mm bulkhead is a body stroke, the hose a stroke of 0.16 mm; the drops are 28 mm.", "Das 15 mm starke Schott ist ein Körperstrich, der Schlauch ein Strich von 0,16 mm; die Tropfen haben 28 mm."),
        "hatching": T("The same two pad hatches as at 1:50: vertical lines in B-B, a 45 degree cross hatch in A-A.", "Dieselben zwei Mattenschraffuren wie bei 1:50: senkrechte Linien in B-B, eine Kreuzschraffur unter 45 Grad in A-A."),
        "styles": T("Darker tones carry the steel and plastic parts, the light tone the far casing panel; body pen 0.18 mm, detail for the hose, ground 0.25 mm.", "Dunklere Töne tragen die Stahl- und Kunststoffteile, der helle Ton die hintere Gehäusewand; Körperstift 0,18 mm, Detail für den Schlauch, Boden 0,25 mm."),
        "overlays": T("Same four regions and one emitter as at 1:50.", "Dieselben vier Regionen und ein Emitter wie bei 1:50."),
    },
    200: {
        "silhouette": T("A 2.5 mm by 3 mm humidifier of a few masses in the same three sections: the casing, the water slab, the pad block behind the bulkhead and the trough; in B-B water, pad and trough fill the casing width; in C-C the trough between the two water surfaces.", "Ein 2,5 mm mal 3 mm großer Befeuchter aus wenigen Massen in denselben drei Schnitten: das Gehäuse, die Wasserplatte, der Mattenblock hinter dem Schott und die Rinne; in B-B füllen Wasser, Matte und Rinne die Gehäusebreite; in C-C die Rinne zwischen den beiden Wasserflächen."),
        "retained": T("Casing, water, pad, bulkhead, trough.", "Gehäuse, Wasser, Matte, Schott, Rinne."),
        "merged": T("Runners and tank floor join the casing; rails, reveal and cassette join the pad block; in B-B the 20 mm strips beside pad and trough are closed (0.1 mm on paper), so the pad block there also stands for the casing behind it; in the plan the trough block carries the regions trough and pad.", "Kufen und Wannenboden gehen im Gehäuse auf; Schienen, Laibung und Kassette gehen im Mattenblock auf; in B-B sind die 20-mm-Streifen neben Matte und Rinne geschlossen (0,1 mm auf Papier), so dass der Mattenblock dort auch für das Gehäuse dahinter steht; im Grundriss trägt der Rinnenblock die Regionen Rinne und Matte."),
        "omitted": T("Hatches, film, drops, pump, hose, float and frames.", "Schraffuren, Film, Tropfen, Pumpe, Schlauch, Schwimmer und Rahmen."),
        "thinElements": T("Water and trough are 40 and 60 mm thick (0.2 and 0.3 mm on paper); the bulkhead is a body stroke.", "Wasser und Rinne sind 40 und 60 mm dick (0,2 und 0,3 mm auf Papier); das Schott ist ein Körperstrich."),
        "hatching": T("None.", "Keine."),
        "styles": T("Darker tones carry the steel and plastic parts, the light tone the far casing panel; body pen 0.13 mm, detail not drawn, ground 0.25 mm.", "Dunklere Töne tragen die Stahl- und Kunststoffteile, der helle Ton die hintere Gehäusewand; Körperstift 0,13 mm, Detail entfällt, Boden 0,25 mm."),
        "overlays": T("Same four regions and one emitter as at 1:50.", "Dieselben vier Regionen und ein Emitter wie bei 1:50."),
    },
}

RIG_BONES = {
    "casing": {"x": -0.25, "y": 0.0, "lengthM": 0.5, "com": (0.25, 0.26), "title": T("Casing with tank on its base runners (base edge, inlet to outlet)", "Gehäuse mit Wanne auf seinen Kufen (Grundkante, Eintritt bis Austritt)")},
    "pad": {"x": 0.15, "y": 0.08, "lengthM": 0.2, "com": (0.1, 0.2), "title": T("Cellulose evaporation pad (lower edge, inlet face to outlet face)", "Cellulose-Verdunstungsmatte (Unterkante, Eintritts- bis Austrittsfläche)")},
    "trough": {"x": 0.13, "y": 0.48, "lengthM": 0.24, "com": (0.12, 0.025), "title": T("Water distribution trough (floor, wall to wall)", "Wasserverteilrinne (Boden, Wand zu Wand)")},
    "water": {"x": 0.01, "y": 0.02, "lengthM": 0.48, "com": (0.24, 0.017), "title": T("Tank water (tank floor, end wall to end wall)", "Wannenwasser (Wannenboden, Stirnwand zu Stirnwand)")},
    "drip": {"x": 0.12, "y": 0.02, "lengthM": 0.02, "com": None, "title": None},
}

ANCHORS = {"base-contact": {"x": 0.25, "y": 0.0, "footprintLengthM": 0.42}, "outlet": {"x": 0.5, "y": 0.3}}

NOTES = T(
    "A pad-type evaporative humidifier section of the 1000 m3/h class, air flowing along x. A duct-section casing of 1 mm galvanised sheet, 0.5 m long, 0.4 m wide and 0.58 m high, with two 30 mm connection frames and a removable service cover with four quarter-turn fasteners, stands on two base runners 20 mm high; its lower 70 mm are the tank. The cross-fluted cellulose pad is 0.36 m wide, 0.40 m high and 0.20 m deep in the air direction (the cassette depth of the 85 to 90 percent efficiency class; 0.1 m gives about 65 percent), 3 kg wet; it stands in a cassette on two rails over the tank, and a bulkhead frame on the inlet side closes the bypass around it. A 3 mm polypropylene distribution trough of 0.24 m by 0.38 m lies on the cassette and drips 0.5 L/min over the pad; a submersible pump with a riser hose feeds it from the tank, which holds 6 L (32 mm of water); a float valve keeps the level and a drain leaves the far wall. The face velocity is 1.9 m/s (0.28 m3/s over 0.144 m2), so no droplet separator is needed. Mass 13.8 kg with the full tank (casing with fittings 4, pad 3, trough 0.8, water 6). Adiabatic saturation takes the latent heat of the evaporating water (about 2.45 MJ/kg) from the air: at 90 percent efficiency the 22 degC room air (50 percent) leaves at about 16 degC and about 95 percent humidity, 6 K cooler and 2.4 g/kg moister (0.28 m3/s: -2.0 kW sensible, +2.0 kW latent, 0.8 g/s or 2.9 L/h of water). The wetted pad and the tank water sit at the wet-bulb temperature of the room air (15 degC, lifted to about 18 degC by the radiation of the warm room), never below it: they carry no heat source, the evaporation is their only sink. The drawings are the sections A-A (lateral mid plane), B-B (across the pad) and C-C (under the top panel). The origin is the centre of the base on the surface.",
    "Ein Matten-Verdunstungsbefeuchterteil der Klasse 1000 m3/h, Luftrichtung entlang x. Ein Kanalgehäuse aus 1 mm verzinktem Blech, 0,5 m lang, 0,4 m breit und 0,58 m hoch, mit zwei 30-mm-Anschlussrahmen und einem abnehmbaren Revisionsdeckel mit vier Schnellverschlüssen, steht auf zwei 20 mm hohen Kufen; seine unteren 70 mm sind die Wanne. Die kreuzgewellte Cellulosematte ist 0,36 m breit, 0,40 m hoch und in Luftrichtung 0,20 m tief (die Kassettentiefe der Wirkungsgradklasse 85 bis 90 Prozent; 0,1 m ergeben etwa 65 Prozent), nass 3 kg; sie steht in einer Kassette auf zwei Schienen über der Wanne, und ein Schottrahmen auf der Eintrittsseite schließt den Bypass um sie. Eine 3 mm starke Polypropylen-Verteilrinne von 0,24 m mal 0,38 m liegt auf der Kassette und tropft 0,5 l/min über die Matte; eine Tauchpumpe mit Steigschlauch speist sie aus der Wanne, die 6 l fasst (32 mm Wasser); ein Schwimmerventil hält den Stand, und ein Ablauf verlässt die hintere Wand. Die Anströmgeschwindigkeit beträgt 1,9 m/s (0,28 m3/s über 0,144 m2), ein Tropfenabscheider ist daher nicht nötig. Die Masse beträgt 13,8 kg mit gefüllter Wanne (Gehäuse mit Einbauten 4, Matte 3, Rinne 0,8, Wasser 6). Die adiabate Sättigung entzieht der Luft die Verdunstungswärme des Wassers (etwa 2,45 MJ/kg): bei 90 Prozent Wirkungsgrad verlässt die 22 degC warme Raumluft (50 Prozent) das Gerät mit etwa 16 degC und etwa 95 Prozent Feuchte, 6 K kühler und 2,4 g/kg feuchter (0,28 m3/s: -2,0 kW fühlbar, +2,0 kW latent, 0,8 g/s oder 2,9 l/h Wasser). Die benetzte Matte und das Wannenwasser liegen auf der Kühlgrenztemperatur der Raumluft (15 degC, durch die Strahlung des warmen Raums auf etwa 18 degC angehoben), nie darunter: sie haben keine Wärmequelle, die Verdunstung ist ihre einzige Senke. Die Zeichnungen sind die Schnitte A-A (Mittelebene), B-B (quer durch die Matte) und C-C (unter dem Deckel). Der Ursprung ist die Mitte des Fußes auf der Unterlage.",
)

SUMMARY = T(
    "A 0.5 m by 0.6 m evaporative humidifier section of an air handler, drawn cut open: water drips from a trough over a fluted cellulose pad into a tank, and the evaporating water cools the pad and the air that passes through it.",
    "Ein 0,5 m mal 0,6 m großer Verdunstungsbefeuchterteil eines Lüftungsgeräts, aufgeschnitten gezeichnet: Wasser tropft aus einer Rinne über eine gewellte Cellulosematte in eine Wanne, und das verdunstende Wasser kühlt die Matte und die durchströmende Luft.",
)

REFERENCES = [{"id": "munters-fa6", "citation": "Munters Europe AB. FA6 evaporative humidifier and cooler, user manual: cassette depth 100, 200 and 300 mm for the efficiency classes 65, 85 and 95 percent; tank, distribution header, circulation pump, drain."}]

STRUCTURE_TEXT = T(
    "The 14 kg humidifier stands on its two base runners: a unilateral contact of 0.42 m by 0.4 m carries the weight; pad and trough rest on the cassette rails, the water on the tank floor.",
    "Der 14 kg schwere Befeuchter steht auf seinen zwei Kufen: ein einseitiger Kontakt von 0,42 m mal 0,4 m trägt das Gewicht; Matte und Rinne ruhen auf den Kassettenschienen, das Wasser auf dem Wannenboden.",
)


def apply_roots(asset, areas):
    """🌱 Writes every root correction into the asset dict in place."""
    asset["summary"] = SUMMARY
    asset["provenance"]["notes"] = NOTES
    have = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in REFERENCES if r["id"] not in have]
    for bone in asset["rig"]["bones"]:
        fix = RIG_BONES[bone["id"]]
        bone["x"], bone["y"], bone["lengthM"] = fix["x"], fix["y"], fix["lengthM"]
        if fix["com"]:
            bone["mass"]["comX"], bone["mass"]["comY"] = fix["com"]
        if fix["title"]:
            bone["title"] = fix["title"]
    for anchor in asset["anchors"]:
        anchor.update(ANCHORS[anchor["id"]])
    for region in asset["thermal"]["regions"]:
        if region["id"] in areas:
            region["areaM2"] = areas[region["id"]]
        if region["id"] == "pad":
            region["layers"][0]["thicknessM"] = 0.2
        if region["id"] == "sump":
            region["layers"][0]["thicknessM"] = 0.032
            region["characteristicLengthM"] = 0.48
        if region["id"] == "trough":
            region["characteristicLengthM"] = 0.38
    asset["structure"]["assumptions"] = [STRUCTURE_TEXT]
    for s in (50, 100, 200):
        rep = asset["representations"][str(s)]
        rep["hatches"] = HATCH[s]
        if s == 100:
            rep["pens"]["detail"] = 0.13
        rep["geometry"] = {"elevation": elevation(s), "front": front(s), "plan": plan(s)}
        new = {}
        for key, value in rep.items():
            if key == "decisions":
                new["sections"] = SECTIONS
            if key != "sections":
                new[key] = value
        new["decisions"] = DECISIONS[s]
        asset["representations"][str(s)] = new
    rep = asset["representations"]["500"]
    rep["reason"] = T("At 1:500 the unit is only 1 mm wide on paper and its smallest parts, the 20 mm drops and the 3 mm trough wall, are far below every print profile.", "Bei 1:500 ist das Gerät auf Papier nur 1 mm breit, und seine kleinsten Teile, die 20 mm großen Tropfen und die 3 mm starke Rinnenwand, liegen weit unter jedem Druckprofil.")
    rep["limitingFeature"] = T("The 20 mm drops and the 3 mm trough wall", "Die 20 mm großen Tropfen und die 3 mm starke Rinnenwand")
