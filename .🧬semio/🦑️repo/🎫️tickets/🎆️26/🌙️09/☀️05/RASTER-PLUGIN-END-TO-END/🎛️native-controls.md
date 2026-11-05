# Native Algorithm Controls Investigation

This continuation read the acceptance and native-selection notes, then attempted to locate the current native Raster algorithm controls before modifying them. No implementation or tests were changed, and no new verification pass is claimed.

The observed Raster plugin directory contains commands, bridge, packages, oracles, artifacts and tests. The actual editor is under the Raster artifact's standard-1 any subset. The initial bounded-depth search missed this deep taxonomy; the source is now located.

New sandboxed `repo tools exec` commands exit without executing their payload because automatic agent attachment cannot create a lock under `/home/deck/.cache/semio/agent-sessions`. A previously approved exact source-reading command still executes. Escalation was stopped following the instruction to avoid repeated permission prompts. A repo MCP read attempt was cancelled; no further permission-triggering attempts were made.

The native-selection agent owns the WGPU paint surface and selection gesture integration.

## Source Implementation

The supported direct `repo <ticket>/inspect.ts` script ingress restored source access without permission prompts or changing any external cache. No approval is required by this workflow.

Inspector now offers Fill Selection, invert/grayscale/clear, flips, both quarter-turns, all thirteen filter choices with a bounded amount control and explicit Apply, and intrinsic resize/crop dimensions, crop origin and bilinear sampling controls. Resize and crop dispatch `transformImage`; filter actions dispatch `applyFilter`; fill dispatches `fillSelection`. Commands already attach the authoritative completed pixel selection and reject protected or incompatible targets. Filter/transform actions are disabled during mask editing, while selection fill supports pixel and group masks.

`filterKind`, `filterAmount` and `imageGrid` are session configuration, separate from document history and pixel scenes. Their schemas are authored in JSON Schema, Rust, TypeScript, GraphQL and Proto. Atomic `SetFilterOptions` and `SetImageGrid` config mutations support exact inverses. `setFilterKind`, `setFilterAmount` and `setImageGrid` are registered config-only retained commands. Numeric text commits are decoded explicitly; changing an option alone does not edit pixels. The closed command census is now 33, including export; 32 routes use the retained command factory. Ellipse and lasso utilities are exposed for the separate native gesture integration.

## Authored Verification

Language-neutral fixtures cover exact operation command arguments/localized labels, every filter's valid/invalid amount, and grid values/bounds. Native Inspector tests assert exact semantic targets, parameter bounds and publication payloads; config tests assert exact inverse and text/binary round trips. TypeScript parser tests compare the same filter/grid vectors with independent Ajv JSON Schema validation. Existing shared pixel/oracle tests continue to own algorithm correctness; this control implementation adds no pixel math or runtime dependency.

These tests have **not executed successfully in this continuation**. The initial red Inspector contract was authored before controls, but the parent test target stopped at source admission before executing tests. Native compilation, end-user runtime and zero-skip suite results are still required. Do not report authored assertions as passing evidence.

## Remaining Requirements

Current parametric filter/transform mutation `diff` and `inverse` paths synchronously clone source pixels and loop `PixelEditJob` to completion. The generic command factory is classified as bounded-first-step; that classification does not establish bounded image work. Correct progress/cancellation requires retained original mutation preparation across normal application, history replay and inverse derivation. Precomputing pixels in a host would bypass the semantic history model and is not an acceptable repair.

React's overlay retains separate local algorithm option staging; these new Inspector config settings do not establish cross-host option retention. Native UI tests/builds, practical control layout, progress/cancellation, live history/save/reload and the broader acceptance matrix remain open.

## Retained Pixel Preparation In Progress

The parent subsequently wired required shared scene editing options and controlled React filter/grid settings. Their test execution remains unverified; that source integration supersedes the preceding private-option staging observation.

Added a neutral host contract for all six semantic pixel routes on a 1024×1024 materialized source, one fuel unit and 32 KiB copy currency per turn. The new `semantic_pixel_routes_yield_during_large_image_work` test requires at least 128 grants and verifies publication does not mutate the original source. It is expected to fail against the still-synchronous host dispatch; no executed red result is claimed.

Added `RasterPixelSourceWork`, a reusable exact-base source owner in the Raster host. It quotes descriptor copying, full contiguous reservation, bounded 32 KiB source initialization/copy and mask normalization separately. It rejects a changed base/frame, withholds partial source results and closes descriptors/partial bytes through generic supplied-grant retirement. Neutral source cancellation cuts and receipt checks are authored but unexecuted.

The shared pixel core now transfers exact private storage through `PixelEditJob::into_storage`, `PixelSelectionJob::into_storage` and `PixelFloodSelectionJob::into_storage`. Shape/flood jobs expose live initialized/capacity measurements; flood queue capacity can be reserved explicitly before traversal. This avoids calling eager cancellation methods from retained owners before their storage has been transferred to explicit retirement.

