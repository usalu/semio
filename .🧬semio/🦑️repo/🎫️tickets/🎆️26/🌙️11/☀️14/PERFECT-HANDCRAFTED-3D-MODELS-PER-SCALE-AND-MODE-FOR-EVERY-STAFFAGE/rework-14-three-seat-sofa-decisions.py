"""🛋️ Author 14, three-seat sofa: the structure sentences of the decisions after rework-14-three-seat-sofa-structure.py, English first, German second.

Run from the ticket folder: python3 rework-14-three-seat-sofa-decisions.py
"""
import json

PATH = "volumes/furniture/three-seat-sofa.json"
TEXT = {
    "50": (
        "Structure: the frame as it is built and as it carries, every member bearing on the next. Four turned feet stand under the arm frames; per arm two posts, a top and a bottom rail from 0.100 m up and the inner ply panel; the plinth's two base rails (25 x 110 mm beech) run through the arm frames on the same level and carry the 18 mm ply deck on their top edges; the back frame (bottom rail, four uprights, top rail, an 8 mm ply face on the uprights) stands on the deck and is bolted through the arm panels into the arm back posts; per loose cushion its foam bearing band (30 x 150 mm along the cushion) lies on the deck under the sitting line or leans at lumbar height. Joints: corner blocks from the base rails into the arm frames, the back bolts, the contact pads of the seat cushions on the deck and of the back cushions through the batting onto the ply face. Supports: four felt glides at the foot anchors. Every bone carries its mass at its centre of mass.",
        "Tragwerk: das Gestell, wie es gebaut ist und trägt, jedes Glied liegt auf dem nächsten auf. Vier gedrechselte Füße stehen unter den Armgestellen; je Arm zwei Pfosten, ein oberer und ein unterer Riegel ab 0,100 m und die innere Sperrholzwand; die zwei Sockelzargen (Buche 25 x 110 mm) laufen auf gleicher Höhe durch die Armgestelle und tragen auf ihren Oberkanten den 18-mm-Sperrholzboden; das Rückengestell (Fußriegel, vier Pfosten, Kopfriegel, 8 mm Sperrholz auf den Pfosten) steht auf dem Boden und ist durch die Armwände in die hinteren Armpfosten verschraubt; je losem Kissen liegt sein Schaumauflagerband (30 x 150 mm längs des Kissens) unter der Sitzlinie auf dem Boden oder lehnt in Lendenhöhe an. Gelenke: Eckklötze von den Sockelzargen in die Armgestelle, die Rückenschrauben, die Kontaktflächen der Sitzkissen auf dem Boden und der Rückenkissen durch die Wattierung auf das Sperrholz. Auflager: vier Filzgleiter an den Fußankern. Jeder Knochen trägt seine Masse in seinem Schwerpunkt.",
    ),
    "100": (
        "Structure: the same frame for 15 mm, every member bearing on the next: feet under the arm frames, base rails through the arms with the deck on their top edges, the back frame (bottom rail, two end uprights, top rail and a 15 mm ply face) standing on the deck and bolted into the arm back posts, per arm posts and rails, the foam bearing bands, corner blocks, back bolts and cushion pads as joints, glides, masses.",
        "Tragwerk: dasselbe Gestell für 15 mm, jedes Glied liegt auf dem nächsten auf: Füße unter den Armgestellen, Sockelzargen durch die Arme mit dem Boden auf ihren Oberkanten, das Rückengestell (Fußriegel, zwei Endpfosten, Kopfriegel und 15 mm Sperrholz) auf dem Boden stehend und in die hinteren Armpfosten verschraubt, je Arm Pfosten und Riegel, die Schaumauflagerbänder, Eckklötze, Rückenschrauben und Kissenflächen als Gelenke, Gleiter, Massen.",
    ),
    "200": (
        "Structure: feet, base rails standing on them with a 30 mm deck on their top edges, one back panel standing on the deck, one 30 mm panel per arm as its frame, the seat bearing bands on the deck and the back cushions as blocks that lean on the back panel, the arm corners as joints, glides, masses.",
        "Tragwerk: Füße, darauf die Sockelzargen mit einem 30-mm-Boden auf ihren Oberkanten, eine Rückwand auf dem Boden stehend, je Arm eine 30-mm-Platte als Gestell, die Sitzauflagerbänder auf dem Boden und die Rückenkissen als Blöcke, die an der Rückwand lehnen, die Armecken als Gelenke, Gleiter, Massen.",
    ),
    "500": (
        "Structure: the feet, two 75 mm base rails with the 75 mm ply deck between them under the seat bands, a 75 mm back panel on which the bearing blocks of the back cushions lean, a 75 mm panel per arm, 75 mm bearing bands of the seat cushions, glides of the foot size, masses.",
        "Tragwerk: die Füße, zwei 75-mm-Sockelzargen mit dem 75-mm-Sperrholzboden dazwischen unter den Sitzbändern, eine 75-mm-Rückwand, an der die Auflagerblöcke der Rückenkissen lehnen, je Arm eine 75-mm-Platte, 75-mm-Auflagerbänder der Sitzkissen, Gleiter in Fußgröße, Massen.",
    ),
}

document = json.load(open(PATH))
for scale, (en, de) in TEXT.items():
    decision = document["volumes"][scale]["decisions"][-1]
    for language, text, mark in (("en", en, "Structure:"), ("de", de, "Tragwerk:")):
        head = decision[language].split(mark)[0]
        assert head != decision[language], (scale, language)
        decision[language] = head + text
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
