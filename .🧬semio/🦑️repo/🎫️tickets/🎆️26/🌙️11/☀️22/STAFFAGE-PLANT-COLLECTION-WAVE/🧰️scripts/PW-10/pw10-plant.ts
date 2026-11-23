/** 🌴️ Shared builder of worker PW-10 for rosette plants (a stem or pseudostem with a terminal crown of two-bone leaves): rig, crown table, poses, clips, loads, thermal, structure, the per-scale volumes and the 2D drawings as true views of each scale's body. It reuses the date-palm generator of round 3 (kit, `FrondGeom`, the product placement `forward`); every form and number is stated by hand in the species files.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-model.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-kit.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { STAFFAGE, meshesAt, type Frame, type Json, type P2, type V3, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";
import { forward } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-project2d.ts";
import { D2R, add, blobOutline, cross, dirOf, dot, ellipsoidOf, hullOf, len, localPoint, loftOf, mul, nearness, outline, pathOf, pointsOf, project, quaternion, r4, r5, sub, tubePolygons, unit, upOf, type Blob, type Section, type Tube } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-kit.ts";
import { FrondGeom } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-parts.ts";
import type { Frond } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-date-palm-rig-fix.ts";
import { polyhedronFault } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { triangulatePolygon } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🔺️polygon/🟦️.ts";

export { D2R, FrondGeom, add, cross, dirOf, dot, len, mul, r4, r5, sub, unit, upOf };
export type { Blob, Frame, Frond, Json, P2, Section, Tube, V3, View };

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };

export const SCALES = [50, 100, 200, 500] as const;
export type Scale = (typeof SCALES)[number];
export const VIEWS: View[] = ["elevation", "front", "plan"];
export const WAVE = join(STAFFAGE, "../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE");
export const OUT = join(WAVE, "🗑️generated", "PW-10");
export const text = (en: string, de: string) => ({ en, de });
export type Text = { en: string; de: string };
export const featureOf = (scale: number): number => (0.15 * scale) / 1000;

/** 📈️ Piecewise linear interpolation in a table of [x, y] rows sorted by x. */
export function table(rows: readonly (readonly [number, number])[], x: number): number {
  if (x <= rows[0]![0]) return rows[0]![1];
  for (let i = 1; i < rows.length; i++) if (x <= rows[i]![0]) {
    const [a, b] = [rows[i - 1]!, rows[i]!];
    return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]);
  }
  return rows[rows.length - 1]![1];
}

/** 🪜️ Evenly spaced values from `from` to `to` with steps of at most `step`. */
export const steps = (from: number, to: number, step: number): number[] => {
  const n = Math.max(1, Math.ceil((to - from) / step - 1e-9));
  return Array.from({ length: n + 1 }, (_, i) => from + ((to - from) * i) / n);
};

/** 🌿️ One leaf of the crown: a `Frond` of the date-palm kit plus the crown bone it grows from, its life state and the masses of its two bones. */
export type Leaf = Frond & { crown: string; state: "green" | "dead"; massA: number; massB: number; label: Text };

/** 🎚️ One tier of a crown table: count, chord elevations, chord lengths, insertion along the crown bone and radius from the axis. */
export type Tier = { tier: "A" | "B" | "C" | "D"; count: number; elA: number; elB: number; La: number; Lb: number; baseU: number; baseR: number; massA: number; massB: number; state?: "green" | "dead"; label: Text };

/** 🌀️ The leaves of one crown, oldest first, spaced by the golden angle from `az0`; `reach` scales the chords per azimuth (an elliptic crown), `wobble` breaks the regularity by hand-chosen steps. */
export function crownTable(opts: { crown: string; prefix: string; az0: number; tiers: Tier[]; reach?: (az: number, tier: string) => number; wobble?: readonly number[]; divergence?: number }): Leaf[] {
  const out: Leaf[] = [];
  let k = 0;
  const wob = opts.wobble ?? [0.6, -0.9, 0.3, -0.4, 0.8, -0.2, 0.5, -0.7];
  const count = { l: 0, r: 0 };
  for (const t of opts.tiers) for (let i = 0; i < t.count; i++, k++) {
    const az = ((((opts.az0 + k * (opts.divergence ?? 137.508)) % 360) + 540) % 360) - 180;
    const w = wob[k % wob.length]!;
    const s = opts.reach ? opts.reach(az, t.tier) : 1;
    const side: "l" | "r" = Math.sin(az * D2R) <= 0 ? "l" : "r";
    const n = ++count[side];
    const elA = t.elA + w * 2, elB = t.elB + w * 3;
    out.push({ k, tier: t.tier, az: Math.round(az * 10) / 10, elA, elB, La: Math.round(t.La * s * 1000) / 1000, Lb: Math.round(t.Lb * s * 1000) / 1000, baseU: t.baseU + (i % 2) * 0.02, baseR: t.baseR, side, n, id: `${opts.prefix}-${side}${n}`, restA: 0, restB: Math.round((elB - elA) * 1e4) / 1e4, crown: opts.crown, state: t.state ?? "green", massA: t.massA, massB: t.massB, label: t.label });
  }
  return out;
}

/** 🦴️ A stem-like bone of the rig as stated by hand. */
export type Bone = Json;

/** 🧮️ Offsets (angle from rest, abduction, twist) of a bone whose world axis is `d` with its normal `up`, under the parent frame `P`. */
export function solveOffsets(P: Frame, d: V3, up: V3, restDeg: number): { angleDeg: number; abductionDeg: number; twistDeg: number } {
  const [du, dv, dw] = [dot(d, P.u), dot(d, P.v), dot(d, P.w)];
  const phi = Math.asin(Math.max(-1, Math.min(1, dw)));
  const theta = Math.atan2(dv, du);
  const u1 = add(mul(P.u, Math.cos(theta)), mul(P.v, Math.sin(theta)));
  const v1 = add(mul(P.u, -Math.sin(theta)), mul(P.v, Math.cos(theta)));
  const w2 = add(mul(u1, -Math.sin(phi)), mul(P.w, Math.cos(phi)));
  const psi = Math.atan2(-dot(up, w2), dot(up, v1));
  let angle = theta / D2R - restDeg;
  while (angle > 180) angle -= 360;
  while (angle < -180) angle += 360;
  const r = (x: number) => Math.round(x * 1e4) / 1e4;
  return { angleDeg: r(angle), abductionDeg: r(phi / D2R), twistDeg: r(psi / D2R) };
}

/** 🌬️ The response of the crown to one load case: proximal and distal elevation changes of the windward (azimuth 180, facing the wind) and leeward (azimuth 0) leaves, the share by which leaves turn towards the lee, the blade roll of side leaves, a uniform elevation change (snow, wilting) and per-leaf phase flutter. */
export type Response = { windward: [number, number]; leeward: [number, number]; stream: number; roll: number; uniform?: [number, number]; flutter?: (leaf: Leaf) => [number, number] };
export const STILL: Response = { windward: [0, 0], leeward: [0, 0], stream: 0, roll: 0 };

/** 🌴️ Rotates `p` about `axis` through the origin by `ang`. */
export const rotate = (p: V3, axis: V3, ang: number): V3 => {
  const k = unit(axis);
  const [c, s] = [Math.cos(ang), Math.sin(ang)];
  return add(add(mul(p, c), mul(cross(k, p), s)), mul(k, dot(k, p) * (1 - c)));
};

