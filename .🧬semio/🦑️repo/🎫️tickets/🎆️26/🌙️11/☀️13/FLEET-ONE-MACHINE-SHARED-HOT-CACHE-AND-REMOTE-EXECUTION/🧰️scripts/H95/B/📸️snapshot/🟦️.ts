import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {constants,copyFileSync,existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,readlinkSync,realpathSync,renameSync,rmSync,statSync,symlinkSync,writeFileSync,type BigIntStats} from "node:fs";
import {dirname,isAbsolute,join,relative,resolve} from "node:path";
import policy from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {snapshotTextV1} from "./📖️texts/🟦️.ts";
import type {SnapshotAdvancePortsV1} from "./⏩️advance/🟦️.ts";

/** 📐️ The closed policy of the source snapshot: which commands run on it, the roots it copies, the capture bounds and where it lives.
 * @see ./🧬️schema/🔣️.json */
export type SnapshotPolicyV1=Readonly<typeof policy>;
/** 🎚️ `root`: the command runs with the snapshot as its checkout; `digest`: it runs on the live tree and only reports the digest of a consistent cut (commands bound to the managed workspaces of the live checkout). */
export type SnapshotModeV1="root"|"digest";
/** 🧾️ What one capture produced and what a result reports: the digest of the content, the counts, whether the cut is one moment and where it lives. */
export type SnapshotReceiptV1=Readonly<{version:1;digest:string;files:number;copied:number;removed:number;rounds:number;consistent:boolean;moved?:readonly string[];seconds:number;liveRoot:string;snapshotRoot:string;mode:SnapshotModeV1;generation?:number;via?:"journal"|"listing";lagSeconds?:number;pending?:number}>;
/** 🔌️ The seams of a capture; tests replace them, the defaults touch the real tree. */
export type SnapshotPortsV1=Readonly<{
 now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;signal?:AbortSignal;base?:string;heavyMembers?:readonly string[];
 beforeRound?:(round:number)=>void;afterCopy?:(path:string)=>void;changed?:(copied:readonly string[],removed:readonly string[])=>void;
 link?:(liveRoot:string,snapshotRoot:string)=>Promise<void>;
 notice?:(line:string)=>void;
}>;

const rules=policy as SnapshotPolicyV1;
type StatV1=Readonly<{size:bigint;mtimeNs:bigint;ino:bigint;link?:string}>;
export type ManifestV1={version:1;open:boolean;entries:Record<string,[string,string|null]>};

/** 🛡️ Reads the policy under its closed schema. */
export function readSnapshotPolicyV1(value:unknown=policy):SnapshotPolicyV1{
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Policy"},value,schema as never);
 if(errors.length)throw Error("snapshot-policy-invalid:"+JSON.stringify(errors[0]));
 return value as SnapshotPolicyV1;
}

