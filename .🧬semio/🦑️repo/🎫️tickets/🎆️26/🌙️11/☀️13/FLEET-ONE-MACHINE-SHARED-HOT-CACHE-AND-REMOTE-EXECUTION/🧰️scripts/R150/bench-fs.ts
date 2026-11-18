import {withArtifactRelativeDirectoryV1} from "@semio-tech/framework-filesystem/observation/descriptor";
import {mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {createHash,randomBytes} from "node:crypto";
const mb=Number(process.env.MB??"64"),chunk=Number(process.env.CHUNK??"49152"),fileBytes=8*1024*1024;
const base=mkdtempSync(join(process.env.BENCH_DIR!,"b-"));
const data=randomBytes(fileBytes);
let checkpoints=0;const counts:Record<string,number>={write:0,sync:0,stat:0};
const t0=performance.now();let chunks=0;
await withArtifactRelativeDirectoryV1(base,()=>{checkpoints++;},async root=>{
 const files=Math.ceil(mb*1024*1024/fileBytes);
 for(let f=0;f<files;f++){
  await root.withNewFile("f"+f,async file=>{
   let pos=0;
   for(let o=0;o<fileBytes;o+=chunk){const n=Math.min(chunk,fileBytes-o);const w=file.write(data,o,n,pos);counts.write++;pos+=w;chunks++;}
   file.sync();counts.sync++;
  });
 }
 await root.sync();
});
const ms=performance.now()-t0;
console.log(JSON.stringify({mb,chunk,chunks,ms:Math.round(ms),msPerChunk:+(ms/chunks).toFixed(3),checkpointsPerChunk:+(checkpoints/chunks).toFixed(1),MBps:+(mb/(ms/1000)).toFixed(1),counts}));
rmSync(base,{recursive:true,force:true});
