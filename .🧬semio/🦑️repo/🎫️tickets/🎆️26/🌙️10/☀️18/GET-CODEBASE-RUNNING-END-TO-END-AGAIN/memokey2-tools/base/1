import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,posix} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {formatChangeCursorV1,parseChangeCursorV1,type ChangeRowV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";

/** 🧠️ The declared constants of the graph memo; the schema file is their single source. */
export const GRAPH_MEMO_POLICY_V1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as Readonly<{
 version:1;directory:string;switch:string;sidecar:string;expect:string;recordSuffix:string;planSuffix:string;relevanceSuffix:string;recheckSuffix:string;maximumRecordBytes:number;maximumPlanBytes:number;maximumRelevanceBytes:number;maximumRecheckBytes:number;maximumRows:number;leafSymlinkCheckConcurrency:number;
 environmentPrefixes:readonly string[];environmentDenied:readonly string[];relevance:Readonly<Record<string,"path"|"analysis"|"link"|"resolve"|"text"|"none">>;texts:Readonly<Record<string,Readonly<Record<"served"|"refused",string>>>>;
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
export type MemoAnswerV1=Readonly<{hit:true;plan:string;pin:string|undefined;projects:number;ageMilliseconds:number;rows:number;rechecked:number}>|Readonly<{hit:false;reason:MemoReasonV1;detail?:string}>;

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
export function renderGraphMemoTextV1(key:"served"|"refused",fields:Readonly<Record<string,string|number>>,language:string):string{
 const table=GRAPH_MEMO_POLICY_V1.texts[language]??GRAPH_MEMO_POLICY_V1.texts.en!;
 return table[key].replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));
}

/** 🔑️ The identity of one request: what is asked, by which implementation, in which environment. Anything that can change the planned slice is a member; volatile process state (pins, pids, plan files) is not. */
export function graphMemoKeyV1(parts:Readonly<{root:string;args:readonly string[];tool:string|undefined;mode:string|undefined;implementation:string;environment:Readonly<Record<string,string|undefined>>}>):string{
 const policy=GRAPH_MEMO_POLICY_V1,names=Object.keys(parts.environment).filter(name=>policy.environmentPrefixes.some(prefix=>name.startsWith(prefix))&&!policy.environmentDenied.includes(name)).sort();
 return sha256(JSON.stringify({version:policy.version,root:parts.root,args:parts.args,tool:parts.tool??null,mode:parts.mode??null,implementation:parts.implementation,runtime:[process.platform,process.arch,process.versions.bun??process.version],environment:names.map(name=>[name,parts.environment[name]??null])}));
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
const MAGIC=Buffer.from("SMR2");
/** 📦️ Packs a set of directories as sorted 53-bit hashes behind a small header (magic, folding flag, count). */
export function packMemoRelevanceV1(directories:ReadonlySet<string>,insensitive:boolean):Buffer{
 const hashes=new Float64Array(directories.size);let count=0;
 for(const path of directories)hashes[count++]=memoHashV1(path,insensitive);
 hashes.sort();
 const out=Buffer.alloc(12+count*8);
 MAGIC.copy(out,0);out.writeUInt32BE(insensitive?1:0,4);out.writeUInt32BE(count,8);
 Buffer.from(hashes.buffer,hashes.byteOffset,count*8).copy(out,12);
 return out;
}
/** 🔓️ A packed relevance table: `holds(path)` is true when the path is one of the directories (by hash). */
export type MemoRelevanceV1=Readonly<{insensitive:boolean;count:number;holds(path:string):boolean}>;
/** 📭️ Reads a packed relevance table back; bytes of another shape are `undefined`. */
export function unpackMemoRelevanceV1(bytes:Uint8Array):MemoRelevanceV1|undefined{
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);
 if(buffer.length<12||!buffer.subarray(0,4).equals(MAGIC))return undefined;
 const insensitive=buffer.readUInt32BE(4)===1,count=buffer.readUInt32BE(8);
 if(buffer.length!==12+count*8)return undefined;
 const hashes=new Float64Array(count);Buffer.from(hashes.buffer).set(buffer.subarray(12));
 return{insensitive,count,holds(path){
  const wanted=memoHashV1(path,insensitive);let low=0,high=count-1;
  while(low<=high){const mid=(low+high)>>>1,value=hashes[mid]!;if(value===wanted)return true;if(value<wanted)low=mid+1;else high=mid-1;}
  return false;
 }};
}

/** ⚖️ The first journal row that touches what the memo read: a row naming one of the directories or lying directly inside one; the modification of a directory (its own time stamp, not its entries) matters only to the directories themselves, because the entries of its parent are unchanged. */
export function firstRelevantChangeV1(relevance:MemoRelevanceV1,rows:readonly ChangeRowV1[]):ChangeRowV1|undefined{
 for(const row of rows){
  const path=row[2],cut=path.lastIndexOf("/");
  if(relevance.holds(path)||!(row[0]==="m"&&row[1]===1)&&relevance.holds(cut<0?"":path.slice(0,cut)))return row;
 }
 return undefined;
}