/** ✅️ Checks a receipt against its closed schema; the empty list means valid. */
export function validateSnapshotReceiptV1(value:unknown):readonly string[]{
 return validateJsonSchemaSubset({$ref:"#/definitions/Receipt"},value,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

/** 🌍️ The languages of one notice: the chosen one, else both (no default language). */
export const snapshotLanguagesV1=(environment:NodeJS.ProcessEnv):readonly ("en"|"de")[]=>environment.SEMIO_LANGUAGE==="de"?["de"]:environment.SEMIO_LANGUAGE==="en"?["en"]:["en","de"];

/** 🎯️ The mode a command line runs in, or nothing when it runs live. `SEMIO_SOURCE_SNAPSHOT_MODE`: `off` runs live; `digest` (the default) only reports the digest of the persistent snapshot when this machine has one (no first full capture is ever paid for it); `on` runs the commands of the policy on their snapshot (`root` mode: the command in a child whose checkout is the pinned lane root; `digest` commands stay in this process) and builds the persistent snapshot when it is missing. Never a command started inside a snapshot run. */
export function snapshotDueV1(operation:readonly string[],environment:NodeJS.ProcessEnv,limits:SnapshotPolicyV1=rules):SnapshotModeV1|undefined{
 const names=limits.environment,choice=environment[names.mode];
 if(environment[names.marker]!==undefined||choice==="off"||choice==="0"||choice==="false")return undefined;
 if(operation.some(word=>word==="--help"||word==="-h"))return undefined;
 const mode=(limits.commands as Record<string,SnapshotModeV1>)[operation[0]??""];
 if(mode===undefined)return undefined;
 return choice==="on"?mode:"digest";
}

export type SnapshotStatV1=StatV1;
/** 🪪️ The identity string of one file: size, modification time (ns), inode and link target. */
export const snapshotFingerprintV1=(stat:StatV1):string=>`${stat.size}:${stat.mtimeNs}:${stat.ino}:${stat.link??""}`;
/** 🔬️ Reads the identity of one file or link without following it; nothing when it is gone. */
export const snapshotStatV1=(path:string):StatV1|undefined=>{
 try{const stat=lstatSync(path,{bigint:true}) as BigIntStats;return {size:stat.size,mtimeNs:stat.mtimeNs,ino:stat.ino,...stat.isSymbolicLink()?{link:readlinkSync(path)}:{}};}catch{return undefined;}
};

/** 🔭️ Lists the source closure of a checkout with the identity of every file (size, modification time, inode, link target): the declared roots, the members of the Rust roots except the heavy ones the workspace links, and the files of the root; the excluded folders never enter. */
export function snapshotEnumerateV1(root:string,heavyMembers:readonly string[],limits:SnapshotPolicyV1=rules):ReadonlyMap<string,StatV1>{
 const found=new Map<string,StatV1>(),excluded=new Set(limits.excludedSegments);
 const walk=(relativePath:string):void=>{
  const stack=[relativePath];
  while(stack.length){
   const directory=stack.pop()!;let entries;
   try{entries=readdirSync(join(root,directory),{withFileTypes:true});}catch{continue;}
   for(const entry of entries){
    if(excluded.has(entry.name))continue;
    const path=directory===""?entry.name:directory+"/"+entry.name;
    if(entry.isDirectory())stack.push(path);
    else{const stat=snapshotStatV1(join(root,path));if(stat!==undefined)found.set(path,stat);}
   }
  }
 };
 for(const name of limits.roots)walk(name);
 for(const name of limits.rustRoots){
  let children;try{children=readdirSync(join(root,name),{withFileTypes:true});}catch{continue;}
  for(const child of children){
   if(heavyMembers.includes(child.name)||excluded.has(child.name))continue;
   const path=name+"/"+child.name;
   if(child.isDirectory())walk(path);else{const stat=snapshotStatV1(join(root,path));if(stat!==undefined)found.set(path,stat);}
  }
 }
 const top=(()=>{try{return readdirSync(root,{withFileTypes:true});}catch{return [];}})();
 for(const entry of top)if(entry.isFile()){const stat=snapshotStatV1(join(root,entry.name));if(stat!==undefined)found.set(entry.name,stat);}
 return found;
}

const sameStat=(left:BigIntStats,right:BigIntStats):boolean=>left.size===right.size&&left.mtimeNs===right.mtimeNs&&left.ino===right.ino;

/** 🪞️ Copies one file of the live tree into the snapshot, atomically (a hidden temporary file, then a rename) and only when the file did not move while it was copied; a copy-on-write clone where the file system can. Returns the identity of what was copied, `null` when the file is gone and `undefined` when it kept moving. */
export function snapshotCopyFileV1(liveRoot:string,snapshotRoot:string,path:string,retries:number,afterCopy?:(path:string)=>void):string|null|undefined{
 const from=join(liveRoot,path),to=join(snapshotRoot,path);
 for(let attempt=0;attempt<retries;attempt++){
  let before:BigIntStats;
  try{before=lstatSync(from,{bigint:true}) as BigIntStats;}catch{return null;}
  mkdirSync(dirname(to),{recursive:true});
  if(before.isSymbolicLink()){
   const target=readlinkSync(from),inside=isAbsolute(target)&&!relative(liveRoot,target).startsWith("..")&&!isAbsolute(relative(liveRoot,target));
   rmSync(to,{force:true});symlinkSync(inside?join(snapshotRoot,relative(liveRoot,target)):target,to);
   return snapshotFingerprintV1({size:before.size,mtimeNs:before.mtimeNs,ino:before.ino,link:target});
  }
  const temporary=`${to}.snapshot-${process.pid}`;
  try{copyFileSync(from,temporary,constants.COPYFILE_FICLONE);}catch{rmSync(temporary,{force:true});continue;}
  afterCopy?.(path);
  let after:BigIntStats;
  try{after=lstatSync(from,{bigint:true}) as BigIntStats;}catch{rmSync(temporary,{force:true});return null;}
  if(sameStat(before,after)){renameSync(temporary,to);return snapshotFingerprintV1({size:after.size,mtimeNs:after.mtimeNs,ino:after.ino});}
  rmSync(temporary,{force:true});
 }
 return undefined;
}

export const snapshotWriteAtomicV1=(path:string,text:string):void=>{mkdirSync(dirname(path),{recursive:true});const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,text);renameSync(temporary,path);};
const writeAtomic=(path:string,text:string):void=>{mkdirSync(dirname(path),{recursive:true});const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,text);renameSync(temporary,path);};
export const snapshotLaneNameV1=(lane:string):string=>lane.replace(/[^A-Za-z0-9._-]+/gu,"-").replace(/^[^A-Za-z0-9]+/u,"")||"lane";
const laneName=(lane:string):string=>lane.replace(/[^A-Za-z0-9._-]+/gu,"-").replace(/^[^A-Za-z0-9]+/u,"")||"lane";
export const snapshotAliveV1=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🗂️ The short folder that holds the snapshots of one checkout (below the root of the `repo land` overlays, so a socket path in it stays short): `<overlay base>/s`. */
export async function snapshotBaseV1(root:string,environment:NodeJS.ProcessEnv=process.env,limits:SnapshotPolicyV1=rules):Promise<string>{
 const {landOverlayBaseV1}=await import("../🚉️land/🟦️.ts");
 return join(landOverlayBaseV1(root,environment),limits.storage.directory);
}

