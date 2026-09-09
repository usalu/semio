# Current Native Gate Audit

## Scope And Method

Read-only recovery of the executor-owned native acceptance evidence. This audit read `📓️nx-execution.md`, `📓️nx-owned-file-ledger.md`, `📓️acceptance-contract.md`, `📓️results.md`, and the raw receipts in `🗑️generated`. It did not start a native build, edit implementation, alter Git state, delete caches, or terminate a process.

`0` below means the named raw command completed successfully. It does **not** prove a later source revision until that exact gate is rerun. `101` means the prior command failed; the listed first diagnostic is evidence classification, not a current-source verdict. “Missing” means there is no later terminal receipt for the acceptance gate.

The supplied initial no-process premise was true before fresh work resumed. During this audit a read-only process check found new concurrent work: PID `21638` is the root PDF Nx build, and PID `21703` belongs to `bun ./📜️script.ts reset-document-ownership` running the Reasoning/Trinity Cargo test from the repository root. This audit did not interfere.

## Acceptance Status

| Required scope | Exact selector | Terminal evidence | Status and first diagnostic |
| --- | --- | --- | --- |
| 23 standalone default libraries | `cargo check -p <package> --lib` | `🗑️generated/owned-23-standalone-default-current.tsv` and `.txt` | **4/23 accepted:** `semio-s-artifact-animate-presentation`, `-demonstrator-playground`, `-sequence-sequence`, `-architect-program`. **19/23 require rerun.** Mathematical first failed on an obsolete `editor/config/🦀️.rs` mount; Flow then hit `ENOSPC`; Shooting first hit shared `semio-framework-plugin` `E0382`; the remaining lane records shared OS/store `E0425`/`E0422` failures. The statuses are not a broad source verdict. |
| Writer default, separate from the 23-leaf matrix | `cargo check -p semio-s-artifact-writer-writer --lib` | `🗑️generated/owned-focused-default-retries-current.tsv`, `.txt` | **Accepted:** `0`, 7m57s. The paired Mathematical entry was `ENOSPC`, so the enclosing command’s nonzero exit does not invalidate Writer’s completed leaf. |
| 24 parent composition defaults | `cargo check -p <parent> --lib` | `🗑️generated/owned-24-parent-composition-current.tsv`, `.txt` | **4/24 accepted:** `semio-s-plugin-flow`, `-animate`, `-shooting`, `-sequence`. **20/24 require rerun.** First distinct diagnostics: Writer stopped at Trinity Rewriting missing `protocol`/`semio_framework_graph`; Mathematical was `ENOSPC`; VCS saw the shared plugin `E0382`; Demonstrator had manifest imports to private/missing `editor`/`viewer`; the remaining parents stopped in the older shared OS/store errors. |
| Flow’s 13 exact native laws | `bun x nx run @semio-tech/flow-plugin:{child-identity-check,child-edit-check,add-widget-retained-check}` | `🗑️generated/flow13-runtime-current-4.tsv`, `.txt` | **Missing native acceptance.** `child-identity-check` first failed while writing a receipt with `ENOSPC`; `child-edit-check` and `add-widget-retained-check` both ended `signal=SIGABRT` before their named law passed exactly once. Their source/oracle gates are green, but no native law may be counted passed. |
| Draw FSM runtime | `cargo test -p semio-s-plugin-draw-fsm --lib` | `🗑️generated/owned-native-enospc-retries-current.tsv`, `.txt` | **Accepted:** `0`; 26 passed, 0 failed. The older `flow13-draw-fsm-runtime-current-2.txt` was only `ENOSPC`. |
| Draw FSM macros | `cargo test -p semio-s-plugin-draw-fsm-macros --lib` | `🗑️generated/owned-native-enospc-retries-current.tsv`, `.txt` | **Accepted:** `0`; 9 passed, 0 failed. The older red receipt was only `ENOSPC`. |
| Required-nullable value derive | `bun x nx run @semio-tech/value-derive-rs:test --test flatten_with_skip required_option_rejects_omission_and_accepts_explicit_null_like_serde` | `🗑️generated/owned-native-enospc-retries-current.tsv`, `.txt` | **Real red, now repaired by the root lane:** `E0369` at `🧰️framework/🔨️modules/🌱️value/✨️derive/🧪️tests/🪗️flatten-with-skip/🦀️.rs:269`. It compares `Result<RequiredNullableField, serde_json::Error>` with `assert_eq!`; `serde_json::Error` lacks `PartialEq`. The prior `value-derive-required-nullable-nx-3.txt` is only a later `ENOSPC` retry. |
| Animate optional feature | `cargo check -p semio-s-artifact-animate-presentation --features preview-window --lib` | `🗑️generated/owned-leaf-feature-gates-current.tsv`, `.txt` | **Accepted:** `0`, finished in 87m47s. |
| Lowpoly optional feature | `cargo check -p semio-s-artifact-lowpoly-lowpoly --features cad-fixtures --lib` | `🗑️generated/owned-native-enospc-retries-current.tsv`, `.txt` | **Accepted:** `0`, finished in 41m14s. The older feature-matrix `101` stopped after Animate because of disk exhaustion; it is superseded for this feature only. Lowpoly’s default leaf and parent remain unaccepted. |

