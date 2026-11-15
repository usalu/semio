/** 🧱️ One unit of a pack: a directory subtree or a single file at the depth where packages live (`node_modules/<name>`, `node_modules/@scope/<name>`), listed pre-order and sorted, with the stat signature that lets a later publish reuse its blob. */
type PackUnitV1=Readonly<{path:string;entries:readonly string[];signature:string}>;
type PackBlobV1=Readonly<{sha256:string;bytes:number}>;

const blobPathOf=(directory:string,sha256:string):string=>join(directory,...policy.installBlobDirectory,sha256.slice(0,2),sha256);
const blobTemporaryDirectory=(directory:string):string=>join(directory,...policy.installBlobDirectory);
const tarOwnerArguments=(platform:NodeJS.Platform):readonly string[]=>platform==="linux"?["--owner=0","--group=0","--numeric-owner"]:["--uid","0","--gid","0","--uname","","--gname",""];

async function placeBlobV1(directory:string,temporary:string,blob:PackBlobV1):Promise<void>{
 const target=blobPathOf(directory,blob.sha256),have=await stat(target).catch(()=>undefined);
 if(have?.isFile()&&have.size===blob.bytes){await rm(temporary,{force:true});const now=new Date();await utimes(target,now,now).catch(()=>{});return;}
 await mkdir(dirname(target),{recursive:true});await rename(temporary,target);
}

/** 🗺️ Splits the roots of a generation into the pieces a pack stores: directories to create, links to recreate and units to archive. A scope folder (`@scope`) is a directory whose children are units; an empty folder is a directory entry, not a unit. */
export async function planFleetInstallPackV1(directory:string,roots:readonly string[],check:()=>void):Promise<Readonly<{directories:readonly string[];links:readonly Readonly<{path:string;target:string}>[];units:readonly PackUnitV1[]}>>{
 const directories:string[]=[],links:{path:string;target:string}[]=[],units:PackUnitV1[]=[];
 const collect=async(absolute:string,relative:string,entries:string[],lines:string[]):Promise<void>=>{
  check();
  const facts=await lstat(absolute);entries.push(relative);
  if(facts.isSymbolicLink()){lines.push(`l ${relative} ${await readlink(absolute)}`);return;}
  if(facts.isDirectory()){lines.push(`d ${relative} ${facts.mode&0o777}`);for(const name of (await readdir(absolute)).sort())await collect(join(absolute,name),`${relative}/${name}`,entries,lines);return;}
  lines.push(`f ${relative} ${facts.size} ${Math.floor(facts.mtimeMs/1000)} ${facts.mode&0o777}`);
 };
 const level=async(absolute:string,relative:string,scoped:boolean):Promise<void>=>{
  for(const name of (await readdir(absolute)).sort()){
   check();
   const child=join(absolute,name),here=`${relative}/${name}`,facts=await lstat(child);
   if(facts.isSymbolicLink()){links.push({path:here,target:await readlink(child)});continue;}
   if(facts.isDirectory()&&scoped&&name.startsWith("@")){directories.push(here);await level(child,here,false);continue;}
   const entries:string[]=[],lines:string[]=[];await collect(child,here,entries,lines);
   if(facts.isDirectory()&&entries.length===1){directories.push(here);continue;}
   units.push({path:here,entries,signature:createHash("sha256").update(lines.join("\n")).digest("hex")});
  }
 };
 for(const root of roots){
  const absolute=join(directory,...root.split("/")),facts=await lstat(absolute).catch(()=>undefined);
  if(!facts)continue;
  if(facts.isSymbolicLink()){links.push({path:root,target:await readlink(absolute)});continue;}
  directories.push(root);await level(absolute,root,true);
 }
 return {directories,links,units};
}

async function packUnitV1(request:FleetInstallRequestV1,blobs:string,unit:PackUnitV1,cache:Map<string,PackBlobV1>):Promise<PackBlobV1>{
 const known=cache.get(unit.signature);
 if(known){const have=await stat(blobPathOf(blobs,known.sha256)).catch(()=>undefined);if(have?.isFile()&&have.size===known.bytes){const now=new Date();await utimes(blobPathOf(blobs,known.sha256),now,now).catch(()=>{});return known;}}
 const scratch=blobTemporaryDirectory(blobs);await mkdir(scratch,{recursive:true});
 const stem=join(scratch,`.${randomBytes(8).toString("hex")}`),listing=stem+".list",temporary=stem+".tmp",hash=createHash("sha256");
 let bytes=0;
 try{
  await writeFile(listing,unit.entries.map(name=>name.startsWith("-")?"./"+name:name).join("\n")+"\n");
  const child=runTar(["-cf","-","--no-xattrs","--no-recursion",...tarOwnerArguments(process.platform),"-C",request.directory,"-T",listing],request.directory,request.check,["ignore","pipe","pipe"]);
  child.stderr?.resume();
  const exit=closed(child);
  const counted=async function*(source:AsyncIterable<Buffer>){for await(const chunk of source){hash.update(chunk);bytes+=chunk.byteLength;yield chunk;}};
  await pipeline(child.stdout!,createGzip({level:1}),counted,createWriteStream(temporary,{flags:"wx",mode:0o644}));
  if(await exit!==0)throw Error("tar refused a unit of the install roots");
  const blob={sha256:hash.digest("hex"),bytes};
  await placeBlobV1(blobs,temporary,blob);
  cache.set(unit.signature,blob);
  return blob;
 }finally{await rm(listing,{force:true}).catch(()=>{});await rm(temporary,{force:true}).catch(()=>{});}
}