/** 🔒️ Takes the snapshot folder of a lane for one command; a lane another live command holds yields a folder of its own (`<lane>-<pid>`), a lock of a dead process is taken over. */
export function snapshotTakeLaneV1(base:string,lane:string,limits:SnapshotPolicyV1=rules,pid:number=process.pid):Readonly<{lane:string;busy:boolean;release:()=>void}>{
 mkdirSync(base,{recursive:true});
 const attempt=(name:string):boolean=>{
  const lock=join(base,name+limits.storage.lockSuffix);
  try{mkdirSync(lock);writeFileSync(join(lock,"owner"),String(pid));return true;}catch{
   let owner=NaN;try{owner=Number(readFileSync(join(lock,"owner"),"utf8"));}catch{}
   if(Number.isInteger(owner)&&owner!==pid&&alive(owner))return false;
   rmSync(lock,{recursive:true,force:true});
   try{mkdirSync(lock);writeFileSync(join(lock,"owner"),String(pid));return true;}catch{return false;}
  }
 };
 const first=laneName(lane);
 let name=first,busy=false;
 if(!attempt(first)){busy=true;name=`${first}-${pid}`;attempt(name);}
 return {lane:name,busy,release:()=>rmSync(join(base,name+limits.storage.lockSuffix),{recursive:true,force:true})};
}

/** 🧹️ Removes the snapshots nobody holds: older than the declared hours, then the oldest beyond the declared number of lanes. */
export function snapshotPruneV1(base:string,nowMs:number,limits:SnapshotPolicyV1=rules):readonly string[]{
 const gone:string[]=[];let names:string[];
 try{names=readdirSync(base).filter(name=>!name.endsWith(limits.storage.lockSuffix)&&!name.endsWith(".json")&&statSync(join(base,name)).isDirectory());}catch{return gone;}
 const rows=names.filter(name=>!existsSync(join(base,name+limits.storage.lockSuffix))).map(name=>({name,at:statSync(join(base,name+"."+limits.storage.manifestFile),{throwIfNoEntry:false})?.mtimeMs??0})).sort((a,b)=>a.at-b.at);
 const surplus=Math.max(0,names.length-limits.storage.maximumLanes);
 rows.forEach((row,index)=>{
  if(nowMs-row.at>limits.storage.keepHours*3600_000||index<surplus){rmSync(join(base,row.name),{recursive:true,force:true});rmSync(join(base,row.name+"."+limits.storage.manifestFile),{force:true});gone.push(row.name);}
 });
 return gone;
}

