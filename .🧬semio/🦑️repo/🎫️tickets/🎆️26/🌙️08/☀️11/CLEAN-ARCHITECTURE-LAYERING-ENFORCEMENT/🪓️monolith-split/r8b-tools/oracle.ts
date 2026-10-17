import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { runResolutionOracle } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
const [dir, root, name, out, ...flags] = process.argv.slice(2);
const r = runResolutionOracle(dir!, join(dir!, root!), name!, flags);
writeFileSync(out!, r.diagnostics.map((d, i) => `${d}\t${r.locations[i]}`).join("\n") + "\n");
console.log(r.diagnostics.length, "exit", r.exit);
