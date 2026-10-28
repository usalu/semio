import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import{createHash}from"node:crypto";
import{chmodSync,closeSync,fstatSync,fsyncSync,lstatSync,mkdirSync,openSync,readFileSync,readSync,readdirSync,renameSync,rmSync,writeSync}from"node:fs";
import{join,relative,resolve,sep}from"node:path";
import policyData from"./🔣️.json"with{type:"json"};

/** 🏛️ The sealed implementation store: the build tool runs from an immutable, content-addressed snapshot of its own sources, so a half-finished edit of the live tree never reaches a running or a new command. This file holds what every process needs before it can run anything: the closed policy, the mode switch, the store layout, the closed records and the cheap stat fingerprint that tells a fresh snapshot from a drifted one. It imports Node built-ins only (the front door loads it before any workspace dependency exists).
 * @see ./🔣️.json the policy, ./🧬️schema/🔣️.json its schema, ./📤️promotion/🟦️.ts how a drifted tree is promoted, ./📥️execution/🟦️.ts the seam of a command */
export type SealedLanguageV1="en"|"de";
/** 🔀️ The implementation a command runs from: the sealed snapshot, the live tree, or a promotion of the live tree to a new snapshot. */
export type ImplementationModeV1="sealed"|"live"|"promote";
/** 🌍️ One message of the policy in every declared language. */
export type SealedTextV1=Readonly<Record<SealedLanguageV1,string>>;
/** 📐️ The closed policy of the store exactly as its schema declares it: modes, languages, bounds, timings, lock, check and the bilingual texts. */
export interface SealedPolicyV1{readonly version:1;readonly modeVariable:string;readonly defaultMode:"sealed"|"live";readonly modes:readonly ImplementationModeV1[];readonly languages:readonly SealedLanguageV1[];readonly cacheDirectory:string;readonly stampedFiles:readonly string[];readonly keepPrevious:number;readonly quietMilliseconds:number;readonly maximumBytes:number;readonly maximumTextCharacters:number;readonly maximumPathCharacters:number;readonly maximumCommandLineCharacters:number;readonly settleMilliseconds:number;readonly captureAttempts:number;readonly blockedRetryMilliseconds:number;readonly coalesceMilliseconds:number;readonly helpTokens:readonly string[];readonly leadingWords:readonly string[];readonly maximumBlockedRows:number;readonly shownBlockedRows:number;readonly lock:Readonly<{pollMilliseconds:number;busyTimeoutMilliseconds:number;waitMaximumMilliseconds:number}>;readonly light:Readonly<{freshMilliseconds:number;promoteAfterMilliseconds:number;refuseAfterMilliseconds:number;refreshDebounceMilliseconds:number}>;readonly promotion:Readonly<{rule:"explicit"|"drift";agedMilliseconds:number;noticeMilliseconds:number}>;readonly check:Readonly<{argument:string;maximumMilliseconds:number;maximumOutputBytes:number;commands:readonly(readonly string[])[];commandMilliseconds:number}>;readonly readyDescriptor:number;readonly codeLoaders:Readonly<Record<string,"ts"|"tsx"|"js"|"jsx">>;readonly texts:Readonly<Record<string,SealedTextV1|Readonly<Record<string,SealedTextV1>>>>}

/** 📜️ The closed policy of the store. */
export const sealedPolicyV1:SealedPolicyV1=policyData as unknown as SealedPolicyV1;

