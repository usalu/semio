"""🔏️ Freeze cumulative shared Boolean sessions against the closed standalone corpus."""
from pathlib import Path
import json,hashlib
ticket=Path(__file__).parent
artifact=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
boolean=artifact/"📐️geometry/🧊️boolean"
owned=[boolean/"🟦️.ts"]+[boolean/"🧪️tests"/name for name in ["session.test.ts","session-cases.json","session.schema.json"]]
old=json.loads((ticket/"🗑️generated/boolean-cooperation-source.json").read_text(encoding="utf8"))
refs=[Path(item["path"])for item in old["owned"]+old["references"]if Path(item["path"])not in owned]
def row(path):return {"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
value={"owned":list(map(row,owned)),"references":list(map(row,refs)),"nativeExecuted":False,"scope":"One cumulative work session, fixed source checkpoints, guarded child lifecycle/reentrancy and first-failure identity. Standalone APIs reuse it. No live workspace reservation or IFC Boolean adapter yet."}
assert len(set(owned+refs))==len(owned+refs)
assert all(item["utf16PathUnits"]<=256 for item in value["owned"]+value["references"])
(ticket/"🗑️generated/boolean-session-source.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG]",json.dumps({"owned":len(owned),"references":len(refs),"maxPath":max(item["utf16PathUnits"]for item in value["owned"]+value["references"])}))

