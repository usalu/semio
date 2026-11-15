
/** 💓️ What the watchdog thread of a running daemon writes every second to `.cache/f/boot/beat.json`, from a thread that a blocked event loop cannot stop: the process, the boot stage that supervises it, the phase (`boot` while the code loads, `open` while the state opens, `run` once the daemon started), the code it runs and where that came from (`worktree`, `snapshot` or `store`), and how long the event loop has not answered (`lagMs`). */
export type BeatV1=Readonly<{version:1;pid:number;parent:number;startedMs:number;wallMs:number;phase:"boot"|"open"|"run";base:string;source:"worktree"|"snapshot"|"store";lagMs:number}>;

/** 🪂️ The guard of a machine that runs a fallback code instead of the code of its worktree: where the code came from and why the worktree code was not used. */
export type FallbackV1=Readonly<{guard:"code-fallback";source:"snapshot"|"store";reason:string}>;

/** 📍️ The path of the beat file. */
export function beatPathV1(root:string):string{return join(root,".cache","f","boot","beat.json");}

const staleBeatMilliseconds=20_000;
const living=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as {code?:string}).code==="EPERM";}};

/** 👀️ Reads the beat of a daemon that lives: the file is valid, its process exists and its last write is at most 20 s old. A daemon whose event loop is blocked still has such a beat, which is how `status` and the lock tell a busy daemon from a dead one. */
export async function readBeatV1(root:string,now:number=Date.now()):Promise<BeatV1|undefined>{
 try{
  const value=JSON.parse(await readFile(beatPathV1(root),"utf8")) as unknown;
  requireResilienceShapeV1("Beat",value,"Closed beat required");
  const beat=value as BeatV1;
  return living(beat.pid)&&now-beat.wallMs<=staleBeatMilliseconds?beat:undefined;
 }catch{return undefined;}
}

/** 🪂️ Reads the fallback guard that the boot stage hands to the daemon in `SEMIO_FLEET_CODE_FALLBACK`; `undefined` when the daemon runs the code of its worktree or the value is not a closed guard. */
export function readFallbackV1(environment:Readonly<Record<string,string|undefined>>):FallbackV1|undefined{
 const raw=environment.SEMIO_FLEET_CODE_FALLBACK;
 if(raw===undefined||raw==="")return undefined;
 try{const value=JSON.parse(raw) as unknown;requireResilienceShapeV1("Fallback",value,"Closed fallback required");return value as FallbackV1;}catch{return undefined;}
}

/** 🧱️ The blocks of the event loop that this process recorded (`kind` `loop-blocked`, how often and when last): the boot stage counts every wake-up that came `blockMs` or more too late. Empty when none happened. */
export function blockedFailuresV1():readonly FailureSummaryV1[]{
 const found=(globalThis as unknown as Record<symbol,{count:number;lastMs:number}|undefined>)[Symbol.for("semio.fleet.blocked")];
 return found===undefined||found.count===0?[]:[{kind:"loop-blocked",count:found.count,lastMs:found.lastMs}];
}

/** 🗄️ The directory of the code snapshots below a store root: `<store>/v1/bootstrap/code/<digest>/{sealed.json,files/}`, the layout of a local sealed snapshot, so one verification reads both. */
export function storeCodeDirectoryV1(store:string):string{return join(store,"v1","bootstrap","code");}

/** 📚️ Lists the valid code snapshots of a store, newest first. */
export async function listStoreCodeV1(store:string):Promise<CodeSnapshotV1[]>{
 const base=storeCodeDirectoryV1(store),found:CodeSnapshotV1[]=[];
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!/^[0-9a-f]{64}$/u.test(name))continue;
  try{
   const seal=JSON.parse(await readFile(join(base,name,"sealed.json"),"utf8")) as unknown;
   requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
   if((seal as SealedCodeV1).digest===name)found.push({digest:name,createdMs:(seal as SealedCodeV1).createdMs,directory:join(base,name)});
  }catch{}
 }
 return found.sort((left,right)=>right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
}

