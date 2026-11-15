/** 🌳️ Author 02 kit: hand-parameterised per-scale models of trees and grasses from the scale's own 2D shapes (hull lofts, lobes, limbs), thermal sheets and structure members.
 * @see ./author-brief.md */
import { writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

import { Mask, TICKET, simplifyRing, entryOf, featureM, memberBones, memberShapes, meshArea, meshesAt, poseState, projectWorld, toLocal, dirToLocal, type Json, type PoseState, type Ring, type V3 } from "./volumes-lib.ts";

export type Iv = [number, number];
export type Shapes = Ring[][];
export type Meta = { material: string; role: string; tone: number };
export type Sec = { c: V3; ax: V3; ay: V3; rx: number; ry: number; exp: number };

const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
const rnd = (v: any): any => (typeof v === "number" ? r5(v) : Array.isArray(v) ? v.map(rnd) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, rnd(x)])) : v);
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const quatOf = (x: V3, y: V3, z: V3): [number, number, number, number] => {
  const m00 = x[0], m10 = x[1], m20 = x[2], m01 = y[0], m11 = y[1], m21 = y[2], m02 = z[0], m12 = z[1], m22 = z[2];
  const tr = m00 + m11 + m22;
  let q: [number, number, number, number];
  if (tr > 0) { const s = Math.sqrt(tr + 1) * 2; q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]; }
  else if (m00 > m11 && m00 > m22) { const s = Math.sqrt(1 + m00 - m11 - m22) * 2; q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]; }
  else if (m11 > m22) { const s = Math.sqrt(1 + m11 - m00 - m22) * 2; q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]; }
  else { const s = Math.sqrt(1 + m22 - m00 - m11) * 2; q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]; }
  const n = Math.hypot(...q);
  return q.map((c) => c / n) as [number, number, number, number];
};

/** 🧮️ Even-odd intervals of the rings along one axis: `axis` "y" slices at y = at and returns x intervals, "x" slices at x = at and returns y intervals. */
export function ivals(rings: Ring[], axis: "x" | "y", at: number): Iv[] {
  const vals: number[] = [];
  for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!, b = ring[i]!;
    const [pa, pb, qa, qb] = axis === "y" ? [a.y, b.y, a.x, b.x] : [a.x, b.x, a.y, b.y];
    if (pa <= at !== pb <= at) vals.push(qa + ((at - pa) / (pb - pa)) * (qb - qa));
  }
  vals.sort((p, q) => p - q);
  const out: Iv[] = [];
  for (let n = 0; n + 1 < vals.length; n += 2) out.push([vals[n]!, vals[n + 1]!]);
  return out;
}
export function mergeIv(list: Iv[]): Iv[] {
  const s = [...list].sort((a, b) => a[0] - b[0]);
  const out: Iv[] = [];
  for (const iv of s) { const last = out[out.length - 1]; if (last && iv[0] <= last[1]) last[1] = Math.max(last[1], iv[1]); else out.push([iv[0], iv[1]]); }
  return out;
}
export const ivalsAll = (shapes: Shapes, axis: "x" | "y", at: number): Iv[] => mergeIv(shapes.flatMap((r) => ivals(r, axis, at)));
export function bboxOfShapes(shapes: Shapes) {
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const s of shapes) for (const r of s) for (const p of r) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  return b;
}
const ringArea = (r: Ring) => { let a = 0; for (let i = 0; i < r.length; i++) { const p = r[i]!, n = r[(i + 1) % r.length]!; a += p.x * n.y - n.x * p.y; } return Math.abs(a) / 2; };
export const areaOfShapes = (shapes: Shapes) => shapes.reduce((t, s) => t + s.reduce((u, r) => u + ringArea(r), 0), 0);

const EXP_TABLE: [number, number][] = [];
for (let n = 2; n <= 8.01; n += 0.05) {
  const gamma = (z: number): number => { if (z < 0.5) return Math.PI / (Math.sin(Math.PI * z) * gamma(1 - z)); z -= 1; const c = [0.99999999999980993, 676.5203681218851, -1259.1392167224028, 771.32342877765313, -176.61502916214059, 12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7]; let x = c[0]!; for (let i = 1; i < 9; i++) x += c[i]! / (z + i); const t = z + 7.5; return Math.sqrt(2 * Math.PI) * Math.pow(t, z + 0.5) * Math.exp(-t) * x; };
  EXP_TABLE.push([n, (gamma(1 + 1 / n) ** 2) / gamma(1 + 2 / n)]);
}
/** 📐️ Superellipse exponent whose area is `ratio` of its bounding rectangle (pi/4 for an ellipse, 1 for a rectangle). */
export function expFor(ratio: number, lo = 2, hi = 5): number {
  let best = EXP_TABLE[0]!;
  for (const e of EXP_TABLE) if (Math.abs(e[1] - ratio) < Math.abs(best[1] - ratio)) best = e;
  return Math.min(hi, Math.max(lo, best[0]));
}

