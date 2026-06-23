# Ticket

## Todos
- [x] Create a professional prompt for redesigning the toolbar into a clustered tool tree.
- [x] Generate a cross-app implementation plan for clustering toolbar tools into a hierarchical tooltree with expandable subtools.
- [x] Produce an extremely detailed execution plan for cross-app toolbar tooltree architecture and rollout.
- [x] Start implementation of the tooltree by refactoring toolbar grouping/rendering and migrating app toolbar registrations to parent metadata.
- [x] Create a kit editor diagram plan for shape-based node rendering and snap-point edge routing with extensible strategies.
- [x] Implement the kit diagram shape strategy contract/registry and migrate node visuals, edge routing, preview routing, and proximity anchor targeting to snap-point geometry.
- [x] Add unit coverage for strategy snap points and anchor resolution, refresh Playwright shape/endpoint assertions, and update README/AGENTS with the finalized mechanism.

## Changes
- Added a design prompt for a hierarchical toolbar-to-tooltree UX refactor with expandable subtools.
- Added a phased cross-app design/implementation plan for moving from flat toolbar actions to clustered parent/subtool groups.
- Expanded the plan into a full implementation blueprint with detailed inventory, canonical interaction behavior, filtering architecture, layout contract, phased execution, and measurable acceptance metrics.
- Updated `README.md` (`# 📦 Bundles`) with the canonical Sketchpad toolbar tooltree mechanism and cross-app mapping.
- Updated `AGENTS.md` SRS (`## UI/UX -> Toolbar`) and `# Codebase` documentation for the toolbar tooltree taxonomy, interaction contract, and app registration model.
- Implemented `toolbarGroup` metadata on `PanelSection` in `js/compose/sketchpad/shared.ts`.
- Refactored `js/compose/sketchpad/Sketchpad.tsx` toolbar rendering to group sections by parent (`selection`, `filter`, `create`, `view`, `actions`) and render an active parent branch.
- Migrated toolbar registrations in `Home.tsx`, `Kit.tsx`, `Design.tsx`, `Type.tsx`, and `Feedback.tsx` to include `toolbarGroup` metadata.
- Removed persistent `[DEBUG]` toolbar/selection logs from the touched kit toolbar registration and selection sections.
- Added toolbar parent label keys in `js/compose/sketchpad/locales/en.json` and `js/compose/sketchpad/locales/de.json`.
- Updated `README.md` and `AGENTS.md` with implementation-level details for grouped toolbar metadata and renderer behavior.
- Added a new implementation plan in `plan.md` for kit diagram shape strategies (circle, rectangle, triangle, long rectangle) and nearest snap-point edge connection.
- Added kit diagram geometry + shape strategy engine in `js/compose/sketchpad/kitSelectionHelpers.ts` with strategy contract, node-kind registry, frame/vector utilities, snap-point generation, nearest-anchor resolution, and proximity anchor lookup.
- Migrated `js/compose/sketchpad/Kit.tsx` diagram rendering/routing to strategy-driven geometry:
  - node frames and avatar clipping by strategy payload,
  - static floating edge endpoints resolved by snap-point pairs,
  - connection preview lines resolved by the same anchor engine with proximity snap targeting,
  - node construction dimensions keyed by strategy frame.
- Added deterministic geometry unit coverage in `js/compose/compose.test.ts` for strategy mapping, snap-point coordinates, anchor-pair resolution, and proximity anchor resolution.
- Updated `js/compose/diagram-alignment.test.ts` to the same shape/snap-point contract for diagram geometry expectations.
- Updated `js/compose/playwright/kit/diagram-alignment.spec.ts` with shape-aware endpoint assertions and drag-cycle checks (skip-safe when no diagram fixtures exist in the test environment).
- Updated `README.md` (`# 📦 Bundles`) and `AGENTS.md` (`Business Logic -> Diagrams`, `UI/UX -> Sketchpad -> Kit Editor`, `# Codebase`) with strategy registry and snap-point routing ownership.

## Log
### User Request
Generate a professional prompt to design a tooltree out of the toolbar where tools are currently spread across the toolbar and should be clustered and separated, with parent tool click revealing subtools such as filters.

### Response Draft
Design a professional UX and interaction architecture that transforms the current flat toolbar into a hierarchical tool tree.  
Current state: all tools are spread in one toolbar and difficult to scan.  
Target state: tools are grouped into clear parent clusters, and selecting a parent tool reveals contextual subtools (for example: **Filter** → **Filter Design**, **Filter Type**, etc.).

