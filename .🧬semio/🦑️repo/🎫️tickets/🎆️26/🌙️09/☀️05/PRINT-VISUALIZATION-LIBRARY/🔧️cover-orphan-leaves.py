# 🩹 Re-attaches every taxonomy leaf that no catalogue entry covers to the kind whose slug it
# names, so `verifyVisualizationCoverage` sees a complete cover. Idempotent: run it again after a
# concurrent agent rewrites its own section's entries and drops the cross-section leaves again.
import io, json, os

PRINT = u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint"
CATALOG = os.path.join(PRINT, u"\U0001f5bc\ufe0fassets", u"\U0001f523\ufe0fviz-catalog.json")
TAXONOMY = os.path.join(PRINT, u"\U0001f5bc\ufe0fassets", u"\U0001f523\ufe0fviz-taxonomy.json")
# leaves whose slug names no kind of its own: the kind that stands for them
ALIAS = {
    "61/themeriver": "2/streamgraph",
    "61/sankey": "9/sankey-diagram",
    "76/sankey": "9/sankey-diagram",
}

catalog = json.load(io.open(CATALOG, encoding="utf-8"))
leaves = [entry["id"] for entry in json.load(io.open(TAXONOMY, encoding="utf-8"))]
covered = {leaf for kind in catalog["kinds"] for leaf in kind.get("covers") or []}
by_slug = {kind["slug"]: kind for kind in catalog["kinds"]}
by_id = {kind["id"]: kind for kind in catalog["kinds"]}

added = []
unresolved = []
for leaf in leaves:
    if leaf in covered:
        continue
    slug = leaf.split("/", 1)[1]
    kind = by_id[ALIAS[leaf]] if leaf in ALIAS else by_slug.get(slug)
    if kind is None:
        unresolved.append(leaf)
        continue
    covers = list(kind.get("covers") or [])
    covers.append(leaf)
    covers.sort(key=lambda value: (int(value.split("/")[0]), value))
    kind["covers"] = covers
    added.append((leaf, kind["id"]))

if added:
    tmp = os.path.join(os.path.dirname(CATALOG), "catalog.tmp.json")
    io.open(tmp, "w", encoding="utf-8", newline="\n").write(
        json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")
    os.replace(tmp, CATALOG)

print("attached", len(added), "leaves")
for leaf, kind in added:
    print("  ", leaf, "->", kind)
if unresolved:
    print("UNRESOLVED (no kind with that slug):", unresolved)
