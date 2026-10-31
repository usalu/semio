import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-C")
from lib import *

BW, BH = 1.40, 3.30
MW = 0.05
SP_TOP = 0.85
T0, T1, T2 = (0.0, 0.05), (0.85, 0.90), (3.25, 3.30)
VIS = (0.90, 3.25)
GX0, GX1 = MW, BW - MW
EDGE = 0.065
A_GLASS = (GX1 - GX0) * (VIS[1] - VIS[0])
A_CENTRE = (GX1 - GX0 - 2 * EDGE) * (VIS[1] - VIS[0] - 2 * EDGE)
A_EDGE = A_GLASS - A_CENTRE
A_SPAN = (GX1 - GX0) * (SP_TOP - 0.05)
A_MULL = 2 * MW * BH
A_TRAN = 3 * (GX1 - GX0) * MW
MASS = {"frame": 2 * 3.3 * 5.5 + 3 * 1.3 * 4.5 + 4.0, "igu": 2 * 6e-3 * 2500 * A_GLASS + 4.0, "spandrel": (6e-3 * 2500 + 5.0 + 11.8) * A_SPAN}
bones = [bone("frame", None, "Mullion-transom frame (fixed root, hung from the slab)", "Pfosten-Riegel-Rahmen (feste Wurzel, an der Decke aufgehängt)", 0, 0, 0, BW, fixed(),
              mass(MASS["frame"], BW / 2, BH / 2, "point", "literature", "profile-catalogues"),
              {"areaM2": 0.00135, "secondMomentM4": 1.4e-6, "extremeFibreM": 0.075, "material": "aluminium", "title": T("Mullion 50 x 150 mm, aluminium", "Pfosten 50 x 150 mm, Aluminium")})]
bones.append(bone_world("igu", "frame", "Insulated glazing unit 6-16-6", "Isolierglaseinheit 6-16-6", GX0, VIS[0], 0, GX1 - GX0, fixed(), bones,
                        mass(MASS["igu"], (GX1 - GX0) / 2, (VIS[1] - VIS[0]) / 2, "point", "literature", "en-572")))
bones.append(bone_world("spandrel", "frame", "Insulated spandrel panel", "Gedämmtes Brüstungspaneel", GX0, 0.05, 0, GX1 - GX0, fixed(), bones,
                        mass(MASS["spandrel"], (GX1 - GX0) / 2, (SP_TOP - 0.05) / 2, "point", "authored-estimate", "profile-catalogues")))
bones.append(bone_world("cradle", "frame", "Maintenance cradle hook on the mid transom (massless)", "Wartungsgondel-Haken am mittleren Riegel (masselos)", GX0, T1[1] - 0.025, 0, 0, prismatic(0, 0, GX1 - GX0), bones))
D = Draw(bones)
GR = lambda: D.line("ground", "frame", "ground", 0, 0.5, [(0.0, 0.0), (BW, 0.0)])
def rect(id, bone_, role, z, tone, x0, y0, x1, y1, **kw):
    return D.rect(id, bone_, role, z, True, tone, x0, y0, x1, y1, **kw)

def s500():
    return [GR(),
            rect("spandrel", "spandrel", "body", 1, 0.65, 0, 0, BW, SP_TOP, regions=["spandrel"], material="spandrel-glass"),
            rect("vision", "igu", "body", 1, 0.15, 0, SP_TOP, BW, BH, regions=["glass-centre", "glass-edge", "mullion", "transom"], material="glass")]

def s200():
    sh = [GR()]
    sh.append(rect("spandrel", "spandrel", "body", 1, 0.65, GX0, T0[1], GX1, T1[0], region="spandrel", material="spandrel-glass"))
    sh.append(D.rect("glass", "igu", "glazing", 1, True, 0.15, GX0, VIS[0], GX1, VIS[1], regions=["glass-centre", "glass-edge"], material="glass"))
    for n, x0 in (("left", 0.0), ("right", BW - MW)):
        sh.append(rect(f"mullion-{n}", "frame", "detail", 3, 0.4, x0, 0.0, x0 + MW, BH, region="mullion", material="aluminium"))
    for n, (a, b) in (("base", T0), ("mid", T1), ("head", T2)):
        sh.append(rect(f"transom-{n}", "frame", "detail", 3, 0.4, GX0, a, GX1, b, region="transom", material="aluminium"))
    return sh

