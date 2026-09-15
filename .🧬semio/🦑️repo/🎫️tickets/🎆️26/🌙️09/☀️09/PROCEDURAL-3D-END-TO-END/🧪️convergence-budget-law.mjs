/** ⚖️ The law over RECORDED status sequences: a green product is never called red because the machine
 * was busy, and a genuinely stalled preview is still called red.
 *
 * Every sequence in `🧫️convergence-sequences.json` is a real per-poll recording taken by
 * `🐍️journey-probe.mjs` on :6023 — the published `World3dComputeStatusV1` of each preview window, the
 * picker's label, the Flow window's widget ids and node statuses, and the mesh oracle's own verdict,
 * once per poll. The law replays them through `convergenceVerdictV1` and asserts what the verdict
 * must be, including the two synthetic derivations the fixture labels as such (a sequence stretched by
 * load, and a sequence whose last published reading simply repeats — which is exactly what a stalled
 * preview publishes).
 *
 * Usage: cd <ticket> && bun 🧪️convergence-budget-law.mjs
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { CONVERGENCE_BUDGET_V1, convergenceLoadFactorV1, convergenceSignatureV1, convergenceVerdictV1 } from "./🐍️convergence-budget.mjs";

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️convergence-sequences.json"), "utf8"));
const failures = [];
const check = (name, actual, expected) => {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures.push(`${name}: expected ${JSON.stringify(expected)}, measured ${JSON.stringify(actual)}`);
  console.log(`${ok ? "✓" : "✗"} ${name} :: ${JSON.stringify(actual)}`);
};

/** 🐌 The same recording, replayed as if every poll had taken `factor` times as long — the machine
 * under load, with the PRODUCT unchanged. A wall-clock budget fails this; a poll-counted one does not. */
const stretched = (samples, factor) => samples.map((sample, index) => ({ ...sample, t: samples[0].t + (sample.t - samples[0].t) * factor + index * 0 }));
/** 🛑 The same recording, cut before it converged and then left publishing its last reading — which is
 * literally what a preview that has stopped does. Derived, and labelled as derived. */
const stalledAfter = (samples, cutIndex, extraPolls) => {
  const head = samples.slice(0, cutIndex).map((sample) => ({ ...sample, converged: false }));
  const frozen = { ...head[head.length - 1] };
  const gap = head.length > 1 ? head[head.length - 1].t - head[head.length - 2].t : 1000;
  return [...head, ...Array.from({ length: extraPolls }, (_, index) => ({ ...frozen, t: frozen.t + gap * (index + 1) }))];
};

console.log(`[DEBUG] convergence budget ${JSON.stringify(CONVERGENCE_BUDGET_V1)}`);
console.log(`[DEBUG] recorded sequences: ${fixture.sequences.length} from ${fixture.recordedFrom}`);

check("every recorded sequence carries more than one poll", fixture.sequences.every((row) => row.samples.length > 1), true);
check("a signature is stable for an unchanged reading", convergenceSignatureV1(fixture.sequences[0].samples[0]) === convergenceSignatureV1({ ...fixture.sequences[0].samples[0] }), true);

for (const row of fixture.sequences) {
  const verdict = convergenceVerdictV1(row.samples);
  check(`recorded "${row.label}" → ${row.expected}`, verdict.verdict, row.expected);
}

//#region 🐌 Load
for (const factor of fixture.loadFactors) {
  const converging = fixture.sequences.filter((row) => row.expected === "converged");
  const verdicts = converging.map((row) => convergenceVerdictV1(stretched(row.samples, factor)).verdict);
  check(`every converging sequence still converges at ${factor}× poll latency`, new Set(verdicts).size === 1 && verdicts[0] === "converged", true);
  const loads = converging.map((row) => Math.round(convergenceLoadFactorV1(stretched(row.samples, factor)) * 10) / 10);
  check(`the load factor is READ, not assumed, at ${factor}×`, loads.every((load) => load >= Math.min(factor, CONVERGENCE_BUDGET_V1.maximumLoadFactor) - 0.6), true);
}
//#endregion 🐌 Load

//#region 🛑 Stall
for (const row of fixture.sequences.filter((entry) => entry.expected === "converged")) {
  const stalled = stalledAfter(row.samples, Math.max(2, Math.min(4, row.samples.length - 1)), CONVERGENCE_BUDGET_V1.stallPolls + 2);
  const verdict = convergenceVerdictV1(stalled);
  check(`"${row.label}" frozen after ${Math.max(2, Math.min(4, row.samples.length - 1))} polls is STALLED`, verdict.verdict, "stalled");
  check(`"${row.label}" stall is reported before the poll ceiling`, verdict.polls < CONVERGENCE_BUDGET_V1.ceilingPolls, true);
  const stalledUnderLoad = convergenceVerdictV1(stretched(stalled, 6));
  check(`"${row.label}" stays STALLED at 6× poll latency — load never rescues a dead preview`, stalledUnderLoad.verdict, "stalled");
}

const shortStall = (samples) => {
  const head = samples.slice(0, 3).map((sample) => ({ ...sample, converged: false }));
  const frozen = { ...head[head.length - 1] };
  return [...head, ...Array.from({ length: CONVERGENCE_BUDGET_V1.stallPolls - 2 }, (_, index) => ({ ...frozen, t: frozen.t + 1000 * (index + 1) }))];
};
check("a pause SHORTER than the stall budget is not yet a stall", convergenceVerdictV1(shortStall(fixture.sequences[0].samples)).verdict, "running");
//#endregion 🛑 Stall

console.log(failures.length === 0 ? `[DEBUG] convergence budget law GREEN (${fixture.sequences.length} recorded sequences)` : `[DEBUG] convergence budget law RED\n${failures.join("\n")}`);
process.exit(failures.length === 0 ? 0 : 1);
