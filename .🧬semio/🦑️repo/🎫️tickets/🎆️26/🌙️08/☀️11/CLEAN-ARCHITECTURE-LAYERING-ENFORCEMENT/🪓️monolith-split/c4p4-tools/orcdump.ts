import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { runResolutionOracle } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
const [root, out] = process.argv.slice(2) as [string, string];
const dump: Record<string, unknown> = {};
for (const flags of [[], ["--test"]] as string[][]) {
  const r = runResolutionOracle(root, join(root, "__combined.rs"), "combined", flags);
  dump[flags.length ? "test" : "lib"] = { exit: r.exit, diagnostics: r.diagnostics, locations: r.locations };
}
writeFileSync(out, JSON.stringify(dump));
console.log("ORCDUMP done");
