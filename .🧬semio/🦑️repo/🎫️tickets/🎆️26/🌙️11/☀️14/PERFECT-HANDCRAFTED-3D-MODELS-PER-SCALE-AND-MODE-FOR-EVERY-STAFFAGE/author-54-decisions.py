import json, sys

SRC = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes/vegetation/%s.json"

T = lambda en, de: {"en": en, "de": de}

D = {}

D["agave"] = {
    "50": [
        T("Body: 20 solids. The caudex is a loft from its elevation and front outlines. The spear is a loft whose width follows the elevation outline and whose depth follows the front outline. Each of the 18 leaf segments (two per curved leaf, one for the straight leaves R5 and L5) is a thick tapering loft in its bone frame: its half width is the drawn outline smoothed over 20 mm, its thickness runs from 40 mm at the base to 22 mm towards the tip, its section is a rounded rectangle (exponent 3.4) and every section lies exactly in the leaf plane, so the leaf does not twist along its length. The channel of the upper face and the marginal teeth are below the 7.5 mm feature and are not modelled; the leaves arch out of the rosette because the bone chain bends at the knee.",
          "Körper: 20 Körper. Der Caudex ist ein Loft aus Aufriss- und Frontkontur. Das Herzblatt ist ein Loft, dessen Breite der Aufrisskontur und dessen Tiefe der Frontkontur folgt. Jedes der 18 Blattsegmente (zwei je gebogenem Blatt, eines für die geraden Blätter R5 und L5) ist ein dickes, sich verjüngendes Loft im Knochenrahmen: die halbe Breite ist die gezeichnete Kontur über 20 mm geglättet, die Dicke läuft von 40 mm an der Basis auf 22 mm zur Spitze, der Schnitt ist ein gerundetes Rechteck (Exponent 3,4) und jeder Schnitt liegt genau in der Blattebene, sodass sich das Blatt nicht verdreht. Die Rinne der Oberseite und die Randzähne liegen unter dem Merkmal von 7,5 mm und sind nicht modelliert; die Blätter wölben sich aus der Rosette, weil die Knochenkette am Knie abknickt."),
        T("Thermal: references to the leaf lofts (spreading lower leaves R1, R2, L1, L2 versus erect inner leaves and spear) and to the caudex loft; two flat transpiration discs mark the emitters. Leaf regions are exchange areas (coverage only, areaM2 untouched).",
          "Thermik: Verweise auf die Blattlofts (ausgebreitete untere Blätter R1, R2, L1, L2 gegenüber aufrechten inneren Blättern und Herzblatt) und auf das Caudex-Loft; zwei flache Transpirationsscheiben markieren die Emitter. Blattregionen sind Austauschflächen (nur Abdeckung, areaM2 unverändert)."),
        T("Structure: each leaf segment and the caudex carries a member that is a 0.7-scale core of its own loft (the stiff succulent leaf beam), joint balls sit at the leaf bases, the root plate carries a short crown bar inside the caudex base and a fixed support block lies on the ground at the anchor.",
          "Tragwerk: jedes Blattsegment und der Caudex trägt einen Stab, der ein auf 0,7 verkleinerter Kern des eigenen Lofts ist (der steife sukkulente Blattbalken), Gelenkkugeln sitzen an den Blattbasen, die Wurzelplatte trägt einen kurzen Kronenstab im Caudexfuß und ein fester Auflagerblock liegt am Anker auf dem Boden."),
    ],
    "100": [
        T("Body modelled anew against the 1:100 drawings: the same 20 solids as separate lofts but with the 1:100 outlines (smoothing over 60 mm, no teeth), rounded sections of exponent 3.4 and the thickness the drawing gives; the knee gussets of the drawn outline are kept so that the elbow of the arching leaves stays filled.",
          "Körper neu gegen die 1:100-Zeichnungen modelliert: dieselben 20 Körper als einzelne Lofts, aber mit den Umrissen von 1:100 (Glättung über 60 mm, keine Zähne), gerundeten Schnitten mit Exponent 3,4 und der Dicke der Zeichnung; die Knieverstärkungen des gezeichneten Umrisses bleiben, damit der Ellbogen der bogigen Blätter gefüllt bleibt."),
        T("Thermal: references to the coarser leaf lofts and the caudex; transpiration discs as at 1:50.",
          "Thermik: Verweise auf die gröberen Blattlofts und den Caudex; Transpirationsscheiben wie bei 1:50."),
        T("Structure: cores of the leaf lofts as members, caudex core, joint balls at the leaf bases, crown bar and support block.",
          "Tragwerk: Kerne der Blattlofts als Stäbe, Caudexkern, Gelenkkugeln an den Blattbasen, Kronenstab und Auflagerblock."),
    ],
    "200": [
        T("Body modelled anew against the 1:200 drawings: the leaves are lofts whose thickness rises to the 31 mm that this scale can show and whose sections are boxy (exponent 5) so that the thick plate reads as the drawn blade; outlines are smoothed over 100 mm; the caudex is a three-section loft.",
          "Körper neu gegen die 1:200-Zeichnungen modelliert: die Blätter sind Lofts, deren Dicke auf die 31 mm steigt, die dieser Maßstab zeigen kann, und deren Schnitte kantig sind (Exponent 5), damit die dicke Platte als gezeichnete Klinge liest; die Umrisse sind über 100 mm geglättet; der Caudex ist ein Loft mit drei Schnitten."),
        T("Thermal: references to the leaf lofts and the caudex; only the total of exposed areas counts at this scale.",
          "Thermik: Verweise auf die Blattlofts und den Caudex; bei diesem Maßstab zählt nur die Summe der freien Flächen."),
        T("Structure: member cores at 30 mm, crown bar, support block.",
          "Tragwerk: Stabkerne mit 30 mm, Kronenstab, Auflagerblock."),
    ],
    "500": [
        T("Body modelled anew against the 1:500 drawings: the 28 real leaves become 16 curved blades (three sections along an arch, 75 mm minimum, rounded-rectangular sections lying in horizontal-perpendicular planes), the spear and the caudex. Tips, arch heights, widths and thicknesses of the blades were fitted by annealing to the mean intersection over union of the three drawn views under the bounding-box limits; the drawn elevation is a filled blob that the front fan and the plan star cannot produce (the visual hull of the three drawings reaches at most 0.80 in elevation), so the fit stays near 0.70 in each view.",
          "Körper neu gegen die 1:500-Zeichnungen modelliert: die 28 wirklichen Blätter werden zu 16 gebogenen Klingen (drei Schnitte entlang eines Bogens, mindestens 75 mm, rechteckig gerundete Schnitte in horizontal-senkrechten Ebenen), dem Herzblatt und dem Caudex. Spitzen, Bogenhöhen, Breiten und Dicken der Klingen wurden durch Annealing an die mittlere Überschneidung der drei gezeichneten Ansichten unter den Hüllmaßgrenzen angepasst; der gezeichnete Aufriss ist ein gefüllter Fleck, den der Frontfächer und der Grundrissstern nicht erzeugen können (die visuelle Hülle der drei Zeichnungen erreicht im Aufriss höchstens 0,80), daher bleibt die Anpassung bei etwa 0,70 je Ansicht."),
        T("Thermal: references to the blade and spear lofts and the caudex; the two transpiration discs are flat and sit inside the rosette.",
          "Thermik: Verweise auf die Klingen- und Herzblattlofts und den Caudex; die beiden Transpirationsscheiben sind flach und liegen in der Rosette."),
        T("Structure: every leaf bone is represented by a two-section core inside the blade that carries it (leaf segments a and b share the two halves of one blade), plus the caudex core, a crown bar and the support block; all members lie inside the body silhouette.",
          "Tragwerk: jeder Blattknochen wird durch einen Kern mit zwei Schnitten in der Klinge dargestellt, die ihn trägt (die Blattsegmente a und b teilen sich die zwei Hälften einer Klinge), dazu Caudexkern, Kronenstab und Auflagerblock; alle Stäbe liegen in der Körperkontur."),
    ],
}

