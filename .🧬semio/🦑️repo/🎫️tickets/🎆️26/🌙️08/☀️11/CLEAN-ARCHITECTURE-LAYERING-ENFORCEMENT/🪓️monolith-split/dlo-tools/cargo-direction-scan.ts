/** 🔬 Runs the authoritative cargoDependencyDirectionReport over the live authored inventory without cargo metadata. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { cargoDirectionInventory, cargoDependencyDirectionReport } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const policy = { areaLayers: taxonomy.areaLayers, ...taxonomy.cargoDependencyDirections };
const inventory = cargoDirectionInventory(root);
const report = cargoDependencyDirectionReport(inventory, inventory, policy);
const by: Record<string, number> = {};
for (const v of report.violations) by[v.rule] = (by[v.rule] ?? 0) + 1;
console.log(`packages=${report.packages} localDeps=${report.localDependencies} violations=${report.violations.length} problems=${report.problems.length}`);
console.log(JSON.stringify(by));
const pb: Record<string, number> = {};
for (const p of report.problems) pb[p.code] = (pb[p.code] ?? 0) + 1;
console.log(JSON.stringify(pb));
for (const v of report.violations) console.log(`V ${v.rule} | ${v.from} | ${v.to} | alias=${v.alias} kind=${v.kind} opt=${v.optional} plat=${v.platform ?? "all"}`);
if (process.argv.includes("--problems")) for (const p of report.problems) console.log(`P ${p.code} ${p.owner}`);
