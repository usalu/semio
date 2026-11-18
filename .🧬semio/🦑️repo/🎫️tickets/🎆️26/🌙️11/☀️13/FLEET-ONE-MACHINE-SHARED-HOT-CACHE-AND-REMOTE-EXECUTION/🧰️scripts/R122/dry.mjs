import{readFileSync}from"node:fs";const{all,apply}=await import(import.meta.dir+"/patches.mjs");
const R="/Users/ueli/Documents/semio/";const keys=[...new Set(all.map(a=>a.file))];
for(const k of keys){const out=apply(k,readFileSync(R+k,"utf8"),all);console.log("ok",k.slice(-44),out.length);}
