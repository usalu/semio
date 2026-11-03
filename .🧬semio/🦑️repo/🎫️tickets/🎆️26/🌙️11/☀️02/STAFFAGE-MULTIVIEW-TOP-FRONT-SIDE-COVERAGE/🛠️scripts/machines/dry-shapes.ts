import { readFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { convert } from "./solidify.ts";
import { extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [id, pose, scale, ...views] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(fileOf(id), "utf8"));
if (process.env.RAW !== "1") convert(doc.asset, []);
const st: any = evaluate(doc.asset, process.env.CLIP ? { kind: "clip", clip: pose, timeS: Number(process.env.T) } : { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(scale) } } as any);
const f = (n: number) => n.toFixed(3);
for (const v of views) { console.log("--", v); for (const sh of st.shapes[v]) { const e = extentOf([sh], true); if (e) console.log(`  ${sh.shape.id} [${sh.shape.bone}] x[${f(e.minX)},${f(e.maxX)}] y[${f(e.minY)},${f(e.maxY)}]`); } }
