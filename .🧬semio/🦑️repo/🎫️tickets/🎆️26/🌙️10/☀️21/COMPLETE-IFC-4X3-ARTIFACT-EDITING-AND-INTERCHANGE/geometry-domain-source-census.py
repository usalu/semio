"""🔏️ Freeze shared consumed-domain matching and its exact circular geometry regression boundary."""
from pathlib import Path
import json,hashlib
ticket=Path(__file__).parent
root=ticket.parents[6]
artifact=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
base=artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any"
io=base/"🚪️io"
original=json.loads((ticket/"🗑️generated/csg-round-source.json").read_text(encoding="utf-8"))
owned=[Path(row["path"])for row in original["owned"]]
owned += [io/"geometry/source.ts",io/"geometry/vector.ts",base/"🧬️schema/⚖️validation/admission.ts",base/"🧬️schema/⚖️validation/type-matching.ts"]
owned += [io/"🧪️tests"/name for name in ["geometry-domain.test.ts","geometry-domain-fixtures.ts","geometry-domain-cases.json","geometry-domain.schema.json"]]
refs=[Path(row["path"])for row in original["references"]if Path(row["path"])not in owned]
refs += [base/"🧬️schema/⚖️validation"/name for name in ["validation.test.ts","control.test.ts","aggregate-kind.test.ts","typeof-select.test.ts","derived-binding.test.ts"]]
def row(path):
 return {"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
result={"owned":list(map(row,owned)),"references":list(map(row,refs)),"nativeExecuted":False,"scope":"Existing structural type matching shared with consumed geometry owner queries, bounded REAL coordinate readers, corrected normative CSG source assumption and exact canonical repair. Raw STEP preservation and semantic NUMBER integer/real remain unchanged. No full IFC/MVD/native claim."}
assert len(set(owned+refs))==len(owned+refs)
assert all(x["utf16PathUnits"]<=256 for x in result["owned"]+result["references"])
(ticket/"🗑️generated/geometry-domain-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
print("[DEBUG]",json.dumps({"owned":len(owned),"references":len(refs),"maxPath":max(x["utf16PathUnits"]for x in result["owned"]+result["references"])}))
