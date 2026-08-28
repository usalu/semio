// 🧪️ DEMO 7 — `ComparisonStage.optional` is documented as excusing ONLY a stage whose probe is not
// yet qualified ("A stage whose probe is not yet qualified. It RUNS and REPORTS; no release gate may
// claim its guarantee."). Nothing in evaluatePipeline / evaluateStageAssertions actually checks that
// invariant: `optional` is just a free boolean the pipeline author sets per stage, so a FULLY
// QUALIFIED probe's gating stage can be marked `optional: true` and its failure — even a hard
// `status: "failed"` from the probe itself — stops mattering to `equal` at all.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const qualifiedProbe: T.ProbeEntry = {
  id: "real-qualified-probe",
  kind: "external-process",
  ecosystem: "python",
  package: "real-lib",
  capabilities: ["geometry.compare"],
  outputSchema: "s",
  deterministic: true,
  license: "MIT",
  testOnly: true,
  qualification: { status: "qualified", evidence: "spike passed, criteria all met", criteria: [{ id: "a", met: true }] },
};

// The pipeline author marks the QUALIFIED probe's only stage `optional: true` — nothing rejects this.
const pipeline: T.ComparisonPipeline = {
  id: "demo-optional-abuse",
  stages: [{ probe: "real-qualified-probe", inputs: ["expected", "actual"], optional: true, assertions: { relativeVolumeErrorMax: 1e-9 } }],
};

// The probe genuinely FAILED — real geometric divergence, or even a hard crash.
const failedReport: T.ProbeReport = { schema: "semio.repository-test.probe-report/v2", probe: "real-qualified-probe", status: "failed", measurements: {} };

const verdict = T.evaluatePipeline(pipeline, new Map([[0, failedReport]]), new Map([["real-qualified-probe", qualifiedProbe]]));
console.log(`pipeline.equal despite the QUALIFIED probe reporting status="failed": ${verdict.equal}`);
console.log(`unqualifiedStages (should be empty — the probe IS qualified): [${verdict.unqualifiedStages.join(", ")}]`);
console.log(`the failing verdict itself: ${JSON.stringify(verdict.verdicts[0])}`);

console.log(
  verdict.equal && verdict.unqualifiedStages.length === 0
    ? "\n>>> VACUOUS PASS CONFIRMED: a stage using a genuinely QUALIFIED probe (not provisional, not missing) that reported a hard FAILURE still reads pipeline.equal=true, purely because the pipeline author set `optional: true` on it. Nothing in evaluatePipeline / evaluateStageAssertions / any validator checks that `optional` is only ever used on a stage whose probe is actually unqualified — the safety property described in the ComparisonStage.optional docstring is not code, it is a comment."
    : "\n>>> could not reproduce",
);
