from author_11_lib import *

B = Ctx("body", "abs-cream", None, "body", 0.78)
TC = B(tone=0.6)
OG = Ctx("body", "steel-mesh", None, "body", 0.2)
IN = Ctx("body", "pc-heat", None, "body", 0.3)
CT = Ctx("body", "pp-dial", None, "body", 0.2)
KN = CT(bone="knob")
FT = Ctx("body", "rubber-foot", None, "body", 0.15)
Z0, Z1 = -0.115, 0.12

TH = lambda ctx, layer: ctx(layer=layer)
T_CAS = B(layer="wall")
T_TOP = TC(layer="cover")
T_GRL = OG(layer="bars")
T_INT = IN(layer="inlet")
T_CTL = CT(layer="dial")
T_FT = FT(layer="foot")


def reg(sol, region):
    sol["region"] = region
    return sol


def handle_outline(n=8):
    return [[-0.045, 0.268]] + bez([(-0.045, 0.268), (-0.045, 0.3), (-0.04, 0.31), (-0.02, 0.31)], n) + [[-0.02, 0.31], [0.03, 0.31]] + bez([(0.03, 0.31), (0.05, 0.31), (0.055, 0.3), (0.055, 0.268)], n) + [[0.055, 0.268]]


def foot(sid, x0, x1, y1, z0, z1, h=0.016, e=4, rep=None):
    cz, cx = (z0 + z1) / 2, (x0 + x1) / 2
    hz, hx = (z1 - z0) / 2, (x1 - x0) / 2
    return lf(sid, FT, "v", [(0, hz - 0.0015, hx - 0.002, e, cz, cx), (h, hz, hx, e, cz, cx)], repeat={"count": 2, "translation": [0, 0, rep]} if rep else None)


def knob_dial(sid, ctx, zo, depth, r, rh):
    pr = [[0, 0], [0, rh], [0.0012, r - 0.002], [0.003, r], [depth, r], [depth, 0]]
    return rev(sid, ctx, pr, axis="w", origin=(0, 0, zo))


def mass_foot(sid, ctx, anchor, x, h, repz):
    return tag(box(sid, ctx, (x - 0.0175, 0, -0.1), (x + 0.0175, h, -0.07), repeat={"count": 2, "translation": [0, 0, repz]}), tags=[anchor])