D["hosta-clump"] = {
    "50": [
        T("Body: 18 solids. Each of the nine blades is a thin lofted sheet along its bone: its half width follows the drawn heart-shaped outline (smoothed over 12 mm, asymmetry kept), its thickness is a ridge of 9 mm at the midrib falling to 5 mm at the tip with a flattened diamond-to-rounded-rectangle section (exponent 2.5), and all sections lie in the blade plane. The nine petioles are tapering round lofts from 32 to 24 mm diameter. The drawn midrib strokes are the ridge of the blade, not extra solids.",
          "Körper: 18 Körper. Jede der neun Spreiten ist ein dünn gelofteter Flächenkörper entlang ihres Knochens: die halbe Breite folgt dem gezeichneten herzförmigen Umriss (über 12 mm geglättet, Asymmetrie bleibt), die Dicke ist ein Grat von 9 mm an der Mittelrippe, der zur Spitze auf 5 mm abfällt, mit einem Schnitt zwischen abgeflachter Raute und gerundetem Rechteck (Exponent 2,5); alle Schnitte liegen in der Spreitenebene. Die neun Blattstiele sind sich verjüngende runde Lofts von 32 auf 24 mm Durchmesser. Die gezeichneten Mittelrippenstriche sind der Grat der Spreite, keine eigenen Körper."),
        T("Thermal: blades are references to the blade lofts (outer blades R1, R2, L1, L2; inner blades R3, R4, L3, L4, C1); the petioles region is marked hidden (largely covered), so its solids are own solids of the petiole shape, not references; two flat transpiration discs mark the emitters.",
          "Thermik: Spreiten sind Verweise auf die Spreitenlofts (äußere Spreiten R1, R2, L1, L2; innere Spreiten R3, R4, L3, L4, C1); die Region Blattstiele ist als verdeckt markiert (weitgehend überdeckt), ihre Körper sind daher eigene Körper der Stielform und keine Verweise; zwei flache Transpirationsscheiben markieren die Emitter."),
        T("Structure: a member along every petiole (the load path from the crown to the blade) and along every blade (core of the ridge), joint balls at the blade bases, a short crown bar on the root plate and the support block.",
          "Tragwerk: ein Stab entlang jedes Blattstiels (Lastpfad von der Krone zur Spreite) und jeder Spreite (Kern des Grats), Gelenkkugeln an den Spreitenbasen, ein kurzer Kronenstab auf der Wurzelplatte und der Auflagerblock."),
    ],
    "100": [
        T("Body modelled anew against the 1:100 drawings: the same nine blade lofts and nine petiole lofts, outlines smoothed over 20 mm and the ridge thickened to 16 mm so that it stays above the 15 mm feature.",
          "Körper neu gegen die 1:100-Zeichnungen modelliert: dieselben neun Spreiten- und neun Stiellofts, Umrisse über 20 mm geglättet und der Grat auf 16 mm verstärkt, damit er über dem Merkmal von 15 mm bleibt."),
        T("Thermal: as at 1:50 (blade references, own petiole solids, emitter discs).",
          "Thermik: wie bei 1:50 (Spreitenverweise, eigene Stielkörper, Emitterscheiben)."),
        T("Structure: members along petioles and blades, crown bar and support block.",
          "Tragwerk: Stäbe entlang Blattstielen und Spreiten, Kronenstab und Auflagerblock."),
    ],
    "200": [
        T("Body modelled anew against the 1:200 drawings, which show the hosta as merged masses (two outer arcs and an inner mound; the petioles are below the print minimum and are not drawn): seven ellipsoid lobes with yaw and roll fitted by annealing to the three drawn views from the visual hull of the drawings.",
          "Körper neu gegen die 1:200-Zeichnungen modelliert, die die Funkie als verschmolzene Massen zeigen (zwei äußere Bögen und ein innerer Hügel; die Blattstiele liegen unter dem Druckminimum und sind nicht gezeichnet): sieben Ellipsoidlappen mit Gier und Rollen, durch Annealing aus der visuellen Hülle der Zeichnungen an die drei gezeichneten Ansichten angepasst."),
        T("Thermal: lobes are references (outer arcs for the outer blades, the upper lobes for the inner blades); the hidden petioles are nine own lofts at the real petiole positions inside the mass; two emitter discs.",
          "Thermik: Lappen sind Verweise (äußere Bögen für die äußeren Spreiten, die oberen Lappen für die inneren Spreiten); die verdeckten Blattstiele sind neun eigene Lofts an den wirklichen Stielpositionen in der Masse; zwei Emitterscheiben."),
        T("Structure: members at 30 mm along the rig bones, crown bar and support block. The petioles stand in the hollow below the drawn inner mound, so their members lie partly outside the drawn silhouette.",
          "Tragwerk: Stäbe mit 30 mm entlang der Rigknochen, Kronenstab und Auflagerblock. Die Blattstiele stehen in der Höhlung unter dem gezeichneten inneren Hügel, ihre Stäbe liegen daher teils außerhalb der gezeichneten Kontur."),
    ],
}

