/** 🧊️ Rig audit: one 3D chain, three projections. Usage: bun audit.ts [--only slug,slug] [--out file] [--scales all|finest]. */
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { Asset, AuthoredRepresentation, Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate, sampleOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { transition } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";

const ROOT = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const only = flag("--only")?.split(",");
const outFile = flag("--out");
const finestOnly = flag("--scales") === "finest";

type Query = Parameters<typeof evaluate>[1];
type Ext = { minX: number; maxX: number; minY: number; maxY: number };
type Found = { category: string; slug: string; check: string; count: number; worst: number; example: string };

const found = new Map<string, Found>();
const stats = { assets: 0, evaluations: 0, clipSamples: 0, poseSamples: 0, transitionSamples: 0, scrubChecks: 0, selectorChecks: 0 };

function add(category: string, slug: string, check: string, delta: number, example: string): void {
  const key = `${category}/${slug}/${check}`;
  const entry = found.get(key);
  if (entry === undefined) found.set(key, { category, slug, check, count: 1, worst: delta, example });
  else {
    entry.count++;
    if (delta > entry.worst) {
      entry.worst = delta;
      entry.example = example;
    }
  }
}

const extent = (state: ReturnType<typeof evaluate>, view: Viewpoint): Ext | undefined => {
  const pts = state.shapes[view].filter((s) => s.shape.role !== "ground").flatMap((s) => s.rings.flat());
  if (pts.length === 0) return undefined;
  let [minX, maxX, minY, maxY] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const p of pts) {
    minX = Math.min(minX, p.x);
    maxX = Math.max(maxX, p.x);
    minY = Math.min(minY, p.y);
    maxY = Math.max(maxY, p.y);
  }
  return { minX, maxX, minY, maxY };
};

const boneBox = (state: ReturnType<typeof evaluate>, view: Viewpoint, bone: string): Ext | undefined => {
  const pts = state.shapes[view].filter((s) => s.shape.bone === bone && s.shape.role !== "ground").flatMap((s) => s.rings.flat());
  if (pts.length === 0) return undefined;
  return { minX: Math.min(...pts.map((p) => p.x)), maxX: Math.max(...pts.map((p) => p.x)), minY: Math.min(...pts.map((p) => p.y)), maxY: Math.max(...pts.map((p) => p.y)) };
};

const tolOf = (a: Asset): number => Math.max(0.02, 0.03 * Math.max(a.dimensions.heightM, a.dimensions.widthM, a.dimensions.depthM));
const gap = (v: number, lo: number, hi: number): number => Math.max(lo - v, v - hi, 0);

function queriesOf(asset: Asset): { name: string; kind: "pose" | "clip" | "transition"; query: Query }[] {
  const out: { name: string; kind: "pose" | "clip" | "transition"; query: Query }[] = [];
  for (const pose of asset.poses) out.push({ name: `pose ${pose.id}`, kind: "pose", query: { kind: "pose", pose: pose.id } });
  for (const clip of asset.clips) {
    const times = new Set<number>();
    const round = (t: number): number => Math.round(t * 1e6) / 1e6;
    for (const key of clip.keys) times.add(round(key.t));
    const sorted = [...clip.keys.map((k) => k.t)].sort((a, b) => a - b);
    for (let i = 0; i + 1 < sorted.length; i++) times.add(round((sorted[i]! + sorted[i + 1]!) / 2));
    for (let k = 0; k <= 16; k++) times.add(round((clip.durationS * k) / 16));
    times.add(round(clip.durationS * 0.999));
    if (clip.loop) times.add(round(clip.durationS * 1.37));
    for (const t of [...times].sort((a, b) => a - b)) out.push({ name: `clip ${clip.id} @${t}`, kind: "clip", query: { kind: "clip", clip: clip.id, timeS: t } });
  }
  const ids = asset.poses.map((p) => p.id);
  const pairs: [string, string][] = [];
  const base = asset.presentation.defaultPose;
  for (const id of ids) if (id !== base) pairs.push([base, id], [id, base]);
  if (ids.length <= 6) for (const a of ids) for (const b of ids) if (a !== b && a !== base && b !== base) pairs.push([a, b]);
  for (const [from, to] of pairs) for (const t of [0, 0.25, 0.5, 0.75, 1]) out.push({ name: `transition ${from}>${to} @${t}`, kind: "transition", query: { kind: "transition", from, to, durationS: 1, timeS: t } });
  return out;
}

