import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const workspace = "/Users/ueli/Documents/semio";
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const observed = new Set<string>();
for (const entry of profile.entries) {
  const result = await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "native-wgpu-input-oracle", setup(build) { build.onLoad({ filter: /.*/ }, args => { observed.add(args.path.slice(workspace.length + 1).replaceAll("\\", "/")); return undefined; }); } }] });
  console.log(entry.id, result.success, result.logs.map(String).join("\n").slice(0, 400));
}
const sorted = [...observed].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
const listed: string[] = profile.sourceModulePaths;
const gen = (a: string[], b: string[]) => a.filter((x) => !b.includes(x));
console.log("observed", sorted.length, "listed", listed.length);
console.log("missing from taxonomy:", gen(sorted, listed).length);
for (const m of gen(sorted, listed)) console.log("  +", m);
console.log("stale in taxonomy:", gen(listed, sorted).length);
for (const m of gen(listed, sorted)) console.log("  -", m);
writeFileSync(process.argv[2] ?? "/dev/null", JSON.stringify(sorted, null, 0));
