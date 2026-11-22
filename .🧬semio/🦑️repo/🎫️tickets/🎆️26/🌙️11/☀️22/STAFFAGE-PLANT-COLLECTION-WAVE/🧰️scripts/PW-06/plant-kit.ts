/** 🌱️ PW-06 plant kit: turns a species file (every trunk row, limb, branch, lobe, berry cluster and outlet stated by hand per scale in world metres: x ahead, y up, z to the left) into a complete `semio.staffage/v2` vegetation asset: planar botanical rig with lateral offsets, masses and sections from the 1:50 wood, poses and clips, thermal regions and emitters, per-scale volumes (body, thermal, structure), the 2D drawings of every scale as true orthographic views of that scale's body, the plan symbol and the manifest row.
 * It only converts and projects; the forms, numbers and texts live in the species files.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts
 * @see ../../../../../🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/authoring-v2.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { frames, toLocal, type Frame, type Json, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";
import { D2R, add, along, branch, dirOf, dot3, len, lobeOutline, mul, nearness, outline, pathOf, pointsOf, project, r4, r5, ringLobe, solidOf, sub, surfacePoint, toDrawing, tubePolygons, unit, type Lobe, type Tube, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";

export { D2R, add, along, branch, dirOf, dot3, len, mul, r4, r5, ringLobe, sub, surfacePoint, unit, type Lobe, type Tube, type V3, type View, type Json };

export type Text = { en: string; de: string };
export const tx = (en: string, de: string): Text => ({ en, de });
export const SCALES = [50, 100, 200, 500] as const;
export type Scale = (typeof SCALES)[number];
export const VIEWS: View[] = ["elevation", "front", "plan"];
const HERE = dirname(fileURLToPath(import.meta.url));
export const STAFFAGE = join(HERE, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
export const WAVE = join(HERE, "../..");
const RHO_AIR = 1.25;
const G = 9.80665;
const feature = (scale: number): number => (0.15 * scale) / 1000;

export type Group = "root" | "stem" | "leader" | "limb" | "branch" | "foliage";
export type BoneDef = { id: string; parent: string | null; title: Text; group: Group; limit: number; from: V3; to: V3; material?: string };
export type Outlet = [emitter: string, anchor: string | null, lobe: string, az: number, el: number];
export type Cluster = Lobe & { clusterOf: string };
export type ScaleModel = { scale: Scale; tubes: Tube[]; hiddenTubes: Tube[]; lobes: Lobe[]; clusters: Cluster[]; hiddenClusters: Cluster[]; outlets: Outlet[]; crownBone?: string; decisions: Text[] };
export type RegionDef = Json & { id: string };
export type PoseDef = { id: string; title: Text; description: Text; bones: Record<string, { angleDeg: number }>; thermal: Json; loads: Json[] };
export type Glyph = (c: P2, k: number) => { closed: boolean; rings: P2[][] };
export type BarkMarks = (view: View, tube: Tube, scale: Scale) => P2[][];

/** 🌳️ Everything a species file states. */
export type Species = {
  id: string; title: Text; summary: Text; tags: string[]; provenance: Json; materials: Json[];
  bones: BoneDef[]; models: Record<Scale, ScaleModel>;
  woodDensity: number; leafMassKgM2: number; barkFraction: number;
  regionOfTube: (t: Tube) => string; regionOfLobe: (l: Lobe) => string; fruitRegion?: string;
  regions: RegionDef[]; emitters: Json[]; environment: Json; thermalAssumptions: Text[]; structureAssumptions: Text[];
  poses: (ctx: Ctx) => PoseDef[]; clips: (ctx: Ctx) => Json[]; presentation: Json;
  drawing: { decisions: Record<Scale, Json>; glyph: Glyph; bark: BarkMarks; arcScales: Scale[]; tones: { root: number; stem: number; limb: number } };
  symbol: { id: string; title: Text; description: Text };
  distinction: Text; anchorsTitle: (emitter: string, n: number) => Text;
};

export type Ctx = { spec: Species; F: Record<string, Frame>; bones: Json[]; boneOf: (id: string) => Json; foliage: string[]; wood: string[]; windArea: Record<string, number>; planArea: Record<string, number>; centroid: Record<string, V3>; stems: string[]; group: (id: string) => Group };

const deg = (r: number): number => (r * 180) / Math.PI;
const norm180 = (a: number): number => ((((a + 180) % 360) + 360) % 360) - 180;

/** 🦴️ The planar botanical rig: every bone lies in the sagittal plane at the lateral position of its own origin; the lateral offset of a child is `z`. */
export function rigOf(defs: BoneDef[]): Json[] {
  const world: Record<string, { o: V3; th: number }> = {};
  return defs.map((b) => {
    const d = [b.to[0] - b.from[0], b.to[1] - b.from[1]];
    const L = Math.hypot(d[0]!, d[1]!);
    const th = b.group === "foliage" ? (b.parent ? world[b.parent]!.th : 90) : deg(Math.atan2(d[1]!, d[0]!));
    world[b.id] = { o: b.from, th };
    const joint = b.group === "root" ? { type: "fixed" } : { type: "revolute", minDeg: -b.limit, maxDeg: b.limit };
    if (!b.parent) return { id: b.id, parent: null, title: b.title, x: 0, y: 0, angleDeg: r4(th), lengthM: r4(L), joint };
    const p = world[b.parent]!;
    const [c, s] = [Math.cos(p.th * D2R), Math.sin(p.th * D2R)];
    const [dx, dy] = [b.from[0] - p.o[0], b.from[1] - p.o[1]];
    const dz = b.from[2] - p.o[2];
    return { id: b.id, parent: b.parent, title: b.title, x: r4(dx * c + dy * s), y: r4(-dx * s + dy * c), ...(Math.abs(dz) > 1e-4 ? { z: r4(dz) } : {}), angleDeg: r4(norm180(th - p.th)), lengthM: b.group === "foliage" ? 0.3 : r4(L), joint };
  });
}

