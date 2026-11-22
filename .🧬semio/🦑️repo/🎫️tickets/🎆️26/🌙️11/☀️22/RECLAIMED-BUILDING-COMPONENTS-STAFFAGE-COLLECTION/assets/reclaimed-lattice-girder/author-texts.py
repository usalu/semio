"""🗒️ Every bilingual text of the reclaimed lattice girder: summary, references, assumptions and the decisions of every drawn and modelled scale."""


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """🧾️ The eight decisions of one representation."""
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles, "overlays": overlays}


summary = T(
    "A riveted parallel-chord lattice girder of about 1930, taken out of a factory-hall roof: 10.24 m long, 1.00 m deep, span 10.00 m, chords of two L 80 x 80 x 8 angles, verticals 2 L 50 x 50 x 5 and diagonals 2 L 60 x 60 x 6 on 10 mm gusset plates with snap rivets. It shows its service and its dismantling: empty rivet holes, a flame-cut purlin cleat and a cut bracing stub, a slightly burnt end and weathered red-lead paint. Stored on edge on two timber bearers with four props, lifted on two round slings, reinstalled under purlin and snow loads.",
    "Ein genieteter Parallelfachwerkträger von etwa 1930 aus dem Dach einer Fabrikhalle: 10,24 m lang, 1,00 m hoch, Stützweite 10,00 m, Gurte aus je zwei Winkeln L 80 x 80 x 8, Pfosten 2 L 50 x 50 x 5 und Diagonalen 2 L 60 x 60 x 6 auf 10-mm-Knotenblechen mit Halbrundnieten. Er zeigt Nutzung und Rückbau: leere Nietlöcher, eine abgebrannte Pfettenknagge und einen abgetrennten Verbandsstummel, ein angebranntes Ende und verwitterte Bleimennige. Hochkant auf zwei Kanthölzern mit vier Streben gelagert, an zwei Rundschlingen angehoben, unter Pfetten- und Schneelasten wieder eingebaut.",
)

