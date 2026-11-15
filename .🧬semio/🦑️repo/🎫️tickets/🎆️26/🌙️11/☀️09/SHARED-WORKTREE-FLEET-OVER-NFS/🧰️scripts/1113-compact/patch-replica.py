#!/usr/bin/env python3
"""Anchored edits of the replica sources for the compaction landing. Usage: patch-replica.py <module root>.
Every edit asserts that its anchor occurs exactly once; on a live tree that moved on the script fails before it writes."""
import sys, os, re

root = sys.argv[1]
changes = {}

def load(path):
    if path not in changes:
        with open(os.path.join(root, path), encoding="utf8") as handle:
            changes[path] = handle.read()
    return changes[path]

def rep(path, old, new, count=1):
    text = load(path)
    found = text.count(old)
    assert found == count, f"{path}: anchor occurs {found} times, expected {count}: {old[:90]!r}"
    changes[path] = text.replace(old, new)

def between(path, start, end, new):
    text = load(path)
    assert text.count(start) == 1 and text.count(end) == 1, f"{path}: markers {start[:50]!r} {end[:50]!r}"
    a = text.index(start)
    b = text.index(end)
    assert a < b
    changes[path] = text[:a] + new + text[b:]

R = "🔁️replica/🟦️.ts"
C = "🔁️replica/⌨️command/🟦️.ts"
T = "🔁️replica/📖️texts/🔣️.json"
S = "🔁️replica/🧬️schema/🔣️.json"

rep(R, 'import {compileIgnoreV1,foldNameV1,gitPrivateV1,hashFileV1,linkRefusalV1,refusePathV1,scanSharedV1,sharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../🚧️boundary/🟦️.ts";',
    'import {compileIgnoreV1,foldNameV1,gitPrivateV1,hashFileV1,linkRefusalV1,refusePathV1,scanSharedV1,sharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../🚧️boundary/🟦️.ts";\n'
    'import {compactionDefaultsV1,cursorAheadV1,estimateEventsBehindV1,listSnapshotsV1,planRetirementV1,pruneSnapshotsV1,reclaimBlobsV1,requireCompactionShapeV1,retireSegmentsV1,scanSnapshotV1,snapshotSavesV1,storeReportV1,withCompactionLockV1,writeCompactionRecordV1,type CompactionSettingsV1,type ReclaimResultV1,type SnapshotRefV1,type StoreReportV1} from "../🗜️compaction/🟦️.ts";')
rep(R, "import {appendEventsV1,blobPathV1,copyVerifiedV1,", "import {appendEventsV1,blobPathV1,copyVerifiedV1,touchBlobV1,")
rep(R, "storeGate?:()=>Promise<void>;logFiles?:readonly string[]}>;", "storeGate?:()=>Promise<void>;logFiles?:readonly string[];compaction?:CompactionSettingsV1}>;")
rep(R, "persistMilliseconds:number;modeless:boolean;now:()=>number}>;", "persistMilliseconds:number;modeless:boolean;now:()=>number;compaction:CompactionSettingsV1}>;")
rep(R, 'modeless:options.modeless??process.platform==="win32",now:options.now??Date.now};', 'modeless:options.modeless??process.platform==="win32",now:options.now??Date.now,compaction:options.compaction??compactionDefaultsV1};')
rep(R, "health:HealthV1|null}>;", "health:HealthV1|null;store:StoreReportV1|null}>;")
rep(R, "failures:beat.failures}};\n requireReplicaShapeV1(\"Status\"",
    "failures:beat.failures},store:reachable?await storeReportV1(settings.storeRoot,settings.compaction,settings.now(),signal,deep).catch(()=>null):null};\n if(report.store!==null)requireCompactionShapeV1(\"Store\",report.store,\"Closed compaction store report required\");\n requireReplicaShapeV1(\"Status\"")

