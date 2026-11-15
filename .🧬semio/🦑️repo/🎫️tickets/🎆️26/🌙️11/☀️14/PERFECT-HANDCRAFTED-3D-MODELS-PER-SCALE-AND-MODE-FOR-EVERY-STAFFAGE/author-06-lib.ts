/** 🐾️ Author 06 helpers: scale-wise lofts from the own 2D shapes of each scale (default pose), thermal shells and structural members for the animals.
 * @see ./author-brief.md */
import { Mask, distanceField, entryOf, featureM, loadSource, memberBones, meshMask, meshesAt, poseState, projectedPoints, round5, sourcePath, toLocal, toWorld, writeJson, type Json, type P2, type PoseState, type Ring, type V3, type View } from "./volumes-lib.ts";
import { shrink } from "./volumes-draft.ts";

const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const ALONG_U = [0.5, 0.5, 0.5, 0.5];

export type Hull = {
  bone?: string;
  ring?: number;
  plan?: string;
  front?: string;
  zc?: number;
  zh?: number;
  zhK?: number;
  p?: number;
  ex?: number;
  n?: number;
  mat?: string;
  tone?: number;
  regions?: string[];
  role?: string;
  vShift?: number;
  vScale?: number;
  t0?: number;
  t1?: number;
  endK?: number;
  zAt?: (t: number) => number;
  zhAt?: (t: number) => number;
  fw?: number;
};

export type Solid = Json;

export class Ctx {
  readonly asset: Json;
  readonly scale: number;
  readonly S: PoseState;
  readonly feature: number;
  readonly body: Solid[] = [];
  readonly thermal: Solid[] = [];
  readonly structure: Solid[] = [];
  readonly regionsOf = new Map<string, string[]>();
  readonly shapeOf = new Map<string, string>();
  readonly tubes = new Set<string>();
  readonly regionMat = new Map<string, string>();
  readonly regionById = new Map<string, Json>();

  constructor(readonly slug: string, scale: number) {
    const { asset } = entryOf(`animals/${slug}`);
    this.asset = asset;
    this.scale = scale;
    this.S = poseState(asset, scale);
    this.feature = featureM(scale, asset);
    for (const r of asset.thermal.regions) {
      this.regionById.set(r.id, r);
      if (r.material !== undefined) this.regionMat.set(r.id, r.material);
    }
  }

  get min(): number {
    return this.feature;
  }

  frame(bone: string) {
    const f = this.S.frames[bone];
    if (f === undefined) throw new Error(`${this.slug}: no bone ${bone}`);
    return f;
  }

  shape(view: "elevation" | "front" | "plan", id: string) {
    return this.S[view].find((e) => e.shape.id === id);
  }

  need(view: "elevation" | "front" | "plan", id: string) {
    const e = this.shape(view, id);
    if (e === undefined) throw new Error(`${this.slug} 1:${this.scale}: no ${view} shape ${id}`);
    return e;
  }

  local(bone: string, p: readonly number[]): V3 {
    return toLocal(this.frame(bone), p as V3).map(r4) as V3;
  }

  world(bone: string, p: readonly number[]): V3 {
    return toWorld(this.frame(bone), p as V3);
  }

  angle(bone: string): number {
    const u = this.frame(bone).u;
    return Math.atan2(u[1], u[0]);
  }

  private cutLocal(ring: Ring, bone: string, u: number): [number, number] | null {
    const fr = this.frame(bone);
    const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
    let lo = Infinity;
    let hi = -Infinity;
    for (let i = 0; i < loc.length; i++) {
      const a = loc[i]!;
      const b = loc[(i + 1) % loc.length]!;
      if ((a[0] - u) * (b[0] - u) <= 0 && a[0] !== b[0]) {
        const v = a[1] + ((u - a[0]) / (b[0] - a[0])) * (b[1] - a[1]);
        lo = Math.min(lo, v);
        hi = Math.max(hi, v);
      }
    }
    return lo === Infinity ? null : [lo, hi];
  }

  private cutX(ring: Ring, x: number): [number, number] | null {
    let lo = Infinity;
    let hi = -Infinity;
    for (let i = 0; i < ring.length; i++) {
      const a = ring[i]!;
      const b = ring[(i + 1) % ring.length]!;
      if ((a.x - x) * (b.x - x) <= 0 && a.x !== b.x) {
        const y = a.y + ((x - a.x) / (b.x - a.x)) * (b.y - a.y);
        lo = Math.min(lo, y);
        hi = Math.max(hi, y);
      }
    }
    return lo === Infinity ? null : [lo, hi];
  }

  lat(plan?: string, front?: string): [number, number] | null {
    let lo = Infinity;
    let hi = -Infinity;
    if (plan !== undefined) {
      const e = this.shape("plan", plan);
      if (e !== undefined && e.rings.length > 0) for (const p of e.rings.flat()) { lo = Math.min(lo, p.y); hi = Math.max(hi, p.y); }
    }
    if (front !== undefined) {
      const e = this.shape("front", front);
      if (e !== undefined && e.rings.length > 0) for (const p of e.rings.flat()) { lo = Math.min(lo, p.x); hi = Math.max(hi, p.x); }
    }
    return lo === Infinity ? null : [lo, hi];
  }

  meta(shapeId: string, o: { mat?: string; tone?: number; regions?: string[]; role?: string }, bone: string): Json {
    const sh = this.shape("elevation", shapeId)?.shape ?? this.shape("plan", shapeId)?.shape ?? this.shape("front", shapeId)?.shape;
    const regions: string[] = o.regions ?? sh?.regions ?? (sh?.region === undefined ? [] : [sh.region]);
    const material = o.mat ?? (regions.length === 0 ? undefined : this.regionMat.get(regions[0]!)) ?? this.fallbackMaterial(bone);
    return { material, role: o.role ?? (sh?.role === "detail" ? "detail" : "body"), tone: o.tone ?? sh?.tone ?? 0.5, regions };
  }

  fallbackMaterial(bone: string): string {
    for (const view of ["elevation", "plan", "front"] as const) for (const e of this.S[view]) if (e.shape.bone === bone) {
      const rs: string[] = e.shape.regions ?? (e.shape.region === undefined ? [] : [e.shape.region]);
      for (const r of rs) if (this.regionMat.get(r) !== undefined) return this.regionMat.get(r)!;
    }
    return this.asset.materials[0].id;
  }

  put(id: string, bone: string, kind: string, m: Json, geometry: Json, shapeId?: string): Solid {
    if (this.body.some((s) => s.id === id)) throw new Error(`${this.slug} 1:${this.scale}: duplicate ${id}`);
    const { regions, ...rest } = m;
    const solid: Solid = { id, kind, bone, ...rest, ...geometry };
    this.body.push(solid);
    this.regionsOf.set(id, regions);
    if (shapeId !== undefined) this.shapeOf.set(id, shapeId);
    return solid;
  }