/** 🧬️ One captured file of the live tree: its repository path, content digest and the stat stamp it had when it was captured. */
export type SealedFileStampV1=readonly[path:string,sha256:string,stamp:string];
/** 🌳️ The facts a snapshot rests on: its files with stamps, the identity of every ancestor directory, the digest of every directory listing the capture depended on and every path it found absent. */
export interface SealedTreeV1{readonly files:readonly SealedFileStampV1[];readonly directories:readonly(readonly[path:string,device:string,inode:string])[];readonly listings:readonly(readonly[path:string,digest:string])[];readonly absent:readonly string[]}
/** 👈️ The pointer to the snapshot an entry currently runs from. */
export interface SealedPointerV1{readonly version:1;readonly entry:string;readonly digest:string;readonly loader:string;readonly bytes:number;readonly files:number;readonly promotedAtMs:number;readonly previous:readonly string[]}
/** 🧮️ What a promotion changed against the snapshot it replaced: the cause of a new implementation digest (a rebuild of the graph must trace back to an edited input or to such a change). */
export interface SealedChangesV1{readonly previous:string|null;readonly added:number;readonly removed:number;readonly changed:number;readonly paths:readonly string[]}
/** 🧾️ What a promotion proved about one snapshot. */
export interface SealedReceiptV1{readonly version:1;readonly entry:string;readonly digest:string;readonly loader:string;readonly createdAtMs:number;readonly changes:SealedChangesV1;readonly validation:Readonly<{syntaxFiles:number;syntaxMilliseconds:number;loadMilliseconds:number|null}>;readonly tree:SealedTreeV1|null}
/** 🚧️ One place that blocks a promotion. */
export interface SealedRowV1{readonly path:string;readonly line:number;readonly column:number;readonly message:string}
/** 🚫️ The record of the last blocked promotion: what blocked it and the fingerprint of the tree that was refused, so the same tree is never captured twice. */
export interface SealedBlockedV1{readonly version:1;readonly entry:string;readonly atMs:number;readonly rows:readonly SealedRowV1[];readonly tree:SealedTreeV1|null}

const hex=/^[a-f0-9]{64}$/u,digits=/^[0-9]+$/u;
/** #️⃣ The SHA-256 of bytes or text as lower-case hexadecimal. */
export const sealedHashV1=(value:Uint8Array|string):string=>createHash("sha256").update(value).digest("hex");
const closed=(value:unknown,keys:readonly string[]):value is Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value)&&Object.keys(value).sort().join(",")===[...keys].sort().join(",");
const text=(value:unknown,maximum=sealedPolicyV1.maximumTextCharacters):value is string=>typeof value==="string"&&value.length>0&&value.length<=maximum;
const count=(value:unknown):value is number=>Number.isSafeInteger(value)&&(value as number)>=0;
const rows=<T>(value:unknown,each:(row:unknown)=>row is T):value is T[]=>Array.isArray(value)&&value.every(each);
const tuple=(size:number)=>(row:unknown):row is unknown[]=>Array.isArray(row)&&row.length===size;

/** 🧱️ Admits a fingerprint record: closed, every row well formed. */
export function sealedTreeOfV1(value:unknown):SealedTreeV1|undefined{
 if(!closed(value,["files","directories","listings","absent"]))return undefined;
 const files=rows(value.files,(row):row is SealedFileStampV1=>tuple(3)(row)&&text((row as unknown[])[0])&&hex.test(String((row as unknown[])[1]))&&text((row as unknown[])[2],200)),directories=rows(value.directories,(row):row is readonly[string,string,string]=>tuple(3)(row)&&typeof (row as unknown[])[0]==="string"&&digits.test(String((row as unknown[])[1]))&&digits.test(String((row as unknown[])[2]))),listings=rows(value.listings,(row):row is readonly[string,string]=>tuple(2)(row)&&text((row as unknown[])[0])&&hex.test(String((row as unknown[])[1]))),absent=rows(value.absent,(row):row is string=>text(row));
 return files&&directories&&listings&&absent?value as unknown as SealedTreeV1:undefined;
}
/** 🪝️ Admits a pointer record. */
export function sealedPointerOfV1(value:unknown):SealedPointerV1|undefined{
 return closed(value,["version","entry","digest","loader","bytes","files","promotedAtMs","previous"])&&value.version===1&&text(value.entry)&&hex.test(String(value.digest))&&hex.test(String(value.loader))&&count(value.bytes)&&value.bytes>0&&count(value.files)&&value.files>0&&count(value.promotedAtMs)&&rows(value.previous,(row):row is string=>hex.test(String(row)))&&value.previous.length<=8?value as unknown as SealedPointerV1:undefined;
}
/** 🪪️ Admits a receipt record. */
export function sealedReceiptOfV1(value:unknown):SealedReceiptV1|undefined{
 if(!closed(value,["version","entry","digest","loader","createdAtMs","changes","validation","tree"])||value.version!==1||!text(value.entry)||!hex.test(String(value.digest))||!hex.test(String(value.loader))||!count(value.createdAtMs)||!closed(value.changes,["previous","added","removed","changed","paths"])||!(value.changes.previous===null||hex.test(String(value.changes.previous)))||!count(value.changes.added)||!count(value.changes.removed)||!count(value.changes.changed)||!rows(value.changes.paths,(row):row is string=>text(row))||value.changes.paths.length>200||!closed(value.validation,["syntaxFiles","syntaxMilliseconds","loadMilliseconds"])||!count(value.validation.syntaxFiles)||!count(value.validation.syntaxMilliseconds)||!(value.validation.loadMilliseconds===null||count(value.validation.loadMilliseconds)))return undefined;
 return value.tree===null||sealedTreeOfV1(value.tree)?value as unknown as SealedReceiptV1:undefined;
}
const rowOf=(row:unknown):row is SealedRowV1=>closed(row,["path","line","column","message"])&&text(row.path,1024)&&count(row.line)&&count(row.column)&&text(row.message,300);
/** 🛑️ Admits a blocked record. */
export function sealedBlockedOfV1(value:unknown):SealedBlockedV1|undefined{
 return closed(value,["version","entry","atMs","rows","tree"])&&value.version===1&&text(value.entry)&&count(value.atMs)&&rows(value.rows,rowOf)&&value.rows.length>0&&(value.tree===null||sealedTreeOfV1(value.tree)!==undefined)?value as unknown as SealedBlockedV1:undefined;
}

