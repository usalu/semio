# Plan

## 11) 2026-02-03 Shape Strategy Delivery (Completed)

1. Implement kit diagram shape-strategy contract and strategy registry in `js/compose/sketchpad/kitSelectionHelpers.ts`.
2. Migrate `js/compose/sketchpad/Kit.tsx` node rendering, edge anchor resolution, connection preview routing, and proximity anchor targeting to strategy snap points.
3. Add deterministic geometry unit coverage in `js/compose/compose.test.ts` and keep `js/compose/diagram-alignment.test.ts` aligned to the same shape/snap-point contract.
4. Update Playwright alignment assertions in `js/compose/playwright/kit/diagram-alignment.spec.ts` for shape-aware endpoint checks (skip-safe when fixtures are empty).
5. Update dev docs in `README.md` and `AGENTS.md` with strategy registry, snap-point routing, and resolver ownership details.
6. Validate with `npm run test` in `js/compose` and `npx playwright test playwright/kit/diagram-alignment.spec.ts --project=firefox` in `js/compose`.

## 10) 2026-02-03 Execution Slice (Current Run)

1. Reopen and continue ticket `2026/02/02/DESIGN-TOOLBAR-TOOL-TREE-PROMPT` with the shape-strategy + snap-point scope as primary.
2. Audit current toolbar migration changes in `Sketchpad.tsx`, `Home.tsx`, `Kit.tsx`, `Design.tsx`, `Type.tsx`, and `Feedback.tsx` to keep or complete only canonical tooltree behavior.
3. Implement shape strategy geometry engine and registry in kit diagram code, then migrate node rendering and edge anchor resolution to strategy snap points.
4. Route static edges, preview edges, and proximity-connect logic through one shared anchor resolver utility.
5. Add/adjust unit tests for shape snap points and anchor-pair resolution, then update Playwright alignment assertions for each supported node shape.
6. Update `README.md` and `AGENTS.md` for SRS + codebase documentation of tooltree metadata and kit snap-point routing.
7. Record implementation log + summary in ticket workspace artifacts, close ticket with touched files list.

## 9) Kit Diagram Shape Strategy and Snap-Point Edge Plan (Current Task)

1. Define a shape strategy contract for kit diagram nodes with deterministic geometry APIs: shape id, node frame dimensions, render payload, snap-point generation, and nearest-point resolution against a target vector.
2. Add a strategy registry keyed by diagram node kind so shape assignment is declarative and extensible:
   - `design` -> circle strategy with 4 snap points (N, E, S, W).
   - `type` -> rectangle strategy with 4 snap points (midpoint of each edge).
   - `file` -> triangle strategy with 3 snap points (apex + two base corners or base midpoint pair depending on visual direction).
   - default (`quality`, `port`, `tag`, `concept`, `folder`, `author`) -> long-rectangle strategy with 4 snap points (midpoint of each edge with elongated width ratio).
3. Refactor kit node rendering so visual shape comes from strategy output instead of a single avatar box, while preserving current hover/selection state, i18n ids, and pointer behavior.
4. Replace circle-radius edge intersection math in `Kit.tsx` with snap-point pair resolution:
   - compute absolute snap points for source and target nodes from their active strategies,
   - choose source/target anchors by nearest compatible pair along center-to-center direction,
   - derive `sourcePosition` and `targetPosition` from chosen anchors for React Flow bezier generation.
5. Route connection preview lines through the same snap-point resolver so static edges and interactive connection lines share one geometry mechanism.
6. Integrate proximity-connect behavior with the snap-point resolver by evaluating distance from pointer/target to strategy snap points instead of implicit center/radius assumptions.
7. Add small geometry utility helpers (frame normalization, vector math, side inference, absolute coordinate conversion) and keep them strategy-agnostic for future shape additions.
8. Add tests:
   - unit tests for strategy snap-point coordinates per shape,
   - unit tests for nearest-anchor selection between shape pairs,
   - Playwright assertion updates for edge endpoint alignment on circle/rectangle/triangle/long-rectangle nodes during drag and after simulation ticks.
9. Update docs after implementation:
   - `README.md` (`# 📦 Bundles`) with shape strategy mechanism and snap-point routing overview,
   - `AGENTS.md` SRS (`# UI/UX` and `# Business Logic`) with shape/snap-point requirements,
   - `AGENTS.md` `# Codebase` with the concrete kit diagram strategy registry and edge resolver locations.
