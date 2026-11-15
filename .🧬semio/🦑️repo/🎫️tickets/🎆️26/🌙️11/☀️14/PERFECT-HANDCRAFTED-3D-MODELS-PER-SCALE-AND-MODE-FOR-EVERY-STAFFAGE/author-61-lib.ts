/** 🌿️ Author 61 helpers: 3D limb tubes from the principal axes of a limb's elevation and front outlines, and station selection by Douglas-Peucker.
 * @see ./author-61.md */
import type { Kit, Sec, Shapes } from "./author-02-lib.ts";
import type { V3 } from "./volumes-lib.ts";

type P = { x: number; y: number };
export type Centre = { c: P; w: number; t: number }[];

/** 📏️ Centre line and width of a polygon cut perpendicular to its principal axis at `n` stations, ordered from the end nearest to `root`. */
export function centreline(rings: Shapes, root: P, n: number): Centre {
  const pts = rings.flatMap((s) => s.flatMap((r) => r));
  const m = pts.length;
  const cx = pts.reduce((t, p) => t + p.x, 0) / m, cy = pts.reduce((t, p) => t + p.y, 0) / m;
  let sxx = 0, syy = 0, sxy = 0;
  for (const p of pts) { sxx += (p.x - cx) ** 2; syy += (p.y - cy) ** 2; sxy += (p.x - cx) * (p.y - cy); }
  const th = 0.5 * Math.atan2(2 * sxy, sxx - syy);
  let a: P = { x: Math.cos(th), y: Math.sin(th) };
  const proj = (p: P) => (p.x - cx) * a.x + (p.y - cy) * a.y;
  const lo0 = Math.min(...pts.map(proj)), hi0 = Math.max(...pts.map(proj));
  const near = (s: number) => Math.hypot(cx + a.x * s - root.x, cy + a.y * s - root.y);
  if (near(hi0) < near(lo0)) a = { x: -a.x, y: -a.y };
  const b: P = { x: -a.y, y: a.x };
  const lo = Math.min(...pts.map(proj)), hi = Math.max(...pts.map(proj));
  const out: Centre = [];
  for (let i = 0; i < n; i++) {
    const t = i / (n - 1);
    const s = lo + (hi - lo) * (0.004 + 0.992 * t);
    const cuts: number[] = [];
    for (const ring of rings.flatMap((r) => r)) for (let j = 0, k = ring.length - 1; j < ring.length; k = j++) {
      const p = ring[k]!, q = ring[j]!;
      const sp = proj(p), sq = proj(q);
      if (sp <= s !== sq <= s) { const f = (s - sp) / (sq - sp); cuts.push(((p.x + f * (q.x - p.x)) - cx) * b.x + ((p.y + f * (q.y - p.y)) - cy) * b.y); }
    }
    if (cuts.length < 2) continue;
    const c0 = Math.min(...cuts), c1 = Math.max(...cuts);
    const mid = (c0 + c1) / 2;
    out.push({ c: { x: cx + a.x * s + b.x * mid, y: cy + a.y * s + b.y * mid }, w: c1 - c0, t });
  }
  const edges = rings.flatMap((r) => r.map((p, i) => [r[(i + 1) % r.length]!, p] as const));
  const ray = (o: P, d: P): number => {
    let best = Infinity;
    for (const [p, q] of edges) {
      const ex = q.x - p.x, ey = q.y - p.y;
      const den = d.x * ey - d.y * ex;
      if (Math.abs(den) < 1e-12) continue;
      const u = ((p.x - o.x) * d.y - (p.y - o.y) * d.x) / den;
      const s = ((p.x - o.x) * ey - (p.y - o.y) * ex) / den;
      if (u >= 0 && u <= 1 && s > 1e-9 && s < best) best = s;
    }
    return best;
  };
  for (let iter = 0; iter < 2; iter++) {
    const next = out.map((st, i) => {
      const p0 = out[Math.max(0, i - 1)]!.c, p1 = out[Math.min(out.length - 1, i + 1)]!.c;
      const tl = Math.hypot(p1.x - p0.x, p1.y - p0.y) || 1;
      const nrm: P = { x: -(p1.y - p0.y) / tl, y: (p1.x - p0.x) / tl };
      const dp = ray(st.c, nrm), dm = ray(st.c, { x: -nrm.x, y: -nrm.y });
      if (!Number.isFinite(dp) || !Number.isFinite(dm)) return st;
      return { c: { x: st.c.x + (nrm.x * (dp - dm)) / 2, y: st.c.y + (nrm.y * (dp - dm)) / 2 }, w: dp + dm, t: st.t };
    });
    out.splice(0, out.length, ...next);
  }
  return out;
}

