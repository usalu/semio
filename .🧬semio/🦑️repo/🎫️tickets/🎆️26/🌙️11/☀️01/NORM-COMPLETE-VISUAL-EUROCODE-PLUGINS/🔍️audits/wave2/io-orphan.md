# io-orphan (wave 2): orphan rule in the norm io crates

## Verdict
Confirmed. Every EN 1990-1999 artifact (and din/iso/vdi) had illegal impls; no io crate has ever compiled (the only cargo log in the cache stops in `norm-ndp`; git history shows the layout, with the impl in the io crate and the type in the mutations/model crate, since before commit 7880cc36f78; no E0117 log exists because every run died before reaching them).

## Illegal impls (en1990 shown, same shape in every artifact; line numbers of en1990 before the fix)
- `io/📝️text/📸️snapshot/🦀️.rs:78` `impl_norm_artifact_record!(@text …)` expands to `impl NormArtifactRecord for En1990Snapshot` and `impl pack::snapshot::ArtifactDsl for En1990Snapshot`: foreign trait, foreign type (E0117).
- `io/💾️binary/📸️snapshot/🦀️.rs:46` `@binary` expands to `ArtifactPack` and `ArtifactStoreSnapshot` for the snapshot (E0117).
- `io/🪶️sqlite/📸️snapshot/🦀️.rs:66` inherent `impl En1990Snapshot` (E0116), `:127` `ArtifactSqliteSnapshotHost` and `:161` `ArtifactSqliteSnapshot` for the snapshot (E0117).
- `io/📝️text/🧬️mutations/🦀️.rs:433` `impl protocol::OpText for En1990Mutation` (E0117).
- `io/💾️binary/🧬️mutations/🦀️.rs:25` `impl protocol::OpBinary for En1990Mutation` (E0117).
- Same in en1991 (sqlite :70/:106/:147, OpBinary :11, OpText :10, records), en1992 (sqlite :79/:88, OpBinary :9, OpText :412), en1993 (sqlite :53/:87/:107, :34, :12), en1994 (:40/:51/:70, :26, :12), en1995 (:34/:55/:76, :33, :13), en1996 (:63/:92/:112, :34, :10), en1997 (:40/:60/:74, OpBinary :39/:48, OpText :295/:665), en1998 (:38/:59/:79, :53, :671), en1999 (:40/:58/:92, OpBinary :38/:47, OpText :186/:508).
- Legal and untouched: `ArtifactComposition/Builder/Analysis for En19xx{Composer,Builder,Analyzer}*` (local types), `Census` in en1992 cells (local), test `SubsetRoundtripSpec` impls.
- Crate roles verified: `En19xxSnapshot` is defined in `-model` (`🧬️schema/📸️snapshot`), `En19xxMutation` in `-mutations`, the traits in `pack`, `os-kernel` (`OpText`/`OpBinary`) and `norm-compliance` (`NormArtifactRecord`).

## Fix (framework pattern of `🧊️generation3d`: codec files mounted into the owning crate via `#[path]`)
Script `🧰️scripts/io-orphan-owner-mount.py` (idempotent, backups in `🗑️generated/io-orphan/backup`), applied to en1990-en1999:
- New `🚪️io/🪞️records/🦀️.rs` per artifact (one `impl_norm_artifact_record!(Snap, extension, envelope_id, sqlite=…)` call), mounted as private `mod records` in the model lib; the two `@text`/`@binary` lines removed from the io snapshot files.
- `🚪️io/🪶️sqlite/🦀️.rs` mounted as `pub mod sqlite` in the model lib (sed `semio_…_model::` to `crate::`); io re-exports `pub use <model>::sqlite;`.
- `🚪️io/📝️text/🧬️mutations` and `🚪️io/💾️binary/🧬️mutations` mounted as `pub mod text`/`pub mod binary` in the mutations lib (sed of `crate::…mutations` paths and self-crate paths); io-text/io-binary re-export `pub use <mutations>::text as mutations;`, so all downstream paths (`io::text::mutations::…`) are unchanged.
- Cargo.toml of model/mutations gained the deps the moved code uses (pack, value-native-owned, norm-contract, diagnostic, compliance, plugin, dsl-record(-derive), artifact-reference); `Cargo.lock` package entries hand-patched (`pack` is `semio-framework-pack`); if the runner uses `--locked` and complains, run the lock sync.

## Open items
- Source-boundary gate now fails for en1998/en1999/en1993/en1996-mutations and en1991/en1996/en1997-model (numbers in `📌️important/requests.md`). Needs crate-budget splitting; the codec must remain in the type-defining crate.
- din16798, din18599, din4108, iso16757, vdi3805 have the identical defect: request logged for their owners, not edited.
- Unverified (no rustc): glob-import ambiguity inside `mutations_codec` (`use crate::text::*; use crate::*;`), `COMPONENT_*_PATH` strings changed to the mutations crate module path (only used as labels), tests mounted in the moved dirs now compile in the mutations crate (deps added to `[dependencies]`/`[dev-dependencies]` by script).
