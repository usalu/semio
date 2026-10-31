import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

G = 1 / 12
ANG = math.degrees(math.atan(G))
X = [0.0, 1.5, 5.1, 6.6, 10.2, 11.7]
Y = [0.0, 0.0, 0.30, 0.30, 0.60, 0.60]
WIDTH = 1.4
RAIL_H = 0.85
KERB = 0.10
POSTS = [1.5, 3.3, 5.1, 6.6, 8.4, 10.2]

def surf(x):
    for i in range(len(X) - 1):
        if X[i] <= x <= X[i + 1]:
            t = (x - X[i]) / (X[i + 1] - X[i])
            return Y[i] + t * (Y[i + 1] - Y[i])
    raise ValueError(x)

RAIL = [(1.2, RAIL_H), (1.5, RAIL_H), (5.1, 0.30 + RAIL_H), (6.6, 0.30 + RAIL_H), (10.2, 0.60 + RAIL_H), (10.5, 0.60 + RAIL_H)]
SLOPE_LEN = math.hypot(3.6, 0.3)
poly_area = 0.0
cx = cy = 0.0
pts = [(1.5, 0.0), (5.1, 0.3), (6.6, 0.3), (10.2, 0.6), (11.7, 0.6), (11.7, 0.0)]
for i in range(len(pts)):
    x0, y0 = pts[i]; x1, y1 = pts[(i + 1) % len(pts)]
    c = x0 * y1 - x1 * y0
    poly_area += c; cx += (x0 + x1) * c; cy += (y0 + y1) * c
poly_area /= 2; cx /= 6 * poly_area; cy /= 6 * poly_area; poly_area = abs(poly_area)
KERB_LEN = 2 * SLOPE_LEN + 1.5
BODY_KG = (poly_area * WIDTH + 2 * KERB * 0.1 * KERB_LEN) * 2500
RAIL_KG = 2 * 9.325 * 2.55 + 12 * 0.9 * 3.56 + 6

bones = [bone("body", None, "Reinforced concrete ramp body with landings (rigid root on the ground)", "Stahlbeton-Rampenkörper mit Podesten (starre Wurzel auf dem Boden)", 0, 0, 0, 11.7, fixed(),
              mass(BODY_KG, cx, cy, "point", "literature", "en-1991-1-1"),
              {"areaM2": 0.2, "secondMomentM4": 1.4 * 0.15 ** 3 / 12, "extremeFibreM": 0.075, "material": "concrete", "title": T("1.4 m wide, 150 mm minimum thickness", "1,4 m breit, 150 mm Mindestdicke")})]
bones.append(bone_world("handrail", "body", "Handrails, posts and brackets of both sides", "Handläufe, Pfosten und Halter beider Seiten", 1.2, 0, 0, 9.3, fixed(), bones,
                        mass(RAIL_KG, 4.65, 1.0, "point", "literature", "en-10219"),
                        {"areaM2": 0.000354, "secondMomentM4": 7.6e-8, "extremeFibreM": 0.0212, "material": "steel", "title": T("CHS 42.4 x 2.6 handrail tube", "Rohr 42,4 x 2,6 Handlauf")}))
bones.append(bone_world("chair-carrier", "body", "Wheelchair carrier on the second run (massless)", "Rollstuhlträger auf dem zweiten Lauf (masselos)", 6.6, 0.30, ANG, 0.0, prismatic(0, 0, SLOPE_LEN), bones))
D = Draw(bones)

def surf_line(x0, x1, dy=0.0):
    xs = sorted({x0, x1, *[x for x in X if x0 < x < x1]})
    return [(x, surf(x) + dy) for x in xs]

def band(x0, x1, y_off0, y_off1):
    top = surf_line(x0, x1, y_off1)
    bot = surf_line(x0, x1, y_off0)
    return bot + top[::-1]

def rail_band(w):
    up = [(x, y + w / 2) for x, y in RAIL]
    dn = [(x, y - w / 2) for x, y in RAIL]
    return dn + up[::-1]

GROUND = lambda: D.line("ground", "body", "ground", 0, 0.5, [(0.0, 0.0), (11.7, 0.0)])
BODY_REG = ["ramp-surface", "ramp-side"]

