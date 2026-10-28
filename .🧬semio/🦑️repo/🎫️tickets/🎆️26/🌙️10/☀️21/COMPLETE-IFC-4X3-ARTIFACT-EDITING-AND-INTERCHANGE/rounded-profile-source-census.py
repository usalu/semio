"""🔲️ Freeze the actual convex boundary and rounded IFC producer with bounded geometric references."""
from pathlib import Path
import json,hashlib

ticket=Path(__file__).parent
artifact=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
io=artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"
convex=artifact/"📐️geometry/🔲️convex"
owned=[convex/name for name in["🟦️.ts","🦀️.rs","🧪️tests/cases.json","🧪️tests/cases.schema.json","🧪️tests/🟦️.test.ts"]]+[artifact/"📐️geometry/🦀️.rs"]+[io/name for name in["geometry.ts","geometry/circular-solid.ts","geometry/convex-solid.ts","geometry/rounded.ts","🧪️tests/rounded-rectangle-cases.json","🧪️tests/rounded-rectangle.schema.json","🧪️tests/rounded-rectangle-fixtures.ts","🧪️tests/rounded-rectangle.test.ts","🧪️tests/rounded-workflow-cases.json","🧪️tests/rounded-workflow-cases.schema.json","🧪️tests/rounded-workflow.test.ts"]]
previous=json.loads((ticket/"🗑️generated/csg-primitive-source.json").read_text(encoding="utf8"))
references=[Path(row["path"])for row in previous["owned"]+previous["references"]if Path(row["path"])not in owned and "🧊️boolean"not in row["path"]]
def row(path):return{"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
value={"owned":list(map(row,owned)),"references":list(map(row,references)),"nativeTypeChecked":False,"nativeExecuted":False,"scope":"Paired pure scalar boundary and actual TS rounded extrusion; native module mounting and rustfmt do not establish native execution or funding.","excludedReference":"Unimported Boolean kernel is changing independently and is outside this producer closure."}
assert len(set(owned+references))==len(owned+references)
assert all(item["utf16PathUnits"]<=256 for item in value["owned"]+value["references"])
(ticket/"🗑️generated/rounded-profile-source.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] "+json.dumps({"owned":len(owned),"references":len(references),"maximumPath":max(row["utf16PathUnits"]for row in value["owned"]+value["references"])}))