const norm = (v: V3): V3 => { const l = Math.hypot(...v) || 1; return [v[0] / l, v[1] / l, v[2] / l]; };
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** 🪵️ Sections of a tube along a 3D path (perpendicular to the tangent, circular unless `flat` < 1 squeezes the vertical-ish axis). */
export function tubeSections(path: V3[], radii: number[], exp = 2): Sec[] {
  return path.map((c, i) => {
    const p0 = path[Math.max(0, i - 1)]!, p1 = path[Math.min(path.length - 1, i + 1)]!;
    const T = norm([p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]]);
    const ref: V3 = Math.abs(T[1]) > 0.9 ? [0, 0, 1] : [0, 1, 0];
    const ax = norm(cross(ref, T));
    const ay = norm(cross(T, ax));
    return { c, ax: ay, ay: ax, rx: radii[i]!, ry: radii[i]!, exp };
  });
}

/** 🌳️ A limb as a tube: centre (x, y) from the elevation outline, z from the front outline's centre line at the same fraction of the length, radius from the elevation width. */
export function limbTube(k: Kit, id: string, bone: string, E: Shapes, F: Shapes, m: { material: string; role: string; tone: number }, o: { n?: number; rootBone?: P; extra?: Record<string, unknown>; rk?: number; fz?: (t: number) => number; keep?: (p: V3) => boolean; zFix?: (x: number, z: number) => number; zInside?: (x: number, z: number) => boolean; P?: Shapes }): Sec[] {
  const f = k.fr[bone]!;
  const root: P = o.rootBone ?? { x: f.o[0], y: f.o[1] };
  const n = o.n ?? 6;
  const ce = centreline(E, root, n);
  const rootF = ce.length ? { x: ce[0]!.c.x, y: ce[0]!.c.y } : root;
  const cf = F.length ? centreline(F, { x: 0, y: rootF.y }, n) : [];
  const cp = o.P !== undefined && o.P.length > 0 ? centreline(o.P, { x: rootF.x, y: 0 }, n) : [];
  const zAt = (t: number): number => {
    if (o.fz) return o.fz(t);
    if (cp.length) { const j = cp.reduce((bi, c, i) => (Math.abs(c.t - t) < Math.abs(cp[bi]!.t - t) ? i : bi), 0); return cp[j]!.c.y; }
    if (!cf.length) return 0;
    const j = cf.reduce((bi, c, i) => (Math.abs(c.t - t) < Math.abs(cf[bi]!.t - t) ? i : bi), 0);
    return cf[j]!.c.x;
  };
  let path = ce.map((s) => [s.c.x, s.c.y, o.zFix === undefined ? zAt(s.t) : o.zFix(s.c.x, zAt(s.t))] as V3);
  let radii = ce.map((s) => Math.max((s.w / 2) * (o.rk ?? 1), k.feat * 0.55));
  if (o.zInside !== undefined) {
    for (let kappa = 1; kappa >= 0; kappa -= 0.04) {
      if (path.every((q) => o.zInside!(q[0], q[2] * kappa))) { path = path.map((q) => [q[0], q[1], q[2] * kappa] as V3); break; }
      if (kappa < 0.05) break;
    }
  }
  if (o.keep !== undefined) {
    const cut = path.findIndex((q, i) => i > 0 && !o.keep!(q));
    if (cut > 0) {
      let lo = 0, hi = 1;
      const a = path[cut - 1]!, b = path[cut]!;
      const at = (f: number): V3 => [a[0] + f * (b[0] - a[0]), a[1] + f * (b[1] - a[1]), a[2] + f * (b[2] - a[2])];
      for (let i = 0; i < 24; i++) { const mid = (lo + hi) / 2; if (o.keep(at(mid))) lo = mid; else hi = mid; }
      const r = radii[cut - 1]! + lo * (radii[cut]! - radii[cut - 1]!);
      path = [...path.slice(0, cut), at(lo)];
      radii = [...radii.slice(0, cut), r];
    }
  }
  const secs = tubeSections(path, radii);
  k.loft("body", id, bone, secs, m, o.extra ?? {});
  return secs;
}

