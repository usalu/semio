/** 📍️ `../MH-4/where.ts` with a scanline raster (the guide dog at 1:50 did not end in twenty minutes there): `bun far.ts <id|file.json> --scale 50 --view elevation [--top 6]` measures the view as `🧊️catalog-silhouette` does (`measureAsset`), prints its IoU and largest boundary distance and then, on a plain raster of the same cell without the hole and seal rules of the guard, the places (view metres) where the outline of the drawing and the outline of the body are farthest apart in each direction. Run it from `S/📦️packages/🟦️typescript`.
 * @see ../MH-4/where.ts — the tool this one repeats */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
type Point = { x: number; y: number };
type Group = readonly (readonly Point[])[];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const scale = Number(flag("scale"));
const view = flag("view")!;
const top = Number(flag("top") ?? 6);
const feature = featureOf(scale);
const m = measureAsset(asset, scale, undefined, undefined, feature).views[view];
console.log(`${asset.id} 1:${scale} ${view}: IoU ${m.iou.toFixed(4)}, largest boundary distance ${(m.hausdorffM * 1000).toFixed(1)} mm (limit ${(2 * feature * 1000).toFixed(0)} mm), guard cell ${(m.cell * 1000).toFixed(1)} mm`);
const [drawn, body]: Group[][] = [[...m.filled, ...m.outlines].map((group: Group) => [...group]), m.reference.map((group: Group) => [...group])];
const all = [...drawn, ...body].flat(2);
const cell = m.cell as number;
const [x0, y0] = [Math.min(...all.map((p) => p.x)) - 2 * cell, Math.min(...all.map((p) => p.y)) - 2 * cell];
const [w, h] = [Math.ceil((Math.max(...all.map((p) => p.x)) - x0) / cell) + 3, Math.ceil((Math.max(...all.map((p) => p.y)) - y0) / cell) + 3];
const raster = (list: readonly Group[]): Uint8Array => {
  const mask = new Uint8Array(w * h);
  for (const group of list) {
    const ys = group.flat().map((p) => p.y);
    const [j0, j1] = [Math.max(0, Math.floor((Math.min(...ys) - y0) / cell) - 1), Math.min(h - 1, Math.ceil((Math.max(...ys) - y0) / cell) + 1)];
    for (let j = j0; j <= j1; j++) {
      const y = y0 + (j + 0.5) * cell;
      const cuts: number[] = [];
      for (const ring of group) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) if (ring[i]!.y > y !== ring[k]!.y > y) cuts.push(((ring[k]!.x - ring[i]!.x) * (y - ring[i]!.y)) / (ring[k]!.y - ring[i]!.y) + ring[i]!.x);
      cuts.sort((a, b) => a - b);
      for (let c = 0; c + 1 < cuts.length; c += 2) for (let i = Math.max(0, Math.ceil((cuts[c]! - x0) / cell - 0.5)); i < w && x0 + (i + 0.5) * cell < cuts[c + 1]!; i++) mask[j * w + i] = 1;
    }
  }
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
    for (const q of to) {
      const d = ((q % w) - i) ** 2 + (Math.floor(q / w) - j) ** 2;
      if (d < best) best = d;
    }
    return { x: x0 + (i + 0.5) * cell, y: y0 + (j + 0.5) * cell, d: Math.sqrt(best) * cell };
  }).sort((p, q) => q.d - p.d);
  const picked: typeof rows = [];
  for (const row of rows) if (picked.length < top && picked.every((p) => Math.hypot(p.x - row.x, p.y - row.y) > 6 * cell)) picked.push(row);
  console.log(`   ${label}: ${picked.map((p) => `${(p.d * 1000).toFixed(0)} mm at (${p.x.toFixed(3)}, ${p.y.toFixed(3)})`).join("; ")}`);
};
far(a, b, "drawn outline far from the body outline");
far(b, a, "body outline far from the drawn outline");
