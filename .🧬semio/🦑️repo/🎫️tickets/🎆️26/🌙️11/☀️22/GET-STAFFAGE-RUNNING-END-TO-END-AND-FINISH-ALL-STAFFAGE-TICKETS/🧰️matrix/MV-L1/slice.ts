/** 🍞️ Draws the front and the plan of every foliage mass as ONE real body whose projections are the authored outlines, as the last step of the chain of a plant: `bun slice.ts <id|file.json> [--save ops.json | --ops ops.json] [--out file] [--write] [--scales 50,100] [--only bone,bone] [--paper 0.05] [--bones regex]`.
 *
 * A foliage mass is one elevation outline, one front outline and one plan outline on its bone. The elevation outline turns rigidly in its own plane; the flat front and plan outline are one sheet at one depth and foreshorten when the wind turns the bone, and the depth plates `mvc-*` with the sheets `-mvs` of `../MV-1/carry.ts` mend the extents but show cuts, notches and slivers. This tool replaces the closed body outline of the bone in the front and in the plan by one `loft` shape (same id, paint order, role, tone, region and material):
 * - front: at every station of height the section is the ellipse that fills the lateral interval of the authored front outline and the forward interval of the authored elevation outline; at rest the projection to the front is the authored front outline (notches that undercut a height filled), and to the side the elevation outline;
 * - plan: at every station of the forward axis the section is the ellipse that fills the lateral interval of the authored plan outline and the height interval of the elevation outline.
 * The sections of one loft are parallel and their stations increase, so the body never folds. Stations are inserted until the straight edge between two of them is within `paper` millimetres on the paper of the scale of both outlines.
 * Every open stroke and the side elevation stay as authored. `--save` writes the shapes as absolute values; `--ops` applies such a file: it removes every `mvc-*` plate and every suffix `-mvs` and sets the listed shapes in the place of the shape of the same bone and id, so the step is idempotent and runs after `../MV-1/make.sh` whatever that step wrote.
 * @see ../MV-1/carry.ts — the plates this tool replaces
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts — `loftRings`, the sections the engine draws
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
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const only = flag("only")?.split(",");
const scales = flag("scales")?.split(",");
const bones = new RegExp(flag("bones") ?? "^fol-", "u");
const PAPER = Number(flag("paper") ?? 0.05);
const tolerance: number = engine.toleranceM(asset);
const PLATE = "mvc-";
const SHEET = "-mvs";
const COMMON = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const VIEWS: readonly View[] = ["front", "plan"];
const TINY = 5e-4;
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const rest = restFrames(asset.rig) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const mm = (m: number): string => (m * 1000).toFixed(0);
const baseId = (id: string): string => (id.endsWith(SHEET) ? id.slice(0, -SHEET.length) : id);
const sheetOf = (shape: any): boolean => shape.kind !== "loft" && shape.kind !== "member" && shape.solid === undefined && shape.role === "body" && shape.filled === true && flattenGeometry(shape).closed === true;

/** 🧹️ The geometry without the depth plates and without the sheet suffix. */
function strip(geometry: any): void {
  for (const view of VIEWS) {
    geometry[view] = geometry[view].filter((shape: any) => !String(shape.id).startsWith(PLATE));
    for (const shape of geometry[view]) if (String(shape.id).endsWith(SHEET)) shape.id = baseId(shape.id);
  }
}

/** 📏️ The interval of the other coordinate an outline covers at station `s` (the widest over all its parts), `along` the coordinate of the stations; nothing beyond its ends. */
function interval(rings: readonly (readonly P[])[], along: "x" | "y", s: number): [number, number] | undefined {
  const across = along === "x" ? "y" : "x";
  let lo = Infinity, hi = -Infinity;
  for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[j]!, ring[i]!];
    if (Math.min(a[along], b[along]) > s + 1e-12 || Math.max(a[along], b[along]) < s - 1e-12) continue;
    for (const c of Math.abs(b[along] - a[along]) < 1e-12 ? [a[across], b[across]] : [a[across] + ((b[across] - a[across]) * (s - a[along])) / (b[along] - a[along])]) { lo = Math.min(lo, c); hi = Math.max(hi, c); }
  }
  return hi >= lo ? [lo, hi] : undefined;
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

