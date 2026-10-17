import { writeFileSync } from "node:fs";
import { verifyAppliedLayout } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { runResolutionOracle } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
import { join } from "node:path";
const [repoRoot, artifactDir, sandbox, out] = process.argv.slice(2) as [string, string, string, string];
const report = await verifyAppliedLayout(repoRoot, artifactDir, sandbox, "HEAD", true);
const dump: Record<string, unknown> = { report };
for (const side of ["base", "combined"]) for (const flags of [[], ["--test"]] as string[][]) {
  const root = `${sandbox}-${side}`;
  const r = runResolutionOracle(root, join(root, "__combined.rs"), "combined", flags);
  dump[`${side}${flags.length ? "-test" : "-lib"}`] = { exit: r.exit, diagnostics: r.diagnostics, locations: r.locations };
}
writeFileSync(out, JSON.stringify(dump));
console.log("ABDUMP done");
