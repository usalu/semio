"""📝 The bilingual drawing and volume decisions of `used-coat-stand`, one set per preset scale, stated against the numbers of `author_geometry.py`.

@see ./author-coat-stand.py — the script that writes them into the document
"""


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def rep(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """🗂️ The eight decision texts of one representation."""
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles, "overlays": overlays}


NONE = T("None: no part is cut.", "Keine: kein Teil ist geschnitten.")

REP_DECISIONS = {
    50: rep(
        T("Coat stand 14.0 x 37.8 mm on paper, the true views of the 1:50 model: the turned pole with its bottom bead, the two collars and the finial, the near and far foot of each diagonal pair as one bent band, the umbrella ring as a 18 mm band across the legs, the upper hooks ahead and behind in full curve, the edge-on hooks hidden in the pole, the lower diagonal hooks, the broken stump and the screwed-on chromed hook on the near rear side, and the folded blanket under the feet with its rounded folds. Front view the same from ahead with the lot label on the pole; plan: blanket, four feet on the diagonals, ring annulus, three lower hooks, the stump and the replacement hook, the four upper hooks on the axes and the finial disc.",
          "Ständer 14,0 x 37,8 mm auf dem Papier, die wahren Ansichten des 1:50-Modells: die gedrechselte Stange mit Fußwulst, den beiden Bunden und dem Knauf, nahe und ferne Füße jedes Diagonalpaars als ein gebogenes Band, der Schirmring als 18-mm-Band über den Beinen, die oberen Haken vorn und hinten in voller Krümmung, die kantig gesehenen Haken in der Stange verborgen, die unteren Diagonalhaken, der abgebrochene Stumpf und der angeschraubte verchromte Haken auf der nahen hinteren Seite und die gefaltete Decke unter den Füßen mit ihren gerundeten Falten. Vorderansicht ebenso von vorn mit dem Losetikett an der Stange; Grundriss: Decke, vier Füße auf den Diagonalen, Ringkreis, drei untere Haken, Stumpf und Ersatzhaken, die vier oberen Haken auf den Achsen und die Knaufscheibe."),
        T("Pole Ø47 with collars Ø54 and Ø56 and finial Ø82, feet Ø26, upper hooks Ø22, lower hooks Ø18, ring Ø18, replacement hook plate 6 mm and arm Ø9 with its Ø12 ball, blanket 45 mm.", "Stange Ø47 mit Bunden Ø54 und Ø56 und Knauf Ø82, Füße Ø26, obere Haken Ø22, untere Haken Ø18, Ring Ø18, Ersatzhakenplatte 6 mm und Arm Ø9 mit Kugel Ø12, Decke 45 mm."),
        T("Each diagonal pair of feet and lower hooks coincides in the side and front views and is one band; the two screws of the replacement hook lie in its plate.", "Jedes Diagonalpaar von Füßen und unteren Haken fällt in Seiten- und Vorderansicht zusammen und ist ein Band; die beiden Schrauben des Ersatzhakens liegen in seiner Platte."),
        T("The dowels of the hooks, the wood screws of the feet and the ring, the wood grain.", "Die Dübel der Haken, die Holzschrauben von Füßen und Ring, die Maserung."),
        T("Rods at their true diameters drawn with a 0.25 mm pen (feet 0.52 mm, hooks 0.44 and 0.36 mm wide); scuffs at the foot tips and the worn hook tips are 0.13 mm texture strokes, the glue line of the cracked rear foot and the turned grooves of the collars 0.25 mm detail strokes, the quilting of the blanket 0.13 mm texture.", "Stäbe in wahren Durchmessern mit 0,25-mm-Stift (Füße 0,52 mm, Haken 0,44 und 0,36 mm breit); Abriebstellen an den Fußspitzen und die abgegriffenen Hakenspitzen sind 0,13-mm-Texturstriche, die Leimfuge des gerissenen hinteren Fußes und die Drehrillen der Bunde 0,25-mm-Detailstriche, die Steppung der Decke 0,13-mm-Textur."),
        NONE,
        T("Stained beech 0.4, worn wood 0.62, chrome 0.85, blanket 0.66, label 0.95.", "Gebeizte Buche 0,4, abgenutztes Holz 0,62, Chrom 0,85, Decke 0,66, Etikett 0,95."),
        T("One shape per part and region: pole, feet, hooks, ring, replacement hook and blanket each carry their own thermal region.", "Eine Form je Teil und Region: Stange, Füße, Haken, Ring, Ersatzhaken und Decke tragen je ihre eigene thermische Region."),
    ),
    100: rep(
        T("Coat stand 7.0 x 18.9 mm: the pole with bead, collars and finial in simplified turning, both foot bands of each view, the ring band, the upper hooks ahead and behind, the lower diagonal hooks, the stump and the replacement hook as one Ø13 arm, the blanket with its rounded folds. Front view from ahead the same, plan as at 1:50 without the plate of the replacement hook.", "Ständer 7,0 x 18,9 mm: die Stange mit Wulst, Bunden und Knauf in vereinfachter Drehform, beide Fußbänder jeder Ansicht, das Ringband, die oberen Haken vorn und hinten, die unteren Diagonalhaken, der Stumpf und der Ersatzhaken als ein Arm Ø13, die Decke mit gerundeten Falten. Vorderansicht von vorn ebenso, Grundriss wie bei 1:50 ohne die Platte des Ersatzhakens."),
        T("Pole Ø47, collars, finial Ø82, feet Ø26, hooks Ø22 and Ø18, ring Ø18, replacement hook arm Ø13, blanket 45 mm.", "Stange Ø47, Bunde, Knauf Ø82, Füße Ø26, Haken Ø22 und Ø18, Ring Ø18, Ersatzhakenarm Ø13, Decke 45 mm."),
        T("The plate and ball of the replacement hook into its arm; the small transitions of the turning into straight steps.", "Platte und Kugel des Ersatzhakens in seinen Arm; die kleinen Übergänge der Drehform in gerade Stufen."),
        T("Lot label, scuffs, worn tips, glue line, quilting.", "Losetikett, Abriebstellen, abgegriffene Spitzen, Leimfuge, Steppung."),
        T("Rods at their true diameters with a 0.18 mm pen; collar grooves as 0.18 mm detail strokes.", "Stäbe in wahren Durchmessern mit 0,18-mm-Stift; Bundrillen als 0,18-mm-Detailstriche."),
        NONE,
        T("As at 1:50.", "Wie bei 1:50."),
        T("One shape per part and region.", "Eine Form je Teil und Region."),
    ),
    200: rep(
        T("Coat stand 3.5 x 9.5 mm: pole with collar block and finial, feet, ring, upper and lower hooks as filled bands of their true width under a 0.18 mm outline, the replacement hook as one 18 mm stub on the pole, the blanket as a rounded slab.", "Ständer 3,5 x 9,5 mm: Stange mit Bundblock und Knauf, Füße, Ring, obere und untere Haken als gefüllte Bänder in wahrer Breite unter 0,18-mm-Umriss, der Ersatzhaken als ein 18-mm-Stummel an der Stange, die Decke als gerundete Platte."),
        T("Pole, finial, collars, four feet, ring, seven hooks, replacement hook, blanket.", "Stange, Knauf, Bunde, vier Füße, Ring, sieben Haken, Ersatzhaken, Decke."),
        T("The two collars and the neck into one turned block; the replacement hook into a Ø18 stub; the folds of the blanket into rounded edges.", "Die beiden Bunde und der Hals zu einem gedrehten Block; der Ersatzhaken zu einem Stummel Ø18; die Falten der Decke zu gerundeten Kanten."),
        T("The broken stump (60 mm) and every detail stroke.", "Der abgebrochene Stumpf (60 mm) und jeder Detailstrich."),
        T("Feet 0.13 mm, hooks 0.11 and 0.09 mm wide, each read through its 0.18 mm outline; the hook curves keep their control points.", "Füße 0,13 mm, Haken 0,11 und 0,09 mm breit, jeweils durch ihren 0,18-mm-Umriss lesbar; die Hakenbögen behalten ihre Stützpunkte."),
        NONE,
        T("As at 1:50.", "Wie bei 1:50."),
        T("One shape per part and region.", "Eine Form je Teil und Region."),
    ),
    500: rep(
        T("Coat stand sign 1.4 x 3.8 mm: the pole with its finial knob, the four feet and the four upper hooks as filled bands of their true width (0.05 and 0.04 mm) read through the 0.25 mm outline and drawn over the pole, the ring as a Ø18 band and annulus, the blanket as a slab.", "Ständerzeichen 1,4 x 3,8 mm: die Stange mit ihrem Knauf, die vier Füße und die vier oberen Haken als gefüllte Bänder in wahrer Breite (0,05 und 0,04 mm), durch den 0,25-mm-Umriss lesbar und über der Stange gezeichnet, der Ring als Band und Kreisring Ø18, die Decke als Platte."),
        T("Pole, finial, feet, upper hooks, ring, blanket.", "Stange, Knauf, Füße, obere Haken, Ring, Decke."),
        T("The replacement hook into the pole shape, which carries both regions.", "Der Ersatzhaken in die Stangenform, die beide Regionen trägt."),
        T("The lower hooks (0.17 m reach, 0.34 mm of paper), the stump and all details.", "Die unteren Haken (0,17 m Ausladung, 0,34 mm Papier), der Stumpf und alle Einzelheiten."),
        T("Feet, hooks and ring at their true Ø26, Ø22 and Ø18 under a 0.25 mm outline, so every band prints at least 0.27 mm wide up to 1:1000 (with the 0.18 mm pen the hooks measured 0.196 mm there).", "Füße, Haken und Ring in wahren Ø26, Ø22 und Ø18 unter 0,25-mm-Umriss, so druckt jedes Band bis 1:1000 mindestens 0,27 mm breit (mit dem 0,18-mm-Stift maßen die Haken dort 0,196 mm)."),
        NONE,
        T("Wood 0.4, blanket 0.66.", "Holz 0,4, Decke 0,66."),
        T("The pole shape carries the pole and replacement-hook regions; the bands their own.", "Die Stangenform trägt die Regionen Stange und Ersatzhaken; die Bänder ihre eigenen."),
    ),
}

