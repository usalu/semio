import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

WALL_T = 0.27
SILL_TOP, HEAD_BOT, HEAD_TOP = 1.0, 2.25, 2.75
BOX_X1, BOX_Y0, BOX_Y1 = 0.15, 2.32, 2.53
XC = 0.105
W_SLAT, PITCH = 0.09, 0.08
N = 17
Y_LOW = [BOX_Y0 + 0.005 - PITCH * i for i in range(N)]
RAIL_Y0 = Y_LOW[-1] - 0.025
RAIL_H = 0.04
PACK = 0.0075
IGU_X0, IGU_X1 = -0.20, -0.176
ALPHA = 35.0

def tau(beta_deg, alpha=ALPHA):
    a = math.radians(alpha); b = math.radians(beta_deg)
    return max(0.0, min(1.0, 1 - W_SLAT * math.sin(a + b) / (PITCH * math.cos(a))))

def heights(H):
    out = {}
    for i in range(N):
        pack = H + 0.03 + PACK * (N - 1 - i)
        out[i] = max(Y_LOW[i], pack) - Y_LOW[i]
    return out

bones = [bone("facade", None, "Facade wall stubs with the window (fixed root)", "Fassadenwandstücke mit Fenster (feste Wurzel)", 0, 0, 0, 0.45, fixed(),
              mass(355.0, -0.135, 1.3, "point", "authored-estimate", "en-iso-10456"))]
bones.append(bone_world("box", "facade", "Headrail box with guide rails", "Kastenblende mit Führungsschienen", 0, BOX_Y0, 0, BOX_X1, fixed(), bones,
                        mass(5.7, 0.075, 0.1, "point", "authored-estimate", "blind-catalogues")))
bones.append(bone_world("bottom-rail", "facade", "Bottom rail (lifts the stack)", "Unterschiene (hebt das Paket)", XC, RAIL_Y0, 0, 0.09, prismatic(90, 0, 1.45), bones,
                        mass(1.4, 0, 0, "point", "authored-estimate", "blind-catalogues")))
for i in range(N):
    bones.append(bone_world(f"slide-{i}", "facade", f"Lift of slat {i + 1}", f"Hub der Lamelle {i + 1}", XC, Y_LOW[i], 0, 0, prismatic(90, 0, 1.45), bones))
for i in range(N):
    bones.append(bone_world(f"slat-{i}", f"slide-{i}", f"Slat {i + 1} (top is 1) tilting about its centre", f"Lamelle {i + 1} (oben ist 1), kippt um ihre Mitte", XC, Y_LOW[i], 0, W_SLAT / 2, revolute(-90, 35), bones,
                            mass(0.14, 0, 0, "point", "authored-estimate", "blind-catalogues")))
D = Draw(bones)
GR = lambda: D.line("ground", "facade", "ground", 0, 0.5, [(-WALL_T, 0.0), (BOX_X1, 0.0)])
WALLS = [("sill", 0.0, SILL_TOP), ("head", HEAD_BOT, HEAD_TOP)]

def walls(role, hatch, z=1, pen=None):
    return [D.rect(f"wall-{n}", "facade", role, z, True, 0.5, -WALL_T, y0, 0.0, y1, region="facade-wall", material="masonry", hatch=hatch, penMm=pen) for n, y0, y1 in WALLS]

def slats(kind, pen=None, role="detail"):
    sh = []
    for i in range(N):
        y = Y_LOW[i]
        if kind == "line":
            sh.append(D.line(f"slat-{i}", f"slat-{i}", role, 3, 0.4, [(XC - W_SLAT / 2, y), (XC + W_SLAT / 2, y)], region="lamellae", material="aluminium", penMm=pen))
        else:
            sh.append(D.rect(f"slat-{i}", f"slat-{i}", role, 3, True, 0.35, XC - W_SLAT / 2, y - 0.006, XC + W_SLAT / 2, y + 0.006, region="lamellae", material="aluminium"))
    return sh

