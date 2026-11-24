"""Builds the L1 patches from the play snapshot's own file contents: python3 make.py <snapshot-root> <live-root>.
Every edit is an exact anchored replacement on the snapshot text; the live copies are only used for the new test directory."""
import hashlib, os, shutil, subprocess, sys
snap, live = sys.argv[1:3]
here = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
U = L + "/⚡️caching/🦀️cargo/🗃️unit-cache"
CP = L + "/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler"
work = os.path.join(here, "work")
shutil.rmtree(work, ignore_errors=True)

def edit(rel, pairs):
    src = os.path.join(snap, rel)
    text = open(src, encoding="utf8").read()
    for old, new in pairs:
        assert text.count(old) == 1, (rel, old[:80], text.count(old))
        text = text.replace(old, new)
    out = os.path.join(work, rel)
    os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out, "w", encoding="utf8").write(text)
    return rel

rows = []
rows.append(edit(U + "/🧬️schema/🔣️.json", [
('''            "retainMilliseconds": 43200000
          },
          "catalogClaim": "pool-catalog"''','''            "retainMilliseconds": 43200000
          },
          "recentMilliseconds": 43200000,
          "flavourPattern": "^[a-z0-9-]{1,16}$",
          "catalogClaim": "pool-catalog"'''),
('''        "poolBusy": {''','''        "poolCannotHold": {
          "en": "[repo cargo] the lanes of build flavour {flavour} need {needMib} MiB but the compiler image holds {imageMib} MiB below its high-water mark; no lane was removed, the build continues and refills from the shared unit cache (enlarge the image or build a smaller flavour)",
          "de": "[repo cargo] Die Lanes der Build-Variante {flavour} brauchen {needMib} MiB, das Compiler-Image fasst unterhalb seiner Hochwassermarke aber nur {imageMib} MiB; es wurde keine Lane entfernt, der Build läuft weiter und füllt sich aus dem gemeinsamen Unit-Cache (Image vergrößern oder eine kleinere Variante bauen)"
        },
        "poolBusy": {''')]))
