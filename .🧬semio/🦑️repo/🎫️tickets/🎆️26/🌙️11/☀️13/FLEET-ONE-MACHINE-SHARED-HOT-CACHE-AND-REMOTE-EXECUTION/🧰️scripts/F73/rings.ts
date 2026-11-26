import {readSeriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
const [origin,from,to]=process.argv.slice(2);
const f=Date.parse(from),t=Date.parse(to);
const s=readSeriesV1({directory:`/Users/Shared/semio-store/v1/origins/${origin}/metrics`,fromMs:f,toMs:t,ids:["host.memory.available","host.swap.used","host.swap.rate","host.memory.pressure","warden.level","warden.cancelled","warden.subjects.memory","graph.pool.memory","graph.pool.failed","process.repo.memory","process.ready.rss","ready.state"]});
const rows=new Map<number,Record<string,string>>();
for(const[id,pts] of Object.entries(s.metrics))for(const p of pts){const r=rows.get(p[0])??{};r[id]=`${Math.round(p[4])}..${Math.round(p[5])}`;rows.set(p[0],r);}
for(const k of [...rows.keys()].sort())console.log(new Date(k).toISOString().slice(11,19),JSON.stringify(rows.get(k)));
