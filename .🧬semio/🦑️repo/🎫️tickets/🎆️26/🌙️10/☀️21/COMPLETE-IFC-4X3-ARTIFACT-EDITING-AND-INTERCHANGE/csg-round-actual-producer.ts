/** 🏭️ Capture actual public producer STEP, meshes and GLBs for independent physical and glTF observers. */
import{mkdir,writeFile}from'node:fs/promises';import{resolve}from'node:path';import{createHash}from'node:crypto';
import cases from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-round-cases.json';
import{ifcRoundFixture}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-round-fixtures.ts';
import{extractIfc4x3Triangles}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry.ts';
import{ifc4x3ToGltf,writeIfcGltfGlb}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/gltf.ts';
import{writeIfc4x3Step}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import{ifcReal}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import{ifcMutationDiff}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import{applyIfcDiff}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts';
import type{IfcSnapshot}from'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const folder=resolve(import.meta.dir,'🗑️generated/csg-round-admitted-actual');await mkdir(folder);const rows:unknown[]=[],encode=(value:unknown)=>JSON.stringify(value,(_name,item)=>typeof item==='bigint'?String(item):item);
async function emit(row:(typeof cases.cases)[number],snapshot:IfcSnapshot,repair:unknown=null){
 const before=structuredClone(snapshot),control={batchSize:4096,curveSampling:row.source.policy},mesh=await extractIfc4x3Triangles(snapshot,control),out=await ifc4x3ToGltf(snapshot,control);
 if(mesh.diagnostics.length||!mesh.artifact||out.diagnostics.length!==1||out.diagnostics[0]!.code!=='exchange.loss'||!out.artifact)throw Error('Actual CSG producer '+row.id);
 const step=await writeIfc4x3Step(snapshot,{batchSize:4096}),glb=await writeIfcGltfGlb(out.artifact,{batchSize:4096});
 if(encode(before)!==encode(snapshot))throw Error('Actual CSG producer mutated source '+row.id);
 await writeFile(resolve(folder,row.id+'.ifc'),step);await writeFile(resolve(folder,row.id+'.glb'),glb);
 rows.push({id:row.id,source:row.source,expected:row.expected,repair,step:row.id+'.ifc',stepSha256:createHash('sha256').update(step).digest('hex'),glb:row.id+'.glb',glbSha256:createHash('sha256').update(glb).digest('hex'),mesh:mesh.artifact[0],diagnostics:out.diagnostics});
}
for(const row of cases.cases.filter(row=>row.expected.status==='supported'))await emit(row,ifcRoundFixture(row).snapshot);
const original=cases.cases.find(row=>row.id==='cylinder-integer-carriers')!,fixture=ifcRoundFixture(original),before=structuredClone(fixture.snapshot),raw=await writeIfc4x3Step(fixture.snapshot,{batchSize:4096}),mutation={mutation:'applyDiff' as const,diff:{entities:{modified:[{id:fixture.primitive.id,diff:{args:{modified:[{index:1,value:ifcReal(3)},{index:2,value:ifcReal(1)}]}}}]}}},diff=ifcMutationDiff(fixture.snapshot,mutation),repaired=applyIfcDiff(fixture.snapshot,diff);
await emit({...original,id:'cylinder-integer-repaired',source:{...original.source,heightKind:'real',radiusKind:'real'},expected:cases.cases[0]!.expected},repaired,{sourceId:original.id,originalStepSha256:createHash('sha256').update(raw).digest('hex'),mutation,diff});if(encode(before)!==encode(fixture.snapshot))throw Error('Canonical repair mutated the raw source');
await writeFile(resolve(folder,'producer.json'),JSON.stringify({runtime:{bun:Bun.version,nodeCompatibility:process.version,executable:process.execPath},nativeExecuted:false,rows},(_name,item)=>typeof item==='bigint'?String(item):item,2));
console.log('[DEBUG]',JSON.stringify({admittedSourceRows:rows.length-1,canonicalRepairRows:1,actualPublicRows:rows.length,sourceUnchanged:true,nativeExecuted:false}));
