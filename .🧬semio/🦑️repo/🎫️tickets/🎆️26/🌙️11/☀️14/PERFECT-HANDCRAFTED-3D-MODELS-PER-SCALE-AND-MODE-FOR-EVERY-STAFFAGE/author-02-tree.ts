/** 🌲️ Shared thermal and structure builders of the author-02 trees: bark by area calibration, foliage as stacked leaf sheets plus emitter patches, members along bones or body limb paths, joints, supports and masses.
 * @see ./author-02-lib.ts */
import { toWorld, type Json, type V3 } from "./volumes-lib.ts";
import { Kit, type Meta, type Sec } from "./author-02-lib.ts";

export type Group = { id: string; bone: string; region: string; c: V3; h: V3; fit?: { phi: number; ha: number; hb: number; exp: number }; yRange: [number, number]; list?: string };

export type Ctx = {
  k: Kit;
  regionOf: Map<string, string>;
  groups: Group[];
  paths: Map<string, Sec[]>;
};

export const newCtx = (k: Kit): Ctx => ({ k, regionOf: new Map(), groups: [], paths: new Map() });

const scaleSolid = (s: Json, f: number): Json => {
  const o = JSON.parse(JSON.stringify(s));
  if (o.kind === "loft") {
    const cx = o.sections.reduce((t: number, x: Json) => t + x.at[0], 0) / o.sections.length;
    void cx;
    for (const x of o.sections) { x.rx *= f; x.ry *= f; }
  } else if (o.kind === "capsule") { o.r0 *= f; o.r1 *= f; }
  else if (o.kind === "ellipsoid") o.radii = o.radii.map((r: number) => r * f);
  return o;
};

/** 🪵️ Bark region: reference the body solids of that region when their area is within `tol` of the region area, else scaled real solids (calibrated) or bone capsules when the scale has no body solid of the region. */
export function thermalBark(c: Ctx, region: string, o: { fallbackBones?: string[]; tol?: number; radius?: number; minRadius?: number } = {}) {
  const { k } = c;
  const reg = k.asset.thermal.regions.find((r: Json) => r.id === region);
  const body = k.body.filter((s) => c.regionOf.get(s.id) === region);
  const tol = o.tol ?? (k.scale === 50 ? 0.1 : 0.2);
  if (body.length) {
    const a = k.area(body);
    if (Math.abs(a / reg.areaM2 - 1) <= tol) { for (const s of body) k.thermal.push({ id: `th-${s.id}`, of: s.id, region }); return a; }
    const best = k.calibrate((f) => body.map((s) => ({ ...scaleSolid(s, f), id: `th-${s.id}` })), reg.areaM2, 0.05, 6);
    for (const s of best.solids) { const t = { ...s, region }; k.thermal.push(t); }
    return best.area;
  }
  const bones = o.fallbackBones ?? [];
  const mat = reg.material;
  const meta: Meta = { material: mat, role: "body", tone: 0.35 };
  const minR = (o.minRadius ?? 0);
  const make = (fl: number, fr: number): Json[] => {
    const tmp = new Kit(k.spec, k.scale, k.asset);
    for (const b of bones) {
      const p = tmp.origin(b), t = tmp.tip(b);
      const e: V3 = [p[0] + (t[0] - p[0]) * fl, p[1] + (t[1] - p[1]) * fl, p[2] + (t[2] - p[2]) * fl];
      const r = Math.max(minR, (o.radius ?? tmp.boneDef(b).section?.extremeFibreM ?? 0.02) * fr);
      tmp.cap("thermal", `th-${region}-${b}`, b, p, e, r, r * 0.85, meta, { region });
    }
    return tmp.thermal;
  };
  let best = k.calibrate((f) => make(f, 1), reg.areaM2, 0.05, 1, 16);
  if (Math.abs(best.area / reg.areaM2 - 1) > 0.05) best = k.calibrate((f) => make(1, f), reg.areaM2, 0.2, 6, 16);
  k.thermal.push(...best.solids.map((s) => ({ ...s, region })));
  return best.area;
}

/** 🍃️ Foliage region (ruling C1: exposed surface): references to the exposed foliage body solids of the region, plus small transpiration outlet patches at the emitter anchors inside the crown. */
export function thermalFoliage(c: Ctx, region: string) {
  const { k } = c;
  const reg = k.asset.thermal.regions.find((r: Json) => r.id === region);
  const body = k.body.filter((s) => c.regionOf.get(s.id) === region);
  for (const s of body) k.thermal.push({ id: `th-${s.id}`, of: s.id, region });
  const t = Math.max(0.02, k.feat * 2.2);
  const meta: Meta = { material: reg.material, role: "body", tone: 0.5 };
  for (const e of (k.asset.thermal.emitters ?? []).filter((x: Json) => x.region === region)) {
    const anchors: string[] = e.anchors ?? [e.anchor];
    anchors.forEach((aid, i) => {
      const a = k.asset.anchors.find((x: Json) => x.id === aid);
      if (!a) return;
      const w = toWorld(k.fr[a.bone]!, [a.x, a.y, a.z ?? 0]);
      const r = Math.max(k.feat * 1.2, Math.min(e.diameterM / 2, 0.3));
      k.loft("thermal", `emit-${e.id}-${i + 1}`, a.bone, [k.secY([w[0], w[1] - t / 2, w[2]], r, r, 2), k.secY([w[0], w[1] + t / 2, w[2]], r, r, 2)], meta, { region, tags: [e.id, "emitter"] });
    });
  }
  return k.area(body);
}

