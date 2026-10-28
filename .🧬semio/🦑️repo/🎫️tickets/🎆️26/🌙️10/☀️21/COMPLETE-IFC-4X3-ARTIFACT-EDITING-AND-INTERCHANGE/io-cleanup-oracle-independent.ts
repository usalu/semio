/** ⚖️ Compare public first-refusal cleanup semantics with pinned Babel IteratorClose. */
import {createRequire} from 'node:module';
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const ticket=import.meta.dir,base=resolve(ticket,'../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any');
const control=await import(pathToFileURL(resolve(base,'🚪️io/control.ts')).href),edit=await import(pathToFileURL(resolve(base,'🧬️schema/🔺️diff/🟦️.ts')).href);
const vectors=JSON.parse(await readFile(resolve(base,'🚪️io/🧪️tests/control-failure-cases.json'),'utf8'));
const provenance=JSON.parse(await readFile(resolve(ticket,'🗑️generated/close-ref/provenance.json'),'utf8'));
const hash=createHash('sha256').update(await readFile(resolve(ticket,'🗑️generated/close-ref/createForOfIteratorHelper.js'))).digest('hex');
if(provenance.version!=='7.28.4'||hash!=='50c8759e852b08a52b46901e6871a6dd9ec5dfbfcf6ed5501a52ae6204d5ef2f')throw Error('Independent helper provenance changed');
interface IteratorClose {s():void;n():IteratorResult<void,boolean>;e(error:unknown):void;f():void}
const independent=createRequire(import.meta.url)(resolve(ticket,'🗑️generated/close-ref/createForOfIteratorHelper.js')) as (steps:Generator<void,boolean,void>)=>IteratorClose;
const rows=[];
for(const row of vectors.failures){
 const original=row.kind==='edit'?new edit.IfcEditError(row.code,'original callback'):row.kind==='step'?new control.IfcStepError(row.code,'original callback'):row.kind==='undefined'?undefined:row.kind==='number'?17:row.kind==='error'?new Error('original callback'):Object.freeze({original:true});
 const results=[];
 for(const backend of ['actual','babel']){
  let closed=false,caught=false,exact=false,calls=0;
  function* steps():Generator<void,boolean,void>{try{yield;yield;return true;}finally{closed=true;throw new Error('distinct cleanup failure');}}
  try{
   if(backend==='actual')await control.runIfcIoSteps(steps(),{batchSize:1,progress:(event:{completed:number})=>{if(event.completed>0){calls++;throw original;}}},vectors.cleanup.phase);
   else{const iterator=independent(steps());try{iterator.s();for(let next=iterator.n();!next.done;next=iterator.n()){calls++;throw original;}}catch(error){iterator.e(error);}finally{iterator.f();}}
  }catch(error){caught=true;exact=error===original;}
  results.push({closed,caught,exact,calls});
 }
 if(JSON.stringify(results[0])!==JSON.stringify(results[1])||!results[0]!.exact||!results[0]!.closed||results[0]!.calls!==1)throw Error('IteratorClose disagreement: '+JSON.stringify(row));
 rows.push({input:row,actual:results[0],independent:results[1]});
}
await writeFile(resolve(ticket,'🗑️generated/io-cleanup-oracle-independent.json'),JSON.stringify({library:{name:provenance.name,version:provenance.version,integrity:provenance.integrity,sha256:provenance.sha256,helperSha256:hash},rows,scope:'Nine exact original observer values versus a distinct finally throw; finite yielding cleanup and native proof are separate'},null,2));
console.log('[DEBUG] '+JSON.stringify({compared:rows.length,mismatches:0,library:provenance.name,version:provenance.version}));

