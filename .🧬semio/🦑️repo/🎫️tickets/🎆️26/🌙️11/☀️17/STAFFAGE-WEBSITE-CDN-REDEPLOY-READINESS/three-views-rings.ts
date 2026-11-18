import { catalog } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [id, scale, pose, view, shape] = process.argv.slice(2);
const a = catalog().find((x) => x.id === id)!;
const st = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: +scale } } as any);
const s = st.shapes[view as "plan"].find((x) => x.shape.id === shape)!;
console.log(JSON.stringify(s.rings.map((r) => r.map((p) => [+p.x.toFixed(3), +p.y.toFixed(3)]))));
const b = st.bones[(s.shape as any).bone]; console.log(JSON.stringify(b, (k, v) => (typeof v === "number" ? +v.toFixed(3) : v)));