/** 🧭️ The rest angles of the leaves (so that the calm pose needs no angle offset) and their bones under each crown. */
export function leafBones(rig: Json, leaves: Leaf[]): Bone[] {
  const frames = solveChain(rig, {}, { x: 0, y: 0, angleDeg: 0 });
  const bones: Bone[] = [];
  for (const f of leaves) {
    const P = frames[f.crown]!.frame;
    const d = dirOf(f.az, f.elA);
    f.restA = Math.round((Math.atan2(dot(d, P.v), dot(d, P.u)) / D2R) * 1e4) / 1e4;
    const radial: V3 = [Math.cos(f.az * D2R), 0, -Math.sin(f.az * D2R)];
    const off = mul(radial, f.baseR);
    const side = f.side === "l" ? text("left", "links") : text("right", "rechts");
    bones.push({
      id: `${f.id}a`, parent: f.crown,
      title: text(`${f.label.en} ${f.n} ${side.en}, base and lower part`, `${f.label.de} ${f.n} ${side.de}, Basis und unterer Teil`),
      x: r4(f.baseU), y: r4(dot(off, P.v)), z: r4(dot(off, P.w)), angleDeg: f.restA, lengthM: f.La,
      joint: { type: "revolute", minDeg: -45, maxDeg: 45 },
      mass: { kg: f.massA, comX: r4(f.La * 0.45), comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" },
    });
    bones.push({
      id: `${f.id}b`, parent: `${f.id}a`,
      title: text(`${f.label.en} ${f.n} ${side.en}, upper part`, `${f.label.de} ${f.n} ${side.de}, oberer Teil`),
      x: f.La, y: 0, angleDeg: f.restB, lengthM: f.Lb,
      joint: { type: "revolute", minDeg: -30, maxDeg: 30 },
      mass: { kg: f.massB, comX: r4(f.Lb * 0.45), comY: 0, distribution: "point", source: "authored-estimate", reference: "jones-2014" },
    });
  }
  return bones;
}

/** 🎛️ Bone offsets of a whole pose: the stem offsets as given, every leaf solved from its table direction changed by the response. */
export function poseOffsets(rig: Json, leaves: Leaf[], stem: Record<string, Json>, resp: Response): Json {
  const frames = solveChain(rig, stem, { x: 0, y: 0, angleDeg: 0 });
  const out: Json = { ...stem };
  for (const f of leaves) {
    const P = frames[f.crown]!.frame;
    const t = (1 - Math.cos(f.az * D2R)) / 2;
    const fl = resp.flutter ? resp.flutter(f) : [0, 0];
    const uni = resp.uniform ?? [0, 0];
    const dead = f.state === "dead" ? 0.4 : 1;
    const da = (t * resp.windward[0] + (1 - t) * resp.leeward[0]) * dead + uni[0] + fl[0];
    const db = (t * resp.windward[1] + (1 - t) * resp.leeward[1]) * dead + uni[1] + fl[1];
    const az = f.az - resp.stream * Math.sin(f.az * D2R) * 40;
    const ea = Math.max(-89, Math.min(89, f.elA + da));
    const d = dirOf(az, ea);
    let up = upOf(az, ea);
    const roll = resp.roll * Math.sin(f.az * D2R);
    if (roll !== 0) up = rotate(up, d, roll * D2R);
    out[`${f.id}a`] = solveOffsets(P, d, up, f.restA);
    const bend = Math.round((db - da) * 1e4) / 1e4;
    if (bend !== 0) out[`${f.id}b`] = { angleDeg: bend };
  }
  return out;
}

/** 🗺️ The frames of every bone in a pose. */
export const framesOf = (rig: Json, offsets: Json): Record<string, Frame> => Object.fromEntries(Object.entries(solveChain(rig, offsets, { x: 0, y: 0, angleDeg: 0 })).map(([id, s]) => [id, s.frame]));

/** 🕸️ A planar mesh of a thin part: vertices on the mid surface in world space with their surface normal, and quads (or triangles) in counter-clockwise order seen along the normal. */
export type Sheet = { id: string; bone: string; p: V3[]; n: V3[]; faces: number[][]; half: number | number[]; fields: Json; region?: string; faceU?: number[] };

/** 🧱️ A closed polyhedron from a sheet: top and bottom faces at ± half thickness along the normals, walls along the boundary; outward orientation by the signed volume. */
export function thicken(s: Sheet): { verts: V3[]; tris: [number, number, number][] } {
  const N = s.p.length;
  const h = (i: number) => (Array.isArray(s.half) ? s.half[i]! : s.half);
  const verts: V3[] = [...s.p.map((p, i) => add(p, mul(s.n[i]!, h(i)))), ...s.p.map((p, i) => sub(p, mul(s.n[i]!, h(i))))];
  const tris: [number, number, number][] = [];
  const edges = new Map<string, number>();
  for (const f of s.faces) {
    for (let i = 1; i + 1 < f.length; i++) {
      tris.push([f[0]!, f[i]!, f[i + 1]!]);
      tris.push([N + f[0]!, N + f[i + 1]!, N + f[i]!]);
    }
    for (let i = 0; i < f.length; i++) {
      const [a, b] = [f[i]!, f[(i + 1) % f.length]!];
      const key = a < b ? `${a},${b}` : `${b},${a}`;
      edges.set(key, (edges.get(key) ?? 0) + 1);
    }
  }
  for (const f of s.faces) for (let i = 0; i < f.length; i++) {
    const [a, b] = [f[i]!, f[(i + 1) % f.length]!];
    if (edges.get(a < b ? `${a},${b}` : `${b},${a}`) !== 1) continue;
    tris.push([b, a, N + a]);
    tris.push([b, N + a, N + b]);
  }
  let vol = 0;
  for (const [a, b, c] of tris) vol += dot(verts[a]!, cross(verts[b]!, verts[c]!));
  if (vol < 0) for (const t of tris) [t[1], t[2]] = [t[2], t[1]];
  return { verts, tris };
}

/** 🔲️ A grid sheet: `rows` × `cols` vertices given by a function of (i, j); quads between neighbours. */
export function gridSheet(rows: number, cols: number, at: (i: number, j: number) => { p: V3; n: V3 }): { p: V3[]; n: V3[]; faces: number[][] } {
  const p: V3[] = [], n: V3[] = [];
  for (let i = 0; i < rows; i++) for (let j = 0; j < cols; j++) {
    const q = at(i, j);
    p.push(q.p);
    n.push(q.n);
  }
  const faces: number[][] = [];
  for (let i = 0; i + 1 < rows; i++) for (let j = 0; j + 1 < cols; j++) faces.push([i * cols + j, i * cols + j + 1, (i + 1) * cols + j + 1, (i + 1) * cols + j]);
  return { p, n, faces };
}

/** 🧩️ Appends sheet `b` to sheet `a`, welding the vertices `weld` of b (pairs [index in b, index in a]). */
export function weldSheet(a: { p: V3[]; n: V3[]; faces: number[][] }, b: { p: V3[]; n: V3[]; faces: number[][] }, weld: [number, number][]): void {
  const map = new Map<number, number>(weld);
  const idx: number[] = [];
  for (let i = 0; i < b.p.length; i++) {
    if (map.has(i)) idx.push(map.get(i)!);
    else {
      idx.push(a.p.length);
      a.p.push(b.p[i]!);
      a.n.push(b.n[i]!);
    }
  }
  for (const f of b.faces) a.faces.push(f.map((i) => idx[i]!));
}

/** 🥢️ A capsule in world space. */
export type Rod = { id: string; bone: string; p0: V3; p1: V3; r0: number; r1: number; fields: Json };

/** 🖊️ Texture or detail strokes on a leaf bone: lines in the bone's (u, w) plane, with their v heights for the span of the extruded 2D solid. */
export type Stroke = { id: string; bone: string; lines: { a: V3; b: V3 }[]; role: "texture" | "detail" };

/** 🧺️ Every part of a plant at one scale. */
export type Parts = {
  tubes: Tube[]; rods: Rod[]; blobs: Blob[]; sheets: Sheet[]; repeats: Json[]; strokes: Stroke[];
  regions: Record<string, string>;
  groups: Record<string, { bone: string; region?: string; role: string; tone: number; material?: string; parts: string[] }>;
  members: Tube[];
  emitters: { id: string; anchor: string; bone: string; tube: Tube; anchorP: V3 }[];
  worldTexture: { id: string; bone: string; role: "texture" | "detail"; polys: (view: View) => { lines: P2[][]; depth: number[] } }[];
};
export const emptyParts = (): Parts => ({ tubes: [], rods: [], blobs: [], sheets: [], repeats: [], strokes: [], regions: {}, groups: {}, members: [], emitters: [], worldTexture: [] });

/** 🌳️ Everything a species file states: identity and texts, rig, crown, poses, physics, the parts of every scale and the decisions. */
export type Species = {
  id: string; title: Text; summary: Text; tags: string[]; references: { id: string; citation: string }[]; notes: Text; created: string;
  materials: Json[];
  stems: Bone[];
  leaves: Leaf[];
  leafBone: (bone: string) => boolean;
  bands: (bone: string, scale: Scale) => number;
  thermal: Json;
  structureAssumptions: Text[];
  poses: { id: string; title: Text; description: Text; stem: Record<string, Json>; resp: Response; thermal: Json; loads: Json[] }[];
  clips: (ctx: { offsets: (stem: Record<string, Json>, resp: Response) => Json; pose: (id: string) => Json }) => Json[];
  parts: (scale: Scale, F: Record<string, Frame>) => Parts;
  anchors: (F: Record<string, Frame>, parts: Parts) => Json[];
  volumeDecisions: Record<Scale, Text[]>;
  repDecisions: Record<Scale, Json>;
  pens: Record<Scale, Json>;
  symbolTitle: Text;
  massCheckKg?: number;
  defaultScale: number;
};

/** 🔁️ The world-space copies of a tube that carries a rotational repeat about an axis of its bone. */
function copies(t: Tube, F: Record<string, Frame>): Tube[] {
  const rep = t.fields.repeat;
  if (rep === undefined) return [t];
  const f = F[t.bone]!;
  const axis = add(add(mul(f.u, rep.rotationAxis[0]), mul(f.v, rep.rotationAxis[1])), mul(f.w, rep.rotationAxis[2]));
  const o = add(f.o, add(add(mul(f.u, rep.origin?.[0] ?? 0), mul(f.v, rep.origin?.[1] ?? 0)), mul(f.w, rep.origin?.[2] ?? 0)));
  return Array.from({ length: rep.count }, (_, k) => ({ ...t, sections: t.sections.map((s) => ({ ...s, p: add(o, rotate(sub(s.p, o), axis, rep.rotationRad * k)), x: rotate(s.x, axis, rep.rotationRad * k), y: rotate(s.y, axis, rep.rotationRad * k) })) }));
}

const toLocal3 = (F: Frame, p: V3): V3 => {
  const d = sub(p, F.o);
  return [dot(d, F.u), dot(d, F.v), dot(d, F.w)];
};

const capsuleRing = (view: View, p0: V3, p1: V3, r0: number, r1: number): P2[] => {
  const ring = (c: V3, r: number) => Array.from({ length: 16 }, (_, i) => {
    const a = (2 * Math.PI * i) / 16 + 0.07;
    const q = project(view, c);
    return { x: q.x + r * Math.cos(a), y: q.y + r * Math.sin(a) };
  });
  return hullOf([...ring(p0, r0), ...ring(p1, r1)]);
};

/** 🔷️ The ring of one tube section in world space (a superellipse with `n` points). */
function sectionPoints(s: Section, n = 14): V3[] {
  const e = s.n ?? 2;
  return Array.from({ length: n }, (_, i) => {
    const t = (2 * Math.PI * i) / n + 0.11;
    const [c, d] = [Math.cos(t), Math.sin(t)];
    const k = Math.pow(Math.pow(Math.abs(c), e) + Math.pow(Math.abs(d), e), -1 / e);
    return add(s.p, add(mul(s.x, s.a * c * k), mul(s.y, s.b * d * k)));
  });
}

const ringArea = (r: P2[]): number => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2;

/** ⛓️ The boundary loops of a set of faces of a planar mesh (edges used once), as vertex index loops. */
function boundaryLoops(faces: number[][]): number[][] {
  const count = new Map<string, number>();
  const directed: [number, number][] = [];
  for (const f of faces) for (let i = 0; i < f.length; i++) {
    const [a, b] = [f[i]!, f[(i + 1) % f.length]!];
    const key = a < b ? `${a},${b}` : `${b},${a}`;
    count.set(key, (count.get(key) ?? 0) + 1);
    directed.push([a, b]);
  }
  const next = new Map<number, number[]>();
  for (const [a, b] of directed) if (count.get(a < b ? `${a},${b}` : `${b},${a}`) === 1) next.set(a, [...(next.get(a) ?? []), b]);
  const loops: number[][] = [];
  const used = new Set<string>();
  for (const [start, outs] of next) for (const first of outs) {
    if (used.has(`${start},${first}`)) continue;
    const loop = [start];
    let [a, b] = [start, first];
    used.add(`${a},${b}`);
    while (b !== start && loop.length < 100000) {
      loop.push(b);
      const cand = (next.get(b) ?? []).filter((c) => !used.has(`${b},${c}`));
      if (cand.length === 0) break;
      [a, b] = [b, cand[0]!];
      used.add(`${a},${b}`);
    }
    if (loop.length >= 3) loops.push(loop);
  }
  return loops;
}

/** 🍃️ A plate of a leaf bone: its (u, w) ring and the v span at the bone origin and at the bone tip (the span runs straight between them). The plate is the body solid of that part of the leaf and its drawing at once, so the three views are the true views of the body by construction. */
export type Plate = { id: string; bone: string; kind: "blade" | "rib"; ring: P2[]; z0: number; z1: number; zTip: [number, number]; centre: V3 };

/** 🍃️ The plates of one leaf bone: the bone is cut into `bands` along u (fewer where a band would be shorter than 1.3 times the least plate size); in each band the blade faces, cut at the band edges, are one plate per connected outline and the tube pieces (petiole, midrib, stipe, rachis) another, each with the straight v span fitted to the faces whose centre lies in the band. */
function platesOf(bone: string, P: Parts, F: Frame, L: number, bands: number, scale: Scale, minThick: number, dropped: string[]): Plate[] {
  const feature = featureOf(scale);
  const least = minThick;
  type Item = { u: number; poly: P2[]; vs: number[]; us: number[] };
  const blades: Item[] = [], ribs: Item[] = [];
  for (const s of P.sheets.filter((x) => x.bone === bone)) {
    const loc = s.p.map((p) => toLocal3(F, p));
    const hs = s.p.map((_, i) => (Array.isArray(s.half) ? s.half[i]! : s.half));
    for (const [fi, f] of s.faces.entries()) {
      const pts = f.map((i) => loc[i]!);
      const cu = s.faceU?.[fi] ?? pts.reduce((a, p) => a + p[0], 0) / pts.length;
      const poly = pts.map((p) => ({ x: p[0], y: p[2] }));
      const vs = f.flatMap((i) => [loc[i]![1] + hs[i]! * Math.abs(dot(s.n[i]!, F.v)), loc[i]![1] - hs[i]! * Math.abs(dot(s.n[i]!, F.v))]);
      blades.push({ u: cu, poly, vs, us: f.flatMap((i) => [loc[i]![0], loc[i]![0]]) });
    }
  }
  for (const t of P.tubes.filter((x) => x.bone === bone)) {
    const rings = t.sections.map((s) => sectionPoints(s).map((p) => toLocal3(F, p)));
    for (let i = 0; i + 1 < rings.length; i++) {
      const pts = [...rings[i]!, ...rings[i + 1]!];
      const cu = pts.reduce((a, p) => a + p[0], 0) / pts.length;
      ribs.push({ u: cu, poly: hullOf(pts.map((p) => ({ x: p[0], y: p[2] }))), vs: pts.map((p) => p[1]), us: pts.map((p) => p[0]) });
    }
  }
  const all = [...blades, ...ribs];
  if (all.length === 0) return [];
  const [umin, umax] = [Math.min(...all.map((x) => x.u)), Math.max(...all.map((x) => x.u))];
  const count = Math.max(1, Math.min(bands, Math.floor((umax - umin) / (least * 1.3))));
  const edges = steps(umin - 1e-6, umax + 1e-6, (umax - umin + 2e-6) / count);
  const plates: Plate[] = [];
  for (const [kind, items] of [["blade", blades], ["rib", ribs]] as const) for (let b = 0; b + 1 < edges.length; b++) {
    const centred = items.filter((x) => x.u >= edges[b]! && x.u < edges[b + 1]! + (b + 2 === edges.length ? 1e-6 : 0));

    const [from, to] = [b === 0 ? -Infinity : edges[b]!, b + 2 === edges.length ? Infinity : edges[b + 1]!];
    const within = (ring: P2[]) => (to === Infinity ? ring : clipRing(ring, to, false));
    const polys = items.map((x) => (ringArea(x.poly) < 0 ? [...x.poly].reverse() : x.poly)).map((x) => within(from === -Infinity ? x : clipRing(x, from, true))).filter((x) => x.length >= 3 && Math.abs(ringArea(x)) > 1e-10);
    const [from0, to0] = [edges[b]!, edges[b + 1]!];
    const inBand = centred.length ? centred : items.filter((x) => Math.max(...x.us) > from0 && Math.min(...x.us) < to0);
    if (inBand.length === 0 || polys.length === 0) continue;
    const us = inBand.flatMap((x) => x.us), vs = inBand.flatMap((x) => x.vs);
    const n = us.length;
    const mu = us.reduce((a, x) => a + x, 0) / n, mv = vs.reduce((a, x) => a + x, 0) / n;
    const sxx = us.reduce((a, x) => a + (x - mu) ** 2, 0), sxy = us.reduce((a, x, i) => a + (x - mu) * (vs[i]! - mv), 0);
    const m = sxx > 1e-9 ? sxy / sxx : 0;
    const c = mv - m * mu;
    let lo = Math.min(...vs.map((v, i) => v - (m * us[i]! + c)));
    let hi = Math.max(...vs.map((v, i) => v - (m * us[i]! + c)));
    if (hi - lo < minThick) {
      const mid = (hi + lo) / 2;
      [lo, hi] = [mid - minThick / 2, mid + minThick / 2];
    }
    const pieces: { ring: P2[]; tag: string; z: [number, number, number, number] }[] = [];
    const at = (u: number) => m * u + c;
    const ringsOf = (cut: (ring: P2[]) => P2[]) => outline(polys.map(cut).filter((x) => x.length >= 3 && Math.abs(ringArea(x)) > 1e-10), feature / 4, (feature * feature) / 16).filter((r) => ringArea(r) > 0).map((r) => (kind === "rib" ? widened(r, least) : r)).map(cleanRing).filter((r) => {
      const keep = r.length >= 3 && narrowest(r) >= least * 0.98;
      if (!keep && r.length >= 3) dropped.push(`${bone}-${kind}-${b + 1} ${(narrowest(r) * 1000).toFixed(0)} mm`);
      return keep;
    });
    const [u0, u1] = [Math.min(...polys.flat().map((q) => q.x)), Math.max(...polys.flat().map((q) => q.x))];
    if (u0 < -least * 1.02) for (const ring of ringsOf((x) => clipRing(x, 0, false))) { const e = Math.min(...ring.map((q) => q.x)); pieces.push({ ring, tag: "p", z: [Math.min(at(e), at(0)) + lo, Math.max(at(e), at(0)) + hi, Math.min(at(e), at(0)) + lo, Math.max(at(e), at(0)) + hi] }); }
    for (const ring of ringsOf((x) => clipRing(clipRing(x, 0, true), L, false))) pieces.push({ ring, tag: "", z: [at(0) + lo, at(0) + hi, at(L) + lo, at(L) + hi] });
    if (u1 > L + least * 1.02) for (const ring of ringsOf((x) => clipRing(x, L, true))) { const e = Math.max(...ring.map((q) => q.x)); pieces.push({ ring, tag: "q", z: [Math.min(at(e), at(L)) + lo, Math.max(at(e), at(L)) + hi, Math.min(at(e), at(L)) + lo, Math.max(at(e), at(L)) + hi] }); }
    pieces.forEach((piece, k) => {
      const cu = piece.ring.reduce((sum, q) => sum + q.x, 0) / piece.ring.length;
      const centre = add(F.o, add(mul(F.u, cu), mul(F.v, m * cu + c)));
      plates.push({ id: `${bone}-${kind}-${b + 1}${piece.tag}${pieces.filter((x) => x.tag === piece.tag).length > 1 ? `-${pieces.filter((x, j) => x.tag === piece.tag && j <= k).length}` : ""}`, bone, kind, ring: piece.ring, z0: r5(piece.z[0]), z1: r5(piece.z[1]), zTip: [r5(piece.z[2]), r5(piece.z[3])], centre });
    });
  }
  return plates;
}

/** 📏️ The narrowest width of a ring over the directions of its own edges (a plate thinner than the smallest feature is no solid of the scale and is left out). */
function narrowest(ring: P2[]): number {
  let best = Infinity;
  ring.forEach((a, i) => {
    const b = ring[(i + 1) % ring.length]!;
    const l = Math.hypot(b.x - a.x, b.y - a.y);
    if (l < 1e-9) return;
    const d = ring.map((q) => ((q.x - a.x) * (b.y - a.y) - (q.y - a.y) * (b.x - a.x)) / l);
    best = Math.min(best, Math.max(...d) - Math.min(...d));
  });
  return best;
}

/** ✂️ The part of a ring on one side of the line u = `at` (Sutherland-Hodgman; `above` keeps u ≥ at). The span of a drawn plate is straight only between the bone origin and the bone tip and constant beyond them, so a plate is cut there and every piece has one flat upper and lower face (an end beyond the origin or the tip that is shorter than the smallest feature is left out). */
function clipRing(ring: P2[], at: number, above: boolean): P2[] {
  const inside = (q: P2) => (above ? q.x >= at : q.x <= at);
  const out: P2[] = [];
  ring.forEach((q, i) => {
    const prev = ring[(i + ring.length - 1) % ring.length]!;
    if (inside(q) !== inside(prev)) { const k = (at - prev.x) / (q.x - prev.x); out.push({ x: at, y: prev.y + (q.y - prev.y) * k }); }
    if (inside(q)) out.push(q);
  });
  return out;
}

/** 🧹️ A ring on the grid of the shipped numbers without repeated points and without points on the straight line of their neighbours (they would give triangles without area). */
function cleanRing(ring: P2[]): P2[] {
  let out = ring.map((p) => ({ x: r5(p.x), y: r5(p.y) })).filter((p, i, all) => { const q = all[(i + all.length - 1) % all.length]!; return Math.hypot(p.x - q.x, p.y - q.y) > 2e-5; });
  for (let changed = true; changed && out.length > 3; ) {
    changed = false;
    for (let i = 0; i < out.length && out.length > 3; i++) {
      const [a, b, c] = [out[(i + out.length - 1) % out.length]!, out[i]!, out[(i + 1) % out.length]!];
      if (Math.abs((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)) < 1e-9 * Math.max(1, Math.hypot(c.x - a.x, c.y - a.y) * 1e3)) { out = out.filter((_, k) => k !== i); changed = true; i--; }
    }
  }
  return out;
}

/** 🧱️ The body solid of a plate: the closed prism over its ring between the straight lower and upper v spans (the same clamp at the bone origin and tip as the drawn solid), triangulated by the kernel; `null` when the kernel refuses the result. */
function plateSolid(pl: Plate, L: number, fields: Json): Json | null {
  const n = pl.ring.length;
  const span = (u: number): [number, number] => { const t = L > 0 ? Math.min(Math.max(u / L, 0), 1) : 0; return [pl.z0 + (pl.zTip[0] - pl.z0) * t, pl.z1 + (pl.zTip[1] - pl.z1) * t]; };
  const vertices = [...pl.ring.map((p) => [p.x, r5(span(p.x)[0]), p.y]), ...pl.ring.map((p) => [p.x, r5(span(p.x)[1]), p.y])];
  let flat: number[];
  try { flat = triangulatePolygon(pl.ring.map((p) => [p.x, p.y] as [number, number]), []); } catch { return null; }
  const triangles: [number, number, number][] = [];
  for (let i = 0; i + 2 < flat.length; i += 3) {
    const [a, b, c] = [flat[i]!, flat[i + 1]!, flat[i + 2]!];
    triangles.push([a, b, c], [n + a, n + c, n + b]);
  }
  for (let i = 0; i < n; i++) {
    const j = (i + 1) % n;
    triangles.push([j, i, n + i], [j, n + i, n + j]);
  }
  let volume = 0;
  for (const [a, b, c] of triangles) volume += dot(vertices[a] as V3, cross(vertices[b] as V3, vertices[c] as V3));
  if (volume < 0) for (const tri of triangles) [tri[1], tri[2]] = [tri[2], tri[1]];
  if (polyhedronFault(vertices, triangles) !== null) return null;
  return { id: pl.id, kind: "polyhedron", bone: pl.bone, ...fields, vertices, triangles };
}

/** 🍃️ The plates of every leaf bone at a scale. No plate is thinner or narrower than `least`: a little above the smallest feature, and at 1:500 the 0.24 mm of paper that a filled shape safely needs to print in every pose (a plate seen edge-on shows its thickness). */
export function leafPlates(sp: Species, rig: Json, scale: Scale, P: Parts, F: Record<string, Frame>, say: string[]): Plate[] {
  const feature = featureOf(scale);
  const least = scale === 500 ? 0.12 : Math.max(feature * 1.04, scale === 50 ? 0.008 : 0);
  const dropped: string[] = [];
  const plates = rig.bones.filter((b: Json) => sp.leafBone(b.id)).flatMap((b: Json) => platesOf(b.id, P, F[b.id]!, b.lengthM, sp.bands(b.id, scale), scale, least, dropped));
  if (dropped.length) say.push(`[DEBUG] 1:${scale} plates below ${(least * 1000).toFixed(0)} mm left out (${dropped.length}): ${dropped.slice(0, 14).join(", ")}`);
  return plates;
}

/** ↔️ A rib ring whose points keep at least half of `least` from its own middle line across the bone (a midrib thinner than the scale can show is drawn at the least width, as a slender bar is). */
function widened(ring: P2[], least: number): P2[] {
  const [lo, hi] = [Math.min(...ring.map((q) => q.y)), Math.max(...ring.map((q) => q.y))];
  const mid = (lo + hi) / 2;
  return ring.map((q) => (Math.abs(q.y - mid) < 1e-6 ? q : { x: q.x, y: mid + Math.sign(q.y - mid) * Math.max(Math.abs(q.y - mid), least * 0.52) }));
}

/** 🧊️ Body, thermal and structure lists of one scale. */
function volumesOf(sp: Species, rig: Json, scale: Scale, P: Parts, F: Record<string, Frame>, plates: Plate[], say: string[]): Json {
  const feature = featureOf(scale);
  const body: Json[] = [];
  const plated = new Set<string>();
  for (const t of P.tubes) if (sp.leafBone(t.bone)) plated.add(t.id); else body.push(loftOf(t, F[t.bone]!));
  for (const r of P.rods) body.push({ id: r.id, kind: "capsule", bone: r.bone, ...r.fields, p0: localPoint(F[r.bone]!, r.p0), p1: localPoint(F[r.bone]!, r.p1), r0: r5(r.r0), r1: r5(r.r1) });
  for (const b of P.blobs) body.push(ellipsoidOf(b, F[b.bone]!));
  for (const s of P.sheets) {
    if (sp.leafBone(s.bone)) { plated.add(s.id); continue; }
    const { verts, tris } = thicken(s);
    body.push({ id: s.id, kind: "polyhedron", bone: s.bone, ...s.fields, vertices: verts.map((p) => localPoint(F[s.bone]!, p)), triangles: tris });
  }
  body.push(...P.repeats);
  const thermal: Json[] = Object.entries(P.regions).filter(([id]) => !plated.has(id)).map(([id, region]) => ({ id: `th-${id}`, of: id, region }));
  const refused: string[] = [];
  for (const pl of plates) {
    const dead = leafState(sp, pl.bone) === "dead";
    const solid = plateSolid(pl, rig.bones.find((b: Json) => b.id === pl.bone).lengthM, { material: dead ? sp.thermal.deadMaterial : sp.thermal.leafMaterial, role: "body", tone: dead ? 0.34 : 0.52 });
    if (solid === null) { refused.push(pl.id); continue; }
    body.push(solid);
    thermal.push({ id: `th-${pl.id}`, of: pl.id, region: dead ? sp.thermal.deadRegion : sp.thermal.leafRegion });
  }
  if (refused.length) say.push(`[DEBUG] 1:${scale} plates refused by the kernel (${refused.length}): ${refused.slice(0, 12).join(", ")}`);
  for (const id of refused) plates.splice(plates.findIndex((x) => x.id === id), 1);
  for (const e of P.emitters) thermal.push({ ...loftOf(e.tube, F[e.bone]!), region: sp.thermal.root.emitters[0].region, tags: [sp.thermal.root.emitters[0].id] });
  const structure: Json[] = [];
  for (const t of P.members) structure.push({ ...loftOf(t, F[t.bone]!), member: t.bone, part: "member", ...(t.fields.slender ? { slender: true } : {}) });
  const joint = (id: string, bone: string, r: number) => structure.push({ id, kind: "ellipsoid", bone, material: t0(sp), role: "detail", tone: 0.5, member: bone, part: "joint", radii: [r, r, r].map(r5), transform: { translation: [0, 0, 0] } });
  for (const b of sp.stems) if (b.parent !== null && b.jointRadius) joint(`j-${b.id}`, b.id, Math.max(b.jointRadius, feature * 0.6));
  if (scale !== 500) for (const f of sp.leaves) {
    joint(`j-${f.id}a`, `${f.id}a`, Math.max(sp.thermal.leafJointR ?? 0.03, feature * 0.6));
    joint(`j-${f.id}b`, `${f.id}b`, Math.max((sp.thermal.leafJointR ?? 0.03) * 0.6, feature * 0.55));
  }
  const support = rig.bones[0];
  const t = Math.max(feature, 0.0075);
  structure.push({ id: "s-base-support", kind: "revolve", bone: support.id, material: t0(sp), role: "ground", tone: 0.5, member: support.id, part: "support", anchor: "base-support", axis: [1, 0, 0], origin: [0, 0, 0], profile: [[-r5(t), 0], [-r5(t), sp.thermal.supportR ?? 0.5], [0, sp.thermal.supportR ?? 0.5], [0, 0]] });
  for (const b of sp.stems) if (b.massMarker) {
    const r = Math.max(b.massMarker, feature * 0.6);
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: t0(sp), role: "detail", tone: 0.5, member: b.id, part: "mass", radii: [r, r, r].map(r5), transform: { translation: [r5(b.mass.comX), r5(b.mass.comY), 0] } });
  }
  return { body, thermal, structure };
}
const t0 = (sp: Species): string => sp.materials.find((m: Json) => m.structural)?.id ?? sp.materials[0].id;
/** 🍂️ The state of the leaf a leaf bone belongs to. */
const leafState = (sp: Species, bone: string): string | undefined => sp.leaves.find((f) => bone.startsWith(`${f.id}`) && bone.length === f.id.length + 1)?.state;

