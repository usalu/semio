import { entryOf, loadSource, merged, scaleMeshes, featureM } from "./volumes-lib.ts";
import { drawnSet, cellFor } from "./volumes-silhouette.ts";
import { addMeshes, emptySet, tileCompare } from "./volumes-tiles.ts";
const [spec, scaleArg, viewArg] = process.argv.slice(2);
const scale = Number(scaleArg);
const { entry, asset: raw } = entryOf(spec!);
const asset: any = merged(raw, loadSource(entry)!);
const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
const body = sm.body.filter((_: any, i: number) => sm.solids.body[i]!.texture !== true);
const ids = sm.solids.body.filter((s: any) => s.texture !== true).map((s: any) => s.id);
const f = featureM(scale, asset);
const long = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
for (const view of (viewArg ? [viewArg] : ["elevation", "front", "plan"]) as any[]) {
  const drawn = drawnSet(asset, scale, view);
  const run = (list: any[]) => { const v = emptySet(); addMeshes(v, view, list); return tileCompare(drawn, v, { cell: Math.max(f / 3, long / 700), capM: 6 * f }); };
  const base = run(body);
  const inter = (base.iou * (base.cellsA + base.cellsB)) / (1 + base.iou);
  console.log(`== ${view} iou=${base.iou.toFixed(3)} drawnCells=${base.cellsA} bodyCells=${base.cellsB} drawnOnly=${base.cellsA - inter | 0} bodyOnly=${base.cellsB - inter | 0}`);
  const rows: [number, string][] = [];
  for (let i = 0; i < body.length; i++) {
    const r = run(body.filter((_: any, j: number) => j !== i));
    rows.push([r.iou - base.iou, ids[i]]);
  }
  rows.sort((a, b) => b[0] - a[0]);
  console.log("  removing improves:", rows.slice(0, 6).map((r) => `${r[1]} ${r[0].toFixed(3)}`).join(", "));
  console.log("  removing hurts:", rows.slice(-6).map((r) => `${r[1]} ${r[0].toFixed(3)}`).join(", "));
}
