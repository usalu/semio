import sys,glob
root=sys.argv[1]
p=glob.glob(root+"/*/*/*/*/*/*replica/\U0001F7E6️.ts")[0]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:90],s.count(old))
    s=s.replace(old,new)

# mint
rep('''function mint(replica:ReplicaV1):string{
 replica.clock=tickClockV1(replica.clock,replica.settings.now());replica.dirty=true;''','''function mint(replica:ReplicaV1):string{
 const wall=replica.settings.now();
 if(replica.clock.ms>wall+futureMilliseconds)throw new ClockAheadV1(Math.ceil((replica.clock.ms-wall)/1000));
 replica.clock=tickClockV1(replica.clock,wall);replica.dirty=true;''')

# ensureHome with key
a=s.index("async function ensureHome(replica:ReplicaV1):Promise<void>{")
b=s.index("type Job=Readonly<")
new='''async function ensureHome(replica:ReplicaV1):Promise<void>{
 if(replica.homeWritten)return;
 const {origins,tmp}=storePathsV1(replica.settings.storeRoot),directory=join(origins,replica.settings.originId),file=join(directory,"home.json");
 for(let attempt=0;attempt<3;attempt++){
  const existing=await readHome(replica.settings.storeRoot,replica.settings.originId);
  if(existing?.key!==undefined&&existing.key!==replica.key)throw new OriginTakenV1(replica.settings.originId);
  if(existing?.key===replica.key){replica.homeWritten=true;return;}
  await ensureStoreDirectoryV1(replica.settings.storeRoot,directory);await ensureStoreDirectoryV1(replica.settings.storeRoot,tmp);
  const temporary=join(tmp,`${replica.settings.originId}-${process.pid}-${token()}`);
  await writeFile(temporary,JSON.stringify({version:1,group:replica.settings.groupId,key:replica.key}));
  try{
   if(existing===undefined)await link(temporary,file);else await rename(temporary,file);
   replica.homeWritten=true;return;
  }catch(error){if(code(error)!=="EEXIST")throw error;}
  finally{await rm(temporary,{force:true});}
 }
 throw new OriginTakenV1(replica.settings.originId);
}

'''
s=s[:a]+new+s[b:]

# noteOnce
rep('''function noteOnce(replica:ReplicaV1,key:string,event:Omit<ReplicaEventV1,"version">):void{
 if(replica.noted.has(key))return;
 replica.noted.add(key);replica.say(event);
}''','''function noteOnce(replica:ReplicaV1,key:string,event:Omit<ReplicaEventV1,"version">):void{
 const {announce,count}=replica.failures.note(key,event.message??event.reason??key);
 if(replica.noted.size>=noteCapacity)replica.noted.clear();
 replica.noted.add(key);
 if(announce)replica.say(count>1?{...event,count}:event);
}

function resolved(replica:ReplicaV1,key:string):void{replica.noted.delete(key);replica.failures.clear(key);}''')
rep(' replica.noted.delete(`blocked:${path}`);\n if(result.state==="linear")',' resolved(replica,`blocked:${path}`);\n if(result.state==="linear")')
rep('replica.noted.delete(`blocked:${path}`);dequeue(replica,path);}','resolved(replica,`blocked:${path}`);dequeue(replica,path);}')
rep('  replica.noted.delete(`rejoin:${origin}`);','  resolved(replica,`rejoin:${origin}`);')
rep('replica.held.delete(origin);replica.noted.delete(`skew:${origin}`);}','replica.held.delete(origin);resolved(replica,`skew:${origin}`);}')
rep('noteOnce(replica,`unstable:${entry.path}:${entry.mtimeMs}`,','noteOnce(replica,`unstable:${entry.path}`,',2) if False else None
s=s.replace('noteOnce(replica,`unstable:${entry.path}:${entry.mtimeMs}`,','noteOnce(replica,`unstable:${entry.path}`,')

# repair hook for journal appends
rep('''const indexHosts=new WeakMap<ReplicaV1,IndexHostV1>();''','''const journalRepair=(replica:ReplicaV1)=>(repair:Readonly<{segment:number;bytes:number;found:number}>):void=>{replica.say({event:"journal-repaired",count:repair.segment,bytes:repair.bytes,done:repair.found,reason:"segment-shorter-than-head"});};

const indexHosts=new WeakMap<ReplicaV1,IndexHostV1>();''')
rep('const head=await appendEventsV1(settings.storeRoot,settings.originId,stamped);','const head=await appendEventsV1(settings.storeRoot,settings.originId,stamped,journalRepair(replica));')
rep('const head=await appendEventsV1(storeRoot,originId,events);\n    replica.cursors[originId]','const head=await appendEventsV1(storeRoot,originId,events,journalRepair(replica));\n    replica.cursors[originId]')
rep('try{await ensureHome(replica);head=await appendEventsV1(storeRoot,originId,events);}','try{await ensureHome(replica);head=await appendEventsV1(storeRoot,originId,events,journalRepair(replica));}')

# applyQueue wrapper
rep('async function applyQueue(replica:ReplicaV1,signal:AbortSignal|undefined,stage?:string):Promise<Applied>{\n','async function applyQueue(replica:ReplicaV1,signal:AbortSignal|undefined,stage?:string):Promise<Applied>{\n replica.applying++;\n try{return await applyQueueWork(replica,signal,stage);}finally{replica.applying--;}\n}\n\nasync function applyQueueWork(replica:ReplicaV1,signal:AbortSignal|undefined,stage?:string):Promise<Applied>{\n')

# unsupported event classification
rep('    }else noteOnce(replica,`error:${origin}:${messageOf(error)}`,{event:"error",origin,message:messageOf(error)});\n    break;','    }else noteOnce(replica,`${messageOf(error).includes("unsupported event version")?"unsupported":"error"}:${origin}:${messageOf(error)}`,{event:"error",origin,message:messageOf(error)});\n    break;')

rep('const homes=new Map<string,string>();','''/** 🔬️ Loads the current local state of a worktree (checkpoint, log, cursors, clock, machine) into throw-away tables without taking the lock and without writing anything; `null` when this code can read it, else the reason. The self-check uses it so that a daemon never restarts into code that cannot read the state of its machine. */
export async function dryOpenStateV1(worktree:string):Promise<string|null>{
 try{await loadLocal(localPathsV1(worktree),tablesV1());return null;}catch(error){return messageOf(error);}
}

const homes=new Map<string,string>();''')
open(p,"w",encoding="utf8").write(s)
print("part2 ok")
