/** 🩻 The axial skeleton of the round 4 review group G06 (`humans/walking-commuter`, `humans/jogger`, `humans/running-child`), stated once as the table of an adult man (torso bone 0.51 m from L5/S1 to C7/T1, hip joints 0.09 m off the midline) and fitted to each figure by its own stated factors.
 * Frame of every statement: bone-local `[s, v, w]` with s along the trunk bone (up), v towards the back, w to the figure's left. Every vertebral level, rib, border and ramus is written down by hand below; the helpers only place those statements.
 * Scales: 1:50 all twelve ribs with costal cartilages, spinous processes, shoulder blade with spine and acromion, hip bone with wing, pubic and ischial rami; 1:100 six rib pairs; 1:200 three rib pairs; 1:500 the cage and the basin as masses on a slender spine.
 * @see ./r4-review-brief.md
 * @see https://en.wikipedia.org/wiki/Rib_cage
 * @see https://en.wikipedia.org/wiki/Pelvis */
export type Json = Record<string, any>;
export type V3 = readonly [number, number, number];
type Q4 = [number, number, number, number];

/** 📐 How the adult table fits one figure: `ks` along the trunk, `kv` in depth, `kw` in width, `dv` a shift of the thorax in depth, `kp` and `kpv` for the pelvis (height and width, depth), `drop` the share of the adult rib slope (a child's ribs lie flatter), `shoulder` the s of the shoulder joint in the torso bone (the girdle hangs on it; a child's shoulders sit lower under the seventh cervical vertebra than a man's). */
export type Fit = { material: string; tone: number; ks: number; kv: number; kw: number; dv: number; kp: number; kpv: number; drop: number; shoulder: number; torso: string; pelvis: string };
export type Scale = 50 | 100 | 200 | 500;

const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const MIN: Record<Scale, number> = { 50: 0.00375, 100: 0.0075, 200: 0.015, 500: 0.0375 };

function quatOf(x: V3, y: V3, z: V3): Q4 {
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]];
  const t = m00 + m11 + m22;
  let q: Q4;
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
  return q.map((c) => Math.round(c * 1e6) / 1e6) as Q4;
}
/** 🧭 The rotation of a plate section that stands across the trunk bone: its long axis lies `deg` degrees from the forward direction towards the side `g`, its thin axis across it. */
const plate = (deg: number, g: number): Q4 => {
  const a = (deg * Math.PI) / 180;
  const y: V3 = [0, -Math.cos(a), g * Math.sin(a)];
  const x: V3 = [0, g * Math.sin(a), Math.cos(a)];
  return quatOf(x, y, [1, 0, 0]);
};

