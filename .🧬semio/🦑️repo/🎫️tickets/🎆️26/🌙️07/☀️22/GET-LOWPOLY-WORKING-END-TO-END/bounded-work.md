# Bounded Work Remaining

## Current Evidence

The editor retained-command implementation advertises resumable work with a 7,500-microsecond turn contract. LowpolyRetainedCommandWork::extent returns two units, stage zero yields Preparing Lowpoly command, and the next step invokes lowpoly_retained_reduce synchronously. The reducer directly calls complete modeling handlers, import/export handlers, and paint fill handlers. Cancellation is observed between retained turns; the source does not make an algorithm already running interruptible. Current preparation-only progress controls and the new mounted pre-publication cancellation regression do not prove algorithm-interior cancellation or latency compliance.

The lowpoly_paint_tick fill branch materializes one complete layer, clones it, calls flood_fill, and scans the entire result with pixel_runs_from_diff. The schema flood_fill uses an unbounded stack/visited traversal of the fixed texture; pixel_runs_from_diff also scans a complete buffer. The retained command cursor has no owned algorithm workspace today. Its copy demand is size_of Self, its checkpoint is 72 bytes with a u8 preparation stage, and closing only retires that fixed frame. Adding dynamic buffers without updating those accounting/retirement/checkpoint obligations would be incorrect.

## Required Local Design

Define language-neutral work-state and prefix contracts before adding algorithm cursors. Each expensive command needs explicit bounded phases, actual work extent, a stable command/operation/generation/base/context identity, bounded byte and work-unit demand, and bounded retirement of any dynamic workspace. Output must remain private until one final publication; cancellation must retire it without an edit or leaked preview. Preserve exact gesture commit/abort behavior through the same bounded fill path.

For fill, phases should resolve the target, materialize input in bounded slices, traverse a bounded number of four-connected pixels per turn using the existing exact RGBA equality predicate, coalesce changed whole RGBA pixels incrementally, and transfer the final runs into one mutation. Use one cursor implementation for one-shot fills and fill gestures; do not leave a synchronous gesture bypass. Cursor restoration must resume or deterministically replay through bounded steps under the original identity. Do not widen time limits, truncate accepted geometry, or claim a preparing yield bounds the subsequent reducer.

Modeling needs command-specific bounded topology phases, including connectivity reconstruction and validation, rather than repeated complete-mesh reductions on smaller selections. Import/export needs bounded parsing/serialization paths that preserve all supported formats. These remain required work, not a proposed substitute for the full end-to-end goal.

## Verification Needed

Neutral contracts must cover step budgets, cancellation at every prefix, no publication before completion, exact one-edit completion, checkpoint/restoration at every phase, stale identity refusal, and bounded close with no retained dynamic storage. Compare final fill pixels against an independent third-party four-connected exact-color implementation; preserve the existing Three.js and gltf independent geometry oracles. Run the actual retained host tests and observe progress/cancel in the live browser, including a document large enough to require several algorithm steps. No timing benchmark, bounded cursor implementation, native test result, or live cancellation acceptance is claimed here.

## Scope And Status

The fleet investigation was interrupted by the provider usage limit before implementation. Main-chat source inspection confirmed the boundaries above. The single authentic native validation67153 remains active and must not be duplicated. This report is an implementation-ready requirement record; it is not completion evidence.

## Schema-First Fill Budget Regression

Added editor retained-command fixture tests/fixtures/fill-budget.json (actual taxonomy path uses the existing emoji folders) and the private lib regression retained_fill_pays_for_pixel_work_and_exposes_cancellation_boundaries. For a real 256-square layer, the contract declares original fuel256 per turn, bounded retained-copy/capacity/release grants, at least65,536 paid work units and256 progress boundaries, exact one paint delta, all65,536 changed pixels, exact RGBA sums, and bounded workspace close. The test observes StepContext.original_work_units_spent rather than treating the preparing yield as domain work. No production cursor implementation was added yet.

The retained fill-budget-oracle.mjs uses existing third-party Graphlib Dijkstra reachability on a65,536-node four-connected grid. It independently passed the changed-pixel count and all four channel sums; the same neutral JSON drives Rust. Current synchronous Lowpoly work does not consume algorithm fuel, so source inspection predicts the new paid-work assertion will fail. The Rust regression has NOT run: this is pending red evidence, not an executed red/green claim. Its syntax-format probe required another Cargo setup reservation and was cancelled before formatting; no parse/compiler result is claimed. Scoped diff whitespace check passed.

Important correction: ArtifactCommandWork.extent is staged edit rows, not algorithm-loop work. Returning2 for one invertible edit is intentional. Algorithm fuel/progress must be accounted separately; do not inflate edit extent to disguise pixel traversal.

Concurrent current framework source added required ArtifactCommandInputs.snapshot_read and existing Pending/CompleteDownload step variants. Updated the Lowpoly production destructure to explicitly ignore unused read custody, and all8 pure unit input literals to the public ArtifactCommandSnapshotRead.unavailable operation view (the drifted fixture uses its drifted operation). This does not mint captured Store authority or add a compatibility adapter. The new fill test handles current step variants and allows multiple bounded close turns.

## Existing Pixel Kernel And Actual Host Authority

Located the framework pixel editing module (modules/🔲️pixels/✍️editing) and its existing neutral floodSelections corpus. Its Rust flood_selection is synchronous; TypeScript floodSelection is async, and neither exposes a bounded flood cursor job. Existing PixelEditJob handles full-image edits with explicit pixel grants, but it does not make flood_selection itself cooperative. Raster fill-region also calls the synchronous flood-selection function. No framework source was edited during this inspection. A reusable Rust/TypeScript flood cursor belongs in this existing domain-neutral pixel interface rather than copying a Raster implementation or inventing an unrelated runtime dependency. Preserve Lowpoly direct RGBA replacement semantics; framework fill_in_place performs source-over and cannot replace the existing color law blindly.

The current real MountedOwnerPolicyV1 interactive grant is32 items,32KiB copy,16MiB capacity,1MiB release, depth4096. Corrected the neutral fill grant to those exact physical authorities and bound it to the current public maintenance policy in the Rust regression. Domain fuel is deliberately narrowed to256 to exercise cooperation. Closing now uses actual bounded release authority rather than an arbitrary16KiB release restriction that would strand a legitimately funded image allocation. This is a test-contract correction before any cursor implementation or executed Rust red/green result.