const unitCachePath=(root:string):string=>join(root,".cache","f","install-pack-units.json");
const readUnitCache=async(root:string):Promise<Map<string,PackBlobV1>>=>{
 try{const value=JSON.parse(await readFile(unitCachePath(root),"utf8")) as {version?:number;rows?:Record<string,PackBlobV1>};return new Map(value.version===1?Object.entries(value.rows??{}):[]);}catch{return new Map();}
};
const writeUnitCache=async(root:string,cache:Map<string,PackBlobV1>):Promise<void>=>{
 try{
  const rows=Object.fromEntries([...cache].slice(-policy.installUnitCacheEntries)),path=unitCachePath(root),temporary=path+`.${randomBytes(4).toString("hex")}.tmp`;
  await mkdir(dirname(path),{recursive:true});await writeFile(temporary,JSON.stringify({version:1,rows}));await rename(temporary,path);
 }catch{}
};

const poolV1=async<T>(items:readonly T[],width:number,work:(item:T)=>Promise<void>):Promise<void>=>{
 let next=0,failure:unknown;
 await Promise.all(Array.from({length:Math.min(width,items.length)},async()=>{for(;;){if(failure!==undefined)return;const index=next++;if(index>=items.length)return;try{await work(items[index]!);}catch(error){failure??=error;return;}}}));
 if(failure!==undefined)throw failure;
};

/** 📤️ Packs the roots of a complete installed generation into content-addressed unit blobs in the group store (one gzip tar per package, named by its own SHA-256, so a package that several generations share is stored once), then an index blob, then the signed manifest. Blobs are renamed into place before the index and the index before the manifest, so a reader that sees the manifest sees every blob. Units whose stat signature a previous publish of this machine packed are reused without reading them. Refuses a tree with a link that cannot move to another worktree or past the size bound; `false` leaves nothing referenced behind. */
export async function fleetInstallPublishV1(request:FleetInstallRequestV1,replace:boolean=false):Promise<boolean>{
 const {context}=request,directory=directoryOf(context),key=fleetInstallKeyV1(request.projectionSha256,request.runtime),installed=request.installed();
 if(directory===undefined||installed===undefined||context.root===undefined)return false;
 const lock=await lockOf(request.directory);
 if(lock===undefined){request.event?.({stage:"skip",detail:"no bun.lock to bind the pack to"});return false;}
 if(!replace&&await fleetInstallManifestV1(context,key)){request.event?.({stage:"skip",detail:"published"});return true;}
 const roots=[];for(const root of request.roots)if(await lstat(join(request.directory,...root.split("/"))).then(()=>true,()=>false))roots.push(root);
 if(roots.length===0)return false;
 const origin=readFleetMachineV1(context.root)?.originId;
 if(origin===undefined||signFleetDocumentV1(context.root,"cache-write","install-pack","probe")?.origin!==origin){request.event?.({stage:"skip",detail:"this machine has no fleet identity to sign the pack"});return false;}
 const bad=await firstNonRelocatableLinkV1(request.directory,roots,request.check,request.storeRoot);
 if(bad!==undefined){request.event?.({stage:"skip",detail:`non-relocatable link ${bad}`});return false;}
 const shape=await request.scan();request.check();const fingerprint=await fleetInstallFingerprintV1(request.directory,roots,request.storeRoot,request.check);
 await mkdir(directory,{recursive:true});
 try{
  const plan=await planFleetInstallPackV1(request.directory,roots,request.check),cache=await readUnitCache(context.root),packed:{path:string;sha256:string;bytes:number}[]=[];
  let total=0;
  await poolV1(plan.units,policy.installPackConcurrency,async unit=>{
   const blob=await packUnitV1(request,directory,unit,cache);
   packed.push({path:unit.path,...blob});total+=blob.bytes;
   if(total>policy.installMaximumBytes)throw Error("Install pack past its bound");
  });
  request.check();
  packed.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
  const index={version:1 as const,directories:[...plan.directories].sort(),links:[...plan.links].sort((left,right)=>left.path<right.path?-1:1),units:packed};
  guardIndex(index);
  const text=gzipSync(Buffer.from(JSON.stringify(index)),{level:6}),indexBlob={sha256:createHash("sha256").update(text).digest("hex"),bytes:text.byteLength};
  const scratch=blobTemporaryDirectory(directory);await mkdir(scratch,{recursive:true});
  const temporary=join(scratch,`.${randomBytes(8).toString("hex")}.tmp`);await writeFile(temporary,text,{flag:"wx",mode:0o644});await placeBlobV1(directory,temporary,indexBlob);
  await writeUnitCache(context.root,cache);
  const draft={version:1 as const,key,projectionSha256:request.projectionSha256,runtime:request.runtime,installed,shape,storeRoot:request.storeRoot,fingerprint,roots,index:{...indexBlob,units:packed.length,links:plan.links.length,packedBytes:total},lock,createdAtMs:context.now()};
  const signed=signFleetDocumentV1(context.root,"cache-write","install-pack",signedText(draft,origin));
  if(signed===undefined||signed.origin!==origin){request.event?.({stage:"skip",detail:"this machine has no fleet identity to sign the pack"});return false;}
  const manifest:FleetInstallManifestV1={...draft,publisher:{origin:signed.origin,signature:signed.signature}};
  guard(manifest,"Closed install manifest required");
  const staged=join(directory,`.${randomBytes(8).toString("hex")}.json.tmp`);
  await writeFile(staged,JSON.stringify(manifest)+"\n",{flag:"wx",mode:0o644});
  await rename(staged,join(directory,key+policy.installManifestSuffix));
  request.event?.({stage:"publish",detail:key,bytes:total});
  await announce(context,key,manifest);
  await pruneFleetInstallsV1(directory,context.now()).catch(()=>{});
  return true;
 }catch(error){
  if(error instanceof Error&&/cancel|abort/iu.test(error.message))throw error;
  request.event?.({stage:"skip",detail:`publish failed: ${error instanceof Error?error.message:String(error)}`});
  return false;
 }
}

