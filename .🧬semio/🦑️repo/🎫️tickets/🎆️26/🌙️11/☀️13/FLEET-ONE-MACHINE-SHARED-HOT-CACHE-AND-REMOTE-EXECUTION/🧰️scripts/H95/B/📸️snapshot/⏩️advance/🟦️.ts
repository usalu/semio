import {createHash} from "node:crypto";
import {appendFileSync,existsSync,lstatSync,mkdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,relative,resolve} from "node:path";
import policy from "../🔣️.json" with {type:"json"};
import {journalReadV1,type JournalCursorV1} from "../../📜️change-journal/🟦️.ts";
import {snapshotAliveV1,snapshotCaptureV1,snapshotCopyFileV1,snapshotDigestV1,snapshotFingerprintV1,snapshotHashV1,snapshotLaneNameV1,snapshotLinkTargetV1,snapshotReadManifestV1,snapshotStatV1,snapshotWriteAtomicV1,type ManifestV1,type SnapshotPolicyV1,type SnapshotPortsV1,type SnapshotReceiptV1,type SnapshotModeV1} from "../🟦️.ts";

const rules=policy as SnapshotPolicyV1;

/** 🧱️ The persistent snapshot of a machine: one tree advanced from the replica journal, its state (generation, digest, journal cursor, moment of the last full verification) and the generation log the lane roots refresh from. */
export type SnapshotBaseStateV1=Readonly<{version:1;generation:number;digest:string;files:number;cursor:JournalCursorV1;verifiedAtMs:number;changelogFrom:number;pending?:Readonly<Record<string,Readonly<{deadlineMs:number;operation:string;sha256?:string}>>>}>;
/** 📸️ What advancing the persistent snapshot did. */
export type SnapshotAdvanceV1=Readonly<{generation:number;digest:string;files:number;via:"journal"|"listing";changed:number;rounds:number;consistent:boolean;moved?:readonly string[];seconds:number;lagSeconds:number;pending:number}>;
/** 🔌️ The seams of advance and refresh. */
export type SnapshotAdvancePortsV1=SnapshotPortsV1&Readonly<{store?:string;extraPaths?:readonly string[];fresh?:boolean;clone?:(source:string,target:string)=>Promise<void>;journal?:typeof journalReadV1;liveRoot?:string}>;

/** 🗂️ The files of the persistent snapshot below the snapshot folder. */
export const snapshotBasePathsV1=(base:string,limits:SnapshotPolicyV1=rules)=>{const name=limits.incremental.lane;return {directory:join(base,name),manifest:join(base,name+"."+limits.storage.manifestFile),state:join(base,name+".state.json"),changelog:join(base,name+".changelog.jsonl"),lock:join(base,name+limits.storage.lockSuffix)};};

const readState=(path:string):SnapshotBaseStateV1|undefined=>{try{const value=JSON.parse(readFileSync(path,"utf8")) as SnapshotBaseStateV1;return value.version===1&&typeof value.generation==="number"?value:undefined;}catch{return undefined;}};

/** 🔒️ Holds the lock of the persistent snapshot for one piece of work (a dead owner's lock is taken over, a live one is waited for). */
export async function snapshotWithBaseLockV1<T>(base:string,work:()=>Promise<T>,ports:Readonly<{sleep?:(milliseconds:number)=>Promise<void>;now?:()=>number}>={},limits:SnapshotPolicyV1=rules):Promise<T>{
 const paths=snapshotBasePathsV1(base,limits),sleep=ports.sleep??(milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))),now=ports.now??Date.now,limit=now()+limits.incremental.lockWaitSeconds*1000;
 mkdirSync(base,{recursive:true});
 for(;;){
  try{mkdirSync(paths.lock);writeFileSync(join(paths.lock,"owner"),String(process.pid));break;}catch{
   let owner=NaN;try{owner=Number(readFileSync(join(paths.lock,"owner"),"utf8"));}catch{}
   if(!(Number.isInteger(owner)&&snapshotAliveV1(owner))){rmSync(paths.lock,{recursive:true,force:true});continue;}
   if(now()>limit)throw Error("snapshot-base-lock-busy");
   await sleep(100);
  }
 }
 try{return await work();}finally{rmSync(paths.lock,{recursive:true,force:true});}
}

/** 🎯️ Whether a journal path belongs to the closure the snapshot copies (the same rule as the listing). */
export function snapshotInClosureV1(path:string,heavyMembers:readonly string[],limits:SnapshotPolicyV1=rules):boolean{
 const parts=path.split("/");
 if(parts.some(part=>limits.excludedSegments.includes(part)))return false;
 if(parts.length===1)return true;
 if(limits.roots.includes(parts[0]!))return true;
 return limits.rustRoots.includes(parts[0]!)&&parts.length>2&&!heavyMembers.includes(parts[1]!)||limits.rustRoots.includes(parts[0]!)&&parts.length===2;
}

