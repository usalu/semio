import { readFileSync } from "node:fs";
import { join } from "node:path";
const workspace = process.cwd();
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const observed = new Set<string>();
for (const entry of profile.entries) {
  const result = await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "oracle", setup(build) { build.onLoad({ filter: /.*/ }, (args) => { observed.add(args.path.slice(workspace.length + 1).replaceAll("\\", "/")); return undefined; }); } }] });
  if (!result.success) console.log("BUILD FAILED", entry.sourceRelativePath, result.logs.map(String).slice(0, 3).join("\n"));
}
const expected = new Set<string>(profile.sourceModulePaths);
const missing = [...observed].filter((p) => !expected.has(p)).sort(), unused = [...expected].filter((p) => !observed.has(p)).sort();
const strip = (s: string) => s.replace(/[^\x00-\x7f]/gu, "");
console.log("observed", observed.size, "expected", expected.size);
console.log("MISSING (in bundle, not in list):\n" + missing.map((p) => "  " + p).join("\n"));
console.log("UNUSED (in list, not in bundle):\n" + unused.map((p) => "  " + p).join("\n"));
