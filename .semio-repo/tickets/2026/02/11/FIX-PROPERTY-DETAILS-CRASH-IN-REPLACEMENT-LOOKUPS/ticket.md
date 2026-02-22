---
goal: R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS
---

# Ticket

## Summary

Property/details panel parity hardened: canonical selection guid resolution, known-connection validated details routing, and Design e2e assertions for no-selection + alternate payloads; tsc and unit tests pass, e2e blocked by webServer start failure in this environment.
## Panel Parity Completion 2026-02-22

### Plan
- Compare old and current selection-to-details flows (`Design.Details.tsx.old`, `Design.Diagram.tsx.old`, `Design.Model.tsx.old`, `Desing.tsx.old`, `Design.tsx`, `Sketchpad.tsx`) and isolate non-canonical routing paths.
- Refactor details routing to use validated known ids only for piece/connection sections.
- Extend existing `semio/js/sketchpad.test.ts` `Design` flow to assert no-selection design details and fallback absence for valid piece selections.
- Run `tsc`, unit tests, and available e2e flow.

### Todo
- [x] Enforce canonical guid extraction for selection strings to avoid unresolved-string false positives.
- [x] Route connection details only from known selected connections.
- [x] Add explicit no-selection assertions for design details panel content.
- [x] Keep existing panel topology, locations, and tab structure unchanged.
- [x] Run required verification commands and capture outcomes.

### Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Introduced shared `GUID_PATTERN` and tightened `resolveSelectionEntryGuid` to return only GUID-like values (or extracted GUIDs from node/wrapped shapes), not arbitrary strings.
  - In details-section routing effect, added known-connection validation:
    - build `knownConnectionGuids` from current `design.connections`.
    - resolve selected connections only when GUIDs match known connection ids or structured connection ids match existing connections.
    - derive `hasConnections` from resolved connection count, preventing empty/invalid connection-section routing.
  - Reused resolved connection list for section rendering to keep section selection deterministic under partial/invalid payloads.
- Updated `semio/js/sketchpad.test.ts` (existing file only):
  - Added no-selection assertions in `Design` test:
    - design name field is visible in right details panel.
    - `No valid pieces found in selection.` is absent.
  - During piece selection, assert fallback text remains absent while piece section/id render.
  - Kept existing alternate payload-shape checks (`nodeId`, nested object, wrapped string) and fallback absence checks.

### Verification
- `npx tsc --noEmit -p semio/js/tsconfig.json` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).
- `npm run test:e2e` in `semio/js` did not complete in this environment (hung without emitted output).
- Existing approved e2e command was executed:
  - `/bin/bash -lc "cd semio/js && npx playwright test sketchpad.test.ts --grep \"Design\" --timeout 240000 --workers=1 --max-failures=1 --reporter=list > /tmp/semio-design-playwright.log 2>&1; echo EXIT:$?; tail -n 200 /tmp/semio-design-playwright.log"`
  - Command output included: `Error: Process from config.webServer was not able to start. Exit code: 1`.
## Wrapped Payload Fix 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Added `resolveSelectionEntryGuidByKnownIds` to resolve ids by matching selection payload content against known selectable piece ids.
  - Applied known-id resolution in `PiecesSectionForm` selected piece extraction.
  - Applied known-id resolution in details section routing piece detection.
- Updated `semio/js/sketchpad.test.ts` existing `Design` test:
  - Added third selection payload regression variant: wrapped string format (`selected-piece:<guid>:active`).
  - Asserted piece details still render and fallback message is not shown.

## Resolver Correction 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Fixed `resolveSelectionEntryGuidByKnownIds` to return only known ids (no longer returns arbitrary unresolved strings).
  - Added raw payload fallback extraction in `PiecesSectionForm`:
    - when parsed valid selection ids are empty, derive selected ids by matching known piece ids against serialized selection payload.
  - Wired piece resolution to use fallback-selected ids for final piece lookup/mapping.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).

### Todo
- [x] Remove non-known id leakage from known-id resolver.
- [x] Add fallback known-id extraction from raw selection payload.
- [x] Re-run typecheck and tests.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).

