from author_24_lib import *

sym_skip = tuple(f"{p}-l" for p in ("sleeve", "forearm", "hand", "thumb", "shoulder", "elbow", "fist"))
sym_skip = sym_skip + ("arm-l", "upper-arm-l")
DROP = {
    100: ["eye-l", "eye-r"],
    200: ["eye-l", "eye-r", "ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "boot-sole-l", "boot-sole-r", "boot-toe-sole-l", "boot-toe-sole-r", "sweater-hem-rib"],
    500: ["shoulder-l", "shoulder-r", "elbow-l", "elbow-r", "knee-l", "knee-r"],
}


def repair50(vols):
    pass


def post50(vols):
    for s in vols["50"]["body"]:
        if s["id"] in ("forearm-r", "elbow-r"):
            for q in s.get("sections", []):
                q["at"][2] += 0.006
            if s["kind"] == "ellipsoid":
                s["transform"]["translation"][2] += 0.006


def post_finish(vols):
    t = vols["50"]["thermal"]
    t[:] = [x for x in t if not x["id"].startswith(("th-ankle-boots-socks-boot-sole", "th-ankle-boots-socks-boot-toe-sole"))]


KITFIT = {500: ["--shift --sections --rounds 3 --bounds 0.6,2.2"]}
FIT = {500: dict(bounds="0.6,2.4", rounds=5, steps="0.15,0.08,0.04")}


def special(b, s):
    if s == 500:
        for x in b:
            if x["id"].startswith("boot-toe-") and x["kind"] == "loft":
                q = copy.deepcopy(x["sections"][0])
                q["at"][0] = -0.025
                x["sections"].insert(0, q)


DECISIONS = {
    50: [
        D("Body: the limbs, the sweater trunk, the neck and the head are lofts of superelliptic sections fitted to the elevation, front and plan drawings of the hand-on-back pose (the arms are not mirrored: the right hand rests on the lower back with the elbow out, the left arm hangs forward); the belly is a separate ellipsoid in front of the sweater trunk that carries the pregnancy, sleeves and forearms are knitted cloth lofts, the hands are mittens with a thumb, the boots are lofts with their own soles, ears, eyes and nose are small skin ellipsoids. In the plan drawing the four arm polygons were 112 mm wide against 96 mm in the front view; their half-width was scaled by 0.93 at the root so that both views give the same overall depth (C11), and the right forearm was shifted 6 mm inwards so that the body depth is 0.504 m against 0.5 m.", "Körper: Gliedmassen, Pullover-Rumpf, Hals und Kopf sind Lofts aus superelliptischen Querschnitten, an die Zeichnungen von Aufriss, Vorderansicht und Grundriss der Haltung Hand-am-Rücken angepasst (die Arme sind nicht gespiegelt: die rechte Hand ruht mit ausgestelltem Ellbogen auf dem unteren Rücken, der linke Arm hängt nach vorn); der Bauch ist ein eigenes Ellipsoid vor dem Pullover-Rumpf und trägt die Schwangerschaft, Ärmel und Unterarme sind Strick-Lofts, die Hände Fäustlinge mit Daumen, die Stiefel Lofts mit eigenen Sohlen, Ohren, Augen und Nase kleine Hautellipsoide. Im Grundriss waren die vier Armpolygone 112 mm breit gegenüber 96 mm in der Vorderansicht; ihre Halbbreite wurde an der Wurzel mit 0,93 skaliert, damit beide Ansichten dieselbe Gesamttiefe ergeben (C11), und der rechte Unterarm wurde um 6 mm nach innen versetzt, sodass die Körpertiefe 0,504 m gegenüber 0,5 m beträgt."),
        D("Thermal: each region consists of the body solids that carry it (references); the layered regions are nested shells, innermost first (T-shirt under knit, briefs under leggings, socks under boot), each inset by the thicknesses of the layers outside it; the shells of the 4 mm soles are left out because they would be thinner than the 7.5 mm feature, the socks layer still has shells in the boots; nose and mouth outlets are small capsules at their anchors, the body plume is a thin disc over the crown.", "Thermik: jede Region besteht aus den Körpern, die sie tragen (Verweise); die mehrschichtigen Regionen sind verschachtelte Schalen, innerste zuerst (T-Shirt unter Strick, Slip unter Leggings, Socken unter Stiefel), jede um die Dicken der äusseren Schichten eingerückt; die Schalen der 4-mm-Sohlen entfallen, weil sie dünner als das Merkmal von 7,5 mm wären, die Sockenschicht hat in den Stiefeln weiterhin Schalen; Nasen- und Mundauslass sind kleine Kapseln an ihren Ankern, die Körperfahne ist eine dünne Scheibe über dem Scheitel."),
        D("Structure: a member loft with a core section (about 60 % of the clothed limb, 72 % in trunk and pelvis, 80 % in the head) along every load-bearing bone, ball joints at the rig joints and mass ellipsoids at the centres of mass of the heavy segments; the weight path runs from the boots through shanks and thighs into the pelvis, from there into the trunk, the neck and the head and through the shoulders into both arms, the belly load is carried by its own mass ellipsoid on the trunk member.", "Tragwerk: ein Stab-Loft mit Kernquerschnitt (etwa 60 % der bekleideten Gliedmasse, 72 % in Rumpf und Becken, 80 % im Kopf) entlang jedes lastabtragenden Knochens, Kugelgelenke an den Rig-Gelenken und Massenellipsoide in den Schwerpunkten der schweren Segmente; der Lastweg verläuft von den Stiefeln über Unterschenkel und Oberschenkel ins Becken, von dort in Rumpf, Hals und Kopf und über die Schultern in beide Arme, die Bauchlast trägt ein eigenes Massenellipsoid am Rumpfstab."),
    ],
    100: [
        D("1:100 is modelled anew against the 1:100 drawings: loft sections reduced to those that carry the outline (7.5 mm tolerance), the eyes below 15 mm are merged into the head, every part is refitted to the 1:100 silhouette; thermal references and shells follow the new solids (the clothing layers under 15 mm form one shell), structure has member lofts with three sections and joints at all rig joints.", "1:100 ist neu gegen die 1:100-Zeichnungen modelliert: Loft-Querschnitte auf die konturtragenden reduziert (Toleranz 7,5 mm), die Augen unter 15 mm gehen im Kopf auf, jeder Teil ist an die 1:100-Silhouette neu angepasst; thermische Verweise und Schalen folgen den neuen Körpern (die Bekleidungsschichten unter 15 mm bilden eine Schale), das Tragwerk hat Stab-Lofts mit drei Querschnitten und Gelenke an allen Rig-Gelenken."),
    ],
    200: [
        D("1:200 merges: ears, nose, eyes, thumbs, the boot soles and the sweater hem rib (below 30 mm) are absorbed by the neighbouring lofts, the lofts keep two or three sections; the belly stays its own ellipsoid because the 1:200 drawings draw it separately. Thermal: one surface per region; structure: member lofts with two sections, balls only at the main joints.", "1:200 fasst zusammen: Ohren, Nase, Augen, Daumen, Stiefelsohlen und der Pullover-Saum (unter 30 mm) gehen in den Nachbar-Lofts auf, die Lofts behalten zwei oder drei Querschnitte; der Bauch bleibt ein eigenes Ellipsoid, weil ihn die 1:200-Zeichnungen getrennt zeichnen. Thermik: eine Oberfläche je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, Kugeln nur an den Hauptgelenken."),
    ],
    500: [
        D("1:500 is a generalised silhouette model fitted with a fine raster and section shifts: every part is a loft of two or three sections at the 75 mm feature, shoulder, elbow and knee balls are absorbed by the limbs, the toe cap overlaps the boot so that it reaches 75 mm; the belly stays its own ellipsoid. Thermal: one solid per region; structure: member lofts with two sections, joints merged into the members.", "1:500 ist ein verallgemeinertes Silhouettenmodell, mit feinem Raster und Querschnittsverschiebung angepasst: jeder Teil ist ein Loft aus zwei oder drei Querschnitten beim 75-mm-Merkmal, Schulter-, Ellbogen- und Kniekugeln gehen in den Gliedmassen auf, die Stiefelkappe überlappt den Stiefel, damit sie 75 mm erreicht; der Bauch bleibt ein eigenes Ellipsoid. Thermik: ein Körper je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, Gelenke in den Stäben zusammengefasst."),
    ],
}