  hull(id: string, shapeId: string, o: Hull = {}): Solid {
    const e = this.need("elevation", shapeId);
    const bone = o.bone ?? e.shape.bone;
    const fr = this.frame(bone);
    const ring = e.rings[o.ring ?? 0]!;
    const loc = ring.map((p) => toLocal(fr, [p.x, p.y, 0]));
    const u0 = Math.min(...loc.map((p) => p[0]));
    const u1 = Math.max(...loc.map((p) => p[0]));
    const planE = o.plan === undefined ? undefined : this.shape("plan", o.plan);
    const l = this.lat(o.plan ?? shapeId, o.front ?? shapeId);
    const planLat = this.lat(o.plan, undefined);
    const frontLat = o.front === undefined ? null : this.lat(undefined, o.front);
    const cutMode = planE !== undefined && Math.abs(fr.u[0]) > 0.55 && o.zh === undefined;
    const zc0 = o.zc ?? (l === null ? 0 : (l[0] + l[1]) / 2);
    const zh0 = (o.zh ?? (l === null ? 0.3 * (u1 - u0) / 2 : (l[1] - l[0]) / 2)) * (o.zhK ?? 1);
    const n = o.n ?? ({ 50: 18, 100: 13, 200: 9, 500: 7 } as Record<number, number>)[this.scale] ?? 12;
    const tt: number[] = [];
    const lo = o.t0 ?? 0;
    const hi = o.t1 ?? 1;
    for (let i = 0; i <= n; i++) {
      const x = i / n;
      const s = 0.5 - 0.5 * Math.cos(Math.PI * x);
      tt.push(lo + (hi - lo) * (0.35 * x + 0.65 * s));
    }
    const rows = tt.map((t) => {
      const u = u0 + (u1 - u0) * t;
      const uc = Math.min(u1 - 1e-5, Math.max(u0 + 1e-5, u));
      return { t, u, c: this.cutLocal(ring, bone, uc) };
    });
    const half = (c: [number, number] | null) => (c === null ? 0 : (c[1] - c[0]) / 2);
    const ryMax = Math.max(...rows.map((x) => half(x.c)), 1e-4);
    const p = o.p ?? 0.5;
    const MIN = Math.max(0.002, this.feature * 0.45);
    const sections = rows.map((x, k) => {
      const end = k === 0 || k === rows.length - 1;
      let c = x.c;
      if (c === null) {
        const near = rows[k === 0 ? 1 : rows.length - 2]!.c;
        c = near === null ? [0, 0] : [(near[0] + near[1]) / 2, (near[0] + near[1]) / 2];
      }
      let vc = ((c[0] + c[1]) / 2) * (o.vScale ?? 1) + (o.vShift ?? 0);
      let ry = Math.max(MIN, half(c) * (o.vScale ?? 1));
      let zc = o.zAt === undefined ? zc0 : o.zAt(x.t);
      let rz: number;
      if (cutMode) {
        const w = this.world(bone, [x.u, vc, 0]);
        const pc = this.cutX(planE!.rings[0]!, w[0]);
        if (pc === null) {
          rz = Math.max(MIN, ry * 0.5);
        } else {
          let a0 = pc[0], a1 = pc[1];
          if (planLat !== null && frontLat !== null && Math.abs((frontLat[1] - frontLat[0]) / (planLat[1] - planLat[0]) - 1) > 0.05) {
            const kk = (frontLat[1] - frontLat[0]) / (planLat[1] - planLat[0]);
            const w = o.fw ?? 1;
            a0 = a0 + w * (frontLat[0] + (a0 - planLat[0]) * kk - a0);
            a1 = a1 + w * (frontLat[0] + (a1 - planLat[0]) * kk - a1);
          }
          zc = (a0 + a1) / 2;
          rz = Math.max(MIN, ((a1 - a0) / 2) * (o.zhK ?? 1));
        }
      } else {
        const zh = o.zhAt === undefined ? zh0 : o.zhAt(x.t);
        rz = Math.max(MIN, zh * Math.pow(Math.max(1e-4, half(c)) / ryMax, p));
      }
      if (end) {
        const k0 = o.endK ?? 0.35;
        ry = Math.max(MIN, ry * k0);
        if (!cutMode) rz = Math.max(MIN, rz * k0);
      }
      const s: Json = { at: [r4(x.u), r4(vc), r4(zc - fr.o[2])], rx: r4(ry), ry: r4(rz), rotation: ALONG_U };
      if ((o.ex ?? 2) !== 2) s.exponent = o.ex;
      return s;
    });
    const need = this.feature * 1.08;
    const mv = Math.max(...sections.map((q) => q.rx)) * 2;
    const mz = Math.max(...sections.map((q) => q.ry)) * 2;
    const kv = mv < need ? need / mv : 1;
    const kz = mz < need ? need / mz : 1;
    if (kv > 1 || kz > 1) for (const q of sections) { q.rx = r4(q.rx * kv); q.ry = r4(q.ry * kz); }
    const m = this.meta(shapeId, o, bone);
    return this.put(id, bone, "loft", m, { sections, capStart: true, capEnd: true }, shapeId);
  }

  ell(id: string, bone: string, c: readonly number[], radii: readonly number[], angDeg = 0, o: { shape?: string; mat?: string; tone?: number; regions?: string[]; role?: string } = {}): Solid {
    const a = (angDeg * Math.PI) / 180 - this.angle(bone);
    const transform: Json = { translation: this.local(bone, c) };
    if (Math.abs(a) > 1e-6) transform.rotation = [0, 0, Math.sin(a / 2), Math.cos(a / 2)].map((x) => Math.round(x * 1e6) / 1e6);
    const m = this.meta(o.shape ?? "", o, bone);
    return this.put(id, bone, "ellipsoid", m, { radii: radii.map(r4), transform }, o.shape);
  }

  cap(id: string, bone: string, a: readonly number[], b: readonly number[], r0: number, r1: number, o: { shape?: string; mat?: string; tone?: number; regions?: string[]; role?: string } = {}): Solid {
    const m = this.meta(o.shape ?? "", o, bone);
    return this.put(id, bone, "capsule", m, { p0: this.local(bone, a), p1: this.local(bone, b), r0: r4(r0), r1: r4(r1) }, o.shape);
  }

