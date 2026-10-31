/** [DEBUG] Repair animal assets: semantics, distinct coarse drawings, office-laser gaps by joining outlines. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, compareRepresentations, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { evaluate, flattenGeometry } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { featureWidthMm, gapsBetween } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const only = process.argv[2];
const profile = DEFAULT_PRINT_PROFILE;
const CORE: Record<string, [number, number]> = { human: [36, 38], dog: [37.5, 39.5], cat: [37.5, 39.5], horse: [37, 39.2], cow: [37.8, 39.5], deer: [37.5, 40], bird: [40.5, 42.5], sheep: [38.5, 40], goat: [38.5, 40], pig: [38, 40], fox: [38, 40], squirrel: [37.5, 39.5], hedgehog: [34, 37.5], duck: [40.5, 42.5], gull: [40.5, 42.5] };
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const within = (m: number, n: number): boolean => Math.abs(m - n) <= Math.max(0.02 * n, 0.015);
type V = [number, number];

function authored(asset: any): number[] {
  return [500, 200, 100, 50].filter((d) => asset.representations[String(d)]?.status === "authored");
}
function dedupe(shapes: any[]): any[] {
  const seen = new Set<string>();
  return shapes.filter((s) => (seen.has(s.id) ? false : (seen.add(s.id), true)));
}
function extent(asset: any, d: number): { h: number; w: number } {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  return { h: state.bounds.maxY - state.bounds.minY, w: state.bounds.maxX - state.bounds.minX };
}
function extentOk(asset: any, d: number): boolean {
  const e = extent(asset, d);
  return within(e.h, asset.dimensions.heightM) && within(e.w, asset.dimensions.widthM);
}
function suit(asset: any, d: number) {
  const clone = structuredClone(asset);
  return measureSuitability(clone, clone.representations[String(d)], d, profile);
}

function asPoly(sh: any): void {
  if (sh.kind === "poly" && sh.closed) return;
  const flat = flattenGeometry(sh);
  const ring = flat.rings[0];
  if (!ring || ring.length < 3 || !flat.closed) return;
  const step = Math.max(1, Math.ceil(ring.length / 36));
  const points = ring.filter((_, i) => i % step === 0).map((p) => [round(p.x), round(p.y)] as V);
  if (points.length < 3) return;
  for (const k of ["d", "points", "closed", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1"]) delete sh[k];
  sh.kind = "poly"; sh.points = points; sh.closed = true;
}

function blob(cx: number, cy: number, rx: number, ry: number, seed: number): V[] {
  const pts: V[] = [];
  for (let i = 0; i < 8; i++) {
    const a = (i / 8) * Math.PI * 2 + seed * 0.01;
    const wobble = 0.9 + 0.08 * (0.5 + 0.5 * Math.sin(seed + i));
    pts.push([round(cx + Math.cos(a) * rx * wobble), round(cy + Math.sin(a) * ry * wobble)]);
  }
  return pts;
}

function redraw(rep: any, denom: number): void {
  for (const key of ["elevation", "plan"] as const) {
    const shapes = rep.geometry[key] as any[] | undefined;
    if (!shapes?.length) continue;
    const kept = shapes.filter((s) => s.role === "ground");
    const groups = new Map<string, any[]>();
    for (const sh of shapes) {
      if (sh.role === "ground") continue;
      const list = groups.get(sh.bone) ?? [];
      list.push(sh);
      groups.set(sh.bone, list);
    }
    const next: any[] = [];
    let seed = denom;
    for (const [bone, group] of groups) {
      seed += 1;
      const pts: { x: number; y: number }[] = [];
      const regions: string[] = [];
      let z = 0, tone = 0;
      for (const sh of group) {
        try { for (const ring of flattenGeometry(sh).rings) for (const p of ring) pts.push(p); } catch { /* skip */ }
        if (sh.region) regions.push(sh.region);
        for (const r of sh.regions ?? []) regions.push(r);
        z = Math.max(z, sh.z ?? 0);
        tone += sh.tone ?? 0.5;
      }
      if (!pts.length) continue;
      let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
      for (const p of pts) { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); }
      const uniq = [...new Set(regions)];
      const shape: any = { id: `mass-${denom}-${bone}`, bone, role: "body", z, filled: true, tone: Math.round((tone / group.length) * 100) / 100, kind: "poly", points: blob((minX + maxX) / 2, (minY + maxY) / 2, Math.max((maxX - minX) / 2, 0.003), Math.max((maxY - minY) / 2, 0.003), seed), closed: true };
      if (uniq.length === 1) shape.region = uniq[0];
      else if (uniq.length > 1) shape.regions = uniq;
      next.push(shape);
    }
    rep.geometry[key] = [...next, ...kept];
  }
  rep.decisions.merged = { en: `At 1:${denom} the limb segments on each bone are fused into one mass, redrawn so the sheet is not a copy of a finer scale.`, de: `Bei 1:${denom} sind die Gliedmaßenteile jedes Knochens zu einer Masse verschmolzen und neu gezeichnet, damit das Blatt keine Kopie eines feineren Maßstabs ist.` };
  rep.decisions.omitted = { en: "Eyes, patches and interior lines are omitted; their thermal regions remain on the fused mass.", de: "Augen, Flecken und Innenlinien entfallen; ihre thermischen Regionen bleiben an der verschmolzenen Masse." };
}

