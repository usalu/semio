/** 🌳️ PW-05 broadleaf tree kit: turns a hand-written species design (stem chain, limbs, foliage clusters, shade cores and coarse crown masses per scale, all in world rest metres) into a complete `semio.staffage/v2` vegetation asset: rig, anchors, masses, sections, poses, clips, thermal and structural model, the four individually modelled volumes and their 2D drawings as true orthographic views of each scale's body.
 * Every number of a tree lives in its species file; this kit only converts, projects and formats.
 * Imports the round-3 kit of the apple tree (tubes, lobes, outlines) and the volume helpers of the 3D-model ticket from their place.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/design.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { frames, toLocal, type Frame, type Json, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";
import { add, dot3, lobeOutline, mul, nearness, outline, pathOf, project, quaternion, solidOf, sub, toDrawing, tubePolygons, unit, type Lobe, type Tube, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";

export type Text = { en: string; de: string };
export type Scale = 50 | 100 | 200 | 500;
export const SCALES: Scale[] = [50, 100, 200, 500];
export const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
export const D2R = Math.PI / 180;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
export const r1 = (x: number): number => Math.round(x * 10) / 10;
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const len3 = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const dirOf = (azDeg: number, elDeg: number): V3 => [Math.cos(elDeg * D2R) * Math.cos(azDeg * D2R), Math.sin(elDeg * D2R), Math.cos(elDeg * D2R) * Math.sin(azDeg * D2R)];
export const tangentOf = (azDeg: number): V3 => [-Math.sin(azDeg * D2R), 0, Math.cos(azDeg * D2R)];
const de = (x: number, digits = 2): string => x.toFixed(digits).replace(".", ",");
const en = (x: number, digits = 2): string => x.toFixed(digits);

/** 🪵️ One stem segment above the root plate: length, bark radius at base and tip, lean towards +x (degrees from the vertical) and the revolute limit of its joint. */
export type StemSeg = { id: string; title: Text; len: number; r0: number; r1: number; leanDeg: number; limit: number };
/** 🌿️ One bone of a limb: length, elevation above the horizontal, azimuth change against the limb and bark radius at its tip. */
export type LimbSeg = { len: number; el: number; dAz?: number; r1: number };
/** 🌲️ A scaffold limb: springs from stem bone `parent` at `at` metres along it, azimuth `az` (degrees from +x towards +z, the left), base bark radius `r0`, its bones, its end shoot (`fol-<id>`), its foliage cluster `size` (radial, vertical, tangential semi-axes of the main mass), a template turn `spin` and side branches at 1:50 and 1:100. */
export type Limb = { id: string; title: Text; parent: string; at: number; az: number; r0: number; segs: LimbSeg[]; fol: { len: number; el: number; dAz?: number; r: number }; size: V3; spin?: number; side?: { seg: number; at: number; dAz: number; el: number; len: number; r: number }[]; raceme?: boolean };
/** 🍃️ A lobe of a foliage template in cluster units: offsets along (radial, up, tangential) in units of the cluster semi-axes and radius factors. */
export type LobeT = [number, number, number, number, number, number];
/** 🌥️ A mass in world metres bound to a bone: centre, semi-axes (radial, vertical, tangential), azimuth of the radial axis and region group. */
export type Mass = { id: string; bone: string; c: V3; radii: V3; az: number; group: "sun" | "shade" };

