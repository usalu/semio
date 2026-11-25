# Plugin tests-06 port and the new laws

## Findings that differ from the brief
- The folder-reload laws are not in `semio-plugin-tests-06`. They live in `semio-plugin-tests-07-3` (`🧪️folder-reload-route`), which depends on tests-06. The closed-original registration law is in `semio-plugin-native-identity-tests` (`🧵️instance/🌱️initialization/🧪️tests/🪟️native-window`).
- The tests-06 lib has 8 extra errors when built alone: its `artifact-app-testing` feature is off. Dependents must request it. tests-07-3 now does.
- Two of the 76 errors were split artifacts, not API drift: `member::toy_member_job` and `semio_plugin::Fault`.
- The machine ran out of disk twice during the work, and gate waits were long. Other agents' in-flight framework edits changed results between runs.

## Results (last run, "final")
- tests-06: compiles. `boxed_work` passes. The operation law passed in the earlier run (2/2) and failed in the last one at its heap assertion `(392, 0)`. Other agents' wire/codec edits landed in between; I did not re-diagnose it.
- tests-07-3 compiles (45 own errors ported). 3 of 11 pass:
  - `a_whole_document_load_counts_the_records_its_replacement_decoded` passes (the new decoded-count law).
  - `a_whole_document_load_reports_the_work_it_did_and_never_goes_back` passes.
  - `a_late_release_of_a_press_the_runtime_ended_leaves_zero_trace` passes.
- Failing:
  - `a_cancel_at_any_point_...` and `a_cancelled_load_stops_reading_as_loading_...`.
  - `every_refused_whole_document_load_...`, `a_folder_bound_document_reloads_...`, `two_programs_on_one_folder_...`.
  - Three gesture laws.
- Registration law: not confirmed. See the root cause below, which is fixed. After it the law moved past line 54; a later maintenance turn then showed an unreceipted release of 10752 bytes, not localised. The last run fails earlier at line 139 (`construction.advance`) with retained 36760. Earlier in the session it failed with "Deflate restart requires complete output custody". Both are consistent with concurrent edits by other agents to the operation-wire/deflate code.

## Root causes found
1. Receipt accounting at `registration/🧪️tests/🦀️.rs:54`. `ToolCancellationIssuer::admission_demands` quoted the keyed registries as a flat `ARTIFACT_LIVE_OUTPUT_SLOTS * size_of::<(u64, Scope)>()` (4608). `ArtifactFixedRegistry` is now paged: it allocates only its page table (32 bytes), and pages are born on first insert. The quote is now the registry's own `birth_demands` figure. The receipt now matches the heap at that turn.
2. Cancel at any point, accounting defect not fixed. With the cancel law driven by maintenance turns, ladder stage 14 (store replacement, state Initializing, cancelled) fails with "original lifecycle child receipt differs from its whole producer receipt". `original_composed_lifecycle_step` subtracts the reserved-return progress (1 item, 118 bytes) from the step's progress. The step's own (1, 118) handoff receipt is entirely absorbed, and 1 item and 118 bytes more were posted. Not localised further. Not fixed.
3. The laws' poll loops never renewed the host turn or paid maintenance, so loads never terminated. A fixture helper `poll_load` now does that, as `settle_document_archive_load` does.
4. The time-travel toy app installed generic store owners without the cold replay authority ("initialization requires its original installed native semantic producer"). It now uses `cold_replay_document_store_owners`.

## Changed files
Under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`:
- `🧷️companions/🍉️tests-06/📦️packages/🦀️rust/Cargo.toml`: adds instance-laws dependency.
- `🧷️companions/🍉️tests-06/🦀️.rs`: adds `recursion_limit`.
- `🧷️companions/🫑️tests-07-3/📦️packages/🦀️rust/Cargo.toml`: feature on tests-06, `semio-framework-job`.
- `🧪️tests/🔬️tool-run/🦀️.rs`, `🌱️operation/🦀️.rs`, `🌱️operation/🧪️tests/🦀️.rs`, `🚦️unit/🦀️.rs`, `📬️outcome/🦀️.rs`, `📬️native-ledger/🦀️.rs`: effect-based close, new job step protocol.
- `🧪️tests/🔬️tool-run/🧸️member-job/🦀️.rs` (new): the toy member job, moved out of `🧪️tool-run-member/🦀️.rs` (which loses that region).
- `🧪️tests/🧪️time-travel/🦀️.rs`: caller-based `verb`, `render_*`, `publish_as`, `act`; cold replay owners.
- `🧪️tests/🧪️folder-reload-route/🦀️.rs`, `🧪️tests/🧪️gesture/🦀️.rs`: ported to the caller context; `poll_load` helper.
- `🧪️testing/🧪️artifact-app-laws/🦀️.rs`: re-exports `admit_document_archive_merge`.
- `♻️cancellation/🎟️issuer/🦀️.rs`: registry-exact admission quote (the root-cause fix).

All temporary `[DEBUG]` code and the temporary trace dependency were removed.
Scripts: `pt06-cargo.sh` (gated, private build and target dirs).
Not touched: `unit/🧪️tests/🦀️.rs` (mounted by part-1) still uses the old job step API and needs its own port.