10. Execute incrementally in three delivery slices: geometry engine + registry, node visuals + edge routing migration, then test/doc hardening and regression pass.

## 8) Implementation Sprint (Current Task)

1. Extend toolbar section metadata in shared sketchpad types so each toolbar contribution can declare a parent tooltree group (selection/filter/create/view/actions), label id, and parent order.
2. Refactor toolbar rendering in `Sketchpad.tsx` from flat section list to grouped parent rail + expandable parent content panel with app-scoped active parent persistence.
3. Add canonical parent ordering and parent label resolution via i18n ids with fallback labels.
4. Migrate toolbar registrations in Home, Kit, Design, Type, and Feedback to provide tooltree parent metadata.
5. Remove temporary `[DEBUG]` logs from kit toolbar registration/selection paths while touching those sections.
6. Update `README.md` and `AGENTS.md` with implementation-level details for the new toolbar group metadata and grouped renderer behavior.
7. Record implementation details and completion summary in ticket workspace artifacts and close the ticket.

## 0) Scope, Ground Rules, and Baseline

1. Keep the existing floating toolbar placement and panel plumbing in `js/compose/sketchpad/Sketchpad.tsx`; refactor only the internal toolbar content model from flat sections to parent/subtool groups.
2. Apply one shared tooltree model across Home, Kit, Design, Type, Feedback, and reserve extension points for Docs/Quality even if they currently register no toolbar sections.
3. Preserve all current app behaviors (no functional drop): same actions, same state outcomes, same URL filter semantics, and same active-tool integrations with app state machines.
4. Treat this as a structural refactor with no backward-compatibility requirements for old toolbar section shape, but maintain temporary adapter bridges during migration phases.
5. Keep i18n-first IDs for all visible labels and tooltips; no raw text in toolbar UI.

## 1) Exhaustive Toolbar Inventory and Taxonomy Mapping

### 1.1 Inventory Collection Procedure

1. Enumerate all `addSection("toolbar", ...)` registrations in:
   - `js/compose/sketchpad/Home.tsx`
   - `js/compose/sketchpad/Kit.tsx`
   - `js/compose/sketchpad/Design.tsx`
   - `js/compose/sketchpad/Type.tsx`
   - `js/compose/sketchpad/Feedback.tsx`
2. Enumerate all tool widgets rendered inside those sections (`Toggle`, `ToggleGroup`, `Button`, `ToolGroup`).
3. Capture each control with:
   - app
   - section id
   - control id
   - behavior type (toggle, toggle+action, dropdown mode, direct action)
   - current state source (URL params, app store field, machine state, local state)
4. Build a normalized inventory matrix used as migration truth source.

### 1.2 Current-State Inventory (Per App)

1. **Home**
   - Section: `compose.sketchpad.app.home.toolbar.filters`
   - Controls:
     - `showTemporary` + `createTemporary`
     - `showLocal` + `createLocal`
     - `showRemote` + `createRemote`
   - State: URL `kind` param (single active kind) + create action.
2. **Kit**
   - Sections:
     - `compose.sketchpad.app.kit.toolbar.filters`
     - `compose.sketchpad.app.kit.toolbar.selection`
   - Filter controls (all toggle+action):
     - Designs, Types, Qualities, Ports, Tags, Concepts, Files, Folders, Authors
   - Selection controls:
     - Pointer (`SELECTION_NORMAL`)
     - Hand (`HAND`)
   - State: URL multi-kind filters + kit app active tool.
3. **Design**
   - Section: `compose.sketchpad.app.design.tools`
   - `ToolGroup` clusters:
     - Selection modes: normal/additive/subtractive
     - Lasso modes: rectangular/freeform
   - State: design app active tool.
4. **Type**
   - Section: `compose.sketchpad.app.type.tools`
   - `ToolGroup` clusters:
     - Selection modes: normal/additive/subtractive
     - Connector mode
   - State: type app active tool.
5. **Feedback**
   - Section: `compose.sketchpad.app.feedback.toolbar.send`
   - Control:
     - Send button (triggers form submit button click)
   - State: form submission lifecycle.
6. **Docs/Quality**
   - No active toolbar sections currently.
   - Must still conform to new registration contract with empty/default branch support.

### 1.3 Shared Parent Taxonomy (Canonical)

1. **Selection**
   - Pointer/select modes, additive/subtractive variants, lasso variants, hand/pan mode.
