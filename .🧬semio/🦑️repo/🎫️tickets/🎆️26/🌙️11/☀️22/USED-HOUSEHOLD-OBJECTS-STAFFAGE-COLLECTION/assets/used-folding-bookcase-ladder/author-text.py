"""📝️ Every bilingual text, the materials, the thermal regions and the provenance of the used folding ladder shelf (English first, German second).

@see ./author-build.py"""


def T(en, de):
    """🗣️ A text in both languages."""
    return {"en": en, "de": de}


SUMMARY = T(
    "A second-hand folding A-frame ladder shelf of solid pine, 0.60 m wide and 1.80 m high, with five graduated shelves 30 to 10 cm deep hinged to the leaning front rails and resting on beech cleats of the rear rails that stand against the wall. It comes from a flat clearance with grey scuffed feet, a chipped front corner on the second shelf, a mismatched hex bolt replacing one hinge bolt, a price sticker and sun-faded upper shelves; it is stored folded flat and leaning on a folded moving blanket with its unscrewed wall-anchor block beside it, carried by one person at two grips and set up again at a wall with books and plants.",
    "Ein gebrauchtes klappbares Leiterregal aus massiver Kiefer, 0,60 m breit und 1,80 m hoch, mit fünf gestaffelten Böden von 30 bis 10 cm Tiefe, die an den angelehnten Vorderholmen angelenkt sind und hinten auf Buchenleisten der an der Wand stehenden Hinterholme liegen. Es stammt aus einer Haushaltsauflösung, mit grau abgestoßenen Füßen, einer abgeplatzten Vorderecke am zweiten Boden, einer nicht passenden Sechskantschraube statt einer Gelenkschraube, einem Preisaufkleber und sonnengebleichten oberen Böden; gelagert wird es flach zusammengeklappt, angelehnt auf einer gefalteten Möbeldecke mit dem abgeschraubten Wandankerklotz daneben, getragen von einer Person an zwei Griffstellen und an einer Wand mit Büchern und Pflanzen wieder aufgestellt.",
)

BONE_TITLES = {
    "ground": T("Floor reference", "Bodenbezug"),
    "blanket": T("Folded moving blanket on the floor (storage aid, stays behind)", "Gefaltete Möbeldecke am Boden (Lagerhilfe, bleibt liegen)"),
    "lift": T("Lift of the shelf off the floor", "Anheben des Regals vom Boden"),
    "slide": T("Shift of the shelf along the floor", "Verschieben des Regals am Boden"),
    "ladder": T("Front ladder: two leaning pine rails from the feet to the hinge bolts", "Vorderleiter: zwei angelehnte Kiefernholme von den Füßen bis zu den Gelenkschrauben"),
    "rear": T("Rear frame: two upright pine rails with ten beech cleats, hinged under the top", "Hinterrahmen: zwei stehende Kiefernholme mit zehn Buchenleisten, unter dem Kopf angelenkt"),
    "shelf-1": T("Shelf 1 (bottom, 298 mm deep) on its hinge pins", "Boden 1 (unten, 298 mm tief) auf seinen Gelenkstiften"),
    "shelf-2": T("Shelf 2 (242 mm deep, chipped corner) on its hinge pins", "Boden 2 (242 mm tief, abgeplatzte Ecke) auf seinen Gelenkstiften"),
    "shelf-3": T("Shelf 3 (195 mm deep) on its hinge pins", "Boden 3 (195 mm tief) auf seinen Gelenkstiften"),
    "shelf-4": T("Shelf 4 (149 mm deep) on its hinge pins", "Boden 4 (149 mm tief) auf seinen Gelenkstiften"),
    "shelf-5": T("Shelf 5 (top, 102 mm deep) on its hinge pins", "Boden 5 (oben, 102 mm tief) auf seinen Gelenkstiften"),
    "anchor-lift": T("Height of the wall-anchor block", "Höhe des Wandankerklotzes"),
    "anchor-slide": T("Shift of the wall-anchor block", "Verschieben des Wandankerklotzes"),
    "wall-anchor": T("Wall-anchor block: pine block 60 x 28 x 22 mm, on the wall in use, unscrewed beside the shelf in storage", "Wandankerklotz: Kiefernklotz 60 x 28 x 22 mm, in Gebrauch an der Wand, im Lager abgeschraubt neben dem Regal"),
}

ANCHOR_TITLES = {
    "foot-front-l": T("Left front foot", "Linker Vorderfuß"),
    "foot-front-r": T("Right front foot", "Rechter Vorderfuß"),
    "foot-rear-l": T("Left rear foot", "Linker Hinterfuß"),
    "foot-rear-r": T("Right rear foot", "Rechter Hinterfuß"),
    "rest-wall-l": T("Top of the left rear rail against a wall (folded and leaning)", "Kopf des linken Hinterholms an einer Wand (zusammengeklappt, angelehnt)"),
    "rest-wall-r": T("Top of the right rear rail against a wall (folded and leaning)", "Kopf des rechten Hinterholms an einer Wand (zusammengeklappt, angelehnt)"),
    "grip-r": T("Lower hand under the right front rail", "Untere Hand unter dem rechten Vorderholm"),
    "grip-l": T("Upper hand under the left front rail", "Obere Hand unter dem linken Vorderholm"),
    "wall-fixing": T("Wall fixing: anchor block screwed to the wall and to the left rear rail", "Wandbefestigung: Ankerklotz an die Wand und an den linken Hinterholm geschraubt"),
    "pull-point": T("Left end of the front edge of the top shelf, where a pull acts", "Linkes Ende der Vorderkante des obersten Bodens, an dem ein Zug angreift"),
}

