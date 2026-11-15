/** Author 25 quadruped driver: one loft per drawn bone from the scale's own drawings with a per-animal anatomy table (names, section exponents, tones, lateral sources, ear flaps). */
import { drawn, type Ctx, type Json } from "./kit-lib.ts";
import { part, type Part } from "./author-25-lib.ts";

export type Cfg = {
  material: string;
  name: (bone: string) => string | undefined;
  e: (bone: string) => number;
  tone: (bone: string) => number;
  elevOnly?: Record<string, string[]>;
  elevExclude?: RegExp;
  latExclude?: RegExp;
  fallbackHalf?: Record<string, number>;
  latBone?: Record<string, string>;
  latFallback?: Record<string, string>;
  twin?: Record<string, string>;
  latScale?: Record<string, number>;
  latTaper?: Record<string, { taper?: [number, number]; inward?: [number, number] }>;
  bevel?: Record<string, number>;
  ears?: Record<string, { roll: number; thick: number; scale: number }>;
  skip?: string[];
  tolFactor?: number;
  twinAll?: boolean;
  tweak?: (bone: string, scale: number, p: Part) => Part;
};

export function autoParts(c: Ctx, cfg: Cfg): Part[] {
  const bones = new Set<string>();
  for (const v of ["elevation", "front", "plan"] as const) for (const s of drawn(c, v)) if (s.role === "body") bones.add(s.bone);
  for (const e of c.state.elevation as Json[]) if (!e.closed && e.shape.role === "body" && typeof e.shape.bone === "string") bones.add(e.shape.bone);
  if (cfg.twinAll === true) for (const [k, v] of Object.entries(cfg.twin ?? {})) if (bones.has(v)) bones.add(k);
  const out: Part[] = [];
  for (const bone of c.raw.rig.bones.map((b: Json) => b.id as string)) {
    if (!bones.has(bone) || cfg.skip?.includes(bone) === true) continue;
    const id = cfg.name(bone);
    if (id === undefined) continue;
    const ids = (b: string): string[] => [
      ...drawn(c, "elevation", b).filter((s) => s.role === "body").map((s) => s.id),
      ...(c.state.elevation as Json[]).filter((e) => !e.closed && e.shape.role === "body" && e.shape.bone === b).map((e) => e.shape.id as string),
    ];
    let elevBone = bone;
    let elev = ids(bone);
    if (elev.length === 0 && cfg.twin?.[bone] !== undefined) {
      elevBone = cfg.twin[bone]!;
      elev = ids(elevBone);
    }
    if (elev.length === 0) continue;
    if (cfg.elevExclude !== undefined) elev = elev.filter((x) => !cfg.elevExclude!.test(x));
    if (cfg.elevOnly?.[bone] !== undefined) elev = elev.filter((x) => cfg.elevOnly![bone]!.includes(x));
    const ear = cfg.ears?.[bone];
    const own = drawn(c, "plan", bone).some((x) => x.role === "body") || drawn(c, "front", bone).some((x) => x.role === "body");
    const latBone = cfg.latBone?.[bone] ?? (own ? bone : cfg.latFallback?.[bone] ?? bone);
    const latIds = cfg.latExclude === undefined ? undefined : [...drawn(c, "plan", latBone), ...drawn(c, "front", latBone)].filter((x) => x.role === "body" && !cfg.latExclude!.test(x.id)).map((x) => x.id);
    const lat = { view: "auto" as const, bone: latBone, ...(latIds === undefined ? {} : { ids: latIds }), ...(cfg.latScale?.[bone] === undefined ? {} : { scale: cfg.latScale[bone] }), ...(cfg.latTaper?.[bone] ?? {}) };
    const hasLat = drawn(c, "plan", latBone).some((x) => x.role === "body") || drawn(c, "front", latBone).some((x) => x.role === "body");
    let p: Part = { id, bone, material: cfg.material, tone: cfg.tone(bone), elev, elevBone, lat: hasLat ? lat : { half: cfg.fallbackHalf?.[bone] ?? 0.05 }, e: cfg.e(bone), ...(cfg.tolFactor === undefined ? {} : { tol: cfg.tolFactor * c.feature }), ...(cfg.bevel?.[bone] === undefined ? {} : { bevel: cfg.bevel[bone] }) };
    if (ear !== undefined) p = { ...p, lat: { ...lat, scale: ear.scale }, roll: ear.roll, thick: ear.thick };
    out.push(cfg.tweak?.(bone, c.scale, p) ?? p);
  }
  return out;
}

