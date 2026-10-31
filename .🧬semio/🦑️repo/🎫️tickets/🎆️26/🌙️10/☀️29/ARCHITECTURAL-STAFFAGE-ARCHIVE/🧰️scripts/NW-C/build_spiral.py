import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

R = 0.80
RC = 0.0842
RISE = 2.80 / 15
NT = 15
DEL = 11.25
TT = 0.06
RH = 0.775
RAILH = 5 * RISE
RT = [0.0 for _ in range(NT)]
def phi(k): return 11.25 + 22.5 * (k - 1)
def yk(k): return RISE * k
def cos(d): return math.cos(math.radians(d))
def sin(d): return math.sin(math.radians(d))
def extent(k):
    xs = [r * cos(phi(k) + s * DEL) for r in (RC, R) for s in (-1, 1)]
    return min(xs), max(xs)
def rail_y(p):
    return RISE + (p - phi(1)) / 22.5 * RISE + RAILH
def rail_pts(step=2.8125):
    pts = []
    p = phi(1)
    while p < phi(NT) - 1e-9:
        pts.append((RH * cos(p), rail_y(p)))
        p += step
    pts.append((RH * cos(phi(NT)), rail_y(phi(NT))))
    return pts
RAIL = rail_pts()
RAIL_L = sum(math.hypot(RH * math.radians(22.5) / 1, RISE) for _ in range(NT - 1))
TREAD_KG = 8.0
COL_KG = 25.2 * 2.8 + 8.0
RAILING_KG = RAIL_L * 2.55 + NT * 0.93 * 0.89

bones = [bone("column", None, "Central column CHS 168.3 x 6.3 on its base plate (fixed root)", "Mittelsäule CHS 168,3 x 6,3 auf der Fußplatte (feste Wurzel)", 0, 0, 90, 2.8, fixed(),
              mass(COL_KG, 1.4, 0, "uniform", "literature", "en-10219"),
              {"areaM2": 0.00321, "secondMomentM4": 1.068e-5, "extremeFibreM": 0.0842, "material": "steel", "title": T("CHS 168.3 x 6.3", "CHS 168,3 x 6,3")})]
for k in range(1, NT + 1):
    bones.append(bone_world(f"tread-{k}", "column", f"Tread {k} (pie plate on the column)", f"Stufe {k} (Kreisringsektor an der Säule)", 0, yk(k), 0, R, fixed(), bones,
                            mass(TREAD_KG, 0.528 * cos(phi(k)), 0, "point", "authored-estimate", "spiral-catalogues")))
bones.append(bone_world("balustrade", "column", "Balustrade: helical handrail and 15 balusters", "Geländer: Wendelhandlauf und 15 Stäbe", 0, 0, 0, 1.0, fixed(), bones,
                        mass(RAILING_KG, 0, rail_y(phi(8)) + 0.0, "point", "authored-estimate", "spiral-catalogues")))
bones.append(bone_world("walker-x", "column", "Walker carrier, horizontal (massless)", "Personenträger, horizontal (masselos)", 0, 0, 0, 0, prismatic(0, -0.7, 0.7), bones))
bones.append(bone_world("walker-y", "walker-x", "Walker carrier, vertical (massless)", "Personenträger, vertikal (masselos)", 0, 0, 0, 0, prismatic(90, 0, 2.9), bones))
D = Draw(bones)
GR = lambda: D.line("ground", "column", "ground", 0, 0.5, [(-0.8, 0.0), (0.8, 0.0)])

def silhouette500():
    rt, lt = [(0.2, 0.0), (0.2, 0.02)], []
    for k in range(1, NT + 1):
        a, b = extent(k)
        rt.append((max(b, RC), yk(k) - TT / 2))
        lt.append((min(a, -RC), yk(k) - TT / 2))
    top = [(RC, 2.82), (-RC, 2.82)]
    return rt + top + lt[::-1] + [(-0.2, 0.02), (-0.2, 0.0)]

def s500():
    sh = [GR()]
    sh.append(D.poly("stair-mass", "column", "body", 1, True, 0.55, silhouette500(), True, regions=["treads", "column-steel"], material="steel"))
    sh.append(D.line("handrail", "balustrade", "body", 3, 0.3, RAIL, region="balustrade-steel", material="steel", penMm=0.13))
    return sh

