import json

path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json"
doc = json.load(open(path, encoding="utf-8"))
print(doc["required"])
