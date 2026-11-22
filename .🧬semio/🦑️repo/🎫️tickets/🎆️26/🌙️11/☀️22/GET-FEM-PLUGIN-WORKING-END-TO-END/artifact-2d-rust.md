# FEM 2D artifact Rust workspace: check and test status (PARTIAL, not green)

Workspace: `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\Cargo.toml`

## What ran
- `repo cargo --store=shared --configuration=prototype -- check --all-targets`: 4 attempts (logs `artifact-2d-rust-check1/2/4.log`). Outcomes: C: full (capacity door waited 10+ min), repeated `EBUSY` on `host-capacity/grants/*.json` (retry script `artifact-2d-rust-retry.sh` added), then `cargo:build-entry` (workspace `synchronize --manifest`) stuck 45+ min. Aborted per coordinator.
- Native fallback (plain `cargo` is refused by the compiler ingress; the working form is `CARGO_TARGET_DIR=G:\t\2d CARGO_BUILD_BUILD_DIR=G:\t\2d-b cargo --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' check --all-targets --message-format short --keep-going`). Run 1 (`artifact-2d-rust-native-check.log`) completed with `--keep-going`: all libs and bins compile; only test targets fail. Run 1 had its build-dir on C: so editor_boot / editor_app_2 / editor_app showed silent "could not compile" = os error 112 (disk full). Notably `editor_app` itself had NO source errors, only the disk-full write failure: the 163 old errors are already ported by the earlier ticket.
- Run 2 (build-dir on G:) was killed by me while the machine was saturated by other agents' builds (progress slow, no errors reported yet). `cargo test` was NOT run because check is not green.
- Nothing left running from my side.

## Fixed (files)
- `🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/cold-tests.rs` and `original-receiving-tests.rs`: `snapshot.nodes.get(i)` is the sealed wrapper accessor (0 args); now `.get().get(i)`. Not re-verified by compiler.
- `🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:35`: `zip(&snapshot.nodes)` -> `zip(snapshot.nodes.get().iter())`. Not re-verified.
- `Cargo.lock` of the workspace was refreshed by cargo (one added package, `semio-plugin-instance-natural-publication`).

## Still failing (test targets only; first-error inventory from run 1)
1. `semio-s-artifact-fem-2d-snapshot-tests` lib test (10): `🧪️tests` crate imports removed `NativeDecodeWorkRecipient` / `NativeEncodeWorkRecipient` and the `RetireOwned` derive (`identity-index-tests.rs`, `native-tests.rs`, `write-pack-tests.rs`, `loads/receiver-tests.rs`). The framework native decode/encode control API was reworked (`NativeDecodeControl::with_work_recipient` etc.); these fixtures need a real port.
2. `semio-s-artifact-fem-2d` lib test (2): `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` uses missing crate `semio_plugin_reactor_app` and builds `Fem2dSnapshot` fields directly (must build `Fem2dSnapshotValue` then `Fem2dSnapshot::from_reference`; `every_word` and mutations must edit the value type).
3. `semio-s-artifact-fem-2d-editor-interaction` lib test (28) and `semio-s-artifact-fem-2d-editor` lib test (8, same files): `🕹️interaction/🧪️tests/🔬️unit/🦀️.rs` and `🧭️gumball/🧪️tests/🔬️unit/🦀️.rs` mutate `Fem2dSnapshot` in place, use `SealedShared<String>` ids as `&str`, `model_window::SELECTION_COLOR_2D`/`HOVER_COLOR_2D` (gone), `editor::fem2d::{panels,terminology}` paths (moved to their own crates), `validate_interaction_identities_controlled`, `ComponentTree.native_resources`, `RetainedCloneGrant` vs `&mut RetainedAllocationEffect`. Whole files are pre-persistent-snapshot and need rewriting.
4. `editor/🧪️tests/🔬️fem2d-artifact-preparation-laws/🦀️.rs`: `Fem2dArtifactPreparation` gained `last_work`; step/poll calls now take `&mut RetainedAllocationEffect` and return `RetainedWorkPoll`.
5. `semio-s-artifact-fem-2d-results-state` lib test (2): `🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:110` `close_step` now takes `(grant)` -> `RetainedWorkPoll` (no `.expect`).
6. `editor_boot`, `editor_app_2`, `editor_app`: unknown until re-run with build-dir on G: (previous failures were disk full).

## Root cause notes
- Most remaining failures are fixtures lagging the framework value/retirement/native-decoding rework (other agents are still editing `🧰️framework/🔨️modules/🌱️value`), so fixing them now chases a moving API.
- `C:` hit 0 bytes free repeatedly; `.cargo/config.toml` puts build-dir on `C:\semio\.cache`. Always set `CARGO_BUILD_BUILD_DIR` to G: for native runs.
- Machine is saturated (a single `check --all-targets` of this workspace takes 30+ min when other agents compile).

## Next
Re-run the native check with `-p <crate> --tests` per crate in the order above, port items 1-5, then `cargo test` (same env). No tests have been executed yet.