/** 🦴 The vertebral column from L5/S1 to C7/T1: `[s, v, radius]` of the vertebral bodies; lumbar lordosis forward, thoracic kyphosis back. */
const SPINE: readonly V3[] = [[-0.012, 0.04, 0.024], [0.04, 0.026, 0.024], [0.09, 0.02, 0.023], [0.15, 0.027, 0.022], [0.22, 0.044, 0.02], [0.3, 0.06, 0.018], [0.38, 0.063, 0.016], [0.45, 0.052, 0.015], [0.515, 0.034, 0.014]];
/** 🪥 The levels of the spinous processes from L5 to T1 (s of the vertebra). */
const PROCESS: readonly number[] = [0.012, 0.048, 0.084, 0.12, 0.156, 0.2, 0.227, 0.254, 0.28, 0.307, 0.334, 0.361, 0.388, 0.414, 0.441, 0.468, 0.495];
/** 🫁 The twelve ribs: `[number, s of the head at the vertebra, half width, v of the most forward point, drop to the front end, kind]` and for a true rib the s of its sternal joint, for a false rib the point where its cartilage meets the costal arch, for a floating rib the share of the half hoop it covers. */
type Rib = readonly [n: number, s: number, w: number, vFront: number, drop: number, end: number | V3, floating?: boolean];
const RIBS: readonly Rib[] = [
  [1, 0.495, 0.055, -0.03, 0.03, 0.425],
  [2, 0.468, 0.085, -0.055, 0.05, 0.388],
  [3, 0.441, 0.104, -0.074, 0.065, 0.362],
  [4, 0.414, 0.118, -0.086, 0.075, 0.338],
  [5, 0.388, 0.127, -0.094, 0.085, 0.315],
  [6, 0.361, 0.133, -0.098, 0.095, 0.296],
  [7, 0.334, 0.136, -0.098, 0.105, 0.28],
  [8, 0.307, 0.137, -0.094, 0.105, [0.252, -0.09, 0.05]],
  [9, 0.28, 0.135, -0.086, 0.1, [0.232, -0.08, 0.075]],
  [10, 0.254, 0.128, -0.074, 0.09, [0.208, -0.064, 0.098]],
  [11, 0.227, 0.114, -0.05, 0.06, 0.56, true],
  [12, 0.2, 0.09, -0.02, 0.04, 0.42, true],
];
/** 🩹 The breastbone from the xiphoid tip to the jugular notch: `[s, v, half width, half thickness]`. */
const STERNUM: readonly (readonly [number, number, number, number])[] = [[0.24, -0.094, 0.006, 0.004], [0.262, -0.096, 0.011, 0.005], [0.276, -0.096, 0.02, 0.006], [0.33, -0.09, 0.019, 0.006], [0.384, -0.078, 0.017, 0.007], [0.392, -0.076, 0.024, 0.008], [0.42, -0.068, 0.031, 0.008], [0.436, -0.062, 0.027, 0.007]];
/** 🦯 The left collar bone from the breastbone to the acromion: convex forward at the breastbone, concave at the shoulder. */
const CLAVICLE: readonly V3[] = [[0.432, -0.06, 0.02], [0.445, -0.072, 0.066], [0.458, -0.054, 0.114], [0.467, -0.02, 0.152], [0.47, 0.004, 0.172]];
/** 🪽 The blade of the left shoulder blade from the inferior to the superior angle: `[s, v, w of the middle, half width]`; the blade lies 32 degrees out of the frontal plane on the ribs. */
const BLADE: readonly (readonly [number, number, number, number])[] = [[0.272, 0.07, 0.092, 0.011], [0.305, 0.074, 0.098, 0.028], [0.35, 0.074, 0.107, 0.042], [0.395, 0.068, 0.113, 0.05], [0.428, 0.064, 0.107, 0.04], [0.442, 0.062, 0.1, 0.022]];
const BLADE_TURN = 32;
/** ⛰️ The spine of the left shoulder blade from its root at the medial border to the acromion over the shoulder joint. */
const SCAPULAR_SPINE: readonly V3[] = [[0.405, 0.088, 0.072], [0.428, 0.078, 0.112], [0.452, 0.056, 0.15], [0.47, 0.022, 0.178], [0.47, 0, 0.184]];
const GLENOID: V3 = [0.4447, 0.016, 0.152];
const CORACOID: readonly [V3, V3] = [[0.452, 0.012, 0.148], [0.446, -0.03, 0.142]];

/** 🪑 The sacrum and coccyx in the pelvis bone: `[s, v, half width, half thickness]`. */
const SACRUM: readonly (readonly [number, number, number, number])[] = [[0.096, 0.042, 0.05, 0.02], [0.07, 0.06, 0.05, 0.018], [0.036, 0.075, 0.038, 0.014], [0.004, 0.08, 0.023, 0.011], [-0.024, 0.072, 0.011, 0.0075]];
/** 🦋 The left hip bone above the socket, from the body at the socket to the crest: `[s, v, w of the middle, half length, half thickness, turn of the long axis from the forward direction towards the side]`. */
const ILIUM: readonly (readonly [number, number, number, number, number, number])[] = [[0.01, 0.002, 0.092, 0.03, 0.013, 8], [0.045, 0.014, 0.09, 0.04, 0.008, 16], [0.085, 0.02, 0.092, 0.062, 0.006, 26], [0.118, 0.019, 0.094, 0.076, 0.006, 30], [0.134, 0.017, 0.095, 0.07, 0.0065, 32], [0.143, 0.016, 0.095, 0.054, 0.006, 32], [0.147, 0.016, 0.095, 0.03, 0.005, 32]];
const PUBIS: readonly V3[] = [[0, -0.03, 0.074], [-0.018, -0.06, 0.04], [-0.03, -0.076, 0.008]];
const ISCHIUM: readonly V3[] = [[-0.012, 0.012, 0.082], [-0.05, 0.034, 0.07], [-0.082, 0.04, 0.06]];
const TUBEROSITY: readonly [V3, V3] = [[-0.086, 0.04, 0.059], [0.017, 0.02, 0.014]];
const RAMUS: readonly V3[] = [[-0.088, 0.03, 0.054], [-0.072, -0.03, 0.03], [-0.046, -0.072, 0.009]];

