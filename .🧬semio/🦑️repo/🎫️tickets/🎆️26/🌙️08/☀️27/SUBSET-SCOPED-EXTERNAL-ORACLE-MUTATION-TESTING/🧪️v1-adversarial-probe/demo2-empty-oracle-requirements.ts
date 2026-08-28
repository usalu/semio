// 🧪️ DEMO 2 — buildCoverageMatrix / measureCoverage never call mutationManifestProblems before
// trusting registry.mutationManifests. A mutation whose oracleRequirements is the EMPTY ARRAY is
// CONTRACT-INVALID (mutationManifestProblems flags it), but `[].every(...)` is vacuously true, so
// measureCoverage's externalOracleCoverage counts it as fully oracle-backed — with ZERO oracles
// registered anywhere in the repository.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const brokenManifest: T.MutationManifest = {
  schema: "semio.repository-test.mutation-manifest/v2",
  artifact: "demo.broken",
  standard: "v1",
  subset: "shell",
  mutations: [
    {
      id: "delete-everything",
      capability: "demo.destroy",
      outcomes: ["applied"],
      productionDispatch: { operation: "delete-everything", bridgeVersion: 1 },
      oracleRequirements: [], // <-- contract-invalid: "must name at least one qualifying capability"
    },
  ],
};

console.log("mutationManifestProblems(brokenManifest):");
for (const p of T.mutationManifestProblems(brokenManifest)) console.log(`  - ${p}`);

const registry: T.OracleRegistry = {
  schemaVersion: 2,
  oracles: [], // <-- literally zero oracles registered in the whole repository
  probes: [],
  noOracleDecisions: [],
  comparisonProfiles: T.CORE_COMPARISON_PROFILES,
  comparisonPipelines: [],
  toleranceProfiles: [],
  oracleHostPackages: [],
  mutationCatalogs: [],
  mutationManifests: [brokenManifest], // buildCoverageMatrix/measureCoverage read this directly, unvalidated
  fixtureManifests: [],
  contributions: [],
};

const results: T.TestResult[] = [
  {
    testId: "demo::broken::mutate-delete-everything::rust::subject",
    owner: "demo",
    case: "broken",
    scenario: "mutate-delete-everything",
    implementation: "rust",
    role: "subject",
    level: "quick",
    status: "passed",
    durationMs: 1,
    mutation: "delete-everything",
    outcome: "applied",
    productionDispatch: { invoked: true, operation: "delete-everything", bridgeVersion: 1 },
    output: { rawHash: "x", projectionHash: "y" },
    diagnostics: [],
  },
];

const rows = T.buildCoverageMatrix("/dev/null", registry, results, "deadbeef");
const measurements = T.measureCoverage(registry, rows, results, []);
const externalOracle = measurements.find((m) => m.dimension === "externalOracleCoverage")!;
console.log(`\nexternalOracleCoverage: ${(externalOracle.ratio * 100).toFixed(1)}% (${externalOracle.covered}/${externalOracle.total}), missing=[${externalOracle.missing.join(", ")}]`);

// The oracle-specific contract check, called directly on the same (invalid) manifest/mutation:
const oracleBreaches = T.oracleRequirementBreaches(registry, "demo/🧪️oracle/🔣️.json", brokenManifest, brokenManifest.mutations[0]!);
console.log(`oracleRequirementBreaches on the same mutation directly: ${oracleBreaches.length} breach(es)`);

console.log(
  externalOracle.ratio === 1
    ? "\n>>> VACUOUS PASS CONFIRMED: externalOracleCoverage reads 100% for a mutation whose manifest is CONTRACT-INVALID and which names ZERO oracle requirements, with ZERO oracles registered anywhere. `[].every(...)` on an empty oracleRequirements array is vacuously true."
    : "\n>>> could not reproduce",
);