between(R, "async function recoverFromSnapshot(replica:ReplicaV1,origin:string):Promise<boolean>{", "type Spans=Awaited<ReturnType<typeof readEventSpansV1>>;",
'''async function adoptSnapshot(replica:ReplicaV1,origin:string|undefined,heads?:ReadonlyMap<string,FleetHeadV1|undefined>):Promise<boolean>{
 const {storeRoot,originId,compaction}=replica.settings,newest=(await listSnapshotsV1(storeRoot))[0];
 if(newest===undefined)return false;
 const useful=(cursors:Readonly<Record<string,FleetCursorV1>>):boolean=>origin!==undefined?cursors[origin]!==undefined&&cursorAheadV1(cursors[origin]!,replica.cursors[origin]):heads!==undefined&&snapshotSavesV1(replica.cursors,cursors,heads,originId,compaction.catchUpEvents);
 if(!useful(newest.cursors))return false;
 const staged=new CompactTableV1(replica.book,false),snapshot=await readNewestSnapshotV1(storeRoot,undefined,()=>{staged.clear();return staged;});
 if(snapshot===undefined||!useful(snapshot.cursors))return false;
 try{replica.clock=mergeClockV1(replica.clock,snapshot.hlc,replica.settings.now());}catch{replica.say({event:"clock-refused",origin:origin??"",hlc:snapshot.hlc});}
 for(const id of staged.ids()){
  const entry=staged.entryAt(id);
  if(!winsV1(entry.hlc,replica.manifest.live(id)?replica.manifest.hlcAt(id):undefined))continue;
  replica.manifest.write(id,entry);
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 for(const [other,cursor] of Object.entries(snapshot.cursors))if(other!==originId&&cursorAheadV1(cursor,replica.cursors[other]))replica.cursors[other]=cursor;
 replica.dirty=true;
 replica.say({event:"progress",stage:origin===undefined?"jump":"rejoin",origin:origin??"",done:staged.size,total:staged.size});
 return true;
}

const jumpTried=new WeakMap<ReplicaV1,number>(),jumpRetryMilliseconds=30_000;

async function jumpIfFar(replica:ReplicaV1,heads:ReadonlyMap<string,FleetHeadV1|undefined>):Promise<void>{
 const {compaction,originId}=replica.settings,now=replica.settings.now();
 if(!compaction.automatic||now-(jumpTried.get(replica)??-Infinity)<jumpRetryMilliseconds||estimateEventsBehindV1(replica.cursors,heads,originId)<compaction.catchUpEvents)return;
 jumpTried.set(replica,now);
 try{await adoptSnapshot(replica,undefined,heads);}catch(error){noteOnce(replica,`jump:${messageOf(error)}`,{event:"error",stage:"jump",message:messageOf(error)});}
}

''')
rep(R, "if(attempt===0&&await recoverFromSnapshot(replica,origin))continue;", "if(attempt===0&&await adoptSnapshot(replica,origin))continue;")
rep(R, "filter(origin=>origin!==originId),heads=await followHeads(replica,origins,signal);\n let events=0,more=false;",
    "filter(origin=>origin!==originId),heads=await followHeads(replica,origins,signal);\n await jumpIfFar(replica,heads);\n let events=0,more=false;")

rep(R, "const staged=new CompactTableV1(manifest.book,false),own=recovered?undefined:await readNewestSnapshotV1(storeRoot,origin,()=>{staged.clear();return staged;}),boundary=own?.cursors[origin],at=cursors[origin];",
    "const staged=new CompactTableV1(manifest.book,false),own=recovered?undefined:await readNewestSnapshotV1(storeRoot,undefined,()=>{staged.clear();return staged;}),boundary=own?.cursors[origin],at=cursors[origin];")
rep(R, "     cursors[origin]=boundary;\n     if(own.hlc>hlc)hlc=own.hlc;",
    "     for(const [other,cursor] of Object.entries(own.cursors))if(cursorAheadV1(cursor,cursors[other]))cursors[other]=cursor;\n     if(own.hlc>hlc)hlc=own.hlc;")

rep(R, "if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}", "if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}\n    void autoCompactV1(replica,signal);")
rep(R, "  if(verifiedBlobs.has(destination)||(await hashFileV1(destination).catch(()=>undefined))?.sha256===sha256){if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();verifiedBlobs.add(destination);return false;}",
    "  if(verifiedBlobs.has(destination)||(await hashFileV1(destination).catch(()=>undefined))?.sha256===sha256){if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();verifiedBlobs.add(destination);if(await touchBlobV1(destination))return false;}")

