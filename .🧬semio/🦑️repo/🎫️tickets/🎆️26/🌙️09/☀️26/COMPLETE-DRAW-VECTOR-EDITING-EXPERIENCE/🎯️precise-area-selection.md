# Precise Area Selection

## Source Inspection

The prepared scene picker uses cached painted regions for point selection, including fill, stroke, Boolean results, and traced geometry. Rectangle and lasso selection currently use only each node's axis-aligned painted bounds. This permits crossing rectangles in a hollow region or empty corners of a path's bounds to select artwork that has no paint there. Lasso containment also tests bounds instead of the actual painted boundary.

Source: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🎨️paint/📋️prepare/🎯️query/🦀️.rs` and its TypeScript counterpart.

The picker already enforces cache identity, bounded grants, cancellation, private partial results, locked ancestry, visibility, and a 256-result limit. Any area-query refinement must preserve these contracts in Rust and TypeScript. The existing language-neutral corpus covers point holes and broad area selection but does not yet distinguish paint from bounds for area queries.

## Required Evidence

Extend the neutral contract before implementation with empty-hole crossing, disjoint triangle corners, concave containment, painted stroke/dash boundaries, resolved Boolean/trace regions, and rotated image quadrilaterals. Validate equivalent outcomes with an existing independent geometry or raster library. Retain grant-partition, cancellation, identity, and immutable-cache laws.

No area-query implementation or acceptance claim has been made in this inspection. The current Draw describe operation is finishing the history ownership repair; its source is being kept stable until that component can be tested in the editor.

## Existing Independent Oracle

The scene-query TypeScript suite already uses AJV, Sharp-rendered SVG alpha samples, and Three matrix/bounds calculations. Its 14 current neutral queries run under grants 1, 7, and 4096 and retain immutable caches. The native counterpart runs the same neutral input through actual vector resolution and painted cache preparation. An area-query extension can reuse these producers and independent SVG rendering without adding a runtime dependency or another script.

## Current Preview Baseline

On the unchanged preview owner at port 6064, clicking the nested Red Frame layer selected its tree item and subsequently mounted the real layer inspector, including Even-Odd fill rule, stroke settings, transforms, arrange operations, and Path Nodes. Evidence is retained in `🗑️generated/draw-nested-layer-baseline.json`. This is an existing-component baseline, not acceptance of the pending history rebuild or exact area selection.

## Crossing Rectangle Regression

The neutral corpus now includes crossing rectangles strictly inside a resolved Boolean difference hole and a native PNG-trace pixel-cell hole. Full TypeScript RED handle 6689 is terminal exit 1: 609 passed, two failed, one skipped, 612 tests. Both failures observed a selected result despite an empty expected selection; the independent SVG alpha assertion passed before the picker assertion failed. This is an actual production regression witness.

The TypeScript picker now refines broad-phase overlapping path bounds by borrowing the same painted cache: one minimum-corner point query followed by incremental fill/stroke boundary clipping. The existing `paint` phase exposes progress for each borrowed edge and preserves private partial hits and exact cache authority. Image/text fallback quadrilaterals use their actual transformed corners. Rectangle containment retains the convex bounds rule; lasso still needs its separate exact containment repair.

Focused handle 21603 passed all four assertions but failed strict compilation on a lost union narrowing inside the image-corner callback. That callback is now a typed quad helper, and a fresh focused gate is running. No strict-green or native parity claim is made yet. The existing Draw script gains `scene-picking` as a test selector, and the adjacent launch configuration exposes it; no new script or task target is introduced.

The staged history component precedes this area-query production change. Its forthcoming activation can validate completion ownership only. Area selection must receive its own native assertion run and rebuilt browser component before end-user acceptance.

## Current Recovery and Verification

An executor interruption removed the original process handles and browser session. Read-only process inspection confirmed no Cargo/rustc/describe/test owner and no port-6064 listener. Original area native 59253 and history describe 16559 have no terminal success evidence and are interrupted, not live waits or passes. The final lifecycle test had five passing assertions but no retained terminal strict-compilation result.

Current-source native validation resumed at 81747, and focused TypeScript resumed at 51122. TypeScript is terminal exit 0 with five tests, zero failures, 1293 assertions and strict compilation. The corpus has 20 queries across grants 1, 7, and 4096; it adds paint-covered queries, a rectangle crossing the inner hole boundary, a query enclosing the whole object, and reversed hole coordinates. Independent Sharp SVG comparison now covers 2608 area samples and 18 existing point samples. Actual cancellation/build/precision interruption occurs during pending painted-boundary traversal.

The Rust counterpart uses the same fixed rectangle metadata and short per-edge region borrows. Its new native interruption law consumes the same neutral hole input. Native handle 81747 is terminal exit 0: three targeted tests passed, 574 filtered skips, with actual boundary-interruption and neutral-query diagnostics. Full TypeScript handle 26252 is terminal exit 0: 613 passed, no failures or skips, 2,795,019 assertions, 43 files and strict compilation. Its existing native PDF oracle directory was reused; no fresh PDF emission is claimed for this stage.

Current Draw describe 73308 includes both the unchanged ownership repair and crossing-area source. It is terminal exit 0: descriptor emission succeeded after 13m59s total, including prerequisites. Emitted component hash is 0a69eb58600588f659a2dc6815569563daefbf59732f95e250c3580258e4f7d6; descriptor hash is 595efe9295444735f9105ce725e83baf7bbcf1179e89ddacfc8bfac1e26a8e06. Materialization 86347 passed after 1m07s. Activation 52491 passed after 9m39s, including its native component prerequisite. A direct post-activation byte hash confirms the staged component still matches the emitted descriptor. Fresh preview 96339 is starting; actual browser acceptance remains pending. No cancelled process, lock removal or weakened build budget was used.

## Browser Access Refusal

Preview 96339 now serves http://127.0.0.1:6064/ and reports one staged component matching its source and activation receipt. The existing browser tab remained on its earlier connection-error page. Browser security rejected reload because that page's protocol is disallowed, and explicitly refused alternative routes to revisit it. No workaround, new tab, indirect browser navigation or hidden state access was attempted. Manual opening of the allowed HTTP page was requested from the user. This is a browser access refusal, not evidence of a Draw runtime failure or successful Undo/Redo. Independent source and test work can continue; the full goal remains active.
