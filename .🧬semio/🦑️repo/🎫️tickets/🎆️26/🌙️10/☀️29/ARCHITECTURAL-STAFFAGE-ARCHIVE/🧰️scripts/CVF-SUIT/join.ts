/** [DEBUG] Join outlines that stay under the office-laser gap after pens are at the cap, by a local translation that makes the ink touch. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset, AuthoredRepresentation, Shape } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import type { Affine } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { pointInside, type Vec } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/📐️geometry/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { compareRepresentations, gapsBetween, measureSuitability, PRINT_PROFILES, type PaperShape } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { apply, invert } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = ["vegetation", "furniture", "objects", "vehicles", "machinery", "components"];
const only = new Set((process.argv[2] ?? "").split(",").filter(Boolean));
const profile = PRINT_PROFILES.profiles.find((p) => p.id === "office-laser")!;
const SCALES = ["500", "200", "100", "50"] as const;

function files(): string[] {
  const out: string[] = [];
  for (const cat of readdirSync(root)) {
    if (!cats.some((c) => cat.includes(c))) continue;
    for (const folder of readdirSync(resolve(root, cat), { withFileTypes: true })) {
      if (!folder.isDirectory()) continue;
      if (only.size && !only.has(folder.name)) continue;
      const doc = readdirSync(resolve(root, cat, folder.name)).find((f) => f.endsWith(".json"));
      if (doc) out.push(resolve(root, cat, folder.name, doc));
    }
  }
  return out;
}

function add(n: number, d: number): number {
  return Math.round((n + d) * 1e5) / 1e5;
}

function shiftPath(d: string, dx: number, dy: number): string {
  let i = 0;
  return d.replace(/-?\d*\.?\d+(?:e[+-]?\d+)?/giu, (token) => {
    const delta = i % 2 === 0 ? dx : dy;
    i++;
    return String(add(Number(token), delta));
  });
}

function shiftShape(shape: Shape, dx: number, dy: number): void {
  const s = shape as Shape & { x?: number; y?: number; cx?: number; cy?: number; x0?: number; y0?: number; x1?: number; y1?: number; points?: [number, number][]; d?: string };
  if (s.kind === "rect") {
    s.x = add(s.x!, dx);
    s.y = add(s.y!, dy);
  } else if (s.kind === "ellipse") {
    s.cx = add(s.cx!, dx);
    s.cy = add(s.cy!, dy);
  } else if (s.kind === "capsule") {
    s.x0 = add(s.x0!, dx);
    s.y0 = add(s.y0!, dy);
    s.x1 = add(s.x1!, dx);
    s.y1 = add(s.y1!, dy);
  } else if (s.kind === "poly") s.points = s.points!.map(([x, y]) => [add(x, dx), add(y, dy)]);
  else if (s.kind === "path") s.d = shiftPath(s.d!, dx, dy);
}

function segmentDistance(p: Vec, q: Vec, r: Vec, s: Vec): { d: number; a: Vec; b: Vec } {
  const d1x = q.x - p.x;
  const d1y = q.y - p.y;
  const d2x = s.x - r.x;
  const d2y = s.y - r.y;
  const on = (x: Vec, a: Vec, b: Vec): Vec => {
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const l2 = dx * dx + dy * dy;
    const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((x.x - a.x) * dx + (x.y - a.y) * dy) / l2));
    return { x: a.x + t * dx, y: a.y + t * dy };
  };
  const candidates = [
    { a: p, b: on(p, r, s) },
    { a: q, b: on(q, r, s) },
    { a: on(r, p, q), b: r },
    { a: on(s, p, q), b: s },
  ].map((c) => ({ ...c, d: Math.hypot(c.a.x - c.b.x, c.a.y - c.b.y) }));
  return candidates.reduce((best, c) => (c.d < best.d ? c : best));
}

function edges(shape: PaperShape): { a: Vec; b: Vec }[] {
  const out: { a: Vec; b: Vec }[] = [];
  for (const ring of shape.rings) {
    for (let i = 0; i + 1 < ring.length; i++) out.push({ a: ring[i]!, b: ring[i + 1]! });
    if (shape.closed && ring.length > 2) out.push({ a: ring[ring.length - 1]!, b: ring[0]! });
  }
  return out;
}

function closest(a: PaperShape, b: PaperShape): { a: Vec; b: Vec; d: number } | null {
  let best: { a: Vec; b: Vec; d: number } | null = null;
  for (const ea of edges(a)) for (const eb of edges(b)) {
    const hit = segmentDistance(ea.a, ea.b, eb.a, eb.b);
    if (best === null || hit.d < best.d) best = hit;
  }
  return best;
}

function extentOk(asset: Asset, denominator: number): boolean {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } });
  const height = state.bounds.maxY - state.bounds.minY;
  const width = state.bounds.maxX - state.bounds.minX;
  const within = (measured: number, nominal: number): boolean => Math.abs(measured - nominal) <= Math.max(0.02 * nominal, 0.015);
  return within(height, asset.dimensions.heightM) && within(width, asset.dimensions.widthM);
}

function localOf(matrix: Affine, worldDx: number, worldDy: number): { x: number; y: number } {
  const inv = invert(matrix);
  const origin = apply(inv, 0, 0);
  const moved = apply(inv, worldDx, worldDy);
  return { x: moved.x - origin.x, y: moved.y - origin.y };
}

type Move = { dx: number; dy: number };

function tryMove(asset: Asset, rep: AuthoredRepresentation, denominator: number, id: string, worldDx: number, worldDy: number, pose: string, totals: Map<string, Move>): boolean {
  const shape = rep.geometry.elevation.find((s) => s.id === id);
  if (!shape) return false;
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator } });
  const matrix = state.bones[shape.bone]?.world;
  if (matrix === undefined) return false;
  const local = localOf(matrix, worldDx, worldDy);
  const backup = structuredClone(shape);
  shiftShape(shape, local.x, local.y);
  if (!extentOk(asset, denominator)) {
    Object.assign(shape, backup);
    return false;
  }
  const key = `${denominator}:${id}`;
  const prev = totals.get(key) ?? { dx: 0, dy: 0 };
  totals.set(key, { dx: prev.dx + local.x, dy: prev.dy + local.y });
  return true;
}

function paperShapes(asset: Asset, denominator: number, pose: string): { state: ReturnType<typeof evaluate>; unclipped: PaperShape[]; clipped: PaperShape[]; filled: { rings: Vec[][] }[] } {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator } });
  const prepared = prepare(state, "elevation", sourceOf(state), 1000 / denominator);
  const unclipped = prepared.filter((p) => p.shape.role !== "ground").map((p): PaperShape => ({ id: p.shape.id, rings: p.rings, closed: p.closed, penMm: p.weightMm }));
  const clipped: PaperShape[] = [];
  for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") clipped.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
  const filled = prepared.filter((p) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p) => ({ rings: p.rings.map((ring) => [...ring]) }));
  return { state, unclipped, clipped, filled };
}

function violatingPose(drawn: { pose: string; unclipped: PaperShape[]; clipped: PaperShape[]; filled: { rings: Vec[][] }[] }[], a: string, b: string): { pose: string; gapMm: number; mover: string; other: string; from: Vec; to: Vec } | null {
  let best: { pose: string; gapMm: number; mover: string; other: string; from: Vec; to: Vec } | null = null;
  const key = [a, b].sort().join("|");
  for (const pose of drawn) {
    if (!gapsBetween(pose.unclipped, 1).some((g) => [g.a, g.b].sort().join("|") === key)) continue;
    for (const gap of gapsBetween(pose.clipped, 1)) {
      if ([gap.a, gap.b].sort().join("|") !== key || gap.gapMm >= profile.minGapMm) continue;
      if (pose.filled.some((f) => pointInside(gap.at, f.rings))) continue;
      const left = pose.unclipped.find((s) => s.id === gap.a);
      const right = pose.unclipped.find((s) => s.id === gap.b);
      if (!left || !right) continue;
      const hit = closest(left, right);
      if (!hit || best !== null && gap.gapMm >= best.gapMm) continue;
      best = { pose: pose.pose, gapMm: gap.gapMm, mover: gap.b, other: gap.a, from: hit.b, to: hit.a };
    }
  }
  return best;
}

function nudgePair(asset: Asset, rep: AuthoredRepresentation, denominator: number, drawn: { pose: string; unclipped: PaperShape[]; clipped: PaperShape[]; filled: { rings: Vec[][] }[] }[], a: string, b: string, totals: Map<string, Move>): boolean {
  const found = violatingPose(drawn, a, b);
  if (!found) {
    console.log(`[DEBUG] no-geometry ${a}+${b}`);
    return false;
  }
  const k = 1000 / denominator;
  const pdx = found.to.x - found.from.x;
  const pdy = found.to.y - found.from.y;
  const plen = Math.hypot(pdx, pdy);
  if (plen < 1e-6) return false;
  const scale = (found.gapMm + 0.02) / plen;
  const worldDx = (pdx * scale) / k;
  const worldDy = -(pdy * scale) / k;
  if (tryMove(asset, rep, denominator, found.mover, worldDx, worldDy, found.pose, totals)) return true;
  console.log(`[DEBUG] extent ${found.mover} 1:${denominator} world ${worldDx.toFixed(4)},${worldDy.toFixed(4)}`);
  if (tryMove(asset, rep, denominator, found.other, -worldDx, -worldDy, found.pose, totals)) return true;
  const halfB = tryMove(asset, rep, denominator, found.mover, worldDx / 2, worldDy / 2, found.pose, totals);
  const halfA = tryMove(asset, rep, denominator, found.other, -worldDx / 2, -worldDy / 2, found.pose, totals);
  return halfB || halfA;
}

function scanString(raw: string, i: number): number {
  i++;
  while (i < raw.length) {
    if (raw[i] === "\\") i += 2;
    else if (raw[i] === "\"") return i;
    else i++;
  }
  throw new Error("unterminated string");
}

function scanContainer(raw: string, i: number, open: string, close: string): number {
  let depth = 0;
  while (i < raw.length) {
    const c = raw[i]!;
    if (c === "\"") {
      i = scanString(raw, i) + 1;
      continue;
    }
    if (c === open) depth++;
    else if (c === close) {
      depth--;
      if (depth === 0) return i + 1;
    }
    i++;
  }
  throw new Error("unbalanced");
}

function representationSpans(raw: string): Record<string, [number, number]> {
  const keyAt = raw.indexOf("\"representations\"");
  const brace = raw.indexOf("{", keyAt);
  const spans: Record<string, [number, number]> = {};
  let i = brace + 1;
  while (i < raw.length) {
    while (" \n\r\t,".includes(raw[i]!)) i++;
    if (raw[i] === "}") break;
    const endKey = scanString(raw, i);
    const key = raw.slice(i + 1, endKey);
    i = endKey + 1;
    while (raw[i] !== ":") i++;
    i++;
    while (" \n".includes(raw[i]!)) i++;
    const start = i;
    i = raw[i] === "{" ? scanContainer(raw, i, "{", "}") : raw[i] === "[" ? scanContainer(raw, i, "[", "]") : scanString(raw, i) + 1;
    if ((SCALES as readonly string[]).includes(key)) spans[key] = [start, i];
  }
  return spans;
}

function shapeSpan(block: string, id: string): [number, number] | null {
  const token = `"id": "${id}"`;
  let from = 0;
  while (from < block.length) {
    const found = block.indexOf(token, from);
    if (found < 0) return null;
    if (block.slice(found, found + 180).includes("\"bone\"")) {
      let open = found;
      while (open > 0 && block[open] !== "{") open--;
      return [open, scanContainer(block, open, "{", "}")];
    }
    from = found + token.length;
  }
  return null;
}

function patchMove(raw: string, scale: string, id: string, move: Move): string {
  const spans = representationSpans(raw);
  const span = spans[scale];
  if (!span) throw new Error(`missing scale ${scale}`);
  const block = raw.slice(span[0], span[1]);
  const shape = shapeSpan(block, id);
  if (!shape) throw new Error(`missing shape ${scale} ${id}`);
  const body = block.slice(shape[0], shape[1]);
  const kind = body.match(/"kind": "(\w+)"/)?.[1];
  let next = body;
  if (kind === "path") {
    next = body.replace(/"d": "([^"]*)"/u, (_all, d: string) => `"d": "${shiftPath(d, move.dx, move.dy)}"`);
  } else   if (kind === "poly") {
    const at = body.indexOf("\"points\"");
    const bracket = body.indexOf("[", at);
    const end = scanContainer(body, bracket, "[", "]");
    let i = 0;
    const shifted = body.slice(bracket, end).replace(/-?\d*\.?\d+(?:e[+-]?\d+)?/giu, (token) => {
      const delta = i % 2 === 0 ? move.dx : move.dy;
      i++;
      return String(add(Number(token), delta));
    });
    next = body.slice(0, bracket) + shifted + body.slice(end);
  } else if (kind === "rect") {
    next = body.replace(/"x": (-?\d*\.?\d+)/u, (_a, n: string) => `"x": ${add(Number(n), move.dx)}`).replace(/"y": (-?\d*\.?\d+)/u, (_a, n: string) => `"y": ${add(Number(n), move.dy)}`);
  } else if (kind === "ellipse") {
    next = body.replace(/"cx": (-?\d*\.?\d+)/u, (_a, n: string) => `"cx": ${add(Number(n), move.dx)}`).replace(/"cy": (-?\d*\.?\d+)/u, (_a, n: string) => `"cy": ${add(Number(n), move.dy)}`);
  } else if (kind === "capsule") {
    next = body
      .replace(/"x0": (-?\d*\.?\d+)/u, (_a, n: string) => `"x0": ${add(Number(n), move.dx)}`)
      .replace(/"y0": (-?\d*\.?\d+)/u, (_a, n: string) => `"y0": ${add(Number(n), move.dy)}`)
      .replace(/"x1": (-?\d*\.?\d+)/u, (_a, n: string) => `"x1": ${add(Number(n), move.dx)}`)
      .replace(/"y1": (-?\d*\.?\d+)/u, (_a, n: string) => `"y1": ${add(Number(n), move.dy)}`);
  }
  return raw.slice(0, span[0]) + block.slice(0, shape[0]) + next + block.slice(shape[1]) + raw.slice(span[1]);
}

let changed = 0;
for (const path of files()) {
  const original = readFileSync(path, "utf8");
  const doc = JSON.parse(original) as { asset: Asset };
  console.log(`[DEBUG] ${doc.asset.id}`);
  const totals = new Map<string, Move>();
  let moved = 0;
  for (let iter = 0; iter < 5; iter++) {
    const clone = structuredClone(doc.asset);
    let dirty = false;
    for (const d of SCALES) {
      const rep = doc.asset.representations[d];
      const cloned = clone.representations[d];
      if (rep === undefined || cloned === undefined || rep.status !== "authored" || cloned.status !== "authored") continue;
      const suit = measureSuitability(clone, cloned, Number(d), profile);
      if (suit.violations.length) console.log(`[DEBUG] 1:${d} ${suit.violations.slice(0, 8).map((v) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`).join(" ; ")}`);
      const gaps = suit.violations.filter((v) => v.check === "gap" && v.shapes.length === 2);
      if (!gaps.length) continue;
      const drawn = doc.asset.poses.filter((p) => !p.key).map((pose) => ({ pose: pose.id, ...paperShapes(doc.asset, Number(d), pose.id) }));
      const used = new Set<string>();
      for (const gap of gaps) {
        if (used.has(gap.shapes[0]!) || used.has(gap.shapes[1]!)) continue;
        if (nudgePair(doc.asset, rep, Number(d), drawn, gap.shapes[0]!, gap.shapes[1]!, totals)) {
          used.add(gap.shapes[0]!);
          used.add(gap.shapes[1]!);
          dirty = true;
          moved++;
        }
      }
    }
    if (!dirty) break;
  }
  if (!moved) continue;
  let raw = original;
  for (const [key, move] of totals) {
    const [scale, id] = [key.slice(0, key.indexOf(":")), key.slice(key.indexOf(":") + 1)];
    raw = patchMove(raw, scale, id, move);
  }
  const written = JSON.parse(raw) as { asset: Asset };
  const issues = validateAsset(written.asset).filter((issue) => issue.code.startsWith("extent") || issue.code === "representation-copied" || issue.code === "pen-units");
  const reps = SCALES.map((d) => written.asset.representations[d]).filter((rep): rep is AuthoredRepresentation => rep !== undefined && rep.status === "authored");
  let copied = false;
  for (let i = 0; i < reps.length; i++) for (let j = i + 1; j < reps.length; j++) if (compareRepresentations(reps[i]!, reps[j]!).kind !== "distinct") copied = true;
  if (issues.length || copied) {
    console.log(`[DEBUG] SKIP ${written.asset.id}: ${issues.map((i) => i.code).join(",")}${copied ? " copied" : ""}`);
    continue;
  }
  let left = 0;
  for (const d of SCALES) {
    const rep = written.asset.representations[d];
    if (rep === undefined || rep.status !== "authored") continue;
    left += measureSuitability(written.asset, rep, Number(d), profile).violations.length;
  }
  writeFileSync(path, raw.endsWith("\n") ? raw : `${raw}\n`);
  changed++;
  console.log(`[DEBUG] ${written.asset.id}: moved ${moved}, ${left} violations left`);
}
console.log(`[DEBUG] join changed ${changed}`);
