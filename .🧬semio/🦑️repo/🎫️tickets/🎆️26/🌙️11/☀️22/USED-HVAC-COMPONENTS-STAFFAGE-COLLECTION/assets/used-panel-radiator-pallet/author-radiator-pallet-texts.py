"""📜 Texts, materials, thermal model, structure and decisions of the used panel radiator pallet, executed into the namespace of author-radiator-pallet.py."""

AREA = {"pallet": 3.42, "rack": 0.75, "radiator": 1.5, "convector": 2.8, "stack": 3.5, "valve": 0.0093, "thermostat-head": 0.016, "brackets": 0.054, "straps": 0.51}

SUMMARY = T(
    f"A pallet of eight dismantled steel panel radiators type 22 as a component exchange offers them: each 600 mm high, 1000 mm long and 100 mm deep (two profiled water panels, two convector layers, top grille and side covers, valve-compact with bottom connections G 3/4 at 50 mm centres), faded and yellowed white powder coat with scuffs and an old stock label, drained and capped. They stand on edge on two 80 x 80 mm battens in a tube rack bolted to a Euro pallet 1200 x 800 mm; seven are tied by two 50 mm ratchet straps, the eighth stands loose in front with its two wall brackets taped to its face and its thermostat head still fitted; {M_TOTAL:.0f} kg in all. Poses: stored, the loose radiator lifted out by hand, and reinstalled on its brackets 0.15 m above the floor, refilled and heating with 1600 W at 75/65/20 degC.",
    f"Eine Palette mit acht ausgebauten Stahl-Plattenheizkörpern Typ 22, wie eine Bauteilbörse sie anbietet: jeder 600 mm hoch, 1000 mm lang und 100 mm tief (zwei profilierte Wasserplatten, zwei Konvektorlagen, Abdeckgitter und Seitenblenden, Ventilkompakt mit Anschlüssen G 3/4 unten im Abstand 50 mm), verblichene und vergilbte weiße Pulverbeschichtung mit Schrammen und einem alten Lageretikett, entleert und verschlossen. Sie stehen hochkant auf zwei Kanthölzern 80 x 80 mm in einem Rohrgestell, das auf eine Europalette 1200 x 800 mm geschraubt ist; sieben halten zwei Ratschengurte 50 mm zusammen, der achte steht lose davor, seine zwei Wandkonsolen auf die Front geklebt und den Thermostatkopf noch aufgesetzt; zusammen {M_TOTAL:.0f} kg. Posen: gelagert, der lose Heizkörper von Hand herausgehoben, und wieder eingebaut an seinen Konsolen 0,15 m über dem Boden, befüllt und mit 1600 W bei 75/65/20 °C heizend.")

