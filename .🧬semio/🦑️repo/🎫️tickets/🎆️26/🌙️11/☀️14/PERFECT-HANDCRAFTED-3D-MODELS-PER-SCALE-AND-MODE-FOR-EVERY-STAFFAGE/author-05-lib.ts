import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { Mask, TICKET, featureM, loadAsset, poseState, resolveAsset, toLocal, type Evaluated, type Json, type P2, type V3 } from "./volumes-lib.ts";

import { toWorld as toWorldLocal } from "./volumes-base.ts";
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const rv = (v: readonly number[]): number[] => v.map(r4);
const MIN = 0.004;
const FINE = /eye|nostril|nose|pad|teat|blaze|patch|speculum|mantle|tongue|beard|ring|cere|wingtip|forelock/;
const ALONG_U = [0.5, 0.5, 0.5, 0.5];

export type Fit = { zc?: number; zh?: number; zhK?: number; from?: string; ex?: number; fracs?: number[]; p?: number; zhAt?: (f: number) => number; end?: number; endStart?: number; vShift?: number; vScale?: number; bone?: string; maxDen?: number | null; mat?: string; tone?: number; region?: string; regions?: string[]; role?: string; ring?: number; auto?: boolean; endZ?: number; adapt?: boolean; uLo?: number; uHi?: number; flatStart?: boolean; flatEnd?: boolean };
export type Sec = { p: V3; rz: number; ry: number; ex?: number };
export type Extra = { mat?: string; tone?: number; region?: string; regions?: string[]; role?: string; maxDen?: number | null; tags?: string[]; weights?: { bone: string; weight: number }[]; repeat?: Json };

const sub3 = (a: readonly number[], b: readonly number[]): V3 => [a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!];
const norm3 = (a: V3): V3 => {
  const l = Math.hypot(a[0], a[1], a[2]) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

function basisQuat(d: V3): number[] {
  const n = norm3([-d[1], d[0], 0]);
  const y: V3 = [d[1] * n[2] - d[2] * n[1], d[2] * n[0] - d[0] * n[2], d[0] * n[1] - d[1] * n[0]];
  const m = [[n[0], y[0], d[0]], [n[1], y[1], d[1]], [n[2], y[2], d[2]]];
  const t = m[0]![0]! + m[1]![1]! + m[2]![2]!;
  let q: number[];
  if (t > 0) {
    const s = Math.sqrt(t + 1) * 2;
    q = [(m[2]![1]! - m[1]![2]!) / s, (m[0]![2]! - m[2]![0]!) / s, (m[1]![0]! - m[0]![1]!) / s, s / 4];
  } else if (m[0]![0]! > m[1]![1]! && m[0]![0]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[0]![0]! - m[1]![1]! - m[2]![2]!) * 2;
    q = [s / 4, (m[0]![1]! + m[1]![0]!) / s, (m[0]![2]! + m[2]![0]!) / s, (m[2]![1]! - m[1]![2]!) / s];
  } else if (m[1]![1]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[1]![1]! - m[0]![0]! - m[2]![2]!) * 2;
    q = [(m[0]![1]! + m[1]![0]!) / s, s / 4, (m[1]![2]! + m[2]![1]!) / s, (m[0]![2]! - m[2]![0]!) / s];
  } else {
    const s = Math.sqrt(1 + m[2]![2]! - m[0]![0]! - m[1]![1]!) * 2;
    q = [(m[0]![2]! + m[2]![0]!) / s, (m[1]![2]! + m[2]![1]!) / s, s / 4, (m[1]![0]! - m[0]![1]!) / s];
  }
  return q.map((x) => Math.round(x * 1e6) / 1e6);
}


export class Hull {
  readonly c: number;
  private readonly E: Mask;
  private readonly P: Mask;
  private readonly F: Mask;

