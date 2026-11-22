"""📚️ PW-12 texts: references, assumptions and the per-scale decisions of the three flowering shrubs, in English and German.

@see ./lilac.py
@see ./hydrangea.py
@see ./rhododendron.py"""
from kit import T

common = [
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapters 4 and 5 (density, moisture and mechanical properties of green wood); used as a guide, species values are authored estimates."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile."},
    {"id": "vogel-1989", "citation": "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
]
hillier = {"id": "hillier-2014", "citation": "Hillier Nurseries (2014). The Hillier Manual of Trees and Shrubs, 8th ed., Royal Horticultural Society. Habit, size and flowering of garden shrubs; used for proportions, not for measured values."}
koerner = {"id": "koerner-1979", "citation": "Körner, C., Scheel, J. A. and Bauer, H. (1979). Maximum leaf diffusive conductance in vascular plants. Photosynthetica 13(1), 45-82. Range of maximum leaf conductances per life form; the species values here are authored estimates within it."}
keim = {"id": "keim-2006", "citation": "Keim, R. F., Skaugset, A. E. and Weiler, M. (2006). Storage of water on vegetation under simulated rainfall of varying intensity. Advances in Water Resources 29(7), 974-986. Order of magnitude of water stored on foliage."}
larcher = {"id": "larcher-2003", "citation": "Larcher, W. (2003). Physiological Plant Ecology, 4th ed., Springer. Frost responses of evergreen leaves (thermonastic leaf rolling and drooping) and stomatal closure in the cold."}


def rhs(id, name, what):
    return {"id": id, "citation": "Royal Horticultural Society, plant entry %s, RHS Find a Plant (rhs.org.uk): %s. Used for the size band; the drawn plant is one authored specimen inside it." % (name, what)}


