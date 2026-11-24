/** 🔬️ The drawn (D) and volume (V) masks of the tile of the guard that holds a point, printed as characters around that point after the small holes are filled (`#` both, `d` drawing only, `v` volume only, `.` neither): shows why a tile measures a distance that one raster does not.
 *  bun tilemask.ts <id|file.json> <scale> <view> <x> <y> [half-width-cells] [half-height-cells] */
import { S, fileOf } from "./lib.ts";
const { fillSmallHoles, measureView, worldSolids, Mask, TILE_CELLS, WINDOW_FEATURES } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);

const [name, scaleArg, view, xs, ys, hw = "30", hh = "6"] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const whole = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const { cell, box: frame } = whole;
const side = TILE_CELLS * cell;
const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
const [i, j] = [Math.floor((Number(xs) - frame.minX) / side), Math.floor((Number(ys) - frame.minY) / side)];
const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
const tile = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false, { core, box, cell });
const drawn = new Mask(box, cell), vol = new Mask(box, cell);
for (const g of tile.filled) drawn.fill(g);
for (const g of tile.outlines) drawn.fill(g);
for (const g of tile.reference) vol.fill(g, true);
if (tile.section) for (const g of tile.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.max(1, Math.round((4 * feature / cell) ** 2));
const [fd, fv] = [fillSmallHoles(drawn.cells, w, h, holes), fillSmallHoles(vol.cells, w, h, holes)];
const [ci, cj] = [Math.floor((Number(xs) - box.minX) / cell), Math.floor((Number(ys) - box.minY) / cell)];
console.log(`tile ${i},${j} window x ${box.minX.toFixed(4)}..${box.maxX.toFixed(4)} y ${box.minY.toFixed(4)}..${box.maxY.toFixed(4)} (${w} x ${h} cells), H ${(tile.hausdorffM * 1000).toFixed(1)} mm; rows from high y to low y, columns from x ${(box.minX + (ci - Number(hw)) * cell).toFixed(4)}`);
for (let b = Math.min(h - 1, cj + Number(hh)); b >= Math.max(0, cj - Number(hh)); b--) {
  let line = "";
  for (let a = Math.max(0, ci - Number(hw)); a <= Math.min(w - 1, ci + Number(hw)); a++) line += fd[b * w + a] ? (fv[b * w + a] ? "#" : "d") : fv[b * w + a] ? "v" : ".";
  console.log(`${(box.minY + (b + 0.5) * cell).toFixed(4)} ${line}`);
}
