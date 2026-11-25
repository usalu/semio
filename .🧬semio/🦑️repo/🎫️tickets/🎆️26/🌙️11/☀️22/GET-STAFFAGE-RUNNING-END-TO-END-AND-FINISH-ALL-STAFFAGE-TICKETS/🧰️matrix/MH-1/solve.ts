/** 🧩️ Settles the repair of one asset in ONE process (the rule of `tune.py`, without one queued run per candidate): `bun solve.ts <base.json> --plan <plan.json> --ops <settled.json> --out <trial.json>` reads the document the batch started from, applies the entries of the plan in order on a copy and keeps of every entry only what holds the silhouette guard, writes the kept ops (the file `fit.ts` replays, the last step of the asset's chain) and the trial document. Run it from `S/📦️packages/🟦️typescript`.
 *
 * A plan entry is
 * - an op of `fit.ts` with `"fixed": true`: applied without a measurement (an anchor, a `set`);
 * - an op of `fit.ts`: one candidate;
 * - `{ "op": "volume", scale, id, "as": "auto" }`: the candidates `revolved`, `prism-w`, `prism-v`, `prism-u` and `prism-w` with the drawn elevation outline, of which the best is kept (`"order": […]` states other kinds, among them `loft` and `loft:<pose>`);
 * - `{ "choose": [[op…], [op…]], scale, label }`: candidates of several ops each (each without the scale); `"best": true` keeps the best instead of the first that holds.
 *
 * - `{ "search": true, scale, id, "ids"?, "subsets"?, "order"? }`: the candidates are the kinds of `volume` in every subset of the three views (the part redrawn as the solid of the volume in one or two views only, the other views kept as drawn), and a candidate is kept only when it ALSO has fewer hard findings of the matrix (`geometryFindings` over the poses and every `--stride`-th clip instant and transition at that scale, default 3) than the document before it; the best is the one with the fewest hard findings, then the smallest silhouette score; the first candidate that leaves no hard finding ends the search. `"sync": true` on any other entry adds the same condition.
 *
 * A candidate holds when, at its scale and against the base document, no check of the silhouette guard fails that did not fail, the IoU of every view is at most 0.02 below the base (never below 0.924 unless the base is), the largest boundary distance of every view is at most max(base, 90 % of the limit of the scale), and the validator issues with the print violations of the scale (`measureSuitability`) and the three-views findings are not more. The best candidate is the one with the smallest sum over the views of (1 − IoU) + boundary distance / limit.
 * @see ./fit.ts — the ops
 * @see ./tune.py — the same rule, one queued run per candidate */
import { join } from "node:path";
import { parseArgs } from "node:util";
import { S, readDoc } from "../MR/lib.ts";
import { applyOps } from "./fit.ts";

const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { validateAsset } = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
const { consistency, sweep } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const engine = await import(join(S, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));

type Metrics = { fails: number; views: Record<string, [number, number]>; validate: number; sweep: number };
const { values, positionals } = parseArgs({ allowPositionals: true, options: { plan: { type: "string" }, ops: { type: "string" }, out: { type: "string" }, stride: { type: "string" } } });
const read = readDoc(positionals[0]!);
const plan = JSON.parse(await Bun.file(values.plan!).text()) as any[];

/** 🖨️ The shapes the print profile of the check CLI refuses at one scale (strokes, gaps, features too small on paper). */
function unprintable(asset: any, scale: string): number {
  try {
    return measureSuitability(asset, asset.representations[scale], Number(scale), DEFAULT_PRINT_PROFILE).violations.length;
  } catch {
    return 100;
  }
}

/** 📐️ The silhouette guard of one scale with the validator and three-views counts. */
function metrics(asset: any, scale: string): Metrics {
  const feature = featureOf(Number(scale));
  const m = measureAsset(asset, Number(scale), undefined, undefined, feature);
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  let fails = m.bbox.filter((b: number, k: number) => Math.abs(b - dims[k]!) > feature).length;
  const views: Record<string, [number, number]> = {};
  for (const view of VIEWPOINTS) {
    views[view] = [m.views[view].iou, m.views[view].hausdorffM * 1000];
    if (m.views[view].iou < 0.92) fails++;
    if (m.views[view].hausdorffM > 2 * feature) fails++;
  }
  return { fails, views, validate: validateAsset(asset).length + unprintable(asset, scale), sweep: [...sweep(asset, Number(scale)), ...consistency(asset, Number(scale))].length };
}

