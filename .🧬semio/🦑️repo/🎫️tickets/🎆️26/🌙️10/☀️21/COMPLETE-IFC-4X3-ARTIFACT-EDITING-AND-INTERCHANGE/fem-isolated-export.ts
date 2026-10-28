/** 🔬 Export actual neutral FEM mapper outcomes for independent full EXPRESS comparison. */
import {mkdir,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const io=resolve(process.cwd(),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io");
const {fem3dToIfc4x3}=await import(pathToFileURL(resolve(io,"fem.ts")).href),{writeIfc4x3Step}=await import(pathToFileURL(resolve(io,"step.ts")).href),{femConnectivityFixture}=await import(pathToFileURL(resolve(io,"🧪️tests/fem-isolated-fixtures.ts")).href);
const cases=await Bun.file(resolve(io,"🧪️tests/fem-isolated-cases.json")).json(),stage=process.argv[2];
if(stage!=="before"&&stage!=="after")throw Error("Expected before or after stage");
const output=resolve(import.meta.dir,"🗑️generated/fem-isolated",stage);await mkdir(output,{recursive:true});const rows=[];
for(const row of cases.cases){
 const result=await fem3dToIfc4x3(femConnectivityFixture(row)),artifact=result.artifact!==undefined;
 if(artifact)await writeFile(resolve(output,row.id+".ifc"),await writeIfc4x3Step(result.artifact));
 rows.push({...row,artifact,actualCode:result.diagnostics.find((row:{severity:string})=>row.severity==="error")?.code??null});
}
await writeFile(resolve(output,"outcomes.json"),JSON.stringify({stage,rows},null,2)+"\n");
console.log("[DEBUG] "+JSON.stringify({stage,cases:rows.length,artifacts:rows.filter(row=>row.artifact).length}));
