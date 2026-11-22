/** 🌳️ PW-02 broadleaf park-tree kit: turns a hand-written species design (stem stations, limbs, branches, crown envelope and lobe rings per scale, physics) into a complete `semio.staffage/v2` asset: a sagittal botanical rig with lateral offsets, the four individually composed scales as true views of their own bodies, the volumes, poses, clips, thermal and structural models.
 * Every number of a tree is stated in its species file; this kit only places, converts, projects and formats.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md */
import { frames, toLocal, type Frame, type Json, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { D2R, add, dot3, len, lobeOutline, mul, nearness, outline, pathOf, project, r4, r5, solidOf, sub, toDrawing, tubePolygons, unit, type Lobe, type Tube, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts";

export type Text = { en: string; de: string };
export const tx = (en: string, de: string): Text => ({ en, de });
export const SCALES = [50, 100, 200, 500] as const;
export type Scale = (typeof SCALES)[number];
export const VIEWS: View[] = ["elevation", "front", "plan"];
const RHO_AIR = 1.25;
const G = 9.80665;
const r3 = (x: number): number => Math.round(x * 1000) / 1000;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const r1 = (x: number): number => Math.round(x * 10) / 10;
const feature = (scale: number): number => (0.15 * scale) / 1000;

/** 🪵️ One stem bone: from height `y0` to `y1` of the stem axis. */
export type StemBone = { id: string; y0: number; y1: number; title: Text; limit: number; region: "trunk-bark" | "limb-bark" };
/** 🌿️ A limb with one or two bones: leaves its parent at height `at` (stem) or at fraction `at` of a parent limb, azimuth `az` (degrees from +x towards +z), elevation profile `elev` [[s, deg]], length, radii; `split` gives the fraction where the outer bone starts. */
export type LimbSpec = { id: string; parent: string; at: number; az: number; elev: [number, number][]; L: number; r0: number; r1: number; bend?: number; split?: number; title: Text; titleB?: Text; limit?: number };
/** 🌱️ A side branch without a bone, carried by the limb bone that holds fraction `t`; drawn only at the listed scales. */
export type BranchSpec = { id: string; limb: string; t: number; az: number; elev: [number, number][]; L: number; r0: number; r1: number; bend?: number; scales: Scale[] };
/** 🫧️ A ring of foliage lobes at height `y`: `n` lobes from azimuth `phase`, radii `a` radial, `b` vertical, `c` tangential (or `ck` times half the arc spacing), outer reach `reach` times the envelope, `tilt` droops the radial axis, `region` sun or shade. */
export type Ring = { y: number; n: number; phase: number; a: number; b: number; c?: number; ck?: number; tilt?: number; reach?: number; region: "sun" | "shade"; jitter?: number; skip?: number[] };
/** 🧭️ The per-scale design of one tree. */
export type ScaleDesign = { rings: Ring[]; top: { y: number; a: number; b: number }; tubeN: number; rmin: number; twigR: number; leafMarks: number; markSize: number; barkMarks: number; candles?: number; candleSize?: [number, number] };
/** 🌸️ Panicles that stand on top of sun lobes in the flowering pose and are withdrawn into the crown otherwise (prismatic bones). */
export type CandleSpec = { length: number; radius: number; lean: number; retract: number; massKg: number; region: string; material: string; title: Text; minY: number };

export type Species = {
  id: string; title: Text; summary: Text; distinction: Text; tags: string[]; created: string; notes: Text;
  references: Json[]; materials: Json[];
  barkMaterial: string; youngBarkMaterial: string; woodMaterial: string; sunMaterial: string; shadeMaterial: string;
  density: number; foliageKgPerM2: number;
  stem: { stations: [number, number, number, number][]; bones: StemBone[] };
  limbs: LimbSpec[]; branches: BranchSpec[];
  envelope: (y: number, az: number) => number;
  scales: Record<Scale, ScaleDesign>;
  leafArea: { sun: number; shade: number; projectionM2: number };
  regions: Json[]; leafRegionTitles: { sun: Text; shade: Text };
  emitterDiameter: { sun: number; shade: number };
  tones: { trunk: number; limb: number; sun: number; shade: number; candle?: number };
  porosity: number; cdStem: number;
  thermal: { environment: Json; assumptions: Text[] };
  structureAssumptions: Text[];
  poses: PoseSpec[]; defaultPose: string; breezePose: string; calmPose: string;
  sway: { period: number; amp: Record<string, number>; lag: number; text: Text };
  gust: { from: number; peak: number; text: Text };
  diurnal: { text: Text; sun: [number, number][]; shade: [number, number][]; sunlit: Record<string, [number, number][]> };
  extraClips?: (ctx: Ctx) => Json[];
  candles?: CandleSpec;
  leafMark: (c: P2, size: number, deg: number, view: View) => P2[][];
  barkMark: (k: number, view: View, stem: (y: number) => { x: number; z: number; r: number }) => P2[] | null;
  texts: { scale: Record<Scale, { silhouette: Text; merged: Text; omitted: Text; thinElements: Text; styles: Text }>; volume: Record<Scale, Text>; symbol: Text };
  symbolLimbs: string[];
  defaultScale: Scale;
};

/** 🌬️ A pose: wind speed at 10 m and drag coefficient (0 for calm), lean factor, droop, snow or fruit load per foliage plan area, thermal binding. */
export type PoseSpec = { id: string; title: Text; description: Text; v10: number; cd: number; lean: number; droop: number; snowKgM2?: number; fruitKgM2?: number; fruitTitle?: Text; candlesOut: boolean; regions: Record<string, { exposedFraction: number; sunlitFraction: number; wettedness?: number }>; emitters: { sun: number; shade: number } };

export type Bone = Json;
export type Model = { scale: Scale; wood: Tube[]; lobes: (Lobe & { region: "sun" | "shade"; ring: number })[]; candles: (Tube & { cbone: string })[]; marks: Json[] };
export type Ctx = { sp: Species; bones: Bone[]; F: Record<string, Frame>; models: Record<Scale, Model>; loadsOf: (p: PoseSpec) => Json[]; windLoadIds: string[]; candleBones: string[]; poseBones: (p: PoseSpec, k: number) => Json };

const lerpRows = (rows: [number, number][], s: number): number => {
  if (s <= rows[0]![0]) return rows[0]![1];
  for (let i = 0; i + 1 < rows.length; i++) if (s <= rows[i + 1]![0]) return rows[i]![1] + ((rows[i + 1]![1] - rows[i]![1]) * (s - rows[i]![0])) / (rows[i + 1]![0] - rows[i]![0]);
  return rows[rows.length - 1]![1];
};

/** 📏️ The stem axis at height y: lateral offsets and radius. */
export function stemAt(sp: Species, y: number): { x: number; z: number; r: number } {
  const st = sp.stem.stations;
  const col = (k: number): [number, number][] => st.map((row) => [row[0], row[k]!] as [number, number]);
  return { x: lerpRows(col(1), y), z: lerpRows(col(2), y), r: lerpRows(col(3), y) };
}

/** 🧵️ The world centre line of a limb or branch: `n` stations from `start`. */
function curve(start: V3, az: number, elev: [number, number][], L: number, bend: number, n: number): V3[] {
  const pts: V3[] = [start];
  let p = start;
  const steps = 48;
  const out: V3[] = [start];
  for (let i = 1; i <= steps; i++) {
    const s = (i - 0.5) / steps;
    const e = lerpRows(elev, s) * D2R, a = (az + bend * s) * D2R;
    p = add(p, mul([Math.cos(e) * Math.cos(a), Math.sin(e), Math.cos(e) * Math.sin(a)], L / steps));
    pts.push(p);
  }
  for (let k = 1; k <= n; k++) out.push(pts[Math.round((k * steps) / n)]!);
  return out;
}

type Axis = { id: string; bone: string; pts: V3[]; r: number[]; region: string; material: string; kind: "stem" | "limb" | "branch" | "twig" };

/** 🦴️ The rig, solved from the 1:50 axes so every bone starts at its true joint: stem bones on the stem axis, limb bones from their base along the projected chord. */
export function buildRig(sp: Species): { bones: Bone[]; limbAxes: Record<string, V3[]>; boneOf: (limb: string, s: number) => string } {
  const bones: Bone[] = [];
  const world: Record<string, { o: V3; phi: number }> = {};
  const add3 = (id: string, parent: string | null, o: V3, phi: number, L: number, title: Text, joint: Json): void => {
    const p = parent ? world[parent]! : null;
    let x = o[0], y = o[1], z = o[2], ang = phi;
    if (p) {
      const d = sub(o, p.o);
      const u = [Math.cos(p.phi * D2R), Math.sin(p.phi * D2R)], v = [-Math.sin(p.phi * D2R), Math.cos(p.phi * D2R)];
      x = d[0] * u[0]! + d[1] * u[1]!;
      y = d[0] * v[0]! + d[1] * v[1]!;
      z = d[2];
      ang = phi - p.phi;
    }
    world[id] = { o, phi };
    bones.push({ id, parent, title, x: r4(x), y: r4(y), ...(Math.abs(z) > 1e-4 ? { z: r4(z) } : {}), angleDeg: r4(ang), lengthM: r4(L), joint });
  };
  for (const [i, b] of sp.stem.bones.entries()) {
    const a = stemAt(sp, b.y0), c = stemAt(sp, b.y1);
    const o: V3 = [a.x, b.y0, a.z];
    const phi = Math.atan2(b.y1 - b.y0, c.x - a.x) / D2R;
    add3(b.id, i === 0 ? null : sp.stem.bones[i - 1]!.id, o, phi, Math.hypot(b.y1 - b.y0, c.x - a.x), b.title, i === 0 ? { type: "fixed" } : { type: "revolute", minDeg: -b.limit, maxDeg: b.limit });
  }
  const limbAxes: Record<string, V3[]> = {};
  const stemBoneAt = (y: number): string => (sp.stem.bones.find((b) => y >= b.y0 && y < b.y1) ?? sp.stem.bones[sp.stem.bones.length - 1]!).id;
  const startOf = (l: LimbSpec): V3 => {
    if (limbAxes[l.parent]) return pointAt(limbAxes[l.parent]!, l.at);
    const s = stemAt(sp, l.at);
    return [s.x, l.at, s.z];
  };
  const splitOf = (l: LimbSpec): number => l.split ?? 1;
  for (const l of sp.limbs) {
    const pts = curve(startOf(l), l.az, l.elev, l.L, l.bend ?? 0, 48);
    limbAxes[l.id] = pts;
    const parentBone = limbAxes[l.parent] ? boneOfLimb(l.parent, l.at) : sp.stem.bones.some((b) => b.id === l.parent) ? l.parent : stemBoneAt(l.at);
    const parts: [string, number, number, Text][] = l.split ? [[l.id, 0, l.split, l.title], [`${l.id}-b`, l.split, 1, l.titleB ?? l.title]] : [[l.id, 0, 1, l.title]];
    let par = parentBone;
    for (const [id, s0, s1, title] of parts) {
      const o = pointAt(pts, s0), e = pointAt(pts, s1);
      let phi = Math.atan2(e[1] - o[1], e[0] - o[0]) / D2R;
      let L = Math.hypot(e[1] - o[1], e[0] - o[0]);
      if (L < 0.3) {
        phi = 90;
        L = 0.3;
      }
      add3(id, par, o, phi, L, title, { type: "revolute", minDeg: -(l.limit ?? 10) * (id.endsWith("-b") ? 1.4 : 1), maxDeg: (l.limit ?? 10) * (id.endsWith("-b") ? 1.4 : 1) });
      par = id;
    }
    void splitOf;
  }
  function boneOfLimb(limb: string, s: number): string {
    const l = sp.limbs.find((x) => x.id === limb)!;
    return l.split && s >= l.split ? `${l.id}-b` : l.id;
  }
  return { bones, limbAxes, boneOf: boneOfLimb };
}

/** 📍️ The point at fraction s of a polyline (by station index). */
export function pointAt(pts: V3[], s: number): V3 {
  const x = Math.max(0, Math.min(1, s)) * (pts.length - 1);
  const i = Math.min(pts.length - 2, Math.floor(x));
  return add(pts[i]!, mul(sub(pts[i + 1]!, pts[i]!), x - i));
}

const resample = (pts: V3[], s0: number, s1: number, n: number): V3[] => Array.from({ length: n + 1 }, (_, k) => pointAt(pts, s0 + ((s1 - s0) * k) / n));

/** 🫧️ A lobe of a ring: kit form plus region and ring index. */
function ringLobe(sp: Species, ring: Ring, j: number, index: number, scale: number): Lobe & { region: "sun" | "shade"; ring: number } {
  const jit = ring.jitter ?? 0;
  const h = (k: number): number => Math.sin(12.9898 * (j + 1) * (k + 1) + 78.233 * (index + 1) + scale) * 43758.5453 % 1;
  const az = ring.phase + (360 * j) / ring.n + jit * 18 * h(1);
  const y = ring.y + jit * 0.25 * ring.b * h(2);
  const reachF = (ring.reach ?? 1) * (1 + jit * 0.03 * h(3));
  const reach = sp.envelope(y, az) * reachF;
  const s = stemAt(sp, y);
  const c = ring.c ?? ((ring.ck ?? 1.25) * Math.PI * Math.max(0.6, reach - ring.a)) / ring.n;
  const tilt = (ring.tilt ?? 0) * D2R;
  const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const up: V3 = [0, 1, 0];
  const tan: V3 = [-Math.sin(az * D2R), 0, Math.cos(az * D2R)];
  const rad1 = sub(mul(rad, Math.cos(tilt)), mul(up, Math.sin(tilt)));
  const up0 = add(mul(up, Math.cos(tilt)), mul(rad, Math.sin(tilt)));
  const roll = jit * 22 * h(5) * D2R;
  const up1 = add(mul(up0, Math.cos(roll)), mul(tan, Math.sin(roll)));
  const tan1 = sub(mul(tan, Math.cos(roll)), mul(up0, Math.sin(roll)));
  const k = 1 + 0.14 * jit * h(4);
  const [ra, rb] = [ring.a * k, ring.b * (1 + 0.1 * jit * h(6))];
  const rcT = Math.min(Math.max(c, Math.min(ra, rb) * 0.6), 1.45 * Math.max(ra, rb));
  const extent = Math.hypot(ra * Math.cos(tilt), rb * Math.sin(tilt));
  const rc = Math.max(0, reach - extent);
  return { kind: "lobe", id: "", bone: "", material: "", tone: 0, group: ring.region, c: [s.x + rc * rad[0], y, s.z + rc * rad[2]], axes: [rad1, up1, tan1], radii: [ra, rb, rcT], region: ring.region, ring: index };
}

const insideLobe = (l: Lobe, p: V3, k = 1): boolean => {
  const d = sub(p, l.c);
  return (dot3(d, l.axes[0]) / l.radii[0]) ** 2 + (dot3(d, l.axes[1]) / l.radii[1]) ** 2 + (dot3(d, l.axes[2]) / l.radii[2]) ** 2 <= k * k;
};

export type Stretch = { xn: number; xp: number; zn: number; zp: number; y: number };

/** 🧰️ The parts of one scale: stem, limbs, branches and twigs as tubes, crown lobes, candles. `stretch` moves the lobe centres away from the stem axis (per side) and up from the lowest ring, so a coarse scale fills the box of the 1:50 body. */
export function buildModel(sp: Species, scale: Scale, limbAxes: Record<string, V3[]>, boneOf: (limb: string, s: number) => string, stretch: Stretch = { xn: 1, xp: 1, zn: 1, zp: 1, y: 1 }): Model {
  const d = sp.scales[scale];
  const axes: Axis[] = [];
  const floor = (r: number): number => Math.max(d.rmin, r);
  const n = d.tubeN;
  for (const [i, b] of sp.stem.bones.entries()) {
    const y0 = i === 0 ? 0 : b.y0 - Math.min(0.25, 0.5 * stemAt(sp, b.y0).r);
    const cnt = Math.max(2, Math.round(((b.y1 - y0) / 1.2) * (n / 8)));
    const ys = Array.from({ length: cnt + 1 }, (_, k) => y0 + ((b.y1 - y0) * k) / cnt);
    if (i === 0) ys.splice(1, 0, 0.05, 0.14, 0.26);
    const uniq = [...new Set(ys.map((y) => r4(y)))].sort((p, q) => p - q);
    axes.push({ id: b.id, bone: b.id, pts: uniq.map((y) => { const s = stemAt(sp, y); return [s.x, y, s.z] as V3; }), r: uniq.map((y) => floor(stemAt(sp, y).r)), region: b.region, material: b.region === "trunk-bark" ? sp.barkMaterial : sp.youngBarkMaterial, kind: "stem" });
  }
  const top = sp.stem.bones[sp.stem.bones.length - 1]!;
  for (const l of sp.limbs) {
    const pts = limbAxes[l.id]!;
    const parts: [string, number, number][] = l.split ? [[l.id, 0, l.split], [`${l.id}-b`, l.split - 0.02, 1]] : [[l.id, 0, 1]];
    for (const [id, s0, s1] of parts) {
      const m = Math.max(2, Math.round(n * (s1 - s0)));
      const P = resample(pts, s0, s1, m);
      const R = P.map((_, k) => floor(l.r0 + (l.r1 - l.r0) * (s0 + ((s1 - s0) * k) / m)));
      const parentR = limbAxes[l.parent] ? 0.5 * Math.max(d.rmin, (() => { const q = sp.limbs.find((x) => x.id === l.parent)!; return q.r0 + (q.r1 - q.r0) * l.at; })()) : Math.min(0.6 * stemAt(sp, P[0]![1]).r, 0.3);
      if (s0 === 0) P[0] = sub(P[0]!, mul(unit(sub(P[1]!, P[0]!)), parentR));
      axes.push({ id, bone: id, pts: P, r: R, region: "limb-bark", material: sp.youngBarkMaterial, kind: "limb" });
    }
  }
  for (const b of sp.branches.filter((x) => x.scales.includes(scale))) {
    const start = pointAt(limbAxes[b.limb]!, b.t);
    const m = Math.max(2, Math.round(n * 0.7));
    const P = curve(start, b.az, b.elev, b.L, b.bend ?? 0, m);
    axes.push({ id: `br-${b.id}`, bone: boneOf(b.limb, b.t), pts: P, r: P.map((_, k) => floor(b.r0 + ((b.r1 - b.r0) * k) / m)), region: "limb-bark", material: sp.youngBarkMaterial, kind: "branch" });
  }
  const lobes: Model["lobes"] = [];
  d.rings.forEach((ring, index) => {
    for (let j = 0; j < ring.n; j++) if (!(ring.skip ?? []).includes(j)) lobes.push(ringLobe(sp, ring, j, index, scale));
  });
  const ts = stemAt(sp, d.top.y);
  lobes.push({ kind: "lobe", id: "", bone: "", material: "", tone: 0, group: "sun", c: [ts.x, d.top.y, ts.z], axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii: [d.top.a, d.top.b, d.top.a], region: "sun", ring: -1 });
  const yLow = Math.min(...lobes.map((l) => l.c[1]));
  for (const l of lobes) {
    const s = stemAt(sp, l.c[1]);
    const [dx, dz] = [l.c[0] - s.x, l.c[2] - s.z];
    l.c = [s.x + dx * (dx < 0 ? stretch.xn : stretch.xp), yLow + (l.c[1] - yLow) * stretch.y, s.z + dz * (dz < 0 ? stretch.zn : stretch.zp)];
  }
  const woodAxes = [...axes];
  const twigs: Axis[] = [];
  lobes.forEach((l, k) => {
    let best = { dist: Infinity, p: [0, 0, 0] as V3, axis: woodAxes[0]!, i: 0 };
    for (const a of woodAxes) for (let i = 0; i + 1 < a.pts.length; i++) {
      const [p, q] = [a.pts[i]!, a.pts[i + 1]!];
      const dq = sub(q, p);
      const tt = Math.max(0, Math.min(1, dot3(sub(l.c, p), dq) / Math.max(1e-9, dot3(dq, dq))));
      const x = add(p, mul(dq, tt));
      const dist = len(sub(l.c, x));
      const penalty = a.kind === "stem" && l.region === "sun" ? 1.6 : 1;
      if (dist * penalty < best.dist) best = { dist: dist * penalty, p: x, axis: a, i };
    }
    l.bone = best.axis.bone;
    l.id = `lobe-${String(k + 1).padStart(3, "0")}`;
    l.material = l.region === "sun" ? sp.sunMaterial : sp.shadeMaterial;
    l.tone = l.region === "sun" ? sp.tones.sun : sp.tones.shade;
    if (!insideLobe(l, best.p, 0.92)) {
      const dir = unit(sub(l.c, best.p));
      const tip = add(l.c, mul(dir, -0.25 * Math.min(...l.radii)));
      const tr = floor(d.twigR);
      twigs.push({ id: `tw-${String(k + 1).padStart(3, "0")}`, bone: l.bone, pts: [sub(best.p, mul(dir, 0.3 * best.axis.r[best.i]!)), add(best.p, mul(sub(tip, best.p), 0.5)), tip], r: [tr * 1.25, tr * 1.1, tr], region: "limb-bark", material: sp.youngBarkMaterial, kind: "twig" });
    }
  });
  const wood: Tube[] = [...axes, ...twigs].map((a) => ({ kind: "tube", id: a.id, bone: a.bone, material: a.material, tone: a.region === "trunk-bark" ? sp.tones.trunk : sp.tones.limb, group: a.region, points: a.pts.map((p, i) => ({ p, r: r5(a.r[i]!) })), flags: { kind: a.kind } }));
  const candles: Model["candles"] = [];
  if (sp.candles && d.candles) {
    const [cl, cr] = d.candleSize ?? [sp.candles.length, sp.candles.radius];
    const hosts = lobes.filter((l) => l.region === "sun" && l.c[1] + l.radii[1] > sp.candles!.minY).filter((l) => l.ring >= 0 && l.c[1] + l.radii[1] + cl < d.top.y + d.top.b - 0.2).sort((p, q) => p.ring - q.ring || Math.atan2(p.c[2], p.c[0]) - Math.atan2(q.c[2], q.c[0]));
    const step = hosts.length / d.candles;
    for (let k = 0; k < d.candles && k * step < hosts.length; k++) {
      const l = hosts[Math.floor(k * step)]!;
      const out = unit([l.axes[0][0], 0, l.axes[0][2]]);
      const dir = unit(add([0, 1, 0], mul(out, Math.tan(sp.candles.lean * D2R))));
      const base = add(l.c, mul(unit(add(mul(out, 0.45), [0, 1, 0])), 1));
      const q = Math.hypot(dot3(sub(base, l.c), l.axes[0]) / l.radii[0], dot3(sub(base, l.c), l.axes[1]) / l.radii[1], dot3(sub(base, l.c), l.axes[2]) / l.radii[2]);
      const surface = add(l.c, mul(sub(base, l.c), 1 / q));
      const foot = sub(surface, mul(dir, 0.3 * cl));
      const pts: V3[] = [foot, add(foot, mul(dir, 0.35 * cl)), add(foot, mul(dir, 0.7 * cl)), add(foot, mul(dir, 1.05 * cl)), add(foot, mul(dir, 1.3 * cl))];
      const rr = [cr * 0.9, cr, cr * 0.82, cr * 0.55, cr * 0.28].map((r) => Math.max(r, feature(scale) * 0.55));
      candles.push({ kind: "tube", id: `candle-${String(k + 1).padStart(3, "0")}`, bone: "", cbone: l.bone, material: sp.candles.material, tone: sp.tones.candle ?? 0.9, group: "candle", points: pts.map((p, i) => ({ p, r: r5(rr[i]!) })) });
    }
  }
  return { scale, wood, lobes, candles, marks: [] };
}

/** 🧊️ Area, volume and centroid of a tube (frustum sum). */
function tubeStats(t: Tube): { area: number; volume: number; c: V3 } {
  let area = 0, volume = 0;
  let c: V3 = [0, 0, 0];
  for (let i = 0; i + 1 < t.points.length; i++) {
    const [a, b] = [t.points[i]!, t.points[i + 1]!];
    const h = len(sub(b.p, a.p));
    const v = (Math.PI * h * (a.r * a.r + a.r * b.r + b.r * b.r)) / 3;
    area += Math.PI * (a.r + b.r) * Math.hypot(h, a.r - b.r);
    volume += v;
    c = add(c, mul(add(a.p, b.p), v / 2));
  }
  return { area, volume, c: volume > 0 ? mul(c, 1 / volume) : t.points[0]!.p };
}

const lobeVolume = (l: Lobe): number => (4 / 3) * Math.PI * l.radii[0] * l.radii[1] * l.radii[2];

/** 🔲️ The world box half extents of a lobe. */
const lobeHalf = (l: Lobe): V3 => [0, 1, 2].map((k) => Math.sqrt(l.axes.reduce((s, a, i) => s + (a[k]! * l.radii[i]!) ** 2, 0))) as V3;

/** 📐️ The body bounds of a model (rest pose). */
export function boundsOf(m: Model): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  const take = (p: V3, h: V3): void => {
    for (let k = 0; k < 3; k++) {
      min[k] = Math.min(min[k]!, p[k]! - h[k]!);
      max[k] = Math.max(max[k]!, p[k]! + h[k]!);
    }
  };
  for (const t of [...m.wood, ...m.candles]) for (const s of t.points) take(s.p, [s.r, s.r, s.r]);
  for (const l of m.lobes) take(l.c, lobeHalf(l));
  min[1] = Math.max(0, min[1]!);
  return { min, max };
}

/** ✏️ The 2D drawings of one scale: one filled outline per bone and part kind (wood, twigs, sun or shade foliage, candles), texture marks of leaves and bark, painted by depth. */
function drawScale(sp: Species, m: Model, F: Record<string, Frame>, view: View, stemFn: (y: number) => { x: number; z: number; r: number }): Json[] {
  const f = feature(m.scale);
  const tol = f / 4;
  type Item = { shape: Json; near: number; rings: P2[][]; closed: boolean };
  const items: Item[] = [];
  const set = (shape: Json, rings: P2[][], closed: boolean, near: number): void => {
    if (rings.length === 0 || rings[0]!.length < 2) return;
    items.push({ shape, rings, closed, near });
  };
  const tubeNear = (t: Tube): number => Math.max(...t.points.map((s) => nearness(view, s.p) + s.r));
  const lobeNear = (l: Lobe): number => nearness(view, l.c) + Math.abs(dot3(lobeHalf(l), view === "elevation" ? [0, 0, 1] : view === "front" ? [1, 0, 0] : [0, 1, 0]));
  const byBone = new Map<string, Tube[]>();
  for (const t of m.wood) {
    const key = `${t.bone}|${t.group}`;
    byBone.set(key, [...(byBone.get(key) ?? []), t]);
  }
  for (const [key, list] of byBone) {
    const [bone, region] = key.split("|") as [string, string];
    const isMain = list.some((t) => t.id === bone);
    const rings = outline(list.flatMap((t) => tubePolygons(view, t)), tol, (3 * f) ** 2);
    set({ id: `${isMain ? "wood" : "twigs"}-${bone}`, bone, role: "body", filled: true, tone: region === "trunk-bark" ? sp.tones.trunk : sp.tones.limb, region, material: region === "trunk-bark" ? sp.barkMaterial : sp.youngBarkMaterial }, rings, true, Math.max(...list.map(tubeNear)));
  }
  const groups = new Map<string, Model["lobes"]>();
  for (const l of m.lobes) groups.set(`${l.bone}|${l.region}`, [...(groups.get(`${l.bone}|${l.region}`) ?? []), l]);
  for (const [key, list] of groups) {
    const [bone, region] = key.split("|") as [string, string];
    const rings = outline(list.map((l) => lobeOutline(view, l, 60)), tol, (5 * f) ** 2);
    set({ id: `${region}-${bone}`, bone, role: "body", filled: true, tone: region === "sun" ? sp.tones.sun : sp.tones.shade, region: region === "sun" ? "leaf-sun" : "leaf-shade", material: region === "sun" ? sp.sunMaterial : sp.shadeMaterial }, rings, true, Math.max(...list.map(lobeNear)));
  }
  const cg = new Map<string, Tube[]>();
  for (const c of m.candles) cg.set(c.bone, [...(cg.get(c.bone) ?? []), c]);
  for (const [bone, list] of cg) set({ id: bone, bone, role: "body", filled: true, tone: sp.tones.candle ?? 0.9, region: sp.candles!.region, material: sp.candles!.material }, outline(list.flatMap((t) => tubePolygons(view, t)), tol, (3 * f) ** 2), true, Math.max(...list.map(tubeNear)));
  if (m.scale <= 100) {
    const sd = sp.scales[m.scale];
    const visible = [...m.lobes].filter((l) => l.region === "sun").sort((p, q) => lobeNear(q) - lobeNear(p));
    const rims = m.lobes.map((l) => ({ l, ring: lobeOutline(view, l, 36), near: lobeNear(l) }));
    let count = 0;
    for (const [i, l] of visible.entries()) {
      if (count >= sd.leafMarks) break;
      const c0 = project(view, l.c);
      const spread = Math.min(l.radii[0], l.radii[1], l.radii[2]);
      const c = { x: c0.x + 0.3 * spread * Math.cos(i * 2.3), y: c0.y + 0.25 * spread * Math.sin(i * 1.7) };
      const size = sd.markSize;
      const mark = sp.leafMark(c, size, (i * 47) % 360, view);
      const pts = mark.flat();
      const covered = pts.some((p) => rims.some((o) => o.l !== l && o.near > lobeNear(l) && pip(p, o.ring, 2.5 * f)));
      const own = pts.every((p) => pip(p, lobeOutline(view, l, 36), -3 * f));
      if (covered || !own) continue;
      count++;
      mark.forEach((stroke, s) => set({ id: `leaf-${l.bone}-${count}-${s + 1}`, bone: l.bone, role: "texture", filled: false, tone: 0.3, region: l.region === "sun" ? "leaf-sun" : "leaf-shade", material: sp.sunMaterial }, [stroke], false, lobeNear(l) + 1e-4));
    }
    if (view !== "plan") {
      for (let k = 0; k < sd.barkMarks; k++) {
        const stroke = sp.barkMark(k, view, stemFn);
        if (!stroke) continue;
        const y = stroke[Math.floor(stroke.length / 2)]!.y;
        const bone = sp.stem.bones.find((b) => y >= b.y0 && y < b.y1)?.id ?? sp.stem.bones[1]!.id;
        const near = view === "elevation" ? -stemFn(y).z + stemFn(y).r : stemFn(y).x + stemFn(y).r;
        const blockers = m.lobes.filter((l) => lobeNear(l) > near).map((l) => lobeOutline(view, l, 36));
        if (stroke.some((p) => blockers.some((ring) => pip(p, ring, 2.5 * f)))) continue;
        set({ id: `bark-${k + 1}`, bone, role: "texture", filled: false, tone: 0.2, region: "trunk-bark", material: sp.barkMaterial }, [stroke], false, near + 1e-4);
      }
    }
  }
  items.sort((p, q) => p.near - q.near);
  return items.map((it, i) => {
    const frame = F[it.shape.bone]!;
    const local = toDrawing(view, frame, it.rings);
    const z = r3(1 + i * 0.01);
    if (it.rings.length > 1 || it.closed) return { ...it.shape, z, kind: "path", d: pathOf(local, it.closed) };
    return { ...it.shape, z, kind: "poly", points: local[0]!.map((p) => [r4(p.x), r4(p.y)]), closed: it.closed };
  });
}

/** 🎯️ Point in ring, with a margin (positive grows the ring, negative shrinks it). */
function pip(p: P2, ring: P2[], margin = 0): boolean {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  if (margin === 0) return hit;
  let dmin = Infinity;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    const dx = b.x - a.x, dy = b.y - a.y;
    const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / Math.max(1e-12, dx * dx + dy * dy)));
    dmin = Math.min(dmin, Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy));
  }
  return margin > 0 ? hit || dmin <= margin : hit && dmin >= -margin;
}