/** 📎️ The paths a command line names (an argument that is a file below the checkout, relative to the checkout or to the working directory): the files the caller just edited and wants checked, verified against the live tree whatever the journal says. */
export function snapshotArgumentPathsV1(root:string,cwd:string,args:readonly string[]):readonly string[]{
 const found=new Set<string>();
 for(const argument of args){
  if(argument===""||argument.startsWith("-"))continue;
  for(const candidate of [resolve(cwd,argument),resolve(root,argument)]){
   const inside=relative(root,candidate);
   if(inside===""||inside.startsWith("..")||isAbsolute(inside))continue;
   try{if(lstatSync(candidate).isFile()){found.add(inside.split(/[\\/]/u).join("/"));break;}}catch{}
  }
 }
 return [...found];
}

const appendChangelog=(paths:ReturnType<typeof snapshotBasePathsV1>,generation:number,changed:readonly string[],full:boolean,state:SnapshotBaseStateV1|undefined,limits:SnapshotPolicyV1):number=>{
 const wide=full||changed.length>limits.incremental.fullChangelogThreshold;
 appendFileSync(paths.changelog,JSON.stringify(wide?{g:generation,full:true}:{g:generation,paths:changed})+"\n");
 let from=state?.changelogFrom??generation;
 if(wide)from=generation;
 if(generation-from>=limits.incremental.changelogKeep)from=generation-limits.incremental.changelogKeep+1;
 if(from>(state?.changelogFrom??0)){
  const keep=readFileSync(paths.changelog,"utf8").split("\n").filter(line=>{if(line==="")return false;try{return (JSON.parse(line) as {g:number}).g>=from;}catch{return false;}});
  snapshotWriteAtomicV1(paths.changelog,keep.join("\n")+"\n");
 }
 return from;
};

