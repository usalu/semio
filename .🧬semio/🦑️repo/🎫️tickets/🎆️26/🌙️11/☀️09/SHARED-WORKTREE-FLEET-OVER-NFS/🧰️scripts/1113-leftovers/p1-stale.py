import sys,re
w=sys.argv[1]
p=w+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(s.count(old),old[:80])
    s=s.replace(old,new)
sub('import {clashKeyV1,clashV1} from "./⚖️clash/🟦️.ts";','import {clashKeyV1,clashV1,staleRepublishV1,type ContentEventV1} from "./⚖️clash/🟦️.ts";')
i=s.index("const histories=new WeakMap")
j=s.index("async function adoptSnapshot")
new='''const histories=new WeakMap<ReplicaV1,Map<number,ContentEventV1[]>>(),stales=new WeakMap<ReplicaV1,Map<number,string>>(),savedHistory=new WeakMap<ReplicaV1,{dirty:boolean;at:number}>(),historyDepth=6,historyPaths=20_000,historySaved=1000,historySaveMilliseconds=30_000,historyMemo="stale.history";

const shortSha=(value:string|null|undefined):string|null=>value==null?null:value.slice(0,32);
const contentOf=(event:FleetEventV1):string|null=>event.operation==="put"?shortSha(event.sha256):event.operation==="link"?`link ${event.target}`:null;

/** 🕰️ Remembers one content event of a path for the stale rule: its hlc, the content it leaves and its `base`, the newest `historyDepth` events of each of the newest `historyPaths` paths. A path without history is seeded with the manifest entry it holds, so the events that follow have a predecessor. The memory is a cache of journal events and is kept with the local state (`stale.history`); a replica that lost it decides from the events it still reads. */
function track(replica:ReplicaV1,event:FleetEventV1):void{
 if(sealed(event.path)||event.path.startsWith(".git/"))return;
 let held=histories.get(replica);
 if(held===undefined){held=new Map();histories.set(replica,held);}
 const id=replica.book.intern(event.path),list=held.get(id)??[];
 if(list.length===0&&replica.manifest.live(id)){
  const prior=replica.manifest.operationAt(id)!,hlc=replica.manifest.hlcAt(id);
  if(prior!=="delete"&&hlc!==event.hlc)list.push({hlc,sha256:prior==="put"?shortSha(replica.manifest.shaAt(id)):`link ${replica.manifest.targets.get(id)}`,base:null});
 }
 if(list.some(item=>item.hlc===event.hlc))return;
 list.push({hlc:event.hlc,sha256:contentOf(event),base:shortSha(event.base)});
 list.sort((left,right)=>left.hlc<right.hlc?-1:left.hlc>right.hlc?1:0);
 if(list.length>historyDepth)list.splice(0,list.length-historyDepth);
 held.delete(id);held.set(id,list);
 if(held.size>historyPaths)held.delete(held.keys().next().value!);
 savedHistory.set(replica,{dirty:true,at:savedHistory.get(replica)?.at??0});
}

/** 🪢 Writes the stale-rule memory into the local state memo, at most every 30 s unless `force`: the newest 1000 paths. */
function keepHistory(replica:ReplicaV1,force:boolean):void {
 const saved=savedHistory.get(replica),held=histories.get(replica);
 if(saved===undefined||!saved.dirty||held===undefined)return;
 const now=replica.settings.now();
 if(!force&&now-saved.at<historySaveMilliseconds)return;
 const rows:[string,ContentEventV1[]][]=[];
 for(const [id,list] of [...held].slice(-historySaved))rows.push([replica.book.name(id),list]);
 rememberV1(replica,historyMemo,encoder.encode(JSON.stringify({version:1,paths:rows})));
 savedHistory.set(replica,{dirty:false,at:now});
}

function restoreHistory(replica:ReplicaV1):void {
 const raw=replica.memo.get(historyMemo);
 if(raw===undefined)return;
 try{
  const value=JSON.parse(Buffer.from(raw).toString("utf8")) as {version:number;paths:[string,ContentEventV1[]][]};
  if(value.version!==1||!Array.isArray(value.paths))return;
  const held=new Map<number,ContentEventV1[]>();
  for(const [path,list] of value.paths)if(typeof path==="string"&&Array.isArray(list)&&path.isWellFormed())held.set(replica.book.intern(path),list.filter(item=>typeof item?.hlc==="string"&&(item.sha256===null||typeof item.sha256==="string")&&(item.base===null||typeof item.base==="string")).slice(-historyDepth));
  histories.set(replica,held);savedHistory.set(replica,{dirty:false,at:replica.settings.now()});
 }catch{}
}

function ingest(replica:ReplicaV1,origin:string,events:readonly FleetEventV1[]):void{
 const wall=replica.settings.now(),accepted:FleetEventV1[]=[];
 for(const event of events){
  try{replica.clock=mergeClockV1(replica.clock,event.hlc,wall);accepted.push(event);}
  catch{replica.say({event:"clock-refused",origin,hlc:event.hlc,path:event.path});}
 }
 for(const event of accepted)track(replica,event);
 for(const event of foldEventsV1(replica.manifest,accepted)){
  const id=replica.book.intern(event.path);
  stales.get(replica)?.delete(id);
  const stale=event.operation==="put"&&!event.path.startsWith(".git/")?staleRepublishV1(histories.get(replica)?.get(id)??[],event.hlc):undefined;
  if(stale?.winner!=null){
   let marks=stales.get(replica);
   if(marks===undefined){marks=new Map();stales.set(replica,marks);}
   marks.set(id,stale.winner);
  }
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 replica.dirty=true;
}

'''
s=s[:i]+new+s[j:]
# publish side
sub("   if(fresh){const id=replica.book.intern(path);if(replica.manifest.live(id)&&replica.manifest.operationAt(id)===\"put\"&&(state.operation!==\"put\"||!replica.manifest.shaIs(id,state.sha256!)))remember(replica,id,replica.manifest.shaAt(id));}\n","")
sub("""    const head=await appendEventsV1(storeRoot,originId,events,journalRepair(replica));
    replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
    published+=events.length;""","""    const head=await appendEventsV1(storeRoot,originId,events,journalRepair(replica));
    replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
    for(const event of events)track(replica,event);
    published+=events.length;""")
