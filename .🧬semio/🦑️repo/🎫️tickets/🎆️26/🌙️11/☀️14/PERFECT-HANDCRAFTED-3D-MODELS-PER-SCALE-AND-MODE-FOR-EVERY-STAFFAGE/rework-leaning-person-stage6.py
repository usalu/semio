"""📝 Stage 6 of humans/leaning-person: the decisions say what rig and skeleton are now, English first, German second.

Loaded by rework-leaning-person-build.py after stage 5.
"""

ADD = {
    "50": {
        "en": "Rig and skeleton 1:50: 56 bones. Pelvis, lumbar spine and thorax form the trunk; each clavicle runs from its sternoclavicular joint 22 mm beside the mid-plane to the acromioclavicular joint 165 mm out, 76 mm further back and 36 mm higher, and the arm hangs from it at a shoulder joint 175 mm beside the mid-plane; each hip bone runs from the sacroiliac joint to the hip joint 90 mm beside the mid-plane and carries the leg. Both hands have the same fifteen finger bones in mirrored placement, and the grip of the right hand round the phone is a pose. The skeleton shows the skull with facial bones and mandible, the lumbar and thoracic vertebrae on the curve of the spine, the ribs with the sternum, scapula with spine, acromion and coracoid on the clavicle bone, sacrum, iliac wing, pubis and ischium on the hip bone, the long bones with their ends, and the bones of hands and feet.",
        "de": "Skelett und Tragwerk 1:50: 56 Knochen. Becken, Lendenwirbelsäule und Brustkorb bilden den Rumpf; jedes Schlüsselbein läuft von seinem Brustbein-Schlüsselbein-Gelenk 22 mm neben der Mittelebene zum Schultereckgelenk 165 mm außen, 76 mm weiter hinten und 36 mm höher, und der Arm hängt daran an einem Schultergelenk 175 mm neben der Mittelebene; jedes Hüftbein läuft vom Kreuzbein-Darmbein-Gelenk zum Hüftgelenk 90 mm neben der Mittelebene und trägt das Bein. Beide Hände haben dieselben fünfzehn Fingerknochen in gespiegelter Lage, und der Griff der rechten Hand um das Telefon ist eine Haltung. Das Skelett zeigt den Schädel mit Gesichtsknochen und Unterkiefer, die Lenden- und Brustwirbel auf der Krümmung der Wirbelsäule, die Rippen mit dem Brustbein, Schulterblatt mit Gräte, Schulterhöhe und Rabenschnabelfortsatz am Schlüsselbeinknochen, Kreuzbein, Darmbeinschaufel, Schambein und Sitzbein am Hüftbein, die Röhrenknochen mit ihren Enden und die Knochen von Händen und Füßen.",
    },
    "100": {
        "en": "Rig and skeleton 1:100: the rig of 1:50; the vertebrae, six rib pairs of 15 mm, clavicle and scapula on the clavicle bone, sacrum and the parts of the hip bone on the hip bone, and the long bones of 1:50; the finger bones carry nothing at this scale, the hands are mittens.",
        "de": "Skelett und Tragwerk 1:100: das Skelett von 1:50; die Wirbel, sechs Rippenpaare von 15 mm, Schlüsselbein und Schulterblatt am Schlüsselbeinknochen, Kreuzbein und die Teile des Hüftbeins am Hüftbein und die Röhrenknochen von 1:50; die Fingerknochen tragen in diesem Maßstab nichts, die Hände sind Fäustlinge.",
    },
}
SWAP = {
    "200": [
        ("a skeleton of bars: spine, three rib hoops per side, sternum, shoulder girdles, hip bones and sacrum, skull and facial block, one bar per limb segment, hand plates", "a skeleton of bars: lumbar and thoracic spine as two rods, the rib cage as one barrel, clavicle and scapula on the clavicle bones, sacrum, iliac wings, pubis and ischium on the hip bones, skull and facial block, the long bones at their true shaft section, hand plates"),
        ("ein Skelett aus Stäben: Wirbelsäule, drei Rippenbögen je Seite, Brustbein, Schultergürtel, Hüftbeine und Kreuzbein, Schädel und Gesichtsblock, ein Stab je Gliedsegment, Handplatten", "ein Skelett aus Stäben: Lenden- und Brustwirbelsäule als zwei Stäbe, der Brustkorb als eine Tonne, Schlüsselbein und Schulterblatt an den Schlüsselbeinknochen, Kreuzbein, Darmbeinschaufeln, Schambein und Sitzbein an den Hüftbeinen, Schädel und Gesichtsblock, die Röhrenknochen mit ihrem wahren Schaftquerschnitt, Handplatten"),
    ],
    "500": [
        ("a stick skeleton of 38 mm bars and one rib hoop per side, hip bones and sacrum, skull", "a stick skeleton: one spine rod through lumbar spine, thorax and neck, the rib cage as one barrel, the pelvis as one ring over both hip bones, clavicles and long bones at their true section, skull with facial block"),
        ("ein Stabskelett aus 38-mm-Stäben und einem Rippenbogen je Seite, Hüftbeine und Kreuzbein, Schädel", "ein Stabskelett: ein Wirbelsäulenstab durch Lendenwirbelsäule, Brustkorb und Hals, der Brustkorb als eine Tonne, das Becken als ein Ring über beide Hüftbeine, Schlüsselbeine und Röhrenknochen mit ihrem wahren Querschnitt, Schädel mit Gesichtsblock"),
    ],
}


def apply(a, m):
    for scale, text in ADD.items():
        a["volumes"][scale]["decisions"].append(dict(text))
    for scale, pairs in SWAP.items():
        for old, new in pairs:
            hit = 0
            for entry in a["volumes"][scale]["decisions"]:
                for lang in ("en", "de"):
                    if old in entry[lang]:
                        entry[lang] = entry[lang].replace(old, new)
                        hit += 1
            print(f"[DEBUG] decisions 1:{scale}: {hit} replaced")