/** 🧬️ The whole hand-written design of one species. */
export type Species = {
  id: string;
  title: Text;
  summary: Text;
  tags: string[];
  latin: string;
  references: { id: string; citation: string }[];
  notes: Text;
  distinction: Text;
  materials: Json[];
  wood: { material: string; density: number; bark: string; barkYoung: string; leafSun: string; leafShade: string; flower?: string };
  root: { len: number; flareR: number; topR: number; roots: { az: number; len: number; r: number }[] };
  stem: StemSeg[];
  top: { len: number; r: number; size: V3 };
  limbs: Limb[];
  templates: Record<Scale, LobeT[]>;
  coarse: Record<Scale, number>;
  shade: Record<Scale, Mass[]>;
  crown500: Mass[];
  ridges?: { count: number; depth: number; width: number; scales: Scale[] };
  leaf: { sunGs: number; shadeGs: number; size: number; kgPerM2: number; lai: number; sunShare: number; porosity: number; snowKgM2: number };
  heights: { crownBase: number };
  sway: { periodS: number; windyLean: number; stormLean: number };
  poseTexts: Record<string, { title: Text; description: Text }>;
  scaleTexts: Record<Scale, { silhouette: Text; body: Text }>;
  extraPoses?: (ctx: Ctx) => Json[];
  racemes?: { len: number; r: number; kgPerCluster: number; areaPerCluster: number };
};

export type DesignBone = { id: string; parent: string | null; title: Text; o: V3; u: V3; w: V3; len: number; r0: number; r1: number; kind: "root" | "stem" | "limb" | "fol" | "raceme" | "top"; limit: [number, number] | null; az: number; limb?: Limb; seg?: number };
export type Ctx = { sp: Species; bones: DesignBone[]; byId: Map<string, DesignBone>; F: Record<string, Frame>; asset: Json; parts: Record<Scale, Part[]>; leafAreas: Record<string, number>; frontal: Record<string, number>; planArea: Record<string, number>; centroid: Record<string, V3> };
export type Part = (Tube | Lobe) & { slender?: boolean; texture?: boolean };

/** 🧭️ The design bones of a species: root plate, stem chain, limbs with their end shoots, the top shoot and the raceme bones. */
export function designBones(sp: Species): DesignBone[] {
  const out: DesignBone[] = [];
  const up: V3 = [0, 1, 0];
  const z: V3 = [0, 0, 1];
  out.push({ id: "root-plate", parent: null, title: { en: "Root plate and root flare", de: "Wurzelteller und Wurzelanlauf" }, o: [0, 0, 0], u: up, w: z, len: sp.root.len, r0: sp.root.flareR, r1: sp.root.topR, kind: "root", limit: null, az: 0 });
  let o: V3 = [0, sp.root.len, 0];
  let parent = "root-plate";
  let r0 = sp.root.topR;
  for (const s of sp.stem) {
    const u: V3 = [Math.sin(s.leanDeg * D2R), Math.cos(s.leanDeg * D2R), 0];
    out.push({ id: s.id, parent, title: s.title, o, u, w: z, len: s.len, r0: s.r0 ?? r0, r1: s.r1, kind: "stem", limit: [-s.limit, s.limit], az: 0 });
    o = add(o, mul(u, s.len));
    parent = s.id;
    r0 = s.r1;
  }
  const last = out[out.length - 1]!;
  out.push({ id: "fol-top", parent: last.id, title: { en: "Top shoot with its foliage", de: "Wipfeltrieb mit Laub" }, o, u: last.u, w: z, len: sp.top.len, r0: last.r1, r1: sp.top.r, kind: "top", limit: [-14, 14], az: 0 });
  const stemOf = new Map(out.map((b) => [b.id, b]));
  for (const limb of sp.limbs) {
    const host = stemOf.get(limb.parent)!;
    let p = add(host.o, mul(host.u, limb.at));
    let az = limb.az;
    let r = limb.r0;
    let par = limb.parent;
    limb.segs.forEach((s, i) => {
      az += s.dAz ?? 0;
      const u = dirOf(az, s.el);
      const id = `${limb.id}-${"abc"[i]}`;
      out.push({ id, parent: par, title: { en: `${limb.title.en}, ${["inner", "middle", "outer"][i]} part`, de: `${limb.title.de}, ${["innerer", "mittlerer", "äußerer"][i]} Teil` }, o: p, u, w: tangentOf(az), len: s.len, r0: r, r1: s.r1, kind: "limb", limit: i === 0 ? [-6, 6] : [-9, 9], az, limb, seg: i });
      p = add(p, mul(u, s.len));
      r = s.r1;
      par = id;
    });
    az += limb.fol.dAz ?? 0;
    out.push({ id: `fol-${limb.id}`, parent: par, title: { en: `${limb.title.en}, end shoots with foliage`, de: `${limb.title.de}, Endtriebe mit Laub` }, o: p, u: dirOf(az, limb.fol.el), w: tangentOf(az), len: limb.fol.len, r0: r, r1: limb.fol.r, kind: "fol", limit: [-14, 14], az, limb });
  }
  return out;
}

