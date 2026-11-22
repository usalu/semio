/** 🌳️ PW-08 asset assembly: turns a species design (bones, per-scale parts as world meshes, drawing groups, physics and texts, all hand-chosen in the species file) into the shipped `semio.staffage/v2` document: rig, masses, sections, anchors, poses with loads from EN 1991-1-4, clips, thermal regions and emitters, the four representations drawn as true orthographic views of each scale's own body, the plan symbol and the three volume lists per scale.
 *
 * @see ./lib.ts
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts */
import {
  BANDS, FEATURE_M, Grid, SCALES, VIEWS, add, area, bounds, centroid, chaikin, clamp, dot, edt, fillHoles, frames, islands, morph, mul, nearness, outerRing, paper, r1, r2, r3, r4, ringArea, rng, simplify, sub, t, tipOf, toLocal, tube, unionArea, volume,
  type BoneSpec, type Frame, type Mesh, type P2, type Scale, type Text, type V3, type View,
} from "./lib.ts";

export type Region = "wood" | "foliage";
/** 🧩️ One body part of one scale: a closed world-space mesh on a bone, of one thermal region. */
export type Part = { id: string; bone: string; region: string; material: string; tone: number; mesh: Mesh; kind: Region };
/** 🖌️ A drawing group of one scale: the parts drawn as one outline per view, bound to `bone`. */
export type Group = { id: string; bone: string; parts: string[]; regions: string[]; material: string; tone: number };
/** ✒️ A texture mark generator: paper-space polylines of one view placed at a point of a group. */
export type MarkFn = (view: View, scale: Scale, at: P2, group: Group, random: () => number) => P2[][];
export type ScaleDesign = {
  parts: Part[];
  groups?: Group[];
  pens: { body: number; detail: number | null; texture: number | null; glazing: null; ground: number };
  marks?: { spacingM: number; marginM: number; groups: (g: Group) => boolean; fn: MarkFn; views: View[] }[];
  decisions: { silhouette: Text; retained: Text; merged: Text; omitted: Text; thinElements: Text; hatching: Text; styles: Text; overlays: Text };
  volumeDecisions: Text[];
  members: Member[];
  joints: { bone: string; at: V3; r: number }[];
  masses: { bone: string; member: string; at: V3; r: number; material: string }[];
  supportR: number;
  emitterSolids: { emitter: string; region: string; bone: string; at: V3; radii: V3; material: string }[];
};
/** 🏗️ A structural member solid: a tapered round bar along the bone axis (world points), slender where the scale cannot show it thicker. */
export type Member = { bone: string; from: V3; to: V3; d0: number; d1: number; material: string; slender?: boolean; members?: string[] };

const ROOT = "../../../../../../../../../";
export const SCHEMA = "../../../🧬️schema/🔣️.json#/$defs/AssetDocument";

/** 📐️ The raster cell for drawing a scale of an asset whose longest extent is `longest`. */
const cellFor = (scale: Scale, longest: number): number => Math.max(FEATURE_M[scale] / 3, longest / 6000);

type Drawn = { group: Group; view: View; rings: P2[][]; near: number; z: number };

/** 🖼️ The paper outlines of every group of a scale in every view, from the union of its parts' projections, closed by half a feature, holes filled, islands under four features squared dropped, simplified within a third of a feature and softened once. */
export function drawGroups(parts: readonly Part[], groups: readonly Group[], scale: Scale, longest: number): Drawn[] {
  const f = FEATURE_M[scale];
  const cell = cellFor(scale, longest);
  const byId = new Map(parts.map((p) => [p.id, p]));
  const out: Drawn[] = [];
  for (const view of VIEWS) {
    for (const g of groups) {
      const ms = g.parts.map((id) => byId.get(id)!.mesh);
      const pts = ms.flatMap((m) => m.p.map((q) => paper(view, q)));
      const grid = Grid.around(pts, cell, 4 * f + 4 * cell);
      for (const m of ms) grid.mesh(view, m);
      const closed = morph(morph(grid, f / 2), -f / 2);
      closed.or(grid);
      const full = fillHoles(closed);
      const rings: P2[][] = [];
      for (const isl of islands(full)) {
        if (isl.count() * cell * cell < 4 * f * f) continue;
        const raw = outerRing(isl);
        const soft = simplify(chaikin(raw, 2), Math.max(cell / 2, f / 4));
        if (soft.length >= 3 && Math.abs(ringArea(soft)) > 4 * f * f) rings.push(soft);
      }
      const near = Math.max(...ms.flatMap((m) => m.p.map((q) => nearness(view, q))));
      out.push({ group: g, view, rings, near, z: 0 });
    }
    const mine = out.filter((d) => d.view === view).sort((a, b) => a.near - b.near);
    mine.forEach((d, i) => { d.z = r3(1 + i * 0.01); });
  }
  return out;
}

