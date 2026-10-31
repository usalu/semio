/** [DEBUG] Close remaining office-laser gaps, features, strokes and hatches on the six staffage categories by the smallest pen increase that joins the ink. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Asset, AuthoredRepresentation } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { compareRepresentations, measureSuitability, PRINT_PROFILES } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = ["vegetation", "furniture", "objects", "vehicles", "machinery", "components"];
const only = new Set((process.argv[2] ?? "").split(",").filter(Boolean));
const profile = PRINT_PROFILES.profiles.find((p) => p.id === "office-laser")!;
const CAP = 1.99;
const SCALES = ["500", "200", "100", "50"] as const;

type Doc = { asset: Asset };

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

function round3(value: number): number {
  return Math.round(value * 1000) / 1000;
}

function penOf(rep: AuthoredRepresentation, id: string): number {
  const shape = rep.geometry.elevation.find((s) => s.id === id);
  if (!shape || shape.role === "ground") return CAP;
  return shape.penMm ?? rep.pens[shape.role] ?? CAP;
}

function room(rep: AuthoredRepresentation, id: string): number {
  return Math.max(0, CAP - penOf(rep, id));
}

/** Applies as much of `add` as the cap allows and returns the amount actually added. */
function give(rep: AuthoredRepresentation, id: string, add: number): number {
  if (add <= 1e-9) return 0;
  const shape = rep.geometry.elevation.find((s) => s.id === id);
  if (!shape || shape.role === "ground") return 0;
  const cur = shape.penMm ?? rep.pens[shape.role] ?? 0.13;
  const next = round3(Math.min(CAP, cur + add));
  const applied = round3(next - cur);
  if (applied <= 0) return 0;
  (shape as { penMm?: number }).penMm = next;
  return applied;
}

function givePair(rep: AuthoredRepresentation, a: string, b: string, gap: number): number {
  let need = 2 * (gap + 0.006);
  need = round3(need - give(rep, a, Math.min(need / 2, room(rep, a))));
  need = round3(need - give(rep, b, Math.min(need, room(rep, b))));
  need = round3(need - give(rep, a, Math.min(need, room(rep, a))));
  return need;
}

function distPointSeg(px: number, py: number, ax: number, ay: number, bx: number, by: number): number {
  const dx = bx - ax;
  const dy = by - ay;
  const l2 = dx * dx + dy * dy;
  const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((px - ax) * dx + (py - ay) * dy) / l2));
  return Math.hypot(px - (ax + t * dx), py - (ay + t * dy));
}

function nearestShapes(asset: Asset, rep: AuthoredRepresentation, denominator: number, at: { xMm: number; yMm: number }): string[] {
  let best = Infinity;
  let rings: { id: string; ring: { x: number; y: number }[] }[] = [];
  for (const pose of asset.poses.filter((p) => !p.key)) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator } });
    const prepared = prepare(state, "elevation", sourceOf(state), 1000 / denominator);
    const here = prepared.filter((p) => p.shape.role !== "ground").map((p) => ({ id: p.shape.id, ring: p.rings.flat() }));
    let near = Infinity;
    for (const shape of here) {
      const pts = shape.ring;
      for (let i = 0; i + 1 < pts.length; i++) near = Math.min(near, distPointSeg(at.xMm, at.yMm, pts[i]!.x, pts[i]!.y, pts[i + 1]!.x, pts[i + 1]!.y));
    }
    if (near < best) {
      best = near;
      rings = here;
    }
  }
  const ranked = rings
    .map((shape) => {
      let d = Infinity;
      const pts = shape.ring;
      for (let i = 0; i + 1 < pts.length; i++) d = Math.min(d, distPointSeg(at.xMm, at.yMm, pts[i]!.x, pts[i]!.y, pts[i + 1]!.x, pts[i + 1]!.y));
      if (pts.length > 2) d = Math.min(d, distPointSeg(at.xMm, at.yMm, pts[pts.length - 1]!.x, pts[pts.length - 1]!.y, pts[0]!.x, pts[0]!.y));
      return { id: shape.id, d };
    })
    .sort((a, b) => a.d - b.d);
  const picked = ranked.filter((s, i) => i < 2 || s.d < 0.45).slice(0, 3);
  return picked.map((s) => s.id);
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
    while (raw[i] === " " || raw[i] === "\n" || raw[i] === "\r" || raw[i] === "\t" || raw[i] === ",") i++;
    if (raw[i] === "}") break;
    if (raw[i] !== "\"") throw new Error(`expected key at ${i}`);
    const endKey = scanString(raw, i);
    const key = raw.slice(i + 1, endKey);
    i = endKey + 1;
    while (raw[i] !== ":") i++;
    i++;
    while (raw[i] === " " || raw[i] === "\n") i++;
    const start = i;
    i = raw[i] === "{" ? scanContainer(raw, i, "{", "}") : raw[i] === "[" ? scanContainer(raw, i, "[", "]") : scanString(raw, i) + 1;
    if ((SCALES as readonly string[]).includes(key)) spans[key] = [start, i];
  }
  return spans;
}

