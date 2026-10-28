/** ⭕️ Emit actual authored IFC, controlled circle meshes and GLB for independent compliance checks. */
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.argv[2]!),out=resolve(import.meta.dir,'🗑️generated/circle-extrusion');await mkdir(out,{recursive:true});
const {ifcCircleFixture}=await import(pathToFileURL(resolve(io,'🧪️tests/circle-extrusion-fixtures.ts')).href);
const {writeIfc4x3Step,parseIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const {ifc4x3ToGltf,writeIfcGltfGlb}=await import(pathToFileURL(resolve(io,'gltf.ts')).href);
const {extractIfc4x3Triangles}=await import(pathToFileURL(resolve(io,'geometry.ts')).href);
const cases=JSON.parse(await readFile(resolve(io,'🧪️tests/circle-extrusion-cases.json'),'utf8')),rows=[];
for(const row of cases.cases){
 const graph=ifcCircleFixture(row),source=await writeIfc4x3Step(graph.snapshot,{batchSize:4096});await writeFile(resolve(out,row.id+'.ifc'),source);
 if(row.expected!=='supported')continue;
 const control={batchSize:4096,curveSampling:row.sampling},mesh=(await extractIfc4x3Triangles(graph.snapshot,control)).artifact![0]!,gltf=await ifc4x3ToGltf(graph.snapshot,control),bytes=await writeIfcGltfGlb(gltf.artifact!,{batchSize:4096});await writeFile(resolve(out,row.id+'.glb'),bytes);
 rows.push({id:row.id,product:Number(graph.product.value),case:row,positions:mesh.positions,indices:mesh.indices});
}
const original=await parseIfc4x3Step(await readFile(resolve(io,'../🧪️fixtures/official/slab-openings.ifc'),'utf8'),{batchSize:4096}),official=(await extractIfc4x3Triangles(original,{batchSize:4096})).artifact!.find((mesh:{productId:bigint})=>mesh.productId===325n)!;
rows.push({id:'official-opening-325',product:325,positions:official.positions,indices:official.indices});
await writeFile(resolve(out,'owned.json'),JSON.stringify({rows,nativeExecution:false,scope:'Ideal profile-plane chord certificate; binary64 transform and GLB FLOAT rounding remain separate'},null,2));console.log('[DEBUG] '+JSON.stringify({sources:cases.cases.length,emitted:rows.length-1,official:1}));