  tube(id: string, bone: string, pts: { p: readonly number[]; rv: number; rz: number; ex?: number }[], o: { shape?: string; mat?: string; tone?: number; regions?: string[]; role?: string; capStart?: boolean; capEnd?: boolean } = {}): Solid {
    const loc = pts.map((q) => this.local(bone, q.p));
    const sections = pts.map((q, i) => {
      const a = loc[Math.max(0, i - 1)]!;
      const b = loc[Math.min(loc.length - 1, i + 1)]!;
      const d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
      const len = Math.hypot(d[0]!, d[1]!, d[2]!) || 1;
      const s: Json = { at: loc[i], rx: r4(Math.max(0.002, q.rv)), ry: r4(Math.max(0.002, q.rz)), rotation: basisQuat([d[0]! / len, d[1]! / len, d[2]! / len]) };
      if (q.ex !== undefined && q.ex !== 2) s.exponent = q.ex;
      return s;
    });
    const m = this.meta(o.shape ?? "", o, bone);
    this.tubes.add(id);
    return this.put(id, bone, "loft", m, { sections, capStart: o.capStart ?? true, capEnd: o.capEnd ?? true }, o.shape);
  }

  stroke(id: string, shapeId: string, o: { bone?: string; zc?: number; r?: number; n?: number; mat?: string; tone?: number; regions?: string[] } = {}): Solid {
    const e = this.need("elevation", shapeId);
    const bone = o.bone ?? e.shape.bone;
    const pts = e.rings[0]!;
    const pen = (e.shape.penMm ?? 0.2) * this.scale / 1000;
    const r = o.r ?? Math.max(pen / 2, this.feature * 0.58);
    const l = this.lat(shapeId, shapeId);
    const zc = o.zc ?? (l === null ? 0 : (l[0] + l[1]) / 2);
    const m = this.meta(shapeId, o, bone);
    return this.tube(id, bone, pts.map((q, i) => ({ p: [q.x, q.y, zc], rv: r * (i === pts.length - 1 ? 0.97 : 1), rz: r })), { ...m, shape: shapeId, regions: m.regions, mat: m.material, tone: m.tone, role: m.role });
  }

  strokeLocal(id: string, sh: Json): Solid {
    const bone = sh.bone as string;
    const pts: number[][] = sh.points;
    const pen = (sh.penMm ?? 0.2) * this.scale / 1000;
    const r = Math.max(pen / 2, this.feature * 0.58);
    const l = this.lat(sh.id, sh.id);
    const zc = l === null ? 0 : (l[0] + l[1]) / 2;
    const m = this.meta(sh.id, {}, bone);
    return this.tube(id, bone, pts.map((q, i) => ({ p: this.world(bone, [q[0]!, q[1]!, 0]).map((x, k) => (k === 2 ? zc : x)), rv: r * (i === pts.length - 1 ? 0.97 : 1), rz: r })), { shape: sh.id, regions: m.regions, mat: m.material, tone: m.tone, role: m.role });
  }