function dp(xs: number[], ys: number[], eps: number): number[] {
  const keep = new Set<number>([0, xs.length - 1]);
  const rec = (a: number, b: number) => {
    let worst = -1, wi = -1;
    for (let i = a + 1; i < b; i++) {
      const t = (xs[i]! - xs[a]!) / (xs[b]! - xs[a]!);
      const d = Math.abs(ys[a]! + t * (ys[b]! - ys[a]!) - ys[i]!);
      if (d > worst) { worst = d; wi = i; }
    }
    if (worst > eps) { keep.add(wi); rec(a, wi); rec(wi, b); }
  };
  rec(0, xs.length - 1);
  return [...keep].sort((p, q) => p - q);
}


const clipX = (ring: { x: number; y: number }[], x0: number, x1: number): { x: number; y: number }[] => {
  const cut = (pts: { x: number; y: number }[], keep: (p: { x: number; y: number }) => boolean, at: (a: { x: number; y: number }, b: { x: number; y: number }) => { x: number; y: number }) => {
    const out: { x: number; y: number }[] = [];
    for (let i = 0; i < pts.length; i++) {
      const a = pts[i]!, b = pts[(i + 1) % pts.length]!;
      const ka = keep(a), kb = keep(b);
      if (ka) out.push(a);
      if (ka !== kb) out.push(at(a, b));
    }
    return out;
  };
  const lo = cut(ring, (p) => p.x >= x0, (a, b) => ({ x: x0, y: a.y + ((x0 - a.x) / (b.x - a.x)) * (b.y - a.y) }));
  if (lo.length < 3) return [];
  return cut(lo, (p) => p.x <= x1, (a, b) => ({ x: x1, y: a.y + ((x1 - a.x) / (b.x - a.x)) * (b.y - a.y) }));
};
const centroidOf = (r: { x: number; y: number }[]) => {
  let a = 0, cx = 0, cy = 0;
  for (let i = 0; i < r.length; i++) { const p = r[i]!, q = r[(i + 1) % r.length]!; const w = p.x * q.y - q.x * p.y; a += w; cx += (p.x + q.x) * w; cy += (p.y + q.y) * w; }
  return a === 0 ? { x: r[0]!.x, y: r[0]!.y } : { x: cx / (3 * a), y: cy / (3 * a) };
};

export class Kit {
  readonly asset: Json;
  readonly st: PoseState;
  readonly feat: number;
  readonly sF: number;
  readonly sPx: number;
  readonly sPz: number;
  body: Json[] = [];
  thermal: Json[] = [];
  structure: Json[] = [];
  decisions: { en: string; de: string }[] = [];
  lastPath: Sec[] = [];
  constructor(readonly spec: string, readonly scale: number, asset?: Json) {
    this.asset = asset ?? entryOf(spec).asset;
    this.st = poseState(this.asset, scale);
    this.feat = featureM(scale, this.asset);
    this.sF = projectWorld("front", [0, 0, 1]).x;
    this.sPx = projectWorld("plan", [1, 0, 0]).x;
    this.sPz = projectWorld("plan", [0, 0, 1]).y;
    if (projectWorld("front", [0, 1, 0]).y !== 1 || projectWorld("elevation", [1, 0, 0]).x !== 1) throw new Error("unexpected view axes");
  }
  get fr() { return this.st.frames; }
  L(bone: string, p: V3): V3 { return toLocal(this.fr[bone]!, p); }
  D(bone: string, d: V3): V3 { return dirToLocal(this.fr[bone]!, d); }
  boneDef(id: string): Json { return this.asset.rig.bones.find((b: Json) => b.id === id); }
  tip(bone: string): V3 { const f = this.fr[bone]!; const L = this.boneDef(bone).lengthM; return [f.o[0] + f.u[0] * L, f.o[1] + f.u[1] * L, f.o[2] + f.u[2] * L]; }
  origin(bone: string): V3 { return [...this.fr[bone]!.o] as V3; }

  shapes(view: "elevation" | "front" | "plan", pred: (s: Json) => boolean = () => true, roles = ["body", "detail", "glazing"]) {
    return this.st[view].filter((e) => e.closed && e.rings.length > 0 && roles.includes(e.shape.role) && pred(e.shape));
  }
  /** 🖼️ Elevation shapes in (x, y). */
  E(pred: (s: Json) => boolean, roles?: string[]): Shapes { return this.shapes("elevation", pred, roles).map((e) => e.rings); }
  /** 🖼️ Front shapes in (z, y). */
  F(pred: (s: Json) => boolean, roles?: string[]): Shapes { return this.shapes("front", pred, roles).map((e) => e.rings.map((r) => r.map((p) => ({ x: this.sF * p.x, y: p.y })))); }
  /** 🖼️ Plan shapes in (x, z). */
  P(pred: (s: Json) => boolean, roles?: string[]): Shapes { return this.shapes("plan", pred, roles).map((e) => e.rings.map((r) => r.map((p) => ({ x: this.sPx * p.x, y: this.sPz * p.y })))); }
  byId = (...ids: string[]) => (s: Json) => ids.includes(s.id);
  metaOf(view: "elevation" | "front" | "plan", pred: (s: Json) => boolean, over: Partial<Meta> = {}): Meta {
    const e = this.st[view].find((x) => x.closed && pred(x.shape));
    const s = e?.shape ?? {};
    const material = over.material ?? s.material ?? this.asset.thermal.regions.find((r: Json) => r.id === s.region)?.material;
    if (!material) throw new Error(`no material for ${JSON.stringify(s.id)}`);
    return { material, role: over.role ?? s.role ?? "body", tone: over.tone ?? s.tone ?? 0.5 };
  }