### Todo
- [x] Resolve selection ids embedded in wrapped payload strings.
- [x] Extend existing Design test coverage for wrapped payload variant.
- [x] Keep panel structure/location unchanged.
- [x] Re-run typecheck and tests.

## Regression Completion 2026-02-22

### Changes
- Updated `semio/js/sketchpad.test.ts` in existing `Design` test:
  - Added assertions for alternate selection payload shapes:
    - node-id format: `piece-<index>-<guid>`
    - nested object format: `{ data: { piece: { guid } } }`
  - For each shape, details panel must show piece properties and must not show `No valid pieces found in selection.`
- Updated `semio/js/sketchpad/Design.tsx`:
  - Extended `resolveSelectionEntryGuid` with GUID-pattern extraction from arbitrary strings.
  - Added direct design snapshot fallback in `PiecesSectionForm` so unknown hook-resolved entries can still resolve to real selected pieces.
- Updated `semio/js/sketchpad/Sketchpad.tsx`:
  - Extended `usePiecesFromIds` candidate extraction with GUID-pattern matching in raw ids.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).
- Playwright execution attempted for `Design` test with grep; this environment did not provide reliable e2e completion output due webServer bind/startup constraints.

### Todo
- [x] Add regression checks in existing e2e framework file.
- [x] Harden selection-id normalization and fallback resolution paths.
- [x] Keep panel structure and location unchanged.
- [x] Re-run typecheck and available tests.

## Final Hardening 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Added deep nested GUID extraction fallback in `resolveSelectionEntryGuid`.
  - Filtered details piece resolution to valid selected ids present in current design pieces/included-design entries.
  - Updated details routing selection checks to treat only known piece ids as piece selections.
- Updated `semio/js/sketchpad/Sketchpad.tsx`:
  - Extended `usePiecesFromIds` id candidate extraction with recursive nested-object scanning and GUID pattern detection.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
## Selection Resolution Fix 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Design.tsx`:
  - Added `resolveSelectionEntryGuid` to normalize selection entries from multiple shapes:
    - plain guid strings
    - reactflow-like node ids (`piece-<index>-<guid>`, `connection-<index>-<guid>`)
    - objects with `guid`, `id_`, `id`
    - nested `{ piece: { guid } }` and `{ data: { piece: { guid } } }`
  - Switched piece and connection extraction in details section routing and piece form selection resolution to use the normalized resolver.
  - Switched diagram selection-sync and model selection-set derivation to use normalized ids for stable cross-view behavior.
- Updated `semio/js/sketchpad/Sketchpad.tsx`:
  - Extended `usePiecesFromIds` with robust id candidate resolution, including node-id parsing and nested selection-object shapes.
  - Kept fallback behavior for unknown ids.
- Kept panel topology and panel position unchanged.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).

### Todo
- [x] Reopen ticket for persisted selected-piece details regression.
- [x] Normalize selection entry shape handling across details and selection-sync paths.
- [x] Keep panel structure/location unchanged.
- [x] Re-run typecheck and tests.
## Crash Fix 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Sketchpad.tsx` in `useReplacableTypes`:
  - Build a minimal `kit` with both `types` and current `design` (`designs: [design]`) so `findReplacableTypesFor*` does not throw on missing design.
  - Normalize incoming selected ids (`string`, `{guid}`, `{id_}`) and filter to ids that exist in current `pieces`.
  - Return `[]` when no valid piece ids resolve.
  - Wrap replacement computation in `try/catch` and return safe fallback `[]`, with `[DEBUG]` warning log payload.
- Kept panel topology and location unchanged.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).

### Todo
- [x] Isolate crash path from runtime stack trace.
- [x] Prevent replacement lookup from throwing when selection ids are not design-piece ids.
- [x] Keep details panel structure and placement unchanged.
- [x] Re-run typecheck and tests.
## Scope Hotfix 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Design.tsx` details section registration to wrap selected-item sections in `DesignScopeProvider`:
  - connector properties section
  - piece properties section
  - connection properties section
- Kept panel topology and panel positions unchanged.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).

