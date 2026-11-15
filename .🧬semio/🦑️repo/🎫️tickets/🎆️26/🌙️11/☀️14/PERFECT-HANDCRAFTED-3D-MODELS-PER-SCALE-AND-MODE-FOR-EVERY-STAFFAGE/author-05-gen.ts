import { readFileSync } from "node:fs";

import { Author, type Fit } from "./author-05-lib.ts";
import { shrink } from "./volumes-draft.ts";
import { Mask, SCALES, gridFor, projectWorld, toWorld, VIEWS, distanceField, entryOf, featureM, memberBones, meshMask, meshesAt, merged, poseState, resolveList, round5, scaleMeshes, sourcePath, viewMasks, writeJson, type Json, type V3 } from "./volumes-lib.ts";

export type Rule = { match: RegExp; fit?: Fit; skip?: boolean; extra?: Json };
export type Text = { en: string; de: string };
export type Ctx = { a: Author; asset: Json; scale: number; feature: number; body: Json[]; thermal: Json[]; structure: Json[] };

export type Cfg = {
  slug: string;
  rules: Rule[];
  fracs?: Record<number, number[]>;
  body: (c: Ctx) => void;
  thermal?: (c: Ctx) => void;
  structure?: (c: Ctx) => void;
  decisions: (scale: number) => Text[];
  memberRadius?: (bone: Json, scale: number) => number;
  massScale?: number;
  emitterRegion?: (em: Json) => string;
};

export const FRACS: Record<number, number[]> = {
  50: [0, 0.03, 0.12, 0.28, 0.46, 0.64, 0.82, 0.94, 0.985, 1],
  100: [0, 0.05, 0.2, 0.45, 0.7, 0.9, 1],
  200: [0, 0.08, 0.35, 0.7, 0.94, 1],
  500: [0, 0.1, 0.5, 0.9, 1],
};

const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const cat = (id: string): { p: number; ex: number } => {
  if (/hoof|paw|foot|pad/.test(id)) return { p: 0.5, ex: 2.6 };
  if (/cannon|pastern|hock|wrist|tarsus|metac|toe/.test(id)) return { p: 0.2, ex: 2.2 };
  if (/thigh|scapula|shoulder|humerus|gaskin|shank|radius|forearm|arm|hand|leg|fore|hind-up|upper|lower/.test(id)) return { p: 0.5, ex: 2.2 };
  if (/tail/.test(id)) return { p: 0.3, ex: 2 };
  if (/neck/.test(id)) return { p: 0.5, ex: 2.2 };
  if (/head|jaw|muzzle|snout|bill/.test(id)) return { p: 0.35, ex: 2.3 };
  if (/ear|mane|beard|forelock/.test(id)) return { p: 0.6, ex: 2 };
  return { p: 0.3, ex: 2.3 };
};

export function bodyFromShapes(c: Ctx, cfg: Cfg): void {
  const a = c.a;
  const seen = new Set<string>();
  for (const e of (a as any).rest.elevation as Json[]) {
    const s = e.shape;
    if (s.role !== "body" || seen.has(s.id)) continue;
    seen.add(s.id);
    const rule = cfg.rules.find((r) => r.match.test(s.id));
    if (rule?.skip) continue;
    const def = cat(s.id);
    const fit: Fit = { ...def, auto: true, fracs: cfg.fracs?.[c.scale] ?? FRACS[c.scale], ...(rule?.fit ?? {}) };
    a.fit(s.id, s.id, fit, rule?.extra ?? {});
  }
}

