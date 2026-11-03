import json
import os
from pathlib import Path

os.chdir("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
P = Path("🔌️appliances/electric-kettle/🔣️.json")
d = json.loads(P.read_text(encoding="utf-8"))
shell = next(r for r in d["asset"]["thermal"]["regions"] if r["id"] == "shell")
shell["heatCapacityJM2K"] = 2765
shell["layers"][0]["heatCapacityJM2K"] = 2765
text = next(a for a in d["asset"]["thermal"]["assumptions"] if "1955 W" in a["en"])
text["en"] = (
    "The 2 kW element heats the water region directly at a constant 1955 W (plate 40, lid 5): 1 kg of water (1 L, 4186 J/(kg K)) rises by about 78 K from the room temperature (326 kJ), because the water under the closed lid starts at the room temperature "
    "(enclosed, no evaporative chilling). The jug shell is not heated by itself: below the fill level (60 percent of the wall) it follows the water through the inside water film of 0.0025 m2K/W, stores 0.7 mm of steel (2765 J/(m2 K)) and loses heat to the room, "
    "all of it taken from the water, so the boil still takes about 184 seconds. The region heat capacity is the water's per gauge area. At 100 degC the water's phase change caps the temperature and the surplus power after the wall losses evaporates it at 2.257 MJ/kg (about 0.8 g/s); "
    "the steam emitter releases exactly this vapour rate."
)
text["de"] = (
    "Das 2-kW-Heizelement heizt die Wasserregion direkt mit konstant 1955 W (Platte 40, Deckel 5): 1 kg Wasser (1 l, 4186 J/(kg K)) steigt von der Raumtemperatur um etwa 78 K (326 kJ), denn das Wasser unter dem geschlossenen Deckel startet mit Raumtemperatur "
    "(eingeschlossen, keine Verdunstungskühlung). Die Kannenhülle wird nicht für sich geheizt: unterhalb des Füllstands (60 Prozent der Wand) folgt sie dem Wasser über den wasserseitigen Film von 0,0025 m2K/W, speichert 0,7 mm Stahl (2765 J/(m2 K)) und gibt Wärme an den Raum ab, "
    "alles aus dem Wasser, sodass das Aufkochen weiterhin etwa 184 Sekunden dauert. Die Wärmekapazität der Region ist die des Wassers je Wasserstandsfläche. Bei 100 °C deckelt die Phasenumwandlung des Wassers die Temperatur und die nach den Wandverlusten überschüssige Leistung verdampft es mit 2,257 MJ/kg (etwa 0,8 g/s); "
    "der Dampfemitter gibt genau diese Dampfrate ab."
)
P.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="")
print("ok")
