/** 🕸️ Export actual evaluator closure over every released named seed for independent graph verification. */
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const base=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema');
const {ExpressEvaluator}=await import(pathToFileURL(resolve(base,'⚖️validation/evaluator.ts')).href),{ifc4x3Catalog}=await import(pathToFileURL(resolve(base,'📚️catalog/🟦️.ts')).href),cases=await Bun.file(resolve(base,'⚖️validation/typeof-select-cases.json')).json();
const evaluator=new ExpressEvaluator(),rows=[];
for(const name of [...Object.keys(ifc4x3Catalog.entities),...Object.keys(ifc4x3Catalog.types)]){
 const seeds=evaluator.ancestry(name),result=await evaluator.call('TYPEOF',[{kind:'entity',types:seeds,identity:1n}]);rows.push({id:name,seeds,actual:result.value.map(value=>value.value)});
}
for(const row of cases.seeds){const result=await evaluator.call('TYPEOF',[{kind:'entity',types:row.names,identity:1n}]);rows.push({id:row.id,seeds:row.names,actual:result.value.map(value=>value.value)});}
const output=resolve(import.meta.dir,'🗑️generated/typeof-select');await mkdir(output,{recursive:true});await writeFile(resolve(output,'owned.json'),JSON.stringify({schema:ifc4x3Catalog.schema,rows,nativeExecution:false},null,2)+'\n');console.log('[DEBUG] '+JSON.stringify({namedSeeds:1312,apiCases:cases.seeds.length,rows:rows.length}));