export function thermalFromBody(c: Ctx, regionOf: (s: Json) => string | string[] | undefined = (s) => s.region ?? s.regions?.[0]): void {
  const regions: Json[] = c.asset.thermal.regions;
  for (const s of c.a.solids) {
    const r = regionOf(s);
    if (r === undefined) continue;
    const list = Array.isArray(r) ? r : [r];
    for (const region of list) {
      const def = regions.find((x) => x.id === region);
      if (def === undefined) throw new Error(`unknown region ${region} for ${s.id}`);
      const layers: Json[] = def.layers ?? [];
      if (c.scale === 50 && layers.length > 1) {
        let off = 0;
        for (let k = layers.length - 1; k >= 0; k--) {
          const l = layers[k];
          const t = layerThickness(l, def);
          if (k === layers.length - 1) c.thermal.push({ id: `th-${s.id}-${l.id}`, of: s.id, region, layer: l.id });
          else {
            let g = shrink(s, off);
            for (const f of [0.7, 0.5, 0.35, 0.2]) if (g === null) g = shrink(s, off * f);
            if (g === null) g = scaleSolid(stripRegion(s), 0.85);
            const { id: _i, region: _r, regions: _rs, ...rest } = g;
            c.thermal.push({ ...rest, id: `th-${s.id}-${l.id}`, region, layer: l.id });
          }
          off += t;
        }
      } else c.thermal.push({ id: list.length === 1 ? `th-${s.id}` : `th-${region}-${s.id}`, of: s.id, region });
    }
  }
}

const layerThickness = (layer: Json, region: Json): number => {
  if (typeof layer.thicknessM === "number") return layer.thicknessM;
  const k = region.conductivityWMK ?? layer.conductivityWMK;
  return typeof layer.resistanceM2KW === "number" && typeof k === "number" ? layer.resistanceM2KW * k : 0.004;
};

const bodyLateral = (c: Ctx, bone: string): number | null => {
  const s = c.a.solids.filter((q) => q.bone === bone && q.kind === "loft" && q.role === "body").sort((x, y) => y.sections.length - x.sections.length)[0];
  if (s === undefined) return null;
  const zs: number[] = s.sections.map((q: Json) => q.at[2]);
  return zs.reduce((n, x) => n + x, 0) / zs.length;
};

const extentOfBone = (c: Ctx, bone: string): { zc: number; zh: number; vh: number } | null => {
  const ls = c.a.solids.filter((q) => q.bone === bone && q.kind === "loft");
  if (ls.length === 0) return null;
  const s = ls.sort((x, y) => y.sections.length - x.sections.length)[0]!;
  const secs: Json[] = s.sections.slice(1, -1);
  const zc = secs.reduce((n: number, q: Json) => n + q.at[2], 0) / secs.length;
  return { zc, zh: Math.min(...secs.map((q: Json) => q.ry)), vh: Math.min(...secs.map((q: Json) => q.rx)) };
};

export function memberRadiusDefault(bone: Json, mass: number): number {
  const L = bone.lengthM as number;
  const k = Math.cbrt(mass / 550);
  const id: string = bone.id;
  if (/cannon|metac|tarsus|wrist|hock|pastern/.test(id)) return Math.max(0.1 * L, 0.02 * k);
  if (/hoof|paw|foot/.test(id)) return 0.1 * L;
  if (/pelvis|lumbar|thorax|spine|body|rump|chest/.test(id)) return 0.1 * k + 0.1 * L * 0.1;
  if (/neck/.test(id)) return 0.5 * 0.1 * L + 0.03 * k;
  if (/head/.test(id)) return 0.06 * k + 0.03 * L;
  if (/tail/.test(id)) return 0.04 * L + 0.005;
  return 0.09 * L + 0.01 * k;
}

