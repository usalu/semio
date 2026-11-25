# Snapshot bim: EffectTurns migration (2026-11-24)

Tree: `/Users/ueli/.sinc/play`. Verified by reading only. Nothing was compiled (no cargo/rustc).

## Definitions found
- `region_boolean(operation, subject, clip, control, turns: &mut impl EffectTurns)` and `offset_regions(regions, distance, join, control, turns: &mut impl EffectTurns)`:
  `/Users/ueli/.sinc/play/🧰️framework/🔨️modules/◻️2d/🧱️regions/🦀️.rs` (lines 147, 215). No other sibling gained `turns`; `regions_from_path` is unchanged.
- `trait EffectTurns { fn turn<R>(&mut self, grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> Result<R, ValueError>; }`:
  `/Users/ueli/.sinc/play/🧰️framework/🔨️modules/◻️2d/🧹️retire/🦀️.rs`. Its only implementor is `BlockingTurns` (`BlockingTurns::new(maximum_work: u64)`, `Copy`), described as "the blocking host edge".
- Re-exports: `semio_framework_2d::regions::EffectTurns`, `semio_framework_2d::retirement::{BlockingTurns, ..}`.

## Pattern followed
Every migrated caller is a blocking edge with no host-owned turns value. It constructs `BlockingTurns::new(work)` in place and passes `&mut` of it. No caller threads `turns` through its signature.
- Framework tests: `region_boolean(.., &mut |_| true, &mut semio_framework_2d_retire::BlockingTurns::new(TURN_WORK))` with `const TURN_WORK: u64 = 1 << 20;` (`.../🧱️regions/🧪️tests/🔬️unit/🦀️.rs`).
- Raster plugin: `let mut turns = BlockingTurns::new(work); turns.turn(grant, ..)` (`🖨️raster/.../🧬️schema/🖼️asset-content-and-patch/🦀️.rs`).

No bim function has a caller that owns a turns value, so the bim crates follow the same pattern. Each file got `use semio_framework_2d::retirement::BlockingTurns;` and `const TURN_WORK: u64 = 1 << 20;` with an emoji docstring. Each call passes `&mut BlockingTurns::new(TURN_WORK)` as the fifth argument. No function signature changed.

## Files changed
All under `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`:
- `🪑️components/🩺️findings/🦀️.rs`: `overlap_area`, 1 call.
- `🌡️energy-envelope/🦀️.rs`: `horizontal_surfaces`, 1 call.
- `🖼️view-linework/🦀️.rs`: union area, 1 call.
- `🖼️view-linework/👁️hidden-lines/🦀️.rs`: union of cut regions, 1 call.
- `🏠️spaces/🦀️.rs` (crate `semio-s-artifact-bim-model-spaces`): `run` (region_boolean) and `finish` (offset_regions), 2 calls.
- `🎨️finishes/🦀️.rs`: `covered_area`, 2 calls.
- `⚠️diagnostics/💥️clashes/🦀️.rs`: `intersection_area`, 1 call.

That is 9 call sites in 7 files. The unit-test files in `🌡️energy-envelope` and `🏠️spaces` only import the two functions and never call them, so they are unchanged.

## Drift check
- The only `semio_framework_2d` items the bim crates use are `Region`, `OffsetJoin`, `region_boolean`, `offset_regions`, `BooleanOperation` and `BooleanProgress`. All exist with matching shapes. `Region::new(&[Vec2], &[Vec<Vec2>])` and `Region::area` are unchanged. No other callee drift was found in the touched functions, but I only checked the framework signatures, not every local helper.
- `🧬️schema/🔺️diff/🦀️.rs` (`ModelDiff`) against `/Users/ueli/.sinc/play/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🔺️diff/🦀️.rs` and `.../📡️spr/🎮️command/🦀️.rs`, plus the `MutationDiff` supertrait bounds:
  - `ColdMutationDiff<P>` needs `MutationDiff<P> + FromValue`, `fn absorb(&mut self, other: Self)` and `fn apply(&self, &mut P, ApplyCapability) -> MutationApplyResult<()>`. All present with the right signatures.
  - `ColdDiffAlgebra<P>` needs `fn inverse(&self, base: &P) -> Self`. Present.
  - `DiffAlgebra::is_empty` is present.
  - `MutationDiff` needs `Default + PartialEq + ToValue + DiffAlgebra`. The derives and impls cover these.
  - `DiffRegions::touches -> TouchedPaths` matches. It is re-exported from the spr command module as `protocol`.
  - `retire_cold` and `retire_projection` have defaults, so no override is required.

