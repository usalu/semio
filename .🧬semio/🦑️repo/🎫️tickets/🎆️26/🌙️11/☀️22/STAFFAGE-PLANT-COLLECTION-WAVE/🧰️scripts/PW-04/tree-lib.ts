/** 🌳️ PW-04 tree kit: expands the hand-chosen design tables of one street tree (stem stations, scaffold limbs, side branches, foliage clusters, poses) into a `semio.staffage/v2` asset.
 *
 * The rig is sagittal (every bone turns about the world z axis, lateral placement by `Bone.z`), so a wind sway in the x-y plane never turns a drawing plane out of the paper. Every part is one of three primitives in world rest coordinates: a capsule (stem, limbs, branches), an axis-aligned ellipsoid (foliage lobes) and a revolved root flare. The 3D solids are these primitives in the bone frame, the 2D shapes their exact orthographic projections (capsule → stadium, ellipsoid → ellipse, revolve → profile and disc) or the union of those projections, so drawing and model agree by construction.
 *
 * @see ../../📋️brief.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";

export type Text = { en: string; de: string };
export type V3 = [number, number, number];
export type P2 = { x: number; y: number };
export type Json = any;
export type View = "elevation" | "front" | "plan";
export type Scale = 50 | 100 | 200 | 500;

export const SCALES: readonly Scale[] = [500, 200, 100, 50];
export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
export const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
export const RHO_AIR = 1.25;
export const G = 9.80665;

/** 🔤️ A bilingual text. */
export const T = (en: string, de: string): Text => ({ en, de });

/** 🔢️ A number rounded to `d` decimals, negative zero folded. */
export const r = (x: number, d = 4): number => {
  const v = Math.round(x * 10 ** d) / 10 ** d;
  return Object.is(v, -0) ? 0 : v;
};

/** 🇩🇪️ A decimal number with a comma for German texts. */
export const de = (x: number, d = 1): string => x.toFixed(d).replace(".", ",");

/** 🧭️ A unit direction from an azimuth (degrees from +x towards +z, the left) and an elevation (degrees above horizontal). */
export function dir(azimuthDeg: number, elevationDeg: number): V3 {
  const a = (azimuthDeg * Math.PI) / 180;
  const e = (elevationDeg * Math.PI) / 180;
  return [Math.cos(e) * Math.cos(a), Math.sin(e), Math.cos(e) * Math.sin(a)];
}

export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const lerp = (a: V3, b: V3, t: number): V3 => [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];

