# Demonstrator pane labeling — Entwerfen mit Bestand + reuse (2026-09-17)

## Problem

With `terminology: "reuse"` locked on every demonstrator shell brand, the navbar title comes from
`resolveAppBreadcrumb(app, "reuse")` → `app.terminologyBreadcrumbs.reuse`, falling back to
`app.breadcrumb` when reuse crumbs are missing. Most bundled apps still had `breadcrumb: ["semio", …]`
and empty `terminologyBreadcrumbs`, so panes showed **semio · …** in the shell chrome.

Landing page, HTML `<title>`, and `windowTitle` on brands were already correct.

## Fix

1. **Rust app builders** — `.terminology("reuse").terminology_document("reuse", ["Entwerfen mit Bestand", "<Pane>"])` on every demonstrator editor surface:
   - Generator (`generation3d`), Koordinator (`cad`, was `cad` → **Koordinator**), Aussuchen, Bearbeiten, Verfolgen, Energie, Statik.
   - Aggregator (`puzzle3d`) already had correct crumbs.
2. **Committed plugin descriptors** (`🔣️.json`) — same `terminologyBreadcrumbs.reuse` + `terminologies: ["reuse"]` for those controller ids across demonstrator, procedural, sourcing, process, gis, energy, fem, cad, puzzle bundles.
3. **Vitest** — `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorpanebranding/🟦️.ts` asserts all eight pane controller ids resolve to `Entwerfen mit Bestand · <Pane>` and never contain `semio`.

## Follow-up (navbar still showed semio on Energie / Statik)

Those panes load **separate** `energy` / `fem` WASM shards; until those components are restaged, the
in-plugin manifest can still carry `breadcrumb: ["semio", …]`. The browser tab already used
`brand.windowTitle`; the **navbar** did not.

`ShellHost` now uses `brand.windowTitle` for every `isEntwerfenMitBestandBrandId` shell (same source as
`document.title`), so all eight panes show **Entwerfen mit Bestand · …** immediately without waiting on
plugin rebuilds.

## Verification

Run: `cd ♻️mit-bestand/🧺️demonstrator && bun ./📜️script.ts test quick`
