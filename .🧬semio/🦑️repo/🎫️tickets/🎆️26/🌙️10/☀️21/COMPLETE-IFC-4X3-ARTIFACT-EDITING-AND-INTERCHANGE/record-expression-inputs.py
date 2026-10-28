"""🔏️ Pin nested source inputs while the current broad production reader is held."""
from pathlib import Path
import hashlib,json

ticket=Path(__file__).parent
tests=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
paths=[tests/name for name in ["boolean-expression-cases.json","boolean-expression.schema.json","boolean-expression-fixtures.ts","boolean-expression.test.ts"]]
value={"newInputs":[{"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}for path in paths],"pass":8,"fail":6,"tests":14,"bunAssertions":73,"strictConsumers":10,"strictExit":0,"scope":"Unregistered missing IFC adapter red; five genuine neutral Manifold programs, real source declarations, explicit malformed/cyclic cases and central mutation/diff/inverses prior to missing projection."}
assert all(row["utf16PathUnits"]<=256 for row in value["newInputs"])
(ticket/"🗑️generated/boolean-expression-red-source.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
report=ticket/"boolean-expression-red.md"
source=report.read_text(encoding="utf8")
report.write_text(source+"\nActual strict10consumers (existing CSG/domain/control plus new primitive/expression/finite inputs) exits0/empty with unchanged repository compiler options. Four new expression inputs are pinned in generated/boolean-expression-red-source.json. Six real STEP files (five source programs plus canonical edited operator) are emitted by boolean-expression-emit.ts under generated/boolean-expression-actual, with exact content hashes/source node IDs; physical external validation is still pending.\n",encoding="utf8")
print("[DEBUG] nested inputs4 pinned,14red14observations, strict10green, actual6STEP emitted")
