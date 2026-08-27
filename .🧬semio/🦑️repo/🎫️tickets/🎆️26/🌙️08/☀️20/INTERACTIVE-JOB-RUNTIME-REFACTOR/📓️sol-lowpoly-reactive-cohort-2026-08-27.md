# Lowpoly Reactive Cohort

## Scope

This source-only cohort replaces Lowpoly's blanket 47-row `Migrated` claim with the largest bounded subset whose reducer output and Store publication lane can be named exactly. It does not modify Writer, Sequence, Process3d, Flow, master descriptors, shared official reports, or git state.

The baseline is the official live r12 ledger:

- `📊️coordinator-official-tool-jobs-live-r12-working-writer-host-routes-2026-08-27.json`
- Lowpoly rows before this cohort: 47 remaining command rows.
- Common r12 blocker: retained reducer existed, but no exact publishable lane/preparation was accepted.

## Exact Partition

### Migrated — 19

Artifact lane with real app-owned Store one-item preparation:

- `patchObject`
- `addPaintLayer`

Config lane with real app-owned Store one-item preparation:

- `setActiveObject`
- `setActivePaintLayer`
- `setUtilityParam`
- `engagementInput`
- `toggleShowEdges`
- `toggleSun`
- `setSunAzimuth`
- `setSunElevation`
- `setSunIntensity`
- `setCamera`
- `paintSample`

HostOnly lane:

- `importSnapshotJson`
- `setFixtureJson`

Transient lane with exact request-owned transient-generation freshness:

- `paintStrokeBegin`
- `transformBegin`

Config plus Transient lanes:

- `setActiveUtility`

Artifact plus Transient lanes with a persistent 16 KiB paint-diff cursor:

- `paintStrokeEnd`

### BatchOnlyPendingRewrite — 28

- `addPrimitive`
- `extrude`
- `inset`
- `bevel`
- `loopCut`
- `subdivide`
- `triangulate`
- `mirror`
- `decimate`
- `flipFaces`
- `merge`
- `dissolve`
- `snap`
- `toggleSmooth`
- `unwrapActive`
- `markUvSeam`
- `clearSeam`
- `translateSelection`
- `rotateSelection`
- `scaleSelection`
- `paintFill`
- `fillBucket`
- `transformEnd`
- `engagementSubmit`
- `paintStroke`
- `paintAt`
- `canvasPointerDown`
- `canvasPointerMove`

These routes remain honest BatchOnly because their geometry, UV, remaining paint, or transform reducers still lack a bounded operation-owned cursor or exact child/transient publication authority.

## Implementation

The retained work owner now stores the exact tool, operation id, generation, canonical base revision, and request-context identity, which covers transient generation and child-content identity. It emits progress on the first turn, checkpoints every turn in the fixed 88-byte `LPC2` form, restores through explicit replay turns, rejects freshness drift, leaves failed terminal reduction retryable, rejects work after cancellation begins, and closes under item/byte grants.

The factory, proof table, tool IDs, and publication contracts cover only the 19 accepted routes. The manifest classifies the other 28 as `BatchOnlyPendingRewrite`.

`paintSample` performs the exact alpha fold at one selected pixel across at most eight admitted layers instead of compositing an entire 4 MiB texture. `paintStrokeBegin`, `transformBegin`, and `setActiveUtility` publish shallow persistent transient roots without cloning paint or mesh payloads. The two fixture JSON routes are bounded by the 16 KiB retained wire envelope and publish only HostOnly load-document effects.

`paintStrokeEnd` is a real resumable algorithm: it compares 16 KiB of the exact transient-owned before/after buffers per poll, retains bounded sparse runs, checkpoints cursor plus digest, reconstructs those runs during replay, publishes one `EditPaintLayer` mutation plus the finished transient root, and retires partial run owners incrementally on cancellation.

The Artifact preparation admits only the exact mutation vocabulary produced by the Artifact routes: `RenameObject`, `ChangeObjectSmoothShading`, `InsertPaintLayer`, and bounded `EditPaintLayer`. It derives exact inverse and sparse diff from the declared base, applies the diff, bounds retained mutation/base/result bytes, creates one Store-owned edit, and retires each owner under item/byte grants.

The Config preparation covers the complete `LowpolyConfigMutation` vocabulary because the twelve Config-publishing routes can emit its field-setter variants. It validates exact operation/generation/base authority, derives the exact snapshot inverse, prepares one Store-owned item, supports cancellation, and retires bounded owners incrementally.

The previous process-global `OnceLock<(LowpolySnapshot, HashMap<String, String>)>` child payload cache is removed. `default_owned_document()` now creates a fresh caller-owned snapshot/mesh-workspace pair from deterministic primitive serialization. The snapshot owns only its `ArtifactChild` handle; the app session owns the matching payload.

## Language-Neutral Contract

`🧪️interactive-job/🔣️schema.json` is a strict JSON Schema 2020-12 contract. `🧪️interactive-job/🔣️component.json` is the exhaustive 47-route fixture. The schema fixes the owner, poll ceiling, route count, classifications, exact Artifact/Config/HostOnly/Transient lane and Store-preparation tuples, null blockers for migrated routes, and non-empty blockers for BatchOnly routes.

The Lowpoly package's permanent `📜️script.ts` provides:

- an implementation-owned exhaustive source/fixture validator;
- an independent Ajv 2020-12 oracle;
- hostile duplicate-route, missing-lane, empty-blocker, and lane/preparation-mismatch cases;
- source assertions for exact classifications, publication lanes, proof rows, progress, replay, operation/context freshness, chunked paint ownership, checkpointing, bounded close, both Store preparation factories, and removal of the scoped global child payload cache.

## Files

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️component.rs`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️schema.json`
- `✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️component.json`
- this report and `🧪️sol-lowpoly-reactive-cohort-bun-2026-08-27.txt`

## Validation

- Direct Bun owned source/fixture test: exit 0.
- Ajv 2020-12 canonical and four hostile cases: exit 0.
- Separately invoked Ajv 2020-12 canonical/duplicate-hostile oracle: exit 0.
- Scoped `git diff --check`: exit 0, empty output.

Cargo, Nx, rustfmt, rustc, and all compiler paths were intentionally not run because the parent assigned this as a source-only cohort while shared stdio is concurrently incomplete.

## Remaining Blockers

The 28 BatchOnly routes require command-specific resumable algorithms rather than a scan-before-monolith wrapper:

- geometry/UV topology commands need bounded child-document mesh cursors and exact ArtifactChild publication;
- transform gestures need bounded mesh cursors plus exact child publication;
- flood-fill and paint-tick commands need persistent queue/buffer cursors for their 4 MiB transient paint owners;
- engagement submit remains coupled to the unresolved geometry command family.

No claim is made that these routes migrated.
