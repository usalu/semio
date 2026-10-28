/** 🧪️ Export admitted real endpoint sources for independent STEP validation and projection. */
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io');
const {ifcExchangeSourceFixture}=await import(pathToFileURL(resolve(io,'🧪️tests/exchange-source-fixtures.ts')).href),{writeIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const output=resolve(import.meta.dir,'🗑️generated/exchange-source');await mkdir(output,{recursive:true});
const phases=[['ifc-to-bim','bim-import'],['ifc-to-fem','fem-import'],['ifc-triangles','geometry']],rows=[];
for(const [id,phase] of phases){
 const {snapshot,project}=await ifcExchangeSourceFixture(id),baseline=await project(snapshot);if(!baseline.artifact)throw Error(id+' baseline refused');
 await writeFile(resolve(output,id+'.ifc'),await writeIfc4x3Step(snapshot));
 let changed=false;const captured=await project(snapshot,{progress:progress=>{if(progress.phase===phase&&!changed){changed=true;snapshot.entities=[];snapshot.header.fileSchema=[];}}});
 if(!changed||!isDeepStrictEqual(baseline,captured))throw Error(id+' capture mismatch');
 rows.push({id,phase,baseline,captureEqual:true});
}
await writeFile(resolve(output,'owned.json'),JSON.stringify({rows},(_,v)=>typeof v==='bigint'?v.toString():v,2)+'\n');
console.log('[DEBUG] '+JSON.stringify({endpoints:rows.length,captureMismatches:0}));