rep(R, "/** 🔍️ Rehashes the local shared tree and compares it with the persisted state and with the folded store;",
'''/** 🥁️ What a fleet compaction did, or in `plan` mode would do: the snapshot it rests on, the journal segments and blobs removed (or removable), the blobs left because they are younger than the safety age, the old snapshots removed, and why it did nothing (`locked` by another machine, `refused` because the fold needs a rejoin, `empty` store, `nosnapshot` in plan mode when no snapshot exists yet, `aborted` blob reclaim). */
export type FleetCompactResultV1=Readonly<{mode:"plan"|"apply";snapshot:string|null;entries:number;segments:number;segmentBytes:number;blobs:number;blobBytes:number;pending:number;pendingBytes:number;snapshots:number;ignored:Readonly<Record<string,string>>;skipped:"locked"|"refused"|"empty"|"nosnapshot"|"aborted"|null}>;

/** 🎴️ Compacts the whole store so that the cost of joining, catching up and rebuilding follows the live paths and not the history: writes a snapshot of the fold (one machine; two writing at once is safe, each snapshot is a complete fold written by tmp, fsync and rename), checks the snapshot by reading it whole, removes the journal segments of every origin that the snapshot covers, that every fresh origin with synchronisation on has attested to have passed and that are older than `safetyHours` (origins silent for `staleHours`, off or retired do not hold it back and resume from the snapshot when they return), reclaims blobs that neither the snapshot nor a later event names and that are older than `safetyHours` (see `reclaimBlobsV1`), removes snapshots beyond the newest two and writes `compaction.json`. In `plan` mode nothing is written: it measures against the newest existing snapshot. `apply` holds the store-wide compaction lock; a held lock gives `locked`. Cancellable, with `progress` events of the stages `retire` and `reclaim`. */
export async function fleetCompactV1(storeRoot:string,originId:string,settings:CompactionSettingsV1,mode:"plan"|"apply",signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},now:()=>number=Date.now,options:Readonly<{blobs?:boolean}>={}):Promise<FleetCompactResultV1>{
 requireOriginIdV1(originId);
 const safety=settings.safetyHours*3_600_000,stale=settings.staleHours*3_600_000,blank:FleetCompactResultV1={mode,snapshot:null,entries:0,segments:0,segmentBytes:0,blobs:0,blobBytes:0,pending:0,pendingBytes:0,snapshots:0,ignored:{},skipped:null};
 const work=async():Promise<FleetCompactResultV1>=>{
  let ref:SnapshotRefV1|undefined;
  if(mode==="apply"){
   let incomplete=false;
   const made=await snapshotV1(storeRoot,originId,signal,event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);});
   if(made===undefined||incomplete)return {...blank,skipped:incomplete?"refused":"empty"};
   ref=(await listSnapshotsV1(storeRoot)).find(item=>item.file===made.file);
  }else ref=(await listSnapshotsV1(storeRoot))[0];
  if(ref===undefined)return {...blank,skipped:"nosnapshot"};
  const referenced=new Set<string>(),entries=await scanSnapshotV1(ref.file,ref.hlc,options.blobs===false?undefined:sha=>{referenced.add(sha);});
  const plan=await planRetirementV1(storeRoot,ref,{now:now(),safetyMilliseconds:safety,staleMilliseconds:stale,signal});
  let segments=plan.segments,segmentBytes=plan.bytes,reclaimed:ReclaimResultV1={scanned:0,blobs:0,bytes:0,pending:0,pendingBytes:0,restored:0,aborted:false},snapshots=0;
  if(mode==="apply"){const removed=await retireSegmentsV1(storeRoot,plan,{signal,progress:(done,total)=>sink({version:1,event:"progress",stage:"retire",done,total})});segments=removed.segments;segmentBytes=removed.bytes;}
  if(options.blobs!==false)reclaimed=await reclaimBlobsV1(storeRoot,ref,{now:now(),safetyMilliseconds:safety,apply:mode==="apply",signal,referenced,progress:(done,total,blobs,bytes)=>sink({version:1,event:"progress",stage:"reclaim",done,total,count:blobs,bytes})});
  if(mode==="apply"){
   snapshots=(await pruneSnapshotsV1(storeRoot,{keep:2,now:now(),safetyMilliseconds:safety,signal})).files;
   await reclaimStagingV1(storeRoot,{olderThanMilliseconds:staleStagingMilliseconds,now:now()});
   await writeCompactionRecordV1(storeRoot,{version:1,wallMs:now(),originId,snapshot:{hlc:ref.hlc,origin:ref.origin,entries},segments:{removed:segments,bytes:segmentBytes},blobs:{removed:reclaimed.blobs,bytes:reclaimed.bytes,pending:reclaimed.pending,pendingBytes:reclaimed.pendingBytes},snapshots});
   sink({version:1,event:"compacted",count:segments+reclaimed.blobs+snapshots,bytes:segmentBytes+reclaimed.bytes});
  }
  return {mode,snapshot:ref.file,entries,segments,segmentBytes,blobs:reclaimed.blobs,blobBytes:reclaimed.bytes,pending:reclaimed.pending,pendingBytes:reclaimed.pendingBytes,snapshots,ignored:plan.ignored,skipped:reclaimed.aborted?"aborted":null};
 };
 if(mode==="plan")return work();
 const held=await withCompactionLockV1(storeRoot,originId,work,now);
 return held===undefined?{...blank,skipped:"locked"}:held.value;
}

const compacting=new WeakSet<ReplicaV1>(),compactedAt=new WeakMap<ReplicaV1,number>(),autoCompactMilliseconds=600_000;

/** 🃏️ The automatic compaction of the group server (origin `<group>-server`), off unless `replica.compaction.automatic` is set: at most every ten minutes it compares the newest snapshot with the heads and runs `fleetCompactV1` in `apply` mode when there is none, when it is `snapshotEvents` events behind, or when it is behind at all and `safetyHours` old (so that segments and blobs that have come of age are removed once a day). Returns the running promise, or `undefined` when nothing was started; never throws. */
export function autoCompactV1(replica:ReplicaV1,signal?:AbortSignal):Promise<void>|undefined{
 const {settings}=replica,{compaction}=settings,now=settings.now();
 if(!compaction.automatic||settings.originId!==`${settings.groupId}-server`||compacting.has(replica)||now-(compactedAt.get(replica)??-Infinity)<autoCompactMilliseconds)return undefined;
 compactedAt.set(replica,now);compacting.add(replica);
 return (async()=>{
  const heads=new Map<string,FleetHeadV1|undefined>();
  for(const origin of await listOriginsV1(settings.storeRoot))heads.set(origin,await readHeadV1(settings.storeRoot,origin));
  const newest=(await listSnapshotsV1(settings.storeRoot))[0],behind=estimateEventsBehindV1(newest?.cursors??{},heads);
  if(newest===undefined||behind>=compaction.snapshotEvents||behind>0&&now-newest.mtimeMs>=compaction.safetyHours*3_600_000)await fleetCompactV1(settings.storeRoot,settings.originId,compaction,"apply",signal,event=>replica.say(event as Omit<ReplicaEventV1,"version">),settings.now);
 })().catch(error=>{if(signal?.aborted!==true)noteOnce(replica,`compaction:${messageOf(error)}`,{event:"error",stage:"compaction",message:messageOf(error)});}).finally(()=>{compacting.delete(replica);});
}

/** 🔍️ Rehashes the local shared tree and compares it with the persisted state and with the folded store;''')