function setPenText(raw: string, span: [number, number], id: string, pen: number): string {
  const block = raw.slice(span[0], span[1]);
  const token = `"id": "${id}"`;
  let from = 0;
  let at = -1;
  while (from < block.length) {
    const found = block.indexOf(token, from);
    if (found < 0) break;
    const window = block.slice(found, found + 180);
    if (window.includes("\"bone\"")) {
      at = found;
      break;
    }
    from = found + token.length;
  }
  if (at < 0) throw new Error(`shape ${id} not in representation`);
  let open = at;
  while (open > 0 && block[open] !== "{") open--;
  const close = scanContainer(block, open, "{", "}") - 1;
  const shape = block.slice(open, close + 1);
  const formatted = String(round3(pen));
  let next: string;
  const existing = shape.match(/"penMm": -?\d+(?:\.\d+)?/);
  if (existing) next = shape.replace(/"penMm": -?\d+(?:\.\d+)?/, `"penMm": ${formatted}`);
  else {
    const indent = shape.match(/\n(\s*)"id"/)?.[1] ?? "        ";
    next = `${shape.slice(0, -1)},\n${indent}"penMm": ${formatted}\n${indent.slice(0, Math.max(0, indent.length - 1))}}`;
  }
  return raw.slice(0, span[0]) + block.slice(0, open) + next + block.slice(close + 1) + raw.slice(span[1]);
}

function distinct(asset: Asset): boolean {
  const reps = SCALES.map((d) => asset.representations[d]).filter((rep): rep is AuthoredRepresentation => rep !== undefined && rep.status === "authored");
  for (let i = 0; i < reps.length; i++) for (let j = i + 1; j < reps.length; j++) if (compareRepresentations(reps[i]!, reps[j]!).kind !== "distinct") return false;
  return true;
}

