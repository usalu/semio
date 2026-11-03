"""🧫 Writes the EN 1990 check-outcome fixture `annex-tests-known` (annex-all-pass with test series of known coefficient of variation: n = 10 and n = 2)."""
import copy, json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
d = json.load(open(root / "annex-all-pass/🔣️.json", encoding="utf8"))
first = d["testSeries"][0]
first.update(variationKnown=0.1)
second = copy.deepcopy(first)
second.update(id="tes-2", sampleSize=2)
d["testSeries"] = [first, second]
out = root / "annex-tests-known"
out.mkdir(exist_ok=True)
with open(out / "🔣️.json", "w", encoding="utf8") as handle:
    json.dump(d, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
