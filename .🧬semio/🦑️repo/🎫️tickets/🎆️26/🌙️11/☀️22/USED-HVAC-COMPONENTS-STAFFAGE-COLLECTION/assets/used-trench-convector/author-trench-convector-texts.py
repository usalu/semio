"""📜 Texts, materials, thermal model, structure and decisions of the used trench convector, executed into the namespace of author-trench-convector.py."""

AREA = {"trough": 2.05, "frame": 0.24, "heat-exchanger": 3.90, "tails": 0.0055, "grille": 0.336, "bearers": 0.42}

_D = YT - YB - SH
TROUGH_SECTION_A = (2 * TZ + 2 * _D) * SH
_YC = (2 * _D * SH * (_D / 2 + SH)) / TROUGH_SECTION_A
TROUGH_SECTION_I = 2 * TZ * SH * _YC ** 2 + 2 * (SH * _D ** 3 / 12 + _D * SH * (_D / 2 + SH - _YC) ** 2)
TROUGH_SECTION_E = max(_YC, YT - YB - _YC)

SUMMARY = T(
    f"A dismantled floor trench convector for natural convection as an HVAC reuse dealer offers it: a 2.0 m long folded steel trough of 260 x 110 mm (1.25 mm galvanised sheet, black powder-coated inside) with its natural-anodised aluminium edge frame, a copper-aluminium heat exchanger of 1.80 m (two 15 mm copper tubes through aluminium fins at 8 mm pitch, dusty, two fins dented) on three brackets on the window side, the flow and return tails cut from their hoses and capped with brass caps, a bleed valve, four levelling brackets and grey screed remnants on the outer wall; drained, {M_CONVECTOR:.1f} kg. The roll-up aluminium grille of 2000 x 250 mm is removed and lies strapped as a 236 mm coil on one of the two 80 x 80 mm softwood bearers beside it; {M_TOTAL:.1f} kg in all. Poses: stored on the bearers, carried by two people with bearers and grille left behind, and reinstalled on a slab heating at 75/65 degC with warm air rising over the fins.",
    f"Ein ausgebauter Unterflurkonvektor für freie Konvektion, wie ihn ein Händler für gebrauchte Haustechnik anbietet: eine 2,0 m lange abgekantete Stahlwanne von 260 x 110 mm (verzinktes Blech 1,25 mm, innen schwarz pulverbeschichtet) mit ihrem natur eloxierten Aluminium-Randrahmen, ein Kupfer-Aluminium-Wärmetauscher von 1,80 m (zwei Kupferrohre 15 mm durch Aluminiumlamellen im Abstand von 8 mm, verstaubt, zwei Lamellen verbogen) auf drei Haltern an der Fensterseite, Vor- und Rücklaufanschluss von ihren Schläuchen getrennt und mit Messingkappen verschlossen, ein Entlüftungsventil, vier Nivellierwinkel und graue Estrichreste an der Außenwand; entleert {M_CONVECTOR:.1f} kg. Der Rollrost aus Aluminium von 2000 x 250 mm ist ausgebaut und liegt umreift als Rolle von 236 mm auf einem der zwei Nadelholz-Kanthölzer 80 x 80 mm daneben; zusammen {M_TOTAL:.1f} kg. Posen: auf den Kanthölzern gelagert, von zwei Personen getragen, wobei Kanthölzer und Rost liegen bleiben, und wieder eingebaut auf einer Rohdecke im Heizbetrieb 75/65 °C mit warmer Luft über den Lamellen.")

