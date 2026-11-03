"""en1992 flat slab with openings next to the columns (EN 1992-1-1 6.4.2(3), Figure 6.14): interior, edge and corner columns with an opening within 6 d (perimeter reduced between the tangents), one with an opening beyond 6 d (unchanged), one with two overlapping openings. Prints the oracle rows for the feature table."""
import copy, json, pathlib, sys
FX = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
document = json.loads((FX / "flat-slab-punching" / "🔣️.json").read_text(encoding="utf8"))
by_id = {m["id"]: m for m in document["members"]}
def opening(ident, x, y, width, depth):
    return {"id": ident, "x": x, "y": y, "width": width, "depth": depth}
rows = [("slab-FS-interior", "slab-FS-interior-opening-near", "Flat slab interior column, opening within 6 d", "Flachdecke Innenstütze, Öffnung innerhalb 6 d", [opening("op-1", 0.9, 0.1, 0.4, 0.3)]),
        ("slab-FS-interior", "slab-FS-interior-opening-far", "Flat slab interior column, opening beyond 6 d", "Flachdecke Innenstütze, Öffnung jenseits 6 d", [opening("op-1", 2.5, 0.0, 0.4, 0.3)]),
        ("slab-FS-interior", "slab-FS-interior-opening-two", "Flat slab interior column, two overlapping openings", "Flachdecke Innenstütze, zwei überlappende Öffnungen", [opening("op-1", 0.9, 0.1, 0.4, 0.3), opening("op-2", 0.6, 0.8, 0.5, 0.4)]),
        ("slab-FS-edge", "slab-FS-edge-opening", "Flat slab edge column, opening within 6 d", "Flachdecke Randstütze, Öffnung innerhalb 6 d", [opening("op-1", -0.8, 0.5, 0.3, 0.3)]),
        ("slab-FS-corner", "slab-FS-corner-opening", "Flat slab corner column, opening within 6 d", "Flachdecke Eckstütze, Öffnung innerhalb 6 d", [opening("op-1", -0.8, -0.3, 0.3, 0.3)])]
members = []
for source, ident, label_en, label_de, openings in rows:
    member = copy.deepcopy(by_id[source])
    member.update({"id": ident, "labelEn": label_en, "labelDe": label_de, "openings": openings})
    members.append(member)
document["members"] = members
document["title"] = "Flat slab with openings next to the columns"
target = FX / "flat-slab-punching-openings" / "🔣️.json"
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
print(target)
if "--rows" in sys.argv:
    sys.path.insert(0, str(pathlib.Path(str(target)).parents[4] / "🔮️oracles" / "🚦️check-outcome"))
    import importlib.util
    spec = importlib.util.spec_from_file_location("oracle", pathlib.Path(str(target)).parents[4] / "🔮️oracles" / "🚦️check-outcome" / "🐍️.py")
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    for row in oracle.evaluate(document)["checks"]:
        if ".6.4." in row["id"] or "studs" in row["id"]:
            print(row["id"], round(float(row["utilization"]), 9))
