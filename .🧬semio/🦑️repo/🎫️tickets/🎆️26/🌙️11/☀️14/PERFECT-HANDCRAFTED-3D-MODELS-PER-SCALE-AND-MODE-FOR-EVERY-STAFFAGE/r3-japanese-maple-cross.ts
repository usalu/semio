/** 🔀️ Where the body silhouettes of two scales of the japanese maple part in one view: cells of one silhouette that lie farther than `--min` from the other, grouped in 0.25 m bins, with the solids of each scale found there.
 * Usage: bun r3-japanese-maple-cross.ts <fine> <coarse> <view> [--min 0.1]
 * @see ./r3-vegetation-japanese-maple.md */
import { distanceField, entryOf, loadSource, merged, meshesAt, meshMask, projectWorld, resolveList, type View } from "./volumes-lib.ts";

const [fine, coarse] = [Number(process.argv[2]), Number(process.argv[3])];
const view = process.argv[4] as View;
const min = Number(process.argv[process.argv.indexOf("--min") + 1] ?? 0.1) || 0.1;
const { entry, asset: raw } = entryOf("vegetation/japanese-maple");
const asset = merged(raw, loadSource(entry)!);
const cell = 0.01;
const box = { minX: -3.5, minY: -3.5, maxX: 3.5, maxY: 5 };
const masks = [fine, coarse].map((s) => {
  const solids = resolveList(asset.volumes[String(s)], "body").filter((x: any) => !x.texture && !x.slender);
  const meshes = meshesAt(asset, s, solids);
  return { s, meshes, mask: meshMask(view, meshes, box, cell) };
});
for (const [a, b] of [[masks[0]!, masks[1]!], [masks[1]!, masks[0]!]]) {
  const inv = new (a.mask.constructor as any)(box, cell);
  for (let i = 0; i < inv.cells.length; i++) inv.cells[i] = (b.mask as any).cells[i];
  const d = distanceField(inv);
  const bins = new Map<string, { n: number; worst: number }>();
  for (let i = 0; i < (a.mask as any).cells.length; i++) {
    if (!(a.mask as any).cells[i] || (b.mask as any).cells[i]) continue;
    const dist = d[i]!;
    if (dist < min) continue;
    const x = box.minX + ((i % a.mask.width) + 0.5) * cell, y = box.minY + (Math.floor(i / a.mask.width) + 0.5) * cell;
    const k = `${(Math.round(x * 4) / 4).toFixed(2)},${(Math.round(y * 4) / 4).toFixed(2)}`;
    const e = bins.get(k) ?? { n: 0, worst: 0 };
    e.n++;
    e.worst = Math.max(e.worst, dist);
    bins.set(k, e);
  }
  console.log(`1:${a.s} (${(a.mask as any).cells.reduce((s: number, c: number) => s + (c ? 1 : 0), 0)} cells, ${a.meshes.length} solids) outside 1:${b.s} (${view}): ${[...bins.values()].reduce((s, e) => s + e.n, 0)} cells`);
  for (const [k, e] of [...bins.entries()].sort((p, q) => q[1].n - p[1].n).slice(0, 12)) console.log(`  at ${k}: ${e.n} cells, worst ${e.worst.toFixed(3)} m`);
}
void projectWorld;