## Exact Retry Checklist

1. After the shared source lanes settle, preserve the existing ticket target directory and rerun the required-nullable Nx filter above. It is the only confirmed local diagnostic in this set; do not treat the old `ENOSPC` receipt as a second defect.

2. Re-run all three Flow targets without `--oracle-only`, retaining their existing exact law groups in `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts`:
   - `child-identity-check`: nine `semio-s-artifact-flow-flow` laws plus `flow_actual_surface_factories_close_all_owners_under_neutral_grants` from `semio-s-plugin-flow`.
   - `child-edit-check`: `add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content` from `semio-s-artifact-flow-flow`.
   - `add-widget-retained-check`: `retained_add_widget_factory_is_exact_child_only_and_legacy_closed` and `retained_add_widget_dispatches_one_acknowledged_child_group_and_retires` from that same leaf.

3. Re-run the 19 unaccepted standalone packages from the matrix with their default library selector: `semio-s-artifact-mathematical-equation`, `-flow-flow`, `-vcs-vcs`, `-shooting-shooting`, `-process-process3d`, `-lowpoly-lowpoly`, `-reasoning-wires`, `-forms-forms`, `-layout-layout`, `-cad-cad`, `-playbook-playbook`, `-imperative-procedure`, `-remodel-remodeling`, `-energy-model`, `-dag-dag`, `-draw-drawing`, `-raster-raster`, `-note-note`, and `-sourcing-curation`. Do not reuse the prior shared OS/store diagnostics as current failures.

4. Re-run the 20 unaccepted parents with their default library selector: `semio-s-plugin-writer`, `-mathematical`, `-vcs`, `-demonstrator`, `-architect`, `-process`, `-lowpoly`, `-reasoning-mindmap`, `-forms`, `-layout`, `-cad`, `-playbook`, `-imperative`, `-remodel`, `-energy`, `-dag`, `-draw`, `-raster`, `-note`, and `-sourcing`. Process, CAD, and Sourcing use their declared default `plugin-entry`; do not add a feature override. Record a new terminal TSV rather than carrying forward `101` values.

5. Retain the already accepted Draw FSM/macros, Animate `preview-window`, and Lowpoly `cad-fixtures` receipts. They need no repeat unless their inputs change. Lowpoly default and parent are separate gates and remain in step 3/4.

## Current-Source Qualification