# command
rep(C, 'import {compactV1,dryOpenStateV1,', 'import {compactV1,dryOpenStateV1,fleetCompactV1,')
rep(C, '"snapshot"|"relay"|"compact"|"verify"|', '"snapshot"|"relay"|"compact"|"compaction"|"verify"|')
rep(C, '  verify:{valued:["--worktree","--store","--lang"],plain:["--json"]},', '  compaction:{valued:["--worktree","--store","--lang"],plain:["--plan","--apply","--no-blobs"]},\n  verify:{valued:["--worktree","--store","--lang"],plain:["--json"]},')
rep(C, "retentionDays:limits(\"--retention-days\",defaults.retentionDays,0,3650)}};", "retentionDays:limits(\"--retention-days\",defaults.retentionDays,0,3650),compaction:defaults.compaction}};")
rep(C, ' if(parsed.operation==="compact"){', '''
 if(parsed.operation==="compaction"){
  const plan=parsed.flags.has("--plan"),apply=parsed.flags.has("--apply");
  if(plan===apply)throw Error(both("error.compaction"));
  const result=await fleetCompactV1(storeRoot,options.originId,options.compaction!,apply?"apply":"plan",signal,sink,Date.now,{blobs:!parsed.flags.has("--no-blobs")});
  if(result.skipped==="locked"||result.skipped==="refused"||result.skipped==="empty"||result.skipped==="nosnapshot"){say(`compaction.${result.skipped}`);return {exitCode:result.skipped==="locked"||result.skipped==="refused"?1:0};}
  for(const [origin,why] of Object.entries(result.ignored))say("compaction.ignored",{origin,why:`@compaction.why.${why}`});
  say(apply?"compaction.done":"compaction.plan",{entries:result.entries,segments:result.segments,segmentbytes:formatReplicaBytesV1(result.segmentBytes),blobs:result.blobs,blobbytes:formatReplicaBytesV1(result.blobBytes),pending:result.pending,pendingbytes:formatReplicaBytesV1(result.pendingBytes),snapshots:result.snapshots});
  if(result.skipped==="aborted")say("compaction.aborted");
  return {exitCode:result.skipped==="aborted"?1:0};
 }
 if(parsed.operation==="compact"){''')
