// ⏱️ Sync vs libuv-pool brotli on the persisted np records: main-thread blocked time and wall (read-only).
import {readdirSync,readFileSync} from "node:fs";import {readFile} from "node:fs/promises";import {brotliDecompressSync,brotliDecompress,brotliCompressSync,brotliCompress,constants} from "node:zlib";import {join} from "node:path";
const dir=process.argv[2],limit=Number(process.argv[3]??"80");const files=readdirSync(dir).filter(f=>f.endsWith(".json")).slice(0,limit).map(f=>join(dir,f));
const main=()=>{const u=process.cpuUsage();return (u.user+u.system)/1e3;};
let t=performance.now(),c=main();const texts=[];for(const f of files)texts.push(brotliDecompressSync(readFileSync(f)));
console.log("decompress sync",JSON.stringify({files:files.length,mb:+(texts.reduce((a,b)=>a+b.length,0)/1e6).toFixed(1),wallMs:Math.round(performance.now()-t),cpuMs:Math.round(main()-c)}));
t=performance.now();c=main();let next=0;await Promise.all(Array.from({length:8},async()=>{while(next<files.length){const f=files[next++];await new Promise((a,r)=>brotliDecompress(readFileSync(f),(e,v)=>e?r(e):a(v)));}}));
console.log("decompress async x8",JSON.stringify({wallMs:Math.round(performance.now()-t),cpuMs:Math.round(main()-c)}));
const opts=b=>({params:{[constants.BROTLI_PARAM_QUALITY]:5,[constants.BROTLI_PARAM_LGWIN]:22,[constants.BROTLI_PARAM_SIZE_HINT]:b.length}});
t=performance.now();c=main();for(const b of texts)brotliCompressSync(b,opts(b));
console.log("compress q5 sync",JSON.stringify({wallMs:Math.round(performance.now()-t),cpuMs:Math.round(main()-c)}));
t=performance.now();c=main();next=0;await Promise.all(Array.from({length:8},async()=>{while(next<texts.length){const b=texts[next++];await new Promise((a,r)=>brotliCompress(b,opts(b),(e,v)=>e?r(e):a(v)));}}));
console.log("compress q5 async x8",JSON.stringify({wallMs:Math.round(performance.now()-t),cpuMs:Math.round(main()-c)}));
