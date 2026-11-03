"""💡 pa: OI-28 - the pendant lamp stays a 60 W incandescent lamp (the clips draw the hot A60 bulb as free heat): 95 % (57 W) leave in its surfaces, 3 W as light absorbed by the room; cap area 0.005 m2 - run once."""
from jio import load, save

path, data = load("pendant-lamp")
a = data["asset"]
regions = {r["id"]: r for r in a["thermal"]["regions"]}
regions["bulb-glass"]["operating"]["heatW"] = 34
regions["shade"]["operating"]["heatW"] = 17
regions["lampholder"]["operating"]["heatW"] = 6
regions["lampholder"]["areaM2"] = 0.005

pairs = {
    "en": [
        ("almost all of the 60 W ends as heat, the glass and the shade are the warm surfaces.", "95 % (57 W) leave as heat in its surfaces and the visible 3 W warm the room where the walls absorb them, so all 60 W end as heat in the room; the glass and the shade are the warm surfaces."),
        ("36 W at the glass (about 130 to 160 degrees Celsius), 18 W at the shade that the radiation and the rising hot air warm, 6 W at the cap and the collar; the split is an authored estimate.", "34 W at the glass (about 130 to 160 degrees Celsius), 17 W at the shade that the radiation and the rising hot air warm, 6 W at the cap and the collar (0.005 m2, an E27 cap of 27 mm diameter with its collar, about 100 degrees Celsius); together 57 W, 95 % of the rating, and the remaining 3 W leave as visible light that the room surfaces absorb; the split is an authored estimate."),
        ("The 60 W end entirely as heat in the room: the glass releases 36 W, the shade 18 W and the cap with the collar 6 W at level 1; the light output of about 3 W is absorbed by room surfaces and counted in these surfaces.", "The 60 W end entirely as heat in the room: the glass releases 34 W, the shade 17 W and the cap with the collar 6 W at level 1 (57 W, 95 %); the light output of about 3 W leaves the lamp as radiation and is absorbed by the room surfaces, where it is counted."),
    ],
    "de": [
        ("fast die gesamten 60 W enden als Wärme, Glas und Schirm sind die warmen Flächen.", "95 % (57 W) verlassen die Flächen als Wärme und die sichtbaren 3 W erwärmen den Raum dort, wo die Wände sie absorbieren, sodass alle 60 W als Wärme im Raum enden; Glas und Schirm sind die warmen Flächen."),
        ("36 W am Glas (etwa 130 bis 160 Grad Celsius), 18 W am Schirm, den Strahlung und aufsteigende Heißluft erwärmen, 6 W an Sockel und Hülse; die Aufteilung ist ein Schätzwert.", "34 W am Glas (etwa 130 bis 160 Grad Celsius), 17 W am Schirm, den Strahlung und aufsteigende Heißluft erwärmen, 6 W an Sockel und Hülse (0,005 m2, ein E27-Sockel von 27 mm Durchmesser mit Hülse, etwa 100 Grad Celsius); zusammen 57 W, 95 % der Nennleistung, und die übrigen 3 W verlassen die Leuchte als sichtbares Licht, das die Raumflächen absorbieren; die Aufteilung ist ein Schätzwert."),
        ("Die 60 W enden vollständig als Wärme im Raum: das Glas gibt bei Stufe 1 36 W ab, der Schirm 18 W und Sockel mit Hülse 6 W; die Lichtleistung von etwa 3 W wird von den Raumflächen absorbiert und ist in diesen Flächen mitgezählt.", "Die 60 W enden vollständig als Wärme im Raum: das Glas gibt bei Stufe 1 34 W ab, der Schirm 17 W und Sockel mit Hülse 6 W (57 W, 95 %); die Lichtleistung von etwa 3 W verlässt die Leuchte als Strahlung und wird von den Raumflächen absorbiert und dort mitgezählt."),
    ],
}

def patch(holder):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)

patch(a["summary"])
patch(a["provenance"]["notes"])
patch(a["thermal"]["assumptions"][0])
save(path, data)
print("ok")