export function insideRings(rings: P[][], x: number, y: number): boolean { return rings.some((r) => { let w = false; for (let i = 0, j = r.length - 1; i < r.length; j = i++) if (r[i]!.y > y !== r[j]!.y > y && x < ((r[j]!.x - r[i]!.x) * (y - r[i]!.y)) / (r[j]!.y - r[i]!.y) + r[i]!.x) w = !w; return w; }); }

export type PlanFit = { phi: number; centre: [number, number]; ha: number; hb: number; exp: number; iou: number };
const inRing = (r: P[], x: number, y: number): boolean => { let w = false; for (let i = 0, j = r.length - 1; i < r.length; j = i++) if (r[i]!.y > y !== r[j]!.y > y && x < ((r[j]!.x - r[i]!.x) * (y - r[i]!.y)) / (r[j]!.y - r[i]!.y) + r[i]!.x) w = !w; return w; };

/** 🫧️ Oriented superellipse fitted to a plan footprint by coordinate descent on the intersection over union (centre, angle, both half sizes, exponent). */
export function fitPlan(P: Shapes, init?: { phi: number }): PlanFit {
  const rings = P.flatMap((s) => s);
  const pts = rings.flatMap((r) => r);
  const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
  const x0 = Math.min(...xs), x1 = Math.max(...xs), y0 = Math.min(...ys), y1 = Math.max(...ys);
  const cell = Math.max(x1 - x0, y1 - y0) / 56;
  const samples: { x: number; y: number; d: boolean }[] = [];
  const pad = Math.max(x1 - x0, y1 - y0) * 0.25;
  for (let x = x0 - pad; x <= x1 + pad; x += cell) for (let y = y0 - pad; y <= y1 + pad; y += cell) samples.push({ x, y, d: rings.some((r) => inRing(r, x, y)) });
  let q = { phi: init?.phi ?? 0, cx: (x0 + x1) / 2, cz: (y0 + y1) / 2, ha: (x1 - x0) / 2, hb: (y1 - y0) / 2, e: 2.4 };
  const score = (v: typeof q): number => {
    const co = Math.cos(v.phi), si = Math.sin(v.phi);
    let inter = 0, uni = 0;
    for (const s of samples) {
      const a = ((s.x - v.cx) * co + (s.y - v.cz) * si) / v.ha, b = ((s.x - v.cx) * si - (s.y - v.cz) * co) / v.hb;
      const m = Math.abs(a) ** v.e + Math.abs(b) ** v.e <= 1;
      if (m && s.d) inter++;
      if (m || s.d) uni++;
    }
    return inter / (uni || 1);
  };
  let best = score(q);
  for (let round = 0; round < 5; round++) {
    const step = 0.12 / (1 + round);
    for (const key of ["cx", "cz", "ha", "hb", "phi", "e"] as const) for (const dir of [-1, 1]) {
      for (let k = 0; k < 6; k++) {
        const t = { ...q };
        const d = key === "cx" || key === "cz" ? step * (key === "cx" ? x1 - x0 : y1 - y0) * 0.5 : key === "phi" ? step : key === "e" ? step * 3 : step * (key === "ha" ? q.ha : q.hb);
        t[key] = q[key] + dir * d;
        if (t.e < 2 || t.e > 8 || t.ha < 0.02 || t.hb < 0.02) break;
        const sc = score(t);
        if (sc > best + 1e-9) { best = sc; q = t; } else break;
      }
    }
  }
  return { phi: q.phi, centre: [q.cx, q.cz], ha: q.ha, hb: q.hb, exp: q.e, iou: best };
}

