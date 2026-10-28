import {compileIfc4x3Express,IfcExpressContext,parseExpressExpression} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/index.ts';
import {ifc4x3Catalog} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
const program=(await compileIfc4x3Express()).program;
const declarations=Object.entries(ifc4x3Catalog.entities);
const snapshot:any={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:declarations.map(([name,entity],index)=>({id:BigInt(index+1),name,args:entity.allAttributes.map(()=>({kind:'unset'})),complex:[]}))};
const context=new IfcExpressContext(snapshot,program,{maximumWork:20000000}),results:any[]=[];
await context.initialize();
for(const rule of program.rules){const index=declarations.findIndex(([name])=>name===rule.owner);const self=index<0?{kind:'unknown'}:context.entities.get(BigInt(index+1));try{const result=await context.evaluator.evaluate(rule.expression,new Map([['SELF',self as any],...(self?.kind==='entity'?Array.from(self.fields??[]):[])]));results.push({owner:rule.owner,rule:rule.label,kind:result.kind,result:result.value});}catch(error){results.push({owner:rule.owner,rule:rule.label,error:String(error)});}}
for(const fn of program.functions){try{const result=await context.evaluator.call(fn.name,fn.parameters.flatMap(parameter=>parameter.names.map(()=>({kind:'unknown'}))));results.push({function:fn.name,kind:result.kind});}catch(error){results.push({function:fn.name,error:String(error)});}}
await Bun.write(new URL('./🗑️generated/express-execution-coverage.json',import.meta.url),JSON.stringify(results,(_,v)=>typeof v==='bigint'?String(v):v,2));console.log(results.length,results.filter(row=>row.error));
