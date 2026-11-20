/** 🎋️ The round-3 bamboo clump (Phyllostachys aureosulcata) stated by hand in world space in the default pose, per scale: the HDPE rhizome barrier with the leaf-litter bed, six culms with their internodes and node rings (sheath scar and supranodal ridge), two unequal branches at every node of the upper culm on alternating sides, branchlets ending in fans of drooping lanceolate leaves (1:50), leaf sprays (1:100), branch tufts (1:200), three foliage masses per culm (1:500), and the twelve transpiration outlets.
 * Every number below is a stated measure of the real plant (sources in `r3-vegetation-bamboo-clump.md`, section 1); the code only places and converts them.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-bamboo-clump.md */
import type { Frame, Json, V3 } from "./volumes-lib.ts";
import { D2R, add, cross, dot, len, lin, mul, sub, sweep, table, unit, type Blob, type Section, type Tube } from "./r3-date-palm-kit.ts";

export type Scale = 50 | 100 | 200 | 500;
export const UP: V3 = [0, 1, 0];

/** 🎋️ One culm as authored: base position, lean and the measures of the real culm. */
export type CulmSpec = { k: number; z0: number; leanDeg: number; d0: number; phi0: number; drift: number; reach: number; nod: number };

/** 📏️ Culm table: base lateral offset z0 (m), outward lateral lean (deg; the joints of the upper bones sit on the leaning culm through their lateral offsets `z`), base diameter d0 (m, height-diameter allometry of Phyllostachys, about 155 D), azimuth of the first branch (deg) and its drift per node (deg; the branches alternate sides, the outer culms almost in one plane), branch reach factor and nod of the culm top (m). */
export const CULMS: CulmSpec[] = [
  { k: 1, z0: 0.25, leanDeg: 4.3, d0: 0.024, phi0: 180, drift: 9, reach: 1.1, nod: 0.2 },
  { k: 2, z0: -0.28, leanDeg: -2.6, d0: 0.029, phi0: 110, drift: 23, reach: 0.95, nod: 0.03 },
  { k: 3, z0: 0.12, leanDeg: 2.2, d0: 0.0265, phi0: -50, drift: 23, reach: 0.95, nod: 0.12 },
  { k: 4, z0: -0.22, leanDeg: -3.5, d0: 0.023, phi0: 0, drift: 9, reach: 1.0, nod: 0.12 },
  { k: 5, z0: 0.2, leanDeg: 3.5, d0: 0.019, phi0: 10, drift: 9, reach: 1.1, nod: 0.1 },
  { k: 6, z0: -0.1, leanDeg: -5.5, d0: 0.016, phi0: 190, drift: 9, reach: 1.14, nod: 0.16 },
];

/** 📐️ Diameter along the culm (share of the base diameter over the share of the culm length). */
const TAPER: [number, number][] = [[0, 1], [0.08, 0.995], [0.3, 0.93], [0.5, 0.8], [0.7, 0.6], [0.85, 0.4], [0.95, 0.22], [1, 0.13]];
/** 📐️ Internode length along the culm (share of the longest internode): short near the ground, longest at a third of the height, shorter again at the top. */
const INTERNODE: [number, number][] = [[0, 0.22], [0.04, 0.35], [0.12, 0.72], [0.28, 1], [0.5, 0.95], [0.75, 0.78], [1, 0.55]];
/** 📐️ Half width of a lanceolate leaf blade over its length, as a share of the widest half width: narrowed base with the short petiole, widest at 30 %, acuminate tip. */
const BLADE: [number, number][] = [[0, 0.16], [0.05, 0.34], [0.14, 0.82], [0.3, 1], [0.5, 0.9], [0.7, 0.63], [0.86, 0.34], [1, 0.05]];

/** 📐️ Half width of the spray that stands for one leaf fan at 1:100, as a share of the half width of the fan: the sector the splayed leaves sweep, widening from the branchlet to three quarters of the leaf length and closing to the tip of the middle leaf. */
const FAN: [number, number][] = [[0, 0.1], [0.2, 0.3], [0.5, 0.7], [0.74, 1], [0.87, 0.73], [0.95, 0.47], [1, 0.12]];

export const WALL = 0.13;
export const BRANCH_FROM = 0.4;
export const LEAF_LMA = 0.13;

/** 🔢️ A deterministic variation in [-1, 1] for a key. */
export function jitter(...key: number[]): number {
  let h = 2166136261;
  for (const k of key) h = Math.imul(h ^ Math.round(k * 1000 + 7919), 16777619);
  h ^= h >>> 13;
  h = Math.imul(h, 0x5bd1e995);
  h ^= h >>> 15;
  return ((h >>> 0) / 4294967295) * 2 - 1;
}

const rotateAbout = (p: V3, axis: V3, ang: number): V3 => {
  const k = unit(axis);
  const [c, s] = [Math.cos(ang), Math.sin(ang)];
  return add(add(mul(p, c), mul(cross(k, p), s)), mul(k, dot(k, p) * (1 - c)));
};
const horiz = (d: V3): V3 => unit([d[0], 0, d[2]]);
const perp = (d: V3, t: V3): V3 => unit(sub(d, mul(t, dot(d, t))));

