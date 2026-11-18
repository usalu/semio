/** 🛡️ The tiled measurement of the guard, tile by tile: the tiles whose Hausdorff distance exceeds the limit, with the position of the tile.
 *  bun machinery-guard.ts <asset> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { WINDOW_FEATURES, TILE_CELLS, boundsOf, drawnFor, measureView, referenceOf, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view] = process.argv.slice(2) as [string, string, Viewpoint];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((x) => x.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const coarse = measureView(asset, state, solids, view, scale, 2000, feature, Infinity, true);
const edge = coarse.edge!;
const cell = feature / 4;
const side = TILE_CELLS * cell;
const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
const refs = referenceOf(view, solids, (asset.representations[String(scale) as never] as any).sections?.[view], drawnFor(asset, state, view, scale));
const bounds = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
const growBox = (b: readonly [number, number, number, number]): void => { bounds.minX = Math.min(bounds.minX, b[0]); bounds.maxX = Math.max(bounds.maxX, b[1]); bounds.minY = Math.min(bounds.minY, b[2]); bounds.maxY = Math.max(bounds.maxY, b[3]); };
for (const g of refs.full) growBox(boundsOf(g));
for (const g of refs.cut) growBox(boundsOf(g));
for (const s of (state.shapes[view] as any[]).filter((q) => ["body", "detail", "glazing"].includes(q.shape.role) && q.rings.length > 0)) growBox(boundsOf(s.rings));
const frame = { minX: bounds.minX - 2 * cell, minY: bounds.minY - 2 * cell, maxX: bounds.maxX + 2 * cell, maxY: bounds.maxY + 2 * cell };
const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
const hasEdge = (b: { minX: number; maxX: number; minY: number; maxY: number }): boolean => {
  const [i0, i1] = [Math.max(0, Math.floor((b.minX - edge.minX) / edge.cell)), Math.min(edge.width - 1, Math.floor((b.maxX - edge.minX) / edge.cell))];
  const [j0, j1] = [Math.max(0, Math.floor((b.minY - edge.minY) / edge.cell)), Math.min(edge.height - 1, Math.floor((b.maxY - edge.minY) / edge.cell))];
  for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) if (edge.mask[j * edge.width + i]) return true;
  return false;
};
const evaluatedAll = (state.shapes[view] as any[]).filter((q) => ["body", "detail", "glazing"].includes(q.shape.role) && q.rings.length > 0);
let worst = 0;
for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
  const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
  const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
  const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
  if (!hasEdge({ minX: core.minX - edge.cell, minY: core.minY - edge.cell, maxX: core.maxX + edge.cell, maxY: core.maxY + edge.cell })) continue;
  const hit = (b: readonly [number, number, number, number]) => b[1] >= box.minX && b[0] <= box.maxX && b[3] >= box.minY && b[2] <= box.maxY;
  const mode = process.env.MODE ?? "both";
  const prepared = { reference: mode === "eval" ? refs.cut : refs.cut.filter((g) => hit(boundsOf(g))), visible: mode === "eval" ? refs.visible : refs.visible.filter((g) => hit(boundsOf(g))), full: mode === "eval" ? refs.full : refs.full.filter((g) => hit(boundsOf(g))), evaluated: mode === "ref" ? evaluatedAll : evaluatedAll.filter((s) => hit(boundsOf(s.rings))) };
  const tile = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false, { core, box, cell, prepared } as never);
  if (tile.hausdorffM > 0.02) for (const s of evaluatedAll) { const bb = boundsOf(s.rings); const pen = ((s.shape.penMm ?? 0.5) * scale) / 1000; if (!hit(bb) && bb[1] + pen >= box.minX && bb[0] - pen <= box.maxX && bb[3] + pen >= box.minY && bb[2] - pen <= box.maxY) console.log("  dropped", s.shape.id, bb.map((v: number) => +v.toFixed(4)).join(","), "box", [box.minX, box.maxX, box.minY, box.maxY].map((v) => +v.toFixed(4)).join(","));}
  worst = Math.max(worst, tile.hausdorffM);
  if (tile.hausdorffM > 2 * feature) console.log(`tile core x ${core.minX.toFixed(2)}..${core.maxX.toFixed(2)} y ${core.minY.toFixed(2)}..${core.maxY.toFixed(2)} hausdorff ${tile.hausdorffM.toFixed(4)}`);
}
console.log("worst", worst.toFixed(4), "tiles", nx * ny);
