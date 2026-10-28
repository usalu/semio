/** ⏱️ Bound cold metadata scan work using actual reference TYPEOF calls; no native runtime claim. */
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const io=resolve(process.cwd(),'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'),base=resolve(io,'../🧬️schema/⚖️validation');
const {ExpressEvaluator}=await import(pathToFileURL(resolve(base,'evaluator.ts')).href),{compileIfc4x3Express}=await import(pathToFileURL(resolve(base,'compiler.ts')).href),{validateIfc4x3Express}=await import(pathToFileURL(resolve(base,'population.ts')).href),{parseIfc4x3Step}=await import(pathToFileURL(resolve(io,'step.ts')).href);
const program=(await compileIfc4x3Express()).program,original=ExpressEvaluator.prototype.call,rows=[];let calls=0,seeds=0;
ExpressEvaluator.prototype.call=async function(name,...args){const result=await original.call(this,name,...args);if(name.toUpperCase()==='TYPEOF'){calls++;seeds+=result.value.filter(value=>String(value.value).startsWith('IFC4X3_ADD2.')).length;}return result;};
try{for(const name of ['triangulated','alignment','structural']){calls=0;seeds=0;const snapshot=await parseIfc4x3Step(await readFile(resolve(io,'🧪️tests/'+name+'.ifc'),'utf8'),{batchSize:4096}),report=await validateIfc4x3Express(snapshot,{batchSize:4096},program);rows.push({name,entities:snapshot.entities.length,complete:report.complete,valid:report.valid,referenceWork:report.work,typeofCalls:calls,qualifiedResultNames:seeds,coldAdditionalDeclarationMemberLowerBound:seeds*(436+271),nativeExecution:false});}}finally{ExpressEvaluator.prototype.call=original;}
await writeFile(resolve(import.meta.dir,'🗑️generated/cold-typeof-work.json'),JSON.stringify({rows},null,2)+'\n');console.log('[DEBUG] '+JSON.stringify(rows));

