import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

R = 0.5          # track curve radius
HD = 2.0         # door height
PL = 0.5         # panel height
NP = 4
ARC = math.pi * R / 2
HEND = 2.6       # horizontal track end x
TW = 0.02       # half width of the track band
SH = (0.12, 2.75)   # spring shaft centre
DR = 0.09        # drum radius
DRUM_R = 0.08    # cable winding radius

def P(s):
    if s < 0:
        return (0.0, HD + s)
    if s <= ARC:
        th = math.pi - s / R
        return (R + R * math.cos(th), HD + R * math.sin(th))
    return (R + (s - ARC), HD + R)

def prev_hinge(s_next):
    lo, hi = s_next - PL - 0.5, s_next
    for _ in range(80):
        mid = (lo + hi) / 2
        a, b = P(mid), P(s_next)
        if math.hypot(a[0] - b[0], a[1] - b[1]) > PL:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2

def chain(s4):
    s = [0.0] * (NP + 1)
    s[NP] = s4
    for k in range(NP - 1, -1, -1):
        s[k] = prev_hinge(s[k + 1])
    return s

S_OPEN = ARC + 2.0
def state(u):
    s = chain(u * S_OPEN)
    H = [P(v) for v in s]
    ang = [math.degrees(math.atan2(H[k + 1][1] - H[k][1], H[k + 1][0] - H[k][0])) for k in range(NP)]
    off = {"carriage-x": {"dx": H[0][0]}, "carriage-y": {"dy": H[0][1]}, "panel-1": {"angleDeg": ang[0] - 90}}
    for k in range(1, NP):
        off[f"panel-{k + 1}"] = {"angleDeg": ang[k] - ang[k - 1]}
    travel = s[0] + HD
    off["spring-shaft"] = {"angleDeg": travel / DRUM_R * 180 / math.pi}
    return off, ang, s

PANEL_KG = [15.0, 13.8, 13.8, 13.8]
TRACK_KG = 28.0
SHAFT_KG = 16.0
W_DOOR = sum(PANEL_KG) * 9.81
def cable_T(u):
    off, ang, s = state(u)
    return 0.95 * sum(m * 9.81 * math.sin(math.radians(a)) for m, a in zip(PANEL_KG, ang))

bones = [bone("track", None, "Track frame with brackets and ceiling hangers (fixed root)", "Laufschienenrahmen mit Winkeln und Deckenabhängern (feste Wurzel)", 0, 0, 0, HEND, fixed(),
              mass(TRACK_KG, 1.2, 1.4, "point", "authored-estimate", "door-catalogues"))]
bones.append(bone_world("carriage-x", "track", "Horizontal travel of the lowest hinge (massless)", "Horizontaler Weg des untersten Scharniers (masselos)", 0, 0, 0, 0, prismatic(0, 0, 0.6), bones))
bones.append(bone_world("carriage-y", "carriage-x", "Vertical travel of the lowest hinge (massless)", "Vertikaler Weg des untersten Scharniers (masselos)", 0, 0, 0, 0, prismatic(90, 0, 2.6), bones))
for k in range(1, NP + 1):
    parent = "carriage-y" if k == 1 else f"panel-{k - 1}"
    bones.append(bone_world(f"panel-{k}", parent, f"Door panel {k} (bottom is 1)", f"Torpaneel {k} (unten ist 1)", 0, PL * (k - 1), 90, PL,
                            revolute(-95, 5) if k == 1 else revolute(-70, 8), bones,
                            mass(PANEL_KG[k - 1], 0.25, 0.0, "uniform" if False else "point", "authored-estimate", "door-catalogues")))
bones.append(bone_world("spring-shaft", "track", "Torsion spring shaft with cable drums", "Torsionsfederwelle mit Seiltrommeln", SH[0], SH[1], 0, DR, revolute(-2200, 2200), bones,
                        mass(SHAFT_KG, 0, 0, "point", "authored-estimate", "door-catalogues")))
D = Draw(bones)

