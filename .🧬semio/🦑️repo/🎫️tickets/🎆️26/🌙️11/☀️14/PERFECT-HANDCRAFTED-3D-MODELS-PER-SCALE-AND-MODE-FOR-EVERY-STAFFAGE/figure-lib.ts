/** 🧍️ Shared anatomical figure kit: loft templates per body part (head, neck, thorax, pelvis, upper arm, forearm, hand, thigh, shank, foot, toe) whose sections follow real anatomy, fitted to one asset's own rig bone lengths and to the 2D outline of one scale; garments, hair, shoes, structure (skeleton load path) and thermal shells are built on the same stations.
 *
 * Anthropometric sources (values rounded, 50th percentile, tabulated at stature 1.75 m male / 1.62 m female and scaled by stature): DIN 33402-2:2005 (body dimensions of adults, percentiles), ISO 7250-1:2017 (basic human body measurements), Dreyfuss Associates, "The Measure of Man and Woman" (2002: breadths, depths, hand, foot, head), Winter, "Biomechanics and Motor Control of Human Movement" (2009, table 4.1: segment lengths as fractions of stature, Drillis-Contini 1966), de Leva (1996, segment masses and centres), Gray's Anatomy (long-bone shaft diameters: femur 28-32 mm, tibia 25-30 mm, humerus 22-26 mm, radius+ulna 2 x 12-14 mm). The numbers were tabulated from memory of those publications; `figure.md` (v2, anthropometry) states which of them were cross-checked against the catalog's own 28 humans and DuBois (verified) and which remain recollection; they set shape (ratios, exponents, centre offsets) and size where no drawing exists, the drawn outline of the scale overrules them where it exists.
 * Frames (A1): bone-local u along the bone, v sagittal, w lateral; a loft section stands in the plane (w, v), `rx` lateral half extent, `ry` sagittal half extent, centre `at = [s, cv, cw]`.
 * @see ./figure.md
 * @see ./kit-lib.ts */
import { drawn, featureM, loftOf, selectStations, shellOffset, type Ctx, type Json, type Section, type Station, type View } from "./kit-lib.ts";

export type V3 = [number, number, number];
export type Quat = [number, number, number, number];
export type Person = { sex: "male" | "female"; age: number; build: number; pregnant: number; stoop: number; statureM: number };
export type Config = Partial<Omit<Person, "statureM">> & { statureM?: number; gap?: Record<string, number>; skip?: string[]; keep?: string[]; keepRegions?: string[]; hairThicknessM?: number; shoeSoleM?: number; notes?: string };
export type Row = [t: number, w: number, d: number, ac: number, e: number];
export type PartName = "head" | "neck" | "torso" | "pelvis" | "uarm" | "farm" | "hand" | "thigh" | "shank" | "foot" | "toe";

export const REF_STATURE = { male: 1.75, female: 1.62 } as const;
export const HUMAN_BONE = /^(pelvis|torso|neck|head|arm|forearm|hand|thigh|shank|foot|toe)(-[lr])?$/;
export const partOf = (bone: string): PartName | null => {
  const m = HUMAN_BONE.exec(bone);
  return m === null ? null : ({ pelvis: "pelvis", torso: "torso", neck: "neck", head: "head", arm: "uarm", forearm: "farm", hand: "hand", thigh: "thigh", shank: "shank", foot: "foot", toe: "toe" } as const)[m[1] as "pelvis"];
};

