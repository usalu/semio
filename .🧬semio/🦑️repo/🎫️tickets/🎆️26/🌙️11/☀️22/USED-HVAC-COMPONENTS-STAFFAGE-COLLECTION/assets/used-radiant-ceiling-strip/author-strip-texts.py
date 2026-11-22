"""📜 Texts, materials, thermal model, structure and decisions of the used radiant ceiling heating strip, executed into the namespace of author-strip.py."""

AREA = {"plate": 12.2, "insulation": 10.9, "tubes": 2.10, "headers": 0.225, "traverses": 0.53, "bearers": 1.40}

SUMMARY = T(
    f"A dismantled radiant ceiling heating strip of an industrial hall as a component exchange offers it: a 6.0 m long, 0.9 m wide white powder-coated steel plate of 1.25 mm with 50 mm side upstands, four precision steel water tubes 28 x 1.5 mm pressed into it at 225 mm pitch, two 42.4 mm headers at the ends with the flow and return stubs cut and capped, a 40 mm foil-faced mineral-wool blanket on the back and four galvanised suspension traverses whose chains were cut at the M10 eye nuts, one cut link still hanging in each; drained, yellowed and dusty, {M_TOTAL:.0f} kg with its three 100 x 100 mm softwood bearers. Poses: stored radiant face down on the bearers, lifted on a spreader sling with the bearers left behind, and reinstalled hanging at 6 m in a hall, fed at 75/65 degC and radiating about 2.6 kW.",
    f"Eine ausgebaute Deckenstrahlplatte einer Industriehalle, wie eine Bauteilbörse sie anbietet: ein 6,0 m langes, 0,9 m breites, weiß pulverbeschichtetes Stahlblech 1,25 mm mit seitlichen Aufkantungen von 50 mm, vier Präzisionsstahlrohre 28 x 1,5 mm im Abstand von 225 mm eingepresst, zwei Sammler 42,4 mm an den Enden mit abgetrennten und verschlossenen Vor- und Rücklaufstutzen, eine 40 mm dicke alukaschierte Mineralwollmatte auf der Rückseite und vier verzinkte Aufhängetraversen, deren Ketten an den Ringmuttern M10 abgetrennt wurden, in jeder hängt noch ein abgezwicktes Glied; entleert, vergilbt und staubig, {M_TOTAL:.0f} kg mit ihren drei Nadelholz-Kanthölzern 100 x 100 mm. Posen: mit der Strahlfläche nach unten auf den Kanthölzern gelagert, an einem Traversengehänge gehoben, wobei die Kanthölzer liegen bleiben, und in 6 m Höhe in einer Halle wieder aufgehängt, mit 75/65 °C versorgt und etwa 2,6 kW abstrahlend.")