def s100():
    sh = [GR()]
    sh.append(rect("spandrel", "spandrel", "body", 1, 0.65, GX0, T0[1], GX1, T1[0], region="spandrel", material="spandrel-glass"))
    sh.append(D.rect("glass", "igu", "glazing", 1, True, 0.15, GX0, VIS[0], GX1, VIS[1], regions=["glass-centre", "glass-edge"], material="glass"))
    for n, x0 in (("left", 0.0), ("right", BW - MW)):
        sh.append(rect(f"mullion-{n}", "frame", "body", 3, 0.4, x0, 0.0, x0 + MW, BH, region="mullion", material="aluminium"))
    for n, (a, b) in (("base", T0), ("mid", T1), ("head", T2)):
        sh.append(rect(f"transom-{n}", "frame", "body", 3, 0.4, GX0, a, GX1, b, region="transom", material="aluminium"))
    sh.append(D.line("reflex-a", "igu", "glazing", 2, 0.2, [(0.25, 0.3), (0.45, 0.8)], region="glass-centre"))
    sh.append(D.line("reflex-b", "igu", "glazing", 2, 0.2, [(0.35, 0.3), (0.55, 0.8)], region="glass-centre"))
    return sh

def s50():
    sh = [GR()]
    sh.append(rect("spandrel", "spandrel", "body", 1, 0.65, GX0, T0[1], GX1, T1[0], region="spandrel", material="spandrel-glass"))
    sh.append(D.rect("spandrel-box", "spandrel", "detail", 2, False, 0.4, GX0 + 0.06, 0.11, GX1 - 0.06, SP_TOP - 0.06, region="spandrel"))
    sh.append(D.rect("glass", "igu", "glazing", 1, True, 0.15, GX0, VIS[0], GX1, VIS[1], region="glass-centre", material="glass"))
    sh.append(D.rect("spacer", "igu", "detail", 2, False, 0.4, GX0 + EDGE, VIS[0] + EDGE, GX1 - EDGE, VIS[1] - EDGE, region="glass-edge", material="glass"))
    for n, x0 in (("left", 0.0), ("right", BW - MW)):
        sh.append(rect(f"mullion-{n}", "frame", "body", 3, 0.4, x0, 0.0, x0 + MW, BH, region="mullion", material="aluminium"))
    for n, (a, b) in (("base", T0), ("mid", T1), ("head", T2)):
        sh.append(rect(f"transom-{n}", "frame", "body", 3, 0.4, GX0, a, GX1, b, region="transom", material="aluminium"))
    sh.append(D.line("reflex-a", "igu", "glazing", 3, 0.2, [(0.25, 0.3), (0.45, 0.8)], region="glass-centre"))
    sh.append(D.line("reflex-b", "igu", "glazing", 3, 0.2, [(0.35, 0.3), (0.55, 0.8)], region="glass-centre"))
    return sh

