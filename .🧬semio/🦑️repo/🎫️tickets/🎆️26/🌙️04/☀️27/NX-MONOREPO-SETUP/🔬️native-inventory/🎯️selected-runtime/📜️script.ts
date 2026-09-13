import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!, output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
const { testSelectedFontDependencies, testSelectedPackageIdentities } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎯️selected-runtime/🟦️.ts"));
testSelectedPackageIdentities(root);
const { runTool } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts"));
if (process.argv.includes("--refresh-lock")) console.log(await runTool("cargo", ["tree", "--offline", "-p", "semio-framework-os-font-assets"], root, AbortSignal.timeout(60_000), true));
await testSelectedFontDependencies(root);
const rows = [];
for (const name of ["semio-s-artifact-puzzle-3d", "semio-s-plugin-puzzle"]) {
  const result = await runTool("cargo", ["tree", "--locked", "--offline", "-p", name, "--target", "wasm32-wasip2", "--edges", "normal,build", "--prefix", "none", "--format", "{p}"], root, AbortSignal.timeout(60_000), true);
  const crates = [...new Set(result.trim().split("\n").map(line => line.split(" ")[0]))].sort();
  rows.push({ name, crates, flow: crates.filter(name => name.includes("flow")), wgpu: crates.filter(name => /wgpu|vello/.test(name)) });
}
writeFileSync(join(output, "selected-cargo-dependencies.json"), JSON.stringify(rows, null, 2));
console.log(JSON.stringify(rows.map(({ name, crates, flow, wgpu }) => ({ name, crateCount: crates.length, flow, wgpu }))));
