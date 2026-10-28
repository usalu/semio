import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,posix} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {formatChangeCursorV1,parseChangeCursorV1,type ChangeRowV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {runtimeIdentityV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {GENERATION_POLICY_V1,frontierFactHoldsV1} from "../../../🕸️graph/🕰️generations/🟨️.mjs";
import {GRAPH_MEMO_RETENTION_V1,formatGraphMemoCollectionV1,planGraphMemoCollectionV1,type GraphMemoCollectionV1,type GraphMemoFileV1,type GraphMemoRecordV1} from "./🧹️retention/🟦️.ts";

/** 🧠️ The declared constants of the graph memo; the schema file is their single source. */
export const GRAPH_MEMO_POLICY_V1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as Readonly<{
 version:1;directory:string;switch:string;sidecar:string;expect:string;requestedAt:string;recordSuffix:string;planSuffix:string;relevanceSuffix:string;recheckSuffix:string;stampSuffix:string;editSuffix:string;maximumRecordBytes:number;maximumPlanBytes:number;maximumRelevanceBytes:number;maximumRecheckBytes:number;maximumStampBytes:number;maximumEditBytes:number;maximumEditPaths:number;maximumEditRows:number;maximumRows:number;leafSymlinkCheckConcurrency:number;
 argumentsDenied:readonly string[];stamped:readonly string[];following:readonly string[];environmentPrefixes:readonly string[];environmentDenied:readonly string[];relevance:Readonly<Record<string,"path"|"analysis"|"link"|"resolve"|"text"|"none">>;texts:Readonly<Record<string,Readonly<Record<"served"|"refused"|"carried"|"edited",string>>>>;carry:Readonly<{shared:readonly string[]}>;
}>);
/** 📝️ One fact row of a graph generation: what a computation read and what it saw. */
export type MemoRowV1=Readonly<{kind:string;path:string;fact:unknown}>;
/** 🪪️ The recorded identity of a source an analysis read: the stat image the fact carries. */
export type MemoIdentityV1=Readonly<{path:string;size:number;mtimeMs:number;ctimeMs:number;ino:number}>;
/** 🔁️ The facts a lookup must check itself because the journal cannot see what they read. */
export type MemoRecheckV1=Readonly<{rows:readonly MemoRowV1[];identities:readonly MemoIdentityV1[]}>;
/** 🚫️ Why a memo could not serve (or be sealed). */
export type MemoReasonV1=typeof schema.definitions.Reason.enum[number];
/** 🎯️ What a lookup answers: the planned slice and the pin of this invocation, or the reason it is planned again. */
export type MemoAnswerV1=Readonly<{hit:true;plan:string;pin:string|undefined;projects:number;ageMilliseconds:number;rows:number;rechecked:number;carried?:Readonly<{from:number;to:number}>;edited?:Readonly<{paths:number;rows:number}>}>|Readonly<{hit:false;reason:MemoReasonV1;detail?:string}>;

const sha256=(data:string|Uint8Array):string=>createHash("sha256").update(data).digest("hex");
const fold=(path:string):string=>/^[\u0000-\u007f]*$/u.test(path)?path.toLowerCase():path.normalize("NFC").toLowerCase();
const isSha=(value:unknown):value is string=>typeof value==="string"&&/^[0-9a-f]{64}$/u.test(value);

/** 🎚️ The switch `SEMIO_BUILD_GRAPH_MEMO=on|off` (default on); anything else is refused in both languages. */
export function graphMemoEnabledV1(environment:Readonly<Record<string,string|undefined>>=process.env):boolean{
 const raw=environment[GRAPH_MEMO_POLICY_V1.switch]??"on",value=raw.toLowerCase();
 if(value==="on")return true;
 if(value==="off")return false;
 throw Error(`Expected ${GRAPH_MEMO_POLICY_V1.switch}=on|off, got "${raw}" / Erwartet ${GRAPH_MEMO_POLICY_V1.switch}=on|off, erhalten "${raw}"`);
}

/** 🗣️ Renders one memo text in a language of the table; an unknown language reads the English one. */
export function renderGraphMemoTextV1(key:"served"|"refused"|"carried"|"edited",fields:Readonly<Record<string,string|number>>,language:string):string{
 const table=GRAPH_MEMO_POLICY_V1.texts[language]??GRAPH_MEMO_POLICY_V1.texts.en!;
 return table[key].replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));
}

/** ✂️ The arguments of a request without the options whose values are execution settings (`--name=value` and `--name value`), which the admission sets per run. */
export function memoArgumentsV1(args:readonly string[],denied:readonly string[]=GRAPH_MEMO_POLICY_V1.argumentsDenied):readonly string[]{
 const kept:string[]=[];
 for(let index=0;index<args.length;index++){const name=args[index]!.split("=",1)[0]!;if(!denied.includes(name)){kept.push(args[index]!);continue;}if(!args[index]!.includes("=")&&/^\d+$/u.test(args[index+1]??""))index++;}
 return kept;
}

/** 🔑️ The identity of one request: what is asked, by which implementation, in which environment. Anything that can change the planned slice is a member; volatile process state (pins, pids, plan files, the granted job count) is not. */
export function graphMemoKeyV1(parts:Readonly<{root:string;args:readonly string[];tool:string|undefined;mode:string|undefined;implementation:string;environment:Readonly<Record<string,string|undefined>>}>):string{
 const policy=GRAPH_MEMO_POLICY_V1,names=Object.keys(parts.environment).filter(name=>policy.environmentPrefixes.some(prefix=>name.startsWith(prefix))&&!policy.environmentDenied.includes(name)).sort();
 return sha256(JSON.stringify({version:policy.version,root:parts.root,args:memoArgumentsV1(parts.args,policy.argumentsDenied),tool:parts.tool??null,mode:parts.mode??null,implementation:parts.implementation,runtime:[process.platform,process.arch,runtimeIdentityV1().version],environment:names.map(name=>[name,parts.environment[name]??null])}));
}