const lerp = (a: number, b: number, t: number): number => a + (b - a) * t;
function spineAt(s: number): [number, number] {
  for (let i = 0; i + 1 < SPINE.length; i++) {
    const [a, b] = [SPINE[i]!, SPINE[i + 1]!];
    if (s <= b[0] || i + 2 === SPINE.length) {
      const t = (s - a[0]) / (b[0] - a[0]);
      return [lerp(a[1], b[1], t), lerp(a[2], b[2], t)];
    }
  }
  return [SPINE[0]![1], SPINE[0]![2]];
}

/** 🏗️ The builder of one figure at one scale. */
export class Skeleton {
  readonly solids: Json[] = [];
  private readonly min: number;
  private readonly kr: number;
  constructor(readonly fit: Fit, readonly scale: Scale) {
    this.min = MIN[scale];
    this.kr = (fit.kv + fit.kw) / 2;
  }
  private t(p: V3): number[] {
    return [r5(p[0] * this.fit.ks), r5(p[1] * this.fit.kv + this.fit.dv), r5(p[2] * this.fit.kw)];
  }
  private g(p: V3): number[] {
    return [r5(this.fit.shoulder + (p[0] - GLENOID[0]) * this.fit.ks), r5(p[1] * this.fit.kv + this.fit.dv), r5(p[2] * this.fit.kw)];
  }
  private p(p: V3): number[] {
    return [r5(p[0] * this.fit.kp), r5(p[1] * this.fit.kpv), r5(p[2] * this.fit.kp)];
  }
  private r(radius: number): number {
    return r5(Math.max(radius * this.kr, this.min));
  }
  private head(id: string, bone: string): Json {
    return { id, kind: "loft", bone, material: this.fit.material, role: "body", tone: this.fit.tone };
  }
  private tube(id: string, bone: string, path: readonly number[][], radii: readonly number[], extra: Json = {}): void {
    this.solids.push({ ...this.head(id, bone), sections: path.map((at, i) => ({ at, rx: radii[Math.min(i, radii.length - 1)]!, ry: radii[Math.min(i, radii.length - 1)]! })), capStart: true, capEnd: true, ...extra });
  }
  private ball(id: string, bone: string, at: number[], radii: number[]): void {
    this.solids.push({ id, kind: "ellipsoid", bone, material: this.fit.material, role: "body", tone: this.fit.tone, radii, transform: { translation: at } });
  }

  /** 🦴 The column with its two curves and, at 1:50, the row of spinous processes under the skin of the back. */
  spine(): this {
    const f = this.fit;
    const rows = this.scale === 50 ? SPINE : this.scale === 100 ? [0, 1, 2, 3, 4, 5, 6, 7, 8].map((i) => SPINE[i]!) : [0, 2, 4, 6, 8].map((i) => SPINE[i]!);
    if (this.scale === 500) {
      const length = (SPINE[SPINE.length - 1]![0] - SPINE[0]![0]) * f.ks;
      this.tube("sk-spine", f.torso, SPINE.map(([s, v]) => this.t([s, v, 0])), [r5(Math.min(0.02 * this.kr, length / 42))], { slender: true });
      return this;
    }
    this.tube("sk-spine", f.torso, rows.map(([s, v]) => this.t([s, v, 0])), rows.map((q) => this.r(q[2])));
    if (this.scale === 50)
      PROCESS.forEach((s, i) => {
        const [v, r] = spineAt(s);
        const lumbar = i < 5;
        const base = this.t([s, v + r * 0.8, 0]);
        const tip = this.t([s - (lumbar ? 0.004 : 0.016), v + r + (lumbar ? 0.026 : 0.02), 0]);
        this.solids.push({ id: `sk-process-${i < 5 ? `l${5 - i}` : `t${17 - i}`}`, kind: "capsule", bone: f.torso, material: f.material, role: "body", tone: f.tone, p0: base, p1: tip, r0: this.r(0.005), r1: this.r(0.004) });
      });
    return this;
  }