/** ⭕️ The view polygons of a thick tube with 72 points per section: the 20 points of the kit leave the outline of a 0.5 m trunk 3 mm inside its solid, which closes the slots beside it in the drawing. */
function roundPolygons(view: View, tube: Tube): P2[][] {
  const rings = tube.sections.map((s) => sectionPoints(s, 72).map((q) => project(view, q)));
  return rings.length === 1 ? [hullOf(rings[0]!)] : rings.slice(1).map((ring, i) => hullOf([...rings[i]!, ...ring]));
}

/** ✏️ Affine maps of the paper of a view into the frame of each shape, solved from three probe points placed by the product. */
function affineFor(cand: Json, scale: number, view: View, metas: Json[]): ((p: P2) => P2)[] {
  const placed = forward(cand, scale, view, metas.map((meta) => ({ meta, points: [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 1 }] })));
  return placed.map(([o, a, b]) => {
    const [m00, m10, m01, m11] = [a!.x - o!.x, a!.y - o!.y, b!.x - o!.x, b!.y - o!.y];
    const det = m00 * m11 - m01 * m10;
    return (p: P2) => {
      const [dx, dy] = [p.x - o!.x, p.y - o!.y];
      return { x: (m11 * dx - m01 * dy) / det, y: (-m10 * dx + m00 * dy) / det };
    };
  });
}

