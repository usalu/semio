/** 🏛️ Export actual authored frames and explicit previous-shape counterexamples for independent checks. */
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'),output=resolve(import.meta.dir,'🗑️generated/bim-axis');await mkdir(output,{recursive:true});
const {ifcBimAxisFixture}=await import(pathToFileURL(resolve(io,'🧪️tests/bim-axis-fixtures.ts')).href),{bimSpatialToIfc4x3}=await import(pathToFileURL(resolve(io,'bim.ts')).href),{writeIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const cases=await Bun.file(resolve(io,'🧪️tests/bim-axis-cases.json')).json();
for(const row of cases.cases){
 const result=await bimSpatialToIfc4x3(ifcBimAxisFixture(row.angle));if(!result.artifact)throw Error('export refused');
 await writeFile(resolve(output,row.id+'.ifc'),await writeIfc4x3Step(result.artifact));
 if(row.angle!==0){const prior=structuredClone(result.artifact),axis=prior.entities.find(entity=>entity.name==='IFCAXIS2PLACEMENT3D'&&entity.args[1]?.kind==='reference'&&entity.args[2]?.kind==='reference');if(!axis)throw Error('rotated frame absent');axis.args[1]={kind:'unset'};await writeFile(resolve(output,row.id+'.one-sided.ifc'),await writeIfc4x3Step(prior));}
}
await writeFile(resolve(output,'owned.json'),JSON.stringify(cases,null,2)+'\n');console.log('[DEBUG] '+JSON.stringify({frames:cases.cases.length,oneSidedCounterexamples:cases.cases.filter(row=>row.angle!==0).length}));

