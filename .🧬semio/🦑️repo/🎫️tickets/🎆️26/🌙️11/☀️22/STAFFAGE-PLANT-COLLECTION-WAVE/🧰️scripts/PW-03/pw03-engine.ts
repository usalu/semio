/** 🌳️ PW-03 engine: turns a species design (per-scale wood, crown clumps, flowers or fruit, outlets, texts) and the base document of `pw03_build.py` into a complete `semio.staffage/v2` asset: bone-local volumes per scale (body, thermal, structure), the three views of every scale as true orthographic views of that scale's body, symbols, dimensions, emitter anchors and bark areas.
 * Every form is stated in the species files; this module converts, projects and assembles.
 * @see ./pw03_build.py
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r5-apple-tree-model.ts */
import { D2R, add, along, branch, dirOf, dot3, lobeOutline, mul, nearness, outline, pathOf, project, r4, r5, ringLobe, solidOf, sub, surfacePoint, toDrawing, tubePolygons, unit, type Lobe, type Tube, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";
import { dirToLocal, frames, toLocal, type Frame, type Json, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";

export type Text = { en: string; de: string };
export type Goblet = { kind: "goblet"; id: string; bone: string; material: string; tone: number; group: string; base: V3; profile: [number, number][] };
export type Extra = Lobe | Goblet;
export type Outlet = [emitter: string, anchor: string | null, lobe: string, az: number, el: number];
export type ClumpRow = [id: string, bone: string, az: number, y: number, s: number, k: number, bosses: string, tilt: number];
export type SideRow = [id: string, parent: string, f: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number];
export type Model = { scale: number; wood: Tube[]; twigs: Tube[]; hiddenWood: Tube[]; hiddenTwigs: Tube[]; lobes: Lobe[]; extras: Extra[]; hiddenExtras: Extra[]; outlets: Outlet[]; crownBone?: string };
export type Counts = { scale: number; lobes: number; mains: number; extras: number; seen: Record<View, number>; wood: number; twigs: number };
export type Species = {
  id: string;
  trunk: string;
  woodMaterial: string;
  twigRegion?: string;
  extraRegion: string;
  leafGlyph: (c: P2, deg: number) => P2[];
  extraGlyph: "cluster" | "fruit" | "goblet";
  bark: (view: View, scale: number, ctx: Context) => { id: string; bone: string; role: string; points: V3[] }[];
  stems?: string[];
  model: (scale: number, ctx: Context) => Model;
  texts: (scale: number, c: Counts) => { drawn: Record<string, Text>; volume: Text[] };
  symbolTexts: { plan: Record<string, Text>; sign: Record<string, Text> };
  scaffolds: string[];
  areaOverrides?: Record<string, number>;
};
export type Context = { F: Record<string, Frame>; skeleton: Json; asset: Json; bone: (id: string) => Json };

export const SCALES = [50, 100, 200, 500] as const;
export const feature = (scale: number): number => (0.15 * scale) / 1000;

/** 🪵️ A skeleton limb as a tube at one scale: `keep` picks the section indices (all when absent), `lead` extends the butt into the parent. */
export function limbTube(ctx: Context, id: string, keep: number[] | null, tone: number, lead = 0.05, group?: string): Tube {
  const k = ctx.skeleton.skeleton[id];
  const pts: V3[] = k.points;
  const idx = keep ?? pts.map((_: V3, i: number) => i);
  const points = idx.map((i) => ({ p: pts[i]! as V3, r: k.radii[i] as number }));
  if (lead > 0) {
    const d = unit(sub(pts[1]!, pts[0]!));
    points.unshift({ p: sub(pts[0]!, mul(d, lead)), r: k.radii[0] });
  }
  const material = ctx.asset.thermal.regions.find((r: Json) => r.id === k.region).material;
  return { kind: "tube", id, bone: id, material, tone, group: group ?? k.group, points };
}

/** 🌳️ A vertical stem part from rows (y, r, dx, dz). */
export function stemTube(ctx: Context, id: string, tone: number, group: string, rows: [number, number, number, number][]): Tube {
  const region = ctx.skeleton.skeleton[id].region;
  const material = ctx.asset.thermal.regions.find((r: Json) => r.id === region).material;
  return { kind: "tube", id, bone: id, material, tone, group, points: rows.map(([y, r, dx, dz]) => ({ p: [dx, y, dz] as V3, r })) };
}

/** 🌿️ Side branches from a table on parent tubes (bound to the parent's bone). */
export function sides(rows: SideRow[], wood: Tube[], material: string, tone: number, n: number): Tube[] {
  return rows.map(([id, parent, f, az, e0, e1, L, r0, r1, bend]) => {
    const p = wood.find((t) => t.id === parent);
    if (!p) throw new Error(`side branch ${id} on unknown tube ${parent}`);
    return branch(id, p.bone, material, tone, "side", along(p, f), az, e0, e1, L, r0, r1, bend, n);
  });
}

export const BOSS: Record<string, [V3, number]> = {
  "1": [[1, 0.08, 0], 0.46], "2": [[0.62, 0.72, 0.2], 0.43], "3": [[0.72, -0.58, -0.22], 0.4], "4": [[0.42, 0.16, 0.9], 0.41],
  "5": [[0.45, 0.06, -0.9], 0.39], "6": [[0.08, 1, -0.28], 0.42], "7": [[0.85, 0.3, -0.55], 0.36],
};

/** 🍃️ Crown clumps of one scale: a main mass per row on the envelope `reach(az, y)` with the bosses the scale keeps. */
export function clumps(rows: ClumpRow[], o: { reach: (az: number, y: number) => number; axis: [number, number]; keep: string | null; grow: number; bossGrow: number; shape: [number, number, number]; inset: number; material: string; tone: number; bone?: string; prefix?: string }): Lobe[] {
  const out: Lobe[] = [];
  for (const [id, bone, az, y, s, k, bosses, tilt] of rows) {
    const R = o.reach(az, y) * k;
    const [a, b, c] = [o.shape[0] * s * o.grow, o.shape[1] * s * o.grow, o.shape[2] * s * o.grow];
    const on = o.bone ?? bone;
    const main = ringLobe(`${o.prefix ?? ""}${id}-m`, on, o.material, o.tone, "sun", az, R - o.inset * s, y, a, b, c, tilt, 0);
    main.c = [main.c[0] + o.axis[0], main.c[1], main.c[2] + o.axis[1]];
    out.push(main);
    for (const n of [...bosses].filter((x) => o.keep === null || o.keep.includes(x))) {
      const [dl, rf] = BOSS[n]!;
      const d = unit(add(add(mul(main.axes[0], dl[0]), mul([0, 1, 0], dl[1])), mul(main.axes[2], dl[2])));
      const rb = rf * s * o.bossGrow;
      const centre = add(surfacePoint(main, d), mul(d, -0.55 * rb));
      out.push({ kind: "lobe", id: `${o.prefix ?? ""}${id}-b${n}`, bone: on, material: o.material, tone: o.tone, group: "sun", c: centre, axes: main.axes, radii: [rb, rb * 0.86, rb] });
    }
  }
  return out;
}

/** 🔵️ A round lobe (core, cap, cluster or fruit). */
export const ball = (id: string, bone: string, material: string, tone: number, group: string, c: V3, radii: V3): Lobe => ({ kind: "lobe", id, bone, material, tone, group, c, axes: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], radii });

/** 🎯️ Items on lobe surfaces: per row the lobe, azimuth and elevation of the surface point, and the offset of the item centre outside it. */
export function onLobes(rows: [lobe: string, az: number, el: number][], lobes: Lobe[], make: (id: string, bone: string, at: V3, d: V3) => Extra, prefix: string): Extra[] {
  const count: Record<string, number> = {};
  return rows.flatMap(([lobeId, az, el]) => {
    const l = lobes.find((x) => x.id === lobeId);
    if (!l) return [];
    const d = dirOf(az, el);
    const n = (count[l.bone] = (count[l.bone] ?? 0) + 1);
    return [make(`${prefix}-${l.bone}-${n}`, l.bone, surfacePoint(l, d), d)];
  });
}

const ellipsoidHit = (l: Lobe, p: V3, d: V3): boolean => {
  const o = l.axes.map((a, i) => dot3(sub(p, l.c), a) / l.radii[i]!);
  const v = l.axes.map((a, i) => dot3(d, a) / l.radii[i]!);
  const A = v[0]! ** 2 + v[1]! ** 2 + v[2]! ** 2, B = o[0]! * v[0]! + o[1]! * v[1]! + o[2]! * v[2]!, C = o[0]! ** 2 + o[1]! ** 2 + o[2]! ** 2 - 1;
  if (C < 0) return true;
  const disc = B * B - A * C;
  return disc > 0 && -B + Math.sqrt(disc) > 0;
};
const VIEW_DIR: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
const extraTop = (e: Extra): V3 => (e.kind === "lobe" ? e.c : add(e.base, [0, Math.max(...e.profile.map((q) => q[0])) * 0.6, 0]));
const extraFace = (e: Extra, view: View): V3 => (e.kind === "lobe" ? add(e.c, mul(VIEW_DIR[view], e.radii[0] * 0.6)) : add(extraTop(e), mul(VIEW_DIR[view], Math.max(...e.profile.map((q) => q[1])) * 0.5)));

/** 👁️ Whether an item is seen from a viewpoint: the ray from its visible face towards the viewer leaves every lobe. */
export const seen = (e: Extra, view: View, lobes: Lobe[]): boolean => {
  const size = e.kind === "lobe" ? e.radii[0] : Math.max(...e.profile.map((q) => q[1]));
  const [a, b]: [V3, V3] = view === "elevation" ? [[1, 0, 0], [0, 1, 0]] : view === "front" ? [[0, 0, 1], [0, 1, 0]] : [[1, 0, 0], [0, 0, 1]];
  const face = extraFace(e, view);
  return [[0, 0], [0.8, 0], [-0.8, 0], [0, 0.8], [0, -0.8]].some(([i, j]) => !lobes.some((l) => ellipsoidHit(l, add(face, add(mul(a, i! * size), mul(b, j! * size))), VIEW_DIR[view])));
};

/** 🌷️ The orthographic outline of an upright goblet. */
function gobletOutline(view: View, g: Goblet): P2[] {
  const c = project(view, g.base);
  if (view === "plan") {
    const r = Math.max(...g.profile.map((q) => q[1]));
    return Array.from({ length: 16 }, (_, i) => ({ x: c.x + r * Math.cos((2 * Math.PI * i) / 16), y: c.y + r * Math.sin((2 * Math.PI * i) / 16) }));
  }
  const right = g.profile.map(([a, r]) => ({ x: c.x + r, y: c.y + a }));
  const left = g.profile.slice(1, -1).reverse().map(([a, r]) => ({ x: c.x - r, y: c.y + a }));
  return [...right, ...left];
}

/** 🌸️ The outline of a blossom cluster: an ellipse shadow with five shallow scallops inside it. */
function clusterOutline(view: View, l: Lobe): P2[] {
  const ring = lobeOutline(view, l, 40);
  const c = project(view, l.c);
  return ring.map((p, i) => {
    const k = 0.9 + 0.1 * Math.abs(Math.cos((5 * Math.PI * i) / 40));
    return { x: c.x + (p.x - c.x) * k, y: c.y + (p.y - c.y) * k };
  });
}

/** 🧊️ The solid of an extra part. */
function extraSolid(e: Extra, F: Record<string, Frame>, role: string): Json {
  const f = F[e.bone]!;
  if (e.kind === "lobe") return { ...solidOf(e, F), role };
  return { id: e.id, kind: "revolve", bone: e.bone, material: e.material, role, tone: e.tone, profile: e.profile.map(([a, r]) => [r5(a), r5(r)]), axis: dirToLocal(f, [0, 1, 0]).map(r5), origin: toLocal(f, e.base).map(r5) };
}

/** 🧭️ Region of a part. */
function regionOf(sp: Species, ctx: Context, p: Tube | Extra): string {
  if (p.kind === "goblet" || p.group === "flower" || p.group === "fruit") return sp.extraRegion;
  if (p.kind === "lobe") return p.group === "shade" ? "leaf-shade" : "leaf-sun";
  if (p.group === "twig") return sp.twigRegion ?? ctx.skeleton.skeleton[p.bone]?.region ?? "limb-bark";
  return ctx.skeleton.skeleton[p.bone]?.region ?? "limb-bark";
}

/** 🧊️ The authored volume of a scale: body, thermal (references, hidden parts, outlet lenses) and structure (members, joints, support, masses). */
export function volumeOf(sp: Species, ctx: Context, m: Model, decisions: Text[], anchorsOut: Record<string, { bone: string; local: V3 }>): Json {
  const { F } = ctx;
  const s = (p: Tube | Lobe): Json => solidOf(p, F);
  const body: Json[] = [...m.wood.map(s), ...m.twigs.map(s), ...m.lobes.map(s), ...m.extras.map((e) => extraSolid(e, F, "detail"))];
  const thermal: Json[] = [];
  for (const p of [...m.wood, ...m.twigs, ...m.lobes, ...m.extras]) thermal.push({ id: `th-${p.id}`, of: p.id, region: regionOf(sp, ctx, p) });
  for (const p of [...m.hiddenWood, ...m.hiddenTwigs]) thermal.push({ ...s({ ...p, id: `th-${p.id}` }), region: regionOf(sp, ctx, p) });
  for (const e of m.hiddenExtras) thermal.push({ ...extraSolid({ ...e, id: `th-${e.id}` }, F, "detail"), region: sp.extraRegion });
  m.outlets.forEach(([emitter, anchor, lobeId, az, el], i) => {
    const l = m.lobes.find((x) => x.id === lobeId);
    if (!l) throw new Error(`1:${m.scale} outlet on unknown lobe ${lobeId}`);
    const d = dirOf(az, el);
    const p = surfacePoint(l, d);
    const q = sub(p, l.c);
    const n = unit(l.axes.reduce((sum, axis, k) => add(sum, mul(axis, dot3(q, axis) / l.radii[k]! ** 2)), [0, 0, 0] as V3));
    const side = unit(Math.abs(n[1]) > 0.98 ? sub([1, 0, 0], mul(n, n[0])) : [n[2], 0, -n[0]]);
    const third: V3 = [side[1] * n[2] - side[2] * n[1], side[2] * n[0] - side[0] * n[2], side[0] * n[1] - side[1] * n[0]];
    const rho = Math.min(l.radii[0], l.radii[2]) * (m.scale === 500 ? 0.45 : 0.6);
    const h = Math.max(0.1, feature(m.scale), ((rho * rho) / (2 * Math.min(...l.radii))) * 0.8);
    const lens: Lobe = { ...l, id: `th-${anchor ?? `${emitter}-${i + 1}`}`, c: sub(p, mul(n, h - 0.02)), axes: [side, n, third], radii: [rho, h, rho] };
    thermal.push({ ...s(lens), role: "detail", region: l.group === "shade" ? "leaf-shade" : "leaf-sun", tags: [emitter] });
    if (anchor) {
      if (anchorsOut[anchor] && anchorsOut[anchor]!.bone !== l.bone) throw new Error(`anchor ${anchor} moves to bone ${l.bone}`);
      anchorsOut[anchor] = { bone: l.bone, local: toLocal(F[l.bone]!, add(p, mul(n, 0.04))) };
    }
  });
  const structure: Json[] = [];
  const bark = (r: number): number => 0.002 + 0.04 * r;
  const members = [...m.wood, ...m.hiddenWood, ...m.twigs, ...m.hiddenTwigs].filter((t) => t.id === t.bone && ctx.bone(t.bone).section);
  for (const t of members) {
    const declared = Math.sqrt(ctx.bone(t.bone).section.areaM2 / Math.PI);
    const pts = m.scale === 500 ? t.points.map((q) => ({ p: q.p, r: t.bone === "root-plate" ? q.r - bark(q.r) : Math.min(declared, q.r - bark(q.r) + 0.02) })) : t.points.map((q) => ({ p: q.p, r: q.r - bark(q.r) }));
    structure.push({ ...s({ ...t, id: `member-${t.id}`, material: sp.woodMaterial, tone: 0.4, points: pts, flags: t.group === "twig" ? { slender: true } : undefined }), member: t.bone, part: "member" });
  }
  const fe = feature(m.scale);
  for (const t of members) {
    if (t.bone === "root-plate") continue;
    const f = F[t.bone]!;
    const first = t.points.find((q) => toLocal(f, q.p)[0] >= -1e-6) ?? t.points[0]!;
    const r = Math.min(first.r - 0.004, (first.r - bark(first.r)) * 1.12);
    if (2 * r < fe || (m.scale === 500 && ![sp.trunk].includes(t.bone))) continue;
    structure.push({ id: `joint-${t.bone}`, kind: "ellipsoid", bone: t.bone, material: sp.woodMaterial, role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: [0, 0, 0] }, member: t.bone, part: "joint" });
  }
  structure.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: sp.woodMaterial, role: "detail", tone: 0.25, profile: [[r5(-Math.max(0.04, fe)), 0.8], [0, 0.8]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  for (const id of ["root-plate", sp.trunk]) {
    const t = [...m.wood, ...m.hiddenWood].find((x) => x.id === id)!;
    const q = t.points[Math.floor(t.points.length / 2)]!;
    const r = Math.max(fe * 0.7, Math.min(0.1, q.r * 0.6));
    structure.push({ id: `mass-${id}`, kind: "ellipsoid", bone: id, material: sp.woodMaterial, role: "detail", tone: 0.3, radii: [r5(r), r5(r), r5(r)], transform: { translation: toLocal(F[id]!, q.p).map(r5) }, member: id, part: "mass" });
  }
  if (m.scale !== 500) for (const b of ctx.asset.rig.bones.filter((x: Json) => x.id.startsWith("fol-"))) {
    const r = Math.max(fe * 0.6, Math.cbrt((3 * b.mass.kg) / (4 * Math.PI * 800)));
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: sp.woodMaterial, role: "detail", tone: 0.3, radii: [r5(r), r5(r), r5(r)], transform: { translation: [b.mass.comX, b.mass.comY, 0] }, member: b.parent, part: "mass" });
  }
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions };
}