export const buildBody = (c: Ctx, cfg: Cfg): Json[] => autoParts(c, cfg).map((p) => part(c, p));

export type StructCfg = {
  radius: (bone: string) => [number, number];
  lateral: (bone: string) => number | undefined;
  material: string;
  jointScale?: number;
  length?: Record<string, number>;
  jointScales?: number[];
  centreV?: RegExp;
  diagonal?: RegExp;
};

/** Structure: a capsule member along every eligible bone (B1), a joint ellipsoid at its origin at 1:50 and 1:100, and a mass ellipsoid for bones carrying at least 10 % of the mass. */
export function buildStructure(c: Ctx, cfg: StructCfg): Json[] {
  const bones: Json[] = c.raw.rig.bones;
  const height: number = c.raw.dimensions.heightM;
  const total = bones.reduce((n, b) => n + (b.mass?.kg ?? 0), 0);
  const floor = 0.51 * c.feature;
  const out: Json[] = [];
  const base = { material: cfg.material, role: "body", tone: 0.4 };
  for (const b of bones) {
    const eligible = b.section !== undefined || b.lengthM >= 0.05 * height;
    if (!eligible) continue;
    const grid = 0.5 * c.feature;
    const [r0, r1] = cfg.radius(b.id).map((r) => Math.max(floor, Math.round(r / grid) * grid)) as [number, number];
    const undrawn = c.scale >= 200 && !(c.vol.body as Json[]).some((x) => x.bone === b.id);
    const sol = (c.vol.body as Json[]).find((x) => x.bone === b.id && x.kind === "loft" && x.role === "body");
    const w = cfg.lateral(b.id) ?? (sol === undefined ? 0 : (sol.sections as Json[]).reduce((n: number, q: Json) => n + q.at[2], 0) / sol.sections.length);
    const vc = cfg.centreV?.test(b.id) === true && sol !== undefined ? (sol.sections as Json[]).reduce((n: number, q: Json) => n + q.at[1], 0) / sol.sections.length : 0;
    const secs = sol === undefined ? [] : (sol.sections as Json[]);
    const diag = cfg.diagonal?.test(b.id) === true && secs.length >= 2;
    const lerpAt = (u: number): [number, number] => {
      let k = 0;
      while (k + 2 < secs.length && secs[k + 1].at[0] < u) k++;
      const a = secs[k], z = secs[k + 1];
      const t = Math.min(1, Math.max(0, (u - a.at[0]) / (z.at[0] - a.at[0] || 1)));
      return [a.at[1] + (z.at[1] - a.at[1]) * t, a.at[2] + (z.at[2] - a.at[2]) * t];
    };
    if (diag) {
      const u0 = secs[0].at[0] + 0.6 * r0 + 0.02 * b.lengthM;
      const u1 = secs[secs.length - 1].at[0] - 0.9 * r1 - 0.02 * b.lengthM;
      const [v0, w0] = lerpAt(u0);
      const [v1, w1] = lerpAt(u1);
      out.push({ id: `m-${b.id}`, kind: "capsule", bone: b.id, ...base, member: b.id, part: "member", p0: [u0, v0, w0], p1: [u1, v1, w1], r0, r1 });
      if (c.scale <= 50 || (c.scale === 100 && r0 >= 0.02)) out.push({ id: `j-${b.id}`, kind: "ellipsoid", bone: b.id, ...base, tone: 0.32, member: b.id, part: "joint", radii: [r0 * (cfg.jointScale ?? 1.3), r0 * (cfg.jointScale ?? 1.3), r0 * (cfg.jointScale ?? 1.3)], transform: { translation: [u0 + 0.5 * r0, v0, w0] } });
      continue;
    }
    out.push({ id: `m-${b.id}`, kind: "capsule", bone: b.id, ...base, member: b.id, part: "member", p0: [0.6 * r0, vc, w], p1: [undrawn ? 0.6 * r0 + 2 * floor : Math.max(b.lengthM * (cfg.length?.[b.id] ?? 1) - r1, 0.5 * b.lengthM), vc, w], r0: undrawn ? floor : r0, r1: undrawn ? floor : r1 });
    if (c.scale <= 50 || (c.scale === 100 && r0 >= 0.02)) out.push({ id: `j-${b.id}`, kind: "ellipsoid", bone: b.id, ...base, tone: 0.32, member: b.id, part: "joint", radii: [r0 * (cfg.jointScale ?? 1.3), r0 * (cfg.jointScale ?? 1.3), r0 * (cfg.jointScale ?? 1.3)], transform: { translation: [1.1 * r0, vc, w] } });
    if (b.mass !== undefined && b.mass.kg / total >= 0.1) {
      const vol = b.mass.kg / 1000;
      const r = Math.cbrt((3 * vol) / (4 * Math.PI));
      const cx = b.mass.comX ?? b.lengthM / 2;
      let cy = b.mass.comY ?? 0;
      if (sol !== undefined) {
        const q = sectionAt(sol, cx);
        cy = Math.min(Math.max(cy, q[0] - Math.max(q[1] * 0.95 - 0.45 * r, 0)), q[0] + Math.max(q[1] * 0.95 - 0.45 * r, 0));
      }
      out.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, ...base, tone: 0.55, member: b.id, part: "mass", radii: [0.7 * r, 0.45 * r, 0.4 * r], transform: { translation: [cx, cy, w] } });
    }
  }
  return out;
}

