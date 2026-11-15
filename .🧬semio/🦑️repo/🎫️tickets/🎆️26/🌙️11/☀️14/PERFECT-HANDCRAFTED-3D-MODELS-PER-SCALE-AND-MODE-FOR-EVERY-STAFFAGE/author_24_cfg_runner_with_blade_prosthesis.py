from author_24_lib import *

sym_skip = ("thigh-l", "shorts-l")
DROP = {
    100: ["eye-l", "eye-r", "blade-bolt"],
    200: ["eye-l", "eye-r", "ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "singlet-hem", "socket-trim", "shoe-sole", "shoe-toe-sole", "blade-bolt", "bib"],
    500: ["knee-l", "knee-r", "shoulders", "socket-pyramid", "blade-toe"],
}
KITFIT = {500: ["--shift --sections --rounds 3 --bounds 0.6,2.2"]}
FIT = {500: dict(bounds="0.6,2.4", rounds=5, steps="0.15,0.08,0.04")}


def repair50(vols):
    pass


def post50(vols):
    for s in vols["50"]["body"]:
        if s["id"] == "blade-toe":
            for q in s["sections"]:
                q["at"][1] -= 0.0165
                q["ry"] = 0.0045
        if s["id"] == "socket-trim":
            for q in s["sections"]:
                q["rx"] += 0.004
                q["ry"] += 0.004
        if s["id"] == "bib":
            s["texture"] = True


thermal_skip = ("bib",)


def post_finish(vols):
    t = vols["50"]["thermal"]
    t[:] = [x for x in t if not x["id"].startswith(("th-running-shoe-socks-shoe-sole", "th-running-shoe-socks-shoe-toe-sole"))]
    for sc in vols:
        for x in vols[sc]["thermal"]:
            if x["id"] == "th-body-plume":
                x["radii"][2] = min(x["radii"][2], 0.145)
                x["radii"][1] = min(x["radii"][1], 0.165)
        vols[sc]["structure"][:] = [x for x in vols[sc]["structure"] if x["id"] != "joint-blade-tip-r"]


def special(b, s):
    pass


DECISIONS = {
    50: [
        D("Body: limbs, singlet trunk, neck and head are lofts of superelliptic sections fitted to the elevation, front and plan drawings of the standing-ready pose, left and right mirrored where the rig is symmetric; the left leg is a thigh, shorts, knee ball and shank with a running shoe (sole and toe cap are own lofts), the right leg is a thigh in shorts that ends in a carbon-fibre socket (loft with trim, pyramid adapter box and bolt) from which the J-shaped blade runs as two lofts (upper laminate, curl) with a rubber toe pad lying below the blade tip; the hands are mittens with a thumb, ears, eyes and nose are small skin ellipsoids and the race bib is a thin texture box. In the plan drawing the shorts ellipses reached 0.1545 m against 0.150 m in the front view; their radius was set to 75.5 mm at the root (C11).", "Körper: Gliedmassen, Trikot-Rumpf, Hals und Kopf sind Lofts aus superelliptischen Querschnitten, an die Zeichnungen von Aufriss, Vorderansicht und Grundriss der Startpose angepasst, links und rechts gespiegelt, wo das Rig symmetrisch ist; das linke Bein besteht aus Oberschenkel, Shorts, Kniekugel und Unterschenkel mit Laufschuh (Sohle und Zehenkappe sind eigene Lofts), das rechte Bein ist ein Oberschenkel in Shorts, der in einem Carbon-Schaft endet (Loft mit Rand, Pyramidenadapter-Quader und Bolzen), aus dem die J-förmige Feder als zwei Lofts (obere Lamelle, Krümmung) läuft, mit einem Gummi-Zehenpad unter der Federspitze; die Hände sind Fäustlinge mit Daumen, Ohren, Augen und Nase kleine Hautellipsoide und die Startnummer ein dünner Texturquader. Im Grundriss reichten die Shorts-Ellipsen bis 0,1545 m gegenüber 0,150 m in der Vorderansicht; ihr Radius wurde an der Wurzel auf 75,5 mm gesetzt (C11)."),
        D("Thermal: each region consists of the body solids that carry it (references); the singlet, shorts, shoe and socket layers are nested shells, innermost first, each inset by the thickness of the layers outside it; the 4 mm shoe soles get no sock shell (thinner than the 7.5 mm feature); nose and mouth outlets are capsules at their anchors, the body plume is a thin disc over the crown whose extent is held inside the body box. The areaM2 of socket (0.0959 to 0.068 m2) and blade (0.09 to 0.076 m2) were corrected at the root to what the modelled solids expose (factor 1.4 and 1.2), the physiological areas of the body regions were not touched; the hands expose only 0.038 of 0.0888 m2 because the drawn arms hang over the thighs (finding, see the report).", "Thermik: jede Region besteht aus den Körpern, die sie tragen (Verweise); die Schichten von Trikot, Shorts, Schuh und Schaft sind verschachtelte Schalen, innerste zuerst, jede um die Dicke der äusseren Schichten eingerückt; die 4-mm-Schuhsohlen erhalten keine Sockenschale (dünner als das Merkmal von 7,5 mm); Nasen- und Mundauslass sind Kapseln an ihren Ankern, die Körperfahne ist eine dünne Scheibe über dem Scheitel, deren Ausdehnung im Körperquader bleibt. Die Fläche areaM2 von Schaft (0,0959 auf 0,068 m2) und Feder (0,09 auf 0,076 m2) wurde an der Wurzel auf das korrigiert, was die modellierten Körper freilegen (Faktor 1,4 und 1,2), die physiologischen Flächen der Körperregionen blieben unberührt; die Hände legen nur 0,038 von 0,0888 m2 frei, weil die gezeichneten Arme über den Oberschenkeln hängen (Befund, siehe Bericht)."),
        D("Structure: a member loft with a core section (about 60 % of the clothed limb, 72 % in trunk and pelvis, 80 % in the head) along every load-bearing bone including the socket and the blade (the blade members follow the carbon laminate, the blade is one rigid piece, so it has no joint ball at the curl), ball joints at the other rig joints and mass ellipsoids at the centres of mass of the heavy segments; the load path on the right runs from the blade through the socket and its adapter into the thigh and pelvis, on the left through shoe, shank and thigh.", "Tragwerk: ein Stab-Loft mit Kernquerschnitt (etwa 60 % der bekleideten Gliedmasse, 72 % in Rumpf und Becken, 80 % im Kopf) entlang jedes lastabtragenden Knochens einschliesslich Schaft und Feder (die Federstäbe folgen dem Carbonlaminat, die Feder ist ein starres Stück und hat deshalb keine Gelenkkugel an der Krümmung), Kugelgelenke an den übrigen Rig-Gelenken und Massenellipsoide in den Schwerpunkten der schweren Segmente; der Lastweg rechts verläuft von der Feder durch Schaft und Adapter in Oberschenkel und Becken, links durch Schuh, Unterschenkel und Oberschenkel."),
    ],
    100: [
        D("1:100 is modelled anew against the 1:100 drawings: loft sections reduced to those that carry the outline (7.5 mm tolerance), eyes and the bolt (below 15 mm) are merged, every part is refitted to the 1:100 silhouette; thermal references and shells follow the new solids (the layers under 15 mm form one shell), structure has member lofts with three sections and joints at the rig joints.", "1:100 ist neu gegen die 1:100-Zeichnungen modelliert: Loft-Querschnitte auf die konturtragenden reduziert (Toleranz 7,5 mm), Augen und Bolzen (unter 15 mm) entfallen, jeder Teil ist an die 1:100-Silhouette neu angepasst; thermische Verweise und Schalen folgen den neuen Körpern (die Schichten unter 15 mm bilden eine Schale), das Tragwerk hat Stab-Lofts mit drei Querschnitten und Gelenke an den Rig-Gelenken."),
    ],
    200: [
        D("1:200 merges: ears, nose, eyes, thumbs, the bib, the singlet hem, the socket trim, the shoe soles and the bolt (below 30 mm) are absorbed by the neighbouring lofts; the lofts keep two or three sections, the blade stays two lofts plus the toe pad because the 1:200 front draws all of them. Thermal: one surface per region; structure: member lofts with two sections, balls only at the main joints.", "1:200 fasst zusammen: Ohren, Nase, Augen, Daumen, Startnummer, Trikotsaum, Schaftrand, Schuhsohlen und Bolzen (unter 30 mm) gehen in den Nachbar-Lofts auf; die Lofts behalten zwei oder drei Querschnitte, die Feder bleibt zwei Lofts plus Zehenpad, weil die 1:200-Vorderansicht alle zeichnet. Thermik: eine Oberfläche je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, Kugeln nur an den Hauptgelenken."),
    ],
    500: [
        D("1:500 is a generalised silhouette model fitted with a fine raster and section shifts: every part is a loft of two or three sections at the 75 mm feature (the blade is drawn as a 65 mm pen stroke and is modelled that thick), knee balls, shoulders, the socket adapter and the toe pad are absorbed. Thermal: one solid per region; structure: member lofts with two sections, joints merged into the members.", "1:500 ist ein verallgemeinertes Silhouettenmodell, mit feinem Raster und Querschnittsverschiebung angepasst: jeder Teil ist ein Loft aus zwei oder drei Querschnitten beim 75-mm-Merkmal (die Feder ist als 65-mm-Federstrich gezeichnet und wird so dick modelliert), Kniekugeln, Schultern, Schaftadapter und Zehenpad gehen auf. Thermik: ein Körper je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, Gelenke in den Stäben zusammengefasst."),
    ],
}