/** 🖊️ All shapes of one scale and view: flat outlines of the stem parts (true projections mapped into their bone frames) and extruded plates and strokes on the leaf bones. */
function drawView(sp: Species, cand: Json, scale: Scale, view: View, P: Parts, F: Record<string, Frame>, rig: Json, plates: Plate[]): Json[] {
  const feature = featureOf(scale);
  const tol = feature / 3;
  type Pending = { meta: Json; rings: P2[][]; closed: boolean; depth: number; kind: "outline" | "lines" };
  const pending: Pending[] = [];
  const polysOf = (ids: string[]): P2[][] => ids.flatMap((id) => {
    const t = P.tubes.find((x) => x.id === id);
    if (t) return copies(t, F).flatMap((c) => (Math.max(...c.sections.map((s) => Math.max(s.a, s.b))) > 0.06 ? roundPolygons(view, c) : tubePolygons(view, c)));
    const r = P.rods.find((x) => x.id === id);
    if (r) return [capsuleRing(view, r.p0, r.p1, r.r0, r.r1)];
    const b = P.blobs.find((x) => x.id === id);
    if (b) return [blobOutline(view, b)];
    throw new Error(`no part ${id}`);
  });
  const centreOf = (id: string): V3 => {
    const t = P.tubes.find((x) => x.id === id);
    if (t) return t.sections[Math.floor(t.sections.length / 2)]!.p;
    const r = P.rods.find((x) => x.id === id);
    if (r) return mul(add(r.p0, r.p1), 0.5);
    return P.blobs.find((x) => x.id === id)!.c;
  };
  for (const [id, g] of Object.entries(P.groups)) {
    if (g.parts.length === 0) continue;
    const outer = outline(polysOf(g.parts), tol, feature * feature).filter((r) => ringArea(r) > 0);
    if (outer.length === 0) continue;
    const depth = g.parts.reduce((s, pid) => s + nearness(view, centreOf(pid)), 0) / g.parts.length;
    pending.push({ meta: { id, bone: g.bone, role: g.role, filled: true, tone: g.tone, ...(g.region ? { region: g.region } : {}), ...(g.material ? { material: g.material } : {}) }, rings: outer, closed: true, depth, kind: "outline" });
  }
  if (view !== "plan") for (const tex of P.worldTexture) {
    const { lines, depth } = tex.polys(view);
    if (lines.length === 0) continue;
    pending.push({ meta: { id: tex.id, bone: tex.bone, role: tex.role, filled: false, tone: 0.2 }, rings: lines, closed: false, depth: depth.length ? depth.reduce((a, b) => a + b, 0) / depth.length : 0, kind: "lines" });
  }
  const depths: number[] = [...pending.map((p) => p.depth), ...plates.map((p) => nearness(view, p.centre))];
  const [dmin, dmax] = [Math.min(...depths), Math.max(...depths)];
  const zOf = (d: number) => r4(1 + (4 * (d - dmin)) / Math.max(1e-6, dmax - dmin));
  const ownerZ = (bone: string) => {
    const own = pending.find((p) => p.kind === "outline" && p.meta.bone === bone);
    return own ? zOf(own.depth) : 2;
  };
  const metas = pending.map((p) => ({ ...p.meta, z: p.kind === "outline" ? zOf(p.depth) : r4(ownerZ(p.meta.bone) + 0.003) }));
  const maps = metas.length ? affineFor(cand, scale, view, metas) : [];
  const shapes: Json[] = pending.map((p, i) => {
    const local = p.rings.map((r) => r.map(maps[i]!));
    const meta = metas[i]!;
    if (p.kind === "outline" && local.length === 1) return { ...meta, kind: "poly", points: pointsOf(local[0]!), closed: true };
    return { ...meta, kind: "path", d: pathOf(local, p.closed) };
  });
  const leafRegion = (bone: string) => (leafState(sp, bone) === "dead" ? sp.thermal.deadRegion : sp.thermal.leafRegion);
  const leafMaterial = (bone: string) => (leafState(sp, bone) === "dead" ? sp.thermal.deadMaterial : sp.thermal.leafMaterial);
  for (const pl of plates) shapes.push({ id: pl.id, bone: pl.bone, role: "body", z: zOf(nearness(view, pl.centre)), filled: true, tone: leafRegion(pl.bone) === sp.thermal.deadRegion ? 0.34 : 0.52, region: leafRegion(pl.bone), material: leafMaterial(pl.bone), kind: "poly", points: pointsOf(pl.ring), closed: true, solid: { kind: "extruded", sweep: "v", z0: pl.z0, z1: pl.z1, zTip: [pl.zTip[0], pl.zTip[1]] } });
  for (const st of P.strokes) {
    const Fb = F[st.bone]!;
    const L = rig.bones.find((b: Json) => b.id === st.bone).lengthM;
    const loc = st.lines.map((l) => [toLocal3(Fb, l.a), toLocal3(Fb, l.b)] as const);
    const us = loc.flatMap(([a, b]) => [a[0], b[0]]), vs = loc.flatMap(([a, b]) => [a[1], b[1]]);
    const n = us.length;
    const mu = us.reduce((a, x) => a + x, 0) / n, mv = vs.reduce((a, x) => a + x, 0) / n;
    const sxx = us.reduce((a, x) => a + (x - mu) ** 2, 0), sxy = us.reduce((a, x, i) => a + (x - mu) * (vs[i]! - mv), 0);
    const m = sxx > 1e-9 ? sxy / sxx : 0;
    const c = mv - m * mu;
    const own = plates.filter((x) => x.bone === st.bone);
    const z = own.length ? Math.max(...own.map((x) => zOf(nearness(view, x.centre)))) + 0.003 : 4;
    shapes.push({ id: st.id, bone: st.bone, role: st.role, z: r4(z), filled: false, tone: 0.25, kind: "path", d: pathOf(loc.map(([a, b]) => [{ x: a[0], y: a[2] }, { x: b[0], y: b[2] }]), false), solid: { kind: "extruded", sweep: "v", z0: r5(c), z1: r5(c + 0.002), zTip: [r5(c + m * L), r5(c + m * L + 0.002)] } });
  }
  return shapes;
}

