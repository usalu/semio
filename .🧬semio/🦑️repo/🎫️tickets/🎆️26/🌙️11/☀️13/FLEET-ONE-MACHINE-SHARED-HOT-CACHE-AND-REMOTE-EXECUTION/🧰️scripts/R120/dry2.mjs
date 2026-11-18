import{readFileSync}from"node:fs";import{all,apply}from"./patches.mjs";
const R="/Users/ueli/Documents/semio/";
for(const k of new Set(all.map(a=>a.file))){const n=apply(k,readFileSync(R+k,"utf8"),all);const loader=k.endsWith(".ts")?"ts":k.endsWith(".mjs")?"js":k.endsWith(".json")?"json":null;if(!loader)continue;try{if(loader==="json")JSON.parse(n);else new Bun.Transpiler({loader}).transformSync(n);console.log("ok",k.slice(-30));}catch(e){console.log("FAIL",k.slice(-30),String(e).slice(0,300));}}
