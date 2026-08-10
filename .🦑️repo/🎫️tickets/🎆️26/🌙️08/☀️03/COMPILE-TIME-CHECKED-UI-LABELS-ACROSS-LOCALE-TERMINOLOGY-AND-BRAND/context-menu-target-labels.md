# Context Menu Target Labels

## Requirement

Every context menu title must identify the element under the pointer. For example, a right-click on a vortex must open a menu titled `Vortex Menu`, while a right-click on the 3D background must use `Scene Menu`.

## Finding

The React renderer currently stores only coordinates and resolved rows after a surface menu request. Each host supplies one fixed surface title (`Scene actions`, `Board actions`, and similar), even though the request already carries the freshly hit-tested target domain in `surface.hits[0].domain`.

The shared `openSurfaceContextMenu` funnel is the correct single source of truth. It can resolve a typed translation key from the most-specific hit domain, then fall back to the surface kind when the click is on the background. Returning that key with the rows prevents individual hosts from drifting and preserves locale-aware labels.

## Covered Target Domains

`architecture`, `attraction`, `block`, `edge`, `entry`, `feature`, `group`, `handle`, `layer`, `node`, `object`, `part`, `path`, `pixel`, `position`, `reference`, `route`, `row`, `slider`, and `vortex`.

## Covered Background Surfaces

Canvas, scene, node graph, editor, table, paint, map, board, ink, history, block list, diff, event feed, file system, and workspace fallback menus.

## Implementation

- Added a typed, target-first title resolver to the shared surface context-menu funnel.
- Returned the resolved translation key together with the menu rows so every React surface uses the same title decision.
- Added English and German menu-title labels for every emitted target domain and normalized background titles to the `X Menu` form.
- Migrated every React surface host to retain and render the resolved title key.
- Completed the 3D hit propagation by treating a plain `selection.hoveredId` as an object hit after the vortex, component, and reference checks. Without this branch, right-clicking an object selected the object but incorrectly titled its menu `Scene Menu`.

## Verification

- `bun nx run @semio-tech/framework-renderer-react:test-quick -- --run '🧪️index.test.ts' -t 'resolves the right-click context menu target|titles context menus|covers every target domain'`: 3 passed, 303 skipped.
- `bun nx run @semio-tech/framework-renderer-react:lint`: passed the region and host-contract lint.
- `bun nx run-many -t typecheck check-chrome-i18n -p @semio-tech/ui-react --parallel=2`: Chrome i18n validation passed with zero violations; typecheck was blocked by the unrelated existing generated-manifest parse error at `🛢️manifest/🤖️generated/🟦️manifest.ts:154`.
- `bun nx run @semio-tech/framework-renderer-react:test-long`: reached the repository's 300-second execution budget and was killed without reporting an assertion failure.
- Live Puzzle 3D verification: right-clicking model geometry renders `Object Menu`; right-clicking empty viewport space renders `Scene Menu`. The title is therefore driven by the element under the pointer, rather than the surface's fixed label.
- `git diff --check`: passed.
