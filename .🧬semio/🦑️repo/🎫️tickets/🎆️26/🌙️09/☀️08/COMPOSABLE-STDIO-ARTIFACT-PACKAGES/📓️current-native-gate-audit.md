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
