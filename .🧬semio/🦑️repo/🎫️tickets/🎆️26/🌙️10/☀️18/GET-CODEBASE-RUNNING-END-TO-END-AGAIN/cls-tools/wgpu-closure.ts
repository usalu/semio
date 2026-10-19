// usage: bun wgpu-closure.ts : prints observed browser input closure vs taxonomy sourceModulePaths (same oracle as testBrowserInputClosure)
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const workspace = "/Users/ueli/Documents/semio";
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile, observed = new Set<string>();
const failures: string[] = [];
for (const entry of profile.entries) {
  const result = await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "browser-transport-inputs", setup(build) { build.onLoad({ filter: /.*/ }, args => { observed.add(relative(workspace, args.path).replaceAll("\\", "/")); return undefined; }); } }] });
  if (!result.success) failures.push(entry.id + ": " + result.logs.map(String).join("\n"));
}
const expected = new Set<string>(profile.sourceModulePaths);
const missing = [...observed].filter(path => !expected.has(path)).sort(), unused = [...expected].filter(path => !observed.has(path)).sort();
console.log(JSON.stringify({ failures, observed: observed.size, expected: expected.size, missing, unused }, null, 1));
writeFileSync(process.argv[2] ?? "/dev/null", JSON.stringify([...observed].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))), null, 1));
