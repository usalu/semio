import copy, json, sys
fixtures = sys.argv[1]
def edit(leaf, change):
    for role in ("before", "after"):
        path = f"{fixtures}/{leaf}/✅apply/🔣️.{role}.json"
        text = open(path, encoding="utf-8").read()
        document = json.loads(text)
        assert json.dumps(document, indent=2, ensure_ascii=False) + "\n" == text, path
        change(document, role)
        open(path, "w", encoding="utf-8", newline="\n").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
def vertex(document, role):
    if role == "before":
        next(v for v in document["outlineVertices"] if v["id"] == "v-1")["elementId"] = "flange-top"
def ring(document, role):
    document["shells"].append({**copy.deepcopy(document["shells"][0]), "id": "shell-alt"})
    if role == "before":
        next(r for r in document["ringStiffeners"] if r["id"] == "ring-1")["shellId"] = "shell-alt"
edit("🔷️change-vertex-element-id", vertex)
edit("⭕️change-ring-shell-id", ring)
