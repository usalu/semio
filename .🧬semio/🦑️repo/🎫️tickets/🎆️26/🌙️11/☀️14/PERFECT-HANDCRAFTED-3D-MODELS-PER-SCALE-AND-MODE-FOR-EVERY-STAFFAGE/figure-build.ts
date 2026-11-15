/** 🏗️ Builds body, thermal and structure of one human asset at one scale from the figure kit (`figure-lib.ts`) and the asset's own drawings.
 * Usage: `bun figure-build.ts <category/slug> [--scale N] [--config path] [--write] [--from-existing] [--refit]` (default: all four scales; config default `figure/<slug>.json`, absent = defaults). Prints counts and what was built; `--write` saves body, thermal, structure and decisions of the scale (props and emitters of the existing lists are kept).
 * @see ./figure.md */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { flag, has, openCtx, saveLists, type Ctx, type Json } from "./kit-lib.ts";
import { anatRange, boneInfos, drawnGroups, fitStations, layerShells, layersThickness, loftStations, partOf, personOf, rangeOf, rangeUnion, ribbonStations, skinUnder, at, type BoneInfo, type Config, type Group, type PartName, type Person, type Stn, type V3 } from "./figure-lib.ts";

const TICKET = import.meta.dir;
const tokens = (id: string): string[] => id.split("-");
const FEATURES = { test: (id: string): boolean => tokens(id).some((t) => ["button", "pocket", "lapel", "buckle", "flap", "strap", "zip", "lace", "stripe", "logo", "seam", "eye", "ear", "nose", "thumb", "mouth", "brow"].includes(t)) };
const OWN = { test: (id: string): boolean => tokens(id).some((t) => ["cuff", "collar", "hem", "hood", "brim", "visor"].includes(t)) };

type Lod = { n: number; landmarks: boolean; tol: number; skin: boolean; details: boolean; joints: boolean; fine: boolean };
const LOD: Record<number, Lod> = { 50: { n: 11, landmarks: true, tol: 0.18, skin: true, details: true, joints: true, fine: true }, 100: { n: 8, landmarks: false, tol: 0.25, skin: false, details: true, joints: true, fine: false }, 200: { n: 6, landmarks: false, tol: 0.4, skin: false, details: false, joints: false, fine: false }, 500: { n: 6, landmarks: false, tol: 0.4, skin: false, details: false, joints: false, fine: false } };

