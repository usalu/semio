// 🔬 Walks the plugin-registry catalog static-import closure like registryCatalogInputPaths, collecting every workspace specifier that is not schema-bound (resolving them through node_modules/@semio-tech to keep walking). usage: bun registry-unbound.ts
import {existsSync, readFileSync, realpathSync} from "node:fs";
import {dirname, relative, resolve, join} from "node:path";
import {loadTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import {registryCompilerInputDependencies} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const taxonomy = loadTaxonomy();
const authority = (taxonomy.generatorContracts as any)["plugin-registry"].inputDiscovery;
const seen = new Set<string>();
const unbound = new Map<string, string[]>();
const resolveBare = (spec: string): string | null => {
  const m = /^(@[^/]+\/[^/]+)(\/.*)?$/.exec(spec);
  if (!m) return null;
  const dir = join(root, "node_modules", m[1]!);
  if (!existsSync(dir)) return null;
  const real = realpathSync(dir);
  const manifest = JSON.parse(readFileSync(join(real, "package.json"), "utf8"));
  const target = manifest.exports?.["." + (m[2] ?? "")];
  return typeof target === "string" ? relative(root, resolve(real, target)) : null;
};
const visit = (path: string, role: any, via: string): void => {
  if (seen.has(path)) return;
  seen.add(path);
  if (!existsSync(join(root, path))) { console.log("MISSING", path, "via", via); return; }
  const deps = registryCompilerInputDependencies(readFileSync(join(root, path), "utf8"), path, role);
  for (const spec of deps.imports) {
    if (spec.startsWith("node:") || spec.startsWith("bun:")) continue;
    if (spec.startsWith(".")) { visit(relative(root, resolve(root, dirname(path), spec)), "static-import", path); continue; }
    if (authority.workspaceImports[spec]) { visit(authority.workspaceImports[spec].entryPath, "implementation-entry", path); continue; }
    unbound.set(spec, [...(unbound.get(spec) ?? []), path]);
    const target = resolveBare(spec);
    if (target) visit(target, "implementation-entry", path);
    else console.log("UNRESOLVED", spec, "in", path);
  }
};
for (const p of authority.implementationEntryPaths) visit(p, "implementation-entry", "<entry>");
console.log("visited", seen.size);
for (const [spec, importers] of unbound) console.log(spec, "<-", importers.length, importers[0]);