const frustum = (r0: number, r1: number, h: number): number => (Math.PI * h * (r0 * r0 + r0 * r1 + r1 * r1)) / 3;
const tubeVolume = (t: Tube): number => t.points.slice(1).reduce((s, q, i) => s + frustum(t.points[i]!.r, q.r, len(sub(q.p, t.points[i]!.p))), 0);
const tubeCentroid = (t: Tube): V3 => {
  let [m, c] = [0, [0, 0, 0] as V3];
  t.points.slice(1).forEach((q, i) => {
    const v = frustum(t.points[i]!.r, q.r, len(sub(q.p, t.points[i]!.p)));
    m += v;
    c = add(c, mul(add(q.p, t.points[i]!.p), v / 2));
  });
  return mul(c, 1 / (m || 1));
};
/** 🪵️ Lateral bark surface of a tube (frustum mantles). */
export const tubeArea = (t: Tube): number => t.points.slice(1).reduce((s, q, i) => s + Math.PI * (t.points[i]!.r + q.r) * Math.hypot(len(sub(q.p, t.points[i]!.p)), q.r - t.points[i]!.r), 0);
const lobeVolume = (l: Lobe): number => (4 / 3) * Math.PI * l.radii[0] * l.radii[1] * l.radii[2];
const ringArea = (ring: P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;
const rings2Area = (rings: P2[][]): number => rings.reduce((s, r) => s + ringArea(r), 0);

/** 📦️ The world extent of a scale's body. */
export function extentOf(m: ScaleModel): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const t of m.tubes) for (const q of t.points) for (let k = 0; k < 3; k++) {
    min[k] = Math.min(min[k]!, q.p[k]! - (k === 1 ? 0 : q.r));
    max[k] = Math.max(max[k]!, q.p[k]! + q.r);
  }
  for (const l of [...m.lobes, ...m.clusters]) for (let k = 0; k < 3; k++) {
    const e = Math.sqrt(l.axes.reduce((s, a, i) => s + (a[k]! * l.radii[i]!) ** 2, 0));
    min[k] = Math.min(min[k]!, l.c[k]! - e);
    max[k] = Math.max(max[k]!, l.c[k]! + e);
  }
  min[1] = Math.max(0, min[1]!);
  return { min, max };
}

const inLobe = (l: Lobe, p: V3, s = 1): boolean => Math.hypot(...l.axes.map((a, i) => dot3(sub(p, l.c), a) / (l.radii[i]! * s))) < 1;
const nearTube = (t: Tube, p: V3, extra: number): boolean => t.points.some((q, i) => {
  if (i === 0) return false;
  const a = t.points[i - 1]!.p, b = q.p, ab = sub(b, a);
  const k = Math.max(0, Math.min(1, dot3(sub(p, a), ab) / (dot3(ab, ab) || 1)));
  return len(sub(p, add(a, mul(ab, k)))) < t.points[i - 1]!.r + (q.r - t.points[i - 1]!.r) * k + extra;
});

/** 🔗️ Lobes and clusters that touch nothing else of the body (they would float). */
export function floating(m: ScaleModel): string[] {
  const out: string[] = [];
  const parts = [...m.lobes, ...m.clusters];
  for (const l of parts) {
    const samples: V3[] = [];
    for (let i = 0; i < 6; i++) for (let j = 0; j < 12; j++) {
      const th = (Math.PI * (i + 0.5)) / 6, ph = (2 * Math.PI * j) / 12;
      const d: V3 = [Math.sin(th) * Math.cos(ph), Math.cos(th), Math.sin(th) * Math.sin(ph)];
      samples.push(add(l.c, l.axes.reduce((s, a, k) => add(s, mul(a, d[k]! * l.radii[k]! * 0.98)), [0, 0, 0] as V3)));
    }
    samples.push(l.c);
    const touches = samples.some((p) => parts.some((o) => o !== l && inLobe(o, p)) || m.tubes.some((t) => nearTube(t, p, 0.002))) || m.tubes.some((t) => t.points.some((q) => inLobe(l, q.p)));
    if (!touches) out.push(l.id);
  }
  return out;
}

/** 🧱️ The smallest radius of wood at a scale: the larger of the smallest 3D feature and the smallest printable 2D feature (0.2 mm), plus 15 %. */
export const minRadius = (scale: number): number => (1.15 * Math.max(0.15, 0.2) * scale) / 2000;

/** 🪵️ The tubes of a scale with every radius raised to {@link minRadius} (generalisation of the thin tips). */
export const clamp = (scale: number, tubes: Tube[]): Tube[] => tubes.map((t) => ({ ...t, points: t.points.map((q) => ({ p: q.p, r: Math.max(q.r, minRadius(scale)) })) }));

/** 📍️ The point of a stem tube's axis at height `y`. */
export function axisAt(t: Tube, y: number): V3 {
  const i = Math.max(0, t.points.findIndex((q) => q.p[1] >= y) - 1);
  const [a, b] = [t.points[i]!, t.points[Math.min(t.points.length - 1, i + 1)]!];
  const k = b.p[1] === a.p[1] ? 0 : (y - a.p[1]) / (b.p[1] - a.p[1]);
  return [a.p[0] + (b.p[0] - a.p[0]) * k, y, a.p[2] + (b.p[2] - a.p[2]) * k];
}

/** 🪵️ Bark marks along a tube in a view: every `step` metres of its axis, `mark(n, r)` gives the stroke as [across, along] offsets (metres) about the axis point, `r` the local radius. */
export function barkMarks(view: View, t: Tube, step: number, mark: (n: number, r: number) => [number, number][]): P2[][] {
  const proj = (p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : { x: p[2], y: p[1] });
  const total = t.points.slice(1).reduce((acc, q, i) => acc + len(sub(q.p, t.points[i]!.p)), 0);
  const out: P2[][] = [];
  for (let n = 0, s = 0.1; s < total - 0.1; n++, s += step) {
    const f = s / total;
    const [a, b] = [proj(along(t, f)), proj(along(t, Math.min(1, f + 0.01)))];
    const d = Math.hypot(b.x - a.x, b.y - a.y) || 1;
    const [tg, nm] = [{ x: (b.x - a.x) / d, y: (b.y - a.y) / d }, { x: -(b.y - a.y) / d, y: (b.x - a.x) / d }];
    const i = Math.min(t.points.length - 2, Math.floor(f * (t.points.length - 1)));
    const r = t.points[i]!.r + (t.points[i + 1]!.r - t.points[i]!.r) * (f * (t.points.length - 1) - i);
    out.push(mark(n, r).map(([u, v]) => ({ x: a.x + nm.x * u + tg.x * v, y: a.y + nm.y * u + tg.y * v })));
  }
  return out;
}

/** 🌳️ A stem part: rows [y, r, dx, dz]. */
export const stemTube = (id: string, bone: string, material: string, tone: number, group: string, rows: [number, number, number, number][]): Tube => ({ kind: "tube", id, bone, material, tone, group, points: rows.map(([y, r, dx, dz]) => ({ p: [dx, y, dz] as V3, r })) });

/** 🍃️ A ring of lobes at height `y`: `n` lobes from azimuth `az0`, semi-axes `a` radial, `b` vertical, `c` tangential, drooping by `tilt`, their outer faces at `k` times the crown envelope. */
export type RingDef = { id: string; y: number; n: number; az0: number; a: number; b: number; c: number; tilt?: number; k?: number };
export function ringLobes(rings: RingDef[], envelope: (az: number, y: number) => number, bone: (az: number, y: number) => string, material: (bone: string) => string, group = "sun"): Lobe[] {
  return rings.flatMap((r) => Array.from({ length: r.n }, (_, i) => {
    const az = r.az0 + (360 * i) / r.n;
    const b = bone(az, r.y);
    const reach = Math.max(envelope(az, r.y) * (r.k ?? 1), Math.hypot(r.a, r.b) * 0.6);
    return ringLobe(`${r.id}-${i + 1}`, b, material(b), group === "shade" ? 0.45 : 0.6, group, az, reach, r.y, r.a, r.b, r.c, r.tilt ?? 0);
  }));
}

