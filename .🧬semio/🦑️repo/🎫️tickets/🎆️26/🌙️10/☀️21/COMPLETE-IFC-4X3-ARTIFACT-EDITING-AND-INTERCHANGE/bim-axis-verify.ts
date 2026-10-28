/** 🔁️ Verify supported BIM frames after independent physical STEP serialization. */
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'),output=resolve(import.meta.dir,'🗑️generated/bim-axis');
const {ifc4x3ToBimSpatial}=await import(pathToFileURL(resolve(io,'bim.ts')).href),{parseIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const cases=JSON.parse(await readFile(resolve(output,'owned.json'),'utf8')),rows=[];
for(const row of cases.cases){
 const snapshot=await parseIfc4x3Step(await readFile(resolve(output,row.id+'.independent.ifc'),'utf8')),result=await ifc4x3ToBimSpatial(snapshot),building=result.artifact&&Object.values(result.artifact.buildings)[0];
 if(!building||Math.abs(building.rotation-row.angle)>1e-14||building.origin.x!==1||building.origin.y!==2||building.elevation!==4)throw Error(row.id+' reimport mismatch');
 rows.push({id:row.id,frameEqual:true});
}
await writeFile(resolve(output,'verification.json'),JSON.stringify({rows,nativeExecution:false},null,2)+'\n');console.log('[DEBUG] '+JSON.stringify({frames:rows.length,mismatches:0}));

