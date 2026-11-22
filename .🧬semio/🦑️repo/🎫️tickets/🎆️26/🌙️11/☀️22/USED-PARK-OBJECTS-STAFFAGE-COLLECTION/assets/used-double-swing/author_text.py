"""🗣️ Every bilingual text of the used double swing, English first, German second.

@see ./author-double-swing.py — where the texts are placed
"""


def t(en, de):
    """🌍️ One text in both languages."""
    return {"en": en, "de": de}


TITLE = t("Used Double Swing Frame", "Gebrauchte Doppelschaukel")

SUMMARY = t(
    "A municipal timber double swing as it stands after about fifteen years in a park playground: two A-frames of greyed and checked round pine 120 mm on galvanised post anchors carry a 3.30 m round beam 140 mm with four swing hangers 2.30 m above the gravel; a flat EPDM seat and a cradle seat hang on worn short-link chains, and the children's feet have dragged a 90 mm hollow into the safety gravel under the flat seat. The flat seat swings as a 1.64 m pendulum with a child (T = 2.61 s at 30 degrees); in maintenance it is unshackled and set down into its hollow for replacement.",
    "Eine kommunale Doppelschaukel aus Holz, wie sie nach etwa fünfzehn Jahren auf einem Parkspielplatz steht: zwei A-Böcke aus vergrautem, rissigem Kiefernrundholz 120 mm auf verzinkten Pfostenankern tragen einen 3,30 m langen Rundholzbalken 140 mm mit vier Schaukellagern 2,30 m über dem Kies; ein flacher EPDM-Sitz und ein Krippensitz hängen an abgenutzten Rundstahlketten, und die Füße der Kinder haben unter dem Flachsitz eine 90 mm tiefe Mulde in den Fallschutzkies gescharrt. Der Flachsitz schwingt mit einem Kind als Pendel von 1,64 m (T = 2,61 s bei 30 Grad); in der Wartung wird er ausgeschäkelt und zum Austausch in seine Mulde abgesetzt.",
)

BONES = {
    "ground": t("Ground frame", "Bezugsrahmen"),
    "surfacing": t("Safety gravel bed (sample, cut)", "Fallschutzkiesbett (Ausschnitt, geschnitten)"),
    "leg-back": t("Back legs of both A-frames", "Hintere Beine beider A-Böcke"),
    "tie": t("Half-round ties of both A-frames", "Halbrund-Zangen beider A-Böcke"),
    "beam": t("Beam, apex brackets and swing hangers", "Querbalken, Kopfbeschläge und Schaukellager"),
    "leg-front": t("Front legs of both A-frames", "Vordere Beine beider A-Böcke"),
    "chain-flat-l": t("Outer chain of the flat seat", "Äußere Kette des Flachsitzes"),
    "chain-flat-r": t("Inner chain of the flat seat", "Innere Kette des Flachsitzes"),
    "seat-flat": t("Flat seat (hitch at the eye bolts)", "Flachsitz (Gelenk an den Ringschrauben)"),
    "chain-cradle-l": t("Inner chain of the cradle seat", "Innere Kette des Krippensitzes"),
    "chain-cradle-r": t("Outer chain of the cradle seat", "Äußere Kette des Krippensitzes"),
    "seat-cradle": t("Cradle seat with bridles (hitch at the spreader rings)", "Krippensitz mit Gehängen (Gelenk an den Spreizringen)"),
}

SECTIONS = {
    "leg": t("Round pine 120 mm, C24 (EN 338)", "Kiefernrundholz 120 mm, C24 (EN 338)"),
    "beam": t("Round pine 140 mm, C24 (EN 338)", "Kiefernrundholz 140 mm, C24 (EN 338)"),
    "tie": t("Half-round pine 120/60 mm", "Halbrundholz Kiefer 120/60 mm"),
    "chain": t("Two link wires 6 mm in tension (DIN 766 6 x 18.5)", "Zwei Gliedquerschnitte 6 mm auf Zug (DIN 766 6 x 18,5)"),
    "seat-flat": t("Aluminium core 160 x 8 mm of the flat seat", "Aluminiumkern 160 x 8 mm des Flachsitzes"),
    "seat-cradle": t("EPDM pan 284 x 16 mm of the cradle", "EPDM-Boden 284 x 16 mm der Krippe"),
}

HATCH = {
    "gravel": t("Round gravel 2/8", "Rundkies 2/8"),
    "algae": t("Green algae on the shaded face", "Grünalgen auf der Schattenseite"),
}

ANCHORS = {
    "foot-back-l": t("Back foot, left A-frame: post anchor fixed in its concrete foundation", "Hinterer Fuß, linker A-Bock: Pfostenanker im Betonfundament eingespannt"),
    "foot-back-r": t("Back foot, right A-frame: post anchor fixed in its concrete foundation", "Hinterer Fuß, rechter A-Bock: Pfostenanker im Betonfundament eingespannt"),
    "foot-front-l": t("Front foot, left A-frame: post anchor fixed in its concrete foundation", "Vorderer Fuß, linker A-Bock: Pfostenanker im Betonfundament eingespannt"),
    "foot-front-r": t("Front foot, right A-frame: post anchor fixed in its concrete foundation", "Vorderer Fuß, rechter A-Bock: Pfostenanker im Betonfundament eingespannt"),
    "seat-flat-point": t("Sitting point of the flat seat", "Sitzpunkt des Flachsitzes"),
    "seat-cradle-point": t("Sitting point in the cradle", "Sitzpunkt in der Krippe"),
    "seat-flat-rest": t("Underside of the flat seat set down on the gravel", "Unterseite des auf dem Kies abgesetzten Flachsitzes"),
}