### Todo
- [x] Reopen scope rendering regression ticket.
- [x] Fix selected-item details section scope wiring.
- [x] Keep panel structure and location unchanged.
- [x] Re-run typecheck and tests.
## Hotfix 2026-02-22

### Changes
- Updated `semio/js/sketchpad/Sketchpad.tsx` `usePiecesFromIds` to resolve piece ids from `string`, `{guid}`, and `{id_}` entries.
- Updated `semio/js/sketchpad/Design.tsx` `PiecesSectionForm` to normalize `selection.pieces` into canonical guid ids before resolving piece records.
- Updated `semio/js/sketchpad/Design.tsx` selected connector resolution to accept both `selection.connector` and `selection.connectors[0]`.
- Updated `semio/js/sketchpad/Design.tsx` details-section routing effect to normalize piece/connection ids and resolve connections from both guid and object-id selection entries.
- Kept panel topology unchanged: no new panels and no panel location changes.

### Verification
- `npx tsc --noEmit -p tsconfig.json` in `semio/js` passed.
- `npm run test` in `semio/js` passed (`semio.test.ts`, 13 tests).

### Todo
- [x] Reopen regression ticket for selected-element details rendering.
- [x] Normalize selection payload shapes for property resolution paths.
- [x] Keep panel structure and panel placement unchanged.
- [x] Run TypeScript check.
- [x] Run test suite.
## Analysis 2026-02-22

### Analysis Refresh 2026-02-22 (Requested Q&A)

#### Purpose & Responsibilities
- What user problems does the Property Panel solve?
  - It provides one contextual place to inspect and edit whichever design entities are selected, reducing context switching between diagram/model and metadata editing.
  - It supports both single-item precision edits and multi-item bulk edits with mixed-value handling.
- What responsibilities does it own vs delegate?
  - Owns: section routing based on selection lane/cardinality, field presentation, mixed-value UX, partial-data fallback rendering.
  - Delegates: canonical data storage, command execution, transaction persistence, replacement-candidate lookup, and undo/redo stack semantics.
- Which responsibilities are essential vs accidental?
  - Essential: contextual routing, edit orchestration with transaction boundaries, resilient fallback behavior.
  - Accidental: local width/visibility ownership, monolithic component orchestration, direct dependence on legacy selection object shape.

#### Data Flow & State Model
- Where does panel data originate?
  - Old: pull from editor hooks (`useDesign`, `useDesignEditorSelection`, metadata + replacement hooks).
  - Current: actor snapshot selectors keyed by app scope plus panel section registries.
- How are changes propagated?
  - Old: push from UI handlers to command hooks, mostly synchronous assumptions around focus/pointer/drag lifecycle.
  - Current: push via actor events and command facades; pull via selectors; panel composition updates through section registration.
- Old assumptions about ownership/lifecycle/mutability:
  - Selection is plain mutable state with immediate read-after-write behavior.
  - One component tree coordinates panel state and content.
  - Layout concerns (width/visibility) can be owned locally by the panel implementation.

#### Interaction Mechanisms
- How does the panel react to selection changes?
  - None selected: design section.
  - Port/connector selected: connector/port-focused section.
  - Pieces selected: piece editor (single or multi).
  - Connections selected: connection editor (single or multi).
  - Mixed piece+connection: warning/constraint section.
- How does it react to external state updates?
  - Old: rerender from hook snapshots.
  - Current: selector-driven updates + add/remove section registration.
  - Model/Diagram are selection producers; Property Panel is a selection consumer. `Design.Model.tsx.old` confirms model selection writes into the same selection channel as diagram selection (`selectPiece`, `selectPieces`, `deselectAll`), so panel intent is cross-view consistency, not diagram-only behavior.
- How does it handle invalid or partial data?
  - Degrades gracefully with explicit fallback sections/messages (`No valid pieces found`, `Port not found`) instead of hard failure.
- Which behaviors are rule-based, derived, conditional?
  - Rule-based: right-side exclusivity (`details/chat/settings`), mixed-selection editing restrictions.
  - Derived: common values, replacement options, section labels based on cardinality.
  - Conditional: section activation on selection lane presence and priority.