The shared `E0382` raw diagnostic is stale at its quoted location: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:811` now calculates `more_work` from `lifecycle_work`; it no longer borrows an `events` value. The old result must therefore be rerun after the GIS-owned shared-plugin change rather than reported as still failing.

The separate bounded source audit is [📓️current-native-source-audit.md](./📓️current-native-source-audit.md).

## Coordinator Execution Follow-Up

Required-nullable now passes the exact ordinary Nx/Nextest filter (1 passed,14 skipped,exit0); see `📓️required-nullable-test-recovery.md`. The coordinator started all39 unaccepted default library checks through their actual Nx check targets, preserving each Cargo manifest owner and `--lib`. The plan maps Cargo package names to verified current Nx project roots from the normal graph (`native-library-recovery-plan.json`). One sequential process writes per-package logs and `native-library-recovery.tsv`; no broad success is claimed before those terminal rows exist. Flow current fixture recovery is separate in `📓️flow-native-fixture-recovery.md`.

## Bounded First-Failure Watch — 2026-09-09T14:08:54+02:00

A five-minute read-only watch stopped early when `🗑️generated/stdio-semio-full-final-recovery-2.txt` became terminal. Nextest had started 2,447 fundamental tests with 62 skipped, then the budget wrapper killed `cargo nextest run` for exceeding 15,000ms. The first terminal diagnostic is at lines 295-301 of that receipt. It names no Rust source location and no failing test.

The matching `🗑️generated/nextest-semio-full-final-recovery-2/semio-nextest-cWZtxI/` directory contains only `binaries-metadata.json`; no test-result artifact was emitted. This is an execution-budget timeout, not evidence for a Semio source repair or for test acceptance. During the watch, `flow-recovery-9-artifacts`, `flow-recovery-9-child-identity-check.txt`, and `native-library-recovery.tsv` did not change. No session, process, source file, or cache was touched.

The bounded receipt is `🗑️generated/native-first-failure-watch-stdio-semio-2026-09-09T140854+0200.json`.

## Semio2 Budget Classification And Retry3

Root reaped ordinary Semio2 with exact exit1. The native test binary compiled successfully, and Nextest began2447 tests under the fundamental profile with62 higher-level tests skipped. The wrapper killed the aggregate after its default15000ms; no complete pass/failure census was emitted. The ordinary target took44m24s including compilation and shared Cargo contention. This is compiler acceptance but not runtime acceptance.

Retry3 uses the same ordinary Nx target, library selection, fundamental profile and no-fail-fast mode with the documented per-invocation SEMIO_TEST_BUDGET_MS=600000 override. It reuses completed private Nx state/cache and shared Cargo; new raw output and Nextest metadata use retry3 names. This changes only the aggregate verification allowance, preserves cancellation and does not change interactive callback/ownership policies or classify unexecuted tests as passing.

## Retained Semio2 Fundamental-Selector Inventory — 2026-09-09T14:17:43+02:00

The retained Semio2 metadata names exactly one compiled library test binary: `semio-s-artifact-stdio-semio` (binary ID and name), package `semio-s-artifact-stdio-semio@0.1.0`, for `aarch64-apple-darwin`. Its binary path is `🗑️generated/cargo/debug/deps/semio_s_artifact_stdio_semio-5282082d70ade0f1`. Metadata carries no testcase list, so this audit ran only that retained executable's `--list`; it did not execute a test or build an artifact. The complete listing is `🗑️generated/semio-full-final-recovery-2-test-list.txt`.

The Semio2 receipt invokes the `fundamental` profile with precisely `--skip quick:: --skip long:: --skip exhaustive::` (raw lines 295-297). The `--list` output contains 61 selector names under those three tier prefixes, but contains all 12 selected recovery laws outside them:

- Object default: `minimal_schema_valid_json_defaults_optional_children` (list line 1720).
- Kit default: `minimal_schema_valid_json_defaults_collection_slots` (1458).
- Drawing codec: `op_binary_roundtrip_law` (770).
- Document fixture: `inverse_law` and `mutation_diff_law` (745 and 747).
- BREP tolerance: diff codec, create/delete vertex, dependent-edge inverse, all-variant mutation inverse, and all three language-neutral tolerance laws (151, 454, 456, 460, 462-464).

Current source declares the Object/Kit snapshot test modules through their `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"]` selectors, the Drawing binary test module through the same selector, and declares the targeted Drawing, Document, and BREP functions as async tests. The scoped selector-source read found no `#[ignore]`, `quick::`, `long::`, or `exhaustive::` marking on those twelve functions. The retained binary therefore includes the requested laws under the profile's ordinary selection; no runtime result is inferred until Retry3 has a terminal receipt.

Raw machine-readable selector evidence is `🗑️generated/semio-fundamental-selected-law-inventory.json`.

## Semio3 Historical-Failure Baseline — 2026-09-09T14:23:48+02:00

The only historical receipt used here is `🗑️generated/stdio-representative-native-after-disk.txt`: it compiled the Semio leaf and ended with 2,306 passed, 196 failed, and one ignored test. It records the following 17 named failures. They are a comparison baseline only; Semio3 must emit the same fully qualified selector and a terminal failure before any of them is classified as present again.

