import { readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
const workspace = process.cwd();
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const bare = new Map<string, Set<string>>(), modules = new Set<string>();
for (const entry of profile.entries) {
  await Bun.build({ root: workspace, entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "p", setup(b) {
    b.onResolve({ filter: /.*/ }, (r) => { if (!r.path.startsWith(".") && !r.path.startsWith("/")) { const s = bare.get(r.path) ?? new Set(); s.add(Bun.resolveSync(r.path, dirname(r.importer))); bare.set(r.path, s); } return undefined; });
    b.onLoad({ filter: /.*/ }, (a) => { modules.add(relative(workspace, a.path)); return undefined; });
  } }] });
}
const old = Object.keys(profile.workspaceImports);
console.log("bare", bare.size, "old", old.length);
console.log("new not old:", [...bare.keys()].filter(k => !old.includes(k)));
console.log("old not new:", old.filter(k => !bare.has(k)));
for (const [k, v] of bare) if (v.size > 1) console.log("MULTI", k, [...v]);
for (const [k, v] of bare) { const e = [...v][0]!; const rel = relative(workspace, e); const b = profile.workspaceImports[k]; if (b && b.entryPath !== rel) console.log("ENTRY DIFF", k, rel, b.entryPath); if (!modules.has(rel)) console.log("not loaded", k, rel); }
for (const k of ["@semio-tech/plugin-registry/deployment/routes.json","@semio-tech/plugin-registry/runtime","@semio-tech/ui-locale-rs/index","@semio-tech/framework-plugin/receipt","@semio-tech/framework-plugin/describe/component-receipt/schema","@semio-tech/assets/mesh"]) console.log(k, [...bare.get(k)!].map(p=>relative(workspace,p)));
