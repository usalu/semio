# Snapshot Block 2d/3d/5d and Imperative Procedure Fixes (2026-11-24)

Tree edited: `/Users/ueli/.sinc/play` (`P` = `/Users/ueli/.sinc/play/✏️s/🔌️plugins`). Nothing was compiled or run (no cargo/rustc); all fixes are by reading the compiler log, the framework derives/macros and compiled sibling crates. The release build in progress is the verifier.
Live (`/Users/ueli/Documents/semio`) was diffed for every touched file: it is identical to the snapshot except for these edits, i.e. live has no fix for any of the five crates.

## 1. semio-s-artifact-block-2d-schema (E0117 x4)
- Cause: `♻️retirement/🦀️.rs` ran `artifact_retire_struct!` (implements foreign `RetireOwned`) for `Block2d{Presentation,HandleKind,HandleTemplate,Snapshot}`, types owned by `semio-s-artifact-block-2d-model`. The model already derives `RetireOwned` on all four (so it would also conflict, E0119).
- Model followed: block-3d/5d snapshot structs (impl lives on the type via derive in the owning crate).
- Fix: removed the `retirement` mount from the schema crate and deleted the now-empty retirement file and folder.
- Changed:
  - `P/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` (mount removed)
  - deleted `.../◻️2d/.../🧬️schema/♻️retirement/🦀️.rs` (+ dir)

## 2. semio-s-artifact-block-3d-editor-config (DslOwnedReceivingVariants x5)
- Cause: `Block3dConfigMutation` calls `original_ordinal_operation_receiving!` but lacked `DslReceiving`; its `SetCamera` field `BlockCamera3d` also lacked `DslReceiving`.
- Model followed: lowpoly / forms / generation2d config enums (`DslEnum` + `DslReceiving`). The derive supports named-field and unit variants (read in `🗣️dsl/🧬️schema/✨️derive/🪢️receiving-and-projection` and `📥️owned`), so the inline variants were kept (no payload structs, no caller changes). All compiled siblings use tuple payload structs, so named-variant support is verified by reading only.
- Changed:
  - `P/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs` (`DslReceiving` on enum)
  - `P/🧱️block/🧬️schema/🧱️document-fields/🦀️.rs` (`DslReceiving` on `BlockCamera3d`; plugin-block deps already include dsl-record + derive)

## 3. semio-s-artifact-block-3d-mutations and 4. semio-s-artifact-block-5d-mutations
- Cause: the five leaves (`RenameRepresentation`, `ChangeRepresentationLod`, `ChangeRepresentationDescription`, `AddRepresentationTag`, `RemoveRepresentationTag`) live in the `*-mut-representations-2` crate and are re-exported as modules by `*-mut-representations`, but the enum file imported only five other leaves.
- Model followed: the compiler's own suggestion (`...mut_representations::add_representation_tag::AddRepresentationTag`) and the block-2d mutations crate (imports every leaf).
- Fix: extended the existing `use ..._mut_representations::{...}` line with `rename_representation::RenameRepresentation, change_representation_lod::ChangeRepresentationLod, change_representation_description::ChangeRepresentationDescription, add_representation_tag::AddRepresentationTag, remove_representation_tag::RemoveRepresentationTag`. No manifest change (dependency already present).
- Changed:
  - `P/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
  - `P/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`

## 5. semio-s-artifact-imperative-procedure-inferences (ProcedureMutation: OpBinaryReceiving)
- Cause: `OpBinaryReceiving` comes only from `semio_os_kernel::original_ordinal_operation_receiving!`, which needs `DslOwnedReceivingVariants` (derive `DslReceiving`) and `BorrowedDslVariants` (derive `DslEnum`). The uninhabited `ProcedureMutation` had neither and no macro call.
- Model followed: framework `NoConfigMutation` (empty-lanes): `DslEnum` + `DslReceiving` + macro call next to the hand-written `OpBinary`.
- Fix is in the snapshot crate that owns the type (not the inferences crate).
- Changed:
  - `P/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` (derives)
  - `P/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (macro call inside `mutations_codec`)

## Sibling follow-ups (same families, not yet reached by the compiler)
- block-5d-schema: same orphan retirement module as 2d. Removed mount + deleted `.../🖐️5d/.../🧬️schema/♻️retirement/🦀️.rs`; edited `.../🖐️5d/.../🧬️schema/🦀️.rs`. All five 5d types already derive `RetireOwned`.
- block-3d-schema: its retirement module held a manual `RetireOwned for Block3dSnapshot` (orphan, and it conflicted with the derive on the model). Moved the impl (kit-catalog lease) into the owning model crate: removed `RetireOwned` from the `Block3dSnapshot` derive and appended the impl (with its `use`) to `.../🧊️3d/.../🧬️schema/📸️snapshot/🦀️.rs`; removed the mount from `.../🧊️3d/.../🧬️schema/🦀️.rs`; deleted `.../♻️retirement/🦀️.rs`. The model already imports `SemioKitSnapshot`.
- block-2d / 5d editor-config: uninhabited `Block2dConfigMutation` / `Block5dConfigMutation` call the same macro without `DslEnum`/`DslReceiving`; added both derives in `.../◻️2d/.../✏️editor/🎚️config/🦀️.rs` and `.../🖐️5d/.../✏️editor/🎚️config/🦀️.rs`.

## Open risks
- Nothing compiled; every claim is from reading.
- `BlockCamera3d` is in `semio-s-plugin-block`, so the plugin change invalidates every dependent crate in the running build (cache-wide rebuild of block consumers).
- Block3d config enum: named-field/unit variants under `DslReceiving` are supported by the derive source but not exercised by any compiled sibling; fallback is payload structs like lowpoly (touches the ~10 command files and tests).
- The 3d manual `RetireOwned` impl now lives in the model crate; it needs `ArtifactChild::take_local_owner` and `SharedControlledRetirement` visible there (same deps as before, not compile-checked).
- Further errors may appear after these families clear (rustc stops per family).
