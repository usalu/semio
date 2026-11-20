"""📝 Round 3 roe deer: the drawing decisions of every scale, rewritten for the redrawn 2D (true orthographic views of the scale's body); only the members named here change, the rest of each `decisions` stays.

Usage: python3 r3-roe-deer-texts.py   (patches representations.<scale>.decisions of the work document)
"""
import importlib.util

spec = importlib.util.spec_from_file_location("io", "r3-roe-deer-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)

TAIL_EN = " Every filled shape is the orthographic outline of the body solids of its bone at this scale; the neck is one extruded solid, so front and plan are its projections in every pose; limb shapes keep followTip and stretch along a swinging limb in front and plan."
TAIL_DE = " Jede gefüllte Form ist der orthografische Umriss der Körper ihres Knochens in diesem Maßstab; der Hals ist ein extrudierter Körper, sodass Vorderansicht und Draufsicht in jeder Pose seine Projektionen sind; die Formen der Läufe behalten followTip und strecken sich in Vorderansicht und Draufsicht entlang eines schwingenden Laufs."
TEXT = {
    "50": {
        "silhouette": ("Full articulation at 20.7 × 22.4 mm: the roe buck with croup above the withers, tucked flank and deep chest; croup, loin and chest as three trunk shapes, thigh and gaskin, hock with its point, cannon, pastern and cloven hoof, shoulder, upper arm, forearm and fore cannon, long neck, head with separate jaw, nose leather, eye, two leaf ears and the six-point antlers; brow, mouth line, inner ear and three coat strokes. Front view: the buck head-on with both antlers, ears, eyes and nose leather, the narrow chest between the shoulders and all eight leg chains. Plan view: croup, loin and chest, the shoulders and thighs beside the trunk, neck, head wedge, ears and antlers; the white rump patch shows as the crescent behind the croup." + TAIL_EN,
                       "Volle Gliederung bei 20,7 × 22,4 mm: der Rehbock mit Kruppe über dem Widerrist, aufgezogener Flanke und tiefer Brust; Kruppe, Lende und Brust als drei Rumpfformen, Keule und Unterschenkel, Sprunggelenk mit Fersenhöcker, Röhre, Fessel und gespaltene Schale, Schulter, Oberarm, Unterarm und vordere Röhre, langer Hals, Kopf mit eigenem Unterkiefer, Nasenspiegel, Auge, zwei Blattlauscher und das Sechsergehörn; Braue, Maullinie, Innenohr und drei Fellstriche. Vorderansicht: der Bock von vorn mit beiden Stangen, Lauschern, Augen und Nasenspiegel, der schmalen Brust zwischen den Schultern und allen acht Laufketten. Draufsicht: Kruppe, Lende und Brust, Schultern und Keulen neben dem Rumpf, Hals, Kopfkeil, Lauscher und Gehörn; der weiße Spiegel erscheint als Sichel hinter der Kruppe." + TAIL_DE),
        "retained": ("Antlers with burr, brow tine and back tine, dewclaws, both claws of every hoof, inner ear, brow, mouth line, three coat strokes.", "Gehörn mit Rose, Vorder- und Hintersprosse, Afterklauen, beide Klauen jeder Schale, Innenohr, Braue, Maullinie, drei Fellstriche."),
        "omitted": ("Individual hairs, lashes, the tail (2 to 3 cm, hidden in the rump patch); the hoof cleft is not drawn in the side elevation because it cannot be seen from the side, the eyes not in the plan.", "Einzelne Haare, Wimpern, der Wedel (2 bis 3 cm, im Spiegel verborgen); der Schalenspalt wird in der Seitenansicht nicht gezeichnet, weil er von der Seite nicht sichtbar ist, die Augen nicht in der Draufsicht."),
        "thinElements": ("Cannons 0.019 to 0.022 m = 0.38 to 0.44 mm at true width; antler beam 0.016 to 0.02 m = 0.32 to 0.4 mm, tines down to 0.17 mm at their tips.", "Röhrbeine 0,019 bis 0,022 m = 0,38 bis 0,44 mm in wahrer Breite; Stange 0,016 bis 0,02 m = 0,32 bis 0,4 mm, Sprossen bis 0,17 mm an den Spitzen."),
    },
    "100": {
        "silhouette": ("Articulated outline of 10.4 × 11.2 mm: every segment of the buck at true width, croup, loin and chest, the white rump patch, ears, jaw, nose leather, eye, antlers as beam with two tines and one hoof per foot. Front view: head-on with antlers, ears, eyes and all leg chains. Plan view: the three trunk shapes, shoulders and thighs, neck, head wedge, ears and antlers." + TAIL_EN,
                       "Gegliederter Umriss von 10,4 × 11,2 mm: jedes Segment des Bocks in wahrer Breite, Kruppe, Lende und Brust, der weiße Spiegel, Lauscher, Unterkiefer, Nasenspiegel, Auge, Gehörn als Stange mit zwei Sprossen und eine Schale je Fuß. Vorderansicht: von vorn mit Gehörn, Lauschern, Augen und allen Laufketten. Draufsicht: die drei Rumpfformen, Schultern und Keulen, Hals, Kopfkeil, Lauscher und Gehörn." + TAIL_DE),
        "retained": ("Eye, nose leather, antlers, hock point, shoulder and hip lines.", "Auge, Nasenspiegel, Gehörn, Fersenhöcker, Schulter- und Hüftlinie."),
        "omitted": ("Dewclaws, the cleft between the claws, brow, mouth line, coat strokes, the eyes in the plan.", "Afterklauen, der Spalt zwischen den Klauen, Braue, Maullinie, Fellstriche, die Augen in der Draufsicht."),
        "thinElements": ("Cannons 0.02 to 0.022 m = 0.2 to 0.22 mm at true width, antler beam and tines 0.017 m = 0.17 mm at the size floor of the model.", "Röhrbeine 0,02 bis 0,022 m = 0,2 bis 0,22 mm in wahrer Breite, Stange und Sprossen 0,017 m = 0,17 mm an der Größenuntergrenze des Modells."),
    },
    "200": {
        "silhouette": ("Main masses of 5.2 × 5.6 mm: croup, loin, chest, neck, head with jaw, both ears, an antler spike pair, the white rump patch and all legs in segments with one rounded hoof form. Front view: the same masses head-on. Plan view: the three trunk shapes, shoulders and thighs, neck, head wedge, ears and antler spikes." + TAIL_EN,
                       "Hauptmassen von 5,2 × 5,6 mm: Kruppe, Lende, Brust, Hals, Kopf mit Unterkiefer, beide Lauscher, ein Spießerpaar als Gehörn, der weiße Spiegel und alle Läufe in Segmenten mit einer gerundeten Schalenform. Vorderansicht: dieselben Massen von vorn. Draufsicht: die drei Rumpfformen, Schultern und Keulen, Hals, Kopfkeil, Lauscher und Spieße." + TAIL_DE),
        "retained": ("Rump patch, both ears, antler spikes, hooves, shoulder and hip lines.", "Spiegel, beide Lauscher, Spieße, Schalen, Schulter- und Hüftlinie."),
        "merged": ("Nose leather and eyes into the head mass; the tines into one antler spike; pastern and hoof drawn as one shape.", "Nasenspiegel und Augen in die Kopfmasse; die Sprossen in einen Spieß; Fessel und Schale als eine Form gezeichnet."),
        "omitted": ("Eye, brow, mouth line, dewclaws, coat strokes.", "Auge, Braue, Maullinie, Afterklauen, Fellstriche."),
        "thinElements": ("Cannons, pasterns and antler spikes are drawn 0.033 m = 0.165 mm wide, the size floor of the model, against a real 0.02 m; they are the first to check against a print profile.", "Röhrbeine, Fesseln und Spieße sind 0,033 m = 0,165 mm breit gezeichnet, die Größenuntergrenze des Modells, bei real 0,02 m; sie sind als Erste gegen ein Druckprofil zu prüfen."),
        "styles": ("0.18 mm body outline, 0.1 mm detail lines; rump patch light.", "0,18-mm-Körperkontur, 0,1-mm-Detaillinien; Spiegel hell."),
        "overlays": ("One region per shape; nose leather and eyes are carried by the head mass.", "Eine Region je Form; Nasenspiegel und Augen werden von der Kopfmasse getragen."),
    },
    "500": {
        "silhouette": ("One composed silhouette of 2.1 × 2.2 mm that is still a roe buck: a loin-and-croup mass and a chest mass, a tapering neck, a head wedge with two ear stubs and two antler spikes, and per leg three filled segments (thigh, shank, cannon with foot; upper arm, forearm, cannon with foot) so that the legs bend at stifle, hock, elbow and carpus in every gait. Front view: the same masses head-on. Plan view: back and chest, thighs and upper arms beside them, neck, head wedge, ears and spikes." + TAIL_EN,
                       "Eine zusammengesetzte Silhouette von 2,1 × 2,2 mm, die noch ein Rehbock ist: eine Lenden-Kruppen-Masse und eine Brustmasse, ein sich verjüngender Hals, ein Kopfkeil mit zwei Lauscherstummeln und zwei Spießen und je Lauf drei gefüllte Segmente (Keule, Unterschenkel, Röhre mit Fuß; Oberarm, Unterarm, Röhre mit Fuß), sodass die Läufe in jeder Gangart an Knie, Sprunggelenk, Ellbogen und Vorderfußwurzel beugen. Vorderansicht: dieselben Massen von vorn. Draufsicht: Rücken und Brust, Keulen und Oberarme daneben, Hals, Kopfkeil, Lauscher und Spieße." + TAIL_DE),
        "retained": ("The slender build: croup above the withers, deep chest, long neck, long legs that bend, raised head with ears and antlers.", "Der schlanke Körperbau: Kruppe über dem Widerrist, tiefe Brust, langer Hals, lange beugende Läufe, erhobener Kopf mit Lauschern und Gehörn."),
        "merged": ("Croup into the loin mass, shoulder into the chest mass, jaw, eye and nose leather into the head, pastern and hoof into the cannon shape; the white rump patch into the hindquarter mass (listed as region).", "Kruppe in die Lendenmasse, Schulter in die Brustmasse, Unterkiefer, Auge und Nasenspiegel in den Kopf, Fessel und Schale in die Röhrenform; der weiße Spiegel in die Hinterhandmasse (als Region geführt)."),
        "omitted": ("Jaw, hoof outlines, tines, all detail lines.", "Unterkiefer, Schalenumrisse, Sprossen, alle Detaillinien."),
        "thinElements": ("Shanks, forearms, cannons, ears and antler spikes are filled shapes of 0.076 m = 0.15 mm, the size floor of the model, against real cannons of 0.02 m; this is accepted because the legs vanish otherwise.", "Unterschenkel, Unterarme, Röhren, Lauscher und Spieße sind gefüllte Formen von 0,076 m = 0,15 mm, die Größenuntergrenze des Modells, bei realen Röhren von 0,02 m; das wird akzeptiert, weil die Läufe sonst verschwinden."),
        "styles": ("Mid-grey masses with 0.13 mm outline; far legs darker.", "Mittelgraue Massen mit 0,13-mm-Kontur; ferne Läufe dunkler."),
    },
}
reps = io.work()["representations"]
for scale, texts in TEXT.items():
    for key, (en, de) in texts.items():
        assert key in reps[scale]["decisions"], (scale, key)
        reps[scale]["decisions"][key] = {"en": en, "de": de}
io.candidate({"representations": reps})
print("ok texts")
