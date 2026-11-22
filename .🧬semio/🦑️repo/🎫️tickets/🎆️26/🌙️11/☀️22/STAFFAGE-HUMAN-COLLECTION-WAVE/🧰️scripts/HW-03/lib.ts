/** 🧰️ Derivation helpers of HW-03: read the closest existing human of the catalog, remove, rename, re-bind and scale its parts, and write the new asset. Every content value is authored in the per-asset scripts; these helpers only move data.
 * @see ../../📋️brief.md — the brief of the wave
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — the v2 types */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

export const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
export const CATALOG = join(S, "🗂️catalog/👤️humans");
export const SCALES = ["500", "200", "100", "50"] as const;
export const VIEWS = ["elevation", "front", "plan"] as const;
export const LISTS = ["body", "thermal", "structure"] as const;

// deno-lint-ignore no-explicit-any
export type J = any;

/** 📥️ A deep copy of the base document of a human: the snapshot `base-<slug>.json` kept next to the generators (the catalog file of the base changes under concurrent work), else the catalog file. */
export async function load(slug: string): Promise<J> {
  const snapshot = Bun.file(join(import.meta.dir, `base-${slug}.json`));
  return structuredClone(await ((await snapshot.exists()) ? snapshot : Bun.file(join(CATALOG, slug, "🔣️.json"))).json());
}