  private ribPath(rib: Rib, g: number): number[][] {
    const [, s, w, vFront, drop0, end, floating] = rib;
    const drop = drop0 * this.fit.drop;
    const [vSpine, rSpine] = spineAt(s);
    const vBack = vSpine + 0.03;
    const [vMid, rv] = [(vBack + vFront) / 2, (vBack - vFront) / 2];
    const at = (th: number): V3 => [s - (drop * (1 - Math.cos(th))) / 2, vMid + rv * Math.cos(th), g * w * Math.sin(th)];
    const last = floating ? (end as number) * Math.PI : typeof end === "number" ? 0.8 * Math.PI : 0.72 * Math.PI;
    const first = this.scale === 50 ? 0.32 : 0.2;
    const steps = floating ? 5 : this.scale === 200 ? 7 : 9;
    const path: V3[] = [];
    if (this.scale === 50) path.push([s, vSpine + rSpine * 0.5, g * (rSpine + 0.004)]);
    for (let i = 0; i <= steps; i++) path.push(at(first + ((last - first) * i) / steps));
    if (!floating) {
      const tip: V3 = typeof end === "number" ? [end, lerp(STERNUM[2]![1], STERNUM[7]![1], (end - STERNUM[2]![0]) / (STERNUM[7]![0] - STERNUM[2]![0])), g * 0.014] : [end[0], end[1], g * end[2]];
      const from = path[path.length - 1]!;
      path.push([lerp(from[0], tip[0], 0.5) - 0.006 * this.fit.drop, lerp(from[1], tip[1], 0.5) - 0.004, lerp(from[2], tip[2], 0.5)], tip);
    }
    return path.map((q) => this.t(q));
  }
  /** 🫁 The rib cage: every rib of the scale as a bent bar from its head at the vertebra round the side and down to the front, its cartilage rising to the breastbone or to the costal arch; the breastbone with manubrium, body and xiphoid. */
  cage(): this {
    const f = this.fit;
    const pick = this.scale === 50 ? RIBS : this.scale === 100 ? [0, 2, 4, 6, 8, 10].map((i) => RIBS[i]!) : (f.kw < 0.8 ? [1, 5] : [1, 3, 6]).map((i) => RIBS[i]!);
    for (const rib of pick)
      for (const [sfx, g] of [["l", 1], ["r", -1]] as const) this.tube(`sk-rib-${rib[0]}-${sfx}`, f.torso, this.ribPath(rib, g), [this.r(0.0052)]);
    const rows = this.scale === 50 ? STERNUM : this.scale === 100 ? [1, 2, 4, 5, 6, 7].map((i) => STERNUM[i]!) : [2, 4, 6, 7].map((i) => STERNUM[i]!);
    this.solids.push({ ...this.head("sk-sternum", f.torso), sections: rows.map(([s, v, hw, ht]) => ({ at: this.t([s, v, 0]), rx: r5(Math.max(hw * f.kw, this.min)), ry: this.r(ht), exponent: 2.4, rotation: [0, -0.70711, 0, 0.70711] })), capStart: true, capEnd: true });
    return this;
  }