/** 📐️ The extent of the 1:50 body in world space in the default pose: tubes, rods, blobs and sheets. */
export function extentOf(sp: Species, rig: Json, P: Parts, F: Record<string, Frame>, plates: Plate[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  const take = (p: V3) => { for (const k of [0, 1, 2]) { min[k] = Math.min(min[k]!, p[k]!); max[k] = Math.max(max[k]!, p[k]!); } };
  for (const t of P.tubes) if (!sp.leafBone(t.bone)) for (const c of copies(t, F)) for (const s of c.sections) for (const q of sectionPoints(s, 24)) take(q);
  for (const r of P.rods) for (const [p, rr] of [[r.p0, r.r0], [r.p1, r.r1]] as [V3, number][]) for (const d of [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]] as V3[]) take(add(p, mul(d, rr)));
  for (const b of P.blobs) for (const ax of b.axes.keys()) for (const sg of [-1, 1]) take(add(b.c, mul(b.axes[ax]!, sg * b.radii[ax]!)));
  for (const s of P.sheets) if (!sp.leafBone(s.bone)) { const { verts } = thicken(s); verts.forEach(take); }
  for (const pl of plates) {
    const f = F[pl.bone]!, L = rig.bones.find((b: Json) => b.id === pl.bone).lengthM;
    for (const p of pl.ring) { const k = L > 0 ? Math.min(Math.max(p.x / L, 0), 1) : 0; for (const v of [pl.z0 + (pl.zTip[0] - pl.z0) * k, pl.z1 + (pl.zTip[1] - pl.z1) * k]) take(add(f.o, add(mul(f.u, p.x), add(mul(f.v, v), mul(f.w, p.y))))); }
  }
  return { min, max };
}

