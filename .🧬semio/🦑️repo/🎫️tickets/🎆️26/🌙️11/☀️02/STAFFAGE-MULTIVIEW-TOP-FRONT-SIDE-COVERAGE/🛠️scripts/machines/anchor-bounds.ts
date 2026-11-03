import { catalog, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
for (const a of catalog()) { if (!["machinery","vehicles"].includes(a.category)) continue;
  for (const pose of [a.presentation.defaultPose, ...a.poses.map(p=>p.id)].filter((v,i,l)=>l.indexOf(v)===i)) {
    const st:any = evaluate(a,{kind:"pose",pose},{kind:"scale",scale:{denominator:100}} as any);
    const e = extentOf(st.shapes.elevation)!; const m=0.5;
    for (const an of a.anchors) { const at=st.anchors[an.id]?.at; if(!at) continue;
      if (at.x<e.minX-m||at.x>e.maxX+m||at.y<e.minY-m||at.y>e.maxY+m) console.log(`${a.id} pose ${pose}: anchor ${an.id}@${an.bone} at (${at.x.toFixed(2)},${at.y.toFixed(2)}) outside [${e.minX.toFixed(1)},${e.maxX.toFixed(1)}]x[${e.minY.toFixed(1)},${e.maxY.toFixed(1)}]`); } } }