/** 🎲️ A deterministic pseudo-random sequence (mulberry32) for the hand-seeded lobe and mark layouts. */
export function rng(seed: number): () => number {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** 🪵️ A woody segment that is a bone: from `a` to `b` in world rest coordinates, radius `r0` at `a` and `r1` at `b`. `level` 0 stem, 1 leader, 2 scaffold limb, 3 limb extension; it sets sway lag and joint limits. */
export type WoodBone = { id: string; parent: string | null; a: V3; b: V3; r0: number; r1: number; level: 0 | 1 | 2 | 3; title: Text; region: "stem-bark" | "limb-bark"; scales?: readonly Scale[] };

/** 🍃️ A foliage bone at the end of a limb: a short sagittal bone that carries the clusters of that limb system. */
export type FoliageBone = { id: string; parent: string; a: V3; b: V3; title: Text };

/** 🌿️ A side branch without a bone of its own, carried by `host`, drawn and modelled only at the scales listed. */
export type Branch = { id: string; host: string; a: V3; b: V3; r0: number; r1: number; scales: readonly Scale[] };

/** ☁️ A foliage cluster: an ellipsoid of world semi-axes `rad` about `c`, carried by `host`; at 1:100 and 1:50 it is a core with lobes, at 1:500 it may be merged into its `mass`. */
export type Cluster = { id: string; host: string; c: V3; rad: V3; region: "leaf-sun" | "leaf-shade"; mass: string; seed: number };

/** 📐️ How the clusters break into lobes at one scale: `n` lobes, core factor, lobe centre factor `k` and lobe size `s` (reach `k + s` of the cluster radius), lobes facing down below `minUp` are left out. */
export type LobeRule = { n: number; core: number; coreSide: number; k: number; s: number; minUp: number };

/** 🗒️ Everything a species states by hand. */
export type Species = {
  id: string;
  title: Text;
  summary: Text;
  distinction: Text;
  tags: string[];
  provenance: Json;
  materials: Json[];
  regions: { stem: Json; limb: Json; sun: Json; shade: Json };
  assumptions: Text[];
  structureAssumptions: Text[];
  environment: Json;
  densityKgM3: number;
  leafAreaM2: { sun: number; shade: number };
  leafFreshKgM2: number;
  rootFlare: { profile: [number, number][]; lengthM: number };
  wood: WoodBone[];
  foliage: FoliageBone[];
  branches: Branch[];
  clusters: Cluster[];
  masses500: { id: string; host: string; members: string[]; shrink: V3 }[];
  lobes: { 100: LobeRule; 50: LobeRule };
  marks: { kind: "fan" | "pinnate" | "star"; size: number; spacing: number };
  bark: { kind: "furrow" | "plate" | "ridge"; lines: number; seed: number };
  sway: { periodS: number; breezeMS: number; galeMS: number; gustPeak: number; lean: Record<0 | 1 | 2 | 3 | 4, number> };
  cd: { breeze: number; gale: number; stem: number };
  poses: PoseSpec[];
  texts: SpeciesTexts;
};

/** 🧍️ A named pose beyond the computed wind state: thermal binding scales, lean factors and loads. */
export type PoseSpec = { id: string; title: Text; description: Text; windMS: number; leanFactor: number; snowKgM2?: number; sun: Record<string, number>; wet?: number; emitters: { sun: number; shade: number } };

/** 📝️ The species words that the decisions of every scale reuse. */
export type SpeciesTexts = { crown: Text; stem: Text; limbs: Text; leaf: Text; bark: Text; omitted500: Text; omitted200: Text };

/** 🧱️ One primitive of one scale in world rest coordinates. */
export type Prim =
  | { kind: "capsule"; id: string; host: string; a: V3; b: V3; r0: number; r1: number; region: string; material: string; tone: number; slender?: boolean }
  | { kind: "ellipsoid"; id: string; host: string; c: V3; rad: V3; axes?: [V3, V3, V3]; region: string; material: string; tone: number; cluster?: string }
  | { kind: "revolve"; id: string; host: string; profile: [number, number][]; region: string; material: string; tone: number };

/** 🦴️ A rest frame of a sagittal bone: origin and the angle of `u` in the x-y plane. */
export type Frame = { o: V3; angle: number };

/** 🧮️ The bones of a species in rig order with their rest frames. */
export function frames(sp: Species): Map<string, Frame> {
  const m = new Map<string, Frame>();
  m.set("root-plate", { o: [0, 0, 0], angle: Math.PI / 2 });
  for (const b of [...sp.wood, ...sp.foliage]) m.set(b.id, { o: b.a, angle: Math.atan2(b.b[1] - b.a[1], b.b[0] - b.a[0]) });
  return m;
}

/** 🔁️ A world point in the bone frame (u along the bone, v its normal, w lateral). */
export function toLocal(f: Frame, p: V3): V3 {
  const d = sub(p, f.o);
  const c = Math.cos(f.angle);
  const s = Math.sin(f.angle);
  return [d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2]];
}

/** 📍️ A world point (x, y) in the 2D frame of the elevation shapes of a bone. */
export const elevLocal = (f: Frame, p: P2): P2 => {
  const l = toLocal(f, [p.x, p.y, f.o[2]]);
  return { x: l[0], y: l[1] };
};

/** 🎥️ The paper coordinates of a world point in a view: elevation (x, y), front (z, y), plan (x, z). */
export const project = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });

/** 👁️ How near a world point is to the viewer of a view (larger is nearer). */
export const nearness = (view: View, p: V3): number => (view === "elevation" ? -p[2] : view === "front" ? p[0] : p[1]);

/** ⭕️ An ellipse as a ring. */
export function ellipseRing(cx: number, cy: number, rx: number, ry: number, n = 40): P2[] {
  return Array.from({ length: n }, (_, i) => ({ x: cx + rx * Math.cos((2 * Math.PI * i) / n), y: cy + ry * Math.sin((2 * Math.PI * i) / n) }));
}

