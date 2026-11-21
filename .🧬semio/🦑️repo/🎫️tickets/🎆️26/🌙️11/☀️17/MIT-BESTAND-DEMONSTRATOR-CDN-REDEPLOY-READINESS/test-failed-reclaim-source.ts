import { mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { sharedProvenanceCustodyV1, cargoProvenancePhysicalControlV1 } from '../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts';
import { failedPassConcurrentEditV1 } from '../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️failure-churn/🟦️.ts';
const dir = mkdtempSync(join(import.meta.dir, '🗑️generated', 'failed-reclaim-'));
const source=resolve(dir,'source.rs'),output=resolve(dir,'build_script_build');
const control=()=>cargoProvenancePhysicalControlV1({cancelled:()=>false,onProgress:()=>{}});
try {
 writeFileSync(source,'source-v1');writeFileSync(output,'compiler-output');
 const custody=sharedProvenanceCustodyV1(dir,control), port=custody.invocation().port;
 await port.digest(source);await port.digest(output);rmSync(output);
 const falseEdit=await failedPassConcurrentEditV1({startedMs:Date.now()+1000,files:[source],recheck:()=>custody.recheck()});
 if(falseEdit?.path!==output||falseEdit.reason!=='vanished')throw Error('expected deleted output to be misclassified');
 const inputView=custody.files.subset(new Set([source]));await inputView.recheck(control);
 writeFileSync(source,'source-v2');
 let caught=false;try{await inputView.recheck(control);}catch{caught=true}if(!caught)throw Error('source drift was not detected');
 console.log('real custody reproduces deleted-output false edit; a source-only file subset still catches changed bytes');
} finally {rmSync(dir,{recursive:true,force:true});}
