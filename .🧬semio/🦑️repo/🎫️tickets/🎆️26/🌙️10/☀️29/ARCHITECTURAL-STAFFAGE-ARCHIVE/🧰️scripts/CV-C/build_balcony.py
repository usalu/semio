import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
from lib import *

AID = "cantilever-balcony"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])

# ---- rig: top-mounted post on the slab top at 1.45 m (base plate inside the slab edge), 1.1 m barrier height
rail = next(b for b in a["rig"]["bones"] if b["id"] == "railing")
rail.update({"x": 1.45, "y": 0.1, "lengthM": 1.1})
rail["mass"].update({"kg": 6.355, "comX": 0.55})
for an in a["anchors"]:
    if an["id"] == "railing-top":
        an["x"] = 1.1
a["dimensions"].update({"heightM": 1.3, "widthM": 1.8, "depthM": 1.0, "massKg": 756.36})
a["summary"] = T(
    "A 1.5 m reinforced concrete slab cantilevered from an externally insulated wall through a thermal-break element, with a top-mounted steel railing post, carrying a moving crowd point load; the wall connection carries the full bending moment and acts as the classic balcony thermal bridge.",
    "Eine 1,5 m auskragende Stahlbetonplatte an einer außengedämmten Wand mit Dämmelement (Kragplattenanschluss) und aufgesetztem Stahl-Geländerpfosten, die eine wandernde Einzellast trägt; der Wandanschluss nimmt das volle Biegemoment auf und ist die klassische Wärmebrücke des Balkons.")
a["provenance"]["notes"] = T(
    "Strip model: one metre of balcony width. The wall stub shows the three layers of an externally insulated wall (reinforced concrete 180 mm, mineral wool 100 mm, render 20 mm); the thermal-break module replaces the insulation over the slab depth. The crowd point load rides a massless load-carrier bone. The railing post stands on the slab top, 50 mm inside the edge.",
    "Streifenmodell: ein Meter Balkonbreite. Das Wandstück zeigt die drei Schichten einer außengedämmten Wand (Stahlbeton 180 mm, Mineralwolle 100 mm, Putz 20 mm); das Dämmelement ersetzt die Dämmung auf der Plattenhöhe. Die Einzellast läuft auf einem masselosen Lastträger-Knochen. Der Geländerpfosten steht 50 mm hinter der Kante auf der Plattenoberseite.")

# ---- materials with the properties the thermal model reads
set_material(a, "concrete", conductivityWmK=2.3, specificHeatJKgK=1000)
set_material(a, "steel", conductivityWmK=50, specificHeatJKgK=450)
set_material(a, "mineral-wool", title=T("Mineral wool, ETICS insulation", "Mineralwolle, WDVS-Dämmung"), emissivity=0.9, solarAbsorptivity=0.5,
             densityKgM3=100, conductivityWmK=0.035, specificHeatJKgK=840, source="literature", reference="en-iso-10456")
set_material(a, "render", title=T("Mineral render, light", "Mineralputz, hell"), emissivity=0.9, solarAbsorptivity=0.4,
             densityKgM3=1800, conductivityWmK=0.8, specificHeatJKgK=1000, source="literature", reference="en-iso-10456")
set_material(a, "break-module", title=T("Balcony thermal-break module (insulation body with stainless steel bars)", "Kragplatten-Dämmelement (Dämmkörper mit Edelstahlstäben)"),
             emissivity=0.9, solarAbsorptivity=0.5, conductivityWmK=0.124, source="authored-estimate")

