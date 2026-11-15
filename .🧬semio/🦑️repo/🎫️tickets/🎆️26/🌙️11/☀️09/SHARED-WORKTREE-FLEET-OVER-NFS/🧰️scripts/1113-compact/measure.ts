import {readdir,readFile,stat} from "node:fs/promises";
import {createReadStream} from "node:fs";
import {createInterface} from "node:readline";
import {join} from "node:path";

const store=process.argv[2]??"/Users/Shared/semio-store/v1",scanBlobs=process.argv[3]!=="noblobs";
const origins=(await readdir(join(store,"origins"))).filter(n=>/^[a-z0-9][a-z0-9-]*$/.test(n)).sort();
const winner=new Map<string,{hlc:string;op:string;sha?:string}>();
const per:Record<string,{events:number;bytes:number;segments:number;files:string[]}>={};
const t0=Date.now();
async function lines(file:string,visit:(line:string)=>void){for await(const line of createInterface({input:createReadStream(file,{encoding:"utf8"}),crlfDelay:Infinity}))if(line!=="")visit(line);}
for(const origin of origins){
 const dir=join(store,"origins",origin,"segments"),names=(await readdir(dir)).filter(n=>/^[0-9]{12}\.jsonl$/.test(n)).sort();
 const entry={events:0,bytes:0,segments:names.length,files:names};per[origin]=entry;
 for(const name of names){
  const file=join(dir,name);entry.bytes+=(await stat(file)).size;
  await lines(file,line=>{
   const e=JSON.parse(line) as {hlc:string;path:string;operation:string;sha256?:string};entry.events++;
   const cur=winner.get(e.path);
   if(cur===undefined||e.hlc>cur.hlc)winner.set(e.path,{hlc:e.hlc,op:e.operation,sha:e.sha256});
  });
 }
}
let live=0,tomb=0,links=0;const referenced=new Set<string>();
for(const w of winner.values()){if(w.op==="delete")tomb++;else live++;if(w.op==="link")links++;if(w.op==="put")referenced.add(w.sha!);}
const rows:Record<string,{events:number;superseded:number;bytes:number;segments:number;supersededBytes:number}>={};
let total=0,sup=0;
for(const origin of origins){
 const dir=join(store,"origins",origin,"segments");let events=0,superseded=0,supBytes=0;
 for(const name of per[origin]!.files)await lines(join(dir,name),line=>{
  const e=JSON.parse(line) as {hlc:string;path:string};events++;
  if(winner.get(e.path)!.hlc!==e.hlc){superseded++;supBytes+=Buffer.byteLength(line)+1;}
 });
 rows[origin]={events,superseded,bytes:per[origin]!.bytes,segments:per[origin]!.segments,supersededBytes:supBytes};total+=events;sup+=superseded;
}
const out:Record<string,unknown>={origins:rows,totalEvents:total,superseded:sup,supersededShare:sup/total,paths:winner.size,livePaths:live,tombstones:tomb,links,referencedBlobs:referenced.size,foldSeconds:(Date.now()-t0)/1000};
const snapDir=join(store,"snapshots");const snaps:string[]=[];
for(const o of await readdir(snapDir).catch(()=>[] as string[]))for(const n of await readdir(join(snapDir,o)).catch(()=>[] as string[]))if(n.endsWith(".json")&&!n.startsWith("._"))snaps.push(`${o}/${n}`);
out.snapshots=snaps;
if(scanBlobs){
 let count=0,bytes=0,uCount=0,uBytes=0,uOld=0,uOldBytes=0,refBytes=0;const now=Date.now();const t1=Date.now();
 for(const p of await readdir(join(store,"blobs"))){
  if(!/^[0-9a-f]{2}$/.test(p))continue;
  for(const n of await readdir(join(store,"blobs",p))){
   if(!/^[0-9a-f]{64}$/.test(n))continue;
   const s=await stat(join(store,"blobs",p,n)).catch(()=>undefined);if(!s)continue;
   count++;bytes+=s.size;
   if(referenced.has(n))refBytes+=s.size;else{uCount++;uBytes+=s.size;if(now-s.mtimeMs>86400000){uOld++;uOldBytes+=s.size;}}
  }
 }
 Object.assign(out,{blobs:count,blobBytes:bytes,referencedBlobBytes:refBytes,unreferencedBlobs:uCount,unreferencedBytes:uBytes,unreferencedOlderThan1d:uOld,unreferencedOlderThan1dBytes:uOldBytes,blobScanSeconds:(Date.now()-t1)/1000});
}
console.log(JSON.stringify(out,null,1));