| Cohort | Historically failed selectors | Historical first diagnostic | Current source qualification |
| --- | --- | --- | --- |
| Value named-diff (2) | `absorb_map_associativity`; `absorb_nodes_associativity` | `mutation.apply.invalid-add-index`, raw lines 3015-3024 | Both selectors remain in the retained Semio2 binary list at lines 2371 and 2376. The current `absorb_named` still appends later additions at source line 703. Its exact implementation and unit-test files equal `HEAD`. |
| BREP Boolean (5) | `box_minus_cylinder_bore_exact_volume_and_validates`; `box_union_cylinder_through_exact_volume_and_validates`; `sphere_union_sphere_lens_exact_volume_and_validates`; `tangent_spheres_union_volume_is_exact_sum`; `union_and_intersect_are_commutative_by_volume` | `shell-not-closed` or `non-manifold-edge`, raw lines 150-153 and 217-238 | All five are in Semio2's retained list (135, 136, 143-145). The Boolean implementation and unit test equal `HEAD`. |
| BREP Offset (5) | `draft_box_side_face_matches_trapezoid_magnitude`; `offset_cylinder_matches_closed_form`; `offset_solid_box_round_matches_minkowski_closed_form`; `shell_box_one_open_face_matches_closed_form`; `thicken_planar_face_matches_box_volume` | wrong closed-form magnitude or zero volume, raw lines 225-257 and 3179-3184 | All five are listed (205, 207, 210, 214-215). The Offset implementation and unit test equal `HEAD`. |
| BREP snapshot body (4) | `cylinder_round_trips_through_snapshot`; `foreign_string_ids_mint_fresh_labels`; `sphere_round_trips_through_snapshot`; `torus_round_trips_through_snapshot` | curve-count/topology mismatch; the foreign-ID fixture reports missing pcurves and `shell-not-closed`, raw lines 635-653 | All four are listed (478-479, 482-483). The body implementation and unit test equal `HEAD`. |
| BREP engine (1) | `sphere_torus_cut_produces_preview_mesh` | `imprint point does not lie on face boundary loop`, raw lines 775-777 | It is listed at 281. The engine implementation and unit test equal `HEAD`. |

The current source/test fingerprint snapshot uses repository revision `9b605a4550f8fff55a6a7a699cc70592dccec07b`: all 10 files above are byte-equal to that revision. The current tolerance/schema work differs from `HEAD` in five separate BREP generator, tolerance-fixture, transport, and mutation-unit paths; it does not overlap any of the eight geometry/body/engine source-or-test paths. It also does not overlap the two Value files.

This supports keeping the two Value and 15 geometry/topology names as documented pre-existing cohorts relative to the current tolerance/schema work. It does **not** prove byte stability from the historical execution itself: the old receipt records no Git revision or source hash. Therefore do not suppress a Semio3 failure merely because its name matches this table. Classify a matching Semio3 failure as a historical-name continuation with unproven historical-byte identity; any changed selector, diagnostic, or source provenance is a new observation.

The complete hashes, revision comparison, and tolerance-path separation are in `🗑️generated/semio3-historical-failure-provenance.json`.

## Five-Minute Native First-Result Watch — 2026-09-09T15:00:23+02:00

Read-only watch found no newly terminal result and no first diagnostic. Semio3 (`🗑️generated/stdio-semio-full-final-recovery-3.txt`), PDF27 (`🗑️generated/pdf-native-cache-27-unchanged-local.txt`), and the Binary/TXT lane (`🗑️generated/stdio-binary-txt-native-final.txt`) continued writing their ordinary running-progress lines through the final snapshot. The retained GIS6 receipt, the 39-row matrix TSV, and the available Flow9 source receipt did not change during the window. No test, build, process, or cache action was taken; this is not runtime acceptance evidence.

## Native First-Result Watch — 2026-09-09T15:20:40+02:00

No additional terminal result or first source diagnostic appeared. Semio3, PDF27, and Binary/TXT continued ordinary progress through the final snapshot. The matrix TSV and available Flow9 receipt did not change. GIS6's default 15-second aggregate-budget result was already known before this watch; its Block auto-stage reached the Block Nx target but emitted no compiler or test diagnostic and did not terminate. No command was started, stopped, or altered.

## Writer Parent Compile — 2026-09-09T15:23:36+02:00

`semio-s-plugin-writer` added a terminal `0` row to `🗑️generated/native-library-recovery.tsv`. Its ordinary `@semio-tech/writer-plugin:check --lib` Nx route completed successfully after Cargo finished the dev profile; exact receipt: `🗑️generated/native-library-recovery-semio-s-plugin-writer.txt`. This is a compile-only gate, so it selected and ran no test laws and has no passed/failed test-law counts. The only diagnostics were two non-fatal `unnecessary qualification` warnings in transitive Trinity Jack transient sources (the editor window at line 92 and results window at line 120); no compiler error or Writer assertion appeared.

## Coordinator Queue Continuation — 2026-09-09T12:54:34.885173+00:00

The four root-owned native sessions remain active:39-row library recovery (19 terminal leaf rows; Writer parent queued), PDF unchanged stage27, Flow recovery9, and Semio full recovery3. At this observation there is no native PDF local-restoration, source-isolation, or restored-consumer proof receipt. The GIS owner separately verified intact Cargo/Nextest ancestry and advancing concrete format compilation at12:53:08UTC. No cache or native runtime acceptance is inferred from queued commands.

## Exact Semio Result Comparison Plan

Prepared a machine-readable comparison plan with12 required current regression selectors and17 uniquely resolved historical fully qualified failure names. Every selector resolves exactly once in the retained compiled test list. The plan requires a complete current census and disallows automatic historical-failure exclusions. It is stored as `🗑️generated/semio-current-acceptance-comparison-plan.json`; preparation is not runtime acceptance.

