/** 📦 Round 3 london plane: the world bounding box of every foliage group per scale (x, y, z), to keep the coarser scales on the 1:50 crown. */
import { meshesAt } from "./volumes-lib.ts";
import { REGS, asset, build } from "./r3-london-plane-pollarded-model.ts";
const lists = build();
const rows: Record<string, Record<string, string>> = {};
for (const scale of ["50", "100", "200"]) {
  const ms = new Map(meshesAt({ ...asset, volumes: Object.fromEntries(Object.entries(lists).map(([k, v]) => [k, { status: "authored", denominator: Number(k), ...v }])) }, Number(scale), lists[scale]!.body).map((m) => [m.id, m]));
  const box: Record<string, number[]> = {};
  for (const [id, r] of Object.entries(REGS[scale]!)) {
    if (!r.group || !ms.has(id)) continue;
    const b = (box[r.group] ??= [9, 9, 9, -9, -9, -9]), w = ms.get(id)!.world;
    for (let i = 0; i < w.length; i += 3) for (let a = 0; a < 3; a++) { b[a] = Math.min(b[a]!, w[i + a]!); b[a + 3] = Math.max(b[a + 3]!, w[i + a]!); }
  }
  for (const [g, b] of Object.entries(box)) (rows[g] ??= {})[scale] = `x ${b[0]!.toFixed(2)}..${b[3]!.toFixed(2)} y ${b[1]!.toFixed(2)}..${b[4]!.toFixed(2)} z ${b[2]!.toFixed(2)}..${b[5]!.toFixed(2)}`;
}
for (const [g, r] of Object.entries(rows)) console.log(g.padEnd(7), ["50", "100", "200"].map((s) => `${s}: ${r[s] ?? "-"}`).join(" | "));