def bars(role, z, tone, pen=None, joints=False):
    sh = []
    for k in range(1, NT + 1):
        a, b = extent(k)
        sh.append(D.rect(f"tread-{k}", f"tread-{k}", role, z, True, tone, a, yk(k) - TT, b, yk(k), region="treads", material="steel", penMm=pen))
    return sh

def s200():
    sh = [GR()]
    sh.append(D.rect("column", "column", "body", 1, True, 0.5, -RC, 0.0, RC, 2.8, region="column-steel", material="steel"))
    sh.append(D.rect("base-plate", "column", "body", 1, True, 0.5, -0.2, 0.0, 0.2, 0.02, region="column-steel", material="steel"))
    sh += bars("body", 2, 0.5, 0.13)
    sh.append(D.line("handrail", "balustrade", "detail", 4, 0.3, RAIL, region="balustrade-steel", material="steel", penMm=0.21))
    return sh

def balusters(pen):
    return [D.line(f"baluster-{k}", "balustrade", "detail", 3, 0.3, [(RH * cos(phi(k)), yk(k)), (RH * cos(phi(k)), yk(k) + RAILH)], region="balustrade-steel", material="steel", penMm=pen) for k in range(1, NT + 1)]

def s100():
    sh = [GR()]
    sh.append(D.rect("column", "column", "body", 1, True, 0.5, -RC, 0.0, RC, 2.8, region="column-steel", material="steel"))
    sh.append(D.rect("base-plate", "column", "body", 1, True, 0.5, -0.2, 0.0, 0.2, 0.02, region="column-steel", material="steel"))
    sh += bars("body", 2, 0.5, 0.18)
    sh += balusters(0.12)
    sh.append(D.line("handrail", "balustrade", "detail", 4, 0.3, RAIL, region="balustrade-steel", material="steel", penMm=0.42))
    return sh

def s50():
    sh = [GR()]
    sh.append(D.rect("column", "column", "body", 1, True, 0.5, -RC, 0.0, RC, 2.8, region="column-steel", material="steel"))
    sh.append(D.rect("base-plate", "column", "body", 1, True, 0.5, -0.2, 0.0, 0.2, 0.02, region="column-steel", material="steel"))
    for k in range(1, NT + 1):
        sh.append(D.line(f"sleeve-{k}", "column", "texture", 1, 0.2, [(-RC, yk(k) - TT), (RC, yk(k) - TT)], region="column-steel", penMm=0.1))
    sh += bars("body", 2, 0.5, 0.25)
    for k in range(1, NT + 1):
        a, b = extent(k)
        sh.append(D.line(f"nosing-{k}", f"tread-{k}", "texture", 3, 0.2, [(a, yk(k) - 0.012), (b, yk(k) - 0.012)], region="treads", penMm=0.1))
    sh += balusters(0.24)
    sh.append(D.line("handrail", "balustrade", "detail", 4, 0.3, RAIL, region="balustrade-steel", material="steel", penMm=0.85))
    return sh

reps = {}
rng = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
reps["500"] = rep(500, s500(), dec(
    ("One twisting mass: the envelope through the tread tips (x = R cos(phi), left and right) around the column, a base plate foot, and one helical handrail line above it.", "Eine sich windende Masse: die Hüllkurve durch die Stufenspitzen (x = R cos(phi), links und rechts) um die Säule, ein Fußplattensockel und darüber eine Wendelhandlauflinie."),
    ("Total height 3.70 m (rail), width 1.60 m (tread tips), the helix pattern of the tips: 15 risers of 0.1867 m, 22.5 degrees per tread.", "Gesamthöhe 3,70 m (Handlauf), Breite 1,60 m (Stufenspitzen), das Wendelmuster der Spitzen: 15 Steigungen von 0,1867 m, 22,5 Grad je Stufe."),
    ("The 15 treads (0.06 m thick, 0.13 m gaps = 0.27 mm) and the column are one filled mass listing treads and column; the notches between treads close because the bars are thinner than 0.2 mm.", "Die 15 Stufen (0,06 m dick, 0,13 m Lücken = 0,27 mm) und die Säule sind eine gefüllte Masse mit den Regionen Stufen und Säule; die Einkerbungen zwischen den Stufen schließen sich, weil die Balken dünner als 0,2 mm wären."),
    ("Balusters, sleeve joints, nosings.", "Stäbe, Hülsenfugen, Stufenkanten."),
    ("The 0.0424 m handrail is a 0.13 mm line (0.065 m, wider than real).", "Der 0,0424 m dicke Handlauf ist eine 0,13-mm-Linie (0,065 m, breiter als real)."),
    ("None.", "Keine."), ("Mid-grey mass, black rail line.", "Mittelgraue Masse, schwarze Handlauflinie."),
    ("Cells: the stair mass (treads and column merged) and the rail line.", "Zellen: die Treppenmasse (Stufen und Säule zusammengefasst) und die Handlauflinie.")),
    pens_=pens(0.13, None, None, None, 0.18), rng=rng["500"])
