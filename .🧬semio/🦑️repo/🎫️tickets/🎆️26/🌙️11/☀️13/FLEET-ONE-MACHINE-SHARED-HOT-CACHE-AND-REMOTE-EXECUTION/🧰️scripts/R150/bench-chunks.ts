import {withArtifactRelativeDirectoryV1} from "@semio-tech/framework-filesystem/observation/descriptor";
import {copyProtectedArtifactInputV1,protectedArtifactInputChunkV1,readProtectedArtifactInputV1} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🧮️read-plan/🟦️.ts";
import {mkdtempSync,rmSync} from "node:fs";
import {freemem} from "node:os";
import {join} from "node:path";
import {createHash,randomBytes} from "node:crypto";
const MIB=1048576,total=Number(process.env.MB??"165")*MIB,fileBytes=8*MIB,files=Math.ceil(total/fileBytes);
const buffer=randomBytes(fileBytes),data=buffer.toString("base64"),sha=createHash("sha256").update(buffer).digest("hex");
const base=mkdtempSync(join(process.env.BENCH_DIR!,"c-"));
for(const [name,chunk] of [["old64k",{encoded:65536,decoded:49152}],["derived",protectedArtifactInputChunkV1(total,freemem())]] as const){
 const calls:Record<string,number>={};let checkpoints=0,steps=0;const bump=(k:string)=>{calls[k]=(calls[k]??0)+1;};
 const ports={check(){},async step(){steps++;}};
 const t0=performance.now();
 await withArtifactRelativeDirectoryV1(base,()=>{checkpoints++;},async root=>{
  for(let f=0;f<files;f++)await root.withNewFile(name+f,async file=>{bump("withNewFile");await copyProtectedArtifactInputV1({write:(...a)=>{bump("write");return file.write(...a);}},data,chunk,ports);bump("sync");file.sync();});
  await root.sync();
  const t1=performance.now();
  for(let f=0;f<files;f++)await root.withFile(name+f,async file=>{bump("withFile");const h=createHash("sha256");await readProtectedArtifactInputV1({read:(...a)=>{bump("read");return file.read(...a);}},fileBytes,chunk,p=>h.update(p),ports);if(h.digest("hex")!==sha)throw Error("digest");});
 });
 const ms=performance.now()-t0,mb=files*8,callCount=Object.values(calls).reduce((a,b)=>a+b,0);
 console.log(JSON.stringify({name,MB:mb,chunkDecoded:chunk.decoded,steps,ms:Math.round(ms),msPerStep:+(ms/steps).toFixed(3),MBpsWriteAndRead:+(mb*2/(ms/1000)).toFixed(0),checkpoints,checkpointsPerMB:+(checkpoints/(mb*2)).toFixed(2),interfaceCallsPerMB:+(callCount/(mb*2)).toFixed(2),calls}));
}
rmSync(base,{recursive:true,force:true});