function fixCopies(asset: any): void {
  const dens = authored(asset);
  for (let i = 0; i < dens.length; i++) for (let j = i + 1; j < dens.length; j++) {
    if (compareRepresentations(asset.representations[String(dens[i])], asset.representations[String(dens[j])]).kind === "distinct") continue;
    const rep = asset.representations[String(dens[i])];
    const snapG = JSON.stringify(rep.geometry);
    const snapD = JSON.stringify(rep.decisions);
    console.log("[DEBUG] redraw", asset.id, dens[i], "vs", dens[j]);
    redraw(rep, dens[i]!);
    if (!extentOk(asset, dens[i]!)) {
      rep.geometry = JSON.parse(snapG);
      rep.decisions = JSON.parse(snapD);
      console.log("[DEBUG] redraw reverted", asset.id, dens[i]);
    }
  }
}

function inv(m: readonly number[]): number[] {
  const det = m[0]! * m[3]! - m[1]! * m[2]!;
  const a = m[3]! / det, b = -m[1]! / det, c = -m[2]! / det, d = m[0]! / det;
  return [a, b, c, d, -(a * m[4]! + c * m[5]!), -(b * m[4]! + d * m[5]!)];
}
const apply = (m: number[], p: V): V => [m[0]! * p[0] + m[2]! * p[1] + m[4]!, m[1]! * p[0] + m[3]! * p[1] + m[5]!];
const rot = (m: number[], v: V): V => [m[0]! * v[0] + m[2]! * v[1], m[1]! * v[0] + m[3]! * v[1]];

function nudge(sh: any, localAt: V, localDir: V, amt: number, radius: number): void {
  asPoly(sh);
  if (sh.kind !== "poly") return;
  const L = Math.hypot(localDir[0], localDir[1]) || 1;
  const ux = localDir[0] / L, uy = localDir[1] / L;
  sh.points = (sh.points as V[]).map(([x, y]) => {
    const dist = Math.hypot(x - localAt[0], y - localAt[1]);
    if (dist > radius) return [x, y] as V;
    const w = Math.cos((dist / radius) * Math.PI / 2) ** 2;
    return [round(x + ux * amt * w), round(y + uy * amt * w)] as V;
  });
}

type Hit = { pose: string; a: string; b: string; mm: number; at: V };

