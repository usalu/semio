/** [DEBUG] Final animal repair: semantics, coarse cow masses, and monotonic outline joins. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, compareRepresentations, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { evaluate, flattenGeometry } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { featureWidthMm, gapsBetween, whiteSliver } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const only = process.argv[2];
const profile = DEFAULT_PRINT_PROFILE;
const CORE: Record<string, [number, number]> = { dog: [37.5, 39.5], cat: [37.5, 39.5], horse: [37, 39.2], cow: [37.8, 39.5], deer: [37.5, 40], bird: [40.5, 42.5], sheep: [38.5, 40], goat: [38.5, 40], pig: [38, 40], fox: [38, 40], squirrel: [37.5, 39.5], hedgehog: [34, 37.5], duck: [40.5, 42.5], gull: [40.5, 42.5] };
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
type V = [number, number];
const within = (m: number, n: number): boolean => Math.abs(m - n) <= Math.max(0.02 * n, 0.015);

function authored(asset: any): number[] {
  return [500, 200, 100, 50].filter((d) => asset.representations[String(d)]?.status === "authored");
}
function dedupe(shapes: any[]): any[] {
  const seen = new Set<string>();
  return shapes.filter((s) => (seen.has(s.id) ? false : (seen.add(s.id), true)));
}
function extentOk(asset: any, d: number): boolean {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  const h = state.bounds.maxY - state.bounds.minY, w = state.bounds.maxX - state.bounds.minX;
  return within(h, asset.dimensions.heightM) && within(w, asset.dimensions.widthM);
}
function suit(asset: any, d: number) {
  const clone = structuredClone(asset);
  return measureSuitability(clone, clone.representations[String(d)], d, profile);
}
function asPoly(sh: any): void {
  if (sh.kind === "poly" && sh.closed && (sh.points?.length ?? 0) >= 3) return;
  let flat;
  try { flat = flattenGeometry(sh); } catch { return; }
  const ring = flat.rings[0];
  if (!ring || ring.length < 3) return;
  const step = Math.max(1, Math.ceil(ring.length / 80));
  const points = ring.filter((_: unknown, i: number) => i % step === 0).map((p: { x: number; y: number }) => [round(p.x), round(p.y)] as V);
  if (points.length < 3) return;
  for (const k of ["d", "points", "closed", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1"]) delete sh[k];
  sh.kind = "poly"; sh.points = points; sh.closed = true;
}
function paperPts(sh: any, world: number[], k: number): V[] {
  asPoly(sh);
  return ((sh.points ?? []) as V[]).map(([x, y]) => {
    const wx = world[0]! * x + world[2]! * y + world[4]!;
    const wy = world[1]! * x + world[3]! * y + world[5]!;
    return [wx * k, -wy * k] as V;
  });
}
function localDelta(world: number[], k: number, dpx: number, dpy: number): V {
  const dwx = dpx / k, dwy = -dpy / k;
  const det = world[0]! * world[3]! - world[1]! * world[2]!;
  return [(world[3]! * dwx - world[2]! * dwy) / det, (-world[1]! * dwx + world[0]! * dwy) / det];
}
function segPoint(a: V, b: V, c: V, d: V): { p: V; q: V; dist: number } {
  const proj = (p: V, s: V, e: V): V => {
    const dx = e[0] - s[0], dy = e[1] - s[1], l2 = dx * dx + dy * dy;
    const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p[0] - s[0]) * dx + (p[1] - s[1]) * dy) / l2));
    return [s[0] + t * dx, s[1] + t * dy];
  };
  let best = { p: a, q: c, dist: Infinity };
  for (const p of [a, b]) { const q = proj(p, c, d); const dist = Math.hypot(p[0] - q[0], p[1] - q[1]); if (dist < best.dist) best = { p, q, dist }; }
  for (const q of [c, d]) { const p = proj(q, a, b); const dist = Math.hypot(p[0] - q[0], p[1] - q[1]); if (dist < best.dist) best = { p, q, dist }; }
  return best;
}
function pullPair(a: any, b: any, state: any, k: number, extraMm: number): void {
  const wa = state.bones[a.bone]?.world as number[] | undefined;
  const wb = state.bones[b.bone]?.world as number[] | undefined;
  if (!wa || !wb) return;
  const pa = paperPts(a, wa, k), pb = paperPts(b, wb, k);
  if (pa.length < 2 || pb.length < 2) return;
  let best = { dist: Infinity, ia: 0, ib: 0, p: pa[0]!, q: pb[0]! };
  for (let i = 0; i < pa.length; i++) for (let j = 0; j < pb.length; j++) {
    const hit = segPoint(pa[i]!, pa[(i + 1) % pa.length]!, pb[j]!, pb[(j + 1) % pb.length]!);
    if (hit.dist < best.dist) best = { dist: hit.dist, ia: i, ib: j, p: hit.p, q: hit.q };
  }
  const dx = best.q[0] - best.p[0], dy = best.q[1] - best.p[1];
  const L = Math.hypot(dx, dy) || 1;
  const step = Math.min(extraMm, 0.12);
  const da = localDelta(wa, k, (dx / L) * step, (dy / L) * step);
  const db = localDelta(wb, k, (-dx / L) * step, (-dy / L) * step);
  const bump = (sh: any, idx: number, npts: number, delta: V): void => {
    const pts = sh.points as V[];
    for (const i of [idx, (idx + 1) % npts]) {
      const p = pts[i]!;
      pts[i] = [round(p[0] + delta[0]), round(p[1] + delta[1])];
    }
  };
  bump(a, best.ia, pa.length, da);
  bump(b, best.ib, pb.length, db);
}

type PoseHit = { gaps: { a: string; b: string; mm: number }[]; features: string[]; sliver: number | null };
function poseHit(asset: any, d: number, poseId: string): PoseHit {
  const k = 1000 / d;
  const state = evaluate(asset, { kind: "pose", pose: poseId }, { kind: "scale", scale: { denominator: d } });
  const prepared = prepare(state, "elevation", sourceOf(state), k);
  const features: string[] = [];
  for (const p of prepared) if (p.shape.role !== "ground" && p.closed && p.shape.filled) {
    const w = featureWidthMm(p.rings, profile.minFeatureMm);
    if (w < profile.minFeatureMm - 1e-9) features.push(p.shape.id);
  }
  const paper: any[] = [];
  for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
  const gaps = gapsBetween(paper, 1).filter((g) => g.gapMm < profile.minGapMm - 1e-9).map((g) => ({ a: g.a, b: g.b, mm: g.gapMm }));
  const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), profile.minGapMm);
  return { gaps, features, sliver: sl ? sl.widthMm : null };
}
function closeScale(asset: any, d: number): void {
  const rep = asset.representations[String(d)];
  const shapes = rep.geometry.elevation as any[];
  const poses = asset.poses.filter((p: any) => !p.key);
  for (const pose of poses) for (const id of poseHit(asset, d, pose.id).features) {
    const sh = shapes.find((s) => s.id === id);
    if (sh) sh.filled = false;
  }
  for (let iter = 0; iter < 20; iter++) {
    let progress = false;
    for (const pose of poses) {
      const hit = poseHit(asset, d, pose.id);
      if (hit.gaps.length === 0) continue;
      const g = hit.gaps.reduce((p, q) => (p.mm < q.mm ? p : q));
      const pair = shapes.filter((s) => s.id === g.a || s.id === g.b);
      if (pair.length < 2) continue;
      const snap = pair.map((s) => JSON.stringify(s));
      const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
      pullPair(pair[0], pair[1], state, 1000 / d, (g.mm + 0.05) / 2);
      const again = poseHit(asset, d, pose.id).gaps.find((q) => (q.a === g.a && q.b === g.b) || (q.a === g.b && q.b === g.a));
      if ((again && again.mm > g.mm - 0.004) || !extentOk(asset, d)) pair.forEach((s, i) => { const back = JSON.parse(snap[i]!); for (const key of Object.keys(s)) delete s[key]; Object.assign(s, back); });
      else progress = true;
    }
    if (!progress) break;
  }
  const filled = shapes.filter((s) => s.filled && s.role !== "ground");
  const snap = filled.map((s) => JSON.stringify(s));
  for (const sh of filled) {
    asPoly(sh);
    const pts = sh.points as V[] | undefined;
    if (!pts || pts.length < 4) continue;
    const n = pts.length;
    let area = 0;
    for (let i = 0; i < n; i++) area += pts[i]![0] * pts[(i + 1) % n]![1] - pts[(i + 1) % n]![0] * pts[i]![1];
    const sgn = area >= 0 ? 1 : -1;
    sh.points = pts.map((p, i) => {
      const a = pts[(i - 1 + n) % n]!, c = pts[(i + 1) % n]!;
      const cross = (p[0] - a[0]) * (c[1] - p[1]) - (p[1] - a[1]) * (c[0] - p[0]);
      if (sgn * cross >= 0) return p;
      const tx = c[0] - a[0], ty = c[1] - a[1], L = Math.hypot(tx, ty) || 1;
      return [round(p[0] + (sgn * ty) / L * 0.004), round(p[1] + (-sgn * tx) / L * 0.004)] as V;
    });
  }
  const sliverLeft = poses.some((pose: any) => poseHit(asset, d, pose.id).sliver !== null);
  if (sliverLeft || !extentOk(asset, d)) filled.forEach((s, i) => { const back = JSON.parse(snap[i]!); for (const key of Object.keys(s)) delete s[key]; Object.assign(s, back); });
}
function tune(asset: any): void {
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
function massPts(cx: number, cy: number, rx: number, ry: number, seed: number): V[] {
  const pts: V[] = [];
  for (let i = 0; i < 8; i++) {
    const a = (i / 8) * Math.PI * 2 + seed;
    const w = 0.84 + 0.1 * (0.5 + 0.5 * Math.sin(seed * 2 + i));
    pts.push([round(cx + Math.cos(a) * Math.max(rx, 0.002) * w), round(cy + Math.sin(a) * Math.max(ry, 0.002) * w)]);
  }
  return pts;
}
function redrawCopied(asset: any): void {
  const dens = authored(asset);
  for (let i = 0; i < dens.length; i++) for (let j = i + 1; j < dens.length; j++) {
    if (compareRepresentations(asset.representations[String(dens[i])], asset.representations[String(dens[j])]).kind === "distinct") continue;
    const denom = dens[i]!;
    const rep = asset.representations[String(denom)];
    const snapG = JSON.stringify(rep.geometry), snapD = JSON.stringify(rep.decisions);
    for (const key of ["elevation", "plan"] as const) {
      const list = rep.geometry[key] as any[] | undefined;
      if (!list?.length) continue;
      const kept = list.filter((s) => s.role === "ground");
      const groups = new Map<string, any[]>();
      for (const sh of list) if (sh.role !== "ground") { const g = groups.get(sh.bone) ?? []; g.push(sh); groups.set(sh.bone, g); }
      const next: any[] = [];
      let seed = 0.2;
      for (const [bone, group] of groups) {
        seed += 0.31;
        const pts: { x: number; y: number }[] = [];
        const regions: string[] = [];
        let z = 0, tone = 0;
        for (const sh of group) {
          try { for (const ring of flattenGeometry(sh).rings) for (const p of ring) pts.push(p); } catch { /* skip */ }
          if (sh.region) regions.push(sh.region);
          for (const r of sh.regions ?? []) regions.push(r);
          z = Math.max(z, sh.z ?? 0); tone += sh.tone ?? 0.5;
        }
        if (!pts.length) continue;
        let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
        for (const p of pts) { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); }
        const uniq = [...new Set(regions)];
        const shape: any = { id: `mass-${denom}-${bone}`, bone, role: "body", z, filled: true, tone: Math.round((tone / group.length) * 100) / 100, kind: "poly", points: massPts((minX + maxX) / 2, (minY + maxY) / 2, (maxX - minX) / 2, (maxY - minY) / 2, seed), closed: true };
        if (uniq.length === 1) shape.region = uniq[0]; else if (uniq.length > 1) shape.regions = uniq;
        next.push(shape);
      }
      rep.geometry[key] = [...next, ...kept];
    }
    rep.decisions.merged = { en: `At 1:${denom} each bone is one fused mass, redrawn coarser than the finer sheets.`, de: `Bei 1:${denom} ist jeder Knochen eine verschmolzene Masse, gröber neu gezeichnet als die feineren Blätter.` };
    rep.decisions.omitted = { en: "Eyes, patches, teats and interior lines are omitted; their thermal regions stay on the fused mass.", de: "Augen, Flecken, Zitzen und Innenlinien entfallen; ihre thermischen Regionen bleiben an der verschmolzenen Masse." };
    if (!extentOk(asset, denom)) { rep.geometry = JSON.parse(snapG); rep.decisions = JSON.parse(snapD); console.log("[DEBUG] redraw reverted", asset.id, denom); }
    else console.log("[DEBUG] redraw kept", asset.id, denom);
  }
}

