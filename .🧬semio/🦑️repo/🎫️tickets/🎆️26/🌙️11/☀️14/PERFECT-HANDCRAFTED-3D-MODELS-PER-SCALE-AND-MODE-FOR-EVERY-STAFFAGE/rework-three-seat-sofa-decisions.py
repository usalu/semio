"""📝 Three-seat sofa, author pass 1: the decision texts of the thermal and structure lists at 1:50, 1:100 and 1:200.

Run from the ticket folder: python3 rework-three-seat-sofa-decisions.py
@see ./rework-three-seat-sofa-frame.py
"""
import json

SOURCE = "volumes/furniture/three-seat-sofa.json"
TEXT = {
    "50": {
        "en": "Thermal: every region is its body surface with its layers nested inside at their real build-up and in their own material (HR foam cores of the cushions, polyester batting of the arms and of the back frame, the 18 mm ply core of the back frame ending under the top roll, the ply core of the plinth under the fabric), feet as beech. Structure, the real load path of a hardwood sofa frame: four turned feet as posts under the arm frames; per arm a front and a back board post, a top and a bottom rail and the inner ply panel, the bottom rail and the posts reaching down to the arm base at 0.10 m so that they sit on the feet; the two base rails (25 x 90 mm beech) running through the arm frames, tied by two cross stretchers (25 x 90 mm) under the two cushion joints, which halve the 0.8 m span of the 18 mm ply deck; the back frame of a bottom rail, four raked uprights sawn from the board (115 mm deep at the deck, 50 mm under the top rail, rear edge flush with the rails), the top rail on their heads and a 12 mm ply face on their raked front edges that takes the back cushions; per loose cushion its foam bearing band (30 x 150 mm) on the deck under the sitting line or on the frame face at lumbar height. Joints: corner blocks of the base rails in the arm frames, bolts of the back frame into the arm back posts, the contact pads of the cushions. Supports: four felt glides at the foot anchors. Every bone carries its mass at its centre. Serpentine springs or webbing are not modelled: the deck is a ply platform.",
        "de": "Thermik: jede Region ist ihre Körperoberfläche, die Schichten liegen in ihrem realen Aufbau und in ihrem eigenen Material darin (HR-Schaumkerne der Kissen, Polyestervlies der Armlehnen und des Rückenrahmens, der 18-mm-Sperrholzkern des Rückenrahmens endet unter der Oberrolle, der Sperrholzkern des Sockels unter dem Stoff), Füße aus Buche. Tragwerk, der reale Lastpfad eines Hartholz-Sofagestells: vier gedrechselte Füße als Stützen unter den Armrahmen; je Armlehne ein vorderer und ein hinterer Brettpfosten, eine obere und eine untere Zarge und die innere Sperrholzplatte, untere Zarge und Pfosten reichen bis zur Armunterkante auf 0,10 m und sitzen damit auf den Füßen; die zwei Sockelschienen (25 x 90 mm Buche) laufen durch die Armrahmen und sind unter den beiden Kissenfugen durch zwei Querriegel (25 x 90 mm) verbunden, die die 0,8-m-Spannweite des 18-mm-Sperrholzdecks halbieren; der Rückenrahmen aus unterer Zarge, vier geneigten, aus dem Brett gesägten Pfosten (am Deck 115 mm tief, unter der oberen Zarge 50 mm, Hinterkante bündig mit den Zargen), der oberen Zarge auf ihren Köpfen und einer 12-mm-Sperrholzfläche auf ihren geneigten Vorderkanten, die die Rückenkissen trägt; je losem Kissen sein Schaum-Auflagerband (30 x 150 mm) auf dem Deck unter der Sitzlinie oder an der Rahmenfläche auf Lendenhöhe. Gelenke: Eckklötze der Sockelschienen in den Armrahmen, Schrauben des Rückenrahmens in die hinteren Armpfosten, die Kontaktflächen der Kissen. Auflager: vier Filzgleiter an den Fußankern. Jeder Knochen trägt seine Masse im Schwerpunkt. Wellenfedern oder Gurte sind nicht modelliert: das Deck ist eine Sperrholzplattform.",
    },
    "100": {
        "en": "Thermal: every region is its body surface with its layers nested inside in their own material (foam cores of the cushions, batting of the arms and of the back frame, the ply cores of the back frame and of the plinth); the 1 mm cover is kept at its real thickness, so the layers read in the cut and not on the surface. Structure: the same frame for 15 mm, every member bearing on the next: feet under the arm frames, base rails through the arms tied by two cross stretchers under the cushion joints, the deck on their top edges, the back frame (bottom rail, two end uprights, top rail and a 15 mm ply face) standing on the deck and bolted into the arm back posts, per arm posts and rails, the foam bearing bands, corner blocks, back bolts and cushion pads as joints, glides, masses.",
        "de": "Thermik: jede Region ist ihre Körperoberfläche, die Schichten liegen in ihrem eigenen Material darin (Schaumkerne der Kissen, Vlies der Armlehnen und des Rückenrahmens, die Sperrholzkerne von Rückenrahmen und Sockel); der 1-mm-Bezug behält seine reale Dicke, die Schichten zeigen sich deshalb im Schnitt und nicht an der Oberfläche. Tragwerk: derselbe Rahmen für 15 mm, jedes Glied lagert auf dem nächsten: Füße unter den Armrahmen, Sockelschienen durch die Armlehnen, unter den Kissenfugen durch zwei Querriegel verbunden, das Deck auf ihren Oberkanten, der Rückenrahmen (untere Zarge, zwei Endpfosten, obere Zarge und eine 15-mm-Sperrholzfläche) steht auf dem Deck und ist in die hinteren Armpfosten geschraubt, je Armlehne Pfosten und Zargen, die Schaum-Auflagerbänder, Eckklötze, Rückenschrauben und Kissenflächen als Gelenke, Gleiter, Massen.",
    },
    "200": {
        "en": "Thermal: every region is its body surface with its core layers nested inside in their own material (foam, batting, ply), read in the cut. Structure: feet, base rails standing on them and tied by two 30 mm cross stretchers under the cushion joints, a 30 mm deck on their top edges, one back panel standing on the deck, one 30 mm panel per arm as its frame, the seat bearing bands on the deck and the back cushions as blocks that lean on the back panel, the arm corners as joints, glides, masses.",
        "de": "Thermik: jede Region ist ihre Körperoberfläche, die Kernschichten liegen in ihrem eigenen Material darin (Schaum, Vlies, Sperrholz) und zeigen sich im Schnitt. Tragwerk: Füße, darauf die Sockelschienen, unter den Kissenfugen durch zwei 30-mm-Querriegel verbunden, ein 30-mm-Deck auf ihren Oberkanten, eine Rückenplatte auf dem Deck, je Armlehne eine 30-mm-Platte als Rahmen, die Sitz-Auflagerbänder auf dem Deck und die Rückenkissen als Blöcke, die an der Rückenplatte lehnen, die Armecken als Gelenke, Gleiter, Massen.",
    },
}


CROWN = {
    "50": {
        "en": ("with a 10 mm crown and", "with a 20 mm domed crown above the top boxing seam (the top panel rises from the seam to the middle, no flat plateau) and"),
        "de": ("mit 10 mm Bombierung und", "mit 20 mm gewölbter Bombierung über der oberen Boxnaht (die Deckplatte steigt von der Naht zur Mitte, kein flaches Plateau) und"),
    },
    "100": {
        "en": ("seat cushions crowned,", "seat cushions with a 20 mm domed crown in four stations,"),
        "de": ("Sitzkissen bombiert,", "Sitzkissen mit 20 mm gewölbter Bombierung in vier Stationen,"),
    },
}


def main():
    """✍️ Replace the second decision of the three finer scales and name the domed crown in the first."""
    doc = json.load(open(SOURCE, encoding="utf-8"))
    for scale, text in TEXT.items():
        doc["volumes"][scale]["decisions"][1] = text
    for scale, swaps in CROWN.items():
        for lang, (old, new) in swaps.items():
            body = doc["volumes"][scale]["decisions"][0]
            assert old in body[lang] or new in body[lang], (scale, lang)
            body[lang] = body[lang].replace(old, new)
    json.dump(doc, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("decisions written", list(TEXT))


main()
