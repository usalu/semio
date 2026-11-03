"""en1992 flat slab with unbalanced column moments (EN 1992-1-1 6.4.3(3) to (5)): the committed `flat-slab-punching` document with the method from unbalanced moments on every column (interior one axis, interior both axes, edge with a parallel moment, corner) next to the simplified one."""
import copy, json, pathlib
FX = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
document = json.loads((FX / "flat-slab-punching" / "🔣️.json").read_text(encoding="utf8"))
by_id = {m["id"]: m for m in document["members"]}
both = copy.deepcopy(by_id["slab-FS-interior"])
both.update({"id": "slab-FS-interior-both", "labelEn": "Flat slab interior column, moments about both axes", "labelDe": "Flachdecke Innenstütze, Momente um beide Achsen"})
moments = {"slab-FS-interior": (60e3, 0.0), "slab-FS-interior-both": (40e3, 25e3), "slab-FS-edge": (20e3, 30e3), "slab-FS-corner": (10e3, 10e3)}
members = [by_id["slab-FS-interior"], both, by_id["slab-FS-edge"], by_id["slab-FS-corner"]]
for member in members:
    m1, m2 = moments[member["id"]]
    member["punching"].update({"betaMethod": "UnbalancedMoments", "momentC1": m1, "momentC2": m2})
document["members"] = members
document["title"] = "Flat slab with unbalanced column moments"
target = FX / "flat-slab-punching-moments" / "🔣️.json"
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
print(target)
