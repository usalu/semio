# Snapshot Derive Sweep (Receiving / Clone Families), 2026-11-24

Tree edited: `/Users/ueli/.sinc/play` (plugin crates only). Live `/Users/ueli/Documents/semio` was read only. **Nothing was compiled.** No cargo or rustc was run. Every finding below comes from reading the snapshot framework, the compiler log `.cache/b/r/m/components/runs/wGjp1Sg/l/cargo-release-*.log` and plugin sources. The next pinned release run is the real verification.

## 1. Rule (snapshot framework)

- `original_ordinal_operation_receiving!(T)` and `original_tagged_operation_receiving!(T, P)` (`🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs`) implement `OpBinaryReceiving` through `OriginalOperationReceiving<T>`. They need `T: DslOwnedReceivingVariants + BorrowedDslVariants`, which means `T` derives `DslEnum` plus `DslReceiving` or `DslOwnedReceiving`. `DslOwnedReceivingVariants: RetireOwned`, so `T` also needs `RetireOwned`.
- `DslReceiving` / `DslOwnedReceiving` (`🔨️modules/🗣️dsl/🧬️schema/✨️derive/📥️owned`, `🪢️receiving-and-projection`) put every field type (struct fields, named-variant fields and tuple payloads, `#[dsl(skip)]` included) into `OwnedReceivingSlot<F>` or `MappedReceiving<F, _>`. So every `F` needs `DslOwnedReceivingField` (which also needs `RetireOwned`). With the synchronous `DslReceiving`, every `F` also needs `DslReceivingField`, and tuple payloads need `DslReceivingRecord`. Field types are covered by the derive on plugin records, by built-ins (scalars, `String`, `Option`/`Vec`/`Box`/`PagedList`/`PersistentList`/`OrderedMap`, `ArtifactChild<S>`) or by the macros `dsl_owned_scalar_receiving!`, `dsl_scalar_newtype_receiving!`, `dsl_transparent_receiving!` and `native_field_receiving!`. An all-unit enum deriving `DslReceiving` also needs `Copy`.
- `original_pack_value_operation_receiving!(T)` needs only `T: FromValue + RetireOwned`.
- `StoreMutationFor<P>` (`📡️spr/📏️store-bounds`) requires `OpBinaryReceiving`. Four bindings require `StoreMutationFor` or `OpBinaryReceiving`:
  - `ArtifactViewer` / `ArtifactEditor` / `ArtifactApp::{Mutation, ConfigMutation, DraftMutation}`
  - `WindowConfigOwner::Mutation`
  - the genesis `Mutation`

  Presence and transient mutation lanes do **not** require it.
- Orphan rule: the receiving macro must expand in the crate that defines `T`.
- `#[derive(RetainedClone)]` needs `RetainedClone` on every field or payload type.
- `#[derive(NativeReceive)]` puts `F: NativeReceive<dyn NativeValueSource>` (plus `NativeDefault` for optional fields) into the impl's `where` clause. A miss shows up at the use site (E0277/E0599), as in architect `AdjacencyKind`.

## 2. Types checked

Method: a Python index over all 36,863 plugin `.rs` files (17,294 struct/enum definitions) plus the framework (11,776 definitions). It records manual impls and the impl-producing macros. False positives were resolved by hand. Calibration: crates that compiled in run `wGjp1Sg` (1,815 artifacts) prove their types clean. CAD is the derive-list reference.

- **Receiving-macro targets:** all 54 `original_ordinal/tagged_operation_receiving!` calls, with their enums, payload leaves and field types followed while the derive chain continues.
  - Violation: `HomeConfigMutation` (space home). Fixed.
  - Clean (`ok`): trinity jack/rewriting (4), raster (2), process3d, cad, block config enums 2d/3d/5d (already fixed earlier), dag (2), tiff, wires (2), animate presentation, space index, generation 2d/3d (3), gis (3), wfc (8), imperative procedure, sourcing (2), note (2), forms config, shooting (2), puzzle 2d/3d/5d (6), fem (4), draw, playbook, lowpoly config, energy.