const stride = Number(values.stride ?? "3");
const SUBSETS = [["plan"], ["front"], ["elevation"], ["front", "plan"], ["elevation", "front"], ["elevation", "plan"], ["elevation", "front", "plan"]];

/** 🚦️ The hard findings of the matrix at one scale over the poses and every `stride`-th other subject (clip instants, transitions). */
function hardOf(asset: any, scale: string): number {
  let count = 0;
  engine.subjects(asset, Number(scale)).forEach((subject: any, index: number) => {
    if (subject.query.kind !== "pose" && index % stride !== 0) return;
    try {
      const state = evaluate(asset, subject.query, engine.scaleSelector(Number(scale)));
      count += engine.geometryFindings(asset, state, subject.isDefault).filter((finding: any) => finding.tier === "hard").length;
    } catch {
      count += 1000;
    }
  });
  return count;
}

/** 🔭️ Where the shapes of the ids a candidate names lie in the default pose, per view (paper x and y ranges in metres): what a shape covered before and what it covers now. */
function extents(asset: any, scale: string, ids: readonly string[]): string {
  try {
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
    const span = (values: number[]): string => (values.length === 0 ? "-" : `${Math.min(...values).toFixed(3)},${Math.max(...values).toFixed(3)}`);
    return VIEWPOINTS.map((view: string) => {
      const points = state.shapes[view].filter((placed: any) => ids.includes(placed.shape.id)).flatMap((placed: any) => placed.rings.flat());
      return `${view[0]} x[${span(points.map((p: any) => p.x))}] y[${span(points.map((p: any) => p.y))}]`;
    }).join(" ");
  } catch (error) {
    return `no extents: ${(error as Error).message.slice(0, 80)}`;
  }
}

const limitOf = (scale: string): number => 2 * featureOf(Number(scale)) * 1000;
const holds = (m: Metrics, ref: Metrics, scale: string): boolean =>
  m.fails <= ref.fails && m.validate <= ref.validate && m.sweep <= ref.sweep && VIEWPOINTS.every((view: string) => m.views[view]![0] >= Math.min(ref.views[view]![0], Math.max(0.924, ref.views[view]![0] - 0.02)) && m.views[view]![1] <= Math.max(ref.views[view]![1], 0.9 * limitOf(scale)) + 0.05);
const score = (m: Metrics, scale: string): number => VIEWPOINTS.reduce((sum: number, view: string) => sum + (1 - m.views[view]![0]) + m.views[view]![1] / limitOf(scale), 0);
const show = (m: Metrics): string => `${VIEWPOINTS.map((view: string) => `${view} ${m.views[view]![0].toFixed(4)}/${m.views[view]![1].toFixed(1)}`).join(" ")} fails ${m.fails} validate ${m.validate} sweep ${m.sweep}`;

