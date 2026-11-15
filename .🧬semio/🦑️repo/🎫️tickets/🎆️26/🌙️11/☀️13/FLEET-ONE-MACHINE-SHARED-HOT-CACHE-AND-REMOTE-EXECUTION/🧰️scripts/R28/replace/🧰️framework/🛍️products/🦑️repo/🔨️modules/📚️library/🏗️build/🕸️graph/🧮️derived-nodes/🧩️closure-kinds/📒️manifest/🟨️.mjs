import{createHash}from"node:crypto";
import{rowKeyV1}from"../../../🕰️generations/🧮️fact-rows/🟨️.mjs";

/** 📒️ Creates the settle manifest: for every closure its answer digest, canonical answer, support length and the set of its parts as a bitset over part numbers; the inverted index of the parts (by path, directory, bare ancestor, configuration, volatility and outside rows) that tells which parts a dirty set can move; and the marks of the index the closures were published in. It is read once per process and answers a round without reading a record of a closure or a part. */
export function createManifestV1(){return{partNames:[],partNo:new Map(),indexed:new Set(),entries:new Map(),marks:new Map(),paths:new Map(),dirs:new Map(),bare:new Map(),volatile:new Set(),config:new Set(),bareAny:new Set(),outside:new Map(),revs:new Map(),saved:new Map(),fullAt:0};}

const partNumberV1=(book,id)=>{let number=book.partNo.get(id);if(number===undefined){number=book.partNames.length;book.partNames.push(id);book.partNo.set(id,number);}return number;};
const bitsOfV1=numbers=>{let top=0;for(const number of numbers)if(number>top)top=number;const words=new Uint32Array((top>>5)+1);for(const number of numbers)words[number>>5]|=1<<(number&31);return words;};
const SHARDS_V1=128;
const shardOfV1=text=>{let hash=2166136261;for(let at=0;at<text.length;at++)hash=Math.imul(hash^text.charCodeAt(at),16777619);return(hash>>>0)%SHARDS_V1;};
const bump=(book,key)=>{book.revs.set(key,(book.revs.get(key)??0)+1);};
const sha=value=>createHash("sha256").update(value).digest("hex");
const addTo=(map,key,number)=>{const list=map.get(key);if(list===undefined)map.set(key,[number]);else list.push(number);};

/** 🧭️ Whether two part sets share a part. */
export function intersectsV1(left,right){const length=Math.min(left.length,right.length);for(let at=0;at<length;at++)if((left[at]&right[at])!==0)return true;return false;}

/** 🧭️ Registers what can move one part (its full summary) in the inverted index, once. */
export function indexPartV1(book,id,meta){
 if(book.indexed.has(id))return;book.indexed.add(id);const number=partNumberV1(book,id);
 for(const path of meta.paths){addTo(book.paths,path,number);bump(book,"i"+shardOfV1(path));}
 for(const dir of meta.dirs){addTo(book.dirs,dir,number);bump(book,"i"+shardOfV1(dir));}
 for(const dir of meta.bare){addTo(book.bare,dir,number);bump(book,"i"+shardOfV1(dir));}
 if(meta.bare.length)book.bareAny.add(number);
 if(meta.volatile)book.volatile.add(number);
 if(meta.config)book.config.add(number);
 for(const row of meta.outside){const key=rowKeyV1(row),known=book.outside.get(key);if(known===undefined)book.outside.set(key,{row,parts:[number]});else known.parts.push(number);}
}

/** ✍️ Records the answer of one closure. */
export function recordEntryV1(book,name,{digest,fact,length,ids,vouch}){book.entries.set(name,{digest,fact,length,vouch,bits:bitsOfV1(ids.map(id=>partNumberV1(book,id)))});bump(book,"e"+shardOfV1(name));}

/** 🗑️ Forgets the answer of one closure; whether there was one. */
export function forgetEntryV1(book,name){if(!book.entries.delete(name))return false;bump(book,"e"+shardOfV1(name));return true;}

/** 📚️ The sorted part ids of one closure entry. */
export function partIdsOfV1(book,entry){const out=[];for(let at=0;at<entry.bits.length;at++){let word=entry.bits[at];while(word!==0){const bit=31-Math.clz32(word&-word);out.push(book.partNames[at*32+bit]);word&=word-1;}}return out.sort();}