/** 🥚️ A curtain sheet: spindle loft whose section is the plan footprint's fitted superellipse, scaled along the height by the sheet's own front width profile; the z drift follows the front outline's centre. */
export function sheet(k: Kit, id: string, bone: string, P: Shapes, F: Shapes, m: { material: string; role: string; tone: number }, o: { n?: number; kx?: number; floor?: number; extra?: Record<string, unknown> } = {}): { fit: PlanFit; secs: Sec[] } | null {
  const fit = fitPlan(P);
  const ys = F.flatMap((s) => s.flatMap((r) => r.map((p) => p.y)));
  const y0 = Math.min(...ys), y1 = Math.max(...ys);
  const prof = (y: number): [number, number] => { const iv = ivalsAllF(F, y); return iv === null ? [0, 0] : iv; };
  const N = 48;
  const wid: number[] = [], mid: number[] = [], yy: number[] = [];
  for (let i = 0; i <= N; i++) { const y = y0 + 1e-3 + ((y1 - y0 - 2e-3) * i) / N; const [a, b] = prof(y); yy.push(y); wid.push(b - a); mid.push((a + b) / 2); }
  const wmax = Math.max(...wid);
  const zmid = (Math.min(...mid.filter((_, i) => wid[i]! > 0.1 * wmax)) + Math.max(...mid.filter((_, i) => wid[i]! > 0.1 * wmax))) / 2;
  const keep = dpIdx(yy, wid, o.floor ?? Math.max(k.feat * 2, 0.04) * wmax * 0.05);
  const n = o.n;
  const idx = n === undefined ? keep : Array.from({ length: n }, (_, i) => Math.round(((N) * (1 - Math.cos((Math.PI * i) / (n - 1)))) / 2));
  const secs: Sec[] = idx.map((i) => {
    const s = Math.max(0.02, wid[i]! / wmax);
    const dz = wid[i]! > 0.1 * wmax ? mid[i]! - zmid : 0;
    return k.secY([fit.centre[0], yy[i]!, fit.centre[1] + dz], fit.ha * s * (o.kx ?? 1), fit.hb * s * (o.kx ?? 1), fit.exp, fit.phi);
  });
  k.loft("body", id, bone, secs, m, o.extra ?? {});
  return { fit, secs };
}

