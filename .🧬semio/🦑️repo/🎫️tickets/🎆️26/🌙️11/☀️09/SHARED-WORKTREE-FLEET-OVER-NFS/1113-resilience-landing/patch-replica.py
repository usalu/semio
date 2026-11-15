import sys,glob
root=sys.argv[1]
p=glob.glob(root+"/*/*/*/*/*/*replica/\U0001F7E6️.ts")[0]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:90],s.count(old))
    s=s.replace(old,new)

# imports
rep('import {chmod,copyFile,lstat,mkdir,open,opendir,readdir,readFile,readlink,rename,rm,rmdir,stat,symlink,truncate,unlink,writeFile} from "node:fs/promises";','import {chmod,copyFile,link,lstat,mkdir,open,opendir,readdir,readFile,readlink,rename,rm,rmdir,stat,symlink,truncate,unlink,utimes,writeFile} from "node:fs/promises";\nimport {uptime} from "node:os";')
rep('import {digestHexV1,foldTermV1,manifestTermV1,createAttesterV1,fleetReportV1,readRetirementV1,readRetirementsV1,type AttesterGitV1,type AttesterViewV1,type FleetReportV1} from "../🩺️attest/🟦️.ts";','import {digestHexV1,foldTermV1,manifestTermV1,codeDigestV1,codeRootV1,createAttesterV1,fleetReportV1,readRetirementV1,readRetirementsV1,type AttesterGitV1,type AttesterViewV1,type FleetReportV1,type RestartCheckV1} from "../🩺️attest/🟦️.ts";\nimport {createFailuresV1,createStoreGuardV1,pruneLostV1,readHealthV1,rotateLogV1,runSelfCheckV1,sealCodeV1,writeHealthV1,type FailuresV1,type HealthV1} from "../🛡️resilience/🟦️.ts";')

# options
rep('viewMilliseconds?:number;viewDebounceMilliseconds?:number}>;\n\n/** 📡️ One NDJSON progress line','viewMilliseconds?:number;viewDebounceMilliseconds?:number;watchRoot?:string;selfCheck?:(digest:string,signal?:AbortSignal)=>Promise<RestartCheckV1>;selfCheckMilliseconds?:number;restartJitterMilliseconds?:number;minimumUptimeMilliseconds?:number;storeDeadlineMilliseconds?:number;storeProbe?:(storeRoot:string)=>Promise<boolean>;storeGate?:()=>Promise<void>;logFiles?:readonly string[]}>;\n\n/** 📡️ One NDJSON progress line')
rep('"restarting"|"retired"|"error";stage?','"restarting"|"restart-refused"|"journal-repaired"|"state-recovered"|"retired"|"error";stage?')
rep(' readonly noted:Set<string>;\n',' readonly noted:Set<string>;\n readonly failures:FailuresV1;\n readonly key:string;\n applying:number;\n')
rep('retired:Readonly<{by:string;reason:string;wallMs:number}>|null}>;\n\n/** 🎞️ A written snapshot. */','retired:Readonly<{by:string;reason:string;wallMs:number}>|null;health:HealthV1|null}>;\n\n/** 🎞️ A written snapshot. */')
rep('type Machine=Readonly<{version:1;originId:string;groupId:string;storeRoot:string}>;','type Machine=Readonly<{version:1;originId:string;groupId:string;storeRoot:string;key?:string}>;')

# storeReachable with deadline and in-flight sharing
rep('''/** 📶️ Tells whether the store directory answers; an unmounted export leaves an empty mount point and counts as unreachable. */
export async function storeReachableV1(storeRoot:string):Promise<boolean>{
 try{const present=(await stat(storePathsV1(storeRoot).root)).isDirectory();if(present)markStoreSeenV1(storeRoot);return present;}catch{return false;}
}''','''const probesInFlight=new Map<string,Promise<boolean>>();

/** 📶️ Tells whether the store directory answers; an unmounted export leaves an empty mount point and counts as unreachable, and a store that does not answer within `deadlineMilliseconds` (15 s) counts as unreachable too. A `stat` that a hard mount blocks cannot be cancelled, so concurrent callers share the one probe that is still in flight instead of blocking one more worker thread each. */
export async function storeReachableV1(storeRoot:string,deadlineMilliseconds:number=15_000):Promise<boolean>{
 let probe=probesInFlight.get(storeRoot);
 if(probe===undefined){
  probe=stat(storePathsV1(storeRoot).root).then(info=>{const present=info.isDirectory();if(present)markStoreSeenV1(storeRoot);return present;},()=>false).finally(()=>{probesInFlight.delete(storeRoot);});
  probesInFlight.set(storeRoot,probe);
 }
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([probe,new Promise<boolean>(done=>{timer=setTimeout(()=>{done(false);},deadlineMilliseconds);timer.unref?.();})]);}
 finally{clearTimeout(timer);}
}''')

