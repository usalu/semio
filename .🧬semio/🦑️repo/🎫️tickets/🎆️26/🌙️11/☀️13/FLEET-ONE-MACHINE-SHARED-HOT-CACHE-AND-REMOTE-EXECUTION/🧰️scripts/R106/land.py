import os,sys,shutil,json
ROOT="/Users/ueli/Documents/semio"
LIB=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B=LIB+"/🏗️build"
GS=B+"/🪪️graph-state"
N=B+"/🕸️graph/📥️inference/📦️project-nodes"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R106"
BK=SC+"/backup"
HERE=os.path.dirname(os.path.abspath(__file__))
CHECK="--check" in sys.argv
edits=[]
def rep(path,old,new):
    edits.append((path,old,new))

EX=GS+"/🛰️exchange/🟦️.ts"
rep(EX,"""function holdOf(client:GraphFleetClientV1,key:string,owner:string""","""/** ⏭️ The answer of a decision that leaves an owner to a live peer: this reader has other owners to infer, so it goes on with them and comes back to this one later. */
export type GraphSkipV1=Readonly<{outcome:"skip";by:string;reason:"busy";waitedMs:number}>;

const firstHeld=new Map<string,number>(),waiting=new Map<string,Readonly<{kind:string;digest:string;by:string;sinceMs:number}>>();

/** ⏳️ The owners this process waits for right now: the kind and digest of the entry, the machine that holds its claim and since when it waits; what a reader that makes no progress names as its reason. */
export function graphFleetWaitingV1():readonly Readonly<{kind:string;digest:string;by:string;sinceMs:number}>[]{return[...waiting.values()];}

/** ⚖️ What a reader does about an owner whose claim another machine holds: `lapsed` (the lease ended: claim it again, nobody is working on it), `compute` (a large machine never waits for a smaller one, and an owner held for the lease time by a holder that keeps renewing is taken over: the oldest-held first), `skip` (a reader with other owners to infer goes on with them) or `wait` (nothing else to infer, the lease is live and younger than the lease time).
 * @see ../🧪️tests/⏳️lease-wait/🟦️.ts */
export type GraphHeldVerdictV1="lapsed"|"compute"|"skip"|"wait";
export function graphHeldVerdictV1(input:Readonly<{untilMilliseconds:number;now:number;sinceMilliseconds:number;busy:boolean;strong:boolean}>):GraphHeldVerdictV1{
 if(!(input.untilMilliseconds>input.now))return "lapsed";
 if(input.strong||input.now-input.sinceMilliseconds>=policy.leaseTakeOverMilliseconds)return "compute";
 return input.busy?"skip":"wait";
}

function holdOf(client:GraphFleetClientV1,key:string,owner:string""")

rep(EX,"""/** 🧭️ The protocol for one state: look it up; on a miss claim it; when another machine holds the claim wait for its upload (bounded) and fetch it; when the holder lapses take over. Anything unreachable, denied or too slow ends in `compute` without a hold, because correctness never depends on the fleet. */
export async function decideGraphEntryV1(input:Readonly<{client:GraphFleetClientV1;state:GraphStateV1;owner:string;signal?:AbortSignal;say?:GraphSayV1;expectedMilliseconds?:number;waitLimitMilliseconds?:number;skip?:GraphHeldV1;defer?:(name:string)=>boolean;now?:()=>number}>):Promise<GraphDecisionV1>{""","""/** 🧭️ What {@link decideGraphEntryV1} is given: `busy` says whether the caller has other owners to infer (then an owner held by a live peer is skipped, never waited for), `strong` that the caller is a large machine (it never waits for a smaller one). */
export type GraphDecideInputV1=Readonly<{client:GraphFleetClientV1;state:GraphStateV1;owner:string;signal?:AbortSignal;say?:GraphSayV1;expectedMilliseconds?:number;waitLimitMilliseconds?:number;skip?:GraphHeldV1;defer?:(name:string)=>boolean;now?:()=>number;busy?:()=>boolean;strong?:boolean}>;

/** 🧭️ The protocol for one state: look it up; on a miss claim it; when another machine holds a live claim and this reader has nothing else to infer wait for its upload (at most the lease time since the owner was first seen held) and fetch it; when the holder lapses, or the lease time has passed, take over; a busy reader skips the owner and a large machine computes it at once. Anything unreachable, denied or too slow ends in `compute` without a hold, because correctness never depends on the fleet.
 * @see ../🧪️tests/⏳️lease-wait/🟦️.ts */
export async function decideGraphEntryV1(input:GraphDecideInputV1&Readonly<{busy:()=>boolean}>):Promise<GraphDecisionV1|GraphSkipV1>;
export async function decideGraphEntryV1(input:GraphDecideInputV1):Promise<GraphDecisionV1>;
export async function decideGraphEntryV1(input:GraphDecideInputV1):Promise<GraphDecisionV1|GraphSkipV1>{""")

