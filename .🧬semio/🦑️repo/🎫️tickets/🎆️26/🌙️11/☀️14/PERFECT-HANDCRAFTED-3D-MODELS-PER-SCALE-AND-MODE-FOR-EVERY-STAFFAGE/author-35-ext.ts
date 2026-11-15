import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const m = merged(asset, loadSource(entry)!) as any;
const sc = Number(process.argv[3]);
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } });
const list = (process.argv[5] ?? "body") as any;
for (const w of worldSolids(m, st.bones as never, sc, list)) { const q = w.positions; const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9]; for (let i = 0; i < q.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, q[i + k]!); mx[k] = Math.max(mx[k]!, q[i + k]!); } if (!process.argv[4] || w.id.startsWith(process.argv[4])) console.log(w.id.padEnd(26), mn.map((x) => x.toFixed(2).padStart(7)).join(" "), "|", mx.map((x) => x.toFixed(2).padStart(7)).join(" ")); }
