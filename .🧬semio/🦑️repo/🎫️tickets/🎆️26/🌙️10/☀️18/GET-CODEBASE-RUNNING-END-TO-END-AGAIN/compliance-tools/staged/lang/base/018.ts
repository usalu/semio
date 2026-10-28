/** 📸️ Captures the declared sources of a Bazel plan once into the generated workspace, so actions stage and hash the planned bytes however the live checkout changes before they start. */
import{createHash,randomUUID}from"node:crypto";
import{constants}from"node:fs";
import{chmod,copyFile,lstat,mkdir,open,readFile,readdir,rename,rm,rmdir,stat,unlink,utimes,writeFile}from"node:fs/promises";
import{dirname,join}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };

const policy=schema.definitions.Policy.const,absent=new Set(["ENOENT","ENOTDIR","ELOOP"]),code=(error:unknown)=>(error as NodeJS.ErrnoException).code??"";

/** 🧾️ One captured file: its content address, the live identity it was captured from and the modification times of its private copies. */
export type BazelSourceSnapshotRowV1={path:string;sha256:string;bytes:number;executable:boolean;liveMtimeNs:string;liveIno:string;racy:boolean;aliasMtimeNs:string;realMtimeNs?:string};

/** 🎟️ What one capture needs: the live checkout, the generated workspace, an optional content pool on the same volume whose identical files every package clones instead of copying, the two view directory names, the alias of a repository directory, the files to capture and the subset that the repository view names too. */
export interface BazelSourceSnapshotRequestV1{readonly root:string;readonly directory:string;readonly pool?:string;readonly files:readonly string[];readonly real?:ReadonlySet<string>;readonly views:{readonly alias:string;readonly real:string};readonly alias:(directory:string)=>string;readonly check:()=>void;readonly progress:()=>Promise<void>;readonly now?:()=>number}

/** 🗒️ What a plan asks its publisher to capture: the declarable files and the subset the repository view names too; the plan depends on which files exist, the capture on their bytes. */
export type BazelSourceSnapshotPlanV1={readonly version:1;readonly files:readonly string[];readonly real:readonly string[]};

/** 📊️ The captured files in order, the requested files that vanished before capture, the alias to directory mapping and what the capture did. */
export interface BazelSourceSnapshotV1{readonly files:string[];readonly vanished:string[];readonly directories:ReadonlyMap<string,string>;readonly captured:number;readonly reused:number;readonly pruned:number}

function requirePathV1(path:string):void{if(!path||path.startsWith("/")||/[\0\\]/u.test(path)||path.split("/").some(part=>!part||part==="."||part===".."))throw Error("Closed repository source path required: "+path);}

async function removeEntryV1(path:string):Promise<void>{
 let facts;try{facts=await lstat(path);}catch(error){if(absent.has(code(error)))return;throw error;}
 if(facts.isDirectory()&&!facts.isSymbolicLink()){await rm(path,{recursive:true,force:true});return;}
 try{await unlink(path);}catch(error){if(!["EISDIR","EPERM"].includes(code(error)))throw error;await rmdir(path);}
}

async function ensureTreeV1(path:string):Promise<void>{
 let facts;try{facts=await lstat(path);}catch(error){if(!absent.has(code(error)))throw error;}
 if(facts&&(facts.isSymbolicLink()||!facts.isDirectory())){await removeEntryV1(path);facts=undefined;}
 if(!facts)await mkdir(path,{recursive:true});
}

async function hashFileV1(path:string):Promise<{sha256:string;bytes:number}>{
 const file=await open(path,"r");
 try{const hash=createHash("sha256"),buffer=Buffer.allocUnsafe(policy.readChunkBytes);let bytes=0;for(;;){const row=await file.read(buffer,0,buffer.length,null);if(!row.bytesRead)break;hash.update(buffer.subarray(0,row.bytesRead));bytes+=row.bytesRead;}return{sha256:hash.digest("hex"),bytes};}
 finally{await file.close();}
}

