"""🧫 Writes the EN 1990 check-outcome fixture `bridge-life-rc-warning` (road-bridge-showcase-pass with a 60 a design working life that is outside the 100 a of Annex A2.1(1), and consequence class 3 over reliability class RC2, an Annex B.3 override)."""
import json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
d = json.load(open(root / "road-bridge-showcase-pass/🔣️.json", encoding="utf8"))
d.update(designWorkingLifeYears=60.0, consequenceClass=3)
out = root / "bridge-life-rc-warning"
out.mkdir(exist_ok=True)
with open(out / "🔣️.json", "w", encoding="utf8") as handle:
    json.dump(d, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
