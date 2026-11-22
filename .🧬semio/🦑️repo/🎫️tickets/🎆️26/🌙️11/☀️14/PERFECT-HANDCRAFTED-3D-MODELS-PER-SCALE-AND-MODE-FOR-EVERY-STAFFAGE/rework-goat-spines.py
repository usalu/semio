"""🦴 Chisel of the goat author: the thoracic column of `animals/goat` at 1:50 as bodies with thirteen spinous processes.

Frame of the thorax bone (u from the last thoracic vertebra to the first, v towards the back line). Each row is
(u of the vertebra, v of the tip of its process, lean of the tip towards the loin in metres): long and swept back at
the withers (T2 to T5), shortening to the upright anticlinal vertebra T11. The supraspinous ligament ties the tips,
as the nuchal ligament does in front of them; the muscular sling of the blades holds on to it.
"""
import json
import sys

SOURCE = "volumes/animals/goat.json"
BODY = (0.014, 0.013)
SPINES = [
    (0.3250, 0.060, 0.018), (0.2983, 0.072, 0.028), (0.2716, 0.074, 0.031), (0.2450, 0.068, 0.030), (0.2183, 0.060, 0.028),
    (0.1916, 0.054, 0.024), (0.1650, 0.047, 0.020), (0.1383, 0.042, 0.016), (0.1116, 0.037, 0.012), (0.0850, 0.034, 0.006),
    (0.0583, 0.031, 0.0), (0.0317, 0.027, 0.0), (0.0050, 0.024, 0.0),
]
RADIUS = 0.0042


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    structure = document["volumes"]["50"]["structure"]
    structure[:] = [e for e in structure if not e.get("id", "").startswith(("spinous-t", "supraspinous-"))]
    index = next(i for i, e in enumerate(structure) if e.get("id") == "thoracic")
    column = structure[index]
    for section in column["sections"]:
        section["at"] = [section["at"][0], 0.0, 0]
        section["rx"], section["ry"] = BODY
    base = {k: column[k] for k in ("bone", "material", "role", "tone", "member", "part") if k in column}
    added = []
    for number, (u, tip, lean) in enumerate(SPINES, start=1):
        added.append({"id": "spinous-t%d" % number, "kind": "capsule", **base, "p0": [u, 0.006, 0], "p1": [round(u - lean, 5), tip, 0], "r0": 0.0052, "r1": RADIUS})
    tips = [(round(u - lean, 5), tip) for u, tip, lean in SPINES]
    for number, (a, b) in enumerate(zip(tips, tips[1:]), start=1):
        added.append({"id": "supraspinous-%d" % number, "kind": "capsule", **base, "p0": [a[0], a[1], 0], "p1": [b[0], b[1], 0], "r0": RADIUS, "r1": RADIUS})
    structure[index + 1:index + 1] = added
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] thoracic spines written", file=sys.stderr)


main()