/** 🎯️ A free lobe centred at `c` with world-axis semi-axes. */
export const freeLobe = (id: string, bone: string, material: string, group: string, c: V3, radii: V3): Lobe => ({ kind: "lobe", id, bone, material, tone: group === "shade" ? 0.45 : 0.6, group, c, axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii });

/** 🍒️ A berry cluster hanging on the face of a lobe in the direction (`az`, `el`), its centre `sink` times its radius beyond the surface (negative: sunk into the foliage). */
export function clusterOn(id: string, l: Lobe, az: number, el: number, radii: V3, material: string, sink = -0.25): Cluster {
  const d = dirOf(az, el);
  const p = surfacePoint(l, d);
  return { kind: "lobe", id, bone: l.bone, material, tone: 0.22, group: "fruit", c: add(p, mul(d, radii[0] * sink)), axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii, clusterOf: l.id };
}

/** 🧭️ The context of one species: rest frames, foliage and wood bones, their wind and plan areas and centroids from the 1:50 body. */
export function contextOf(spec: Species): Ctx {
  const bones = rigOf(spec.bones);
  const F = frames({ rig: { kind: "botanical", root: bones[0].id, bones } });
  const groupOf: Record<string, Group> = Object.fromEntries(spec.bones.map((b) => [b.id, b.group]));
  const m = spec.models[50];
  const foliage = spec.bones.filter((b) => b.group === "foliage").map((b) => b.id);
  const wood = spec.bones.filter((b) => b.group !== "foliage").map((b) => b.id);
  const windArea: Record<string, number> = {}, planArea: Record<string, number> = {}, centroid: Record<string, V3> = {};
  for (const id of foliage) {
    const mine = m.lobes.filter((l) => l.bone === id);
    if (mine.length === 0) throw new Error(`foliage bone ${id} has no lobe at 1:50`);
    windArea[id] = rings2Area(outline(mine.map((l) => lobeOutline("front", l, 48)), 0.01, 0.01));
    planArea[id] = rings2Area(outline(mine.map((l) => lobeOutline("plan", l, 48)), 0.01, 0.01));
    const v = mine.reduce((s, l) => s + lobeVolume(l), 0);
    centroid[id] = mul(mine.reduce((s, l) => add(s, mul(l.c, lobeVolume(l))), [0, 0, 0] as V3), 1 / v);
  }
  for (const id of wood) {
    const mine = m.tubes.filter((t) => t.bone === id);
    const v = mine.reduce((s, t) => s + tubeVolume(t), 0);
    centroid[id] = v > 0 ? mul(mine.reduce((s, t) => add(s, mul(tubeCentroid(t), tubeVolume(t))), [0, 0, 0] as V3), 1 / v) : F[id]!.o;
  }
  return { spec, F, bones, boneOf: (id) => bones.find((b) => b.id === id), foliage, wood, windArea, planArea, centroid, stems: spec.bones.filter((b) => b.group === "stem" || b.group === "leader" || b.group === "root").map((b) => b.id), group: (id) => groupOf[id]! };
}

/** ⚖️ Masses (green wood of the 1:50 body, fresh leaves and twigs by lobe volume) and solid circular sections of the wood bones. */
function massesAndSections(ctx: Ctx): number {
  const { spec, F, bones } = ctx;
  const m = spec.models[50];
  const leafTotal = spec.regions.filter((r) => r.model === "leaf").reduce((s, r) => s + r.areaM2, 0) * spec.leafMassKgM2;
  const lobeTotal = m.lobes.reduce((s, l) => s + lobeVolume(l), 0);
  let total = 0;
  for (const b of bones) {
    const def = spec.bones.find((d) => d.id === b.id)!;
    let kg = 0;
    if (def.group === "foliage") kg = (m.lobes.filter((l) => l.bone === b.id).reduce((s, l) => s + lobeVolume(l), 0) / lobeTotal) * leafTotal;
    else kg = m.tubes.filter((t) => t.bone === b.id).reduce((s, t) => s + tubeVolume(t), 0) * spec.woodDensity;
    const c = toLocal(F[b.id]!, ctx.centroid[b.id]!);
    b.mass = { kg: Math.round(kg * 100) / 100, comX: r4(c[0]), comY: r4(c[1]), distribution: "point", source: "authored-estimate", reference: "wood-handbook-2021" };
    total += b.mass.kg;
    if (def.group !== "foliage") {
      const t = m.tubes.find((x) => x.id === b.id);
      if (!t) throw new Error(`wood bone ${b.id} needs a 1:50 tube of its own id`);
      const q = t.points.find((p) => toLocal(F[b.id]!, p.p)[0] >= -1e-6) ?? t.points[0]!;
      const d = 2 * q.r * (1 - spec.barkFraction);
      b.section = { areaM2: r5((Math.PI * d * d) / 4), secondMomentM4: Number(((Math.PI * d ** 4) / 64).toPrecision(4)), extremeFibreM: r4(d / 2), material: def.material ?? spec.materials.find((x: Json) => x.elasticModulusPa)!.id, title: tx(`Solid section, d = ${Math.round(d * 100)} cm`, `Vollquerschnitt, d = ${Math.round(d * 100)} cm`) };
    }
  }
  return Math.round(total);
}

/** 🌬️ Wind drag F = ½ ρ cd A v(z)² on every foliage mass (front area of its 1:50 lobes, power-law profile v10 (z/10)^0.22) and line loads on the stems. */
export function windLoads(ctx: Ctx, v10: number, cd: number): Json[] {
  const v = (z: number): number => v10 * (Math.max(2, z) / 10) ** 0.22;
  const out: Json[] = [];
  for (const id of ctx.foliage) {
    const c = ctx.centroid[id]!;
    const l = toLocal(ctx.F[id]!, c);
    const fx = 0.5 * RHO_AIR * cd * ctx.windArea[id]! * v(c[1]) ** 2;
    out.push({ id: `wind-${id}`, kind: "wind", title: tx(`Wind drag on foliage mass ${id}`, `Winddruck auf Laubmasse ${id}`), bone: id, x: r4(l[0]), y: r4(l[1]), fxN: Math.round(fx * 10) / 10, fyN: 0, source: "literature", reference: "en-1991-1-4" });
  }
  for (const id of ctx.stems.filter((s) => ctx.group(s) !== "root")) {
    const b = ctx.boneOf(id);
    const d = 2 * Math.sqrt(b.section.areaM2 / Math.PI);
    const yMid = ctx.F[id]!.o[1] + (b.lengthM / 2) * ctx.F[id]!.u[1];
    const w = 0.5 * RHO_AIR * 1.0 * d * v(yMid) ** 2;
    out.push({ id: `wind-${id}`, kind: "wind", title: tx(`Wind drag on the stem, ${id}`, `Winddruck auf den Stamm, ${id}`), fxN: 0, fyN: 0, distributed: { bone: id, fromM: 0, toM: b.lengthM, wxNPerM: r4(w * ctx.F[id]!.u[1]), wyNPerM: r4(-w * ctx.F[id]!.u[0]) }, source: "literature", reference: "en-1991-1-4" });
  }
  return out;
}