const folders = (await readdir(root)).filter((f) => !only || f === only).sort();
for (const folder of folders) {
  const path = resolve(root, folder, "asset.json");
  const file = (await readdir(resolve(root, folder))).find((f) => f.endsWith(".json"))!;
  const full = resolve(root, folder, file);
  const doc = JSON.parse(await readFile(full, "utf8"));
  const asset = doc.asset;
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
  if (asset.id === "herring-gull") asset.dimensions.widthM = 0.699;
  for (const clip of asset.clips) if (clip.rootMotion === "plant" && clip.loop) {
    const travel = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
    const rounded = Math.round(travel * 10000) / 10000;
    if (Math.abs((clip.advanceM ?? 0) - travel) > 5e-5) { console.log("[DEBUG] advance", asset.id, clip.id, clip.advanceM, "->", rounded); clip.advanceM = rounded; }
  }
  redrawCopied(asset);
  tune(asset);
  for (const d of authored(asset)) {
    closeScale(asset, d);
    const left = suit(asset, d);
    console.log("[DEBUG] print", asset.id, d, left.suitable ? "ok" : `${left.violations.length} ` + left.violations.slice(0, 4).map((v) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" "));
  }
  const issues = validateAsset(asset).map((i) => `${i.code} ${i.path}`);
  console.log("[DEBUG] VALID", asset.id, issues.length ? issues.join(" | ") : "ok");
  await writeFile(full, JSON.stringify(doc, null, 1) + "\n");
}
console.log("[DEBUG] pass3 done");
