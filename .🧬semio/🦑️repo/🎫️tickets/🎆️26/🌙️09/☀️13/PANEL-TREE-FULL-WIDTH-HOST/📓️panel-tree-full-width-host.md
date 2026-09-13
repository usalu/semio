# Panel Tree Full-Width Host

## Symptom

Top-left chrome-hosted panels (Inspektion, Dokument, Katalog) showed a large empty left region with a lone document icon; the real tree lived in a narrow column on the right instead of using the full panel width.

## Root cause

`uiNodeToTreePanelConfig` in `🛠️ShellHelpers/🟦️.tsx` wrapped every plugin panel body (`InterpretedUiNode`) as the `control` of an empty-label `TreeDataItem`. Items with a `control` use property-layout rows (`layoutKind: "property"`), whose grid is `1fr` label column + fixed value column. The default row icon sat in the wide label column; the entire authored tree was confined to the value column (~`controlValueColumnUiSpacing`).

Same failure mode as ticket `26/08/01/FIX-INSPECTOR-TREES-TO-MATCH-DOCUMENT` pass 3, which had regressed to the empty-label wrapper.

## Fix

Host the interpreted body on `TreePanelConfig.emptyState` with `sections: []`, so `Panel` → `Tree` renders the body at full width. Inner documents whose snapshot root is `component.type === "tree"` still render their own `Tree` via `TreeView`.

## Tests

Engine contract: `panelTreePanelHost` helper + assertion that config uses `emptyState` not section/item control wrapper.
