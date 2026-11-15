import { existsSync, readFileSync, writeFileSync } from "node:fs";

import { loftOf, openCtx, saveLists, superellipseSection, featureM, drawn, type Json, type Section, type V3 } from "./kit-lib.ts";
import { Mask, meshMask, meshesAt } from "./volumes-lib.ts";
import { quatOf, spanAt, vcross, vnorm, type Pt, type V } from "./author-54-lib.ts";

const scale = 500;
const c = openCtx("vegetation/agave", scale);
const F = featureM(scale);
const LEAF = "agave-leaf";
const CAUDEX = "agave-caudex";
const PARAMS = "🗑️generated/author-54/agave500.params.json";
const geo = c.raw.representations["500"].geometry;
const shape = (view: string, id: string): Json | undefined => geo[view].find((s: Json) => s.id === id);

type Blade = { id: string; T: [number, number, number]; arch: number; rw: number; rt: number; bx?: number; bz?: number };
const INIT: Blade[] = [
  { id: "blade-r1", T: [1.24, 0.19, -0.19], arch: 0.55, rw: 0.14, rt: 0.05 },
  { id: "blade-l1", T: [-1.11, 0.24, 0.26], arch: 0.5, rw: 0.14, rt: 0.05 },
  { id: "blade-l2", T: [-0.86, 0.36, 0.24], arch: 0.5, rw: 0.12, rt: 0.05 },
  { id: "blade-z1", T: [0.06, 0.74, -1.09], arch: 0.3, rw: 0.12, rt: 0.05 },
  { id: "blade-z2", T: [0.11, 0.71, 1.11], arch: 0.3, rw: 0.12, rt: 0.05 },
  { id: "blade-r2", T: [0.59, 0.79, -0.69], arch: 0.2, rw: 0.12, rt: 0.05 },
  { id: "blade-l3", T: [-0.54, 0.59, 0.74], arch: 0.2, rw: 0.12, rt: 0.05 },
  { id: "blade-r3", T: [0.49, 0.46, 0.41], arch: 0.2, rw: 0.1, rt: 0.05 },
  { id: "blade-l4", T: [-0.44, 0.59, -0.36], arch: 0.2, rw: 0.1, rt: 0.05 },
  { id: "blade-r4", T: [0.59, 0.21, 0.01], arch: 0.3, rw: 0.1, rt: 0.05 },
  { id: "blade-e1", T: [0.25, 1.3, 0.12], arch: 0.1, rw: 0.1, rt: 0.05 },
  { id: "blade-e2", T: [-0.25, 1.28, 0.15], arch: 0.1, rw: 0.1, rt: 0.05 },
  { id: "blade-e3", T: [0.05, 1.25, -0.28], arch: 0.1, rw: 0.1, rt: 0.05 },
  { id: "blade-r5", T: [0.9, 0.5, 0.6], arch: 0.3, rw: 0.1, rt: 0.05 },
  { id: "blade-l5", T: [-0.7, 0.3, -0.5], arch: 0.3, rw: 0.1, rt: 0.05 },
  { id: "blade-e4", T: [0.35, 1.1, -0.5], arch: 0.2, rw: 0.1, rt: 0.05 },
];
const toLocal = (p: V): V3 => [p[1], -p[0], p[2]];

function bladeSolid(b: Blade): Json {
  const B: V = [b.bx ?? 0, 0.33, b.bz ?? 0];
  const T: V = b.T;
  const M: V = [(B[0] + T[0]) / 2, (B[1] + T[1]) / 2 + b.arch, (B[2] + T[2]) / 2];
  const at = (t: number): V => [(1 - t) ** 2 * B[0] + 2 * (1 - t) * t * M[0] + t * t * T[0], (1 - t) ** 2 * B[1] + 2 * (1 - t) * t * M[1] + t * t * T[1], (1 - t) ** 2 * B[2] + 2 * (1 - t) * t * M[2] + t * t * T[2]];
  const dt = (t: number): V => vnorm([2 * (1 - t) * (M[0] - B[0]) + 2 * t * (T[0] - M[0]), 2 * (1 - t) * (M[1] - B[1]) + 2 * t * (T[1] - M[1]), 2 * (1 - t) * (M[2] - B[2]) + 2 * t * (T[2] - M[2])]);
  const hx = T[0] - B[0], hz = T[2] - B[2];
  const hl = Math.hypot(hx, hz) || 1;
  const wdir: V = [-hz / hl, 0, hx / hl];
  const wp = [0.45, 0.85, 1, 0.8, 0.28];
  const tp = [1, 1, 0.9, 0.75, 0.55];
  const sections: Section[] = [0, 0.25, 0.5, 0.75, 1].map((t, i) => {
    const tan = dt(t);
    const d = tan[0] * wdir[0] + tan[2] * wdir[2];
    const z = vnorm([tan[0] - d * wdir[0], tan[1], tan[2] - d * wdir[2]]);
    const y = vcross(z, wdir);
    return { at: toLocal(at(t)), rx: Math.max(b.rw * wp[i]!, F * 0.55), ry: Math.max(b.rt * tp[i]!, F * 0.55), exponent: 3, rotation: quatOf(toLocal(wdir), toLocal(y), toLocal(vcross(wdir, y))) };
  });
  return loftOf({ id: b.id, bone: "caudex", material: LEAF, tone: 0.45 }, sections);
}