## Unresolved / risks
- Not compiled. The per-turn budget `1 << 20` copies the framework test value. If a turn needs more work, `BlockingTurns` returns an error that surfaces as `BooleanError::Refused`, which the callers' `unwrap_or_default` and `map_or` turn into empty or zero results. That could hide a budget shortfall for large inputs.
- Unchecked: whether each touched crate's `Cargo.toml` pulls `semio-framework-2d` with the `retirement` re-export. They already import `semio_framework_2d::regions` and `::booleans`, so it should. A full-tree grep of `✏️s/🔌️plugins` for other stale callers had not completed when I stopped. The bim grep was complete.

## Follow-up: `Filter<..>` is not `Clone` (bim-model-components, E0599)
- Log: `.cache/b/r/m/components/runs/wGjp1Sg/l/cargo-release-00-b0.log`, 9 errors, all `.clone()` on `Filter<impl Iterator<Item=(&String,&T)>, closure>` in `💡️inferences/⚠️diagnostics/🦀️.rs` (columns, beams, slabs, ceilings, roofs).
- Live repo: byte-identical to the snapshot for that file, so it had no fix to port.
- Fix (crate-local, framework untouched): each filtered iterator is collected once into `Vec<_>` (`.collect::<Vec<_>>()`). The first use becomes `dep_records(x.iter().copied())` and the second `x.iter().map(..)`.
  - `⚠️diagnostics/🦀️.rs`: `storey_dependency` (columns, beams, slabs, ceilings, roofs) and `building_dependency` (columns, beams, slabs, ceilings).
  - `🗺️plan-linework/🦀️.rs`: `dependency` (columns, beams, slabs, ceilings), which had the identical `.clone()` pattern. It would have failed in the same way in its own crate.
- Both under `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`.
- Scan of the other bim crates for `<name>.clone()` on a filtered iterator: no further instances. The remaining `.clone()` hits are on owned collections.
- Not compiled; checked by reading only. The `|(_, row)|` closures over `&(&String,&T)` rely on default binding modes, which I believe is fine.

## model-graph Arc retirement
Crate `semio-s-artifact-bim-model-inferences`. Log: `.cache/b/r/m/components/runs/woZ2eQO/l/cargo-release-00-b0.log`, lines 244668-246221. The log reports 69 errors in total, not 19. **Nothing was compiled.**

### Cause
- `Data` (`🕸️model-graph/🦀️.rs`, the value enum of `ModelValue`) derived `semio_framework_value::RetireOwned`. The derive needs every variant field to implement `RetireOwned` (the snapshot derive has no per-field or per-variant shared/skip attribute; `#[value(send_only)]` only switches off the shared-original path of structs). The snapshot framework has no `RetireOwned for Arc<T>`, so the 33 `Arc` variants and the two `Option<Arc<..>>` variants all failed.
- `RetireOwned` is required by `InferredField::Value: RetireOwned + Clone + Send + Sync`, so it cannot be dropped.
- Two non-`Arc` payloads had no impl at all: `Band` and `CutRect`. Behind the `Arc` errors, five more payloads also had none: `HostExtent`, `SolidEntry` (through `RoofFallback`), `ComponentEntry` (which also holds `Arc<FamilyValue>`), `AttachSurface` and `SolidProbe`.

