/** 🧪️ Probe: the shapes that reach furthest in each direction of one view at one scale in a pose. bun probe-extent.ts <file> <pose> <scale> <view> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [f, pose, scale, view] = process.argv.slice(2);
const a = (await Bun.file(f!).json()).asset;
const st: any = evaluate(a, { kind: "pose", pose } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
const rows = st.shapes[view!].map((e: any) => { const p = e.rings.flat(); return { id: e.shape.id, x0: Math.min(...p.map((q: any) => q.x)), x1: Math.max(...p.map((q: any) => q.x)), y0: Math.min(...p.map((q: any) => q.y)), y1: Math.max(...p.map((q: any) => q.y)) }; }).filter((r: any) => isFinite(r.x0));
for (const k of ["x0", "x1", "y0", "y1"]) { const s = [...rows].sort((p: any, q: any) => (k.endsWith("0") ? p[k] - q[k] : q[k] - p[k])).slice(0, 3); console.log(k, s.map((r: any) => `${r.id} ${r[k].toFixed(3)}`).join(" | ")); }
