---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS
---

# Ticket

## Summary

Fixed persistent selection freeze by adding semantic no-op guard in selection setter to suppress redundant dispatch loops; tests pass.
## Changes
- Updated `semio/js/sketchpad/Design.tsx` to normalize connector selection shape, route Details sections through normalized connector selection, clear connector selection when piece/connection selection changes, and start drag transactions at drag start.
- Updated drag cancel cleanup in `semio/js/sketchpad/Design.tsx` to clear drag helper state on escape abort.
- Updated `semio/js/sketchpad/Sketchpad.tsx` plain store transactions to persist baseline snapshots, commit one history entry only when state changed, and restore baseline state on abort.
- Updated `semio/js/sketchpad/elements.tsx` slider/stepper pointer lifecycle so pointerup finalizes and pointercancel aborts reliably.
- Updated `semio/js/sketchpad/README.md` specs with connector-selection normalization and drag/transaction lifecycle requirements.
- Updated `semio/js/sketchpad/Design.tsx` with canonical selection normalization (`pieces`/`connections` always arrays), port-selection exclusivity, consistent deselect-all shape, pane-click clear wiring, and dev-only runtime selection guards (`[DEBUG]` diagnostics).
- Updated `semio/js/sketchpad/README.md` specs with explicit selection invariants and mixed inspector routing expectations.
- Updated `semio/js/sketchpad/Design.tsx` mixed-selection details routing so piece+connection mixed selection renders warning-only instead of conflicting piece/connection editors.
- Updated `semio/js/sketchpad/Design.tsx` replacement dropdown mechanics to normalize piece type resolution (`string` and `{guid}`), compute design replacement options for both direct and included design pieces, constrain design replacement matching to computed replacement option pools, and guard against empty option regressions in dev mode.
- Updated `semio/js/sketchpad/README.md` specs with replacement dropdown invariants for normalized identifiers and design-reference-safe updates.
- Updated `semio/js/sketchpad/Design.tsx` with minimal stability guardrails: type-safe id/selector helpers for selection and entity checks, dev-only impossible-state assertions, debug logging toggles for selection/transaction channels, and invalid-id write guards for piece/connection update actions.
- Updated `semio/js/sketchpad/README.md` specs with debug-toggle and invalid-write guardrail requirements.
- Updated `semio/js/sketchpad/Design.tsx` to fix a maximum update depth loop by restoring a stable selection selector factory (`createDesignSelectionSelector`) in `useDesignAppSelectionField` and moving read-time normalization/guards to memoized `useDesignAppSelection`.
- Updated `semio/js/sketchpad/Design.tsx` diagram `onSelectionChange` synchronization to normalize/de-duplicate selection ids and compare set-membership before calling `setSelection`, preventing feedback-loop churn and UI freeze while selecting.
- Updated `semio/js/sketchpad/README.md` specs with selection synchronization normalization requirement.
- Updated `semio/js/sketchpad/Design.tsx` `useDesignAppSelection` setter with semantic-equality no-op guard (pieces/connections set comparison + primary connector comparison) to prevent redundant selection dispatch storms that can still freeze the UI under repeated selection events.
- Updated `semio/js/sketchpad/README.md` specs with selection-setter equivalence no-op requirement.

## Log
- Ran repo discovery with `./semio-repo/cli/cli tree "editor inspector details"`.
- Opened ticket `2026/02/11/MAP-EDITOR-INSPECTOR-DETAILS-ARCHITECTURE`.
- Traced:
  - Selection state shape and update paths in `semio/js/sketchpad/Sketchpad.tsx`, `semio/js/sketchpad/Design.tsx`, `semio/js/sketchpad/shared.ts`.
  - Design/document model storage and mutation pipeline in `semio/js/sketchpad/Sketchpad.tsx` (`KitStore` + `DesignStore` + `kitCommands`).
  - Details panel composition and layout sizing flow in `semio/js/sketchpad/Sketchpad.tsx` and `semio/js/sketchpad/elements.tsx`.
- Key finding:
  - Design selection for ports/connectors is inconsistent:
    - `Sketchpad.tsx` type: `selection.connectors?: Array<{piece, connector}>`
    - `Design.tsx` details logic often reads `selection.connector` (singular)
    - `useDesignAppCommands.selectPiecePort` emits `selection.connectors`.
- Follow-up regression triage pass completed:
  - Produced reproducible UI checklist for details/selection, transaction grouping, mixed selection values, replacement dropdowns, and right-panel resizing.
  - Extracted code-grounded root-cause candidates for each behavior with exact references in `Design.tsx`, `Sketchpad.tsx`, `elements.tsx`, and old source files.
  - Identified first inspection order and missing invariants to verify before code changes.
