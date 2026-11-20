/** ✏️ Draws the 2D representation of the stroller asset (one scale, the three viewpoints side by side, one pose) as a PNG: filled shapes in their tone in z order, outlines and open strokes in ink, the rig joints as red dots in the elevation.
 *
 * Usage: `bun r3-stroller-view2d.ts [--scale 50] [--pose id] [--source file] [--px 900]`
 * @see ./round3-brief.md */
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { Canvas, type RGB } from "./volumes-raster.ts";
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const args = process.argv.slice(2);
const opt = (n: string, d: string): string => (args.includes(n) ? args[args.indexOf(n) + 1]! : d);
const scale = Number(opt("--scale", "50"));
const px = Number(opt("--px", "900"));
const { entry, asset } = entryOf("humans/parent-with-stroller");
const assetFile = opt("--asset", "");
const base = assetFile === "" ? asset : JSON.parse(readFileSync(assetFile, "utf8")).asset;
const m: any = merged(base, loadSource(entry));
const pose = opt("--pose", m.presentation.defaultPose);
const clip = opt("--clip", "");
const query = clip === "" ? { kind: "pose", pose } : { kind: "clip", clip, timeS: Number(opt("--time", "0")) };
const st: any = evaluate(m, query as any, { kind: "scale", scale: { denominator: scale } });
const views = ["elevation", "front", "plan"] as const;
const boxes = views.map((v) => {
  let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
  for (const s of st.shapes[v]) for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  return { x0: x0 - 0.05, x1: x1 + 0.05, y0: y0 - 0.05, y1: y1 + 0.05 };
});
const k = px / Math.max(...boxes.map((b) => b.y1 - b.y0), ...boxes.map((b) => (b.x1 - b.x0) * 0.8));
const widths = boxes.map((b) => Math.ceil((b.x1 - b.x0) * k));
const H = Math.ceil(Math.max(...boxes.map((b) => b.y1 - b.y0)) * k) + 30;
const W = widths.reduce((a, b) => a + b, 0);
const c = new Canvas(W, H, [250, 250, 246]);
const fillRings = (rings: { x: number; y: number }[][], col: RGB): void => {
  let y0 = Infinity, y1 = -Infinity;
  for (const r of rings) for (const p of r) { y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  for (let y = Math.max(0, Math.floor(y0)); y <= Math.min(H - 1, Math.ceil(y1)); y++) {
    const xs: [number, number][] = [];
    for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) {
      const a = r[j]!, b = r[i]!;
      if ((a.y <= y + 0.5) !== (b.y <= y + 0.5)) xs.push([a.x + ((y + 0.5 - a.y) / (b.y - a.y)) * (b.x - a.x), b.y > a.y ? 1 : -1]);
    }
    xs.sort((p, q) => p[0] - q[0]);
    let wind = 0;
    for (let i = 0; i + 1 < xs.length; i++) {
      wind += xs[i]![1];
      if (wind !== 0) for (let x = Math.round(xs[i]![0]); x < Math.round(xs[i + 1]![0]); x++) c.set(x, y, col);
    }
  }
};
let ox = 0;
views.forEach((v, vi) => {
  const b = boxes[vi]!;
  const T = (p: { x: number; y: number }) => ({ x: ox + (p.x - b.x0) * k, y: v === "plan" ? 20 + (p.y - b.y0) * k : H - 10 - (p.y - b.y0) * k });
  const shapes = [...st.shapes[v]].sort((a: any, b2: any) => (a.shape.z ?? 0) - (b2.shape.z ?? 0));
  for (const s of shapes) {
    const rings = s.rings.map((r: any[]) => r.map(T));
    const closed = s.shape.closed !== false && s.shape.kind !== "line";
    const g = Math.round(255 * (s.shape.tone ?? 0.5));
    if (s.shape.filled === true) fillRings(rings, [g, g, g]);
    const ink: RGB = s.shape.role === "texture" ? [120, 120, 160] : s.shape.role === "detail" ? [60, 60, 60] : [0, 0, 0];
    for (const r of rings) for (let i = closed ? 0 : 1, j = closed ? r.length - 1 : 0; i < r.length; j = i++) c.line(r[j].x, r[j].y, r[i].x, r[i].y, ink);
  }
  if (v === "elevation") for (const bn of Object.values(st.bones) as any[]) { const p = T(bn.origin); c.rect(Math.round(p.x) - 2, Math.round(p.y) - 2, 4, 4, [230, 30, 30]); const q = T(bn.tip); c.line(p.x, p.y, q.x, q.y, [230, 30, 30]); }
  c.text(ox + 6, 4, `${v} 1:${scale} ${clip === "" ? pose : clip + "@" + opt("--time", "0")}`, [20, 20, 20], 2);
  ox += widths[vi]!;
});
const dir = join(import.meta.dir, "🗑️generated", "r3-parent-with-stroller");
mkdirSync(dir, { recursive: true });
const out = join(dir, `2d.s${scale}.${clip === "" ? pose : clip + "-" + opt("--time", "0")}${opt("--tag", "")}.png`);
writeFileSync(out, c.png());
console.log(out);