/** 🎋️ The geometry of one culm in the default pose: centre line, tangent and diameter by arclength from the ground, the nodes, and which bone carries each stretch. */
export class Culm {
  readonly bones: { a: string; b: string; c: string; f: string };
  readonly L: number;
  readonly s1: number;
  readonly s2: number;
  readonly sf: number;
  readonly nodes: number[];
  readonly out: V3;
  constructor(readonly spec: CulmSpec, readonly F: Record<string, Frame>, rig: Json) {
    const id = (p: string) => `${p}`;
    const k = spec.k;
    this.bones = { a: id(`culm-${k}-a`), b: id(`culm-${k}-b`), c: id(`culm-${k}-c`), f: id(`fol-${k}`) };
    const bone = (b: string) => rig.bones.find((x: Json) => x.id === b);
    const [La, Lb, Lc] = [bone(this.bones.a).lengthM, bone(this.bones.b).lengthM, bone(this.bones.c).lengthM];
    this.s1 = La;
    this.s2 = La + Lb;
    this.sf = La + Lb + bone(this.bones.f).x;
    this.L = La + Lb + Lc;
    const ff = F[this.bones.f]!;
    this.out = horiz(add(ff.u, mul(ff.w, Math.tan(spec.leanDeg * D2R) * 3)));
    this.nodes = this.layNodes();
  }

  /** 🪜️ Node positions by arclength: internodes follow `INTERNODE`, the nodes nearest to the three joints are moved onto them so that every rig joint sits at a node. */
  private layNodes(): number[] {
    const longest = 0.19 * Math.pow(this.spec.d0 / 0.029, 0.6);
    const raw: number[] = [0.025];
    while (raw[raw.length - 1]! < this.L - 0.05) raw.push(raw[raw.length - 1]! + longest * table(INTERNODE, raw[raw.length - 1]! / this.L));
    raw.pop();
    const joints = [this.s1, this.s2, this.sf];
    const fixed = joints.map((j) => raw.reduce((best, s, i) => (Math.abs(s - j) < Math.abs(raw[best]! - j) ? i : best), 0));
    const out = [...raw];
    const anchors: [number, number][] = [[0, raw[0]!], ...fixed.map((i, n) => [i, joints[n]!] as [number, number]), [raw.length - 1, raw[raw.length - 1]!]];
    for (let a = 0; a + 1 < anchors.length; a++) {
      const [i0, v0] = anchors[a]!;
      const [i1, v1] = anchors[a + 1]!;
      for (let i = i0; i <= i1; i++) out[i] = i1 === i0 ? v0 : v0 + ((raw[i]! - raw[i0]!) * (v1 - v0)) / (raw[i1]! - raw[i0]!);
    }
    return out;
  }

  /** 🦴️ The bone that carries the culm at arclength s. */
  boneAt(s: number): string {
    return s < this.s1 ? this.bones.a : s < this.s2 ? this.bones.b : s < this.sf ? this.bones.c : this.bones.f;
  }

  /** 🧷️ A point on the axis of a bone at x from its joint, with the lateral lean of the culm. */
  private on(bone: string, x: number): V3 {
    const f = this.F[bone]!;
    return add(add(f.o, mul(f.u, x)), mul(f.w, x * Math.tan(this.spec.leanDeg * D2R)));
  }

  /** 🪢️ The four rig joints of the culm (ground, a-b, b-c, c-top) with the tangent of the culm at each: a culm does not kink at a node, so the tangent at a joint is the bisector of the two bone axes that meet there, and the base leaves the ground on the arc of the lowest bone. */
  private knots(): { s: number; p: V3; t: V3 }[] {
    if (this.cache) return this.cache;
    const p = [this.on(this.bones.a, 0), this.on(this.bones.b, 0), this.on(this.bones.c, 0), this.on(this.bones.f, 0)];
    const d = [unit(sub(p[1]!, p[0]!)), unit(sub(p[2]!, p[1]!)), unit(sub(p[3]!, p[2]!)), unit(sub(this.on(this.bones.f, 1), p[3]!))];
    const t1 = unit(add(d[0]!, d[1]!));
    const t = [unit(sub(mul(d[0]!, 2), t1)), t1, unit(add(d[1]!, d[2]!)), unit(add(d[2]!, d[3]!))];
    this.cache = [0, this.s1, this.s2, this.sf].map((s, i) => ({ s, p: p[i]!, t: t[i]! }));
    return this.cache;
  }
  private cache?: { s: number; p: V3; t: V3 }[];