Added `RasterImageContentHash`, which processes at most 32 KiB of canonical image fields per call, including length prefixes, metadata, profiles and frame bytes. The cold identity helper delegates to it. A new neutral image vector has independent Node crypto digest `6a5b078c041617b1`; the ticket's `hash-oracle.ts` produced that digest successfully through the supported direct repo script ingress. Native arbitrary-grant hashing assertions are authored, **not executed**. Running the oracle does not prove the Rust implementation or the normal project test targets.

Still required in this phase: metadata/extra-frame copying, retained algorithm/selection/flood orchestration, bounded result comparison and adoption, source restoration/inverse preparation, six-route host integration, resource-denial/cancellation tests and native verification. The currently mounted host candidate still calls the cold pixel mutation branch. Do not claim source/hash helpers alone establish end-user cancellation.

## Bounded Compute and Result Owners

`RasterPixelComputeWork` now retains all six semantic pixel routes through granted source lookup/copy, RLE coverage validation/expansion, shared edit/flood computation and explicit storage transfer. Source discovery uses a fixed-depth granted DFS rather than recursive whole-tree lookup, propagating inherited visibility and lock state. Geometry-only callers can stop before allocating image storage. Flood clipping, scratch and queue reservations, and storage retirement are explicit. The neutral fixture also describes the maximum 4,096-point stroke: point storage is reserved once and its 64 KiB content is validated/copied in pages of at most 32 KiB. `paint_stroke::operation_with_points` adopts that prepared storage without another point copy.

`RasterImageCopyWork` copies schema, profiles, metadata, frame headers and additional frames in separate granted phases; it adopts a same-extent first-frame replacement without copying its pixels. `RasterPixelResultWork` composes computation, bounded source/output no-op comparison, metadata-preserving image preparation, canonical hashing and key creation. Existing destination assets require exact canonical image equality before reuse, including headers, metadata and every frame; the 64-bit content key alone is not accepted as proof. Completed transform results use fresh single-frame images, matching the existing semantic behavior. All three workers retain unfinished storage and have explicit supplied-grant close paths. These owners are not yet mounted in the original six-route candidate branch, and currently have no user-visible cancellation effect.

Cold full-pool replacement now accounts for removals when validating net asset capacity and removes released entries before inserting new images. This permits an exclusive replacement at the existing 64-asset limit without overflowing an intermediate map. Shared original assets still require their own retained slot. A neutral host regression is authored for this case. This correction concerns private atomic candidate arithmetic, not a capacity increase.

The store's original one-item declaration still has a 1 MiB envelope, while full decoded image values and inverse payloads can be much larger. Parent work is introducing a transparent immutable image payload that preserves actual durable image bytes while sharing custody safely; publication/inverse integration must use that contract. No blanket budget increase or handle-only event payload is acceptable. Source, compute, result, collision and full-pool assertions remain **unexecuted** because normal test source admission still fails before tests. This continuation has only read source and authored code/tests; no compiler or runtime proof is claimed.

## Mounted Retained Publication Checkpoint

The preceding unmounted-owner status is superseded by source integration. `RasterPixelMutationWork` now composes the source/compute/result owners with granted attachment capture, a reference census, funded immutable output custody, exact child headers, capacity-safe asset replacement and inverse preparation. `RasterAssetMutationWork` similarly handles Add/Remove asset leaves with incremental image hashing and exact original-root aliases. The mutation candidate dispatches all eight routes through these owners; the former cold pixel diff/apply and clone/hash asset mint branches have been removed.

`RasterOneItemApply` now hands off `RasterPreparedMutation { snapshot, inverse }`. Editor preparation consumes the cached inverse for those routes, avoiding a second synchronous pixel computation or source clone. Normal replay uses the same private application owner without preparing an unnecessary inverse. Cancellation/fault closes every partial result and pending inverse, and the dynamic owner Box's final release is separately funded. Full-pool undo detaches/removes the new image before restoring the old asset; all six pixel schemas admit up to four inverse rows.

The shared pixel edit job retains unfinished stroke-row indexing and offers `advance_bounded`; a 4,096-point row no longer needs 32,772 bytes in one 32 KiB callback. Flood jobs expose actual visited/initialized work units in addition to physical write receipts. Config nonselection tests now apply sparse diffs to concrete snapshots, and selection-bearing command fixtures use genuine original receiving and explicit funded retirement.

These are source changes, **not passing native evidence**. The capacity audit in [Pixel Preparation Capacity Authority](🎟️pixel-capacity.md) distinguishes the real native lifetime source from mounted callback limits and records the remaining descriptor-budget and caller-policy admission work. Live progress checkpoints, cancellation UI, full native compilation, zero-skip tests and runtime history/reload remain unverified.