/** 🧭️ Paper coordinates of a view into the coordinates of a shape of that view bound to the frame `f` (elevation: bone-local u, v; front: lateral, height; plan: ahead, lateral). */
export const toShape = (view: View, f: Frame, q: P2): P2 => {
  if (view !== "elevation") return q;
  const d: V3 = [q[0] - f.o[0], q[1] - f.o[1], 0];
  return [dot(d, f.u), dot(d, f.v)];
};

/** ✍️ Texture marks of a view: centres on a jittered grid inside the visible part of a group (painted last-on-top by `z`), at least `marginM` from every visible outline. */
export function marksFor(drawn: readonly Drawn[], view: View, scale: Scale, spec: NonNullable<ScaleDesign["marks"]>[number], seed: number): { group: Group; z: number; lines: P2[][] }[] {
  const mine = drawn.filter((d) => d.view === view).sort((a, b) => a.z - b.z);
  const pts = mine.flatMap((d) => d.rings.flat());
  if (pts.length === 0) return [];
  const cell = Math.max(FEATURE_M[scale] / 2, spec.marginM / 6);
  const owner = Grid.around(pts, cell, 2 * spec.marginM);
  const label = new Int32Array(owner.w * owner.h).fill(-1);
  mine.forEach((d, k) => {
    const g = owner.like();
    for (const r of d.rings) g.ring(r);
    for (let i = 0; i < g.d.length; i++) if (g.d[i]) label[i] = k;
  });
  const edge = owner.like();
  for (let j = 0; j < owner.h; j++) for (let i = 0; i < owner.w; i++) {
    const k = j * owner.w + i;
    const l = label[k]!;
    const diff = (ii: number, jj: number) => ii < 0 || jj < 0 || ii >= owner.w || jj >= owner.h ? l !== -1 : label[jj * owner.w + ii] !== l;
    if (diff(i - 1, j) || diff(i + 1, j) || diff(i, j - 1) || diff(i, j + 1)) edge.d[k] = 1;
  }
  const dist = edt(edge, (v) => v === 0);
  const ok = (q: P2, k: number): boolean => {
    const i = Math.floor((q[0] - owner.x0) / cell), j = Math.floor((q[1] - owner.y0) / cell);
    if (i < 0 || j < 0 || i >= owner.w || j >= owner.h) return false;
    return label[j * owner.w + i] === k && Math.sqrt(dist[j * owner.w + i]!) * cell >= spec.marginM;
  };
  const random = rng(seed);
  const result = new Map<number, P2[][]>();
  const s = spec.spacingM;
  const [x0, y0] = [owner.x0, owner.y0];
  for (let y = y0 + s / 2; y < y0 + owner.h * cell; y += s) {
    for (let x = x0 + s / 2; x < x0 + owner.w * cell; x += s) {
      const q: P2 = [x + (random() - 0.5) * s * 0.5, y + (random() - 0.5) * s * 0.5];
      const i = Math.floor((q[0] - owner.x0) / cell), j = Math.floor((q[1] - owner.y0) / cell);
      if (i < 0 || j < 0 || i >= owner.w || j >= owner.h) continue;
      const k = label[j * owner.w + i]!;
      if (k < 0 || !spec.groups(mine[k]!.group)) continue;
      const lines = spec.fn(view, scale, q, mine[k]!.group, random);
      if (lines.length === 0 || !lines.every((l) => l.every((p) => ok(p, k)))) continue;
      (result.get(k) ?? result.set(k, []).get(k)!).push(...lines);
    }
  }
  return [...result.entries()].map(([k, lines]) => ({ group: mine[k]!.group, z: mine[k]!.z, lines }));
}