/** 📐️ Half breadth `w`, half depth `d` (metres, male 1.75 m), anterior centre offset `ac` (metres, positive towards the front) and section exponent `e` at fraction `t` of each part, from the sources in the file header. Head: chin (0) to vertex (1); torso: waist (0) to shoulder line (1); pelvis: crotch (0) to iliac crest (1); limbs: proximal joint (0) to distal joint (1); hand: wrist to fingertips; foot: heel to ball; toe: ball to tip. */
export const TABLE: Record<PartName, Row[]> = {
  head: [[0, 0.018, 0.024, 0.006, 2], [0.06, 0.036, 0.042, 0.004, 2.1], [0.18, 0.052, 0.064, 0, 2.2], [0.32, 0.066, 0.08, 0.002, 2.2], [0.46, 0.073, 0.091, 0.003, 2.3], [0.6, 0.0775, 0.0975, 0, 2.4], [0.74, 0.077, 0.095, -0.006, 2.4], [0.87, 0.063, 0.079, -0.011, 2.2], [0.95, 0.04, 0.05, -0.012, 2.1], [1, 0.014, 0.018, -0.012, 2]],
  neck: [[0, 0.07, 0.062, -0.006, 2.4], [0.3, 0.058, 0.058, -0.002, 2.2], [0.7, 0.052, 0.055, 0.002, 2.1], [1, 0.05, 0.054, 0.004, 2.1]],
  torso: [[0, 0.15, 0.108, 0, 2.8], [0.12, 0.155, 0.112, 0, 2.9], [0.3, 0.16, 0.118, 0.001, 2.9], [0.55, 0.166, 0.126, 0.004, 3], [0.75, 0.178, 0.116, 0.002, 2.7], [0.9, 0.196, 0.088, -0.008, 2.4], [1, 0.12, 0.074, -0.01, 2.2]],
  pelvis: [[0, 0.12, 0.1, 0, 2.4], [0.25, 0.17, 0.125, -0.012, 2.6], [0.55, 0.178, 0.118, -0.006, 2.7], [0.85, 0.16, 0.11, 0, 2.8], [1, 0.15, 0.108, 0, 2.8]],
  uarm: [[0, 0.052, 0.054, 0, 2.2], [0.15, 0.056, 0.058, 0, 2.2], [0.45, 0.047, 0.049, 0, 2.2], [0.8, 0.041, 0.044, 0, 2.2], [1, 0.038, 0.038, 0, 2.2]],
  farm: [[0, 0.038, 0.036, 0, 2.2], [0.15, 0.041, 0.039, 0.002, 2.3], [0.5, 0.034, 0.033, 0, 2.3], [0.85, 0.025, 0.02, 0, 2.5], [1, 0.023, 0.016, 0, 2.6]],
  hand: [[0, 0.03, 0.017, 0, 2.5], [0.12, 0.04, 0.022, 0, 2.6], [0.4, 0.043, 0.024, 0, 2.7], [0.58, 0.04, 0.021, 0, 2.7], [0.82, 0.034, 0.016, 0, 2.6], [1, 0.022, 0.01, 0, 2.3]],
  thigh: [[0, 0.088, 0.092, -0.004, 2.3], [0.15, 0.09, 0.096, -0.004, 2.3], [0.45, 0.08, 0.082, 0, 2.3], [0.75, 0.066, 0.068, 0.002, 2.2], [0.92, 0.057, 0.06, 0.002, 2.2], [1, 0.055, 0.058, 0.002, 2.2]],
  shank: [[0, 0.055, 0.058, 0.002, 2.2], [0.08, 0.052, 0.058, 0.004, 2.2], [0.28, 0.052, 0.062, -0.01, 2.2], [0.55, 0.043, 0.05, -0.005, 2.2], [0.85, 0.032, 0.038, -0.004, 2.1], [1, 0.036, 0.036, 0, 2.1]],
  foot: [[0, 0.03, 0.03, 0, 2.4], [0.12, 0.034, 0.04, 0, 2.4], [0.35, 0.036, 0.045, 0.005, 2.4], [0.6, 0.04, 0.03, 0.004, 2.5], [0.85, 0.048, 0.022, 0, 2.6], [1, 0.044, 0.02, 0, 2.6]],
  toe: [[0, 0.048, 0.022, 0, 2.6], [0.6, 0.044, 0.02, 0, 2.5], [1, 0.03, 0.012, 0, 2.2]],
};

/** ♀️ Female against male (DIN 33402-2 P50 ratios): width and depth multipliers by part before the stature scale (hip and thigh broader, shoulders and neck narrower, hand and foot smaller). */
export const FEMALE: Record<PartName, [number, number]> = { head: [0.96, 0.96], neck: [0.88, 0.88], torso: [0.92, 0.95], pelvis: [1.04, 1.04], uarm: [0.92, 0.94], farm: [0.9, 0.92], hand: [0.9, 0.9], thigh: [1.04, 1.04], shank: [0.97, 0.97], foot: [0.92, 0.92], toe: [0.92, 0.92] };
/** 🏋️ Gain of the build parameter (0 lean, 1 average, 2 heavy) on width and depth by part (soft tissue grows at trunk, pelvis, thigh and upper arm, not at head, hand, foot). */
export const BUILD_GAIN: Record<PartName, number> = { head: 0.02, neck: 0.1, torso: 0.16, pelvis: 0.18, uarm: 0.18, farm: 0.1, hand: 0.02, thigh: 0.2, shank: 0.1, foot: 0.03, toe: 0.03 };

