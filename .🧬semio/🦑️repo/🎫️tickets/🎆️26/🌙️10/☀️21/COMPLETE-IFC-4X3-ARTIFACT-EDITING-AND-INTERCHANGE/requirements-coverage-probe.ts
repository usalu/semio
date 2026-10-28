/** 🔢️ Exercise actual shared coverage arithmetic without asserting source or view admission. */
import {readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const root=resolve(import.meta.dir,'../../../../../../..');
const directory=resolve(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧾️requirements');
const cases=JSON.parse(await readFile(resolve(directory,'cases.json'),'utf8'));
const {ifcRequirementCoverageSteps}=await import(pathToFileURL(resolve(directory,'coverage.ts')).href);
const rows=[];
for(const item of cases.coverage){const input=item.rows.map((row:any)=>({selection:{applicable:row.applicable},implementation:row.implementation,status:row.status,hasFailure:row.hasFailure,...(row.applicable===true&&row.implementation==='implemented'?{child:{}}:{})}));const steps=ifcRequirementCoverageSteps(input,item.scope);let next=steps.next();while(!next.done)next=steps.next();rows.push({id:item.id,...next.value});}
process.stdout.write(JSON.stringify({nativeExecuted:false,rows},null,2)+'\n');
