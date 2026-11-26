# Flow host read carving

Source admission refused `semio-os-flow-host-read` (28 files, 263625 of 262144 bytes). The `✏️authoring` subtree moved into its own crate `semio-os-flow-host-read-authoring`; the boundary metadata is unchanged.

## Before / after (Rust sources)

| crate | before | after |
| --- | --- | --- |
| semio-os-flow-host-read | 28 files, 263625 B (rs+json) | 6 `.rs`, 91855 B |
| semio-os-flow-host-read-authoring | - | 21 `.rs`, 198255 B (+ 6 json fixtures, about 15 KB) |

Both are below 262144 B and 32 files. No cycle: authoring never referenced read items (only the root import prelude, which it now carries itself); read does not depend on authoring.

## Mechanics

- New manifest and project.json in `✏️authoring/📦️packages/🦀️rust/`, lib path `../../🦀️.rs`; the authoring root became the crate root with a minimal import header (test-only `artifact::*` and `Arc`).
- Children: `use super::super::*` became `use super::*`; `endpoint` uses `super::super::{HistoryFoldIndex,OperatorInfo}`.
- The controller laws test module moved with the crate (`controller_laws`).
- Read: authoring module, re-export and laws module removed; unused imports pruned.

## Dependents

- `semio-os-flow-host` (the facade): new dependency plus `pub use semio_os_flow_host_read_authoring::*;`. `semio-os-flow` re-exports `FlowAuthoring*` through that facade, `semio-os-flow-host-laws` and `-session` use none of the moved items, so they need no change.
- Registries: root `Cargo.toml`, `🧰️framework/Cargo.toml`, `🔒️dependencies.json`, `📋️build-owners.json`, browser-package `📜️script.ts` test list.

## Verification (repo binary only)

- `check -p authoring -p read -p session -p host -p host-laws -p semio-os-flow`: exit 0.
- `check` of `✏️s/.../🗂️curation` workspace with `--tests`: source admission passed all 884 units with no boundary refusal; the build then stops on an unrelated in-progress error in `semio-s-artifact-sourcing-curation-operations` (`curation_set` missing). Note `-p semio-s-artifact-sourcing-curation` from the root matches no package; the suite is its own workspace.
- Tests: not runnable. `check --tests` shows the existing tests do not compile independent of this carve: the compare tests in read and the controller laws in authoring call `close_step(grant)`/`step(..,grant)`/`begin_granted`/`step_granted` with the pre-`RetainedAllocationEffect` signatures (test files untouched since Oct/Nov). A later `test` run additionally hit a peer-side break in `semio-framework-geometry-primitives`. Test counts are therefore unmeasured; test code moved verbatim.

## Tests

Ported to the effect-shaped API (`RetainedAllocationEffect`): read compare tests, authoring controller laws and the host-laws suites. Shared lending helper: `⚖️laws/🧰️effect/🦀️.rs`.

| crate | result |
| --- | --- |
| semio-os-flow-host-read | 1 passed (compare law) |
| semio-os-flow-host-read-authoring | 0 passed, 1 failed: controller law passes every operation, profile and cancel point and fails only on its last line, 3 retired registry versions remain (see below) |
| semio-os-flow-host-laws | lib 37 passed, 18 failed; `original-evaluation-pipeline` 1 passed, 1 failed (`..._under_one_fuel_and_512_items`); `recorded-math-resume` 1 passed |

Production changes:
- `SourceCompare::demands` quoted 0 copy bytes for the page reservation although the paged list requires its header copy bytes; it now quotes `next_reserve_copy_byte_demand`.
- Authoring: every work embedded the 7856-byte `OriginalConstructionAdmission` (ConnectWork 28768 B). The five top-level works and the `WidgetWork` of add/insert (34401 B) live in a seeded `Slot` (bounded 1 KB template parts, box handoff, separate release events).
- Authoring `Construction` no longer drives a decoder construction. See "Controller law" below.

Fixture: the first controller profile granted 1 item and 4096 copy bytes per call. The documented settled-construction quote (`original_construction_settled_turn_quote`) needs 29 items and 14165 copy bytes for one construction turn, so the profile is now 64 items / 16384 bytes (second profile unchanged).

### Controller law

Root cause of the zero-grant progress (7 items, 2472 B, 880 capacity): not a pending construction. `admit_original_construction_settled` settles bind, constructor and settle inside one call. The hidden cost is the resident Source birth service, which `OriginalJobFixtureCaller::turn_with_decoder` runs at the start of every turn from the widened grant, whatever the business grant: the first decoder receiving requests the return table (`birth_requested`, never cleared), the next turn start births it (7 items, 880 B), and from then on each turn start refreshes the hidden origin (9 items, 6064 B) once a receiving used it. No producer can avoid that charge as long as it uses the decoder; a zero-grant call that follows a decoder construction always records progress.

Fix: `Construction::make/run` (`✏️authoring/📦️storage/🦀️.rs`) now performs each Vec reservation as one funded original effect (`StepContext::with_original_effect`, exact receipt through `consume_work` and `consume_retained`, the same path the `Slot` events use). It needs no decoder, return table or origin, so no call leaves a refresh owing. Works lost their vestigial admission slot and close wrappers.

Remaining: the law's last line `assert_eq!(drain_flow_extension_registry_retirements(),0)` sees 3 retired registry versions. The drain loop stops when the front version (demand 40 copy bytes, depth 3) makes no progress; raising the drain grant (copy, items, pump fuel) did not change it, so a live reader of an old registry version is still held. Not a producer defect; owner is `📔️registry` (flow operators).

