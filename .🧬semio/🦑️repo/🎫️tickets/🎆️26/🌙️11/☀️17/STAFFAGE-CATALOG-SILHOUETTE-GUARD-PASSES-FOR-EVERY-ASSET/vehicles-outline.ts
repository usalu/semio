/** ✏️ Outer outline of the volume silhouette of chosen solids as a simplified polygon in world metres of the view: bun vehicles-outline.ts <asset> <scale> <view> <id-substring,...|*> [tolerance-m] [x0,x1,y0,y1 window]
 *  The polygon is the outer boundary of the union of the matching solids (small holes closed), simplified by Douglas-Peucker; prints JSON points. */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, extentOf, fillSmallHoles, referenceOf, worldSolids, drawnFor } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, ids, tolArg] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string, string | undefined];
const scale = Number(scaleArg);
const feature = featureOf(scale);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const all = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const keys = ids.split(",");
const [skipKeys, takeKeys] = [keys.filter((k) => k.startsWith("!")).map((k) => k.slice(1)), keys.filter((k) => !k.startsWith("!"))];
const hit = (id: string, k: string): boolean => (k.startsWith("=") ? id === k.slice(1) : k.startsWith("^") ? id.startsWith(k.slice(1)) : id.includes(k));
const solids = all.filter((s) => (takeKeys.includes("*") || takeKeys.some((k) => hit(s.id, k))) && !skipKeys.some((k) => hit(s.id, k)));
const e = extentOf(solids);
const drawn = drawnFor(asset, state, view, scale);
const groups = referenceOf(view, solids, undefined, drawn).cut;
const cell = feature / 4;
const px = (v: "elevation" | "front" | "plan", p: readonly number[]) => (v === "plan" ? [p[0]!, p[2]!] : v === "front" ? [p[2]!, p[1]!] : [p[0]!, p[1]!]);
const [a, b] = [px(view, e.min), px(view, e.max)];
const box = { minX: Math.min(a[0]!, b[0]!) - 0.1, maxX: Math.max(a[0]!, b[0]!) + 0.1, minY: Math.min(a[1]!, b[1]!) - 0.1, maxY: Math.max(a[1]!, b[1]!) + 0.1 };
const mask = new Mask(box, cell);
for (const g of groups) mask.fill(g, true);
mask.seal();
const [w, h] = [mask.width, mask.height];
const cells = fillSmallHoles(mask.cells, w, h, Math.round((4 * feature / cell) ** 2));
const on = (i: number, j: number) => i >= 0 && j >= 0 && i < w && j < h && cells[j * w + i] === 1;
let start: [number, number] | undefined;
outer: for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (on(i, j) && !on(i - 1, j)) { start = [i, j]; break outer; }
const pts: [number, number][] = [];
if (start !== undefined) {
  let [x, y, dir] = [start[0], start[1], 0];
  const dx = [0, 1, 0, -1], dy = [1, 0, -1, 0];
  const first = [x, y];
  let guard = 0;
  do {
    pts.push([x, y]);
    const [left, right] = [dir, dir];
    void left; void right;
    const ahead = (d: number) => {
      const [fx, fy] = [x + (dx[d]! > 0 ? 0 : dx[d]! < 0 ? -1 : 0) + (dy[d]! !== 0 ? 0 : 0), 0];
      void fx; void fy;
    };
    void ahead;
    const cellL = (d: number): [number, number] => d === 0 ? [x - 1, y] : d === 1 ? [x, y] : d === 2 ? [x, y - 1] : [x - 1, y - 1];
    const cellR = (d: number): [number, number] => d === 0 ? [x, y] : d === 1 ? [x, y - 1] : d === 2 ? [x - 1, y - 1] : [x - 1, y];
    const nd = (d: number) => (d + 1) % 4, pd = (d: number) => (d + 3) % 4;
    const open = (d: number) => { const [l, r] = [cellL(d), cellR(d)]; return on(r[0], r[1]) && !on(l[0], l[1]); };
    const turnR = () => { const d = nd(dir); const [l, r] = [cellL(d), cellR(d)]; return on(l[0], l[1]) !== on(r[0], r[1]); };
    void turnR;
    const pref = [pd(dir), dir, nd(dir), nd(nd(dir))];
    let moved = false;
    for (const d of pref) {
      const l = cellL(d), r = cellR(d);
      if (on(l[0], l[1]) && !on(r[0], r[1])) { dir = d; moved = true; break; }
      void open;
    }
    if (!moved) break;
    x += dx[dir]!; y += dy[dir]!;
    if (++guard > 5_000_000) break;
  } while (x !== first[0] || y !== first[1] || pts.length < 4);
}
const world = pts.map(([i, j]) => [box.minX + i * cell, box.minY + j * cell] as [number, number]);
const tol = Number(tolArg ?? feature / 2);
const rdp = (p: [number, number][], lo: number, hi: number, keep: boolean[]): void => {
  let [m, idx] = [0, -1];
  const [ax, ay, bx, by] = [p[lo]![0], p[lo]![1], p[hi]![0], p[hi]![1]];
  const len = Math.hypot(bx - ax, by - ay) || 1e-12;
  for (let k = lo + 1; k < hi; k++) { const d = Math.abs((bx - ax) * (ay - p[k]![1]) - (ax - p[k]![0]) * (by - ay)) / len; if (d > m) { m = d; idx = k; } }
  if (m > tol && idx > 0) { keep[idx] = true; rdp(p, lo, idx, keep); rdp(p, idx, hi, keep); }
};
const closed = [...world, world[0]!];
const keep = new Array(closed.length).fill(false) as boolean[];
keep[0] = keep[closed.length - 1] = true;
let far = 0, fi = 0;
for (let k = 0; k < closed.length; k++) { const d = Math.hypot(closed[k]![0] - closed[0]![0], closed[k]![1] - closed[0]![1]); if (d > far) { far = d; fi = k; } }
keep[fi] = true;
rdp(closed, 0, fi, keep); rdp(closed, fi, closed.length - 1, keep);
const out = closed.filter((_, k) => keep[k]).slice(0, -1).map(([x, y]) => [+x.toFixed(3), +y.toFixed(3)]);
console.log(JSON.stringify(out));
console.error(`${out.length} points from ${world.length} cells (${solids.length} solids)`);