export const snapshotReadManifestV1=(path:string):ManifestV1=>readManifest(path);
const readManifest=(path:string):ManifestV1=>{
 try{const value=JSON.parse(readFileSync(path,"utf8")) as ManifestV1;if(value.version===1&&!value.open&&typeof value.entries==="object")return value;}catch{}
 return {version:1,open:false,entries:{}};
};

/** #️⃣ The digest of a snapshot: SHA-256 over the sorted `path NUL content-hash` lines, so two machines with the same sources report the same digest whatever the modification times. */
export function snapshotDigestV1(hashes:ReadonlyMap<string,string>):string{
 const hash=createHash("sha256");
 for(const path of [...hashes.keys()].sort((a,b)=>a<b?-1:a>b?1:0))hash.update(`${path}\0${hashes.get(path)}\n`);
 return "sha256:"+hash.digest("hex");
}

/** #️⃣ The content hash of one snapshot file, location independent: the SHA-256 of its bytes, for a link the target with the live root replaced by a token (so a digest does not depend on where a snapshot lives). */
export function snapshotHashV1(directory:string,path:string,liveRoot:string,print:string):string{
 const link=print.split(":").slice(3).join(":");
 if(link!==""){const inside=isAbsolute(link)&&!relative(liveRoot,link).startsWith("..")&&!isAbsolute(relative(liveRoot,link));return "link:"+(inside?"@root/"+relative(liveRoot,link).split("\\").join("/"):link);}
 return createHash("sha256").update(readFileSync(join(directory,path))).digest("hex");
}

/** 🔗️ The target a link of a snapshot must have in a tree below `to`: a link that pointed into the live root points into that tree, any other keeps its target. Takes the hash of a link entry (`link:@root/…`). */
export const snapshotLinkTargetV1=(hash:string,to:string):string|undefined=>hash.startsWith("link:@root/")?join(to,hash.slice("link:@root/".length)):undefined;

/** 📸️ Captures the source closure of a checkout as a consistent snapshot: files are copied one by one, atomically and only when they did not move during the copy; the closure is listed again and whatever moved is copied again until one listing equals the copy (one moment) or the declared rounds end (then the cut is reported inconsistent with examples). A snapshot of the same lane is refreshed in place, so only changed files are copied and the compiler's incremental state survives. */
export async function snapshotCaptureV1(request:Readonly<{root:string;lane:string;mode?:SnapshotModeV1}>,ports:SnapshotPortsV1={},limits:SnapshotPolicyV1=rules):Promise<SnapshotReceiptV1>{
 const now=ports.now??Date.now,sleep=ports.sleep??(milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))),began=now(),root=resolve(request.root),base=ports.base??await snapshotBaseV1(root);
 const directory=join(base,request.lane),manifestPath=join(base,request.lane+"."+limits.storage.manifestFile);
 const heavy=ports.heavyMembers??await heavyMembersV1();
 let manifest=readManifest(manifestPath);
 if(!manifest.entries||!existsSync(directory))manifest={version:1,open:false,entries:{}};
 if(Object.keys(manifest.entries).length===0)rmSync(directory,{recursive:true,force:true});
 writeAtomic(manifestPath,JSON.stringify({version:1,open:true,entries:{}}));
 mkdirSync(directory,{recursive:true});
 const applied=new Map<string,string>(Object.entries(manifest.entries).map(([path,row])=>[path,row[0]])),hashes=new Map<string,string>(Object.entries(manifest.entries).filter(([,row])=>row[1]!==null).map(([path,row])=>[path,row[1]!]));
  let copied=0,removed=0,rounds=0,consistent=false,moved:string[]=[];
 for(let round=1;round<=limits.capture.maximumRounds;round++){
  if(ports.signal?.aborted)throw Error("snapshot-cancelled");
  ports.beforeRound?.(round);rounds=round;
  const live=snapshotEnumerateV1(root,heavy,limits),todo=[...live].filter(([path,stat])=>applied.get(path)!==snapshotFingerprintV1(stat)).map(([path])=>path),gone=[...applied.keys()].filter(path=>!live.has(path));
  if(!todo.length&&!gone.length){consistent=true;moved=[];break;}
  for(const path of gone){rmSync(join(directory,path),{force:true});applied.delete(path);hashes.delete(path);removed++;}
  for(const path of todo){
   const result=snapshotCopyFileV1(root,directory,path,limits.capture.fileRetries,ports.afterCopy);
   if(result===null){applied.delete(path);hashes.delete(path);}
   else if(result===undefined){applied.delete(path);hashes.delete(path);}
   else{applied.set(path,result);hashes.delete(path);copied++;}
  }
  moved=todo;ports.changed?.(todo,gone);
  if(round<limits.capture.maximumRounds)await sleep(limits.capture.backoffMilliseconds);
 }
 const entries:Record<string,[string,string|null]>={};
 for(const [path,print] of applied){
  let hash=hashes.get(path);
  if(hash===undefined){
   hash=snapshotHashV1(directory,path,root,print);
  hashes.set(path,hash);
  }
  entries[path]=[print,hash];
 }
 const digest=snapshotDigestV1(hashes);
 await (ports.link??defaultLinkV1)(root,directory);
 writeAtomic(manifestPath,JSON.stringify({version:1,open:false,entries}));
 const receipt:SnapshotReceiptV1={version:1,digest,files:applied.size,copied,removed,rounds,consistent,...consistent?{}:{moved:moved.slice(0,limits.capture.movedExamples)},seconds:Math.round((now()-began)/10)/100,liveRoot:root,snapshotRoot:directory,mode:request.mode??"root"};
 return Object.freeze(receipt);
}

