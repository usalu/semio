"""🛏️ single-bed 1:500 structure: the centre leg carries the 40 x 65 mm cross beam between the side rails (true section, `slender`), as at the finer scales.

Before, the leg member ended 65 mm under the slat plates with nothing on it. Run, then `volumes-insert.py`.
"""
import json

SOURCE = __file__.rsplit("/", 1)[0] + "/volumes/furniture/single-bed.json"
source = json.load(open(SOURCE))
scale = source["volumes"]["500"]
structure = scale["structure"]
assert not any(s["id"] == "member-cross-beam" for s in structure)
index = next(i for i, s in enumerate(structure) if s["id"] == "member-leg-mid")
structure.insert(index, {"id": "member-cross-beam", "kind": "box", "bone": "frame", "material": "pine-timber", "role": "body", "tone": 0.5, "size": [0.04, 0.065, 0.91], "transform": {"translation": [1.06, 0.1725, 0.0]}, "member": "frame", "part": "member", "slender": True})
scale["decisions"][1] = {
    "en": "Thermal: panels, tray and leg are the frame surface, the mattress blocks the mattress surface, the slatted base is one hidden plate inside the tray. Structure: four posts, two side rails, the two end rails, the cross beam at its true 40 x 65 mm section on the centre leg and three slat plates as members; glides; bearing strips and masses of the mattress parts.",
    "de": "Thermik: Platten, Wanne und Fuß sind die Rahmenoberfläche, die Matratzenblöcke die Matratzenoberfläche, der Lattenrost ist eine verdeckte Platte in der Wanne. Tragwerk: vier Pfosten, zwei Seitenholme, die beiden Querriegel, der Mittelsteg im wahren Querschnitt 40 x 65 mm auf dem Mittelfuß und drei Rostplatten als Stäbe; Gleiter; Tragstreifen und Massen der Matratzenteile.",
}
open(SOURCE, "w").write(json.dumps(source, ensure_ascii=False, indent=1))
print("1:500 cross beam written")
