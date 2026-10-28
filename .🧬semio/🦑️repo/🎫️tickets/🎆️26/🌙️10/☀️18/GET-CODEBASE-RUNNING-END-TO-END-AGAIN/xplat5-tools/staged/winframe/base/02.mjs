import{createHash,randomBytes}from"node:crypto";
import{brotliCompressSync,brotliDecompressSync,constants as zlibConstants}from"node:zlib";
import{lstat,mkdir,open,readdir,readFile,rename,rm}from"node:fs/promises";
import{dirname,join,sep}from"node:path";
import{createProjectFileIndexV1}from"../🧭️file-owner/🟨️.mjs";
import{graphStringInternerV1,internJsonStringsV1}from"../../🧠️memory/🟨️.mjs";
import{canonicalJsonV1,compareCodePointsV1,decodeFactIndexV1,encodeFactIndexV1,openFactIndexV1}from"./🧮️fact-codec/🟨️.mjs";
import{coverageOfV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"./🔭️coverage/🟨️.mjs";
import{derivedUpsertsV1,holdersOfDerivedV1,isDerivedOwnerV1,orphanedDerivedOwnersV1,scopeWithDerivedV1,settleSuspectsV1,splitStaleConsumersV1,unsupportedDerivedV1,verifiableRowIdsV1}from"./✂️cutoff/🟨️.mjs";

export{canonicalJsonV1,compareCodePointsV1};

/** 🕰️ The declared names and limits of the graph generation store; the schema file declares the same constants and a law keeps both equal. */
export const GENERATION_POLICY_V1=Object.freeze({
 version:1,pointer:"current.json",manifests:"m",fragments:"o",indexes:"x",frontiers:"f",envOwner:"@env",
 policies:Object.freeze(["fresh","current"]),
 retainGenerations:4,graceMilliseconds:600000,
 verifyChunkRows:4096,verifyParallelChunks:4,bisectBudgetCalls:64,racyMilliseconds:2000,readConcurrency:32,
 publishWaitMilliseconds:30000,pollMaximumMilliseconds:250,
 maximumOwners:65536,maximumManifestBytes:134217728,maximumFragmentBytes:67108864,maximumIndexBytes:536870912,
 compactionFactor:2,compactionFloor:1024,compactionGrowth:1.25,brotliQuality:5,brotliWindow:22, reasonRows:16,
 stampVerification:true,waitNoticeMilliseconds:20000,sweepWaitMilliseconds:120000,hintMaximumAgeMilliseconds:1000,
});
const P=GENERATION_POLICY_V1,HEX=/^[a-f0-9]{64}$/u,sha256=bytes=>createHash("sha256").update(bytes).digest("hex"),isObject=value=>value!==null&&typeof value==="object"&&!Array.isArray(value);

/** 📦️ Refuses canonical UTF8 and compressed documents beyond the same finite reader byte ceiling before publication. */
export function packGenerationDocumentV1(value,maximumBytes){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>P.maximumIndexBytes)throw fail("Finite generation document byte limit required");
 const text=canonicalJsonV1(value),expandedBytes=Buffer.byteLength(text,"utf8");if(expandedBytes>maximumBytes)throw fail("Stored document exceeds its finite limit ("+expandedBytes+" bytes expanded, limit "+maximumBytes+")");
 const bytes=brotliCompressSync(Buffer.from(text,"utf8"),{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:P.brotliQuality,[zlibConstants.BROTLI_PARAM_LGWIN]:P.brotliWindow,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:expandedBytes}});
 if(bytes.length>maximumBytes)throw fail("Stored document exceeds its finite limit ("+bytes.length+" bytes packed, limit "+maximumBytes+")");return bytes;
}
/** 🗜️ Packs the logical fact index into its binary document (`🧮️fact-codec`) and brotli: refuses an expanded or packed size beyond the finite reader byte ceiling before publication and answers the expanded size. */
export function packFactIndexV1(index,maximumBytes){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>P.maximumIndexBytes)throw fail("Finite generation document byte limit required");
 const raw=encodeFactIndexV1(index,{maximumBytes}),bytes=brotliCompressSync(raw,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:P.brotliQuality,[zlibConstants.BROTLI_PARAM_LGWIN]:P.brotliWindow,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:raw.length}});
 if(bytes.length>maximumBytes)throw fail("Stored document exceeds its finite limit ("+bytes.length+" bytes packed, limit "+maximumBytes+")");return{bytes,expandedBytes:raw.length};
}
/** 📖️ Reads a packed fact index within the finite reader ceiling straight from the binary document, without any intermediate text. */
export function unpackFactIndexV1(bytes,maximumBytes){return readFactIndexV1(decodeFactIndexV1(brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),{maximumBytes,share:text=>graphStringInternerV1.share(text)}));}
/** 🪶️ Opens a packed fact index for verification only: strings, facts and rows stay in the document until they are read (`🧮️fact-codec` `openFactIndexV1`). */
export function openPackedFactIndexV1(bytes,maximumBytes){return openFactIndexV1(brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),{maximumBytes,share:text=>graphStringInternerV1.share(text)});}
const unpack=(bytes,limit)=>JSON.parse(brotliDecompressSync(bytes,{maxOutputLength:limit}).toString("utf8"));
const aborted=signal=>signal?.throwIfAborted();
const fail=(message,code)=>Object.assign(Error(message),code?{code}:{});

function requireKeys(value,required,optional,what){
 if(!isObject(value))throw fail("Closed "+what+" required");
 for(const key of required)if(!Object.hasOwn(value,key))throw fail("Closed "+what+" required: missing "+key);
 for(const key of Object.keys(value))if(!required.includes(key)&&!optional.includes(key))throw fail("Closed "+what+" required: unknown "+key);
}
const requireHex=(value,what)=>{if(typeof value!=="string"||!HEX.test(value))throw fail("Digest required: "+what);};
const requireCount=(value,what,maximum=Number.MAX_SAFE_INTEGER)=>{if(!Number.isSafeInteger(value)||value<0||value>maximum)throw fail("Bounded count required: "+what);};
const requireText=(value,what,maximum=4096)=>{if(typeof value!=="string"||!value||value.length>maximum)throw fail("Bounded text required: "+what);};

/** 📌️ Admits the closed pointer to the newest generation: a manifest digest, or null after an explicit invalidation. */
export function readGenerationPointerV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","generation","manifest","parent"],["reason","verified"],"generation pointer");
 if(value.version!==1)throw fail("Generation pointer version 1 required");
 requireCount(value.generation,"generation");
 if(value.manifest!==null)requireHex(value.manifest,"manifest");
 if(value.parent!==null)requireHex(value.parent,"parent");
 if(value.reason!==undefined)requireText(value.reason,"reason",256);
 if(value.verified!==undefined)readVerifiedStampV1(value.verified);
 return Object.freeze(value);
}

