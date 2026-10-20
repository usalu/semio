import * as fs from 'node:fs/promises';
import {constants} from 'node:fs';
import {join,dirname} from 'node:path';
import {fileURLToPath} from 'node:url';
import {compileJsonSchemaSubset} from '../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts';
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from '../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts';
import {createHostSource} from '../📦️fresh/host.ts';
import {fileDigest} from './entry.ts';
import {hash} from './core.ts';
import {readPlan} from '../🌱️linux/script.ts';
import {observeBoundary} from './boundary.ts';
import schema from './boundary.schema.json' with {type:'json'};
const generated=join(dirname(fileURLToPath(import.meta.url)),'../🗑️generated');
async function main(){const[name,expected,...extra]=process.argv.slice(2);if(extra.length||!/^fresh-boundary-[a-f0-9]{32}-(before-lock|after-lock|after-sync|after-gates)\.json$/u.test(name??'')||!/^[a-f0-9]{64}$/u.test(expected??''))throw Error('Exact sealed boundary input required');const raw=(await fileDigest(join(generated,name),65536,()=>{},true)).body!;if(hash(raw)!==expected)throw Error('Boundary input changed');const p=JSON.parse(raw.toString());if(compileJsonSchemaSubset(schema).validate(p).length)throw Error('Closed boundary plan');await receiveScriptProcessInvocation(process.env,async original=>{const issued=createIssuedResourceControlV1(original,'fresh:Source-boundary',p.milliseconds),check=()=>{issued.signal.throwIfAborted();if((issued.remainingMilliseconds()??1)<=0)throw Error('Boundary deadline');},read=async(ref:any,json=true)=>{const b=(await fileDigest(ref.path,ref.bytes,check,true)).body!;if(hash(b)!==ref.sha256||b.length!==ref.bytes)throw Error('Pinned boundary evidence changed');return json?JSON.parse(b.toString()):b;},cohort=readPlan(await read(p.cohortPlan)),initial=await read(p.initialManifest);if(p.initialManifest.sha256!==cohort.source.manifestSha256||initial.kind!=='fresh-source-manifest')throw Error('Initial Source binding');for(const r of p.ownerEvidence)await read(r,false);const control={signal:issued.signal,remainingMilliseconds:()=>issued.remainingMilliseconds()??p.milliseconds,progress:async(stage:string,facts:any)=>{console.log('[DEBUG] '+stage+' '+JSON.stringify(facts));await issued.yieldContinuation();}},source=createHostSource(p.workspaceRoot,issued,initial.criticalPaths,original),result=await observeBoundary(p,cohort,source,control,()=>new Date().toISOString());check();const fd=await fs.open(join(generated,'fresh-boundary-'+cohort.cohort+'-'+p.boundary+'-observation.json'),constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,384);try{await fd.writeFile(JSON.stringify(result)+'\n');await fd.sync();}finally{await fd.close();}});}
if(import.meta.main)await main();
