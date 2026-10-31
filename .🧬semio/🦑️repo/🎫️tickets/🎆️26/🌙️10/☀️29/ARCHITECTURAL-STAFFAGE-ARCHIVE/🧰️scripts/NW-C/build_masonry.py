import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

W, HT = 3.0, 2.75
C = 1 / 12
OX0, OX1 = 0.875, 2.125
SILL_Y, HEAD_Y = 11 * C, 27 * C
LX0, LX1, LY1 = 0.625, 2.375, 30 * C
SX0, SX1, SY0 = 0.8, 2.2, 10 * C
DEPTH = 0.445

HOLE = [(LX0, LY1), (LX1, LY1), (LX1, HEAD_Y), (OX1, HEAD_Y), (OX1, SILL_Y), (SX1, SILL_Y), (SX1, SY0), (SX0, SY0), (SX0, SILL_Y), (OX0, SILL_Y), (OX0, HEAD_Y), (LX0, HEAD_Y)]
OUTER = [(0, 0), (W, 0), (W, HT), (0, HT)]
OPENING = [(OX0, SILL_Y), (OX1, SILL_Y), (OX1, HEAD_Y), (OX0, HEAD_Y)]
LINTEL = [(LX0, HEAD_Y), (LX1, HEAD_Y), (LX1, LY1), (LX0, LY1)]
SILL = [(SX0, SY0), (SX1, SY0), (SX1, SILL_Y), (SX0, SILL_Y)]
area = lambda ring: abs(sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))) / 2
A_LINTEL = area(LINTEL); A_SILL = area(SILL); A_OPEN = area(OPENING)
A_FIELD = W * HT - A_OPEN - A_LINTEL - A_SILL
FACE_KG = 548.0
bones = [bone("wall", None, "Cavity wall field with the opening (fixed root on the foundation)", "Zweischalige Wandscheibe mit Öffnung (feste Wurzel auf dem Fundament)", 0, 0, 0, W, fixed(),
              mass(A_FIELD * FACE_KG, 1.5, 1.35, "point", "literature", "en-1991-1-1"),
              {"areaM2": 0.175 * 1.0, "secondMomentM4": 1.0 * 0.175 ** 3 / 12, "extremeFibreM": 0.0875, "material": "block", "title": T("1 m strip of the 175 mm inner leaf", "1-m-Streifen der 175-mm-Innenschale")})]
bones.append(bone_world("lintel", "wall", "Precast concrete lintel (1.75 m, 0.25 m bearing each side)", "Betonfertigsturz (1,75 m, 0,25 m Auflager je Seite)", LX0, HEAD_Y, 0, LX1 - LX0, fixed(), bones,
                        mass(A_LINTEL * 0.215 * 2400, (LX1 - LX0) / 2, (LY1 - HEAD_Y) / 2, "point", "literature", "en-1991-1-1")))
bones.append(bone_world("sill", "wall", "Precast concrete sill", "Betonfertigteil-Fensterbank", SX0, SY0, 0, SX1 - SX0, fixed(), bones,
                        mass(A_SILL * 0.15 * 2400, (SX1 - SX0) / 2, C / 2, "point", "literature", "en-1991-1-1")))
bones.append(bone_world("load-carrier", "wall", "Beam bearing carrier on the wall head (massless)", "Trägerauflager-Läufer auf dem Wandkopf (masselos)", 0, HT, 0, 0, prismatic(0, 0, W), bones))
D = Draw(bones)
GR = lambda: D.line("ground", "wall", "ground", 0, 0.5, [(0.0, 0.0), (W, 0.0)])

def facing(role, z, tone, pen, hatch=None, **kw):
    return D.rings("field", "wall", role, z, True, tone, [OUTER, HOLE], region="masonry-field", material="brick", hatch=hatch, penMm=pen, **kw)
def lint(role, z, hatch, pen):
    return D.poly("lintel", "lintel", role, z, True, 0.35, LINTEL, True, region="lintel", material="concrete", hatch=hatch, penMm=pen)
def sillshape(role, z, hatch, pen):
    return D.poly("sill", "sill", role, z, True, 0.35, SILL, True, region="sill", material="concrete", hatch=hatch, penMm=pen)