/** 🧭️ The workspace-relative form of a path a fact names: relative paths stay, absolute paths inside the root are cut, every other absolute path is `undefined`. Paths already in normal form take the fast road. */
export function memoRelativeV1(root:string,path:string):string|undefined{
 const base=root.replaceAll("\\","/").replace(/\/+$/u,"");
 if(path.startsWith(base+"/")&&!path.includes("\\")&&!/(?:^|\/)\.{1,2}(?:\/|$)|\/\/|\/$/u.test(path))return path.slice(base.length+1);
 const text=path.replaceAll("\\","/");
 if(!text.startsWith("/")&&!/^[A-Za-z]:\//u.test(text)){const clean=posix.normalize(text);return clean==="."?"":clean.replace(/^\.\//u,"");}
 if(text===base)return "";
 return text.startsWith(base+"/")?posix.normalize(text.slice(base.length+1)):undefined;
}

/** 🛑️ The journal's own exclusion rule as a cached predicate: a workspace-relative path is excluded when it equals or lies below a rooted exclusion or has an excluded name anywhere (the rule of `changeRootExcludedV1` and the client's `excluder`, which a law keeps equal). */
export function memoExcluderV1(exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>,insensitive:boolean):(relative:string)=>boolean{
 const key=insensitive?fold:(value:string)=>value,rooted=new Set(exclude.rooted.map(key)),names=new Set(exclude.names.map(key)),known=new Map<string,boolean>([["",false]]);
 const check=(path:string):boolean=>{
  const cached=known.get(path);if(cached!==undefined)return cached;
  const cut=path.lastIndexOf("/"),leaf=path.slice(cut+1),parent=cut<0?"":path.slice(0,cut),answer=check(parent)||names.has(key(leaf))||rooted.has(key(path));
  known.set(path,answer);return answer;
 };
 return path=>check(path.replace(/^\/+|\/+$/gu,""));
}

/** 🔎️ What reading one fact row means for the journal: the workspace-relative entries it reads and whether anything it reads lies out of the journal's sight (outside the root, an excluded region, a symbolic link by nature, a bare module request). `undefined` for a kind the table does not name. */
export function memoReachV1(row:MemoRowV1,root:string,excluded:(relative:string)=>boolean):Readonly<{mentions:readonly string[];recheck:boolean}>|undefined{
 const how=GRAPH_MEMO_POLICY_V1.relevance[row.kind];
 if(how===undefined||typeof row.path!=="string")return undefined;
 if(how==="none")return{mentions:[],recheck:false};
 const mentions:string[]=[];let recheck=how==="link";
 const take=(path:unknown):void=>{
  if(typeof path!=="string"){recheck=true;return;}
  const relative=memoRelativeV1(root,path);
  if(relative===undefined||excluded(relative))recheck=true;else mentions.push(relative);
 };
 if(how==="path"||how==="analysis"||how==="link"){take(row.path);return{mentions,recheck};}
 if(how==="resolve"){
  let parsed:unknown;try{parsed=JSON.parse(row.path);}catch{return{mentions:[],recheck:true};}
  if(!Array.isArray(parsed)||typeof parsed[0]!=="string"||typeof parsed[1]!=="string")return{mentions:[],recheck:true};
  const [from,request]=parsed as [string,string];
  take(from);
  if(request.startsWith("."))take(posix.join(dirname(from.replaceAll("\\","/")),request));else if(isAbsolute(request)||/^[A-Za-z]:[\\/]/u.test(request))take(request);else recheck=true;
  if(typeof row.fact==="string"&&!row.fact.startsWith("!"))take(row.fact);
  return{mentions,recheck};
 }
 const base=root.replaceAll("\\","/").replace(/\/+$/u,""),escaped=base.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
 for(const found of row.path.replaceAll("\\\\","/").matchAll(new RegExp(escaped+"(?:/[^\"\\\\\\u0000-\\u001f]*)?","gu")))take(found[0]);
 return{mentions,recheck};
}

/** 📚️ Every row a generation holds with the rows an analysis fact carries inside it, in order. */
export function* memoLeavesV1(row:MemoRowV1):Generator<MemoRowV1>{
 yield row;
 if(row.kind==="analysis"){const nested=(row.fact as {nestedObservations?:unknown}|null)?.nestedObservations;if(Array.isArray(nested))for(const inner of nested)if(inner&&typeof inner==="object")yield*memoLeavesV1(inner as MemoRowV1);}
}

/** 🧩️ How one vouched fact reads the directories it names: `names` every entry it reads (a row naming one of them, or an ancestor, is relevant), `listing` the directories whose children it lists by name or node type (a stamp of the frontier can vouch for them), `strict` the directories where any row directly inside is relevant (resolutions and derived facts, which probe candidates the table does not enumerate, and listings that follow symbolic links). Nothing for a fact the journal cannot see. */
export function memoClassesV1(row:MemoRowV1,reach:Readonly<{mentions:readonly string[];recheck:boolean}>):Readonly<{names:readonly string[];strict:readonly string[];listing:readonly string[]}>{
 if(reach.recheck)return{names:[],strict:[],listing:[]};
 const how=GRAPH_MEMO_POLICY_V1.relevance[row.kind];
 if(how==="path"&&GRAPH_MEMO_POLICY_V1.stamped.includes(row.kind))return{names:reach.mentions,strict:[],listing:reach.mentions};
 if(how==="path"&&GRAPH_MEMO_POLICY_V1.following.includes(row.kind))return{names:reach.mentions,strict:reach.mentions,listing:[]};
 if(how==="resolve"||how==="text")return{names:reach.mentions,strict:reach.mentions.flatMap(mention=>{const cut=mention.lastIndexOf("/");return[mention,cut<0?"":mention.slice(0,cut)];}),listing:[]};
 return{names:reach.mentions,strict:[],listing:[]};
}

/** 🗺️ The directories that matter for a set of vouchable entries: each entry, its directory and every directory above it up to the root (`""`). A change is relevant when it names one of them or lies directly inside one of them. */
export function memoDirectoriesV1(mentions:Iterable<string>,into:Set<string>=new Set<string>()):Set<string>{
 for(const mention of mentions){
  for(let path=mention;!into.has(path);){
   into.add(path);
   if(path==="")break;
   const cut=path.lastIndexOf("/");path=cut<0?"":path.slice(0,cut);
  }
 }
 return into;
}

/** 🔢️ The 53-bit identity of one entry in a relevance table (a string hash kept exact in a double): a collision can only add a relevant entry, never remove one. */
export function memoHashV1(path:string,insensitive:boolean):number{
 const text=insensitive?fold(path):path;
 let h1=0xdeadbeef,h2=0x41c6ce57;
 for(let at=0;at<text.length;at++){const code=text.charCodeAt(at);h1=Math.imul(h1^code,2654435761);h2=Math.imul(h2^code,1597334677);}
 h1=Math.imul(h1^(h1>>>16),2246822507)^Math.imul(h2^(h2>>>13),3266489909);h2=Math.imul(h2^(h2>>>16),2246822507)^Math.imul(h1^(h1>>>13),3266489909);
 return 4294967296*(2097151&h2)+(h1>>>0);
}
const MAGIC=Buffer.from("SMR3");
/** 📦️ Packs the three directory sets of a relevance (names, strict, stamped) as sorted 53-bit hashes behind a small header (magic, folding flag, three counts). `strict` defaults to every name and `stamped` to none, which is the rule without a stamp. */
export function packMemoRelevanceV1(names:ReadonlySet<string>,insensitive:boolean,strict:ReadonlySet<string>=names,stamped:ReadonlySet<string>=new Set<string>()):Buffer{
 const sets=[names,strict,stamped].map(set=>{const hashes=new Float64Array(set.size);let count=0;for(const path of set)hashes[count++]=memoHashV1(path,insensitive);return hashes.sort();});
 const out=Buffer.alloc(20+sets.reduce((sum,hashes)=>sum+hashes.length*8,0));
 MAGIC.copy(out,0);out.writeUInt32BE(insensitive?1:0,4);
 let at=20;
 for(const [index,hashes] of sets.entries()){out.writeUInt32BE(hashes.length,8+index*4);Buffer.from(hashes.buffer,hashes.byteOffset,hashes.length*8).copy(out,at);at+=hashes.length*8;}
 return out;
}
/** 🔓️ A packed relevance: `holds(path)` is true for a name (an entry some fact read, every directory above it, the frontier), `strict(path)` for a directory whose every direct child row is relevant, `stamped(path)` for a directory whose names the frontier stamp vouches for (by hash). */
export type MemoRelevanceV1=Readonly<{insensitive:boolean;count:number;holds(path:string):boolean;strict(path:string):boolean;stamped(path:string):boolean}>;
/** 📭️ Reads a packed relevance back; bytes of another shape are `undefined`. */
export function unpackMemoRelevanceV1(bytes:Uint8Array):MemoRelevanceV1|undefined{
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);
 if(buffer.length<20||!buffer.subarray(0,4).equals(MAGIC))return undefined;
 const insensitive=buffer.readUInt32BE(4)===1,counts=[8,12,16].map(position=>buffer.readUInt32BE(position));
 if(buffer.length!==20+counts.reduce((sum,count)=>sum+count*8,0))return undefined;
 let at=20;
 const tables=counts.map(count=>{const hashes=new Float64Array(count);Buffer.from(hashes.buffer).set(buffer.subarray(at,at+count*8));at+=count*8;return hashes;});
 const has=(hashes:Float64Array,path:string):boolean=>{
  const wanted=memoHashV1(path,insensitive);let low=0,high=hashes.length-1;
  while(low<=high){const mid=(low+high)>>>1,value=hashes[mid]!;if(value===wanted)return true;if(value<wanted)low=mid+1;else high=mid-1;}
  return false;
 };
 return{insensitive,count:counts[0]!,holds:path=>has(tables[0]!,path),strict:path=>has(tables[1]!,path),stamped:path=>has(tables[2]!,path)};
}