async function eachBoundedV1<T>(items:readonly T[],limit:number,work:(item:T)=>Promise<void>):Promise<void>{
 let next=0,failure:{error:unknown}|undefined;
 const run=async()=>{for(;;){if(failure)return;const index=next++;if(index>=items.length)return;try{await work(items[index]!);}catch(error){failure??={error};return;}}};
 await Promise.all(Array.from({length:Math.min(limit,items.length)},run));if(failure)throw failure.error;
}

/** 🗺️ Maps the alias of every directory that holds a requested file to that directory and refuses collisions: the planner names its alias packages by it before anything is captured, the capture places the copies by the same map. */
export function bazelSourceSnapshotDirectoriesV1(files:readonly string[],alias:(directory:string)=>string):Map<string,string>{
 const directories=new Map<string,string>(),seen=new Set<string>();
 for(const path of files){requirePathV1(path);const folder=path.includes("/")?path.slice(0,path.lastIndexOf("/")):"";if(seen.has(folder))continue;seen.add(folder);const name=alias(folder),known=directories.get(name);if(known!==undefined)throw Error(`Bazel source alias collision: ${name}: ${known} and ${folder}`);directories.set(name,folder);}
 return directories;
}

/** 🧷️ The entries of a workspace root that the capture owns (manifest, scratch directory and both views): the publisher rewrites them after every plan, so they are never part of what a plan record proves. */
export function bazelSourceSnapshotEntriesV1(views:{readonly alias:string;readonly real:string}):readonly string[]{return [policy.manifest,policy.temporary,views.alias,views.real];}

/** 📛️ The name of the capture plan in a workspace root. */
export const bazelSourceSnapshotPlanFileV1:string=policy.plan;

/** ✒️ The text of the capture plan the planner leaves in a workspace: sorted and duplicate free, so an unchanged file set gives unchanged bytes. */
export function bazelSourceSnapshotPlanTextV1(files:Iterable<string>,real:Iterable<string>):string{return JSON.stringify({version:1,files:[...new Set(files)].sort(),real:[...new Set(real)].sort()});}

/** 📥️ Reads the capture plan a planner left in a workspace; a missing or malformed plan is refused, never read as empty. */
export async function readBazelSourceSnapshotPlanV1(directory:string):Promise<BazelSourceSnapshotPlanV1>{
 const path=join(directory,policy.plan);
 if((await stat(path)).size>policy.maximumManifestBytes)throw Error("Bounded Bazel source snapshot plan required");
 const value:unknown=JSON.parse(await readFile(path,"utf8"));
 if(validateJsonSchemaSubset(schema.definitions.Plan,value,schema).length)throw Error("Closed Bazel source snapshot plan required");
 return value as BazelSourceSnapshotPlanV1;
}

/** 📖️ Reads the manifest that a workspace's last capture published; a missing, oversized or malformed manifest reads as empty so the next capture starts from the live files. */
export async function readBazelSourceSnapshotV1(directory:string):Promise<Map<string,BazelSourceSnapshotRowV1>>{
 const path=join(directory,policy.manifest);let text:string;
 try{if((await stat(path)).size>policy.maximumManifestBytes)return new Map();text=await readFile(path,"utf8");}catch(error){if(absent.has(code(error)))return new Map();throw error;}
 let value:unknown;try{value=JSON.parse(text);}catch{return new Map();}
 if(validateJsonSchemaSubset(schema.definitions.Manifest,value,schema).length)return new Map();
 return new Map((value as {rows:BazelSourceSnapshotRowV1[]}).rows.map(row=>[row.path,row]));
}

/** 🔎️ Names the declared files whose live identity differs from the captured one (changed, replaced or removed since the capture). */
export async function bazelSourceSnapshotDriftV1(root:string,rows:ReadonlyMap<string,BazelSourceSnapshotRowV1>,paths:Iterable<string>):Promise<string[]>{
 const drifted:string[]=[];
 for(const path of paths){const row=rows.get(path);if(!row){drifted.push(path);continue;}let facts;try{facts=await stat(join(root,path),{bigint:true});}catch(error){if(absent.has(code(error))){drifted.push(path);continue;}throw error;}if(!facts.isFile()||facts.mtimeNs.toString()!==row.liveMtimeNs||facts.ino.toString()!==row.liveIno||facts.size!==BigInt(row.bytes))drifted.push(path);}
 return drifted;
}