  /** 📍️ Centre of the culm at arclength s: a smooth curve through the rig joints (cubic Hermite per bone, tangents from `knots`) up to the foliage joint, then the top nods outwards and down, leaving the joint on the same tangent. */
  at(s: number): V3 {
    const K = this.knots();
    if (s <= this.sf) {
      const i = s <= this.s1 ? 0 : s <= this.s2 ? 1 : 2;
      const [k0, k1] = [K[i]!, K[i + 1]!];
      const chord = len(sub(k1.p, k0.p));
      const u = Math.max(0, (s - k0.s) / (k1.s - k0.s));
      const [u2, u3] = [u * u, u * u * u];
      return add(add(mul(k0.p, 2 * u3 - 3 * u2 + 1), mul(k0.t, (u3 - 2 * u2 + u) * chord)), add(mul(k1.p, -2 * u3 + 3 * u2), mul(k1.t, (u3 - u2) * chord)));
    }
    const f = this.F[this.bones.f]!;
    const t = s - this.sf;
    const top = this.L - this.sf;
    const q = (t / top) ** 2;
    const axis = sub(this.on(this.bones.f, 1), this.on(this.bones.f, 0));
    const ease = mul(sub(mul(K[3]!.t, len(axis)), axis), t * (1 - t / top) ** 2);
    return add(add(this.on(this.bones.f, t * (1 - 0.04 * q)), ease), add(mul(perp(this.out, f.u), this.spec.nod * q), mul(perp(UP, f.u), -0.25 * this.spec.nod * q)));
  }

  tangent(s: number): V3 {
    const h = 0.004;
    return unit(sub(this.at(Math.min(this.L, s + h)), this.at(Math.max(0, s - h))));
  }

  /** ⭕️ Outer diameter at arclength s. */
  d(s: number): number {
    return this.spec.d0 * table(TAPER, s / this.L);
  }

  /** 🧭️ Radial direction of azimuth phi (deg) about the culm at s: phi 0 is world x, 90 world z, projected perpendicular to the culm. */
  radial(s: number, phiDeg: number): V3 {
    const t = this.tangent(s);
    return perp([Math.cos(phiDeg * D2R), 0, Math.sin(phiDeg * D2R)], t);
  }
}

/** 🧩️ A part of the plant: a loft, a capsule rod, an ellipsoid or an extrude, with its bone, list membership, region and the 2D shape it is drawn in. */
export type Part = {
  id: string;
  bone: string;
  tube?: Tube;
  rod?: { p0: V3; p1: V3; r0: number; r1: number };
  blob?: Blob;
  extrude?: Json;
  fields: Json;
  region?: "culms" | "leaves-sun" | "leaves-shade";
  shape?: string;
  body: boolean;
  slender?: boolean;
  texture?: boolean;
  fan?: string;
  proxy?: boolean;
  role: "body" | "detail";
  centre: V3;
  leafArea?: number;
};

/** 🍃️ A leaf, spray, tuft or mass and how exposed it is (for the sun and shade regions). */
export type Foliage = { part: Part; culm: number; score: number; arc: { p: V3; n: V3 }[]; leaves?: { part: Part; arc: { p: V3; n: V3 }[] }[] };

export type Plant = { culms: Culm[]; parts: Part[]; foliage: Foliage[]; emitters: { id: string; anchor: string; emitter: string; bone: string; part: Part; anchorPoint: V3 }[]; leafCount: number };

const CULM_FIELDS = { material: "bamboo-culm", role: "body", tone: 0.38 };
const BRANCH_FIELDS = { material: "bamboo-culm", role: "detail", tone: 0.3 };
const LEAF_FIELDS = { material: "bamboo-leaf", role: "body", tone: 0.6 };
const BLADE_FIELDS = { material: "bamboo-leaf", role: "texture", tone: 0.6 };

/** 🪜️ The stations of the culm stretch of one bone and the swelling of the radius at each: base or joint overlap, internodes, and at 1:50 and 1:100 the node rings (sheath scar below, supranodal ridge above). */
export function culmStations(c: Culm, s0: number, s1: number, scale: Scale): [number, number][] {
  const at: [number, number][] = [[s0, 1], [s1, 1]];
  if (scale <= 100) {
    const ring = scale === 50 ? [[-0.011, 1], [-0.004, 1.055], [-0.0015, 1.035], [0.003, 1.07], [0.009, 1.02], [0.02, 1]] : [[-0.012, 1], [0, 1.06], [0.014, 1]];
    for (const n of c.nodes) for (const [ds, k] of ring) if (n + ds! > s0 + 0.002 && n + ds! < s1 - 0.002) at.push([n + ds!, k!]);
    const steps = Math.max(2, Math.ceil((s1 - s0) / 0.35));
    for (let i = 1; i < steps; i++) {
      const s = s0 + ((s1 - s0) * i) / steps;
      if (!c.nodes.some((n) => Math.abs(n - s) < 0.03)) at.push([s, 1]);
    }
  } else {
    const steps = Math.max(2, Math.ceil((s1 - s0) / (scale === 200 ? 0.35 : 0.6)));
    for (let i = 1; i < steps; i++) at.push([s0 + ((s1 - s0) * i) / steps, 1]);
  }
  return at.sort((x, y) => x[0] - y[0]);
}