const STAMPS=Buffer.from("SML1");
/** 🪧️ Packs the entry stamps `[mtimeMs, inode]` of the stamped directories as records (hash, mtime, inode) sorted by hash behind a small header (magic, folding flag, count). */
export function packMemoStampsV1(stamps:ReadonlyMap<string,readonly[number,number]>,insensitive:boolean):Buffer{
 const records=[...stamps].map(([path,[mtimeMs,ino]])=>[memoHashV1(path,insensitive),mtimeMs,ino] as const).sort((a,b)=>a[0]-b[0]),out=Buffer.alloc(12+records.length*24);
 STAMPS.copy(out,0);out.writeUInt32BE(insensitive?1:0,4);out.writeUInt32BE(records.length,8);
 for(const [index,[hash,mtimeMs,ino]] of records.entries()){out.writeDoubleBE(hash,12+index*24);out.writeDoubleBE(mtimeMs,20+index*24);out.writeDoubleBE(ino,28+index*24);}
 return out;
}
const stampRecordAt=(buffer:Buffer,insensitive:boolean,directory:string):number=>{
 if(buffer.length<12||!buffer.subarray(0,4).equals(STAMPS)||(buffer.readUInt32BE(4)===1)!==insensitive)return -1;
 const count=buffer.readUInt32BE(8);if(buffer.length!==12+count*24)return -1;
 const wanted=memoHashV1(directory,insensitive);let low=0,high=count-1;
 while(low<=high){const mid=(low+high)>>>1,value=buffer.readDoubleBE(12+mid*24);if(value===wanted)return mid;if(value<wanted)low=mid+1;else high=mid-1;}
 return -1;
};
/** ✅️ Whether every directory still has the stamp the frontier captured (same inode, same modification time): the proof the planner uses that no name, node type or absence below it moved. A directory without a record, or a table of another shape, does not hold. */
export function memoStampsHoldV1(root:string,table:Uint8Array,directories:Iterable<string>,insensitive:boolean):boolean{
 const buffer=Buffer.from(table.buffer,table.byteOffset,table.byteLength);
 for(const directory of directories){
  const found=stampRecordAt(buffer,insensitive,directory);
  if(found<0)return false;
  try{const status=lstatSync(join(root,directory));if(!status.isDirectory()||status.mtimeMs!==buffer.readDoubleBE(20+found*24)||!frontierFactHoldsV1(status.ino,buffer.readDoubleBE(28+found*24)))return false;}catch{return false;}
 }
 return true;
}
/** 🔖️ A copy of a stamp table in which the records of some directories carry new stamps `[mtimeMs, inode]` (the directories an edit settlement listed again); `undefined` when the table has another shape or a directory has no record. */
export function restampMemoStampsV1(table:Uint8Array,stamps:ReadonlyMap<string,readonly[number,number]>,insensitive:boolean):Buffer|undefined{
 const buffer=Buffer.from(table);
 for(const [directory,[mtimeMs,ino]] of stamps){const found=stampRecordAt(buffer,insensitive,directory);if(found<0)return undefined;buffer.writeDoubleBE(mtimeMs,20+found*24);buffer.writeDoubleBE(ino,28+found*24);}
 return buffer;
}