/** 🧒️ Child and elderly proportions: head size against the stature scale (a 5-year-old head is about 87 % of an adult head breadth on 63 % of the stature), trunk fullness, limb thickness (Dreyfuss, children; ageing: sarcopenia). */
export function ageFactors(age: number): { head: number; trunk: number; limb: number; kyphosis: number } {
  const g = Math.min(1, Math.max(0, age / 16));
  const old = Math.min(1, Math.max(0, (age - 60) / 30));
  return { head: 1 + 0.5 * (1 - g) ** 1.3, trunk: 1 + 0.14 * (1 - g), limb: 1 - 0.1 * (1 - g) - 0.08 * old, kyphosis: 0.3 * old };
}

export const personOf = (cfg: Config, bones: Record<string, Json>): Person => {
  const len = (id: string): number => bones[id]?.lengthM ?? bones[`${id}-l`]?.lengthM ?? 0;
  const stature = cfg.statureM ?? 0.08 + ["shank", "thigh", "pelvis", "torso", "neck", "head"].reduce((a, k) => a + len(k), 0);
  return { sex: cfg.sex ?? "male", age: cfg.age ?? 35, build: cfg.build ?? 1, pregnant: cfg.pregnant ?? 0, stoop: cfg.stoop ?? 0, statureM: stature };
};

/** 🧬️ One interpolated template station of `part` for this person at fraction `t`: half extents, anterior centre offset and exponent, scaled by stature, sex, build, age, stoop (kyphosis behind the upper trunk) and pregnancy (abdominal bump on the lower trunk and pelvis front).
 * @example tplAt("thigh", 0.5, personOf({}, bones)) // { w: 0.08, d: 0.082, ac: 0, e: 2.3 } */
export function tplAt(part: PartName, t: number, p: Person): { w: number; d: number; ac: number; e: number } {
  const rows = TABLE[part];
  let i = 0;
  while (i < rows.length - 2 && rows[i + 1]![0] < t) i++;
  const a = rows[i]!;
  const b = rows[i + 1]!;
  const k = Math.min(1, Math.max(0, (t - a[0]) / Math.max(1e-9, b[0] - a[0])));
  const l = (x: number, y: number): number => x + (y - x) * k;
  const s = p.statureM / REF_STATURE[p.sex];
  const f = p.sex === "female" ? FEMALE[part] : [1, 1];
  const ag = ageFactors(p.age);
  const gain = 1 + (p.build - 1) * BUILD_GAIN[part];
  const group = part === "head" ? ag.head : part === "torso" || part === "pelvis" ? ag.trunk : part === "hand" || part === "foot" || part === "toe" ? 1 : ag.limb;
  let w = l(a[1], b[1]) * s * f[0]! * gain * group;
  let d = l(a[2], b[2]) * s * f[1]! * gain * group;
  let ac = l(a[3], b[3]) * s;
  if (part === "torso") {
    ac -= (ag.kyphosis + p.stoop) * 0.04 * Math.max(0, t - 0.4);
    const bump = p.pregnant * Math.exp(-((t - 0.18) ** 2) / 0.05);
    d *= 1 + 0.5 * bump;
    ac += 0.5 * (d - l(a[2], b[2]) * s) * (bump > 0 ? 1 : 0);
  }
  if (part === "pelvis") {
    const bump = p.pregnant * Math.max(0, t - 0.5);
    d *= 1 + 0.3 * bump;
    ac += 0.35 * (d - l(a[2], b[2]) * s) * (bump > 0 ? 1 : 0);
  }
  return { w, d, ac, e: l(a[4], b[4]) };
}

/* ---------------------------------------------------------------- frames and drawn constraints */

export type Frame = { o: V3; u: V3; v: V3; w: V3 };
export type BoneInfo = { id: string; part: PartName; frame: Frame; L: number; ant: 1 | -1; side: "l" | "r" | "" };

/** 🧭️ Human bones of the rig with their default-pose frame, length and anterior sign: `ant` = +1 when the local +v points to the front of the body in the rest orientation of the rig (accumulated 2D rest angles; the body faces +x at rest), -1 when it points backwards. For foot and toe +v is dorsal.
 * @example boneInfos(c)["thigh-l"].ant // 1 */