2. **Filter**
   - Visibility/attribute filters by app domain (kind/type/design/status/concepts/etc).
3. **Create**
   - Artifact creation actions attached to relevant filter domains.
4. **View**
   - Presentation/view-mode toggles and window-scoped visual tools (reserved for growth).
5. **Actions**
   - Stateless commands (send/reset/apply/etc).

### 1.4 App-to-Taxonomy Mapping Target

1. **Home** -> Filter + Create
2. **Kit** -> Filter + Create + Selection
3. **Design** -> Selection (and future Filter/View)
4. **Type** -> Selection + Create (connector creation mode)
5. **Feedback** -> Actions
6. **Docs/Quality** -> Contract placeholders for future Filter/View/Actions

## 2) Canonical Tooltree Interaction Model

### 2.1 Interaction State Model

1. Add toolbar tooltree state with explicit fields:
   - `activeParentByApp`
   - `expandedParentsByApp` (set)
   - `activeSubtoolByParent`
   - `focusAnchorByParent` (for focus return)
   - `lastInteractionMode` (pointer vs keyboard)
2. Persist per-app active parent + subtool to existing app state persistence channel so the toolbar restores user context.
3. Keep source-of-truth state for functional behavior in app machines/stores; tooltree state only orchestrates UI grouping and open/close behavior.

### 2.2 Pointer Interaction Rules

1. Parent single click:
   - If collapsed -> expand parent and focus first enabled subtool.
   - If expanded and same parent active -> collapse parent.
   - If another parent active -> switch active parent and expand new parent.
2. Subtool click:
   - Execute subtool action.
   - Mark parent active.
   - Preserve expansion if parent is sticky; otherwise auto-collapse based on parent policy.
3. Outside click:
   - Collapse non-sticky expanded parents.
4. Re-click behavior:
   - For mode parents (Selection), re-click parent cycles default mode only when branch is collapsed.
   - For filter parents, re-click does not mutate selected filters unless explicit subtool interaction.

### 2.3 Keyboard Interaction Rules

1. `Tab` / `Shift+Tab`: move between parent nodes and expanded subtool lists in natural DOM order.
2. `ArrowRight`: expand current parent (or move into subtool list if already expanded).
3. `ArrowLeft`: collapse current parent (or return from subtool list to parent trigger).
4. `ArrowDown` / `ArrowUp`: move within sibling subtools of expanded parent.
5. `Enter` / `Space`: activate focused parent or subtool.
6. `Escape`: collapse current parent and restore focus to parent trigger.
7. Home/End keys (optional but recommended): jump to first/last parent trigger.

### 2.4 Active-State and Collapse Policy

1. Always show one active parent per app when any tool is selected.
2. Allow multiple expanded parents only if explicitly configured; default to single-expanded-parent mode for scanability.
3. Keep parent-level active indicator even when collapsed.
4. Keep subtool-level selected indicator inside expanded branch.
5. On app route change, clear expansion and restore only persisted active parent/subtool.

### 2.5 Accessibility Contract

1. Parent triggers expose `aria-expanded`, `aria-controls`, and `aria-pressed` where relevant.
2. Subtool groups use semantic group labeling (`aria-label` via i18n id).
3. Full keyboard support without pointer dependency.
4. Visible focus ring on parent and subtool nodes.
5. No color-only selection indicators; include shape/state contrast.

## 3) Filtering Cluster as First-Class Parent

### 3.1 Filter Parent Structure

1. Parent node: `Filter`
2. Subtools:
   - `Filter Design`
   - `Filter Type`
   - `Filter Status`
   - `Reset Filters`
3. App-specific enrichments:
   - Home: `Filter Kind` (temporary/local/remote)
   - Kit: `Filter Artifact Kind` (design/type/quality/port/tag/concept/file/folder/author)
   - Design/Type: placeholder filter hooks for future graph/model filtering

### 3.2 Filter Value UX Rules

1. Multi-select filters show per-subtool selected counts.
2. Single-select filters show current value chip.
3. `Reset Filters` is always visible inside filter branch and disabled when no active filter.
4. Filter state changes are reflected immediately in table/diagram/list synchronization.
5. Filter changes preserve current selection tool mode (no forced reset to pointer unless explicitly configured).

### 3.3 Filter State Synchronization

