/** 📐 The extremal drawn shapes per view for one pose or clip instant. bun spread-ext.ts <asset> <scale> <pose:ID|clip:ID:T> [N] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { loadAsset } from "./humansa-lib.ts";
const [name, scale, q, n = "4"] = process.argv.slice(2);
const a = await loadAsset(name);
const p = q.split(":");
const query: any = p[0] === "pose" ? { kind: "pose", pose: p[1] } : { kind: "clip", clip: p[1], timeS: Number(p[2]) };
const st = evaluate(a, query, { kind: "scale", scale: { denominator: Number(scale) } }) as any;
const rows = (v: string) => st.shapes[v].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const pts = s.rings.flat(); const xs = pts.map((t: any) => t.x), ys = pts.map((t: any) => t.y); return { id: s.shape.id, bone: s.shape.bone, solid: s.shape.solid !== undefined, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) }; });
const f = (r: any) => `${String(r.id).padEnd(28)} ${String(r.bone).padEnd(18)} ${r.solid ? "S" : " "} x[${r.x0.toFixed(3)},${r.x1.toFixed(3)}] y[${r.y0.toFixed(3)},${r.y1.toFixed(3)}]`;
for (const v of ["elevation", "front", "plan"]) {
  const r = rows(v);
  const e = { x0: Math.min(...r.map((t: any) => t.x0)), x1: Math.max(...r.map((t: any) => t.x1)), y0: Math.min(...r.map((t: any) => t.y0)), y1: Math.max(...r.map((t: any) => t.y1)) };
  console.log(`== ${v} x[${e.x0.toFixed(3)},${e.x1.toFixed(3)}] y[${e.y0.toFixed(3)},${e.y1.toFixed(3)}]`);
  for (const [k, key, d] of [["minX", "x0", 1], ["maxX", "x1", -1], ["minY", "y0", 1], ["maxY", "y1", -1]] as const) { console.log(` ${k}:`); for (const t of [...r].sort((A: any, B: any) => d * (A[key] - B[key])).slice(0, Number(n))) console.log("   " + f(t)); }
}
