from author_24_lib import *

FIT50 = False
floor_skip = ("shoe", "boot")
sym_skip = ()
DROP = {
    100: ["eye-l", "eye-r", "cup-sleeve"],
    200: ["eye-l", "eye-r", "ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "cup-sleeve", "cup-lid", "pad-pages", "sweater-hem"],
    500: ["eye-l", "eye-r", "ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "cup-sleeve", "cup-lid", "pad-pages", "sweater-hem", "shoulder-l", "shoulder-r", "knee-l", "knee-r", "cowl", "shoulders"],
}


def repair50(vols):
    for s in vols["50"]["body"]:
        if s["id"] == "page-sheet":
            s["size"][1] = 0.0075
            s["transform"]["translation"][1] = 0.0155


def post50(vols):
    for s in vols["50"]["body"]:
        if s["id"] == "pelvis-jeans":
            for q in s["sections"]:
                if q["at"][0] > 0.0:
                    q["rx"] *= 0.80
                    q["ry"] *= 0.82
        if s["id"] in ("sweater", "sweater-hem"):
            for q in s["sections"][:5]:
                q["rx"] *= 1.07
                q["ry"] *= 1.07


def special(b, s):
    if s == 500:
        for x in b:
            if x["id"] == "bench-seat":
                x["profile"] = [[-0.28, 0.4205], [0.2, 0.4205], [0.2, 0.378], [-0.28, 0.378]]
            if x["id"] in ("bench-leg-front", "bench-leg-rear"):
                x["size"] = [0.06, 0.38, 0.06]
                x["transform"]["translation"][0] = 0.14 if x["id"].endswith("front") else -0.23
            if x["id"] == "bench-back":
                x["profile"] = [[-0.3, 0.42], [-0.259, 0.42], [-0.339, 0.86], [-0.38, 0.86]]


skip_members = ()


def struct_extra(a, body, scale):
    f = FEATURE[scale]
    out = []
    for bone in member_bones(a):
        p = primary(body, bone)
        if p is None or p["kind"] in ("loft", "ellipsoid"):
            continue
        if p["kind"] == "extrude":
            c = shrink(p, {50: 0.01, 100: 0.012, 200: 0.012, 500: 0.0}[scale], f"member-{bone}")
            c.pop("tags", None)
        else:
            c = scaled(p, 0.7 if scale < 500 else 1.0, f"member-{bone}")
            if c["kind"] == "box":
                c["size"] = [min(o, max(n, f)) for o, n in zip(p["size"], c["size"])]
        c.pop("region", None)
        c["member"], c["part"], c["role"] = bone, "member", "body"
        out.append(c)
    return out
DECISIONS = {
    50: [
        D("Body: lofts of superelliptic sections for the sweater trunk with its hem and cowl, the jeans over pelvis and thighs, sleeves, forearms, mitten hands with thumbs, shoes in a rear and a toe part, neck, head, hair with bun, ears, eyes and nose; the reader holds a notebook (cover and pages, one loose page) in both hands and sits upright on a wooden bench with a 40 mm seat plank, a back plank of 35 mm leaning 11 degrees and four square legs, a paper cup with lid and sleeve stands on the seat. The jeans waist is taken in 18-20 % above the hip and the sweater hem is 7 % wider so that the sweater hangs over the waist; the loose page is 7.5 mm thick (feature).", "Körper: Lofts mit superelliptischen Querschnitten für den Pullover-Rumpf mit Saum und Rollkragen, die Jeans über Becken und Oberschenkeln, Ärmel, Unterarme, Fäustlingshände mit Daumen, Schuhe aus Hinter- und Zehenteil, Hals, Kopf, Haar mit Dutt, Ohren, Augen und Nase; die lesende Person hält ein Notizbuch (Deckel und Seiten, ein loses Blatt) in beiden Händen und sitzt aufrecht auf einer Holzbank mit 40-mm-Sitzbrett, einem 35-mm-Rückenbrett mit 11 Grad Neigung und vier Vierkantbeinen, ein Pappbecher mit Deckel und Hülse steht auf dem Sitz. Die Jeanstaille ist über der Hüfte um 18-20 % eingezogen und der Pullover-Saum 7 % weiter, damit der Pullover über die Taille fällt; das lose Blatt ist 7,5 mm dick (Merkmal)."),
        D("Thermal: references to the body solids that carry each region; sweater, jeans and shoes are nested shells per layer, innermost first; outlets and plume are tagged capsules and a disc.", "Thermik: Verweise auf die tragenden Körper je Region; Pullover, Jeans und Schuhe sind verschachtelte Schalen je Schicht, innerste zuerst; Auslässe und Fahne sind markierte Kapseln und eine Scheibe."),
        D("Structure: member lofts with a core section along every bone, cores of the bench plank, the notebook, the loose page and the cup, ball joints at the rig joints and mass ellipsoids at the heavy segments; the weight runs from the shoes through shanks and thighs into the pelvis and trunk, from there into the neck and head and through the arms to the notebook, the bench carries the pelvis and the cup.", "Tragwerk: Stab-Lofts mit Kernquerschnitt entlang jedes Knochens, Kerne von Sitzbrett, Notizbuch, losem Blatt und Becher, Kugelgelenke an den Rig-Gelenken und Massenellipsoide an den schweren Segmenten; die Last läuft von den Schuhen über Unter- und Oberschenkel in Becken und Rumpf, von dort in Hals und Kopf und über die Arme zum Notizbuch, die Bank trägt Becken und Becher."),
    ],
    100: [
        D("1:100 is derived from 1:50 and refitted to the 1:100 drawings: sections reduced to those that carry the outline, eyes and the cup sleeve below 15 mm are absorbed, shoes and waist taken in slightly so that the sweater stays near 30 % of its area; the thermal and structure lists follow the new solids.", "1:100 ist aus 1:50 abgeleitet und an die 1:100-Zeichnungen angepasst: Querschnitte auf die konturtragenden reduziert, Augen und Becherhülse unter 15 mm gehen auf, Schuhe und Taille leicht eingezogen, damit der Pullover nahe bei 30 % seiner Fläche bleibt; Thermik- und Tragwerkslisten folgen den neuen Körpern."),
    ],
    200: [
        D("1:200 merges ears, eyes, nose, thumbs, the cup lid and sleeve, the loose pages and the sweater hem (below 30 mm) into their neighbours; the bench keeps its planks and legs. Thermal: one surface per region; structure: member lofts with two sections.", "1:200 fasst Ohren, Augen, Nase, Daumen, Becherdeckel und -hülse, die losen Seiten und den Pullover-Saum (unter 30 mm) in den Nachbarn zusammen; die Bank behält Bretter und Beine. Thermik: eine Oberfläche je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten."),
    ],
    500: [
        D("1:500 is a generalised silhouette model fitted with the fine raster and section shifts: the bench legs sit at the drawn 60 mm and 140 / minus 230 mm, the seat plank is 42 mm, the toe overlaps the shoe to reach 75 mm, the notebook is a 79 mm slab (the 2D bounding extents of its drawings forbid a thinner one, so it shows blue beside the drawn thin notebook); the hair bun stays. Thermal: one solid per region; structure: member lofts with two sections.", "1:500 ist ein verallgemeinertes Silhouettenmodell, mit feinem Raster und Querschnittsverschiebung angepasst: die Bankbeine stehen bei den gezeichneten 60 mm und 140 bzw. minus 230 mm, das Sitzbrett ist 42 mm, der Zeh überlappt den Schuh, um 75 mm zu erreichen, das Notizbuch ist eine 79-mm-Platte (die 2D-Hüllmasse seiner Zeichnungen verbieten eine dünnere, daher blau neben dem gezeichneten dünnen Notizbuch); der Haardutt bleibt. Thermik: ein Körper je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten."),
    ],
}

KITFIT = {500: ["--shift --sections --exp --rounds 6 --bounds 0.6,2.2 --solid sweater,pelvis-jeans,neck,head,hair,hair-bun,sleeve-l,sleeve-r,forearm-l,forearm-r,hand-l,hand-r,thigh-l,thigh-r,shank-l,shank-r,shoe-rear-l,shoe-rear-r"]}
FIT = {500: dict(bounds="0.6,2.4", rounds=5, steps="0.15,0.08,0.04")}

TOL = {500: 0.012}


def post_fit(b, s):
    if s == 500:
        for x in b:
            if x["id"].startswith("shoe-toe") and x["kind"] == "loft":
                x["sections"][-1]["at"][0] = max(x["sections"][-1]["at"][0], 0.0765)
