// 🧮 Compares a traced read set with the declared inputs of the catalog generator (contract patterns plus registry discovery) and prints the rest.
import { readFileSync } from "node:fs";
import { join, relative } from "node:path";
const root = process.cwd(), library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const taxonomy = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8")), contract = taxonomy.generatorContracts["plugin-registry"];
const { loadCatalogTaxonomy, registryCatalogInputView } = await import(join(library, "🔍️discovery/🔐️taxonomy-authority/🟦️.ts"));
const { registryCatalogInputPaths } = await import(join(library, "🔍️discovery/🧲️registry-catalog-projection/🟦️.ts"));
const loaded = loadCatalogTaxonomy(), view = registryCatalogInputView(root, loaded);
const declared = new Set<string>([...contract.inputPatterns, ...registryCatalogInputPaths(root, loaded, view)]);
const outputs = contract.outputRoots.map((row: { path: string }) => row.path);
const ignored = /(^|\/)(node_modules|\.git|\.cache|\.🧬semio)(\/|$)|^\/(private|var|tmp|Users\/[^/]+\/(\.bun|Library|\.cache))/u;
const reads = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map(path => relative(root, path)).filter(path => !path.startsWith("..") && !ignored.test(path));
const files = new Set(reads), isDeclared = (path: string) => declared.has(path) || outputs.some((output: string) => path === output || path.startsWith(output + "/"));
const undeclared = reads.filter(path => !isDeclared(path) && ![...files].some(other => other.startsWith(path + "/") && isDeclared(other)));
console.log(`reads in repo: ${reads.length}; declared set: ${declared.size}; undeclared: ${undeclared.length}`);
for (const path of undeclared.slice(0, 400)) console.log(path);
process.exitCode = undeclared.length ? 1 : 0;