def s500():
    sh = [GR()]
    sh.append(D.rings("field", "wall", "body", 1, True, 0.55, [OUTER, OPENING], regions=["masonry-field", "sill"], material="brick"))
    sh.append(D.poly("lintel", "lintel", "body", 2, True, 0.3, LINTEL, True, region="lintel", material="concrete"))
    return sh

def s200():
    sh = [GR(), facing("body", 1, 0.55, None), lint("body", 2, None, None), sillshape("body", 2, None, None)]
    sh[1]["penMm"] = 0.18 if False else None
    sh[1].pop("penMm", None); sh[2].pop("penMm", None); sh[3].pop("penMm", None)
    return sh

def courses():
    sh = []
    for j in range(1, 33):
        y = j * C
        if abs(y - HT) < 1e-9:
            continue
        cuts = []
        if SILL_Y - 1e-9 < y < HEAD_Y + 1e-9 and y > SILL_Y + 1e-9 and y < HEAD_Y - 1e-9:
            cuts.append((OX0, OX1))
        if abs(y - SY0) < 1e-9:
            cuts.append((SX0, SX1))
        if HEAD_Y - 1e-9 < y < LY1 - 1e-9:
            cuts.append((LX0, LX1))
        if abs(y - SILL_Y) < 1e-9:
            cuts.append((SX0, SX1))
        if abs(y - HEAD_Y) < 1e-9:
            cuts.append((LX0, LX1))
        if abs(y - LY1) < 1e-9:
            cuts.append((LX0, LX1))
        segs, x = [], 0.0
        for a, b in sorted(cuts):
            if a > x + 1e-9:
                segs.append((x, a))
            x = max(x, b)
        if x < W - 1e-9:
            segs.append((x, W))
        for n, (a, b) in enumerate(segs):
            sh.append(D.line(f"bed-{j}-{n}", "wall", "texture", 1, 0.3, [(a, y), (b, y)], region="masonry-field"))
    return sh

def s100():
    sh = [GR(), facing("body", 1, 0.8, 0.25, "coursing"), lint("body", 2, "concrete-stipple", 0.25), sillshape("body", 2, "concrete-stipple", 0.25)]
    return sh

def s50():
    sh = [GR(), facing("body", 1, 0.85, 0.35), lint("body", 2, "concrete-stipple", 0.35), sillshape("body", 2, "concrete-stipple", 0.35)]
    sh += courses()
    return sh

rng = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
reps = {}
reps["500"] = rep(500, s500(), dec(
    ("A 3.0 x 2.75 m wall mass with the 1.25 x 1.33 m opening cut out and the 0.25 m precast lintel as a darker band above it.", "Eine 3,0 x 2,75 m große Wandmasse mit ausgeschnittener Öffnung 1,25 x 1,33 m und dem 0,25 m hohen Betonfertigsturz als dunklerem Band darüber."),
    ("Wall outline, opening, the lintel with its 0.25 m bearing on both sides (1.75 m long, 0.5 mm tall).", "Wandumriss, Öffnung, der Sturz mit 0,25 m Auflager auf beiden Seiten (1,75 m lang, 0,5 mm hoch)."),
    ("The wall field and the 0.083 m sill (0.17 mm) are one mass listing both regions.", "Das Wandfeld und die 0,083 m hohe Fensterbank (0,17 mm) sind eine Masse mit beiden Regionen."),
    ("Coursing, joints, sill projection, hatching.", "Schichtung, Fugen, Fensterbankauskragung, Schraffur."),
    ("None: the lintel is drawn at its real 0.25 m height.", "Keine: der Sturz ist in seiner realen Höhe von 0,25 m gezeichnet."),
    ("None.", "Keine."), ("Mid-grey wall mass with 0.13 mm outline; lintel darker.", "Mittelgraue Wandmasse mit 0,13-mm-Kontur; Sturz dunkler."),
    ("Cells: wall mass (field and sill merged), lintel.", "Zellen: Wandmasse (Feld und Fensterbank zusammengefasst), Sturz.")),
    pens_=pens(0.13, None, None, None, 0.18), rng=rng["500"])
