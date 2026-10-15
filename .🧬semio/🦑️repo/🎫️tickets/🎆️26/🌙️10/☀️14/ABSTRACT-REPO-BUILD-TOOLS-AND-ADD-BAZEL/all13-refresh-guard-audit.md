# ALL13 Refresh Guard Evidence

This is a bounded read-only investigation of the current loader Source and Root-owned `🗑️generated/final-all-current13.log`. No Source, quota, receipts, process state, authority or producer was changed. No FactStore, ledger or other runtime producer was opened. Root owns terminal/PID and publication qualification.

## Actual Refusals

The first log read was bounded at the observed size **121,079,210 bytes**; the follow-up read was only the final **1 MiB** of the terminal log (EOF **121,280,480 bytes**). The second read is distinct from the initial incomplete view.

1. Graph discovery `build:graph:complete` phase began at **734.642 s** and completed at **737.305 s**. The explicit `[DEBUG] invalidated completed graph` record at byte **119,927,152** reports `membershipChanged:true`, no removed paths, and two added members:
   - `🧰️framework/🔨️modules/🧵️job/👷️worker/🎟️authority/🧪️tests/🟦️.ts`.
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/📍️workspace/🟦️.ts`.
   The record has `stale:[]`; this means no stale examples were logged, not an independently established `holds:true`. Membership inequality alone is sufficient for refusal. Refresh began at **755.277 s**.
2. The second graph-complete phase began at **860.907 s** and completed at **864.279 s**. Its explicit invalidated-completed record reports `membershipChanged:false`, no added/removed paths, and a stale file observation of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch`. The recorded fact is 121,224 bytes, SHA-256 `b8ed82f74478d01ac1756a9d006e3ea7bb62d7303ab03a92b7681833674274e3`, inode 332424039, mtime/ctime `1792042481937.7683`. This diagnoses failure of that recorded Source observation. Current file bytes/stamps were not read here, so byte-change versus metadata-change is not classified. Refresh 2 began at **873.414 s** and the existing qualification guard refused a third candidate after two real invalidations. The terminal log contains `OwnedCommandFailure: build:all-owned-producer failed (1)`. This is the existing qualification-count refusal, not an original-deadline claim or authorized stop.

The earlier Cargo-admission round-0 settlement conflict is a different inner inference event. It cannot substitute for either explicit later completed-candidate refusal above. Cumulative project/image/assembly counters are not used here to infer completed project counts.

## Loader Branch Mapping

The inspected `loadBuildGraphAttemptV1` has the following relevant undefined paths. Both actual refusals above map to the final membership/Source-facts branch at graph module line 301.

| Guard | Existing trace/behavior | What this run establishes |
| --- | --- | --- |
| Initial inventory throws `BUILD_MEMBERSHIP_CHANGED` (274) | Silent undefined before discovery | Possible generally; not the observed post-completion branch |
| Current cached graph passes schema/digest but second current-facts check fails (283) | Silent undefined on cached path | Not evidenced as either identified refusal |
| Discovery failure followed by changed membership/facts (294) | Normally logs original failure and invalidated inference; typed recheck membership error can return silently | Not the actual completed-graph records |
| Final inventory throws `BUILD_MEMBERSHIP_CHANGED` (296) | Silent undefined | Actual records prove final inventory returned for both identified refusals |
| Selected initial/final admission differs (300) | Silent undefined | Inapplicable to this ALL selection |
| Final membership differs or captured Source facts fail (301) | Explicit invalidated-completed record with membership/add/remove/stale examples | **Actual first membership refusal and actual second patch-observation refusal** |
| Final post-staging current-facts check fails (303) | Silent undefined; staging files cleaned in finally | Possible generally, but not reached after the identified earlier return |

Module-authority mismatch, finite publication-size refusal, parse/schema failure and OS writes are thrown errors, not independent unconditional undefined guards. Some discovery errors can be transformed to invalidation only through the explicit changed-source catch path. No such extra cause is inferred from the phase counters.

`build:graph:complete` is emitted in `discoverBuildGraphV1` at line 236, before its portable-input/schema return and before loader final inventory, complete Source checks and publication writes. It means discovery reached that phase, not accepted publication. `loadBuildGraphV1` emits `build:refresh` after an undefined attempt result; the ticket's unchanged original publish wrapper refuses at its second real refresh. The production loader's own bounded attempt loop and the ticket qualification guard are distinct control layers, neither changed here.

The final membership digest includes admitted paths, types and modes; directory membership is also recorded where inventory supplies it. A membership mismatch is not a content-SHA assertion. Conversely the stale patch row includes a recorded SHA, but does not expose the later/current bytes in this trace. No pin is accepted by this investigation.

## Source Identity at Review

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts`: SHA-256 `41ddc48e0bc2b369d64eb370a3e8ad0db45a690e5d0291bd2576f180ba79c12b`. This is a current read identity, not a substitute for Root’s original loaded-module receipt.