rep(EX,"""compute=(reason:string,hold?:GraphHoldV1):GraphDecisionV1=>({outcome:"compute",hold,reason,waitedMs:now()-started})""","""compute=(reason:string,hold?:GraphHoldV1):GraphDecisionV1=>{firstHeld.delete(key);return{outcome:"compute",hold,reason,waitedMs:now()-started};}""")
rep(EX,"""const hit=(found:{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>},via:"cache"|"wait"):GraphDecisionV1=>{say?.("hit",""","""const hit=(found:{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>},via:"cache"|"wait"):GraphDecisionV1=>{firstHeld.delete(key);say?.("hit",""")
rep(EX,"""   if(deadline===Infinity){deadline=now()+Math.min(policy.claimWaitMaximumMilliseconds,input.waitLimitMilliseconds??policy.claimWaitMaximumMilliseconds);say?.("waited",{kind:state.frame.kind,digest:label,seconds:0});}""","""   const at=now(),since=firstHeld.get(key)??at;firstHeld.set(key,since);
   const verdict=graphHeldVerdictV1({untilMilliseconds:claim.untilMilliseconds,now:at,sinceMilliseconds:since,busy:input.busy?.()===true,strong:input.strong===true});
   if(verdict==="skip"){await sleep(100,input.signal);return{outcome:"skip",by:claim.by,reason:"busy",waitedMs:now()-started};}
   if(verdict==="compute")return compute(input.strong===true?"strong":"holder-slow");
   if(verdict==="lapsed"){await sleep(250,input.signal);continue;}
   waiting.set(key,{kind:state.frame.kind,digest:label,by:claim.by,sinceMs:since});
   if(deadline===Infinity){deadline=since+Math.min(policy.claimWaitMaximumMilliseconds,input.waitLimitMilliseconds??policy.claimWaitMaximumMilliseconds,policy.leaseTakeOverMilliseconds);say?.("waited",{kind:state.frame.kind,digest:label,seconds:0});}""")
rep(EX,"""  return compute("rounds");
 }catch{return compute("error");}
}""","""  return compute("rounds");
 }catch{return compute("error");}finally{waiting.delete(key);}
}""")

rep(GS+"/🧬️schema/🔣️.json",""""claimWaitMaximumMilliseconds": 900000,""",""""claimWaitMaximumMilliseconds": 900000,
        "leaseTakeOverMilliseconds": 60000,""")

PR=GS+"/📦️project-records/🟦️.ts"
rep(PR,"""export type ProjectGateV1=Readonly<{seeded:boolean;publish():void;abandon():Promise<void>}>;""","""export type ProjectGateV1=Readonly<{seeded:boolean;deferred?:boolean;publish():void;abandon():Promise<void>}>;""")
rep(PR,"""export function createProjectFleetPortV1(input:Readonly<{root:string;environment:Environment;client?:GraphFleetClientV1;""","""export function createProjectFleetPortV1(input:Readonly<{root:string;environment:Environment;busy?():boolean;strong?:boolean;client?:GraphFleetClientV1;""")
rep(PR,"""   const decide=()=>decideGraphEntryV1({client,state,owner,skip:heldOf(info,held),...info.closures===undefined?{}:{defer:isClosureDocument}});
   let decision=await decide();""","""   const decide=():Promise<Awaited<ReturnType<typeof decideGraphEntryV1>>|Readonly<{outcome:"skip"}>>=>decideGraphEntryV1({client,state,owner,skip:heldOf(info,held),...input.busy===undefined?{}:{busy:input.busy},...input.strong===undefined?{}:{strong:input.strong},...info.closures===undefined?{}:{defer:isClosureDocument}});
   let decision=await decide();
   if(decision.outcome==="skip")return{seeded:false,deferred:true,...nothing};""")