/** 🏗️ Structure: member lofts along the bone (or the recorded limb path) with the bone section; joints; supports; masses. */
export function structure(c: Ctx, o: { joints?: "all" | "main" | "none"; minR?: number; masses?: Group[]; supportBone?: string; massScale?: number; mergeJoint?: string[]; memberTaper?: number; jointK?: number }) {
  const { k } = c;
  const feat = k.feat;
  const mat = (b: Json) => b.section?.material ?? k.asset.materials[0].id;
  const bones: string[] = k.memberBones();
  const minR = o.minR ?? feat * 0.55;
  const radiusOf = (b: Json): number => {
    const s = b.section;
    const r = s?.extremeFibreM ?? (s?.areaM2 ? Math.sqrt(s.areaM2 / Math.PI) : Math.max(0.02, b.lengthM * 0.04));
    return Math.max(r, minR);
  };
  const rOf = new Map<string, number>();
  for (const id of bones) rOf.set(id, radiusOf(k.boneDef(id)));
  for (const id of bones) {
    const b = k.boneDef(id);
    const f = k.fr[id]!;
    const r0 = rOf.get(id)!;
    const child = bones.map((x) => k.boneDef(x)).filter((x) => x.parent === id).map((x) => rOf.get(x.id)!);
    const r1 = Math.max(minR, child.length ? Math.min(r0, Math.max(...child) * 1.1) : r0 * (o.memberTaper ?? 0.8));
    const path = c.paths.get(id);
    const meta: Meta = { material: mat(b), role: "body", tone: 0.5 };
    const extra = { member: id, part: "member" };
    if (path && path.length >= 2) {
      const secs = path.map((s, i) => { const t = i / (path.length - 1); const r = r0 + (r1 - r0) * t; return { ...s, rx: r, ry: r, exp: 2 }; });
      k.loft("structure", `member-${id}`, id, secs, meta, extra);
    } else {
      const a = k.origin(id), tp = k.tip(id);
      const mk = (p: V3, r: number): Sec => ({ c: p, ax: f.v, ay: f.w, rx: r, ry: r, exp: 2 });
      const mid: V3 = [(a[0] + tp[0]) / 2, (a[1] + tp[1]) / 2, (a[2] + tp[2]) / 2];
      k.loft("structure", `member-${id}`, id, [mk(a, r0), mk(mid, (r0 + r1) / 2), mk(tp, r1)], meta, extra);
    }
  }
  const jointMode = o.joints ?? "all";
  if (jointMode !== "none") {
    for (const id of bones) {
      const b = k.boneDef(id);
      if (!b.parent || !bones.includes(b.parent)) continue;
      if (o.mergeJoint && !o.mergeJoint.includes(id)) continue;
      const r = rOf.get(id)! * (o.jointK ?? 1.3);
      const p = k.origin(id);
      k.ell("structure", `joint-${id}`, id, [p[0], p[1], c.paths.get(id)?.[0]?.c[2] ?? p[2]], [r, r, r], { material: mat(b), role: "detail", tone: 0.35 }, { member: id, part: "joint" });
    }
  }
  for (const g of o.masses ?? []) {
    const s = o.massScale ?? 0.22;
    const rr = Math.max(feat * 0.6, Math.min(g.h[0], g.h[1], g.h[2]) * s);
    const rx = Math.max(feat * 0.6, g.h[0] * s), ry = Math.max(feat * 0.6, g.h[1] * s), rz = Math.max(feat * 0.6, g.h[2] * s);
    void rr;
    k.ell("structure", `mass-${g.id}`, g.bone, g.c, [rx, ry, rz], { material: k.asset.materials.find((m: Json) => /leaf|needle|blade|foliage|culm/.test(m.id))?.id ?? k.asset.materials[0].id, role: "detail", tone: 0.3 }, { member: g.bone, part: "mass" });
  }
  for (const sid of k.asset.structure.supports ?? []) {
    const a = k.asset.anchors.find((x: Json) => x.id === sid);
    if (!a) continue;
    const w = toWorld(k.fr[a.bone]!, [a.x, a.y, 0]);
    const fw = Math.min(a.footprintWidthM ?? 0.5, o.massScale ? 10 : 10);
    const fl = a.footprintLengthM ?? fw;
    const h = Math.max(2 * feat, 0.04);
    const sx = Math.min(fl, c.paths.size ? fl : fl), sz = Math.min(fw, fw);
    void sx; void sz;
    const w0 = o.supportBone ?? a.bone;
    k.box("structure", `support-${sid}`, w0, [w[0], w[1] + h / 2, w[2]], [Math.min(fl, 0.6), h, Math.min(fw, 0.6)], { material: mat(k.boneDef(a.bone)), role: "detail", tone: 0.25 }, { member: a.bone, part: "support", anchor: sid, tags: [sid] });
  }
}