/** 🧾️ Everything a species file states besides geometry. */
export type Species = {
  id: string;
  title: Text;
  summary: Text;
  tags: string[];
  references: { id: string; citation: string }[];
  notes: Text;
  materials: unknown[];
  bones: BoneSpec[];
  scales: Record<Scale, ScaleDesign>;
  woodDensity: Record<string, number>;
  foliageDensity: Record<string, number>;
  hiddenWood: (bone: string) => number;
  regions: unknown[];
  emitters: { id: string; region: string; title: Text; anchors: { bone: string; at: V3 }[]; diameterM: number; source: string; reference: string }[];
  thermalEnvironment: unknown;
  thermalAssumptions: Text[];
  structureAssumptions: Text[];
  poses: unknown[];
  clips: unknown[];
  symbol: { title: Text; description: Text; note: Text };
  defaultScale: Scale;
  symbolStemD: number;
  distinction: Text;
  massSource: { wood: string; foliage: string; reference: string };
};

/** 🦴️ The schema rig bones with masses from the 1:50 parts (volume times green density for wood, crown bulk density for foliage, plus the hidden branch wood of a foliage bone). */
export function rigOf(sp: Species): { bones: unknown[]; masses: Record<string, { kg: number; com: V3 }> } {
  const fr = frames(sp.bones);
  const parts = sp.scales[50].parts;
  const masses: Record<string, { kg: number; com: V3 }> = {};
  for (const b of sp.bones) {
    const mine = parts.filter((p) => p.bone === b.id);
    let kg = 0;
    let mom: V3 = [0, 0, 0];
    for (const p of mine) {
      const v = volume(p.mesh);
      const rho = p.kind === "wood" ? sp.woodDensity[p.material]! : sp.foliageDensity[p.material]!;
      const m = v * rho;
      kg += m;
      mom = add(mom, mul(centroid(p.mesh), m));
    }
    const hidden = sp.hiddenWood(b.id);
    if (hidden > 0) {
      kg += hidden;
      mom = add(mom, mul(add(b.o, mul(sub(tipOf(b), b.o), 0.45)), hidden));
    }
    masses[b.id] = { kg, com: kg > 0 ? mul(mom, 1 / kg) : b.o };
  }
  const bones = sp.bones.map((b) => {
    const parent = b.parent === null ? null : sp.bones.find((x) => x.id === b.parent)!;
    const pf = parent === null ? null : fr[parent.id]!;
    const local = pf === null ? b.o : toLocal(pf, b.o);
    const m = masses[b.id]!;
    const com = toLocal(fr[b.id]!, m.com);
    const isWood = parts.some((p) => p.bone === b.id && p.kind === "wood") && !parts.some((p) => p.bone === b.id && p.kind === "foliage");
    const section = b.section === undefined ? undefined : { areaM2: r4(sec(b.section.d).a), secondMomentM4: Number(sec(b.section.d).i.toPrecision(4)), extremeFibreM: r4(b.section.d / 2), material: b.section.material, title: b.section.title };
    return {
      id: b.id, parent: b.parent, title: b.title, x: r4(local[0]), y: r4(local[1]), ...(Math.abs(local[2]) > 1e-6 ? { z: r4(local[2]) } : {}),
      angleDeg: r3(b.angle - (parent?.angle ?? 0)), lengthM: r3(b.length), joint: b.joint,
      ...(m.kg > 0 ? { mass: isWood ? { kg: r1(m.kg), comX: r3(b.length / 2), comY: 0, distribution: "uniform", source: sp.massSource.wood, reference: sp.massSource.reference } : { kg: r1(m.kg), comX: r3(com[0]), comY: r3(com[1]), distribution: "point", source: sp.massSource.foliage, reference: sp.massSource.reference } } : {}),
      ...(section === undefined ? {} : { section }),
    };
  });
  return { bones, masses };
}

export const sec = (d: number) => ({ a: (Math.PI * d * d) / 4, i: (Math.PI * d ** 4) / 64 });

