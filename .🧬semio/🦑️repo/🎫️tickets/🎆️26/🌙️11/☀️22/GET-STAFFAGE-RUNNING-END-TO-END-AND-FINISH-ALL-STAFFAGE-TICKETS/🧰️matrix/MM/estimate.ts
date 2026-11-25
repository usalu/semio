/** ⏲️ Estimates the seconds the matrix of every asset takes, without running it: `bun estimate.ts --out <rows.jsonl> [--tree <root of a tree with 🎓️teaching>] [<asset id> …]` (default: the live tree and the whole catalog, one document at a time; ids that have a row are skipped).
 *
 * The matrix evaluates a subject once per scale entry and draws its modes and languages from that state, and it measures the extreme instants and the loops once per representation. Per authored representation the processor seconds (user and system, so a loaded host does not inflate them) of four parts are measured on the default pose at the preset scale: `first` (the print suitability, once per scale entry), `evaluate` (one state), `share` (what the first drawing of a state prepares for the others) and `draw` (one further mode). The counts follow the sampling of the engine; ten extreme instants per clip are assumed.
 * One JSON line per asset: `{ id, category, bytes, seconds, evaluations, draws, rssMb, representations: [{ key, first, evaluate, share, draw, full, reduced }] }`. Progress lines carry the `[DEBUG] ` prefix.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts — the engine whose sampling it counts
 */
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const S = join(resolve(flag("tree") ?? join(import.meta.dir, "../../../../../../../../..")), "🎓️teaching", "🏛️architecture", "🧍️staffage");
const { drawingsOf } = (await import(join(S, "🔨️modules/🔭️multiview/🟦️.ts"))) as typeof import("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts");
const { evaluate } = (await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"))) as typeof import("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts");
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = (await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"))) as typeof import("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts");
const { EXTREME_PROBES, MODE_LIST, TRANSITION_FRACTIONS, VIEWPOINT_LIST, clipInstants, drawOptions, mapCatalog, scaleEntries, scaleSelector, transitionPairs } = (await import(join(S, "🧪️tests/🔭️multiview-coverage/🧰️.ts"))) as typeof import("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts");

const out = resolve(flag("out")!);
const ids = args.filter((arg, index) => !arg.startsWith("--") && !(args[index - 1] ?? "").startsWith("--"));
const known = new Set(existsSync(out) ? readFileSync(out, "utf8").split("\n").filter(Boolean).map((line) => (JSON.parse(line) as { id: string }).id) : []);
const started = Date.now();
let count = 0;

function timed(run: () => unknown): number {
  const before = process.cpuUsage();
  try {
    run();
  } catch {}
  const used = process.cpuUsage(before);
  return (used.user + used.system) / 1e6;
}

function estimate(asset: Asset): { id: string } {
  count++;
  if (known.has(asset.id)) return { id: asset.id };
  const instants = asset.clips.map((clip) => clipInstants(asset, clip.id));
  const both = instants.reduce((sum, found) => sum + found.filter((instant) => instant.tag === "start" || instant.tag === "key" || instant.tag === "end").length + 10, 0);
  const english = instants.reduce((sum, found) => sum + found.filter((instant) => instant.tag === "mid" || instant.tag === "uniform").length, 0);
  const transitions = asset.poses.length > 1 ? transitionPairs(asset).length * TRANSITION_FRACTIONS.length : 0;
  const once = asset.clips.length * (EXTREME_PROBES + 1) + 2 * asset.clips.filter((clip) => clip.loop).length;
  const whole = { states: asset.poses.length + both + english + transitions, draws: MODE_LIST.length * (2 * asset.poses.length + 2 * both + english + transitions) };
  const edge = { states: asset.poses.length + 2 * asset.clips.length, draws: MODE_LIST.length * 2 * (asset.poses.length + 2 * asset.clips.length) };
  const entries = scaleEntries(asset).filter((entry) => entry.kind !== "alternative" && asset.representations[entry.representation as keyof Asset["representations"]]?.status === "authored");
  const query = { kind: "pose", pose: asset.presentation.defaultPose } as const;
  let evaluations = 0;
  let draws = 0;
  let seconds = 0;
  const representations = [...new Set(entries.map((entry) => entry.representation))].map((key) => {
    const denominator = Number(key);
    const source = { kind: "scale", denominator } as const;
    const first = timed(() => resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE));
    evaluate(asset, query, scaleSelector(denominator));
    const state = timed(() => evaluate(asset, query, scaleSelector(denominator)));
    const drawings = drawingsOf(asset, query);
    const opening = timed(() => drawings(VIEWPOINT_LIST, source, "light-line", drawOptions("light-line", denominator, false, "en")));
    const seven = timed(() => MODE_LIST.forEach((mode) => drawings(VIEWPOINT_LIST, source, mode, drawOptions(mode, denominator, false, "de"))));
    const draw = seven / MODE_LIST.length;
    const share = Math.max(0, opening - state - draw);
    const mine = entries.filter((entry) => entry.representation === key);
    const full = mine.filter((entry) => entry.kind === "preset" || entry.kind === "declared").length;
    const reduced = mine.length - full;
    const drawn = full * whole.states + reduced * edge.states;
    const pictures = full * whole.draws + reduced * edge.draws;
    evaluations += once + drawn;
    draws += pictures;
    seconds += mine.length * first + (once + drawn) * state + drawn * share + pictures * draw;
    return { key, first, evaluate: state, share, draw, full, reduced };
  });
  const rssMb = Math.round(process.memoryUsage().rss / 1048576);
  appendFileSync(out, `${JSON.stringify({ id: asset.id, category: asset.category, bytes: JSON.stringify(asset).length, seconds, evaluations, draws, rssMb, representations })}\n`);
  console.error(`[DEBUG] ${count} ${asset.id} estimate ${seconds.toFixed(0)} s (${evaluations} evaluations, ${draws} drawings) t ${((Date.now() - started) / 1000).toFixed(0)} s rss ${rssMb} MB`);
  return { id: asset.id };
}

mapCatalog(estimate, ids.length === 0 ? undefined : ids);