def s200():
    sh = [GR()] + walls("body", None, pen=0.13)
    sh.append(D.line("glass", "facade", "glazing", 2, 0.2, [(IGU_X0 + 0.012, SILL_TOP), (IGU_X0 + 0.012, HEAD_BOT)], region="glazing-behind", material="glass"))
    sh.append(D.rect("box", "box", "body", 5, True, 0.45, 0.0, BOX_Y0, BOX_X1, BOX_Y1, region="box-and-rails", material="aluminium", penMm=0.13))
    sh.append(D.line("guide", "box", "detail", 2, 0.3, [(XC, BOX_Y0), (XC, RAIL_Y0 + 0.01)], region="box-and-rails", penMm=0.1))
    sh += slats("line", 0.1)
    sh.append(D.line("bottom-rail", "bottom-rail", "detail", 4, 0.4, [(XC - 0.045, RAIL_Y0), (XC + 0.045, RAIL_Y0)], region="box-and-rails", material="aluminium", penMm=0.18))
    return sh

def s100():
    sh = [GR()] + walls("body", "cut-masonry", pen=0.18)
    sh.append(D.line("glass", "facade", "glazing", 2, 0.2, [(IGU_X0 + 0.012, SILL_TOP), (IGU_X0 + 0.012, HEAD_BOT)], region="glazing-behind", material="glass", penMm=0.13))
    sh.append(D.rect("box", "box", "body", 5, True, 0.45, 0.0, BOX_Y0, BOX_X1, BOX_Y1, region="box-and-rails", material="aluminium", penMm=0.18))
    sh.append(D.line("guide", "box", "detail", 2, 0.3, [(XC, BOX_Y0), (XC, RAIL_Y0 + RAIL_H / 2)], region="box-and-rails", penMm=0.13))
    sh += slats("line", 0.13)
    sh.append(D.rect("bottom-rail", "bottom-rail", "body", 4, True, 0.4, XC - 0.045, RAIL_Y0 - RAIL_H / 2, XC + 0.045, RAIL_Y0 + RAIL_H / 2, region="box-and-rails", material="aluminium", penMm=0.13))
    return sh

def s50():
    sh = [GR()]
    for n, y0, y1 in WALLS:
        sh.append(D.rect(f"block-{n}", "facade", "body", 1, True, 0.5, -WALL_T, y0, -0.085, y1, region="facade-wall", material="masonry", hatch="cut-masonry", penMm=0.25))
        sh.append(D.rect(f"insulation-{n}", "facade", "detail", 1, True, 0.7, -0.085, y0, 0.0, y1, region="facade-wall", material="eps", hatch="insulation"))
        sh.append(D.line(f"render-{n}", "facade", "detail", 2, 0.4, [(-0.0075, y0), (-0.0075, y1)], region="facade-wall", material="masonry", penMm=0.3))
    for x, nm in ((IGU_X0, "a"), (IGU_X1, "b")):
        sh.append(D.line(f"glass-{nm}", "facade", "glazing", 2, 0.2, [(x, SILL_TOP), (x, HEAD_BOT)], region="glazing-behind", material="glass"))
    sh.append(D.rect("box", "box", "body", 5, True, 0.45, 0.0, BOX_Y0, BOX_X1, BOX_Y1, region="box-and-rails", material="aluminium"))
    sh.append(D.line("box-slot", "box", "texture", 6, 0.3, [(0.03, BOX_Y0 + 0.015), (0.12, BOX_Y0 + 0.015)], region="box-and-rails"))
    for n, x in (("a", XC - 0.01), ("b", XC + 0.01)):
        sh.append(D.line(f"guide-{n}", "box", "detail", 2, 0.3, [(x, BOX_Y0), (x, RAIL_Y0 + RAIL_H / 2)], region="box-and-rails"))
    sh += slats("poly", None)
    sh.append(D.rect("bottom-rail", "bottom-rail", "body", 4, True, 0.4, XC - 0.045, RAIL_Y0 - RAIL_H / 2, XC + 0.045, RAIL_Y0 + RAIL_H / 2, region="box-and-rails", material="aluminium", penMm=0.25))
    return sh

reps = {}
reps["500"] = unavailable(500,
    ("The lamellae pitch is 0.08 m (0.16 mm at 1:500) and a lamella is 0.012 m thick; every tilt state would print as the same grey strip, so the asset's purpose (lamella angle and shading) cannot be shown.", "Der Lamellenabstand beträgt 0,08 m (0,16 mm bei 1:500) und eine Lamelle ist 0,012 m dick; jeder Kippzustand würde als derselbe graue Streifen drucken, der Zweck des Assets (Lamellenwinkel und Verschattung) ist nicht darstellbar."),
    ("Lamella pitch 0.08 m (0.16 mm) and lamella thickness 0.012 m (0.024 mm).", "Lamellenabstand 0,08 m (0,16 mm) und Lamellendicke 0,012 m (0,024 mm)."),
    0.08, [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}])