PROVENANCE = {
    "author": "Semio staffage component authors (used HVAC wave, author 09)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "en-14037-1", "citation": "DIN EN 14037-1:2016 Free hanging heating and cooling surfaces for water with a temperature below 120 degC, Part 1: Pre-fabricated ceiling mounted radiant panels for space heating, technical specifications and requirements (construction from radiant plate, tubes, headers, insulation and suspension; marking)."},
        {"id": "en-14037-2", "citation": "DIN EN 14037-2:2016 Part 2: Test method for thermal output of ceiling mounted radiant panels (standard output at a temperature difference of 55 K between mean water and reference air temperature)."},
        {"id": "radiant-panel-catalogues", "citation": "Catalogue and installation data of water-fed radiant ceiling strips for industrial halls of the 1990s to 2010s (Zehnder ZBN, Frenger, Kabe, Sabiana and similar): coated steel radiant plate 1 to 1.5 mm with folded side upstands, tubes 28 mm or 1/2 inch at 100 to 225 mm pitch pressed or welded into saddles, end headers DN 25 to DN 32, 30 to 40 mm mineral-wool blanket with aluminium foil on the back, suspension traverses every 1.5 to 2 m on chains or threaded rods, widths 300 to 1200 mm, lengths up to 7.5 m per element, 13 to 20 kg/m dry, output about 400 to 550 W/m at 55 K for 900 mm widths, mounting heights 4 to 30 m; the four-tube 900 x 6000 mm strip of this asset is a typical member of the family, not one model (authored-estimate)."},
        {"id": "en-10305-3", "citation": "DIN EN 10305-3:2016 Steel tubes for precision applications, welded cold sized tubes (28 x 1.5 mm: 0.98 kg/m)."},
        {"id": "en-10255", "citation": "DIN EN 10255:2007 Non-alloy steel tubes suitable for welding and threading (DN 32, 42.4 x 2.6 mm, 2.55 kg/m; DN 25, 33.7 x 2.6 mm)."},
        {"id": "din-582", "citation": "DIN 582:2018 Lifting eye nuts (M10: eye inner diameter 25 mm, outer diameter 45 mm, working load limit 2.3 kN in axial pull, about 0.1 kg)."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3 (450 kg/m3 used for the weathered, air-dry bearers), E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (mineral wool lambda 0.035 to 0.040 W/mK, softwood 0.13 W/mK, steel 50 W/mK)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: white enamel and powder paint 0.9 to 0.95, oxidised steel 0.8, dusty aluminium foil 0.2 to 0.3, weathered galvanised steel 0.25 to 0.3, wood 0.9)."},
        {"id": "recknagel-2019", "citation": "Recknagel, Sprenger, Albers: Taschenbuch für Heizung und Klimatechnik, 79th ed., ITM 2019 (radiant ceiling panels in halls: radiant share 60 to 70 percent, design flow temperatures 70 to 90 degC, mounting heights and comfort limits)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation."},
    ],
    "notes": T(
        "Dimensions, tube count and pitch, water content, output and masses are typical values of four-tube 900 mm radiant ceiling strips (authored estimates from the cited catalogues and EN 14037, nothing is measured); the tubes follow EN 10305-3 and EN 10255, the eye nuts DIN 582 and the bearers EN 338. The suspension chains are not part of the asset: they were cut at the eye nuts and only the cut first link hangs in each; the reinstalled poses assume new chains to the hall roof, represented by the eight pinned eye nuts.",
        "Maße, Rohrzahl und -teilung, Wasserinhalt, Leistung und Massen sind typische Werte vierrohriger Deckenstrahlplatten von 900 mm Breite (Schätzwerte nach den zitierten Katalogen und EN 14037, nichts ist gemessen); die Rohre folgen EN 10305-3 und EN 10255, die Ringmuttern DIN 582 und die Kanthölzer EN 338. Die Aufhängeketten gehören nicht zum Asset: sie wurden an den Ringmuttern abgetrennt, in jeder hängt nur noch das abgezwickte erste Glied; die Einbauposen nehmen neue Ketten zum Hallendach an, dargestellt durch die acht gelenkig gehaltenen Ringmuttern."),
}

