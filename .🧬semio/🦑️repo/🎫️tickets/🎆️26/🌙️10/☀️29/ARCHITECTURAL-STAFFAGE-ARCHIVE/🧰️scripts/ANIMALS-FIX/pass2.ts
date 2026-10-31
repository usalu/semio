/** [DEBUG] Second pass: merge the copied cow sheet and close remaining office-laser gaps. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, compareRepresentations, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { evaluate, flattenGeometry } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { featureWidthMm, gapsBetween, whiteSliver } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const only = process.argv[2];
const profile = DEFAULT_PRINT_PROFILE;
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
type V = [number, number];
const within = (m: number, n: number): boolean => Math.abs(m - n) <= Math.max(0.02 * n, 0.015);

function authored(asset: any): number[] {
  return [500, 200, 100, 50].filter((d) => asset.representations[String(d)]?.status === "authored");
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
  if (sh.kind === "poly" && sh.closed && (sh.points?.length ?? 0) >= 3) return;
  let flat;
  try { flat = flattenGeometry(sh); } catch { return; }
  const ring = flat.rings[0];
  if (!ring || ring.length < 3) return;
  const step = Math.max(1, Math.ceil(ring.length / 48));
  const points = ring.filter((_: unknown, i: number) => i % step === 0).map((p: { x: number; y: number }) => [round(p.x), round(p.y)] as V);
  if (points.length < 3) return;
  for (const k of ["d", "points", "closed", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1"]) delete sh[k];
  sh.kind = "poly";
  sh.points = points;
  sh.closed = true;
}
function inv(m: readonly number[]): number[] {
  const det = m[0]! * m[3]! - m[1]! * m[2]!;
  const a = m[3]! / det, b = -m[1]! / det, c = -m[2]! / det, d = m[0]! / det;
  return [a, b, c, d, -(a * m[4]! + c * m[5]!), -(b * m[4]! + d * m[5]!)];
}
const applyM = (m: number[], p: V): V => [m[0]! * p[0] + m[2]! * p[1] + m[4]!, m[1]! * p[0] + m[3]! * p[1] + m[5]!];

function pullEdge(sh: any, target: V, amount: number): void {
  asPoly(sh);
  const pts = sh.points as V[] | undefined;
  if (!pts || pts.length < 3) return;
  const n = pts.length;
  let bestI = 0;
  let bestD = Infinity;
  let bestP: V = pts[0]!;
  for (let i = 0; i < n; i++) {
    const a = pts[i]!, b = pts[(i + 1) % n]!;
    const dx = b[0] - a[0], dy = b[1] - a[1];
    const l2 = dx * dx + dy * dy;
    const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((target[0] - a[0]) * dx + (target[1] - a[1]) * dy) / l2));
    const px = a[0] + t * dx, py = a[1] + t * dy;
    const dist = Math.hypot(px - target[0], py - target[1]);
    if (dist < bestD) { bestD = dist; bestI = i; bestP = [px, py]; }
  }
  const ux = target[0] - bestP[0], uy = target[1] - bestP[1];
  const L = Math.hypot(ux, uy) || 1;
  const step = Math.min(amount, bestD + amount);
  const mx = (ux / L) * step, my = (uy / L) * step;
  const touch = (i: number): void => {
    const p = pts[i]!;
    const near = Math.hypot(p[0] - bestP[0], p[1] - bestP[1]);
    if (near > 0.05) return;
    const w = Math.cos(Math.min(1, near / 0.05) * Math.PI / 2) ** 2;
    pts[i] = [round(p[0] + mx * w), round(p[1] + my * w)];
  };
  touch(bestI);
  touch((bestI + 1) % n);
}

type Hit = { pose: string; a: string; b: string; mm: number; at: V; kind: "gap" | "sliver" | "feature" };

function collect(asset: any, d: number): Hit[] {
  const hits: Hit[] = [];
  const k = 1000 / d;
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    for (const p of prepared) if (p.shape.role !== "ground" && p.closed && p.shape.filled) {
      const w = featureWidthMm(p.rings, profile.minFeatureMm);
      if (w < profile.minFeatureMm - 1e-9) hits.push({ pose: pose.id, a: p.shape.id, b: "", mm: w, at: [0, 0], kind: "feature" });
    }
    const paper: any[] = [];
    for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
    const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), profile.minGapMm);
    if (sl) hits.push({ pose: pose.id, a: "negative-space", b: "", mm: sl.widthMm, at: [sl.at.x, sl.at.y], kind: "sliver" });
    for (const g of gapsBetween(paper, 1)) if (g.gapMm < profile.minGapMm - 1e-9) hits.push({ pose: pose.id, a: g.a, b: g.b, mm: g.gapMm, at: [g.at.x, g.at.y], kind: "gap" });
  }
  return hits;
}

function closeScale(asset: any, d: number): void {
  const rep = asset.representations[String(d)];
  const shapes = rep.geometry.elevation as any[];
  for (let iter = 0; iter < 6; iter++) {
    const hits = collect(asset, d);
    const feats = new Set(hits.filter((h) => h.kind === "feature").map((h) => h.a));
    for (const sh of shapes) if (feats.has(sh.id)) sh.filled = false;
    const gaps = hits.filter((h) => h.kind === "gap");
    const worst = new Map<string, Hit>();
    for (const g of gaps) { const key = [g.a, g.b].sort().join("|"); const o = worst.get(key); if (!o || g.mm < o.mm) worst.set(key, g); }
    if (worst.size === 0) break;
    let moved = 0;
    for (const g of worst.values()) {
      const pair = shapes.filter((s) => s.id === g.a || s.id === g.b);
      const snap = pair.map((s) => JSON.stringify(s));
      const state = evaluate(asset, { kind: "pose", pose: g.pose }, { kind: "scale", scale: { denominator: d } });
      const k = 1000 / d;
      const amount = Math.min((g.mm + 0.06) / 2, 50 / d) / k;
      for (const sh of pair) {
        const bone = state.bones[sh.bone];
        if (!bone) continue;
        const target = applyM(inv(bone.world as number[]), [g.at[0] / k, -g.at[1] / k]);
        pullEdge(sh, target, amount);
      }
      if (!extentOk(asset, d)) pair.forEach((s, i) => { const back = JSON.parse(snap[i]!); for (const key of Object.keys(s)) delete s[key]; Object.assign(s, back); });
      else moved++;
    }
    if (moved === 0) break;
  }
  const after = collect(asset, d);
  for (const sh of shapes) if (after.some((h) => h.kind === "feature" && h.a === sh.id)) sh.filled = false;
  const slivers = after.filter((h) => h.kind === "sliver");
  if (slivers.length) {
    const filled = shapes.filter((s) => s.filled && s.role !== "ground");
    const snap = filled.map((s) => JSON.stringify(s));
    const delta = Math.min(0.01, (0.1 * d) / 1000);
    for (const sh of filled) {
      asPoly(sh);
      const pts = sh.points as V[];
      if (!pts?.length) continue;
      let cx = 0, cy = 0;
      for (const p of pts) { cx += p[0]; cy += p[1]; }
      cx /= pts.length; cy /= pts.length;
      sh.points = pts.map(([x, y]) => {
        const dx = x - cx, dy = y - cy, L = Math.hypot(dx, dy) || 1;
        return [round(x + (dx / L) * delta), round(y + (dy / L) * delta)] as V;
      });
    }
    if (!extentOk(asset, d)) filled.forEach((s, i) => { const back = JSON.parse(snap[i]!); for (const key of Object.keys(s)) delete s[key]; Object.assign(s, back); });
  }
}

function mass(cx: number, cy: number, rx: number, ry: number, seed: number): V[] {
  const pts: V[] = [];
  const sx = Math.max(rx, 0.002), sy = Math.max(ry, 0.002);
  for (let i = 0; i < 8; i++) {
    const a = (i / 8) * Math.PI * 2 + seed;
    const w = 0.82 + 0.12 * (0.5 + 0.5 * Math.sin(seed * 3 + i));
    pts.push([round(cx + Math.cos(a) * sx * w), round(cy + Math.sin(a) * sy * w)]);
  }
  return pts;
}

function redrawCow(asset: any): void {
  const dens = authored(asset);
  for (let i = 0; i < dens.length; i++) for (let j = i + 1; j < dens.length; j++) {
    if (compareRepresentations(asset.representations[String(dens[i])], asset.representations[String(dens[j])]).kind === "distinct") continue;
    const denom = dens[i]!;
    const rep = asset.representations[String(denom)];
    const before = extent(asset, denom);
    const snapG = JSON.stringify(rep.geometry);
    const snapD = JSON.stringify(rep.decisions);
    for (const key of ["elevation", "plan"] as const) {
      const list = rep.geometry[key] as any[] | undefined;
      if (!list?.length) continue;
      const kept = list.filter((s) => s.role === "ground");
      const groups = new Map<string, any[]>();
      for (const sh of list) if (sh.role !== "ground") {
        const g = groups.get(sh.bone) ?? [];
        g.push(sh);
        groups.set(sh.bone, g);
      }
      const next: any[] = [];
      let seed = 0.4;
      for (const [bone, group] of groups) {
        seed += 0.17;
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
        const shape: any = { id: `mass-${denom}-${bone}`, bone, role: "body", z, filled: true, tone: Math.round((tone / group.length) * 100) / 100, kind: "poly", points: mass((minX + maxX) / 2, (minY + maxY) / 2, (maxX - minX) / 2, (maxY - minY) / 2, seed), closed: true };
        if (uniq.length === 1) shape.region = uniq[0];
        else if (uniq.length > 1) shape.regions = uniq;
        next.push(shape);
      }
      rep.geometry[key] = [...next, ...kept];
    }
    rep.decisions.merged = { en: `At 1:${denom} each bone is one fused mass, redrawn coarser than the finer sheets.`, de: `Bei 1:${denom} ist jeder Knochen eine verschmolzene Masse, gröber neu gezeichnet als die feineren Blätter.` };
    rep.decisions.omitted = { en: "Eyes, patches, teats and interior lines are omitted; their thermal regions stay on the fused mass.", de: "Augen, Flecken, Zitzen und Innenlinien entfallen; ihre thermischen Regionen bleiben an der verschmolzenen Masse." };
    const after = extent(asset, denom);
    console.log("[DEBUG] redraw extent", asset.id, denom, "before", before.w.toFixed(3), before.h.toFixed(3), "after", after.w.toFixed(3), after.h.toFixed(3), "limit", asset.dimensions.widthM, asset.dimensions.heightM);
    if (!extentOk(asset, denom)) {
      rep.geometry = JSON.parse(snapG);
      rep.decisions = JSON.parse(snapD);
      console.log("[DEBUG] redraw reverted", asset.id, denom);
    }
  }
}

const folders = (await readdir(root)).filter((f) => !only || f === only).sort();
for (const folder of folders) {
  const path = resolve(root, folder, "🔣️.json");
  const doc = JSON.parse(await readFile(path, "utf8"));
  const asset = doc.asset;
  if (asset.id === "dairy-cow") redrawCow(asset);
  for (const d of authored(asset)) {
    closeScale(asset, d);
    const left = suit(asset, d);
    console.log("[DEBUG] print", asset.id, d, left.suitable ? "ok" : `${left.violations.length} ` + left.violations.slice(0, 5).map((v) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" "));
  }
  const issues = validateAsset(asset).map((i) => `${i.code} ${i.path}`);
  console.log("[DEBUG] VALID", asset.id, issues.length ? issues.join(" | ") : "ok");
  await writeFile(path, JSON.stringify(doc, null, 1) + "\n");
}
console.log("[DEBUG] pass2 done");