sub("""  replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
  for(const event of events){setFile(replica,event.path,{hlc:event.hlc,operation:"delete",mtimeMs:0});""","""  replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
  for(const event of events)track(replica,event);
  for(const event of events){setFile(replica,event.path,{hlc:event.hlc,operation:"delete",mtimeMs:0});""")
# materialise mark compare
sub("known?.operation===\"put\"&&known.sha256===mark){","known?.operation===\"put\"&&shortSha(known.sha256)===mark){")
sub("replica.say({event:\"stale-republish\",path,hlc:entry.hlc,winner:mark,loser:entry.sha256,reason:\"content-superseded\"});","replica.say({event:\"stale-republish\",path,hlc:entry.hlc,winner:known.sha256,loser:entry.sha256,reason:\"content-superseded\"});")
# keepState / persist
sub("  keepState(replica);\n  if(roll)await checkpointReplicaV1(replica);","  keepState(replica);keepHistory(replica,force);\n  if(roll)await checkpointReplicaV1(replica);")
sub("""  if(hot.length>0||replica.memo.has("hot"))rememberV1(replica,"hot",hot.length>0?encoder.encode(hot.join("\\0")):undefined);
  await persist(replica,true);""","""  if(hot.length>0||replica.memo.has("hot"))rememberV1(replica,"hot",hot.length>0?encoder.encode(hot.join("\\0")):undefined);
  keepHistory(replica,true);
  await persist(replica,true);""")
sub("""  const hot=local.memo.get("hot");
  if(hot!==undefined)""","""  restoreHistory(replica);
  const hot=local.memo.get("hot");
  if(hot!==undefined)""")
open(p,"w",encoding="utf8").write(s)