/** 📐️ The rest placement fields of a bone in its parent's frame (the solve of `rig-rest.ts`). */
function placement(b: DesignBone, parent: { o: V3; u: V3; v: V3; w: V3 } | null): Json {
  if (parent === null) return { x: 0, y: 0, angleDeg: 90 };
  const u = unit(b.u);
  const w = unit(sub(b.w, mul(u, dot3(b.w, u))));
  const v = cross(w, u);
  const d = sub(b.o, parent.o);
  const deg = 180 / Math.PI;
  const lateral = Math.min(1, Math.max(-1, dot3(u, parent.w)));
  const abductionDeg = Math.asin(lateral) * deg;
  const angleDeg = Math.atan2(dot3(u, parent.v), dot3(u, parent.u)) * deg;
  const [c, s] = [Math.cos(angleDeg / deg), Math.sin(angleDeg / deg)];
  const u1 = add(mul(parent.u, c), mul(parent.v, s));
  const v1 = add(mul(parent.u, -s), mul(parent.v, c));
  const w2 = add(mul(u1, -Math.sin(abductionDeg / deg)), mul(parent.w, Math.cos(abductionDeg / deg)));
  const twistDeg = Math.atan2(-dot3(v, w2), dot3(v, v1)) * deg;
  const out: Json = { x: r4(dot3(d, parent.u)), y: r4(dot3(d, parent.v)) };
  const zz = r4(dot3(d, parent.w));
  if (Math.abs(zz) > 0) out.z = zz;
  out.angleDeg = r3(angleDeg);
  if (Math.abs(abductionDeg) > 5e-4) out.abductionDeg = r3(abductionDeg);
  if (Math.abs(twistDeg) > 5e-4) out.twistDeg = r3(twistDeg);
  return out;
}

const frameOf = (b: DesignBone) => {
  const u = unit(b.u);
  const w = unit(sub(b.w, mul(u, dot3(b.w, u))));
  return { o: b.o, u, v: cross(w, u), w };
};

/** 🦴️ The rig of the design: placements solved from the world design, joints, masses (green wood volume times density, foliage at its fresh mass per leaf area) and solid circular sections of the wood under the bark. */
export function rigOf(sp: Species, bones: DesignBone[], massOf: (b: DesignBone) => { kg: number; comX: number }): Json {
  const byId = new Map(bones.map((b) => [b.id, b]));
  return {
    kind: "botanical",
    root: "root-plate",
    bones: bones.map((b) => {
      const parent = b.parent === null ? null : frameOf(byId.get(b.parent)!);
      const m = massOf(b);
      const rs = 0.92 * (b.r0 + b.r1) / 2;
      const joint = b.limit === null ? { type: "fixed" } : { type: "revolute", minDeg: b.limit[0], maxDeg: b.limit[1] };
      const out: Json = { id: b.id, parent: b.parent, title: b.title, ...placement(b, parent), lengthM: r4(b.len), joint, mass: { kg: r1(m.kg) || 0.1, comX: r3(m.comX), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "wood-handbook-2021" } };
      if (b.kind !== "raceme") {
        const d = 2 * rs;
        const label = b.kind === "root" ? { en: "Root collar", de: "Wurzelhals" } : b.kind === "stem" ? { en: "Stem", de: "Stamm" } : b.kind === "limb" ? { en: "Limb", de: "Ast" } : { en: "End shoot", de: "Endtrieb" };
        out.section = { areaM2: Number((Math.PI * rs * rs).toPrecision(4)), secondMomentM4: Number(((Math.PI * rs ** 4) / 4).toPrecision(4)), extremeFibreM: r4(rs), material: sp.wood.material, title: { en: `${label.en}, d = ${en(d)} m`, de: `${label.de}, d = ${de(d)} m` } };
      }
      return out;
    }),
  };
}