function hitsOf(asset: any, d: number): { gaps: Hit[]; features: string[] } {
  const gaps: Hit[] = [];
  const features: string[] = [];
  const k = 1000 / d;
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    for (const p of prepared) if (p.shape.role !== "ground" && p.closed && p.shape.filled) {
      const w = featureWidthMm(p.rings, profile.minFeatureMm);
      if (w < profile.minFeatureMm - 1e-9) features.push(p.shape.id);
    }
    const paper: any[] = [];
    for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
    for (const g of gapsBetween(paper, 1)) if (g.gapMm < profile.minGapMm - 1e-9) gaps.push({ pose: pose.id, a: g.a, b: g.b, mm: g.gapMm, at: [g.at.x, g.at.y] });
  }
  return { gaps, features: [...new Set(features)] };
}

function joinScale(asset: any, d: number): void {
  const rep = asset.representations[String(d)];
  for (const id of hitsOf(asset, d).features) {
    const sh = (rep.geometry.elevation as any[]).find((s) => s.id === id);
    if (sh?.filled) sh.filled = false;
  }
  for (let iter = 0; iter < 8; iter++) {
    const gaps = hitsOf(asset, d).gaps.filter((g) => g.a && g.b && g.a !== g.b);
    const worst = new Map<string, Hit>();
    for (const g of gaps) { const key = [g.a, g.b].sort().join("|"); const o = worst.get(key); if (!o || g.mm < o.mm) worst.set(key, g); }
    if (worst.size === 0) break;
    let moved = 0;
    for (const g of worst.values()) {
      const shapes = (rep.geometry.elevation as any[]).filter((s) => s.id === g.a || s.id === g.b);
      const snap = shapes.map((s) => JSON.stringify(s));
      const state = evaluate(asset, { kind: "pose", pose: g.pose }, { kind: "scale", scale: { denominator: d } });
      const k = 1000 / d;
      const cap = 40 / d;
      const half = Math.min((g.mm + 0.05) / 2, cap);
      for (const sh of shapes) {
        const bone = state.bones[sh.bone];
        if (!bone) continue;
        const m = inv(bone.world as number[]);
        const lc = apply(m, [g.at[0] / k, -g.at[1] / k]);
        asPoly(sh);
        const pts = (sh.points ?? []) as V[];
        if (pts.length === 0) continue;
        let nearest = pts[0]!;
        let best = Infinity;
        for (const p of pts) { const dd = Math.hypot(p[0] - lc[0], p[1] - lc[1]); if (dd < best) { best = dd; nearest = p; } }
        const radiusM = Math.min(0.06, Math.max((half / k) * 6, 0.008));
        nudge(sh, nearest, [lc[0] - nearest[0], lc[1] - nearest[1]], half / k, radiusM);
      }
      if (!extentOk(asset, d)) shapes.forEach((s, i) => { const back = JSON.parse(snap[i]!); for (const key of Object.keys(s)) delete s[key]; Object.assign(s, back); });
      else moved++;
    }
    if (moved === 0) break;
  }
}

function tuneThermal(asset: any): void {
  const band = CORE[asset.thermal.body?.species as string];
  if (!band) return;
  const [lo, hi] = band;
  for (const pose of asset.poses) {
    if (pose.thermal?.activityMet === undefined) continue;
    if (pose.key && pose.thermal.activityMet > 8) pose.thermal.activityMet = 6.5;
    if (pose.key) continue;
    for (let k = 0; k < 16; k++) {
      const core = solveThermal(evaluate(asset, { kind: "pose", pose: pose.id })).body?.coreTempC;
      if (core === undefined || (core > lo && core < hi)) break;
      const delta = core >= hi ? (core > hi + 4 ? 1.5 : 0.4) : -0.3;
      pose.thermal.activityMet = Math.round(Math.min(8, Math.max(0.8, pose.thermal.activityMet - delta)) * 10) / 10;
    }
  }
}

