"""🐮 Round 5 dairy cow: writes the handcrafted volumes on the corrected rig: body, thermal and structure lists and the decisions of every authored scale.

Usage: python3 r5-dairy-cow-model.py   (writes volumes/animals/dairy-cow.json and 🗑️generated/r5-dairy-cow/groups.json)
@see ./r5_dairy_cow_body.py
@see ./r5_dairy_cow_frame.py
"""
import json
import os

import r5_dairy_cow_body as body
import r5_dairy_cow_frame as frame
import r3_dairy_cow_io as io
from r3_dairy_cow_kit import Kit

T = os.path.dirname(os.path.abspath(__file__))
SKIN = {"material": "bare-skin", "role": "detail", "tone": 0.2}
COAT = {"material": "white-hair", "role": "detail", "tone": 0.8}

DECISIONS = {
    "50": [
        {"en": "Body at 1:50: a Holstein-Friesian cow of 1.45 m at the withers, modelled mass by mass. The trunk is the barrel (ribcage and abdomen, widest at the rumen), the back with the lumbar shelf and the withers, and the boxy rump with hook and pin bones and the sacral crest; chest and hindquarters meet in a shared section at the thoracolumbar joint. Each hind leg has the thigh plate, stifle, gaskin with the Achilles line, hock with the point of the hock, cannon, pastern, two dewclaws and two claws; each foreleg the shoulder, upper arm, elbow point, forearm, knee, cannon, pastern, dewclaws and claws. The neck is deep and narrow with crest and dewlap, the head has poll, brows, eyes, the blaze, the bare muzzle with nostrils and the jaw as its own moving piece; the ears stand out sideways; the udder is two halves with four teats; the tail is root, shaft and switch. The black coat patches (saddle, croup, lower chest with the brisket) lie on the skin as thin straps that follow its curvature.", "de": "Körper bei 1:50: eine Holstein-Friesian-Kuh mit 1,45 m Widerristhöhe, Masse für Masse modelliert. Der Rumpf besteht aus der Tonne (Brustkorb und Bauch, am Pansen am breitesten), dem Rücken mit der Lendenplatte und dem Widerrist und der kantigen Kruppe mit Hüft- und Sitzbeinhöckern und dem Kreuzbeinkamm; Brust und Hinterhand treffen sich in einem gemeinsamen Schnitt am Übergang von Brust- zu Lendenwirbelsäule. Jedes Hinterbein hat die Keule, das Knie, den Unterschenkel mit der Achillessehne, das Sprunggelenk mit dem Fersenhöcker, das Röhrbein, die Fessel, zwei Afterklauen und zwei Klauen; jedes Vorderbein die Schulter, den Oberarm, den Ellbogenhöcker, den Unterarm, die Vorderfußwurzel, das Röhrbein, die Fessel, Afterklauen und Klauen. Der Hals ist tief und schmal mit Kamm und Triel, der Kopf hat Genick, Augenbögen, Augen, die Blesse, das nackte Flotzmaul mit Nüstern und den Unterkiefer als eigenes bewegtes Teil; die Ohren stehen seitlich ab; das Euter besteht aus zwei Hälften mit vier Zitzen; der Schwanz aus Wurzel, Rübe und Quaste. Die schwarzen Fellflecken (Sattel, Kruppe, Unterbrust mit der Brustspitze) liegen als dünne Bänder auf der Haut und folgen ihrer Krümmung."},
        {"en": "Thermal at 1:50: every region is the body surface it names, with the single coat layer of the region; the rumen flank is a strap on each flank of the loin, the two nostrils are the breath outlets and a strap on the back is the outlet of the plume. The hooves are modelled at their real size; their surface is larger than the declared share.", "de": "Wärme bei 1:50: Jede Region ist die Körperoberfläche, die sie benennt, mit der einen Fellschicht der Region; die Pansenflanke ist ein Band auf jeder Flanke der Lende, die beiden Nüstern sind die Atemauslässe und ein Band auf dem Rücken ist der Auslass der Wärmefahne. Die Klauen sind in ihrer wahren Größe modelliert; ihre Oberfläche ist größer als der angegebene Anteil."},
        {"en": "Structure at 1:50: the skeleton. Skull with orbits and two mandible halves, seven cervical vertebrae with the atlas, the thoracic and lumbar column with thirteen and six dorsal processes and the lumbar transverse processes, thirteen pairs of ribs (eight closed at the sternum, five open below), sternum, sacrum with its crest, iliac crests, ilia, ischia and pubis, shoulder blades with their spines, humeri, radii with the olecranon, carpi, cannon bones, the two digits of every foot with long pastern, short pastern and claw bone, femora with the greater trochanter, patellae, tibiae, tarsi with the calcaneus, the caudal vertebrae, the ear cartilages and the median ligament that carries the udder; joints as balls of the size of the real joint and three masses at the centres of pelvis, loin and chest. The asset declares no structural supports, so there are no support solids.", "de": "Tragwerk bei 1:50: das Skelett. Schädel mit Augenhöhlen und zwei Unterkieferhälften, sieben Halswirbel mit dem Atlas, Brust- und Lendenwirbelsäule mit dreizehn und sechs Dornfortsätzen und den Querfortsätzen der Lende, dreizehn Rippenpaare (acht am Brustbein geschlossen, fünf unten offen), Brustbein, Kreuzbein mit Kamm, Darmbeinkämme, Darmbeine, Sitzbeine und Schambein, Schulterblätter mit Gräte, Oberarmbeine, Speichen mit Ellbogenhöcker, Vorderfußwurzeln, Röhrbeine, die zwei Zehen jedes Fußes mit Fesselbein, Kronbein und Klauenbein, Oberschenkelbeine mit großem Rollhügel, Kniescheiben, Schienbeine, Sprunggelenke mit Fersenbein, die Schwanzwirbel, die Ohrknorpel und das mittlere Aufhängeband des Euters; Gelenke als Kugeln in der Größe des echten Gelenks und drei Massen in den Schwerpunkten von Becken, Lende und Brust. Das Asset nennt keine Auflager, daher gibt es keine Auflagerkörper."},
    ],
    "100": [
        {"en": "Body at 1:100, authored for a smallest feature of 15 mm: the trunk keeps its three masses with fewer sections, hook and pin bones and the sacral crest are part of them; stifle, elbow point and point of the hock are merged into thigh, forearm and cannon; each cloven hoof is one block under its pastern; the udder is one bag with four teats; the head has no brows and nostrils, the neck no separate crest. The three coat patches stay.", "de": "Körper bei 1:100, für ein kleinstes Merkmal von 15 mm gezeichnet: Der Rumpf behält seine drei Massen mit weniger Schnitten, Hüft- und Sitzbeinhöcker und der Kreuzbeinkamm gehen darin auf; Knie, Ellbogenhöcker und Fersenhöcker sind mit Keule, Unterarm und Röhrbein verschmolzen; jede Klaue ist ein Block unter ihrer Fessel; das Euter ist ein Beutel mit vier Zitzen; der Kopf hat keine Augenbögen und Nüstern, der Hals keinen eigenen Kamm. Die drei Fellflecken bleiben."},
        {"en": "Thermal at 1:100: the regions follow the 1:100 body; rumen flank, nostril outlets and the back outlet as at 1:50.", "de": "Wärme bei 1:100: Die Regionen folgen dem Körper von 1:100; Pansenflanke, Nüstern und Rückenauslass wie bei 1:50."},
        {"en": "Structure at 1:100: the skeleton with seven rib hoops, the dorsal processes as two fins and the lumbar transverse processes as one shelf, one mandible, the cervical vertebrae, ilium and ischium, every long bone, calcaneus and olecranon, and the digits of each foot as one bar; patella, trochanter, orbits and pubis are left out.", "de": "Tragwerk bei 1:100: das Skelett mit sieben Rippenreifen, den Dornfortsätzen als zwei Kämmen und den Lendenquerfortsätzen als einer Platte, einem Unterkiefer, den Halswirbeln, Darm- und Sitzbein, jedem Röhrenknochen, Fersenbein und Ellbogenhöcker und den Zehen jedes Fußes als einem Stab; Kniescheibe, Rollhügel, Augenhöhlen und Schambein entfallen."},
    ],
    "200": [
        {"en": "Body at 1:200, authored for 30 mm: one mass per rig bone. Hindquarters with loin and the chest are one egg each, saddle and croup patch stay as straps so that the animal still reads as a Holstein; each leg segment is one loft, pastern and hoof are one piece, the udder is a bag without teats, muzzle and eyes are part of the head, the tail switch is the third tail piece.", "de": "Körper bei 1:200, für 30 mm gezeichnet: eine Masse je Bone. Hinterhand mit Lende und die Brust sind je ein Ei, Sattel- und Kruppenfleck bleiben als Bänder, damit das Tier als Holstein lesbar bleibt; jedes Beinsegment ist ein Loft, Fessel und Klaue sind ein Stück, das Euter ist ein Beutel ohne Zitzen, Flotzmaul und Augen gehören zum Kopf, die Quaste ist das dritte Schwanzstück."},
        {"en": "Thermal at 1:200: regions on the merged masses; muzzle, eyes and the rumen flank are thermal solids of their own because the body merges them.", "de": "Wärme bei 1:200: Regionen auf den verschmolzenen Massen; Flotzmaul, Augen und Pansenflanke sind eigene Wärmekörper, weil der Körper sie verschmilzt."},
        {"en": "Structure at 1:200: spine in three pieces with two fins and the lumbar shelf, four rib hoops, sternum, skull, mandible, the neck column as one curved bar, one hip bone per side, the long bones, calcaneus and olecranon, digits as one bar; joints only at the limb joints and at the neck.", "de": "Tragwerk bei 1:200: Wirbelsäule in drei Stücken mit zwei Kämmen und der Lendenplatte, vier Rippenreifen, Brustbein, Schädel, Unterkiefer, die Halswirbelsäule als ein gebogener Stab, je Seite ein Hüftbein, die Röhrenknochen, Fersenbein und Ellbogenhöcker, Zehen als ein Stab; Gelenke nur an den Gliedmaßen und am Hals."},
    ],
    "500": [
        {"en": "Body at 1:500, authored for 75 mm and minimal: hindquarters and chest as two eggs in one mixed coat tone, neck and head as two wedges with ears, jaw and muzzle merged in, thigh and shoulder masses, each lower leg in two tapered pieces of which the lower ends as the hoof, the udder as one bag and the tail in three pieces as thick as it is drawn.", "de": "Körper bei 1:500, für 75 mm gezeichnet und minimal: Hinterhand und Brust als zwei Eier in einem gemischten Fellton, Hals und Kopf als zwei Keile, in denen Ohren, Unterkiefer und Flotzmaul aufgehen, Keule und Schultermasse, jedes Unterbein in zwei verjüngten Stücken, deren unteres als Klaue endet, das Euter als ein Beutel und der Schwanz in drei Stücken so dick, wie er gezeichnet ist."},
        {"en": "Thermal at 1:500: the regions the body merges (patches, rumen flank, ears, eyes, muzzle, hooves) are thermal solids of their own on the merged masses.", "de": "Wärme bei 1:500: Die Regionen, die der Körper verschmilzt (Flecken, Pansenflanke, Ohren, Augen, Flotzmaul, Klauen), sind eigene Wärmekörper auf den verschmolzenen Massen."},
        {"en": "Structure at 1:500: the load path only: the spine as three beams with the withers fin, two rib hoops, one hip bone per side, skull, mandible, the neck bar, and every leg as bars from shoulder blade or femur to the digits, with joints at the main limb joints.", "de": "Tragwerk bei 1:500: nur der Lastpfad: die Wirbelsäule als drei Balken mit dem Widerristkamm, zwei Rippenreifen, je Seite ein Hüftbein, Schädel, Unterkiefer, der Halsstab und jedes Bein als Stäbe vom Schulterblatt oder Oberschenkelbein bis zu den Zehen, mit Gelenken an den Hauptgelenken der Gliedmaßen."},
    ],
}