/** 🪵️ A straight tapered tube along a design bone, from `s0` to `s1` metres along it (negative `s0` starts inside the parent), radius `ra` to `rb`. */
export function boneTube(id: string, b: DesignBone, material: string, tone: number, group: string, s0: number, s1: number, ra: number, rb: number, n = 2, bone = b.id): Part {
  const points = Array.from({ length: n + 1 }, (_, i) => {
    const f = i / n;
    return { p: add(b.o, mul(b.u, s0 + (s1 - s0) * f)) as V3, r: r4(ra + (rb - ra) * f) };
  });
  return { kind: "tube", id, bone, material, tone, group, points };
}

/** 🪾️ A tube between two world points with its own radii, bound to `bone`. */
export function pointTube(id: string, bone: string, material: string, tone: number, group: string, a: V3, b: V3, ra: number, rb: number, n = 2): Part {
  return { kind: "tube", id, bone, material, tone, group, points: Array.from({ length: n + 1 }, (_, i) => ({ p: add(a, mul(sub(b, a), i / n)) as V3, r: r4(ra + (rb - ra) * (i / n)) })) };
}

/** 🥚️ A world ellipsoid with its radial axis at azimuth `az`, tilted outwards by `tilt` degrees. */
export function massLobe(id: string, bone: string, material: string, tone: number, group: string, c: V3, radii: V3, az: number, tilt = 0): Part {
  const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const upv: V3 = [0, 1, 0];
  const tan = tangentOf(az);
  const [ct, st] = [Math.cos(tilt * D2R), Math.sin(tilt * D2R)];
  return { kind: "lobe", id, bone, material, tone, group, c, axes: [sub(mul(rad, ct), mul(upv, st)), add(mul(upv, ct), mul(rad, st)), tan], radii };
}

/** 🌳️ The cluster centre of a limb: 55 % along its end shoot. */
export const clusterCentre = (b: DesignBone): V3 => add(b.o, mul(b.u, 0.55 * b.len));

/** 🍀️ The lobes of one foliage cluster at a scale: the species template turned by the limb's `spin`, scaled by the cluster size and the scale's coarse growth. */
export function clusterLobes(sp: Species, b: DesignBone, scale: Scale): Part[] {
  const limb = b.limb!;
  const c = clusterCentre(b);
  const grow = sp.coarse[scale];
  const [A, B, C] = limb.size;
  const az = b.az + (limb.spin ?? 0);
  const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const tan = tangentOf(az);
  return sp.templates[scale].map((t, i) => {
    const off = add(add(mul(rad, t[0] * A), [0, t[1] * B, 0]), mul(tan, t[2] * C));
    return massLobe(`${b.id}-l${i + 1}`, b.id, sp.wood.leafSun, 0.6, "sun", add(c, off), [r4(t[3] * A * grow), r4(t[4] * B * grow), r4(t[5] * C * grow)], az);
  });
}

/** 🔭️ Extent of a part along a world axis: lobes exactly, tubes by their end discs. */
export function partExtent(p: Part, axis: 0 | 1 | 2): [number, number] {
  const e: V3 = [0, 0, 0];
  e[axis] = 1;
  if (p.kind === "lobe") {
    const h = Math.sqrt(p.axes.reduce((s, a, i) => s + (dot3(a, e) * p.radii[i]!) ** 2, 0));
    return [p.c[axis] - h, p.c[axis] + h];
  }
  const n = p.points.length;
  const d = unit(sub(p.points[n - 1]!.p, p.points[0]!.p));
  const k = Math.sqrt(Math.max(0, 1 - dot3(d, e) ** 2));
  let lo = Infinity, hi = -Infinity;
  for (const s of p.points) {
    lo = Math.min(lo, s.p[axis] - s.r * k);
    hi = Math.max(hi, s.p[axis] + s.r * k);
  }
  return [lo, hi];
}