/** 🌐️ The languages a message is rendered in: the one the language protocol chooses, both declared languages when it chooses none. */
export function sealedLanguagesV1(environment:Readonly<Record<string,string|undefined>>=process.env,policy:SealedPolicyV1=sealedPolicyV1):readonly SealedLanguageV1[]{
 return noticeLanguagesV1(policy.languages,environment);
}
/** 💬️ Renders one declared text in the selected languages, one block per language; every placeholder must be given. */
export function sealedTextV1(languages:readonly SealedLanguageV1[],key:string,values:Readonly<Record<string,string|number>>={},policy:SealedPolicyV1=sealedPolicyV1):string{
 const entry=key.split(".").reduce<unknown>((table,part)=>(table as Record<string,unknown>|undefined)?.[part],policy.texts) as SealedTextV1|undefined;
 if(!entry||typeof entry.en!=="string"||typeof entry.de!=="string")throw Error("sealed-text-undeclared:"+key);
 return languages.map(language=>entry[language].replace(/\{([A-Za-z]+)\}/gu,(_,name)=>{if(!Object.hasOwn(values,name))throw Error("sealed-text-placeholder-missing:"+name);return String(values[name]);})).join("\n");
}
/** 🎛️ Reads the mode switch: `sealed` runs the stored snapshot (promoting a drifted live tree first), `live` runs the live tree (the opt-in of the agent that edits the tool), `promote` promotes the live tree now and runs nothing; an unknown value is a refusal in both languages. */
export function implementationModeV1(environment:Readonly<Record<string,string|undefined>>=process.env,policy:SealedPolicyV1=sealedPolicyV1):ImplementationModeV1{
 const raw=environment[policy.modeVariable];if(raw===undefined||raw==="")return policy.defaultMode;
 const value=raw.toLowerCase() as ImplementationModeV1;
 if(!policy.modes.includes(value))throw Object.assign(Error(sealedTextV1(sealedLanguagesV1(environment,policy),"mode",{variable:policy.modeVariable,value:raw},policy)),{code:"SEMIO_IMPLEMENTATION_MODE"});
 return value;
}

/** 🗂️ The layout of one entry of the store. */
export interface SealedEntryV1{readonly entry:string;readonly directory:string;readonly pointer:string;readonly blocked:string;readonly checked:string;readonly lock:string;snapshot(digest:string):string;receipt(digest:string):string;loader(digest:string):string}
/** 🔑️ The directory name of an entry: the first 16 hexadecimal digits of the digest of its repository path. */
export const sealedEntryKeyV1=(entry:string):string=>sealedHashV1(entry).slice(0,16);
/** 🗺️ Resolves the store of a workspace (outside every checkout copy: it lives in the repository cache root) and the layout of one entry in it. */
export function sealedEntryOfV1(workspace:string,entry:string,policy:SealedPolicyV1=sealedPolicyV1):SealedEntryV1{
 const directory=join(resolve(workspace),...policy.cacheDirectory.split("/"),sealedEntryKeyV1(entry));
 return Object.freeze({entry,directory,pointer:join(directory,"current.json"),blocked:join(directory,"blocked.json"),checked:join(directory,"checked.json"),lock:join(directory,"lock.sqlite"),snapshot:(digest:string)=>join(directory,digest+".json"),receipt:(digest:string)=>join(directory,digest+".seal.json"),loader:(digest:string)=>join(directory,digest+".loader.js")});
}
/** 📁️ The directory that holds every entry of the store. */
export const sealedStoreDirectoryV1=(workspace:string,policy:SealedPolicyV1=sealedPolicyV1):string=>join(resolve(workspace),...policy.cacheDirectory.split("/"));