### Model followed
The lowpoly session transient (`💠️lowpoly/.../✏️editor/🖌️session/🦀️.rs`) uses a hand-written `RetireOwned` in the new shape. `type RetirementCursor = <Owners as RetireOwned>::RetirementCursor`, `retirement_seed` maps every `Arc` to `ArcLease::new(arc)` and seeds the owners, and `controlled_retirement_supported` delegates. `ArcLease<T>` needs `T: RetireOwned + Sync`.
- `Data`: the derive is removed. A `#[doc(hidden)] pub enum DataOwners` with the same variants (each `Arc<T>` becomes `ArcLease<T>`, `Option<Arc<T>>` becomes `Option<ArcLease<T>>`) derives `RetireOwned`. The derive's enum cursor works without `send_only`, because the shared-original path only applies to structs. `impl RetireOwned for Data` maps variant by variant. It is `pub` so the public associated cursor type can name it.
- `ComponentEntry`: hand-written impl over `(ComponentValue, ArcLease<FamilyValue>)`. This is the lowpoly state pattern.
- `Band`: `semio_framework_value::artifact_retire_leaf!(Band)`. It is `Copy`, and its `BulgeSeg` field is a framework geometry type with no `RetireOwned`, so the field-wise derive is impossible. Leaf use in plugins follows trinity rewriting, raster and flow.
- `CutRect`, `HostExtent`, `SolidEntry`, `RoofFallback`: added `semio_framework_value::RetireOwned` to their derive lists. Every field type already implements it (`Resolved`, `OpeningCut`, `Axis`, `Point2`, `(f64, f64)`, `Vec<ElevationPoint>`, `ElementSolid`, and a unit enum).
- No payload type changed from `Arc`, and there is no `unsafe`.

### Files changed
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🕸️model-graph/🦀️.rs` (`Data` derive removed, `DataOwners` added, `impl RetireOwned for Data` added, `ArcLease`/`RetireOwned` import added)
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪑️components/🦀️.rs` (`impl RetireOwned for ComponentEntry` and import)
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️wall-layout/🔗️joins/🦀️.rs` (`artifact_retire_leaf!(Band)`)
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪟️opening-frames/🦀️.rs` (`HostExtent`, `CutRect` derive)
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🦀️.rs` (`SolidEntry` derive)
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🏠️roofs/🦀️.rs` (`RoofFallback` derive)

### Other instances
A balanced-body scan of every `derive(..RetireOwned..)` struct and enum in `🏙️bim` for `Arc<`, `Rc<` or `SharedAllocation<` fields found none left. The scan was checked against the unfixed live copy, where it does flag `Data`. No `type X = Arc<..>` aliases exist in bim.

### Unresolved
- **`AttachSurface` and `SolidProbe` still have no `RetireOwned`**, so `DataOwners::Surface(ArcLease<AttachSurface>)` and `DataOwners::Probe(ArcLease<SolidProbe>)` will still fail with E0277.
  - `AttachSurface` holds `Vec<Piece>`, and `Piece` holds `Vec<semio_framework_geometry::Point>` and `ZPlane`.
  - `SolidProbe` holds `Aabb`, `TriMesh` and `Bvh` (whose fields are private).
  - None of these framework geometry types implement `RetireOwned` (there are zero hits in `🧰️framework/🔨️modules/📐️geometry`, in the snapshot and in the live repo). The orphan rule forbids impls in bim, and there is no safe cursor for a `Vec` of foreign elements.
  - Fix belongs in the framework geometry crate: `artifact_retire_leaf!` for `Point`, `ZPlane`, `Aabb` and `BulgeSeg`, plus derived or authored `RetireOwned` for `TriMesh` and `Bvh`. After that, `Piece`, `AttachSurface` and `SolidProbe` can derive it. `Band` could then also switch from leaf to derive.