hat = [H("cut-masonry", "Cut masonry", "Geschnittenes Mauerwerk", "lines", 45, 0.5, 0.1)]
reps["200"] = rep(200, s200(), dec(
    ("A cut facade stub below and above the window, the glazing as a line, the headrail box as a filled block and 17 lamella lines hanging from it with a bottom rail.", "Ein geschnittenes Fassadenstück unter und über dem Fenster, die Verglasung als Linie, die Kastenblende als gefüllter Block und 17 daran hängende Lamellenlinien mit Unterschiene."),
    ("All 17 lamellae at their real pitch 0.08 m (0.4 mm) and width 0.09 m, box 0.15 x 0.21 m, guide line, bottom rail, glass line between the wall stubs.", "Alle 17 Lamellen im echten Abstand 0,08 m (0,4 mm) und mit 0,09 m Breite, Kasten 0,15 x 0,21 m, Führungslinie, Unterschiene, Glaslinie zwischen den Wandstücken."),
    ("The wall layers (render, insulation, block) are one filled stub; box, guide rails and bottom rail share the region box-and-rails.", "Die Wandschichten (Putz, Dämmung, Block) sind ein gefülltes Stück; Kasten, Führungsschienen und Unterschiene teilen die Region box-and-rails."),
    ("Lamella profile (Z-shape), cords, rollers, window frame.", "Lamellenprofil (Z-Form), Schnüre, Rollen, Fensterrahmen."),
    ("Each lamella (0.012 m real) is a 0.1 mm line (0.02 m at this scale); the IGU (0.024 m) is a 0.1 mm line. Named tilt poses keep the lines 0.28 mm apart or touching.", "Jede Lamelle (real 0,012 m) ist eine 0,1-mm-Linie (0,02 m in diesem Maßstab); die Isolierverglasung (0,024 m) eine 0,1-mm-Linie. Die benannten Kipp-Posen halten die Linien 0,28 mm auseinander oder berührend."),
    ("None: no hatch inside the stubs, only tone.", "Keine: keine Schraffur in den Stücken, nur Ton."),
    ("Filled dark stubs and box, light lamella lines.", "Gefüllte dunkle Stücke und Kasten, leichte Lamellenlinien."),
    ("Cells: wall stubs, glass line, box (with guide and rail), 17 lamellae that show the sunlit shading.", "Zellen: Wandstücke, Glaslinie, Kasten (mit Führung und Schiene), 17 Lamellen, die die besonnte Verschattung zeigen.")),
    pens_=pens(0.18, 0.1, None, 0.1, 0.25), rng=(150, 350))
reps["100"] = rep(100, s100(), dec(
    ("Hatched cut wall stubs, the glazing line, a box with two guide lines, 17 lamellae as 0.13 mm lines and a filled bottom rail.", "Schraffierte geschnittene Wandstücke, die Glaslinie, ein Kasten mit zwei Führungslinien, 17 Lamellen als 0,13-mm-Linien und eine gefüllte Unterschiene."),
    ("Lamella pitch 0.8 mm, width 0.9 mm; guide rail slot 0.02 m between two lines; bottom rail 0.09 x 0.04 m.", "Lamellenabstand 0,8 mm, Breite 0,9 mm; Führungsschiene 0,02 m zwischen zwei Linien; Unterschiene 0,09 x 0,04 m."),
    ("Wall layers merged into one hatched stub.", "Wandschichten in einem schraffierten Stück zusammengefasst."),
    ("Lamella profile, cords, window frame.", "Lamellenprofil, Schnüre, Fensterrahmen."),
    ("A 0.13 mm line equals the real 0.013 m lamella depth; the IGU is one 0.13 mm line instead of two (0.24 mm cannot separate).", "Eine 0,13-mm-Linie entspricht der realen Lamellentiefe von 0,013 m; die Isolierverglasung ist eine 0,13-mm-Linie statt zwei (0,24 mm lassen sich nicht trennen)."),
    ("Cut masonry as 45 degree lines at 0.5 mm.", "Geschnittenes Mauerwerk als 45-Grad-Linien mit 0,5 mm."),
    ("Walls 0.18 mm, box 0.18 mm, lamellae 0.13 mm.", "Wände 0,18 mm, Kasten 0,18 mm, Lamellen 0,13 mm."),
    ("As at 1:200 with the guide split into two lines.", "Wie bei 1:200, die Führung in zwei Linien geteilt.")),
    hatches=hat, pens_=pens(0.25, 0.13, 0.1, 0.1, 0.35), rng=(75, 150))