NZ0, NZ1 = 0.0976, 0.1055
# ------------------------------------------------------------------ 1:50
Q = qz(math.degrees(math.atan2(-0.009, 0.019)))
FIN = math.hypot(0.019, 0.009)
body50 = [
    foot("foot-back", -0.075, -0.04, 0.016, -0.1, -0.07, rep=0.17),
    foot("foot-front", 0.04, 0.075, 0.016, -0.1, -0.07, rep=0.17),
    rrloft("housing", B, 0.016, 0.25, -0.09, 0.078, Z0, Z1, e=7),
    ext("housing-front", B, rrect(0.016, Z0, 0.25, Z1, 0.02), "yz", 0.078, 0.095, holes=[rrect(0.06, -0.105, 0.23, 0.105, 0.004)]),
    rrloft("top-cover", TC, 0.25, 0.272, -0.0903, 0.0953, Z0 - 0.0003, Z1 + 0.0003, e=7),
    ext("belt", B(role="detail", tone=0.3), rrect(Z0 - 0.0005, -0.0905, Z1 + 0.0005, 0.0955, 0.0205), "zx", 0.0445, 0.0525, holes=[rrect(Z0, -0.09, Z1, 0.095, 0.02)]),
    ext("handle", B(tone=0.75), handle_outline(), "xy", -0.065, 0.065, holes=[rrect(-0.028, 0.278, 0.037, 0.298, 0.008, 4)]),
    box("outlet-mesh", OG, (0.0705, 0.06, -0.105), (0.078, 0.23, 0.105), cr=0.002),
    box("louvres", OG(role="detail", tone=0.9), (-FIN / 2, -0.00375, -0.1), (FIN / 2, 0.00375, 0.1), tr=(0.0885, 0.0705 + 0.0, 0), rot=qz(math.degrees(math.atan2(-0.009, 0.019))), repeat={"count": 8, "translation": [0, 0.02, 0]}),
    box("inlet-plate", IN(tone=0.15), (-0.0975, 0.05, -0.105), (-0.09, 0.21, 0.105), cr=0.002),
    box("inlet-slat-bottom", IN, (-0.098, 0.05, -0.105), (-0.0905, 0.059, 0.105), cr=0.003),
    box("inlet-slats", IN, (-0.098, 0.061, -0.105), (-0.0905, 0.061 + 0.019667, 0.105), cr=0.003, repeat={"count": 6, "translation": [0, 0.021667, 0]}),
    box("inlet-slat-top", IN, (-0.098, 0.191, -0.105), (-0.0905, 0.21, 0.105), cr=0.003),
    knob_dial("dial", KN, -0.12, 0.008, 0.024, 0.017),
    box("mode-switch", CT, (-0.02, 0.092, -0.119), (0.02, 0.112, -0.1105), cr=0.0019),
]
th50 = [
    reg(th("th-housing", "housing", "casing", "wall"), "casing"),
    reg(th("th-handle", "handle", "casing", "wall"), "casing"),
    reg(th("th-top-cover", "top-cover", "top-cover", "cover"), "top-cover"),
    reg(th("th-outlet-mesh", "outlet-mesh", "grille", "bars"), "grille"),
    reg(box("th-louvres-3", OG(layer="bars", role="detail", tone=0.9), (-FIN / 2, -0.00375, -0.1), (FIN / 2, 0.00375, 0.1), tr=(0.0885, 0.0705, 0), rot=qz(math.degrees(math.atan2(-0.009, 0.019))), repeat={"count": 2, "translation": [0, 0.06, 0]}), "grille"),
    tag(reg(lf("th-warm-air", T_GRL, "u", [(NZ0, 0.05, 0.05, 2, 0, 0.145), (NZ1, 0.05, 0.05, 2, 0, 0.145)], cs=False, ce=False), "grille"), tags=["warm-air-1", "warm-air-2"]),
    reg(th("th-inlet-plate", "inlet-plate", "intake", "inlet"), "intake"),
    reg(th("th-inlet-slat-bottom", "inlet-slat-bottom", "intake", "inlet"), "intake"),
    reg(th("th-inlet-slat-top", "inlet-slat-top", "intake", "inlet"), "intake"),
    reg(th("th-dial", "dial", "control", "dial"), "control"),
    reg(th("th-foot-back", "foot-back", "feet", "foot"), "feet"),
    reg(th("th-foot-front", "foot-front", "feet", "foot"), "feet"),
]
steel = Ctx("body", "abs-cream", None, "detail", 0.25)
SM = steel(member="body", part="support")
st50 = [
    stm("member-body", "housing", "body", "member"),
    cyl("member-knob", steel(member="knob", part="member", bone="knob"), "w", (0, 0, -0.1025), 0.025, 0.006),
    cyl("joint-knob", steel(member="knob", part="joint", bone="knob"), "w", (0, 0, -0.1085), 0.0125, 0.0105),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, 0.14, 0), (0.06, 0.09, 0.07)),
    tag(box("support-foot-back", SM, (-0.0735, 0, -0.098), (-0.0415, 0.0075, -0.072), repeat={"count": 2, "translation": [0, 0, 0.17]}), tags=["foot-back"]),
    tag(box("support-foot-front", SM, (0.0415, 0, -0.098), (0.0735, 0.0075, -0.072), repeat={"count": 2, "translation": [0, 0, 0.17]}), tags=["foot-front"]),
]
D50 = [
    text("Own 1:50 model against the 1:50 elevation, front and plan: rounded housing as a superelliptic loft (0.02 m corner radius, offset 2.5 mm to +z as drawn) with a separate front frame whose opening carries a 7.5 mm mesh plate and eight slanted louvre fins that reach the drawn +x extreme of 0.098 m; belt seam, top cover skin, handle extruded from the drawn side outline with the finger opening as a true hole, four lofted rubber feet.",
         "Eigenes 1:50-Modell nach Ansicht, Seitenansicht und Grundriss von 1:50: abgerundetes Gehäuse als superelliptischer Loft (Eckenradius 0,02 m, wie gezeichnet 2,5 mm nach +z versetzt) mit eigenem Frontrahmen, dessen Öffnung eine 7,5 mm Gitterplatte und acht schräge Lamellen trägt, die das gezeichnete Maximum +x von 0,098 m erreichen; Gürtelfuge, Deckelhaut, Griff aus der gezeichneten Seitenkontur mit Grifföffnung als echtem Loch, vier geloftete Gummifüsse."),
    text("Sub-feature parts are repaired for the 7.5 mm limit: the dial is a revolved knob 8 mm deep (3 mm outside the wall, the rest sunk into it, which gives the 0.24 m depth), the mode switch stands 8.5 mm with 4 mm relief, the louvre fins are 7.5 mm thick; the dial pointer, hub ring and mode mark are merged into the dial and the switch.",
         "Teile unter dem 7,5-mm-Merkmal sind repariert: die Skala ist ein gedrehter Knopf von 8 mm Tiefe (3 mm ausserhalb der Wand, der Rest in ihr versenkt, das ergibt die Tiefe von 0,24 m), der Modusschalter steht 8,5 mm mit 4 mm Relief, die Lamellen sind 7,5 mm dick; Skalenzeiger, Nabenring und Modusmarke gehen im Knopf und im Schalter auf."),
    text("Thermal (area = exposed surface): casing = housing and handle by reference, top cover = its own skin slab, grille = the 7.5 mm mesh plate and two representative louvre fins (the other six are passive vanes) plus the warm-air nozzle (tags warm-air-1 and warm-air-2, 0.1 m on the outlet anchor), intake = the plate and the two end slats (the six middle slats are passive vanes), control = the dial, feet = the four feet. Faces buried in other solids and the floor contact do not count, so areaM2 was corrected at the root to the modelled exposed surfaces (see report). Single-layer regions are surfaces, so no nested shells.",
         "Thermik (Fläche = freie Oberfläche): Gehäuse = Gehäuse und Griff als Verweis, Deckel = eigene Hautplatte, Gitter = die 7,5-mm-Gitterplatte und zwei stellvertretende Lamellen (die übrigen sechs sind passive Flügel) samt Warmluftdüse (Markierungen warm-air-1 und warm-air-2, 0,1 m am Auslassanker), Ansaugung = die Platte und die zwei Endlamellen (die sechs mittleren Lamellen sind passive Flügel), Bedienung = die Skala, Füsse = die vier Füsse. Verdeckte Flächen und der Bodenkontakt zählen nicht, daher wurde areaM2 an der Wurzel auf die modellierten freien Flächen korrigiert (siehe Bericht). Einlagige Regionen sind Flächen, verschachtelte Schalen entfallen."),
    text("Structure: the housing is the body member, the knob shaft (12 mm) with its bearing sleeve is the knob member and joint, a compact mass ellipsoid sits at the centre of mass (y 0.14 m), and contact pads under the two foot pairs are the supports.",
         "Tragwerk: das Gehäuse ist der Körperstab, die Knopfwelle (12 mm) mit Lagerhülse ist Knopfstab und Gelenk, ein kompakter Massenellipsoid liegt im Schwerpunkt (y 0,14 m), und Kontaktflächen unter den beiden Fusspaaren sind die Auflager."),
]