  constructor(state: ReturnType<typeof poseState>, feature: number) {
    this.c = Math.max(feature * 0.5, 0.0025);
    const shapes = (v: "elevation" | "front" | "plan") => state[v].filter((e: Json) => e.closed && ["body", "detail", "glazing"].includes(e.shape.role) && e.rings.length > 0);
    const ext = (v: "elevation" | "front" | "plan") => {
      const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
      for (const e of shapes(v)) for (const r of e.rings) for (const p of r) {
        b.minX = Math.min(b.minX, p.x);
        b.maxX = Math.max(b.maxX, p.x);
        b.minY = Math.min(b.minY, p.y);
        b.maxY = Math.max(b.maxY, p.y);
      }
      return { ...b, minX: b.minX - 2 * this.c, minY: b.minY - 2 * this.c, maxX: b.maxX + 2 * this.c, maxY: b.maxY + 2 * this.c };
    };
    const mk = (v: "elevation" | "front" | "plan"): Mask => {
      const m = new Mask(ext(v), this.c);
      for (const e of shapes(v)) m.fill(e.rings);
      return m;
    };
    this.E = mk("elevation");
    this.F = mk("front");
    this.P = mk("plan");
  }

  private at(m: Mask, x: number, y: number): boolean {
    const i = Math.floor((x - m.box.minX) / this.c), j = Math.floor((y - m.box.minY) / this.c);
    if (i < 0 || j < 0 || i >= m.width || j >= m.height) return false;
    return m.cells[j * m.width + i] === 1;
  }

  inside(x: number, y: number, z: number): boolean {
    return this.at(this.E, x, y) && this.at(this.P, x, z) && this.at(this.F, z, y);
  }
}

export class Author {
  readonly asset: Json;
  readonly f: Record<string, { o: V3; u: V3; v: V3; w: V3 }>;
    readonly solids: Json[] = [];
  readonly decisions: { en: string; de: string }[] = [];
  private readonly rest: ReturnType<typeof poseState>;
  private readonly regionMaterial = new Map<string, string>();
  private readonly shows = new Map<string, number>();
  readonly slug: string;

  readonly scale: number;
  readonly hull: Hull;
  readonly minHalf: number;

  constructor(slug: string, scale: number) {
    this.slug = slug;
    this.scale = scale;
    this.asset = loadAsset(resolveAsset(slug).path);
    this.rest = poseState(this.asset, scale);
    this.f = this.rest.frames as never;
    this.minHalf = featureM(scale, this.asset) * 0.52;
    this.hull = new Hull(this.rest, featureM(scale, this.asset));
    for (const region of this.asset.thermal?.regions ?? []) if (region.material !== undefined) this.regionMaterial.set(region.id, region.material);
    for (const [key, rep] of Object.entries<Json>(this.asset.representations)) {
      const g = rep.geometry;
      if (g === undefined) continue;
      for (const view of ["elevation", "front", "plan"]) for (const s of g[view] ?? []) this.shows.set(s.id, Math.max(this.shows.get(s.id) ?? 0, Number(key)));
    }
  }

  get coarsest(): number {
    return Math.max(...Object.entries<Json>(this.asset.representations).filter(([, rep]) => rep.geometry !== undefined).map(([k]) => Number(k)));
  }

  denominatorOf(...ids: string[]): number | undefined {
    const m = Math.max(0, ...ids.map((id) => this.shows.get(id) ?? 0));
    return m === 0 || m >= this.coarsest ? undefined : m;
  }

  elevation(id: string): Evaluated {
    const e = this.rest.elevation.find((s) => s.shape.id === id);
    if (e === undefined) throw new Error(`${this.slug}: no elevation shape "${id}"`);
    return e;
  }

  tryShape(id: string): Json | undefined {
    for (const v of ["elevation", "front", "plan"] as const) {
      const e = this.rest[v].find((q) => q.shape.id === id);
      if (e !== undefined) return e.shape;
    }
    return undefined;
  }

  anyShape(id: string): Json {
    for (const v of ["elevation", "front", "plan"] as const) {
      const e = this.rest[v].find((q) => q.shape.id === id);
      if (e !== undefined) return e.shape;
    }
    throw new Error(`${this.slug}: no shape "${id}"`);
  }