/** ❄️ A vertical load per foliage mass: `kgM2` on its 1:50 plan area (snow, ice), or a share of `totalKg` by plan area (fruit, blossom). */
export function verticalLoads(ctx: Ctx, kind: "snow" | "carried", prefix: string, title: (id: string) => Text, reference: string, kgM2: number | null, totalKg?: number, only?: string[]): Json[] {
  const ids = only ?? ctx.foliage;
  const sum = ids.reduce((s, id) => s + ctx.planArea[id]!, 0);
  return ids.map((id) => {
    const l = toLocal(ctx.F[id]!, ctx.centroid[id]!);
    const kg = kgM2 !== null ? kgM2 * ctx.planArea[id]! : (totalKg! * ctx.planArea[id]!) / sum;
    return { id: `${prefix}-${id}`, kind, title: title(id), bone: id, x: r4(l[0]), y: r4(l[1]), fxN: 0, fyN: -Math.round(kg * G * 10) / 10, source: "authored-estimate", reference };
  });
}

/** 🎚️ Bone offsets: `lean` turns every bone of a group clockwise in the side elevation (downwind for wind along +x); `droop` lowers the tip of every bone of a group by its angle. */
export function offsets(ctx: Ctx, lean: Partial<Record<Group, number>>, droop: Partial<Record<Group, number>> = {}): Record<string, { angleDeg: number }> {
  const out: Record<string, { angleDeg: number }> = {};
  for (const b of ctx.bones) {
    const g = ctx.group(b.id);
    const ux = ctx.F[b.id]!.u[0];
    const tipSide = g === "foliage" ? Math.sign(ctx.centroid[b.id]![0] - ctx.F[b.id]!.o[0] || ux) : Math.sign(ux);
    let a = -(lean[g] ?? 0) + (droop[g] ?? 0) * -tipSide;
    const lim = b.joint.maxDeg ?? 0;
    a = Math.max(-0.9 * lim, Math.min(0.9 * lim, a));
    if (Math.abs(a) > 1e-3) out[b.id] = { angleDeg: Math.round(a * 100) / 100 };
  }
  return out;
}

/** 🌡️ A thermal binding of every region and emitter. */
export function binding(spec: Species, sunlit: Record<string, number>, emitters: Record<string, number>, wet = 0, contact: Record<string, { fraction: number; tempC: number }> = {}): Json {
  const regions: Json = {};
  for (const r of spec.regions) regions[r.id] = { exposedFraction: 1, sunlitFraction: sunlit[r.id] ?? 0, ...(wet > 0 ? { wettedness: wet } : {}), ...(contact[r.id] ? { contact: contact[r.id] } : {}) };
  return { regions, emitters };
}

/** 🌊️ The wind-sway keys: `T` seconds, the fresh-breeze lean plus a sinusoid of `amp` times that lean, each group lagging by `lag` periods. */
export function swayKeys(ctx: Ctx, pose: string, lean: Partial<Record<Group, number>>, amp: number, lag: Partial<Record<Group, number>>, T: number, n = 8): Json[] {
  const keys: Json[] = [];
  for (let i = 0; i <= n; i++) {
    const t = (T * i) / n;
    const scaled: Partial<Record<Group, number>> = {};
    for (const [g, a] of Object.entries(lean) as [Group, number][]) scaled[g] = a * (1 + amp * Math.sin(2 * Math.PI * (i / n - (lag[g] ?? 0))));
    keys.push({ t: r4(t), pose, ease: "smooth", bones: offsets(ctx, scaled) });
  }
  return keys;
}

/** 💨️ Gust keys every 0.5 s over 8 s: the lean follows the load factor `g(t)` of the gust (0.05 to `peak` of the fresh-breeze load) to the power 0.8, capped by the gale lean. */
export const GUST: [number, number][] = [[0, 0.05], [0.5, 0.15], [1, 0.3], [1.5, 0.55], [2, 1.4], [2.5, 2.75], [3, 2.2], [3.5, 1.5], [4, 1], [4.5, 0.8], [5, 0.5], [5.5, 0.38], [6, 0.28], [6.5, 0.2], [7, 0.12], [7.5, 0.08], [8, 0.05]];
export function gustKeys(ctx: Ctx, pose: string, lean: Partial<Record<Group, number>>, cap: Partial<Record<Group, number>>): Json[] {
  return GUST.map(([t, g]) => {
    const scaled: Partial<Record<Group, number>> = {};
    for (const [k, a] of Object.entries(lean) as [Group, number][]) scaled[k] = Math.min(a * g ** 0.8, 0.7 * (cap[k] ?? a * 3));
    return { t, pose, ease: "smooth", bones: offsets(ctx, scaled) };
  });
}