const EDITS=Buffer.from("SME1");
/** 🖋️ What an edit index is built from: the rows (by their position in the fact index of the generation) that name each entry, the rows that read the children of each directory (listings, and resolutions and derived facts that probe candidates there), the directories with something relevant below them, and the frontier directories and members of the generation. */
export type MemoEditInputV1=Readonly<{named:Readonly<{paths:readonly string[];ids:readonly number[]}>;inside:Readonly<{paths:readonly string[];ids:readonly number[]}>;above:Iterable<string>;frontier:Iterable<string>;members:Iterable<string>}>;
/** 🗂️ Packs an edit index: two tables of distinct (53-bit path hash, row id) pairs sorted by hash and three sorted hash sets behind a small header (magic, folding flag, five counts). A collision only adds rows to re-check or a refusal, never removes one. */
export function packMemoEditIndexV1(input:MemoEditInputV1,insensitive:boolean):Buffer{
 const pairs=(table:Readonly<{paths:readonly string[];ids:readonly number[]}>)=>{
  const hashes=Float64Array.from(table.paths,path=>memoHashV1(path,insensitive)),order=Uint32Array.from(hashes.keys()).sort((a,b)=>hashes[a]!-hashes[b]!||table.ids[a]!-table.ids[b]!),kept:number[]=[];
  for(const at of order){const last=kept[kept.length-1];if(last===undefined||hashes[last]!==hashes[at]||table.ids[last]!==table.ids[at])kept.push(at);}
  return{hashes:Float64Array.from(kept,at=>hashes[at]!),ids:Uint32Array.from(kept,at=>table.ids[at]!)};
 };
 const tables=[pairs(input.named),pairs(input.inside)],sets=[input.above,input.frontier,input.members].map(paths=>Float64Array.from(new Set(Array.from(paths,path=>memoHashV1(path,insensitive)))).sort());
 const out=Buffer.alloc(28+tables.reduce((sum,table)=>sum+table.hashes.length*12,0)+sets.reduce((sum,set)=>sum+set.length*8,0));
 EDITS.copy(out,0);out.writeUInt32BE(insensitive?1:0,4);
 let at=28;
 const put=(array:Float64Array|Uint32Array)=>{Buffer.from(array.buffer,array.byteOffset,array.byteLength).copy(out,at);at+=array.byteLength;};
 tables.forEach((table,index)=>{out.writeUInt32BE(table.hashes.length,8+index*4);put(table.hashes);put(table.ids);});
 sets.forEach((set,index)=>{out.writeUInt32BE(set.length,16+index*4);put(set);});
 return out;
}
/** 🔍️ An unpacked edit index: `named(path)` the rows that name an entry, `inside(path)` the rows that read the children of a directory, `above(path)` whether something relevant lies below a directory, `frontier(path)` and `member(path)` whether the frontier walked a directory or holds a member. */
export type MemoEditIndexV1=Readonly<{insensitive:boolean;named(path:string):readonly number[];inside(path:string):readonly number[];above(path:string):boolean;frontier(path:string):boolean;member(path:string):boolean}>;
/** 📬️ Reads a packed edit index back; bytes of another shape are `undefined`. */
export function unpackMemoEditIndexV1(bytes:Uint8Array):MemoEditIndexV1|undefined{
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);
 if(buffer.length<28||!buffer.subarray(0,4).equals(EDITS))return undefined;
 const insensitive=buffer.readUInt32BE(4)===1,counts=[8,12,16,20,24].map(position=>buffer.readUInt32BE(position));
 if(buffer.length!==28+(counts[0]!+counts[1]!)*12+(counts[2]!+counts[3]!+counts[4]!)*8)return undefined;
 let at=28;
 const doubles=(count:number)=>{const out=new Float64Array(count);Buffer.from(out.buffer).set(buffer.subarray(at,at+count*8));at+=count*8;return out;},words=(count:number)=>{const out=new Uint32Array(count);Buffer.from(out.buffer).set(buffer.subarray(at,at+count*4));at+=count*4;return out;};
 const named={hashes:doubles(counts[0]!),ids:words(counts[0]!)},inside={hashes:doubles(counts[1]!),ids:words(counts[1]!)},above=doubles(counts[2]!),frontier=doubles(counts[3]!),members=doubles(counts[4]!);
 const first=(hashes:Float64Array,wanted:number):number=>{let low=0,high=hashes.length;while(low<high){const mid=(low+high)>>>1;if(hashes[mid]!<wanted)low=mid+1;else high=mid;}return low;};
 const rows=(table:{hashes:Float64Array;ids:Uint32Array},path:string):number[]=>{const wanted=memoHashV1(path,insensitive),out:number[]=[];for(let index=first(table.hashes,wanted);index<table.hashes.length&&table.hashes[index]===wanted;index++)out.push(table.ids[index]!);return out;};
 const has=(hashes:Float64Array,path:string):boolean=>{const wanted=memoHashV1(path,insensitive),index=first(hashes,wanted);return index<hashes.length&&hashes[index]===wanted;};
 return{insensitive,named:path=>rows(named,path),inside:path=>rows(inside,path),above:path=>has(above,path),frontier:path=>has(frontier,path),member:path=>has(members,path)};
}

/** 🌲️ The frontier of a generation with the planner's own walk predicates: `descend(name, path, parent)` whether the walk enters a directory, `member(path)` whether a file is a member. Frontier paths are workspace-relative with the root `.`. */
export type MemoFrontierV1=Readonly<{frontier:Readonly<{directories:readonly (readonly [string,number,number,...unknown[]])[];members:readonly (readonly [string,number,...unknown[]])[]}>;descend:(name:string,path:string,parent:string)=>boolean;member:(path:string)=>boolean}>;
/** 🏞️ The first path whose membership the frontier no longer proves, or `undefined`: an `identity` directory is still the very directory the frontier walked (a directory with its recorded, known inode, so nothing was renamed over it and an entry below it changes only by a row of its own); a `listing` directory is that, and the subdirectories the walk enters and the member files it holds are exactly the frontier's, with unchanged modes; a `member` is still a file with its recorded mode. This is `advanceFrontierV1` for the directories the journal names, so a membership it keeps is the planner's own.
 * @see ../../../🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs `advanceFrontierV1` */
export function memoFrontierHoldsV1(root:string,input:MemoFrontierV1,checks:Readonly<{identity:Iterable<string>;listing:Iterable<string>;members:Iterable<string>}>):string|undefined{
 const key=(path:string)=>path==="."?"":path,parentOf=(path:string)=>{const cut=path.lastIndexOf("/");return cut<0?"":path.slice(0,cut);};
 const listing=new Set(checks.listing),directories=new Map<string,number>(),members=new Map<string,number>(),children=new Map<string,Set<string>>();
 const child=(path:string,kind:string)=>{const parent=parentOf(path);if(!listing.has(parent))return;let set=children.get(parent);if(!set)children.set(parent,set=new Set());set.add(kind+path);};
 for(const row of input.frontier.directories){const path=key(row[0]);directories.set(path,row[2]);if(path!=="")child(path,"d:");}
 for(const row of input.frontier.members){members.set(row[0],row[1]);child(row[0],"f:");}
 const same=(path:string):boolean=>{const ino=directories.get(path);if(!ino)return false;try{const status=lstatSync(join(root,path));return status.isDirectory()&&status.ino===ino;}catch{return false;}};
 const member=(path:string):boolean=>{const mode=members.get(path);if(mode===undefined)return false;try{const status=lstatSync(join(root,path));return status.isFile()&&frontierFactHoldsV1(status.mode,mode);}catch{return false;}};
 for(const path of checks.identity)if(!same(path))return path;
 for(const path of listing){
  if(!same(path))return path;
  const found=new Set<string>();
  try{for(const entry of readdirSync(join(root,path),{withFileTypes:true})){const name=String(entry.name),inner=path===""?name:path+"/"+name;if(entry.isDirectory()){if(input.descend(name,inner,path))found.add("d:"+inner);}else if(entry.isFile()&&input.member(inner))found.add("f:"+inner);}}catch{return path;}
  const recorded=children.get(path)??new Set<string>();
  if(found.size!==recorded.size||[...found].some(entry=>!recorded.has(entry)))return path;
  for(const entry of found)if(entry.startsWith("f:")&&!member(entry.slice(2)))return path;
 }
 for(const path of checks.members)if(!member(path))return path;
 return undefined;
}

