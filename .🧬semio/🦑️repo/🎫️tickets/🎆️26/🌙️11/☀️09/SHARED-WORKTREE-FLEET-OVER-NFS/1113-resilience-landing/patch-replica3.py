import sys,glob
root=sys.argv[1]
p=glob.glob(root+"/*/*/*/*/*/*replica/\U0001F7E6️.ts")[0]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:90],s.count(old))
    s=s.replace(old,new)

# maintenance helper before runV1 doc
rep('/** 🏃️ The long-running daemon:','''const defaultLogFiles=(worktree:string):readonly string[]=>[...process.platform==="darwin"?["/var/log/semio-fleet-replica.log"]:[],join(worktree,".cache","f","native-bootstrap.log")];

async function maintain(replica:ReplicaV1,options:ReplicaOptionsV1):Promise<void>{
 const {settings,paths}=replica,now=Date.now();
 await pruneLostV1(paths.lost,{now,retainMs:settings.retentionDays*86_400_000,maxEntries:lostEntries}).catch(()=>0);
 for(const file of options.logFiles??defaultLogFiles(settings.worktree))await rotateLogV1(file,{maxBytes:logLimitFileBytes,keepBytes:logKeepBytes}).catch(()=>false);
 replica.failures.forgetOlderThan(3_600_000);
 const cool=cooling.get(replica);
 if(cool!==undefined)for(const path of cool.keys()){const id=replica.book.find(path);if(id===undefined||!replica.queue.hasId(id))cool.delete(path);}
}

/** 🏃️ The long-running daemon:''')

# runV1: replace from "export async function runV1" to "/** 🌱️ Seeds the store"
a=s.index("export async function runV1(")
b=s.index("/** 🌱️ Seeds the store")
body=s[a:b]
# docstring addition
d=s.rindex("/** 🏃️ The long-running daemon:",0,a)
doc_end=s.index("*/",d)
doc=s[d:doc_end]
doc+=" Every store turn runs under a deadline guard: a store call that a hard mount blocks cannot be cancelled, so when the store has not answered a probe within `storeDeadlineMilliseconds` (10 s) the round is abandoned and aborted, `store-unreachable` is reported, no further store call is issued while one is outstanding, local work (hints, attestation to memory, state persistence, the health file) keeps running, and the daemon resumes by itself when the store answers again. New code is adopted only after a fresh process proved that it starts (`restart-refused` otherwise), after a deterministic per-origin delay of 0 to 60 s and while the incoming queue is empty. Recurring failures are counted per kind, announced at most once a minute and shown in `status` and the attestation. "
new_doc_text=doc
s=s[:d]+new_doc_text+s[doc_end:]
a=s.index("export async function runV1(")
b=s.index("/** 🌱️ Seeds the store")

