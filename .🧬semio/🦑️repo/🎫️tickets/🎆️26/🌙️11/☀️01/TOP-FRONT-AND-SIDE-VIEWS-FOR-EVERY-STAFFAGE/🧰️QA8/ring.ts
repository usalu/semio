import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [id, scale, pose, vp, sid] = process.argv.slice(2);
const asset = catalog().find((a: any) => a.id === id)!;
const st: any = evaluate(asset, { kind: "pose", pose } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
for (const s of st.shapes[vp].filter((s: any) => s.shape.id === sid)) console.log(s.shape.kind, JSON.stringify(s.rings.map((r: any[]) => r.map((p) => [Number(p.x.toFixed(3)), Number(p.y.toFixed(3))]))));
