"""🫧️ Pin current native SurfaceModel policy inputs without source copying or platform-specific roots."""
from pathlib import Path
import hashlib
import json
ticket = Path(__file__).parent
c = ticket.parents[6] / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts"
b = c / "🧱️body"
def row(path):
    """🔖️ Preserve actual source hashes and UTF16 path bounds for independent review."""
    text = str(path)
    return {"path":text,"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(text.encode("utf-16-le"))//2}
owned = [b / name for name in ["🦀️.rs","domain.rs","clause.rs","work.rs","helpers.rs","native-tests.rs","clause-tests.rs","advanced-tests.rs","surface-tests.rs","tessellation-tests.rs","family-tests.rs"]]
refs = [c / "types.rs", c / "native.rs", c / "preparation.rs", c / "population.rs", c.parent / "🦀️.rs"] + [c / "🫧️surface-model" / name for name in ["definition.json","cases.json","provenance.json","🟦️.ts"]]
result = {"scope":"native source only; no authentic type/runtime/physical receipts","owned":list(map(row,owned)),"references":list(map(row,refs))}
assert all(value["utf16PathUnits"] <=256 for value in result["owned"]+result["references"])
(ticket / "🗑️generated/native-surface-model-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding="utf-8")
