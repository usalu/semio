"""🔺️ Freeze actual indexed publication while excluding independently moving IFC shape writers."""
from pathlib import Path
import json,hashlib

ticket=Path(__file__).parent
artifact=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
boolean=artifact/"📐️geometry/🧊️boolean"
owned=[boolean/name for name in["🟦️.ts","🧪️tests/indexed-publication.test.ts","🧪️tests/indexed-publication-cases.json","🧪️tests/indexed-publication.schema.json"]]
previous=json.loads((ticket/"🗑️generated/boolean-workspace-kernel-source.json").read_text(encoding="utf8"))
references=[]
for item in previous["owned"]+previous["references"]:
 path=Path(item["path"])
 if path in owned:continue
 normalized=path.as_posix()
 if "🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry"in normalized:continue
 references.append(path)
def row(path):return{"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
value={"owned":list(map(row,owned)),"references":list(map(row,references)),"nativeExecuted":False,"nativeTypeChecked":False,"scope":"Actual generic owned flat publication. Moving4x3 source geometry helpers are excluded; no production import of those helpers exists in this generic path."}
assert len(set(owned+references))==len(owned+references)
assert all(item["utf16PathUnits"]<=256 for item in value["owned"]+value["references"])
(ticket/"🗑️generated/indexed-publication-source.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] "+json.dumps({"owned":len(owned),"references":len(references),"maxPath":max(item["utf16PathUnits"]for item in value["owned"]+value["references"])}))
