import json

path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json"
doc = json.load(open(path, encoding="utf-8"))
props = doc["properties"]
required = set(doc.get("required", []))
names = [
    "jointPlates",
    "foundationBlocks",
    "materialDuctility",
    "formingGains",
    "bucklingLengths",
    "fatigueStresses",
    "frameActions",
    "frameMemberBases",
    "fastenedJoints",
    "placements",
]
for name in names:
    value = props.get(name)
    print(name, "required" if name in required else "optional", "missing" if value is None else list(value)[:4])
