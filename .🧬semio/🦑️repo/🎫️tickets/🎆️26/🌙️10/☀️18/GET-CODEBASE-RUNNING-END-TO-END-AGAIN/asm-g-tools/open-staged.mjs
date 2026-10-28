// [asm-g] law + cost of the staged lazy opener on the real index: usage bun open-staged.mjs <gen-dir> <codec module path>
import{readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const D=process.argv[2],c=await import(process.argv[3]);
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const raw=brotliDecompressSync(readFileSync(join(D,"x",manifest.factIndex+".facts.br")));
const cpu=()=>{const u=process.cpuUsage();return Math.round((u.user+u.system)/1000);};
let c0=cpu();const lazy=c.openFactIndexV1(raw);console.log("open cpu",cpu()-c0);
c0=cpu();let n=0;for(let id=0;id<lazy.rows.length;id+=3){lazy.rows[id];n++;}console.log("read",n,"rows cpu",cpu()-c0);
c0=cpu();const eager=c.decodeFactIndexV1(raw);console.log("eager cpu",cpu()-c0);
let bad=0;for(let id=0;id<eager.rows.length;id+=3)if(JSON.stringify(lazy.rows[id])!==JSON.stringify(eager.rows[id]))bad++;console.log("mismatches",bad);