NZ0, NZ1 = 0.0955, 0.1105
# ------------------------------------------------------------------ 1:100
body100 = [
    lf("foot-back", FT, "v", [(0, 0.0135, 0.0155, 4, -0.085, -0.0575), (0.02, 0.015, 0.0175, 4, -0.085, -0.0575)], repeat={"count": 2, "translation": [0, 0, 0.17]}),
    lf("foot-front", FT, "v", [(0, 0.014, 0.0155, 4, -0.085, 0.0575), (0.02, 0.015, 0.0175, 4, -0.085, 0.0575)], repeat={"count": 2, "translation": [0, 0, 0.17]}),
    rrloft("housing", B, 0.02, 0.272, -0.09, 0.0955, Z0, Z1, e=6),
    rrloft("top-cover", TC, 0.257, 0.272, -0.0903, 0.0958, Z0 - 0.0003, Z1 + 0.0003, e=6),
    ext("handle", B(tone=0.75), handle_outline(), "xy", -0.065, 0.065),
    box("outlet-grille", OG, (0.0795, 0.06, -0.105), (0.098, 0.23, 0.105), cr=0.003),
    box("inlet-block", IN(tone=0.2), (-0.098, 0.05, -0.105), (-0.0825, 0.21, 0.105), cr=0.003),
    knob_dial("dial", KN, -0.12, 0.015, 0.024, 0.02),
]
th100 = [
    reg(th("th-housing", "housing", "casing", "wall"), "casing"),
    reg(th("th-handle", "handle", "casing", "wall"), "casing"),
    reg(th("th-top-cover", "top-cover", "top-cover", "cover"), "top-cover"),
    reg(th("th-outlet-grille", "outlet-grille", "grille", "bars"), "grille"),
    tag(reg(lf("th-warm-air", T_GRL, "u", [(NZ0, 0.05, 0.05, 2, 0, 0.145), (NZ1, 0.05, 0.05, 2, 0, 0.145)], cs=False, ce=False), "grille"), tags=["warm-air-1", "warm-air-2"]),
    reg(th("th-inlet-block", "inlet-block", "intake", "inlet"), "intake"),
    reg(th("th-dial", "dial", "control", "dial"), "control"),
    reg(th("th-foot-back", "foot-back", "feet", "foot"), "feet"),
    reg(th("th-foot-front", "foot-front", "feet", "foot"), "feet"),
]
st100 = [
    stm("member-body", "housing", "body", "member"),
    cyl("member-knob", steel(member="knob", part="member", bone="knob"), "w", (0, 0, -0.1025), 0.025, 0.0075),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, 0.14, 0), (0.06, 0.09, 0.07)),
    tag(box("support-foot-back", SM, (-0.0735, 0, -0.098), (-0.0415, 0.015, -0.072), repeat={"count": 2, "translation": [0, 0, 0.17]}), tags=["foot-back"]),
    tag(box("support-foot-front", SM, (0.0415, 0, -0.098), (0.0735, 0.015, -0.072), repeat={"count": 2, "translation": [0, 0, 0.17]}), tags=["foot-front"]),
]
D100 = [
    text("Own 1:100 model drawn against the 1:100 shapes: one housing loft from the 20 mm feet line up to 0.272 m, the outlet grille as a 18.5 mm block (the mesh plate, front frame and eight louvre fins are merged, the four drawn louvre lines fall below 15 mm), a 15 mm inlet block, the handle extruded from the side outline without the finger opening, a 15 mm knob.",
         "Eigenes 1:100-Modell nach den 1:100-Formen: ein Gehäuse-Loft von der 20-mm-Fusslinie bis 0,272 m, das Auslassgitter als 18,5-mm-Block (Gitterplatte, Frontrahmen und acht Lamellen sind zusammengefasst, die vier gezeichneten Lamellenlinien liegen unter 15 mm), ein 15-mm-Einlassblock, der Griff aus der Seitenkontur ohne Grifföffnung, ein 15-mm-Knopf."),
    text("Thermal and structure are rebuilt on these masses: housing and handle, the 15 mm top-cover slab, the grille block with the nozzle, the inlet block, the dial and the feet, all by reference to the 1:100 body; structure as at 1:50 with the 15 mm feature (housing member, knob shaft, mass ellipsoid, foot pads); the bearing sleeve is merged into the shaft.",
         "Thermik und Tragwerk sind auf diesen Massen neu aufgebaut: Gehäuse und Griff, die 15-mm-Deckelplatte, der Gitterblock mit der Düse, der Einlassblock, die Skala und die Füsse, alle als Verweis auf den 1:100-Körper; Tragwerk wie bei 1:50 mit dem 15-mm-Merkmal (Gehäusestab, Knopfwelle, Massenellipsoid, Fussauflager); die Lagerhülse geht in der Welle auf."),
]