/** 🧊️ The volume of one scale: body solids, thermal references and transpiration outlets, the structural skeleton. */
function volumeOf(sp: Species, m: Model, F: Record<string, Frame>, bones: Bone[], anchorsOf: (scale: Scale) => { sun: Json[]; shade: Json[] }, texts: Text): Json {
  const f = feature(m.scale);
  const body: Json[] = [];
  for (const t of m.wood) {
    const solid = solidOf({ ...t, flags: undefined }, F);
    body.push(solid);
  }
  for (const l of m.lobes) body.push(solidOf(l, F));
  for (const c of m.candles) body.push({ ...solidOf({ ...c, bone: c.bone }, F), ...(m.scale === 500 ? { envelope: true } : {}) });
  const regionOfWood = (t: Tube): string => t.group;
  const thermal: Json[] = [
    ...m.wood.map((t) => ({ id: `th-${t.id}`, of: t.id, region: regionOfWood(t) })),
    ...m.lobes.map((l) => ({ id: `th-${l.id}`, of: l.id, region: l.region === "sun" ? "leaf-sun" : "leaf-shade" })),
    ...m.candles.map((c) => ({ id: `th-${c.id}`, of: c.id, region: sp.candles!.region })),
  ];
  const outlets = anchorsOf(m.scale);
  for (const [kind, list] of [["sun", outlets.sun], ["shade", outlets.shade]] as const)
    list.forEach((a: Json, k: number) => {
      const fr = F[a.bone]!;
      const half = Math.max(0.6 * f, 0.05);
      const rad = Math.max(sp.emitterDiameter[kind] / 2, 2 * f);
      const up = unit(toLocalDir(fr, [0, 1, 0]));
      const ref: V3 = Math.abs(up[0]) < 0.9 ? [1, 0, 0] : [0, 1, 0];
      const e1 = unit(sub(ref, mul(up, dot3(ref, up))));
      const e2: V3 = [up[1] * e1[2] - up[2] * e1[1], up[2] * e1[0] - up[0] * e1[2], up[0] * e1[1] - up[1] * e1[0]];
      const at = sub(a.local as V3, mul(up, 0.4 * half));
      thermal.push({ id: `em-${kind}-${k + 1}`, kind: "ellipsoid", bone: a.bone, material: kind === "sun" ? sp.sunMaterial : sp.shadeMaterial, role: "detail", tone: 0.5, region: kind === "sun" ? "leaf-sun" : "leaf-shade", tags: ["emitter", `transpiration-${kind}`], radii: [r5(rad), r5(half), r5(rad)], transform: { translation: at.map(r5), rotation: quat(e1, up, e2) } });
    });
  const structure: Json[] = [];
  const mains = m.wood.filter((t) => bones.some((b) => b.id === t.id));
  for (const t of mains) {
    const pts = t.points.map((s) => ({ p: s.p, r: s.r * 0.55 }));
    const memberTube: Tube = { ...t, id: `member-${t.id}`, points: pts, flags: undefined };
    const L = tubeStats(t).volume > 0 ? t.points.slice(1).reduce((s, q, i) => s + len(sub(q.p, t.points[i]!.p)), 0) : 0;
    const maxD = 2 * Math.max(...pts.map((s) => s.r));
    const solid = solidOf(memberTube, F);
    structure.push({ ...solid, role: "body", tone: 0.4, material: sp.woodMaterial, member: t.id, part: "member", ...(L >= 5 * maxD && L >= 2 * f ? { slender: true } : {}) });
    const j0 = t.points[t.id === sp.stem.bones[0]!.id ? 0 : 1]!;
    const sb = sp.stem.bones.find((x) => x.id === t.id);
    const jy = sb ? (sb.y0 === 0 ? 0.32 : sb.y0) : 0;
    const jp: V3 = sb ? [stemAt(sp, jy).x, jy, stemAt(sp, jy).z] : t.points[0]!.p;
    const jr = Math.max(sb ? Math.min(0.5 * stemAt(sp, jy).r, jy - 0.02) : 0.62 * t.points[0]!.r, f * 0.6);
    void j0;
    structure.push({ id: `joint-${t.id}`, kind: "ellipsoid", bone: t.id, material: sp.woodMaterial, role: "detail", tone: 0.3, member: t.id, part: "joint", radii: [r5(jr), r5(jr), r5(jr)], transform: { translation: toLocal(F[t.id]!, jp).map(r5) } });
  }
  const root = sp.stem.bones[0]!.id;
  const rr = stemAt(sp, 0).r * 1.05;
  structure.push({ id: "support-root-plate", kind: "loft", bone: root, material: sp.woodMaterial, role: "body", tone: 0.5, member: root, part: "support", anchor: "base-support", capStart: true, capEnd: true, sections: [0, Math.max(f, 0.06)].map((h) => ({ at: toLocal(F[root]!, [0, h, 0]).map(r5), rx: r5(rr), ry: r5(rr), rotation: [0.5, 0.5, 0.5, 0.5] })) });
  const decisions = [texts, scaleListText(m, body.length, thermal.length, structure.length)];
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions };
}

