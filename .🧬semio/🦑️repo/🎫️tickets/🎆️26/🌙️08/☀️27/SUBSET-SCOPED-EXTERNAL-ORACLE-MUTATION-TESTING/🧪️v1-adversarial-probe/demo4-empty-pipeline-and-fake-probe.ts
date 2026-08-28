// 🧪️ DEMO 4 — no validator exists anywhere in index.ts for ComparisonPipeline / ProbeEntry /
// ToleranceProfile records (confirmed: grep finds no *Problems function for any of the three, and
// readContribution casts parsed.comparisonPipelines/probes/toleranceProfiles with zero validation).
// Two consequences, demonstrated directly against evaluatePipeline:
//   (a) a pipeline declared with `stages: []` is vacuously `equal: true` — `[].every(...)`.
//   (b) a stage whose probe is NOT REGISTERED AT ALL still gates on a self-reported ProbeReport —
//       `missingProbes` is populated but never folds into `equal`.
import * as T from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

// (a) ---------------------------------------------------------------------
const emptyPipeline: T.ComparisonPipeline = { id: "demo-empty", stages: [] };
const emptyVerdict = T.evaluatePipeline(emptyPipeline, new Map(), new Map());
console.log(`(a) empty-stage pipeline: equal=${emptyVerdict.equal}, verdicts=${emptyVerdict.verdicts.length}`);

// (b) ---------------------------------------------------------------------
// A pipeline with one gating (non-optional) stage naming a probe id that is registered NOWHERE in
// the registry's probe table.
const pipelineWithGhostProbe: T.ComparisonPipeline = {
  id: "demo-ghost",
  stages: [{ probe: "ghost-probe-nobody-registered", inputs: ["a", "b"], assertions: { relativeVolumeErrorMax: 1e-6 } }],
};
// The adapter/host fabricates a report as if the (nonexistent) probe ran and measured a perfect 0 error.
const fabricatedReport: T.ProbeReport = { schema: "semio.repository-test.probe-report/v2", probe: "ghost-probe-nobody-registered", status: "ok", measurements: { relativeVolumeError: 0 } };
const ghostVerdict = T.evaluatePipeline(pipelineWithGhostProbe, new Map([[0, fabricatedReport]]), new Map() /* empty probe registry */);
console.log(`(b) ghost-probe pipeline: equal=${ghostVerdict.equal}, missingProbes=[${ghostVerdict.missingProbes.join(", ")}]`);

console.log(
  emptyVerdict.equal && ghostVerdict.equal
    ? "\n>>> VACUOUS PASS CONFIRMED (both): (a) an empty pipeline reads equal:true having measured nothing; (b) a gating stage whose probe is registered nowhere still reads equal:true off a self-reported report — evaluatePipeline never checks `missingProbes` before computing `equal`, and no ComparisonPipeline validator exists anywhere in this file to reject either shape at ingest."
    : "\n>>> could not reproduce",
);