reps["50"] = rep(50, s50(), dec(
    ("The vertical section of an external venetian blind in front of a double-glazed window in an insulated wall: layered wall stubs, IGU as two lines, headrail box with slot, guide rails and 17 lamellae as thin filled profiles tilting about their centres.", "Der Vertikalschnitt einer Außenjalousie vor einem doppelt verglasten Fenster in einer gedämmten Wand: geschichtete Wandstücke, Isolierverglasung als zwei Linien, Kastenblende mit Schlitz, Führungsschienen und 17 Lamellen als dünne gefüllte Profile, die um ihre Mitte kippen."),
    ("Wall layers (block 0.175 m, EPS 0.07 m, render 0.015 m as a 0.3 mm line), both glass panes, box slot, two guide lines, bottom rail.", "Wandschichten (Block 0,175 m, EPS 0,07 m, Putz 0,015 m), beide Glasscheiben, Kastenschlitz, zwei Führungslinien, Unterschiene."),
    ("Nothing merged.", "Nichts zusammengefasst."),
    ("Z-curl of the lamella edges (drawn as 0.09 x 0.012 m bars), the argon gap, the spacer.", "Z-Kante der Lamellen (als Balken 0,09 x 0,012 m gezeichnet), Argonraum, Abstandhalter."),
    ("Lamellae are 0.24 mm bars (real 0.012 m): thin but printable.", "Lamellen sind 0,24-mm-Balken (real 0,012 m): dünn, aber druckbar."),
    ("Cut masonry as lines at 0.5 mm, EPS as stipple.", "Geschnittenes Mauerwerk als Linien mit 0,5 mm, EPS als Punktierung."),
    ("Block outline 0.25 mm, box 0.35 mm, lamellae 0.13 mm.", "Blockkontur 0,25 mm, Kasten 0,35 mm, Lamellen 0,13 mm."),
    ("One cell per region; each lamella is its own cell so tilt and shade stay readable.", "Eine Zelle je Region; jede Lamelle ist eine eigene Zelle, damit Kippung und Schatten lesbar bleiben.")),
    hatches=hat[:1] and [H("cut-masonry", "Cut masonry", "Geschnittenes Mauerwerk", "lines", 45, 0.5, 0.1), H("insulation", "Insulation", "Dämmung", "stipple", 0, 0.5, 0.1)], pens_=pens(0.35, 0.18, 0.1, 0.1, 0.5), rng=(20, 75))
# the 1:50 slats use detail pen 0.13
reps["50"]["pens"]["detail"] = 0.13
reps["50"]["pens"]["glazing"] = 0.13
reps["50"]["pens"]["texture"] = 0.1

