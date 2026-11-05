"""🪪️ Independently validate graph preview source identities and committed carriers."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve()
while not (ROOT / ".git").exists():
    ROOT = ROOT.parent

def read(path):
    return json.loads((ROOT / path).read_text(encoding="utf-8"))

shared = "✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️preview-revisions/"
cases = read(shared + "🔣️.json")
for case in cases["cases"]:
    source, current = case["sourceRevision"], case["currentRevision"]
    assert (current is not None and len(source) == 32 and bytes(source) == bytes(current)) == case["visible"], case["id"]
checked = 0
for artifact, fill in [("◻️2d", "🔣️fill"), ("🧊️3d", "📑️fill")]:
    base = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/" + artifact + "/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
    tool = base + "🎭️modes/✏️edit/🛠️tools/" + fill + "/"
    transient = base + "🫧️transient/"
    fixture = transient + "🧪️tests/🧫️fixtures/🏁️set-solve/✅️set-solve-applied/"
    before = read(fixture + "📸️snapshot/⬅️before/🔣️.json")
    mutation = read(fixture + "🦠️mutation/🔣️.json")["SetSolve"]
    after = read(fixture + "📸️snapshot/➡️after/🔣️.json")
    diff = read(fixture + "🔺️diff/🔣️.json")
    expected = dict(before)
    expected.update(mutation)
    assert expected == after
    assert {**before, **diff} == after
    assert before["sourceRevision"] == [] and len(after["sourceRevision"]) == 32
    checked += 1
print("validated", len(cases["cases"]), "portable revision cases and", checked, "graph mutation carriers with the independent byte comparison")