/** ⏩️ Advances the persistent snapshot to the present: with a fleet store only the paths the replica journal names since the saved cursor (plus the paths the caller names) are looked at, copied or removed, in rounds until a read of the journal brings nothing new (the cut is then consistent with the journal); without a journal, without a base, after the declared minutes without a full check or when asked (`fresh`) the whole closure is listed and verified. A change gives a new generation, a line in the generation log and a new digest; no change costs one journal read. Returns what happened; the caller holds the lock. */
export async function snapshotAdvanceV1(request:Readonly<{root:string;base:string}>,ports:SnapshotAdvancePortsV1={},limits:SnapshotPolicyV1=rules):Promise<SnapshotAdvanceV1>{
 const now=ports.now??Date.now,began=now(),root=resolve(request.root),paths=snapshotBasePathsV1(request.base,limits),heavy=ports.heavyMembers??(await import("../../🚉️land/🟦️.ts")).readLandPolicyV1().overlay.heavyMembers,read=ports.journal??journalReadV1;
 const state=readState(paths.state),dueFull=ports.store===undefined||state===undefined||ports.fresh===true||!existsSync(paths.directory)||began-state.verifiedAtMs>limits.incremental.fullVerifyMinutes*60_000||!existsSync(paths.manifest);
 if(dueFull){
  const start=ports.store===undefined?undefined:read(ports.store,state?.cursor??{}),changedPaths:string[]=[];
  const receipt=await snapshotCaptureV1({root,lane:limits.incremental.lane},{...ports,base:request.base,heavyMembers:heavy,link:async()=>{},changed:(copied,removed)=>{changedPaths.push(...copied,...removed);}},limits);
  const generation=(state?.generation??0)+1,from=appendChangelog(paths,generation,changedPaths,state===undefined,state,limits);
  snapshotWriteAtomicV1(paths.state,JSON.stringify({version:1,generation,digest:receipt.digest,files:receipt.files,cursor:start?.cursor??{},verifiedAtMs:now(),changelogFrom:from} satisfies SnapshotBaseStateV1));
  return {generation,digest:receipt.digest,files:receipt.files,via:"listing",changed:receipt.copied+receipt.removed,rounds:receipt.rounds,consistent:receipt.consistent,...receipt.moved===undefined?{}:{moved:receipt.moved},seconds:Math.round((now()-began)/10)/100,lagSeconds:0,pending:0};
 }
 let cursor=state.cursor,rounds=0,consistent=false,newest=0,loaded:ManifestV1|undefined;
 const changed=new Set<string>(),removed=new Set<string>(),extra=new Set((ports.extraPaths??[]).filter(path=>snapshotInClosureV1(path,heavy,limits))),unstable=new Set<string>();
 const pending=new Map<string,{deadlineMs:number;operation:string;sha256?:string}>(Object.entries(state.pending??{}));
 const manifest=():ManifestV1=>loaded??=snapshotReadManifestV1(paths.manifest);
 const contentSha=(path:string):string|undefined=>{try{return createHash("sha256").update(readFileSync(join(root,path))).digest("hex");}catch{return undefined;}};
 const settled=(path:string,row:{operation:string;sha256?:string}):boolean=>row.operation==="delete"?snapshotStatV1(join(root,path))===undefined:row.sha256===undefined||contentSha(path)===row.sha256;
 for(let round=1;round<=limits.incremental.maximumRounds;round++){
  rounds=round;
  ports.beforeRound?.(round);
  const journal=read(ports.store!,cursor);cursor=journal.cursor;
  const batch=new Set<string>(round===1?[...extra,...pending.keys()]:[]);
  for(const event of journal.events)if(snapshotInClosureV1(event.path,heavy,limits)){batch.add(event.path);newest=Math.max(newest,event.atMs);pending.set(event.path,{deadlineMs:now()+limits.incremental.pendingSeconds*1000,operation:event.operation,...event.sha256===undefined?{}:{sha256:event.sha256}});}
  if(!batch.size){consistent=unstable.size===0;break;}
  const entries=manifest().entries;
  for(const path of batch){
   const live=snapshotStatV1(join(root,path)),known=entries[path];
   if(live===undefined){if(known!==undefined){rmSync(join(paths.directory,path),{force:true});delete entries[path];removed.add(path);changed.delete(path);}}
   else if(known?.[0]!==snapshotFingerprintV1(live)){
    const result=snapshotCopyFileV1(root,paths.directory,path,limits.capture.fileRetries,ports.afterCopy);
    if(result===null){if(known!==undefined){delete entries[path];removed.add(path);}}
    else if(result===undefined){unstable.add(path);if(known!==undefined)delete entries[path];}
    else{entries[path]=[result,snapshotHashV1(paths.directory,path,root,result)];changed.add(path);removed.delete(path);unstable.delete(path);}
   }
   const row=pending.get(path);
   if(row!==undefined&&(settled(path,row)||now()>row.deadlineMs))pending.delete(path);
  }
  if(round===limits.incremental.maximumRounds)consistent=false;
 }
 const total=changed.size+removed.size;
 let generation=state.generation,digest=state.digest,files=state.files;
 if(total>0){
  const entries=manifest().entries,hashes=new Map(Object.entries(entries).map(([path,row])=>[path,row[1]!]));
  generation=state.generation+1;digest=snapshotDigestV1(hashes);files=hashes.size;
  const from=appendChangelog(paths,generation,[...changed,...removed],false,state,limits);
  snapshotWriteAtomicV1(paths.manifest,JSON.stringify({version:1,open:false,entries}));
  snapshotWriteAtomicV1(paths.state,JSON.stringify({version:1,generation,digest,files,cursor,verifiedAtMs:state.verifiedAtMs,changelogFrom:from,...pending.size?{pending:Object.fromEntries(pending)}:{}} satisfies SnapshotBaseStateV1));
 }else snapshotWriteAtomicV1(paths.state,JSON.stringify({...state,cursor,pending:pending.size?Object.fromEntries(pending):undefined} satisfies SnapshotBaseStateV1));
 return {generation,digest,files,via:"journal",changed:total,rounds,consistent,...consistent?{}:{moved:[...unstable].slice(0,limits.capture.movedExamples)},seconds:Math.round((now()-began)/10)/100,lagSeconds:newest===0?0:Math.max(0,Math.round((now()-newest)/1000)),pending:pending.size};
}

const cloneDirectory=async(source:string,target:string):Promise<void>=>{
 const {landCloneTreeV1}=await import("../../🚉️land/🔬️gates/🟦️.ts");
 await landCloneTreeV1(source,target);
};

