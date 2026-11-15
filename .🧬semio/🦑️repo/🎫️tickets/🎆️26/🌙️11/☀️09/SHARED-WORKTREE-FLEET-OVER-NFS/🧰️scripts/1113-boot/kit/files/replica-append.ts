
/** ✍️ Tells whether every file of the code on disk that differs from the code that runs was last written by this machine: the files of the module and of everything it imports are compared with the sealed copy of the running code (`.cache/f/code/<running>/sealed.json`), and each one that differs, is new or is gone must have been published by this origin (the holder of its register in the local state). Code that arrived from another origin, even in part, is therefore never judged authored here, and a running code that was never sealed cannot be compared, so nothing is authored. */
export async function authoredCodeV1(replica:ReplicaV1,moduleRoot:string,running:string):Promise<boolean>{
 const root=replica.settings.worktree;
 let seal:SealedCodeV1;
 try{seal=JSON.parse(await readFile(join(codeDirectoryV1(root),running,"sealed.json"),"utf8")) as SealedCodeV1;}catch{return false;}
 const before=new Map(seal.files.map(file=>[file.path,file.sha256])),sources=await moduleSourcesV1(moduleRoot),paths=[...sources,...await importClosureV1(moduleRoot,sources)].map(path=>relative(root,resolve(moduleRoot,...path.split("/"))).split(sep).join("/")),now=new Set(paths);
 const mine=(path:string,deleted:boolean):boolean=>{const id=replica.book.find(path);return id!==undefined&&replica.files.live(id)&&(replica.files.operationAt(id)==="delete")===deleted&&replica.book.origins[replica.files.origin[id]!]===replica.settings.originId;};
 let changed=0;
 for(const path of paths){
  const data=await readFile(join(root,...path.split("/"))).catch(()=>undefined);
  if(data===undefined)return false;
  if(createHash("sha256").update(data).digest("hex")===before.get(path))continue;
  changed++;
  if(!mine(path,false))return false;
 }
 for(const path of before.keys())if(!now.has(path)){changed++;if(!mine(path,true))return false;}
 return changed>0;
}

/** 🚪️ The admission of new code on disk, asked before its self-check: a daemon switches only to code that some origin attests to run (any other origin whose attestation carries the digest, or whose healthy run left a code snapshot of it in the store) or that was authored on this machine (`authoredCodeV1`). Anything else is a change set that arrived from elsewhere and that nobody has proven: a mix of new and stale files, or a change whose rest is still on its way. It waits, is asked again every heartbeat, and the digest changes as more files arrive, so the decision is made again for every state of the module. `bounded` runs one store read under a deadline. */
export async function admitCodeV1(replica:ReplicaV1,input:Readonly<{digest:string;running:string;moduleRoot:string;bounded:<T>(work:()=>Promise<T>)=>Promise<T>;signal?:AbortSignal}>):Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>{
 const {storeRoot,originId}=replica.settings;
 if(input.digest===input.running)return {ok:true};
 const attestations=await input.bounded(()=>readAttestationsV1(storeRoot,input.signal));
 if(Object.values(attestations).some(item=>item.originId!==originId&&item.code===input.digest))return {ok:true};
 if((await input.bounded(()=>listStoreCodeV1(storeRoot))).some(item=>item.digest===input.digest))return {ok:true};
 if(await authoredCodeV1(replica,input.moduleRoot,input.running))return {ok:true};
 return {ok:false,reason:"no origin runs this code yet and it was not all written on this machine"};
}

/** 🔬️ Opens the state of this machine the way the daemon does, on a private mirror: the checkpoint (linked, never written), the log, the cursors, the clock and the identity are mirrored below a temporary directory and `openReplicaV1` runs on it without the lock, so everything the open does (loading every table, replaying the own tail, requeueing) happens with the checked code while the real state and the store stay untouched. `null` when it opens, else the reason. A machine without an identity has no state to open, and only the load is tried. */
export async function dryOpenReplicaV1(worktree:string):Promise<string|null>{
 const paths=localPathsV1(worktree),machine=await readMachine(paths.machine) as Machine|undefined;
 if(machine===undefined)return dryOpenStateV1(worktree);
 const mirror=await mkdtemp(join(tmpdir(),"fleet-dry-open-")),target=localPathsV1(mirror);
 try{
  await mkdir(target.directory,{recursive:true});
  for(const name of ["checkpoint","wal","takeoverState","takeoverLog","cursor","clock","machine"] as const){
   if(await stat(paths[name]).then(()=>false,()=>true))continue;
   if(name==="checkpoint"||name==="takeoverState")await symlink(paths[name],target[name]).catch(()=>copyFile(paths[name],target[name]));
   else await copyFile(paths[name],target[name]);
  }
  await openReplicaV1({worktree:mirror,storeRoot:machine.storeRoot,originId:machine.originId,groupId:machine.groupId,lock:false});
  return null;
 }catch(error){return messageOf(error);}
 finally{await rm(mirror,{recursive:true,force:true}).catch(()=>{});}
}

/** 💨️ Boots the daemon of the checked code once in a throw-away worktree and store: seeds three files, opens and closes the state (which writes a checkpoint), then runs `runV1` on it until its first attestation reaches the scratch store, so that a daemon that cannot start, cannot load its own checkpoint or cannot attest is found before any real machine restarts into it. `null` when it booted, else the reason. */
export async function smokeBootV1(signal?:AbortSignal,milliseconds:number=60_000):Promise<string|null>{
 const base=await mkdtemp(join(tmpdir(),"fleet-smoke-")),worktree=join(base,"w"),store=join(base,"s");
 try{
  await mkdir(worktree,{recursive:true});await writeFile(join(worktree,"smoke.txt"),"smoke\n");
  const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"smoke",groupId:"g0",pollMilliseconds:20,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,watch:false,selfCheck:async()=>({ok:true}),gitState:async()=>({present:false,head:null,commit:null,indexTree:null}),gitView:async()=>null,hintMilliseconds:20,logFiles:[],restartStableMilliseconds:3_600_000};
  await seedV1(options);
  await closeReplicaV1(await openReplicaV1(options));
  const controller=new AbortController(),events:ReplicaEventV1[]=[],abort=():void=>{controller.abort();};
  signal?.addEventListener("abort",abort,{once:true});
  let failure:string|null=null;
  const done=runV1(options,controller.signal,event=>{events.push(event);}).catch(error=>{failure=messageOf(error);});
  const end=Date.now()+milliseconds;
  while(failure===null&&!controller.signal.aborted&&await readAttestationV1(store,"smoke").then(found=>found===undefined,()=>true)){
   const bad=events.find(event=>event.event==="error");
   if(bad!==undefined){failure=bad.message??"error event";break;}
   if(Date.now()>end){failure=`the daemon wrote no attestation within ${milliseconds} ms`;break;}
   await new Promise<void>(next=>setTimeout(next,25));
  }
  controller.abort();signal?.removeEventListener("abort",abort);
  await done;
  return failure;
 }catch(error){return messageOf(error);}
 finally{await rm(base,{recursive:true,force:true}).catch(()=>{});}
}
