/** 🌿️ Author 01 kit: bone-local conversion of world rest coordinates, solid builders with explicit data, silhouette slices of the authored 2D shapes of ONE scale, and the three lists (body, thermal, structure) of that scale.
 * Everything a script emits is plain data; `Kit.save` writes `{ volumes }` for `volumes-insert.py`.
 * @see ./design.md */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { TICKET, bboxOf, frames, loadAsset, meshes, resolveAsset, restState, toLocal, dirToLocal, extentOf, type Json, type P2, type V3 } from "./volumes-lib.ts";
import { memberBones, meshArea, viewMasks, meshMask, distanceField, toWorld, VIEWS } from "./volumes-lib.ts";

export type { V3, Json };
export type Meta = { material: string; role: string; tone: number };
export type ListKey = "body" | "thermal" | "structure";
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
const rnd = (v: any): any => (typeof v === "number" ? r5(v) : Array.isArray(v) ? v.map(rnd) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, rnd(x)])) : v);

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
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V3): V3 => { const n = Math.hypot(...a) || 1; return [a[0] / n, a[1] / n, a[2] / n]; };
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V3, b: V3) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

export type Section = { c: V3; rx: number; ry: number; exp?: number };


export type Section = { c: V3; rx: number; ry: number; exp?: number };

