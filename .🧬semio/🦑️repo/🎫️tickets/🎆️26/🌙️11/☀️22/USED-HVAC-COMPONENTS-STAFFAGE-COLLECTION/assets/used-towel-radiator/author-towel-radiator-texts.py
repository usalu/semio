"""📜 Texts, materials, thermal model, structure and decisions of the used towel radiator, executed into the namespace of author-towel-radiator.py."""

AREA = {"rails": 0.535, "upright-flow": 0.135, "upright-return": 0.14, "valve": 0.015, "brackets": 0.051, "rack": 1.80, "edge-strips": 0.293, "sills": 0.330}

SUMMARY = T(
    f"A used chrome-plated ladder towel radiator of 500 x 1200 mm as a component exchange offers it: two D-section steel uprights 35 x 30 mm with flat backs to the wall and eighteen round 22 mm rails in three groups of six, still carrying its four chrome wall brackets with their rosettes, a chrome thermostatic angle valve body on the flow upright with its head removed, a white protection cap on the spindle and a cut 15 mm copper stub, a blanking plug in the return tapping and the bleed plug on top; the chrome is dulled and worn through in spots where wet towels hung. Drained, {M_RADIATOR:.1f} kg, it leans {LEAN:.0f} degrees back in a steel A-frame rack of 40 x 40 mm tubes, standing on an EPDM-covered timber sill block and resting against the EPDM edge strip of the upper rail ({M_TOTAL:.0f} kg with the rack). Poses: stored in the A-frame, lifted on a round sling with the rack left behind, and reinstalled on a bathroom wall at 75/65 degC (EN 442 rating point), at the comfort setting and cold.",
    f"Ein gebrauchter verchromter Leiter-Badheizkörper 500 x 1200 mm, wie ihn eine Bauteilbörse anbietet: zwei D-förmige Stahl-Sammelrohre 35 x 30 mm mit flacher Rückseite zur Wand und achtzehn runde Sprossen 22 mm in drei Gruppen zu sechs, noch mit seinen vier verchromten Wandkonsolen und deren Rosetten, einem verchromten Thermostat-Eckventilkörper am Vorlauf-Sammelrohr mit abgenommenem Kopf, einer weißen Schutzkappe auf der Spindel und einem abgetrennten Kupferrohrstummel 15 mm, einem Blindstopfen im Rücklaufanschluss und dem Entlüftungsstopfen oben; das Chrom ist matt und dort, wo nasse Handtücher hingen, stellenweise durchgerieben. Entleert, {M_RADIATOR:.1f} kg, lehnt er um {LEAN:.0f} Grad zurück in einem Stahl-A-Bock aus Rohren 40 x 40 mm, steht auf einem EPDM-belegten Holz-Auflageklotz und liegt am EPDM-Kantenschutz der oberen Traverse an ({M_TOTAL:.0f} kg mit dem Bock). Posen: im A-Bock gelagert, an einer Rundschlinge gehoben, wobei der Bock stehen bleibt, und wieder an einer Badwand eingebaut bei 75/65 °C (Normpunkt EN 442), in der Komfortstufe und kalt.")