/** 💊️ The tangent hull of two circles as a ring. */
export function capsuleRing(x0: number, y0: number, x1: number, y1: number, r0: number, r1: number, n = 16): P2[] {
  const d = Math.hypot(x1 - x0, y1 - y0);
  if (d < 1e-6 || d + Math.min(r0, r1) <= Math.max(r0, r1)) return r0 >= r1 ? ellipseRing(x0, y0, r0, r0, 2 * n) : ellipseRing(x1, y1, r1, r1, 2 * n);
  const th = Math.atan2(y1 - y0, x1 - x0);
  const phi = Math.asin((r0 - r1) / d);
  const out: P2[] = [];
  for (let i = 0; i <= n; i++) {
    const a = th + Math.PI / 2 + phi + ((Math.PI - 2 * phi) * i) / n;
    out.push({ x: x0 + r0 * Math.cos(a), y: y0 + r0 * Math.sin(a) });
  }
  for (let i = 0; i <= n; i++) {
    const a = th - Math.PI / 2 - phi + ((Math.PI + 2 * phi) * i) / n;
    out.push({ x: x1 + r1 * Math.cos(a), y: y1 + r1 * Math.sin(a) });
  }
  return out;
}

/** 🧭️ The unit world axes of an ellipsoid primitive (the world axes unless it is turned). */
export const axesOf = (p: { axes?: [V3, V3, V3] }): [V3, V3, V3] => p.axes ?? [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

/** 🥚️ The outline ellipse of an ellipsoid in a view: centre, semi-axes and the angle of the first semi-axis in degrees, from the covariance of the projected semi-axes. */
export function projectedEllipse(view: View, p: { c: V3; rad: V3; axes?: [V3, V3, V3] }): { cx: number; cy: number; a: number; b: number; angleDeg: number } {
  const ax = axesOf(p).map((e, k) => project(view, mul(e, p.rad[k]!)));
  const [sxx, sxy, syy] = [ax.reduce((s, q) => s + q.x * q.x, 0), ax.reduce((s, q) => s + q.x * q.y, 0), ax.reduce((s, q) => s + q.y * q.y, 0)];
  const tr = (sxx + syy) / 2;
  const disc = Math.sqrt(Math.max(0, ((sxx - syy) / 2) ** 2 + sxy * sxy));
  const angle = 0.5 * Math.atan2(2 * sxy, sxx - syy);
  const c = project(view, p.c);
  return { cx: c.x, cy: c.y, a: Math.sqrt(tr + disc), b: Math.sqrt(Math.max(1e-12, tr - disc)), angleDeg: (angle * 180) / Math.PI };
}

/** 🗺️ The projection of a primitive in a view as one ring in paper (world) coordinates. */
export function primRing(view: View, p: Prim): P2[] {
  if (p.kind === "capsule") {
    const [a, b] = [project(view, p.a), project(view, p.b)];
    return capsuleRing(a.x, a.y, b.x, b.y, p.r0, p.r1);
  }
  if (p.kind === "ellipsoid") {
    const e = projectedEllipse(view, p);
    const t = (e.angleDeg * Math.PI) / 180;
    return ellipseRing(0, 0, e.a, e.b, 48).map((q) => ({ x: e.cx + q.x * Math.cos(t) - q.y * Math.sin(t), y: e.cy + q.x * Math.sin(t) + q.y * Math.cos(t) }));
  }
  if (view === "plan") {
    const rm = Math.max(...p.profile.map((q) => q[1]));
    return ellipseRing(0, 0, rm, rm, 48);
  }
  return [...p.profile.map(([h, rr]) => ({ x: rr, y: h })), ...[...p.profile].reverse().map(([h, rr]) => ({ x: -rr, y: h }))];
}

/** 👁️ The nearest point of a primitive in a view, for the authored paint order. */
export function primNear(view: View, p: Prim): number {
  if (p.kind === "capsule") return Math.max(nearness(view, p.a) + p.r0, nearness(view, p.b) + p.r1);
  if (p.kind === "ellipsoid") {
    const k = view === "elevation" ? 2 : view === "front" ? 0 : 1;
    const reach = Math.sqrt(axesOf(p).reduce((s, e, i) => s + (e[k]! * p.rad[i]!) ** 2, 0));
    return nearness(view, p.c) + (view === "elevation" ? reach : reach);
  }
  const rm = Math.max(...p.profile.map((q) => q[1]));
  return view === "plan" ? Math.max(...p.profile.map((q) => q[0])) : rm;
}

/** 📏️ Signed area of a ring. */
export const ringArea = (ring: readonly P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;

/** 🫧️ The union of rings (outer rings counter-clockwise, holes clockwise), sliver holes below `minHole` square metres dropped. */
export function union(rings: readonly P2[][], minHole = 0): P2[][] {
  const out = unionPolygons(rings) as P2[][];
  return out.filter((q) => ringArea(q) > 0 || -ringArea(q) >= minHole);
}

/** 〰️ An SVG path of rings in a 2D frame. */
export const pathOf = (rings: readonly P2[][]): string => rings.map((q) => `M ${q.map((p) => `${r(p.x)} ${r(p.y)}`).join(" L ")} Z`).join(" ");

/** 📦️ The box of points. */
export function boxOf(points: readonly P2[]): { minX: number; maxX: number; minY: number; maxY: number } {
  return points.reduce((b, p) => ({ minX: Math.min(b.minX, p.x), maxX: Math.max(b.maxX, p.x), minY: Math.min(b.minY, p.y), maxY: Math.max(b.maxY, p.y) }), { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity });
}

/** 🎯️ Whether a point lies inside a ring (even-odd). */
export function inRing(p: P2, ring: readonly P2[]): boolean {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c;
  }
  return c;
}

/** 📐️ The distance of a point to the boundary of rings. */
export function ringDistance(p: P2, rings: readonly P2[][]): number {
  let best = Infinity;
  for (const ring of rings)
    for (let i = 0; i < ring.length; i++) {
      const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
      const [dx, dy] = [b.x - a.x, b.y - a.y];
      const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
      best = Math.min(best, Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy));
    }
  return best;
}

