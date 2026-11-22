"""📝️ Bilingual texts, references and material data of the used bicycle shelter (English first, German second)."""
from author_geometry import t

TITLE = t("Used Bicycle Shelter", "Gebrauchter Fahrradunterstand")
SUMMARY = t(
    "A municipal 5 m bicycle shelter of hot-dip galvanised steel as it stands after some fifteen years beside a park entrance: three rear posts RHS 100 x 100 on dowelled base plates carry cantilever roof beams RHS 120 x 60 at 5 degrees with knee braces, three purlins and four 16 mm multiwall polycarbonate sheets under aluminium glazing bars, 2.1 m deep and 2.6 m high; a half-round gutter clogged with leaves and moss drains into a downpipe behind the right-hand post. Four Sheffield stands (0.75 x 0.80 m) at 1.20 m centres stand under the roof, one dented by a reversing van, and an abandoned city bike with a soft rear tyre and a depot tag is chained to the second stand. The original sheets are yellowed and mossy at their low edge; bay 3 has a clear replacement sheet. Base-plate nuts are rusted, the posts carry a tag and sticker residue.",
    "Ein städtischer Fahrradunterstand von 5 m aus feuerverzinktem Stahl, wie er nach rund fünfzehn Jahren an einem Parkeingang steht: drei hintere Stützen RR 100 x 100 auf verdübelten Fußplatten tragen Kragträger RR 120 x 60 mit 5 Grad Neigung und Kopfbändern, drei Pfetten und vier 16-mm-Stegplatten aus Polycarbonat unter Aluminium-Sprossen, 2,1 m tief und 2,6 m hoch; eine halbrunde, mit Laub und Moos verstopfte Rinne entwässert in ein Fallrohr hinter der rechten Stütze. Vier Anlehnbügel (0,75 x 0,80 m) im Abstand von 1,20 m stehen unter dem Dach, einer von einem rangierenden Transporter eingedellt, und ein zurückgelassenes Stadtrad mit weichem Hinterreifen und Bauhof-Banderole ist an den zweiten Bügel angeschlossen. Die ursprünglichen Platten sind vergilbt und an der Tiefkante bemoost; Feld 3 hat eine klare Ersatzplatte. Die Muttern der Fußplatten sind verrostet, die Stützen tragen ein Tag und Aufkleberreste.",
)


def ref(rid, en):
    """📚️ One reference."""
    return {"id": rid, "citation": en}


REFERENCES = [
    ref("din-en-10219-2", "DIN EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (RHS 100 x 100 x 4: A 14.9 cm2, I 226 cm4, 11.7 kg/m; RHS 120 x 60 x 4: A 13.4 cm2, Iy 245 cm4, 10.5 kg/m; RHS 60 x 40 x 3: 4.25 kg/m; CHS 60.3 x 3.2: A 5.74 cm2, I 23.5 cm4, 4.51 kg/m; CHS 48.3 x 3.2: A 4.53 cm2, I 11.6 cm4, 3.56 kg/m)."),
    ref("din-en-1993-1-1", "DIN EN 1993-1-1:2010 Eurocode 3: Design of steel structures - Part 1-1: General rules (S235: fy 235 MPa, E 210 GPa, density 7850 kg/m3)."),
    ref("din-en-1991-1-1", "DIN EN 1991-1-1:2010 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads (steel 77 to 78.5 kN/m3; horizontal loads from persons leaning on partitions and barriers)."),
    ref("din-en-1991-1-3", "DIN EN 1991-1-3:2010 with the German National Annex DIN EN 1991-1-3/NA:2019, Snow loads: snow load zone 2 at low altitude sk = 0.85 kN/m2, shape coefficient mu1 = 0.8 for a mono-pitch roof below 30 degrees, s = 0.68 kN/m2."),
    ref("din-en-1991-1-4", "DIN EN 1991-1-4:2010 with National Annex, Wind actions on structures (canopy roofs, free-standing mono-pitch canopies)."),
    ref("din-en-iso-1461", "DIN EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (zinc coat 70 to 85 um on 3 to 6 mm sections; dull grey patina after years outdoors)."),
    ref("din-en-16153", "DIN EN 16153:2015 Light transmitting flat multiwall polycarbonate (PC) sheets for internal and external use in roofs, walls and ceilings - Requirements and test methods (16 mm sheets about 2.7 kg/m2; yellowing and loss of light transmission by weathering)."),
    ref("din-en-612", "DIN EN 612:2005 Eaves gutters with bead stiffened fronts and rainwater pipes with seamed joints made of metal sheet (half-round gutter nominal size 333, about 127 mm wide; downpipe 76 mm)."),
    ref("din-79008-1", "DIN 79008-1:2016 Stationary and mobile bicycle parking systems - Part 1: Requirements (stability of stands against parked and leaning bicycles, holding of the frame, spacing of stands)."),
    ref("fgsv-fahrradparken-2012", "FGSV (2012). Hinweise zum Fahrradparken (FGSV 239). Forschungsgesellschaft fuer Strassen- und Verkehrswesen, Cologne: Sheffield (Anlehnbuegel) stands, their height, length and spacing for two bicycles per stand, covered parking."),
    ref("etrto-2024", "ETRTO Standards Manual 2024: bicycle tyre and rim designation 40-622 (28 x 1.5 inch: rim bead seat diameter 622 mm, tyre width 40 mm, outer diameter about 702 mm)."),
    ref("iso-4210-2", "ISO 4210-2:2023 Cycles - Safety requirements for bicycles - Part 2: Requirements for city and trekking, young adult, mountain and racing bicycles (saddle and handlebar heights, frame and fork)."),
    ref("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: emissivities and solar absorptivities of galvanised and painted steel, aluminium, rubber and plastics."),
    ref("vdi-heat-atlas-2010", "VDI e. V. (2010). VDI Heat Atlas, 2nd ed., Springer, sections D6 (thermophysical properties of solids) and K1 (emissivities: weathered zinc 0.2 to 0.3, oxidised steel 0.8, rubber 0.9 to 0.95)."),
    ref("iso-10456-2007", "DIN EN ISO 10456:2010 Building materials and products - Hygrothermal properties (steel 7800 kg/m3, 50 W/(m K), 450 J/(kg K); aluminium 2700 kg/m3, 160 W/(m K); polycarbonate 1200 kg/m3, 0.20 W/(m K); rubber 1200 kg/m3)."),
    ref("shelter-catalogue-typical", "Catalogue-typical data of German municipal bicycle shelters of the 2000s and 2010s (types, not brands): mono-pitch cantilever shelters of hot-dip galvanised steel, 5 m long modules for 8 bicycles, roof depth 2.0 to 2.2 m, clear height 2.2 m, posts RHS 100 x 100 on base plates dowelled to point footings, 16 mm multiwall polycarbonate roof, Sheffield stands CHS 48.3; mass 250 to 400 kg without stands; city bike 16 to 20 kg."),
]