/** 🧾️ The identity check of an analysed source: the stat image the fact recorded is the one the file still has. */
export function memoIdentityHoldsV1(root:string,identity:MemoIdentityV1):boolean{
 try{const status=statSync(isAbsolute(identity.path)?identity.path:join(root,identity.path));return status.size===identity.size&&status.mtimeMs===identity.mtimeMs&&status.ctimeMs===identity.ctimeMs&&status.ino===identity.ino;}catch{return false;}
}

const atomic=(target:string,data:string|Uint8Array):void=>{const temporary=target+"."+process.pid+"."+Date.now()+".tmp";try{writeFileSync(temporary,data,{mode:0o600});renameSync(temporary,target);}catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}};
const readBounded=(target:string,maximum:number):Buffer|undefined=>{try{if(statSync(target).size>maximum)return undefined;return readFileSync(target);}catch{return undefined;}};
const named=(directory:string,name:string,suffix:string):string=>join(directory,name+suffix);
const pointerOf=(bytes:Buffer|undefined):Readonly<{generation:number;manifest:string}>|undefined=>{try{const value=JSON.parse(bytes!.toString("utf8")) as {generation?:unknown;manifest?:unknown};return Number.isSafeInteger(value.generation)&&isSha(value.manifest)?{generation:value.generation as number,manifest:value.manifest}:undefined;}catch{return undefined;}};