const woodRegionOf = (b: WoodBone): string => b.region;

/** 🧾️ The primitives of one scale: stem, leader and limbs as tapered capsules, the root flare, the side branches of that scale and the foliage lobes of the scale's rule. */
export function primsOf(sp: Species, scale: Scale): Prim[] {
  const f = FEATURE[scale];
  const prims: Prim[] = [{ kind: "revolve", id: "root-flare", host: "root-plate", profile: sp.rootFlare.profile, region: "stem-bark", material: `${sp.id}-bark`, tone: 0.28 }];
  const minR = scale === 500 ? 0.05 : scale === 200 ? 0.035 : f / 2;
  for (const b of sp.wood) {
    if (b.scales !== undefined && !b.scales.includes(scale)) continue;
    const r0 = Math.max(minR, b.r0);
    const r1 = Math.max(minR, b.r1);
    prims.push({ kind: "capsule", id: b.id, host: b.id, a: b.a, b: b.b, r0, r1, region: woodRegionOf(b), material: b.region === "stem-bark" ? `${sp.id}-bark` : `${sp.id}-bark-young`, tone: b.level <= 1 ? 0.28 : 0.34 });
  }
  for (const br of sp.branches) {
    if (!br.scales.includes(scale)) continue;
    prims.push({ kind: "capsule", id: br.id, host: br.host, a: br.a, b: br.b, r0: Math.max(minR, br.r0), r1: Math.max(minR, br.r1), region: "limb-bark", material: `${sp.id}-bark-young`, tone: 0.36 });
  }
  const toneOf = (region: string): number => (region === "leaf-sun" ? 0.64 : 0.46);
  if (scale === 500) {
    for (const m of sp.masses500) {
      const all = sp.clusters.filter((c) => m.members.includes(c.id));
      const parts = all.length <= 2 ? [all] : Array.from({ length: Math.ceil(all.length / 2) }, (_, k) => all.slice(2 * k, 2 * k + 2));
      parts.forEach((cs, k) => {
        const region = cs[0]!.region;
        prims.push({ kind: "ellipsoid", id: parts.length === 1 ? m.id : `${m.id}-${"abcdefgh"[k]}`, host: cs[0]!.host, ...massOf(cs, m.shrink), region, material: `${sp.id}-${region}`, tone: toneOf(region), cluster: m.id });
      });
    }
    return prims;
  }
  for (const c of sp.clusters) {
    if (scale === 200) {
      prims.push({ kind: "ellipsoid", id: c.id, host: c.host, c: c.c, rad: c.rad, region: c.region, material: `${sp.id}-${c.region}`, tone: toneOf(c.region), cluster: c.id });
      continue;
    }
    const rule = sp.lobes[scale];
    prims.push({ kind: "ellipsoid", id: `${c.id}-core`, host: c.host, c: c.c, rad: [c.rad[0] * rule.coreSide, c.rad[1] * rule.core, c.rad[2] * rule.coreSide], region: c.region, material: `${sp.id}-${c.region}`, tone: toneOf(c.region), cluster: c.id });
    const rand = rng(c.seed * 7 + scale);
    const golden = Math.PI * (3 - Math.sqrt(5));
    const turn = rand() * 2 * Math.PI;
    let made = 0;
    for (let i = 0; i < rule.n * 2 && made < rule.n; i++) {
      const y = 1 - ((i + 0.5) / (rule.n * 2)) * 2;
      if (y < rule.minUp) continue;
      const rr = Math.sqrt(1 - y * y);
      const th = turn + golden * i;
      const d: V3 = [rr * Math.cos(th), y, rr * Math.sin(th)];
      const jitter = 0.9 + 0.2 * rand();
      const centre: V3 = [c.c[0] + rule.k * c.rad[0] * d[0], c.c[1] + rule.k * c.rad[1] * d[1], c.c[2] + rule.k * c.rad[2] * d[2]];
      prims.push({ kind: "ellipsoid", id: `${c.id}-l${made + 1}`, host: c.host, c: centre, rad: mul(c.rad, rule.s * jitter), region: c.region, material: `${sp.id}-${c.region}`, tone: toneOf(c.region), cluster: c.id });
      made++;
    }
  }
  return prims;
}

