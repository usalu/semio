import{readFileSync}from"node:fs";const{all,apply}=await import(import.meta.dir+"/patches4.mjs");
for(const k of new Set(all.map(a=>a.file))){const o=apply(k,readFileSync("/Users/ueli/Documents/semio/"+k,"utf8"),all);console.log("ok",k.slice(-30),o.length);}
