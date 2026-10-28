import {writeFile} from 'node:fs/promises';
const base='C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/';
const {ExpressEvaluator,expressAggregate}=await import(base+'evaluator.ts');const {parseExpressExpression}=await import(base+'parser.ts');
const scope=new Map([['B',expressAggregate([{kind:'string',value:'x'},{kind:'string',value:'x'}],'BAG')],['S',expressAggregate([{kind:'string',value:'x'}],'SET')]]);
const rows=[];for(const source of ["SIZEOF('INTEGER' + TYPEOF(1))","TYPEOF(B * S)","SIZEOF((B * S) + ['x'])","'IFC4X3_ADD2.IFCAXIS2PLACEMENT' IN TYPEOF(IFCAXIS2PLACEMENT3D(IFCCARTESIANPOINT([0.,0.,0.]),?,?))"]){rows.push({source,result:await new ExpressEvaluator().evaluate(parseExpressExpression(source),scope)});}
const text=JSON.stringify({nativeExecution:false,producer:'reference-typescript',rows},(_,v)=>typeof v==='bigint'?v.toString():v,2);await writeFile(new URL('./🗑️generated/retained-type-inverse-probe.json',import.meta.url),text);console.log(text);
