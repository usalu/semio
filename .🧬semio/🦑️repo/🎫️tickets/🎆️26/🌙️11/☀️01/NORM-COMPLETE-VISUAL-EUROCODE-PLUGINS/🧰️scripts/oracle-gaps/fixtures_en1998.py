"""🧫 Writes the EN 1998 check-outcome fixture `seismic-structure-fail` (copy of seismic-rich-pass with a thin plan, an unbacked multiple-system claim, a duplicate analysis direction, a Rayleigh period without stiffnesses and an irregular elevation)."""
import copy, json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
d = json.load(open(root / "seismic-rich-pass/🔣️.json", encoding="utf8"))
office, annex = d["buildings"]
office["planWidthM"] = 0.005
office["multipleResistingSystems"] = True
office["systems"][1]["direction"] = "x"
col, beam = office["members"][0], office["members"][1]
col.update(minDimensionM=0.2, omegaWd=0.01, rhoPrime=0.05, detailingCompatibleWithQ=False)
beam.update(minDimensionM=0.15, rhoPrime=0.001)
annex["t1Method"] = "rayleigh"
annex["elevationRegular"] = False
for st in annex["storeys"]:
    st["stiffnessX"] = 0.0
    st["stiffnessY"] = 0.0
out = root / "seismic-structure-fail"
out.mkdir(exist_ok=True)
json.dump(d, open(out / "🔣️.json", "w", encoding="utf8"), ensure_ascii=False, indent=2)
open(out / "🔣️.json", "a").write("\n")
