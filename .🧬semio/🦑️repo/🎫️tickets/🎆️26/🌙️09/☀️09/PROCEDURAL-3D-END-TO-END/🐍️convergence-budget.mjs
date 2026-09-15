/** ⏱️ The journey probe's convergence decision, as one pure function over a RECORDED status sequence.
 *
 * 🪪️ Why this exists. The old predicate was `converged(snapshot)` polled once a second inside a fixed
 * `seconds` budget. That makes the verdict a function of the MACHINE, not of the product: the same
 * green build converged 13, then 21, then 13 of 23 rows across three runs with every mesh oracle green
 * in all three (`📓️host-refresh-latency-2026-09-15.md` §5.5, §8). A fixed wall-clock budget on a shared
 * 20-agent box is a load meter wearing a correctness mask.
 *
 * ⚖️ What replaces it. The budget is counted in POLLS, not seconds, and a poll only counts against the
 * step once the preview has stopped moving. A busy machine stretches every poll, so a poll-counted
 * budget stretches with it automatically; the wall ceiling is derived from the observed stretch rather
 * than assumed. What is NOT relaxed is the stall rule: a preview whose published status and oracle
 * reading have not changed for `stallPolls` consecutive polls has genuinely stopped, and is reported
 * red however long the machine took to say so. That is the half a law can check, and
 * `🧪️convergence-budget-law.mjs` checks it over sequences recorded from real runs.
 *
 * @see 🐍️journey-probe.mjs — the consumer
 * @see 🧪️convergence-budget-law.mjs — the law over recorded sequences
 */

/** ⏱️ How a step is graded, in polls. */
export const CONVERGENCE_BUDGET_V1 = Object.freeze({
  /** ✅ Consecutive polls that must all read converged before the step is called converged. */
  stablePolls: 3,
  /** 🛑 Consecutive polls with NO movement in the status signature after which a step is stalled. */
  stallPolls: 20,
  /** 🚧 Hard ceiling, in polls, so a preview that keeps twitching forever still terminates. */
  ceilingPolls: 240,
  /** ⏳ The poll interval the budget is nominally written against. */
  nominalPollMs: 1000,
  /** 🐌 The most a measured poll may be stretched by load before the ceiling stops growing with it. */
  maximumLoadFactor: 8,
});

/** 🔑️ Everything about one poll that means "the preview moved". Deliberately coarse: it is the
 * PUBLISHED status plus the oracle's own reading, never a wall clock and never an internal counter, so
 * a preview that is still computing, still delivering meshes, or still changing phase reads as movement
 * and one that has stopped reads as stillness. */
export function convergenceSignatureV1(sample) {
  const previews = (sample?.previews ?? []).map((preview) => [preview.surfaceId ?? null, preview.phase ?? null, preview.ratio ?? null, preview.computing ?? null, preview.fault ?? null, preview.meshes ?? null, preview.triangles ?? null].join("|"));
  return JSON.stringify([sample?.example ?? null, sample?.widgetIds ?? null, sample?.nodeStatuses ?? null, previews, sample?.oracleOk ?? null]);
}

/** 🐌 How much slower this run's polls actually were than the budget's nominal poll — the honest load
 * signal a probe can read without instrumenting the host. The MEDIAN, so one long GC pause does not
 * declare the machine busy and one fast poll does not declare it idle. */
export function convergenceLoadFactorV1(samples, nominalPollMs = CONVERGENCE_BUDGET_V1.nominalPollMs) {
  const gaps = [];
  for (let index = 1; index < samples.length; index += 1) {
    const gap = Number(samples[index].t) - Number(samples[index - 1].t);
    if (Number.isFinite(gap) && gap > 0) gaps.push(gap);
  }
  if (gaps.length === 0) return 1;
  gaps.sort((a, b) => a - b);
  const median = gaps[(gaps.length - 1) >> 1];
  return Math.max(1, Math.min(CONVERGENCE_BUDGET_V1.maximumLoadFactor, median / nominalPollMs));
}

/**
 * ⚖️ The verdict for one recorded sequence.
 *
 * `samples` are polls in order, each `{ t, converged, previews, example, widgetIds, nodeStatuses,
 * oracleOk }` — `converged` is the caller's own product predicate (published status + picker + graph +
 * mesh oracle), and everything else only decides HOW LONG to keep asking.
 *
 * Answers `{ verdict, polls, seconds, stalledForPolls, loadFactor, reason }` where `verdict` is
 * `"converged"`, `"stalled"` or `"running"` — `"running"` meaning the sequence ended before either
 * terminal was reached, i.e. the caller should keep polling.
 */
export function convergenceVerdictV1(samples, budget = CONVERGENCE_BUDGET_V1) {
  let stable = 0;
  let lastSignature = null;
  let lastMovementIndex = 0;
  const loadFactor = convergenceLoadFactorV1(samples, budget.nominalPollMs);
  for (const [index, sample] of samples.entries()) {
    const signature = convergenceSignatureV1(sample);
    if (signature !== lastSignature) {
      lastSignature = signature;
      lastMovementIndex = index;
    }
    stable = sample.converged ? stable + 1 : 0;
    const elapsedMs = Number(samples[index].t) - Number(samples[0].t);
    const seconds = Number.isFinite(elapsedMs) ? elapsedMs / 1000 : index;
    if (stable >= budget.stablePolls) return { verdict: "converged", polls: index + 1, seconds, stalledForPolls: 0, loadFactor, reason: `settled for ${stable} consecutive polls` };
    const stalledForPolls = index - lastMovementIndex;
    if (stalledForPolls >= budget.stallPolls) return { verdict: "stalled", polls: index + 1, seconds, stalledForPolls, loadFactor, reason: `no published movement for ${stalledForPolls} polls` };
    if (index + 1 >= budget.ceilingPolls) return { verdict: "stalled", polls: index + 1, seconds, stalledForPolls, loadFactor, reason: `poll ceiling ${budget.ceilingPolls} reached while still moving` };
  }
  return { verdict: "running", polls: samples.length, seconds: samples.length === 0 ? 0 : (Number(samples[samples.length - 1].t) - Number(samples[0].t)) / 1000, stalledForPolls: samples.length === 0 ? 0 : samples.length - 1 - lastMovementIndex, loadFactor, reason: "sequence ended before a terminal" };
}
