/** 🔁 Reimport independent physical STEP outputs and compare actual supported FEM semantics. */
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";
const io=resolve(process.cwd(),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"),output=resolve(import.meta.dir,"🗑️generated/fem-isolated/after");
const {ifc4x3ToFem3d}=await import(pathToFileURL(resolve(io,"fem.ts")).href),{parseIfc4x3Step}=await import(pathToFileURL(resolve(io,"step.ts")).href),{femConnectivityFixture}=await import(pathToFileURL(resolve(io,"🧪️tests/fem-isolated-fixtures.ts")).href);
const cases=await Bun.file(resolve(io,"🧪️tests/fem-isolated-cases.json")).json();
function normalize(model:any){
 const bits=(value:{bits:bigint})=>value.bits.toString(),node=(row:any)=>[bits(row.x),bits(row.y),bits(row.z)].join("/"),nodes=new Map(model.nodes.map((row:any)=>[row.id,node(row)]));
 const angle=(value:{bits:bigint})=>(value.bits&0x7fff_ffff_ffff_ffffn)===0n?"0":bits(value);
 const material=(row:any)=>[row.name,bits(row.e),bits(row.g),bits(row.nu),bits(row.rho)],materials=new Map(model.materials.map((row:any)=>[row.id,material(row)]));
 const section=(row:any)=>[row.name,bits(row.area),bits(row.iy),bits(row.iz),bits(row.j)],sections=new Map(model.sections.map((row:any)=>[row.id,section(row)]));
 const sort=(rows:any[])=>rows.map(row=>JSON.stringify(row)).sort();
 return{
  nodes:sort([...nodes.values()]),
  members:sort(model.elements.map((row:any)=>[row.kind,nodes.get(row.start),nodes.get(row.end),materials.get(row.materialId),sections.get(row.sectionId),row.kind==="frame"?angle(row.roll):null])),
  supports:sort(model.supports.map((row:any)=>[nodes.get(row.nodeId),[...row.fixed].sort()])),
  loads:sort(model.loadCases.map((row:any)=>[row.name,row.selfWeight,sort(row.loads.map((load:any)=>[load.kind,nodes.get(load.nodeId),load.dof,bits(load.value)]))]))
 };
}
let matched=0;
for(const row of cases.cases.filter((row:{code:string|null})=>row.code===null)){
 const graph=await parseIfc4x3Step(await Bun.file(resolve(output,row.id+".independent.ifc")).text()),actual=await ifc4x3ToFem3d(graph);
 assert.ok(actual.artifact,row.id+" independent reimport refused");assert.deepEqual(normalize(actual.artifact),normalize(femConnectivityFixture(row)),row.id);matched++;
}
console.log("[DEBUG] "+JSON.stringify({independentPhysicalOutputs:matched,supportedSemanticMatches:matched,nativeExecution:false}));
