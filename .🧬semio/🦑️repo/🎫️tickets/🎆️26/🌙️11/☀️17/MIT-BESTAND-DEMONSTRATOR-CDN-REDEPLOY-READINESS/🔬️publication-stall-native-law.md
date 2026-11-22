# Publication stall native law (mounted browser grant)

Status: law written, NOT yet executed. Two repository gates in the compile path failed for reasons outside this change, so there is no pass or fail verdict on the stall yet.

## The law

- Test `a_document_archive_load_and_its_typed_publication_complete_under_the_browser_mounted_grant`, included from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🪜️mounted-grant-archive-publication/🦀️.rs` (new `include!` line in the part-4 `🦀️.rs`; crate `semio-plugin-tests-14-2`).
- Language-agnostic rows: `.../🪜️mounted-grant-archive-publication/🧫️fixtures/🔣️.json` (browser grant 64 items / 32 KiB copy / 16 MiB capacity / 1 MiB release / depth 64 at fuel 2,000,000 and 200,000; the current `INTERACTIVE_COMPONENT` grant).
- Flow per row: archive a composed parent with an owned member (real `document_archive`), admit the real archive load into a fresh app (store replacement plus member open), then drive only the production path `MountedOwnerTurnV1::admit` + `mounted_owner_maintenance_step` (same entry the browser reactor uses, which calls `maintenance_ladder_step` with `covering_loan`) with the per-turn ceiling renewed each turn, polling the load. It must reach `Ready` within 4096 mounted turns (the stall-limit scale, not raised). Then a typed `CompositeEdit` is dispatched and settled through `settle_registered_typed_operation` under the same policy, and the child must show the published value. A failure names the row, the turn count and the widest quoted copy demand against the unit loan.
- Not covered: WASI run (not attempted), and the fixture TestApp may quote smaller demands than the real Demonstrator (377,696 B member open); the failure message records the widest quote so this can be judged.

## Why no verdict

Runs via `repo cargo ... --parallelism=1 --store=shared`:
1. First run waited about 22 min on compiler-image headroom, then stopped on `semio-plugin-runtime-ops exceeds its source boundary before the compiler: 268,268 of 262,144 bytes` (a peer's concurrent growth; the crate is about 272 KB now).
2. Reruns stop in a repo tooling step: `Closed compiler builtin-read import coverage refused` (Bun capture script, before any rustc). Not caused by this change.
Both need their owners; I did not touch them.

## Static finding worth checking when it runs

`covering_loan` widens the unit grant up to `cx.retained_grant()`, but `MountedOwnerTurnV1::advance_lifecycle` then refuses (`original lifecycle receipt differs from its caller authority`) any turn whose receipt does not fit the mounted grant (`current.fits(grant)`). A unit whose step copies more than the mounted 32 KiB would therefore fault there rather than publish, so widening alone may not be enough; the law's mounted path exercises exactly this.

## Static root cause and fix (second pass)

Cargo is paused on the coordinator's request; nothing below has been compiled or run yet.

Root cause (confirmed by reading): two authorities disagreed about the same unit grant.
- The ladder widened a unit's grant with `covering_loan` up to `cx.retained_grant()`, the original turn ceiling (GB-scale copy).
- `MountedOwnerTurnV1::advance_lifecycle` accepted a producer's receipt only if `current.fits(self.grant())` and `current_progress.fits(self.grant)`, the mounted policy grant (32 KiB copy). A covered step such as the 33,976 B history-replay preparation therefore failed `original lifecycle receipt differs from its caller authority`; without the widening (a peer removed `covering_loan` from the ladder at 06:27, so the silent `consume_fuel(1); continue` skip is back) the same step is skipped forever. Either way the load never moves and the typed operation stalls.

Fix (one source of truth, stall limits untouched):
- `semio-plugin-mounted-owner`: `covering_grant(loan, demand, ceiling)` (moved from the ladder) and `MountedOwnerTurnV1::covering_ceiling()` (mounted item batch over the original turn's remaining copy/capacity/release); new `advance_covering_lifecycle`, sharing one private `lifecycle` body with `advance_lifecycle`, accepts a receipt against that ceiling and the cumulative turn receipt against the admit-time ceiling. `advance_lifecycle` (close, nested jobs) keeps the strict mounted grant, so the existing close-phase refusal laws are unchanged.
- Ladder (`instance/staging/maintenance-ladder/🦀️.rs`) widens with `covering_grant`; `mounted_owner_maintenance_step` (`plugin-app-impl`) uses `advance_covering_lifecycle`. The ladder's local `covering_loan`, its test module and fixture are removed (the law moved).
- Laws: `covering_grant_covers_an_indivisible_demand_only_within_the_ceiling` (fixture `mounted-owner/tests/fixtures/covering-grant/json`: 1 KB funded, 33,976 B, 377,696 B member open, 20 MiB capacity, clamp at ceiling, ceiling below loan) and `a_covered_step_above_the_mounted_loan_is_accepted_only_by_the_covering_lifecycle` (1 KB, 33,976 B, 377,696 B receipts: strict lifecycle refuses above 32 KiB, covering accepts). The native publication law gains a 4 KiB-copy row.
- Note: `semio-plugin-mounted-owner` is not a workspace member for `cargo test -p`; its unit tests need the companion crate that mounts them (none found by grep), so run them through the owning test target.

Not yet run: all of the above.

## Run attempt (third pass)

`repo cargo test -p semio-plugin-tests-14-2 --lib <law>` at `--parallelism=1` compiled the host crate and stopped on 115 compile errors in the part-4 test host, all from other included test files, not from the fix: they call `cfg(test)`-only helpers of other crates (`test_dispatch_emit`, `test_history`, `test_interaction_selection_snapshot`, `test_snapshot`, `document_pack_bytes`), access now-private fields (`.0`/`.1` of `ContractApp`, `app.stores.store`) and hit a `queries overflow the depth limit` recursion-limit error. The host crate cannot compile any law until that split is repaired. Two of those errors were in this law (`test_snapshot`, `document_pack_bytes`); they are replaced by the public `PluginApp::document_archive` member count. The include path of the fixture was also corrected (relative to the included file).

Verdict: still no pass/fail for the stall. Next: either repair the part-4 host (owner of the cfg(test) split) or mount the law in a host crate that compiles, then run it at `-j1`.