let changedFiles = 0;
const notes: string[] = [];
for (const path of files()) {
  const original = readFileSync(path, "utf8");
  const doc = JSON.parse(original) as Doc;
  console.log(`[DEBUG] ${doc.asset.id}`);
  const before = new Map<string, number | undefined>();
  for (const d of SCALES) {
    const rep = doc.asset.representations[d];
    if (rep === undefined || rep.status !== "authored") continue;
    for (const shape of rep.geometry.elevation) before.set(`${d}:${shape.id}`, shape.penMm);
    for (const hatch of rep.hatches) if (hatch.spacingMm < profile.minHatchSpacingMm) hatch.spacingMm = profile.minHatchSpacingMm;
    for (const hatch of rep.hatches) if (hatch.penMm < profile.minStrokeMm) hatch.penMm = profile.minStrokeMm;
  }
  let touched = false;
  for (let iter = 0; iter < 1; iter++) {
    const clone = structuredClone(doc.asset);
    let dirty = false;
    for (const d of SCALES) {
      const rep = doc.asset.representations[d];
      const cloned = clone.representations[d];
      if (rep === undefined || cloned === undefined || rep.status !== "authored" || cloned.status !== "authored") continue;
      console.log(`[DEBUG] ${doc.asset.id} 1:${d}`);
      const suit = measureSuitability(clone, cloned, Number(d), profile);
      for (const violation of suit.violations) {
        if (violation.check === "gap" && violation.shapes[0] === "negative-space") {
          const ids = nearestShapes(clone, cloned, Number(d), violation.at ?? { xMm: 0, yMm: 0 });
          for (const id of ids) if (give(rep, id, profile.minGapMm) > 0) dirty = true;
        } else if (violation.check === "gap" && violation.shapes.length === 2) {
          const [a, b] = violation.shapes;
          const pensBefore = penOf(rep, a!) + penOf(rep, b!);
          const left = givePair(rep, a!, b!, violation.measuredMm);
          if (left > 0.02) console.log(`[DEBUG] pen cap ${doc.asset.id} 1:${d} ${a}+${b} leftover ${left}`);
          if (penOf(rep, a!) + penOf(rep, b!) > pensBefore + 1e-9) dirty = true;
        } else if (violation.check === "feature" || violation.check === "stroke") {
          const id = violation.shapes[0]!;
          const shape = rep.geometry.elevation.find((s) => s.id === id);
          if (shape?.hatch !== undefined) {
            const hatch = rep.hatches.find((h) => h.id === shape.hatch);
            if (hatch && hatch.penMm < profile.minStrokeMm) {
              hatch.penMm = profile.minStrokeMm;
              dirty = true;
            }
          }
          const add = violation.check === "stroke" ? Math.max(0, profile.minStrokeMm - violation.measuredMm) : Math.max(0.005, profile.minFeatureMm - violation.measuredMm + 0.005);
          if (give(rep, id, add) > 0) dirty = true;
        } else if (violation.check === "hatch") {
          for (const hatch of rep.hatches) if (hatch.spacingMm < profile.minHatchSpacingMm) {
            hatch.spacingMm = profile.minHatchSpacingMm;
            dirty = true;
          }
        }
      }
    }
    if (!dirty) break;
    touched = true;
  }
  const updates: { scale: string; id: string; pen: number }[] = [];
  for (const d of SCALES) {
    const rep = doc.asset.representations[d];
    if (rep === undefined || rep.status !== "authored") continue;
    for (const shape of rep.geometry.elevation) {
      const prev = before.get(`${d}:${shape.id}`);
      if (shape.penMm !== undefined && shape.penMm !== prev) updates.push({ scale: d, id: shape.id, pen: shape.penMm });
    }
  }
  const hatchDirty = JSON.stringify(doc.asset.representations) !== JSON.stringify((JSON.parse(original) as Doc).asset.representations) && updates.length === 0;
  if (!updates.length && !touched) continue;
  let raw = original;
  const spans0 = representationSpans(raw);
  for (const update of updates) {
    const spans = representationSpans(raw);
    raw = setPenText(raw, spans[update.scale] ?? spans0[update.scale]!, update.id, update.pen);
  }
  if (hatchDirty) raw = JSON.stringify(doc, null, 1) + (original.endsWith("\n") ? "\n" : "");
  const written = JSON.parse(raw) as Doc;
  const geometrySame = (left: Asset, right: Asset): boolean => {
    for (const d of SCALES) {
      const a = left.representations[d];
      const b = right.representations[d];
      if (a === undefined || b === undefined || a.status !== "authored" || b.status !== "authored") continue;
      if (a.geometry.elevation.length !== b.geometry.elevation.length) return false;
      for (let i = 0; i < a.geometry.elevation.length; i++) {
        const sa = { ...a.geometry.elevation[i]! };
        const sb = { ...b.geometry.elevation[i]! };
        delete sa.penMm;
        delete sb.penMm;
        if (JSON.stringify(sa) !== JSON.stringify(sb)) return false;
      }
    }
    return true;
  };
  if (!geometrySame(doc.asset, written.asset)) {
    notes.push(`SKIP ${doc.asset.id}: geometry changed`);
    console.log(`[DEBUG] SKIP ${doc.asset.id}: geometry changed`);
    continue;
  }
  const issues = validateAsset(written.asset).filter((issue) => issue.code.startsWith("extent") || issue.code === "representation-copied" || issue.code === "pen-units");
  if (!distinct(written.asset) || issues.length) {
    notes.push(`SKIP ${written.asset.id}: ${issues.map((i) => i.code).join(",") || "copied"}`);
    continue;
  }
  let left = 0;
  for (const d of SCALES) {
    const rep = written.asset.representations[d];
    if (rep === undefined || rep.status !== "authored") continue;
    left += measureSuitability(written.asset, rep, Number(d), profile).violations.length;
  }
  writeFileSync(path, raw.endsWith("\n") ? raw : `${raw}\n`);
  changedFiles++;
  notes.push(`${written.asset.id}: ${updates.length} pens, ${left} suitability violations left`);
  console.log(`[DEBUG] ${notes[notes.length - 1]}`);
}
console.log(`[DEBUG] changed ${changedFiles}`);
for (const note of notes.filter((n) => n.startsWith("SKIP") || n.includes("violations left") && !n.endsWith("0 suitability violations left"))) console.log(note);