rows.append(edit(U + "/🧵️lane/🟦️.ts", [
('import {join} from "node:path";','import {basename,join} from "node:path";'),
('crates?:readonly string[]}>;','crates?:readonly string[];usedMs?:number;flavour?:string}>;'),
('export function enterLaneV1(root:string,id:string,owner?:LaneOwnerV1,crates?:readonly string[]):Readonly','export function enterLaneV1(root:string,id:string,owner?:LaneOwnerV1,crates?:readonly string[],flavour?:string):Readonly'),
('writeAtomicV1(paths.metadata,JSON.stringify({version:1,id,createdMs:before?.createdMs??Date.now(),','const kept=flavour??before?.flavour;\n writeAtomicV1(paths.metadata,JSON.stringify({version:1,id,createdMs:before?.createdMs??Date.now(),usedMs:Date.now(),...kept===undefined?{}:{flavour:kept},'),
(' const touch=()=>{try{const now=new Date();utimesSync(paths.metadata,now,now);}catch{}};',' const touch=()=>{try{const now=new Date(),record=readLaneV1(root,id);if(record!==undefined)writeAtomicV1(paths.metadata,JSON.stringify({...record,usedMs:now.getTime()}));utimesSync(paths.metadata,now,now);}catch{}};'),
('/** 🧬️ The identifiers of the lanes that exist in a cache root. */','''/** 🕰️ When a lane was last used: the later of its recorded use stamp (`usedMs`, content that a restore or copy keeps) and the modification time of its metadata (heartbeat of older records), else the modification time of the lane directory, else `0`. */
export function laneUsedMsV1(root:string,id:string):number{
 const paths=lanePathsV1(root,id),stamp=(path:string):number=>{try{return Math.floor(statSync(path).mtimeMs);}catch{return 0;}};
 const record=readLaneV1(root,id);return Math.max(Number.isSafeInteger(record?.usedMs)?record!.usedMs!:0,stamp(paths.metadata),record===undefined?stamp(paths.base):0);
}

/** 🌡️ Whether a lane of a declared build flavour was used within `recentMilliseconds`: the lanes a build of that flavour needs are not shed for the pool's soft bounds, only by free-space pressure and never when the starting build requests them. */
export function laneRecentV1(flavour:string|undefined,lastUsedMs:number,nowMs:number):boolean{return flavour!==undefined&&nowMs-lastUsedMs<policy.pool.recentMilliseconds;}

/** 🏷️ The build flavour a shared compiler build directory stands for: its own directory name (one per build mode, see the compiler layout), `undefined` when the name is no flavour. Pure. */
export function laneFlavourV1(sharedBuild:string):string|undefined{const name=basename(sharedBuild);return new RegExp(policy.pool.flavourPattern,"u").test(name)?name:undefined;}

/** 🧬️ The identifiers of the lanes that exist in a cache root. */'''),
('const paths=lanePathsV1(root,id),limit={files:2_000_000};let lastUsedMs=0;try{lastUsedMs=Math.floor(statSync(paths.metadata).mtimeMs);}catch{}','const paths=lanePathsV1(root,id),limit={files:2_000_000},lastUsedMs=laneUsedMsV1(root,id);'),
('retained:laneRetainedV1(id,lastUsedMs,nowMs)};});','retained:laneRetainedV1(id,lastUsedMs,nowMs)||laneRecentV1(readLaneV1(root,id)?.flavour,lastUsedMs,nowMs)};});'),
]))
P = U + "/🧵️lane/🏊️pool/🟦️.ts"
rows.append(edit(P, [
('import {mkdirSync,statSync} from "node:fs";','import {mkdirSync} from "node:fs";'),
('import {crateKeyV1,enterLaneV1,laneIdsV1,laneLiveV1,lanePathsV1,laneRetainedV1,readLaneSizeV1,readLaneV1,removeLaneV1,type LaneOwnerV1} from "../🟦️.ts";','import {crateKeyV1,enterLaneV1,laneIdsV1,laneLiveV1,lanePathsV1,laneRecentV1,laneRetainedV1,laneUsedMsV1,readLaneSizeV1,readLaneV1,removeLaneV1,type LaneOwnerV1} from "../🟦️.ts";'),
('product?:boolean;retained?:boolean}>;','product?:boolean;retained?:boolean;recent?:boolean;flavour?:string}>;'),
('/** 🪣️ The bounds of the pool for a byte quota. */','''/** 🌡️ What a trim knows about the build that is starting: the lanes it requests (never victims) and the image it runs on (`volume`, whose free space decides whether recent lanes may be shed). */
export type PoolTrimContextV1=Readonly<{requested?:readonly string[];flavour?:string;volume?:()=>Readonly<{volumeBytes:number;freeBytes:number}>}>;

/** 📦️ The bytes the lanes of the largest declared build flavour need: the recorded bytes of the lanes of each flavour summed, the greatest sum. The pool's byte bound is never below it. Pure. */
export function poolNeedBytesV1(lanes:readonly PoolLaneV1[]):number{
 const sums=new Map<string,number>();for(const lane of lanes)if(lane.flavour!==undefined)sums.set(lane.flavour,(sums.get(lane.flavour)??0)+(lane.bytes??0));
 return Math.max(0,...sums.values());
}

/** 🪣️ The bounds of the pool for a byte quota. */'''),
('''/** ✂️ Chooses the idle lanes to remove so the pool is back inside its bounds: least recently used first, never a claimed lane. Pure; applying the result and planning again removes nothing. */
export function planPoolTrimV1(lanes:readonly PoolLaneV1[],bounds:PoolBoundsV1):string[]{
 const victims:string[]=[];let count=lanes.length,bytes=recorded(lanes);
 for(const lane of lanes.filter(row=>!row.claimed).sort(oldest)){''','''/** ✂️ Chooses the idle lanes to remove so the pool is back inside its bounds: least recently used first, never a claimed lane, never a requested lane, and a recent lane of a declared flavour only under free-space `pressure`. Pure; applying the result and planning again removes nothing. */
export function planPoolTrimV1(lanes:readonly PoolLaneV1[],bounds:PoolBoundsV1,context:Readonly<{requested?:readonly string[];pressure?:boolean}>={}):string[]{
 const victims:string[]=[],requested=new Set(context.requested??[]);let count=lanes.length,bytes=recorded(lanes);
 for(const lane of lanes.filter(row=>!row.claimed&&!requested.has(row.id)&&(context.pressure===true||row.recent!==true)).sort(oldest)){'''),
('  const paths=lanePathsV1(root,id);let lastUsedMs=0;try{lastUsedMs=Math.floor(statSync(paths.metadata).mtimeMs);}catch{}\n','  const paths=lanePathsV1(root,id),lastUsedMs=laneUsedMsV1(root,id),flavour=readLaneV1(root,id)?.flavour;\n'),
('retained:laneRetainedV1(id,lastUsedMs,nowMs),claimed:','retained:laneRetainedV1(id,lastUsedMs,nowMs),recent:laneRecentV1(flavour,lastUsedMs,nowMs),...flavour===undefined?{}:{flavour},claimed:'),
('/** 🧹️ Removes the idle lanes the pool must shed. Every removal takes','/** 🧹️ Removes the idle lanes the pool must shed, a lane the starting build requests never among them. The byte bound is raised to what the largest declared build flavour needs; when the image cannot hold that need below its high-water mark the pool says so and removes nothing; recent lanes of a declared flavour go only while the image is under free-space pressure. Every removal takes'),
('''export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number,report?:PoolReportV1):Readonly<{removed:number;freedBytes:number}>{
 const lanes=boundedPoolV1(readPoolV1(root,nowMs)),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0]));let removed=0,freedBytes=0;''','''export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number,report?:PoolReportV1,context:PoolTrimContextV1={}):Readonly<{removed:number;freedBytes:number}>{
 const lanes=boundedPoolV1(readPoolV1(root,nowMs)),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0])),need=poolNeedBytesV1(lanes),volume=context.volume?.(),high=volume===undefined?Infinity:Math.floor(volume.volumeBytes*UNIT_CACHE_POLICY_V1.gc.highWaterShare);let removed=0,freedBytes=0;
 if(need>high){report?.("poolCannotHold",{flavour:context.flavour??"",needMib:Math.round(need/1048576),imageMib:Math.round(high/1048576)});return Object.freeze({removed,freedBytes});}
 const pressure=volume!==undefined&&volume.volumeBytes-volume.freeBytes>high,requested=[...context.requested??[],...lanes.filter(lane=>context.flavour!==undefined&&lane.flavour===context.flavour&&lane.recent===true).map(lane=>lane.id)];'''),
('for(const id of planPoolTrimV1(lanes,bounds))if','for(const id of planPoolTrimV1(lanes,{...bounds,quotaBytes:Math.max(bounds.quotaBytes,need)},{requested,pressure}))if'),
('export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy"|','export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy"|"poolCannotHold"|'),
('request:Readonly<{crates:readonly string[];quotaBytes:number;owner?:LaneOwnerV1}>,options:Readonly<{signal?:AbortSignal;now?:()=>number;report?:PoolReportV1}>={}','request:Readonly<{crates:readonly string[];quotaBytes:number;owner?:LaneOwnerV1;flavour?:string}>,options:Readonly<{signal?:AbortSignal;now?:()=>number;report?:PoolReportV1;volume?:PoolTrimContextV1["volume"]}>={}'),
('enterLaneV1(root,id,request.owner,request.crates);}catch','enterLaneV1(root,id,request.owner,request.crates,request.flavour);}catch'),
('try{const trimmed=trimPoolV1(root,bounds,now(),options.report);if(trimmed.removed>0)options.report?.("trimmed",{count:laneIdsV1(root).length,','try{const trimmed=trimPoolV1(root,bounds,now(),options.report,{requested:[id],...request.flavour===undefined?{}:{flavour:request.flavour},...options.volume===undefined?{}:{volume:options.volume}});if(trimmed.removed>0)options.report?.("trimmed",{count:readPoolV1(root,now()).length,'),
]))
rows.append(edit(U + "/🧵️lane/🏊️pool/🎯️product/🟦️.ts", [
('import {laneHoldersV1,poolBoundsV1,readPoolV1,trimPoolV1,type PoolReportV1} from "../🟦️.ts";','import {laneHoldersV1,poolBoundsV1,readPoolV1,trimPoolV1,type PoolReportV1,type PoolTrimContextV1} from "../🟦️.ts";'),
('request:Readonly<{key:string;crates:readonly string[];quotaBytes:number;owner:LaneOwnerV1}>,options:Readonly<{signal:AbortSignal;report?:PoolReportV1}>','request:Readonly<{key:string;crates:readonly string[];quotaBytes:number;owner:LaneOwnerV1;flavour?:string}>,options:Readonly<{signal:AbortSignal;report?:PoolReportV1;volume?:PoolTrimContextV1["volume"]}>'),
('const entered=enterLaneV1(root,id,request.owner,request.crates);try{trimPoolV1(root,bounds,Date.now(),options.report);}','const entered=enterLaneV1(root,id,request.owner,request.crates,request.flavour);try{trimPoolV1(root,bounds,Date.now(),options.report,trimContext);}'),
('if(!lanes.some(lane=>lane.id===id)&&lanes.length>=bounds.maximumLiveLanes){trimPoolV1(root,bounds,Date.now(),options.report);','if(!lanes.some(lane=>lane.id===id)&&lanes.length>=bounds.maximumLiveLanes){trimPoolV1(root,bounds,Date.now(),options.report,trimContext);'),
('bounds=poolBoundsV1(request.quotaBytes);let poll=policy.pollMilliseconds,reported=false;','bounds=poolBoundsV1(request.quotaBytes),trimContext:PoolTrimContextV1={requested:[id],...request.flavour===undefined?{}:{flavour:request.flavour},...options.volume===undefined?{}:{volume:options.volume}};let poll=policy.pollMilliseconds,reported=false;'),
]))
rows.append(edit(CP + "/🛤️lane/🟦️.ts", [
('export type CompilerLaneRequestV1=Readonly<{crates:readonly string[];unitCache?:boolean}>;','export type CompilerLaneRequestV1=Readonly<{crates:readonly string[];unitCache?:boolean;flavour?:string}>;'),
('import {laneMessageV1,unitCacheCapacityBytesV1,','import {measureVolumeV1} from "../../../../🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts";\nimport {laneMessageV1,unitCacheCapacityBytesV1,'),
('const quotaBytes=await compilerLaneQuotaBytesV1(input.host,store.environment,unitCacheCapacityBytesV1(space));','const capacityBytes=unitCacheCapacityBytesV1(space),quotaBytes=await compilerLaneQuotaBytesV1(input.host,store.environment,capacityBytes),flavour=request.flavour;'),
('options={signal:control.signal,report:','options={signal:control.signal,volume:()=>measureVolumeV1(cacheRoot,capacityBytes),report:'),
('acquirePooledLaneV1(cacheRoot,{crates:request.crates,quotaBytes,owner},options)','acquirePooledLaneV1(cacheRoot,{crates:request.crates,quotaBytes,owner,...flavour===undefined?{}:{flavour}},options)'),
('acquireProductLaneV1(cacheRoot,{key:product,crates:request.crates,quotaBytes,owner},options)','acquireProductLaneV1(cacheRoot,{key:product,crates:request.crates,quotaBytes,owner,...flavour===undefined?{}:{flavour}},options)'),
]))
rows.append(edit(CP + "/🟦️.ts", [
('import {withCompilerLaneViewV1','import {laneFlavourV1} from "../../../🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts";\nimport {withCompilerLaneViewV1'),
('return withCompilerLaneViewV1({host:owner.host,store,control,request,register:','return withCompilerLaneViewV1({host:owner.host,store,control,request:{...request,...laneFlavourV1(store.buildDirectory)===undefined?{}:{flavour:laneFlavourV1(store.buildDirectory)!}},register:'),
('request:{crates:captured.crates},product,register:','request:{crates:captured.crates,...laneFlavourV1(store.buildDirectory)===undefined?{}:{flavour:laneFlavourV1(store.buildDirectory)!}},product,register:'),
]))
LP = L + "/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
rows.append(edit(LP, [
('import {crateKeyV1,measureLaneBytesV1} from','import {crateKeyV1,laneFlavourV1,measureLaneBytesV1} from'),
('import {gcOutOfSpaceV1} from','import {gcOutOfSpaceV1,measureVolumeV1} from'),
('classes=options.classes??["lane"];\n','classes=options.classes??["lane"],flavour=laneFlavourV1(options.sharedBuild);\n'),
('acquirePooledLaneV1(options.cacheRoot,{crates:options.crates,quotaBytes,owner:{session:agentSessionAddressV1(session),agent:session.agent.id}},{signal:options.control.signal,report:','acquirePooledLaneV1(options.cacheRoot,{crates:options.crates,quotaBytes,owner:{session:agentSessionAddressV1(session),agent:session.agent.id},...flavour===undefined?{}:{flavour}},{signal:options.control.signal,volume:()=>measureVolumeV1(options.cacheRoot,options.capacityBytes),report:'),
]))
PT = U + "/🧵️lane/🏊️pool/🎯️product/🧪️tests/🟦️.ts"
rows.append(edit(PT, [
('rmSync,utimesSync} from "node:fs"','rmSync,utimesSync,writeFileSync} from "node:fs"'),
('import {recordLaneSizeV1} from "../../../🟦️.ts";','import {readLaneV1,recordLaneSizeV1} from "../../../🟦️.ts";'),
('utimesSync(product.paths.metadata,aged,aged);','writeFileSync(product.paths.metadata,JSON.stringify({...readLaneV1(root,product.paths.id),usedMs:aged.getTime()}));utimesSync(product.paths.metadata,aged,aged);'),
]))
newdir = U + "/🧵️lane/🏊️pool/🧪️tests/🌡️flavour-need"
files = []
for name in os.listdir(os.path.join(live, newdir)):
    out = os.path.join(work, newdir, name)
    os.makedirs(os.path.dirname(out), exist_ok=True)
    shutil.copyfile(os.path.join(live, newdir, name), out)
    files.append(newdir + "/" + name)