NZ0, NZ1 = 0.0955, 0.1255
# ------------------------------------------------------------------ 1:200
body200 = [
    lf("foot-back", FT, "v", [(0, 0.0175, 0.02, 2, -0.085, -0.0575), (0.03, 0.0175, 0.02, 2, -0.085, -0.0575)], repeat={"count": 2, "translation": [0, 0, 0.17]}),
    lf("foot-front", FT, "v", [(0, 0.0175, 0.02, 2, -0.085, 0.0575), (0.03, 0.0175, 0.02, 2, -0.085, 0.0575)], repeat={"count": 2, "translation": [0, 0, 0.17]}),
    rrloft("housing", B, 0.015, 0.242, -0.09, 0.0955, Z0, Z1, e=10),
    rrloft("top-cover", TC, 0.242, 0.272, -0.0903, 0.0958, Z0 - 0.0003, Z1 + 0.0003, e=10),
    box("inlet-block", IN(tone=0.2), (-0.098, 0.05, -0.105), (-0.068, 0.21, 0.105)),
    ext("handle", B(tone=0.75), [[-0.045, 0.262], [-0.045, 0.28], [-0.04, 0.31], [0.03, 0.31], [0.055, 0.28], [0.055, 0.262]], "xy", -0.065, 0.065),
    box("outlet-grille", OG, (0.0675, 0.06, -0.105), (0.098, 0.23, 0.105), cr=0.005),
    knob_dial("dial", KN, -0.12, 0.03, 0.024, 0.02),
]
th200 = [
    reg(th("th-housing", "housing", "casing", "wall"), "casing"),
    reg(th("th-handle", "handle", "casing", "wall"), "casing"),
    reg(th("th-top-cover", "top-cover", "top-cover", "cover"), "top-cover"),
    reg(th("th-outlet-grille", "outlet-grille", "grille", "bars"), "grille"),
    tag(reg(lf("th-warm-air", T_GRL, "u", [(NZ0, 0.05, 0.05, 2, 0, 0.145), (NZ1, 0.05, 0.05, 2, 0, 0.145)], cs=False, ce=False), "grille"), tags=["warm-air-1", "warm-air-2"]),
    reg(th("th-inlet-block", "inlet-block", "intake", "inlet"), "intake"),
    reg(th("th-dial", "dial", "control", "dial"), "control"),
    reg(th("th-foot-back", "foot-back", "feet", "foot"), "feet"),
    reg(th("th-foot-front", "foot-front", "feet", "foot"), "feet"),
]
st200 = [
    stm("member-body", "housing", "body", "member"),
    box("member-knob", steel(member="knob", part="member", bone="knob"), (-0.015, -0.015, -0.115), (0.015, 0.015, -0.075)),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, 0.14, 0), (0.06, 0.09, 0.07)),
    tag(box("support-foot-back", SM, (-0.073, 0, -0.1005), (-0.0425, 0.0305, -0.0695), repeat={"count": 2, "translation": [0, 0, 0.17]}), tags=["foot-back"]),
    tag(box("support-foot-front", SM, (0.0425, 0, -0.1005), (0.073, 0.0305, -0.0695), repeat={"count": 2, "translation": [0, 0, 0.17]}), tags=["foot-front"]),
]
D200 = [
    text("Own 1:200 model: four 30 mm feet (two lofts with repeat), one housing loft from 15 mm to 0.242 m, a separate 30 mm top-cover slab on it, a flat handle bridge extruded from a simplified 0.31 m outline, a 30 mm inlet block, the 30 mm outlet block and a 30 mm knob stub; the eight louvre fins, the mesh plate, the belt seam, the front frame and the mode switch vanish into these masses.",
         "Eigenes 1:200-Modell: vier 30-mm-Füsse (zwei Lofts mit repeat), ein Gehäuse-Loft von 15 mm bis 0,242 m, darauf eine eigene 30-mm-Deckelplatte, eine flache Griffbrücke aus einer vereinfachten Kontur bis 0,31 m, ein 30-mm-Einlassblock, der 30-mm-Auslassblock und ein 30-mm-Knopfstummel; die acht Lamellen, die Gitterplatte, die Gürtelfuge, der Frontrahmen und der Modusschalter gehen in diesen Massen auf."),
    text("Thermal and structure at 1:200 by reference to these masses: housing and handle, the top-cover slab, the outlet block with the nozzle, the inlet block, the dial and the two foot pairs; structure keeps the housing member, the knob member (the joint is merged into it), the mass and both foot supports at 30 mm.",
         "Thermik und Tragwerk bei 1:200 als Verweis auf diese Massen: Gehäuse und Griff, die Deckelplatte, der Auslassblock mit der Düse, der Einlassblock, die Skala und die zwei Fusspaare; das Tragwerk behält Gehäusestab, Knopfstab (Gelenk darin aufgegangen), Masse und beide Fussauflager in 30 mm."),
]
write("electric-fan-heater", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
