/** 🦴️ Fauna: anatomical loft templates for birds and quadrupeds on top of the author-41 `Zoo` (world-space lofts constrained by the scale's own drawings).
 * Anatomical sources the proportions follow: Ellenberger/Dittrich/Baum "Handbook of Animal Anatomy for Artists" (skeleton, ribcage, limb joints), Goldfinger "Animal Anatomy for Artists" (surface forms of cat, dog, horse, deer, pig, sheep), Baumel et al. "Handbook of Avian Anatomy: Nomina Anatomica Avium" (keel, wing, tarsometatarsus), Proctor and Lynch "Manual of Ornithology" (folded wing, remiges), Olsen and Larsson "Gulls of Europe, Asia and North America" and Cramp/Simmons BWP (herring gull and pigeon dimensions), breed and species standards for proportions.
 * @see ./fauna.md
 * @see ./author-41-lib.ts */
import { Zoo, range, type Look, type Station } from "./author-41-lib.ts";
import { type Json, type V3 } from "./kit-lib.ts";

export { range, type Look, type Station };
export type P3 = V3;

const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const crs = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V3): V3 => { const l = Math.hypot(...a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
const lerp = (a: number, b: number, t: number): number => a + (b - a) * t;
const clamp = (x: number, a: number, b: number): number => Math.max(a, Math.min(b, x));
const smooth = (t: number): number => { const u = clamp(t, 0, 1); return u * u * (3 - 2 * u); };

/** 📈️ Piecewise-linear profile through `[t, value]` knots (t ascending), clamped at both ends.
 * @example prof([[0, 1], [1, 3]])(0.5) // 2 */
export const prof = (knots: readonly (readonly [number, number])[]): ((t: number) => number) => (t) => {
  if (t <= knots[0]![0]) return knots[0]![1];
  for (let i = 1; i < knots.length; i++) if (t <= knots[i]![0]) { const a = knots[i - 1]!; const b = knots[i]!; return lerp(a[1], b[1], (t - a[0]) / (b[0] - a[0] || 1)); }
  return knots[knots.length - 1]![1];
};

/** 📍️ A station of a path loft: centre `p` (world), lateral radius `rl`, sagittal radius `rs`, section exponent `e`, roll (rad) of the section about the path. */
export type PP = { p: V3; rl: number; rs: number; e?: number; roll?: number };
export type Lod = 0 | 1 | 2;

export class Fauna extends Zoo {
  /** 🧶️ Loft along a 3D path: every section is perpendicular to the local path tangent, `rl` along the lateral reference (default world z), `rs` perpendicular to both.
   * @example S.path("thigh", "thigh-n", [{ p: [0, 0.2, -0.04], rl: 0.02, rs: 0.03 }, { p: [0.03, 0.15, -0.04], rl: 0.012, rs: 0.014 }], look) */
  path(id: string, bone: string, pts: readonly PP[], look: Look, o: { lat?: V3; caps?: boolean; floor?: number } = {}): Json {
    const f = o.floor ?? this.feature * 0.55;
    const st: Station[] = pts.map((q, i) => {
      const a = pts[Math.max(0, i - 1)]!.p;
      const b = pts[Math.min(pts.length - 1, i + 1)]!.p;
      const t = unit(sub(b, a));
      const ref = o.lat ?? [0, 0, 1];
      let l = sub(ref, mul(t, dot(ref, t)));
      if (Math.hypot(...l) < 0.2) l = sub([0, 1, 0], mul(t, dot([0, 1, 0], t)));
      l = unit(l);
      let s = crs(t, l);
      if (q.roll) { const c = Math.cos(q.roll); const n = Math.sin(q.roll); const l2 = add(mul(l, c), mul(s, n)); s = add(mul(s, c), mul(l, -n)); l = l2; }
      return { c: q.p, a: mul(l, Math.max(f, q.rl)), b: mul(s, Math.max(f, q.rs)), e: q.e };
    });
    return this.loft(id, bone, st, look, o.caps ?? true);
  }

  /** 🥚️ Spindle-shaped muscle mass between `a` and `b` (world): sections perpendicular to the axis whose radii follow a rounded profile that peaks at the fraction `peak` (0.5 = ellipsoid, smaller = the bulk sits near `a`, as a ham or a shoulder hangs from its top), `rl` lateral and `rs` sagittal at the peak, both ends closed to the floor. `lat` is the lateral reference of the sections, `e` their exponent.
   * @example S.mass("ham", "thigh-n", [0, 0.67, -0.11], [0.06, 0.33, -0.1], { rl: 0.08, rs: 0.13, peak: 0.4, n: 9 }, look) */
  mass(id: string, bone: string, a: V3, b: V3, o: { rl: number; rs: number; peak?: number; n: number; e?: number; lat?: V3; floor?: number }, look: Look): Json {
    const pk = o.peak ?? 0.5;
    const pts: PP[] = [];
    for (let i = 0; i <= o.n; i++) {
      const t = i / o.n;
      const u = t < pk ? (pk - t) / pk : (t - pk) / (1 - pk);
      const w = Math.sqrt(Math.max(0, 1 - u * u));
      pts.push({ p: [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)], rl: o.rl * w, rs: o.rs * w, e: o.e ?? 2.2 });
    }
    return this.path(id, bone, pts, look, { lat: o.lat ?? [0, 0, 1], ...(o.floor === undefined ? {} : { floor: o.floor }) });
  }

  /** 🔘️ Ellipsoid with its long axis along `dir` (world), `r = [along, lateral, sagittal]`. */
  blob(id: string, bone: string, at: V3, r: V3, look: Look, dir: V3 = [1, 0, 0]): Json {
    const d = unit(dir);
    let l = sub([0, 0, 1], mul(d, dot([0, 0, 1], d)));
    if (Math.hypot(...l) < 0.2) l = sub([0, 1, 0], mul(d, dot([0, 1, 0], d)));
    l = unit(l);
    const f = this.feature * 0.55;
    return this.ell(id, bone, at, [Math.max(f, r[0]), Math.max(f, r[1]), Math.max(f, r[2])], look, [d, l]);
  }

  /** 🧭️ Outline spans: vertical extent of the drawn elevation shapes `ids` at x. */
  ys(ids: readonly string[], x: number): [number, number] | null { return this.span("elevation", ids, "x", x); }
  zs(ids: readonly string[], x: number): [number, number] | null { return this.span("plan", ids, "x", x); }

  /** 🦻️ Pointed leaf ear (cat, fox, deer): a flattened cone along y between `y0` (base) and `y1` (tip): broad in z as drawn in the front view (`front` shape ids), thin in x (`thin` of the drawn elevation width), the tip narrowing to a point, a slight outward lean `lean` (m over the height) and a cupped front (the axis of the leaf curves forward by `cup`).
   * @example S.ear("near-ear", "ear-n", { elev: ["near-ear"], front: ["near-ear"], y0: 0.305, y1: 0.357, n: 6, look, lean: -0.004 }) */
  ear(id: string, bone: string, o: { elev: readonly string[]; front: readonly string[]; y0: number; y1: number; n: number; look: Look; lean?: number; thin?: number; cup?: number; e?: number; floor?: number }): Json {
    const pts: PP[] = [];
    const f = o.floor ?? this.feature * 0.55;
    for (const [i, y] of range(o.y0, o.y1, o.n).entries()) {
      const t = i / (o.n - 1);
      const ex = this.span("elevation", o.elev, "y", y);
      const fz = this.span("front", o.front, "y", y);
      if (ex === null || fz === null) continue;
      const taper = t > 0.7 ? Math.max(0.03, 1 - (t - 0.7) / 0.3 * 0.97) : 1;
      pts.push({ p: [(ex[0] + ex[1]) / 2 + (o.cup ?? 0) * Math.sin(Math.PI * t), y, (fz[0] + fz[1]) / 2 + (o.lean ?? 0) * t], rl: Math.max(f, ((ex[1] - ex[0]) / 2) * (o.thin ?? 0.4) * taper), rs: Math.max(f, ((fz[1] - fz[0]) / 2) * taper), e: o.e ?? 2.4 });
    }
    return this.path(id, bone, pts, o.look, { lat: [1, 0, 0], ...(o.floor === undefined ? {} : { floor: o.floor }) });
  }

  /* -------------------------------------------------------------- quadrupeds */

  /** 🦴️ Dorsal ridge (spinous processes under the skin, withers to croup): a slender loft that follows the top edge of the drawn elevation shapes `elev` between `x0` and `x1`, `w` half width, `h` half height, sunk `sink` below the outline so it stays inside the drawn silhouette.
   * @example S.ridge("spine", "lumbar", { elev: ["hindbody", "chest"], x0: -0.2, x1: 0.3, n: 9, w: 0.01, h: 0.008, sink: 0.004, look }) */
  ridge(id: string, bone: string, o: { elev: readonly string[]; x0: number; x1: number; n: number; w: number; h: number; sink: number; look: Look; e?: number }): Json {
    const pts: PP[] = [];
    for (const x of range(o.x0, o.x1, o.n)) {
      const sp = this.ys(o.elev, x);
      if (sp === null) continue;
      pts.push({ p: [x, sp[1] - o.sink - o.h, 0], rl: o.w, rs: o.h, e: o.e ?? 2.4 });
    }
    return this.path(id, bone, pts, o.look, { lat: [0, 0, 1] });
  }

  /** 💪️ Pair of lateral muscle masses (shoulder, ham, cheek) placed at (x, y) just inside the drawn plan half width, so they swell the flank without leaving the silhouette: radii `r = [along, sagittal, lateral]`, `inset` 0..1 (1 = outer surface on the plan outline).
   * @example S.lateralPair("ham", "thigh-hn", -0.15, 0.55, [0.09, 0.1, 0.05], look, { plan: ["hindbody"], bones: ["thigh-hn", "thigh-hf"] }) */
  lateralPair(id: string, bone: string, x: number, y: number, r: V3, look: Look, o: { plan: readonly string[]; bones?: [string, string]; inset?: number; dir?: V3 }): void {
    const pl = this.zs(o.plan, x);
    if (pl === null) return;
    const half = Math.min(-pl[0], pl[1]);
    const zc = Math.max(0, half - r[2] * (o.inset ?? 0.92));
    for (const [i, sd] of ([-1, 1] as const).entries()) this.blob(`${sd < 0 ? "near" : "far"}-${id}`, o.bones?.[i] ?? bone, [x, y, sd * zc], r, look, o.dir ?? [1, 0, 0]);
  }

  /* ------------------------------------------------------------------ birds */

  /** 🪽️ Folded wing of a perched bird lying on the flank, built from the scale's own elevation outline of the wing (`elev` shape ids) and its drawn lateral position: coverts and arm as a thick lens that keeps the drawn thickness `th0` (leading edge forward, tapering trailing edge), a carpal knob at the front, then the hand: secondaries and the long primaries as thinner sheets (`thh` at the wrist, `th1` at the tip) ending in a point, a shorter stepped primary group beside them and tertials over the rump.
   * `sd` -1 near / +1 far; `x0` shoulder x, `xj` wrist (hand bone origin) x, `x1` tip x; `zc` lateral centre of the arm, `zh` of the hand; `bones` [arm, hand]. */
  wing(sd: -1 | 1, o: { elev: readonly string[]; x0: number; xj: number; x1: number; zc: number; zh: number; th0: number; thh: number; th1: number; xc?: number; xp?: number; ec?: number; elevC?: readonly string[]; vfixC?: [number, number]; bones: [string, string]; id: string; lod: Lod; look: Look; tip: Look; roll?: number }): void {
    const [arm, hand] = o.bones;
    const roll = (o.roll ?? 0.1) * -sd;
    const f = this.feature * 0.55;
    const xt = o.x1 + 0.0015;
    const mk = (id: string, bone: string, xa: number, xb: number, n: number, band: [number, number], th: (x: number) => number, zc: (x: number) => number, e: number, look: Look, ids: readonly string[] = o.elev, vfix?: [number, number]): void => {
      const pts: PP[] = [];
      for (const x of range(xa, xb, n)) {
        let sp = this.ys(ids, x);
        if (sp === null) continue;
        if (vfix !== undefined) sp = vfix;
        const h = sp[1] - sp[0];
        const lo = sp[0] + band[0] * h;
        const hi = sp[0] + band[1] * h;
        const end = clamp((x - o.x1) / 0.05, 0.12, 1);
        pts.push({ p: [x, (lo + hi) / 2, sd * zc(x)], rl: Math.max(f, th(x) * (x < o.xj ? 1 : Math.sqrt(end))), rs: Math.max(f, ((hi - lo) / 2) * (x < o.x1 + 0.02 ? end : 1)), e, roll });
      }
      if (pts.length >= 2 && xb <= o.x1 + 0.003) { const q = pts[pts.length - 1]!; pts.push({ p: [o.x1, q.p[1], q.p[2]], rl: f, rs: f, e, roll }); }
      if (pts.length >= 2) this.path(id, bone, pts, look);
    };
    const A = (x: number) => lerp(o.th0, o.thh, smooth((o.x0 - x) / (o.x0 - o.xj)) * 0.6);
    const H = (x: number) => lerp(o.thh, o.th1, Math.pow(clamp((o.xj - x) / (o.xj - o.x1), 0, 1), 0.8));
    const n = o.lod === 2 ? 9 : o.lod === 1 ? 7 : 5;
    mk(`${o.id}-coverts`, arm, o.x0, o.xc ?? o.xj - 0.03, n, [0, 1], A, () => o.zc, o.ec ?? 3.1, o.look, o.elevC ?? o.elev, o.vfixC);
    mk(`${o.id}-primaries`, hand, o.xp ?? o.xj + 0.03, xt, n + 2, [0, 1], H, () => o.zh, 3.0, o.tip);
    if (o.lod === 2) {
      const sp = this.ys(o.elev, o.x0 - 0.012);
      if (sp !== null) this.blob(`${o.id}-carpal`, arm, [o.x0 - 0.006, sp[0] + (sp[1] - sp[0]) * 0.58, sd * o.zc], [0.018, o.th0 * 0.95, (sp[1] - sp[0]) * 0.3], o.look, [1, 0, 0]);
      mk(`${o.id}-secondaries`, arm, lerp(o.x0, o.xj, 0.45), lerp(o.xj, o.x1, 0.55), 6, [0, 0.5], (x) => H(x) * 0.95, () => o.zh - sd * 0 + 0.0, 3, o.tip);
      mk(`${o.id}-tertials`, arm, lerp(o.x0, o.xj, 0.3), lerp(o.xj, o.x1, 0.12), 5, [0, 0.62], (x) => A(x) * 0.8, () => o.zc + 0.003, 2.8, o.look);
    }
  }

  /** 🪶️ Tail fan of a bird: a flat wedge of twelve rectrices (thin loft, wide at the tip) carried by the elevation outline `elev` and the plan outline `plan`, with undertail coverts as a soft loft below the base. */
  tail(id: string, bone: string, o: { elev: readonly string[]; plan: readonly string[]; x0: number; x1: number; look: Look; thick: number; n: number; coverts?: Look; droop?: number }): void {
    const pts: PP[] = [];
    for (const x of range(o.x0, o.x1, o.n)) {
      const sp = this.ys(o.elev, x);
      const pl = this.zs(o.plan, x);
      if (sp === null || pl === null) continue;
      const t = clamp((o.x0 - x) / (o.x0 - o.x1), 0, 1);
      const h = Math.min((sp[1] - sp[0]) / 2, o.thick * lerp(1, 0.55, t));
      pts.push({ p: [x, sp[1] - h, (pl[0] + pl[1]) / 2], rl: (pl[1] - pl[0]) / 2, rs: h, e: 3.2 });
    }
    if (pts.length >= 2) { const q = pts[pts.length - 1]!; if (q.p[0] > o.x1 + 0.002) pts.push({ p: [o.x1 + 0.0005, q.p[1] - q.rs * 0.1, q.p[2]], rl: q.rl * 0.7, rs: this.feature * 0.55, e: 3.2 }); }
    this.path(id, bone, pts, o.look);
    if (o.coverts !== undefined) {
      const q: PP[] = [];
      for (const x of range(o.x0, lerp(o.x0, o.x1, 0.42), Math.max(3, Math.round(o.n / 2)))) {
        const sp = this.ys(o.elev, x);
        const pl = this.zs(o.plan, x);
        if (sp === null || pl === null) continue;
        const t = clamp((o.x0 - x) / (o.x0 - o.x1), 0, 1);
        const top = sp[1] - 2 * o.thick * lerp(1, 0.55, t);
        const h = Math.max(this.feature * 0.55, (top - sp[0]) / 2);
        q.push({ p: [x, sp[0] + h, 0], rl: ((pl[1] - pl[0]) / 2) * lerp(0.7, 0.5, t), rs: h, e: 2.2 });
      }
      if (q.length >= 2) this.path(`${id}-coverts`, bone, q, o.coverts);
    }
  }

  /** 👄️ Bill of a bird from its own elevation and plan outlines: an upper mandible with culmen and hooked tip above the gape line, a lower mandible with gonys below it, a cere/nostril pad optional.
   * `gape`: y of the gape line as a function of the fraction along the bill; `hook`: extra drop of the upper tip. */
  bill(id: string, bone: string, o: { elev: readonly string[]; plan: readonly string[]; x0: number; x1: number; look: Look; lower?: Look; n: number; gape?: (t: number) => number; lowerW?: number }): void {
    const up: PP[] = [];
    const lo: PP[] = [];
    for (const x of range(o.x0, o.x1, o.n)) {
      const sp = this.ys(o.elev, x);
      const pl = this.zs(o.plan, x);
      if (sp === null || pl === null) continue;
      const t = (x - o.x0) / (o.x1 - o.x0);
      const g = o.gape === undefined ? lerp(sp[0], sp[1], 0.42) : o.gape(t);
      const hw = (pl[1] - pl[0]) / 2;
      up.push({ p: [x, (g + sp[1]) / 2, 0], rl: hw * 0.97, rs: Math.max(this.feature * 0.55, (sp[1] - g) / 2), e: 2.2 });
      lo.push({ p: [x, (sp[0] + g) / 2 - 0.0005, 0], rl: hw * (o.lowerW ?? 0.85) * (1 - 0.15 * t), rs: Math.max(this.feature * 0.55, (g - sp[0]) / 2), e: 2.3 });
    }
    if (up.length >= 2) { const q = up[up.length - 1]!; const l = lo[lo.length - 1]!; const e = this.feature * 0.55; up.push({ p: [o.x1, q.p[1] - q.rs * 0.2, 0], rl: e, rs: e, e: 2.2 }); lo.push({ p: [o.x1 - 0.004, l.p[1] + 0.001, 0], rl: e, rs: e, e: 2.3 }); }
    this.path(`${id}-upper`, bone, up, o.look);
    if (lo.length >= 2) this.path(`${id}-lower`, bone, lo, o.lower ?? o.look);
  }

  /** 🦶️ Bird foot on the ground (y = `at[1]`): toes splayed by `spread` rad either side of the middle toe from the tarsus foot point `at`, a web sheet between the toes when `web` > 0 (fraction of the spread width), a hind toe at `lod` 2; `yaw` turns the whole foot about the vertical, `sd` is the side (-1 near). Lengths in m. */
  foot(id: string, bone: string, o: { at: V3; sd: -1 | 1; len: number; spread: number; thick: number; toeW: number; web?: number; hind?: number; look: Look; lod: Lod; yaw?: number }): void {
    const f = this.feature * 0.55;
    const y = o.at[1];
    const th = Math.max(f, o.thick / 2);
    const tw = Math.max(f, o.toeW / 2);
    const dirOf = (a: number): V3 => [Math.cos((o.yaw ?? 0) + a), 0, o.sd * Math.sin((o.yaw ?? 0) + a)];
    const flat = (d: V3): V3 => unit(crs([0, 1, 0], d));
    const base: V3 = [o.at[0], y, o.at[2]];
    const mid = dirOf(0);
    const half = o.len * Math.sin(o.spread) * 0.9;
    if ((o.web ?? 0) > 0) {
      const ks = [0.04, 0.4, 0.68, 0.93];
      const ws = [0.3, 0.78, 1, 0.45];
      this.path(`${id}-web`, bone, ks.map((k, i) => ({ p: add(base, mul(mid, o.len * k)), rl: Math.max(th, ws[i]! * half * (o.web ?? 1)), rs: th, e: 3.6 })), o.look, { lat: flat(mid) });
    }
    const angles = o.lod === 2 ? [-o.spread, 0, o.spread] : [-o.spread, o.spread];
    for (const [i, a] of angles.entries()) {
      const d = dirOf(a);
      const l = o.len * (a === 0 ? 1 : 0.9);
      this.path(`${id}-toe-${i + 1}`, bone, [
        { p: base, rl: tw, rs: th, e: 2.4 },
        { p: add(base, mul(d, l * 0.55)), rl: tw, rs: th, e: 2.4 },
        { p: add(base, mul(d, l)), rl: tw * 0.55, rs: th * 0.85, e: 2.4 },
      ], o.look, { lat: flat(d) });
    }
    if ((o.hind ?? 0) > 0 && o.lod === 2) this.path(`${id}-hind`, bone, [{ p: base, rl: tw, rs: th }, { p: add(base, mul(dirOf(Math.PI), o.hind!)), rl: tw * 0.7, rs: th * 0.85 }], o.look, { lat: [0, 0, 1] });
  }

  /** 👁️ Eye with orbital ring: a bright ring ellipsoid and a dark eyeball standing proud of the head. */
  eye(id: string, bone: string, at: V3, r: number, sd: -1 | 1, ring: Look, ball: Look): void {
    this.blob(`${id}-ring`, bone, at, [r * 1.25, r * 0.7, r * 1.25], ring, [0, 0, sd]);
    this.blob(id, bone, add(at, [0, 0, sd * r * 0.28]), [r * 0.85, r * 0.7, r * 0.85], ball, [0, 0, sd]);
  }
}
