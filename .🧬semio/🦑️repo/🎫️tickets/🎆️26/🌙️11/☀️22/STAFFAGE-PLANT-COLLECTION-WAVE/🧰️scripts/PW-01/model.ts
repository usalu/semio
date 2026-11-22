/** 🌳️ World-space model of a PW-01 maple per scale: wood as swept tubes (root flare, buttress roots, stem, leaders, scaffold limbs, side branches) and foliage as ellipsoid lobes on rings of the crown envelope, every number taken from the species design file; plus the planar botanical rig that carries them.
 * x ahead, y up, z lateral (left +), metres. The forms are stated per scale (rings, counts, sizes), never derived from a finer scale.
 * @see ./species.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts */
import { D2R, add, dirOf, mul, sub, unit, type Lobe, type Tube } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts";
import type { V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { rng } from "./geom.ts";

export type Text = { en: string; de: string };
export type Scale = 50 | 100 | 200 | 500;
export const SCALES: Scale[] = [50, 100, 200, 500];
export const featureOf = (s: number): number => (0.15 * s) / 1000;

/** 🪵️ A stem or leader: its world centreline stations [x, y, z, radius] at 1:50; coarser scales keep every second station but the ends. */
export type AxisSpec = { id: string; parent: string; title: Text; limit: number; region: "trunk-bark" | "limb-bark"; rows: [number, number, number, number][] };
/** 🌿️ A scaffold limb leaving its parent axis at height `at`: azimuth (degrees from +x towards +z), first and last elevation, length, first and last radius, azimuth bend over its length. */
export type LimbSpec = { id: string; parent: string; title: Text; at: number; az: number; e0: number; e1: number; L: number; r0: number; r1: number; bend: number; limit: number; sides: SideSpec[] };
/** 🌱️ A side branch at fraction `f` of its limb: azimuth relative to the limb, elevations, length, radii, bend. */
export type SideSpec = { f: number; daz: number; e0: number; e1: number; L: number; r0: number; r1: number; bend: number };
/** 🟢️ A ring of crown lobes at height fraction `y` (0 crown base, 1 top): `n` lobes, size `s` (fraction of the widest crown radius), droop `tilt`, reach `k` (share of the envelope radius their outer pole touches). */
export type RingSpec = { y: number; n: number; s: number; tilt: number; k: number; flat?: number };
/** 🌑️ A ring of shade lobes of the inner crown at height fraction `y`, reach share `k`, size `s`. */
export type ShadeSpec = { y: number; n: number; k: number; s: number };

export type CrownSpec = { base: number; widest: number; top: number; rx: number; rz: number; lowerExp: number; upperP: number; upperQ: number; lumps: [number, number, number][]; cx: number };

/** 🫧️ The shaded core of the crown: `n` stacked ellipsoids between the height fractions `from` and `to`, their horizontal radii `k` times the envelope, so no line of sight passes through the crown. */
export type CoreSpec = { n: number; from: number; to: number; k: number };
export type ScaleCrown = { rings: RingSpec[]; shade: ShadeSpec[]; core: CoreSpec; cap: [number, number]; tufts: number; jitter: number };

export type Species = {
  id: string; seed: number; title: Text; summary: Text; distinction: Text; tags: string[]; notes: Text;
  axes: AxisSpec[]; limbs: LimbSpec[]; buttress: [number, number][]; flare: [number, number][];
  crown: CrownSpec; scales: Record<Scale, ScaleCrown>;
  tone: { root: number; stem: number; limb: number; sun: number; shade: number };
};

export type Model = { scale: Scale; wood: Tube[]; lobes: Lobe[]; foliageBones: string[] };

/** 🥚️ The crown envelope radius at height y in the azimuth az. */
export function envelope(c: CrownSpec, y: number, az: number): number {
  const base = 1 / Math.hypot(Math.cos(az * D2R) / c.rx, Math.sin(az * D2R) / c.rz);
  const lump = 1 + c.lumps.reduce((s, [k, a, ph]) => s + a * Math.cos(k * az * D2R + ph), 0);
  const prof = y >= c.widest ? Math.max(0, 1 - ((y - c.widest) / (c.top - c.widest)) ** c.upperP) ** c.upperQ : Math.max(0, 1 - ((c.widest - y) / (c.widest - c.base)) ** c.lowerExp) ** 0.5;
  return base * lump * prof;
}

/** 📍️ The centreline point of a tube at the world height y (clamped to its ends). */
function atHeight(t: Tube, y: number): V3 {
  const pts = t.points;
  for (let i = 0; i + 1 < pts.length; i++) {
    const [a, b] = [pts[i]!.p, pts[i + 1]!.p];
    if (y <= b[1] || i + 2 === pts.length) {
      const f = Math.max(0, Math.min(1, (y - a[1]) / (b[1] - a[1] || 1e-9)));
      return add(a, mul(sub(b, a), f));
    }
  }
  return pts[0]!.p;
}

const radiusAt = (t: Tube, y: number): number => {
  const pts = t.points;
  for (let i = 0; i + 1 < pts.length; i++) if (y <= pts[i + 1]!.p[1]) {
    const f = Math.max(0, Math.min(1, (y - pts[i]!.p[1]) / (pts[i + 1]!.p[1] - pts[i]!.p[1] || 1e-9)));
    return pts[i]!.r + (pts[i + 1]!.r - pts[i]!.r) * f;
  }
  return pts[pts.length - 1]!.r;
};

/** 🌱️ A tube from `start` with azimuth az and elevation turning from e0 to e1 (and azimuth by `bend`) over length L, radius r0 to r1, in n steps. */
function sweep(start: V3, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number, n: number): { p: V3; r: number }[] {
  const pts: { p: V3; r: number }[] = [{ p: start, r: r0 }];
  let p = start;
  for (let i = 1; i <= n; i++) {
    const s = (i - 0.5) / n;
    p = add(p, mul(dirOf(az + bend * s, e0 + (e1 - e0) * s), L / n));
    pts.push({ p, r: r0 + (r1 - r0) * (i / n) ** 0.85 });
  }
  return pts;
}

/** 🪵️ The wood of one scale. Coarser scales thin the stations, keep the radii above the smallest feature and drop side branches (1:200, 1:500). */
export function woodOf(sp: Species, scale: Scale): Tube[] {
  const fmin = featureOf(scale) * 0.62;
  const keep = <T>(rows: T[]): T[] => (scale === 50 ? rows : scale === 100 ? rows.filter((_, i) => i === 0 || i === rows.length - 1 || i % 2 === 0) : rows.filter((_, i) => i === 0 || i === rows.length - 1 || i % 3 === 0));
  const tubes: Tube[] = [];
  const flareRows = keep(sp.flare);
  tubes.push({ kind: "tube", id: "root-plate", bone: "root-plate", material: "bark", tone: sp.tone.root, group: "root", points: flareRows.map(([y, r]) => ({ p: [sp.axes[0]!.rows[0]![0], y, sp.axes[0]!.rows[0]![2]] as V3, r })) });
  if (scale !== 500) sp.buttress.forEach(([az, reach], i) => {
    const d = (rr: number, y: number): V3 => [rr * Math.cos(az * D2R), y, rr * Math.sin(az * D2R)];
    const r0 = sp.flare[0]![1];
    const rows: [number, number, number][] = scale === 50 ? [[0.55, 0.9, 0.16], [0.7, 0.55, 0.2], [0.9, 0.32, 0.17], [1.15, 0.16, 0.12], [1.45, 0.07, 0.07], [1.75 * reach, 0.03, 0.035]] : [[0.6, 0.8, 0.17], [0.95, 0.3, 0.16], [1.45, 0.08, 0.08], [1.7 * reach, 0.035, 0.04]];
    tubes.push({ kind: "tube", id: `root-${i + 1}`, bone: "root-plate", material: "bark", tone: sp.tone.root, group: "root", points: rows.map(([f, y, r]) => ({ p: d(f * r0, y * r0), r: Math.max(fmin, r * r0) })) });
  });
  const byId: Record<string, Tube> = {};
  for (const a of sp.axes) {
    const rows = keep(a.rows);
    const t: Tube = { kind: "tube", id: a.id, bone: a.id, material: a.region === "trunk-bark" ? "bark" : "young", tone: a.region === "trunk-bark" ? sp.tone.stem : sp.tone.limb, group: a.region === "trunk-bark" ? "stem" : "limb", points: rows.map(([x, y, z, r]) => ({ p: [x, y, z] as V3, r: Math.max(fmin, r) })) };
    tubes.push(t);
    byId[a.id] = t;
  }
  for (const l of sp.limbs) {
    const parent = byId[l.parent]!;
    const start = atHeight(parent, l.at);
    const n = scale === 50 ? 7 : scale === 100 ? 5 : scale === 200 ? 4 : 3;
    const t: Tube = { kind: "tube", id: l.id, bone: l.id, material: "young", tone: sp.tone.limb, group: "limb", points: sweep(start, l.az, l.e0, l.e1, l.L, Math.min(l.r0, radiusAt(parent, l.at) * 0.92), Math.max(fmin, l.r1), l.bend, n).map((q) => ({ p: q.p, r: Math.max(fmin, q.r) })) };
    tubes.push(t);
    byId[l.id] = t;
    if (scale === 50 || scale === 100) l.sides.forEach((s, i) => {
      const pts = t.points;
      const x = s.f * (pts.length - 1);
      const k = Math.min(pts.length - 2, Math.floor(x));
      const p0 = add(pts[k]!.p, mul(sub(pts[k + 1]!.p, pts[k]!.p), x - k));
      const rr = pts[k]!.r + (pts[k + 1]!.r - pts[k]!.r) * (x - k);
      const m = scale === 50 ? 5 : 3;
      const thick = scale === 100 ? 1.15 : 1;
      tubes.push({ kind: "tube", id: `${l.id}-side-${i + 1}`, bone: l.id, material: "young", tone: sp.tone.limb, group: "side", points: sweep(p0, l.az + s.daz, s.e0, s.e1, s.L, Math.min(s.r0 * thick, rr * 0.9), Math.max(fmin, s.r1 * thick), s.bend, m).map((q) => ({ p: q.p, r: Math.max(fmin, q.r) })) });
    });
  }
  return tubes;
}

/** 🍃️ The tip of a limb at 1:50 (the origin of its foliage bone). */
export function limbTip(sp: Species, id: string): V3 {
  const t = woodOf(sp, 50).find((x) => x.id === id)!;
  return t.points[t.points.length - 1]!.p;
}

/** 🟢️ An ellipsoid lobe whose outer pole touches the radius `reach` at azimuth az and height y; a radial, b vertical, c tangential semi-axis, `tilt` droops the radial axis outward. */
function lobe(id: string, group: string, az: number, reach: number, y: number, a: number, b: number, c: number, tilt: number, cx: number, material: string, tone: number): Lobe {
  const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const up: V3 = [0, 1, 0];
  const tan: V3 = [-Math.sin(az * D2R), 0, Math.cos(az * D2R)];
  const [ct, st] = [Math.cos(tilt * D2R), Math.sin(tilt * D2R)];
  const r1 = sub(mul(rad, ct), mul(up, st));
  const u1 = add(mul(up, ct), mul(rad, st));
  const extent = Math.hypot(a * ct, b * st);
  const rc = Math.max(0, reach - extent);
  return { kind: "lobe", id, bone: "", material, tone, group, c: [cx + rc * rad[0], y, rc * rad[2]], axes: [r1, u1, tan], radii: [a, b, c] };
}

/** 🌳️ The foliage of one scale: rings of sun lobes on the envelope, the top cap, shade lobes of the inner crown and, at 1:50, small leaf tufts standing proud of the lobes. Bones: the nearest foliage bone (limb tips and the top), shade lobes on the two core bones; at 1:500 every lobe rides on the first leader. */
export function foliageOf(sp: Species, scale: Scale, foliageOrigins: Record<string, V3>): Lobe[] {
  const c = sp.crown;
  const sc = sp.scales[scale];
  const rand = rng(sp.seed * 7919 + scale);
  const Rmax = Math.max(c.rx, c.rz);
  const H = c.top - c.base;
  const out: Lobe[] = [];
  sc.rings.forEach((ring, ri) => {
    const phase = rand() * 360;
    for (let i = 0; i < ring.n; i++) {
      const az = phase + (360 * (i + (rand() - 0.5) * sc.jitter)) / ring.n;
      const y0 = c.base + ring.y * H;
      const size = ring.s * Rmax * (1 + (rand() - 0.5) * 0.3 * sc.jitter);
      const a = size * 0.92, b = size * (ring.flat ?? 0.78), cc = size * (1.05 + (rand() - 0.5) * 0.2);
      const y = Math.min(c.top - b * 0.95, y0 + (rand() - 0.5) * 0.25 * b * sc.jitter);
      const reach = envelope(c, y, az) * (ring.k - rand() * 0.06 * sc.jitter);
      out.push(lobe(`r${ri + 1}-${i + 1}`, "sun", az, reach, y, a, b, cc, ring.tilt, c.cx, "sun", sp.tone.sun));
    }
  });
  const [capA, capB] = sc.cap;
  out.push({ kind: "lobe", id: "cap", bone: "", material: "sun", tone: sp.tone.sun, group: "sun", c: [c.cx, c.top - capB * Rmax, 0], axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii: [capA * Rmax, capB * Rmax, capA * Rmax * 0.97] });
  {
    const { n, from, to, k } = sc.core;
    const step = ((to - from) * H) / Math.max(1, n - 1);
    for (let i = 0; i < n; i++) {
      const y = c.base + from * H + i * step;
      const h = Math.min(step * 0.78, y - c.base - 0.3, c.top - y - 0.6);
      out.push({ kind: "lobe", id: `core-${i + 1}`, bone: "", material: "shade", tone: sp.tone.shade, group: "shade", c: [c.cx, y, 0], axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii: [k * Math.min(envelope(c, y - h * 0.5, 0), envelope(c, y + h * 0.5, 0), envelope(c, y, 180)), Math.max(h, 0.3), k * Math.min(envelope(c, y - h * 0.5, 90), envelope(c, y + h * 0.5, 90), envelope(c, y, 270))] });
    }
  }
  sc.shade.forEach((ring, ri) => {
    const phase = rand() * 360;
    for (let i = 0; i < ring.n; i++) {
      const az = phase + (360 * i) / ring.n;
      const y = c.base + ring.y * H;
      const size = ring.s * Rmax;
      out.push(lobe(`s${ri + 1}-${i + 1}`, "shade", az, envelope(c, y, az) * ring.k, y, size, size * 0.8, size * 1.05, 0, c.cx, "shade", sp.tone.shade));
    }
  });
  if (sc.tufts > 0) {
    const parents = out.filter((l) => l.group === "sun" && l.id !== "cap");
    parents.forEach((l, i) => {
      if (rand() > sc.tufts) return;
      const azl = Math.atan2(l.c[2], l.c[0] - c.cx) / D2R + (rand() - 0.5) * 50;
      const el = 10 + rand() * 50;
      const d = dirOf(azl, el);
      const q = Math.hypot(...l.axes.map((ax, k) => (ax[0] * d[0] + ax[1] * d[1] + ax[2] * d[2]) / l.radii[k]!));
      const surf = add(l.c, mul(d, 1 / q));
      const r = 0.16 + rand() * 0.14;
      const ce = sub(surf, mul(d, r - 0.08));
      const reach = envelope(c, ce[1], azl);
      const hx = ce[0] - c.cx, hz = ce[2];
      const far = Math.hypot(hx, hz);
      const most = reach - r - 0.004;
      const at: V3 = far > most && far > 0 ? [c.cx + (hx * most) / far, Math.min(ce[1], c.top - r), (hz * most) / far] : [ce[0], Math.min(ce[1], c.top - r), ce[2]];
      out.push({ kind: "lobe", id: `t-${l.id}`, bone: "", material: "sun", tone: sp.tone.sun, group: "tuft", c: at, axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii: [r, r * 0.85, r * 1.1] });
      void i;
    });
  }
  const suns = Object.entries(foliageOrigins).filter(([id]) => !id.startsWith("fol-core"));
  for (const l of out) {
    if (scale === 500) l.bone = sp.axes[1]!.id;
    else if (l.group === "shade") l.bone = l.c[1] < c.base + 0.5 * H ? "fol-core-low" : "fol-core-high";
    else {
      let best = Infinity;
      for (const [id, o] of suns) {
        const d = Math.hypot(l.c[0] - o[0], (l.c[1] - o[1]) * 1.25, l.c[2] - o[2]);
        if (d < best) [best, l.bone] = [d, id];
      }
    }
  }
  return out;
}

/** 🔁️ Unit vectors and the world angle (degrees) of the planar projection of a direction. */
export const planarAngle = (d: V3): number => Math.atan2(d[1], d[0]) / D2R;
export { unit };
