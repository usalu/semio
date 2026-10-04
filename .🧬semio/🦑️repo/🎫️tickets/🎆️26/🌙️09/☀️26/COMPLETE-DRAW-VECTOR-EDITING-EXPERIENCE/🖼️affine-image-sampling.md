# Affine Image Sampling — 2026-10-03

This continuation implements and integrates filtered affine decoded images in Rust and TypeScript. The editor goal and ticket remain active. Production PNG still uses the reduced SemioDrawing bridge; no end-user export completion is claimed.

## Contract and Implementation

Schema-first source RGBA, output extent/origin, affine matrix and explicit nearest/bilinear/area/auto modes precede the implementations. Twenty-one manually authored neutral RGBA cases cover hidden RGB, premultiplied interpolation, fractional image edges, odd and anisotropic reduction, rotation, shear, reflection, singular transforms and world/crop origins. Both implementations reproduce exact bytes under grants 1/7/4096.

Coverage of the transformed image rectangle is exact through the existing coverage job. Bilinear interpolation weights premultiplied colors, clamps edge color samples and applies boundary coverage. Area filtering clips an inverse destination-pixel quadrilateral against each source pixel cell and weights source alpha/color by exact area. Auto chooses area when the largest inverse singular value exceeds one, including an oblique shrinking direction with unit determinant. Fixed polygon scratch and scalar buffer swaps avoid per-cell allocation. One source cell is visited per reduction work unit. Candidates remain private until complete; failures stay sticky; cancellation drops unpublished buffers without changing previously returned output. TypeScript yields between grants and respects aborts before construction, between progress callbacks and before publication.

Scene integration crops filtered image candidates to the viewport pixel lattice. Exact axis-permutation/reflection matrices with integer translations retain a zero-resampling fast path. Other affine images admit source pixels plus cropped output before allocation, drive a delegated sampler one work unit at a time, and apply layer/group opacity and blend after filtering. Offscreen, singular and transparent-ancestor images skip candidate admission. Twenty-three neutral scene image cases include isolated filtered overlap, premultiplied sampling before layer opacity and oblique reduction. Shared test-only area/native-canvas oracles are close to the sampling domain and have no runtime imports.

## Executed Checks

- TypeScript stub: 62840 exited 1, 24 intended unimplemented failures.
- Native stub: 99979 exited 1 after compilation, intended unimplemented fixture failure; fail-fast stopped its sibling lifecycle group.
- Initial sampler TypeScript 12562: 24 passed, strict production checks passed. Independent polygon-clipping verifies 32 seeded transformed source-cell intersections; native canvas verifies applicable interpolation. Alpha/premultiplied reference differences stay within existing one/two-level bounds; neutral bytes remain exact.
- Full pixels TypeScript 7459: 222 passed, zero failures, strict production checks passed.
- Full pixels Rust 15703: 48 passed, zero skipped.
- Focused native runtime 64853: 2 passed, 46 intentionally filtered/skipped; emitted the twenty-one-case shared RGBA diagnostic.
- Scene TypeScript initial import mistake was corrected before claiming behavioral red. Corrected old nearest-only consumer 30311: 33 passed / 14 failed. Integrated candidate 32696: 47 passed, strict checks passed. Extended scene corpus 6235: 50 passed, strict checks passed.
- First full Draw TS 43177: 357 passed / one failed. Adding an enum to the stored layer schema conflicted with the already-authored literal-string document contract. Corrected the form metadata instead. Full corrected 35311: 358 passed. Final 51797: 361 passed / zero failures, strict production checks, 69-case Ajv field-patch and publication audits passed; PDF reference executed.
- Final TypeScript scalar-swap sampler 96483 exited 0: 24 passed, strict production checks passed. This final scalar-swap change avoids even temporary pair allocation inside the fixed-scratch clipping loop.
- Full Draw Rust 17836 exited 0: 491 passed / zero skipped. Log: `🗑️generated/affine-draw-native-full.txt`.
- Focused native scene runtime 89705 exited 0: 6 passed, 485 intentionally filtered/skipped. Emitted diagnostics confirm all twenty-three filtered image scenes and nineteen painted scenes reproduce native neutral RGBA under grants 1/7/4096, and all fifty-four cropped curve/stroke cases match uncropped output. Log: `🗑️generated/affine-draw-native-runtime.txt`.

## Create Layer Input Schema

Previous scene native 3915 is terminal: 488 passed / one failed / zero skipped. All four scene raster groups passed. The failing runtime history input gate rejected `/layer/blendMode`: enum option labels were attached to a non-enum owned string field.

Stored layer blend identifiers deliberately remain literal strings, as the existing neutral field cases, independent Ajv checks and owned parser tests require. The layer's form annotation now presents its string as text. The Set Layer Blend Mode mutation and inspector still own the sixteen supported enum choices and English/German labels. This fixes the invalid form contract without narrowing the document grammar or weakening the resolver gate. The gate now passes in full native 17836.

## Launch and Shared Workspace

Added focused sampler selections to the existing pixels owner script, included production/source tests in its full suite, exported the native module through its existing glue, and registered TS/native commands in `.vscode/🧩️launch.seed.jsonc`. Registry generation attempts 28904/50565 failed on concurrently changing Flow taxonomy ordering/required preview metadata; this ticket did not edit Flow. After those sources changed, final generator 56951 exited 0. Both seed and generated launch output contain the commands.

Scoped whitespace checks passed. Other staged/unstaged work was preserved. No runtime dependency, extra executable script, modifying Git action, worktree, subagent, process cleanup or browser interaction occurred.

## Required Follow-On Work

Incremental document/trace/boolean preparation; bounded supported-image decoding; full authored font family/weight/spacing and text outlines; production scene-to-PNG scheduling with progress/cancellation and owned final bytes; real browser download/round-trip acceptance. The inspected decoder/font/export prerequisites are recorded in [image preparation](📥️image-decode-preparation.md). Repo ticket tools remain unavailable in this tool catalog; the ticket is kept open. All verification runs from this continuation are terminal; there is no remaining verification handle to poll.

## Source Inventory for This Continuation

- `🧰️framework/🔨️modules/🔲️pixels/🎨️sampling/↗️affine/`: new schema, neutral fixtures, Rust/TS implementations, Rust/TS tests and shared independent test oracles.
- Pixels `📜️script.ts` and `📦️packages/🦀️rust/🦀️.rs`: full/focused checks and native module registration.
- Draw `…/✳️any/🧬️schema/🎬️scene/📷️raster/`: Rust/TS filtered image consumer, neutral image fixtures, TS image tests and native scene tests.
- Draw `…/✳️any/🧬️schema/🔣️.json`: owned literal-string blend form metadata.
- Draw `📦️packages/🟦️typescript/📜️script.ts`: image-scene tests in focused and full suites.
- `.vscode/🧩️launch.seed.jsonc`: focused sampler commands; `.vscode/launch.json` regenerated through its registered owner.
- Ticket affine, decoder-preparation, scene, acceptance and raster-export Markdown reports. Generated verification logs remain under the active ticket's `🗑️generated`; remove those outputs only when the whole ticket is done.

Inventory names this ticket's edits only; other simultaneous changes in shared files were preserved and are not attributed to this work.