/** 🔌️ The ports an edit settlement reads: the rows of the sealed generation's fact index by position and its frontier with the planner's walk predicates (`undefined` when that generation is no longer retained). */
export type GraphMemoEditPortsV1=Readonly<{
 rows:(index:string,ids:readonly number[],signal?:AbortSignal)=>Promise<readonly MemoRowV1[]|undefined>;
 frontier:(digest:string,signal?:AbortSignal)=>Promise<MemoFrontierV1|undefined>;
}>;
/** 📋️ What an edit settlement answers: held, with the number of changed paths, the rows it re-checked and the new stamps of the stamped directories it listed again, or the first path that does not hold. */
export type MemoEditOutcomeV1=Readonly<{held:true;paths:number;rows:number;stamps:ReadonlyMap<string,readonly[number,number]>}>|Readonly<{held:false;path:string}>;

/** 🧺️ Every journal row that is relevant by itself (the rule of {@link classifyMemoChangesV1}, without stopping at the first) and, for every stamped directory, the first row that only says 'modified' directly inside it. */
export function collectMemoChangesV1(relevance:MemoRelevanceV1,rows:readonly ChangeRowV1[]):Readonly<{changed:readonly ChangeRowV1[];pending:ReadonlyMap<string,ChangeRowV1>}>{
 const changed:ChangeRowV1[]=[],pending=new Map<string,ChangeRowV1>();
 for(const row of rows){
  const path=row[2],cut=path.lastIndexOf("/"),parent=cut<0?"":path.slice(0,cut);
  if(relevance.holds(path)||relevance.strict(parent)&&!(row[0]==="m"&&row[1]===1)||relevance.stamped(parent)&&row[0]!=="m")changed.push(row);
  else if(relevance.stamped(parent)&&!pending.has(parent))pending.set(parent,row);
 }
 return{changed,pending};
}

/** ✏️ Settles the relevant journal rows of a sealed memo with the planner's own checks instead of the planner: a row naming an entry re-checks the rows that name it (and proves a directory with relevant entries below it, or a frontier directory, to be the same directory, and a member to keep its mode); a row directly inside a strict or stamped directory, and a stamped directory whose stamp moved (`listed`), re-checks the rows that read its children and, for a frontier directory, lists it again with the walk predicates. A path the memo cannot explain, a touched path that became a symbolic link (the journal cannot see behind it), a row the check cannot verify, a retired generation and more than `maximumEditPaths` paths or `maximumEditRows` rows refuse. When everything holds, a stamped directory listed again gets the stamp it had before the checks, if it still has it and lies outside the racy window. */
export async function settleMemoEditsV1(input:Readonly<{root:string;relevance:MemoRelevanceV1;edits:MemoEditIndexV1;changed:readonly ChangeRowV1[];listed:ReadonlyMap<string,string>;index:string;frontier:string|null;ports:GraphMemoEditPortsV1;holds:(rows:readonly MemoRowV1[],signal?:AbortSignal)=>Promise<boolean>;now:()=>number;signal?:AbortSignal}>):Promise<MemoEditOutcomeV1>{
 const {root,relevance,edits,signal}=input,policy=GRAPH_MEMO_POLICY_V1,groups=new Map<string,Set<number>>(),listed=new Map(input.listed),identity=new Map<string,string>(),listing=new Map<string,string>(),members=new Map<string,string>(),touched=new Map<string,string>();
 const refuse=(path:string):MemoEditOutcomeV1=>({held:false,path});
 const group=(path:string,ids:readonly number[]):void=>{let set=groups.get(path);if(!set)groups.set(path,set=new Set());for(const id of ids)set.add(id);};
 if(input.changed.length+listed.size>policy.maximumEditPaths)return refuse(input.changed[0]?.[2]??listed.values().next().value!);
 for(const row of input.changed){
  const path=row[2],cut=path.lastIndexOf("/"),parent=cut<0?"":path.slice(0,cut);
  if(relevance.holds(path)){
   const ids=edits.named(path),above=edits.above(path),frontier=edits.frontier(path),member=edits.member(path);
   if(above&&!frontier||!ids.length&&!frontier&&!member)return refuse(path);
   group(path,ids);touched.set(path,path);
   if(frontier)identity.set(path,path);
   if(member)members.set(path,path);
  }
  if((relevance.strict(parent)&&!(row[0]==="m"&&row[1]===1)||relevance.stamped(parent)&&row[0]!=="m")&&!listed.has(parent))listed.set(parent,path);
 }
 for(const [directory,path] of listed){group(path,edits.inside(directory));touched.set(directory,path);if(edits.frontier(directory))listing.set(directory,path);}
 const all=new Set<number>();for(const set of groups.values())for(const id of set)all.add(id);
 if(all.size>policy.maximumEditRows)return refuse(groups.keys().next().value!);
 for(const [path,detail] of touched){try{if(lstatSync(join(root,path)).isSymbolicLink())return refuse(detail);}catch{}}
 const before=new Map<string,readonly[number,number]>();
 for(const directory of listed.keys())if(relevance.stamped(directory)){try{const status=lstatSync(join(root,directory));if(status.isDirectory())before.set(directory,[status.mtimeMs,status.ino]);}catch{}}
 if(identity.size||listing.size||members.size){
  const first=identity.values().next().value??listing.values().next().value??members.values().next().value!;
  const frontier=input.frontier===null?undefined:await input.ports.frontier(input.frontier,signal);
  if(!frontier)return refuse(first);
  const failed=memoFrontierHoldsV1(root,frontier,{identity:identity.keys(),listing:listing.keys(),members:members.keys()});
  if(failed!==undefined)return refuse(identity.get(failed)??listing.get(failed)??members.get(failed)??failed);
 }
 if(all.size){
  const ids=[...all],rows=await input.ports.rows(input.index,ids,signal);
  if(!rows||rows.length!==ids.length)return refuse(groups.keys().next().value!);
  const byId=new Map(ids.map((id,at)=>[id,rows[at]!]));
  for(const [path,set] of groups)if(set.size&&!await input.holds([...set].map(id=>byId.get(id)!),signal))return refuse(path);
 }
 const stamps=new Map<string,readonly[number,number]>(),at=input.now();
 for(const [directory,[mtimeMs,ino]] of before){try{const status=lstatSync(join(root,directory));if(status.isDirectory()&&status.mtimeMs===mtimeMs&&status.ino===ino&&at-mtimeMs>=GENERATION_POLICY_V1.racyMilliseconds)stamps.set(directory,[mtimeMs,ino]);}catch{}}
 return{held:true,paths:new Set([...input.changed.map(row=>row[2]),...input.listed.values()]).size,rows:all.size,stamps};
}

