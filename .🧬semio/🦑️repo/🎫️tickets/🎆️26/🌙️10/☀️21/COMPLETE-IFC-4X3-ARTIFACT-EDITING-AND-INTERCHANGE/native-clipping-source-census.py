"""✂️ Freeze native selected Clipping source without copying any implementation."""
from pathlib import Path
import hashlib
import json
ticket=Path(__file__).parent
c=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts"
b=c/"🧱️body"
def row(path):
    """🎯️ Observe exact bytes and cross-platform path limits."""
    text=str(path)
    return {"path":text,"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(text.encode("utf-16-le"))//2}
owned=[b/name for name in ["🦀️.rs","domain.rs","clause.rs","work.rs","helpers.rs","native-tests.rs","clause-tests.rs","advanced-tests.rs","surface-tests.rs","clipping-tests.rs","tessellation-tests.rs","family-tests.rs"]]
refs=[c/"types.rs",c/"native.rs",c/"preparation.rs",c/"population.rs",c.parent/"🦀️.rs"]+[c/"✂️clipping"/name for name in ["definition.json","cases.json","provenance.json","🟦️.ts"]]
result={"scope":"native source only, no authentic type/runtime/receipt/output proof","owned":list(map(row,owned)),"references":list(map(row,refs))}
assert all(item["utf16PathUnits"]<=256 for item in result["owned"]+result["references"])
(ticket/"🗑️generated/native-clipping-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding="utf-8")
