// [asm-g] law on the REAL index: openFactIndexV1 rows equal decodeFactIndexV1 rows for a sample and for every read id; resident size comparison. usage: bun open-vs-decode.mjs <gen-dir>
import{readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const D=process.argv[2],B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/",c=await import(B+"🧮️fact-codec/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const raw=brotliDecompressSync(readFileSync(join(D,"x",manifest.factIndex+".facts.br")));
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
let t=performance.now();const lazy=c.openFactIndexV1(raw);const openMs=Math.round(performance.now()-t);Bun.gc(true);const r1=rss();
console.log("open ms",openMs,"rows",lazy.rows.length,"rss",r1,"read",lazy.rowsRead());
const ids=[];for(let i=0;i<lazy.rows.length;i+=7)ids.push(i);for(const id of ids)lazy.rows[id];
console.log("read 1/7 ->",lazy.rowsRead(),"rss",rss());
t=performance.now();const eager=c.decodeFactIndexV1(raw);console.log("eager ms",Math.round(performance.now()-t),"rss",rss());
let bad=0;for(const id of ids){if(JSON.stringify(lazy.rows[id])!==JSON.stringify(eager.rows[id]))bad++;}
console.log("row mismatches",bad,"owners equal",JSON.stringify(lazy.owners)===JSON.stringify(eager.owners),"classes equal",JSON.stringify(lazy.classes)===JSON.stringify(eager.classes));
let kindBad=0;for(let id=0;id<eager.rows.length;id+=3)if(lazy.rowKindOf(id)!==eager.rows[id].kind)kindBad++;console.log("kind mismatches",kindBad);
