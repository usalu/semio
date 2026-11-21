/** 🧱️ Check material-separated rigid face data against the independent part fixture. */
import {readFileSync} from "node:fs";
import {resolve,join} from "node:path";
import assert from "node:assert/strict";
import {WebIO} from "@gltf-transform/core";
import {encodeGlb} from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts";
const dir=resolve(import.meta.dir,"../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb");
const f=JSON.parse(readFileSync(join(dir,"🧪️tests/🧪️oracles/🧫️fixtures/🧱️shared-rigid-parts/🔣️.json"),"utf8"));
const {document,options}=JSON.parse(readFileSync(join(dir,"🧫️fixtures/🧊️rig-full/🔣️.json"),"utf8"));
const count=3*f.trianglesPerPart;
const source=f.partOrders.map((ids:string[],b:number)=>{
 const positions=f.partOffsetsM[b].flatMap((offset:number)=>Array.from({length:f.trianglesPerPart},(_,i)=>f.triangle.flatMap(([x,y,z]:number[])=>[x+2*i,y,z+offset])).flat());
 return {mode:"triangles",positions,indices:Array.from({length:2*count},(_,i)=>i),normals:Array.from({length:2*count},()=>[0,0,1]).flat(),material:f.materials[b],solids:ids.map((id,i)=>({id,first:i*count,count}))};
});
document.meshes[1].primitives=source;
const bytes=encodeGlb(document,options,{cancelled:()=>false,report:()=>{}});
const j=(await new WebIO().binaryToJSON(bytes)).json;
const prims=j.meshes[1].primitives;
const group=(id:string)=>prims.find((p:any)=>p.extras.solids[0].id===id);
console.log(JSON.stringify({bytes:bytes.length,groups:prims.length,positionAccessors:f.sharedParts.map((id:string)=>group(id)?.attributes.POSITION)}));
assert.equal(prims.length,4);assert.equal(group(f.sharedParts[0]).attributes.POSITION,group(f.sharedParts[1]).attributes.POSITION);
