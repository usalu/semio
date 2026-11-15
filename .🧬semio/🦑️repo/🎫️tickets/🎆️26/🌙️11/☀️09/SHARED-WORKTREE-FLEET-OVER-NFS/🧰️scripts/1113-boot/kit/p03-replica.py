import sys;sys.path.insert(0,__import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from lib import *
R='🔁️replica/🟦️.ts'
sub(R,'import {basename,dirname,join,relative,sep} from "node:path";','import {basename,dirname,join,relative,resolve,sep} from "node:path";')
sub(R,'import {uptime} from "node:os";','import {tmpdir,uptime} from "node:os";')
sub(R,'import {chmod,copyFile,link,lstat,mkdir,open,','import {chmod,copyFile,link,lstat,mkdir,mkdtemp,open,')
sub(R,'import {digestHexV1,foldTermV1,manifestTermV1,createAttesterV1,fleetReportV1,readAttestationV1,','import {digestHexV1,foldTermV1,manifestTermV1,createAttesterV1,fleetReportV1,readAttestationV1,readAttestationsV1,')
sub(R,'type FleetGuardsV1,type FleetReportV1} from "../🩺️attest/🟦️.ts";','type FleetFallbackV1,type FleetGuardsV1,type FleetReportV1} from "../🩺️attest/🟦️.ts";')
sub(R,'import {createFailuresV1,createStoreGuardV1,pruneLostV1,readHealthV1,','import {codeDirectoryV1,createFailuresV1,createStoreGuardV1,importClosureV1,installBootStageV1,listStoreCodeV1,moduleSourcesV1,pruneLostV1,pruneStoreCodeV1,publishCodeSnapshotV1,readBeatV1,readHealthV1,StoreStalledV1,')
sub(R,'type FailuresV1,type HealthV1} from "../🛡️resilience/🟦️.ts";','type FailuresV1,type HealthV1,type SealedCodeV1} from "../🛡️resilience/🟦️.ts";')
sub(R,'storeGate?:()=>Promise<void>;logFiles?:readonly string[]}>;','storeGate?:()=>Promise<void>;logFiles?:readonly string[];fallback?:FleetFallbackV1;admit?:(digest:string,signal?:AbortSignal)=>Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>;publishCodeMilliseconds?:number;codeKeep?:number;returnMilliseconds?:number}>;')
sub(R,'daemon:Readonly<{running:boolean;pid:number|null}>;sync:boolean;','daemon:Readonly<{running:boolean;pid:number|null;busySeconds?:number}>;code?:Readonly<{source:"worktree"|"snapshot"|"store";phase:"boot"|"open"|"run"}>;sync:boolean;')
sub(R,'timeoutMs:options.selfCheckMilliseconds??60_000','timeoutMs:options.selfCheckMilliseconds??180_000')
sub(R,'check:options.selfCheck??checkCode,busy:','check:options.selfCheck??checkCode,admit:options.admit??(options.selfCheck===undefined?(digest,aborted)=>admitCodeV1(replica,{digest,running:attester.running(),moduleRoot:watching,bounded,signal:aborted}):undefined),fallback:options.fallback,returnMilliseconds:options.returnMilliseconds,busy:')
sub(R,' const attester=createAttesterV1({',' const bounded=async<T>(work:()=>Promise<T>):Promise<T>=>{\n  if(guard.state()!=="ok")throw new StoreStalledV1();\n  let timer:ReturnType<typeof setTimeout>|undefined;\n  try{return await Promise.race([work(),new Promise<never>((_,fail)=>{timer=setTimeout(()=>{fail(new StoreStalledV1());},options.storeDeadlineMilliseconds??10_000);timer.unref?.();})]);}\n  finally{clearTimeout(timer);}\n };\n const attester=createAttesterV1({')
sub(R,'''void sealCodeV1({root:settings.worktree,moduleRoot:running,digest:attester.running(),digestOf:codeDigestV1,signal}).catch(()=>{});});''','''void sealCodeV1({root:settings.worktree,moduleRoot:running,digest:attester.running(),digestOf:codeDigestV1,signal}).catch(()=>{}).then(()=>installBootStageV1(settings.worktree,join(running,"🔁️replica","📜️script.ts"))).catch(()=>{});});
 const bootWall=Date.now(),publishAfter=options.publishCodeMilliseconds??600_000;
 let shared=false;
 const shareCode=async():Promise<void>=>{
  const digest=attester.running();
  await bounded(()=>publishCodeSnapshotV1({root:settings.worktree,store:settings.storeRoot,digest,originId:settings.originId}));
  const live=new Set(Object.values(await bounded(()=>readAttestationsV1(settings.storeRoot))).map(item=>item.code));
  live.add(digest);
  await bounded(()=>pruneStoreCodeV1(settings.storeRoot,options.codeKeep??4,live));
 };''')
sub(R,'''    if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}''','''    if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}
    if(!shared&&wall-bootWall>=publishAfter&&guard.state()==="ok"&&attester.running()!==""&&!parked&&replica.failures.summary().every(item=>wall-item.lastMs>=publishAfter)){shared=true;void shareCode().catch(()=>{shared=false;});}''')
append(R,files('replica-append.ts'))
sub(R,'held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree),beat=await readHealthV1(settings.worktree);','held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree),beat=await readHealthV1(settings.worktree),pulse=await readBeatV1(settings.worktree,settings.now());')
sub(R,'daemon:{running:held!==undefined&&!held.stale,pid:held!==undefined&&!held.stale&&held.pid>0?held.pid:null},sync,','daemon:{running:held!==undefined&&!held.stale||pulse!==undefined,pid:held!==undefined&&!held.stale&&held.pid>0?held.pid:pulse?.pid??null,...pulse!==undefined&&pulse.lagMs>=5000?{busySeconds:Math.floor(pulse.lagMs/1000)}:{}},...pulse===undefined?{}:{code:{source:pulse.source,phase:pulse.phase}},sync,')
sub(R,'health:beat===undefined||!(held!==undefined&&!held.stale)?null:','health:beat===undefined||!(held!==undefined&&!held.stale||pulse!==undefined)?null:')
sub(R,'''   if(!stale&&!own){
    const age=Date.now()-((await stat(file).catch(()=>undefined))?.mtimeMs??Date.now());
    if(value.hb===true&&age>staleLockMilliseconds)stale=true;''','''   if(!stale&&!own){
    const age=Date.now()-((await stat(file).catch(()=>undefined))?.mtimeMs??Date.now());
    if(value.hb===true&&age>staleLockMilliseconds)stale=(await readBeatV1(dirname(dirname(dirname(file))),Date.now()))?.pid!==value.pid;''')
