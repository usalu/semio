"""🌐 The bilingual texts of the used TV sideboard (English first, German second)."""


def T(en, de):
    """🔤 One text in both languages."""
    return {"en": en, "de": de}


TXT = {
    "title": T("Used TV Sideboard", "Gebrauchtes TV-Lowboard"),
    "summary": T(
        "A second-hand chipboard TV lowboard 1.80 x 0.40 x 0.45 m in scuffed white foil with two drop-down flap doors, a drawer under an open niche with a cable hole, on six plastic glide feet; its top is yellowed by years of window sun except where the TV foot stood, one door corner is chipped and the lost drawer handle was replaced by a beech knob. Stored in the reuse depot with its fronts taped shut and a price label on the top, carried level by two people, back in use under a 55 inch TV.",
        "Ein gebrauchtes TV-Lowboard aus Spanplatte, 1,80 x 0,40 x 0,45 m, in abgestoßener weißer Folie mit zwei Klappentüren nach unten und einer Schublade unter einem offenen Fach mit Kabeldurchlass, auf sechs Kunststoffgleitern; die Deckplatte ist von Jahren Fenstersonne vergilbt außer dort, wo der Fernseherfuß stand, eine Türecke ist abgeplatzt, und der verlorene Schubladengriff wurde durch einen Buchenknopf ersetzt. Im Gebrauchtwarenlager mit zugeklebten Fronten und Preisschild auf der Platte, von zwei Personen waagrecht getragen, wieder in Gebrauch unter einem 55-Zoll-Fernseher.",
    ),
    "provenance-notes": T(
        "Flat-pack lowboard of the 2000s type: 16 mm particleboard P2 in white paper foil, top 1.80 x 0.40 m lying on the sides, bottom between the sides 25 mm above the floor on six PP glide feet (40 mm, 25 mm high), two dividers making three compartments of 0.579 m, 3 mm HDF back nailed on with a 60 mm cable hole behind the open niche, a fixed niche shelf at 0.185 m, the drawer (16 mm front, 12 mm sides, 3 mm HDF bottom) on roller runners of 12.5 mm, the flaps on flap hinges at the bottom front edge of the carcass so that the opened flap lies flush with the bottom board. Traces modelled: chip 30 x 12 x 6 mm at the outer top corner of the right flap, the drawer knob replacing a 128 mm bar handle (its two screw holes left), scuffs along the edges, ring marks of a mug and a plant pot, the paler patch under the former TV foot. Storage state: fronts taped shut with 50 mm packing tape cut at the joints when opened, depot price label. Masses from board volume x density (650 kg/m3 particleboard, 850 kg/m3 HDF): 34.0 kg.",
        "Zerlegbares Lowboard der 2000er-Jahre: 16 mm Spanplatte P2 mit weißer Papierfolie, Deckplatte 1,80 x 0,40 m auf den Seiten liegend, Boden zwischen den Seiten 25 mm über dem Fußboden auf sechs PP-Gleitern (40 mm, 25 mm hoch), zwei Mittelwände für drei Fächer von 0,579 m, 3 mm HDF-Rückwand aufgenagelt mit 60-mm-Kabeldurchlass hinter dem offenen Fach, fester Fachboden auf 0,185 m, die Schublade (16-mm-Front, 12-mm-Zargen, 3-mm-HDF-Boden) auf Rollenauszügen von 12,5 mm, die Klappen auf Klappenscharnieren an der unteren Vorderkante des Korpus, so dass die geöffnete Klappe bündig mit dem Boden liegt. Modellierte Spuren: Abplatzung 30 x 12 x 6 mm an der oberen Außenecke der rechten Klappe, der Schubladenknopf anstelle eines 128-mm-Bügelgriffs (dessen zwei Schraublöcher geblieben sind), Abstoßungen an den Kanten, Ränder einer Tasse und eines Blumentopfs, die hellere Fläche unter dem früheren Fernseherfuß. Lagerzustand: Fronten mit 50-mm-Packband zugeklebt, beim Öffnen an den Fugen aufgeschnitten, Preisschild des Lagers. Massen aus Plattenvolumen x Dichte (650 kg/m3 Spanplatte, 850 kg/m3 HDF): 34,0 kg.",
    ),
    "mat-foil": T("Particleboard P2, 16 mm, white paper foil", "Spanplatte P2, 16 mm, weiße Papierfolie"),
    "mat-foil-yellowed": T("Particleboard P2, 16 mm, sun-yellowed white foil (top)", "Spanplatte P2, 16 mm, sonnenvergilbte weiße Folie (Deckplatte)"),
    "mat-hdf": T("High-density fibreboard, 3 mm (back, drawer bottom)", "Hochdichte Faserplatte, 3 mm (Rückwand, Schubladenboden)"),
    "mat-pp": T("Polypropylene glide feet", "Kunststoffgleiter aus Polypropylen"),
    "mat-alu": T("Satin-anodised aluminium bar handles", "Bügelgriffe aus matt eloxiertem Aluminium"),
    "mat-steel": T("Zinc-plated steel runners and hinges", "Verzinkte Stahlauszüge und Scharniere"),
    "mat-beech": T("Lacquered beech knob", "Lackierter Buchenknopf"),
    "mat-tape": T("Polypropylene packing tape, 50 mm", "Packband aus Polypropylen, 50 mm"),
    "mat-label": T("Paper price label", "Papier-Preisschild"),
    "section-door": T("Flap 598 x 16 mm", "Klappe 598 x 16 mm"),
    "bone-ground": T("Floor reference", "Bodenbezug"),
    "bone-carcass": T("Carcass with top, back, niche shelf, runners and feet", "Korpus mit Deckplatte, Rückwand, Fachboden, Auszügen und Füßen"),
    "bone-flap-l": T("Left flap door (hinged at its bottom edge)", "Linke Klappentür (an der Unterkante angeschlagen)"),
    "bone-flap-r": T("Right flap door with the chipped corner", "Rechte Klappentür mit der abgeplatzten Ecke"),
    "bone-drawer": T("Drawer on roller runners", "Schublade auf Rollenauszügen"),
    "anchor-foot-front-left": T("Front left glide foot", "Gleiter vorne links"),
    "anchor-foot-front-middle": T("Front middle glide foot", "Gleiter vorne Mitte"),
    "anchor-foot-front-right": T("Front right glide foot", "Gleiter vorne rechts"),
    "anchor-foot-back-left": T("Back left glide foot", "Gleiter hinten links"),
    "anchor-foot-back-middle": T("Back middle glide foot", "Gleiter hinten Mitte"),
    "anchor-foot-back-right": T("Back right glide foot", "Gleiter hinten rechts"),
    "anchor-grip-left": T("Left carrier's hands under the bottom board", "Hände des linken Trägers unter dem Boden"),
    "anchor-grip-right": T("Right carrier's hands under the bottom board", "Hände des rechten Trägers unter dem Boden"),
    "load-tv": T("55 inch LED TV on its centre foot, 18 kg", "55-Zoll-LED-Fernseher auf Mittelfuß, 18 kg"),
    "load-receiver": T("AV receiver in the niche, 4 kg", "AV-Receiver im offenen Fach, 4 kg"),
    "load-media-left": T("DVDs and games behind the left flap, 6 kg", "DVDs und Spiele hinter der linken Klappe, 6 kg"),
    "load-media-right": T("Media behind the right flap, 6 kg", "Medien hinter der rechten Klappe, 6 kg"),
    "load-drawer-items": T("Remote controls and cables in the drawer, 2 kg", "Fernbedienungen und Kabel in der Schublade, 2 kg"),
    "pose-stored": T("Stored in the depot", "Im Lager abgestellt"),
    "pose-stored-d": T("Emptied, wiped and standing on its six glide feet in the reuse depot, the flaps and the drawer taped shut with packing tape, a price label on the top.", "Geleert, abgewischt und auf seinen sechs Gleitern im Wiederverwendungslager stehend, Klappen und Schublade mit Packband zugeklebt, ein Preisschild auf der Deckplatte."),
    "pose-carried": T("Carried by two people", "Von zwei Personen getragen"),
    "pose-carried-d": T("Lifted level by two people gripping under the bottom board at the ends, its underside 0.725 m above the floor; the tape keeps the fronts shut, each pair of hands takes half of the 34 kg.", "Von zwei Personen waagrecht gehoben, die an den Enden unter den Boden greifen, Unterseite 0,725 m über dem Fußboden; das Packband hält die Fronten zu, jedes Händepaar trägt die Hälfte der 34 kg."),
    "pose-in-use": T("In use under a TV", "In Gebrauch unter einem Fernseher"),
    "pose-in-use-d": T("Back in a living room: a 55 inch TV on its centre foot on the top, an AV receiver in the niche, media behind the flaps, small items in the drawer; the cut tape ends are still stuck at the joints.", "Wieder in einem Wohnzimmer: ein 55-Zoll-Fernseher auf Mittelfuß auf der Platte, ein AV-Receiver im offenen Fach, Medien hinter den Klappen, Kleinteile in der Schublade; die aufgeschnittenen Klebebandenden kleben noch an den Fugen."),
    "pose-flap-open": T("Left flap open", "Linke Klappe offen"),
    "pose-flap-open-d": T("The left flap dropped through 90 degrees about its bottom hinge line, lying flush with the bottom board and held by its stay.", "Die linke Klappe um 90 Grad um ihre untere Scharnierlinie heruntergeklappt, bündig mit dem Boden und von ihrem Klappenhalter gehalten."),
    "pose-drawer-out": T("Drawer pulled out", "Schublade herausgezogen"),
    "pose-drawer-out-d": T("The drawer pulled out 0.30 m on its roller runners, its last 54 mm still in the carcass.", "Die Schublade auf ihren Rollenauszügen 0,30 m herausgezogen, die letzten 54 mm noch im Korpus."),
    "clip-carry": T("Carry", "Tragen"),
    "clip-carry-d": T("Two people take hold under the ends and lift the taped lowboard from the depot floor to carrying height.", "Zwei Personen greifen unter die Enden und heben das zugeklebte Lowboard vom Lagerboden auf Traghöhe."),
    "clip-life": T("Flap and drawer", "Klappe und Schublade"),
    "clip-life-d": T("A day at the TV: the left flap is dropped to take out a game and lifted again, then the drawer is pulled for the remote and pushed back.", "Ein Tag am Fernseher: die linke Klappe wird für ein Spiel heruntergeklappt und wieder hochgeklappt, dann wird die Schublade für die Fernbedienung aufgezogen und zurückgeschoben."),
    "phase-grip": T("Grip under the ends", "Unter die Enden greifen"),
    "phase-lift": T("Lift to carrying height", "Auf Traghöhe heben"),
    "phase-flap-down": T("Flap drops", "Klappe öffnet"),
    "phase-flap-held": T("Flap open", "Klappe offen"),
    "phase-flap-up": T("Flap lifted shut", "Klappe wird geschlossen"),
    "phase-drawer-pull": T("Drawer pulled", "Schublade wird aufgezogen"),
    "phase-drawer-held": T("Drawer out", "Schublade offen"),
    "phase-drawer-push": T("Drawer pushed in", "Schublade wird zugeschoben"),
    "event-grip": T("Hands take the load", "Hände übernehmen die Last"),
    "event-lift-off": T("Feet leave the floor", "Gleiter heben vom Boden ab"),
    "event-flap": T("Left flap released", "Linke Klappe entriegelt"),
    "event-drawer": T("Drawer pulled at its knob", "Schublade am Knopf gezogen"),
    "region-top-foil": T("Top in sun-yellowed foil", "Deckplatte in sonnenvergilbter Folie"),
    "region-carcass-foil": T("Carcass sides, bottom, dividers and niche shelf", "Korpusseiten, Boden, Mittelwände und Fachboden"),
    "region-fronts": T("Two flaps and the drawer front", "Zwei Klappen und Schubladenfront"),
    "region-back-panel": T("HDF back with cable hole", "HDF-Rückwand mit Kabeldurchlass"),
    "region-feet": T("Six glide feet", "Sechs Gleiter"),
    "region-handles": T("Two aluminium bar handles", "Zwei Aluminium-Bügelgriffe"),
    "region-knob": T("Beech knob of the drawer", "Buchenknopf der Schublade"),
    "layer-top-board": T("Particleboard 16 mm", "Spanplatte 16 mm"),
    "layer-carcass-board": T("Particleboard 16 mm", "Spanplatte 16 mm"),
    "layer-front-board": T("Particleboard 16 mm", "Spanplatte 16 mm"),
    "layer-back-board": T("HDF 3 mm", "HDF 3 mm"),
    "layer-foot-shell": T("Polypropylene wall 5 mm", "Polypropylenwand 5 mm"),
    "layer-handle-metal": T("Aluminium, half the bar radius", "Aluminium, halber Stangenradius"),
    "layer-knob-wood": T("Beech, half the knob radius", "Buche, halber Knopfradius"),
    "th-assume-materials": T("Passive surfaces: particleboard 650 kg/m3, 0.13 W/(m K), 1700 J/(kg K) and HDF after ISO 10456; areal heat capacity of each region from its layer (16 mm board 17.7 kJ/(m2 K)). The furniture draws no power: its nameplate is stated as passive 0 W.", "Passive Oberflächen: Spanplatte 650 kg/m3, 0,13 W/(m K), 1700 J/(kg K) und HDF nach ISO 10456; flächenbezogene Wärmekapazität jeder Region aus ihrer Schicht (16-mm-Platte 17,7 kJ/(m2 K)). Das Möbel nimmt keine Leistung auf: sein Typenschild ist als passiv 0 W angegeben."),
    "th-assume-ageing": T("Ageing: new white paper foil absorbs about 0.25 of the solar radiation, the top that stood under a window for years is yellowed to about 0.34 (Incropera Table A.11 range for white coatings, authored estimate); the emissivity of 0.9 is unchanged.", "Alterung: neue weiße Papierfolie absorbiert etwa 0,25 der Sonnenstrahlung, die jahrelang unter einem Fenster stehende Deckplatte ist auf etwa 0,34 vergilbt (Bereich weißer Beschichtungen nach Incropera Tabelle A.11, Schätzwert); der Emissionsgrad 0,9 bleibt."),
    "th-assume-cases": T("Reference case: living room 21 degC, still air, a 350 W/m2 window sun patch on 30 percent of the top in use; the TV foot covers 15 percent of the top at 25 degC; in the depot no sun and a 16 degC floor under the feet.", "Referenzfall: Wohnzimmer 21 degC, ruhige Luft, ein Fenster-Sonnenfleck von 350 W/m2 auf 30 Prozent der Deckplatte in Gebrauch; der Fernseherfuß bedeckt 15 Prozent der Platte bei 25 degC; im Lager keine Sonne und ein 16 degC kalter Boden unter den Gleitern."),
    "st-assume-carcass": T("The carcass is one rigid body of 16 mm P2 boards (EN 312: bending strength 11 MPa, E 1.6 GPa) on six glide feet; the top spans 0.579 m between the dividers: the 18 kg TV at mid-span gives M = P L / 4 = 25.5 Nm, a bending stress of 1.5 MPa and an elastic sag of 3.3 mm, which creep of the board roughly doubles over years (not modelled as geometry, below the 1:50 feature).", "Der Korpus ist ein starrer Körper aus 16-mm-Platten P2 (EN 312: Biegefestigkeit 11 MPa, E 1,6 GPa) auf sechs Gleitern; die Deckplatte spannt 0,579 m zwischen den Mittelwänden: der 18-kg-Fernseher in Feldmitte ergibt M = P L / 4 = 25,5 Nm, eine Biegespannung von 1,5 MPa und eine elastische Durchbiegung von 3,3 mm, die das Kriechen der Platte über Jahre etwa verdoppelt (nicht als Geometrie modelliert, unter dem Merkmal von 1:50)."),
    "st-assume-loads": T("Loads in use (authored estimates in the frame of EN 16122 / EN 14749 storage furniture tests): TV 18 kg on the top, receiver 4 kg on the niche shelf, 6 kg of media in each flap compartment, 2 kg in the drawer; masses from board volume x density sum to 34.0 kg.", "Lasten in Gebrauch (Schätzwerte im Rahmen der Prüfungen für Aufbewahrungsmöbel nach EN 16122 / EN 14749): Fernseher 18 kg auf der Platte, Receiver 4 kg auf dem Fachboden, je 6 kg Medien in den Klappenfächern, 2 kg in der Schublade; die Massen aus Plattenvolumen x Dichte ergeben 34,0 kg."),
    "st-assume-flap": T("An open flap is a 0.39 m cantilever of 2.5 kg from its hinge line (4.9 Nm); the joint moment stands for the two flap hinges and the stay together. The drawer, 2.4 kg plus 2 kg contents, hangs 0.30 m out on its runners.", "Eine offene Klappe ist ein Kragarm von 0,39 m und 2,5 kg an ihrer Scharnierlinie (4,9 Nm); das Gelenkmoment steht für die zwei Klappenscharniere und den Klappenhalter zusammen. Die Schublade, 2,4 kg plus 2 kg Inhalt, hängt 0,30 m ausgezogen an ihren Auszügen."),
    "st-assume-carry": T("Carried, the feet are off the floor and the two hands under the bottom board at the ends (0.85 m either side of the centre) are the only supports, about 167 N each; at 0.45 m height and 0.40 m depth the lowboard is far from the tipping risk EN 14749 addresses for tall storage, so no wall fixing is provided.", "Beim Tragen sind die Gleiter vom Boden, und die zwei Händepaare unter dem Boden an den Enden (je 0,85 m neben der Mitte) sind die einzigen Auflager, je etwa 167 N; mit 0,45 m Höhe und 0,40 m Tiefe ist das Lowboard weit vom Kipprisiko, das EN 14749 für hohe Aufbewahrungsmöbel behandelt, daher keine Wandbefestigung."),
}