VOLUME_DECISIONS = {
    50: [
        T("1:50 is the full coat stand at its 7.5 mm feature: the pole is one revolve of its turned profile (36 points: bottom bead, Ø47 shaft with a slight swell, lower collar Ø54, upper collar Ø56, neck and finial Ø82); the four feet, four upper hooks and three lower hooks are lofts of their steam-bent round sections along Catmull-Rom centre lines (three sections per control segment, every section perpendicular to the path, rounded knob tips); every rod is at least 18 mm, above the 7.5 mm feature, so none needs a slender declaration and all are compared at their true section.", "1:50 ist der ganze Garderobenständer bei seinem 7,5-mm-Merkmal: die Stange ist ein Rotationskörper ihres Drehprofils (36 Punkte: Fußwulst, Schaft Ø47 mit leichter Schwellung, unterer Bund Ø54, oberer Bund Ø56, Hals und Knauf Ø82); die vier Füße, vier oberen und drei unteren Haken sind Lofts ihrer dampfgebogenen Rundquerschnitte entlang Catmull-Rom-Mittellinien (drei Schnitte je Stützsegment, jeder senkrecht zur Bahn, gerundete Knaufspitzen); jeder Stab hat mindestens 18 mm, über dem 7,5-mm-Merkmal, keiner braucht daher eine Schlankheitserklärung und alle werden in ihrem wahren Querschnitt verglichen."),
        T("Traces of use as geometry: the fourth lower hook is a 60 mm broken stump with a flat fracture face, and the chromed zinc hook screwed on 0.12 m below it is its plate (50 x 20 x 6 mm, set into the round pole by 2 mm), a Ø9 arm and a Ø12 ball. The scuffed foot tips, worn hook tips, the glue line of a cracked foot and the paper lot label are texture and detail strokes of the drawing only (under 1 mm deep). The feet press 3 mm into the folded blanket, the ring is screwed 2 mm into the legs, every hook end sits in its collar, so the body is one object.", "Gebrauchsspuren als Geometrie: der vierte untere Haken ist ein 60-mm-Stumpf mit ebener Bruchfläche, und der 0,12 m darunter angeschraubte verchromte Zinkhaken ist seine Platte (50 x 20 x 6 mm, 2 mm in die runde Stange gesetzt), ein Arm Ø9 und eine Kugel Ø12. Abgestoßene Fußspitzen, abgegriffene Hakenspitzen, die Leimfuge eines gerissenen Fußes und das Papier-Losetikett sind nur Textur- und Detailstriche der Zeichnung (unter 1 mm tief). Die Füße drücken 3 mm in die gefaltete Decke, der Ring ist 2 mm in die Beine geschraubt, jedes Hakenende sitzt in seinem Bund, der Körper ist daher ein Objekt."),
        T("Thermal: every body solid is the surface of its region ({ of } references). Structure: the pole as a slender Ø45 member from 0.095 to 1.745 m, the feet, ring and hooks as members of the stand bone, joints at the four screwed foot tops, the four ring screws, both dowelled collars and the replacement hook plate, the blanket as its own member, mass markers at the centres of mass.", "Thermik: jeder Körper ist die Oberfläche seiner Region ({ of }-Verweise). Tragwerk: die Stange als schlanker Stab Ø45 von 0,095 bis 1,745 m, Füße, Ring und Haken als Stäbe des Ständerknochens, Gelenke an den vier verschraubten Fußköpfen, den vier Ringschrauben, beiden gedübelten Bunden und der Ersatzhakenplatte, die Decke als eigener Stab, Massenmarken in den Schwerpunkten."),
    ],
    100: [
        T("1:100 at its 15 mm feature: the pole revolve with 22 profile points (bead and turning transitions simplified), feet and hooks with two loft sections per control segment, the ring loop with 14 points; the plate and ball of the replacement hook (6 and 12 mm) merge into one Ø13 arm that keeps the surface of the hook (0.0063 m2), drawn 12.4 mm wide; the stump stays (Ø18).", "1:100 bei seinem 15-mm-Merkmal: der Rotationskörper der Stange mit 22 Profilpunkten (Wulst und Drehübergänge vereinfacht), Füße und Haken mit zwei Loftschnitten je Stützsegment, die Ringschleife mit 14 Punkten; Platte und Kugel des Ersatzhakens (6 und 12 mm) gehen in einen Arm Ø13 auf, der die Oberfläche des Hakens (0,0063 m2) behält, 12,4 mm breit gezeichnet; der Stumpf bleibt (Ø18)."),
        T("Structure: the pole member from 0.10 to 1.74 m, joints at the foot tops and ring screws only; the collar dowels are inside the pole member at this scale.", "Tragwerk: der Stangenstab von 0,10 bis 1,74 m, Gelenke nur an den Fußköpfen und Ringschrauben; die Bunddübel liegen bei diesem Maßstab im Stangenstab."),
    ],
    200: [
        T("1:200 at its 30 mm feature: the pole revolve with 15 profile points (collars and neck one turned block), feet and hooks lofted through their hand-picked control points only (10, 13 and 13 sections), ring loop of 10 points, the replacement hook one Ø18 capsule, the stump omitted (60 mm), the blanket a rounded box. Rods thinner than 30 mm keep their true section because the drawing of their bone draws them at it (the thinnest is the Ø18 lower hook, rule C13).", "1:200 bei seinem 30-mm-Merkmal: der Rotationskörper der Stange mit 15 Profilpunkten (Bunde und Hals ein gedrehter Block), Füße und Haken nur durch ihre ausgewählten Stützpunkte geloftet (10, 13 und 13 Schnitte), Ringschleife mit 10 Punkten, der Ersatzhaken eine Kapsel Ø18, der Stumpf entfällt (60 mm), die Decke ein gerundeter Quader. Stäbe unter 30 mm behalten ihren wahren Querschnitt, weil die Zeichnung ihres Knochens sie so zeichnet (der dünnste ist der untere Haken Ø18, Regel C13)."),
        T("Structure: pole member 0.11 to 1.73 m, joints at the four foot tops, masses at their centres.", "Tragwerk: Stangenstab 0,11 bis 1,73 m, Gelenke an den vier Fußköpfen, Massen in ihren Schwerpunkten."),
    ],
    500: [
        T("1:500 at its 75 mm feature: the pole a revolve of 9 points (shaft, collar block, finial knob Ø80), the feet and upper hooks lofts through 7 and 8 control points with flat ends at their true Ø26 and Ø22 (drawn at that width, rule C13), the ring a loop of 8 points at its true Ø18, the blanket a plain box. The lower hooks (0.17 m reach, Ø18) and the stump are left out; the replacement hook survives in the thermal list as a slender Ø5 stand-in of its region, drawn inside the pole shape.", "1:500 bei seinem 75-mm-Merkmal: die Stange ein Rotationskörper aus 9 Punkten (Schaft, Bundblock, Knauf Ø80), Füße und obere Haken Lofts durch 7 und 8 Stützpunkte mit flachen Enden in wahren Ø26 und Ø22 (so gezeichnet, Regel C13), der Ring eine Schleife aus 8 Punkten in wahren Ø18, die Decke ein einfacher Quader. Die unteren Haken (0,17 m Ausladung, Ø18) und der Stumpf entfallen; der Ersatzhaken bleibt in der thermischen Liste als schlanker Platzhalter Ø5 seiner Region, in der Stangenform gezeichnet."),
        T("Structure: pole member 0.12 to 1.72 m, feet, ring and upper hooks as members, no joint markers (the connections lie inside the members at this scale), masses at their centres.", "Tragwerk: Stangenstab 0,12 bis 1,72 m, Füße, Ring und obere Haken als Stäbe, keine Gelenkmarken (die Verbindungen liegen bei diesem Maßstab in den Stäben), Massen in ihren Schwerpunkten."),
    ],
}