const heavyMembersV1=async():Promise<readonly string[]>=>{
 const {readLandPolicyV1}=await import("../🚉️land/🟦️.ts");
 return readLandPolicyV1().overlay.heavyMembers;
};

const defaultLinkV1=async(liveRoot:string,snapshotRoot:string):Promise<void>=>{
 const {landLinkWorkspaceV1}=await import("../🚉️land/🔬️gates/🟦️.ts");
 await landLinkWorkspaceV1(liveRoot,snapshotRoot);
};

/** 🧾️ Reads the receipt of the snapshot a command runs on (from the receipt file named in its environment), for result writers that add `sourceSnapshot` to their output; nothing when the command runs live. */
export function snapshotReceiptOfV1(environment:NodeJS.ProcessEnv,limits:SnapshotPolicyV1=rules):SnapshotReceiptV1|undefined{
 const file=environment[limits.environment.receipt];
 if(file===undefined)return undefined;
 try{const value=JSON.parse(readFileSync(file,"utf8"));return validateSnapshotReceiptV1(value).length?undefined:value as SnapshotReceiptV1;}catch{return undefined;}
}

/** 🔤️ The notice lines of a receipt in the chosen languages. */
export function snapshotNoticesV1(receipt:SnapshotReceiptV1,languages:readonly ("en"|"de")[]):readonly string[]{
 const digest=receipt.digest.slice(0,19);
 return languages.map(language=>receipt.consistent?snapshotTextV1(language,"capture.done",{digest,files:receipt.files,copied:receipt.copied,removed:receipt.removed,rounds:receipt.rounds,seconds:receipt.seconds}):snapshotTextV1(language,"capture.inconsistent",{digest,moved:receipt.moved?.length??0,first:receipt.moved?.[0]??"-"}));
}

/** 🧭️ What one command run did: it ran in a child on the snapshot (with its exit code) or it continues in this process. */
export type SnapshotRunV1=Readonly<{kind:"spawned";code:number;receipt:SnapshotReceiptV1}>|Readonly<{kind:"inline";receipt?:SnapshotReceiptV1}>;

