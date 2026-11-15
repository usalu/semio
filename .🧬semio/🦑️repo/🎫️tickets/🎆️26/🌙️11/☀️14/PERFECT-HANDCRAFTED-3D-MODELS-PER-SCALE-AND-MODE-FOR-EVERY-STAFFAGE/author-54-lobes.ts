import { existsSync, readFileSync, writeFileSync } from "node:fs";

import { drawn, featureM, openCtx, type Json } from "./kit-lib.ts";
import { Mask, meshMask, meshesAt } from "./volumes-lib.ts";

/**
 * 🫧️ Lobe fitter: ellipsoid masses (centre, radii, yaw, roll) of one bone fitted to the three drawn views of one scale by annealing, starting from k-means clusters of the visual hull of the drawings.
 * Usage: bun author-54-lobes.ts <asset> <scale> --n N --rounds R [--bone root-plate] [--fixed file.json] [--out file.json] [--seed S] [--reset] [--pick regex] [--material m] [--ids prefix] [--bbox]
 */
const args = process.argv.slice(2);
const flag = (n: string, d?: string): string | undefined => (args.includes(n) ? args[args.indexOf(n) + 1] : d);
const spec = args[0]!;
const scale = Number(args[1]);
const N = Number(flag("--n", "6"));
const rounds = Number(flag("--rounds", "4000"));
const bone = flag("--bone", "root-plate")!;
const material = flag("--material", "mass")!;
const prefix = flag("--ids", "lobe")!;
const out = flag("--out", `🗑️generated/author-54/${spec.replace("/", "-")}.s${scale}.lobes.json`)!;
const fixed: Json[] = flag("--fixed") !== undefined ? JSON.parse(readFileSync(flag("--fixed")!, "utf8")) : [];
const c = openCtx(spec, scale);
const F = featureM(scale);
const dims = c.m.dimensions;
const tone = Number(flag("--tone", "0.5"));
const rMax = Number(flag("--rmax", "1.5"));

const pickRe = flag("--pick") === undefined ? undefined : new RegExp(flag("--pick")!);
const shapesOf = (v: "elevation" | "front" | "plan") => drawn(c, v).filter((d) => pickRe === undefined || pickRe.test(d.id));

const cell = Number(flag("--cell", String(Math.max(F / 3, 0.01))));
const frame = (v: string) => {
  const rings = shapesOf(v as never).flatMap((d) => d.rings.flat());
  const xs = rings.map((p) => p.x), ys = rings.map((p) => p.y);
  return { minX: Math.min(...xs) - 4 * cell, maxX: Math.max(...xs) + 4 * cell, minY: Math.min(...ys) - 4 * cell, maxY: Math.max(...ys) + 4 * cell };
};
const views = ["elevation", "front", "plan"] as const;
const truth: Record<string, Mask> = {};
const allTruth: Record<string, Mask> = {};
for (const v of views) {
  const bx = frame(v);
  const m = new Mask(bx, cell);
  for (const d of shapesOf(v)) m.fill(d.rings);
  truth[v] = m;
  const m2 = new Mask(bx, cell);
  for (const d of drawn(c, v)) m2.fill(d.rings);
  allTruth[v] = m2;
}

type Lobe = { c: [number, number, number]; r: [number, number, number]; yaw: number; roll: number };
const q = (yaw: number, roll: number): [number, number, number, number] => {
  const [cy, sy] = [Math.cos(yaw / 2), Math.sin(yaw / 2)];
  const [cz, sz] = [Math.cos(roll / 2), Math.sin(roll / 2)];
  return [sy * sz, sy * cz, cy * sz, cy * cz];
};
const solid = (l: Lobe, i: number): Json => ({ id: `${prefix}-${i + 1}`, kind: "ellipsoid", bone, material, role: "body", tone, radii: l.r.map((x) => Math.round(x * 1e4) / 1e4), transform: { translation: l.c.map((x) => Math.round(x * 1e4) / 1e4), rotation: q(l.yaw, l.roll).map((x) => Math.round(x * 1e5) / 1e5) } });

