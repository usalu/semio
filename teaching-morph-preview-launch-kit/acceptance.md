# Morph and preview acceptance contract

These are required behaviors and evidence, not a prescribed API or a claim about the current repository. Adapt the existing architecture while preserving these invariants.

## Preview/final behavior

| Concern | Preview | Final |
| --- | --- | --- |
| Transition before completion | Only the source; translate/scale according to the declared placement and easing | The authored full morph, including required intermediate states |
| Terminal sample | Atomically show the exact target; source absent | Exact authored target endpoint |
| Canonical references and time | Same semantic IDs, scene state, duration, narration, and endpoint timing | Same |
| Asset detail | Centrally resolved lower LoD, loaded without constructing the final asset first | Required final detail |
| Expensive morph work | Zero full-morph preparation/evaluation attributable to this job | Real full path executes; no silent preview fallback |
| Teaching evidence | Preserve important values, labels, events, and causal relationships with cheaper rendering | Preserve the full didactic progression |
| Cached output | Explicitly profile-scoped and bounded | Cannot consume preview output as final |

## Terminal sample definition

For an inclusive transition frame range `[f0, f1]`:

- Before `f1`, the visible identity is the source.
- At and after `f1`, the visible identity is the target, until a subsequent authored event takes ownership.
- For `f1 > f0`, derive normalized progress from the scheduled frame/time position and clamp it. Easing changes the translation/scale, never the identity boundary.
- For a one-frame interval, the sole frame is the target. A zero-duration transition switches atomically at its declared boundary.
- A ten-frame transition scheduled at frames 100 through 109 shows the source on 100 through 108 and the target on 109. The terminal sample belongs to this transition, even if the containing clip continues.
- A half-open internal interval must normalize to a rendered terminal sample once in the shared evaluator. Do not wait for an end timestamp that the output schedule never samples.
- For continuous-time playback, define the equivalent canonical boundary explicitly. If frames are dropped and playback crosses it, the next evaluation must show the target.
- Adjacent transitions have explicit ordering/ownership, with no double drawing or missing subject at a shared boundary.
- Rendering a trimmed subrange must evaluate the original timeline correctly; it must not start a new local morph or require earlier frames to have executed.

The source/target pose sampling policy (live or snapshot) is declared in data. Logical anchors, units, and placement remain stable across different mesh/path bounds and LoDs.

## Required correctness cases

1. Immediately before, exactly at, and immediately after the terminal sample.
2. Forward playback, reverse playback, arbitrary seek, scrubbing, pause/resume, loop restart, and offline evaluation at the same timestamp.
3. Skipped terminal frames, variable delivery cadence, representative output frame rates, zero/one-frame intervals, adjacent/chained transitions, and trimmed exports.
4. Easing that overshoots or reaches one early; it must not switch identity early.
5. Different asset dimensions/origins, nested transforms, asymmetric and zero-size bounds, and live versus snapshot pose/state.
6. Every supported view, scale, LoD, and analytical scenario used by migrated clips; missing combinations produce actionable validation errors.
7. Persist, close, reopen, and render the declarative definition with equivalent results.
8. Switch preview/final rapidly and run independent preview/final jobs sharing assets; no mutation, stale results, or profile contamination.
9. Authoritative source-only preview intermediates, target-only terminal frames, and genuine full-morph intermediate frames for each semantic category actually used by the course.

## Work and lifecycle instrumentation

Measure per job/profile, including cold preparation, imports/loaders, eager factories, prefetch, worker jobs, and cache creation. A preview job must not construct a full morph offscreen. Bounded low-detail target preparation is permitted so the swap has no blank frame.

Repeat open/play/seek/switch/export/cancel/close cycles long enough to expose retained work. After warm-up and cancellation/drain, check that retained resources plateau within an explicit budget: cache entries/bytes, active tasks/workers, timers/animation callbacks, listeners/subscriptions, object URLs, decoded assets, heap and GPU allocations where observable. Shared resources remain valid for other owners, then become releasable after the last owner exits. Record instrumentation gaps honestly.

## Performance and integration evidence

Capture a baseline and comparison with matching scene fixtures, revision context, device, viewport/output settings, time schedule, and cache state. Avoid competing heavy workloads. Report cold preparation/first-frame latency, warm frame-time median and tail, seek responsiveness, total render time, and retained resources where measurable. Set achievable preview budgets from the measurements and show whether they were met.

Provide a coverage matrix for real lesson/player/editor routes, preview surfaces, scrubbing, thumbnails/posters, exports, and generated-media references. Include actual browser/runtime checks and inspection of representative exported media. A standalone demonstration does not establish complete migration.

Record each encountered fundamental defect, including unrelated ones, with reproduction, owner, root-cause fix, and regression evidence. Final completion requires implemented behavior, measured results, relevant passing gates, preserved teaching content, and explicit concrete blockers for anything that remains unverified.