const fixed: Json[] = [];
{
  const e = shape("elevation", "caudex")!.points as Pt[];
  const f = shape("front", "caudex")!.points as Pt[];
  const secs: Section[] = [0, 0.21, 0.42].map((u, i) => {
    const se = spanAt(e, 0, Math.min(0.4199, Math.max(1e-4, u)))!;
    const sf = spanAt(f, 1, Math.min(0.4199, Math.max(1e-4, u)))!;
    return superellipseSection([u, 0, 0], ((sf[1] - sf[0]) / 2) * (i === 2 ? 0.95 : 1), ((se[1] - se[0]) / 2) * (i === 2 ? 0.95 : 1), 2.4);
  });
  fixed.push(loftOf({ id: "caudex", bone: "caudex", material: CAUDEX, tone: 0.3 }, secs));
  const eh = shape("elevation", "heart-0")!.points as Pt[];
  const fh = shape("front", "heart")!.points as Pt[];
  const hs: Section[] = [];
  for (let i = 0; i <= 6; i++) {
    const u = 0.05 + (1.1 * i) / 6;
    const se = spanAt(eh, 0, Math.min(1.109, u)), sf = spanAt(fh, 1, 0.3 + u);
    hs.push(superellipseSection([u, 0, 0], Math.max((((sf?.[1] ?? 0) - (sf?.[0] ?? 0)) / 2) * 1.4, F * 0.55), Math.max((((se?.[1] ?? 0) - (se?.[0] ?? 0)) / 2) * 0.6, F * 0.55), 2.2));
  }
  fixed.push(loftOf({ id: "leaf-s", bone: "leaf-s", material: LEAF, tone: 0.35 }, hs));
}

const cell = 0.025;
const bx = { minX: -1.5, maxX: 1.5, minY: -0.2, maxY: 1.6 };
const truth: Record<string, Mask> = {};
for (const v of ["elevation", "front", "plan"] as const) {
  const m = new Mask(v === "plan" ? { minX: -1.5, maxX: 1.5, minY: -1.5, maxY: 1.5 } : bx, cell);
  for (const d of drawn(c, v)) m.fill(d.rings);
  truth[v] = m;
}
let lastBox = [0, 0, 0];
function score(blades: Blade[]): number[] {
  const list = [...fixed, ...blades.map(bladeSolid)];
  const ms = meshesAt(c.m, scale, list);
  const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (const m of ms) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]!); hi[k] = Math.max(hi[k]!, m.world[i + k]!); }
  lastBox = [hi[0]! - lo[0]! - 2.52, hi[1]! - lo[1]! - 1.43, hi[2]! - lo[2]! - 2.4];
  return (["elevation", "front", "plan"] as const).map((v) => {
    const t = truth[v]!;
    const m = meshMask(v, ms, t.box, cell);
    let i = 0, u = 0;
    for (let k = 0; k < t.cells.length; k++) { const a = m.cells[k]!, b = t.cells[k]!; if (a && b) i++; if (a || b) u++; }
    return i / u;
  });
}
const pen = (): number => lastBox.reduce((a, d) => a + Math.max(0, Math.abs(d) - 0.05) * 6, 0);
const obj = (s: number[]): number => -pen() + s[0]! * 0.7 + s[1]! * 1.2 + s[2]! * 1.2 + 1.5 * (Math.min(s[0]!, 0.9) + Math.min(s[1]!, 0.88) + Math.min(s[2]!, 0.88));

let blades: Blade[] = existsSync(PARAMS) && !process.env.RESET ? JSON.parse(readFileSync(PARAMS, "utf8")) : INIT;
const rounds = Number(process.env.ROUNDS ?? 0);
const RW = Number(process.env.RW ?? 0.2), RT = Number(process.env.RT ?? 0.1);
let best = score(blades);
let bo = obj(best);
console.log("start", best.map((x) => x.toFixed(3)).join(" "));
let seed = Number(process.env.SEED ?? 12345);
let top = { blades, s: best };
let topO = bo;
const rnd = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
for (let r = 0; r < rounds; r++) {
  const step = 0.12 * (1 - r / rounds) + 0.02;
  const i = Math.floor(rnd() * blades.length);
  const cand: Blade[] = blades.map((b) => ({ ...b, T: [...b.T] as [number, number, number] }));
  const b = cand[i]!;
  const k = Math.floor(rnd() * 6);
  const d = (rnd() - 0.5) * 2 * step;
  if (k < 3) b.T[k] = k === 1 ? Math.min(1.43, Math.max(0.15, b.T[1] + d)) : Math.min(k === 0 ? 1.22 : 1.15, Math.max(k === 0 ? -1.22 : -1.15, b.T[k]! + d * 2));
  else if (k === 3) b.arch = Math.max(0, b.arch + d);
  else if (k === 4) b.rw = Math.min(RW, Math.max(F * 0.55, b.rw + d * 0.5));
  else b.rt = Math.min(RT, Math.max(F * 0.55, b.rt + d * 0.5));
  const s = score(cand);
  const o = obj(s);
  const temp = 0.08 * (1 - r / rounds) ** 2;
  if (o > bo || rnd() < Math.exp((o - bo) / Math.max(temp, 1e-9))) { blades = cand; best = s; bo = o; if (o > topO) { topO = o; top = { blades: cand, s }; } }
}
if (rounds > 0) { blades = top.blades; best = top.s; writeFileSync(PARAMS, JSON.stringify(blades)); }
console.log("end", best.map((x) => x.toFixed(3)).join(" "));
const body = [...fixed, ...blades.map(bladeSolid)];
saveLists(c, { body });
console.log(body.map((s) => s.id).join(" "));