def thermal_assumptions(plant, flowers):
    return [
        ("Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance is an authored estimate for the life form, inside the ranges of Körner et al. (1979); no published species maximum is claimed. Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.",
         "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für die Lebensform innerhalb der Bereiche von Körner et al. (1979); es wird kein veröffentlichtes Artmaximum behauptet. Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
        ("Bark and stems are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of about 1.5 cm, an authored estimate guided by the Wood Handbook. No surface temperature is authored.",
         "Rinde und Stämme sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von etwa 1,5 cm, eine Autorenschätzung angelehnt an das Wood Handbook. Es werden keine Oberflächentemperaturen vorgegeben."),
        ("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; the direction is the vertical in the frame of the first anchor's bone, since the release is diffuse. Sun and shade foliage are separate regions: the sun leaves form the outer masses, the shade leaves the inner core, and each pose authors the sunlit share of each region.",
         "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker über die Krone; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist. Sonnen- und Schattenlaub sind getrennte Regionen: Die Sonnenblätter bilden die äußeren Massen, die Schattenblätter den inneren Kern, und jede Pose gibt den besonnten Anteil jeder Region vor."),
        flowers,
        ("Flowering, leaf fall and snow are not geometric: a pose cannot hide shapes, so the drawn flowers belong to the plant in every pose and the seasonal poses change the thermal binding (stomatal closure, wetness, sun) and the loads only.",
         "Blüte, Laubfall und Schnee sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, daher gehören die gezeichneten Blüten in jeder Pose zur Pflanze, und die jahreszeitlichen Posen ändern nur die thermische Bindung (Stomataschluss, Nässe, Sonne) und die Lasten."),
    ]


def structure_assumptions(en, de):
    return [
        ("The shrub is one rigid body per evaluated pose, fixed at the root crown; wind drag acts at the centre of each foliage mass and on the lower stems as line loads, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates scaled with the drag force, not a computed deflection.",
         "Der Strauch ist je ausgewerteter Pose ein starrer Körper, am Wurzelstock eingespannt; Winddruck wirkt im Schwerpunkt jeder Laubmasse und als Linienlast an den unteren Stammteilen, Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zur Widerstandskraft, keine berechnete Durchbiegung."),
        ("Section properties of the %s are solid circles of green wood at the base of each segment (authored-estimate properties guided by the Wood Handbook); side shoots and twigs are carried inside the foliage masses." % en,
         "Querschnittswerte der %s sind Vollkreise aus frischem Holz am Fuß jedes Abschnitts (Autorenschätzungen angelehnt an das Wood Handbook); Seitentriebe und Zweige stecken in den Laubmassen." % de),
    ]


def decisions_lilac():
    return {
        "50": {
            "silhouette": ("The lilac is drawn as it stands, 4.0 m high, 3.6 m ahead and 3.0 m across: seven stems fanning out of a thicket of root suckers into a vase, twenty-four rounded foliage masses and thirty-three upright panicles; 1 m is 20 mm, so every stem and every flower cone is a shape of its own.",
                           "Der Flieder ist so gezeichnet, wie er steht, 4,0 m hoch, 3,6 m in Blickrichtung und 3,0 m quer: sieben Stämme fächern aus einem Dickicht von Wurzelausläufern zu einer Vase auf, vierundzwanzig gerundete Laubmassen und dreiunddreißig aufrechte Rispen; 1 m sind 20 mm, daher ist jeder Stamm und jeder Blütenkegel eine eigene Form."),
            "retained": ("Retained: all seven stems in two tapering segments at their true diameter (10.5 to 3 cm), the shoot masses with their small top lumps, the two shaded cores and the leaf texture of the outer masses.",
                         "Beibehalten: alle sieben Stämme in zwei sich verjüngenden Abschnitten in wahrem Durchmesser (10,5 bis 3 cm), die Triebmassen mit ihren kleinen Kuppen, die beiden beschatteten Kerne und die Blattstruktur der äußeren Massen."),
            "merged": ("Merged: the side shoots, twigs and the countless single leaves into the twenty-three foliage masses; the florets of a panicle into one cone.",
                       "Zusammengefasst: Seitentriebe, Zweige und die unzähligen Einzelblätter zu dreiundzwanzig Laubmassen; die Einzelblüten einer Rispe zu einem Kegel."),
            "omitted": ("Omitted: the single sucker shoots inside their thicket, the bark lenticels and the seed capsules of last year.",
                        "Weggelassen: die einzelnen Ausläufertriebe in ihrem Dickicht, die Lentizellen der Rinde und die Samenkapseln des Vorjahres."),
            "thinElements": ("Nothing is thickened: the thinnest stem top (3 cm) is 0.6 mm and the panicle tips 0.5 mm on paper.",
                             "Nichts wird verstärkt: die dünnste Stammspitze (3 cm) misst 0,6 mm und die Rispenspitzen 0,5 mm auf dem Papier."),
            "hatching": ("The two shaded cores carry a 45-degree shade hatch at 0.8 mm; short leaf strokes (texture pen 0.13 mm) mark the heart-shaped leaves on the outer masses.",
                         "Die beiden beschatteten Kerne tragen eine Schattenschraffur unter 45 Grad mit 0,8 mm Abstand; kurze Blattstriche (Texturstift 0,13 mm) zeichnen die herzförmigen Blätter auf den äußeren Massen."),
            "styles": ("Light line: elliptic outlines over each other with the leaf strokes. Dark solid: the crown dark, the cores darker, the panicles as pale cones standing out of it, the stems mid-grey.",
                       "Helle Linie: gerundete Umrisse übereinander mit den Blattstrichen. Dunkel flächig: die Krone dunkel, die Kerne dunkler, die Rispen als helle Kegel davor, die Stämme mittelgrau."),
            "overlays": ("Thermal and structural overlays map onto stem-bark (stems and root crown), leaf-sun (outer masses), leaf-shade (two cores) and flowers (every panicle); the member views colour the fourteen stem segments.",
                         "Thermische und statische Überlagerungen bilden ab auf stem-bark (Stämme und Wurzelstock), leaf-sun (äußere Massen), leaf-shade (zwei Kerne) und flowers (jede Rispe); die Stabansichten färben die vierzehn Stammabschnitte."),
        },
        "100": {
            "silhouette": ("Seven stems, the sucker thicket and fifteen rounded masses, 40 mm high on paper: the vase of bare stems below and the lumpy crown with eighteen panicles above read at a glance.",
                           "Sieben Stämme, das Ausläuferdickicht und fünfzehn gerundete Massen, 40 mm hoch auf dem Papier: unten die Vase der kahlen Stämme, oben die bucklige Krone mit achtzehn Rispen, auf einen Blick lesbar."),
            "retained": ("Retained: every stem in two segments at true diameter, one top and one lower mass per outer stem, the crown top, both shaded cores, two panicles per shoot mass and three on the top.",
                         "Beibehalten: jeder Stamm in zwei Abschnitten in wahrem Durchmesser, je eine obere und eine untere Masse pro äußerem Stamm, die Kronenspitze, beide beschatteten Kerne, zwei Rispen je Triebmasse und drei auf der Spitze."),
            "merged": ("Merged: the small top lumps of 1:50 into slightly larger shoot masses; the panicle of the lower masses into the shoot masses above.",
                       "Zusammengefasst: die kleinen Kuppen von 1:50 in etwas größere Triebmassen; die Rispen der unteren Massen in die Triebmassen darüber."),
            "omitted": ("Omitted: the leaf texture of the lower masses and the third panicle of each shoot mass.",
                        "Weggelassen: die Blattstruktur der unteren Massen und die dritte Rispe jeder Triebmasse."),
            "thinElements": ("No thickening: the 3 cm stem tops print at 0.3 mm, the panicle tips at 0.28 mm.",
                             "Keine Verstärkung: die 3 cm dünnen Stammspitzen drucken mit 0,3 mm, die Rispenspitzen mit 0,28 mm."),
            "hatching": ("Shade hatch at 0.6 mm in both cores; three leaf strokes per outer top mass with the 0.1 mm texture pen.",
                         "Schattenschraffur mit 0,6 mm in beiden Kernen; drei Blattstriche je oberer Außenmasse mit dem Texturstift 0,1 mm."),
            "styles": ("Light line: outlines and few strokes. Dark solid: crown dark, cores darker, panicles pale, stems mid-grey.",
                       "Helle Linie: Umrisse und wenige Striche. Dunkel flächig: Krone dunkel, Kerne dunkler, Rispen hell, Stämme mittelgrau."),
            "overlays": ("Overlays map onto stem-bark, leaf-sun, leaf-shade and flowers exactly as at 1:50; the fourteen stem segments carry the member colours.",
                         "Überlagerungen bilden wie bei 1:50 auf stem-bark, leaf-sun, leaf-shade und flowers ab; die vierzehn Stammabschnitte tragen die Stabfarben."),
        },
        "200": {
            "silhouette": ("At 20 mm height the lilac is a vase of seven lower stems carrying fourteen smooth lobes, an upper and a lower one per outer stem, with eight panicle cones on the rim.",
                           "Bei 20 mm Höhe ist der Flieder eine Vase aus sieben unteren Stammteilen mit vierzehn glatten Laubbuckeln, einem oberen und einem unteren je äußerem Stamm, und acht Rispenkegeln am Rand."),
            "retained": ("Retained: the seven lower stems at true diameter (0.35 to 0.5 mm), the two-tier lobe rhythm of the stems, one panicle per shoot mass and two on the top.",
                         "Beibehalten: die sieben unteren Stammteile in wahrem Durchmesser (0,35 bis 0,5 mm), der zweistufige Buckelrhythmus der Stämme, eine Rispe je Triebmasse und zwei auf der Spitze."),
            "merged": ("Merged: the top lumps into the shoot masses, the masses into smooth lobes; both shaded cores into one core; the upper stems into the crown (they stay in the thermal model).",
                       "Zusammengefasst: die Kuppen in die Triebmassen, die Massen zu glatten Buckeln; beide beschatteten Kerne zu einem Kern; die oberen Stammteile in die Krone (im thermischen Modell bleiben sie erhalten)."),
            "omitted": ("Omitted: leaf texture and hatching, the upper stem segments inside the crown.",
                        "Weggelassen: Blattstruktur und Schraffur, die oberen Stammteile innerhalb der Krone."),
            "thinElements": ("No thickening; the upper stems (3 cm, 0.15 mm) are left inside the crown instead of being widened.",
                             "Keine Verstärkung; die oberen Stammteile (3 cm, 0,15 mm) bleiben in der Krone, statt verbreitert zu werden."),
            "hatching": ("No hatching; the core is told apart by its tone in the solid styles only.",
                         "Keine Schraffur; der Kern unterscheidet sich nur in den Flächenstilen durch seinen Ton."),
            "styles": ("Light line: fourteen lobe outlines, the top and the stem fan. Dark solid: one dark crown with pale cones.",
                       "Helle Linie: vierzehn Buckelumrisse, die Kuppe und der Stammfächer. Dunkel flächig: eine dunkle Krone mit hellen Kegeln."),
            "overlays": ("Overlays map onto stem-bark (seven lower stems and root crown), leaf-sun (fourteen lobes and the top), leaf-shade (one core) and flowers (eight cones).",
                         "Überlagerungen bilden ab auf stem-bark (sieben untere Stammteile und Wurzelstock), leaf-sun (vierzehn Buckel und die Kuppe), leaf-shade (ein Kern) und flowers (acht Kegel)."),
        },
        "500": {
            "silhouette": ("At 8 mm the lilac is composed as three crown lobes over the fan of its stem clump and the low sucker thicket at its foot; the outline keeps the true 4.0 m height and 3.6 m width.",
                           "Bei 8 mm ist der Flieder als drei Kronenbuckel über dem Fächer seines Stammhorsts und dem niedrigen Ausläuferdickicht am Fuß komponiert; der Umriss hält die wahre Höhe von 4,0 m und die Breite von 3,6 m."),
            "retained": ("Retained: the bare-legged vase as one fan from 0.5 m at the ground to 1.2 m under the crown, and the crown as front, back and top lobes.",
                         "Beibehalten: die kahle Stammvase als ein Fächer von 0,5 m am Boden bis 1,2 m unter der Krone, die Krone als vorderer, hinterer und oberer Buckel."),
            "merged": ("Merged: the seven stems into the fan (single stems would be 0.15 to 0.2 mm and their gaps narrower than the printable 0.15 mm), all foliage masses, the core and the panicles into three lobes listing leaf-sun, leaf-shade and flowers.",
                       "Zusammengefasst: die sieben Stämme zum Fächer (einzelne Stämme wären 0,15 bis 0,2 mm, ihre Zwischenräume schmaler als die druckbaren 0,15 mm), alle Laubmassen, der Kern und die Rispen zu drei Buckeln mit leaf-sun, leaf-shade und flowers."),
            "omitted": ("Omitted: single panicles (0.4 mm cones would only blur the rim), texture and hatching.",
                        "Weggelassen: einzelne Rispen (Kegel von 0,4 mm würden den Rand nur verwischen), Struktur und Schraffur."),
            "thinElements": ("No element is thickened; the thin stems are merged into the fan at their true outer spread.",
                             "Kein Element wird verstärkt; die dünnen Stämme gehen in ihrer wahren äußeren Spreizung im Fächer auf."),
            "hatching": ("No hatching; pens 0.15 mm for outlines and 0.25 mm for the ground line.",
                         "Keine Schraffur; Stifte 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
            "styles": ("Light line: three closed outlines and the fan. Dark solid: one dark crown on a lighter stem fan.",
                       "Helle Linie: drei geschlossene Umrisse und der Fächer. Dunkel flächig: eine dunkle Krone auf hellerem Stammfächer."),
            "overlays": ("The crown lobes are merged paths of leaf-sun, leaf-shade and flowers and show their area-weighted value, labelled as merged; the fan maps onto stem-bark.",
                         "Die Kronenbuckel sind zusammengefasste Pfade aus leaf-sun, leaf-shade und flowers und zeigen deren flächengewichteten Wert, als zusammengefasst beschriftet; der Fächer bildet auf stem-bark ab."),
        },
    }


def volume_decisions_lilac():
    return {
        "50": [("Body at 1:50: the root crown as a frustum, fourteen tapered stem capsules at true diameter along their 3D axes, twenty-four foliage ellipsoids (the sucker thicket at the foot among them) that each follow the drawn mass of all three views and thirty-three panicle cones.",
                "Körper bei 1:50: der Wurzelstock als Kegelstumpf, vierzehn sich verjüngende Stammkapseln in wahrem Durchmesser entlang ihrer 3D-Achsen, vierundzwanzig Laubellipsoide (darunter das Ausläuferdickicht am Fuß), die der gezeichneten Masse in allen drei Ansichten folgen, und dreiunddreißig Rispenkegel."),
               ("Thermal at 1:50: every body solid by reference with its region; two small outlets inside the crown top and the lower core carry the transpiration emitters.",
                "Thermik bei 1:50: jeder Körper als Verweis mit seiner Region; zwei kleine Auslässe in der Kronenspitze und im unteren Kern tragen die Transpirationsemitter."),
               ("Structure at 1:50: the fourteen stem segments as four-station lofts of green wood, the root crown as a frustum and a fixed disc below the ground at the anchor.",
                "Tragwerk bei 1:50: die vierzehn Stammabschnitte als Lofts aus frischem Holz mit vier Stationen, der Wurzelstock als Kegelstumpf und eine eingespannte Scheibe unter dem Boden am Anker.")],
        "100": [("Body at 1:100: stems as at 1:50, sixteen foliage ellipsoids (the top lumps grown into their shoot masses) and eighteen panicles.",
                 "Körper bei 1:100: Stämme wie bei 1:50, sechzehn Laubellipsoide (die Kuppen in ihre Triebmassen aufgegangen) und achtzehn Rispen."),
                ("Thermal at 1:100: references and two outlets.", "Thermik bei 1:100: Verweise und zwei Auslässe."),
                ("Structure at 1:100: three-station lofts per segment and the root frustum.", "Tragwerk bei 1:100: Lofts mit drei Stationen je Abschnitt und der Wurzelkegelstumpf.")],
        "200": [("Body at 1:200: the seven lower stems, seventeen lobes with the sucker thicket and eight panicle cones; the upper stems lie inside the crown and are left to the thermal model.",
                 "Körper bei 1:200: die sieben unteren Stammteile, siebzehn Buckel mit dem Ausläuferdickicht und acht Rispenkegel; die oberen Stammteile liegen in der Krone und bleiben dem thermischen Modell überlassen."),
                ("Thermal at 1:200: references, the seven upper stems as slender bark solids of true section and two outlets.",
                 "Thermik bei 1:200: Verweise, die sieben oberen Stammteile als schlanke Rindenkörper in wahrem Querschnitt und zwei Auslässe."),
                ("Structure at 1:200: one tapered two-station loft per segment and the root frustum.", "Tragwerk bei 1:200: ein sich verjüngender Loft mit zwei Stationen je Abschnitt und der Wurzelkegelstumpf.")],
        "500": [("Body at 1:500: the stem clump as one fan frustum 1.45 m high and the crown as three ellipsoids and the sucker thicket as a fourth, as the drawing composes them.",
                 "Körper bei 1:500: der Stammhorst als ein Fächerkegelstumpf von 1,45 m Höhe und die Krone als drei Ellipsoide und das Ausläuferdickicht als viertes, wie die Zeichnung sie komponiert."),
                ("Thermal at 1:500: the fan and the lobes by reference; the shaded core and four panicles are thermal solids of their own inside the crown; two outlets.",
                 "Thermik bei 1:500: Fächer und Buckel als Verweis; der beschattete Kern und vier Rispen sind eigene Thermikkörper in der Krone; zwei Auslässe."),
                ("Minimal: the structure at 1:500 keeps every stem segment as a two-station loft of its declared section and the root frustum, as at 1:200 with the section of the drawing's member table; it cannot drop a member without losing a load path.",
                 "Minimal: Das Tragwerk bei 1:500 behält jeden Stammabschnitt als Loft mit zwei Stationen in seinem angegebenen Querschnitt und den Wurzelkegelstumpf, wie bei 1:200 mit dem Querschnitt der Stabtabelle; es kann keinen Stab weglassen, ohne einen Lastpfad zu verlieren.")],
    }


def decisions_hydrangea():
    return {
        "50": {
            "silhouette": ("The hydrangea is drawn as it stands, a mound 1.55 m high with its heads, 1.9 m ahead and 1.85 m across: ten canes out of a low woody crown, twenty rounded leaf masses to the ground and twenty mophead balls; 1 m is 20 mm.",
                           "Die Hortensie ist so gezeichnet, wie sie steht, ein Hügel von 1,55 m Höhe mit den Blütenbällen, 1,9 m in Blickrichtung und 1,85 m quer: zehn Ruten aus einem niedrigen verholzten Wurzelstock, zwanzig gerundete Blattmassen bis zum Boden und zwanzig Blütenbälle; 1 m sind 20 mm."),
            "retained": ("Retained: every cane in two tapering segments at true diameter (2.6 to 1.1 cm), the upper and lower leaf mass of each cane, the shaded core, the mopheads with their floret scallops and the leaf strokes.",
                         "Beibehalten: jede Rute in zwei sich verjüngenden Abschnitten in wahrem Durchmesser (2,6 bis 1,1 cm), obere und untere Blattmasse jeder Rute, der beschattete Kern, die Blütenbälle mit ihren Blütchenwellen und die Blattstriche."),
            "merged": ("Merged: the leaves into twenty masses, the florets of a head into one rounded ball.", "Zusammengefasst: die Blätter zu zwanzig Massen, die Blütchen eines Blütenstands zu einem gerundeten Ball."),
            "omitted": ("Omitted: the leaf teeth, the flower buds for next year and the dead heads of last summer.", "Weggelassen: die Blattzähne, die Blütenknospen für das nächste Jahr und die vertrockneten Blütenstände des Vorjahres."),
            "thinElements": ("Nothing is thickened: the thinnest cane tip (1.1 cm) is 0.22 mm on paper.", "Nichts wird verstärkt: die dünnste Rutenspitze (1,1 cm) misst 0,22 mm auf dem Papier."),
            "hatching": ("The shaded core carries the 45-degree shade hatch at 0.8 mm; short leaf strokes (0.13 mm) mark the broad leaves.", "Der beschattete Kern trägt die Schattenschraffur unter 45 Grad mit 0,8 mm; kurze Blattstriche (0,13 mm) zeichnen die breiten Blätter."),
            "styles": ("Light line: rounded masses and balls. Dark solid: a dark mound with pale balls on top and the core darker inside.", "Helle Linie: gerundete Massen und Bälle. Dunkel flächig: ein dunkler Hügel mit hellen Bällen obenauf, innen der dunklere Kern."),
            "overlays": ("Overlays map onto stem-bark (canes and crown), leaf-sun (outer masses), leaf-shade (core) and flowers (every ball); the member views colour the twenty cane segments.",
                         "Überlagerungen bilden ab auf stem-bark (Ruten und Wurzelstock), leaf-sun (äußere Massen), leaf-shade (Kern) und flowers (jeder Ball); die Stabansichten färben die zwanzig Rutenabschnitte."),
        },
        "100": {
            "silhouette": ("A mound 15.5 mm high: twenty leaf masses redrawn as broader ellipses and eleven mopheads, the canes visible under the skirt.", "Ein Hügel von 15,5 mm Höhe: zwanzig Blattmassen als breitere Ellipsen neu gezeichnet und elf Blütenbälle, unter dem Saum die Ruten sichtbar."),
            "retained": ("Retained: all canes at true diameter, both leaf masses of every cane, the core, one ball per cane and two on the top.", "Beibehalten: alle Ruten in wahrem Durchmesser, beide Blattmassen jeder Rute, der Kern, ein Ball je Rute und zwei auf der Kuppe."),
            "merged": ("Merged: the second ball of each cane into its neighbour.", "Zusammengefasst: der zweite Ball jeder Rute in seinen Nachbarn."),
            "omitted": ("Omitted: leaf strokes of the lower masses.", "Weggelassen: Blattstriche der unteren Massen."),
            "thinElements": ("No thickening: the 1.1 cm cane tips print at 0.11 mm inside a 0.2 mm outline.", "Keine Verstärkung: die 1,1 cm dünnen Rutenspitzen drucken mit 0,11 mm innerhalb eines Umrisses von 0,2 mm."),
            "hatching": ("Shade hatch at 0.6 mm in the core; three leaf strokes per upper mass.", "Schattenschraffur mit 0,6 mm im Kern; drei Blattstriche je oberer Masse."),
            "styles": ("Light line: outlines and few strokes. Dark solid: dark mound, pale balls.", "Helle Linie: Umrisse und wenige Striche. Dunkel flächig: dunkler Hügel, helle Bälle."),
            "overlays": ("As at 1:50: stem-bark, leaf-sun, leaf-shade and flowers.", "Wie bei 1:50: stem-bark, leaf-sun, leaf-shade und flowers."),
        },
        "200": {
            "silhouette": ("At 7.8 mm height the hydrangea is a mound of twenty smooth cane lobes around the top and core, six balls on the rim and the lower canes under the skirt.", "Bei 7,8 mm Höhe ist die Hortensie ein Hügel aus zwanzig glatten Rutenbuckeln um Kuppe und Kern, sechs Bällen am Rand und den unteren Ruten unter dem Saum."),
            "retained": ("Retained: the lower canes at true diameter, a smooth upper and lower lobe per cane, the top, the core and six balls of 22 cm (1.1 mm).", "Beibehalten: die unteren Ruten in wahrem Durchmesser, ein glatter oberer und unterer Buckel je Rute, die Kuppe, der Kern und sechs Bälle von 22 cm (1,1 mm)."),
            "merged": ("Merged: the leaf masses into smooth lobes; the upper canes into the mound (they stay in the thermal model).", "Zusammengefasst: die Blattmassen zu glatten Buckeln; die oberen Ruten in den Hügel (im thermischen Modell bleiben sie erhalten)."),
            "omitted": ("Omitted: every second ball, texture and hatching.", "Weggelassen: jeder zweite Ball, Struktur und Schraffur."),
            "thinElements": ("No thickening; the canes are slender lines at their true diameter.", "Keine Verstärkung; die Ruten sind schlanke Linien in wahrem Durchmesser."),
            "hatching": ("No hatching.", "Keine Schraffur."),
            "styles": ("Light line: lobe outlines. Dark solid: one dark mound with pale balls.", "Helle Linie: Buckelumrisse. Dunkel flächig: ein dunkler Hügel mit hellen Bällen."),
            "overlays": ("Overlays map onto stem-bark, leaf-sun (twenty lobes and top), leaf-shade (core) and flowers (six balls).", "Überlagerungen bilden ab auf stem-bark, leaf-sun (zwanzig Buckel und Kuppe), leaf-shade (Kern) und flowers (sechs Bälle)."),
        },
        "500": {
            "silhouette": ("At 3 mm the hydrangea is composed as one low mound with a flatter top over its woody crown; the outline keeps the true 1.55 m height and 1.9 m width.", "Bei 3 mm ist die Hortensie als ein niedriger Hügel mit flacherer Kuppe über ihrem Wurzelstock komponiert; der Umriss hält die wahre Höhe von 1,55 m und die Breite von 1,9 m."),
            "retained": ("Retained: the mound, its top and the woody crown at the foot.", "Beibehalten: der Hügel, seine Kuppe und der Wurzelstock am Fuß."),
            "merged": ("Merged: all leaf masses, the core and the balls into two closed paths listing leaf-sun, leaf-shade and flowers.", "Zusammengefasst: alle Blattmassen, der Kern und die Bälle zu zwei geschlossenen Pfaden mit leaf-sun, leaf-shade und flowers."),
            "omitted": ("Omitted: the canes (0.05 mm) and the single balls (0.4 mm dots on the rim).", "Weggelassen: die Ruten (0,05 mm) und die einzelnen Bälle (Punkte von 0,4 mm am Rand)."),
            "thinElements": ("Nothing is thickened.", "Nichts wird verstärkt."),
            "hatching": ("No hatching; 0.15 mm outline, 0.25 mm ground line.", "Keine Schraffur; Umriss 0,15 mm, Bodenlinie 0,25 mm."),
            "styles": ("Light line: one mound outline. Dark solid: a dark mound.", "Helle Linie: ein Hügelumriss. Dunkel flächig: ein dunkler Hügel."),
            "overlays": ("The mound paths are merged leaf-sun, leaf-shade and flowers, shown with their area-weighted value and labelled as merged; the crown maps onto stem-bark.", "Die Hügelpfade fassen leaf-sun, leaf-shade und flowers zusammen, zeigen deren flächengewichteten Wert und sind als zusammengefasst beschriftet; der Wurzelstock bildet auf stem-bark ab."),
        },
    }


def volume_decisions_hydrangea():
    return {
        "50": [("Body at 1:50: the woody crown as a frustum, twenty cane capsules at true diameter, twenty leaf ellipsoids and twenty mophead balls, each following its drawing in all three views.", "Körper bei 1:50: der Wurzelstock als Kegelstumpf, zwanzig Rutenkapseln in wahrem Durchmesser, zwanzig Blattellipsoide und zwanzig Blütenbälle, jeder seiner Zeichnung in allen drei Ansichten folgend."),
               ("Thermal at 1:50: references and two outlets in the top and the core.", "Thermik bei 1:50: Verweise und zwei Auslässe in Kuppe und Kern."),
               ("Structure at 1:50: four-station lofts per cane segment, the crown frustum and a fixed disc below the ground.", "Tragwerk bei 1:50: Lofts mit vier Stationen je Rutenabschnitt, der Kegelstumpf des Wurzelstocks und eine eingespannte Scheibe unter dem Boden.")],
        "100": [("Body at 1:100: canes as at 1:50, twenty leaf ellipsoids redrawn and eleven balls.", "Körper bei 1:100: Ruten wie bei 1:50, zwanzig neu gezeichnete Blattellipsoide und elf Bälle."),
                ("Thermal at 1:100: references and two outlets.", "Thermik bei 1:100: Verweise und zwei Auslässe."),
                ("Structure at 1:100: three-station lofts per segment.", "Tragwerk bei 1:100: Lofts mit drei Stationen je Abschnitt.")],
        "200": [("Body at 1:200: the lower canes, twenty-two lobes and six balls.", "Körper bei 1:200: die unteren Ruten, zweiundzwanzig Buckel und sechs Bälle."),
                ("Thermal at 1:200: references, the upper canes as slender bark solids of true section, two outlets.", "Thermik bei 1:200: Verweise, die oberen Ruten als schlanke Rindenkörper in wahrem Querschnitt, zwei Auslässe."),
                ("Structure at 1:200: one tapered two-station loft per segment.", "Tragwerk bei 1:200: ein sich verjüngender Loft mit zwei Stationen je Abschnitt.")],
        "500": [("Body at 1:500: the woody crown and the mound as two ellipsoids, as drawn.", "Körper bei 1:500: der Wurzelstock und der Hügel als zwei Ellipsoide, wie gezeichnet."),
                ("Thermal at 1:500: the crown and mound by reference; all canes as slender bark solids, the core and four balls as thermal solids of their own; two outlets.", "Thermik bei 1:500: Wurzelstock und Hügel als Verweis; alle Ruten als schlanke Rindenkörper, der Kern und vier Bälle als eigene Thermikkörper; zwei Auslässe."),
                ("Minimal: the structure at 1:500 keeps every cane segment as a two-station loft of its declared section and the crown frustum; it cannot drop a member without losing a load path.", "Minimal: Das Tragwerk bei 1:500 behält jeden Rutenabschnitt als Loft mit zwei Stationen in seinem angegebenen Querschnitt und den Kegelstumpf des Wurzelstocks; es kann keinen Stab weglassen, ohne einen Lastpfad zu verlieren.")],
    }


def decisions_rhododendron():
    return {
        "50": {
            "silhouette": ("The rhododendron is drawn as it stands, a dome 3.0 m high, 3.5 m ahead and 3.4 m across on nine crooked stems: thirty-four rounded leaf masses from the skirt at 25 cm to the crown, the shaded core and fifty-two flower trusses; 1 m is 20 mm.",
                           "Der Rhododendron ist so gezeichnet, wie er steht, eine Kuppel von 3,0 m Höhe, 3,5 m in Blickrichtung und 3,4 m quer auf neun knorrigen Stämmen: vierunddreißig gerundete Blattmassen vom Saum in 25 cm Höhe bis zur Kuppe, der beschattete Kern und zweiundfünfzig Blütenstände; 1 m sind 20 mm."),
            "retained": ("Retained: every stem in three crooked segments at true diameter (8.5 to 2.6 cm), skirt, middle and top masses of every stem with the small top lumps, the core, the trusses with their flower scallops and the leaf strokes.",
                         "Beibehalten: jeder Stamm in drei knorrigen Abschnitten in wahrem Durchmesser (8,5 bis 2,6 cm), Saum-, Mittel- und Spitzenmasse jedes Stamms mit den kleinen Kuppen, der Kern, die Blütenstände mit ihren Blütenwellen und die Blattstriche."),
            "merged": ("Merged: the whorled leathery leaves into thirty-four masses, the ten to fifteen funnels of a truss into one domed head.", "Zusammengefasst: die quirligen ledrigen Blätter zu vierunddreißig Massen, die zehn bis fünfzehn Trichter eines Stands zu einem gewölbten Kopf."),
            "omitted": ("Omitted: the leaf buds, last year's seed capsules and the branch layering on the ground.", "Weggelassen: die Blattknospen, die Samenkapseln des Vorjahres und die Absenker am Boden."),
            "thinElements": ("Nothing is thickened: the thinnest stem top (2.6 cm) is 0.5 mm on paper.", "Nichts wird verstärkt: die dünnste Stammspitze (2,6 cm) misst 0,5 mm auf dem Papier."),
            "hatching": ("The core carries the 45-degree shade hatch at 0.8 mm; long narrow leaf strokes (0.13 mm) mark the elliptic leaves.", "Der Kern trägt die Schattenschraffur unter 45 Grad mit 0,8 mm; lange schmale Blattstriche (0,13 mm) zeichnen die elliptischen Blätter."),
            "styles": ("Light line: dense rounded masses and trusses. Dark solid: a very dark dome (evergreen leaves) with pale trusses and the core darkest.", "Helle Linie: dichte gerundete Massen und Blütenstände. Dunkel flächig: eine sehr dunkle Kuppel (immergrünes Laub) mit hellen Blütenständen, der Kern am dunkelsten."),
            "overlays": ("Overlays map onto stem-bark (stems and root crown), leaf-sun (outer masses), leaf-shade (core) and flowers (every truss); the member views colour the twenty-seven stem segments.", "Überlagerungen bilden ab auf stem-bark (Stämme und Wurzelstock), leaf-sun (äußere Massen), leaf-shade (Kern) und flowers (jeder Stand); die Stabansichten färben die siebenundzwanzig Stammabschnitte."),
        },
        "100": {
            "silhouette": ("A dome 30 mm high: twenty-six leaf masses redrawn as broader ellipses, the core and twenty-seven trusses, the stem feet visible under the skirt.", "Eine Kuppel von 30 mm Höhe: sechsundzwanzig Blattmassen als breitere Ellipsen neu gezeichnet, der Kern und siebenundzwanzig Blütenstände, unter dem Saum die Stammfüße sichtbar."),
            "retained": ("Retained: all stem segments at true diameter, skirt, middle and top masses, two trusses per shoot mass, one per middle mass and three on the top.", "Beibehalten: alle Stammabschnitte in wahrem Durchmesser, Saum-, Mittel- und Spitzenmassen, zwei Stände je Triebmasse, einer je Mittelmasse und drei auf der Kuppe."),
            "merged": ("Merged: the small top lumps into slightly larger shoot masses.", "Zusammengefasst: die kleinen Kuppen in etwas größere Triebmassen."),
            "omitted": ("Omitted: the leaf strokes of the skirt and middle masses and every second truss.", "Weggelassen: die Blattstriche der Saum- und Mittelmassen und jeder zweite Blütenstand."),
            "thinElements": ("No thickening: the 2.6 cm stem tops print at 0.26 mm.", "Keine Verstärkung: die 2,6 cm dünnen Stammspitzen drucken mit 0,26 mm."),
            "hatching": ("Shade hatch at 0.6 mm in the core; three leaf strokes per top mass.", "Schattenschraffur mit 0,6 mm im Kern; drei Blattstriche je Spitzenmasse."),
            "styles": ("Light line: outlines and few strokes. Dark solid: dark dome, pale trusses.", "Helle Linie: Umrisse und wenige Striche. Dunkel flächig: dunkle Kuppel, helle Blütenstände."),
            "overlays": ("As at 1:50: stem-bark, leaf-sun, leaf-shade and flowers.", "Wie bei 1:50: stem-bark, leaf-sun, leaf-shade und flowers."),
        },
        "200": {
            "silhouette": ("At 15 mm height the rhododendron is a dome of eight upper and eight lower lobes around the top and the core, eleven trusses on the rim and the stem feet under the skirt.", "Bei 15 mm Höhe ist der Rhododendron eine Kuppel aus acht oberen und acht unteren Buckeln um Kuppe und Kern, elf Blütenständen am Rand und den Stammfüßen unter dem Saum."),
            "retained": ("Retained: the lowest stem segments at true diameter, two lobes per stem, the top, the core and one truss per upper lobe.", "Beibehalten: die untersten Stammabschnitte in wahrem Durchmesser, zwei Buckel je Stamm, die Kuppe, der Kern und ein Blütenstand je oberem Buckel."),
            "merged": ("Merged: top mass and lumps into the upper lobe, middle and skirt mass into the lower lobe; the upper stem segments into the dome (they stay in the thermal model).", "Zusammengefasst: Spitzenmasse und Kuppen zum oberen Buckel, Mittel- und Saummasse zum unteren Buckel; die oberen Stammabschnitte in die Kuppel (im thermischen Modell bleiben sie erhalten)."),
            "omitted": ("Omitted: texture, hatching and the trusses of the middle masses.", "Weggelassen: Struktur, Schraffur und die Blütenstände der Mittelmassen."),
            "thinElements": ("No thickening.", "Keine Verstärkung."),
            "hatching": ("No hatching.", "Keine Schraffur."),
            "styles": ("Light line: lobe outlines. Dark solid: one dark dome with pale trusses.", "Helle Linie: Buckelumrisse. Dunkel flächig: eine dunkle Kuppel mit hellen Blütenständen."),
            "overlays": ("Overlays map onto stem-bark, leaf-sun (sixteen lobes and top), leaf-shade (core) and flowers (eleven trusses).", "Überlagerungen bilden ab auf stem-bark, leaf-sun (sechzehn Buckel und Kuppe), leaf-shade (Kern) und flowers (elf Blütenstände)."),
        },
        "500": {
            "silhouette": ("At 6 mm the rhododendron is composed as a dome of a front, a back and a top lobe over its root crown; the outline keeps the true 3.0 m height and 3.5 m width.", "Bei 6 mm ist der Rhododendron als Kuppel aus vorderem, hinterem und oberem Buckel über seinem Wurzelstock komponiert; der Umriss hält die wahre Höhe von 3,0 m und die Breite von 3,5 m."),
            "retained": ("Retained: the dome to 25 cm above the ground and the root crown with the stem feet as one frustum under it.", "Beibehalten: die Kuppel bis 25 cm über dem Boden und der Wurzelstock mit den Stammfüßen als ein Kegelstumpf darunter."),
            "merged": ("Merged: all leaf masses, the core and the trusses into three closed paths listing leaf-sun, leaf-shade and flowers; the stem feet into the root frustum.", "Zusammengefasst: alle Blattmassen, der Kern und die Blütenstände zu drei geschlossenen Pfaden mit leaf-sun, leaf-shade und flowers; die Stammfüße in den Wurzelkegelstumpf."),
            "omitted": ("Omitted: the single trusses (0.35 mm dots on the rim), texture and hatching.", "Weggelassen: die einzelnen Blütenstände (Punkte von 0,35 mm am Rand), Struktur und Schraffur."),
            "thinElements": ("Nothing is thickened.", "Nichts wird verstärkt."),
            "hatching": ("No hatching; 0.15 mm outline, 0.25 mm ground line.", "Keine Schraffur; Umriss 0,15 mm, Bodenlinie 0,25 mm."),
            "styles": ("Light line: three closed outlines and the foot. Dark solid: a dark dome on a lighter foot.", "Helle Linie: drei geschlossene Umrisse und der Fuß. Dunkel flächig: eine dunkle Kuppel auf hellerem Fuß."),
            "overlays": ("The dome paths are merged leaf-sun, leaf-shade and flowers, shown with their area-weighted value and labelled as merged; the foot maps onto stem-bark.", "Die Kuppelpfade fassen leaf-sun, leaf-shade und flowers zusammen, zeigen deren flächengewichteten Wert und sind als zusammengefasst beschriftet; der Fuß bildet auf stem-bark ab."),
        },
    }


def volume_decisions_rhododendron():
    return {
        "50": [("Body at 1:50: the root crown as a frustum, twenty-seven crooked stem capsules at true diameter, thirty-four leaf ellipsoids, the core and fifty-two trusses, each following its drawing in all three views.", "Körper bei 1:50: der Wurzelstock als Kegelstumpf, siebenundzwanzig knorrige Stammkapseln in wahrem Durchmesser, vierunddreißig Blattellipsoide, der Kern und zweiundfünfzig Blütenstände, jeder seiner Zeichnung in allen drei Ansichten folgend."),
               ("Thermal at 1:50: references and two outlets in the top and the core.", "Thermik bei 1:50: Verweise und zwei Auslässe in Kuppe und Kern."),
               ("Structure at 1:50: four-station lofts per stem segment, the root frustum and a fixed disc below the ground.", "Tragwerk bei 1:50: Lofts mit vier Stationen je Stammabschnitt, der Wurzelkegelstumpf und eine eingespannte Scheibe unter dem Boden.")],
        "100": [("Body at 1:100: stems as at 1:50, twenty-six leaf ellipsoids redrawn, the core and twenty-seven trusses.", "Körper bei 1:100: Stämme wie bei 1:50, sechsundzwanzig neu gezeichnete Blattellipsoide, der Kern und siebenundzwanzig Blütenstände."),
                ("Thermal at 1:100: references and two outlets.", "Thermik bei 1:100: Verweise und zwei Auslässe."),
                ("Structure at 1:100: three-station lofts per segment and the root frustum.", "Tragwerk bei 1:100: Lofts mit drei Stationen je Abschnitt und der Wurzelkegelstumpf.")],
        "200": [("Body at 1:200: the lowest stem segments, eighteen lobes and eleven trusses.", "Körper bei 1:200: die untersten Stammabschnitte, achtzehn Buckel und elf Blütenstände."),
                ("Thermal at 1:200: references, the upper stem segments as bark solids of true section and two outlets.", "Thermik bei 1:200: Verweise, die oberen Stammabschnitte als Rindenkörper in wahrem Querschnitt und zwei Auslässe."),
                ("Structure at 1:200: one tapered two-station loft per segment and the root frustum.", "Tragwerk bei 1:200: ein sich verjüngender Loft mit zwei Stationen je Abschnitt und der Wurzelkegelstumpf.")],
        "500": [("Body at 1:500: the root crown with the stem feet as one frustum and the dome as three ellipsoids, as drawn.", "Körper bei 1:500: der Wurzelstock mit den Stammfüßen als ein Kegelstumpf und die Kuppel als drei Ellipsoide, wie gezeichnet."),
                ("Thermal at 1:500: the foot and the dome by reference; one slender bark solid per stem from the knee to the tip, the core and four trusses as thermal solids of their own; two outlets.", "Thermik bei 1:500: Fuß und Kuppel als Verweis; je Stamm ein schlanker Rindenkörper vom Knie bis zur Spitze, der Kern und vier Blütenstände als eigene Thermikkörper; zwei Auslässe."),
                ("Minimal: the structure at 1:500 keeps every stem segment as a two-station loft of its declared section and the root frustum; it cannot drop a member without losing a load path.", "Minimal: Das Tragwerk bei 1:500 behält jeden Stammabschnitt als Loft mit zwei Stationen in seinem angegebenen Querschnitt und den Wurzelkegelstumpf; es kann keinen Stab weglassen, ohne einen Lastpfad zu verlieren.")],
    }