#### Extensibility & Coupling
- How are new properties added?
  - Old: add branches and fields in monolithic details component.
  - Current: register a new details section with id/specificity/order and keep field logic scoped to that section.
- What is tightly coupled and should be abstracted?
  - Selection shape coupling (`connector` vs `connectors` forms).
  - UI section decisions embedded directly in imperative component effects.
  - Mixed-value/value-resolution logic duplicated per section.
- Which extension points are implicit but undocumented?
  - Selection normalization boundary.
  - Section planning contract from normalized selection to ordered section descriptors.
  - Transaction policy per interaction class (form/pointer/drag).
  - Shared fallback policy for missing/invalid entity references.

#### Infrastructure Dependencies
- What depends on framework patterns?
  - React hook timing and effect cleanup patterns currently drive section registration and side effects.
- What depends on legacy services/stores?
  - Old implementation assumes legacy editor command/store hooks and synchronous selection write behavior.
- What synchronous assumptions may no longer hold?
  - Immediate local read-after-set correctness and single-component ownership of layout state.
- What must be redesigned for new infrastructure?
  - Normalize selection once at boundary.
  - Move section decision logic to deterministic planner.
  - Keep panel stateless for layout/visibility/tab state.
  - Keep command-driven mutations with explicit transaction envelopes.
  - Preserve cross-view selection parity (model + diagram) as a hard invariant.

### Objective
- Extract what the old Property Panel does and why (`semio/js/sketchpad/Design.Details.tsx.old`, `semio/js/sketchpad/Design.Diagram.tsx.old`, `semio/js/sketchpad/Design.Model.tsx.old`, `semio/js/sketchpad/Desing.tsx.old`).
- Translate that intent into the current app architecture (`semio/js/sketchpad/Design.tsx`, `semio/js/sketchpad/Sketchpad.tsx`, `semio/js/sketchpad/elements.tsx`, `semio/js/sketchpad/shared.ts`).

### Purpose And Responsibilities
- User problem solved:
  - Give a single contextual editing surface for selected diagram entities (design, piece(s), connection(s), connector/port) with immediate visibility of editable and read-only metadata.
  - Collapse multiple editing entry points into one deterministic place so users can inspect and edit without navigating away from the canvas.
- Old panel responsibilities:
  - Own rendering selection routing (`none -> design`, `port -> port section`, `pieces -> piece section`, `connections -> connection section`, mixed piece+connection warning).
  - Own field-level mutation orchestration (`startTransaction`, mutation, `finalizeTransaction` / `abortTransaction`).
  - Own value harmonization for multi-selection (`getCommonValue`, "Mixed values" placeholders, bulk apply).
  - Own validation/degradation for partial data (`No valid pieces found`, `Port not found`).
  - Own local width UI state and resize interaction.
- Delegated in old panel:
  - Canonical model reads/writes delegated to editor/store hooks (`useDesign`, `useDesignEditorCommands`, `setPiece`, `setConnection`, `setDesign`, `executeCommand`).
  - Replacement candidate discovery delegated to store hooks (`useReplacableTypes`, `useReplacableDesigns`).
- Essential responsibilities:
  - Contextual section routing from selection state.
  - Transaction-safe mutations for gesture/form edits.
  - Multi-selection common-value semantics.
  - Graceful behavior for invalid/partial references.
- Accidental/legacy responsibilities:
  - Local panel width ownership in app component.
  - Hardwired chat/details mutual exclusion in the same component.
  - Direct coupling between section rendering and specific store shape (`selection.port`, object-based ids).

### Data Flow And State Model
- Old data origin:
  - Pull model/state from synchronous hooks each render (`useDesign`, `useDesignEditorSelection`, `useKit`, metadata hooks).
  - Selection shape used rich objects (`selection.pieces`, `selection.connections`, `selection.port`).
- Old propagation model:
  - Mostly push writes from inputs and gestures through command hooks with explicit transaction boundaries.
  - Per-event synchronous assumptions (focus/blur, pointerdown/up/cancel, drag start/stop).
- Old state assumptions:
  - Selection is mutable as a plain object and always immediately available.
  - A single render tree owns panel composition and visibility.
  - Width state is local and not cross-app persisted.