### Round 3 (flow-host-3)

Fixed in production: `OrderedMap` now has `from_value_controlled` and `default_value_controlled` (admits each row's node bytes, refuses duplicate keys), which unblocked `layout` of `FlowHostSnapshot` in controlled decoding; `FlowInvocationOriginWork` phase 10 quoted only the lease capacity while `SharedOriginalStorage::admit_from` needs `source_admission_demand` (copy, capacity, depth, work), so the origin work never completed; `publish_eval_json` dropped an unchanged counted alias instead of retiring it; the cold registry drain funded 4096 copy bytes while the quote of the front registry is 14200 (`test_flow_registry_retirement_step` now funds the quote); new accessors `FlowEvalSession::neural_cache_ref` (no counted alias to retire) and `next_retirement_reserve_demands`.

Test ports: pipeline and publication laws read the shared caller fixture instead of hard-coding 512 items and 32768 fuel; host retirement law builds its Store through `FLOW_HISTORY_STORE_FACTORY` (a bare `FlowStore::new` has no catalog and no owner disposer by design); latch laws expect `Progress(empty)` for a zero item grant; copy grants fund the quote (`copy.max(quote)`) in the Work, session close, host close and preview laws; displaced or unchanged counted aliases are retired instead of dropped; the origin work, baseline and caller retire loops use `depth.max(1)`.

Remaining failures (18 lib, 1 pipeline, 1 authoring):
- Authoring controller law: the front retired registry version keeps quoting 40 copy bytes at depth 3 and its `close_step` returns an empty `Progress` with a full quote funded; rotating the queue did not help. A reader or alias of an old registry version is still held; not found in the authoring crate (it never touches the registry), look in `neural_engine` `RegistryRetirement`/`ControlledRetirement<Registry>` and the contributed `OperatorImpl`s.
- Work laws (`work_tests` x2, `original_work_opportunity_laws` x2, pipeline `..._one_fuel_...`): the caller loans exactly 1 fuel per call and the first Work or close step quotes more than one work unit (phase 0 of Work with demand 137 copy bytes, depth 2); passing needs the Work quote to fit one unit or a decision on the law, not a larger loan.
- Preview laws (`moves_mesh16`, `replaces_same_handle`): now run to completion; the committed mesh `to_value` lacks `attributes`/`colors` of the oracle (production gap in the completed mesh receiving, or an oracle that the receiver no longer carries).
- Zero or denied grant records progress (Source birth service in `OriginalJobFixtureCaller::turn_with_decoder`): `original_json_field...`, `original_work_publication_preserves_genuine_completion...`, `flow_source_read_rejects_original_stale_foreign...`.
- Old `StepContextOwner` callers without a parent Source: `evaluation_tests::...donates_dictionary...`, three `read_tests`; plus `original_invocation_ledger...` (1 item caller cannot pay the origin refresh).
- `original_dag_host_retirement...`: `EffectBank` effects carry no producer invocation (`native retirement requires its original producer invocation`); the test must drive the DAG close through an invocation-capable effect.
- `original_baseline_publication...`: byte ledger leaves 25445 bytes unreleased (84190 vs 109635); `session_reset...`: `reserve_retirement_step` does not admit the queue slot with the quoted grant.

### Host laws: failures left (each needs its own port)

- Zero-grant or denied-grant call records progress (same Source birth service, or a non-atomic step): `original_json_field_receives_actual_snapshot_and_closes_canceled_partial_custody` (7 items, 880 B), `original_work_publication_preserves_genuine_completion_and_all_physical_owners` (1 item, 63 B), `flow_source_read_rejects_original_stale_foreign_and_missing_sources_before_any_edit` (1 item, 173 B).
- Fixed caller grant of 1 item cannot also pay the per-turn origin refresh, so closes never finish (`turn < 65535`, `close.terminal_is_empty()`): the three `preview_tests::original_completed_preview_*` mesh and cancel laws, `original_invocation_ledger_preserves_sources_and_allocation_receipts`.
- Old `StepContextOwner` callers have no parent Source any more (`original caller has no parent source supplier`): `evaluation_tests::original_completed_preview_evaluation_donates_dictionary_to_actual_original_paid_cache`, `read_tests::flow_source_read_tick_close_keeps_original_context_until_every_owner_is_empty`, `read_tests::flow_source_read_tick_store_read_keeps_exact_original_issuer_and_full_copy_close`, `read_tests::flow_source_read_owner_births_its_exact_allocation_and_releases_it_on_close`.
- Production gaps: `layout.value owner has no controlled native construction implementation` (`work_tests` x2), `artifact store has no installed owner disposer`, `native retirement requires its original producer invocation`, shared original alias not retired (`session_retirement_tests` x7), `original origin is transferred once` (`invocation_origin_work_tests`).
- Stale policy fixture: `🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json` now grants 16777216 items and 65536000000 fuel while the opportunity laws assert 512 items and 32768 fuel: both pipeline target tests, the two `original_work_opportunity_laws`, `original_work_publication_fits_original_editor_opportunity_without_new_funding`.

Test ports done this round: dispatch law uses the shared request fixture's extension id; invocation identity and ledger loops, session close loop and preview retention and cancellation loops fund the real demand (copy bytes) instead of the tiny body budget; publication work test local `done` flag no longer shadows the helper; read compare and invocation session steps pass effects; read test retires the shared snapshot through `elect_cold`.
