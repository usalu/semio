import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// usage: maxx.ts <asset> <scale> <clip> <t>   prints the shapes that set the x extent per view
const [id, scale, clip, t] = process.argv.slice(2);
const asset = catalog().find((a: any) => a.id === id)!;
const st: any = evaluate(asset, { kind: "clip", clip, timeS: Number(t) } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
for (const vp of ["elevation", "plan"]) {
  const rows = (st.shapes[vp] as any[]).filter((s) => s.shape.role !== "ground").map((s) => ({ id: s.shape.id, max: Math.max(...s.rings.flat().map((p: any) => p.x)), min: Math.min(...s.rings.flat().map((p: any) => p.x)) }));
  rows.sort((a, b) => b.max - a.max);
  console.log(vp, "MAX", rows.slice(0, 4).map((r) => `${r.id}:${r.max.toFixed(3)}`).join(" "), "| MIN", [...rows].sort((a, b) => a.min - b.min).slice(0, 2).map((r) => `${r.id}:${r.min.toFixed(3)}`).join(" "));
}