def s500():
    sh = [GROUND()]
    sh.append(D.poly("ramp", "body", "body", 1, True, 0.5, [(2.4, 0.0), (2.4, 0.075)] + pts[1:], True, regions=["ramp-surface", "ramp-side", "kerb"], material="concrete"))
    sh.append(D.line("handrail", "handrail", "body", 2, 0.3, RAIL, region="handrail-steel", material="steel", penMm=0.13))
    return sh

def s200():
    sh = [GROUND()]
    sh.append(D.poly("ramp", "body", "body", 1, True, 0.5, pts, True, regions=BODY_REG, material="concrete"))
    sh.append(D.poly("kerb", "body", "body", 2, True, 0.4, band(1.5, 11.7, 0, KERB) if False else surf_line(1.5, 10.2, 0) + surf_line(1.5, 10.2, KERB)[::-1], True, region="kerb", material="concrete"))
    sh.append(D.line("handrail", "handrail", "detail", 3, 0.3, RAIL, region="handrail-steel", material="steel", penMm=0.18))
    for i, x in enumerate(POSTS):
        sh.append(D.line(f"post-{i}", "handrail", "detail", 3, 0.3, [(x, surf(x) + KERB), (x, surf(x) + RAIL_H)], region="handrail-steel", material="steel", penMm=0.18))
    return sh

def joints(role="detail"):
    return [D.line(f"joint-{i}", "body", role, 2, 0.4, [(x, 0.0), (x, surf(x))], region="ramp-side") for i, x in enumerate((5.1, 6.6, 10.2))]

def s100():
    sh = [GROUND()]
    sh.append(D.poly("ramp", "body", "body", 1, True, 0.55, pts, True, regions=BODY_REG, material="concrete", hatch="concrete-stipple"))
    sh += joints()
    sh.append(D.poly("kerb", "body", "body", 2, True, 0.4, surf_line(1.5, 10.2, 0) + surf_line(1.5, 10.2, KERB)[::-1], True, region="kerb", material="concrete"))
    sh.append(D.poly("handrail", "handrail", "body", 4, True, 0.3, rail_band(0.0424), True, region="handrail-steel", material="steel", penMm=0.13))
    for i, x in enumerate(POSTS):
        sh.append(D.rect(f"post-{i}", "handrail", "detail", 3, True, 0.3, x - 0.025, surf(x) + KERB, x + 0.025, surf(x) + RAIL_H, region="handrail-steel", material="steel"))
    return sh

def s50():
    sh = [GROUND()]
    sh.append(D.poly("ramp", "body", "body", 1, True, 0.6, pts, True, regions=BODY_REG, material="concrete", hatch="concrete-stipple"))
    sh += joints()
    sh.append(D.poly("kerb", "body", "body", 2, True, 0.45, surf_line(1.5, 10.2, 0) + surf_line(1.5, 10.2, KERB)[::-1], True, region="kerb", material="concrete"))
    sh.append(D.poly("handrail", "handrail", "detail", 5, True, 0.35, rail_band(0.0424), True, region="handrail-steel", material="steel", penMm=0.18))
    for i, x in enumerate(POSTS):
        sh.append(D.rect(f"post-{i}", "handrail", "detail", 3, True, 0.35, x - 0.025, surf(x) + KERB, x + 0.025, surf(x) + RAIL_H - 0.021, region="handrail-steel", material="steel"))
        sh.append(D.rect(f"plate-{i}", "handrail", "detail", 4, True, 0.3, x - 0.06, surf(x) + KERB, x + 0.06, surf(x) + KERB + 0.02, region="handrail-steel", material="steel"))
    for i, (x, y) in enumerate((RAIL[0], RAIL[-1])):
        sh.append(D.line(f"rail-return-{i}", "handrail", "detail", 5, 0.35, [(x, y - 0.021), (x, y - 0.12)], region="handrail-steel", material="steel", penMm=0.18))
    sh.append(D.line("rail-axis", "handrail", "texture", 6, 0.2, RAIL, region="handrail-steel", penMm=0.1))
    return sh