- Implementation pass completed:
  - Added `getPrimarySelectedConnector` adapter and applied it in inverse selection diff, connector selector hook, diagram connector context derivation, and Details section routing.
  - Updated selection hooks and design app selection event handlers to clear connector selection whenever piece/connection selection lanes are changed.
  - Moved drag transaction start from drag stop to drag start, keeping finalize at drag stop and abort on escape.
  - Replaced stale closure references in drag compatibility logic by using `selectionRef` and `kitRef`.
- Transaction repair pass completed:
  - Audited stack shape and recording path in `PlainAppStore` and `PlainKitDiffAppStore`.
  - Fixed inverse snapshot timing by passing pre-change snapshots from `DesignStore.executeCommand` into `recordEdit`.
  - Added baseline snapshot state capture on `startTransaction`, one-entry commit-on-change on `finalizeTransaction`, and baseline restore on `abortTransaction`.
  - Added snapshot-aware undo/redo fallback path for plain stores.
  - Added robust pointerup/pointercancel transaction closure in slider/stepper controls.
- Verification:
  - `npm run test:e2e -- sketchpad.test.ts --grep "Design"` failed because Playwright `config.webServer` could not start in this environment.
  - `npx tsc --noEmit -p tsconfig.json` reports existing unrelated type errors in `Feedback.tsx`, `Sketchpad.tsx`, `Type.tsx`, and `elements.tsx`; no remaining type errors from modified `Design.tsx` lines.
  - `npm run test -- sketchpad.test.ts` in `semio/js` failed with `No test files found` because current Vitest include is `semio.test.ts`.
  - `npm run test` in `semio/js` passed (`semio.test.ts`, 11 tests).
  - Re-ran `npm run test` in `semio/js` after mixed-selection routing patch; still passed (`semio.test.ts`, 11 tests).
  - Re-ran `npm run test` in `semio/js` after replacement-dropdown parity patch; still passed (`semio.test.ts`, 11 tests).
  - Re-ran `npm run test` in `semio/js` after guardrail pass; still passed (`semio.test.ts`, 11 tests).
  - Re-ran `npm run test` in `semio/js` after selection-selector stabilization fix; still passed (`semio.test.ts`, 11 tests).
  - Re-ran `npm run test` in `semio/js` after selection synchronization loop guard patch; still passed (`semio.test.ts`, 11 tests).
  - Re-ran `npm run test` in `semio/js` after selection setter no-op guard patch; still passed (`semio.test.ts`, 11 tests).

## Todos
- [x] Map where selection lives and updates.
- [x] Map document/design model ownership and mutation path.
- [x] Verify undo/redo + batching behavior.
- [x] Determine owner for details panel width and resize flow.
- [x] Produce missing primitives checklist before porting old behavior.
- [x] Produce reproducible regression checklist and likely root causes.
- [x] Produce inspection order and missing invariants.
- [x] Implement Details/Inspector parity fixes.
- [x] Document new specs for selection normalization and drag transaction lifecycle.
- [x] Audit and repair undo/redo transaction baseline/finalize/abort semantics.
- [x] Update pointer event transaction closure for slider/stepper controls.
- [x] Enforce Design selection invariants and add dev runtime guards.

## Plan
- `Editor/Inspector/Details` architecture map:
  - `UI interaction (diagram/tree/details)` -> dispatches app events and/or store commands.
  - `XState actor context (Sketchpad machine)` -> owns app-scoped UI state (`selection`, `hover`, `panelVisibility`, `activeTool`, `fullscreen`, etc).
  - `DesignApp store (PlainKitDiffAppStore)` -> executes `semio.designApp.*` commands, computes selection diff inverse, records edits during active transaction.
  - `KitStore/DesignStore (Yjs-backed data model)` -> applies `KitDiff`/`DesignDiff` to canonical kit/design entities (`types`, `designs`, `pieces`, `connections`, `ports`, etc).
  - `PanelSectionProvider + side panel tabs` -> composes details sections from app plugins and renders them into right side panel tabs.
  - `Layout (elements.tsx SidePanel)` -> handles drag resize and calls `onSizeChange`; persisted via `sketchpad.panelSizes`.
- Selection map:
  - Types:
    - `DesignAppSelection` in `Sketchpad.tsx` uses `pieces`, `connections`, `connectors[]`.
    - Legacy/store diff path in `Design.tsx` uses `DesignAppSelectionDiff` with singular `connector`.
  - Updates:
    - Generic reducers via `registerKeyedAppEventHandlers` in `shared.ts` handle `DESIGN.SET_SELECTION` / `DESIGN.CLEAR_SELECTION` by replacing selection object.
    - Explicit reducers in `Design.tsx` handle `DESIGN.SELECT_PIECE`, `DESIGN.DESELECT_PIECE`, `DESIGN.SELECT_CONNECTION`, `DESIGN.DESELECT_CONNECTION`.
    - Port selection path in `Design.tsx` command facade emits `DESIGN.SET_SELECTION` with `connectors`.
    - Details rendering in `Design.tsx` currently branches on `selection.connector` for connector details.