- **`DslReceiving` derives:** 399 roots. **`DslOwnedReceiving`:** 168 roots. All field-type candidates resolved as false positives:
  - jack `PropertyDef` is the framework type, which derives `DslReceiving`.
  - raster `RasterConfigViewportSize` is an alias of `RasterViewportSize`, which derives `NativeReceive` + `DslOwnedReceiving`.
- **`RetainedClone` derives:** 5,598 roots. Clean. `RasterSpanPage` sits inside `PersistentList`, which only needs `RetireOwned + Sync`.
- **`NativeReceive` derives:** 898 roots. All candidates are manual impls (raster `RasterSelectionSpans`, `RasterAdjustmentNumber`, `RasterLayerNode`, wfc `SetInputPixels`) or sit in crates that compiled in this run (energy `EntityId`/`ScheduleId`, gis terrain, layout `QuestionDictionary`). stdio registry `Part21Value` is not in the release graph.
- **`OpBinaryReceiving` on bound mutation types:** I compared every `type Mutation / ConfigMutation / DraftMutation = X` against every plugin receiving macro or manual impl.
  - 24 violations, listed in section 3.
  - Transient and presence types were excluded: their trait bounds do not require it, and fem/dag ones compiled.
  - `ZipIso21320Mutation` / `SemioModelMutation` are clean (zip compiled; `SemioModelMutation` uses pack-value).

## 3. Files changed

### 3.1 Ported exactly from live
Live has these fixes, and the snapshot framework has the same macros and derives. Each file is now byte-identical to live (`cmp` checked).

- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
  - Added `#[derive(semio_framework_value::RetainedClone,semio_framework_value::NativeReceive,semio_framework_dsl_record_derive::DslOwnedReceiving)]` to `HomeConfigMutation`, `RetireLocalStudio` and `ListLocalStudio`. This was the `original_ordinal_operation_receiving!(HomeConfigMutation)` violation.
  - Live's extra `begin_copy_byte_demand` method was deliberately not ported. It belongs to a different family.
- `SHomeMutation` and `SSpaceMutation` (document lanes): `DslReceiving` on each enum and its leaves, plus `dsl::original_tagged_operation_receiving!(X, COMPONENT_PROTOCOL_SEMIO);` in the io binary module. That module is mounted in the same crate as the enum (`…-operations` carve).
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` (enum derive)
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🦀️.rs` (leaf derive)
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (macro call)
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` (enum derive)
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{🏷️rename-artifact,🗑️delete-artifact,🕒touch-artifact,🌱create-artifact}/🦀️.rs` (leaf derives; `SpaceArtifactRow` already derives `DslReceiving`)
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (macro call)

### 3.2 Added `original_pack_value_operation_receiving!(X);`
Live has no fix for these. The added line sits directly after the enum, in the defining file. The model is CAD (`dsl::original_pack_value_operation_receiving!(CadMutation)`), norm window configs (pack-value receiving paired with `variants_binary::encode_op`) and flow (`field_set_mutations_binary!` paired with pack-value). Every enum already derives `FromValue` and `RetireOwned`, and every owning crate depends on `semio-os-kernel`.

Prefix `dsl::` (block files, same alias as their `dsl::Mutations`); every other file uses `semio_os_kernel::`.

