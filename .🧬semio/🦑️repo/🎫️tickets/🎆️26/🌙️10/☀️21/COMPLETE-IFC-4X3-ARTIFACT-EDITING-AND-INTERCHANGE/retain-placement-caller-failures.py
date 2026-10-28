from pathlib import Path
import json
ticket=Path(__file__).parent
io=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"
path=io/"geometry.ts"
source=path.read_text(encoding="utf8")
old='if(error instanceof IfcStepError)throw new IfcStepError("exchange.placement",error.message);'
new='if(error instanceof IfcStepError&&(error.code==="geometry.declaration"||error.code==="exchange.vector"||error.code==="geometry.domain"))throw new IfcStepError("exchange.placement",error.message);'
assert source.count(old)==1
path.write_text(source.replace(old,new),encoding="utf8")
path=io/"🧪️tests/axis.test.ts"
source=path.read_text(encoding="utf8")
source=source.replace("import {IfcGeometryGraph} from '../geometry.ts';","import {IfcGeometryGraph,extractIfc4x3Triangles} from '../geometry.ts';\nimport {IfcStepError,type IfcIoControl} from '../control.ts';\nimport {ifcTfsFixture} from './tfs-admission-fixtures.ts';\nimport tfsCases from './tfs-admission-cases.json';\nimport {ifc4x3ToGltf,writeIfcGltfGlb} from '../gltf.ts';",1)
addition="\ntest('placement boundaries preserve same-class original observer identity',async()=>{\n const source=ifcTfsFixture(tfsCases.cases[0]!).snapshot,before=structuredClone(source);\n for(const endpoint of ['triangles','glb'])for(const code of ['geometry.declaration','exchange.vector','geometry.domain'])for(const frontier of ['initial','interior','completion']){\n  const events:number[]=[],run=async(control:IfcIoControl)=>{if(endpoint==='triangles')return extractIfc4x3Triangles(source,control);const out=await ifc4x3ToGltf(source,control);return writeIfcGltfGlb(out.artifact!,control);};await run({batchSize:1,progress:event=>{if(event.phase==='placements')events.push(event.completed);}});expect(events.length).toBeGreaterThan(2);const target=frontier==='initial'?0:frontier==='completion'?events.length-1:1,original=new IfcStepError(code,'exact original placement observer');let calls=0,redirected=0,caught=false;const control:IfcIoControl={batchSize:1,progress:event=>{if(event.phase==='placements'&&calls++===target){control.progress=()=>{redirected++;};throw original;}}};\n  try{await run(control);}catch(error){caught=true;expect(error).toBe(original);}expect(caught).toBe(true);expect(calls).toBe(target+1);expect(redirected).toBe(0);expect(source).toEqual(before);\n }\n});\n"
assert "placement boundaries preserve same-class original observer identity"not in source
path.write_text(source+addition,encoding="utf8")
config=ticket/"csg-round-tsconfig.json"
value=json.loads(config.read_text(encoding="utf8"))
for name in ["axis.test.ts","tfs-admission.test.ts"]:
 target="../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/"+name
 if target not in value["files"]:value["files"].append(target)
config.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] narrow owned placement validation categories plus exact original same-class observer laws")

