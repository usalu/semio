"""🫖 Electric kettle: the concealed underfloor heating element as a real disc under the jug floor in the thermal list at 1:100 as well (124 mm across, with its thermostat 15 mm deep)."""
from rework_16_lib import load, save, put, revolve, thermal, decide

doc = load("electric-kettle")
mid = doc["volumes"]["100"]
put(mid["thermal"], thermal(revolve("th-element", "body", "stainless", [[0.0, 0.0], [0.0, 0.062], [0.015, 0.062], [0.015, 0.0]], [0, 1, 0], [0.0, 0.0, 0.0], role="detail", tone=0.3), "plate", inner=True), after="th-base")
decide(mid, "Thermal: the heat source is a real part, the concealed element disc with its thermostat (124 mm across, 15 mm deep) under the jug floor below the water.", "Thermik: Die Wärmequelle ist ein echtes Bauteil, die verdeckte Heizscheibe mit Thermostat (124 mm Durchmesser, 15 mm stark) unter dem Kannenboden unterhalb des Wassers.")
for scale in ("200", "500"):
    decide(doc["volumes"][scale], "Thermal: the 14 mm element disc under the jug floor is thinner than the smallest feature of this scale; the base plate region carries its heat.", "Thermik: Die 14 mm starke Heizscheibe unter dem Kannenboden ist dünner als das kleinste Merkmal dieses Massstabs; die Region der Sockelplatte trägt ihre Wärme.")
save("electric-kettle", doc)