_VOL = {
    "50": (
        T("Body 1:50 is the lowboard as built: 16 mm top, sides, bottom, dividers and niche shelf, the 3 mm HDF back with its 60 mm cable hole as a sheet, six turned glide feet with their chamfer, both roller runners, the drawer with 12 mm sides and back and a 3 mm bottom, the two flaps (the right one a polyhedron with the 30 x 12 x 6 mm chip at its outer top corner), two bar handles with their legs, the turned beech knob, the cut packing-tape pieces and the price label as texture solids.", "Der Körper 1:50 ist das Lowboard, wie es gebaut ist: 16-mm-Deckplatte, Seiten, Boden, Mittelwände und Fachboden, die 3-mm-HDF-Rückwand mit 60-mm-Kabeldurchlass als dünne Platte, sechs gedrehte Gleiter mit Fase, beide Rollenauszüge, die Schublade mit 12-mm-Zargen und 3-mm-Boden, die zwei Klappen (die rechte ein Polyeder mit der Abplatzung 30 x 12 x 6 mm an der oberen Außenecke), zwei Bügelgriffe mit Füßen, der gedrehte Buchenknopf, die aufgeschnittenen Packbandstücke und das Preisschild als Texturkörper."),
        T("Thermal 1:50 references the board, back, feet, front, handle and knob surfaces; the closed compartments behind the flaps and the drawer are cavities (joints of 1.5 mm and less), the niche is open. Structure 1:50: every board, the back and the feet as members of the carcass, each flap and the drawer parts as their own members, the hinge lines as joint rods, the runners as the drawer joint and four mass markers; the floor is reached through the feet.", "Die Thermik 1:50 verweist auf die Flächen der Platten, der Rückwand, der Gleiter, der Fronten, der Griffe und des Knopfes; die geschlossenen Fächer hinter den Klappen und der Schublade sind Hohlräume (Fugen von 1,5 mm und weniger), das offene Fach ist offen. Tragwerk 1:50: jede Platte, die Rückwand und die Gleiter als Glieder des Korpus, jede Klappe und die Schubladenteile als eigene Glieder, die Scharnierlinien als Gelenkstäbe, die Auszüge als Schubladengelenk und vier Massenmarken; der Boden wird über die Gleiter erreicht."),
    ),
    "100": (
        T("Body 1:100 keeps every board at 16 mm, the back with the cable hole, the feet as plain cylinders and the knob as a plain cylinder; the drawer box is one block behind its front, the chip and the 12 mm bar handles (below 15 mm) are left out, tape and label stay as texture.", "Der Körper 1:100 behält jede Platte mit 16 mm, die Rückwand mit Kabeldurchlass, die Gleiter als einfache Zylinder und den Knopf als einfachen Zylinder; der Schubladenkasten ist ein Block hinter der Front, die Abplatzung und die 12-mm-Bügelgriffe (unter 15 mm) entfallen, Klebeband und Preisschild bleiben als Textur."),
        T("Thermal 1:100 references the boards, back, feet, fronts and knob; the handles region keeps two 15 mm square bars 120 mm long in front of the flaps, sized to the true handle area. Structure 1:100 as at 1:50 with the drawer box as one member, 15 mm hinge rods and 15 mm runner joints.", "Die Thermik 1:100 verweist auf Platten, Rückwand, Gleiter, Fronten und Knopf; die Griffregion behält zwei 15-mm-Vierkantstäbe von 120 mm vor den Klappen, auf die wahre Grifffläche bemessen. Tragwerk 1:100 wie 1:50 mit dem Schubladenkasten als einem Glied, 15-mm-Scharnierstäben und 15-mm-Auszugsgelenken."),
    ),
    "200": (
        T("Body 1:200: the feet and the 25 mm floor gap are merged into a plinth, so the sides and the bottom reach the floor; the boards keep their 16 mm because the drawing draws them so (top band, fronts in plan), the back is a 3 mm sheet with the cable hole, the drawer is its front and one box block, the knob a cylinder.", "Körper 1:200: die Gleiter und der 25-mm-Bodenspalt sind zu einem Sockel zusammengefasst, Seiten und Boden reichen bis zum Fußboden; die Platten behalten 16 mm, weil die Zeichnung sie so zeigt (Deckplattenband, Fronten im Grundriss), die Rückwand ist eine 3-mm-Platte mit Kabeldurchlass, die Schublade Front und ein Kastenblock, der Knopf ein Zylinder."),
        T("Thermal 1:200: references to the boards and fronts, the lower plinth band as six foot blocks of 40 x 25 x 40 mm and the bottom board above it, handle bars 16 mm. Structure 1:200: the boards with the plinth, flaps and drawer as members, 16 mm hinge rods and runner joints, mass markers of 30 mm.", "Thermik 1:200: Verweise auf Platten und Fronten, das untere Sockelband als sechs Gleiterblöcke 40 x 25 x 40 mm und der Boden darüber, Griffstäbe 16 mm. Tragwerk 1:200: die Platten mit dem Sockel, Klappen und Schublade als Glieder, 16-mm-Scharnierstäbe und Auszugsgelenke, Massenmarken von 30 mm."),
    ),
    "500": (
        T("Body 1:500: the carcass is one mass from the floor to the top with the 16 mm overhang of the top over the fronts; niche, back, dividers and feet are merged into it; the two flaps and the drawer front stay separate plates in front of it (they still move), the drawer box is a block, the knob the only part beyond the front.", "Körper 1:500: der Korpus ist eine Masse vom Fußboden bis zur Deckplatte mit dem 16-mm-Überstand der Platte über die Fronten; offenes Fach, Rückwand, Mittelwände und Gleiter gehen darin auf; die zwei Klappen und die Schubladenfront bleiben eigene Platten davor (sie bewegen sich noch), der Schubladenkasten ist ein Block, der Knopf der einzige Teil vor der Front."),
        T("Thermal 1:500 is modelled anew and hollow as in reality: the top, one U-section for the sides and bottom, one H-section for the dividers and niche shelf, the back with its cable hole, six foot blocks, the fronts and the knob, 16 mm handle bars. Structure 1:500: the carcass mass as one member, flaps and drawer as members, 20 mm hinge rods and runner joints, mass markers of 75 mm.", "Die Thermik 1:500 ist neu und wie in Wirklichkeit hohl modelliert: Deckplatte, ein U-Profil für Seiten und Boden, ein H-Profil für Mittelwände und Fachboden, die Rückwand mit Kabeldurchlass, sechs Gleiterblöcke, die Fronten und der Knopf, Griffstäbe von 16 mm. Tragwerk 1:500: die Korpusmasse als ein Glied, Klappen und Schublade als Glieder, 20-mm-Scharnierstäbe und Auszugsgelenke, Massenmarken von 75 mm."),
    ),
}
for _sc, (_body, _lists) in _VOL.items():
    TXT[f"vol{_sc}-body"] = _body
    TXT[f"vol{_sc}-lists"] = _lists