# ---- thermal v2
cap_conc = 184000
env = {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0,
       "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325}
a["thermal"] = {
    "environment": env,
    "operating": [],
    "regions": [
        region("slab-top", "Slab top surface", "Plattenoberseite", "surface", "passive", "concrete", 1.5, 0.3, "horizontal-plate-up", 0.95, [], cap_conc, "authored-estimate", "en-iso-13786"),
        region("slab-soffit", "Slab soffit", "Plattenunterseite", "surface", "passive", "concrete", 1.5, 0.3, "horizontal-plate-down", 0.05, [], cap_conc, "authored-estimate", "en-iso-13786"),
        region("slab-edge", "Slab edge", "Plattenkante", "surface", "passive", "concrete", 0.2, 0.2, "vertical-plate", 0.5, [], cap_conc, "authored-estimate", "en-iso-13786"),
        region("railing-steel", "Railing post, handrail and mid-rail steel", "Geländerpfosten, Handlauf und Zwischenstab Stahl", "surface", "passive", "steel", 0.24, 0.05, "cylinder", 0.9, [], 14100, "authored-estimate", "en-iso-10456"),
        region("wall-face", "Insulated wall above the slab (outer face)", "Gedämmte Wand über der Platte (Außenseite)", "wall", "conditioned", "render", 1.1, 1.1, "vertical-plate", 0.5, [
            layer("wall-concrete", "Reinforced concrete core 180 mm", "Stahlbetonkern 180 mm", "masonry", "concrete", 0.18, 0.0783, "literature", "en-iso-10456", 414000),
            layer("wall-wool", "Mineral wool 100 mm", "Mineralwolle 100 mm", "insulation", "mineral-wool", 0.10, 2.857, "literature", "en-iso-10456", 8400),
            layer("wall-render", "Render 20 mm", "Putz 20 mm", "shell", "render", 0.02, 0.025, "literature", "en-iso-10456", 36000)],
               40000, "literature", "en-iso-6946", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
        region("thermal-break", "Thermal-break module in the insulation plane (thermal bridge)", "Dämmelement in der Dämmebene (Wärmebrücke)", "wall", "conditioned", "break-module", 0.2, 0.2, "vertical-plate", 0.3, [
            layer("break-concrete", "Reinforced concrete core 180 mm", "Stahlbetonkern 180 mm", "masonry", "concrete", 0.18, 0.0783, "literature", "en-iso-10456", 414000),
            layer("break-body", "Break module 120 mm, equivalent conductivity 0.124 W/mK incl. bars", "Dämmelement 120 mm, äquivalente Wärmeleitfähigkeit 0,124 W/mK inkl. Stäbe", "insulation", "break-module", 0.12, 0.968, "authored-estimate", None, 20000)],
               20000, "authored-estimate", "en-iso-10211", {"interiorTempC": 20, "interiorFilmM2KW": 0.13}),
    ],
    "emitters": [],
    "assumptions": [
        T("The wall-face region is a conditioned element: interior at 20 degC, U about 0.32 W/(m2 K) (R_si 0.13 + 0.078 + 2.857 + 0.025 + R_se 0.04 m2K/W per EN ISO 6946).",
          "Die Wandfläche ist ein konditioniertes Bauteil: Innenraum 20 °C, U etwa 0,32 W/(m² K) (R_si 0,13 + 0,078 + 2,857 + 0,025 + R_se 0,04 m²K/W nach EN ISO 6946)."),
        T("The balcony thermal bridge is approximated by one region: the break module has an equivalent conductivity of 0.124 W/(mK), which gives a linear transmittance of about 0.10 W/(mK), the order of magnitude of commercial break modules (authored estimate; EN ISO 10211 would resolve it in 2D).",
          "Die Wärmebrücke des Balkons ist durch eine Region angenähert: das Dämmelement hat eine äquivalente Wärmeleitfähigkeit von 0,124 W/(mK), das ergibt einen längenbezogenen Wärmedurchgangskoeffizienten von etwa 0,10 W/(mK), die Größenordnung handelsüblicher Dämmelemente (Schätzung; EN ISO 10211 löst sie zweidimensional auf)."),
        T("Passive concrete faces store heat in an effective depth of 80 mm (about half the 24 h penetration depth of 0.17 m, EN ISO 13786); the slab is not heated from inside.",
          "Passive Betonflächen speichern Wärme in einer wirksamen Tiefe von 80 mm (etwa die Hälfte der 24-h-Eindringtiefe von 0,17 m, EN ISO 13786); die Platte wird nicht von innen beheizt."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: top and railing in full sun, soffit and break module in permanent shade, wall in partial shade.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Oberseite und Geländer in voller Sonne, Unterseite und Dämmelement dauernd im Schatten, Wand teilweise verschattet."),
        T("Reference case: a mild sunny day (air 20 degC, sky 8 degC, ground 18 degC, 500 W/m2 normal irradiance, 20 % diffuse); all numbers are authored estimates.",
          "Referenzfall: ein milder sonniger Tag (Luft 20 °C, Himmel 8 °C, Boden 18 °C, 500 W/m² Normalstrahlung, 20 % diffus); alle Zahlen sind Schätzungen."),
    ],
}
bind_poses(a, {"slab-top": (1, 1.0), "slab-soffit": (1, 0.0), "slab-edge": (1, 0.6), "railing-steel": (1, 0.8), "wall-face": (1, 0.6), "thermal-break": (1, 0.0)})
a["clips"].append({
    "id": "sun-and-shade", "title": T("Sun and shade", "Sonne und Schatten"),
    "description": T("A cloud shadow crosses the balcony: top, edge, railing and wall lose the direct sun and regain it; the soffit and the thermal break stay shaded.",
                    "Ein Wolkenschatten wandert über den Balkon: Oberseite, Kante, Geländer und Wand verlieren die direkte Sonne und bekommen sie zurück; Unterseite und Dämmelement bleiben im Schatten."),
    "durationS": 8, "fps": 30, "loop": True, "rootMotion": "none",
    "keys": [{"t": 0, "pose": "self-weight", "ease": "smooth"}, {"t": 8, "pose": "self-weight", "ease": "smooth"}],
    "contacts": [], "phases": [{"id": "shadow", "fromS": 0, "toS": 8, "title": T("Cloud shadow passes", "Wolkenschatten zieht durch")}],
    "events": [{"t": 4, "kind": "thermal", "title": T("Deepest shade", "Tiefster Schatten")}],
    "tracks": [{"target": f"region:{r}:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": v}, {"t": 4, "v": 0.1 * v}, {"t": 8, "v": v}]}
               for r, v in (("slab-top", 1.0), ("slab-edge", 0.6), ("railing-steel", 0.8), ("wall-face", 0.6))]})

# ---- representations
D = Draw(a["rig"]["bones"])
SL, RL = "slab", "railing"
SLAB = ["slab-top", "slab-soffit", "slab-edge"]
wall_notch = [(-0.3, 0), (-0.12, 0), (-0.12, 0.2), (0, 0.2), (0, 1.3), (-0.3, 1.3)]

# 1:500
s500 = [
    D.poly("balcony-solid", SL, "body", 1, True, 0.25, [(-0.3, 0), (1.5, 0), (1.5, 0.2), (0, 0.2), (0, 1.3), (-0.3, 1.3)], regions=["wall-face", "thermal-break"] + SLAB),
    D.line("railing-line", RL, "body", 2, 0.45, [(1.45, 0.2), (1.45, 1.3)], region="railing-steel", penMm=0.13),
]
r500 = rep(500, s500, dec(
    ("One closed poché L merges wall stub, thermal break and slab (0.30 m wall plus 1.50 m slab, 1.30 m high); the railing is a single pen line.",
     "Eine geschlossene Poché-L-Kontur fasst Wandscheibe, Dämmelement und Platte zusammen (0,30 m Wand plus 1,50 m Platte, 1,30 m hoch); das Geländer ist eine einzige Linie."),
    ("Overall extents 1.80 x 1.30 m, wall thickness 0.30 m (0.6 mm), slab depth 0.20 m (0.4 mm), railing height 1.10 m.",
     "Gesamtmaße 1,80 x 1,30 m, Wanddicke 0,30 m (0,6 mm), Plattenstärke 0,20 m (0,4 mm), Geländerhöhe 1,10 m."),
    ("Concrete core, mineral wool, render, thermal break and slab are one solid; slab top, soffit and edge cannot be separated at 0.4 mm.",
     "Betonkern, Mineralwolle, Putz, Dämmelement und Platte sind ein Körper; Plattenoberseite, -unterseite und -kante lassen sich bei 0,4 mm nicht trennen."),
    ("Reinforcement, drip groove, base plate, bolts, handrail tube, mid-rail and every layer line.",
     "Bewehrung, Tropfkante, Fußplatte, Schrauben, Handlaufrohr, Zwischenstab und alle Schichtlinien."),
    ("The 50 mm post is a 0.13 mm pen line (65 mm real, inside the outline tolerance); the 60 mm handrail tube is merged into the end of that line.",
     "Der 50-mm-Pfosten ist eine Linie von 0,13 mm (65 mm real, innerhalb der Konturtoleranz); das 60-mm-Handlaufrohr geht im Linienende auf."),
    ("No hatch: the solid poché is the convention for cut elements at 1:500; any hatch spacing would be below the printable minimum.",
     "Keine Schraffur: Vollton-Poché ist bei 1:500 die Konvention für geschnittene Bauteile; jeder Schraffurabstand läge unter dem druckbaren Minimum."),
    ("Dark filled L reads as cut concrete; the thin line reads as steel.", "Die dunkle Fläche liest sich als geschnittener Beton; die dünne Linie als Stahl."),
    ("Thermal and structural overlays colour the merged solid with the area-weighted values of wall face, thermal break, slab top, soffit and edge (labelled merged); the line carries the railing steel.",
     "Thermik- und Statik-Überlagerungen färben den Körper mit den flächengewichteten Werten von Wandfläche, Dämmelement, Plattenoberseite, -unterseite und -kante (als zusammengefasst gekennzeichnet); die Linie trägt den Geländerstahl.")))

# 1:200
s200 = [
    D.poly("wall", SL, "body", 1, True, 0.6, wall_notch, region="wall-face", material="concrete"),
    D.line("wall-core-edge", SL, "detail", 1.5, 0.3, [(-0.12, 0.2), (-0.12, 1.3)]),
    D.rect("thermal-break-block", SL, "body", 2, True, 0.3, -0.12, 0, 0, 0.2, region="thermal-break", material="break-module"),
    D.rect("slab", SL, "body", 3, True, 0.72, 0, 0, 1.5, 0.2, regions=SLAB, material="concrete"),
    D.line("railing-post", RL, "body", 5, 0.45, [(1.45, 0.2), (1.45, 1.3)], region="railing-steel", material="steel", penMm=0.25),
]
r200 = rep(200, s200, dec(
    ("Wall, break element and slab are three separate masses in one assembly; the L shape stays readable, the railing is a post line with the handrail on top.",
     "Wand, Dämmelement und Platte sind drei getrennte Massen in einer Baugruppe; die L-Form bleibt lesbar, das Geländer ist eine Pfostenlinie mit Handlauf."),
    ("Wall (0.30 m, 1.5 mm), break module (0.6 x 1.0 mm), slab (7.5 x 1.0 mm), post line and the concrete/insulation edge inside the wall.",
     "Wand (0,30 m, 1,5 mm), Dämmelement (0,6 x 1,0 mm), Platte (7,5 x 1,0 mm), Pfostenlinie und die Kante Beton/Dämmung in der Wand."),
    ("Mineral wool and render are merged into the wall mass; slab top, soffit and edge are one slab shape.",
     "Mineralwolle und Putz sind in der Wandmasse zusammengefasst; Plattenoberseite, -unterseite und -kante bilden eine Plattenform."),
    ("Reinforcement, drip groove, base plate, bolts, mid-rail and the 20 mm render line.", "Bewehrung, Tropfkante, Fußplatte, Schrauben, Zwischenstab und die 20-mm-Putzlinie."),
    ("The 50 mm post is a line of 0.25 mm pen (50 mm real, exact); the handrail tube (60 mm, 0.3 mm) is merged into the post top.",
     "Der 50-mm-Pfosten ist eine Linie mit 0,25 mm Feder (50 mm real, exakt); das Handlaufrohr (60 mm, 0,3 mm) ist im Pfostenkopf zusammengefasst."),
    ("No hatch: tones separate concrete, break module and steel; the hatch spacing would be below the printable minimum.",
     "Keine Schraffur: Tonwerte trennen Beton, Dämmelement und Stahl; der Schraffurabstand läge unter dem druckbaren Minimum."),
    ("Mid-grey concrete, dark break module (the thermal bridge), thin steel line.", "Mittelgrauer Beton, dunkles Dämmelement (die Wärmebrücke), dünne Stahllinie."),
    ("Overlays colour wall, break module, slab (merged top, soffit, edge) and railing separately, so the thermal bridge at the wall joint is visible as its own cell.",
     "Überlagerungen färben Wand, Dämmelement, Platte (Ober-/Unterseite, Kante zusammengefasst) und Geländer getrennt; die Wärmebrücke am Wandanschluss ist als eigene Zelle sichtbar.")))

# 1:100
hatch100 = [H("concrete", "Reinforced concrete", "Stahlbeton", "stipple", 0, 0.8, 0.1), H("break", "Thermal-break module", "Dämmelement", "lines", 45, 0.5, 0.1)]
ins_line = zigzag(-0.06, 0.2, -0.06, 1.3, 11, 0.05, axis="y")
s100 = [
    D.rect("wall-core", SL, "body", 1, True, 0.6, -0.3, 0, -0.12, 1.3, region="wall-face", material="concrete", hatch="concrete"),
    D.rect("wall-insulation-render", SL, "body", 1, True, 0.9, -0.12, 0.2, 0, 1.3, region="wall-face", material="mineral-wool"),
    D.line("wall-insulation-zigzag", SL, "texture", 1.2, 0.3, ins_line, region="wall-face"),
    D.rect("thermal-break-block", SL, "body", 2, True, 0.3, -0.12, 0, 0, 0.2, region="thermal-break", material="break-module", hatch="break"),
    D.rect("slab", SL, "body", 3, True, 0.72, 0, 0, 1.5, 0.2, regions=SLAB, material="concrete", hatch="concrete"),
    D.rect("railing-post", RL, "body", 5, True, 0.45, 1.425, 0.2, 1.475, 1.24, region="railing-steel", material="steel"),
    D.circ("railing-handrail", RL, "body", 5.1, True, 0.35, 1.45, 1.27, 0.03, region="railing-steel", material="steel"),
]
r100 = rep(100, s100, dec(
    ("Wall layers and the cantilever slab are drawn as separate cut masses with material hatch; post and handrail are slim steel solids.",
     "Wandschichten und Kragplatte sind als getrennte Schnittmassen mit Materialschraffur gezeichnet; Pfosten und Handlauf sind schlanke Stahlkörper."),
    ("Concrete core (1.8 mm), insulation (1.2 mm) with zigzag line, thermal-break block (1.2 x 2 mm), slab 15 x 2 mm, post, handrail circle.",
     "Betonkern (1,8 mm), Dämmung (1,2 mm) mit Zickzacklinie, Dämmelement (1,2 x 2 mm), Platte 15 x 2 mm, Pfosten, Handlaufkreis."),
    ("The 20 mm render is merged into the insulation shape; slab top, soffit and edge are one slab shape.",
     "Der 20-mm-Putz ist in der Dämmungsform zusammengefasst; Plattenoberseite, -unterseite und -kante bilden eine Plattenform."),
    ("Reinforcement bars, drip groove, base plate (12 mm, 0.12 mm), bolts and the mid-rail.", "Bewehrungsstäbe, Tropfkante, Fußplatte (12 mm, 0,12 mm), Schrauben und der Zwischenstab."),
    ("The 50 mm post is a 0.5 mm filled bar (exact); the handrail tube is a 0.6 mm circle (exact).",
     "Der 50-mm-Pfosten ist ein gefüllter Balken von 0,5 mm (exakt); das Handlaufrohr ein Kreis von 0,6 mm (exakt)."),
    ("Concrete is stippled (0.8 mm), the break module hatched at 45 degrees (0.5 mm), mineral wool carries the DIN ISO 128-50 zigzag line; steel stays solid.",
     "Beton ist punktiert (0,8 mm), das Dämmelement 45° schraffiert (0,5 mm), Mineralwolle trägt die Zickzacklinie nach DIN ISO 128-50; Stahl bleibt voll."),
    ("Outlines 0.25 mm, zigzag and hatch 0.1 mm; light insulation, mid-grey concrete, dark steel.", "Konturen 0,25 mm, Zickzack und Schraffur 0,1 mm; helle Dämmung, mittelgrauer Beton, dunkler Stahl."),
    ("Wall layers, break module, slab and railing are separate overlay cells; the slab shows merged top, soffit and edge values.",
     "Wandschichten, Dämmelement, Platte und Geländer sind getrennte Überlagerungszellen; die Platte zeigt zusammengefasste Werte für Ober-/Unterseite und Kante.")), hatches=hatch100)

# 1:50
hatch50 = [H("concrete", "Reinforced concrete", "Stahlbeton", "stipple", 0, 1.0, 0.1), H("break", "Thermal-break module", "Dämmelement", "lines", 45, 0.8, 0.1)]
ins50 = zigzag(-0.07, 0.2, -0.07, 1.3, 14, 0.04, axis="y")
s50 = [
    D.rect("wall-core", SL, "body", 1, True, 0.6, -0.3, 0, -0.12, 1.3, region="wall-face", material="concrete", hatch="concrete"),
    D.line("wall-rebar-inner", SL, "detail", 1.1, 0.2, [(-0.27, 0.25), (-0.27, 1.25)], region="wall-face", penMm=0.2),
    D.line("wall-rebar-outer", SL, "detail", 1.1, 0.2, [(-0.15, 0.25), (-0.15, 1.25)], region="wall-face", penMm=0.2),
    D.rect("wall-insulation", SL, "body", 1, True, 0.9, -0.12, 0.2, -0.02, 1.3, region="wall-face", material="mineral-wool"),
    D.line("wall-insulation-zigzag", SL, "texture", 1.2, 0.3, ins50, region="wall-face"),
    D.rect("wall-render", SL, "body", 1, True, 0.55, -0.02, 0.2, 0, 1.3, region="wall-face", material="render"),
    D.rect("thermal-break-block", SL, "body", 2, True, 0.3, -0.12, 0, 0, 0.2, region="thermal-break", material="break-module", hatch="break"),
    D.rect("slab", SL, "body", 3, True, 0.72, 0, 0, 1.5, 0.2, regions=SLAB, material="concrete", hatch="concrete"),
    D.line("rebar-top", SL, "detail", 4, 0.1, [(-0.25, 0.165), (1.45, 0.165)], region="slab-top", penMm=0.25),
    D.line("rebar-top-hook", SL, "detail", 4, 0.1, [(-0.25, 0.165), (-0.25, 0.05)], region="wall-face", penMm=0.25),
    D.line("rebar-bottom", SL, "detail", 4, 0.1, [(0.0, 0.035), (1.0, 0.035)], region="slab-soffit", penMm=0.25),
    D.line("break-bar-top", SL, "detail", 4, 0.1, [(-0.12, 0.165), (0, 0.165)], region="thermal-break", penMm=0.25),
    D.line("break-bar-bottom", SL, "detail", 4, 0.1, [(-0.12, 0.035), (0, 0.035)], region="thermal-break", penMm=0.25),
    D.line("drip-groove", SL, "detail", 4, 0.2, [(1.43, 0), (1.43, 0.02)], region="slab-soffit"),
    D.rect("railing-base-plate", RL, "body", 5, True, 0.3, 1.40, 0.2, 1.50, 0.212, region="railing-steel", material="steel", penMm=0.18),
    D.rect("railing-bolt-a", RL, "detail", 5.3, True, 0.1, 1.405, 0.212, 1.425, 0.224, region="railing-steel", material="steel"),
    D.rect("railing-bolt-b", RL, "detail", 5.3, True, 0.1, 1.475, 0.212, 1.495, 0.224, region="railing-steel", material="steel"),
    D.rect("railing-post", RL, "body", 5, True, 0.45, 1.425, 0.212, 1.475, 1.24, region="railing-steel", material="steel"),
    D.circ("railing-handrail", RL, "body", 5.1, True, 0.35, 1.45, 1.27, 0.03, region="railing-steel", material="steel"),
    D.circ("railing-mid-rail", RL, "body", 5.1, True, 0.35, 1.45, 0.75, 0.02, region="railing-steel", material="steel"),
]
r50 = rep(50, s50, dec(
    ("Every layer of the wall, the break module, the cantilever slab with its reinforcement and the railing post with base plate, bolts, handrail and mid-rail are drawn as separate parts.",
     "Jede Wandschicht, das Dämmelement, die Kragplatte mit Bewehrung und der Geländerpfosten mit Fußplatte, Schrauben, Handlauf und Zwischenstab sind als eigene Teile gezeichnet."),
    ("Concrete core with two vertical bars, 100 mm mineral wool, 20 mm render, break module with two tension bars, slab with top and bottom reinforcement and hook, drip groove, plate, bolts, post, handrail, mid-rail.",
     "Betonkern mit zwei senkrechten Stäben, 100 mm Mineralwolle, 20 mm Putz, Dämmelement mit zwei Zugstäben, Platte mit oberer und unterer Bewehrung und Haken, Tropfkante, Platte, Schrauben, Pfosten, Handlauf, Zwischenstab."),
    ("Slab top, soffit and edge share one slab shape (separate regions are bound through the bar and groove details).",
     "Plattenoberseite, -unterseite und -kante teilen sich eine Plattenform (getrennte Regionen sind über Stab- und Tropfkantendetails gebunden)."),
    ("Rebar spacing, fillets, anchor details of the break module and the post section (hollow) which is smaller than the pen.",
     "Bewehrungsabstände, Hohlkehlen, Ankerdetails des Dämmelements und der Hohlprofilquerschnitt des Pfostens, der kleiner als die Feder ist."),
    ("Bars are drawn as pen lines of 0.25 mm (12 mm real, exact); the 12 mm base plate is 0.24 mm and drawn at true size; the 4 mm post wall is not resolved.",
     "Stäbe sind Federlinien von 0,25 mm (12 mm real, exakt); die 12-mm-Fußplatte ist 0,24 mm und in Originalgröße gezeichnet; die 4-mm-Pfostenwand wird nicht aufgelöst."),
    ("Concrete stippled (1.0 mm), break module hatched at 45 degrees (0.8 mm), mineral wool with the DIN ISO 128-50 zigzag line; render and steel solid.",
     "Beton punktiert (1,0 mm), Dämmelement 45° schraffiert (0,8 mm), Mineralwolle mit Zickzacklinie nach DIN ISO 128-50; Putz und Stahl voll."),
    ("Outlines 0.35 mm, details 0.18 mm, hatch 0.1 mm; reinforcement lines read darkest inside the concrete.",
     "Konturen 0,35 mm, Details 0,18 mm, Schraffur 0,1 mm; Bewehrungslinien sind im Beton am dunkelsten."),
    ("Every layer is its own overlay cell; bar details are bound to the surface they lie in, so the thermal bridge zone, slab surfaces and wall layers read separately.",
     "Jede Schicht ist eine eigene Überlagerungszelle; Stabdetails sind an die Fläche gebunden, in der sie liegen, so sind Wärmebrückenzone, Plattenflächen und Wandschichten getrennt lesbar."),
), hatches=hatch50)

a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["symbols"] = []
a["presentation"]["viewpoints"] = ["elevation"]
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
