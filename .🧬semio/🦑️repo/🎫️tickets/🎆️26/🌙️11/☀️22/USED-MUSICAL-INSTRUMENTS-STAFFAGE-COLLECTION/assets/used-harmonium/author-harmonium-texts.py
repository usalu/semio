"""📜 Texts, provenance, thermal model, structure and decisions of the used harmonium, executed into the namespace of author-harmonium.py."""

AREA = {"case": 9.16, "keys": 0.41, "treadles": 0.275, "bellows": 1.38}
if os.environ.get("HARMONIUM_AREA_PROBE"):
    AREA = {k: 0.01 for k in AREA}

SUMMARY = T(
    f"A used German suction-wind pedal harmonium of the 1950s as a used-instrument dealer offers it: a walnut-veneered case 1.20 m wide, 0.53 m deep and 1.15 m high with a carved crest, five octaves C to c (61 keys, white key tops 0.72 m above the floor), ten stop knobs, a fall board and a folding music desk; two carpeted treadles pump two exhausters under the reservoir by leather straps. Use has left yellowed and one chipped key covering, carpets worn through and torn at the toe, shellac rubbed off the key cheeks and the kneeboard, a water ring and crazing on the lid, a broken scroll on the crest and the dealer's inventory label; {M_TOTAL:.0f} kg, cleaned and playable.",
    f"Ein gebrauchtes deutsches Saugwind-Harmonium mit Tretbälgen aus den 1950er Jahren, wie es ein Gebrauchtinstrumentenhandel anbietet: ein nussbaumfurniertes Gehäuse von 1,20 m Breite, 0,53 m Tiefe und 1,15 m Höhe mit geschnitztem Aufsatz, fünf Oktaven C bis c (61 Tasten, Oberkante der Untertasten 0,72 m über dem Boden), zehn Registerknöpfe, eine Tastenklappe und ein klappbares Notenpult; zwei mit Teppich belegte Tritte bewegen über Lederriemen zwei Schöpfbälge unter dem Magazinbalg. Der Gebrauch hat vergilbte und einen abgeplatzten Tastenbelag, durchgetretene und an der Spitze eingerissene Teppiche, an den Backen und am Kniebrett abgeriebene Politur, einen Wasserrand und Haarrisse auf dem Deckel, eine abgebrochene Volute am Aufsatz und das Inventaretikett des Händlers hinterlassen; {M_TOTAL:.0f} kg, gereinigt und spielbar.")