const parse=(path:string):unknown=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}};
/** 🧲️ The current pointer of an entry, or nothing when there is none or it is torn or foreign. */
export const readSealedPointerV1=(entry:SealedEntryV1):SealedPointerV1|undefined=>{const value=sealedPointerOfV1(parse(entry.pointer));return value?.entry===entry.entry?value:undefined;};
/** 📄️ The receipt of one snapshot. */
export const readSealedReceiptV1=(entry:SealedEntryV1,digest:string):SealedReceiptV1|undefined=>{const value=sealedReceiptOfV1(parse(entry.receipt(digest)));return value?.digest===digest&&value.entry===entry.entry?value:undefined;};
/** ⛔️ The last blocked promotion of an entry. */
export const readSealedBlockedV1=(entry:SealedEntryV1):SealedBlockedV1|undefined=>{const value=sealedBlockedOfV1(parse(entry.blocked));return value?.entry===entry.entry?value:undefined;};
/** 🔍️ The last stat check of a light command against one snapshot: when it ran, whether the live tree had drifted, and when a detached promotion was last started for it. Light commands read it to skip the check while it is recent. */
export interface SealedCheckedV1{readonly version:1;readonly entry:string;readonly digest:string;readonly atMs:number;readonly drifted:boolean;readonly refreshedAtMs:number|null}
/** 🎟️ Admits a check record. */
export function sealedCheckedOfV1(value:unknown):SealedCheckedV1|undefined{
 return closed(value,["version","entry","digest","atMs","drifted","refreshedAtMs"])&&value.version===1&&text(value.entry)&&hex.test(String(value.digest))&&count(value.atMs)&&typeof value.drifted==="boolean"&&(value.refreshedAtMs===null||count(value.refreshedAtMs))?value as unknown as SealedCheckedV1:undefined;
}
/** ✅️ The check record of one snapshot; a record of another snapshot is no record. */
export const readSealedCheckedV1=(entry:SealedEntryV1,digest:string):SealedCheckedV1|undefined=>{const value=sealedCheckedOfV1(parse(entry.checked));return value?.entry===entry.entry&&value.digest===digest?value:undefined;};
/** 🧳️ Keeps the check record; a store that cannot be written costs the next command one more check and nothing else. */
export function writeSealedCheckedV1(entry:SealedEntryV1,record:SealedCheckedV1):void{try{writeSealedFileV1(entry.checked,JSON.stringify(record));}catch{}}

/** ⏳️ What a light command (help or a declared light operation) does with the sealed snapshot it found. While the snapshot or its last check is younger than `light.freshMilliseconds` it runs it without looking at the tree. Otherwise it asks whether the tree drifted: no drift runs it; a tree already known to be blocked runs it (the promotion would only refuse again); a snapshot older than `light.promoteAfterMilliseconds` is promoted before the command runs; a younger one runs while a detached promotion is started (`start`), at most once per `light.refreshDebounceMilliseconds`. `probed` says that the tree was looked at, so the caller records it.
 * @see ./🧪️tests/🗂️store/🟦️.ts the laws, including the independent model */
