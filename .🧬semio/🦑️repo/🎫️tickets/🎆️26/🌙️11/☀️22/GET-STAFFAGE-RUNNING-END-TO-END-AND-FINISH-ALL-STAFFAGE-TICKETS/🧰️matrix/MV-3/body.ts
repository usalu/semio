/** 🫧️ Replaces the depth plates of a plant by ONE real body per part and view, as the last drawing step of its chain: `bun body.ts <id|file.json> [--save ops.json | --ops ops.json] [--out file] [--write] [--scales 50,100] [--only bone,bone] [--paper 0.05] [--margin 0.002]`.
 *
 * `carry.ts` lays flat plates `mvc-*` under the front and plan outlines of the parts the wind turns; in a wind pose every plate shows as a straight cut beside the outline. This tool takes every part (scale, view, bone) that carries plates and lays one `loft` shape `mvb-<bone>` in their place, below the drawn outlines:
 * - the stations run along the height (front) or the forward axis (plan); at every station the section is the ellipse over the lateral interval INSIDE the drawn outlines of the part (the run of intervals that hangs together, starting at the widest one, so the body never bridges a notch) and over the depth interval of the elevation outlines of the bone at that station;
 * - every interval is drawn in by `paper` millimetres on the paper of the scale plus `margin` metres, and the run is evaluated in the default pose: a body that shows beside the drawn outlines there is drawn in further (up to five times) or left out and named, so the default pose draws what it drew before;
 * - stations are kept until the straight edge between two of them is within `paper` millimetres of both outlines.
 * In a wind pose the bone turns the body itself, so what shows beside the foreshortened outline is the smooth edge of one body. The outlines themselves (and the suffix `-mvs` `carry.ts` gave them) stay as they are.
 * `--save` writes the bodies as absolute values with the place they stand at and the ids of the sheets that carry `-mvs`; `--ops` applies such a file: it removes every `mvc-*` plate and every `mvb-*` body, sets the suffixes as saved and lays the saved bodies, so the step is idempotent and runs after `carry.ts` whatever that step wrote.
 * @see ./carry.ts — the plates this tool replaces
 * @see ./visible.ts — what shows beside the outlines in the wind poses (`--prefix mvb-`)
 * @see ../MV-L1/slice.ts — the sections of a mass from its two outlines, the construction this one draws inside the outline
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate, flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { loftShapeIssues } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volume", "🟦️.ts"));

type View = "front" | "plan";
type P = { x: number; y: number };
type V3 = [number, number, number];
type Span = [number, number];
type Part = { key: string; view: View; bone: string; under: string; common: any; inset: number; shape?: any };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const only = flag("only")?.split(",");
const scales = flag("scales")?.split(",");
const PAPER = Number(flag("paper") ?? 0.05);
const MARGIN = Number(flag("margin") ?? 0.002);
const PLATE = "mvc-";
const BODY = "mvb-";
const SHEET = "-mvs";
const COMMON = ["bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const VIEWS: readonly View[] = ["front", "plan"];
const TINY = 5e-4;
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const rest = restFrames(asset.rig) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const mm = (m: number): string => (m * 1000).toFixed(1);
const baseId = (id: string): string => (id.endsWith(SHEET) ? id.slice(0, -SHEET.length) : id);
const made = (shape: any): boolean => String(shape.id).startsWith(PLATE) || String(shape.id).startsWith(BODY);
const sheetOf = (shape: any): boolean => !made(shape) && shape.kind !== "loft" && shape.kind !== "member" && shape.solid === undefined && shape.filled === true && flattenGeometry(shape).closed === true;

/** 📏️ The spans of the other coordinate inside the outlines at station `s` (even-odd per outline, the spans of all outlines merged). */
function spans(outlines: readonly (readonly (readonly P[])[])[], along: "x" | "y", s: number): Span[] {
  const across = along === "x" ? "y" : "x";
  const found: Span[] = [];
  for (const rings of outlines) {
    const cuts: number[] = [];
    for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const [a, b] = [ring[j]!, ring[i]!];
      if ((a[along] > s) === (b[along] > s)) continue;
      cuts.push(a[across] + ((b[across] - a[across]) * (s - a[along])) / (b[along] - a[along]));
    }
    cuts.sort((m, n) => m - n);
    for (let i = 0; i + 1 < cuts.length; i += 2) found.push([cuts[i]!, cuts[i + 1]!]);
  }
  found.sort((m, n) => m[0] - n[0]);
  const merged: Span[] = [];
  for (const span of found) { const last = merged[merged.length - 1]; if (last !== undefined && span[0] <= last[1] + 1e-9) last[1] = Math.max(last[1], span[1]); else merged.push([span[0], span[1]]); }
  return merged;
}

