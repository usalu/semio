import {readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
const dir=process.env.HOME+"/Library/Caches/semio/host-capacity/metrics";
const [a,b]=[process.argv[2]!,process.argv[3]!];
const from=new Date("2026-11-27T"+a+"+01:00").getTime(),to=new Date("2026-11-27T"+b+"+01:00").getTime();
const ids=(process.argv[4]??"").split(",");
const s=readSeriesV1({directory:dir,fromMs:from,toMs:to,ids});
const rows=new Map<number,Record<string,string>>();
for(const id of ids)for(const p of s.metrics[id]??[]){const [start,width,tier,count,min,max,sum,last]=p;const r=rows.get(start)??{};r[id]=(id.includes("memory")||id.includes("swap.used")||id.includes("compressed")||id.includes("rss")?(max/1048576).toFixed(0)+"M":String(max))+(tier>0?`(t${tier})`:"");rows.set(start,r);}
for(const k of [...rows.keys()].sort())console.log(new Date(k).toTimeString().slice(0,8),ids.map(i=>i.split(".").slice(-2).join(".")+"="+(rows.get(k)![i]??"-")).join(" "));
