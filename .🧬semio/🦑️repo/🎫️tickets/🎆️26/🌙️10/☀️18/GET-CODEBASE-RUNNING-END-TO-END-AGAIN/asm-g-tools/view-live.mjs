// [asm-g] live heap of the lazy fact index: after open, after reading every 3rd row (about the rows of a 550-project closure).
import{readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";import{heapStats}from"bun:jsc";
const D=process.argv[2],B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/",c=await import(B+"🧮️fact-codec/🟨️.mjs"),mem=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const packed=readFileSync(join(D,"x",manifest.factIndex+".facts.br"));
const live=()=>{Bun.gc(true);Bun.gc(true);const h=heapStats();return Math.round((h.heapSize+h.extraMemorySize)/1048576);};
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
console.log("base live",live(),"rss",rss());
let raw=brotliDecompressSync(packed);console.log("raw MB",Math.round(raw.length/1048576),"live",live());
const share=process.env.SHARE==="1"?(t=>mem.graphStringInternerV1.share(t)):undefined;
const view=c.openFactIndexV1(raw,{share});raw=null;console.log("opened live",live(),"rss",rss());
const every=Number(process.env.EVERY??"3");let n=0;for(let id=0;id<view.rows.length;id+=every){view.rows[id];n++;}
console.log("read",n,"live",live(),"rss",rss());
let sr=0,so=0,mx=0;for(const k of view.classes){sr+=k.r.length;so+=k.o.length;mx=Math.max(mx,k.o.length);}
console.log("classes",view.classes.length,"sum r",sr,"sum o",so,"max o",mx,"owners",view.owners.length);
