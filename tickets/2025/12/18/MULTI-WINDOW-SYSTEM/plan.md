# Previously

The sketchpad already contains a GoldenLayout-based multi-window renderer (`LayoutCanvas`), and some apps already persist a `windowLayout`.

Current multi-window integration is inconsistent across apps:

- `AppWindowConfig` / `WindowKindDefinition` / `createDefaultLayout` are declared in more than one place.
- `windowLayout` persistence format is inconsistent (raw object vs JSON string).
- Each app re-implements similar boilerplate (default layout building, config building, layout change handlers, layout validation).
- Some apps still use a single-window wrapper rather than the GoldenLayout layout host.

The goal is to make every app a first-class multi-window system where each app registers window kinds and uses a single generalized layout/persistence mechanism.

# Plan

## Goals

- **Unify** window contracts and layout host usage across all apps.
- **Require** every app to register its supported window kinds.
- **Generalize** layout persistence and validation/migration.
- **Remove duplication** in window config/default layout wiring.
- **Enable** future features (window commands, presets, layout reset, per-scope persistence) with minimal additional work.

## Non-Goals

- Implement new product-level windows or redesign UX beyond what is required to unify the architecture.
- Provide backwards compatibility for legacy state shapes (the codebase is under refactor; breaking changes are acceptable).

## Key Decisions (lock early)

- **Canonical types location**: pick one source of truth for window-related types and helpers and remove duplicates.
- **Persistence format**: store `windowLayout` consistently as a **JSON string** in Yjs for all apps.
- **Validation strategy**: sanitize stored layouts against registered window kinds; reset to default if invalid/empty.
- **Registration mechanism**: every app must provide `getWindows()` (or equivalent) via `AppConfig` so the host can query window kinds without importing app internals.

## Architecture (target end state)

### Window Kind

A window kind is a reusable, app-defined content renderer.

- `id`: stable string identifier
- `label`: UI label (string key or resolved label)
- `icon`: optional icon
- `component`: `(props) => ReactNode` (window content)
- `controls`: optional controls surfaced by the host (toggle/dropdown)
- `variants`: optional variants that map to different `componentProps`

### App Window Registry

Each app registers:

- `windowKinds`: list of supported kinds
- `defaultLayout`: initial GoldenLayout config referencing the window kind ids
- optional: `sanitizeLayout(layout)` or `requiredWindowKinds` (to enforce invariants like "Design must have Scene")

### Layout Host

A single generalized `LayoutCanvas` renders GoldenLayout and uses the app window registry to resolve components.

Responsibilities:

- render windows by `componentName` == window kind id
- add-window UI (splitter hover / add window) via window kinds
- emit `onLayoutChange` with serializable config
- maintain `activeWindow` if required by UX

### Persistence

Each app store/state provides:

- `windowLayout?: string` (JSON)
- read helper: parse JSON or `undefined`
- write helper: stringify JSON or delete

Layout is stored per app scope (e.g. per kit/design/type/quality id) as part of the app state.

## Milestones

### Milestone A — Inventory & invariants (fast, required)

- Identify all existing window/layout related declarations:
  - duplicated type declarations
  - per-app `windowLayout` fields and persistence
  - per-app default layouts and invariants
- Define a strict list of invariants for each app:
  - which window kinds exist
  - which window kinds are required
  - whether multi-window is optional or mandatory per app scope

**Acceptance criteria**

- A written mapping of each app -> window kinds -> default layout -> persistence shape -> invariants.

### Milestone B — Canonicalize window contracts (types + helpers)

- Choose canonical location for window contract types and helpers.
- Remove duplicates and update imports.

**Concrete tasks**

- Ensure only one definition of:
  - `WindowKindDefinition`
  - `AppWindowConfig`
  - `createDefaultLayout`
- Ensure `LayoutCanvas` only depends on canonical types.

**Touchpoints**

- `js/js/sketchpad/shared.ts`
- `js/js/sketchpad/Sketchpad.tsx`

**Acceptance criteria**

- There is only one exported `AppWindowConfig` type.
- Apps compile without referencing the removed duplicate declarations.

### Milestone C — Canonicalize layout persistence (JSON string everywhere)

- Provide shared helpers:
  - `parseWindowLayout(str)`
  - `stringifyWindowLayout(layout)`
  - `sanitizeWindowLayout(layout, allowedKindIds, requiredKindIds?)`
- Migrate each app’s store/state to the same persistence format.

**Concrete tasks**

- Convert apps storing raw objects (e.g. `Kit`) to JSON string storage.
- Keep existing apps that already stringify (e.g. `Design`, `Quality`) but route through the shared helper.

**Touchpoints**

- `js/js/sketchpad/shared.ts` (or a dedicated shared module)
- `js/js/sketchpad/Kit.tsx`
- `js/js/sketchpad/Design.tsx`
- `js/js/sketchpad/Quality.tsx`
- `js/js/sketchpad/Home.tsx` (once it participates)

