from author_33_lib import *
slug = "kneeling-gardener"
d = load(slug)
v = d["volumes"]
a50, a100 = v["50"]["decisions"], v["100"]["decisions"]
a50[0] = D(a50[0]["en"].replace("a straw hat whose brim is a revolved disc", "a straw hat (a dome crown on a 9.5 mm thin brim that droops 31 mm towards the rim, revolved as a hollow dish, with a hat band) over a hair mass on the back and top of the skull").replace("4 % fuller", "4 % fuller"),
           a50[0]["de"].replace("einen Strohhut, dessen Krempe eine Rotationsscheibe ist", "einen Strohhut (Kuppelkrone auf einer 9,5 mm dünnen, zum Rand hin um 31 mm abfallenden Krempe, als hohle Schale gedreht, mit Hutband) über einer Haarmasse an Hinterkopf und Scheitel"))
a50[1] = D(a50[1]["en"].replace("the hat exposes 0.45 m2 against 0.226 m2 because the drawn straw brim is a 0.47 m disc of which both faces count.", "the hat region (straw 0.36 m2 including the hair mass) exposes 0.36 m2 against 0.226 m2: the declared value counts the sky-facing side of the brim once (0.143 m2 of brim + 0.075 m2 crown and band = 0.218), while the exposed surface counts both faces of the brim (hand calculation in area-review.md). The hair mass buries the skull skin so that the face region exposes 0.06 m2 of 0.0685."),
           a50[1]["de"].replace("der Hut legt 0,45 m2 frei gegenüber 0,226 m2, weil die gezeichnete Strohkrempe eine Scheibe von 0,47 m ist, deren beide Seiten zählen.", "die Hutregion (Stroh samt Haarmasse) legt 0,36 m2 frei gegenüber 0,226 m2: der deklarierte Wert zählt die himmelwärts gewandte Seite der Krempe einmal (0,143 m2 Krempe + 0,075 m2 Krone und Band = 0,218), die freiliegende Fläche zählt beide Krempenseiten (Handrechnung in area-review.md). Die Haarmasse deckt die Schädelhaut ab, sodass die Gesichtsregion 0,06 m2 von 0,0685 freilegt."))
a100[0] = D(a100[0]["en"] + " The hat is the same dished brim (15.5 mm thick at this scale) and dome crown over a hair mass.", a100[0]["de"] + " Der Hut ist dieselbe Krempe als Schale (hier 15,5 mm dick) mit Kuppelkrone über einer Haarmasse.")
save(slug, d)
for n in (50, 100): print(n, [(x["en"][:40]) for x in v[str(n)]["decisions"]])