new_run='''export async function runV1(options:ReplicaOptionsV1,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 const replica=await openReplicaV1(options,sink),settings=replica.settings,touched=new Set<string>();
 let rounds=0,walk:Promise<void>|undefined,walked:WalkV1|undefined,wake:(()=>void)|undefined,lastTouch=0,full=true,nextScan=0,outage=false,recovering=true,watcher:ReturnType<typeof watch>|undefined,suspended=false,failed=0,healthAt=-Infinity,maintainedAt=-Infinity;
 const pause=(milliseconds:number)=>new Promise<void>(done=>{
  const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);wake=undefined;done();};
  const timer=setTimeout(finish,milliseconds);
  signal.addEventListener("abort",finish,{once:true});wake=finish;
 });
 const guard=createStoreGuardV1({probe:()=>(options.storeProbe??storeReachableV1)(settings.storeRoot),deadlineMs:options.storeDeadlineMilliseconds??10_000,now:Date.now,changed:()=>{}});
 const down=()=>{if(!outage){outage=true;replica.say({event:"store-unreachable",path:settings.storeRoot,reason:guard.state()});}};
 let syncOn=true,restart=false,parked=false;
 const running=options.codeRoot??codeRootV1,watching=options.watchRoot??running;
 const checkCode=async(digest:string,aborted?:AbortSignal):Promise<RestartCheckV1>=>{
  const result=await runSelfCheckV1({command:process.execPath,args:[join(watching,"🔁️replica","📜️script.ts"),"self-check"],cwd:settings.worktree,timeoutMs:options.selfCheckMilliseconds??60_000,signal:aborted});
  if(!result.ok)return {ok:false,reason:result.reason};
  if(result.digest!==digest)return {ok:false,reason:"code changed during the check",again:true};
  const sealed=await sealCodeV1({root:settings.worktree,moduleRoot:watching,digest,digestOf:codeDigestV1,signal:aborted});
  if(sealed.reason==="code-changed-while-copying")return {ok:false,reason:sealed.reason,again:true};
  if(sealed.reason!==null)noteOnce(replica,`seal:${sealed.reason}`,{event:"error",stage:"seal",message:sealed.reason});
  return {ok:true};
 };
 const attester=createAttesterV1({storeRoot:settings.storeRoot,originId:settings.originId,groupId:settings.groupId,now:settings.now,heartbeatMilliseconds:options.attestMilliseconds,stableMilliseconds:options.restartStableMilliseconds,codeRoot:options.codeRoot,watchRoot:options.watchRoot,check:options.selfCheck??checkCode,busy:()=>replica.queue.size>0||replica.applying>0,jitterSpanMilliseconds:options.restartJitterMilliseconds,minimumUptimeMilliseconds:options.minimumUptimeMilliseconds,refused:(digest,reason)=>{replica.say({event:"restart-refused",reason:digest,message:reason});},failures:()=>replica.failures.summary(),store:work=>guard.call(work),git:signal=>(options.gitState??gitStateV1)(settings.worktree,signal),view:signal=>(options.gitView??gitViewV1)(settings.worktree,signal),viewMilliseconds:options.viewMilliseconds,viewDebounceMilliseconds:options.viewDebounceMilliseconds,reachable:()=>guard.reachable(),prepare:()=>ensureHome(replica)});
 replica.say({event:"started",originId:settings.originId,groupId:settings.groupId});
 await normaliseGitConfigV1(settings.worktree,undefined,signal);
 await attester.start(signal).catch(()=>{});
 if(relative(settings.worktree,running).startsWith("..")===false&&!relative(settings.worktree,running).startsWith(".cache")&&attester.running()!=="")void sealCodeV1({root:settings.worktree,moduleRoot:running,digest:attester.running(),digestOf:codeDigestV1,signal}).catch(()=>{});
 const watchFailed=(error:unknown):void=>{watcher?.close();watcher=undefined;noteOnce(replica,"watch:disabled",{event:"error",stage:"watch",message:`file watching stopped, only the scans remain: ${messageOf(error)}`});};
 try{
  watcher=watch(settings.worktree,{recursive:true,persistent:false},(kind,name)=>{
   if(name===null||name===undefined)return;
   const path=String(name).split(sep).join("/").normalize("NFC");
   if(path===".cache/f"||path.startsWith(".cache/f/")||(gitPrivateV1(path)&&path!==".git/index")||(path.split("/").includes(".git")&&!path.startsWith(".git/")&&path!==".git")||(!(path===".git"||path.startsWith(".git/"))&&replica.ignore.ignores(path,false)))return;
   if(path===".gitignore"||touched.size>5000){full=true;touched.clear();}
   else{touched.add(path);if(kind==="rename")touched.add(`${path}/`);}
   lastTouch=settings.now();wake?.();
  });
  watcher.on("error",watchFailed);
 }catch(error){watchFailed(error);}
 try{
  while(!signal.aborted){
   try{
    const enabled=await syncEnabledV1(settings.worktree);syncOn=enabled;
    if(!enabled){if(!suspended){suspended=true;replica.say({event:"sync",reason:"off"});}}
    else{
    if(suspended){suspended=false;full=true;replica.say({event:"sync",reason:"on"});}
    if(!await guard.reachable())down();
    else{
     if(outage){outage=false;replica.say({event:"store-reachable"});full=true;}
     const turn=await guard.turn(async signal=>{
     await options.storeGate?.();
     const retirement=await readRetirementV1(settings.storeRoot,settings.originId);
     if(retirement?.retired===true){if(!parked){parked=true;replica.say({event:"retired",originId:settings.originId,origin:retirement.by,reason:"retired",message:retirement.reason});}}
     else{
     if(parked){parked=false;full=true;replica.say({event:"sync",reason:"on"});}
     const now=settings.now();
     const retry:string[]=[];
     if(await repairRequestedV1(settings.worktree)){replica.say({event:"repaired",count:await repairV1(replica,signal)});full=true;}
     if(recovering){recovering=false;await followOnceV1(replica,signal);}
     if(full){full=false;walked=undefined;touched.clear();nextScan=settings.now()+scanDelayV1(settings.scanSeconds);retry.push(...(await publishOnceV1(replica,signal)).retry);}
     else{
      if(walked!==undefined){const scan=walked;walked=undefined;retry.push(...(await publishOnceV1(replica,signal,undefined,scan)).retry);}
      else if(walk===undefined&&now>=nextScan){
       nextScan=settings.now()+scanDelayV1(settings.scanSeconds);
       const digest=replica.ignoreHash;
       walk=walkLocalV1(replica,signal).then(scan=>{walk=undefined;if(replica.ignoreHash===digest)walked=scan;else full=true;settleHeap();},error=>{walk=undefined;if(!signal.aborted)noteOnce(replica,`scan:${messageOf(error)}`,{event:"error",message:messageOf(error)});});
      }
      if(touched.size>0&&settings.now()-lastTouch>=settings.debounceMilliseconds){
       const batch=[...touched];touched.clear();
       try{retry.push(...(await publishOnceV1(replica,signal,batch)).retry);}catch(error){for(const path of batch)touched.add(path);throw error;}
      }
     }
     if(retry.length>0){for(const path of retry)touched.add(path);lastTouch=settings.now();}
     await followOnceV1(replica,signal);
     }
     },signal);
     if(turn.status==="stalled")down();
    }
    }
    failed=0;
   }catch(error){
    if(signal.aborted)break;
    if(error instanceof OriginTakenV1)throw error;
    if(!await guard.reachable())down();
    else{failed++;noteOnce(replica,`round:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
   }
   if(!signal.aborted){
    try{if(await attester.tick({sync:syncOn&&!parked,files:replica.files.shared,manifest:replica.files.manifestHex(),cursors:replica.cursors,incoming:replica.queue.size,hlc:formatHlcV1(replica.clock,settings.originId)},signal)==="restart"){restart=true;break;}}
    catch(error){if(!signal.aborted)noteOnce(replica,`attest:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
    const wall=Date.now();
    if(wall-healthAt>=healthMilliseconds){healthAt=wall;await writeHealthV1(settings.worktree,{version:1,wallMs:wall,pid:process.pid,code:attester.running()===""?"0".repeat(64):attester.running(),store:guard.state(),storeSinceMs:guard.since(),pending:attester.pending(),failures:[...replica.failures.summary()]}).catch(()=>{});}
    if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}
   }
   if(++rounds%heapRounds===0)settleHeap();
   await pause(Math.max(touched.size>0?Math.min(settings.pollMilliseconds,settings.debounceMilliseconds):settings.pollMilliseconds,failed===0?0:Math.min(30_000,250*2**Math.min(failed,7))));
  }
 }finally{
  watcher?.close();
  await closeReplicaV1(replica);
  if(restart)replica.say({event:"restarting"});
  replica.say({event:"stopped"});
 }
}

'''
s=s[:a]+new_run+s[b:]
rep('import {basename,dirname,join,sep} from "node:path";','import {basename,dirname,join,relative,sep} from "node:path";')

