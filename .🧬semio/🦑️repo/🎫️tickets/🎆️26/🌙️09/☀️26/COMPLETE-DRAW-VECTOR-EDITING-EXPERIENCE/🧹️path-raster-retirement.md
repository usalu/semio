# Painted Path Stroke Cleanup

## Contract and Scope

The actual Draw PathRasterJob currently computes a complete stroke and immediately drops the producer when coverage adopts its polygons. This stage replaces that successful child handoff with a consuming transfer and an explicit work-granted `strokeCleanup` phase before fill/stroke coverage starts. The private polygon output moves once; downstream coverage cannot run while the real stroke owner is still closing. Both Rust and TypeScript consume the same neutral path cases and retain the current painted RGBA expectations and independent SVG/Canvas oracles.

This is a successful stroke composition boundary. Flatten/coverage/paint ownership and whole-parent failure/cancellation retirement remain required; no complete painted-path cancellation or allocator byte admission is claimed. Current source/query/native-browser acceptance also remains unfinished. The full goal and ticket remain active.

## Verification

Six hand-authored neutral sources cover solid closed-path strokes, dashes, zero-length round caps, fill-only rendering, no appearance and a singular transform. Cleanup work totals count the actual ten fixed stroke slots plus retained source/run owners; native additionally owns its input contour. Grants 1/7/4096 must preserve the real polygon buffer during closure and match current neutral pixels and independent SVG output. Invalid grants must preserve that child/output ownership. Interrupting cleanup must refuse partial pixels.

Initial full Draw TypeScript **22575 exited 1: 579 passed / two failed**. One failure correctly observed the absent strokeCleanup phase, while the initial progress validator mistakenly applied the complete input schema to progress. That validator was corrected to reference only the schema definitions before the implementation. Log: `🗑️generated/path-raster-stroke-handoff-red.log`. This is a recorded test-harness correction; that first validator failure is not attributed to a production defect.

Corrected registered path-raster red **3856 exited 1: 26 passed / two failed**, both observing the absent producer cleanup phase. Log: `🗑️generated/path-raster-stroke-handoff-schema-fixed-red.log`. An initial unsupported argument selection **39197 exited 1** at the existing script's argument guard; it executed no assertions and is not test failure evidence. The registered `path-raster` selector was used for the corrected red gate.

Rust and TypeScript now consume the completed stroke owner into a retained private retirement frontier and a separate polygon buffer. Each parent step gives that actual child exactly one work unit. A separate later parent step checks terminal emptiness, releases the empty frontier and starts fill coverage. Stroke coverage subsequently takes the moved polygons; no synchronous native stroke into_result or exposed TypeScript stroke result call remains in the parent. The existing successful output, geometry limits and paint behavior remain covered by all twenty painted sources and 162 independent curved stroke cases.

Full Draw TypeScript **47795** and native **86177** are tracked in `🗑️generated/path-raster-stroke-handoff-green.log` and `🗑️generated/path-raster-stroke-handoff-native.log`. Results remain unverified until their handles terminate. The shared Draw workspace lock now contains bitflags 2.13.2; that concurrent change supplies evidence for rerunning the native gate after the preceding lopdf resolver failure. This task did not edit either Cargo lock or downgrade the oracle.

TypeScript **47795 exited 1 after all 581 assertions/tests passed** because its strict checker rejected an invented stroke-specific retirement type import. The kernel exposes the real shared WorkRetirement interface; the parent now imports that exact first-party interface directly. A fresh full gate follows this narrow type-only repair. Actual stderr already confirms eighteen six-source/grant SVG comparisons through the production handoff in the failed strict gate; no full green result is claimed for that handle.

Final full Draw TypeScript **8518 exited 0: 581 passed / zero failed / 1,922,734 assertions**, strict production checks and enabled independent PDF/SVG gates passed. Its real stderr confirms eighteen six-source/grant comparisons through the actual child handoff. Log: `🗑️generated/path-raster-stroke-handoff-owner-type-fixed.log`. Native **86177** remains the only live validation handle; it has passed metadata resolution and is awaiting shared artifact build access. No native painted-parent pass is claimed while pending.

The native observation law deliberately steps through child closure one parent unit at a time so it can read the real terminal child work total before its empty header is released. It then continues downstream at the fixture's 1/7/4096 grant. The existing twenty-source native pixel law exercises the whole pipeline at all three grants. TypeScript instruments and delegates the actual child frontier during all three parent grants, enforcing no downstream coverage while that child advances. Neither test invents cleanup totals from the expected fixture.

## Current Native Terminal Audit

Handle **86177 exited 1**, reporting shared JSON artifact DSL record/derive prerequisites and a private protocol ValueError path; it did not compile Draw or run any current Draw assertions. Current JSON manifests/source now contain those direct dependencies and public first-party error ownership after concurrent edits. This task did not make or attribute those edits. All earlier mentions of 86177 as live are historical; there is no live duplicate of that gate.

Fresh full Draw native **92563 exited 1** on shared PNG value/DSL/controlled Deflate prerequisites before Draw compilation. See current [coverage evidence](🧹️coverage-retirement.md). Full Draw TypeScript **55353 passed 582 tests** with strict and independent PDF/SVG gates after successful fill/stroke coverage cleanup composition. All stage handles are terminal; full editor completion and current native/browser acceptance remain unclaimed.