PROVENANCE = {
    "author": "Semio staffage component authors (used HVAC wave, author 11)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "en-442-2", "citation": "EN 442-2:2014 Radiators and convectors, part 2: test methods and rating (standard rating at 75/65/20 degC, excess temperature 50 K, heat output characteristic Q = K dT^n)."},
        {"id": "towel-radiator-catalogues", "citation": "Catalogue data of chrome-plated steel ladder towel radiators of the 2000s and 2010s (Zehnder, Kermi, Vogel & Noot, Purmo and the generic chrome ladder trade ranges): 500 mm wide, 1200 mm high, uprights round 25 to 30 mm or D-section about 35 x 30 mm, 16 to 22 round rails 22 mm in groups, about 4 to 6 l water, 8 to 11 kg, G 1/2 bottom tappings at 450 to 470 mm centres, output about 350 to 400 W chrome and 500 to 600 W white at 75/65/20 degC (chrome some 30 to 40 % less), four clamp brackets with stand-off pins and rosettes; this radiator is a typical member of the family, not one model (authored-estimate)."},
        {"id": "en-10305-3", "citation": "EN 10305-3:2016 Steel tubes for precision applications, welded cold sized tubes (22 x 1.2 mm: 0.616 kg/m)."},
        {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded structural hollow sections, tolerances, dimensions and sectional properties (SHS 40 x 40 x 2: 2.31 kg/m, A 2.94 cm2, I 6.94 cm4)."},
        {"id": "en-215", "citation": "EN 215:2019 Thermostatic radiator valves, requirements and test methods (valve body with G 1/2 tail, removable sensor head on the M30 x 1.5 spindle thread)."},
        {"id": "iso-228-1", "citation": "ISO 228-1:2000 Pipe threads where pressure-tight joints are not made on the threads (G 1/2 tappings of the uprights)."},
        {"id": "en-1057", "citation": "EN 1057:2006+A1:2010 Copper and copper alloys, seamless round copper tubes for water and gas (15 mm connection pipe)."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3 (450 kg/m3 used for the air-dry sill blocks), E0,mean 11 GPa."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (EPDM rubber lambda 0.25 W/mK, 1150 kg/m3, 1000 J/kgK; softwood 0.13 W/mK; steel 50 W/mK)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: polished chromium 0.06 to 0.1, oxidised copper 0.6, paints 0.9, rubber 0.9)."},
        {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch für Heizung und Klimatechnik, 79th ed., ITM 2019 (room heating surfaces: towel radiators, share of radiation of bright metal surfaces, design temperatures)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."},
    ],
    "notes": T(
        "Dimensions, rail count, water content, masses and output are typical values of chrome ladder towel radiators (authored estimates from the cited catalogues, nothing is measured); tubes, rack sections and the timber follow EN 10305-3, EN 10219-2 and EN 338. The A-frame rack, its sill blocks and its EPDM edge strips are storage aids of the component exchange and stay behind when the radiator is lifted.",
        "Maße, Sprossenzahl, Wasserinhalt, Massen und Leistung sind typische Werte verchromter Leiter-Badheizkörper (Schätzwerte nach den zitierten Katalogen, nichts ist gemessen); Rohre, Bockprofile und Holz folgen EN 10305-3, EN 10219-2 und EN 338. Der A-Bock, seine Auflageklötze und sein EPDM-Kantenschutz sind Lagerhilfen der Bauteilbörse und bleiben beim Anheben des Heizkörpers stehen."),
}

MATERIALS = [
    {"id": "chrome-steel", "title": T("Chrome-plated steel tube, dulled, worn through in spots", "Verchromtes Stahlrohr, matt, stellenweise durchgerieben"), "emissivity": 0.12, "solarAbsorptivity": 0.4, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "chrome-brass", "title": T("Chrome-plated brass of valve body, nuts and plugs", "Verchromtes Messing von Ventilkörper, Muttern und Stopfen"), "emissivity": 0.1, "solarAbsorptivity": 0.4, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": "copper", "title": T("Oxidised copper pipe stub 15 mm, sawn", "Oxidierter Kupferrohrstummel 15 mm, abgesägt"), "emissivity": 0.6, "solarAbsorptivity": 0.65, "densityKgM3": 8900, "conductivityWmK": 390, "specificHeatJKgK": 385, "source": "literature", "reference": "en-1057"},
    {"id": "abs-plastic", "title": T("White ABS protection cap on the valve spindle", "Weiße ABS-Schutzkappe auf der Ventilspindel"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "en-iso-10456"},
    {"id": "painted-steel", "title": T("Grey powder-coated steel tube of the A-frame, scuffed", "Grau pulverbeschichtetes Stahlrohr des A-Bocks, verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "en-10219-2"},
    {"id": "epdm", "title": T("Black EPDM edge-protection strip", "Schwarzer EPDM-Kantenschutz"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "softwood", "title": T("Softwood sill block C24, grey with age", "Nadelholz-Auflageklotz C24, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
]

CIRCUIT_ROWS = [{"v": 0.515, "supplyTempC": 55, "returnTempC": 45}, {"v": 0.748, "supplyTempC": 65, "returnTempC": 55}, {"v": 1, "supplyTempC": 75, "returnTempC": 65}]


def hydro(pos, film):
    """💧 The heating circuit seen at one position along the water path."""
    return {"key": "heating", "rows": CIRCUIT_ROWS, "flowPosition": pos, "filmM2KW": film}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, model, material, length, geometry, layers, extra, water_m=0.0, src="authored-estimate", ref="towel-radiator-catalogues"):
    """🔥 One thermal region with its area from AREA and its heat capacity from its layers plus the water behind each square metre."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.5,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers) + water_m * 1000 * 4186, 1), "source": src, "reference": ref}
    out.update(extra)
    return out


TUBE = layer("tube", "Chrome-plated steel wall 1.2 mm", "Verchromte Stahlwand 1,2 mm", "metal", "chrome-steel", 0.0012, 50, 7850, 460, ref="en-10305-3")
RAIL_WATER = (RAIL_R - 0.0012) ** 2 / (2 * RAIL_R)
UP_WATER = (UP_AREA - UP_PERIM * 0.0012) / UP_PERIM

THERMAL = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 19, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "heating", "title": T("Heating water through the thermostatic valve", "Heizwasser durch das Thermostatventil"), "levels": [
            {"v": 0, "title": T("Off: drained in store, or valve closed", "Aus: im Lager entleert oder Ventil geschlossen")},
            {"v": 0.515, "title": T("Night, dT 30 K (55/45/20 degC)", "Nacht, dT 30 K (55/45/20 °C)")},
            {"v": 0.748, "title": T("Comfort, dT 40 K (65/55/20 degC)", "Komfort, dT 40 K (65/55/20 °C)")},
            {"v": 1, "title": T("Rated, dT 50 K (75/65/20 degC)", "Nennleistung, dT 50 K (75/65/20 °C)")}]},
    ],
    "regions": [
        region("rails", "Eighteen chrome rails 22 mm", "Achtzehn verchromte Sprossen 22 mm", "heater", "hydronic", "chrome-steel", 0.022, "cylinder", [TUBE],
               {"hydronic": hydro(0.5, 0.002), "ratedPowerW": 0, "ratedPowerBasis": "passive"}, water_m=RAIL_WATER),
        region("upright-flow", "Left D-section upright, flow side with the valve", "Linkes D-Sammelrohr, Vorlaufseite mit Ventil", "heater", "hydronic", "chrome-steel", 1.19, "vertical-plate", [TUBE],
               {"hydronic": hydro(0.1, 0.002)}, water_m=UP_WATER),
        region("upright-return", "Right D-section upright, return side with the blanking plug", "Rechtes D-Sammelrohr, Rücklaufseite mit Blindstopfen", "heater", "hydronic", "chrome-steel", 1.19, "vertical-plate", [TUBE],
               {"hydronic": hydro(0.9, 0.002)}, water_m=UP_WATER),
        region("valve", "Thermostatic angle valve body, nut, copper stub and cap", "Thermostat-Eckventilkörper, Mutter, Kupferstummel und Kappe", "surface", "hydronic", "chrome-brass", 0.03, "cylinder",
               [layer("brass", "Brass wall 3 mm", "Messingwand 3 mm", "metal", "chrome-brass", 0.003, 110, 8500, 380, ref="en-215")], {"hydronic": hydro(0.0, 0.01)}, ref="en-215"),
        region("brackets", "Four chrome wall brackets with stand-off pins and rosettes", "Vier verchromte Wandkonsolen mit Abstandsbolzen und Rosetten", "surface", "passive", "chrome-steel", 0.05, "cylinder",
               [layer("zinc", "Chrome-plated die casting 3 mm", "Verchromter Druckguss 3 mm", "metal", "chrome-steel", 0.003, 50, 7850, 460, ref="incropera-2018")], {}),
        region("rack", "Powder-coated A-frame of SHS 40 x 40 x 2", "Pulverbeschichteter A-Bock aus MSH 40 x 40 x 2", "surface", "passive", "painted-steel", 0.04, "vertical-plate",
               [layer("steel", "Steel wall 2 mm", "Stahlwand 2 mm", "metal", "painted-steel", 0.002, 50, 7850, 460, ref="en-10219-2")], {"source": "literature", "reference": "en-10219-2"}),
        region("edge-strips", "EPDM edge strips on the upper rails and sill blocks", "EPDM-Kantenschutz auf den oberen Traversen und Auflageklötzen", "surface", "passive", "epdm", 0.04, "vertical-plate",
               [layer("rubber", "EPDM 15 mm", "EPDM 15 mm", "insulation", "epdm", 0.015, 0.25, 1150, 1000)], {"source": "literature", "reference": "en-iso-10456"}),
        region("sills", "Two softwood sill blocks", "Zwei Nadelholz-Auflageklötze", "surface", "passive", "softwood", 0.06, "vertical-plate",
               [layer("timber", "Softwood, thermally active 20 mm", "Nadelholz, thermisch wirksam 20 mm", "timber", "softwood", 0.02, 0.13, 450, 1600, ref="en-338")], {"source": "literature", "reference": "en-338"}),
    ],
    "emitters": [
        {"id": "radiator-plume", "kind": "plume", "title": T("Free-convection plume rising over the top rail", "Freie Konvektionsfahne über der obersten Sprosse"), "anchor": "radiator-top", "directionDeg": 90, "diameterM": 0.12, "region": "rails", "source": "authored-estimate", "reference": "recknagel-2019"},
    ],
    "assumptions": [
        T("Reference case: a bathroom at 20 degC (the EN 442 reference air temperature), no sun; in store the drained radiator and the rack share the same 20 degC as an assumption of a heated hall.",
          "Referenzfall: ein Bad mit 20 °C (EN-442-Bezugslufttemperatur), keine Sonne; im Lager haben der entleerte Heizkörper und der Bock dieselben 20 °C als Annahme einer beheizten Halle."),
        T(f"No heat is authored on the radiator: it is fed from elsewhere and its rated power is passive, 0 W. The water enters the flow upright through the valve at the supply temperature and falls along the rails to the return upright (75/65, 65/55 and 55/45 degC at the rated, comfort and night levels); the {WATER_L:.1f} l of water are in the heat capacity of the rails and uprights. A chrome towel radiator of this size gives off about 350 to 400 W at 75/65/20 degC, some 30 to 40 % less than a white one, because chrome of emissivity 0.12 radiates little and the output is mostly convection.",
          f"Am Heizkörper ist keine Wärme vorgegeben: er wird von außen versorgt, seine Nennleistung ist passiv, 0 W. Das Wasser tritt mit Vorlauftemperatur durch das Ventil in das Vorlauf-Sammelrohr und kühlt über die Sprossen bis zum Rücklauf-Sammelrohr ab (75/65, 65/55 und 55/45 °C bei Nenn-, Komfort- und Nachtstufe); die {WATER_L:.1f} l Wasser stecken in der Wärmekapazität von Sprossen und Sammelrohren. Ein verchromter Badheizkörper dieser Größe gibt bei 75/65/20 °C etwa 350 bis 400 W ab, rund 30 bis 40 % weniger als ein weißer, weil Chrom mit einem Emissionsgrad von 0,12 kaum strahlt und die Leistung überwiegend Konvektion ist."),
        T("The dulled chrome with worn-through spots is taken at emissivity 0.12 (bright chromium 0.06 to 0.1 when new, Incropera); the scuffed powder coat of the rack, the EPDM and the grey timber at 0.9.",
          "Das matte, stellenweise durchgeriebene Chrom wird mit einem Emissionsgrad von 0,12 angesetzt (blankes Chrom neu 0,06 bis 0,1, Incropera); die verschrammte Pulverbeschichtung des Bocks, das EPDM und das vergraute Holz mit 0,9."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the A-frame stands with its four foot ends on the floor and carries the {M_RADIATOR:.1f} kg radiator on its sill block and its upper edge strip; lifted, the round sling at the top rail holds the radiator while the rack feet stay on the floor; reinstalled, the four bracket rosettes are pinned to the bathroom wall and the empty rack stands aside on its feet.",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert steht der A-Bock mit seinen vier Fußenden auf dem Boden und trägt den {M_RADIATOR:.1f} kg schweren Heizkörper auf seinem Auflageklotz und seinem oberen Kantenschutz; gehoben hält die Rundschlinge an der obersten Sprosse den Heizkörper, während die Bockfüße am Boden bleiben; eingebaut sind die vier Konsolrosetten an der Badwand gelenkig befestigt, und der leere Bock steht auf seinen Füßen daneben."),
    T(f"The solver treats the whole asset as one rigid body, so in the lifted and reinstalled poses the {M_RACK:.1f} kg rack and the radiator share the sling, the wall pins and the rack feet in proportion to stiffness, not as two separate bodies; the rack is heavier than the radiator, so read the sling and wall reactions as those of the whole asset.",
      f"Der Löser behandelt das ganze Asset als einen starren Körper, daher teilen sich in der gehobenen und den eingebauten Posen der {M_RACK:.1f} kg schwere Bock und der Heizkörper Schlinge, Wandgelenke und Bockfüße nach Steifigkeit, nicht als zwei getrennte Körper; der Bock ist schwerer als der Heizkörper, die Reaktionen von Schlinge und Wand gelten daher für das ganze Asset."),
    T(f"Masses: uprights {UP_KGM:.3f} kg/m (D-section 35 x 30 x 1.2 mm), rails {RAIL_KGM:.3f} kg/m (22 x 1.2 mm, EN 10305-3), valve body 0.34 kg, plugs 0.12 kg, brackets 4 x 0.11 kg; rack {RACK_TUBE_M:.2f} m of SHS 40 x 40 x 2 at 2.31 kg/m (EN 10219-2), sill blocks 450 kg/m3 (C24), EPDM 1150 kg/m3. Reinstalled, the {WATER_L:.1f} l of water and two wet towels of 3 kg load the rails.",
      f"Massen: Sammelrohre {UP_KGM:.3f} kg/m (D-Profil 35 x 30 x 1,2 mm), Sprossen {RAIL_KGM:.3f} kg/m (22 x 1,2 mm, EN 10305-3), Ventilkörper 0,34 kg, Stopfen 0,12 kg, Konsolen 4 x 0,11 kg; Bock {RACK_TUBE_M:.2f} m MSH 40 x 40 x 2 zu 2,31 kg/m (EN 10219-2), Auflageklötze 450 kg/m3 (C24), EPDM 1150 kg/m3. Eingebaut belasten die {WATER_L:.1f} l Wasser und zwei nasse Handtücher von 3 kg die Sprossen."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(
        ("Side elevation from the right, 18 mm long and 30 mm high on paper: the right A-frame end frame with its two inclined legs, foot and ridge, the upper rail end with its EPDM wedge and the sill block with its strip, and the radiator seen edge-on leaning 10 degrees: the right D-section upright with its two brackets, top and blanking plugs and, below the left upright, the valve body with its outlet, compression nut and the sawn copper stub. Front, 17 x 30 mm: the ladder of eighteen rails in three groups between the two uprights, brackets, plugs, valve body with bonnet and white cap, in front of the rack rails, strips, sills and legs. Plan, 18 x 17 mm: ridge, legs, feet and cross tubes, the two D-section uprights, the top rail and the brackets.",
         "Seitenansicht von rechts, auf Papier 18 mm lang und 30 mm hoch: der rechte Bockrahmen mit seinen zwei geneigten Beinen, Fuß und First, das Traversenende mit seinem EPDM-Keil und der Auflageklotz mit seinem Streifen, und der Heizkörper hochkant um 10 Grad angelehnt: das rechte D-Sammelrohr mit seinen zwei Konsolen, Kopf- und Blindstopfen und unter dem linken Sammelrohr der Ventilkörper mit Abgang, Klemmringmutter und dem abgesägten Kupferstummel. Vorderansicht, 17 x 30 mm: die Leiter aus achtzehn Sprossen in drei Gruppen zwischen den zwei Sammelrohren, Konsolen, Stopfen, Ventilkörper mit Oberteil und weißer Kappe vor den Traversen, Streifen, Klötzen und Beinen des Bocks. Grundriss, 18 x 17 mm: First, Beine, Füße und Quertraversen, die zwei D-Sammelrohre, die oberste Sprosse und die Konsolen."),
        ("Every part at its true size: the D section of the uprights, the 22 mm rails, the 16 mm stand-off pins and 50 mm rosettes, the 15 mm copper stub, the 24 mm bonnet and 30 mm cap, the 40 mm rack tubes and the 15 mm EPDM strips; texture: chrome highlights, dull wear spots on rails and uprights, the cut face of the copper stub, the ribs of the edge strip, the sill grain and the welds of the feet.",
         "Jedes Teil in wahrer Größe: das D-Profil der Sammelrohre, die 22 mm Sprossen, die 16 mm Abstandsbolzen und 50 mm Rosetten, der 15 mm Kupferstummel, das 24 mm Oberteil und die 30 mm Kappe, die 40 mm Bockrohre und die 15 mm EPDM-Streifen; Textur: Chromlichter, matte Abriebstellen an Sprossen und Sammelrohren, die Schnittfläche des Kupferstummels, die Rippen des Kantenschutzes, die Holzmaserung des Klotzes und die Schweißnähte der Füße."),
        ("The left upright, the left brackets and the left end frame lie behind the right ones in the side elevation; the rails, hidden there behind the upright, show in front, and the top rail stands for all eighteen in plan.",
         "Das linke Sammelrohr, die linken Konsolen und der linke Bockrahmen liegen in der Seitenansicht hinter den rechten; die dort hinter dem Sammelrohr verborgenen Sprossen erscheinen in der Vorderansicht, und die oberste Sprosse steht im Grundriss für alle achtzehn."),
        ("The threads, the bleed screw of 4 mm and the screw holes of the rosettes are not modelled; the removed thermostat head is not part of the asset.",
         "Gewinde, die 4 mm Entlüftungsschraube und die Schraubenlöcher der Rosetten sind nicht modelliert; der abgenommene Thermostatkopf gehört nicht zum Asset."),
        ("The 1.2 mm tube walls are the outline of the solid tubes; the 10 mm plugs are 0.2 mm and the 15 mm copper stub 0.3 mm on paper.",
         "Die 1,2 mm Rohrwände sind die Umrisse der vollen Rohre; die 10 mm Stopfen messen 0,2 mm und der 15 mm Kupferstummel 0,3 mm auf Papier."),
        ("The sawn end of the front sill block carries a 45 degree end-grain hatch in the side elevation.", "Das gesägte Ende des vorderen Auflageklotzes trägt in der Seitenansicht eine Hirnholzschraffur unter 45 Grad."),
        ("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; tones: chrome 0.74 to 0.86, highlights 0.98, wear spots 0.55, rack 0.45, EPDM 0.15, timber 0.62, copper 0.5, cap 0.95.",
         "Körperstift 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Töne: Chrom 0,74 bis 0,86, Lichter 0,98, Abriebstellen 0,55, Bock 0,45, EPDM 0,15, Holz 0,62, Kupfer 0,5, Kappe 0,95."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    100: D(
        ("Side elevation 9 x 15 mm, front 8.4 x 15 mm, plan 9 x 8.4 mm: the A-frame with rails, strips and sills, the leaning ladder with its uprights, rails, brackets and valve.",
         "Seitenansicht 9 x 15 mm, Vorderansicht 8,4 x 15 mm, Grundriss 9 x 8,4 mm: der A-Bock mit Traversen, Streifen und Klötzen, die angelehnte Leiter mit Sammelrohren, Sprossen, Konsolen und Ventil."),
        ("The D section of the uprights, all eighteen 22 mm rails, the 40 mm rack tubes, the 15 mm strips, each bracket as one 50 x 20 x 45 mm block, the valve as body, outlet and bonnet.",
         "Das D-Profil der Sammelrohre, alle achtzehn 22 mm Sprossen, die 40 mm Bockrohre, die 15 mm Streifen, jede Konsole als ein Block 50 x 20 x 45 mm, das Ventil als Körper, Abgang und Oberteil."),
        ("Collar, pin and rosette of each bracket are one block; tail nut and valve body are one cylinder, outlet, compression nut and copper stub one, bonnet and cap one; the 10 mm plugs are taken into the uprights.",
         "Schelle, Bolzen und Rosette jeder Konsole sind ein Block; Überwurfmutter und Ventilkörper ein Zylinder, Abgang, Klemmringmutter und Kupferstummel einer, Oberteil und Kappe einer; die 10 mm Stopfen gehen in den Sammelrohren auf."),
        ("Blanking plug, wear spots, ribs, grain, welds and the hatch.", "Blindstopfen, Abriebstellen, Rippen, Maserung, Schweißnähte und Schraffur."),
        ("The 22 mm rails are 0.22 mm on paper, the 15 mm strips 0.15 mm.", "Die 22 mm Sprossen messen 0,22 mm auf Papier, die 15 mm Streifen 0,15 mm."),
        ("None.", "Keine."),
        ("Body pen 0.25 mm, detail 0.13 mm; the tones of 1:50.", "Körperstift 0,25 mm, Details 0,13 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    200: D(
        ("Side elevation 4.5 x 7.5 mm, front 4.2 x 7.5 mm, plan 4.5 x 4.2 mm: the A outline of the rack with its rails and sills, the leaning ladder of two uprights and eighteen rails, the brackets and the valve as blocks.",
         "Seitenansicht 4,5 x 7,5 mm, Vorderansicht 4,2 x 7,5 mm, Grundriss 4,5 x 4,2 mm: der A-Umriss des Bocks mit Traversen und Klötzen, die angelehnte Leiter aus zwei Sammelrohren und achtzehn Sprossen, Konsolen und Ventil als Blöcke."),
        ("Uprights and rack tubes at their true sections, the rails as slender 22 mm tubes so the ladder reads, brackets as 50 x 30 x 45 mm blocks, the valve as one block.",
         "Sammelrohre und Bockrohre in ihren wahren Querschnitten, die Sprossen als schlanke 22-mm-Rohre, damit die Leiter lesbar bleibt, Konsolen als Blöcke 50 x 30 x 45 mm, das Ventil als ein Block."),
        ("Rail and EPDM strip of each face are one wedge, sill block and strip one block; the valve with its outlet, stub and bonnet is one block.",
         "Traverse und EPDM-Streifen jeder Seite sind ein Keil, Auflageklotz und Streifen ein Block; das Ventil mit Abgang, Stummel und Oberteil ist ein Block."),
        ("Plugs, cap, every seam and texture.", "Stopfen, Kappe, alle Fugen und Texturen."),
        ("The rails stay at their true 22 mm (0.11 mm on paper) as slender tubes; nothing else is below 30 mm.", "Die Sprossen bleiben als schlanke Rohre bei wahren 22 mm (0,11 mm auf Papier); sonst ist nichts unter 30 mm."),
        ("None.", "Keine."),
        ("One body pen of 0.18 mm; the tones of 1:50.", "Ein Körperstift von 0,18 mm; die Töne von 1:50."),
        ("The merged wedges and sill blocks list both their regions.", "Die zusammengefassten Keile und Klötze führen beide Regionen."),
    ),
}

DECISIONS_3D = {
    50: [T("Body: every part of the 1:50 drawing as a solid: the D-section uprights as extrusions of their true section, the eighteen rails, plugs, valve body, nut, outlet, compression nut, copper stub, bonnet and cap as revolves, the bracket collars as boxes with revolved pins and rosettes; the rack of SHS 40 tubes with the inclined legs as extrusions, the EPDM wedges as extrusions whose fronts lie in the plane of the leaning uprights, sill blocks and strips as boxes.",
           "Körper: jedes Teil der Zeichnung 1:50 als Körper: die D-Sammelrohre als Extrusionen ihres wahren Querschnitts, die achtzehn Sprossen, Stopfen, Ventilkörper, Mutter, Abgang, Klemmringmutter, Kupferstummel, Oberteil und Kappe als Drehkörper, die Konsolschellen als Quader mit gedrehten Bolzen und Rosetten; der Bock aus MSH-40-Rohren mit den geneigten Beinen als Extrusionen, die EPDM-Keile als Extrusionen, deren Vorderseiten in der Ebene der angelehnten Sammelrohre liegen, Auflageklötze und Streifen als Quader."),
         T("Thermal: every region by reference to its body solids and a plume patch on the top rail. Structure: uprights, rails and every rack tube, strip and sill as members, the sill bearing of the bottom rail and the two lean contacts of the uprights as joints, three mass markers.",
           "Wärme: jede Region als Verweis auf ihre Körper und ein Fahnenfeld auf der obersten Sprosse. Tragwerk: Sammelrohre, Sprossen und jedes Bockrohr, jeder Streifen und Klotz als Stäbe, das Klotzauflager der untersten Sprosse und die zwei Anlehnkontakte der Sammelrohre als Gelenke, drei Massenmarken.")],
    100: [T("Body for the 15 mm feature: each bracket one block, the valve as body, outlet and bonnet cylinders, the plugs taken into the uprights, the blanking plug left out; uprights, rails, rack tubes and 15 mm strips keep their true sections.",
            "Körper für das Merkmal von 15 mm: jede Konsole ein Block, das Ventil als Zylinder für Körper, Abgang und Oberteil, die Stopfen in den Sammelrohren aufgegangen, der Blindstopfen entfällt; Sammelrohre, Sprossen, Bockrohre und 15-mm-Streifen behalten ihre wahren Querschnitte."),
          T("Thermal and structure as at 1:50 on the parts of this scale, joints and markers of the 15 mm feature.",
            "Wärme und Tragwerk wie bei 1:50 auf den Teilen dieses Maßstabs, Gelenke und Marken für das Merkmal von 15 mm.")],
    200: [T("Body for the 30 mm feature: rail and strip of each face one wedge, sill and strip one block, each bracket a 50 x 30 x 45 mm block, the valve one block; uprights and rack tubes at their true sections, the rails slender at 22 mm.",
            "Körper für das Merkmal von 30 mm: Traverse und Streifen jeder Seite ein Keil, Klotz und Streifen ein Block, jede Konsole ein Block 50 x 30 x 45 mm, das Ventil ein Block; Sammelrohre und Bockrohre in wahren Querschnitten, die Sprossen schlank bei 22 mm."),
          T("Thermal: the regions by reference, the merged wedges under the edge strips and the merged sills under the sills; structure: members, joints of 30 mm and the mass markers.",
            "Wärme: die Regionen als Verweise, die zusammengefassten Keile unter dem Kantenschutz und die zusammengefassten Klötze unter den Klötzen; Tragwerk: Stäbe, Gelenke von 30 mm und die Massenmarken.")],
}
