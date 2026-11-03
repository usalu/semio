"""🧫 Writes the EN 1997 check-outcome fixtures `footing-trio-pass` and `footing-trio-fail` (geotechnical-rich-pass with a third and a fourth placed footing (the plane through three points leaves no residual), so the settlement plane is fitted and the relative rotation rows exist; the fail variant loads F4 twelve times harder)."""
import copy, json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
for name, factor, factor4 in (("footing-trio-pass", 1.0, 1.0), ("footing-trio-fail", 1.0, 12.0)):
    d = json.load(open(root / "geotechnical-rich-pass/🔣️.json", encoding="utf8"))
    third = copy.deepcopy(d["footings"][1])
    third["id"] = "footing-F3"
    for key in ("labelEn", "labelDe"):
        if key in third:
            third[key] = "Footing F3" if key == "labelEn" else "Fundament F3"
    for lc in third["loadCases"]:
        lc["verticalPermanent"] *= factor
        lc["verticalVariable"] *= factor
    fourth = copy.deepcopy(third)
    fourth["id"] = "footing-F4"
    for lc in fourth["loadCases"]:
        lc["verticalPermanent"] *= factor4 / factor
        lc["verticalVariable"] *= factor4 / factor
    d["footings"] += [third, fourth]
    d["placements"] += [{"id": "footing-F3", "x": 3.0, "y": 6.0, "z": 0.0, "yaw": 0.0}, {"id": "footing-F4", "x": -4.0, "y": 5.0, "z": 0.0, "yaw": 0.0}]
    out = root / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(d, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
