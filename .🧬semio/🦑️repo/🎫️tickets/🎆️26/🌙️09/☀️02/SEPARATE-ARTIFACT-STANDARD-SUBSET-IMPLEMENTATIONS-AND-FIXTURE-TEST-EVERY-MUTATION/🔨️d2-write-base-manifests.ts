// 🔨️ D2 one-off: after renaming ✳️any -> ✳️base for the 6 wildcard-owner artifacts B5/C3 diagnosed,
// write each renamed subset's v2 mutation manifest from its own leaf descriptors, bypassing the
// oracle-qualification gate exactly like B5's own 🔨️b5-write-manifests-from-leaves.ts did for its
// first 100 -- for this ticket's law #2 an invisible capability-without-manifest gap is strictly
// worse than a visible missing-external-oracle one. See $TICKET/📓️d2-final-residuals.md.
//
// Usage: bun 🔨️d2-write-base-manifests.ts [--dry]
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { leafDescriptorCoverage, loadOracleRegistry, manifestFromLeafDescriptors, type MutationManifest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = process.cwd();
const dry = process.argv.includes("--dry");

const TARGET_CAPABILITIES = new Set([
  "zip-2-0-mutate", "pptx-ecma-376-mutate", "ifc-2x3-base-mutate", "step-ap214-base-mutate", "xlsx-ecma-376-mutate",
]);

const registry = loadOracleRegistry(repoRoot);
let written = 0;
const skipped: string[] = [];

for (const contribution of registry.contributions) {
  for (const catalog of contribution.mutationCatalogs) {
    if (!TARGET_CAPABILITIES.has(catalog.capability)) continue;
    const coverage = leafDescriptorCoverage(repoRoot, contribution.owner);
    const manifest = manifestFromLeafDescriptors(repoRoot, contribution.owner, catalog.capability);
    if (coverage.leaves === 0 || coverage.missing.length > 0 || manifest === null) {
      skipped.push(`${catalog.capability} @ ${contribution.owner} — leaves=${coverage.leaves} missing=${coverage.missing.length} manifest=${manifest === null ? "null" : "ok"}`);
      continue;
    }
    const path = join(repoRoot, contribution.manifestPath);
    const parsed = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
    const all = (parsed.mutationManifests as MutationManifest[] | undefined) ?? [];
    const prior = all.find((entry) => entry.artifact === manifest.artifact && entry.standard === manifest.standard && entry.subset === manifest.subset);
    const existingOthers = all.filter((entry) => entry !== prior);
    const carried = new Map((prior?.mutations ?? []).map((mutation) => [mutation.id, mutation] as const));
    const merged = { ...manifest, mutations: manifest.mutations.map((mutation) => {
      const before = carried.get(mutation.id);
      if (before === undefined) return mutation;
      return {
        ...mutation,
        ...(before.oracleRequirements !== undefined ? { oracleRequirements: before.oracleRequirements } : {}),
        ...((before as { invariants?: unknown }).invariants !== undefined ? { invariants: (before as { invariants?: unknown }).invariants } : {}),
      };
    }) };
    if (!dry) {
      parsed.mutationManifests = [...existingOthers, merged];
      parsed.schemaVersion = 2;
      writeFileSync(path, `${JSON.stringify(parsed, null, 2)}\n`);
    }
    written += 1;
    console.log(`[d2] ${dry ? "would write" : "wrote"} ${manifest.mutations.length} mutation(s) for ${catalog.capability} into ${contribution.manifestPath}`);
  }
}
console.log(`\n[d2] written=${written} skipped=${skipped.length}`);
for (const s of skipped) console.log(`[d2] SKIP ${s}`);