const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
const crossV = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const dotV = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const reachAlong = (c: { rad: V3 }, d: V3): number => 1 / Math.sqrt(d.reduce((s, x, i) => s + (x * x) / (c.rad[i]! * c.rad[i]!), 0));

/** 🫐️ The 1:500 crown mass of several clusters: one ellipsoid turned along the line through the first and the last cluster centre, long enough to reach both ends and as broad as the broadest cluster across it, scaled by `shrink`, then grown (at most 15 %) until it reaches as far ahead and sideways as its clusters, so the coarse crown keeps the true width and depth. */
export function massOf(cs: readonly Cluster[], shrink: V3): { c: V3; rad: V3; axes: [V3, V3, V3] } {
  const first = cs[0]!;
  const last = cs[cs.length - 1]!;
  const span = sub(last.c, first.c);
  const d = len(span);
  const e1: V3 = d < 0.05 ? [1, 0, 0] : unit(span);
  const e2: V3 = Math.abs(e1[1]) > 0.95 ? unit(crossV([0, 0, 1], e1)) : unit(crossV(e1, [0, 1, 0]));
  const e3 = unit(crossV(e1, e2));
  const c = mul(add(first.c, last.c), 0.5);
  const along = d / 2 + Math.max(reachAlong(first, e1), reachAlong(last, e1));
  const side = (e: V3) => Math.max(...cs.map((q) => reachAlong(q, e) + Math.abs(dotV(sub(q.c, c), e))));
  const rad: V3 = [along * shrink[0], side(e2) * shrink[1], side(e3) * shrink[2]];
  const axes: [V3, V3, V3] = [e1, e2, e3];
  const half = (k: number) => Math.sqrt(axes.reduce((s, e, i) => s + (e[k]! * rad[i]!) ** 2, 0));
  const need = (k: number) => Math.max(...cs.map((q) => Math.abs(q.c[k]! - c[k]!) + q.rad[k]!));
  const grow = Math.min(1.15, Math.max(1, need(0) / half(0), need(2) / half(2)));
  return { c, rad: rad.map((x, i) => (Math.abs(axes[i]![1]) > 0.8 ? x : x * grow)) as V3, axes };
}

/** 🔄️ A unit quaternion [x, y, z, w] of a rotation matrix given by its columns. */
export function quatOf(c0: V3, c1: V3, c2: V3): [number, number, number, number] {
  const [m00, m10, m20] = c0;
  const [m01, m11, m21] = c1;
  const [m02, m12, m22] = c2;
  const tr = m00 + m11 + m22;
  let q: [number, number, number, number];
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4];
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s];
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s];
  }
  const n = Math.hypot(...q);
  return q.map((x) => r(x / n, 6)) as [number, number, number, number];
}

