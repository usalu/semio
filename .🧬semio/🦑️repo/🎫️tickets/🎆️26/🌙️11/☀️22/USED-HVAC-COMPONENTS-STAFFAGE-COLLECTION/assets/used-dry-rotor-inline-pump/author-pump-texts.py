"""📜 Texts, materials, thermal model, structure and decisions of the used dry-rotor inline pump, executed into the namespace of author-pump.py."""

AREA = {"pallet": 1.92, "casing": 0.49, "lantern": 0.22, "motor": 0.28, "terminal-box": 0.072, "fan-cowl": 0.146, "blanking-plates": 0.156}

SUMMARY = T(
    f"A dismantled dry-rotor inline circulator DN 80 PN 16 of a heating riser as a component exchange offers it: a grey cast-iron volute housing with 360 mm port-to-port length and two EN 1092-1 flanges of 200 mm, each closed by a bolted 12 mm steel blanking plate held by four of the eight M16 bolts, a cast lantern with its mechanical seal, coupling and two lifting lugs, a 3 kW four-pole IE3 motor of frame size 100 with its faded paint, a cut cable at the terminal box and a dented fan cowl; drained, {M_PUMP:.0f} kg of pump standing on its foot between two chocks on an 800 x 600 mm half pallet, {M_TOTAL:.0f} kg in all. Poses: stored on the pallet, lifted on a two-leg sling with the pallet left behind, and reinstalled between the flanges of a riser, delivering 50 m3/h against 14 m with heating water of 70 degC.",
    f"Eine ausgebaute Trockenläufer-Inlinepumpe DN 80 PN 16 aus einem Heizungssteigstrang, wie sie eine Bauteilbörse anbietet: ein Spiralgehäuse aus Grauguss mit 360 mm Baulänge und zwei Flanschen nach EN 1092-1 von 200 mm, jeder mit einer angeschraubten Stahl-Blindscheibe 12 mm an vier der acht Schrauben M16 verschlossen, eine gegossene Laterne mit Gleitringdichtung, Kupplung und zwei Hebeösen, ein vierpoliger IE3-Motor 3 kW der Baugröße 100 mit verblichenem Lack, ein abgeschnittenes Kabel am Klemmenkasten und eine verbeulte Lüfterhaube; entleert steht die {M_PUMP:.0f} kg schwere Pumpe auf ihrem Fuß zwischen zwei Keilhölzern auf einer Halbpalette 800 x 600 mm, zusammen {M_TOTAL:.0f} kg. Posen: auf der Palette gelagert, am Zweistranggehänge gehoben mit der Palette am Boden, und zwischen den Flanschen eines Steigstrangs wieder eingebaut, wo sie mit Heizwasser von 70 °C 50 m3/h gegen 14 m fördert.",
)