/**
 * 🎯️ The parts a dirty set can move, as a bitset: exactly the parts `rowTouchedV1` would call touched (volatile, configuration when the set names one, named files, named directories, bare ancestors, anything behind an `excluded` predicate), plus the parts of every outside row that no longer holds (`verifyOutside(rows)` returns the rows that failed; the journal never watches them, so they are checked every round, once each).
 * @param {ReturnType<typeof createManifestV1>} book
 * @param {{paths:ReadonlySet<string>;directories:ReadonlySet<string>;configs:boolean;modules:boolean;excluded?:(path:string)=>boolean}} summary
 * @param {(rows:unknown[])=>Promise<unknown[]>} verifyOutside
 */
export async function affectedPartsV1(book,summary,verifyOutside){
 const hit=new Set(book.volatile),add=list=>{if(list!==undefined)for(const number of list)hit.add(number);};
 if(summary.configs)add(book.config);
 for(const path of summary.paths)add(book.paths.get(path));
 for(const dir of summary.directories){add(book.dirs.get(dir));add(book.bare.get(dir));}
 if(summary.modules)add(book.bareAny);
 if(summary.excluded!==undefined){for(const[path,list]of book.paths)if(summary.excluded(path)===true)add(list);for(const[dir,list]of book.dirs)if(summary.excluded(dir)===true)add(list);}
 if(book.outside.size){for(const row of await verifyOutside([...book.outside.values()].map(entry=>entry.row)))add(book.outside.get(rowKeyV1(row))?.parts);}
 return bitsOfV1(hit);
}

const KINDS_V1=Object.freeze(["paths","dirs","bare"]);
const shardKeysV1=()=>{const keys=[];for(let at=0;at<SHARDS_V1;at++)keys.push("e"+at,"i"+at);return keys;};
const wordsOf=base64=>{const bytes=Buffer.from(base64,"base64");return new Uint32Array(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength));};
const REFRESH_V1=86400000;
const headCheckV1=head=>sha(JSON.stringify([head.at,head.parts,head.indexed,head.marks,head.volatile,head.config,head.bareAny,head.outside,head.shards]));

/** 🧱️ The payloads of the wanted shards: an entry shard holds the answers of the closures whose name falls into it (names, digests, facts, lengths, vouches and the part sets as words of one common width), an index shard the inverted index of the paths, directories and bare ancestors that fall into it; both sorted, so equal content is equal text. */
function payloadsV1(book,wanted){
 const out=new Map(),slot=key=>{let payload=out.get(key);if(payload===undefined){payload=key[0]==="e"?{names:[],digests:[],facts:[],lengths:[],vouches:[],width:1,rows:[]}:{paths:[],dirs:[],bare:[]};out.set(key,payload);}return payload;};
 for(const[name,entry]of book.entries){const key="e"+shardOfV1(name);if(!wanted.has(key))continue;const payload=slot(key);payload.rows.push([name,entry]);if(entry.bits.length>payload.width)payload.width=entry.bits.length;}
 for(const kind of KINDS_V1)for(const[path,list]of book[kind]){const key="i"+shardOfV1(path);if(wanted.has(key))slot(key)[kind].push([path,list]);}
 for(const[key,payload]of out){
  if(key[0]==="e"){
   payload.rows.sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0);
   const words=new Uint32Array(payload.rows.length*payload.width);
   payload.rows.forEach(([name,entry],at)=>{payload.names.push(name);payload.digests.push(entry.digest);payload.facts.push(entry.fact);payload.lengths.push(entry.length);payload.vouches.push(entry.vouch??null);words.set(entry.bits,at*payload.width);});
   payload.bits=Buffer.from(words.buffer).toString("base64");delete payload.rows;
  }else for(const kind of KINDS_V1)payload[kind].sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0);
 }
 return out;
}

/**
 * 💾️ Plans one save of the manifest (every shard again when the last full write is a day old, so no shard outlives the retention of the node store unseen): the shards whose content changed since they were loaded or saved (`writes`, each under a subject that names its digest, so a reader of the previous head still finds the previous shard) and the head that lists every shard with its digest and seals the whole (`head`). A shard is serialized only when something touched it, and written only when its digest differs from the saved one. `commit()` tells the book what is durable; the caller writes the shards first and the head last.
 * @param {ReturnType<typeof createManifestV1>} book
 */
