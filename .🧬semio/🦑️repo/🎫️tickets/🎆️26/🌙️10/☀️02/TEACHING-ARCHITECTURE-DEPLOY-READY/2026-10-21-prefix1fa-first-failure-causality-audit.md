# Prefix1fa First Failure and Wait Causality Audit

Frozen log217762B/SHA256f53d266804ea194f50079480d535852be1c25153475d6a76d6a6b0c1b1bed4a6;1190 newline records. Read-only; no Source/helper/test execution or Native/process/cache effects. Owner actual exit1/no cancellation is separately established.

## Observable Chronology

Lines1–459 host capacity wait; last2694s/3584MB/7running/0ahead.460 admits exec;461–462 repository:build repo loading/running.463–639 boot recovery/parser metadata/dependency verification.640–706 shared graph-image request (initial8/8); explicit SHwait642 through706.707 resource-directory admission;708–710 exclusive request5/5. Last EXactivity1174=1784084ms;1177 request4/7. Queued PID is not verified kernel holder. No unit/prefix/lateSource markers.1185 is only anchored actual error; preceding Error strings in diagnostic code are not earlier failures.

## Primary Preservation Finding

Final stack remaining214:643 → control18:29 → persistent Native command142:5706. Exact current column lies in maintenance acquisition catch: catch(error){checkResourceLeaseControl(actual);receiver.stage="closed";persistentReceivers.delete(receiver);...throw error;}. The recheck can throw NEW deadline error, replace caught primary and skip closed-state bookkeeping. Fixed backend98 repeats pattern. Original caught object/message is not logged; cannot reconstruct first cause or honestly identify final deadline as primary.

Wait helper platform/⏳️wait/🟦️.ts records acquire/observer failure, then unconditionally replaces it on final parent-control failure. Authored policy deliberately prioritizes parent cancellation over owned withdrawal, but independent earlier error followed by expiry is a distinct missing vector.

withNativeImageTerminalCloseV1 correctly preserves body primary over close, but executes only AFTER acquisition succeeds and maintenance owner is issued. Final stack is preceding wait catch, not evidence of terminal-close/cleanup-body failure. Directory name cleanup does not prove cleanup phase.

## Original and Loan Evidence

Both backends request0 maintenance refinement from existing opening control, retaining inherited deadline. Withdrawal child also requests0. Actual repository command/control existed before prefix entry. Log contains no Original id/wire/deadline ancestor; exact issuance and winning ancestor are delegated to Root/other auditors. Neither wall delay nor queue elapsed substitutes for that proof.

## Existing Authored Tests

Native storage/platform/⏳️wait/🧪️tests/🟦️.ts + sibling🔣️.json + platform schema:18 closed cases (queue/withdrawal/unrelated progress/parent cancellation-deadline/identity/undefined/queue cleanup/late release/joined observation), SQLite decision oracle, backend AST bracketing. These use single/few-turn ports/setImmediate, not2694s admission plus1784s EXwait.

Caching graph/headroom tests cover actual macOS pressure/reader/queued/cancel, queued optional cancellation, Nx close/recovery and revalidation. Terminal-close primary test covers falsy/undefined, not pre-acquisition catch overwrite. No hour-scale issuance inheritance or earlier-independent-error-plus-later-expiry proof found in inspected owners. Tests not executed.

Recommend schema-first earlier primary plus later control failure (including undefined), actual wait and backend catch-body identity/state assertions, while preserving intentional parent-over-owned-withdrawal rule. Use deterministic clock/control inheritance vectors instead of multi-hour sleeps. No implementation or run undertaken.