**Acceptance criteria**

- All apps persist `windowLayout` as a string.
- No app persists a non-string layout value in Yjs.

### Milestone D — Add window registry to `AppConfig` (registration)

Extend `AppConfig` to allow apps to register their window kinds and defaults.

**Concrete tasks**

- Add optional function to `AppConfig`:
  - `getWindows: (...) => AppWindowConfig`
- Define a stable context signature for `getWindows` so apps can derive scope-dependent windows if needed.

**Touchpoints**

- `js/js/sketchpad/shared.ts` (`AppConfig`)
- App configs:
  - `js/js/sketchpad/Home.tsx`
  - `js/js/sketchpad/Kit.tsx`
  - `js/js/sketchpad/Design.tsx`
  - `js/js/sketchpad/Type.tsx`
  - `js/js/sketchpad/Quality.tsx`
  - `js/js/sketchpad/Docs.tsx`
  - `js/js/sketchpad/Feedback.tsx`

**Acceptance criteria**

- Every app provides a window config.
- The host can query window kinds for the currently active app.

### Milestone E — Generalize per-app boilerplate into shared hook/util

Remove repeated code patterns from apps.

**Concrete tasks**

- Add a shared helper/hook that binds:
  - `storedLayout` + default layout
  - sanitation against window kinds
  - `onLayoutChange => store.change({ windowLayout })`
- Apply to each app.

**Touchpoints**

- `js/js/sketchpad/Sketchpad.tsx` (best place for shared UI hooks currently)
- `js/js/sketchpad/Design.tsx`
- `js/js/sketchpad/Kit.tsx`
- `js/js/sketchpad/Quality.tsx`
- `js/js/sketchpad/Type.tsx`
- `js/js/sketchpad/Home.tsx`

**Acceptance criteria**

- Apps no longer implement their own `windowLayout || defaultLayout` composition.
- Apps no longer implement bespoke layout change handlers.

### Milestone F — Migrate remaining apps to GoldenLayout host

Ensure every app renders via `LayoutCanvas` even when there is only one window.

**Concrete tasks**

- Migrate single-window apps to a single-kind multi-window configuration:
  - `Feedback`: `Form` window kind, default single window layout
  - `Docs`: `Page` window kind, default single window layout (later: tree/outline as separate kinds)
  - `Home`: `Table` window kind, default single window layout (later: additional views)
- Ensure consistent persistence for these apps.

**Touchpoints**

- `js/js/sketchpad/Feedback.tsx`
- `js/js/sketchpad/Docs.tsx`
- `js/js/sketchpad/Home.tsx`

**Acceptance criteria**

- Every app page renders a `LayoutCanvas`.
- Every app has at least one window kind.

### Milestone G — Enforce invariants and add layout migrations

Add migration hooks so changes to window kind ids / defaults do not brick stored layouts.

**Concrete tasks**

- Introduce `layoutVersion` per app (optional) and migrate stored layouts.
- For apps with required windows, enforce them:
  - `Design` must always include `Scene` (and optionally `Diagram`).

**Acceptance criteria**

- Stored layouts are auto-repaired or reset when window kinds change.
- No app can end up with an empty layout.

### Milestone H — Add shared commands (optional follow-up)

Once all apps are on the unified system:

- Command palette actions:
  - add window
  - close active window
  - reset layout
  - save/load preset
- Persist presets under `sketchpad.settings.apps[appId]`.

## App-by-app Notes (current expectations)

- **Design**: already uses GoldenLayout; generalize layout validation (currently checks for Scene).
- **Kit**: already uses GoldenLayout; normalize persistence to JSON string.
- **Quality**: already uses GoldenLayout; route persistence through shared helpers.
- **Type**: ensure it renders via GoldenLayout consistently (single window is still a window config).
- **Home**: define window kinds and move main table content into a window kind.
- **Docs**: migrate from direct `Window` wrapper to GoldenLayout.
- **Feedback**: migrate from direct `Window` wrapper to GoldenLayout.

## Global Acceptance Criteria

- Every app has a window registry (`getWindows`) and uses `LayoutCanvas`.
- Window contract types exist in one place.
- `windowLayout` persistence is uniform (JSON string).
- Stored layout invalidation is handled consistently (sanitize/reset).
- App code contains no repeated layout boilerplate that can be centralized.

# Changes

- Canonicalized window contract types and helpers in `js/js/sketchpad/shared.ts`.
- Normalized `windowLayout` persistence to JSON strings using `parseWindowLayout` / `stringifyWindowLayout`.
- Updated `LayoutCanvas` to parse persisted layout state and default layouts consistently.
- Migrated `Home`, `Docs`, and `Feedback` to persist `windowLayout` as JSON strings.
- Updated `README.md` and `AGENTS.md` to document the multi-window system and window chrome requirements.