_REP = {
    "50": {
        "silhouette": T("Lowboard 36 x 9 mm on paper. Side elevation: section A-A through the centre (cable hole, drawer knob, middle feet cut), looking at the left half: cut top, back, bottom, niche shelf and drawer, beyond them the left divider, the drawer side, the edge of the left flap with its handle. Front: the whole face with niche, cable hole, three fronts. Plan: the top with its traces.", "Lowboard 36 x 9 mm auf dem Papier. Seitenansicht: Schnitt A-A durch die Mitte (Kabeldurchlass, Schubladenknopf, mittlere Gleiter geschnitten) mit Blick auf die linke Hälfte: geschnittene Deckplatte, Rückwand, Boden, Fachboden und Schublade, dahinter die linke Mittelwand, die Schubladenzarge, die Kante der linken Klappe mit ihrem Griff. Vorderansicht: die ganze Front mit offenem Fach, Kabeldurchlass, drei Fronten. Grundriss: die Deckplatte mit ihren Spuren."),
        "retained": T("True 16 mm boards and 3 mm back, the 25 mm floor gap with turned feet, the flap hinge line at the bottom front edge, the 60 mm cable hole, both bar handles, the replacement knob and the screw holes of the lost handle, the chip, scuffs, ring marks, the paler TV patch, tape and label.", "Echte 16-mm-Platten und 3-mm-Rückwand, der 25-mm-Bodenspalt mit gedrehten Gleitern, die Scharnierlinie der Klappen an der unteren Vorderkante, der 60-mm-Kabeldurchlass, beide Bügelgriffe, der Ersatzknopf und die Schraublöcher des verlorenen Griffs, die Abplatzung, Abstoßungen, Ränder, die hellere Fernseherfläche, Klebeband und Preisschild."),
        "merged": T("Foil and board are one material; edge banding is part of the board; the runners hidden behind the drawer side are a texture line.", "Folie und Platte sind ein Material; die Kante gehört zur Platte; die hinter der Zarge verborgenen Auszüge sind eine Texturlinie."),
        "omitted": T("Flap hinges and stays, cam fittings, shelf pins and the nails of the back (below 7.5 mm or hidden).", "Klappenscharniere und Klappenhalter, Exzenterverbinder, Bodenträger und die Nägel der Rückwand (unter 7,5 mm oder verborgen)."),
        "thinElements": T("The 3 mm back (0.06 mm) and the drawer bottom are drawn true and read by their outlines; tape is drawn as outlines because it is 0.1 mm thick and turns edge-on when the flap opens.", "Die 3-mm-Rückwand (0,06 mm) und der Schubladenboden sind wahr gezeichnet und wirken durch ihre Kontur; das Klebeband ist als Kontur gezeichnet, weil es 0,1 mm dick ist und beim Öffnen der Klappe hochkant steht."),
        "hatching": T("None; cut faces are dark poché (tone 0.3), foil by light tone.", "Keine; Schnittflächen sind dunkel angelegt (Ton 0,3), die Folie durch hellen Ton."),
        "styles": T("Cut faces 0.3, fronts 0.93 with 0.35 mm outlines, inner boards 0.86 with 0.25 mm, back 0.6 to 0.72, feet 0.35, aluminium 0.5, beech 0.55, traces in texture pen 0.13 mm.", "Schnittflächen 0,3, Fronten 0,93 mit 0,35-mm-Konturen, innere Platten 0,86 mit 0,25 mm, Rückwand 0,6 bis 0,72, Gleiter 0,35, Aluminium 0,5, Buche 0,55, Spuren mit Texturstift 0,13 mm."),
        "overlays": T("One shape per board, front and fitting; regions top-foil, carcass-foil, fronts, back-panel, feet, handles and knob are drawn by their own shapes.", "Eine Form je Platte, Front und Beschlag; die Regionen top-foil, carcass-foil, fronts, back-panel, feet, handles und knob sind durch eigene Formen gezeichnet."),
    },
    "100": {
        "silhouette": T("Lowboard 18 x 4.5 mm on paper. Side elevation from the right: right side, back edge, top edge, right flap edge, the knob of the drawer, two feet. Front with niche, cable hole and three fronts; plan with top, label, tape and the large ring mark.", "Lowboard 18 x 4,5 mm auf dem Papier. Seitenansicht von rechts: rechte Seite, Rückwandkante, Deckplattenkante, Kante der rechten Klappe, der Knopf der Schublade, zwei Gleiter. Vorderansicht mit offenem Fach, Kabeldurchlass und drei Fronten; Grundriss mit Platte, Preisschild, Klebeband und dem großen Rand."),
        "retained": T("16 mm boards, the floor gap with feet, cable hole, knob, tape and label.", "16-mm-Platten, Bodenspalt mit Gleitern, Kabeldurchlass, Knopf, Klebeband und Preisschild."),
        "merged": T("The bar handles are merged into their flaps (region handles drawn with the fronts).", "Die Bügelgriffe gehen in ihren Klappen auf (Region handles mit den Fronten gezeichnet)."),
        "omitted": T("The chip, the screw holes, scuffs, the small ring mark and the TV patch.", "Die Abplatzung, die Schraublöcher, Abstoßungen, der kleine Rand und die Fernseherfläche."),
        "thinElements": T("The 3 mm back at true size within the side outline; the 16 mm boards (0.16 mm) read through their 0.25 mm outlines.", "Die 3-mm-Rückwand in wahrer Größe in der Seitenkontur; die 16-mm-Platten (0,16 mm) wirken durch ihre 0,25-mm-Konturen."),
        "hatching": T("None; tone only.", "Keine; nur Ton."),
        "styles": T("Outlines 0.25 mm, fronts 0.93, carcass 0.84 to 0.88, back 0.6, feet 0.35, tape and label in texture pen.", "Konturen 0,25 mm, Fronten 0,93, Korpus 0,84 bis 0,88, Rückwand 0,6, Gleiter 0,35, Klebeband und Preisschild mit Texturstift."),
        "overlays": T("Flaps and drawer front carry fronts and handles; the side carries carcass-foil.", "Klappen und Schubladenfront tragen fronts und handles; die Seite trägt carcass-foil."),
    },
    "200": {
        "silhouette": T("Lowboard 9 x 2.25 mm on paper standing on the floor: the feet and the 25 mm gap are drawn as a dark plinth band; front with niche and cable hole, three fronts and the knob.", "Lowboard 9 x 2,25 mm auf dem Papier, auf dem Boden stehend: Gleiter und 25-mm-Spalt sind als dunkles Sockelband gezeichnet; Vorderansicht mit offenem Fach und Kabeldurchlass, drei Fronten und Knopf."),
        "retained": T("The three compartments, the niche, the cable hole, the knob that sticks out 25 mm.", "Die drei Fächer, das offene Fach, der Kabeldurchlass, der 25 mm vorstehende Knopf."),
        "merged": T("Feet and floor gap into the plinth band; handles into the flaps.", "Gleiter und Bodenspalt in das Sockelband; Griffe in die Klappen."),
        "omitted": T("Tape, label and every trace of use (below 0.15 mm on paper).", "Klebeband, Preisschild und alle Gebrauchsspuren (unter 0,15 mm auf dem Papier)."),
        "thinElements": T("Boards at 16 mm (0.08 mm) and the back read through the 0.25 mm body pen.", "Platten mit 16 mm (0,08 mm) und die Rückwand wirken durch den 0,25-mm-Körperstift."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, detail 0.13 mm; plinth 0.45, fronts 0.93.", "Körperstift 0,25 mm, Detail 0,13 mm; Sockel 0,45, Fronten 0,93."),
        "overlays": T("The plinth band carries feet and carcass-foil; flaps carry fronts and handles.", "Das Sockelband trägt feet und carcass-foil; die Klappen tragen fronts und handles."),
    },
    "500": {
        "silhouette": T("Lowboard 3.6 x 0.9 mm on paper: one carcass mass with its top band, the dark niche, three fronts and the knob.", "Lowboard 3,6 x 0,9 mm auf dem Papier: eine Korpusmasse mit Deckplattenband, das dunkle offene Fach, drei Fronten und der Knopf."),
        "retained": T("Overall size, the niche as a tone, the three fronts, the knob that sets the depth to 0.425 m.", "Gesamtgröße, das offene Fach als Ton, die drei Fronten, der Knopf, der die Tiefe auf 0,425 m bringt."),
        "merged": T("Feet, gap, back, dividers and niche into the carcass mass.", "Gleiter, Spalt, Rückwand, Mittelwände und offenes Fach in die Korpusmasse."),
        "omitted": T("Cable hole, tape, label and every trace of use.", "Kabeldurchlass, Klebeband, Preisschild und alle Gebrauchsspuren."),
        "thinElements": T("The top band and the flap edge (16 mm, 0.03 mm) read through the 0.2 mm body pen.", "Deckplattenband und Klappenkante (16 mm, 0,03 mm) wirken durch den 0,2-mm-Körperstift."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.2 mm only; carcass 0.88, niche 0.55, fronts 0.93.", "Nur Körperstift 0,2 mm; Korpus 0,88, offenes Fach 0,55, Fronten 0,93."),
        "overlays": T("The carcass mass carries carcass-foil, back-panel and feet.", "Die Korpusmasse trägt carcass-foil, back-panel und feet."),
    },
}
for _sc, _fields in _REP.items():
    for _k, _v in _fields.items():
        TXT[f"rep{_sc}-{_k}"] = _v