export function structureGeneric(c: Ctx, cfg: Cfg): void {
  const asset = c.asset;
  const bones: Json[] = asset.rig.bones;
  const byId = new Map(bones.map((b) => [b.id, b]));
  const mass = asset.dimensions.massKg as number;
  const ids = memberBones(asset, c.scale);
  const rf = cfg.memberRadius ?? ((b: Json) => memberRadiusDefault(b, mass));
  const minR = c.feature * 0.6;
  const radiusOf: Record<string, number> = {};
  const centre: Record<string, number> = {};
  for (const id of ids) {
    const b = byId.get(id)!;
    const L = b.lengthM as number;
    const ls = c.a.solids.filter((q) => q.bone === id && q.kind === "loft" && q.role === "body").sort((x, y) => y.sections.length - x.sections.length);
    let p0: V3 = [0, 0, 0];
    let p1: V3 = [L, 0, 0];
    let cap = Infinity;
    if (ls.length > 0) {
      const secs: Json[] = ls[0].sections;
      const inner = secs.length > 4 ? secs.slice(1, -1) : secs;
      const a0 = inner[0], a1 = inner[inner.length - 1];
      p0 = [Math.max(0, a0.at[0]), a0.at[1], a0.at[2]];
      p1 = [Math.min(L, a1.at[0]), a1.at[1], a1.at[2]];
      if (p1[0] - p0[0] < 0.3 * L) {
        p0 = [0, a0.at[1], a0.at[2]];
        p1 = [L, a1.at[1], a1.at[2]];
      }
      cap = 0.8 * Math.min(...inner.map((q: Json) => Math.min(q.rx, q.ry)));
    }
    let r = Math.max(rf(b, c.scale), minR);
    r = Math.max(minR, Math.min(r, cap));
    radiusOf[id] = r;
    centre[id] = p0[2];
    const [q0, q1] = [p0.map(r4) as V3, p1.map(r4) as V3];
    const base: Json = { id: `m-${id}`, bone: id, material: matOf(c, id), role: "body", tone: 0.35, member: id, part: "member" };
    const sec = (at: V3, k: number): Json => ({ at, rx: r4(r * k), ry: r4(r * k), rotation: [0.5, 0.5, 0.5, 0.5] });
    const lerp = (t: number): V3 => [r4(q0[0] + (q1[0] - q0[0]) * t), r4(q0[1] + (q1[1] - q0[1]) * t), r4(q0[2] + (q1[2] - q0[2]) * t)];
    if (c.scale === 50) c.structure.push({ ...base, kind: "loft", sections: [sec(q0, 1.15), sec(lerp(0.25), 0.98), sec(lerp(0.5), 0.9), sec(lerp(0.75), 0.98), sec(q1, 1.15)], capStart: true, capEnd: true });
    else if (c.scale === 100) c.structure.push({ ...base, kind: "loft", sections: [sec(q0, 1.08), sec(lerp(0.5), 0.94), sec(q1, 1.08)], capStart: true, capEnd: true });
    else c.structure.push({ ...base, kind: "loft", sections: [sec(q0, 1), sec(q1, 1)], capStart: true, capEnd: true });
  }
  const joints: string[] = jointBones(c, ids, byId);
  for (const id of joints) {
    const b = byId.get(id)!;
    if (b.parent === null || b.parent === undefined) continue;
    const r = Math.max(c.feature * 0.55, (radiusOf[id] ?? minR) * 1.15);
    c.structure.push({ id: `j-${id}`, kind: "ellipsoid", bone: id, material: matOf(c, id), role: "body", tone: 0.3, radii: [r4(r), r4(r), r4(r)], transform: { translation: [r4(Math.min(r * 0.6, (b.lengthM as number) * 0.4)), 0, r4(centre[id] ?? 0)] }, member: id, part: "joint" });
  }
  for (const b of bones) {
    const m = b.mass;
    if (m === undefined || m.distribution !== "point" || m.kg < 0.1 * mass) continue;
    const ls = c.a.solids.filter((q) => q.bone === b.id && q.kind === "loft" && q.role === "body").sort((x, y) => y.sections.length - x.sections.length);
    const secs: Json[] = ls.length === 0 ? [] : ls[0].sections.slice(1, -1);
    const zh = secs.length === 0 ? Infinity : Math.min(...secs.map((q: Json) => q.ry));
    const vh = secs.length === 0 ? Infinity : Math.min(...secs.map((q: Json) => q.rx));
    const r = Math.cbrt((3 * m.kg) / (4 * Math.PI * 1000)) * (cfg.massScale ?? 0.7);
    const rz = Math.max(c.feature * 0.55, Math.min(r, zh * 0.8));
    const ry = Math.max(c.feature * 0.55, Math.min(r, vh * 0.8));
    const rx = Math.max(c.feature * 0.55, Math.min(r * 1.25, (b.lengthM as number) * 0.5));
    const zc = secs.length === 0 ? 0 : secs.reduce((n: number, q: Json) => n + q.at[2], 0) / secs.length;
    c.structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: matOf(c, b.id), role: "body", tone: 0.5, radii: [r4(rx), r4(ry), r4(rz)], transform: { translation: [r4(m.comX), r4(m.comY), r4(zc)] }, member: b.id, part: "mass" });
  }
  for (const id of asset.structure.supports ?? []) {
    const an = (asset.anchors ?? []).find((x: Json) => x.id === id);
    if (an === undefined) continue;
    const f = Math.max(c.feature, 0.01);
    const w = Math.max(an.footprintWidthM ?? f, f * 2);
    const l = Math.max(an.footprintLengthM ?? f, f * 2);
    c.structure.push({ id: `support-${an.id}`, kind: "box", bone: an.bone, material: matOf(c, an.bone), role: "body", tone: 0.2, size: [r4(l), r4(f * 2), r4(w)], transform: { translation: [r4(an.x), r4(an.y), r4(centre[an.bone] ?? 0)] }, member: an.bone, part: "support", tags: [an.id] });
  }
  shrinkInside(c);
}

