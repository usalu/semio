/** 🔒️ Shrinks stale baseline records to the live declaration scan: a package the scan finds test-oracle only is recorded test-oracle, with the scan's users. Never widens. */
import { readFileSync, writeFileSync } from "node:fs";
import { scanDeclaredDependencies, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio", path = `${root}/🔒️dependencies.json`;
const baseline = JSON.parse(readFileSync(path, "utf8"));
const scanned = scanDeclaredDependencies(root, loadOracleRegistry(root)) as any[];
const pythonOracleLinked = new Set(process.argv.slice(2));
let changed = 0;
for (const entry of baseline.entries) {
  const live = scanned.find((candidate) => candidate.ecosystem === entry.ecosystem && candidate.name === entry.name);
  if (live && live.productionReachable === false && JSON.stringify(live.kinds) === JSON.stringify(["test-oracle"]) && (entry.productionReachable !== false || JSON.stringify(entry.kinds) !== JSON.stringify(["test-oracle"]))) {
    console.log(`shrink ${entry.ecosystem}:${entry.name} ${JSON.stringify(entry.kinds)}/${entry.productionReachable} -> ["test-oracle"]/false`);
    entry.kinds = ["test-oracle"]; entry.productionReachable = false; entry.users = [...live.users].sort(); changed++;
  }
  if (entry.ecosystem === "python" && pythonOracleLinked.has(entry.name) && JSON.stringify(entry.kinds) !== JSON.stringify(["test-oracle"])) {
    console.log(`shrink python:${entry.name} ${JSON.stringify(entry.kinds)} -> ["test-oracle"] (linked only by oracles)`);
    entry.kinds = ["test-oracle"]; entry.productionReachable = false; changed++;
  }
}
writeFileSync(path, `${JSON.stringify(baseline, null, 2)}\n`);
console.log(`${changed} record(s) shrunk`);
