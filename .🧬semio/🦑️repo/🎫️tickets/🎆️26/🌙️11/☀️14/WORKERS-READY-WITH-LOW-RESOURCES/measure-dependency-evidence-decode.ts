import { Buffer } from "node:buffer";
import { createHash } from "node:crypto";
import { decodeInstalledLedgerV1, decodeOwnershipIndexV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🟦️.ts";

const kind=process.argv[2],total=Number(process.argv[3]);
if(!["ownership","installed"].includes(kind)||!Number.isInteger(total)||total<2||total>32768)throw Error("Bounded private fixture required");
const evidence=(kind:string,total:number)=>{
 const stamp=["1","10","16877","2","160","12000000000","13000000000"];
 const chunks=[];
 for(let i=0;i<total;i++){
  const path=i===0?".":"node-"+String(i).padStart(6,"0");
  const raw=kind==="ownership"?["d",path,...stamp.slice(0,5),4,[["one","directory"],["two","file"],["three","file"],["four","symlink"]]]:["g",path,...stamp,1,"a".repeat(32),i===0?total-1:0,2,1];
  chunks.push(JSON.stringify(raw)+"\n");
 }
 return Buffer.from(chunks.join(""));
};
const bytes=evidence(kind!,total),sha256=createHash("sha256").update(bytes).digest("hex"),reference={bytes:bytes.length,sha256,rows:total,name:sha256};
Bun.gc(true);
const before=process.memoryUsage().heapUsed,rss=process.resourceUsage().maxRSS,start=performance.now();
const decoded=kind==="ownership"?decodeOwnershipIndexV1("/workspace",bytes,reference):decodeInstalledLedgerV1(bytes,reference);
const elapsed=performance.now()-start,usage=process.memoryUsage();
Bun.gc(true);
console.log(JSON.stringify({kind,rows:decoded.length,bytes:bytes.length,decodeMilliseconds:Math.round(elapsed*10)/10,heapBefore:before,heapAtReturn:usage.heapUsed,heapAfterCollection:process.memoryUsage().heapUsed,peakResidentBeforeKiB:rss,peakResidentAfterKiB:process.resourceUsage().maxRSS,last:decoded.at(-1)?.path}));