function shrinkInside(c: Ctx): void {
  const minR = c.feature * 0.6;
  for (let pass = 0; pass < 8; pass++) {
    const vol = { status: "authored", denominator: c.scale, body: c.a.solids.map(stripRegion), thermal: [], structure: c.structure, decisions: [] };
    const m = merged(c.asset, { volumes: { [String(c.scale)]: vol } });
    const sm = scaleMeshes(m, c.scale, vol);
    const state = poseState(m, c.scale);
    const bad = new Set<string>();
    for (const view of VIEWS) {
      const vm = viewMasks(view, state, sm.body);
      const field = distanceField(vm.volume);
      sm.structure.forEach((mesh, i) => {
        const s = sm.solids.structure[i];
        if (s.part === "support") return;
        const mk = meshMask(view, [mesh], vm.box, vm.cell);
        let worst = 0;
        for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k] && field[k]! > worst) worst = field[k]!;
        if (worst - vm.cell > c.feature * 0.9) bad.add(s.id);
      });
    }
    if (bad.size === 0) return;
    let changed = false;
    for (const s of c.structure) {
      if (!bad.has(s.id)) continue;
      const k = 0.82;
      if (s.kind === "ellipsoid") {
        const n = s.radii.map((x: number) => r4(Math.max(minR, x * k)));
        if (n.some((x: number, i: number) => x !== s.radii[i])) changed = true;
        s.radii = n;
      } else if (s.kind === "capsule") {
        const n = [r4(Math.max(minR, s.r0 * k)), r4(Math.max(minR, s.r1 * k))];
        if (n[0] !== s.r0 || n[1] !== s.r1) changed = true;
        s.r0 = n[0];
        s.r1 = n[1];
      } else if (s.kind === "loft") {
        for (const q of s.sections) {
          const [a, b] = [r4(Math.max(minR, q.rx * k)), r4(Math.max(minR, q.ry * k))];
          if (a !== q.rx || b !== q.ry) changed = true;
          q.rx = a;
          q.ry = b;
        }
      }
    }
    if (!changed) {
      process.stderr.write(`[author-05] 1:${c.scale} structure still outside: ${[...bad].join(", ")}\n`);
      return;
    }
  }
}

const stripRegion = (s: Json): Json => {
  const { region: _r, regions: _rs, maxDenominator: _m, ...rest } = s;
  return rest;
};


const matOf = (c: Ctx, bone: string): string => {
  const s = c.a.solids.find((q) => q.bone === bone) ?? c.a.solids[0]!;
  return s.material;
};

const jointBones = (c: Ctx, ids: string[], byId: Map<string, Json>): string[] => {
  const asset = c.asset;
  const H = asset.dimensions.heightM as number;
  const lim = c.scale <= 50 ? 0 : c.scale <= 100 ? 0.06 * H : c.scale <= 200 ? 0.14 * H : 0.22 * H;
  return ids.filter((id) => (byId.get(id)!.lengthM as number) >= lim && byId.get(id)!.joint !== undefined);
};


