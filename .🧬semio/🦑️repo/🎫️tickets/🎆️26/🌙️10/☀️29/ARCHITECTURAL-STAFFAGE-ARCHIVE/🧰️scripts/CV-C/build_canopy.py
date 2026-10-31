import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
from lib import *

AID = "tied-canopy"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
a["dimensions"].update({"heightM": 3.14, "widthM": 6.0, "depthM": 1.0, "massKg": 86.78})
a["summary"] = T(
    "A 5.5 m IPE 160 steel canopy beam (S355) with a glass roof, pinned to an externally insulated concrete wall through a thermally separated plate and held by a tube strut hinged at both ends rising to the wall above; it carries snow in tension and wind uplift in compression.",
    "Ein 5,5 m langer Stahlträger IPE 160 (S355) eines Vordachs mit Glasdeckung, über eine thermisch getrennte Platte gelenkig an einer außengedämmten Betonwand gelagert und durch einen beidseitig gelenkig angeschlossenen Rohrstab zur Wand darüber gehalten; er trägt Schnee auf Zug und Windsog auf Druck.")
a["provenance"]["notes"] = T(
    "The tie is a tube strut hinged at both ends: a bilateral roller along the rod, so uplift is carried in compression. Beam spacing 1.0 m sets the line loads. The wall stub shows the three layers of an externally insulated wall (concrete 300 mm, mineral wool 180 mm, render 20 mm) with two steel anchor plates that are the canopy's thermal bridges. The glass cover is drawn but its weight is not part of the rig mass.",
    "Der Zugstab ist ein beidseitig gelenkig angeschlossenes Rohr: eine beidseitige Rolle entlang des Stabs, sodass Windsog auf Druck abgetragen wird. Trägerabstand 1,0 m legt die Linienlasten fest. Das Wandstück zeigt die drei Schichten einer außengedämmten Wand (Beton 300 mm, Mineralwolle 180 mm, Putz 20 mm) mit zwei Stahl-Ankerplatten, die die Wärmebrücken des Vordachs sind. Die Glasdeckung ist gezeichnet, ihr Gewicht gehört nicht zur Rigmasse.")
a["structure"]["assumptions"].append(T(
    "The glass cover (about 0.4 kN/m2) is drawn but not part of the beam mass; the snow pose carries only snow.",
    "Die Glasdeckung (etwa 0,4 kN/m²) ist gezeichnet, aber nicht Teil der Trägermasse; die Schneepose trägt nur Schnee."))
set_material(a, "steel", conductivityWmK=50, specificHeatJKgK=450)
set_material(a, "concrete", conductivityWmK=2.3, specificHeatJKgK=1000)
set_material(a, "glass", title=T("Laminated glass 2 x 8 mm", "Verbundglas 2 x 8 mm"), emissivity=0.837, solarAbsorptivity=0.08, densityKgM3=2500, conductivityWmK=1.0,
             specificHeatJKgK=750, elasticModulusPa=70000000000, source="literature", reference="en-12898")
set_material(a, "mineral-wool", title=T("Mineral wool, ETICS insulation", "Mineralwolle, WDVS-Dämmung"), emissivity=0.9, solarAbsorptivity=0.5, densityKgM3=100,
             conductivityWmK=0.035, specificHeatJKgK=840, source="literature", reference="en-iso-10456")
set_material(a, "render", title=T("Mineral render, light", "Mineralputz, hell"), emissivity=0.9, solarAbsorptivity=0.4, densityKgM3=1800, conductivityWmK=0.8,
             specificHeatJKgK=1000, source="literature", reference="en-iso-10456")
set_material(a, "anchor-pad", title=T("Anchor plate with thermal separation pad (equivalent)", "Ankerplatte mit thermischer Trennung (äquivalent)"), emissivity=0.85, solarAbsorptivity=0.5,
             conductivityWmK=0.57, source="authored-estimate")