  local(bone: string, p: readonly number[]): V3 {
    return toLocal(this.f[bone]!, p as V3).map(r4) as V3;
  }

  angle(bone: string): number {
    const u = this.f[bone]!.u;
    return Math.atan2(u[1], u[0]);
  }

  private meta(extra: Extra, shapeId?: string, bone?: string): Json {
    const shape = shapeId === undefined ? undefined : this.tryShape(shapeId);
    const region = extra.region ?? (extra.regions === undefined ? shape?.region : undefined);
    const regions = extra.regions ?? (extra.region === undefined ? shape?.regions : undefined);
    const material = extra.mat ?? (region === undefined ? undefined : this.regionMaterial.get(region)) ?? (regions === undefined ? undefined : this.regionMaterial.get(regions[0]!));
    if (material === undefined) throw new Error(`${this.slug}: no material for ${shapeId ?? bone}`);
    const out: Json = { material, role: extra.role ?? shape?.role ?? "body", tone: extra.tone ?? shape?.tone ?? 0.5 };
    if (region !== undefined) out.region = region;
    if (regions !== undefined) out.regions = regions;
    return out;
  }

  private place(id: string, kind: string, bone: string, meta: Json, geometry: Json, extra: Extra, shapeIds: string[]): Json {
    if (this.solids.some((s) => s.id === id)) throw new Error(`${this.slug}: duplicate solid "${id}"`);
    const solid: Json = { id, kind, bone, ...meta, ...geometry };
    if (extra.tags !== undefined) solid.tags = extra.tags;
    if (extra.weights !== undefined) solid.weights = extra.weights;
    if (extra.repeat !== undefined) solid.repeat = extra.repeat;
    this.solids.push(solid);
    return solid;
  }

  private cut(ring: P2[], bone: string, u: number): [number, number] | null {
    const fr = this.f[bone]!;
    const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
    let lo = Infinity, hi = -Infinity;
    for (let i = 0; i < loc.length; i++) {
      const a = loc[i]!, b = loc[(i + 1) % loc.length]!;
      if ((a[0] - u) * (b[0] - u) <= 0 && a[0] !== b[0]) {
        const v = a[1] + ((u - a[0]) / (b[0] - a[0])) * (b[1] - a[1]);
        lo = Math.min(lo, v);
        hi = Math.max(hi, v);
      }
    }
    return lo === Infinity ? null : [lo, hi];
  }

  lat(shapeId: string): [number, number] | null {
    for (const v of ["plan", "front"] as const) {
      const e = this.rest[v].find((q) => q.shape.id === shapeId);
      if (e === undefined) continue;
      const zs = e.rings.flat().map((p) => (v === "plan" ? p.y : p.x));
      if (zs.length > 0) return [Math.min(...zs), Math.max(...zs)];
    }
    return null;
  }


  centreOf(shapeId: string): V3 | null {
    const e = this.rest.elevation.find((q) => q.shape.id === shapeId);
    if (e === undefined) return null;
    const pts = e.rings.flat();
    const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
    const [x, y] = [(Math.min(...xs) + Math.max(...xs)) / 2, (Math.min(...ys) + Math.max(...ys)) / 2];
    const f = this.rest.front.find((q) => q.shape.id === shapeId);
    const z = f === undefined ? 0 : (() => { const zs = f.rings.flat().map((p) => p.x); return (Math.min(...zs) + Math.max(...zs)) / 2; })();
    return [x, y, z];
  }

  zOf(shapeId: string): number | null {
    const f = this.rest.front.find((q) => q.shape.id === shapeId);
    if (f === undefined) return null;
    const zs = f.rings.flat().map((p) => p.x);
    return (Math.min(...zs) + Math.max(...zs)) / 2;
  }

