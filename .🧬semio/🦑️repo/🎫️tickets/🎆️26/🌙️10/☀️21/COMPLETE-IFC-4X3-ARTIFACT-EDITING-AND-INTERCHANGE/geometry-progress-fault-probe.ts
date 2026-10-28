import {writeFile} from 'node:fs/promises';
import {pathToFileURL,fileURLToPath} from 'node:url';
import {dirname,join} from 'node:path';
import {existsSync} from 'node:fs';
let repo=dirname(fileURLToPath(import.meta.url));while(!existsSync(join(repo,'✏️s')))repo=dirname(repo);
const io=join(repo,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io')+'/';
const {createIfc4x3ExchangeGraph}=await import(pathToFileURL(io+'gltf.ts').href);
const {extractIfc4x3Triangles}=await import(pathToFileURL(io+'geometry.ts').href);
const {ifcReal}=await import(pathToFileURL(io+'step-values.ts').href);
const graph=createIfc4x3ExchangeGraph('progress-probe'),u={kind:'unset'},a=(value:any[])=>({kind:'aggregate',value}),s=(value:string)=>({kind:'string',value});
const points=graph.add('IFCCARTESIANPOINTLIST3D',[a([[0,0,0],[1,0,0],[0,1,0]].map(row=>a(row.map(ifcReal)))),u]);
const faces=graph.add('IFCTRIANGULATEDFACESET',[points,u,u,a([a([1n,2n,3n].map(value=>({kind:'integer',value})))]),u]);
const rep=graph.add('IFCSHAPEREPRESENTATION',[graph.context,s('Body'),s('Tessellation'),a([faces])]);const shape=graph.add('IFCPRODUCTDEFINITIONSHAPE',[u,u,a([rep])]);graph.add('IFCBUILDINGELEMENTPROXY',[s('0000000000000000000001'),u,s('probe'),u,u,u,shape,u,{kind:'enum',value:'NOTDEFINED'}]);
const rows=[];for(const phase of ['geometry','vertices','triangles']){const original=new Error('caller-'+phase);let result;try{const value=await extractIfc4x3Triangles(graph.snapshot,{batchSize:4096,progress:(event:any)=>{if(event.phase===phase)throw original;}});result={phase,rejected:false,fidelity:value.fidelity,diagnostics:value.diagnostics};}catch(error){result={phase,rejected:true,same:error===original,message:String(error)};}rows.push(result);}
await writeFile(new URL('./🗑️generated/geometry-progress-fault-after.json',import.meta.url),JSON.stringify(rows,(_key,value)=>typeof value==='bigint'?String(value):value,2));console.log('[DEBUG] '+JSON.stringify(rows,(_key,value)=>typeof value==='bigint'?String(value):value));



