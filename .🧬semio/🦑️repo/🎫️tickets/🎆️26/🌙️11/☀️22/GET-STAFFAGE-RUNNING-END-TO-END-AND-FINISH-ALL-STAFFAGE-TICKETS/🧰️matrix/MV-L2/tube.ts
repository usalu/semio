/** 🌾️ Draws the slender parts of a plant (blades, plume strokes, culms, twigs) as the bodies they already have, in the place of flat sheets on depth plates: `bun tube.ts <id|file.json> [--save ops.json | --ops ops.json] [--out file] [--write] [--scales 50,100] [--views front,plan] [--only bone,bone] [--keep bone,bone]`.
 *
 * `carry.ts` laid flat plates `mvc-*` under the front and plan strokes of the bones the wind turns and renamed the strokes to `-mvs`; in a wind pose the plates show as boxes, horns and clots beside the strokes. Every such stroke has its body in the volume of the scale: a polyhedron of the same id on the same bone, a tube of rings along the stroke. This tool sets, for every sheet `<id>-mvs` of a front or plan view whose polyhedron is such a tube, ONE `loft` shape `<id>` in the place of the sheet (same paint order, role, tone, region, material and pen): one section per ring of the tube, the ellipse over the two principal half extents of the ring about its centre, in the plane of the ring. The engine turns that body itself, so the stroke is the true projection of its body in every pose, clip instant and transition, and no plate is needed: the plates of a bone are removed when every sheet of the bone in that view became a loft (a bone of `--keep`, or one with a sheet that has no tube, keeps its plates and sheets and is named).
 * Tube layouts read from the triangles (both generators of the catalog): rings of m vertices in order, with or without two lid centres after the last ring.
 * The run prints per scale and view the lofts, the bones left, and how far the box of a loft is from the box of the sheet it replaces in the default pose (the largest over the view). `--save` writes the lofts as absolute values; `--ops` applies such a file: it sets every saved loft in the place of the shape of the same bone and id (with or without `-mvs`) and removes the `mvc-*` plates of the saved bones, so the step is idempotent.
 * @see ../MV-2/carry.ts — the plates and sheets this tool replaces
 * @see ../MV-3/body.ts — the same for foliage masses, whose outline is not a tube
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { loftShapeIssues } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volume", "🟦️.ts"));

type View = "front" | "plan";
type V3 = [number, number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const only = flag("only")?.split(",");
const keep = new Set(flag("keep")?.split(",") ?? []);
const scales = flag("scales")?.split(",");
const VIEWS: readonly View[] = (flag("views")?.split(",") as View[] | undefined) ?? ["front", "plan"];
const [PLATE, SHEET, TINY] = ["mvc-", "-mvs", 5e-4];
const COMMON = ["role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const round = (n: number): number => Math.round(n * 1e6) / 1e6;
const baseId = (id: string): string => (id.endsWith(SHEET) ? id.slice(0, -SHEET.length) : id);
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V3): V3 => { const n = Math.hypot(...a); return n === 0 ? [0, 0, 0] : [a[0] / n, a[1] / n, a[2] / n]; };

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

/** 💍️ The rings of a tube polyhedron, or the reason it is none. */
function ringsOf(solid: any): V3[][] | string {
  const vertices: V3[] = solid.vertices, triangles: number[][] = solid.triangles;
  if (solid.kind !== "polyhedron" || solid.transform !== undefined || triangles.length < 4) return "no plain polyhedron";
  const [first, second] = [triangles[0]!, triangles[1]!];
  const [m, lids] = second[0] === first[0] ? [second[2]!, 0] : [first[2]!, 2];
  if (m < 3 || (vertices.length - lids) % m !== 0 || (vertices.length - lids) / m < 2) return `no tube (${vertices.length} vertices, ring of ${m})`;
  const body = vertices.length - lids;
  for (const triangle of triangles) { const own = triangle.filter((index) => index < body); if (Math.max(...own) - Math.min(...own) > 2 * m - 1) return "no tube (a face spans more than two rings)"; }
  return Array.from({ length: body / m }, (_, j) => vertices.slice(j * m, (j + 1) * m));
}

