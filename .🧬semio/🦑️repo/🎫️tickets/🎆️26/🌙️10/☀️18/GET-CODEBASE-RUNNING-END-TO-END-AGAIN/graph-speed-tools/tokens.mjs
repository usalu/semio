import {readdirSync,readFileSync,statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const dir="/Users/ueli/Documents/semio/.cache/b/g/m/inference/np";
const cl="/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures";
const files=readdirSync(dir).filter(f=>f.endsWith(".json")).sort();
let n=0,totalTok=0,totalExpanded=0,totalImage=0,distinct=new Map(),projects=0,maxTok=0;
const step=1;
for(let i=0;i<files.length;i+=step){
 const f=files[i];let text;try{text=brotliDecompressSync(readFileSync(dir+"/"+f)).toString("utf8");}catch{continue;}
 const nl=text.indexOf("\n");const image=text.slice(nl+1);totalImage+=image.length;globalThis.hdr=(globalThis.hdr||0)+nl;projects++;
 const toks=[...image.matchAll(/SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})/g)];
 totalTok+=toks.length;maxTok=Math.max(maxTok,toks.length);
 for(const m of toks){
  const key=m[0];let size=distinct.get(key);
  if(size===undefined){let c=0;try{const p=(m[1]==="SOURCE_CLOSURE"?cl:cl.replace("input","lock"))+"/"+m[2]+".json";const v=JSON.parse(readFileSync(p,"utf8"));c=m[1]==="SOURCE_CLOSURE"?(v.files?.length??Object.keys(v).length):2;}catch(e){c=-1}
   size=c;distinct.set(key,size);}
  totalExpanded+=Math.max(0,size);
 }
}
console.log(JSON.stringify({sampled:projects,of:files.length,avgImageBytes:totalImage/projects,tokensPerProject:totalTok/projects,maxTok,distinctTokens:distinct.size,avgExpandedEntriesPerProject:totalExpanded/projects}));

console.log("header bytes total",globalThis.hdr,"image total",totalImage,"expanded entries total",totalExpanded);
