/** 🧪️ Probe: per-shape bounds of two asset files in one pose, scale and view, printing the shapes that differ. bun probe-diff.ts <a.json> <b.json> <pose> <scale> <view> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [fa, fb, pose, scale, view] = process.argv.slice(2);
const load = async (f: string) => (f.endsWith(".json") ? (await Bun.file(f).json()).asset : null);
const bounds = (a: any) => { const s: any = evaluate(a, { kind: "pose", pose } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any); return s.shapes[view!].map((e: any) => { const p = e.rings.flat(); return [e.shape.id, p.length ? Math.min(...p.map((q: any) => q.x)) : NaN, Math.max(...p.map((q: any) => q.x)), Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))]; }); };
const A = bounds(await load(fa!)), B = bounds(await load(fb!));
for (let i = 0; i < A.length; i++) { const d = A[i].slice(1).map((v: number, k: number) => Math.abs(v - B[i][k + 1])); if (Math.max(...d) > 0.003 || isNaN(Math.max(...d))) console.log(A[i][0], A[i].slice(1).map((v: number) => v.toFixed(3)).join(" "), "->", B[i].slice(1).map((v: number) => v.toFixed(3)).join(" ")); }