/** ✏️ The three views of a scale as true orthographic views of its body. */
export function drawScale(sp: Species, ctx: Context, m: Model, rep: Json): Counts {
  const { F } = ctx;
  const fe = feature(m.scale);
  const tol = fe / 4;
  const counts: Counts = { scale: m.scale, lobes: m.lobes.length, mains: m.lobes.filter((l) => l.id.endsWith("-m")).length, extras: m.extras.length, seen: { elevation: 0, front: 0, plan: 0 }, wood: m.wood.length, twigs: m.twigs.length };
  const hatch = rep.hatches.find((h: Json) => h.id === "shade-lines")?.id;
  const allRegions = ctx.asset.thermal.regions.map((r: Json) => r.id as string);
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const out: Json[] = [];
    const put = (shape: Json, rings: P2[][], closed: boolean): void => {
      if (rings.length === 0 || rings[0]!.length < 2) return;
      out.push({ ...shape, kind: "path", d: pathOf(toDrawing(view, F[shape.bone]!, rings), closed) });
    };
    const woodBones = [...new Set(m.wood.filter((t) => t.group !== "twig").map((t) => t.bone))];
    woodBones.forEach((b, i) => {
      const rings = outline(m.wood.filter((t) => t.bone === b && t.group !== "twig").flatMap((t) => tubePolygons(view, t)), tol, (3 * fe) ** 2);
      const region = ctx.skeleton.skeleton[b]?.region ?? "limb-bark";
      const t0 = m.wood.find((t) => t.bone === b)!;
      put({ id: b === sp.trunk ? "stem" : `wood-${b}`, bone: b, role: "body", z: 1 + i / 100, filled: true, tone: t0.tone, region, material: t0.material }, rings, true);
    });
    const crownOnly = m.crownBone !== undefined;
    type Mass = { key: string; bone: string; group: string; lobes: Lobe[]; near: number };
    const masses: Mass[] = [];
    for (const l of m.lobes) {
      const key = crownOnly ? "crown" : `${l.bone}:${l.group}`;
      let g = masses.find((x) => x.key === key);
      if (!g) masses.push((g = { key, bone: l.bone, group: l.group, lobes: [], near: 0 }));
      g.lobes.push(l);
    }
    for (const g of masses) g.near = g.lobes.reduce((s, l) => s + nearness(view, l.c), 0) / g.lobes.length;
    masses.sort((a, b) => a.near - b.near);
    masses.forEach((g, i) => {
      const rings = outline(g.lobes.map((l) => lobeOutline(view, l)), tol, (4 * fe) ** 2);
      const shade = g.group === "shade";
      const base = { bone: g.bone, role: "body", z: 3 + i / 10, filled: true, tone: g.lobes[0]!.tone, material: g.lobes[0]!.material };
      if (crownOnly) put({ ...base, id: "crown", regions: allRegions.filter((r: string) => r !== ctx.skeleton.skeleton[sp.trunk].region) }, rings, true);
      else {
        const regions = sp.twigRegion && m.twigs.length === 0 && !shade ? { regions: ["leaf-sun", sp.twigRegion] } : { region: shade ? "leaf-shade" : "leaf-sun" };
        put({ ...base, id: `${g.bone}-${shade ? "shade" : "mass"}`, ...regions, ...(shade && hatch ? { hatch } : {}) }, rings, true);
      }
    });
    const zOf = (bone: string): number => {
      const i = masses.findIndex((g) => g.bone === bone);
      return 3 + Math.max(0, i) / 10 + 0.05;
    };
    if (m.scale <= 100) {
      const visible = m.lobes.filter((l) => !m.lobes.some((o) => o !== l && nearness(view, o.c) > nearness(view, l.c) && inside(project(view, l.c), lobeOutline(view, o, 36))));
      const arcs = m.scale === 100 ? visible.filter((l) => l.id.endsWith("-m") || l.group === "shade") : visible;
      const used: Record<string, number> = {};
      for (const l of arcs) {
        const c = project(view, l.c);
        const mid = view === "plan" ? (Math.atan2(c.y, c.x) * 180) / Math.PI : 90;
        const n = (used[l.bone] = (used[l.bone] ?? 0) + 1);
        const shrink: Lobe = { ...l, radii: l.radii.map((r) => r * 0.72) as V3 };
        put({ id: `${l.bone}-arc${n}`, bone: l.bone, role: m.scale === 100 ? "detail" : "texture", z: zOf(l.bone), filled: false, tone: 0.2 }, [lobeOutline(view, shrink, 10, mid - 55, mid + 55)], false);
      }
      if (m.scale === 50) {
        const leafUsed: Record<string, number> = {};
        visible.filter((l) => l.group !== "shade").forEach((l, i) => {
          if (i % 2 === 1) return;
          const c = project(view, l.c);
          const n = (leafUsed[l.bone] = (leafUsed[l.bone] ?? 0) + 1);
          const at = { x: c.x + 0.35 * l.radii[2] * Math.cos(i * 2.4 + 0.6), y: c.y - 0.1 - 0.25 * l.radii[1] * Math.abs(Math.sin(i * 1.7 + 0.4)) };
          put({ id: `${l.bone}-leaf${n}`, bone: l.bone, role: "detail", z: zOf(l.bone) + 0.01, filled: false, tone: 0.2 }, [sp.leafGlyph(at, 35 + i * 67)], true);
        });
      }
    }
    for (const t of m.twigs) put({ id: `twig-${t.id}`, bone: t.bone, role: m.scale === 50 ? "texture" : "detail", z: 2.5, filled: false, tone: 0.2, ...(sp.twigRegion ? { region: sp.twigRegion } : {}) }, [t.points.slice(1).map((q) => project(view, q.p))], false);
    if (view !== "plan" && m.scale <= 100) for (const b of sp.bark(view, m.scale, ctx)) put({ id: b.id, bone: b.bone, role: b.role, z: 1.2, filled: false, tone: 0.2 }, [b.points.map((p) => project(view, p))], false);
    const marks = m.extras.filter((e) => seen(e, view, m.lobes));
    counts.seen[view] = marks.length;
    const extraRegion = sp.extraRegion;
    const material = m.extras[0]?.material;
    marks.forEach((e, i) => {
      const ring = e.kind === "goblet" ? gobletOutline(view, e) : sp.extraGlyph === "cluster" ? clusterOutline(view, e) : lobeOutline(view, e, 24);
      put({ id: e.id, bone: e.bone, role: m.scale >= 200 ? "body" : "detail", z: 9 + i / 1000, filled: true, tone: e.tone, region: extraRegion, material }, [ring], true);
    });
    if (view === "plan" && m.scale <= 200) (sp.stems ?? [sp.trunk]).forEach((stem, k) => {
      const t = m.wood.find((x) => x.id === stem)!;
      const q = t.points[Math.floor(t.points.length / 2)]!;
      put({ id: k === 0 ? "trunk-dot" : `trunk-dot-${k + 1}`, bone: "root-plate", role: "body", z: 10, filled: true, tone: t.tone, region: ctx.skeleton.skeleton[stem].region, material: t.material }, [Array.from({ length: 16 }, (_, i) => ({ x: q.p[0] + q.r * Math.cos((2 * Math.PI * i) / 16), y: q.p[2] + q.r * Math.sin((2 * Math.PI * i) / 16) }))], true);
    });
    rep.geometry[view] = out;
  }
  return counts;
}