- The same crate has non-retirement errors outside this brief:
  - E0432: `protocol::step_field_update` no longer exists, at `🕸️model-graph/📡️session/🦀️.rs:13` and used at line 213. No `*field_update*` fn exists in the snapshot's `💡️inference` module, so this is real API drift.
  - E0599: `mutation.diff(base)` on `Op: Mutation<..>` at `🕸️model-graph/🧳️instance/🦀️.rs:84`. `diff` moved to `protocol::ColdMutation`, as in the lowpoly/block ports.
- Risks:
  - `DataOwners` is `pub` + `#[doc(hidden)]`.
  - `Data`'s readiness now uses the default `retirement_ready` (true), as lowpoly does.
  - `FamilyValue` and the other payloads are assumed `Sync`, which `Data: Send + Sync` already implies.

### Follow-up: crate completion (supersedes "Unresolved" above)
Still **nothing compiled**. The framework was not touched; every change below is inside `🏙️bim`.

#### 1. `AttachSurface` and `SolidProbe` (the `Surface` and `Probe` payloads)
Mechanisms the framework offers, from reading `🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs`:
- `artifact_retire_leaf!` expands to a `Leaf<T>` cursor bounded `T: Copy + Send + 'static`. It moves the value out in one charged step and never drops anything, so it really does require `Copy`. That rules it out for `Vec`-holding values.
- `artifact_retire_struct!` and the `RetireOwned` derive both need every field to implement `RetireOwned`.
- Every shared cursor (`ArcLease`, `SealedShared`, `SharedIssuer`) needs `T: RetireOwned`.
- Uncontrolled owners (`controlled_retirement_supported() == false`) are gated by the framework's `🔓️uncontrolled-owners` allow-list fixture.
- There is no framework drop or plain cursor for a heap-holding value of a foreign type.

How plugins handle foreign heap values:
- remodeling `✏️editor/⚙️engine/🖼️images/🦀️.rs` has `OpaqueOwner<T>` / `OpaqueOwnerRetirement<T>`. It is documented as a foreign value that "retires by one drop whose frame is charged, because its crate owns no retirement cursor". It is written in the new cursor shape: `native_retirement_reply!`, one `record_retirement_work`, a `retained_clone_grant_admits` check, and drop.
- layout drawing composition uses `artifact_retire_leaf!` for `Copy` records holding foreign types.
- puzzle 3d `CollisionMesh` uses a bespoke buffer-by-buffer cursor over its own collision mesh. That is not applicable here: `semio_framework_geometry::collision::Bvh` has private fields.
- No plugin derives or implements `RetireOwned` over `semio_framework_geometry` types.
- Not verified: remodel is not among the `Compiling` lines of any run log under `.cache/b/r/m/components/runs`. The same cursor shape is used by the framework's own `Leaf` and `ArcLease` cursors and by layout's `PlanRetirement` (layout compiled).

Mechanism used:
- The remodeling `OpaqueOwner` pattern, copied once into the bim crate that both payload crates already depend on.
- It is applied field-wise only to the framework geometry parts. Every field that has a real cursor keeps it.
- No `unsafe`, no uncontrolled owner, and `Data` keeps its payloads as `Arc` behind `ArcLease`, so behaviour is unchanged.

Details:
- `OpaqueOwner<T>(pub T)` and `#[doc(hidden)] OpaqueOwnerRetirement<T>` are in `🧊️element-solids/🦀️.rs` (crate `semio-s-artifact-bim-model-diff`, Helpers region).
- `Piece` (`🧱️wall-layout/🧲️attach/🦀️.rs`) holds `Vec<Point>`, `Vec<Vec<Point>>` and `ZPlane`, all framework geometry with no cursor. It gets a hand-written `RetireOwned` whose cursor is `OpaqueOwnerRetirement<Piece>`. `AttachSurface` then derives `RetireOwned` (`bool`, `Vec<Piece>`, `f64`, `f64`).
- `SolidProbe` (`🧨️clash-sets/🦀️.rs`) gets a hand-written `RetireOwned` in the forms/puzzle shape:
  - `type RetirementCursor = retirement_fields_type!(Option<ElementClass>, String, OpaqueOwner<Option<Aabb>>, Vec<[f64; 3]>, Vec<[f64; 3]>, Vec<[u32; 3]>, OpaqueOwner<Bvh>)`.
  - `retirement_seed` destructures `TriMesh { positions, normals, indices }`, whose fields are all `pub`. The mesh buffers therefore retire through the framework `Vec`/array cursors. Only the `Copy` box and the private-field `Bvh` are opaque.

