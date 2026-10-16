import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { runResolutionOracle } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";

const [sandbox, out, ...flags] = process.argv.slice(2);
const run = runResolutionOracle(sandbox!, join(sandbox!, "__combined.rs"), "combined", flags);
writeFileSync(out!, run.diagnostics.map((line, index) => `${line}\t@${run.locations[index] ?? ""}`).join("\n") + "\n");
console.log("diagnostics", run.diagnostics.length);