const bboxOf = (rings: { x: number; y: number }[][]): [number, number, number, number] => {
  const xs = rings.flat().map((q) => q.x);
  const ys = rings.flat().map((q) => q.y);
  return [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
};
const area = (ring: { x: number; y: number }[]): number => Math.abs(ring.reduce((a, q, i) => a + (ring[(i + 1) % ring.length]!.x * q.y - q.x * ring[(i + 1) % ring.length]!.y), 0)) / 2;

type Piece = { sol: Json; st: Stn[]; region: string };
type Main = { bone: string; part: PartName; region: string; kind: string; st: Stn[]; sol: Json; hidden: boolean };
export type Class = "anatomy" | "garment" | "hair" | "cap";
export type Built = { body: Json[]; thermal: Json[]; structure: Json[]; decisions: Json[]; notes: string[]; cls: Map<string, Class>; capIds: Set<string> };

/** 🦵️ Joint cap factor per part (the ball end that rounds the proximal end of a part where it meets its parent: hip, knee, ankle, shoulder (deltoid), elbow, wrist): section scale of the cap against the first station, and the share of the section it rises along the bone. */
const CAP: Partial<Record<PartName, [number, number]>> = { thigh: [1.0, 0.7], shank: [1.0, 0.75], foot: [1.0, 0.6], uarm: [1.04, 0.8], farm: [1.0, 0.75], hand: [1.0, 0.7], neck: [1.0, 0.7] };

export function build(c: Ctx, cfg: Config, reach?: Record<string, number>): Built {
  const lod = LOD[c.scale]!;
  const f = c.feature;
  const fl = 0.502 * f;
  const infos = boneInfos(c);
  const groups = drawnGroups(c, infos);
  const bonesById = Object.fromEntries((c.m.rig.bones as Json[]).map((b) => [b.id, b]));
  const person = personOf(cfg, bonesById);
  const regions: Record<string, Json> = Object.fromEntries((c.m.thermal.regions as Json[]).map((r) => [r.id, r]));
  const notes: string[] = [];
  const geometry = c.m.representations[String(c.scale)].geometry as Record<string, Json[]>;
  const shapeInfo: Record<string, Json> = {};
  for (const list of Object.values(geometry)) for (const s of list) shapeInfo[s.id] ??= s;
  for (const [bone, byRegion] of Object.entries(groups)) {
    const blank = byRegion[""];
    if (blank === undefined) continue;
    const info = infos[bone]!;
    const cands = Object.values(byRegion).filter((g) => g.region !== "" && regions[g.region]?.model === "body" && regions[g.region]!.kind !== "hair");
    const skinRegion = Object.values(regions).find((r) => r.model === "body" && r.kind === "skin" && (r.body?.segment ?? "").startsWith(info.part === "head" || info.part === "neck" ? "head" : info.part))?.id;
    const target = cands.length === 1 ? cands[0]!.region : info.part === "head" || info.part === "neck" || info.part === "hand" ? skinRegion : undefined;
    if (target === undefined) {
      delete byRegion[""];
      continue;
    }
    const g = (byRegion[target] ??= { bone, region: target, ids: new Set(), kind: regions[target]!.kind, shapes: [] });
    for (const id of blank.ids) g.ids.add(id);
    g.shapes.push(...blank.shapes);
    delete byRegion[""];
  }
  const SEGMENT: Record<PartName, string> = { head: "head", neck: "head", torso: "trunk", pelvis: "trunk", uarm: "arm", farm: "arm", hand: "hand", thigh: "leg", shank: "leg", foot: "foot", toe: "foot" };
  const skinOfPart = (part: PartName): Json | undefined => Object.values(regions).find((r) => r.model === "body" && r.kind === "skin" && r.body?.segment === SEGMENT[part]);
  for (const [bone, info] of Object.entries(infos)) {
    const sr = skinOfPart(info.part);
    if (groups[bone] !== undefined || cfg.skip?.includes(bone) || sr === undefined || info.part === "toe") continue;
    groups[bone] = { [sr.id]: { bone, region: sr.id, ids: new Set(), kind: "skin", shapes: [] } };
    notes.push(`bone ${bone} has no drawn closed shape at 1:${c.scale}: skin loft from the template, clamped to everything drawn`);
  }
  const clip = { shapes: Object.values(groups).flatMap((byRegion) => Object.values(byRegion).flatMap((g) => g.shapes.filter((x) => x.role === "body"))) };
  const cls = new Map<string, Class>();
  const capIds = new Set<string>();
  const mains: Main[] = [];
  const body: Json[] = [];
  const thermal: Json[] = [];
  const outer: Record<string, Piece[]> = {};
  const mainSt: Record<string, Stn[]> = {};
  const gap = (r: Json): number => cfg.gap?.[r.id] ?? (r.kind === "clothing" ? (/coat|jacket|hoodie|top|shirt|parka/.test(r.id) ? 0.004 : 0.002) : 0);
  const matOf = (ids: Iterable<string>, reg: Json): { material: string; tone: number } => {
    for (const id of ids) if (shapeInfo[id]?.material) return { material: shapeInfo[id].material, tone: shapeInfo[id].tone ?? 0.5 };
    return { material: reg.material ?? "skin", tone: 0.5 };
  };
  const isMain = (id: string): boolean => !FEATURES.test(id) && !OWN.test(id) && shapeInfo[id]?.role !== "texture" && shapeInfo[id]?.role !== "ground";
  const hasElev = (g: Group): boolean => g.shapes.some((x) => x.view === "elevation");
  const mm = (sol: Json, region: string, st: Stn[]): void => void (outer[sol.bone] ??= []).push({ sol, st, region });
  const usable = (g: Group, info: BoneInfo): boolean => g.shapes.some((x) => x.view === "elevation" || x.view === "front" || (x.view === "plan" && Math.abs(info.frame.u[1]) < Math.abs(info.frame.u[0])));
  const regionsOf = (bone: string): Group[] => {
    const all = Object.values(groups[bone] ?? {}).filter((g) => regions[g.region]?.model === "body");
    const use = all.filter((g) => usable(g, infos[bone]!));
    const withElev = use.filter(hasElev);
    if (use.length > 0) return withElev.length > 0 ? withElev : use;
    const first = all.find((g) => regions[g.region]!.kind !== "hair") ?? all[0];
    return first === undefined ? [] : [{ bone, region: first.region, ids: new Set(), kind: first.kind, shapes: [] }];
  };

  for (const bone of Object.keys(groups)) {
    const info = infos[bone]!;
    if (cfg.skip?.includes(bone)) continue;
    const part = info.part;
    const mine = regionsOf(bone);
    const boneAll = { shapes: mine.flatMap((x) => x.shapes) };
    const skinG = mine.find((g) => regions[g.region]!.kind === "skin");
    for (const g of mine) {
      const reg = regions[g.region]!;
      const kind = reg.kind as string;
      const ids = new Set([...g.ids].filter((id) => isMain(id) && (c.scale > 50 || !/thumb/.test(id))));
      if (c.scale > 50) for (const id of g.ids) if (/(^|-)thumb(-|$)/.test(id)) ids.add(id);
      if (ids.size === 0 && g.shapes.length > 0) continue;
      const drawnR = (part === "hand" ? rangeUnion(info, g, ids) : null) ?? rangeOf(info, g, ids) ?? rangeOf(info, boneAll) ?? anatRange(part, info.L, person.statureM);
      const mat = matOf(ids, reg);
      const common = { bone, material: mat.material, tone: mat.tone };
      if (kind === "hair") {
        const tail = /pony|braid|plait|(^|-)bun(-|$)|(^|-)tail(-|$)/;
        const tailIds = new Set([...ids].filter((id) => tail.test(id)));
        const mainIds = new Set([...ids].filter((id) => !tail.test(id)));
        const body0 = mainIds.size > 0 ? mainIds : ids;
        const endsH = mainIds.size > 0 ? rangeOf(info, g, mainIds) ?? drawnR : drawnR;
        const st = fitStations(info, g, { n: Math.max(lod.n, 14), feature: f, tol: Math.min(lod.tol, 0.2) * f, landmarks: false, only: body0, person, part, ends: endsH, e: 2.2 });
        if (st.length >= 2) {
          const sol = loftStations({ id: `${bone}-hair`, ...common }, st);
          body.push(sol);
          cls.set(sol.id, "hair");
          thermal.push(...layerShells(sol, reg, sol.id));
        }
        if (mainIds.size > 0 && tailIds.size > 0) {
          const rb = ribbonStations(info, g, tailIds, f, lod.n + 2, fl);
          if (rb.length >= 3) {
            const sol = loftStations({ id: `${bone}-hair-tail`, ...common }, rb);
            body.push(sol);
            cls.set(sol.id, "hair");
            thermal.push(...layerShells(sol, reg, sol.id));
          }
        }
        continue;
      }
      let anat = anatRange(part, info.L, person.statureM);
      if (part === "head") anat = (skinG !== undefined ? rangeOf(info, skinG, new Set([...skinG.ids].filter(isMain))) : null) ?? drawnR;
      const lo = drawnR[0] >= anat[0] - 1.5 * f ? drawnR[0] : Math.max(drawnR[0], anat[0]);
      const hi = drawnR[1] <= anat[1] + 1.5 * f ? drawnR[1] : Math.min(drawnR[1], anat[1]);
      const isShoe = (part === "foot" || part === "toe") && kind !== "skin";
      const pieces: { id: string; ends: [number, number]; ext: boolean }[] = [];
      if (hi - lo > 0.5 * f) pieces.push({ id: kind === "skin" ? bone : isShoe ? bone : `${bone}-${g.region}`, ends: [lo, hi], ext: false });
      if (c.scale <= 100 || hi - lo <= 0.5 * f) {
        if (drawnR[0] < anat[0] - 1.5 * f) pieces.push({ id: `${bone}-${g.region}-lo`, ends: [drawnR[0], Math.min(anat[0], drawnR[1])], ext: true });
        if (drawnR[1] > anat[1] + 1.5 * f) pieces.push({ id: `${bone}-${g.region}-hi`, ends: [Math.max(anat[1], drawnR[0]), drawnR[1]], ext: true });
      } else {
        if (drawnR[0] < anat[0] - 1.5 * f) pieces.push({ id: `${bone}-${g.region}-lo`, ends: [drawnR[0], anat[0]], ext: true });
        if (drawnR[1] > anat[1] + 1.5 * f) pieces.push({ id: `${bone}-${g.region}-hi`, ends: [anat[1], drawnR[1]], ext: true });
      }
      let main: Stn[] | undefined;
      const fingered = part === "hand" && kind === "skin" && lod.fine && pieces.length > 0;
      if (fingered) pieces[0]!.ends = [pieces[0]!.ends[0], pieces[0]!.ends[0] + 0.56 * (pieces[0]!.ends[1] - pieces[0]!.ends[0])];
      for (const pc of pieces) {
        const twist: [number, number] | undefined = part === "farm" ? [0, 28] : undefined;
        const st = fitStations(info, g, { n: pc.ext ? 4 : lod.n, feature: f, tol: lod.tol * f, landmarks: lod.landmarks && !pc.ext, only: ids, person, part, ends: pc.ends, anat: pc.ext ? undefined : anat, twist, clip, e: pc.ext ? main?.[pc.ends[0] <= anat[0] ? 0 : main.length - 1]?.e : undefined });
        if (st.length < 2) continue;
        if (!pc.ext) {
          main = st;
          mainSt[bone] ??= st;
        }
        if (isShoe && !pc.ext && c.scale <= 100) {
          const sole = cfg.shoeSoleM ?? 0.022;
          const upper: Stn[] = [];
          const soleSt: Stn[] = [];
          const span = Math.max(1e-6, st[st.length - 1]!.s - st[0]!.s);
          for (const q of st) {
            const k = Math.min(sole * (0.6 + 0.4 * (1 - (q.s - st[0]!.s) / span)), q.ry * 1.3);
            const lowV = q.cv - q.ry;
            soleSt.push({ ...q, cv: lowV + k / 2, ry: Math.max(k / 2, fl), e: 3.2 });
            const top = q.cv + q.ry;
            const bot = lowV + k - Math.min(0.003, 0.3 * k);
            upper.push({ ...q, cv: (top + bot) / 2, ry: Math.max((top - bot) / 2, fl) });
          }
          const up = loftStations({ id: `${bone}-upper`, ...common }, upper);
          const so = loftStations({ id: `${bone}-sole`, bone, material: mat.material, tone: Math.max(0.05, mat.tone * 0.4) }, soleSt);
          body.push(up, so);
          cls.set(up.id, "garment");
          cls.set(so.id, "garment");
          mm(up, g.region, st);
          thermal.push({ id: `th-${bone}-sole`, of: so.id, region: g.region, ...((reg.layers ?? []).length > 0 ? { layer: reg.layers[reg.layers.length - 1].id } : {}) });
          continue;
        }
        const hairy = Object.values(regions).some((r) => r.model === "body" && r.kind === "hair");
        if (kind === "skin" && part === "head" && !pc.ext && hairy && c.scale < 500 && st.length >= 3) {
          const sp = anat[0] + 0.5 * (anat[1] - anat[0]);
          const lower = [...st.filter((q) => q.s < sp - 1e-6), at(st, sp)];
          const upper = [at(st, sp), ...st.filter((q) => q.s > sp + 1e-6)];
          if (lower.length >= 2 && upper.length >= 2) {
            const face = loftStations({ id: pc.id, ...common }, lower);
            const scalp = loftStations({ id: `${pc.id}-scalp`, ...common }, upper);
            body.push(face, scalp);
            cls.set(face.id, "anatomy");
            cls.set(scalp.id, "anatomy");
            thermal.push(...layerShells(face, reg, face.id));
            continue;
          }
        }
        const sol = loftStations({ id: pc.id, ...common }, st);
        body.push(sol);
        cls.set(sol.id, kind === "skin" ? "anatomy" : "garment");
        if (!pc.ext) mains.push({ bone, part, region: g.region, kind, st, sol, hidden: false });
        if (kind === "skin") thermal.push(...layerShells(sol, reg, sol.id));
        else mm(sol, g.region, st);
      }
      if (fingered && main !== undefined) {
        const palm = main[main.length - 1]!;
        const tip = Math.max(hi, Math.min(reach?.[bone] ?? hi, anat[1]));
        const mid = fitStations(info, g, { n: 2, feature: f, landmarks: false, only: ids, person, part, ends: [palm.s, tip] });
        const m1 = mid.length > 0 ? at(mid, (palm.s + tip) / 2) : palm;
        const pitch = (2 * palm.rx) / 4.4;
        const r = Math.max(fl, Math.min(Math.max(m1.ry, palm.ry * 0.8) * 0.95, pitch * 0.48));
        [0.95, 1, 0.93, 0.8].forEach((lf, i) => {
          const w = m1.cw + (i - 1.5) * pitch * (1 + 0.04 * Math.abs(i - 1.5));
          const len = lf * (tip - palm.s);
          const curl = info.ant * 0.3 * r;
          const knuckle: V3 = [palm.s - 0.004, palm.cv, w];
          const pip: V3 = [palm.s + 0.5 * len, m1.cv + 0.4 * curl, w];
          const dip: V3 = [palm.s + len - r, m1.cv + curl, w];
          const parts = [
            { n: "a", p0: knuckle, p1: pip, r0: r, r1: r * 0.92 },
            { n: "b", p0: pip, p1: dip, r0: r * 0.92, r1: r * 0.8 },
          ];
          for (const q of parts) {
            const sol = { id: `${bone}-finger-${i + 1}${q.n === "a" ? "" : "-tip"}`, kind: "capsule", bone, role: "body", ...common, p0: q.p0, p1: q.p1, r0: q.r0, r1: q.r1 };
            body.push(sol);
            cls.set(sol.id, "anatomy");
            thermal.push(...layerShells(sol, reg, sol.id));
          }
        });
      }
      for (const fid of g.ids) {
        if (!OWN.test(fid) || FEATURES.test(fid) || (c.scale > 100 && !tokens(fid).some((t) => ["hood", "brim", "visor"].includes(t)))) continue;
        const st = fitStations(info, g, { n: 3, feature: f, tol: 0.3 * f, landmarks: false, only: new Set([fid]), person, part, ends: rangeOf(info, g, new Set([fid])) ?? drawnR, ref: main, e: 2.6 });
        if (st.length < 2 || Math.min(...st.map((q) => Math.min(q.rx, q.ry))) < 0.9 * fl) continue;
        const sh = loftStations({ id: `${bone}-${fid}`, bone, material: shapeInfo[fid]?.material ?? mat.material, tone: shapeInfo[fid]?.tone ?? mat.tone }, st);
        body.push(sh);
        cls.set(sh.id, "garment");
        mm(sh, g.region, st);
      }
    }
    if (lod.details) {
      const fe = features(bone, info, boneAll, outer[bone], shapeInfo, fl);
      for (const x of fe) cls.set(x.id, "anatomy");
      body.push(...fe);
    }
  }

  for (const [bone, list] of Object.entries(outer)) {
    const info = infos[bone]!;
    for (const o of list) thermal.push(...layerShells(o.sol, regions[o.region]!, o.sol.id, c.scale === 50));
    const bare = regionsOf(bone).some((g) => regions[g.region]!.kind === "skin");
    if (!lod.skin || bare || list.length === 0) continue;
    const m0 = list[0]!;
    const inner = layersThickness(regions[m0.region]) + gap(regions[m0.region]!);
    const anat = anatRange(info.part, info.L, person.statureM);
    const st = skinUnder(m0.st, info.part, person, inner, fl, (s) => Math.min(1, Math.max(0, (s - anat[0]) / Math.max(1e-6, anat[1] - anat[0]))));
    const under = loftStations({ id: `${bone}-skin`, bone, material: "skin", tone: 0.3 }, st);
    body.push(under);
    cls.set(under.id, "anatomy");
    mains.push({ bone, part: info.part, region: "", kind: "skin", st, sol: under, hidden: true });
    const sr = skinOfPart(info.part);
    const spans: [number, number][] = [];
    const shod = info.part === "foot" || info.part === "toe";
    if (!shod && st[0]!.s - anat[0] > 1.5 * f) spans.push([anat[0], st[0]!.s]);
    if (!shod && anat[1] - st[st.length - 1]!.s > 1.5 * f) spans.push([st[st.length - 1]!.s, anat[1]]);
    spans.forEach(([a, b], k) => {
      const bst = fitStations(info, { shapes: [] }, { n: 5, feature: f, tol: lod.tol * f, landmarks: false, person, part: info.part, ends: [a, b], anat, clip });
      if (bst.length < 2) return;
      const bare = loftStations({ id: `${bone}-skin-bare${k}`, bone, material: "skin", tone: 0.3 }, bst);
      body.push(bare);
      cls.set(bare.id, "anatomy");
      if (sr !== undefined) thermal.push(...layerShells(bare, sr, bare.id));
      notes.push(`bone ${bone}: ${(b - a).toFixed(3)} m of the bone lie outside its garment: bare skin piece ${bare.id}`);
    });
  }

  if (lod.joints && process.env.FIGURE_NOCAP === undefined) {
    const sizeK = person.statureM / 1.75;
    for (const e of mains) {
      const k = CAP[e.part];
      const q = e.part === "neck" ? e.st[e.st.length - 1] : e.st[0];
      if (k === undefined || q === undefined || (e.part === "thigh" && e.kind !== "skin") || e.kind === "surface" || /-(lo|hi)$/.test(e.sol.id) || Math.abs(q.s - (e.part === "neck" ? infos[e.bone]!.L : 0)) > 0.012 * sizeK + 1.5 * f) continue;
      const rv = Math.max(fl, q.ry * k[0]);
      const rw = Math.max(fl, q.rx * k[0]);
      const id = `${e.sol.id}-cap`;
      const cap = { id, kind: "ellipsoid", bone: e.bone, role: "body", material: e.sol.material, tone: e.sol.tone, radii: [Math.max(fl, k[1] * Math.min(rv, rw)), rv, rw], transform: { translation: [q.s, q.cv, q.cw] } };
      body.push(cap);
      cls.set(id, e.hidden || e.kind === "skin" ? "anatomy" : "cap");
    }
  }

  const covered = new Set(thermal.map((t) => t.region));
  for (const reg of Object.values(regions)) {
    if (reg.model !== "body" || covered.has(reg.id)) continue;
    const bone = reg.kind === "hair" ? "head" : reg.kind === "clothing" && (reg.body?.segment === "head" || /scarf|collar|hood/.test(reg.id)) ? "neck" : undefined;
    const st = bone === undefined ? undefined : mainSt[bone];
    if (bone === undefined || st === undefined) continue;
    const top = reg.kind === "hair" ? st.filter((q) => q.s >= st[0]!.s + 0.5 * (st[st.length - 1]!.s - st[0]!.s)) : st;
    const k = reg.kind === "hair" ? 0.003 : 0.012;
    const sol = loftStations({ id: `th-${reg.id}`, bone, material: reg.material ?? "skin", tone: 0.4 }, top.map((q) => ({ ...q, rx: q.rx + k, ry: q.ry + k })));
    thermal.push({ ...sol, region: reg.id });
    cls.set(sol.id, reg.kind === "hair" ? "hair" : "anatomy");
    notes.push(`region ${reg.id} has no drawn shape at 1:${c.scale}: thermal solid from the ${bone} template`);
  }
  const dedupe = new Set<string>();
  for (const s of body) {
    if (dedupe.has(s.id)) throw new Error(`duplicate body id ${s.id}`);
    dedupe.add(s.id);
  }
  const preserved = preserve(c, regions, infos, cfg);
  body.push(...preserved.body);
  const bodyIds = new Set(body.map((s) => s.id));
  thermal.push(...preserved.thermal.filter((t) => t.of === undefined || bodyIds.has(t.of)));
  const structure = structureOf(c, infos, mainSt, person, fl, bonesById);
  structure.push(...preserved.structure);
  return { body, thermal, structure, decisions: decide(c, body, thermal, structure, lod, person), notes, cls, capIds };
}

type Shp = Group["shapes"][number];
function features(bone: string, info: BoneInfo, all: { shapes: Shp[] }, pieces: Piece[] | undefined, shapeInfo: Record<string, Json>, fl: number): Json[] {
  const out: Json[] = [];
  const fr = info.frame;
  const has = (sh: Shp, tok: string): boolean => tokens(sh.id).includes(tok);
  const pick = (tok: string, view: string): Shp[] => all.shapes.filter((sh) => sh.view === view && has(sh, tok));
  const local = (x: number, y: number): [number, number] => [(x - fr.o[0]) * fr.u[0]! + (y - fr.o[1]) * fr.u[1]!, (x - fr.o[0]) * fr.v[0]! + (y - fr.o[1]) * fr.v[1]!];
  const halfUV = (b: [number, number, number, number]): [number, number] => {
    const hx = (b[1] - b[0]) / 2;
    const hy = (b[3] - b[2]) / 2;
    return [hx * Math.abs(fr.u[0]!) + hy * Math.abs(fr.u[1]!), hx * Math.abs(fr.v[0]!) + hy * Math.abs(fr.v[1]!)];
  };
  const mat = (sh: Shp): { material: string; tone: number } => ({ material: shapeInfo[sh.id]?.material ?? "skin", tone: shapeInfo[sh.id]?.tone ?? 0.3 });
  const ellip = (id: string, sh: Shp, c3: V3, r: V3): void => void out.push({ id, kind: "ellipsoid", bone, role: "detail", ...mat(sh), radii: r.map((x) => Math.max(x, fl)), transform: { translation: c3 } });
  for (const sh of pick("nose", "elevation")) {
    const b = bboxOf(sh.rings);
    const [u, v] = local((b[0] + b[1]) / 2, (b[2] + b[3]) / 2);
    const [hu, hv] = halfUV(b);
    ellip(`${bone}-nose`, sh, [u, v, 0], [hu, hv, Math.min(0.013, 0.7 * hu)]);
  }
  const earsE = pick("ear", "elevation")[0];
  for (const sh of pick("ear", "front")) {
    if (earsE === undefined) break;
    const b = bboxOf(earsE.rings);
    const [u, v] = local((b[0] + b[1]) / 2, (b[2] + b[3]) / 2);
    const [hu, hv] = halfUV(b);
    const bf = bboxOf(sh.rings);
    const cw = (bf[0] + bf[1]) / 2 - fr.o[2];
    ellip(`${bone}-${sh.id}`, sh, [u, v, cw], [hu, hv, Math.max(fl, (bf[1] - bf[0]) / 2)]);
  }
  const eyeE = pick("eye", "elevation")[0];
  for (const sh of pick("eye", "front")) {
    if (eyeE === undefined) break;
    const b = bboxOf(eyeE.rings);
    const [u, v] = local((b[0] + b[1]) / 2, (b[2] + b[3]) / 2);
    const [hu, hv] = halfUV(b);
    const bf = bboxOf(sh.rings);
    ellip(`${bone}-${sh.id}`, sh, [u, v, (bf[0] + bf[1]) / 2 - fr.o[2]], [hu, hv, Math.max(fl, (bf[1] - bf[0]) / 2)]);
  }
  for (const sh of pick("thumb", "elevation")) {
    const pts = sh.rings[0]!;
    let best: [number, number, number] = [0, 0, 0];
    for (let i = 0; i < pts.length; i++) for (let j = i + 1; j < pts.length; j++) {
      const d = Math.hypot(pts[i]!.x - pts[j]!.x, pts[i]!.y - pts[j]!.y);
      if (d > best[0]) best = [d, i, j];
    }
    const [d, i, j] = best;
    const [p0, p1] = [local(pts[i]!.x, pts[i]!.y), local(pts[j]!.x, pts[j]!.y)];
    const r = Math.max(fl, area(pts) / (2 * Math.max(d, 1e-6)));
    const palm = pieces?.[0]?.st ?? [];
    const q = palm.length > 0 ? at(palm, (p0[0] + p1[0]) / 2) : { rx: 0.04, cw: 0, cv: 0, ry: 0.02 };
    const medial = info.side === "l" ? -1 : 1;
    const w = q.cw + medial * Math.max(0, q.rx - r) * 0.55;
    const [a, b] = p0[0] <= p1[0] ? [p0, p1] : [p1, p0];
    out.push({ id: `${bone}-${sh.id}`, kind: "capsule", bone, role: "body", ...mat(sh), p0: [a[0], a[1], w], p1: [b[0], b[1], w], r0: r * 1.15, r1: r * 0.85 });
  }
  return out;
}

function preserve(c: Ctx, regions: Record<string, Json>, infos: Record<string, BoneInfo>, cfg: Config): { body: Json[]; thermal: Json[]; structure: Json[] } {
  const propRegions = new Set(Object.values(regions).filter((r) => r.model !== "body").map((r) => r.id));
  const thermal: Json[] = (c.vol.thermal as Json[]).filter((t) => propRegions.has(t.region) || (t.tags ?? []).includes("emitter"));
  const refs = new Set(thermal.map((t) => t.of).filter(Boolean));
  const body: Json[] = (c.vol.body as Json[]).filter((s) => infos[s.bone] === undefined || refs.has(s.id) || (cfg.keep ?? []).includes(s.id));
  const structure: Json[] = (c.vol.structure as Json[]).filter((s) => infos[s.member ?? s.bone] === undefined);
  return { body, thermal, structure };
}

const BONE_R: Record<string, [number, number][]> = {
  thigh: [[0, 0.02], [0.12, 0.016], [0.85, 0.016], [1, 0.026]],
  shank: [[0, 0.026], [0.1, 0.02], [0.7, 0.013], [1, 0.021]],
  uarm: [[0, 0.024], [0.15, 0.012], [0.85, 0.012], [1, 0.019]],
  farm: [[0, 0.014], [0.5, 0.013], [1, 0.016]],
  hand: [[0, 0.011], [1, 0.008]],
  foot: [[0, 0.02], [0.5, 0.017], [1, 0.012]],
  toe: [[0, 0.011], [1, 0.007]],
  neck: [[0, 0.018], [1, 0.015]],
  torso: [[0, 0.02], [0.5, 0.017], [1, 0.015]],
  pelvis: [[0, 0.05], [1, 0.05]],
  head: [[0, 0.03], [1, 0.03]],
};
const lin = (rows: [number, number][], t: number): number => {
  for (let i = 0; i + 1 < rows.length; i++) if (t <= rows[i + 1]![0]) return rows[i]![1] + ((rows[i + 1]![1] - rows[i]![1]) * (t - rows[i]![0])) / (rows[i + 1]![0] - rows[i]![0]);
  return rows[rows.length - 1]![1];
};

const JOINT_R: Record<string, number> = { thigh: 0.026, shank: 0.027, uarm: 0.026, farm: 0.019, hand: 0.013, foot: 0.02, neck: 0.02, head: 0.02, torso: 0.026 };

function structureOf(c: Ctx, infos: Record<string, BoneInfo>, mainSt: Record<string, Stn[]>, person: Person, fl: number, bones: Record<string, Json>): Json[] {
  const out: Json[] = [];
  const sizeK = person.statureM / 1.75;
  for (const [bone, info] of Object.entries(infos)) {
    const st = mainSt[bone] ?? fitStations(info, { shapes: [] }, { n: 4, feature: c.feature, landmarks: false, person, part: info.part, ends: anatRange(info.part, info.L, person.statureM) });
    if (st.length < 2) continue;
    const s0 = st[0]!.s;
    const s1 = Math.max(st[st.length - 1]!.s, s0 + 2.3 * fl);
    const nSec = c.scale === 50 ? 5 : c.scale === 100 ? 4 : c.scale === 200 ? 3 : 2;
    const secs: Stn[] = [];
    for (let i = 0; i < nSec; i++) {
      const t = nSec === 1 ? 0 : i / (nSec - 1);
      const q = at(st, s0 + (s1 - s0) * (0.03 + 0.94 * t));
      const rb = lin(BONE_R[info.part]!, t) * sizeK;
      const lim = 0.7;
      const rx = Math.max(fl, Math.min(info.part === "pelvis" ? 0.1 * sizeK : info.part === "head" ? 0.07 * sizeK : rb, lim * q.rx));
      const ry = Math.max(fl, Math.min(info.part === "pelvis" ? 0.045 * sizeK : info.part === "head" ? 0.08 * sizeK : rb, lim * q.ry));
      const post = info.part === "torso" ? -info.ant * 0.3 * q.ry : 0;
      secs.push({ s: s0 + (s1 - s0) * (0.03 + 0.94 * t), cv: q.cv + post, cw: q.cw, rx: info.part === "head" ? rx : Math.min(rx, ry === 0 ? rx : Math.max(rx, ry)), ry, e: info.part === "pelvis" ? 3 : 2 });
    }
    if (info.part === "head") {
      const q = at(st, s0 + (s1 - s0) * 0.62);
      out.push({ id: `member-${bone}`, kind: "ellipsoid", bone, material: "skin", role: "body", tone: 0.5, radii: [Math.max(fl, Math.min(0.09 * sizeK, 0.66 * q.ry * 1.1)), Math.max(fl, 0.72 * q.ry), Math.max(fl, 0.72 * q.rx)], transform: { translation: [q.s, q.cv, q.cw] }, member: bone, part: "member" });
    } else out.push({ ...loftStations({ id: `member-${bone}`, bone, material: "skin", tone: 0.5 }, secs), member: bone, part: "member" });
    const lodJ = c.scale <= 100;
    const jr = JOINT_R[info.part];
    if (lodJ && jr !== undefined && bones[bone]?.parent && infos[bones[bone].parent] !== undefined) {
      const u = 0 >= s0 && 0 <= s1 ? 0 : s0 + 0.02 * (s1 - s0);
      const q = at(st, u);
      const r = Math.max(fl, Math.min(jr * sizeK, 0.8 * Math.min(q.rx, q.ry)));
      out.push({ id: `joint-${bone}`, kind: "ellipsoid", bone, material: "skin", role: "body", tone: 0.4, radii: [r, r, r], transform: { translation: [u, q.cv, q.cw] }, member: bone, part: "joint" });
    }
    const total = (Object.values(bones) as Json[]).reduce((a, b) => a + (b.mass?.kg ?? 0), 0);
    const kg = bones[bone]?.mass?.kg ?? 0;
    if (total > 0 && kg / total >= 0.1) {
      const u = Math.min(s1, Math.max(s0, bones[bone]!.mass.comX));
      const q = at(st, u);
      const L = bones[bone]!.lengthM;
      const r: [number, number, number] = [Math.max(fl, 0.16 * L), Math.max(fl, 0.5 * q.ry), Math.max(fl, 0.5 * q.rx)];
      const base = { id: `mass-${bone}`, bone, material: "skin", role: "detail", tone: 0.2, member: bone, part: "mass", transform: { translation: [u, q.cv, q.cw] } };
      out.push(c.scale <= 100 ? { ...base, kind: "ellipsoid", radii: r } : { ...base, kind: "box", size: r.map((x) => 2 * x) });
    }
  }
  return out;
}

function decide(c: Ctx, body: Json[], thermal: Json[], structure: Json[], lod: Lod, person: Person): Json[] {
  const n = (l: Json[]): number => l.length;
  const t: Record<number, [string, string]> = {
    50: ["full anatomy: skin lofts per part from the DIN 33402 / Dreyfuss templates fitted to the 1:50 outlines, ears, nose, eyes, thumbs, cuffs, shoe upper and sole, garment shells over the skin", "volle Anatomie: Hautlofts je Körperteil aus den Vorlagen nach DIN 33402 / Dreyfuss, angepasst an die Konturen 1:50, Ohren, Nase, Augen, Daumen, Bündchen, Schuhschaft und Sohle, Kleidungsschalen über der Haut"],
    100: ["simplified anatomy: hands and feet merged into mittens and shoe masses, no facial detail, no skin under garments (the garment shell is the surface), fewer stations per limb", "vereinfachte Anatomie: Hände und Füsse zu Fäustlingen und Schuhmassen verschmolzen, keine Gesichtsdetails, keine Haut unter der Kleidung (die Kleidungsschale ist die Oberfläche), weniger Stationen je Gliedmasse"],
    200: ["a few masses per limb: three to four stations per part cut from the 1:200 outline, no garment details", "wenige Massen je Gliedmasse: drei bis vier Stationen je Teil aus der Kontur 1:200, keine Kleidungsdetails"],
    500: ["oblique-bone hulls: one loft per bone with two or three stations cut from the 1:500 outline", "schräge Knochen-Hüllen: ein Loft je Knochen mit zwei bis drei Stationen aus der Kontur 1:500"],
  };
  const [en, de] = t[c.scale]!;
  return [
    { en: `1:${c.scale} body, ${n(body)} solids, ${en}. Stature ${person.statureM.toFixed(2)} m, ${person.sex}, ${person.age} years.`, de: `Körper 1:${c.scale}, ${n(body)} Solids, ${de}. Statur ${person.statureM.toFixed(2)} m, ${person.sex === "male" ? "männlich" : "weiblich"}, ${person.age} Jahre.` },
    { en: `Thermal, ${n(thermal)} solids: garment lofts carry the layer shells nested on the skin (real layer thicknesses, 1:50 only), bare skin and hair reference their body solids; props and breath emitters are kept.`, de: `Thermik, ${n(thermal)} Solids: Kleidungslofts tragen die Lagenschalen auf der Haut verschachtelt (echte Lagendicken, nur 1:50), nackte Haut und Haar verweisen auf ihre Körper-Solids; Requisiten und Atemauslässe bleiben.` },
    { en: `Structure, ${n(structure)} solids: long bones as tubes of plausible shaft diameter (femur, tibia, humerus, radius and ulna), spine, pelvic ring, skull, joint balls at the rig joints and mass ellipsoids, all inside the body.`, de: `Tragwerk, ${n(structure)} Solids: Röhrenknochen als Röhren mit plausiblem Schaftdurchmesser (Femur, Tibia, Humerus, Speiche und Elle), Wirbelsäule, Beckenring, Schädel, Gelenkkugeln an den Rig-Gelenken und Massen-Ellipsoide, alle im Körper.` },
  ];
}

/** 🔁️ Adoption of an already authored human: keeps its props, garments (every body solid a clothing or surface region references, and unreferenced non-skin solids such as buttons), hair and decisions; replaces the body anatomy (solids of skin regions, skin under garments) by the kit's, adds the joint caps over the kept garments, and replaces all human-bone structure members. A skin region the kit could not build keeps its existing solids.
 * @example adopt(openCtx("humans/jogger", 50), cfg, build(c, cfg)) */
export function adopt(c: Ctx, cfg: Config, kit: Built): Built & { report: string[] } {
  const regions: Record<string, Json> = Object.fromEntries((c.m.thermal.regions as Json[]).map((r) => [r.id, r]));
  const infos = boneInfos(c);
  const exBody: Json[] = c.vol.body;
  const exThermal: Json[] = c.vol.thermal;
  const report: string[] = [];
  const kept = new Set(cfg.keepRegions ?? []);
  const owned = (rid: string): boolean => regions[rid]?.model === "body" && regions[rid]!.kind === "skin";
  const kitById = new Map(kit.body.map((x) => [x.id, x]));
  const exById = new Map(exBody.map((x) => [x.id, x]));
  const boneOfEntry = (t: Json, byId: Map<string, Json>): string | undefined => (t.of !== undefined ? byId.get(t.of)?.bone : t.bone);
  const kitBones = new Map<string, Set<string>>();
  for (const t of kit.thermal) {
    const b = boneOfEntry(t, kitById);
    if (b !== undefined && regions[t.region] !== undefined && !kit.capIds.has(t.of ?? "")) (kitBones.get(t.region) ?? kitBones.set(t.region, new Set()).get(t.region)!).add(b);
  }
  const KIT = "figure-kit";
  const isKit = (id: string | undefined): boolean => id !== undefined && ((exById.get(id)?.tags ?? []) as string[]).includes(KIT);
  const exHair = new Set(exThermal.filter((t) => regions[t.region]?.kind === "hair" && !isKit(t.of)).map((t) => t.region));
  const dropEntry = (t: Json): boolean => {
    const r = regions[t.region];
    if (r === undefined || r.model !== "body") return false;
    if (r.kind === "hair") return false;
    if (r.kind !== "skin" || kept.has(t.region)) return false;
    const b = boneOfEntry(t, exById);
    return b !== undefined && infos[b] !== undefined && kitBones.get(t.region)?.has(b) === true;
  };
  const dropRegion = (rid: string): boolean => (regions[rid]?.kind === "hair" && !exHair.has(rid) && kitBones.has(rid)) || owned(rid);
  const keptThermal = exThermal.filter((t) => !dropEntry(t));
  const refs = new Set(keptThermal.map((t) => t.of).filter((x): x is string => x !== undefined));
  const droppedRefs = new Set(exThermal.filter(dropEntry).map((t) => t.of).filter((x): x is string => x !== undefined));
  const kitSkinBones = new Set([...kitBones.entries()].filter(([r]) => owned(r) && !kept.has(r)).flatMap(([, v]) => [...v]));
  const keptBody = exBody.filter((s) => {
    if (infos[s.bone] === undefined) return true;
    if (((s.tags ?? []) as string[]).includes(KIT)) return false;
    if (refs.has(s.id)) return true;
    if (droppedRefs.has(s.id)) return false;
    return s.material !== "skin" || !kitSkinBones.has(s.bone);
  });
  const dropped = exBody.length - keptBody.length;
  const keptBones = new Set(exThermal.filter((t) => kept.has(t.region)).map((t) => boneOfEntry(t, exById)).filter((b): b is string => b !== undefined));
  const takeBody = kit.body.filter((s) => {
    const k = kit.cls.get(s.id);
    if (keptBones.has(s.bone) && k !== "hair" && k !== "cap") return false;
    return k === "anatomy" || k === "cap" || (k === "hair" && kit.thermal.some((t) => t.of === s.id && !exHair.has(t.region)));
  });
  const covered = new Set(keptThermal.map((t) => t.region));
  const takeIds = new Set(takeBody.map((s) => s.id));
  const keptIds = new Set(keptBody.map((s) => s.id));
  const takeThermal = kit.thermal.filter((t) => {
    if (kit.capIds.has(t.of ?? "") || (t.id !== undefined && [...kit.capIds].some((id) => t.id.startsWith(`th-${id}`)))) return true;
    const r = regions[t.region];
    if (r === undefined) return false;
    if (r.kind === "skin") return !kept.has(t.region) && (t.of === undefined || takeIds.has(t.of));
    if (r.kind === "hair") return !exHair.has(t.region) && (t.of === undefined || takeIds.has(t.of));
    return !covered.has(t.region) && (t.of === undefined || takeIds.has(t.of));
  });
  const rename = new Map<string, string>();
  const bodyOut = [...keptBody];
  for (const s of takeBody) {
    let id = s.id;
    while (keptIds.has(id)) id = `${id}-k`;
    if (id !== s.id) rename.set(s.id, id);
    keptIds.add(id);
    bodyOut.push({ ...s, id, tags: [...new Set([...(s.tags ?? []), KIT])] });
  }
  const thermalOut = [...keptThermal];
  const tids = new Set(thermalOut.map((t) => t.id).filter(Boolean));
  for (const t of takeThermal) {
    let id = t.id;
    while (id !== undefined && tids.has(id)) id = `${id}-k`;
    if (id !== undefined) tids.add(id);
    thermalOut.push({ ...t, ...(id === undefined ? {} : { id }), ...(t.of !== undefined && rename.has(t.of) ? { of: rename.get(t.of) } : {}) });
  }
  const have = new Set(bodyOut.map((x) => x.id));
  const dangling = thermalOut.filter((t) => t.of !== undefined && !have.has(t.of));
  for (const t of dangling) report.push(`thermal ${t.id} referenced the missing solid ${t.of}: dropped`);
  thermalOut.splice(0, thermalOut.length, ...thermalOut.filter((t) => !dangling.includes(t)));
  for (const rid of Object.keys(regions)) if (regions[rid]!.model === "body" && !thermalOut.some((t) => t.region === rid)) report.push(`region ${rid} has no thermal solid after adoption`);
  const stamp = /figure kit|Figurenbausatz/;
  const decisions = [
    ...(c.vol.decisions as Json[]).filter((d) => !stamp.test(d.en ?? "")),
    { en: `Figure kit adoption (1:${c.scale}): the human anatomy (skin lofts, skin under garments, ears, nose, eyes, thumbs, finger segments, joint caps over hip, knee, ankle, shoulder, elbow and wrist) and the human structure (tube members with real shaft diameters, joint balls, mass ellipsoids) are replaced by the figure kit's, cut from this scale's drawings; props, garments, hair, thermal layer shells of garments and all earlier decisions are kept.`, de: `Übernahme durch den Figurenbausatz (1:${c.scale}): Die Anatomie des Menschen (Hautlofts, Haut unter der Kleidung, Ohren, Nase, Augen, Daumen, Fingerglieder, Gelenkkappen an Hüfte, Knie, Knöchel, Schulter, Ellbogen und Handgelenk) und das Tragwerk des Menschen (Röhrenglieder mit echten Schaftdurchmessern, Gelenkkugeln, Massen-Ellipsoide) werden durch die des Figurenbausatzes ersetzt, geschnitten aus den Zeichnungen dieses Massstabs; Requisiten, Kleidung, Haar, thermische Lagenschalen der Kleidung und alle früheren Entscheide bleiben.` },
  ];
  report.push(`kept ${keptBody.length} body (${dropped} anatomy solids dropped), took ${takeBody.length} kit anatomy and cap solids; thermal ${thermalOut.length} (kept ${keptThermal.length}); structure ${kit.structure.length}`);
  return { ...kit, body: bodyOut, thermal: thermalOut, decisions, report };
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const spec = args[0]!;
  const slug = spec.split("/")[1]!;
  const cfgPath = flag(args, "--config", join(TICKET, "figure", `${slug}.json`))!;
  const cfg: Config = existsSync(cfgPath) ? JSON.parse(readFileSync(cfgPath, "utf8")) : {};
  const scales = flag(args, "--scale") === undefined ? [50, 100, 200, 500] : [Number(flag(args, "--scale"))];
  const existing = has(args, "--from-existing");
  for (const sc of scales) {
    const c = openCtx(spec, sc);
    const reach: Record<string, number> = {};
    if (existing) for (const x of c.vol.body as Json[]) if (x.kind === "loft" && x.sections?.length > 0) reach[x.bone] = Math.max(reach[x.bone] ?? -Infinity, ...x.sections.map((q: Json) => q.at[0]));
    const kit = build(c, cfg, existing ? reach : undefined);
    const r = existing ? adopt(c, cfg, kit) : kit;
    console.log(`1:${sc} body ${r.body.length} thermal ${r.thermal.length} structure ${r.structure.length}`);
    if (existing) for (const l of (r as ReturnType<typeof adopt>).report) console.log(`  ${l}`);
    if (has(args, "--write")) saveLists(c, { body: r.body, thermal: r.thermal, structure: r.structure, decisions: r.decisions });
    if (has(args, "--write") && has(args, "--refit") && sc >= 200) {
      const human = r.body.filter((x) => partOf(x.bone) !== null && x.slender !== true && x.role !== "detail" && (!existing || kit.cls.has(x.id))).map((x) => x.id).join(",");
      const run = Bun.spawnSync([process.execPath, join(TICKET, "author-24-kitfit.ts"), spec, "--scale", String(sc), "--solid", human, "--shift", "--sections", "--rounds", "3", "--bounds", "0.6,2.2", "--write"], { cwd: TICKET });
      console.log(`  refit 1:${sc}: ${run.exitCode === 0 ? "ok" : "failed"}`);
    }
  }
}
