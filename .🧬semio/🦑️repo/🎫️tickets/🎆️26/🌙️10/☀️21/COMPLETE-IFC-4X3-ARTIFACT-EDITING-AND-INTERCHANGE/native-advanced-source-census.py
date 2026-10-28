"""🌪️ Pin the narrow third native Body policy and its shared implementation dependencies."""
from pathlib import Path
import hashlib
import json
ticket = Path(__file__).parent
c = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts"
b = c / "🧱️body"
def row(path):
    """📍️ Read actual bytes and path lengths without retaining source copies."""
    text = str(path)
    return {"path":text,"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(text.encode("utf-16-le"))//2}
owned = [b / name for name in ["🦀️.rs","domain.rs","clause.rs","work.rs","helpers.rs","native-tests.rs","clause-tests.rs","advanced-tests.rs","tessellation-tests.rs","family-tests.rs"]]
refs = [c / "types.rs", c / "native.rs", c / "preparation.rs", c / "population.rs", c.parent / "🦀️.rs"] + [c / "🌪️advanced-swept-solid" / name for name in ["definition.json","cases.json","provenance.json","🟦️.ts"]]
result = {"scope":"native source only; authentic type/runtime/physical receipt proof unavailable","owned":list(map(row,owned)),"references":list(map(row,refs))}
assert all(value["utf16PathUnits"] <=256 for value in result["owned"]+result["references"])
(ticket / "🗑️generated/native-advanced-swept-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding="utf-8")