/** ⚖️ What a stretch of journal rows means for a relevance: the first row that is relevant by itself (it names an entry some fact read, an ancestor of one or a frontier directory; it lies directly inside a strict directory, except for the modification of a directory; it adds, removes or renames a name directly inside a stamped directory), and for every stamped directory the first row that only says 'modified' directly inside it, which the stamp of that directory must still confirm because the kind of a row is advisory. */
export function classifyMemoChangesV1(relevance:MemoRelevanceV1,rows:readonly ChangeRowV1[]):Readonly<{changed?:ChangeRowV1;pending:ReadonlyMap<string,ChangeRowV1>}>{
 const pending=new Map<string,ChangeRowV1>();
 for(const row of rows){
  const path=row[2],cut=path.lastIndexOf("/"),parent=cut<0?"":path.slice(0,cut);
  if(relevance.holds(path))return{changed:row,pending};
  if(relevance.strict(parent)&&!(row[0]==="m"&&row[1]===1))return{changed:row,pending};
  if(relevance.stamped(parent)){if(row[0]!=="m")return{changed:row,pending};if(!pending.has(parent))pending.set(parent,row);}
 }
 return{pending};
}
/** 🔬️ The first row that touches what the memo read when no stamp table is at hand: a row that is relevant by itself, else the first row that needs a stamp. */
export function firstRelevantChangeV1(relevance:MemoRelevanceV1,rows:readonly ChangeRowV1[]):ChangeRowV1|undefined{
 const found=classifyMemoChangesV1(relevance,rows);
 return found.changed??found.pending.values().next().value;
}

/** 🧾️ The identity check of an analysed source: the stat image the fact recorded is the one the file still has. */
export function memoIdentityHoldsV1(root:string,identity:MemoIdentityV1):boolean{
 try{const status=statSync(isAbsolute(identity.path)?identity.path:join(root,identity.path));return status.size===identity.size&&status.mtimeMs===identity.mtimeMs&&status.ctimeMs===identity.ctimeMs&&status.ino===identity.ino;}catch{return false;}
}

const atomic=(target:string,data:string|Uint8Array):void=>{const temporary=target+"."+process.pid+"."+Date.now()+".tmp";try{writeFileSync(temporary,data,{mode:0o600});renameSync(temporary,target);}catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}};
const readBounded=(target:string,maximum:number):Buffer|undefined=>{try{if(statSync(target).size>maximum)return undefined;return readFileSync(target);}catch{return undefined;}};
const named=(directory:string,name:string,suffix:string):string=>join(directory,name+suffix);
const pointerOf=(bytes:Buffer|undefined):Readonly<{generation:number;manifest:string}>|undefined=>{try{const value=JSON.parse(bytes!.toString("utf8")) as {generation?:unknown;manifest?:unknown};return Number.isSafeInteger(value.generation)&&isSha(value.manifest)?{generation:value.generation as number,manifest:value.manifest}:undefined;}catch{return undefined;}};

/** 🧱️ The ports of one memo: the journal client, the record directory, the workspace root, the identity of the request, the engine check of the rows out of the journal's sight (the planner's own check, which also re-checks the rows an edit touched), the carry of a sealed slice to the generation the pointer names now (without it a moved pointer refuses) and the ports of the edit settlement (without them a relevant change refuses). */
export type GraphMemoOptionsV1=Readonly<{
 root:string;directory:string;client:ChangeJournalClientV1;key:string;insensitive?:boolean;now?:()=>number;signal?:AbortSignal;environment?:Readonly<Record<string,string|undefined>>;
 holds?:(rows:readonly MemoRowV1[],signal?:AbortSignal)=>Promise<boolean>;
 carry?:(input:Readonly<{from:Readonly<{generation:number;manifest:string}>;to:Readonly<{generation:number;manifest:string}>;pin:string}>,signal?:AbortSignal)=>Promise<Readonly<{carried:true;pin:string}>|Readonly<{carried:false;reason:string}>>;
 edit?:GraphMemoEditPortsV1;
}>;
/** 🪟️ One request view of the memo. */
export interface GraphMemoV1{
 /** 📖️ Answers from a sealed record when the journal proves that nothing the planned generation read has changed, or when the facts that read the changed paths still hold by the planner's own checks ({@link settleMemoEditsV1}). */
 lookup():Promise<MemoAnswerV1>;
 /** 🔏️ Keeps the slice a planner child produced, after it exited cleanly. With the child's `sidecar` (its relevance of the generation, and its edit index when it wrote one) the record is sealed; without it only a light record is kept, which the next identical request turns into a sealed one. */
 seal(input:Readonly<{plan:string;pin:string|undefined;projects:number;sidecar?:string}>):Promise<MemoReasonV1|"sealed"|"light">;
 /** 🧲️ The generation a light record kept for this request: the child of the next run computes its relevance only when it serves this generation or a newer one holding the same slice. */
 expected():string|undefined;
 /** 📍️ The journal position taken before the planner child started (`undefined` when the journal cannot vouch). */
 readonly cursor:string|undefined;
 /** ⏲️ The clock time right after the cursor was taken (`undefined` without a cursor): a planner child whose slice is sealed must have verified it at or after this time, never earlier (a peer's stamp or an outer pin may predate an edit that lands before the cursor). */
 readonly cursorAtMs:number|undefined;
}

const reown=(pin:string,environment:Readonly<Record<string,string|undefined>>,now:number):string|undefined=>{
 try{
  const value=JSON.parse(pin) as {generation:number;manifest:string;covered:{count:number;owners:string}},owner=Number(environment.SEMIO_BUILD_INVOCATION_PID);
  return JSON.stringify({version:1,owner:Number.isSafeInteger(owner)&&owner>0?owner:process.pid,generation:value.generation,manifest:value.manifest,covered:{atMs:now,count:value.covered.count,owners:value.covered.owners}});
 }catch{return undefined;}
};
const manifestOfPin=(pin:string):string|undefined=>{try{const value=JSON.parse(pin) as {manifest?:unknown};return isSha(value.manifest)?value.manifest:undefined;}catch{return undefined;}};