LOAD_TITLES = [
    T("Shelf 1: art books and folders, 22 kg", "Boden 1: Bildbände und Ordner, 22 kg"),
    T("Shelf 2: hardback novels, 15 kg", "Boden 2: gebundene Romane, 15 kg"),
    T("Shelf 3: paperbacks and a small potted plant, 8.5 kg", "Boden 3: Taschenbücher und eine kleine Topfpflanze, 8,5 kg"),
    T("Shelf 4: a watered potted plant in a 14 cm pot and two books, 5 kg", "Boden 4: eine gegossene Topfpflanze im 14-cm-Topf und zwei Bücher, 5 kg"),
    T("Shelf 5: a trailing plant in a 12 cm pot, 1.5 kg", "Boden 5: eine Hängepflanze im 12-cm-Topf, 1,5 kg"),
]

POSE_TEXT = {
    "stored": (T("Stored folded at the depot", "Zusammengeklappt im Lager"), T("The rear frame is folded against the front rails and held by a turn of packing tape, the five shelves are folded up between the rails; the flat package (78 mm) leans with its top against the depot wall and stands on a moving blanket folded eight times, the unscrewed wall-anchor block lies on the blanket beside the feet.", "Der Hinterrahmen ist an die Vorderholme geklappt und mit einer Lage Packband gehalten, die fünf Böden sind zwischen die Holme hochgeklappt; das flache Paket (78 mm) lehnt mit dem Kopf an der Lagerwand und steht auf einer achtfach gefalteten Möbeldecke, der abgeschraubte Wandankerklotz liegt neben den Füßen auf der Decke.")),
    "carried": (T("Carried folded by one person", "Zusammengeklappt von einer Person getragen"), T("The folded package is lifted off the blanket and carried tilted at 70 degrees with the feet 0.30 m above the floor, one hand under each front rail 0.60 m apart along the rails around the centre of mass; the blanket and the anchor block stay behind on the floor.", "Das zusammengeklappte Paket ist von der Decke gehoben und wird unter 70 Grad geneigt getragen, die Füße 0,30 m über dem Boden, je eine Hand unter einem Vorderholm, 0,60 m auseinander um den Schwerpunkt; Decke und Ankerklotz bleiben am Boden liegen.")),
    "in-use": (T("In use with books and plants", "In Gebrauch mit Büchern und Pflanzen"), T("Open at a living-room wall: the rear rails stand against the wall and are screwed to the anchor block, the front rails lean at 8.8 degrees, the five shelves lie level on their cleats and carry 52 kg of books and potted plants; the moving blanket still lies folded on the floor in front.", "Offen an einer Wohnzimmerwand: die Hinterholme stehen an der Wand und sind am Ankerklotz verschraubt, die Vorderholme lehnen unter 8,8 Grad, die fünf Böden liegen waagrecht auf ihren Leisten und tragen 52 kg Bücher und Topfpflanzen; die Möbeldecke liegt noch gefaltet davor am Boden.")),
    "opened-empty": (T("Opened and empty", "Aufgeklappt und leer"), T("Set up at the wall and fixed to the anchor block, all shelves down and empty.", "An der Wand aufgestellt und am Ankerklotz gesichert, alle Böden heruntergeklappt und leer.")),
    "pull-test": (T("Pull test of the wall fixing", "Zugprobe der Wandbefestigung"), T("Empty and fixed, a forward pull of 50 N acts on the left end of the front edge of the top shelf at 1.52 m, in line with the anchor block; without the anchor the empty shelf would tip over its front feet from about 14 N, the wall fixing takes 40 N of the pull and the feet the rest by friction.", "Leer und gesichert, ein Zug von 50 N nach vorn greift am linken Ende der Vorderkante des obersten Bodens auf 1,52 m an, in einer Flucht mit dem Ankerklotz; ohne Anker kippte das leere Regal schon ab etwa 14 N über die Vorderfüße, die Wandbefestigung nimmt 40 N des Zugs auf, die Füße den Rest durch Reibung.")),
    "folded-home": (T("Folded against the home wall", "Zusammengeklappt an der Wand der Wohnung"), T("Key pose of the unfolding: the folded package leans on the wall where the anchor block is already screwed on.", "Schlüsselpose des Aufklappens: das zusammengeklappte Paket lehnt an der Wand, an der der Ankerklotz schon angeschraubt ist.")),
}

CLIP_TEXT = {
    "carry": (T("Carry off the depot wall", "Von der Lagerwand wegtragen"), T("One person grips the folded shelf at both front rails, pulls its top off the depot wall, lifts the feet off the moving blanket and tilts the package to 70 degrees to carry it; blanket and anchor block stay where they lay.", "Eine Person greift das zusammengeklappte Regal an beiden Vorderholmen, zieht den Kopf von der Lagerwand, hebt die Füße von der Möbeldecke und kippt das Paket zum Tragen auf 70 Grad; Decke und Ankerklotz bleiben liegen.")),
    "unfold": (T("Unfold at the wall", "An der Wand aufklappen"), T("At its new wall the rear frame swings back against the wall onto the anchor block, then the shelves are folded down one by one from the top until each rests on its two cleats.", "An der neuen Wand schwenkt der Hinterrahmen zurück an die Wand auf den Ankerklotz, dann werden die Böden einer nach dem anderen von oben heruntergeklappt, bis jeder auf seinen zwei Leisten liegt.")),
}