export function sealedLightActionV1(input:Readonly<{now:number;promotedAtMs:number;checked:Pick<SealedCheckedV1,"atMs"|"drifted"|"refreshedAtMs">|undefined;drifted:()=>boolean;blocked:()=>boolean}>,policy:SealedPolicyV1=sealedPolicyV1):Readonly<{action:"run"|"promote"|"refresh";probed:boolean;drifted:boolean;start:boolean}>{
 const light=policy.light,age=input.now-input.promotedAtMs,since=input.checked?input.now-input.checked.atMs:Infinity;
 if(Math.min(age,since)<light.freshMilliseconds)return Object.freeze({action:"run",probed:false,drifted:input.checked?.drifted??false,start:false});
 if(!input.drifted())return Object.freeze({action:"run",probed:true,drifted:false,start:false});
 if(input.blocked())return Object.freeze({action:"run",probed:true,drifted:true,start:false});
 if(age>=light.promoteAfterMilliseconds)return Object.freeze({action:"promote",probed:true,drifted:true,start:false});
 const recent=input.checked?.refreshedAtMs!=null&&input.now-input.checked.refreshedAtMs<light.refreshDebounceMilliseconds;
 return Object.freeze({action:"refresh",probed:true,drifted:true,start:!recent});
}
/** 🧷️ What a command does with the snapshot it found when promotion is explicit (`promotion.rule` = `explicit`): it runs it, so a peer's edit of the tool never moves the implementation digest (the key of the graph memo and of the Bazel plan) under a running fleet; the landing gate promotes with a `promote` run after a landing. Only a snapshot older than `promotion.agedMilliseconds`, or a restart after the running implementation yielded to a moved format stamp (it cannot follow the live tree without a new snapshot), promotes a drifted tree that is not known to be blocked on its own (a forgotten promotion cannot run day-old code for ever). The tree is compared at most once per `promotion.noticeMilliseconds` (the shared check record) and a drift that is not promoted is announced once per comparison.
 * @see ./🧪️tests/🗂️store/🟦️.ts the laws, including the independent model, ./🚦️trampoline/🟦️.ts the caller */
export function sealedExplicitActionV1(input:Readonly<{now:number;promotedAtMs:number;checked:Pick<SealedCheckedV1,"atMs"|"drifted">|undefined;restarted:boolean;drifted:()=>boolean;blocked:()=>boolean}>,policy:SealedPolicyV1=sealedPolicyV1):Readonly<{action:"run"|"promote";probed:boolean;drifted:boolean;notice:boolean}>{
 const urgent=input.restarted||input.now-input.promotedAtMs>=policy.promotion.agedMilliseconds,since=input.checked?input.now-input.checked.atMs:Infinity;
 if(!urgent&&since<policy.promotion.noticeMilliseconds)return Object.freeze({action:"run",probed:false,drifted:input.checked?.drifted??false,notice:false});
 if(!input.drifted())return Object.freeze({action:"run",probed:true,drifted:false,notice:false});
 const promote=urgent&&!input.blocked();
 return Object.freeze({action:promote?"promote":"run",probed:true,drifted:true,notice:!promote});
}
/** 📖️ The text of a stored snapshot when the file is there, as long as its pointer says and hashes to its name; anything else is no snapshot. */
export function readSealedSnapshotV1(entry:SealedEntryV1,pointer:SealedPointerV1):string|undefined{
 try{const value=readFileSync(entry.snapshot(pointer.digest),"utf8");return Buffer.byteLength(value)===pointer.bytes&&sealedHashV1(value)===pointer.digest?value:undefined;}catch{return undefined;}
}

const pause=(milliseconds:number)=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};
/** ⚛️ Writes a file so that every reader sees all of it or none of it: a temporary file in the same directory is flushed and renamed over the name (a rename onto a file another process is reading is retried on the platforms that refuse it). */
export function writeSealedFileV1(path:string,data:string|Uint8Array,mode=0o644):void{
 mkdirSync(join(path,".."),{recursive:true});
 const temporary=path+"."+process.pid+"."+Math.random().toString(16).slice(2)+".tmp",descriptor=openSync(temporary,"wx",mode);
 try{const bytes=typeof data==="string"?Buffer.from(data):data;for(let offset=0;offset<bytes.length;)offset+=writeSync(descriptor,bytes,offset,bytes.length-offset);fsyncSync(descriptor);}finally{closeSync(descriptor);}
 for(let attempt=1;;attempt++){try{renameSync(temporary,path);break;}catch(error){const code=(error as NodeJS.ErrnoException).code;if(attempt>=8||code!=="EPERM"&&code!=="EBUSY"&&code!=="EACCES"){rmSync(temporary,{force:true});throw error;}pause(20*attempt);}}
 try{chmodSync(path,mode);}catch{}
}