- Current data origin:
  - App state is sourced from Sketchpad machine context keyed by `kitGuid:designGuid` selectors (`createDesignSelectionSelector`, `createDesignPanelVisibilitySelector`).
  - Section content is composed through panel registries (`PanelSectionProvider`, `SidePanelTabProvider`) rather than a single static Details component.
- Current propagation model:
  - Event-driven writes through actor messages (`DESIGN.SET_SELECTION`, `DESIGN.SET_PANEL_VISIBILITY`) and command facades.
  - Global layout state (panel sizes, active side tab) is persisted in sketchpad state and reused across app instances.
- Current assumptions to avoid carrying forward:
  - Do not depend on synchronous local state updates for correctness.
  - Do not assume details panel is the only right-side content (chat/settings/property tabs are mutually exclusive at visibility layer).

### Interaction Mechanisms
- Selection change reactions (old):
  - Port selected: show dedicated port section, suppress piece/connection sections.
  - Pieces selected: show piece editor (single or multi variant).
  - Connections selected: show connection editor (single or multi variant).
  - Pieces + connections mixed: show warning section.
  - None selected: show design section.
- Selection change reactions (current):
  - `Design.tsx` useEffect computes the same conceptual rules and dynamically adds/removes detail sections with explicit IDs/specificity/order via `addSection`/`removeSection`.
  - Layout renders those sections inside a right side panel tab generated from panel configs.
- External state updates:
  - Old: rerender-based pull from hooks.
  - Current: selector-driven updates from machine context; panel content changes through registration updates.
- Invalid/partial data:
  - Old explicitly degrades (`No valid pieces found`, `Port not found`).
  - Current should keep this policy, but via section-level fallback views.
- Rule-based vs derived vs conditional:
  - Rule-based: mixed selection restrictions and right-side panel exclusivity (`rightSidePanel/chat/settings`).
  - Derived: common values and replacement candidates from selected entities.
  - Conditional: section activation by selection lane and selection cardinality.

### Extensibility And Coupling
- Old extensibility model:
  - Add fields/sections by editing monolithic `Details` module branches.
  - Implicit extension points existed but undocumented: replacement discovery hooks, transaction wrappers, metadata lookups.
- Current extensibility model:
  - Add/replace sections through panel section registration (`addSection("details", section)`), order, and specificity.
  - Add right-side experiences through side-panel tab registration and panel config mapping.
- Tight coupling to remove in redesign:
  - Section rendering directly reading unstable raw selection fields (`connector` vs `connectors`) from whichever producer wrote last.
  - Section logic depending on concrete storage shape instead of a normalized selection projection.
  - Business rules embedded directly in React effect cleanup cycles.
- Implicit extension points to formalize:
  - Selection-to-section planner.
  - Shared value harmonizer for multi-selection.
  - Shared transaction policy per interaction kind.
  - Shared "entity resolution with fallback" adapter.

### Infrastructure Dependencies And Required Redesign
- Legacy dependencies in old panel:
  - Framework-local state management (`useState` widths/toggles) and component-level resize handlers.
  - Direct store hook shape assumptions from legacy editor store.
  - Immediate synchronous command lifecycle expectation.
- Current infra dependencies that must shape redesign:
  - Sketchpad machine selectors and event dispatch.
  - Panel section registry and side panel tab system.
  - Global panel visibility and size ownership in `Sketchpad.tsx` + `elements.tsx`.
  - Multi-app composability where Details is one tab among right-panel contenders.
- Redesign requirements:
  - Keep panel stateless regarding layout, visibility, and active tab; panel content should be pure projection from app snapshot + resolved entities.
  - Introduce a normalized `PropertySelection` adapter boundary consumed by section planner (single connector representation regardless of source shape).
  - Move section planning into deterministic pure function: `(selection, modelAvailability, policy) -> section descriptors`.
  - Keep mutation entrypoints command-driven with standardized transaction envelopes by interaction class.
  - Preserve graceful partial-data behavior as explicit fallback sections, not thrown errors.

