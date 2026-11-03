"""🧫 Writes the EN 1997 check-outcome fixtures `footing-clay-pass` and `footing-clay-fail` (geotechnical-rich-pass with a stiff clay under the footings in a transient situation, so the undrained bearing (D.3) and sliding rows exist)."""
import json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
for name, factor in (("footing-clay-pass", 1.0), ("footing-clay-fail", 4.0)):
    d = json.load(open(root / "geotechnical-rich-pass/🔣️.json", encoding="utf8"))
    d["designSituation"] = "transient"
    sand = d["layers"][0]
    sand.update(soilType="clay", cohesionUndrained=80000.0, phiPrimeDeg=22.0)
    for footing in d["footings"]:
        for lc in footing["loadCases"]:
            lc["designSituation"] = "transient"
            lc["verticalPermanent"] *= factor
            lc["verticalVariable"] *= factor
    out = root / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(d, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