let current = structuredClone(read.doc);
const base: Record<string, Metrics> = {};
const settled: any[] = [];
const dropped: string[] = [];
for (const entry of plan) {
  if (entry.fixed === true) {
    const { fixed: _fixed, ...op } = entry;
    const { lines } = applyOps(current.asset, [op]);
    console.log(`  fixed ${lines[0]}`.slice(0, 400));
    settled.push(op);
    continue;
  }
  const scale = entry.scale as string;
  const label = entry.label ?? `${entry.search === true ? "search" : entry.op} ${entry.id}`;
  const kinds = (entry.order as string[] | undefined) ?? ["revolved", "prism-w", "prism-v", "prism-u", "prism-w:elevation"];
  const { choose: _choose, best: _best, label: _label, order: _order, sync: _sync, search: _search, subsets: _subsets, ...plain } = entry;
  const wantSync = entry.search === true || entry.sync === true;
  const candidates: [string, any[]][] =
    entry.search === true
      ? ((entry.subsets as string[][] | undefined) ?? SUBSETS).flatMap((views) => kinds.map((kind) => [`${kind}@${views.join("+")}`, [{ ...plain, op: "volume", views, as: kind.split(":")[0], ...(kind.includes(":") ? { [kind.startsWith("loft") ? "pose" : "profile"]: kind.split(":")[1] } : {}) }]] as [string, any[]]))
      : entry.choose !== undefined
      ? (entry.choose as any[][]).map((ops, i) => [`choice ${i + 1}`, ops.map((op) => ({ scale, ...op }))])
      : entry.op === "volume" && entry.as === "auto"
        ? kinds.map((kind) => [kind, [{ ...plain, as: kind.split(":")[0], ...(kind.includes(":") ? { [kind.startsWith("loft") ? "pose" : "profile"]: kind.split(":")[1] } : {}) }]])
        : [[entry.op, [plain]]];
  const pickBest = entry.best === true || entry.search === true || (entry.op === "volume" && entry.as === "auto");
  const hardBefore = wantSync ? hardOf(current.asset, scale) : 0;
  if (wantSync) console.log(`  1:${scale} ${label}: ${hardBefore} hard findings before (stride ${stride})`);
  base[scale] ??= metrics(read.doc.asset, scale);
  let kept: { name: string; ops: any[]; doc: any; value: number; hard: number } | undefined;
  const named = [...new Set(candidates.flatMap(([, ops]) => ops.flatMap((op) => [op.id, ...Object.values(op.ids ?? {})] as string[])))];
  console.log(`  1:${scale} ${label} drawn: ${extents(current.asset, scale, named)}`);
  for (const [name, ops] of candidates) {
    const trial = structuredClone(current);
    const { lines, refused } = applyOps(trial.asset, ops);
    if (refused > 0) {
      console.log(`  1:${scale} ${label} ${name}: refused: ${lines.find((line) => line.includes("REFUSED"))?.split("REFUSED: ")[1]?.slice(0, 160)}`);
      continue;
    }
    let m: Metrics;
    try {
      m = metrics(trial.asset, scale);
    } catch (error) {
      console.log(`  1:${scale} ${label} ${name}: no measure: ${(error as Error).message.slice(0, 160)}`);
      continue;
    }
    const guard = holds(m, base[scale]!, scale);
    const hardNow = guard && wantSync ? hardOf(trial.asset, scale) : 0;
    const good = guard && (!wantSync || hardNow < hardBefore);
    console.log(`  1:${scale} ${label} ${name}: ${show(m)} ${good ? "holds" : guard ? "no gain" : "no"} ${score(m, scale).toFixed(4)}${guard && wantSync ? ` hard ${hardNow}` : ""} | ${extents(trial.asset, scale, named)}`);
    if (good && (kept === undefined || hardNow < kept.hard || (hardNow === kept.hard && score(m, scale) < kept.value))) kept = { name, ops, doc: trial, value: score(m, scale), hard: hardNow };
    if (good && (!pickBest || (wantSync && hardNow === 0))) break;
  }
  if (kept === undefined) {
    dropped.push(`1:${scale} ${label}`);
    console.log(`  1:${scale} ${label}: NO candidate holds the silhouette guard${wantSync ? " and lowers the hard findings" : ""} (base ${show(base[scale]!)}), dropped`);
    continue;
  }
  console.log(`  1:${scale} ${label}: KEEP ${kept.name}`);
  current = kept.doc;
  settled.push(...kept.ops);
}
for (const scale of Object.keys(base)) console.log(`1:${scale} base  ${show(base[scale]!)}\n1:${scale} trial ${show(metrics(current.asset, scale))}`);
const second = read.text.split("\n", 2)[1] ?? "";
await Bun.write(values.out!, JSON.stringify(current, null, second.length - second.trimStart().length) + (read.text.endsWith("\n") ? "\n" : ""));
await Bun.write(values.ops!, `[\n${settled.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
console.log(`${read.doc.asset.id}: ${settled.length} ops settled, ${dropped.length} dropped${dropped.length ? `: ${dropped.join("; ")}` : ""}`);