rng = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
reps = {}
stip = lambda sp: [H("concrete-stipple", "Concrete texture", "Betonstruktur", "stipple", 0, sp, 0.1)]
reps["500"] = rep(500, s500(), dec(
    ("One poche silhouette of the stepped ramp (at-grade landing, run, middle landing, run, top landing) and one rail line.", "Eine Pochésilhouette der gestuften Rampe (ebenerdiges Podest, Lauf, Zwischenpodest, Lauf, oberes Podest) und eine Handlauflinie."),
    ("Ramp body 11.7 m long and 0.6 m high at the real 1:12 slope, the handrail polyline at 0.85 m above the surface with its horizontal extensions.", "Rampenkörper 11,7 m lang und 0,6 m hoch mit der echten Steigung 1:12, die Handlaufpolylinie 0,85 m über der Oberfläche mit ihren waagerechten Verlängerungen."),
    ("Surface, side faces and the wheel-guard kerb are one filled mass listing all three regions; the 0.10 m kerb would be 0.2 mm.", "Oberfläche, Seitenflächen und Radabweiser sind eine gefüllte Masse mit allen drei Regionen; der 0,10 m hohe Radabweiser wäre 0,2 mm."),
    ("Posts, slab joints, rail ends, stipple, and the first 0.9 m of the ramp wedge (thinner than 0.1 m = 0.2 mm), which would print as a sliver; the drawn body starts with a 0.075 m toe step.", "Pfosten, Plattenfugen, Handlaufenden, Punktierung und die ersten 0,9 m des Rampenkeils (dünner als 0,1 m = 0,2 mm), die als Sliver druckten; der gezeichnete Körper beginnt mit einer 0,075 m hohen Fußstufe."),
    ("The 0.042 m handrail tube is one 0.13 mm line (0.065 m at this scale, about 0.025 m wider than real).", "Das 0,042 m dicke Handlaufrohr ist eine 0,13-mm-Linie (0,065 m bei diesem Maßstab, etwa 0,025 m breiter als real)."),
    ("None.", "Keine."), ("Filled mid-grey mass, 0.13 mm outline.", "Gefüllte mittelgraue Masse, 0,13-mm-Kontur."),
    ("Thermal cells: the ramp mass (merged concrete surfaces) and the handrail line.", "Thermische Zellen: die Rampenmasse (zusammengefasste Betonflächen) und die Handlauflinie.")),
    pens_=pens(0.13, None, None, None, 0.18), rng=rng["500"])
reps["200"] = rep(200, s200(), dec(
    ("Stepped ramp body with the wheel-guard kerb as a separate band, a handrail line and six post lines.", "Gestufter Rampenkörper mit dem Radabweiser als eigenem Band, eine Handlauflinie und sechs Pfostenlinien."),
    ("Body, kerb band (0.10 m = 0.5 mm), handrail polyline with extensions, six posts.", "Körper, Radabweiserband (0,10 m = 0,5 mm), Handlaufpolylinie mit Verlängerungen, sechs Pfosten."),
    ("Surface and side face stay one mass (regions merged).", "Oberfläche und Seitenfläche bleiben eine Masse (Regionen zusammengefasst)."),
    ("Slab joints, post base plates, rail returns, stipple.", "Plattenfugen, Pfostenfußplatten, Handlaufrücklenkungen, Punktierung."),
    ("Handrail tube and posts are 0.18 mm lines (rail 0.042 m real = 0.21 mm, post 0.05 m = 0.25 mm).", "Handlaufrohr und Pfosten sind 0,18-mm-Linien (Rohr real 0,042 m = 0,21 mm, Pfosten 0,05 m = 0,25 mm)."),
    ("None.", "Keine."), ("Filled mass 0.18 mm, rail lines 0.18 mm.", "Gefüllte Masse 0,18 mm, Handlauflinien 0,18 mm."),
    ("Cells: ramp mass, kerb band, rail and posts.", "Zellen: Rampenmasse, Radabweiserband, Handlauf und Pfosten.")),
    pens_=pens(0.18, 0.1, None, None, 0.25), rng=rng["200"])
reps["100"] = rep(100, s100(), dec(
    ("The ramp body with stipple, slab joints at the landing transitions, the kerb band, a filled handrail band and filled posts.", "Der Rampenkörper mit Punktierung, Plattenfugen an den Podestübergängen, das Radabweiserband, ein gefülltes Handlaufband und gefüllte Pfosten."),
    ("Handrail tube drawn at its real 0.042 m width (0.42 mm), posts at 0.05 m (0.5 mm), three joints.", "Handlaufrohr in seiner echten Breite 0,042 m (0,42 mm), Pfosten mit 0,05 m (0,5 mm), drei Fugen."),
    ("Surface and side face remain one body.", "Oberfläche und Seitenfläche bleiben ein Körper."),
    ("Post base plates (0.12 x 0.01 m), rail returns.", "Pfostenfußplatten (0,12 x 0,01 m), Handlaufrücklenkungen."),
    ("None thickened: tube and posts are filled at real width.", "Nichts verdickt: Rohr und Pfosten sind in echter Breite gefüllt."),
    ("Concrete texture as stipple at 0.8 mm.", "Betonstruktur als Punktierung mit 0,8 mm."),
    ("Outline 0.25 mm for the body, 0.13 mm for rail and posts.", "Kontur 0,25 mm für den Körper, 0,13 mm für Handlauf und Pfosten."),
    ("Cells: body, kerb, rail band, posts.", "Zellen: Körper, Radabweiser, Handlaufband, Pfosten.")),
    hatches=stip(0.8), pens_=pens(0.25, 0.13, 0.1, None, 0.35), rng=rng["100"])
