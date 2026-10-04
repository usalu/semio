/** 🌍️ Work package A5: runs the reference world of the clearance unit suite for the published metrics and ablations.
 *
 * From the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/clearance_world.ts [--seeds 24] [--ticks 30000] [--span 1]
 *       [--ablation-seeds 12] [--ablation-ticks 15000] [--trace 0] [--teeth 0] [--without <rule>] [--name full]
 *
 * The world lives in `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🟦️.ts` and is exported from there; Vitest
 * is replaced by an idle stand-in here, so importing the suite registers no test. Output: `🗑️generated/a5/<name>.json`
 * (every sweep with its tally and counts) and `<name>.md` (the tables). `--trace n` also writes `trace-<name>-<seed>.json`
 * for the first `n` seeds (the boxes of every body at every tick, for `clearance_trace_check.py`); `--without <rule>`
 * runs the first sweep with one rule switched off, a negative control for that check. `--teeth n` looks, rule
 * by rule, for the first tick with an overlap in each of the seeds 1…n (within `--ticks`) and writes `<name>-teeth.json`:
 * the unit suite takes its cheapest ablation runs from it.
 */
import { plugin } from "bun";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const idle = (): void => {};
plugin({
  name: "vitest-stand-in",
  setup(build) {
    build.module("vitest", () => ({ exports: { describe: Object.assign(idle, { each: () => idle }), it: Object.assign(idle, { each: () => idle }), expect: idle }, loader: "object" }));
  },
});

const here = dirname(fileURLToPath(import.meta.url));
const suite = await import(resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🟦️.ts"));
const { RULES, openWorld, tickWorld, bodiesOf } = suite;

/** 🔧️ A numeric option of the command line. */
function option(name: string, fallback: number): number {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : Number(process.argv[at + 1]);
}

/** 🏷️ A text option of the command line. */
function label(name: string, fallback: string): string {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : String(process.argv[at + 1]);
}

const out = resolve(here, "🗑️generated/a5");
mkdirSync(out, { recursive: true });

type Sum = Record<string, number>;
type Box = { extent: { x0: number; y0: number; x1: number; y1: number } };

const span = option("span", 1);

/** ➕️ Runs one rule set over the seeds and adds everything up. */
function sweep(rules: unknown, seeds: number, ticks: number, traces: number): Sum {
  const sum: Sum = { seeds, ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 };
  for (let seed = 1; seed <= seeds; seed++) {
    const world = openWorld(seed, rules, span);
    const trace: number[][][] = [];
    for (let tick = 0; tick < ticks; tick++) {
      tickWorld(world);
      if (seed <= traces) trace.push(bodiesOf(world).map((body: Box) => [body.extent.x0, body.extent.y0, body.extent.x1, body.extent.y1]));
    }
    if (seed <= traces) writeFileSync(resolve(out, `trace-${label("name", "full")}-${seed}.json`), JSON.stringify({ seed, ticks, overlaps: world.tally.overlaps, boxes: trace }));
    for (const [key, value] of Object.entries(world.tally)) sum[key] = (sum[key] ?? 0) + (value as number);
    for (const [key, value] of Object.entries(world.counts)) sum[key] = (sum[key] ?? 0) + (value as number);
    sum.pets = (sum.pets ?? 0) + world.pets.length;
  }
  return sum;
}

/** 🦷️ The first tick with an overlap for a rule set and a seed, or −1 within the ticks. */
function bite(rules: unknown, seed: number, ticks: number): number {
  const world = openWorld(seed, rules, span);
  for (let tick = 1; tick <= ticks; tick++) {
    tickWorld(world);
    if (world.tally.overlaps > 0) return tick;
  }
  return -1;
}

const seeds = option("seeds", 24);
const ticks = option("ticks", 30000);
const ablationSeeds = option("ablation-seeds", 12);
const ablationTicks = option("ablation-ticks", 15000);
const teeth = option("teeth", 0);
const name = label("name", "full");
const core = ["guard", "corridors", "vetting", "rests", "projection", "eviction"];
const sets: Record<string, Record<string, boolean>> = {};
for (const rule of Object.keys(RULES)) sets[`without ${rule}`] = { ...RULES, [rule]: false };
sets["without heads and steering"] = { ...RULES, heads: false, steering: false };
sets["without heads, steering and poof"] = { ...RULES, heads: false, steering: false, poof: false };

if (teeth > 0) {
  const found: Record<string, number[]> = {};
  for (const rule of [...core.map((entry) => `without ${entry}`), "without heads, steering and poof"]) {
    found[rule] = [];
    for (let seed = 1; seed <= teeth; seed++) found[rule]!.push(bite(sets[rule], seed, ticks));
  }
  writeFileSync(resolve(out, `${name}-teeth.json`), JSON.stringify({ ticks, seeds: teeth, firstOverlapTick: found }, null, 2));
  process.stdout.write(JSON.stringify(found) + "\n");
} else {
  const started = performance.now();
  const without = label("without", "");
  const full = sweep(without === "" ? RULES : { ...RULES, [without]: false }, seeds, ticks, option("trace", 0));
  const seconds = (performance.now() - started) / 1000;
  const ablations: Record<string, Sum> = {};
  if (ablationSeeds > 0) {
    ablations["all rules"] = sweep(RULES, ablationSeeds, ablationTicks, 0);
    for (const [title, rules] of Object.entries(sets)) ablations[title] = sweep(rules, ablationSeeds, ablationTicks, 0);
  }
  const columns = ["ticks", "actors", "overlaps", "disorders", "brushes", "nears", "poofs", "timeouts", "evictions", "crowded", "seated", "spilled", "waits", "stalls", "delays", "falls", "heads", "steered", "chutes", "hops", "glides", "lanes", "refusals", "surveys", "grabs"];
  /** 📋️ One row of a table. */
  const row = (title: string, sum: Sum): string => `| ${title} | ${columns.map((column) => String(sum[column] ?? 0)).join(" | ")} | ${(((sum.poofs ?? 0) * 1e6) / Math.max(sum.actors ?? 1, 1)).toFixed(1)} | ${((sum.waits ?? 0) / Math.max(sum.actors ?? 1, 1)).toFixed(4)} |`;
  const head = `| run | ${columns.join(" | ")} | poofs per million actor-ticks | waits per actor-tick |\n|${"---|".repeat(columns.length + 3)}`;
  const text = [`# Reference world of 🚧️clearance — ${name}`, "", `${without === "" ? "Full rule set" : `Without ${without}`}: ${seeds} seeds × ${ticks} ticks, slices of ${span} tick(s) (${seconds.toFixed(1)} s).`, "", head, row(without === "" ? "all rules" : `without ${without}`, full), ""];
  if (ablationSeeds > 0) text.push(`Ablations: ${ablationSeeds} seeds × ${ablationTicks} ticks each.`, "", head, ...Object.entries(ablations).map(([title, sum]) => row(title, sum)), "");
  writeFileSync(resolve(out, `${name}.md`), text.join("\n"));
  writeFileSync(resolve(out, `${name}.json`), JSON.stringify({ seeds, ticks, span, seconds, full, ablationSeeds, ablationTicks, ablations }, null, 2));
  process.stdout.write(text.join("\n") + "\n");
}
