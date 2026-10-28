/** 🔬 rustfast-g: wall and CPU of reading + sha256 of one file with async reads of N bytes. usage: bun chunk-bench.ts <file> <chunkBytes> [yieldEvery] */
import {open} from "node:fs/promises";
import {createHash} from "node:crypto";
const [path,chunk,every]=[process.argv[2]!,Number(process.argv[3]),Number(process.argv[4]??0)];
const t0=performance.now(),c0=process.cpuUsage();
const file=await open(path,"r"),hash=createHash("sha256"),buffer=Buffer.allocUnsafe(chunk);let offset=0,reads=0;
for(;;){const row=await file.read(buffer,0,chunk,offset);if(!row.bytesRead)break;hash.update(buffer.subarray(0,row.bytesRead));offset+=row.bytesRead;reads++;if(every&&reads%every===0)await new Promise(resolve=>setImmediate(resolve));}
await file.close();
const cpu=process.cpuUsage(c0);
console.log(JSON.stringify({chunk,reads,bytes:offset,wallMs:Math.round(performance.now()-t0),cpuMs:Math.round((cpu.user+cpu.system)/1000),sha:hash.digest("hex").slice(0,12)}));