/** 🌿️ The plan symbol: the true crown extent as a star through the leaf tips and the stem section, in model metres in all three views. */
function symbolOf(sp: Species, P: Parts, F: Record<string, Frame>, ext: { min: V3; max: V3 }, stemR: number, stemTop: number): Json {
  const tips: V3[] = [];
  for (const s of P.sheets) for (const p of s.p) tips.push(p);
  const geometry: Json = {};
  for (const view of VIEWS) {
    const pts = tips.map((p) => project(view, p));
    const hull = hullOf(pts);
    const c = view === "plan" ? { x: 0, y: 0 } : { x: hull.reduce((a, p) => a + p.x, 0) / hull.length, y: hull.reduce((a, p) => a + p.y, 0) / hull.length };
    const star: P2[] = [];
    const n = Math.min(32, hull.length);
    const pick = Array.from({ length: n }, (_, i) => hull[Math.floor((i * hull.length) / n)]!);
    pick.forEach((p, i) => {
      star.push(p);
      const q = pick[(i + 1) % n]!;
      const m = { x: (p.x + q.x) / 2, y: (p.y + q.y) / 2 };
      star.push({ x: c.x + (m.x - c.x) * 0.72, y: c.y + (m.y - c.y) * 0.72 });
    });
    const stem = view === "plan" ? Array.from({ length: 24 }, (_, i) => ({ x: stemR * Math.cos((i * Math.PI) / 12), y: stemR * Math.sin((i * Math.PI) / 12) })) : [{ x: -stemR * 1.3, y: 0 }, { x: -stemR, y: 0.3 }, { x: -stemR * 0.8, y: stemTop }, { x: stemR * 0.8, y: stemTop }, { x: stemR, y: 0.3 }, { x: stemR * 1.3, y: 0 }];
    geometry[view] = [
      { id: "crown", bone: sp.stems[0].id, role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: pointsOf(star), closed: true },
      { id: "stem", bone: sp.stems[0].id, role: "body", z: view === "plan" ? 4 : 3, filled: true, tone: view === "plan" ? 0.25 : 0.3, kind: "poly", points: pointsOf(stem), closed: true },
    ];
  }
  const [w, h, d] = [ext.max[0] - ext.min[0], ext.max[1], ext.max[2] - ext.min[2]].map((x) => Math.round(x * 100) / 100);
  const fmt = (x: number, lang: "en" | "de") => (lang === "en" ? String(x) : String(x).replace(".", ","));
  return {
    id: `${sp.id}-symbol`, title: sp.symbolTitle,
    description: text("A conventional plan sign of the plant for site and landscape plans: outline of the true crown extent and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der Pflanze für Lage- und Freiraumpläne: Umriss der wahren Kronenausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
    semantics: "true-extent", units: "model-m", denominators: [500, 200, 100],
    pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry,
    dimensionalNote: text(`A conventional simplification of the plant drawn at its true extent in model metres in all three views (${fmt(w, "en")} m wide, ${fmt(d, "en")} m deep and ${fmt(h, "en")} m high): the outline is the crown, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale.`, `Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten (${fmt(w, "de")} m breit, ${fmt(d, "de")} m tief und ${fmt(h, "de")} m hoch): Der Umriss ist die Krone, der Stamm ist in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.`),
  };
}

/** 🏗️ Builds the whole asset document of a species and writes it (candidate under `OUT/<id>/🔣️.json`, and the catalog file with `--ship`); returns a short report. */
export function buildDocument(sp: Species, opts: { ship: boolean; no2d?: boolean }): string[] {
  const say: string[] = [];
  const rig: Json = { kind: "botanical", root: sp.stems[0].id, bones: sp.stems.map(({ jointRadius, massMarker, ...b }: Json) => b) };
  rig.bones.push(...leafBones(rig, sp.leaves));
  const poses = sp.poses.map((p) => ({ id: p.id, title: p.title, description: p.description, bones: poseOffsets(rig, sp.leaves, p.stem, p.resp), ground: "authored", contacts: [], thermal: p.thermal, loads: p.loads }));
  const clips = sp.clips({ offsets: (stem, resp) => poseOffsets(rig, sp.leaves, stem, resp), pose: (id) => poses.find((p) => p.id === id) });
  const calm = poses[0]!;
  const F = framesOf(rig, calm.bones);
  const parts = Object.fromEntries(SCALES.map((s) => [s, sp.parts(s, F)])) as Record<Scale, Parts>;
  const anchors = sp.anchors(F, parts[50]);
  const plates = Object.fromEntries(SCALES.map((s) => [s, leafPlates(sp, rig, s, parts[s], F, say)])) as Record<Scale, Plate[]>;
  const volumes: Json = {};
  for (const s of SCALES) volumes[String(s)] = { status: "authored", denominator: s, ...volumesOf(sp, rig, s, parts[s], F, plates[s], say), decisions: sp.volumeDecisions[s] };
  const ext = extentOf(sp, rig, parts[50], F, plates[50]);
  const massKg = Math.round(rig.bones.reduce((s: number, b: Json) => s + b.mass.kg, 0) * 10) / 10;
  const dims = { heightM: r4(Math.ceil(ext.max[1] * 100) / 100), widthM: r4(Math.ceil((ext.max[0] - ext.min[0]) * 100) / 100), depthM: r4(Math.ceil((ext.max[2] - ext.min[2]) * 100) / 100), massKg };
  say.push(`extent 1:50 x ${ext.min[0].toFixed(3)}..${ext.max[0].toFixed(3)} y ${ext.min[1].toFixed(3)}..${ext.max[1].toFixed(3)} z ${ext.min[2].toFixed(3)}..${ext.max[2].toFixed(3)} -> dimensions ${JSON.stringify(dims)}`);
  const stemBase = sp.stems[1] ?? sp.stems[0];
  const representations: Json = {};
  const ranges: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
  for (const s of SCALES) representations[String(s)] = { status: "authored", denominator: s, validRange: { minDenominator: ranges[s][0], maxDenominator: ranges[s][1] }, pens: sp.pens[s], hatches: [], geometry: { elevation: [], front: [], plan: [] }, decisions: sp.repDecisions[s] };
  const asset: Json = {
    id: sp.id, category: "vegetation", rig, anchors, title: sp.title, summary: sp.summary, tags: sp.tags,
    provenance: { author: "Semio staffage vegetation content worker PW-10", method: "handcrafted", created: sp.created, references: sp.references, notes: sp.notes },
    dimensions: dims, materials: sp.materials.map(({ structural, ...m }: Json) => m), poses, clips,
    thermal: { ...sp.thermal.root },
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: sp.structureAssumptions, clinical: false },
    volumes, presentation: { defaultPose: calm.id, defaultClip: clips[0].id, defaultScale: sp.defaultScale },
    exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    completion: "authored", representations,
    symbols: [symbolOf(sp, parts[50], F, ext, Math.max(...parts[50].tubes.filter((t) => t.bone === stemBase.id).flatMap((t) => t.sections.map((x) => x.a))), sp.thermal.symbolStemTop)],
  };
  const box = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
  for (const m of meshesAt(asset, 50, volumes["50"].body.filter((s: Json) => s.texture !== true))) for (let i = 0; i < m.world.length; i++) { box.min[i % 3] = Math.min(box.min[i % 3]!, m.world[i]!); box.max[i % 3] = Math.max(box.max[i % 3]!, m.world[i]!); }
  asset.dimensions = { heightM: r4(Math.round((box.max[1]! - box.min[1]!) * 1000) / 1000), widthM: r4(Math.round((box.max[0]! - box.min[0]!) * 1000) / 1000), depthM: r4(Math.round((box.max[2]! - box.min[2]!) * 1000) / 1000), massKg };
  say.push(`tessellated 1:50 body x ${box.min[0]!.toFixed(3)}..${box.max[0]!.toFixed(3)} y ${box.min[1]!.toFixed(3)}..${box.max[1]!.toFixed(3)} z ${box.min[2]!.toFixed(3)}..${box.max[2]!.toFixed(3)} -> dimensions ${JSON.stringify(asset.dimensions)}`);
  if (!opts.no2d) for (const s of SCALES) for (const view of VIEWS) {
    asset.representations[String(s)].geometry[view] = drawView(sp, asset, s, view, parts[s], F, rig, plates[s]);
    say.push(`2D 1:${s} ${view}: ${asset.representations[String(s)].geometry[view].length} shapes`);
  }
  for (const s of SCALES) {
    const v = volumes[String(s)];
    const expanded = (l: Json[]) => l.reduce((n, x) => n + (x.repeat?.count ?? 1), 0);
    say.push(`1:${s}: body ${v.body.length} (${expanded(v.body)} expanded), thermal ${v.thermal.length}, structure ${v.structure.length}`);
  }
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const cand = join(OUT, sp.id, "🔣️.json");
  mkdirSync(dirname(cand), { recursive: true });
  writeFileSync(cand, `${JSON.stringify(doc)}\n`);
  say.push(`written ${cand}`);
  if (opts.ship) {
    const cat = join(STAFFAGE, "🗂️catalog", "🌳️vegetation", sp.id, "🔣️.json");
    mkdirSync(dirname(cat), { recursive: true });
    writeFileSync(cat, `${JSON.stringify(doc, null, 1)}\n`);
    say.push(`shipped ${cat}`);
  }
  return say;
}

