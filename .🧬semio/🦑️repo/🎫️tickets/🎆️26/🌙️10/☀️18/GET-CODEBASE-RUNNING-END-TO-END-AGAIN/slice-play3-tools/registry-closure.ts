import { readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { registryCompilerInputDependencies } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../../../🧰️framework/🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";
const repoRoot = process.cwd(), taxonomy = JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json","utf8"));
const authority = taxonomy.generatorContracts["plugin-registry"]!.inputDiscovery as any;
const visited = new Set<string>(), offenders = new Map<string, string[]>();
const visit = (path: string, role: any, via: string): void => {
  if (visited.has(path)) return;
  visited.add(path);
  let source: string;
  try { source = readFileSync(resolve(repoRoot, path), "utf8"); } catch { offenders.set("<missing>", [...(offenders.get("<missing>") ?? []), path + " via " + via]); return; }
  const dependencies = registryCompilerInputDependencies(source, path, role);
  for (const specifier of dependencies.imports) {
    if (specifier.startsWith("node:") || isRuntimeBuiltinSpecifierV1(specifier)) continue;
    if (specifier.startsWith(".")) { visit(relative(repoRoot, resolve(repoRoot, dirname(path), specifier)).replaceAll("\\", "/"), "static-import", path); continue; }
    const binding = authority.workspaceImports[specifier];
    if (!binding) { offenders.set(specifier, [...(offenders.get(specifier) ?? []), path]); continue; }
    visit(binding.entryPath, "implementation-entry", path);
  }
};
for (const path of authority.implementationEntryPaths) visit(path, "implementation-entry", "<entry>");
if (process.argv[2] !== "--json") console.log("visited", visited.size);
if (process.argv[2] === "--json") console.log(JSON.stringify([...offenders].map(([specifier, paths]) => ({ specifier, paths })))); else for (const [specifier, paths] of offenders) console.log(specifier, "<-", paths.slice(0, 3).map(p => p.slice(-90)).join(" | "), paths.length);