  private readonly sourceCache = new Map<string, string>();
  source(shapeId: string, view: "front" | "plan"): string {
    const key = `${shapeId}|${view}`;
    const hit = this.sourceCache.get(key);
    if (hit !== undefined) return hit;
    let out = shapeId;
    if (!this.rest[view].some((q) => q.shape.id === shapeId)) {
      const el = this.rest.elevation.find((q) => q.shape.id === shapeId);
      let bone: string | null = el?.shape.bone ?? null;
      const bones = new Map<string, Json>((this.asset.rig.bones as Json[]).map((b) => [b.id, b]));
      while (bone !== null && bone !== undefined) {
        const found = this.rest[view].find((q) => q.shape.bone === bone && q.shape.role === "body");
        if (found !== undefined) {
          out = found.shape.id;
          break;
        }
        bone = bones.get(bone)?.parent ?? null;
      }
    }
    this.sourceCache.set(key, out);
    return out;
  }

  latCut(shapeId: string, wx: number, wy: number, vertical: boolean): [number, number] | null {
    const view = vertical ? "front" : "plan";
    const e = this.rest[view].find((q) => q.shape.id === this.source(shapeId, view));
    if (e === undefined) return null;
    let lo = Infinity, hi = -Infinity;
    for (const ring of e.rings) {
      for (let i = 0; i < ring.length; i++) {
        const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
        const [pa, pb, key, pos] = vertical ? [a.x, b.x, [a.y, b.y], wy] : [a.y, b.y, [a.x, b.x], wx];
        const k0 = key[0]!, k1 = key[1]!;
        if ((k0 - pos) * (k1 - pos) <= 0 && k0 !== k1) {
          const z = pa + ((pos - k0) / (k1 - k0)) * (pb - pa);
          lo = Math.min(lo, z);
          hi = Math.max(hi, z);
        }
      }
    }
    return lo === Infinity ? null : [lo, hi];
  }

  envCut(wx: number, wy: number, vertical: boolean): [number, number] | null {
    const view = vertical ? "front" : "plan";
    let lo = Infinity, hi = -Infinity;
    for (const e of this.rest[view]) {
      if (e.shape.role !== "body" || !e.closed) continue;
      const q = this.latCut(e.shape.id, wx, wy, vertical);
      if (q !== null) {
        lo = Math.min(lo, q[0]);
        hi = Math.max(hi, q[1]);
      }
    }
    return lo === Infinity ? null : [lo, hi];
  }