rng = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
reps = {}
reps["500"] = rep(500, s500(), dec(
    ("Two filled blocks: a dark spandrel and a light vision field above it, outlined by the frame.", "Zwei gefüllte Blöcke: eine dunkle Brüstung und darüber das helle Sichtfeld, vom Rahmen umrissen."),
    ("Bay 1.40 x 3.30 m, spandrel 0.85 m high (1.7 mm), vision field 2.45 m.", "Feld 1,40 x 3,30 m, Brüstung 0,85 m hoch (1,7 mm), Sichtfeld 2,45 m."),
    ("Mullions (0.05 m = 0.1 mm), transoms and both glass zones are one block listing their regions; the 0.13 mm outline stands for the frame.", "Pfosten (0,05 m = 0,1 mm), Riegel und beide Glaszonen sind ein Block mit den Regionen; die 0,13-mm-Kontur steht für den Rahmen."),
    ("Spacer line, reflections, individual members.", "Abstandhalterlinie, Reflexe, einzelne Profile."),
    ("The frame is the outline (0.13 mm = 0.065 m, wider than the real 0.05 m).", "Der Rahmen ist die Kontur (0,13 mm = 0,065 m, breiter als die realen 0,05 m)."),
    ("None.", "Keine."), ("Light glass, dark spandrel, black outline.", "Helles Glas, dunkle Brüstung, schwarze Kontur."),
    ("Cells: spandrel, vision block (glass, frame merged).", "Zellen: Brüstung, Sichtblock (Glas und Rahmen zusammengefasst).")),
    pens_=pens(0.13, None, None, None, 0.18), rng=rng["500"])
reps["200"] = rep(200, s200(), dec(
    ("The grid of a mullion-transom bay: two mullion bars, three transom bars, a filled spandrel and a light glass pane.", "Das Raster eines Pfosten-Riegel-Feldes: zwei Pfostenbalken, drei Riegelbalken, eine gefüllte Brüstung und eine helle Glasscheibe."),
    ("Mullions and transoms as filled bars of their real 0.05 m width (0.25 mm), spandrel 0.8 m, glass 2.35 m.", "Pfosten und Riegel als gefüllte Balken ihrer realen Breite 0,05 m (0,25 mm), Brüstung 0,8 m, Glas 2,35 m."),
    ("Both glass zones (centre and edge) are one pane.", "Beide Glaszonen (Mitte und Rand) sind eine Scheibe."),
    ("Spacer line, reflections, spandrel box.", "Abstandhalterlinie, Reflexe, Brüstungskasten."),
    ("Frame members are filled at their real width, nothing thickened.", "Rahmenprofile sind in ihrer realen Breite gefüllt, nichts verdickt."),
    ("None.", "Keine."), ("Frame bars 0.1 mm outline, dark; glass 0.1 mm; spandrel dark.", "Rahmenbalken 0,1-mm-Kontur, dunkel; Glas 0,1 mm; Brüstung dunkel."),
    ("Cells: spandrel, glass (centre and edge merged), mullions, transoms.", "Zellen: Brüstung, Glas (Mitte und Rand zusammengefasst), Pfosten, Riegel.")),
    pens_=pens(0.18, 0.1, None, 0.1, 0.25), rng=rng["200"])
reps["100"] = rep(100, s100(), dec(
    ("Filled mullions and transoms of true width around a glass pane with two reflection strokes and a dark spandrel.", "Gefüllte Pfosten und Riegel in wahrer Breite um eine Glasscheibe mit zwei Reflexstrichen und eine dunkle Brüstung."),
    ("Mullions 0.5 mm, transoms 0.5 mm, reflection strokes, spandrel 0.8 m.", "Pfosten 0,5 mm, Riegel 0,5 mm, Reflexstriche, Brüstung 0,8 m."),
    ("Glass centre and edge zone are one pane.", "Glasmitte und Randzone sind eine Scheibe."), ("Spacer line, spandrel box.", "Abstandhalterlinie, Brüstungskasten."),
    ("None thickened.", "Nichts verdickt."), ("None.", "Keine."), ("Frame outline 0.25 mm, glass strokes 0.1 mm.", "Rahmenkontur 0,25 mm, Glasstriche 0,1 mm."),
    ("Cells: spandrel, glass, mullions, transoms.", "Zellen: Brüstung, Glas, Pfosten, Riegel.")),
    pens_=pens(0.25, 0.13, 0.1, 0.1, 0.35), rng=rng["100"])
