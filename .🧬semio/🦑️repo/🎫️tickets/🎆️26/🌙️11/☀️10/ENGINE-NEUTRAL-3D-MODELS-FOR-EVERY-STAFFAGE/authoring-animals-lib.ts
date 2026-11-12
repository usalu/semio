import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { TICKET, frames, loadAsset, resolveAsset, restState, toLocal, type Evaluated, type Json, type P2, type V3 } from "./volume-lib.ts";

import { toWorld as toWorldLocal } from "./volume-lib.ts";
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const rv = (v: readonly number[]): number[] => v.map(r4);
const MIN = 0.004;
const FINE = /eye|nostril|nose|pad|teat|blaze|patch|speculum|mantle|tongue|beard|ring|cere|wingtip|forelock/;
const ALONG_U = [0.5, 0.5, 0.5, 0.5];

export type Fit = { zc?: number; zh?: number; zhK?: number; from?: string; ex?: number; fracs?: number[]; p?: number; zhAt?: (f: number) => number; end?: number; endStart?: number; vShift?: number; vScale?: number; bone?: string; maxDen?: number | null; mat?: string; tone?: number; region?: string; regions?: string[]; role?: string; ring?: number };
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

export class Author {
  readonly asset: Json;
  readonly f: Record<string, { o: V3; u: V3; v: V3; w: V3 }>;
  readonly st = restState;
  readonly solids: Json[] = [];
  readonly decisions: { en: string; de: string }[] = [];
  private readonly rest: ReturnType<typeof restState>;
  private readonly regionMaterial = new Map<string, string>();
  private readonly shows = new Map<string, number>();
  readonly slug: string;

  constructor(slug: string) {
    this.slug = slug;
    this.asset = loadAsset(resolveAsset(slug).path);
    this.f = frames(this.asset) as never;
    this.rest = restState(this.asset, 50);
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
    const shape = shapeId === undefined ? undefined : this.anyShape(shapeId);
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
    const fine = meta.role === "detail" || FINE.test(id);
    const den = extra.maxDen === undefined ? (fine ? this.denominatorOf(...shapeIds) : undefined) : extra.maxDen ?? undefined;
    if (den !== undefined) solid.maxDenominator = den;
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

  fit(id: string, shapeId: string, o0: Fit, extra: Extra = {}): Json {
    const l = this.lat(o0.from ?? shapeId);
    if (l === null && (o0.zh === undefined || o0.zc === undefined) && o0.zh === undefined) throw new Error(`${this.slug}: no lateral extent for "${shapeId}"`);
    const o: Fit = { ...o0, zc: o0.zc ?? (l === null ? 0 : (l[0] + l[1]) / 2), zh: (o0.zh ?? (l![1] - l![0]) / 2) * (o0.zhK ?? 1) };
    const e = this.elevation(shapeId);
    const bone = o.bone ?? e.shape.bone;
    const ring = e.rings[o.ring ?? 0]!;
    const fr = this.f[bone]!;
    const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
    const u0 = Math.min(...loc.map((p) => p[0])), u1 = Math.max(...loc.map((p) => p[0]));
    const fracs = o.fracs ?? [0, 0.03, 0.12, 0.28, 0.46, 0.64, 0.82, 0.94, 0.985, 1];
    const rows = fracs.map((fr_) => {
      const u = u0 + (u1 - u0) * fr_;
      const c = this.cut(ring, bone, Math.min(u1 - 1e-6, Math.max(u0 + 1e-6, u)));
      return { fr: fr_, u, c };
    });
    const half = (c: [number, number] | null) => (c === null ? 0 : (c[1] - c[0]) / 2);
    const ryMax = Math.max(...rows.map((x) => half(x.c)));
    const p = o.p ?? 0.5;
    const sections = rows.map((x, k) => {
      let c = x.c;
      let last = k === 0 || k === rows.length - 1;
      if (c === null || last) {
        const near = rows[k === 0 ? 1 : rows.length - 2]!.c!;
        c = [(near[0] + near[1]) / 2, (near[0] + near[1]) / 2];
      }
      const vc = ((c[0] + c[1]) / 2) * (o.vScale ?? 1) + (o.vShift ?? 0);
      let ry = Math.max(MIN, half(c) * (o.vScale ?? 1));
      let zh = o.zhAt === undefined ? o.zh! : o.zhAt(x.fr);
      let rz = Math.max(MIN, zh * Math.pow(Math.max(0.0001, half(c)) / ryMax, p));
      if (last) {
        const k0 = k === 0 ? (o.endStart ?? o.end ?? 0.1) : (o.end ?? 0.1);
        ry = Math.max(MIN, ry * 0 + ryMax * k0 * 0.25);
        rz = Math.max(MIN, zh * k0 * 0.25);
      }
      const s: Json = { at: rv([x.u, vc, (o.zc ?? 0) - fr.o[2]]), rx: r4(ry), ry: r4(rz), rotation: ALONG_U };
      if ((o.ex ?? 2) !== 2) s.exponent = o.ex;
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

  write(): string {
    const path = join(TICKET, "volumes", "animals", `${this.slug}.json`);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, JSON.stringify({ volume: { status: "authored", solids: this.solids, decisions: this.decisions } }, null, 1) + "\n");
    return path;
  }
}