export function boxOf(parts: readonly Part[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const p of parts) for (const a of [0, 1, 2] as const) {
    const [lo, hi] = partExtent(p, a);
    min[a] = Math.min(min[a], lo);
    max[a] = Math.max(max[a], hi);
  }
  return { min, max };
}

const polyArea = (ring: readonly P2[]): number => Math.abs(ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2);
export const ringsArea = (rings: P2[][]): number => rings.reduce((s, r) => s + polyArea(r) * (signed(r) >= 0 ? 1 : -1), 0);
const signed = (ring: readonly P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0);

/** 🖼️ The union outline of parts in a view, simplified to a quarter feature. */
export function outlineOf(view: View, parts: readonly Part[], scale: Scale): P2[][] {
  const polys = parts.flatMap((p) => (p.kind === "lobe" ? [lobeOutline(view, p, 72)] : tubePolygons(view, p)));
  return outline(polys, FEATURE[scale] / 4, (3 * FEATURE[scale]) ** 2);
}

/** 🌡️ Region of a part group. */
export const regionOf = (group: string): string => (group === "stem" || group === "root" || group === "ridge" ? "trunk-bark" : group === "sun" ? "leaf-sun" : group === "shade" ? "leaf-shade" : group === "raceme" ? "racemes" : "limb-bark");

/** 🧱️ Lateral area of a tube (frusta) plus its end discs. */
export function tubeArea(t: Tube): number {
  let a = 0;
  for (let i = 0; i + 1 < t.points.length; i++) {
    const [p, q] = [t.points[i]!, t.points[i + 1]!];
    const l = len3(sub(q.p, p.p));
    a += Math.PI * (p.r + q.r) * Math.hypot(l, p.r - q.r);
  }
  return a + Math.PI * t.points[t.points.length - 1]!.r ** 2;
}

/** 🗜️ Volume of a tube (frusta). */
export function tubeVolume(t: Tube): number {
  let v = 0;
  for (let i = 0; i + 1 < t.points.length; i++) {
    const [p, q] = [t.points[i]!, t.points[i + 1]!];
    v += (Math.PI * len3(sub(q.p, p.p)) * (p.r * p.r + p.r * q.r + q.r * q.r)) / 3;
  }
  return v;
}

/** 🧊️ The body solid of a part, with the solid flags it declares. */
export function bodySolid(p: Part, F: Record<string, Frame>): Json {
  const s = solidOf(p as Tube | Lobe, F);
  if (p.kind === "lobe") s.role = "body";
  if (p.slender) s.slender = true;
  if (p.texture) s.texture = true;
  return s;
}

/** 📏️ Section rings of a member along a bone in its local frame: a straight round loft at `factor` of the bark radius through `n` stations. */
export function memberLoft(id: string, b: DesignBone, material: string, factor: number, n: number, minR: number, part: "member", slenderIfThin: number): Json {
  const sections = Array.from({ length: n + 1 }, (_, i) => {
    const f = i / n;
    const r = Math.max(minR, factor * (b.r0 + (b.r1 - b.r0) * f));
    return { at: [r4(b.kind === "root" ? b.len * f : -0.02 * b.len + 1.04 * b.len * f), 0, 0], rx: r4(r), ry: r4(r), rotation: [0.5, 0.5, 0.5, 0.5] };
  });
  const thin = Math.min(...sections.map((s) => 2 * s.rx));
  const out: Json = { id, kind: "loft", bone: b.id, material, role: "body", tone: 0.4, sections, capStart: true, capEnd: true, member: b.id, part };
  if (thin < slenderIfThin && b.len >= 5 * 2 * Math.max(...sections.map((s) => s.rx))) out.slender = true;
  return out;
}