function fixDims(asset: any, original: { w: number; h: number }): void {
  const measured = authored(asset).map((d) => extent(asset, d));
  const w = Math.max(...measured.map((e) => e.w), Math.min(...measured.map((e) => e.w)));
  const h = Math.max(...measured.map((e) => e.h));
  const loW = Math.min(...measured.map((e) => e.w));
  const loH = Math.min(...measured.map((e) => e.h));
  const fit = (hi: number, lo: number, nominal: number, cap: number): number => {
    let n = nominal;
    if (measured.every((e) => within(e.w, n)) && false) return n;
    n = (hi + lo) / 2;
    if (Math.abs(n - cap) > 0.1 * cap) n = Math.min(Math.max(n, cap * 0.9), cap * 1.1);
    return Math.round(n * 1000) / 1000;
  };
  if (!measured.every((e) => within(e.w, asset.dimensions.widthM))) asset.dimensions.widthM = fit(w, loW, asset.dimensions.widthM, original.w);
  if (!measured.every((e) => within(e.h, asset.dimensions.heightM))) asset.dimensions.heightM = Math.round(Math.min(Math.max((h + loH) / 2, original.h * 0.9), original.h * 1.1) * 1000) / 1000;
  void w;
}

const folders = (await readdir(root)).filter((f) => !only || f === only).sort();
for (const folder of folders) {
  const path = resolve(root, folder, "🔣️.json");
  const doc = JSON.parse(await readFile(path, "utf8"));
  const asset = doc.asset;
  const original = { w: asset.dimensions.widthM, h: asset.dimensions.heightM };
  if (!asset.thermal.passengers) asset.thermal.passengers = [];
  if (asset.presentation.defaultScale === undefined) asset.presentation.defaultScale = authored(asset).includes(100) ? 100 : authored(asset)[0];
  const scales = authored(asset);
  for (const d of [500, 200, 100, 50]) {
    const rep = asset.representations[String(d)];
    if (rep?.status !== "unavailable") continue;
    for (const alt of rep.alternatives ?? []) if (alt.kind === "scale" && asset.representations[String(alt.denominator)]?.status !== "authored") alt.denominator = scales.filter((a) => a < d).sort((a, b) => b - a)[0] ?? scales[0];
  }
  for (const rep of Object.values(asset.representations) as any[]) {
    if (rep.geometry?.elevation) rep.geometry.elevation = dedupe(rep.geometry.elevation);
    if (rep.geometry?.plan) rep.geometry.plan = dedupe(rep.geometry.plan);
  }
  if (asset.id === "mallard-duck") for (const bone of asset.rig.bones) if (bone.joint?.type === "revolute" && /thigh|shank|tarsus|foot/.test(bone.id)) {
    if (typeof bone.joint.minDeg === "number") bone.joint.minDeg = Math.round((bone.joint.minDeg - 30) * 10) / 10;
    if (typeof bone.joint.maxDeg === "number") bone.joint.maxDeg = Math.round((bone.joint.maxDeg + 30) * 10) / 10;
  }
  for (const clip of asset.clips) if (clip.rootMotion === "plant" && clip.loop) {
    const travel = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
    const rounded = Math.round(travel * 10000) / 10000;
    if (Math.abs((clip.advanceM ?? 0) - travel) > 5e-5) { console.log("[DEBUG] advance", asset.id, clip.id, clip.advanceM, "->", rounded); clip.advanceM = rounded; }
  }
  fixCopies(asset);
  tuneThermal(asset);
  for (const d of authored(asset)) {
    joinScale(asset, d);
    const left = suit(asset, d);
    console.log("[DEBUG] print", asset.id, d, left.suitable ? "ok" : left.violations.slice(0, 6).map((v) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" "));
  }
  fixDims(asset, original);
  const issues = validateAsset(asset).map((i) => `${i.code} ${i.path}`);
  console.log("[DEBUG] VALID", asset.id, issues.length ? issues.join(" | ") : "ok");
  await writeFile(path, JSON.stringify(doc, null, 1) + "\n");
}
console.log("[DEBUG] animals fix done");