export function boneInfos(c: Ctx): Record<string, BoneInfo> {
  const bones: Json[] = c.m.rig.bones;
  const byId = Object.fromEntries(bones.map((b) => [b.id, b])) as Record<string, Json>;
  const ang = (id: string): number => (byId[id]!.angleDeg ?? 0) + (byId[id]!.parent ? ang(byId[id]!.parent) : 0);
  const out: Record<string, BoneInfo> = {};
  for (const b of bones) {
    const part = partOf(b.id);
    if (part === null) continue;
    const a = (ang(b.id) * Math.PI) / 180;
    out[b.id] = { id: b.id, part, frame: c.state.frames[b.id] as Frame, L: b.lengthM, ant: -Math.sin(a) >= 0 ? 1 : -1, side: b.id.endsWith("-l") ? "l" : b.id.endsWith("-r") ? "r" : "" };
  }
  return out;
}

const dot3 = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const PROJ: Record<View, (p: readonly number[]) => [number, number]> = { elevation: (p) => [p[0]!, p[1]!], front: (p) => [p[2]!, p[1]!], plan: (p) => [p[0]!, p[2]!] };

export type Group = { bone: string; region: string; ids: Set<string>; kind: string; shapes: { id: string; view: View; role: string; rings: { x: number; y: number }[][] }[] };

/** ✏️ The drawn closed shapes of every human bone of one scale grouped by (bone, region): the evidence the loft stations are cut from.
 * @example drawnGroups(c, infos)["torso"]["coat"].ids */
export function drawnGroups(c: Ctx, infos: Record<string, BoneInfo>): Record<string, Record<string, Group>> {
  const kinds = Object.fromEntries((c.m.thermal.regions as Json[]).map((r) => [r.id, r]));
  const out: Record<string, Record<string, Group>> = {};
  for (const view of ["elevation", "front", "plan"] as View[])
    for (const s of drawn(c, view)) {
      if (infos[s.bone] === undefined) continue;
      const region = s.region ?? "";
      const g = ((out[s.bone] ??= {})[region] ??= { bone: s.bone, region, ids: new Set(), kind: kinds[region]?.kind ?? "", shapes: [] });
      g.ids.add(s.id);
      g.shapes.push({ id: s.id, view, role: s.role, rings: s.rings });
    }
  return out;
}

