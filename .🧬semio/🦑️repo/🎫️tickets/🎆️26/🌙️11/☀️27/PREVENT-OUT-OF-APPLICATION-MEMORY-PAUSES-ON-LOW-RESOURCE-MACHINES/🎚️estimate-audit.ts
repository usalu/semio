/** 🎚️ Audit of the learned classes of estimates.json (MEM-ADMIT): per class with absolute peaks, the estimate the store gives (decayed quantile x margin) against the median and the maximum of its own peaks and the shares of runs that stayed under it. `bun 🎚️estimate-audit.ts` prints a table and the summary. */
import {readFileSync} from "node:fs";
import buildPolicy from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {measuredHostPeakV1,hostSampleQuantileV1,hostSampleWeightsV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts";
import {readHostCapacityPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),store=JSON.parse(readFileSync(process.env.HOME+"/Library/Caches/semio/host-capacity/estimates.json","utf8")),now=Date.now();
const rows:{name:string;n:number;median:number;max:number;estimate:number;over:number;shareUnder:number;ratioTime:number}[]=[];
for(const [name,value] of Object.entries(store.classes) as [string,{samples:any[]}][]){
 const peaks=value.samples.filter(sample=>sample.peakMegabytes!==undefined).map(sample=>sample.peakMegabytes as number).sort((a,b)=>a-b);
 if(peaks.length<3)continue;
 const estimate=measuredHostPeakV1(policy,store,[name],now);if(estimate===undefined)continue;
 const median=peaks[Math.floor(peaks.length/2)]!,max=peaks[peaks.length-1]!;
 const active=value.samples.map(sample=>sample.activeMilliseconds as number).sort((a,b)=>a-b)[Math.floor(value.samples.length/2)]!;
 rows.push({name:name.slice(0,60),n:peaks.length,median,max,estimate,over:+(estimate/median).toFixed(2),shareUnder:+(peaks.filter(peak=>peak<=estimate).length/peaks.length).toFixed(2),ratioTime:Math.round(active/1000)});
}
console.table(rows);
const overs=rows.map(row=>row.over).sort((a,b)=>a-b),mean=overs.reduce((s,v)=>s+v,0)/overs.length;
console.log(JSON.stringify({classes:rows.length,medianOverMedianPeak:overs[Math.floor(overs.length/2)],meanOverMedianPeak:+mean.toFixed(2),classesAbove1_5xMedian:rows.filter(row=>row.over>1.5).length,classesWhereMaxExceedsEstimate:rows.filter(row=>row.max>row.estimate).length,policy:{quantile:policy.estimates.quantileBasisPoints,margin:policy.estimates.marginBasisPoints,minimumSamples:policy.estimates.minimumSamples,maximumSamples:policy.estimates.maximumSamples,halfLifeDays:policy.estimates.halfLifeMilliseconds/86400000}}));