reps["50"] = rep(50, s50(), dec(
    ("The full bay: mullions and transoms, the IGU with its edge-seal rectangle 0.065 m inside the glass edge, two reflection strokes, the spandrel with its shadow-box outline.", "Das volle Feld: Pfosten und Riegel, die Isolierverglasung mit ihrem Randverbund-Rechteck 0,065 m innerhalb des Glasrandes, zwei Reflexstriche, die Brüstung mit ihrem Schattenkastenumriss."),
    ("The edge zone (0.065 m, EN ISO 10077-1 convention) as its own line and region, the spandrel box inset 0.06 m.", "Die Randzone (0,065 m, Konvention nach EN ISO 10077-1) als eigene Linie und Region, der Brüstungskasten 0,06 m eingerückt."),
    ("Nothing merged.", "Nichts zusammengefasst."), ("Pressure plate screws, gaskets, thermal break (not visible in elevation).", "Pressleistenschrauben, Dichtungen, thermische Trennung (in der Ansicht nicht sichtbar)."),
    ("Members are filled at real width, nothing thickened.", "Profile sind in realer Breite gefüllt, nichts verdickt."),
    ("None.", "Keine."), ("Frame outline 0.35 mm, details 0.18 mm, reflexes 0.13 mm.", "Rahmenkontur 0,35 mm, Details 0,18 mm, Reflexe 0,13 mm."),
    ("One cell per region: spandrel, glass centre, edge zone line, mullions, transoms.", "Eine Zelle je Region: Brüstung, Glasmitte, Randzonenlinie, Pfosten, Riegel.")),
    pens_=pens(0.35, 0.18, 0.1, 0.13, 0.5), rng=rng["50"])

REFS = [
    ref("en-13830", "EN 13830:2015 Curtain walling, product standard (mullion-transom systems, wind load, thermal transmittance U_cw)."),
    ref("en-iso-12631", "EN ISO 12631:2017 Thermal performance of curtain walling, calculation of thermal transmittance (component method: U_g, U_f, psi_g, spandrel panels)."),
    ref("en-iso-10077-2", "EN ISO 10077-2:2017 Thermal performance of windows, doors and shutters, frames: thermally broken aluminium profiles U_f about 1.8 to 2.8 W/m2K (2.0 used for mullions, 2.2 for transoms)."),
    ref("en-iso-10077-1", "EN ISO 10077-1:2017: Ug 1.1 W/m2K for 6-16-6 argon low-e glazing, edge-of-glass linear transmittance psi_g 0.04 W/mK for a warm-edge spacer, edge zone 0.065 m."),
    ref("en-572", "EN 572-1:2012 Glass in building, basic soda lime silicate glass products (density 2500 kg/m3, E 70 GPa)."),
    ref("profile-catalogues", "Manufacturer data for mullion-transom facade systems: mullion 50 x 150 mm about 5.5 kg/m, transom 50 x 100 about 4.5 kg/m, spandrel as enamelled 6 mm glass with 100 mm mineral wool and 1.5 mm steel back pan. Typical values consulted."),
    ref("en-iso-6946", "EN ISO 6946:2017 Building components, thermal resistance and transmittance (R_si 0.13, R_se 0.04 m2K/W)."),
    ref("en-1998-1", "EN 1998-1:2004 Eurocode 8, part 1, section 4.3.5: non-structural elements, horizontal seismic force F_a = S_a W_a gamma_a / q_a; 0.12 W_a is an authored estimate for a low-seismicity site, not a code evaluation."),
    ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts."),
    ref("din-1356-1", "DIN 1356-1:1995 Building drawings, facade elevations, scales and symbols."),
]
mats = [
    mat("aluminium", "Thermally broken aluminium profile, powder coated grey", "Thermisch getrenntes Aluminiumprofil, grau pulverbeschichtet", 0.85, 0.55, 2800, 160, 880, 70e9, 150e6, "literature", "en-iso-10077-2"),
    mat("glass", "Float glass 6 mm (low-e inside)", "Floatglas 6 mm (Low-E innen)", 0.837, 0.1, 2500, 1.0, 750, 70e9, None, "literature", "en-572"),
    mat("spandrel-glass", "Enamelled spandrel glass 6 mm, grey", "Emailliertes Brüstungsglas 6 mm, grau", 0.84, 0.8, 2500, 1.0, 750, 70e9, None, "literature", "en-572"),
    mat("wool", "Mineral wool 035", "Mineralwolle 035", 0.9, 0.5, 60, 0.035, 1030, None, None, "literature", "en-iso-10077-1"),
    mat("steel", "Galvanised steel back pan 1.5 mm", "Verzinkte Stahlrückwand 1,5 mm", 0.3, 0.55, 7850, 50, 450, 210e9, 235e6, "literature", "en-iso-6946"),
]
def L(id, en, de, kind, m, d, R, rho, c, ref_):
    return layer(id, en, de, kind, m, d, R, "literature", ref_, c=d * rho * c)
