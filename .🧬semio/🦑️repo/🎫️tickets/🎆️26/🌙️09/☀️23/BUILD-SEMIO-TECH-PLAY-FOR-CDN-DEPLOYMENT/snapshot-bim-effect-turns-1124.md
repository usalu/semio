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