LOADS = {
    "child-flat": t("Child on the flat seat (25 kg)", "Kind auf dem Flachsitz (25 kg)"),
    "toddler-cradle": t("Toddler in the cradle (12 kg)", "Kleinkind in der Krippe (12 kg)"),
    "seat-carried": t("Unshackled flat seat carried by the fitter's hands and then by the gravel (its own weight)", "Ausgeschäkelter Flachsitz, von den Händen des Monteurs und danach vom Kies getragen (sein Eigengewicht)"),
}

POSES = {
    "in-service": {
        "title": t("In service", "Im Betrieb"),
        "description": t(
            "Both seats hang still over the worn gravel on a summer afternoon; the frame stands on its four post anchors, fixed in concrete foundations below the gravel.",
            "Beide Sitze hängen an einem Sommernachmittag still über dem ausgetretenen Kies; das Gestell steht auf seinen vier Pfostenankern, eingespannt in Betonfundamente unter dem Kies.",
        ),
    },
    "maintenance": {
        "title": t("Maintenance: flat seat replaced", "Wartung: Flachsitz getauscht"),
        "description": t(
            "The depot fitter has opened both shackles at the eye bolts and set the worn flat seat down into its hollow (EN 1176-7 corrective maintenance); the shackles stay on the chain ends for the new seat, the cradle seat hangs on.",
            "Der Bauhof-Monteur hat beide Schäkel an den Ringschrauben geöffnet und den abgenutzten Flachsitz in seine Mulde abgesetzt (Instandsetzung nach EN 1176-7); die Schäkel bleiben für den neuen Sitz an den Kettenenden, der Krippensitz hängt weiter.",
        ),
    },
    "in-use": {
        "title": t("In use: child at the forward turning point", "In Benutzung: Kind am vorderen Umkehrpunkt"),
        "description": t(
            "A 25 kg child swings on the flat seat and stands at the forward turning point, 30 degrees out, the seat square to its chains; a 12 kg toddler sits in the cradle at rest.",
            "Ein Kind von 25 kg schaukelt auf dem Flachsitz und steht am vorderen Umkehrpunkt, 30 Grad ausgelenkt, der Sitz rechtwinklig zu seinen Ketten; ein Kleinkind von 12 kg sitzt ruhig in der Krippe.",
        ),
    },
}

CLIPS = {
    "maintenance": {
        "title": t("Seat replacement", "Sitztausch"),
        "description": t(
            "From service to maintenance in 6 s: the shackles are opened at 1 s, the flat seat slides down along its chains and is set into the gravel hollow at 5 s.",
            "Vom Betrieb zur Wartung in 6 s: bei 1 s werden die Schäkel geöffnet, der Flachsitz gleitet entlang seiner Ketten nach unten und wird bei 5 s in die Kiesmulde abgesetzt.",
        ),
        "phases": [t("Shackles opened", "Schäkel geöffnet"), t("Seat lowered", "Sitz abgelassen"), t("Seat in its hollow", "Sitz in der Mulde")],
        "events": [t("Shackles opened", "Schäkel geöffnet"), t("Seat sets down on the gravel", "Sitz setzt auf dem Kies auf")],
    },
    "swinging": {
        "title": t("Swinging with a child", "Schaukeln mit Kind"),
        "description": t(
            "One period of the flat seat with a seated child, theta(t) = 30 cos(2 pi t / T), T = 2 pi sqrt(L/g) (1 + theta0^2/16) = 2.61 s for the 1.64 m from the hanger pin to the centre of mass of child and seat; looping, the seat square to its chains, the toddler resting in the cradle.",
            "Eine Periode des Flachsitzes mit sitzendem Kind, theta(t) = 30 cos(2 pi t / T), T = 2 pi sqrt(L/g) (1 + theta0^2/16) = 2,61 s für die 1,64 m vom Lagerbolzen bis zum Schwerpunkt von Kind und Sitz; wiederkehrend, der Sitz rechtwinklig zu seinen Ketten, das Kleinkind ruhig in der Krippe.",
        ),
        "phases": [t("Forward half-swing", "Vorderer Halbschwung"), t("Backward half-swing", "Hinterer Halbschwung")],
        "events": [t("Forward turning point", "Vorderer Umkehrpunkt"), t("Backward turning point", "Hinterer Umkehrpunkt")],
    },
}

LAYERS = {
    "timber-skin": t("Weathered pine, 23.5 mm thermally active depth", "Verwitterte Kiefer, 23,5 mm thermisch wirksame Tiefe"),
    "steel-wall": t("Galvanised steel, 6 mm mean wall", "Verzinkter Stahl, 6 mm mittlere Wand"),
    "link-wire": t("Chain link wire 6 mm (3 mm active)", "Kettendraht 6 mm (3 mm wirksam)"),
    "epdm-flat": t("EPDM over the core, 17.5 mm", "EPDM über dem Kern, 17,5 mm"),
    "epdm-cradle": t("EPDM wall 16 mm (8 mm active from the sunlit face)", "EPDM-Wand 16 mm (8 mm wirksam von der Sonnenseite)"),
    "gravel-top": t("Loose round gravel, 43 mm thermally active depth", "Loser Rundkies, 43 mm thermisch wirksame Tiefe"),
}

