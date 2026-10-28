/** 🎛️ Probe public standalone evaluator state against its original finite grants. */
import {writeFile} from 'node:fs/promises';
import {compileIfc4x3Express,ExpressEvaluator,parseExpressExpression,type ExpressControl} from 'C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/index.ts';
const compiled=await compileIfc4x3Express({batchSize:4096});if(!compiled.complete)throw Error('Actual compiler failed');const rows=[];
for(const kind of ['work','depth','control'])for(const mutation of [false,true]){
 let evaluator:ExpressEvaluator,observed=0,changed=false;const control:ExpressControl={batchSize:1,maximumWork:kind==='depth'?1000:4,maximumDepth:kind==='depth'?1:256,progress:()=>{observed++;if(!mutation)return;if(kind==='work'){evaluator.work=0;changed=true;}else if(kind==='depth'){evaluator.depth=0;changed=true;}else changed=Reflect.set(evaluator,'control',{...evaluator.control,maximumWork:1000});}};
 evaluator=new ExpressEvaluator(compiled.program,control);let result:unknown;try{const value=await evaluator.evaluate(parseExpressExpression(kind==='depth'?'1+(2*3)':'SIZEOF([1:20])'));result={kind:value.kind,value:String(value.value)};}catch(error){result={error:error instanceof Error?error.message:String(error),code:error&&typeof error==='object'&&'code' in error?error.code:undefined};}
 rows.push({kind,mutation,originalWork:control.maximumWork,originalDepth:control.maximumDepth,observed,changed,work:evaluator.work,depth:evaluator.depth,controlWritable:Object.getOwnPropertyDescriptor(evaluator,'control')?.writable,result});
}
await writeFile(new URL('./🗑️generated/express-evaluator-state-before.json',import.meta.url),JSON.stringify({actualProgramRules:compiled.program.rules.length,rows,nativeExecution:false},null,2),{flag:'wx'});console.log('[DEBUG] '+JSON.stringify(rows));
