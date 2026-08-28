// 🧪️ DEMO 5 — two independent "the flag is trusted, not verified" gaps.
//
// (a) `productionDispatch.invoked` is a bare boolean set by whatever code populates a TestResult.
//     Nothing in index.ts/script.ts cross-checks it against an actual dispatch trace, a manifest
//     mutation id that exists, or anything at all. vectorReplayBreaches and the productionBridgeCoverage
//     dimension both just read the flag.
//
// (b) `isQualifiedProbe` only checks `qualification.status === "qualified"`; it never checks that
//     `criteria` exist or are all `met: true`, nor that `evidence` says anything real.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

// (a) ------------------------------------------------------------------------------------------
const fabricated: T.TestResult = {
  testId: "demo::x::mutate-anything::rust::subject",
  owner: "demo",
  case: "x",
  scenario: "mutate-anything",
  implementation: "rust",
  role: "subject",
  level: "quick",
  status: "passed",
  durationMs: 1,
  mutation: "a-mutation-id-that-appears-in-no-manifest-anywhere",
  outcome: "applied",
  // Nothing ever called production code for this result. The adapter just wrote this literal.
  productionDispatch: { invoked: true, operation: "totally-made-up-operation", bridgeVersion: 999999 },
  output: { rawHash: "x", projectionHash: "y" },
  diagnostics: [],
};
const replayBreaches = T.vectorReplayBreaches([fabricated]);
console.log(`(a) vectorReplayBreaches on a fully fabricated dispatch claim: ${replayBreaches.length} breach(es)`);

const registry: T.OracleRegistry = { schemaVersion: 2, oracles: [], probes: [], noOracleDecisions: [], comparisonProfiles: [], comparisonPipelines: [], toleranceProfiles: [], oracleHostPackages: [], mutationCatalogs: [], mutationManifests: [], fixtureManifests: [], contributions: [] };
const measurements = T.measureCoverage(registry, [], [fabricated], []);
const bridgeCoverage = measurements.find((m) => m.dimension === "productionBridgeCoverage")!;
console.log(`    productionBridgeCoverage counts it as dispatched: ${(bridgeCoverage.ratio * 100).toFixed(0)}% (${bridgeCoverage.covered}/${bridgeCoverage.total})`);

// (b) ------------------------------------------------------------------------------------------
const selfCertified: T.ProbeEntry = {
  id: "demo-self-certified-probe",
  kind: "external-process",
  ecosystem: "python",
  package: "demo-probe-lib",
  capabilities: ["demo.measure"],
  outputSchema: "semio.repository-test.probe-report/v2",
  deterministic: true,
  license: "MIT",
  testOnly: true,
  // No criteria at all, evidence is a single meaningless character — nothing checks either.
  qualification: { status: "qualified", evidence: "x" },
};
console.log(`(b) isQualifiedProbe(selfCertified with 0 criteria, evidence="x"): ${T.isQualifiedProbe(selfCertified)}`);

console.log(
  replayBreaches.length === 0 && bridgeCoverage.ratio === 1 && T.isQualifiedProbe(selfCertified)
    ? "\n>>> CONFIRMED: (a) a subject result can claim production dispatch for an operation and mutation id that exist nowhere in any manifest, and both vectorReplayBreaches and productionBridgeCoverage accept it at face value — the flag is entirely self-reported and never cross-checked in this file. (b) a probe can self-certify `qualified` with zero criteria and a one-character 'evidence' string; isQualifiedProbe performs no sanity check on either."
    : "\n>>> could not reproduce",
);