function ivalsAllF(F: Shapes, y: number): [number, number] | null {
  let lo = Infinity, hi = -Infinity;
  for (const s of F) for (const r of s) for (let i = 0, j = r.length - 1; i < r.length; j = i++) {
    const a = r[j]!, b = r[i]!;
    if (a.y <= y !== b.y <= y) { const x = a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x); lo = Math.min(lo, x); hi = Math.max(hi, x); }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

function dpIdx(xs: number[], ys: number[], eps: number): number[] {
  const keep = new Set<number>([0, xs.length - 1]);
  const rec = (a: number, b: number): void => {
    let w = -1, wi = -1;
    for (let i = a + 1; i < b; i++) { const t = (xs[i]! - xs[a]!) / (xs[b]! - xs[a]!); const d = Math.abs(ys[a]! + t * (ys[b]! - ys[a]!) - ys[i]!); if (d > w) { w = d; wi = i; } }
    if (w > eps) { keep.add(wi); rec(a, wi); rec(wi, b); }
  };
  rec(0, xs.length - 1);
  return [...keep].sort((p, q) => p - q);
}

type Cell = { x: number; y: number };
/** 🫧️ Oriented superellipse fitted to a set of raster cells (centre, angle, half sizes, exponent) by coordinate descent on the intersection over union against the fitted shape sampled on the same grid. */
export function fitCells(cells: Cell[], cell: number, init?: { phi: number }, recall = 1): PlanFit | null {
  if (cells.length < 4) return null;
  const xs = cells.map((c) => c.x), ys = cells.map((c) => c.y);
  const x0 = Math.min(...xs), x1 = Math.max(...xs), y0 = Math.min(...ys), y1 = Math.max(...ys);
  const set = new Set(cells.map((c) => `${Math.round(c.x / cell)},${Math.round(c.y / cell)}`));
  const pad = Math.max(x1 - x0, y1 - y0) * 0.25;
  const samples: { x: number; y: number; d: boolean }[] = [];
  const stride = Math.max(1, Math.round(Math.max(x1 - x0, y1 - y0) / cell / 48));
  for (let x = x0 - pad; x <= x1 + pad; x += cell * stride) for (let y = y0 - pad; y <= y1 + pad; y += cell * stride) samples.push({ x, y, d: set.has(`${Math.round(x / cell)},${Math.round(y / cell)}`) });
  let sxx = 0, syy = 0, sxy = 0;
  const cx0 = xs.reduce((a, b) => a + b, 0) / xs.length, cy0 = ys.reduce((a, b) => a + b, 0) / ys.length;
  for (const c of cells) { sxx += (c.x - cx0) ** 2; syy += (c.y - cy0) ** 2; sxy += (c.x - cx0) * (c.y - cy0); }
  let q = { phi: init?.phi ?? 0.5 * Math.atan2(2 * sxy, sxx - syy), cx: (x0 + x1) / 2, cz: (y0 + y1) / 2, ha: (x1 - x0) / 2 + cell / 2, hb: (y1 - y0) / 2 + cell / 2, e: 2.4 };
  const score = (v: typeof q): number => {
    const co = Math.cos(v.phi), si = Math.sin(v.phi);
    let inter = 0, uni = 0;
    for (const s of samples) {
      const a = ((s.x - v.cx) * co + (s.y - v.cz) * si) / v.ha, b = ((s.x - v.cx) * si - (s.y - v.cz) * co) / v.hb;
      const m = Math.abs(a) ** v.e + Math.abs(b) ** v.e <= 1;
      if (m && s.d) inter++;
      else if (s.d) uni += recall;
      else if (m) uni++;
    }
    return inter / (inter + uni || 1);
  };
  let best = score(q);
  for (let round = 0; round < 6; round++) {
    const step = 0.12 / (1 + round);
    for (const key of ["cx", "cz", "ha", "hb", "phi", "e"] as const) for (const dir of [-1, 1]) {
      for (let k = 0; k < 8; k++) {
        const t = { ...q };
        const d = key === "cx" ? step * (x1 - x0) * 0.5 : key === "cz" ? step * (y1 - y0) * 0.5 : key === "phi" ? step : key === "e" ? step * 3 : step * (key === "ha" ? q.ha : q.hb);
        t[key] = q[key] + dir * d;
        if (t.e < 2 || t.e > 8 || t.ha < cell || t.hb < cell) break;
        const sc = score(t);
        if (sc > best + 1e-9) { best = sc; q = t; } else break;
      }
    }
  }
  return { phi: q.phi, centre: [q.cx, q.cz], ha: q.ha, hb: q.hb, exp: q.e, iou: best };
}

/** 🧱️ Visual-hull lobes of three silhouettes: at every station height the plan mask cut by the elevation and front strips, split at x = `split` (overlapping by `overlap`), each part fitted by an oriented superellipse, lofted along y. Returns the number of lofts. */
export function hullLobes(k: Kit, id: string, bone: string, E: Shapes, F: Shapes, P: Shapes, m: { material: string; role: string; tone: number }, o: { stations: number; splits: number[]; overlap: number; cell: number; inset?: number; recall?: number }): number {
  const pb = bbox(P), eb = bbox(E), fb = bbox(F);
  const y0 = Math.max(eb.minY, fb.minY), y1 = Math.min(eb.maxY, fb.maxY);
  const parts = o.splits.length + 1;
  const bounds = (i: number): [number, number] => [i === 0 ? -Infinity : o.splits[i - 1]! - o.overlap, i === parts - 1 ? Infinity : o.splits[i]! + o.overlap];
  const rings = P.flatMap((s) => s);
  const made: Sec[][] = Array.from({ length: parts }, () => []);
  const prevPhi: (number | undefined)[] = Array(parts).fill(undefined);
  for (let s = 0; s < o.stations; s++) {
    const y = y0 + ((y1 - y0) * (0.004 + 0.992 * (1 - Math.cos((Math.PI * s) / (o.stations - 1))) / 2));
    const ie = ivalsAllIv(E, "y", y), iff = ivalsAllIv(F, "y", y);
    for (let i = 0; i < parts; i++) {
      const [bl, bh] = bounds(i);
      const cells: Cell[] = [];
      for (let x = pb.minX; x <= pb.maxX; x += o.cell) {
        if (x < bl || x > bh || !ie.some((v) => x >= v[0] && x <= v[1])) continue;
        for (let z = pb.minY; z <= pb.maxY; z += o.cell) if (iff.some((v) => z >= v[0] && z <= v[1]) && rings.some((r) => inRing(r, x, z))) cells.push({ x, y: z });
      }
      const fit = fitCells(cells, o.cell, prevPhi[i] === undefined ? undefined : { phi: prevPhi[i]! }, o.recall ?? 1);
      if (fit === null) continue;
      if (fit.hb > fit.ha) { [fit.ha, fit.hb] = [fit.hb, fit.ha]; fit.phi += Math.PI / 2; }
      if (prevPhi[i] !== undefined) while (Math.abs(fit.phi - prevPhi[i]!) > Math.PI / 2) fit.phi += fit.phi > prevPhi[i]! ? -Math.PI : Math.PI;
      prevPhi[i] = fit.phi;
      const k0 = o.inset ?? 0;
      made[i]!.push(k.secY([fit.centre[0], y, fit.centre[1]], Math.max(fit.ha - k0, o.cell), Math.max(fit.hb - k0, o.cell), fit.exp, fit.phi));
    }
  }
  let n = 0;
  made.forEach((secs, i) => { if (k.loft("body", parts === 1 ? id : `${id}-${i + 1}`, bone, secs, m)) n++; });
  return n;
}

function bbox(S: Shapes): { minX: number; maxX: number; minY: number; maxY: number } {
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const s of S) for (const r of s) for (const p of r) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  return b;
}
function ivalsAllIv(S: Shapes, axis: "x" | "y", at: number): [number, number][] {
  const all: [number, number][] = [];
  for (const s of S) {
    const vals: number[] = [];
    for (const ring of s) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const a = ring[k]!, b = ring[i]!;
      const [pa, pb, qa, qb] = axis === "y" ? [a.y, b.y, a.x, b.x] : [a.x, b.x, a.y, b.y];
      if (pa <= at !== pb <= at) vals.push(qa + ((at - pa) / (pb - pa)) * (qb - qa));
    }
    vals.sort((p, q) => p - q);
    for (let n = 0; n + 1 < vals.length; n += 2) all.push([vals[n]!, vals[n + 1]!]);
  }
  all.sort((p, q) => p[0] - q[0]);
  const out: [number, number][] = [];
  for (const v of all) { const l = out[out.length - 1]; if (l !== undefined && v[0] <= l[1]) l[1] = Math.max(l[1], v[1]); else out.push([v[0], v[1]]); }
  return out;
}

