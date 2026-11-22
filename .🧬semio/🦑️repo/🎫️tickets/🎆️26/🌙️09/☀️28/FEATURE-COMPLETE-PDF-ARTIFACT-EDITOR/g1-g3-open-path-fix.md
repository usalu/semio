# G1 to G3 Open Path Fix

Scope: the mounted Open File path of the PDF editor (gaps G1, G2, G3, G21 of `pdf-goal-gap-audit.md`). Source paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## Root causes

### G1 and G21: trap in controlled retirement during Open

The panic is the `Drop` assertion of `ControlledRetirement` (terminal-empty required) reached from `CommandIngressTurn::exchange_generic`. Two independent holes existed.

1. The failed-decoded branch copied the primary message to a shell fault and discarded the live typed failure, so the boxed domain was dropped before its retirement ran. The staged correction (failed-decoded commands retire through `retire_command_ingress` and the typed failure returns through the original turn owner) is already integrated in `⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs` and `⚛️reactor/🏃️runtime/🧰️ops/📥️ingress/🦀️.rs` (`retire_failed_step_on_original`). It is covered by the existing `original_failed_open_keeps_the_paid_domain_through_ingress_retirement`.
2. Still open in source before this change: `cancel_for_close` runs before `exchange_generic` in the same turn. When a close interrupts output retirement with a bounded grant, `output` has moved into `output_close`, yet `exchange_generic` ran the command exchange again on the half-retired owner and finally replaced the whole `Generic` state (`put_state(None)`), dropping a non-empty `ControlledRetirement`. That is the "dropped before terminal-empty" trap, and the following close trapped on the same state.

Fix: `CommandIngressOwner::exchange_may_run` refuses to run or replace a generic owner whose instance is closing or whose `output_close` is populated. The retirement ladder is then the sole owner until every member is terminal empty.

### G2: lifecycle accounting refusal ("original lifecycle receipt differs from its caller authority")

Two causes, both already corrected in source by earlier slices, now each pinned by a native regression test:

- The WASM32 conversion of the default 4,294,967,296-byte Pack allocation limit saturated to 4,294,967,295 and the retained cursor refused it before any PDF byte (`original Pack receiving requires its retained source and caller`). The refusal then sent the fresh editor through retirement. Fix present: `addressable_allocation_limit` in `👷️job/🛬️receiving/📥️pack/🦀️.rs` (limit capped at `isize::MAX`); native law `👷️job/🛬️receiving/📥️pack/🧪️tests/🦀️.rs` reads the neutral `allocationLimits` corpus.
- The store close rung settled a physical move or box release with zero work, and `MountedOwnerTurnV1::advance_lifecycle` refuses fresh physical progress without a fuel debit (third condition of the refusal). The charge is now the named function `original_close_work` (`🪜️maintenance-ladder/📥️staging/🏪️lifecycle-stores/🦀️.rs`) used by the rung in `🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs`; the new native law `original_store_close_work_charges_every_unmetered_physical_receipt` checks every row of the neutral `closeWork` corpus (SQLite-verified by `📜️pdf-close-store-work.ts`).

### G3: 963-byte PDF never publishes

The 963-byte fixture selects the synchronous lane, but the Pack export of its snapshot spans about 32,960 work units in 64-unit slices. `NaturalFileImportJob::note_publication_work` previously delivered, acknowledged and retired one 17-byte checkpoint (one 16 KiB payload backing) for every nonempty slice, 515 round trips, so the mounted Open spent its whole poll budget in Worker stage with a live session and no publication (the sampled stall at polls 8,192 to 8,448 and the six matching 16,384-byte allocate/release pairs per 64 receipts). Fix (already in source): `publication_checkpoint_due` emits one checkpoint per contract quantum and on stage change, 9 instead of 515. Worker progress is reported from the Worker's own checkpoint progress (`🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs`).

## Changes

| File | Change |
|---|---|
| `⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs` | `exchange_generic` returns early unless `exchange_may_run` |
| `⚛️reactor/🔄️turn/🔌️extension-invocation-and-command-ingress/🦀️.rs` | new `CommandIngressOwner::exchange_may_run` |
| `⚛️reactor/🔄️turn/🧪️tests/⚖️invocation-failure/fixture.rs` | fixture accessors `output_retiring`, `exchange_may_run` |
| `⚛️reactor/🔄️turn/🧪️tests/⚖️invocation-failure/🦀️.rs` | new test `original_guest_failure_debit_interrupted_output_retirement_holds_the_exchange_until_terminal_empty` (G21) |
| `🪜️maintenance-ladder/📥️staging/🏪️lifecycle-stores/🦀️.rs` | new `original_close_work` |
| `🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs` | store rung uses `original_close_work` |
| `♻️lifecycle/🏪️stores/🧪️tests/🦀️.rs` | new test `original_store_close_work_charges_every_unmetered_physical_receipt` (G2) |
| `📜️declaration/📥️natural-file/🦀️.rs` | `publication_checkpoint_due` is `pub(crate)` for its law |
| `🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/📶️publication-checkpoints.json` | neutral checkpoint schedule (G3) |
| `🧪️tests/🧬️mutation-fixtures-surface/📄️natural-file-reserved-job-laws/🦀️.rs` | new test `natural_file_publication_checkpoints_follow_the_contract_quantum_not_every_pack_slice` (G3) |
| ticket: `g1-g3-checkpoint-oracle.py` | independent Python schedule that writes and verifies the fixture |

Note: `🧪️tests/📦️units/📋️surface-fixture.json` already disagreed with HEAD for the touched test file before this work, so its hashes were not regenerated.

## Test evidence

Verified so far:

- `python3 -I g1-g3-checkpoint-oracle.py`: independent schedule (515 legacy checkpoints to 9) matches the fixture.
- A first `repo cargo ... test -p semio-plugin-store-stage-laws` compile showed that crate's test fixture (`♻️lifecycle/🏪️stores/🧪️tests/🦀️.rs`, `ControlledRetirement::step` API drift) did not compile before this work; it was repaired (the new store-close-work test itself raised no compile error).

NOT yet executed (tests written, never run): the repo capacity gate queued every compile behind other agents' builds, and repo tooling failed twice from other agents' in-flight edits (`admissionReserveBytesV1` import, host capacity policy `/swap/activeMegabytesPerSecond`). A chained retry script is running in the background and writes logs next to this file:

`sh 🗑️generated/g1-g3/run-all.sh` runs, in order, through `repo cargo --store=shared --configuration=prototype -- test --offline`:

1. `-p semio-plugin --lib --features artifact-app-testing,original-actor-context-testing original_guest_failure_debit_` (G1/G21 new test plus the existing failed-open laws) into `reactor.log`
2. `-p semio-plugin-surface-fixture --lib natural_file_` (G3) into `surface.log`
3. `-p semio-plugin-store-stage-laws --lib original_` (G2) into `stores2.log`

Each log ends with `exit=<code>`. Unverified risk: the G1 test's bounded grant must actually interrupt output retirement (it asserts `interrupted > 0`); if the ladder completes in one step the grant needs narrowing.
No mounted browser run was attempted (G9 owns the component build).
