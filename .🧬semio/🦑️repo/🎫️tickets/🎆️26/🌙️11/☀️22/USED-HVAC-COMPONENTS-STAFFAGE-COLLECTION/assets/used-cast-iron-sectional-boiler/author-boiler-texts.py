"""📜 Texts, materials, thermal model, structure and decisions of the used cast-iron sectional boiler, executed into the namespace of author-boiler.py."""

AREA = {"jacket": 2.77, "block": 6.40, "doors": 0.50, "blanking-plate": 0.12, "stubs": 0.035, "flue-spigot": 0.125, "control-panel": 0.38, "steelwork": 0.39, "bearers": 0.84}

SUMMARY = T(
    f"A dismantled floor-standing cast-iron sectional boiler of the 1970s as a component exchange offers it: six cast sections (front, four middle, rear; 0.74 m block) for 45 kW output at 50 kW oil or gas heat input, in a faded orange-enamelled 1 mm sheet-steel jacket of 0.80 x 0.80 x 0.62 m with 25 mm mineral wool, a sloped control panel whose dials are pulled, the cast burner door with the burner removed and its bore closed by a bolted 12 mm blanking plate, the lower cleaning cover, a sawn 150 mm flue spigot at the rear, flow and return stubs cut and capped, a drain cock, two M16 eye bolts and two U 50 base channels; drained, {M_TOTAL:.0f} kg with its two 100 x 100 mm softwood bearers. Poses: stored on the bearers, lifted on a chain sling with the bearers left behind, and reinstalled on a boiler room floor, refilled and firing at 90/70 degC with the flue gas leaving the spigot.",
    f"Ein ausgebauter bodenstehender Guss-Gliederkessel der 1970er Jahre, wie ihn eine Bauteilbörse anbietet: sechs Gussglieder (Vorder-, vier Mittel-, Hinterglied; Block 0,74 m) für 45 kW Leistung bei 50 kW Öl- oder Gas-Wärmebelastung, in einem verblichen orange emaillierten Stahlblechmantel 1 mm von 0,80 x 0,80 x 0,62 m mit 25 mm Mineralwolle, ein schräges Schaltfeld mit ausgebauten Reglern, die gusseiserne Brennertür mit abgebautem Brenner und einer angeschraubten Blindplatte 12 mm über der Öffnung, der untere Reinigungsdeckel, ein abgesägter Abgasstutzen 150 mm hinten, Vor- und Rücklaufstutzen abgetrennt und verschlossen, ein KFE-Hahn, zwei Ringschrauben M16 und zwei Grundschienen U 50; entleert, {M_TOTAL:.0f} kg mit seinen zwei Nadelholz-Kanthölzern 100 x 100 mm. Posen: auf den Kanthölzern gelagert, am Kettengehänge gehoben, wobei die Kanthölzer liegen bleiben, und wieder eingebaut auf dem Boden eines Heizraums, befüllt und mit 90/70 °C befeuert, das Abgas verlässt den Stutzen.")