regions = [
    region("glass-centre", "IGU centre of glass", "Isolierglas, Scheibenmitte", "glazing", "conditioned", "glass", A_CENTRE, 2.2, "vertical-plate", 0.5,
           [L("pane-out", "Outer pane 6 mm", "Außenscheibe 6 mm", "glass", "glass", 0.006, 0.006, 2500, 750, "en-iso-10077-1"),
            L("cavity", "Argon cavity 16 mm", "Argonzwischenraum 16 mm", "air-gap", "glass", 0.016, 0.745, 1.7, 520, "en-iso-10077-1"),
            L("pane-in", "Inner pane 6 mm, low-e", "Innenscheibe 6 mm, Low-E", "glass", "glass", 0.006, 0.006, 2500, 750, "en-iso-10077-1")], 22500, "literature", "en-iso-10077-1", conditioned=(20, 0.13)),
    region("glass-edge", "IGU edge zone with warm-edge spacer", "Isolierglas, Randzone mit Warme-Kante-Abstandhalter", "glazing", "conditioned", "glass", A_EDGE, 0.065, "vertical-plate", 0.5,
           [L("pane-out", "Outer pane 6 mm", "Außenscheibe 6 mm", "glass", "glass", 0.006, 0.006, 2500, 750, "en-iso-10077-1"),
            L("spacer-cavity", "Spacer-bridged cavity (psi_g 0.04 W/mK equivalent)", "Abstandhalter-überbrückter Zwischenraum (psi_g 0,04 W/mK äquivalent)", "air-gap", "glass", 0.016, 0.399, 1.7, 520, "en-iso-10077-1"),
            L("pane-in", "Inner pane 6 mm", "Innenscheibe 6 mm", "glass", "glass", 0.006, 0.006, 2500, 750, "en-iso-10077-1")], 22500, "authored-estimate", "en-iso-10077-1", conditioned=(20, 0.13)),
    region("spandrel", "Insulated spandrel panel", "Gedämmtes Brüstungspaneel", "wall", "conditioned", "spandrel-glass", A_SPAN, 0.8, "vertical-plate", 0.5,
           [L("steel-pan", "Steel back pan 1.5 mm", "Stahlrückwand 1,5 mm", "metal", "steel", 0.0015, 0.00003, 7850, 450, "en-iso-6946"),
            L("wool", "Mineral wool 100 mm", "Mineralwolle 100 mm", "insulation", "wool", 0.1, 2.857, 60, 1030, "en-iso-12631"),
            L("air", "Air layer 20 mm", "Luftschicht 20 mm", "air-gap", "wool", 0.02, 0.17, 1.2, 1000, "en-iso-6946"),
            L("enamel", "Enamelled glass 6 mm", "Emailliertes Glas 6 mm", "glass", "spandrel-glass", 0.006, 0.006, 2500, 750, "en-iso-12631")], 15000 + 3800, "literature", "en-iso-12631", conditioned=(20, 0.13)),
    region("mullion", "Mullions (thermally broken aluminium)", "Pfosten (thermisch getrenntes Aluminium)", "wall", "conditioned", "aluminium", A_MULL, 0.05, "vertical-plate", 0.5,
           [L("cap", "Pressure plate and cap 2 mm", "Pressleiste und Deckschale 2 mm", "metal", "aluminium", 0.002, 0.00001, 2800, 880, "en-iso-10077-2"),
            L("break", "Polyamide break with cavities (U_f 2.0 W/m2K)", "Polyamidsteg mit Kammern (U_f 2,0 W/m2K)", "polymer", "aluminium", 0.03, 0.33, 1000, 1500, "en-iso-10077-2")], 4900, "literature", "en-iso-10077-2", conditioned=(20, 0.13)),
    region("transom", "Transoms (thermally broken aluminium)", "Riegel (thermisch getrenntes Aluminium)", "wall", "conditioned", "aluminium", A_TRAN, 0.05, "vertical-plate", 0.5,
           [L("cap", "Pressure plate and cap 2 mm", "Pressleiste und Deckschale 2 mm", "metal", "aluminium", 0.002, 0.00001, 2800, 880, "en-iso-10077-2"),
            L("break", "Polyamide break with cavities (U_f 2.2 W/m2K)", "Polyamidsteg mit Kammern (U_f 2,2 W/m2K)", "polymer", "aluminium", 0.03, 0.284, 1000, 1500, "en-iso-10077-2")], 4900, "literature", "en-iso-10077-2", conditioned=(20, 0.13)),
]
U = lambda r: 1 / (0.13 + sum(l["resistanceM2KW"] for l in r["layers"]) + 0.04)
anchors = [anchor("head-anchor", "frame", BW / 2, BH, "support", "Slab anchor at the head, pin (hangs the bay)", "Deckenanker am Kopf, Festlager (hängt das Feld auf)", support="pin"),
           anchor("base-guide", "frame", BW / 2, 0, "support", "Base guide, horizontal roller", "Fußführung, horizontales Rollenlager", support="roller", normal=0),
           anchor("cradle-hook", "cradle", 0, 0, "load", "Hook of the maintenance cradle", "Haken der Wartungsgondel"),
           anchor("igu-centre", "igu", (GX1 - GX0) / 2, (VIS[1] - VIS[0]) / 2, "load", "Centre of the glazing unit", "Mitte der Glaseinheit"),
           anchor("spandrel-centre", "spandrel", (GX1 - GX0) / 2, (SP_TOP - 0.05) / 2, "load", "Centre of the spandrel", "Mitte der Brüstung")]
