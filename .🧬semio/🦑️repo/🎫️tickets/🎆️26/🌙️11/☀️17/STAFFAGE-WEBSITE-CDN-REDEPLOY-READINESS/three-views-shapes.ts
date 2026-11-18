import { catalog, extentOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [id, scale, pose, ...views] = process.argv.slice(2);
const a = catalog().find((x) => x.id === id)!;
const st = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: +scale } } as any);
for (const v of views) { console.log("==", v); for (const s of st.shapes[v as "plan"]) { const e = extentOf([s], true); if (e && s.shape.role !== "ground") console.log(s.shape.id.padEnd(22), (s.shape as any).bone, [e.minX, e.maxX, e.minY, e.maxY].map((n) => n.toFixed(3)).join(" ")); } }