/** ✂️ Extent of the shapes of a group along the line through the bone axis at station `s` in direction `d`, in the given view; `null` when no shape is crossed. */
function cutShapes(shapes: Group["shapes"], view: View, P: V3, d: V3, only?: ReadonlySet<string>): [number, number] | null {
  const [px, py] = PROJ[view](P);
  const [dx, dy] = PROJ[view](d);
  let lo = Infinity;
  let hi = -Infinity;
  for (const sh of shapes) {
    if (sh.view !== view || (only !== undefined && !only.has(sh.id))) continue;
    for (const ring of sh.rings)
      for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
        const a = ring[k]!;
        const b = ring[i]!;
        const ex = b.x - a.x;
        const ey = b.y - a.y;
        const den = dx * ey - dy * ex;
        if (Math.abs(den) < 1e-12) continue;
        const t = ((a.x - px) * ey - (a.y - py) * ex) / den;
        const q = ((a.x - px) * dy - (a.y - py) * dx) / den;
        if (q >= 0 && q <= 1) {
          lo = Math.min(lo, t);
          hi = Math.max(hi, t);
        }
      }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

export type Cut = { v: [number, number] | null; w: [number, number] | null };
/** 🔪️ Sagittal (v, from the elevation) and lateral (w, from front for upright bones, plan for lying ones, the other as fallback) extent of the group at station `s` along the bone, as signed offsets from the bone axis. */
export function cutAt(info: BoneInfo, g: Pick<Group, "shapes">, s: number, only?: ReadonlySet<string>): Cut {
  const f = info.frame;
  const P: V3 = [0, 1, 2].map((k) => f.o[k]! + s * f.u[k]!) as V3;
  const upright = Math.abs(f.u[1]!) >= Math.abs(f.u[0]!);
  const first: View = upright ? "front" : "plan";
  const second: View = upright ? "plan" : "front";
  const v = cutShapes(g.shapes, "elevation", P, f.v, only);
  const w = cutShapes(g.shapes, first, P, f.w, only) ?? cutShapes(g.shapes, second, P, f.w, only);
  return { v, w };
}

/** 📏️ Range `[s0, s1]` of a group along the bone axis (bone-local u), from the elevation shapes, else plan, else front. */
export function rangeOf(info: BoneInfo, g: Pick<Group, "shapes">, only?: ReadonlySet<string>): [number, number] | null {
  const f = info.frame;
  const proj = (v: View): number => (v === "elevation" ? Math.hypot(f.u[0]!, f.u[1]!) : v === "plan" ? Math.hypot(f.u[0]!, f.u[2]!) : Math.hypot(f.u[1]!, f.u[2]!));
  for (const view of (["elevation", "plan", "front"] as View[]).sort((a, b) => proj(b) - proj(a))) {
    if (proj(view) < 0.5) continue;
    let lo = Infinity;
    let hi = -Infinity;
    for (const sh of g.shapes) {
      if (sh.view !== view || (only !== undefined && !only.has(sh.id))) continue;
      for (const ring of sh.rings)
        for (const q of ring) {
          const sv = view === "elevation" ? (q.x - f.o[0]) * f.u[0]! + (q.y - f.o[1]) * f.u[1]! : view === "plan" ? (q.x - f.o[0]) * f.u[0]! + (q.y - f.o[2]) * f.u[2]! : (q.y - f.o[1]) * f.u[1]! + (q.x - f.o[2]) * f.u[2]!;
          lo = Math.min(lo, sv);
          hi = Math.max(hi, sv);
        }
    }
    if (Number.isFinite(lo) && hi - lo > 1e-3) return [lo, hi];
  }
  return null;
}

/** 📏️ Union of the ranges along the bone axis seen in every view where the axis projects with at least 0.35 (an oblique hand drawn shorter in the elevation than in the plan keeps the longer reach). */
export function rangeUnion(info: BoneInfo, g: Pick<Group, "shapes">, only?: ReadonlySet<string>): [number, number] | null {
  const f = info.frame;
  const proj: Record<View, number> = { elevation: Math.hypot(f.u[0]!, f.u[1]!), plan: Math.hypot(f.u[0]!, f.u[2]!), front: Math.hypot(f.u[1]!, f.u[2]!) };
  let lo = Infinity;
  let hi = -Infinity;
  for (const view of ["elevation", "plan", "front"] as View[]) {
    if (proj[view] < 0.35) continue;
    const r = rangeOf(info, { shapes: g.shapes.filter((x) => x.view === view) }, only);
    if (r === null) continue;
    lo = Math.min(lo, r[0]);
    hi = Math.max(hi, r[1]);
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

/* ---------------------------------------------------------------- sections and solids */

export const ROT: Quat = [0, -0.70711, 0, 0.70711];
/** 🌀️ Section frame x = w, y = v, z = -u twisted about u by `phiDeg` (forearm pronation: the section turns from the elbow to the wrist). */
export function twistRotation(phiDeg: number): Quat {
  if (Math.abs(phiDeg) < 1e-6) return ROT;
  const h = (phiDeg * Math.PI) / 360;
  const qz: Quat = [0, 0, Math.sin(h), Math.cos(h)];
  const [ax, ay, az, aw] = ROT;
  const [bx, by, bz, bw] = qz;
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz];
}

export type Stn = { s: number; cv: number; cw: number; rx: number; ry: number; e: number; twist?: number };
export const sectionOf = (q: Stn): Section => ({ at: [q.s, q.cv, q.cw], rx: q.rx, ry: q.ry, exponent: q.e, rotation: twistRotation(q.twist ?? 0) });
export type Common = { id: string; bone: string; material: string; tone?: number; role?: string };
export const loftStations = (p: Common, st: readonly Stn[], caps = true): Json => loftOf(p, st.map(sectionOf), caps);

/** 📈️ Linear interpolation of a station list at `s` (clamped to the ends). */
export function at(st: readonly Stn[], s: number): Stn {
  if (s <= st[0]!.s) return st[0]!;
  for (let i = 0; i + 1 < st.length; i++) {
    const a = st[i]!;
    const b = st[i + 1]!;
    if (s <= b.s) {
      const k = (s - a.s) / Math.max(1e-9, b.s - a.s);
      const l = (x: number, y: number): number => x + (y - x) * k;
      return { s, cv: l(a.cv, b.cv), cw: l(a.cw, b.cw), rx: l(a.rx, b.rx), ry: l(a.ry, b.ry), e: l(a.e, b.e), twist: l(a.twist ?? 0, b.twist ?? 0) };
    }
  }
  return st[st.length - 1]!;
}

export type FitOptions = { n: number; feature: number; fmin?: number; tol?: number; landmarks?: boolean; ends: [number, number]; anat?: [number, number]; only?: ReadonlySet<string>; person: Person; part: PartName; twist?: [number, number]; ref?: readonly Stn[]; e?: number; clip?: Pick<Group, "shapes"> };

/** 🦴️ Anatomical range of a part along its bone, `[a0, a1]` in bone-local u: the part the template describes (torso waist to shoulder line, pelvis crotch to iliac crest, foot heel to ball, limbs joint to joint); a garment drawn beyond it becomes an extension solid. `head` is taken from the bare head outline by the caller.
 * @example anatRange("pelvis", 0.1, 1.75) // [-0.135, 0.1] */
export function anatRange(part: PartName, L: number, stature: number): [number, number] {
  const k = stature / 1.75;
  return part === "pelvis" ? [-0.135 * k, L] : part === "foot" ? [-0.065 * k, L] : [0, L];
}

/** 🫥️ Cut of a part without any drawn shape: the template extents, clamped to the union of everything drawn at that station (`clip`) so the solid stays inside the silhouette. */
function templateCut(info: BoneInfo, tpl: { w: number; d: number; ac: number }, clip: Pick<Group, "shapes"> | undefined, s: number): Cut {
  const v: [number, number] = [info.ant * tpl.ac - tpl.d, info.ant * tpl.ac + tpl.d];
  const w: [number, number] = [-tpl.w, tpl.w];
  const all = clip === undefined ? { v: null, w: null } : cutAt(info, clip, s);
  const hit = (a: [number, number], b: [number, number] | null): [number, number] => (b === null ? a : [Math.max(a[0], b[0]), Math.min(a[1], b[1])][0]! < [Math.max(a[0], b[0]), Math.min(a[1], b[1])][1]! ? [Math.max(a[0], b[0]), Math.min(a[1], b[1])] : a);
  return { v: hit(v, all.v), w: hit(w, all.w) };
}

/** 🧩️ Stations of one group between `ends`: the template landmarks (and `n` uniform stations) of the anatomical range plus both ends; the drawn extent at every station (sagittal from the elevation, lateral from front or plan; where one view has no shape the missing extent follows the template ratio of the other, or the neighbouring `ref` stations); the template supplies the exponent. Returned along u.
 * @example fitStations(info, group, { n: 10, feature: 0.0075, person, part: "thigh", ends: [0, 0.435] }) */
export function fitStations(info: BoneInfo, g: Pick<Group, "shapes">, o: FitOptions): Stn[] {
  const [s0, s1] = o.ends;
  const [a0, a1] = o.anat ?? o.ends;
  const fl = (o.fmin ?? 0.502) * o.feature;
  const tOf = (s: number): number => Math.min(1, Math.max(0, (s - a0) / Math.max(1e-6, a1 - a0)));
  const ts = new Set<number>(Array.from({ length: o.n }, (_, i) => (o.n === 1 ? 0.5 : i / (o.n - 1))));
  if (o.landmarks !== false) for (const r of TABLE[o.part]) ts.add(r[0]);
  const near = Array.from({ length: 6 }, (_, k) => (k + 1) * 0.5 * o.feature).flatMap((d) => [s0 + d, s1 - d]);
  const ss = [...new Set([s0, s1, ...near, ...[...ts].map((t) => a0 + (a1 - a0) * t)].filter((s) => s >= s0 - 1e-9 && s <= s1 + 1e-9))].sort((a, b) => a - b);
  const stations: (Station & { e: number; twist?: number })[] = [];
  for (const s of ss) {
    const t = tOf(s);
    const tpl = tplAt(o.part, t, o.person);
    const em = Math.min(0.002, 0.15 * (s1 - s0));
    const eps = s === s0 ? em : s === s1 ? -em : 0;
    const cut = g.shapes.length === 0 ? templateCut(info, tpl, o.clip, s) : cutAt(info, g, s + eps, o.only);
    if (cut.v === null && cut.w === null) continue;
    const ratio = tpl.w / Math.max(tpl.d, 1e-6);
    const ref = o.ref === undefined ? undefined : at(o.ref, s);
    const ry = cut.v === null ? (cut.w![1] - cut.w![0]) / 2 / ratio : (cut.v[1] - cut.v[0]) / 2;
    const rx = cut.w === null ? (ref === undefined ? ry * ratio : ref.rx + (ry - ref.ry)) : (cut.w[1] - cut.w[0]) / 2;
    stations.push({ s, c1: cut.v === null ? 0 : (cut.v[0] + cut.v[1]) / 2, c2: cut.w === null ? (ref?.cw ?? 0) : (cut.w[0] + cut.w[1]) / 2, rx: Math.max(rx, fl), ry: Math.max(ry, fl), e: o.e ?? tpl.e, twist: o.twist === undefined ? 0 : o.twist[0] + (o.twist[1] - o.twist[0]) * t });
  }
  const nearest = (s: number) => stations.reduce((b, x) => (Math.abs(x.s - s) < Math.abs(b.s - s) ? x : b), stations[0]!);
  const keep = o.tol === undefined || stations.length < 3 ? stations : (selectStations(stations, o.tol, fl) as typeof stations).map((q) => ({ ...q, e: nearest(q.s).e, twist: nearest(q.s).twist }));
  return keep.map((q) => ({ s: q.s, cv: q.c1 ?? 0, cw: q.c2 ?? 0, rx: q.rx, ry: q.ry, e: q.e, twist: q.twist }));
}

/** 🎀️ A hanging mass drawn as one closed elevation outline (ponytail, braid, bun, tail): sections perpendicular to its principal axis, centres on the mid-chords, half chords from the outline, lateral half extent from the plan outline of the same id (tapered with the chord). The path may leave the bone axis, which a station list along `u` cannot do.
 * @example ribbonStations(info, group, new Set(["ponytail"]), 0.0075, 9, 0.502 * 0.0075) */
export function ribbonStations(info: BoneInfo, g: Pick<Group, "shapes">, only: ReadonlySet<string>, feature: number, n: number, fl: number): Stn[] {
  const f = info.frame;
  const loc = (q: { x: number; y: number }): [number, number] => [(q.x - f.o[0]) * f.u[0]! + (q.y - f.o[1]) * f.u[1]!, (q.x - f.o[0]) * f.v[0]! + (q.y - f.o[1]) * f.v[1]!];
  const pts = g.shapes.filter((x) => x.view === "elevation" && only.has(x.id)).flatMap((x) => x.rings.flatMap((r) => r.map(loc)));
  if (pts.length < 6) return [];
  const mu = [pts.reduce((a, q) => a + q[0], 0) / pts.length, pts.reduce((a, q) => a + q[1], 0) / pts.length];
  let sxx = 0, sxy = 0, syy = 0;
  for (const q of pts) { const dx = q[0] - mu[0]!, dy = q[1] - mu[1]!; sxx += dx * dx; sxy += dx * dy; syy += dy * dy; }
  const th = 0.5 * Math.atan2(2 * sxy, sxx - syy);
  const d: [number, number] = [Math.cos(th), Math.sin(th)];
  const nn: [number, number] = [-d[1], d[0]];
  const tt = pts.map((q) => (q[0] - mu[0]!) * d[0] + (q[1] - mu[1]!) * d[1]);
  const t0 = Math.min(...tt);
  const t1 = Math.max(...tt);
  const rings = g.shapes.filter((x) => x.view === "elevation" && only.has(x.id)).flatMap((x) => x.rings.map((r) => r.map(loc)));
  const plan = g.shapes.filter((x) => x.view === "plan" && only.has(x.id)).flatMap((x) => x.rings.flat());
  const halfW = plan.length === 0 ? 0 : (Math.max(...plan.map((q) => q.y)) - Math.min(...plan.map((q) => q.y))) / 2;
  const out: Stn[] = [];
  let hmax = fl;
  const raw: { t: number; c: number; h: number }[] = [];
  for (let k = 0; k < n; k++) {
    const t = t0 + ((t1 - t0) * (0.02 + 0.96 * k / (n - 1)));
    let lo = Infinity, hi = -Infinity;
    for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) {
      const a = r[j]!, b = r[i]!;
      const ta = (a[0] - mu[0]!) * d[0] + (a[1] - mu[1]!) * d[1], tb = (b[0] - mu[0]!) * d[0] + (b[1] - mu[1]!) * d[1];
      if ((ta <= t) !== (tb <= t)) {
        const k2 = (t - ta) / (tb - ta);
        const nv = ((a[0] + k2 * (b[0] - a[0])) - mu[0]!) * nn[0] + ((a[1] + k2 * (b[1] - a[1])) - mu[1]!) * nn[1];
        lo = Math.min(lo, nv); hi = Math.max(hi, nv);
      }
    }
    if (!Number.isFinite(lo)) continue;
    raw.push({ t, c: (lo + hi) / 2, h: Math.max((hi - lo) / 2, fl) });
    hmax = Math.max(hmax, (hi - lo) / 2);
  }
  for (const q of raw) {
    const u = mu[0]! + q.t * d[0] + q.c * nn[0];
    const v = mu[1]! + q.t * d[1] + q.c * nn[1];
    out.push({ s: u, cv: v, cw: 0, rx: Math.max(fl, halfW === 0 ? q.h : Math.min(halfW, halfW * (q.h / hmax) ** 0.6)), ry: q.h, e: 2.2 });
  }
  return out;
}

/** 🧥️ The station list pulled in by `t` metres on both axes (garment layer, skin under a garment); floors at `fl`. */
export const inset = (st: readonly Stn[], t: number, fl: number): Stn[] => st.map((q) => ({ ...q, rx: Math.max(q.rx - t, fl), ry: Math.max(q.ry - t, fl) }));

/** 🦴️ The skin station list under a garment: the template of the part for this person, clamped inside the garment's inner surface (outer minus layers minus air gap) and centred on it.
 * @example skinUnder(garmentStations, "thigh", person, 0.0033 + 0.003, fl, 0.5) */
export function skinUnder(outer: readonly Stn[], part: PartName, person: Person, inner: number, fl: number, tOf: (s: number) => number): Stn[] {
  return outer.map((q) => {
    const tpl = tplAt(part, tOf(q.s), person);
    const mw = Math.max(q.rx - inner, fl);
    const md = Math.max(q.ry - inner, fl);
    return { ...q, rx: Math.max(Math.min(tpl.w, mw), fl), ry: Math.max(Math.min(tpl.d, md), fl), e: Math.min(q.e, tpl.e + 0.2) };
  });
}

export const FEATURE_OF = (c: Ctx): number => featureM(c.scale, c.raw);
export { shellOffset };

/** 🫧️ A loft inset by `t` on both section axes; sections thinner than 2.2 t keep 45 % of their radius (tapered ends stay closed and inside).
 * @example shellLoft(garment, 0.006, "th-torso-overcoat") */
export function shellLoft(sol: Json, t: number, id: string): Json {
  const n = JSON.parse(JSON.stringify(sol));
  n.id = id;
  for (const k of ["of", "region", "layer", "tags"]) delete n[k];
  for (const q of n.sections) for (const r of ["rx", "ry"] as const) q[r] = q[r] > 2.2 * t ? q[r] - t : Math.max(q[r] * 0.45, 0.0006);
  return n;
}

/** 🧅️ Layer shells of a garment solid for a region with layers listed innermost first: the outermost layer references the garment (`of`), each inner layer is the garment inset by the thicknesses of the layers outside it (B2); `null` entries (a shell that would vanish) are dropped by the caller.
 * @example layerShells(garment, region, "coat") */
export function layerShells(sol: Json, region: Json, tag: string, nested = true): Json[] {
  const layers: Json[] = region.layers ?? [];
  const out: Json[] = [];
  if (layers.length <= 1 || !nested) return [{ id: `th-${tag}`, of: sol.id, region: region.id, ...(layers.length === 1 ? { layer: layers[0].id } : {}) }];
  let cum = 0;
  for (let i = layers.length - 1; i >= 0; i--) {
    const l = layers[i]!;
    if (i === layers.length - 1) out.push({ id: `th-${tag}-${l.id}`, of: sol.id, region: region.id, layer: l.id });
    else {
      const sh = sol.kind === "loft" ? shellLoft(sol, cum, `th-${tag}-${l.id}`) : shellOffset(sol, cum, `th-${tag}-${l.id}`, 0.0004);
      if (sh !== null) out.push({ ...sh, region: region.id, layer: l.id, role: "body" });
    }
    cum += l.thicknessM ?? 0.002;
  }
  return out;
}

export const layersThickness = (region: Json | undefined): number => (region?.layers ?? []).reduce((a: number, l: Json) => a + (l.thicknessM ?? 0.002), 0);

export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
