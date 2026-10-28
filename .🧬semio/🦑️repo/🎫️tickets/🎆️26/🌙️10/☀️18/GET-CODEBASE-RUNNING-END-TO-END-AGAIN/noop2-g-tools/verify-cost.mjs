import {lstatSync,readdirSync} from "node:fs";
import {join} from "node:path";
const {verifyWorldWitnessV1}=await import(process.argv[2]+"/🔭️witness/🟨️.mjs");
const files=[];const walk=d=>{for(const e of readdirSync(d,{withFileTypes:true})){const p=join(d,e.name);if(e.isDirectory())walk(p);else files.push(p);if(files.length>=40000)return;}};walk("node_modules");
const root=process.cwd();
const rows=files.map(f=>{const p=join(root,f),s=lstatSync(p);return ["l",p,[s.dev,s.ino,s.mode,s.size,s.mtimeMs,s.ctimeMs].join(":")]});
console.log(rows.length,"rows; RSS MB",Math.round(process.memoryUsage().rss/1e6));
for(const concurrency of[16,64,128,256]){const t=performance.now(),cpu=process.cpuUsage();const v=await verifyWorldWitnessV1({observations:rows},{concurrency});console.log("concurrency",concurrency,JSON.stringify(v),Math.round(performance.now()-t),"ms cpu",Math.round((process.cpuUsage(cpu).user+process.cpuUsage(cpu).system)/1000),"ms");}
