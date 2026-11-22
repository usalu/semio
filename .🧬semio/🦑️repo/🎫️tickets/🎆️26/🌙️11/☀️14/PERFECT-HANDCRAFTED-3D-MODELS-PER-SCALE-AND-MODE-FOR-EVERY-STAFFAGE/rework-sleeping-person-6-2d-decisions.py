"""✒️ Decisions texts of humans/sleeping-person (the drawing of the quilt cells stays at 0.22 mm: a thinner pen fails the print profile, see the report).

Usage: python3 rework-sleeping-person-6-2d-decisions.py   (writes 🗑️generated/rework-sleeping-person/cand3/🔣️.json from the catalog document)
The body texts say what is modelled at each scale.
"""
import json
import os
import re

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
CANDIDATE = T + "/🗑️generated/rework-sleeping-person/cand3/🔣️.json"
CLOTH = {
    "en": "The duvet is a quilt of 8 x 7 stitched squares, each made of two triangular down-filled cells that ride on a support point and four cloth diagonals of the rig, so the cloth is laid anew over trunk, pelvis, thighs, shanks, feet and toes in every pose and at every instant of the turn. The bound sides hang as hems over the mattress edges.",
    "de": "Die Bettdecke ist eine Steppdecke aus 8 x 7 Karos, jedes aus zwei dreieckigen daunengefüllten Zellen, die auf einem Auflagepunkt und vier Gewebediagonalen des Skeletts sitzen, sodass der Stoff in jeder Pose und zu jedem Zeitpunkt der Drehung neu über Rumpf, Becken, Ober- und Unterschenkel, Füße und Zehen gelegt ist. Die eingefassten Seiten hängen als Säume über die Matratzenkanten.",
}
DUVET_50 = {
    "en": "The 135 x 200 cm duvet with its foot end folded back is modelled cell by cell as the first text says",
    "de": "Die Bettdecke 135 x 200 cm mit zurückgeschlagenem Fußende ist Zelle für Zelle modelliert, wie der erste Text sagt",
}
HIPS_50 = {
    "en": " Hips and knees: the pelvis block ends in the rounded perineum 0.1 m below the hip joints, each thigh starts with the rounded gluteal mass around the hip joint and ends in the rounded knee, each shank starts with a rounded head, so hips and knees stay closed when she curls up (70° and 105°).",
    "de": " Hüften und Knie: der Beckenblock endet im gerundeten Damm 0,1 m unter den Hüftgelenken, jeder Oberschenkel beginnt mit der gerundeten Gesäßmasse um das Hüftgelenk und endet im gerundeten Knie, jeder Unterschenkel beginnt mit einem gerundeten Kopf, sodass Hüften und Knie beim Einrollen (70° und 105°) geschlossen bleiben.",
}
BODY = {
    "100": {
        "en": "Body at 1:100 (smallest feature 15 mm): the whole woman lies under the duvet, not only what shows: head with nose, hair cap and loose hair, neck, trunk from pelvis to shoulders, pelvis block with rounded perineum, thighs with gluteal head and knee end, shanks, ankles, feet with a toe block, both sleeves and fore-sleeves, both hands as flat lofts with a thumb, and the pillow. The duvet keeps its 112 cells because each is carried by its own cloth bones in every pose.",
        "de": "Körper bei 1:100 (kleinstes Merkmal 15 mm): die ganze Frau liegt unter der Decke, nicht nur das Sichtbare: Kopf mit Nase, Haarkappe und offenem Haar, Hals, Rumpf vom Becken bis zu den Schultern, Beckenblock mit gerundetem Damm, Oberschenkel mit Gesäßkopf und Knieende, Unterschenkel, Knöchel, Füße mit Zehenblock, beide Ärmel und Unterärmel, beide Hände als flache Körper mit Daumen und das Kissen. Die Decke behält ihre 112 Zellen, weil jede in jeder Pose von ihren eigenen Gewebeknochen getragen wird.",
    },
    "200": {
        "en": "Body at 1:200 (30 mm): head and hair, neck, trunk from pelvis to shoulders, pelvis block, thighs, shanks, ankles and feet as lofts of two to five sections, sleeves and fore-sleeves as capsules, both hands as ellipsoids, the pillow. The duvet keeps its cells and cloth bones as at 1:100.",
        "de": "Körper bei 1:200 (30 mm): Kopf und Haar, Hals, Rumpf vom Becken bis zu den Schultern, Beckenblock, Oberschenkel, Unterschenkel, Knöchel und Füße als Körper aus zwei bis fünf Schnitten, Ärmel und Unterärmel als Kapseln, beide Hände als Ellipsoide, das Kissen. Die Decke behält ihre Zellen und Gewebeknochen wie bei 1:100.",
    },
    "500": {
        "en": "Body at 1:500 (75 mm): head and hair, one trunk mass from pelvis to shoulders, pelvis block, thighs and shanks of three sections, ankles and feet, sleeves, fore-sleeves and both hands as lofts not thinner than 75 mm, the pillow. The duvet keeps its cells and cloth bones; that is finer than this scale needs and is kept so that the cloth follows every pose.",
        "de": "Körper bei 1:500 (75 mm): Kopf und Haar, eine Rumpfmasse vom Becken bis zu den Schultern, Beckenblock, Oberschenkel und Unterschenkel aus drei Schnitten, Knöchel und Füße, Ärmel, Unterärmel und beide Hände als Körper nicht dünner als 75 mm, das Kissen. Die Decke behält ihre Zellen und Gewebeknochen; das ist feiner, als dieser Maßstab braucht, und bleibt, damit der Stoff jeder Pose folgt.",
    },
}


def texts(volumes):
    for scale, lists in volumes.items():
        lists["decisions"][0] = dict(CLOTH)
        if scale in BODY:
            lists["decisions"][1] = dict(BODY[scale])
    body = volumes["50"]["decisions"][1]
    body["en"] = re.sub(r"The 135 x 200 cm duvet .*?(?=; the 40 x 80 cm pillow)", DUVET_50["en"], body["en"], flags=re.S)
    body["de"] = re.sub(r"Die Bettdecke 135 x 200 cm .*?(?=; das Kissen 40 x 80 cm)", DUVET_50["de"], body["de"], flags=re.S)
    for language in ("en", "de"):
        if HIPS_50[language] not in body[language]:
            body[language] += HIPS_50[language]


def main():
    document = json.load(open(CATALOG))
    texts(document["asset"]["volumes"])
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w"), ensure_ascii=False)


main()
