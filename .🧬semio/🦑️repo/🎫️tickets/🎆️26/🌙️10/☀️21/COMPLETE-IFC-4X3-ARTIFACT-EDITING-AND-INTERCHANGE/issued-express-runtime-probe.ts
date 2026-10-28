/** 🧾️ Retain actual issued-owner outputs without translating or replacing the canonical program. */
import {readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const root=resolve(import.meta.dir,'../../../../../../..');
const directory=resolve(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation');
const api=await import(pathToFileURL(resolve(directory,'index.ts')).href),fixture=await import(pathToFileURL(resolve(directory,'🧪️coverage/fixture.ts')).href),step=await import(pathToFileURL(resolve(directory,'../../🚪️io/step.ts')).href);
const cases=JSON.parse(await readFile(resolve(directory,'🧪️coverage/cases.json'),'utf8')),language=JSON.parse(await readFile(resolve(directory,'🧪️coverage/language.json'),'utf8')),model=JSON.parse(await readFile(resolve(directory,'models.json'),'utf8'));
const compilation=await api.compileIfc4x3Express(),functions=[],globals=[],expressions=[];
for(const row of cases.functions){const evaluator=new api.ExpressEvaluator(compilation.program),args=await fixture.issuedOwnerArguments(evaluator,row.args);functions.push({id:row.id,owner:row.owner,value:fixture.issuedOwnerValue(await evaluator.call(row.owner,args)),work:evaluator.work});}
for(const row of cases.globals){const result=await api.validateIfc4x3Express(await step.parseIfc4x3Step(model.header+row.body+model.footer),{},compilation.program);globals.push({id:row.id,owner:row.owner,complete:result.complete,valid:result.valid,results:result.results.filter((item:any)=>item.kind==='global'),diagnostics:result.diagnostics,work:result.work});}
for(const row of language){const evaluator=new api.ExpressEvaluator(compilation.program),scope=new Map();for(const [name,value]of Object.entries(row.bindings))scope.set(name,(await fixture.issuedOwnerArguments(evaluator,[value]))[0]);try{expressions.push({id:row.id,value:fixture.issuedOwnerValue(await evaluator.evaluate(api.parseExpressExpression(row.expression),scope))});}catch(error){expressions.push({id:row.id,error:(error as any).code,message:String(error)});}}
process.stdout.write(JSON.stringify({nativeExecuted:false,totalBranchCoverage:false,functions,globals,expressions},(_,value)=>typeof value==='bigint'?String(value):value,2)+'\n');