PROVENANCE = {
    "author": "Semio staffage component authors (used HVAC wave, author 01)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "din-4702-1", "citation": "DIN 4702-1:1990 Heizkessel; Begriffe, Anforderungen, Prüfung, Kennzeichnung (nominal output and nominal heat input, boiler efficiency at nominal output, type plate data of oil and gas boilers)."},
        {"id": "din-en-303-1", "citation": "DIN EN 303-1:2017 Heating boilers with forced draught burners: terminology, general requirements, testing and marking (flue gas connection, burner mounting flange, water-side test pressure)."},
        {"id": "sectional-boiler-catalogues", "citation": "Catalogue and installation data of floor-standing cast-iron sectional boilers of the 1970s and 1980s for forced-draught oil or gas burners (Buderus Lollar G 105 / G 205 / G 115 family, Viessmann and Strebel sectional boilers): sections of about 110 to 130 mm pitch joined by conical push nipples and four tie rods, 4 to 9 sections for 20 to 70 kW, about 6 to 7 l of water and 30 to 45 kg per section, burner door with refractory lining on the front section, cleaning cover below, flue outlet 130 to 180 mm at the rear, flow and return R 1 1/4 to R 1 1/2 at the top and rear, enamelled sheet-steel jacket with 20 to 30 mm mineral wool, boiler control panel on top; the six-section 45 kW boiler of this asset is a typical member of the family, not one model (authored-estimate)."},
        {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch für Heizung und Klimatechnik, 79th ed., ITM 2019 (heat generators: cast-iron sectional boilers, standby losses, flue gas temperatures of older boilers 180 to 250 degC, design temperatures 90/70 degC of the period)."},
        {"id": "din-1026-1", "citation": "DIN 1026-1:2009 Hot rolled steel channels, taper flange channels: U 50 with h 50 mm, b 38 mm, s 5 mm, t 7 mm, 5.59 kg/m, A 7.12 cm2."},
        {"id": "din-en-1561", "citation": "DIN EN 1561:2012 Founding, grey cast irons (EN-GJL-150/200: density about 7200 kg/m3, E about 100 GPa, tensile strength 150 to 200 MPa)."},
        {"id": "din-580", "citation": "DIN 580:2010 Lifting eye bolts: M16 eye bolt, eye inner diameter 35 mm, working load limit 7 kN in axial pull per bolt."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3 (450 kg/m3 used for the weathered, air-dry bearers), E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (mineral wool lambda 0.035 to 0.045 W/mK, softwood 0.13 W/mK, steel 50 W/mK; aged compressed wool taken at 0.045 W/mK)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: oxidised cast iron 0.8, enamel paint 0.9, rusted steel 0.8, tarnished brass 0.3, aluminium 0.2)."},
        {"id": "turns-2012", "citation": "Turns, S. R. (2012). An Introduction to Combustion, 3rd ed., McGraw-Hill (light fuel oil: lower heating value 42.7 MJ/kg, stoichiometric air 14.3 kg/kg, water vapour from 13 % hydrogen)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."},
    ],
    "notes": T(
        "Dimensions, section count, water content and masses are typical values of the 1970s cast-iron sectional boiler family (authored estimates from the cited catalogues, nothing is measured); the steel parts, the U 50 channels and the timber bearers follow DIN 1026, DIN 580 and EN 338. The burner is not part of the asset: the reinstalled poses assume a replacement forced-draught burner of 18 kg bolted on the burner door in place of the blanking plate, carried as an equipment load.",
        "Maße, Gliederzahl, Wasserinhalt und Massen sind typische Werte der Guss-Gliederkessel der 1970er Jahre (Schätzwerte nach den zitierten Katalogen, nichts ist gemessen); die Stahlteile, die U-50-Schienen und die Kanthölzer folgen DIN 1026, DIN 580 und EN 338. Der Brenner gehört nicht zum Asset: die Einbauposen nehmen einen Ersatz-Gebläsebrenner von 18 kg an der Brennertür anstelle der Blindplatte an, der als Ausrüstungslast getragen wird."),
}

