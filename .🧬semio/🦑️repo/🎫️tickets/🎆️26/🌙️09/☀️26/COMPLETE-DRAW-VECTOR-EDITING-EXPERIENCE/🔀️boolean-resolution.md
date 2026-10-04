# Boolean Resolution

## Current Document Trace Stage — 2026-10-03

[Complete document trace verification](🔍️document-trace.md) supersedes the earlier integration status below. DocumentRasterJob now requires DocumentAlgorithmLimits {booleans, trace, maxWork} and runs preparation → shared-source trace production → Boolean resolution → bounded handoff → private raster output. Native-pixel geometry retains authored transforms/paint; sources decode once and traces can supply Boolean operands. Real nested progress, cancellation, resource limits and a whole-document work cap apply. Only unresolved text still refuses the actual document pipeline; the synchronous resolved-plan helper continues to refuse raw algorithm records.

Full Draw TypeScript **60719 exited 0: 537 passed / zero failed / 1,690,079 assertions / 39 files**, including strict production and enabled independent PDF/SVG checks. Ten neutral actual trace documents have independent PNG/SVG and real document-to-pixels evidence. Native **94613 exited 1** on thirteen shared ZIP/retained-OPC compile errors before Draw; final **44732 exited 1** at its 300-second shared artifact-lock budget without executing Draw tests. New native document behavior is unverified. All this stage's handles are terminal.

Actual editor activation, real fonts/outlines, production encoding/export/import/download and rebuilt browser/multiuser acceptance remain unfinished. The full goal and ticket stay active.

## Earlier Milestone Record

The paragraphs and handle evidence below describe preceding milestones; their remaining-integration statements are superseded by the current stage above.

Latest shared trace storage verification: [document trace ownership](🔍️document-trace-boundary.md), native **49 passed**, TypeScript **140 passed** with strict checks. Native Draw handle **47980 exited 1** before Draw tests on two shared 3D mesh module-path errors, already corrected concurrently when inspected. Native retry **10109 exited 1** at its 300-second budget after an initial artifact-lock wait and then framework compilation; no Draw tests ran. All this stage's verification handles are terminal.

## Actual Document Boolean Stage — 2026-10-03

Actual document Boolean resolution now runs in the document-to-pixels API. See [document Boolean verification](🔀️document-booleans.md): thirteen neutral complete drawings, independent SVG pixels, nested/shared result-accuracy propagation, grant limits and cancellation; final full TypeScript **519 passed / zero failed**, strict checks and enabled PDF oracle. Native handle **28538 exited 1** after the 300-second budget expired in shared artifact-lock wait; no new native Draw test pass is claimed. Text/trace, actual editor/export integration and browser/multiuser acceptance remain unfinished. The goal and ticket remain active.

## Verified Kernel and Curve Preparation — 2026-10-03

The previous turn made concrete progress: it replaced the native planar kernel, added its TypeScript twin and shared schema/fixtures, implemented a bounded curved-path producer in both languages, and verified the complete geometry owners. The full editing goal and existing ticket remain active.

`BooleanJob` consumes filled contours with an explicit nonzero/evenodd rule for every operand. Union, difference, intersection and XOR preserve holes and accept legitimate empty inputs/results. Source admission, Morton/BVH construction and queries, intersections, unique parameter splitting, snapped atomic edges, winding classification, contour stitching, collinear compaction and canonical emission advance under caller work grants. Explicit source/parameter/atomic/output/work limits precede publication. Outputs stay private, failures remain sticky and cancellation releases candidates. TypeScript async operations yield and recheck abort after completion observers.

The absolute epsilon contract refuses values below coordinate precision. A neutral subnormal-contour regression first failed with a nonfinite closest-point projection; normalized length projection now avoids squared-length underflow in both implementations. Short collapsed atomic edges are rejected before vertex interning, preserving valid cap accounting.

`PathBooleanJob` admits one source segment per grant, runs real adaptive quadratic/cubic/SVG-arc preparation with tolerance measured after the complete affine matrix, transforms one resulting point per grant, and enters the region job. The same global work budget covers every producer stage. Aggregate source segments and flattened points share the explicit edge cap. Open filled contours close implicitly, and reflected/sheared/collapsed transforms retain fill semantics. Sparse/null operand and segment entries now refuse instead of silently truncating the operation. The input remains stable and is copied incrementally. This is a real geometry producer; it is not yet a document or editor integration.

The existing synchronous native linear-path entrypoints now use the bounded region implementation, validate operations and aggregate admission, honor nonzero winding, publish empty results and explicitly refuse curves that require preparation. Their old opposite-oriented/repeated-close expectations and empty-result refusals were corrected by hand.

