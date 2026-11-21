/** 🔍️ Round 5 pigeon: where the 2D front (or another view) of one scale disagrees with its 3D body: extra (drawn, no body) and missing (body, not drawn) area per height band and lateral side, from the same rasters as the silhouette rule. Usage: bun r5-pigeon-frontdiff.ts <scale> [view]
 * @see ./volumes-silhouette.ts */
import { drawnSet } from "./volumes-silhouette.ts";
import { addMeshes, emptySet, rasterize } from "./volumes-tiles.ts";
import { entryOf, loadSource, merged, scaleMeshes, type View } from "./volumes-lib.ts";

const [scale, view] = [Number(process.argv[2] ?? 200), (process.argv[3] ?? "front") as View];
const { entry, asset } = entryOf("animals/pigeon");
const source = asset;
const m = merged(asset, source);
const sm = scaleMeshes(m, scale, source.volumes[String(scale)]);
const vol = emptySet();
addMeshes(vol, view, sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true));
const drawn = drawnSet(m, scale, view, vol);
const box = { minX: -0.2, minY: -0.01, maxX: 0.2, maxY: 0.3 };
const cell = 0.001;
const [a, b] = [rasterize(drawn, box, cell), rasterize(vol, box, cell)];
const w = a.width ?? Math.round((box.maxX - box.minX) / cell);
const rows: Record<string, [number, number]> = {};
let [ex, mi, both] = [0, 0, 0];
for (let i = 0; i < a.cells.length; i++) {
  const [d, v] = [a.cells[i]!, b.cells[i]!];
  if (d && v) both++;
  if (d === v) continue;
  const y = box.minY + Math.floor(i / w) * cell;
  const x = box.minX + (i % w) * cell;
  const key = `y ${(Math.floor(y / 0.02) * 20).toString().padStart(3)}-${Math.floor(y / 0.02) * 20 + 20} mm ${x < 0 ? "neg" : "pos"}`;
  rows[key] ??= [0, 0];
  if (d) { rows[key]![0]++; ex++; } else { rows[key]![1]++; mi++; }
}
console.log(`[DEBUG] 1:${scale} ${view} iou ${(both / (both + ex + mi)).toFixed(3)} extra ${(ex * 1e-6 * 1e4).toFixed(1)} cm2 missing ${(mi * 1e-6 * 1e4).toFixed(1)} cm2`);
for (const [k, [e, n]] of Object.entries(rows).sort()) if (e + n > 20) console.log(`[DEBUG] ${k} extra ${e} missing ${n}`);