D["bamboo-clump"] = {
    "50": [
        T("Body: the 18 culm segments (six culms of three segments) are hollow tubes, each a revolve whose profile is the outer wall with a 2 mm node ring every 220 mm, a 4 mm wall and the bore; outer diameters taper 29, 27, 23 and 16 mm along the culm, and every tube is placed on the leaning centre line that the elevation and the front drawing give (the culms stand at different depths and lean in two planes). They are slender (ratio 25 to 77) and keep their true size. The foliage is 14 ellipsoid lobes (centre, three radii, yaw, roll) fitted by annealing to the three drawn views from the visual hull of the drawings and bound to the nearest foliage bone, so the soft crowns follow the drawn scallops as masses and not as one ball.",
          "Körper: die 18 Halmsegmente (sechs Halme zu drei Segmenten) sind hohle Rohre, jedes ein Rotationskörper aus der Außenwand mit einem 2-mm-Knotenring alle 220 mm, 4 mm Wand und Bohrung; die Außendurchmesser verjüngen sich entlang des Halms 29, 27, 23 und 16 mm, und jedes Rohr steht auf der geneigten Mittellinie, die Aufriss und Frontansicht ergeben (die Halme stehen in verschiedenen Tiefen und neigen sich in zwei Ebenen). Sie sind schlank (Verhältnis 25 bis 77) und behalten ihre wahre Größe. Das Laub besteht aus 14 Ellipsoidlappen (Mitte, drei Radien, Gier, Rollen), durch Annealing aus der visuellen Hülle der Zeichnungen an die drei gezeichneten Ansichten angepasst und an den nächsten Laubknochen gebunden, sodass die weichen Kronen den gezeichneten Buchten als Massen folgen und nicht als eine Kugel."),
        T("Thermal: the culm region has its own solid skins (the outer surface of each tube without the bore, so the exposed area is the 1.6 m2 culm area and not the bore as well); the sun and shade foliage regions are references to the lobes (upper lobes sunlit, the rest shaded); the twelve transpiration emitters are flat discs at their anchors.",
          "Thermik: die Halmregion hat eigene Hautkörper (die Außenfläche jedes Rohrs ohne Bohrung, damit die freie Fläche die 1,6 m2 Halmfläche ist und nicht zusätzlich die Bohrung); die Laubregionen Sonne und Schatten sind Verweise auf die Lappen (obere Lappen besonnt, der Rest beschattet); die zwölf Transpirationsemitter sind flache Scheiben an ihren Ankern."),
        T("Structure: every culm segment is a member that references its hollow tube (the real load-bearing section), every foliage bone carries a short rod inside its lobes, the root plate carries a crown bar inside the base of culm 3 and one support block lies at the anchor.",
          "Tragwerk: jedes Halmsegment ist ein Stab, der auf sein hohles Rohr verweist (der wirkliche tragende Querschnitt), jeder Laubknochen trägt einen kurzen Stab in seinen Lappen, die Wurzelplatte trägt einen Kronenstab im Fuß des Halms 3 und ein Auflagerblock liegt am Anker."),
    ],
    "100": [
        T("Body modelled anew against the 1:100 drawings: the same 18 hollow culm tubes with a node ring every 300 mm (the ring is below the 15 mm feature and kept as the slender detail of the tube), and 10 foliage lobes fitted to the 1:100 foliage masses.",
          "Körper neu gegen die 1:100-Zeichnungen modelliert: dieselben 18 hohlen Halmrohre mit einem Knotenring alle 300 mm (der Ring liegt unter dem Merkmal von 15 mm und bleibt als schlankes Detail des Rohrs) und 10 Laublappen, an die Lappenmassen von 1:100 angepasst."),
        T("Thermal: as at 1:50 (own culm skins, lobe references, emitter discs).",
          "Thermik: wie bei 1:50 (eigene Halmhäute, Lappenverweise, Emitterscheiben)."),
        T("Structure: members that reference the hollow tubes, foliage rods, crown bar and support block.",
          "Tragwerk: Stäbe, die auf die hohlen Rohre verweisen, Laubstäbe, Kronenstab und Auflagerblock."),
    ],
    "200": [
        T("Body modelled anew against the 1:200 drawings: the bore of 8 mm and the node rings are not resolvable, so the 18 culm segments are solid slender cones of the true diameter (29 to 16 mm), and the foliage is 8 ellipsoid lobes fitted to the drawn masses.",
          "Körper neu gegen die 1:200-Zeichnungen modelliert: die Bohrung von 8 mm und die Knotenringe sind nicht auflösbar, daher sind die 18 Halmsegmente massive schlanke Kegel mit dem wahren Durchmesser (29 bis 16 mm), und das Laub besteht aus 8 Ellipsoidlappen, an die gezeichneten Massen angepasst."),
        T("Thermal: own culm skins, lobe references, emitter discs; only the total of exposed areas counts at this scale.",
          "Thermik: eigene Halmhäute, Lappenverweise, Emitterscheiben; bei diesem Maßstab zählt nur die Summe der freien Flächen."),
        T("Structure: members that reference the culm cones, foliage rods, crown bar and support block.",
          "Tragwerk: Stäbe, die auf die Halmkegel verweisen, Laubstäbe, Kronenstab und Auflagerblock."),
    ],
    "500": [
        T("Body modelled anew against the 1:500 drawings: seven ellipsoid lobes (centre, three radii, yaw, roll) fitted by annealing to the three drawn views. The six culms are merged as drawn into the stem band, which is five lobes on the stem bone, and each of the two drawn crowns is one lobe on its foliage bone. The true-size culms are not in the body at this scale (the drawing shows a filled band) but stay in the thermal and structure lists as slender parts.",
          "Körper neu gegen die 1:500-Zeichnungen modelliert: sieben Ellipsoidlappen (Mitte, drei Radien, Gier, Rollen), durch Annealing an die drei gezeichneten Ansichten angepasst. Die sechs Halme sind wie gezeichnet zum Halmband verschmolzen, das aus fünf Lappen am Halmknochen besteht, und jede der zwei gezeichneten Kronen ist ein Lappen an ihrem Laubknochen. Die Halme in wahrer Größe liegen bei diesem Maßstab nicht im Körper (die Zeichnung zeigt ein gefülltes Band), bleiben aber als schlanke Teile in Thermik und Tragwerk."),
        T("Thermal: the 18 true-size culm cones are own slender solids of the culm region (the exposed area is the 1.6 m2 culm area), the sun and shade foliage regions are references to the lobes, the emitters are flat discs.",
          "Thermik: die 18 Halmkegel in wahrer Größe sind eigene schlanke Körper der Halmregion (die freie Fläche ist die Halmfläche von 1,6 m2), die Laubregionen Sonne und Schatten sind Verweise auf die Lappen, die Emitter sind flache Scheiben."),
        T("Structure: 18 slender box members along the culm segments (a box is the smallest closed section that this scale can carry), foliage rods, crown bar and support block.",
          "Tragwerk: 18 schlanke Kastenstäbe entlang der Halmsegmente (ein Kasten ist der kleinste geschlossene Querschnitt, den dieser Maßstab tragen kann), Laubstäbe, Kronenstab und Auflagerblock."),
    ],
}

if __name__ == "__main__":
    for slug in sys.argv[1:]:
        p = SRC % slug
        d = json.load(open(p))
        for sc, lines in D[slug].items():
            d["volumes"][sc]["decisions"] = lines
        json.dump(d, open(p, "w"), ensure_ascii=False)
        print("decisions written", slug, list(D[slug]))