export function trimToHull(c: Ctx): void {
  const a = c.a as any;
  const tol0 = c.feature * 0.9;
  const views = ["front", "plan"] as const;
  const fields = views.map((view) => {
    const shapes: Json[] = a.rest[view].filter((e: Json) => e.closed && ["body", "detail", "glazing"].includes(e.shape.role) && e.rings.length > 0);
    const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
    for (const e of shapes) for (const r of e.rings) for (const p of r) {
      b.minX = Math.min(b.minX, p.x);
      b.maxX = Math.max(b.maxX, p.x);
      b.minY = Math.min(b.minY, p.y);
      b.maxY = Math.max(b.maxY, p.y);
    }
    const g = gridFor(b, Math.max(b.maxX - b.minX, b.maxY - b.minY));
    const mask = new Mask(g.box, g.cell);
    for (const e of shapes) mask.fill(e.rings);
    return { view, mask, field: distanceField(mask), cell: g.cell };
  });
  const dist = (f: (typeof fields)[number], p: { x: number; y: number }): number => {
    const i = Math.floor((p.x - f.mask.box.minX) / f.cell), j = Math.floor((p.y - f.mask.box.minY) / f.cell);
    if (i < 0 || j < 0 || i >= f.mask.width || j >= f.mask.height) return 1;
    return f.field[j * f.mask.width + i]!;
  };
  for (const s of c.a.solids as Json[]) {
    if (s.kind !== "loft" || s.role !== "body") continue;
    const fr = a.f[s.bone];
    const secs: Json[] = s.sections;
    for (let i = 0; i < secs.length; i++) {
      const q = secs[i];
      const n = q.exponent ?? 2;
      const valid = (zl: number, ry: number): boolean => {
        for (let t = 0; t < 24; t++) {
          const th = (2 * Math.PI * t) / 24;
          const cs = Math.cos(th), sn = Math.sin(th);
          const w = toWorld(fr, [q.at[0], q.at[1] + q.rx * Math.sign(cs) * Math.pow(Math.abs(cs), 2 / n), zl + ry * Math.sign(sn) * Math.pow(Math.abs(sn), 2 / n)]);
          for (const f of fields) if (dist(f, projectWorld(f.view, w)) > tol0 + f.cell) {
            if (process.env.DEBUG_TRIM === s.id) process.stderr.write(`[trim ${s.id} #${i}] ry ${ry.toFixed(3)} theta ${t} ${f.view} dist ${(dist(f, projectWorld(f.view, w)) * 1000).toFixed(0)}mm at ${w.map((x) => x.toFixed(3)).join(",")}\n`);
            return false;
          }
        }
        return true;
      };
      if (valid(q.at[2], q.ry)) continue;
      let done = false;
      for (let ry = q.ry * 0.88; ry > (c.a as any).minHalf && !done; ry *= 0.88) if (valid(q.at[2], ry)) {
        q.ry = r4(ry);
        done = true;
      }
      if (done) continue;
      const ry = i === 0 || i === secs.length - 1 ? q.ry : Math.max((c.a as any).minHalf, Math.min(q.ry, 0.01));
      for (let d = 0.004; d < 0.5 && !done; d += 0.004) for (const sg of [1, -1]) if (valid(q.at[2] + sg * d, ry)) {
        q.at[2] = r4(q.at[2] + sg * d);
        q.ry = r4(ry);
        done = true;
        break;
      }
    }
  }
}

export const scaleSolid = (s: Json, k: number): Json => {
  const c = JSON.parse(JSON.stringify(s));
  if (c.kind === "loft") for (const q of c.sections) { q.rx = r4(q.rx * k); q.ry = r4(q.ry * k); }
  else if (c.kind === "ellipsoid") c.radii = c.radii.map((x: number) => r4(x * k));
  else if (c.kind === "capsule") { c.r0 = r4(c.r0 * k); c.r1 = r4(c.r1 * k); }
  else throw new Error(`cannot scale ${c.kind}`);
  return c;
};

