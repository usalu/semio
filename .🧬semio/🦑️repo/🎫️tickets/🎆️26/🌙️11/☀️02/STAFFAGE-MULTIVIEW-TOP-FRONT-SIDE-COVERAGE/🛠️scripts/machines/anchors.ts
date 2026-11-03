import { catalog, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const kinds = (process.env.KINDS ?? "emitter").split(",");
for (const a of catalog()) { if (!["machinery","vehicles"].includes(a.category)) continue;
  const st:any = evaluate(a, {kind:"pose",pose:a.presentation.defaultPose},{kind:"scale",scale:{denominator:100}} as any);
  for (const an of a.anchors.filter((x:any)=>kinds.includes(x.kind))) {
    const at = st.anchors[an.id]; if (!at) continue;
    const cands = st.shapes.plan.filter((s:any)=>{ const e=extentOf([s]); return e && at.at.x>=e.minX-0.05 && at.at.x<=e.maxX+0.05; }).map((s:any)=>{const e=extentOf([s])!; return `${s.shape.id}:z[${e.minY.toFixed(2)},${e.maxY.toFixed(2)}]`;});
    const near = cands.filter((c:string)=>/stack|tail|exhaust|vent|outlet|hvac|resistor|motor|louvre|grille|collar|port|flue|fan|glass|dome/i.test(c));
    console.log(`${a.id} ${an.id} at(${at.at.x.toFixed(2)},${at.at.y.toFixed(2)}) z=${at.z?.toFixed?.(2)} | ${near.slice(0,4).join(" ")}`);
  } }