/** 🧭️ The unit quaternion of the rotation with the columns `x`, `y`, `z`. */
function quaternion(x: V3, y: V3, z: V3): number[] {
  const [m00, m10, m20] = x, [m01, m11, m21] = y, [m02, m12, m22] = z;
  const trace = m00 + m11 + m22;
  let q: number[];
  if (trace > 0) { const s = 2 * Math.sqrt(trace + 1); q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]; }
  else if (m00 > m11 && m00 > m22) { const s = 2 * Math.sqrt(1 + m00 - m11 - m22); q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]; }
  else if (m11 > m22) { const s = 2 * Math.sqrt(1 + m11 - m00 - m22); q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]; }
  else { const s = 2 * Math.sqrt(1 + m22 - m00 - m11); q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]; }
  const n = Math.hypot(...q);
  return q.map((c) => Math.round((c / n) * 1e9) / 1e9);
}

/** 🫧️ The sections of the body of one part in one view, or the reason there is none. */
function sectionsOf(view: View, bone: string, drawn: P[][][], side: P[][][], budget: number, inset: number): any[] | string {
  const f = rest[bone]!.frame;
  const world = side.map((rings) => rings.map((ring) => ring.map((p) => ({ x: f.o[0] + p.x * f.u[0] + p.y * f.v[0], y: f.o[1] + p.x * f.u[1] + p.y * f.v[1] }))));
  const along: "x" | "y" = view === "front" ? "y" : "x";
  const reach = (outlines: P[][][]): Span => { const list = outlines.flat(2).map((p) => p[along]); return [Math.min(...list), Math.max(...list)]; };
  const [d0, d1] = reach(drawn);
  const [e0, e1] = reach(world);
  if (!(d1 - d0 > 4 * inset) || !(e1 - e0 > 0)) return "too small";
  const grid = new Set<number>();
  for (const p of drawn.flat(2)) for (const s of [p[along] - 2e-4, p[along] + 2e-4]) grid.add(Math.round(s * 1e5) / 1e5);
  for (let i = 0; i <= 120; i++) grid.add(Math.round((d0 + ((d1 - d0) * i) / 120) * 1e5) / 1e5);
  const fine = [...grid].filter((s) => s > d0 + inset && s < d1 - inset).sort((a, b) => a - b);
  const all = fine.map((s) => spans(drawn, along, s + 1.3e-6));
  let start = -1, widest = 0;
  all.forEach((list, i) => { for (const span of list) if (span[1] - span[0] > widest) { widest = span[1] - span[0]; start = i; } });
  if (start < 0) return "no span";
  const chosen = new Map<number, Span>();
  chosen.set(start, all[start]!.reduce((best, span) => (span[1] - span[0] > best[1] - best[0] ? span : best)));
  for (const step of [1, -1]) {
    let last = chosen.get(start)!;
    for (let i = start + step; i >= 0 && i < fine.length; i += step) {
      let best: Span | undefined, most = 0;
      for (const span of all[i]!) { const shared = Math.min(span[1], last[1]) - Math.max(span[0], last[0]); if (shared > most) { most = shared; best = span; } }
      if (best === undefined || best[1] - best[0] <= 2 * inset + 2 * TINY) break;
      chosen.set(i, best);
      last = best;
    }
  }
  const run = [...chosen.keys()].sort((a, b) => a - b).filter((i) => chosen.get(i)![1] - chosen.get(i)![0] > 2 * inset + 2 * TINY);
  let from = 0, best: Span = [0, -1];
  for (let i = 1; i <= run.length; i++) if (i === run.length || run[i]! !== run[i - 1]! + 1) { if (i - 1 - from > best[1] - best[0]) best = [from, i - 1]; from = i; }
  const keep = run.slice(best[0], best[1] + 1);
  if (keep.length < 3) return "no run of spans";
  const deepAt = (s: number): Span | undefined => {
    const list = spans(world, along, e0 + ((s - d0) * (e1 - e0)) / (d1 - d0) + 1.3e-6);
    return list.length === 0 ? undefined : [list[0]![0], list[list.length - 1]![1]];
  };
  const rows = keep.map((i) => ({ s: fine[i]!, drawn: [chosen.get(i)![0] + inset, chosen.get(i)![1] - inset] as Span, deep: deepAt(fine[i]!) })).filter((row) => row.deep !== undefined) as { s: number; drawn: Span; deep: Span }[];
  if (rows.length < 3) return "no depth";
  const kept = new Set<number>([0, rows.length - 1]);
  const split = (i: number, j: number): void => {
    let worst = 1, where = -1;
    for (let k = i + 1; k < j; k++) {
      const t = (rows[k]!.s - rows[i]!.s) / (rows[j]!.s - rows[i]!.s);
      const off = (name: "drawn" | "deep", end: 0 | 1): number => Math.abs(rows[k]![name][end] - rows[i]![name][end] - t * (rows[j]![name][end] - rows[i]![name][end]));
      const miss = Math.max(off("drawn", 0) / budget, off("drawn", 1) / budget, off("deep", 0) / (3 * budget), off("deep", 1) / (3 * budget));
      if (miss > worst) { worst = miss; where = k; }
    }
    if (where < 0) return;
    kept.add(where);
    split(i, where);
    split(where, j);
  };
  split(0, rows.length - 1);
  const X: V3 = [1, 0, 0], Y: V3 = [0, 1, 0], Z: V3 = [0, 0, 1];
  const local = (d: V3): V3 => [d[0] * f.u[0] + d[1] * f.u[1] + d[2] * f.u[2], d[0] * f.v[0] + d[1] * f.v[1] + d[2] * f.v[2], d[0] * f.w[0] + d[1] * f.w[1] + d[2] * f.w[2]];
  const q = view === "front" ? quaternion(local(Z), local(X), local(Y)) : quaternion(local(Y), local(Z), local(X));
  return [...kept].sort((a, b) => a - b).map((k) => {
    const { s, drawn: [a, b], deep: [c, d] } = rows[k]!;
    const [cross, depth] = [(a + b) / 2, (c + d) / 2];
    const p: V3 = view === "front" ? [depth, s, cross] : [s, depth, cross];
    const [half, thick] = [Math.max((b - a) / 2, TINY), Math.max((d - c) / 2 - inset, TINY)];
    return { at: local([p[0] - f.o[0], p[1] - f.o[1], p[2] - f.o[2]]).map(round), rx: round(view === "front" ? half : thick), ry: round(view === "front" ? thick : half), rotation: q };
  });
}