/** 🍞️ The sections of one mass in one view, or nothing when an outline is missing. */
function sectionsOf(view: View, bone: string, drawn: P[][], side: P[][], budget: number): any[] | undefined {
  const f = rest[bone]!.frame;
  const world = side.map((ring) => ring.map((p) => ({ x: f.o[0] + p.x * f.u[0] + p.y * f.v[0], y: f.o[1] + p.x * f.u[1] + p.y * f.v[1] })));
  const along: "x" | "y" = view === "front" ? "y" : "x";
  const reach = (rings: P[][]): [number, number] => { const list = rings.flat().map((p) => p[along]); return [Math.min(...list), Math.max(...list)]; };
  const [d0, d1] = reach(drawn);
  const [e0, e1] = reach(world);
  const eps = Math.min(0.001, (d1 - d0) / 200);
  const at = (s: number): { drawn: [number, number]; deep: [number, number] } => ({ drawn: interval(drawn, along, Math.min(Math.max(s, d0 + eps), d1 - eps))!, deep: interval(world, along, e0 + eps + ((Math.min(Math.max(s, d0 + eps), d1 - eps) - d0 - eps) * (e1 - e0 - 2 * eps)) / (d1 - d0 - 2 * eps))! });
  const stations = new Set<number>([d0 + eps, d1 - eps, ...drawn.flat().map((p) => p[along]).filter((s) => s > d0 + eps && s < d1 - eps)]);
  const all = [...stations].sort((a, b) => a - b).flatMap((s) => [s - 2e-4, s, s + 2e-4]).filter((s) => s >= d0 + eps && s <= d1 - eps);
  for (let i = 0; i <= 80; i++) all.push(d0 + eps + ((d1 - d0 - 2 * eps) * i) / 80);
  const fine = [...new Set(all.map((s) => Math.round(s * 1e5) / 1e5))].filter((s) => s >= d0 + eps / 2 && s <= d1 - eps / 2).sort((a, b) => a - b);
  const values = fine.map((s) => at(s));
  const kept = new Set<number>([0, fine.length - 1]);
  const split = (i: number, j: number): void => {
    let worst = 1, where = -1;
    for (let k = i + 1; k < j; k++) {
      const t = (fine[k]! - fine[i]!) / (fine[j]! - fine[i]!);
      const off = (key: "drawn" | "deep", side: 0 | 1): number => Math.abs(values[k]![key][side] - values[i]![key][side] - t * (values[j]![key][side] - values[i]![key][side]));
      const miss = Math.max(off("drawn", 0) / budget, off("drawn", 1) / budget, off("deep", 0) / (3 * budget), off("deep", 1) / (3 * budget));
      if (miss > worst) { worst = miss; where = k; }
    }
    if (where < 0) return;
    kept.add(where);
    split(i, where);
    split(where, j);
  };
  split(0, fine.length - 1);
  const X: V3 = [1, 0, 0], Y: V3 = [0, 1, 0], Z: V3 = [0, 0, 1];
  const local = (d: V3): V3 => [d[0] * f.u[0] + d[1] * f.u[1] + d[2] * f.u[2], d[0] * f.v[0] + d[1] * f.v[1] + d[2] * f.v[2], d[0] * f.w[0] + d[1] * f.w[1] + d[2] * f.w[2]];
  const q = view === "front" ? quaternion(local(Z), local(X), local(Y)) : quaternion(local(Y), local(Z), local(X));
  return [...kept].sort((a, b) => a - b).map((k) => {
    const s = fine[k]!, { drawn: [a, b], deep: [c, d] } = values[k]!;
    const [cross, depth] = [(a + b) / 2, (c + d) / 2];
    const p: V3 = view === "front" ? [depth, s, cross] : [s, depth, cross];
    const [half, thick] = [Math.max((b - a) / 2, TINY), Math.max((d - c) / 2, TINY)];
    return { at: local([p[0] - f.o[0], p[1] - f.o[1], p[2] - f.o[2]]).map(round), rx: round(view === "front" ? half : thick), ry: round(view === "front" ? thick : half), rotation: q };
  });
}

/** 🔧️ The ops applied to the asset: plates and suffixes removed, the listed shapes set in the place of the shape of their bone and id. */
function applyOps(target: any, ops: any): number {
  let count = 0;
  for (const key of Object.keys(target.representations)) {
    const geometry = target.representations[key].geometry;
    if (geometry === undefined) continue;
    strip(geometry);
    for (const view of VIEWS) for (const shape of ops[key]?.[view] ?? []) {
      const at = geometry[view].findIndex((known: any) => known.bone === shape.bone && known.id === shape.id);
      if (at < 0) throw new Error(`${target.id} 1:${key} ${view}: no shape "${shape.id}" on bone "${shape.bone}" to set`);
      geometry[view][at] = structuredClone(shape);
      count++;
    }
  }
  return count;
}

