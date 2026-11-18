import{readFileSync}from"node:fs";const{all,apply}=await import(import.meta.dir+"/patches.mjs");const R="/Users/ueli/Documents/semio/";
for(const k of new Set(all.map(a=>a.file))){const t=apply(k,readFileSync(R+k,"utf8"),all);console.log(k.slice(-48),readFileSync(R+k,"utf8").length,"->",t.length);}