  fit(id: string, shapeId: string, o0: Fit, extra: Extra = {}): Json {
    const l = this.lat(o0.from ?? shapeId);
    const o: Fit = { ...o0, zc: o0.zc ?? (l === null ? 0 : (l[0] + l[1]) / 2), zh: o0.zh !== undefined ? o0.zh * (o0.zhK ?? 1) : l === null ? undefined : ((l[1] - l[0]) / 2) * (o0.zhK ?? 1) };
    const e = this.elevation(shapeId);
    const bone = o.bone ?? e.shape.bone;
    const ring = e.rings[o.ring ?? 0]!;
    const fr = this.f[bone]!;
    const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
    const u0 = o.uLo ?? Math.min(...loc.map((p) => p[0])), u1 = o.uHi ?? Math.max(...loc.map((p) => p[0]));
    const fracs = o.fracs ?? [0, 0.03, 0.12, 0.28, 0.46, 0.64, 0.82, 0.94, 0.985, 1];
    const rows = fracs.map((fr_) => {
      const u = u0 + (u1 - u0) * fr_;
      const c = this.cut(ring, bone, Math.min(u1 - 1e-6, Math.max(u0 + 1e-6, u)));
      return { fr: fr_, u, c };
    });
    const half = (c: [number, number] | null) => (c === null ? 0 : (c[1] - c[0]) / 2);
    const ryMax = Math.max(...rows.map((x) => half(x.c)));
    const p = o.p ?? 0.5;
    if (o.zh === undefined) o.zh = ryMax * 0.8 * (o0.zhK ?? 1);
    const sections = rows.map((x, k) => {
      let c = x.c;
      let last = k === 0 || k === rows.length - 1;
      if (last && ((k === 0 && o.flatStart) || (k !== 0 && o.flatEnd))) last = false;
      if (c === null || last) {
        const near = rows[k === 0 ? 1 : rows.length - 2]!.c!;
        c = [(near[0] + near[1]) / 2, (near[0] + near[1]) / 2];
      }
      const vc = ((c[0] + c[1]) / 2) * (o.vScale ?? 1) + (o.vShift ?? 0);
      let ry = Math.max(last ? MIN : this.minHalf, half(c) * (o.vScale ?? 1));
      let zh = o.zhAt === undefined ? o.zh! : o.zhAt(x.fr);
      let rz = Math.max(MIN, zh * Math.pow(Math.max(0.0001, half(c)) / ryMax, p));
      let zcs = o.zc ?? 0;
      if (o.auto && !last) {
        const wp = toWorldLocal(fr, [x.u, vc, 0]);
        const vertical = Math.abs(fr.u[1]) > Math.abs(fr.u[0]);
        const id0 = o.from ?? shapeId;
        const prim = vertical ? this.latCut(id0, wp[0], wp[1], true) : this.latCut(id0, wp[0], wp[1], false);
        const hs = half(c);
        if (prim !== null) {
          const zc0 = (prim[0] + prim[1]) / 2;
          let rz0 = Math.max(MIN, (prim[1] - prim[0]) / 2);
          const n = o.ex ?? 2;
          for (let t = 1; t < 12; t++) {
            const th = -Math.PI / 2 + (Math.PI * t) / 12;
            const sn = Math.sin(th), cs = Math.pow(Math.abs(Math.cos(th)), 2 / n);
            if (cs < 0.2) continue;
            const wt = toWorldLocal(fr, [x.u, vc + sn * hs, 0]);
            const q = vertical ? this.latCut(id0, wt[0], wt[1], false) : this.latCut(id0, wt[0], wt[1], true);
            if (q === null) continue;
            const lim = Math.min((q[1] - zc0) / cs, (zc0 - q[0]) / cs);
            if (lim > 2 * MIN) rz0 = Math.min(rz0, lim);
          }
          zcs = zc0;
          rz = rz0;
        }
        const envs = [this.envCut(wp[0], wp[1], false), this.envCut(wp[0], wp[1], true)];
        for (const env of envs) {
          if (env === null) continue;
          const lo = Math.max(zcs - rz, env[0]), hi = Math.min(zcs + rz, env[1]);
          if (hi - lo > 2 * MIN) {
            zcs = (lo + hi) / 2;
            rz = (hi - lo) / 2;
          } else {
            zcs = Math.min(env[1] - 2 * MIN, Math.max(env[0] + 2 * MIN, zcs));
            rz = 2 * MIN;
          }
        }
      }
      if (last) {
        const k0 = k === 0 ? (o.endStart ?? o.end ?? 0.1) : (o.end ?? 0.1);
        ry = Math.max(MIN, ry * 0 + ryMax * k0 * 0.25);
        rz = Math.max(MIN, zh * (o.endZ ?? k0 * 0.25));
      }
      if (!last) rz = Math.max(this.minHalf, rz);
      const s: Json = { at: rv([x.u, vc, zcs - fr.o[2]]), rx: r4(ry), ry: r4(rz), rotation: ALONG_U };
      if ((o.ex ?? 2) !== 2) s.exponent = o.ex;
      if (o.auto && !last && o.adapt !== false) {
        let best = o.ex ?? 2, bestMiss = Infinity;
        for (const n of [2, 2.5, 3, 4, 6, 9]) {
          let miss = 0;
          for (let a = -5; a <= 5; a++) for (let b = -5; b <= 5; b++) {
            const dv = (a / 5.5) * ry, dz = (b / 5.5) * rz;
            const insideSE = Math.pow(Math.abs(dv) / ry, n) + Math.pow(Math.abs(dz) / rz, n) <= 1;
            const w = toWorldLocal(fr, [x.u, vc + dv, zcs - fr.o[2] + dz]);
            const insideH = this.hull.inside(w[0], w[1], w[2]);
            if (insideSE !== insideH) miss++;
          }
          if (miss < bestMiss - 1) { bestMiss = miss; best = n; }
        }
        s.exponent = best;
      }
      return s;
    });
    const meta = this.meta({ ...extra, mat: o.mat ?? extra.mat, tone: o.tone ?? extra.tone, region: o.region ?? extra.region, regions: o.regions ?? extra.regions, role: o.role ?? extra.role }, shapeId, bone);
    return this.place(id, "loft", bone, meta, { sections, capStart: true, capEnd: true }, o.maxDen === undefined ? extra : { ...extra, maxDen: o.maxDen }, [shapeId]);
  }

