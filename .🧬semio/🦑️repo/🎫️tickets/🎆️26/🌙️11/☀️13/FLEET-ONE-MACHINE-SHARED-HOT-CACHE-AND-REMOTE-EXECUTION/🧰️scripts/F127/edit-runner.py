import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:80],s.count(old));s=s.replace(old,new)
    tmp=path+".f127tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
R=B+"🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts"
SR="../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts"
pairs=[
('import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";\n','import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";\nimport {createNativeImageSharedReaderV1,type NativeImageSharedReaderV1} from "'+SR+'";\n'),
# ports
(' platform?:NodeJS.Platform;\n delegate?(',' platform?:NodeJS.Platform;\n images?:CacheImagesV1;\n delegate?('),
# types + factory after ports
('/** 🧟️ Asks the operating system whether a process still exists;',
'''/** 👓️ What the collector asks of the image a path lives in: `run` executes one short step under the shared lease of that image (a path outside every image runs the step as it is). */
export type CacheImageReaderV1=Pick<NativeImageSharedReaderV1,"run">;
/** 🖼️ The shared readers of the cache images of a workspace, one per image, made on first use: `reader` answers the reader of the image a path lies in, `settle` gives every lease back and forgets the readers (before an owner takes an image exclusively, and at the end of a command). */
export type CacheImagesV1=Readonly<{reader(path:string):CacheImageReaderV1;settle():Promise<void>}>;
const direct:CacheImageReaderV1=Object.freeze({run:async<T>(body:()=>T|Promise<T>):Promise<T>=>body()});

/** 🔎️ The reader of the image a path lies in; without readers, or for a path outside every image, the step runs as it is. */
export function cacheImageReaderOfV1(images:CacheImagesV1|undefined,path:string):CacheImageReaderV1{return images?.reader(path)??direct;}

/** 🏭️ Makes the readers of the images of the declared partitions: a step holds the shared lease of its image only while it runs, a hold ends after the idle release or the hold limit of the reader, and a wait behind a waiting exclusive writer ends after `timeoutMs`. The partition that holds the lease directory has no reader (a lease cannot guard its own directory). */
export function createCacheImagesV1(input:Readonly<{workspace:string;partitions:CachePartitionBudgetsV1;leaseDirectory:string;signal:AbortSignal;timeoutMs:number}>):CacheImagesV1{
 const leases=resolve(input.leaseDirectory),directories=Object.values(input.partitions).map(partition=>join(input.workspace,partition.directory)).filter(directory=>!insideBase(resolve(directory),leases)),readers=new Map<string,NativeImageSharedReaderV1>();
 return Object.freeze({
  reader(path:string):CacheImageReaderV1{
   const target=resolve(path),image=directories.find(directory=>target===resolve(directory)||insideBase(resolve(directory),target));
   if(image===undefined)return direct;
   let reader=readers.get(image);
   if(reader===undefined){reader=createNativeImageSharedReaderV1({leaseDirectory:input.leaseDirectory,imageDirectory:image,signal:input.signal,timeoutMs:input.timeoutMs});readers.set(image,reader);}
   return reader;
  },
  async settle():Promise<void>{const open=[...readers.values()];readers.clear();for(const reader of open)await reader.close();}
 });
}

/** 🚫️ Whether an error is a lease wait that ended: another process holds or awaits the image exclusively. */
export function cacheLeaseRefusedV1(error:unknown):boolean{return /timed out/iu.test(String((error as Error)?.message??error));}

/** 🧟️ Asks the operating system whether a process still exists;'''),
# measure
('''/** 🚶️ Walks one tree without following links, counts hardlinks once, takes recency from file times only (directory times only when no file exists), and stops at the entry limit or on cancellation. */
export async function measureCacheTreeV1(path:string,ports:CacheRunnerPortsV1,limit:number,stage:string=""):Promise<CacheMeasureV1>{
 let first;try{first=await lstat(path);}catch''','''/** 🚶️ Walks one tree without following links, counts hardlinks once, takes recency from file times only (directory times only when no file exists), and stops at the entry limit or on cancellation. Every touch of the tree is one short step under the shared lease of its image: one directory listing, or one batch of `checkEvery` entries; no descriptor outlives a step. */
export async function measureCacheTreeV1(path:string,ports:CacheRunnerPortsV1,limit:number,stage:string=""):Promise<CacheMeasureV1>{
 const reader=cacheImageReaderOfV1(ports.images,path);
 let first;try{first=await reader.run(()=>lstat(path));}catch'''),
('''  await tick();const directory=stack.pop()!;let children;
  try{children=await readdir(directory,{withFileTypes:true});}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT"||code==="ENOTDIR")continue;if(code==="EACCES"||code==="EPERM"){partial=true;continue;}throw error;}
  if(directory.endsWith(".queue")&&queues.length<runtime.sqliteProbesPerUnit)queues.push(directory);
  for(const child of children){
   if(++entries>limit){partial=true;stack.length=0;break;}
   if(entries%runtime.checkEvery===0)await tick();
   const full=join(directory,child.name);
   if(child.isDirectory()){stack.push(full);try{directories=Math.max(directories,(await lstat(full)).mtimeMs);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){partial=true;directories=Math.max(directories,ports.now());recency=Math.max(recency,ports.now());}}continue;}
   let info;try{info=await lstat(full);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){partial=true;recency=Math.max(recency,ports.now());}continue;}
   if(info.isSymbolicLink())continue;
   if(info.nlink>1){const exact=Number.isSafeInteger(info.ino)?info:await lstat(full,{bigint:true}).catch(()=>info),key=exact.dev+":"+exact.ino;if(seen.has(key))continue;seen.add(key);}
   bytes+=allocated(info);files++;recency=Math.max(recency,cacheRecencyMsV1(info,ports.now()));
   if(child.name.endsWith(".sqlite")&&sqlite.length<runtime.sqliteProbesPerUnit)sqlite.push(full);
  }
 }
''','''   await tick();const directory=stack.pop()!;let children;
  try{children=await reader.run(()=>readdir(directory,{withFileTypes:true}));}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT"||code==="ENOTDIR")continue;if(code==="EACCES"||code==="EPERM"){partial=true;continue;}throw error;}
  if(directory.endsWith(".queue")&&queues.length<runtime.sqliteProbesPerUnit)queues.push(directory);
  for(let from=0;from<children.length;from+=runtime.checkEvery){
   await tick();
   const stop=await reader.run(async():Promise<boolean>=>{
    for(const child of children.slice(from,from+runtime.checkEvery)){
     if(++entries>limit){partial=true;return true;}
     const full=join(directory,child.name);
     if(child.isDirectory()){stack.push(full);try{directories=Math.max(directories,(await lstat(full)).mtimeMs);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){partial=true;directories=Math.max(directories,ports.now());recency=Math.max(recency,ports.now());}}continue;}
     let info;try{info=await lstat(full);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){partial=true;recency=Math.max(recency,ports.now());}continue;}
     if(info.isSymbolicLink())continue;
     if(info.nlink>1){const exact=Number.isSafeInteger(info.ino)?info:await lstat(full,{bigint:true}).catch(()=>info),key=exact.dev+":"+exact.ino;if(seen.has(key))continue;seen.add(key);}
     bytes+=allocated(info);files++;recency=Math.max(recency,cacheRecencyMsV1(info,ports.now()));
     if(child.name.endsWith(".sqlite")&&sqlite.length<runtime.sqliteProbesPerUnit)sqlite.push(full);
    }
    return false;
   });
   if(stop){stack.length=0;break;}
  }
 }
'''),
# probe
(''' for(const file of measure.sqlite){ports.signal.throwIfAborted();if(!await ports.sqliteFree(file))return "sqlite:"+file;}
 for(const queue of measure.queues){
  ports.signal.throwIfAborted();let names:string[];try{names=await readdir(queue);}catch{continue;}''',''' for(const file of measure.sqlite){ports.signal.throwIfAborted();const reader=cacheImageReaderOfV1(ports.images,file);if(!await reader.run(()=>ports.sqliteFree(file)))return "sqlite:"+file;}
 for(const queue of measure.queues){
  ports.signal.throwIfAborted();const reader=cacheImageReaderOfV1(ports.images,queue);let names:string[];try{names=await reader.run(()=>readdir(queue));}catch{continue;}'''),
# exists
('const exists=async(path:string):Promise<boolean>=>lstat(path).then(()=>true,()=>false);','const exists=(path:string,reader:CacheImageReaderV1):Promise<boolean>=>reader.run(()=>lstat(path).then(()=>true,()=>false));'),
# partition state
(''' let outer;try{outer=await lstat(directory);}catch{return{state:"absent"''',''' const reader=cacheImageReaderOfV1(ports.images,directory);
 let outer;try{outer=await reader.run(()=>lstat(directory));}catch{return{state:"absent"'''),
('const inner=join(directory,mount);let child;try{child=await lstat(inner);}catch{child=undefined;}','const inner=join(directory,mount);let child;try{child=await reader.run(()=>lstat(inner));}catch{child=undefined;}'),
('if(await exists(join(directory,backing)))return{state:"detached"','if(await exists(join(directory,backing),reader))return{state:"detached"'),
# fleet
(' let original;\n try{original=await lstat(directory,{bigint:true});}catch',' const reader=cacheImageReaderOfV1(ports.images,directory);\n let original;\n try{original=await reader.run(()=>lstat(directory,{bigint:true}));}catch'),
('const measure=await measureCacheTreeV1(directory,ports,runtime.statusEntriesPerTarget,id),current=await lstat(directory,{bigint:true});','const measure=await measureCacheTreeV1(directory,ports,runtime.statusEntriesPerTarget,id),current=await reader.run(()=>lstat(directory,{bigint:true}));'),
# report: status without deep never leases
('''/** 📈️ Reads the ledger's status from the disk without taking a lease and, by default, without walking any tree: partitions come from native filesystem measurements of their mounted image, every other size from the size ledger that collections and deep status write, existence from one lstat or readdir. `deep` walks everything with progress and cancellation, then records what it measured. */
export async function reportCacheLedgerV1(workspace:string,ledger:CacheLedgerV1,ports:CacheRunnerPortsV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions,only?:ReadonlySet<string>,options:Readonly<{deep:boolean;measureFleetForCollection?:boolean}>={deep:false}):Promise<CacheReportV1>{
 const deep=options.deep,''','''/** 📈️ Reads the ledger's status from the disk and, by default, without walking any tree and without taking any lease (it must never wait behind a writer of an image): partitions come from native filesystem measurements of their mounted image, every other size from the size ledger that collections and deep status write, existence from one lstat or readdir. `deep` walks everything under the shared lease of each image, with progress and cancellation, then records what it measured. */
export async function reportCacheLedgerV1(workspace:string,ledger:CacheLedgerV1,given:CacheRunnerPortsV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions,only?:ReadonlySet<string>,options:Readonly<{deep:boolean;measureFleetForCollection?:boolean}>={deep:false}):Promise<CacheReportV1>{
 const{images:_images,...quick}=given,ports:CacheRunnerPortsV1=options.deep?given:quick;
 const deep=options.deep,'''),
('for(const path of paths)if(await exists(path)){present=true;break;}','for(const path of paths)if(await exists(path,cacheImageReaderOfV1(ports.images,path))){present=true;break;}'),
('const matching=async(root:string,pattern:string):Promise<string[]>=>{const matcher=new RegExp(pattern,"u");try{return(await readdir(root)).filter(','const matching=async(root:string,pattern:string):Promise<string[]>=>{const matcher=new RegExp(pattern,"u"),reader=cacheImageReaderOfV1(ports.images,root);try{return(await reader.run(()=>readdir(root))).filter('),
]
patch(R,pairs)
print("part1 ok")
pairs=[
# removal
('''/** 🗑️ Removes one tree in cancellable batches without ever following a link; a link is unlinked, never entered. */
export async function removeCacheTreeV1(path:string,ports:CacheRunnerPortsV1,stage:string):Promise<void>{
 let info;try{info=await lstat(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
 if(!info.isDirectory()){await unlinkPath(path);return;}''','''/** 🗑️ Removes one tree in cancellable batches without ever following a link; a link is unlinked, never entered. Each directory listing, each batch of `removeBatch` files and each emptied directory is one short step under the shared lease of the image: removing a file frees space and never changes the geometry of the image, so the exclusive custody that retires, regrows and detaches an image is not needed, and a reader that holds a removed file keeps its descriptor. */
export async function removeCacheTreeV1(path:string,ports:CacheRunnerPortsV1,stage:string):Promise<void>{
 const reader=cacheImageReaderOfV1(ports.images,path);
 let info;try{info=await reader.run(()=>lstat(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
 if(!info.isDirectory()){await reader.run(()=>unlinkPath(path));return;}'''),
('''  if(top.visited){stack.pop();await rmdir(top.path).catch(error=>{if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;});continue;}
  top.visited=true;const files:string[]=[];
  for(const child of await readdir(top.path,{withFileTypes:true}).catch(error=>{if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}))child.isDirectory()?stack.push({path:join(top.path,child.name),visited:false}):files.push(join(top.path,child.name));
  for(let index=0;index<files.length;index+=runtime.removeBatch){await tick();await Promise.all(files.slice(index,index+runtime.removeBatch).map(unlinkPath));}''','''  if(top.visited){stack.pop();await reader.run(()=>rmdir(top.path).catch(error=>{if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}));continue;}
  top.visited=true;const files:string[]=[];
  for(const child of await reader.run(()=>readdir(top.path,{withFileTypes:true}).catch(error=>{if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;})))child.isDirectory()?stack.push({path:join(top.path,child.name),visited:false}):files.push(join(top.path,child.name));
  for(let index=0;index<files.length;index+=runtime.removeBatch){await tick();await reader.run(()=>Promise.all(files.slice(index,index+runtime.removeBatch).map(unlinkPath)));}'''),
# unit names
('''async function unitNamesV1(base:string,depth:1|2):Promise<string[]>{
 const children=await readdir(base).catch(()=>[] as string[]);
 if(depth===1)return children;
 const names:string[]=[];
 for(const child of children){const grandchildren=await readdir(join(base,child)).catch(()=>null);''','''async function unitNamesV1(base:string,depth:1|2,ports:CacheRunnerPortsV1):Promise<string[]>{
 const reader=cacheImageReaderOfV1(ports.images,base),children=await reader.run(()=>readdir(base).catch(()=>[] as string[]));
 if(depth===1)return children;
 const names:string[]=[];
 for(const child of children){ports.signal.throwIfAborted();const grandchildren=await reader.run(()=>readdir(join(base,child)).catch(()=>null));'''),
# collect: area loop
('''   const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions);let names:string[]=[],base:string;
   const matcher=area.collect.units===undefined?undefined:new RegExp(area.collect.units,"u");
   if(directory!==null){base=directory;names=(await unitNamesV1(directory,area.collect.depth??1)).filter(name=>matcher===undefined||matcher.test(name));}
   else{base=cacheRootDirectoryV1(workspace,ledger,area.root);const matcher=new RegExp(area.pattern!,"u");try{names=(await readdir(base)).filter(name=>matcher.test(name));}catch{names=[];}}
   const lease=area.collect.custody==="lease"?await ports.exclusive(area.collect.lease!):undefined,heldBy=area.collect.custody==="lease"&&!lease?area.collect.lease!:undefined;
   const units=heldBy?[]:await observe(base,names,area.id,area.collect.custody,ports,matcher);
   resolved.push(''','''   const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions);let names:string[]=[],base:string,lease:Readonly<{release():void}>|undefined,heldBy:string|undefined,units:CacheUnitV1[];
   const matcher=area.collect.units===undefined?undefined:new RegExp(area.collect.units,"u");
   try{
    if(directory!==null){base=directory;names=(await unitNamesV1(directory,area.collect.depth??1,ports)).filter(name=>matcher===undefined||matcher.test(name));}
    else{base=cacheRootDirectoryV1(workspace,ledger,area.root);const matcher=new RegExp(area.pattern!,"u"),reader=cacheImageReaderOfV1(ports.images,base);try{names=(await reader.run(()=>readdir(base))).filter(name=>matcher.test(name));}catch(error){if(cacheLeaseRefusedV1(error))throw error;names=[];}}
    lease=area.collect.custody==="lease"?await ports.exclusive(area.collect.lease!):undefined;heldBy=area.collect.custody==="lease"&&!lease?area.collect.lease!:undefined;
    units=heldBy?[]:await observe(base,names,area.id,area.collect.custody,ports,matcher);
   }catch(error){
    lease?.release();
    if(ports.signal.aborted||!cacheLeaseRefusedV1(error))throw error;
    notes.push(text("gc.held",{path:area.id,holder:"image:"+(area.partition??area.id)}));continue;
   }
   resolved.push('''),
# retired rmdir
('if(!request.dryRun&&row.target.kind==="retired"&&plan.removals.length===row.target.units.length&&!failed.some(item=>item.target===plan.id))await rmdir(row.base).catch(()=>{});','if(!request.dryRun&&row.target.kind==="retired"&&plan.removals.length===row.target.units.length&&!failed.some(item=>item.target===plan.id)){const reader=cacheImageReaderOfV1(ports.images,row.base);await reader.run(()=>rmdir(row.base).catch(()=>{}));}'),
# atime
('if(!await(ports.atimeReliable??cacheAtimeReliableV1)(row.base)){','if(!await cacheImageReaderOfV1(ports.images,row.base).run(()=>(ports.atimeReliable??cacheAtimeReliableV1)(row.base))){'),
# removal measure
('''    const again=await measureCacheTreeV1(path,ports,runtime.collectEntriesPerUnit,plan.id),unit=row.target.units.find(candidate=>candidate.name===removal.name)!;
    if(!again.present)continue;
    const held=await probeCacheHeldV1(again,ports)??(row.custody==="pid"?cachePidHolderV1(removal.name,row.units,ports):undefined);
''','''    const unit=row.target.units.find(candidate=>candidate.name===removal.name)!;let again:CacheMeasureV1,held:string|undefined;
    try{
     again=await measureCacheTreeV1(path,ports,runtime.collectEntriesPerUnit,plan.id);
     if(!again.present)continue;
     held=await probeCacheHeldV1(again,ports)??(row.custody==="pid"?cachePidHolderV1(removal.name,row.units,ports):undefined);
    }catch(error){if(ports.signal.aborted||!cacheLeaseRefusedV1(error))throw error;failed.push({target:plan.id,name:removal.name,message:String((error as Error).message??error)});continue;}
'''),
# settle before delegates
('  for(const row of deferred){\n   const answer=ports.delegate?','  await ports.images?.settle();\n  for(const row of deferred){\n   const answer=ports.delegate?'),
]
patch(R,pairs)
print("part2 ok")
