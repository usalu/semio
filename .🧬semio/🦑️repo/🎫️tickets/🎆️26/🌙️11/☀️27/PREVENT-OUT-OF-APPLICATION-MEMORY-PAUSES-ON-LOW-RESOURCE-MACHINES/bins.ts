import {readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
const dir=process.env.HOME+"/Library/Caches/semio/host-capacity/metrics";
const from=new Date("2026-11-25T00:00:00+01:00").getTime(),to=new Date("2026-11-27T18:06:00+01:00").getTime();
const ids=["host.memory.compressed","host.memory.pressure","host.swap.used","host.memory.available","host.swap.rate"];
const s=readSeriesV1({directory:dir,fromMs:from,toMs:to,ids});
const rows=new Map<number,Record<string,number>>();
for(const id of ids)for(const p of s.metrics[id]??[]){if(p[1]>60000)continue;const r=rows.get(p[0])??{};r.w=p[1];r[id]=p[5];rows.set(p[0],r);}
console.log("rows",rows.size,"width",[...rows.values()][0]?.w);
const GB=1073741824;const bins=new Map<number,{n:number;warn:number;swapMax:number;availMin:number}>();
for(const r of rows.values()){const c=r["host.memory.compressed"];if(c===undefined)continue;const b=Math.floor(c/GB);const e=bins.get(b)??{n:0,warn:0,swapMax:0,availMin:1e18};e.n++;if((r["host.memory.pressure"]??0)>=1)e.warn++;e.swapMax=Math.max(e.swapMax,r["host.swap.used"]??0);e.availMin=Math.min(e.availMin,r["host.memory.available"]??1e18);bins.set(b,e);}
for(const b of [...bins.keys()].sort((a,b)=>a-b)){const e=bins.get(b)!;console.log(`compressed ${b}-${b+1} GB: buckets=${e.n} pressure>=warn=${e.warn} (${(100*e.warn/e.n).toFixed(0)}%) swapUsedMax=${(e.swapMax/GB).toFixed(2)}G availMin=${(e.availMin/GB).toFixed(2)}G`);}
