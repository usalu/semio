/** 🔎️ Where the drawing and the body of `humans/kneeling-gardener` disagree in one view: the cells of each silhouette farther than a limit from the other silhouette, as boxes in view coordinates (owned raster of `volumes-lib.ts`, catalog asset with the source merged over it).
 * Usage: `bun r3-kneeling-gardener-diag.ts <scale> <elevation|front|plan> [limitM]` */
import { distanceField, entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks } from "./volumes-lib.ts";
const { entry, asset } = entryOf("humans/kneeling-gardener");
const m = merged(asset, loadSource(entry)!);
const scale = Number(process.argv[2]);
const view = process.argv[3] as "elevation" | "front" | "plan";
const limit = Number(process.argv[4] ?? 0.03);
const st = poseState(m, scale);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const { truth, volume } = viewMasks(view, st, sm.body);
for (const [name, from, to] of [["drawn only", truth, volume], ["3D only", volume, truth]] as const) {
  const field = distanceField(to);
  const far: [number, number][] = [];
  for (let i = 0; i < from.cells.length; i++) if (from.cells[i]! && field[i]! > limit) far.push([i % from.width, Math.floor(i / from.width)]);
  const xs = far.map((p) => truth.box.minX + p[0] * truth.cell);
  const ys = far.map((p) => truth.box.minY + p[1] * truth.cell);
  console.log(name, far.length, "cells beyond", limit, far.length === 0 ? "" : `x ${Math.min(...xs).toFixed(3)}..${Math.max(...xs).toFixed(3)} y ${Math.min(...ys).toFixed(3)}..${Math.max(...ys).toFixed(3)}`);
}
if (process.argv[5] === "shapes") for (const s of st[view] as { shape: { id: string }; rings: { x: number; y: number }[][] }[]) {
  const p = s.rings.flat();
  if (p.length > 0) console.log(s.shape.id.padEnd(22), "x", Math.min(...p.map((q) => q.x)).toFixed(3), Math.max(...p.map((q) => q.x)).toFixed(3), "y", Math.min(...p.map((q) => q.y)).toFixed(3), Math.max(...p.map((q) => q.y)).toFixed(3));
}