/** 🚪️ Opens the memo of one request. The journal cursor is taken here, before any planner child starts, so that every change made while the child plans is reported later. */
export async function openGraphMemoV1(options:GraphMemoOptionsV1):Promise<GraphMemoV1>{
 const {root,directory,client,key,signal,environment=process.env}=options,now=options.now??Date.now,insensitive=options.insensitive??(process.platform==="darwin"||process.platform==="win32"),policy=GRAPH_MEMO_POLICY_V1;
 let cursorText:string|undefined,cursorAtMs:number|undefined;
 try{const taken=await client.cursor(signal);if(!("exact" in taken)){cursorText=formatChangeCursorV1(taken);cursorAtMs=Date.now();}}catch(error){signal?.throwIfAborted();}
 const recordFile=named(directory,key.slice(0,40),policy.recordSuffix);
 const readRecord=():Record<string,any>|undefined=>{
  const bytes=readBounded(recordFile,policy.maximumRecordBytes);if(!bytes)return undefined;
  try{
   const value=JSON.parse(bytes.toString("utf8")) as Record<string,any>;
   if(value.version!==1||value.key!==key||typeof value.cursor!=="string"||!parseChangeCursorV1(value.cursor)||!isSha(value.manifest)||!isSha(value.plan?.sha256)||typeof value.pin!=="string")return undefined;
   if(value.state==="sealed"&&!(typeof value.pointer?.file==="string"&&Number.isSafeInteger(value.pointer?.generation)&&value.pointer?.manifest===value.manifest&&isSha(value.relevance?.id)&&isSha(value.relevance?.stamps)&&Number.isSafeInteger(value.relevance?.rechecked)&&(value.relevance?.edit===undefined||isSha(value.relevance.edit?.id)&&isSha(value.relevance.edit?.index)&&(value.relevance.edit?.frontier===null||isSha(value.relevance.edit?.frontier)))))return undefined;
   return value.state==="sealed"||value.state==="light"?value:undefined;
  }catch{return undefined;}
 };
 return{
  cursor:cursorText,
  cursorAtMs,
  expected:()=>readRecord()?.manifest,
  async lookup(){
   const miss=(reason:MemoReasonV1,detail?:string):MemoAnswerV1=>({hit:false,reason,...(detail===undefined?{}:{detail})});
   try{
    const record=readRecord();if(!record)return miss("none");
    if(record.state==="light")return miss("light");
    const pointer=pointerOf(readBounded(record.pointer.file,65536)),moved=!pointer||pointer.generation!==record.pointer.generation||pointer.manifest!==record.pointer.manifest;if(!pointer||moved&&!options.carry)return miss("pointer");
    const answer=await client.since(parseChangeCursorV1(record.cursor)!,[""],signal);
    if(!answer.exact)return miss("journal",answer.reason);
    const relBytes=readBounded(named(directory,record.relevance.id,policy.relevanceSuffix),policy.maximumRelevanceBytes),checkBytes=readBounded(named(directory,record.relevance.id,policy.recheckSuffix),policy.maximumRecheckBytes),relevance=relBytes&&unpackMemoRelevanceV1(relBytes);
    if(!relBytes||!checkBytes||!relevance||relevance.insensitive!==insensitive||sha256(Buffer.concat([relBytes,Buffer.from("\n"),checkBytes]))!==record.relevance.id)return miss("generation");
    const classified=collectMemoChangesV1(relevance,answer.rows),listed=new Map<string,string>(),edit=record.relevance.edit as Readonly<{id:string;index:string;frontier:string|null}>|undefined,stampFile=()=>{const bytes=readBounded(named(directory,record.relevance.stamps,policy.stampSuffix),policy.maximumStampBytes);return bytes&&sha256(bytes)===record.relevance.stamps?bytes:undefined;};
    let rechecked=0,edited:MemoEditOutcomeV1|undefined;
    if(classified.pending.size>0){
     const stampBytes=stampFile();if(!stampBytes)return miss("generation");
     for(const [pending,row] of classified.pending)if(memoStampsHoldV1(root,stampBytes,[pending],insensitive))rechecked++;else listed.set(pending,row[2]);
    }
    const touched=classified.changed[0]?.[2]??listed.values().next().value;
    if(touched!==undefined&&!(options.edit&&options.holds&&edit))return miss("changed",touched);
    if(record.relevance.rechecked>0){
     const recheck=JSON.parse(checkBytes.toString("utf8")) as MemoRecheckV1;
     if(!options.holds)return miss("generation");
     rechecked+=recheck.rows.length+recheck.identities.length;
     if(!recheck.identities.every(identity=>memoIdentityHoldsV1(root,identity))||!await options.holds(recheck.rows,signal))return miss("stale");
    }
    if(touched!==undefined&&edit){
     const editBytes=readBounded(named(directory,edit.id,policy.editSuffix),policy.maximumEditBytes),edits=editBytes&&sha256(editBytes)===edit.id?unpackMemoEditIndexV1(editBytes):undefined;
     if(!edits||edits.insensitive!==insensitive)return miss("generation");
     edited=await settleMemoEditsV1({root,relevance,edits,changed:classified.changed,listed,index:edit.index,frontier:edit.frontier,ports:options.edit!,holds:options.holds!,now,...(signal?{signal}:{})});
     if(!edited.held)return miss("changed",edited.path);
    }
    let served=record;
    if(moved){const carried=await options.carry!({from:{generation:record.pointer.generation,manifest:record.pointer.manifest},to:pointer,pin:record.pin},signal);if(!carried.carried)return miss("pointer",carried.reason);served={...record,manifest:pointer.manifest,pointer:{...record.pointer,generation:pointer.generation,manifest:pointer.manifest},pin:carried.pin};}
    const planBytes=readBounded(named(directory,record.plan.sha256,policy.planSuffix),policy.maximumPlanBytes);
    if(!planBytes||planBytes.length!==record.plan.bytes||sha256(planBytes)!==record.plan.sha256)return miss("plan");
    const at=now(),pin=reown(served.pin,environment,at);
    if(pin===undefined)return miss("pin");
    try{
     let stamps=record.relevance.stamps as string;
     if(edited?.held&&edited.stamps.size){const table=stampFile(),next=table&&restampMemoStampsV1(table,edited.stamps,insensitive);if(next){stamps=sha256(next);if(!existsSync(named(directory,stamps,policy.stampSuffix)))atomic(named(directory,stamps,policy.stampSuffix),next);}}
     atomic(recordFile,JSON.stringify({...served,cursor:formatChangeCursorV1(answer.cursor),relevance:{...served.relevance,stamps}}));
    }catch{}
    return{hit:true,plan:planBytes.toString("utf8"),pin,projects:record.projects,ageMilliseconds:Math.max(0,at-record.sealedAtMs),rows:record.relevance.rows,rechecked,...(moved?{carried:{from:record.pointer.generation,to:pointer.generation}}:{}),...(edited?.held?{edited:{paths:edited.paths,rows:edited.rows}}:{})};
   }catch(error){signal?.throwIfAborted();return miss("error",error instanceof Error?error.message:String(error));}
  },
  async seal(input){
   if(cursorText===undefined)return "journal";
   try{
    const manifest=input.pin===undefined?undefined:manifestOfPin(input.pin);
    if(input.pin===undefined||manifest===undefined)return "pin";
    const plan=Buffer.from(input.plan,"utf8"),planSha=sha256(plan);
    if(plan.length>policy.maximumPlanBytes)return "plan";
    mkdirSync(directory,{recursive:true,mode:0o700});
    if(!existsSync(named(directory,planSha,policy.planSuffix)))atomic(named(directory,planSha,policy.planSuffix),plan);
    const base={version:1,key,cursor:cursorText,sealedAtMs:now(),manifest,plan:{sha256:planSha,bytes:plan.length},pin:input.pin,projects:input.projects};
    let header:Record<string,any>|undefined;
    if(input.sidecar!==undefined){try{header=JSON.parse(readFileSync(input.sidecar,"utf8")) as Record<string,any>;}catch{header=undefined;}}
    const sealed=header&&header.version===1&&header.unsealable===null&&header.pointer?.manifest===manifest?header:undefined;
    if(sealed){
     const pointer=pointerOf(readBounded(sealed.pointer.file,65536)),stem=input.sidecar!.replace(/\.json$/u,""),relBytes=readBounded(stem+policy.relevanceSuffix,policy.maximumRelevanceBytes),checkBytes=readBounded(stem+policy.recheckSuffix,policy.maximumRecheckBytes),stampBytes=readBounded(stem+policy.stampSuffix,policy.maximumStampBytes);
     if(pointer&&pointer.generation===sealed.pointer.generation&&pointer.manifest===manifest&&relBytes&&checkBytes&&stampBytes&&unpackMemoRelevanceV1(relBytes)){
      const id=sha256(Buffer.concat([relBytes,Buffer.from("\n"),checkBytes])),stamps=sha256(stampBytes);
      if(!existsSync(named(directory,stamps,policy.stampSuffix)))atomic(named(directory,stamps,policy.stampSuffix),stampBytes);
      if(!existsSync(named(directory,id,policy.relevanceSuffix))){atomic(named(directory,id,policy.recheckSuffix),checkBytes);atomic(named(directory,id,policy.relevanceSuffix),relBytes);}
      const editBytes=isSha(sealed.edit?.index)&&(sealed.edit?.frontier===null||isSha(sealed.edit?.frontier))?readBounded(stem+policy.editSuffix,policy.maximumEditBytes):undefined,edit=editBytes&&unpackMemoEditIndexV1(editBytes)?{id:sha256(editBytes),index:sealed.edit.index as string,frontier:sealed.edit.frontier as string|null}:undefined;
      if(edit&&!existsSync(named(directory,edit.id,policy.editSuffix)))atomic(named(directory,edit.id,policy.editSuffix),editBytes!);
      const kept=readRecord(),own=parseChangeCursorV1(cursorText)!,older=kept&&parseChangeCursorV1(kept.cursor);
      if(!(kept&&kept.state==="sealed"&&older&&older.epoch===own.epoch&&older.seq>=own.seq&&kept.plan.sha256===planSha&&kept.relevance.id===id&&kept.relevance.stamps===stamps&&kept.relevance.edit?.id===edit?.id))atomic(recordFile,JSON.stringify({...base,state:"sealed",pointer:{file:sealed.pointer.file,generation:sealed.pointer.generation,manifest},relevance:{id,stamps,rows:sealed.rows,rechecked:sealed.rechecked,...(edit?{edit}:{})}}));
      collectGraphMemoV1(directory);
      return "sealed";
     }
    }
    atomic(recordFile,JSON.stringify({...base,state:"light"}));
    collectGraphMemoV1(directory);
    return "light";
   }catch{return "error";}
  },
 };
}