reps["50"] = rep(50, s50(), dec(
    ("The full side elevation of the ramp: body with stipple and three pour joints, the wheel-guard kerb, rail tube as outlined band with its centre axis, six posts with base plates and the downturned rail ends.", "Die vollständige Seitenansicht der Rampe: Körper mit Punktierung und drei Betonierfugen, der Radabweiser, das Handlaufrohr als Konturband mit Mittelachse, sechs Pfosten mit Fußplatten und die abgebogenen Handlaufenden."),
    ("Everything of the 1:100 sheet plus base plates (0.12 m), 0.12 m rail returns (DIN 18040-1, rail ends must not protrude), rail axis.", "Alles des 1:100-Blattes plus Fußplatten (0,12 m), 0,12 m lange Handlaufrücklenkungen (DIN 18040-1: Handlaufenden dürfen nicht abstehen), Handlaufachse."),
    ("The far-side handrail and posts coincide exactly with the near side and are one region (both sides).", "Der Handlauf und die Pfosten der Gegenseite fallen genau mit der Nahseite zusammen und sind eine Region (beide Seiten)."),
    ("Tube bends at the landings, anchors, slab reinforcement.", "Rohrkrümmungen an den Podesten, Verankerungen, Plattenbewehrung."),
    ("Rail tube 0.042 m (0.84 mm) and posts 0.05 m (1.0 mm) drawn at real width; the base plates are 0.02 m thick (0.4 mm, thicker than the real 8 mm) so that they print.", "Handlaufrohr 0,042 m (0,84 mm) und Pfosten 0,05 m (1,0 mm) in echter Breite; die Fußplatten sind 0,02 m dick (0,4 mm, dicker als die realen 8 mm), damit sie drucken."),
    ("Concrete texture as stipple at 1.0 mm.", "Betonstruktur als Punktierung mit 1,0 mm."),
    ("Body 0.35 mm, rail band 0.18 mm, axis 0.1 mm.", "Körper 0,35 mm, Handlaufband 0,18 mm, Achse 0,1 mm."),
    ("One cell per region: body, kerb, steel handrail with posts.", "Eine Zelle je Region: Körper, Radabweiser, Stahlhandlauf mit Pfosten.")),
    hatches=stip(1.0), pens_=pens(0.35, 0.18, 0.1, None, 0.5), rng=rng["50"])