OB=B+"/👁️observation/🟨️.mjs"
rep(OB,"""gate=await lookupLapAsyncV1("gate",()=>port.gate(info)).catch(()=>undefined);if(gate?.seeded)reused=await lookup();}""","""gate=await lookupLapAsyncV1("gate",()=>port.gate(info)).catch(()=>undefined);if(gate?.deferred)throw Object.assign(Error("owner left to a live peer: this reader has other owners to infer / Owner einem aktiven Peer überlassen: dieser Leser hat andere Owner abzuleiten"),{code:"BUILD_OWNER_DEFERRED"});if(gate?.seeded)reused=await lookup();}""")

TK=N+"/📥️take/🟨️.mjs"
rep(TK,"""/** 📒️ What a reader knows about its owners while it resolves them""","""/** 💪️ Whether a machine is large enough never to wait for another: its memory and its cores both reach the declared floor (`strongMemoryMegabytes`, `strongCores`), so the fleet never makes the biggest machine wait for the smallest ones.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function takeStrongMachineV1({ memoryMegabytes, cores }, policy = TAKE_POLICY_V1) {
  return memoryMegabytes >= policy.strongMemoryMegabytes && cores >= policy.strongCores;
}

/** 🚦️ A watch over the progress of a read: `check(sign)` is told a value that changes whenever anything moves (owners done, looks, rows), and answers how long nothing moved once that reached `windowMilliseconds`, else undefined.
 * @see ./🧪️tests/🚦️no-progress/🟦️.ts */
export function createProgressWatchV1({ now = () => performance.now(), windowMilliseconds = TAKE_POLICY_V1.noProgressMilliseconds } = {}) {
  let last, since = now();
  return Object.freeze({
    check(sign) {
      const at = now();
      if (sign !== last) { last = sign; since = at; return undefined; }
      return at - since >= windowMilliseconds ? Object.freeze({ sinceMilliseconds: at - since }) : undefined;
    },
  });
}

/** 🗣️ The plain text of a read that stopped for lack of progress, in one language: how long nothing moved, how many owners are done, which run and what the read waits on. */
export function takeNoProgressTextV1(language, fields) { return fill(TEXTS[language === "de" ? "de" : "en"].noProgress, fields); }

/** 📒️ What a reader knows about its owners while it resolves them""")

TS=N+"/📥️take/🧬️schema/🔣️.json"
rep(TS,""""progressMilliseconds", "passBoundMilliseconds", "releaseMilliseconds"],""",""""progressMilliseconds", "passBoundMilliseconds", "releaseMilliseconds", "noProgressMilliseconds", "watchMilliseconds", "strongMemoryMegabytes", "strongCores"],""")
rep(TS,""""releaseMilliseconds": { "type": "integer", "minimum": 1 }
      },""",""""releaseMilliseconds": { "type": "integer", "minimum": 1 },
        "noProgressMilliseconds": { "type": "integer", "minimum": 1 },
        "watchMilliseconds": { "type": "integer", "minimum": 1 },
        "strongMemoryMegabytes": { "type": "integer", "minimum": 1 },
        "strongCores": { "type": "integer", "minimum": 1 }
      },""")
rep(TS,""""passBoundMilliseconds": 900000, "releaseMilliseconds": 30000 }""",""""passBoundMilliseconds": 900000, "releaseMilliseconds": 30000, "noProgressMilliseconds": 300000, "watchMilliseconds": 10000, "strongMemoryMegabytes": 24576, "strongCores": 8 }""")
rep(TS,"""wanted from the fleet, {stale} stale in the fleet", "wave":""","""wanted from the fleet, {stale} stale in the fleet", "noProgress": "no owner made progress for {minutes} minutes ({done} of {total} owners done). Running: {running}. Waiting on: {waiting}. The read stops instead of blocking every other read of this machine; run the command again when memory or the fleet is free", "wave":""")
rep(TS,"""{stale} in der Flotte veraltet", "wave":""","""{stale} in der Flotte veraltet", "noProgress": "kein Projekt kam {minutes} Minuten voran ({done} von {total} Projekten fertig). Es laufen: {running}. Es wartet auf: {waiting}. Das Lesen bricht ab, statt jedes andere Lesen dieses Rechners zu blockieren; führe den Befehl erneut aus, sobald Speicher oder Flotte frei sind", "wave":""")