/** 📤️ Publishes the sealed local snapshot of `digest` to the store so that a machine whose worktree code and local snapshots are all unusable can fetch working code: the files are re-verified against their seal, copied into a private `.tmp-` directory next to the target with `sealed.json` written last, and renamed into `<store>/v1/bootstrap/code/<digest>`, so a snapshot in the store either is complete or does not exist. A present intact snapshot is left alone, a damaged one is replaced. */
export async function publishCodeSnapshotV1(options:Readonly<{root:string;store:string;digest:string;originId:string}>):Promise<Readonly<{published:boolean;reason:string|null}>>{
 const local=join(codeDirectoryV1(options.root),options.digest),base=storeCodeDirectoryV1(options.store),target=join(base,options.digest);
 const broken=await verifyCodeSnapshotV1(local);
 if(broken!==null)return {published:false,reason:`local snapshot: ${broken}`};
 if(await verifyCodeSnapshotV1(target)===null)return {published:false,reason:"present"};
 const temporary=join(base,`.tmp-${options.originId}-${tokenOf()}`),seal=JSON.parse(await readFile(join(local,"sealed.json"),"utf8")) as SealedCodeV1;
 try{
  for(const file of seal.files){
   const destination=join(temporary,"files",...file.path.split("/"));
   await mkdir(dirname(destination),{recursive:true});
   await writeFile(destination,await readFile(join(local,"files",...file.path.split("/"))));
  }
  const handle=await open(join(temporary,"sealed.json"),"w");
  try{await handle.writeFile(JSON.stringify(seal));await handle.sync();}finally{await handle.close();}
  await rm(target,{recursive:true,force:true});
  try{await rename(temporary,target);}catch(error){if(await verifyCodeSnapshotV1(target)!==null)throw error;await rm(temporary,{recursive:true,force:true});return {published:false,reason:"present"};}
  return {published:true,reason:null};
 }catch(error){await rm(temporary,{recursive:true,force:true}).catch(()=>{});return {published:false,reason:(error instanceof Error?error.message:String(error)).slice(0,256)};}
}

/** ✂️ Keeps the newest `keep` code snapshots of a store and every digest in `protect` (the digests that a live origin attests), removes the others and every unfinished `.tmp-` directory older than an hour; returns the removed digests. */
export async function pruneStoreCodeV1(store:string,keep:number,protect:ReadonlySet<string>=new Set(),now:number=Date.now()):Promise<string[]>{
 const base=storeCodeDirectoryV1(store),removed:string[]=[];
 for(const snapshot of (await listStoreCodeV1(store)).slice(Math.max(0,keep))){
  if(protect.has(snapshot.digest))continue;
  await rm(snapshot.directory,{recursive:true,force:true});removed.push(snapshot.digest);
 }
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!name.startsWith(".tmp-"))continue;
  const info=await stat(join(base,name)).catch(()=>undefined);
  if(info!==undefined&&now-info.mtimeMs>3_600_000)await rm(join(base,name),{recursive:true,force:true});
 }
 return removed;
}

/** 🪞️ Copies the boot stage (`📜️script.ts` of the module) to `.cache/f/boot/stage.ts` with `stage.json` (its sha256), both through tmp + rename and the signature last, so the worktree entry, which prefers a copy whose hash matches the signature, never runs a torn copy. Called by `join` and `seed` (provisioning) and by a healthy daemon, so a machine that joined before the stage existed gets it from the next healthy daemon. Returns whether the copy changed. */
export async function installBootStageV1(root:string,script:string):Promise<Readonly<{installed:boolean}>>{
 const data=await readFile(script),digest=sha(data),directory=join(root,".cache","f","boot"),copy=join(directory,"stage.ts"),signature=join(directory,"stage.json");
 const known=await readFile(signature,"utf8").then(text=>(JSON.parse(text) as {sha256?:string}).sha256,()=>undefined),present=await readFile(copy).then(bytes=>sha(bytes),()=>undefined);
 if(known===digest&&present===digest)return {installed:false};
 await mkdir(directory,{recursive:true});
 const first=`${copy}.${tokenOf()}.tmp`,second=`${signature}.${tokenOf()}.tmp`;
 try{
  await writeFile(first,data);await rm(signature,{force:true});await rename(first,copy);
  await writeFile(second,JSON.stringify({version:1,sha256:digest,bytes:data.length,wallMs:Date.now()}));await rename(second,signature);
 }catch(error){await rm(first,{force:true});await rm(second,{force:true});throw error;}
 return {installed:true};
}

/** ⏳️ A cooperative yield for long loops over a whole manifest, queue or event batch: `turn()` costs next to nothing per call and, once `budgetMilliseconds` (8) of work have passed since the last yield, returns a promise that settles on the next turn of the event loop, so timers, signals, the store guard and the heartbeats of the daemon are served between the slices. Use it as `await turn()` inside the loop. */
export function yieldingV1(budgetMilliseconds:number=8):()=>Promise<void>|undefined{
 let calls=0,started=performance.now();
 return ()=>{
  if((++calls&63)!==0||performance.now()-started<budgetMilliseconds)return undefined;
  return new Promise<void>(done=>{setImmediate(()=>{started=performance.now();done();});});
 };
}
