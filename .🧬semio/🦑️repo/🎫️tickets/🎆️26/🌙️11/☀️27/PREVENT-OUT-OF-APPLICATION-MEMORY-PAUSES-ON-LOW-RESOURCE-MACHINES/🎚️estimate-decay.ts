/** 🎚️ How much trust the learned classes of estimates.json still earn after their decay (MEM-ADMIT): per class the age of its newest sample, the effective samples, the raw ratio (quantile x margin of its samples) and the ratio the store answers, which is blended with the 100 % default by the trust the effective samples earn. */
import {readFileSync} from "node:fs";
import buildPolicy from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {estimateHostRatioV1,hostSampleQuantileV1,hostSampleWeightsV1,robustHostSamplesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts";
import {readHostCapacityPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),store=JSON.parse(readFileSync(process.env.HOME+"/Library/Caches/semio/host-capacity/estimates.json","utf8")),now=Number(process.env.NOW??Date.now());
const rows=Object.entries(store.classes as Record<string,{samples:any[]}>).map(([name,value])=>{
 const {samples,weight}=robustHostSamplesV1(policy,value.samples,now),effective=weight.reduce((s:number,v:number)=>s+v,0)/1e6;
 const raw=samples.length===0?null:Math.round(hostSampleQuantileV1(samples,weight,"ratioBasisPoints",policy.estimates.quantileBasisPoints)*(10000+policy.estimates.marginBasisPoints)/10000);
 const answer=estimateHostRatioV1(policy,store,name.split(":").reduce<string[]>((rows,part)=>[...rows,rows.length===0?part:rows[rows.length-1]+":"+part],[]).filter(k=>store.classes[k]!==undefined),now);
 return {name:name.slice(0,58),samples:value.samples.length,newestHoursAgo:Math.round((now-Math.max(...value.samples.map((s:any)=>s.atMs)))/3600000),effective:+effective.toFixed(2),rawRatioPct:raw===null?null:Math.round(raw/100),storeRatioPct:Math.round(answer.ratioBasisPoints/100)};
});
console.table(rows);
const old=rows.filter(row=>row.effective<policy.estimates.minimumSamples);
console.log(JSON.stringify({classes:rows.length,belowMinimumTrust:old.length,ofThoseRawBelowStore:old.filter(row=>row.rawRatioPct!==null&&row.rawRatioPct<row.storeRatioPct).length,halfLifeHours:policy.estimates.halfLifeMilliseconds/3600000}));