REGIONS = {
    "timber-frame": t("Greyed round-timber legs, ties and beam", "Vergraute Rundholzbeine, Zangen und Balken"),
    "frame-steel": t("Galvanised post anchors, apex brackets, hangers and bolts", "Verzinkte Pfostenanker, Kopfbeschläge, Schaukellager und Schrauben"),
    "chain-steel": t("Chains, shackles, eye bolts, spreader rings", "Ketten, Schäkel, Ringschrauben, Spreizringe"),
    "seat-flat": t("Black EPDM flat seat", "Schwarzer EPDM-Flachsitz"),
    "seat-cradle": t("Faded EPDM cradle seat", "Ausgeblichener EPDM-Krippensitz"),
    "gravel-surface": t("Worn surface of the safety gravel", "Ausgetretene Oberfläche des Fallschutzkieses"),
}

MATERIALS = {
    "pine-weathered": t("Weathered pine round timber, pressure-treated", "Verwittertes Kiefernrundholz, kesseldruckimprägniert"),
    "steel-galvanised-weathered": t("Hot-dip galvanised steel, weathered", "Feuerverzinkter Stahl, bewittert"),
    "steel-chain-worn": t("Galvanised chain steel, worn", "Verzinkter Kettenstahl, abgenutzt"),
    "epdm-seat-black": t("Black EPDM rubber", "Schwarzer EPDM-Gummi"),
    "epdm-cradle-faded": t("EPDM rubber, UV-faded", "EPDM-Gummi, UV-ausgeblichen"),
    "aluminium-core": t("Aluminium seat core", "Aluminium-Sitzkern"),
    "gravel-round-2-8": t("Round gravel 2/8 mm, loose", "Rundkies 2/8 mm, lose"),
}

THERMAL_ASSUMPTIONS = [
    t(
        "Summer afternoon in a park playground: 800 W/m2 sun, 28 degC air, 1.5 m/s wind; six passive regions, no emitters and no rated power (0 W, passive, stated on the timber region). The black EPDM flat seat (solar absorptance 0.93) is the hot surface a child sits on; the weathered grey pine absorbs 0.65 of the sun where new pine would absorb about 0.45, the weathered galvanised steel 0.62 where new zinc would absorb 0.4 to 0.5.",
        "Sommernachmittag auf einem Parkspielplatz: 800 W/m2 Sonne, 28 degC Luft, 1,5 m/s Wind; sechs passive Regionen, keine Emitter und keine Nennleistung (0 W, passiv, an der Holzregion angegeben). Der schwarze EPDM-Flachsitz (solarer Absorptionsgrad 0,93) ist die heiße Fläche, auf die sich ein Kind setzt; die verwitterte graue Kiefer absorbiert 0,65 der Sonne, wo neue Kiefer etwa 0,45 absorbieren würde, der bewitterte verzinkte Stahl 0,62, wo neues Zink 0,4 bis 0,5 absorbieren würde.",
    ),
    t(
        "Heat capacities are rho cp times the depth heat reaches in one hour, sqrt(a 3600 s): 23.5 mm for pine, 43 mm for loose gravel; the steel and the rubber count their real wall. The gravel region is the worn top surface of the bed; its cut faces against the surrounding surfacing and the subsoil are section faces and exchange no heat in this model.",
        "Wärmekapazitäten sind rho cp mal die Tiefe, die Wärme in einer Stunde erreicht, sqrt(a 3600 s): 23,5 mm bei Kiefer, 43 mm bei losem Kies; Stahl und Gummi zählen ihre wahre Wand. Die Kiesregion ist die ausgetretene Oberseite des Betts; ihre Schnittflächen zur umgebenden Fallschutzschicht und zum Untergrund sind Schnittflächen und tauschen in diesem Modell keine Wärme.",
    ),
    t(
        "In use the child's clothing covers 55 % of the flat seat at about 33 degC and the toddler half of the cradle; in maintenance the seat lies on the gravel with 45 % of its area in contact. Contact and shade fractions are authored estimates.",
        "In Benutzung bedeckt die Kleidung des Kindes 55 % des Flachsitzes bei etwa 33 degC und das Kleinkind die Hälfte der Krippe; in der Wartung liegt der Sitz mit 45 % seiner Fläche auf dem Kies. Kontakt- und Schattenanteile sind Schätzwerte.",
    ),
]