const sphereSolid = (id: string, bone: string, material: string, at: V3, r: number, member: string, part: "joint" | "mass"): Json => ({ id, kind: "ellipsoid", bone, material, role: "detail", tone: part === "joint" ? 0.35 : 0.3, radii: [r4(r), r4(r), r4(r)], transform: { translation: at.map(r4) }, member, part });

/** 🏗️ The structural list of a scale: members along every load-bearing bone, joint spheres where the scale shows joints, the root-plate support disc and mass spheres. */
export function structureList(ctx: Ctx, scale: Scale): Json[] {
  const { sp, bones, asset } = ctx;
  const f = FEATURE[scale];
  const out: Json[] = [];
  const stations: Record<Scale, number> = { 50: 4, 100: 3, 200: 2, 500: 1 };
  for (const b of bones) {
    if (b.kind === "raceme") continue;
    const m = memberLoft(`member-${b.id}`, b, sp.wood.material, b.kind === "root" ? 0.68 : 0.8, b.kind === "root" ? 1 : stations[scale], 0.5 * f * 1.02, "member", f);
    out.push(m);
  }
  const jointed = bones.filter((b) => b.limit !== null && b.kind !== "raceme" && (scale === 50 || (scale === 100 && b.kind !== "fol") || (scale === 200 && (b.kind === "stem" || (b.kind === "limb" && b.seg === 0))) || (scale === 500 && b.kind === "stem")));
  for (const b of jointed) {
    const r = Math.max(0.55 * f * 1.02, 0.85 * b.r0);
    out.push(sphereSolid(`joint-${b.id}`, b.id, sp.wood.material, [0, 0, 0], r, b.id, "joint"));
  }
  const support = (asset.anchors as Json[]).find((a) => a.id === "base-support");
  out.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: sp.wood.material, role: "detail", tone: 0.25, profile: [[-Math.max(f * 1.2, 0.02), r4(sp.root.flareR * 0.9)], [0, r4(sp.root.flareR * 0.9)]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: support.id });
  const massed = bones.filter((b) => b.kind === "root" || b.kind === "stem" || (b.kind === "limb" && b.seg === 0 && scale <= 100));
  for (const b of massed) {
    const rig = (asset.rig.bones as Json[]).find((x) => x.id === b.id);
    const r = Math.max(0.55 * f * 1.02, Math.min(0.6 * b.r1, 0.12));
    out.push(sphereSolid(`mass-${b.id}`, b.id, sp.wood.material, [rig.mass.comX, 0, 0], r, b.id, "mass"));
  }
  return out;
}