function audit(category: string, asset: Asset): void {
  const slug = asset.id;
  const tol = tolOf(asset);
  const scales = Object.values(asset.representations).filter((r): r is AuthoredRepresentation => r.status === "authored").map((r) => r.denominator).sort((a, b) => a - b);
  const useScales = finestOnly ? scales.slice(0, 1) : scales;
  const parentOf = new Map(asset.rig.bones.map((b) => [b.id, b.parent] as const));
  const ancestors = (id: string): Set<string> => {
    const out = new Set<string>([id]);
    for (let p = parentOf.get(id); p; p = parentOf.get(p)) out.add(p);
    return out;
  };
  for (const representation of Object.values(asset.representations).filter((r): r is AuthoredRepresentation => r.status === "authored"))
    for (const view of ["elevation", "front", "plan"] as const)
      for (const shape of representation.geometry[view] ?? [])
        if (shape.solid?.kind === "revolved" && !ancestors(shape.bone).has(shape.solid.axisBone)) add(category, slug, "solid-axis-bone-not-own-or-ancestor", 1, `1:${representation.denominator} ${view} shape ${shape.id} on ${shape.bone} revolves about ${shape.solid.axisBone}`);
  const queries = queriesOf(asset);
  const defaultQuery = (asset.presentation.defaultPose);
  const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as const;
  const dimsDefault = (d: number): void => {
    const s = evaluate(asset, { kind: "pose", pose: defaultQuery }, sel(d));
    const [e, f, p] = (["elevation", "front", "plan"] as const).map((v) => extent(s, v));
    const m = (check: string, v: number, nominal: number): void => {
      if (!(Math.abs(v - nominal) <= tol)) add(category, slug, check, Math.abs(v - nominal), `1:${d} measured ${v.toFixed(3)} vs ${nominal.toFixed(3)}`);
    };
    if (e) {
      m("dim-elev-width", e.maxX - e.minX, asset.dimensions.widthM);
      m("dim-elev-height", e.maxY - e.minY, asset.dimensions.heightM);
    }
    if (f) {
      m("dim-front-depth", f.maxX - f.minX, asset.dimensions.depthM);
      m("dim-front-height", f.maxY - f.minY, asset.dimensions.heightM);
    }
    if (p) {
      m("dim-plan-width", p.maxX - p.minX, asset.dimensions.widthM);
      m("dim-plan-depth", p.maxY - p.minY, asset.dimensions.depthM);
    }
  };
  for (const d of useScales) dimsDefault(d);
  const finest = scales[0]!;
  for (const { name, kind, query } of queries) {
    stats[kind === "pose" ? "poseSamples" : kind === "clip" ? "clipSamples" : "transitionSamples"]++;
    const sample = sampleOf(asset, query);
    const chain = solveChain(asset.rig, sample.offsets, sample.root);
    for (const d of useScales) {
      let state;
      try {
        state = evaluate(asset, query, sel(d));
      } catch (error) {
        add(category, slug, "evaluate-fails", 1, `1:${d} ${name}: ${(error as Error).message}`);
        continue;
      }
      stats.evaluations++;
      const where = `1:${d} ${name}`;
      const [e, f, p] = (["elevation", "front", "plan"] as const).map((v) => extent(state, v));
      for (const [v, ex] of [["elevation", e], ["front", f], ["plan", p]] as const) if (ex === undefined) add(category, slug, `${v}-empty`, 1, where);
      if (e && f && p) {
        const r = (a: number, b: number): number => Math.abs(a - b);
        const dx = Math.max(r(e.minX, p.minX), r(e.maxX, p.maxX));
        const dy = Math.max(r(e.minY, f.minY), r(e.maxY, f.maxY));
        const dz = Math.max(r(f.minX, p.minY), r(f.maxX, p.maxY));
        if (dx > tol) add(category, slug, "axis-elev-x-vs-plan-x", dx, where);
        if (dy > tol) add(category, slug, "axis-elev-y-vs-front-y", dy, where);
        if (dz > tol) add(category, slug, "axis-front-lateral-vs-plan-lateral", dz, where);
        if (kind !== "pose" || name !== `pose ${defaultQuery}`) {
          const over = Math.max((f.maxX - f.minX) - asset.dimensions.depthM - tol, (p.maxY - p.minY) - asset.dimensions.depthM - tol, (e.maxY - e.minY) - asset.dimensions.heightM - tol, 0);
          if (over > 0.25 * asset.dimensions.heightM + 0.1) add(category, slug, "extent-exceeds-dimensions", over, where);
        }
      }
      if (d !== finest && !finestOnly) continue;
      for (const bone of asset.rig.bones) {
        const c = chain[bone.id]!;
        const eb = state.bones[bone.id]!;
        const tipX = c.frame.o[0] + c.frame.u[0] * bone.lengthM;
        const tipY = c.frame.o[1] + c.frame.u[1] * bone.lengthM;
        const err = Math.max(Math.abs(eb.origin.x - c.frame.o[0]), Math.abs(eb.origin.y - c.frame.o[1]), Math.abs(eb.tip.x - tipX), Math.abs(eb.tip.y - tipY));
        if (err > 1e-9) add(category, slug, "bone-elevation-vs-chain", err, `${where} ${bone.id}`);
        const mid = [c.frame.o[0] + (c.frame.u[0] * bone.lengthM) / 2, c.frame.o[1] + (c.frame.u[1] * bone.lengthM) / 2, c.frame.o[2] + (c.frame.u[2] * bone.lengthM) / 2] as const;
        const be = boneBox(state, "elevation", bone.id);
        const bf = boneBox(state, "front", bone.id);
        const bp = boneBox(state, "plan", bone.id);
        const ptol = Math.max(tol, 0.5 * bone.lengthM);
        if (be && bf && bp) {
          const gE = Math.hypot(gap(mid[0], be.minX, be.maxX), gap(mid[1], be.minY, be.maxY));
          const gF = Math.hypot(gap(mid[2], bf.minX, bf.maxX), gap(mid[1], bf.minY, bf.maxY));
          const gP = Math.hypot(gap(mid[0], bp.minX, bp.maxX), gap(mid[2], bp.minY, bp.maxY));
          if (gE <= ptol && (gF > ptol || gP > ptol)) add(category, slug, "bone-midpoint-outside-front-or-plan-shapes", Math.max(gF, gP), `${where} ${bone.id} mid=(${mid.map((v) => v.toFixed(2))}) front gap ${gF.toFixed(2)} plan gap ${gP.toFixed(2)}`);
        }
      }
      for (const { anchor, at } of Object.values(state.anchors)) {
        const c = chain[anchor.bone]!;
        const z = c.frame.o[2] + anchor.x * c.frame.u[2] + anchor.y * c.frame.v[2] + (anchor.z ?? 0) * c.frame.w[2];
        const ex = c.frame.o[0] + anchor.x * c.frame.u[0] + anchor.y * c.frame.v[0] + (anchor.z ?? 0) * c.frame.w[0];
        const ey = c.frame.o[1] + anchor.x * c.frame.u[1] + anchor.y * c.frame.v[1] + (anchor.z ?? 0) * c.frame.w[1];
        if (Math.hypot(ex - at.x, ey - at.y) > 1e-9) add(category, slug, "anchor-elevation-vs-chain", Math.hypot(ex - at.x, ey - at.y), `${where} ${anchor.id}`);
        if (anchor.kind === "load") continue;
        const bf = boneBox(state, "front", anchor.bone);
        const bp = boneBox(state, "plan", anchor.bone);
        const aTol = Math.max(tol, 0.05);
        if (bf && bp && (anchor.kind === "attachment" || anchor.kind === "emitter" || anchor.kind === "contact" || anchor.kind === "support")) {
          const gF = Math.hypot(gap(z, bf.minX, bf.maxX), gap(at.y, bf.minY, bf.maxY));
          const gP = Math.hypot(gap(at.x, bp.minX, bp.maxX), gap(z, bp.minY, bp.maxY));
          const worst = Math.max(gF, gP);
          if (worst > aTol) add(category, slug, `anchor-${anchor.kind}-outside-bone-front-or-plan`, worst, `${where} ${anchor.id} @(${at.x.toFixed(2)},${at.y.toFixed(2)}) z=${z.toFixed(2)} front gap ${gF.toFixed(2)} plan gap ${gP.toFixed(2)}`);
        }
      }
    }
  }
  const poseBase = { kind: "pose", pose: defaultQuery } as const;
  const noSel = evaluate(asset, poseBase);
  for (const d of useScales) {
    const s = evaluate(asset, poseBase, sel(d));
    stats.selectorChecks++;
    if (JSON.stringify([s.bones, s.anchors, s.contacts]) !== JSON.stringify([noSel.bones, noSel.anchors, noSel.contacts])) add(category, slug, "view-switch-changes-state", 1, `1:${d}`);
  }
  for (const symbol of asset.symbols) {
    const s = evaluate(asset, poseBase, { kind: "symbol", symbol: symbol.id });
    stats.selectorChecks++;
    if (JSON.stringify([s.bones, s.anchors, s.contacts]) !== JSON.stringify([noSel.bones, noSel.anchors, noSel.contacts])) add(category, slug, "view-switch-changes-state", 1, `symbol ${symbol.id}`);
  }
  const clone = JSON.parse(JSON.stringify(asset)) as Asset;
  for (const clip of asset.clips) {
    const times = Array.from({ length: 24 }, (_, i) => (clip.durationS * i * 1.13) / 12);
    const scrub = [...times].reverse().map((t) => JSON.stringify(evaluate(clone, { kind: "clip", clip: clip.id, timeS: t }, sel(finest)).bones)).reverse();
    const play = times.map((t) => JSON.stringify(evaluate(asset, { kind: "clip", clip: clip.id, timeS: t }, sel(finest)).bones));
    stats.scrubChecks++;
    const bad = times.findIndex((_, i) => scrub[i] !== play[i]);
    if (bad >= 0) add(category, slug, "scrub-differs-from-playback", 1, `clip ${clip.id} @${times[bad]}`);
    if (clip.loop) {
      const h = 1e-4;
      const a = evaluate(asset, { kind: "clip", clip: clip.id, timeS: clip.durationS - h }).bones[asset.rig.root]!.origin;
      const b = evaluate(asset, { kind: "clip", clip: clip.id, timeS: clip.durationS + h }).bones[asset.rig.root]!.origin;
      const jump = Math.hypot(a.x - b.x, a.y - b.y);
      if (jump > 0.02) add(category, slug, "root-jumps-at-loop-seam", jump, `clip ${clip.id} jump ${jump.toFixed(3)} m`);
    }
  }
  const cs = sampleOf(asset, { kind: "pose", pose: defaultQuery });
  if (!Number.isFinite(cs.root.x)) add(category, slug, "root-not-finite", 1, "");
  void transition;
}

const catalogRoot = join(ROOT, "🗂️catalog");
for (const group of readdirSync(catalogRoot, { withFileTypes: true }).filter((e) => e.isDirectory())) {
  for (const folder of readdirSync(join(catalogRoot, group.name), { withFileTypes: true }).filter((e) => e.isDirectory())) {
    const file = join(catalogRoot, group.name, folder.name, "🔣️.json");
    if (!existsSync(file)) continue;
    if (only !== undefined && !only.includes(folder.name)) continue;
    const asset = (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
    stats.assets++;
    const start = Date.now();
    try {
      audit(group.name, asset);
    } catch (error) {
      add(group.name, asset.id, "audit-crash", 1, (error as Error).message);
    }
    console.error(`[audit] ${group.name}/${asset.id} ${((Date.now() - start) / 1000).toFixed(1)}s`);
  }
}

const result = { stats, findings: [...found.values()].sort((a, b) => `${a.category}/${a.slug}/${a.check}`.localeCompare(`${b.category}/${b.slug}/${b.check}`)) };
if (outFile !== undefined) writeFileSync(outFile, JSON.stringify(result, null, 1));
else console.log(JSON.stringify(result, null, 1));
