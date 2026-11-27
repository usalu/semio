import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:70],s.count(old));s=s.replace(old,new)
    tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
SHARED="../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts"
patch(B+"🧬️schema/🔣️.json",[
 ('        "dedupeCountProgressStride": 512,\n','        "dedupeCountProgressStride": 512,\n        "dedupeReadChunksPerHold": 8,\n        "dedupeEntriesPerHold": 256,\n'),
 ('          "dedupeCountProgressStride": "After how many counted files the deduplication checks for progress and cancellation.",\n','          "dedupeCountProgressStride": "After how many counted files the deduplication checks for progress and cancellation.",\n          "dedupeReadChunksPerHold": "How many chunks of one megabyte the deduplication reads of a file before it gives the shared lease of the image back (a longer file is read in several holds).",\n          "dedupeEntriesPerHold": "How many directory entries the deduplication inspects before it gives the shared lease of the image back.",\n'),
 ('          "dedupeCountProgressStride": "Nach wie vielen gezählten Dateien die Deduplizierung auf Fortschritt und Abbruch prüft.",\n','          "dedupeCountProgressStride": "Nach wie vielen gezählten Dateien die Deduplizierung auf Fortschritt und Abbruch prüft.",\n          "dedupeReadChunksPerHold": "Wie viele Blöcke zu einem Megabyte die Deduplizierung von einer Datei liest, bevor sie die geteilte Lease des Images zurückgibt (eine längere Datei wird in mehreren Haltephasen gelesen).",\n          "dedupeEntriesPerHold": "Wie viele Verzeichniseinträge die Deduplizierung prüft, bevor sie die geteilte Lease des Images zurückgibt.",\n'),
])
patch(B+"👯️dedupe/🟦️.ts",[
 ('import type {CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";\n','import type {CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";\nimport type {NativeImageSharedReaderV1} from "'+SHARED+'";\n\n/** 👓️ The shared lease of the image the tree lives in: every directory read, stat, hash read, clone, rename and removal of the deduplication runs inside `run`, which keeps the image attached while it works and gives it back between bursts, so a retirement, regrow or detach is never held up for longer than one step. */\nexport type DedupeReaderV1=Pick<NativeImageSharedReaderV1,"run">;\n'),
 ('/** 🎯️ One declared place where duplicates are looked for. */\nexport type DedupeRequestV1=Readonly<{base:string;minimumBytes','/** 🎯️ One declared place where duplicates are looked for, with the reader that holds the shared lease of its image. */\nexport type DedupeRequestV1=Readonly<{base:string;reader:DedupeReaderV1;minimumBytes'),
 ('''async function sha256V1(path:string,ports:CacheRunnerPortsV1):Promise<string|null>{
 let handle;try{handle=await open(path,constants.O_RDONLY|(constants.O_NOATIME??0)).catch(()=>open(path,"r"));}catch{return null;}
 try{const hash=createHash("sha256"),buffer=Buffer.allocUnsafe(1<<20);for(;;){ports.signal.throwIfAborted();const{bytesRead}=await handle.read(buffer,0,buffer.length,null);if(!bytesRead)break;hash.update(buffer.subarray(0,bytesRead));}return hash.digest("hex");}
 catch(error){if(ports.signal.aborted)throw error;return null;}
 finally{await handle.close().catch(()=>{});}
}
''','''async function sha256V1(path:string,ports:CacheRunnerPortsV1,reader:DedupeReaderV1):Promise<string|null>{
 const hash=createHash("sha256"),buffer=Buffer.allocUnsafe(1<<20);let position=0;
 for(;;){
  ports.signal.throwIfAborted();
  const burst=await reader.run(async():Promise<{bytes:number;done:boolean}|null>=>{
   let handle;try{handle=await open(path,constants.O_RDONLY|(constants.O_NOATIME??0)).catch(()=>open(path,"r"));}catch{return null;}
   try{
    let bytes=0;
    for(let chunk=0;chunk<CACHE_GC_LIMITS_V1.dedupeReadChunksPerHold;chunk++){const{bytesRead}=await handle.read(buffer,0,buffer.length,position+bytes);if(!bytesRead)return{bytes,done:true};hash.update(buffer.subarray(0,bytesRead));bytes+=bytesRead;}
    return{bytes,done:false};
   }
   catch(error){if(ports.signal.aborted)throw error;return null;}
   finally{await handle.close().catch(()=>{});}
  });
  if(burst===null)return null;
  if(burst.done)return hash.digest("hex");
  position+=burst.bytes;
 }
}
'''),
 ('async function* filesV1(base:string,ports:CacheRunnerPortsV1):AsyncGenerator<{path:string;size:number;identity:string;links:number;recencyMs:number;mtimeMs:number;mode:number;ino:number}>{\n const stack=[base];\n while(stack.length){\n  ports.signal.throwIfAborted();const directory=stack.pop()!;let children;\n  try{children=await readdir(directory,{withFileTypes:true});}catch{continue;}\n  for(const child of children){\n   const path=join(directory,child.name);\n   if(child.isDirectory()){stack.push(path);continue;}\n   if(!child.isFile())continue;\n   let info;try{info=await lstat(path,{bigint:false});}catch{continue;}\n   if(!info.isFile())continue;\n   yield{path,size:info.size,identity:info.dev+":"+info.ino,links:info.nlink,recencyMs:cacheRecencyMsV1(info,ports.now()),mtimeMs:info.mtimeMs,mode:info.mode&0o7777,ino:info.ino};\n  }\n }\n}\n',
  '''type FileRowV1={path:string;size:number;identity:string;links:number;recencyMs:number;mtimeMs:number;mode:number;ino:number};

async function* filesV1(base:string,ports:CacheRunnerPortsV1,reader:DedupeReaderV1):AsyncGenerator<FileRowV1>{
 const stack=[base];
 while(stack.length){
  ports.signal.throwIfAborted();const directory=stack.pop()!;
  const children=await reader.run(()=>readdir(directory,{withFileTypes:true}).catch(()=>undefined));
  if(children===undefined)continue;
  for(let from=0;from<children.length;from+=CACHE_GC_LIMITS_V1.dedupeEntriesPerHold){
   ports.signal.throwIfAborted();
   const rows=await reader.run(async()=>{
    const found:FileRowV1[]=[];
    for(const child of children.slice(from,from+CACHE_GC_LIMITS_V1.dedupeEntriesPerHold)){
     const path=join(directory,child.name);
     if(child.isDirectory()){stack.push(path);continue;}
     if(!child.isFile())continue;
     let info;try{info=await lstat(path,{bigint:false});}catch{continue;}
     if(!info.isFile())continue;
     found.push({path,size:info.size,identity:info.dev+":"+info.ino,links:info.nlink,recencyMs:cacheRecencyMsV1(info,ports.now()),mtimeMs:info.mtimeMs,mode:info.mode&0o7777,ino:info.ino});
    }
    return found;
   });
   yield*rows;
  }
 }
}
'''),
 ('async function volumeClonesV1(base:string,clone:(source:string,destination:string)=>Promise<boolean>):Promise<boolean>{\n const source=join(base,".dedupe-probe-"+process.pid),destination=source+".clone";\n try{await writeFile(source,"x");return await clone(source,destination);}catch{return false;}\n finally{await unlink(source).catch(()=>{});await unlink(destination).catch(()=>{});}\n}\n',
  'async function volumeClonesV1(base:string,clone:(source:string,destination:string)=>Promise<boolean>,reader:DedupeReaderV1):Promise<boolean>{\n const source=join(base,".dedupe-probe-"+process.pid),destination=source+".clone";\n return reader.run(async()=>{\n  try{await writeFile(source,"x");return await clone(source,destination);}catch{return false;}\n  finally{await unlink(source).catch(()=>{});await unlink(destination).catch(()=>{});}\n });\n}\n'),
 ('Progress and cancellation per file; memory stays bounded by `maximumFiles` candidates.\n */','Progress and cancellation per file; memory stays bounded by `maximumFiles` candidates. Every touch of the tree runs in a short step inside the request\'s reader (see `DedupeReaderV1`); a lease that cannot be taken in time ends the run with the reader\'s timeout error before anything is changed.\n */'),
 ('if(!request.dryRun&&!await volumeClonesV1(request.base,clone)){','if(!request.dryRun&&!await volumeClonesV1(request.base,clone,request.reader)){'),
 ('for await(const file of filesV1(request.base,ports)){if(file.size<request.minimumBytes)continue;','for await(const file of filesV1(request.base,ports,request.reader)){if(file.size<request.minimumBytes)continue;'),
 (' for await(const file of filesV1(request.base,ports)){\n  if(file.size<request.minimumBytes||',' for await(const file of filesV1(request.base,ports,request.reader)){\n  if(file.size<request.minimumBytes||'),
 ('hash=await sha256V1(file.path,ports);','hash=await sha256V1(file.path,ports,request.reader);'),
])
s=open(B+"👯️dedupe/🟦️.ts",encoding="utf8").read()
a=s.index("  const stamp=stamps.get(row.path)!,temporary=");b=s.index(" return Object.freeze({plan,clones:true,replaced,freedBytes:freed,skipped,failed,partial});\n}")
new='''  const stamp=stamps.get(row.path)!,temporary=join(dirname(row.path),".dedupe-"+process.pid+"-"+(counter++));
  const step=await request.reader.run(async():Promise<{kind:"replaced"}|{kind:"changed";path:string}|{kind:"unsupported"}|{kind:"failed";message:string}>=>{
   try{
    const before=await lstat(row.path);
    if(!before.isFile()||before.ino!==stamp.ino||before.mtimeMs!==stamp.mtimeMs||before.size!==stamp.size||ports.now()-before.mtimeMs<request.quietMs)return{kind:"changed",path:row.path};
    const origin=stamps.get(row.canonical)!,unchanged=async():Promise<boolean>=>{const source=await lstat(row.canonical);return source.isFile()&&source.ino===origin.ino&&source.mtimeMs===origin.mtimeMs&&source.size===origin.size&&source.size===row.bytes;};
    if(!await unchanged())return{kind:"changed",path:row.canonical};
    if(!await clone(row.canonical,temporary))return{kind:"unsupported"};
    await chmod(temporary,stamp.mode);await utimes(temporary,before.atime,before.mtime);
    const again=await lstat(row.path);
    if(again.ino!==stamp.ino||again.mtimeMs!==stamp.mtimeMs||again.size!==stamp.size||!await unchanged()){await unlink(temporary).catch(()=>{});return{kind:"changed",path:row.path};}
    await rename(temporary,row.path);
    return{kind:"replaced"};
   }catch(error){await unlink(temporary).catch(()=>{});if(ports.signal.aborted)throw error;return{kind:"failed",message:row.path+": "+String((error as Error).message??error)};}
  });
  if(step.kind==="changed"){skipped++;request.write(text("dedupe.changed",{path:step.path}));}
  else if(step.kind==="unsupported"){skipped++;request.write(text("dedupe.unsupported",{path:row.path}));}
  else if(step.kind==="failed")failed.push(step.message);
  else{
   replaced++;
   const identity=candidateOf.get(row.path)!.identity;if(!seenIdentity.has(identity)){seenIdentity.add(identity);freed+=row.bytes;}
   request.write(text("dedupe.replaced",{path:row.path,bytes:formatCacheBytesV1(row.bytes)}));
  }
 }
'''
s=s[:a]+new+s[b:]
tmp=B+"👯️dedupe/🟦️.ts.f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,B+"👯️dedupe/🟦️.ts")
print("dedupe ok")
