// usage: bun bench.ts <module root> <copy of a store: dir that holds v1>   -- times fold (join/rebuild) before and after a fleet compaction on a COPY of the live journals (blobs not copied: run with blobs:false)
import {createHash} from "node:crypto";
import {join} from "node:path";
const [moduleRoot,store]=[process.argv[2]!,process.argv[3]!];
const {foldStoreV1,fleetCompactV1}=await import(join(moduleRoot,"🔁️replica","🟦️.ts"));
const {listSnapshotsV1,planRetirementV1,storeReportV1}=await import(join(moduleRoot,"🗜️compaction","🟦️.ts"));
const settings={automatic:true,snapshotEvents:100000,safetyHours:24,staleHours:12,catchUpEvents:20000};
async function fold(label:string){
 const t=performance.now(),before=process.memoryUsage().rss;
 const f=await foldStoreV1(store);
 const hash=createHash("sha256");
 for(const path of [...f.manifest.keys()].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))))hash.update(`${path}\t${JSON.stringify(f.manifest.get(path),Object.keys(f.manifest.get(path)!).sort())}\n`);
 console.log(JSON.stringify({label,seconds:Math.round(performance.now()-t)/1000,entries:f.manifest.size,rssMB:Math.round((process.memoryUsage().rss)/1e6),digest:hash.digest("hex").slice(0,16)}));
}
await fold("fold before compaction (snapshot 03:30 plus tail)");
const report=await storeReportV1(store,settings,Date.now());
console.log(JSON.stringify({label:"status before",report}));
const plan=await fleetCompactV1(store,"g1-server",settings,"plan",undefined,()=>{},()=>Date.now(),{blobs:false});
console.log(JSON.stringify({label:"plan on the 03:30 snapshot, now",skipped:plan.skipped,segments:plan.segments,segmentBytes:plan.segmentBytes,ignored:plan.ignored}));
let t=performance.now();
const applied=await fleetCompactV1(store,"g1-server",settings,"apply",undefined,()=>{},()=>Date.now(),{blobs:false});
console.log(JSON.stringify({label:"apply now (new snapshot)",seconds:Math.round(performance.now()-t)/1000,entries:applied.entries,segments:applied.segments,segmentBytes:applied.segmentBytes,snapshots:applied.snapshots,skipped:applied.skipped}));
await fold("fold after compaction now");
const day=24*3600e3;
const later=await fleetCompactV1(store,"g1-server",{...settings,safetyHours:1},"apply",undefined,()=>{},()=>Date.now(),{blobs:false});
console.log(JSON.stringify({label:"apply with a 1 hour safety age (stands for: 24 h after the storm)",entries:later.entries,segments:later.segments,segmentBytes:later.segmentBytes,snapshots:later.snapshots,ignored:later.ignored,skipped:later.skipped}));
await fold("fold after the second compaction");
console.log(JSON.stringify({label:"status after",report:await storeReportV1(store,settings,Date.now())}));
