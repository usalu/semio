"""🍞 Toaster: the heating elements (mica cards wound with resistance ribbon behind their guard wires) as real parts beside the two slots in the thermal list, at every scale."""
from rework_16_lib import load, save, put, drop, extrude, thermal, decide

doc = load("toaster")
CARD = [[-0.066, 0.05], [0.066, 0.05], [0.066, 0.165], [-0.066, 0.165]]
SPANS = {
    "50": [("outer-front", -0.053, -0.047), ("inner-front", -0.013, -0.007), ("inner-rear", 0.007, 0.013), ("outer-rear", 0.047, 0.053)],
    "100": [("outer-front", -0.056, -0.043), ("centre", -0.0125, 0.0125), ("outer-rear", 0.043, 0.056)],
}
BANK = {
    "200": ([[-0.058, -0.074], [0.058, -0.074], [0.058, 0.074], [-0.058, 0.074]], [[[-0.046, -0.066], [-0.014, -0.066], [-0.014, 0.066], [-0.046, 0.066]], [[0.014, -0.066], [0.046, -0.066], [0.046, 0.066], [0.014, 0.066]]]),
    "500": ([[-0.06, -0.074], [0.06, -0.074], [0.06, 0.074], [-0.06, 0.074]], [[[-0.046, -0.066], [0.046, -0.066], [0.046, 0.066], [-0.046, 0.066]]]),
}
for scale, volume in doc["volumes"].items():
    drop(volume["thermal"], *[s["id"] for s in volume["thermal"] if s["id"].startswith("th-element-")])
    if scale in BANK:
        outline, slots = BANK[scale]
        put(volume["thermal"], thermal(extrude("th-element-bank", "base", "mica-steel", outline, "zx", 0.05, 0.155, tone=0.3, holes=slots), "casing", inner=True))
    for name, z0, z1 in SPANS.get(scale, []):
        profile = CARD
        put(volume["thermal"], thermal(extrude(f"th-element-{name}", "base", "mica-steel", profile, "xy", z0, z1, tone=0.3), "casing", inner=True))
    decide(volume, "Thermal: the heat sources are real parts inside the casing, the element cards of 132 x 115 mm (mica wound with resistance ribbon, with their guard wires) on both sides of each bread slot; at 1:100 the two cards between the slots are one centre element, at 1:200 and 1:500 the cards are one element bank around the slots, because a single card is thinner than the smallest feature.", "Thermik: Die Wärmequellen sind echte Bauteile im Gehäuse, die Heizkarten von 132 x 115 mm (Glimmer mit Widerstandsband samt Schutzdrähten) beidseits jedes Brotschlitzes; bei 1:100 sind die zwei Karten zwischen den Schlitzen ein Mittelelement, bei 1:200 und 1:500 bilden die Karten eine Heizbank um die Schlitze, weil eine einzelne Karte dünner ist als das kleinste Merkmal.")
save("toaster", doc)