const opsFile = flag("ops");
const result: any = structuredClone(asset);
if (opsFile !== undefined) console.log(`${asset.id}: ${applyOps(result, JSON.parse(readFileSync(opsFile, "utf8")))} shapes set from ${opsFile}`);
else {
  const ops: any = {};
  const made: { key: string; view: View; bone: string; sheet: any }[] = [];
  let sections = 0;
  for (const key of authored) {
    const geometry = result.representations[key].geometry;
    strip(geometry);
    const budget = (PAPER / 1000) * Number(key);
    for (const view of VIEWS) for (const bone of [...new Set<string>(geometry[view].map((shape: any) => shape.bone))]) {
      if (only !== undefined && !only.includes(bone)) continue;
      const list: any[] = geometry[view];
      const sheets = list.filter((shape) => shape.bone === bone && sheetOf(shape));
      const side = geometry.elevation.filter((shape: any) => shape.bone === bone && sheetOf(shape));
      if (!bones.test(bone)) continue;
      if (sheets.length !== 1 || side.length === 0) { console.log(`left ${key} ${view} ${bone}: ${sheets.length} closed outlines, ${side.length} in the elevation`); continue; }
      const found = sectionsOf(view, bone, flattenGeometry(sheets[0]).rings, side.flatMap((shape: any) => flattenGeometry(shape).rings), budget);
      if (found === undefined) { console.log(`left ${key} ${view} ${bone}: no interval`); continue; }
      const shape = { ...Object.fromEntries(Object.keys(sheets[0]).filter((name) => COMMON.includes(name)).map((name) => [name, sheets[0][name]])), kind: "loft", sections: found };
      loftShapeIssues(shape as any, `${key} ${view} ${bone}`, (path: string, code: string, en: string) => console.log(`ISSUE ${code} ${path} ${en}`));
      list[list.indexOf(sheets[0])] = shape;
      ((ops[key] ??= {})[view] ??= []).push(shape);
      made.push({ key, view, bone, sheet: sheets[0] });
      sections += found.length;
      console.log(`${key} ${view} ${bone}: ${found.length} sections`);
    }
  }
  console.log(`${asset.id}: ${made.length} lofts, ${sections} sections`);
  for (const key of authored) {
    const t0 = performance.now();
    const state = evaluate(result, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
    const shapes = { front: state.shapes.front, plan: state.shapes.plan };
    const t1 = performance.now();
    const before = evaluate(previous.asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
    void before.shapes.front; void before.shapes.plan;
    const t2 = performance.now();
    let worst = { boundary: 0, at: "" };
    for (const item of made.filter((entry) => entry.key === key)) {
      const rings: P[][] = shapes[item.view].filter((shape: any) => shape.shape.bone === item.bone && shape.shape.kind === "loft").flatMap((shape: any) => shape.rings);
      const drawn: P[][] = flattenGeometry(item.sheet).rings;
      const distance = (p: P, list: P[][]): number => {
        let best = Infinity;
        for (const ring of list) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
          const [u, v] = [ring[i]!, ring[j]!];
          const [dx, dy] = [v.x - u.x, v.y - u.y];
          const t = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - u.x) * dx + (p.y - u.y) * dy) / (dx * dx + dy * dy)));
          best = Math.min(best, Math.hypot(p.x - u.x - t * dx, p.y - u.y - t * dy));
        }
        return best;
      };
      const boundary = Math.max(...drawn.flat().map((p) => distance(p, rings)), ...rings.flat().map((p) => distance(p, drawn)));
      console.log(`rest ${key} ${item.view} ${item.bone}: boundary ${mm(boundary)} mm, ${rings.length} rings (authored ${drawn.length}), ${rings.flat().length} points (authored ${drawn.flat().length})`);
      if (boundary > worst.boundary) worst = { boundary, at: `${item.view} ${item.bone}` };
    }
    console.log(`1:${key}: one evaluation of front and plan ${(t1 - t0).toFixed(0)} ms (document before ${(t2 - t1).toFixed(0)} ms); worst boundary distance at rest ${mm(worst.boundary)} mm (${worst.at})`);
  }
  const save = flag("save");
  if (save !== undefined) { writeFileSync(save, `${JSON.stringify(ops)}\n`); console.log(`${asset.id}: ops saved to ${save}`); }
}
let plates = 0, sheets = 0, lofts = 0;
for (const key of Object.keys(result.representations)) for (const view of VIEWS) for (const shape of result.representations[key].geometry?.[view] ?? []) { if (String(shape.id).startsWith(PLATE)) plates++; else if (String(shape.id).endsWith(SHEET)) sheets++; else if (shape.kind === "loft") lofts++; }
console.log(`${asset.id}: tolerance ${mm(tolerance)} mm; ${plates} depth plates, ${sheets} sheets carried by the silhouette, ${lofts} lofts`);
next.asset = result;
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