/** 🌬️ EN 1991-1-4 drag F = ½ ρ c_d A v² with ρ = 1.25 kg/m³, rounded to 0.1 N. */
export const drag = (cd: number, areaM2: number, vMS: number): number => Math.round(0.5 * 1.25 * cd * areaM2 * vMS * vMS * 10) / 10;

/** 🌀️ Standard wind and transpiration clips of a rosette plant, from the species' stem leans and crown responses. */
export function standardClips(ctx: { offsets: (stem: Record<string, Json>, resp: Response) => Json; pose: (id: string) => Json }, c: {
  stems: string[]; windyLean: number[]; swayS: number; swayAmp: number; flutterDeg: number; windy: Response; storm: Response; leaves: Leaf[];
  emitter: string; regions: { id: string; peak: number }[]; texts: { sway: Text; gust: Text; diurnal: Text; swayTitle?: Text };
  loadTargets: string[];
}): Json[] {
  const scaleResp = (r: Response, k: number, extra?: (f: Leaf) => [number, number]): Response => ({ windward: [r.windward[0] * k, r.windward[1] * k], leeward: [r.leeward[0] * k, r.leeward[1] * k], stream: r.stream * k, roll: r.roll * k, ...(extra ? { flutter: extra } : {}) });
  const stemAt = (k: number) => Object.fromEntries(c.stems.map((id, i) => [id, { angleDeg: Math.round(c.windyLean[i]! * k * 100) / 100 }]));
  const n = 8;
  const swayKeys = Array.from({ length: n + 1 }, (_, i) => {
    const t = (c.swayS * i) / n;
    const ph = (2 * Math.PI * i) / n;
    const k = 1 + c.swayAmp * Math.sin(ph);
    const flutter = (f: Leaf): [number, number] => {
      const lag = f.k * 0.7;
      return [Math.round(c.flutterDeg * 0.4 * Math.sin(ph - lag) * 100) / 100, Math.round(c.flutterDeg * Math.sin(ph - lag - 0.5) * 100) / 100];
    };
    return { t: Math.round(t * 1000) / 1000, pose: "windy", ease: "smooth", bones: ctx.offsets(stemAt(k), scaleResp(c.windy, k, flutter)) };
  });
  const gustLevels: [number, number][] = [[0, 0.1], [1.2, 0.45], [2.5, 1.7], [3.6, 1.2], [5, 0.6], [8, 0.1]];
  const gustKeys = gustLevels.map(([t, g]) => {
    const toStorm = Math.max(0, Math.min(1, (g - 1) / 1)) * 0.7;
    const base = Math.min(1, g);
    const mix = (a: [number, number], b: [number, number]): [number, number] => [a[0] * base * (1 - toStorm) + b[0] * toStorm, a[1] * base * (1 - toStorm) + b[1] * toStorm];
    const resp: Response = { windward: mix(c.windy.windward, c.storm.windward), leeward: mix(c.windy.leeward, c.storm.leeward), stream: c.windy.stream * base * (1 - toStorm) + c.storm.stream * toStorm, roll: c.windy.roll * base * (1 - toStorm) + c.storm.roll * toStorm };
    return { t, pose: "windy", ease: "smooth", bones: ctx.offsets(stemAt(g), resp) };
  });
  const loadTrack = [[0, 0.05], [1.2, 0.45], [2.5, 2.75], [3.6, 1.4], [5, 0.5], [8, 0.05]].map(([t, v]) => ({ t, v }));
  const calmBones = ctx.pose("calm").bones;
  const sun = [[0, 0], [6, 0], [8, 0.43], [10, 0.79], [12, 1], [14, 0.79], [16, 0.36], [18, 0], [24, 0]];
  return [
    {
      id: "wind-sway", title: c.texts.swayTitle ?? text("Wind sway", "Windschwingung"), description: c.texts.sway, durationS: c.swayS, fps: 30, loop: true, rootMotion: "none", contacts: [],
      phases: [{ id: "sway", fromS: 0, toS: c.swayS, title: text("Sway cycle", "Schwingungszyklus") }],
      events: [{ t: 0, kind: "motion", title: text("Cycle start at mean lean", "Zyklusbeginn in mittlerer Neigung") }],
      keys: swayKeys,
      tracks: [{ target: `emitter:${c.emitter}`, ease: "smooth", keys: [{ t: 0, v: 1 }, { t: c.swayS / 2, v: 0.9 }, { t: c.swayS, v: 1 }] }],
    },
    {
      id: "gust", title: text("Gust", "Windböe"), description: c.texts.gust, durationS: 8, fps: 30, loop: false, rootMotion: "none", contacts: [],
      phases: [{ id: "rise", fromS: 0, toS: 2.5, title: text("Gust front", "Böenfront") }, { id: "peak", fromS: 2.5, toS: 3.6, title: text("Peak", "Spitze") }, { id: "decay", fromS: 3.6, toS: 8, title: text("Decay and recovery", "Abklingen und Erholung") }],
      events: [{ t: 2.5, kind: "load-on", title: text("Peak gust load", "Böenspitze") }, { t: 5, kind: "load-off", title: text("Load below the mean again", "Last wieder unter dem Mittelwert") }],
      keys: gustKeys,
      tracks: c.loadTargets.map((target) => ({ target: `load:${target}`, ease: "smooth", keys: loadTrack })),
    },
    {
      id: "diurnal-transpiration", title: text("Day of transpiration", "Transpirationstag"), description: c.texts.diurnal, durationS: 24, fps: 10, loop: true, rootMotion: "none", contacts: [],
      phases: [{ id: "night", fromS: 0, toS: 6, title: text("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: text("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: text("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: text("Evening and night", "Abend und Nacht") }],
      events: [{ t: 6, kind: "thermal", title: text("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: text("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration") }, { t: 18, kind: "thermal", title: text("Sunset", "Sonnenuntergang") }],
      keys: [0, 12, 24].map((t) => ({ t, pose: "calm", ease: "smooth", bones: calmBones })),
      tracks: [
        { target: `emitter:${c.emitter}`, ease: "smooth", keys: [[0, 0.03], [5, 0.05], [7, 0.3], [9, 0.6], [12, 0.8], [15, 0.7], [18, 0.35], [20, 0.08], [24, 0.03]].map(([t, v]) => ({ t, v })) },
        ...c.regions.map((r) => ({ target: `region:${r.id}:sunlitFraction`, ease: "smooth", keys: sun.map(([t, v]) => ({ t, v: Math.round(v! * r.peak * 1000) / 1000 })) })),
      ],
    },
  ];
}

/** 🍃️ The lamina of a leaf along the curved midrib of a `FrondGeom`, cut into the leaf's two bones: an inner band of half width `ti` (share of the half width `W`) on both sides of the midrib stays whole, the outer part of each side is cut into strips at the tear (or pinna) positions of that side; each tear opens towards the margin by `gap(S)`, never by more than 45 percent of the distance to the next row of its strip (the rows of a side also hold the tears of the other side), so an opened corner never passes its neighbour. `fold(t, S)` lifts the surface along the frond normal (t = -1 right margin, 0 midrib, +1 left margin). Strip faces are banded with the u of their inner edge, so a strip never leaves the band of its base. */
export function lamina(o: {
  f: Leaf; g: FrondGeom; F: Record<string, Frame>; id: string; from: number; to: number; W: (S: number) => number; ti: number; tears: { l: number[]; r: number[] }; gap: (S: number) => number;
  fold: (t: number, S: number) => number; rowStep: number; outerCols: number; half: number; fields: Json; region: string;
}): Sheet[] {
  const out: Sheet[] = [];
  for (const h of ["a", "b"] as const) {
    const [s0, s1] = h === "a" ? [Math.max(o.from, -1), Math.min(o.to, o.f.La)] : [Math.max(o.from, o.f.La), o.to];
    if (s1 - s0 < 1e-3) continue;
    const bone = `${o.f.id}${h}`;
    const B = o.F[bone]!;
    const tearsIn = [...o.tears.l, ...o.tears.r].filter((s) => s > s0 + 0.02 && s < s1 - 0.02);
    const grid = [...new Set([...steps(s0, s1, o.rowStep).filter((x) => x === s0 || x === s1 || tearsIn.every((t) => Math.abs(t - x) > o.rowStep * 0.4)), ...tearsIn].map((x) => Math.round(x * 1e5) / 1e5))].sort((a, b) => a - b);
    const pos = (S: number, t: number) => {
      const q = o.g.at(S, h);
      return { p: add(q.p, add(mul(q.w, t * o.W(S)), mul(q.n, o.fold(t, S)))), n: q.n };
    };
    const tis = [-o.ti, 0, o.ti];
    const sheet = gridSheet(grid.length, 3, (i, j) => pos(grid[i]!, tis[j]!));
    const faceU: number[] = sheet.faces.map((face) => face.reduce((a, i) => a + dot(sub(sheet.p[i]!, B.o), B.u), 0) / face.length);
    if (o.ti < 1) for (const side of [1, -1]) {
      const tears = (side > 0 ? o.tears.l : o.tears.r).filter((s) => s > s0 + 0.02 && s < s1 - 0.02).map((s) => grid.indexOf(Math.round(s * 1e5) / 1e5));
      const cuts = [0, ...tears, grid.length - 1];
      const nc = o.outerCols + 1;
      const ts = Array.from({ length: nc }, (_, j) => o.ti + ((1 - o.ti) * j) / (nc - 1)).map((t) => t * side);
      if (side < 0) ts.reverse();
      for (let c = 0; c + 1 < cuts.length; c++) {
        const [ia, ib] = [cuts[c]!, cuts[c + 1]!];
        if (ib <= ia) continue;
        const rows = ib - ia + 1;
        const strip = gridSheet(rows, nc, (i, j) => {
          const S0 = grid[ia + i]!;
          const frac = (Math.abs(ts[j]!) - o.ti) / (1 - o.ti);
          let S = S0;
          const g = Math.min(o.gap(S0) / 2, (grid[ib]! - grid[ia]!) * 0.225);
          if (i === 0 && ia > 0) S += Math.min(g, (grid[ia + 1]! - grid[ia]!) * 0.45) * frac;
          if (i === rows - 1 && ib < grid.length - 1) S -= Math.min(g, (grid[ib]! - grid[ib - 1]!) * 0.45) * frac;
          return pos(S, ts[j]!);
        });
        const innerCol = side > 0 ? 0 : nc - 1;
        const weld: [number, number][] = Array.from({ length: rows }, (_, i) => [i * nc + innerCol, (ia + i) * 3 + (side > 0 ? 2 : 0)]);
        const before = sheet.faces.length;
        weldSheet(sheet, strip, weld);
        const key = weld.reduce((a, [, k]) => a + dot(sub(sheet.p[k]!, B.o), B.u), 0) / weld.length;
        for (let k = before; k < sheet.faces.length; k++) faceU.push(key);
      }
    }
    out.push({ id: `${bone}-${o.id}`, bone, ...sheet, half: o.half, fields: o.fields, region: o.region, faceU });
  }
  return out;
}