/** 📜️ Admits one closed immutable generation manifest. */
export function readGenerationManifestV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","generation","parent","identity","implementation","env","membership","frontier","factIndex","global","owners","built"],[],"generation manifest");
 if(value.version!==1)throw fail("Generation manifest version 1 required");
 requireCount(value.generation,"generation");
 if(value.parent!==null)requireHex(value.parent,"parent");
 for(const key of["identity","implementation","env","membership","factIndex","global"])requireHex(value[key],key);
 if(value.frontier!==null)requireHex(value.frontier,"frontier");
 if(!isObject(value.owners)||Object.keys(value.owners).length>P.maximumOwners)throw fail("Bounded owner table required");
 for(const[name,row]of Object.entries(value.owners)){
  requireText(name,"owner name");requireKeys(row,["root","kind","fragment","bytes","edgesEnv","targets"],["config","nativeRoot","tag"],"generation owner");
  requireText(row.root,"owner root");if(!["project","unavailable"].includes(row.kind))throw fail("Closed owner kind required");
  requireHex(row.fragment,"fragment");requireHex(row.edgesEnv,"edgesEnv");requireCount(row.bytes,"fragment bytes",P.maximumFragmentBytes);
  if(!Array.isArray(row.targets)||row.targets.some(target=>typeof target!=="string"))throw fail("Target name list required");
  if(row.config!==undefined)requireText(row.config,"owner config");if(row.nativeRoot!==undefined)requireText(row.nativeRoot,"owner native root");if(row.tag!==undefined)requireText(row.tag,"owner tag",64);
 }
 requireKeys(value.built,["kind","base","dirty"],[],"generation provenance");
 if(!["cold","delta","refresh"].includes(value.built.kind))throw fail("Closed provenance kind required");
 if(value.built.base!==null)requireCount(value.built.base,"base generation");
 if(!Array.isArray(value.built.dirty)||value.built.dirty.some(name=>typeof name!=="string"))throw fail("Dirty owner list required");
 return Object.freeze(value);
}

/** 🧩️ Admits one closed owner fragment: the neutral node, its outgoing edges and the environment those edges were computed under. */
export function readGenerationFragmentV1(value){
 requireKeys(value,["version","name","root","kind","node","edges","edgesEnv","targets"],["config","nativeRoot"],"generation fragment");
 if(value.version!==1)throw fail("Generation fragment version 1 required");
 requireText(value.name,"fragment name");requireText(value.root,"fragment root");requireHex(value.edgesEnv,"edgesEnv");
 if(!["project","unavailable"].includes(value.kind)||!Array.isArray(value.edges))throw fail("Closed fragment required");
 return value;
}

function readFactIndexV1(value){
 requireKeys(value,["version","owners","rows","classes"],[],"fact index");
 if(value.version!==1||!Array.isArray(value.owners)||!Array.isArray(value.rows)||!Array.isArray(value.classes))throw fail("Closed fact index required");
 const owners=value.owners.length,rows=value.rows.length;
 for(const cls of value.classes){
  requireKeys(cls,["r","o"],[],"fact class");
  if(!Array.isArray(cls.r)||!Array.isArray(cls.o)||cls.r.some(id=>!Number.isSafeInteger(id)||id<0||id>=rows)||cls.o.some(id=>!Number.isSafeInteger(id)||id<0||id>=owners))throw fail("Fact class references out of range");
 }
 return value;
}

const rowKeys=new WeakMap();
/** 🪪️ The identity of one observation row: its canonical text, or `#` and the SHA-256 of that text when the text is longer than `ROW_KEY_TEXT_LIMIT_V1` (an analysis row carries kilobytes of nested facts, and a build keys every row of a graph: the key stays below 70 bytes however large the fact is); two rows have one key exactly when their canonical texts are equal, and a shared row object is keyed once. */
export function rowKeyV1(row){let key=rowKeys.get(row);if(key===undefined){const text=canonicalJsonV1([row.kind,row.path,row.fact]);key=text.length<=ROW_KEY_TEXT_LIMIT_V1?text:"#"+sha256(text);rowKeys.set(row,key);}return key;}
export const ROW_KEY_TEXT_LIMIT_V1=128;

const ownerIdMaps=new WeakMap();
const ownerIdMap=index=>{let map=ownerIdMaps.get(index);if(!map){map=new Map(index.owners.map((name,id)=>[name,id]));ownerIdMaps.set(index,map);}return map;};

/** 🗃️ Builds the inverted fact index: rows are interned once, and owners partition the pairs into classes of rows that share exactly the same consumers, so one stale row names its consumers without reading any fragment. */
export function createFactIndexV1(ownerRows){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),lists=[],empty=[];let seen=new Int32Array(1<<16);
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name),mine=new Uint32Array(list.length);let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+1));grown.set(seen);seen=grown;}
   if(seen[id]!==owner+1){seen[id]=owner+1;mine[count++]=id;}
  }
  if(!count)empty.push({r:[],o:[owner]});
  lists.push(mine.subarray(0,count));
 });
 const first=new Uint32Array(rows.length+1);
 for(const list of lists)for(const id of list)first[id+1]++;
 for(let id=0;id<rows.length;id++)first[id+1]+=first[id];
 const flat=new Uint32Array(first[rows.length]),fill=first.slice(0,rows.length);
 lists.forEach((list,owner)=>{for(const id of list)flat[fill[id]++]=owner;});
 const buckets=new Map(),classes=[];
 for(let id=0;id<rows.length;id++){
  const from=first[id],to=first[id+1];let hash=to-from;
  for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^flat[at])>>>0;
  let chain=buckets.get(hash),group;
  if(chain===undefined)buckets.set(hash,chain=[]);
  for(const candidate of chain){
   const other=candidate.span;if(other[1]-other[0]!==to-from)continue;
   let same=true;for(let at=0;at<to-from;at++)if(flat[other[0]+at]!==flat[from+at]){same=false;break;}
   if(same){group=candidate;break;}
  }
  if(group===undefined){group={r:[],o:Array.from(flat.subarray(from,to)),span:[from,to]};chain.push(group);classes.push(group);}
  group.r.push(id);
 }
 return{version:1,owners,rows,classes:[...classes.map(({r,o})=>({r,o})),...empty]};
}

/** 🩹️ Replaces the consumers in `upserts` (their old classes disappear), adds rows to the consumers in `append`, drops `drop`, prunes unreferenced rows and rebuilds the partition once classes outgrow the owners. */
export function patchFactIndexV1(index,{drop=[],upserts=new Map(),append=new Map()}){
 const ownerIds=new Map(ownerIdMap(index)),rows=index.rows.slice(),owners=index.owners.slice(),original=index.rows.length,byPath=pathIndexOf(index),added=new Map();
 const idOf=name=>{let id=ownerIds.get(name);if(id===undefined){id=owners.length;owners.push(name);ownerIds.set(name,id);}return id;};
 const touched=new Set(),gone=new Set();
 for(const name of drop)if(ownerIds.has(name)){touched.add(ownerIds.get(name));if(!upserts.has(name))gone.add(ownerIds.get(name));}
 for(const name of upserts.keys())touched.add(idOf(name));
 const classes=[];
 for(const cls of index.classes){const kept=cls.o.filter(id=>!touched.has(id));if(kept.length)classes.push({r:cls.r,o:kept});}
 const idOfRow=row=>{
  const key=rowKeyV1(row);let id=added.get(key);if(id!==undefined)return id;
  const candidates=byPath.get(row.path);
  if(candidates!==undefined)for(const candidate of typeof candidates==="number"?[candidates]:candidates)if(rows[candidate].kind===row.kind&&rowKeyV1(rows[candidate])===key)return candidate;
  id=rows.length;rows.push(row);added.set(key,id);return id;
 };
 const rowIdsFor=list=>{const out=new Set();for(const row of list)out.add(idOfRow(row));return[...out].sort((a,b)=>a-b);};
 for(const name of[...upserts.keys()].sort(compareCodePointsV1))classes.push({r:rowIdsFor(upserts.get(name)),o:[ownerIds.get(name)]});
 for(const name of[...append.keys()].sort(compareCodePointsV1)){const extra=rowIdsFor(append.get(name));classes.push({r:extra,o:[idOf(name)]});}
 const used=new Uint8Array(rows.length),live=new Uint8Array(owners.length);
 for(const cls of classes){for(const id of cls.r)used[id]=1;for(const id of cls.o)if(!gone.has(id))live[id]=1;}
 const rowMap=new Int32Array(rows.length).fill(-1),ownerMap=new Int32Array(owners.length).fill(-1),rowsOut=[],ownersOut=[];
 for(let id=0;id<rows.length;id++)if(used[id]){rowMap[id]=rowsOut.length;rowsOut.push(rows[id]);}
 for(let id=0;id<owners.length;id++)if(live[id]){ownerMap[id]=ownersOut.length;ownersOut.push(owners[id]);}
 const compact=classes.filter(cls=>cls.o.some(id=>live[id])).map(cls=>({r:cls.r.map(id=>rowMap[id]),o:cls.o.filter(id=>live[id]).map(id=>ownerMap[id])}));
 const result={version:1,owners:ownersOut,rows:rowsOut,classes:compact};
 return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor&&result.classes.length>P.compactionGrowth*index.classes.length?refineFactIndexV1(result):result;
}