/** 🔓️ Reads a file of the live tree from the disk. A process that runs from a snapshot answers `readFileSync` of any path the snapshot holds from the snapshot, so a check of the live tree must not use it: this reads through a descriptor, which no snapshot intercepts. */
export function readLiveFileV1(path:string):Buffer{
 const descriptor=openSync(path,"r");
 try{const size=fstatSync(descriptor).size,bytes=Buffer.allocUnsafe(size);let offset=0;while(offset<size){const count=readSync(descriptor,bytes,offset,size-offset,offset);if(!count)break;offset+=count;}return bytes.subarray(0,offset);}finally{closeSync(descriptor);}
}
const stampOf=(row:{dev:bigint;ino:bigint;size:bigint;nlink:bigint;mtimeNs:bigint;ctimeNs:bigint}):string=>[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(":");
const absolute=(root:string,path:string):string=>join(root,...path.split("/"));
const facts=(path:string)=>{try{return lstatSync(path,{bigint:true});}catch{return undefined;}};
/** 📚️ The digest of the entries of a directory (name and kind, sorted by bytes), the same for a listing a capture remembered and for one read now; a link or a special file makes it unequal to everything. */
export function sealedListingDigestV1(entries:readonly(readonly[string,"d"|"f"])[]):string{return sealedHashV1(JSON.stringify(entries));}
function listedV1(path:string):string|undefined{
 try{
  const found:(readonly[string,"d"|"f"])[]=[];
  for(const row of readdirSync(path,{withFileTypes:true})){if(row.isSymbolicLink())return undefined;if(row.isDirectory())found.push([row.name,"d"]);else if(row.isFile())found.push([row.name,"f"]);else return undefined;}
  found.sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])));
  return sealedListingDigestV1(found);
 }catch{return undefined;}
}
/** 📢️ Tells whether the live tree still is the tree a snapshot was captured from, without reading a file that did not change: every file keeps its stat stamp (a changed stamp is settled by hashing that one file, so a touch or a restored checkout is no drift), every ancestor directory keeps its identity, every remembered listing and every remembered absence holds. */
export function sealedTreeDriftV1(workspace:string,tree:SealedTreeV1):Readonly<{fresh:boolean;changed:readonly string[]}>{
 const root=resolve(workspace),changed:string[]=[];
 for(const[path,sha256,recorded] of tree.files){
  const row=facts(absolute(root,path));if(!row||!row.isFile()){changed.push(path);continue;}
  if(stampOf(row)===recorded)continue;
  try{if(sealedHashV1(readLiveFileV1(absolute(root,path)))!==sha256)changed.push(path);}catch{changed.push(path);}
 }
 for(const[path,device,inode] of tree.directories){const row=facts(path===""?root:absolute(root,path));if(!row||!row.isDirectory()||String(row.dev)!==device||String(row.ino)!==inode)changed.push(path===""?".":path);}
 for(const[path,digest] of tree.listings)if(listedV1(absolute(root,path))!==digest)changed.push(path);
 for(const path of tree.absent)if(facts(absolute(root,path)))changed.push(path);
 return Object.freeze({fresh:changed.length===0,changed:Object.freeze(changed)});
}
/** 📸️ What a capture remembers beyond the bytes of its files: the directories it passed, the listings it read and the absences it found. */
export interface SealedStructureV1{readonly directories:readonly Readonly<{path:string;dev:string;ino:string}>[];readonly observations:readonly(Readonly<{kind:"absent";path:string}>|Readonly<{kind:"directory";path:string;entries:readonly(readonly[string,"d"|"f"])[]}>|Readonly<{kind:"package"}>)[]}
/** 📏️ Fingerprints the live closure of a capture right now: the stamp of every captured file and of the declared stamped files (a lock file moves installed packages), plus the structure the capture remembered. The sha-256 of a file is the one the capture recorded, never a second read. */
export function sealedTreeOfCaptureV1(workspace:string,captured:readonly Readonly<{path:string;sha256:string}>[],structure:SealedStructureV1|undefined,policy:SealedPolicyV1=sealedPolicyV1):SealedTreeV1|undefined{
 if(!structure)return undefined;
 const root=resolve(workspace),files:SealedFileStampV1[]=[];
 for(const row of captured){const found=facts(absolute(root,row.path));if(!found||!found.isFile())return undefined;files.push([row.path,row.sha256,stampOf(found)]);}
 for(const path of policy.stampedFiles){const found=facts(absolute(root,path));if(found?.isFile())try{files.push([path,sealedHashV1(readLiveFileV1(absolute(root,path))),stampOf(found)]);}catch{return undefined;}}
 const listings:(readonly[string,string])[]=[],absent:string[]=[];
 for(const row of structure.observations){if(row.kind==="directory")listings.push([row.path,sealedListingDigestV1(row.entries)]);else if(row.kind==="absent")absent.push(row.path);}
 return Object.freeze({files,directories:structure.directories.map(row=>[row.path,row.dev,row.ino] as const),listings,absent});
}
/** 🕰️ The newest modification or change time of the files of a fingerprint, in milliseconds; a tree edited within the settle window may still be mid-edit. */
export function sealedNewestChangeV1(workspace:string,tree:SealedTreeV1):number{
 const root=resolve(workspace);let newest=0;
 for(const[path] of tree.files){const row=facts(absolute(root,path));if(row)newest=Math.max(newest,Number((row.mtimeNs>row.ctimeNs?row.mtimeNs:row.ctimeNs)/1000000n));}
 return newest;
}
/** 🔎️ Repository-relative form of a path for a notice: inside the workspace it is relative with `/`, outside it stays as it is. */
export function sealedRelativeV1(workspace:string,path:string):string{const inside=relative(resolve(workspace),path);return inside&&!inside.startsWith("..")&&!inside.includes(":")?inside.split(sep).join("/"):path;}

