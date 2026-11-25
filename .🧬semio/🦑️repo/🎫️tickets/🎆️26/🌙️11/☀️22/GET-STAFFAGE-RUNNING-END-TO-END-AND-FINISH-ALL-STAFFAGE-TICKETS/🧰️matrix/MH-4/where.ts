/** 📍️ Where a view leaves the silhouette guard: `bun where.ts <id|file.json> --scale 500 --view front [--top 5]` measures the view as `🧊️catalog-silhouette` does (`measureAsset`), prints its IoU and largest boundary distance, and then, on a plain raster of the same cell without the hole and seal rules of the guard, the places (view metres) where the outline of the drawing and the outline of the body are farthest apart in each direction, so the part that moves the guard can be named. Run it from `S/📦️packages/🟦️typescript`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
type Point = { x: number; y: number };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(flag("scale"));
const view = flag("view")!;
const top = Number(flag("top") ?? 5);
const feature = featureOf(scale);
const m = measureAsset(asset, scale, undefined, undefined, feature).views[view];
console.log(`${asset.id} 1:${scale} ${view}: IoU ${m.iou.toFixed(4)}, largest boundary distance ${(m.hausdorffM * 1000).toFixed(1)} mm (limit ${(2 * feature * 1000).toFixed(0)} mm), guard cell ${(m.cell * 1000).toFixed(1)} mm`);
const groups = (list: readonly (readonly (readonly Point[])[])[]): readonly (readonly Point[])[][] => list.map((group) => [...group]);
const [drawn, body] = [groups([...m.filled, ...m.outlines]), groups(m.reference)];
const all = [...drawn, ...body].flat(2);
const cell = m.cell;
const [x0, y0] = [Math.min(...all.map((p) => p.x)) - 2 * cell, Math.min(...all.map((p) => p.y)) - 2 * cell];
const [w, h] = [Math.ceil((Math.max(...all.map((p) => p.x)) - x0) / cell) + 3, Math.ceil((Math.max(...all.map((p) => p.y)) - y0) / cell) + 3];
const inside = (group: readonly (readonly Point[])[], x: number, y: number): boolean => {
  let odd = false;
  for (const ring of group) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > y !== ring[j]!.y > y && x < ((ring[j]!.x - ring[i]!.x) * (y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) odd = !odd;
  return odd;
};
const raster = (list: readonly (readonly (readonly Point[])[])[]): Uint8Array => {
  const mask = new Uint8Array(w * h);
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (list.some((group) => inside(group, x0 + (i + 0.5) * cell, y0 + (j + 0.5) * cell))) mask[j * w + i] = 1;
  return mask;
};
const edge = (mask: Uint8Array): number[] => {
  const cells: number[] = [];
  for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) if (mask[j * w + i] === 1 && (mask[j * w + i - 1] === 0 || mask[j * w + i + 1] === 0 || mask[(j - 1) * w + i] === 0 || mask[(j + 1) * w + i] === 0)) cells.push(j * w + i);
  return cells;
};
const [a, b] = [edge(raster(drawn)), edge(raster(body))];
const far = (from: number[], to: number[], label: string): void => {
  const rows = from.map((k) => {
    const [i, j] = [k % w, Math.floor(k / w)];
    let best = Infinity;
    for (const q of to) best = Math.min(best, Math.hypot((q % w) - i, Math.floor(q / w) - j));
    return { x: x0 + (i + 0.5) * cell, y: y0 + (j + 0.5) * cell, d: best * cell };
  }).sort((p, q) => q.d - p.d);
  const picked: typeof rows = [];
  for (const row of rows) if (picked.length < top && picked.every((p) => Math.hypot(p.x - row.x, p.y - row.y) > 6 * cell)) picked.push(row);
  console.log(`   ${label}: ${picked.map((p) => `${(p.d * 1000).toFixed(0)} mm at (${p.x.toFixed(3)}, ${p.y.toFixed(3)})`).join("; ")}`);
};
far(a, b, "drawn outline far from the body outline");
far(b, a, "body outline far from the drawn outline");