# snapshot refusal
rep(''' requireOriginIdV1(originId);
 const folded=await foldStoreV1(storeRoot,signal,sink);
 if(folded.hlc===""||folded.manifest.size===0)return undefined;''',''' requireOriginIdV1(originId);
 let incomplete=false;
 const folded=await foldStoreV1(storeRoot,signal,event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);});
 if(incomplete){sink({version:1,event:"refused",stage:"snapshot",reason:"rejoin-required"});return undefined;}
 if(folded.hlc===""||folded.manifest.size===0)return undefined;''')

# status health
rep(" const settings=settle(options),paths=localPathsV1(settings.worktree),tables=tablesV1(),local=await loadLocal(paths,tables),reachable=await storeReachableV1(settings.storeRoot),held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree);"," const settings=settle(options),paths=localPathsV1(settings.worktree),tables=tablesV1(),local=await loadLocal(paths,tables),reachable=await storeReachableV1(settings.storeRoot),held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree),health=await readHealthV1(settings.worktree);")
rep("retired:own?.retired===true?{by:own.by,reason:own.reason,wallMs:own.wallMs}:null};","retired:own?.retired===true?{by:own.by,reason:own.reason,wallMs:own.wallMs}:null,health:health===undefined||!(held!==undefined&&!held.stale)?null:{wallMs:health.wallMs,store:health.store,storeSinceMs:health.storeSinceMs,code:health.code,pending:health.pending,failures:health.failures}};")

# relayBlob verification
rep(''' const destination=blobPathV1(ownStore,sha256),own=storePathsV1(ownStore);
 if(await stat(destination).then(()=>true,()=>false))return false;''',''' const destination=blobPathV1(ownStore,sha256),own=storePathsV1(ownStore);
 if(await stat(destination).then(()=>true,()=>false)){
  if(verifiedBlobs.has(destination)||(await hashFileV1(destination).catch(()=>undefined))?.sha256===sha256){if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();verifiedBlobs.add(destination);return false;}
  await rm(destination,{force:true});sink({version:1,event:"blob-missing",origin,reason:`blob-corrupt:${sha256}`});
 }''')
rep('const homes=new Map<string,string>();','const homes=new Map<string,string>(),verifiedBlobs=new Set<string>();')
open(p,"w",encoding="utf8").write(s)
print("part3 ok")
