"""🏗️ Round 5 hand fixes of `machinery/tower-crane` found in the 1:50 zoom renders: the hoist winch house moves from 8 m to 5 m behind the slewing axis (it stood in the counter-jib pendant, which runs from the 8.6 m apex to the tail: at 8 m the pendant is 6.90 m high over a 6.95 m roof; at 5 m it clears the roof by 0.15 m, the place where EC-H class winches sit, right behind the tower head); the motor grille lies on the roof instead of in its plane (z-fighting); the trolley cross beams end inside the side plates instead of flush with their outer faces (z-fighting). Writes the volumes source and, by text surgery, the catalog members `rig.bones[winch].x`, the true-extent symbol's winch house and its stale dimensional note.

@see ./r5-brief.md
@see ./r3-machinery-tower-crane.md
"""
import json
SRC = "volumes/machinery/tower-crane.json"
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
doc = json.load(open(SRC, encoding="utf-8"))
n = 0
for scale, vol in doc["volumes"].items():
    for s in vol["body"]:
        if s["id"].startswith("trolley-beam-") and s["size"][2] == 1.46:
            s["size"][2] = 1.42
            n += 1
        if s["id"] == "winch-motor-grille":
            t = s["size"][1]
            s["transform"]["translation"][1] = round(1.8 + t / 2, 5)
            n += 1
json.dump(doc, open(SRC, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
print("volumes source:", n, "solids")

text = open(CAT, encoding="utf-8").read()
asset = json.loads(text)["asset"]
edits = [
    ('"id": "winch",', '"x": -8,', '"x": -5,'),
    ('"id": "elevation-winch-house"', None, None),
]
i = text.index('"id": "winch",\n')
j = text.index('"x": -8,', i)
assert j - i < 400
text = text[:j] + '"x": -5,' + text[j + len('"x": -8,'):]
for sid in ("elevation-winch-house", "plan-winch-house"):
    k = text.index(f'"id": "{sid}"')
    a = text.rindex("{", 0, k)
    seg_end = text.index("}", k)
    seg = text[a:seg_end]
    assert '"x": -10,' in seg, seg
    text = text[:a] + seg.replace('"x": -10,', '"x": -7,') + text[seg_end:]
old_note = "foundation 7 x 3.5 m"
if old_note in text:
    text = text.replace(old_note, "cross-base 7 x 7 m")
    text = text.replace("Fundament 7 x 3,5 m", "Kreuzrahmen 7 x 7 m")
new = json.loads(text)["asset"]
assert [b for b in new["rig"]["bones"] if b["id"] == "winch"][0]["x"] == -5
open(CAT, "w", encoding="utf-8").write(text)
print("catalog: winch x -8 -> -5, symbol winch house x -10 -> -7")
print([v for v in json.dumps(new["symbols"][0]["dimensionalNote"], ensure_ascii=False).split(";")][:3])
