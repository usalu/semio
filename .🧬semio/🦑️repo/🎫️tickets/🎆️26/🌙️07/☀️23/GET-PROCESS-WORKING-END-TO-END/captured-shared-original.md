# Captured Shared Original: Work Rebind

## Origin
`FactorySharedRetirement` (`🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs`) captures an `OriginalWorkBindingIdentity` (scope, issuer, Source turn) at admission (`admit_original_into`) or at its first close action. Every later close turn compared the effect against that first binding (`follow_between_actions` / `matches_effect`), so a legitimate continuation under a later Source turn (new Work scope per turn) raised "captured shared original changed its Work or destination".

## Fix (found already applied in the shared tree by the store-close/job agents; verified, not re-edited)
- `invalid(cause)` now names the exact cause (message string is gone from the tree).
- `close`: when no reply is pending and the effect carries a Source turn, the binding is rebound to that effect (`*work=from_effect`); a pending/recovery action still requires the same Source (`matches_source`); no Source after binding to one is refused.
- `close_original_birth_step`: same rule (rebind unless a reply is pending).
- Same principle as seed rebind: pin the owner identity, admit per turn under the funding effect.

## Verification
- `semio-framework-artifact-space-collection --lib builtin_creation`: 1 passed (`stale-c2.txt`).
- process3d editor `process3d_bearbeiten_pane_smoke` (`replay-ps4.txt`): 15 passed, 3 failed. No failure carries the captured-shared message any more. Remaining failures are different:
  - `pane_smoke::bearbeiten_pane_loads_applies_undoes_and_closes`: "host lifetime fuel exhausted across fixture turns" (host-stall-and-drain lead).
  - 2 tests: "registered fixture did not reach its exact terminal-empty witness" (release_bytes quote, depth 3/4), i.e. the close drain, not the binding.

## Law status
No new law added: the existing shared-factory laws use the law bank, whose effects carry no Source turn, so they cannot cross two Source turns. A cross-turn law needs a Job-turn fixture (`OriginalJobFixtureCaller`, as in plugin `registry/original` tests) with two `StepContext` turns; the end-to-end coverage today is the 15 passing process3d pane tests plus the collection test. Open item.

## Refund lead (unquoted receiving share)
`🧵️job/⏱️context/🛬️owned/🦀️.rs` charges `unquoted_receiving_grant(whole)` in `admit_scope`; I did not change that file (another agent is editing it with `crate_dbg` prints). Not verified whether settle refunds the unused share; no law added. Needs the job-context owner.
