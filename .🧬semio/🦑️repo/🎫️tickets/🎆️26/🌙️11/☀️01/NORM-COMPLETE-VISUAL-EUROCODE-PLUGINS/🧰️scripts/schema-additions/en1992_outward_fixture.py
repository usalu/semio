"""en1992 flat slab with outward eccentricities (EN 1992-1-1 6.4.3(4) and (5)): edge and corner columns whose unbalanced moment moves the load towards the free edge must still be verified (general Eq. 6.39 with the full u1), here failing; one mild outward moment passes at the inward floor."""
import copy, json, pathlib
FX = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
document = json.loads((FX / "flat-slab-punching" / "🔣️.json").read_text(encoding="utf8"))
by_id = {m["id"]: m for m in document["members"]}
rows = [("slab-FS-edge", "slab-FS-edge-outward-fail", "Flat slab edge column, eccentricity towards the free edge", "Flachdecke Randstütze, Ausmitte zum freien Rand", -400e3, 0.0),
        ("slab-FS-edge", "slab-FS-edge-outward-parallel-fail", "Flat slab edge column, outward and parallel moments", "Flachdecke Randstütze, Momente nach außen und parallel", -400e3, 30e3),
        ("slab-FS-edge", "slab-FS-edge-outward-mild", "Flat slab edge column, mild outward moment", "Flachdecke Randstütze, kleines Moment nach außen", -20e3, 0.0),
        ("slab-FS-corner", "slab-FS-corner-outward-fail", "Flat slab corner column, eccentricity towards the free edge", "Flachdecke Eckstütze, Ausmitte zum freien Rand", -170e3, 0.0),
        ("slab-FS-corner", "slab-FS-corner-outward-mixed-fail", "Flat slab corner column, outward along c2 and inward along c1", "Flachdecke Eckstütze, außen entlang c2 und innen entlang c1", 30e3, -180e3)]
members = []
for source, ident, label_en, label_de, m1, m2 in rows:
    member = copy.deepcopy(by_id[source])
    member.update({"id": ident, "labelEn": label_en, "labelDe": label_de})
    member["punching"].update({"betaMethod": "UnbalancedMoments", "momentC1": m1, "momentC2": m2})
    members.append(member)
document["members"] = members
document["title"] = "Flat slab with outward column eccentricities"
target = FX / "flat-slab-punching-outward" / "🔣️.json"
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
print(target)
