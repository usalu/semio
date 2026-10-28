from pathlib import Path
import json,hashlib
ticket=Path(__file__).parent
artifact=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
boolean=artifact/"📐️geometry/🧊️boolean"
owned=[boolean/"🟦️.ts",boolean/"execution.schema.json"]+[boolean/"🧪️tests"/name for name in ["control-authority.json","control-authority.schema.json","control-authority.test.ts","cooperative.test.ts","control-receiver.json","control-receiver.schema.json","control-receiver.test.ts","control-policy.json","control-policy.schema.json","control-policy.test.ts"]]
refs=[path for path in boolean.rglob("*")if path.is_file() and path not in owned]
refs += [artifact/"📐️geometry/🟦️.ts",artifact/"📐️geometry/📜️limits.json",artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry.ts",artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry/csg.ts"]
def row(path):return {"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
result={"owned":list(map(row,owned)),"references":list(map(row,refs)),"nativeExecuted":False,"scope":"Shared TypeScript cooperative Boolean/halfspace kernel, original sync adapters and new step/async consumers. IFC4x3 arbitrary Boolean projection is not yet implemented; original native Boolean core is reference-only."}
assert len(set(owned+refs))==len(owned+refs)
assert all(item["utf16PathUnits"]<=256 for item in result["owned"]+result["references"])
(ticket/"🗑️generated/boolean-cooperation-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG]",json.dumps({"owned":len(owned),"references":len(refs),"maxPath":max(row["utf16PathUnits"]for row in result["owned"]+result["references"])}))

