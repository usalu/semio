"""🧥️ Second round, pass 1 of `humans/skateboarder`: one shoulder line at 1:50 and 1:100.

The sleeve heads stood 60 mm above the shoulder joint as separate lobes with a notch towards the neck. The trunk of the hoodie
now carries the shoulder seam out over the acromion (half breadths stated below against the height along the trunk) and the
sleeve head is flattened to 50 mm above the joint, so the seam runs in one slope from the neck to the shoulder point. The
thermal shells of the upper arm follow with the same factor.

Usage: python3 rework-skateboarder-r2-shoulder.py   (edits T/volumes/humans/skateboarder.json in place; run once on the state after rework-skateboarder-r2-coarse.py)
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "skateboarder.json")
SHOULDER_LINE = {
    "50": {0.43: 0.2, 0.455: 0.198, 0.478: 0.19, 0.496: 0.166, 0.508: 0.132, 0.519: 0.098},
    "100": {0.44: 0.198, 0.478: 0.188, 0.497: 0.162, 0.51: 0.128},
}
HEAD_FROM = -0.015
HEAD_FACTOR = 0.05 / 0.06


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    volumes = document["volumes"]
    for scale, line in SHOULDER_LINE.items():
        trunk = next(s for s in volumes[scale]["body"] if s["id"] == "hoodie")
        for section in trunk["sections"]:
            if section["at"][0] in line:
                section["rx"] = line[section["at"][0]]
    count = 0
    for scale in ("50", "100"):
        for name in ("body", "thermal"):
            for solid in volumes[scale][name]:
                if solid.get("kind") == "loft" and solid.get("bone") in ("upper-arm-l", "upper-arm-r"):
                    for section in solid["sections"]:
                        if section["at"][0] < HEAD_FROM:
                            section["at"][0] = round(section["at"][0] * HEAD_FACTOR, 5)
                            count += 1
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("sleeve head stations moved:", count)
    for scale in ("50", "100"):
        for name in ("body", "thermal"):
            print(scale, name, [(s["id"], [x["at"][0] for x in s["sections"][:4]]) for s in volumes[scale][name] if s.get("kind") == "loft" and s.get("bone") == "upper-arm-l"])
        print(scale, "trunk", [(s["id"], [(x["at"][0], x["rx"]) for x in s["sections"][-7:]]) for s in volumes[scale]["body"] + volumes[scale]["thermal"] if s.get("bone") == "torso" and s.get("kind") == "loft" and len(s["sections"]) > 6])


main()
