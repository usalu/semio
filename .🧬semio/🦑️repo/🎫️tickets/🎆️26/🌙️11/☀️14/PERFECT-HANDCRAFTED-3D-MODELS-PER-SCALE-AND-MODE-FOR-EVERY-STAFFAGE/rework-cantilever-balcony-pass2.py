"""🪜 Second round on components/cantilever-balcony: 1:200 wall as concrete core plus insulated skin (as its drawing shows), 1:500 bottom rail raised to the true 80 mm clear gap above the flat slab; edits the volumes source and the 1:500 front drawing of the catalog file: rework-cantilever-balcony-pass2.py <volumes source> <catalog asset>."""
import json
import sys

source_path, catalog_path = sys.argv[1], sys.argv[2]
SLAB_TOP_FLAT = 0.2
RAILING_ORIGIN = 0.1793
CLEAR_GAP = 0.08
RAIL = 0.075
HANDRAIL_BOTTOM = 1.025
rail_centre = round(SLAB_TOP_FLAT + CLEAR_GAP + RAIL / 2 - RAILING_ORIGIN, 4)
baluster_foot = round(rail_centre + RAIL / 2, 4)
baluster_length = round(HANDRAIL_BOTTOM - baluster_foot, 4)

document = json.load(open(source_path))
volumes = document["volumes"]

body200 = volumes["200"]["body"]
index = next(i for i, solid in enumerate(body200) if solid["id"] == "wall")
body200[index:index + 1] = [
    {"id": "wall-core", "kind": "box", "bone": "slab", "material": "concrete", "role": "body", "tone": 0.62, "size": [0.18, 1.3, 1.0], "transform": {"translation": [-0.21, 0.55, 0.0]}, "sectionFaces": ["-v", "+v", "-w", "+w"]},
    {"id": "wall-insulation-render", "kind": "box", "bone": "slab", "material": "mineral-wool", "role": "body", "tone": 0.92, "size": [0.12, 1.1, 1.0], "transform": {"translation": [-0.06, 0.65, 0.0]}, "sectionFaces": ["+v", "-w", "+w"]},
]
volumes["200"]["decisions"][0] = {
    "en": "Body for a 30 mm feature: the wall is its 180 mm concrete core and one 120 mm insulated skin (wool and render merged, as the line in the drawing), the 120 mm break module sits under the skin in front of the core, and the slab is a wedge with its fall. The railing is reduced to what 30 mm can carry: a closed 50 mm post from the slab into the handrail without a foot, the closed 60.3 mm handrail with its top at 1.10 m, two rails 40 x 30 mm and the eight balusters thickened to 30 mm at their true pitch, so the bar field still reads as bars and matches the pen lines of the drawing.",
    "de": "Körper für 30 mm Mindestmaß: Die Wand besteht aus dem Betonkern von 180 mm und einer gedämmten Haut von 120 mm (Wolle und Putz zusammengefasst, wie die Linie der Zeichnung), das Dämmelement von 120 mm sitzt unter der Haut vor dem Kern, die Platte ist ein Keil mit Gefälle. Das Geländer ist auf das reduziert, was 30 mm tragen: ein geschlossener Pfosten von 50 mm von der Platte bis in den Handlauf ohne Fuß, der geschlossene Handlauf von 60,3 mm mit Oberkante auf 1,10 m, zwei Holme 40 x 30 mm und die acht Füllstäbe, auf 30 mm verstärkt, im wahren Raster, damit das Stabfeld als Stäbe lesbar bleibt und zu den Federlinien der Zeichnung passt.",
}

for solid in volumes["500"]["body"]:
    if solid["id"].startswith("railing-bottom-chord"):
        solid["transform"]["translation"][0] = rail_centre
    if solid["id"].startswith("railing-balusters"):
        solid["profile"] = [[0.0, 0.0375], [baluster_length, 0.0375]]
        solid["origin"][0] = baluster_foot
decision = volumes["500"]["decisions"][0]
for language, old, new in (
    ("en", "one 75 mm bottom rail 85 mm above the post foot", "one 75 mm bottom rail with the true 80 mm clear gap above the flat slab (a rail at its true centre line would leave 27 mm, neither open nor closed at this scale)"),
    ("de", "ein unterer Holm von 75 mm 85 mm über dem Pfostenfuß", "ein unterer Holm von 75 mm mit dem wahren lichten Abstand von 80 mm über der ebenen Platte (ein Holm auf seiner wahren Achse ließe 27 mm, in diesem Maßstab weder offen noch geschlossen)"),
):
    assert decision[language].count(old) == 1, old
    decision[language] = decision[language].replace(old, new)
json.dump(document, open(source_path, "w"), ensure_ascii=False, indent=1)

text = open(catalog_path).read()
start = text.index('"denominator": 500')
front = text.index('"front"', start)
plan = text.index('"plan"', front)
segment = text[front:plan]
line_y = round(RAILING_ORIGIN + rail_centre, 4)
assert segment.count("0.2643") == 4, segment.count("0.2643")
segment = segment.replace("0.2643", str(line_y))
text = text[:front] + segment + text[plan:]
open(catalog_path, "w").write(text)
print("[DEBUG] rail centre", rail_centre, "baluster", baluster_foot, baluster_length, "line", line_y)