reps["200"] = rep(200, s200(), dec(
    ("The column as a filled bar on its base plate with 15 tread bars whose projected lengths follow the helix, and the handrail as a sine line.", "Die Säule als gefüllter Balken auf der Fußplatte mit 15 Stufenbalken, deren Projektionslängen der Wendel folgen, und der Handlauf als Sinuslinie."),
    ("Every tread bar (0.06 m = 0.3 mm thick) with its true projected extent, column 0.168 m, base plate 0.4 m, rail at 0.0424 m.", "Jeder Stufenbalken (0,06 m = 0,3 mm dick) mit seiner wahren Projektionslänge, Säule 0,168 m, Fußplatte 0,4 m, Handlauf 0,0424 m."),
    ("Tread and column outlines join where bars cross the column.", "Stufen- und Säulenkonturen verbinden sich, wo die Balken die Säule kreuzen."),
    ("The 15 balusters (0.012 m = 0.06 mm), sleeve joints, nosings.", "Die 15 Stäbe (0,012 m = 0,06 mm), Hülsenfugen, Stufenkanten."),
    ("The rail is a 0.21 mm line, equal to its real 0.0424 m.", "Der Handlauf ist eine 0,21-mm-Linie, gleich seinen realen 0,0424 m."),
    ("None.", "Keine."), ("Bars and column 0.13 mm outline, rail 0.21 mm.", "Balken und Säule 0,13-mm-Kontur, Handlauf 0,21 mm."),
    ("Cells: the 15 treads (one region), the column with its plate, the rail.", "Zellen: die 15 Stufen (eine Region), die Säule mit Platte, der Handlauf.")),
    pens_=pens(0.18, 0.1, None, None, 0.25), rng=rng["200"])
reps["100"] = rep(100, s100(), dec(
    ("Tread bars with the helical projection, the column, 15 balusters and the rail at its real width.", "Stufenbalken mit der Wendelprojektion, die Säule, 15 Stäbe und der Handlauf in echter Breite."),
    ("All 15 balusters at 0.12 mm (real 0.012 m), the rail at 0.42 mm (real 0.0424 m).", "Alle 15 Stäbe mit 0,12 mm (real 0,012 m), der Handlauf mit 0,42 mm (real 0,0424 m)."),
    ("Nothing merged.", "Nichts zusammengefasst."), ("Sleeve joints, nosings.", "Hülsenfugen, Stufenkanten."),
    ("Balusters and rail are lines whose pen equals the real width.", "Stäbe und Handlauf sind Linien, deren Stift der realen Breite entspricht."),
    ("None.", "Keine."), ("Bars 0.18 mm outline, balusters 0.12 mm, rail 0.42 mm.", "Balken 0,18-mm-Kontur, Stäbe 0,12 mm, Handlauf 0,42 mm."),
    ("Cells: treads, column, balusters and rail.", "Zellen: Stufen, Säule, Stäbe und Handlauf.")),
    pens_=pens(0.25, 0.13, 0.1, None, 0.35), rng=rng["100"])