/** ▶️ Runs one command on a snapshot: captures it (a failure is announced and the command runs live), then `root` mode starts the same command in a child whose checkout is the snapshot, with the live root, the digest and the receipt file in its environment (it neither captures again nor restarts on a peer edit: nothing it reads moves), forwards cancellation, and prints the digest of the result at the end; `digest` mode keeps the command in this process and only binds it to the digest. */
export async function snapshotRunV1(request:Readonly<{root:string;cwd:string;operation:readonly string[];mode:SnapshotModeV1;entry:string;arguments:readonly string[];environment:NodeJS.ProcessEnv;lane?:string;extraPaths?:readonly string[]}>,ports:SnapshotAdvancePortsV1={},limits:SnapshotPolicyV1=rules):Promise<SnapshotRunV1>{
 const notice=ports.notice??(line=>{process.stderr.write(line+"\n");}),languages=snapshotLanguagesV1(request.environment),names=limits.environment,environment=request.environment;
 let base:string,taken:ReturnType<typeof snapshotTakeLaneV1>,receipt:SnapshotReceiptV1;
 try{
  base=ports.base??await snapshotBaseV1(request.root,environment);
  taken=snapshotTakeLaneV1(base,request.lane??environment[names.agent]??"repo",limits);
 }catch(error){
  for(const language of languages)notice(snapshotTextV1(language,"capture.failed",{reason:String((error as Error).message??error).slice(0,200)}));
  return {kind:"inline"};
 }
 try{
  const {snapshotAcquireV1,snapshotBasePathsV1}=await import("./⏩️advance/🟦️.ts"),{journalMachineV1}=await import("../📜️change-journal/🟦️.ts");
  const store=ports.store??journalMachineV1(request.root,environment)?.store,build=environment[names.mode]==="on";
  if(build&&store!==undefined&&!existsSync(snapshotBasePathsV1(base,limits).state))for(const language of languages)notice(snapshotTextV1(language,"base.building"));
  for(const language of languages)notice(snapshotTextV1(language,"capture.begin",{command:request.operation[0]??""}));
  const got=await snapshotAcquireV1({root:request.root,lane:taken.lane,mode:request.mode,base,build},{...ports,base,...store===undefined?{}:{store},extraPaths:request.extraPaths??[],fresh:environment.SEMIO_SOURCE_SNAPSHOT_FRESH==="1"},limits);
  if(got===undefined){taken.release();if(environment[names.mode]==="digest")for(const language of languages)notice(snapshotTextV1(language,"base.absent"));return {kind:"inline"};}
  receipt=got;
 }catch(error){
  taken.release();
  for(const language of languages)notice(snapshotTextV1(language,"capture.failed",{reason:String((error as Error).message??error).slice(0,200)}));
  return {kind:"inline"};
 }
 const receiptFile=join(base,taken.lane+".receipt.json");
 writeAtomic(receiptFile,JSON.stringify(receipt));
 for(const line of snapshotNoticesV1(receipt,languages))notice(line);
 const bound:NodeJS.ProcessEnv={[names.marker]:receipt.digest,[names.liveRoot]:receipt.liveRoot,[names.receipt]:receiptFile};
 if(request.mode==="digest"){Object.assign(environment,bound);return {kind:"inline",receipt};}
 const cwd=join(receipt.snapshotRoot,relative(request.root,request.cwd)),inside=relative(request.root,request.entry),mapped=!inside.startsWith("..")&&!isAbsolute(inside)?join(receipt.snapshotRoot,inside):request.entry;
 const child=spawn(process.execPath,[...process.execArgv,existsSync(mapped)?mapped:request.entry,...request.arguments],{cwd:existsSync(cwd)?cwd:receipt.snapshotRoot,env:{...environment,...bound,SEMIO_WORKSPACE_ROOT:receipt.snapshotRoot,REPO_ROOT:receipt.snapshotRoot},stdio:"inherit",windowsHide:true});
 const forward=(signal:NodeJS.Signals)=>()=>{child.kill(signal);};
 const handlers:[NodeJS.Signals,()=>void][]=(["SIGINT","SIGTERM"] as NodeJS.Signals[]).map(signal=>[signal,forward(signal)]);
 for(const [signal,handler] of handlers)process.on(signal,handler);
 const code=await new Promise<number>(done=>{child.on("error",()=>done(127));child.on("close",(status,signal)=>done(status??(signal==="SIGINT"?130:143)));});
 for(const [signal,handler] of handlers)process.off(signal,handler);
 taken.release();
 try{snapshotPruneV1(base,Date.now(),limits);}catch{}
 for(const language of languages)notice(snapshotTextV1(language,"run.report",{digest:receipt.digest.slice(0,19),mode:request.mode}));
 return {kind:"spawned",code,receipt};
}