/** 📚️ Reads the index blob a manifest names; `undefined` when it is missing, short, corrupt or not a closed index. */
export async function fleetInstallIndexV1(directory:string,manifest:FleetInstallManifestV1):Promise<FleetInstallIndexV1|undefined>{
 try{
  const bytes=await readFile(blobPathOf(directory,manifest.index.sha256));
  if(bytes.byteLength!==manifest.index.bytes||createHash("sha256").update(bytes).digest("hex")!==manifest.index.sha256)return undefined;
  const value=JSON.parse(gunzipSync(bytes).toString("utf8")) as FleetInstallIndexV1;guardIndex(value);return value;
 }catch{return undefined;}
}

/** 🧹️ Keeps the newest `installKeepStates` published states and drops every older manifest, then every blob no kept manifest or index names (blobs and temporary files younger than an hour stay: a publisher may be about to name them); a store holds a few lock states, never every one the fleet has seen. */
export async function pruneFleetInstallsV1(directory:string,now:number):Promise<number>{
 const rows:{name:string;createdAtMs:number;manifest:FleetInstallManifestV1}[]=[];let removed=0;
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  const path=join(directory,name);
  if(name.endsWith(".tmp")){const facts=await stat(path).catch(()=>undefined);if(facts&&now-facts.mtimeMs>3600000){await rm(path,{force:true});removed++;}continue;}
  if(!name.endsWith(policy.installManifestSuffix))continue;
  try{const value=JSON.parse(await readFile(path,"utf8")) as FleetInstallManifestV1;guard(value,"Closed install manifest required");rows.push({name,createdAtMs:value.createdAtMs,manifest:value});}catch{}
 }
 rows.sort((left,right)=>right.createdAtMs-left.createdAtMs);
 for(const row of rows.slice(policy.installKeepStates)){await rm(join(directory,row.name),{force:true});removed++;}
 const referenced=new Set<string>();
 for(const row of rows.slice(0,policy.installKeepStates)){
  const index=await fleetInstallIndexV1(directory,row.manifest);
  if(index===undefined)return removed;
  referenced.add(row.manifest.index.sha256);for(const unit of index.units)referenced.add(unit.sha256);
 }
 const base=join(directory,...policy.installBlobDirectory);
 for(const folder of await readdir(base).catch(()=>[] as string[])){
  const inside=join(base,folder);
  if(folder.startsWith(".")){const facts=await stat(inside).catch(()=>undefined);if(facts&&now-facts.mtimeMs>3600000){await rm(inside,{force:true});removed++;}continue;}
  for(const name of await readdir(inside).catch(()=>[] as string[])){
   if(referenced.has(name))continue;
   const facts=await stat(join(inside,name)).catch(()=>undefined);
   if(facts&&now-facts.mtimeMs>3600000){await rm(join(inside,name),{force:true});removed++;}
  }
 }
 return removed;
}
