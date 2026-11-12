"""Renames the non-slug `volume` solid ids of vegetation assets to lower case (they mirrored capital-letter bone ids), in the catalog file and the ticket source volumes/<category>/<slug>.json, by exact replacement of `"id": "<old>"` inside the volume text only. Usage: catalog-fix-solid-ids.py <category-folder/slug>..."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from catalog_edit import S
SLUG = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
T = os.path.dirname(os.path.abspath(__file__))
for spec in sys.argv[1:]:
    path = S + spec + "/🔣️.json"
    doc = json.load(open(path, encoding="utf-8"))
    ids = [s["id"] for s in doc["asset"]["volume"]["solids"]]
    mapping = {i: i.lower() for i in ids if not SLUG.match(i)}
    assert all(SLUG.match(v) for v in mapping.values()), mapping
    new = [mapping.get(i, i) for i in ids]
    assert len(set(new)) == len(new), "collision"
    def fix(text):
        i = text.index('"volume": {')
        head, vol = text[:i], text[i:]
        for old, nw in mapping.items(): vol = vol.replace(f'"id": "{old}"', f'"id": "{nw}"')
        return head + vol
    raw = open(path, encoding="utf-8").read()
    out = fix(raw)
    assert open(path, encoding="utf-8").read() == raw
    open(path, "w", encoding="utf-8").write(out); json.loads(out)
    src = os.path.join(T, "volumes", spec.split("/")[0].lstrip("🌳️🏗️🐕️👤️🔌️🚜️🛋️🧱️🪑️"), spec.split("/")[1] + ".json")
    if os.path.exists(src):
        t = open(src, encoding="utf-8").read()
        for old, nw in mapping.items(): t = t.replace(f'"id": "{old}"', f'"id": "{nw}"')
        open(src, "w", encoding="utf-8").write(t)
    print(spec, len(mapping), "solid ids renamed")
