# P10n Renderer React Test Closure

## Outcome

- Closed all 14 failures in the full `@semio-tech/framework-renderer-react:test-quick` suite: 4/4 files and 436/436 tests now pass.
- Restored the production `uiTreeNodeToTreePanelConfig` export in the renderer Interpreter as an owned tree-panel boundary. The mapping retains sections, recursive items, selection/highlight state, activity state, row actions, manifest action dispatch, deterministic drag payload routing, and drop metadata without exporting external implementation types.
- Restored the associated declarative tree drag controller used by `ShellHelpers` while keeping manifest actions behind the renderer-owned `ActionDescriptor` contract.
- Routed all four renderer test harness imports through `@semio-tech/ui-react/test`; JavaScript parity again reports zero undeclared imports.
- Updated stale tests only where the live source/schema established the intended contract: complete external-slot nodes, generated virtual-file-system names, the stored numeric argument schema, window-owned action enumeration, document labels, plural demonstrator logo assets, and current command descriptor dispatch.
- Replaced the unavailable jest-dom-only assertion with an equivalent built-in Vitest assertion.

## Initial Failure Census

The first exact run of `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` reported 4 files, 436 tests, 14 failed, and 422 passed. The failures were:

- one incomplete external-slot input;
- one stale virtual-file-system host/schema name;
- one missing `uiTreeNodeToTreePanelConfig` export;
- one unavailable jest-dom matcher;
- three window-action staging tests using the removed `control` field instead of the stored `schema` contract;
- one window action resolver expectation that discarded valid global/window-owned actions;
- one stale document category label;
- four singular demonstrator logo-directory expectations;
- one obsolete three-argument command callback expectation.

## Exact Validation

- Focused failure-cohort run with `--testNamePattern='framework external slots|framework renderer hosts|window action panel|registry-derived utilities|resolveCommands|shell option locks|buildCommandCategoryTree'` — 122/122 passed, 314 skipped.
- `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` — 4/4 files and 436/436 tests passed after the final minimal-diff cleanup.
- `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache` — passed (`region/host-contract lint passed`).
- `bun ./📜️script.ts verify dependencies parity js` — passed with 83 manifests, 296 external rows, 143 evidenced, 153 unowned, and 0 undeclared imports.
- `git diff --check` for the two edited renderer TypeScript files — passed.

## Typecheck Classification

`bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` was attempted and remains red on the repository's broader concurrent schema migration. Representative diagnostics are in demonstrator brand data, identity schema usage, the infinite-world Three boundary, renderer barrel exports, removed framework `UiNode` shapes, and existing `ShellHelpers` call sites. A filtered rerun found no diagnostic in the changed `Interpreter/🟦️component.tsx`; therefore the packet does not misrepresent the package-wide typecheck as green or broaden into unrelated concurrent ownership.

## Scope

- Changed renderer TypeScript source and tests only.
- Preserved the existing `@semio-tech/ui-react/test` subpath and renderer test rendering boundary.
- Added no dependency rows, allowlists, suppressions, compatibility layers, or Cargo work.
- Did not touch renderer Rust, Puzzle sources, manifests, lockfiles, or caches.