  boneTube(id: string, sh: Json): Solid {
    const bone = sh.bone as string;
    const b = (this.asset.rig.bones as Json[]).find((q) => q.id === bone)!;
    const L = (b.lengthM ?? 0.1) as number;
    const l = this.lat(sh.id, sh.id);
    const fr = this.shape("front", sh.id);
    const pl = this.shape("plan", sh.id);
    const half = (e: typeof fr, axis: "x" | "y") => (e === undefined ? undefined : (Math.max(...e.rings.flat().map((p) => p[axis])) - Math.min(...e.rings.flat().map((p) => p[axis]))) / 2);
    const hw = Math.max(half(fr, "x") ?? 0, half(pl, "y") ?? 0, this.feature * 0.6);
    const zc = l === null ? 0 : (l[0] + l[1]) / 2;
    const m = this.meta(sh.id, {}, bone);
    const p0 = this.world(bone, [0, 0, 0]);
    let p1 = this.world(bone, [L, 0, 0]);
    const drawn = new Set<string>();
    for (const v of ["elevation", "front", "plan"] as const) for (const e of this.S[v]) drawn.add(e.shape.bone);
    for (const ch of (this.asset.rig.bones as Json[]).filter((q) => q.parent === bone && !drawn.has(q.id) && /hoof|paw|foot|toe/.test(q.id))) p1 = this.world(ch.id, [ch.lengthM ?? 0, 0, 0]);
    return this.tube(id, bone, [{ p: [p0[0], p0[1], zc], rv: hw, rz: hw }, { p: [(p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2, zc], rv: hw * 0.92, rz: hw * 0.92 }, { p: [p1[0], p1[1], zc], rv: hw * 0.85, rz: hw * 0.85 }], { shape: sh.id, regions: m.regions, mat: m.material, tone: m.tone, role: m.role });
  }

  blob(id: string, sh: Json): Solid {
    const bone = sh.bone as string;
    const fr = this.shape("front", sh.id);
    const pl = this.shape("plan", sh.id);
    const el = this.shape("elevation", sh.id);
    const bb = (r: Ring[]) => {
      const pts = r.flat();
      return { x0: Math.min(...pts.map((p) => p.x)), x1: Math.max(...pts.map((p) => p.x)), y0: Math.min(...pts.map((p) => p.y)), y1: Math.max(...pts.map((p) => p.y)) };
    };
    let xc = 0, yc = 0, zc = 0, hx = this.feature, hy = this.feature, hz = this.feature;
    if (pl !== undefined) { const b = bb(pl.rings); xc = (b.x0 + b.x1) / 2; zc = (b.y0 + b.y1) / 2; hx = (b.x1 - b.x0) / 2; hz = (b.y1 - b.y0) / 2; }
    if (fr !== undefined) { const b = bb(fr.rings); yc = (b.y0 + b.y1) / 2; hy = (b.y1 - b.y0) / 2; if (pl === undefined) { zc = (b.x0 + b.x1) / 2; hz = (b.x1 - b.x0) / 2; } }
    if (el !== undefined) { const b = bb(el.rings); xc = (b.x0 + b.x1) / 2; yc = (b.y0 + b.y1) / 2; hx = (b.x1 - b.x0) / 2; hy = (b.y1 - b.y0) / 2; }
    const m = this.meta(sh.id, {}, bone);
    const mn = this.feature * 0.55;
    return this.put(id, bone, "ellipsoid", m, { radii: [r4(Math.max(mn, hx)), r4(Math.max(mn, hy)), r4(Math.max(mn, hz))], transform: { translation: this.local(bone, [xc, yc, zc]) } }, sh.id);
  }

  private fieldCache: Record<string, { field: Float32Array; mask: Mask }> | null = null;

  /** 🔍️ Distance (m) of a world point to the projected body in the three views (max over views). */
  outsideBy(p: readonly number[]): number {
    if (this.fieldCache === null) {
      const ms = meshesAt(this.asset, this.scale, this.body);
      const cache: Record<string, { field: Float32Array; mask: Mask }> = {};
      for (const view of ["elevation", "front", "plan"] as View[]) {
        const pts = projectedPoints(view, ms);
        const b = { minX: Math.min(...pts.map((q) => q.x)), maxX: Math.max(...pts.map((q) => q.x)), minY: Math.min(...pts.map((q) => q.y)), maxY: Math.max(...pts.map((q) => q.y)) };
        const cell = Math.max(this.feature / 3, Math.max(b.maxX - b.minX, b.maxY - b.minY) / 400);
        const box = { minX: b.minX - 0.1, minY: b.minY - 0.1, maxX: b.maxX + 0.1, maxY: b.maxY + 0.1 };
        const mask = meshMask(view, ms, box, cell);
        cache[view] = { field: distanceField(mask), mask };
      }
      this.fieldCache = cache;
    }
    let worst = 0;
    const proj: Record<string, P2> = { elevation: { x: p[0]!, y: p[1]! }, front: { x: p[2]!, y: p[1]! }, plan: { x: p[0]!, y: p[2]! } };
    for (const view of ["elevation", "front", "plan"]) {
      const { field, mask } = this.fieldCache![view]!;
      const q = proj[view]!;
      const i = Math.floor((q.x - mask.box.minX) / mask.cell);
      const j = Math.floor((q.y - mask.box.minY) / mask.cell);
      if (i < 0 || j < 0 || i >= mask.width || j >= mask.height) return 1;
      worst = Math.max(worst, field[j * mask.width + i]!);
    }
    return worst;
  }

  sectionAt(id: string, u: number): { vc: number; zc: number; rv: number; rz: number } | null {
    const s = this.body.find((q) => q.id === id);
    if (s === undefined || s.kind !== "loft") return null;
    const secs: Json[] = s.sections;
    if (u < secs[0].at[0] - 1e-9 || u > secs[secs.length - 1].at[0] + 1e-9) return null;
    let k = 0;
    while (k < secs.length - 2 && secs[k + 1].at[0] < u) k++;
    const a = secs[k];
    const b = secs[k + 1];
    const t = Math.min(1, Math.max(0, (u - a.at[0]) / (b.at[0] - a.at[0] || 1)));
    const mix = (x0: number, x1: number) => x0 + (x1 - x0) * t;
    return { vc: mix(a.at[1], b.at[1]), zc: mix(a.at[2], b.at[2]), rv: mix(a.rx, b.rx), rz: mix(a.ry, b.ry) };
  }

  bodyOfBone(bone: string): Solid[] {
    return this.body.filter((s) => s.bone === bone && s.kind === "loft" && s.role === "body");
  }

  extent(): { min: V3; max: V3 } {
    return { min: [0, 0, 0], max: [0, 0, 0] };
  }
}

function basisQuat(d: V3): number[] {
  const nl = Math.hypot(-d[1], d[0]) || 1;
  const n: V3 = Math.hypot(-d[1], d[0]) < 1e-9 ? [1, 0, 0] : [-d[1] / nl, d[0] / nl, 0];
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

export { basisQuat };

/** 🧅️ A solid shrunk inwards by `d`; loft radii are floored at 40 % of one feature so thin ends do not vanish. */
export function shellOf(s: Solid, d: number, feature: number): Solid | null {
  if (s.kind === "loft") {
    const c = JSON.parse(JSON.stringify(s));
    const floor = Math.max(0.0015, feature * 0.4);
    c.sections = c.sections.map((q: Json) => ({ ...q, rx: r4(Math.max(floor, q.rx - d)), ry: r4(Math.max(floor, q.ry - d)) }));
    const need = feature * 1.08;
    const mv = Math.max(...c.sections.map((q: Json) => q.rx)) * 2;
    const mz = Math.max(...c.sections.map((q: Json) => q.ry)) * 2;
    const kv = mv < need ? need / mv : 1;
    const kz = mz < need ? need / mz : 1;
    if (kv > 1 || kz > 1) for (const q of c.sections) { q.rx = r4(q.rx * kv); q.ry = r4(q.ry * kz); }
    return c;
  }
  const t = shrink(s, d);
  if (t !== null) return t;
  const c = JSON.parse(JSON.stringify(s));
  if (c.kind === "ellipsoid") c.radii = c.radii.map((x: number) => r4(Math.max(feature * 0.4, x - d)));
  else if (c.kind === "capsule") { c.r0 = r4(Math.max(feature * 0.4, c.r0 - d)); c.r1 = r4(Math.max(feature * 0.4, c.r1 - d)); }
  else return null;
  return c;
}

/** 🌡️ Thermal list: shells per layer at 1:50, references otherwise, plus hand solids. `extra` are real thermal solids (own geometry) with `region`. */
export function buildThermal(c: Ctx, extra: Solid[], skipRef: Set<string> = new Set()): Solid[] {
  const out: Solid[] = [];
  const covered = new Set<string>();
  for (const e of extra) covered.add(e.region);
  const regions: Json[] = c.asset.thermal.regions;
  for (const s of c.body) {
    if (s.role !== "body" || skipRef.has(s.id)) continue;
    const rs = (c.regionsOf.get(s.id) ?? []).filter((r) => c.regionById.has(r));
    if (rs.length === 0) continue;
    const dominant = [...rs].sort((a, b) => (c.regionById.get(b)!.areaM2 ?? 0) - (c.regionById.get(a)!.areaM2 ?? 0)).find((r) => !extra.some((x) => x.region === r && x.exclusive === true)) ?? rs[0]!;
    const region = c.regionById.get(dominant)!;
    const layers: Json[] = region.layers ?? [];
    const tag = `th-${s.id}`;
    if (c.scale === 50 && layers.length > 1) {
      let outside = 0;
      for (let i = layers.length - 1; i >= 0; i--) {
        const L = layers[i];
        if (i === layers.length - 1) out.push({ id: `${tag}-${L.id}`, of: s.id, region: dominant, layer: L.id });
        else {
          const sh = shellOf(s, outside, c.feature);
          if (sh !== null) out.push({ ...sh, id: `${tag}-${L.id}`, region: dominant, layer: L.id, tone: Math.max(0, (sh.tone ?? 0.5) - 0.1) });
          else throw new Error(`${c.slug}: cannot shell ${s.id}/${L.id}`);
        }
        outside += L.thicknessM ?? 0;
      }
    } else out.push({ id: tag, of: s.id, region: dominant, ...(layers.length > 0 ? { layer: layers[layers.length - 1].id } : {}) });
    void regions;
  }
  for (const e of extra) {
    const { exclusive: _x, ...rest } = e;
    out.push(rest);
  }
  void covered;
  return out;
}

export type StructCfg = {
  k?: Record<string, number>;
  kDefault?: number;
  joints?: "all" | "limbs" | "none";
  zOf?: Record<string, number>;
  skipMembers?: string[];
  spine?: string[];
  spineK?: [number, number, number];
  jointK?: number;
  jointMax?: number;
  sections?: number;
  minR?: number;
  skipJoints?: string[];
};

const sideTwin = (b: string): string | null => {
  const pairs: [string, string][] = [["-hn", "-hf"], ["-fn", "-ff"], ["-n", "-f"], ["-l", "-r"]];
  for (const [a, bb] of pairs) {
    if (b.endsWith(a)) return b.slice(0, -a.length) + bb;
    if (b.endsWith(bb)) return b.slice(0, -bb.length) + a;
  }
  return null;
};

const zFallback = (c: Ctx, id: string, cfg: StructCfg): number => {
  if (cfg.zOf?.[id] !== undefined) return cfg.zOf[id]!;
  const fine = poseState(c.asset, 50);
  const zs = fine.plan.filter((e) => e.shape.bone === id && e.rings.length > 0).flatMap((e) => e.rings.flat().map((q) => q.y));
  if (zs.length > 0) return (Math.min(...zs) + Math.max(...zs)) / 2;
  const t = sideTwin(id);
  if (t !== null) {
    const zt = fine.plan.filter((e) => e.shape.bone === t && e.rings.length > 0).flatMap((e) => e.rings.flat().map((q) => q.y));
    if (zt.length > 0) return -(Math.min(...zt) + Math.max(...zt)) / 2;
  }
  return 0;
};

/** 🏗️ Structure list: shaft lofts along the body centre lines, joints, mass solids. */
export function buildStructure(c: Ctx, cfg: StructCfg = {}): Solid[] {
  const out: Solid[] = [];
  const bones: Json[] = c.asset.rig.bones;
  const byId = new Map(bones.map((b) => [b.id, b]));
  const members = memberBones(c.asset, c.scale).filter((b) => !(cfg.skipMembers ?? []).includes(b));
  const radius = new Map<string, number>();
  const feat = c.feature;
  const H = c.asset.dimensions.heightM as number;
  const spine = new Set(cfg.spine ?? []);
  const material = (b: Json): string => b.section?.material ?? c.fallbackMaterial(b.id);
  const nsec = cfg.sections ?? (c.scale <= 50 ? 5 : c.scale <= 100 ? 4 : c.scale <= 200 ? 3 : 2);
  for (const id of members) {
    const b = byId.get(id);
    if (b === undefined) continue;
    const L = Math.max(b.lengthM ?? 0.05, feat);
    const own = c.bodyOfBone(id);
    const twin = sideTwin(id);
    const twinBody = twin === null ? [] : c.bodyOfBone(twin);
    let zUse = 0;
    const sample = (u: number): { vc: number; zc: number; rv: number; rz: number } => {
      let best: { vc: number; zc: number; rv: number; rz: number } | null = null;
      for (const s of own) {
        const v = c.sectionAt(s.id, u);
        if (v !== null && (best === null || Math.min(v.rv, v.rz) > Math.min(best.rv, best.rz))) best = v;
      }
      if (best !== null) return best;
      for (const s of twinBody) {
        const v = c.sectionAt(s.id, u);
        if (v !== null) return { vc: v.vc, zc: -v.zc, rv: v.rv, rz: v.rz };
      }
      const rr = Math.min(0.045 * H, Math.max(feat, Math.sqrt((b.mass?.kg ?? 0.1) / (900 * Math.PI * L)) * 1.6));
      return { vc: 0, zc: zUse, rv: rr / 0.55, rz: rr / 0.55 };
    };
    const tubeBody = own.find((q) => c.tubes.has(q.id));
    if (tubeBody !== undefined) {
      const secs2: Json[] = tubeBody.sections.map((q: Json) => ({ ...q, rx: r4(Math.max(cfg.minR ?? feat * 0.6, q.rx * 0.9)), ry: r4(Math.max(cfg.minR ?? feat * 0.6, q.ry * 0.9)) }));
      radius.set(id, Math.max(...secs2.map((q) => Math.max(q.rx, q.ry))));
      out.push({ id: `member-${id}`, kind: "loft", bone: id, material: material(b), role: "body", tone: 0.55, member: id, part: "member", sections: secs2, capStart: true, capEnd: true });
      continue;
    }
    const free = own.length === 0 && twinBody.length === 0;
    let uA = 0;
    let uB = L;
    let off: V3 | null = null;
    zUse = zFallback(c, id, cfg);
    if (free) {
      const N = 40;
      let top: { a: number; b: number; z: number; d: number } | null = null;
      for (const zc of [zUse, zUse * 0.5, 0]) {
        let best: [number, number] | null = null;
        let run: number | null = null;
        let bu = 0, bd = Infinity;
        for (let i = 0; i <= N; i++) {
          const u = (L * i) / N;
          const w = c.world(id, [u, 0, 0]);
          const R = feat * 0.6;
          const d = Math.max(c.outsideBy([w[0], w[1], zc]), c.outsideBy([w[0] + R, w[1], zc]), c.outsideBy([w[0] - R, w[1], zc]), c.outsideBy([w[0], w[1] + R, zc]), c.outsideBy([w[0], w[1] - R, zc]), c.outsideBy([w[0], w[1], zc + R]), c.outsideBy([w[0], w[1], zc - R]));
          if (d < bd) { bd = d; bu = u; }
          const ok = d <= feat * 0.2;
          if (ok && run === null) run = u;
          if ((!ok || i === N) && run !== null) {
            const end = ok ? u : (L * (i - 1)) / N;
            if (best === null || end - run > best[1] - best[0]) best = [run, end];
            run = null;
          }
        }
        const cand = best !== null ? { a: best[0], b: best[1], z: zc, d: 0 } : { a: Math.max(0, bu - feat), b: Math.min(L, bu + feat), z: zc, d: bd };
        if (top === null || (cand.d === 0 && (top.d > 0 || cand.b - cand.a > top.b - top.a)) || (cand.d > 0 && top.d > 0 && cand.d < top.d)) top = cand;
        if (top.d === 0 && top.b - top.a > 0.6 * L) break;
      }
      uA = top!.a; uB = top!.b; zUse = top!.z;
      if (top!.d > feat * 0.3) {
        let anc = b.parent as string | null;
        while (anc !== null && c.bodyOfBone(anc).length === 0) anc = (byId.get(anc)?.parent as string | null) ?? null;
        const pb = anc === null ? undefined : c.bodyOfBone(anc)[0];
        if (pb !== undefined) {
          const secsP: Json[] = pb.sections;
          const mid = secsP[Math.floor(secsP.length / 2)];
          const wp = c.world(anc!, [mid.at[0], mid.at[1], mid.at[2]]);
          const lp = c.local(id, wp);
          const len = Math.min(L, 3 * feat);
          off = [lp[0], lp[1], lp[2]];
          uA = -len / 2; uB = len / 2;
        }
      }
      if (uB - uA < 1.5 * feat) { const m = (uA + uB) / 2; uA = m - feat * 0.8; uB = m + feat * 0.8; }
    }
    if (!(off !== null) && uB - uA < 1.15 * feat) { const m = (uA + uB) / 2; uA = m - 0.6 * feat; uB = m + 0.6 * feat; }
    const k = cfg.k?.[id] ?? cfg.kDefault ?? 0.5;
    const isSpine = spine.has(id);
    const secs: Json[] = [];
    for (let i = 0; i < nsec; i++) {
      const f = nsec === 1 ? 0.5 : i / (nsec - 1);
      const u = uA + (uB - uA) * f;
      const s = sample(Math.min(L, Math.max(0, u)));
      const ends = i === 0 || i === nsec - 1;
      const prof = ends ? 1.0 : 0.82;
      let rv: number, rz: number, vc = s.vc;
      if (isSpine) {
        const [kv, kz, off] = cfg.spineK ?? [0.2, 0.3, 0.45];
        rv = s.rv * kv;
        rz = s.rz * kz;
        vc = s.vc + s.rv * off;
      } else {
        rv = Math.min(s.rv, s.rz) * k * prof;
        rz = rv;
      }
      const mr = cfg.minR ?? feat * 0.6;
      void ends;
      secs.push({ at: off === null ? [r4(u), r4(vc), r4(s.zc)] : [r4(off[0] + u), r4(off[1]), r4(off[2])], rx: r4(Math.max(mr, rv)), ry: r4(Math.max(mr, rz)), rotation: ALONG_U });
    }
    radius.set(id, Math.max(...secs.map((q) => Math.max(q.rx, q.ry))));
    out.push({ id: `member-${id}`, kind: "loft", bone: id, material: material(b), role: "body", tone: 0.55, member: id, part: "member", sections: secs, capStart: true, capEnd: true });
  }
  const jm = cfg.joints ?? (c.scale <= 50 ? "all" : c.scale <= 100 ? "limbs" : "none");
  if (jm !== "none")
    for (const b of bones) {
      if (b.parent === null || !radius.has(b.id) || !radius.has(b.parent)) continue;
      if (spine.has(b.id) && spine.has(b.parent)) continue;
      if ((cfg.skipJoints ?? []).includes(b.id)) continue;
      if (jm === "limbs" && !/thigh|shank|humerus|forearm|scapula|metatarsus|metacarpus|knee|elbow|tarsus|wing|arm|leg|hand|foot|neck/.test(b.id)) continue;
      const own = out.find((q) => q.id === `member-${b.id}`)!;
      const rOwn = Math.max(own.sections[0].rx, own.sections[0].ry);
      const rPar = spine.has(b.parent) ? rOwn : radius.get(b.parent)!;
      const r = Math.min(cfg.jointMax ?? 0.05 * H, Math.max(feat * 0.6, (cfg.jointK ?? 1.2) * Math.max(rOwn, rPar)));
      const jw = c.world(b.id, own.sections[0].at);
      const bad = [[0, 0, 0], [r, 0, 0], [-r, 0, 0], [0, r, 0], [0, -r, 0], [0, 0, r], [0, 0, -r]].some((d) => c.outsideBy([jw[0] + d[0]!, jw[1] + d[1]!, jw[2] + d[2]!]) > feat * 0.1);
      if (bad) continue;
      out.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: material(b), role: "detail", tone: 0.35, member: b.id, part: "joint", radii: [r4(r), r4(r), r4(r)], transform: { translation: [own.sections[0].at[0], own.sections[0].at[1], own.sections[0].at[2]] } });
    }
  const total = bones.reduce((n, b) => n + (b.mass?.kg ?? 0), 0);
  for (const b of bones) {
    const kg = b.mass?.kg ?? 0;
    if (kg < 0.1 * total) continue;
    const L = b.lengthM ?? 0.1;
    const cx = Math.min(L, Math.max(0, b.mass.comX ?? L / 2));
    let vc = 0, zc = 0, rv = feat, rz = feat;
    const bodies = c.bodyOfBone(b.id);
    let found = false;
    for (const s of bodies) {
      const v = c.sectionAt(s.id, cx);
      if (v !== null) { vc = v.vc; zc = v.zc; rv = v.rv; rz = v.rz; found = true; break; }
    }
    const own = out.find((q) => q.id === `member-${b.id}`);
    if (!found && own !== undefined) { vc = own.sections[Math.floor(own.sections.length / 2)].at[1]; zc = own.sections[Math.floor(own.sections.length / 2)].at[2]; rv = own.sections[0].rx / 0.5; rz = rv; }
    const r = Math.cbrt((3 * kg) / (4 * Math.PI * 1000));
    const rmax = Math.min(rv, rz) * 0.65;
    const rr = Math.max(feat * 0.6, Math.min(r, rmax));
    out.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: material(b), role: "detail", tone: 0.15, member: b.id, part: "mass", radii: [r4(rr), r4(rr), r4(rr)], transform: { translation: [r4(cx), r4(vc), r4(zc)] } });
  }
  for (const q of out) {
    if (q.part !== "member" && q.part !== "mass") continue;
    for (let it = 0; it < 4; it++) {
      let worst = 0;
      const secs: Json[] = q.sections ?? [];
      for (const sec of secs) {
        const w = c.world(q.bone, sec.at);
        const rr = Math.max(sec.rx, sec.ry);
        for (const d of [[0, 0, 0], [rr, 0, 0], [-rr, 0, 0], [0, rr, 0], [0, -rr, 0], [0, 0, rr], [0, 0, -rr]]) worst = Math.max(worst, c.outsideBy([w[0] + d[0]!, w[1] + d[1]!, w[2] + d[2]!]));
      }
      if (worst <= feat * 0.1 || secs.length === 0) break;
      const mr = cfg.minR ?? feat * 0.6;
      for (const sec of secs) { sec.rx = r4(Math.max(mr, sec.rx * 0.75)); sec.ry = r4(Math.max(mr, sec.ry * 0.75)); }
    }
  }
  return out;
}

