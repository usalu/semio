/** 🧹 Tries clip/grow/redraw trials of every plain drawn shape near a point and prints the resulting excess; env OPS applies ops first; writes the best to env OUT as ops when BEST=1.
 *  bun humansa-sweep.ts <asset> <scale> <view> <x> <y> [d=0.02] [radius=0.045] */
import { writeFileSync } from "node:fs";
import { applyOps, applyShape, clustersOf, loadAsset, nearOf, patchShape, plainOf, redrawShape, shapesOf, clipStroke, type Viewpoint } from "./humansa-lib.ts";
const [name, sc, view, xs, ys, dArg, rArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string | undefined, string | undefined];
const asset = await loadAsset(name);
const scale = Number(sc);
applyOps(asset, process.env.OPS);
const c = { side: (process.env.SIDE ?? "volume-only") as "drawn-only" | "volume-only", x: Number(xs), y: Number(ys), d: Number(dArg ?? 0.02) };
const base = clustersOf(asset, scale, view);
console.log(`base h ${(base.h * 1000).toFixed(1)} excess ${base.excess.toFixed(2)} iou ${base.iou.toFixed(3)}`);
const near = nearOf(asset, scale, view, c, Number(rArg ?? 0.045));
const rows: { label: string; shapes: any[]; excess: number; h: number; iou: number }[] = [];
const run = (label: string, shapes: (any | undefined)[]): void => {
  if (shapes.some((s) => s === undefined)) return;
  const before = shapes.map((s) => shapesOf(asset, scale, view).find((x) => x.id === s.id));
  shapes.forEach((s) => applyShape(asset, scale, view, s));
  const r = clustersOf(asset, scale, view);
  before.forEach((b) => applyShape(asset, scale, view, b));
  rows.push({ label, shapes, excess: r.excess, h: r.h, iou: r.iou });
};
for (const s of near.shapes) {
  const sh = plainOf(asset, scale, view, s.id);
  if (sh === undefined) continue;
  if (sh.filled === false) { for (const er of [0, 0.5, 1]) run(`stroke ${s.id} e${er}`, [clipStroke(asset, scale, view, s.id, er)]); continue; }
  const ids = near.solids.filter((v) => v.id === s.id || v.id.startsWith(s.id + "-")).map((v) => v.id);
  const tw = /-[lr]($|-)/.test(s.id) ? s.id.replace(/-([lr])($|-)/, (_m, a, b) => `-${a === "l" ? "r" : "l"}${b}`) : undefined;
  const twOk = tw !== undefined && plainOf(asset, scale, view, tw) !== undefined && tw !== s.id;
  const twIds = ids.map((i) => (/-[lr]($|-)/.test(i) ? i.replace(/-([lr])($|-)/, (_m, a, b) => `-${a === "l" ? "r" : "l"}${b}`) : i));
  if (ids.length > 0) run(`redraw ${s.id}${twOk ? "+twin" : ""}`, [redrawShape(asset, scale, view, s.id, ids), ...(twOk ? [redrawShape(asset, scale, view, tw!, twIds)] : [])]);
  for (const side of ["clip", "grow"] as const) for (const dr of [0.03, 0.06, 0.1, 0.15]) for (const mg of [0, 0.4]) run(`${side} ${s.id}${twOk ? "+twin" : ""} r${dr} m${mg}`, [patchShape(asset, scale, view, s.id, c, dr, mg, 0.12, side, process.env.ITER === "1"), ...(twOk ? [patchShape(asset, scale, view, tw!, c, dr, mg, 0.12, side, process.env.ITER === "1")] : [])]);
}
rows.sort((a, b) => a.excess - b.excess);
for (const r of rows.slice(0, Number(process.env.ROWS ?? 12))) console.log(`${r.label}: h ${(r.h * 1000).toFixed(1)} excess ${r.excess.toFixed(2)} iou ${r.iou.toFixed(3)}`);
if (process.env.BEST && process.env.OUT && rows[0] !== undefined && rows[0].excess < base.excess) {
  const out = (process.env.OUT && require("node:fs").existsSync(process.env.OUT)) ? JSON.parse(require("node:fs").readFileSync(process.env.OUT, "utf8")) : [];
  for (const s of rows[0].shapes) out.push({ op: "shape", scale: String(scale), view, id: s.id, shape: s });
  writeFileSync(process.env.OUT, JSON.stringify(out, null, 1));
  console.log("saved", rows[0].label);
}