const inside = (p: P2, ring: P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};

/** 🪧️ The true-extent symbol (from the drawn shapes of the 1:100 representation in the default pose, so it has the extent of `dimensions`) and the nominal paper sign. */
export function symbols(sp: Species, m: Model, state: Json): Json[] {
  const tol = 0.02;
  const pts = (ring: P2[]): [number, number][] => ring.map((p) => [r4(p.x), r4(p.y)]);
  const area = (ring: P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;
  const outer = (rings: P2[][]): P2[][] => {
    const sorted = [...rings].sort((x, y) => Math.abs(area(y)) - Math.abs(area(x)));
    const kept: P2[][] = [];
    for (const r of sorted) if (Math.abs(area(r)) > 0.02 && !kept.some((k) => inside(r[0]!, k))) kept.push(r);
    return kept;
  };
  const stem = m.wood.find((t) => t.bone === sp.trunk)!;
  const geom: Json = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const filled = state.shapes[view].filter((e: Json) => e.shape.filled && e.shape.role === "body" && e.closed !== false);
    const crowns = outer(outline(filled.flatMap((e: Json) => e.rings as P2[][]), tol, 0.5));
    const list: Json[] = crowns.map((ring, i) => ({ id: i === 0 ? "crown" : `crown-${i + 1}`, bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: pts(ring), closed: true }));
    if (view === "plan") {
      const q = stem.points[Math.floor(stem.points.length / 2)]!;
      list.push({ id: "stem-1", bone: "root-plate", role: "body", z: 3, filled: true, tone: 0.3, kind: "poly", points: pts(Array.from({ length: 24 }, (_, i) => ({ x: q.p[0] + q.r * Math.cos((2 * Math.PI * i) / 24), y: q.p[2] + q.r * Math.sin((2 * Math.PI * i) / 24) }))), closed: true });
      sp.scaffolds.forEach((id, i) => {
        const t = m.wood.find((x) => x.id === id) ?? m.hiddenWood.find((x) => x.id === id);
        if (!t) return;
        const a = t.points[0]!.p, b = t.points[t.points.length - 1]!.p;
        list.push({ id: `scaffold-${i + 1}`, bone: "root-plate", role: "detail", z: 2.5, filled: false, tone: 0.3, kind: "poly", points: [[r4(a[0]), r4(a[2])], [r4(b[0]), r4(b[2])]], closed: false });
      });
    } else {
      const wood = state.shapes[view].filter((e: Json) => e.shape.id === "stem" || e.shape.bone === "root-plate");
      const ring = outer(outline(wood.flatMap((e: Json) => e.rings as P2[][]), tol, 0.01))[0];
      if (ring) list.push({ id: "stem-1", bone: "root-plate", role: "body", z: 3, filled: true, tone: 0.3, kind: "poly", points: pts(ring), closed: true });
    }
    geom[view] = list;
  }
  const circle = (cx: number, cy: number, r: number): string => {
    const k = 0.5523 * r, f = (n: number) => String(r4(n));
    return `M${f(cx + r)} ${f(cy)}C${f(cx + r)} ${f(cy + k)} ${f(cx + k)} ${f(cy + r)} ${f(cx)} ${f(cy + r)}C${f(cx - k)} ${f(cy + r)} ${f(cx - r)} ${f(cy + k)} ${f(cx - r)} ${f(cy)}C${f(cx - r)} ${f(cy - k)} ${f(cx - k)} ${f(cy - r)} ${f(cx)} ${f(cy - r)}C${f(cx + k)} ${f(cy - r)} ${f(cx + r)} ${f(cy - k)} ${f(cx + r)} ${f(cy)}Z`;
  };
  const sign = (cy: number, stemLine: boolean): Json[] => [
    { id: "sign-circle", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circle(0, cy, 1.6) },
    { id: "sign-dot", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: circle(0, cy, 0.35) },
    { id: "sign-ring", bone: "root-plate", role: "body", z: 3, filled: false, tone: 0.3, kind: "path", d: circle(0, cy, 0.9) },
    ...(stemLine ? [{ id: "sign-stem", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: `M0 0L0 ${r4(cy - 1.6)}` }] : []),
  ];
  return [
    { id: `${sp.id}-symbol`, ...sp.symbolTexts.plan, semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 }, hatches: [], geometry: geom },
    { id: "tree-sign", ...sp.symbolTexts.sign, semantics: "nominal", units: "paper-mm", denominators: [500, 200], pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: { elevation: sign(3.04, true), front: sign(3.04, true), plan: sign(0, false) } },
  ];
}

/** 🦴️ The context of a species: frames of the base rig, skeleton and bone lookup. */
export function contextOf(asset: Json, skeleton: Json): Context {
  return { F: frames(asset), skeleton, asset, bone: (id: string) => asset.rig.bones.find((b: Json) => b.id === id) };
}

/** 🧾️ Lateral mantle area of tubes (for the bark regions). */
export const mantle = (t: Tube): number => t.points.slice(1).reduce((s, q, i) => s + Math.PI * (q.r + t.points[i]!.r) * Math.hypot(Math.hypot(...sub(q.p, t.points[i]!.p)), q.r - t.points[i]!.r), 0);