rep(C, ' say(`status.git.${report.git.reason}`,{tree:report.git.indexTree??"-"});',
    ''' if(report.store!==null){
  say("status.store.journal",{origins:report.store.journal.origins,segments:report.store.journal.segments,bytes:formatReplicaBytesV1(report.store.journal.bytes)});
  if(report.store.snapshot===null)say("status.store.nosnapshot");
  else say("status.store.snapshot",{origin:report.store.snapshot.origin,age:`${report.store.snapshot.ageSeconds}s`,bytes:formatReplicaBytesV1(report.store.snapshot.bytes)});
  say("status.store.reclaim",{segments:report.store.reclaimable.segments,bytes:formatReplicaBytesV1(report.store.reclaimable.bytes)});
  if(report.store.reclaimable.blobs!==null)say("status.store.blobs",{blobs:report.store.reclaimable.blobs.count,bytes:formatReplicaBytesV1(report.store.reclaimable.blobs.bytes),pending:report.store.reclaimable.blobs.pending,pendingbytes:formatReplicaBytesV1(report.store.reclaimable.blobs.pendingBytes)});
  if(report.store.last!==null)say("status.store.last",{age:`${Math.max(0,Math.floor((Date.now()-report.store.last.wallMs)/1000))}s`,blobs:report.store.last.blobs,bytes:formatReplicaBytesV1(report.store.last.blobBytes),pending:report.store.last.pending,pendingbytes:formatReplicaBytesV1(report.store.last.pendingBytes)});
 }
 say(`status.git.${report.git.reason}`,{tree:report.git.indexTree??"-"});''')

