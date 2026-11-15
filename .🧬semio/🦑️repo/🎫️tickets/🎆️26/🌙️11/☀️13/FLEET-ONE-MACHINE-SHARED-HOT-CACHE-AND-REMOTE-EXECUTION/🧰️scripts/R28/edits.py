"""R28: closure verification is resumable and vouched. Anchored edits (rel path from the worktree root, old, new, count)."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
G=B+"🕸️graph/"
CK=G+"🧮️derived-nodes/🧩️closure-kinds/"
MANIFEST=CK+"📒️manifest/🟨️.mjs"
PATHS=CK+"🧭️row-paths/🟨️.mjs"
KINDS=CK+"🟨️.mjs"
MANIFEST_TEST=CK+"📒️manifest/🧪️tests/🟦️.ts"
CUTOFF_TEST=G+"🕰️generations/✂️cutoff/🧪️tests/🟦️.ts"
GEN=G+"🕰️generations/🟨️.mjs"
SCHEMA=G+"🕰️generations/✂️cutoff/🧬️schema/🔣️.json"
E=[]
def edit(rel,old,new,count=1):E.append((rel,old,new,count))

edit(SCHEMA,'''    "settleChunkClosures": 256,
''','''    "settleChunkClosures": 256,
    "manifestSaveMilliseconds": 15000,
    "manifestSaveSpendRatio": 10,
    "vouchConcurrency": 64,
''')

edit(PATHS,'''import{dirname,isAbsolute,resolve}from"node:path";
''','''import{createHash}from"node:crypto";
import{lstat,readdir,readFile,readlink}from"node:fs/promises";
import{dirname,isAbsolute,resolve}from"node:path";
''')
edit(PATHS,'''  if(path.includes("node_modules/"))modules=true;
 }
 return{paths,directories,configs,modules,...dirty.excluded?{excluded:dirty.excluded}:{}};
}
''','''  if(path.includes("node_modules/"))modules=true;
 }
 return{paths,directories,configs,modules,...dirty.excluded?{excluded:dirty.excluded}:{}};
}

const stampMemo=new Map();
/** 🧾️ The stamp of one path as it is on the disk now: the sha256 of the bytes of a file (remembered by its size, times and inode, so an untouched file is read once per process), the sorted names and kinds of a directory, the target of a link, `-` for an absent path; undefined when the path cannot be read, which no vouch may rest on.
 * @param {string} absolute @param {number} [memoEntries] */
export async function stampOfV1(absolute,memoEntries=65536){
 let info;try{info=await lstat(absolute,{bigint:true});}catch{return"-";}
 if(info.isDirectory()){
  const entries=await readdir(absolute,{withFileTypes:true}).catch(()=>undefined);if(entries===undefined)return undefined;
  const hash=createHash("sha256");for(const name of entries.map(entry=>entry.name+(entry.isDirectory()?"/":entry.isSymbolicLink()?"@":"")).sort())hash.update(name).update("\\0");
  return"d:"+hash.digest("hex");
 }
 if(info.isSymbolicLink()){const target=await readlink(absolute).catch(()=>undefined);return target===undefined?undefined:"l:"+target;}
 if(!info.isFile())return"o:"+info.mode;
 const key=absolute+"\\0"+info.size+"\\0"+info.mtimeNs+"\\0"+info.ctimeNs+"\\0"+info.ino,known=stampMemo.get(key);
 if(known!==undefined)return known;
 let bytes;try{bytes=await readFile(absolute);}catch{return undefined;}
 const value="f:"+createHash("sha256").update(bytes).digest("hex");
 stampMemo.set(key,value);if(stampMemo.size>memoEntries)stampMemo.delete(stampMemo.keys().next().value);
 return value;
}
''')

edit(KINDS,'''import{affectedPartsV1,createManifestV1,indexPartV1,intersectsV1,partIdsOfV1,readManifestV1,recordEntryV1,writeManifestV1}from"./📒️manifest/🟨️.mjs";''','''import{affectedPartsV1,createManifestV1,createVouchV1,forgetEntryV1,indexPartV1,intersectsV1,loadManifestV1,partIdsOfV1,planSaveV1,recordEntryV1}from"./📒️manifest/🟨️.mjs";''')
edit(KINDS,'''import{dirtySummaryV1,rowPathsV1,rowTouchedV1}from"./🧭️row-paths/🟨️.mjs";
''','''import{dirtySummaryV1,rowPathsV1,rowTouchedV1}from"./🧭️row-paths/🟨️.mjs";
export{stampOfV1}from"./🧭️row-paths/🟨️.mjs";
''')
edit(KINDS,'''settleChunkClosures:CUTOFF_POLICY_V1.settleChunkClosures,maximumSuspects''','''settleChunkClosures:CUTOFF_POLICY_V1.settleChunkClosures,manifestSaveMilliseconds:CUTOFF_POLICY_V1.manifestSaveMilliseconds,manifestSaveSpendRatio:CUTOFF_POLICY_V1.manifestSaveSpendRatio,vouchConcurrency:CUTOFF_POLICY_V1.vouchConcurrency,maximumSuspects''')
edit(KINDS,'''if(Array.isArray(rows)){rows=rows.map(internRow);table.set(id,rows);}return Array.isArray(rows)?rows:undefined;};''','''if(Array.isArray(rows)&&partIdOf(rows)===id){rows=rows.map(internRow);table.set(id,rows);}else rows=undefined;return rows;};''')
edit(KINDS,'''const runOnce=async({pseudoRows,derive,hold=async()=>true,dirty,signal,progress,answers:keepAnswers=false})=>{''','''const runOnce=async({pseudoRows,derive,hold=async()=>true,dirty,signal,progress,vouchStamp,answers:keepAnswers=false})=>{''')
edit(KINDS,'''  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map();
''','''  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map();
  const vouching=summary===undefined||typeof vouchStamp!=="function"?undefined:createVouchV1({book:()=>book,summary,stamp:vouchStamp,concurrency:limits.vouchConcurrency,signal});
  let savedAt=Date.now(),lastSaveMs=0;
''')
edit(KINDS,'''  const settleChunk=async chunk=>{
   await pool(chunk,''','''  const settleChunk=async chunk=>{
   const recorded=[];
   await pool(chunk,''')
edit(KINDS,'''if(digest!==undefined){recordEntryV1(book,name,{digest,fact:canonicalJsonV1(node.value.fact),length:size,ids});manifestDirty=true;}''','''if(digest!==undefined){recordEntryV1(book,name,{digest,fact:canonicalJsonV1(node.value.fact),length:size,ids});recorded.push(name);manifestDirty=true;}''')
edit(KINDS,'''   await stage("flush",1,async tick=>{await graph.flush();await tick();});
''','''   if(vouching!==undefined&&recorded.length)await vouching.seal(recorded);
   await stage("flush",1,async tick=>{await graph.flush();await tick();});
   if(manifestDirty&&Date.now()-savedAt>=Math.max(limits.manifestSaveMilliseconds,limits.manifestSaveSpendRatio*lastSaveMs)){const began=Date.now();await saveManifest();lastSaveMs=Date.now()-began;savedAt=Date.now();}
''')
edit(KINDS,'''  let asked=0;
  if(names.length)await progress?.''','''  let asked=0,vouched=0;
  if(names.length)await progress?.''')
edit(KINDS,'''   if(equal&&mark!==undefined&&mark.digest===entry.digest&&mark.length===entry.length){kept.add(name);return;}
   if(!equal){changed.set(name,row);return;}
   const ids=partIdsOfV1(book,entry),loaded=(await Promise.all(ids.map(loadPart))).filter(rows=>rows!==undefined);
   held.set(name,partRowsV1(row,loaded,entry.digest,ids,entry.fact));
  };
  if(summary!==undefined&&!keepAnswers&&book.entries.size){
   const affected=await affectedPartsV1(book,summary,async rows=>{if(await classify(rows,signal))return[];return rows.filter(row=>broken.has(row));});
   for(const name of names){const entry=book.entries.get(name);if(entry===undefined||intersectsV1(entry.bits,affected)){slow.push(name);continue;}await answerFromManifest(name,entry);done++;asked++;}
  }else slow.push(...names);
''','''   if(equal&&mark!==undefined&&mark.digest===entry.digest&&mark.length===entry.length){kept.add(name);return true;}
   if(!equal){changed.set(name,row);return true;}
   const ids=partIdsOfV1(book,entry),loaded=(await Promise.all(ids.map(loadPart))).filter(rows=>rows!==undefined);
   if(loaded.length!==ids.length)return false;
   held.set(name,partRowsV1(row,loaded,entry.digest,ids,entry.fact));
   return true;
  };
  if(summary!==undefined&&!keepAnswers&&book.entries.size){
   const outside=[],affected=await affectedPartsV1(book,summary,async rows=>{const failing=await classify(rows,signal)?[]:rows.filter(row=>broken.has(row));outside.push(...failing);return failing;});
   await vouching?.begin(new Set(outside.flatMap(row=>book.outside.get(rowKeyV1(row))?.parts??[])));
   for(const name of names){
    const entry=book.entries.get(name);if(entry===undefined){slow.push(name);continue;}
    const hit=intersectsV1(entry.bits,affected),vouch=hit?vouching?.vouchOf(entry.bits):undefined;
    if(hit&&(vouch===undefined||entry.vouch!==vouch)||!await answerFromManifest(name,entry)){slow.push(name);continue;}
    if(hit)vouched++;
    done++;asked++;
   }
  }else{await vouching?.begin(new Set());slow.push(...names);}
''')
edit(KINDS,'''return{held,kept,changed,answers,failed,reused:Math.max(0,names.length-failed-fresh.size)};''','''return{held,kept,changed,answers,failed,vouched,reused:Math.max(0,names.length-failed-fresh.size)};''')

edit(KINDS,r'''const manifestIdentity="closure-manifest\0all\0";''',r'''const manifestIdentityOf=subject=>"closure-manifest\0"+subject+"\0";''')
edit(KINDS,r'''const manifestOf=async()=>{if(manifest!==undefined)return manifest;const value=(await partStore()?.get(manifestIdentity))?.value;manifest=value===undefined?createManifestV1():readManifestV1(value);return manifest;};''',r'''const manifestOf=async()=>{if(manifest!==undefined)return manifest;const store=partStore();manifest=store===undefined?createManifestV1():await loadManifestV1(async subject=>(await store.get(manifestIdentityOf(subject)))?.value);return manifest;};''')
edit(KINDS,r'''const saveManifest=async()=>{if(!manifestDirty||manifest===undefined)return;manifestDirty=false;partStore()?.put({key:{kind:"closure-manifest",subject:"all",params:""},value:writeManifestV1(manifest),deps:[],rawReads:[],valueDigest:"manifest",computedAtRev:"0"},"");await partStore()?.flush?.();};''',r'''const saveManifest=async()=>{
  if(!manifestDirty||manifest===undefined)return;manifestDirty=false;const store=partStore();if(store===undefined)return;
  const plan=planSaveV1(manifest),put=(subject,value)=>store.put({key:{kind:"closure-manifest",subject,params:""},value,deps:[],rawReads:[],valueDigest:"manifest",computedAtRev:"0"},"");
  for(const[subject,value]of plan.writes)put(subject,value);
  await store.flush?.();put("all",plan.head);await store.flush?.();plan.commit();
 };''')
edit(KINDS,'''if(book.entries.delete(name))manifestDirty=true;}''','''if(forgetEntryV1(book,name))manifestDirty=true;}''')

edit(GEN,'''import{createClosureSettlerV1,sharedClosureSettlerV1}from"../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";''','''import{createClosureSettlerV1,sharedClosureSettlerV1,stampOfV1}from"../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";''')
edit(GEN,'''derive,signal,progress,hold,...journalDirty?{dirty:journalDirty}:{}})),changedHolders''','''derive,signal,progress,hold,...journalDirty?{dirty:journalDirty}:{},...workspaceRoot!==undefined?{vouchStamp:name=>stampOfV1(join(workspaceRoot,name))}:{}})),changedHolders''')

edit(MANIFEST_TEST,'''expect([...back.entries.keys()]).toEqual([...book.entries.keys()]);''','''expect([...back.entries.keys()].sort()).toEqual([...book.entries.keys()].sort());''')
edit(MANIFEST_TEST,'''expect([...back.paths]).toEqual([...book.paths]);''','''expect([...back.paths].sort()).toEqual([...book.paths].sort());''')

edit(CUTOFF_TEST,'''expect(files.length,"three closures, four parts, the four path summaries of the parts and the settle manifest").toBe(3+4+4+1);''','''expect(files.length,"three closures, four parts, the four path summaries of the parts and the settle manifest (its head and the shards that hold something)").toBeGreaterThanOrEqual(3+4+4+2);''')
MANIFEST_BEFORE_SHA="d6ad628dd042942f0433481461df4761cec3392bcb41f21b49e3672a2f530aa1"
REPLACE=[(MANIFEST,"replace/"+MANIFEST)]
REHEARSAL_COPIES=[]