  private push(list: Json[], id: string, bone: string, m: Meta, geometry: Json, extra: Json = {}) {
    if (list.some((s) => s.id === id)) throw new Error(`duplicate id ${id}`);
    list.push(rnd({ id, kind: geometry.kind, bone, material: m.material, role: m.role, tone: m.tone, ...geometry, ...extra }));
  }
  add(list: "body" | "thermal" | "structure", id: string, bone: string, m: Meta, geometry: Json, extra: Json = {}) { this.push(this[list], id, bone, m, geometry, extra); }

  /** 🧵️ Loft through world sections; each section carries its own axes (path tangent = ax x ay). */
  loft(list: "body" | "thermal" | "structure", id: string, bone: string, secs: Sec[], m: Meta, extra: Json = {}, caps = true) {
    if (secs.length < 2) return false;
    const sections = secs.map((s) => ({ at: this.L(bone, s.c), rx: Math.max(s.rx, 1e-3), ry: Math.max(s.ry, 1e-3), exponent: s.exp, rotation: quatOf(this.D(bone, s.ax), this.D(bone, s.ay), this.D(bone, cross(s.ax, s.ay))) }));
    this.push(this[list], id, bone, m, { kind: "loft", sections, capStart: caps, capEnd: caps }, extra);
    return true;
  }
  cap(list: "body" | "thermal" | "structure", id: string, bone: string, a: V3, b: V3, r0: number, r1: number, m: Meta, extra: Json = {}) {
    this.push(this[list], id, bone, m, { kind: "capsule", p0: this.L(bone, a), p1: this.L(bone, b), r0, r1 }, extra);
  }
  ell(list: "body" | "thermal" | "structure", id: string, bone: string, c: V3, radii: V3, m: Meta, extra: Json = {}) {
    this.push(this[list], id, bone, m, { kind: "ellipsoid", radii }, { transform: { translation: this.L(bone, c) }, ...extra });
  }
  /** 🥚️ World-aligned ellipsoid: radii along the world axes, whatever the frame of the bone. */
  ellW(list: "body" | "thermal" | "structure", id: string, bone: string, c: V3, radii: V3, m: Meta, extra: Json = {}) {
    const q = quatOf(this.D(bone, [1, 0, 0]), this.D(bone, [0, 1, 0]), this.D(bone, [0, 0, 1]));
    this.push(this[list], id, bone, m, { kind: "ellipsoid", radii }, { transform: { rotation: q, translation: this.L(bone, c) }, ...extra });
  }
  /** 📦️ World-aligned box: size along the world axes, whatever the frame of the bone. */
  boxW(list: "body" | "thermal" | "structure", id: string, bone: string, c: V3, size: V3, m: Meta, extra: Json = {}) {
    const q = quatOf(this.D(bone, [1, 0, 0]), this.D(bone, [0, 1, 0]), this.D(bone, [0, 0, 1]));
    this.push(this[list], id, bone, m, { kind: "box", size }, { transform: { rotation: q, translation: this.L(bone, c) }, ...extra });
  }
  box(list: "body" | "thermal" | "structure", id: string, bone: string, c: V3, size: V3, m: Meta, extra: Json = {}) {
    this.push(this[list], id, bone, m, { kind: "box", size }, { transform: { translation: this.L(bone, c) }, ...extra });
  }

  secY(c: V3, hx: number, hz: number, exp: number, phi = 0): Sec {
    const co = Math.cos(phi), si = Math.sin(phi);
    return { c, ax: [co, 0, si], ay: [si, 0, -co], rx: hx, ry: hz, exp };
  }
  secX(c: V3, hy: number, hz: number, exp: number): Sec { return { c, ax: [0, 1, 0], ay: [0, 0, 1], rx: hy, ry: hz, exp }; }

  /** 🫧️ Plan outline as oriented superellipse: angle, centre, half sizes along the axes and exponent from the area ratio. */
  planFit(P: Shapes, snapDeg = 14) {
    const pts = P.flatMap((s) => s.flatMap((r) => r));
    const n = pts.length;
    const cx = pts.reduce((t, p) => t + p.x, 0) / n, cz = pts.reduce((t, p) => t + p.y, 0) / n;
    let sxx = 0, szz = 0, sxz = 0;
    for (const p of pts) { sxx += (p.x - cx) ** 2; szz += (p.y - cz) ** 2; sxz += (p.x - cx) * (p.y - cz); }
    let phi = 0.5 * Math.atan2(2 * sxz, sxx - szz);
    if (Math.abs(phi) < (snapDeg * Math.PI) / 180) phi = 0;
    const co = Math.cos(phi), si = Math.sin(phi);
    let a0 = Infinity, a1 = -Infinity, b0 = Infinity, b1 = -Infinity;
    for (const p of pts) { const a = (p.x - cx) * co + (p.y - cz) * si, b = (p.x - cx) * si - (p.y - cz) * co; a0 = Math.min(a0, a); a1 = Math.max(a1, a); b0 = Math.min(b0, b); b1 = Math.max(b1, b); }
    const ha = (a1 - a0) / 2, hb = (b1 - b0) / 2;
    const ca = (a0 + a1) / 2, cb = (b0 + b1) / 2;
    const centre: [number, number] = [cx + ca * co + cb * si, cz + ca * si - cb * co];
    const ratio = areaOfShapes(P) / (4 * ha * hb || 1);
    return { phi, centre, ha, hb, exp: expFor(ratio, 2, 4.2), ratio };
  }

