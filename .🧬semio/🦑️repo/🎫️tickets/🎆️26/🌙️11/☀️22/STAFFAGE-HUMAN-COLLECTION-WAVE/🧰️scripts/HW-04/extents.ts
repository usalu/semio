/** 📐️ Per view, the drawn shapes with the largest and smallest extent along each paper axis in a pose at a scale: bun extents.ts <asset.json> <scale> [pose] */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const [path, scale, poseArg] = process.argv.slice(2);
const asset = (await Bun.file(path!).json()).asset;
const st = evaluate(asset, { kind: "pose", pose: poseArg ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
for (const view of ["elevation", "front", "plan"]) {
  const rows = st.shapes[view].map((e: any) => { const pts = e.rings.flat(); return { id: e.shape.id, x0: Math.min(...pts.map((p: any) => p.x)), x1: Math.max(...pts.map((p: any) => p.x)), y0: Math.min(...pts.map((p: any) => p.y)), y1: Math.max(...pts.map((p: any) => p.y)) }; }).filter((r: any) => Number.isFinite(r.x0));
  const top = (k: string, d: number) => [...rows].sort((a: any, b: any) => d * (b[k] - a[k])).slice(0, 3).map((r: any) => `${r.id}:${r[k].toFixed(3)}`).join(" ");
  console.log(`${view}: x0 ${top("x0", -1)} | x1 ${top("x1", 1)} | y0 ${top("y0", -1)} | y1 ${top("y1", 1)}`);
}