/** 🎨️ The 2D shapes of one scale and view: one outline per stem bone, per limb bone, per foliage cluster and per shade core, painted in the depth order of the view; elevation shapes are bound to the stem bone that carries them, front and plan shapes to their own bone. */
export function drawView(ctx: Ctx, scale: Scale, view: View): Json[] {
  const { sp, byId, F } = ctx;
  const parts = ctx.parts[scale];
  const groups = new Map<string, { id: string; bone: string; ownBone: string; region: string; tone: number; material: string; parts: Part[]; hatch?: string }>();
  const stemHost = (bone: string): string => {
    let b = byId.get(bone);
    while (b && (b.kind === "limb" || b.kind === "fol")) b = byId.get(b.parent!);
    return b ? b.id : bone;
  };
  for (const p of parts) {
    if (p.texture) continue;
    const region = regionOf(p.group);
    const key = p.group === "sun" || p.group === "shade" ? `${p.bone}|${region}|${p.group === "shade" && p.id.startsWith("shade-") ? p.id.replace(/-l\d+$/, "") : ""}` : p.group === "raceme" ? `${p.bone}|racemes` : `${p.bone}|${region}`;
    let g = groups.get(key);
    if (!g) {
      const own = p.bone;
      const isStem = byId.get(own)?.kind === "stem" || byId.get(own)?.kind === "root" || byId.get(own)?.kind === "top" || byId.get(own)?.kind === "raceme";
      const id = p.group === "sun" ? `${own}-sun` : p.group === "shade" ? (p.id.startsWith("shade-") ? p.id.replace(/-l\d+$/, "") : `${own}-shade`) : p.group === "raceme" ? own : own;
      g = { id, bone: view === "elevation" && !isStem ? stemHost(own) : own, ownBone: own, region, tone: p.group === "sun" ? 0.6 : p.group === "shade" ? 0.44 : p.group === "raceme" ? 0.85 : p.group === "stem" || p.group === "root" ? 0.3 : 0.34, material: p.material, parts: [], ...(p.group === "shade" && scale <= 100 ? { hatch: "shade-lines" } : {}) };
      groups.set(key, g);
    }
    g.parts.push(p);
  }
  const list = [...groups.values()].map((g) => {
    const rings = outlineOf(view, g.parts, scale);
    const near = Math.max(...g.parts.map((p) => (p.kind === "lobe" ? nearness(view, p.c) + Math.max(...p.radii) * 0.5 : Math.max(...p.points.map((q) => nearness(view, q.p) + q.r)))));
    return { g, rings, near };
  }).filter((x) => x.rings.length > 0 && x.rings[0]!.length > 2);
  list.sort((a, b) => a.near - b.near || a.g.id.localeCompare(b.g.id));
  return list.map(({ g, rings }, i) => {
    const local = toDrawing(view, F[g.bone]!, rings);
    const shape: Json = { id: g.id, bone: g.bone, role: "body", filled: true, tone: g.tone, region: g.region, material: g.material, kind: "path", d: pathOf(local, true), z: r3(1 + i / 1000) };
    if (g.hatch) shape.hatch = g.hatch;
    return shape;
  });
}

/** ✒️ Texture strokes of the bark (ridges between furrows) on the stem bones of a scale, drawn inside the stem outline of the elevation and front. */
export function barkStrokes(ctx: Ctx, scale: Scale, view: View): Json[] {
  const { sp, F } = ctx;
  if (!sp.ridges || !sp.ridges.scales.includes(scale) || view === "plan") return [];
  const out: Json[] = [];
  for (const b of ctx.bones.filter((x) => x.kind === "stem").slice(0, 2)) {
    const n = scale === 50 ? 5 : 3;
    for (let k = 0; k < n; k++) {
      const f = (k + 0.5) / n - 0.5;
      const pts: P2[] = [];
      const steps = 6;
      for (let i = 0; i <= steps; i++) {
        const s = 0.08 * b.len + (0.84 * b.len * i) / steps;
        const r = b.r0 + (b.r1 - b.r0) * (s / b.len);
        const wig = 0.18 * Math.sin(i * 1.9 + k * 2.3);
        const lateral = r * (1.5 * f + 0.12 * wig) * 0.9;
        const p = add(b.o, mul(b.u, s));
        const q: V3 = view === "elevation" ? [p[0] + lateral, p[1], p[2] - r * 0.95] : [p[0] - r * 0.95, p[1], p[2] + lateral];
        pts.push(project(view, q));
      }
      const local = toDrawing(view, F[b.id]!, [pts])[0]!;
      out.push({ id: `${b.id}-ridge-${k + 1}`, bone: b.id, role: "texture", filled: false, tone: 0.2, kind: "path", d: pathOf([local], false), z: r3(5 + k / 100) });
    }
  }
  return out;
}

/** 🧾️ Formats a number list as "a, b and c" in both languages. */
export const count = (n: number): Text => ({ en: String(n), de: String(n) });

/** 💾️ Writes a JSON document with two-space-free compact rows (one solid or shape per line keeps diffs readable). */
export function writeDoc(path: string, doc: Json): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(doc, null, 1) + "\n");
}

export { frames, toLocal, project, nearness, lobeOutline, quaternion, join, add, mul, sub, unit, dot3 };
export type { Frame, Json, P2, V3, View, Lobe, Tube };
