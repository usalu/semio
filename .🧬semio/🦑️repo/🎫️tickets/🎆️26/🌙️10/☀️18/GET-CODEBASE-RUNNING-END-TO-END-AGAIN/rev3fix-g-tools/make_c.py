from make_lib import *

# ---- C: lane retention vs the pool trimmer, atomic link-less publication
U = L + "/⚡️caching/🦀️cargo/🗃️unit-cache"
ret, lane, pool, schema = U + "/🧵️lane/📤️retention/🟦️.ts", U + "/🧵️lane/🟦️.ts", U + "/🧵️lane/🏊️pool/🟦️.ts", U + "/🧬️schema/🔣️.json"

edit(schema, '"metadata": "lane.json", "size": "lane.size"', '"metadata": "lane.json", "size": "lane.size", "destination": "lane.ledger"')

edit(ret, 'import {closeSync,constants,copyFileSync,createReadStream,existsSync,fsyncSync,linkSync,lstatSync,mkdirSync,openSync,readdirSync,renameSync,rmSync} from "node:fs";',
          'import {closeSync,constants,copyFileSync,fsyncSync,linkSync,lstatSync,mkdirSync,openSync,readFileSync,readSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";')

edit(ret, """async function digestV1(path:string):Promise<Readonly<{sha256:string;bytes:number}>>{
 const hash=createHash("sha256");let bytes=0;
 for await(const chunk of createReadStream(path,{highWaterMark:rules.chunkBytes})){hash.update(chunk as Buffer);bytes+=(chunk as Buffer).length;}
 return{sha256:hash.digest("hex"),bytes};
}
""", """function digestV1(path:string):Readonly<{sha256:string;bytes:number}>{
 const hash=createHash("sha256"),chunk=Buffer.allocUnsafe(rules.chunkBytes),descriptor=openSync(path,"r");let bytes=0;
 try{for(let read=readSync(descriptor,chunk,0,chunk.length,null);read>0;read=readSync(descriptor,chunk,0,chunk.length,null)){hash.update(chunk.subarray(0,read));bytes+=read;}}finally{closeSync(descriptor);}
 return{sha256:hash.digest("hex"),bytes};
}
""")

edit(ret, """async function entriesV1(directory:string,names:readonly string[]):Promise<RetainedEntryV1[]>{""", """function entriesV1(directory:string,names:readonly string[]):RetainedEntryV1[]{""")
edit(ret, """{name,sha256:"",bytes:size}:{name,...await digestV1(path)});""", """{name,sha256:"",bytes:size}:{name,...digestV1(path)});""")