const fine50 = new Map<string, Json[]>();
export function fallbackRegions(c: Ctx, cfg: Cfg): void {
  const have = new Set<string>(c.thermal.map((t) => t.region));
  const missing = (c.asset.thermal.regions as Json[]).filter((r) => !have.has(r.id));
  if (missing.length === 0) return;
  if (!fine50.has(cfg.slug)) {
    const a = new Author(cfg.slug, 50);
    const c50: Ctx = { a, asset: c.asset, scale: 50, feature: featureM(50, c.asset), body: [], thermal: [], structure: [] };
    cfg.body(c50);
    fine50.set(cfg.slug, a.solids);
  }
  for (const r of missing) {
    for (const s of fine50.get(cfg.slug)!) {
      const rs: string[] = s.regions ?? (s.region === undefined ? [] : [s.region]);
      if (rs[0] !== r.id) continue;
      const { region: _r, regions: _rs, ...rest } = s;
      c.thermal.push({ ...rest, id: `th-${s.id}-s${c.scale}`, region: r.id, role: "texture" });
    }
  }
}

export function emitters(c: Ctx, regionOf: (em: Json) => string): void {
  const H = c.asset.dimensions;
  for (const em of c.asset.thermal.emitters ?? []) {
    const an = (c.asset.anchors ?? []).find((x: Json) => x.id === em.anchor);
    if (an === undefined) continue;
    const region = em.region ?? regionOf(em);
    const base: Json = { id: `th-${em.id}`, bone: an.bone, material: matOf(c, an.bone), tone: 0.3, region, tags: [em.id], transform: { translation: [r4(an.x), r4(an.y), 0] } };
    if (em.kind === "plume") {
      const rx = Math.min(em.diameterM / 2, 0.3 * H.widthM), rz = Math.min(em.diameterM / 2, 0.42 * H.depthM);
      c.thermal.push({ ...base, kind: "ellipsoid", role: "texture", radii: [r4(Math.max(rx, c.feature)), r4(c.feature * 0.55), r4(Math.max(rz, c.feature))] });
    } else {
      const r = Math.max(em.diameterM / 2, c.feature * 0.55);
      c.thermal.push({ ...base, kind: "ellipsoid", role: c.scale >= 200 ? "texture" : "body", radii: [r4(r), r4(r), r4(r)] });
    }
  }
}

export function fillBones(c: Ctx): void {
  const a = c.a as any;
  const have = new Set<string>(c.a.solids.map((s: Json) => s.bone));
  const bones = new Map<string, Json>((c.asset.rig.bones as Json[]).map((b) => [b.id, b]));
  for (const view of ["front", "plan", "elevation"] as const) {
    for (const e of a.rest[view] as Json[]) {
      const sh = e.shape;
      if (!["body", "detail"].includes(sh.role) || have.has(sh.bone) || e.rings.length === 0) continue;
      const b = bones.get(sh.bone)!;
      const pts = e.rings.flat();
      const xs = pts.map((p: Json) => p.x), ys = pts.map((p: Json) => p.y);
      const half = Math.max(c.feature * 0.55, (view === "elevation" ? Math.max(...ys) - Math.min(...ys) : Math.max(...xs) - Math.min(...xs)) / 2 * (view === "plan" ? 0 : 1));
      const lat = view === "plan" ? (Math.max(...ys) - Math.min(...ys)) / 2 : view === "front" ? (Math.max(...xs) - Math.min(...xs)) / 2 : half;
      const r = Math.max(c.feature * 0.55, Math.min(lat, (b.lengthM as number) * 0.6));
      const zc = view === "plan" ? (Math.max(...ys) + Math.min(...ys)) / 2 : view === "front" ? (Math.max(...xs) + Math.min(...xs)) / 2 : 0;
      const s = a.cap(`${sh.bone}-fill`, sh.bone, toWorld(a.f[sh.bone], [0, 0, 0]), toWorld(a.f[sh.bone], [b.lengthM, 0, 0]), r, r, [sh.id], {});
      s.p0[2] = r4(zc - a.f[sh.bone].o[2]);
      s.p1[2] = s.p0[2];
      have.add(sh.bone);
    }
  }
}