TD=N+"/📥️take/🟨️.d.mts"
rep(TD,"""/** 🔢 The looks a lane runs at once under the cut of the process. */""","""/** 💪️ Whether a machine is large enough never to wait for another. */
export function takeStrongMachineV1(input: Readonly<{ memoryMegabytes: number; cores: number }>, policy?: TakePolicyV1): boolean;
/** 🚦️ A watch over the progress of a read. */
export function createProgressWatchV1(input?: Readonly<{ now?(): number; windowMilliseconds?: number }>): Readonly<{ check(sign: unknown): Readonly<{ sinceMilliseconds: number }> | undefined }>;
/** 🗣️ The plain text of a read that stopped for lack of progress. */
export function takeNoProgressTextV1(language: string, fields: Readonly<Record<string, string | number>>): string;
/** 🔢 The looks a lane runs at once under the cut of the process. */""")

TX=B+"/📖️texts/🔣️.json"
rep(TX,"""    "graph.owner-too-heavy": {""","""    "graph.no-progress": {
      "en": "The project graph read stopped: no owner made progress for {minutes} minutes ({done} of {total} owners done). Running: {running}. Waiting on: {waiting}. It stops instead of blocking every other read of this machine; run the command again when memory or the fleet is free",
      "de": "Das Lesen des Projektgraphen wurde abgebrochen: kein Projekt kam {minutes} Minuten voran ({done} von {total} Projekten fertig). Es laufen: {running}. Es wartet auf: {waiting}. Es bricht ab, statt jedes andere Lesen dieses Rechners zu blockieren; führe den Befehl erneut aus, sobald Speicher oder Flotte frei sind"
    },
    "graph.owner-too-heavy": {""")

PN=N+"/🟨️.mjs"
rep(PN,"""import { availableParallelism } from "node:os";""","""import { availableParallelism, totalmem } from "node:os";""")
rep(PN,"""import { TAKE_POLICY_V1, createTakeLedgerV1,""","""import { TAKE_POLICY_V1, createProgressWatchV1, createTakeLedgerV1, takeNoProgressTextV1, takeStrongMachineV1,""")
rep(PN,"""let projectIndex=0,cursor=0,stopped=false,takeScope,released;const missed=new Set();""","""let projectIndex=0,cursor=0,stopped=false,stalled=false,takeScope,released;const deferred=new Set();const missed=new Set();""")
rep(PN,"""fleetPort=records.createProjectFleetPortV1({root:workspaceRoot,environment:process.env});""","""fleetPort=records.createProjectFleetPortV1({root:workspaceRoot,environment:process.env,busy:()=>jobs.slice(cursor).some(job=>!taken.has(job.name)&&!never.has(job.name)&&!deferred.has(job.name)),strong:takeStrongMachineV1({memoryMegabytes:Math.floor(totalmem()/1048576),cores:availableParallelism()})});""")
rep(PN,"""const setAside=error=>enforcement.soloLimitBytes!==undefined&&""","""const setAside=error=>deferOwner(error)||enforcement.soloLimitBytes!==undefined&&""")
rep(PN,"""if(!again){finished++;ledger.mark(job.name,"inferred");}}await publishRunning();""","""if(!again&&!deferredNow.delete(job.name)){finished++;ledger.mark(job.name,"inferred");}}await publishRunning();""")
rep(PN,"""  const loop=async end=>{await resolved;""","""  const extras=[],deferredNow=new Set();let helped=false;
  const deferOwner=error=>{if(error?.code!=="BUILD_OWNER_DEFERRED")return false;const owner=jobs.find(job=>job.json.root===error.semioProject);if(owner===undefined)return false;deferred.add(owner.name);deferredNow.add(owner.name);jobs.push(owner);return true;};
  const loop=async end=>{await resolved;if(end===Infinity&&!helped&&alone.size===0&&deferred.size>0){helped=true;for(let extra=1;extra<limit;extra++)extras.push(loop(Infinity));}""")
rep(PN,"""  try{await runTakeBesideInferV1({take:resolveOwners,""","""  const inferAll=async()=>{await awaitGrowingV1(workers);if(!stopped&&cursor<jobs.length){memoryPorts.collect();await loop(Infinity);}await Promise.allSettled(extras);await waitHeavy();};
  try{await runTakeBesideInferV1({take:resolveOwners,""")