/** 🔀️ Brings the run root of a lane to a generation of the persistent snapshot: only the paths the generation log names between the lane's generation and the wanted one are cloned from the snapshot (or removed), so the cost follows the changes; a lane that is new, older than the log or spans a full verification is cloned whole in one copy-on-write directory clone. The lane root keeps its path and its files' times, so the compiler's incremental state survives, and it is not touched again until the lane is refreshed: this is the generation pin of a run. Returns what was copied and removed and whether the lane was cloned whole. */
export async function snapshotRefreshLaneV1(request:Readonly<{base:string;lane:string;generation:number}>,ports:SnapshotAdvancePortsV1={},limits:SnapshotPolicyV1=rules):Promise<Readonly<{directory:string;copied:number;removed:number;whole:boolean}>>{
 const paths=snapshotBasePathsV1(request.base,limits),directory=join(request.base,snapshotLaneNameV1(request.lane)),laneFile=join(request.base,snapshotLaneNameV1(request.lane)+".lane.json"),state=readState(paths.state);
 let lane:{generation:number}|undefined;try{lane=JSON.parse(readFileSync(laneFile,"utf8"));}catch{}
 let whole=lane===undefined||!existsSync(directory)||state===undefined||lane.generation<state.changelogFrom-1;
 const todo=new Set<string>();
 if(!whole&&lane!==undefined){
  if(lane.generation>request.generation)whole=true;
  else for(const line of (()=>{try{return readFileSync(paths.changelog,"utf8").split("\n");}catch{return [];}})()){
   if(line==="")continue;
   const row=JSON.parse(line) as {g:number;full?:boolean;paths?:string[]};
   if(row.g<=lane.generation||row.g>request.generation)continue;
   if(row.full)whole=true;else for(const path of row.paths??[])todo.add(path);
  }
 }
 let copied=0,removed=0;
 if(whole){
  rmSync(directory,{recursive:true,force:true});mkdirSync(dirname(directory),{recursive:true});
  await (ports.clone??cloneDirectory)(paths.directory,directory);
  copied=state?.files??0;
  for(const [path,row] of Object.entries(snapshotReadManifestV1(paths.manifest).entries)){const target=snapshotLinkTargetV1(row[1]??"",directory);if(target!==undefined){rmSync(join(directory,path),{force:true});symlinkSync(target,join(directory,path));}}
 }else for(const path of todo){
  const source=join(paths.directory,path);
  if(!snapshotStatV1(source)){rmSync(join(directory,path),{force:true});removed++;continue;}
  const result=snapshotCopyFileV1(paths.directory,directory,path,limits.capture.fileRetries);
  if(typeof result==="string")copied++;
 }
 await (ports.link??defaultLink)(ports.liveRoot??"",directory);
 snapshotWriteAtomicV1(laneFile,JSON.stringify({generation:request.generation}));
 return {directory,copied,removed,whole};
}

const defaultLink=async(liveRoot:string,directory:string):Promise<void>=>{
 if(liveRoot==="")return;
 const {landLinkWorkspaceV1}=await import("../../🚉️land/🔬️gates/🟦️.ts");
 await landLinkWorkspaceV1(liveRoot,directory);
};

/** 📌️ Pins the source of one command: advances the persistent snapshot (a fleet store makes that O(changes)), refreshes the lane root to that generation and returns the receipt (digest, generation, how it was verified, the lag of the newest change the journal showed). Without a fleet store the whole closure is captured into the lane as before. `build:false` refuses to build a missing persistent snapshot (nothing is returned), so a command that only reports a digest never pays for the first full capture. */
export async function snapshotAcquireV1(request:Readonly<{root:string;lane:string;mode:SnapshotModeV1;base:string;build:boolean}>,ports:SnapshotAdvancePortsV1={},limits:SnapshotPolicyV1=rules):Promise<SnapshotReceiptV1|undefined>{
 const now=ports.now??Date.now,began=now(),root=resolve(request.root);
 if(ports.store===undefined){
  return snapshotCaptureV1({root,lane:request.lane,mode:request.mode},{...ports,base:request.base},limits);
 }
 const paths=snapshotBasePathsV1(request.base,limits);
 if(!request.build&&readState(paths.state)===undefined)return undefined;
 return snapshotWithBaseLockV1(request.base,async()=>{
  const heavy=ports.heavyMembers??(await import("../../🚉️land/🟦️.ts")).readLandPolicyV1().overlay.heavyMembers,advance=await snapshotAdvanceV1({root,base:request.base},{...ports,heavyMembers:heavy},limits);
  const lane=request.mode==="digest"?{directory:paths.directory,copied:0,removed:0}:await snapshotRefreshLaneV1({base:request.base,lane:request.lane,generation:advance.generation},{...ports,liveRoot:root},limits);
  const receipt:SnapshotReceiptV1={version:1,digest:advance.digest,files:advance.files,copied:lane.copied,removed:lane.removed,rounds:advance.rounds,consistent:advance.consistent,...advance.moved===undefined?{}:{moved:advance.moved},seconds:Math.round((now()-began)/10)/100,liveRoot:root,snapshotRoot:lane.directory,mode:request.mode,generation:advance.generation,via:advance.via,lagSeconds:advance.lagSeconds,pending:advance.pending};
  return Object.freeze(receipt);
 },ports,limits);
}
