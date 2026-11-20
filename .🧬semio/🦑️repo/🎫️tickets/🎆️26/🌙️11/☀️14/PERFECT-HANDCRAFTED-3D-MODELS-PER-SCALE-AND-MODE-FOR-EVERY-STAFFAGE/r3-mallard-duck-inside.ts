/** 🩻️ Round 3 mallard: which structure and thermal solids leave the body silhouette, per view (vertices of each solid outside the projected body, with the largest distance to it). Usage: bun r3-mallard-duck-inside.ts <scale> [structure|thermal] */
import { localGeometry, projectWorld, type SolidMesh, type View } from "./volumes-lib.ts";
import { projectMask } from "./r3-project2d.ts";
import { Bench, type Json, type V3 } from "./r3-mallard-duck-kit.ts";

const scale = Number(process.argv[2] ?? 50);
const list = process.argv[3] ?? "structure";
const S = new Bench(scale);
const mesh = (solid: Json): SolidMesh => {
  const fr = S.frame(solid.bone);
  const g = localGeometry(solid, { chordM: 0.0005, minSegments: 16, maxSegments: 48 });
  const world = new Float64Array(g.positions.length);
  for (let i = 0; i < g.positions.length; i += 3) for (let k = 0; k < 3; k++) world[i + k] = (fr.o as V3)[k]! + g.positions[i]! * (fr.u as V3)[k]! + g.positions[i + 1]! * (fr.v as V3)[k]! + g.positions[i + 2]! * (fr.w as V3)[k]!;
  return { id: solid.id, role: solid.role, world, indices: g.indices } as SolidMesh;
};
const body = (S.c.vol.body as Json[]).map(mesh);
for (const view of ["elevation", "front", "plan"] as View[]) {
  const m = projectMask(view, body, 0.0005, 40)!;
  const inside = (x: number, y: number): boolean => {
    const [cx, cy] = [Math.floor((x - m.box.minX) / m.cell), Math.floor((y - m.box.minY) / m.cell)];
    return cx >= 0 && cy >= 0 && cx < m.width && cy < m.height && m.cells[cy * m.width + cx] === 1;
  };
  const out: string[] = [];
  for (const s of (S.c.vol[list] as Json[]).filter((q) => q.of === undefined)) {
    const w = mesh(s).world;
    let worst = 0;
    for (let i = 0; i < w.length; i += 3) {
      const p = projectWorld(view, [w[i]!, w[i + 1]!, w[i + 2]!]);
      if (inside(p.x, p.y)) continue;
      let d = 0.03;
      for (let r = 1; r <= 60 && d === 0.03; r++) for (let a = 0; a < 24; a++) if (inside(p.x + r * m.cell * Math.cos((a * Math.PI) / 12), p.y + r * m.cell * Math.sin((a * Math.PI) / 12))) { d = r * m.cell; break; }
      worst = Math.max(worst, d);
    }
    if (worst > 0.0008) out.push(`${s.id} ${(worst * 1000).toFixed(1)}`);
  }
  console.log(`1:${scale} ${list} ${view}: ${out.join(", ") || "all inside"}`);
}
