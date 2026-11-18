/** 📏 Run lengths of the drawn and volume masks along scan lines of one view: bun vehicles-scan.ts <asset> <scale> <view> <v|h> <from> <to> <step> [tolerance-m]
 *  v scans vertical lines at x = from..to (runs are y intervals), h scans horizontal lines at y (runs are x intervals); prints only lines where a run end differs by more than the tolerance (default the feature). */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const num = (v: string): number => Number(v.replace(/^m/, "-"));
const [name, scaleArg, view, dir, from, to, step, tol] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", "v" | "h", string, string, string, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
const { cell, box } = m;
const drawn = new Mask(box, cell);
const only = process.env.ONLY;
const skip = process.env.SKIP?.split(",");
if (only === undefined && skip === undefined) {
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
} else if (only === undefined) {
  for (const s of (state as any).shapes[view]) if (!skip!.includes(String(s.shape.id)) && s.shape.filled !== false) drawn.fill(s.rings);
} else for (const s of (state as any).shapes[view]) if (only.split(",").includes(String(s.shape.id))) drawn.fill(s.rings);
const vol = new Mask(box, cell);
for (const g of m.reference) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.round((4 * feature / cell) ** 2);
const fd = fillSmallHoles(drawn.cells, w, h, holes), fv = fillSmallHoles(vol.cells, w, h, holes);
const runs = (cells: Uint8Array, line: number): [number, number][] => {
  const out: [number, number][] = [];
  const n = dir === "v" ? h : w;
  let start = -1;
  for (let k = 0; k <= n; k++) {
    const on = k < n && (dir === "v" ? cells[k * w + line] : cells[line * w + k]) === 1;
    if (on && start < 0) start = k;
    if (!on && start >= 0) { out.push([start, k]); start = -1; }
  }
  return out;
};
const origin = dir === "v" ? box.minY : box.minX;
const fmt = (r: [number, number][]) => r.map(([a, b]) => `${(origin + a * cell).toFixed(3)}..${(origin + b * cell).toFixed(3)}`).join(" ");
const limit = Number(tol ?? feature);
for (let t = num(from); t <= num(to) + 1e-9; t += num(step)) {
  const line = Math.floor((t - (dir === "v" ? box.minX : box.minY)) / cell);
  const [a, b] = [runs(fd, line), runs(fv, line)];
  let differs = a.length !== b.length;
  if (!differs) for (let k = 0; k < a.length; k++) if (Math.abs(a[k]![0] - b[k]![0]) * cell > limit || Math.abs(a[k]![1] - b[k]![1]) * cell > limit) differs = true;
  if (differs) console.log(`${dir === "v" ? "x" : "y"}=${t.toFixed(3)} drawn: ${fmt(a)} | volume: ${fmt(b)}`);
}
