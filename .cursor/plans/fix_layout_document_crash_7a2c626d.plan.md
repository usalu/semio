---
name: Fix Layout Document Crash
overview: Fix the field-naming bug that crashes the Layout playground's document panel, add regression coverage, and verify the Layout technology is fully wired end-to-end (playground + OS/S integration).
todos:
  - id: fix-document-bug
    content: Fix snake_case/camelCase field bugs in buildLayoutPlayDocumentTree and buildLayoutPlayInspectorTree (layout/core/js/index.ts)
    status: completed
  - id: add-regression-test
    content: Extend existing vitest block in layout/core/js/index.ts to cover buildLayoutPlayDocumentTree
    status: completed
  - id: run-tests
    content: Run layout-core vitest suite and confirm pass
    status: completed
  - id: verify-playground
    content: Launch layout playground dev server and manually verify document/inspector/preflight panels, undo/redo, and exports work without console errors
    status: completed
  - id: verify-os-integration
    content: Boot S studio dev host, host a layout app instance, confirm blueprint canvas renders and media export/VCS wiring works
    status: completed
isProject: false
---

# Fix Layout Playground Crash and Verify End-to-End Integration

## Root Cause

`buildLayoutPlayDocumentTree` in [layout/core/js/index.ts](layout/core/js/index.ts) reads snake_case fields (`page_ids`, `parent_page_id`, `object_ids`) that only exist on the **Rust** side (`layout/rs/document.rs`, which serializes them as camelCase via `#[serde(rename = ...)]`). The TypeScript `Spread`/`Page`/`Layer` types in [layout/core/js/internal.ts](layout/core/js/internal.ts) correctly declare `pageIds`, `parentPageId`, `objectIds`. Accessing the nonexistent snake_case fields returns `undefined`, so `.join(", ")` throws — this is exactly the reported `Cannot read properties of undefined (reading 'join')` at `index.ts:113`, which crashes `<PlaygroundView>` because the document panel builds eagerly on mount.

There is no root-level `typecheck` script and no dedicated test for this function, so the type mismatch was never caught statically or by tests.

## Fix

In `buildLayoutPlayDocumentTree` ([layout/core/js/index.ts](layout/core/js/index.ts) lines ~113-140), correct 4 occurrences:

- `spread.page_ids.join(", ")` → `spread.pageIds.join(", ")`
- `page.parent_page_id ? ... ${page.parent_page_id}` → `page.parentPageId ? ... ${page.parentPageId}` (document tree, line ~122)
- `layer.object_ids.length` → `layer.objectIds.length`
- `page.parent_page_id ?? "(none)"` → `page.parentPageId ?? "(none)"` (inspector tree, line ~220)

## Regression Coverage

Extend the existing `if (import.meta.vitest)` block at the bottom of [layout/core/js/index.ts](layout/core/js/index.ts) (per repo convention, no new test files) with a test that calls `buildLayoutPlayDocumentTree(DEFAULT_LAYOUT_DOCUMENT_JSON, [])` and asserts it doesn't throw and produces the expected spreads/pages/layers sections — this is exactly the path that was broken and untested.

## Verification (end-to-end)

1. Run `bun nx run @semio-tech/layout-core:test` (vitest) to confirm the fix and new test pass.
2. Launch the standalone Layout playground (`bun run dev:layout`, port 6079, matches the `🛠️dev📄layout` launch config) and load it in the browser to confirm:
   - No console errors / no `<PlaygroundView>` crash.
   - Document panel lists Document/Spreads/Pages/Parent Pages/Layers/Stories/Links/Styles correctly.
   - Inspector panel shows page/frame fields when selecting a document item.
   - Preflight panel lists the expected seeded issues (missing asset, small font).
   - Undo/redo and PNG/SVG/PDF/Package export toolbar buttons work.
3. Spot-check OS ("S") integration: confirm `registerLayoutMediaExportHandlers` (svg/png export via `2d.layout` resource kind) and `createLayoutAppVcsHandler` + `buildLayoutProgramDefinition` (already wired in [s/core/js/program-extensions.ts](s/core/js/program-extensions.ts) and [s/core/js/internal.ts](s/core/js/internal.ts)) work by booting the S studio dev host and hosting a `layout` app instance (renders via the `case "layout"` branch in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)), confirming the blueprint canvas + JSON editor render without errors.

No other gaps were found: the Layout app definition, window bodies, panel tabs, VCS handler, program definition, and media export handlers are already fully registered end-to-end (playground dev registry, S program extensions, S media export registry, launch.json dev target) — this was purely a runtime crash from the field-naming bug, not a missing-integration issue.

## Process Note

Per repo convention this work should happen inside a `.repo/🎫/...` ticket associated with the relevant goal, closed with a summary of changed files. The repo MCP server is currently reporting "not ready"; I will retry opening/closing the ticket at execution start, and will flag it if it's still unavailable.