reps["50"] = rep(50, s50(), dec(
    ("The full front elevation of the helix: column with sleeve joints under every tread, 15 tread bars with nosing lines, 15 balusters and the handrail band.", "Die volle Vorderansicht der Wendel: Säule mit Hülsenfuge unter jeder Stufe, 15 Stufenbalken mit Kantenlinie, 15 Stäbe und das Handlaufband."),
    ("Sleeve joints (column built of 0.127 m spacer sleeves), nosing lines at 0.012 m below the tread top, balusters at 0.24 mm, the rail at 0.85 mm.", "Hülsenfugen (Säule aus 0,127 m langen Abstandshülsen), Kantenlinien 0,012 m unter der Trittfläche, Stäbe mit 0,24 mm, der Handlauf mit 0,85 mm."),
    ("Nothing merged.", "Nichts zusammengefasst."), ("Anchors, bolts, tread riser lips.", "Anker, Schrauben, Setzstufenlippen."),
    ("All thin parts at real width.", "Alle dünnen Teile in echter Breite."),
    ("None.", "Keine."), ("Bars 0.25 mm, balusters 0.24 mm, rail 0.85 mm, joints 0.1 mm.", "Balken 0,25 mm, Stäbe 0,24 mm, Handlauf 0,85 mm, Fugen 0,1 mm."),
    ("One cell per region; each tread is a cell of the treads region.", "Eine Zelle je Region; jede Stufe ist eine Zelle der Region Stufen.")),
    pens_=pens(0.35, 0.18, 0.1, None, 0.5), rng=rng["50"])

# plan symbol
def wedge(k):
    pts = [(RC * cos(phi(k) - DEL), RC * sin(phi(k) - DEL))]
    n = 6
    pts += [(R * cos(phi(k) - DEL + 2 * DEL * i / n), R * sin(phi(k) - DEL + 2 * DEL * i / n)) for i in range(n + 1)]
    pts.append((RC * cos(phi(k) + DEL), RC * sin(phi(k) + DEL)))
    return pts