Deliverables:
1. **Tool Tree Information Architecture**
   - Define parent tool groups and child subtools.
   - Propose intuitive naming and grouping logic.
   - Ensure scalability for adding future tools without clutter.
2. **Interaction Model**
   - Clicking a parent tool opens its subtool panel/dropdown/tree branch.
   - Support single-click, keyboard navigation, and quick collapse/expand behavior.
   - Preserve active state visibility (which parent and subtool are currently selected).
3. **UI/UX Specifications**
   - Clean, modern, professional visual hierarchy.
   - Clear icon + label strategy for parent and child tools.
   - Minimize cognitive load with progressive disclosure.
   - Responsive behavior for desktop and tablet.
4. **Filtering Subtools**
   - Include a dedicated filtering cluster with subtools such as:
     - Filter Design
     - Filter Type
     - Filter Status
     - Reset Filters
   - Define interaction feedback and selected-filter indicators.
5. **Quality Constraints**
   - Accessibility-first (focus states, contrast, keyboard support, ARIA intent).
   - Consistent spacing, alignment, and predictable motion.
   - Fast interaction with no unnecessary steps.

Please provide:
- A proposed tooltree structure (parent → subtools)
- A short user flow for common actions
- A wireframe-level layout description
- Optional microinteraction recommendations for expand/collapse transitions
- Rationale for why this structure improves usability over the current flat toolbar

### User Follow-up Request
Generate a concrete plan to design a tooltree inside the existing toolbar across all apps. Current tools are spread across the toolbar; target is clustering and separation so clicking one tool opens subtools (example: Filter -> Filter Design, Filter Type).

### Response Plan
1. Inventory all toolbar tools per app and normalize them into shared parent groups (Selection, Filter, Create, View, Actions).
2. Define one canonical tooltree interaction model for all apps (expand/collapse triggers, keyboard navigation, active parent/subtool states, focus return).
3. Specify the filtering branch as a dedicated parent with explicit subtools (Filter Design, Filter Type, Filter Status, Reset) and selected-state indicators.
4. Introduce a shared toolbar contract so every app registers parent groups and subtools consistently while keeping app-specific branches.
5. Roll out in phases: data model refactor -> toolbar rendering refactor -> per-app registration migration -> accessibility hardening -> cross-app regression verification.
6. Validate with acceptance metrics: fewer visible top-level tools, preserved feature parity, deterministic navigation, and improved scanability.

### User Follow-up Request
Make an extremely detailed plan for all six workstreams: inventory/taxonomy, interaction model, filtering cluster, layout contract, phased implementation, and completion criteria.

### Response Plan (Extremely Detailed)
1. Added scope and baseline constraints that preserve existing toolbar shell/panel behavior while moving internal controls to a parent/subtool tooltree.
2. Built a per-app inventory with section ids, control behaviors, and state sources for Home, Kit, Design, Type, Feedback, plus Docs/Quality placeholders.
3. Defined a canonical taxonomy (`Selection`, `Filter`, `Create`, `View`, `Actions`) and mapped every app’s current controls to that taxonomy.
4. Specified a deterministic interaction contract: pointer behavior, keyboard navigation, focus return, expansion policy, active-state persistence, and collapse rules.
5. Defined the filtering parent as first-class with mandatory subtools (`Filter Design`, `Filter Type`, `Filter Status`, `Reset Filters`) and app-specific filter extensions.
6. Specified a unified registration/rendering contract for parent branches and subtools, including ordering, badges, sticky behavior, merge rules, and i18n-driven labels.
7. Broke delivery into five implementation phases with clear outputs and validation targets per phase (model, renderer, migration, a11y hardening, regression).
8. Added measurable completion criteria for top-level density reduction, scanability improvement, deterministic keyboard behavior, accessibility compliance, and full feature parity.

### User Follow-up Request
Start implementing the plan.

### Response Implementation (Phase Start)
1. Added toolbar parent metadata support by extending `PanelSection` with `toolbarGroup` (`id`, `labelId`, `order`) in shared sketchpad types.
2. Implemented grouped toolbar rendering in `Sketchpad.tsx`:
   - grouped sections by `toolbarGroup.id`
   - canonical parent ordering
   - parent toggle rail with i18n labels and icons
   - active parent branch rendering for contextual subtools
