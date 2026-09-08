/**
 * 🧩️ Measures the exact effect of row 133 (skip declared git submodule paths in the schema scope walk)
 * without editing the library partition: `inventorySchemaScopes` takes its taxonomy as a parameter, and the
 * walk's opaque-prefix filter is fed from `pathExclusions`, so adding the `.gitmodules` paths there prunes
 * exactly the same subtree the proposed `walkRepositoryTree` patch prunes.
 * @see `📓️wp2f-root-script.md` §2 for the patch this probe stands in for
 */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd();
const discovery = (await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts")).href)) as {
  inventorySchemaScopes: (repoRoot: string, taxonomy: unknown) => { diagnostics: { code: string; path: string; detail: string }[]; placement: { code: string; path: string; detail: string }[]; modules: unknown[]; catalog: { scopes: Record<string, unknown> } };
  loadCatalogTaxonomy: () => Record<string, unknown>;
};

const gitmodules = join(root, ".gitmodules");
const submodulePaths = existsSync(gitmodules)
  ? [...readFileSync(gitmodules, "utf8").matchAll(/^[\t ]*path[\t ]*=[\t ]*(.+?)[\t ]*$/gmu)].map((match) => match[1]!.replaceAll("\\", "/").replace(/\/+$/u, "").normalize("NFC")).filter(Boolean)
  : [];
console.log(`[probe] .gitmodules declares ${submodulePaths.length} submodule path(s): ${JSON.stringify(submodulePaths)}`);

const taxonomy = discovery.loadCatalogTaxonomy();
const patched = {
  ...taxonomy,
  pathExclusions: {
    ...(taxonomy.pathExclusions as Record<string, unknown>),
    ...Object.fromEntries(submodulePaths.map((path, index) => [`git-submodule-${index}`, { path: `${path}/`, mode: "opaque", reason: "Declared git submodule: a foreign repository tracked as one gitlink" }])),
  },
};

/** ♻️ Peers edit this tree while the probe walks it, so a walk that loses a file mid-hash is retried, never reported. */
const measure = (label: string, used: unknown): Map<string, string[]> => {
  for (let attempt = 1; ; attempt += 1) {
    try {
      const inventory = discovery.inventorySchemaScopes(root, used);
      const findings = [...inventory.diagnostics, ...inventory.placement];
      const byKey = new Map<string, string[]>();
      for (const finding of findings) byKey.set(`${finding.code} ${finding.path} ${finding.detail}`, [finding.code, finding.path]);
      console.log(`[probe] ${label}: modules=${inventory.modules.length} scopes=${Object.keys(inventory.catalog.scopes).length} findings=${findings.length}`);
      return byKey;
    } catch (error) {
      if (attempt >= 5) throw error;
      console.log(`[probe] ${label}: attempt ${attempt} lost a concurrently edited file (${error instanceof Error ? error.message : String(error)}); retrying`);
    }
  }
};

const before = measure("baseline", taxonomy);
const after = measure("submodules-skipped", patched);
const removed = [...before].filter(([key]) => !after.has(key));
const added = [...after].filter(([key]) => !before.has(key));
console.log(`[probe] removed=${removed.length} added=${added.length}`);
const counts = new Map<string, number>();
for (const [, [code]] of removed) counts.set(code!, (counts.get(code!) ?? 0) + 1);
for (const [code, count] of [...counts].sort(([left], [right]) => left.localeCompare(right))) console.log(`[probe] removed ${code}=${count}`);
for (const [, [code, path]] of removed) console.log(`[probe] removed row ${code} ${path}`);
for (const [, [code, path]] of added) console.log(`[probe] added row ${code} ${path}`);