  /** 🪽 The shoulder girdle of both sides: collar bone, and the shoulder blade as a thin blade on the back of the ribs with its spine running out to the acromion, the socket and (1:50) the coracoid. */
  girdle(): this {
    const f = this.fit;
    for (const [sfx, g] of [["l", 1], ["r", -1]] as const) {
      const side = (q: V3): V3 => [q[0], q[1], g * q[2]];
      this.tube(`sk-clavicle-${sfx}`, f.torso, CLAVICLE.map((q, i) => { const [a, b] = [this.t(side(q)), this.g(side(q))]; return [r5(lerp(a[0]!, b[0]!, i / (CLAVICLE.length - 1))), a[1]!, a[2]!]; }), [this.r(0.0075), this.r(0.0065), this.r(0.006), this.r(0.007), this.r(0.008)]);
      this.solids.push({
        ...this.head(`sk-scapula-${sfx}`, f.torso),
        sections: BLADE.map(([s, v, w, hw]) => ({ at: this.g([s, v, g * w]), rx: r5(Math.max(0.0042 * this.kr, this.min)), ry: r5(Math.max(hw * f.kw, this.min)), exponent: 2.6, rotation: plate(90 - BLADE_TURN, g) })),
        capStart: true,
        capEnd: true,
      });
      if (this.scale === 200) continue;
      this.tube(`sk-scapular-spine-${sfx}`, f.torso, SCAPULAR_SPINE.map((q) => this.g(side(q))), [this.r(0.005), this.r(0.006), this.r(0.007), this.r(0.008), this.r(0.008)]);
      if (this.scale !== 50) continue;
      this.ball(`sk-glenoid-${sfx}`, f.torso, this.g(side(GLENOID)), [this.r(0.017), this.r(0.008), this.r(0.012)]);
      this.solids.push({ id: `sk-coracoid-${sfx}`, kind: "capsule", bone: f.torso, material: f.material, role: "body", tone: f.tone, p0: this.g(side(CORACOID[0])), p1: this.g(side(CORACOID[1])), r0: this.r(0.007), r1: this.r(0.0055) });
    }
    return this;
  }

  /** 🪑 The pelvic ring: sacrum with coccyx, two hip bones with their wings up to the crests, the pubic rami meeting at the symphysis, the ischia with their tuberosities and (1:50, 1:100) the lower rami that close the obturator openings. */
  pelvis(): this {
    const f = this.fit;
    const rows = this.scale === 200 ? [0, 2, 4].map((i) => SACRUM[i]!) : SACRUM;
    this.solids.push({ ...this.head("sk-sacrum", f.pelvis), sections: rows.map(([s, v, hw, ht]) => ({ at: this.p([s, v, 0]), rx: r5(Math.max(hw * f.kp, this.min)), ry: r5(Math.max(ht * f.kpv, this.min)), exponent: 2.3, rotation: [0, -0.70711, 0, 0.70711] })), capStart: true, capEnd: true });
    for (const [sfx, g] of [["l", 1], ["r", -1]] as const) {
      const side = (q: V3): V3 => [q[0], q[1], g * q[2]];
      this.solids.push({
        ...this.head(`sk-ilium-${sfx}`, f.pelvis),
        sections: ILIUM.map(([s, v, w, hl, ht, turn]) => ({ at: this.p([s, v, g * w]), rx: r5(Math.max(ht * f.kp, this.min)), ry: r5(Math.max(hl * f.kpv, this.min)), exponent: 2.5, rotation: plate(turn, g) })),
        capStart: true,
        capEnd: true,
      });
      const pr = (x: number): number => r5(Math.max(x * f.kp, this.min));
      this.tube(`sk-pubis-${sfx}`, f.pelvis, PUBIS.map((q) => this.p(side(q))), [pr(0.012), pr(0.0105), pr(0.011)]);
      this.tube(`sk-ischium-${sfx}`, f.pelvis, ISCHIUM.map((q) => this.p(side(q))), [pr(0.014), pr(0.013), pr(0.014)]);
      if (this.scale === 200) continue;
      this.ball(`sk-tuberosity-${sfx}`, f.pelvis, this.p(side(TUBEROSITY[0])), TUBEROSITY[1].map((x) => pr(x)));
      this.tube(`sk-ischiopubic-ramus-${sfx}`, f.pelvis, RAMUS.map((q) => this.p(side(q))), [pr(0.009), pr(0.0075), pr(0.0085)]);
    }
    return this;
  }

