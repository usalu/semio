import {readFileSync} from "node:fs";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const G=await import(B+"🟨️.mjs");
const {patchFactIndexBytesV1}=await import(B+"🌊️stream-patch/🟨️.mjs");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const view:any=G.openPackedFactIndexV1(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),512*1048576,"t");
const name=process.argv[2]??"@semio-tech/s-dev-composition";const oid=view.owners.indexOf(name);
const ids=new Set<number>();for(let c=0;c<view.classes.length;c++){const cls=view.classes[c];if(cls.o.includes(oid))for(const r of cls.r)ids.add(r);}
console.log("owner",name,"rows",ids.size);
const rows=[...ids].map(id=>{const r=view.rows[id];return {kind:r.kind,path:r.path,fact:structuredClone(r.fact)};});
let t=performance.now();
const out=await patchFactIndexBytesV1(view,{upserts:new Map([[name,rows]])},{maximumBytes:512*1048576});
console.log("patch ms",Math.round(performance.now()-t),"bytes",out.length);