  /** 🥚️ A lobe: plan outline as oriented superellipse swept over the height with the front outline's width profile. */
  lobe(id: string, bone: string, P: Shapes, F: Shapes | null, m: Meta, o: { n?: number; yRange?: Iv; list?: "body" | "thermal" | "structure"; extra?: Json; k?: number } = {}) {
    const fit = this.planFit(P);
    let y0: number, y1: number;
    const prof: (y: number) => Iv = (() => {
      if (F) {
        const b = bboxOfShapes(F);
        y0 = b.minY; y1 = b.maxY;
        return (y: number) => { const iv = ivalsAll(F, "y", y); return iv.length ? [iv[0]![0], iv[iv.length - 1]![1]] : [0, 0]; };
      }
      [y0, y1] = o.yRange!;
      const zc = fit.centre[1], hz = fit.hb;
      return (y: number) => { const u = (2 * (y - y0)) / (y1 - y0) - 1; const f = Math.sqrt(Math.max(0, 1 - u * u)); return [zc - hz * f, zc + hz * f]; };
    })();
    if (o.yRange) [y0!, y1!] = o.yRange;
    const n = o.n ?? 9;
    let zmax = 0, zmid = 0;
    {
      const pr: Iv[] = []; for (let k = 0; k <= 24; k++) pr.push(prof(y0! + ((y1! - y0!) * (k + 0.5)) / 25));
      zmax = Math.max(...pr.map((p) => (p[1] - p[0]) / 2)); zmid = (Math.min(...pr.map((p) => p[0])) + Math.max(...pr.map((p) => p[1]))) / 2;
    }
    const zext = (() => { const co = Math.cos(fit.phi), si = Math.sin(fit.phi); let m2 = 0; for (let t = 0; t < 64; t++) { const a = (2 * Math.PI * t) / 64; const c = Math.cos(a), s = Math.sin(a); const sg = (v: number) => Math.sign(v) * Math.abs(v) ** (2 / fit.exp); const u = sg(c) * fit.ha, v = sg(s) * fit.hb; m2 = Math.max(m2, Math.abs(u * si - v * co)); } return m2; })();
    const kz = (fit.phi === 0 ? 1 : zmax / (zext || 1)) * (o.k ?? 1);
    const secs: Sec[] = [];
    for (let k = 0; k < n; k++) {
      const t = (1 - Math.cos((Math.PI * k) / (n - 1))) / 2;
      const y = y0! + (y1! - y0!) * t;
      const yy = Math.min(y1! - 1e-4, Math.max(y0! + 1e-4, y));
      const [za, zb] = prof(yy);
      const s = Math.max(0.02, (zb - za) / 2 / (zmax || 1));
      const dz = (za + zb) / 2 - zmid;
      secs.push(this.secY([fit.centre[0], y, fit.centre[1] + dz], fit.ha * s * kz, fit.hb * s * kz, fit.exp, fit.phi));
    }
    return this.loft(o.list ?? "body", id, bone, secs, m, o.extra ?? {});
  }

