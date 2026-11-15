import sys,glob
root=sys.argv[1]
p=glob.glob(root+"/*/*/*/*/*/*replica/\U0001F7E6️.ts")[0]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:90],s.count(old))
    s=s.replace(old,new)

rep('/** 🏃️ The long-running daemon.','''const defaultLogFiles=(worktree:string):readonly string[]=>[...process.platform==="darwin"?["/var/log/semio-fleet-replica.log"]:[],join(worktree,".cache","f","native-bootstrap.log")];

async function maintain(replica:ReplicaV1,options:ReplicaOptionsV1):Promise<void>{
 const {settings,paths}=replica,now=Date.now();
 await pruneLostV1(paths.lost,{now,retainMs:settings.retentionDays*86_400_000,maxEntries:lostEntries}).catch(()=>0);
 for(const file of options.logFiles??defaultLogFiles(settings.worktree))await rotateLogV1(file,{maxBytes:logLimitFileBytes,keepBytes:logKeepBytes}).catch(()=>false);
 replica.failures.forgetOlderThan(3_600_000);
 const cool=cooling.get(replica);
 if(cool!==undefined)for(const path of cool.keys()){const id=replica.book.find(path);if(id===undefined||!replica.queue.hasId(id))cool.delete(path);}
}

/** 🏃️ The long-running daemon.''')
d=s.index("/** 🏃️ The long-running daemon.")
e=s.index("*/",d)
s=s[:e]+" Every store turn runs under a deadline guard: a store call that a hard mount blocks cannot be cancelled, so when the store has not answered a probe within `storeDeadlineMilliseconds` (10 s) the round is abandoned and aborted, `store-unreachable` is reported (reason `stalled`), no further store call is issued while one is outstanding, the loop, the hint channel, the attestation to memory and the state persistence keep running, and the daemon resumes by itself when the store answers again. New code is adopted only after a fresh process proved that it starts (`restart-refused` otherwise), after a deterministic per-origin delay of 0 to 60 s and while the incoming queue is empty. Recurring failures are counted per kind, announced at most once a minute and shown in `status`, the health file and the attestation. "+s[e:]

rep('hinting=false,settledAt=-1;','hinting=false,settledAt=-1,failed=0,healthAt=-Infinity,maintainedAt=-Infinity;')
rep(' const switchOn=cached(slow,()=>syncEnabledV1(settings.worktree)),reachable=cached(slow,()=>storeReachableV1(settings.storeRoot)),',' const guard=createStoreGuardV1({probe:()=>(options.storeProbe??storeReachableV1)(settings.storeRoot),deadlineMs:options.storeDeadlineMilliseconds??10_000,now:Date.now,changed:()=>{}});\n const switchOn=cached(slow,()=>syncEnabledV1(settings.worktree)),reachable=cached(slow,()=>guard.reachable()),')
rep('replica.say({event:"store-unreachable",path:settings.storeRoot});}};','replica.say({event:"store-unreachable",path:settings.storeRoot,reason:guard.state()});}};')
rep(' const attester=createAttesterV1({',''' const running=options.codeRoot??codeRootV1,watching=options.watchRoot??running;
 const checkCode=async(digest:string,aborted?:AbortSignal):Promise<RestartCheckV1>=>{
  const result=await runSelfCheckV1({command:process.execPath,args:[join(watching,"🔁️replica","📜️script.ts"),"self-check"],cwd:settings.worktree,timeoutMs:options.selfCheckMilliseconds??60_000,signal:aborted});
  if(!result.ok)return {ok:false,reason:result.reason};
  if(result.digest!==digest)return {ok:false,reason:"code changed during the check",again:true};
  const sealed=await sealCodeV1({root:settings.worktree,moduleRoot:watching,digest,digestOf:codeDigestV1,signal:aborted});
  if(sealed.reason==="code-changed-while-copying")return {ok:false,reason:sealed.reason,again:true};
  if(sealed.reason!==null)noteOnce(replica,`seal:${sealed.reason}`,{event:"error",stage:"seal",message:sealed.reason});
  return {ok:true};
 };
 const attester=createAttesterV1({''')
rep('codeRoot:options.codeRoot,git:signal=>','codeRoot:options.codeRoot,watchRoot:options.watchRoot,check:options.selfCheck??checkCode,busy:()=>replica.queue.size>0||replica.applying>0,jitterSpanMilliseconds:options.restartJitterMilliseconds,minimumUptimeMilliseconds:options.minimumUptimeMilliseconds,refused:(digest,reason)=>{replica.say({event:"restart-refused",reason:digest,message:reason});},failures:()=>replica.failures.summary(),store:work=>guard.call(work),git:signal=>')
rep(' const booted=attester.start(signal).catch(()=>{});\n',' const booted=attester.start(signal).catch(()=>{});\n void booted.then(()=>{if(!relative(settings.worktree,running).startsWith("..")&&!relative(settings.worktree,running).startsWith(".cache")&&attester.running()!=="")void sealCodeV1({root:settings.worktree,moduleRoot:running,digest:attester.running(),digestOf:codeDigestV1,signal}).catch(()=>{});});\n')
rep('  watcher?.on("error",()=>{watcher?.close();watcher=undefined;});\n }catch{watcher=undefined;}','  watcher?.on("error",watchFailed);\n }catch(error){watchFailed(error);}')
rep(' try{\n  if(options.watch!==false)watcher=watch(',' const watchFailed=(error:unknown):void=>{watcher?.close();watcher=undefined;noteOnce(replica,"watch:disabled",{event:"error",stage:"watch",message:`file watching stopped, only the scans remain: ${messageOf(error)}`});};\n try{\n  if(options.watch!==false)watcher=watch(')
rep('''     if(outage){outage=false;replica.say({event:"store-reachable"});full=true;}
     const stopped=await retirement.get();''','''     if(outage){outage=false;replica.say({event:"store-reachable"});full=true;}
     const turn=await guard.turn(async signal=>{
     await options.storeGate?.();
     const stopped=await retirement.get();''')