const clonable=new Map<string,Promise<boolean>>(),poolFailures=new Set(["ENOTSUP","EOPNOTSUPP","EXDEV","ENOSYS","EINVAL","EPERM"]);
/** 🧪️ Whether the pool directory lies on a volume that clones files copy-on-write (APFS, btrfs, XFS, ReFS): one tiny file is cloned once per process and pool; anywhere else the capture keeps plain private copies. */
async function poolClonesV1(pool:string):Promise<boolean>{
 let known=clonable.get(pool);
 if(!known){
  known=(async()=>{const probe=join(pool,policy.poolProbe+"-"+process.pid);try{await mkdir(pool,{recursive:true});await writeFile(probe,"x");await copyFile(probe,probe+".clone",constants.COPYFILE_FICLONE_FORCE);return true;}catch{return false;}finally{await rm(probe,{force:true}).catch(()=>undefined);await rm(probe+".clone",{force:true}).catch(()=>undefined);}})();
  clonable.set(pool,known);
 }
 return known;
}
/** 🧬️ Places a captured file at its destination: a pool hit is a copy-on-write clone of the entry (its change time is renewed first, so the collector, which only removes entries by that time, leaves it alone), a miss, an entry that vanished meanwhile or a collector that emptied the folder just places the captured copy itself and then re-seeds the pool with a clone of it. A missing pool entry therefore never fails a capture; false only means the volume refused cloning, which switches the pool off for this process and leaves the captured copy for a plain placement. */
async function placeFromPoolV1(pool:string,holder:string,destination:string,clone:string,digest:Readonly<{sha256:string;bytes:number}>,executable:boolean):Promise<boolean>{
 const entry=join(pool,digest.sha256.slice(0,policy.poolFanout),digest.sha256+(executable?policy.poolExecutableSuffix:""));
 try{
  const hit=await lstat(entry).then(facts=>facts.isFile()&&facts.size===digest.bytes,error=>{if(absent.has(code(error)))return false;throw error;});
  if(hit)try{const facts=await stat(entry);await utimes(entry,facts.atime,facts.mtime);await copyFile(entry,clone,constants.COPYFILE_FICLONE_FORCE);await rename(clone,destination);await rm(holder,{force:true});return true;}
  catch(error){await rm(clone,{force:true}).catch(()=>undefined);if(!absent.has(code(error)))throw error;}
  await rename(holder,destination);
  const seed=entry+"."+randomUUID()+".tmp";
  try{await mkdir(dirname(entry),{recursive:true});await copyFile(destination,seed,constants.COPYFILE_FICLONE_FORCE);await rename(seed,entry);}
  catch(error){await rm(seed,{force:true}).catch(()=>undefined);if(poolFailures.has(code(error)))clonable.set(pool,Promise.resolve(false));else if(!absent.has(code(error)))throw error;}
  return true;
 }catch(error){
  await rm(clone,{force:true}).catch(()=>undefined);
  if(!poolFailures.has(code(error)))throw error;
  clonable.set(pool,Promise.resolve(false));return false;
 }
}

