# r11-exec-store-pt3

Scope: plugin test files `🧪️tests/🔬️app-typed-command-full-operation/**` (main file plus `🧩️child-operations/{🫙️owner,📢️publication,🪪️registry,🛫️encoder}`), `🔬️app-artifact-reserved-tool-job`, `🔬️app-artifact-fixed-registry`, `⏳️completion`.

## Status (honest)

- Compile: verified. Gate run c8 (`cargo check -p semio-framework-plugin --lib --tests`, log `🗑️generated/r11-store-pt3/c8.txt`) finished with 0 errors in the whole crate, so my files compile.
  - c3, the last check that showed errors in my files, had two of mine: a missing `ErasedSnapshotRetirement` import (fixed), and a publication used as an erased retirement (a peer rewrote that loop at the same time, using the `retirement_demands` and `close_step` pair).
  - c4 to c7 stopped earlier, in `semio-framework-io-sqlite-snapshot`, on missing value-crate re-exports. r11-store added them.
- Tests: NOT run. Run t1 (`cargo test -p semio-framework-plugin --lib -- --test-threads=1` with filters `typed_command_full_operation child_complete artifact_reserved_tool_job_tests artifact_fixed_registry_tests retained_latest_wins_cancellation_guards retained_latest_wins_reserved_slots retained_latest_wins_real_document checkpoint_then_restore_requeues checkpoint_restart_transient_close`, log `t1.txt`) did not build its upstream dependency.
  - It fails in `semio-framework` (lib) in the test cfg: 39 errors such as `ColdPairIngressStatus`, `JobCheckpoint` and `ActorUiPatchReceipt` missing `serde::Serialize`/`Deserialize`.
  - This is the peer break already noted in `r11-exec-store-f.md`, outside my scope.
  - I claim no test passes. Re-run t1 with the same filters once `semio-framework` builds in the test cfg.

## Changes

- Child owner fixtures (`🧩️child-operations`): every `ErasedSnapshotRetirement` impl is rewritten to the grant design.
  - Affected owners: Text, ChildLabelOwner, PagedChildOwner, PagedChildDecodeOwner, ChildSymbolOwner, ChildGroupPublicationOwner, RetainedSourceRegistry.
  - Each has a private `demands()` quote. The four `next_*_demand` methods read it, and `close_step(grant)` checks it first.
  - An under-granted axis yields `Progress(default)`, and a depth below the quote returns `Err(DepthLimit)`.
  - Nested owners get `nested_grant` (one item, depth minus 1). A child's `Complete` is settled to `Progress`, and the parent pops the terminal child on a later turn as a `copied_items:1` handoff.
  - Shared helpers sit in the encoder file: `nested_grant`, `grant_funds`, `deeper`, `quote`, `handoff`, `yielded`, `settle`, `pop_demand`, `release_demand`, `release_page`.
  - The quotes follow the semantics of the owner's new test `child_complete_candidate_retirement_keeps_currencies_independent`: capacity quote 0, zero terminal quotes, and receipt equal to physical release.
- Main file:
  - `SnapshotRetirementStep`, `PluginCloseStep` and `maximum_bytes` pairs are gone. Fixtures now use 5-axis `ArtifactStoreOneItemGrant` (all byte axes from the fixture's `maximumBytes`, depth 128).
  - New helpers `close_mounted`, `close_app`, `advance_registry` drive owners by quote, then grant exactly. The 4096 page ceiling is still asserted on the quoted copy and release axes.
  - Two fixture changes: the two-turn preparation gets `close_step(grant)` plus the four `next_close_*` quotes, and the local-root retirement factory gets the admitted `retire(snapshot, grant)` signature (`RetainedCloneBirthDemand`, which hands the snapshot back on refusal).
  - Other drives: the presence store retirement uses `store::test_support::drive_retirement`. `drain_retained` quotes then grants exact release (copy is `max(quote, 4096)`) and asserts that a funded turn made progress. ChildEmit tests use `close_one(grant)` and `next_close_byte_demand`.
  - Source-lint test: `pending.close_step(1, PAGE)` becomes `self.handoff_mounted_durable_publication(mounted)?`, the seam the lib uses now.
- Law replacement, not a weakening: in `retained_latest_wins_slot_and_publication_fairness`, the maintenance stage 0 no longer retires Retiring typed operations. The lib moved that to the turn driver via `retire_typed_operation_unit`, so the test now calls the app's one retirement unit with its quoted grant and asserts the publication is released. The stage-18 cancellation-lock law is kept via `maintenance_step(quote)`. The old cursor fairness assertion for stage 0 is gone with the old stage.
- `⏳️completion`: the restart test drives `maintenance_retirement_demands` and `close_retirement_demands` with exact grants. Fixture `🧫️fixtures/⏳️completion/🔣️.json` key `closeBytes` is renamed `closeBodyBytes` (it is the quote body, not a ceiling). No `.ts` reads it.
- `TestConfig` (`🧪️tests/🖥️test-app-mutations-config`): added the `RetireOwned` derive. This is outside my file list but required by the completion errors.
- Reserved tool job and fixed registry: `ArtifactReservedJob` is now a marker trait; tests use `InteractiveJob::close_step` with quoted grants and the exact release byte checks.

## Open risks (runtime, unrun)

- Registry `RetainedSourceRegistry` yields on a wrap or on a non-closing registry (no progress). The tests call `begin_close` where needed.
- The fairness test holds the cancellation lock while retiring op 1 through `retire_typed_operation_unit`. The lease retirement may need that lock. If it deadlocks or yields, the retirement order in that test needs a rework.
- `mounted.retirement_demands` is private to the lib module. It was reachable in the old test, so I expect the same here.
- A peer edited the stale-publication loop in the main file at the same time; I left their version.
