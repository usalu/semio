#!/usr/bin/env python3
"""q2: replica: blob wants, offers and the repair. usage: q2-blobs.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:70],s.count(old))
    return s.replace(old,new)
p=root+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
s=sub(s,'import {appendEventsV1,blobPathV1,','import {appendEventsV1,repairBlobV1,blobPathV1,')
s=sub(s,'import schema from "./🧬️schema/🔣️.json" with {type:"json"};','import {readOthersWantsV1,wantsLimitV1,writeWantsV1} from "./🩹️blobs/🟦️.ts";\nimport schema from "./🧬️schema/🔣️.json" with {type:"json"};')
s=sub(s,'|"drift"|"error";stage?:string','|"drift"|"blob-repaired"|"error";stage?:string')
s=sub(s,'''  noteOnce(replica,`blob-missing:${path}:${entry.sha256}`,{event:"blob-missing",path,hlc:entry.hlc,reason:entry.sha256});''','''  wantBlob(replica,path,entry.sha256!);
  noteOnce(replica,`blob-wanted:${path}`,{event:"blob-missing",path,hlc:entry.hlc,reason:entry.sha256});''')
s=sub(s,''' if(verdict==="corrupt"){noteOnce(replica,`corrupt:${path}:${entry.sha256}`,{event:"blob-missing",path,hlc:entry.hlc,reason:"blob-corrupt"});return "queued";}''',''' if(verdict==="corrupt"){wantBlob(replica,path,entry.sha256!);noteOnce(replica,`blob-wanted:${path}`,{event:"blob-missing",path,hlc:entry.hlc,reason:"blob-corrupt"});return "queued";}''')
s=sub(s,''' setFile(replica,path,{...plain(entry),mtimeMs:stamp,seenMs:stamp%1000===0?seenMs:Math.max(seenMs,stamp+racyMilliseconds)});
 return "applied";
}''',''' setFile(replica,path,{...plain(entry),mtimeMs:stamp,seenMs:stamp%1000===0?seenMs:Math.max(seenMs,stamp+racyMilliseconds)});
 releaseBlob(replica,path,entry.sha256!);
 return "applied";
}

const wanted=new WeakMap<ReplicaV1,Map<string,string>>(),announced=new WeakMap<ReplicaV1,string>(),offered=new WeakMap<ReplicaV1,Map<string,number>>(),offerPauseMilliseconds=300_000;

function wantBlob(replica:ReplicaV1,path:string,sha256:string):void{
 let held=wanted.get(replica);
 if(held===undefined){held=new Map();wanted.set(replica,held);}
 held.set(sha256,path);
}

function releaseBlob(replica:ReplicaV1,path:string,sha256:string):void{
 if(wanted.get(replica)?.delete(sha256)===true)resolved(replica,`blob-wanted:${path}`);
}

/** 🔍️ The applied, unmodified-looking file of this machine that the manifest entry for a content hash describes, or `undefined` when this machine holds no live copy of it. */
function holderOf(replica:ReplicaV1,sha256:string):Readonly<{file:string;path:string}>|undefined{
 const {manifest,files,queue,book,settings}=replica;
 for(const id of manifest.ids()){
  if(manifest.operationAt(id)!=="put"||!manifest.shaIs(id,sha256)||!files.live(id)||files.operationAt(id)!=="put"||!files.sameHlcAs(manifest,id)||queue.hasId(id)||indexRealmV1(book.name(id)))continue;
  return {file:absolute(settings.worktree,book.name(id)),path:book.name(id)};
 }
 return undefined;
}

/** 🫗️ Looks after unreadable blobs: announces the blobs this machine cannot read (missing, or not hashing to their name; at most 64) in `origins/<origin>/wants.json` of the store whenever they change, and puts back every blob that another origin announces and that this machine still holds as a live, applied file whose bytes hash to the blob name (the store copy is replaced through a staged copy and a rename; a holder whose file changed writes nothing). A hash is paid once per blob and five minutes. Returns how many wants this machine announces and how many blobs it repaired. */
export async function tendBlobsV1(replica:ReplicaV1,signal?:AbortSignal):Promise<Readonly<{announced:number;repaired:number}>>{
 const {storeRoot,originId}=replica.settings,mine=[...(wanted.get(replica)??new Map<string,string>())].sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0).slice(0,wantsLimitV1).map(([sha256,path])=>({sha256,path})),signature=JSON.stringify(mine);
 if(signature!==(announced.get(replica)??"[]")){
  await writeWantsV1(storeRoot,{version:1,originId,wallMs:replica.settings.now(),wants:mine});
  announced.set(replica,signature);
 }
 let repaired=0,pauses=offered.get(replica);
 if(pauses===undefined){pauses=new Map();offered.set(replica,pauses);}
 for(const record of await readOthersWantsV1(storeRoot,originId)){
  for(const want of record.wants){
   signal?.throwIfAborted();
   const now=replica.settings.now(),until=pauses.get(want.sha256);
   if(until!==undefined&&now<until)continue;
   const source=holderOf(replica,want.sha256);
   if(source===undefined)continue;
   if(pauses.size>=4096)pauses.clear();
   pauses.set(want.sha256,now+offerPauseMilliseconds);
   const outcome=await repairBlobV1(storeRoot,originId,want.sha256,source.file).catch(()=>"unstable" as const);
   if(outcome==="repaired"){repaired++;replica.say({event:"blob-repaired",path:source.path,reason:want.sha256});}
  }
 }
 return {announced:mine.length,repaired};
}''')
s=sub(s,'''if(outcome==="queued"&&replica.noted.has(`blocked:${path}`)){''','''if(outcome==="queued"&&(replica.noted.has(`blocked:${path}`)||replica.noted.has(`blob-wanted:${path}`))){''')
s=sub(s,'''healthAt=-Infinity,maintainedAt=-Infinity;''','''healthAt=-Infinity,maintainedAt=-Infinity,tendedAt=-Infinity;''')
s=sub(s,'''    if(wall-maintainedAt>=maintenanceMilliseconds){''','''    if(wall-tendedAt>=tendMilliseconds&&syncOn&&!parked){tendedAt=wall;await tendBlobsV1(replica,signal).catch(()=>{});}
    if(wall-maintainedAt>=maintenanceMilliseconds){''')
s=sub(s,'healthMilliseconds=5000,','healthMilliseconds=5000,tendMilliseconds=20_000,')
s=sub(s,'''async function relayAttestation(ownStore:string,peerStore:string,origin:string):Promise<void>{
 await relayOriginFile(ownStore,peerStore,origin,"view.txt");''','''async function relayAttestation(ownStore:string,peerStore:string,origin:string):Promise<void>{
 await relayOriginFile(ownStore,peerStore,origin,"wants.json");
 await relayOriginFile(ownStore,peerStore,origin,"view.txt");''')
open(p,"w",encoding="utf8").write(s)

p=root+"/🔁️replica/🧬️schema/🔣️.json"
s=open(p,encoding="utf8").read()
s=sub(s,'"stale-republish", "drift", "error"]','"stale-republish", "drift", "blob-repaired", "error"]')
open(p,"w",encoding="utf8").write(s)
print("ok")
