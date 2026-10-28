from pathlib import Path
import json,hashlib
ticket=Path(__file__).parent
root=ticket.parents[6]
artifact=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
base=artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
original=json.loads((artifact/"📐️geometry/🧊️boolean/🧪️tests/operators.json").read_text(encoding="utf8"))
rows=[]
for index,item in enumerate(original["cases"]):
 for number,op in enumerate(["difference","union","intersection"]):
  volume=item[op]
  rows.append({"id":item["name"]+"-"+op,"operation":op.upper(),"root":"csg"if(index+number)%2 else"direct","cut":item["cut"],"unitPrefix":None,"mapping":[],"expected":"unsupported"if volume is None else"empty"if volume==0 else"supported","volume":volume})
schema={"$schema":"http://json-schema.org/draft-07/schema#","type":"object","additionalProperties":False,"required":["schema","cases"],"properties":{"schema":{"const":"IFC4X3_ADD2"},"cases":{"type":"array","minItems":30,"maxItems":30,"items":{"type":"object","additionalProperties":False,"required":["id","operation","root","cut","unitPrefix","mapping","expected","volume"],"properties":{"id":{"type":"string","minLength":1},"operation":{"enum":["DIFFERENCE","UNION","INTERSECTION"]},"root":{"enum":["direct","csg"]},"cut":{"type":"array","minItems":2,"maxItems":2,"items":{"type":"array","minItems":3,"maxItems":3,"items":{"type":"number"}}},"unitPrefix":{"type":"null"},"mapping":{"type":"array","maxItems":0},"expected":{"enum":["supported","empty","unsupported"]},"volume":{"type":["number","null"],"minimum":0}}}}}}
for name,value in [("boolean-finite-cases.json",{"schema":"IFC4X3_ADD2","cases":rows}),("boolean-finite.schema.json",schema)]:
 (base/name).write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] first finite IFC Boolean30 neutral contracts written; original shared matrix retained unchanged")