MATERIALS = [
    {"id": "coated-steel", "title": T("White powder-coated sheet steel 1.25 mm (RAL 9010), yellowed and dusty", "Weiß pulverbeschichtetes Stahlblech 1,25 mm (RAL 9010), vergilbt und staubig"), "emissivity": 0.93, "solarAbsorptivity": 0.35, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel-tube", "title": T("Precision steel tube 28 x 1.5 mm, primed, inside the blanket", "Präzisionsstahlrohr 28 x 1,5 mm, grundiert, in der Matte"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "en-10305-3"},
    {"id": "mineral-wool", "title": T("Mineral-wool blanket 40 mm with aluminium foil facing, dusty and creased", "Mineralwollmatte 40 mm mit Alukaschierung, staubig und zerknittert"), "emissivity": 0.3, "solarAbsorptivity": 0.4, "densityKgM3": 30, "conductivityWmK": 0.04, "specificHeatJKgK": 840, "source": "literature", "reference": "en-iso-10456"},
    {"id": "galvanised-steel", "title": T("Hot-dip galvanised steel, weathered (traverses, eye nuts, cut chain links)", "Feuerverzinkter Stahl, verwittert (Traversen, Ringmuttern, abgezwickte Kettenglieder)"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel", "title": T("Black steel, rusty (cut stubs, caps)", "Schwarzstahl, angerostet (abgetrennte Stutzen, Kappen)"), "emissivity": 0.8, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "softwood", "title": T("Weathered softwood bearers C24", "Verwitterte Nadelholz-Kanthölzer C24"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "aluminium", "title": T("Printed aluminium type label", "Bedrucktes Aluminium-Typenschild"), "emissivity": 0.2, "solarAbsorptivity": 0.3, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
]

CIRCUIT = {"key": "heating", "rows": [{"v": 0.5, "supplyTempC": 55, "returnTempC": 45}, {"v": 1, "supplyTempC": 75, "returnTempC": 65}]}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, model, material, length, geometry, layers, extra, src="authored-estimate", ref="radiant-panel-catalogues"):
    """🔥 One thermal region with its area from AREA and its heat capacity from its layers."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.2,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers), 1), "source": src, "reference": ref}
    out.update(extra)
    return out


THERMAL = {
    "environment": {"airTempC": 15, "meanRadiantTempC": 14, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [
        {"id": "heating", "title": T("Heating water through the strip", "Heizwasser durch die Platte"), "levels": [{"v": 0, "title": T("Drained or cold", "Entleert oder kalt")}, {"v": 0.5, "title": T("Reduced, 55/45 degC", "Abgesenkt, 55/45 °C")}, {"v": 1, "title": T("Design, 75/65 degC (55 K over the 15 degC hall)", "Auslegung, 75/65 °C (55 K über der 15 °C kalten Halle)")}]},
    ],
    "regions": [
        region("plate", "Powder-coated radiant plate with its upstands and end plates", "Pulverbeschichtetes Strahlblech mit Aufkantungen und Stirnblechen", "heater", "hydronic", "coated-steel", 0.9, "horizontal-plate-down",
               [layer("sheet", "Coated sheet steel 1.25 mm", "Beschichtetes Stahlblech 1,25 mm", "metal", "coated-steel", SHEET, 50, 7850, 460, ref="incropera-2018")],
               {"hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.03), "ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("insulation", "Foil-faced mineral-wool blanket on the back", "Alukaschierte Mineralwollmatte auf der Rückseite", "surface", "hydronic", "mineral-wool", 0.9, "horizontal-plate-up",
               [layer("wool", "Mineral wool 40 mm with aluminium foil", "Mineralwolle 40 mm mit Alufolie", "insulation", "mineral-wool", INS, 0.04, 30, 840)],
               {"hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.01)}),
        region("tubes", "Four water tubes 28 x 1.5 mm pressed into the plate", "Vier Wasserrohre 28 x 1,5 mm im Blech eingepresst", "surface", "hydronic", "steel-tube", 0.028, "cylinder",
               [layer("wall", "Steel tube wall 1.5 mm", "Stahlrohrwand 1,5 mm", "metal", "steel-tube", 0.0015, 50, 7850, 460, ref="en-10305-3")],
               {"hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.0005), "hidden": T(
                   "The four tubes lie between the radiant plate and the wool blanket, pressed into the blanket over their whole length; no drawing shows them. Their area is the outer surface of the four tubes of 5.84 m as modelled (2.1 m2).",
                   "Die vier Rohre liegen zwischen Strahlblech und Wollmatte, auf ganzer Länge in die Matte gedrückt; keine Zeichnung zeigt sie. Ihre Fläche ist die Außenfläche der vier Rohre von 5,84 m wie modelliert (2,1 m2).")}),
        region("headers", "Two end headers 42.4 mm with the cut and capped flow and return stubs", "Zwei Endsammler 42,4 mm mit abgetrennten und verschlossenen Vor- und Rücklaufstutzen", "surface", "hydronic", "coated-steel", 0.042, "cylinder",
               [layer("tube", "Steel tube wall 2.6 mm", "Stahlrohrwand 2,6 mm", "metal", "coated-steel", 0.0026, 50, 7850, 460, ref="en-10255")],
               {"hydronic": dict(CIRCUIT, flowPosition=0.5, filmM2KW=0.002)}),
        region("traverses", "Galvanised suspension traverses with eye nuts and cut chain links", "Verzinkte Aufhängetraversen mit Ringmuttern und abgezwickten Kettengliedern", "surface", "passive", "galvanised-steel", 0.04, "horizontal-plate-up",
               [layer("steel", "Galvanised steel 2 mm", "Verzinkter Stahl 2 mm", "metal", "galvanised-steel", 0.002, 50, 7850, 460, ref="incropera-2018")], {}),
        region("bearers", "Three squared softwood bearers", "Drei Nadelholz-Kanthölzer", "surface", "passive", "softwood", 0.1, "horizontal-plate-up",
               [layer("timber", "Softwood, thermally active 20 mm", "Nadelholz, thermisch wirksam 20 mm", "timber", "softwood", 0.02, 0.13, 450, 1600, ref="en-338")], {"source": "literature", "reference": "en-338"}),
    ],
    "emitters": [
        {"id": "convection-plume", "kind": "plume", "title": T("Warm convection plume rising round the edges and over the back of the strip", "Warme Konvektionsfahne, die an den Rändern und über der Rückseite der Platte aufsteigt"), "anchor": "insulation-top", "directionDeg": 90, "diameterM": 0.9, "region": "insulation", "source": "authored-estimate", "reference": "recknagel-2019"},
    ],
    "assumptions": [
        T(f"Stored and lifted, the strip is drained, cold and at the 15 degC of the hall; reinstalled it is refilled with {WATER_L:.1f} l and its water follows the heating circuit (75/65 degC at design, 55/45 degC reduced); the radiant plate, the hidden tubes, the headers and through its 40 mm of wool the blanket take that water through their layers and films.",
          f"Gelagert und gehoben ist die Platte entleert, kalt und hat die 15 °C der Halle; eingebaut ist sie mit {WATER_L:.1f} l befüllt und ihr Wasser folgt dem Heizkreis (75/65 °C Auslegung, 55/45 °C abgesenkt); Strahlblech, verborgene Rohre, Sammler und durch ihre 40 mm Wolle die Matte übernehmen dieses Wasser durch ihre Schichten und Filme."),
        T("The strip is a fed emitter and draws no power (passive, 0 W); its standard output at 55 K (EN 14037-2) is about 430 W/m, 2.6 kW for the 6 m element, of which about two thirds leave the plate as radiation down into the hall and one third as the convection plume over it.",
          "Die Platte ist ein versorgter Heizkörper und nimmt keine Leistung auf (passiv, 0 W); ihre Normleistung bei 55 K (EN 14037-2) beträgt etwa 430 W/m, 2,6 kW für das 6 m lange Element, davon verlassen etwa zwei Drittel das Blech als Strahlung nach unten in die Halle und ein Drittel als Konvektionsfahne darüber."),
        T("Worn surfaces carry the emissivities of their state (Incropera): the yellowed, dusty powder coat 0.93 and solar absorptance 0.35 (about 0.25 when new), the dusty creased aluminium foil 0.3 (about 0.05 when new), weathered galvanising 0.28 (about 0.1 when new).",
          "Gealterte Oberflächen haben die Emissionsgrade ihres Zustands (Incropera): die vergilbte, staubige Pulverbeschichtung 0,93 und solare Absorption 0,35 (neu etwa 0,25), die staubige, zerknitterte Alufolie 0,3 (neu etwa 0,05), verwitterte Verzinkung 0,28 (neu etwa 0,1)."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the {M_PANEL:.0f} kg strip lies with its plate on three 100 x 100 mm softwood bearers 2.4 m apart that rest with their six ends on the floor (contacts, friction 0.5); lifted, the four eye nuts of the two outer traverses, 4.5 m apart, hang it on the spreader sling (pins); reinstalled, all eight eye nuts hang it on new chains from the hall roof (pins, DIN 582 M10, 2.3 kN each in axial pull).",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert liegt die {M_PANEL:.0f} kg schwere Platte mit ihrem Blech auf drei Nadelholz-Kanthölzern 100 x 100 mm im Abstand von 2,4 m, die mit ihren sechs Enden auf dem Boden liegen (Kontakte, Reibung 0,5); gehoben hängen sie die vier Ringmuttern der beiden äußeren Traversen, 4,5 m auseinander, ins Traversengehänge (Gelenke); eingebaut hängen sie alle acht Ringmuttern an neuen Ketten vom Hallendach (Gelenke, DIN 582 M10, je 2,3 kN axial)."),
    T(f"The solver treats the whole asset as one rigid body: in the lifted and reinstalled poses the {M_BEARER:.1f} kg of bearers left on the floor are counted in the sling or chain reactions ({100 * M_BEARER / M_TOTAL:.1f} % of the asset weight).",
      f"Der Löser behandelt das ganze Asset als einen starren Körper: in der gehobenen und den Einbauposen gehen die {M_BEARER:.1f} kg der am Boden liegenden Kanthölzer in die Reaktionen von Gehänge oder Ketten ein ({100 * M_BEARER / M_TOTAL:.1f} % des Assetgewichts)."),
    T(f"The strip is one composite member: plate 900 x 1.25 mm, two 50 mm upstands and four tubes 28 x 1.5 mm, A {SEC_A * 1e4:.1f} cm2, I {SEC_I * 1e8:.1f} cm4; the four 40 x 20 x 2 mm traverses carry it into the eye nuts. Reinstalled it carries {WATER_L:.1f} l of water ({WATER_L * 0.98 * 9.81:.0f} N) and the two flexible hoses (50 N each).",
      f"Die Platte ist ein zusammengesetzter Stab: Blech 900 x 1,25 mm, zwei Aufkantungen 50 mm und vier Rohre 28 x 1,5 mm, A {SEC_A * 1e4:.1f} cm2, I {SEC_I * 1e8:.1f} cm4; die vier Traversen 40 x 20 x 2 mm leiten sie in die Ringmuttern. Eingebaut trägt sie {WATER_L:.1f} l Wasser ({WATER_L * 0.98 * 9.81:.0f} N) und die zwei Schläuche (je 50 N)."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(
        ("Side elevation from the right, 120 mm long and 4.5 mm high on paper: the 50 mm upstand of the 6 m plate on three sawn bearer ends, the four traverses on top with their eye nuts and cut links, the two capped stubs at the ends. Front, 22 x 4.5 mm: plate, upstands, end plate, traverse with both eye nuts and links, the flow stub, over the 1.1 m bearer. Plan, 120 x 22 mm: the foil-faced blanket between the two headers, upstands and end plates, the four traverses with eye nuts and links, the two stubs and the bearer ends beyond the plate.",
         "Seitenansicht von rechts, auf Papier 120 mm lang und 4,5 mm hoch: die 50 mm hohe Aufkantung des 6 m langen Blechs auf drei gesägten Kantholzenden, die vier Traversen oben mit Ringmuttern und abgezwickten Gliedern, die zwei verschlossenen Stutzen an den Enden. Vorderansicht, 22 x 4,5 mm: Blech, Aufkantungen, Stirnblech, Traverse mit beiden Ringmuttern und Gliedern, der Vorlaufstutzen, über dem 1,1 m langen Kantholz. Grundriss, 120 x 22 mm: die alukaschierte Matte zwischen den zwei Sammlern, Aufkantungen und Stirnbleche, die vier Traversen mit Ringmuttern und Gliedern, die zwei Stutzen und die über das Blech ragenden Kantholzenden."),
        ("Every part at its true size: the 1.25 mm sheet of plate, upstands and end plates, the 42.4 mm headers, the 33.7 mm stubs and their 44 mm caps, the 40 x 20 mm traverses, the 45 mm eye nuts with their 25 mm bore and the 26 mm cut links, the type label, the lip fold, the blanket joints and the foil tape, dust streaks along the lower edge.",
         "Jedes Teil in wahrer Größe: das 1,25 mm Blech von Strahlfläche, Aufkantungen und Stirnblechen, die Sammler 42,4 mm, die Stutzen 33,7 mm mit ihren Kappen 44 mm, die Traversen 40 x 20 mm, die Ringmuttern 45 mm mit ihrer Öffnung 25 mm und die abgezwickten Glieder 26 mm, das Typenschild, die Lippenkante, die Mattenstöße und das Aluband, Staubstreifen an der Unterkante."),
        ("The far upstand, the far eye nuts and the two far bearers lie behind the near ones in elevation and front; the headers are hidden behind the end plates in front and drawn in plan.",
         "Die hintere Aufkantung, die hinteren Ringmuttern und die zwei hinteren Kanthölzer liegen in Seiten- und Vorderansicht hinter den vorderen; die Sammler sind in der Vorderansicht hinter den Stirnblechen verborgen und im Grundriss gezeichnet."),
        ("The four water tubes inside the blanket are not drawn: blanket, plate and upstands hide them.", "Die vier Wasserrohre in der Matte sind nicht gezeichnet: Matte, Blech und Aufkantungen verbergen sie."),
        ("The 1.25 mm sheet is 0.025 mm on paper and drawn as the edge of its shape; the 6 mm link wire is 0.12 mm, the 10 mm eye nut 0.2 mm.",
         "Das 1,25 mm Blech misst auf Papier 0,025 mm und ist als Kante seiner Form gezeichnet; der 6 mm Gliederdraht misst 0,12 mm, die 10 mm Ringmutter 0,2 mm."),
        ("The sawn ends of the bearers carry a 45 degree end-grain hatch in the side elevation.", "Die gesägten Kantholzenden tragen in der Seitenansicht eine Hirnholzschraffur unter 45 Grad."),
        ("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm; tones: plate 0.86, blanket 0.7, bearers 0.62, traverses 0.5, stubs 0.4, eye nuts and links 0.3 to 0.35.",
         "Körperstift 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Töne: Blech 0,86, Matte 0,7, Kanthölzer 0,62, Traversen 0,5, Stutzen 0,4, Ringmuttern und Glieder 0,3 bis 0,35."),
        ("Every shape names its thermal region; the hidden tubes have none.", "Jede Form nennt ihre Wärmeregion; die verborgenen Rohre haben keine."),
    ),
    100: D(
        ("Side elevation 60 x 2.3 mm, front 11 x 2.3 mm, plan 60 x 11 mm: plate with upstands, traverses, eye nuts as discs and the capped stubs on the three bearers.",
         "Seitenansicht 60 x 2,3 mm, Vorderansicht 11 x 2,3 mm, Grundriss 60 x 11 mm: Blech mit Aufkantungen, Traversen, Ringmuttern als Scheiben und die verschlossenen Stutzen auf den drei Kanthölzern."),
        ("The 1.25 mm sheets as declared sheets, the 40 x 20 mm traverses, the 45 mm eye nuts as 16 mm discs, each stub with its cap one 44 mm cylinder, the headers in plan.",
         "Die 1,25 mm Bleche als erklärte Bleche, die Traversen 40 x 20 mm, die Ringmuttern 45 mm als Scheiben von 16 mm, jeder Stutzen mit seiner Kappe ein Zylinder von 44 mm, die Sammler im Grundriss."),
        ("Stub and cap are one cylinder; the eye nut loses its bore.", "Stutzen und Kappe sind ein Zylinder; die Ringmutter verliert ihre Öffnung."),
        ("Cut chain links, type label, dust, blanket joints, foil tape and bearer grain.", "Abgezwickte Kettenglieder, Typenschild, Staub, Mattenstöße, Aluband und Holzmaserung."),
        ("The 20 mm traverse is 0.2 mm on paper, the 1.25 mm sheet an edge.", "Die 20 mm hohe Traverse misst 0,2 mm auf Papier, das 1,25 mm Blech eine Kante."),
        ("None.", "Keine."),
        ("Body pen 0.25 mm, detail 0.13 mm; the tones of 1:50.", "Körperstift 0,25 mm, Details 0,13 mm; die Töne von 1:50."),
        ("Every shape names its thermal region.", "Jede Form nennt ihre Wärmeregion."),
    ),
    200: D(
        ("Side elevation 30 x 0.9 mm, front 5.5 x 0.9 mm, plan 30 x 5.5 mm: the trough of the strip as one 50 mm band, the four traverses as 30 mm bars, the two capped stubs, the three bearers.",
         "Seitenansicht 30 x 0,9 mm, Vorderansicht 5,5 x 0,9 mm, Grundriss 30 x 5,5 mm: der Trog der Platte als ein Band von 50 mm, die vier Traversen als Balken von 30 mm, die zwei verschlossenen Stutzen, die drei Kanthölzer."),
        ("Trough, traverses, stubs and bearers, all at least 30 mm.", "Trog, Traversen, Stutzen und Kanthölzer, alles mindestens 30 mm."),
        ("Plate, upstands, end plates, blanket and headers are one trough box 6.0 x 0.05 x 0.9 m; the 20 mm traverses are drawn and modelled 30 mm high; each stub with its cap is one 44 mm cylinder.",
         "Blech, Aufkantungen, Stirnbleche, Matte und Sammler sind ein Trogkasten 6,0 x 0,05 x 0,9 m; die 20 mm hohen Traversen sind 30 mm hoch gezeichnet und modelliert; jeder Stutzen ist mit seiner Kappe ein Zylinder von 44 mm."),
        ("Eye nuts, cut links and every seam and texture.", "Ringmuttern, abgezwickte Glieder und alle Fugen und Texturen."),
        ("No part below 30 mm (0.15 mm on paper) remains; the traverses are 10 mm higher than real (0.05 mm on paper).", "Kein Teil unter 30 mm (0,15 mm auf Papier) bleibt; die Traversen sind 10 mm höher als in Wirklichkeit (0,05 mm auf Papier)."),
        ("None.", "Keine."),
        ("One body pen of 0.18 mm; the tones of 1:50.", "Ein Körperstift von 0,18 mm; die Töne von 1:50."),
        ("The trough shape lists plate, blanket and headers as merged regions.", "Die Trogform führt Blech, Matte und Sammler als zusammengeführte Regionen."),
    ),
    500: D(
        ("Side elevation 12 x 0.35 mm, front 2.2 x 0.35 mm, plan 12 x 2.2 mm: the strip as one 75 mm band on the three 100 mm bearers.",
         "Seitenansicht 12 x 0,35 mm, Vorderansicht 2,2 x 0,35 mm, Grundriss 12 x 2,2 mm: die Platte als ein Band von 75 mm auf den drei Kanthölzern von 100 mm."),
        ("The strip band and the three bearers.", "Das Plattenband und die drei Kanthölzer."),
        ("Plate, blanket, headers and traverses are one 6.0 x 0.075 x 0.9 m band, 5 mm higher than the 70 mm of trough and traverses (0.01 mm on paper).",
         "Blech, Matte, Sammler und Traversen sind ein Band 6,0 x 0,075 x 0,9 m, 5 mm höher als die 70 mm von Trog und Traversen (0,01 mm auf Papier)."),
        ("Eye nuts, links and the traverses as separate bars; each capped stub is one 90 mm block reaching the 0.225 m top of the eye nuts and links, so the drawing keeps the real height and the stubs print over the whole range.", "Ringmuttern, Glieder und die Traversen als eigene Balken; jeder verschlossene Stutzen ist ein Block von 90 mm bis zur Oberkante 0,225 m der Ringmuttern und Glieder, damit die Zeichnung die wahre Höhe behält und die Stutzen über den ganzen Bereich drucken."),
        ("No part below 75 mm (0.15 mm on paper) remains.", "Kein Teil unter 75 mm (0,15 mm auf Papier) bleibt."),
        ("None.", "Keine."),
        ("One body pen of 0.13 mm; the tones of 1:50.", "Ein Körperstift von 0,13 mm; die Töne von 1:50."),
        ("The band lists plate, blanket, headers and traverses as merged regions.", "Das Band führt Blech, Matte, Sammler und Traversen als zusammengeführte Regionen."),
    ),
}

DECISIONS_3D = {
    50: [T("Body: every part of the 1:50 drawing as a solid: the radiant plate, the two upstands and the two end plates as declared 1.25 mm sheets, the wool blanket as one box, the four tubes as slender revolves inside it, the two headers and the stubs with caps as revolves, the traverses as 40 x 20 mm boxes, the eye nuts as rings with their bore, the cut chain links as slender hoops through the eyes, the bearers with 4 mm arrises and the type label as a declared texture sheet.",
           "Körper: jedes Teil der Zeichnung 1:50 als Körper: Strahlblech, die zwei Aufkantungen und die zwei Stirnbleche als erklärte Bleche von 1,25 mm, die Wollmatte als ein Quader, die vier Rohre als schlanke Drehkörper darin, die zwei Sammler und die Stutzen mit Kappen als Drehkörper, die Traversen als Quader 40 x 20 mm, die Ringmuttern als Ringe mit Öffnung, die abgezwickten Kettenglieder als schlanke Ringe durch die Ösen, die Kanthölzer mit 4 mm Fasen und das Typenschild als erklärtes Texturblech."),
         T("Thermal: plate, blanket, headers, stubs, traverses, eye nuts, links and bearers by reference to their body solids, the four tubes as the hidden region of their own revolves, the plume patch on the blanket top. Structure: bearers and traverses as members, the strip as one composite member box with the four tubes as slender members in it, three bearing pads as joints, two mass markers.",
           "Wärme: Blech, Matte, Sammler, Stutzen, Traversen, Ringmuttern, Glieder und Kanthölzer als Verweise auf ihre Körper, die vier Rohre als verborgene Region eigener Drehkörper, das Fahnenfeld auf der Mattenoberseite. Tragwerk: Kanthölzer und Traversen als Stäbe, die Platte als ein zusammengesetzter Stabquader mit den vier Rohren als schlanken Stäben darin, drei Auflagerpolster als Gelenke, zwei Massenmarken.")],
    100: [T("Body for the 15 mm feature: the sheets stay declared sheets, the eye nuts are 45 mm discs 16 mm thick without bore, each stub with its cap one 44 mm cylinder; the cut links and the type label are left out.",
            "Körper für das Merkmal von 15 mm: die Bleche bleiben erklärte Bleche, die Ringmuttern sind Scheiben von 45 mm und 16 mm Dicke ohne Öffnung, jeder Stutzen mit Kappe ein Zylinder von 44 mm; die abgezwickten Glieder und das Typenschild entfallen."),
          T("Thermal and structure as at 1:50 without the links.", "Wärme und Tragwerk wie bei 1:50 ohne die Glieder.")],
    200: [T("Body for the 30 mm feature: plate, upstands, end plates, blanket, tubes and headers are one trough box 50 mm high, the traverses 30 mm high bars, each stub with its cap one 44 mm cylinder; eye nuts and links are left out.",
            "Körper für das Merkmal von 30 mm: Blech, Aufkantungen, Stirnbleche, Matte, Rohre und Sammler sind ein Trogkasten von 50 mm Höhe, die Traversen Balken von 30 mm Höhe, jeder Stutzen mit Kappe ein Zylinder von 44 mm; Ringmuttern und Glieder entfallen."),
          T("Thermal: the plate as its five sheet shells, the blanket and the headers as own solids, the tubes as one hidden 28 mm water layer, the other regions by reference. Structure: bearers, the trough and the traverses as members, 30 mm bearing pads, mass markers.",
            "Wärme: das Blech als seine fünf Blechschalen, Matte und Sammler als eigene Körper, die Rohre als eine verborgene Wasserschicht von 28 mm, die anderen Regionen als Verweise. Tragwerk: Kanthölzer, Trog und Traversen als Stäbe, Auflagerpolster von 30 mm, Massenmarken.")],
    500: [T("Body for the 75 mm feature: the strip with its traverses is one 75 mm band on the three 100 mm bearers, each capped stub a 90 mm block to the 0.225 m top; eye nuts and links are left out.",
            "Körper für das Merkmal von 75 mm: die Platte mit ihren Traversen ist ein Band von 75 mm auf den drei Kanthölzern von 100 mm, jeder verschlossene Stutzen ein Block von 90 mm bis zur Oberkante 0,225 m; Ringmuttern und Glieder entfallen."),
          T("Thermal: the plate as its five sheet shells, blanket, hidden water layer, headers and traverses as layer shells of their regions, the bearers by reference. Structure: bearers and the band as members, 75 mm bearing pads, mass markers.",
            "Wärme: das Blech als seine fünf Blechschalen, Matte, verborgene Wasserschicht, Sammler und Traversen als Schichtschalen ihrer Regionen, die Kanthölzer als Verweise. Tragwerk: Kanthölzer und Band als Stäbe, Auflagerpolster von 75 mm, Massenmarken.")],
}
