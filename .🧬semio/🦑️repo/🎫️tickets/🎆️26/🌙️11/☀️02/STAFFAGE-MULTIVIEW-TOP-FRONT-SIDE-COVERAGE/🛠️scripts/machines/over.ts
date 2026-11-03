import { catalog, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const id = process.env.ID!, s = Number(process.env.SCALE), lim = Number(process.env.LIM);
const a = catalog().find(x=>x.id===id)!;
const st:any = evaluate(a,{kind:"pose",pose:process.env.POSE??a.presentation.defaultPose},{kind:"scale",scale:{denominator:s}} as any);
for (const v of ["front","plan"]) for (const sh of st.shapes[v]) { const e=extentOf([sh]); if(!e) continue; const [lo,hi]= v==="front"?[e.minX,e.maxX]:[e.minY,e.maxY]; if (Math.abs(lo)>lim||Math.abs(hi)>lim) console.log(v, sh.shape.id, sh.shape.bone, lo.toFixed(3), hi.toFixed(3)); }