/** 🧮️ The unit eigenvectors of a symmetric 3 by 3 matrix, largest eigenvalue first (Jacobi). */
function axes(matrix: number[][]): V3[] {
  const a = matrix.map((row) => [...row]);
  const v = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  for (let sweep = 0; sweep < 32; sweep++) for (const [p, q] of [[0, 1], [0, 2], [1, 2]] as const) {
    if (Math.abs(a[p]![q]!) < 1e-18) continue;
    const theta = (a[q]![q]! - a[p]![p]!) / (2 * a[p]![q]!);
    const t = Math.sign(theta || 1) / (Math.abs(theta) + Math.sqrt(theta * theta + 1));
    const [c, s] = [1 / Math.sqrt(t * t + 1), t / Math.sqrt(t * t + 1)];
    for (let k = 0; k < 3; k++) { const [x, y] = [a[k]![p]!, a[k]![q]!]; a[k]![p] = c * x - s * y; a[k]![q] = s * x + c * y; }
    for (let k = 0; k < 3; k++) { const [x, y] = [a[p]![k]!, a[q]![k]!]; a[p]![k] = c * x - s * y; a[q]![k] = s * x + c * y; }
    for (let k = 0; k < 3; k++) { const [x, y] = [v[k]![p]!, v[k]![q]!]; v[k]![p] = c * x - s * y; v[k]![q] = s * x + c * y; }
  }
  return [0, 1, 2].sort((i, j) => a[j]![j]! - a[i]![i]!).map((i) => unit([v[0]![i]!, v[1]![i]!, v[2]![i]!]));
}

/** 🫧️ The loft sections of a tube: one per ring, in the plane of the ring. */
function sectionsOf(rings: V3[][]): any[] {
  const centres = rings.map((ring) => ring.reduce((sum, p) => [sum[0] + p[0] / ring.length, sum[1] + p[1] / ring.length, sum[2] + p[2] / ring.length] as V3, [0, 0, 0] as V3));
  const sections: any[] = [];
  let last: [V3, V3] | undefined;
  const made = rings.map((ring, j) => {
    const c = centres[j]!;
    const tangent = unit(sub(centres[Math.min(j + 1, rings.length - 1)]!, centres[Math.max(j - 1, 0)]!));
    const spread = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    for (const p of ring) { const d = sub(p, c); for (let i = 0; i < 3; i++) for (let k = 0; k < 3; k++) spread[i]![k]! += d[i]! * d[k]!; }
    const reach = Math.max(...ring.map((p) => Math.hypot(...sub(p, c))));
    let pair: [V3, V3] | undefined = reach > TINY ? (axes(spread).slice(0, 2) as [V3, V3]) : last;
    if (pair === undefined) return { c, reach, tangent, pair: undefined as [V3, V3] | undefined, ring };
    last = pair;
    return { c, reach, tangent, pair, ring };
  });
  const known = made.find((entry) => entry.pair !== undefined)?.pair;
  for (const entry of made) {
    let [x, y] = entry.pair ?? known ?? [[0, 1, 0] as V3, [0, 0, 1] as V3];
    let normal = unit(cross(x, y));
    if (dot(normal, entry.tangent) < 0) { y = [-y[0], -y[1], -y[2]]; normal = [-normal[0], -normal[1], -normal[2]]; }
    const at = entry.c.map(round);
    if (sections.length > 0 && Math.hypot(...sub(at as V3, sections[sections.length - 1].at)) < 1e-5) continue;
    sections.push({ at, rx: round(Math.max(TINY, ...entry.ring.map((p) => Math.abs(dot(sub(p, entry.c), x))))), ry: round(Math.max(TINY, ...entry.ring.map((p) => Math.abs(dot(sub(p, entry.c), y))))), rotation: quaternion(x, y, normal) });
  }
  return sections;
}

/** 🔧️ The ops applied to the asset: every saved loft in the place of the shape of its bone and id, the plates of the saved bones removed. */
function applyOps(target: any, ops: any): number {
  let count = 0;
  for (const key of Object.keys(ops)) for (const view of Object.keys(ops[key])) {
    const geometry = target.representations[key]?.geometry;
    if (geometry === undefined) continue;
    const bones = new Set<string>(ops[key][view].bones);
    geometry[view] = geometry[view].filter((shape: any) => !(String(shape.id).startsWith(PLATE) && bones.has(shape.bone)));
    for (const shape of ops[key][view].lofts) {
      const at = geometry[view].findIndex((known: any) => known.bone === shape.bone && baseId(String(known.id)) === shape.id);
      if (at < 0) throw new Error(`${target.id} 1:${key} ${view}: no shape "${shape.id}" on bone "${shape.bone}"`);
      geometry[view][at] = structuredClone(shape);
      count++;
    }
  }
  return count;
}