import { simplifyRing } from "./author-47-lib.ts";

/** ✂️ A polygon clipped to the rectangle [x0, x1] x [y0, y1] (Sutherland-Hodgman, four half planes). */
export function clipRect(ring: P[], x0: number, x1: number, y0: number, y1: number): P[] {
  let out = ring;
  const edges: { inside: (p: P) => boolean; cut: (a: P, b: P) => P }[] = [
    { inside: (p) => p.x >= x0, cut: (a, b) => ({ x: x0, y: a.y + ((x0 - a.x) / (b.x - a.x)) * (b.y - a.y) }) },
    { inside: (p) => p.x <= x1, cut: (a, b) => ({ x: x1, y: a.y + ((x1 - a.x) / (b.x - a.x)) * (b.y - a.y) }) },
    { inside: (p) => p.y >= y0, cut: (a, b) => ({ x: a.x + ((y0 - a.y) / (b.y - a.y)) * (b.x - a.x), y: y0 }) },
    { inside: (p) => p.y <= y1, cut: (a, b) => ({ x: a.x + ((y1 - a.y) / (b.y - a.y)) * (b.x - a.x), y: y1 }) },
  ];
  for (const e of edges) {
    const inp = out;
    out = [];
    for (let i = 0; i < inp.length; i++) {
      const a = inp[i]!, b = inp[(i + 1) % inp.length]!;
      if (e.inside(a)) { out.push(a); if (!e.inside(b)) out.push(e.cut(a, b)); } else if (e.inside(b)) out.push(e.cut(a, b));
    }
    if (out.length === 0) break;
  }
  return out;
}
export const ringArea = (r: P[]): number => { let a = 0; for (let i = 0; i < r.length; i++) { const p = r[i]!, q = r[(i + 1) % r.length]!; a += p.x * q.y - q.x * p.y; } return Math.abs(a) / 2; };

