import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {writeFile} from "node:fs/promises";
const base=join(process.cwd(),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"),output=join(import.meta.dir,"🗑️generated/ifc-inputs");
const {parseIfc4x3Step,writeIfc4x3Step,ifcReal}=await import(pathToFileURL(join(base,"step.ts")).href),{ifc4x3ToFem3d,fem3dToIfc4x3}=await import(pathToFileURL(join(base,"fem.ts")).href);
const read=async(name:string)=>parseIfc4x3Step(await Bun.file(join(output,name+"-export.ifc")).text()),ref=(value:bigint)=>({kind:"reference",value}),next=(snapshot:any)=>snapshot.entities.reduce((max:bigint,row:any)=>row.id>max?row.id:max,0n)+1n;
const bim=await read("bim"),unit=next(bim);bim.entities.push({id:unit,name:"IFCSIUNIT",args:[{kind:"derived"},{kind:"enum",value:"LENGTHUNIT"},{kind:"enum",value:"MILLI"},{kind:"enum",value:"METRE"}],complex:[]});
for(const property of bim.entities.filter((row:any)=>row.name==="IFCPROPERTYSINGLEVALUE"&&["Height","CutHeight"].includes(row.args[0].value))) {property.args[3]=ref(unit);property.args[2].value.items[0]=ifcReal(property.args[0].value==="Height"?3000:1200);}
await writeFile(join(output,"bim-mm.ifc"),await writeIfc4x3Step(bim));
const fem=await read("fem");let id=next(fem);const member=fem.entities.find((row:any)=>row.name==="IFCSTRUCTURALCURVEMEMBER")!,duplicate=structuredClone(member);duplicate.id=id++;duplicate.args[0]={kind:"string",value:"0000000000000000000099"};fem.entities.push(duplicate);
for(const row of [...fem.entities]) {
 if(row.name==="IFCRELCONNECTSSTRUCTURALMEMBER"&&row.args[4].value===member.id) {const clone=structuredClone(row);clone.id=id++;clone.args[0]={kind:"string",value:String(clone.id).padStart(22,"0")};clone.args[4]=ref(duplicate.id);fem.entities.push(clone);}
 if(["IFCRELASSOCIATESMATERIAL","IFCRELDEFINESBYPROPERTIES","IFCRELASSIGNSTOGROUP"].includes(row.name)&&row.args[4].kind==="aggregate"&&row.args[4].value.some((value:any)=>value.value===member.id))row.args[4].value.push(ref(duplicate.id));
}
const nodes=fem.entities.filter((row:any)=>row.name==="IFCSTRUCTURALPOINTCONNECTION");nodes[1].args[7]=structuredClone(nodes[0].args[7]);
await writeFile(join(output,"fem-shared.ifc"),await writeIfc4x3Step(fem));
const imported=await ifc4x3ToFem3d(fem);if(!imported.artifact)throw Error(JSON.stringify(imported.diagnostics));
const exported=await fem3dToIfc4x3(imported.artifact);if(!exported.artifact)throw Error(JSON.stringify(exported.diagnostics));
await writeFile(join(output,"fem-shared-roundtrip.ifc"),await writeIfc4x3Step(exported.artifact));
console.log("[DEBUG] Explicit BIM units and shared FEM definition oracle inputs prepared");
