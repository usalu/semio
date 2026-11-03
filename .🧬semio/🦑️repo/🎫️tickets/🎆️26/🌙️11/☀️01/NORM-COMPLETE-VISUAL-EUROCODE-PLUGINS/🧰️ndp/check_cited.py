import re, json, pathlib, sys
ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
REG = {e["id"] for e in json.load(open("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json"))["entries"]}
bad = {}
for f in ART.rglob("*.rs"):
    for i in re.findall(r'"(ndp\.en199\d\.[a-z0-9.\-]*[a-z0-9])"', f.read_text(errors="ignore")):
        if i not in REG and not any(r.startswith(i + '.') for r in REG): bad.setdefault(i, set()).add(f.name)
print(len(REG), "missing:", sorted(bad)[:40], len(bad))