/** 🧹️ The geometry without plates and bodies. */
function strip(geometry: any): void {
  for (const view of VIEWS) geometry[view] = geometry[view].filter((shape: any) => !made(shape));
}

/** 🔧️ The ops applied to the asset: plates and bodies removed, the suffixes set as saved, the saved bodies laid before the shape they stand under. */
function applyOps(target: any, ops: any): number {
  let count = 0;
  for (const key of Object.keys(ops)) {
    const geometry = target.representations[key]?.geometry;
    if (geometry === undefined) continue;
    strip(geometry);
    for (const view of VIEWS) {
      const carried = new Set<string>(ops[key]?.[view]?.carried ?? []);
      for (const shape of geometry[view]) { const base = baseId(String(shape.id)); if (typeof shape.id === "string") shape.id = carried.has(`${shape.bone}|${base}`) ? `${base}${SHEET}` : base; }
      for (const entry of ops[key]?.[view]?.bodies ?? []) {
        const at = geometry[view].findIndex((known: any) => known.bone === entry.shape.bone && baseId(String(known.id)) === entry.under);
        if (at < 0) throw new Error(`${target.id} 1:${key} ${view}: no shape "${entry.under}" on bone "${entry.shape.bone}" to lay the body under`);
        geometry[view].splice(at, 0, structuredClone(entry.shape));
        count++;
      }
    }
  }
  return count;
}

/** 🎯️ How far a point lies outside the outlines (0 inside). */
function outside(p: P, outlines: readonly (readonly (readonly P[])[])[]): number {
  let nearest = Infinity;
  for (const rings of outlines) {
    let inside = false;
    for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const [u, v] = [ring[i]!, ring[j]!];
      if ((u.y > p.y) !== (v.y > p.y) && p.x < u.x + ((v.x - u.x) * (p.y - u.y)) / (v.y - u.y)) inside = !inside;
      const [dx, dy] = [v.x - u.x, v.y - u.y];
      const t = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - u.x) * dx + (p.y - u.y) * dy) / (dx * dx + dy * dy)));
      nearest = Math.min(nearest, Math.hypot(p.x - u.x - t * dx, p.y - u.y - t * dy));
    }
    if (inside) return 0;
  }
  return nearest;
}

