const publicationWindow=setupSchema.definitions.FundingPublicationWindowProtocol.const;
/** 🪟️ What the staged funding touches outside its own decisions: the clock, process liveness, this process, the one exclusive hold of the store (taken only for the publish, with a signal that ends the wait), the progress line and an optional fault hook that tests use to stop between the steps of the publish. */
export type FundingWindowPortsV1=Readonly<{now():number;alive(pid:number,startedAtMs:number):boolean;pid:number;startedAtMs:number;exclusive<T>(body:()=>Promise<T>,signal:AbortSignal):Promise<T>;say(heldMilliseconds:number,limitMilliseconds:number,what:string):void;hook?(point:"pointer-written"|"renamed"):Promise<void>}>;
const processStartedAtMsV1=(pid:number,now:number):number|undefined=>{
  if(process.platform==="win32")return undefined;
  const run=spawnSync("ps",["-o","etime=","-p",String(pid)],{encoding:"utf8",env:{...process.env,LC_ALL:"C"}}),match=/^\s*(?:(?:(\d+)-)?(\d+):)?(\d+):(\d+)\s*$/u.exec(run.stdout);
  return run.status===0&&match?now-((Number(match[1]??0)*24+Number(match[2]??0))*3600+Number(match[3])*60+Number(match[4]))*1000:undefined;
};
/** 🏭️ The real ports of the staged funding for this process: the system clock, liveness that also compares the start time of the process with the one recorded (a reused pid is a gone process), the caller's exclusive hold of the store, and the bilingual `exclusive for N ms` line. */
export function fundingWindowPortsV1(exclusive:FundingWindowPortsV1["exclusive"]):FundingWindowPortsV1{
  return Object.freeze({now:()=>Date.now(),pid:process.pid,startedAtMs:leaseProcessStartedAtMsV1,exclusive,
    alive:(pid,startedAtMs)=>{if(!processAliveV1(pid))return false;if(pid===process.pid)return true;const started=processStartedAtMsV1(pid,Date.now());return started===undefined||Math.abs(started-startedAtMs)<=publicationWindow.startToleranceMilliseconds;},
    say:(ms,limit,what)=>sayCachingV1("dependency-store.exclusive-window",{ms,limit,what},ms>limit?"warn":"info")});
}
/** 📍️ The published state: for every scope the one generation readers see, a sequence that grows with each publish and the time at which a generation stopped being current. */
export type FundingPointerV1=Readonly<{version:1;sequence:number;scopes:Readonly<Record<string,string>>;superseded:Readonly<Record<string,number>>}>;
/** 🧱️ One scope that was built beside the store and is complete: its staging directory name and the generation it becomes. */
export type FundingStagedScopeV1=Readonly<{scope:string;generation:string;name:string}>;
/** 🔭️ What a reader holds: the pointer it opened and a release that drops its pins; every generation of the pointer stays on disk until then. */
export type FundingSnapshotV1=Readonly<{pointer:FundingPointerV1;directory(scope:string):string;release():void}>;
const fundingScopeOk=(value:string)=>/^[a-z0-9][a-z0-9._-]{0,63}$/u.test(value),fundingGenerationOk=(value:string)=>/^[a-f0-9]{20}$/u.test(value);
const fundingLayoutV1=(root:string)=>Object.freeze({staging:join(root,publicationWindow.stagingDirectory),generations:join(root,publicationWindow.generationDirectory),pins:join(root,publicationWindow.pinDirectory),retiring:join(root,publicationWindow.retiringDirectory),pointer:join(root,publicationWindow.pointerName)});
const fundingNonce=()=>randomInt(0,0xffffffff).toString(16).padStart(8,"0");
const fundingSyncFileV1=(path:string)=>{const handle=openSync(path,"r");try{fsyncSync(handle);}finally{closeSync(handle);}};
const fundingWriteDurableV1=(path:string,text:string)=>{const handle=openSync(path,"w");try{writeFileSync(handle,text);fsyncSync(handle);}finally{closeSync(handle);}};
const fundingParseOwnerV1=(text:string):Readonly<{pid:number;startedAtMs:number}>|undefined=>{try{const row=JSON.parse(text);return Number.isSafeInteger(row?.pid)&&Number.isSafeInteger(row?.startedAtMs)?Object.freeze({pid:row.pid,startedAtMs:row.startedAtMs}):undefined;}catch{return undefined;}};
/** 📍️ Reads the published pointer; no pointer yet is the empty state, and a pointer that is not exactly the schema shape is refused because it is only ever written by an atomic rename. */
export function readFundingPointerV1(root:string):FundingPointerV1{
  let text:string;try{text=readFileSync(fundingLayoutV1(root).pointer,"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return Object.freeze({version:1,sequence:0,scopes:Object.freeze({}),superseded:Object.freeze({})});throw error;}
  const row=JSON.parse(text);
  if(row?.version!==1||!Number.isSafeInteger(row.sequence)||row.sequence<1||typeof row.scopes!=="object"||row.scopes===null||typeof row.superseded!=="object"||row.superseded===null)throw Error("Exact funding pointer required");
  for(const[scope,generation]of Object.entries(row.scopes))if(!fundingScopeOk(scope)||typeof generation!=="string"||!fundingGenerationOk(generation))throw Error("Exact funding pointer scope required");
  for(const[generation,at]of Object.entries(row.superseded))if(!fundingGenerationOk(generation)||!Number.isSafeInteger(at))throw Error("Exact funding pointer supersession required");
  return Object.freeze({version:1,sequence:row.sequence,scopes:Object.freeze({...row.scopes}),superseded:Object.freeze({...row.superseded})});
}
/** 🧹️ Makes the store valid after any interruption: a staging directory whose owner process is gone is removed unless it is complete (then it is kept for the next run of the same scope and generation to adopt), a pointer temporary file of a gone process is removed; nothing a live process owns is touched. */
export function recoverFundingStagingV1(root:string,ports:FundingWindowPortsV1):Readonly<{removed:readonly string[];adoptable:readonly string[]}>{
  const layout=fundingLayoutV1(root),removed:string[]=[],adoptable:string[]=[];
  for(const name of existsSync(layout.staging)?readdirSync(layout.staging):[]){
    const directory=join(layout.staging,name),owner=(()=>{try{return fundingParseOwnerV1(readFileSync(join(directory,"owner.json"),"utf8"));}catch{return undefined;}})();
    if(owner&&ports.alive(owner.pid,owner.startedAtMs))continue;
    if(owner&&existsSync(join(directory,publicationWindow.completeName))){adoptable.push(name);continue;}
    rmSync(directory,{recursive:true,force:true});removed.push(name);
  }
  for(const name of existsSync(root)?readdirSync(root):[]){const match=/^current\.json\.(\d+)\.(\d+)\.[0-9a-f]{8}\.tmp$/u.exec(name);if(match&&!ports.alive(Number(match[1]),Number(match[2]))){rmSync(join(root,name),{force:true});removed.push(name);}}
  if(removed.length)sayCachingV1("dependency-store.staging-recovered",{count:removed.length},"warn");
  return Object.freeze({removed:Object.freeze(removed),adoptable:Object.freeze(adoptable)});
}
/** 🧱️ Builds one scope beside the store with no hold at all: a scope already published at this generation is a hit, a complete staging of a gone process for it is adopted, otherwise `build` fills a fresh staging directory (owner first, completeness marker last, so an interrupted build is never complete); cancellation removes the staging directory. Two scopes stage concurrently. */
export async function stageFundingScopeV1(root:string,ports:FundingWindowPortsV1,request:Readonly<{scope:string;generation:string;signal:AbortSignal}>,build:(directory:string,signal:AbortSignal)=>Promise<void>):Promise<Readonly<{origin:"published"|"adopted"|"built";staged?:FundingStagedScopeV1}>>{
  if(!fundingScopeOk(request.scope)||!fundingGenerationOk(request.generation))throw Error("Exact funding scope and generation required");
  const layout=fundingLayoutV1(root);request.signal.throwIfAborted();
  if(readFundingPointerV1(root).scopes[request.scope]===request.generation)return Object.freeze({origin:"published"});
  mkdirSync(layout.staging,{recursive:true});
  const prefix=request.scope+"."+request.generation+".",found=recoverFundingStagingV1(root,ports).adoptable.find(name=>name.startsWith(prefix));
  if(found){const name=prefix+ports.pid+"-"+fundingNonce();renameSync(join(layout.staging,found),join(layout.staging,name));fundingWriteDurableV1(join(layout.staging,name,"owner.json"),JSON.stringify({pid:ports.pid,startedAtMs:ports.startedAtMs}));return Object.freeze({origin:"adopted",staged:Object.freeze({scope:request.scope,generation:request.generation,name})});}
  const name=prefix+ports.pid+"-"+fundingNonce(),directory=join(layout.staging,name);mkdirSync(directory);
  try{
    fundingWriteDurableV1(join(directory,"owner.json"),JSON.stringify({pid:ports.pid,startedAtMs:ports.startedAtMs}));
    await build(join(directory,"tree"),request.signal);request.signal.throwIfAborted();
    fundingWriteDurableV1(join(directory,publicationWindow.completeName),JSON.stringify({version:1,scope:request.scope,generation:request.generation}));
  }catch(error){rmSync(directory,{recursive:true,force:true});throw error;}
  return Object.freeze({origin:"built",staged:Object.freeze({scope:request.scope,generation:request.generation,name})});
}
/** 🪟️ Publishes complete staged scopes: the checks happen before the hold, the hold is taken for the intent file, the renames and one pointer swap only, waits at most the schema bound for it and is reported with its duration as `exclusive for N ms`; a crash before the pointer swap leaves the old state published (unreferenced generations are retired as orphans), after it the new one, and a generation that is already published is not replaced. */
export async function publishFundingScopesV1(root:string,ports:FundingWindowPortsV1,staged:readonly FundingStagedScopeV1[],signal:AbortSignal):Promise<Readonly<{sequence:number;exclusiveMilliseconds:number;within:boolean}>>{
  const layout=fundingLayoutV1(root);
  for(const row of staged){if(!fundingScopeOk(row.scope)||!fundingGenerationOk(row.generation)||row.name.includes("/"))throw Error("Exact staged funding scope required");if(!existsSync(join(layout.staging,row.name,publicationWindow.completeName)))throw Error("Complete staged funding scope required");}
  mkdirSync(layout.generations,{recursive:true});
  const wait=AbortSignal.any([signal,AbortSignal.timeout(publicationWindow.exclusiveWaitMilliseconds)]);
  return ports.exclusive(async()=>{
    const started=ports.now(),current=readFundingPointerV1(root),scopes={...current.scopes},superseded:Record<string,number>={};
    for(const row of staged){const previous=scopes[row.scope];scopes[row.scope]=row.generation;if(previous!==undefined&&previous!==row.generation&&!Object.values(scopes).includes(previous))superseded[previous]=ports.now();}
    for(const[generation,at]of Object.entries(current.superseded))if(!Object.values(scopes).includes(generation)&&existsSync(join(layout.generations,generation)))superseded[generation]??=at;
    const next:FundingPointerV1={version:1,sequence:current.sequence+1,scopes,superseded},temporary=layout.pointer+"."+ports.pid+"."+ports.startedAtMs+"."+fundingNonce()+".tmp";
    fundingWriteDurableV1(temporary,JSON.stringify(next));await ports.hook?.("pointer-written");
    for(const row of staged){
      const target=join(layout.generations,row.generation);
      if(existsSync(target))rmSync(join(layout.staging,row.name),{recursive:true,force:true});else{fundingSyncFileV1(join(layout.staging,row.name));renameSync(join(layout.staging,row.name,"tree"),target);rmSync(join(layout.staging,row.name),{recursive:true,force:true});}
    }
    await ports.hook?.("renamed");renameSync(temporary,layout.pointer);
    const held=Math.round(ports.now()-started),within=held<=publicationWindow.maximumExclusiveMilliseconds;
    ports.say(held,publicationWindow.maximumExclusiveMilliseconds,staged.map(row=>row.scope).join(","));
    return Object.freeze({sequence:next.sequence,exclusiveMilliseconds:held,within});
  },wait);
}
/** 🔭️ Opens the published state for reading without any lock: reads the pointer, pins every generation it names with a file that carries this process, reads the pointer again and retries when it moved; the pinned generations are complete and stay on disk until `release`. */
export function openFundingSnapshotV1(root:string,ports:FundingWindowPortsV1):FundingSnapshotV1{
  const layout=fundingLayoutV1(root);
  for(let attempt=0;attempt<publicationWindow.maximumSnapshotAttempts;attempt++){
    const pointer=readFundingPointerV1(root),pins:string[]=[];
    try{
      for(const generation of Object.values(pointer.scopes)){const folder=join(layout.pins,generation);mkdirSync(folder,{recursive:true});const pin=join(folder,ports.pid+"."+ports.startedAtMs+"."+fundingNonce());writeFileSync(pin,"");pins.push(pin);if(!existsSync(join(layout.generations,generation)))throw Object.assign(Error("moved"),{moved:true});}
      if(readFundingPointerV1(root).sequence!==pointer.sequence)throw Object.assign(Error("moved"),{moved:true});
    }catch(error){for(const pin of pins)rmSync(pin,{force:true});if((error as{moved?:boolean}).moved)continue;throw error;}
    let released=false;
    return Object.freeze({pointer,directory:(scope:string)=>{const generation=pointer.scopes[scope];if(generation===undefined)throw Error("Published funding scope required");return join(layout.generations,generation);},release:()=>{if(released)return;released=true;for(const pin of pins)rmSync(pin,{force:true});}});
  }
  throw Error("Stable funding snapshot required");
}
/** ♻️ Retires generations that no reader pins: a generation leaves when it is not in the pointer, was superseded longer ago than the schema grace (or never was published and no live publish is in flight) and no live process holds a pin (pins of gone processes are removed); it is renamed away first and removed after, renamed back when a pin shows up in between, and what an interrupted retirement left is finished the same way. No exclusive hold is needed. */
export function retireFundingGenerationsV1(root:string,ports:FundingWindowPortsV1):readonly string[]{
  const layout=fundingLayoutV1(root),pointer=readFundingPointerV1(root),current=new Set(Object.values(pointer.scopes)),retired:string[]=[];
  const pinned=(generation:string)=>{const folder=join(layout.pins,generation);let live=false;for(const name of existsSync(folder)?readdirSync(folder):[]){const[pid,started]=name.split(".");if(ports.alive(Number(pid),Number(started)))live=true;else rmSync(join(folder,name),{force:true});}return live;};
  const publishing=(existsSync(root)?readdirSync(root):[]).some(name=>{const match=/^current\.json\.(\d+)\.(\d+)\.[0-9a-f]{8}\.tmp$/u.exec(name);return match!==null&&ports.alive(Number(match[1]),Number(match[2]));});
  for(const name of existsSync(layout.retiring)?readdirSync(layout.retiring):[]){
    const generation=name.split(".")[0]!,away=join(layout.retiring,name);
    if(pinned(generation)&&!existsSync(join(layout.generations,generation)))renameSync(away,join(layout.generations,generation));else{rmSync(away,{recursive:true,force:true});if(!pinned(generation))rmSync(join(layout.pins,generation),{recursive:true,force:true});}
  }
  for(const generation of existsSync(layout.generations)?readdirSync(layout.generations):[]){
    if(!fundingGenerationOk(generation)||current.has(generation))continue;
    const at=pointer.superseded[generation];if(at===undefined?publishing:ports.now()-at<publicationWindow.retireGraceMilliseconds)continue;
    if(pinned(generation))continue;
    mkdirSync(layout.retiring,{recursive:true});const away=join(layout.retiring,generation+"."+fundingNonce());
    try{renameSync(join(layout.generations,generation),away);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}
    if(pinned(generation)){renameSync(away,join(layout.generations,generation));continue;}
    rmSync(away,{recursive:true,force:true});rmSync(join(layout.pins,generation),{recursive:true,force:true});retired.push(generation);
  }
  return Object.freeze(retired);
}