/** Surface lateral coordinate (bone-local w) of a loft along u at (u, v): superellipse section interpolated between the neighbouring sections. */
export function surfaceW(solid: Json, u: number, v: number): number {
  const s = solid.sections as { at: number[]; rx: number; ry: number; exponent: number }[];
  let i = 0;
  while (i + 2 < s.length && s[i + 1]!.at[0]! < u) i++;
  const a = s[i]!;
  const b = s[i + 1]!;
  const t = Math.min(1, Math.max(0, (u - a.at[0]!) / (b.at[0]! - a.at[0]! || 1)));
  const l = (x: number, y: number) => x + (y - x) * t;
  const [c1, c2, rv, rw, e] = [l(a.at[1]!, b.at[1]!), l(a.at[2]!, b.at[2]!), l(a.rx, b.rx), l(a.ry, b.ry), l(a.exponent, b.exponent)];
  const q = Math.min(1, Math.abs((v - c1) / rv));
  return c2 + rw * Math.pow(Math.max(0, 1 - Math.pow(q, e)), 1 / e);
}

/** Sagittal centre and half height [c1, ry] of a loft at bone-local u, interpolated between its sections. */
function sectionAt(solid: Json, u: number): [number, number] {
  const s = solid.sections as { at: number[]; ry: number; rx: number }[];
  let i = 0;
  while (i + 2 < s.length && s[i + 1]!.at[0]! < u) i++;
  const a = s[i]!;
  const b = s[i + 1]!;
  const t = Math.min(1, Math.max(0, (u - a.at[0]!) / (b.at[0]! - a.at[0]! || 1)));
  return [a.at[1]! + (b.at[1]! - a.at[1]!) * t, a.rx + (b.rx - a.rx) * t];
}
