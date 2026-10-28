/** 🔬 Compare invalid dense layouts through the original Khronos Box GLB and official validator. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const root=resolve(import.meta.dir,"../../../../../../.."),io=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"),output=resolve(import.meta.dir,"🗑️generated/gltfval"),bytes=await readFile(resolve(io,"🧪️tests/box.glb")),frame=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength),jsonLength=frame.getUint32(12,true),original=JSON.parse(new TextDecoder().decode(bytes.subarray(20,20+jsonLength))),binOffset=28+jsonLength,binLength=frame.getUint32(20+jsonLength,true),binary=Array.from(bytes.subarray(binOffset,binOffset+binLength)),cases=JSON.parse(await readFile(resolve(io,"🧪️tests/gltf-accessor-cases.json"),"utf8"));
const validator=await import(pathToFileURL(resolve(output,"package/index.js")).href),results=[];
for(const row of cases.invalid) {
 if(row.edit==="nonbyteBufferValue")continue;
 const document=structuredClone(original),buffer=[...binary],position=document.accessors[document.meshes[0].primitives[0].attributes.POSITION],view=document.bufferViews[position.bufferView];
 if(row.edit==="misalignedViews"){buffer.unshift(0);document.buffers[0].byteLength++;for(const item of document.bufferViews)item.byteOffset=(item.byteOffset??0)+1;}
 else if(row.edit==="shortDeclaredBuffer")document.buffers[0].byteLength=1;
 else view.target=34963;
 const json=new TextEncoder().encode(JSON.stringify(document)),jsonSize=Math.ceil(json.length/4)*4,binSize=Math.ceil(buffer.length/4)*4,target=new Uint8Array(28+jsonSize+binSize),data=new DataView(target.buffer);data.setUint32(0,0x46546c67,true);data.setUint32(4,2,true);data.setUint32(8,target.length,true);data.setUint32(12,jsonSize,true);data.setUint32(16,0x4e4f534a,true);target.fill(32,20,20+jsonSize);target.set(json,20);data.setUint32(20+jsonSize,binSize,true);data.setUint32(24+jsonSize,0x004e4942,true);target.set(buffer,28+jsonSize);
 const result=await validator.validateBytes(target,{format:"glb",writeTimestamp:false,maxIssues:0});if(!result.issues.numErrors)throw Error("Independent invalid layout unexpectedly accepted: "+row.name);results.push({name:row.name,errors:result.issues.numErrors,messages:result.issues.messages});await writeFile(resolve(output,"accessor-"+row.edit+".glb"),target);
}
await writeFile(resolve(output,"accessor-oracle.json"),JSON.stringify({implementation:"Khronos glTF Validator",version:validator.version(),results},null,2));console.log("[DEBUG] "+JSON.stringify(results.map(row=>({name:row.name,errors:row.errors,codes:row.messages.map((message:{code:string})=>message.code)}))));