const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
function init(): Lobe[] {
  const g = 0.025;
  const [el, fr, pl] = [truth.elevation!, truth.front!, truth.plan!];
  const at = (m: Mask, x: number, y: number): number => { const i = Math.floor((x - m.box.minX) / m.cell), j = Math.floor((y - m.box.minY) / m.cell); return i < 0 || j < 0 || i >= m.width || j >= m.height ? 0 : m.cells[j * m.width + i]!; };
  const vox: [number, number, number][] = [];
  const bb = { x: [el.box.minX, el.box.maxX], y: [el.box.minY, el.box.maxY], z: [fr.box.minX, fr.box.maxX] };
  for (let x = bb.x[0]!; x < bb.x[1]!; x += g) for (let y = bb.y[0]!; y < bb.y[1]!; y += g) for (let z = bb.z[0]!; z < bb.z[1]!; z += g) if (at(el, x, y) && at(fr, z, y) && at(pl, x, z)) vox.push([x, y, z]);
  if (vox.length < N) throw new Error(`visual hull has ${vox.length} voxels`);
  let rs = 99991;
  const rr = () => ((rs = (rs * 1664525 + 1013904223) % 4294967296) / 4294967296);
  let cent = Array.from({ length: N }, () => vox[Math.floor(rr() * vox.length)]!.slice() as [number, number, number]);
  for (let it = 0; it < 12; it++) {
    const sums = cent.map(() => [0, 0, 0, 0]);
    for (const v of vox) { let b = 0, bd = 1e9; cent.forEach((cc, k) => { const d = (cc[0] - v[0]) ** 2 + (cc[1] - v[1]) ** 2 + (cc[2] - v[2]) ** 2; if (d < bd) [bd, b] = [d, k]; }); sums[b]![0]! += v[0]; sums[b]![1]! += v[1]; sums[b]![2]! += v[2]; sums[b]![3]!++; }
    cent = cent.map((cc, k) => (sums[k]![3]! > 0 ? [sums[k]![0]! / sums[k]![3]!, sums[k]![1]! / sums[k]![3]!, sums[k]![2]! / sums[k]![3]!] : cc));
  }
  return cent.map((cc) => {
    const near = vox.filter((v) => cent.every((o) => (o[0] - v[0]) ** 2 + (o[1] - v[1]) ** 2 + (o[2] - v[2]) ** 2 >= (cc[0] - v[0]) ** 2 + (cc[1] - v[1]) ** 2 + (cc[2] - v[2]) ** 2 - 1e-12));
    const sd = [0, 1, 2].map((k) => Math.sqrt(near.reduce((a, v) => a + (v[k]! - cc[k]!) ** 2, 0) / Math.max(1, near.length)) * 1.9 + F * 0.6);
    return { c: cc, r: sd as [number, number, number], yaw: 0, roll: 0 };
  });
}

let lastBox = [0, 0, 0];
function score(lobes: Lobe[]): number[] {
  const list = [...fixed, ...lobes.map(solid)];
  const ms = meshesAt(c.m, scale, list);
  lo.fill(1e9); hi.fill(-1e9);
  for (const m of ms) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]!); hi[k] = Math.max(hi[k]!, m.world[i + k]!); }
  lastBox = [hi[0]! - lo[0]! - dims.widthM, hi[1]! - lo[1]! - dims.heightM, hi[2]! - lo[2]! - dims.depthM];
  return views.map((v) => {
    const t = allTruth[v]!;
    const m = meshMask(v, ms, t.box, cell);
    let i = 0, u = 0;
    for (let k = 0; k < t.cells.length; k++) { const a = m.cells[k]!, b = t.cells[k]!; if (a && b) i++; if (a || b) u++; }
    return i / u;
  });
}
const pen = (): number => lastBox.reduce((a, d, k) => a + Math.max(0, Math.abs(d) - (args.includes("--bbox") ? 0.02 : 0.04) * [dims.widthM, dims.heightM, dims.depthM][k]!) * 6, 0);
const obj = (s: number[]): number => -pen() + 0.7 * s[0]! + 1.2 * s[1]! + 1.2 * s[2]! + 1.5 * (Math.min(s[0]!, 0.9) + Math.min(s[1]!, 0.88) + Math.min(s[2]!, 0.88));

let lobes: Lobe[] = existsSync(out) && !args.includes("--reset") ? JSON.parse(readFileSync(out, "utf8")).lobes : init();
let best = score(lobes);
let bo = obj(best);
let top = { lobes, s: best };
let topO = bo;
console.log("start", best.map((x) => x.toFixed(3)).join(" "));
let seed = Number(flag("--seed", "4242"));
const rnd = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
for (let r = 0; r < rounds; r++) {
  const step = 0.1 * (1 - r / rounds) + 0.01;
  const cand: Lobe[] = lobes.map((l) => ({ ...l, c: [...l.c] as Lobe["c"], r: [...l.r] as Lobe["r"] }));
  const l = cand[Math.floor(rnd() * cand.length)]!;
  const k = Math.floor(rnd() * 8);
  const d = (rnd() - 0.5) * 2 * step;
  if (k < 3) l.c[k] += d * dims.heightM;
  else if (k < 6) l.r[k - 3] = Math.min(rMax, Math.max(F * 0.6, l.r[k - 3]! + d * dims.heightM * 0.5));
  else if (k === 6) l.yaw += d * 3;
  else l.roll += d * 3;
  const s = score(cand);
  const o = obj(s);
  const temp = 0.05 * (1 - r / rounds) ** 2;
  if (o > bo || rnd() < Math.exp((o - bo) / Math.max(temp, 1e-9))) {
    lobes = cand; best = s; bo = o;
    if (o > topO) { topO = o; top = { lobes: cand, s }; }
  }
}
lobes = top.lobes;
best = top.s;
writeFileSync(out, JSON.stringify({ lobes, solids: lobes.map(solid), iou: best }));
console.log("end", best.map((x) => x.toFixed(3)).join(" "), "bbox dev", lastBox.map((x) => x.toFixed(3)).join(" "));