## Evidence

- Region contract: 29 neutral fixtures, grants 1/7/4096, authored holes/repeated winding/self intersections/empty operands, translated and tiny geometry, exact cap admission, sticky refusals and cancellation in every public phase. Independent test-only polygon-clipping and Sharp SVG engines validate filled results.
- Curved-path contract: eight neutral fixtures and 13 TypeScript tests; both owners cover quadratic/cubic/arc geometry, full matrices, fill rules, open contours, collapsed/empty output, resource/work caps and cancellation. Independent Sharp SVG operand masks validate the curved output.
- Sparse geometry fixture: 128 separated squares require 512 candidate edge pairs and 73,930 granted operations, establishing that admission does not perform a global eager all-pairs scan.
- Runtime `[DEBUG]` output confirms identical grant results and geometry work. The transformed-rectangle curve fixture completes at 689 operations; the reflected quadratic case uses 18,651; cubic difference 18,505; arc XOR 57,876. These are logical work bounds, not hard real-time allocator guarantees.
- Final strict TypeScript geometry owner, handle **70355**, exited **0**: **139 passed / seven files**. Log: `🗑️generated/boolean-path-ts-final-strict.log`.
- Final native geometry owner, handle **77320**, exited **0**: **48 passed / zero skipped**. Log: `🗑️generated/boolean-path-native-final.log`.
- Full Draw TypeScript, handle **74625**, exited **0**: **496 passed / zero failed / 256,642 assertions / 37 files**, plus field-patch, publication-authority, scheduler and enabled PDF checks. Log: `🗑️generated/boolean-path-draw-ts.log`. This run precedes the final malformed-entry admission guard; the final strict geometry gate covers that guard.
- Native Draw handle **20704** exited **1** after its 240-second build budget expired waiting for the shared Cargo artifact lock. No Draw tests ran.
- Native Draw retry **47793** exited **1** after compilation reached shared DXF/PDF error boundaries, including **315 errors in semio-s-artifact-stdio-pdf**. Typed ValueError-to-String conversion and type mismatches block Draw compilation; no Draw tests ran. Log: `🗑️generated/boolean-path-draw-native.log`. The retry followed a successful fresh geometry build and changed Rust producer sources. No native validation handle remains live.

## Files

Production schema, fixtures and Rust/TypeScript twins live under `🧰️framework/🔨️modules/◻️2d/🔀️booleans` and `🔀️booleans/🛤️paths`; each has `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`, implementation and `🧪️tests` files. The region owner retains updated existing native unit tests. Shared `🟦️.ts` exports both jobs. The native booleans module mounts the producer. The geometry test config and existing TypeScript `📜️script.ts` include production strict checking and both suites; existing Rust/TypeScript project inputs include the entire boolean taxonomy. The flatten schema now declares the stable ID referenced by the path contract. Existing generated launch entries `◻️2D 🧪️geometry 🟦️typescript` and `◻️2D 🧪️geometry 🦀️native` already invoke these owner gates; no executable command or runtime third-party dependency was added.

## Remaining Actual Document and End-User Work

DocumentSceneJob publishes explicit Boolean/trace/text records. DocumentRasterJob now resolves actual traces before Booleans through the bounded document owners; text remains refused until real font geometry is produced. The legacy document helper still eagerly flattens, applies only child-local transforms, filters empty operands and swallows failures. Those behaviors must be replaced by bounded actual document resolution, not routed around.

The actual document resolver now verifies every operand, shared geometry ownership, complete ancestor transforms, per-operand fill rules, nested group/Boolean references, cycle/refusal semantics and result-local paint. Its thirteen actual document fixtures cover the reference graph and transform semantics: CombineBoolean currently stores references to selected layers as a separate root layer, leaving operands in their existing forest, and the result layer has an independently editable transform. A world-space conversion followed by its inverse alone would cancel that result transform; do not inadvertently make result-layer movement ineffective. The baseline and ancestor/result transforms are now verified in document raster; actual editor/export activation remains required.

The shared trace kernel now accepts moved native masks with verified cancellation release; actual decoded-source/document wiring remains required. Real font shaping/outlines, responsive encoding/import/export, editor scheduling/history/cancellation, browser journeys and multiuser/reconnect acceptance remain required. No browser interaction occurred in this stage. Pure geometry passes do not establish an end-user algorithm workflow.

Repo ticket tools and `repo://goals` are unavailable in the enabled tool set. This work continues in the existing ticket folder; no ticket lifecycle or repo-goal mutation is claimed. Generated evidence remains inside the active ticket, and research/input files are preserved.
