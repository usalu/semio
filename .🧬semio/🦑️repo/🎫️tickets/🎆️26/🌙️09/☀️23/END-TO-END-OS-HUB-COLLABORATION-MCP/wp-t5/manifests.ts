// 🧾️ Dumps every mutation manifest's coordinate and owner, so the bridge generator targets exactly what the gate reads.
import { loadOracleRegistry, readRuntimeInventory } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(repoRoot);
const rows = registry.contributions.flatMap((c) => c.mutationManifests.map((m) => ({ owner: c.owner, manifestPath: c.manifestPath, artifact: m.artifact, standard: m.standard, subset: m.subset, ids: m.mutations.map((x) => x.id), inventory: readRuntimeInventory(repoRoot, m) !== null })));
console.log(JSON.stringify(rows, null, 1));