/** 🧱️ A polyhedron solid of a world mesh in the frame of its bone. */
function polyOf(id: string, bone: string, f: Frame, m: Mesh, material: string, role: string, tone: number, extra: Record<string, unknown> = {}): Record<string, unknown> {
  return { id, kind: "polyhedron", bone, material, role, tone, ...extra, vertices: m.p.map((q) => toLocal(f, q).map(r4)), triangles: m.t };
}

/** 🧊️ The authored volumes of every scale: body polyhedra, thermal references and emitter outlets, structure members, joints, support and mass markers. */
export function volumesOf(sp: Species): Record<string, unknown> {
  const fr = frames(sp.bones);
  const out: Record<string, unknown> = {};
  for (const scale of SCALES) {
    const d = sp.scales[scale];
    const f = FEATURE_M[scale];
    const body = d.parts.map((p) => polyOf(p.id, p.bone, fr[p.bone]!, p.mesh, p.material, "body", p.tone));
    const thermal: unknown[] = d.parts.map((p) => ({ id: `th-${p.id}`, of: p.id, region: p.region }));
    d.emitterSolids.forEach((e, k) => {
      const c = toLocal(fr[e.bone]!, e.at);
      thermal.push({ id: `th-${e.emitter}-${k + 1}`, kind: "ellipsoid", bone: e.bone, material: e.material, role: "detail", tone: 0.5, region: e.region, tags: [e.emitter], radii: e.radii, transform: { translation: c.map(r4) } });
    });
    const structure: unknown[] = d.members.map((m) => {
      const mesh = tube([{ c: m.from, r: m.d0 / 2 }, { c: add(m.from, mul(sub(m.to, m.from), 0.5)), r: (m.d0 + m.d1) / 4 }, { c: m.to, r: m.d1 / 2 }], { 50: 12, 100: 10, 200: 8, 500: 6 }[scale]);
      return polyOf(`member-${m.bone}`, m.bone, fr[m.bone]!, mesh, m.material, "body", 0.4, { member: m.bone, part: "member", ...(m.slender ? { slender: true } : {}), ...(m.members ? { members: m.members } : {}) });
    });
    for (const j of d.joints) structure.push({ id: `joint-${j.bone}`, kind: "ellipsoid", bone: j.bone, material: d.members.find((m) => m.bone === j.bone)?.material ?? d.members[0]!.material, role: "detail", tone: 0.35, member: j.bone, part: "joint", radii: [j.r, j.r, j.r], transform: { translation: toLocal(fr[j.bone]!, j.at).map(r4) } });
    structure.push({ id: "support-base-support", kind: "revolve", bone: sp.bones[0]!.id, material: d.members[0]!.material, role: "detail", tone: 0.25, member: sp.bones[0]!.id, part: "support", anchor: "base-support", profile: [[-Math.max(f, 0.04), d.supportR], [0, d.supportR]], axis: [1, 0, 0], origin: [0, 0, 0] });
    for (const m of d.masses) structure.push({ id: `mass-${m.bone}`, kind: "ellipsoid", bone: m.bone, material: m.material, role: "detail", tone: 0.3, member: m.member, part: "mass", radii: [m.r, m.r, m.r], transform: { translation: toLocal(fr[m.bone]!, m.at).map(r4) } });
    out[String(scale)] = { status: "authored", denominator: scale, body, thermal, structure, decisions: d.volumeDecisions };
  }
  return out;
}