const SUN: [number, number][] = [[0, 0], [6, 0], [8, 0.43], [10, 0.79], [12, 1], [14, 0.79], [16, 0.36], [18, 0], [24, 0]];
/** ☀️ The diurnal clip: 24 s for 24 h, sunlit fractions of every region follow the sun, transpiration strengths follow the given profile. */
export function diurnalClip(spec: Species, pose: string, noonSunlit: Record<string, number>, emitterProfile: Record<string, [number, number][]>): Json {
  const tracks: Json[] = Object.entries(emitterProfile).map(([id, keys]) => ({ target: `emitter:${id}`, ease: "smooth", keys: keys.map(([t, v]) => ({ t, v })) }));
  for (const r of spec.regions) tracks.push({ target: `region:${r.id}:sunlitFraction`, ease: "smooth", keys: SUN.map(([t, k]) => ({ t, v: r4(k * (noonSunlit[r.id] ?? 0)) })) });
  return {
    id: "diurnal-transpiration", title: tx("Day of transpiration", "Transpirationstag"),
    description: tx("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00) in calm summer weather: the sunlit fraction of every region and the transpiration strength follow the sun, with the stomata nearly closed at night; leaf and bark temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Sommerwetter: Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Spaltöffnungen fast geschlossen; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    durationS: 24, fps: 10, loop: true, rootMotion: "none", keys: [{ t: 0, pose, ease: "smooth" }, { t: 12, pose, ease: "smooth" }, { t: 24, pose, ease: "smooth" }], contacts: [],
    phases: [{ id: "night", fromS: 0, toS: 6, title: tx("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: tx("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: tx("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: tx("Evening and night", "Abend und Nacht") }],
    events: [{ t: 6, kind: "thermal", title: tx("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: tx("Solar noon, peak transpiration", "Sonnenhöchststand, größte Transpiration") }, { t: 18, kind: "thermal", title: tx("Sunset", "Sonnenuntergang") }],
    tracks,
  };
}

/** 🧊️ An ellipsoid part (lobe or berry cluster) as a bone-local solid. */
const ellipsoidSolid = (l: Lobe, F: Record<string, Frame>, role = "body"): Json => ({ ...solidOf(l, F), role });

/** 🧊️ The volume of one scale: body solids, thermal references and outlet lenses, structure members, joints, support and masses. */
function volumeOf(ctx: Ctx, m: ScaleModel, anchorsOut: Record<string, { bone: string; local: V3 }>): Json {
  const { spec, F } = ctx;
  const f = feature(m.scale);
  const hidden = (l: Lobe): boolean => spec.regions.find((r) => r.id === spec.regionOfLobe(l))?.hidden !== undefined;
  const body: Json[] = [...m.tubes.map((t) => solidOf(t, F)), ...m.lobes.filter((l) => !hidden(l)).map((l) => ellipsoidSolid(l, F)), ...m.clusters.map((c) => ellipsoidSolid(c, F, "detail"))];
  const thermal: Json[] = [];
  for (const t of m.tubes) thermal.push({ id: `th-${t.id}`, of: t.id, region: spec.regionOfTube(t) });
  for (const l of m.lobes) thermal.push(hidden(l) ? { ...ellipsoidSolid({ ...l, id: `th-${l.id}` }, F), region: spec.regionOfLobe(l) } : { id: `th-${l.id}`, of: l.id, region: spec.regionOfLobe(l) });
  for (const c of m.clusters) thermal.push({ id: `th-${c.id}`, of: c.id, region: spec.fruitRegion });
  for (const t of m.hiddenTubes) thermal.push({ ...solidOf({ ...t, id: `th-${t.id}` }, F), region: spec.regionOfTube(t) });
  for (const c of m.hiddenClusters) thermal.push({ ...ellipsoidSolid({ ...c, id: `th-${c.id}` }, F, "detail"), region: spec.fruitRegion });
  m.outlets.forEach(([emitter, anchor, lobeId, az, el], i) => {
    const l = m.lobes.find((x) => x.id === lobeId);
    if (!l) throw new Error(`1:${m.scale} outlet on unknown lobe ${lobeId}`);
    const d = dirOf(az, el);
    const p = surfacePoint(l, d);
    const q = sub(p, l.c);
    const n = unit(l.axes.reduce((s, a, k) => add(s, mul(a, dot3(q, a) / l.radii[k]! ** 2)), [0, 0, 0] as V3));
    const side = unit(Math.abs(n[1]) > 0.98 ? [1, 0, 0] : [n[2], 0, -n[0]]);
    const third: V3 = [side[1] * n[2] - side[2] * n[1], side[2] * n[0] - side[0] * n[2], side[0] * n[1] - side[1] * n[0]];
    const rho = Math.min(l.radii[0], l.radii[2]) * 0.5;
    const h = Math.max(1.2 * f, 0.03);
    const lens: Lobe = { ...l, id: `th-${anchor ?? `${emitter}-${m.scale}-${i + 1}`}`, c: sub(p, mul(n, h)), axes: [side, n, third], radii: [rho, h, rho] };
    thermal.push({ ...solidOf(lens, F), role: "detail", region: spec.regionOfLobe(l), tags: [emitter] });
    if (anchor) anchorsOut[anchor] = { bone: l.bone, local: toLocal(F[l.bone]!, add(p, mul(n, 0.04))) };
  });
  const structure: Json[] = [];
  const woodMat = spec.materials.find((x: Json) => x.elasticModulusPa)!.id;
  const bark = (r: number): number => r * spec.barkFraction;
  const members = [...m.tubes, ...m.hiddenTubes].filter((t) => t.id === t.bone);
  for (const t of members) {
    const b = ctx.boneOf(t.bone);
    const declared = Math.sqrt(b.section.areaM2 / Math.PI);
    const pts = m.scale === 500 && t.group !== "root" ? t.points.map((q) => ({ p: q.p, r: Math.min(declared, q.r - bark(q.r)) })) : t.points.map((q) => ({ p: q.p, r: q.r - bark(q.r) }));
    const thin = Math.min(...pts.map((q) => 2 * q.r));
    const L = len(sub(pts[pts.length - 1]!.p, pts[0]!.p));
    const slender = thin < f && L >= 5 * Math.max(...pts.map((q) => 2 * q.r)) && L >= 2 * f;
    structure.push({ ...solidOf({ ...t, id: `member-${t.id}`, material: woodMat, tone: 0.4, points: pts, flags: slender ? { slender: true } : undefined } as Tube, F), member: t.bone, part: "member" });
  }
  for (const t of members) {
    if (ctx.group(t.bone) === "root") continue;
    const first = t.points.find((q) => toLocal(F[t.bone]!, q.p)[0] >= -1e-6) ?? t.points[0]!;
    const declared = Math.sqrt(ctx.boneOf(t.bone).section.areaM2 / Math.PI);
    const r = (m.scale === 500 ? Math.min(declared, first.r - bark(first.r)) : first.r - bark(first.r)) * 1.1;
    if (2 * r < f) continue;
    structure.push({ id: `joint-${t.bone}`, kind: "ellipsoid", bone: t.bone, material: woodMat, role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: [0, 0, 0] }, member: t.bone, part: "joint" });
  }
  const root = spec.bones[0]!;
  const rootTube = m.tubes.find((t) => t.id === root.id)!;
  const plate = Math.max(0.04, f);
  structure.push({ id: "support-base-support", kind: "revolve", bone: root.id, material: woodMat, role: "detail", tone: 0.25, profile: [[r5(-plate), r5(rootTube.points[0]!.r * 1.1)], [0, r5(rootTube.points[0]!.r * 1.1)]], axis: [1, 0, 0], origin: [0, 0, 0], member: root.id, part: "support", anchor: "base-support" });
  for (const id of ctx.wood) {
    const t = members.find((x) => x.id === id);
    if (!t) continue;
    const b = ctx.boneOf(id);
    const c = toLocal(F[id]!, ctx.centroid[id]!);
    const rr = Math.min(...t.points.map((q) => q.r)) * 0.7;
    if (2 * rr < f) continue;
    const along = ctx.group(id) === "stem" || ctx.group(id) === "leader" ? Math.max(rr, Math.min(0.2 * b.lengthM, 0.6)) : rr;
    structure.push({ id: `mass-${id}`, kind: "ellipsoid", bone: id, material: woodMat, role: "detail", tone: 0.3, radii: [r5(along), r5(rr), r5(rr)], transform: { translation: [r5(c[0]), r5(c[1]), r5(c[2])] }, member: id, part: "mass" });
  }
  if (m.scale !== 500) for (const id of ctx.foliage) {
    const b = ctx.boneOf(id);
    const r = Math.max(f * 0.6, Math.min(0.3, Math.cbrt((3 * b.mass.kg) / (4 * Math.PI * 600))));
    const c = toLocal(F[id]!, ctx.centroid[id]!);
    structure.push({ id: `mass-${id}`, kind: "ellipsoid", bone: id, material: spec.materials.find((x: Json) => x.id.includes("leaf"))!.id, role: "detail", tone: 0.3, radii: [r5(r), r5(r), r5(r)], transform: { translation: [r5(c[0]), r5(c[1]), r5(c[2])] }, member: b.parent, part: "mass" });
  }
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions: m.decisions };
}

type Ring = P2[];
const segDist = (p: P2, a: P2, b: P2): number => {
  const [dx, dy] = [b.x - a.x, b.y - a.y];
  const k = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
  return Math.hypot(p.x - a.x - k * dx, p.y - a.y - k * dy);
};
const inside = (p: P2, ring: Ring): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};
const insideRings = (p: P2, rings: Ring[]): boolean => rings.filter((r) => inside(p, r)).length % 2 === 1;
const edgeDist = (p: P2, rings: Ring[]): number => Math.min(...rings.map((r) => Math.min(...r.map((a, i) => segDist(p, a, r[(i + 1) % r.length]!)))));
const hitsLobe = (l: Lobe, p: V3, d: V3): boolean => {
  const o = l.axes.map((a, i) => dot3(sub(p, l.c), a) / l.radii[i]!);
  const v = l.axes.map((a, i) => dot3(d, a) / l.radii[i]!);
  const [A, B, C] = [v[0]! ** 2 + v[1]! ** 2 + v[2]! ** 2, o[0]! * v[0]! + o[1]! * v[1]! + o[2]! * v[2]!, o[0]! ** 2 + o[1]! ** 2 + o[2]! ** 2 - 1];
  if (C < 0) return true;
  const disc = B * B - A * C;
  return disc > 0 && -B + Math.sqrt(disc) > 0;
};
const VIEW_DIR: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** ✏️ The drawings of one scale: every bone's wood and foliage as the true view of its solids, visible berry clusters, and the bark, leaf and arc marks of the scale. */
function drawScale(ctx: Ctx, m: ScaleModel): { geometry: Json; counts: Json } {
  const { spec, F } = ctx;
  const f = feature(m.scale);
  const tol = f / 4;
  const paper = m.scale / 1000;
  const geometry: Json = {};
  const counts: Json = {};
  const tones = spec.drawing.tones;
  const toneOf = (bone: string, kind: "wood" | "lobe" | "fruit", region: string): number => kind === "fruit" ? 0.22 : kind === "lobe" ? (region.includes("shade") ? 0.45 : 0.6) : ctx.group(bone) === "root" ? tones.root : ctx.group(bone) === "stem" || ctx.group(bone) === "leader" ? tones.stem : tones.limb;
  for (const view of VIEWS) {
    const shapes: Json[] = [];
    const filled: Ring[][] = [];
    const set = (base: Json, rings: Ring[], closed = true): Json | null => {
      if (rings.length === 0 || rings[0]!.length < 2) return null;
      const local = toDrawing(view, F[base.bone]!, rings);
      if (closed && local.length === 1 && view !== "elevation") return { ...base, kind: "poly", points: pointsOf(local[0]!), closed: true };
      return { ...base, kind: "path", d: pathOf(local, closed) };
    };
    const woodBones = [...new Set(m.tubes.map((t) => `${t.bone}|${spec.regionOfTube(t)}`))];
    let z = 1;
    for (const key of woodBones) {
      const [bone, region] = key.split("|") as [string, string];
      const tubes = m.tubes.filter((t) => t.bone === bone && spec.regionOfTube(t) === region);
      if (view === "plan") continue;
      const rings = outline(tubes.flatMap((t) => tubePolygons(view, t)), tol, (3 * f) ** 2);
      const main = tubes.some((t) => t.id === bone);
      const s = set({ id: `${main ? "wood" : "twigs"}-${bone}`, bone, role: "body", z: r4(z++ / 100 + 1), filled: true, tone: toneOf(bone, "wood", region), region, material: tubes[0]!.material }, rings);
      if (s) {
        shapes.push(s);
        filled.push(rings);
      }
    }
    if (view === "plan") {
      const stem = m.tubes.find((t) => t.id === spec.bones[1]!.id)!;
      const q = stem.points.reduce((best, p) => (Math.abs(p.p[1] - 1.3) < Math.abs(best.p[1] - 1.3) ? p : best));
      shapes.push({ id: "stem-dot", bone: stem.bone, role: "body", z: 1.5, filled: true, tone: toneOf(stem.bone, "wood", "x"), region: spec.regionOfTube(stem), material: stem.material, kind: "ellipse", cx: r4(q.p[0]), cy: r4(q.p[2]), rx: r4(q.r), ry: r4(q.r) });
    }
    const shown = m.lobes.filter((l) => spec.regions.find((r) => r.id === spec.regionOfLobe(l))?.hidden === undefined);
    const keyOf = (l: Lobe): string => (m.crownBone === l.bone ? `${l.bone}|crown` : `${l.bone}|${spec.regionOfLobe(l)}`);
    const foliageKeys = [...new Set(shown.map(keyOf))];
    for (const key of foliageKeys) {
      const [bone, region] = key.split("|") as [string, string];
      const lobes = shown.filter((l) => keyOf(l) === key);
      const rings = outline(lobes.map((l) => lobeOutline(view, l, 72)), tol, (4 * f) ** 2);
      const merged = m.crownBone === bone ? { regions: [...new Set([...spec.regions.filter((r) => r.kind === "foliage" && r.hidden === undefined).map((r) => r.id), ...m.hiddenTubes.map((t) => spec.regionOfTube(t)), ...(spec.fruitRegion ? [spec.fruitRegion] : [])])] } : { region };
      const shade = region.includes("shade") && m.scale <= 100;
      const s = set({ id: m.crownBone === bone ? "crown" : `fol-${bone}${region.includes("shade") && !bone.includes("core") ? "-shade" : ""}`, bone, role: "body", z: r4(3 + z++ / 100), filled: true, tone: toneOf(bone, "lobe", region), ...merged, material: lobes[0]!.material, ...(shade ? { hatch: "shade-lines" } : {}) }, rings);
      if (s) {
        shapes.push(s);
        filled.push(rings);
      }
    }
    const toViewer = mul(VIEW_DIR[view], -1);
    const visibleClusters = m.clusters.filter((c) => !shown.some((l) => hitsLobe(l, add(c.c, mul(toViewer, Math.max(...c.radii) * 0.85)), toViewer)));
    const seen: Json = {};
    counts[`${view}Fruit`] = 0;
    if (m.scale < 500) for (const c of visibleClusters) {
      const n = (seen[c.bone] = (seen[c.bone] ?? 0) + 1);
      const s = set({ id: `${c.bone}-fruit${n}`, bone: c.bone, role: m.scale === 200 ? "body" : "detail", z: r4(8 + n / 1000), filled: true, tone: 0.22, region: spec.fruitRegion, material: c.material }, [lobeOutline(view, c, 20)]);
      if (s) {
        shapes.push(s);
        filled.push([lobeOutline(view, c, 20)]);
        counts[`${view}Fruit`]++;
      }
    }
    const allRings = filled.flat();
    const crownRings = outline(shown.map((l) => lobeOutline(view, l, 48)), tol, (4 * f) ** 2);
    const free = (pts: P2[], margin: number, placed: P2[][]): boolean => pts.every((p) => insideRings(p, crownRings) && edgeDist(p, allRings) > margin) && placed.every((o) => o.every((a) => pts.every((p) => Math.hypot(p.x - a.x, p.y - a.y) > margin * 1.6)));
    const visible = shown.filter((l) => !shown.some((o) => o !== l && nearness(view, o.c) > nearness(view, l.c) && inside(project(view, l.c), lobeOutline(view, o, 36)))).sort((a, b) => nearness(view, b.c) - nearness(view, a.c));
    counts[`${view}Glyphs`] = 0;
    counts[`${view}Arcs`] = 0;
    if (m.scale === 50) {
      const placed: P2[][] = [];
      visible.forEach((l, i) => {
        const c = project(view, l.c);
        for (let j = 0; j < 2; j++) {
          const ang = i * 2.39 + j * 2.6;
          const rr = 0.35 * Math.min(...l.radii);
          const at = { x: c.x + rr * Math.cos(ang), y: c.y + rr * Math.sin(ang) };
          const g = spec.drawing.glyph(at, i + j);
          const pts = g.rings.flat();
          if (!free(pts, 0.4 * paper, placed)) continue;
          placed.push(pts);
          const s = set({ id: `leaf-${l.id}-${j}`, bone: l.bone, role: "texture", z: r4(9 + counts[`${view}Glyphs`] / 1000), filled: false, tone: 0.3 }, g.rings, g.closed);
          if (s) shapes.push(s);
          counts[`${view}Glyphs`]++;
        }
      });
    }
    if (spec.drawing.arcScales.includes(m.scale)) {
      visible.slice(0, 40).forEach((l, i) => {
        const c = project(view, l.c);
        const mid = view === "plan" ? (Math.atan2(c.y, c.x) * 180) / Math.PI : 90;
        const arc = lobeOutline(view, { ...l, radii: mul(l.radii, 0.72) as V3 }, 10, mid - 50, mid + 50);
        if (!arc.every((p) => insideRings(p, crownRings) && edgeDist(p, crownRings) > 0.4 * paper)) return;
        const s = set({ id: `arc-${l.id}`, bone: l.bone, role: "detail", z: r4(9.5 + i / 1000), filled: false, tone: 0.3 }, [arc], false);
        if (s) {
          shapes.push(s);
          counts[`${view}Arcs`]++;
        }
      });
    }
    counts[`${view}Bark`] = 0;
    if (m.scale <= 100 && view !== "plan") for (const t of m.tubes.filter((x) => x.group === "stem" || x.group === "root" || (m.scale === 50 && x.group === "limb"))) {
      const strokes = spec.drawing.bark(view, t, m.scale).filter((stroke) => stroke.every((p) => edgeDist(p, allRings) > 0.35 * paper));
      const s = set({ id: `bark-${t.id}`, bone: t.bone, role: m.scale === 50 ? "texture" : "detail", z: 2, filled: false, tone: 0.3 }, strokes, false);
      if (s) {
        shapes.push(s);
        counts[`${view}Bark`] += strokes.length;
      }
    }
    geometry[view] = shapes;
    counts[`${view}Shapes`] = shapes.length;
  }
  return { geometry, counts };
}

const PENS: Record<Scale, Json> = {
  50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: 0.1, ground: 0.35 },
  100: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
  200: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
};
const RANGES: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
const HATCH: Record<number, Json[]> = { 50: [], 100: [], 200: [], 500: [] };

/** 🔣️ The true-extent plan symbol: the 1:500 crown outline, the stem at its real section, in all three views. */
function symbolOf(ctx: Ctx, m: ScaleModel): Json {
  const { spec } = ctx;
  const geometry: Json = {};
  const stem = m.tubes.find((t) => t.id === spec.bones[1]!.id)!;
  for (const view of VIEWS) {
    const outer = (ring: { x: number; y: number }[]): boolean => ring.reduce((a, q, i) => a + q.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * q.y, 0) > 0;
    const shapes: Json[] = outline(m.lobes.map((l) => lobeOutline(view, l, 48)), 0.05, 1).filter(outer).map((ring, i) => ({ id: i === 0 ? "crown" : `crown-${i + 1}`, bone: spec.bones[0]!.id, role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: pointsOf(ring), closed: true }));
    if (view === "plan") {
      const q = stem.points[1]!;
      shapes.push({ id: "stem-1", bone: spec.bones[0]!.id, role: "body", z: 3, filled: true, tone: 0.3, kind: "ellipse", cx: r4(q.p[0]), cy: r4(q.p[2]), rx: r4(q.r), ry: r4(q.r) });
    } else {
      const stemRings = outline(m.tubes.filter((t) => ctx.group(t.bone) !== "limb" && ctx.group(t.bone) !== "branch").flatMap((t) => tubePolygons(view, t)), 0.02, 0.01);
      shapes.push({ id: "stem-1", bone: spec.bones[0]!.id, role: "body", z: 3, filled: true, tone: 0.3, kind: "poly", points: pointsOf(stemRings[0]!), closed: true });
    }
    geometry[view] = shapes;
  }
  const e = extentOf(m);
  return {
    id: spec.symbol.id, title: spec.symbol.title, description: spec.symbol.description, semantics: "true-extent", units: "model-m", denominators: [500, 200, 100],
    pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [],
    dimensionalNote: tx(`A conventional simplification of the plant drawn at its true extent in model metres in all three views (${(e.max[0] - e.min[0]).toFixed(2)} m wide, ${(e.max[2] - e.min[2]).toFixed(2)} m deep and ${e.max[1].toFixed(2)} m high): the outline is the crown, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale.`, `Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten (${(e.max[0] - e.min[0]).toFixed(2).replace(".", ",")} m breit, ${(e.max[2] - e.min[2]).toFixed(2).replace(".", ",")} m tief und ${e.max[1].toFixed(2).replace(".", ",")} m hoch): Der Umriss ist die Krone, der Stamm ist in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.`),
    geometry,
  };
}

/** 🏗️ Builds the asset document, writes it to `out` (candidate) and returns it with a report. */
export function build(spec: Species): { document: Json; report: string[]; row: Json } {
  const ctx = contextOf(spec);
  const report: string[] = [];
  const massKg = massesAndSections(ctx);
  for (const s of SCALES) {
    const fl = floating(spec.models[s]);
    if (fl.length) report.push(`[DEBUG] 1:${s} floating parts: ${fl.join(" ")}`);
  }
  const e50 = extentOf(spec.models[50]);
  const dims = { heightM: r4(e50.max[1]), widthM: r4(e50.max[0] - e50.min[0]), depthM: r4(e50.max[2] - e50.min[2]), massKg };
  for (const s of SCALES) {
    const e = extentOf(spec.models[s]);
    report.push(`1:${s} extent x ${e.min[0].toFixed(3)}..${e.max[0].toFixed(3)} y ${e.max[1].toFixed(3)} z ${e.min[2].toFixed(3)}..${e.max[2].toFixed(3)} (dx ${(((e.max[0] - e.min[0]) / dims.widthM - 1) * 100).toFixed(1)} %, dy ${((e.max[1] / dims.heightM - 1) * 100).toFixed(1)} %, dz ${(((e.max[2] - e.min[2]) / dims.depthM - 1) * 100).toFixed(1)} %); tubes ${spec.models[s].tubes.length}, lobes ${spec.models[s].lobes.length}, clusters ${spec.models[s].clusters.length}`);
  }
  const barkArea: Record<string, number> = {};
  for (const t of spec.models[50].tubes) barkArea[spec.regionOfTube(t)] = (barkArea[spec.regionOfTube(t)] ?? 0) + tubeArea(t);
  report.push(`bark mantle area at 1:50: ${Object.entries(barkArea).map(([k, v]) => `${k} ${v.toFixed(2)} m2`).join(", ")}`);
  const anchorsOut: Record<string, { bone: string; local: V3 }> = {};
  const volumes: Json = {};
  for (const s of SCALES) volumes[String(s)] = volumeOf(ctx, spec.models[s], anchorsOut);
  const anchors: Json[] = [{ id: "base-support", bone: spec.bones[0]!.id, x: 0, y: 0, kind: "support", title: tx("Root plate anchorage", "Verankerung des Wurzeltellers"), support: "fixed", normalDeg: 90, footprintLengthM: r4(2 * spec.models[50].tubes[0]!.points[0]!.r * 2.5), footprintWidthM: r4(2 * spec.models[50].tubes[0]!.points[0]!.r * 2.5), frictionCoefficient: 0.6 }];
  for (const em of spec.emitters) em.anchors.forEach((id: string, i: number) => {
    const a = anchorsOut[id];
    if (!a) throw new Error(`anchor ${id} has no 1:50 outlet`);
    anchors.push({ id, bone: a.bone, x: r4(a.local[0]), y: r4(a.local[1]), z: r4(a.local[2]), kind: "emitter", title: spec.anchorsTitle(em.id, i + 1) });
  });
  for (const em of spec.emitters) {
    const b = anchors.find((a) => a.id === em.anchor)!.bone;
    const u = ctx.F[b]!.u;
    em.directionDeg = r4(norm180(90 - deg(Math.atan2(u[1], u[0]))));
  }
  const representations: Json = {};
  const counts: Json = {};
  for (const s of SCALES) {
    const { geometry, counts: c } = drawScale(ctx, spec.models[s]);
    counts[s] = c;
    const dec = spec.drawing.decisions[s];
    const decisions: Json = {};
    for (const k of ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]) decisions[k] = typeof dec[k] === "function" ? dec[k](c) : dec[k];
    representations[String(s)] = { status: "authored", denominator: s, validRange: { minDenominator: RANGES[s][0], maxDenominator: RANGES[s][1] }, pens: PENS[s], hatches: HATCH[s], geometry, decisions };
    report.push(`1:${s} drawing ${JSON.stringify(c)}`);
  }
  const thermal = { environment: spec.environment, passengers: [], operating: [], regions: spec.regions, emitters: spec.emitters, assumptions: spec.thermalAssumptions };
  const draft: Json = { id: spec.id, category: "vegetation", title: spec.title, summary: spec.summary, tags: spec.tags, provenance: spec.provenance, dimensions: dims, materials: spec.materials, rig: { kind: "botanical", root: spec.bones[0]!.id, bones: ctx.bones }, volumes, anchors, representations, symbols: [symbolOf(ctx, spec.models[500])], poses: [], clips: [], thermal, structure: { gravityMS2: G, dynamics: "quasi-static", supports: ["base-support"], assumptions: spec.structureAssumptions, clinical: false }, presentation: spec.presentation, exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], completion: "authored" };
  draft.poses = spec.poses(ctx);
  draft.clips = spec.clips(ctx);
  const document = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset: draft };
  const row = { id: spec.id, category: "vegetation", title: spec.title, heightM: dims.heightM, widthM: dims.widthM, powerW: 0, poses: draft.poses.map((p: Json) => p.id), clips: draft.clips.map((c: Json) => c.id), representations: { "50": "authored", "100": "authored", "200": "authored", "500": "authored" }, symbols: draft.symbols.map((s: Json) => s.id), analytics: ["thermal", "transport", "structural"], exports: draft.exports, distinction: spec.distinction, completion: "authored" };
  report.push(`dimensions ${JSON.stringify(dims)}; bones ${ctx.bones.length}; leaf mass and wood mass in the bones`);
  return { document, report, row };
}

/** 💾️ Writes the candidate (generated folder) or the catalog file and the manifest row. */
export function write(spec: Species, result: { document: Json; report: string[]; row: Json }, target: "candidate" | "catalog"): string {
  const path = target === "candidate" ? join(WAVE, "🗑️generated", "PW-06", spec.id, "🔣️.json") : join(STAFFAGE, "🗂️catalog", "🌳️vegetation", spec.id, "🔣️.json");
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(result.document, null, 2) + "\n");
  if (target === "catalog") {
    const row = join(WAVE, "📥️manifest-rows", `${spec.id}.json`);
    mkdirSync(dirname(row), { recursive: true });
    writeFileSync(row, JSON.stringify(result.row, null, 2) + "\n");
  }
  return path;
}
