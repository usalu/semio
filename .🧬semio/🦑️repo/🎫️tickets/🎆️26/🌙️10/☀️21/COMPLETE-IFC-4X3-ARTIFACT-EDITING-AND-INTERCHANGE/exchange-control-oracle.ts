/** 🔬 Compare original cancellation outcomes with the pinned independent p-retry implementation. */
import {mkdir,writeFile,readFile} from 'node:fs/promises';
import {join,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {ifcExchangeControlEndpoint} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/exchange-control-fixtures.ts';
import cases from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/exchange-control-cases.json';
import type {IfcIoControl} from '../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/control.ts';

const modules=resolve(import.meta.dir,'🗑️generated/ctrlmod'),{default:pRetry}=await import(pathToFileURL(join(modules,'p-retry/index.js')).href);
type Options=IfcIoControl&{retries?:number};
async function observe(run:(control:Options)=>Promise<unknown>){
 const original=new AbortController(),replacement=new AbortController();let callbacks=0,replacementCallbacks=0,published=false,cancelled=false;
 const control:Options={signal:original.signal,progress:()=>{callbacks++;control.signal=replacement.signal;control.progress=()=>replacementCallbacks++;original.abort();}};
 try{await run(control);published=true;}
 catch(error){cancelled=original.signal.aborted&&(error===original.signal.reason||(error as {code?:string}).code===cases.code);if(!cancelled)throw error;}
 return{cancelled,callbacks,replacementCallbacks,published};
}
const rows=[];
for(const endpoint of cases.endpoints){
 const owned=await observe(await ifcExchangeControlEndpoint(endpoint));
 const independent=await observe(async control=>{
  control.retries=0;
  return pRetry(async()=>{control.progress?.({phase:'checkpoint',completed:0,total:1});return 'complete';},control);
 });
 if(JSON.stringify(owned)!==JSON.stringify(independent)||!owned.cancelled||owned.callbacks!==cases.callbacksBeforeCancellation||owned.replacementCallbacks||owned.published)throw new Error(`Original cancellation mismatch: ${endpoint}`);
 rows.push({endpoint,owned,independent});
}
const provenance=JSON.parse(await readFile(join(modules,'provenance.json'),'utf8'));
const output=resolve(import.meta.dir,'🗑️generated/exchange-control');await mkdir(output,{recursive:true});
await writeFile(join(output,'oracle.json'),JSON.stringify({implementation:'p-retry',version:'8.0.1',provenance,rows,nativeExecution:false},null,2)+'\n');
console.log(JSON.stringify({implementation:'p-retry',version:'8.0.1',endpoints:rows.length,mismatches:0,nativeExecution:false}));
