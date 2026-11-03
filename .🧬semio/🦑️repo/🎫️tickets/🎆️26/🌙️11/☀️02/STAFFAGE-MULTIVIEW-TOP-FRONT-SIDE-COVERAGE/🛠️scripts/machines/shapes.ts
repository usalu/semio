import { catalog, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const id = process.env.ID!, s = Number(process.env.SCALE ?? 200), pose = process.env.POSE;
const a = catalog().find(x=>x.id===id)!;
const q:any = process.env.CLIP ? {kind:"clip",clip:process.env.CLIP,timeS:Number(process.env.TIME??0)} : {kind:"pose",pose:pose??a.presentation.defaultPose};
const st:any = evaluate(a,q,{kind:"scale",scale:{denominator:s}} as any);
const f=(x:number)=>x.toFixed(2);
for (const v of (process.env.VIEWS??"elevation,front,plan").split(",")) {
  console.log("--",v);
  for (const sh of st.shapes[v]) { const e=extentOf([sh],true); if(e) console.log(`  ${sh.shape.id} [${sh.shape.bone}/${sh.shape.role}${sh.shape.region?"/r:"+sh.shape.region:""}] x[${f(e.minX)},${f(e.maxX)}] y[${f(e.minY)},${f(e.maxY)}]`); }
}