reps["200"] = rep(200, s200(), dec(
    ("The wall mass with the opening, the lintel band (0.25 m = 1.25 mm) and the sill band (0.083 m = 0.42 mm) as separate filled shapes.", "Die Wandmasse mit der Öffnung, das Sturzband (0,25 m = 1,25 mm) und das Fensterbankband (0,083 m = 0,42 mm) als eigene gefüllte Formen."),
    ("Lintel with bearings, sill with its 0.075 m projection each side, opening.", "Sturz mit Auflagern, Fensterbank mit 0,075 m Überstand je Seite, Öffnung."),
    ("Nothing merged.", "Nichts zusammengefasst."), ("Coursing (0.42 mm pitch would be noise), joints, hatches.", "Schichtung (0,42 mm Abstand wäre Rauschen), Fugen, Schraffuren."),
    ("None.", "Keine."), ("None: tone only.", "Keine: nur Ton."), ("Wall mid-grey, lintel and sill darker.", "Wand mittelgrau, Sturz und Fensterbank dunkler."),
    ("Cells: wall field, lintel, sill.", "Zellen: Wandfeld, Sturz, Fensterbank.")),
    pens_=pens(0.18, 0.1, None, None, 0.25), rng=rng["200"])
reps["100"] = rep(100, s100(), dec(
    ("The wall elevation with coursing hatch (courses of 0.0833 m = 0.83 mm), a stippled precast lintel and sill.", "Die Wandansicht mit Schichtenschraffur (Schichthöhe 0,0833 m = 0,83 mm), ein punktierter Betonfertigsturz und Fensterbank."),
    ("Real course height as hatch spacing, lintel 1.75 x 0.25 m, sill 1.4 x 0.083 m.", "Reale Schichthöhe als Schraffurabstand, Sturz 1,75 x 0,25 m, Fensterbank 1,4 x 0,083 m."),
    ("Nothing merged.", "Nichts zusammengefasst."), ("Head joints of the stretcher bond (0.25 m = 2.5 mm).", "Stoßfugen des Läuferverbands (0,25 m = 2,5 mm)."),
    ("None.", "Keine."), ("Coursing as lines 0.1 mm at 0.83 mm; the phase is not registered to the lintel and sill levels. Concrete as stipple 0.6 mm.", "Schichtung als 0,1-mm-Linien mit 0,83 mm; die Phase ist nicht auf Sturz- und Fensterbankhöhe abgestimmt. Beton als Punktierung mit 0,6 mm."),
    ("Outline 0.25 mm; wall light, concrete darker.", "Kontur 0,25 mm; Wand hell, Beton dunkler."),
    ("Cells: wall field, lintel, sill.", "Zellen: Wandfeld, Sturz, Fensterbank.")),
    hatches=[H("coursing", "Brick courses", "Ziegelschichten", "lines", 0, C * 10, 0.1), H("concrete-stipple", "Precast concrete", "Betonfertigteil", "stipple", 0, 0.6, 0.1)], pens_=pens(0.25, 0.13, 0.1, None, 0.35), rng=rng["100"])
reps["50"] = rep(50, s50(), dec(
    ("The wall elevation with every bed joint drawn at its real height (32 courses of 0.0833 m), interrupted by the opening, the lintel and the sill, which register with the courses exactly.", "Die Wandansicht mit jeder Lagerfuge in ihrer realen Höhe (32 Schichten zu 0,0833 m), unterbrochen von Öffnung, Sturz und Fensterbank, die genau auf die Schichten abgestimmt sind."),
    ("All bed joints as separate lines, opening 11 to 27 courses above the floor, lintel of three courses, sill one course.", "Alle Lagerfugen als eigene Linien, Öffnung 11 bis 27 Schichten über dem Boden, Sturz aus drei Schichten, Fensterbank eine Schicht."),
    ("Nothing merged.", "Nichts zusammengefasst."), ("Head joints (0.25 m = 5 mm, would be 330 short lines) and the cavity layers behind the face.", "Stoßfugen (0,25 m = 5 mm, wären 330 kurze Linien) und die Schichten hinter der Ansichtsfläche."),
    ("Joints are 0.1 mm lines (real mortar joint 10 mm = 0.2 mm).", "Fugen sind 0,1-mm-Linien (reale Mörtelfuge 10 mm = 0,2 mm)."),
    ("Stipple 0.8 mm for the precast concrete; the brick courses are explicit lines, not a hatch.", "Punktierung mit 0,8 mm für den Beton; die Ziegelschichten sind explizite Linien, keine Schraffur."),
    ("Outline 0.35 mm, joints 0.1 mm.", "Kontur 0,35 mm, Fugen 0,1 mm."),
    ("Cells: wall field (the bed joints belong to it), lintel, sill.", "Zellen: Wandfeld (die Lagerfugen gehören dazu), Sturz, Fensterbank.")),
    hatches=[H("concrete-stipple", "Precast concrete", "Betonfertigteil", "stipple", 0, 0.8, 0.1)], pens_=pens(0.35, 0.18, 0.1, None, 0.5), rng=rng["50"])
