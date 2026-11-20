/** 🩻️ Round 3 pigeon: which structure solid leaves the body silhouette, per view (largest distance of a vertex outside the projected body in millimetres). Usage: bun r3-pigeon-inside.ts <scale> */
import { meshesAt, projectWorld, type View } from "./volumes-lib.ts";
import { projectMask } from "./r3-project2d.ts";
import { openCtx } from "./kit-lib.ts";

const scale = Number(process.argv[2] ?? 50);
const c = openCtx("animals/pigeon", scale);
const body = meshesAt(c.m, scale, c.vol.body);
const structure = meshesAt(c.m, scale, c.vol.structure);
for (const view of ["elevation", "front", "plan"] as View[]) {
  const mask = projectMask(view, body, c.feature / 20, 60)!;
  const inside: [number, number][] = [];
  for (let y = 0; y < mask.height; y++) for (let x = 0; x < mask.width; x++) if (mask.cells[y * mask.width + x] === 1) inside.push([x, y]);
  const rows: string[] = [];
  for (const m of structure) {
    let worst = 0;
    for (let i = 0; i < m.world.length; i += 3) {
      const p = projectWorld(view, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]);
      const [px, py] = [(p.x - mask.box.minX) / mask.cell, (p.y - mask.box.minY) / mask.cell];
      if (mask.cells[Math.floor(py) * mask.width + Math.floor(px)] === 1) continue;
      let best = Infinity;
      for (let k = 0; k < inside.length; k += 7) best = Math.min(best, Math.hypot(inside[k]![0] - px, inside[k]![1] - py));
      worst = Math.max(worst, best * mask.cell);
    }
    if (worst > c.feature * 0.5) rows.push(`${m.id} ${(worst * 1000).toFixed(1)}`);
  }
  console.log(view, rows.join(", ") || "all inside");
}