STRUCTURE_ASSUMPTIONS = [
    t(
        "Four post anchors are fixed supports at the gravel surface: their 60 x 8 mm flat bars are cast into point foundations 0.40 x 0.40 x 0.80 m (C20/25) below the 300 mm gravel layer, which are not part of the asset. The rig is the tree back leg, beam, front leg, so the cut through each leg carries its share of the beam load; the half-round ties close each A-frame at 1.0 m.",
        "Vier Pfostenanker sind feste Auflager an der Kiesoberfläche: ihre Flachstähle 60 x 8 mm sind in Punktfundamente 0,40 x 0,40 x 0,80 m (C20/25) unter der 300 mm starken Kiesschicht einbetoniert, die nicht zum Asset gehören. Der Rig ist der Baum hinteres Bein, Balken, vorderes Bein, sodass der Schnitt durch jedes Bein seinen Anteil der Balkenlast trägt; die Halbrund-Zangen schließen jeden A-Bock auf 1,0 m.",
    ),
    t(
        "The flat seat and the cradle are rigid pendulums about the hanger pins, the right chain of each seat coupled to the left by the same angle; with `inertial` dynamics the swinging clip adds the d'Alembert forces of the child and seat (at the bottom of a 30-degree swing the chains carry 1.54 times the static weight). Chains carry tension only. Timber C24 (EN 338: f_m,k 24 MPa, E 11 GPa); the beam check for two seats at the EN 1176-1 dynamic load is in the report.",
        "Flachsitz und Krippe sind starre Pendel um die Lagerbolzen, die rechte Kette jedes Sitzes ist über denselben Winkel an die linke gekoppelt; mit `inertial` fügt der Schaukel-Clip die d'Alembert-Kräfte von Kind und Sitz hinzu (im tiefsten Punkt eines 30-Grad-Schwungs tragen die Ketten das 1,54-Fache des Eigengewichts). Ketten tragen nur Zug. Holz C24 (EN 338: f_m,k 24 MPa, E 11 GPa); der Balkennachweis für zwei Sitze unter der dynamischen Last nach EN 1176-1 steht im Bericht.",
    ),
    t(
        "The gravel bed is a cut sample of the impact area that rests on the subsoil by itself; it carries no segment mass and is no load on the frame (its 1.01 m3 of loose gravel weigh about 1.6 t). The asset mass is the swing above its foundations.",
        "Das Kiesbett ist ein geschnittener Ausschnitt der Aufprallfläche, der selbst auf dem Untergrund liegt; es trägt keine Segmentmasse und ist keine Last für das Gestell (seine 1,01 m3 losen Kieses wiegen etwa 1,6 t). Die Masse des Assets ist die der Schaukel über ihren Fundamenten.",
    ),
    t(
        "In maintenance the unshackled flat seat is a body of its own: the fitter's hands and then the gravel carry it, applied as a carrying force of its weight (39 N) under the seat, because supports of the archive are springs of equal stiffness that would share the weight of the whole frame with the seat's resting point.",
        "Bei der Wartung ist der ausgeschäkelte Flachsitz ein eigener Körper: die Hände des Monteurs und danach der Kies tragen ihn, angesetzt als Tragkraft seines Gewichts (39 N) unter dem Sitz, weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht des ganzen Gestells mit dem Auflagepunkt des Sitzes teilen würden.",
    ),
]

PROVENANCE = {
    "author": "Semio staffage content (used park objects wave)",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [
        {"id": "en-1176-1-2017", "citation": "EN 1176-1:2017 Playground equipment and surfacing - Part 1: General safety requirements and test methods (user loads and dynamic factors, free height of fall, impact area; loose gravel 2-8 mm: 200 mm layer for a free height of fall up to 2 m plus 100 mm for displacement)."},
        {"id": "en-1176-2-2017", "citation": "EN 1176-2:2017+AC:2019 Playground equipment and surfacing - Part 2: Additional specific safety requirements and test methods for swings (seat at least 350 mm above the surface, distance between seats at least 0.2 l + 300 mm, between seat and frame at least 0.2 l + 200 mm, cradle seats for small children)."},
        {"id": "en-1176-7-2020", "citation": "EN 1176-7:2020 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (routine inspection of chain wear at shackles and bearings, levelling of loose surfacing hollows under swings, replacement of worn seats)."},
        {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24: f_m,k 24 MPa, E0,mean 11 GPa, mean density 420 kg/m3 at 12 % moisture; pressure-treated pine outdoors at about 18 % moisture taken as 530 kg/m3)."},
        {"id": "din-68800-3", "citation": "DIN 68800-3:2020 Wood preservation - Part 3: Preventive protection of wood with wood preservatives (pressure impregnation of pine for use class 4, ground contact avoided by post anchors)."},
        {"id": "din-766", "citation": "DIN 766:2016 Round steel link chains, short link (6 x 18.5: wire 6 mm, pitch 18.5 mm, outer width about 21 mm, about 0.75 kg/m)."},
        {"id": "en-iso-1461-2022", "citation": "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (coating of anchors, brackets and hangers; weathered zinc turns dull grey with white rust, emissivity and absorptance of weathered zinc are authored estimates)."},
        {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, specific heat and density of softwood, steel, aluminium, rubber, sand and gravel)."},
        {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative surface properties, thermal penetration depth)."},
        {"id": "pendulum-halliday", "citation": "Halliday, D., Resnick, R., Walker, J. (2014). Fundamentals of Physics, 10th ed., Wiley, chapter 15 (pendulum period T = 2 pi sqrt(L/g), large-amplitude correction 1 + theta0^2/16)."},
        {"id": "municipal-swing-catalogue", "citation": "Catalogue-typical municipal timber double swing (A-frame of round pine 100-140 mm on steel post anchors, beam 3.0-3.6 m, suspension height 2.3-2.5 m, flat seat 430-480 x 170 mm with aluminium core, cradle seat about 450 x 300 mm), as offered by German playground makers in the 1990s and 2000s; no single product."},
    ],
    "notes": t(
        "Side elevation along the beam, seen from the right: the near A-frame with its back and front leg, the half-round tie, the apex bracket round the beam end, the chains and both seats one behind the other, and the gravel bed cut 0.15 m below its surface with the profiles of the worn hollows. Pendulum length to the centre of mass of a seated child and the seat 1.64 m (hanger pin 2.30 m, seat top 0.435 m); period with the large-amplitude correction 2.61 s at 30 degrees. Clearances (EN 1176-2, l = 1.83 m): seat to seat 0.715 m against 0.67 m, seat to frame 0.58 m against 0.57 m, flat seat 0.40 m and cradle 0.42 m above the original gravel level.",
        "Seitenansicht entlang des Balkens von rechts: der vordere A-Bock mit hinterem und vorderem Bein, der Halbrund-Zange, dem Kopfbeschlag um das Balkenende, die Ketten und beide Sitze hintereinander sowie das 0,15 m unter seiner Oberfläche geschnittene Kiesbett mit den Profilen der ausgetretenen Mulden. Pendellänge bis zum Schwerpunkt von sitzendem Kind und Sitz 1,64 m (Lagerbolzen 2,30 m, Sitzoberseite 0,435 m); Periode mit Großwinkelkorrektur 2,61 s bei 30 Grad. Abstände (EN 1176-2, l = 1,83 m): Sitz zu Sitz 0,715 m gegen 0,67 m, Sitz zu Gestell 0,58 m gegen 0,57 m, Flachsitz 0,40 m und Krippe 0,42 m über dem ursprünglichen Kiesniveau.",
    ),
}

