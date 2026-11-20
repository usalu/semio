"""📝 Root fix of the dairy cow: the `decisions` of the four 2D representations described the old construction (ellipses, capsules, columns); they now say what the redrawn views are.

Usage: python3 r3-dairy-cow-decisions.py
"""
import copy

import r3_dairy_cow_io as io

TEXT = {
    "50": {
        "silhouette": ("The three views are the orthographic views of the 1:50 body in the standing pose, one outline per bone-bound mass: rump with loin, chest, neck with dewlap, head, jaw, ears, the udder with its teats, the tail in root, shaft and switch, and every leg in thigh or shoulder, gaskin or upper arm and forearm, cannon and cloven hoof. Saddle, croup and chest patch are drawn where they are the visible surface; in the front view the forelegs lie over the chest.", "Die drei Ansichten sind die orthografischen Ansichten des Körpers von 1:50 im Stand, ein Umriss je knochengebundener Masse: Kruppe mit Lende, Brust, Hals mit Triel, Kopf, Unterkiefer, Ohren, das Euter mit seinen Zitzen, der Schwanz in Wurzel, Rübe und Quaste und jedes Bein in Keule oder Schulter, Unterschenkel oder Oberarm und Unterarm, Röhrbein und Klaue. Sattel-, Kruppen- und Brustfleck sind dort gezeichnet, wo sie die sichtbare Oberfläche sind; in der Vorderansicht liegen die Vorderbeine über der Brust."),
        "retained": ("Coronet lines of the near hooves, knee line, nostril, mouth line, neck fold and two coat strokes, redrawn on the new forms.", "Kronsaumlinien der nahen Klauen, Linie der Vorderfußwurzel, Nüster, Maulspalte, Halsfalte und zwei Fellstriche, auf den neuen Formen neu gezeichnet."),
        "merged": ("Hook and pin bones and the sacral crest with the rump; brows, poll and blaze outline with the head in the side view; the rumen flank is drawn in the side view only.", "Hüft- und Sitzbeinhöcker und der Kreuzbeinkamm mit der Kruppe; Augenbögen, Genick und der Umriss der Blesse in der Seitenansicht mit dem Kopf; die Pansenflanke ist nur in der Seitenansicht gezeichnet."),
        "thinElements": ("Teats 0.028 m = 0.56 mm and the tail shaft 0.035 m = 0.7 mm at true size.", "Zitzen 0,028 m = 0,56 mm und die Schwanzrübe 0,035 m = 0,7 mm in wahrer Größe."),
        "styles": ("0.35 mm body, 0.18 mm detail, 0.1 mm texture; white coat light, black coat dark, far limbs a shade darker.", "0,35 mm Körper, 0,18 mm Detail, 0,1 mm Textur; weißes Fell hell, schwarzes Fell dunkel, ferne Gliedmaßen eine Stufe dunkler."),
    },
    "100": {
        "silhouette": ("The three views are the orthographic views of the 1:100 body: the same masses as at 1:50 with the joint knobs merged in, each cloven hoof one block, the udder one bag with teats, the three coat patches where they are visible.", "Die drei Ansichten sind die orthografischen Ansichten des Körpers von 1:100: dieselben Massen wie bei 1:50 mit eingeschmolzenen Gelenkhöckern, jede Klaue ein Block, das Euter ein Beutel mit Zitzen, die drei Fellflecken, wo sie sichtbar sind."),
        "retained": ("Teats, blaze, rumen flank, hip line.", "Zitzen, Blesse, Pansenflanke, Hüftlinie."),
        "thinElements": ("Teats 0.03 m = 0.3 mm and the tail shaft 0.035 m = 0.35 mm at true width.", "Zitzen 0,03 m = 0,3 mm und die Schwanzrübe 0,035 m = 0,35 mm in wahrer Breite."),
        "styles": ("0.25 mm body, 0.13 mm detail; white coat light, black coat dark.", "0,25 mm Körper, 0,13 mm Detail; weißes Fell hell, schwarzes Fell dunkel."),
    },
    "200": {
        "silhouette": ("The three views are the orthographic views of the 1:200 body: one mass per bone, hindquarters and chest as two eggs with the saddle and the croup patch on them, the udder as a bag, the tail in three pieces, every leg in its segments with pastern and hoof as one piece.", "Die drei Ansichten sind die orthografischen Ansichten des Körpers von 1:200: eine Masse je Knochen, Hinterhand und Brust als zwei Eier mit Sattel- und Kruppenfleck, das Euter als Beutel, der Schwanz in drei Stücken, jedes Bein in seinen Segmenten mit Fessel und Klaue als einem Stück."),
        "retained": ("Saddle and croup patch, the udder, the switch, ears and hooves.", "Sattel- und Kruppenfleck, das Euter, die Quaste, Ohren und Klauen."),
        "omitted": ("Eyes, muzzle, teats, the chest patch and all interior lines; their thermal regions stay on the fused masses.", "Augen, Flotzmaul, Zitzen, der Brustfleck und alle Innenlinien; ihre Wärmeregionen bleiben auf den verschmolzenen Massen."),
        "thinElements": ("Cannons at true width 0.07 m = 0.35 mm; tail shaft 0.035 m = 0.18 mm.", "Röhrbeine in wahrer Breite 0,07 m = 0,35 mm; Schwanzrübe 0,035 m = 0,18 mm."),
        "overlays": ("The two patches map to the coat-patches region, the pale trunk masses to coat-trunk and the rumen flank.", "Die zwei Flecken gehören zur Region der Fellflecken, die hellen Rumpfmassen zum Rumpffell und zur Pansenflanke."),
    },
    "500": {
        "silhouette": ("The three views are the orthographic views of the 1:500 body: hindquarters and chest as two eggs, neck and head as two wedges, thigh and shoulder masses, each lower leg in two tapered pieces that end in the hoof, the udder as a bag and the tail in three pieces. The ears are one stroke across the head in front view and plan. Black-and-white patches are not drawn.", "Die drei Ansichten sind die orthografischen Ansichten des Körpers von 1:500: Hinterhand und Brust als zwei Eier, Hals und Kopf als zwei Keile, Keule und Schultermasse, jedes Unterbein in zwei verjüngten Stücken, die in der Klaue enden, das Euter als Beutel und der Schwanz in drei Stücken. Die Ohren sind in Vorderansicht und Grundriss ein Strich quer über den Kopf. Schwarzweiße Flecken sind nicht gezeichnet."),
        "merged": ("Coat, patches and rumen flank into the trunk masses (three regions listed); ears, muzzle, jaw and eye into the head mass; pastern and hoof into the lower leg piece.", "Fell, Flecken und Pansenflanke in die Rumpfmassen (drei Regionen genannt); Ohren, Flotzmaul, Unterkiefer und Auge in die Kopfmasse; Fessel und Klaue in das untere Beinstück."),
        "thinElements": ("Gaskin, forearm and cannon are filled outlines at true width (0.07 to 0.12 m = 0.14 to 0.24 mm); the tail is 0.06 m = 0.12 mm.", "Unterschenkel, Unterarm und Röhrbein sind gefüllte Umrisse in wahrer Breite (0,07 bis 0,12 m = 0,14 bis 0,24 mm); der Schwanz ist 0,06 m = 0,12 mm."),
        "styles": ("Mid-grey trunk masses (white and black coat mixed) with a 0.13 mm outline; head and neck dark, legs light.", "Mittelgraue Rumpfmassen (weißes und schwarzes Fell gemischt) mit 0,13 mm Umriss; Kopf und Hals dunkel, Beine hell."),
    },
}


def change(asset):
    reps = copy.deepcopy(asset["representations"])
    for scale, rows in TEXT.items():
        for key, (en, de) in rows.items():
            reps[scale]["decisions"][key] = {"en": en, "de": de}
    return {"representations": reps}


print("written", io.write(change))