PROVENANCE = {
    "author": "Semio staffage component authors (used HVAC wave, author 10)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "en-442-1", "citation": "DIN EN 442-1:2015 Radiators and convectors, technical specifications and requirements (standard rating at 75/65/20 degC, excess temperature 49.8 K, radiator exponent n about 1.3 for panel radiators)."},
        {"id": "radiator-catalogues", "citation": "Catalogue data of steel panel radiators type 22 (Kermi therm-x2 Profil-V, Purmo Ventil Compact CV22, Buderus Logatrend VK-Profil 22): height 600 mm, length 1000 mm, depth 100 to 102 mm, rated output 1550 to 1660 W at 75/65/20 degC, mass 27.6 to 29.3 kg, water content 5.5 to 6.3 l, panel sheet 1.25 mm, bottom connections G 3/4 male at 50 mm centres on the right, built-in valve insert with M30 x 1.5 thread at the top right, wall brackets 35 to 50 mm wall distance; the radiator of this asset is a typical member of the family, not one model (authored-estimate)."},
        {"id": "en-13698-1", "citation": "DIN EN 13698-1:2004 Pallet production specification, construction specification for 800 mm x 1200 mm flat wooden pallets (EPAL Euro pallet): 144 mm high, deck boards 22 mm (145 and 100 mm wide), three stringer boards 145 x 22 mm, nine blocks 78 mm high, three bottom boards 22 mm; mass about 25 kg."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3 (450 kg/m3 used for the battens), E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "en-10219-2", "citation": "DIN EN 10219-2:2019 Cold formed welded structural hollow sections: square hollow section 30 x 30 x 2 mm, A 2.14 cm2 with rounded corners, 1.68 kg/m (rack mass taken from the table area, the drawn corners are square below the printable radius)."},
        {"id": "en-12195-2", "citation": "DIN EN 12195-2:2001 Load restraint assemblies on road vehicles, web lashing made from man-made fibres: polyester ratchet strap 50 mm, LC 2000 daN in the loop, web about 1.5 mm thick, web and ratchet about 0.45 kg for a 2.6 m loop."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (softwood 0.13 W/mK, 1600 J/kgK; steel 50 W/mK; polyester about 0.2 W/mK)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: white enamel and powder paint 0.9, galvanised and rusted steel 0.3 to 0.8, nickel-plated brass 0.1 to 0.3, plastics 0.9)."},
        {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch für Heizung und Klimatechnik, 79th ed., ITM 2019 (panel radiators: share of radiation about 30 % for type 22, part load by the radiator exponent, installation 100 to 150 mm above the floor)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."},
    ],
    "notes": T(
        "Dimensions, outputs, water content and masses of the radiators are typical catalogue values of steel panel radiators type 22, 600 x 1000 mm (authored estimates, nothing is measured); the pallet follows EN 13698-1, the rack tubes EN 10219-2 and the straps EN 12195-2. The radiators are shown as one product generation; the faded powder coat absorbs more sunlight than new paint (0.35 instead of about 0.25).",
        "Maße, Leistungen, Wasserinhalt und Massen der Heizkörper sind typische Katalogwerte von Stahl-Plattenheizkörpern Typ 22, 600 x 1000 mm (Schätzwerte, nichts ist gemessen); die Palette folgt EN 13698-1, die Gestellrohre EN 10219-2 und die Gurte EN 12195-2. Die Heizkörper sind als eine Produktgeneration gezeigt; die verblichene Pulverbeschichtung absorbiert mehr Sonnenstrahlung als neuer Lack (0,35 statt etwa 0,25)."),
}

