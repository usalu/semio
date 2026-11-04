import json

path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json"
doc = json.load(open(path, encoding="utf-8"))
for name in ["placements", "jointPlates"]:
    print(name, json.dumps(doc["properties"][name], ensure_ascii=False)[:500])
    print("---")
