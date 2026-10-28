/** 🔎 Observe actual unchanged issued numeric owners without substituting bodies or program authority. */
import {resolve} from 'node:path';
const root=resolve(import.meta.dir,'../../../../../../..'),base=resolve(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation');
const {compileIfc4x3Express,ExpressEvaluator}=await import(resolve(base,'index.ts')),{numericOwnerArguments,numericOwnerValue}=await import(resolve(base,'🧪️numeric/fixture.ts'));
const data=await Bun.file(resolve(base,'🧪️numeric/cases.json')).json(),program=(await compileIfc4x3Express()).program,functions=[];
for(const row of data.functions){const evaluator=new ExpressEvaluator(program);try{const args=await numericOwnerArguments(evaluator,row.args),value=await numericOwnerValue(evaluator,await evaluator.call(row.owner,args),row.projection);functions.push({id:row.id,owner:row.owner,value,work:evaluator.work});}catch(error){functions.push({id:row.id,owner:row.owner,error:String(error),work:evaluator.work});}}
await Bun.write(resolve(import.meta.dir,'🗑️generated/issued-express-numeric-runtime.json'),JSON.stringify({source:data.source,functions},null,2)+'\n');
console.log('[DEBUG] '+JSON.stringify({functions:functions.length,errors:functions.filter(row=>'error'in row).length}));