  /** 🥚 1:500: the rib cage as one egg that is narrow at the first rib, widest at the eighth and open towards the belly, and the pelvis as one basin from the crests down to the ischia; both hang on the slender spine. */
  masses(): this {
    const f = this.fit;
    const cage: readonly (readonly [number, number, number, number])[] = [[0.172, -0.002, 0.07, 0.05], [0.19, -0.004, 0.108, 0.078], [0.25, -0.01, 0.136, 0.094], [0.33, -0.012, 0.132, 0.09], [0.42, -0.004, 0.098, 0.066], [0.488, 0.012, 0.054, 0.0375], [0.5, 0.014, 0.0375, 0.0375]];
    this.solids.push({ ...this.head("sk-ribcage", f.torso), sections: cage.map(([s, v, hw, hd]) => ({ at: this.t([s, v, 0]), rx: r5(Math.max(hw * f.kw, this.min)), ry: r5(Math.max(hd * f.kv, this.min)), exponent: 2.2, rotation: [0, -0.70711, 0, 0.70711] })), capStart: true, capEnd: true });
    const basin: readonly (readonly [number, number, number, number])[] = [[-0.1, 0.014, 0.0375, 0.0375], [-0.084, 0.012, 0.07, 0.05], [-0.03, 0.004, 0.1, 0.072], [0.04, 0.014, 0.118, 0.07], [0.105, 0.02, 0.14, 0.066], [0.136, 0.02, 0.134, 0.058], [0.147, 0.02, 0.1, 0.0375]];
    this.solids.push({ ...this.head("sk-pelvis", f.pelvis), sections: basin.map(([s, v, hw, hd]) => ({ at: this.p([s, v, 0]), rx: r5(Math.max(hw * f.kp, this.min)), ry: r5(Math.max(hd * f.kpv, this.min)), exponent: 2.4, rotation: [0, -0.70711, 0, 0.70711] })), capStart: true, capEnd: true });
    return this;
  }
}

/** 🧱 The axial skeleton of one figure at one scale: spine, rib cage, shoulder girdle and pelvic ring (1:50 to 1:200) or the spine with the cage and basin masses (1:500). */
export function axial(fit: Fit, scale: Scale): Json[] {
  const k = new Skeleton(fit, scale);
  if (scale === 500) return k.spine().masses().solids;
  return k.spine().cage().girdle().pelvis().solids;
}

/** 💀 The facial skeleton and the lower jaw in the head bone: the face as one rounded mass under the cranium, the jaw as two rami from the joints in front of the ears down to the angles and two halves of the body meeting at the chin. `face` is `[centre, radii]`, the jaw points are those of the left side. */
export function face(fit: Fit, bone: string, scale: Scale, faceAt: readonly [V3, V3], condyle: V3, angle: V3, chin: V3, bar: number): Json[] {
  const min = MIN[scale];
  const out: Json[] = [{ id: "sk-face", kind: "ellipsoid", bone, material: fit.material, role: "body", tone: fit.tone, radii: faceAt[1].map(r5), transform: { translation: faceAt[0].map(r5) } }];
  const r = r5(Math.max(bar, min));
  for (const [sfx, g] of [["l", 1], ["r", -1]] as const) {
    const side = (q: V3): number[] => [r5(q[0]), r5(q[1]), r5(g * q[2])];
    out.push({ id: `sk-mandible-ramus-${sfx}`, kind: "capsule", bone, material: fit.material, role: "body", tone: fit.tone, p0: side(condyle), p1: side(angle), r0: r, r1: r });
    out.push({ id: `sk-mandible-body-${sfx}`, kind: "capsule", bone, material: fit.material, role: "body", tone: fit.tone, p0: side(angle), p1: side(chin), r0: r, r1: r5(r * 1.1) });
  }
  return out;
}

/** 🥢 A long bone at 1:500 as a slender rod of its real thickness, never thicker than a twentieth of its length. */
export function rod(fit: Fit, id: string, bone: string, p0: V3, p1: V3, radius: number): Json {
  const length = Math.hypot(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);
  const r = r5(Math.min(radius, length / 42));
  return { id, kind: "capsule", bone, material: fit.material, role: "body", tone: fit.tone, p0: p0.map(r5), p1: p1.map(r5), r0: r, r1: r, slender: true };
}