edit(ret, """async function publishV1(source:string,destination:string,sha256:string):Promise<"published"|"same"|"conflict">{
 const temporary=join(dirname(destination),"."+basename(destination)+"."+randomBytes(4).toString("hex")+".tmp");
 try{
  copyFileSync(source,temporary,constants.COPYFILE_EXCL);
  if((await digestV1(temporary)).sha256!==sha256)throw Error("Lane provenance file changed while it was retained: "+basename(source));
  const descriptor=openSync(temporary,"r+");try{fsyncSync(descriptor);}finally{closeSync(descriptor);}
  try{linkSync(temporary,destination);return"published";}
  catch(error){
   const code=(error as NodeJS.ErrnoException).code;
   if(code==="EEXIST")return(await digestV1(destination)).sha256===sha256?"same":"conflict";
   if(!["EPERM","ENOTSUP","EXDEV","ENOSYS","EACCES"].includes(code??""))throw error;
   if(existsSync(destination))return(await digestV1(destination)).sha256===sha256?"same":"conflict";
   renameSync(temporary,destination);return"published";
  }
 }finally{rmSync(temporary,{force:true});}
}
""", """/** 🔎️ What a name that is already taken means: the same receipt (`same`) or anything else, a file with other bytes or an entry that cannot be read (`conflict`). */
function takenV1(destination:string,sha256:string):"same"|"conflict"{
 try{return digestV1(destination).sha256===sha256?"same":"conflict";}catch{return"conflict";}
}

/** 🪢️ The publication on a file system without hard links: the destination name is created exclusively from the verified temporary, so a name taken by a peer at any moment (whatever its bytes, even a link that leads nowhere) is never replaced, and the copy is read back; a copy that does not read back as the receipt is removed again, so no reader keeps a damaged receipt under a receipt name. */
export function publishWithoutLinkV1(temporary:string,destination:string,sha256:string):"published"|"same"|"conflict"{
 try{copyFileSync(temporary,destination,constants.COPYFILE_EXCL);}
 catch(error){if((error as NodeJS.ErrnoException).code==="EEXIST")return takenV1(destination,sha256);throw error;}
 if(digestV1(destination).sha256===sha256)return"published";
 rmSync(destination,{force:true});throw Error("Lane provenance file changed while it was retained: "+basename(destination));
}

function publishV1(source:string,destination:string,sha256:string):"published"|"same"|"conflict"{
 const temporary=join(dirname(destination),"."+basename(destination)+"."+randomBytes(4).toString("hex")+".tmp");
 try{
  copyFileSync(source,temporary,constants.COPYFILE_EXCL);
  if(digestV1(temporary).sha256!==sha256)throw Error("Lane provenance file changed while it was retained: "+basename(source));
  const descriptor=openSync(temporary,"r+");try{fsyncSync(descriptor);}finally{closeSync(descriptor);}
  try{linkSync(temporary,destination);return"published";}
  catch(error){
   const code=(error as NodeJS.ErrnoException).code;
   if(code==="EEXIST")return takenV1(destination,sha256);
   if(!["EPERM","ENOTSUP","EXDEV","ENOSYS","EACCES"].includes(code??""))throw error;
   return publishWithoutLinkV1(temporary,destination,sha256);
  }
 }finally{rmSync(temporary,{force:true});}
}
""")

edit(ret, """export async function retainLaneOutputsV1(input:Readonly<{lane:string;shared:string;laneId:string;language:"en"|"de";report?:(line:string)=>void}>):Promise<RetentionSummaryV1>{""",
          """export function retainLaneOutputsV1(input:Readonly<{lane:string;shared:string;laneId:string;language:"en"|"de";report?:(line:string)=>void}>):RetentionSummaryV1{""")
edit(ret, """  const lane=await entriesV1(from,names),present=regularNamesV1(to).filter(name=>names.includes(name)),shared=await entriesV1(to,present),""",
          """  const lane=entriesV1(from,names),present=regularNamesV1(to).filter(name=>names.includes(name)),shared=entriesV1(to,present),""")
edit(ret, """:await publishV1(join(from,action.name)""", """:publishV1(join(from,action.name)""")

edit(ret, """/** 🔗️ Runs `body` in a held lane, then publishes the lane's provenance""", """/** 📝️ Records beside a lane which shared ledger receives its receipts, so whoever removes the lane while its holder is gone knows where they belong. Best effort: a lane without the record retains nothing when it is removed. */
function recordDestinationV1(base:string,shared:string):void{
 try{const path=join(base,layout.destination),temporary=path+"."+randomBytes(3).toString("hex");writeFileSync(temporary,shared);renameSync(temporary,path);}catch{}
}

/** 🧹️ The retention of a lane that a trimmer removes while its holder is gone (killed between its body and its own retention): the receipts the lane still holds are published into the shared ledger the holder recorded before the lane goes, so the removal cannot lose the provenance of a build that finished. The caller holds the lane's claim and has seen it idle. A lane without a record, a record that is not an absolute path and a failing retention retain nothing and never block the removal (the pool bound is the hard limit; a killed build is rebuilt by its next run); the names that conflict with the shared ledger are returned.
 * @see ../🏊️pool/🟦️.ts */
export function retainRemovedLaneV1(paths:Readonly<{id:string;base:string;build:string}>):readonly string[]{
 let shared:string;try{shared=readFileSync(join(paths.base,layout.destination),"utf8");}catch{return[];}
 if(!isAbsolute(shared))return[];
 try{return retainLaneOutputsV1({lane:paths.build,shared,laneId:paths.id,language:"en"}).conflicts;}catch{return[];}
}

/** 🔗️ Runs `body` in a held lane, then publishes the lane's provenance""")
edit(ret, """ let value!:T,bodyFailed=false,bodyFailure:unknown,retentionFailed=false,retentionFailure:unknown;
 try{value=await body();}catch(error){bodyFailed=true;bodyFailure=error;}
 try{await retainLaneOutputsV1(""", """ let value!:T,bodyFailed=false,bodyFailure:unknown,retentionFailed=false,retentionFailure:unknown;
 recordDestinationV1(dirname(input.lane.paths.build),input.shared);
 try{value=await body();}catch(error){bodyFailed=true;bodyFailure=error;}
 try{retainLaneOutputsV1(""")