/** 🧹️ Keeps the newest `keep` records of a memo directory within the byte budget of the retention schema and removes every plan and relevance file no kept record names (and none younger than the grace); the removal is the plan of `planGraphMemoCollectionV1`, and a removal the byte budget drove is announced once on stderr in English and German. */
export function collectGraphMemoV1(directory:string,keep:number=GRAPH_MEMO_RETENTION_V1.recordsKept,maximumBytes:number=GRAPH_MEMO_RETENTION_V1.maximumDirectoryBytes):GraphMemoCollectionV1|undefined{
 try{
  const policy=GRAPH_MEMO_POLICY_V1,retention={...GRAPH_MEMO_RETENTION_V1,recordsKept:keep,maximumDirectoryBytes:maximumBytes},records:GraphMemoRecordV1[]=[],files:Record<string,GraphMemoFileV1>={};
  for(const name of readdirSync(directory)){
   try{
    const facts=lstatSync(join(directory,name));
    if(!name.endsWith(policy.recordSuffix)){files[name]={bytes:facts.size,mtimeMs:facts.mtimeMs};continue;}
    const owned:string[]=[];
    try{const value=JSON.parse(readFileSync(join(directory,name),"utf8")) as {relevance?:{id?:string;stamps?:string;edit?:{id?:string}};plan?:{sha256?:string}};if(value.relevance?.id)owned.push(String(value.relevance.id)+policy.relevanceSuffix,String(value.relevance.id)+policy.recheckSuffix,String(value.relevance.stamps)+policy.stampSuffix);if(value.relevance?.edit?.id)owned.push(String(value.relevance.edit.id)+policy.editSuffix);owned.push(String(value.plan?.sha256)+policy.planSuffix);}catch{}
    records.push({name,bytes:facts.size,mtimeMs:facts.mtimeMs,owned});
   }catch{}
  }
  const plan=planGraphMemoCollectionV1(records,files,Date.now(),retention);
  for(const name of [...plan.removeRecords,...plan.removeFiles])rmSync(join(directory,name),{force:true});
  if(plan.overBudget>0)process.stderr.write(formatGraphMemoCollectionV1(plan,"en",retention)+" / "+formatGraphMemoCollectionV1(plan,"de",retention)+"\n");
  return plan;
 }catch{return undefined;}
}