export function detailsFromShapes(c: Ctx, opts: { skip?: RegExp; zFor?: (id: string, a: Author, cx: number, cy: number) => number[] } = {}): void {
  const a = c.a as any;
  const regions = new Set<string>((c.asset.thermal.regions as Json[]).map((r) => r.id));
  const seen = new Set<string>();
  for (const e of a.rest.elevation as Json[]) {
    const sh = e.shape;
    if (sh.role !== "detail" || sh.filled === false && !regions.has(sh.region) || !regions.has(sh.region) || seen.has(sh.id) || e.rings.length === 0 || opts.skip?.test(sh.id)) continue;
    seen.add(sh.id);
    const pts = e.rings.flat();
    const xs = pts.map((p: Json) => p.x), ys = pts.map((p: Json) => p.y);
    const [cx, cy] = [(Math.min(...xs) + Math.max(...xs)) / 2, (Math.min(...ys) + Math.max(...ys)) / 2];
    const [rx, ry] = [(Math.max(...xs) - Math.min(...xs)) / 2, (Math.max(...ys) - Math.min(...ys)) / 2];
    if (Math.min(rx, ry) * 2 < c.feature * 1.05) continue;
    let zs: number[] = [];
    for (const v of ["front", "plan"] as const) for (const q of a.rest[v] as Json[]) {
      if (q.shape.role !== "detail" || !(q.shape.id === sh.id || q.shape.id.endsWith(`-${sh.id}`) || q.shape.id.startsWith(`${sh.id}-`))) continue;
      const zz = q.rings.flat().map((p: Json) => (v === "front" ? p.x : p.y));
      if (zz.length > 0) zs.push((Math.min(...zz) + Math.max(...zz)) / 2);
    }
    zs = [...new Set(zs.map((z) => Math.round(z * 1e4) / 1e4))];
    if (zs.length === 0) zs = opts.zFor?.(sh.id, c.a, cx, cy) ?? [0];
    const rz = Math.max(c.feature * 0.55, Math.min(rx, ry) * 0.9);
    zs.forEach((z, k) => {
      c.a.ell(zs.length === 1 ? sh.id : `${sh.id}-${k === 0 ? "a" : "b"}`, sh.bone, [cx, cy, z], [Math.max(rx, c.feature * 0.55), Math.max(ry, c.feature * 0.55), rz], 0, [sh.id], { region: sh.region });
    });
  }
}

