# Snapshot IO Drift Sweep (2026-11-24)

Tree edited: `/Users/ueli/.sinc/play` (`P` = `/Users/ueli/.sinc/play/✏️s/🔌️plugins`, `H` = `/Users/ueli/.sinc/play/🌎️hub`).

**Nothing was compiled.** No cargo or rustc was run. Every finding comes from reading the snapshot's framework sources and the plugin code. The release build is the only verifier.

## Method

- Crate membership: I parsed all 3405 `Cargo.toml` `[lib] path` roots under `P` and `H/🧩️compositions` and resolved their `#[path] mod` trees. That gives 34,296 files. All orphan checks are crate-aware (they look at the crate that mounts the file, not at the folder).
- Compile evidence: I read the newest release jsonl (`.cache/b/r/m/components/runs/wGjp1Sg/l/cargo-release-00-b0.jsonl`, read-only). It lists 1808 crates that compiled. A candidate inside a crate that already compiled was treated as a false positive.
- Live repo: I diffed every file before editing it. Where live already had a fix, I ported it byte-identically.

## A. sqlite snapshot native codec signature

- **Contract** (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📥️lifted/🚪️io/🦀️.rs`, `ArtifactSqliteSnapshotHost`):
  - `decode_sqlite_snapshot_native(payload, control, native_owner, output:&mut Option<SnapshotReceivingOutput<Self>>) -> SnapshotReceivingOutcome`
  - `encode_sqlite_snapshot_native(&self, encoding, control, native_owner, output:&mut Option<SnapshotReceivingOutput<IoPayload>>) -> SnapshotReceivingOutcome`
  - Helpers `decode_sqlite_snapshot_record_native` / `encode_sqlite_snapshot_record_native` take `&mut impl FnMut` constructors (decode: 3 arguments) and a trailing `output`.
- **Candidates:** 130 files / 120 host types implementing the methods, 201 record-helper call sites, and every non-trait function that takes `SqliteSnapshotControl` plus `NativeSnapshot*Owner`.
- **Violations:** 2.
  - Writer: both methods had the old form. The decode closure also had 4 arguments (`_body`). I ported it to the lowpoly model: pre-checks return `NotStarted`, and the post-decode `admit_values` returns a `Settlement` failure. Live is identical to the broken snapshot, so there was nothing to port.
  - Space (`s.space`): decode had the old form. Live already had the fix, so I ported it byte-identically. The live fix drops the post-decode `admit_sqlite_values` call; lowpoly keeps the equivalent check as a `Settlement`.
- **Clean:** all other hosts, including the multi-line declarations in trinity jack/rewriting, note and draw. All other delegating helpers (equation, procedural generation, stdio semio) already use the `output` form.
- **Changed:**
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️native/🦀️.rs`
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (now identical to live)
- **Not touched:** the writer unit tests (`🧪️tests/🦀️.rs`) still use the old call shape. They were already stale and are not part of the release build.

## B. RetireOwned on snapshot hosts, orphan retirement, send_only

- **Contract:**
  - `ArtifactSqliteSnapshotHost: ArtifactSqliteSnapshot + RetireOwned`.
  - `derive(RetireOwned)` requires every field to be `RetireOwned`. The value crate implements it for `Vec`, `Option`, `Box`, maps/sets, tuples, arrays, `String` and so on, but not for `Arc`/`Rc`.
  - `ArtifactChild<S>` has no bound on `S`.
- **Candidates:**
  - 120 host types: all derive or implement `RetireOwned`.
  - Recursive nested-field check: 1818 local type definitions reached from the hosts. 1 hit, a false positive: the `LayoutBackgroundDrawing` marker inside `ArtifactChild<_>`.
  - 295 `impl RetireOwned for` / `artifact_retire_struct!` / `artifact_retire_leaf!` / `artifact_retirement_sequence!` sites, checked crate-aware: 0 orphans. 19 types are macro-generated in the same crate.
  - The 10 `♻️retirement` modules all sit in the crate that owns their types.
  - `send_only`: 3 `derive(RetireOwned)` types hold a `BudgetedOwnedEntry`, and all 3 already carry `#[value(send_only)]` (flow brep).
- **Violations fixed:** 0.
- **Unresolved** (same `RetireOwned` family, but no model fix exists and the framework has no `Arc` impl; byte-identical in live):
  - `P/💠️lowpoly/…/✏️editor/🖌️session/🦀️.rs`: `LowpolyTransientState.mesh_workspace: Arc<BTreeMap<…>>` and `LowpolyTransient.state: Arc<…>` under `derive(RetireOwned)`. This matches the earlier "lowpoly view/viewer `Arc<…>: RetireOwned`" observation.
  - `P/🏙️bim/…/🧬️schema/💡️inferences/🕸️model-graph/🦀️.rs:402`: `enum Data` with `Arc<…>` variants under `derive(RetireOwned)`.
  - Possible fixes: `ArcLease<T>`, a manual impl, or dropping the derive. Each is a design decision, so I left them for the coordinator.

## C. Impl-generating macros in non-owning crates; MutationDiff split

- **Contract:**
  - `diff_binary!(T)` expands to `impl ::semio_os_kernel::DiffBinary for T`, which needs `::semio_os_kernel`.
  - `diff_text!(T)` expands to `impl ::semio_os_kernel::DiffText for T`, which needs `::semio_framework_dsl_record` and `::semio_framework_diagnostic`.
  - Both must be mounted in the crate that defines `T`.
  - `MutationDiff` keeps only `retire_cold` and `retire_projection`. `DiffAlgebra` has `is_empty`. `ColdDiffAlgebra` has `inverse`. `ColdMutationDiff` has `absorb` and `apply`.
- **Candidates:**
  - 102 diff-macro sites (40 `diff_binary!`, 40 `diff_text!`, and 22 record-generating `config_diff!`, `sparse_record_diff!` and similar, which define their own types). 16 sites in 6 crates were orphans.
  - A general crate-aware scan of 1500 type-taking macro invocations found no further orphans. The remaining hits implement file-local traits (architect `register_sql!`/`enum_sql!`), define their own types (flow brep `geo_operation!`) or are expressions (bim `set!`).
  - 462 `MutationDiff`/`DiffAlgebra` impls: 0 define `apply`/`absorb`/`inverse`. The 4 `.inverse(` callers in non-Cold files are mutation inverses, not diffs.
- **Fixed** (model: imperative/live space-home; mount `io::{binary,text}::diff` in the crate that owns the diff, re-export from the old crate so consumer paths stay the same):
  - space-home operations (`SHomeDiff` is owned by space-home-inferences). Live had the fix, ported byte-identically:
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/🦀️.rs`
  - architect-program-catalog (`ProgramDiff` is owned by architect-program-inferences, which already depends on kernel, dsl-record and diagnostic):
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs`
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗂️catalog/📦️packages/🦀️rust/🦀️.rs`
  - vcs-vcs-tests (`VcsDiff` is owned by vcs-vcs-inferences, which has the same dependencies; the tests crate is a normal dependency in `H/Cargo.lock`):
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`
- **Unresolved** (needs a new crate dependency inside `✏️s`):
  - block-2d/3d/5d io. `P/🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/…/🚪️io/{💾️binary,📝️text}/🔺️diff/🦀️.rs` run `diff_binary!`/`diff_text!` for `semio_s_artifact_block_Nd_diff::BlockNdDiff` inside `semio-s-artifact-block-Nd-io`. That is E0117 x2 per crate, not yet reached by the compiler. Live is identical.
  - Required fix:
    1. Add `semio-framework-diagnostic` to each `semio-s-artifact-block-Nd-diff` Cargo.toml, and to the root `Cargo.lock`.
    2. Mount both io diff files in the diff crate root (`…/🧬️schema/🔺️diff/🦀️.rs`, for example as `pub mod io { pub mod binary { #[path] pub mod diff; } pub mod text { … } }`).
    3. Replace the two `#[path] pub mod diff;` lines in `…/🚪️io/💾️binary/🦀️.rs` and `…/🚪️io/📝️text/🦀️.rs` with `pub use semio_s_artifact_block_Nd_diff::…::diff;`.
  - I did not do a binary-only partial move: the crate would still fail on `diff_text`.

## D. Path drift

- **`crate::snapshot::` → `crate::sqlite_snapshot`:** I resolved crate-root modules for every crate whose files use `crate::snapshot::`. 9 hits, all false positives: the module comes from a `pub use …::snapshot;` in a mirror, or the crate already compiled. No violations.
- **Local `pack` shadowing the extern `pack`:** only 1 file in the tree declares `mod pack` and uses `pack::…`: forms `🚪️io/💾️binary/📸️snapshot/🦀️.rs`. Line 27 was already fixed.
  - Risk, not edited: line 455 `use pack::snapshot::ArtifactPack;` inside `forms_genesis_child_pack`. The run that reported line 27 did not report it, so I left it alone.
  - If the compiler flags it next, the fix is `use semio_os_kernel::os_store::ArtifactPack;`, the path line 16 of the same file uses.

## E. region_boolean/offset_regions arity; non-Clone iterators

- **Contract:**
  - Both functions take a fifth argument, `turns:&mut impl EffectTurns`.
  - `RetainedOrderedMap::{iter,keys,values}` return `impl Iterator`, which is not `Clone`.
- **Calls:** 9 call sites in the whole plugin and hub crate set, all in bim and all already 5-argument. No other plugin calls them.
- **Iterator clones:** I checked 27 let-bound iterator bindings that are later `.clone()`d.
  - 1 violation: bim `next_view` (`snapshot.views: RetainedOrderedMap<String, View>`, `drawable.clone()`). Fixed the same way as diagnostics: collect into a `Vec` once, take `first()`, then `into_iter().find(..)`. Live is identical to the broken snapshot.
  - The other 26 are over `Vec`s or slices with `Clone` closures.
  - Changed: `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/📄️sheets/🦀️.rs`

## F. Hub compositions: dyn_enum_close! without semio-framework-value

- **Contract:** the expansion names `semio_framework_value::…`, so the crate needs that dependency (or a `use … as semio_framework_value` alias, as layout has).
- **Candidates:** 35 compositions use the macro; 11 lack the dependency.
  - Layout has `use semio_plugin::{…, __value as semio_framework_value}` and already compiled, so I left it alone.
  - 10 fixed: added `semio-framework-value.workspace = true` after the `semio-framework-dispatch-macros` line, as in the animate model.
  - Shooting: live already had a path dependency (`semio-framework-value = { path = "../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust" }`), so I ported that file byte-identically.
- **Changed manifests** (`/Users/ueli/.sinc/play/🌎️hub/🧩️compositions/<X>/📦️packages/🦀️rust/Cargo.toml`), where `<X>` is: `✒️writer`, `➗️mathematical`, `🎥️shooting`, `🎬️sequence`, `🏙️bim`, `🏛️architect`, `💠️lowpoly`, `📋️forms`, `📜️imperative`, `🧱️block`.
- **`--locked` lockfile:** `/Users/ueli/.sinc/play/🌎️hub/Cargo.lock` gained `"semio-framework-value",` in alphabetical position in the `dependencies = [...]` list of:
  - `semio-hub-writer`, `semio-hub-mathematical`, `semio-hub-shooting`, `semio-hub-sequence`, `semio-hub-bim`
  - `semio-hub-architect`, `semio-hub-lowpoly`, `semio-hub-forms`, `semio-hub-imperative`, `semio-hub-block`
  - That is 10 lines added and nothing else changed. The script asserted that each list was sorted and lacked the entry before inserting.

## Unresolved summary

1. Block 2d/3d/5d io `diff_binary!`/`diff_text!` orphans. These need `semio-framework-diagnostic` on the block diff crates plus a root `Cargo.lock` update (C).
2. `Arc<…>` fields under `derive(RetireOwned)` in the lowpoly editor session transient and the bim model-graph `Data` (B).
3. Risk: forms binary snapshot line 455 `use pack::snapshot::ArtifactPack` (D).
4. Limits of the sweep:
   - Files not reachable from any lib root (tests, bins) were not checked.
   - The nested `RetireOwned` check covers only plugin-defined field types, plus a scan for `Arc`/`Rc`/`Cell`/`Mutex`.

## Block io orphan move

Resolves "Unresolved summary" item 1 for block 2d, 3d and 5d. Nothing was compiled.

- **Claim verified:**
  - `BlockNdDiff` is defined in `semio-s-artifact-block-Nd-diff` (crate root `…/🧬️schema/🔺️diff/🦀️.rs`).
  - `diff_binary!`/`diff_text!` (proc macros in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🏗️document-diff-and-composite-expansion/🦀️.rs`) expand to `impl ::semio_os_kernel::DiffBinary/DiffText for T`. Their only path roots are `::semio_os_kernel`, `::semio_framework_dsl_record` and `::semio_framework_diagnostic`, plus `T::__dsl_spec/__dsl_to_record/__dsl_from_record` from the `DslRecord` derive.
  - Both macros were invoked only in `semio-s-artifact-block-Nd-io`, so both impls are orphans (E0117).
  - The diff crates already had kernel and dsl-record. They lacked diagnostic, which every artifact workspace already declares in `[workspace.dependencies]`.
- **Move** (same model as imperative procedure):
  - Each diff crate root gains `//#region 🪢️Mounts` with `pub mod io { pub mod binary { pub mod diff } pub mod text { pub mod diff } }`. The inline modules use `#[path = "."]`, and the leaf paths are `../../🚪️io/{💾️binary,📝️text}/🔺️diff/🦀️.rs`.
  - Each leaf's macro argument changed from `semio_s_artifact_block_Nd_diff::BlockNdDiff` to `crate::BlockNdDiff`.
  - The io crate's `binary/🦀️.rs` and `text/🦀️.rs` replace `#[path] pub mod diff;` with `pub use semio_s_artifact_block_Nd_diff::io::{binary,text}::diff;`.
  - Dependents checked:
    - The only users of the old paths are each io crate's own `crate::text::diff::COMPONENT_GRAMMAR_{SEMIO,PATH}`, which resolve through the re-export.
    - Nothing in `✏️s` or `🌎️hub` names `semio_s_artifact_block_Nd_io::{binary,text}::diff`.
    - No other crate mounts the leaf files.
- **Value change:** `COMPONENT_*_PATH` (`module_path!()`) now reads `semio_s_artifact_block_Nd_diff::io::…::diff`. This also happened in the imperative model. The framework only compares `grammar_path` values for equality.
- **Files changed:** each of the following, for `<D>` ∈ {`◻️2d`, `🧊️3d`, `🖐️5d`}, under `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🧱️block/🗿️artifacts/<D>/🏅️standards/🔖️1/🪆️subsets/✳️any/`:
  - `🧬️schema/🔺️diff/🦀️.rs`
  - `🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` (adds `semio-framework-diagnostic = { workspace = true }`)
  - `🚪️io/💾️binary/🦀️.rs`
  - `🚪️io/📝️text/🦀️.rs`
  - `🚪️io/💾️binary/🔺️diff/🦀️.rs`
  - `🚪️io/📝️text/🔺️diff/🦀️.rs`
- **Lockfiles:** `"semio-framework-diagnostic",` was inserted in sorted position, first in the list before `"semio-framework-dsl-record"`. Every lockfile uses the plain single-version form.
  - Lockfiles that contain all three `semio-s-artifact-block-{2d,3d,5d}-diff` packages:
    - `/Users/ueli/.sinc/play/🌎️hub/Cargo.lock`
    - `/Users/ueli/.sinc/play/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock`
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.lock`
  - Lockfiles that contain only their own diff package:
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/Cargo.lock`
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/Cargo.lock`
    - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/Cargo.lock`
  - That is 12 lines in total. The root `Cargo.lock` and `✏️s/Cargo.lock` do not contain these packages, so they are unchanged. The search covered every `Cargo.lock` in the tree outside `.cache`, `node_modules` and `target`.
- **Cycles:** none. `-io` depends on `-diff`, and `-diff` depends on neither `-io` nor anything downstream of it.
- **Unresolved:** none for this item. Not compiled. The next release build is the verifier.
