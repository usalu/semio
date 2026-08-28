// 🧪️ DEMO 6 — computeRunKey folds baselineSha, manifest/fixture digests, oracleLockDigest (package@
// version or lockDigest), oracleEngineDigest, probeDigest (package@version or lockDigest),
// comparisonProfileDigest (profile+pipeline+tolerance config), subjectDigest, platform, seed, level.
//
// Two fields that DECIDE whether an oracle/probe result may satisfy a requirement — OracleEntry.kind
// (qualifying vs. supplemental vs. absent) and ProbeEntry.qualification.status (qualified vs.
// provisional vs. rejected) — are NEVER hashed into the key. Flip either one with the underlying
// package/version/engine held fixed, and the run key is byte-identical, so a cached "equal:true"
// parity result computed while an oracle was cross-semio-implementation-only (non-qualifying) or a
// probe was provisional would be silently reused after either is reclassified, with no re-execution.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const base = { baselineSha: "deadbeef", manifest: null, fixtures: [], probes: [], subjectDigest: "s", platform: "linux-x64" as const, level: "quick" as const, comparison: T.CORE_COMPARISON_PROFILES[0], pipeline: undefined, tolerance: undefined };

const oracleQualifying: T.OracleEntry = { id: "o", kind: "third-party-library", ecosystem: "javascript", package: "lib", version: "1.0.0", capabilities: ["c"], comparisonProfiles: [], license: "MIT", testOnly: true, engine: { family: "engineA", implementation: "x", version: "1" } };
const oracleDegradedToSemio: T.OracleEntry = { ...oracleQualifying, kind: "cross-semio-implementation" }; // same package/version/engine
const oracleUnclassified: T.OracleEntry = { ...oracleQualifying, kind: undefined };

const keyQualifying = T.computeRunKey({ ...base, oracle: oracleQualifying }).key;
const keyDegraded = T.computeRunKey({ ...base, oracle: oracleDegradedToSemio }).key;
const keyUnclassified = T.computeRunKey({ ...base, oracle: oracleUnclassified }).key;
console.log(`run key with oracle.kind="third-party-library":       ${keyQualifying}`);
console.log(`run key with oracle.kind="cross-semio-implementation": ${keyDegraded}`);
console.log(`run key with oracle.kind=undefined:                    ${keyUnclassified}`);
console.log(`all three identical: ${keyQualifying === keyDegraded && keyDegraded === keyUnclassified}`);

const probeQualified: T.ProbeEntry = { id: "p", kind: "external-process", ecosystem: "python", package: "probe-lib", version: "2.0.0", capabilities: ["c"], outputSchema: "s", deterministic: true, license: "MIT", testOnly: true, qualification: { status: "qualified", evidence: "spike passed" } };
const probeProvisional: T.ProbeEntry = { ...probeQualified, qualification: { status: "provisional", evidence: "spike not yet run" } };
const probeRejected: T.ProbeEntry = { ...probeQualified, qualification: { status: "rejected", evidence: "spike failed" } };

const keyProbeQualified = T.computeRunKey({ ...base, oracle: undefined, probes: [probeQualified] }).key;
const keyProbeProvisional = T.computeRunKey({ ...base, oracle: undefined, probes: [probeProvisional] }).key;
const keyProbeRejected = T.computeRunKey({ ...base, oracle: undefined, probes: [probeRejected] }).key;
console.log(`\nrun key with probe qualification="qualified":  ${keyProbeQualified}`);
console.log(`run key with probe qualification="provisional": ${keyProbeProvisional}`);
console.log(`run key with probe qualification="rejected":    ${keyProbeRejected}`);
console.log(`all three identical: ${keyProbeQualified === keyProbeProvisional && keyProbeProvisional === keyProbeRejected}`);

console.log(
  keyQualifying === keyDegraded && keyProbeQualified === keyProbeProvisional
    ? "\n>>> CONFIRMED: computeRunKey is blind to OracleEntry.kind and ProbeEntry.qualification.status. Reclassifying an oracle from a qualifying kind to cross-semio-implementation (or to no kind at all), or a probe from rejected/provisional to qualified, changes what the comparison is ALLOWED to prove without changing the run key at all — a cached result survives the reclassification untouched."
    : "\n>>> could not reproduce",
);