  /** 🔬️ Visual-hull chains of up to three silhouettes (x, y), (z, y) and (x, z): sections along y, connected components per level linked into chains. */
  hullChains(o: { E?: Shapes; F?: Shapes; P?: Shapes; step: number; cell?: number; minArea?: number; yRange?: Iv; clampP?: boolean; grow?: number; growE?: number; growF?: number }) {
    const { E, F, P } = o;
    const grow = o.grow ?? 0;
    const growE = o.growE ?? grow, growF = o.growF ?? grow;
    const ivG = (S: Shapes, y: number, g: number): Iv[] => {
      if (!g) return ivalsAll(S, "y", y);
      const all: Iv[] = [];
      for (const dy of [-g, -g / 2, 0, g / 2, g]) for (const v of ivalsAll(S, "y", y + dy)) all.push([v[0] - g * Math.sqrt(Math.max(0, 1 - (dy / g) ** 2)), v[1] + g * Math.sqrt(Math.max(0, 1 - (dy / g) ** 2))]);
      return mergeIv(all);
    };
    const eb = E ? bboxOfShapes(E) : null, fb = F ? bboxOfShapes(F) : null, pb = P ? bboxOfShapes(P) : null;
    let y0 = Math.max(eb?.minY ?? -Infinity, fb?.minY ?? -Infinity), y1 = Math.min(eb?.maxY ?? Infinity, fb?.maxY ?? Infinity);
    if (o.yRange) [y0, y1] = o.yRange; else { y0 -= growE; y1 += growE; }
    const x0 = (eb?.minX ?? pb!.minX) - Math.max(growE, grow), x1 = (eb?.maxX ?? pb!.maxX) + Math.max(growE, grow), z0 = (fb?.minX ?? pb!.minY) - Math.max(growF, grow), z1 = (fb?.maxX ?? pb!.maxY) + Math.max(growF, grow);
    const cell = o.cell ?? Math.max(this.feat * 0.5, Math.max(x1 - x0, z1 - z0) / 280);
    const nx = Math.ceil((x1 - x0) / cell) + 2, nz = Math.ceil((z1 - z0) / cell) + 2;
    const gx = x0 - cell, gz = z0 - cell;
    let pm: Uint8Array | null = null;
    if (P) {
      const mk = new Mask({ minX: gx, minY: gz, maxX: gx + nx * cell, maxY: gz + nz * cell }, cell);
      for (const s of P) mk.fill(s);
      pm = mk.cells;
      const r = Math.round(grow / cell);
      if (r > 0) {
        const dil = new Uint8Array(pm.length);
        for (let j = 0; j < nz; j++) for (let i = 0; i < nx; i++) if (pm[j * nx + i]) for (let dj = -r; dj <= r; dj++) for (let di = -r; di <= r; di++) { if (di * di + dj * dj > r * r) continue; const ii = i + di, jj = j + dj; if (ii >= 0 && jj >= 0 && ii < nx && jj < nz) dil[jj * nx + ii] = 1; }
        pm = dil;
      }
    }
    const n = Math.max(2, Math.round((y1 - y0) / o.step));
    type Comp = { x0: number; x1: number; z0: number; z1: number; area: number; cells: number[] };
    type Chain = { secs: { y: number; c: Comp }[] };
    const chains: Chain[] = [];
    let prevLabel: Int32Array | null = null, prevComps: Comp[] = [], prevChain: (Chain | null)[] = [];
    const minArea = o.minArea ?? (2 * this.feat) ** 2;
    for (let k = 0; k <= n; k++) {
      const y = y0 + ((y1 - y0) * Math.min(Math.max(k, 0.02), n - 0.02)) / n;
      const ie = E ? ivG(E, y, growE) : null, iff = F ? ivG(F, y, growF) : null;
      const okx = new Uint8Array(nx), okz = new Uint8Array(nz);
      for (let i = 0; i < nx; i++) { const x = gx + (i + 0.5) * cell; okx[i] = !ie || ie.some((v) => x >= v[0] && x <= v[1]) ? 1 : 0; }
      for (let j = 0; j < nz; j++) { const z = gz + (j + 0.5) * cell; okz[j] = !iff || iff.some((v) => z >= v[0] && z <= v[1]) ? 1 : 0; }
      const label = new Int32Array(nx * nz).fill(-1);
      const comps: Comp[] = [];
      const stack: number[] = [];
      for (let j = 0; j < nz; j++) for (let i = 0; i < nx; i++) {
        const idx = j * nx + i;
        if (label[idx] !== -1 || !okx[i] || !okz[j] || (pm && !pm[idx])) continue;
        const comp: Comp = { x0: Infinity, x1: -Infinity, z0: Infinity, z1: -Infinity, area: 0, cells: [] };
        const id = comps.length;
        label[idx] = id; stack.push(idx);
        while (stack.length) {
          const c = stack.pop()!; const ci = c % nx, cj = (c - ci) / nx;
          comp.cells.push(c);
          comp.x0 = Math.min(comp.x0, gx + ci * cell); comp.x1 = Math.max(comp.x1, gx + (ci + 1) * cell);
          comp.z0 = Math.min(comp.z0, gz + cj * cell); comp.z1 = Math.max(comp.z1, gz + (cj + 1) * cell);
          for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) {
            const ni = ci + di, nj = cj + dj;
            if (ni < 0 || nj < 0 || ni >= nx || nj >= nz) continue;
            const ii = nj * nx + ni;
            if (label[ii] !== -1 || !okx[ni] || !okz[nj] || (pm && !pm[ii])) continue;
            label[ii] = id; stack.push(ii);
          }
        }
        comp.area = comp.cells.length * cell * cell;
        comps.push(comp);
      }
      const keep = comps.map((c) => c.area >= minArea);
      const curChain: (Chain | null)[] = comps.map(() => null);
      if (prevLabel) {
        const ov = new Map<number, number>();
        for (let i = 0; i < label.length; i++) if (label[i]! >= 0 && prevLabel[i]! >= 0) { const key = label[i]! * 4096 + prevLabel[i]!; ov.set(key, (ov.get(key) ?? 0) + 1); }
        const pairs = [...ov.entries()].map(([key, v]) => ({ cur: Math.floor(key / 4096), prev: key % 4096, v })).sort((a, b) => b.v - a.v);
        const usedPrev = new Set<number>();
        for (const p of pairs) {
          if (curChain[p.cur] || usedPrev.has(p.prev) || !prevChain[p.prev] || !keep[p.cur]) continue;
          const small = Math.min(comps[p.cur]!.cells.length, prevComps[p.prev]!.cells.length);
          if (p.v < 0.2 * small) continue;
          curChain[p.cur] = prevChain[p.prev]!; usedPrev.add(p.prev);
        }
      }
      comps.forEach((c, i) => {
        if (!keep[i]) return;
        if (!curChain[i]) { const ch: Chain = { secs: [] }; chains.push(ch); curChain[i] = ch; }
        curChain[i]!.secs.push({ y, c });
      });
      prevLabel = label; prevComps = comps; prevChain = curChain;
    }
    return { chains, y0, y1, step: (y1 - y0) / n, gx, gz, nx, cell };
  }

  /** 🧱️ Loft solids of the hull chains of three (or two) silhouettes: sections are oriented superellipses (principal axis of the chain, bounding half sizes along it, exponent from the area ratio). */
  hull(id: string, bone: string, m: Meta, o: { E?: Shapes; F?: Shapes; P?: Shapes; step: number; cell?: number; pinch?: number; extra?: Json; list?: "body" | "thermal" | "structure"; expLo?: number; expHi?: number; minArea?: number; yRange?: Iv; grow?: number; growE?: number; growF?: number; shrink?: number; snapDeg?: number; orient?: boolean; pinchLo?: number; pinchHi?: number; areaK?: number }) {
    const { chains, y0, y1, step, gx, gz, nx, cell } = this.hullChains(o);
    let made = 0;
    const pinch = o.pinch ?? 0.5;
    chains.forEach((ch, ci) => {
      if (ch.secs.length < 1) return;
      let phi = 0;
      if (o.orient !== false && o.P) {
        let sxx = 0, szz = 0, sxz = 0;
        for (const s of ch.secs) {
          const cs = s.c.cells; let mx = 0, mz = 0;
          for (const c of cs) { mx += gx + ((c % nx) + 0.5) * cell; mz += gz + (Math.floor(c / nx) + 0.5) * cell; }
          mx /= cs.length; mz /= cs.length;
          for (const c of cs) { const dx = gx + ((c % nx) + 0.5) * cell - mx, dz = gz + (Math.floor(c / nx) + 0.5) * cell - mz; sxx += dx * dx; szz += dz * dz; sxz += dx * dz; }
        }
        phi = 0.5 * Math.atan2(2 * sxz, sxx - szz);
        if (Math.abs(phi) < ((o.snapDeg ?? 12) * Math.PI) / 180) phi = 0;
      }
      const co = Math.cos(phi), si = Math.sin(phi);
      const mk = (y: number, c: any, f: number) => {
        let a0 = Infinity, a1 = -Infinity, b0 = Infinity, b1 = -Infinity;
        if (phi === 0) { a0 = c.x0; a1 = c.x1; b0 = c.z0; b1 = c.z1; }
        else for (const k of c.cells as number[]) {
          const px = gx + ((k % nx) + 0.5) * cell, pz = gz + (Math.floor(k / nx) + 0.5) * cell;
          const a = px * co + pz * si, b = px * si - pz * co;
          a0 = Math.min(a0, a - cell / 2); a1 = Math.max(a1, a + cell / 2); b0 = Math.min(b0, b - cell / 2); b1 = Math.max(b1, b + cell / 2);
        }
        const ratio = c.area / (4 * ((a1 - a0) / 2) * ((b1 - b0) / 2) || 1);
        const eq = o.P && ratio < Math.PI / 4 ? Math.sqrt(ratio / (Math.PI / 4)) ** (o.areaK ?? 1) : 1;
        const ha = ((a1 - a0) / 2) * f * eq, hb = ((b1 - b0) / 2) * f * eq, ca = (a0 + a1) / 2, cb = (b0 + b1) / 2;
        const centre: V3 = phi === 0 ? [ca, y, cb] : [ca * co + cb * si, y, ca * si - cb * co];
        return this.secY(centre, ha * (o.shrink ?? 1), hb * (o.shrink ?? 1), o.P ? expFor(ratio, o.expLo ?? 2, o.expHi ?? 4) : 2.6, phi);
      };
      const first = ch.secs[0]!, last = ch.secs[ch.secs.length - 1]!;
      const yA = Math.max(y0, first.y - step * 0.5), yB = Math.min(y1, last.y + step * 0.5);
      const secs: Sec[] = [];
      secs.push(mk(yA, first.c, first.y - yA < 1e-6 ? 1 : (o.pinchLo ?? pinch)));
      for (const s of ch.secs) secs.push(mk(s.y, s.c, 1));
      secs.push(mk(yB, last.c, yB - last.y < 1e-6 ? 1 : (o.pinchHi ?? pinch)));
      const dedup = secs.filter((s, i) => i === 0 || Math.abs(s.c[1] - secs[i - 1]!.c[1]) > 1e-5);
      if (this.loft(o.list ?? "body", chains.length === 1 ? id : `${id}-${ci + 1}`, bone, dedup, m, o.extra ?? {})) made++;
    });
    return made;
  }

  /** 🌿️ Limb solids along the elevation shape: sections along the dominant axis; z centre and depth from the front (z, y) or plan (x, z) outline of the same shape else round. */
  limb(id: string, bone: string, E: Shapes, F: Shapes | null, P: Shapes | null, m: Meta, o: { n?: number; exp?: number; round?: number; list?: "body" | "thermal" | "structure"; extra?: Json; axis?: "x" | "y"; ends?: boolean } = {}) {
    if (!E.length && F) {
      const fb = bboxOfShapes(F);
      const L = this.boneDef(bone);
      const f = this.fr[bone]!;
      const n0 = o.n ?? 5;
      const secs0: Sec[] = [];
      for (let k = 0; k < n0; k++) {
        const y = fb.minY + ((fb.maxY - fb.minY) * k) / (n0 - 1);
        const iv = ivalsAll(F, "y", Math.min(fb.maxY - 1e-4, Math.max(fb.minY + 1e-4, y)));
        if (!iv.length) continue;
        const t = f.u[1] ? (y - f.o[1]) / f.u[1] : 0;
        const x = f.o[0] + f.u[0] * t;
        const hz = (iv[0]![1] - iv[0]![0]) / 2;
        secs0.push(this.secY([x, y, (iv[0]![0] + iv[0]![1]) / 2], hz, hz, 2));
      }
      void L;
      this.lastPath = secs0;
      return this.loft(o.list ?? "body", id, bone, secs0, m, o.extra ?? {});
    }
    const b = bboxOfShapes(E);
    const axis = o.axis ?? (b.maxY - b.minY >= b.maxX - b.minX ? "y" : "x");
    const lo = axis === "y" ? b.minY : b.minX, hi = axis === "y" ? b.maxY : b.maxX;
    const n = o.n ?? Math.max(3, Math.min(14, Math.round((hi - lo) / Math.max(this.feat * 5, 0.25)) + 1));
    const secs: Sec[] = [];
    let prev: Iv | null = null;
    const ts = Array.from({ length: n }, (_, k) => k / (n - 1));
    if (o.ends) for (const e of [0.015, 0.04, 0.08, 0.14]) { ts.push(e, 1 - e); }
    ts.sort((a, b) => a - b);
    for (const t of ts) {
      const s = lo + (hi - lo) * t;
      const sc = Math.min(hi - 1e-4, Math.max(lo + 1e-4, s));
      let iv = ivalsAll(E, axis === "y" ? "y" : "x", sc);
      if (!iv.length) iv = prev ? [prev] : [];
      if (!iv.length) continue;
      const pick = prev ? iv.reduce((a, c) => (Math.abs((c[0] + c[1]) / 2 - (prev![0] + prev![1]) / 2) < Math.abs((a[0] + a[1]) / 2 - (prev![0] + prev![1]) / 2) ? c : a)) : iv.reduce((a, c) => (c[1] - c[0] > a[1] - a[0] ? c : a));
      prev = pick;
      const half = (pick[1] - pick[0]) / 2, mid = (pick[0] + pick[1]) / 2;
      let cz = 0, hz = half * (o.round ?? 1);
      let zi: Iv[] = [];
      if (axis === "y" && F) zi = ivalsAll(F, "y", sc);
      if (axis === "x" && P) zi = ivalsAll(P, "x", sc);
      if (zi.length) { const w = zi.reduce((a, c) => (c[1] - c[0] > a[1] - a[0] ? c : a)); cz = (w[0] + w[1]) / 2; hz = (w[1] - w[0]) / 2; }
      secs.push(axis === "y" ? this.secY([mid, sc, cz], half, hz, o.exp ?? 2) : this.secX([sc, mid, cz], half, hz, o.exp ?? 2));
    }
    this.lastPath = secs;
    return this.loft(o.list ?? "body", id, bone, secs, m, o.extra ?? {});
  }


  /** 🧱️ Extrusion along world y of a plan polygon (x, z) between two heights with end scales about its centroid; bone-local transform keeps the solid world-aligned. */
  extrudeY(list: "body" | "thermal" | "structure", id: string, bone: string, ring: { x: number; y: number }[], y0: number, y1: number, s0: number, s1: number, m: Meta, o: { dz?: number; dx?: number; extra?: Json } = {}) {
    const q = quatOf(this.D(bone, [1, 0, 0]), this.D(bone, [0, 1, 0]), this.D(bone, [0, 0, 1]));
    const origin = this.L(bone, [o.dx ?? 0, 0, o.dz ?? 0]);
    this.push(this[list], id, bone, m, { kind: "extrude", plane: "zx", profile: ring.map((p) => [p.y, p.x]), from: y0, to: y1, taperTo: [s0, s1] }, { transform: { rotation: q, translation: origin }, ...(o.extra ?? {}) });
  }

  /** 🥞️ A lobe as a stack of frusta of its exact plan outline whose scale follows the front outline's width profile (breakpoints by Douglas-Peucker), centre drift from the front outline. */
  lobeSlabs(id: string, bone: string, P: Shapes, F: Shapes, m: Meta, o: { tol?: number; simplify?: number; list?: "body" | "thermal" | "structure"; yShift?: number; minScale?: number; yTrim?: Iv; yMap?: (y: number) => number; clipE?: Shapes; clipMargin?: number } = {}) {
    const ring0 = P.flatMap((s) => s).reduce((a, r) => (ringArea(r) > ringArea(a) ? r : a));
    const ring = simplifyRing(ring0, o.simplify ?? this.feat * 0.5);
    const fb = bboxOfShapes(F);
    const [y0, y1] = o.yTrim ?? [fb.minY, fb.maxY];
    const N = 48;
    const ys: number[] = [], hw: number[] = [], mid: number[] = [];
    for (let k = 0; k <= N; k++) {
      const y = y0 + ((y1 - y0) * k) / N;
      const yy = Math.min(fb.maxY - 1e-4, Math.max(fb.minY + 1e-4, y));
      const iv = ivalsAll(F, "y", yy);
      ys.push(y);
      if (iv.length) { hw.push((iv[iv.length - 1]![1] - iv[0]![0]) / 2); mid.push((iv[0]![0] + iv[iv.length - 1]![1]) / 2); } else { hw.push(0); mid.push(mid[mid.length - 1] ?? 0); }
    }
    const wmax = Math.max(...hw);
    const minS = o.minScale ?? 0.1;
    let keep = dp(ys, hw, o.tol ?? Math.max(this.feat * 2, 0.012) * (wmax > 0.6 ? 1 : 0.6));
    keep = keep.filter((v, i) => i === 0 || i === keep.length - 1 || ((ys[v]! - ys[keep[i - 1]!]!) >= this.feat * 1.6 && (ys[keep[keep.length - 1]!]! - ys[v]!) >= this.feat * 1.6));
    const zc = (Math.min(...mid) + Math.max(...mid)) / 2;
    const cz = (() => { const b = bboxOfShapes(P); return (b.minY + b.maxY) / 2; })();
    let n = 0;
    for (let i = 0; i + 1 < keep.length; i++) {
      const a = keep[i]!, b = keep[i + 1]!;
      const sa = Math.max(minS, hw[a]! / wmax), sb = Math.max(minS, hw[b]! / wmax);
      void zc; void cz;
      const ya = o.yMap ? o.yMap(ys[a]!) : ys[a]! + (o.yShift ?? 0), yb = o.yMap ? o.yMap(ys[b]!) : ys[b]! + (o.yShift ?? 0);
      let prof = ring;
      if (o.clipE && o.clipE.length) {
        const c0 = centroidOf(ring), sm = (sa + sb) / 2;
        const iv = ivalsAll(o.clipE, "y", (ya + yb) / 2);
        if (iv.length) {
          const lo = Math.min(...iv.map((v) => v[0])) + (o.clipMargin ?? 0), hi = Math.max(...iv.map((v) => v[1])) - (o.clipMargin ?? 0);
          const scaled = ring.map((p) => ({ x: c0.x + sm * (p.x - c0.x), y: p.y === undefined ? 0 : c0.y + sm * (p.y - c0.y) }));
          const cl = clipX(scaled, lo, hi);
          if (cl.length >= 3) { const c1 = centroidOf(cl); prof = cl.map((p) => ({ x: c1.x + (p.x - c1.x) / sm, y: c1.y + (p.y - c1.y) / sm })); }
        }
      }
      this.extrudeY(o.list ?? "body", `${id}-${i + 1}`, bone, prof, ya, yb, sa, sb, m);
      n++;
    }
    return n;
  }

  /** 📊️ Surface area of solids (resolved), summed over the tessellation. */
  area(solids: Json[]): number {
    if (!solids.length) return 0;
    return meshesAt(this.asset, this.scale, solids).reduce((t, m) => t + meshArea(m), 0);
  }
  /** 🎯️ Finds k in [lo, hi] such that `area(make(k))` meets the target (bisection on a monotone area). */
  calibrate(make: (k: number) => Json[], target: number, lo = 0.2, hi = 3, rounds = 14): { k: number; solids: Json[]; area: number } {
    let a = lo, b = hi, best = { k: 1, solids: make(1), area: 0 };
    best.area = this.area(best.solids);
    for (let i = 0; i < rounds; i++) {
      const k = (a + b) / 2;
      const solids = make(k);
      const ar = this.area(solids);
      if (Math.abs(ar / target - 1) < Math.abs(best.area / target - 1)) best = { k, solids, area: ar };
      if (ar < target) a = k; else b = k;
      if (Math.abs(ar / target - 1) < 0.01) break;
    }
    return best;
  }

  note(en: string, de: string) { this.decisions.push({ en, de }); }
  volume() {
    return { status: "authored", denominator: this.scale, body: this.body, thermal: this.thermal, structure: this.structure, decisions: this.decisions };
  }
  memberBones() { return memberBones(this.asset, this.scale); }
  memberShapes() { return memberShapes(this.asset, this.scale); }
}

export function writeSource(slug: string, volumes: Json) {
  const file = join(TICKET, "volumes", "vegetation", `${slug}.json`);
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify({ volumes }, null, 1)}\n`);
  console.log(`${file}: ${Object.entries(volumes).map(([k, v]: [string, Json]) => `${k}:${v.status === "authored" ? `${v.body.length}/${v.thermal.length}/${v.structure.length}` : "u"}`).join(" ")}`);
}