/** 🩹️ Regions without any thermal solid: a capsule along each bone that carries the region at 1:50, sized from the 1:50 drawing. */
export function fillMissing(c: Ctx, thermal: Solid[], only?: string[]): string[] {
  const have = new Set(thermal.map((s) => s.region));
  const added: string[] = [];
  const fine = poseState(c.asset, 50);
  for (const r of c.asset.thermal.regions as Json[]) {
    if (have.has(r.id) || (only !== undefined && !only.includes(r.id))) continue;
    const bones = new Set<string>();
    for (const v of ["elevation"] as const) for (const e of fine[v]) {
      const rs: string[] = e.shape.regions ?? (e.shape.region === undefined ? [] : [e.shape.region]);
      if (rs.includes(r.id) && e.shape.role === "body") bones.add(e.shape.bone);
    }
    for (const bone of (c.scale >= 200 ? [...bones].slice(0, 1) : [...bones])) {
      const f50 = fine.frames[bone]!;
      const shapes = fine.elevation.filter((e) => e.shape.bone === bone && e.shape.role === "body" && e.rings.length > 0);
      const loc = shapes.flatMap((e) => e.rings.flat().map((p) => toLocal(f50, [p.x, p.y, 0])));
      if (loc.length === 0) continue;
      const u0 = Math.min(...loc.map((p) => p[0])), u1 = Math.max(...loc.map((p) => p[0]));
      const v0 = Math.min(...loc.map((p) => p[1])), v1 = Math.max(...loc.map((p) => p[1]));
      const zs = fine.plan.filter((e) => e.shape.bone === bone && e.rings.length > 0).flatMap((e) => e.rings.flat().map((p) => p.y));
      const zc = zs.length === 0 ? 0 : (Math.min(...zs) + Math.max(...zs)) / 2;
      const zh = zs.length === 0 ? (v1 - v0) / 2 : (Math.max(...zs) - Math.min(...zs)) / 2;
      const rv = Math.max(c.feature * 0.68, (v1 - v0) / 2);
      const rz = Math.max(c.feature * 0.68, zh);
      const vm = (v0 + v1) / 2;
      const sec = (u: number, k: number): Json => ({ at: [r4(u), r4(vm), r4(zc - f50.o[2])], rx: r4(rv * k), ry: r4(rz * k), rotation: ALONG_U });
      const mat = c.regionMat.get(r.id) ?? c.fallbackMaterial(bone);
      thermal.push({ id: `th-${r.id}-${bone}`, kind: "loft", bone, material: mat, role: "body", tone: 0.5, region: r.id, ...(r.layers?.length > 0 ? { layer: r.layers[r.layers.length - 1].id } : {}), sections: [sec(u0, 0.85), sec(u0 + (u1 - u0) * 0.15, 1), sec(u0 + (u1 - u0) * 0.85, 1), sec(u1, 0.85)], capStart: true, capEnd: true });
    }
    if (bones.size === 0) {
      for (const e of fine.elevation) {
        const rs: string[] = e.shape.regions ?? (e.shape.region === undefined ? [] : [e.shape.region]);
        if (!rs.includes(r.id) || e.rings.length === 0) continue;
        const pts = e.rings.flat();
        const x0 = Math.min(...pts.map((q) => q.x)), x1 = Math.max(...pts.map((q) => q.x));
        const y0 = Math.min(...pts.map((q) => q.y)), y1 = Math.max(...pts.map((q) => q.y));
        const pl = fine.plan.find((q) => q.shape.id === e.shape.id && q.rings.length > 0);
        const zs = pl === undefined ? [] : pl.rings.flat().map((q) => q.y);
        const zc = zs.length === 0 ? 0 : (Math.min(...zs) + Math.max(...zs)) / 2;
        const rr = Math.max(c.feature * 0.62, (x1 - x0) / 2, (y1 - y0) / 2);
        const bone = e.shape.bone as string;
        const mat = c.regionMat.get(r.id) ?? c.fallbackMaterial(bone);
        thermal.push({ id: `th-${r.id}-${bone}`, kind: "ellipsoid", bone, material: mat, role: "body", tone: 0.1, region: r.id, ...(r.layers?.length > 0 ? { layer: r.layers[r.layers.length - 1].id } : {}), radii: [r4(rr), r4(rr), r4(rr)], transform: { translation: c.local(bone, [(x0 + x1) / 2, (y0 + y1) / 2, zc]) } });
        added.push(r.id);
        break;
      }
    }
    if (bones.size > 0) added.push(r.id);
  }
  return added;
}