/** 🎋️ The culm stretch of one bone as a loft through its stations. */
function culmTube(c: Culm, bone: string, s0: number, s1: number, scale: Scale): Tube {
  return { id: bone, bone, sections: sweep(culmStations(c, s0, s1, scale).map(([s, k]) => ({ p: c.at(s), a: (c.d(s) / 2) * k })), [0, 0, 1]), fields: CULM_FIELDS };
}

/** 🌿️ A curved twig or branch from p0 leaving in direction d0 with chord length l, drooping by `droopDeg` at its end. */
function branchCurve(p0: V3, d0: V3, l: number, droopDeg: number, n: number): V3[] {
  const el = Math.asin(Math.max(-1, Math.min(1, d0[1])));
  const h = horiz(d0);
  const endDir = unit(add(mul(h, Math.cos(el - droopDeg * D2R)), mul(UP, Math.sin(el - droopDeg * D2R))));
  const chord = unit(add(d0, endDir));
  const e = add(p0, mul(chord, l * 0.96));
  const m = add(p0, mul(d0, 0.5 * l));
  return Array.from({ length: n + 1 }, (_, i) => {
    const t = i / n;
    return lin(lin(p0, (1 - t) * (1 - t), m, 2 * t * (1 - t)), 1, e, t * t);
  });
}

/** 🍃️ A drooping lanceolate leaf from its base q in direction l0: the blade arcs down to the tip (sag), the face turned up, rolled by `roll`. */
function leafSections(q: V3, l0: V3, length: number, halfWidth: number, halfThick: number, sag: number, roll: number, n = 7, form: [number, number][] = BLADE): { sections: Section[]; arc: { p: V3; n: V3 }[] } {
  const side = unit(cross(l0, UP));
  const down = unit(cross(side, l0));
  const pts = Array.from({ length: n + 1 }, (_, i) => {
    const t = i / n;
    return add(add(q, mul(l0, length * t)), mul(down, -sag * t * t));
  });
  const arc: { p: V3; n: V3 }[] = [];
  const sections = pts.map((p, i) => {
    const t = i / n;
    const tan = unit(sub(pts[Math.min(n, i + 1)]!, pts[Math.max(0, i - 1)]!));
    const x0 = perp(rotateAbout(side, l0, roll), tan);
    const y = unit(cross(tan, x0));
    arc.push({ p, n: y[1] >= 0 ? y : mul(y, -1) });
    return { p, x: x0, y, a: Math.max(halfWidth * table(form, t), 0.0004), b: halfThick };
  });
  return { sections, arc };
}

const bladeArea = (length: number, halfWidth: number): number => {
  let a = 0;
  for (let i = 0; i < 40; i++) a += 2 * halfWidth * table(BLADE, (i + 0.5) / 40) * (length / 40);
  return a;
};

