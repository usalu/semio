/** 🧭️ Export actual axis graph results alongside canonical independent test inputs. */
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io');
const {ifcAxisFixture}=await import(pathToFileURL(resolve(io,'🧪️tests/axis-fixtures.ts')).href),{IfcGeometryGraph}=await import(pathToFileURL(resolve(io,'geometry.ts')).href),{writeIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const cases=await Bun.file(resolve(io,'🧪️tests/axis-cases.json')).json(),output=resolve(import.meta.dir,'🗑️generated/axis');await mkdir(output,{recursive:true});const rows=[];
for(const row of cases.cases){
 const {snapshot,placement}=ifcAxisFixture(row),graph=(await IfcGeometryGraph.capture(snapshot)).artifact;if(!graph)throw Error('source refused');
 let matrix,error;try{matrix=graph.axis(placement);}catch(refusal){error=refusal.code;}
 rows.push({...row,placementId:Number(placement.value),actual:matrix,actualError:error});
 await writeFile(resolve(output,row.id+'.ifc'),await writeIfc4x3Step(snapshot));
}
await writeFile(resolve(output,'owned.json'),JSON.stringify({rows},null,2)+'\n');console.log('[DEBUG] '+JSON.stringify({axisCases:rows.length}));