rep('''     await followOnceV1(replica,signal,true);
     }
    }
    }
   }catch(error){
    if(signal.aborted)break;
    reachable.expire();
    if(!await reachable.get())down();
    else replica.say({event:"error",message:messageOf(error)});
   }''','''     await followOnceV1(replica,signal,true);
     }
     },signal);
     if(turn.status==="stalled")down();
    }
    }
    failed=0;
   }catch(error){
    if(signal.aborted)break;
    if(error instanceof OriginTakenV1)throw error;
    reachable.expire();
    if(!await reachable.get())down();
    else{failed++;noteOnce(replica,`round:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
   }''')
rep('nextWalk=walkNext(spent());if(!signal.aborted)replica.say({event:"error",message:messageOf(error)});});','nextWalk=walkNext(spent());if(!signal.aborted)noteOnce(replica,`scan:${messageOf(error)}`,{event:"error",message:messageOf(error)});});')
rep('''    catch(error){if(!signal.aborted)noteOnce(replica,`attest:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
   }
''','''    catch(error){if(!signal.aborted)noteOnce(replica,`attest:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
    const wall=Date.now();
    if(wall-healthAt>=(options.attestMilliseconds??healthMilliseconds)){healthAt=wall;await writeHealthV1(settings.worktree,{version:1,wallMs:wall,pid:process.pid,code:attester.running()===""?"0".repeat(64):attester.running(),store:guard.state(),storeSinceMs:guard.since(),pending:attester.pending(),failures:[...replica.failures.summary()]}).catch(()=>{});}
    if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}
   }
''')
rep('   await pause(hot.active||sweep.active||file.active?sliceGapMilliseconds:touched.size>0?Math.min(settings.pollMilliseconds,settings.debounceMilliseconds):settings.pollMilliseconds);','   await pause(Math.max(hot.active||sweep.active||file.active?sliceGapMilliseconds:touched.size>0?Math.min(settings.pollMilliseconds,settings.debounceMilliseconds):settings.pollMilliseconds,failed===0?0:Math.min(30_000,250*2**Math.min(failed,7))));')
rep('import {basename,dirname,join,sep} from "node:path";','import {basename,dirname,join,relative,sep} from "node:path";')
# snapshot refusal
rep(''' requireOriginIdV1(originId);
 const folded=await foldStoreV1(storeRoot,signal,sink);
 if(folded.hlc===""||folded.manifest.size===0)return undefined;''',''' requireOriginIdV1(originId);
 let incomplete=false;
 const folded=await foldStoreV1(storeRoot,signal,event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);});
 if(incomplete){sink({version:1,event:"refused",stage:"snapshot",reason:"rejoin-required"});return undefined;}
 if(folded.hlc===""||folded.manifest.size===0)return undefined;''')
# status
rep("reachable=await storeReachableV1(settings.storeRoot),held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree);","reachable=await storeReachableV1(settings.storeRoot),held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree),beat=await readHealthV1(settings.worktree);")
rep("retired:own?.retired===true?{by:own.by,reason:own.reason,wallMs:own.wallMs}:null};","retired:own?.retired===true?{by:own.by,reason:own.reason,wallMs:own.wallMs}:null,health:beat===undefined||!(held!==undefined&&!held.stale)?null:{wallMs:beat.wallMs,store:beat.store,storeSinceMs:beat.storeSinceMs,code:beat.code,pending:beat.pending,failures:beat.failures}};")
# relay blob
rep(''' const destination=blobPathV1(ownStore,sha256),own=storePathsV1(ownStore);
 if(await stat(destination).then(()=>true,()=>false))return false;''',''' const destination=blobPathV1(ownStore,sha256),own=storePathsV1(ownStore);
 if(await stat(destination).then(()=>true,()=>false)){
  if(verifiedBlobs.has(destination)||(await hashFileV1(destination).catch(()=>undefined))?.sha256===sha256){if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();verifiedBlobs.add(destination);return false;}
  sink({version:1,event:"blob-missing",origin,reason:`blob-corrupt:${sha256}`});
 }''')
rep('const homes=new Map<string,string>();','const homes=new Map<string,string>(),verifiedBlobs=new Set<string>();')
open(p,"w",encoding="utf8").write(s)
print("part3 ok")