/** 🗺️ The four representations: per scale and view the group outlines (true views of that scale's body) and the texture marks, with pens, ranges and decisions. */
export function representationsOf(sp: Species, longest: number): { reps: Record<string, unknown>; drawn: Record<number, Drawn[]> } {
  const fr = frames(sp.bones);
  const reps: Record<string, unknown> = {};
  const drawnAll: Record<number, Drawn[]> = {};
  for (const scale of SCALES) {
    const d = sp.scales[scale];
    const groups = d.groups ?? defaultGroups(d.parts);
    const drawn = drawGroups(d.parts, groups, scale, longest);
    drawnAll[scale] = drawn;
    const geometry: Record<View, unknown[]> = { elevation: [], front: [], plan: [] };
    for (const view of VIEWS) {
      const items: { z: number; shape: Record<string, unknown> }[] = [];
      for (const dr of drawn.filter((x) => x.view === view)) {
        dr.rings.forEach((ring, k) => {
          const f = fr[dr.group.bone]!;
          items.push({ z: dr.z, shape: { id: k === 0 ? dr.group.id : `${dr.group.id}-${k + 1}`, bone: dr.group.bone, role: "body", z: dr.z, filled: true, tone: dr.group.tone, ...(dr.group.regions.length === 1 ? { region: dr.group.regions[0] } : { regions: dr.group.regions }), material: dr.group.material, kind: "poly", points: ring.map((q) => toShape(view, f, q).map(r4)), closed: true } });
        });
      }
      (d.marks ?? []).forEach((spec, si) => {
        if (!spec.views.includes(view)) return;
        for (const m of marksFor(drawn, view, scale, spec, scale * 31 + si * 7 + VIEWS.indexOf(view))) {
          const f = fr[m.group.bone]!;
          m.lines.forEach((line, k) => items.push({ z: r4(m.z + 0.001 + si * 0.0001), shape: { id: `${m.group.id}-mark-${si}-${k + 1}`, bone: m.group.bone, role: "texture", z: r4(m.z + 0.001 + si * 0.0001), filled: false, tone: m.group.tone, ...(m.group.regions.length === 1 ? { region: m.group.regions[0] } : { regions: m.group.regions }), material: m.group.material, kind: "poly", points: line.map((q) => toShape(view, f, q).map(r4)), closed: false } }));
        }
      });
      geometry[view] = items.sort((a, b) => a.z - b.z).map((x) => x.shape);
    }
    reps[String(scale)] = { status: "authored", denominator: scale, validRange: { minDenominator: BANDS[scale][0], maxDenominator: BANDS[scale][1] }, pens: d.pens, hatches: [], geometry, decisions: d.decisions };
  }
  return { reps, drawn: drawnAll };
}

/** 🧺️ One group per bone and region: the parts of a bone that belong to one thermal region are drawn as one outline. */
export function defaultGroups(parts: readonly Part[]): Group[] {
  const keys = new Map<string, Part[]>();
  for (const p of parts) (keys.get(`${p.bone}|${p.region}`) ?? keys.set(`${p.bone}|${p.region}`, []).get(`${p.bone}|${p.region}`)!).push(p);
  return [...keys.values()].map((ps) => ({ id: ps.length === parts.filter((p) => p.bone === ps[0]!.bone).length ? ps[0]!.bone : `${ps[0]!.bone}-${ps[0]!.region}`, bone: ps[0]!.bone, parts: ps.map((p) => p.id), regions: [ps[0]!.region], material: ps[0]!.material, tone: ps[0]!.tone }));
}

/** 🧩️ One group per part: every part of a scale is drawn as its own outline (several parts may share a bone). */
export const onePerPart = (parts: readonly Part[]): Group[] => parts.map((p) => ({ id: p.id, bone: p.bone, parts: [p.id], regions: [p.region], material: p.material, tone: p.tone }));

