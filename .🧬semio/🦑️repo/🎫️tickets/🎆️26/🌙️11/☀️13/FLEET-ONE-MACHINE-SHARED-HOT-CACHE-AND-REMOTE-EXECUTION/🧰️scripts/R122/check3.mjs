import{readFileSync}from"node:fs";const{all,apply}=await import(import.meta.dir+"/patches2.mjs");
const k=all.map(a=>a.file).find(f=>f.endsWith("🏊️pool/🧬️schema/🔣️.json"));
const j=JSON.parse(apply(k,readFileSync("/Users/ueli/Documents/semio/"+k,"utf8"),all));
console.log(j.$defs.Messages.const.en.holding,"|",j.$defs.Messages.const.de.holding);
