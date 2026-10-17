import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const workspace = "/Users/ueli/Documents/semio";
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const observed = new Set<string>();
for (const entry of profile.entries) {
  const result = await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "o", setup(build) { build.onLoad({ filter: /.*/ }, args => { observed.add(args.path.slice(workspace.length + 1).replaceAll("\\", "/")); return undefined; }); } }] });
  console.log(entry.id, result.success, result.logs.map(String).join("\n").slice(0, 300));
}
const obs = [...observed].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
const cur: string[] = profile.sourceModulePaths;
const set = new Set(cur), oset = new Set(obs);
console.log("observed", obs.length, "recorded", cur.length);
console.log("MISSING in taxonomy:", obs.filter((x) => !set.has(x)));
console.log("STALE in taxonomy:", cur.filter((x) => !oset.has(x)));
writeFileSync(process.argv[2], JSON.stringify(obs));
