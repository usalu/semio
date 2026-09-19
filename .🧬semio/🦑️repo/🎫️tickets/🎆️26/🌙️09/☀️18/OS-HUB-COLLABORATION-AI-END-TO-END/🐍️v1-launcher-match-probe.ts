/** 🔎️ Counts react dev launchers per playground variant under three candidate predicates, to decide
 * which one is exact for all 65 variants (the `🧪️tests/🚀️launch` law currently mixes two). */

import { generatePlaygroundRegistry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";
import { generateLaunchJson } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts";

const repoRoot = process.argv[2]!;
const playgrounds = generatePlaygroundRegistry(repoRoot);
const launch = Bun.JSONC.parse(generateLaunchJson(repoRoot, playgrounds, [])) as {
  readonly configurations: readonly { name?: string; command?: string; env?: Readonly<Record<string, string>> }[];
};
const react = launch.configurations.filter((entry) => String(entry.name ?? "").endsWith("⚛️react"));
const tally = (label: string, match: (entry: (typeof react)[number], playground: (typeof playgrounds)[number]) => boolean): void => {
  const bad = playgrounds.map((playground) => ({ playground, count: react.filter((entry) => match(entry, playground)).length })).filter((row) => row.count !== 1);
  console.log(`${label}: ${bad.length} variant(s) not exactly 1 of ${playgrounds.length}`);
  for (const row of bad.slice(0, 12)) console.log(`  ${row.playground.variant} (${row.playground.pluginId}) = ${row.count}`);
};
console.log(`react launchers: ${react.length}, playgrounds: ${playgrounds.length}`);
tally("current law (env pluginId OR command contains -- variant)", (entry, playground) => entry.env?.SEMIO_PLUGIN === playground.pluginId || String(entry.command ?? "").includes(`-- ${playground.variant}`));
tally("command endsWith -- variant", (entry, playground) => String(entry.command ?? "").trimEnd().endsWith(`-- ${playground.variant}`));
tally("command endsWith -- variant AND env pluginId when present", (entry, playground) => String(entry.command ?? "").trimEnd().endsWith(`-- ${playground.variant}`) && (entry.env?.SEMIO_PLUGIN === undefined || entry.env.SEMIO_PLUGIN === playground.pluginId));
