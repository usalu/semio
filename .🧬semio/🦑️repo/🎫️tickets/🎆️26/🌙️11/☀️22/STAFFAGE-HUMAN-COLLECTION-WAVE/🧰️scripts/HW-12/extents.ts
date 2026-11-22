/** 📏️ Which drawn shapes and which solids make the extremes of each view and axis in the default pose: bun extents.ts <slug> <scale> */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { HUMANS } from "./lib.ts";
const [slug, scaleArg] = process.argv.slice(2);
const scale = Number(scaleArg);
const asset = (await Bun.file(`${HUMANS}/${slug}/🔣️.json`).json()).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
for (const view of ["elevation", "front", "plan"] as const) {
  const items = state.shapes[view].map((s: any) => { const pts = s.rings.flat(); return { id: `${s.shape.id}@${s.shape.bone}`, x0: Math.min(...pts.map((p: any) => p.x)), x1: Math.max(...pts.map((p: any) => p.x)), y0: Math.min(...pts.map((p: any) => p.y)), y1: Math.max(...pts.map((p: any) => p.y)) }; });
  const ext = (k: "x0" | "x1" | "y0" | "y1", f: (a: number, b: number) => boolean) => items.reduce((a: any, b: any) => (f(b[k], a[k]) ? b : a));
  const show = (k: "x0" | "x1" | "y0" | "y1", lo: boolean) => { const e = ext(k, lo ? (a, b) => a < b : (a, b) => a > b); return `${k} ${e[k].toFixed(3)} ${e.id}`; };
  console.log(view.padEnd(10), show("x0", true), "|", show("x1", false), "|", show("y0", true), "|", show("y1", false));
}
const solids = worldSolids(asset, state.bones as never, scale);
for (let k = 0; k < 3; k++) {
  const ext = solids.map((s: any) => { let lo = Infinity, hi = -Infinity; for (let i = k; i < s.positions.length; i += 3) { lo = Math.min(lo, s.positions[i]); hi = Math.max(hi, s.positions[i]); } return { id: s.id, lo, hi }; });
  const lo = ext.reduce((a: any, b: any) => (b.lo < a.lo ? b : a)), hi = ext.reduce((a: any, b: any) => (b.hi > a.hi ? b : a));
  console.log(`3D axis ${"xyz"[k]}: ${lo.lo.toFixed(3)} ${lo.id} .. ${hi.hi.toFixed(3)} ${hi.id}`);
}
