import {ExpressEvaluator,expressAggregate} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/evaluator.ts';
import {parseExpressExpression} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/parser.ts';
import {compileIfc4x3Express} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/compiler.ts';
const evaluator=new ExpressEvaluator();
const one={kind:'integer' as const,value:1n},two={kind:'integer' as const,value:2n},u={kind:'unknown' as const};
const rows=[];
for(const [name,op,a,b,expected] of [
 ['bag multiplicity','<=',expressAggregate([one,one],'BAG'),expressAggregate([one,two],'BAG'),'false'],
 ['indeterminate subset','<=',expressAggregate([u],'SET'),expressAggregate([one],'SET'),'unknown'],
 ['indeterminate intersection','*',expressAggregate([u],'SET'),expressAggregate([one],'SET'),'unknown'],
] as const){const actual=await evaluator.binary(op,a,b);rows.push({name,expected,actual});}
for(const source of ['TYPEOF(IFCCARTESIANPOINT([0.,0.,0.]))','TYPEOF(IFCPOSITIVELENGTHMEASURE(1.))','[1,2,3][0:1]'])rows.push({source,actual:await evaluator.evaluate(parseExpressExpression(source))});
const program=(await compileIfc4x3Express()).program;
rows.push({unique:program.unique.map(x=>({owner:x.owner,label:x.label,expression:x.expression}))});
await Bun.write(new URL('./🗑️generated/review-probes.json',import.meta.url),JSON.stringify(rows,(_,v)=>typeof v==='bigint'?String(v):v,2));
console.log(JSON.stringify(rows,(_,v)=>typeof v==='bigint'?String(v):v,2));
