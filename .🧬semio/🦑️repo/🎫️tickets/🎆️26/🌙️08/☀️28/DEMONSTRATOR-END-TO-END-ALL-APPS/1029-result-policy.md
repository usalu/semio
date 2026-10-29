# 1029 result: policy

STATUS (05:20): YELLOW. All planned edits are applied (compile-atomic, scripted with strict asserts). Native lib check of the framework half is GREEN (`policy4-fw.rc` 0: instance-construction, instance-app, plugin facade, os-kernel plus 17 crates verified in earlier runs). The energy half (11 mut-* wiring modules, native-lifecycle/singletons, derive consumers, model editor/viewer, hub-energy) and the ifc half could NOT be compiled: they sit behind foreign errors that are not in any file this lane touched (see "Blockers"). wasm32-wasip2 lib check of the framework half is GREEN too (`C/🗑️generated/1029/policy5-wasm.{txt,rc}`, rc 0: instance-construction, instance-initialization, plugin facade, os-kernel, plugin-job, time-travel, instance-stores, instance-history, retained-command). Kept logs: `policy4-fw.txt` (native), `policy5-wasm.txt`; all other `policy*` logs were deleted.

## Blockers for verification (foreign, not mine)

| Crate | Error | Owner |
|---|---|---|
| `semio-s-artifact-energy-model-diff` | `protocol::mutation` unresolved: energy crates alias `protocol` to `semio-framework-os-kernel`, which has no root `mutation` module (`🧬️original/🦀️.rs:5`); `EnergyModelDiff` follows. All energy mut-* crates depend on it | energy |
| `semio-s-artifact-stdio-semio-object-snapshot` (4), `-presentation-snapshot` (10) | `ArtifactChild<SemioValueSnapshot>: NativeReceive<dyn NativeValueSource>` unsatisfied, `PlaceholderKind: DslField` | neural / childstore / stdio-semio |
| `semio-s-artifact-energy-model-session` (8), `-editor-interaction` (15) | `EnergyModelState` fields renamed, `PagedListProgress.released_bytes`, `os_kernel::native_snapshot` import, private `publication_entry` | energy |
| `semio-s-artifact-stdio-ifc-2x3-base` / `-2x3-base-model` | source-boundary gate refuses before compile (96 files > 32, bytes limit); every ifc crate (also `-4`, `-4x3`, composition, catalog) depends on it. Pre-existing, limits NOT raised | splitfix |
| hub lane `wasm-encoder`, first wasm run | `No space left on device` mid-build (twice, 04:40 and 05:00; 124 GiB free afterwards, so the fleet transiently filled the data volume); the wasm framework lane was simply re-run and went green | infra |

## 1 Verification of the 1028 policyfix

