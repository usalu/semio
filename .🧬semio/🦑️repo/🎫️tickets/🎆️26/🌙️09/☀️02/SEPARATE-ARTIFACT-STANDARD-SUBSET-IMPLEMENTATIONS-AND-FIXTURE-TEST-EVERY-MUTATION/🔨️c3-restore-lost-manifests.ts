// 🔨️ B5 one-off: write v2 mutation manifests for capabilities whose mutation leaves are FULLY
// described (every leaf under 🧬️schema/🧬️mutations/<mutation>/ carries a descriptor) but which the
// stock `manifest --write` command skips because no QUALIFYING third-party oracle is registered yet
// for that capability. Skipping is the right default for the general command — it protects normal
// runs from silently declaring a requirement nothing discharges — but for THIS ticket's law #2 the
// invisible state (`capability-without-manifest`) is strictly worse than the visible one
// (`missing-external-oracle`), so this script performs the identical merge-write the CLI performs for
// "ready" rows, minus the oracle-qualification gate. See $TICKET/📓️b5-capability-without-manifest.md.
//
// Usage: bun 🔨️b5-write-manifests-from-leaves.ts [--dry]
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { leafDescriptorCoverage, loadOracleRegistry, manifestFromLeafDescriptors, type MutationManifest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

// 🎯️ Must be run with cwd = repo root (`cd /Users/ueli/Documents/semio`).
const repoRoot = process.cwd();
const dry = process.argv.includes("--dry");

// 🎯️ Exactly the capabilities this shard owns from breach-capability-without-manifest.json AND whose
// leaf descriptors are complete (verified separately via `manifest --dry --json`, reason ===
// "described, but no QUALIFYING third-party oracle supplies this capability").
const TARGET_CAPABILITIES = new Set([
  "csv-rfc4180-mutate", "tsv-iana-mutate", "jpg-jfif-1-01-baseline-mutate", "wav-riff-pcm-mutate",
  "tiff-6-0-baseline-mutate",
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
    console.log(`[b5] ${dry ? "would write" : "wrote"} ${manifest.mutations.length} mutation(s) for ${catalog.capability} into ${contribution.manifestPath}`);
  }
}

console.log(`\n[b5] ${written}/${TARGET_CAPABILITIES.size} target capabilities written`);
if (skipped.length > 0) {
  console.log(`[b5] ${skipped.length} target capabilities could NOT be written:`);
  for (const line of skipped) console.log(`[b5]   ${line}`);
}
const unmatched = [...TARGET_CAPABILITIES].filter((cap) => !registry.contributions.some((c) => c.mutationCatalogs.some((k) => k.capability === cap)));
if (unmatched.length > 0) {
  console.log(`[b5] ${unmatched.length} target capabilities were never found in any catalog at all:`);
  for (const cap of unmatched) console.log(`[b5]   ${cap}`);
}
