"""🗣️ Every bilingual text of `reclaimed-cobblestone-bag` (English first, German second), with materials and provenance.

@see ./author-build.py — the writer that uses these texts
"""

SIDES = {"r": ("right", "rechte"), "l": ("left", "linke")}

TEXTS = {
    "title": ("Reclaimed Cobblestones in a Big Bag", "Gebrauchtes Kleinpflaster im Big Bag"),
    "summary": ("About one tonne of reclaimed granite setts 9/11 (384 stones of about 100 mm, heads polished by traffic, worn arrises, joint grit and mortar on the flanks), taken up course by course from a segmental-arc paving and packed in eight layers into a woven 95 x 95 x 90 cm big bag with four loops on a Euro pallet; lifted by the loops with the pallet left on the ground, and relaid as a 1.6 m wide segmental-arc field of 24 courses under a 10 kN wheel load.",
                "Etwa eine Tonne gebrauchtes Granit-Kleinpflaster 9/11 (384 Steine von etwa 100 mm, Köpfe vom Verkehr glatt gefahren, abgenutzte Kanten, Fugensplitt und Mörtel an den Flanken), Reihe für Reihe aus einem Segmentbogenpflaster aufgenommen und in acht Lagen in einen gewebten Big Bag 95 x 95 x 90 cm mit vier Schlaufen auf einer Europalette gepackt; an den Schlaufen gehoben, die Palette bleibt am Boden, und als 1,6 m breites Segmentbogenfeld aus 24 Reihen unter einer Radlast von 10 kN wieder verlegt."),
    "pallet-bone": ("Euro pallet (stays on the ground)", "Europalette (bleibt am Boden)"),
    "pallet-section": ("Top deck, five boards 22 mm, 635 mm total width", "Deckbretter, fünf Bretter 22 mm, zusammen 635 mm breit"),
    "bag-bone": ("Big bag (lifting slide, bone along the near wall)", "Big Bag (Hubschlitten, Knochen entlang der vorderen Wand)"),
    "bag-section": ("Woven polypropylene wall, 0.4 mm around 3.8 m", "Polypropylen-Gewebewand, 0,4 mm auf 3,8 m Umfang"),
    "loop-front-right": ("Front right lifting loop", "Hebeschlaufe vorn rechts"),
    "loop-back-right": ("Back right lifting loop", "Hebeschlaufe hinten rechts"),
    "loop-front-left": ("Front left lifting loop", "Hebeschlaufe vorn links"),
    "loop-back-left": ("Back left lifting loop", "Hebeschlaufe hinten links"),
    "loop-section": ("Two legs of 80 x 3 mm webbing", "Zwei Schenkel Gurtband 80 x 3 mm"),
    "wheel-bone": ("Wheel load carrier (massless, moves the wheel load)", "Radlastträger (masselos, bewegt die Radlast)"),
    "pallet-ground": ("Pallet on the yard ground", "Palette auf dem Lagerplatz"),
    "wheel-load": ("Wheel contact on the relaid setts", "Radaufstand auf dem verlegten Pflaster"),
    "wheel-load-title": ("Wheel load 10 kN (light vehicle)", "Radlast 10 kN (leichtes Fahrzeug)"),
    "hatch-weave": ("Woven fabric", "Gewebe"),
    "hatch-grain": ("Timber, along the grain", "Holz, in Faserrichtung"),
    "hatch-granite": ("Natural stone, granite", "Naturstein, Granit"),
    "reg-setts": ("Granite setts (polished heads, soiled flanks)", "Granitpflastersteine (glatte Köpfe, verschmutzte Flanken)"),
    "reg-bag-fabric": ("Big bag fabric and loops (UV-faded, dusty)", "Big-Bag-Gewebe und Schlaufen (UV-gebleicht, staubig)"),
    "reg-pallet-timber": ("Pallet timber (weathered grey)", "Palettenholz (vergraut)"),
    "pose-stored": ("Stored on the pallet", "Auf der Palette gelagert"),
    "pose-stored-d": ("The filled bag stands on its Euro pallet in the yard, the four loops upright; the setts lie inside in eight hand-packed layers of six arc courses each.",
                      "Der gefüllte Big Bag steht auf seiner Europalette im Lager, die vier Schlaufen stehen aufrecht; die Steine liegen darin in acht von Hand gepackten Lagen zu je sechs Bogenreihen."),
    "pose-lifted": ("Lifted by the loops", "An den Schlaufen gehoben"),
    "pose-lifted-d": ("The bag hangs 0.40 m above the pallet from its four loops (forklift tines or a crane hook through the loops); the empty pallet stays on the ground.",
                      "Der Big Bag hängt 0,40 m über der Palette an seinen vier Schlaufen (Gabelzinken oder Kranhaken durch die Schlaufen); die leere Palette bleibt am Boden."),
    "pose-reinstalled": ("Relaid as segmental-arc paving", "Als Segmentbogenpflaster verlegt"),
    "pose-reinstalled-d": ("All 384 setts are relaid in front of the pallet as a segmental-arc field of 24 courses, 1.6 m chord and 0.13 m rise, joints 2 to 12 mm, carrying a 10 kN wheel load; the emptied bag stays on the pallet.",
                           "Alle 384 Steine sind vor der Palette als Segmentbogenfeld aus 24 Reihen verlegt, Sehne 1,6 m, Stich 0,13 m, Fugen 2 bis 12 mm, unter einer Radlast von 10 kN; der geleerte Big Bag bleibt auf der Palette."),
    "clip-lift": ("Lift the bag", "Big Bag anheben"),
    "clip-lift-d": ("The loops take up the load, then the bag rises 0.40 m off the pallet and is held there.", "Die Schlaufen nehmen die Last auf, dann hebt der Big Bag 0,40 m von der Palette ab und wird gehalten."),
    "clip-wheel": ("Wheel crossing the relaid field", "Rad überrollt das verlegte Feld"),
    "clip-wheel-d": ("A 10 kN wheel rolls along the right half of the relaid field from its first to its last course.", "Ein Rad mit 10 kN rollt über die rechte Hälfte des verlegten Feldes von der ersten bis zur letzten Reihe."),
    "ph-take-up": ("Loops take up the load", "Schlaufen nehmen die Last auf"),
    "ph-lift": ("Lifting", "Anheben"),
    "ph-hold": ("Held", "Gehalten"),
    "ph-travel": ("Wheel travel", "Radfahrt"),
    "ev-loops": ("Loops carry the bag", "Schlaufen tragen den Big Bag"),
    "ev-pallet": ("Bag leaves the pallet", "Big Bag verlässt die Palette"),
    "ev-wheel": ("Wheel enters the field", "Rad fährt auf das Feld"),
}