MATERIALS = [
    {"id": "painted-steel", "title": T("White powder-coated sheet steel 1.25 mm, faded and yellowed", "Weiß pulverbeschichtetes Stahlblech 1,25 mm, verblichen und vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "rack-steel", "title": T("Hollow steel tube 30 x 30 x 2 mm, paint worn and rusty", "Stahl-Hohlprofil 30 x 30 x 2 mm, Lack abgenutzt und angerostet"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "en-10219-2"},
    {"id": "steel", "title": T("Steel (bearing pads of the structure list)", "Stahl (Auflagerpolster der Tragwerksliste)"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "brass", "title": T("Nickel-plated brass valve insert and connection nipples, tarnished", "Vernickelter Messing-Ventileinsatz und Anschlussnippel, angelaufen"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": "abs-plastic", "title": T("White ABS of the thermostat head and the protective caps, yellowed", "Weißes ABS des Thermostatkopfs und der Schutzkappen, vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "en-iso-10456"},
    {"id": "polyester", "title": T("Polyester web of the ratchet straps", "Polyestergewebe der Ratschengurte"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1380, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "literature", "reference": "en-12195-2"},
    {"id": "softwood", "title": T("Shipping-dry softwood of the pallet and the battens (at most 22 % moisture), greyed", "Versandtrockenes Nadelholz der Palette und der Kanthölzer (höchstens 22 % Holzfeuchte), vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 540, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
]

CIRCUIT = {"key": "heating", "rows": [{"v": 0.515, "supplyTempC": 55, "returnTempC": 45}, {"v": 0.748, "supplyTempC": 65, "returnTempC": 55}, {"v": 1, "supplyTempC": 75, "returnTempC": 65}]}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, model, material, length, geometry, layers, extra, src="authored-estimate", ref="radiator-catalogues"):
    """♨ One thermal region with its area from AREA and its heat capacity from its layers."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.2,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers), 1), "source": src, "reference": ref}
    out.update(extra)
    return out


SHEET_LAYER = layer("panel", "Powder-coated steel sheet 1.25 mm", "Pulverbeschichtetes Stahlblech 1,25 mm", "metal", "painted-steel", 0.00125, 50, 7850, 460, ref="radiator-catalogues")
PASSIVE = {"ratedPowerW": 0, "ratedPowerBasis": "passive"}

THERMAL = {
    "environment": {"airTempC": 18, "meanRadiantTempC": 17, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "heating", "title": T("Heating water through the reinstalled radiator", "Heizwasser durch den wieder eingebauten Heizkörper"), "levels": [
            {"v": 0, "title": T("Off: drained on the pallet, or thermostat closed", "Aus: entleert auf der Palette oder Thermostat geschlossen")},
            {"v": 0.515, "title": T("Night, 55/45 degC", "Nacht, 55/45 °C")},
            {"v": 0.748, "title": T("Comfort, 65/55 degC", "Komfort, 65/55 °C")},
            {"v": 1, "title": T("Rated, 75/65/20 degC, 1600 W", "Nennleistung, 75/65/20 °C, 1600 W")}]},
    ],
    "regions": [
        region("radiator", "Water panels, side covers and top grille of the loose radiator", "Wasserplatten, Seitenblenden und Abdeckgitter des losen Heizkörpers", "heater", "hydronic", "painted-steel", 0.6, "vertical-plate", [SHEET_LAYER],
               dict(PASSIVE, hydronic=dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.003))),
        region("convector", "Convector fins between the panels of the loose radiator", "Konvektorlamellen zwischen den Platten des losen Heizkörpers", "heater", "hydronic", "painted-steel", 0.6, "vertical-plate",
               [layer("fins", "Steel fin sheet 0.5 mm", "Stahl-Lamellenblech 0,5 mm", "metal", "painted-steel", 0.0005, 50, 7850, 460, ref="radiator-catalogues")],
               dict(PASSIVE, hydronic=dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.05), areaBasis="exchange")),
        region("valve", "Valve insert and bottom connection nipples of the loose radiator", "Ventileinsatz und untere Anschlussnippel des losen Heizkörpers", "surface", "hydronic", "brass", 0.03, "cylinder",
               [layer("brass", "Brass wall 3 mm", "Messingwand 3 mm", "metal", "brass", 0.003, 110, 8500, 380, ref="incropera-2018")], dict(PASSIVE, hydronic=dict(CIRCUIT, flowPosition=0.0, filmM2KW=0.01))),
        region("thermostat-head", "Thermostat head on the valve insert", "Thermostatkopf auf dem Ventileinsatz", "surface", "passive", "abs-plastic", 0.05, "cylinder",
               [layer("shell", "ABS shell 2 mm", "ABS-Schale 2 mm", "polymer", "abs-plastic", 0.002, 0.17, 1050, 1400)], PASSIVE),
        region("brackets", "Two wall brackets", "Zwei Wandkonsolen", "surface", "passive", "painted-steel", 0.04, "vertical-plate",
               [layer("sheet", "Steel sheet 2 mm", "Stahlblech 2 mm", "metal", "painted-steel", 0.002, 50, 7850, 460, ref="radiator-catalogues")], PASSIVE),
        region("stack", "The seven strapped radiators with caps and capped nipples", "Die sieben verzurrten Heizkörper mit Kappen und verschlossenen Nippeln", "surface", "passive", "painted-steel", 0.6, "vertical-plate", [SHEET_LAYER], PASSIVE),
        region("straps", "Two polyester ratchet straps", "Zwei Polyester-Ratschengurte", "surface", "passive", "polyester", 0.035, "vertical-plate",
               [layer("web", "Polyester web 1.5 mm", "Polyestergewebe 1,5 mm", "polymer", "polyester", 0.0015, 0.2, 1380, 1200, ref="en-12195-2")], dict(PASSIVE, source="literature", reference="en-12195-2")),
        region("rack", "Tube end frames of the rack", "Rohr-Stirnrahmen des Gestells", "surface", "passive", "rack-steel", 0.03, "cylinder",
               [layer("tube", "Steel tube wall 2 mm", "Stahlrohrwand 2 mm", "metal", "rack-steel", 0.002, 50, 7850, 460, ref="en-10219-2")], dict(PASSIVE, source="literature", reference="en-10219-2")),
        region("pallet", "Euro pallet and two battens", "Europalette und zwei Kanthölzer", "surface", "passive", "softwood", 0.1, "horizontal-plate-up",
               [layer("timber", "Softwood, thermally active 20 mm", "Nadelholz, thermisch wirksam 20 mm", "timber", "softwood", 0.02, 0.13, 540, 1600, ref="en-338")], dict(PASSIVE, source="literature", reference="en-13698-1")),
    ],
    "emitters": [
        {"id": "convection-outlet", "kind": "vent", "title": T("Warm air rising from the top grille of the reinstalled radiator", "Warme Luft aus dem Abdeckgitter des wieder eingebauten Heizkörpers"), "anchor": "grille-centre", "directionDeg": 90, "diameterM": 0.28, "operatingKey": "heating",
         "volumeFlowM3S": 0.033, "temperatureRiseK": 28, "source": "authored-estimate", "reference": "recknagel-2019"},
    ],
    "assumptions": [
        T("On the pallet and when lifted all eight radiators are drained, cold and at the 18 degC of the tempered hall of the exchange; reinstalled, the loose radiator is refilled with 5.8 l and its panels, convector and valve follow the heating circuit (75/65 degC rated, 65/55 comfort, 55/45 night), the thermostat head, the brackets and everything left on the pallet stay passive.",
          "Auf der Palette und beim Heben sind alle acht Heizkörper entleert, kalt und haben die 18 °C der temperierten Halle der Börse; eingebaut ist der lose Heizkörper mit 5,8 l befüllt, und seine Platten, sein Konvektor und sein Ventil folgen dem Heizkreis (75/65 °C Nennleistung, 65/55 Komfort, 55/45 Nacht), Thermostatkopf, Konsolen und alles auf der Palette bleiben passiv."),
        T("A radiator fed from the boiler is passive: 0 W own power; its rated output is 1600 W at 75/65/20 degC (EN 442), about 30 % as radiation and 1120 W as warm air of 0.033 m3/s rising 28 K above the room from the top grille.",
          "Ein vom Kessel versorgter Heizkörper ist passiv: 0 W Eigenleistung; seine Normleistung beträgt 1600 W bei 75/65/20 °C (EN 442), etwa 30 % als Strahlung und 1120 W als warme Luft von 0,033 m3/s, die 28 K über Raumtemperatur aus dem Abdeckgitter steigt."),
        T("The faded, yellowed powder coat keeps the emissivity 0.9 of white paint and absorbs 0.35 of the solar radiation instead of about 0.25 when new; the convector area is the exchange area of the two fin layers (2.8 m2), not the box that stands for them.",
          "Die verblichene, vergilbte Pulverbeschichtung behält den Emissionsgrad 0,9 weißen Lacks und absorbiert 0,35 der Sonnenstrahlung statt etwa 0,25 im Neuzustand; die Konvektorfläche ist die Austauschfläche der zwei Lamellenlagen (2,8 m2), nicht der Quader, der sie vertritt."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the {M_TOTAL:.0f} kg pallet unit stands with its bottom boards on the hall floor (contacts, friction 0.5) and the radiators stand on edge on the two 80 x 80 mm battens; lifted, two hands at the top grille carry the loose {M_R1:.1f} kg radiator (pins) while the pallet keeps standing; reinstalled, the two wall brackets carry the radiator (pins at the wall).",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert steht die {M_TOTAL:.0f} kg schwere Paletteneinheit mit ihren Bodenbrettern auf dem Hallenboden (Kontakte, Reibung 0,5), und die Heizkörper stehen hochkant auf den zwei Kanthölzern 80 x 80 mm; gehoben tragen zwei Hände am Abdeckgitter den losen, {M_R1:.1f} kg schweren Heizkörper (Gelenke), während die Palette stehen bleibt; eingebaut tragen ihn die zwei Wandkonsolen (Gelenke an der Wand)."),
    T("The solver treats the whole asset as one rigid body; with the pallet contacts and the grips or brackets together the reactions are shared, so the grip and bracket reactions are a share of the asset weight, not the radiator weight alone. The strapped seven are one member on the battens, each radiator a pair of 1.25 mm panels joined by the convector.",
      "Der Löser behandelt das ganze Asset als einen starren Körper; mit den Palettenkontakten und den Griffen oder Konsolen zusammen teilen sich die Reaktionen, so dass die Griff- und Konsolenreaktionen ein Anteil des Assetgewichts sind, nicht nur des Heizkörpergewichts. Die sieben verzurrten sind ein Bauteil auf den Kanthölzern, jeder Heizkörper ein Paar Platten 1,25 mm, verbunden durch den Konvektor."),
    T("Reinstalled, the radiator carries its 5.8 l of water (57 N) and the two connection pipes with the H-block (30 N each); the masses of pallet, battens and rack follow from their sections and EN 13698-1, those of the radiators from the catalogues.",
      "Eingebaut trägt der Heizkörper seine 5,8 l Wasser (57 N) und die zwei Anschlussrohre mit dem Hahnblock (je 30 N); die Massen von Palette, Kanthölzern und Gestell folgen aus ihren Querschnitten und EN 13698-1, die der Heizkörper aus den Katalogen."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(
        ("Side elevation from the near long side, 24 mm long and 16.5 mm high on paper: the loose radiator with its profiled front panel, the two taped brackets, the old label and the thermostat head on the right, in front of the end frames of the rack, on the two battens and the Euro pallet with its blocks and boards. Front, 16.8 x 16.5 mm: the right end frame of the rack around the ends of the eight radiators, the head, the valve caps, the straps around the seven and the battens on the pallet. Plan, 24 x 16.8 mm: eight top grilles side by side, the straps across seven, the brackets, the head, the rack and the pallet deck.",
         "Seitenansicht von der vorderen Längsseite, auf Papier 24 mm lang und 16,5 mm hoch: der lose Heizkörper mit seiner profilierten Frontplatte, den zwei angeklebten Konsolen, dem alten Etikett und dem Thermostatkopf rechts, vor den Stirnrahmen des Gestells, auf den zwei Kanthölzern und der Europalette mit ihren Klötzen und Brettern. Vorderansicht, 16,8 x 16,5 mm: der rechte Stirnrahmen des Gestells um die Enden der acht Heizkörper, der Kopf, die Ventilkappen, die Gurte um die sieben und die Kanthölzer auf der Palette. Grundriss, 24 x 16,8 mm: acht Abdeckgitter nebeneinander, die Gurte über sieben, die Konsolen, der Kopf, das Gestell und das Palettendeck."),
        ("Every part at its true size: the two 20 mm water panels, the 1.5 mm side covers, the 14 mm grille with its slots, the 24 mm nipple shanks, the 30 mm caps, the 52 mm head, the 35 mm brackets with their tape, the 50 mm straps of 1.5 mm web, the 30 mm tubes, five deck boards, three stringers, nine blocks and three bottom boards of the pallet.",
         "Jedes Teil in wahrer Größe: die zwei 20 mm Wasserplatten, die 1,5 mm Seitenblenden, das 14 mm Gitter mit seinen Schlitzen, die 24 mm Nippelschäfte, die 30 mm Kappen, der 52 mm Kopf, die 35 mm Konsolen mit ihrem Klebeband, die 50 mm Gurte aus 1,5 mm Gewebe, die 30 mm Rohre, fünf Deckbretter, drei Querbretter, neun Klötze und drei Bodenbretter der Palette."),
        ("The seven radiators behind the loose one are hidden in the elevation except where they show in the reinstalled pose; the convectors show only from below; in elevation and front each pallet block is drawn together with the end of the stringer board above it, the joint between them is not drawn.",
         "Die sieben Heizkörper hinter dem losen sind in der Seitenansicht verdeckt, außer wo die Einbaupose sie zeigt; die Konvektoren zeigen sich nur von unten; in Seiten- und Vorderansicht ist jeder Palettenklotz zusammen mit dem Ende des Querbretts darüber gezeichnet, die Fuge dazwischen nicht."),
        ("The fins inside the convectors and the nails of the pallet are not drawn.", "Die Lamellen in den Konvektoren und die Nägel der Palette sind nicht gezeichnet."),
        ("The 1.5 mm side covers and straps are 0.03 mm on paper and print as lines; the profile channels of the front panel are texture strokes at 33 mm pitch.",
         "Die 1,5 mm Seitenblenden und Gurte messen 0,03 mm auf Papier und drucken als Linien; die Profilrillen der Frontplatte sind Texturstriche im Abstand von 33 mm."),
        ("The sawn ends of the battens carry a 45 degree end-grain hatch in the elevation.", "Die gesägten Enden der Kanthölzer tragen in der Seitenansicht eine Hirnholzschraffur unter 45 Grad."),
        ("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; tones: radiators 0.86, grille 0.8, convector 0.55, timber 0.5 to 0.7, rack 0.3, straps 0.2, head 0.9, caps 0.12.",
         "Körperstift 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Töne: Heizkörper 0,86, Gitter 0,8, Konvektor 0,55, Holz 0,5 bis 0,7, Gestell 0,3, Gurte 0,2, Kopf 0,9, Kappen 0,12."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    100: D(
        ("Side elevation 12 x 8.3 mm, front 8.4 x 8.3 mm, plan 12 x 8.4 mm: the loose radiator and the strapped seven as two boxes, the brackets, head, caps and connection blocks, the rack and the pallet with its bottom boards, nine blocks and deck slab.",
         "Seitenansicht 12 x 8,3 mm, Vorderansicht 8,4 x 8,3 mm, Grundriss 12 x 8,4 mm: der lose Heizkörper und die verzurrten sieben als zwei Quader, Konsolen, Kopf, Kappen und Anschlussblöcke, das Gestell und die Palette mit Bodenbrettern, neun Klötzen und Deckplatte."),
        ("The 100 mm radiators with grille edge and panel joints as detail strokes, the 30 mm caps, 35 mm brackets, the 50 mm straps, the 22 mm bottom boards and the 78 mm blocks of the pallet.",
         "Die 100 mm Heizkörper mit Gitterkante und Plattenfugen als Detailstriche, die 30 mm Kappen, 35 mm Konsolen, die 50 mm Gurte, die 22 mm Bodenbretter und die 78 mm Klötze der Palette."),
        ("Each radiator is one box (panels, convector, grille and covers merged); the seven strapped ones are one block 0.7 m deep with their joints as lines; the two connection nipples 24 mm apart (0.24 mm, below the printable gap) are one connection block; the five deck boards with their 41 mm gaps and the three stringers are one 44 mm deck slab.",
         "Jeder Heizkörper ist ein Quader (Platten, Konvektor, Gitter und Blenden zusammengefasst); die sieben verzurrten sind ein Block von 0,7 m Tiefe mit ihren Fugen als Linien; die zwei Anschlussnippel mit 24 mm Abstand (0,24 mm, unter dem druckbaren Spalt) sind ein Anschlussblock; die fünf Deckbretter mit ihren 41 mm Fugen und die drei Querbretter sind eine Deckplatte von 44 mm."),
        ("Profile channels, grille slots, tape, label, scuffs and grain.", "Profilrillen, Gitterschlitze, Klebeband, Etikett, Schrammen und Maserung."),
        ("The 22 mm bottom boards are 0.22 mm on paper; the straps stay as 0.015 mm sheets drawn as lines.", "Die 22 mm Bodenbretter messen 0,22 mm auf Papier; die Gurte bleiben 0,015 mm Bleche, als Linien gezeichnet."),
        ("None.", "Keine."),
        ("Body pen 0.25 mm, detail 0.13 mm; the tones of 1:50.", "Körperstift 0,25 mm, Details 0,13 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    200: D(
        ("Side elevation 6 x 4.1 mm, front 4.2 x 4.1 mm, plan 6 x 4.2 mm: the two radiator boxes on the battens, the head, the brackets, the straps, the rack frames and the pallet as a deck slab on nine feet.",
         "Seitenansicht 6 x 4,1 mm, Vorderansicht 4,2 x 4,1 mm, Grundriss 6 x 4,2 mm: die zwei Heizkörperquader auf den Kanthölzern, Kopf, Konsolen, Gurte, die Gestellrahmen und die Palette als Deckplatte auf neun Füßen."),
        ("Radiator boxes, the 30 x 30 mm tubes, 80 mm battens, 35 mm brackets, the 52 mm head on a 30 mm valve body and the straps as 50 mm wide sheets.",
         "Heizkörperquader, die 30 x 30 mm Rohre, 80 mm Kanthölzer, 35 mm Konsolen, der 52 mm Kopf auf einem 30 mm Ventilkörper und die Gurte als 50 mm breite Bleche."),
        ("The five deck boards and three stringers are one 44 mm deck slab; each block column with its bottom board is one 100 mm foot.",
         "Die fünf Deckbretter und drei Querbretter sind eine Deckplatte von 44 mm; jede Klotzsäule mit ihrem Bodenbrett ist ein Fuß von 100 mm."),
        ("Caps and nipples (20 and 25 mm, below the 30 mm feature), every detail stroke, and in the front view the 1.5 mm edges of the strap runs beside the stack (0.0075 mm on paper).", "Kappen und Nippel (20 und 25 mm, unter dem Merkmal von 30 mm), alle Detailstriche und in der Vorderansicht die 1,5 mm Kanten der Gurtläufe neben dem Stapel (0,0075 mm auf Papier)."),
        ("No solid part below 30 mm (0.15 mm on paper) remains apart from the declared strap sheets.", "Kein Vollteil unter 30 mm (0,15 mm auf Papier) bleibt außer den deklarierten Gurtblechen."),
        ("None.", "Keine."),
        ("One body pen of 0.18 mm; the tones of 1:50.", "Ein Körperstift von 0,18 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
}

DECISIONS_3D = {
    50: [T("Body: every radiator as two 20 mm panel boxes with 4 mm rounded edges, the convector box between them, the 14 mm top grille and two 1.5 mm side covers over the full depth as declared sheets, the seven strapped ones as a repeat at 100 mm pitch; head, valve insert, caps and nipples as revolves, the straps as hollow extrusions round the seven, brackets, battens, rack tubes and every board and block of the Euro pallet as boxes.",
           "Körper: jeder Heizkörper als zwei Plattenquader von 20 mm mit 4 mm gerundeten Kanten, der Konvektorquader dazwischen, das 14 mm Abdeckgitter und zwei 1,5 mm Seitenblenden über die ganze Tiefe als deklarierte Bleche, die sieben verzurrten als Wiederholung im Abstand von 100 mm; Kopf, Ventileinsatz, Kappen und Nippel als Drehkörper, die Gurte als hohle Extrusionen um die sieben, Konsolen, Kanthölzer, Gestellrohre und jedes Brett und jeder Klotz der Europalette als Quader."),
         T("Thermal: every region by reference to its body solids, the convector of the loose radiator drawn with the radiator region and as its own exchange-area box, and the outlet patch on its grille. Structure: the pallet parts, battens and rack as the pallet member, the seven and the loose radiator as box members, bearing pads on the battens as joints, mass markers at the centres of the pallet boards, each batten, each rack frame, the seven and the loose radiator.",
           "Wärme: jede Region als Verweis auf ihre Körper, der Konvektor des losen Heizkörpers mit der Heizkörperregion gezeichnet und als eigener Quader der Austauschfläche, und das Auslassfeld auf seinem Gitter. Tragwerk: Palettenteile, Kanthölzer und Gestell als Palettenbauteil, die sieben und der lose Heizkörper als Quaderbauteile, Auflagerpolster auf den Kanthölzern als Gelenke, Massenmarken in den Schwerpunkten der Palettenbretter, jedes Kantholzes, jedes Gestellrahmens, der sieben und des losen Heizkörpers.")],
    100: [T("Body for the 15 mm feature: each radiator one box, the seven strapped ones one block 0.7 m deep; the two nipples one connection block, the deck boards and stringers one 44 mm deck slab; caps, head, brackets, straps, rack, blocks and bottom boards keep their true sections (all at least 15 mm, the straps as declared sheets).",
            "Körper für das Merkmal von 15 mm: jeder Heizkörper ein Quader, die sieben verzurrten ein Block von 0,7 m Tiefe; die zwei Nippel ein Anschlussblock, Deckbretter und Querbretter eine Deckplatte von 44 mm; Kappen, Kopf, Konsolen, Gurte, Gestell, Klötze und Bodenbretter behalten ihre wahren Querschnitte (alle mindestens 15 mm, die Gurte als deklarierte Bleche)."),
          T("Thermal: the regions by reference and the convector box inside the loose radiator; structure as at 1:50 with 15 mm pads.",
            "Wärme: die Regionen als Verweise und der Konvektorquader im losen Heizkörper; Tragwerk wie bei 1:50 mit Polstern von 15 mm.")],
    200: [T("Body for the 30 mm feature: the pallet as one 44 mm deck slab on nine 100 mm feet, the two radiator boxes, the 30 mm tubes, battens, brackets, straps, a 30 mm valve body and the head; caps and nipples are left out.",
            "Körper für das Merkmal von 30 mm: die Palette als eine Deckplatte von 44 mm auf neun Füßen von 100 mm, die zwei Heizkörperquader, die 30 mm Rohre, Kanthölzer, Konsolen, Gurte, ein Ventilkörper von 30 mm und der Kopf; Kappen und Nippel entfallen."),
          T("Thermal: the regions by reference, the convector box and a 30 mm outlet patch; structure: deck slab, feet, battens and rack as the pallet member, the two radiator boxes, 30 mm pads and the mass markers.",
            "Wärme: die Regionen als Verweise, der Konvektorquader und ein Auslassfeld von 30 mm; Tragwerk: Deckplatte, Füße, Kanthölzer und Gestell als Palettenbauteil, die zwei Heizkörperquader, Polster von 30 mm und die Massenmarken.")],
}
