import { writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, out] = process.argv.slice(2) as string[];
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!) as any;
const st = evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: 100 } }) as any;
const res: any = {};
for (const [k, b] of Object.entries(st.bones) as any) { const f = b.frame; res[k] = { o: [f.o.x, f.o.y, f.o.z], u: [f.u.x, f.u.y, f.u.z], v: [f.v.x, f.v.y, f.v.z], w: [f.w.x, f.w.y, f.w.z] }; console.log(k, JSON.stringify(res[k], (_, x) => typeof x === "number" ? Math.round(x * 1000) / 1000 : x)); }
writeFileSync(out!, JSON.stringify(res));
