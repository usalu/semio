import { writeFileSync } from "node:fs";

import { hullLoft, openCtx, featureM, type Json } from "./kit-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/bamboo-clump", scale);
const F = featureM(scale);
const MAT = "bamboo-culm";
const R: Record<string, [number, number]> = { a: [0.0145, 0.0135], b: [0.0135, 0.0115], c: [0.0115, 0.008] };
const WALL = 0.004;
const bones = c.raw.rig.bones.filter((b: Json) => /^culm-\d-[abc]$/.test(b.id));
const solids: Json[] = [];
const front = c.raw.representations[String(scale)].geometry.front;
const zOf = (id: string): [number, number] | null => {
  const sh = front.find((x: Json) => x.id === id);
  if (sh === undefined || sh.points === undefined || sh.points.length < 4) return null;
  const pts = [...sh.points].sort((p: number[], q: number[]) => p[1]! - q[1]!);
  const lo = pts.slice(0, 2), hi = pts.slice(-2);
  return [(lo[0]![0]! + lo[1]![0]!) / 2, (hi[0]![0]! + hi[1]![0]!) / 2];
};
for (const b of bones) {
  const L: number = b.lengthM;
  const [r0, r1] = R[b.id.slice(-1)]!;
  const rAt = (s: number): number => r0 + (r1 - r0) * (s / L);
  const z = zOf(b.id);
  const place = z === null ? {} : { transform: { translation: [0, 0, Math.round(z[0] * 1e4) / 1e4], rotation: [0, Math.round(Math.sin(-Math.atan2(z[1] - z[0], L) / 2) * 1e5) / 1e5, 0, Math.round(Math.cos(-Math.atan2(z[1] - z[0], L) / 2) * 1e5) / 1e5] } };
  if (scale === 50 || scale === 100) {
    const step = scale === 50 ? 0.22 : 0.3;
    const nodes: number[] = [];
    for (let s = step; s < L - 0.05; s += step) nodes.push(s);
    const outer: [number, number][] = [[0, rAt(0)]];
    for (const s of nodes) outer.push([s - 0.008, rAt(s - 0.008)], [s - 0.005, rAt(s) + 0.002], [s + 0.005, rAt(s) + 0.002], [s + 0.008, rAt(s + 0.008)]);
    outer.push([L, rAt(L)]);
    const inner: [number, number][] = [[L, rAt(L) - WALL], [0, rAt(0) - WALL]];
    solids.push({ id: b.id, kind: "revolve", bone: b.id, material: MAT, role: "body", tone: 0.35, slender: true, ...place, axis: [1, 0, 0], origin: [0, 0, 0], profile: [...outer, ...inner].map(([u, r]) => [Math.round(u * 1e5) / 1e5, Math.round(r * 1e5) / 1e5]) });
  } else {
    solids.push({ id: b.id, kind: "revolve", bone: b.id, material: MAT, role: "body", tone: 0.35, slender: true, ...place, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[0, 0], [0, rAt(0)], [L, rAt(L)], [L, 0]] });
  }
}
if (scale === 500) {
  solids.length = 0;
  const h = hullLoft(c, "culm-3-a", "u", { id: "stems", material: MAT, tone: 0.35 }, { stations: 5, e: 2.4, tol: F * 0.5 });
  if (h !== null) solids.push(h);
}
writeFileSync(`🗑️generated/author-54/bamboo.s${scale}.culms.json`, JSON.stringify(solids));
console.log(solids.length, "culm solids");
