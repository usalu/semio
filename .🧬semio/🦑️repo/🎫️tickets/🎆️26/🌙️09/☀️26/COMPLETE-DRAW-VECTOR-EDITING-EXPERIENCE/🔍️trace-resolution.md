# Bitmap Trace Resolution

## Current Document Trace Stage — 2026-10-03

[Complete document trace verification](🔍️document-trace.md) supersedes the earlier integration status below. DocumentRasterJob now requires DocumentAlgorithmLimits {booleans, trace, maxWork} and runs preparation → shared-source trace production → Boolean resolution → bounded handoff → private raster output. Native-pixel geometry retains authored transforms/paint; sources decode once and traces can supply Boolean operands. Real nested progress, cancellation, resource limits and a whole-document work cap apply. Only unresolved text still refuses the actual document pipeline; the synchronous resolved-plan helper continues to refuse raw algorithm records.

Full Draw TypeScript **60719 exited 0: 537 passed / zero failed / 1,690,079 assertions / 39 files**, including strict production and enabled independent PDF/SVG checks. Ten neutral actual trace documents have independent PNG/SVG and real document-to-pixels evidence. Native **94613 exited 1** on thirteen shared ZIP/retained-OPC compile errors before Draw; final **44732 exited 1** at its 300-second shared artifact-lock budget without executing Draw tests. New native document behavior is unverified. All this stage's handles are terminal.

Actual editor activation, real fonts/outlines, production encoding/export/import/download and rebuilt browser/multiuser acceptance remain unfinished. The full goal and ticket stay active.

## Earlier Milestone Record

The paragraphs and handle evidence below describe preceding milestones; their remaining-integration statements are superseded by the current stage above.

The active Draw goal requires algorithm layers to resolve into usable vector geometry. The document preparation API currently retains trace and boolean records but refuses them at raster resolution. This stage examines the existing algorithm owners and replaces bitmap tracing with a bounded, cancellable implementation in both TypeScript and Rust before connecting it to document resolution.

The existing tracing function scans the whole bitmap eagerly, builds all edges at once, recursively simplifies contours, clamps invalid parameters, and treats an empty foreground as an error. Its edge tracing is deterministic pixel boundary extraction rather than marching squares. The new contract will state that geometry explicitly, preserve holes and isolated single pixels, reject invalid inputs, and publish paths only on completion.

Repo ticket tools remain unavailable in this session. The existing ticket and overall goal remain active; no ticket lifecycle operation is claimed.

## Completed Kernel

The shared 2D owner now contains schema-first TypeScript and Rust `BitmapTraceJob` implementations. The Rust synchronous `trace_bitmap_paths` entry point delegates to the same job. TypeScript additionally exposes `traceBitmap` with yielding, AbortSignal and progress callbacks. No third-party runtime dependency was added.

The input explicitly requires width, height, exact mask bytes, threshold, simplification epsilon, maximum pixels, maximum edges, maximum output segments and maximum work. Dimensions are bounded by 8192 per axis, total pixels by 16,777,216, edges and output segments by 65,536, and work by one billion. Threshold is an inclusive comparison with the nearest byte value; threshold zero intentionally includes zero-valued mask bytes. This is a mask/luma kernel: image decoding, alpha policy and authored trace dimensions belong to the scene resource/resolution boundary.

Each logical grant advances one pixel, contour edge, split vertex, compacted vertex, Douglas–Peucker sample, topology pair, scanline edge or emitted segment. Scanning creates exact pixel-cell boundaries, separates diagonal foreground components, decomposes corner-touching holes into distinct contours and removes collinear points. Douglas–Peucker subdivision uses an explicit stack over two anchored arcs. A candidate must preserve contour orientation, avoid edge crossings/overlaps and retain every original foreground/background pixel center. Rejected simplification retains exact compacted contours. Small regions and holes are never removed by an area threshold. A blank bitmap publishes an empty path array.

Scanline coverage comparison takes work proportional to image height times candidate edges plus pixels. Pairwise candidate topology checking is quadratic in candidate edges; the explicit work cap supplies a deterministic refusal, and grants/cancellation keep it cooperative. Progress does not pretend to be a percentage or guarantee hard real-time allocator latency.

Caller source bytes must remain a stable snapshot for TypeScript. Rust now accepts borrowed or owned first-party AsRef buffers; the later ownership report records the 49-test native verification. Neither implementation mutates input or returns partial paths; invalid inputs and cap failures remain sticky. Cancelling clears private work and refuses subsequent access. Already published paths survive cancellation. Source decoding into an owned/native child job will require an explicit ownership handoff; self-referential borrows must not be introduced during scene integration.

## Verification

The initial implementation stub was verified red: handle **73501 exited 1**, all **20 trace tests failed** on the missing kernel. After implementation, focused **25176 exited 0** and strict-production scanline **63748 exited 0**, each with **20 passing trace tests**.