# lane: the removal hook
edit(lane, """/** 🗑️ Removes a whole lane or only its incremental directories. It takes the lane's own claim first, so it can never race an acquirer, and refuses while the claim is held or a live marker exists. */
export function removeLaneV1(root:string,id:string,scope:"lane"|"incremental",nowMs:number):boolean{""",
"""/** 🗑️ Removes a whole lane or only its incremental directories. It takes the lane's own claim first, so it can never race an acquirer, and refuses while the claim is held or a live marker exists; `beforeRemoval` runs for a whole lane under that claim, once the lane is known to be idle and before anything is deleted (the retention of what a holder that is gone left behind). */
export function removeLaneV1(root:string,id:string,scope:"lane"|"incremental",nowMs:number,beforeRemoval?:(paths:ReturnType<typeof lanePathsV1>)=>void):boolean{""")
edit(lane, """  if(scope==="lane")rmSync(paths.base,{recursive:true,force:true});else""", """  if(scope==="lane"){beforeRemoval?.(paths);rmSync(paths.base,{recursive:true,force:true});}else""")

# pool: retain before the trimmer removes
edit(pool, 'import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";\n', 'import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";\nimport {retainRemovedLaneV1} from "../📤️retention/🟦️.ts";\n')
edit(pool, """/** 🧹️ Removes the idle lanes the pool must shed. Every removal takes the lane's own claim first, so a lane that was taken since the plan survives; returns what was removed. */
export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number):Readonly<{removed:number;freedBytes:number}>{
 const lanes=readPoolV1(root,nowMs),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0]));let removed=0,freedBytes=0;
 for(const id of planPoolTrimV1(lanes,bounds))if(removeLaneV1(root,id,"lane",nowMs)){removed++;freedBytes+=size.get(id)!;}""",
"""/** 🧹️ Removes the idle lanes the pool must shed. Every removal takes the lane's own claim first, so a lane that was taken since the plan survives, and publishes the receipts a holder that was killed before its own retention left in the lane (see `retainRemovedLaneV1`) before the lane goes; returns what was removed. */
export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number,report?:PoolReportV1):Readonly<{removed:number;freedBytes:number}>{
 const lanes=readPoolV1(root,nowMs),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0]));let removed=0,freedBytes=0;
 const retain=(paths:ReturnType<typeof lanePathsV1>):void=>{const conflicts=retainRemovedLaneV1(paths);if(conflicts.length)report?.("retentionConflict",{lane:paths.id,count:conflicts.length,names:conflicts.slice(0,4).join(", ")});};
 for(const id of planPoolTrimV1(lanes,bounds))if(removeLaneV1(root,id,"lane",nowMs,retain)){removed++;freedBytes+=size.get(id)!;}""")
edit(pool, 'export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy",', 'export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy"|"retentionConflict",')
edit(pool, "const trimmed=trimPoolV1(root,bounds,now());", "const trimmed=trimPoolV1(root,bounds,now(),options.report);")
R = U + "/🧵️lane/📤️retention/🧪️tests"
newfile(R + "/🪦️dead-holder/🟦️.ts", "dead-holder.ts")
newfile(R + "/🪦️dead-holder/📥️worker/🟦️.ts", "dead-holder-worker.ts")
newfile(R + "/🪢️link-less/🟦️.ts", "link-less.ts")