const toLocalDir = (f: Frame, d: V3): V3 => [dot3(d, f.u), dot3(d, f.v), dot3(d, f.w)];

function quat(c0: V3, c1: V3, c2: V3): number[] {
  const [m00, m10, m20] = c0, [m01, m11, m21] = c1, [m02, m12, m22] = c2;
  const t = m00 + m11 + m22;
  let q: number[];
  if (t > 0) {
    const s = Math.sqrt(t + 1) * 2;
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
  return q.map((x) => r5(x / n));
}

function scaleListText(m: Model, nb: number, nt: number, ns: number): Text {
  const wood = m.wood.filter((t) => !t.id.startsWith("tw-")).length, twigs = m.wood.length - wood;
  const sun = m.lobes.filter((l) => l.region === "sun").length, shade = m.lobes.length - sun;
  return tx(
    `Lists at 1:${m.scale}: body ${nb} solids (${wood} stem, limb and branch lofts, ${twigs} twig lofts into the foliage, ${sun} sun and ${shade} shade foliage lobes${m.candles.length ? `, ${m.candles.length} flower panicles` : ""}); thermal ${nt} solids (every body solid by reference with its region, plus the transpiration outlets on the tops of the foliage lobes that carry the emitter anchors); structure ${ns} solids (one round member per rig bone along its true axis at 0.55 of the bark radius, a joint sphere at every joint and the fixed support disc of the root plate on the ground).`,
    `Listen bei 1:${m.scale}: Body ${nb} Körper (${wood} Lofts für Stamm, Äste und Zweige, ${twigs} Zweiglofts ins Laub, ${sun} Sonnen- und ${shade} Schattenlaubkörper${m.candles.length ? `, ${m.candles.length} Blütenrispen` : ""}); Thermik ${nt} Körper (jeder Body-Körper als Verweis mit seiner Region, dazu die Transpirationsauslässe auf den Oberseiten der Laubkörper, die die Emitteranker tragen); Tragwerk ${ns} Körper (je Skelettknochen ein Rundstab entlang seiner wahren Achse mit dem 0,55-fachen Rindenradius, eine Gelenkkugel an jedem Gelenk und die eingespannte Auflagerscheibe des Wurzeltellers auf dem Boden).`,
  );
}

/** 🏗️ Builds the whole asset document of a species. */
export function buildAsset(sp: Species): { doc: Json; row: Json; report: string[] } {
  const report: string[] = [];
  const { bones, limbAxes, boneOf } = buildRig(sp);
  const models = { 50: buildModel(sp, 50, limbAxes, boneOf) } as Record<Scale, Model>;
  const box = boundsOf(models[50]);
  for (const s of SCALES.filter((x) => x !== 50)) {
    const stretch: Stretch = { xn: 1, xp: 1, zn: 1, zp: 1, y: 1 };
    for (let round = 0; round < 12; round++) {
      models[s] = buildModel(sp, s, limbAxes, boneOf, stretch);
      const b = boundsOf(models[s]);
      stretch.xn *= box.min[0] / b.min[0];
      stretch.xp *= box.max[0] / b.max[0];
      stretch.zn *= box.min[2] / b.min[2];
      stretch.zp *= box.max[2] / b.max[2];
      stretch.y *= box.max[1] / b.max[1];
    }
    const b = boundsOf(models[s]);
    report.push(`1:${s} fitted to the 1:50 box: stretch ${JSON.stringify(Object.fromEntries(Object.entries(stretch).map(([k, v]) => [k, r4(v)])))}, residual mm ${[b.min[0] - box.min[0], b.max[0] - box.max[0], b.max[1] - box.max[1], b.min[2] - box.min[2], b.max[2] - box.max[2]].map((v) => Math.round(v * 1000)).join(" ")}`);
  }
  const candleBones: string[] = [];
  const asset0: Json = { rig: { kind: "botanical", root: sp.stem.bones[0]!.id, bones } };
  let F = frames(asset0);
  if (sp.candles) {
    const hostBones = [...new Set(SCALES.flatMap((s) => models[s].candles.map((c) => c.cbone)))];
    for (const hb of hostBones) {
      const id = `flowers-${hb}`;
      const all = SCALES.flatMap((s) => models[s].candles.filter((c) => c.cbone === hb));
      const o = mul(all.reduce((s, c) => add(s, c.points[0]!.p), [0, 0, 0] as V3), 1 / all.length);
      const pf = F[hb]!;
      const phiP = Math.atan2(pf.u[1], pf.u[0]) / D2R;
      const d = sub(o, pf.o);
      const x = d[0] * pf.u[0] + d[1] * pf.u[1], y = d[0] * pf.v[0] + d[1] * pf.v[1];
      const axisDeg = r3(90 - phiP);
      bones.push({ id, parent: hb, title: tx(`Flower panicles on ${hb}`, `Blütenrispen an ${hb}`), x: r4(x), y: r4(y), z: r4(d[2]), angleDeg: r4(90 - phiP), lengthM: 0.3, joint: { type: "prismatic", axisDeg, minM: -(sp.candles.retract + 0.02), maxM: 0.02 } });
      candleBones.push(id);
      for (const s of SCALES) for (const c of models[s].candles) if (c.cbone === hb) c.bone = id;
    }
    F = frames(asset0);
  }
  for (const b of bones) if (!F[b.id]) throw new Error(`frame of ${b.id} missing`);
  const boneById = new Map(bones.map((b) => [b.id, b] as const));
  const m50 = models[50];
  for (const b of bones) {
    const own = m50.wood.filter((t) => t.bone === b.id);
    const lobes = m50.lobes.filter((l) => l.bone === b.id);
    const cand = m50.candles.filter((c) => c.bone === b.id);
    let kg = 0;
    let c: V3 = [0, 0, 0];
    for (const t of own) {
      const st = tubeStats(t);
      const w = st.volume * sp.density;
      kg += w;
      c = add(c, mul(st.c, w));
    }
    const leafTotal = (sp.leafArea.sun + sp.leafArea.shade) * sp.foliageKgPerM2;
    const lobeVolTotal = m50.lobes.reduce((s, l) => s + lobeVolume(l), 0);
    for (const l of lobes) {
      const w = (leafTotal * lobeVolume(l)) / lobeVolTotal;
      kg += w;
      c = add(c, mul(l.c, w));
    }
    for (const q of cand) {
      kg += sp.candles!.massKg;
      c = add(c, mul(q.points[1]!.p, sp.candles!.massKg));
    }
    if (kg <= 0) continue;
    c = mul(c, 1 / kg);
    const l = toLocal(F[b.id]!, c);
    b.mass = { kg: kg >= 10 ? Math.round(kg) : r2(kg), comX: r3(l[0]), comY: r3(l[1]), distribution: "point", source: "authored-estimate", reference: "wood-handbook-2021" };
    const main = own.find((t) => t.id === b.id);
    if (main) {
      const rm = main.points[Math.floor(main.points.length / 2)]!.r;
      const rr = Math.min(rm, ...(b.id === sp.stem.bones[0]!.id ? [stemAt(sp, 1.3).r] : [rm]));
      b.section = { areaM2: Number((Math.PI * rr * rr).toPrecision(4)), secondMomentM4: Number(((Math.PI * rr ** 4) / 4).toPrecision(4)), extremeFibreM: r4(rr), material: sp.woodMaterial };
    }
  }
  const stemFn = (y: number) => stemAt(sp, y);
  const bounds = boundsOf(m50);
  const dims = { heightM: r3(bounds.max[1] - Math.max(0, bounds.min[1])), widthM: r3(bounds.max[0] - bounds.min[0]), depthM: r3(bounds.max[2] - bounds.min[2]), massKg: Math.round(bones.reduce((s, b) => s + (b.mass?.kg ?? 0), 0)) };
  report.push(`dims ${JSON.stringify(dims)} bounds ${JSON.stringify(bounds)}`);
  for (const s of SCALES) {
    const b = boundsOf(models[s]);
    report.push(`1:${s} bbox x ${r3(b.max[0] - b.min[0])} y ${r3(b.max[1])} z ${r3(b.max[2] - b.min[2])} wood ${models[s].wood.length} lobes ${models[s].lobes.length} candles ${models[s].candles.length}`);
  }
  const anchors: Json[] = [{ id: "base-support", bone: sp.stem.bones[0]!.id, x: 0, y: 0, kind: "support", title: tx("Root plate anchorage", "Verankerung des Wurzeltellers"), support: "fixed", normalDeg: 90, footprintLengthM: r2(2 * stemAt(sp, 0).r), footprintWidthM: r2(2 * stemAt(sp, 0).r), frictionCoefficient: 0.6 }];
  const pickAnchors = (m: Model, region: "sun" | "shade", n: number): Json[] => {
    const list = m.lobes.filter((l) => l.region === region && !candleBones.includes(l.bone));
    const picked: typeof list = [];
    const sorted = [...list].sort((p, q) => Math.atan2(p.c[2], p.c[0]) - Math.atan2(q.c[2], q.c[0]) || p.c[1] - q.c[1]);
    for (let k = 0; k < n && k < sorted.length; k++) picked.push(sorted[Math.floor((k * sorted.length) / n)]!);
    return picked.map((l) => {
      const p = add(l.c, mul([0, 1, 0], lobeHalf(l)[1] * 0.98));
      const top: V3 = [l.c[0], l.c[1] + l.radii[1] * l.axes[1][1] + l.radii[0] * Math.abs(l.axes[0][1]) * 0.0, l.c[2]];
      void p;
      const q = surfaceUp(l);
      return { bone: l.bone, world: q, local: toLocal(F[l.bone]!, q), lobe: l.id };
      void top;
    });
  };
  const nSun = 9, nShade = 5;
  const sunAnchors = pickAnchors(m50, "sun", nSun), shadeAnchors = pickAnchors(m50, "shade", nShade);
  sunAnchors.forEach((a, k) => anchors.push({ id: `tp-sun-${k + 1}`, bone: a.bone, x: r4(a.local[0]), y: r4(a.local[1]), z: r4(a.local[2]), kind: "emitter", title: tx(`Transpiration of the sun leaves, anchor ${k + 1}`, `Transpiration der Sonnenblätter, Anker ${k + 1}`) }));
  shadeAnchors.forEach((a, k) => anchors.push({ id: `tp-shade-${k + 1}`, bone: a.bone, x: r4(a.local[0]), y: r4(a.local[1]), z: r4(a.local[2]), kind: "emitter", title: tx(`Transpiration of the shade leaves, anchor ${k + 1}`, `Transpiration der Schattenblätter, Anker ${k + 1}`) }));
  const anchorsOf = (scale: Scale): { sun: Json[]; shade: Json[] } => {
    const m = models[scale];
    const at = (a: Json): Json => {
      const best = [...m.lobes].sort((p, q) => len(sub(p.c, a.world)) - len(sub(q.c, a.world)))[0]!;
      const q = surfaceUp(best);
      return { bone: best.bone, local: toLocal(F[best.bone]!, q) };
    };
    return { sun: sunAnchors.map(at), shade: shadeAnchors.map(at) };
  };
  const emitters = [
    { id: "transpiration-sun", kind: "transpiration", title: tx("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), anchor: "tp-sun-1", anchors: sunAnchors.map((_, k) => `tp-sun-${k + 1}`), directionDeg: r2(90 - Math.atan2(F[sunAnchors[0]!.bone]!.u[1], F[sunAnchors[0]!.bone]!.u[0]) / D2R), diameterM: sp.emitterDiameter.sun, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    { id: "transpiration-shade", kind: "transpiration", title: tx("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), anchor: "tp-shade-1", anchors: shadeAnchors.map((_, k) => `tp-shade-${k + 1}`), directionDeg: r2(90 - Math.atan2(F[shadeAnchors[0]!.bone]!.u[1], F[shadeAnchors[0]!.bone]!.u[0]) / D2R), diameterM: sp.emitterDiameter.shade, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  ];
  const woodArea = (region: string): number => m50.wood.filter((t) => t.group === region).reduce((s, t) => s + tubeStats(t).area, 0);
  const regions = sp.regions.map((r: Json) => {
    if (r.id === "trunk-bark" || r.id === "limb-bark") return { ...r, areaM2: r2(woodArea(r.id) * (r.areaFactor ?? 1)), areaFactor: undefined };
    if (r.id === "leaf-sun") return { ...r, areaM2: sp.leafArea.sun, leaf: { ...r.leaf, leafAreaIndex: r2(sp.leafArea.sun / sp.leafArea.projectionM2) } };
    if (r.id === "leaf-shade") return { ...r, areaM2: sp.leafArea.shade, leaf: { ...r.leaf, leafAreaIndex: r2(sp.leafArea.shade / sp.leafArea.projectionM2) } };
    return r;
  }).map((r: Json) => JSON.parse(JSON.stringify(r)));
  const groupsOf = (m: Model): { bone: string; area: number; plan: number; c: V3 }[] => {
    const g = new Map<string, Lobe[]>();
    for (const l of m.lobes) g.set(l.bone, [...(g.get(l.bone) ?? []), l]);
    return [...g].map(([bone, list]) => {
      const area = list.reduce((s, l) => { const h = lobeHalf(l); return s + Math.PI * h[1] * h[2]; }, 0);
      const plan = list.reduce((s, l) => { const h = lobeHalf(l); return s + Math.PI * h[0] * h[2]; }, 0);
      const vol = list.reduce((s, l) => s + lobeVolume(l), 0);
      const c = mul(list.reduce((s, l) => add(s, mul(l.c, lobeVolume(l))), [0, 0, 0] as V3), 1 / vol);
      return { bone, area, plan, c };
    });
  };
  const groups50 = groupsOf(m50);
  const frontalTotal = groups50.reduce((s, g) => s + g.area, 0);
  const planTotal = groups50.reduce((s, g) => s + g.plan, 0);
  const crownFront = outlineArea(m50, "front");
  const crownPlan = outlineArea(m50, "plan");
  report.push(`crown front ${r1(crownFront)} m2 (sum of groups ${r1(frontalTotal)}), plan ${r1(crownPlan)} m2 (sum ${r1(planTotal)})`);
  const windLoadIds: string[] = [];
  const v = (v10: number, y: number): number => v10 * Math.pow(Math.max(y, 2) / 10, 0.22);
  const loadsOf = (p: PoseSpec): Json[] => {
    const loads: Json[] = [];
    for (const g of groups50) {
      const fr = F[g.bone]!;
      const loc = toLocal(fr, g.c);
      const share = g.area / frontalTotal;
      const A = crownFront * share * sp.porosity;
      const fx = 0.5 * RHO_AIR * p.cd * A * v(p.v10, g.c[1]) ** 2;
      const title = boneById.get(g.bone)!.title as Text;
      loads.push({ id: `wind-${g.bone}`, kind: "wind", title: tx(`Drag on the foliage of ${lower(title.en)}`, `Windwiderstand am Laub: ${title.de}`), bone: g.bone, x: r3(loc[0]), y: r3(loc[1]), fxN: r1(fx), fyN: 0, source: "literature", reference: "en-1991-1-4" });
      if (!windLoadIds.includes(`wind-${g.bone}`)) windLoadIds.push(`wind-${g.bone}`);
      if (p.snowKgM2) loads.push({ id: `snow-${g.bone}`, kind: "snow", title: tx(`Wet snow on the foliage of ${lower(title.en)}`, `Nassschnee auf dem Laub: ${title.de}`), bone: g.bone, x: r3(loc[0]), y: r3(loc[1]), fxN: 0, fyN: r1(-p.snowKgM2 * G * crownPlan * (g.plan / planTotal)), source: "literature", reference: "en-1991-1-3" });
      if (p.fruitKgM2) loads.push({ id: `fruit-${g.bone}`, kind: "imposed", title: tx(`${p.fruitTitle!.en} on ${lower(title.en)}`, `${p.fruitTitle!.de}: ${title.de}`), bone: g.bone, x: r3(loc[0]), y: r3(loc[1]), fxN: 0, fyN: r1(-p.fruitKgM2 * G * crownPlan * (g.plan / planTotal)), source: "authored-estimate", reference: "roloff-2010" });
    }
    const trunk = sp.stem.bones[1]!;
    const D = 2 * stemAt(sp, (trunk.y0 + trunk.y1) / 2).r;
    loads.push({ id: "wind-stem", kind: "wind", title: tx("Drag on the clear stem", "Widerstand am Schaft"), fxN: 0, fyN: 0, distributed: { bone: trunk.id, fromM: 0, toM: r4(boneById.get(trunk.id)!.lengthM), wxNPerM: r2(0.5 * RHO_AIR * (p.cd > 0 ? sp.cdStem : 0) * D * v(p.v10, (trunk.y0 + trunk.y1) / 2) ** 2), wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" });
    return loads;
  };
  const level = (b: Bone): string => {
    if (b.joint.type === "fixed") return "root";
    if (b.joint.type === "prismatic") return "flowers";
    if (sp.stem.bones.some((s) => s.id === b.id)) return sp.stem.bones.indexOf(sp.stem.bones.find((s) => s.id === b.id)!) <= 1 ? "trunk" : "upper";
    return b.id.endsWith("-b") ? "outer" : "limb";
  };
  const poseBones = (p: PoseSpec, k: number): Json => {
    const out: Json = {};
    for (const b of bones) {
      const lv = level(b);
      if (lv === "root") continue;
      if (lv === "flowers") {
        if (!p.candlesOut) {
          const ax = b.joint.axisDeg * D2R;
          out[b.id] = { dx: -sp.candles!.retract * Math.cos(ax), dy: -sp.candles!.retract * Math.sin(ax) };
        }
        continue;
      }
      const factor = { trunk: 0.25, upper: 0.6, limb: 1, outer: 1.6 }[lv as "trunk" | "upper" | "limb" | "outer"];
      let a = -p.lean * factor * k;
      const phi = Math.atan2(F[b.id]!.u[1], F[b.id]!.u[0]);
      if (p.droop && (lv === "limb" || lv === "outer")) a += -Math.sign(Math.cos(phi)) * Math.abs(Math.cos(phi)) * p.droop * (lv === "outer" ? 1.5 : 1);
      const lim = b.joint.maxDeg ?? 10;
      a = Math.max(-lim * 0.95, Math.min(lim * 0.95, a));
      if (Math.abs(a) >= 0.005) out[b.id] = { angleDeg: r2(a) };
    }
    return out;
  };
  const allRegions = regions.map((r: Json) => r.id);
  const poses = sp.poses.map((p) => ({
    id: p.id, title: p.title, description: p.description, bones: poseBones(p, 1), ground: "authored", contacts: [],
    thermal: { regions: Object.fromEntries(allRegions.map((id: string) => [id, p.regions[id] ?? { exposedFraction: 1, sunlitFraction: 0.3 }])), emitters: { "transpiration-sun": p.emitters.sun, "transpiration-shade": p.emitters.shade } },
    loads: loadsOf(p),
  }));
  const ctx: Ctx = { sp, bones, F, models, loadsOf, windLoadIds, candleBones, poseBones };
  const clips = buildClips(ctx);
  const representations: Json = {};
  const volumes: Json = {};
  const bands: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
  const pens: Record<Scale, Json> = {
    50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
    100: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
    200: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
    500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
  };
  for (const s of SCALES) {
    const m = models[s];
    const geometry: Json = {};
    for (const view of VIEWS) geometry[view] = drawScale(sp, m, F, view, stemFn);
    const counts = Object.fromEntries(VIEWS.map((vw) => [vw, geometry[vw].length]));
    report.push(`1:${s} shapes ${JSON.stringify(counts)}`);
    const t = sp.texts.scale[s];
    representations[String(s)] = {
      status: "authored", denominator: s, validRange: { minDenominator: bands[s][0], maxDenominator: bands[s][1] }, pens: pens[s], hatches: [],
      geometry,
      decisions: {
        silhouette: t.silhouette,
        retained: tx(`Retained: ${geometry.elevation.filter((x: Json) => x.id.startsWith("wood-") || x.id.startsWith("twigs-")).length} wood outlines (one per rig bone and bark region), ${geometry.elevation.filter((x: Json) => /^(sun|shade)-/.test(x.id)).length} foliage outlines (one per carrying bone and leaf region)${m.candles.length ? `, ${geometry.elevation.filter((x: Json) => x.id.startsWith("flowers-")).length} panicle outlines` : ""}${s <= 100 ? `, ${geometry.elevation.filter((x: Json) => x.role === "texture").length} texture strokes of leaves and bark in the side elevation` : ""}, each the true view of the solids of this scale and painted in the depth order of the view.`, `Beibehalten: ${geometry.elevation.filter((x: Json) => x.id.startsWith("wood-") || x.id.startsWith("twigs-")).length} Holzumrisse (je Skelettknochen und Rindenregion einer), ${geometry.elevation.filter((x: Json) => /^(sun|shade)-/.test(x.id)).length} Laubumrisse (je tragendem Knochen und Blattregion einer)${m.candles.length ? `, ${geometry.elevation.filter((x: Json) => x.id.startsWith("flowers-")).length} Rispenumrisse` : ""}${s <= 100 ? `, ${geometry.elevation.filter((x: Json) => x.role === "texture").length} Texturstriche von Blättern und Rinde in der Seitenansicht` : ""}, jeder die wahre Ansicht der Körper dieses Maßstabs und in der Tiefenfolge der Ansicht gemalt.`),
        merged: t.merged, omitted: t.omitted, thinElements: t.thinElements,
        hatching: tx("No hatching: tone and texture strokes carry the materials.", "Keine Schraffur: Ton und Texturstriche tragen die Materialien."),
        styles: t.styles,
        overlays: tx("Every thermal region keeps its own outlines bound to the bones that carry it: bark of stem and limbs, sun and shade foliage" + (sp.candles ? " and the flower panicles" : "") + ", so sway, loads and temperatures act on the drawn parts; the structural views colour the wood outlines of every member bone.", "Jede thermische Region behält ihre eigenen, an die tragenden Knochen gebundenen Umrisse: Rinde von Stamm und Ästen, Sonnen- und Schattenlaub" + (sp.candles ? " sowie die Blütenrispen" : "") + ", sodass Schwingung, Lasten und Temperaturen an den gezeichneten Teilen wirken; die Tragwerksansichten färben die Holzumrisse jedes Stabknochens."),
      },
    };
    volumes[String(s)] = volumeOf(sp, m, F, bones, anchorsOf, sp.texts.volume[s]);
  }
  const symbols = buildSymbols(sp, models, dims);
  const materials = [...sp.materials, ...(sp.candles ? [] : [])];
  const asset = {
    id: sp.id, category: "vegetation", title: sp.title, summary: sp.summary, tags: sp.tags,
    provenance: { author: "Semio staffage plant collection worker PW-02", method: "handcrafted", created: sp.created, references: sp.references, notes: sp.notes },
    dimensions: dims, materials, rig: { kind: "botanical", root: sp.stem.bones[0]!.id, bones }, volumes, anchors, representations, symbols, poses, clips,
    thermal: { environment: sp.thermal.environment, regions, emitters, passengers: [], assumptions: sp.thermal.assumptions, operating: [] },
    structure: { gravityMS2: G, dynamics: "quasi-static", supports: ["base-support"], assumptions: sp.structureAssumptions, clinical: false },
    presentation: { defaultPose: sp.defaultPose, defaultClip: "wind-sway", defaultScale: sp.defaultScale },
    exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    completion: "authored",
  };
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const row = { id: sp.id, category: "vegetation", title: sp.title, heightM: dims.heightM, widthM: dims.widthM, powerW: 0, poses: poses.map((p) => p.id), clips: clips.map((c: Json) => c.id), representations: { "50": "authored", "100": "authored", "200": "authored", "500": "authored" }, symbols: symbols.map((s: Json) => s.id), analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction: sp.distinction, completion: "authored" };
  return { doc, row, report };
}

const lower = (s: string): string => s.charAt(0).toLowerCase() + s.slice(1);

/** ⬆️ The top surface point of a lobe (world). */
function surfaceUp(l: Lobe): V3 {
  const d: V3 = [0, 1, 0];
  const q = Math.hypot(dot3(d, l.axes[0]) / l.radii[0], dot3(d, l.axes[1]) / l.radii[1], dot3(d, l.axes[2]) / l.radii[2]);
  return add(l.c, mul(d, 1 / q));
}

/** 📐️ The area of the union of the crown lobes in a view. */
function outlineArea(m: Model, view: View): number {
  const rings = outline(m.lobes.map((l) => lobeOutline(view, l, 48)), 0.02, 0.5);
  return rings.reduce((s, ring) => s + ring.reduce((a, p, i) => a + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2, 0);
}

/** 🎞️ wind-sway (periodic, looping), gust (non-looping) and diurnal-transpiration (24 s day), plus species clips. */
function buildClips(ctx: Ctx): Json[] {
  const { sp } = ctx;
  const breeze = sp.poses.find((p) => p.id === sp.breezePose)!;
  const calm = sp.poses.find((p) => p.id === sp.calmPose)!;
  const P = sp.sway.period;
  const keys: Json[] = [];
  const level = (id: string): string => {
    const b = ctx.bones.find((x) => x.id === id)!;
    if (sp.stem.bones.some((s) => s.id === id)) return sp.stem.bones.findIndex((s) => s.id === id) <= 1 ? "trunk" : "upper";
    return b.id.endsWith("-b") ? "outer" : "limb";
  };
  const lagOf: Record<string, number> = { trunk: 0, upper: 1, limb: 1.5, outer: 2.5 };
  for (let k = 0; k <= 8; k++) {
    const t = (P * k) / 8;
    const base = ctx.poseBones(breeze, 1);
    const bones: Json = {};
    for (const [id, off] of Object.entries(base) as [string, Json][]) {
      if (off.angleDeg === undefined) {
        bones[id] = off;
        continue;
      }
      const lv = level(id);
      const amp = sp.sway.amp[lv] ?? 0.3;
      const a = off.angleDeg + amp * Math.sin((2 * Math.PI * t) / P - sp.sway.lag * lagOf[lv]!);
      bones[id] = { angleDeg: r2(a) };
    }
    keys.push({ t: r2(t), pose: breeze.id, ease: "smooth", bones });
  }
  keys[8].bones = keys[0].bones;
  const loadTrack = (gain: (t: number) => number, times: number[]): Json[] => ctx.windLoadIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: times.map((t) => ({ t, v: r3(gain(t)) })) }));
  const sway = {
    id: "wind-sway", title: tx("Steady wind sway", "Gleichmäßiges Schwanken im Wind"),
    description: tx(sp.sway.text.en.replace("%P", P.toFixed(1)), sp.sway.text.de.replace("%P", P.toFixed(1).replace(".", ","))),
    durationS: P, fps: 30, loop: true, rootMotion: "none", keys, contacts: [],
    phases: [{ id: "sway", fromS: 0, toS: P, title: tx("Sway cycle", "Schwingungszyklus") }],
    events: [{ t: 0, kind: "motion", title: tx("Cycle start at the mean wind lean", "Zyklusbeginn in der mittleren Windneigung") }],
    tracks: loadTrack((t) => 1 + 0.12 * Math.sin((2 * Math.PI * t) / P), [0, P / 4, P / 2, (3 * P) / 4, P]),
  };
  const ratio = ((sp.gust.peak / sp.gust.from) ** 2 * 0.3) / breeze.cd;
  const gt = [0, 1.2, 2.5, 3.6, 5, 6.5, 8];
  const gain = (t: number): number => {
    const rows: [number, number][] = [[0, 1], [1.2, 1 + 0.12 * (ratio - 1)], [2.5, ratio], [3.6, 1 + 0.7 * (ratio - 1)], [5, 1 + 0.15 * (ratio - 1)], [6.5, 1 + 0.03 * (ratio - 1)], [8, 1]];
    return lerpRows(rows, t);
  };
  const gustKeys: Json[] = [];
  for (const t of [0, 0.6, 1.2, 1.9, 2.5, 3.0, 3.6, 4.3, 5, 5.8, 6.5, 7.2, 8]) {
    const base = ctx.poseBones(breeze, 1);
    const bones: Json = {};
    for (const [id, off] of Object.entries(base) as [string, Json][]) {
      if (off.angleDeg === undefined) {
        bones[id] = off;
        continue;
      }
      const lv = level(id);
      const lag = { trunk: 0.45, upper: 0.3, limb: 0.2, outer: 0.1 }[lv as "trunk"]!;
      const b = ctx.bones.find((x) => x.id === id)!;
      const lim = (b.joint.maxDeg ?? 10) * 0.95;
      bones[id] = { angleDeg: r2(Math.max(-lim, off.angleDeg * gain(Math.max(0, t - lag)))) };
    }
    gustKeys.push({ t, pose: breeze.id, ease: "smooth", bones });
  }
  const gust = {
    id: "gust", title: tx("Gust", "Windböe"),
    description: tx(sp.gust.text.en.replace("%A", sp.gust.from.toFixed(1)).replace("%B", sp.gust.peak.toFixed(1)), sp.gust.text.de.replace("%A", sp.gust.from.toFixed(1).replace(".", ",")).replace("%B", sp.gust.peak.toFixed(1).replace(".", ","))),
    durationS: 8, fps: 30, loop: false, rootMotion: "none", keys: gustKeys, contacts: [],
    phases: [{ id: "rise", fromS: 0, toS: 2.5, title: tx("Gust front", "Böenfront") }, { id: "peak", fromS: 2.5, toS: 3.6, title: tx("Peak", "Spitze") }, { id: "decay", fromS: 3.6, toS: 8, title: tx("Decay and recovery", "Abklingen und Erholung") }],
    events: [{ t: 2.5, kind: "load-on", title: tx("Peak gust load", "Böenspitze") }, { t: 5, kind: "load-off", title: tx("Load back near the mean", "Last wieder nahe dem Mittelwert") }],
    tracks: loadTrack(gain, gt),
  };
  const sunlitTracks = Object.entries(sp.diurnal.sunlit).map(([id, rows]) => ({ target: `region:${id}:sunlitFraction`, ease: "smooth", keys: rows.map(([t, v]) => ({ t, v })) }));
  const diurnal = {
    id: "diurnal-transpiration", title: tx("Day of transpiration", "Transpirationstag"), description: sp.diurnal.text,
    durationS: 24, fps: 10, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: calm.id, ease: "smooth" }, { t: 12, pose: calm.id, ease: "smooth" }, { t: 24, pose: calm.id, ease: "smooth" }], contacts: [],
    phases: [{ id: "night", fromS: 0, toS: 6, title: tx("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: tx("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: tx("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: tx("Evening and night", "Abend und Nacht") }],
    events: [{ t: 6, kind: "thermal", title: tx("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: tx("Solar noon", "Sonnenhöchststand") }, { t: 18, kind: "thermal", title: tx("Sunset", "Sonnenuntergang") }],
    tracks: [
      { target: "emitter:transpiration-sun", ease: "smooth", keys: sp.diurnal.sun.map(([t, v]) => ({ t, v })) },
      { target: "emitter:transpiration-shade", ease: "smooth", keys: sp.diurnal.shade.map(([t, v]) => ({ t, v })) },
      ...sunlitTracks,
    ],
  };
  return [sway, gust, diurnal, ...(sp.extraClips ? sp.extraClips(ctx) : [])];
}

/** 🔣️ The true-extent plan symbol (crown outline of the 1:500 body, stem, main limb directions) in all three views and the nominal tree sign. */
function buildSymbols(sp: Species, models: Record<Scale, Model>, dims: Json): Json[] {
  const m = models[500];
  const geometry: Json = {};
  for (const view of VIEWS) {
    const crown = outline(m.lobes.map((l) => lobeOutline(view, l, 48)), 0.05, 4)[0]!;
    const shapes: Json[] = [{ id: "crown", bone: sp.stem.bones[0]!.id, role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", closed: true, points: crown.map((p) => [r3(p.x), r3(p.y)]) }];
    const r0 = stemAt(sp, 1.3).r;
    if (view === "plan") {
      const s = stemAt(sp, 1.3);
      shapes.push({ id: "stem", bone: sp.stem.bones[0]!.id, role: "body", z: 4, filled: true, tone: 0.25, kind: "poly", closed: true, points: Array.from({ length: 24 }, (_, i) => [r4(s.x + r0 * Math.cos((i * Math.PI) / 12)), r4(s.z + r0 * Math.sin((i * Math.PI) / 12))]) });
      sp.symbolLimbs.forEach((id, k) => {
        const t = models[200].wood.find((w) => w.id === id || w.id === `${id}-b`);
        const all = models[200].wood.filter((w) => w.id === id || w.id === `${id}-b`).flatMap((w) => w.points);
        if (!t || all.length === 0) return;
        const pts = [all[0]!, all[Math.floor(all.length / 2)]!, all[all.length - 1]!].map((q) => [r3(q.p[0]), r3(q.p[2])]);
        shapes.push({ id: `limb-${k + 1}`, bone: sp.stem.bones[0]!.id, role: "detail", z: 3, filled: false, tone: 0.35, kind: "poly", closed: false, points: pts });
      });
    } else {
      const ys = [0, 0.15, 0.4, 1.3, 2.2];
      const top = Math.min(...m.lobes.map((l) => l.c[1] - lobeHalf(l)[1])) + 0.6;
      ys.push(top);
      const side = (sgn: number) => ys.map((y) => { const s = stemAt(sp, y); const c = view === "elevation" ? s.x : s.z; return [r3(c + sgn * s.r), r3(y)]; });
      shapes.push({ id: "stem", bone: sp.stem.bones[0]!.id, role: "body", z: 3, filled: true, tone: 0.3, kind: "poly", closed: true, points: [...side(-1), ...side(1).reverse()] });
    }
    geometry[view] = shapes;
  }
  const circle = (cx: number, cy: number, r: number): string => {
    const n = 16;
    return Array.from({ length: n }, (_, i) => `${i === 0 ? "M" : "L"} ${r3(cx + r * Math.cos((2 * Math.PI * i) / n))} ${r3(cy + r * Math.sin((2 * Math.PI * i) / n))}`).join(" ") + " Z";
  };
  const sign = (view: View): Json[] => view === "plan"
    ? [{ id: "sign-circle", bone: sp.stem.bones[0]!.id, role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circle(0, 0, 1.9) }, { id: "sign-dot", bone: sp.stem.bones[0]!.id, role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: circle(0, 0, 0.35) }]
    : [{ id: "sign-circle", bone: sp.stem.bones[0]!.id, role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circle(0, 3.7, 1.9) }, { id: "sign-dot", bone: sp.stem.bones[0]!.id, role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: circle(0, 3.7, 0.35) }, { id: "sign-stem", bone: sp.stem.bones[0]!.id, role: "body", z: 1, filled: false, tone: 0.3, kind: "path", d: "M 0 0 L 0 1.8" }];
  const fmt = (x: number, de = false): string => (de ? x.toFixed(2).replace(".", ",") : x.toFixed(2));
  return [
    {
      id: `${sp.id}-symbol`, title: tx("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), description: sp.texts.symbol,
      semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 }, hatches: [], geometry,
      dimensionalNote: tx(`A conventional simplification of the tree drawn at its true extent in model metres in all three views (${fmt(dims.widthM)} m wide, ${fmt(dims.depthM)} m deep and ${fmt(dims.heightM)} m high): the outline is the crown of the 1:500 body, the stem is drawn at its real position and breast-height thickness. It is a symbol, never used silently for a pictorial scale.`, `Eine konventionelle Vereinfachung des Baums in seiner wahren Ausdehnung in Modellmetern in allen drei Ansichten (${fmt(dims.widthM, true)} m breit, ${fmt(dims.depthM, true)} m tief und ${fmt(dims.heightM, true)} m hoch): Der Umriss ist die Krone des Körpers 1:500, der Stamm ist an seiner wahren Lage und in Brusthöhendicke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.`),
    },
    {
      id: "tree-sign", title: tx("Nominal tree sign with stem centre", "Nominelles Baumzeichen mit Stammmitte"), description: tx("The conventional circle-and-dot tree sign of small-scale site plans: the dot marks the stem centre, the circle is a fixed paper size.", "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Lagepläne: Der Punkt markiert die Stammmitte, der Kreis hat eine feste Papiergröße."),
      semantics: "nominal", units: "paper-mm", denominators: [500, 200], pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, hatches: [],
      geometry: { elevation: sign("elevation"), front: sign("front"), plan: sign("plan") },
      dimensionalNote: tx("A conventional symbol in paper millimetres, not to scale: a fixed 3.8 mm tree sign, drawn as plan circle with the stem dot and as a lollipop in side and front elevation. Its size carries no dimension.", "Ein konventionelles Symbol in Papiermillimetern, nicht maßstäblich: ein festes Baumzeichen von 3,8 mm, gezeichnet als Grundrisskreis mit Stammpunkt und als Lutscher in Seiten- und Vorderansicht. Seine Größe trägt keine Abmessung."),
    },
  ];
}
