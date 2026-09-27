#!/usr/bin/env bun
/**
 * 🧭️ R10 item 1: for every unresolved directory in the given scopes, prints its parent's resolved kind and the member
 * kinds already owned by that kind (the registries a new member name belongs in). Uses `instrumented/normalization.ts`
 * (a ticket-local copy of the live normalization module whose only change is `kindId` on directory entries).
 * Usage: bun taxonomy-kinds.ts [--taxonomy <candidate.json>] --json <out> <scope…>
 */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const { inventoryTaxonomy } = await import("./instrumented/normalization.ts");
const args = process.argv.slice(2);
const option = (name: string) => { const index = args.indexOf(name); if (index < 0) return undefined; const value = args[index + 1]; args.splice(index, 2); return value; };
const taxonomyPath = option("--taxonomy");
const jsonOut = option("--json")!;
const taxonomy = JSON.parse(readFileSync(taxonomyPath ?? `${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8"));
const memberKinds = Object.entries(taxonomy.semanticDirectoryMemberKinds as Record<string, { ownerKindIds: string[]; memberNames: string[] }>);
const rows: { path: string; name: string; code: string; parent: string; parentKind: string | null; ancestors: (string | null)[]; ownedMemberKinds: string[] }[] = [];
for (const scope of args) {
  const inventory = inventoryTaxonomy({ repoRoot: ROOT, scope, ...(taxonomyPath ? { taxonomyPath } : {}) });
  const kinds = new Map<string, string | null>();
  for (const entry of inventory.entries) if (entry.nodeKind === "directory") kinds.set(entry.sourcePath, (entry as { kindId?: string | null }).kindId ?? null);
  const resolveKind = (path: string): string | null => kinds.get(path) ?? null;
  for (const entry of inventory.entries) {
    for (const violation of entry.violations) {
      if (violation.code !== "directory-kind-unresolved" && violation.code !== "directory-kind-ambiguous") continue;
      const ancestors: (string | null)[] = [];
      for (let parent = dirname(entry.sourcePath); parent && parent !== "."; parent = dirname(parent)) ancestors.push(resolveKind(parent));
      const parentKind = ancestors[0] ?? null;
      rows.push({ path: entry.sourcePath, name: entry.sourcePath.split("/").at(-1)!, code: violation.code, parent: dirname(entry.sourcePath), parentKind, ancestors, ownedMemberKinds: parentKind ? memberKinds.filter(([, spec]) => spec.ownerKindIds.includes(parentKind)).map(([id]) => id) : [] });
    }
  }
  console.log(`[taxonomy-kinds] scope=${scope} unresolved=${rows.filter((row) => row.path.startsWith(scope)).length}`);
}
for (const row of rows) console.log(`${row.code === "directory-kind-ambiguous" ? "AMBIG " : ""}${row.path}\n    parent=${row.parentKind} owned=[${row.ownedMemberKinds.join(",")}] ancestors=${row.ancestors.slice(1, 4).join(" < ")}`);
writeFileSync(jsonOut, JSON.stringify(rows, null, 1));