export class Author {
  readonly asset: Json;
  readonly fr: ReturnType<typeof frames>;
  readonly st: ReturnType<typeof restState>;
  readonly lists: Record<ListKey, Json[]> = { body: [], thermal: [], structure: [] };
  readonly ends = new Map<string, { o: V3; t: V3 }>();
  readonly decisions: { en: string; de: string }[] = [];
  readonly slug: string;
  readonly scale: number;
  target: ListKey = "body";
  defaults: Json = {};
  private readonly rawShapes = new Map<string, Json>();
  constructor(slug: string, scale: number) {
    this.slug = slug;
    this.scale = scale;
    this.asset = loadAsset(resolveAsset(`vegetation/${slug}`).path);
    this.fr = frames(this.asset);
    this.st = restState(this.asset, scale);
    const g = this.asset.representations[String(scale)].geometry;
    for (const view of ["elevation", "front", "plan"]) for (const s of g[view]) if (!this.rawShapes.has(s.id)) this.rawShapes.set(s.id, s);
  }
  get solids(): Json[] { return this.lists[this.target]; }
  into(list: ListKey, defaults: Json = {}) { this.target = list; this.defaults = defaults; return this; }
  get feature() { return (0.15 * this.scale) / 1000; }
  L(bone: string, p: V3): V3 { return toLocal(this.fr[bone]!, p); }
  D(bone: string, d: V3): V3 { return dirToLocal(this.fr[bone]!, d); }
  has(id: string) { return this.rawShapes.has(id); }
  meta(id: string, over: Partial<Meta & { region: string }> = {}): Meta {
    const s = this.rawShapes.get(id);
    const regionId = (over as Json).region ?? s?.region;
    const region = this.asset.thermal.regions.find((r: Json) => r.id === regionId);
    const material = over.material ?? s?.material ?? region?.material;
    if (material === undefined) throw new Error(`no material for ${id}`);
    return { material, role: over.role ?? s?.role ?? "body", tone: over.tone ?? s?.tone ?? 0.5 };
  }
  mat(id: string, tone = 0.5, role = "body"): Meta { return { material: id, role, tone }; }
  private push(id: string, bone: string, m: Meta, geometry: Json, extra: Json = {}) {
    if (this.solids.some((s) => s.id === id)) throw new Error(`duplicate solid id ${id} in ${this.target}`);
    const solid = { id, kind: geometry.kind, bone, ...m, ...geometry, ...this.defaults, ...extra };
    this.solids.push(rnd(solid));
  }
  /** 🔗️ a thermal reference to a body solid of this scale. */
  ref(id: string, of: string, region: string, layer?: string) {
    if (!this.lists.body.some((s) => s.id === of)) throw new Error(`no body solid ${of}`);
    this.lists.thermal.push({ id, of, region, ...(layer ? { layer } : {}) });
  }
  sref(id: string, of: string, member: string, part: string) {
    if (!this.lists.body.some((s) => s.id === of)) throw new Error(`no body solid ${of}`);
    this.lists.structure.push({ id, of, member, part });
  }
  /** 📐️ surface area in m2 of the solids of a list (references resolved), or of the ones matching `pick`. */
  areaOf(list: ListKey, pick: (s: Json) => boolean = () => true): number {
    const bodyById = new Map(this.lists.body.map((s) => [s.id, s]));
    const solids = this.lists[list].filter(pick).map((s) => {
      if (s.of === undefined) return s;
      const { id: _i, of: _o, ...tags } = s;
      return { ...bodyById.get(s.of)!, ...tags, id: s.id };
    });
    const ms = meshes(this.asset, solids, 20);
    return ms.reduce((n, m) => n + meshArea(m), 0);
  }
  /** 💊️ capsule between two world points (a tapered limb, a culm, a rib). */
  cap(id: string, bone: string, a: V3, b: V3, r0: number, r1: number, m: Meta, extra: Json = {}) {
    this.push(id, bone, m, { kind: "capsule", p0: this.L(bone, a), p1: this.L(bone, b), r0, r1 }, extra);
  }
  /** 🥚️ ellipsoid centred on a world point with radii along world x, y, z, turned about world z by `tiltDeg` (and about world y by `yawDeg`). */
  ell(id: string, bone: string, c: V3, radii: V3, m: Meta, o: { tiltDeg?: number; yawDeg?: number; extra?: Json } = {}) {
    const t = ((o.tiltDeg ?? 0) * Math.PI) / 180, y = ((o.yawDeg ?? 0) * Math.PI) / 180;
    const ex: V3 = [Math.cos(t) * Math.cos(y), Math.sin(t), -Math.cos(t) * Math.sin(y)];
    const ey: V3 = [-Math.sin(t) * Math.cos(y), Math.cos(t), Math.sin(t) * Math.sin(y)];
    const ez: V3 = cross(ex, ey);
    const q = quatOf(this.D(bone, ex), this.D(bone, ey), this.D(bone, ez));
    const identity = Math.abs(q[3]) > 0.999999;
    this.push(id, bone, m, { kind: "ellipsoid", radii }, { transform: { ...(identity ? {} : { rotation: q }), translation: this.L(bone, c) }, ...(o.extra ?? {}) });
  }
  /** 🧵️ loft through sections given by world centres; section frames follow the path (explicit rotations, identical to the volume module's `loftSections`). */
  loft(id: string, bone: string, secs: Section[], m: Meta, o: { capStart?: boolean; capEnd?: boolean; extra?: Json; axis?: V3; xdir?: V3; local?: boolean } = {}) {
    const pts = secs.map((s) => (o.local ? s.c : this.L(bone, s.c)));
    const fixed = o.axis ? unit(o.local ? o.axis : this.D(bone, o.axis)) : null;
    const sections = secs.map((s, i) => {
      const from = pts[Math.min(i, pts.length - 2)]!, to = pts[Math.min(i + 1, pts.length - 1)]!;
      const z = fixed ?? unit(sub(to, from));
      let x: V3 = [0, 0, 1];
      const d = dot(x, z);
      x = [x[0] - d * z[0], x[1] - d * z[1], x[2] - d * z[2]];
      if (Math.hypot(...x) < 1e-6) { x = [0, 1, 0]; const e = dot(x, z); x = [x[0] - e * z[0], x[1] - e * z[1], x[2] - e * z[2]]; }
      if (o.xdir) x = unit(this.D(bone, o.xdir));
      else x = unit(x);
      const y = cross(z, x);
      return { at: pts[i], rx: s.rx, ry: s.ry, ...(s.exp !== undefined ? { exponent: s.exp } : {}), rotation: quatOf(x, y, z) };
    });
    this.push(id, bone, m, { kind: "loft", sections, capStart: o.capStart ?? true, capEnd: o.capEnd ?? true }, o.extra ?? {});
  }
  /** 📦️ box centred on a world point with full sizes along world x, y, z. */
  box(id: string, bone: string, c: V3, size: V3, m: Meta, o: { cornerRadius?: number; tiltDeg?: number; extra?: Json } = {}) {
    const t = ((o.tiltDeg ?? 0) * Math.PI) / 180;
    const ex: V3 = [Math.cos(t), Math.sin(t), 0], ey: V3 = [-Math.sin(t), Math.cos(t), 0];
    const q = quatOf(this.D(bone, ex), this.D(bone, ey), this.D(bone, [0, 0, 1]));
    this.push(id, bone, m, { kind: "box", size, ...(o.cornerRadius ? { cornerRadius: o.cornerRadius } : {}) }, { transform: { ...(Math.abs(q[3]) > 0.999999 ? {} : { rotation: q }), translation: this.L(bone, c) }, ...(o.extra ?? {}) });
  }
  /** 🌀️ revolve about a world axis through a world point. profile = [axial, radial]; `axis` world direction. */
  revolve(id: string, bone: string, origin: V3, axis: V3, profile: [number, number][], m: Meta, o: { lateralScale?: number; extra?: Json } = {}) {
    this.push(id, bone, m, { kind: "revolve", profile, axis: unit(this.D(bone, axis)), origin: this.L(bone, origin), ...(o.lateralScale ? { lateralScale: o.lateralScale } : {}) }, o.extra ?? {});
  }
  /** 🔬️ closed rings of an authored shape in paper coordinates of a view. */
  rings(view: "elevation" | "front" | "plan", ids: string | string[]): P2[][] {
    const set = new Set(Array.isArray(ids) ? ids : [ids]);
    const out: P2[][] = [];
    for (const s of this.st[view]) if (set.has(s.shape.id) && s.closed) out.push(...s.rings);
    if (out.length === 0) throw new Error(`no closed rings for ${[...set]} in ${view}`);
    return out;
  }
  area(view: "elevation" | "front" | "plan", ids: string | string[]) {
    let total = 0;
    for (const ring of this.rings(view, ids)) { let q = 0; for (let i = 0; i < ring.length; i++) { const p = ring[i]!, n = ring[(i + 1) % ring.length]!; q += p.x * n.y - n.x * p.y; } total += Math.abs(q) / 2; }
    return total;
  }
  box2(view: "elevation" | "front" | "plan", ids: string | string[]) { return bboxOf(this.rings(view, ids).flat()); }
  /** ✂️ extent [lo,hi] of the rings along the first paper axis where the line second-axis = at crosses them. */
  slice(rings: P2[][], axis: "y" | "x", at: number): [number, number] | null {
    const vals: number[] = [];
    for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const a = ring[k]!, b = ring[i]!;
      const [pa, pb, qa, qb] = axis === "y" ? [a.y, b.y, a.x, b.x] : [a.x, b.x, a.y, b.y];
      if (pa <= at !== pb <= at) vals.push(qa + ((at - pa) / (pb - pa)) * (qb - qa));
    }
    return vals.length ? [Math.min(...vals), Math.max(...vals)] : null;
  }
  /** 🧵️ a loft whose silhouettes in elevation (x, y) and front (z, y) follow authored shapes: sections along y (or along x with axis "x"). */
  loftFromViews(id: string, bone: string, o: { elev: string | string[]; front?: string | string[]; zspan?: [number, number]; n?: number; exp?: number; axis?: "y" | "x"; pinch?: number; m?: Meta; shrink?: number; cx?: number; extra?: Json; range?: [number, number]; zScale?: number; xScale?: number }) {
    const m = o.m ?? this.meta(Array.isArray(o.elev) ? o.elev[0]! : o.elev);
    const E = this.rings("elevation", o.elev);
    const F = o.front ? this.rings("front", o.front) : null;
    const axis = o.axis ?? "y";
    const eb = bboxOf(E.flat());
    const [lo, hi] = o.range ?? (axis === "y" ? [eb.minY, eb.maxY] : [eb.minX, eb.maxX]);
    const n = o.n ?? 7;
    const pinch = o.pinch ?? 0.3;
    const secs: Section[] = [];
    let prev: { c0: number; c1: number; half0: number; half1: number } | null = null;
    for (let k = 0; k < n; k++) {
      const t = k / (n - 1);
      const s = lo + (hi - lo) * t;
      const sc = Math.min(hi - 1e-4, Math.max(lo + 1e-4, s));
      const e = this.slice(E, axis, sc) ?? (prev ? [prev.c0 - prev.half0, prev.c0 + prev.half0] as [number, number] : null);
      if (e === null) continue;
      const f = F ? this.slice(F, axis, sc) : null;
      const [x0, x1] = e;
      let cz = 0, hz = (o.zspan ? (o.zspan[1] - o.zspan[0]) / 2 : (x1 - x0) / 2);
      if (f) { cz = (f[0] + f[1]) / 2; hz = (f[1] - f[0]) / 2; } else if (o.zspan) cz = (o.zspan[0] + o.zspan[1]) / 2;
      const end = k === 0 || k === n - 1 ? pinch : 1;
      const shrink = o.shrink ?? 1;
      const hx = ((x1 - x0) / 2) * end * shrink * (o.xScale ?? 1);
      const hzz = hz * end * shrink * (o.zScale ?? 1);
      prev = { c0: (x0 + x1) / 2, c1: cz, half0: (x1 - x0) / 2, half1: hz };
      const c: V3 = axis === "y" ? [(x0 + x1) / 2, s, cz] : [s, (x0 + x1) / 2, cz];
      secs.push({ c, rx: Math.max(hzz, 0.004), ry: Math.max(hx, 0.004), exp: o.exp ?? 2.2 });
    }
    this.loft(id, bone, secs, m, { extra: o.extra, axis: axis === "y" ? [0, 1, 0] : [1, 0, 0] });
  }
  /** 🫧️ a squarish ellipsoid-like blob: vertical (y) loft whose sections are superellipses scaled by an elliptic profile, centre world, half sizes along x, y, z. */
  blob(id: string, bone: string, c: V3, h: V3, m: Meta, o: { exp?: number; n?: number; extra?: Json; tiltDeg?: number } = {}) {
    const n = o.n ?? 7;
    const secs: Section[] = [];
    for (let k = 0; k < n; k++) {
      const u = (2 * (k + 0.5)) / n - 1;
      const f = Math.sqrt(Math.max(0, 1 - u * u));
      secs.push({ c: [c[0], c[1] + u * h[1], c[2]], rx: Math.max(h[2] * f, 0.004), ry: Math.max(h[0] * f, 0.004), exp: o.exp ?? 2.4 });
    }
    this.loft(id, bone, secs, m, { extra: o.extra, axis: [0, 1, 0] });
  }
  /** 🔎️ ids of the closed authored shapes of a view that sit on a bone with a role. */
  shapesOf(view: "elevation" | "front" | "plan", bone: string, roles: string[] = ["body"]): string[] {
    const out: string[] = [];
    for (const sh of this.st[view]) if (sh.shape.bone === bone && sh.closed && roles.includes(sh.shape.role) && !out.includes(sh.shape.id)) out.push(sh.shape.id);
    return out;
  }
  /** 🍃️ ids of the closed body shapes of a view in a thermal region matching `re`. */
  idsIn(view: "elevation" | "front" | "plan", re: RegExp): string[] {
    const out: string[] = [];
    for (const sh of this.st[view]) if (sh.closed && sh.shape.role === "body" && re.test(sh.shape.region ?? "") && !out.includes(sh.shape.id)) out.push(sh.shape.id);
    return out;
  }
  rawOf(id: string): Json { return this.rawShapes.get(id); }
  bone(id: string): Json { return this.asset.rig.bones.find((b: Json) => b.id === id); }
  tip(bone: string): V3 { const f = this.fr[bone]!; const L = this.bone(bone).lengthM; return [f.o[0] + f.u[0] * L, f.o[1] + f.u[1] * L, f.o[2] + f.u[2] * L]; }
  origin(bone: string): V3 { return [...this.fr[bone]!.o] as V3; }
  /** 🌿️ limb capsules along bones: the centreline follows the bone in the sagittal plane and drifts laterally to `zEnd` (from the front view when omitted: the extreme of the bone's front outline). A bone whose origin lies on its parent starts at the parent's lateral position there. */
  limbs(specs: { bone: string; zStart?: number; zEnd?: number; r0?: number; r1?: number; id?: string; front?: string | string[]; meta?: Meta; split?: number }[]) {
    const ends = this.ends;
    for (const sp of specs) {
      const b = this.bone(sp.bone);
      const o = this.origin(sp.bone), t = this.tip(sp.bone);
      const eIds = this.shapesOf("elevation", sp.bone);
      let rAvg = b.section?.extremeFibreM ?? 0.05;
      if (eIds.length) {
        let area = 0;
        for (const ring of this.rings("elevation", eIds[0]!)) { let q = 0; for (let i = 0; i < ring.length; i++) { const p = ring[i]!, n = ring[(i + 1) % ring.length]!; q += p.x * n.y - n.x * p.y; } area += Math.abs(q) / 2; }
        rAvg = area / b.lengthM / 2;
      }
      const r0 = sp.r0 ?? rAvg * 1.25, r1 = sp.r1 ?? rAvg * 0.75;
      let zStart = 0;
      const parent = b.parent ? ends.get(b.parent) : undefined;
      if (parent) {
        const d: V3 = sub(parent.t, parent.o);
        const len2 = d[0] * d[0] + d[1] * d[1];
        const tt = Math.max(0, Math.min(1, ((o[0] - parent.o[0]) * d[0] + (o[1] - parent.o[1]) * d[1]) / (len2 || 1)));
        zStart = parent.o[2] + tt * (parent.t[2] - parent.o[2]);
      }
      if (sp.zStart !== undefined) zStart = sp.zStart;
      let zEnd = sp.zEnd;
      const ids0 = sp.front ?? this.shapesOf("front", sp.bone);
      if (zEnd === undefined && ids0.length === 0) {
        const L = Math.hypot(t[0] - o[0], t[1] - o[1]);
        const pl = parent ? Math.hypot(parent.t[0] - parent.o[0], parent.t[1] - parent.o[1]) : 1;
        const slope = parent ? (parent.t[2] - parent.o[2]) / (pl || 1) : 0;
        zEnd = zStart + slope * L;
      }
      if (zEnd === undefined) {
        const ids = ids0;
        const bb = this.box2("front", ids);
        const ext = Math.abs(bb.minX) > Math.abs(bb.maxX) ? bb.minX : bb.maxX;
        zEnd = ext - Math.sign(ext) * r1;
      }
      const a: V3 = [o[0], o[1], zStart], c: V3 = [t[0], t[1], zEnd];
      ends.set(sp.bone, { o: a, t: c });
      const m = sp.meta ?? this.meta(this.shapesOf("elevation", sp.bone)[0] ?? sp.bone);
      this.cap(sp.id ?? sp.bone, sp.bone, a, c, r0, r1, m);
    }
    return ends;
  }
  /** 🍃️ one loft per foliage bone, silhouettes read from its authored elevation and front outlines. */
  masses(bones: string[], o: { n?: number; exp?: number; pinch?: number; xScale?: number; zScale?: number; twig?: number; twigMeta?: Meta } = {}) {
    for (const bone of bones) {
      const e = this.shapesOf("elevation", bone), f = this.shapesOf("front", bone);
      if (e.length === 0) throw new Error(`no elevation mass for ${bone}`);
      if (o.twig !== undefined) {
        const eb = this.box2("elevation", e), fb = f.length ? this.box2("front", f) : null;
        const parent = this.ends.get(this.bone(bone).parent ?? "");
        const og = this.origin(bone);
        const z0 = parent ? parent.t[2] : 0;
        const c: V3 = [(eb.minX + eb.maxX) / 2, (eb.minY + eb.maxY) / 2, fb ? (fb.minX + fb.maxX) / 2 : z0];
        const tm = o.twigMeta ?? this.meta(e[0]!, { material: this.asset.materials.find((m: Json) => /twig|bark|wood/.test(m.id))!.id, role: "body", tone: 0.3, region: undefined });
        this.cap(`${bone}-twig`, bone, [og[0], og[1], z0], c, o.twig, o.twig * 0.6, tm);
      }
      this.loftFromViews(`${bone}-mass`, bone, { elev: e, front: f.length ? f : undefined, n: o.n ?? 9, exp: o.exp ?? 2.4, pinch: o.pinch ?? 0.25, m: this.meta(e[0]!), xScale: o.xScale, zScale: o.zScale });
    }
  }
  /** 🫧️ a crown fill blob under a plan outline: x and z from the plan bbox, y from the intersection of the elevation and front unions at its centre, shrunk. */
  fill(id: string, bone: string, planId: string, o: { elevIds: string[]; frontIds: string[]; shrink?: number; n?: number; exp?: number; y?: [number, number]; m?: Meta; extra?: Json }) {
    const b = this.box2("plan", planId);
    const cx = (b.minX + b.maxX) / 2, cz = (b.minY + b.maxY) / 2;
    const ye = this.slice(this.rings("elevation", o.elevIds), "x", cx);
    const yf = this.slice(this.rings("front", o.frontIds), "x", cz);
    let lo = Math.max(ye?.[0] ?? -Infinity, yf?.[0] ?? -Infinity), hi = Math.min(ye?.[1] ?? Infinity, yf?.[1] ?? Infinity);
    if (o.y) [lo, hi] = o.y;
    const mid = (lo + hi) / 2, half = ((hi - lo) / 2) * (o.shrink ?? 0.9);
    this.blob(id, bone, [cx, mid, cz], [(b.maxX - b.minX) / 2, half, (b.maxY - b.minY) / 2], o.m ?? this.meta(planId), { n: o.n ?? 9, exp: o.exp ?? 2.6, extra: o.extra });
  }
  /** 📐️ principal-axis ellipse of an authored plan outline: centre (x, z), major direction angle `phi` in the plan, semi axes a >= b (boundary samples, ellipse fit). */
  pca(planId: string, k = 1) {
    const ring = this.rings("plan", planId).flat();
    const n = ring.length;
    const cx = ring.reduce((t, p) => t + p.x, 0) / n, cz = ring.reduce((t, p) => t + p.y, 0) / n;
    let sxx = 0, szz = 0, sxz = 0;
    for (const p of ring) { sxx += (p.x - cx) ** 2; szz += (p.y - cz) ** 2; sxz += (p.x - cx) * (p.y - cz); }
    sxx /= n; szz /= n; sxz /= n;
    const phi = 0.5 * Math.atan2(2 * sxz, sxx - szz);
    const tr = sxx + szz, det = sxx * szz - sxz * sxz, disc = Math.sqrt(Math.max(0, tr * tr / 4 - det));
    return { cx, cz, phi, a: Math.SQRT2 * Math.sqrt(tr / 2 + disc) * k, b: Math.SQRT2 * Math.sqrt(Math.max(1e-6, tr / 2 - disc)) * k };
  }
  /** 📐️ lean of the authored elevation mass: angle (deg, about z; positive leans the top toward -x) of its major axis from vertical and the aspect ratio of its principal axes. */
  elevLean(ids: string | string[]) {
    const ring = this.rings("elevation", ids).flat();
    const n = ring.length;
    const cx = ring.reduce((t, p) => t + p.x, 0) / n, cy = ring.reduce((t, p) => t + p.y, 0) / n;
    let sxx = 0, syy = 0, sxy = 0;
    for (const p of ring) { sxx += (p.x - cx) ** 2; syy += (p.y - cy) ** 2; sxy += (p.x - cx) * (p.y - cy); }
    const phi = 0.5 * Math.atan2(2 * sxy, sxx - syy);
    const tr = sxx + syy, det = sxx * syy - sxy * sxy, disc = Math.sqrt(Math.max(0, tr * tr / 4 - det));
    const l1 = tr / 2 + disc, l2 = Math.max(1e-9, tr / 2 - disc);
    let dx = Math.cos(phi), dy = Math.sin(phi);
    if (dy < 0) { dx = -dx; dy = -dy; }
    return { tiltDeg: (Math.atan2(-dx, dy) * 180) / Math.PI, aspect: Math.sqrt(l1 / l2), vertical: Math.abs(dy) > Math.abs(dx) };
  }
  /** 🥚️ a flat foliage lobe: the oriented ellipse (principal axes of the authored plan outline) at the height of the authored front outline; optionally a twig from `from` to its centre. */
  lobe(id: string, bone: string, planId: string, frontId: string | null, o: { k?: number; yRange?: [number, number]; m?: Meta; extra?: Json; from?: V3; twig?: number; twigMeta?: Meta; ky?: number; xBounds?: [number, number]; tiltDeg?: number; boxy?: number; cz?: number } = {}) {
    const ring = this.rings("plan", planId).flat();
    const n = ring.length;
    const cx = ring.reduce((t, p) => t + p.x, 0) / n, cz = o.cz ?? ring.reduce((t, p) => t + p.y, 0) / n;
    let sxx = 0, szz = 0, sxz = 0;
    for (const p of ring) { sxx += (p.x - cx) ** 2; szz += (p.y - cz) ** 2; sxz += (p.x - cx) * (p.y - cz); }
    sxx /= n; szz /= n; sxz /= n;
    const phi = 0.5 * Math.atan2(2 * sxz, sxx - szz);
    const tr = sxx + szz, det = sxx * szz - sxz * sxz, disc = Math.sqrt(Math.max(0, tr * tr / 4 - det));
    const k = o.k ?? 1.04;
    const a = Math.SQRT2 * Math.sqrt(tr / 2 + disc) * k, b = Math.SQRT2 * Math.sqrt(Math.max(1e-6, tr / 2 - disc)) * k;
    const [y0, y1] = o.yRange ?? (frontId ? ((bb) => [bb.minY, bb.maxY] as [number, number])(this.box2("front", frontId)) : [0, 1]);
    const ky = o.ky ?? 1;
    const m = o.m ?? this.meta(planId);
    const cy = (y0 + y1) / 2;
    let cxx = cx;
    if (o.xBounds) {
      const xe = Math.sqrt((a * Math.cos(phi)) ** 2 + (b * Math.sin(phi)) ** 2);
      cxx = Math.min(o.xBounds[1] - xe, Math.max(o.xBounds[0] + xe, cx));
    }
    if (o.boxy) {
      const n = 7, hy = ((y1 - y0) / 2) * ky;
      const secs: Section[] = [];
      for (let q = 0; q < n; q++) { const u = -1 + (2 * q) / (n - 1); const f = Math.max(0.3, (1 - Math.abs(u) ** o.boxy) ** (1 / o.boxy)); secs.push({ c: [cxx, cy + u * hy, cz], rx: a * f, ry: b * f, exp: 2.6 }); }
      this.loft(id, bone, secs, m, { axis: [0, 1, 0], xdir: [Math.cos(phi), 0, Math.sin(phi)], extra: o.extra });
    } else this.ell(id, bone, [cxx, cy, cz], [a, ((y1 - y0) / 2) * ky, b], m, { yawDeg: (-phi * 180) / Math.PI, tiltDeg: o.tiltDeg, extra: o.extra });
    if (o.twig !== undefined && o.from) {
      const tm = o.twigMeta ?? m;
      this.cap(`${id}-twig`, bone, o.from, [cxx, cy - ((y1 - y0) / 2) * 0.3, cz], o.twig, o.twig * 0.6, tm);
    }
    return { cx, cy, cz, a, b, y0, y1 };
  }
  /** 🌳️ foliage tiers: for every front outline `f-<name>` the plan outline `p-<name>`; cores and tops (matching `coreRe`) become lofts from elevation and front, the others oriented lobes with a twig from the carrying bone's origin. */
  lobeTiers(o: { twig: number; twigMeta: Meta; coreRe: RegExp; k?: number; ky?: number; frontPrefix?: string; planPrefix?: string; blendY?: boolean; n?: number; exp?: number; clampX?: boolean; lean?: number; clampY?: boolean; boxy?: number }) {
    const fp = o.frontPrefix ?? "f-", pp = o.planPrefix ?? "p-";
    for (const sh of this.st.front) {
      const id = sh.shape.id as string;
      if (!sh.closed || sh.shape.role !== "body" || !id.startsWith(fp)) continue;
      const bone = sh.shape.bone as string;
      const name = id.slice(fp.length);
      if (o.coreRe.test(name)) {
        this.loftFromViews(`crown-${name}`, bone, { elev: this.shapesOf("elevation", bone), front: id, n: o.n ?? 8, exp: o.exp ?? 2.4, pinch: 0.3, m: this.meta(id) });
        continue;
      }
      const fb = this.box2("front", id);
      const eIds = this.shapesOf("elevation", bone);
      const e = eIds.length ? this.box2("elevation", eIds) : fb;
      const og = this.origin(bone);
      const parent = this.ends.get(this.bone(bone).parent ?? "");
      this.lobe(`lobe-${name}`, bone, `${pp}${name}`, id, { k: o.k, ky: o.ky, boxy: o.boxy, xBounds: o.clampX && !(o.lean && eIds.length && this.elevLean(eIds).aspect > o.lean) ? [e.minX, e.maxX] : undefined, tiltDeg: o.lean && eIds.length && this.elevLean(eIds).aspect > o.lean && this.elevLean(eIds).vertical ? this.elevLean(eIds).tiltDeg : undefined, m: this.meta(id), yRange: o.clampY ? ((h, eh) => { const hh = Math.min(h, eh); const c = Math.min(e.maxY - hh / 2, Math.max(e.minY + hh / 2, (fb.minY + fb.maxY) / 2)); return [c - hh / 2, c + hh / 2] as [number, number]; })(fb.maxY - fb.minY, e.maxY - e.minY) : o.blendY === false ? undefined : [(fb.minY + e.minY) / 2, (fb.maxY + e.maxY) / 2], from: [og[0], og[1], parent ? parent.t[2] : 0], twig: o.twig, twigMeta: o.twigMeta });
    }
  }

  /** 🧲️ the world point of an anchor (bone-local x, y at rest). */
  anchorWorld(id: string): V3 {
    const an = this.asset.anchors.find((x: Json) => x.id === id);
    const f = this.fr[an.bone]!;
    return toWorld(f, [an.x, an.y, 0]);
  }
  /** 🧮️ binary search of a scalar `k` so that the thermal surface area of `region` reaches `target` m2 (make(k) must add the region's solids). */
  fitArea(region: string, target: number, make: (k: number) => void, lo = 0.05, hi = 6) {
    const keep = this.lists.thermal.length;
    let best = lo;
    for (let it = 0; it < 22; it++) {
      const k = (lo + hi) / 2;
      this.lists.thermal.length = keep;
      make(k);
      const area = this.areaOf("thermal", (s) => s.region === region);
      if (area < target) lo = k; else hi = k;
      best = k;
    }
    this.lists.thermal.length = keep;
    make(best);
    const got = this.areaOf("thermal", (s) => s.region === region);
    console.log(`[fit] 1:${this.scale} ${region}: k=${best.toFixed(3)} area=${got.toFixed(3)} target=${target}`);
    return { k: best, area: got, target };
  }
  /** 💨️ an emitter solid: a flat disc of the emitter's own diameter (capped to `maxD`) at an anchor, normal along world x (directionDeg 0) unless `normal` says otherwise. */
  emitter(emitterId: string, o: { at?: V3; maxD?: number; region?: string; bone?: string; idx?: number; tilt?: number; flat?: "z" } = {}) {
    const em = this.asset.thermal.emitters.find((e: Json) => e.id === emitterId);
    const anchorId = em.anchors?.[o.idx ?? 0] ?? em.anchor;
    const an = this.asset.anchors.find((x: Json) => x.id === anchorId);
    const at = o.at ?? this.anchorWorld(anchorId);
    const d = Math.min(em.diameterM ?? 0.1, o.maxD ?? 1e9);
    const t = Math.max(this.feature * 0.6, 0.01);
    const region = o.region ?? em.region ?? this.asset.thermal.regions[0].id;
    const m = this.mat(this.asset.thermal.regions.find((r: Json) => r.id === region).material, 0.45, "detail");
    const prev = [this.target, this.defaults] as const;
    this.into("thermal", { region });
    this.ell(`th-${emitterId}`, o.bone ?? an.bone, at, o.flat === "z" ? [d / 2, d / 2, t] : [t, d / 2, d / 2], m, { tiltDeg: o.tilt ?? 0, extra: { tags: [emitterId] } });
    this.into(prev[0], prev[1]);
  }
  /** 🪵️ a member capsule along a bone's local +x between fractions t0..t1 of its length, offset (dy, dz) in the bone frame. */
  member(id: string, bone: string, r0: number, r1: number, o: { t0?: number; t1?: number; dy?: number; dz?: number; tone?: number; material?: string; part?: string } = {}) {
    const b = this.bone(bone);
    const L = b.lengthM;
    const m = this.mat(o.material ?? b.section?.material ?? this.asset.materials[0].id, o.tone ?? 0.4, "body");
    const prev = [this.target, this.defaults] as const;
    this.into("structure", { member: bone, part: o.part ?? "member" });
    const t0 = o.t0 ?? 0, t1 = o.t1 ?? 1;
    const spec = { id, bone, r0, r1, t0, t1, dy: o.dy ?? 0, dz: o.dz ?? 0, L };
    this.members.push(spec);
    this.push(id, bone, m, { kind: "capsule", p0: [t0 * L, spec.dy, spec.dz], p1: [t1 * L, spec.dy, spec.dz], r0, r1 });
    this.into(prev[0], prev[1]);
  }
  readonly members: { id: string; bone: string; r0: number; r1: number; t0: number; t1: number; dy: number; dz: number; L: number }[] = [];
  /** ⚓️ a support solid (fixed block, pin, roller or contact pad) at a support anchor: box of the footprint, top face on the anchor, buried by `depth`. */
  support(anchorId: string, kind: "block" | "pad" | "pin" | "roller", o: { size?: [number, number, number]; centre?: V3; material?: string; depth?: number } = {}) {
    const an = this.asset.anchors.find((x: Json) => x.id === anchorId);
    const w = o.size?.[0] ?? Math.max(an.footprintLengthM ?? 0.1, this.feature);
    const dpt = o.size?.[2] ?? Math.max(an.footprintWidthM ?? 0.1, this.feature);
    const h = o.size?.[1] ?? Math.max(this.feature * 2, o.depth ?? 0.05);
    const c = o.centre ?? ((p) => [p[0], p[1] - h / 2, p[2]] as V3)(this.anchorWorld(anchorId));
    const prev = [this.target, this.defaults] as const;
    this.into("structure", { member: an.bone, part: "support", anchor: anchorId });
    const m = this.mat(o.material ?? this.asset.materials[0].id, 0.25, "detail");
    if (kind === "pin") this.ell(`support-${anchorId}`, an.bone, c, [Math.min(w, h) / 2, h / 2, Math.min(w, dpt) / 2], m);
    else if (kind === "roller") this.cap(`support-${anchorId}`, an.bone, [c[0] - w / 2 + h / 2, c[1], c[2]], [c[0] + w / 2 - h / 2, c[1], c[2]], h / 2, h / 2, m);
    else this.box(`support-${anchorId}`, an.bone, c, [w, h, dpt], m, { cornerRadius: kind === "pad" ? Math.min(h, w, dpt) * 0.2 : 0 });
    this.into(prev[0], prev[1]);
  }
  /** 🔩️ a joint ellipsoid at the bone's origin (world) of the given radius. */
  joint(id: string, bone: string, r: number, o: { at?: V3; material?: string } = {}) {
    const b = this.bone(bone);
    const prev = [this.target, this.defaults] as const;
    this.into("structure", { member: bone, part: "joint" });
    this.ell(id, bone, o.at ?? this.origin(bone), [r, r, r], this.mat(o.material ?? b.section?.material ?? this.asset.materials[0].id, 0.35, "detail"));
    this.into(prev[0], prev[1]);
  }
  /** 🎯️ a compact mass solid at a bone's centre of mass. */
  mass(id: string, bone: string, r: number, o: { at?: V3; radii?: V3 } = {}) {
    const b = this.bone(bone);
    const f = this.fr[bone]!;
    const at = o.at ?? toWorld(f, [b.mass.comX ?? 0, b.mass.comY ?? 0, 0]);
    const prev = [this.target, this.defaults] as const;
    this.into("structure", { member: bone, part: "mass" });
    this.ell(id, bone, at, o.radii ?? [r, r, r], this.mat(this.asset.materials[0].id, 0.15, "detail"));
    this.into(prev[0], prev[1]);
  }
  /** 📏️ shrinks member capsules until their projections lie within one feature (plus one raster cell) of the body silhouette in all three views; returns what moved. */
  fitMembers(maxIter = 14): string[] {
    const moved: string[] = [];
    const body = meshes(this.asset, this.lists.body, 20);
    const view = VIEWS.map((v) => viewMasks(v, this.st, body));
    const fields = view.map((vm) => distanceField(vm.volume));
    for (const spec of this.members) {
      for (let it = 0; it < maxIter; it++) {
        const solid = this.lists.structure.find((s) => s.id === spec.id)!;
        const [m] = meshes(this.asset, [solid], 20);
        let worst = 0;
        VIEWS.forEach((v, k) => {
          const mk = meshMask(v, [m!], view[k]!.box, view[k]!.cell);
          for (let i = 0; i < mk.cells.length; i++) if (mk.cells[i]! && fields[k]![i]! - view[k]!.cell > worst) worst = fields[k]![i]! - view[k]!.cell;
        });
        if (worst <= this.feature * 0.6) break;
        const len = spec.t1 - spec.t0;
        if (it % 2 === 0) { spec.t1 -= len * 0.08; spec.t0 += len * 0.04; } else { spec.r0 *= 0.88; spec.r1 *= 0.88; }
        spec.r0 = Math.max(spec.r0, this.feature * 0.55); spec.r1 = Math.max(spec.r1, this.feature * 0.55);
        solid.p0 = [spec.t0 * spec.L, spec.dy, spec.dz].map((x) => Math.round(x * 1e5) / 1e5);
        solid.p1 = [spec.t1 * spec.L, spec.dy, spec.dz].map((x) => Math.round(x * 1e5) / 1e5);
        solid.r0 = Math.round(spec.r0 * 1e5) / 1e5; solid.r1 = Math.round(spec.r1 * 1e5) / 1e5;
        if (!moved.includes(spec.id)) moved.push(spec.id);
      }
    }
    return moved;
  }

  /** 🧅️ a copy of a body solid scaled by `k` about its own centre (loft, ellipsoid, capsule, box, revolve), pushed into the current list under a new id with the current defaults. */
  scaled(bodyId: string, id: string, k: number, extra: Json = {}, kz = k) {
    const base = this.lists.body.find((x) => x.id === bodyId);
    if (base === undefined) throw new Error(`no body solid ${bodyId}`);
    const c = JSON.parse(JSON.stringify(base));
    c.id = id;
    if (c.kind === "loft") {
      const mid = [0, 1, 2].map((i) => c.sections.reduce((n: number, q: Json) => n + q.at[i], 0) / c.sections.length);
      c.sections = c.sections.map((q: Json) => ({ ...q, at: q.at.map((v: number, i: number) => mid[i]! + (v - mid[i]!) * (i === 2 ? kz : k)), rx: q.rx * kz, ry: q.ry * k }));
    } else if (c.kind === "ellipsoid") c.radii = c.radii.map((v: number) => v * k);
    else if (c.kind === "capsule") {
      const mid = [0, 1, 2].map((i) => (c.p0[i] + c.p1[i]) / 2);
      c.p0 = c.p0.map((v: number, i: number) => mid[i]! + (v - mid[i]!) * k);
      c.p1 = c.p1.map((v: number, i: number) => mid[i]! + (v - mid[i]!) * k);
      c.r0 *= k; c.r1 *= k;
    } else if (c.kind === "box") { c.size = c.size.map((v: number) => v * k); if (c.cornerRadius) c.cornerRadius *= k; }
    else if (c.kind === "revolve") c.profile = c.profile.map(([u, r]: number[]) => [u * k, r * k]);
    else throw new Error(`cannot scale ${c.kind}`);
    this.solids.push(rnd({ ...c, ...this.defaults, ...extra }));
  }

  /** 🔎️ per non-support structure solid the worst distance (m, minus one cell) of its projection outside the body silhouette. */
  outside(): [string, number][] {
    const body = meshes(this.asset, this.lists.body, 20);
    const view = VIEWS.map((v) => viewMasks(v, this.st, body));
    const fields = view.map((vm) => distanceField(vm.volume));
    const out: [string, number][] = [];
    for (const solid of this.lists.structure) {
      if (solid.part === "support") continue;
      const resolved = solid.of === undefined ? solid : { ...this.lists.body.find((b) => b.id === solid.of)!, id: solid.id };
      const [m] = meshes(this.asset, [resolved], 20);
      let worst = 0;
      VIEWS.forEach((v, k) => {
        const mk = meshMask(v, [m!], view[k]!.box, view[k]!.cell);
        for (let i = 0; i < mk.cells.length; i++) if (mk.cells[i]! && fields[k]![i]! - view[k]!.cell > worst) worst = fields[k]![i]! - view[k]!.cell;
      });
      out.push([solid.id, worst]);
    }
    return out;
  }

  /** 🌡️ thermal solids of one region from body solids chosen by `pick`: `ref` reuses them, `scaled` shrinks every copy by a common factor fitted to the authored area (never above 1). */
  thermalRegion(region: string, pick: (s: Json) => boolean, mode: "ref" | "scaled", opts: { prefix?: string } = {}) {
    const reg = this.asset.thermal.regions.find((r: Json) => r.id === region);
    const ids = this.lists.body.filter(pick).map((s) => s.id);
    if (ids.length === 0) throw new Error(`no body solids for region ${region}`);
    const px = opts.prefix ?? "th-";
    if (mode === "ref") { for (const id of ids) this.ref(`${px}${id}`, id, region); return; }
    const prev = [this.target, this.defaults] as const;
    this.into("thermal", { region });
    this.fitArea(region, reg.areaM2, (k) => { for (const id of ids) this.scaled(id, `${px}${id}`, k); }, 0.05, 1);
    this.into(prev[0], prev[1]);
  }
  /** 🪵️ a member capsule along every bone of the structural view that has none yet; radii from the bone section (circumscribed circle) else from `r`. */
  autoMembers(r: (bone: Json) => [number, number], o: { t0?: number; t1?: number; skip?: string[] } = {}) {
    for (const id of memberBones(this.asset, this.scale)) {
      if (o.skip?.includes(id) || this.members.some((m) => m.bone === id) || this.lists.structure.some((x) => x.member === id && x.part === "member")) continue;
      const b = this.bone(id);
      const [r0, r1] = r(b);
      this.member(`member-${id}`, id, Math.max(r0, this.feature * 0.55), Math.max(r1, this.feature * 0.55), { t0: o.t0 ?? 0.03, t1: o.t1 ?? 0.97 });
    }
  }
  /** 🔩️ joint balls at the origin of every non-fixed bone that has a member solid. */
  autoJoints(r: (bone: Json) => number, o: { only?: (b: Json) => boolean } = {}) {
    for (const b of this.asset.rig.bones) {
      if (b.parent === null || b.joint?.type === "fixed") continue;
      if (!this.lists.structure.some((x) => x.member === b.id && x.part === "member")) continue;
      if (o.only && !o.only(b)) continue;
      this.joint(`joint-${b.id}`, b.id, Math.max(r(b), this.feature * 0.6));
    }
  }

  /** 🔎️ thermal solids that leave the body bounding box by more than one feature. */
  strays(): string[] {
    const e = extentOf(meshes(this.asset, this.lists.body, 20))!;
    const bodyById = new Map(this.lists.body.map((s) => [s.id, s]));
    const out: string[] = [];
    for (const t of this.lists.thermal) {
      const r = t.of === undefined ? t : { ...bodyById.get(t.of)!, id: t.id };
      const te = extentOf(meshes(this.asset, [r], 20))!;
      const d = [0, 1, 2].reduce((m, k) => Math.max(m, e.min[k]! - te.min[k]!, te.max[k]! - e.max[k]!), 0);
      if (d > this.feature) out.push(`${t.id}:${d.toFixed(3)}`);
    }
    return out;
  }

  /** 🪵️ a member capsule between two world points (placed where the body really is when the bone axis lies outside it). */
  memberWorld(id: string, bone: string, p0: V3, p1: V3, r: number, o: { material?: string } = {}) {
    const prev = [this.target, this.defaults] as const;
    this.into("structure", { member: bone, part: "member" });
    this.cap(id, bone, p0, p1, r, r, this.mat(o.material ?? this.bone(bone).section?.material ?? this.asset.materials[0].id, 0.4));
    this.into(prev[0], prev[1]);
  }
  note(en: string, de: string) { this.decisions.push({ en, de }); }
  volume() {
    return { status: "authored", denominator: this.scale, body: this.lists.body, thermal: this.lists.thermal, structure: this.lists.structure, decisions: this.decisions };
  }
}

export function save(slug: string, authors: Partial<Record<number, Author>>) {
  const asset = loadAsset(resolveAsset(`vegetation/${slug}`).path);
  const volumes: Json = {};
  for (const s of [50, 100, 200, 500]) {
    if (asset.representations[String(s)].status === "unavailable") volumes[String(s)] = { status: "unavailable", denominator: s };
    else volumes[String(s)] = authors[s]!.volume();
  }
  const file = join(TICKET, "volumes", "vegetation", `${slug}.json`);
  writeFileSync(file, `${JSON.stringify({ volumes }, null, 1)}\n`);
  console.log(`${file}: ` + [50, 100, 200, 500].map((s) => (authors[s] ? `${s}: ${authors[s]!.lists.body.length}/${authors[s]!.lists.thermal.length}/${authors[s]!.lists.structure.length}` : `${s}: unavailable`)).join("  "));
}