The exhaustive independent polygon test then exposed a genuine defect: **84523 exited 1**, **89 passed / one failed**. Bitmap 239 had a hole touching the exterior at one corner; the initial walk emitted a self-touching contour. The bounded repeated-vertex split was added to both owners and the case became a neutral fixture.

Final full 2D TypeScript **85288 exited 0: 91 passed / zero failed / five files**, with strict production checks. Full native 2D **30762 exited 0: 40 passed / zero skipped**. These include **19 neutral fixtures** under grants 1, 7 and 4096, every phase cancelled privately, exact cap checks, invalid parameters, an accepted simplification with hand-authored path output, wide blank linear work, and all **512 three-by-three bitmaps at three tolerances**. TypeScript compares independent polygon-clipping topology/areas and Sharp SVG pixels; Rust emits runtime `[DEBUG]` fixture and exact-limit evidence.

Broader Draw TypeScript **35616 exited 0: 496 passed / zero failed / 256,642 assertions / 37 files**, plus strict production and field/publication/scheduler checks. The enabled PDF/SVG reference executed successfully. Broader native Draw **4547 exited 1** after the build lock became available: shared dependencies compiled through Stdio Semio, then Rust could not write its metadata because the volume had **no space left**. No Draw native test executed. Unlike the preceding continuation's 1,239 typed-error compilation failure, this attempt showed no Rust type error before the metadata write failure. The isolated 2D native pass does not establish native Draw or browser behavior. All verification handles are now terminal.

Launch regeneration **26836 exited 0**; both new geometry-owner commands exist in the source seed and generated launch file. Scoped whitespace verification exited 0.

After the native metadata failure, the volume reported 284 MiB available. Two inactive generated Nx workspace caches owned by this ticket (`🗑️generated/nx` and `🗑️generated/nx-current`) were checked with `lsof` and removed. No open handle was found, the cleanup command exited 0, and available space increased to 641 MiB. Current `nx-oct02`, inputs, reports, shared Cargo caches and other developers' processes were preserved. No broad cleanup command was run. This is still insufficient headroom to claim a completed large native assembly, so an unchanged full native retry was not started.

## Remaining End-User Work

The current document raster API resolves traces and Booleans through actual bounded producers; raw text remains refused. This kernel is an algorithm foundation, not a completed trace tool, image import flow or export workflow. Scene integration must admit/decode shared resources once, map pixels to intrinsic/authored geometry explicitly, carry full transforms/paint, resolve algorithm operands, forward nested progress and publish only a current complete result. The existing document model exposes only threshold and epsilon; foreground/alpha/scale controls need an explicit authored contract rather than hidden artboard scaling or asset overrides.

Full boolean resolution, typography, editor import/export/progress/cancellation wiring and rebuilt browser/multiuser acceptance remain required. The goal and ticket remain active. No browser interaction occurred in this stage.

The existing boolean owner and tests have now been inspected; [the follow-on boolean report](🔀️boolean-resolution.md) records its missing fill-rule/progress/empty-result behavior and the intended replacement boundary. No boolean implementation was changed during trace verification.

## File Inventory

- Created `🧰️framework/🔨️modules/◻️2d/🔍️trace/🧬️schema/🔣️.json`.
- Created `🧰️framework/🔨️modules/◻️2d/🔍️trace/🧫️fixtures/🔣️.json`.
- Created `🧰️framework/🔨️modules/◻️2d/🔍️trace/🟦️.ts`.
- Replaced the implementation in `🧰️framework/🔨️modules/◻️2d/🔍️trace/🦀️.rs`.
- Created `🧰️framework/🔨️modules/◻️2d/🔍️trace/🧪️tests/🟦️.ts`.
- Created `🧰️framework/🔨️modules/◻️2d/🔍️trace/🧪️tests/🦀️.rs`.
- Updated `🧰️framework/🔨️modules/◻️2d/🔍️trace/🧪️tests/🔬️unit/🦀️.rs`.
- Updated `🧰️framework/🔨️modules/◻️2d/🟦️.ts`.
- Updated `🧰️framework/🔨️modules/◻️2d/🧪️tests/🎚️config/🟦️.ts`.
- Updated `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📜️script.ts`.
- Updated `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📋️project.json`.
- Updated `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/📋️project.json`.
- Updated `.vscode/🧩️launch.seed.jsonc` and regenerated `.vscode/launch.json`.
- Created this report and `🔀️boolean-resolution.md`; updated `📋️acceptance.md` and `📋️scene-preparation.md` in this ticket.
- Removed the inactive generated ticket workspace cache directories `🗑️generated/nx` and `🗑️generated/nx-current`.

Logs stay in the active ticket's generated directory. No ticket input/report was removed, no modifying Git operation/worktree/AGENTS edit occurred, and unrelated shared edits were preserved.
