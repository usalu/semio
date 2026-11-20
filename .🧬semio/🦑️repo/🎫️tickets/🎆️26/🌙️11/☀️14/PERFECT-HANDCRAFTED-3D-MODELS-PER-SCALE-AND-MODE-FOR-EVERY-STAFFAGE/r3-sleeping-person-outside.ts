/** 🚧️ Which structure solids of a scale stick out of the body silhouette, per view, and by how much (vertices outside the body mask, distance in cells as a lower bound): a debug aid for `volumes-structure-coverage`.
 * Usage: `bun r3-sleeping-person-outside.ts <scale> [document]`
 * @see ./r3-project2d.ts */
import { readFileSync } from "node:fs";
import { projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt } from "./r3-lib.ts";
import { projectWorld, type View } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 50);
const asset = JSON.parse(readFileSync(process.argv[3] ?? "🗑️generated/r3-sleeping-person/🔣️.json", "utf8")).asset;
const frames = rigAt(asset, scale, { kind: "pose", pose: "supine" }).frames;
const body = meshesIn(asset, frames, (asset.volumes[scale].body as any[]).filter((s) => s.slender !== true));
const structure = (asset.volumes[scale].structure as any[]).filter((s) => s.of === undefined);
const cell = 0.002;
for (const view of ["elevation", "front", "plan"] as View[]) {
  const m: any = projectMask(view, body, cell, 40)!;
  const inside = (x: number, y: number): boolean => {
    const i = Math.floor((x - m.box.minX) / cell), j = Math.floor((y - m.box.minY) / cell);
    return i >= 0 && j >= 0 && i < m.width && j < m.height && m.cells[j * m.width + i] === 1;
  };
  const out: string[] = [];
  for (const s of meshesIn(asset, frames, structure)) {
    let worst = 0;
    for (let i = 0; i < s.world.length; i += 3) {
      const p = projectWorld(view, [s.world[i]!, s.world[i + 1]!, s.world[i + 2]!]);
      if (inside(p.x, p.y)) continue;
      let d = 0;
      while (d < 0.1 && ![[d, 0], [-d, 0], [0, d], [0, -d]].some(([a, b]) => inside(p.x + a!, p.y + b!))) d += cell;
      worst = Math.max(worst, d);
    }
    if (worst > 0.004) out.push(`${(s as any).solid?.id ?? (s as any).id}:${(worst * 1000).toFixed(0)}mm`);
  }
  console.log(view, out.join(" "));
}