# texts: operation list + new messages
text = load(T)
assert text.count("compact|verify") >= 1
changes[T] = text.replace("compact|verify", "compact|compaction|verify")
rep(T, '    "compact.done": {', '''    "error.compaction": {
      "en": "compaction needs exactly one of --plan or --apply.",
      "de": "compaction braucht genau eines von --plan oder --apply."
    },
    "compaction.locked": {
      "en": "Another machine is compacting the store right now (compaction.lock); nothing was done.",
      "de": "Eine andere Maschine verdichtet den Speicher gerade (compaction.lock); es wurde nichts getan."
    },
    "compaction.refused": {
      "en": "The store cannot be folded without a rejoin; no snapshot was written and nothing was removed.",
      "de": "Der Speicher lässt sich nicht ohne erneuten Beitritt falten; es wurde kein Schnappschuss geschrieben und nichts entfernt."
    },
    "compaction.nosnapshot": {
      "en": "No snapshot exists yet; compaction --apply writes one first. Nothing was changed.",
      "de": "Es gibt noch keinen Schnappschuss; compaction --apply schreibt zuerst einen. Es wurde nichts geändert."
    },
    "compaction.empty": {
      "en": "The store holds no events yet; nothing to compact.",
      "de": "Der Speicher enthält noch keine Ereignisse; nichts zu verdichten."
    },
    "compaction.aborted": {
      "en": "The blob reclaim stopped because another compaction moved past the snapshot; everything it had set aside was put back.",
      "de": "Das Freigeben der Blobs wurde abgebrochen, weil eine andere Verdichtung den Schnappschuss überholt hat; alles Beiseitegelegte wurde zurückgestellt."
    },
    "compaction.ignored": {
      "en": "{origin} does not hold back the retirement of segments ({why}).",
      "de": "{origin} hält das Entfernen von Segmenten nicht auf ({why})."
    },
    "compaction.why.stale": {"en": "silent for too long, it resumes from the snapshot", "de": "zu lange still, es setzt beim Schnappschuss wieder auf"},
    "compaction.why.off": {"en": "synchronisation is off", "de": "Synchronisation ist aus"},
    "compaction.why.retired": {"en": "retired", "de": "ausgemustert"},
    "compaction.why.unattested": {"en": "never attested", "de": "nie bestätigt"},
    "compaction.plan": {
      "en": "Plan on the newest snapshot ({entries} paths): {segments} journal segments ({segmentbytes}) and {blobs} blobs ({blobbytes}) could be removed; {pending} blobs ({pendingbytes}) are still too young. Nothing was changed.",
      "de": "Plan auf dem neuesten Schnappschuss ({entries} Pfade): {segments} Journal-Segmente ({segmentbytes}) und {blobs} Blobs ({blobbytes}) ließen sich entfernen; {pending} Blobs ({pendingbytes}) sind noch zu jung. Es wurde nichts geändert."
    },
    "compaction.done": {
      "en": "Compacted on a snapshot of {entries} paths: {segments} journal segments ({segmentbytes}) and {blobs} blobs ({blobbytes}) removed, {snapshots} old snapshots removed; {pending} blobs ({pendingbytes}) were too young and stay.",
      "de": "Verdichtet auf einem Schnappschuss von {entries} Pfaden: {segments} Journal-Segmente ({segmentbytes}) und {blobs} Blobs ({blobbytes}) entfernt, {snapshots} alte Schnappschüsse entfernt; {pending} Blobs ({pendingbytes}) waren zu jung und bleiben."
    },
    "status.store.journal": {
      "en": "Journal: {bytes} in {segments} segments of {origins} origins.",
      "de": "Journal: {bytes} in {segments} Segmenten von {origins} Origins."
    },
    "status.store.snapshot": {
      "en": "Newest snapshot by {origin}: {age} old, {bytes}.",
      "de": "Neuester Schnappschuss von {origin}: {age} alt, {bytes}."
    },
    "status.store.nosnapshot": {
      "en": "No snapshot yet: a new machine reads the whole journal.",
      "de": "Noch kein Schnappschuss: eine neue Maschine liest das ganze Journal."
    },
    "status.store.reclaim": {
      "en": "Reclaimable now: {segments} journal segments ({bytes}).",
      "de": "Jetzt freigebbar: {segments} Journal-Segmente ({bytes})."
    },
    "status.store.blobs": {
      "en": "Reclaimable blobs (measured by --deep): {blobs} ({bytes}); {pending} more ({pendingbytes}) are still too young.",
      "de": "Freigebbare Blobs (mit --deep gemessen): {blobs} ({bytes}); weitere {pending} ({pendingbytes}) sind noch zu jung."
    },
    "status.store.last": {
      "en": "Last compaction {age} ago: {blobs} blobs ({bytes}) removed; {pending} blobs ({pendingbytes}) were still too young.",
      "de": "Letzte Verdichtung vor {age}: {blobs} Blobs ({bytes}) entfernt; {pending} Blobs ({pendingbytes}) waren noch zu jung."
    },
    "compact.done": {''')

# schema: status.store
rep(S, '"required": ["version", "originId", "groupId", "storeRoot", "reachable", "daemon", "sync", "files", "queue", "lost", "refused", "git", "origins", "retired", "latency", "health"],',
       '"required": ["version", "originId", "groupId", "storeRoot", "reachable", "daemon", "sync", "files", "queue", "lost", "refused", "git", "origins", "retired", "latency", "health", "store"],')
rep(S, '        "health": {\n          "description": "Null, or what the running daemon wrote',
       '        "store": {\n          "description": "Null while the store is unreachable, else the size report of the store (see `Store` of the compaction schema, which validates it): journal size, newest snapshot, what a compaction would reclaim now.",\n          "oneOf": [{ "type": "null" }, { "type": "object" }]\n        },\n        "health": {\n          "description": "Null, or what the running daemon wrote')

for path, text in changes.items():
    with open(os.path.join(root, path), "w", encoding="utf8") as handle:
        handle.write(text)
print("patched", len(changes), "files")