  mirror(id: string, shapeId: string, o: Fit, zc: number, extra: Extra = {}): void {
    this.fit(id, shapeId, { ...o, zc }, extra);
  }

  tube(id: string, bone: string, secs: Sec[], shapeIds: string[], extra: Extra = {}, ends: { start?: boolean; end?: boolean } = {}): Json {
    const pts = secs.map((s) => this.local(bone, s.p));
    const sections = secs.map((s, i) => {
      const d = norm3(sub3(pts[Math.min(pts.length - 1, i + 1)]!, pts[Math.max(0, i - 1)]!));
      const o: Json = { at: pts[i], rx: r4(Math.max(MIN, s.ry)), ry: r4(Math.max(MIN, s.rz)), rotation: basisQuat(d) };
      if (s.ex !== undefined && s.ex !== 2) o.exponent = s.ex;
      return o;
    });
    const meta = this.meta(extra, shapeIds[0], bone);
    return this.place(id, "loft", bone, meta, { sections, capStart: ends.start ?? true, capEnd: ends.end ?? true }, extra, shapeIds);
  }

  ell(id: string, bone: string, c: V3, radii: V3, ang: number, shapeIds: string[], extra: Extra = {}): Json {
    const a = (ang * Math.PI) / 180 - this.angle(bone);
    const geometry: Json = { radii: rv(radii) };
    const transform: Json = { translation: this.local(bone, c) };
    if (Math.abs(a) > 1e-6) transform.rotation = [0, 0, Math.sin(a / 2), Math.cos(a / 2)].map((x) => Math.round(x * 1e6) / 1e6);
    geometry.transform = transform;
    return this.place(id, "ellipsoid", bone, this.meta(extra, shapeIds[0], bone), geometry, extra, shapeIds);
  }

  box(id: string, bone: string, c: V3, size: V3, ang: number, shapeIds: string[], extra: Extra = {}, cornerRadius?: number): Json {
    const a = (ang * Math.PI) / 180 - this.angle(bone);
    const geometry: Json = { size: rv(size) };
    if (cornerRadius !== undefined) geometry.cornerRadius = cornerRadius;
    const transform: Json = { translation: this.local(bone, c) };
    if (Math.abs(a) > 1e-6) transform.rotation = [0, 0, Math.sin(a / 2), Math.cos(a / 2)].map((x) => Math.round(x * 1e6) / 1e6);
    geometry.transform = transform;
    return this.place(id, "box", bone, this.meta(extra, shapeIds[0], bone), geometry, extra, shapeIds);
  }

  cap(id: string, bone: string, a: V3, b: V3, r0: number, r1: number, shapeIds: string[], extra: Extra = {}, lat?: number): Json {
    const p0 = this.local(bone, a), p1 = this.local(bone, b);
    const geometry: Json = { p0, p1, r0: r4(r0), r1: r4(r1) };
    if (lat !== undefined) {
      const d = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
      const l = Math.hypot(d[0]!, d[1]!, d[2]!);
      if (Math.abs(d[0]! / l) < 0.999) throw new Error(`${this.slug}/${id}: lateralScale needs a capsule along the bone axis u`);
      geometry.lateralScale = lat;
    }
    return this.place(id, "capsule", bone, this.meta(extra, shapeIds[0], bone), geometry, extra, shapeIds);
  }

