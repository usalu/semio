import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {mkdir,writeFile} from "node:fs/promises";
const base=join(process.cwd(),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io");
const {ifcReal,writeIfc4x3Step}=await import(pathToFileURL(join(base,"step.ts")).href);
const {fem3dToIfc4x3}=await import(pathToFileURL(join(base,"fem.ts")).href);
const {bimSpatialToIfc4x3}=await import(pathToFileURL(join(base,"bim.ts")).href);
const {readIfcGltfGlb,gltfToIfc4x3}=await import(pathToFileURL(join(base,"gltf.ts")).href);
const w=(n:number)=>ifcReal(n).value;
const fem={nodes:[{id:"n1",x:w(0),y:w(0),z:w(0)},{id:"n2",x:w(2),y:w(0),z:w(0)}],elements:[{kind:"frame",id:"f1",start:"n1",end:"n2",materialId:"steel",sectionId:"s1",roll:w(0)}],materials:[{id:"steel",name:"Steel",e:w(2e11),g:w(8e10),nu:w(.25),rho:w(7850)}],sections:[{id:"s1",name:"Section",area:w(.01),iy:w(1e-5),iz:w(2e-5),j:w(3e-5)}],solids:[],supports:[{id:"fixed",nodeId:"n1",fixed:["Tx","Ty","Tz","Rx","Ry","Rz"]}],loadCases:[{id:"case",name:"Load",selfWeight:false,loads:[{kind:"nodal",id:"force",nodeId:"n2",dof:"Tz",value:w(-1000)},{kind:"memberUdl",id:"udl",elementId:"f1",wx:w(0),wy:w(0),wz:w(-500)}]}],combinations:[],analysis:{modalCount:0n,bucklingCount:0n,deformationScale:w(1)}};
const bim={schema:"s.bim.model@1",project:{name:"Project",description:"Spatial fixture",author:"Author",organization:"Organization",phase_names:["Phase"]},sites:{site:{name:"Site",latitude:47.5,longitude:8.5,elevation:400,true_north:.2,boundary:[]}},buildings:{building:{site:"site",name:"Building",origin:{x:1,y:2},rotation:.3,elevation:4}},storeys:{ground:{building:"building",name:"Ground",level:0,height:3,cut_height:1.2},upper:{building:"building",name:"Upper",level:1,height:3}}};
const output=join(import.meta.dir,"🗑️generated/ifc-inputs");await mkdir(output,{recursive:true});
const gltf=await readIfcGltfGlb(new Uint8Array(await Bun.file(join(base,"🧪️tests/box.glb")).arrayBuffer()));
for(const [name,outcome] of [["fem-export",await fem3dToIfc4x3(fem)],["bim-export",await bimSpatialToIfc4x3(bim)],["box-import",await gltfToIfc4x3(gltf)]] as const) {
 if(!outcome.artifact)throw Error(JSON.stringify(outcome.diagnostics));
 await writeFile(join(output,name+".ifc"),await writeIfc4x3Step(outcome.artifact));
}
const binary=await import(pathToFileURL(join(base,"step.ts")).href);
const doc=await binary.parseIfc4x3Step('ISO-10303-21;HEADER;FILE_DESCRIPTION((\'\'),\'2;1\');FILE_NAME(\'\',\'\',(\'\'),(\'\'),\'\',\'\',\'\');FILE_SCHEMA((\'IFC4X3_ADD2\'));ENDSEC;DATA;#1=IFCPIXELTEXTURE(.T.,.T.,$,$,$,1,1,3,("0001122"));ENDSEC;END-ISO-10303-21;');
await writeFile(join(output,"binary.ifc"),await writeIfc4x3Step(doc));
console.log("[DEBUG] IFC semantic exchange oracle inputs prepared");
