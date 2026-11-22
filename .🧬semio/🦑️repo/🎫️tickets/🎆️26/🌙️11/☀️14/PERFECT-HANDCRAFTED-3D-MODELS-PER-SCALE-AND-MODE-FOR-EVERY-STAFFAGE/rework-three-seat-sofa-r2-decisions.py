"""📝 Second round, pass 1 of `furniture/three-seat-sofa`: the decisions of every scale name what this pass modelled (welts and seams at 1:50, the seat foam as spring block and sinking slab, the raked back uprights at 1:100, the named cover layer of the thermal surfaces).

Usage: python3 rework-three-seat-sofa-r2-decisions.py
"""
import json

PATH = "volumes/furniture/three-seat-sofa.json"
MARK = {"en": " Seat foam:", "de": " Sitzschaum:"}
FOAM = {
    "en": " Seat foam: under every seat cushion a spring block of foam stands on the deck (bound to the frame) and a 90 mm slab above it moves with the cushion, so a loaded cushion presses the slab into the block and nothing passes through the deck; the contact pad lies on the deck.",
    "de": " Sitzschaum: unter jedem Sitzkissen steht ein Federblock aus Schaum auf dem Deck (am Rahmen gebunden), darüber bewegt sich eine 90-mm-Platte mit dem Kissen, sodass ein belastetes Kissen die Platte in den Block drückt und nichts durch das Deck tritt; die Kontaktfläche liegt auf dem Deck.",
}
FOAM_COARSE = {
    "en": " Seat foam: a spring block on the deck (bound to the frame) under a 90 mm slab that moves with the cushion, so a loaded cushion never passes through the deck.",
    "de": " Sitzschaum: ein Federblock auf dem Deck (am Rahmen gebunden) unter einer 90-mm-Platte, die sich mit dem Kissen bewegt, sodass ein belastetes Kissen nie durch das Deck tritt.",
}
EXTRA = {
    "50": [
        {
            "en": " Sewing: an 8 mm welt runs up the sides and over the top roll of the front and the back panel of each arm (flush at the bottom edge), and each back cushion carries a knife-edge seam along its mid line, 3 mm proud, that follows the belly and the leaning crown.",
            "de": " Nähte: ein 8-mm-Keder läuft an Vorder- und Rückspiegel jeder Armlehne die Seiten hinauf und über die obere Rolle (unten bündig), und jedes Rückenkissen trägt entlang seiner Mittellinie eine 3 mm vorstehende Messerkantennaht, die dem Bauch und der geneigten Krone folgt.",
        },
        {"en": FOAM["en"] + " Thermal surfaces name the cover layer `fabric`.", "de": FOAM["de"] + " Thermische Oberflächen benennen die Bezugsschicht `fabric`."},
    ],
    "100": [
        {"en": " Arm welts and back cushion seams are omitted (8 mm below the feature).", "de": " Armkeder und Rückenkissennähte entfallen (8 mm unter dem Merkmal)."},
        {
            "en": FOAM["en"] + " Back frame: four uprights sawn to the rake (115 mm at the deck, 50 mm under the top rail) with a 15 mm ply face on their leaning edges, as at 1:50. Thermal surfaces name the cover layer `fabric`.",
            "de": FOAM["de"] + " Rückenrahmen: vier auf die Neigung gesägte Pfosten (115 mm am Deck, 50 mm unter der Oberschiene) mit einer 15-mm-Sperrholzfläche auf den geneigten Kanten, wie bei 1:50. Thermische Oberflächen benennen die Bezugsschicht `fabric`.",
        },
    ],
    "200": [None, {"en": FOAM_COARSE["en"] + " Thermal surfaces name the cover layer `fabric`.", "de": FOAM_COARSE["de"] + " Thermische Oberflächen benennen die Bezugsschicht `fabric`."}],
    "500": [
        None,
        {
            "en": FOAM_COARSE["en"] + " The thermal blocks are the cover surfaces of their regions (material and layer `fabric`).",
            "de": FOAM_COARSE["de"] + " Die thermischen Blöcke sind die Bezugsoberflächen ihrer Regionen (Material und Schicht `fabric`).",
        },
    ],
}
GUARD = [{"en": " Sewing:", "de": " Nähte:"}, MARK]
GUARD_100 = [{"en": " Arm welts and back", "de": " Armkeder und"}, MARK]

with open(PATH, encoding="utf-8") as handle:
    source = json.load(handle)
for scale, extras in EXTRA.items():
    decisions = source["volumes"][scale]["decisions"]
    guards = GUARD if scale == "50" else GUARD_100 if scale == "100" else [None, MARK]
    for index, extra in enumerate(extras):
        if extra is None:
            continue
        for language in ("en", "de"):
            if guards[index][language] not in decisions[index][language]:
                decisions[index][language] += extra[language]
with open(PATH, "w", encoding="utf-8") as handle:
    json.dump(source, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
