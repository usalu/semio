"""📶 Independent schedule for the natural-file publication checkpoint cadence; writes or verifies the neutral fixture."""
import json
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
fixture = next((root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle").glob("📶*checkpoints.json"))
quantum, slice_units, empty_every, polls = 4096, 64, 7, 600
expected, threshold = [], quantum
for poll in range(1, polls + 1):
    work = (poll - poll // empty_every) * slice_units
    if work >= threshold:
        expected.append([poll, work])
        threshold = (work // quantum + 1) * quantum
if expected[-1][0] != polls:
    expected.append([polls, (polls - polls // empty_every) * slice_units])
legacy = polls - polls // empty_every
document = json.loads(fixture.read_text())
if "--write" in sys.argv:
    document.update(quantum=quantum, sliceUnits=slice_units, emptySliceEvery=empty_every, polls=polls, legacyCheckpoints=legacy, expectedCheckpoints=expected)
    fixture.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
assert document["expectedCheckpoints"] == expected and document["legacyCheckpoints"] == legacy
print(f"{legacy} legacy checkpoints -> {len(expected)}: {expected}")