const pathIndexes=new WeakMap();
/** 🔎️ The rows of an index by path (a row id, or the ids of the rows that share the path), read once per index: finds the stored twin of a row from its path and kind, so a patch compares the facts of a handful of candidates instead of building the identity of every stored row. */
function pathIndexOf(index){
 let map=pathIndexes.get(index);if(map)return map;
 map=new Map();
 for(let id=0;id<index.rows.length;id++){const path=index.rows[id].path,held=map.get(path);if(held===undefined)map.set(path,id);else if(typeof held==="number")map.set(path,[held,id]);else held.push(id);}
 pathIndexes.set(index,map);return map;
}

/** ⚖️ Merges the classes of an index into the coarsest exact partition: rows with the same consumers share one class. */
export function refineFactIndexV1(index){
 const rowCount=index.rows.length,start=new Uint32Array(rowCount+1);
 for(const cls of index.classes)for(const id of cls.r)start[id+1]+=cls.o.length;
 for(let id=0;id<rowCount;id++)start[id+1]+=start[id];
 const flat=new Uint32Array(start[rowCount]),fill=start.slice(0,rowCount);
 for(const cls of index.classes)for(const id of cls.r)for(const owner of cls.o)flat[fill[id]++]=owner;
 const groups=new Map(),classes=[];
 for(let id=0;id<rowCount;id++){
  const list=flat.subarray(start[id],start[id+1]).sort();let unique=0;
  for(let at=0;at<list.length;at++)if(at===0||list[at]!==list[at-1])list[unique++]=list[at];
  const owners=Array.from(list.subarray(0,unique)),key=owners.join(",");let group=groups.get(key);
  if(!group){group={r:[],o:owners};groups.set(key,group);classes.push(group);}
  group.r.push(id);
 }
 const covered=new Set(classes.flatMap(cls=>cls.o));
 for(const cls of index.classes)for(const owner of cls.o)if(!covered.has(owner)){covered.add(owner);classes.push({r:[],o:[owner]});}
 return{version:1,owners:index.owners,rows:index.rows,classes};
}

/** 📚️ The sorted consumers of every row of the index, as `[rowKey, names]` pairs (a pure view for laws). */
export function factConsumersV1(index){
 const consumers=index.rows.map(()=>new Set());
 for(const cls of index.classes)for(const id of cls.r)for(const owner of cls.o)consumers[id].add(index.owners[owner]);
 return index.rows.map((row,id)=>[rowKeyV1(row),[...consumers[id]].sort(compareCodePointsV1)]);
}

/** 🔎️ Finds exactly the stale rows among `ids`: a chunk that holds is cleared by one call, a failing chunk is bisected within a call budget and then checked row by row. */
export async function staleFactRowsV1(index,ids,hold,{signal,progress,chunk=P.verifyChunkRows,parallel=P.verifyParallelChunks,budget=P.bisectBudgetCalls}={}){
 const stale=new Set(),chunks=[];let done=0,calls=0,next=0;
 for(let at=0;at<ids.length;at+=chunk)chunks.push(ids.slice(at,at+chunk));
 const check=async list=>{aborted(signal);calls++;return hold(list.map(id=>index.rows[id]),signal);};
 const find=async(list,state)=>{
  if(await check(list))return;
  if(list.length===1){stale.add(list[0]);return;}
  if(state.left<=0){for(const id of list)if(!await check([id]))stale.add(id);return;}
  state.left-=2;const mid=list.length>>1;await find(list.slice(0,mid),state);await find(list.slice(mid),state);
 };
 await progress?.({phase:"facts",done:0,total:ids.length});
 await Promise.all(Array.from({length:Math.min(parallel,chunks.length)},async()=>{
  while(next<chunks.length){const list=chunks[next++];await find(list,{left:budget});done+=list.length;await progress?.({phase:"facts",done,total:ids.length});}
 }));
 return{stale,calls};
}

/** 🧭️ Selects the classes that name one of `only` (every class without it) and the distinct rows they list. */
export function selectFactClassesV1(index,only){
 const map=ownerIdMap(index),wanted=only?new Set([...only].map(name=>map.get(name)).filter(id=>id!==undefined)):undefined,classes=[],used=new Uint8Array(index.rows.length),ids=[];
 index.classes.forEach((cls,at)=>{if(wanted&&!cls.o.some(id=>wanted.has(id)))return;classes.push(at);for(const id of cls.r)if(!used[id]){used[id]=1;ids.push(id);}});
 return{classes,ids};
}

/** 🧱️ The consumers of the stale rows within the selected classes. */
export function staleConsumersV1(index,classes,stale){
 const names=new Set();
 for(const at of classes){const cls=index.classes[at];if(cls.r.some(id=>stale.has(id)))for(const owner of cls.o)names.add(index.owners[owner]);}
 return names;
}

/** 🗺️ Captures the membership frontier: every directory with its entry stamp (`[path, mtimeMs, ino]`) and every member with its mode (`[path, mode]`), so unchanged membership is proven by stat calls instead of a walk. */
export function createFrontierV1({directories,members,capturedAt}){
 return{version:1,capturedAt,directories:[...directories].sort((a,b)=>compareCodePointsV1(a[0],b[0])),members:[...members].sort((a,b)=>compareCodePointsV1(a[0],b[0]))};
}

/** 🚪️ True when no directory entry stamp and no member mode moved since the capture; a directory touched within the racy window of the capture never counts as proven. */
export async function frontierHoldsV1(frontier,stat,{signal,concurrency=P.readConcurrency*2,racyMilliseconds=P.racyMilliseconds}={}){
 const rows=[...frontier.directories.map(row=>({row,directory:true})),...frontier.members.map(row=>({row,directory:false}))];
 let next=0,ok=true;
 await Promise.all(Array.from({length:Math.min(concurrency,rows.length)},async()=>{
  while(ok&&next<rows.length){
   aborted(signal);const{row,directory}=rows[next++];let status;
   try{status=await stat(row[0]);}catch{ok=false;return;}
   if(!status){ok=false;return;}
   if(directory){if(!status.directory||status.mtimeMs!==row[1]||status.ino!==row[2]||frontier.capturedAt-row[1]<racyMilliseconds)ok=false;}
   else if(!status.file||status.mode!==row[1])ok=false;
  }
 }));
 return ok;
}

/** 🧱️ The directories whose entry sets are proven unchanged since `before` was captured: same stamp and inode in `after`, and not touched within the racy window of the earlier capture (a change in the same clock tick would be invisible). A recorded listing, type or absence of an entry of such a directory cannot have moved, because an entry appears, vanishes or changes type only by changing its parent. */
export function stableDirectoriesV1(before,after,{racyMilliseconds=P.racyMilliseconds}={}){
 const old=new Map(before.directories.map(row=>[row[0],row])),stable=new Set();
 for(const row of after.directories){const was=old.get(row[0]);if(was&&was[1]===row[1]&&was[2]===row[2]&&before.capturedAt-was[1]>=racyMilliseconds)stable.add(row[0]);}
 return stable;
}

