/** 🪢️ Emit actual authored IFC and GLB outputs for independent physical profile validation. */
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.argv[2]!),out=resolve(import.meta.dir,'🗑️generated/polyline-extrusion');await mkdir(out,{recursive:true});
const {ifcPolylineFixture}=await import(pathToFileURL(resolve(io,'🧪️tests/polyline-extrusion-fixtures.ts')).href);
const {writeIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const {ifc4x3ToGltf,writeIfcGltfGlb}=await import(pathToFileURL(resolve(io,'gltf.ts')).href);
const {extractIfc4x3Triangles}=await import(pathToFileURL(resolve(io,'geometry.ts')).href);
const cases=JSON.parse(await readFile(resolve(io,'🧪️tests/polyline-extrusion-cases.json'),'utf8')),rows=[];
for(const row of cases.cases){
 const graph=ifcPolylineFixture(row),source=await writeIfc4x3Step(graph.snapshot,{batchSize:4096});await writeFile(resolve(out,row.id+'.ifc'),source);
 if(row.expected!=='supported')continue;
 const mesh=(await extractIfc4x3Triangles(graph.snapshot,{batchSize:4096})).artifact![0]!,gltf=await ifc4x3ToGltf(graph.snapshot,{batchSize:4096}),bytes=await writeIfcGltfGlb(gltf.artifact!,{batchSize:4096});await writeFile(resolve(out,row.id+'.glb'),bytes);
 rows.push({id:row.id,product:Number(graph.product.value),expectedBounds:row.bounds,expectedVolume:row.volume,positions:mesh.positions,indices:mesh.indices});
}
await writeFile(resolve(out,'owned.json'),JSON.stringify({rows,nativeExecution:false},null,2));console.log('[DEBUG] '+JSON.stringify({sources:cases.cases.length,emitted:rows.length}));