CF = {"interiorTempC": 20, "interiorFilmM2KW": 0.13}
a["thermal"] = {
    "environment": {"airTempC": 0, "meanRadiantTempC": 0, "skyTempC": -12, "groundTempC": 2, "relativeHumidity": 0.8, "windMS": 3, "windDirectionDeg": 0,
                    "solarWM2": 300, "diffuseFraction": 0.25, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("canopy-steel", "IPE 160 canopy beam steel", "Vordachträger IPE 160 Stahl", "surface", "passive", "steel", 3.5, 0.16, "vertical-plate", 0.5, [], 25000, "authored-estimate", "en-iso-10456"),
        region("canopy-glass", "Laminated glass roof, top face", "Verbundglasdach, Oberseite", "glazing", "passive", "glass", 5.3, 0.3, "horizontal-plate-up", 1.0, [], 24000, "authored-estimate", "en-iso-10456"),
        region("rod-steel", "Tube strut surface", "Rohrstab Oberfläche", "surface", "passive", "steel", 0.93, 0.0603, "cylinder", 0.7, [], 12000, "authored-estimate", "en-iso-10456"),
        region("wall-face", "Insulated wall (outer face)", "Gedämmte Wand (Außenseite)", "wall", "conditioned", "render", 2.66, 2.66, "vertical-plate", 0.5, [
            layer("wall-concrete", "Concrete core 300 mm", "Betonkern 300 mm", "masonry", "concrete", 0.30, 0.1304, "literature", "en-iso-10456", 690000),
            layer("wall-wool", "Mineral wool 180 mm", "Mineralwolle 180 mm", "insulation", "mineral-wool", 0.18, 5.143, "literature", "en-iso-10456", 15100),
            layer("wall-render", "Render 20 mm", "Putz 20 mm", "shell", "render", 0.02, 0.025, "literature", "en-iso-10456", 36000)],
               40000, "literature", "en-iso-6946", CF),
        region("wall-plate", "Steel anchor plates of the beam and the strut (thermal bridges)", "Stahlankerplatten von Träger und Stab (Wärmebrücken)", "surface", "conditioned", "anchor-pad", 0.48, 0.24, "vertical-plate", 0.5, [
            layer("plate-concrete", "Concrete core 300 mm", "Betonkern 300 mm", "masonry", "concrete", 0.30, 0.1304, "literature", "en-iso-10456", 690000),
            layer("plate-pad", "Anchor shaft and separation pad 180 mm, equivalent resistance 0.35 m2K/W", "Ankerschaft und Trennlage 180 mm, äquivalenter Widerstand 0,35 m²K/W", "insulation", "anchor-pad", 0.18, 0.35, "authored-estimate", "en-iso-10211", 15000),
            layer("plate-steel", "Steel plate 20 mm", "Stahlplatte 20 mm", "metal", "steel", 0.02, 0.0004, "literature", "en-iso-10456", 70000)],
               70000, "authored-estimate", "en-iso-10211", CF),
    ],
    "emitters": [],
    "assumptions": [
        T("Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2 normal irradiance; the wall interior is at 20 degC with R_si 0.13 m2K/W. The viewer may select any preset.",
          "Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m² Normalstrahlung; die Wand hat innen 20 °C mit R_si 0,13 m²K/W. Der Betrachter kann jede Voreinstellung wählen."),
        T("The wall is a conditioned element with U about 0.17 W/(m2 K) (concrete 300, wool 180, render 20 mm); the anchor plates are a thermal bridge approximated by a separate region with an equivalent separation-pad resistance of 0.35 m2K/W, giving U about 1.4 W/(m2 K) (authored estimate; EN ISO 10211 would resolve the bridge).",
          "Die Wand ist ein konditioniertes Bauteil mit U etwa 0,17 W/(m² K) (Beton 300, Wolle 180, Putz 20 mm); die Ankerplatten sind eine durch eine eigene Region angenäherte Wärmebrücke mit einem äquivalenten Trennlagenwiderstand von 0,35 m²K/W, U etwa 1,4 W/(m² K) (Schätzung; EN ISO 10211 würde die Brücke auflösen)."),
        T("The steel beam, strut and the glass roof are passive exterior parts; the glass reacts with its 16 mm of glass (heat capacity 24 kJ/(m2 K)).",
          "Stahlträger, Stab und Glasdach sind passive Außenbauteile; das Glas reagiert mit seinen 16 mm Glas (Wärmekapazität 24 kJ/(m² K))."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: the glass roof fully sunlit, the beam seen under the glass 60 %, the strut 70 %, the wall half shaded by the canopy, the anchor plates half.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: das Glasdach voll besonnt, der Träger unter dem Glas 60 %, der Stab 70 %, die Wand durch das Vordach zur Hälfte verschattet, die Ankerplatten zur Hälfte."),
    ],
}
bind_poses(a, {"canopy-steel": (1, 0.6), "canopy-glass": (1, 1.0), "rod-steel": (1, 0.7), "wall-face": (1, 0.5), "wall-plate": (1, 0.5)})

D = Draw(a["rig"]["bones"])
C = "canopy"
WALL = ["wall-face", "wall-plate"]
BEAM = ["canopy-steel", "canopy-glass"]
SX, SY = 4.0, 0.12
WY = 3.0
glass_x = [(0.1 + i * 1.34, 0.1 + i * 1.34 + 1.28) for i in range(4)]

r500 = rep(500, [
    D.rect("wall", C, "body", 1, True, 0.6, -0.5, 0, 0, 3.14, regions=WALL, material="concrete"),
    D.rect("beam", C, "body", 3, True, 0.4, 0, 0.04, 5.5, 0.22, regions=BEAM, material="steel"),
    D.line("strut", C, "body", 5, 0.45, [(SX, SY), (0, WY)], region="rod-steel", material="steel", penMm=0.13)],
    dec(("A tall wall bar on the left, a horizontal canopy bar projecting 5.5 m from its foot and a single strut line rising from the beam to the wall; a classic tied canopy diagram.",
         "Ein hoher Wandbalken links, ein waagerechter Vordachbalken, der 5,5 m aus seinem Fuß auskragt, und eine einzelne Stablinie, die vom Träger zur Wand ansteigt; das klassische Bild eines abgehängten Vordachs."),
        ("Wall (1 x 6.3 mm), beam with glass cover (11 x 0.36 mm) and the strut angle (36 degrees to the beam).", "Wand (1 x 6,3 mm), Träger mit Glasdeckung (11 x 0,36 mm) und der Stabwinkel (36 Grad zum Träger)."),
        ("Wall layers and anchor plates are one wall bar; steel beam and glass roof are one beam bar.", "Wandschichten und Ankerplatten sind ein Wandbalken; Stahlträger und Glasdach sind ein Trägerbalken."),
        ("Flange lines, glass joints, clevises, pins and plate details.", "Flanschlinien, Glasfugen, Gabelköpfe, Bolzen und Plattendetails."),
        ("The 60 mm tube strut is a 0.13 mm pen line (65 mm real); the 0.36 mm beam bar is drawn at its true height of 0.18 m.", "Der 60-mm-Rohrstab ist eine Linie mit 0,13 mm Feder (65 mm real); der 0,36 mm hohe Trägerbalken ist in seiner wahren Höhe von 0,18 m gezeichnet."),
        ("No hatch.", "Keine Schraffur."),
        ("Mid-grey wall, dark beam, thin strut line.", "Mittelgraue Wand, dunkler Träger, dünne Stablinie."),
        ("Overlays colour the wall (merged layers and plates), the beam (merged steel and glass) and the strut separately; the strut line carries the tension or compression.",
         "Überlagerungen färben Wand (Schichten und Platten zusammengefasst), Träger (Stahl und Glas zusammengefasst) und Stab getrennt; die Stablinie trägt Zug oder Druck.")))

r200 = rep(200, [
    D.rect("wall-core", C, "body", 1, True, 0.6, -0.5, 0, -0.2, 3.14, region="wall-face", material="concrete"),
    D.rect("wall-outer", C, "body", 1, True, 0.9, -0.2, 0, 0, 3.14, regions=WALL, material="mineral-wool"),
    D.rect("beam", C, "body", 3, True, 0.4, 0, 0.04, 5.5, 0.2, region="canopy-steel", material="steel"),
    D.line("glass-roof", C, "glazing", 3.5, 0.9, [(0.1, 0.2), (5.4, 0.2)], region="canopy-glass", material="glass", penMm=0.1),
    D.line("strut", C, "body", 5, 0.45, [(SX, SY), (0, WY)], region="rod-steel", material="steel", penMm=0.3)],
    dec(("A two-layer wall bar, a beam band with a glass line on top and a thick strut line to the wall.",
         "Ein zweischichtiger Wandbalken, ein Trägerband mit einer Glaslinie darauf und eine kräftige Stablinie zur Wand."),
        ("Concrete core (1.5 mm), insulation and render with the anchor plates (1 mm), beam band (0.8 mm tall), glass line, strut line.",
         "Betonkern (1,5 mm), Dämmung und Putz mit den Ankerplatten (1 mm), Trägerband (0,8 mm hoch), Glaslinie, Stablinie."),
        ("Mineral wool, render and anchor plates share the outer wall shape; the glass panels are one line.", "Mineralwolle, Putz und Ankerplatten teilen sich die äußere Wandform; die Glasscheiben sind eine Linie."),
        ("Flange lines, glass joints, clevises, pins and the anchor plate outlines.", "Flanschlinien, Glasfugen, Gabelköpfe, Bolzen und die Ankerplattenumrisse."),
        ("The 20 mm glass is a 0.1 mm pen line (20 mm exact); the 60 mm strut is a 0.3 mm pen line (60 mm exact).", "Das 20-mm-Glas ist eine Linie mit 0,1 mm Feder (20 mm genau); der 60-mm-Stab ist eine Linie mit 0,3 mm Feder (60 mm genau)."),
        ("No hatch: tones separate concrete from insulation.", "Keine Schraffur: Tonwerte trennen Beton von der Dämmung."),
        ("Mid-grey concrete, light insulation, dark steel.", "Mittelgrauer Beton, helle Dämmung, dunkler Stahl."),
        ("Wall core, outer wall with plates, beam, glass line and strut are separate overlay cells; the outer wall is a merged layer and plate cell.",
         "Wandkern, Außenwand mit Platten, Träger, Glaslinie und Stab sind getrennte Überlagerungszellen; die Außenwand ist eine zusammengefasste Schicht-und-Platten-Zelle.")))

hc = [H("concrete", "Reinforced concrete", "Stahlbeton", "stipple", 0, 0.8, 0.1)]
ins = zigzag(-0.11, 0.3, -0.11, 2.82, 26, 0.05, axis="y")
strut_w = 0.03
import math
ang = math.atan2(WY - SY, 0 - SX)
nx, ny = -math.sin(ang) * strut_w, math.cos(ang) * strut_w
strut_poly = [(SX + nx, SY + ny), (0 + nx, WY + ny), (0 - nx, WY - ny), (SX - nx, SY - ny)]
r100 = rep(100, [
    D.rect("wall-core", C, "body", 1, True, 0.6, -0.5, 0, -0.2, 3.14, region="wall-face", material="concrete", hatch="concrete"),
    D.rect("wall-insulation", C, "body", 1, True, 0.9, -0.2, 0, 0, 3.14, region="wall-face", material="mineral-wool"),
    D.line("wall-insulation-zigzag", C, "texture", 1.2, 0.3, ins, region="wall-face"),
    D.rect("plate-beam", C, "body", 1.5, True, 0.3, -0.2, 0, 0, 0.24, region="wall-plate", material="anchor-pad"),
    D.rect("plate-strut", C, "body", 1.5, True, 0.3, -0.2, 2.88, 0, 3.12, region="wall-plate", material="anchor-pad"),
    D.rect("beam", C, "body", 3, True, 0.4, 0, 0.04, 5.5, 0.2, region="canopy-steel", material="steel"),
] + [D.rect(f"glass-{i}", C, "glazing", 3.5, True, 0.92, x0, 0.2, x1, 0.22, region="canopy-glass", material="glass", penMm=0.1) for i, (x0, x1) in enumerate(glass_x)] + [
    D.poly("strut", C, "body", 5, True, 0.45, strut_poly, region="rod-steel", material="steel"),
    D.circ("pin-beam", C, "detail", 5.3, True, 0.1, SX, SY, 0.015, region="rod-steel"),
    D.circ("pin-wall", C, "detail", 5.3, True, 0.1, 0.0, WY, 0.015, region="rod-steel")],
    dec(("A hatched concrete wall with insulation, two anchor plates, a steel beam with four glass panels and a strut with two pins.",
         "Eine schraffierte Betonwand mit Dämmung, zwei Ankerplatten, ein Stahlträger mit vier Glasscheiben und ein Stab mit zwei Bolzen."),
        ("Wall core (3 mm, stipple), insulation with zigzag (1.8 mm), two anchor plates (2 x 2.4 mm), beam (1.6 mm tall), four glass panels (12.8 mm each, 0.6 mm joints), strut (0.6 mm wide), two pin dots.",
         "Wandkern (3 mm, Punktierung), Dämmung mit Zickzack (1,8 mm), zwei Ankerplatten (2 x 2,4 mm), Träger (1,6 mm hoch), vier Glasscheiben (je 12,8 mm, Fugen 0,6 mm), Stab (0,6 mm breit), zwei Bolzenpunkte."),
        ("The 20 mm render is merged into the insulation shape.", "Der 20-mm-Putz ist in der Dämmungsform zusammengefasst."),
        ("Flange lines, clevises and the plate-to-beam welds.", "Flanschlinien, Gabelköpfe und die Plattenschweißnähte zum Träger."),
        ("The 20 mm glass panels are 0.2 mm filled strips with 0.1 mm outlines; the strut is drawn at its true 0.6 mm width.", "Die 20-mm-Glasscheiben sind 0,2 mm hohe Streifen mit 0,1 mm Kontur; der Stab hat seine wahre Breite von 0,6 mm."),
        ("Concrete stippled (0.8 mm); insulation with the DIN ISO 128-50 zigzag; steel stays solid because the elevation is not a cut.", "Beton punktiert (0,8 mm); Dämmung mit Zickzack nach DIN ISO 128-50; Stahl bleibt voll, weil die Ansicht kein Schnitt ist."),
        ("Outline 0.25 mm, glass 0.1 mm; dark steel, light glass, mid-grey concrete.", "Kontur 0,25 mm, Glas 0,1 mm; dunkler Stahl, helles Glas, mittelgrauer Beton."),
        ("Wall layers, the two anchor plates, beam, each glass panel and the strut are separate overlay cells.", "Wandschichten, die beiden Ankerplatten, Träger, jede Glasscheibe und der Stab sind getrennte Überlagerungszellen.")), hatches=hc)

hc50 = [H("concrete", "Reinforced concrete", "Stahlbeton", "stipple", 0, 1.0, 0.1)]
ins50 = zigzag(-0.11, 0.3, -0.11, 2.82, 33, 0.04, axis="y")
s50 = [
    D.rect("wall-core", C, "body", 1, True, 0.6, -0.5, 0, -0.2, 3.14, region="wall-face", material="concrete", hatch="concrete"),
    D.line("wall-rebar-inner", C, "detail", 1.1, 0.2, [(-0.45, 0.3), (-0.45, 3.0)], region="wall-face", penMm=0.2),
    D.line("wall-rebar-outer", C, "detail", 1.1, 0.2, [(-0.25, 0.3), (-0.25, 3.0)], region="wall-face", penMm=0.2),
    D.rect("wall-insulation", C, "body", 1, True, 0.9, -0.2, 0, -0.02, 3.14, region="wall-face", material="mineral-wool"),
    D.line("wall-insulation-zigzag", C, "texture", 1.2, 0.3, ins50, region="wall-face"),
    D.rect("wall-render", C, "body", 1.05, True, 0.55, -0.02, 0, 0, 3.14, region="wall-face", material="render"),
    D.rect("plate-beam", C, "body", 1.5, True, 0.3, -0.02, 0, 0, 0.24, region="wall-plate", material="steel"),
    D.rect("pad-beam", C, "body", 1.4, True, 0.7, -0.2, 0, -0.02, 0.24, region="wall-plate", material="anchor-pad"),
    D.rect("plate-strut", C, "body", 1.5, True, 0.3, -0.02, 2.88, 0, 3.12, region="wall-plate", material="steel"),
    D.rect("pad-strut", C, "body", 1.4, True, 0.7, -0.2, 2.88, -0.02, 3.12, region="wall-plate", material="anchor-pad"),
    D.rect("beam", C, "body", 3, True, 0.4, 0, 0.04, 5.5, 0.2, region="canopy-steel", material="steel"),
    D.line("beam-flange-top", C, "detail", 3.1, 0.3, [(0.02, 0.1926), (5.5, 0.1926)], region="canopy-steel"),
    D.line("beam-flange-bottom", C, "detail", 3.1, 0.3, [(0.02, 0.0474), (5.5, 0.0474)], region="canopy-steel"),
]
s50 += [D.rect(f"glass-{i}", C, "glazing", 3.5, True, 0.92, x0, 0.2, x1, 0.22, region="canopy-glass", material="glass") for i, (x0, x1) in enumerate(glass_x)]
s50 += [
    D.capsule("strut-tube", C, "body", 5, True, 0.45, SX, SY, 0, WY, 0.03, region="rod-steel", material="steel"),
    D.line("strut-axis", C, "detail", 5.1, 0.3, [(SX - 0.12 * math.cos(ang) * -1 * 0 - 0.1, SY + 0.1 * math.tan(-ang) * 0 + 0.0), (0.1, WY - 0.0)] if False else
           [(SX + 0.1 * math.cos(ang), SY + 0.1 * math.sin(ang)), (0.1 * -math.cos(ang), WY - 0.1 * math.sin(ang))], region="rod-steel"),
    D.rect("clevis-beam", C, "body", 5.2, True, 0.3, SX - 0.06, SY - 0.12, SX + 0.06, SY + 0.12, region="rod-steel", material="steel", r=0.01),
    D.rect("clevis-wall", C, "body", 5.2, True, 0.3, -0.04, WY - 0.12, 0.04, WY + 0.12, region="rod-steel", material="steel", r=0.01),
    D.circ("pin-beam", C, "detail", 5.3, True, 0.1, SX, SY, 0.012, region="rod-steel"),
    D.circ("pin-wall", C, "detail", 5.3, True, 0.1, 0.0, WY, 0.012, region="rod-steel"),
]
r50 = rep(50, s50, dec(
    ("The full detail of the tied canopy: the insulated concrete wall with reinforcement, two anchor plates with separation pads, the IPE 160 beam with flange lines, a glass roof of four panels with joints, and the tube strut with clevises and pins at both ends.",
     "Das volle Detail des abgehängten Vordachs: die gedämmte Betonwand mit Bewehrung, zwei Ankerplatten mit Trennlagen, der Träger IPE 160 mit Flanschlinien, ein Glasdach aus vier Scheiben mit Fugen und der Rohrstab mit Gabelköpfen und Bolzen an beiden Enden."),
    ("Wall core (6 mm) with two bars, 180 mm insulation with zigzag, render, plates and separation pads (0.4 mm plates), beam (3.2 mm tall) with two flange lines (7.4 mm flanges), four glass panels (25.6 x 0.4 mm, joints 1.2 mm), strut, clevises (2.4 x 4.8 mm), pins.",
     "Wandkern (6 mm) mit zwei Stäben, 180 mm Dämmung mit Zickzack, Putz, Platten und Trennlagen (Platten 0,4 mm), Träger (3,2 mm hoch) mit zwei Flanschlinien (Flansche 7,4 mm), vier Glasscheiben (25,6 x 0,4 mm, Fugen 1,2 mm), Stab, Gabelköpfe (2,4 x 4,8 mm), Bolzen."),
    ("The strut tube is one capsule with an axis line; the render below the beam plate is drawn together with the wool.", "Das Stabrohr ist eine Kapsel mit Achslinie; der Putz unter der Trägerplatte ist mit der Wolle zusammen gezeichnet."),
    ("Weld seams, plate bolts, the web stiffener and the hollow-section wall of the strut.", "Schweißnähte, Plattenschrauben, die Stegsteife und die Wand des Rohrprofils."),
    ("The tube wall (3.2 mm, 0.06 mm) is not resolved; the 20 mm glass is a 0.4 mm filled strip with a 0.13 mm outline, the 20 mm render a 0.4 mm strip.",
     "Die Rohrwand (3,2 mm, 0,06 mm) wird nicht aufgelöst; das 20-mm-Glas ist ein 0,4 mm hoher gefüllter Streifen mit 0,13 mm Kontur, der 20-mm-Putz ein 0,4 mm breiter Streifen."),
    ("Concrete stippled (1.0 mm), bars as 0.2 mm lines, insulation with the zigzag line (DIN ISO 128-50); steel is solid because the elevation is not a cut.",
     "Beton punktiert (1,0 mm), Stäbe als 0,2-mm-Linien, Dämmung mit Zickzacklinie (DIN ISO 128-50); Stahl bleibt voll, weil die Ansicht kein Schnitt ist."),
    ("Outline 0.35 mm, details 0.18 mm, glass 0.13 mm; darkest steel, lighter pads, light glass.", "Kontur 0,35 mm, Details 0,18 mm, Glas 0,13 mm; Stahl am dunkelsten, hellere Trennlagen, helles Glas."),
    ("Wall layers, plates with pads, beam, each glass panel and the strut with its clevises are separate overlay cells, so the cold plate bridges read against the insulated wall.",
     "Wandschichten, Platten mit Trennlagen, Träger, jede Glasscheibe und der Stab mit Gabelköpfen sind getrennte Überlagerungszellen, so heben sich die kalten Plattenbrücken von der gedämmten Wand ab.")), hatches=hc50)
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["symbols"] = []
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
