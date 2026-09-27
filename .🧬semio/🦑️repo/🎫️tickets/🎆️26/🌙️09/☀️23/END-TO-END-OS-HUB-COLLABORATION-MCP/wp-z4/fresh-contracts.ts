/** 🧊️ Z4: runs the repository's own generator-contract validator (the gate every `loadTaxonomy()` caller hits) against a
 * fresh-clone view, and lists every contract output a clone cannot have (tracked-declared but git-ignored/untracked,
 * external + ignored). usage: bun fresh-contracts.ts <repo root> <fresh view root> */
import { existsSync } from "node:fs";
import { join } from "node:path";

const [repo, view] = process.argv.slice(2);
const discovery = await import(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
const t0 = performance.now();
const problems: string[] = discovery.validateGeneratorContractsAgainstWorkspace(view);
console.log(`validator(view) ${problems.length} problem(s) in ${Math.round(performance.now() - t0)} ms`);
for (const p of problems) console.log(`  - ${p}`);
const real: string[] = discovery.validateGeneratorContractsAgainstWorkspace(repo);
console.log(`validator(repo) ${real.length} problem(s)`);
const taxonomy = JSON.parse(await Bun.file(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")).text());
const under = (path: string) => existsSync(join(view, path));
for (const [id, contract] of Object.entries<any>(taxonomy.generatorContracts ?? {})) {
  for (const output of contract.outputRoots ?? []) {
    const inView = existsSync(join(view, output.path));
    if (output.inclusion === "tracked" && !under(output.path)) console.log(`TRACKED-NOT-IN-CLONE ${id} ${output.path} view=${inView}`);
    if (contract.ownership === "external" && output.inclusion === "ignored") console.log(`EXTERNAL-IGNORED ${id} ${output.path} view=${inView}`);
  }
  for (const pattern of contract.inputPatterns ?? []) {
    if (/[*?]/u.test(pattern)) continue;
    if (!existsSync(join(view, pattern))) console.log(`INPUT-NOT-IN-CLONE ${id} ${pattern}`);
  }
}