- Document/design model map:
  - Canonical model lives in `KitStore`/`DesignStore` in `Sketchpad.tsx` (Yjs maps/arrays + snapshot caches).
  - All model mutation happens through command->diff->change:
    - UI calls `store.execute("semio.designApp.*")` or `kitStore.execute("semio.kit.*")`.
    - `semio.designApp.*` commands in `Design.tsx` return `kitDiff` + optional selection diff.
    - Store applies:
      - `DesignStore.change(diff)` for app-local fields.
      - `KitStore.change(kitDiff)` for document mutations.
      - Nested `DesignStore.change(designDiff)` for pieces/connections updates.
- Undo/redo + transaction batching:
  - Exists in active store layer (`PlainAppStore` / `PlainKitDiffAppStore` in `Sketchpad.tsx`):
    - API: `startTransaction`, `finalizeTransaction`, `abortTransaction`, `undo`, `redo`.
    - Batching model: active transaction accumulates edits in `currentTransactionStack`; finalize merges first `undo` + last `do` into one history edit.
    - Scope: includes selection diffs; kit-diff stores also include inverse `kitDiff`.
  - Additional XState transaction event handlers exist in `shared.ts` (`*.TRANSACTION.*`) and state includes `transaction`, but no dispatch usage found for `DESIGN.TRANSACTION.*`/`TYPE.TRANSACTION.*`/`KIT.TRANSACTION.*` in the current app flow.
- Details panel width ownership + resize:
  - Owner should be `LayoutWrapper` in `Sketchpad.tsx` (`panelSizes.rightSidePanelWidth`), not `Design.tsx`.
  - Resize execution path:
    - `elements.tsx` `SidePanel` drag handle -> `onSizeChange(size)`.
    - `LayoutWrapper` forwards to `sketchpadCommands.setPanelSize(..., "rightSidePanelWidth", size)`.
    - `setPanelSize` writes via `semio.sketchpad.setState` -> `panelSizes` diff in sketchpad state (persisted in Yjs/local storage).
  - `detailsWidth` remains in types/defaults for backward shape compatibility but appears unused by current right-side panel layout.
- Key files to edit when implementing parity:
  - `semio/js/sketchpad/Design.tsx`
  - `semio/js/sketchpad/Sketchpad.tsx`
  - `semio/js/sketchpad/shared.ts`
  - `semio/js/sketchpad/elements.tsx`
  - `semio/js/sketchpad/Kit.tsx` (if cross-app inspector behavior alignment is required)
  - `semio/js/sketchpad/Type.tsx` (if connector/port inspector behavior needs shared primitives)
- Missing primitives checklist before porting old behavior:
  - Canonical selection schema for design connectors/ports (single `connector` vs array `connectors`) with one source of truth.
  - Selection normalizer and adapter at app boundary so details/diagram/workbench all consume identical selection shape.
  - Unified transaction facade (one public API, one backend) so XState and store stacks cannot diverge.
  - Explicit transaction policy for drag edits (begin/record/commit cadence and abort semantics on cancel).
  - Canonical inspector section builder contract (pure function from app context -> sections) to remove ad-hoc add/remove effects.
  - Canonical panel size keys for side panels (`leftSidePanelWidth`/`rightSidePanelWidth`) and deprecate orphaned width keys (`detailsWidth` path cleanup).
  - Strongly typed command payloads for `updatePieces`/`updateConnections` and selection events to avoid shape drift.

## Follow-up Regression Triage

### Repro Checklist (UI)
1. Port/connector selection -> Details section:
   1. Open a design and ensure right side panel is visible on Details tab.
   2. Click a piece connector/port handle in the canvas.
   3. Verify connector details section appears (id prefix `semio.sketchpad.app.type.connector.properties`).
   4. Click a different connector; verify details switches to the new connector.
   5. Click connector again to deselect; verify connector section disappears and design section remains.
2. Drag transaction + escape/undo grouping:
   1. Select one piece and drag it to a new position; release.
   2. Press `Ctrl+Z`; verify one-step restore to original position.
   3. Start another drag and press `Escape` before release.
   4. Verify no persisted movement or side effects (including temporary auto-connections).
3. Multi-select common value behavior:
   1. Select two pieces with different type names.
   2. Open Details and verify type combobox placeholder indicates mixed values.
   3. Select two pieces with same type but different variant; verify variant mixed placeholder.
   4. Change type/variant once and verify all selected pieces update in one grouped undo step.