/** 🌐️ A quaternion that turns the bone frame back to the world axes (rotation by `-angle` about w). */
const backQuat = (angle: number): [number, number, number, number] => [0, 0, r(Math.sin(-angle / 2), 6), r(Math.cos(-angle / 2), 6)];

/** 🧊️ The body solid of a primitive in the frame of its host bone. */
export function solidOf(p: Prim, fr: Map<string, Frame>, extra: Json = {}): Json {
  const f = fr.get(p.host)!;
  const role = "body";
  if (p.kind === "capsule") {
    const a = toLocal(f, p.a);
    const b = toLocal(f, p.b);
    const longOk = len(sub(p.b, p.a)) >= 20 * 2 * Math.max(p.r0, p.r1);
    return { id: p.id, kind: "capsule", bone: p.host, material: p.material, role, tone: p.tone, p0: a.map((x) => r(x)), p1: b.map((x) => r(x)), r0: r(p.r0), r1: r(p.r1), ...(p.slender === true && longOk ? { slender: true } : {}), ...extra };
  }
  if (p.kind === "ellipsoid") {
    const c = toLocal(f, p.c);
    const rotation = p.axes === undefined ? backQuat(f.angle) : quatOf(...(p.axes.map((e) => toLocal({ o: [0, 0, 0], angle: f.angle }, e)) as [V3, V3, V3]));
    return { id: p.id, kind: "ellipsoid", bone: p.host, material: p.material, role, tone: p.tone, radii: p.rad.map((x) => r(x)), transform: { rotation, translation: c.map((x) => r(x)) }, ...extra };
  }
  return { id: p.id, kind: "revolve", bone: p.host, material: p.material, role, tone: p.tone, profile: p.profile.map(([h, rr]) => [r(h), r(rr)]), axis: [1, 0, 0], origin: [0, 0, 0], ...extra };
}

/** ✏️ A shape of one primitive in a view, in the frame the product expects (elevation: bone-local, front: (z, y), plan: (x, z) of the rest pose). */
export function shapeOfPrim(view: View, p: Prim, fr: Map<string, Frame>, meta: Json): Json {
  const f = fr.get(p.host)!;
  if (p.kind === "capsule") {
    const [a, b] = view === "elevation" ? [elevLocal(f, project(view, p.a)), elevLocal(f, project(view, p.b))] : [project(view, p.a), project(view, p.b)];
    return { ...meta, kind: "capsule", x0: r(a.x), y0: r(a.y), x1: r(b.x), y1: r(b.y), r0: r(p.r0), r1: r(p.r1) };
  }
  if (p.kind === "ellipsoid") {
    const e = projectedEllipse(view, p);
    if (view === "elevation") {
      const c = elevLocal(f, { x: e.cx, y: e.cy });
      return { ...meta, kind: "ellipse", cx: r(c.x), cy: r(c.y), rx: r(e.a), ry: r(e.b), rotationDeg: r(e.angleDeg - (f.angle * 180) / Math.PI, 3) };
    }
    return { ...meta, kind: "ellipse", cx: r(e.cx), cy: r(e.cy), rx: r(e.a), ry: r(e.b), ...(Math.abs(e.angleDeg) > 1e-3 ? { rotationDeg: r(e.angleDeg, 3) } : {}) };
  }
  if (view === "plan") {
    const rm = Math.max(...p.profile.map((q) => q[1]));
    return { ...meta, kind: "ellipse", cx: 0, cy: 0, rx: r(rm), ry: r(rm) };
  }
  const ring = primRing(view, p);
  const pts = view === "elevation" ? ring.map((q) => elevLocal(f, q)) : ring;
  return { ...meta, kind: "poly", points: pts.map((q) => [r(q.x), r(q.y)]), closed: true };
}

/** 🖊️ A filled path shape from world rings, bound to `bone`. */
export function pathShape(view: View, rings: P2[][], bone: string, fr: Map<string, Frame>, meta: Json): Json {
  const f = fr.get(bone)!;
  const local = view === "elevation" ? rings.map((q) => q.map((p) => elevLocal(f, p))) : rings;
  return { ...meta, bone, kind: "path", d: pathOf(local) };
}