REFS = [
    ref("ada-2010", "2010 ADA Standards for Accessible Design, section 405 Ramps: running slope not steeper than 1:12, rise per run at most 30 in (760 mm), landings at least 60 in (1525 mm) long, handrails 34 to 38 in high on both sides, 12 in extensions, 4 in edge protection."),
    ref("din-18040-1", "DIN 18040-1:2010-10 Barrier-free building, public buildings, section 4.3.8 / 4.3.3: ramps with clear width 1.20 m, landings 1.50 m, run at most 6 m, handrails at 0.85 m on both sides extending 0.30 m, wheel guard 0.10 m. DIN recommends at most 6 % slope; the 1:12 (8.3 %) of this asset is the ADA maximum and exceeds the DIN limit, it is an authored ADA-style ramp."),
    ref("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, actions on structures, part 1-1: Table A.1 reinforced concrete 25 kN/m3; Table 6.2 imposed loads, category C3 (areas without obstacles for moving people) 5 kN/m2; Table 6.12 line load on barriers, 1.0 kN/m chosen within the recommended 0.5 to 1.0 range (National Annex governs)."),
    ref("en-10219", "EN 10219-2:2019 Cold formed welded structural hollow sections: CHS 42.4 x 2.6 (2.55 kg/m, A 3.54 cm2, I 7.6 cm4) and CHS 48.3 x 3.2 (3.56 kg/m)."),
    ref("en-iso-10456", "EN ISO 10456:2007 Building materials, hygrothermal properties: reinforced concrete 1 % steel rho 2300, lambda 2.3 W/mK, 1000 J/kgK; steel 50 W/mK, 450 J/kgK; emissivity of concrete 0.9."),
    ref("en-1992-1-1", "EN 1992-1-1:2004 Eurocode 2, design of concrete structures: E_cm 31 GPa (C25/30), f_ck 25 MPa."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, types of drawings, scales and symbols."),
    ref("asce-solar", "Typical solar absorptivity of weathered grey concrete 0.6 to 0.7 and of galvanised steel 0.5 to 0.6 (ASHRAE Handbook Fundamentals 2021, ch. 25, rounded)."),
]
mats = [
    mat("concrete", "Reinforced concrete C25/30, broom finish", "Stahlbeton C25/30, Besenstrich", 0.9, 0.65, 2500, 2.3, 1000, 31e9, 25e6, "literature", "en-1992-1-1"),
    mat("steel", "Hot-dip galvanised steel tube", "Feuerverzinktes Stahlrohr", 0.3, 0.55, 7850, 50, 450, 210e9, 235e6, "literature", "en-10219"),
]
LEN_SURF = 2 * SLOPE_LEN + 3.0
regions = [
    region("ramp-surface", "Walking surface of runs and landings", "Gehfläche von Läufen und Podesten", "surface", "passive", "concrete", LEN_SURF * 1.2, 1.2, "horizontal-plate-up", 1.0,
           [layer("slab-top", "Concrete, effective storage depth 0.10 m", "Beton, wirksame Speichertiefe 0,10 m", "masonry", "concrete", 0.10, 0.0435, "literature", "en-iso-10456", c=0.10 * 2300 * 1000)], 230000, "literature", "en-iso-10456"),
    region("ramp-side", "Concrete side faces", "Betonseitenflächen", "surface", "passive", "concrete", 2 * poly_area, 0.45, "vertical-plate", 0.5,
           [layer("side-concrete", "Concrete, effective storage depth 0.10 m", "Beton, wirksame Speichertiefe 0,10 m", "masonry", "concrete", 0.10, 0.0435, "literature", "en-iso-10456", c=0.10 * 2300 * 1000)], 230000, "literature", "en-iso-10456"),
    region("kerb", "Wheel-guard kerbs", "Radabweiser", "surface", "passive", "concrete", 4 * KERB * (2 * SLOPE_LEN + 1.5), 0.1, "vertical-plate", 0.5,
           [layer("kerb-concrete", "Concrete, effective storage depth 0.05 m", "Beton, wirksame Speichertiefe 0,05 m", "masonry", "concrete", 0.05, 0.0217, "literature", "en-iso-10456", c=0.05 * 2300 * 1000)], 115000, "literature", "en-iso-10456"),
    region("handrail-steel", "Handrails and posts, both sides", "Handläufe und Pfosten beider Seiten", "surface", "passive", "steel", 4.2, 0.0424, "cylinder", 0.8,
           [layer("tube-wall", "Steel tube wall 2.6 mm", "Stahlrohrwand 2,6 mm", "metal", "steel", 0.0026, 0.0026 / 50, "literature", "en-iso-10456", c=0.0026 * 7850 * 450)], 9200, "literature", "en-10219"),
]
anchors = [anchor(f"ground-{i}", "body", x, 0, "support", f"Ground bearing {i + 1}", f"Auflager auf dem Boden {i + 1}", support="contact", normal=90, fl=2.0, fw=1.4, mu=0.6) for i, x in enumerate((2.5, 5.0, 8.0, 11.0))]
anchors += [anchor("chair-point", "chair-carrier", 0, 0, "load", "Wheelchair wheel base", "Rollstuhlaufstandsfläche"),
            anchor("rail-mid", "handrail", 4.65, 1.0, "load", "Handrail centre", "Handlaufmitte")]
def regs(over=None):
    base = {"ramp-surface": (1.0, 1.0), "ramp-side": (1.0, 0.45), "kerb": (1.0, 0.7), "handrail-steel": (1.0, 0.9)}
    base.update(over or {})
    return {k: RB(*v) for k, v in base.items()}
q = load("imposed-c3", "imposed", "Imposed load category C3, 5 kN/m2 on 1.2 m clear width (6 kN/m)", "Nutzlast Kategorie C3, 5 kN/m2 auf 1,2 m lichter Breite (6 kN/m)", 0, 0, "literature", "en-1991-1-1",
         bone_="body", x=0, y=0, dist={"bone": "body", "fromM": 1.5, "toM": 11.7, "wxNPerM": 0, "wyNPerM": -6000})
chair = load("wheelchair", "occupant", "Power wheelchair and user, 200 kg", "Elektrorollstuhl mit Nutzer, 200 kg", 0, -1962, "authored-estimate", anchor_="chair-point")
push = load("rail-push", "imposed", "Handrail line load 1.0 kN/m, horizontal", "Handlauflinienlast 1,0 kN/m, waagerecht", 0, 0, "literature", "en-1991-1-1", bone_="handrail", x=0, y=0,
            dist={"bone": "handrail", "fromM": 0, "toM": 9.3, "wxNPerM": 1000, "wyNPerM": 0})
poses = [
    pose("self-weight", "Self-weight", "Eigengewicht", "The ramp unloaded, in full sun.", "Die unbelastete Rampe in voller Sonne.", {}, [], regs()),
    pose("crowd-live-load", "Crowded, category C3", "Belegt, Kategorie C3", "Uniform imposed load 5 kN/m2 over the runs and landings.", "Gleichmäßige Nutzlast 5 kN/m2 über Läufe und Podeste.", {}, [q], regs()),
    pose("wheelchair-on-run-2", "Wheelchair on the second run", "Rollstuhl auf dem zweiten Lauf", "A 200 kg power wheelchair at the middle of the upper run.", "Ein 200 kg schwerer Elektrorollstuhl in der Mitte des oberen Laufs.", {"chair-carrier": {"dx": 1.8}}, [chair], regs()),
    pose("handrail-push", "Pushing on the handrails", "Druck auf die Handläufe", "Horizontal handrail line load of 1 kN/m (handrail in partial shade).", "Waagerechte Handlauflinienlast von 1 kN/m (Handlauf teilweise im Schatten).", {}, [push], regs({"handrail-steel": (1.0, 0.3), "ramp-surface": (1.0, 0.6)})),
]
shade = lambda rid, hi, lo: track(f"region:{rid}:sunlitFraction", [(0, hi), (4, lo), (8, hi)])
clips = [
    clip("wheelchair-ascent", "Wheelchair climbs the second run", "Rollstuhl befährt den zweiten Lauf", "The wheelchair load moves up the 1:12 run from the middle landing to the top landing.", "Die Rollstuhllast fährt den Lauf 1:12 vom Zwischenpodest zum oberen Podest hinauf.",
         10, False, [key(0, "wheelchair-on-run-2", bones={"chair-carrier": {"dx": 0.0}}), key(10, "wheelchair-on-run-2", bones={"chair-carrier": {"dx": SLOPE_LEN}})],
         [phase("climb", 0, 10, "Climb at 0.36 m/s", "Auffahrt mit 0,36 m/s")], [event(0, "motion", "Wheelchair at the middle landing", "Rollstuhl am Zwischenpodest"), event(10, "motion", "Wheelchair at the top landing", "Rollstuhl am oberen Podest")]),
    clip("live-load-ramp", "Crowd loading builds up", "Personenbelastung baut sich auf", "The category C3 live load rises from nothing to full value over the ramp.", "Die Nutzlast Kategorie C3 steigt von null auf den vollen Wert über die Rampe.",
         6, False, [key(0, "crowd-live-load"), key(6, "crowd-live-load")],
         [phase("ramp", 0, 4, "Ramp", "Aufbau"), phase("hold", 4, 6, "Hold", "Halten")], [event(4, "load-on", "Imposed load at full value", "Nutzlast vollständig")],
         [track("load:imposed-c3", [(0, 0), (4, 1), (6, 1)])]),
    clip("sun-and-shade", "Cloud shadow crosses the ramp", "Wolkenschatten zieht über die Rampe", "A cloud shadow passes: the sunlit fractions of the surface, side, kerb and handrail drop and recover.", "Ein Wolkenschatten zieht durch: die Besonnungsanteile von Fläche, Seite, Radabweiser und Handlauf fallen und erholen sich.",
         8, True, [key(0, "self-weight"), key(8, "self-weight")], [phase("shadow", 0, 8, "Shadow passes", "Schatten zieht durch")], [event(4, "thermal", "Deepest shade", "Tiefster Schatten")],
         [shade("ramp-surface", 1.0, 0.1), shade("ramp-side", 0.45, 0.05), shade("kerb", 0.7, 0.07), shade("handrail-steel", 0.9, 0.09)]),
]
a = {
    "id": "wheelchair-ramp", "category": "components",
    "title": T("Wheelchair Ramp 1:12 with Landings", "Rollstuhlrampe 1:12 mit Podesten"),
    "summary": T("A 1.4 m wide reinforced concrete access ramp rising 0.60 m in two 3.6 m runs at 1:12 (8.3 %), with an at-grade landing, a middle landing and a top landing of 1.5 m, 0.10 m wheel-guard kerbs and continuous steel handrails at 0.85 m on both sides with 0.30 m extensions. 1:12 is the ADA maximum; DIN 18040-1 asks for at most 6 %.",
                 "Eine 1,4 m breite Stahlbeton-Zugangsrampe, die in zwei 3,6 m langen Läufen mit 1:12 (8,3 %) um 0,60 m ansteigt, mit ebenerdigem Podest, Zwischenpodest und oberem Podest von je 1,5 m, 0,10 m hohen Radabweisern und durchgehenden Stahlhandläufen in 0,85 m Höhe auf beiden Seiten mit 0,30 m Verlängerung. 1:12 ist das ADA-Maximum; DIN 18040-1 verlangt höchstens 6 %."),
    "tags": ["component", "structural", "ramp", "accessibility", "handrail", "concrete"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Gradient 1:12 and 1.5 m landings follow ADA 405; DIN 18040-1 demands at most 6 %, so this ramp is an ADA-style ramp. Concrete is solid (earth-retained) with 25 kN/m3.", "Gefälle 1:12 und 1,5-m-Podeste folgen ADA 405; DIN 18040-1 fordert höchstens 6 %, diese Rampe ist daher eine ADA-Rampe. Der Beton ist massiv (erdangefüllt) mit 25 kN/m3.")},
    "materials": mats,
    "rig": {"kind": "structural", "root": "body", "bones": bones},
    "anchors": anchors, "representations": reps, "symbols": [],
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(15, 15, -3, 18, 0.55, 2, 600, 0.2), "operating": [], "regions": regions, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Reference case: mild sunny spring day (air 15 degC, 600 W/m2 normal irradiance, wind 2 m/s); all regions are passive and respond with their concrete storage depth.", "Referenzfall: milder sonniger Frühlingstag (Luft 15 °C, 600 W/m2 Normalstrahlung, Wind 2 m/s); alle Regionen sind passiv und reagieren mit ihrer Betonspeichertiefe."),
                    T("Sun and shade come from the pose sunlit fractions: the surface and rails are sunlit, the side face is partly shaded by the kerb and the near-side wall geometry.", "Sonne und Schatten kommen aus den Besonnungsanteilen der Posen: Fläche und Handläufe sind besonnt, die Seitenfläche ist teilweise durch Radabweiser und Umgebung verschattet."),
                    T("The far-side handrail and posts coincide with the near side in elevation; one region carries both (areas doubled).", "Handlauf und Pfosten der Gegenseite fallen in der Ansicht mit der Nahseite zusammen; eine Region trägt beide (Flächen verdoppelt).")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [x["id"] for x in anchors if x.get("support")],
                  "assumptions": [T("The earth-retained concrete body bears on the ground as a rigid block (unilateral contacts with friction 0.6); loads are per 1.4 m width; handrail posts are rigidly attached.", "Der erdangefüllte Betonkörper liegt als starrer Block auf dem Boden (einseitige Kontakte, Reibung 0,6); Lasten gelten für 1,4 m Breite; die Handlaufpfosten sind starr befestigt."),
                                T("Category C3 (5 kN/m2) is used for a public ramp; a dwelling ramp (category A, 2 kN/m2) would be lighter. The wheelchair load is an authored estimate.", "Kategorie C3 (5 kN/m2) gilt für eine öffentliche Rampe; eine Wohnungsrampe (Kategorie A, 2 kN/m2) wäre leichter. Die Rollstuhllast ist ein Schätzwert.")], "clinical": False},
    "presentation": {"defaultPose": "self-weight", "defaultClip": "wheelchair-ascent", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": WIDTH,
}
assemble(a, "wheelchair-ramp")
