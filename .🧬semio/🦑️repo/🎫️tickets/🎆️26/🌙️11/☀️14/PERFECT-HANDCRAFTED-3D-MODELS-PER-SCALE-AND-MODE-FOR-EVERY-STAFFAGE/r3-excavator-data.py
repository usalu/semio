"""📏 Root data of the round 3 excavator that the drawings and the model do not carry themselves: the overall dimensions of the transport pose, the outer surface areas of the thermal regions measured on the 1:50 model (each within a factor 1.5 of the former value, design C16 and C29) and the drawing decisions of every scale."""

T = lambda en, de: {"en": en, "de": de}
DIMENSIONS = {"heightM": 3.25, "widthM": 9.63, "depthM": 2.98}
AREAS = {"house": 45.3, "counterweight": 9.25, "cab-glazing": 5.47, "tracks": 57.4}
DECISIONS = {
    "50": {
        "silhouette": T("True side elevation of the 22 t machine in its transport pose: every outline is the projection of the modelled part.", "Wahre Seitenansicht der 22-t-Maschine in Transportstellung: Jeder Umriss ist die Projektion des modellierten Teils."),
        "retained": T("Track band with link rail, grouser ticks, sprocket with final drive hub, idler, eight track rollers and two carrier rollers under the track frame, carbody, turret and slew bearing; deck, engine house, tool box, hood cover, vent grille with louvres, door seams, exhaust stack, handrail, mirrors, work light, counterweight with lifting eyes; cab with roof, seat and side glass; boom with bosses, cylinder lug, weld seams, steel lines and lamp, stick with forked tail, bucket with lip, teeth, ears and wear strips, idler links and H link, four cylinders with barrel, gland, eyes and chrome rod. Front view and plan are the same solids seen from ahead and from above: fork cheeks, link plates and cylinder pairs are drawn singly.", "Kettenband mit Kettenschiene, Stollenstriche, Antriebsrad mit Fahrgetriebenabe, Leitrad, acht Laufrollen und zwei Stützrollen unter dem Laufwerksrahmen, Mittelrahmen, Turm und Drehkranzlager; Plattform, Maschinenhaus, Werkzeugkasten, Haubendeckel, Lüftungsgitter mit Lamellen, Türfugen, Auspuffrohr, Handlauf, Spiegel, Arbeitsscheinwerfer, Gegengewicht mit Hebeösen; Kabine mit Dach, Sitz und Seitenscheibe; Ausleger mit Naben, Zylinderlasche, Schweißnähten, Stahlleitungen und Leuchte, Stiel mit Gabelende, Löffel mit Schneide, Zähnen, Laschen und Schleißleisten, Umlenkhebel und H-Lasche, vier Zylinder mit Rohr, Führungsbuchse, Augen und Chromstange. Vorderansicht und Grundriss zeigen dieselben Körper von vorn und von oben: Gabelwangen, Laschenbleche und Zylinderpaare sind einzeln gezeichnet."),
        "merged": T("Nothing merged.", "Nichts zusammengefasst."),
        "omitted": T("Hoses at the joints, steps, cab interior beyond the seat, bolts. Seams, welds and louvres are drawn as thin filled bands of their real width so that they stay solids when the upper structure slews.", "Schläuche an den Gelenken, Tritte, Kabineninneres außer dem Sitz, Schrauben. Fugen, Schweißnähte und Lamellen sind als schmale gefüllte Bänder ihrer wirklichen Breite gezeichnet, damit sie beim Schwenken des Oberwagens Körper bleiben."),
        "thinElements": T("Seam bands 4 to 8 mm wide, grouser ticks 26 mm long at the texture pen, handrail and steel lines at their real 32 and 34 mm.", "Fugenbänder 4 bis 8 mm breit, Stollenstriche 26 mm lang mit dem Texturstift, Handlauf und Stahlleitungen in ihren wirklichen 32 und 34 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Three line weights 0.25 / 0.18 / 0.10 mm.", "Drei Linienstärken 0,25 / 0,18 / 0,10 mm."),
        "overlays": T("All regions at real extent.", "Alle Regionen in realer Ausdehnung."),
    },
    "100": {
        "silhouette": T("The machine drawn anew for 1:100: one track band on its link chain, wheels as discs, house, cab block and the forked boom and stick with the linkage.", "Die Maschine für 1:100 neu gezeichnet: ein Kettenband auf seinem Kettenstrang, Räder als Scheiben, Oberwagen, Kabinenblock und die gegabelten Ausleger und Stiel mit der Kinematik."),
        "retained": T("Track band, sprocket with hub, idler, eight track rollers, two carrier rollers, track frame, carbody and turret drum; deck, engine house, tool box, hood cover, both grilles, exhaust stack, handrail, boom foot, counterweight; cab block with side glass, windscreen in the front view and its strip in the plan; boom with root and tip boss and cylinder lug, stick with root boss and lug, bucket with five teeth and ears, both links, four cylinders as barrel and rod.", "Kettenband, Antriebsrad mit Nabe, Leitrad, acht Laufrollen, zwei Stützrollen, Laufwerksrahmen, Mittelrahmen und Turmtrommel; Plattform, Maschinenhaus, Werkzeugkasten, Haubendeckel, beide Gitter, Auspuffrohr, Handlauf, Auslegerfuß, Gegengewicht; Kabinenblock mit Seitenscheibe, Frontscheibe in der Vorderansicht und ihr Streifen im Grundriss; Ausleger mit Fuß- und Kopfnabe und Zylinderlasche, Stiel mit Fußnabe und Lasche, Löffel mit fünf Zähnen und Laschen, beide Hebel, vier Zylinder als Rohr und Stange."),
        "merged": T("Shoes and grousers into one band, the link rails into one chain, the lug pairs into one block each, gland and eyes into barrel and rod.", "Platten und Stollen zu einem Band, die Kettenschienen zu einem Strang, die Laschenpaare zu je einem Block, Führungsbuchse und Augen in Rohr und Stange."),
        "omitted": T("Grouser ticks, louvres, seams, welds, lamps, mirrors, seat, wear strips, pins.", "Stollenstriche, Lamellen, Fugen, Schweißnähte, Leuchten, Spiegel, Sitz, Schleißleisten, Bolzen."),
        "thinElements": T("Handrail at its real 32 mm (0.32 mm), grille plates 20 mm proud.", "Handlauf in seinen wirklichen 32 mm (0,32 mm), Gitterbleche 20 mm vorstehend."),
        "hatching": T("None.", "Keine."),
        "styles": T("Two line weights 0.18 / 0.13 mm, glazing 0.10 mm.", "Zwei Linienstärken 0,18 / 0,13 mm, Verglasung 0,10 mm."),
        "overlays": T("All regions at real extent.", "Alle Regionen in realer Ausdehnung."),
    },
    "200": {
        "silhouette": T("Masses: track band around the frame beam with two end discs, house prism with cab block, counterweight, forked boom and stick with generalised outlines, bucket, links and cylinders.", "Massen: Kettenband um den Rahmenträger mit zwei Endscheiben, Hausprisma mit Kabinenblock, Gegengewicht, gegabelte Ausleger und Stiel mit generalisierten Umrissen, Löffel, Laschen und Zylinder."),
        "retained": T("Track band, frame beam, sprocket with hub and idler, carbody and turret; deck, house, tool box, grilles, exhaust stack (0.12 m = 0.6 mm), boom foot, counterweight, cab block with glass; boom with cylinder lug, stick, bucket with tooth bar and ears, both links as bars, four cylinders as barrel and rod.", "Kettenband, Rahmenträger, Antriebsrad mit Nabe und Leitrad, Mittelrahmen und Turm; Plattform, Haus, Werkzeugkasten, Gitter, Auspuffrohr (0,12 m = 0,6 mm), Auslegerfuß, Gegengewicht, Kabinenblock mit Glas; Ausleger mit Zylinderlasche, Stiel, Löffel mit Zahnleiste und Laschen, beide Hebel als Stäbe, vier Zylinder als Rohr und Stange."),
        "merged": T("Shoes, grousers and links into one band; rollers into the frame beam; teeth into one bar; paired link plates into one bar each.", "Platten, Stollen und Glieder zu einem Band; Rollen in den Rahmenträger; Zähne zu einer Leiste; paarige Laschenbleche zu je einem Stab."),
        "omitted": T("Rollers, carrier rollers, hood cover, handrail, bosses, the bucket cylinder lug.", "Laufrollen, Stützrollen, Haubendeckel, Handlauf, Naben, die Lasche des Löffelzylinders."),
        "thinElements": T("Chrome rods 85 to 100 mm (0.43 to 0.5 mm), grille plates 40 mm proud.", "Chromstangen 85 bis 100 mm (0,43 bis 0,5 mm), Gitterbleche 40 mm vorstehend."),
        "hatching": T("None.", "Keine."),
        "styles": T("Two line weights 0.13 / 0.10 mm.", "Zwei Linienstärken 0,13 / 0,10 mm."),
        "overlays": T("All regions at real extent.", "Alle Regionen in realer Ausdehnung."),
    },
    "500": {
        "silhouette": T("Crawler excavator as two solid track blocks, one house with cab and counterweight and the three arm masses with their cylinders; stowed, the stick folds under the boom.", "Kettenbagger als zwei massive Kettenblöcke, ein Oberwagen mit Kabine und Gegengewicht und die drei Armmassen mit ihren Zylindern; gestaut klappt der Stiel unter den Ausleger."),
        "retained": T("Track blocks, carbody, turret, house, tool box, exhaust stack, cab block with glass, counterweight, boom foot, boom, stick, bucket with tooth bar, both links and the four cylinders as barrel and rod so that the drive of the arm is still shown.", "Kettenblöcke, Mittelrahmen, Turm, Haus, Werkzeugkasten, Auspuffrohr, Kabinenblock mit Glas, Gegengewicht, Auslegerfuß, Ausleger, Stiel, Löffel mit Zahnleiste, beide Hebel und die vier Zylinder als Rohr und Stange, damit der Armantrieb sichtbar bleibt."),
        "merged": T("Engine compartment and exhaust stack merge into the house fill (area-weighted overlay); track band, frame, rollers, sprocket and idler merge into the track block; the deck merges into house, tool box and cab.", "Motorraum und Auspuffrohr gehen in der Hausfläche auf (flächengewichtete Überlagerung); Kettenband, Rahmen, Rollen, Antriebs- und Leitrad gehen im Kettenblock auf; die Plattform geht in Haus, Werkzeugkasten und Kabine auf."),
        "omitted": T("Grilles, cylinder lugs, bucket ears, bosses.", "Gitter, Zylinderlaschen, Löffellaschen, Naben."),
        "thinElements": T("Chrome rods 85 to 100 mm (0.17 to 0.2 mm) at the body pen.", "Chromstangen 85 bis 100 mm (0,17 bis 0,2 mm) mit dem Körperstift."),
        "hatching": T("None.", "Keine."),
        "styles": T("One line weight 0.15 mm, glazing 0.13 mm.", "Eine Linienstärke 0,15 mm, Verglasung 0,13 mm."),
        "overlays": T("House, engine compartment and exhaust stack share the house fill; all other regions at real extent.", "Haus, Motorraum und Auspuffrohr teilen die Hausfläche; alle übrigen Regionen in realer Ausdehnung."),
    },
}


def apply(a):
    a["dimensions"].update(DIMENSIONS)
    for region in a["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    for scale, decisions in DECISIONS.items():
        rep = a["representations"][scale]
        assert set(decisions) == set(rep["decisions"]), (scale, set(rep["decisions"]) ^ set(decisions))
        rep["decisions"] = decisions