PROVENANCE = {
    "author": "Semio staffage component authors (used HVAC wave, author 12)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "trench-convector-catalogues", "citation": "Catalogue and installation data of natural-convection floor trench convectors of the 1990s to 2010s (Kampmann Katherm NK, Jaga Mini Canal, Möhlenhoff Wärmetechnik, Emco): troughs 180 to 420 mm wide and 90 to 200 mm deep in lengths of 0.8 to 5 m, galvanised sheet 1.0 to 1.5 mm with black inner coating, aluminium edge frames, copper-aluminium heat exchangers on the window side with 15 mm copper tubes and aluminium fins, height-adjustable levelling brackets, roll-up linear grilles of anodised aluminium; heat output of a 260 x 110 mm trough of 2 m about 400 to 550 W at 75/65/20 degC, about 1.5 kg of aluminium grille per metre of 250 mm width; the convector of this asset is a typical member of the family, not one model (authored-estimate)."},
        {"id": "din-en-16430", "citation": "DIN EN 16430-1/-2:2015 Fan assisted radiators, convectors and trench convectors: technical specifications and requirements, test method and rating for thermal output (rated output at 75/65/20 degC, characteristic equation with exponent n of about 1.4 for natural-convection trench heaters)."},
        {"id": "din-en-442", "citation": "DIN EN 442-2:2015 Radiators and convectors: test methods and rating (standard conditions 75/65/20 degC, conversion of output to other temperature differences)."},
        {"id": "din-en-1057", "citation": "DIN EN 1057:2010 Copper and copper alloys, seamless round copper tubes for water and gas: 15 x 1.0 mm, 0.391 kg/m, 0.133 l/m."},
        {"id": "din-en-10346", "citation": "DIN EN 10346:2015 Continuously hot-dip coated steel flat products (DX51D+Z275 sheet 1.25 mm, density 7850 kg/m3)."},
        {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch für Heizung und Klimatechnik, 79th ed., ITM 2019 (convectors and trench heaters in front of glazed facades: cold-air drop at the window, warm air rising through the grille, low water content and fast response)."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3 (450 kg/m3 used for the weathered, air-dry bearers), E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (softwood 0.13 W/mK, steel 50 W/mK, aluminium 160 W/mK, copper 380 W/mK, cement screed 1.4 W/mK)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: black paint 0.9, anodised aluminium 0.8, dusty aluminium fins 0.3, oxidised copper 0.6, tarnished brass 0.3, weathered galvanised steel 0.28)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."},
    ],
    "notes": T(
        "Dimensions, output, water content and masses are typical values of the natural-convection trench convector family (authored estimates from the cited catalogues, nothing is measured); the copper tubes follow DIN EN 1057, the sheet DIN EN 10346 and the bearers EN 338. The grille is part of the asset but stays rolled up in every pose: the reinstalled poses show the commissioning state before the screed is laid, when the trough is open for bleeding and inspection.",
        "Maße, Leistung, Wasserinhalt und Massen sind typische Werte der Unterflurkonvektoren für freie Konvektion (Schätzwerte nach den zitierten Katalogen, nichts ist gemessen); die Kupferrohre folgen DIN EN 1057, das Blech DIN EN 10346 und die Kanthölzer EN 338. Der Rost gehört zum Asset, bleibt aber in jeder Pose aufgerollt: die Einbauposen zeigen den Zustand der Inbetriebnahme vor dem Estrich, wenn die Wanne zum Entlüften und Prüfen offen ist."),
}