/** 🌱️ The whole plant at a scale. */
export function buildPlant(scale: Scale, F: Record<string, Frame>, rig: Json): Plant {
  const feature = (0.15 * scale) / 1000;
  const culms = CULMS.map((spec) => new Culm(spec, F, rig));
  const parts: Part[] = [];
  const foliage: Foliage[] = [];
  const emitters: Plant["emitters"] = [];
  let leafCount = 0;
  const centroid: V3 = mul(culms.reduce((s, c) => add(s, c.at(c.L * 0.7)), [0, 0, 0] as V3), 1 / culms.length);

  if (scale !== 500) {
    const lip = 0.055;
    const ring = (rx: number, rz: number, n = 40): [number, number][] => Array.from({ length: n }, (_, i) => {
      const a = (2 * Math.PI * i) / n;
      const [c, s] = [Math.cos(a), Math.sin(a)];
      const e = 4;
      const k = Math.pow(Math.pow(Math.abs(c), e) + Math.pow(Math.abs(s), e), -1 / e);
      return [-0.1 + rx * c * k, rz * s * k];
    });
    const outer = ring(0.8, 0.5);
    const inner = ring(0.798, 0.498);
    const bed = scale === 200 ? 0.032 : 0.03;
    parts.push({ id: "barrier", bone: "root-plate", extrude: { outer, inner, y0: 0, y1: lip }, fields: { material: "hdpe-root-barrier", role: "detail", tone: 0.15 }, shape: "barrier", body: true, role: "detail", centre: [-0.1, lip / 2, 0] });
    parts.push({ id: "bed", bone: "root-plate", extrude: { outer: inner, inner: null, y0: 0, y1: bed }, fields: { material: "leaf-litter", role: "detail", tone: 0.7 }, shape: "bed", body: true, role: "detail", centre: [-0.1, bed / 2, 0] });
  }

  for (const c of culms) {
    const k = c.spec.k;
    const ov = 0.012;
    const stretches: [string, number, number][] = [[c.bones.a, 0, c.s1 + ov], [c.bones.b, c.s1 - ov, c.s2 + ov], [c.bones.c, c.s2 - ov, c.sf + ov], [c.bones.f, c.sf - ov, c.L]];
    for (const [bone, s0, s1] of stretches) {
      const tube = culmTube(c, bone, s0, s1, scale);
      parts.push({ id: bone, bone, tube, fields: CULM_FIELDS, region: "culms", shape: bone, body: true, slender: true, role: "body", centre: c.at((s0 + s1) / 2) });
    }

    const branched = c.nodes.map((s, i) => ({ s, i })).filter((n) => n.s / c.L >= BRANCH_FROM && n.s < c.L - 0.06);
    const lmax = 0.62 * Math.pow(c.spec.d0 / 0.029, 0.8) * c.spec.reach;
    const masses: { bone: string; pts: V3[] }[] = [];
    const tufts: { j: number; bone: string; rows: { p: V3; a: number; b: number }[]; side: V3 }[] = [];
    for (const [j, { s, i }] of branched.entries()) {
      const tau = branched.length > 1 ? j / (branched.length - 1) : 0;
      const bone = c.boneAt(s + 0.001);
      const phi = c.spec.phi0 + 180 * j + c.spec.drift * j + 8 * jitter(k, i);
      const T = c.tangent(s);
      const pair = [{ tag: "p", dphi: 0, share: 1, alpha: 52 - 20 * tau }, { tag: "s", dphi: 30 * (j % 2 === 0 ? 1 : -1), share: 0.55, alpha: 60 - 18 * tau }];
      const nodeLeaves: V3[] = [];
      for (const br of pair) {
        if (scale >= 200 && br.tag === "s") continue;
        const rad = c.radial(s, phi + br.dphi);
        const lb = Math.max(0.12, lmax * (1 - 0.68 * Math.pow(tau, 0.9)) * br.share * (1 + 0.08 * jitter(k, i, br.share)));
        const d0 = unit(add(mul(T, Math.cos(br.alpha * D2R)), mul(rad, Math.sin(br.alpha * D2R))));
        const p0 = add(c.at(s + 0.01), mul(rad, c.d(s) * 0.35));
        const droop = 32 + 10 * (1 - tau);
        const db = Math.max(0.0022, Math.min(0.006, 0.2 * c.d(s))) * (br.tag === "s" ? 0.72 : 1);
        const curve = branchCurve(p0, d0, lb, droop, scale === 50 ? 6 : 4);
        const bid = `b${k}-${i}${br.tag}`;
        const pathLen = curve.reduce((acc, p, q) => acc + (q ? len(sub(p, curve[q - 1]!)) : 0), 0);
        const tipR = scale === 50 ? 0.0007 : Math.max(0.0007, db * 0.3);
        const sections = sweep(curve.map((p, q) => ({ p, a: db / 2 + (tipR - db / 2) * (q / (curve.length - 1)) })), UP);
        const branchPart: Part = { id: bid, bone, tube: { id: bid, bone, sections, fields: BRANCH_FIELDS }, fields: BRANCH_FIELDS, region: "culms", shape: `${bone}-branches`, body: scale <= 100, slender: true, role: "detail", centre: curve[3]! };
        if (pathLen / db < 20) throw new Error(`branch ${bid} is not slender: ${pathLen.toFixed(3)} / ${db}`);
        if (scale <= 200) parts.push(branchPart);
        else parts.push({ ...branchPart, body: false });
        const tipDir = unit(sub(curve[curve.length - 1]!, curve[curve.length - 2]!));
        const fans: { q: V3; dir: V3; n: number; twig?: [V3, V3] }[] = [];
        const nTwigs = br.tag === "p" ? Math.max(1, Math.min(6, Math.round(lb / 0.075))) : 2;
        for (let t = 0; t < nTwigs; t++) {
          const u = 0.22 + (0.66 * t) / Math.max(1, nTwigs);
          const qi = Math.min(curve.length - 2, Math.floor(u * (curve.length - 1)));
          const f = u * (curve.length - 1) - qi;
          const q = lin(curve[qi]!, 1 - f, curve[qi + 1]!, f);
          const bt = unit(sub(curve[qi + 1]!, curve[qi]!));
          const sideSign = t % 2 === 0 ? 1 : -1;
          const sideDir = unit(cross(bt, UP));
          const tw = unit(add(add(mul(bt, Math.cos(45 * D2R)), mul(sideDir, sideSign * Math.sin(45 * D2R))), mul(UP, -0.25)));
          const tl = 0.045 + 0.025 * (1 - u) + 0.008 * jitter(k, i, t);
          const end = add(q, mul(tw, tl));
          fans.push({ q: end, dir: tw, n: 3 + (t % 2), twig: [q, end] });
        }
        fans.push({ q: curve[curve.length - 1]!, dir: tipDir, n: br.tag === "p" ? 5 : 4 });
        for (const [fi, fan] of fans.entries()) {
          if (fan.twig && scale === 50) parts.push({ id: `${bid}-t${fi}`, bone, rod: { p0: fan.twig[0], p1: fan.twig[1], r0: 0.0009, r1: 0.0007 }, fields: BRANCH_FIELDS, region: "culms", shape: `${bone}-branches`, body: true, slender: true, role: "detail", centre: fan.twig[1] });
          const L = (0.13 - 0.025 * (1 - tau)) * (0.95 + 0.05 * jitter(k, i, fi)) * Math.pow(c.spec.d0 / 0.029, 0.15);
          const W = 0.0085 * (L / 0.13);
          const leaves: { q: V3; dir: V3; L: number }[] = [];
          for (let li = 0; li < fan.n; li++) {
            const splay = (li - (fan.n - 1) / 2) * (fan.n > 3 ? 21 : 27) + 6 * jitter(k, i, fi, li);
            const droopBase = 28 + 14 * jitter(k, i, fi, li, 1);
            const sideDir = unit(cross(fan.dir, UP));
            let dir = rotateAbout(fan.dir, UP, splay * D2R);
            dir = rotateAbout(dir, sideDir, -droopBase * D2R);
            leaves.push({ q: fan.q, dir: unit(dir), L: L * (1 - 0.08 * Math.abs(li - (fan.n - 1) / 2)) });
          }
          for (const lf of leaves) nodeLeaves.push(add(lf.q, mul(lf.dir, lf.L * 0.55)), add(add(lf.q, mul(lf.dir, lf.L)), [0, -0.02, 0]));
          const blades: { part: Part; arc: { p: V3; n: V3 }[] }[] = [];
          if (scale === 50) {
            for (const [li, lf] of leaves.entries()) {
              const id = `${bid}-f${fi}-l${li}`;
              const { sections: ls, arc } = leafSections(lf.q, lf.dir, lf.L, W, 0.0004, 0.022 * (lf.L / 0.13), 12 * jitter(k, i, fi, li, 2) * D2R);
              const part: Part = { id, bone, tube: { id, bone, sections: ls, fields: BLADE_FIELDS }, fields: BLADE_FIELDS, shape: `${bone}-leaves`, body: true, role: "body", centre: add(lf.q, mul(lf.dir, lf.L / 2)), leafArea: bladeArea(lf.L, W), fan: `${bid}-s${fi}` };
              blades.push({ part, arc });
              parts.push(part);
              leafCount++;
            }
          }
          if (scale <= 100) {
            const mean = unit(leaves.reduce((acc, lf) => add(acc, lf.dir), [0, 0, 0] as V3));
            const spread = fan.n >= 5 ? 0.046 : fan.n === 4 ? 0.04 : 0.034;
            const id = `${bid}-s${fi}`;
            const length = L * (scale === 50 ? 0.9 : 1.02);
            const splayed = 0.8 * (L * Math.sin(((fan.n - 1) / 2) * (fan.n > 3 ? 21 : 27) * D2R) + W);
            const { sections: ls, arc } = scale === 50 ? leafSections(fan.q, mean, length * 1.12, spread * 0.8, Math.max(0.0079, feature * 0.53), 0.05, 0, 6) : leafSections(fan.q, mean, L, splayed, Math.max(0.0079, feature * 0.53), 0.045, 0, 8, FAN);
            const part: Part = { id, bone, tube: { id, bone, sections: ls.map((x, q) => ({ ...x, a: Math.max(x.a, q === 0 ? 0.01 : x.a) })), fields: LEAF_FIELDS }, fields: LEAF_FIELDS, shape: `${bone}-leaves`, body: scale === 100, proxy: scale === 50, role: "body", centre: add(fan.q, mul(mean, length / 2)), leafArea: bladeArea(L, W) * fan.n };
            parts.push(part);
            foliage.push({ part, culm: k, score: 0, arc, leaves: blades });
          }
        }
        if (scale >= 200) {
          const id = `${bid}-tuft`;
          const reachEnd = add(curve[curve.length - 1]!, mul(unit(add(tipDir, mul(UP, -0.7))), 0.11));
          const pts = [...curve.slice(1), reachEnd];
          const n = pts.length;
          const rows = pts.map((p, q) => {
            const t = q / (n - 1);
            const a = Math.max(0.0155, 0.012 + 0.1 * Math.sin(Math.PI * Math.min(1, t * 0.95 + 0.05)) * (lb / 0.5));
            return { p: add(p, mul(UP, -0.03 * t)), a, b: Math.max(0.0158, feature * 0.53, 0.4 * a) };
          });
          if (scale === 500) {
            tufts.push({ j, bone, rows, side: unit(cross(d0, UP)) });
            continue;
          }
          const sec = sweep(rows, unit(cross(d0, UP)));
          sec[sec.length - 1] = { ...sec[sec.length - 1]!, a: 0.016 };
          const part: Part = { id, bone, tube: { id, bone, sections: sec, fields: LEAF_FIELDS }, fields: LEAF_FIELDS, shape: `${bone}-leaves`, body: true, role: "body", centre: curve[Math.floor(n * 0.6)]!, leafArea: 0 };
          parts.push(part);
          foliage.push({ part, culm: k, score: 0, arc: sec.map((x) => ({ p: x.p, n: x.y[1] >= 0 ? x.y : mul(x.y, -1) })) });
        }
      }
      masses.push({ bone, pts: nodeLeaves });
    }

    {
      const bone = c.bones.f;
      const tipT = c.tangent(c.L - 0.01);
      const q = c.at(c.L - 0.004);
      const rise = unit(add(mul(horiz(tipT), Math.cos(28 * D2R)), mul(UP, Math.sin(28 * D2R))));
      const L = 0.12 * Math.pow(c.spec.d0 / 0.029, 0.15);
      const W = 0.0085 * (L / 0.13);
      const dirs = [-34, 0, 34].map((sp, li) => rotateAbout(rise, UP, (sp + 5 * jitter(k, 99, li)) * D2R));
      const apexPts = dirs.map((d) => add(q, mul(d, L * 0.6)));
      masses.push({ bone, pts: apexPts });
      const blades: { part: Part; arc: { p: V3; n: V3 }[] }[] = [];
      if (scale === 50) for (const [li, d] of dirs.entries()) {
        const id = `apex${k}-l${li}`;
        const { sections: ls, arc } = leafSections(q, d, L, W, 0.0004, 0.03, 10 * jitter(k, 98, li) * D2R);
        const part: Part = { id, bone, tube: { id, bone, sections: ls, fields: BLADE_FIELDS }, fields: BLADE_FIELDS, shape: `${bone}-leaves`, body: true, role: "body", centre: add(q, mul(d, L / 2)), leafArea: bladeArea(L, W), fan: `apex${k}-s` };
        blades.push({ part, arc });
        parts.push(part);
        leafCount++;
      }
      if (scale <= 100) {
        const id = `apex${k}-s`;
        const { sections: ls, arc } = leafSections(q, rise, L, 0.05, Math.max(0.0079, feature * 0.53), 0.03, 0, 6);
        const part: Part = { id, bone, tube: { id, bone, sections: ls.map((x, n) => ({ ...x, a: Math.max(x.a, n === 0 ? 0.01 : x.a) })), fields: LEAF_FIELDS }, fields: LEAF_FIELDS, shape: `${bone}-leaves`, body: scale === 100, proxy: scale === 50, role: "body", centre: add(q, mul(rise, L / 2)), leafArea: bladeArea(L, W) * 3 };
        parts.push(part);
        foliage.push({ part, culm: k, score: 0, arc, leaves: blades });
      }
      if (scale >= 200) {
        const id = `apex${k}-tuft`;
        const sec = sweep([0, 0.35, 0.7, 1].map((t, n) => ({ p: add(add(q, mul(rise, L * t)), mul(UP, -0.012 * t * t)), a: Math.max([0.016, 0.035, 0.03, 0.016][n]!, feature * 0.5), b: Math.max(0.0158, feature * 0.53) })), UP);
        const part: Part = { id, bone, tube: { id, bone, sections: sec, fields: LEAF_FIELDS }, fields: LEAF_FIELDS, shape: `${bone}-leaves`, body: true, role: "body", centre: add(q, mul(rise, L / 2)), leafArea: 0 };
        parts.push(part);
        foliage.push({ part, culm: k, score: 0, arc: sec.map((x) => ({ p: x.p, n: x.y[1] >= 0 ? x.y : mul(x.y, -1) })) });
      }
    }

    if (scale === 500) {
      for (let g = 0; g < tufts.length; g += 4) for (const parity of [0, 1]) {
        const group = [tufts[g + parity], tufts[g + parity + 2]].filter((x): x is (typeof tufts)[number] => x !== undefined);
        if (!group.length) continue;
        const n = Math.min(...group.map((x) => x.rows.length));
        const rows = Array.from({ length: n }, (_, q) => {
          const ps = group.map((x) => x.rows[Math.round((q * (x.rows.length - 1)) / (n - 1))]!);
          const p = mul(ps.reduce((acc, r) => add(acc, r.p), [0, 0, 0] as V3), 1 / ps.length);
          const sep = Math.max(...ps.map((r) => len(sub(r.p, p))));
          const a = Math.max(0.04, Math.max(...ps.map((r) => r.a)) + sep * 0.55);
          return { p, a, b: Math.max(0.0395, 0.45 * a) };
        });
        rows[rows.length - 1] = { ...rows[rows.length - 1]!, a: 0.04 };
        const side = unit(group.reduce((acc, x) => add(acc, x.side), [0, 0, 0] as V3));
        const first = group[0]!;
        const id = `fol-${k}-plume-${first.j + 1}`;
        const sec = sweep(rows, len(side) > 0.1 ? side : group[0]!.side);
        const part: Part = { id, bone: first.bone, tube: { id, bone: first.bone, sections: sec, fields: LEAF_FIELDS }, fields: LEAF_FIELDS, shape: `${first.bone}-leaves`, body: true, role: "body", centre: rows[Math.floor(n * 0.6)]!.p, leafArea: 0 };
        parts.push(part);
        foliage.push({ part, culm: k, score: 0, arc: sec.map((x) => ({ p: x.p, n: x.y[1] >= 0 ? x.y : mul(x.y, -1) })) });
      }
    }
  }

  const R = Math.max(...foliage.map((f) => Math.hypot(f.part.centre[0] - centroid[0], f.part.centre[2] - centroid[2])));
  const Hmax = Math.max(...foliage.map((f) => f.part.centre[1]));
  for (const f of foliage) {
    const r = Math.hypot(f.part.centre[0] - centroid[0], f.part.centre[2] - centroid[2]);
    f.score = 0.72 * (f.part.centre[1] / Hmax) + 0.28 * (r / Math.max(1e-6, R));
  }
  const sorted = [...foliage].sort((a, b) => b.score - a.score);
  const weights = sorted.map((f) => (scale === 50 || scale === 100 ? f.part.leafArea ?? 1 : 1));
  const total = weights.reduce((a, b) => a + b, 0);
  let acc = 0;
  for (const [i, f] of sorted.entries()) {
    const sun = acc < total * (5.4 / 13.6);
    acc += weights[i]!;
    const region = sun ? "leaves-sun" : "leaves-shade";
    f.part.region = region;
    f.part.shape = `${f.part.bone}-${region}`;
    f.part.fields = { ...f.part.fields, tone: region === "leaves-sun" ? 0.66 : 0.5 };
    if (f.part.tube) f.part.tube.fields = f.part.fields;
    if (f.part.blob) f.part.blob.fields = f.part.fields;
  }

  const fans = new Map(foliage.map((f) => [f.part.id, f.part]));
  for (const p of parts) if (p.fan) {
    const fan = fans.get(p.fan)!;
    p.region = fan.region;
    p.shape = `${p.bone}-${fan.region}`;
    p.fields = { ...p.fields, tone: fan.fields.tone };
    p.tube!.fields = p.fields;
  }

  for (const c of culms) {
    const k = c.spec.k;
    const mine = foliage.filter((f) => f.culm === k && f.part.bone === c.bones.f);
    for (const [kind, emitter, pick] of [["sun", "transpiration-sun", (a: Foliage, b: Foliage) => b.score - a.score], ["shade", "transpiration-shade", (a: Foliage, b: Foliage) => a.score - b.score]] as const) {
      const pool = mine.filter((f) => f.part.region === `leaves-${kind}`);
      const host = [...(pool.length ? pool : mine)].sort(pick)[0];
      if (!host) throw new Error(`culm ${k}: no foliage on ${c.bones.f} for the ${kind} outlet`);
      const blade = host.leaves?.length ? host.leaves[Math.floor(host.leaves.length / 2)]! : undefined;
      const arc = blade ? blade.arc : host.arc;
      const part25 = arc.slice(Math.floor(arc.length * 0.25), Math.max(Math.floor(arc.length * 0.25) + 2, Math.ceil(arc.length * 0.8)));
      const mid = len(sub(part25[part25.length - 1]!.p, part25[0]!.p)) < feature * 1.1 ? arc : part25;
      const hostHalf = blade ? 0.0004 : host.part.tube ? Math.max(...host.part.tube.sections.map((s) => s.b)) : host.part.blob!.radii[2];
      const patchHalf = Math.max(0.0035, feature * 0.52);
      const lift = host.part.blob ? 0 : hostHalf;
      const pts = mid.map((x) => ({ p: add(x.p, mul(x.n, lift)), a: scale === 50 ? 0.0045 : Math.max(0.02, feature * 0.6), b: patchHalf }));
      let sections: Section[];
      if (host.part.blob) {
        const b = host.part.blob;
        const top = add(b.c, mul(UP, b.radii[1] * 0.9));
        sections = sweep([{ p: add(top, [-0.04, 0, 0]), a: Math.max(0.03, feature * 0.6), b: patchHalf }, { p: add(top, [0.04, 0, 0]), a: Math.max(0.03, feature * 0.6), b: patchHalf }], UP);
      } else sections = sweep(pts, UP).map((s, q) => ({ ...s, y: mid[q]!.n, x: perp(s.x, unit(cross(mid[q]!.n, s.x))) }));
      sections = sections.map((s) => {
        const t = unit(cross(s.x, s.y));
        const y = perp(s.y, t);
        return { ...s, y, x: unit(cross(y, t)) };
      });
      const id = `th-tp-${kind}-${k}`;
      const centre = sections[Math.floor(sections.length / 2)]!;
      const part: Part = { id, bone: c.bones.f, tube: { id, bone: c.bones.f, sections, fields: { material: "bamboo-leaf", role: "detail", tone: 0.5 } }, fields: { material: "bamboo-leaf", role: "detail", tone: 0.5 }, region: `leaves-${kind}` as Part["region"], body: false, role: "detail", centre: centre.p };
      emitters.push({ id, anchor: `tp-${kind}-${k}`, emitter, bone: c.bones.f, part, anchorPoint: add(centre.p, mul(centre.y[1] >= 0 ? centre.y : mul(centre.y, -1), patchHalf + 0.003)) });
    }
  }
  return { culms, parts, foliage, emitters, leafCount };
}