def band_around(pts_n, w):
    path = [P(-HD + i * HD / 20) for i in range(0, 20)]
    path += [P(ARC * i / 16) for i in range(0, 17)]
    path += [(R + (HEND - R) * i / 8, HD + R) for i in range(1, 9)]
    dedup = []
    for p in path:
        if not dedup or math.hypot(p[0] - dedup[-1][0], p[1] - dedup[-1][1]) > 1e-9:
            dedup.append(p)
    left, right = [], []
    n = len(dedup)
    for i, p in enumerate(dedup):
        a = dedup[max(i - 1, 0)]; b = dedup[min(i + 1, n - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        l = math.hypot(dx, dy); nx, ny = -dy / l, dx / l
        left.append((p[0] + nx * w, p[1] + ny * w)); right.append((p[0] - nx * w, p[1] - ny * w))
    return dedup, left, right

CENTRE, LEFT, RIGHT = band_around(0, TW)
CENTRE2, LEFT2, RIGHT2 = band_around(0, 0.02)
HANGERS = [1.3, 2.5]
HANG_TOP = 2.85

def ground(): return D.line("ground", "track", "ground", 0, 0.5, [(-0.2, 0.0), (HEND + 0.05, 0.0)])
def panel_rect(k, role, z, tone, **kw):
    y0, y1 = PL * (k - 1), PL * k
    return D.rect(f"panel-{k}", f"panel-{k}", role, z, True, tone, -0.02, y0, 0.02, y1, **kw)
def panel_line(k, pen):
    return D.line(f"panel-{k}", f"panel-{k}", "body", 3, 0.5, [(0, PL * (k - 1)), (0, PL * k)], regions=["panel-sandwich", "hardware-steel"], material="sandwich-skin", penMm=pen)

def s500():
    sh = [ground()]
    for k in range(1, NP + 1):
        sh.append(panel_line(k, 0.13))
    sh.append(D.poly("track", "track", "body", 1, False, 0.4, CENTRE, False, region="track-steel", material="steel", penMm=0.13))
    sh.append(D.circ("drum", "spring-shaft", "body", 2, True, 0.4, SH[0], SH[1], DR, region="spring-assembly", material="steel"))
    return sh

def s200():
    sh = [ground()]
    for k in range(1, NP + 1):
        sh.append(D.line(f"panel-{k}", f"panel-{k}", "body", 3, 0.5, [(0, PL * (k - 1)), (0, PL * k)], regions=["panel-sandwich", "hardware-steel"], material="sandwich-skin", penMm=0.18))
    sh.append(D.poly("track", "track", "detail", 1, False, 0.4, CENTRE, False, region="track-steel", material="steel"))
    sh.append(D.circ("drum", "spring-shaft", "body", 2, True, 0.4, SH[0], SH[1], DR, region="spring-assembly", material="steel"))
    sh.append(D.line("drum-index", "spring-shaft", "detail", 3, 0.9, [SH, (SH[0] + DR, SH[1])], region="spring-assembly", penMm=0.1))
    return sh

def hangers(role, pen):
    return [D.line(f"hanger-{i}", "track", role, 1, 0.4, [(x, HD + R), (x, HANG_TOP)], region="track-steel", material="steel", penMm=pen) for i, x in enumerate(HANGERS)]

def s100():
    sh = [ground()]
    sh.append(D.poly("track", "track", "detail", 1, False, 0.4, LEFT, False, region="track-steel", material="steel"))
    sh.append(D.poly("track-b", "track", "detail", 1, False, 0.4, RIGHT, False, region="track-steel", material="steel"))
    sh += hangers("detail", 0.13)
    for k in range(1, NP + 1):
        sh.append(panel_rect(k, "body", 3, 0.55, regions=["panel-sandwich", "hardware-steel"], material="sandwich-skin"))
    sh.append(D.circ("drum", "spring-shaft", "body", 2, True, 0.4, SH[0], SH[1], DR, region="spring-assembly", material="steel"))
    sh.append(D.line("drum-index", "spring-shaft", "detail", 3, 0.9, [SH, (SH[0] + DR, SH[1])], region="spring-assembly"))
    return sh

def s50():
    sh = [ground()]
    sh.append(D.poly("track", "track", "detail", 1, False, 0.4, LEFT, False, region="track-steel", material="steel"))
    sh.append(D.poly("track-b", "track", "detail", 1, False, 0.4, RIGHT, False, region="track-steel", material="steel"))
    sh += hangers("detail", 0.18)
    for k in range(1, NP + 1):
        sh.append(panel_rect(k, "body", 3, 0.55, region="panel-sandwich", material="sandwich-skin"))
    for k in range(0, NP + 1):
        wy = PL * k
        bone_ = f"panel-{k}" if k >= 1 else "panel-1"
        yloc = wy
        sh.append(D.circ(f"roller-{k}", bone_ if k >= 1 else "panel-1", "detail", 4, True, 0.2, 0.0, wy, 0.011, region="hardware-steel", material="steel"))
        if 0 < k < NP:
            sh.append(D.rect(f"hinge-{k}", f"panel-{k + 1}", "detail", 4, True, 0.3, 0.025, wy - 0.05, 0.045, wy + 0.05, region="hardware-steel", material="steel"))
    sh.append(D.circ("drum", "spring-shaft", "body", 2, True, 0.5, SH[0], SH[1], DR, region="spring-assembly", material="steel"))
    sh.append(D.circ("coil", "spring-shaft", "detail", 3, False, 0.4, SH[0], SH[1], 0.0375, region="spring-assembly", material="steel"))
    sh.append(D.circ("shaft", "spring-shaft", "detail", 4, True, 0.2, SH[0], SH[1], 0.0125, region="spring-assembly", material="steel"))
    sh.append(D.line("drum-index", "spring-shaft", "texture", 5, 0.9, [(SH[0] + 0.0375, SH[1]), (SH[0] + DR, SH[1])], region="spring-assembly"))
    return sh

rng = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
reps = {}
reps["500"] = rep(500, s500(), dec(
    ("Four panel lines joined at the hinges, one track line, the spring drum as one dot.", "Vier an den Scharnieren verbundene Paneellinien, eine Schienenlinie, die Federtrommel als Punkt."),
    ("The 2.0 m door as four 0.5 m panels on the real quarter-circle track (R 0.5 m) and 2.1 m horizontal run, drum 0.18 m.", "Das 2,0 m hohe Tor als vier 0,5-m-Paneele auf der echten Viertelkreisschiene (R 0,5 m) mit 2,1 m Horizontalstück, Trommel 0,18 m."),
    ("Hinges, rollers and brackets are merged into the panel lines (regions listed); the sandwich panel and its steel hardware share one stroke.", "Scharniere, Rollen und Winkel sind in die Paneellinien eingeschmolzen (Regionen aufgeführt); Sandwichpaneel und Stahlbeschläge teilen einen Strich."),
    ("Hangers, shaft, coil, cable, seal, handle.", "Abhänger, Welle, Feder, Seil, Dichtung, Griff."),
    ("A 0.04 m panel is a 0.13 mm line (0.065 m at this scale, 0.025 m wider than real); the 0.07 m track is the same line type.", "Ein 0,04 m dickes Paneel ist eine 0,13-mm-Linie (0,065 m in diesem Maßstab, 0,025 m breiter als real); die 0,07 m breite Schiene ist derselbe Linientyp."),
    ("None.", "Keine."), ("Black lines for panels and track; the drum a filled dot.", "Schwarze Linien für Paneele und Schiene; die Trommel ein gefüllter Punkt."),
    ("Cells: door line (all panels), track line, drum.", "Zellen: Torlinie (alle Paneele), Schienenlinie, Trommel.")),
    pens_=pens(0.13, None, None, None, 0.18), rng=rng["500"])
reps["200"] = rep(200, s200(), dec(
    ("Four panel lines of 0.18 mm on a centre-line track, the drum as a filled disc with an index mark.", "Vier 0,18-mm-Paneellinien auf einer Mittellinienschiene, die Trommel als gefüllte Scheibe mit Marke."),
    ("Panels at the real 0.5 m height and 0.04 m thickness (0.18 mm line), track centre line, drum 0.18 m with its index line that shows the winding.", "Paneele in der echten Höhe 0,5 m und Dicke 0,04 m (0,18-mm-Linie), Schienenmittellinie, Trommel 0,18 m mit Markierungslinie, die das Aufwickeln zeigt."),
    ("Hinges and rollers are merged into the panel lines.", "Scharniere und Rollen sind in die Paneellinien eingeschmolzen."),
    ("Hangers, brackets, coil, shaft.", "Abhänger, Winkel, Feder, Welle."),
    ("Panel line 0.18 mm is about the real 0.04 m; track 0.1 mm instead of 0.07 m (0.35 mm).", "Paneellinie 0,18 mm entspricht etwa den realen 0,04 m; Schiene 0,1 mm statt 0,07 m (0,35 mm)."),
    ("None.", "Keine."), ("Panels bold, track light.", "Paneele kräftig, Schiene leicht."),
    ("Cells: door line (panel and hardware merged), track, drum.", "Zellen: Torlinie (Paneel und Beschläge zusammengefasst), Schiene, Trommel.")),
    pens_=pens(0.18, 0.1, None, None, 0.25), rng=rng["200"])
reps["100"] = rep(100, s100(), dec(
    ("Four filled panel bars between a double-line track band, two ceiling hangers and the drum.", "Vier gefüllte Paneelbalken zwischen einem Doppellinien-Schienenband, zwei Deckenabhänger und die Trommel."),
    ("Panels at 0.04 x 0.5 m (0.4 mm), track band of 0.07 m, hangers, drum with index.", "Paneele mit 0,04 x 0,5 m (0,4 mm), Schienenband 0,07 m, Abhänger, Trommel mit Marke."),
    ("Hinges, rollers and brackets stay merged into the panel bars.", "Scharniere, Rollen und Winkel bleiben in den Paneelbalken zusammengefasst."),
    ("Rollers (0.022 m = 0.2 mm), shaft, coil.", "Rollen (0,022 m = 0,2 mm), Welle, Feder."),
    ("None thickened.", "Nichts verdickt."), ("None: the door is an elevation of a section.", "Keine: das Tor ist eine Schnittansicht."),
    ("Panels 0.25 mm outline filled mid-grey, track 0.13 mm.", "Paneele mit 0,25-mm-Kontur mittelgrau gefüllt, Schiene 0,13 mm."),
    ("Cells: panel bars, track band, drum.", "Zellen: Paneelbalken, Schienenband, Trommel.")),
    pens_=pens(0.25, 0.13, 0.1, None, 0.35), rng=rng["100"])
reps["50"] = rep(50, s50(), dec(
    ("The full section of a sectional door: four panels with hinge plates and rollers on a track band, curved track, horizontal track with hangers, torsion spring drum with coil, shaft and index.", "Der volle Schnitt eines Sektionaltors: vier Paneele mit Scharnierplatten und Rollen auf einem Schienenband, gebogene Schiene, Horizontalschiene mit Abhängern, Torsionsfedertrommel mit Feder, Welle und Marke."),
    ("Five rollers (0.022 m), three hinge plates, drum, coil, shaft, index.", "Fünf Rollen (0,022 m), drei Scharnierplatten, Trommel, Feder, Welle, Marke."),
    ("Nothing merged.", "Nichts zusammengefasst."),
    ("Cable, bottom seal, handle, lock, panel ribs.", "Seil, Bodendichtung, Griff, Schloss, Paneelsicken."),
    ("Rollers and hinge plates at real size; the panel skins (0.5 mm) are the outline.", "Rollen und Scharnierplatten in echter Größe; die Paneelhäute (0,5 mm) sind die Kontur."),
    ("None.", "Keine."), ("Panels 0.35 mm outline, hardware 0.18 mm, index 0.1 mm.", "Paneele 0,35-mm-Kontur, Beschläge 0,18 mm, Marke 0,1 mm."),
    ("One cell per region: panels, hardware (rollers and hinges), track band and hangers, spring assembly.", "Eine Zelle je Region: Paneele, Beschläge (Rollen und Scharniere), Schienenband und Abhänger, Federbaugruppe.")),
    pens_=pens(0.35, 0.18, 0.1, None, 0.5), rng=rng["50"])

REFS = [
    ref("en-13241", "EN 13241:2003+A2:2016 Industrial, commercial, garage doors and gates, product standard (sectional doors, opening and closing forces, wind resistance)."),
    ref("en-12424", "EN 12424:2000 Industrial, commercial and garage doors and gates, resistance to wind load, classification (class 3 about 700 Pa)."),
    ref("door-catalogues", "Manufacturer data for insulated sectional garage doors, 2.5 m wide, 2.0 m high, 4 panels of 40 mm PU sandwich (steel skins 0.5 mm), U about 0.5 to 0.9 W/m2K, about 9 to 10 kg/m2, torsion spring balance with cable drums and standard-lift track (curve radius about 0.5 m). Typical values consulted, not a table look-up."),
    ref("en-iso-10456", "EN ISO 10456:2007 Building materials, hygrothermal properties: steel 50 W/mK, 450 J/kgK; PU foam lambda 0.022 to 0.028 W/mK."),
    ref("en-iso-6946", "EN ISO 6946:2017 Building components, thermal resistance and transmittance (R_si 0.13, R_se 0.04 m2K/W)."),
    ref("en-10219", "EN 10219-2:2019 Cold formed welded structural hollow sections; steel density 7850 kg/m3."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, types of drawings, scales and symbols."),
]
mats = [
    mat("sandwich-skin", "Galvanised steel skin 0.5 mm, polyester coated", "Stahlhaut 0,5 mm, verzinkt und polyesterbeschichtet", 0.85, 0.4, 7850, 50, 450, 210e9, 235e6, "literature", "en-iso-10456"),
    mat("steel", "Galvanised steel track and hardware", "Verzinkte Stahlschiene und Beschläge", 0.3, 0.55, 7850, 50, 450, 210e9, 235e6, "literature", "en-10219"),
    mat("pu-foam", "Polyurethane foam core", "Polyurethan-Hartschaumkern", 0.9, 0.5, 40, 0.022, 1400, None, None, "literature", "en-iso-10456"),
]
regions = [
    region("panel-sandwich", "Panel sandwich, outer skin side", "Paneelsandwich, Außenhaut", "wall", "conditioned", "sandwich-skin", 5.0, 2.0, "vertical-plate", 0.5,
           [layer("skin-inner", "Inner steel skin 0.5 mm", "Innenhaut Stahl 0,5 mm", "metal", "sandwich-skin", 0.0005, 0.00001, "literature", "en-iso-10456", c=0.0005 * 7850 * 450),
            layer("pu-core", "PU foam 40 mm", "PU-Schaum 40 mm", "insulation", "pu-foam", 0.04, 0.04 / 0.022, "literature", "en-iso-10456", c=0.04 * 40 * 1400),
            layer("skin-outer", "Outer steel skin 0.5 mm", "Außenhaut Stahl 0,5 mm", "metal", "sandwich-skin", 0.0005, 0.00001, "literature", "en-iso-10456", c=0.0005 * 7850 * 450)],
           2200 + 2240 + 1770, "literature", "en-iso-10456", conditioned=(10, 0.13)),
    region("hardware-steel", "Hinges, rollers and corner brackets", "Scharniere, Rollen und Eckwinkel", "surface", "passive", "steel", 0.35, 0.1, "vertical-plate", 0.5,
           [layer("hw-plate", "Steel plate 2 mm", "Stahlblech 2 mm", "metal", "steel", 0.002, 0.002 / 50, "literature", "en-iso-10456", c=0.002 * 7850 * 450)], 7000, "literature", "en-iso-10456"),
    region("track-steel", "Track, curve and hangers", "Schiene, Bogen und Abhänger", "surface", "passive", "steel", 1.9, 0.07, "vertical-plate", 0.7,
           [layer("track-wall", "Steel sheet 2 mm", "Stahlblech 2 mm", "metal", "steel", 0.002, 0.002 / 50, "literature", "en-iso-10456", c=0.002 * 7850 * 450)], 7000, "literature", "en-iso-10456"),
    region("spring-assembly", "Torsion spring, shaft and cable drums", "Torsionsfeder, Welle und Seiltrommeln", "surface", "passive", "steel", 0.6, 0.075, "cylinder", 0.8,
           [layer("coil-wire", "Spring steel wire 8 mm", "Federstahldraht 8 mm", "metal", "steel", 0.008, 0.008 / 50, "literature", "en-iso-10456", c=0.008 * 7850 * 450)], 28000, "literature", "en-iso-10456"),
]
anchors = [
    anchor("jamb-fix", "track", 0, 0, "support", "Vertical track fixed to the jamb", "Vertikalschiene am Pfosten befestigt", support="fixed"),
    anchor("hanger-1", "track", HANGERS[0], HANG_TOP, "support", "Ceiling hanger 1", "Deckenabhänger 1", support="roller", normal=90),
    anchor("hanger-2", "track", HANGERS[1], HANG_TOP, "support", "Ceiling hanger 2", "Deckenabhänger 2", support="roller", normal=90),
] + [anchor(f"wind-{k}", f"panel-{k}", 0.25, 0, "load", f"Wind point on panel {k}", f"Windangriffspunkt Paneel {k}") for k in range(1, NP + 1)] + [
    anchor("cable-eye", "panel-1", 0, 0, "attachment", "Cable eye on the bottom bracket", "Seilöse am unteren Winkel"),
    anchor("drum-cable", "spring-shaft", -DR, 0, "attachment", "Cable leaving the drum", "Seilabgang der Trommel")]

def regs(over=None):
    base = {"panel-sandwich": (1.0, 0.85), "hardware-steel": (1.0, 0.6), "track-steel": (1.0, 0.3), "spring-assembly": (1.0, 0.0)}
    base.update(over or {})
    return {k: RB(*v) for k, v in base.items()}
INSIDE = {"panel-sandwich": (1.0, 0.0), "hardware-steel": (1.0, 0.0), "track-steel": (1.0, 0.0), "spring-assembly": (1.0, 0.0)}

def cable_loads(u):
    T_ = cable_T(u)
    return [load("cable-pull", "equipment", "Spring cable pull on the bottom bracket", "Federseilzug am unteren Winkel", 0, T_, "authored-estimate", anchor_="cable-eye"),
            load("cable-reaction", "equipment", "Cable reaction at the drum", "Seilreaktion an der Trommel", 0, -T_, "authored-estimate", anchor_="drum-cable")]
def mk(id, en, de, den, dde, u, extra=(), regsb=None, key_=False):
    off, _, _ = state(u)
    return pose(id, en, de, den, dde, off, cable_loads(u) + list(extra), regsb or regs(), key=key_)
wind = [load(f"wind-{k}", "wind", f"Wind pressure 0.7 kPa on panel {k}", f"Winddruck 0,7 kPa auf Paneel {k}", 875, 0, "literature", "en-12424", anchor_=f"wind-{k}") for k in range(1, NP + 1)]
poses = [
    mk("closed", "Closed", "Geschlossen", "Door closed in the opening: panels vertical, outside face in the sun, springs wound, cable under tension.", "Tor geschlossen in der Öffnung: Paneele senkrecht, Außenseite in der Sonne, Federn gespannt, Seil unter Zug.", 0.0),
    mk("half-open", "Half open", "Halb offen", "The top two panels have rounded the curve and lie horizontal, the lower two still run up the vertical track.", "Die oberen zwei Paneele sind um den Bogen gelaufen und liegen waagerecht, die unteren zwei laufen noch in der Vertikalschiene.", 0.5, regsb=regs({"panel-sandwich": (1.0, 0.45)})),
    mk("open", "Open", "Offen", "All panels hang horizontally under the ceiling, springs unwound, door out of the sun.", "Alle Paneele hängen waagerecht unter der Decke, Federn entspannt, Tor außerhalb der Sonne.", 1.0, regsb=regs(INSIDE)),
    mk("closed-wind", "Closed under wind class 3", "Geschlossen unter Windklasse 3", "Closed door with 700 Pa wind pressure on every panel (875 N per panel for the 2.5 m width).", "Geschlossenes Tor mit 700 Pa Winddruck auf jedem Paneel (875 N je Paneel bei 2,5 m Breite).", 0.0, wind),
]
N = 8
for i in range(1, N):
    poses.append(mk(f"travel-{i}", f"Travel {i}/{N}", f"Weg {i}/{N}", "In-between key pose of the opening.", "Zwischenpose der Öffnungsbewegung.", i / N, key_=True,
                    regsb=regs({"panel-sandwich": (1.0, max(0.0, 0.85 * (1 - i / N * 1.1)))})))
def travel_pose(i):
    return "closed" if i == 0 else "open" if i == N else f"travel-{i}"
up = [key(2 * i, travel_pose(i)) for i in range(N + 1)]
down = [key(16 + 2 * i, travel_pose(N - i)) for i in range(1, N + 1)]
sunk = lambda rid, a, b: track(f"region:{rid}:sunlitFraction", [(0, a), (6, b), (12, a)])
clips = [
    clip("open-close", "Open and close", "Öffnen und Schließen", "The door rises, its panels turn through the curve one after the other and park under the ceiling; the springs wind off and on again while it closes.", "Das Tor steigt, seine Paneele drehen nacheinander durch den Bogen und parken unter der Decke; die Federn wickeln beim Schließen ab und wieder auf.",
         32, True, up + down,
         [phase("open", 0, 16, "Opening", "Öffnen"), phase("close", 16, 32, "Closing", "Schließen")],
         [event(16, "motion", "Fully open", "Ganz offen"), event(0, "motion", "Closed", "Geschlossen")],
         [track("load:cable-pull", [(0, 1), (8, 0.5), (16, 0), (24, 0.5), (32, 1)]), track("load:cable-reaction", [(0, 1), (8, 0.5), (16, 0), (24, 0.5), (32, 1)])]),
    clip("wind-gust", "Wind gust on the closed door", "Windböe auf das geschlossene Tor", "A gust builds up to class 3 pressure and fades again while the door stays closed.", "Eine Böe baut sich bis zum Druck der Klasse 3 auf und klingt wieder ab, während das Tor geschlossen bleibt.",
         8, False, [key(0, "closed-wind"), key(8, "closed-wind")], [phase("gust", 0, 8, "Gust", "Böe")], [event(4, "load-on", "Peak pressure", "Spitzendruck")],
         [track(f"load:wind-{k}", [(0, 0), (4, 1), (8, 0)]) for k in range(1, NP + 1)]),
    clip("sun-and-shade", "Sun passes the closed door", "Sonne wandert über das geschlossene Tor", "A cloud shadow crosses the closed door: the panel and track sunlit fractions dip and recover.", "Ein Wolkenschatten zieht über das geschlossene Tor: die Besonnungsanteile von Paneelen und Schiene sinken und erholen sich.",
         12, True, [key(0, "closed"), key(12, "closed")], [phase("shadow", 0, 12, "Shadow", "Schatten")], [event(6, "thermal", "Deepest shade", "Tiefster Schatten")],
         [sunk("panel-sandwich", 0.85, 0.08), sunk("hardware-steel", 0.6, 0.06), sunk("track-steel", 0.3, 0.03)]),
]
a = {
    "id": "sectional-garage-door", "category": "components",
    "title": T("Sectional Garage Door", "Sektionaltor"),
    "summary": T("An insulated 2.5 m wide, 2.0 m high sectional garage door of four 40 mm PU sandwich panels (about 56 kg) shown in vertical section: hinged panels with rollers ride a vertical, curved (R 0.5 m) and horizontal track, balanced by a torsion spring on a cable-drum shaft; closed, half open and open poses and the full opening motion.",
                 "Ein gedämmtes, 2,5 m breites und 2,0 m hohes Sektionaltor aus vier 40-mm-PU-Sandwichpaneelen (etwa 56 kg) im Vertikalschnitt: gelenkig verbundene Paneele mit Rollen laufen in einer senkrechten, gebogenen (R 0,5 m) und waagerechten Schiene, ausgeglichen durch eine Torsionsfeder auf einer Seiltrommelwelle; Posen geschlossen, halb offen und offen sowie die vollständige Öffnungsbewegung."),
    "tags": ["component", "door", "garage", "sectional", "mechanism", "spring", "conditioned"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Panel poses are solved analytically: the hinge rollers lie on the track path and every hinge-to-hinge chord is 0.5 m. Cable tension is 0.95 of the weight component along the panels, an authored estimate of a spring-balanced door.", "Die Panelposen sind analytisch berechnet: die Scharnierrollen liegen auf der Schienenbahn, jede Scharnier-zu-Scharnier-Sehne misst 0,5 m. Der Seilzug beträgt 0,95 der Gewichtskomponente entlang der Paneele, eine Schätzung für ein federausgeglichenes Tor.")},
    "materials": mats,
    "rig": {"kind": "mechanical", "root": "track", "bones": bones},
    "anchors": anchors, "representations": reps, "symbols": [],
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(0, 0, -12, 2, 0.8, 3, 300, 0.25), "operating": [], "regions": regions, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2 normal irradiance; the unheated-but-frost-free garage side is 10 degC (R_si 0.13 m2K/W).", "Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m2 Normalstrahlung; die unbeheizte, frostfreie Garagenseite hat 10 °C (R_si 0,13 m2K/W)."),
                    T("The sandwich U-value is 1 / (0.13 + 0.0001 + 1.82 + 0.04) = 0.50 W/m2K centre-of-panel; panel joints and rollers are thermal bridges that the hardware region approximates.", "Der Sandwich-U-Wert beträgt 1 / (0,13 + 0,0001 + 1,82 + 0,04) = 0,50 W/m2K in Paneelmitte; Paneelfugen und Rollen sind Wärmebrücken, die die Beschlagregion annähert."),
                    T("When the door is open the panels hang under the ceiling out of the sun; their sunlit fraction is 0 and they no longer separate exterior from interior.", "Bei geöffnetem Tor hängen die Paneele außerhalb der Sonne unter der Decke; ihr Besonnungsanteil ist 0 und sie trennen Außen nicht mehr von Innen.")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": ["jamb-fix", "hanger-1", "hanger-2"],
                  "assumptions": [T("Instantaneously rigid body: the track frame is fixed at the jamb and hung from the ceiling at two points; panels and shaft are masses at their poses. Rollers are modelled as perfectly guided.", "Augenblicklich starrer Körper: der Schienenrahmen ist am Pfosten befestigt und an zwei Punkten abgehängt; Paneele und Welle sind Massen in ihren Posen. Die Rollen gelten als ideal geführt."),
                                T("The spring cable pull and its equal reaction at the drum are internal forces of the door, shown so the joint loads of the hinge chain are visible; their magnitude is an authored estimate.", "Der Federseilzug und seine gleich große Reaktion an der Trommel sind innere Kräfte des Tores und werden gezeigt, damit die Gelenkkräfte der Scharnierkette sichtbar werden; ihre Größe ist ein Schätzwert."),
                                T("Wind is the EN 12424 class 3 pressure of about 700 Pa on a 2.5 m wide door, a point load per panel.", "Wind ist der Druck der Klasse 3 nach EN 12424 von etwa 700 Pa auf ein 2,5 m breites Tor, eine Einzellast je Paneel.")], "clinical": False},
    "presentation": {"defaultPose": "closed", "defaultClip": "open-close", "defaultScale": 50, "viewpoints": ["elevation"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": 2.5,
}
assemble(a, "sectional-garage-door")
