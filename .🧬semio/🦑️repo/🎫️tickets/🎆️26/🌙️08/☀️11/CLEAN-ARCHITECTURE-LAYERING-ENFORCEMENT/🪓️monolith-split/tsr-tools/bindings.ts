import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const workspace = "/Users/ueli/Documents/semio";
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const specifiers = new Set<string>();
const importers = new Map<string, string>();
for (const entry of profile.entries) {
  const result = await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "spec", setup(build) { build.onResolve({ filter: /^[^./]/ }, (args) => { specifiers.add(args.path); importers.set(args.path, args.importer); return undefined; }); } }] });
  console.log(entry.id, result.success);
}
const packages = new Map<string, string>();
const manifests = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "--", "🧰️framework", ":(glob)**/package.json"], { cwd: workspace }).stdout.toString().split("\n").filter((path) => path.endsWith("/package.json"));
for (const path of manifests) { try { const name = JSON.parse(readFileSync(join(workspace, path), "utf8")).name; if (name) packages.set(name, path); } catch {} }
const out: Record<string, { manifestPath: string; entryPath: string }> = {};
for (const specifier of [...specifiers].sort()) {
  const segments = specifier.split("/");
  const name = specifier.startsWith("@") ? segments.slice(0, 2).join("/") : segments[0]!;
  const subpath = "." + specifier.slice(name.length);
  const manifestPath = packages.get(name);
  if (!manifestPath) { console.log("NO PACKAGE", specifier, "<=", importers.get(specifier)); continue; }
  const manifest = JSON.parse(readFileSync(join(workspace, manifestPath), "utf8"));
  const target = typeof manifest.exports === "string" ? (subpath === "." ? manifest.exports : undefined) : manifest.exports?.[subpath];
  if (typeof target !== "string") { console.log("NO EXPORT", specifier, JSON.stringify(manifest.exports)?.slice(0, 120)); continue; }
  out[specifier] = { manifestPath, entryPath: relative(workspace, resolve(workspace, dirname(manifestPath), target)).replaceAll("\\", "/") };
}
console.log("specifiers", specifiers.size, "bound", Object.keys(out).length);
writeFileSync(process.argv[2]!, JSON.stringify(out, null, 2));