def t(key, arg=None):
    """🔤️ The text `key`, filled for a patch or loop where it needs one."""
    if key == "slide":
        side, n = SIDES[arg[0]], arg[1:]
        return {"en": f"Slide of the {side[0]} patch {n} (drop from its layer)", "de": f"Schlitten des {side[1]}n Feldteils {n} (Absenken aus seiner Lage)"}
    if key == "patch":
        side, n = SIDES[arg[0]], arg[1:]
        return {"en": f"Setts patch {n}, {side[0]} half: 48 setts in six half courses (turns half a turn between bag and field)", "de": f"Pflasterfeldteil {n}, {side[1]} Hälfte: 48 Steine in sechs halben Reihen (dreht sich zwischen Big Bag und Feld um eine halbe Drehung)"}
    if key == "bed":
        side, n = SIDES[arg[0]], arg[1:]
        return {"en": f"Bedding under the {side[0]} patch {n}", "de": f"Bettung unter dem {side[1]}n Feldteil {n}"}
    if key == "sling":
        en, de = TEXTS[arg]
        return {"en": f"Lifting point: {en.lower()}", "de": f"Anschlagpunkt: {de}"}
    en, de = TEXTS[key]
    return {"en": en, "de": de}


def materials():
    """🧱️ Granite, woven polypropylene, webbing and pallet softwood."""
    return [
        {"id": "granite", "title": {"en": "Granite, reclaimed setts (heads polished, flanks soiled)", "de": "Granit, gebrauchte Pflastersteine (Köpfe glatt, Flanken verschmutzt)"}, "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 2650, "conductivityWmK": 2.8, "specificHeatJKgK": 1000, "elasticModulusPa": 50000000000, "strengthPa": 150000000, "source": "literature", "reference": "en-iso-10456"},
        {"id": "pp-fabric", "title": {"en": "Woven polypropylene fabric, UV-faded and dusty", "de": "Polypropylen-Bändchengewebe, UV-gebleicht und staubig"}, "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 910, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "elasticModulusPa": 1500000000, "strengthPa": 250000000, "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": "pp-webbing", "title": {"en": "Polypropylene lifting webbing, soiled", "de": "Polypropylen-Hebegurtband, verschmutzt"}, "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 910, "conductivityWmK": 0.22, "specificHeatJKgK": 1800, "elasticModulusPa": 1500000000, "strengthPa": 250000000, "source": "authored-estimate", "reference": "din-en-iso-21898"},
        {"id": "pallet-softwood", "title": {"en": "Pallet softwood, weathered grey", "de": "Palettenholz Nadelholz, vergraut"}, "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 540, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "din-en-13698-1"},
    ]


def provenance():
    """📚️ Sources and notes."""
    refs = [
        ("din-en-1342", "DIN EN 1342:2013-03 Setts of natural stone for external paving, requirements and test methods (dimension classes and tolerances, declared flexural and compressive strength, surface finish)."),
        ("din-18502", "DIN 18502:1965 Pflastersteine, Naturstein (withdrawn): size classes of small setts Kleinpflaster 8/10, 9/11 and 10/12 cm (edge length and height), the trade names still used for reclaimed setts."),
        ("din-18318", "DIN 18318:2019-09 VOB Part C, ATV Paving and slab pavements without binder, edgings: laying patterns including segmental-arc paving (Segmentbogenpflaster), bedding and joint widths of natural stone setts."),
        ("ztv-pflaster", "FGSV ZTV Pflaster-StB 06: Zusätzliche Technische Vertragsbedingungen und Richtlinien zum Bau von Pflasterdecken, Plattenbelägen und Einfassungen (bedding 3 to 5 cm, joint filling with grit)."),
        ("din-en-iso-21898", "DIN EN ISO 21898:2005 Packaging, flexible intermediate bulk containers (FIBCs) for non-dangerous goods: safe working load, safety factor 5:1 for single-trip bags, lifting by the loops."),
        ("din-en-13698-1", "DIN EN 13698-1:2004 Pallet production specification, Part 1: Construction specification for 800 mm x 1200 mm flat wooden pallets (Euro pallet: 22 mm boards, 78 mm blocks, 144 mm overall height, about 25 kg)."),
        ("en-1936", "EN 1936:2006 Natural stone test methods, determination of real density and apparent density (granite 2600 to 2700 kg/m3)."),
        ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: design conductivity and specific heat (granite 2.8 W/mK, natural stone 1000 J/kgK; polypropylene 0.22 W/mK, 1800 J/kgK; softwood 0.13 W/mK, 1600 J/kgK)."),
        ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, Part 1-1, 6.3.3 and Table 6.8: traffic areas category F (vehicles up to 30 kN), axle load Qk 20 kN, wheel contact square of 100 mm."),
        ("ashrae-fundamentals", "ASHRAE Handbook Fundamentals (2021), Chapter 4 and 26: emissivity and solar absorptance of building materials (rounded typical values, weathered surfaces)."),
        ("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts (hatching)."),
        ("din-1356-1", "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."),
    ]
    return {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in refs],
            "notes": {"en": "Setts modelled as 100 mm cubes (mean of the 9/11 class) with 8 mm worn arrises, 2.61 kg each at 2650 kg/m3; 384 setts = 1.0 t. Field: arcs of radius 2.2 m through the sett centres, 16 setts per course at 102.5 mm pitch (joints 2.5 mm at the head centre, closed at the foot), courses 112 mm apart; chord 1.53 m between the outer sett centres, rise 0.133 m (about 1/11). Big bag 95 x 95 x 90 cm on a Euro pallet 1200 x 800 x 144 mm. Parameters are literature values or authored estimates; nothing is measured.",
                      "de": "Steine als Würfel von 100 mm (Mittel der Klasse 9/11) mit 8 mm abgenutzten Kanten, je 2,61 kg bei 2650 kg/m3; 384 Steine = 1,0 t. Feld: Bögen mit 2,2 m Radius durch die Steinmitten, 16 Steine je Reihe im Abstand 102,5 mm (Fugen 2,5 mm in Kopfmitte, am Fuß geschlossen), Reihenabstand 112 mm; Sehne 1,53 m zwischen den äußeren Steinmitten, Stich 0,133 m (etwa 1/11). Big Bag 95 x 95 x 90 cm auf einer Europalette 1200 x 800 x 144 mm. Parameter sind Literaturwerte oder Schätzungen; nichts ist gemessen."}}


def thermal_assumptions():
    """🌡️ Thermal assumptions."""
    return [
        {"en": "The setts of one patch exchange heat through the envelope of the patch: in the field the joints are filled with grit, in the bag the stones touch each other; the internal joint faces are not counted.", "de": "Die Steine eines Feldteils tauschen Wärme über die Hülle des Feldteils: im Feld sind die Fugen mit Splitt gefüllt, im Big Bag berühren sich die Steine; die inneren Fugenflächen zählen nicht."},
        {"en": "Granite stores heat in about 50 mm (half the sett height; the 24 h penetration depth is 170 mm for 2.8 W/mK, 2650 kg/m3, 1000 J/kgK, EN ISO 13786), so the bag and the field respond slowly to sun and night.", "de": "Granit speichert Wärme in etwa 50 mm (halbe Steinhöhe; die 24-h-Eindringtiefe beträgt 170 mm bei 2,8 W/mK, 2650 kg/m3, 1000 J/kgK, EN ISO 13786), daher reagieren Big Bag und Feld träge auf Sonne und Nacht."},
        {"en": "Weathering: polished heads and soiled flanks of the reclaimed granite absorb 0.65 of the sun (new sawn granite about 0.55); the UV-faded fabric 0.45 and the grey pallet timber 0.65, all authored estimates.", "de": "Bewitterung: glatte Köpfe und verschmutzte Flanken des gebrauchten Granits absorbieren 0,65 der Sonne (neuer gesägter Granit etwa 0,55); das UV-gebleichte Gewebe 0,45 und das vergraute Palettenholz 0,65, alles Schätzungen."},
        {"en": "Nothing is a device: every region is passive and the total power is 0 W.", "de": "Nichts ist ein Gerät: jede Region ist passiv, die Gesamtleistung ist 0 W."},
    ]


def structure_assumptions():
    """🏗️ Structural assumptions."""
    return [
        {"en": "The asset is solved as one rigid body on the contacts of the pose: stored on the pallet footprint, lifted on the four loops, relaid on the pallet and the bedding of the eight patches.", "de": "Das Asset wird als ein starrer Körper auf den Kontakten der Pose gelöst: gelagert auf der Palettenaufstandsfläche, gehoben an den vier Schlaufen, verlegt auf der Palette und der Bettung der acht Feldteile."},
        {"en": "When lifted, the empty pallet (24 kg, 2.3 % of the total) is counted with the hanging load because the rigid-body model does not separate it; the loops therefore carry 2.3 % more than in reality.", "de": "Beim Heben wird die leere Palette (24 kg, 2,3 % der Gesamtmasse) mit der hängenden Last gezählt, weil das Starrkörpermodell sie nicht abtrennt; die Schlaufen tragen daher 2,3 % mehr als in Wirklichkeit."},
        {"en": "Each patch of 48 setts is one point mass at the yaw pivot of its bone (half way between bag and field, laterally 0.21 m from the centre of the patch); left and right patches balance in the bag.", "de": "Jeder Feldteil aus 48 Steinen ist eine Punktmasse am Drehpunkt seines Knochens (auf halbem Weg zwischen Big Bag und Feld, seitlich 0,21 m von der Mitte des Feldteils); linke und rechte Feldteile gleichen sich im Big Bag aus."},
        {"en": "The 10 kN wheel load (EN 1991-1-1 category F) rolls along the right half of the field at 0.40 m from its centre line; bedding and sub-base belong to the site and are not part of the asset.", "de": "Die Radlast 10 kN (EN 1991-1-1 Kategorie F) rollt über die rechte Feldhälfte 0,40 m neben der Mittellinie; Bettung und Tragschicht gehören zur Baustelle und nicht zum Asset."},
    ]


VOL = {
    "50": ("Body: every one of the 384 setts as a 100 mm cube with 8 mm worn arrises, eight per half course turned along the 2.2 m arc and repeated over the six courses of each of the eight patches; the setts touch at their feet along a course (closed wedge joints) and rest layer on layer in the bag, the left-hand layers shifted 20 mm to rest on the right-hand ones. The Euro pallet with all 20 boards and blocks, the bag as four hollow wall bands of true 0.4 mm fabric (bulging 20 mm at mid height) on a bottom sheet, and four flat 80 mm webbing loops sewn 120 mm down the side walls. Thermal: pallet and bag as their body surfaces, each patch as its envelope. Structure: pallet boards, bag fabric and loops as members, one mass marker per patch.",
           "Körper: jeder der 384 Steine als Würfel von 100 mm mit 8 mm abgenutzten Kanten, acht je halbe Reihe entlang des 2,2-m-Bogens gedreht und über die sechs Reihen jedes der acht Feldteile wiederholt; die Steine einer Reihe berühren sich am Fuß (geschlossene Keilfugen) und liegen im Big Bag Lage auf Lage, die linken Lagen um 20 mm versetzt auf den rechten. Die Europalette mit allen 20 Brettern und Klötzen, der Big Bag als vier hohle Wandzonen aus wahrem 0,4-mm-Gewebe (20 mm Bauchung in halber Höhe) auf einem Bodenblatt und vier flache Gurtschlaufen von 80 mm, 120 mm tief an die Seitenwände genäht. Thermisch: Palette und Big Bag als ihre Körperflächen, jeder Feldteil als seine Hülle. Tragwerk: Palettenbretter, Gewebe und Schlaufen als Stäbe, je Feldteil eine Massenmarke."),
    "100": ("The 2.5 to 12 mm joints are below the 15 mm feature: each half course of eight setts becomes one curved band (48 bands as eight solids repeated six times); the five deck boards become one 22 mm deck plate, stringers, blocks and bottom boards stay (22 mm and more). The bag keeps its four bands because the 20 mm bulge is above the feature; loops unchanged (80 mm webbing sheets). Thermal envelopes per patch drawn from the bands, structure as at 1:50 on the merged parts.",
            "Die Fugen von 2,5 bis 12 mm liegen unter der kleinsten Einzelheit von 15 mm: jede halbe Reihe aus acht Steinen wird ein gekrümmtes Band (48 Bänder als acht Körper, sechsmal wiederholt); die fünf Deckbretter werden eine Deckplatte von 22 mm, Querbretter, Klötze und Bodenbretter bleiben (22 mm und mehr). Der Big Bag behält seine vier Zonen, weil die Bauchung von 20 mm über der Einzelheit liegt; Schlaufen unverändert (Gurtband 80 mm als Blatt). Thermische Hüllen je Feldteil aus den Bändern, Tragwerk wie in 1:50 auf den zusammengefassten Teilen."),
    "200": ("At 30 mm every patch of six courses is one block of 100 mm with the outline of its courses; the deck (22 mm) and stringers merge into one 44 mm deck, blocks and bottom boards into nine 100 mm feet. The 20 mm bulge is below the feature: the bag walls are straight at 0.98 m, the loops sit on them. Thermal surfaces are the body solids.",
            "Bei 30 mm wird jeder Feldteil aus sechs Reihen ein Block von 100 mm mit dem Umriss seiner Reihen; Deck (22 mm) und Querbretter verschmelzen zu einem Deck von 44 mm, Klötze und Bodenbretter zu neun Füßen von 100 mm. Die Bauchung von 20 mm liegt unter der Einzelheit: die Wände stehen gerade bei 0,98 m, die Schlaufen sitzen darauf. Die thermischen Flächen sind die Körper."),
    "500": ("At 75 mm the pallet is one 144 mm block, each patch one 100 mm block with a coarser outline, the bag a straight hollow of true fabric on its bottom sheet and the four 80 mm loops kept as sheets because they carry the lifted pose. Thermal surfaces are the body solids; mass markers 90 mm.",
            "Bei 75 mm ist die Palette ein Block von 144 mm, jeder Feldteil ein Block von 100 mm mit gröberem Umriss, der Big Bag eine gerade Hülle aus wahrem Gewebe auf seinem Bodenblatt, und die vier Schlaufen von 80 mm bleiben als Blätter, weil sie die gehobene Pose tragen. Die thermischen Flächen sind die Körper; Massenmarken 90 mm."),
}


def volume_decisions(scale):
    """🧊️ The decisions of the volume of a scale."""
    en, de = VOL[scale]
    return [{"en": en, "de": de}]


REP = {
    "50": {
        "silhouette": ("Side elevation: the Euro pallet (1.2 m, 24 mm on paper) with deck, three stringers, the near blocks and bottom board, the bulging bag (90 cm, bulge 20 mm) with its corner seams and hem, and the two near loops; in the relaid pose a 100 mm band of setts with the joints of the near edge. Front: the 0.8 m pallet end with five deck boards, the bag and the loops edge-on. Plan: the deck boards beyond the bag, the bag wall ring and every sett of the top layer as a turned square; in the relaid pose all 384 setts in their arcs.",
                       "Seitenansicht: die Europalette (1,2 m, 24 mm auf Papier) mit Deck, drei Querbrettern, den vorderen Klötzen und Bodenbrett, der bauchige Big Bag (90 cm, 20 mm Bauchung) mit Ecknähten und Saum und die zwei vorderen Schlaufen; in der verlegten Pose ein Steinband von 100 mm mit den Fugen der vorderen Kante. Vorderansicht: das 0,8-m-Palettenende mit fünf Deckbrettern, der Big Bag und die Schlaufen von der Kante. Draufsicht: die Deckbretter neben dem Big Bag, der Wandring und jeder Stein der obersten Lage als gedrehtes Quadrat; in der verlegten Pose alle 384 Steine in ihren Bögen."),
        "retained": ("Every sett (2 mm on paper), the joints of 2.5 to 12 mm as gaps between the squares in plan, all pallet boards of 22 mm (0.44 mm) and the 78 mm blocks, the 80 mm loops.", "Jeder Stein (2 mm auf Papier), die Fugen von 2,5 bis 12 mm als Lücken zwischen den Quadraten im Grundriss, alle Palettenbretter von 22 mm (0,44 mm) und die Klötze von 78 mm, die Schlaufen von 80 mm."),
        "merged": ("In side and front view each patch of 48 setts is one band, because the setts of the six courses cover each other in these directions.", "In Seiten- und Vorderansicht ist jeder Feldteil aus 48 Steinen ein Band, weil sich die Steine der sechs Reihen in diesen Richtungen überdecken."),
        "omitted": ("The 8 mm worn arrises (0.16 mm) and the split texture of the flanks; the far loops in side view (they cover the near ones exactly); the layers below the top one in plan are hidden by it.", "Die abgenutzten Kanten von 8 mm (0,16 mm) und die Spaltstruktur der Flanken; die hinteren Schlaufen in der Seitenansicht (sie decken sich mit den vorderen); die Lagen unter der obersten sind im Grundriss von ihr verdeckt."),
        "thinElements": ("The 0.4 mm fabric is drawn as the filled outline of the bag and, in plan, as the 14 mm ring between rim and bulge; the 3 mm loops are 0.06 mm strips in front view and plan.", "Das 0,4-mm-Gewebe ist als gefüllter Umriss des Big Bag und im Grundriss als 14-mm-Ring zwischen Rand und Bauchung gezeichnet; die 3-mm-Schlaufen sind in Vorderansicht und Draufsicht Streifen von 0,06 mm."),
        "hatching": ("Granite stipple on the setts, grain lines on the pallet boards, a 45 degree cross hatch for the woven fabric (DIN ISO 128-50 in spirit; views, not sections).", "Granitpunktierung auf den Steinen, Faserlinien auf den Palettenbrettern, 45-Grad-Kreuzschraffur für das Gewebe (DIN ISO 128-50 sinngemäß; Ansichten, keine Schnitte)."),
        "styles": ("Outline 0.35 mm, seams and joints 0.18 mm, hatches 0.07 mm; light fabric, mid-grey stone, darker timber and webbing.", "Kontur 0,35 mm, Nähte und Fugen 0,18 mm, Schraffuren 0,07 mm; helles Gewebe, mittelgrauer Stein, dunkleres Holz und Gurtband."),
        "overlays": ("Setts, bag fabric with loops and pallet timber are separate overlay cells.", "Steine, Gewebe mit Schlaufen und Palettenholz sind getrennte Überlagerungszellen."),
    },
    "100": {
        "silhouette": ("Pallet with one deck plate, stringers, blocks and bottom board; the bulging bag with seams and hem and the loops; in plan the bag ring and the six arc bands of the top layer, in the relaid pose 48 arc bands.", "Palette mit einer Deckplatte, Querbrettern, Klötzen und Bodenbrett; der bauchige Big Bag mit Nähten und Saum und die Schlaufen; im Grundriss der Wandring und die sechs Bogenbänder der obersten Lage, in der verlegten Pose 48 Bogenbänder."),
        "retained": ("The 112 mm course rhythm (1.1 mm), the gaps of 5 to 12 mm between courses as thin light lines, the 20 mm bulge, the 22 mm boards.", "Der Reihenrhythmus von 112 mm (1,1 mm), die Lücken von 5 bis 12 mm zwischen den Reihen als feine helle Linien, die Bauchung von 20 mm, die Bretter von 22 mm."),
        "merged": ("The eight setts of a half course are one band (joints 0.025 to 0.05 mm); the five deck boards one plate.", "Die acht Steine einer halben Reihe sind ein Band (Fugen 0,025 bis 0,05 mm); die fünf Deckbretter eine Platte."),
        "omitted": ("Single setts and their joints along the arc, arrises, the joint lines of the side view.", "Einzelne Steine und ihre Fugen entlang des Bogens, Kanten, die Fugenlinien der Seitenansicht."),
        "thinElements": ("Fabric as filled outline and 0.14 mm ring in plan; loops as 0.03 mm strips edge-on.", "Gewebe als gefüllter Umriss und 0,14-mm-Ring im Grundriss; Schlaufen als Streifen von 0,03 mm von der Kante."),
        "hatching": ("Grain lines on the pallet only; stone and fabric are tones.", "Faserlinien nur auf der Palette; Stein und Gewebe sind Töne."),
        "styles": ("Outline 0.25 mm, seams 0.13 mm.", "Kontur 0,25 mm, Nähte 0,13 mm."),
        "overlays": ("Setts, fabric and timber are separate overlay cells.", "Steine, Gewebe und Holz sind getrennte Überlagerungszellen."),
    },
    "200": {
        "silhouette": ("Pallet as a 44 mm deck on three feet in side view, the straight bag (0.98 m) with two loops; in plan the bag ring and the outline of the top patch with its five course lines; relaid, eight patches with their course lines.", "Palette als Deck von 44 mm auf drei Füßen in der Seitenansicht, der gerade Big Bag (0,98 m) mit zwei Schlaufen; im Grundriss der Wandring und der Umriss des obersten Feldteils mit seinen fünf Reihenlinien; verlegt acht Feldteile mit ihren Reihenlinien."),
        "retained": ("The arc of the courses as detail lines (0.56 mm apart), overall height 1.32 m, the 0.40 m lift.", "Der Bogen der Reihen als Detaillinien (0,56 mm Abstand), Gesamthöhe 1,32 m, der Hub von 0,40 m."),
        "merged": ("Six courses are one patch; deck and stringers one deck, blocks and bottom boards the feet.", "Sechs Reihen sind ein Feldteil; Deck und Querbretter ein Deck, Klötze und Bodenbretter die Füße."),
        "omitted": ("Bulge, seams and hem of the bag, the deck board gaps, the fork openings below the stringers.", "Bauchung, Nähte und Saum des Big Bag, die Fugen der Deckbretter, die Gabelöffnungen unter den Querbrettern."),
        "thinElements": ("Fabric as filled outline and 0.05 mm ring closed by the 0.18 mm pen; loops as thin strips.", "Gewebe als gefüllter Umriss und 0,05-mm-Ring, von der 0,18-mm-Feder geschlossen; Schlaufen als feine Streifen."),
        "hatching": ("No hatch.", "Keine Schraffur."),
        "styles": ("Outline 0.18 mm, course lines 0.1 mm.", "Kontur 0,18 mm, Reihenlinien 0,1 mm."),
        "overlays": ("Setts, fabric and timber are separate overlay cells.", "Steine, Gewebe und Holz sind getrennte Überlagerungszellen."),
    },
    "500": {
        "silhouette": ("Pallet as one 144 mm block (2.4 x 0.29 mm), the bag as a 1.8 mm square outline with its loops; relaid, eight patch outlines forming the arc field (5.6 x 3.3 mm).", "Palette als ein Block von 144 mm (2,4 x 0,29 mm), der Big Bag als Quadrat von 1,8 mm mit seinen Schlaufen; verlegt acht Feldteilumrisse, die das Bogenfeld bilden (5,6 x 3,3 mm)."),
        "retained": ("The arc of the field and its eight patches, the loops that carry the lifted pose, the 0.40 m lift.", "Der Bogen des Feldes und seine acht Feldteile, die Schlaufen, die die gehobene Pose tragen, der Hub von 0,40 m."),
        "merged": ("Pallet boards and blocks are one block; courses are merged into patches.", "Palettenbretter und Klötze sind ein Block; Reihen sind in Feldteilen zusammengefasst."),
        "omitted": ("Course lines, seams, bulge, fork openings.", "Reihenlinien, Nähte, Bauchung, Gabelöffnungen."),
        "thinElements": ("The fabric ring and the 3 mm loops are closed by the 0.13 mm pen.", "Der Gewebering und die 3-mm-Schlaufen werden von der 0,13-mm-Feder geschlossen."),
        "hatching": ("No hatch.", "Keine Schraffur."),
        "styles": ("Outline 0.13 mm, tones only.", "Kontur 0,13 mm, nur Töne."),
        "overlays": ("Setts, fabric and timber are separate overlay cells.", "Steine, Gewebe und Holz sind getrennte Überlagerungszellen."),
    },
}


def rep_decisions(scale):
    """🖨️ The representation decisions of a scale."""
    return {k: {"en": v[0], "de": v[1]} for k, v in REP[scale].items()}
