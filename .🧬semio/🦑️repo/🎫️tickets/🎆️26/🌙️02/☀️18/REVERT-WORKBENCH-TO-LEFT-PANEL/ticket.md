---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Migrated workbench to left toggle panel with panel-kind plumbing, app layout migration cleanup, and updated Design e2e selectors.

## Changes

- Updated `compose/js/sketchpad/shared.ts`:
  - Added `PanelKind.WORKBENCH`.
  - Added `workbench` to `PanelKey` and `PanelSections`.
  - Added `panelKindConfigs[PanelKind.WORKBENCH]` with left panel configuration and `WorkbenchIcon`.
- Updated `compose/js/sketchpad/Sketchpad.tsx`:
  - Added `workbench` entry to `PanelSectionProvider` initial state.
  - Registered `workbench` sections in `sectionsByKind` for side panel tab content.
- Updated `compose/js/sketchpad/Design.tsx`:
  - Removed workbench from `DesignAppWindowKind`.
  - Removed workbench column from default GoldenLayout and rebalanced widths.
  - Removed workbench window component from `windowConfig`.
  - Added workbench panel sections (`compose.sketchpad.app.kit.pieces`, `compose.sketchpad.app.design.windows`).
  - Added `PanelKind.WORKBENCH` to app panel config.
  - Added stored-layout migration cleanup for legacy `"workbench"` window nodes.
- Updated `compose/js/sketchpad/Docs.tsx`:
  - Removed workbench from `DocsAppWindowKind`.
  - Removed workbench column from default GoldenLayout.
  - Removed workbench window component from `windowConfig`.
  - Added workbench panel sections (`compose.sketchpad.app.docs.docs`, `compose.sketchpad.app.docs.overview`).
  - Added `PanelKind.WORKBENCH` to app panel config.
  - Added stored-layout migration cleanup for legacy `"workbench"` window nodes.
- Updated `compose/js/sketchpad/Quality.tsx`:
  - Removed workbench from `QualityAppWindowKind`.
  - Removed workbench column from default GoldenLayout and rebalanced widths.
  - Removed workbench window component from `windowConfig`.
  - Added workbench panel sections (`compose.sketchpad.app.quality.workbench.nodes`, `compose.sketchpad.app.quality.workbench.qualities`).
  - Added `PanelKind.WORKBENCH` to app panel config.
  - Added runtime layout migration cleanup for legacy `"workbench"` window nodes.
- Updated `compose/js/sketchpad.test.ts`:
  - Migrated Design workbench assertions from `#workbench` window selectors to left side panel selectors.
  - Added explicit workbench tab activation via `compose.sketchpad.navbar.panelToggle.workbench.show` before DnD and action assertions.

## Log

- Reopened existing ticket for workbench-left-panel migration continuation.
- Verified current regression: workbench content still mixed between window layout and incomplete left-panel registration.
- Refactored shared panel model + app configs + panel sections to make left-panel workbench the primary path.
- Added legacy window-layout migration cleanup to prevent stale persisted workbench windows from breaking current layout.
- Migrated existing Design e2e coverage to left panel workbench selectors and left-tab activation flow.
- Ran tests:
  - `npm --prefix compose/js run test:unit` ✅️ passed (13/13).
  - `/bin/bash -lc "cd /workspaces/semio/compose/js && npx playwright test sketchpad.test.ts --grep \"Design\" --timeout 240000 --workers=1 --max-failures=1 --reporter=list"` ❌️ `config.webServer` failed to start.
  - `/bin/bash -lc "cd /workspaces/semio/compose/js && PLAYWRIGHT_SKIP_WEBSERVER=1 timeout 360s npx playwright test sketchpad.test.ts --grep \"Design\" --timeout 240000 --workers=1 --max-failures=1 --reporter=line"` ❌️ timed out (`exit 124`) after reaching known ReactFlow node attachment stall (`[Design Test] ReactFlow nodes not attached after 60s`).
  - `npm --prefix compose/js run test:e2e` ❌️ failed before tests; Playwright `config.webServer` did not start.
  - `npm --prefix compose/js exec tsc --noEmit` ❌️ blocked by pre-existing syntax errors in `repo/vscode/codegen/*` and `assets/logo/logo.ts`.

## Todos

- [x] Reopen existing ticket covering workbench left-pane restoration.
- [x] Restore panel-kind plumbing for workbench.
- [x] Remove workbench window usage from Design/Docs/Quality.
- [x] Register workbench content in left panel sections.
- [x] Update existing Design e2e test to left workbench panel selectors.
- [x] Run verification commands (unit + targeted e2e + project typecheck with documented blockers).

## Plan

1. Reopen the existing workbench restoration ticket and inspect current window/panel wiring.
2. Restore `PanelKind.WORKBENCH` path through shared definitions and Sketchpad tab mapping.
3. Move workbench UI content out of GoldenLayout windows into `addSection("workbench", ...)` for affected apps.
4. Validate with automated checks and capture blockers for unrelated failures.