  revolve(id: string, bone: string, profile: [number, number][], axis: V3, origin: V3, lat: number | undefined, shapeIds: string[], extra: Extra = {}): Json {
    const geometry: Json = { profile: profile.map((p) => [r4(p[0]), r4(p[1])]), axis, origin: this.local(bone, origin) };
    if (lat !== undefined) geometry.lateralScale = lat;
    return this.place(id, "revolve", bone, this.meta(extra, shapeIds[0], bone), geometry, extra, shapeIds);
  }

  extrude(id: string, bone: string, profile: [number, number][], plane: "xy" | "yz" | "zx", from: number, to: number, shapeIds: string[], extra: Extra = {}): Json {
    return this.place(id, "extrude", bone, this.meta(extra, shapeIds[0], bone), { profile: profile.map((p) => [r4(p[0]), r4(p[1])]), plane, from: r4(from), to: r4(to) }, extra, shapeIds);
  }

  surface(id: string, x: number, y: number): number | null {
    const s = this.solids.find((q) => q.id === id)!;
    const p = this.local(s.bone, [x, y, 0]);
    const secs: Json[] = s.sections;
    let k = 0;
    while (k < secs.length - 2 && secs[k + 1].at[0] < p[0]) k++;
    const a = secs[k], b = secs[k + 1];
    const t = Math.min(1, Math.max(0, (p[0] - a.at[0]) / (b.at[0] - a.at[0] || 1)));
    const mix = (x0: number, x1: number) => x0 + (x1 - x0) * t;
    const vc = mix(a.at[1], b.at[1]), zc = mix(a.at[2], b.at[2]), ry = mix(a.rx, b.rx), rx = mix(a.ry, b.ry), n = mix(a.exponent ?? 2, b.exponent ?? 2);
    const d = Math.abs(p[1] - vc) / ry;
    if (d >= 1) return null;
    return Math.abs(zc) + rx * Math.pow(1 - Math.pow(d, n), 1 / n);
  }

  patch(id: string, shapeId: string, bodyId: string, side: 1 | -1, lift = 0.014, extra: Extra = {}): Json {
    const e = this.elevation(shapeId);
    const bone = e.shape.bone;
    const body = this.solids.find((q) => q.id === bodyId)!;
    const fr = this.f[bone]!;
    const ring = e.rings[0]!;
    const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
    const u0 = Math.min(...loc.map((p) => p[0])), u1 = Math.max(...loc.map((p) => p[0]));
    const fracs = [0, 0.04, 0.14, 0.3, 0.5, 0.7, 0.86, 0.96, 1];
    const sections: Json[] = [];
    for (const f_ of fracs) {
      const u = u0 + (u1 - u0) * f_;
      const c = this.cut(ring, bone, Math.min(u1 - 1e-6, Math.max(u0 + 1e-6, u)));
      if (c === null) continue;
      const vc = (c[0] + c[1]) / 2;
      const w = toWorldLocal(fr, [u, vc, 0]);
      const zb = this.surface(bodyId, w[0], w[1]);
      if (zb === null) continue;
      const end = f_ === 0 || f_ === 1;
      sections.push({ at: rv([u, vc, side * (zb - 0.004)]), rx: r4(Math.max(MIN, ((c[1] - c[0]) / 2) * (end ? 0.3 : 1))), ry: r4(end ? lift * 0.4 : lift), rotation: ALONG_U });
    }
    const meta = this.meta(extra, shapeId, bone);
    return this.place(id, "loft", bone, meta, { sections, capStart: true, capEnd: true }, extra, [shapeId]);
  }

