/** 🔒️ Freeze original first-party ring outputs before introducing cooperative continuations. */
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const artifact=resolve(process.argv[2]!),directory=resolve(artifact,'📐️geometry/🕳️rings');
const {triangulateRings}=await import(pathToFileURL(resolve(directory,'🟦️.ts')).href);
const fixtures=JSON.parse(await readFile(resolve(directory,'🧪️tests/🔣️.json'),'utf8'));
const rows=[];
for(const row of fixtures.cases)for(const reversed of [false,true])for(const scale of [1,1e-120,1e120]){
 const points:number[][]=[],rings:number[][]=[];
 for(const source of row.rings){const indices:number[]=[];for(const point of reversed?[...source].reverse():source){indices.push(points.length);points.push(point.map((n:number)=>n*scale));}rings.push(indices);}
 let ticks=0;const triangles=triangulateRings(points,rings,()=>ticks++);
 rows.push({name:row.name,reversed,scale,ticks,vertices:points.length,triangles:triangles.length,sha256:createHash('sha256').update(JSON.stringify(triangles)).digest('hex')});
}
const output=resolve(import.meta.dir,'🗑️generated/ring-continuation');await mkdir(output,{recursive:true});
await writeFile(resolve(output,'baseline.json'),JSON.stringify({rows,nativeExecution:false},null,2));console.log('[DEBUG] '+JSON.stringify({rows:rows.length}));