def emitters(k, tables, feature):
    """💨 D2: the outlets of the two emitters on the surface they leave from, never thinner than the smallest feature of the scale."""
    r = max(0.008, feature / 2 + 0.0002)
    for side, name in ((1, "far"), (-1, "near")):
        k.ell(f"breath-nose-{name}", "head", (0.532 - max(0, r - 0.012), 0.012, 0.036), (r, max(r, 0.012), max(r, 0.0125)), SKIN, "emitter", "muzzle", side=side, tags=["emitter", "breath-nose"])
    rows = []
    thick = max(0.01, feature / 2 + 0.0002)
    for x in (0.4, 0.5, 0.62, 0.75, 0.9, 1.0, 1.1):
        p, n = body.skin(x, 0, tables=tables)
        rows.append((p, n, (0, 0, 1), 0.04 if x in (0.4, 1.1) else 0.09, -0.008 if x in (0.4, 1.1) else 0.004))
    k.strap("plume-back", "lumbar", rows, COAT, "emitter", "coat-trunk", thick=thick, tags=["emitter", "plume-back"])


def main():
    text, asset = io.read()
    layered = {r["id"]: r["layers"][0]["id"] for r in asset["thermal"]["regions"] if r["layers"]}
    path = os.path.join(T, "volumes/animals/dairy-cow.json")
    source = json.load(open(path, encoding="utf-8"))
    groups = {}
    builds = (("50", body.body50, body.TRUNK), ("100", body.body100, body.TRUNK100), ("200", body.body200, body.TRUNK200), ("500", body.body500, (body.REAR500, body.FRONT500)))
    for lod, (scale, build, tables) in enumerate(builds):
        bones = asset["rig"]["bones"]
        k = Kit(bones)
        k.thermal = Kit(bones)
        build(k)
        emitters(k.thermal, tables, int(scale) * 0.00015)
        thermal = []
        for s in k.solids:
            m = k.meta[s["id"]]
            row = {"id": "th-" + s["id"], "of": s["id"], "region": m["region"]}
            if m["region"] in layered:
                row["layer"] = layered[m["region"]]
            thermal.append(row)
        for s in k.thermal.solids:
            m = k.thermal.meta[s["id"]]
            row = {**s, "id": "th-" + s["id"], "region": m["region"]}
            if m["region"] in layered and "tags" not in m:
                row["layer"] = layered[m["region"]]
            if "tags" in m:
                row["tags"] = m["tags"]
            thermal.append(row)
        f = Kit(bones)
        frame.frame(f, lod)
        structure = [{**s, "member": f.meta[s["id"]]["group"], "part": f.meta[s["id"]]["region"]} for s in f.solids]
        source["volumes"][scale] = {"status": "authored", "denominator": int(scale), "body": k.solids, "thermal": thermal, "structure": structure, "decisions": DECISIONS[scale]}
        groups[scale] = {"body": k.meta, "thermal": {"th-" + i: m for i, m in k.thermal.meta.items()}}
    json.dump(source, open(path, "w", encoding="utf-8"), ensure_ascii=False)
    os.makedirs(os.path.join(T, "🗑️generated/r5-dairy-cow"), exist_ok=True)
    json.dump(groups, open(os.path.join(T, "🗑️generated/r5-dairy-cow/groups.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print({s: (len(source["volumes"][s]["body"]), len(source["volumes"][s]["thermal"]), len(source["volumes"][s]["structure"])) for s in source["volumes"]})


main()
