/** 🪴️ Writes `ops/potted-monstera.json`: at 1:100 and 1:200 the nine petioles are strokes, drawn in the front and in the plan as flat lines in world coordinates. Their bones are spread and twisted in 3D and the wilting and draught poses swing them by up to 13 degrees; a flat line is carried at one depth and keeps its direction, so the plan and front petiole left the side elevation by 33 to 34 mm (tolerance 32 mm). Each plan petiole becomes the stroke of the side elevation (the line along the bone) as an `extruded` solid, `sweep: "w"`, 1.5 mm thick about the bone axis, as the hornbeam hedge draws its strokes: the full 3D frame of the bone places it in every pose. Run from the multiview test folder: bun ops-potted-monstera.ts [front,plan] (default plan: the front petioles as such solids fail the silhouette guard, front 66 and 139 mm at 1:100 and 1:200, measured, so the front keeps its lines)
 * @see ./apply.ts */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const asset = readDoc("potted-monstera").doc.asset;
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[] } }>;
const HALF = 0.00075;
const VIEWS = (process.argv[2] ?? "plan").split(",");
const round = (n: number): number => Math.round(n * 1e6) / 1e6;
const ops: any[] = [];
for (const scale of ["100", "200"]) {
  const geometry = asset.representations[scale].geometry;
  for (const view of VIEWS)
    for (const shape of geometry[view]) {
      if (!/^petiole-\d+$/.test(shape.bone) || shape.id !== shape.bone || shape.filled !== false) continue;
      const twin = geometry.elevation.find((candidate: any) => candidate.id === shape.id && candidate.bone === shape.bone);
      const z = rest[shape.bone]!.frame.o[2]!;
      ops.push({ op: "set", scale, view, id: shape.id, bone: shape.bone, set: { kind: twin.kind, points: twin.points, closed: twin.closed, solid: { kind: "extruded", sweep: "w", z0: round(z - HALF), z1: round(z + HALF) } } });
    }
}
const file = join(import.meta.dir, "ops", "potted-monstera.json");
writeFileSync(file, `[\n${ops.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
console.log(`${ops.length} ops in ${file}`);
