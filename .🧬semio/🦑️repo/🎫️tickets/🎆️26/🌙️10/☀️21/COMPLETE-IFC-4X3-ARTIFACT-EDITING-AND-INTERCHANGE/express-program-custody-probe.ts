/** 🧫 Probe genuine compiler program ownership against an independently known invalid direction. */
import {writeFile} from 'node:fs/promises';
import {compileIfc4x3Express,validateIfc4x3Express} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/index.ts';
import {parseIfc4x3Step} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.ts';
import model from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/models.json';
const source=await parseIfc4x3Step(model.header+'#1=IFCDIRECTION((0.,0.,0.));'+model.footer),rows=[];
for(const phase of [undefined,'admit','normalize','structure','evaluate','validation-result']){
 const compiled=await compileIfc4x3Express({batchSize:4096});if(!compiled.complete)throw Error('Genuine compiler failed');let changed=false;
 const result=await validateIfc4x3Express(source,{batchSize:1,progress:row=>{if(row.phase===phase&&!changed){changed=true;compiled.program.rules.length=0;}}},compiled.program);
 rows.push({phase:phase??'baseline',changed,remainingRules:compiled.program.rules.length,complete:result.complete,valid:result.valid,diagnostics:result.diagnostics.map(row=>({code:row.code,owner:row.owner,rule:row.rule})),results:result.results.length});
}
await writeFile(new URL('./🗑️generated/express-program-custody-before.json',import.meta.url),JSON.stringify({actualCompiler:true,source:'#1=IFCDIRECTION((0.,0.,0.));',expected:'IFCDIRECTION.MAGNITUDEGREATERZERO',rows,nativeExecution:false},null,2),{flag:'wx'});
console.log('[DEBUG] '+JSON.stringify(rows));