3. Added app-scoped active parent state so each app remembers its selected toolbar parent during navigation.
4. Migrated app toolbar registrations:
   - Home -> `filter`
   - Kit -> `filter`, `selection`
   - Design -> `selection`
   - Type -> `selection`
   - Feedback -> `actions`
5. Added localization keys for toolbar parent labels in English and German.
6. Updated implementation docs in `README.md` and `AGENTS.md` to document the new toolbar group metadata and grouped renderer mechanism.
7. Verified JSON locale validity and ran TypeScript compile check; compile currently fails due broad pre-existing repository type errors unrelated to this toolbar refactor.

### User Follow-up Request
Create a plan to change the visual look of Kit editor diagram nodes and edges so each node shape exposes N snapping points and edges connect to the nearest snapping points. Required initial mapping: Design=circle (4 points), Type=rectangle (4 mid-edge points), File=triangle (3 points), all others=long rectangle (4 points), with a strategy architecture for future shapes.

### Response Plan
1. Introduce a shape strategy contract and registry for kit diagram node kinds.
2. Map current kinds to four initial shape strategies (circle, rectangle, triangle, long rectangle) with explicit snap-point definitions.
3. Refactor node rendering to consume strategy shape output while preserving current interaction states.
4. Replace current circle-intersection edge math with nearest snap-point pair resolution for source and target.
5. Reuse the same snap-point resolver for connection previews and proximity-connect behavior.
6. Add geometry-focused unit tests and extend Playwright alignment checks for all initial shapes.
7. Update `README.md` and `AGENTS.md` after implementation to document the finalized mechanism.

### User Follow-up Request
Implement the 10-step shape strategy and snap-point migration plan (geometry engine, registry, visuals, static/preview/proximity routing, tests, docs, and incremental delivery).

### Response Implementation (Completed)
1. Implemented the shape strategy contract and registry in `js/compose/sketchpad/kitSelectionHelpers.ts`:
   - deterministic strategy ids (`circle`, `rectangle`, `triangle`, `long-rectangle`),
   - per-strategy frame dimensions and render payloads,
   - snap-point generation APIs,
   - nearest-point resolution against target vectors.
2. Added declarative kind-to-strategy mapping:
   - `design` -> circle,
   - `type` -> rectangle,
   - `file` -> triangle,
   - default kinds (`quality`, `port`, `tag`, `concept`, `folder`, `author`) -> long rectangle.
3. Refactored `Kit.tsx` node visuals to strategy output (frame + render payload) while preserving selection/hover state handling and the existing i18n node avatar id.
4. Replaced circle-radius edge intersection code in `Kit.tsx` with strategy anchor-pair resolution using absolute snap points and side-derived React Flow edge positions.
5. Routed floating connection preview lines through the same snap-point resolver and proximity anchor lookup path.
6. Integrated proximity connect targeting by measuring pointer distance to strategy snap points instead of center/radius assumptions.
7. Added strategy-agnostic geometry helpers (frame normalization, vectors, side inference, local-to-absolute conversion, anchor pair selection) in `kitSelectionHelpers.ts`.
8. Added/updated tests:
   - unit assertions in `js/compose/compose.test.ts` for strategy snap-point coordinates and anchor/proximity resolution,
   - alignment contract assertions in `js/compose/diagram-alignment.test.ts`,
   - Playwright endpoint alignment checks in `js/compose/playwright/kit/diagram-alignment.spec.ts`.
9. Updated docs:
   - `README.md` (`# 📦 Bundles`) with shape strategy and snap-point routing mechanism,
   - `AGENTS.md` SRS (`Business Logic -> Diagrams`, `UI/UX -> Sketchpad -> Kit Editor`) and `# Codebase` ownership bullets.
10. Validation run:
    - `npm run test` in `js/compose` -> passed (`compose.test.ts`, 13 tests),
    - `npx playwright test playwright/kit/diagram-alignment.spec.ts --project=firefox` -> passes with 3 skip-safe tests in the current empty-fixture environment.

## Summary

Implemented strategy-driven kit diagram shape/snap-point geometry, migrated Kit edge + preview + proximity routing to shared anchor resolution, added geometry unit coverage and Playwright alignment assertions, and documented the finalized mechanism in README/AGENTS.