  stripe(id: string, bodyId: string, sign: 1 | -1, u0: number, u1: number, half: number, lift: number, shapeIds: string[], extra: Extra = {}, n = 6): Json {
    const body = this.solids.find((q) => q.id === bodyId)!;
    const secs: Json[] = body.sections;
    const at = (u: number) => {
      let k = 0;
      while (k < secs.length - 2 && secs[k + 1].at[0] < u) k++;
      const a = secs[k], b = secs[k + 1];
      const t = Math.min(1, Math.max(0, (u - a.at[0]) / (b.at[0] - a.at[0] || 1)));
      const mix = (x0: number, x1: number) => x0 + (x1 - x0) * t;
      return { vc: mix(a.at[1], b.at[1]), sag: mix(a.rx, b.rx) };
    };
    const sections: Json[] = [];
    for (let i = 0; i <= n; i++) {
      const u = u0 + ((u1 - u0) * i) / n;
      const { vc, sag } = at(u);
      const edge = i === 0 || i === n;
      sections.push({ at: rv([u, vc + sign * (sag - lift * 0.5), 0]), rx: r4(lift), ry: r4(edge ? half * 0.5 : half), rotation: ALONG_U });
    }
    return this.place(id, "loft", body.bone, this.meta(extra, shapeIds[0], body.bone), { sections, capStart: true, capEnd: true }, extra, shapeIds);
  }

  shell(id: string, bodyId: string, sign: 1 | -1, u0: number, u1: number, depthK: number, lift: number, shapeIds: string[], extra: Extra = {}, n = 7): Json {
    const body = this.solids.find((q) => q.id === bodyId)!;
    const secs: Json[] = body.sections;
    const sections: Json[] = [];
    for (let i = 0; i <= n; i++) {
      const u = u0 + ((u1 - u0) * i) / n;
      let k = 0;
      while (k < secs.length - 2 && secs[k + 1].at[0] < u) k++;
      const a = secs[k], b = secs[k + 1];
      const t = Math.min(1, Math.max(0, (u - a.at[0]) / (b.at[0] - a.at[0] || 1)));
      const mix = (x0: number, x1: number) => x0 + (x1 - x0) * t;
      const vc = mix(a.at[1], b.at[1]), zc = mix(a.at[2], b.at[2]), sag = mix(a.rx, b.rx), lat = mix(a.ry, b.ry), ex = mix(a.exponent ?? 2, b.exponent ?? 2);
      const off = sag * (1 - depthK / 2);
      const width = lat * Math.pow(Math.max(0, 1 - Math.pow(1 - depthK / 2, ex)), 1 / ex);
      const edge = i === 0 || i === n;
      const fall = edge ? 0.4 : 1;
      sections.push({ at: rv([u, vc + sign * (off + lift / 2), zc]), rx: r4(Math.max(MIN, (sag * depthK / 2 + lift / 2) * fall)), ry: r4(Math.max(MIN, (width + lift) * fall)), rotation: ALONG_U });
    }
    return this.place(id, "loft", body.bone, this.meta(extra, shapeIds[0], body.bone), { sections, capStart: true, capEnd: true }, extra, shapeIds);
  }

  slice(id: string, bodyId: string, u0: number, u1: number, lift: number, shapeIds: string[], extra: Extra = {}, n = 6): Json {
    const body = this.solids.find((q) => q.id === bodyId)!;
    const secs: Json[] = body.sections;
    const sections: Json[] = [];
    for (let i = 0; i <= n; i++) {
      const u = u0 + ((u1 - u0) * i) / n;
      let k = 0;
      while (k < secs.length - 2 && secs[k + 1].at[0] < u) k++;
      const a = secs[k], b = secs[k + 1];
      const t = Math.min(1, Math.max(0, (u - a.at[0]) / (b.at[0] - a.at[0] || 1)));
      const mix = (x0: number, x1: number) => x0 + (x1 - x0) * t;
      const sec: Json = { at: rv([u, mix(a.at[1], b.at[1]), mix(a.at[2], b.at[2])]), rx: r4(mix(a.rx, b.rx) + lift), ry: r4(mix(a.ry, b.ry) + lift), rotation: ALONG_U };
      const ex = mix(a.exponent ?? 2, b.exponent ?? 2);
      if (Math.abs(ex - 2) > 1e-6) sec.exponent = r4(ex);
      sections.push(sec);
    }
    return this.place(id, "loft", body.bone, this.meta(extra, shapeIds[0], body.bone), { sections, capStart: true, capEnd: true }, extra, shapeIds);
  }

  decide(en: string, de: string): void {
    this.decisions.push({ en, de });
  }

}
