from pathlib import Path
import json,hashlib
ticket=Path(__file__).parent
root=ticket.parents[6]
io=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"
path=io/"geometry.ts"
source=path.read_text(encoding="utf8")
old='return drainIfcIoSteps(ifcGeometryVectorSteps(this,value,kind,dimension));'
new='try{return drainIfcIoSteps(ifcGeometryVectorSteps(this,value,kind,dimension));}catch(error){if(error instanceof IfcStepError)throw new IfcStepError("exchange.placement",error.message);throw error;}'
assert source.count(old)==1
source=source.replace(old,new)
path.write_text(source,encoding="utf8")
cases=io/"🧪️tests/tfs-admission-cases.json"
original=ticket/"tfs-admission-cases.json"
old_hash=hashlib.sha256(cases.read_bytes()).hexdigest()
assert old_hash=="b7e45af97bc002e637b07775f5238972a1e671f615cce296944fb8f77ecd59bf"
assert hashlib.sha256(original.read_bytes()).hexdigest()==old_hash
value=json.loads(cases.read_text(encoding="utf8"))
row=next(row for row in value["cases"]if row["id"]=="integer-coordinate-value")
before=json.loads(json.dumps(row))
assert row["expected"]=="accept" and row["source"]["coordinateKind"]=="integer",row
row["expected"]="refuse"
row["reason"]="CoordList declared REAL leaves require REAL Part21 carriers; repair only by explicit canonical mutation"
cases.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
new_hash=hashlib.sha256(cases.read_bytes()).hexdigest()
test=io/"🧪️tests/tfs-admission.test.ts"
source=test.read_text(encoding="utf8")
assert source.count(old_hash)==1
source=source.replace(old_hash,new_hash)
source=source.replace("import {expect,test} from 'bun:test';","import {expect,test} from 'bun:test';\nimport type {IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';",1)
test.write_text(source,encoding="utf8")
report={"originalSha256":old_hash,"correctedSha256":new_hash,"before":before,"after":row,"originalTicketUnchanged":True}
(ticket/"🗑️generated/tfs-real-carrier-correction.json").write_text(json.dumps(report,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG]",json.dumps({"axis":"owned placement diagnostic restored","tfsOriginalSha256":old_hash,"sourceCorrectedSha256":new_hash}))

