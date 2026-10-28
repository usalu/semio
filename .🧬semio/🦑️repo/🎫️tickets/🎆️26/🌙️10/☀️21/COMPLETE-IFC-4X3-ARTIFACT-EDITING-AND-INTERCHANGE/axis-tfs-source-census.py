"""🔏️ Freeze the actual broad-checkpoint placement/REAL integration correction."""
from pathlib import Path
import json,hashlib
ticket=Path(__file__).parent
artifact=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
base=artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any"
io=base/"🚪️io"
owned=[io/"geometry.ts"]+[io/"🧪️tests"/name for name in ["axis.test.ts","tfs-admission-cases.json","tfs-admission.test.ts"]]
refs=[io/"geometry"/name for name in ["source.ts","vector.ts","triangulated.ts"]]
refs+=[io/name for name in ["control.ts","exchange-source.ts","gltf.ts","orientation.ts"]]
refs+=[io/"🧪️tests"/name for name in ["axis-cases.json","axis.schema.json","axis-fixtures.ts","tfs-admission.schema.json","tfs-admission-fixtures.ts","geometry-domain.test.ts","csg-round.test.ts"]]
refs+=[base/"🧬️schema/⚖️validation"/name for name in ["admission.ts","type-matching.ts"]]
refs+=[base/"🧬️schema/📚️catalog/🟦️.ts"]
def row(path):return {"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
value={"owned":list(map(row,owned)),"references":list(map(row,refs)),"nativeExecuted":False,"scope":"Restore owned placement diagnostics without weakening old19 axis obligations; explicitly correct one original TFS32 REAL carrier expectation and prove actual canonical repair/inverses/STEP/GLB. Original ticket corpus unchanged; failed full145 retained."}
assert len(set(owned+refs))==len(owned+refs)
assert all(item["utf16PathUnits"]<=256 for item in value["owned"]+value["references"])
(ticket/"🗑️generated/axis-tfs-source.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG]",json.dumps({"owned":len(owned),"references":len(refs),"maxPath":max(item["utf16PathUnits"]for item in value["owned"]+value["references"])}))

