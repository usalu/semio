import json, os, sys
path = sys.argv[1]
with open(path, encoding="utf-8") as f:
    text = f.read()
block = '    "closure.derived-provider-differs": {\n      "en": "The derived command walk was recorded under another script provider designation: {provider}",\n      "de": "Der abgeleitete Befehlsdurchlauf wurde unter einer anderen Skriptanbieter-Bezeichnung aufgezeichnet: {provider}"\n    },\n'
assert text.count(block) == 1, text.count(block)
out = text.replace(block, "")
data = json.loads(out)
assert "closure.derived-provider-differs" not in data["messages"]
tmp = os.path.join(os.path.dirname(path), ".🔣️.json.F101.tmp")
with open(tmp, "w", encoding="utf-8", newline="") as f:
    f.write(out)
os.replace(tmp, path)
print("removed; messages now", len(data["messages"]))