/** ⚖️ Compares the files of two snapshots by content: which paths were added, removed or changed (bounded list, exact counts). */
export function sealedChangesV1(previous:Readonly<{digest:string;files:readonly(readonly[string,string])[]}>|undefined,files:readonly(readonly[string,string])[],maximum=200):SealedChangesV1{
 const before=new Map(previous?.files??[]),after=new Map(files),paths:string[]=[];let added=0,removed=0,changed=0;
 for(const[path,sha256] of after){const old=before.get(path);if(old===undefined){added++;paths.push(path);}else if(old!==sha256){changed++;paths.push(path);}}
 for(const[path] of before)if(!after.has(path)){removed++;paths.push(path);}
 return Object.freeze({previous:previous?.digest??null,added,removed,changed,paths:Object.freeze(paths.sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))).slice(0,maximum))});
}

/** 🍞️ The loader text a pointer's snapshot runs under (the loader the candidate was proven with: the snapshot checks its own loader identity), when the stored file hashes to its name. */
export function readSealedLoaderV1(entry:SealedEntryV1,pointer:SealedPointerV1):string|undefined{
 try{const value=readFileSync(entry.loader(pointer.loader),"utf8");return sealedHashV1(value)===pointer.loader?value:undefined;}catch{return undefined;}
}

/** 🏠️ Finds the workspace root from a directory inside it: the nearest ancestor that holds the paired build marker and a package manifest with workspaces (the same rule as the front door's; a law compares the two). */
export function findSealedRootV1(start:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 let directory=resolve(start.trim()||environment.SEMIO_WORKSPACE_ROOT?.trim()||environment.REPO_ROOT?.trim()||process.cwd());
 for(let depth=0;depth<32;depth++){
  if(["📋️project.json","package.json"].every(name=>lstatSync(join(directory,name),{throwIfNoEntry:false})?.isFile())){
   const{workspaces}=JSON.parse(readFileSync(join(directory,"package.json"),"utf8"));
   if(Array.isArray(workspaces)||workspaces!==null&&typeof workspaces==="object")return directory;
  }
  const parent=resolve(directory,"..");if(parent===directory)break;directory=parent;
 }
 throw Error(sealedTextV1(sealedLanguagesV1(),"workspaceRootMissing"));
}

/** 🔖️ The same fingerprint with the stat stamps of the live files as they are now, and the digests of the snapshot kept: a tree is "unchanged since it was refused" while no file changes again, and any later edit of a file (other than back to the snapshot's own bytes) is a change. A file that is gone keeps its old stamp, so it counts as a change. */
export function sealedRestampV1(workspace:string,tree:SealedTreeV1):SealedTreeV1{
 const root=resolve(workspace);
 return Object.freeze({...tree,files:tree.files.map(([path,sha256,stamp])=>{const found=facts(absolute(root,path));return[path,sha256,found&&found.isFile()?stampOf(found):stamp] as const;})});
}

/** 💡️ Whether a command line is one the front door answers without the build implementation (a help request or one of its declared light operations): such a command runs the stored snapshot as it is and never waits for a promotion. */
export function sealedLightCommandV1(argv:readonly string[],operations:readonly string[],policy:SealedPolicyV1=sealedPolicyV1):boolean{
 if(argv.some(token=>policy.helpTokens.includes(token)))return true;
 const words=argv.filter(token=>!token.startsWith("-")),first=words.find(token=>!policy.leadingWords.includes(token));
 return first!==undefined&&operations.includes(first);
}
