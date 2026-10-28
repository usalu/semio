/** 🔮 Verify genuine Rust-owned exchange outputs with pinned Khronos, three.js and IfcOpenShell interfaces. */
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {resolve,relative,sep} from "node:path";
import {pathToFileURL} from "node:url";
import {Texture,Vector3,REVISION,type Mesh} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";

const [folderArg,python,moduleDirectory]=process.argv.slice(2);
if(!folderArg||!python||!moduleDirectory)throw new Error("usage: native-gltf-owned-oracle.ts NATIVE_OUTPUT_FOLDER PYTHON IFCOPENSHELL_MODULE_DIRECTORY");
const folder=resolve(folderArg),generated=resolve(import.meta.dir,"🗑️generated"),child=relative(generated,folder);
if(child===".."||child.startsWith(".."+sep)||resolve(generated,child)!==folder)throw new Error("Native evidence must stay under this ticket's generated folder");
const marker=JSON.parse(await readFile(resolve(folder,"native-owned-exchange.json"),"utf8"));
if(marker.nativeExecuted!==true||marker.producer!=="original-native-gltf-owned-tests"||marker.cases!==4)throw new Error("Genuine output from the actual original-owned native tests is required");
const root=resolve(import.meta.dir,"../../../../../../.."),io=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"),cases=JSON.parse(await readFile(resolve(io,"🧪️tests/gltf-samples/cases.json"),"utf8"));
const validator=await import(pathToFileURL(resolve(generated,"gltfval/package/index.js")).href);
if(typeof ProgressEvent==="undefined")Object.defineProperty(globalThis,"ProgressEvent",{value:class extends Event{lengthComputable=false;loaded=0;total=0;}});
const report=[];await mkdir(resolve(folder,"oracle"),{recursive:true});
for(const row of cases.cases){
 const stem=row.file.split(".")[0],bytes=await readFile(resolve(folder,`native-owned-${stem}.glb`));
 const validation=await validator.validateBytes(bytes,{format:"glb",writeTimestamp:false,maxIssues:0});
 const run=Bun.spawn([python,resolve(import.meta.dir,"ifc-oracle.py"),moduleDirectory,resolve(folder,`native-owned-${stem}.ifc`),resolve(folder,"oracle")],{stdout:"pipe",stderr:"pipe"});
 const [code,stdout,stderr]=await Promise.all([run.exited,new Response(run.stdout).text(),new Response(run.stderr).text()]);
 if(code!==0)throw new Error(`IfcOpenShell refused ${stem}: ${stderr}\n${stdout}`);
 const actual=JSON.parse(await readFile(resolve(folder,`oracle/native-owned-${stem}.json`),"utf8"));
 const original=await readFile(resolve(io,"🧪️tests/gltf-samples",row.file));
 const loader=new GLTFLoader();loader.register(()=>({name:"semio_geometry_only_texture",loadTexture:()=>Promise.resolve(new Texture())}));
 const originalScene=await loader.parseAsync(row.file.endsWith(".glb")?original.buffer.slice(original.byteOffset,original.byteOffset+original.byteLength) as ArrayBuffer:original.toString("utf8"),"");originalScene.scene.updateMatrixWorld(true);
 const expected:Array<{coordinates:number[][];indices:number[][]}>=[];
 originalScene.scene.traverse(child=>{const mesh=child as Mesh;if(!mesh.isMesh)return;const position=mesh.geometry.getAttribute("position"),coordinates:number[][]=[];for(let i=0;i<position.count;i++){const v=new Vector3().fromBufferAttribute(position,i).applyMatrix4(mesh.matrixWorld);coordinates.push([v.x,-v.z,v.y]);}const input=mesh.geometry.index?.array??Array.from({length:position.count},(_,i)=>i),indices:number[][]=[];for(let i=0;i<input.length;i+=3){const face=[input[i]!+1,input[i+1]!+1,input[i+2]!+1];if(mesh.matrixWorld.determinant()<0)[face[1],face[2]]=[face[2]!,face[1]!];indices.push(face);}expected.push({coordinates,indices});});
 const close=(a:number,b:number)=>Math.abs(a-b)<=8*Number.EPSILON*Math.max(1,Math.abs(a),Math.abs(b));
 const coordinates=actual.triangulations.length===expected.length&&actual.triangulations.every((item:{coordinates:number[][]},i:number)=>item.coordinates.length===expected[i]!.coordinates.length&&item.coordinates.every((point:number[],j:number)=>point.every((value:number,k:number)=>close(value,expected[i]!.coordinates[j]![k]!))));
 const indices=actual.triangulations.length===expected.length&&actual.triangulations.every((item:{indices:number[][]},i:number)=>JSON.stringify(item.indices)===JSON.stringify(expected[i]!.indices));
 const low=[0,1,2].map(k=>Math.min(...actual.geometry.map((g:{bounds:number[][]})=>g.bounds[0]![k]!))),high=[0,1,2].map(k=>Math.max(...actual.geometry.map((g:{bounds:number[][]})=>g.bounds[1]![k]!)));
 const within=[...low,...high].every((value,i)=>close(value,row.ifcBounds.flat()[i]));
 report.push({name:row.name,khronosErrors:validation.issues.numErrors,ifcVersion:actual.version,ifcValidation:actual.validation,ifcTriangleCount:actual.triangulations.reduce((n:number,item:{indices:unknown[]})=>n+item.indices.length,0),expectedTriangles:row.triangles,ifcBoundsWithinEightEpsilons:within,independentSourceCoordinates:coordinates,independentSourceIndices:indices});
}
const output={nativeExecuted:true,nativeOutputProducer:marker.producer,validator:{implementation:"Khronos glTF Validator",version:validator.version()},three:{implementation:"three.js",version:REVISION},ifc:{implementation:"IfcOpenShell"},geometryOnly:true,cases:report};
await writeFile(resolve(folder,"oracle/native-owned-oracle.json"),JSON.stringify(output,null,2));
console.log("[DEBUG] "+JSON.stringify(report));
if(report.some(row=>row.khronosErrors!==0||row.ifcValidation.length!==0||row.ifcTriangleCount!==row.expectedTriangles||!row.ifcBoundsWithinEightEpsilons||!row.independentSourceCoordinates||!row.independentSourceIndices))throw new Error("Independent native geometry exchange comparison failed");