REFS = [
    ref("en-13561", "EN 13561:2015 External blinds and awnings, performance requirements including safety (wind resistance classes, operation). The wind pressure used here (0.3 kPa) is an authored estimate within the class 2 to 3 range; the class values were not verified."),
    ref("en-14501", "EN 14501:2021 Blinds and shutters, thermal and visual comfort, performance characteristics and classification (solar transmittance of the blind with glazing)."),
    ref("blind-catalogues", "Manufacturer data for external venetian blinds (raffstore) with Z-lamellae 90 mm, pitch about 80 mm, lamella thickness 0.4 to 0.5 mm aluminium (about 0.13 kg/m per lamella), headrail box 150 x 200 mm, bottom rail and lateral guide rails. Typical values consulted."),
    ref("en-iso-10077-1", "EN ISO 10077-1:2017 Thermal performance of windows, doors and shutters: double glazing 4-16-4 with argon and low-e coating Ug about 1.1 W/m2K."),
    ref("en-iso-6946", "EN ISO 6946:2017 Building components, thermal resistance and transmittance (R_si 0.13, R_se 0.04 m2K/W)."),
    ref("en-iso-10456", "EN ISO 10456:2007 Building materials, hygrothermal properties: EPS 0.035 W/mK, aluminium 160 W/mK 880 J/kgK, glass 1.0 W/mK, clay block (light) 0.12 W/mK."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, types of drawings, scales and symbols."),
    ref("louvre-geometry", "Direct-beam transmission of a louvre array: tau = max(0, 1 - w sin(alpha + beta) / (p cos alpha)) for lamella width w, pitch p, solar profile angle alpha and lamella tilt beta (outer edge down positive), elementary geometry of parallel opaque strips (e.g. Kuhn, Bühler, Platzer 2000, Solar Energy 69 suppl. 6)."),
]
mats = [
    mat("aluminium", "Anodised or lacquered aluminium, white", "Aluminium, eloxiert oder lackiert, weiß", 0.85, 0.3, 2800, 160, 880, 70e9, 150e6, "literature", "en-iso-10456"),
    mat("glass", "Float glass 4 mm", "Floatglas 4 mm", 0.837, 0.08, 2500, 1.0, 750, 70e9, None, "literature", "en-iso-10456"),
    mat("masonry", "Insulated clay-block wall, rendered", "Gedämmte Ziegelwand, verputzt", 0.9, 0.5, 800, 0.12, 1000, None, None, "literature", "en-iso-10456"),
    mat("eps", "Expanded polystyrene", "Expandiertes Polystyrol", 0.9, 0.5, 15, 0.035, 1450, None, None, "literature", "en-iso-10456"),
]
regions = [
    region("lamellae", "Venetian lamellae", "Jalousielamellen", "surface", "passive", "aluminium", 2 * N * W_SLAT * 1.2, 0.09, "vertical-plate", 0.5,
           [layer("lam-sheet", "Aluminium strip 0.45 mm", "Aluminiumband 0,45 mm", "metal", "aluminium", 0.00045, 0.00045 / 160, "literature", "en-iso-10456", c=0.00045 * 2800 * 880)], 1100, "literature", "blind-catalogues"),
    region("box-and-rails", "Headrail box, guide rails and bottom rail", "Kastenblende, Führungsschienen und Unterschiene", "surface", "passive", "aluminium", 1.2, 0.2, "vertical-plate", 0.6,
           [layer("box-wall", "Aluminium extrusion 1.5 mm", "Aluminiumprofil 1,5 mm", "metal", "aluminium", 0.0015, 0.0015 / 160, "literature", "en-iso-10456", c=0.0015 * 2800 * 880)], 3700, "literature", "blind-catalogues"),
    region("glazing-behind", "Double glazing behind the blind", "Doppelverglasung hinter der Jalousie", "glazing", "conditioned", "glass", 1.2 * (HEAD_BOT - SILL_TOP), 1.23, "vertical-plate", 0.5,
           [layer("pane-out", "Outer pane 4 mm", "Außenscheibe 4 mm", "glass", "glass", 0.004, 0.004, "literature", "en-iso-10456", c=0.004 * 2500 * 750),
            layer("cavity", "Argon cavity 16 mm", "Argonzwischenraum 16 mm", "air-gap", "glass", 0.016, 0.73, "literature", "en-iso-10077-1"),
            layer("pane-in", "Inner pane 4 mm, low-e", "Innenscheibe 4 mm, Low-E", "glass", "glass", 0.004, 0.004, "literature", "en-iso-10456", c=0.004 * 2500 * 750)],
           15000, "literature", "en-iso-10077-1", conditioned=(20, 0.13)),
    region("facade-wall", "Insulated wall above and below the window", "Gedämmte Wand über und unter dem Fenster", "wall", "conditioned", "masonry", 1.2 * (SILL_TOP + HEAD_TOP - HEAD_BOT), 0.5, "vertical-plate", 0.5,
           [layer("plaster", "Gypsum plaster 15 mm", "Gipsputz 15 mm", "masonry", "masonry", 0.015, 0.026, "literature", "en-iso-10456", c=0.015 * 1300 * 1000),
            layer("block", "Light clay block 175 mm", "Leichtziegel 175 mm", "masonry", "masonry", 0.175, 0.175 / 0.12, "literature", "en-iso-10456", c=0.175 * 800 * 1000),
            layer("eps", "EPS 70 mm", "EPS 70 mm", "insulation", "eps", 0.07, 0.07 / 0.035, "literature", "en-iso-10456", c=0.07 * 15 * 1450),
            layer("render", "Mineral render 10 mm", "Mineralputz 10 mm", "masonry", "masonry", 0.01, 0.01 / 0.8, "literature", "en-iso-10456", c=0.01 * 1800 * 1000)],
           60000, "literature", "en-iso-6946", conditioned=(20, 0.13)),
]
anchors = [anchor("wall-foot", "facade", 0, 0, "support", "Wall foot, fixed in the slab", "Wandfuß, in der Decke eingespannt", support="fixed"),
           anchor("blind-centre", "facade", XC, 1.7, "load", "Centre of the lowered blind", "Mitte der abgelassenen Jalousie"),
           anchor("box-bracket", "box", 0, 0.1, "attachment", "Box bracket on the head wall", "Kastenhalter an der Sturzwand")]
def regs(sun_gl, lam=(1.0, 1.0), box=(1.0, 0.6), wall=(1.0, 0.8)):
    return {"lamellae": RB(*lam), "box-and-rails": RB(*box), "glazing-behind": RB(1.0, sun_gl), "facade-wall": RB(*wall)}
def pose_state(H, beta):
    off = {"bottom-rail": {"dy": H - RAIL_Y0}}
    dys = heights(H)
    for i in range(N):
        if dys[i] > 1e-9:
            off[f"slide-{i}"] = {"dy": dys[i]}
        if beta:
            off[f"slat-{i}"] = {"angleDeg": -beta}
    if not beta:
        pass
    return off
RET = 2.37
wind = load("wind-gust", "wind", "Wind pressure 0.3 kPa on the lowered blind (0.5 kN)", "Winddruck 0,3 kPa auf die abgelassene Jalousie (0,5 kN)", -504, 0, "authored-estimate", "en-13561", anchor_="blind-centre")
poses = [
    pose("lowered-horizontal", "Lowered, slats horizontal", "Abgelassen, Lamellen waagerecht", "Blind lowered over the window with the slats flat; at 35 degrees sun elevation 21 % of the direct beam passes.", "Jalousie vor dem Fenster abgelassen, Lamellen flach; bei 35 Grad Sonnenhöhe tritt 21 % der Direktstrahlung durch.", {}, [], regs(round(tau(0), 3))),
    pose("retracted", "Retracted into the box", "In den Kasten hochgezogen", "Blind raised, the lamellae stacked in the headrail box, glazing fully sunlit.", "Jalousie hochgezogen, Lamellen im Kasten gestapelt, Verglasung voll besonnt.", pose_state(RET, 0), [], regs(1.0, lam=(0.05, 0.0), box=(1.0, 0.7))),
    pose("lowered-cut-off-45", "Lowered, slats at 45 degrees", "Abgelassen, Lamellen 45 Grad", "Slats tilted 45 degrees (outer edge down): no direct beam reaches the glazing at 35 degrees sun elevation.", "Lamellen 45 Grad geneigt (Außenkante unten): bei 35 Grad Sonnenhöhe erreicht keine Direktstrahlung die Verglasung.", pose_state(Y_LOW[0] * 0 + RAIL_Y0, 45), [], regs(round(tau(45), 3))),
    pose("lowered-closed", "Lowered, slats closed", "Abgelassen, Lamellen geschlossen", "Slats at 90 degrees: the blind is a closed screen.", "Lamellen bei 90 Grad: die Jalousie ist ein geschlossener Schirm.", pose_state(RAIL_Y0, 90), [], regs(0.0)),
    pose("closed-in-gust", "Closed in a wind gust", "Geschlossen in einer Windböe", "Closed screen under 0.3 kPa wind pressure, pushed against the facade.", "Geschlossener Schirm unter 0,3 kPa Winddruck, gegen die Fassade gedrückt.", pose_state(RAIL_Y0, 90), [wind], regs(0.0, wall=(1.0, 0.5))),
]
KEYS = []
for j, Hj in enumerate([1.38, 1.72, 2.06, 2.30]):
    poses.append(pose(f"lift-{j + 1}", f"Lift {j + 1}", f"Hub {j + 1}", "In-between key pose of lifting.", "Zwischenpose des Hochziehens.", pose_state(Hj, 0), [], regs(round(min(1.0, 0.21 + (Hj - RAIL_Y0) / 1.4 * 0.79), 3), lam=(1.0, 1.0)), key=True))
for j, bj in enumerate([-30, 20, 30, 60, 75]):
    poses.append(pose(f"tilt-{j + 1}", f"Tilt {bj}", f"Kippung {bj}", "In-between key pose of tilting.", "Zwischenpose des Kippens.", pose_state(RAIL_Y0, bj), [], regs(round(tau(bj), 3)), key=True))
sweep = [-30, 0, 20, 45, 75, 90]
tilt_keys = [key(2 * i, "lowered-horizontal" if b == 0 else "lowered-cut-off-45" if b == 45 else "lowered-closed" if b == 90 else f"tilt-{[-30, 20, 30, 60, 75].index(b) + 1}") for i, b in enumerate(sweep)]
tilt_keys += [key(10 + 2 * (i + 1), "lowered-horizontal" if b == 0 else "lowered-cut-off-45" if b == 45 else f"tilt-{[-30, 20, 30, 60, 75].index(b) + 1}") for i, b in enumerate(sweep[-2::-1])]
tilt_t = [t for t in range(0, 22, 2)]
tau_track = track("region:glazing-behind:sunlitFraction", [(k["t"], dict(zip(["x"], [0]))["x"]) for k in tilt_keys[:0]] or [])
bseq = sweep + sweep[-2::-1]
tilt_keys = [key(2 * i, ("lowered-horizontal" if b == 0 else "lowered-cut-off-45" if b == 45 else "lowered-closed" if b == 90 else f"tilt-{[-30, 20, 30, 60, 75].index(b) + 1}")) for i, b in enumerate(bseq)]
tau_keys = [(2 * i, round(tau(b), 3)) for i, b in enumerate(bseq)]
lift_seq = ["lowered-horizontal", "lift-1", "lift-2", "lift-3", "lift-4", "retracted"]
lift_keys = [key(2 * i, p) for i, p in enumerate(lift_seq)] + [key(12 + 2 * (i + 1), p) for i, p in enumerate(lift_seq[-2::-1])]
lift_tau = [(2 * i, [round(tau(0), 3), None, None, None, None, 1.0][i] if False else None) for i in range(0)]
clips = [
    clip("raise-lower", "Raise and lower", "Hochziehen und Ablassen", "The bottom rail rises and the lamellae stack up into the box one after the other; the blind is then lowered again.", "Die Unterschiene steigt und die Lamellen stapeln sich nacheinander im Kasten; danach wird die Jalousie wieder abgelassen.",
         22, True, lift_keys, [phase("raise", 0, 10, "Raising", "Hochziehen"), phase("lower", 12, 22, "Lowering", "Ablassen")],
         [event(10, "motion", "Fully retracted", "Ganz hochgezogen"), event(0, "motion", "Fully lowered", "Ganz abgelassen")],
         [track("region:glazing-behind:sunlitFraction", [(0, round(tau(0), 3)), (4, 0.4), (8, 0.8), (10, 1.0), (12, 1.0), (16, 0.7), (20, 0.35), (22, round(tau(0), 3))]),
          track("region:lamellae:exposedFraction", [(0, 1.0), (6, 0.5), (10, 0.05), (12, 0.05), (18, 0.7), (22, 1.0)])]),
    clip("tilt-sweep", "Tilt sweep", "Kippbewegung", "The lamellae tilt from slightly upward through horizontal and cut-off to closed and back; the sunlit fraction of the glazing follows the louvre geometry.", "Die Lamellen kippen von leicht aufwärts über waagerecht und Sonnenschutzstellung bis geschlossen und zurück; der Besonnungsanteil der Verglasung folgt der Lamellengeometrie.",
         22, True, tilt_keys, [phase("close", 0, 10, "Closing the slats", "Lamellen schließen"), phase("open", 10, 22, "Opening the slats", "Lamellen öffnen")],
         [event(10, "motion", "Slats closed", "Lamellen geschlossen")], [track("region:glazing-behind:sunlitFraction", tau_keys)]),
    clip("sun-and-shade", "Sun moves over the facade", "Sonne wandert über die Fassade", "A cloud shadow crosses the lowered blind; the glazing, lamella and wall sunlit fractions dip and recover.", "Ein Wolkenschatten zieht über die abgelassene Jalousie; die Besonnungsanteile von Verglasung, Lamellen und Wand sinken und erholen sich.",
         10, True, [key(0, "lowered-horizontal"), key(10, "lowered-horizontal")], [phase("shadow", 0, 10, "Shadow", "Schatten")], [event(5, "thermal", "Deepest shade", "Tiefster Schatten")],
         [track("region:lamellae:sunlitFraction", [(0, 1.0), (5, 0.1), (10, 1.0)]), track("region:glazing-behind:sunlitFraction", [(0, 0.212), (5, 0.02), (10, 0.212)]), track("region:facade-wall:sunlitFraction", [(0, 0.8), (5, 0.08), (10, 0.8)])]),
]
a = {
    "id": "external-venetian-blind", "category": "components",
    "title": T("External Venetian Blind", "Außenjalousie"),
    "summary": T("A 1.2 m wide external venetian blind (raffstore) of 17 aluminium Z-lamellae (90 mm wide, 80 mm pitch) in a 150 x 210 mm headrail box with guide rails and bottom rail, shown in vertical section in front of a double-glazed window in an insulated wall; retracted, lowered with slats horizontal, at 45 degrees or closed, with the shading effect on the glazing from the louvre geometry.",
                 "Eine 1,2 m breite Außenjalousie (Raffstore) aus 17 Aluminium-Z-Lamellen (90 mm breit, 80 mm Abstand) in einer 150 x 210 mm großen Kastenblende mit Führungsschienen und Unterschiene, im Vertikalschnitt vor einem doppelt verglasten Fenster in einer gedämmten Wand; hochgezogen, abgelassen mit waagerechten, 45-Grad- oder geschlossenen Lamellen, mit der Verschattungswirkung auf die Verglasung aus der Lamellengeometrie."),
    "tags": ["component", "shading", "blind", "louvre", "facade", "mechanism", "conditioned"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Glazing sunlit fractions are the direct-beam transmission of the louvre array at a reference solar profile angle of 35 degrees (tau = 1 - w sin(alpha + beta) / (p cos alpha)); diffuse light is not blocked.", "Die Besonnungsanteile der Verglasung sind die Direktstrahlungs-Durchlässigkeit der Lamellenfläche bei einem Referenz-Sonnenhöhenwinkel von 35 Grad (tau = 1 - w sin(alpha + beta) / (p cos alpha)); diffuses Licht wird nicht abgeschirmt.")},
    "materials": mats,
    "rig": {"kind": "mechanical", "root": "facade", "bones": bones},
    "anchors": anchors, "representations": reps, "symbols": [],
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(32, 36, 21, 50, 0.4, 1.5, 850, 0.12), "operating": [], "regions": regions, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Summer reference case (hot sun: 32 degC, 850 W/m2); interior 20 degC (cooled), R_si 0.13 m2K/W.", "Sommer-Referenzfall (heiße Sonne: 32 °C, 850 W/m2); Innenraum 20 °C (gekühlt), R_si 0,13 m2K/W."),
                    T("The sunlit fraction of the glazing is the direct-beam transmission of the louvres at a 35 degree solar profile angle for w = 0.09 m and p = 0.08 m: 1.0 retracted, 0.21 with flat slats, 0 at 45 and 90 degrees; the blind is a separate passive region that absorbs what the glazing does not see.", "Der Besonnungsanteil der Verglasung ist die Direktstrahlungs-Durchlässigkeit der Lamellen bei 35 Grad Sonnenhöhenwinkel für w = 0,09 m und p = 0,08 m: 1,0 hochgezogen, 0,21 bei flachen Lamellen, 0 bei 45 und 90 Grad; die Jalousie ist eine eigene passive Region, die aufnimmt, was die Verglasung nicht sieht."),
                    T("U of the glazing 1.1 W/m2K (EN ISO 10077-1) and of the wall 0.27 W/m2K; the convective and radiative coupling of the air gap between blind and glass is left to the surface model.", "U der Verglasung 1,1 W/m2K (EN ISO 10077-1) und der Wand 0,27 W/m2K; die konvektive und Strahlungskopplung des Luftspalts zwischen Jalousie und Glas bleibt dem Oberflächenmodell überlassen.")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": ["wall-foot"],
                  "assumptions": [T("The wall stubs form the rigid fixed root; box, bottom rail and every lamella are masses at their poses (lamella pack positions are exact).", "Die Wandstücke bilden die starre, eingespannte Wurzel; Kasten, Unterschiene und jede Lamelle sind Massen in ihren Posen (Lamellenpaketpositionen sind exakt)."),
                                T("Wind of 0.3 kPa on the 1.2 x 1.4 m screen is an authored estimate; EN 13561 class values were not verified.", "Wind von 0,3 kPa auf den 1,2 x 1,4 m großen Schirm ist ein Schätzwert; die Klassenwerte der EN 13561 wurden nicht geprüft.")], "clinical": False},
    "presentation": {"defaultPose": "lowered-horizontal", "defaultClip": "raise-lower", "defaultScale": 50, "viewpoints": ["elevation"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": 1.2,
}
assemble(a, "external-venetian-blind")
