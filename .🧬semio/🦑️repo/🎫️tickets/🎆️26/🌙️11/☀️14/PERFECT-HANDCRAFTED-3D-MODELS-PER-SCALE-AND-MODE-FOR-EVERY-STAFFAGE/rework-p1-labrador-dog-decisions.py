"""📝 Adds the decisions of the hindquarter, neck and per-scale rework to the labrador volumes.

Usage: python3 rework-p1-labrador-dog-decisions.py
"""
import json

SOURCE = "volumes/animals/labrador-dog.json"
RUMP = {
    "en": "Hindquarters and neck base: the thigh starts as a gluteal dome inside the croup and widens to the full thigh below the hip, so rump and thigh read as one mass from behind and above; the neck ends in a ball about the joint to the thorax, so lifting or lowering the neck opens no shelf at the withers.",
    "de": "Hinterhand und Halsansatz: der Oberschenkel beginnt als Kruppenmuskelkuppe in der Kruppe und erreicht erst unter der Hüfte seine volle Breite, sodass Kruppe und Keule von hinten und oben als eine Masse lesbar sind; der Hals endet in einer Kugel um das Gelenk zum Brustkorb, sodass Heben und Senken des Halses am Widerrist keine Stufe öffnet.",
}
DECISIONS = {
    "50": [RUMP, {
        "en": "Skeleton: the ear cartilage is rooted on the skull by its annular cartilage and the shoulder joint markers reach the shoulder blades, so every member lies on one load path down to the pads.",
        "de": "Skelett: der Ohrknorpel sitzt über seinen Ringknorpel am Schädel und die Schultergelenkmarken reichen bis an die Schulterblätter, sodass jedes Glied auf einem Lastpfad bis zu den Ballen liegt.",
    }],
    "100": [RUMP, {
        "en": "Head for 15 mm: the eyelids are left out, the eyes stay as globes, the tongue is thickened to the feature; in the skeleton the zygomatic arches are thickened to the thinnest drawn stroke of the head and the ear cartilage is rooted on the skull.",
        "de": "Kopf für 15 mm: die Lider entfallen, die Augen bleiben als Augäpfel, die Zunge ist auf das Mindestmaß verdickt; im Skelett sind die Jochbögen auf den dünnsten gezeichneten Strich des Kopfes verdickt und der Ohrknorpel sitzt am Schädel.",
    }],
    "200": [RUMP, {
        "en": "Head for 30 mm: lids, pupils and nostrils are left out; eyes and tongue are no longer body solids but thermal surfaces of their regions, nose leather and flews are stated at the feature. Skeleton: the seven lumbar vertebrae are one tapering bar with the breadth of the transverse processes, the mandible is one bar of 31 mm, the zygomatic arches are left out, the ear cartilage is rooted on the skull and the first caudal bar reaches into the sacrum.",
        "de": "Kopf für 30 mm: Lider, Pupillen und Nasenlöcher entfallen; Augen und Zunge sind keine Körper mehr, sondern thermische Flächen ihrer Regionen, Nasenspiegel und Lefzen sind im Mindestmaß gesetzt. Skelett: die sieben Lendenwirbel sind ein sich verjüngender Stab in der Breite der Querfortsätze, der Unterkiefer ist ein Stab von 31 mm, die Jochbögen entfallen, der Ohrknorpel sitzt am Schädel und der erste Schwanzwirbelstab reicht in das Kreuzbein.",
    }],
    "500": [RUMP, {
        "en": "Head for 75 mm: one mass of skull, muzzle and lower jaw, as the drawing shows it; eyes, nose and tongue are thermal surfaces only, lids, pupils, nostrils and flews are left out; paws, front pasterns and ears are stated at the thinnest drawn stroke of their bones. Skeleton as a bar diagram at pen width: neck, thoracic and lumbar spine, shoulder blades, tail and toes are bars of 75 mm (or of the thinnest drawn stroke of their bone) on the rig axes, like the limb bars; the mandible is one bar along the muzzle; at this scale the bars of tail and jaw are wider than the bone they stand for.",
        "de": "Kopf für 75 mm: eine Masse aus Schädel, Fang und Unterkiefer, wie die Zeichnung sie zeigt; Augen, Nase und Zunge sind nur thermische Flächen, Lider, Pupillen, Nasenlöcher und Lefzen entfallen; Pfoten, Vordermittelfüße und Ohren sind im dünnsten gezeichneten Strich ihrer Knochen gesetzt. Skelett als Stabdiagramm in Strichbreite: Hals-, Brust- und Lendenwirbelsäule, Schulterblätter, Rute und Zehen sind Stäbe von 75 mm (oder des dünnsten gezeichneten Strichs ihres Knochens) auf den Rig-Achsen, wie die Gliedmaßenstäbe; der Unterkiefer ist ein Stab entlang des Fangs; in diesem Maßstab sind die Stäbe von Rute und Kiefer breiter als der Knochen, für den sie stehen.",
    }],
}


def main():
    document = json.load(open(SOURCE))
    for scale, added in DECISIONS.items():
        decisions = document["volumes"][scale]["decisions"]
        decisions.extend(entry for entry in added if entry not in decisions)
    five = document["volumes"]["500"]["decisions"]
    for entry in five:
        if entry["en"].startswith("The native skull and jaw"):
            entry["en"] = "A sternum bar joins the rib hoop ventrally, so the ribcage is closed between spine and breastbone."
            entry["de"] = "Ein Brustbeinstab verbindet den Rippenreifen bauchseitig, sodass der Brustkorb zwischen Wirbelsäule und Brustbein geschlossen ist."
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False)


main()
