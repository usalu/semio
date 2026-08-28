// 🧪️ DEMO 3 — MatrixScript builds its `inventories` array as
//   registry.mutationManifests.map(readRuntimeInventory).filter(non-null)
// so a manifest whose owner never ran `test inventory` contributes NOTHING — not even a "missing"
// entry — to measureCoverage's runtimeMutationCoverage. As long as ONE OTHER manifest has been
// inventoried, the denominator is non-zero and the never-inventoried subset simply vanishes.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const inventoried: T.MutationManifest = {
  schema: "semio.repository-test.mutation-manifest/v2",
  artifact: "demo.inventoried",
  standard: "v1",
  subset: "shell",
  mutations: [{ id: "set-name", capability: "demo.rename", outcomes: ["applied"], productionDispatch: { operation: "set-name", bridgeVersion: 1 }, oracleRequirements: [{ capability: "demo.rename", qualifyingKind: "third-party-library" }] }],
};

// Same shape, same oracle, same everything — the ONLY difference is that its owner never ran
// `bun ./📜️script.ts inventory --artifact demo.never-inventoried ...`, so no runtime inventory file
// exists for it on disk. `readRuntimeInventory` returns null for it, and MatrixScript's own
// `.filter((inventory) => inventory !== null)` line silently drops it from the array it hands to
// measureCoverage.
const neverInventoried: T.MutationManifest = {
  schema: "semio.repository-test.mutation-manifest/v2",
  artifact: "demo.never-inventoried",
  standard: "v1",
  subset: "shell",
  mutations: [{ id: "delete-everything", capability: "demo.destroy", outcomes: ["applied"], productionDispatch: { operation: "delete-everything", bridgeVersion: 1 }, oracleRequirements: [{ capability: "demo.destroy", qualifyingKind: "third-party-library" }] }],
};

const oracle: T.OracleEntry = { id: "demo-oracle", kind: "third-party-library", ecosystem: "javascript", package: "demo-lib", capabilities: ["demo.rename", "demo.destroy"], comparisonProfiles: ["ordered-json-v1"], license: "MIT", testOnly: true };

const registry: T.OracleRegistry = {
  schemaVersion: 2,
  oracles: [oracle],
  probes: [],
  noOracleDecisions: [],
  comparisonProfiles: T.CORE_COMPARISON_PROFILES,
  comparisonPipelines: [],
  toleranceProfiles: [],
  oracleHostPackages: [],
  mutationCatalogs: [],
  mutationManifests: [inventoried, neverInventoried],
  fixtureManifests: [],
  contributions: [],
};

// Exactly what MatrixScript does (script.ts MatrixScript.run): map every manifest through
// readRuntimeInventory, then filter(non-null). `neverInventoried` has no file on disk anywhere, so
// this simulates it directly with an in-memory equivalent: only `inventoried`'s runtime file "exists".
const onlyOneRuntimeInventoryExists: T.RuntimeMutationInventory[] = [
  { schema: "semio.repository-test.runtime-inventory/v2", artifact: "demo.inventoried", standard: "v1", subset: "shell", bridgeVersion: 1, mutations: [{ id: "set-name", variant: "", outcomes: ["applied"] }] },
  // (demo.never-inventoried has NO entry here — exactly what `.filter(non-null)` produces when its
  // 🏭️inventory/*.json cache file was never written because `test inventory` was never run for it)
];

const measurements = T.measureCoverage(registry, [], [], onlyOneRuntimeInventoryExists);
const runtimeCoverage = measurements.find((m) => m.dimension === "runtimeMutationCoverage")!;
console.log(`runtimeMutationCoverage: ${(runtimeCoverage.ratio * 100).toFixed(1)}% (${runtimeCoverage.covered}/${runtimeCoverage.total}), missing=[${runtimeCoverage.missing.join(", ")}]`);
console.log(`does "delete-everything" or "demo.never-inventoried" appear ANYWHERE in the measurement? ${JSON.stringify(runtimeCoverage).includes("never-inventoried") || JSON.stringify(runtimeCoverage).includes("delete-everything")}`);

const gate = T.enforceReleaseGates(measurements, { deferredMutations: 0, skipped: 0, wildcardOwners: 0, unregisteredRuntimeMutations: 0 });
const runtimeGateFailure = gate.find((f) => f.includes("runtimeMutationCoverage"));
console.log(`runtimeMutationCoverage gate failure: ${runtimeGateFailure ?? "(none — gate is satisfied)"}`);

// Compare with what `test contract`'s mutationInventoryBreaches (compareInventories) would say about
// the SAME never-inventoried manifest, in isolation:
const equality = T.compareInventories(neverInventoried, null, []);
console.log(`\nFor comparison — what \`test contract\` sees for the SAME manifest: compareInventories(...).runtimeMissing = ${equality.runtimeMissing}`);

console.log(
  runtimeCoverage.ratio === 1 && runtimeGateFailure === undefined
    ? "\n>>> VACUOUS PASS CONFIRMED: `test matrix --enforce`'s runtimeMutationCoverage is 100% and blocks nothing, even though an entire subset with a real, oracle-backed, dispatch-declared mutation was NEVER run through `test inventory` at all. `test contract` (a separate command, separate exit code) is the only place this is visible."
    : "\n>>> could not reproduce",
);