PROVENANCE = {
    "author": "Semio staffage object authors (used instruments wave, author 13)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "hornbostel-sachs-1914", "citation": "Hornbostel, E. M. von; Sachs, C. (1914): Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590: class 412.132, free aerophones, interruptive, sets of free reeds (harmonium, reed organ, accordion)."},
        {"id": "ord-hume-1986", "citation": "Ord-Hume, A. W. J. G. (1986): Harmonium. The History of the Reed Organ and its Makers. David & Charles, Newton Abbot: suction and pressure harmoniums, case types, stops, reed cells and reed pans, exhausters and reservoir, typical weights of the domestic instrument."},
        {"id": "gellerman-1996", "citation": "Gellerman, R. F. (1996): The American Reed Organ and the Harmonium. Vestal Press, Vestal NY: suction action with two exhausters under the reservoir worked by treadle straps, treadle stroke and pumping rate, wind of a few hundred pascals, carpeted treadles, maintenance of bellows cloth and straps."},
        {"id": "harmonium-catalogues", "citation": "Catalogue data of German suction-wind house and school harmoniums of the 1920s to 1960s from Saxon and Württemberg harmonium workshops (types, no single model): five octaves C to c, two to four reed sets, eight to fourteen stops, cases 1.15 to 1.25 m wide, 0.50 to 0.60 m deep, 1.00 to 1.25 m high with the crest, 60 to 110 kg (authored-estimate)."},
        {"id": "keyboard-dimensions", "citation": "Catalogue-typical keyboard dimensions of pianos and harmoniums: octave span 164.5 mm, natural key 23.5 mm wide with about 150 mm visible length, sharp 10 mm wide, 95 mm long and 12 mm above the naturals, natural key tops 0.70 to 0.74 m above the floor for a seated player (authored-estimate)."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, tabulated design values (softwood 450 to 500 kg/m3 and 0.12 to 0.13 W/mK, hardwood 700 kg/m3 and 0.18 W/mK, textile floor coverings 0.06 W/mK, specific heat of wood 1600 J/kgK)."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 to 480 kg/m3, E0,mean 11 GPa, fm,k 24 MPa."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: wood and lacquer 0.9, paper 0.9, oxidised iron 0.7, plastics 0.9 to 0.95; leather and celluloid properties)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections (hatching)."},
    ],
    "notes": T(
        "Dimensions, the case and action layout, stop count and masses are typical values of the German suction-wind harmonium of the 1950s (authored estimates from the cited literature and catalogues, nothing is measured); the masses are the volumes of the modelled parts times the densities of their materials, the reed action and the reservoir with effective densities for their hollow cells. The player is not part of the asset: the played poses carry the feet and hands as loads.",
        "Maße, Aufbau von Gehäuse und Werk, Registerzahl und Massen sind typische Werte des deutschen Saugwind-Harmoniums der 1950er Jahre (Schätzwerte nach der zitierten Literatur und den Katalogen, nichts ist gemessen); die Massen sind die Volumen der modellierten Teile mal die Dichten ihrer Werkstoffe, Zungenwerk und Magazinbalg mit wirksamen Dichten für ihre hohlen Kanzellen. Der Spieler gehört nicht zum Asset: die Spielposen tragen Füße und Hände als Lasten."),
}


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, material, length, geometry, layers, extra, src="authored-estimate", ref="ord-hume-1986"):
    """🔥 One passive thermal region with its area from AREA and its heat capacity from its layers."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": "passive", "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.0,
           "layers": layers, "heatCapacityJM2K": round(sum(l["heatCapacityJM2K"] for l in layers), 1), "source": src, "reference": ref}
    out.update(extra)
    return out


THERMAL = {
    "environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "pumping", "title": T("Treadles pumped", "Tritte getreten"), "levels": [{"v": 0, "title": T("Silent, nobody pumping", "Still, niemand tritt")}, {"v": 1, "title": T("Pumped at 50 strokes a minute per foot", "Getreten mit 50 Hüben je Fuß in der Minute")}]}],
    "regions": [
        region("case", "Walnut-veneered case with crest, lid, desk, fall, stop rail and knobs", "Nussbaumfurniertes Gehäuse mit Aufsatz, Deckel, Pult, Klappe, Registerleiste und Knöpfen", "surface", "veneered-softwood", 0.53, "vertical-plate",
               [layer("board", "Spruce board with walnut veneer and aged shellac 22 mm", "Fichtenplatte mit Nussbaumfurnier und gealterter Politur 22 mm", "timber", "veneered-softwood", 0.022, 0.12, 470, 1600)],
               {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("keys", "Yellowed celluloid naturals and ebonised sharps", "Vergilbte Zelluloid-Untertasten und schwarz gebeizte Obertasten", "surface", "celluloid", 0.15, "horizontal-plate-up",
               [layer("key", "Lime-wood key with 1.5 mm celluloid covering 30 mm", "Lindenholztaste mit 1,5 mm Zelluloidbelag 30 mm", "timber", "celluloid", 0.03, 0.17, 560, 1500, ref="incropera-2018")], {}),
        region("treadles", "Carpeted beech treadles", "Mit Teppich belegte Buchentritte", "surface", "wool-carpet", 0.16, "horizontal-plate-up",
               [layer("carpet", "Worn wool carpet 6 mm", "Abgetretener Wollteppich 6 mm", "insulation", "wool-carpet", 0.006, 0.06, 300, 1300)], {}),
        region("bellows", "Reservoir and exhausters with rubberised cloth, boards and straps", "Magazinbalg und Schöpfbälge mit gummiertem Tuch, Brettern und Riemen", "surface", "rubber-cloth", 0.3, "horizontal-plate-down",
               [layer("cloth-board", "Rubberised cloth over pine board 13 mm", "Gummiertes Tuch über Kiefernbrett 13 mm", "timber", "pine", 0.013, 0.13, 480, 1600, ref="gellerman-1996")], {}),
    ],
    "emitters": [
        {"id": "exhaust-left", "kind": "vent", "title": T("Air expelled through the valve of the left exhauster", "Durch das Ventil des linken Schöpfbalgs ausgestoßene Luft"), "anchor": "exhaust-left", "directionDeg": 90, "diameterM": 0.05, "operatingKey": "pumping",
         "volumeFlowM3S": 0.002, "temperatureRiseK": 0, "vapourKgPerKg": 0, "source": "authored-estimate", "reference": "gellerman-1996"},
        {"id": "exhaust-right", "kind": "vent", "title": T("Air expelled through the valve of the right exhauster", "Durch das Ventil des rechten Schöpfbalgs ausgestoßene Luft"), "anchor": "exhaust-right", "directionDeg": 90, "diameterM": 0.05, "operatingKey": "pumping",
         "volumeFlowM3S": 0.002, "temperatureRiseK": 0, "vapourKgPerKg": 0, "source": "authored-estimate", "reference": "gellerman-1996"},
    ],
    "assumptions": [
        T("The harmonium is an acoustic, passive instrument: it draws no power (0 W, stated as passive on the case) and every region stays near the 18 degC of the showroom; while it is played, fingers at 32 degC touch 5 % of the key tops and shoe soles at 26 degC 30 % of the carpets.",
          "Das Harmonium ist ein akustisches, passives Instrument: es nimmt keine Leistung auf (0 W, als passiv am Gehäuse angegeben), und jede Region bleibt nahe den 18 °C des Ladens; beim Spielen berühren Finger mit 32 °C 5 % der Tastenoberflächen und Schuhsohlen mit 26 °C 30 % der Teppiche."),
        T("The wind is room air: the reeds draw it in through the action into the reservoir at a suction of a few hundred pascals, the exhausters empty the reservoir and expel about 2 l/s through their flap valves at room temperature while their treadle rises; the air is neither warmed nor moistened.",
          "Der Wind ist Raumluft: die Zungen saugen sie durch das Werk in den Magazinbalg bei einem Unterdruck von einigen hundert Pascal, die Schöpfbälge leeren den Magazinbalg und stoßen, während ihr Tritt steigt, etwa 2 l/s mit Raumtemperatur durch ihre Klappenventile aus; die Luft wird weder erwärmt noch befeuchtet."),
        T("Aged surfaces: the darkened shellac absorbs 0.78 of the solar radiation instead of about 0.65 when new, the yellowed celluloid 0.45 instead of 0.3; emissivities of wood, lacquer, textile and cloth 0.9 (Incropera).",
          "Gealterte Oberflächen: die nachgedunkelte Schellackpolitur absorbiert 0,78 der Sonnenstrahlung statt etwa 0,65 im Neuzustand, das vergilbte Zelluloid 0,45 statt 0,3; Emissionsgrade von Holz, Lack, Textil und Tuch 0,9 (Incropera)."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Standing, the {M_TOTAL:.0f} kg harmonium rests on the four feet of its two side panels (contacts 90 x 22 mm, friction 0.45); carried, the two movers hold it by the drop handles {HANDLE_X:.3f} m in front of its back face and 0.71 m up, above its centre of mass at x {COM[0]:.3f} m, so the two pins on one lateral line carry it without tipping.",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Stehend ruht das {M_TOTAL:.0f} kg schwere Harmonium auf den vier Füßen seiner zwei Seitenwände (Kontakte 90 x 22 mm, Reibung 0,45); getragen halten es die zwei Träger an den Klappgriffen {HANDLE_X:.3f} m vor der Rückseite und 0,71 m hoch, über seinem Schwerpunkt bei x {COM[0]:.3f} m, so tragen die zwei Gelenke auf einer Querachse es ohne zu kippen."),
    T("Played, the pressing foot loads the ball of the foot on the carpet with 130 N (wind of about 0.5 kPa on a 0.21 x 0.50 m exhauster plus its return spring, through the strap lever of 0.34 m against the foot lever of 0.22 m), the resting foot with 30 N and each hand the keys with 10 N; the treadles are beech boards hinged at the heel, the exhausters and straps are idealised as rigid members.",
      "Gespielt belastet der tretende Fuß den Teppich am Fußballen mit 130 N (Wind von etwa 0,5 kPa auf einen Schöpfbalg 0,21 x 0,50 m und seine Rückstellfeder, über den Riemenhebel von 0,34 m gegen den Fußhebel von 0,22 m), der ruhende Fuß mit 30 N und jede Hand die Tasten mit 10 N; die Tritte sind an der Ferse angeschlagene Buchenbretter, Schöpfbälge und Riemen sind als starre Stäbe idealisiert."),
    T("The carcass of side panels, back, bottom, keybed, stop rail and reed action is one rigid member of the case; its glued and screwed joints are not modelled, the old glue joints are taken as sound after the dealer's overhaul.",
      "Der Korpus aus Seitenwänden, Rückwand, Boden, Klaviaturboden, Registerleiste und Zungenwerk ist ein starres Bauteil des Gehäuses; seine Leim- und Schraubverbindungen sind nicht modelliert, die alten Leimfugen gelten nach der Überholung durch den Händler als intakt."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(("Side elevation as section A-A 105 mm left of the centre plane through the left treadle, its strap and exhauster, the reservoir, the reed action and the keys, looking at the inside of the left side panel; front elevation from the player's place with keyboard, stops, kneeboard and the treadle well; plan of the lid with the folded desk, the crest and the fall.",
           "Seitenansicht als Schnitt A-A 105 mm links der Mittelebene durch den linken Tritt, seinen Riemen und Schöpfbalg, den Magazinbalg, das Zungenwerk und die Tasten, mit Blick auf die Innenseite der linken Seitenwand; Vorderansicht vom Platz des Spielers mit Klaviatur, Registern, Kniebrett und Trittmulde; Draufsicht auf den Deckel mit geklapptem Pult, Aufsatz und Klappe."),
          ("Every board at its true thickness, the 61 keys with their 25 sharps, ten stop knobs, the scrolled key cheeks, the two feet of each side panel, both treadles with their carpets, the strap and the opened gussets of the exhausters.",
           "Jedes Brett in wahrer Stärke, die 61 Tasten mit ihren 25 Obertasten, zehn Registerknöpfe, die geschwungenen Backen, die zwei Füße jeder Seitenwand, beide Tritte mit Teppich, der Riemen und die geöffneten Falten der Schöpfbälge."),
          ("The reed pan with its cells, pallets and stop mutes is one action block with its boards drawn as lines; the reservoir springs and valves are merged into the reservoir.",
           "Kanzellenbrett mit Kanzellen, Ventilen und Registerklappen ist ein Werkblock, dessen Bretter als Linien gezeichnet sind; Federn und Ventile des Magazinbalgs gehen im Magazinbalg auf."),
          ("Single reeds, the swell shutters and the knee levers, the screws and the treadle return springs.",
           "Einzelne Zungen, die Schwellerjalousie und die Kniehebel, die Schrauben und die Rückholfedern der Tritte."),
          ("The 1 mm key gaps, the porcelain stop faces, the wood grain and the traces of use are texture strokes; the 6 mm carpet and the 8 mm strap are filled at true size.",
           "Die 1-mm-Tastenfugen, die Porzellanschilder der Register, die Holzmaserung und die Gebrauchsspuren sind Texturstriche; der 6-mm-Teppich und der 8-mm-Riemen sind in wahrer Größe gefüllt."),
          ("Cut timber is hatched at 45 degrees, cut cloth, leather and ebonised wood cross-hatched (DIN ISO 128-50); parts beyond the cut are toned without hatch.",
           "Geschnittenes Holz ist unter 45 Grad schraffiert, geschnittenes Tuch, Leder und gebeiztes Holz kreuzschraffiert (DIN ISO 128-50); Teile hinter dem Schnitt sind ohne Schraffur getönt."),
          ("Dark walnut 0.36 to 0.44, yellowed naturals 0.88, sharps and knobs 0.1, carpet 0.42, bellows cloth 0.2; cut outlines 0.35 mm, parts beyond 0.18 mm, texture 0.1 mm.",
           "Dunkles Nussbaum 0,36 bis 0,44, vergilbte Untertasten 0,88, Obertasten und Knöpfe 0,1, Teppich 0,42, Balgtuch 0,2; Schnittkonturen 0,35 mm, Teile dahinter 0,18 mm, Textur 0,1 mm."),
          ("Traces of use: rubbed shellac on both key cheeks and the knee marks on the kneeboard, the torn carpet corner and the worn ball-of-foot zone, a chipped key covering, the water ring and crazing on the lid, the broken scroll of the crest and the inventory label.",
           "Gebrauchsspuren: abgeriebene Politur an beiden Backen und Kniespuren am Kniebrett, die eingerissene Teppichecke und die abgetretene Fußballenzone, ein abgeplatzter Tastenbelag, Wasserrand und Haarrisse auf dem Deckel, die abgebrochene Volute des Aufsatzes und das Inventaretikett.")),
    100: D(("The same section A-A and views with fewer points in the cheek scroll and the crest outline.",
            "Derselbe Schnitt A-A und dieselben Ansichten mit weniger Punkten in der Backenvolute und im Umriss des Aufsatzes."),
           ("Side panels with feet and cheeks, keyboard block, cheek blocks, name board, stop rail with ten knobs, crest, fall, lid and desk, both treadles, exhausters, gussets and straps.",
            "Seitenwände mit Füßen und Backen, Tastenblock, Backenklötze, Namensbrett, Registerleiste mit zehn Knöpfen, Aufsatz, Klappe, Deckel und Pult, beide Tritte, Schöpfbälge, Falten und Riemen."),
           ("The sharps merge into the key block, carpet and treadle board into one 28 mm tread, the desk ledge stays as a 20 mm strip.",
            "Die Obertasten gehen im Tastenblock auf, Teppich und Trittbrett in einen 28-mm-Tritt, die Pultleiste bleibt als 20-mm-Leiste."),
           ("Key divisions, stop faces, grain, the inventory label and the small traces of use; the reed cells.",
            "Tastenteilung, Registerschilder, Maserung, Inventaretikett und die kleinen Gebrauchsspuren; die Kanzellen."),
           ("The 15 mm back is a declared sheet at its true thickness, fall and strap are drawn 15 mm, the gussets 15 mm.",
            "Die 15-mm-Rückwand ist ein erklärtes Blech in wahrer Stärke, Klappe und Riemen sind 15 mm, die Falten 15 mm gezeichnet."),
           ("No hatching; the cut is read by the tone and the heavier outline.", "Keine Schraffur; der Schnitt liest sich über den Ton und die kräftigere Kontur."),
           ("Same tones as 1:50; outlines 0.25 mm, parts beyond 0.13 mm.", "Dieselben Töne wie 1:50; Konturen 0,25 mm, Teile dahinter 0,13 mm."),
           ("Panel lines of the kneeboard and the name board and the veneer seam of the lid stay as details.", "Füllungslinien von Kniebrett und Namensbrett und die Furnierfuge des Deckels bleiben als Details.")),
    200: D(("Plain side elevation of the right side panel with the knobs, lid edge and crest beyond it; front elevation with the key block, the kneeboard and the treadle well; plan of lid, crest and fall.",
            "Einfache Seitenansicht der rechten Seitenwand mit Knöpfen, Deckelkante und Aufsatz dahinter; Vorderansicht mit Tastenblock, Kniebrett und Trittmulde; Draufsicht von Deckel, Aufsatz und Klappe."),
           ("Side panels with the cheek arm, kneeboard, key block, cheek blocks, stop rail, crest, fall, lid, desk and the two treadles; the exhausters under the action.",
            "Seitenwände mit Backenarm, Kniebrett, Tastenblock, Backenklötze, Registerleiste, Aufsatz, Klappe, Deckel, Pult und die zwei Tritte; die Schöpfbälge unter dem Werk."),
           ("Reed action, reservoir and name board are one block, key slip and keybed one board, the drop handles and the ten knobs are left out at 26 to 32 mm.",
            "Zungenwerk, Magazinbalg und Namensbrett sind ein Block, Vorsatzleiste und Klaviaturboden ein Brett, die Klappgriffe und die zehn Knöpfe entfallen bei 26 bis 32 mm."),
           ("The section, the feet cut-out, every detail and texture stroke and the traces of use.", "Der Schnitt, der Fußausschnitt, alle Detail- und Texturstriche und die Gebrauchsspuren."),
           ("Side panels, back, fall, lid, desk, treadles and straps are drawn 30 mm, the smallest printable feature.", "Seitenwände, Rückwand, Klappe, Deckel, Pult, Tritte und Riemen sind 30 mm gezeichnet, das kleinste druckbare Merkmal."),
           ("None.", "Keine."),
           ("Tones as 1:50; outlines 0.18 mm.", "Töne wie 1:50; Konturen 0,18 mm."),
           ("None.", "Keine.")),
}

DECISIONS_3D = {
    50: [T("Every board of the case at its true thickness from the same numbers as the drawings: side panels 22 mm with two feet and the scrolled cheek, back 15 mm, bottom 25 mm, kneeboard 15 mm, keybed 35 mm, name board 16 mm, stop rail 80 x 60 mm, lid 20 mm, crest 30 mm with its broken right scroll.",
           "Jedes Gehäusebrett in wahrer Stärke aus denselben Zahlen wie die Zeichnungen: Seitenwände 22 mm mit zwei Füßen und geschwungener Backe, Rückwand 15 mm, Boden 25 mm, Kniebrett 15 mm, Klaviaturboden 35 mm, Namensbrett 16 mm, Registerleiste 80 x 60 mm, Deckel 20 mm, Aufsatz 30 mm mit abgebrochener rechter Volute."),
         T("The keyboard is one block of 36 naturals with the 25 sharps as five solids repeated per octave; the reed action and the reservoir are blocks of their effective density; the carpets are declared 6 mm sheets with the torn toe corner cut out; the inventory label is a texture sheet.",
           "Die Klaviatur ist ein Block aus 36 Untertasten mit den 25 Obertasten als fünf je Oktave wiederholten Körpern; Zungenwerk und Magazinbalg sind Blöcke ihrer wirksamen Dichte; die Teppiche sind erklärte 6-mm-Bleche mit ausgeschnittener eingerissener Spitze; das Inventaretikett ist ein Texturblech."),
         T("The exhauster gussets are triangles at the full opening that hide inside the reservoir while the exhauster closes; the strap is 8 mm leather that its bone keeps pointing at the treadle toe in every pose.",
           "Die Falten der Schöpfbälge sind Dreiecke der vollen Öffnung, die beim Schließen im Magazinbalg verschwinden; der Riemen ist 8-mm-Leder, das sein Knochen in jeder Pose auf die Trittspitze richtet.")],
    100: [T("The same case with the cheek scroll and the crest in fewer points; sharps, carpet notch and inventory label are left out, the carpets merge into 28 mm treads, fall, strap and gussets are 15 mm, the 15 mm back is a declared sheet.",
            "Dasselbe Gehäuse mit Backenvolute und Aufsatz in weniger Punkten; Obertasten, Teppichkerbe und Inventaretikett entfallen, die Teppiche gehen in 28-mm-Tritte auf, Klappe, Riemen und Falten sind 15 mm, die 15-mm-Rückwand ist ein erklärtes Blech.")],
    200: [T("Generalised for the 30 mm feature: reed action, reservoir and name board are one block, keybed and key slip one board, side panels, back, fall, lid, desk, treadles, exhausters and straps 30 mm; knobs, handles, gussets and the feet cut-out are left out.",
            "Für das 30-mm-Merkmal verallgemeinert: Zungenwerk, Magazinbalg und Namensbrett sind ein Block, Klaviaturboden und Vorsatzleiste ein Brett, Seitenwände, Rückwand, Klappe, Deckel, Pult, Tritte, Schöpfbälge und Riemen 30 mm; Knöpfe, Griffe, Falten und der Fußausschnitt entfallen.")],
}
