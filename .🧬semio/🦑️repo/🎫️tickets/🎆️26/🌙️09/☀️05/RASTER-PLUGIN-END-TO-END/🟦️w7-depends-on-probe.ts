#!/usr/bin/env bun
/** 🔗️ W7 probe: prints the declared runtime-dependency closure the dev runner and the browser boot
 * derive for a few representative plugin filters, so the registry's `dependsOn` semantics change can
 * be verified without booting a shell. Run: `bun ./🟦️w7-depends-on-probe.ts`. */
import { generatePluginRegistry, generatePlaygroundRegistry, resolveRegistryPluginIdsForFilter } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts";

const entries = generatePluginRegistry();
const playgrounds = generatePlaygroundRegistry();
for (const filter of ["raster", "demonstrator", "sequence", "draw", "lowpoly", "block", "writer", "norm"]) {
  const closure = [...resolveRegistryPluginIdsForFilter(filter, entries, playgrounds)].sort();
  console.log(`${filter}: dependsOn=${JSON.stringify(entries.find((entry) => entry.pluginId === filter)?.dependsOn ?? null)} sessionClosure=${JSON.stringify(closure)}`);
}
console.log(`rows with a declared runtime dependency: ${entries.filter((entry) => entry.dependsOn.length > 0).length}/${entries.length}`);
console.log(`rows still naming stdio: ${JSON.stringify(entries.filter((entry) => entry.dependsOn.includes("stdio")).map((entry) => entry.pluginId))}`);
