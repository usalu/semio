/** 📏️ Per view and scale, the shapes that set the extremes of the default-pose drawing of an asset (the extent checks of validateAsset).
 *  bun extents.ts <slug> [pose] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [slug, poseArg] = process.argv.slice(2);
const a = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`).json()).asset;
const pose = poseArg ?? a.presentation.defaultPose;
for (const d of [50, 100, 200, 500]) {
  const s: any = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: d } });
  for (const v of ["elevation", "front", "plan"]) {
    const rows = s.shapes[v].filter((x: any) => x.shape.role !== "ground").map((x: any) => { const p = x.rings.flat(); return { id: x.shape.id, x0: Math.min(...p.map((q: any) => q.x)), x1: Math.max(...p.map((q: any) => q.x)), y0: Math.min(...p.map((q: any) => q.y)), y1: Math.max(...p.map((q: any) => q.y)) }; });
    const ext = (k: "x0" | "x1" | "y0" | "y1", f: (a: number, b: number) => boolean) => rows.reduce((b: any, r: any) => (f(r[k], b[k]) ? r : b));
    const lo = ext("x0", (p, q) => p < q), hi = ext("x1", (p, q) => p > q), bo = ext("y0", (p, q) => p < q), to = ext("y1", (p, q) => p > q);
    console.log(`1:${d} ${v.padEnd(9)} h ${(hi.x1 - lo.x0).toFixed(3)} [${lo.id} ${lo.x0.toFixed(3)} .. ${hi.id} ${hi.x1.toFixed(3)}]  v ${(to.y1 - bo.y0).toFixed(3)} [${bo.id} ${bo.y0.toFixed(3)} .. ${to.id} ${to.y1.toFixed(3)}]`);
  }
}
