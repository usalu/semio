import {readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
const dir=process.env.HOME+"/Library/Caches/semio/host-capacity/metrics";
const day="2026-11-27";
const from=new Date(day+"T16:30:00+01:00").getTime(),to=new Date(day+"T18:06:00+01:00").getTime();
const s=readSeriesV1({directory:dir,fromMs:from,toMs:to});
const ids=Object.keys(s.metrics);
console.error(ids.join(","));
const by=new Map<number,Record<string,number>>();
for(const id of ids)for(const p of s.metrics[id]!){const [start,width,tier,count,min,max,sum,last]=p;const k=Math.floor(start/60000)*60000;const r=by.get(k)??{};const prev=r[id+".min"];r[id+".min"]=prev===undefined?min:Math.min(prev,min);r[id+".max"]=Math.max(r[id+".max"]??-1e18,max);r[id+".last"]=last;by.set(k,r);}
const cols=(process.env.COLS??"").split(",");
for(const k of [...by.keys()].sort()){const r=by.get(k)!;const t=new Date(k).toTimeString().slice(0,5);console.log(t,cols.map(c=>c+"="+(r[c]??"-")).join(" "));}