export function planSaveV1(book,everything=false,now=Date.now()){
 if(now-book.fullAt>REFRESH_V1)everything=true;
 const keys=shardKeysV1(),dirty=new Set(keys.filter(key=>{const saved=book.saved.get(key);return everything||saved===undefined||saved.rev!==(book.revs.get(key)??0);})),payloads=payloadsV1(book,dirty),writes=[],shards=[],next=new Map();
 for(const key of keys){
  const saved=book.saved.get(key),rev=book.revs.get(key)??0;
  if(!dirty.has(key)){if(saved.digest!==undefined)shards.push([key,saved.subject,saved.digest]);next.set(key,saved);continue;}
  const payload=payloads.get(key);
  if(payload===undefined){next.set(key,{rev,subject:undefined,digest:undefined});continue;}
  const digest=sha(JSON.stringify(payload)),subject=key+"-"+digest.slice(0,24);
  shards.push([key,subject,digest]);next.set(key,{rev,subject,digest});
  if(everything||saved?.digest!==digest)writes.push([subject,payload]);
 }
 const head={version:3,at:everything?now:book.fullAt,parts:book.partNames,indexed:[...book.indexed],marks:[...book.marks],volatile:[...book.volatile],config:[...book.config],bareAny:[...book.bareAny],outside:[...book.outside.values()].map(entry=>[entry.row,entry.parts]),shards};
 head.check=headCheckV1(head);
 return{head,writes,commit(){for(const[key,value]of next)book.saved.set(key,value);book.fullAt=head.at;}};
}

/** 💾️ The persisted form of the whole manifest as one value (the head and every shard by subject), for tests and tools. */
export function writeManifestV1(book){const plan=planSaveV1(book,true);return{head:plan.head,shards:Object.fromEntries(plan.writes)};}

const numbersOk=(list,limit)=>Array.isArray(list)&&list.every(number=>Number.isSafeInteger(number)&&number>=0&&number<limit);
const sizedV1=(list,count)=>Array.isArray(list)&&list.length===count;

function readBundleV1({head,shards}){
 if(head?.version!==3||typeof head.check!=="string"||headCheckV1(head)!==head.check)return createManifestV1();
 for(const field of["parts","indexed","marks","volatile","config","bareAny","outside","shards"])if(!Array.isArray(head[field]))return createManifestV1();
 const book=createManifestV1(),limit=head.parts.length;
 if(!Number.isSafeInteger(head.at))return createManifestV1();
 book.fullAt=head.at;
 head.parts.forEach((id,number)=>{book.partNames.push(id);book.partNo.set(id,number);});
 for(const id of head.indexed)book.indexed.add(id);
 for(const[name,mark]of head.marks)book.marks.set(name,mark);
 if(!numbersOk(head.volatile,limit)||!numbersOk(head.config,limit)||!numbersOk(head.bareAny,limit))return createManifestV1();
 for(const number of head.volatile)book.volatile.add(number);for(const number of head.config)book.config.add(number);for(const number of head.bareAny)book.bareAny.add(number);
 for(const[row,parts]of head.outside){if(!numbersOk(parts,limit))return createManifestV1();book.outside.set(rowKeyV1(row),{row,parts});}
 for(const[key,subject,digest]of head.shards){
  const payload=shards?.[subject];
  if(typeof key!=="string"||payload===undefined||payload===null||sha(JSON.stringify(payload))!==digest)return createManifestV1();
  if(key[0]==="e"){
   const count=payload.names?.length,width=payload.width;
   if(!Number.isSafeInteger(count)||!Number.isSafeInteger(width)||width<1||![payload.digests,payload.facts,payload.lengths,payload.vouches].every(list=>sizedV1(list,count))||typeof payload.bits!=="string")return createManifestV1();
   const words=wordsOf(payload.bits);if(words.length!==count*width)return createManifestV1();
   payload.names.forEach((name,at)=>book.entries.set(name,{digest:payload.digests[at],fact:payload.facts[at],length:payload.lengths[at],vouch:payload.vouches[at]??undefined,bits:words.slice(at*width,(at+1)*width)}));
  }else if(key[0]==="i"){
   for(const kind of KINDS_V1)for(const[path,list]of payload[kind]){if(!numbersOk(list,limit))return createManifestV1();book[kind].set(path,list);}
  }else return createManifestV1();
  book.saved.set(key,{rev:0,subject,digest});
 }
 return book;
}

/** 💾️ The manifest of a persisted form: a form of another version, of an inconsistent shape, with a missing shard or whose head or any shard does not match its digest is a miss (an empty manifest), never trusted. */
export function readManifestV1(bundle){try{return readBundleV1(bundle);}catch{return createManifestV1();}}

/** 💾️ Loads the manifest through `read(subject)` (the head is subject `all`): the head first, then its shards in parallel; anything missing or damaged is a miss. */
export async function loadManifestV1(read){
 try{
  const head=await read("all");if(head?.version!==3||!Array.isArray(head.shards)||headCheckV1(head)!==head.check)return createManifestV1();
  const shards={};
  await Promise.all(head.shards.map(async row=>{const subject=Array.isArray(row)?row[1]:undefined;if(typeof subject!=="string")return;const value=await read(subject);if(value!==undefined)shards[subject]=value;}));
  return readManifestV1({head,shards});
 }catch{return createManifestV1();}
}

