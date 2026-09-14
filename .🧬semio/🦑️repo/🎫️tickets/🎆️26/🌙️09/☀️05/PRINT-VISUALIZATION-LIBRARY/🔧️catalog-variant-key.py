# 🏷️ Gives every catalogue entry the `variant` option the schema requires: the entry's own slug,
# which is the convention the 462 entries that already carry one follow.
import io, json, os

CATALOG = (u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/"
           u"\U0001f5bc\ufe0fassets/\U0001f523\ufe0fviz-catalog.json")

catalog = json.load(io.open(CATALOG, encoding="utf-8"))
added = 0
mismatched = []
for kind in catalog["kinds"]:
    options = kind["options"]
    current = options.get("variant")
    if isinstance(current, str):
        if current != kind["slug"]:
            mismatched.append((kind["slug"], current))
        continue
    kind["options"] = dict([("variant", kind["slug"])] + list(options.items()))
    added += 1

tmp = os.path.join(os.path.dirname(CATALOG), "catalog.tmp.json")
io.open(tmp, "w", encoding="utf-8", newline="\n").write(json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")
os.replace(tmp, CATALOG)
print("added variant to", added, "entries")
if mismatched:
    print("variant differs from slug (left alone):", mismatched)
