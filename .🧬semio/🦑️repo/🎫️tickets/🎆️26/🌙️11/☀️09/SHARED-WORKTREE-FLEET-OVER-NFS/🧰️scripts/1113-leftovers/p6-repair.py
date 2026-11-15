import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🔁️replica/🟦️.ts",[
 ('''  catch(error){if(!gone(error))throw error;forget(replica,path);enqueueId(replica,id);count++;}
 }
 await rm(repairRequest(settings.worktree),{force:true});''','''  catch(error){if(!gone(error))throw error;forget(replica,path);enqueueId(replica,id);count++;}
 }
 count+=disagreeing(replica);
 await rm(repairRequest(settings.worktree),{force:true});'''),
 ('export function enqueueId(replica:ReplicaV1,id:number):void{','''/** 🪢 Brings the applied state back in line with the manifest without touching the disk: a manifest entry whose applied row is missing or carries another hlc is queued (the follow then applies or drops it), an applied row without a manifest entry is forgotten (the next walk publishes the file if it still exists). Returns how many paths it touched. */
function disagreeing(replica:ReplicaV1):number{
 const {manifest,files,queue,book}=replica;
 let count=0;
 for(const id of manifest.ids()){
  if(queue.hasId(id)||indexRealmV1(book.name(id)))continue;
  if(!files.live(id)||!files.sameHlcAs(manifest,id)){enqueueId(replica,id);count++;}
 }
 for(const id of files.ids()){
  if(files.operationAt(id)==="delete"||manifest.live(id)||indexRealmV1(book.name(id)))continue;
  forget(replica,book.name(id));count++;
 }
 return count;
}

export function enqueueId(replica:ReplicaV1,id:number):void{'''),
])
edit(w+"/🩺️attest/🧪️tests/🟦️.ts",[
 ('''  const a2=runners[1]!,id=victim!.book.find("a.txt")!,real=victim!.manifest.get("a.txt")!;
  victim!.manifest.write(id,{...real,hlc:"0000000000001-000000-g1-seed",sha256:"e".repeat(64)});
  victim!.manifest.set("phantom.txt",{hlc:"0000000000002-000000-g1-seed",operation:"put",sha256:"f".repeat(64),bytes:1,executable:false});''','''  const a2=runners[1]!,id=victim!.book.find("a.txt")!,real=victim!.files.get("a.txt")!;
  victim!.files.write(id,{...real,hlc:"0000000000001-000000-g1-seed",sha256:"e".repeat(64)});
  victim!.files.set("phantom.txt",{hlc:"0000000000002-000000-g1-seed",operation:"put",sha256:"f".repeat(64),bytes:1,executable:false});'''),
 ('''  expect(victim!.manifest.get("a.txt")).toEqual(real);
  expect(victim!.manifest.get("phantom.txt")).toBeUndefined();''','''  expect(victim!.files.get("a.txt")).toMatchObject({hlc:real.hlc,sha256:real.sha256});
  expect(victim!.files.get("phantom.txt")).toBeUndefined();'''),
])
