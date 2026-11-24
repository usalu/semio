import json, os, sys
produced, fixtures = sys.argv[1:3]
count = 0
for name in sorted(os.listdir(produced)):
    leaf, case = name[:-5].split("__")
    target = os.path.join(fixtures, leaf, case, "🔣️.diff.json")
    assert os.path.isfile(target), target
    with open(os.path.join(produced, name), encoding="utf-8") as source:
        value = json.load(source)
    with open(target, "w", encoding="utf-8", newline="\n") as out:
        out.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")
    count += 1
print(count)
