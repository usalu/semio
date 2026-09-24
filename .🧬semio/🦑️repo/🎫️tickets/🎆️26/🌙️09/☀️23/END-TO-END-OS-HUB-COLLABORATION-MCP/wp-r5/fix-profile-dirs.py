import json, os, re
ROOT = "/Users/ueli/Documents/semio"
files = [
 "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/➗️equation/🔮️oracles/🔣️.json",
 "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/📐️geometry/🔮️oracles/🔣️.json",
 "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🔮️oracles/🔣️.json",
 "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracles/🔣️.json",
 "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracles/🔣️.json",
 "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🔮️oracles/🔣️.json",
 "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🔮️oracles/🔣️.json",
]
for rel in files:
    m = re.search(r"🏅️standards/([^/]+)/🪆️subsets/([^/]+)/", rel)
    std, sub = m.group(1), m.group(2)
    p = os.path.join(ROOT, rel); t = open(p, encoding="utf-8").read(); d = json.loads(t)
    for man in d.get("mutationManifests", []):
        before = (man.get("standardDirectoryName"), man.get("subsetDirectoryName"))
        if "standardDirectoryName" in man: man["standardDirectoryName"] = std
        if "subsetDirectoryName" in man: man["subsetDirectoryName"] = sub
        print(rel.split("🪆️subsets/")[1].split("/")[0], before, "->", (man.get("standardDirectoryName"), man.get("subsetDirectoryName")), man.get("subset"))
    n = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    if n != t: open(p, "w", encoding="utf-8").write(n)
