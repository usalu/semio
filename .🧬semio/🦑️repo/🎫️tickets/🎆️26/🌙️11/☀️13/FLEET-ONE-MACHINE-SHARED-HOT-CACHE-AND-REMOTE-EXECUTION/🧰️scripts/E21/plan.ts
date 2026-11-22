import {planRemoteCacheCollectionV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧹️collector/🟦️.ts";
import {classifySlotsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪢️slots/🟦️.ts";
import {randomBytes} from "node:crypto";
const n=200000,now=Date.now(),rows=[],metas=new Map<string,any>(),slots=Array.from({length:50000},()=>randomBytes(32).toString("hex"));
for(let i=0;i<n;i++){const key="main/ac/xx/"+randomBytes(32).toString("hex");rows.push({key,size:300,accessed:now-i*10});metas.set(key,{version:1,slot:slots[i%slots.length],storedAt:now-i*10,refs:[randomBytes(32).toString("hex")]});}
let t=performance.now();const plan=planRemoteCacheCollectionV1(rows,{maximumBytes:n*300/2,maximumAgeMilliseconds:30*86400000,now,need:0,reserved:0},new Set(),{metas,graceMilliseconds:600000,claimed:()=>false});
console.log("plan",(performance.now()-t).toFixed(0),"ms for",n,"entries; drops",plan.length);
t=performance.now();classifySlotsV1([...metas].map(([key,meta])=>({key,slot:meta.slot,storedAt:meta.storedAt,accessed:0,size:0})),{now,graceMilliseconds:600000});console.log("classify",(performance.now()-t).toFixed(0),"ms");
