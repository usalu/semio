import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('const validators=new Map<string,CompiledJsonSchemaSubset>(),resolvedStores','const verifiedBlobs=new Set<string>(),validators=new Map<string,CompiledJsonSchemaSubset>(),resolvedStores')
a=s.index("/** 📥️ Stores a file as an immutable blob")
# intact helper before storeBlobV1 docstring: find the docstring line preceding "export async function storeBlobV1"
sb=s.index("export async function storeBlobV1")
doc=s.rindex("/**",0,sb)
helper='''async function intact(target:string,sha256:string):Promise<boolean>{
 if(verifiedBlobs.has(target))return true;
 const hash=createHash("sha256");
 try{for await(const chunk of createReadStream(target))hash.update(chunk);}catch(error){if(missing(error))return false;throw error;}
 if(hash.digest("hex")!==sha256)return false;
 if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();
 verifiedBlobs.add(target);
 return true;
}

'''
s=s[:doc]+helper+s[doc:]
rep("  if(await exists(target)){const now=new Date();await utimes(target,now,now).catch(()=>{});return {...known,stable:true,created:false};}","  if(await exists(target)&&await intact(target,known.sha256)){const now=new Date();await utimes(target,now,now).catch(()=>{});return {...known,stable:true,created:false};}")
rep("  if(present??await exists(target)){await unlink(temporary).catch(()=>{});","  if((present??await exists(target))&&await intact(target,copied.sha256)){await unlink(temporary).catch(()=>{});")
rep("  return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:true};\n }catch(error){await unlink(temporary).catch(()=>{});throw error;}\n}","  verifiedBlobs.add(target);\n  return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:true};\n }catch(error){await unlink(temporary).catch(()=>{});throw error;}\n}")
rep("/** 📥️ Stores a file as an immutable blob whose hash the caller already knows, verified against the bytes of the single read; `false` means it was already there,","/** 📥️ Stores a file as an immutable blob whose hash the caller already knows, verified against the bytes of the single read; an existing blob is rehashed (once per process) before it is trusted and one that does not hash to its name is replaced by the new copy; `false` means it was already there and intact,")
rep('''/** ✍️ Appends event lines to the origin's segments (fsync), sealing full segments, then publishes the new head through tmp + rename. */
export async function appendEventsV1(storeRoot:string,originId:string,events:readonly FleetEventV1[]):Promise<FleetHeadV1>{''','''/** 🩹️ What a repair of a segment that was shorter than its head reports: the segment that was sealed short, its real size in bytes and how many complete lines it holds. */
export type FleetJournalRepairV1=Readonly<{segment:number;bytes:number;found:number}>;

/** ✍️ Appends event lines to the origin's segments (fsync), sealing full segments, then publishes the new head through tmp + rename. A segment that is shorter than its head (a store host that crashed before the data reached the disk, a removed file) is sealed at the size it has and the append continues in the next segment: peers read a sealed segment to its end and then the next one from its start, so no cursor ends up in front of new events; `onRepair` is told. */
export async function appendEventsV1(storeRoot:string,originId:string,events:readonly FleetEventV1[],onRepair?:(repair:FleetJournalRepairV1)=>void):Promise<FleetHeadV1>{''')
rep('''   const current=(await handle.stat()).size;
   if(current<bytes)throw Error(`Fleet segment is shorter than its head: ${file}`);
   if(current>bytes)await handle.truncate(bytes);''','''   const current=(await handle.stat()).size;
   if(current<bytes){
    const kept=await readRange(file,0,current),complete=kept.lastIndexOf(10)+1;
    if(complete<current)await handle.truncate(complete);
    onRepair?.({segment,bytes:complete,found:kept.subarray(0,complete).toString("utf8").split("\\n").length-1});
    segment++;bytes=0;count=0;
    continue;
   }
   if(current>bytes)await handle.truncate(bytes);''')
rep('''    try{events.push(requireEventV1(JSON.parse(line)));}
    catch(error){throw Error(`Fleet segment ${segmentName(segment)} of ${originId} is corrupt at byte ${from+local}: ${(error as Error).message}`);}''','''    try{events.push(requireEventV1(JSON.parse(line)));}
    catch(error){
     const version=(()=>{try{return (JSON.parse(line) as {version?:unknown}).version;}catch{return 1;}})();
     if(version!==1&&version!==undefined)throw Error(`Fleet segment ${segmentName(segment)} of ${originId} holds an unsupported event version ${String(version)} at byte ${from+local}: this daemon cannot read it and waits for newer code`);
     throw Error(`Fleet segment ${segmentName(segment)} of ${originId} is corrupt at byte ${from+local}: ${(error as Error).message}`);
    }''')
rep("""       await pending;
       pending=writeFully(output,buffer,bytesRead);
       flip^=1;bytes+=bytesRead;progress?.(bytes,before.size);
      }
     }finally{await pending.catch(()=>{});}""","""       await pending;
       pending=writeFully(output,buffer,bytesRead);pending.catch(()=>{});
       flip^=1;bytes+=bytesRead;progress?.(bytes,before.size);
      }
      await pending;
     }catch(error){await pending.catch(()=>{});throw error;}""")
open(p,"w",encoding="utf8").write(s)
