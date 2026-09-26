/** 🥒️ Prints the scenarios the platform's own parser expands for each feature path given (repo-relative), with the
 * in-repo Python source roots its owner chain contributes, as JSON.
 * `SEMIO_ROOT` points at a scratch copy of the repo modules to parse with a patched platform. */
import { readFileSync } from "node:fs";
const root = process.env.SEMIO_ROOT ?? "/Users/ueli/Documents/semio";
const { parseFeature, oracleHostPackagesFor, loadOracleRegistry, resolveFixtures } = await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`);
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
const base = process.env.SEMIO_FEATURES ?? "/Users/ueli/Documents/semio";
const URI = /\b(shared|local|asset|schema):\/\/([^\s"'`,;)\]]+)/g;
const fixturesOf = (owner: string, scenario: { steps: { text: string; docString?: string; dataTable?: string[][] }[] }) => resolveFixtures("/Users/ueli/Documents/semio", { owner } as never, [...new Set(scenario.steps.flatMap((step) => [step.text, step.docString ?? "", ...(step.dataTable ?? []).flat()]).flatMap((text) => [...text.matchAll(URI)].map((match) => match[0])))]).fixtures;
console.log(JSON.stringify(process.argv.slice(2).map((file) => {
  const owner = file.split("/🧪️tests/")[0]!;
  return { file, localSources: oracleHostPackagesFor(registry, owner, "python").flatMap((entry: { path?: string }) => (entry.path === undefined ? [] : [entry.path])), scenarios: parseFeature(readFileSync(`${base}/${file}`, "utf8")).scenarios.map((scenario: never) => ({ ...(scenario as object), fixtures: fixturesOf(owner, scenario) })) };
})));
