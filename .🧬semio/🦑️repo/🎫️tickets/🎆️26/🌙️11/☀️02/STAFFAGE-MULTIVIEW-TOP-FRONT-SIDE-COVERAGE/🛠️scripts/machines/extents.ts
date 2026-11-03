import { catalog, authoredScales, extentOf, queries } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
const mode = process.env.MODE ?? "default";
const f = (x:number)=>x.toFixed(2);
for (const a of catalog()) {
  if (!["machinery","vehicles"].includes(a.category)) continue;
  if (ids.length && !ids.includes(a.id)) continue;
  const d = a.dimensions;
  console.log(`## ${a.id} dims L${d.widthM} W${d.depthM} H${d.heightM}`);
  for (const s of authoredScales(a)) {
    const qs = mode==="default" ? [{name:"pose "+a.presentation.defaultPose, query:{kind:"pose",pose:a.presentation.defaultPose} as any}] : queries(a);
    for (const {name, query} of qs) {
      let st:any; try { st = evaluate(a, query, {kind:"scale",scale:{denominator:s}} as any); } catch(e){ console.log(`  1:${s} ${name} ERR ${(e as Error).message}`); continue;}
      const [e,fr,p] = [st.shapes.elevation, st.shapes.front, st.shapes.plan].map((x:any)=>extentOf(x));
      const r=(x:any)=>x?`x[${f(x.minX)},${f(x.maxX)}] y[${f(x.minY)},${f(x.maxY)}]`:"NONE";
      console.log(`  1:${s} ${name}\n     E ${r(e)}\n     F ${r(fr)}\n     P ${r(p)}`);
    }
  }
}