const box = (rings: { x: number; y: number }[][]): number[] => { const points = rings.flat(); return [Math.min(...points.map((p) => p.x)), Math.max(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.y))]; };
const opsFile = flag("ops");
const result: any = structuredClone(asset);
if (opsFile !== undefined) console.log(`${asset.id}: ${applyOps(result, JSON.parse(readFileSync(opsFile, "utf8")))} lofts set from ${opsFile}`);
else {
  const ops: any = {};
  let total = 0;
  for (const key of authored) {
    const geometry = result.representations[key].geometry;
    const solids = new Map<string, any>((asset.volumes?.[key]?.body ?? []).map((solid: any) => [`${solid.bone}|${solid.id}`, solid]));
    ops[key] = {};
    for (const view of VIEWS) {
      const list: any[] = geometry[view];
      const lofts: any[] = [], bones: string[] = [];
      const carried = [...new Set<string>(list.filter((shape) => String(shape.id).startsWith(PLATE) || String(shape.id).endsWith(SHEET)).map((shape) => shape.bone))].filter((bone) => only === undefined || only.includes(bone));
      for (const bone of carried) {
        if (keep.has(bone)) { console.log(`left ${key} ${view} ${bone}: kept by --keep`); continue; }
        const sheets = list.filter((shape) => shape.bone === bone && String(shape.id).endsWith(SHEET));
        const found: any[] = [], reasons: string[] = [];
        for (const sheet of sheets) {
          const id = baseId(String(sheet.id));
          const solid = solids.get(`${bone}|${id}`);
          const rings = solid === undefined ? "no solid of this id on the bone" : ringsOf(solid);
          if (typeof rings === "string") { reasons.push(`${id}: ${rings}`); continue; }
          const shape = { id, bone, ...Object.fromEntries(COMMON.filter((name) => sheet[name] !== undefined).map((name) => [name, sheet[name]])), kind: "loft", sections: sectionsOf(rings) };
          const issues: string[] = [];
          if (shape.sections.length < 2) issues.push("fewer than two sections");
          else loftShapeIssues(shape, `${key} ${view} ${id}`, (path: string, code: string, en: string) => issues.push(`${code} ${en}`));
          if (issues.length > 0) reasons.push(`${id}: ${issues[0]}`);
          else found.push(shape);
        }
        if (reasons.length > 0) console.log(`left ${key} ${view} ${bone}: ${reasons.length} of ${sheets.length} sheets have no tube (${reasons.slice(0, 3).join("; ")})${found.length > 0 ? `; its ${found.length} tubes are set, its plates stay` : ""}`);
        else if (sheets.length > 0 || list.some((shape) => shape.bone === bone && String(shape.id).startsWith(PLATE))) bones.push(bone);
        lofts.push(...found);
      }
      ops[key][view] = { bones, lofts };
      total += lofts.length;
    }
    const before = evaluate(structuredClone(result), { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
    applyOps(result, { [key]: ops[key] });
    const after = evaluate(structuredClone(result), { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
    for (const view of VIEWS) {
      let worst = 0, where = "";
      for (const shape of ops[key][view].lofts) {
        const was = before.shapes[view].find((entry: any) => entry.shape.bone === shape.bone && baseId(String(entry.shape.id)) === shape.id);
        const now = after.shapes[view].find((entry: any) => entry.shape.bone === shape.bone && entry.shape.id === shape.id);
        if (was === undefined || now === undefined || now.rings.flat().length === 0) { console.log(`ISSUE ${key} ${view} ${shape.id}: not drawn`); continue; }
        const [a, b] = [box(was.rings), box(now.rings)];
        const off = Math.max(...a.map((value, i) => Math.abs(value - b[i]!)));
        if (off > worst) { worst = off; where = `${shape.id} on ${shape.bone}`; }
      }
      console.log(`${key} ${view}: ${ops[key][view].lofts.length} lofts, plates of ${ops[key][view].bones.length} bones removed; box of a loft at most ${(worst * 1000).toFixed(1)} mm from the box of its sheet in the default pose${where === "" ? "" : ` (${where})`}`);
    }
  }
  console.log(`${asset.id}: ${total} lofts`);
  const save = flag("save");
  if (save !== undefined) { writeFileSync(save, `${JSON.stringify(ops)}\n`); console.log(`${asset.id}: ops saved to ${save}`); }
}
let plates = 0, lofts = 0, sheets = 0;
for (const key of Object.keys(result.representations)) for (const view of ["front", "plan"]) for (const shape of result.representations[key].geometry?.[view] ?? []) { if (String(shape.id).startsWith(PLATE)) plates++; else if (shape.kind === "loft") lofts++; else if (String(shape.id).endsWith(SHEET)) sheets++; }
console.log(`${asset.id}: ${plates} depth plates, ${lofts} lofts, ${sheets} sheets carried by the silhouette`);
next.asset = result;
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