plan = [{"id": f"tread-plan-{k}", "bone": "column", "role": "body", "z": 1, "filled": False, "tone": 0.5, "kind": "poly", "points": [[round(x, 5), round(y, 5)] for x, y in wedge(k)], "closed": True, "region": "treads"} for k in range(1, NT + 1)]
plan.append({"id": "column-plan", "bone": "column", "role": "body", "z": 2, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": 0, "cy": 0, "rx": RC, "ry": RC, "region": "column-steel"})
walk = [(0.6 * cos(phi(1) - DEL + t * 5), 0.6 * sin(phi(1) - DEL + t * 5)) for t in range(0, int((phi(NT) + DEL - phi(1) + DEL) / 5) + 1)]
plan.append({"id": "walking-line", "bone": "column", "role": "detail", "z": 3, "filled": False, "tone": 0.4, "kind": "poly", "points": [[round(x, 5), round(y, 5)] for x, y in walk], "closed": False, "region": "treads", "penMm": 0.13})
plan.append({"id": "balustrade-circle", "bone": "column", "role": "detail", "z": 3, "filled": False, "tone": 0.4, "kind": "ellipse", "cx": 0, "cy": 0, "rx": RH, "ry": RH, "region": "balustrade-steel"})
symbols = [sym("spiral-stair-plan", "Spiral stair in plan", "Wendeltreppe im Grundriss",
               "The conventional plan of a spiral stair: 15 pie treads around the column in true size, the walking line at 0.6 m radius and the balustrade circle (DIN 1356-1, DIN 18065).",
               "Der übliche Grundriss einer Wendeltreppe: 15 Kreisringstufen um die Säule in wahrer Größe, die Lauflinie bei 0,6 m Radius und der Geländerkreis (DIN 1356-1, DIN 18065).",
               "true-extent", "model-m", [200, 100, 50], pens(0.18, 0.13, None, None, 0.25), [], [], plan,
               "Model metres at real size: the 1.6 m diameter and the 22.5 degree tread angle are true.", "Modellmeter in wahrer Größe: der Durchmesser 1,6 m und der Stufenwinkel 22,5 Grad sind echt.")]

REFS = [
    ref("din-18065", "DIN 18065:2015-03 Stairs in buildings, terms, measuring rules, main dimensions (spiral stairs: usable tread width at the walking line 0.15 m inside the outer edge, 2 s + g about 0.59 to 0.65 m, handrail height at least 0.90 m)."),
    ref("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, part 1-1: Table 6.2 category A imposed load on stairs 2 kN/m2 with 2 kN concentrated load; Table 6.12 line load on barriers (category A, 0.5 to 1.0 kN/m, 0.5 kN/m chosen); Table A.4 steel 77 to 78.5 kN/m3."),
    ref("en-10219", "EN 10219-2:2019 Cold formed welded structural hollow sections: CHS 168.3 x 6.3 (25.2 kg/m, A 32.1 cm2, I 1068 cm4); CHS 42.4 x 2.6 (2.55 kg/m)."),
    ref("spiral-catalogues", "Manufacturer data for steel spiral stairs: column 168 mm tube with spacer sleeves, 5 mm pie treads with folded edge (about 8 kg each), balusters 12 mm, handrail tube 42.4 mm, 16 steps per revolution for 2.8 m height. Typical values consulted."),
    ref("en-iso-10456", "EN ISO 10456:2007 Building materials, hygrothermal properties: steel 50 W/mK, 450 J/kgK; emissivity of painted surfaces 0.9."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, types of drawings, scales and symbols (stair plan symbols)."),
]
mats = [
    mat("steel", "Painted steel RAL 7016", "Stahl, lackiert RAL 7016", 0.9, 0.75, 7850, 50, 450, 210e9, 235e6, "literature", "en-10219"),
    mat("steel-bright", "Galvanised steel tube (rail and balusters)", "Verzinktes Stahlrohr (Handlauf und Stäbe)", 0.3, 0.55, 7850, 50, 450, 210e9, 235e6, "literature", "en-10219"),
]
regions = [
    region("treads", "Pie treads, top and underside", "Kreisringstufen, Ober- und Unterseite", "surface", "passive", "steel", 2 * NT * 0.1243, 0.7, "horizontal-plate-up", 0.7,
           [layer("tread-plate", "Steel plate 5 mm", "Stahlblech 5 mm", "metal", "steel", 0.005, 0.005 / 50, "literature", "en-iso-10456", c=0.005 * 7850 * 450)], 17700, "literature", "en-iso-10456"),
    region("column-steel", "Column with sleeves and base plate", "Säule mit Hülsen und Fußplatte", "surface", "passive", "steel", math.pi * 0.1683 * 2.8 + 0.8, 0.168, "cylinder", 0.8,
           [layer("column-wall", "Steel tube wall 6.3 mm", "Stahlrohrwand 6,3 mm", "metal", "steel", 0.0063, 0.0063 / 50, "literature", "en-iso-10456", c=0.0063 * 7850 * 450)], 22200, "literature", "en-10219"),
    region("balustrade-steel", "Handrail and balusters", "Handlauf und Stäbe", "surface", "passive", "steel-bright", RAIL_L * math.pi * 0.0424 + NT * 0.93 * math.pi * 0.012, 0.0424, "cylinder", 0.9,
           [layer("tube-wall", "Steel tube wall 2.6 mm", "Stahlrohrwand 2,6 mm", "metal", "steel-bright", 0.0026, 0.0026 / 50, "literature", "en-iso-10456", c=0.0026 * 7850 * 450)], 9200, "literature", "en-10219"),
]
anchors = [
    anchor("base-fix", "column", 0, 0, "support", "Base plate anchored to the floor, fixed", "Fußplatte am Boden verankert, eingespannt", support="fixed"),
    anchor("top-tie", "column", 2.8, 0, "support", "Tie at the upper floor slab, horizontal roller", "Anbindung an die obere Decke, horizontales Rollenlager", support="roller", normal=0),
    anchor("walker-point", "walker-y", 0, 0, "load", "Foot of the person on the tread", "Fußpunkt der Person auf der Stufe"),
    anchor("rail-point", "balustrade", 0.5, rail_y(phi(NT)), "load", "Handrail at the top landing", "Handlauf am oberen Podest"),
]
def regs(over=None):
    base = {"treads": (1.0, 0.45), "column-steel": (1.0, 0.35), "balustrade-steel": (1.0, 0.5)}
    base.update(over or {})
    return {k: RB(*v) for k, v in base.items()}
def at(k):
    return {"walker-x": {"dx": 0.6 * cos(phi(k))}, "walker-y": {"dy": yk(k)}}
walker = load("walker", "occupant", "Person of 80 kg on the tread", "Person mit 80 kg auf der Stufe", 0, -785, "authored-estimate", anchor_="walker-point")
qk = load("concentrated", "imposed", "Concentrated load 2 kN on the top tread (category A)", "Einzellast 2 kN auf der obersten Stufe (Kategorie A)", 0, -2000, "literature", "en-1991-1-1", anchor_="walker-point")
push = load("rail-push", "imposed", "Barrier line load 0.5 kN/m over 1 m, horizontal", "Geländer-Linienlast 0,5 kN/m auf 1 m, waagerecht", 500, 0, "literature", "en-1991-1-1", anchor_="rail-point")
poses = [
    pose("self-weight", "Self-weight", "Eigengewicht", "The stair unloaded; treads and rail partly in the sun.", "Die unbelastete Treppe; Stufen und Handlauf teilweise besonnt.", {}, [], regs()),
    pose("walker-on-tread-8", "Person on the eighth tread", "Person auf der achten Stufe", "An 80 kg person on tread 8, on the walking line at 0.6 m radius, 168.75 degrees round the column.", "Eine 80 kg schwere Person auf Stufe 8, auf der Lauflinie bei 0,6 m Radius, 168,75 Grad um die Säule.", at(8), [walker], regs()),
    pose("point-load-top-tread", "2 kN on the top tread", "2 kN auf der obersten Stufe", "The category A concentrated load on the outer part of the top tread.", "Die Einzellast der Kategorie A auf dem äußeren Teil der obersten Stufe.", at(15), [qk], regs({"treads": (1.0, 0.8)})),
    pose("rail-push", "Pushing on the balustrade", "Druck auf das Geländer", "A horizontal 0.5 kN barrier load at the handrail of the top landing (shaded side).", "Eine waagerechte Geländerlast von 0,5 kN am Handlauf des oberen Podests (verschattete Seite).", {}, [push], regs({"balustrade-steel": (1.0, 0.1), "treads": (1.0, 0.1)})),
]
climb_keys = [key(k - 1, "walker-on-tread-8", bones=at(k)) for k in range(1, NT + 1)]
shade = lambda rid, a, b: track(f"region:{rid}:sunlitFraction", [(0, a), (4, b), (8, a)])
clips = [
    clip("climb", "Person climbs the spiral", "Person steigt die Wendel hinauf", "An 80 kg person walks up the 15 treads on the walking line; in elevation the foot swings left and right while rising 0.187 m per step.", "Eine 80 kg schwere Person geht die 15 Stufen auf der Lauflinie hinauf; in der Ansicht schwingt der Fuß nach links und rechts und steigt 0,187 m je Schritt.",
         14, False, climb_keys, [phase("climb", 0, 14, "Climb at 1 step per second", "Aufstieg mit einem Schritt pro Sekunde")],
         [event(0, "load-on", "Person on tread 1", "Person auf Stufe 1"), event(14, "load-off", "Person on the top tread", "Person auf der obersten Stufe")]),
    clip("live-load-ramp", "Concentrated load builds up", "Einzellast baut sich auf", "The 2 kN concentrated load rises on the top tread.", "Die Einzellast von 2 kN steigt auf der obersten Stufe.",
         6, False, [key(0, "point-load-top-tread"), key(6, "point-load-top-tread")], [phase("ramp", 0, 4, "Ramp", "Aufbau"), phase("hold", 4, 6, "Hold", "Halten")],
         [event(4, "load-on", "Full load", "Volle Last")], [track("load:concentrated", [(0, 0), (4, 1), (6, 1)])]),
    clip("sun-and-shade", "Sun patch crosses the stair", "Sonnenfleck wandert über die Treppe", "A window sun patch crosses the stair: treads, column and rail sunlit fractions dip and recover.", "Ein Fenster-Sonnenfleck wandert über die Treppe: die Besonnungsanteile von Stufen, Säule und Handlauf sinken und erholen sich.",
         8, True, [key(0, "self-weight"), key(8, "self-weight")], [phase("patch", 0, 8, "Sun patch", "Sonnenfleck")], [event(4, "thermal", "Patch at the top", "Fleck oben")],
         [shade("treads", 0.45, 0.05), shade("column-steel", 0.35, 0.04), shade("balustrade-steel", 0.5, 0.05)]),
]
a = {
    "id": "spiral-stair", "category": "components",
    "title": T("Steel Spiral Stair", "Stahl-Wendeltreppe"),
    "summary": T("A 1.6 m diameter steel spiral stair around a 168 mm central column: 15 risers of 0.187 m (2.80 m floor to floor), 15 pie treads turning 22.5 degrees each, 15 balusters and a 0.93 m high helical handrail, drawn in elevation where every tread projects to a bar whose length follows the helix and the rail reads as a sine curve; a plan symbol gives the true spiral.",
                 "Eine Stahl-Wendeltreppe mit 1,6 m Durchmesser um eine 168-mm-Mittelsäule: 15 Steigungen von 0,187 m (2,80 m Geschosshöhe), 15 Kreisringstufen mit je 22,5 Grad Drehung, 15 Stäbe und ein 0,93 m hoher Wendelhandlauf, in der Ansicht gezeichnet, in der jede Stufe als Balken mit wendelfolgender Länge erscheint und der Handlauf als Sinuskurve; ein Grundrisssymbol zeigt die echte Spirale."),
    "tags": ["component", "structural", "stair", "spiral", "steel", "helix"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Tread k is centred at 11.25 + 22.5 (k-1) degrees so the tread edges fall on multiples of 22.5 degrees; the projected tread bar extent is the true min and max of the four tread corners, the rail is x = 0.775 cos(phi).", "Stufe k ist bei 11,25 + 22,5 (k-1) Grad zentriert, sodass die Stufenkanten auf Vielfachen von 22,5 Grad liegen; die Projektionslänge des Stufenbalkens ist das wahre Minimum und Maximum der vier Stufenecken, der Handlauf ist x = 0,775 cos(phi).")},
    "materials": mats,
    "rig": {"kind": "structural", "root": "column", "bones": bones},
    "anchors": anchors, "representations": reps, "symbols": symbols,
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(15, 15, -3, 18, 0.55, 0.3, 300, 0.4), "operating": [], "regions": regions, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Reference case: a stair in a sunny, draught-free stair hall (air 15 degC, indirect 300 W/m2 through the glazing); all regions are passive steel.", "Referenzfall: eine Treppe in einem sonnigen, zugfreien Treppenhaus (Luft 15 °C, indirekt 300 W/m2 durch die Verglasung); alle Regionen sind passiver Stahl."),
                    T("Sun and shade are the pose sunlit fractions; the underside of every tread is always in shade, so treads carry the area-weighted fraction of top (sunlit) and underside (shaded).", "Sonne und Schatten sind die Besonnungsanteile der Posen; die Unterseite jeder Stufe liegt immer im Schatten, die Stufen tragen daher den flächengewichteten Anteil von Oberseite (besonnt) und Unterseite (beschattet).")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": ["base-fix", "top-tie"],
                  "assumptions": [T("The column, the 15 treads and the balustrade form one rigid body in the elevation plane, fixed at the base and tied horizontally at the upper slab; the person is a point load at the foot. The 3D torsion of the spiral is not represented.", "Säule, 15 Stufen und Geländer bilden einen starren Körper in der Ansichtsebene, am Fuß eingespannt und an der oberen Decke horizontal gehalten; die Person ist eine Punktlast am Fuß. Die räumliche Torsion der Wendel ist nicht abgebildet."),
                                T("Category A (dwellings): 2 kN/m2 and 2 kN concentrated load on stairs, 0.5 kN/m on barriers; a public stair would use category C.", "Kategorie A (Wohnen): 2 kN/m2 und 2 kN Einzellast auf Treppen, 0,5 kN/m auf Geländern; eine öffentliche Treppe würde Kategorie C verwenden.")], "clinical": False},
    "presentation": {"defaultPose": "self-weight", "defaultClip": "climb", "defaultScale": 50, "viewpoints": ["elevation", "plan"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": 1.6,
}
assemble(a, "spiral-stair")
