"""Independent check of the en1994 scene goldens (building, bridge, showcase) against their documents: every entity id and count is derived from the document rows, never from the projection. The showcase document is the committed JSON; the building and bridge documents are `En1994Snapshot::example(De)` and `bridge_girder()`, whose rows are beam-B1, col-C1, slab-S1 and 52 studs `beam-B1-stud-<n>` (typical_row)."""
import json, sys, collections

ANY = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
import os
FIX = os.environ.get("GOLDEN_DIR", f"{ANY}/🎬️scene/🧪️tests/🧫️fixtures")
FLAT = "GOLDEN_DIR" in os.environ
example = {"beams": ["beam-B1"], "columns": ["col-C1"], "slabs": ["slab-S1"], "placements": [], "cases": [], "studs": [("beam-B1", f"beam-B1-stud-{n}") for n in range(52)]}
showcase = json.load(open(f"{ANY}/🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"))
documents = {
    "🏛️scene": example,
    "🌉️bridge": example,
    "🎪️showcase": {"beams": [r["id"] for r in showcase["beams"]], "columns": [r["id"] for r in showcase["columns"]], "slabs": [r["id"] for r in showcase["slabs"]], "placements": [r["id"] for r in showcase["placements"]], "cases": [r["id"] for r in showcase["cases"]], "studs": [(s["beamId"], s["id"]) for s in showcase["studs"]]},
}
status = 0
for name, doc in documents.items():
    path = f"{FIX}/{name}.json" if FLAT else f"{FIX}/{name}/🔣️.json"
    if not os.path.exists(path):
        print(f"{name}: not recorded")
        continue
    text = open(path).read()
    if not text.strip():
        print(f"{name}: golden is empty")
        status = 1
        continue
    scene = json.loads(text)
    ids = [e["entity"]["id"] for e in scene["entities"]]
    problems = []
    dup = [i for i, n in collections.Counter(ids).items() if n > 1]
    if dup:
        problems.append(f"duplicate entity ids {dup}")
    rows = set(doc["beams"] + doc["columns"] + doc["slabs"] + doc["placements"] + doc["cases"])
    bases = {i.split(":")[0] for i in ids}
    phantom = sorted(b for b in bases if b not in rows and b != "project-site")
    if phantom:
        problems.append(f"entities whose base id is no document row {phantom}")
    for row in doc["beams"] + doc["columns"] + doc["slabs"]:
        if row not in ids:
            problems.append(f"missing entity of row {row}")
    for beam in doc["beams"]:
        studs = [s for b, s in doc["studs"] if b == beam]
        drawn = {i for i in ids if i.startswith(f"{beam}:stud-")}
        want = {f"{beam}:stud-{s}" for s in studs}
        if drawn != want:
            problems.append(f"{beam}: {len(drawn)} stud entities for {len(want)} stud rows (missing {sorted(want - drawn)[:3]}, extra {sorted(drawn - want)[:3]})")
    degenerate = [a["data"]["id"] for a in scene.get("annotations", []) if a.get("kind") == "dimension" and a["data"]["from"] == a["data"]["to"]]
    if degenerate:
        problems.append(f"degenerate dimensions {degenerate}")
    print(f"{name}: {len(ids)} entities, {len(rows)} document rows, {len(problems)} problems")
    for p in problems:
        print(" -", p)
    status |= bool(problems)
sys.exit(status)
