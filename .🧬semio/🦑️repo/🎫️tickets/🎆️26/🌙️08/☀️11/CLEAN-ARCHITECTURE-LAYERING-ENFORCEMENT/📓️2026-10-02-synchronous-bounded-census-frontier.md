# Synchronous Bounded Census Frontier

Read-only source/log inspection; no jobs. Current source executor is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts`.

## Actual retained failure evidence

Ticket `🗑️generated/goal-root/rust-scope-facts-physical-provider.log` has25 completed macro row witnesses then original45000ms deadline kill, not30 completion. Logged25 physical-capture spans total40404ms, mean1616/max2455; compile-reference total187ms mean7/max51; graph total222ms mean9/max111; scope-target-input-verdict total17351ms mean694/max1859. Concurrent spans overlap; sums are not serial wall time. Policy phase is named policy, not policy-capture. Current synchronous source change is newer than this failed receipt and still needs its own original-route measurement. These measurements do not support prioritizing regex/graph optimizations ahead of capture/physical inputs.

## Current implementation review

Per-path walk uses lstatSync, readdirSync and readFileSync (:86–94). Stable linked source/Cargo/directory checks and ignore/exclusion decisions are unchanged. Non-Rust data link kind still uses awaited followedStat; directory membership never descends through an observed symlink. Area checks/root dir enumeration remain async (:101–106). Children retain indexed membership; final source map is Buffer-byte-sorted (:122).

Workers now continue the entire current batch after an error, record indexed failures and settle before throwing the lowest index error (:109–119). This closes prior timing-dependent first-error selection. No next breadth layer is admitted after failure. Every64 claimed indices schedules setImmediate; each next walk checks cancellation. Because synchronous walk bodies execute on one JS thread, eight workers do not provide eight concurrent synchronous filesystem calls; the scheduling bound is nevertheless retained, and explicit yields let process signals be delivered between blocks of work. No cancellation-free loop through the entire tree is introduced.

Source lstat then read remains a preexisting no-follow race boundary: changing directory/leaf after its check is not closed by synchronous APIs. This review does not assert a new scheduler grant, but it also cannot certify a stable captured snapshot against concurrent physical replacement without the owned source-access law. Deleted reads throw; denied runs publish no verdict. Fresh per-invocation map is retained.

## Measured next cut

First rerun current registered route at original45s (Root owns jobs). If physical capture falls as expected, next evidenced tail is physical input verdict, not compile-reference/graph. inspectRustSourceInputs still awaits lstat for each ancestor and creates a fresh map per source; preserve raw directory prefixes and exact failure codes while testing whether owned synchronous no-follow reads with bounded yield remove scheduling overhead. Do not share stale node maps between source roots or bypass deletion obligations. Area/root async enumeration can likewise become same owned sync capture if phase timing shows remaining cost.

Keep all30 native builds/runtime binaries/deletion/source facts. Split nativeAndFacts into rustc/runtime/direct-source facts to quantify the independent compiler contribution. Policy loader phase should be aggregated separately before recommending authority cache changes. No deadline extension, native reuse or injected capture is justified by these receipts.

Existing laws: source-direction ownerCases/ownerRejections, physical missing/link census, raw prefix traversals,30 macro mount/deletion rows. Add owned completion-order/error-index/cancellation adapter laws if scheduling becomes reusable API; stable existing source goldens alone do not exercise signal delivery or two-error ordering.
