"""🧾️ Freeze actual native shared Body inputs without copying source into the ticket."""
from pathlib import Path
import hashlib
import json
ticket = Path(__file__).parent
c = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts"
body = c / "🧱️body"
def record(path):
    """📋️ Pin current source bytes and both platform path bounds."""
    raw = path.read_bytes()
    text = str(path)
    return {"path":text, "sha256":hashlib.sha256(raw).hexdigest(), "bytes":len(raw), "utf16PathUnits":len(text.encode("utf-16-le"))//2}
owned = [body / name for name in ["🦀️.rs", "domain.rs", "clause.rs", "work.rs", "helpers.rs", "native-tests.rs", "clause-tests.rs", "tessellation-tests.rs", "family-tests.rs"]]
refs = [c / "types.rs", c / "native.rs", c / "preparation.rs", c / "population.rs", c.parent / "🦀️.rs", body / "family-cases.json", c / "🌊️swept-solid/cases.json", c / "🔺️tessellation/cases.json", c / "🔺️tessellation/query-cases.json"]
result = {"scope":"source-only; no native type/runtime/receipt/heap proof", "owned":list(map(record, owned)), "references":list(map(record, refs)), "removed":[str(c / "🔺️tessellation" / name) for name in ["🦀️.rs","helpers.rs","🧪️.rs"]]}
assert all(row["utf16PathUnits"] <= 256 for row in result["owned"] + result["references"])
assert all(not Path(path).exists() for path in result["removed"])
(ticket / "🗑️generated" / "native-body-clause-source.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
