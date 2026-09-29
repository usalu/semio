#!/usr/bin/env bun
/**
 * 🩻️ R10 window-3: the registry `check` gate's findings WITHOUT its early exit on a stale generated catalog — read-only, so a
 * train can prove its own set while a later train's describes (which only `plugin-registry:generate` after their wasm builds
 * refreshes) keep the committed catalog stale. Prints the stale files, then every later gate: generator contracts, playground
 * registry + sessions, plugin taxonomy tree, package discovery, descriptor gate. Exit 1 on any error-class finding.
 * Usage: bun registry-check-probe.ts
 * @see ../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts (CheckScript)
 */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const REGISTRY = `${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`;
const { renderCatalogFiles, validatePlaygroundSessions } = await import(`${REGISTRY}/📽️projection/🟦️.ts`);
const { findNewContractPluginRoots, validatePlaygroundRegistry, validateTaxonomyTree } = await import(`${REGISTRY}/🗿️taxonomy-validation/🟦️.ts`);
const { validateDescriptors } = await import(`${REGISTRY}/🛂️descriptor-verification/🟦️.ts`);
const { PLUGIN_AREAS_STATE, TAXONOMY } = await import(`${REGISTRY}/🔎️discovery/🟦️.ts`);
const { discoverPackageProblems, validateGeneratorContractsAgainstWorkspace } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`);
const { declaredProjectTargets, generateLaunchJson, LAUNCH_OUTPUT_REL_PATH } = await import(`${REGISTRY}/🚀️launch/🟦️.ts`);

const errors: string[] = [];
errors.push(...(validateGeneratorContractsAgainstWorkspace(ROOT, TAXONOMY) as string[]).map((problem) => `generator contract: ${problem}`));
const { files, entries, playgrounds, frameworkPackages } = renderCatalogFiles(ROOT);
const outDir = join(REGISTRY, "🤖️generated");
const stale = Object.entries(files as Record<string, string>).filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content).map(([name]) => name);
const launchFresh = readFileSync(join(ROOT, LAUNCH_OUTPUT_REL_PATH), "utf8") === generateLaunchJson(ROOT, playgrounds, declaredProjectTargets(ROOT));
console.log(JSON.stringify({ plugins: entries.length, playgrounds: playgrounds.length, frameworkPackages: frameworkPackages.length, staleGenerated: stale, launchFresh }));
const hostPluginIds = new Set(entries.filter((entry: { host?: unknown }) => entry.host !== undefined).map((entry: { pluginId: string }) => entry.pluginId));
errors.push(...[...validatePlaygroundRegistry(playgrounds, ROOT, hostPluginIds), ...validatePlaygroundSessions(ROOT)].map((violation: string) => `playground: ${violation}`));
const taxonomyFindings = findNewContractPluginRoots(ROOT).flatMap(({ pluginId, pluginRoot }: { pluginId: string; pluginRoot: string }) => validateTaxonomyTree(pluginRoot, pluginId));
console.log(`plugin taxonomy tree findings: ${taxonomyFindings.length} (areas ${PLUGIN_AREAS_STATE})`);
if (PLUGIN_AREAS_STATE !== "exempt") errors.push(...taxonomyFindings.map((finding: string) => `taxonomy tree: ${finding}`));
console.log(`package discovery problems (warn): ${discoverPackageProblems(ROOT, TAXONOMY).length}`);
const descriptors = validateDescriptors(entries, ROOT);
console.log(`descriptor gate: ${descriptors.warnings.length} warnings, ${descriptors.errors.length} errors`);
errors.push(...descriptors.errors.map((error: string) => `descriptor: ${error}`));
if (!launchFresh) errors.push(`${LAUNCH_OUTPUT_REL_PATH} differs from the generator`);
for (const error of errors) console.log(`  - ${error}`);
console.log(errors.length ? `REGISTRY PROBE RED (${errors.length})` : "REGISTRY PROBE GREEN (apart from the stale generated catalog listed above)");
process.exit(errors.length ? 1 : 0);