# the hatch of 1:200 shapes is not used
for k in ("200",):
    for s in reps[k]["geometry"]["elevation"]:
        s.pop("hatch", None)

REFS = [
    ref("en-1996-1-1", "EN 1996-1-1:2005 Eurocode 6, design of masonry structures, part 1-1 (masonry density, lintel bearing length at least 0.1 m, arching above openings; the lintel here bears 0.25 m)."),
    ref("din-4172", "DIN 4172:1955 Dimensional co-ordination in building (octametre system): 0.125 m module, 1 m = 12 courses of normal format brick (NF 240 x 115 x 71 with 12 mm joints)."),
    ref("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, part 1-1: Table A.1 densities (reinforced concrete 25 kN/m3, masonry 18 kN/m3), Table 6.2 imposed load category A 2 kN/m2, floor slab 0.20 m (5 kN/m2) plus finishes (1 kN/m2)."),
    ref("en-iso-6946", "EN ISO 6946:2017 Building components, thermal resistance and transmittance (R_si 0.13 and R_se 0.04 m2K/W, layer resistance d / lambda)."),
    ref("en-iso-10456", "EN ISO 10456:2007 Building materials, hygrothermal properties: clay brick (rho 1800) lambda 0.81 W/mK; calcium silicate block 0.79; mineral wool 0.035; gypsum plaster 0.7; reinforced concrete 2.3 W/mK; specific heat 1000 J/kgK."),
    ref("en-iso-10211", "EN ISO 10211:2017 Thermal bridges in building construction: the lintel and sill are linear bridges approximated here by equivalent layered regions."),
    ref("en-13225", "EN 13225:2013 Precast concrete products, linear structural elements; precast lintels of 0.24 m height, 1.75 m length."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, conventions for areas on sections and cuts (brick, concrete)."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, elevations of masonry walls with openings (scales 1:50 to 1:200)."),
]
mats = [
    mat("brick", "Clay facing brick NF with 10 mm mortar joints", "Klinker-Verblender NF mit 10-mm-Mörtelfugen", 0.9, 0.7, 1800, 0.81, 1000, 8e9, 15e6, "literature", "en-iso-10456"),
    mat("block", "Calcium silicate block 175 mm", "Kalksandstein 175 mm", 0.9, 0.5, 1800, 0.79, 1000, 8e9, 15e6, "literature", "en-iso-10456"),
    mat("concrete", "Precast reinforced concrete", "Stahlbetonfertigteil", 0.9, 0.65, 2400, 2.3, 1000, 31e9, 40e6, "literature", "en-iso-10456"),
    mat("wool", "Mineral wool 035", "Mineralwolle 035", 0.9, 0.5, 40, 0.035, 1030, None, None, "literature", "en-iso-10456"),
    mat("plaster", "Gypsum plaster", "Gipsputz", 0.9, 0.5, 1300, 0.7, 1000, None, None, "literature", "en-iso-10456"),
]
def L(id, en, de, kind, m, d, lam, rho, c, ref_="en-iso-10456", R=None):
    return layer(id, en, de, kind, m, d, R if R is not None else d / lam, "literature", ref_, c=d * rho * c)