const opsFile = flag("ops");
const result: any = structuredClone(asset);
if (opsFile !== undefined) console.log(`${asset.id}: ${applyOps(result, JSON.parse(readFileSync(opsFile, "utf8")))} bodies laid from ${opsFile}`);
else {
  const ops: any = {};
  let total = 0;
  for (const key of authored) {
    const geometry = result.representations[key].geometry;
    const budget = (PAPER / 1000) * Number(key);
    const parts: Part[] = [];
    for (const view of VIEWS) {
      const carried = geometry[view].filter((shape: any) => String(shape.id).endsWith(SHEET)).map((shape: any) => `${shape.bone}|${baseId(shape.id)}`);
      (ops[key] ??= {})[view] = { carried, bodies: [] };
      for (const bone of [...new Set<string>(geometry[view].filter((shape: any) => String(shape.id).startsWith(PLATE)).map((shape: any) => shape.bone))]) {
        if (only !== undefined && !only.includes(bone)) continue;
        const list: any[] = geometry[view];
        const plate = list.find((shape) => shape.bone === bone && String(shape.id).startsWith(PLATE));
        const under = list.slice(list.indexOf(plate)).find((shape) => shape.bone === bone && !made(shape));
        if (under === undefined) { console.log(`left ${key} ${view} ${bone}: no shape above the plates`); continue; }
        parts.push({ key, view, bone, under: baseId(String(under.id)), common: Object.fromEntries(Object.keys(plate).filter((name) => COMMON.includes(name)).map((name) => [name, plate[name]])), inset: budget + MARGIN });
      }
    }
    const sheets = (view: View | "elevation", bone: string): P[][][] => geometry[view].filter((shape: any) => shape.bone === bone && sheetOf(shape)).map((shape: any) => flattenGeometry(shape).rings);
    const build = (part: Part): void => {
      const [drawn, side] = [sheets(part.view, part.bone), sheets("elevation", part.bone)];
      const found = drawn.length === 0 || side.length === 0 ? `${drawn.length} closed outlines, ${side.length} in the elevation` : sectionsOf(part.view, part.bone, drawn, side, budget, part.inset);
      if (typeof found === "string") { part.shape = undefined; console.log(`left ${key} ${part.view} ${part.bone}: ${found}`); return; }
      part.shape = { id: `${BODY}${part.bone}`, ...part.common, kind: "loft", sections: found };
      loftShapeIssues(part.shape, `${key} ${part.view} ${part.bone}`, (path: string, code: string, en: string) => console.log(`ISSUE ${code} ${path} ${en}`));
    };
    const lay = (): any => {
      const trial = structuredClone(result);
      applyOps(trial, { [key]: Object.fromEntries(VIEWS.map((view) => [view, { carried: ops[key][view].carried, bodies: parts.filter((part) => part.view === view && part.shape !== undefined).map((part) => ({ under: part.under, shape: part.shape })) }])) });
      return trial;
    };
    for (const part of parts) build(part);
    for (let round = 0; round <= 5; round++) {
      const state = evaluate(lay(), { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
      let again = 0;
      for (const part of parts.filter((entry) => entry.shape !== undefined)) {
        const here: any[] = state.shapes[part.view].filter((shape: any) => shape.shape.bone === part.bone);
        const body: P[] = here.filter((shape) => shape.shape.id === part.shape.id).flatMap((shape) => shape.rings).flat();
        const drawn: P[][][] = here.filter((shape) => !made(shape.shape) && shape.shape.filled === true).map((shape) => shape.rings);
        const beyond = body.length === 0 ? 0 : Math.max(...body.map((p) => outside(p, drawn)));
        if (beyond <= 1e-6) { if (round === 5 || again === 0) console.log(`${key} ${part.view} ${part.bone}: ${part.shape.sections.length} sections, drawn in ${mm(part.inset)} mm, inside the outlines of the default pose`); continue; }
        if (round === 5) { console.log(`left ${key} ${part.view} ${part.bone}: shows ${mm(beyond)} mm beside the outlines of the default pose`); part.shape = undefined; continue; }
        part.inset = part.inset * 1.5 + beyond;
        build(part);
        again++;
      }
      if (again === 0) break;
    }
    for (const part of parts) if (part.shape !== undefined) { ops[key][part.view].bodies.push({ under: part.under, shape: part.shape }); total++; }
    applyOps(result, { [key]: ops[key] });
  }
  for (const key of Object.keys(result.representations)) if (ops[key] === undefined && result.representations[key].geometry !== undefined) {
    ops[key] = Object.fromEntries(VIEWS.map((view) => [view, { carried: result.representations[key].geometry[view].filter((shape: any) => String(shape.id).endsWith(SHEET)).map((shape: any) => `${shape.bone}|${baseId(shape.id)}`), bodies: [] }]));
    applyOps(result, { [key]: ops[key] });
  }
  console.log(`${asset.id}: ${total} bodies`);
  const save = flag("save");
  if (save !== undefined) { writeFileSync(save, `${JSON.stringify(ops)}\n`); console.log(`${asset.id}: ops saved to ${save}`); }
}
let plates = 0, bodies = 0, sheets = 0;
for (const key of Object.keys(result.representations)) for (const view of VIEWS) for (const shape of result.representations[key].geometry?.[view] ?? []) { if (String(shape.id).startsWith(PLATE)) plates++; else if (String(shape.id).startsWith(BODY)) bodies++; else if (String(shape.id).endsWith(SHEET)) sheets++; }
console.log(`${asset.id}: ${plates} depth plates, ${bodies} bodies, ${sheets} sheets carried by the silhouette`);
next.asset = result;
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