/** 🔣️ The plan symbol with the true crown extent: the 1:500 outlines of every view merged and redrawn as one soft closed outline per view, and the stem section. */
export function symbolOf(sp: Species, drawn500: readonly Drawn[], stem: { bone: string; d: number; topY: number }): unknown {
  const geometry: Record<View, unknown[]> = { elevation: [], front: [], plan: [] };
  const all = bounds(sp.scales[50].parts.map((p) => p.mesh));
  const fol = bounds(sp.scales[50].parts.filter((p) => p.kind === "foliage").map((p) => p.mesh));
  const target: Record<View, [[number, number], [number, number]]> = { elevation: [[all.min[0], all.max[0]], [fol.min[1], all.max[1]]], front: [[all.min[2], all.max[2]], [fol.min[1], all.max[1]]], plan: [[all.min[0], all.max[0]], [all.min[2], all.max[2]]] };
  const root = sp.bones[0]!;
  for (const view of VIEWS) {
    const rings = drawn500.filter((d) => d.view === view && d.group.regions.some((r) => /foliage|needle/.test(r))).flatMap((d) => d.rings);
    const pts = rings.flat();
    const cell = 0.05;
    const g = Grid.around(pts, cell, 1);
    for (const r of rings) g.ring(r);
    const closed = fillHoles(morph(morph(g, 0.4), -0.4));
    const isl = islands(closed)[0]!;
    const raw = simplify(chaikin(outerRing(isl), 3), 0.04);
    const [tx, ty] = target[view];
    const bx = [Math.min(...raw.map((q) => q[0])), Math.max(...raw.map((q) => q[0]))], by = [Math.min(...raw.map((q) => q[1])), Math.max(...raw.map((q) => q[1]))];
    const ring = raw.map((q): P2 => [tx[0] + ((q[0] - bx[0]!) / (bx[1]! - bx[0]!)) * (tx[1] - tx[0]), ty[0] + ((q[1] - by[0]!) / (by[1]! - by[0]!)) * (ty[1] - ty[0])]);
    geometry[view].push({ id: "crown", bone: root.id, role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: ring.map((q) => q.map(r4)), closed: true });
    if (view === "plan") {
      const n = 20;
      geometry[view].push({ id: "stem", bone: root.id, role: "body", z: 3, filled: true, tone: 0.25, kind: "poly", points: Array.from({ length: n }, (_, k) => [r4((stem.d / 2) * Math.cos((2 * Math.PI * k) / n)), r4((stem.d / 2) * Math.sin((2 * Math.PI * k) / n))]), closed: true });
    } else {
      const trunk = drawn500.filter((d) => d.view === view && d.group.regions.length === 1 && /bark/.test(d.group.regions[0]!)).flatMap((d) => d.rings);
      const tp = trunk.flat();
      const gg = Grid.around(tp, 0.03, 0.3);
      for (const r of trunk) gg.ring(r);
      const one = islands(fillHoles(morph(morph(gg, 0.1), -0.1)))[0]!;
      geometry[view].push({ id: "stem", bone: root.id, role: "body", z: 3, filled: true, tone: 0.3, kind: "poly", points: simplify(chaikin(outerRing(one), 2), 0.015).map((q) => q.map(r4)), closed: true });
    }
  }
  return { id: `${sp.id}-symbol`, title: sp.symbol.title, description: sp.symbol.description, semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry, dimensionalNote: sp.symbol.note };
}

/** 📏️ The default-pose envelope of the 1:50 body (the rest pose, which every calm default pose is). */
export function dimensionsOf(sp: Species, massKg: number): { heightM: number; widthM: number; depthM: number; massKg: number } {
  const b = bounds(sp.scales[50].parts.map((p) => p.mesh));
  return { heightM: r3(b.max[1] - Math.min(0, b.min[1])), widthM: r3(b.max[0] - b.min[0]), depthM: r3(b.max[2] - b.min[2]), massKg: r1(massKg) };
}

/** 🌫️ The outer area of the union of the 1:50 parts of a region (m²). */
export const regionArea = (sp: Species, region: string, scale: Scale = 50): number => unionArea(sp.scales[scale].parts.filter((p) => p.region === region).map((p) => p.mesh));

/** 🪟️ Projected area (m²) of meshes in a view, rastered at `cell`. */
export function projectedArea(ms: readonly Mesh[], view: View, cell = 0.02): number {
  const pts = ms.flatMap((m) => m.p.map((q) => paper(view, q)));
  const g = Grid.around(pts, cell, 2 * cell);
  for (const m of ms) g.mesh(view, m);
  return g.count() * cell * cell;
}

export { area, bounds, centroid, clamp, r1, r2, r3, r4, t, volume, ROOT };

/** 🧮️ What the build hands a species' physics: the schema bones and the 1:50 bone masses. */
export type Ctx = { bones: unknown[]; masses: Record<string, { kg: number; com: V3 }> };
/** 🌡️ What a species' physics returns. */
export type Physics = { regions: unknown[]; poses: unknown[]; clips: unknown[]; thermalAssumptions: Text[]; structureAssumptions: Text[]; supportFootprintM: number };