/** 🧱️ The ports of one memo: the journal client, the record directory, the workspace root, the identity of the request and the engine check of the rows out of the journal's sight. */
export type GraphMemoOptionsV1=Readonly<{
 root:string;directory:string;client:ChangeJournalClientV1;key:string;insensitive?:boolean;now?:()=>number;signal?:AbortSignal;environment?:Readonly<Record<string,string|undefined>>;
 holds?:(rows:readonly MemoRowV1[],signal?:AbortSignal)=>Promise<boolean>;
}>;
/** 🪟️ One request view of the memo. */
export interface GraphMemoV1{
 /** 📖️ Answers from a sealed record when the journal proves that nothing the planned generation read has changed. */
 lookup():Promise<MemoAnswerV1>;
 /** 🔏️ Keeps the slice a planner child produced, after it exited cleanly. With the child's `sidecar` (its relevance of the generation) the record is sealed; without it only a light record is kept, which the next identical request turns into a sealed one. */
 seal(input:Readonly<{plan:string;pin:string|undefined;projects:number;sidecar?:string}>):Promise<MemoReasonV1|"sealed"|"light">;
 /** 🧲️ The generation a light record kept for this request: the child of the next run computes its relevance only when it serves this one. */
 expected():string|undefined;
 /** 📍️ The journal position taken before the planner child started (`undefined` when the journal cannot vouch). */
 readonly cursor:string|undefined;
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
 let cursorText:string|undefined;
 try{const taken=await client.cursor(signal);if(!("exact" in taken))cursorText=formatChangeCursorV1(taken);}catch(error){signal?.throwIfAborted();}
 const recordFile=named(directory,key.slice(0,40),policy.recordSuffix);
 const readRecord=():Record<string,any>|undefined=>{
  const bytes=readBounded(recordFile,policy.maximumRecordBytes);if(!bytes)return undefined;
  try{
   const value=JSON.parse(bytes.toString("utf8")) as Record<string,any>;
   if(value.version!==1||value.key!==key||typeof value.cursor!=="string"||!parseChangeCursorV1(value.cursor)||!isSha(value.manifest)||!isSha(value.plan?.sha256)||typeof value.pin!=="string")return undefined;
   if(value.state==="sealed"&&!(typeof value.pointer?.file==="string"&&Number.isSafeInteger(value.pointer?.generation)&&value.pointer?.manifest===value.manifest&&isSha(value.relevance?.id)&&Number.isSafeInteger(value.relevance?.rechecked)))return undefined;
   return value.state==="sealed"||value.state==="light"?value:undefined;
  }catch{return undefined;}
 };
 return{
  cursor:cursorText,
  expected:()=>readRecord()?.manifest,
  async lookup(){
   const miss=(reason:MemoReasonV1,detail?:string):MemoAnswerV1=>({hit:false,reason,...(detail===undefined?{}:{detail})});
   try{
    const record=readRecord();if(!record)return miss("none");
    if(record.state==="light")return miss("light");
    const pointer=pointerOf(readBounded(record.pointer.file,65536));if(!pointer||pointer.generation!==record.pointer.generation||pointer.manifest!==record.pointer.manifest)return miss("pointer");
    const answer=await client.since(parseChangeCursorV1(record.cursor)!,[""],signal);
    if(!answer.exact)return miss("journal",answer.reason);
    const relBytes=readBounded(named(directory,record.relevance.id,policy.relevanceSuffix),policy.maximumRelevanceBytes),checkBytes=readBounded(named(directory,record.relevance.id,policy.recheckSuffix),policy.maximumRecheckBytes),relevance=relBytes&&unpackMemoRelevanceV1(relBytes);
    if(!relBytes||!checkBytes||!relevance||relevance.insensitive!==insensitive||sha256(Buffer.concat([relBytes,Buffer.from("\n"),checkBytes]))!==record.relevance.id)return miss("generation");
    const changed=firstRelevantChangeV1(relevance,answer.rows);if(changed)return miss("changed",changed[2]);
    let rechecked=0;
    if(record.relevance.rechecked>0){
     const recheck=JSON.parse(checkBytes.toString("utf8")) as MemoRecheckV1;
     if(!options.holds)return miss("generation");
     rechecked=recheck.rows.length+recheck.identities.length;
     if(!recheck.identities.every(identity=>memoIdentityHoldsV1(root,identity))||!await options.holds(recheck.rows,signal))return miss("stale");
    }
    const planBytes=readBounded(named(directory,record.plan.sha256,policy.planSuffix),policy.maximumPlanBytes);
    if(!planBytes||planBytes.length!==record.plan.bytes||sha256(planBytes)!==record.plan.sha256)return miss("plan");
    const at=now(),pin=reown(record.pin,environment,at);
    if(pin===undefined)return miss("pin");
    try{atomic(recordFile,JSON.stringify({...record,cursor:formatChangeCursorV1(answer.cursor)}));}catch{}
    return{hit:true,plan:planBytes.toString("utf8"),pin,projects:record.projects,ageMilliseconds:Math.max(0,at-record.sealedAtMs),rows:record.relevance.rows,rechecked};
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
     const pointer=pointerOf(readBounded(sealed.pointer.file,65536)),stem=input.sidecar!.replace(/\.json$/u,""),relBytes=readBounded(stem+policy.relevanceSuffix,policy.maximumRelevanceBytes),checkBytes=readBounded(stem+policy.recheckSuffix,policy.maximumRecheckBytes);
     if(pointer&&pointer.generation===sealed.pointer.generation&&pointer.manifest===manifest&&relBytes&&checkBytes&&unpackMemoRelevanceV1(relBytes)){
      const id=sha256(Buffer.concat([relBytes,Buffer.from("\n"),checkBytes]));
      if(!existsSync(named(directory,id,policy.relevanceSuffix))){atomic(named(directory,id,policy.recheckSuffix),checkBytes);atomic(named(directory,id,policy.relevanceSuffix),relBytes);}
      const kept=readRecord(),own=parseChangeCursorV1(cursorText)!,older=kept&&parseChangeCursorV1(kept.cursor);
      if(!(kept&&kept.state==="sealed"&&older&&older.epoch===own.epoch&&older.seq>=own.seq&&kept.plan.sha256===planSha&&kept.relevance.id===id))atomic(recordFile,JSON.stringify({...base,state:"sealed",pointer:{file:sealed.pointer.file,generation:sealed.pointer.generation,manifest},relevance:{id,rows:sealed.rows,rechecked:sealed.rechecked}}));
      collectGraphMemoV1(directory,8);
      return "sealed";
     }
    }
    atomic(recordFile,JSON.stringify({...base,state:"light"}));
    collectGraphMemoV1(directory,8);
    return "light";
   }catch{return "error";}
  },
 };
}

/** 🧹️ Keeps the newest `keep` records of a memo directory and removes every plan and relevance file no kept record names (and no younger than a minute). */
export function collectGraphMemoV1(directory:string,keep:number):void{
 try{
  const policy=GRAPH_MEMO_POLICY_V1,names=readdirSync(directory),records=names.filter(name=>name.endsWith(policy.recordSuffix)).map(name=>({name,time:statSync(join(directory,name)).mtimeMs})).sort((a,b)=>b.time-a.time),alive=new Set<string>();
  for(const [index,{name}] of records.entries()){
   if(index>=keep){rmSync(join(directory,name),{force:true});continue;}
   try{const value=JSON.parse(readFileSync(join(directory,name),"utf8")) as {relevance?:{id?:string};plan?:{sha256?:string}};if(value.relevance?.id){alive.add(String(value.relevance.id)+policy.relevanceSuffix);alive.add(String(value.relevance.id)+policy.recheckSuffix);}alive.add(String(value.plan?.sha256)+policy.planSuffix);}catch{}
  }
  for(const name of names)if(!name.endsWith(policy.recordSuffix)&&!alive.has(name)){try{if(Date.now()-lstatSync(join(directory,name)).mtimeMs>60000)rmSync(join(directory,name),{force:true});}catch{}}
 }catch{}
}