### Proposed Target Architecture
- Property planner pipeline:
  - `Raw app snapshot` -> `NormalizedSelection` -> `PropertyContext` -> `SectionPlan[]` -> `addSection/removeSection`.
- Section ownership:
  - Design app contributes detail sections only.
  - Layout owns rendering shell, tabs, exclusivity, and resize persistence.
- Transaction policy:
  - Form focus/blur edits: begin on focus, finalize on blur.
  - Pointer-based numeric controls: begin on pointerdown, finalize on pointerup, abort on pointercancel.
  - Drag gesture edits: begin on dragstart, finalize on dragstop, abort on escape/cancel.
- Compatibility strategy:
  - No backward-compat layer for old APIs.
  - Normalize once at boundary and delete dual-shape assumptions in downstream section logic.

### Decision Summary
- Keep: contextual property editing, mixed-selection semantics, bulk edit harmonization, resilient fallback rendering.
- Redesign: layout ownership, section registration mechanism, state selection normalization, mutation lifecycle standardization, cross-panel exclusivity management.
- Reject: monolithic details component as source of truth for panel composition and local width/visibility state.

### Trace
- Repo discovery:
  - `./semio-repo/cli/cli tree "property-panel"`
  - `./semio-repo/cli/cli ticket list`
  - `./semio-repo/cli/cli ticket reopen 2026/02/11/MAP-EDITOR-INSPECTOR-DETAILS-ARCHITECTURE "..."`
- Files inspected:
  - `semio/js/sketchpad/Design.Details.tsx.old`
  - `semio/js/sketchpad/Design.Diagram.tsx.old`
  - `semio/js/sketchpad/Design.Model.tsx.old`
  - `semio/js/sketchpad/Desing.tsx.old`
  - `semio/js/sketchpad/Design.tsx`
  - `semio/js/sketchpad/Sketchpad.tsx`
  - `semio/js/sketchpad/elements.tsx`
  - `semio/js/sketchpad/shared.ts`
  - `semio/js/README.md`

### Todo
- [x] Reopen matching existing ticket.
- [x] Extract old panel purpose/responsibilities from old build files.
- [x] Map old interaction and data flow assumptions.
- [x] Map current panel/section infrastructure and state pipeline.
- [x] Produce redesign blueprint decoupled from legacy assumptions.
- [x] Record findings in ticket.
## Changes
- Updated `semio/js/sketchpad/Design.tsx` `PiecesSectionForm` to use real metadata (`usePiecesMetadataMap`) instead of an empty map so parent-connection derivation works for selected pieces and included design pieces.
- Updated `semio/js/sketchpad/Design.tsx` `PiecesSectionForm` to restore old invalid-selection fallback behavior (`No valid pieces found in selection.`) when selected ids resolve to unknown placeholders.
- Updated `semio/js/sketchpad/Design.tsx` piece type resolution and mixed-value derivation to support both string and object (`{guid}` / `{name,variant}`) type shapes for parity with old panel behavior.
- Updated `semio/js/sketchpad/Design.tsx` piece detail visibility logic to match old intent (`hasCenter`/`hasPlane` now activate when any selected piece has values, not only when all do).
- Updated `semio/js/sketchpad/Design.tsx` `ConnectionsSectionForm` to restore multi-selection bulk editing for connection numeric/orientation fields (gap/shift/rise/rotation/turn/tilt/u/v) with transaction wrapping and batch update command path.
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
- Reopened ticket `2026/02/11/MIGRATE-PROPERTY-PANEL-FUNCTIONALITY-IN-CURRENT-BUILD`.
- Compared old details behavior (`Design.Details.tsx.old`) against current `Design.tsx` section forms and identified parity regressions limited to content behavior (no panel layout changes required).
- Implemented migration directly in existing details section components (`PiecesSectionForm`, `ConnectionsSectionForm`) without introducing new panels or changing panel positions.
- Verified with `npm run test` in `semio/js`: passed (`semio.test.ts`, 13 tests).
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
- [x] Reopen the migration ticket.
- [x] Restore missing property-panel content behaviors in current details sections only.
- [x] Keep panel topology unchanged (no new panels, no location changes).
- [x] Run test suite after migration changes.
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
