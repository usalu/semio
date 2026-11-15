import sys
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🩺️attest/🟦️.ts",[
 ('/** 🌐️ Reads the attestations, retirement records and heads of a store and judges the fleet at `now`. */','''/** 🪢 The fields in which `originId` disagrees with the rest of the settled fleet and which a repair of its own state can mend: the manifest. Empty while the origin agrees, is not settled, or differs only in code, git state or view. */
export function repairableDriftV1(report:FleetReportV1,originId:string):readonly FleetFieldV1[]{
 const row=report.rows.find(item=>item.originId===originId);
 return row?.verdict==="diverged"?row.differs.filter(field=>field==="manifest"):[];
}

/** 🌐️ Reads the attestations, retirement records and heads of a store and judges the fleet at `now`. */'''),
])
edit(w+"/🔁️replica/🟦️.ts",[
 ('type AttesterGitV1,type AttesterViewV1,type FleetFallbackV1,','repairableDriftV1,type AttesterGitV1,type AttesterViewV1,type FleetFallbackV1,'),
 ('"stale-republish"|"error";stage?:string;','"stale-republish"|"drift"|"error";stage?:string;'),
 ('returnMilliseconds?:number}>;','returnMilliseconds?:number;driftMilliseconds?:number;opened?:(replica:ReplicaV1)=>void}>;'),
 ('''export async function takeOverRegistersV1(replica:ReplicaV1,signal?:AbortSignal):Promise<number>{
 const {manifest,queue,book,settings}=replica;
 let count=0;
 for(const [path,entry] of (await foldStoreV1(settings.storeRoot,signal)).manifest.entries()){
  const known=manifest.get(path);
  if(known!==undefined&&!winsV1(entry.hlc,known.hlc))continue;
  manifest.set(path,plain(entry));
  const id=book.intern(path);
  if(!queue.hasId(id)){enqueueId(replica,id);count++;}
 }
 return count;
}''','''export async function takeOverRegistersV1(replica:ReplicaV1,signal?:AbortSignal):Promise<number>{
 const {manifest,queue,book,settings}=replica;
 let count=0,complete=true;
 const folded=await foldStoreV1(settings.storeRoot,signal,event=>{if(event.event==="rejoin-required"||event.event==="clock-skew")complete=false;});
 for(const [path,entry] of folded.manifest.entries()){
  const known=manifest.get(path);
  if(known!==undefined&&!winsV1(entry.hlc,known.hlc))continue;
  manifest.set(path,plain(entry));
  const id=book.intern(path);
  if(!queue.hasId(id)){enqueueId(replica,id);count++;}
 }
 if(complete)for(const id of manifest.ids()){
  const path=book.name(id);
  if(folded.manifest.get(path)!==undefined||indexRealmV1(path)&&manifest.operationAt(id)==="delete")continue;
  manifest.clearId(id);replica.changed.add(filesKey(id));replica.dirty=true;
  if(replica.files.live(id)&&!queue.hasId(id))enqueueId(replica,id);
  count++;
 }
 return count;
}'''),
 (' const switchOn=cached(slow,',''' const baseDrift=options.driftMilliseconds??120_000,drift={at:settings.now()+baseDrift,wait:baseDrift,attempts:0};
 const drifted=async(signal:AbortSignal):Promise<readonly string[]>=>{
  const now=settings.now();
  if(now<drift.at)return [];
  const fields=repairableDriftV1(await bounded(()=>fleetReportV1(settings.storeRoot,now,signal)),settings.originId);
  drift.attempts=fields.length===0?0:drift.attempts+1;
  drift.wait=fields.length===0?baseDrift:Math.min(baseDrift*2**drift.attempts,3_600_000);
  drift.at=now+drift.wait;
  return fields;
 };
 const switchOn=cached(slow,'''),
])
