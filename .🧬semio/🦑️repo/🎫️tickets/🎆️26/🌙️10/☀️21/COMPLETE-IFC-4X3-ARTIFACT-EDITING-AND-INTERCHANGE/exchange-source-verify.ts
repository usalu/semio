/** 🔁️ Compare supported semantic projections of independently serialized IFC sources. */
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'),output=resolve(import.meta.dir,'🗑️generated/exchange-source');
const {ifcExchangeSourceFixture}=await import(pathToFileURL(resolve(io,'🧪️tests/exchange-source-fixtures.ts')).href),{parseIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const owned=JSON.parse(await readFile(resolve(output,'owned.json'),'utf8')),rows=[];
for(const row of owned.rows){
 const {project}=await ifcExchangeSourceFixture(row.id),source=await parseIfc4x3Step(await readFile(resolve(output,row.id+'.independent.ifc'),'utf8')),actual=await project(source);
 const normalized=JSON.parse(JSON.stringify(actual,(_,value)=>typeof value==='bigint'?value.toString():value));
 if(!isDeepStrictEqual(row.baseline,normalized))throw Error(row.id+' independent projection mismatch');
 rows.push({id:row.id,projectionEqual:true});
}
await writeFile(resolve(output,'verification.json'),JSON.stringify({rows,nativeExecution:false},null,2)+'\n');console.log('[DEBUG] '+JSON.stringify({endpoints:rows.length,mismatches:0}));