#### 2. API drift
- `protocol::step_field_update(snapshot, cache, update, cx)` was renamed in the snapshot driver to `update_field_cold(snapshot, cache, update)`, without the step context (`🏁️driver/🦀️.rs:206`).
  - Its only caller is the framework's own unit test, `💡️inference/🧪️tests/🔬️unit/🦀️.rs:751`, which calls it inside `caller.turn(grant, |cx| ..)` without passing `cx`.
  - Ported in `📡️session/🦀️.rs`: the import, the call at line 213, and the module doc. The session unit test's import (`📡️session/🧪️tests/🔬️unit/🦀️.rs:11`) is ported too.
  - `cx` is still used in `step` by `begin_update_funded`.
- `record_mutations` (`🧳️instance/🦀️.rs:78`): the bound changed from `Op: Mutation<ModelSnapshot, Diff = ModelDiff>` to `Op: ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff>`.
  - This is the bound every bim mutation kind already uses. `ModelMutation` gets it from `#[mutations(.., reference_diff = ModelDiff)]`.
  - `mutation.diff(base)` now resolves to `ColdMutation::diff`, which also returns `MutationOutcome`, so `.into_parts()` is unchanged.
  - The now-unused `Mutation` import was dropped.

#### 3. Error-class sweep of the 69-error block (log lines 244668-246221)
- 67 × E0277 are all in `🕸️model-graph/🦀️.rs` (the `Data` derive). The 35 distinct payload types each now have a `RetireOwned` path: `ArcLease` over a payload with a derived or hand-written impl, leaf `Band`, derived `CutRect` and `StoreyLevel`.
- 1 × E0432 (`step_field_update`): ported.
- 1 × E0599 (`diff`): ported.
- No other error class exists for this crate. The 20 warnings are unused imports or qualifications and are left alone.

#### Files changed (cumulative)
All under `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`:
- `🕸️model-graph/🦀️.rs`
- `🕸️model-graph/📡️session/🦀️.rs`
- `🕸️model-graph/📡️session/🧪️tests/🔬️unit/🦀️.rs`
- `🕸️model-graph/🧳️instance/🦀️.rs`
- `🪑️components/🦀️.rs`
- `🧱️wall-layout/🔗️joins/🦀️.rs`
- `🧱️wall-layout/🧲️attach/🦀️.rs`
- `🪟️opening-frames/🦀️.rs`
- `🧊️element-solids/🦀️.rs`
- `🧊️element-solids/🏠️roofs/🦀️.rs`
- `🧨️clash-sets/🦀️.rs`

#### Unresolved / risks
- `OpaqueOwnerRetirement` charges one frame and reports zero release bytes. The release accounting for a `Piece` polygon and a probe's `Bvh` therefore under-reports their heap; the real drop still happens in the funded step. This is the same trade-off remodeling makes.
  - The exact fix is a framework geometry cursor, ruled out for this snapshot.
  - The `TriMesh` buffers, the probe's `String` and all other payloads are retired exactly.
- Crates touched beyond `inferences` (and the files each adds or changes):

  | Crate | Changes |
  |---|---|
  | `bim-model-diff` | `OpaqueOwner`, `SolidEntry`, `RoofFallback` |
  | `bim-model-wall-layout` | `Band`, `Piece`, `AttachSurface`, `HostExtent`, `CutRect` |
  | `bim-model-ramp-runs` | `SolidProbe` |
  | `bim-model-components` | `ComponentEntry` |

  These crates compiled in this run before the change, so a mistake there would now surface in them first.