def mat(mid, en, de, eps, alpha, rho, k, c, src, rf, E=None, fy=None):
    """🧱️ One material."""
    m = {"id": mid, "title": t(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
    if E:
        m["elasticModulusPa"] = E
    if fy:
        m["strengthPa"] = fy
    m.update({"source": src, "reference": rf})
    return m


MATERIALS = [
    mat("galv-weathered", "Hot-dip galvanised steel S235, dull grey zinc patina after 15 years", "Feuerverzinkter Stahl S235, stumpfgraue Zinkpatina nach 15 Jahren", 0.28, 0.80, 7850, 50, 460, "authored-estimate", "vdi-heat-atlas-2010", 2.1e11, 2.35e8),
    mat("steel-rusted", "Rusted steel (base plates, nuts, lock cable)", "Verrosteter Stahl (Fußplatten, Muttern, Schlosskabel)", 0.80, 0.85, 7850, 50, 460, "authored-estimate", "vdi-heat-atlas-2010", 2.1e11, 2.35e8),
    mat("pc-yellowed", "Multiwall polycarbonate 16 mm, yellowed and soiled (bulk density of the sheet 2.7 kg/m2 / 16 mm)", "Stegplatte Polycarbonat 16 mm, vergilbt und verschmutzt (Rohdichte der Platte 2,7 kg/m2 / 16 mm)", 0.90, 0.30, 169, 0.20, 1200, "authored-estimate", "din-en-16153", 2.3e9, 6.0e7),
    mat("pc-clear", "Multiwall polycarbonate 16 mm, clear replacement sheet", "Stegplatte Polycarbonat 16 mm, klare Ersatzplatte", 0.90, 0.12, 169, 0.20, 1200, "authored-estimate", "din-en-16153", 2.3e9, 6.0e7),
    mat("moss-litter", "Moss cushions, algae and wet leaf litter", "Moospolster, Algen und nasses Laub", 0.95, 0.80, 400, 0.30, 2500, "authored-estimate", "incropera-2018"),
    mat("alu-weathered", "Aluminium glazing bars, mill finish, weathered", "Aluminium-Sprossen, walzblank, verwittert", 0.20, 0.45, 2700, 160, 900, "authored-estimate", "incropera-2018", 7.0e10, 1.6e8),
    mat("galv-sheet", "Galvanised steel sheet 0.6 mm (gutter, downpipe, clamps), weathered", "Verzinktes Stahlblech 0,6 mm (Rinne, Fallrohr, Schellen), verwittert", 0.25, 0.75, 7850, 50, 460, "authored-estimate", "vdi-heat-atlas-2010", 2.1e11, 2.35e8),
    mat("galv-worn", "Hot-dip galvanised tube, polished bright on top by bicycles and hands", "Feuerverzinktes Rohr, oben von Rädern und Händen blank gerieben", 0.22, 0.70, 7850, 50, 460, "authored-estimate", "vdi-heat-atlas-2010", 2.1e11, 2.35e8),
    mat("bike-steel", "Steel bicycle frame and fittings, faded green paint, rusting chrome", "Stahl-Fahrradrahmen und Anbauteile, ausgeblichener grüner Lack, rostendes Chrom", 0.88, 0.70, 7850, 50, 460, "authored-estimate", "incropera-2018", 2.1e11, 3.55e8),
    mat("rubber-weathered", "Rubber and PVC (tyres, saddle, grips, pedals), cracked by sun", "Gummi und PVC (Reifen, Sattel, Griffe, Pedale), von der Sonne rissig", 0.93, 0.90, 1150, 0.20, 1600, "authored-estimate", "iso-10456-2007"),
]

REGIONS = {
    "steel-frame": {"title": t("Galvanised frames and purlins: posts, roof beams, knee braces, purlins", "Verzinkte Rahmen und Pfetten: Stützen, Kragträger, Kopfbänder, Pfetten"), "kind": "surface", "material": "galv-weathered", "L": 0.1, "geometry": "vertical-plate", "sky": 0.4, "ref": "vdi-heat-atlas-2010"},
    "base-fixings": {"title": t("Rusted base plates and anchor nuts on the paving", "Verrostete Fußplatten und Ankermuttern auf dem Pflaster"), "kind": "surface", "material": "steel-rusted", "L": 0.25, "geometry": "horizontal-plate-up", "sky": 0.6, "ref": "vdi-heat-atlas-2010"},
    "roof-old": {"title": t("Original roof sheets, yellowed and soiled", "Ursprüngliche Dachplatten, vergilbt und verschmutzt"), "kind": "surface", "material": "pc-yellowed", "L": 2.1, "geometry": "horizontal-plate-up", "sky": 1.0, "ref": "din-en-16153"},
    "roof-new": {"title": t("Clear replacement roof sheet in bay 3", "Klare Ersatz-Dachplatte in Feld 3"), "kind": "surface", "material": "pc-clear", "L": 2.1, "geometry": "horizontal-plate-up", "sky": 1.0, "ref": "din-en-16153"},
    "moss": {"title": t("Moss cushions on the low roof edge and leaf litter in the gutter", "Moospolster an der Dachtiefkante und Laub in der Rinne"), "kind": "surface", "material": "moss-litter", "L": 0.3, "geometry": "horizontal-plate-up", "sky": 0.9, "ref": "incropera-2018"},
    "glazing-bars": {"title": t("Aluminium glazing bars over the sheet joints", "Aluminium-Sprossen über den Plattenfugen"), "kind": "surface", "material": "alu-weathered", "L": 0.06, "geometry": "horizontal-plate-up", "sky": 1.0, "ref": "incropera-2018"},
    "rainwater-goods": {"title": t("Gutter, downpipe and clamps", "Rinne, Fallrohr und Schellen"), "kind": "surface", "material": "galv-sheet", "L": 0.127, "geometry": "cylinder", "sky": 0.5, "ref": "vdi-heat-atlas-2010"},
    "bike-stands": {"title": t("Sheffield stands, polished on top", "Anlehnbügel, oben blank gerieben"), "kind": "surface", "material": "galv-worn", "L": 0.0483, "geometry": "cylinder", "sky": 0.3, "ref": "vdi-heat-atlas-2010"},
    "bike-frame": {"title": t("Abandoned bike: frame, wheels, rack and lock", "Zurückgelassenes Rad: Rahmen, Laufräder, Gepäckträger und Schloss"), "kind": "surface", "material": "bike-steel", "L": 0.03, "geometry": "cylinder", "sky": 0.3, "ref": "incropera-2018"},
    "bike-rubber": {"title": t("Abandoned bike: tyres, saddle, grips and pedals", "Zurückgelassenes Rad: Reifen, Sattel, Griffe und Pedale"), "kind": "surface", "material": "rubber-weathered", "L": 0.04, "geometry": "cylinder", "sky": 0.3, "ref": "iso-10456-2007"},
}

ANCHOR_TEXT = {
    "base": lambda f: t(f"Base plate of post {f.upper()}, dowelled with four M16 anchors to its concrete point footing under the paving", f"Fußplatte von Stütze {f.upper()}, mit vier M16-Ankern auf ihr Punktfundament unter dem Pflaster verdübelt"),
    "leg": lambda k, leg: t(f"Stand {k}, {'rear' if leg == 'rear' else 'front'} leg set into its concrete footing", f"Bügel {k}, {'hinteres' if leg == 'rear' else 'vorderes'} Bein im Betonfundament eingesetzt"),
    "tyre-rear": t("Rear tyre of the abandoned bike on the paving (soft, flattened contact)", "Hinterreifen des zurückgelassenen Rads auf dem Pflaster (weich, abgeplattete Aufstandsfläche)"),
    "tyre-front": t("Front tyre of the abandoned bike on the paving", "Vorderreifen des zurückgelassenen Rads auf dem Pflaster"),
    "kickstand": t("Foot of the side stand of the abandoned bike", "Fuß des Seitenständers des zurückgelassenen Rads"),
    "lift": lambda side: t(f"Front corner of the replacement sheet, {'left' if side == 'l' else 'right'}: held by a depot worker on a ladder", f"Vordere Ecke der Ersatzplatte, {'links' if side == 'l' else 'rechts'}: von einem Bauhofmitarbeiter auf der Leiter gehalten"),
    "lean": lambda k: t(f"Top rail of stand {k}, where a visiting bicycle leans", f"Oberholm von Bügel {k}, an dem ein Besucherrad lehnt"),
    "lean-post": t("Front face of post B at 1.2 m, where a visitor leans while sheltering from rain", "Vorderseite von Stütze B in 1,2 m Höhe, an der ein Besucher beim Unterstellen lehnt"),
}

POSE_TEXT = {
    "in-service": (t("In service on its footings", "Im Einsatz auf seinen Fundamenten"), t("The shelter stands on its three dowelled base plates and the eight embedded stand legs in the paving of a park entrance, summer afternoon sun on the roof; the abandoned bike stands on its side stand, chained to stand 2. Only self-weight acts.", "Der Unterstand steht auf seinen drei verdübelten Fußplatten und den acht eingesetzten Bügelbeinen im Pflaster eines Parkeingangs, Sommernachmittagssonne auf dem Dach; das zurückgelassene Rad steht auf seinem Seitenständer, an Bügel 2 angeschlossen. Nur das Eigengewicht wirkt.")),
    "maintenance": (t("Replacement sheet slid out by the depot", "Ersatzplatte vom Bauhof herausgezogen"), t("The depot has loosened the glazing bars over bay 3 and slides the clear replacement sheet 1.0 m forward along the bars to reseal its joints; two workers on ladders hold its front corners. Frames, other sheets, stands and the bike stay in place.", "Der Bauhof hat die Sprossen über Feld 3 gelöst und zieht die klare Ersatzplatte 1,0 m längs der Sprossen nach vorn, um ihre Fugen neu abzudichten; zwei Mitarbeiter auf Leitern halten die vorderen Ecken. Rahmen, übrige Platten, Bügel und Rad bleiben an ihrem Platz.")),
    "in-use": (t("In use: visiting bikes and a sheltering visitor", "In Benutzung: Besucherräder und ein Besucher beim Unterstellen"), t("Two visiting bicycles lean against stands 1 and 3, one owner pulls his lock tight through the frame and the stand, and a visitor leans against post B while waiting out a shower; the shade of the roof falls on the stands.", "Zwei Besucherräder lehnen an den Bügeln 1 und 3, ein Besitzer zieht sein Schloss durch Rahmen und Bügel fest, und ein Besucher lehnt an Stütze B, während er einen Schauer abwartet; der Dachschatten fällt auf die Bügel.")),
    "winter-snow": (t("Winter snow on the roof", "Winterschnee auf dem Dach"), t("A snow cover of 0.68 kN/m2 (zone 2, mono-pitch roof) lies on the sheets and loads the three cantilever beams by their tributary widths; overcast, no sun.", "Eine Schneedecke von 0,68 kN/m2 (Zone 2, Pultdach) liegt auf den Platten und belastet die drei Kragträger nach ihren Einzugsbreiten; bedeckt, keine Sonne.")),
    "snow-load": lambda f, trib: t(f"Snow on the roof over beam {f.upper()} (0.68 kN/m2 over {trib} m)", f"Schnee auf dem Dach über Träger {f.upper()} (0,68 kN/m2 über {str(trib).replace('.', ',')} m)"),
    "lean-bike-1": t("Visiting bicycle leaning against stand 1", "Besucherrad lehnt an Bügel 1"),
    "lean-bike-3": t("Visiting bicycle leaning against stand 3", "Besucherrad lehnt an Bügel 3"),
    "lock-pull": t("Owner pulling the lock tight at stand 3", "Besitzer zieht das Schloss an Bügel 3 fest"),
    "lean-visitor": t("Visitor leaning against post B", "Besucher lehnt an Stütze B"),
    "hold-sheet": lambda side: t(f"Depot worker carrying a quarter of the slid-out sheet at its front {'left' if side == 'l' else 'right'} corner", f"Bauhofmitarbeiter trägt ein Viertel der herausgezogenen Platte an ihrer vorderen {'linken' if side == 'l' else 'rechten'} Ecke"),
}

CLIP_TEXT = {
    "m": (t("Sliding the replacement sheet out", "Die Ersatzplatte herausziehen"), t("The depot loosens the glazing bars over bay 3, two workers take the front corners of the clear replacement sheet and slide it 1.0 m forward along the bars to reseal it; everything else stays where it is.", "Der Bauhof löst die Sprossen über Feld 3, zwei Mitarbeiter fassen die vorderen Ecken der klaren Ersatzplatte und ziehen sie 1,0 m längs der Sprossen nach vorn, um sie neu abzudichten; alles andere bleibt, wo es ist.")),
    "u": (t("An afternoon of parking", "Ein Nachmittag Parken"), t("Twenty seconds of a busy afternoon: a cyclist leans her bike against stand 1, a second one against stand 3 and pulls his lock tight, a visitor leans against post B during a shower and walks on, the first cyclist rides off; the shelter itself does not move.", "Zwanzig Sekunden eines belebten Nachmittags: eine Radfahrerin lehnt ihr Rad an Bügel 1, ein zweiter seines an Bügel 3 und zieht sein Schloss fest, ein Besucher lehnt während eines Schauers an Stütze B und geht weiter, die erste Radfahrerin fährt ab; der Unterstand selbst bewegt sich nicht.")),
    "ph-unclamp": t("Glazing bars over bay 3 loosened", "Sprossen über Feld 3 gelöst"),
    "ph-hold": t("Workers take the front corners", "Mitarbeiter fassen die vorderen Ecken"),
    "ph-slide": t("Sheet slides forward along the bars", "Platte gleitet längs der Sprossen nach vorn"),
    "ph-held": t("Sheet held 1.0 m out", "Platte 1,0 m herausgezogen gehalten"),
    "ev-grip": t("Workers take the weight at the front corners", "Mitarbeiter nehmen das Gewicht an den vorderen Ecken"),
    "ev-start": t("Sheet starts to slide", "Platte beginnt zu gleiten"),
    "ev-stop": t("Sheet stops 1.0 m out", "Platte hält 1,0 m draußen"),
    "ph-a1": t("First cyclist parks at stand 1", "Erste Radfahrerin parkt an Bügel 1"),
    "ph-a3": t("Second cyclist parks and locks at stand 3", "Zweiter Radfahrer parkt und schließt an Bügel 3 an"),
    "ph-dep": t("Visitor shelters, first cyclist leaves", "Besucher stellt sich unter, erste Radfahrerin fährt ab"),
    "ev-b1-on": t("Bike leans against stand 1", "Rad lehnt an Bügel 1"),
    "ev-b3-on": t("Bike leans against stand 3", "Rad lehnt an Bügel 3"),
    "ev-lock": t("Lock pulled tight", "Schloss festgezogen"),
    "ev-lock-off": t("Lock released", "Schloss losgelassen"),
    "ev-visitor": t("Visitor leans against post B", "Besucher lehnt an Stütze B"),
    "ev-visitor-off": t("Visitor walks on", "Besucher geht weiter"),
    "ev-b1-off": t("First bike taken away", "Erstes Rad weggenommen"),
}


def dec(sil, ret, mer, omi, thin, hat, sty, ov):
    """🧭️ Representation decisions."""
    return {"silhouette": sil, "retained": ret, "merged": mer, "omitted": omi, "thinElements": thin, "hatching": hat, "styles": sty, "overlays": ov}


REP_DECISIONS = {
    50: dec(
        t("The shelter at true size: posts 100 mm, beams 120 x 60 mm at 5 degrees, knee braces 60.3 mm, purlins 60 x 40 mm, the 16 mm sheets with their T-shaped glazing bars, the half-round gutter and the 76 mm downpipe with two clamps; four Sheffield stands as tube bands with 180 mm bends, stand 4 dented; the bike with tyres, rims, hubs, frame tubes, rack, saddle, bars, cranks, pedals, side stand and the cable lock to stand 2.", "Der Unterstand in wahrer Größe: Stützen 100 mm, Träger 120 x 60 mm mit 5 Grad, Kopfbänder 60,3 mm, Pfetten 60 x 40 mm, die 16-mm-Platten mit ihren T-förmigen Sprossen, die halbrunde Rinne und das 76-mm-Fallrohr mit zwei Schellen; vier Anlehnbügel als Rohrbänder mit 180-mm-Bögen, Bügel 4 eingedellt; das Rad mit Reifen, Felgen, Naben, Rahmenrohren, Gepäckträger, Sattel, Lenker, Kurbeln, Pedalen, Seitenständer und Kabelschloss an Bügel 2."),
        t("Clear height 2.24 to 2.40 m, roof depth 2.1 m, length 5.04 m, frame spacing 2.40 m, stand spacing 1.20 m, stand 0.75 x 0.80 m, wheelbase 1.10 m, wheel 702 mm.", "Lichte Höhe 2,24 bis 2,40 m, Dachtiefe 2,1 m, Länge 5,04 m, Rahmenabstand 2,40 m, Bügelabstand 1,20 m, Bügel 0,75 x 0,80 m, Radstand 1,10 m, Laufrad 702 mm."),
        t("Tube walls, zinc coat and paint are one filled band per member; the three frames overlap in the side elevation.", "Rohrwand, Zinkschicht und Lack sind je Stab ein gefülltes Band; die drei Rahmen decken sich in der Seitenansicht."),
        t("Weld seams, screw threads, the gutter hangers, chain and brake cables, mudguards and lights of the bike.", "Schweißnähte, Gewinde, Rinnenhaken, Kette und Bremszüge, Schutzbleche und Lichter des Rads."),
        t("Spokes (2 mm) and the moss edge are texture strokes; nuts, screw heads and the depot tag are detail.", "Speichen (2 mm) und die Moosekante sind Texturstriche; Muttern, Schraubenköpfe und die Banderole sind Detail."),
        t("Flute lines on the sheets in plan along the slope.", "Steglinien auf den Platten im Grundriss in Gefällerichtung."),
        t("Body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm, ground 0.5 mm; tones by material, the replacement sheet lighter than the yellowed ones.", "Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,1 mm, Boden 0,5 mm; Töne nach Material, die Ersatzplatte heller als die vergilbten."),
        t("Traces of use: tag and sticker residue on posts A and B, rust creeping from the base plates, the polished top of stand 1, algae streaks on the front edge, moss cushions and leaf litter in plan, the flat contact of the soft rear tyre, the depot tag on the handlebar.", "Gebrauchsspuren: Tag und Aufkleberreste an den Stützen A und B, von den Fußplatten aufsteigender Rost, der blanke Oberholm von Bügel 1, Algenstreifen an der Vorderkante, Moospolster und Laub im Grundriss, die abgeplattete Aufstandsfläche des weichen Hinterreifens, die Banderole am Lenker."),
    ),
    100: dec(
        t("The same shelter for 15 mm: frames, purlins, sheets and T glazing bars, gutter and downpipe with clamps, stands with two-segment bends and the dented stand 4; the bike with merged tyre-and-rim rings, hubs, frame tubes, rack, saddle, bars, cranks and pedals.", "Derselbe Unterstand für 15 mm: Rahmen, Pfetten, Platten und T-Sprossen, Rinne und Fallrohr mit Schellen, Bügel mit zweiteiligen Bögen und der eingedellte Bügel 4; das Rad mit zusammengefassten Reifen-Felgen-Ringen, Naben, Rahmenrohren, Gepäckträger, Sattel, Lenker, Kurbeln und Pedalen."),
        t("All main dimensions as at 1:50.", "Alle Hauptmaße wie in 1:50."),
        t("Tyre and rim are one ring 61 mm deep; rack tubes drawn 15 mm.", "Reifen und Felge sind ein Ring von 61 mm Tiefe; Gepäckträgerrohre 15 mm gezeichnet."),
        t("Nuts, screw heads, the cable lock, sticker residue and the depot tag (below 15 mm).", "Muttern, Schraubenköpfe, Kabelschloss, Aufkleberreste und Banderole (unter 15 mm)."),
        t("Spokes as eight texture strokes per wheel; the 16 mm sheets keep their thickness.", "Speichen als acht Texturstriche je Laufrad; die 16-mm-Platten behalten ihre Dicke."),
        t("Flute lines in plan.", "Steglinien im Grundriss."),
        t("Body pen 0.25 mm, detail 0.13 mm, texture 0.1 mm, ground 0.35 mm.", "Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,1 mm, Boden 0,35 mm."),
        t("Tag strokes on posts A and B, moss and litter in plan, the flat rear tyre.", "Tag-Striche an den Stützen A und B, Moos und Laub im Grundriss, der platte Hinterreifen."),
    ),
    200: dec(
        t("Generalised for 30 mm: frames with 30 mm base plates, purlins, sheets under 30 mm box glazing bars, gutter, downpipe and clamps, the stands as true 48 mm tube bands; the bike as two 61 mm wheel rings, frame tubes of at least 30 mm, a single-slab rack, saddle and bars.", "Generalisiert für 30 mm: Rahmen mit 30-mm-Fußplatten, Pfetten, Platten unter 30-mm-Kastensprossen, Rinne, Fallrohr und Schellen, die Bügel als wahre 48-mm-Rohrbänder; das Rad als zwei 61-mm-Laufradringe, Rahmenrohre von mindestens 30 mm, ein einteiliger Gepäckträger, Sattel und Lenker."),
        t("Overall size, frame and stand spacing, roof slope, wheel size and the place of the bike.", "Gesamtgröße, Rahmen- und Bügelabstand, Dachneigung, Laufradgröße und Lage des Rads."),
        t("Base plate and nuts are one 30 mm plate; the rack is one slab; glazing bar web and cap one box.", "Fußplatte und Muttern sind eine 30-mm-Platte; der Gepäckträger ist eine Platte; Steg und Deckleiste der Sprosse ein Kasten."),
        t("Hubs, grips, cranks, pedals, the dent of stand 4 and all trace strokes except moss.", "Naben, Griffe, Kurbeln, Pedale, die Delle von Bügel 4 und alle Spurenstriche außer dem Moos."),
        t("The 16 mm sheets and the 76 mm downpipe keep their true size; spokes as four texture strokes.", "Die 16-mm-Platten und das 76-mm-Fallrohr behalten ihre wahre Größe; Speichen als vier Texturstriche."),
        t("No hatches.", "Keine Schraffuren."),
        t("Body pen 0.18 mm, detail 0.1 mm, texture 0.1 mm, ground 0.25 mm.", "Körperstift 0,18 mm, Detail 0,1 mm, Textur 0,1 mm, Boden 0,25 mm."),
        t("Moss as a texture line along the low roof edge in the side view; the moss patches are below 30 mm thick and not drawn in plan.", "Moos als Texturlinie entlang der Dachtiefkante in der Seitenansicht; die Moosflecken sind unter 30 mm dick und im Grundriss nicht gezeichnet."),
    ),
    500: dec(
        t("Generalised for 75 mm: posts, 100 mm base plates, a 180 mm beam band taking in the purlins, braces 75 mm, the four sheets with closed joints, the gutter block and the downpipe; stands as 75 mm tube bands; the gutter as a 170 x 120 mm block; the bike as two 75 mm wheel rings with a frame plate, saddle and bars of 75 mm.", "Generalisiert für 75 mm: Stützen, 100-mm-Fußplatten, ein 180-mm-Trägerband mit den Pfetten, Kopfbänder 75 mm, die vier Platten mit geschlossenen Fugen, der Rinnenblock und das Fallrohr; Bügel als 75-mm-Rohrbänder; die Rinne als Block 170 x 120 mm; das Rad als zwei 75-mm-Laufradringe mit Rahmenplatte, Sattel und Lenker von 75 mm."),
        t("Outer size 2.25 x 2.62 x 5.04 m, three posts, four stands, one bike.", "Außenmaß 2,25 x 2,62 x 5,04 m, drei Stützen, vier Bügel, ein Rad."),
        t("Glazing bars merged into the sheets, which close their joints; purlins merged into a 180 mm beam band in the side view; the bike frame one 75 mm plate. In plan the beams, braces and purlins under the sheets are not drawn (hidden), so a slid-out sheet uncovers the stands and the bike below.", "Sprossen in die Platten aufgenommen, die ihre Fugen schließen; Pfetten in der Seitenansicht in ein 180-mm-Trägerband aufgenommen; der Fahrradrahmen eine 75-mm-Platte. Im Grundriss sind Träger, Kopfbänder und Pfetten unter den Platten nicht gezeichnet (verdeckt), daher legt eine herausgezogene Platte die Bügel und das Rad darunter frei."),
        t("The lower clamp, rack, chainring and every trace of use except moss; the glazing bars are one texture line.", "Die untere Schelle, Gepäckträger, Kettenblatt und alle Gebrauchsspuren außer dem Moos; die Sprossen sind eine Texturlinie."),
        t("The sheets keep their 16 mm (declared sheets); the downpipe is a declared slender tube.", "Die Platten behalten ihre 16 mm (erklärte Bleche); das Fallrohr ist ein erklärtes schlankes Rohr."),
        t("No hatches.", "Keine Schraffuren."),
        t("Body pen 0.14 mm, texture 0.1 mm, ground 0.18 mm.", "Körperstift 0,14 mm, Textur 0,1 mm, Boden 0,18 mm."),
        t("Moss as a texture line along the low roof edge and the glazing bars as a texture line on the roof in the side view.", "Moos als Texturlinie entlang der Dachtiefkante und die Sprossen als Texturlinie auf dem Dach in der Seitenansicht."),
    ),
}

VOLUME_DECISIONS = {
    50: [t("Body for 7.5 mm: RHS posts, beams and purlins as boxes, the knee braces and downpipe as turned tubes, base plates with four rusted nuts each, the sheets as declared 16 mm sheets on the purlins, T-shaped aluminium glazing bars, the gutter as a half-round body filled flush with litter, the stands as tube chains (straight legs, three-segment bends, top rail, cross rail at 0.40 m; stand 4 dented 25 mm), the bike part by part with 2 x 16 slender spokes per wheel, and moss and litter as texture patches.", "Körper für 7,5 mm: RR-Stützen, -Träger und -Pfetten als Quader, Kopfbänder und Fallrohr als gedrehte Rohre, Fußplatten mit je vier verrosteten Muttern, die Platten als erklärte 16-mm-Bleche auf den Pfetten, T-förmige Aluminium-Sprossen, die Rinne als halbrunder, bündig mit Laub gefüllter Körper, die Bügel als Rohrketten (gerade Beine, dreiteilige Bögen, Oberholm, Querholm auf 0,40 m; Bügel 4 um 25 mm eingedellt), das Rad Teil für Teil mit 2 x 16 schlanken Speichen je Laufrad, Moos und Laub als Texturflecken."),
           t("Thermal: every body solid referenced in its region. Structure: posts, beams, braces, sheets, stand tubes, downpipe and the bike's load path (tyres, rims, spokes, hubs, frame, side stand) as members; base plates, purlins, clamps and the post heads and brace ends as joints; one mass marker per bone at its centre of mass.", "Thermik: jeder Körper in seiner Region referenziert. Tragwerk: Stützen, Träger, Kopfbänder, Platten, Bügelrohre, Fallrohr und der Lastweg des Rads (Reifen, Felgen, Speichen, Naben, Rahmen, Seitenständer) als Stäbe; Fußplatten, Pfetten, Schellen sowie Stützenköpfe und Kopfbandenden als Knoten; je Knochen ein Massenpunkt im Schwerpunkt.")],
    100: [t("Body for 15 mm: nuts and the cable lock left out; tyre and rim merged into one ring, the spokes into a thin texture disc that ties hub and ring; clamps and rack tubes drawn 15 mm, stand bends in two segments; everything else as at 1:50.", "Körper für 15 mm: Muttern und Kabelschloss entfallen; Reifen und Felge zu einem Ring zusammengefasst, die Speichen zu einer dünnen Texturscheibe, die Nabe und Ring verbindet; Schellen und Gepäckträgerrohre 15 mm, Bügelbögen zweiteilig; alles andere wie in 1:50."),
           t("Thermal and structure follow the body; the spoke disc stands in for the spokes in the load path of the wheels.", "Thermik und Tragwerk folgen dem Körper; die Speichenscheibe vertritt die Speichen im Lastweg der Laufräder.")],
    200: [t("Body for 30 mm: base plates 30 mm thick, glazing bars as 60 x 30 mm boxes, clamps 30 mm; the bike without hubs, grips, cranks and pedals, its tubes at least 30 mm, the rack as one slab with one strut; the stands keep their true 48 mm tube and lose the dent.", "Körper für 30 mm: Fußplatten 30 mm dick, Sprossen als 60 x 30 mm Kästen, Schellen 30 mm; das Rad ohne Naben, Griffe, Kurbeln und Pedale, seine Rohre mindestens 30 mm, der Gepäckträger als eine Platte mit einer Strebe; die Bügel behalten ihr wahres 48-mm-Rohr und verlieren die Delle."),
           t("Thermal and structure follow the body; the bike's load path is frame, spoke discs and wheel rings.", "Thermik und Tragwerk folgen dem Körper; der Lastweg des Rads sind Rahmen, Speichenscheiben und Laufradringe.")],
    500: [t("Body for 75 mm: base plates 100 mm thick, beams 120 mm wide and 180 mm deep (taking in the purlin depth), purlins 100 x 75 mm, braces and stand tubes 75 mm, the sheets without overhang and closed joints, the gutter a 170 x 120 mm block, glazing bars as texture caps, one 75 mm clamp ties the downpipe to post C, the downpipe kept as a declared slender tube down to the gully; the bike as two 75 mm wheel rings with four texture spokes each, one 75 mm frame plate from axle to axle, saddle and bars (side stand, rack and chainring left out).", "Körper für 75 mm: Fußplatten 100 mm dick, Träger 120 mm breit und 180 mm hoch (mit der Pfettenhöhe), Pfetten 100 x 75 mm, Kopfbänder und Bügelrohre 75 mm, die Platten ohne Überstand mit geschlossenen Fugen, die Rinne ein Block 170 x 120 mm, Sprossen als Texturleisten, eine 75-mm-Schelle bindet das Fallrohr an Stütze C, das Fallrohr bleibt als erklärtes schlankes Rohr bis zum Ablauf; das Rad als zwei 75-mm-Laufradringe mit je vier Textur-Speichen, eine 75-mm-Rahmenplatte von Achse zu Achse, Sattel und Lenker (Seitenständer, Gepäckträger und Kettenblatt entfallen)."),
           t("Thermal and structure follow the body; mass markers 75 mm.", "Thermik und Tragwerk folgen dem Körper; Massenpunkte 75 mm.")],
}

STRUCTURE_ASSUMPTIONS = [
    t("Each of the three frames is a cantilever: the roof beam RHS 120 x 60 x 4 is welded on the rear post RHS 100 x 100 x 4 and propped by a knee brace CHS 60.3 x 3.2; the post is fixed at its base plate (250 x 250 x 15 mm, four M16 heavy-duty anchors into a C20/25 point footing, which stays in the ground and is not part of the asset). Purlins RHS 60 x 40 x 3 tie the three beams; their weight and that of the glazing bars is carried by the middle frame in the rig.", "Jeder der drei Rahmen ist ein Kragarm: der Dachträger RR 120 x 60 x 4 ist auf die hintere Stütze RR 100 x 100 x 4 geschweißt und durch ein Kopfband RO 60,3 x 3,2 abgestützt; die Stütze ist an ihrer Fußplatte eingespannt (250 x 250 x 15 mm, vier M16-Schwerlastanker in ein Punktfundament C20/25, das im Boden bleibt und nicht Teil des Assets ist). Pfetten RR 60 x 40 x 3 verbinden die drei Träger; ihr Gewicht und das der Sprossen trägt im Rig der mittlere Rahmen."),
    t("Dead load from section x length x density (EN 10219-2 masses, steel 7850 kg/m3, sheets 2.7 kg/m2, aluminium bars 1.0 kg/m, gutter with litter 2.0 kg/m); the abandoned bike (15.8 kg) is part of the asset and stands on its tyres and side stand. Snow: 0.68 kN/m2 (DIN EN 1991-1-3/NA, zone 2, mu1 0.8) on the tributary widths 1.3, 2.4 and 1.3 m of the beams; wind on the open canopy is noted but not posed.", "Eigengewicht aus Querschnitt x Länge x Dichte (Massen nach EN 10219-2, Stahl 7850 kg/m3, Platten 2,7 kg/m2, Aluminiumsprossen 1,0 kg/m, Rinne mit Laub 2,0 kg/m); das zurückgelassene Rad (15,8 kg) gehört zum Asset und steht auf Reifen und Seitenständer. Schnee: 0,68 kN/m2 (DIN EN 1991-1-3/NA, Zone 2, mu1 0,8) auf den Einzugsbreiten 1,3, 2,4 und 1,3 m der Träger; Wind auf das offene Dach ist erwähnt, aber nicht als Pose angesetzt."),
    t("Leaning bicycles (25 N down, 40 N sideways at the top rail), a lock pulled tight (120 N) and a visitor leaning against a post (150 N horizontal, 60 N down) are authored estimates well inside the stability requirements of DIN 79008-1 and the horizontal loads of EN 1991-1-1. In maintenance two workers hold the front corners of the slid-out sheet while its rear edge still lies on the glazing bars: the sheet is a beam on its rear edge and the hands, so each worker carries a quarter of the 7.2 kg sheet (17.7 N), applied as carrying forces because supports of the archive are springs of equal stiffness that would share the weight of the whole shelter with the hands.", "Lehnende Räder (25 N nach unten, 40 N seitlich am Oberholm), ein festgezogenes Schloss (120 N) und ein an eine Stütze gelehnter Besucher (150 N waagerecht, 60 N nach unten) sind Schätzwerte deutlich innerhalb der Standsicherheitsanforderungen der DIN 79008-1 und der Horizontallasten der EN 1991-1-1. Bei der Wartung halten zwei Mitarbeiter die vorderen Ecken der herausgezogenen Platte, deren hintere Kante noch auf den Sprossen liegt: die Platte ist ein Balken auf ihrer hinteren Kante und den Händen, jeder Mitarbeiter trägt also ein Viertel der 7,2 kg schweren Platte (17,7 N), angesetzt als Tragkräfte, weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht des ganzen Unterstands mit den Händen teilen würden."),
]

THERMAL_ASSUMPTIONS = [
    t("All ten regions are passive surfaces: solar gain, long-wave exchange with sky and ground, forced convection by the park breeze, no internal heat; the shelter draws no power (passive, 0 W).", "Alle zehn Regionen sind passive Flächen: Solargewinn, langwelliger Austausch mit Himmel und Boden, erzwungene Konvektion durch den Parkwind, keine innere Wärme; der Unterstand nimmt keine Leistung auf (passiv, 0 W)."),
    t("Weathered surfaces differ from new ones: the dull zinc patina absorbs about 0.80 of the sun against 0.65 when new and emits 0.28 against 0.05 to 0.13 (VDI Heat Atlas, authored estimate); the yellowed and soiled sheets are treated as opaque plates absorbing 0.30, the clear replacement sheet 0.12 (most of the sun passes through to the paving); moss and wet litter absorb 0.80.", "Verwitterte Flächen unterscheiden sich von neuen: die stumpfe Zinkpatina absorbiert etwa 0,80 der Sonne gegenüber 0,65 im Neuzustand und emittiert 0,28 gegenüber 0,05 bis 0,13 (VDI-Wärmeatlas, Schätzung); die vergilbten und verschmutzten Platten werden als undurchsichtige Platten mit 0,30 Absorption behandelt, die klare Ersatzplatte mit 0,12 (der größte Teil der Sonne geht auf das Pflaster durch); Moos und nasses Laub absorbieren 0,80."),
    t("Heat capacities per area come from the real masses of the parts (section x density) and the specific heats of ISO 10456; the stands and the bike stand in the shade of the roof (sunlit 0.2 to 0.25 on a summer afternoon), the roof in full sun.", "Wärmekapazitäten je Fläche ergeben sich aus den realen Massen der Teile (Querschnitt x Dichte) und den spezifischen Wärmen nach ISO 10456; Bügel und Rad stehen im Schatten des Dachs (besonnt 0,2 bis 0,25 an einem Sommernachmittag), das Dach in voller Sonne."),
]

SYMBOL_TEXT = {
    "title": t("Bicycle shelter, true extent", "Fahrradunterstand, wahre Ausdehnung"),
    "description": t("The shelter at real size for site plans: the 2.25 x 5.04 m roof, three posts and the stands under it; side view with the sloping roof, the rear post and a stand, front view with the roof edge and the three posts.", "Der Unterstand in wahrer Größe für Lagepläne: das Dach 2,25 x 5,04 m, drei Stützen und die Bügel darunter; Seitenansicht mit geneigtem Dach, hinterer Stütze und einem Bügel, Ansicht mit Dachkante und den drei Stützen."),
    "note": t("Model metres at real size in the asset frame (x ahead to the open front, lateral positive to the left, y up); side view from the right, front view from ahead, top view from above, in the default pose.", "Modellmeter in wahrer Größe im Asset-Rahmen (x nach vorn zur offenen Seite, seitlich positiv nach links, y nach oben); Seitenansicht von rechts, Ansicht von vorn, Draufsicht von oben, in der Standardhaltung."),
}