# writeJson fsync
rep('''async function writeJson(paths:LocalPathsV1,file:string,value:unknown):Promise<void>{
 await mkdir(paths.tmp,{recursive:true});
 const temporary=join(paths.tmp,token());
 try{await writeFile(temporary,JSON.stringify(value));await rename(temporary,file);}
 catch(error){await rm(temporary,{force:true});throw error;}
}''','''function syncPath(path:string,directory:boolean):void{
 if(directory&&windows)return;
 try{const descriptor=openSync(path,directory?"r":"r+");try{fsyncSync(descriptor);}finally{closeSync(descriptor);}}catch{}
}

const syncDirectory=async(directory:string):Promise<void>=>{syncPath(directory,true);};

async function writeJson(paths:LocalPathsV1,file:string,value:unknown):Promise<void>{
 await mkdir(paths.tmp,{recursive:true});
 const temporary=join(paths.tmp,token());
 try{
  await writeFile(temporary,JSON.stringify(value));syncPath(temporary,false);
  await rename(temporary,file);await syncDirectory(dirname(file));
 }catch(error){await rm(temporary,{force:true});throw error;}
}''')

# lock
a=s.index("async function readLock(file:string)")
b=s.index("type Tables=Readonly<")
lock='''const bootMilliseconds=():number=>Date.now()-Math.round(uptime()*1000);

async function readLock(file:string):Promise<Readonly<{pid:number;stale:boolean}>|undefined>{
 let text:string;
 try{text=await readFile(file,"utf8");}catch(error){if(gone(error))return undefined;throw error;}
 try{
  const value=JSON.parse(text) as {pid?:unknown;startedMs?:unknown;bootMs?:unknown;hb?:unknown};
  if(typeof value.pid==="number"&&Number.isSafeInteger(value.pid)&&value.pid>0){
   const own=value.pid===process.pid;
   let stale=own?typeof value.startedMs!=="number"||value.startedMs<Date.now()-process.uptime()*1000-1000:!alive(value.pid);
   if(!stale&&!own){
    const age=Date.now()-((await stat(file).catch(()=>undefined))?.mtimeMs??Date.now());
    if(value.hb===true&&age>staleLockMilliseconds)stale=true;
    else if(typeof value.bootMs==="number"&&Math.abs(value.bootMs-bootMilliseconds())>120_000&&age>30_000)stale=true;
   }
   return {pid:value.pid,stale};
  }
 }catch{}
 const info=await stat(file).catch(()=>undefined);
 return {pid:0,stale:info===undefined||Date.now()-info.mtimeMs>5000};
}

async function takeSteal(file:string):Promise<boolean>{
 try{const handle=await open(file,"wx");await handle.close();return true;}
 catch(error){
  if(code(error)!=="EEXIST")throw error;
  const info=await stat(file).catch(()=>undefined);
  if(info!==undefined&&Date.now()-info.mtimeMs>10_000)await rm(file,{force:true});
  return false;
 }
}

async function acquireLock(paths:LocalPathsV1,originId:string):Promise<()=>Promise<void>>{
 await mkdir(paths.directory,{recursive:true});
 const body=JSON.stringify({version:1,pid:process.pid,startedMs:Date.now(),originId,bootMs:bootMilliseconds(),hb:true}),steal=`${paths.lock}.steal`;
 for(let attempt=0;attempt<20;attempt++){
  try{
   const handle=await open(paths.lock,"wx");
   try{await handle.writeFile(body);await handle.sync();}finally{await handle.close();}
   const beat=setInterval(()=>{const now=new Date();utimes(paths.lock,now,now).catch(()=>{});},5000);
   beat.unref();
   return async()=>{clearInterval(beat);const held=await readLock(paths.lock);if(held?.pid===process.pid)await rm(paths.lock,{force:true});};
  }catch(error){
   if(code(error)!=="EEXIST")throw error;
   const held=await readLock(paths.lock);
   if(held===undefined)continue;
   if(held.stale){
    if(!await takeSteal(steal)){await sleep(50);continue;}
    try{const again=await readLock(paths.lock);if(again?.stale===true)await rm(paths.lock,{force:true});}
    finally{await rm(steal,{force:true});}
    continue;
   }
   if(held.pid===0){await sleep(50);continue;}
   throw new ReplicaBusyV1(held.pid,paths.lock);
  }
 }
 throw new ReplicaBusyV1(0,paths.lock);
}

'''
s=s[:a]+lock+s[b:]
rep('stateBytesPerEntry=320;','stateBytesPerEntry=320,staleLockMilliseconds=60_000,noteCapacity=4096,healthMilliseconds=5000,maintenanceMilliseconds=3_600_000,lostEntries=2000,logLimitFileBytes=64*1024*1024,logKeepBytes=4*1024*1024;')

