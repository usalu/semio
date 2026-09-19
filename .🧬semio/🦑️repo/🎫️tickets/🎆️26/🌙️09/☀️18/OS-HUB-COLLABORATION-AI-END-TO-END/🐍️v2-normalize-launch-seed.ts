/** 🧹️ Rewrites the `devLaunchers` table of `.vscode/🧩️launch.seed.jsonc` to the registry-owned shape:
 * only `namePrefix` (recomputed from the registry), `order`, `wgpuOrder`, `env` extras and `users`
 * survive — command, port, plugin/app/renderer env and `serverReadyAction` are now derived. Reads the
 * seed immediately before writing so a peer's concurrent skeleton edit is preserved verbatim. */

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { generatePlaygroundRegistry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";
import { playgroundLaunchNamePrefix } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🏷️name-prefix/🟦️.ts";

const MARKER = ',\n\n  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated\n  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.\n  "devLaunchers": ';
const DERIVED_ENV = new Set(["S_OS_PORT", "SEMIO_PLUGIN", "SEMIO_RENDERER", "SEMIO_APP"]);

const repoRoot = process.argv[2]!;
const seedPath = join(repoRoot, ".vscode/🧩️launch.seed.jsonc");
const playgrounds = generatePlaygroundRegistry(repoRoot);
if (playgrounds.length === 0) throw new Error("playground registry is empty");
const byVariant = new Map(playgrounds.map((entry) => [entry.variant, entry]));

const raw = readFileSync(seedPath, "utf8");
const at = raw.indexOf(MARKER);
if (at === -1) throw new Error("seed is missing the devLaunchers marker");
const head = raw.slice(0, at + MARKER.length);
const table = JSON.parse(raw.slice(at + MARKER.length, raw.length - "\n}\n".length)) as Record<string, Record<string, unknown>>;

const out: Record<string, unknown> = {};
let changed = 0;
for (const [variant, row] of Object.entries(table)) {
  const playground = byVariant.get(variant);
  if (!playground) throw new Error(`devLaunchers["${variant}"] has no playground registry entry`);
  const extras: Record<string, string> = {};
  for (const key of ["reactEnv", "wgpuEnv"] as const) {
    for (const [envKey, envValue] of Object.entries((row[key] ?? {}) as Record<string, string>)) {
      if (!DERIVED_ENV.has(envKey) && !envKey.endsWith("_PLAY_PORT") && !envKey.endsWith("_PORT")) extras[envKey] = envValue;
    }
  }
  const users = row.users as { namePrefixPattern: string; emailPattern: string; env: Record<string, string> } | undefined;
  const next: Record<string, unknown> = {
    namePrefix: playgroundLaunchNamePrefix(playground, repoRoot, playgrounds),
    order: row.order,
    ...(row.wgpuOrder !== undefined ? { wgpuOrder: row.wgpuOrder } : {}),
    ...(Object.keys(extras).length > 0 ? { env: extras } : {}),
    ...(users ? { users: { namePrefixPattern: users.namePrefixPattern, emailPattern: users.emailPattern, env: Object.fromEntries(Object.entries(users.env).filter(([key]) => !DERIVED_ENV.has(key))) } } : {}),
  };
  if (JSON.stringify(next) !== JSON.stringify(row)) changed++;
  out[variant] = next;
}

writeFileSync(seedPath, `${head}${JSON.stringify(out, null, 2).split("\n").map((line, index) => (index === 0 ? line : `  ${line}`)).join("\n")}\n}\n`);
console.log(`devLaunchers rewritten: ${Object.keys(out).length} row(s), ${changed} changed`);
