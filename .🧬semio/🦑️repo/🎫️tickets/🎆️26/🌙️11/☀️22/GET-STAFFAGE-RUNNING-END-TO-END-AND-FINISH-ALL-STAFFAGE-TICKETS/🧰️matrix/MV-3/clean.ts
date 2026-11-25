/** 🧹️ Thins the drawn outlines of a plant that a raster pass left as staircases: `bun clean.ts <id|file.json> [--out file] [--write] [--paper 0.05,50:elevation:0.03,…] [--over 300] [--scales 50,100] [--search ops.json | --ops ops.json]`.
 *
 * A path of more than `--over` points is thinned ring by ring (Douglas-Peucker, `--paper` millimetres on the paper of its scale, one value for all or an own value per scale and view, so no edge moves by more than a third of the thinnest line of the product; 0 leaves the view alone); a ring whose area is below (0.2 mm on paper) squared is dropped, the outer ring of a shape never. Paths with other commands than M, L and Z are left alone. The run is idempotent up to the thinning of already thinned rings, which keeps every point.
 * A silhouette guard that measures pockets of the foliage turns red when one thinned edge opens a pocket. `--search` therefore asks the guard of `🧊️catalog-silhouette` per scale and view: all paths at once, and where that fails by halving the list, so single paths that open a pocket take the next finer step of `--steps` (default 0.05,0.03,0.02) or stay as drawn; the accepted thinning per path is saved as `{ "<scale>:<view>:<id>": millimetres }`. `--ops` applies such a list (the reproducible step of the chain).
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

type P = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const paperOf = (key: string, view: string): number => {
  const entries = (flag("paper") ?? "0.05").split(",").map((entry) => entry.split(":"));
  return Number((entries.find((entry) => entry.length === 3 && entry[0] === key && entry[1] === view) ?? entries.find((entry) => entry.length === 1) ?? ["0.05"]).at(-1));
};
const OVER = Number(flag("over") ?? 300);
const scales = flag("scales")?.split(",");
const areaOf = (ring: readonly P[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;

function thin(ring: readonly P[], eps: number): P[] {
  if (ring.length < 5) return ring.slice() as P[];
  let far = 0;
  ring.forEach((p, i) => { if (Math.hypot(p[0] - ring[0]![0], p[1] - ring[0]![1]) > Math.hypot(ring[far]![0] - ring[0]![0], ring[far]![1] - ring[0]![1])) far = i; });
  const line = (run: P[]): P[] => {
    const keep = new Uint8Array(run.length);
    keep[0] = keep[run.length - 1] = 1;
    const stack: P[] = [[0, run.length - 1]];
    while (stack.length > 0) {
      const [a, b] = stack.pop()!;
      const [p, q] = [run[a]!, run[b]!];
      const length = Math.hypot(q[0] - p[0], q[1] - p[1]) || 1e-12;
      let worst = eps, hit = -1;
      for (let i = a + 1; i < b; i++) {
        const along = ((run[i]![0] - p[0]) * (q[0] - p[0]) + (run[i]![1] - p[1]) * (q[1] - p[1])) / length;
        const d = along < 0 ? Math.hypot(run[i]![0] - p[0], run[i]![1] - p[1]) : along > length ? Math.hypot(run[i]![0] - q[0], run[i]![1] - q[1]) : Math.abs((q[0] - p[0]) * (run[i]![1] - p[1]) - (q[1] - p[1]) * (run[i]![0] - p[0])) / length;
        if (d > worst) { worst = d; hit = i; }
      }
      if (hit >= 0) { keep[hit] = 1; stack.push([a, hit], [hit, b]); }
    }
    return run.filter((_, i) => keep[i] === 1);
  };
  return [...line(ring.slice(0, far + 1) as P[]).slice(0, -1), ...line([...ring.slice(far), ring[0]!] as P[]).slice(0, -1)];
}

type Entry = { key: string; view: string; shape: any; d: string; count: number };
const entries: Entry[] = [];
for (const [key, decision] of Object.entries(next.asset.representations) as [string, any][]) {
  if (decision.status !== "authored" || (scales !== undefined && !scales.includes(key))) continue;
  for (const [view, list] of Object.entries(decision.geometry) as [string, any[]][])
    for (const shape of list) {
      if (shape.kind !== "path" || shape.filled !== true || /[^MLZ\d\s.eE+-]/u.test(shape.d)) continue;
      entries.push({ key, view, shape, d: shape.d, count: (shape.d.match(/[ML]/gu) ?? []).length });
    }
}

/** 🪒️ The path thinned by `mm` millimetres on the paper of its scale; 0 gives the path as drawn. */
function thinned(entry: Entry, mm: number): string {
  if (mm === 0) return entry.d;
  const paper = Number(entry.key) / 1000;
  const rings = entry.d.split("Z").map((sub) => [...sub.matchAll(/(-?[\d.eE+-]+) (-?[\d.eE+-]+)/gu)].map((m) => [Number(m[1]), Number(m[2])] as P)).filter((ring) => ring.length > 0);
  const largest = Math.max(...rings.map(areaOf));
  return rings.filter((ring) => areaOf(ring) === largest || areaOf(ring) >= (0.2 * paper) ** 2).map((ring) => thin(ring, mm * paper)).filter((ring) => ring.length > 2).map((ring) => `M ${ring.map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ");
}

const idOf = (entry: Entry): string => `${entry.key}:${entry.view}:${entry.shape.id}`;
const chosen: Record<string, number> = flag("ops") !== undefined ? JSON.parse(readFileSync(flag("ops")!, "utf8")) : {};
if (flag("search") !== undefined) {
  const { S } = await import("../MR/lib.ts");
  const { join } = await import("node:path");
  const { measureView, worldSolids } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
  const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
  const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
  const STEPS = (flag("steps") ?? "0.05,0.03,0.02").split(",").map(Number);
  for (const key of [...new Set(entries.map((entry) => entry.key))]) {
    const scale = Number(key);
    const feature = featureOf(scale);
    let solids: unknown[] | undefined;
    const measure = (view: string): { hausdorffM: number; iou: number } => {
      const asset = structuredClone(next.asset);
      const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
      solids ??= worldSolids(asset, state.bones as never, scale).filter((solid: any) => solid.texture !== true);
      return measureView(asset, state, solids, view, scale, undefined, feature);
    };
    for (const view of ["elevation", "front", "plan"]) {
      const list = entries.filter((entry) => entry.key === key && entry.view === view && entry.count > OVER);
      if (list.length === 0) continue;
      const start = measure(view);
      const limit = Math.max(start.hausdorffM, 1.8 * feature);
      const passes = (): boolean => { const m = measure(view); return m.hausdorffM <= limit && m.iou >= Math.min(start.iou, 0.97); };
      const settle = (part: Entry[], mm: number): Entry[] => {
        const was = part.map((entry) => entry.shape.d);
        for (const entry of part) entry.shape.d = thinned(entry, mm);
        if (passes()) { for (const entry of part) chosen[idOf(entry)] = mm; return []; }
        part.forEach((entry, i) => { entry.shape.d = was[i]; });
        if (part.length === 1) return part;
        const half = Math.ceil(part.length / 2);
        return [...settle(part.slice(0, half), mm), ...settle(part.slice(half), mm)];
      };
      let open = list;
      for (const mm of STEPS) if (open.length > 0) open = settle(open, mm);
      const end = measure(view);
      console.log(`1:${key} ${view}: ${list.length} paths, ${list.reduce((sum, entry) => sum + entry.count, 0)} -> ${list.reduce((sum, entry) => sum + (entry.shape.d.match(/[ML]/gu) ?? []).length, 0)} points, ${open.length} as drawn, boundary ${(start.hausdorffM * 1000).toFixed(1)} -> ${(end.hausdorffM * 1000).toFixed(1)} mm (gate ${(2000 * feature).toFixed(1)}), IoU ${start.iou.toFixed(4)} -> ${end.iou.toFixed(4)}`);
      writeFileSync(flag("search")!, `${JSON.stringify(chosen, null, 2)}\n`);
    }
  }
}
let [before, after, shapes] = [0, 0, 0];
for (const entry of entries) {
  const mm = flag("search") === undefined && flag("ops") === undefined ? (entry.count > OVER ? paperOf(entry.key, entry.view) : 0) : (chosen[idOf(entry)] ?? 0);
  entry.shape.d = thinned(entry, mm);
  if (mm === 0) continue;
  shapes++;
  before += entry.count;
  after += (entry.shape.d.match(/[ML]/gu) ?? []).length;
}
console.log(`${next.asset.id}: ${shapes} paths, ${before} -> ${after} points`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${next.asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${next.asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