export function stdDecisions(nameEn: string, nameDe: string, extraEn: string, extraDe: string): (n: number) => Text[] {
  return (n) => {
    const f = [7.5, 15, 30, 75][[50, 100, 200, 500].indexOf(n)];
    const sec = { 50: "ten", 100: "seven", 200: "six", 500: "five" }[n as 50]!;
    const secDe = { 50: "zehn", 100: "sieben", 200: "sechs", 500: "fünf" }[n as 50]!;
    const fine = n <= 100;
    return [
      { en: `Body: every bone drawn at 1:${n} is a loft with ${sec} sections whose sagittal profile follows the 1:${n} elevation outline of the ${nameEn} and whose lateral width, centre and superellipse exponent are cut from the visual hull of that scale's own elevation, front and plan drawings, trimmed so no section leaves the drawn silhouettes; smallest cross-section ${f} mm. ${fine ? "Eyes and other small details stay as ellipsoids." : "Details below the feature are dropped and left to the thermal skin."} ${extraEn}`, de: `Körper: jede bei 1:${n} gezeichnete Bone ist ein Loft mit ${secDe} Schnitten, dessen senkrechtes Profil der 1:${n}-Ansicht des Tieres (${nameDe}) folgt und dessen seitliche Breite, Mitte und Superellipsen-Exponent aus der Sichthülle der eigenen Ansicht, Vorderansicht und Draufsicht dieses Maßstabs geschnitten und so beschnitten sind, dass kein Schnitt die gezeichneten Silhouetten verlässt; kleinster Querschnitt ${f} mm. ${fine ? "Augen und andere kleine Details bleiben Ellipsoide." : "Details unter der Mindestgröße entfallen und bleiben der thermischen Haut überlassen."} ${extraDe}` },
      { en: `Thermal: one skin solid per body solid is referenced from the body so the geometry is shared${n === 50 ? "; regions with two layers are nested shells, innermost layer first" : ""}; emitters are thin solids at their anchors; regions without a body solid at this scale use thin texture sheets copied from the fine model; estimated areaM2 values were corrected to the exposed model surface.`, de: `Thermik: je Bodykörper ein Hautkörper, vom Body referenziert, sodass die Geometrie geteilt ist${n === 50 ? "; Regionen mit zwei Schichten sind ineinanderliegende Schalen, innerste Schicht zuerst" : ""}; Emitter sind dünne Körper an ihren Ankern; Regionen ohne Bodykörper in diesem Maßstab nutzen dünne Texturbleche aus dem feinen Modell; geschätzte areaM2-Werte wurden auf die exponierte Modelloberfläche korrigiert.` },
      { en: `Structure: ${n === 50 ? "five-section" : n === 100 ? "three-section" : "two-section cylindrical"} members along the centreline of every load-bearing bone, joint balls ${n === 50 ? "at every joint" : n === 100 ? "at the longer bones" : n === 200 ? "at the large joints only" : "at the hip, shoulder and neck base only"}, mass ellipsoids at the point masses, no supports because the asset declares none; sections never fall below ${f * 1.2} mm.`, de: `Tragwerk: ${n === 50 ? "fünfschnittige" : n === 100 ? "dreischnittige" : "zweischnittige zylindrische"} Stäbe entlang der Mittellinie jeder tragenden Bone, Gelenkkugeln ${n === 50 ? "an jedem Gelenk" : n === 100 ? "an den längeren Bones" : n === 200 ? "nur an den großen Gelenken" : "nur an Hüfte, Schulter und Halsansatz"}, Massenellipsoide an den Punktmassen, keine Auflager, weil das Asset keine deklariert; Querschnitte unterschreiten nie ${f * 1.2} mm.` },
    ];
  };
}

export function build(cfg: Cfg, scale: number): Json {
  const { asset } = entryOf(`animals/${cfg.slug}`);
  const rep = asset.representations[String(scale)];
  if (rep?.status !== "authored") return { status: "unavailable", denominator: scale };
  const a = new Author(cfg.slug, scale);
  const c: Ctx = { a, asset, scale, feature: featureM(scale, asset), body: [], thermal: [], structure: [] };
  cfg.body(c);
  fillBones(c);
  trimToHull(c);
  (cfg.thermal ?? ((x) => { thermalFromBody(x); fallbackRegions(x, cfg); emitters(x, cfg.emitterRegion ?? (() => "")); }))(c);
  (cfg.structure ?? ((x) => structureGeneric(x, cfg)))(c);
  const body = a.solids.map((s) => {
    const { region: _r, regions: _rs, maxDenominator: _m, ...rest } = s;
    return rest;
  });
  return { status: "authored", denominator: scale, body, thermal: c.thermal, structure: c.structure, decisions: cfg.decisions(scale) };
}

export function run(cfg: Cfg): void {
  const only = process.argv.slice(2).map(Number).filter((n) => !Number.isNaN(n));
  const { entry } = entryOf(`animals/${cfg.slug}`);
  const path = sourcePath(entry);
  const volumes: Json = {};
  let existing: Json = {};
  try {
    existing = JSON.parse(readFileSync(path, "utf8")).volumes ?? {};
  } catch {
    existing = {};
  }
  for (const s of SCALES) {
    if (only.length > 0 && !only.includes(s)) {
      volumes[String(s)] = existing[String(s)] ?? build(cfg, s);
      continue;
    }
    volumes[String(s)] = build(cfg, s);
  }
  writeJson(path, { volumes }, true);
  for (const s of SCALES) {
    const v = volumes[String(s)];
    process.stdout.write(`1:${s} ${v.status}${v.body ? ` body ${v.body.length} thermal ${v.thermal.length} structure ${v.structure.length}` : ""}\n`);
  }
}

export { round5 };
export type { V3 };
