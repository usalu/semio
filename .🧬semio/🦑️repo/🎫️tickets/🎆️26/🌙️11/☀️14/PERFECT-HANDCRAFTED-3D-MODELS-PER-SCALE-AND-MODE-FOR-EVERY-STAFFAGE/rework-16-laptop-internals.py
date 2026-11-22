"""💻 Laptop: the heat sources as real parts inside the base of the thermal list at 1:50, the logic board with its heat spreader under the keyboard and the battery pack under the palm rest."""
from rework_16_lib import load, save, put, box, thermal, decide

doc = load("laptop")
fine = doc["volumes"]["50"]
put(fine["thermal"], thermal(box("th-logic-board", "base", "keycap-plastic", [0.078, 0.0075, 0.2], [0.061, 0.00725, 0.0], tone=0.25), "deck", inner=True), after="th-deck")
put(fine["thermal"], thermal(box("th-battery", "base", "keycap-plastic", [0.084, 0.0075, 0.25], [0.158, 0.00725, 0.0], tone=0.4), "underside", inner=True), after="th-logic-board")
decide(fine, "Thermal: the heat sources are real parts inside the 12.5 mm base, the logic board with its heat spreader (200 x 78 mm) under the keyboard next to the hinge and the battery pack (250 x 84 mm) under the palm rest, both 7.5 mm thick; at 1:100 and coarser the base is thinner than the smallest feature and deck and underside carry the heat alone.", "Thermik: Die Wärmequellen sind echte Bauteile im 12,5 mm starken Unterteil, die Hauptplatine mit Wärmespreizer (200 x 78 mm) unter der Tastatur am Scharnier und der Akku (250 x 84 mm) unter der Handballenauflage, beide 7,5 mm stark; ab 1:100 ist das Unterteil dünner als das kleinste Merkmal, Deck und Unterseite tragen die Wärme allein.")
save("laptop", doc)