/** ➗️ Names the paths that appeared, vanished or changed between two frontiers (directories whose stamp moved are `changed`, members whose mode moved are `changed`). */
export function diffFrontiersV1(before,after){
 const changes=[];
 for(const directory of[true,false]){
  const old=new Map((directory?before.directories:before.members).map(row=>[row[0],row])),next=new Map((directory?after.directories:after.members).map(row=>[row[0],row]));
  for(const[path,row]of next){const was=old.get(path);if(!was)changes.push({path,kind:"added",directory});else if(directory?was[1]!==row[1]||was[2]!==row[2]:was[1]!==row[1])changes.push({path,kind:"changed",directory});}
  for(const path of old.keys())if(!next.has(path))changes.push({path,kind:"removed",directory});
 }
 return changes.sort((a,b)=>compareCodePointsV1(a.path,b.path));
}

/** 🗂️ The filesystem port of the store on real files: a missing file reads as undefined, writes are atomic by rename and a content addressed write that finds its file does nothing. */
export function createNodeFilePortV1(){
 return Object.freeze({
  async read(path){try{return await readFile(path);}catch(error){if(error?.code==="ENOENT"||error?.code==="ENOTDIR")return undefined;throw error;}},
  async write(path,bytes,{ifAbsent=false}={}){
   if(ifAbsent){try{await lstat(path);return false;}catch(error){if(error?.code!=="ENOENT")throw error;}}
   await mkdir(dirname(path),{recursive:true});
   const temporary=path+"."+randomBytes(8).toString("hex")+".tmp";
   try{const file=await open(temporary,"wx");try{await file.writeFile(bytes);}finally{await file.close();}await rename(temporary,path);}finally{await rm(temporary,{force:true});}
   return true;
  },
  async names(directory){try{return await readdir(directory);}catch(error){if(error?.code==="ENOENT")return[];throw error;}},
  async list(directory){
   let names;try{names=await readdir(directory);}catch(error){if(error?.code==="ENOENT")return[];throw error;}
   const rows=await Promise.all(names.map(async name=>{try{const status=await lstat(join(directory,name));return status.isFile()?{name,modifiedMs:status.mtimeMs}:undefined;}catch(error){if(error?.code==="ENOENT")return undefined;throw error;}}));
   return rows.filter(Boolean);
  },
  async remove(path){await rm(path,{force:true});},
 });
}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

async function mapLimit(items,limit,visit,signal){
 const out=new Array(items.length);let next=0;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{while(next<items.length){aborted(signal);const at=next++;out[at]=await visit(items[at],at);}}));
 return out;
}

/** 🚧️ The withheld owners of a generation that do not hold the source that withholds them: an untagged `unavailable` owner names the file that does not parse (`config`), and it stays valid only while the bytes of that file are the ones it was withheld for, so a generation that carries no fact about that file (a withholding recorded before its facts were kept) can never notice the repair and its owner must be inferred again. Sources outside the workspace and tagged owners (rediscovered as a group) are not judged. The judgement is global: a repair is bounded by the withheld owners and one build heals all of them, instead of one build per layer of the closure they open. */
export function unfoundedWithheldOwnersV1(index,owners,separator=sep){
 const open=new Map(),kinds=new Set(["file","analysis","digest"]);
 for(const[name,row]of Object.entries(owners)){
  if(row.kind!=="unavailable"||row.tag||typeof row.config!=="string"||row.config.startsWith(".."))continue;
  const at=index.owners.indexOf(name);if(at>=0)open.set(at,[name,row.config]);
 }
 for(const cls of index.classes){
  if(!open.size)break;
  const here=cls.o.filter(at=>open.has(at));if(!here.length)continue;
  const configs=new Set(here.map(at=>open.get(at)[1]));
  for(const id of cls.r){
   const row=index.rows[id];if(!kinds.has(row.kind))continue;
   const path=separator==="/"?row.path:row.path.split(separator).join("/");
   for(const config of configs)if(path===config||path.endsWith("/"+config)){for(const at of here)if(open.get(at)?.[1]===config)open.delete(at);configs.delete(config);}
   if(!configs.size)break;
  }
 }
 return[...open.values()].map(([name])=>name);
}

/**
 * 🕰️ Creates the generation store over `root`: readers never take a lease; a builder publishes the next immutable generation with one compare-and-swap of the pointer under an exclusive hold that covers one small read and one rename.
 * `holds(rows, signal)` answers whether every observation row still holds; `membership(previous, signal)` answers `{digest, frontier, changes}` against the previous frontier (`changes` is meaningful only with a previous frontier); `compute(spec, signal)` refreshes exactly the dirty owners.
 */