# OriginTaken class after ReplicaBusyV1
rep('const bootMilliseconds=','''/** 🪪️ Raised when the origin id of this machine is already owned by another machine: `origins/<origin>/home.json` carries the key of the machine that published first. Two machines for one origin would truncate each other's journal, so the second one is refused instead. */
export class OriginTakenV1 extends Error{
 readonly originId:string;
 constructor(originId:string){super(`Origin ${originId} belongs to another machine: its home.json carries another key`);this.name="OriginTakenV1";this.originId=originId;}
}

/** ⏰️ Raised instead of issuing a clock value more than 60 s ahead of the wall clock: the own journal holds events from the future (the wall clock was set forward and corrected), and a new edit stamped lower would lose against them, one stamped higher would freeze this origin on every other machine. Publishing resumes when the wall clock has caught up. */
export class ClockAheadV1 extends Error{
 readonly seconds:number;
 constructor(seconds:number){super(`Own clock is ${seconds} s ahead of the wall clock; publishing waits until the wall clock has caught up`);this.name="ClockAheadV1";this.seconds=seconds;}
}

const bootMilliseconds=''')

# open: recovery and key
rep('''  const tables=tablesV1(),local=await loadLocal(paths,tables,true);
  if(local.log.size!==local.log.good)await truncate(paths.log,local.log.good);
  if(local.machine!==undefined&&local.machine.originId!==settings.originId)throw Error(`This worktree belongs to origin ${local.machine.originId}, not ${settings.originId}`);
  const machine:Machine={version:1,originId:settings.originId,groupId:settings.groupId,storeRoot:settings.storeRoot};
  await writeJson(paths,paths.machine,machine);
  let clock=local.clock;
  try{const head=await readHeadV1(settings.storeRoot,settings.originId);if(head)clock=mergeClockV1(clock,head.hlc,Number.MAX_SAFE_INTEGER-futureMilliseconds);}catch{}''','''  let tables=tablesV1(),local:Local,recovered:string|undefined,adoptable=false;
  try{local=await loadLocal(paths,tables,true);}
  catch(error){
   recovered=messageOf(error);adoptable=await readFile(paths.machine,"utf8").then(()=>true,()=>false);
   await setAsideState(paths,settings.now());
   tables=tablesV1();local=await loadLocal(paths,tables,true);
  }
  if(local.log.size!==local.log.good)await truncate(paths.log,local.log.good);
  if(local.machine!==undefined&&local.machine.originId!==settings.originId)throw Error(`This worktree belongs to origin ${local.machine.originId}, not ${settings.originId}`);
  let key=local.machine?.key??randomBytes(16).toString("hex");
  const home=await readHome(settings.storeRoot,settings.originId);
  if(home?.key!==undefined&&home.key!==key){if(adoptable)key=home.key;else throw new OriginTakenV1(settings.originId);}
  const machine:Machine={version:1,originId:settings.originId,groupId:settings.groupId,storeRoot:settings.storeRoot,key};
  await writeJson(paths,paths.machine,machine);
  let clock=local.clock;
  try{const head=await readHeadV1(settings.storeRoot,settings.originId);if(head)clock=mergeClockV1(clock,head.hlc,Number.MAX_SAFE_INTEGER-futureMilliseconds);}catch{}''')
rep('noted:new Set(),changed:new Set(),held:new Map(),say:event=>sink({version:1,...event}),release,clock,','noted:new Set(),failures:createFailuresV1({now:settings.now}),key,applying:0,changed:new Set(),held:new Map(),say:event=>sink({version:1,...event}),release,clock,')
rep('''  requeue(replica);
  await replayOwnTail(replica);
  settleHeap();
  return replica;''','''  requeue(replica);
  await replayOwnTail(replica);
  settleHeap();
  if(recovered!==undefined)replica.say({event:"state-recovered",message:recovered.slice(0,200)});
  const ahead=Math.ceil((replica.clock.ms-settings.now()-futureMilliseconds)/1000);
  if(ahead>0)replica.say({event:"clock-refused",reason:"own-clock-ahead",seconds:ahead});
  return replica;''')

# setAsideState + readHome helpers inserted before openReplicaV1 docstring
rep('/** 🚪️ Opens a replica:','''async function setAsideState(paths:LocalPathsV1,now:number):Promise<void>{
 const folder=join(paths.lost,`${String(now).padStart(13,"0")}-000000-state`);
 await mkdir(folder,{recursive:true});
 for(const file of [paths.state,paths.log,paths.cursor,paths.clock,paths.machine])await rename(file,join(folder,basename(file))).catch(error=>{if(!gone(error))throw error;});
}

type HomeRecord=Readonly<{group?:string;key?:string}>;

async function readHome(storeRoot:string,originId:string):Promise<HomeRecord|undefined>{
 try{
  const value=JSON.parse(await readFile(join(storePathsV1(storeRoot).origins,originId,"home.json"),"utf8")) as unknown;
  requireReplicaShapeV1("Home",value,"Closed replica home required");
  return value as HomeRecord;
 }catch{return undefined;}
}

/** 🚪️ Opens a replica:''')
open(p,"w",encoding="utf8").write(s)
print("part1 ok")
