/** [DEBUG] Close the gaps that remain between visible outlines. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { evaluate, flattenGeometry } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { featureWidthMm, gapsBetween } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const only = process.argv[2];
const profile = DEFAULT_PRINT_PROFILE;
const round = (n: number) => Math.round(n * 1e5) / 1e5;
type V = [number, number];
const within = (m: number, n: number) => Math.abs(m - n) <= Math.max(0.02 * n, 0.015);
const authored = (asset: any) => [500, 200, 100, 50].filter((d) => asset.representations[String(d)]?.status === "authored");
function extentOk(asset: any, d: number) {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  return within(state.bounds.maxY - state.bounds.minY, asset.dimensions.heightM) && within(state.bounds.maxX - state.bounds.minX, asset.dimensions.widthM);
}
function suit(asset: any, d: number) {
  const clone = structuredClone(asset);
  return measureSuitability(clone, clone.representations[String(d)], d, profile);
}
function localDelta(world: number[], k: number, dpx: number, dpy: number): V {
  const dwx = dpx / k, dwy = -dpy / k;
  const det = world[0]! * world[3]! - world[1]! * world[2]!;
  return [(world[3]! * dwx - world[2]! * dwy) / det, (-world[1]! * dwx + world[0]! * dwy) / det];
}
function toPaper(sh: any, world: number[], k: number): V[] {
  return ((sh.points ?? []) as V[]).map(([x, y]) => [ (world[0]! * x + world[2]! * y + world[4]!) * k, -(world[1]! * x + world[3]! * y + world[5]!) * k ]);
}
function posePack(asset: any, d: number, poseId: string) {
  const k = 1000 / d;
  const state = evaluate(asset, { kind: "pose", pose: poseId }, { kind: "scale", scale: { denominator: d } });
  const prepared = prepare(state, "elevation", sourceOf(state), k);
  const runs = new Map<string, V[]>();
  for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") {
    const list = runs.get(p.shape.id) ?? [];
    for (const q of run.points) list.push([q.x, q.y]);
    runs.set(p.shape.id, list);
  }
  const gaps = gapsBetween([...runs].map(([id, pts]) => ({ id, rings: [pts.map(([x, y]) => ({ x, y }))], closed: false, penMm: prepared.find((p) => p.shape.id === id)?.weightMm ?? 0.18 })), 1)
    .filter((g) => g.gapMm < profile.minGapMm - 1e-9);
  const features = prepared.filter((p) => p.shape.role !== "ground" && p.closed && p.shape.filled && featureWidthMm(p.rings, profile.minFeatureMm) < profile.minFeatureMm - 1e-9).map((p) => p.shape.id);
  return { state, k, runs, gaps, features };
}
function nudgeVisible(sh: any, world: number[], k: number, from: V, to: V, amount: number) {
  const papers = toPaper(sh, world, k);
  if (!papers.length || !sh.points) return;
  let best = 0, dist = Infinity;
  for (let i = 0; i < papers.length; i++) {
    const dd = Math.hypot(papers[i]![0] - from[0], papers[i]![1] - from[1]);
    if (dd < dist) { dist = dd; best = i; }
  }
  const dx = to[0] - from[0], dy = to[1] - from[1], L = Math.hypot(dx, dy) || 1;
  const delta = localDelta(world, k, (dx / L) * amount, (dy / L) * amount);
  const pts = sh.points as V[];
  for (const i of [best, (best + 1) % pts.length, (best - 1 + pts.length) % pts.length]) {
    const p = pts[i]!;
    const w = i === best ? 1 : 0.45;
    pts[i] = [round(p[0] + delta[0] * w), round(p[1] + delta[1] * w)];
  }
}

function asPoly(sh: any) {
  if (sh.kind === "poly" && sh.closed && (sh.points?.length ?? 0) >= 3) return;
  let flat; try { flat = flattenGeometry(sh); } catch { return; }
  const ring = flat.rings[0]; if (!ring || ring.length < 3) return;
  const step = Math.max(1, Math.ceil(ring.length / 80));
  const points = ring.filter((_: unknown, i: number) => i % step === 0).map((p: { x: number; y: number }) => [round(p.x), round(p.y)]);
  if (points.length < 3) return;
  for (const k of ["d", "points", "closed", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1"]) delete sh[k];
  sh.kind = "poly"; sh.points = points; sh.closed = true;
}
function close(asset: any, d: number) {
  const shapes = asset.representations[String(d)].geometry.elevation as any[];
  const poses = asset.poses.filter((p: any) => !p.key);
  for (const pose of poses) for (const id of posePack(asset, d, pose.id).features) {
    const sh = shapes.find((s) => s.id === id);
    if (sh) sh.filled = false;
  }
  for (let iter = 0; iter < 12; iter++) {
    let progress = false;
    for (const pose of poses) {
      const pack = posePack(asset, d, pose.id);
      if (!pack.gaps.length) continue;
      const g = pack.gaps.reduce((p, q) => (p.gapMm < q.gapMm ? p : q));
      const a = shapes.find((s) => s.id === g.a);
      const b = shapes.find((s) => s.id === g.b);
      const ra = pack.runs.get(g.a) ?? [], rb = pack.runs.get(g.b) ?? [];
      asPoly(a); asPoly(b);
      if (!a?.points || !b?.points || ra.length < 1 || rb.length < 1) continue;
      let best = Infinity, pa = ra[0]!, pb = rb[0]!;
      const stepA = Math.max(1, Math.floor(ra.length / 40)), stepB = Math.max(1, Math.floor(rb.length / 40));
      for (let i = 0; i < ra.length; i += stepA) for (let j = 0; j < rb.length; j += stepB) {
        const dist = Math.hypot(ra[i]![0] - rb[j]![0], ra[i]![1] - rb[j]![1]);
        if (dist < best) { best = dist; pa = ra[i]!; pb = rb[j]!; }
      }
      const snap = [JSON.stringify(a), JSON.stringify(b)];
      const wa = pack.state.bones[a.bone]?.world as number[] | undefined;
      const wb = pack.state.bones[b.bone]?.world as number[] | undefined;
      if (!wa || !wb) continue;
      const amount = Math.min((g.gapMm + 0.06) / 2, 0.12);
      nudgeVisible(a, wa, pack.k, pa, pb, amount);
      nudgeVisible(b, wb, pack.k, pb, pa, amount);
      const again = posePack(asset, d, pose.id).gaps.find((q) => (q.a === g.a && q.b === g.b) || (q.a === g.b && q.b === g.a));
      if ((again && again.gapMm > g.gapMm - 0.004) || !extentOk(asset, d)) {
        for (const [sh, raw] of [[a, snap[0]], [b, snap[1]]] as const) { const back = JSON.parse(raw!); for (const key of Object.keys(sh)) delete sh[key]; Object.assign(sh, back); }
      } else progress = true;
    }
    if (!progress) break;
  }
}
const folders = (await readdir(root)).filter((f) => !only || f === only).sort();
for (const folder of folders) {
  const file = (await readdir(resolve(root, folder))).find((f) => f.endsWith(".json"))!;
  const full = resolve(root, folder, file);
  const doc = JSON.parse(await readFile(full, "utf8"));
  const asset = doc.asset;
  let dirty = false;
  for (const d of authored(asset)) {
    const before = suit(asset, d);
    if (before.suitable) { console.log("[DEBUG] print", asset.id, d, "ok"); continue; }
    close(asset, d);
    dirty = true;
    const left = suit(asset, d);
    console.log("[DEBUG] print", asset.id, d, left.suitable ? "ok" : `${left.violations.length} ` + left.violations.slice(0, 4).map((v) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" "));
  }
  const issues = validateAsset(asset).map((i) => `${i.code} ${i.path}`);
  if (issues.length) console.log("[DEBUG] VALID", asset.id, issues.join(" | "));
  if (dirty) await writeFile(full, JSON.stringify(doc, null, 1) + "\n");
}
console.log("[DEBUG] pass4 done");