| Type | File (under `/Users/ueli/.sinc/play/✏️s/🔌️plugins/`, `~` = `🏅️standards/🔖️1/🪆️subsets/✳️any`) |
|---|---|
| `Block2dMutation` | `🧱️block/🗿️artifacts/◻️2d/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `Block3dMutation` | `🧱️block/🗿️artifacts/🧊️3d/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `Block5dMutation` | `🧱️block/🗿️artifacts/🖐️5d/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `LowpolyMutation` | `💠️lowpoly/🗿️artifacts/💠️lowpoly/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `ModelMutation` (bim) | `🏙️bim/🗿️artifacts/🏢️model/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `FormMutation` | `📋️forms/🗿️artifacts/📋️forms/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `FormsTryWindowConfigMutation` | `📋️forms/🗿️artifacts/📋️forms/~/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs` (beside `field_set_mutations_binary!`, exactly like flow main window) |
| `ProgramMutation` | `🏛️architect/🗿️artifacts/🏛️program/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `ArchitectConfigMutation` | `🏛️architect/🗿️artifacts/🏛️program/~/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` |
| `ArchitectAdjacencyWindowConfigMutation` | `🏛️architect/🗿️artifacts/🏛️program/~/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config/🦀️.rs` |
| `ArchitectGraphWindowConfigMutation` | `…/🪟️windows/🕸️graph/🎚️config/🦀️.rs` (same prefix) |
| `ArchitectRegisterWindowConfigMutation` | `…/🪟️windows/📋️register/🎚️config/🦀️.rs` |
| `ArchitectReportWindowConfigMutation` | `…/🪟️windows/📓️report/🎚️config/🦀️.rs` |
| `EquationMutation` | `➗️mathematical/🗿️artifacts/➗️equation/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `EquationGraphWindowConfigMutation` | `➗️mathematical/🗿️artifacts/➗️equation/~/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` |
| `SequenceMutation` (uninhabited) | `🎬️sequence/🗿️artifacts/🎬️sequence/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `SequenceMainWindowConfigMutation` | `🎬️sequence/🗿️artifacts/🎬️sequence/~/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs` |
| `WriterMutation` | `✒️writer/🗿️artifacts/✒️writer/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `WriterMainWindowConfigMutation` | `✒️writer/🗿️artifacts/✒️writer/~/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` |
| `VcsDemoMutation` | `🌿️vcs/🗿️artifacts/🌿️vcs/~/🧬️schema/🧬️mutations/🦀️.rs` |
| `VcsDemoConfigMutation` | `🌿️vcs/🗿️artifacts/🌿️vcs/~/✏️editor/🎚️config/🦀️.rs` |
| `ImperativeConfigMutation` | `📜️imperative/🗿️artifacts/📜️procedure/~/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` |

## 4. Violations per plugin (found / fixed)

| Plugin | Violations | Detail |
|---|---|---|
| space | 3 / 3 | `HomeConfigMutation`, `SHomeMutation`, `SSpaceMutation` |
| block | 3 / 3 | document mutations 2d/3d/5d |
| architect | 6 / 6 | program, config, 4 window configs |
| forms | 2 / 2 | document, try window |
| equation | 2 / 2 | |
| sequence | 2 / 2 | |
| writer | 2 / 2 | |
| vcs | 2 / 2 | |
| lowpoly | 1 / 1 | |
| bim | 1 / 1 | `ModelMutation` |
| imperative | 1 / 1 | config |
| **Total** | **25 / 25** | |

All others: 0 found.

## 5. Unresolved / risks

- **Nothing compiled.** Every edit invalidates its crate and its dependents. bim `ModelMutation` sits in a low crate, so it triggers a wide bim rebuild.
- **Pack-value is not checked against the `OpBinary` byte layouts.** It satisfies the trait bound and is the framework's own choice for `variants_binary::encode_op` (norm) and `encode_tagged_op` (CAD). Two types have a different `OpBinary` layout:
  - Lowpoly encodes JSON text.
  - Some window configs encode `OpText` bytes.

  For these, receiving at runtime may refuse original operation bytes. That is a runtime (not compile) question, left for the e2e pass.
- **Not in my family, noticed:** live `HomeConfigPreparation` implements `begin_copy_byte_demand`, which the snapshot file lacks. The snapshot framework declares that method in several store traits. If it is a required trait method, space-home will report E0046.
- Framework types were not edited. No framework type was found missing a required trait.