PROVENANCE = {
    "author": "Semio staffage component authors (used HVAC wave, author 16)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "inline-pump-catalogues", "citation": "Catalogue and installation data of dry-rotor inline circulators DN 80 PN 16 for heating and cooling water (Wilo IL 80, Grundfos TP 80, KSB Etaline families): port-to-port length 360 mm, volute housing EN-GJL-250 with integral foot, lantern with mechanical seal and two lifting lugs, B5 motor flange FF215, 3 kW four-pole motors, pump mass 75 to 90 kg, about 5 l water content, duty points of 40 to 60 m3/h at 10 to 16 m."},
        {"id": "en-1092-1", "citation": "EN 1092-1:2018 Flanges and their joints, circular flanges for pipes, valves, fittings and accessories, PN designated, steel flanges: DN 80 PN 16 with outside diameter 200 mm, bolt circle 160 mm, eight M16 bolts in 18 mm holes; blank flanges of type 05."},
        {"id": "en-iso-5199", "citation": "EN ISO 5199:2002 Technical specifications for centrifugal pumps, class II (allowable nozzle loads, pipework to be supported independently of the pump, lifting points)."},
        {"id": "en-iso-9906", "citation": "EN ISO 9906:2012 Rotodynamic pumps, hydraulic performance acceptance tests (hydraulic power P = rho g Q H; 50 m3/h at 14 m gives 1.91 kW, about 2.7 kW at the shaft for 70 % pump efficiency)."},
        {"id": "en-60034-30-1", "citation": "EN 60034-30-1:2014 Rotating electrical machines, efficiency classes of line operated AC motors (IE code): IE3 four-pole 3 kW at 50 Hz with nominal efficiency 87.7 %, so about 370 W losses at rated output."},
        {"id": "en-50347", "citation": "EN 50347:2001 General purpose three-phase induction motors having standard dimensions and outputs, frame numbers 56 to 315 and flange numbers 65 to 740: frame 100L with shaft height 100 mm and flange FF215 of 250 mm."},
        {"id": "din-en-1561", "citation": "DIN EN 1561:2012 Founding, grey cast irons (EN-GJL-250: density about 7200 kg/m3, E about 110 GPa, tensile strength 250 MPa)."},
        {"id": "epal-half-pallet", "citation": "EPAL half pallet (EPAL 6) technical regulations: 800 x 600 x 144 mm, 22 mm boards, 78 mm blocks, nine blocks with three bottom boards, three stringer boards and five top boards of softwood, about 9.5 kg dry; here 470 kg/m3 for the moist used softwood and its full board widths."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3, E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (softwood 0.13 W/mK, steel 50 W/mK, cast iron 50 W/mK, aluminium 160 W/mK)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: oxidised cast iron 0.8, enamel paint 0.9, rusted steel 0.8, galvanised sheet 0.3, tarnished brass 0.3, aluminium 0.2)."},
        {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch für Heizung und Klimatechnik, 79th ed., ITM 2019 (circulating pumps in heating systems, flow temperatures of 70 degC, cooling of surface-cooled motors by their shaft fan)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."},
    ],
    "notes": T(
        "Dimensions, masses, water content and duty point are typical values of the DN 80 inline circulator family (authored estimates from the cited catalogues, nothing is measured); the flanges and blanking plates follow EN 1092-1, the motor EN 50347 and EN 60034-30-1, the pallet the EPAL half pallet. For storage only four of the eight flange holes carry bolts.",
        "Maße, Massen, Wasserinhalt und Betriebspunkt sind typische Werte der Inlinepumpen DN 80 (Schätzwerte nach den zitierten Katalogen, nichts ist gemessen); Flansche und Blindscheiben folgen EN 1092-1, der Motor EN 50347 und EN 60034-30-1, die Palette der EPAL-Halbpalette. Für die Lagerung tragen nur vier der acht Flanschbohrungen Schrauben."),
}