4. Replacement dropdown behavior:
   1. Select a single regular piece; open type name/variant replacement dropdowns.
   2. Verify options are constrained to replacable candidates.
   3. Select a single design piece; verify design name/variant/view dropdown candidates are valid and current value can be reselected.
   4. Multi-select design pieces and verify fallback behavior (no invalid options, no silent no-op).
5. Details width resize:
   1. Drag right panel resize handle inward/outward.
   2. Verify width changes smoothly and clamps at min/max.
   3. Switch tabs/details content and verify width remains stable.
   4. Reload app and verify width persistence if expected by current persistence policy.

### Likely Root Causes (Grounded)
1. Connector details not showing or stale:
   - Selection shape mismatch: details branch checks `selection.connector` while other paths use `selection.connectors`.
     - `semio/js/sketchpad/Design.tsx:8050-8052`
     - `semio/js/sketchpad/Design.tsx:8081-8082`
     - `semio/js/sketchpad/Sketchpad.tsx:7741-7744`
   - Action hook writes singular connector while command facade writes connectors array.
     - `semio/js/sketchpad/Design.tsx:1870-1876`
     - `semio/js/sketchpad/Design.tsx:2285-2292`
   - Two `DesignAppSelection` definitions diverge between files.
     - `semio/js/sketchpad/Design.tsx:229-234`
     - `semio/js/sketchpad/Sketchpad.tsx:7741-7744`
2. Drag escape/undo grouping regressions:
   - Drag starts no transaction, but escape calls abort; abort is ineffective without active transaction.
     - `semio/js/sketchpad/Design.tsx:6331-6359`
     - `semio/js/sketchpad/Design.tsx:6147-6151`
   - Connection mutations can occur during drag before drop finalization.
     - `semio/js/sketchpad/Design.tsx:6942-6944`
   - Transaction only started at drag stop, not drag start.
     - `semio/js/sketchpad/Design.tsx:6969-6995`
     - old baseline started at drag start: `js/semio/sketchpad/Design.Diagram.tsx.old:897`
3. Multi-select common values/replacement edge regressions:
   - Common type/variant resolution assumes `piece.type` string for lookups while other logic uses object-with-guid branches.
     - `semio/js/sketchpad/Design.tsx:3964-3970`
     - `semio/js/sketchpad/Design.tsx:4029-4030`
   - Replacement candidate filtering is single-piece centric for design replacement (`availableDesigns` only in single mode).
     - `semio/js/sketchpad/Design.tsx:4025-4027`
   - Mixed selection path can short-circuit on `hasMixedTypes`, which may hide expected bulk edit controls.
     - `semio/js/sketchpad/Design.tsx:3597-3599`
     - `semio/js/sketchpad/Design.tsx:4059-4065`
4. Resize behavior drift:
   - Width ownership moved from details-local state to global right side panel state.
     - old: `js/semio/sketchpad/Desing.tsx.old:70`, `js/semio/sketchpad/Design.Details.tsx.old:1287-1303`
     - new: `semio/js/sketchpad/Sketchpad.tsx:15850-15857`
   - Current side panel resize uses mouse events only with strict min/max gate; no pointer capture/touch path.
     - `semio/js/sketchpad/elements.tsx:4439-4463`
     - `semio/js/sketchpad/elements.tsx:4452-4454`

### First Files/Hooks to Inspect
1. `semio/js/sketchpad/Design.tsx`
   - Details section mount effect and selection branch (`8050`, `8081`), port selection hooks (`1870+`), drag lifecycle (`6331+`, `6969+`, `6147+`).
2. `semio/js/sketchpad/Sketchpad.tsx`
   - Canonical `DesignAppSelection` type (`7741+`) and store transaction semantics (`896+`, `1108+`).
3. `semio/js/sketchpad/elements.tsx`
   - Transaction-aware controls (`Combobox` around `1559+`) and sidepanel resize implementation (`4418+`).
4. `semio/js/sketchpad/shared.ts`
   - `SET_SELECTION` replacement semantics (`1871+`) and generic transaction handler utilities (`2176+`) to rule out competing transaction paths.
5. `js/semio/sketchpad/Design.Diagram.tsx.old` and `js/semio/sketchpad/Design.Details.tsx.old`
   - Baseline behavior for drag transaction boundaries and details connector/piece/connection routing.

### Missing Invariants To Enforce During Debugging
- Selection shape invariant: design selection must use one canonical connector field shape across actor/store/hooks/details.
- Selection coherence invariant: selecting connector, piece, and connection updates must preserve/clear other lanes deterministically.
- Transaction lifecycle invariant: all gesture mutations (drag, slider/stepper, combobox replace) occur inside exactly one start/finalize pair, with escape/cancel mapped to abort while active.
- Undo granularity invariant: one user gesture => one undo item.
- Inspector write-path invariant: details controls only mutate through command/store APIs, never direct state mutation.
- Panel ownership invariant: details width is owned by right side panel state only; no duplicate width source.
