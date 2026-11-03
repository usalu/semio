"""🧫 Writes the EN 1998 check-outcome fixtures `geotechnics-pass` and `geotechnics-fail` (copy of `families-pass`: a pile through two layers with a stiffness contrast above 6 so the kinematic moment counts, a free-head pile, loose layers and a high water table in the failing copy, an EN annex copy for the NA-DE free registry values)."""
import copy, json, pathlib
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
passing = json.load(open(base / "families-pass/🔣️.json", encoding="utf8"))
fd1, fd2 = passing["foundations"]
fd1.update(pileLowerVsMS=450.0, pileMomentRdNm=900000.0)
fd2.update(pileDiameterM=0.5, pileLengthM=14.0, pileModulusPa=3.0e10, pileSubgradeModulusNM3=1.5e7, pileHeadShearN=90000.0, pileHeadFixed=0.0, pileInterfaceDepthM=0.0, pileMomentRdNm=250000.0)
failing = copy.deepcopy(passing)
f1, f2 = failing["foundations"]
f1.update(liqN160=6.0, liqWaterDepthM=1.0, pileMomentRdNm=250000.0, pileLengthM=7.0)
f2.update(liqN160=9.0, pileMomentRdNm=60000.0, pileHeadShearN=150000.0)
for layer in failing["soilLayers"]:
    if layer["foundationId"] == "fd-1" and layer["id"] in ("l1", "l3"):
        layer["n160"] = 7.0
for name, doc in (("geotechnics-pass", passing), ("geotechnics-fail", failing)):
    out = base / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")