/** 🍂️ The leaf marks of one cluster outline: short species strokes on a jittered grid, at least `margin` inside the outline and `spacing` apart. */
export function marksIn(sp: Species, rings: P2[][], seed: number, margin: number): P2[][][] {
  const outer = rings.filter((q) => ringArea(q) > 0);
  if (outer.length === 0) return [];
  const b = boxOf(outer.flat());
  const rand = rng(seed);
  const step = sp.marks.spacing;
  const out: P2[][][] = [];
  const s = sp.marks.size;
  for (let y = b.minY + step / 2; y < b.maxY; y += step)
    for (let x = b.minX + step / 2 + ((Math.round((y - b.minY) / step) % 2) * step) / 2; x < b.maxX; x += step) {
      const p = { x: x + (rand() - 0.5) * step * 0.35, y: y + (rand() - 0.5) * step * 0.35 };
      if (!rings.some((q) => inRing(p, q)) || ringDistance(p, rings) < margin + s) continue;
      const tilt = (rand() - 0.5) * 1.1;
      const rot = (q: P2): P2 => ({ x: p.x + q.x * Math.cos(tilt) - q.y * Math.sin(tilt), y: p.y + q.x * Math.sin(tilt) + q.y * Math.cos(tilt) });
      if (sp.marks.kind === "fan") out.push([[{ x: -0.5 * s, y: 0.25 * s }, { x: -0.18 * s, y: 0.48 * s }, { x: 0.18 * s, y: 0.48 * s }, { x: 0.5 * s, y: 0.25 * s }].map(rot), [{ x: 0, y: 0.48 * s }, { x: 0, y: -0.45 * s }].map(rot)]);
      else if (sp.marks.kind === "pinnate") out.push([[{ x: -0.5 * s, y: 0 }, { x: 0.5 * s, y: 0.06 * s }].map(rot), [{ x: -0.15 * s, y: 0.02 * s }, { x: -0.05 * s, y: 0.3 * s }].map(rot), [{ x: 0.2 * s, y: 0.04 * s }, { x: 0.3 * s, y: 0.3 * s }].map(rot)]);
      else out.push([0, 1, 2].map((k) => [{ x: 0.5 * s * Math.cos(tilt + (k * Math.PI) / 3 + Math.PI / 2), y: 0.5 * s * Math.sin(tilt + (k * Math.PI) / 3 + Math.PI / 2) }, { x: -0.5 * s * Math.cos(tilt + (k * Math.PI) / 3 + Math.PI / 2), y: -0.5 * s * Math.sin(tilt + (k * Math.PI) / 3 + Math.PI / 2) }].map((q) => ({ x: p.x + q.x, y: p.y + q.y }))));
    }
  return out;
}

/** 🪓️ Bark marks of one stem capsule in a view: `lines` wavy fissures between the stem edges, at least `margin` from the outline. */
export function barkMarks(sp: Species, a: P2, b: P2, r0: number, r1: number, margin: number, seed: number): P2[][] {
  const rand = rng(seed);
  const d = Math.hypot(b.x - a.x, b.y - a.y);
  const [ux, uy] = [(b.x - a.x) / d, (b.y - a.y) / d];
  const [nx, ny] = [-uy, ux];
  const out: P2[][] = [];
  const lines = sp.bark.lines;
  for (let i = 0; i < lines; i++) {
    const across = -1 + ((i + 0.5) / lines) * 2;
    const segs = sp.bark.kind === "plate" ? 4 : sp.bark.kind === "furrow" ? 2 : 1;
    for (let s = 0; s < segs; s++) {
      const t0 = 0.08 + (0.84 * s) / segs + rand() * 0.04;
      const t1 = 0.08 + (0.84 * (s + 1)) / segs - 0.05 - rand() * 0.04;
      const pts: P2[] = [];
      for (let k = 0; k <= 6; k++) {
        const t = t0 + ((t1 - t0) * k) / 6;
        const rad = r0 + (r1 - r0) * t - margin;
        if (rad <= 0) continue;
        const off = across * rad * (0.92 + 0.06 * Math.sin(seed + k * 1.7 + i));
        pts.push({ x: a.x + ux * d * t + nx * off, y: a.y + uy * d * t + ny * off });
      }
      if (pts.length >= 2) out.push(pts);
    }
  }
  return out;
}