patches = os.path.join(here, "patches")
shutil.rmtree(patches, ignore_errors=True); os.makedirs(patches)
lines = ["order\tfile (under the snapshot root)\tpre-image sha256\tpatch\tread\n"]
for index, rel in enumerate(rows, 1):
    pre = hashlib.sha256(open(os.path.join(snap, rel), "rb").read()).hexdigest()
    name = "%02d.patch" % index
    diff = subprocess.run(["diff", "-u", "--label", "a/" + rel, "--label", "b/" + rel, os.path.join(snap, rel), os.path.join(work, rel)], capture_output=True, text=True).stdout
    open(os.path.join(patches, name), "w", encoding="utf8").write(diff)
    lines.append("%d\t%s\t%s\t%s\tW (per rustc/cargo call) except launcher and compiler: D (driver start)\n" % (index, rel, pre, name))
tests = os.path.join(here, "new-files")
shutil.rmtree(tests, ignore_errors=True)
for rel in files:
    out = os.path.join(tests, rel); os.makedirs(os.path.dirname(out), exist_ok=True); shutil.copyfile(os.path.join(work, rel), out)
open(os.path.join(here, "list.tsv"), "w", encoding="utf8").write("".join(lines))
shutil.rmtree(work)
print(len(rows), "patches,", len(files), "new files")