MATERIALS = [
    {"id": "pine-aged", "title": T("Solid Scots pine, clear lacquer yellowed with age", "Massive Kiefer, Klarlack mit dem Alter vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 500, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "din-68364-2003"},
    {"id": "pine-board", "title": T("Glued laminated pine panel 18 mm, oiled, faded by the sun on the upper shelves", "Kiefern-Leimholzplatte 18 mm, geölt, auf den oberen Böden von der Sonne ausgeblichen"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 480, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000.0, "strengthPa": 20000000.0, "source": "literature", "reference": "din-68364-2003"},
    {"id": "beech", "title": T("European beech cleats", "Leisten aus Rotbuche"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 720, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 14000000000.0, "strengthPa": 40000000.0, "source": "literature", "reference": "din-68364-2003"},
    {"id": "steel-zinc", "title": T("Zinc-plated steel hinge bolts, pins and straps, dulled", "Verzinkter Stahl der Gelenkschrauben, Stifte und Laschen, matt geworden"), "emissivity": 0.25, "solarAbsorptivity": 0.55, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "felt-blanket", "title": T("Moving blanket of needled cotton-polyester felt, dark blue", "Möbeldecke aus vernadeltem Baumwoll-Polyester-Filz, dunkelblau"), "emissivity": 0.88, "solarAbsorptivity": 0.7, "densityKgM3": 110, "conductivityWmK": 0.045, "specificHeatJKgK": 1340, "source": "authored-estimate", "reference": "moving-blanket-typical"},
    {"id": "pine-raw", "title": T("Untreated pine offcut of the wall-anchor block", "Unbehandeltes Kiefernreststück des Wandankerklotzes"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 500, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "source": "literature", "reference": "din-68364-2003"},
]


def _layer(id_, title, kind, material, thickness, conductivity, density, cp, source="literature", reference="iso-10456-2007", declare=True):
    layer = {"id": id_, "title": title, "kind": kind, "material": material, "resistanceM2KW": round(thickness / conductivity, 4), "heatCapacityJM2K": round(thickness * density * cp), "source": source, "reference": reference}
    if declare:
        layer["thicknessM"] = thickness
    return layer


REGIONS = [
    {"id": "front-rails", "title": T("Front rails with the hinge bolts", "Vorderholme mit den Gelenkschrauben"), "kind": "surface", "model": "passive", "material": "pine-aged", "characteristicLengthM": 0.044, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [_layer("rail-timber", T("Pine rail, half of the 28 mm section", "Kiefernholm, halber 28-mm-Querschnitt"), "timber", "pine-aged", 0.014, 0.13, 500, 1600, declare=False)], "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "rear-rails", "title": T("Rear rails with the cleats and the wall-anchor block", "Hinterholme mit den Leisten und dem Wandankerklotz"), "kind": "surface", "model": "passive", "material": "pine-aged", "characteristicLengthM": 0.034, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [_layer("rear-timber", T("Pine rear rail, half of the 28 mm section", "Kiefern-Hinterholm, halber 28-mm-Querschnitt"), "timber", "pine-aged", 0.014, 0.13, 500, 1600, declare=False)], "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "shelves", "title": T("Five shelves", "Fünf Böden"), "kind": "surface", "model": "passive", "material": "pine-board", "characteristicLengthM": 0.2, "geometry": "horizontal-plate-up", "skyViewFactor": 0.35, "layers": [_layer("shelf-board", T("Pine panel 18 mm", "Kiefernplatte 18 mm"), "timber", "pine-board", 0.018, 0.12, 480, 1600, declare=False)], "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "blanket", "title": T("Folded moving blanket", "Gefaltete Möbeldecke"), "kind": "surface", "model": "passive", "material": "felt-blanket", "characteristicLengthM": 0.5, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [_layer("blanket-felt", T("Eight layers of 2.5 mm felt", "Acht Lagen Filz zu 2,5 mm"), "polymer", "felt-blanket", 0.02, 0.045, 110, 1340, source="authored-estimate", reference="moving-blanket-typical")], "source": "authored-estimate", "reference": "moving-blanket-typical"},
]

THERMAL_ASSUMPTIONS = [
    T("Passive indoor furniture: solid pine 500 kg/m3, 0.13 W/mK, 1600 J/kgK and beech 720 kg/m3, 0.18 W/mK (ISO 10456 / DIN 68364); heat capacity per area from the solid volume of each region over its outer area.", "Passives Innenmöbel: massive Kiefer 500 kg/m3, 0,13 W/mK, 1600 J/kgK und Buche 720 kg/m3, 0,18 W/mK (ISO 10456 / DIN 68364); flächenbezogene Wärmekapazität aus dem Holzvolumen jeder Region über ihrer Außenfläche."),
    T("Aged surfaces: the yellowed lacquer of the rails absorbs about 0.55 of the sun instead of about 0.45 when new, the sun-faded oiled shelves 0.45; emissivity 0.90 for wood either way (Incropera 2018 tables; the ageing shift is an authored estimate). Dulled zinc plating 0.25 instead of 0.05 to 0.1 when bright.", "Gealterte Oberflächen: der vergilbte Lack der Holme absorbiert etwa 0,55 der Sonne statt etwa 0,45 im Neuzustand, die sonnengebleichten geölten Böden 0,45; Emissionsgrad 0,90 für Holz in beiden Fällen (Tabellen nach Incropera 2018; die Verschiebung durch Alterung ist ein Schätzwert). Matt gewordene Verzinkung 0,25 statt 0,05 bis 0,1 im blanken Zustand."),
    T("The moving blanket is needled cotton-polyester felt (110 kg/m3, 0.045 W/mK, absorptance 0.7 for dark blue), an authored estimate from catalogue-typical moving blankets; the steel pins, bolts and straps (under 0.5 % of the area) belong to no region.", "Die Möbeldecke ist vernadelter Baumwoll-Polyester-Filz (110 kg/m3, 0,045 W/mK, Absorptionsgrad 0,7 für Dunkelblau), ein Schätzwert nach katalogtypischen Möbeldecken; Stahlstifte, Schrauben und Laschen (unter 0,5 % der Fläche) gehören zu keiner Region."),
    T("The shelf draws no power: the region front-rails states 0 W with basis passive.", "Das Regal nimmt keine Leistung auf: die Region front-rails gibt 0 W mit der Grundlage passiv an."),
]

STRUCTURE_ASSUMPTIONS = [
    T("Rigid A-frame: the front ladder of two pine rails 44 x 28 mm (C24-like, E 11 GPa, fm 24 MPa per EN 338) leans at 8.8 degrees and meets the upright rear rails 34 x 28 mm at two M8 hinge bolts 1.76 m up; the wall and the anchor block keep the rear feet from sliding back.", "Starres A-Gestell: die Vorderleiter aus zwei Kiefernholmen 44 x 28 mm (etwa C24, E 11 GPa, fm 24 MPa nach EN 338) lehnt unter 8,8 Grad und trifft an zwei M8-Gelenkschrauben auf 1,76 m die stehenden Hinterholme 34 x 28 mm; Wand und Ankerklotz hindern die Hinterfüße am Zurückgleiten."),
    T("Each shelf hangs on two 8 mm steel pins in the front rails and rests at its rear corners on two beech cleats 20 x 32 mm of the rear rails, so it spans 0.536 m between its two ends: books of 22 kg on the bottom shelf give M = q L / 8 = 14.5 Nm, a bending stress of 0.9 MPa in the 298 x 18 mm board and a sag of 0.3 mm (E 10 GPa).", "Jeder Boden hängt an zwei 8-mm-Stahlstiften in den Vorderholmen und liegt hinten an den Ecken auf zwei Buchenleisten 20 x 32 mm der Hinterholme, er trägt also 0,536 m zwischen seinen Enden: 22 kg Bücher auf dem untersten Boden ergeben M = q L / 8 = 14,5 Nm, eine Biegespannung von 0,9 MPa im Brett 298 x 18 mm und eine Durchbiegung von 0,3 mm (E 10 GPa)."),
    T("Book load from EN 1991-1-1 Annex A (paper and books stacked, about 6 kN/m3) for 0.25 m deep and 0.30 m high rows on the lower shelves; potted plants with watered soil 1.5 to 3 kg; 52 kg in all, well within the 1.5 kg/dm2 shelf load of the EN 14749 / EN 16122 storage tests for the two lower shelves (about 24 and 19 kg).", "Bücherlast nach EN 1991-1-1 Anhang A (Papier und Bücher gestapelt, etwa 6 kN/m3) für 0,25 m tiefe und 0,30 m hohe Reihen auf den unteren Böden; Topfpflanzen mit gegossener Erde 1,5 bis 3 kg; insgesamt 52 kg, deutlich innerhalb der Fachbodenlast von 1,5 kg/dm2 der Prüfungen nach EN 14749 / EN 16122 für die zwei unteren Böden (etwa 24 und 19 kg)."),
    T("Tipping (EN 14749): empty, the centre of mass lies 0.24 m behind the front edge of the front feet at 0.84 m height, so a forward pull at the top shelf (1.52 m) tips the shelf from about 14 N; it is therefore screwed through the anchor block to the wall, a roller support across the wall that the pull-test pose loads with 50 N at the left end of the top shelf.", "Kippen (EN 14749): leer liegt der Schwerpunkt 0,24 m hinter der Vorderkante der Vorderfüße auf 0,84 m Höhe, ein Zug nach vorn am obersten Boden (1,52 m) kippt das Regal also schon ab etwa 14 N; es ist deshalb über den Ankerklotz an die Wand geschraubt, ein Rollenlager quer zur Wand, das die Pose Zugprobe mit 50 N am linken Ende des obersten Bodens belastet."),
    T("Storage aids are not part of the carried mass: the moving blanket (about 1.4 kg) and the anchor block (20 g) rest on the floor in every pose and carry no bone mass, so the hands in the carried pose hold exactly the folded shelf; the stored package stands on the blanket at its two front feet and leans with the tops of the rear rails on the depot wall.", "Lagerhilfen gehören nicht zur getragenen Masse: die Möbeldecke (etwa 1,4 kg) und der Ankerklotz (20 g) liegen in jeder Pose am Boden und tragen keine Knochenmasse, die Hände in der Trage-Pose halten also genau das zusammengeklappte Regal; das gelagerte Paket steht mit seinen zwei Vorderfüßen auf der Decke und lehnt mit den Köpfen der Hinterholme an der Lagerwand."),
    T("Masses from the section of every 1:50 solid times its length and density (pine 500, glued pine panel 480, beech 720, steel 7850 kg/m3).", "Massen aus dem Querschnitt jedes 1:50-Körpers mal Länge und Dichte (Kiefer 500, Kiefern-Leimholz 480, Buche 720, Stahl 7850 kg/m3)."),
]

PROVENANCE = {
    "author": "Semio staffage content (used household objects wave, row 20)",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [
        {"id": "ladder-shelf-typical", "citation": "Catalogue-typical folding A-frame ladder shelf (Leiterregal, klappbar) of solid pine sold by furniture stores and mail-order houses since the 2000s: about 60 x 36 x 180 cm, five graduated shelves of 18 mm glued pine panel, 8 to 12 kg, supplied with an anti-tip wall fixing; types, no brand."},
        {"id": "moving-blanket-typical", "citation": "Catalogue-typical moving blanket (Möbeldecke, Packdecke) of needled cotton-polyester felt, 180 x 200 cm, about 1.3 to 1.5 kg, 2 to 3 mm; folded here three times to 0.90 x 0.50 m in eight layers (20 mm)."},
        {"id": "din-68364-2003", "citation": "DIN 68364:2003-05 Kennwerte von Holzarten - Rohdichte, Elastizitätsmodul und Festigkeiten (Scots pine Pinus sylvestris mean density about 500 to 520 kg/m3 at 12 % moisture, modulus of elasticity about 11 GPa; European beech about 720 kg/m3, 14 GPa)."},
        {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24: characteristic bending strength 24 MPa, mean modulus of elasticity parallel to grain 11 GPa)."},
        {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (softwood 500 kg/m3: 0.13 W/mK, 1600 J/kgK; hardwood 700 kg/m3: 0.18 W/mK; steel 50 W/mK, 450 J/kgK)."},
        {"id": "en-1991-1-1-2002", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (Annex A: densities of stored materials, paper and books)."},
        {"id": "en-14749-2016", "citation": "EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (stability against overturning, shelf load tests, fixing to the building structure)."},
        {"id": "en-16122-2012", "citation": "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (shelf deflection and strength tests, overturning tests)."},
        {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total emissivity of wood, fabrics and zinc-plated steel; solar absorptance of surfaces)."},
    ],
    "notes": T(
        "Folding A-frame ladder shelf as a flat clearance or second-hand department store offers it, dimensioned to the catalogue-typical type (ladder-shelf-typical): front rails 44 x 28 mm cut level at the floor and rounded about the M8 hinge bolts at 1.76 m, rear rails 34 x 28 mm 1.80 m high standing against the wall, the front feet 0.352 m in front of it (8.8 degrees lean). Shelves of 18 mm glued pine panel 536 mm wide hinge on 8 mm pins in the front rails and rest on beech cleats; their tops at 0.26, 0.62, 0.92, 1.22 and 1.52 m and their depths 298, 242, 195, 149 and 102 mm follow from the lean (rear edges 5 mm from the wall). Folded, the rear frame lies behind the front rails (78 mm package) and the shelves lie between them with 62 to 155 mm between their ends. Traces of use: feet scuffed grey for 35 mm, a 15 x 10 mm chip broken out of the front corner of shelf 2 over 25 mm, the right hinge bolt replaced by a hex bolt with a 26 mm washer, a price sticker at 1.1 m on the right rail, the upper shelves faded by the sun and 4 mm hinge play. Books and plants are loads, not drawn, so that the folded and the loaded states differ only physically.",
        "Klappbares A-Leiterregal, wie eine Haushaltsauflösung oder ein Gebrauchtwarenkaufhaus es anbietet, bemaßt nach dem katalogtypischen Typ (ladder-shelf-typical): Vorderholme 44 x 28 mm, am Fuß bodenparallel geschnitten und oben um die M8-Gelenkschrauben auf 1,76 m gerundet, Hinterholme 34 x 28 mm, 1,80 m hoch, an der Wand stehend, die Vorderfüße 0,352 m davor (8,8 Grad Neigung). Böden aus 18 mm Kiefern-Leimholz, 536 mm breit, hängen an 8-mm-Stiften in den Vorderholmen und liegen auf Buchenleisten; ihre Oberkanten auf 0,26, 0,62, 0,92, 1,22 und 1,52 m und ihre Tiefen von 298, 242, 195, 149 und 102 mm folgen aus der Neigung (Hinterkanten 5 mm vor der Wand). Zusammengeklappt liegt der Hinterrahmen hinter den Vorderholmen (78 mm Paket) und die Böden zwischen ihnen mit 62 bis 155 mm zwischen ihren Enden. Gebrauchsspuren: Füße auf 35 mm grau abgestoßen, an der Vorderecke von Boden 2 auf 25 mm ein 15 x 10 mm großes Stück ausgebrochen, die rechte Gelenkschraube durch eine Sechskantschraube mit 26-mm-Scheibe ersetzt, ein Preisaufkleber auf 1,1 m am rechten Holm, die oberen Böden sonnengebleicht und 4 mm Spiel in den Gelenken. Bücher und Pflanzen sind Lasten und nicht gezeichnet, damit sich zusammengeklappter und beladener Zustand nur physikalisch unterscheiden.",
    ),
}


def _dec(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles, "overlays": overlays}


REP_DECISIONS = {
    50: _dec(
        T("Default pose stored: side elevation 13 x 36 mm of the folded 78 mm package leaning on the depot wall, the front rail with its rounded head over the hinge, the rear rail lying behind it 12 mm clear of the blanket, the 20 mm blanket and the anchor block on it. Front view: two rails 28 mm wide, the five folded shelves between them with their gaps, the bolt heads at the top and the blanket 0.90 m wide. Plan: the rails and shelf edges seen end-on over the blanket with its quilting.", "Standardpose gelagert: Seitenansicht 13 x 36 mm des zusammengeklappten 78-mm-Pakets an der Lagerwand, der Vorderholm mit gerundetem Kopf über dem Gelenk, dahinter der Hinterholm 12 mm über der Decke, die 20-mm-Decke und der Ankerklotz darauf. Vorderansicht: zwei 28 mm breite Holme, dazwischen die fünf hochgeklappten Böden mit ihren Lücken, oben die Schraubenköpfe, unten die 0,90 m breite Decke. Draufsicht: Holme und Bodenkanten von oben über der Decke mit ihren Steppnähten."),
        T("Every part at its true size: rail sections, the 18 mm boards with eased front edges, the 4 mm play of the shelves between the rails, the ten beech cleats, the M8 hinge bolts, the 20 mm blanket with rounded folds and the 22 mm anchor block.", "Jedes Teil in wahrer Größe: Holmquerschnitte, die 18-mm-Bretter mit gebrochenen Vorderkanten, 4 mm Spiel der Böden zwischen den Holmen, die zehn Buchenleisten, die M8-Gelenkschrauben, die 20-mm-Decke mit gerundeten Falten und der 22-mm-Ankerklotz."),
        T("Nothing merged; the left and right rails share one elevation outline because they coincide in that view.", "Nichts zusammengefasst; linker und rechter Holm teilen in der Seitenansicht eine Kontur, weil sie dort zusammenfallen."),
        T("The packing tape around the folded package (0.05 mm), the screw holes, grain and the water rings of the plant pots (described, not drawn).", "Das Packband um das gefaltete Paket (0,05 mm), Schraubenlöcher, Maserung und die Wasserringe der Blumentöpfe (beschrieben, nicht gezeichnet)."),
        T("The 4 mm steel straps of the hinges are a detail outline over the rails; the 8 mm pins sit inside rails and shelves and are not drawn; the 4 mm hinge play reads as a joined line.", "Die 4-mm-Stahllaschen der Gelenke sind eine Detailkontur über den Holmen; die 8-mm-Stifte stecken in Holmen und Böden und sind nicht gezeichnet; das Gelenkspiel von 4 mm liest sich als geschlossene Linie."),
        T("No hatch: wood is shown by tone; the blanket by its quilting seams in plan.", "Keine Schraffur: Holz wird durch Ton gezeigt, die Decke im Grundriss durch ihre Steppnähte."),
        T("Rails 0.62 (front) and 0.5 (rear) with 0.35 mm outlines, shelves 0.72 to 0.84 lighter towards the sun-faded top with 0.25 mm, cleats 0.42, blanket 0.3, steel 0.2 to 0.3; the grey scuffed feet 0.35 and the price sticker as texture.", "Holme 0,62 (vorn) und 0,5 (hinten) mit 0,35-mm-Kontur, Böden 0,72 bis 0,84, zum sonnengebleichten oberen Ende heller, mit 0,25 mm, Leisten 0,42, Decke 0,3, Stahl 0,2 bis 0,3; die grau abgestoßenen Füße 0,35 und der Preisaufkleber als Textur."),
        T("One shape per part bound to its bone and region; front and plan are lateral extrusions of the elevation profiles, so the three views agree in every pose and clip instant.", "Eine Form je Teil an Knochen und Region gebunden; Vorderansicht und Draufsicht sind seitliche Extrusionen der Seitenprofile, so stimmen die drei Ansichten in jeder Pose und jedem Clipzeitpunkt überein."),
    ),
    100: _dec(
        T("The folded package on the blanket 6.6 x 18 mm: the two rails side by side, the shelves between the rails, the blanket and the anchor block; front and plan as at 1:50 without the hardware.", "Das zusammengeklappte Paket auf der Decke 6,6 x 18 mm: die zwei Holme nebeneinander, die Böden zwischen den Holmen, Decke und Ankerklotz; Vorderansicht und Draufsicht wie 1:50 ohne Beschläge."),
        T("True rail sections and 18 mm boards, the cleats, the blanket with its rounded folds, the anchor block.", "Wahre Holmquerschnitte und 18-mm-Bretter, die Leisten, die Decke mit gerundeten Falten, der Ankerklotz."),
        T("The 4 mm play of the shelves is closed: the boards reach the rails (0.04 mm on paper).", "Das Spiel der Böden von 4 mm ist geschlossen: die Bretter reichen bis an die Holme (0,04 mm auf dem Papier)."),
        T("Pins, bolt heads, hinge straps, the chipped corner (15 mm), the scuff marks, the sticker and the quilting.", "Stifte, Schraubenköpfe, Gelenklaschen, die abgeplatzte Ecke (15 mm), die Abriebspuren, der Aufkleber und die Steppnähte."),
        T("Rails 0.28 to 0.44 mm, shelves 0.18 mm thick, the blanket 0.2 mm: all drawn true with their outlines.", "Holme 0,28 bis 0,44 mm, Böden 0,18 mm dick, die Decke 0,2 mm: alle wahr mit ihren Konturen gezeichnet."),
        T("None.", "Keine."),
        T("Same tones as 1:50; rails 0.25 mm, shelves 0.18 mm, cleats 0.13 mm outlines.", "Gleiche Töne wie 1:50; Holme 0,25 mm, Böden 0,18 mm, Leisten 0,13 mm Kontur."),
        T("One shape per part; front and plan as extrusions of the elevation profiles.", "Eine Form je Teil; Vorderansicht und Draufsicht als Extrusionen der Seitenprofile."),
    ),
    200: _dec(
        T("The folded package as a 0.4 mm band 9 mm high on the blanket line; front view 3 x 9 mm: two rails and the five shelf panels between them; plan: rails and shelf edges over the blanket.", "Das zusammengeklappte Paket als 0,4-mm-Band, 9 mm hoch auf der Deckenlinie; Vorderansicht 3 x 9 mm: zwei Holme und die fünf Bodenplatten dazwischen; Draufsicht: Holme und Bodenkanten über der Decke."),
        T("Rail sections true (slender bars), the shelf positions and depths, the blanket at its true 20 mm as a declared sheet.", "Holmquerschnitte wahr (schlanke Stäbe), Lage und Tiefe der Böden, die Decke mit wahren 20 mm als erklärte Platte."),
        T("The shelves are thickened from 18 to 30 mm (the smallest feature, 0.15 mm), the cleats are merged into the bearing of the shelves on the rear rails.", "Die Böden sind von 18 auf 30 mm verdickt (kleinstes Merkmal, 0,15 mm), die Leisten gehen im Auflager der Böden an den Hinterholmen auf."),
        T("The anchor block (22 mm, under the feature in two directions), the cleats, all hardware and traces of use.", "Der Ankerklotz (22 mm, in zwei Richtungen unter dem Merkmal), die Leisten, alle Beschläge und Gebrauchsspuren."),
        T("Front view rails 0.14 mm with 0.18 mm outlines, blanket 0.1 mm with 0.18 mm outline.", "Holme in der Vorderansicht 0,14 mm mit 0,18-mm-Kontur, Decke 0,1 mm mit 0,18-mm-Kontur."),
        T("None.", "Keine."),
        T("Rails 0.18 mm, shelves 0.13 mm so that the 62 mm gaps of the folded shelves stay open on paper.", "Holme 0,18 mm, Böden 0,13 mm, damit die 62-mm-Lücken der hochgeklappten Böden auf dem Papier offen bleiben."),
        T("One shape per part; front and plan as extrusions.", "Eine Form je Teil; Vorderansicht und Draufsicht als Extrusionen."),
    ),
    500: _dec(
        T("A 3.6 mm stroke of the leaning package on the blanket line; front view 1.2 x 3.6 mm with the rails and the shelf panels joined into the ladder; plan a short bar over the blanket.", "Ein 3,6-mm-Strich des angelehnten Pakets auf der Deckenlinie; Vorderansicht 1,2 x 3,6 mm mit Holmen und Bodenplatten zur Leiter verbunden; Draufsicht ein kurzer Balken über der Decke."),
        T("Rails, rear rails and the five shelves as separate parts so that the shelves still unfold in use; the blanket.", "Holme, Hinterholme und die fünf Böden als eigene Teile, damit die Böden in Gebrauch noch herunterklappen; die Decke."),
        T("The shelves are 75 mm thick (the feature, 0.15 mm); their folded gaps close under the 0.18 mm pens.", "Die Böden sind 75 mm dick (das Merkmal, 0,15 mm); ihre Lücken im zusammengeklappten Zustand schließen sich unter den 0,18-mm-Stiften."),
        T("The anchor block, cleats, hardware and traces of use.", "Ankerklotz, Leisten, Beschläge und Gebrauchsspuren."),
        T("Rails 0.06 to 0.09 mm wide drawn with 0.18 mm outlines (slender bars at their true section), the blanket 0.04 mm with its outline.", "Holme 0,06 bis 0,09 mm breit mit 0,18-mm-Kontur gezeichnet (schlanke Stäbe in wahrem Querschnitt), die Decke 0,04 mm mit ihrer Kontur."),
        T("None.", "Keine."),
        T("All parts 0.18 mm outlines with the tones of 1:50.", "Alle Teile mit 0,18-mm-Kontur und den Tönen von 1:50."),
        T("One shape per part; front and plan as extrusions.", "Eine Form je Teil; Vorderansicht und Draufsicht als Extrusionen."),
    ),
}

VOL_DECISIONS = {
    50: [
        T("Body 1:50 is the shelf as built and worn: two front rails 44 x 28 mm cut level at the floor and rounded about the hinge, two rear rails 34 x 28 mm with ten beech cleats 34 x 20 x 32 mm, five 18 mm shelves 536 mm wide with eased front edges (shelf 2 in two pieces for its 15 x 10 mm chip), twenty-four 8 mm steel pins in the hinge lines, the 4 mm hinge straps as texture, the original dome bolt on the left and the replaced hex bolt with washer on the right, the folded blanket 0.50 x 0.90 m x 20 mm with rounded folds and the 60 x 28 x 22 mm anchor block.", "Körper 1:50 ist das Regal wie gebaut und abgenutzt: zwei Vorderholme 44 x 28 mm, am Fuß bodenparallel und um das Gelenk gerundet, zwei Hinterholme 34 x 28 mm mit zehn Buchenleisten 34 x 20 x 32 mm, fünf 18-mm-Böden 536 mm breit mit gebrochenen Vorderkanten (Boden 2 wegen seiner 15 x 10 mm großen Abplatzung zweiteilig), 8-mm-Stahlstifte in den Gelenklinien, die 4-mm-Gelenklaschen als Textur, links die ursprüngliche Flachrundschraube, rechts die ersetzte Sechskantschraube mit Scheibe, die gefaltete Decke 0,50 x 0,90 m x 20 mm mit gerundeten Falten und der Ankerklotz 60 x 28 x 22 mm."),
        T("Thermal 1:50 references the rails, cleats, shelves, blanket and anchor block of their four regions (the pine anchor block belongs to the rear rails); steel parts belong to no region. Structure 1:50: the rails are the members of ladder and rear frame, each shelf its own member, the hinge axis and the five pin lines are slender joint rods, the two frames carry mass markers; no permanent support, the feet, wall contacts, grips and the wall fixing act per pose.", "Thermik 1:50 verweist auf Holme, Leisten, Böden, Decke und Ankerklotz ihrer vier Regionen (der Kiefern-Ankerklotz gehört zu den Hinterholmen); Stahlteile gehören zu keiner Region. Tragwerk 1:50: die Holme sind die Stäbe von Leiter und Hinterrahmen, jeder Boden ein eigener Stab, Gelenkachse und die fünf Stiftlinien sind schlanke Gelenkstäbe, die zwei Rahmen tragen Massenmarken; kein dauerndes Auflager, Füße, Wandkontakte, Griffe und Wandbefestigung wirken je Pose."),
    ],
    100: [
        T("Body 1:100: the same rails with a coarser rounding of the head (8 segments), the cleats and the anchor block; the shelves are plain 18 mm boards that reach the rails (the 4 mm play would lie between a quarter and a half feature), the chip, pins, bolts and straps are left out, the blanket keeps its rounded folds.", "Körper 1:100: dieselben Holme mit gröberer Rundung des Kopfs (8 Segmente), die Leisten und der Ankerklotz; die Böden sind einfache 18-mm-Bretter, die bis an die Holme reichen (das Spiel von 4 mm läge zwischen einem Viertel und einem halben Merkmal), Abplatzung, Stifte, Schrauben und Laschen entfallen, die Decke behält ihre gerundeten Falten."),
        T("Thermal 1:100 references the same four regions; structure 1:100 keeps members, hinge and pin rods and the two mass markers.", "Thermik 1:100 verweist auf dieselben vier Regionen; Tragwerk 1:100 behält Stäbe, Gelenk- und Stiftstäbe und die zwei Massenmarken."),
    ],
    200: [
        T("Body 1:200: rails at their true section declared slender, the head rounded in four segments and the rear rails square; the shelves are 30 mm thick (the feature) between the rails; cleats and the 22 mm anchor block are below the feature in two directions and left out; the blanket keeps 20 mm as a declared sheet.", "Körper 1:200: Holme in wahrem Querschnitt als schlank erklärt, der Kopf in vier Segmenten gerundet, die Hinterholme rechteckig; die Böden sind 30 mm dick (das Merkmal) zwischen den Holmen; Leisten und der 22-mm-Ankerklotz liegen in zwei Richtungen unter dem Merkmal und entfallen; die Decke behält 20 mm als erklärte Platte."),
        T("Thermal 1:200 references rails, shelves and blanket; the anchor block, part of the rear-rails region, is left out at this scale. Structure as at 1:100 with larger mass markers.", "Thermik 1:200 verweist auf Holme, Böden und Decke; der Ankerklotz, Teil der Region rear-rails, entfällt in diesem Maßstab. Tragwerk wie 1:100 mit größeren Massenmarken."),
    ],
    500: [
        T("Body 1:500: slender rails at their true section with a two-segment head, shelves 75 mm thick between the rails, the blanket at 20 mm as a declared sheet; nothing else is above the 75 mm feature.", "Körper 1:500: schlanke Holme in wahrem Querschnitt mit zweisegmentigem Kopf, Böden 75 mm dick zwischen den Holmen, die Decke mit 20 mm als erklärte Platte; sonst liegt nichts über dem Merkmal von 75 mm."),
        T("Thermal 1:500 references rails, shelves and blanket; structure keeps members, the slender hinge and pin rods and the two mass markers.", "Thermik 1:500 verweist auf Holme, Böden und Decke; das Tragwerk behält Stäbe, die schlanken Gelenk- und Stiftstäbe und die zwei Massenmarken."),
    ],
}
