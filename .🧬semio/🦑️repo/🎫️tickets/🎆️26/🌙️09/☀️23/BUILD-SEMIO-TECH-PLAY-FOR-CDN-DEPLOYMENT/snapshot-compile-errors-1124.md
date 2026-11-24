# Snapshot compile errors, 2026-11-24

Snapshot: `/Users/ueli/.sinc/play`. Source log: `.cache/b/r/m/components/runs/wAmZ9wq/l/cargo-release-00-b0.log`.

Common finding: for all ten crates the crate directory (including `📦️packages/🦀️rust`) is byte-identical to live `/Users/ueli/Documents/semio`. Live therefore holds no fix to copy. Every error is a mismatch between the crate and the snapshot's framework API, so each was adapted to the SNAPSHOT framework (pattern copied from already-migrated crates: CAD, remodeling, en1995). No framework or derive code was touched. All errors are genuine (explained from snapshot sources), none judged remote-only.

VERIFICATION STATUS: NOTHING IS CONFIRMED COMPILING. Two local `cargo check` runs (native, then wasm32-wasip2 over all 25 hub components) were started and aborted: the first on a first pass before the final edits, the second killed on the coordinator's disk limit. The check target was deleted. The edits below were written from the error text and the framework sources, not from a passing compile. The pinned chain restarted at 15:10 with these edits; its logs under `.cache/b/r/m/components/runs/<newest>/l/` are the real verification.

Paths below are relative to `/Users/ueli/.sinc/play/✏️s/🔌️plugins/`.

## Per crate

- **block-2d / block-3d / block-5d model** (`🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`)
  - Error: `Block*Snapshot: RetireOwned` unsatisfied (`ArtifactSqliteSnapshotHost` now requires it).
  - Fix: added `semio_framework_value::RetireOwned` derive to the snapshot struct (and to nested derive types in 2d and 5d; 3d's nested types already had it).
  - 3d extra: `crate::snapshot::` paths in `🚪️io/🪶️sqlite/📸️snapshot/{🦀️.rs,🛂️admission/🦀️.rs,📏️preflight/🦀️.rs}` replaced by `crate::sqlite_snapshot::` (module name). Also `default_arrangement`/`default_spacing` (used by `Block3dWindowView` field defaults, defined only in the 3d root file) were added to the snapshot crate file (found by first native check).
  - Verified: not compiled after edits.
- **forms-forms-config** (`📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🎚️config/`: `🦀️.rs`, `🧬️schema/🧬️mutations/📸️replace/🦀️.rs`, `🧬️schema/🧬️mutations/🧩️set/🦀️.rs`)
  - Error: leaves `ReplaceConfig`/`SetContributions` lacked `DslOwnedReceivingField`, `RetainedClone`.
  - Fix: added `DslReceiving` + `RetainedClone` derives to both leaves and to `FormsConfig`. Not compiled.
- **lowpoly-lowpoly-config** (`💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`)
  - Error: `LowpolyConfigMutation: DslOwnedReceivingVariants`. Fix: `DslReceiving` derive on the enum and the nine Edit structs. Not compiled.
- **lowpoly-lowpoly-model** (`…/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`)
  - Error: `decode_sqlite_snapshot_native` signature (3 vs 4 params). Fix: ported to the `output` slot form; post-decode `admit` failure returns `SqliteStageFailure::Settlement`. Not compiled.
- **mathematical-equation-snapshot** (`➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`)
  - Error: decode/encode native signatures. Fix: ported both, pre-checks return `NotStarted`. Not compiled.
- **bim-model-diff** (`🏙️bim/…/🧬️schema/🔺️diff/🦀️.rs`)
  - Error: `apply`/`absorb`/`inverse` not members of `MutationDiff`/`DiffAlgebra`. Fix: moved to `ColdMutationDiff` (`apply(&mut P)`, `absorb`) and `ColdDiffAlgebra::inverse`; `DiffAlgebra` keeps `is_empty`. Not compiled. Existing tests call `apply_diff_cold`, which fits.
- **imperative-procedure-viewer** (`📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/🦀️.rs` and `…/👁️viewer/👥️presence/🧬️schema/📦️packages/🦀️rust/🦀️.rs`)
  - Error: orphan rule, `diff_binary!(ProcedureDiff)` mounted in the viewer crate. Fix: mount `io::binary::diff` in the snapshot crate (which defines the diff), re-export from the viewer (consumers keep their path). Not compiled.
- **architect-program-tests-2** (`🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️kernel/🧪️tests/🔬️unit/📦️packages/🦀️rust/🦀️.rs`)
  - Error: unresolved `semio_s_artifact_architect_program_architect`. Cause: the crate is not a dependency and cannot be (architect depends on tests-2, a cycle). Fix: removed the stray `pub use …architect::editor::architect::*;` line. Not compiled; if something relied on that glob, further errors will surface.
- **semio-hub-draw-component**: fixed earlier (missing `pub use plugin::DRAW_MOUNTED_OWNER_POLICY;`), not re-checked.

## Further errors found in dependents (NOT fixed)

Found by local checks, none in the original failing list:

- `semio-s-artifact-bim-model-spaces` (also reproduced on wasm32-wasip2): `region_boolean`/`offset_regions` now take a fifth argument `turns: &mut impl EffectTurns` (`🧰️framework/🔨️modules/◻️2d/🧱️regions/🦀️.rs:147,215`). Callers: `🏙️bim/…/💡️inferences/🏠️spaces/🦀️.rs:144,270`, `…/🌡️energy-envelope/🦀️.rs:626`, `…/🖼️view-linework/👁️hidden-lines/🦀️.rs:437`, and likely more (`🎨️finishes`, `⚠️diagnostics/💥️clashes`, `🩺️findings`, `view-linework`). Identical in live; a framework migration across many bim crates, so out of scope. This will block `semio-hub-bim-component`.
- Seen only in the native run with a narrower `-p` set (possibly feature-unification artefacts, since the real build never reported them; unconfirmed): `forms-forms-snapshot` (`pack::snapshot` missing, `counts`), `architect-program-presence` (`AdjacencyKind: NativeReceive`), `lowpoly-lowpoly-view` and `-viewer` (`Arc<…>: RetireOwned`, `LowpolyMutation: OpBinaryReceiving`), `mathematical-equation-inferences` (`EquationRoot: RetireOwned`, orphan impls in `🔺️diff`), `block-5d-mutations` (missing mutation types), `block-2d-schema` (orphan impls), `imperative-procedure-inferences` (`ProcedureMutation: OpBinaryReceiving`). Some of these (e.g. the lowpoly and imperative `OpBinaryReceiving` ones) look like real missing `DslReceiving` derives of the same family as the fixes above.