export function createGenerationStoreV1({root,fs=createNodeFilePortV1(),lease,holds,derive,resources,now=()=>Date.now(),sleep=defaultSleep,stamp=P.stampVerification}){
 if(typeof root!=="string"||!root||typeof holds!=="function"||!resources?.facts)throw fail("Generation store root, fact check and facts resource required");
 const stats={holdCalls:0,rowsChecked:0,computed:0,published:0,casConflicts:0,publishHeldMs:[],indexLoads:0,fragmentReads:0,indexViews:0};
 const at=(directory,digest,extension)=>join(root,directory,digest+extension),pointerPath=join(root,P.pointer),indexCache=new Map();
 const manifestPath=digest=>at(P.manifests,digest,".json"),fragmentPath=digest=>at(P.fragments,digest,".json.br"),indexPath=digest=>at(P.indexes,digest,".facts.br"),frontierPath=digest=>at(P.frontiers,digest,".json.br");
 const hold=async(rows,signal)=>{stats.holdCalls++;stats.rowsChecked+=rows.length;return holds(rows,signal);};
 const readPointer=async()=>{const bytes=await fs.read(pointerPath);return bytes?readGenerationPointerV1(bytes):undefined;};

 /** 📖️ Reads the newest sealed generation without any lease; a manifest retired between the two reads is followed by re-reading the pointer, so a result is always one complete generation. */
 async function read(signal){
  for(let attempt=0;attempt<8;attempt++){
   aborted(signal);const pointer=await readPointer();if(!pointer)return undefined;
   if(pointer.manifest===null)return{pointer,manifest:undefined};
   const bytes=await fs.read(manifestPath(pointer.manifest));
   if(!bytes){if(attempt===7)return{pointer,manifest:undefined,missing:true};await sleep(5*(attempt+1),signal);continue;}
   if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==pointer.manifest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");
   let manifest;try{manifest=readGenerationManifestV1(bytes);}catch{return{pointer,manifest:undefined};}if(manifest.generation!==pointer.generation)throw fail("Generation manifest differs from its pointer","GENERATION_CORRUPT");
   return{pointer,manifest};
  }
  throw fail("Generation manifest unreadable","GENERATION_UNREADABLE");
 }
 /** 📌️ Reads one sealed generation by the digest of its manifest although a newer pointer replaced it; a retired manifest answers undefined. */
 async function readManifestAt(digest,signal){
  aborted(signal);requireHex(digest,"manifest");const bytes=await fs.read(manifestPath(digest));if(!bytes)return undefined;
  if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==digest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");
  try{return readGenerationManifestV1(bytes);}catch{return undefined;}
 }
 async function readIndex(digest){
  const cached=indexCache.get(digest);if(cached)return cached;
  const bytes=await fs.read(indexPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Fact index content differs from its name","GENERATION_CORRUPT");
  const index=unpackFactIndexV1(bytes,P.maximumIndexBytes);stats.indexLoads++;
  indexCache.set(digest,index);while(indexCache.size>2)indexCache.delete(indexCache.keys().next().value);return index;
 }
 async function readIndexView(digest){
  const key="view:"+digest,cached=indexCache.get(key)??indexCache.get(digest);if(cached)return cached;
  const bytes=await fs.read(indexPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Fact index content differs from its name","GENERATION_CORRUPT");
  const index=openPackedFactIndexV1(bytes,P.maximumIndexBytes);stats.indexViews++;
  indexCache.set(key,index);while(indexCache.size>2)indexCache.delete(indexCache.keys().next().value);return index;
 }
 async function readFrontier(digest){
  if(digest===null)return undefined;const bytes=await fs.read(frontierPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Frontier content differs from its name","GENERATION_CORRUPT");return unpack(bytes,P.maximumIndexBytes);
 }
 async function readFragment(row){
  const bytes=await fs.read(fragmentPath(row.fragment));if(!bytes)return undefined;stats.fragmentReads++;
  if(sha256(bytes)!==row.fragment)throw fail("Fragment content differs from its name","GENERATION_CORRUPT");
  return readGenerationFragmentV1(internJsonStringsV1(unpack(bytes,P.maximumFragmentBytes),graphStringInternerV1));
 }
 /** 🌐️ Reads the global fragment of one manifest: the external nodes, the cross-owner edges and the parse failures. */
 async function readGlobal(manifest){const bytes=await fs.read(fragmentPath(manifest.global));if(!bytes||sha256(bytes)!==manifest.global)throw fail("Global fragment is missing","GENERATION_INCOMPLETE");return unpack(bytes,P.maximumFragmentBytes).value??{};}
 /** 📇️ Lists the projects (name, root, target names) and the unavailable projects of one sealed generation from the manifest and the few unavailable fragments, without reading any project node. */
 async function catalog(current,{signal}={}){
  const manifest=current?.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const rows=Object.entries(manifest.owners),global=await readGlobal(manifest);
  const loaded=await mapLimit(rows.filter(([,row])=>row.kind==="unavailable"),P.readConcurrency,async([name,row])=>{const fragment=await readFragment(row);if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment.node;},signal);
  return{projects:rows.filter(([,row])=>row.kind==="project").map(([name,row])=>({name,root:row.root,targets:row.targets})),unavailable:[...(global.unavailable??[]),...loaded].sort((a,b)=>compareCodePointsV1(a.root,b.root))};
 }
 const keep=async(directory,bytes,extension)=>{const digest=sha256(bytes);await fs.write(at(directory,digest,extension),bytes,{ifAbsent:true});return{digest,bytes:bytes.length};};
 const store=(directory,value,extension,limit)=>keep(directory,packGenerationDocumentV1(value,limit),extension);
 const writeManifest=async manifest=>{const bytes=Buffer.from(canonicalJsonV1(readGenerationManifestV1(Buffer.from(canonicalJsonV1(manifest)))));if(bytes.length>P.maximumManifestBytes)throw fail("Generation manifest exceeds its finite limit");const digest=sha256(bytes);await fs.write(manifestPath(digest),bytes,{ifAbsent:true});return digest;};

 /**
  * 🔍️ Verifies the facts of the selected owners (every owner without a selection) and maps the observed membership changes to their owners through the roots of the manifest.
  * `changes` undefined means the caller cannot diff membership: the plan is cold.
  */
 async function plan({current,identity,changes,only,skipEnvironment=false,signal,progress}){
  const cold=reason=>({kind:"cold",reason,dirty:new Set(),added:[],removed:[],envStale:true,checked:0});
  if(!current?.manifest)return cold(current?(current.missing?"manifest":"invalidated"):"empty");
  const manifest=current.manifest;
  if(manifest.identity!==identity)return cold("identity");
  if(changes===undefined)return cold("frontier");
  const index=await readIndexView(manifest.factIndex);if(!index)return cold("index");
  const present=new Set(await fs.names(join(root,P.fragments)));if(!present.has(manifest.global+".json.br"))return cold("global");
  const dirty=new Set(),added=[],removed=new Set(),reasons=[],owners=Object.entries(manifest.owners),ownerOf=createProjectFileIndexV1(Object.fromEntries(owners.map(([name,row])=>[name,{root:row.root}])));
  const configs=new Map(owners.filter(([,row])=>row.config!==undefined).map(([name,row])=>[row.config,name]));
  let envStale=false;
  for(const change of changes){
   const known=configs.get(change.path);
   if(change.config&&change.kind==="added")added.push(change.path);
   else if(change.config&&change.kind==="removed"&&known)removed.add(known);
   else if(change.config&&known)dirty.add(known);
   else if(change.directory&&change.kind!=="removed")continue;
   else{const owner=ownerOf(change.path);if(owner!==undefined)dirty.add(owner);else{envStale=true;if(change.kind!=="removed")added.push(change.path);}}
  }
  for(const[name,row]of owners)if((!only||only.has(name))&&!present.has(row.fragment+".json.br"))dirty.add(name);
  const unfounded=unfoundedWithheldOwnersV1(index,manifest.owners);for(const name of unfounded)dirty.add(name);
  const selection=selectFactClassesV1(index,only?scopeWithDerivedV1(index,new Set(skipEnvironment?only:[...only,P.envOwner])):undefined);
  const{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),hold,{signal,progress});
  const{direct,suspects}=splitStaleConsumersV1(index,selection.classes,stale),settled=await settleSuspectsV1({index,suspects,derive,signal,progress}),changedHolders=holdersOfDerivedV1(index,settled.changed.keys());
  for(const name of direct){if(name===P.envOwner)envStale=true;else if(manifest.owners[name])dirty.add(name);}
  for(const holders of changedHolders.values())for(const name of holders)if(manifest.owners[name])dirty.add(name);
  for(const change of changes){
   if(reasons.length>=P.reasonRows)break;if(change.directory&&change.kind!=="removed")continue;
   const owner=configs.get(change.path)??ownerOf(change.path);reasons.push({change:{path:change.path,kind:change.kind},owners:[owner??P.envOwner]});
  }
  for(const id of[...stale].sort((a,b)=>a-b)){
   if(reasons.length>=P.reasonRows)break;
   const names=[];
   for(const at of selection.classes){const cls=index.classes[at];if(cls.r.includes(id))for(const owner of cls.o){const name=index.owners[owner];if(!isDerivedOwnerV1(name)&&!names.includes(name))names.push(name);if(names.length>=3)break;}if(names.length>=3)break;}
   if(!names.length)continue;
   reasons.push({row:index.rows[id],owners:names});
  }
  for(const[name,row]of settled.changed){if(reasons.length>=P.reasonRows)break;const names=[...changedHolders.get(name)??[]].sort(compareCodePointsV1).slice(0,3);if(names.length)reasons.push({row,owners:names});}
  for(const name of removed)dirty.delete(name);
  const scoped=only?new Set([...[...dirty].filter(name=>only.has(name)),...unfounded]):dirty,clean=!scoped.size&&!added.length&&!removed.size&&!envStale;
  const staleKinds={};for(const id of stale){const kind=index.rows[id].kind;staleKinds[kind]=(staleKinds[kind]??0)+1;}
  return{kind:clean?"clean":"delta",dirty:scoped,added,removed:[...removed],envStale,checked:selection.ids.length,stale:stale.size,staleKinds,reasons,cutoff:{questions:suspects.size,held:settled.held.size,changed:settled.changed.size,failed:settled.failed},refresh:settled.held};
 }

 /** 🤝️ Swaps the pointer for the next generation only if it still names the generation the builder started from; the exclusive hold covers one small read and one rename and nothing else. */
 async function publish({manifestDigest,expectedGeneration,reason,verified,signal}){
  const body=async()=>{
   const started=now(),pointer=await readPointer();
   if((pointer?.generation??-1)!==expectedGeneration){stats.casConflicts++;stats.publishHeldMs.push(now()-started);return{published:false,pointer};}
   const next={version:1,generation:expectedGeneration+1,manifest:manifestDigest,parent:pointer?.manifest??null,...(reason?{reason}:{}),...(verified?{verified}:{})};
   await fs.write(pointerPath,Buffer.from(canonicalJsonV1(next)));stats.published++;stats.publishHeldMs.push(now()-started);return{published:true,pointer:next};
  };
  return lease?lease.exclusive(resources.facts,body,{signal,timeoutMs:P.publishWaitMilliseconds}):body();
 }

 /** 🏗️ Builds and publishes the next generation from the newest one: the plan names the dirty owners, `compute` refreshes exactly those and unchanged fragments are carried over by digest. */
 async function build({identity,implementation,membership,compute,signal,progress,hint,only,observedAtMs=now(),coverage}){
  const current=await read(signal),usable=current?.manifest&&current.manifest.identity===identity?current.manifest:undefined;
  const reuse=hint&&usable&&current.pointer.generation===hint.generation&&now()-hint.atMs<=P.hintMaximumAgeMilliseconds;
  const previous=usable&&!reuse?{manifest:usable,frontier:await readFrontier(usable.frontier)}:undefined,live=reuse?hint.live:await membership(previous?.frontier?previous:undefined,signal);
  const found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,only,skipEnvironment:coverage!==undefined,signal,progress});
  if(found.kind==="clean"){
   if(usable.membership===live.digest&&!found.refresh?.size)return{current,built:false,published:false,plan:found,computed:0};
   return{...await refreshMetadata({current,live,signal,held:found.refresh}),built:false,plan:found,computed:0};
  }
  const cold=found.kind==="cold",owners0=cold?{}:usable.owners,names=cold?[]:[...found.dirty].sort(compareCodePointsV1);
  aborted(signal);
  const spec={cold,owners:names,discover:found.added,removed:found.removed,changed:cold?[]:(live.changes??[]).filter(change=>!change.directory).map(change=>change.path),previousEnv:cold?null:usable.env,envStale:found.envStale,generation:(current?.pointer.generation??-1)+1,coldReason:cold?found.reason:undefined,reasons:found.reasons??[],staleRows:found.stale??0,staleKinds:found.staleKinds,
   stubs:Object.fromEntries(Object.entries(owners0).map(([name,row])=>[name,{root:row.root,kind:row.kind,targets:row.targets,...(row.nativeRoot?{nativeRoot:row.nativeRoot}:{}),...(row.config?{config:row.config}:{}),...(row.tag?{tag:row.tag}:{})}]))};
  const result=await compute(spec,signal);
  stats.computed+=result.owners.length;aborted(signal);
  const envDigest=result.env.digest,owners={...owners0},dropped=new Set(found.removed),upserts=new Map(),append=new Map(),returned=new Set(),stored=[];
  for(const name of dropped)delete owners[name];
  for(const owner of result.owners){
   aborted(signal);
   if(owner.partial==="edges"){
    const old=owners[owner.name];if(!old)throw fail("Edge refresh requires a published owner","GENERATION_INCOMPLETE");
    stored.push(async()=>{const fragment=await readFragment(old);if(!fragment)throw fail("Edge refresh requires the published fragment","GENERATION_INCOMPLETE");const saved=await store(P.fragments,{...fragment,edges:owner.edges??await result.reEdge(owner.name,fragment),edgesEnv:envDigest},".json.br",P.maximumFragmentBytes);owners[owner.name]={...old,fragment:saved.digest,bytes:saved.bytes,edgesEnv:envDigest};});
    if(owner.rows?.length)append.set(owner.name,owner.rows);
    continue;
   }
   returned.add(owner.name);
   stored.push(async()=>{
    const body={version:1,name:owner.name,root:owner.root,kind:owner.kind,node:owner.node,edges:owner.edges,edgesEnv:envDigest,targets:owner.targets,...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{})};
    const saved=await store(P.fragments,body,".json.br",P.maximumFragmentBytes);
    owners[owner.name]={root:owner.root,kind:owner.kind,fragment:saved.digest,bytes:saved.bytes,edgesEnv:envDigest,targets:owner.targets,...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{})};
   });
   upserts.set(owner.name,owner.rows??[]);
  }
  await mapLimit(stored,P.readConcurrency,job=>job(),signal);
  for(const name of names)if(!returned.has(name)&&!dropped.has(name)){delete owners[name];dropped.add(name);}
  for(const[name,row]of Object.entries(owners))if(row.tag&&result.replaceTags?.includes(row.tag)&&!returned.has(name)){delete owners[name];dropped.add(name);}
  for(const name of returned)dropped.delete(name);
  for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==envDigest)throw fail("Every owner needs edges of the published environment: "+name,"GENERATION_INCOMPLETE");
  upserts.set(P.envOwner,result.env.rows??[]);
  for(const[name,rows]of derivedUpsertsV1(result.derived??[],found.refresh))upserts.set(name,rows);
  let index=cold?createFactIndexV1(upserts):patchFactIndexV1(await readIndex(usable.factIndex),{drop:[...dropped],upserts,append});
  const orphans=orphanedDerivedOwnersV1(index);if(orphans.length)index=patchFactIndexV1(index,{drop:orphans});
  const unsupported=unsupportedDerivedV1(index);if(unsupported.length)throw fail("Derived facts need a supporting computation: "+unsupported.slice(0,3).map(row=>row.path.slice(0,96)).join(", "),"GENERATION_INCOMPLETE");

  const packedIndex=packFactIndexV1(index,P.maximumIndexBytes),indexStored=await keep(P.indexes,packedIndex.bytes,".facts.br");
  const globalStored=await store(P.fragments,{version:1,name:"@global",value:result.global??null},".json.br",P.maximumFragmentBytes);
  const frontierStored=await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes);
  const expectedGeneration=current?.pointer.generation??-1;
  const next={version:1,generation:expectedGeneration+1,parent:current?.pointer.manifest??null,identity,implementation,env:envDigest,membership:live.digest,frontier:frontierStored.digest,factIndex:indexStored.digest,global:globalStored.digest,owners,built:{kind:cold?"cold":"delta",base:cold?null:usable.generation,dirty:[...returned].sort(compareCodePointsV1)}};
  const manifestDigest=await writeManifest(next);aborted(signal);
  const verified=stamp?mergeVerifiedStampV1({owners:Object.keys(owners).sort(compareCodePointsV1),scope:cold?undefined:only,observedAtMs,carried:cold?undefined:coverage}):undefined;
  const outcome=await publish({manifestDigest,expectedGeneration,verified,signal});
  if(!outcome.published)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};
  indexCache.set(indexStored.digest,index);
  await retire({signal}).catch(error=>{if(signal?.aborted)throw error;});
  return{current:{pointer:outcome.pointer,manifest:next},built:true,published:true,plan:found,computed:result.owners.length,index:{rows:index.rows.length,owners:index.owners.filter(name=>!isDerivedOwnerV1(name)).length,expandedBytes:packedIndex.expandedBytes,packedBytes:indexStored.bytes}};
 }

 /** 🧷️ Publishes the same owners under a refreshed membership frontier, so the next verification is a stat pass again; losing the swap is harmless. */
 async function refreshMetadata({current,live,signal,held}){
  const manifest=current.manifest,frontier=live?await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes):undefined;
  let factIndex=manifest.factIndex,patched;
  if(held?.size){const old=await readIndex(manifest.factIndex);if(old){patched=patchFactIndexV1(old,{upserts:derivedUpsertsV1([],held)});factIndex=(await keep(P.indexes,packFactIndexV1(patched,P.maximumIndexBytes).bytes,".facts.br")).digest;}}
  const next={...manifest,generation:manifest.generation+1,parent:current.pointer.manifest,membership:live?live.digest:manifest.membership,frontier:frontier?frontier.digest:manifest.frontier,factIndex,built:{kind:"refresh",base:manifest.generation,dirty:[]}};
  const digest=await writeManifest(next);
  try{const outcome=await publish({manifestDigest:digest,expectedGeneration:current.pointer.generation,verified:current.pointer.verified,signal});if(outcome.published){if(patched)indexCache.set(factIndex,patched);return{current:{pointer:outcome.pointer,manifest:next},published:true};}}
  catch(error){if(signal?.aborted)throw error;}
  return{current:await read(signal)??current,published:false};
 }

 /** 🔁️ Brings the caller to a generation that holds. `current` serves the newest generation of this identity without verification or waiting; `fresh` verifies (only the closure `only` when given), serves a verified generation without any lease and builds under the builder lease otherwise, where a holder that finds the work done by a peer returns that generation untouched. */
 async function refresh({identity,implementation,policy="fresh",only,membership,compute,signal,progress,builder,requestedAt}){
  if(!P.policies.includes(policy))throw fail("Closed graph policy required");
  const started=now(),first=await read(signal),usable=first?.manifest&&first.manifest.identity===identity;
  if(policy==="current"&&usable)return{current:first,served:"current",built:false,published:false,computed:0};
  let hint,scope=only,coverage,pending;
  if(usable&&requestedAt!==undefined&&first.pointer.verified!==undefined){
   const owners=Object.keys(first.manifest.owners).sort(compareCodePointsV1);
   coverage=coverageOfV1({stamp:first.pointer.verified,owners,implementation,manifestImplementation:first.manifest.implementation,requestedAtMs:requestedAt});
   if(coverage){const open=uncoveredOfV1({needed:only,owners,coverage});if(!open.size)return{current:first,served:"covered",built:false,published:false,computed:0};scope=open;}
  }
  if(usable){
   const previous={manifest:first.manifest,frontier:await readFrontier(first.manifest.frontier)},live=await membership(previous.frontier?previous:undefined,signal);
   const verdict=await plan({current:first,identity,changes:previous.frontier?live.changes:undefined,only:scope,skipEnvironment:coverage!==undefined,signal,progress});
   if(verdict.kind==="clean"){
    if(!scope&&live.digest!==first.manifest.membership||verdict.refresh?.size){const refreshed=await refreshMetadata({current:first,live:!scope&&live.digest!==first.manifest.membership?live:undefined,signal,held:verdict.refresh});return{...refreshed,served:"verified",built:false,computed:0,plan:verdict};}
    return{current:first,served:"verified",built:false,published:false,computed:0,plan:verdict};
   }
   if(verdict.kind==="delta")pending={generation:first.pointer.generation,dirty:[...verdict.dirty].sort(compareCodePointsV1),roots:Object.fromEntries([...verdict.dirty].filter(name=>first.manifest.owners[name]).map(name=>[name,first.manifest.owners[name].root]))};
   if(verdict.kind==="delta"&&previous.frontier)hint={generation:first.pointer.generation,plan:verdict,live,atMs:now()};
  }
  const body=()=>build({identity,implementation,membership,compute,signal,progress,hint,only:scope,observedAtMs:started,coverage});
  for(let attempt=0;attempt<8;attempt++){
   const result=await(builder?builder(body,pending):lease?lease.exclusive(resources.builder,body,{signal}):body());
   if(result.published||!result.built)return{...result,served:"built"};
  }
  throw fail("Generation keeps changing while it is built","GENERATION_CONFLICT");
 }

 /** 🧭️ Collects the closure of `roots` through the fragments of one generation. `neighbors(fragment)` names the projects a fragment reaches (the targets of its edges by default); only the fragments of the closure are read. */
 async function closureOf(current,roots,{neighbors,signal}={}){
  const manifest=current?.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const seen=new Set();let frontier=[...new Set(roots)];
  while(frontier.length){
   const batch=[...new Set(frontier.filter(name=>!seen.has(name)))];frontier=[];
   for(const name of batch){if(!manifest.owners[name])throw fail("Unknown project in the closure: "+name,"GENERATION_UNKNOWN_OWNER");seen.add(name);}
   const loaded=await mapLimit(batch,P.readConcurrency,async name=>{const fragment=await readFragment(manifest.owners[name]);if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment;},signal);
   for(const fragment of loaded)for(const target of(neighbors?neighbors(fragment):fragment.edges.map(edge=>edge.target))){if(target==="*"){for(const name of Object.keys(manifest.owners))if(!seen.has(name))frontier.push(name);}else if(manifest.owners[target]&&!seen.has(target))frontier.push(target);}
  }
  return seen;
 }

 /**
  * 🌊️ Streams the graph of exactly the owners in `names` (every owner without it) fragment by fragment: `nodes()` yields `[name, node]` in code point order while at most `readConcurrency` fragments are resident, so a consumer that drops what it was given keeps the resident state independent of the number of owners; `finish()` answers the rest (dependencies, external nodes, unavailable projects, generation, membership) once the nodes were consumed. Nodes and edges come from fragments only, so unrelated owners are never read; the external nodes are those the selected owners' edges reach plus those their inputs (target inputs and named input groups) name in `externalDependencies` (a task hash needs the node of every external dependency it lists, edge or not).
  */
 function assembleStream(current,{names,compare=(a,b)=>a<b?-1:a>b?1:0,signal,progress}={}){
  const manifest=current.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const wanted=(names?[...new Set(names)]:Object.keys(manifest.owners)).sort(compareCodePointsV1),keep=names?new Set(wanted):undefined;
  for(const name of wanted)if(!manifest.owners[name])throw fail("Unknown project in the closure: "+name,"GENERATION_UNKNOWN_OWNER");
  const unavailable=[],edges=new Map(),roots=[],referenced=new Set();let global,drained=false,started=false;
  const referenceExternalInputs=value=>{if(Array.isArray(value)){for(const item of value)referenceExternalInputs(item);}else if(value&&typeof value==="object"){for(const[key,item]of Object.entries(value)){if(key==="externalDependencies"&&Array.isArray(item)){for(const name of item)if(typeof name==="string")referenced.add(name);}else referenceExternalInputs(item);}}};
  async function*nodes(){
   if(started)throw fail("The node stream of an assembly is read once","GENERATION_STREAM_USED");started=true;
   const globalBytes=await fs.read(fragmentPath(manifest.global));if(!globalBytes||sha256(globalBytes)!==manifest.global)throw fail("Global fragment is missing","GENERATION_INCOMPLETE");
   global=unpack(globalBytes,P.maximumFragmentBytes).value??{};
   await progress?.({phase:"assembly",done:0,total:wanted.length});
   const pending=[];let next=0,done=0;
   const launch=()=>{while(pending.length<P.readConcurrency&&next<wanted.length){const name=wanted[next++],job=readFragment(manifest.owners[name]).then(fragment=>{if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment;});job.catch(()=>{});pending.push(job);}};
   launch();
   while(pending.length){
    aborted(signal);const fragment=await pending.shift();launch();
    roots.push(fragment.root);
    for(const edge of fragment.edges)if(!keep||keep.has(edge.target)||global.externalNodes?.[edge.target]!==undefined)edges.set(JSON.stringify(edge),edge);
    if(fragment.kind==="project"){referenceExternalInputs(fragment.node);yield[fragment.name,fragment.node];}else unavailable.push(fragment.node);
    await progress?.({phase:"assembly",done:++done,total:wanted.length});
   }
   drained=true;
  }
  async function finish(){
   if(!drained)throw fail("The node stream must be consumed before the assembly is finished","GENERATION_STREAM_OPEN");
   const external=new Set();
   if(keep){
    for(const edge of edges.values())if(global.externalNodes?.[edge.target]!==undefined)external.add(edge.target);
    for(const name of referenced)if(global.externalNodes?.[name]!==undefined)external.add(name);
    let grown=true;
    while(grown){grown=false;for(const edge of global.edges??[])if((keep.has(edge.source)||external.has(edge.source))&&(keep.has(edge.target)||global.externalNodes?.[edge.target]!==undefined)){edges.set(JSON.stringify(edge),edge);if(global.externalNodes?.[edge.target]!==undefined&&!external.has(edge.target)){external.add(edge.target);grown=true;}}}
   }else for(const edge of global.edges??[])edges.set(JSON.stringify(edge),edge);
   const sorted=[...edges.entries()].sort(([a],[b])=>compare(a,b)).map(([,edge])=>edge);
   const externalNodes=Object.fromEntries(Object.entries(global.externalNodes??{}).filter(([key])=>!keep||external.has(key)));
   const rows=[...(global.unavailable??[]),...unavailable].filter(row=>!keep||roots.some(root=>row.root===root||row.root.startsWith(root+"/")));
   return{dependencies:sorted,externalNodes,unavailable:rows.sort((a,b)=>compareCodePointsV1(a.root,b.root)),generation:manifest.generation,membership:manifest.membership};
  }
  return Object.freeze({nodes,finish});
 }

 /** 🧩️ Assembles the graph of exactly the owners in `names` (every owner without it) into one object: the streamed nodes collected in order, then the rest. */
 async function assemble(current,options){
  const stream=assembleStream(current,options),projects={};
  for await(const[name,node]of stream.nodes())projects[name]=node;
  return{projects,...await stream.finish()};
 }

 /** 📸️ Reads one complete graph without any lease: a fragment retired while the snapshot is read restarts it from the newest pointer, so the result is always the graph of one sealed generation. */
 async function snapshot({roots,neighbors,compare,signal,progress}={}){
  for(let attempt=0;attempt<8;attempt++){
   const current=await read(signal);if(!current?.manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
   try{const names=roots?[...await closureOf(current,roots,{neighbors,signal})]:undefined;return{current,graph:await assemble(current,{names,compare,signal,progress})};}
   catch(error){if(error?.code!=="GENERATION_INCOMPLETE")throw error;await sleep(5*(attempt+1),signal);}
  }
  throw fail("Generation fragments keep disappearing","GENERATION_UNREADABLE");
 }

 /** 🎯️ Brings the caller to a generation that holds for the closure of `roots`: only the facts those projects consume are verified, so an edit elsewhere neither waits for a builder nor is refreshed for this caller. */
 async function refreshClosure({roots,neighbors,closure,identity,implementation,policy="fresh",membership,compute,signal,progress,builder,requestedAt}){
  const closeOf=current=>closure?closure(current,{signal}):closureOf(current,roots,{neighbors,signal});
  const first=await read(signal);let names,unborn=false;
  if(first?.manifest&&first.manifest.identity===identity){try{names=await closeOf(first);}catch(error){if(error?.code==="GENERATION_UNKNOWN_OWNER"){names=new Set();unborn=true;}else if(error?.code!=="GENERATION_INCOMPLETE")throw error;}}
  let verified=names,result=await refresh({identity,implementation,policy,only:names,membership,compute,signal,progress,builder,requestedAt});
  if(result.needsBuild)return result;
  let built=result.built,published=result.published,computed=result.computed;
  for(let round=0;;round++){
   const closure=verified&&!unborn&&result.current.pointer.generation===first.pointer.generation?verified:await closeOf(result.current);
   if(policy==="current"||!verified||round>=8||[...closure].every(name=>verified.has(name)))return{...result,built,published,computed,...(built?{served:"built"}:{}),names:closure};
   verified=new Set([...verified,...closure]);result=await refresh({identity,implementation,policy,only:verified,membership,compute,signal,progress,builder,requestedAt});
   if(result.needsBuild)return result;built=built||result.built;published=published||result.published;computed+=result.computed;
  }
 }

 /** 🧹️ Removes documents that no retained generation names once they are older than the grace window (every file of the index directory is an index document, whatever format its generation used); a failure only costs disk. */
 async function retire({signal}={}){
  aborted(signal);const pointer=await readPointer();if(!pointer)return;
  const files=await fs.list(join(root,P.manifests)),rows=[];
  for(const file of files){aborted(signal);if(!/^[a-f0-9]{64}\.json$/u.test(file.name))continue;const digest=file.name.slice(0,64),bytes=await fs.read(manifestPath(digest));if(!bytes)throw fail("Generation manifest disappeared before retirement","GENERATION_INCOMPLETE");if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==digest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");rows.push({digest,manifest:readGenerationManifestV1(bytes)});}
  if(pointer.manifest!==null&&!rows.some(row=>row.digest===pointer.manifest))throw fail("Current generation manifest missing before retirement","GENERATION_INCOMPLETE");
  rows.sort((a,b)=>b.manifest.generation-a.manifest.generation);
  const kept=rows.filter((row,index)=>index<P.retainGenerations||row.digest===pointer.manifest),live=new Set([pointer.manifest]),fragments=new Set(),indexes=new Set(),frontiers=new Set();
  for(const row of kept){live.add(row.digest);indexes.add(row.manifest.factIndex);fragments.add(row.manifest.global);if(row.manifest.frontier)frontiers.add(row.manifest.frontier);for(const owner of Object.values(row.manifest.owners))fragments.add(owner.fragment);}
  const cutoff=now()-P.graceMilliseconds;
  const sweep=async(directory,keep,extension)=>{for(const file of await fs.list(join(root,directory))){aborted(signal);if(file.name.endsWith(extension)&&!keep.has(file.name.slice(0,64))&&file.modifiedMs<cutoff)await fs.remove(join(root,directory,file.name));}};
  await sweep(P.manifests,live,".json");await sweep(P.fragments,fragments,".json.br");await sweep(P.indexes,indexes,"");await sweep(P.frontiers,frontiers,".json.br");
 }

 /** ♻️ Marks the newest generation invalid without deleting anything; the next fresh reader builds. */
 async function invalidate({reason="invalidated",signal}={}){
  const pointer=await readPointer(),digest=null;
  const outcome=await publish({manifestDigest:digest,expectedGeneration:pointer?.generation??-1,reason,signal});
  return outcome.published;
 }

 return Object.freeze({dropIndexes:()=>indexCache.clear(),read,readManifestAt,readIndex,readIndexView,readFrontier,readFragment,plan,build,refresh,refreshClosure,closureOf,assemble,assembleStream,snapshot,retire,invalidate,publish,stats,readGlobal,catalog});
}

/** 🔒️ A reference exclusive lease over lock files for tests and non repository runtimes: acquisition is `open(wx)`, a dead owner's lock is replaced, waiting polls with backoff and honours the abort signal. */
export function createFileLeasePortV1(directory,{staleMilliseconds=600000,now=()=>Date.now(),sleep=defaultSleep,onHold}={}){
 const alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}};
 return Object.freeze({
  async exclusive(resource,body,{signal,timeoutMs}={}){
   const path=join(directory,sha256(resource)+".lock"),token=randomBytes(12).toString("hex"),started=now();let delay=2;
   await mkdir(directory,{recursive:true});
   for(;;){
    aborted(signal);
    try{const file=await open(path,"wx");try{await file.writeFile(JSON.stringify({pid:process.pid,token,since:now()}));}finally{await file.close();}break;}
    catch(error){
     if(error?.code!=="EEXIST")throw error;
     try{const held=JSON.parse(await readFile(path,"utf8"));if(!alive(held.pid)||now()-held.since>staleMilliseconds){const again=JSON.parse(await readFile(path,"utf8"));if(again.token===held.token)await rm(path,{force:true});continue;}}catch{}
     if(timeoutMs!==undefined&&now()-started>timeoutMs)throw fail("Resource lease timed out: "+resource,"LEASE_TIMEOUT");
     await sleep(delay,signal);delay=Math.min(delay*2,P.pollMaximumMilliseconds);
    }
   }
   const acquired=now();
   try{return await body();}finally{try{const held=JSON.parse(await readFile(path,"utf8"));if(held.token===token)await rm(path,{force:true});}catch{}onHold?.({resource,heldMs:now()-acquired,waitedMs:acquired-started});}
  },
 });
}