export type Auto = { skip?: string[]; over?: Record<string, Hull & { id?: string }>; dflt?: Hull; n?: number };

/** 🧩️ One loft per closed body shape of the scale; lateral extent from the plan (else front) shape of the same id. */
export function auto(c: Ctx, cfg: Auto = {}): void {
  for (const e of c.S.elevation) {
    const sh = e.shape;
    if (e.rings.length === 0 || (sh.role !== "body") || (cfg.skip ?? []).includes(sh.id)) continue;
    const o = cfg.over?.[sh.id] ?? {};
    if (!e.closed) {
      c.stroke(o.id ?? sh.id, sh.id, { zc: o.zc, bone: o.bone });
      continue;
    }
    const plan = o.plan ?? (c.shape("plan", sh.id) !== undefined ? sh.id : undefined);
    const front = o.front ?? (c.shape("front", sh.id) !== undefined ? sh.id : undefined);
    c.hull(o.id ?? sh.id, sh.id, { n: cfg.n, ...(cfg.dflt ?? {}), plan, front, ...o });
  }
  const have = new Set(c.body.map((q) => q.bone));
  for (const view of ["front", "plan"] as const) for (const e of c.S[view]) {
    const sh = e.shape;
    if (sh.role !== "body" || have.has(sh.bone) || (cfg.skip ?? []).includes(sh.id) || e.rings.length === 0) continue;
    const o = cfg.over?.[sh.id] ?? {};
    if (sh.kind === "ellipse") c.blob(o.id ?? sh.id, sh);
    else c.boneTube(o.id ?? sh.id, sh);
    have.add(sh.bone);
  }
  const drawn = new Set<string>();
  for (const v of ["elevation", "front", "plan"] as const) for (const e of c.S[v]) if (["body", "detail", "glazing"].includes(e.shape.role) && e.rings.length > 0) drawn.add(e.shape.bone);
  for (const bone of drawn) if (!have.has(bone)) {
    const b = (c.asset.rig.bones as Json[]).find((q) => q.id === bone)!;
    const L = b.lengthM ?? 0.1;
    const fine = poseState(c.asset, 50);
    const zs = fine.plan.filter((e) => e.shape.bone === bone && e.rings.length > 0).flatMap((e) => e.rings.flat().map((q) => q.y));
    const zc = zs.length > 0 ? (Math.min(...zs) + Math.max(...zs)) / 2 : 0;
    const rr = Math.max(c.feature * 0.6, Math.min(0.12 * L + 0.01, 0.06));
    const p0 = c.world(bone, [0, 0, 0]);
    const p1 = c.world(bone, [L, 0, 0]);
    const m = c.meta("", {}, bone);
    c.tube(`${bone}-core`, bone, [{ p: [p0[0], p0[1], zc], rv: rr, rz: rr }, { p: [(p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2, zc], rv: rr * 1.1, rz: rr * 1.1 }, { p: [p1[0], p1[1], zc], rv: rr, rz: rr }], { regions: m.regions, mat: m.material, tone: m.tone, role: "body" });
  }
}

/** 💨️ Thermal solids of the emitters: nozzle capsules for breath, a flat ellipsoid above the back for plumes. */
export function emitters(c: Ctx, regionOf: Record<string, string>, fixY: Record<string, number> = {}): Solid[] {
  const out: Solid[] = [];
  const f = c.feature;
  const box = bodyBox(c);
  for (const em of c.asset.thermal.emitters ?? []) {
    const a = c.asset.anchors.find((x: Json) => x.id === em.anchor);
    if (a === undefined) throw new Error(`${c.slug}: anchor ${em.anchor}`);
    const region = regionOf[em.id] ?? em.region;
    if (region === undefined) throw new Error(`${c.slug}: no region for emitter ${em.id}`);
    const rmat = c.regionMat.get(region) ?? c.asset.materials[0].id;
    const w = c.world(a.bone, [a.x, a.y, 0]);
    if (fixY[em.id] !== undefined) w[1] = fixY[em.id]!;
    const d = em.diameterM as number;
    if (em.kind === "plume") {
      const h = Math.max(f * 0.6, 0.012);
      const rx = Math.max(f * 0.6, d / 2);
      const hz = Math.min(rx, Math.max(f, (box.max[2] - box.min[2]) / 2 - f));
      const top = bodyBox(c).max[1] - h;
      const lc = c.local(a.bone, [w[0], Math.min(w[1] + h * 0.55, top), 0]);
      out.push({ id: `th-${em.id}`, tags: [em.id], kind: "ellipsoid", bone: a.bone, material: rmat, role: "detail", tone: 0.8, region, radii: [r4(rx), r4(h), r4(hz)], transform: { translation: lc, rotation: rotZ(-c.angle(a.bone)) } });
    } else {
      const ang = ((em.directionDeg ?? 0) * Math.PI) / 180;
      const len = Math.max(f * 1.2, d * 1.4);
      const r = Math.max(f * 0.55, d / 2);
      const p1: V3 = [w[0], w[1], 0];
      const pStart: V3 = [w[0] - Math.cos(ang) * len, w[1] - Math.sin(ang) * len, 0];
      out.push({ id: `th-${em.id}`, tags: [em.id], kind: "capsule", bone: a.bone, material: rmat, role: "detail", tone: 0.7, region, p0: c.local(a.bone, pStart), p1: c.local(a.bone, p1), r0: r4(r), r1: r4(r * 0.8) });
    }
  }
  return out;
}

const rotZ = (a: number): number[] => [0, 0, Math.sin(a / 2), Math.cos(a / 2)].map((x) => Math.round(x * 1e6) / 1e6);

export function bodyBox(c: Ctx): { min: V3; max: V3 } {
  const d = c.asset.dimensions;
  return { min: [0, 0, -d.depthM / 2], max: [d.widthM, d.heightM, d.depthM / 2] };
}


export type Note = { en: string; de: string };

/** 📝️ The bilingual decisions of a scale: the author's notes on the body plus derived sentences on thermal and structure. */
export function decisions(c: Ctx, body: Note[], thermal: Solid[], structure: Solid[], extra: { thermal?: Note; structure?: Note } = {}): Note[] {
  const n = c.scale;
  const f = Math.round(c.feature * 1000 * 10) / 10;
  const layered = thermal.filter((t) => t.layer !== undefined && !t.of).length;
  const regions = new Set(thermal.map((t) => t.region)).size;
  const refs = thermal.filter((t) => t.of !== undefined).length;
  const own = thermal.length - refs;
  const members = structure.filter((q) => q.part === "member").length;
  const joints = structure.filter((q) => q.part === "joint").length;
  const masses = structure.filter((q) => q.part === "mass").length;
  const emit = (c.asset.thermal.emitters ?? []).map((e: Json) => e.id).join(", ");
  const nested = n === 50 && layered > 0 ? { en: ` Multi-layer regions are nested shells, innermost layer first, each shrunk inwards by the real thickness of the layers outside it (${layered} inner shells).`, de: ` Mehrschichtige Regionen sind ineinander geschachtelte Schalen, innerste Schicht zuerst, jede um die echte Dicke der aussen liegenden Schichten nach innen verkleinert (${layered} innere Schalen).` } : { en: " Regions have one layer or are drawn merged at this scale, so each is a single surface solid.", de: " Die Regionen haben nur eine Schicht oder sind in diesem Massstab verschmolzen gezeichnet, daher ist jede ein einzelner Oberflächenkörper." };
  const jtxt = joints > 0 ? { en: `${joints} ball joints sit at the rig joints on the member axes`, de: `${joints} Kugelgelenke sitzen an den Rig-Gelenken auf den Stabachsen` } : { en: "joints are merged into the rounded member ends at this scale", de: "Gelenke sind in diesem Massstab in die gerundeten Stabenden eingegangen" };
  const out: Note[] = [
    ...body,
    {
      en: `Thermal: ${thermal.length} solids cover all ${regions} regions (${refs} reuse a body solid through a reference, ${own} are own solids). Emitters (${emit}) are small nozzle and plume solids tagged with their id.${nested.en}${extra.thermal ? " " + extra.thermal.en : ""} Smallest feature at 1:${n} is ${f} mm.`,
      de: `Thermik: ${thermal.length} Körper decken alle ${regions} Regionen ab (${refs} verwenden einen Körperkörper über eine Referenz, ${own} sind eigene Körper). Emitter (${emit}) sind kleine Düsen- und Fahnenkörper, die mit ihrer Kennung markiert sind.${nested.de}${extra.thermal ? " " + extra.thermal.de : ""} Das kleinste Merkmal bei 1:${n} beträgt ${f} mm.`,
    },
    {
      en: `Structure: ${members} member solids follow the load-bearing bones of the rig (limb shafts along the centre lines of the body lofts, the spine as a slender beam along the back of the trunk); ${jtxt.en}; ${masses} compact mass solids sit at the centres of mass of the heavy segments.${extra.structure ? " " + extra.structure.en : ""} The animal has no support entries; its ground contacts are the hoof or foot anchors.`,
      de: `Tragwerk: ${members} Stabkörper folgen den tragenden Knochen des Rigs (Gliedmassenschäfte entlang der Mittellinien der Körperlofts, die Wirbelsäule als schlanker Balken am Rücken des Rumpfes); ${jtxt.de}; ${masses} kompakte Massekörper sitzen in den Schwerpunkten der schweren Segmente.${extra.structure ? " " + extra.structure.de : ""} Das Tier hat keine Auflager-Einträge; seine Bodenkontakte sind die Huf- oder Fussanker.`,
    },
  ];
  return out;
}

export type ScaleOut = { body: Solid[]; thermal: Solid[]; structure: Solid[]; decisions: { en: string; de: string }[] };

export function finish(slug: string, scales: Record<number, ScaleOut>): void {
  const { entry } = entryOf(`animals/${slug}`);
  const source = loadSource(entry)!;
  for (const [k, v] of Object.entries(scales)) {
    const cur = source.volumes[k];
    source.volumes[k] = { status: "authored", denominator: Number(k), body: strip(v.body), thermal: v.thermal, structure: v.structure, decisions: v.decisions };
    void cur;
  }
  writeJson(sourcePath(entry), source, true);
}

const strip = (list: Solid[]): Solid[] => list;

export { round5, r4, r5 };
export type { P2 };