/** 🧊️ Captures every requested file as a private regular copy `<alias view>/<alias>/<name>` (and `<repository view>/<path>` for the repository-named subset) and publishes `snapshot.json`; copies are clones or plain copies and never hardlinks, a file is read from the live tree once, a torn capture retries a bounded number of times, and an unchanged file keeps its copy so Bazel's state stays stable. A legacy view that links into the live tree is unlinked before anything is written. */
export async function captureBazelSourceSnapshotV1(request:BazelSourceSnapshotRequestV1):Promise<BazelSourceSnapshotV1>{
 const{root,directory,views,check}=request,wanted=[...new Set(request.files)].sort(),real=request.real??new Set<string>();
 if(wanted.length>policy.maximumFiles)throw Error("Bazel source snapshot capacity exceeded");
 const directories=bazelSourceSnapshotDirectoriesV1(wanted,request.alias),aliases=new Map([...directories].map(([alias,folder])=>[folder,alias])),aliasOf=(folder:string)=>aliases.get(folder)!;
 const aliasRoot=join(directory,views.alias),realRoot=join(directory,views.real),temporary=join(directory,policy.temporary);
 const aliasDestination=(path:string)=>{const cut=path.lastIndexOf("/");return join(aliasRoot,aliasOf(cut<0?"":path.slice(0,cut)),path.slice(cut+1));},realDestination=(path:string)=>join(realRoot,...path.split("/"));
 const pool=request.pool!==undefined&&await poolClonesV1(request.pool)?request.pool:undefined;
 await ensureTreeV1(aliasRoot);await ensureTreeV1(realRoot);await rm(temporary,{recursive:true,force:true});await mkdir(temporary,{recursive:true});
 const prior=await readBazelSourceSnapshotV1(directory),current=new Set(wanted),made=new Map<string,Promise<void>>(),rows:BazelSourceSnapshotRowV1[]=[],vanished:string[]=[];let pruned=0,captured=0,reused=0,done=0,counter=0;
 const emptied=new Set<string>(),removeRealV1=async(path:string)=>{await removeEntryV1(realDestination(path));for(let folder=dirname(realDestination(path));folder.length>realRoot.length;folder=dirname(folder))emptied.add(folder);};
 for(const[path,row]of prior)if(!current.has(path)){const cut=path.lastIndexOf("/");await removeEntryV1(join(aliasRoot,request.alias(cut<0?"":path.slice(0,cut)),path.slice(cut+1)));if(row.realMtimeNs!==undefined)await removeRealV1(path);pruned++;}
 for(const entry of await readdir(aliasRoot)){const path=join(aliasRoot,entry);if(!directories.has(entry)||(await lstat(path)).isSymbolicLink())await removeEntryV1(path);}
 const ensureDirectory=(path:string,tree:boolean)=>{let known=made.get(path);if(!known){known=tree?ensureTreeV1(path):mkdir(path,{recursive:true}).then(()=>undefined);made.set(path,known);}return known;};
 const intact=async(path:string,mtimeNs:string,bytes:number)=>{try{const facts=await lstat(path,{bigint:true});return facts.isFile()&&!facts.isSymbolicLink()&&facts.nlink===1n&&facts.size===BigInt(bytes)&&facts.mtimeNs.toString()===mtimeNs;}catch(error){if(absent.has(code(error)))return false;throw error;}};
 const place=async(source:string,destination:string,executable:boolean,tree:boolean):Promise<string>=>{
  const holder=join(temporary,String(counter++));
  try{await copyFile(source,holder,constants.COPYFILE_FICLONE);await chmod(holder,executable?policy.executableMode:policy.fileMode);await ensureDirectory(dirname(destination),tree);await rename(holder,destination);}
  finally{await rm(holder,{force:true});}
  return (await lstat(destination,{bigint:true})).mtimeNs.toString();
 };
 const capture=async(path:string,live:string,first:import("node:fs").BigIntStats):Promise<BazelSourceSnapshotRowV1|undefined>=>{
  const destination=aliasDestination(path);
  for(let attempt=1;attempt<=policy.attempts;attempt++){
   check();const before=attempt===1?first:await stat(live,{bigint:true}).catch(error=>{if(absent.has(code(error)))return undefined;throw error;});
   if(!before?.isFile())return undefined;if(before.size>BigInt(policy.maximumFileBytes))throw Error("Bounded Bazel source file required: "+path);
   const holder=join(temporary,String(counter++));let copied=false;
   try{
    try{await copyFile(live,holder,constants.COPYFILE_FICLONE);copied=true;}catch(error){if(absent.has(code(error)))return undefined;throw error;}
    const after=await stat(live,{bigint:true}).catch(error=>{if(absent.has(code(error)))return undefined;throw error;});
    if(!after?.isFile())return undefined;
    const digest=await hashFileV1(holder);
    if(after.size!==before.size||after.mtimeNs!==before.mtimeNs||after.ino!==before.ino||BigInt(digest.bytes)!==before.size)continue;
    const executable=(before.mode&0o111n)!==0n,racy=(request.now?.()??Date.now())-Number(before.mtimeNs/1000000n)<policy.racyMilliseconds;
    await chmod(holder,executable?policy.executableMode:policy.fileMode);await ensureDirectory(dirname(destination),true);if(!(pool!==undefined&&await placeFromPoolV1(pool,holder,destination,join(temporary,String(counter++)),digest,executable)))await rename(holder,destination);copied=false;
    return{path,sha256:digest.sha256,bytes:digest.bytes,executable,liveMtimeNs:before.mtimeNs.toString(),liveIno:before.ino.toString(),racy,aliasMtimeNs:(await lstat(destination,{bigint:true})).mtimeNs.toString()};
   }finally{if(copied)await rm(holder,{force:true});}
  }
  throw Error(`Bazel source changed during all ${policy.attempts} capture attempts: ${path}`);
 };
 await eachBoundedV1(wanted,policy.concurrency,async path=>{
  check();const live=join(root,path),row=prior.get(path);let facts;
  try{facts=await stat(live,{bigint:true});}catch(error){if(!absent.has(code(error)))throw error;}
  const gone=async()=>{vanished.push(path);if(row){await removeEntryV1(aliasDestination(path));if(row.realMtimeNs!==undefined)await removeRealV1(path);}};
  if(!facts?.isFile()){await gone();return;}
  let next:BazelSourceSnapshotRowV1|undefined,fresh=false;
  if(row&&!row.racy&&row.liveMtimeNs===facts.mtimeNs.toString()&&row.liveIno===facts.ino.toString()&&BigInt(row.bytes)===facts.size&&row.executable===((facts.mode&0o111n)!==0n)&&await intact(aliasDestination(path),row.aliasMtimeNs,row.bytes)){next={...row};reused++;}
  else{next=await capture(path,live,facts);if(!next){await gone();return;}fresh=true;captured++;}
  if(real.has(path)){if(fresh||next.realMtimeNs===undefined||!await intact(realDestination(path),next.realMtimeNs,next.bytes))next.realMtimeNs=await place(aliasDestination(path),realDestination(path),next.executable,false);}
  else if(next.realMtimeNs!==undefined){await removeRealV1(path);delete next.realMtimeNs;}
  rows.push(next);
  if(++done%policy.progressFiles===0)await request.progress();
 });
 for(const folder of[...emptied].sort((left,right)=>right.length-left.length)){try{await rmdir(folder);}catch(error){if(!["ENOTEMPTY","EEXIST","ENOENT","EBUSY","EPERM"].includes(code(error)))throw error;}}
 rows.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);vanished.sort();
 const manifest=join(directory,policy.manifest),holder=manifest+".tmp";
 await writeFile(holder,JSON.stringify({version:1,rows}));await rename(holder,manifest);await rm(temporary,{recursive:true,force:true});
 return Object.freeze({files:rows.map(row=>row.path),vanished,directories,captured,reused,pruned});
}

/** 💬️ Renders the one-line summary of a capture in the requested language. */
export function renderBazelSourceSnapshotV1(result:{readonly files:readonly string[];readonly vanished:readonly string[];readonly captured:number;readonly reused:number;readonly pruned:number},language:"en"|"de"):string{
 const values:Readonly<Record<string,number>>={files:result.files.length,captured:result.captured,reused:result.reused,pruned:result.pruned,vanished:result.vanished.length};
 return schema.definitions.Messages.const[language].summary.replace(/\{(\w+)\}/gu,(_match,name:string)=>String(values[name]));
}