MATERIALS = [
    {"id": "cast-iron", "title": T("Grey cast iron of the sections and doors, oxidised", "Grauguss der Glieder und Türen, oxidiert"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 100000000000, "strengthPa": 150000000, "source": "literature", "reference": "din-en-1561"},
    {"id": "painted-steel", "title": T("Orange enamelled sheet steel 1 mm, faded and scratched", "Orange emailliertes Stahlblech 1 mm, verblichen und verkratzt"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "mineral-wool", "title": T("Mineral wool mat 25 mm behind the jacket, aged", "Mineralwollmatte 25 mm hinter dem Mantel, gealtert"), "emissivity": 0.9, "densityKgM3": 60, "conductivityWmK": 0.045, "specificHeatJKgK": 840, "source": "literature", "reference": "en-iso-10456"},
    {"id": "steel", "title": T("Black steel, rusty (channels, blanking plate, stubs, eye bolts)", "Schwarzstahl, angerostet (Schienen, Blindplatte, Stutzen, Ringschrauben)"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "flue-steel", "title": T("Sooted sheet steel of the flue spigot 2 mm", "Verrußtes Stahlblech des Abgasstutzens 2 mm"), "emissivity": 0.85, "solarAbsorptivity": 0.85, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2018"},
    {"id": "brass", "title": T("Tarnished brass drain cock", "Angelaufener Messing-KFE-Hahn"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": "softwood", "title": T("Weathered softwood bearers C24", "Verwitterte Nadelholz-Kanthölzer C24"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "aluminium", "title": T("Embossed aluminium type plate", "Geprägtes Aluminium-Typenschild"), "emissivity": 0.2, "solarAbsorptivity": 0.3, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
]

CIRCUIT = {"key": "heating", "rows": [{"v": 0.5, "supplyTempC": 60, "returnTempC": 50}, {"v": 1, "supplyTempC": 90, "returnTempC": 70}]}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, model, material, length, geometry, layers, extra, src="authored-estimate", ref="sectional-boiler-catalogues"):
    """🔥 One thermal region with its area from AREA and its heat capacity from its layers."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.2,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers), 1), "source": src, "reference": ref}
    out.update(extra)
    return out


WOOL_LAYER = layer("insulation", "Mineral wool mat 25 mm, aged", "Mineralwollmatte 25 mm, gealtert", "insulation", "mineral-wool", WOOL, 0.045, 60, 840)
SHEET_LAYER = layer("jacket", "Enamelled sheet steel 1 mm", "Emailliertes Stahlblech 1 mm", "metal", "painted-steel", SHEET, 50, 7850, 460, ref="incropera-2018")

THERMAL = {
    "environment": {"airTempC": 15, "meanRadiantTempC": 14, "relativeHumidity": 0.6, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "burner", "title": T("Replacement burner firing", "Ersatzbrenner in Betrieb"), "levels": [{"v": 0, "title": T("Off, burner removed or switched off", "Aus, Brenner abgebaut oder abgeschaltet")}, {"v": 1, "title": T("Firing at 50 kW heat input", "Betrieb mit 50 kW Wärmebelastung")}]},
        {"id": "heating", "title": T("Boiler water", "Kesselwasser"), "levels": [{"v": 0, "title": T("Drained or cold", "Entleert oder kalt")}, {"v": 0.5, "title": T("Reduced, 60/50 degC", "Abgesenkt, 60/50 °C")}, {"v": 1, "title": T("Design, 90/70 degC", "Auslegung, 90/70 °C")}]},
    ],
    "regions": [
        region("jacket", "Enamelled sheet-steel jacket with mineral wool", "Emaillierter Stahlblechmantel mit Mineralwolle", "surface", "hydronic", "painted-steel", 0.8, "vertical-plate", [WOOL_LAYER, SHEET_LAYER],
               {"hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.1), "ratedPowerW": 50000, "ratedPowerBasis": "fuel-input"}),
        region("block", "Six cast-iron sections with the water ways and the flue passes", "Sechs Gussglieder mit Wasserwegen und Heizgaszügen", "surface", "hydronic", "cast-iron", 0.73, "vertical-plate",
               [layer("wall", "Cast-iron section wall 8 mm", "Gussgliederwand 8 mm", "metal", "cast-iron", 0.008, 50, 7200, 460, ref="din-en-1561")],
               {"hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.0005), "hidden": T(
                   "The six sections (0.74 x 0.73 x 0.52 m) stand inside the jacket behind 25 mm of mineral wool and an air gap; no drawing shows them. Their area is the surface of the six sections as modelled, with the faces between neighbouring sections that the sealing cord and the push nipples leave free (6.4 m2); the fireside surface of the flue passes is not modelled.",
                   "Die sechs Glieder (0,74 x 0,73 x 0,52 m) stehen im Mantel hinter 25 mm Mineralwolle und einem Luftspalt; keine Zeichnung zeigt sie. Ihre Fläche ist die Oberfläche der sechs Glieder wie modelliert, mit den Flächen zwischen benachbarten Gliedern, die Dichtschnur und Stecknippel freilassen (6,4 m2); die feuerseitige Fläche der Heizgaszüge ist nicht modelliert.")}),
        region("doors", "Cast-iron burner door and cleaning cover", "Gusseiserne Brennertür und Reinigungsdeckel", "surface", "operating", "cast-iron", 0.36, "vertical-plate",
               [layer("casting", "Grey cast iron 12 mm", "Grauguss 12 mm", "metal", "cast-iron", 0.012, 50, 7200, 460, ref="din-en-1561")], {"operating": {"key": "burner", "heatW": 220}}),
        region("blanking-plate", "Bolted steel blanking plate over the burner bore", "Angeschraubte Stahl-Blindplatte über der Brenneröffnung", "surface", "operating", "steel", 0.24, "vertical-plate",
               [layer("plate", "Steel plate 12 mm", "Stahlplatte 12 mm", "metal", "steel", 0.012, 50, 7850, 460, ref="incropera-2018")], {"operating": {"key": "burner", "heatW": 60}}),
        region("stubs", "Cut and capped flow and return stubs and the drain cock", "Abgetrennte und verschlossene Vor- und Rücklaufstutzen und KFE-Hahn", "surface", "hydronic", "steel", 0.042, "cylinder",
               [layer("tube", "Steel tube 3.25 mm", "Stahlrohr 3,25 mm", "metal", "steel", 0.00325, 50, 7850, 460, ref="incropera-2018")], {"hydronic": dict(CIRCUIT, flowPosition=0.0, filmM2KW=0.002)}),
        region("flue-spigot", "Sawn flue spigot 150 mm", "Abgesägter Abgasstutzen 150 mm", "exhaust", "operating", "flue-steel", 0.15, "cylinder",
               [layer("pipe", "Sooted steel sheet 2 mm", "Verrußtes Stahlblech 2 mm", "metal", "flue-steel", 0.002, 50, 7850, 460, ref="incropera-2018")], {"operating": {"key": "burner", "heatW": 110}}),
        region("control-panel", "Control panel housing with pulled dials", "Schaltfeldgehäuse mit ausgebauten Reglern", "surface", "passive", "painted-steel", 0.2, "horizontal-plate-up", [SHEET_LAYER], {}),
        region("steelwork", "U 50 base channels and M16 eye bolts", "Grundschienen U 50 und Ringschrauben M16", "surface", "passive", "steel", 0.05, "horizontal-plate-down",
               [layer("steel", "Rolled steel 5 to 7 mm", "Walzstahl 5 bis 7 mm", "metal", "steel", 0.006, 50, 7850, 460, ref="din-1026-1")], {}),
        region("bearers", "Two squared softwood bearers", "Zwei Nadelholz-Kanthölzer", "surface", "passive", "softwood", 0.1, "horizontal-plate-up",
               [layer("timber", "Softwood, thermally active 20 mm", "Nadelholz, thermisch wirksam 20 mm", "timber", "softwood", 0.02, 0.13, 450, 1600, ref="en-338")], {"ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "literature", "reference": "en-338"}),
    ],
    "emitters": [
        {"id": "flue-gas", "kind": "flue", "title": T("Flue gas leaving the sawn end of the spigot", "Abgas am abgesägten Ende des Stutzens"), "anchor": "flue-outlet", "directionDeg": 180, "diameterM": 0.146, "operatingKey": "burner",
         "volumeFlowM3S": 0.0296, "temperatureRiseK": 185, "vapourKgPerKg": 0.062, "source": "authored-estimate", "reference": "turns-2012"},
        {"id": "jacket-plume", "kind": "plume", "title": T("Free-convection plume over the warm jacket top", "Freie Konvektionsfahne über der warmen Manteloberseite"), "anchor": "jacket-top", "directionDeg": 90, "diameterM": 0.5, "region": "jacket", "source": "authored-estimate", "reference": "recknagel-2019"},
    ],
    "assumptions": [
        T("Stored and lifted, the boiler is drained, cold and at the 15 degC of the hall; reinstalled it is refilled with 39 l and its water follows the heating circuit (90/70 degC at design, 60/50 degC reduced); the jacket, the hidden block and the stubs take that water through their layers and films, the doors, the blanking plate and the flue spigot are heated by the fire of the replacement burner (released heat 220, 60 and 110 W at full fire).",
          "Gelagert und gehoben ist der Kessel entleert, kalt und hat die 15 °C der Halle; eingebaut ist er mit 39 l befüllt und sein Wasser folgt dem Heizkreis (90/70 °C Auslegung, 60/50 °C abgesenkt); Mantel, verborgener Block und Stutzen übernehmen dieses Wasser durch ihre Schichten und Filme, Türen, Blindplatte und Abgasstutzen werden vom Feuer des Ersatzbrenners geheizt (abgegebene Wärme 220, 60 und 110 W bei Volllast)."),
        T("The nameplate of 50 kW fuel input (45 kW output, 90 % at full load, DIN 4702-1) is stated on the jacket region; the flue gas of light oil at an excess air of 1.25 leaves at about 200 degC with 22 g/s (0.030 m3/s) and 6.2 % water vapour.",
          "Das Typenschild von 50 kW Wärmebelastung (45 kW Leistung, 90 % bei Volllast, DIN 4702-1) steht auf der Mantelregion; das Abgas von Heizöl EL bei einer Luftzahl von 1,25 verlässt den Stutzen mit etwa 200 °C, 22 g/s (0,030 m3/s) und 6,2 % Wasserdampf."),
        T("Weathered and enamelled surfaces carry the emissivities of oxidised, painted and rusted metal (Incropera); the faded orange enamel absorbs 0.6 of the solar radiation instead of about 0.45 when new.",
          "Verwitterte und emaillierte Oberflächen haben die Emissionsgrade von oxidiertem, lackiertem und angerostetem Metall (Incropera); das verblichene orange Email absorbiert 0,6 der Sonnenstrahlung statt etwa 0,45 im Neuzustand."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the {M_BOILER:.0f} kg boiler stands with its two U 50 base channels on two 100 x 100 mm softwood bearers that rest with their four ends on the floor (contacts, friction 0.5); lifted, the two M16 eye bolts (DIN 580, 7 kN each in axial pull) hang it on the chain sling (pins); reinstalled, the four bearing faces of the channels stand on the boiler room floor (contacts).",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert steht der {M_BOILER:.0f} kg schwere Kessel mit seinen zwei Grundschienen U 50 auf zwei Nadelholz-Kanthölzern 100 x 100 mm, die mit ihren vier Enden auf dem Boden liegen (Kontakte, Reibung 0,5); gehoben hängen ihn die zwei Ringschrauben M16 (DIN 580, je 7 kN axial) ins Kettengehänge (Gelenke); eingebaut stehen die vier Auflagerflächen der Schienen auf dem Heizraumboden (Kontakte)."),
    T(f"The solver treats the whole asset as one rigid body: in the lifted pose the {M_BEARER:.1f} kg of bearers left on the floor are counted in the sling reactions, and in the reinstalled poses they are carried by the boiler feet ({100 * M_BEARER / M_TOTAL:.1f} % of the asset weight).",
      f"Der Löser behandelt das ganze Asset als einen starren Körper: in der gehobenen Pose gehen die {M_BEARER:.1f} kg der am Boden liegenden Kanthölzer in die Reaktionen des Gehänges ein, in den Einbauposen tragen die Kesselfüße sie ({100 * M_BEARER / M_TOTAL:.1f} % des Assetgewichts)."),
    T("Reinstalled, the sections carry 39 l of boiler water (383 N), the replacement burner of 18 kg cantilevered 0.17 m in front of the door (177 N, 30 Nm) and the flow and return pipework (0.12 kN each); the cast-iron block with its four tie rods and push nipples is one rigid member on the two channels.",
      "Eingebaut tragen die Glieder 39 l Kesselwasser (383 N), den 0,17 m vor der Tür auskragenden Ersatzbrenner von 18 kg (177 N, 30 Nm) und die Vor- und Rücklaufleitungen (je 0,12 kN); der Gussblock mit seinen vier Zugankern und Stecknippeln ist ein starres Bauteil auf den zwei Schienen."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(
        ("Side elevation from the right, 20.2 mm long and 21 mm high on paper: the orange jacket of 0.80 x 0.80 m on its U 50 channel, the sloped control panel on top front, the cast burner door with the blanking plate and its bolt heads, the cleaning cover and the drain cock on the front, the sawn flue spigot and the capped return stub at the rear, the capped flow stub and two eye bolts on top, all on the 1.0 m bearer. Front, 12.4 x 21 mm: jacket, panel with the two empty dial holes, door with hinge lugs, plate with four bolts and sight boss, cleaning cover and drain cock over the two channel ends and the two sawn bearer ends. Plan, 20.2 x 12.4 mm: jacket top, panel, door and plate, spigot, return stub, flow cap, eye bolts and the bearer ends.",
         "Seitenansicht von rechts, auf Papier 20,2 mm lang und 21 mm hoch: der orange Mantel von 0,80 x 0,80 m auf seiner Schiene U 50, das schräge Schaltfeld oben vorn, die gusseiserne Brennertür mit Blindplatte und Schraubenköpfen, Reinigungsdeckel und KFE-Hahn vorn, der abgesägte Abgasstutzen und der verschlossene Rücklaufstutzen hinten, der verschlossene Vorlaufstutzen und zwei Ringschrauben oben, alles auf dem 1,0 m langen Kantholz. Vorderansicht, 12,4 x 21 mm: Mantel, Schaltfeld mit den zwei leeren Reglerlöchern, Tür mit Scharnierlappen, Platte mit vier Schrauben und Schauglasnabe, Reinigungsdeckel und KFE-Hahn über den zwei Schienenenden und den zwei gesägten Kantholzenden. Grundriss, 20,2 x 12,4 mm: Manteloberseite, Schaltfeld, Tür und Platte, Stutzen, Rücklaufstutzen, Vorlaufkappe, Ringschrauben und die Kantholzenden."),
        ("Every part of the dismantled boiler at its true size: the U 50 profile of the channels with its 7 mm flanges, the 12 mm blanking plate, the 19 mm hexagon bolt heads, the 42 mm stubs and their 52 mm caps, the 2 mm flue spigot wall as a ring in front view, the eye bolts with their bore, the type plate, the jacket seams and four rust streaks at the bottom edge.",
         "Jedes Teil des ausgebauten Kessels in wahrer Größe: das U-50-Profil der Schienen mit seinen 7 mm Flanschen, die 12 mm Blindplatte, die 19 mm Sechskantköpfe, die 42 mm Stutzen und ihre 52 mm Kappen, die 2 mm Wand des Abgasstutzens als Ring, die Ringschrauben mit ihrer Öffnung, das Typenschild, die Mantelfugen und vier Roststreifen an der Unterkante."),
        ("The far bearer and the far channel are hidden behind the near ones in the side elevation; the hinge lugs, hidden behind the door there, show in front and plan.",
         "Das hintere Kantholz und die hintere Schiene liegen in der Seitenansicht hinter den vorderen; die dort hinter der Tür verborgenen Scharnierlappen erscheinen in Vorderansicht und Grundriss."),
        ("The cast-iron sections, the mineral wool and the tie rods inside the jacket are not drawn: the jacket hides them.",
         "Gussglieder, Mineralwolle und Zuganker im Mantel sind nicht gezeichnet: der Mantel verbirgt sie."),
        ("The 1 mm jacket edge and the 1.5 mm type plate are lines; the 12 mm plate is 0.24 mm, the 8 mm bolt heads 0.16 mm on paper.",
         "Der 1 mm Mantelrand und das 1,5 mm Typenschild sind Linien; die 12 mm Platte misst 0,24 mm, die 8 mm Schraubenköpfe 0,16 mm auf Papier."),
        ("The sawn ends of the bearers carry a 45 degree end-grain hatch in the front view.", "Die gesägten Kantholzenden tragen in der Vorderansicht eine Hirnholzschraffur unter 45 Grad."),
        ("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; tones: jacket 0.74, panel 0.5, bearers 0.62, cast iron 0.3, steel 0.2 to 0.4, dial holes 0.08.",
         "Körperstift 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Töne: Mantel 0,74, Schaltfeld 0,5, Kanthölzer 0,62, Guss 0,3, Stahl 0,2 bis 0,4, Reglerlöcher 0,08."),
        ("Every shape names its thermal region; the hidden block has none.", "Jede Form nennt ihre Wärmeregion; der verborgene Block hat keine."),
    ),
    100: D(
        ("Side elevation 10.1 x 10.5 mm, front 6.2 x 10.5 mm, plan 10.1 x 6.2 mm: jacket, sloped panel, door with plate, cleaning cover, spigot, stubs with caps and eye bolts on the bearers.",
         "Seitenansicht 10,1 x 10,5 mm, Vorderansicht 6,2 x 10,5 mm, Grundriss 10,1 x 6,2 mm: Mantel, schräges Schaltfeld, Tür mit Platte, Reinigungsdeckel, Stutzen, Anschlüsse mit Kappen und Ringschrauben auf den Kanthölzern."),
        ("The 52 mm caps, the 150 mm spigot ring, the door with its 20 mm plate and the two hinge lugs as one bar, the channels as 50 x 38 mm bars, the eye bolts as 45 mm discs.",
         "Die 52 mm Kappen, der Ring des 150 mm Stutzens, die Tür mit ihrer 20 mm Platte und die zwei Scharnierlappen als ein Balken, die Schienen als Balken 50 x 38 mm, die Ringschrauben als Scheiben von 45 mm."),
        ("Plate and bolt heads are one 20 mm plate; the two hinge lugs are one bar; the U profile of the channels is a solid bar.",
         "Platte und Schraubenköpfe sind eine Platte von 20 mm; die zwei Scharnierlappen sind ein Balken; das U-Profil der Schienen ist ein voller Balken."),
        ("Drain cock, sight boss, type plate, dial holes, rust streaks and threads.", "KFE-Hahn, Schauglasnabe, Typenschild, Reglerlöcher, Roststreifen und Gewinde."),
        ("The 20 mm plate is 0.2 mm on paper; the 42 mm stubs 0.42 mm.", "Die 20 mm Platte misst 0,2 mm auf Papier; die 42 mm Stutzen 0,42 mm."),
        ("None.", "Keine."),
        ("Body pen 0.25 mm, detail 0.13 mm; the tones of 1:50.", "Körperstift 0,25 mm, Details 0,13 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    200: D(
        ("Side elevation 5.1 x 5.3 mm, front 3.1 x 5.3 mm, plan 5.1 x 3.1 mm: a jacket block with the panel box on top, door and plate as two steps in front, spigot and stubs behind, one base band on the bearers.",
         "Seitenansicht 5,1 x 5,3 mm, Vorderansicht 3,1 x 5,3 mm, Grundriss 5,1 x 3,1 mm: ein Mantelblock mit dem Schaltfeldkasten oben, Tür und Platte als zwei Stufen vorn, Stutzen und Anschlüsse hinten, ein Sockelband auf den Kanthölzern."),
        ("Jacket, panel, door, plate, cleaning cover, spigot, the flow and return caps and the base, all at least 30 mm.", "Mantel, Schaltfeld, Tür, Platte, Reinigungsdeckel, Stutzen, die Vor- und Rücklaufkappen und der Sockel, alles mindestens 30 mm."),
        ("The two channels are one base band 0.76 x 0.05 x 0.48 m; each stub with its cap is one cylinder of the cap diameter; the sloped panel is a box; the door is 45 mm deep and the plate with its bolts 30 mm.",
         "Die zwei Schienen sind ein Sockelband 0,76 x 0,05 x 0,48 m; jeder Stutzen ist mit seiner Kappe ein Zylinder vom Kappendurchmesser; das schräge Schaltfeld ist ein Kasten; die Tür ist 45 mm tief, die Platte mit ihren Schrauben 30 mm."),
        ("Eye bolts, hinge lugs, drain cock, sight boss, every seam and texture; the eye bolts and the channels share the steelwork region of the base and the jacket.",
         "Ringschrauben, Scharnierlappen, KFE-Hahn, Schauglasnabe, alle Fugen und Texturen; Ringschrauben und Schienen teilen die Stahlregion von Sockel und Mantel."),
        ("No part below 30 mm (0.15 mm on paper) remains.", "Kein Teil unter 30 mm (0,15 mm auf Papier) bleibt."),
        ("None.", "Keine."),
        ("One body pen of 0.18 mm; the tones of 1:50.", "Ein Körperstift von 0,18 mm; die Töne von 1:50."),
        ("The jacket shape lists jacket and steelwork as merged regions.", "Die Mantelform führt Mantel und Stahlteile als zusammengeführte Regionen."),
    ),
}

DECISIONS_3D = {
    50: [T("Body: every part of the 1:50 drawing as a solid: the U 50 channels as true DIN 1026 extrusions, the jacket as one box with 8 mm rounded edges, the sloped panel as an extrusion, the door, plate, cleaning cover and lugs as boxes, bolt heads, sight boss, drain cock, stubs and caps as revolves, the flue spigot as a 2 mm tube, the eye bolts as rings with their bore, the bearers with 4 mm arrises and the type plate as a declared texture sheet.",
           "Körper: jedes Teil der Zeichnung 1:50 als Körper: die Schienen U 50 als echte Strangprofile nach DIN 1026, der Mantel als Kasten mit 8 mm gerundeten Kanten, das schräge Schaltfeld als Extrusion, Tür, Platte, Reinigungsdeckel und Lappen als Quader, Schraubenköpfe, Schauglasnabe, KFE-Hahn, Stutzen und Kappen als Drehkörper, der Abgasstutzen als Rohr mit 2 mm Wand, die Ringschrauben als Ringe mit Öffnung, die Kanthölzer mit 4 mm Fasen und das Typenschild als deklariertes Texturblech."),
         T("Thermal: the jacket as two nested shells of five plates each (1 mm enamelled sheet outside, 25 mm mineral wool inside), the six cast-iron sections as the hidden block, every other region by reference to its body solids, and the two outlets: a disc in the open end of the spigot and a patch on the jacket top. Structure: bearers and channels as members, the front, four middle and rear sections and the four slender tie rods as the boiler member, four bearing pads as joints, two mass markers.",
           "Wärme: der Mantel als zwei verschachtelte Schalen aus je fünf Platten (außen 1 mm emailliertes Blech, innen 25 mm Mineralwolle), die sechs Gussglieder als verborgener Block, jede andere Region als Verweis auf ihre Körper, und die zwei Auslässe: eine Scheibe im offenen Ende des Stutzens und ein Feld auf der Manteloberseite. Tragwerk: Kanthölzer und Schienen als Stäbe, Vorder-, vier Mittel- und Hinterglied und die vier schlanken Zuganker als Kesselbauteil, vier Auflagerpolster als Gelenke, zwei Massenmarken.")],
    100: [T("Body for the 15 mm feature: the channels are 50 x 38 mm bars, plate and bolt heads one 20 mm plate, the two hinge lugs one bar, the eye bolts 45 mm discs 16 mm thick without bore; drain cock, sight boss and type plate are left out; spigot and stubs keep their true sections.",
            "Körper für das Merkmal von 15 mm: die Schienen sind Balken 50 x 38 mm, Platte und Schraubenköpfe eine Platte von 20 mm, die zwei Scharnierlappen ein Balken, die Ringschrauben Scheiben von 45 mm und 16 mm Dicke ohne Öffnung; KFE-Hahn, Schauglasnabe und Typenschild entfallen; Stutzen und Anschlüsse behalten ihre wahren Querschnitte."),
          T("Thermal: the jacket is its body box with both layers merged, the block one hidden box; structure: bearers, channels and the block as one member box, bearing pads and mass markers.",
            "Wärme: der Mantel ist sein Körperkasten mit beiden Schichten zusammengefasst, der Block ein verborgener Quader; Tragwerk: Kanthölzer, Schienen und der Block als ein Bauteilquader, Auflagerpolster und Massenmarken.")],
    200: [T("Body for the 30 mm feature: one base band for the two channels, the panel as a box, the door 45 mm and the plate 30 mm deep, the spigot a solid cylinder, each stub with its cap one 52 mm cylinder; eye bolts and hinge lugs are left out.",
            "Körper für das Merkmal von 30 mm: ein Sockelband für die zwei Schienen, das Schaltfeld als Kasten, die Tür 45 mm und die Platte 30 mm tief, der Stutzen ein voller Zylinder, jeder Stutzen mit Kappe ein Zylinder von 52 mm; Ringschrauben und Scharnierlappen entfallen."),
          T("Thermal: jacket box, hidden block, the other regions by reference, the steelwork region on the base band; structure: bearers, base band and block as members, bearing pads of 30 mm, mass markers.",
            "Wärme: Mantelkasten, verborgener Block, die anderen Regionen als Verweise, die Stahlregion auf dem Sockelband; Tragwerk: Kanthölzer, Sockelband und Block als Stäbe, Auflagerpolster von 30 mm, Massenmarken.")],
}
