/** 🔬️ Times the two phases of one scale entry of the matrix on a frozen asset, short enough to run under the sampling profiler: `bun [--cpu-prof] phases.ts --snapshot <asset 🔣️.json> --scale <denominator> [--subjects <n>] [--every <k>]`.
 *
 * - Phase one: the print suitability of the representation at the scale (`resolveRepresentation`), which the first resolve of an entry pays.
 * - Phase two: `resolveViews` of every `k`-th subject (pose, clip instant, transition) of the entry, at most `n` of them, in the modes and languages the matrix renders, with the seconds per mode.
 *
 * Every line carries the `[DEBUG] ` prefix, the elapsed seconds and the resident set.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts — the engine whose subjects and options it uses
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { resolveViews } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { drawOptions, subjects } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = JSON.parse(readFileSync(resolve(flag("snapshot")!), "utf8")).asset;
const denominator = Number(flag("scale"));
const started = Date.now();
const line = (label: string): void => console.error(`[DEBUG] ${label} t ${((Date.now() - started) / 1000).toFixed(1)} s rss ${(process.memoryUsage().rss / 1048576).toFixed(0)} MB heapUsed ${(process.memoryUsage().heapUsed / 1048576).toFixed(0)} MB`);
line("start");
const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
line(`suitability ${resolution.kind}`);
const all = subjects(asset, denominator);
line(`subjects ${all.length}`);
const every = Number(flag("every") ?? 1);
const picked = all.filter((_, index) => index % every === 0).slice(0, Number(flag("subjects") ?? all.length));
const perMode: Record<string, number> = {};
let resolves = 0;
for (const subject of picked)
  for (const language of subject.languages)
    for (const mode of subject.modes) {
      const before = performance.now();
      resolveViews(asset, subject.query, { kind: "scale", denominator }, mode, drawOptions(mode, denominator, false, language));
      perMode[mode] = (perMode[mode] ?? 0) + (performance.now() - before) / 1000;
      resolves++;
    }
line(`resolved ${picked.length} subjects, ${resolves} resolves`);
console.error(`[DEBUG] seconds per mode ${JSON.stringify(Object.fromEntries(Object.entries(perMode).map(([mode, seconds]) => [mode, Number(seconds.toFixed(2))])))}`);