## Writer Parent Recovery Accepted — 2026-09-09T13:23:48.913408+00:00

The ordinary `@semio-tech/writer-plugin:check -- --lib` recovery passed with exact status0, along with all four Nx prerequisites. Native `dev` compilation completed in110m37s; full Nx elapsed111m8s includes shared Cargo waiting. This accepted path also checked current Trinity Jack/Rewriting and Writer artifacts; two unnecessary-qualification warnings do not change the successful compiler status. Raw output: `🗑️generated/native-library-recovery-semio-s-plugin-writer.txt`; exact Nx receipt: `🗑️generated/native-library-writer-parent-accepted-run.json`. The39-row recovery matrix now has20 terminal rows (four accepted, sixteen prior failures awaiting the failed-only rerun) and has advanced to the Mathematical parent. PDF27 acquired the Cargo lock next. This is compile acceptance, not a new native runtime claim.

## PDF 27 Build Terminal — 2026-09-09T15:30:09+02:00

`pdf-native-cache-27-unchanged-local.txt` terminally failed the compile-only `@semio-tech/stdio-pdf-rs:build` target. It selected no test laws, so no passed or failed law count exists. The first and only compiler diagnostic is `E0433` in the framework plugin root source at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7542`: `crate::os_schema_composition::ArtifactCompositionFields` does not resolve. Rustc identifies the available path as `crate::semio_framework_os_kernel::os_schema_composition::ArtifactCompositionFields` and offers importing the module. The failure is in dependency crate `semio-framework-plugin`, before PDF compilation or cache-proof acceptance; Cargo reports one error and one warning, then NX marks only `@semio-tech/stdio-pdf-rs:build` failed after 96m53s. No historical exclusion was applied.

## Ten-Minute First-Result Watch Close — 2026-09-09T15:31:35+02:00

The watch produced two terminal receipts: Writer compile success and PDF 27 compile failure, documented above. No further terminal result or first diagnostic appeared by the deadline. Semio3 and Binary/TXT remained in their running state; Mathematical remained waiting on the Cargo build lock. The original matrix TSV gained no row after Writer.

## Binary Current Native Census — 2026-09-09T13:38:57.536870+00:00

Binary completed its entire Nextest selection:42 run,40 passed,2 failed,0 skipped in0.420s. Failures are absorb_law_cartesian and op_text_binary_roundtrip_law. The latter parser rejects the printer's splice line containing remove-len. The stdio execution lane owns the fixes and retry; the read-only lane independently audits absorption. The enclosing ordinary Nx run-many remains active in TXT, so no final aggregate status is inferred yet. Exact selectors/counts: 🗑️generated/stdio-binary-current-native-census.json; raw output: 🗑️generated/stdio-binary-txt-native-final.txt.

## Flow9 Identity Shared Compiler Failure

The identity route ended in its Cargo build stage with status101 before any native law executed. Its sole compiler error is E0277 at shared plugin validate_parent_child_restore: OwnedDocumentClosureSource::child_projection returns ChildRestoreProjectionError, which cannot be propagated directly into the function's Fault result. The GIS execution lane owns a precise local error translation while preserving child-admission denial/authority behavior; the stdio lane remains focused on Binary. The original Flow9 sequence continues to child-edit/retained routes, and failed identity acceptance will require a current-source rerun. Structured first cause: 🗑️generated/flow-recovery-9-identity-first-cause.json.

## Binary/TXT Final Per-Target Receipt — 2026-09-09T16:44:20+02:00

The terminal ordinary Nx run-many receipt selected Binary and TXT serially. The private Nx run receipt assigns the combined status `1` solely to `@semio-tech/stdio-binary-rs:test` (hash `11937346445160478916`, cache miss): its Nextest census was 42 selected, 40 passed, 2 failed, 0 skipped in 0.420s. The only failures were `absorb_law_cartesian` and `op_text_binary_roundtrip_law`.

`@semio-tech/stdio-txt-rs:test` is independently terminal-successful (hash `14395200475720332396`, cache miss): 54 selected, 54 passed, 0 failed, 0 skipped in 0.702s. The enclosing exit `1` therefore must not be attributed to TXT. The Binary repair retry is live and has not been inspected for a result at this snapshot.

Raw output: `🗑️generated/stdio-binary-txt-native-final.txt`. Nx task receipt: `🗑️generated/nx-cache/norm-stdio-final/run.json`. Normalized census: `🗑️generated/stdio-binary-txt-native-final-census.json`.