wall_layers = [L("plaster", "Gypsum plaster 15 mm", "Gipsputz 15 mm", "masonry", "plaster", 0.015, 0.7, 1300, 1000),
               L("inner-leaf", "Calcium silicate 175 mm", "Kalksandstein 175 mm", "masonry", "block", 0.175, 0.79, 1800, 1000),
               L("fill", "Mineral wool 140 mm (full-fill cavity)", "Mineralwolle 140 mm (Kerndämmung)", "insulation", "wool", 0.14, 0.035, 40, 1030),
               L("facing", "Clay facing brick 115 mm", "Klinker-Verblender 115 mm", "masonry", "brick", 0.115, 0.81, 1800, 1000)]
lintel_layers = [L("plaster", "Gypsum plaster 15 mm", "Gipsputz 15 mm", "masonry", "plaster", 0.015, 0.7, 1300, 1000),
                 L("lintel-inner", "Precast concrete 175 mm", "Beton-Fertigteil 175 mm", "masonry", "concrete", 0.175, 2.3, 2400, 1000),
                 L("lintel-ins", "Insulation insert 80 mm", "Dämmeinlage 80 mm", "insulation", "wool", 0.08, 0.035, 40, 1030),
                 L("lintel-shell", "Facing shell 40 mm", "Vorsatzschale 40 mm", "masonry", "concrete", 0.04, 2.3, 2400, 1000)]
sill_layers = [L("plaster", "Gypsum plaster 15 mm", "Gipsputz 15 mm", "masonry", "plaster", 0.015, 0.7, 1300, 1000),
               L("sill-block", "Calcium silicate 175 mm", "Kalksandstein 175 mm", "masonry", "block", 0.175, 0.79, 1800, 1000),
               L("sill-ins", "Insulation 80 mm", "Dämmung 80 mm", "insulation", "wool", 0.08, 0.035, 40, 1030),
               L("sill-concrete", "Precast sill 70 mm", "Fertigteil-Fensterbank 70 mm", "masonry", "concrete", 0.07, 2.3, 2400, 1000)]
def cap(layers): return sum(l["heatCapacityJM2K"] for l in layers[-2:])
regions = [
    region("masonry-field", "Facing brick wall field", "Verblend-Wandfeld", "wall", "conditioned", "brick", A_FIELD, 2.0, "vertical-plate", 0.5, wall_layers, cap(wall_layers), "literature", "en-iso-6946", conditioned=(20, 0.13)),
    region("lintel", "Precast insulated lintel", "Gedämmter Betonfertigsturz", "wall", "conditioned", "concrete", A_LINTEL, 0.25, "vertical-plate", 0.5, lintel_layers, cap(lintel_layers), "literature", "en-iso-10211", conditioned=(20, 0.13)),
    region("sill", "Precast sill zone", "Fensterbankzone", "wall", "conditioned", "concrete", A_SILL, 0.083, "vertical-plate", 0.5, sill_layers, cap(sill_layers), "literature", "en-iso-10211", conditioned=(20, 0.13)),
]
U = lambda ls: 1 / (0.13 + sum(l["resistanceM2KW"] for l in ls) + 0.04)
anchors = [anchor(f"found-{i}", "wall", x, 0, "support", f"Foundation bearing {i + 1}", f"Fundamentauflager {i + 1}", support="contact", normal=90, fl=0.5, fw=0.445, mu=0.6) for i, x in enumerate((0.4, 1.5, 2.6))]
anchors += [anchor("beam-bearing", "load-carrier", 0, 0, "load", "Floor beam bearing on the wall head", "Auflager des Deckenträgers auf dem Wandkopf")]
def regs(f=1.0, l=1.0, s=1.0):
    return {"masonry-field": RB(1.0, f), "lintel": RB(1.0, l), "sill": RB(1.0, s)}
line = load("floor-line", "imposed", "Floor slab reaction 18 kN/m (g 6 + q 2 kN/m2 over half of 4.5 m span)", "Deckenauflager 18 kN/m (g 6 + q 2 kN/m2 über halbe Spannweite von 4,5 m)", 0, 0, "literature", "en-1991-1-1", bone_="wall", x=0, y=0,
            dist={"bone": "wall", "fromM": 0, "toM": W, "wxNPerM": 0, "wyNPerM": -18000})
