import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { insideAny } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const m = merged(asset, loadSource(entry)!) as any;
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(process.argv[3]) } });
const body = worldSolids(m, st.bones as never, Number(process.argv[3]), "body");
for (const w of body) { const q = w.positions; let mn=[1e9,1e9,1e9], mx=[-1e9,-1e9,-1e9]; for (let i=0;i<q.length;i+=3) for (let k=0;k<3;k++){mn[k]=Math.min(mn[k],q[i+k]);mx[k]=Math.max(mx[k],q[i+k]);} if (w.id.startsWith(process.argv[4]!)) console.log(w.id, mn.map(x=>x.toFixed(3)), mx.map(x=>x.toFixed(3))); }
for (const w of body) if (w.id.startsWith(process.argv[4]!)) console.log(w.id, w.positions.length/3, w.indices.length/3);
for (const w of body) if (w.id.startsWith(process.argv[4]!)) { const q = w.positions; for (let i = 0; i < Math.min(q.length, 72); i += 3) console.log(q[i]!.toFixed(3), q[i + 1]!.toFixed(3), q[i + 2]!.toFixed(3)); }
