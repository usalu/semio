# brep original-jobs compile failure

Crate: `semio-s-plugin-flow-brep-original-jobs` (workspace `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep`, member `〰️analytic-curves/📦️packages/🦀️rust`). The crate is not addressable from the repo root; run `repo cargo` from the brep workspace.

## Original error (wasm release build, ~05:00)

4x E0277 `dyn ErasedSnapshotRetirement` is not `Sync`. `RetainedCloneClose` holds `Box<dyn ErasedSnapshotRetirement>` (trait bound only `Send + Any`), and `#[derive(RetireOwned)]` on `TopologyOutput` and `deconstruct::Metadata` requires `Sync` via `OriginalOwnedEntry`.

## Current tree

The E0277 errors no longer reproduce; the framework-value sources changed since (uncommitted edits in `🧰️framework/🔨️modules/🌱️value`, not by this ticket). A different error remained: E0271 in `loft_profiles` (`i32 + 1` against `usize`, closure parameter `rank` not inferred).

## Fix

`〰️analytic-curves/🦀️.rs`: closure parameter annotated `|rank: usize|`.

## Check

`repo cargo --store=shared --configuration=prototype -- check -j 1 -p semio-s-plugin-flow-brep-original-jobs` (from the brep workspace): Finished `prototype` profile, no errors, warnings only. The wasm release build was not re-run.

## Result
`semio-s-plugin-flow-brep-original-jobs` checks clean (prototype, warnings only) after the `|rank: usize|` annotation.
