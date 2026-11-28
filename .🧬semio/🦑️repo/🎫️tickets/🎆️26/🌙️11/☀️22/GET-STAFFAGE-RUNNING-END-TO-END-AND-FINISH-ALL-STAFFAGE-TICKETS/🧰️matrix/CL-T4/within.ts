/** 🫥️ Which structure solids of an asset lie wholly inside other structure solids: per scale and pose every solid of the `structure` list at the reference facets in the world; a solid is `enclosed` when every vertex and every facet centre of it, pushed outwards from its centroid by `--grow` (default 1.02), lies inside one other real member solid of the list (ray casting against its closed facets), in every named pose of the asset. Such a solid shows from no direction and in no cut-free view; merging it into the member that holds it changes no outline. within.ts <asset id | file.json> [--parts joint,mass] [--grow 1.02] [--out <ops.json>] prints the enclosed and the free solids per scale with the member that holds most of each and writes the `remove` ops of `facet.ts`.
 * @see ./facet.ts */
import { writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { readDoc } from "../MR/lib.ts";

const { values, positionals } = parseArgs({ options: { out: { type: "string" }, parts: { type: "string" }, grow: { type: "string" }, poses: { type: "string" } }, allowPositionals: true });
const asset = readDoc(positionals[0]!).doc.asset;
const parts = new Set((values.parts ?? "joint,mass").split(","));
const grow = Number(values.grow ?? 1.02);
const poses: string[] = values.poses === undefined ? asset.poses.map((p: { id: string }) => p.id) : values.poses.split(",");
type Mesh = { id: string; positions: Float64Array; indices: Uint32Array; box: number[] };
const D = [0.5773502691896258, 0.21132486540518713, 0.7886751345948129];

/** 🎯️ Whether a point lies inside a closed triangle mesh: the parity of the crossings of a ray in a fixed skew direction. */
function inside(p: number[], m: Mesh): boolean {
  if (p[0]! < m.box[0]! || p[1]! < m.box[1]! || p[2]! < m.box[2]! || p[0]! > m.box[3]! || p[1]! > m.box[4]! || p[2]! > m.box[5]!) return false;
  let hits = 0;
  const q = m.positions;
  for (let t = 0; t < m.indices.length; t += 3) {
    const [a, b, c] = [m.indices[t]! * 3, m.indices[t + 1]! * 3, m.indices[t + 2]! * 3];
    const [e1x, e1y, e1z, e2x, e2y, e2z] = [q[b]! - q[a]!, q[b + 1]! - q[a + 1]!, q[b + 2]! - q[a + 2]!, q[c]! - q[a]!, q[c + 1]! - q[a + 1]!, q[c + 2]! - q[a + 2]!];
    const [hx, hy, hz] = [D[1]! * e2z - D[2]! * e2y, D[2]! * e2x - D[0]! * e2z, D[0]! * e2y - D[1]! * e2x];
    const det = e1x * hx + e1y * hy + e1z * hz;
    if (Math.abs(det) < 1e-18) continue;
    const [sx, sy, sz] = [p[0]! - q[a]!, p[1]! - q[a + 1]!, p[2]! - q[a + 2]!];
    const u = (sx * hx + sy * hy + sz * hz) / det;
    if (u < 0 || u > 1) continue;
    const [qx, qy, qz] = [sy * e1z - sz * e1y, sz * e1x - sx * e1z, sx * e1y - sy * e1x];
    const v = (D[0]! * qx + D[1]! * qy + D[2]! * qz) / det;
    if (v < 0 || u + v > 1) continue;
    if ((e2x * qx + e2y * qy + e2z * qz) / det > 0) hits += 1;
  }
  return hits % 2 === 1;
}

const ops: unknown[] = [];
for (const scale of [50, 100, 200, 500]) {
  const list = asset.volumes[`${scale}`].structure as { id: string; part: string; kind?: string; member: string }[];
  const label = new Map(list.map((s) => [s.id, s]));
  const holders = new Map<string, Map<string, number>>();
  const free = new Map<string, string>();
  for (const pose of poses) {
    const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } } as never);
    const body = worldSolids(asset, state.bones as never, scale);
    const meshes: Mesh[] = worldSolids(asset, state.bones as never, scale, "structure", body).map((w) => {
      const box = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
      for (let i = 0; i < w.positions.length; i++) { box[i % 3] = Math.min(box[i % 3]!, w.positions[i]!); box[3 + (i % 3)] = Math.max(box[3 + (i % 3)]!, w.positions[i]!); }
      return { id: w.id, positions: w.positions, indices: w.indices, box };
    });
    const members = meshes.filter((m) => label.get(m.id)?.part === "member");
    for (const m of meshes) {
      if (!parts.has(label.get(m.id)?.part ?? "") || free.has(m.id)) continue;
      const c = [0, 0, 0];
      for (let i = 0; i < m.positions.length; i++) c[i % 3]! += m.positions[i]! / (m.positions.length / 3);
      const probes: number[][] = [];
      for (let i = 0; i < m.positions.length; i += 3) probes.push([0, 1, 2].map((k) => c[k]! + (m.positions[i + k]! - c[k]!) * grow));
      for (let t = 0; t < m.indices.length; t += 3) probes.push([0, 1, 2].map((k) => c[k]! + ((m.positions[m.indices[t]! * 3 + k]! + m.positions[m.indices[t + 1]! * 3 + k]! + m.positions[m.indices[t + 2]! * 3 + k]!) / 3 - c[k]!) * grow));
      const near = members.filter((o) => o.id !== m.id && o.box[0]! < m.box[3]! && o.box[3]! > m.box[0]! && o.box[1]! < m.box[4]! && o.box[4]! > m.box[1]! && o.box[2]! < m.box[5]! && o.box[5]! > m.box[2]!);
      const count = (holders.get(m.id) ?? holders.set(m.id, new Map()).get(m.id)!);
      let out = 0;
      for (const p of probes) { const holder = near.find((o) => inside(p, o)); if (holder === undefined) out += 1; else count.set(holder.id, (count.get(holder.id) ?? 0) + 1); }
      if (out > 0) free.set(m.id, `${out} of ${probes.length} probes outside in ${pose}`);
    }
  }
  const enclosed = list.filter((s) => parts.has(s.part) && !free.has(s.id) && holders.has(s.id));
  console.log(`1:${scale}: ${list.filter((s) => parts.has(s.part)).length} solids of ${[...parts].join(", ")}, ${enclosed.length} enclosed in ${poses.length} poses`);
  for (const s of enclosed) console.log(`  enclosed ${s.id} (${s.part} of ${s.member}) in ${[...holders.get(s.id)!].sort((a, b) => b[1] - a[1]).slice(0, 3).map(([id, n]) => `${id} ${n}`).join(", ")}`);
  for (const [id, why] of free) console.log(`  free ${id} (${label.get(id)!.part} of ${label.get(id)!.member}): ${why}`);
  if (enclosed.length > 0) ops.push({ remove: enclosed.map((s) => s.id.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|"), list: "structure", scales: [scale] });
}
if (values.out !== undefined) writeFileSync(values.out, JSON.stringify(ops, null, 1) + "\n");