1. Continue URL query synchronization for Home/Kit filter semantics.
2. Normalize URL key mapping through one serializer per app to avoid divergent param handling.
3. Ensure filter reset clears only filter params, not unrelated route params (name/version/scope).

## 4) Unified Toolbar Layout Contract (Cross-App)

### 4.1 Registration API Shape

1. Replace app-level flat toolbar fragments with one tooltree registration per app.
2. Registration payload per parent group includes:
   - `id`
   - `labelId`
   - `icon`
   - `order`
   - `kind` (selection/filter/create/view/action)
   - `sticky` (expand policy)
   - `defaultSubtoolId`
   - `subtools[]`
3. Subtool payload includes:
   - `id`
   - `labelId`
   - `icon`
   - `order`
   - `disabled`
   - `selected`
   - `onSelect`
   - `badge` (count/value indicator)

### 4.2 Rendering Contract

1. Toolbar container keeps current bottom-center panel shell and panel level.
2. Internal layout becomes:
   - Parent rail (compact horizontal strip)
   - Expanded branch panel (contextual subtools)
3. Section ordering:
   - global parent order first by taxonomy then app-specific order.
4. Visual separation:
   - cluster separators between parent groups.
   - explicit active parent highlight.

### 4.3 App Branch Contribution Rules

1. Apps may contribute custom subtools under shared parent ids.
2. Apps may define app-only parents if no shared parent fits, but must tag with taxonomy fallback.
3. Duplicate parent ids from same app are merged by `order`.
4. Unknown or missing icons fall back to generic action icon token.

## 5) Phased Implementation Plan

### Phase A — Data Model + Registry Refactor

1. Add tooltree domain types and selectors in shared toolbar state definitions.
2. Add adapter from legacy `ToolGroup`/toggle controls to new parent/subtool schema.
3. Add persistence keys for active parent/subtool.
4. Deliverable: tooltree state can be constructed without rendering changes.

### Phase B — Toolbar Renderer Refactor

1. Refactor toolbar render path in `Sketchpad.tsx` to consume tooltree model.
2. Implement parent rail + expandable branch panel.
3. Implement keyboard navigation, focus management, and collapse controller.
4. Deliverable: one app (Design) rendered end-to-end on new tooltree path.

### Phase C — App Registration Migration

1. Migrate Home registrations to Filter/Create tooltree branch.
2. Migrate Kit registrations to Filter/Create/Selection branches.
3. Migrate Design and Type `ToolGroup` mappings into Selection parent subtools.
4. Migrate Feedback send action into Actions parent.
5. Add empty branch registration stubs for Docs/Quality.
6. Deliverable: no app uses legacy flat toolbar section shape.

### Phase D — Interaction + Accessibility Hardening

1. Add full keyboard contract verification scenarios.
2. Add ARIA attributes and role checks.
3. Validate focus restoration and escape behavior.
4. Remove temporary debug logs or keep only removable `[DEBUG]` diagnostics.
5. Deliverable: interaction spec compliance complete.

### Phase E — Cross-App Regression Validation

1. Validate existing flows still work:
   - Home kit kind filters + create flows
   - Kit artifact filter + create flows + selection mode switching
   - Design mode switching (selection/lasso)
   - Type mode switching (selection/connector)
   - Feedback send action
2. Validate state persistence and route transition behavior.
3. Validate no toolbar panel breakage in desktop/mobile layouts.
4. Deliverable: parity checklist fully green.

## 6) Completion Criteria and Success Metrics

1. **Top-level density reduction**
   - Parent nodes shown by default per app <= 5 (Selection, Filter, Create, View, Actions).
2. **Scanability improvement**
   - First-action discovery time reduced in QA scripts versus flat toolbar baseline.
3. **Deterministic keyboard navigation**
   - Same key sequence always reaches same parent/subtool from same state.
4. **Feature parity**
   - Every existing toolbar action remains reachable and functional.
5. **State integrity**
   - No regressions in machine/store state updates from toolbar actions.
6. **Accessibility**
   - Toolbar interaction is fully operable without pointer.

## 7) Execution Checklist (Ready-to-Implement)

1. Finalize inventory matrix and taxonomy map.
2. Freeze canonical interaction spec (pointer + keyboard).
3. Approve tooltree registration schema.
4. Implement Phase A/B infrastructure.
5. Migrate app branches Phase C.
6. Harden + validate Phase D/E.
7. Remove migration adapters and mark tooltree as canonical toolbar mechanism.