REPRESENTATION = {
    50: {
        "silhouette": t("Side elevation 42 x 53 mm: the A of two greyed round legs in their galvanised post anchors with bolts and nuts, the half-round tie with its carriage bolt heads, the apex bracket (sleeve, two sockets, gusset, nuts) round the beam end, the hangers, the chains with the shackles and both seats, over the gravel bed with the trough under the flat seat and the parent's pit behind the cradle. Front: both A-frames edge-on, the 3.30 m beam with hangers, plates and nuts, four chains, the flat seat with its eye bolts and the cradle with its bridles and leg openings. Plan: the bed with the rims of the hollows and the pushed-out ridges, legs, ties, brackets, beam and both seats.", "Seitenansicht 42 x 53 mm: das A aus zwei vergrauten Rundholzbeinen in ihren verzinkten Pfostenankern mit Schrauben und Muttern, die Halbrund-Zange mit den Köpfen ihrer Schlossschrauben, der Kopfbeschlag (Hülse, zwei Muffen, Knotenblech, Muttern) um das Balkenende, die Lager, die Ketten mit den Schäkeln und beide Sitze über dem Kiesbett mit der Mulde unter dem Flachsitz und der Trittmulde hinter der Krippe. Vorderansicht: beide A-Böcke von der Seite, der 3,30 m lange Balken mit Lagern, Platten und Muttern, vier Ketten, der Flachsitz mit seinen Ringschrauben und die Krippe mit Gehängen und Beinöffnungen. Draufsicht: das Bett mit den Rändern der Mulden und den herausgeschobenen Wällen, Beine, Zangen, Beschläge, Balken und beide Sitze."),
        "retained": t("Every part the 7.5 mm feature shows: the 8 mm anchor bars, M16 nuts 24 mm, the 4 mm socket and 6 mm sleeve walls, shackles, eye bolts and spreader rings, the cradle walls of 16 mm, the 25 mm ridges and the 40 and 90 mm hollows in the gravel.", "Jedes Teil, das das Merkmal 7,5 mm zeigt: die 8-mm-Ankerflachstähle, M16-Muttern 24 mm, die 4-mm-Muffen- und 6-mm-Hülsenwände, das 8-mm-Knotenblech, Schäkel, Ringschrauben und Spreizringe, die 16 mm starken Krippenwände, die 25-mm-Wälle und die 40 und 90 mm tiefen Mulden im Kies."),
        "merged": t("The far A-frame lies exactly behind the near one in the side elevation and the back legs behind the front legs in the front view; the far chain of each seat lies behind the near one. Each chain is drawn as the 21 mm envelope of its links.", "Der hintere A-Bock liegt in der Seitenansicht genau hinter dem vorderen und in der Vorderansicht liegen die hinteren Beine hinter den vorderen; die hintere Kette jedes Sitzes liegt hinter der vorderen. Jede Kette ist als 21-mm-Hülle ihrer Glieder gezeichnet."),
        "omitted": t("The parts of the post anchors and the foundations below the gravel surface, bolt shanks inside the timber, weld seams, the inner nuts hidden behind the legs.", "Die Teile der Pfostenanker und die Fundamente unter der Kiesoberfläche, Schraubenschäfte im Holz, Schweißnähte, die hinter den Beinen verdeckten inneren Muttern."),
        "thinElements": t("The 8 mm anchor bars and gusset at true size (0.16 mm); drying checks, rust at the anchors, chain wear at the shackle, the graffiti tag and the sticker residue as 0.13 mm texture strokes.", "Die 8-mm-Ankerflachstähle und das Knotenblech in wahrer Größe (0,16 mm); Schwindrisse, Rost an den Ankern, Kettenverschleiß am Schäkel, das Graffiti-Tag und Aufkleberreste als 0,13-mm-Texturstriche."),
        "hatching": t("Gravel dots 1.2 mm in the bed; a 0.5 mm stipple of green algae on the shaded back face of the back leg.", "Kiespunkte 1,2 mm im Bett; eine 0,5-mm-Punktierung aus Grünalgen auf der schattigen Rückseite des hinteren Beins."),
        "styles": t("Greyed timber 0.62, galvanised steel 0.45, nuts 0.32, chains 0.36, black flat seat 0.12, faded cradle 0.24, gravel 0.84; body pen 0.5, detail 0.25, texture 0.13.", "Vergrautes Holz 0,62, verzinkter Stahl 0,45, Muttern 0,32, Ketten 0,36, schwarzer Flachsitz 0,12, ausgeblichene Krippe 0,24, Kies 0,84; Körperstift 0,5, Detail 0,25, Textur 0,13."),
        "overlays": t("Legs, ties, beam = timber-frame; anchors, brackets, hangers, bolts = frame-steel; chains, shackles, eyes, rings, bridles = chain-steel; seat-flat; seat-cradle; bed = gravel-surface.", "Beine, Zangen, Balken = timber-frame; Anker, Beschläge, Lager, Schrauben = frame-steel; Ketten, Schäkel, Ösen, Ringe, Gehänge = chain-steel; seat-flat; seat-cradle; Bett = gravel-surface."),
    },
    100: {
        "silhouette": t("Side elevation 21 x 26.5 mm: legs in one anchor shoe each, the half-round tie, sleeve and sockets with the gusset, hangers, chains with shackles, the flat seat with its hitch blocks, the cradle with bridles and leg openings, the bed with both hollows and the ridges. Front and plan as at 1:50 without the hardware.", "Seitenansicht 21 x 26,5 mm: Beine in je einem Ankerschuh, die Halbrund-Zange, Hülse und Muffen mit Knotenblech, Lager, Ketten mit Schäkeln, der Flachsitz mit seinen Gelenkblöcken, die Krippe mit Gehängen und Beinöffnungen, das Bett mit beiden Mulden und den Wällen. Vorder- und Draufsicht wie bei 1:50 ohne Kleineisen."),
        "retained": t("Legs, ties, beam, anchors, apex brackets with their tube walls, hangers, chains, shackles, both seats with the leg openings of the cradle, hollows and ridges.", "Beine, Zangen, Balken, Anker, Kopfbeschläge mit ihren Rohrwänden, Lager, Ketten, Schäkel, beide Sitze mit den Beinöffnungen der Krippe, Mulden und Wälle."),
        "merged": t("The two anchor bars and the leg foot into one 60 x 136 mm shoe; the 8 mm gusset into the sockets; eye ring and shank into one 16 mm hitch block; the spreader rings into the chain ends.", "Die zwei Ankerflachstähle und der Beinfuß zu einem Schuh 60 x 136 mm; das 8-mm-Knotenblech in die Muffen; Ringöse und Schaft zu einem 16-mm-Gelenkblock; die Spreizringe in die Kettenenden."),
        "omitted": t("Nuts, bolt heads, hanger top plates, the eyes of the cradle bridles.", "Muttern, Schraubenköpfe, Kopfplatten der Lager, die Ösen der Krippengehänge."),
        "thinElements": t("The 4 and 6 mm tube walls are drawn as the outlines of the tubes; the 16 mm cradle walls at true size; one drying check per leg, a beam check as texture.", "Die 4- und 6-mm-Rohrwände sind als Umrisse der Rohre gezeichnet; die 16-mm-Krippenwände in wahrer Größe; ein Schwindriss je Bein, ein Balkenriss als Textur."),
        "hatching": t("Gravel dots 1.6 mm.", "Kiespunkte 1,6 mm."),
        "styles": t("Tones as at 1:50; body pen 0.35, detail 0.18, texture 0.13.", "Töne wie bei 1:50; Körperstift 0,35, Detail 0,18, Textur 0,13."),
        "overlays": t("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": t("Side elevation 10.5 x 13 mm: the A of the legs in their shoes, the tie, the bracket as solid sleeve and sockets, the hangers, chains as lines, the flat seat and the cradle as a cup with 30 mm walls, the bed with the trough and the pit.", "Seitenansicht 10,5 x 13 mm: das A der Beine in ihren Schuhen, die Zange, der Beschlag als volle Hülse und Muffen, die Lager, Ketten als Linien, der Flachsitz und die Krippe als Schale mit 30-mm-Wänden, das Bett mit Mulde und Trittmulde."),
        "retained": t("Legs, ties, beam, anchor shoes, brackets, hangers, chains, seats, both hollows.", "Beine, Zangen, Balken, Ankerschuhe, Beschläge, Lager, Ketten, Sitze, beide Mulden."),
        "merged": t("Socket and sleeve walls into solid tubes, the gusset into the sockets, the cradle front with its leg openings into one front wall, shackles and eye bolts into the chain ends, the bridles into the chain line down to the cradle rim.", "Muffen- und Hülsenwände zu vollen Rohren, das Knotenblech in die Muffen, die Krippenfront mit den Beinöffnungen zu einer Frontwand, Schäkel und Ringschrauben in die Kettenenden, die Gehänge in die Kettenlinie bis zum Krippenrand."),
        "omitted": t("Ridges of 25 mm (under the 30 mm feature), all hardware, texture.", "Wälle von 25 mm (unter dem Merkmal 30 mm), alle Kleineisen, Textur."),
        "thinElements": t("Chains as 0.13 mm lines on their centre lines (their 21 mm are 0.1 mm here); the 30 mm cradle walls are generalised from 16 mm.", "Ketten als 0,13-mm-Linien auf ihren Mittellinien (ihre 21 mm sind hier 0,1 mm); die 30-mm-Krippenwände sind aus 16 mm verallgemeinert."),
        "hatching": t("Gravel dots 2.0 mm.", "Kiespunkte 2,0 mm."),
        "styles": t("Tones as at 1:50; body pen 0.25, detail 0.13, no texture.", "Töne wie bei 1:50; Körperstift 0,25, Detail 0,13, keine Textur."),
        "overlays": t("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    500: {
        "silhouette": t("Side elevation 4.2 x 5.3 mm: the A of the two legs standing in the bed, the tie band, the bracket block round the beam end, the chain lines, the flat seat bar and the cradle block over the bed with the trough.", "Seitenansicht 4,2 x 5,3 mm: das A der zwei im Bett stehenden Beine, das Zangenband, der Beschlagblock um das Balkenende, die Kettenlinien, der Flachsitzbalken und der Krippenblock über dem Bett mit der Mulde."),
        "retained": t("Overall size, the A-frames, beam, tie, both seats and the 90 mm trough under the flat seat.", "Gesamtgröße, die A-Böcke, Balken, Zange, beide Sitze und die 90-mm-Mulde unter dem Flachsitz."),
        "merged": t("Anchors into the legs, which stand on the gravel; the half-round tie into a 75 mm band; the flat seat into a 75 mm bar; the cradle into its envelope block.", "Anker in die Beine, die auf dem Kies stehen; die Halbrund-Zange zu einem 75-mm-Band; der Flachsitz zu einem 75-mm-Balken; die Krippe zu ihrem Hüllblock."),
        "omitted": t("Hangers, shackles, rings, the 40 mm pit behind the cradle and the ridges, every detail and texture stroke (no detail pen at this scale).", "Lager, Schäkel, Ringe, die 40-mm-Trittmulde hinter der Krippe und die Wälle, alle Detail- und Texturstriche (kein Detailstift in diesem Maßstab)."),
        "thinElements": t("Chains as 0.13 mm lines; legs 120 mm and beam 140 mm keep their true width.", "Ketten als 0,13-mm-Linien; Beine 120 mm und Balken 140 mm behalten ihre wahre Breite."),
        "hatching": t("None.", "Keine."),
        "styles": t("Tones as at 1:50; body pen 0.18.", "Töne wie bei 1:50; Körperstift 0,18."),
        "overlays": t("Regions as at 1:50; the bracket stands for the frame steel.", "Regionen wie bei 1:50; der Beschlag steht für den Gestellstahl."),
    },
}

VOLUME = {
    50: [
        t("Body at 1:50 (feature 7.5 mm): every part as built. Each A-frame: two round pine legs 120 mm from 0.12 m above the gravel into the sockets of the apex bracket, each foot between two galvanised flat bars 60 x 8 mm cut at the gravel surface with two M16 bolts and four nuts; the half-round tie 120/60 mm on the outer face at 1.0 m with carriage bolt heads; the bracket (sleeve 153/141 mm round the beam, sockets 129/121 mm, 8 mm gusset, nuts). The round beam 140 mm over 3.30 m, four hangers with housing, top plate and nuts; four chains as their 21 mm envelopes with shackles; the flat seat with chamfered edges, eye rings and shanks; the cradle of back wall, pan, two front posts, crotch strap, top bar and side walls 16 mm with eyes, bridles and spreader rings. The gravel bed is a closed heightfield on a 50 mm grid with the 90 mm trough, the 40 mm pit and two 25 mm ridges.", "Körper bei 1:50 (Merkmal 7,5 mm): jedes Teil wie gebaut. Jeder A-Bock: zwei Kiefernrundholzbeine 120 mm von 0,12 m über dem Kies bis in die Muffen des Kopfbeschlags, jeder Fuß zwischen zwei verzinkten Flachstählen 60 x 8 mm, an der Kiesoberfläche geschnitten, mit zwei M16-Schrauben und vier Muttern; die Halbrund-Zange 120/60 mm außen auf 1,0 m mit Schlossschraubenköpfen; der Beschlag (Hülse 153/141 mm um den Balken, Muffen 129/121 mm, 8-mm-Knotenblech, Muttern). Der Rundholzbalken 140 mm über 3,30 m, vier Lager mit Gehäuse, Kopfplatte und Muttern; vier Ketten als ihre 21-mm-Hüllen mit Schäkeln; der Flachsitz mit gefasten Kanten, Ringösen und Schäften; die Krippe aus Rückwand, Boden, zwei Frontpfosten, Schrittsteg, Bügel und Seitenwänden 16 mm mit Ösen, Gehängen und Spreizringen. Das Kiesbett ist ein geschlossenes Höhenfeld auf 50-mm-Raster mit der 90-mm-Mulde, der 40-mm-Trittmulde und zwei 25-mm-Wällen."),
        t("Thermal: six single-layer regions, every solid a reference to its body solid; the bed without its cut faces (the bottom and the four sides continue into the surfacing and the subsoil). The aluminium core of the flat seat lies inside the rubber and is no thermal surface.", "Thermik: sechs einschichtige Regionen, jeder Körper eine Referenz auf sein Körperteil; das Bett ohne seine Schnittflächen (Unterseite und die vier Seiten setzen sich in die Fallschutzschicht und den Untergrund fort). Der Aluminiumkern des Flachsitzes liegt im Gummi und ist keine thermische Fläche."),
        t("Structure: the four legs and two ties as their timber, the beam, the chains as slender rods of the area of two 6 mm link wires (8.5 mm equivalent) from the hanger pin to the hitch, the aluminium core of the flat seat with its eye-bolt shanks, the cradle pan, side walls and bridles. Joints: anchor bars at the feet, sockets, sleeves, carriage bolts, hanger housings, shackles, eye rings, spreader rings and cradle eyes. Four fixed support blocks under the feet; mass markers at the centres of the legs (one per A-frame), ties, beam, chains and seats.", "Tragwerk: die vier Beine und zwei Zangen als ihr Holz, der Balken, die Ketten als schlanke Stäbe mit der Fläche zweier 6-mm-Gliederdrähte (8,5 mm Ersatzdurchmesser) vom Lagerbolzen bis zum Gelenk, der Aluminiumkern des Flachsitzes mit seinen Ringschraubenschäften, Boden, Seitenwände und Gehänge der Krippe. Knoten: Ankerflachstähle an den Füßen, Muffen, Hülsen, Schlossschrauben, Lagergehäuse, Schäkel, Ringösen, Spreizringe und Krippenösen. Vier feste Auflagerblöcke unter den Füßen; Massenmarker in den Schwerpunkten der Beine (einer je A-Bock), Zangen, Balken, Ketten und Sitze."),
    ],
    100: [
        t("Body at 1:100 (feature 15 mm), authored anew: legs, ties and beam at their true sections; each foot in one anchor shoe 60 x 136 mm that takes both flat bars; the bracket keeps its tube walls (sleeve, sockets), the 8 mm gusset is merged into them; hangers as housings without top plates and nuts; chains with shackles; the flat seat with one 16 mm hitch block per side; the cradle with its leg openings and the bridles but without rings and eyes; the bed on a 100 mm grid with trough, pit and ridges.", "Körper bei 1:100 (Merkmal 15 mm), neu modelliert: Beine, Zangen und Balken in ihren wahren Querschnitten; jeder Fuß in einem Ankerschuh 60 x 136 mm, der beide Flachstähle aufnimmt; der Beschlag behält seine Rohrwände (Hülse, Muffen), das 8-mm-Knotenblech ist in sie zusammengefasst; Lager als Gehäuse ohne Kopfplatten und Muttern; Ketten mit Schäkeln; der Flachsitz mit je einem 16-mm-Gelenkblock; die Krippe mit ihren Beinöffnungen und den Gehängen, aber ohne Ringe und Ösen; das Bett auf 100-mm-Raster mit Mulde, Trittmulde und Wällen."),
        t("Thermal: the same six regions as references to the 1:100 body; the bed without its cut faces.", "Thermik: dieselben sechs Regionen als Referenzen auf den Körper 1:100; das Bett ohne seine Schnittflächen."),
        t("Structure: members as at 1:50; joints at the anchor shoes, sockets, sleeves, hangers, shackles and hitch blocks; support blocks and mass markers sized for the 15 mm feature.", "Tragwerk: Stäbe wie bei 1:50; Knoten an den Ankerschuhen, Muffen, Hülsen, Lagern, Schäkeln und Gelenkblöcken; Auflagerblöcke und Massenmarker für das Merkmal 15 mm bemessen."),
    ],
    200: [
        t("Body at 1:200 (feature 30 mm), authored anew: legs in their anchor shoes, ties, beam; the bracket as solid sleeve and sockets (their bores are filled by the beam and the leg heads), the gusset merged; hangers kept; the chains run to the seat tops (shackles and eyes merged) and, at the cradle, straight down to the side-wall rims (bridles merged); the flat seat 35 mm; the cradle as a cup with 30 mm walls and a closed front; the bed on a 150 mm grid with the trough and the pit, the ridges left out.", "Körper bei 1:200 (Merkmal 30 mm), neu modelliert: Beine in ihren Ankerschuhen, Zangen, Balken; der Beschlag als volle Hülse und Muffen (ihre Bohrungen füllen Balken und Beinköpfe), das Knotenblech zusammengefasst; Lager beibehalten; die Ketten laufen bis auf die Sitzoberseiten (Schäkel und Ösen zusammengefasst) und an der Krippe gerade bis auf die Seitenwandränder (Gehänge zusammengefasst); der Flachsitz 35 mm; die Krippe als Schale mit 30-mm-Wänden und geschlossener Front; das Bett auf 150-mm-Raster mit Mulde und Trittmulde, die Wälle entfallen."),
        t("Thermal: the six regions as references to the 1:200 body.", "Thermik: die sechs Regionen als Referenzen auf den Körper 1:200."),
        t("Structure: legs, ties, beam, slender chains reaching the seat core and the cradle rims, the seat core, cradle pan and side walls; joints at the shoes, sockets, sleeves and hangers; support blocks and markers for the 30 mm feature.", "Tragwerk: Beine, Zangen, Balken, schlanke Ketten bis zum Sitzkern und zu den Krippenrändern, Sitzkern, Krippenboden und Seitenwände; Knoten an Schuhen, Muffen, Hülsen und Lagern; Auflagerblöcke und Marker für das Merkmal 30 mm."),
    ],
    500: [
        t("Body at 1:500 (feature 75 mm), authored anew: the legs run down to the gravel (anchors merged), the tie is a 75 mm band on the outer face, the bracket a solid sleeve with solid sockets, the beam 140 mm; the hangers are left out and the chains reach into the beam; the flat seat is generalised to a 75 mm bar, the cradle to its envelope block; the bed on a 300 mm grid keeps only the 90 mm trough.", "Körper bei 1:500 (Merkmal 75 mm), neu modelliert: die Beine reichen bis auf den Kies (Anker zusammengefasst), die Zange ist ein 75-mm-Band auf der Außenseite, der Beschlag eine volle Hülse mit vollen Muffen, der Balken 140 mm; die Lager entfallen und die Ketten reichen in den Balken; der Flachsitz ist zu einem 75-mm-Balken verallgemeinert, die Krippe zu ihrem Hüllblock; das Bett auf 300-mm-Raster behält nur die 90-mm-Mulde."),
        t("Thermal: the six regions as references to the 1:500 body; the bracket is the frame steel at this scale.", "Thermik: die sechs Regionen als Referenzen auf den Körper 1:500; der Beschlag ist bei diesem Maßstab der Gestellstahl."),
        t("Structure: legs, band ties and beam as members, slender chains into the seat bar and onto the cradle block, the seat core sheet and the cradle block; joints at the sockets and sleeves; 105 mm support blocks and 90 mm markers.", "Tragwerk: Beine, Zangenbänder und Balken als Stäbe, schlanke Ketten bis in den Sitzbalken und auf den Krippenblock, das Sitzkernblech und der Krippenblock; Knoten an Muffen und Hülsen; 105-mm-Auflagerblöcke und 90-mm-Marker."),
    ],
}
