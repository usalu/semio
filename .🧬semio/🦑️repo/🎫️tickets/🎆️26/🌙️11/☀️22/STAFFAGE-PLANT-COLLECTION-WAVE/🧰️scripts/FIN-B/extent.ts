/** 📐️ Finisher diagnosis: the extents of one body solid and of the drawn shapes of the same id in every view of the default pose at a scale (paper x and y of the view, metres), to see by how much a drawing is wider or narrower than its solid.
 *
 * `repo tools exec bun -- extent.ts <asset document json> <scale> [<solid id> [<shape id>]]` (without a solid id: the solids that reach farthest on every world axis)
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWPOINTS, project, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [file, scaleText, solidId, shapeId = solidId] = process.argv.slice(2) as [string, string, string?, string?];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const scale = Number(scaleText);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
const worlds = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
if (solidId === undefined) {
  const rows = worlds.map((w) => { const c = [0, 1, 2].map((k) => { const v: number[] = []; for (let i = k; i < w.positions.length; i += 3) v.push(w.positions[i]!); return [Math.min(...v), Math.max(...v)]; }); return { id: w.id, c }; });
  for (const [k, name] of [[0, "x"], [1, "y"], [2, "z"]] as const) console.log(`[DEBUG] ${name}: lowest ${rows.slice().sort((a, b) => a.c[k]![0]! - b.c[k]![0]!).slice(0, 3).map((r) => `${r.id} ${r.c[k]![0]!.toFixed(4)}`).join(", ")} | highest ${rows.slice().sort((a, b) => b.c[k]![1]! - a.c[k]![1]!).slice(0, 3).map((r) => `${r.id} ${r.c[k]![1]!.toFixed(4)}`).join(", ")}`);
  process.exit(0);
}
const solid = worlds.find((w) => w.id === solidId)!;
const span = (values: number[]) => `${Math.min(...values).toFixed(4)}..${Math.max(...values).toFixed(4)}`;
for (const view of VIEWPOINTS) {
  const pts = Array.from({ length: solid.positions.length / 3 }, (_, i) => project(view, solid.positions[3 * i]!, solid.positions[3 * i + 1]!, solid.positions[3 * i + 2]!));
  const drawn = (state.shapes[view] as never as { shape: { id: string }; rings: { x: number; y: number }[][] }[]).filter((s) => s.shape.id === shapeId).flatMap((s) => s.rings.flat());
  console.log(`[DEBUG] ${view}: solid ${solidId} x ${span(pts.map((p) => p.x))} y ${span(pts.map((p) => p.y))} | drawn ${shapeId} ${drawn.length === 0 ? "none" : `x ${span(drawn.map((p) => p.x))} y ${span(drawn.map((p) => p.y))}`}`);
}
