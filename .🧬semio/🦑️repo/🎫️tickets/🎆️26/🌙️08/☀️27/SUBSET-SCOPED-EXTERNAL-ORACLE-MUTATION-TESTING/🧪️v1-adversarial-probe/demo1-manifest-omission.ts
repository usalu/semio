// 🧪️ DEMO 1 — an owner that never declares a MutationManifest for a capability is entirely invisible
// to buildCoverageMatrix / measureCoverage / enforceReleaseGates. The release gate reads 100% / 0
// failures while a real, dangerous, oracle-less mutation exists nowhere in the denominator.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const visibleManifest: T.MutationManifest = {
  schema: "semio.repository-test.mutation-manifest/v2",
  artifact: "demo.visible",
  standard: "v1",
  subset: "shell",
  mutations: [
    {
      id: "set-name",
      capability: "demo.rename",
      outcomes: ["applied"],
      productionDispatch: { operation: "set-name", bridgeVersion: 1 },
      oracleRequirements: [{ capability: "demo.rename", qualifyingKind: "third-party-library" }],
    },
  ],
};

const qualifyingOracle: T.OracleEntry = {
  id: "demo-oracle",
  kind: "third-party-library",
  ecosystem: "javascript",
  package: "demo-lib",
  capabilities: ["demo.rename"],
  comparisonProfiles: ["ordered-json-v1"],
  license: "MIT",
  testOnly: true,
};

// 🦠️ The HIDDEN owner: a capability with a REAL, dangerous, un-oracled mutation. It never registers a
// MutationManifest at all — only visibleManifest exists in registry.mutationManifests.
const registry: T.OracleRegistry = {
  schemaVersion: 2,
  oracles: [qualifyingOracle],
  probes: [],
  noOracleDecisions: [],
  comparisonProfiles: T.CORE_COMPARISON_PROFILES,
  comparisonPipelines: [],
  toleranceProfiles: [],
  oracleHostPackages: [],
  mutationCatalogs: [],
  mutationManifests: [visibleManifest], // <-- "hidden-owner.delete-everything" is nowhere here
  fixtureManifests: [],
  contributions: [],
};

const visibleFixture: T.FixtureManifest = {
  schema: "semio.repository-test.fixture/v2",
  id: "visible-set-name-applied",
  class: "handcrafted",
  target: { artifact: "demo.visible", standard: "v1", subset: "shell" },
  mutation: "set-name",
  outcome: "applied",
  units: { length: "mm", angle: "deg" },
  files: [{ role: "expected", path: "expected.json", mediaType: "application/json", sha256: `sha256:${"0".repeat(64)}` }],
  provenance: { source: "authored", license: "MIT" },
  comparisonProfile: "ordered-json-v1",
  reproducible: true,
};

registry.contributions.push({
  owner: "demo",
  manifestPath: "demo/🧪️oracle/🔣️.json",
  oracles: [],
  noOracleDecisions: [],
  comparisonProfiles: [],
  oracleHostPackages: [],
  mutationCatalogs: [],
  mutationManifests: [],
  fixtureManifests: [visibleFixture],
  probes: [],
  comparisonPipelines: [],
  toleranceProfiles: [],
  problems: [],
});

const visibleInventory: T.RuntimeMutationInventory = {
  schema: "semio.repository-test.runtime-inventory/v2",
  artifact: "demo.visible",
  standard: "v1",
  subset: "shell",
  bridgeVersion: 1,
  mutations: [{ id: "set-name", variant: "", outcomes: ["applied"] }],
};

const results: T.TestResult[] = [
  {
    testId: "demo::visible::mutate-set-name::rust::subject",
    owner: "demo",
    case: "visible",
    scenario: "mutate-set-name",
    implementation: "rust",
    role: "subject",
    level: "quick",
    status: "passed",
    durationMs: 1,
    mutation: "set-name",
    outcome: "applied",
    productionDispatch: { invoked: true, operation: "set-name", bridgeVersion: 1 },
    output: { rawHash: "x", projectionHash: "y" },
    diagnostics: [],
  },
];

const rows = T.buildCoverageMatrix("/dev/null", registry, results, "deadbeef");
console.log(`rows in coverage matrix: ${rows.length}`);
console.log(`rows mention "hidden-owner": ${rows.some((r) => r.artifact.includes("hidden"))}`);

const measurements = T.measureCoverage(registry, rows, results, [visibleInventory]);
const gate = T.enforceReleaseGates(measurements, { deferredMutations: 0, skipped: 0, wildcardOwners: 0, unregisteredRuntimeMutations: 0 });

for (const m of measurements) console.log(`${m.dimension.padEnd(28)} ${(m.ratio * 100).toFixed(1)}% (${m.covered}/${m.total})`);
console.log(`\nrelease gate failures: ${gate.length}`);
for (const f of gate) console.log(`  ${f}`);

console.log(
  gate.length === 0
    ? "\n>>> VACUOUS PASS CONFIRMED: enforceReleaseGates reports ZERO failures while a whole capability with an un-oracled, unmeasured, un-inventoried mutation exists in production and simply has no MutationManifest entry."
    : "\n>>> could not reproduce — gate failed as expected",
);