/** 💾️ Writes a document to its catalog folder (two-space JSON like the catalog). */
export function save(doc: J): string {
  const path = join(CATALOG, doc.asset.id, "🔣️.json");
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(doc, null, 2)}\n`);
  return path;
}

/** 🔁️ Every shape of every representation and symbol, with where it lives. */
export function* shapes(a: J): Generator<{ shape: J; list: J[]; scale: string | null; view: string }> {
  for (const s of SCALES) {
    const r = a.representations[s];
    if (r.status !== "authored") continue;
    for (const v of VIEWS) for (const shape of [...(r.geometry[v] ?? [])]) yield { shape, list: r.geometry[v], scale: s, view: v };
  }
  for (const sym of a.symbols) for (const v of VIEWS) for (const shape of [...(sym.geometry[v] ?? [])]) yield { shape, list: sym.geometry[v], scale: null, view: v };
}

/** 🧊️ Every solid of every authored volume, with where it lives. */
export function* solids(a: J): Generator<{ solid: J; list: J[]; scale: string; kind: string }> {
  for (const s of SCALES) {
    const v = a.volumes[s];
    if (v.status !== "authored") continue;
    for (const l of LISTS) for (const solid of [...v[l]]) yield { solid, list: v[l], scale: s, kind: l };
  }
}

const remove = <T>(list: T[], item: T): void => {
  const i = list.indexOf(item);
  if (i >= 0) list.splice(i, 1);
};

/** 🗑️ Removes shapes matching a predicate (scale null = symbols). */
export function dropShapes(a: J, pred: (shape: J, scale: string | null, view: string) => boolean): number {
  let n = 0;
  for (const { shape, list, scale, view } of shapes(a)) if (pred(shape, scale, view)) (remove(list, shape), n++);
  return n;
}

/** 🗑️ Removes solids matching a predicate and every thermal or structure reference to a removed body solid. */
export function dropSolids(a: J, pred: (solid: J, scale: string, list: string) => boolean): number {
  let n = 0;
  const gone = new Map<string, Set<string>>();
  for (const { solid, list, scale, kind } of solids(a)) {
    if (!pred(solid, scale, kind)) continue;
    remove(list, solid);
    n++;
    if (kind === "body") (gone.get(scale) ?? gone.set(scale, new Set()).get(scale)!).add(solid.id);
  }
  for (const { solid, list, scale, kind } of solids(a)) if (kind !== "body" && solid.of !== undefined && gone.get(scale)?.has(solid.of)) (remove(list, solid), n++);
  return n;
}

/** 🌳️ The ids of a bone and all its descendants. */
export function subtree(a: J, root: string): Set<string> {
  const out = new Set([root]);
  let grew = true;
  while (grew) {
    grew = false;
    for (const b of a.rig.bones) if (b.parent !== null && out.has(b.parent) && !out.has(b.id)) (out.add(b.id), (grew = true));
  }
  return out;
}

/** ✂️ Removes bones with their subtree and everything bound to them: shapes, solids, anchors, loads, offsets, contacts, emitters, supports. */
export function dropBones(a: J, roots: string[]): void {
  const bones = new Set(roots.flatMap((r) => [...subtree(a, r)]));
  a.rig.bones = a.rig.bones.filter((b: J) => !bones.has(b.id));
  dropShapes(a, (s) => bones.has(s.bone) || (s.solid?.axisBone !== undefined && bones.has(s.solid.axisBone)));
  dropSolids(a, (s) => bones.has(s.bone) || (s.member !== undefined && bones.has(s.member)));
  for (const { solid } of solids(a)) {
    if (solid.weights !== undefined) {
      const kept = solid.weights.filter((w: J) => !bones.has(w.bone));
      const sum = kept.reduce((t: number, w: J) => t + w.weight, 0);
      if (kept.length <= 1) delete solid.weights;
      else solid.weights = kept.map((w: J) => ({ ...w, weight: +(w.weight / sum).toFixed(6) }));
    }
    if (solid.members !== undefined) {
      solid.members = solid.members.filter((m: string) => !bones.has(m));
      if (solid.members.length === 0) delete solid.members;
    }
  }
  const anchors = new Set(a.anchors.filter((x: J) => bones.has(x.bone)).map((x: J) => x.id));
  a.anchors = a.anchors.filter((x: J) => !anchors.has(x.id));
  const loadGone = (l: J): boolean => (l.anchor !== undefined && anchors.has(l.anchor)) || (l.bone !== undefined && bones.has(l.bone)) || (l.distributed !== undefined && bones.has(l.distributed.bone));
  const loads = new Set<string>();
  for (const p of a.poses) {
    for (const b of bones) delete p.bones[b];
    p.contacts = p.contacts.filter((c: string) => !anchors.has(c));
    for (const l of p.loads) if (loadGone(l)) loads.add(l.id);
    p.loads = p.loads.filter((l: J) => !loadGone(l));
  }
  for (const c of a.clips) {
    for (const k of c.keys) if (k.bones !== undefined) for (const b of bones) delete k.bones[b];
    c.contacts = c.contacts.filter((x: J) => !anchors.has(x.anchor));
    c.events = c.events.filter((e: J) => e.target === undefined || !(anchors.has(e.target) || bones.has(e.target) || loads.has(e.target)));
    c.tracks = c.tracks.filter((t: J) => !(t.target.startsWith("load:") && loads.has(t.target.slice(5))));
  }
  a.structure.supports = a.structure.supports.filter((s: string) => !anchors.has(s));
  const emitters = new Set(a.thermal.emitters.filter((e: J) => anchors.has(e.anchor)).map((e: J) => e.id));
  dropEmitters(a, [...emitters]);
  if (a.thermal.respiration !== undefined) a.thermal.respiration.motion = a.thermal.respiration.motion.filter((m: J) => !bones.has(m.bone));
}

/** 💨️ Removes emitters, their thermal solids and their pose and clip bindings. */
export function dropEmitters(a: J, ids: string[]): void {
  const set = new Set(ids);
  a.thermal.emitters = a.thermal.emitters.filter((e: J) => !set.has(e.id));
  for (const p of a.poses) for (const id of set) delete p.thermal.emitters[id];
  for (const c of a.clips) c.tracks = c.tracks.filter((t: J) => !(t.target.startsWith("emitter:") && set.has(t.target.slice(8))));
  dropSolids(a, (s, _scale, list) => list === "thermal" && (s.tags ?? []).some((t: string) => set.has(t)));
}

/** 🌡️ Removes a thermal region: its shapes (or only its id in merged `regions`), thermal solids, bindings and tracks. */
export function dropRegion(a: J, id: string): void {
  a.thermal.regions = a.thermal.regions.filter((r: J) => r.id !== id);
  for (const { shape } of shapes(a)) {
    if (shape.regions !== undefined) {
      shape.regions = shape.regions.filter((r: string) => r !== id);
      if (shape.regions.length === 1) (shape.region = shape.regions[0]), delete shape.regions;
      else if (shape.regions.length === 0) delete shape.regions;
    }
  }
  dropShapes(a, (s) => s.region === id);
  dropSolids(a, (s, _scale, list) => list === "thermal" && s.region === id);
  for (const p of a.poses) delete p.thermal.regions[id];
  for (const c of a.clips) c.tracks = c.tracks.filter((t: J) => !t.target.startsWith(`region:${id}:`));
}

/** 🏷️ Renames a thermal region everywhere it is named. */
export function renameRegion(a: J, from: string, to: string): void {
  for (const r of a.thermal.regions) if (r.id === from) r.id = to;
  for (const { shape } of shapes(a)) {
    if (shape.region === from) shape.region = to;
    if (shape.regions !== undefined) shape.regions = [...new Set(shape.regions.map((r: string) => (r === from ? to : r)))];
  }
  for (const { solid } of solids(a)) if (solid.region === from) solid.region = to;
  for (const e of a.thermal.emitters) if (e.region === from) e.region = to;
  for (const p of a.poses) if (p.thermal.regions[from] !== undefined) (p.thermal.regions[to] = p.thermal.regions[from]), delete p.thermal.regions[from];
  for (const c of a.clips) for (const t of c.tracks) if (t.target.startsWith(`region:${from}:`)) t.target = `region:${to}:${t.target.slice(8 + from.length)}`;
}

/** 🧵️ Merges region `from` into `into`: shapes and solids are re-bound, areas add up, shares add up. */
export function mergeRegion(a: J, from: string, into: string): void {
  const f = a.thermal.regions.find((r: J) => r.id === from);
  const t = a.thermal.regions.find((r: J) => r.id === into);
  t.areaM2 = +(t.areaM2 + f.areaM2).toFixed(4);
  if (t.body !== undefined && f.body !== undefined) {
    t.body.metabolicShare = +(t.body.metabolicShare + f.body.metabolicShare).toFixed(4);
    t.body.skinFraction = +(t.body.skinFraction + f.body.skinFraction).toFixed(4);
  }
  a.thermal.regions = a.thermal.regions.filter((r: J) => r.id !== from);
  for (const { shape } of shapes(a)) {
    if (shape.region === from) shape.region = into;
    if (shape.regions !== undefined) {
      shape.regions = [...new Set(shape.regions.map((r: string) => (r === from ? into : r)))];
      if (shape.regions.length === 1) (shape.region = shape.regions[0]), delete shape.regions;
    }
  }
  for (const { solid } of solids(a)) if (solid.region === from) solid.region = into;
  for (const e of a.thermal.emitters) if (e.region === from) e.region = into;
  for (const p of a.poses) delete p.thermal.regions[from];
  for (const c of a.clips) c.tracks = c.tracks.filter((x: J) => !x.target.startsWith(`region:${from}:`));
}

/** 🎨️ Renames a material everywhere and replaces its record. */
export function replaceMaterial(a: J, from: string, record: J): void {
  const i = a.materials.findIndex((m: J) => m.id === from);
  if (i < 0) throw new Error(`material ${from}`);
  a.materials[i] = record;
  const to = record.id;
  for (const { shape } of shapes(a)) if (shape.material === from) shape.material = to;
  for (const { solid } of solids(a)) if (solid.material === from) solid.material = to;
  for (const r of a.thermal.regions) {
    if (r.material === from) r.material = to;
    for (const l of r.layers) if (l.material === from) l.material = to;
  }
  for (const b of a.rig.bones) if (b.section?.material === from) b.section.material = to;
}

/** 🧹️ Removes materials no shape, solid, region, layer or section uses. */
export function pruneMaterials(a: J): string[] {
  const used = new Set<string>();
  for (const { shape } of shapes(a)) if (shape.material) used.add(shape.material);
  for (const { solid } of solids(a)) if (solid.material) used.add(solid.material);
  for (const r of a.thermal.regions) (used.add(r.material), r.layers.forEach((l: J) => used.add(l.material)));
  for (const b of a.rig.bones) if (b.section) used.add(b.section.material);
  const gone = a.materials.filter((m: J) => !used.has(m.id)).map((m: J) => m.id);
  a.materials = a.materials.filter((m: J) => used.has(m.id));
  return gone;
}

/** 📚️ Removes references nothing cites; adds the given ones. */
export function references(a: J, add: J[]): void {
  const text = JSON.stringify({ ...a, provenance: { ...a.provenance, references: [] } });
  const all = [...a.provenance.references.filter((r: J) => !add.some((x) => x.id === r.id)), ...add];
  a.provenance.references = all.filter((r: J) => text.includes(`"${r.id}"`) || add.some((x) => x.id === r.id));
}

/** 🎚️ Recolours (tone) shapes and solids by material. */
export function tone(a: J, material: string, f: (t: number) => number): void {
  for (const { shape } of shapes(a)) if (shape.material === material) shape.tone = +f(shape.tone).toFixed(3);
  for (const { solid } of solids(a)) if (solid.material === material && solid.tone !== undefined) solid.tone = +f(solid.tone).toFixed(3);
}

/** 🆔️ Renames shapes and solids whose id matches. */
export function renameIds(a: J, f: (id: string) => string): void {
  for (const { shape } of shapes(a)) shape.id = f(shape.id);
  for (const { solid } of solids(a)) {
    solid.id = f(solid.id);
    if (solid.of !== undefined) solid.of = f(solid.of);
  }
}

/** 📏️ Uniform scale of every length of an asset by `k` (paper quantities, angles, tones, draw order and masses are untouched; masses are re-set by the caller). */
export function scaleLengths(a: J, k: number): void {
  const r = (x: number): number => +(x * k).toFixed(6);
  const v = (p: number[]): number[] => p.map(r);
  for (const b of a.rig.bones) {
    for (const f of ["x", "y", "z", "lengthM"]) if (typeof b[f] === "number") b[f] = r(b[f]);
    if (b.mass) (b.mass.comX = r(b.mass.comX)), (b.mass.comY = r(b.mass.comY));
    if (b.joint.minM !== undefined) b.joint.minM = r(b.joint.minM);
    if (b.joint.maxM !== undefined) b.joint.maxM = r(b.joint.maxM);
    if (b.section) (b.section.areaM2 *= k * k), (b.section.secondMomentM4 *= k ** 4), (b.section.extremeFibreM = r(b.section.extremeFibreM));
  }
  for (const x of a.anchors) for (const f of ["x", "y", "z", "footprintLengthM", "footprintWidthM"]) if (typeof x[f] === "number") x[f] = r(x[f]);
  for (const { shape, scale } of shapes(a)) {
    if (scale === null) continue;
    scaleShape(shape, k);
  }
  for (const sym of a.symbols) if (sym.units === "model-m") for (const vw of VIEWS) for (const s of sym.geometry[vw] ?? []) scaleShape(s, k);
  for (const s of SCALES) {
    const rep = a.representations[s];
    if (rep.status === "unavailable") rep.limitingSizeM = r(rep.limitingSizeM);
    if (rep.sections) for (const vw of VIEWS) if (rep.sections[vw]) rep.sections[vw].plane = r(rep.sections[vw].plane);
  }
  for (const { solid } of solids(a)) scaleSolid(solid, k);
  for (const p of a.poses) {
    if (p.root) for (const f of ["x", "y"]) if (typeof p.root[f] === "number") p.root[f] = r(p.root[f]);
    for (const o of Object.values(p.bones) as J[]) for (const f of ["dx", "dy"]) if (typeof o[f] === "number") o[f] = r(o[f]);
    for (const l of p.loads) for (const f of ["x", "y", "z"]) if (typeof l[f] === "number") l[f] = r(l[f]);
  }
  for (const c of a.clips) {
    if (typeof c.advanceM === "number") c.advanceM = r(c.advanceM);
    for (const key of c.keys) for (const o of Object.values(key.bones ?? {}) as J[]) for (const f of ["dx", "dy"]) if (typeof o[f] === "number") o[f] = r(o[f]);
  }
  for (const reg of a.thermal.regions) (reg.areaM2 = +(reg.areaM2 * k * k).toFixed(4)), (reg.characteristicLengthM = r(reg.characteristicLengthM));
  for (const e of a.thermal.emitters) e.diameterM = r(e.diameterM);
  if (a.thermal.respiration) for (const m of a.thermal.respiration.motion) m.dy = r(m.dy);
  if (typeof a.thermal.minimumAirPassageM === "number") a.thermal.minimumAirPassageM = r(a.thermal.minimumAirPassageM);
  for (const f of ["heightM", "widthM", "depthM"]) a.dimensions[f] = +(a.dimensions[f] * k).toFixed(3);
  void v;
}

/** 📐️ Scales the model-metre fields of one 2D shape. */
export function scaleShape(s: J, k: number): void {
  const r = (x: number): number => +(x * k).toFixed(6);
  switch (s.kind) {
    case "capsule":
      for (const f of ["x0", "y0", "x1", "y1", "r0", "r1"]) s[f] = r(s[f]);
      break;
    case "ellipse":
      for (const f of ["cx", "cy", "rx", "ry"]) s[f] = r(s[f]);
      break;
    case "rect":
      for (const f of ["x", "y", "w", "h", "r"]) if (typeof s[f] === "number") s[f] = r(s[f]);
      break;
    case "poly":
      s.points = s.points.map((p: number[]) => p.map(r));
      break;
    case "path":
      throw new Error(`path shape ${s.id} is not scaled`);
    case "member":
      for (const e of [s.from, s.to]) for (const f of ["x", "y", "z"]) e[f] = r(e[f]);
      if (s.section.kind === "rect") (s.section.w = r(s.section.w)), (s.section.h = r(s.section.h));
      else s.section.d = r(s.section.d);
      break;
  }
  if (s.solid) {
    for (const f of ["axisX", "axisZ", "z0", "z1"]) if (typeof s.solid[f] === "number") s.solid[f] = r(s.solid[f]);
    if (s.solid.zTip) s.solid.zTip = s.solid.zTip.map(r);
  }
}

/** 🧊️ Scales the metre fields of one solid (a reference has none). */
export function scaleSolid(s: J, k: number): void {
  const r = (x: number): number => +(x * k).toFixed(6);
  const v = (p: number[]): number[] => p.map(r);
  switch (s.kind) {
    case undefined:
      break;
    case "box":
      s.size = v(s.size);
      if (typeof s.cornerRadius === "number") s.cornerRadius = r(s.cornerRadius);
      break;
    case "ellipsoid":
      s.radii = v(s.radii);
      break;
    case "capsule":
      (s.p0 = v(s.p0)), (s.p1 = v(s.p1)), (s.r0 = r(s.r0)), (s.r1 = r(s.r1));
      break;
    case "revolve":
      (s.profile = s.profile.map(v)), (s.origin = v(s.origin));
      break;
    case "extrude":
      s.profile = s.profile.map(v);
      if (s.holes) s.holes = s.holes.map((h: number[][]) => h.map(v));
      (s.from = r(s.from)), (s.to = r(s.to));
      break;
    case "polyhedron":
      s.vertices = s.vertices.map(v);
      break;
    case "loft":
      for (const c of s.sections) (c.at = v(c.at)), (c.rx = r(c.rx)), (c.ry = r(c.ry));
      break;
    default:
      throw new Error(`solid kind ${s.kind}`);
  }
  if (s.transform?.translation) s.transform.translation = v(s.transform.translation);
  if (s.repeat) {
    if (s.repeat.translation) s.repeat.translation = v(s.repeat.translation);
    if (s.repeat.origin) s.repeat.origin = v(s.repeat.origin);
  }
}

/** ⚖️ Re-sets segment masses so they sum to `bodyKg` in the de Leva proportions of the base. */
export function scaleMasses(a: J, ids: (id: string) => boolean, factor: number): void {
  for (const b of a.rig.bones) if (b.mass && ids(b.id)) b.mass.kg = +(b.mass.kg * factor).toFixed(4);
}

/** ➕️ Sum of segment masses. */
export const massOf = (a: J): number => a.rig.bones.reduce((t: number, b: J) => t + (b.mass?.kg ?? 0), 0);

/** 🧍️ DuBois skin area in m² (DuBois and DuBois 1916). */
export const dubois = (kg: number, m: number): number => 0.007184 * kg ** 0.425 * (m * 100) ** 0.725;

/** 🔥️ Mifflin-St Jeor resting energy in W (kcal/day × 4184 / 86400). */
export const mifflin = (kg: number, m: number, age: number, male: boolean | "neutral"): number => {
  const s = male === "neutral" ? -78 : male ? 5 : -161;
  return ((10 * kg + 625 * m - 5 * age + s) * 4184) / 86400;
};

/** 🌍️ A bilingual text. */
export const t = (en: string, de: string): J => ({ en, de });

/** 🔍️ The bone record. */
export const bone = (a: J, id: string): J => {
  const b = a.rig.bones.find((x: J) => x.id === id);
  if (!b) throw new Error(`bone ${id}`);
  return b;
};

/** 🔍️ A shape by scale, view and id. */
export const shapeOf = (a: J, scale: string, view: string, id: string): J => a.representations[scale].geometry[view].find((s: J) => s.id === id);

/** 🔍️ A solid by scale, list and id. */
export const solidOf = (a: J, scale: string, list: string, id: string): J => a.volumes[scale][list].find((s: J) => s.id === id);

/** 🔍️ A thermal region. */
export const regionOf = (a: J, id: string): J => {
  const r = a.thermal.regions.find((x: J) => x.id === id);
  if (!r) throw new Error(`region ${id}`);
  return r;
};

/** 🧾️ Every pose binds every region exactly: missing ones get `binding`, unknown ones are removed. */
export function bindRegions(a: J, binding: (pose: J, region: string) => J): void {
  const ids = new Set(a.thermal.regions.map((r: J) => r.id));
  for (const p of a.poses) {
    for (const k of Object.keys(p.thermal.regions)) if (!ids.has(k)) delete p.thermal.regions[k];
    for (const id of ids) if (p.thermal.regions[id] === undefined) p.thermal.regions[id] = binding(p, id as string);
  }
}

/** 🕰️ Retimes a clip by a factor (keys, spans, phases, events, track keys, duration). */
export function retime(c: J, f: number): void {
  const r = (x: number): number => +(x * f).toFixed(6);
  c.durationS = r(c.durationS);
  for (const k of c.keys) k.t = r(k.t);
  for (const s of c.contacts) (s.fromS = r(s.fromS)), (s.toS = r(s.toS));
  for (const p of c.phases) (p.fromS = r(p.fromS)), (p.toS = r(p.toS));
  for (const e of c.events) e.t = r(e.t);
  for (const tr of c.tracks) for (const k of tr.keys) k.t = r(k.t);
}

/** 📏️ Sets `dimensions` (height, width, depth) to the body of the 1:50 volume in the default pose, the extent every scale is checked against, and the mass to the sum of the segment masses. */
export async function finishDimensions(a: J, scale = 50): Promise<number[]> {
  const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
  const { worldSolids, bodyDimensions } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
  const state = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const [w, h, d] = bodyDimensions(worldSolids(a, state.bones, scale)) as number[];
  a.dimensions = { heightM: +h!.toFixed(3), widthM: +w!.toFixed(3), depthM: +d!.toFixed(3), massKg: +massOf(a).toFixed(2) };
  return [w!, h!, d!];
}

/** 🦾️ Front and plan shapes of limb bones that a pose moves become prisms (`solid` extruded along w) of the elevation outline of the same id, spanning the lateral range of the front shape: exact in every pose, where outlines traced in one pose are not. */
export async function prismLimbs(a: J, bones: Set<string>): Promise<number> {
  const { flattenGeometry, solidDrawingProjector } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
  let n = 0;
  const made: { shape: J; view: string }[] = [];
  for (const s of SCALES) {
    const g = a.representations[s].geometry;
    for (const view of ["front", "plan"]) {
      for (const shape of g[view]) {
        if (!bones.has(shape.bone) || shape.solid !== undefined || shape.kind !== "poly") continue;
        const twin = g.elevation.find((e: J) => e.id === shape.id && e.bone === shape.bone && e.kind === "poly");
        const front = g.front.find((e: J) => e.id === shape.id && e.bone === shape.bone);
        if (!twin || !front) continue;
        const xs = front.points.map((p: number[]) => p[0]);
        const lo = Math.min(...xs), hi = Math.max(...xs);
        shape.points = structuredClone(twin.points);
        shape.closed = twin.closed;
        shape.solid = { kind: "extruded", sweep: "w", z0: +lo.toFixed(4), z1: +hi.toFixed(4) };
        made.push({ shape, view });
        n++;
      }
    }
  }
  void made, flattenGeometry, solidDrawingProjector;
  return n;
}

/** 🩹️ A prism whose projected outline the engine cannot union in some named pose (near-degenerate faces of a long traced profile) takes the convex hull of its profile. */
export async function repairPrisms(a: J): Promise<string[]> {
  const { flattenGeometry, solidDrawingProjector } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
  const fixed: string[] = [];
  for (const pose of a.poses) {
    const project = solidDrawingProjector(structuredClone(a), { kind: "pose", pose: pose.id });
    for (const { shape, view, scale } of shapes(a)) {
      if (!shape.solid || shape.kind !== "poly") continue;
      for (let attempt = 0; attempt < 6; attempt++) {
        try {
          project(shape, flattenGeometry(shape).rings, view);
          break;
        } catch {
          shape.points = attempt === 0 ? hull(shape.points) : shape.points.filter((_: J, i: number) => i % 3 !== 1 || shape.points.length <= 6).map((p: number[]) => p.map((x) => +(x + 0.0004 * attempt).toFixed(5)));
          fixed.push(`${scale}/${view}/${shape.id}/${attempt}`);
        }
      }
    }
  }
  return fixed;
}

/** 🪢️ The convex hull of points (monotone chain), counter-clockwise. */
export function hull(points: number[][]): number[][] {
  const p = [...points].sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]!);
  const cross = (o: number[], a: number[], b: number[]): number => (a[0]! - o[0]!) * (b[1]! - o[1]!) - (a[1]! - o[1]!) * (b[0]! - o[0]!);
  const lower: number[][] = [], upper: number[][] = [];
  for (const q of p) {
    while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop();
    lower.push(q);
  }
  for (const q of p.reverse()) {
    while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop();
    upper.push(q);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

/** ✂️ Cuts a loft at `at[0] = cut` (bone-local u), keeping the side where `keep(u)` holds and closing it with an interpolated section; returns false when nothing stays. */
export function trimLoft(solid: J, cut: number, keep: (u: number) => boolean): boolean {
  const secs: J[] = solid.sections;
  const out: J[] = [];
  for (let i = 0; i < secs.length; i++) {
    const s = secs[i];
    if (keep(s.at[0])) out.push(s);
    const n = secs[i + 1];
    if (n && keep(s.at[0]) !== keep(n.at[0])) {
      const f = (cut - s.at[0]) / (n.at[0] - s.at[0]);
      const lerp = (p: number, q: number): number => +(p + (q - p) * f).toFixed(5);
      out.push({ ...s, at: [0, 1, 2].map((k) => lerp(s.at[k], n.at[k])), rx: lerp(s.rx, n.rx), ry: lerp(s.ry, n.ry), exponent: lerp(s.exponent ?? 2, n.exponent ?? 2) });
    }
  }
  solid.sections = out;
  solid.capStart = true;
  solid.capEnd = true;
  return out.length >= 2;
}

/** 🪶️ Thins a limb: every solid of the bone (lofts, capsules, ellipsoids) and every elevation shape of the bone shrink by `f` across the bone axis (v and w), about the bone axis. */
export function thinBone(a: J, bone: string, f: number, pred: (id: string) => boolean = () => true): void {
  for (const { solid } of solids(a)) {
    if (solid.bone !== bone || !pred(solid.id)) continue;
    if (solid.kind === "loft") for (const c of solid.sections) (c.rx = +(c.rx * f).toFixed(5)), (c.ry = +(c.ry * f).toFixed(5)), (c.at = [c.at[0], +(c.at[1] * f).toFixed(5), +(c.at[2] * f).toFixed(5)]);
    else if (solid.kind === "capsule") (solid.r0 = +(solid.r0 * f).toFixed(5)), (solid.r1 = +(solid.r1 * f).toFixed(5));
    else if (solid.kind === "ellipsoid") solid.radii = [solid.radii[0], +(solid.radii[1] * f).toFixed(5), +(solid.radii[2] * f).toFixed(5)];
  }
  for (const s of SCALES) for (const shape of a.representations[s].geometry.elevation) if (shape.bone === bone && shape.kind === "poly" && pred(shape.id)) shape.points = shape.points.map((p: number[]) => [p[0], +(p[1]! * f).toFixed(5)]);
}
