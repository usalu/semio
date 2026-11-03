import { catalog, extentOf, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const id = process.env.ID!, bone = process.env.BONE!, base = process.env.POSE!, deg = Number(process.env.DEG ?? 90);
const a:any = structuredClone(catalog().find(x=>x.id===id)!);
const p = a.poses.find((x:any)=>x.id===base); p.id="synthetic-twist"; p.bones = {...p.bones, [bone]: {...(p.bones[bone]??{}), twistDeg: deg}};
a.poses.push(p);
const f=(x:number)=>x.toFixed(2);
if (process.env.LIST) { const st:any = evaluate(a,{kind:"pose",pose:"synthetic-twist"},{kind:"scale",scale:{denominator:200}} as any); for (const v of ["front","plan"]) for (const sh of st.shapes[v]) { if(!new RegExp(process.env.LIST!).test(sh.shape.id)) continue; const e=extentOf([sh],true)!; console.log(v, sh.shape.id, `x[${f(e.minX)},${f(e.maxX)}] y[${f(e.minY)},${f(e.maxY)}]`);} }
for (const s of authoredScales(a)) {
  const st:any = evaluate(a,{kind:"pose",pose:"synthetic-twist"},{kind:"scale",scale:{denominator:s}} as any);
  const r=(v:string)=>{const e=extentOf(st.shapes[v]); return e?`x[${f(e.minX)},${f(e.maxX)}] y[${f(e.minY)},${f(e.maxY)}]`:"NONE"};
  console.log(`1:${s} E ${r("elevation")} | F ${r("front")} | P ${r("plan")}`);
}
