---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Restored workbench as a left toggle pane by reintroducing PanelKind.WORKBENCH plumbing and moving Design/Docs/Quality workbench content from windows into left panel sections.
## Changes

- Updated `semio/js/sketchpad/shared.ts` to restore `PanelKind.WORKBENCH`, include workbench in panel key/visibility/sections, and map it to left side panel config.
- Updated `semio/js/sketchpad/Sketchpad.tsx` to track `workbench` sections and map `PanelKind.WORKBENCH` to left side tab content.
- Updated `semio/js/sketchpad/Design.tsx`:
  - Removed workbench from `DesignAppWindowKind` window layout.
  - Removed workbench window component from `windowConfig`.
  - Added workbench panel sections (`semio.sketchpad.app.kit.pieces`, `semio.sketchpad.app.design.windows`).
  - Added `PanelKind.WORKBENCH` to app panels.
- Updated `semio/js/sketchpad/Docs.tsx`:
  - Removed workbench from `DocsAppWindowKind` window layout.
  - Removed workbench window component from `windowConfig`.
  - Added workbench panel sections (`semio.sketchpad.app.docs.docs`, `semio.sketchpad.app.docs.overview`).
  - Added `PanelKind.WORKBENCH` to app panels.
- Updated `semio/js/sketchpad/Quality.tsx`:
  - Removed workbench from `QualityAppWindowKind` window layout.
  - Removed workbench window component from `windowConfig`.
  - Added workbench panel sections (`semio.sketchpad.app.quality.workbench.nodes`, `semio.sketchpad.app.quality.workbench.qualities`).
  - Added `PanelKind.WORKBENCH` to app panels.

## Log

- Reopened ticket with prompt: `Restore workbench as a left toggle pane not as a window`.
- Validated existing code still had workbench as window entries in Design, Docs, Quality.
- Restored shared panel plumbing for a workbench panel kind and panel sections.
- Migrated workbench UI content from app window configs into left side panel sections.
- Updated app panel definitions to register workbench tab.
- Ran tests:
  - `npm --prefix semio/js run test:unit` ✅ passed (12/12).
  - `npm --prefix semio/js run test:e2e` ❌ failed before tests; Playwright `config.webServer` did not start.
  - `npm --prefix semio/js exec tsc --noEmit` ❌ blocked by unrelated pre-existing syntax errors in `semio-repo/vscode/codegen/*` and `semio/assets/logo/logo.ts`.

## Todos

- [x] Reopen existing ticket covering workbench left-pane restoration.
- [x] Restore panel-kind plumbing for workbench.
- [x] Remove workbench window usage from Design/Docs/Quality.
- [x] Register workbench content in left panel sections.
- [x] Run verification commands.

## Plan

1. Reopen the existing workbench restoration ticket and inspect current window/panel wiring.
2. Restore `PanelKind.WORKBENCH` path through shared definitions and Sketchpad tab mapping.
3. Move workbench UI content out of GoldenLayout windows into `addSection("workbench", ...)` for affected apps.
4. Validate with automated checks and capture blockers for unrelated failures.
