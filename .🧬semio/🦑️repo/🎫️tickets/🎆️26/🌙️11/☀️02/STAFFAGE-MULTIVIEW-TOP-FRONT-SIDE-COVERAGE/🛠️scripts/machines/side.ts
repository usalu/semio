import { catalog, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const re = new RegExp(process.env.RE ?? "door|slider|mirror|exhaust|stack|steer|seat|cab|fuel|hatch|bonnet|chute|ladder|cowl|stand");
for (const a of catalog()) { if (!["machinery","vehicles"].includes(a.category)) continue;
  if (process.env.IDS && !process.env.IDS.split(",").includes(a.id)) continue;
  const s = Number(process.env.SCALE ?? 100);
  let st:any; try { st = evaluate(a,{kind:"pose",pose:a.presentation.defaultPose},{kind:"scale",scale:{denominator:s}} as any);} catch { continue; }
  const out:string[]=[];
  for (const sh of st.shapes.plan) { if (!re.test(sh.shape.id)) continue; const e=extentOf([sh]); if(e) out.push(`${sh.shape.id}:z[${e.minY.toFixed(2)},${e.maxY.toFixed(2)}]`); }
  console.log(a.id, out.join(" "));
}