beam = load("beam", "equipment", "Steel beam reaction 30 kN on the wall head", "Stahlträgerauflager 30 kN auf dem Wandkopf", 0, -30000, "authored-estimate", anchor_="beam-bearing")
poses = [
    pose("dead-load", "Self-weight, sunlit", "Eigengewicht, besonnt", "The wall under its own weight in full south sun.", "Die Wand unter Eigengewicht in voller Südsonne.", {}, [], regs(0.9, 0.9, 0.9)),
    pose("floor-line-load", "Under the floor slab", "Unter der Geschossdecke", "The slab above loads the wall head with 18 kN/m (dead plus category A imposed).", "Die Decke darüber belastet den Wandkopf mit 18 kN/m (ständig plus Nutzlast Kategorie A).", {}, [line], regs(0.9, 0.9, 0.9)),
    pose("beam-over-opening", "Beam bearing above the opening", "Trägerauflager über der Öffnung", "A 30 kN steel beam bears on the wall head above the opening centre; the lintel carries it by arching.", "Ein 30-kN-Stahlträger lagert auf dem Wandkopf über der Öffnungsmitte; der Sturz trägt ihn durch Gewölbewirkung.", {"load-carrier": {"dx": 1.5}}, [beam], regs(0.9, 0.9, 0.9)),
    pose("shaded-morning", "Shaded by the neighbour", "Vom Nachbarn verschattet", "No load; the facade is in the shadow of the neighbouring building, only diffuse sky light.", "Keine Last; die Fassade liegt im Schatten des Nachbargebäudes, nur diffuses Himmelslicht.", {}, [], regs(0.0, 0.0, 0.0)),
]
shade = lambda rid, a, b: track(f"region:{rid}:sunlitFraction", [(0, a), (6, b), (12, a)])
clips = [
    clip("load-build-up", "Floor load builds up", "Deckenlast baut sich auf", "The floor line load rises from nothing to 18 kN/m on the wall head.", "Die Deckenlinienlast steigt von null auf 18 kN/m am Wandkopf.",
         6, False, [key(0, "floor-line-load"), key(6, "floor-line-load")], [phase("ramp", 0, 4, "Ramp", "Aufbau"), phase("hold", 4, 6, "Hold", "Halten")], [event(4, "load-on", "Full line load", "Volle Linienlast")], [track("load:floor-line", [(0, 0), (4, 1), (6, 1)])]),
    clip("beam-sweep", "Beam bearing moves along the head", "Trägerauflager wandert am Wandkopf", "A 30 kN beam reaction moves from the left end over the opening to the right end of the wall head.", "Eine Trägerlast von 30 kN wandert vom linken Ende über die Öffnung zum rechten Ende des Wandkopfes.",
         8, False, [key(0, "beam-over-opening", bones={"load-carrier": {"dx": 0.2}}), key(8, "beam-over-opening", bones={"load-carrier": {"dx": 2.8}})],
         [phase("sweep", 0, 8, "Sweep", "Überfahrt")], [event(4, "motion", "Beam above the opening", "Träger über der Öffnung")]),
    clip("sun-and-shade", "Neighbour's shadow crosses the facade", "Schatten des Nachbarn zieht über die Fassade", "The shadow of the neighbouring building crosses the wall: the sunlit fractions of field, lintel and sill dip and recover.", "Der Schatten des Nachbargebäudes zieht über die Wand: die Besonnungsanteile von Feld, Sturz und Fensterbank sinken und erholen sich.",
         12, True, [key(0, "dead-load"), key(12, "dead-load")], [phase("shadow", 0, 12, "Shadow", "Schatten")], [event(6, "thermal", "Deepest shade", "Tiefster Schatten")],
         [shade("masonry-field", 0.9, 0.05), shade("lintel", 0.9, 0.05), shade("sill", 0.9, 0.05)]),
]
a = {
    "id": "masonry-wall-with-lintel", "category": "components",
    "title": T("Masonry Wall with Precast Lintel", "Mauerwerkswand mit Betonfertigsturz"),
    "summary": T(f"A 3.0 m wide, 2.75 m high full-fill cavity wall (175 mm calcium silicate, 140 mm mineral wool, 115 mm facing brick, U about {U(wall_layers):.2f} W/m2K) in elevation with a 1.25 x 1.33 m opening, a precast insulated concrete lintel with 0.25 m bearings (U about {U(lintel_layers):.2f}) and a precast sill; brick courses of 0.0833 m drawn where printable; floor slab and beam loads with arching over the opening.",
                 f"Eine 3,0 m breite, 2,75 m hohe zweischalige Wand mit Kerndämmung (175 mm Kalksandstein, 140 mm Mineralwolle, 115 mm Verblender, U etwa {U(wall_layers):.2f} W/m2K) in der Ansicht mit einer Öffnung von 1,25 x 1,33 m, einem gedämmten Betonfertigsturz mit 0,25 m Auflager (U etwa {U(lintel_layers):.2f}) und einer Fertigteil-Fensterbank; Ziegelschichten von 0,0833 m dort gezeichnet, wo sie druckbar sind; Decken- und Trägerlasten mit Gewölbewirkung über der Öffnung."),
    "tags": ["component", "structural", "wall", "masonry", "lintel", "opening", "conditioned"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Octametre dimensions: 32 courses of 0.0833 m, opening 11 to 27 courses high, lintel 27 to 30; widths are multiples of 0.125 m. Layer resistances are d / lambda with design values.", "Oktametermaße: 32 Schichten zu 0,0833 m, Öffnung 11 bis 27 Schichten hoch, Sturz 27 bis 30; Breiten sind Vielfache von 0,125 m. Schichtwiderstände sind d / lambda mit Bemessungswerten.")},
    "materials": mats,
    "rig": {"kind": "structural", "root": "wall", "bones": bones},
    "anchors": anchors, "representations": reps, "symbols": [],
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(0, 0, -12, 2, 0.8, 3, 300, 0.25), "operating": [], "regions": regions, "emitters": [], "passengers": [],
                "assumptions": [
                    T(f"Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2; interior 20 degC with R_si 0.13. U of the field {U(wall_layers):.2f}, lintel {U(lintel_layers):.2f}, sill zone {U(sill_layers):.2f} W/m2K.", f"Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m2; Innenraum 20 °C mit R_si 0,13. U des Feldes {U(wall_layers):.2f}, des Sturzes {U(lintel_layers):.2f}, der Fensterbankzone {U(sill_layers):.2f} W/m2K."),
                    T("The lintel and sill are linear thermal bridges; they are modelled as separate layered regions with equivalent resistances (authored estimate after EN ISO 10211), not by a 2D calculation.", "Sturz und Fensterbank sind lineare Wärmebrücken; sie sind als eigene geschichtete Regionen mit äquivalenten Widerständen modelliert (Schätzung nach EN ISO 10211), nicht durch eine 2D-Berechnung."),
                    T("The opening is empty (no window modelled); its reveals are not regions. Sun and shade come from the pose sunlit fractions.", "Die Öffnung ist leer (kein Fenster modelliert); ihre Laibungen sind keine Regionen. Sonne und Schatten kommen aus den Besonnungsanteilen der Posen.")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [x["id"] for x in anchors if x.get("support")],
                  "assumptions": [T("The wall field, lintel and sill form one rigid body bearing on three foundation contacts (friction 0.6). Arching action above the opening is not resolved: the body transmits the head load straight to the contacts.", "Wandfeld, Sturz und Fensterbank bilden einen starren Körper auf drei Fundamentkontakten (Reibung 0,6). Die Gewölbewirkung über der Öffnung wird nicht aufgelöst: der Körper leitet die Kopflast direkt in die Kontakte."),
                                T("Floor load: slab 0.20 m (5 kN/m2) plus finishes 1 kN/m2 plus category A 2 kN/m2 on half of a 4.5 m span gives 18 kN/m; the beam reaction of 30 kN is an authored estimate.", "Deckenlast: Platte 0,20 m (5 kN/m2) plus Ausbau 1 kN/m2 plus Kategorie A 2 kN/m2 auf der halben Spannweite von 4,5 m ergibt 18 kN/m; die Trägerlast von 30 kN ist ein Schätzwert.")], "clinical": False},
    "presentation": {"defaultPose": "dead-load", "defaultClip": "beam-sweep", "defaultScale": 50, "viewpoints": ["elevation"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": DEPTH,
}
assemble(a, "masonry-wall-with-lintel")