/** 🧱️ Visual-hull frusta of three silhouettes: between consecutive heights the plan ring is cut to the elevation and front intervals at mid height and tapered so that its area follows the cut at the two heights. One extruded frustum per (elevation interval, front interval) pair. Returns the number of solids. */
export function hullSlabs(k: Kit, id: string, bone: string, E: Shapes, F: Shapes, P: Shapes, m: { material: string; role: string; tone: number }, o: { heights: number[]; tol: number; minArea: number; gap?: number; prism?: boolean; widest?: boolean }): number {
  const ring0 = P.flatMap((s) => s).reduce((a, r) => (ringArea(r) > ringArea(a) ? r : a));
  const ring = simplifyRing(ring0, o.tol);
  let n = 0;
  const near = (iv: [number, number][], v: number): [number, number] => iv.find((q) => v >= q[0] - 1e-6 && v <= q[1] + 1e-6) ?? iv.reduce((a, q) => (Math.abs((q[0] + q[1]) / 2 - v) < Math.abs((a[0] + a[1]) / 2 - v) ? q : a), [v, v] as [number, number]);
  const gap = o.gap ?? 0;
  const merged = (iv: [number, number][]): [number, number][] => { const out: [number, number][] = []; for (const v of iv) { const l = out[out.length - 1]; if (l !== undefined && v[0] - l[1] <= gap) l[1] = Math.max(l[1], v[1]); else out.push([v[0], v[1]]); } return out; };
  const pieceAt = (y: number, x: number, z: number): P[] => {
    const ie = merged(ivalsAllIv(E, "y", y)), iz = merged(ivalsAllIv(F, "y", y));
    if (ie.length === 0 || iz.length === 0) return [];
    const a = near(ie, x), b = near(iz, z);
    return clipRect(ring, a[0], a[1], b[0], b[1]);
  };
  for (let i = 0; i + 1 < o.heights.length; i++) {
    const ya = o.heights[i]!, yb = o.heights[i + 1]!, ym = (ya + yb) / 2;
    for (const ie of merged(ivalsAllIv(E, "y", ym))) for (const iz of merged(ivalsAllIv(F, "y", ym))) {
      const mid = clipRect(ring, ie[0], ie[1], iz[0], iz[1]);
      if (mid.length < 3 || ringArea(mid) < o.minArea) continue;
      const cx = mid.reduce((a, p) => a + p.x, 0) / mid.length, cz = mid.reduce((a, p) => a + p.y, 0) / mid.length;
      const levels = [ya + 1e-3, ya + 0.25 * (yb - ya), ym, yb - 0.25 * (yb - ya), yb - 1e-3];
      const pieces = levels.map((y) => pieceAt(y, cx, cz));
      const areas = pieces.map((q) => (q.length >= 3 ? ringArea(q) : 0));
      const bi = o.prism === true && o.widest !== true ? 2 : areas.indexOf(Math.max(...areas));
      const base = pieces[bi]!;
      if (base.length < 3 || areas[bi]! < o.minArea) continue;
      const sa = Math.sqrt(Math.max(areas[0]!, 1e-6) / areas[bi]!), sb = Math.sqrt(Math.max(areas[4]!, 1e-6) / areas[bi]!);
      k.extrudeY("body", `${id}-${++n}`, bone, simplifyRing(base, o.tol * 0.5), ya, yb, o.prism === true ? 1 : Math.min(Math.max(sa, 0.05), 1), o.prism === true ? 1 : Math.min(Math.max(sb, 0.05), 1), m);
    }
  }
  return n;
}

/** ✂️ The part of a tube path between the points nearest (in elevation) to two world points; sections between are kept, the ends interpolated. `null` when the nearest points coincide. */
export function subPath(secs: Sec[], a: V3, b: V3): Sec[] | null {
  const param = (p: V3): number => {
    let best = Infinity, bt = 0;
    for (let i = 0; i + 1 < secs.length; i++) {
      const s = secs[i]!.c, e = secs[i + 1]!.c;
      const dx = e[0] - s[0], dy = e[1] - s[1];
      const L2 = dx * dx + dy * dy || 1e-12;
      const f = Math.min(1, Math.max(0, ((p[0] - s[0]) * dx + (p[1] - s[1]) * dy) / L2));
      const d = Math.hypot(p[0] - (s[0] + f * dx), p[1] - (s[1] + f * dy));
      if (d < best) { best = d; bt = i + f; }
    }
    return bt;
  };
  let ta = param(a), tb = param(b);
  if (ta > tb) [ta, tb] = [tb, ta];
  if (tb - ta < 0.02) return null;
  const at = (t: number): Sec => {
    const i = Math.min(secs.length - 2, Math.floor(t)), f = t - i;
    const s = secs[i]!, e = secs[i + 1]!;
    const mix = (u: V3, v: V3): V3 => [u[0] + f * (v[0] - u[0]), u[1] + f * (v[1] - u[1]), u[2] + f * (v[2] - u[2])];
    return { c: mix(s.c, e.c), ax: s.ax, ay: s.ay, rx: s.rx + f * (e.rx - s.rx), ry: s.ry + f * (e.ry - s.ry), exp: s.exp };
  };
  const out: Sec[] = [at(ta)];
  for (let i = Math.ceil(ta + 1e-6); i < tb - 1e-6; i++) if (i < secs.length) out.push(secs[i]!);
  out.push(at(tb));
  return out;
}