| Policyfix row | Status |
|---|---|
| store `checkpoint-order.rs` (2 unwraps), `checkout.rs` | fixed + verified (engine-replay lib compiles; its 5 unused imports removed) |
| plugin `📥️pack` (8), job-receiving | fixed + verified (`plugin-job-receiving` compiles) |
| plugin `🏭️factory` (8), `🧺️sources` (2) | fixed + verified (`plugin-instance-construction` lib, policy4-fw) |
| plugin `🧾️services` (1) | fixed + verified (`plugin-instance-initialization`) |
| store alias removal `original_history_branch/transitions` | fixed + verified (os-kernel lib, engine-branch) |
| ifc composition `register()` deletion + 2 allows, hub catalog projection/definition 4 allows | UNVERIFIED (ifc gate blocker above; hub lane died on ENOSPC). Deleted function has no remaining caller (`rg io_registry::register` finds only other crates' own functions) |
| energy 22 allows (11 mut-* wiring modules), `📊️outputs` (12), `✂️deletion/🧱️surface` (9) | UNVERIFIED (energy blocker) |

## 2 Category 2: alias / `Original*` re-export shims (16 audit rows)

| Row | Item | Status |
|---|---|---|
| 13 | `store/🎮️one-item/🧬️semantic` pub use of `OriginalPostRead, PagedOneItemPost*` | fixed + verified: now private `use`, dropped from the one-item facade; paged-one-item and replay-operation import `semio_framework_value_snapshot_projection::snapshot_projection::post` (replay-operation, one-item(-batch) compile) |
| 14 | `store/🔁️replay` `pub use … as replay_preparation` | fixed + verified: private again (HEAD form); 9 consumers import `semio_framework_os_store_replay_operation`; new deps in engine-command, engine-replay, initialization-replay, plugin-ownership (all compile) |
| 16, 17 | `history/🏁️replay` `pub use failure::…`, `checkpoint_order::…` | not a shim: re-export of the crate's own submodule items (module wiring); kept |
| 18 | `store` `pub use engine_replay::{OriginalCheckpointEditOrder, ArtifactOriginalVcsFailure, original_replay_metadata}` | fixed + verified (os-kernel lib); zero live consumers, two dead test files repointed |
| 19 | `store` `pub use engine_replay as original_replay_publication` | fixed + verified: 6 live consumer files in 5 crates (instance-stores, instance-history, plugin-job, retained-command, time-travel) import `semio_framework_os_store_engine_replay` (dep added, all compile); 12 consumer files that no `mod` wires (dead tests under `🏁️replay/*-tests.rs`, `commit-tests.rs`, child-vcs tests, command/space-member tests) repointed textually, not compilable |
| 20, 21 | `original_history_branch`, `original_history_transitions` | already removed by policyfix, verified |
| 23 | `tool-machine` `pub use … OriginalToolOverlayFold` | fixed + verified (plugin facade compiles; zero consumers) |
| 24 | `plugin` `pub use … OriginalVcsApp` | facade side verified (plugin lib compiles); consumer `hub/🧩️compositions/🔋️energy/🦀️.rs` now imports `semio_framework_plugin_instance_construction` (dep added) UNVERIFIED (energy/hub blocker) |
| 25 | `plugin` `pub use … OriginalSingleWindowRegistration` | facade side verified; consumers energy model editor + viewer initialization now use `semio_framework_plugin_instance_initialization::registration::…` (deps added) UNVERIFIED (energy blocker) |
| 27 | `wp2-declaration` `pub use decl_staged_form as staged_form` | facade side verified (plugin lib); 16 ifc consumer files repointed to `semio_framework_plugin_decl_staged_form` (deps added to stdio-ifc-4, stdio-ifc-4x3) UNVERIFIED (ifc gate blocker) |
| 28 | `wp2-declaration` `tool_contract` re-export of 8 decl_tool_contract names | SKIPPED: SDK facade `semio_framework_plugin::tool_contract`, 100+ consumer files in 80+ plugin crates; a mechanical sweep of that size needs its own wave and per-crate dependency edges |
| 77 | energy singletons `pub use transform::{OriginalSingleTransform,…}` | not a shim: own `#[path]` submodule item; kept |
| 116, 117 | instance-construction `pub use original::…`, `sources::…` | not shims: own submodule items; kept |

## 3 Category 3: `#[allow]` (28 rows)

All 28 are gone (22 energy wiring + 6 ifc/hub removed by policyfix). Re-scan of the current tree vs HEAD at 02:40 and 04:30 (tracked diff and untracked `.rs`): no `#[allow]`/`#![allow]` added anywhere. The 30-line allow block of `os-kernel/🦀️.rs` is HEAD content, outside this audit. Compiler confirmation that no lint surfaces in the 17 wiring/composition files after removal: UNVERIFIED (energy/ifc blockers); the framework crates edited in this lane show no new warning class.

## 4 Category 4: `unwrap` / `expect` in non-test code

| Rows | File | Status |
|---|---|---|
| 1, 2 | energy derive `✂️delete/📆️annual` | fixed (5 sites) via `RowBirthSpec::finish -> Result<Row, ValueError>`; derive crate compiles, expansion UNVERIFIED |
| 3 | derive `✂️delete/🦀️.rs` `types.pop().unwrap()` | fixed (syn error); derive crate compiles |
| 4 | derive `🧱️surface` | fixed (expansion UNVERIFIED) |
| 5, 6 | derive `🪟️fenestration` | fixed (4 sites; expansion UNVERIFIED) |
| 7, 8 | derive `🌱️birth` | fixed (all 13 `finish` impls return `Result`, 4 expects gone; expansion UNVERIFIED). `Span::unwrap()` at the end of `expand` is `proc_macro::Span`, not an `Option` unwrap: kept |
| 9 | value derive `🛬️receive` | fixed + verified (`RetainedCloneRef::try_project`; value-native-owned compiles) |
| 10 | value `owned/🌳️value` `[T;N]::finish` `expect` | SKIPPED: no stable fallible array constructor; the invariant is guaranteed by `storage_demand` (count == N) and the unique in-range `push`; the alternatives (Vec-backed storage, unsafe) change the allocation accounting. Needs a design decision |
| 11, 12 | value `owned/🌳️value` `S::child(..).unwrap()`, `owned/🗂️map` | fixed + verified (`try_project`) |
| 15 | store `checkout.rs` | fixed + verified (line 48 plus 4 adjacent sites in `close_step`; one `assert!` became a typed refusal) |
| 26 | `media-transfer` | fixed + verified (5 sites; instance-app compiles) |
| 68, 69, 78, 79 | energy `✂️deletion/🧱️surface`, `↩️inverse` | fixed with `try_project` (UNVERIFIED, energy blocker) |
| 70 – 76 | energy `📊️outputs` | fixed by policyfix (0 remain) (UNVERIFIED) |
| 88 – 94 | ifc composition | fixed by policyfix (UNVERIFIED) |
| 99, 100, 101 – 111, 118, 119 | checkpoint-order, `📥️pack`, `🏭️factory`, `🧺️sources`, `🧾️services` | fixed by policyfix + verified |
| 112, 113 | `📦️original` `delegate_ref/delegate_mut` `expect` | SKIPPED: the `delegate` dispatch macro fixes `fn delegate_ref(&self)->&Self::Target`; a borrow cannot carry an error; changing it is a macro-contract change for every delegating trait |
| 114, 115 | `📦️original` close ladder | fixed + verified (`ok_or_else(Self::closed)?`) |
| 22 | `staged-source-tests.rs` | test code (scanner false positive); kept |

Other free fixes: duplicate `#[cfg(test)] use ColdMutation/MutationDiff` in `store/⚙️engine/📥️lifted/🏪️store/🦀️.rs` (clashed with a peer's unconditional import, broke `os-store-engine` lib test) removed.

## Pre-existing, observed, untouched

- `--all-targets` surfaces unrelated test-target breakage: value-native-owned lib test (176 errors, missing dev-deps / `extern crate self` aliases), value-clone-source lib test (21), replay-operation lib test (31), os-kernel test `sqlite_snapshot_native_admission` (13, PagedBytes API), plugin-instance-stores lib test (duplicate `ArtifactStore` import in `🧵️instance/📥️staging/📨️dispatch/🦀️.rs`).
- `semio-framework-plugin-instance-app` fails under `--all-targets` (feature `artifact-app-testing`): `drive_framework_reserved_spawn_like_host` still calls `settle_framework_reserved_commit` with 3 of 4 arguments (dispatch lane's in-flight `cx` threading); plain lib check is green.

## SHARED CHANGE

- `semio-framework-value-clone-source` (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs`): new additive `RetainedCloneRef::try_project`.
- energy `native-mutations-lifecycle`: `RowBirthSpec::finish` and `CopyRowField::finish` now return `Result<_, ValueError>`; all 15 impls updated (derive output, two unit-test impls).
- `Cargo.toml` path-dependency edges added: engine-command, engine-replay, initialization-replay, plugin-ownership -> `semio-framework-os-store-replay-operation`; instance-stores, instance-history, plugin-job, retained-command, time-travel -> `semio-framework-os-store-engine-replay`; stdio-ifc-4, stdio-ifc-4x3 -> `semio-framework-plugin-decl-staged-form`; hub-energy -> `semio-framework-plugin-instance-construction`; energy model editor + viewer -> `semio-framework-plugin-instance-initialization`. Cargo.lock edges were left to cargo; coordinator: `repo dependencies lock`.

## Next steps

1. When the energy lane compiles again (diff `protocol::mutation`, stdio-semio snapshots), run `C/1029-tools/policy/run4.sh` (energy, hub, etests lanes) to verify the 11 wiring modules, `RowBirthSpec::finish` and the three consumer migrations.
2. When `2x3-base` is split, run the ifc lane of `run4.sh` (`-p semio-s-artifact-stdio-ifc-4 [--tests]`).
3. Remaining policy debt (skipped rows 10, 28, 112, 113) needs decisions, see above.

## Tools

`C/1029-tools/policy/`: `alias_batch.py`, `batch_b.py`, `energy_edit.py`, `edit.py` (transactional edit helper with dependency insertion and cycle check), `owners.py`, `crateof.py`, `run.sh`, `run2.sh` … `run5.sh`.