rep(PN,"""infer:async()=>{await awaitGrowingV1(workers);if(!stopped&&cursor<jobs.length){memoryPorts.collect();await loop(Infinity);}await waitHeavy();}});}catch(error){stopped=true;released();await Promise.allSettled(workers);throw error;}""","""infer:()=>Promise.race([stall,inferAll()])});}catch(error){stopped=true;released();if(!stalled)await Promise.allSettled(workers);throw error;}""")
rep(PN,"""  const workers=Array.from({length:limit},()=>loop(lightEnd)),grow=""","""  let failStalled;const stall=new Promise((_resolve,reject)=>{failStalled=reject;});stall.catch(()=>undefined);
  const watch=createProgressWatchV1(),watcher=setInterval(async()=>{
    const rows=[...active.values()].reduce((sum,row)=>sum+(observedInferenceRowsV1(row.cachePath)??0),0),hit=watch.check([finished,taken.size,looks.count,cursor,projectIndex,deferred.size,rows,ledger.counts().done,Object.values(fleetPort?.producers?.()??{}).reduce((sum,each)=>sum+each,0)].join("|"));
    if(hit===undefined||stalled)return;
    stalled=true;stopped=true;passStop.abort();polling.abort();
    let peers=[];try{peers=(await import("../../../🪪️graph-state/🛰️exchange/🟦️.ts")).graphFleetWaitingV1();}catch{}
    const at=performance.now(),running=[...active].sort((a,b)=>a[1].since-b[1].since).slice(0,6).map(([name,row])=>name+" ("+Math.round((at-row.since)/1000)+" s, "+(observedInferenceRowsV1(row.cachePath)??0)+" rows)").join(", ")||"nothing",reasons=[];
    if(peers.length>0)reasons.push(peers.length+" owners claimed by live peers ("+[...new Set(peers.map(row=>row.by.split("|")[0]))].slice(0,4).join(", ")+") / "+peers.length+" Projekte, die aktive Peers beanspruchen");
    if(decision!==undefined)reasons.push("the memory governor ("+decision.key+": "+decision.used+" of "+decision.budget+" MB) / die Speichersteuerung ("+decision.key+": "+decision.used+" von "+decision.budget+" MB)");
    if(reasons.length===0)reasons.push("the running owners themselves, which show no row / die laufenden Projekte selbst, die keine Zeile zeigen");
    const fields={minutes:Math.round(hit.sinceMilliseconds/60000),done:ledger.counts().done,total:jobs.length,running,waiting:reasons.join("; ")};
    try{for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+takeNoProgressTextV1(language,fields)+"\\n");}catch{}
    failStalled(refuseBuildV1("unserved","graph.no-progress",fields));
  },TAKE_POLICY_V1.watchMilliseconds);watcher.unref?.();
  const workers=Array.from({length:limit},()=>loop(lightEnd)),grow=""")
rep(PN,"""finally{reportOff();samplerOff();collectionsOff();clearInterval(ticker);""","""finally{clearInterval(watcher);reportOff();samplerOff();collectionsOff();clearInterval(ticker);""")

STAGE=sys.argv[sys.argv.index("--stage")+1] if "--stage" in sys.argv else None
def apply():
    cache={}
    if STAGE:
        for i,(path,old,new) in enumerate(edits):
            mapped=STAGE+path[len(ROOT):]
            if not os.path.exists(mapped):
                os.makedirs(os.path.dirname(mapped),exist_ok=True);shutil.copy2(path,mapped)
            edits[i]=(mapped,old,new)
    for path,old,new in edits:
        t=cache.get(path)
        if t is None:t=open(path,encoding="utf8").read()
        n=t.count(old)
        if n!=1:
            print("ANCHOR",n,path.replace(ROOT,""),repr(old[:70]));return False
        cache[path]=t.replace(old,new)
    if CHECK:print("anchors ok");return True
    if STAGE:
        for path,t2 in cache.items():open(path,"w",encoding="utf8").write(t2)
        print("staged",len(cache));return True
    os.makedirs(BK,exist_ok=True)
    for path,t in cache.items():
        shutil.copy2(path,BK+"/"+os.path.basename(os.path.dirname(path))+"__"+os.path.basename(path)+"."+str(abs(hash(path))%100000))
    json.dump({p:BK+"/"+os.path.basename(os.path.dirname(p))+"__"+os.path.basename(p)+"."+str(abs(hash(p))%100000) for p in cache},open(SC+"/backups.json","w"))
    for path,t in cache.items():
        open(path,"w",encoding="utf8").write(t)
    print("applied",len(cache),"files");return True
sys.exit(0 if apply() else 1)