def regs(g=1.0, sp=1.0, fr=1.0):
    return {"glass-centre": RB(1.0, g), "glass-edge": RB(1.0, g), "spandrel": RB(1.0, sp), "mullion": RB(1.0, fr), "transom": RB(1.0, fr)}
cradle = load("cradle", "equipment", "Maintenance cradle hanging from the mid transom, 0.8 kN", "Wartungsgondel am mittleren Riegel, 0,8 kN", 0, -800, "authored-estimate", anchor_="cradle-hook")
seis = [load("seismic-igu", "equipment", "In-plane seismic force on the glass, 0.12 W", "Seismische Kraft in der Ebene auf das Glas, 0,12 W", 0.12 * MASS["igu"] * 9.81, 0, "authored-estimate", "en-1998-1", anchor_="igu-centre"),
        load("seismic-spandrel", "equipment", "In-plane seismic force on the spandrel, 0.12 W", "Seismische Kraft in der Ebene auf die Brüstung, 0,12 W", 0.12 * MASS["spandrel"] * 9.81, 0, "authored-estimate", "en-1998-1", anchor_="spandrel-centre")]
poses = [
    pose("dead-load", "Dead load, full sun", "Eigengewicht, volle Sonne", "The bay hangs from the slab anchor under its own weight, sunlit.", "Das Feld hängt unter Eigengewicht am Deckenanker, besonnt.", {}, [], regs(1.0, 1.0, 1.0)),
    pose("cradle-at-mid", "Cleaning cradle on the mid transom", "Reinigungsgondel am mittleren Riegel", "A 0.8 kN maintenance cradle hangs from the mid transom at the bay centre.", "Eine 0,8-kN-Wartungsgondel hängt in der Feldmitte am mittleren Riegel.", {"cradle": {"dx": (GX1 - GX0) / 2}}, [cradle], regs(1.0, 1.0, 1.0)),
    pose("seismic-in-plane", "In-plane seismic action", "Seismische Einwirkung in der Ebene", "A horizontal in-plane force of 0.12 times the weight on glass and spandrel.", "Eine waagerechte Kraft in der Ebene von 0,12 mal Gewicht auf Glas und Brüstung.", {}, seis, regs(1.0, 1.0, 1.0)),
    pose("shaded-by-overhang", "Upper part in the overhang shadow", "Oberer Teil im Dachüberstandsschatten", "A roof overhang shades the upper 60 % of the glass and the frame; the spandrel stays sunlit.", "Ein Dachüberstand verschattet die oberen 60 % des Glases und den Rahmen; die Brüstung bleibt besonnt.", {}, [], regs(0.4, 1.0, 0.6)),
]
shade = lambda rid, a, b: track(f"region:{rid}:sunlitFraction", [(0, a), (6, b), (12, a)])
clips = [
    clip("cradle-pass", "Cradle moves along the transom", "Gondel fährt am Riegel entlang", "The 0.8 kN cradle hook moves along the mid transom from the left to the right mullion.", "Der 0,8-kN-Gondelhaken fährt am mittleren Riegel vom linken zum rechten Pfosten.",
         8, False, [key(0, "cradle-at-mid", bones={"cradle": {"dx": 0.0}}), key(8, "cradle-at-mid", bones={"cradle": {"dx": GX1 - GX0}})],
         [phase("pass", 0, 8, "Pass", "Überfahrt")], [event(4, "motion", "Cradle at the bay centre", "Gondel in Feldmitte")]),
    clip("shadow-rise", "Building shadow climbs the bay", "Gebäudeschatten steigt am Feld hoch", "The shadow of a neighbouring building rises over the bay: spandrel, glass and frame sunlit fractions fall and recover.", "Der Schatten eines Nachbargebäudes steigt über das Feld: die Besonnungsanteile von Brüstung, Glas und Rahmen fallen und erholen sich.",
         12, True, [key(0, "dead-load"), key(12, "dead-load")], [phase("shadow", 0, 12, "Shadow rises", "Schatten steigt")], [event(6, "thermal", "Bay fully shaded", "Feld ganz im Schatten")],
         [shade("spandrel", 1.0, 0.0), shade("glass-centre", 1.0, 0.05), shade("glass-edge", 1.0, 0.05), shade("mullion", 1.0, 0.05), shade("transom", 1.0, 0.05)]),
    clip("seismic-pulse", "Seismic pulse", "Seismischer Impuls", "The in-plane seismic force rises to its design value and falls again.", "Die seismische Kraft in der Ebene steigt auf ihren Bemessungswert und fällt wieder.",
         6, False, [key(0, "seismic-in-plane"), key(6, "seismic-in-plane")], [phase("pulse", 0, 6, "Pulse", "Impuls")], [event(3, "load-on", "Peak", "Spitze")],
         [track("load:seismic-igu", [(0, 0), (3, 1), (6, 0)]), track("load:seismic-spandrel", [(0, 0), (3, 1), (6, 0)])]),
]
a = {
    "id": "curtain-wall-bay", "category": "components",
    "title": T("Curtain Wall Bay", "Pfosten-Riegel-Fassadenfeld"),
    "summary": T(f"One 1.35 m wide, 3.30 m high mullion-transom bay of a curtain wall in elevation: 50 mm thermally broken aluminium mullions and transoms (U_f 2.0 and 2.2), a 6-16-6 argon insulating glass unit (U_g about {U(regions[0]):.2f}) with warm-edge zone and an insulated spandrel panel (U about {U(regions[2]):.2f}); every element is a conditioned layered region, sun and shade follow the pose sunlit fractions.",
                 f"Ein 1,35 m breites, 3,30 m hohes Pfosten-Riegel-Feld einer Vorhangfassade in der Ansicht: 50 mm breite thermisch getrennte Aluminiumpfosten und -riegel (U_f 2,0 und 2,2), eine Argon-Isolierverglasung 6-16-6 (U_g etwa {U(regions[0]):.2f}) mit Warme-Kante-Randzone und ein gedämmtes Brüstungspaneel (U etwa {U(regions[2]):.2f}); jedes Element ist eine konditionierte, geschichtete Region, Sonne und Schatten folgen den Besonnungsanteilen der Posen."),
    "tags": ["component", "facade", "curtain-wall", "mullion", "glazing", "spandrel", "conditioned"],
    "provenance": {"author": "Semio staffage component authors (NW-C)", "method": "handcrafted", "created": "2026-10-29", "references": REFS,
                   "notes": T("Bay width 1.35 m centre to centre, extents 1.40 m including both mullion faces; storey height 3.30 m. Layer resistances follow EN ISO 12631 component method.", "Feldbreite 1,35 m von Mitte zu Mitte, Gesamtbreite 1,40 m einschließlich beider Pfostenansichten; Geschosshöhe 3,30 m. Die Schichtwiderstände folgen dem Komponentenverfahren nach EN ISO 12631.")},
    "materials": mats,
    "rig": {"kind": "structural", "root": "frame", "bones": bones},
    "anchors": anchors, "representations": reps, "symbols": [],
    "poses": poses, "clips": clips,
    "thermal": {"environment": env(0, 0, -12, 2, 0.8, 3, 300, 0.25), "operating": [], "regions": regions, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2 normal irradiance; interior 20 degC with R_si 0.13 m2K/W.", "Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m2 Normalstrahlung; Innenraum 20 °C mit R_si 0,13 m2K/W."),
                    T(f"Centre-of-glass U {U(regions[0]):.2f}, edge zone {U(regions[1]):.2f}, spandrel {U(regions[2]):.2f}, mullion {U(regions[3]):.2f}, transom {U(regions[4]):.2f} W/m2K; frame regions use the visible face area.", f"U Scheibenmitte {U(regions[0]):.2f}, Randzone {U(regions[1]):.2f}, Brüstung {U(regions[2]):.2f}, Pfosten {U(regions[3]):.2f}, Riegel {U(regions[4]):.2f} W/m2K; die Rahmenregionen nutzen die sichtbare Ansichtsfläche."),
                    T("Sun and shade come from the pose sunlit fractions (overhang or neighbouring building); the argon cavity and the spacer are lumped into the layer resistance.", "Sonne und Schatten kommen aus den Besonnungsanteilen der Posen (Dachüberstand oder Nachbargebäude); Argonraum und Abstandhalter sind im Schichtwiderstand zusammengefasst.")]},
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": ["head-anchor", "base-guide"],
                  "assumptions": [T("The bay hangs from a pin at the slab edge and is held horizontally at the foot; frame, glass and spandrel are rigid parts. Only in-plane actions can be represented; wind acts out of the plane and is not modelled.", "Das Feld hängt an einem Festlager am Deckenrand und ist am Fuß horizontal gehalten; Rahmen, Glas und Brüstung sind starre Teile. Nur Einwirkungen in der Ebene sind abbildbar; Wind wirkt senkrecht zur Ebene und ist nicht modelliert."),
                                T("The seismic force 0.12 W is an authored estimate guided by EN 1998-1 for non-structural elements, not a code evaluation.", "Die seismische Kraft 0,12 W ist ein Schätzwert nach dem Vorbild von EN 1998-1 für nichttragende Bauteile, keine normgemäße Auswertung.")], "clinical": False},
    "presentation": {"defaultPose": "dead-load", "defaultClip": "shadow-rise", "defaultScale": 50, "viewpoints": ["elevation"]},
    "exports": ALL_EXPORTS, "completion": "authored", "_dimsFrom": "50", "_depth": 0.15,
}
assemble(a, "curtain-wall-bay")
