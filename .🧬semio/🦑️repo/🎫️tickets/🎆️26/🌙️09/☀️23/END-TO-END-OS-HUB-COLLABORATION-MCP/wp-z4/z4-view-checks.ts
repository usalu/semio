#!/usr/bin/env bun
/** 🧊️ Z4: runs the window-3 set's own gates against a fresh-clone view with both prepared sets applied — discovery's
 * `validateTaxonomy` + `validateGeneratorContractsAgainstWorkspace`, and the container laws (lifecycle, runtime-bootstrap,
 * devcontainer-context) imported from the VIEW. usage: bun z4-view-checks.ts <view root> <scratch output dir> */
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const [view, output] = process.argv.slice(2) as [string, string];
mkdirSync(output, { recursive: true });
const library = join(view, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const discovery = await import(join(library, "🔍️discovery/🟦️.ts"));
const taxonomyProblems: string[] = discovery.validateTaxonomy(JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8")));
console.log(`validateTaxonomy(view) ${taxonomyProblems.length}`, taxonomyProblems);
const contractProblems: string[] = discovery.validateGeneratorContractsAgainstWorkspace(view);
console.log(`validateGeneratorContractsAgainstWorkspace(view) ${contractProblems.length}`, contractProblems);
const containers = join(library, "⚡️caching/📦️artifacts/🐳️containers/🧪️tests");
const { testDevcontainerLifecycle } = await import(join(containers, "🔁️lifecycle/🟦️.ts"));
await testDevcontainerLifecycle(view, output);
const { testContainerRuntimeBootstrap } = await import(join(containers, "🚀️runtime-bootstrap/🟦️.ts"));
testContainerRuntimeBootstrap(view);
const { testDevcontainerContext } = await import(join(containers, "🐳️devcontainer-context/🟦️.ts"));
testDevcontainerContext(view);
process.exit(taxonomyProblems.length + contractProblems.length ? 1 : 0);
