/** 🔬️ Where drawing and volume disagree: worst drawn points outside the volume and volume points outside the drawing, per view and scale.
 *  bun diag.ts <slug|file> <scale> [view] */
import { join, resolve } from "node:path";
import { evaluate } from "./lib.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset, Mask } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [arg, scaleArg, only] = process.argv.slice(2);
const asset = (await Bun.file(arg!.endsWith(".json") ? arg! : join(catalog, arg!, "🔣️.json")).json()).asset;
const scale = Number(scaleArg);
const m = measureAsset(asset, scale, undefined, undefined, featureOf(scale));
const inside = (p: { x: number; y: number }, ring: readonly { x: number; y: number }[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const a = ring[i]!, b = ring[j]!; if ((a.y > p.y) !== (b.y > p.y) && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c; } return c; };
const segDist = (p: any, a: any, b: any) => { const dx = b.x - a.x, dy = b.y - a.y; const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1))); return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy); };
const dist = (p: any, groups: any[]) => { let inAny = false, d = Infinity; for (const g of groups) { let par = 0; for (const r of g) { if (inside(p, r)) par++; for (let i = 0; i < r.length; i++) d = Math.min(d, segDist(p, r[i], r[(i + 1) % r.length])); } if (par % 2 === 1) inAny = true; } return inAny ? 0 : d; };
const sample = (groups: any[]) => groups.flatMap((g: any) => g.flatMap((r: any) => r.flatMap((a: any, i: number) => { const b = r[(i + 1) % r.length]; const n = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / 0.005)); return Array.from({ length: n }, (_, k) => ({ x: a.x + ((b.x - a.x) * k) / n, y: a.y + ((b.y - a.y) * k) / n })); })));
for (const view of VIEWPOINTS) {
  if (only && view !== only) continue;
  const v = m.views[view] as any;
  const drawnGroups = [...v.filled, ...v.outlines];
  const refGroups = v.full ?? v.reference;
  const worst = (pts: any[], groups: any[]) => pts.map((p) => ({ p, d: dist(p, groups) })).sort((a, b) => b.d - a.d).slice(0, 3).map((e) => `(${e.p.x.toFixed(3)},${e.p.y.toFixed(3)}) ${e.d.toFixed(3)}`);
  console.log(`1:${scale} ${view} iou ${v.iou.toFixed(3)} haus ${v.hausdorffM.toFixed(3)}`);
  console.log("  drawn outside volume:", worst(sample(drawnGroups), refGroups).join("  "));
  console.log("  volume outside drawn:", worst(sample(refGroups), drawnGroups).join("  "));
}