MATERIALS = [
    {"id": "cast-iron", "title": T("Grey cast iron EN-GJL-250 of housing, cover and lantern, rust-brown primer", "Grauguss EN-GJL-250 von Gehäuse, Deckel und Laterne, rostbraune Grundierung"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 110000000000, "strengthPa": 250000000, "source": "literature", "reference": "din-en-1561"},
    {"id": "painted-aluminium", "title": T("Cast aluminium motor frame and terminal box, blue paint faded and chipped", "Aluminiumguss von Motorgehäuse und Klemmenkasten, blauer Lack verblichen und abgeplatzt"), "emissivity": 0.88, "solarAbsorptivity": 0.65, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "elasticModulusPa": 70000000000, "strengthPa": 150000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel-sheet", "title": T("Painted steel sheet of the fan cowl 1.5 mm, dented", "Lackiertes Stahlblech der Lüfterhaube 1,5 mm, verbeult"), "emissivity": 0.85, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel", "title": T("Black steel, rusty (blanking plates, bolts and nuts)", "Schwarzstahl, angerostet (Blindscheiben, Schrauben und Muttern)"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "brass", "title": T("Tarnished brass gauge plugs", "Angelaufene Messing-Manometerstopfen"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": "polyamide", "title": T("Black polyamide cable gland", "Schwarze Kabelverschraubung aus Polyamid"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1700, "source": "literature", "reference": "incropera-2018"},
    {"id": "pvc", "title": T("Grey PVC sheath of the cut motor cable", "Grauer PVC-Mantel des abgeschnittenen Motorkabels"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 1400, "conductivityWmK": 0.17, "specificHeatJKgK": 1000, "source": "literature", "reference": "incropera-2018"},
    {"id": "softwood", "title": T("Used softwood of the half pallet and chocks, grey and moist", "Gebrauchtes Nadelholz der Halbpalette und der Keilhölzer, vergraut und feucht"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 470, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "aluminium", "title": T("Embossed aluminium type plate", "Geprägtes Aluminium-Typenschild"), "emissivity": 0.2, "solarAbsorptivity": 0.3, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
]

CIRCUIT = {"key": "heating", "rows": [{"v": 0.5, "supplyTempC": 45, "returnTempC": 40}, {"v": 1, "supplyTempC": 70, "returnTempC": 55}]}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, model, material, length, geometry, layers, extra, src="authored-estimate", ref="inline-pump-catalogues"):
    """🔥 One thermal region with its area from AREA and its heat capacity from its layers."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.2,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers), 1), "source": src, "reference": ref}
    out.update(extra)
    return out


CAST = layer("casting", "Grey cast iron 10 mm", "Grauguss 10 mm", "metal", "cast-iron", 0.010, 50, 7200, 460, ref="din-en-1561")

THERMAL = {
    "environment": {"airTempC": 15, "meanRadiantTempC": 14, "relativeHumidity": 0.6, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "pump", "title": T("Pump motor", "Pumpenmotor"), "levels": [{"v": 0, "title": T("Off, cable cut or switched off", "Aus, Kabel abgeschnitten oder abgeschaltet")}, {"v": 1, "title": T("Running at 1450 rpm, 50 m3/h against 14 m, about 3.1 kW input", "Betrieb mit 1450 1/min, 50 m3/h gegen 14 m, etwa 3,1 kW Aufnahme")}]},
        {"id": "heating", "title": T("Heating water in the housing", "Heizwasser im Gehäuse"), "levels": [{"v": 0, "title": T("Drained or cold", "Entleert oder kalt")}, {"v": 0.5, "title": T("Reduced, 45 degC", "Abgesenkt, 45 °C")}, {"v": 1, "title": T("Design, 70 degC flow", "Auslegung, 70 °C Vorlauf")}]},
    ],
    "regions": [
        region("casing", "Cast-iron volute housing with ports, flanges, nuts and gauge plugs", "Spiralgehäuse aus Grauguss mit Stutzen, Flanschen, Muttern und Manometerstopfen", "surface", "hydronic", "cast-iron", 0.36, "vertical-plate", [CAST],
               {"hydronic": dict(CIRCUIT, flowPosition=0.0, filmM2KW=0.0003)}),
        region("lantern", "Pump cover and lantern with the mechanical seal, coupling and lugs", "Pumpendeckel und Laterne mit Gleitringdichtung, Kupplung und Ösen", "surface", "operating", "cast-iron", 0.17, "cylinder", [CAST],
               {"operating": {"key": "pump", "heatW": 40}}),
        region("motor", "IE3 motor 3 kW, finned aluminium frame with its flange", "IE3-Motor 3 kW, beripptes Aluminiumgehäuse mit Flansch", "surface", "operating", "painted-aluminium", 0.20, "cylinder",
               [layer("frame", "Cast aluminium frame 6 mm", "Aluminiumgussgehäuse 6 mm", "metal", "painted-aluminium", 0.006, 160, 2700, 900, ref="incropera-2018")],
               {"operating": {"key": "pump", "heatW": 140}, "ratedPowerW": 3000, "ratedPowerBasis": "rated-output"}),
        region("terminal-box", "Terminal box with type plate, gland and cut cable", "Klemmenkasten mit Typenschild, Verschraubung und abgeschnittenem Kabel", "surface", "operating", "painted-aluminium", 0.12, "vertical-plate",
               [layer("box", "Cast aluminium 3 mm", "Aluminiumguss 3 mm", "metal", "painted-aluminium", 0.003, 160, 2700, 900, ref="incropera-2018")], {"operating": {"key": "pump", "heatW": 5}}),
        region("fan-cowl", "Dented steel fan cowl with its grille", "Verbeulte Lüfterhaube aus Stahlblech mit Gitter", "surface", "passive", "steel-sheet", 0.21, "cylinder",
               [layer("sheet", "Steel sheet 1.5 mm", "Stahlblech 1,5 mm", "metal", "steel-sheet", 0.0015, 50, 7850, 460, ref="incropera-2018")], {}),
        region("blanking-plates", "Two steel blanking plates with their bolts", "Zwei Stahl-Blindscheiben mit ihren Schrauben", "surface", "passive", "steel", 0.20, "vertical-plate",
               [layer("plate", "Steel plate 12 mm", "Stahlplatte 12 mm", "metal", "steel", 0.012, 50, 7850, 460, ref="incropera-2018")], {}, ref="en-1092-1"),
        region("pallet", "Half pallet and chocks", "Halbpalette und Keilhölzer", "surface", "passive", "softwood", 0.1, "horizontal-plate-up",
               [layer("timber", "Softwood, thermally active 20 mm", "Nadelholz, thermisch wirksam 20 mm", "timber", "softwood", 0.02, 0.13, 470, 1600, ref="en-338")], {"ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "literature", "reference": "epal-half-pallet"}),
    ],
    "emitters": [
        {"id": "motor-fan", "kind": "fan", "title": T("Cooling air drawn through the cowl grille and blown down along the motor fins", "Kühlluft, durch das Haubengitter angesaugt und an den Motorrippen entlang nach unten geblasen"), "anchor": "fan-intake", "directionDeg": 270, "diameterM": 0.18, "operatingKey": "pump",
         "volumeFlowM3S": 0.06, "temperatureRiseK": 3, "source": "authored-estimate", "reference": "recknagel-2019"},
    ],
    "assumptions": [
        T("Stored and lifted, the pump is drained, cold and at the 15 degC of the hall; reinstalled it is refilled with 5 l and its housing follows the heating water of the riser (70 degC flow at design, 45 degC reduced); the lantern takes the friction of the mechanical seal and the bearings (40 W), the motor its frame losses (140 W) and the terminal box 5 W.",
          "Gelagert und gehoben ist die Pumpe entleert, kalt und hat die 15 °C der Halle; eingebaut ist sie mit 5 l befüllt und ihr Gehäuse folgt dem Heizwasser des Strangs (70 °C Vorlauf bei Auslegung, 45 °C abgesenkt); die Laterne übernimmt die Reibung von Gleitringdichtung und Lagern (40 W), der Motor seine Gehäuseverluste (140 W) und der Klemmenkasten 5 W."),
        T("The nameplate of 3 kW is the rated mechanical output of the IE3 motor (EN 60034-30-1, 87.7 %); at the duty point of 50 m3/h and 14 m the hydraulic power is 1.91 kW and the input about 3.1 kW; of its 370 W losses about 200 W leave with the 0.06 m3/s of fan air, 3 K warmer.",
          "Das Typenschild von 3 kW ist die mechanische Bemessungsleistung des IE3-Motors (EN 60034-30-1, 87,7 %); im Betriebspunkt von 50 m3/h und 14 m beträgt die hydraulische Leistung 1,91 kW und die Aufnahme etwa 3,1 kW; von seinen 370 W Verlusten gehen etwa 200 W mit den 0,06 m3/s Lüfterluft ab, 3 K wärmer."),
        T("Weathered and painted surfaces carry the emissivities of oxidised, painted and rusted metal (Incropera); the faded blue motor paint absorbs 0.65 of the solar radiation instead of about 0.8 when new.",
          "Verwitterte und lackierte Oberflächen haben die Emissionsgrade von oxidiertem, lackiertem und angerostetem Metall (Incropera); der verblichene blaue Motorlack absorbiert 0,65 der Sonnenstrahlung statt etwa 0,8 im Neuzustand."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the {M_PUMP:.0f} kg pump stands with its cast foot between two chocks on the half pallet, whose three bottom boards rest on the floor (four contacts, friction 0.5); lifted, the two lugs of the lantern hang it with its blanking plates on a two-leg sling (pins); reinstalled, the four corners of the foot stand on the plant room floor (contacts, friction 0.4).",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert steht die {M_PUMP:.0f} kg schwere Pumpe mit ihrem gegossenen Fuß zwischen zwei Keilhölzern auf der Halbpalette, deren drei Bodenbretter auf dem Boden liegen (vier Kontakte, Reibung 0,5); gehoben hängen die zwei Ösen der Laterne sie mit ihren Blindscheiben in ein Zweistranggehänge (Gelenke); eingebaut stehen die vier Ecken des Fußes auf dem Boden der Heizzentrale (Kontakte, Reibung 0,4)."),
    T(f"The solver treats the whole asset as one rigid body: in the lifted pose the {M_PALLET:.1f} kg of pallet left on the floor are counted in the sling reactions, and in the reinstalled poses pallet and blanking plates are carried by the pump foot ({100 * (M_PALLET + M_BLANKS) / M_TOTAL:.1f} % of the asset weight).",
      f"Der Löser behandelt das ganze Asset als einen starren Körper: in der gehobenen Pose gehen die {M_PALLET:.1f} kg der am Boden stehenden Palette in die Reaktionen des Gehänges ein, in den Einbauposen trägt der Pumpenfuß Palette und Blindscheiben ({100 * (M_PALLET + M_BLANKS) / M_TOTAL:.1f} % des Assetgewichts)."),
    T("Reinstalled, the housing carries 5 l of heating water (49 N) and the weight of the riser pipework on both flanges (0.25 kN each, below the nozzle loads of EN ISO 5199, the riser being supported on its own); housing, lantern and motor are one rigid member chain on the foot.",
      "Eingebaut trägt das Gehäuse 5 l Heizwasser (49 N) und das Gewicht der Strangleitungen an beiden Flanschen (je 0,25 kN, unter den Stutzenlasten nach EN ISO 5199, der Strang ist eigens abgefangen); Gehäuse, Laterne und Motor sind eine starre Bauteilkette auf dem Fuß."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(
        ("Side elevation 16 mm long and 18.4 mm high on paper, front 12 x 18.4 mm, plan 16 x 12 mm: the half pallet with its boards and blocks, the two chocks, the cast foot and pedestal, the volute with its two 200 mm flanges and their blanking plates, the cover, the lantern with its window, coupling and two lugs, the motor flange, the finned motor with its terminal box and the fan cowl on top.",
         "Seitenansicht auf Papier 16 mm lang und 18,4 mm hoch, Vorderansicht 12 x 18,4 mm, Grundriss 16 x 12 mm: die Halbpalette mit Brettern und Klötzen, die zwei Keilhölzer, Fuß und Sockel aus Guss, das Spiralgehäuse mit seinen zwei Flanschen von 200 mm und deren Blindscheiben, der Deckel, die Laterne mit Fenster, Kupplung und zwei Ösen, der Motorflansch, der berippte Motor mit Klemmenkasten und die Lüfterhaube oben."),
        ("Every part at its true size: 22 mm flanges, 12 mm blanking plates with their M16 bolt heads and the nuts behind the flanges, the lugs with their bore, the brass gauge plugs, cable gland and cut cable, type plate, terminal lid seam, cooling fins, the cowl dent, the casting seam, the flow arrow, rust at the flange edges and the grain and nails of the boards.",
         "Jedes Teil in wahrer Größe: Flansche 22 mm, Blindscheiben 12 mm mit ihren Schraubenköpfen M16 und den Muttern hinter den Flanschen, die Ösen mit ihrer Bohrung, die Messing-Manometerstopfen, Kabelverschraubung und abgeschnittenes Kabel, Typenschild, Deckelfuge des Klemmenkastens, Kühlrippen, die Delle der Haube, die Gussnaht, der Strömungspfeil, Rost an den Flanschrändern sowie Maserung und Nägel der Bretter."),
        ("The far blocks, boards and bolts lie behind the near ones in each view; the terminal box hides part of the motor in the front view.",
         "Die hinteren Klötze, Bretter und Schrauben liegen in jeder Ansicht hinter den vorderen; der Klemmenkasten verdeckt in der Vorderansicht einen Teil des Motors."),
        ("Impeller, shaft seal and motor winding inside the housing are not drawn: the castings hide them; the lantern window shows the coupling.",
         "Laufrad, Wellendichtung und Motorwicklung im Gehäuse sind nicht gezeichnet: die Gussteile verbergen sie; das Laternenfenster zeigt die Kupplung."),
        ("The 1.5 mm type plate is a line; the 12 mm plates are 0.24 mm, the 10 mm bolt heads 0.2 mm on paper.",
         "Das 1,5 mm Typenschild ist eine Linie; die 12 mm Scheiben messen 0,24 mm, die 10 mm Schraubenköpfe 0,2 mm auf Papier."),
        ("The sawn ends of the pallet boards carry a 45 degree end-grain hatch in the front view.", "Die gesägten Enden der Palettenbretter tragen in der Vorderansicht eine Hirnholzschraffur unter 45 Grad."),
        ("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; every board and block has its own depth so that the 22 mm stringers read between blocks and top boards; tones: pallet 0.5 to 0.68, cast iron 0.28 to 0.36, motor 0.46 to 0.5, cowl 0.42, steel 0.12 to 0.2, lantern window 0.08.",
         "Körperstift 0,35 mm, Details 0,18 mm, Textur 0,1 mm; jedes Brett und jeder Klotz hat seine eigene Tiefe, damit die 22 mm Querbretter zwischen Klötzen und Deckbrettern lesbar bleiben; Töne: Palette 0,5 bis 0,68, Guss 0,28 bis 0,36, Motor 0,46 bis 0,5, Haube 0,42, Stahl 0,12 bis 0,2, Laternenfenster 0,08."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    100: D(
        ("Side elevation 8 x 9.2 mm, front 6 x 9.2 mm, plan 8 x 6 mm: pallet, chocks, foot, volute with flanges and blanking plates, cover, lantern with lugs and window, motor flange, motor, terminal box and cowl.",
         "Seitenansicht 8 x 9,2 mm, Vorderansicht 6 x 9,2 mm, Grundriss 8 x 6 mm: Palette, Keilhölzer, Fuß, Spiralgehäuse mit Flanschen und Blindscheiben, Deckel, Laterne mit Ösen und Fenster, Motorflansch, Motor, Klemmenkasten und Haube."),
        ("The 22 mm flanges, the blanking plates with their bolt heads as one 22 mm disc, the lugs as 50 mm discs, the lantern window, three fin lines and the outer cowl grille.",
         "Die 22 mm Flansche, die Blindscheiben mit ihren Schraubenköpfen als eine Scheibe von 22 mm, die Ösen als Scheiben von 50 mm, das Laternenfenster, drei Rippenlinien und das äußere Haubengitter."),
        ("Plate and bolt heads are one disc; the lugs are full discs without bore; the board edges stay square.",
         "Scheibe und Schraubenköpfe sind eine Scheibe; die Ösen sind volle Scheiben ohne Bohrung; die Brettkanten bleiben eckig."),
        ("Nuts, gauge plugs, cable gland, cable, type plate, coupling, seams, dent, rust, grain and nails.", "Muttern, Manometerstopfen, Kabelverschraubung, Kabel, Typenschild, Kupplung, Fugen, Delle, Rost, Maserung und Nägel."),
        ("The 22 mm plates and flanges are 0.22 mm on paper.", "Die 22 mm Scheiben und Flansche messen 0,22 mm auf Papier."),
        ("None.", "Keine."),
        ("Body pen 0.25 mm, detail 0.13 mm; the tones of 1:50.", "Körperstift 0,25 mm, Details 0,13 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    200: D(
        ("Side elevation 4 x 4.6 mm, front 3 x 4.6 mm, plan 4 x 3 mm: the pallet as a deck on three runners, the foot, the volute with the cover, the two flanges with their plates as 30 mm discs, the lantern, the motor flange, the motor with the terminal box and the cowl.",
         "Seitenansicht 4 x 4,6 mm, Vorderansicht 3 x 4,6 mm, Grundriss 4 x 3 mm: die Palette als Deck auf drei Kufen, der Fuß, das Spiralgehäuse mit dem Deckel, die zwei Flansche mit ihren Scheiben als Scheiben von 30 mm, die Laterne, der Motorflansch, der Motor mit Klemmenkasten und die Haube."),
        ("Pallet deck and runners, chocks, foot, volute, necks, flanges, plates, lantern, motor flange, motor, terminal box and cowl, all at least 30 mm.", "Palettendeck und Kufen, Keilhölzer, Fuß, Spiralgehäuse, Stutzen, Flansche, Scheiben, Laterne, Motorflansch, Motor, Klemmenkasten und Haube, alles mindestens 30 mm."),
        ("Boards, blocks and stringers are one deck on three runners; foot and pedestal are one block; volute and cover are one 276 mm cylinder; flanges and plates are 30 mm thick.",
         "Bretter, Klötze und Querbretter sind ein Deck auf drei Kufen; Fuß und Sockel sind ein Block; Spiralgehäuse und Deckel sind ein Zylinder von 276 mm; Flansche und Scheiben sind 30 mm dick."),
        ("Lugs, bolts, nuts, plugs, gland, cable, type plate, window, fins, grille and every texture.", "Ösen, Schrauben, Muttern, Stopfen, Verschraubung, Kabel, Typenschild, Fenster, Rippen, Gitter und alle Texturen."),
        ("No part below 30 mm (0.15 mm on paper) remains.", "Kein Teil unter 30 mm (0,15 mm auf Papier) bleibt."),
        ("None.", "Keine."),
        ("One body pen of 0.18 mm; the tones of 1:50.", "Ein Körperstift von 0,18 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
}

DECISIONS_3D = {
    50: [T("Body: every part of the 1:50 drawing as a solid: pallet boards, blocks and stringers as boxes, the top boards with 3 mm arrises, the volute, necks, flanges, cover, lantern, motor flange, motor and cowl as revolves, foot, pedestal and terminal box as boxes with 4 mm edges, the lugs as rings with their bore, nuts, bolt heads, gauge plugs, gland and cable as revolves, the type plate as a sheet.",
           "Körper: jedes Teil der Zeichnung 1:50 als Körper: Palettenbretter, Klötze und Querbretter als Quader, die Deckbretter mit 3 mm gefasten Kanten, Spiralgehäuse, Stutzen, Flansche, Deckel, Laterne, Motorflansch, Motor und Haube als Drehkörper, Fuß, Sockel und Klemmenkasten als Quader mit 4 mm Kanten, die Ösen als Ringe mit ihrer Bohrung, Muttern, Schraubenköpfe, Manometerstopfen, Verschraubung und Kabel als Drehkörper, das Typenschild als Blech."),
         T("Thermal: every region by reference to its body solids and a disc in the cowl grille for the fan intake. Structure: pallet deck and runners as members, the pump parts and the blanking plates by reference as members, four bearing pads under the foot and the centres of mass of pump, blanking plates and pallet.",
           "Wärme: jede Region als Verweis auf ihre Körper und eine Scheibe im Haubengitter für den Lüftereinlass. Tragwerk: Palettendeck und Kufen als Stäbe, die Pumpenteile und die Blindscheiben als Verweise auf Stäbe, vier Auflagerpolster unter dem Fuß und die Massenmittelpunkte von Pumpe, Blindscheiben und Palette.")],
    100: [T("Body for the 15 mm feature: every board and block of the pallet, plates and bolt heads one 22 mm disc, the lugs 50 mm discs 16 mm thick without bore; nuts, gauge plugs, gland, cable and type plate are left out; the board arrises are square.",
            "Körper für das Merkmal von 15 mm: jedes Brett und jeder Klotz der Palette, Scheiben und Schraubenköpfe eine Scheibe von 22 mm, die Ösen Scheiben von 50 mm und 16 mm Dicke ohne Bohrung; Muttern, Manometerstopfen, Verschraubung, Kabel und Typenschild entfallen; die Brettkanten sind eckig."),
          T("Thermal and structure as at 1:50 on the coarser body; the fan intake disc is 15 mm thick, the bearing pads 15 mm.",
            "Wärme und Tragwerk wie bei 1:50 auf dem gröberen Körper; die Lüftereinlassscheibe ist 15 mm dick, die Auflagerpolster 15 mm.")],
    200: [T("Body for the 30 mm feature: the pallet as a deck on three runners, foot and pedestal one block, volute and cover one cylinder, flanges and plates 30 mm thick, lantern and motor flange each one cylinder; lugs are left out.",
            "Körper für das Merkmal von 30 mm: die Palette als Deck auf drei Kufen, Fuß und Sockel ein Block, Spiralgehäuse und Deckel ein Zylinder, Flansche und Scheiben 30 mm dick, Laterne und Motorflansch je ein Zylinder; die Ösen entfallen."),
          T("Thermal: the regions by reference and a 30 mm fan intake disc; structure: deck, runners and the pump parts as members, bearing pads of 30 mm, mass markers.",
            "Wärme: die Regionen als Verweise und eine Lüftereinlassscheibe von 30 mm; Tragwerk: Deck, Kufen und die Pumpenteile als Stäbe, Auflagerpolster von 30 mm, Massenmarken.")],
}