MATERIALS = [
    {"id": "coated-steel", "title": T("Galvanised sheet steel 1.25 mm, black powder-coated inside, scuffed", "Verzinktes Stahlblech 1,25 mm, innen schwarz pulverbeschichtet, verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 140000000, "source": "literature", "reference": "din-en-10346"},
    {"id": "galvanised-steel", "title": T("Weathered galvanised steel of brackets", "Verwitterter verzinkter Stahl der Halter"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 140000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "anodised-aluminium", "title": T("Natural anodised aluminium of the frame and the grille bars, scratched", "Natur eloxiertes Aluminium von Rahmen und Roststäben, zerkratzt"), "emissivity": 0.8, "solarAbsorptivity": 0.35, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "elasticModulusPa": 70000000000, "strengthPa": 160000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "aluminium-fins", "title": T("Dusty aluminium fins 0.25 mm of the heat exchanger", "Verstaubte Aluminiumlamellen 0,25 mm des Wärmetauschers"), "emissivity": 0.3, "solarAbsorptivity": 0.4, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
    {"id": "copper", "title": T("Oxidised copper tube 15 x 1 mm", "Oxidiertes Kupferrohr 15 x 1 mm"), "emissivity": 0.6, "solarAbsorptivity": 0.65, "densityKgM3": 8930, "conductivityWmK": 380, "specificHeatJKgK": 385, "source": "literature", "reference": "din-en-1057"},
    {"id": "brass", "title": T("Tarnished brass caps and bleed valve", "Angelaufene Messingkappen und Entlüftungsventil"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": "polypropylene", "title": T("Polypropylene strapping band around the grille coil", "Umreifungsband aus Polypropylen um die Rostrolle"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 910, "conductivityWmK": 0.22, "specificHeatJKgK": 1900, "source": "literature", "reference": "en-iso-10456"},
    {"id": "cement-screed", "title": T("Cement screed remnants on the outer trough wall", "Zementestrichreste an der Wannenaußenwand"), "emissivity": 0.93, "solarAbsorptivity": 0.65, "densityKgM3": 2000, "conductivityWmK": 1.4, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "label-film", "title": T("Faded polyester type label", "Verblichenes Typenetikett aus Polyester"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1400, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "literature", "reference": "incropera-2018"},
    {"id": "softwood", "title": T("Weathered softwood bearers C24", "Verwitterte Nadelholz-Kanthölzer C24"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
]

CIRCUIT = {"key": "heating", "rows": [{"v": 0.5, "supplyTempC": 55, "returnTempC": 45}, {"v": 1, "supplyTempC": 75, "returnTempC": 65}]}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, model, material, length, geometry, layers, extra, src="authored-estimate", ref="trench-convector-catalogues"):
    """🔥 One thermal region with its area from AREA and its heat capacity from its layers."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.2,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers), 1), "source": src, "reference": ref}
    out.update(extra)
    return out


THERMAL = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 19, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "heating", "title": T("Heating water through the element", "Heizwasser durch den Wärmetauscher"), "levels": [{"v": 0, "title": T("Drained, or valve closed", "Entleert oder Ventil geschlossen")}, {"v": 0.5, "title": T("Reduced, 55/45 degC", "Abgesenkt, 55/45 °C")}, {"v": 1, "title": T("Rated, 75/65 degC (about 470 W)", "Nennbetrieb, 75/65 °C (etwa 470 W)")}]},
    ],
    "regions": [
        region("trough", "Folded steel trough, levelling and element brackets", "Abgekantete Stahlwanne, Nivellier- und Tauscherhalter", "surface", "passive", "coated-steel", 0.24, "horizontal-plate-up",
               [layer("sheet", "Galvanised sheet 1.25 mm, powder-coated", "Verzinktes Blech 1,25 mm, pulverbeschichtet", "metal", "coated-steel", SH, 50, 7850, 460, ref="din-en-10346")], {}),
        region("frame", "Anodised aluminium edge frame", "Eloxierter Aluminium-Randrahmen", "surface", "passive", "anodised-aluminium", 0.025, "horizontal-plate-up",
               [layer("profile", "Aluminium angle 2 mm", "Aluminiumwinkel 2 mm", "metal", "anodised-aluminium", FRAME_T, 160, 2700, 900, ref="en-iso-10456")], {}),
        region("heat-exchanger", "Copper-aluminium fin element", "Kupfer-Aluminium-Lamellenwärmetauscher", "heater", "hydronic", "aluminium-fins", 0.06, "horizontal-plate-up",
               [layer("fin", "Aluminium fin 0.25 mm on copper tube", "Aluminiumlamelle 0,25 mm auf Kupferrohr", "metal", "aluminium-fins", 0.00025, 200, 2700, 900, ref="incropera-2018")],
               {"areaBasis": "exchange", "hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.02), "ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("tails", "Cut and capped flow and return tails with bleed valve", "Abgetrennte und verschlossene Vor- und Rücklaufanschlüsse mit Entlüftungsventil", "surface", "hydronic", "copper", 0.015, "cylinder",
               [layer("tube", "Copper tube 1 mm", "Kupferrohr 1 mm", "metal", "copper", 0.001, 380, 8930, 385, ref="din-en-1057")], {"hydronic": dict(CIRCUIT, flowPosition=0.0, filmM2KW=0.002)}),
        region("grille", "Roll-up aluminium grille, rolled and strapped", "Rollrost aus Aluminium, aufgerollt und umreift", "surface", "passive", "anodised-aluminium", 0.236, "cylinder",
               [layer("bars", "Aluminium bars, effective 3 mm", "Aluminiumstäbe, wirksam 3 mm", "metal", "anodised-aluminium", 0.003, 160, 2700, 900, ref="en-iso-10456")], {}),
        region("bearers", "Two squared softwood bearers", "Zwei Nadelholz-Kanthölzer", "surface", "passive", "softwood", 0.08, "horizontal-plate-up",
               [layer("timber", "Softwood, thermally active 20 mm", "Nadelholz, thermisch wirksam 20 mm", "timber", "softwood", 0.02, 0.13, 450, 1600, ref="en-338")], {"source": "literature", "reference": "en-338"}),
    ],
    "emitters": [
        {"id": "warm-air", "kind": "vent", "title": T("Warm air rising out of the trough over the fins", "Warme Luft, die über den Lamellen aus der Wanne steigt"), "anchor": "warm-air-outlet", "directionDeg": 90, "diameterM": 0.14, "operatingKey": "heating",
         "volumeFlowM3S": 0.0195, "temperatureRiseK": 20, "source": "authored-estimate", "reference": "din-en-16430"},
        {"id": "trough-plume", "kind": "plume", "title": T("Free-convection plume over the warm fin pack", "Freie Konvektionsfahne über dem warmen Lamellenpaket"), "anchor": "warm-air-outlet", "directionDeg": 90, "diameterM": 0.3, "region": "heat-exchanger", "source": "authored-estimate", "reference": "recknagel-2019"},
    ],
    "assumptions": [
        T("Stored and carried, the convector is drained and at the 20 degC of the heated hall; reinstalled, the element is filled with 0.55 l and follows the heating circuit (75/65 degC rated, 55/45 degC reduced); the fin pack and the tails take that water through their films, the trough, the frame and the grille stay passive.",
          "Gelagert und getragen ist der Konvektor entleert und hat die 20 °C der beheizten Halle; eingebaut ist der Wärmetauscher mit 0,55 l befüllt und folgt dem Heizkreis (75/65 °C Nenn, 55/45 °C abgesenkt); Lamellenpaket und Anschlüsse übernehmen dieses Wasser durch ihre Filme, Wanne, Rahmen und Rost bleiben passiv."),
        T("A fed convector has no nameplate power of its own (ratedPowerW 0, passive): its rated output of about 470 W at 75/65/20 degC (DIN EN 16430, authored estimate for a 2 m trough of 260 x 110 mm) is carried by the warm air of about 0.020 m3/s rising 20 K warmer out of the trough.",
          "Ein gespeister Konvektor hat keine eigene Nennleistung (ratedPowerW 0, passiv): seine Normleistung von etwa 470 W bei 75/65/20 °C (DIN EN 16430, Schätzwert für eine 2 m lange Wanne 260 x 110 mm) trägt die warme Luft von etwa 0,020 m3/s, die 20 K wärmer aus der Wanne steigt."),
        T("The area of the heat exchanger is its exchange area (225 fins of 140 x 60 mm on both faces and the tubes, 3.9 m2), not the envelope of the fin pack; black coating, anodised and dusty surfaces carry the emissivities of aged surfaces (Incropera): the dusty fins 0.3 instead of 0.1 when new.",
          "Die Fläche des Wärmetauschers ist seine Austauschfläche (225 Lamellen 140 x 60 mm beidseitig und die Rohre, 3,9 m2), nicht die Hülle des Lamellenpakets; schwarze Beschichtung, eloxierte und verstaubte Oberflächen haben die Emissionsgrade gealterter Oberflächen (Incropera): die verstaubten Lamellen 0,3 statt 0,1 im Neuzustand."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the {M_CONVECTOR:.1f} kg trough lies with its bottom over the four levelling brackets on two 80 x 80 mm softwood bearers that rest with their four ends on the floor (contacts, friction 0.5); carried, two people hold the two frame ends (pins); reinstalled, the four levelling brackets stand on the slab (contacts).",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert liegt die {M_CONVECTOR:.1f} kg schwere Wanne mit ihrem Boden über den vier Nivellierwinkeln auf zwei Nadelholz-Kanthölzern 80 x 80 mm, die mit ihren vier Enden auf dem Boden liegen (Kontakte, Reibung 0,5); getragen halten zwei Personen die beiden Rahmenenden (Gelenke); eingebaut stehen die vier Nivellierwinkel auf der Rohdecke (Kontakte)."),
    T(f"The solver treats the whole asset as one rigid body: carried, the {M_BEARERS:.1f} kg of bearers and the {M_GRILLE:.1f} kg grille coil left on the floor are counted in the reactions of the carriers, reinstalled they are carried by the trough feet ({100 * (M_BEARERS + M_GRILLE) / M_TOTAL:.0f} % of the asset weight).",
      f"Der Löser behandelt das ganze Asset als einen starren Körper: getragen gehen die {M_BEARERS:.1f} kg der Kanthölzer und die {M_GRILLE:.1f} kg der Rostrolle, die am Boden liegen, in die Reaktionen der Träger ein, eingebaut tragen die Wannenfüße sie ({100 * (M_BEARERS + M_GRILLE) / M_TOTAL:.0f} % des Assetgewichts)."),
    T(f"The trough is a folded open channel of 1.25 mm sheet (A {TROUGH_SECTION_A * 1e4:.2f} cm2, I {TROUGH_SECTION_I * 1e8:.1f} cm4) spanning 1.70 m between the bearers; reinstalled it carries the 0.55 l of water (5.4 N) and the two connection hoses (25 N each); the element, frame and brackets are rigid with it.",
      f"Die Wanne ist ein abgekanteter offener Kanal aus Blech 1,25 mm (A {TROUGH_SECTION_A * 1e4:.2f} cm2, I {TROUGH_SECTION_I * 1e8:.1f} cm4), der 1,70 m zwischen den Kanthölzern spannt; eingebaut trägt sie die 0,55 l Wasser (5,4 N) und die zwei Anschlussschläuche (je 25 N); Wärmetauscher, Rahmen und Halter sind starr mit ihr verbunden."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(
        ("Side elevation from the window side, 40 mm long and 6.3 mm high on paper: the 2.0 m trough wall with its frame edge on the two bearer ends, two levelling brackets, screed remnants and the label, and the grille coil with its bore and windings rising behind the connection end. Front from the connection end, 12.4 x 6.3 mm: the end wall and both frame flanges over the long side of the near bearer and the coil. Plan, 40 x 12.4 mm: the open trough with the fin pack, three brackets, the two copper tubes with their return bend, the capped tails and the bleed valve, the frame, the four levelling brackets, the coil with its strap and both bearers.",
         "Seitenansicht von der Fensterseite, auf Papier 40 mm lang und 6,3 mm hoch: die 2,0 m lange Wannenwand mit Rahmenkante auf den zwei Kantholzenden, zwei Nivellierwinkel, Estrichreste und Etikett, dahinter die Rostrolle mit Öffnung und Windungen am Anschlussende. Vorderansicht vom Anschlussende, 12,4 x 6,3 mm: Stirnwand und beide Rahmenflansche über der Längsseite des vorderen Kantholzes und der Rolle. Grundriss, 40 x 12,4 mm: die offene Wanne mit Lamellenpaket, drei Haltern, den zwei Kupferrohren mit Umlenkbogen, den verschlossenen Anschlüssen und dem Entlüftungsventil, der Rahmen, die vier Nivellierwinkel, die Rolle mit Umreifung und beide Kanthölzer."),
        ("Every part at its true size: the 15 mm tubes and 22 mm caps, the 30 mm element brackets, the 40 mm levelling brackets, the 25 mm frame flanges, the coil of 236 mm with its 100 mm bore, the fin pitch as texture strokes and two dented fins.",
         "Jedes Teil in wahrer Größe: die 15 mm Rohre und 22 mm Kappen, die 30 mm Tauscherhalter, die 40 mm Nivellierwinkel, die 25 mm Rahmenflansche, die Rolle von 236 mm mit ihrer Öffnung von 100 mm, der Lamellenabstand als Texturstriche und zwei verbogene Lamellen."),
        ("The far trough wall and the element are hidden behind the near wall in the side elevation and behind the end wall in the front view; the plan shows them.",
         "Die hintere Wannenwand und der Wärmetauscher liegen in der Seitenansicht hinter der vorderen Wand und in der Vorderansicht hinter der Stirnwand; der Grundriss zeigt sie."),
        ("The 225 single fins are one fin pack with a fin hatch; the spring spacers between the grille bars are not drawn.",
         "Die 225 einzelnen Lamellen sind ein Lamellenpaket mit Lamellenschraffur; die Federabstandshalter zwischen den Roststäben sind nicht gezeichnet."),
        ("The 1.25 mm sheet walls, the 2 mm frame and the strap are declared thin sheets of 0.025 to 0.04 mm on paper and drawn as body lines.",
         "Die Blechwände von 1,25 mm, der 2 mm Rahmen und die Umreifung sind deklarierte dünne Bleche von 0,025 bis 0,04 mm auf Papier und als Körperlinien gezeichnet."),
        ("The bearer ends carry a 45 degree end-grain hatch in the side elevation; the fin pack a fin hatch at 0.4 mm in plan.", "Die Kantholzenden tragen in der Seitenansicht eine Hirnholzschraffur unter 45 Grad; das Lamellenpaket im Grundriss eine Lamellenschraffur von 0,4 mm."),
        ("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; tones: trough 0.2, frame 0.5, coil 0.55, bearers 0.62, fins 0.42, copper 0.38, screed 0.55, label 0.8.",
         "Körperstift 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Töne: Wanne 0,2, Rahmen 0,5, Rolle 0,55, Kanthölzer 0,62, Lamellen 0,42, Kupfer 0,38, Estrich 0,55, Etikett 0,8."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    100: D(
        ("Side elevation 20 x 3.2 mm, front 6.2 x 3.2 mm, plan 20 x 6.2 mm: trough wall with frame, levelling brackets and the coil with its bore; in plan the fin pack, brackets, tubes, caps, frame and bearers.",
         "Seitenansicht 20 x 3,2 mm, Vorderansicht 6,2 x 3,2 mm, Grundriss 20 x 6,2 mm: Wannenwand mit Rahmen, Nivellierwinkel und die Rolle mit Öffnung; im Grundriss Lamellenpaket, Halter, Rohre, Kappen, Rahmen und Kanthölzer."),
        ("The 15 mm tubes, the 22 mm caps, the 30 mm brackets, the 40 mm levelling brackets and the coil bore.", "Die 15 mm Rohre, die 22 mm Kappen, die 30 mm Halter, die 40 mm Nivellierwinkel und die Öffnung der Rolle."),
        ("The tubes count to the tails region as the visible copper of the circuit.", "Die Rohre zählen zur Anschlussregion als sichtbares Kupfer des Kreises."),
        ("Bleed valve, strap, label, screed remnants, windings, dents and grain.", "Entlüftungsventil, Umreifung, Etikett, Estrichreste, Windungen, Dellen und Maserung."),
        ("Sheet walls and frame are declared sheets of 0.0125 to 0.02 mm on paper.", "Blechwände und Rahmen sind deklarierte Bleche von 0,0125 bis 0,02 mm auf Papier."),
        ("The fin hatch at 0.5 mm in plan.", "Die Lamellenschraffur mit 0,5 mm im Grundriss."),
        ("Body pen 0.25 mm, detail 0.13 mm; the tones of 1:50.", "Körperstift 0,25 mm, Details 0,13 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    200: D(
        ("Side elevation 10 x 1.6 mm, front 3.1 x 1.6 mm, plan 10 x 3.1 mm: the trough wall with its frame on the bearers and the coil; in plan the open trough with the fin pack and one connection block.",
         "Seitenansicht 10 x 1,6 mm, Vorderansicht 3,1 x 1,6 mm, Grundriss 10 x 3,1 mm: die Wannenwand mit Rahmen auf den Kanthölzern und die Rolle; im Grundriss die offene Wanne mit Lamellenpaket und einem Anschlussblock."),
        ("Trough, frame, fin pack, the coil with its 100 mm bore, the bearers, and the two tails with their caps as one 30 mm block.", "Wanne, Rahmen, Lamellenpaket, die Rolle mit ihrer Öffnung von 100 mm, die Kanthölzer und die zwei Anschlüsse mit Kappen als ein Block von 30 mm."),
        ("The fin pack reaches down to the trough bottom over its brackets; tubes, caps and bleed valve are one connection block; the frame shapes also carry the trough region.",
         "Das Lamellenpaket reicht über seine Halter bis zum Wannenboden; Rohre, Kappen und Entlüftungsventil sind ein Anschlussblock; die Rahmenformen tragen auch die Wannenregion."),
        ("Levelling brackets, element brackets, strap, hatches and every texture.", "Nivellierwinkel, Tauscherhalter, Umreifung, Schraffuren und alle Texturen."),
        ("Sheet walls and frame remain declared sheets; no other part below 30 mm (0.15 mm on paper).", "Blechwände und Rahmen bleiben deklarierte Bleche; kein anderes Teil unter 30 mm (0,15 mm auf Papier)."),
        ("None.", "Keine."),
        ("One body pen of 0.18 mm; the tones of 1:50.", "Ein Körperstift von 0,18 mm; die Töne von 1:50."),
        ("The frame shapes list frame and trough as merged regions.", "Die Rahmenformen führen Rahmen und Wanne als zusammengeführte Regionen."),
    ),
    500: D(
        ("Side elevation 4 x 0.6 mm, front 1.2 x 0.6 mm, plan 4 x 1.2 mm: one closed trough bar on two bearers and the grille coil as a disc behind it.",
         "Seitenansicht 4 x 0,6 mm, Vorderansicht 1,2 x 0,6 mm, Grundriss 4 x 1,2 mm: ein geschlossener Wannenbalken auf zwei Kanthölzern und die Rostrolle als Scheibe dahinter."),
        ("The 2.0 x 0.26 x 0.11 m trough, the 80 mm bearers and the 236 mm coil.", "Die Wanne 2,0 x 0,26 x 0,11 m, die 80 mm Kanthölzer und die Rolle von 236 mm."),
        ("Trough, frame, element and tails are one closed box that lists their four regions; the coil is a solid disc without its bore.", "Wanne, Rahmen, Wärmetauscher und Anschlüsse sind ein geschlossener Quader, der ihre vier Regionen führt; die Rolle ist eine volle Scheibe ohne Öffnung."),
        ("Every interior part, the brackets, the strap and every texture.", "Alle Innenteile, die Halter, die Umreifung und alle Texturen."),
        ("No part below 75 mm (0.15 mm on paper) remains.", "Kein Teil unter 75 mm (0,15 mm auf Papier) bleibt."),
        ("None.", "Keine."),
        ("One body pen of 0.13 mm; the tones of 1:50.", "Ein Körperstift von 0,13 mm; die Töne von 1:50."),
        ("The trough shape lists trough, frame, heat exchanger and tails as merged regions.", "Die Wannenform führt Wanne, Rahmen, Wärmetauscher und Anschlüsse als zusammengeführte Regionen."),
    ),
}

DECISIONS_3D = {
    50: [T("Body: the trough as five declared 1.25 mm sheets (bottom, two walls, two ends) under four 2 mm frame flanges, the fin pack as the envelope prism of its 225 fins on three brackets, the two copper tubes with their return bend, the brass caps and the bleed valve as revolves, four levelling brackets, the grille coil as a revolve with its bore and the strap as a declared sheet band, the bearers with 4 mm arrises, and the label and three screed remnants as declared texture sheets.",
           "Körper: die Wanne als fünf deklarierte Bleche 1,25 mm (Boden, zwei Wände, zwei Stirnen) unter vier Rahmenflanschen 2 mm, das Lamellenpaket als Hüllprisma seiner 225 Lamellen auf drei Haltern, die zwei Kupferrohre mit Umlenkbogen, die Messingkappen und das Entlüftungsventil als Drehkörper, vier Nivellierwinkel, die Rostrolle als Drehkörper mit Öffnung und die Umreifung als deklariertes Blechband, die Kanthölzer mit 4 mm Fasen sowie Etikett und drei Estrichreste als deklarierte Texturbleche."),
         T("Thermal: every region by reference to its body solids and the warm-air outlet as a thin patch on top of the fin pack. Structure: bearers, trough bottom and walls and the grille coil as members, four bearing pads over the levelling brackets as joints, three mass markers.",
           "Wärme: jede Region als Verweis auf ihre Körper und der Warmluftauslass als dünnes Feld auf dem Lamellenpaket. Tragwerk: Kanthölzer, Wannenboden und -wände und die Rostrolle als Stäbe, vier Auflagerpolster über den Nivellierwinkeln als Gelenke, drei Massenmarken.")],
    100: [T("Body for the 15 mm feature: sheets and frame stay declared sheets, the fin pack on its three brackets, tubes, return bend, caps and the levelling brackets keep their sizes; bleed valve, strap, label and screed remnants are left out.",
            "Körper für das Merkmal von 15 mm: Bleche und Rahmen bleiben deklarierte Bleche, das Lamellenpaket auf seinen drei Haltern, Rohre, Umlenkbogen, Kappen und Nivellierwinkel behalten ihre Größen; Entlüftungsventil, Umreifung, Etikett und Estrichreste entfallen."),
          T("Thermal: the regions by reference, the tubes with the tails region, the outlet patch of 7.5 mm; structure: bearers, trough and coil as members, bearing pads of 15 mm and mass markers.",
            "Wärme: die Regionen als Verweise, die Rohre mit der Anschlussregion, das Auslassfeld von 7,5 mm; Tragwerk: Kanthölzer, Wanne und Rolle als Stäbe, Auflagerpolster von 15 mm und Massenmarken.")],
    200: [T("Body for the 30 mm feature: the fin pack reaches down to the trough bottom, tubes, caps and bleed valve are one 30 mm connection block, brackets and strap are left out; trough sheets, frame and the coil with its bore remain.",
            "Körper für das Merkmal von 30 mm: das Lamellenpaket reicht bis zum Wannenboden, Rohre, Kappen und Entlüftungsventil sind ein Anschlussblock von 30 mm, Halter und Umreifung entfallen; Wannenbleche, Rahmen und die Rolle mit Öffnung bleiben."),
          T("Thermal: the regions by reference and an outlet patch of 15 mm; structure: bearers, trough and coil as members, bearing pads of 30 mm, mass markers.",
            "Wärme: die Regionen als Verweise und ein Auslassfeld von 15 mm; Tragwerk: Kanthölzer, Wanne und Rolle als Stäbe, Auflagerpolster von 30 mm, Massenmarken.")],
    500: [T("Body for the 75 mm feature: the trough with frame is one closed box of 2.0 x 0.11 x 0.26 m, the coil a solid cylinder of 236 mm, the bearers 80 mm squares.",
            "Körper für das Merkmal von 75 mm: die Wanne mit Rahmen ist ein geschlossener Quader von 2,0 x 0,11 x 0,26 m, die Rolle ein voller Zylinder von 236 mm, die Kanthölzer Quadrate von 80 mm."),
          T("Thermal: the trough box by reference and the element as a box inside it; structure: bearers, trough box and coil as members, bearing pads of 75 mm, mass markers.",
            "Wärme: der Wannenquader als Verweis und der Wärmetauscher als Quader darin; Tragwerk: Kanthölzer, Wannenquader und Rolle als Stäbe, Auflagerpolster von 75 mm, Massenmarken.")],
}
