# Actual Document Trace Boundary

## Current Document Trace Stage — 2026-10-03

[Complete document trace verification](🔍️document-trace.md) supersedes the earlier integration status below. DocumentRasterJob now requires DocumentAlgorithmLimits {booleans, trace, maxWork} and runs preparation → shared-source trace production → Boolean resolution → bounded handoff → private raster output. Native-pixel geometry retains authored transforms/paint; sources decode once and traces can supply Boolean operands. Real nested progress, cancellation, resource limits and a whole-document work cap apply. Only unresolved text still refuses the actual document pipeline; the synchronous resolved-plan helper continues to refuse raw algorithm records.

Full Draw TypeScript **60719 exited 0: 537 passed / zero failed / 1,690,079 assertions / 39 files**, including strict production and enabled independent PDF/SVG checks. Ten neutral actual trace documents have independent PNG/SVG and real document-to-pixels evidence. Native **94613 exited 1** on thirteen shared ZIP/retained-OPC compile errors before Draw; final **44732 exited 1** at its 300-second shared artifact-lock budget without executing Draw tests. New native document behavior is unverified. All this stage's handles are terminal.

Actual editor activation, real fonts/outlines, production encoding/export/import/download and rebuilt browser/multiuser acceptance remain unfinished. The full goal and ticket stay active.

## Earlier Milestone Record

The paragraphs and handle evidence below describe preceding milestones; their remaining-integration statements are superseded by the current stage above.

Trace preparation must consume the real encoded asset catalog under image-source/PNG grants, convert each RGBA pixel into alpha-weighted luma under grants, then move the exact private mask into the trace kernel. It must retain native dimensions and authored layer transforms; implicit image resizing and artboard scaling are unsuitable for a predictable editable object. Local paint and inherited group metadata remain intact.

The TypeScript trace job already owns a stable byte reference supplied by its parent. The native job currently borrows a mask, which prevents a parent from retaining both a private mask and its child job without a self-reference. Change the first-party input/job to accept any system AsRef byte buffer, store its mask as an optional owned generic value, and release it on cancellation. Borrowed slices and moved Vec buffers use the same algorithm and neutral fixture outputs; no unsafe pointer, cloning adapter or runtime library is needed.

The document owner needs source identity indexing, shared decode-once ownership, exact cumulative decoded-pixel/source budgets, a pixel-by-pixel luma conversion, source/trace nested progress, retained path-segment and total-work admission, private publication and cancellation. Resolve trace nodes before Boolean dependencies so trace results may serve as actual Boolean operands. Real text outlines remain a separate unfinished stage.

Existing synchronous editor helpers decode PNG through an external image library, resize to 256 pixels, multiply luminance by alpha, swallow failures and scale contours to the inferred artboard. These remain unverified editor behavior and must be replaced by retained snapshot-owned publication when activating the new document pipeline. These opening design paragraphs record the boundary proposed before implementation; the current trace stage now implements and verifies it in TypeScript.

## Verified Mask Ownership — 2026-10-03

The native input/job now accepts a system AsRef byte buffer, including borrowed slices, moved Vec buffers and a custom tracked first-party owner. The job retains an optional buffer value and drops that value on cancellation. There is no unsafe self-reference, extra data clone, compatibility adapter, external runtime dependency or unstable compiler feature. The portable byte/geometry schema is unchanged.

The nineteen existing neutral fixtures now also run with moved native buffers and TypeScript Uint8Array buffers at grants 1/7/4096. Outputs equal the independently polygon-clipping/Sharp-validated fixture paths. TypeScript cancellation preserves caller bytes and published geometry. A native tracked drop owner confirms one immediate mask release at every cancellation phase, with private incomplete/cancelled results.

- Native red **76819 exited 1** on the two intended borrowed-only mask type failures before implementation; log `🗑️generated/trace-owned-native-red.log`.
- Intermediate native **40697** and **81248** exited 1 on type-changing struct update/coercion errors. Fixed through explicit field ownership and slice types; no nightly feature was enabled.
- Final native **99413 exited 0**, **49 passed / zero skipped**, including the new moved-storage lifecycle group and `[DEBUG]` storage-release confirmation. Log: `🗑️generated/trace-owned-native-verified.log`.
- Geometry TypeScript **6045 exited 0**, **140 passed / seven files**, strict production checks; `🗑️generated/trace-owned-ts.log`.
- Actual Draw native **47980 exited 1** on two shared 3D mesh module-path errors before Draw tests; log `🗑️generated/document-boolean-after-owned-trace-native.log`. The shared checkout already corrected those two references when inspected; native retry **10109 exited 1** after its 300-second budget expired following initial artifact-lock wait and framework compilation; no Draw tests ran. All verification handles are terminal; log `🗑️generated/document-boolean-native-after-3d-repair.log`.

Changed files are `🧰️framework/🔨️modules/◻️2d/🔍️trace/🦀️.rs` and the existing TypeScript/native bounded test files under its `🧪️tests` directory. Existing framework geometry launch commands execute these tests. Decoded-image/document trace production and editor activation remain unfinished; the goal and ticket remain active.