references = [
    {"id": "din-1028", "citation": "DIN 1028:1994 (editions since 1931), hot rolled equal-leg angles of steel; now DIN EN 10056-1:2017. L 80 x 80 x 8: A 12.3 cm2, 9.66 kg/m, e 2.26 cm, I 72.3 cm4, r1 10 mm; L 60 x 60 x 6: A 6.91 cm2, 5.42 kg/m, e 1.69 cm, r1 8 mm; L 50 x 50 x 5: A 4.80 cm2, 3.77 kg/m, e 1.40 cm, r1 7 mm."},
    {"id": "din-124", "citation": "DIN 124:2011 (editions since 1927), round head rivets for steel structures: snap head diameter about 1.6 d and height about 0.65 d; for d = 16 mm a 26 mm head 10.5 mm high, hole 17 mm."},
    {"id": "din-17100", "citation": "DIN 17100:1980 (St 37 since DIN 1612 of 1924), general structural steels; St 37-2 is S235JR of EN 10025-2: yield strength 235 MPa, E 210 GPa, density 7850 kg/m3."},
    {"id": "din-1050", "citation": "DIN 1050:1937, Stahl im Hochbau, Berechnungsgrundlagen: permissible stress of St 37 1400 kg/cm2 (137 MPa) in the main load case; rivet pitch 3 d to 6 d."},
    {"id": "sci-p427", "citation": "Brown, D. G., Pimentel, R. J., Sansom, M. D.: Structural steel reuse: assessment, testing and design principles. SCI P427, The Steel Construction Institute, Ascot 2019 (inspection, testing and reuse of steel from existing buildings)."},
    {"id": "petersen-stahlbau", "citation": "Petersen, C.: Stahlbau. Grundlagen der Berechnung und baulichen Ausbildung von Stahlbauten, 4th ed., Springer Vieweg, Wiesbaden 2013, chapter on riveted connections and lattice girders."},
    {"id": "en-1492-2", "citation": "EN 1492-2:2000+A1:2008, textile slings, round slings of man-made fibres: colour code violet = 1000 kg working load limit in a straight lift, mode factor 0.8 for a choked hitch."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002, Eurocode 1, densities and self-weights; roof cladding with purlins taken as 0.35 kN/m2 (authored estimate)."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 with the German national annex DIN EN 1991-1-3/NA:2019: snow load zone 2 up to 285 m above sea level sk = 0.85 kN/m2, shape coefficient mu1 = 0.8 for a roof pitch up to 30 degrees."},
    {"id": "en-338", "citation": "EN 338:2016, structural timber strength classes: C24 mean density 420 kg/m3, E0,mean 11 GPa; old air-dry spruce of the bearers and props taken at 450 kg/m3."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007, building materials, design thermal values: steel 50 W/mK, 450 to 460 J/kgK; softwood 0.13 W/mK, 1600 J/kgK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017, dynamic thermal characteristics: periodic penetration depth; about half of it is taken as the effective storage depth of an exposed timber face."},
    {"id": "ashrae-fundamentals", "citation": "ASHRAE Handbook Fundamentals (2021), radiation properties of surfaces: weathered and rusted steel emissivity about 0.85 to 0.9, solar absorptance 0.7 to 0.85; red-lead paint about 0.9 / 0.7 (typical rounded values)."},
    {"id": "trgs-505", "citation": "TRGS 505 Blei (Technische Regeln für Gefahrstoffe, 2007, amended): lead-containing coatings such as red-lead primer are assessed before reuse and not ground or burnt without protection."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002, technical drawings, conventions for areas on sections and cuts (no hatch is used: the girder is never drawn cut)."},
]

notes = T(
    "Panel length 1.00 m, eleven nodes from x = 0.12 m to 10.12 m (bearing centres), chord axes 955 mm apart. Pratt pattern: diagonals fall towards midspan and are in tension under gravity. Rivet heads are drawn and modelled at 1:50 only. Lifting points 1.50 m either side of midspan in the middle of panels 3 and 6; slings 2.40 m to the hook. Stored, lifted and reinstalled poses share one rig; the bearers slide out and the props tip down for the reinstalled pose.",
    "Feldlänge 1,00 m, elf Knoten von x = 0,12 m bis 10,12 m (Auflagermitten), Gurtachsen 955 mm auseinander. Strebenfachwerk mit fallenden Diagonalen zur Feldmitte, unter Eigengewicht auf Zug. Nietköpfe sind nur bei 1:50 gezeichnet und modelliert. Anschlagpunkte 1,50 m beiderseits der Feldmitte in der Mitte der Felder 3 und 6; Schlingen 2,40 m bis zum Haken. Gelagerte, angehobene und wieder eingebaute Pose teilen ein Rig; für den Wiedereinbau gleiten die Unterleghölzer heraus und die Streben kippen ab.",
)

thermal = [
    T("All faces are passive. The paint is a weathered red-lead primer with patches of grey top coat (emissivity 0.9, solar absorptance 0.72); the bare bearing plates are rusted (0.85 / 0.8); the flame-cut faces carry their oxide skin (0.8 / 0.85).", "Alle Flächen sind passiv. Der Anstrich ist eine verwitterte Bleimennige mit Flecken grauer Deckschicht (Emissionsgrad 0,9, solarer Absorptionsgrad 0,72); die blanken Lagerplatten sind angerostet (0,85 / 0,8); die Brennschnittflächen tragen ihre Zunderhaut (0,8 / 0,85)."),
    T("The areal heat capacity of the lattice is its whole steel mass times 460 J/kgK over its outer area: the 5 to 10 mm sections follow the air within minutes, so sun and shade show at once.", "Die flächenbezogene Wärmekapazität des Fachwerks ist seine ganze Stahlmasse mal 460 J/kgK über seiner Außenfläche: die 5- bis 10-mm-Profile folgen der Luft in Minuten, Sonne und Schatten zeigen sich sofort."),
    T("The timber bearers and props store heat in about 36 mm of their faces (half the 24 h penetration depth, EN ISO 13786); a sixth of the bearer surface lies on the yard at 12 degrees C, the props lie on it with a quarter of their surface once tipped down.", "Kanthölzer und Streben speichern Wärme in etwa 36 mm ihrer Oberfläche (halbe 24-h-Eindringtiefe, EN ISO 13786); ein Sechstel der Oberfläche der Unterleghölzer liegt auf dem Platz bei 12 Grad C, die abgelegten Streben mit einem Viertel ihrer Oberfläche."),
    T("Reference case: a clear spring afternoon in the yard of the component exchange, 18 degrees C air, 650 W/m2 sun; under the reinstalled roof the sunlit shares fall to a tenth.", "Referenzfall: ein klarer Frühlingsnachmittag auf dem Platz der Bauteilbörse, Luft 18 Grad C, Sonne 650 W/m2; unter dem Dach des Wiedereinbaus fallen die besonnten Anteile auf ein Zehntel."),
]

structure = [
    T("The lattice is one member with the section of its chords: 4 L 80 x 80 x 8, A 49.2 cm2, I 0.00112 m4 about the centroid (chord axes 955 mm apart); St 37 with E 210 GPa. Bending moment and shear along it are the girder's global action; the bar forces follow as chord force M / 0.955 m and diagonal force V / sin 43.7 degrees.", "Das Fachwerk ist ein Stab mit dem Querschnitt seiner Gurte: 4 L 80 x 80 x 8, A 49,2 cm2, I 0,00112 m4 um den Schwerpunkt (Gurtachsen 955 mm auseinander); St 37 mit E 210 GPa. Moment und Querkraft entlang des Stabs sind die globale Beanspruchung des Trägers; die Stabkräfte folgen als Gurtkraft M / 0,955 m und Diagonalkraft V / sin 43,7 Grad."),
    T("Stored: the girder stands on two squared-timber bearers under bottom nodes 1 and 9, each bearer on the yard at both ends (unilateral contacts, friction 0.5); the props only stop it tipping and carry nothing under gravity.", "Gelagert: der Träger steht auf zwei Kanthölzern unter den Untergurtknoten 1 und 9, jedes Kantholz an beiden Enden auf dem Platz (einseitige Kontakte, Reibung 0,5); die Streben verhindern nur das Kippen und tragen unter Eigengewicht nichts."),
    T("Lifted: the two slings meet at the hook, which is the only support (two coincident pins); the solver treats the asset as one rigid body, so the 31 kg of bearers and props that stay on the yard are hung on the hook as well, 4 percent too much hook force.", "Angehoben: die beiden Schlingen treffen sich am Haken, er ist das einzige Auflager (zwei zusammenfallende Gelenke); der Löser behandelt das Asset als einen starren Körper, so hängen auch die 31 kg der auf dem Platz verbliebenen Unterleghölzer und Streben am Haken, 4 Prozent zu viel Hakenkraft."),
    T("Reinstalled: simply supported on its bearing plates, fixed left and sliding right, span 10.00 m; roof and purlins 0.35 kN/m2 and snow 0.68 kN/m2 over a 5.0 m bay arrive as point loads at the six purlin nodes (half loads at the ends). Bearers and props are again counted with the girder (3 percent).", "Wieder eingebaut: Einfeldträger auf seinen Lagerplatten, links fest und rechts verschieblich, Stützweite 10,00 m; Dach und Pfetten 0,35 kN/m2 und Schnee 0,68 kN/m2 über ein 5,0-m-Feld kommen als Einzellasten an den sechs Pfettenknoten an (an den Enden halbe Lasten). Unterleghölzer und Streben werden wieder mit dem Träger gerechnet (3 Prozent)."),
    T("Reuse check after SCI P427: no section loss beyond the paint, the empty rivet holes lie in the cut bracing gusset and in the purlin cleats only, outside the chords; the girder is fit for reuse after testing the steel and assessing the lead paint (TRGS 505).", "Wiederverwendungsprüfung nach SCI P427: kein Querschnittsverlust über den Anstrich hinaus, die leeren Nietlöcher liegen nur im abgetrennten Verbandsblech und in den Pfettenknaggen, außerhalb der Gurte; der Träger ist nach Prüfung des Stahls und Bewertung der Bleifarbe (TRGS 505) wiederverwendbar."),
]

representations = {
    "50": D(
        T("Side elevation of the whole lattice at 1:50 (205 x 22 mm on paper): the near angles of both chords, eleven verticals and ten diagonals as filled strips at their true leg widths of 80, 50 and 60 mm, the 22 gusset plates behind them, every near snap rivet head (26 mm, 0.52 mm), the purlin cleats and their stump, the bearing plates, the choke loops and the slings, the two bearers end-on and the near props in front. Front view: the end of the girder as one filled outline of everything behind it, bearers, props and slings end-on. Plan: the top chord, cleats, bearing plates with their anchor holes, the bracing gusset with its cut stub, bearers, props and slings.", "Seitenansicht des ganzen Fachwerks bei 1:50 (205 x 22 mm auf Papier): die vorderen Winkel beider Gurte, elf Pfosten und zehn Diagonalen als gefüllte Streifen in ihren wahren Schenkelbreiten von 80, 50 und 60 mm, die 22 Knotenbleche dahinter, jeder vordere Halbrundniet (26 mm, 0,52 mm), die Pfettenknaggen und ihr Stummel, die Lagerplatten, die Schnürschlaufen und die Schlingen, die beiden Kanthölzer von vorn und die vorderen Streben davor. Vorderansicht: das Trägerende als eine gefüllte Kontur von allem dahinter, Kanthölzer, Streben und Schlingen von vorn. Draufsicht: Obergurt, Knaggen, Lagerplatten mit ihren Ankerlöchern, das Verbandsblech mit seinem abgetrennten Stummel, Kanthölzer, Streben und Schlingen."),
        T("Every member, gusset and rivet head at its true place; the 15 mm clearance of the web members to the chord legs; the flange edge of each chord; the burnt end of the top chord; the 51 degree sling geometry follows from the 2.40 m slings.", "Jeder Stab, jedes Knotenblech und jeder Nietkopf an seiner wahren Stelle; der Abstand von 15 mm der Füllstäbe zu den Gurtschenkeln; die Flanschkante jedes Gurts; das angebrannte Obergurtende; die Schlingengeometrie von 51 Grad folgt aus den 2,40-m-Schlingen."),
        T("The near and far angle of each member are one strip; the front and the plan of the steelwork are one outline each.", "Vorderer und hinterer Winkel jedes Stabs sind ein Streifen; Vorderansicht und Draufsicht des Stahlbaus sind je eine Kontur."),
        T("The far rivet heads, the fillers and the rivets of the bearing plates and the bracing stub lie behind or inside other parts and are not drawn; the root fillets of the angles (7 to 10 mm) stay inside the pens.", "Die hinteren Nietköpfe, die Futterbleche und die Niete der Lagerplatten und des Verbandsstummels liegen hinter oder in anderen Teilen und werden nicht gezeichnet; die Ausrundungen der Winkel (7 bis 10 mm) bleiben in den Federn."),
        T("The 16 mm bearing plates (0.32 mm) and the 8 mm flange edges are the thinnest parts; flange edges are detail lines.", "Die 16-mm-Lagerplatten (0,32 mm) und die 8-mm-Flanschkanten sind die dünnsten Teile; die Flanschkanten sind Detaillinien."),
        T("No hatch: the girder is never cut.", "Keine Schraffur: der Träger wird nie geschnitten."),
        T("Mid-dark steel tones with lighter gussets and dark rivets; light timber and the pale sling.", "Mitteldunkle Stahltöne mit helleren Knotenblechen und dunklen Nieten; helles Holz und die blasse Schlinge."),
        T("Thermal and structural overlays colour each strip by its region and by the girder member.", "Thermische und statische Überlagerungen färben jeden Streifen nach seiner Region und nach dem Trägerstab."),
    ),
    "100": D(
        T("Side elevation at 1:100 (102 x 11 mm): chords, verticals and diagonals as filled strips, gussets behind, cleats, plates, choke loops, slings, bearers and props; no rivets. Front view and plan as at 1:50 from the merged sections.", "Seitenansicht bei 1:100 (102 x 11 mm): Gurte, Pfosten und Diagonalen als gefüllte Streifen, Knotenbleche dahinter, Knaggen, Platten, Schnürschlaufen, Schlingen, Kanthölzer und Streben; keine Niete. Vorder- und Draufsicht wie bei 1:50 aus den zusammengefassten Querschnitten."),
        T("The true leg widths (0.5 to 0.8 mm), every gusset and the 15 mm clearances (0.15 mm) at the nodes.", "Die wahren Schenkelbreiten (0,5 bis 0,8 mm), jedes Knotenblech und die Abstände von 15 mm (0,15 mm) an den Knoten."),
        T("Each pair of angles is one T section with the 10 mm gap closed; the cleat is one L; the purlin stump goes into the chord.", "Jedes Winkelpaar ist ein T-Querschnitt mit geschlossener 10-mm-Fuge; die Knagge ist ein L; der Pfettenstummel geht im Gurt auf."),
        T("Rivet heads (10.5 mm high), fillers, the bracing stub and the empty holes are below the 15 mm feature.", "Nietköpfe (10,5 mm hoch), Futterbleche, der Verbandsstummel und die leeren Löcher liegen unter dem Merkmal von 15 mm."),
        T("Gussets and the bracing plate are 10 mm sheets, kept true.", "Knotenbleche und Verbandsblech sind 10-mm-Bleche und bleiben wahr."),
        T("No hatch.", "Keine Schraffur."),
        T("As at 1:50 with lighter pens.", "Wie bei 1:50 mit leichteren Federn."),
        T("Overlays per strip.", "Überlagerungen je Streifen."),
    ),
    "200": D(
        T("Side elevation at 1:200 (51 x 5.6 mm): chords as filled 80 mm strips, verticals and diagonals as 0.3 mm strokes on their axes over the gussets, cleats as small blocks; bearers, props and slings. Front view and plan from the block sections.", "Seitenansicht bei 1:200 (51 x 5,6 mm): Gurte als gefüllte 80-mm-Streifen, Pfosten und Diagonalen als 0,3-mm-Striche auf ihren Achsen über den Knotenblechen, Knaggen als kleine Blöcke; Kanthölzer, Streben und Schlingen. Vorder- und Draufsicht aus den Blockquerschnitten."),
        T("Every member of the lattice, every gusset and cleat, the two bearing plates merged into the chord ends.", "Jeder Stab des Fachwerks, jedes Knotenblech und jede Knagge, die beiden Lagerplatten in den Gurtenden."),
        T("Chords are 170 x 80 mm blocks, web members blocks of their angle pairs; the bearing plates are 30 mm blocks reaching into the chord.", "Gurte sind Blöcke 170 x 80 mm, Füllstäbe Blöcke ihrer Winkelpaare; die Lagerplatten sind 30-mm-Blöcke, die in den Gurt reichen."),
        T("Root radii, the T shapes, plate holes and the stump are below the 30 mm feature.", "Ausrundungen, T-Formen, Plattenlöcher und der Stummel liegen unter dem Merkmal von 30 mm."),
        T("Web members (50 and 60 mm, 0.25 and 0.3 mm) are strokes of 0.3 mm pen.", "Füllstäbe (50 und 60 mm, 0,25 und 0,3 mm) sind Striche mit 0,3-mm-Feder."),
        T("No hatch.", "Keine Schraffur."),
        T("Strokes and filled chords read as a lattice at this size.", "Striche und gefüllte Gurte lesen sich in dieser Größe als Fachwerk."),
        T("Overlays colour chords and strokes.", "Überlagerungen färben Gurte und Striche."),
    ),
    "500": D(
        T("At 1:500 (20.5 x 2.2 mm) the lattice is its envelope: one light filled band 10.24 x 1.00 m with the cleats as bumps, chords, verticals and diagonals as thin strokes over it; bearers, props and slings beside it.", "Bei 1:500 (20,5 x 2,2 mm) ist das Fachwerk seine Hüllfläche: ein helles gefülltes Band 10,24 x 1,00 m mit den Knaggen als Höckern, Gurte, Pfosten und Diagonalen als feine Striche darüber; Kanthölzer, Streben und Schlingen daneben."),
        T("Length, depth and the cleat height; the panel rhythm of eleven verticals and ten diagonals.", "Länge, Höhe und Knaggenhöhe; der Feldrhythmus von elf Pfosten und zehn Diagonalen."),
        T("The whole steelwork is one band; bearing plates merge into it.", "Der ganze Stahlbau ist ein Band; die Lagerplatten gehen darin auf."),
        T("Gussets, plates, loops and every detail below 75 mm; slings are not drawn end-on in the front view.", "Knotenbleche, Platten, Schlaufen und jede Einzelheit unter 75 mm; die Schlingen werden in der Vorderansicht nicht von vorn gezeichnet."),
        T("Members are strokes of 0.13 and 0.18 mm over the band.", "Stäbe sind Striche von 0,13 und 0,18 mm über dem Band."),
        T("No hatch.", "Keine Schraffur."),
        T("The light band with dark strokes reads as an open truss.", "Das helle Band mit dunklen Strichen liest sich als offenes Fachwerk."),
        T("Overlays colour the band with the lattice value.", "Überlagerungen färben das Band mit dem Fachwerkwert."),
    ),
}

volumes = {
    "50": [
        T("Body: every angle of the lattice as an extrude of its true section with root fillet (chords L 80 x 80 x 8 the full 10.24 m, verticals L 50 x 50 x 5 between 95 and 905 mm, diagonals L 60 x 60 x 6 trimmed 10 mm clear of chords and verticals), the 22 gussets from the hulls of their rivet groups, fillers at the stitch rivets, every snap rivet head on both faces as repeated half-buried ellipsoids, bearing plates with round and slotted anchor holes, five purlin cleats with their bolt holes and the flame-cut stump, the bracing gusset with its cut L 60 stub and empty holes, the two choke loops; bearers, props and slings.", "Körper: jeder Winkel des Fachwerks als Extrusion seines wahren Querschnitts mit Ausrundung (Gurte L 80 x 80 x 8 über die vollen 10,24 m, Pfosten L 50 x 50 x 5 zwischen 95 und 905 mm, Diagonalen L 60 x 60 x 6, 10 mm vor Gurten und Pfosten abgelängt), die 22 Knotenbleche aus den Hüllen ihrer Nietgruppen, Futter an den Heftnieten, jeder Halbrundnietkopf auf beiden Seiten als wiederholte, halb eingebettete Ellipsoide, Lagerplatten mit runden und Langlöchern, fünf Pfettenknaggen mit Schraubenlöchern und der abgebrannte Stummel, das Verbandsblech mit seinem abgetrennten L-60-Stummel und leeren Löchern, die beiden Schnürschlaufen; Kanthölzer, Streben und Schlingen."),
        T("Thermal: every body solid in its region (painted lattice, bare bearing plates, bearers, props, slings).", "Thermisch: jeder Körper in seiner Region (gestrichenes Fachwerk, blanke Lagerplatten, Kanthölzer, Streben, Schlingen)."),
        T("Structure: chords, verticals and diagonals are the members of the girder bone, the gussets its joints; bearers, props and slings are their own members, the choke loops the sling joints. Supports are the pose contacts (bearers, hook, bearing plates).", "Tragwerk: Gurte, Pfosten und Diagonalen sind die Stäbe des Trägerknochens, die Knotenbleche seine Gelenke; Kanthölzer, Streben und Schlingen sind eigene Stäbe, die Schnürschlaufen die Gelenke der Schlingen. Auflager sind die Kontakte der Posen (Kanthölzer, Haken, Lagerplatten)."),
    ],
    "100": [
        T("Body: each angle pair is one T-section extrude (gap closed), gussets and the bracing plate are 10 mm declared sheets, cleats single L extrudes, bearing plates as at 1:50 with their holes; rivets, fillers, the stump and the bracing stub are below the 15 mm feature and left out.", "Körper: jedes Winkelpaar ist eine T-Extrusion (Fuge geschlossen), Knotenbleche und Verbandsblech sind erklärte 10-mm-Bleche, Knaggen einfache L-Extrusionen, Lagerplatten wie bei 1:50 mit ihren Löchern; Niete, Futter, Stummel und Verbandsstummel liegen unter dem Merkmal von 15 mm und entfallen."),
        T("Thermal and structure as at 1:50 on these solids.", "Thermik und Tragwerk wie bei 1:50 auf diesen Körpern."),
    ],
    "200": [
        T("Body: chords as 170 x 80 mm blocks, verticals and diagonals as blocks of their angle pairs (50 x 110 and 60 x 130 mm), gussets and bracing plate as 10 mm sheets, cleats as 80 mm blocks, bearing plates as 30 mm blocks reaching into the chord ends; bearers, props and slings.", "Körper: Gurte als Blöcke 170 x 80 mm, Pfosten und Diagonalen als Blöcke ihrer Winkelpaare (50 x 110 und 60 x 130 mm), Knotenbleche und Verbandsblech als 10-mm-Bleche, Knaggen als 80-mm-Blöcke, Lagerplatten als 30-mm-Blöcke bis in die Gurtenden; Kanthölzer, Streben und Schlingen."),
        T("Thermal and structure on these solids as at 1:100.", "Thermik und Tragwerk auf diesen Körpern wie bei 1:100."),
    ],
    "500": [
        T("Body: the lattice is typified as its envelope prism 10.24 x 1.00 x 0.17 m (`envelope: true`), as the 1:500 drawing; the cleats stay as 80 mm blocks so the height holds; bearers, props and slings.", "Körper: das Fachwerk ist als seine Hüllprisma 10,24 x 1,00 x 0,17 m typisiert (`envelope: true`), wie die Zeichnung 1:500; die Knaggen bleiben als 80-mm-Blöcke, damit die Höhe stimmt; Kanthölzer, Streben und Schlingen."),
        T("Thermal: the envelope stands for the painted lattice (coverage only), the bearing plates are 75 mm blocks inside it.", "Thermisch: die Hülle steht für das gestrichene Fachwerk (nur Abdeckung), die Lagerplatten sind 75-mm-Blöcke in ihr."),
        T("Structure: chords, verticals and diagonals keep their own slender member solids at the true sections of their angle pairs inside the envelope.", "Tragwerk: Gurte, Pfosten und Diagonalen behalten eigene schlanke Stabkörper in den wahren Querschnitten ihrer Winkelpaare in der Hülle."),
    ],
}

texts = {"summary": summary, "references": references, "notes": notes, "thermal": thermal, "structure": structure, "representations": representations, "volumes": volumes}