/**
 * 🧾️ The vouches of closures against one dirty set. The journal names what changed since the generation, not since the last verification, so every start over the same edits would verify the same closures again. A vouch says: the files and directories of the dirty set that the parts of this closure read had exactly these stamps when the closure was last verified. A closure whose recomputed vouch equals the recorded one reads the same bytes, so its recorded answer holds and nothing is checked or walked. A part that a configuration, a volatile row, an install tree, a path the journal does not watch or an outside row that fails can move is never vouched for; a stamp that changed while the run verified is never vouched for.
 * `book()` is the manifest, `stamp(name)` the stamp of a workspace-relative name.
 * @param {{book:()=>ReturnType<typeof createManifestV1>;summary:{paths:ReadonlySet<string>;directories:ReadonlySet<string>;configs:boolean;modules:boolean;excluded?:(path:string)=>boolean};stamp:(name:string)=>Promise<string|undefined>;concurrency?:number;signal?:AbortSignal}} input
 */
export function createVouchV1({book,summary,stamp,concurrency=64,signal}){
 const start=new Map(),unstable=new Set();let lost=new Set(),touches=new Map(),hard=new Set(),hashes=new Map();
 const touch=()=>{
  const b=book(),items=new Map(),fixed=new Set(b.volatile),add=(list,item)=>{if(list===undefined)return;for(const number of list){const row=items.get(number);if(row===undefined)items.set(number,[item]);else row.push(item);}};
  if(summary.configs)for(const number of b.config)fixed.add(number);
  if(summary.modules)for(const number of b.bareAny)fixed.add(number);
  for(const path of summary.paths)add(b.paths.get(path),"p"+path);
  for(const dir of summary.directories){add(b.dirs.get(dir),"d"+dir);add(b.bare.get(dir),"d"+dir);}
  if(summary.excluded!==undefined){
   for(const[path,list]of b.paths)if(summary.excluded(path)===true)for(const number of list)fixed.add(number);
   for(const[dir,list]of b.dirs)if(summary.excluded(dir)===true)for(const number of list)fixed.add(number);
  }
  for(const number of lost)fixed.add(number);
  touches=items;hard=fixed;hashes=new Map();
 };
 const measure=async wanted=>{
  const list=[...wanted],out=new Map();let next=0;
  await Promise.all(Array.from({length:Math.min(concurrency,list.length)},async()=>{while(next<list.length){signal?.throwIfAborted();const item=list[next++];let value;try{value=await stamp(item.slice(1));}catch{value=undefined;}out.set(item,value);}}));
  return out;
 };
 const numbersOf=bits=>{const out=[];for(let at=0;at<bits.length;at++){let word=bits[at];while(word!==0){out.push(at*32+31-Math.clz32(word&-word));word&=word-1;}}return out;};
 const hashOf=number=>{
  const known=hashes.get(number);if(known!==undefined)return known||undefined;
  const hash=createHash("sha256");let ok=true;
  for(const item of[...touches.get(number)].sort()){const value=start.get(item);if(value===undefined||unstable.has(item)){ok=false;break;}hash.update(item).update("\0").update(value).update("\n");}
  const done=ok?hash.digest("hex"):"";hashes.set(number,done);return done||undefined;
 };
 return Object.freeze({
  async begin(outside=new Set()){lost=outside;touch();const wanted=new Set();for(const list of touches.values())for(const item of list)wanted.add(item);for(const[item,value]of await measure(wanted))start.set(item,value);},
  vouchOf(bits){
   const parts=[];
   for(const number of numbersOf(bits)){if(hard.has(number))return undefined;if(!touches.has(number))continue;const hash=hashOf(number);if(hash===undefined)return undefined;parts.push(hash);}
   return parts.length===0?undefined:createHash("sha256").update(parts.sort().join("\n")).digest("hex");
  },
  async seal(names){
   touch();const b=book(),entries=names.map(name=>[name,b.entries.get(name)]).filter(row=>row[1]!==undefined),wanted=new Set();
   for(const[,entry]of entries)for(const number of numbersOf(entry.bits))for(const item of touches.get(number)??[])wanted.add(item);
   for(const[item,value]of await measure(wanted))if(value===undefined||start.get(item)!==value)unstable.add(item);
   hashes=new Map();
   for(const[name,entry]of entries){const vouch=this.vouchOf(entry.bits);if(vouch!==entry.vouch){entry.vouch=vouch;bump(b,"e"+shardOfV1(name));}}
  }
 });
}
