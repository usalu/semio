# Ticket

## Todos
# Previously

# Plan

State management & overfetch refactor plan

**Scope of this plan:** the uploaded Sketchpad apps/components:

- `Sketchpad.tsx`, `Docs.tsx`, `Home.tsx`, `Feedback.tsx`, `Kit.tsx`, `Design.tsx`, `Type.tsx`, `Quality.tsx`, `Tutorials.tsx`

**Main goals**

1. **Stop overfetching** (components should only subscribe to the smallest state slice they actually need).
2. **Use the correct mechanism consistently** (prefer `registerEventHandler` + XState triadic hooks; avoid duplicate mechanisms for the same state changes).
3. **Remove derived/duplicated state** (no “copy store state into `useState` then re-sync via `useEffect`” unless there is a clear, documented reason).
4. **Reduce eager module/data loading** (avoid loading every app module / every MDX page at startup).
5. **Make rerenders predictable** (fix missing dependencies and remove `react-hooks/exhaustive-deps` disables where possible).

---

## 1) Findings: state-management smells & why they matter

### A. Derived state in React components (copying store-derived data into local `useState`)

**Symptom:** `useState(baseNodes)` followed by `useEffect(() => setNodes(baseNodes), [baseNodes])`.

**Examples**

- **Design diagram nodes** are derived from `designToNodesAndEdges(...)` and then copied into local state, re-synced via effect.
  - This can overwrite user interaction mid-drag, creates “two sources of truth”, and makes it easier to ship stale state bugs.
  - Also: the focus list effect depends only on `[nodes, edges]` but reads `focusContext`, so focus updates can be missed when focus context changes.

**Fix direction**

- Keep store-derived nodes as **computed** (`useMemo`) and keep only true UI-ephemeral state locally (e.g. “current drag position”, “pending updates”).
- If ReactFlow requires controlled nodes for interaction, store the authoritative diagram positions in the app store (Yjs/XState) and treat the ReactFlow nodes as a projection.

---

### B. Guarded initialization with `hasInitialized` that never resets when the route scope changes

**Symptom:** `const hasInitialized = useRef(false)` with:

```ts
useLayoutEffect(() => {
  if (hasInitialized.current || !kitGuid || !designGuid) return;
  actor.send({ type: "DESIGN.INIT", ... });
  hasInitialized.current = true;
}, [kitGuid, designGuid]);
```

**Why it’s a smell**

- If the component remains mounted and `kitGuid/designGuid/typeGuid` changes (route change within the same component tree), INIT will never run again → stale XState state.

**Fix direction**

- Key the initialization ref by `(kitGuid, designGuid, typeGuid, qualityGuid)` or remove the ref and instead compute an initialization “key”:
  - `const initKey = kitGuid + ":" + designGuid;`
  - `useRef<string | null>(null)` and re-init when key changes.
- Ensure cleanup: if you install subscriptions or set callbacks, tear them down when scope changes.

---

### C. Duplicate state update mechanisms: `registerEventHandler` **and** `registerRuntimeAction` for the same events

**Symptom:** Home/Feedback register _both_ event handlers and runtime actions that appear to do the same updates.

**Why it’s a smell**

- Two sources of truth; unclear which path runs in production.
- Makes behavior change unexpectedly when refactoring the machine pipeline.

**Fix direction**

- Standardize:
  - **Preferred:** `registerEventHandler` for event-driven context diffs (per `AGENTS.md` guidance).
  - **Fallback:** `registerRuntimeAction` only for cross-cutting actions that must be invoked from multiple places or avoid circular deps.
- Remove duplicates once the “single path” is confirmed.

---

### D. Global mutable state outside of the app state model

**Examples**

- `docsPanelVisibilityState` in `Sketchpad.tsx` is a module-level variable with manual subscribe/notify.
- Docs “headings” state uses a global store-ish pattern and timeouts to update.

**Why it’s a smell**

- Hidden singleton state that persists across mounts.
- Harder to reset per kit/app scope.
- Easy to leak listeners or end up with stale values across navigation.

**Fix direction**

- Move these into:
  - XState docs app state (`DocsAppState.panelVisibility`, headings list + active heading), **or**
  - a dedicated Docs `PlainAppStore` owned by SketchpadStore so it follows lifecycle/scope.

---

### E. Eager module/data loading (overfetch at startup)

**Examples**

- SketchpadStore `_loadModules()` eagerly imports `Design/Home/Kit/Type/Quality` modules.
- `Docs.tsx` eagerly imports all `./pages/**/*.mdx` modules via `import.meta.glob(..., { eager: true })`.

**Why it’s a smell**

- Increases initial JS cost and memory footprint, even when the user only visits one route/app.
- Hurts TTI and makes profiling noisy.

**Fix direction**

- Convert to **lazy-by-route**:
  - Use `React.lazy(() => import("./Design"))` etc at the router/switch boundary.
  - Extract _lightweight_ config (id, route, icons, panel defs) into `*.config.ts` so Sketchpad can register apps without importing huge modules.

---

### F. “Deep sync everything” subscriptions that cause wide rerenders

**Examples**

- `useSyncDeep(store, (s) => s)` and similar patterns, then sending sync events into XState on every change.

**Why it’s a smell**

- Any tiny change in Yjs store triggers a full-state snapshot, JSON compare, rerender, and a machine event.
- Can cascade into large rerender trees.

**Fix direction**

- Subscribe to specific fields:
  - `useSyncDeep(store, (s) => s.windowLayout)` etc (already used in places), and prefer even narrower.
- For Yjs → XState bridging: send targeted events (e.g. `KIT.SYNC_PANEL_VISIBILITY`, `KIT.SYNC_SELECTION`) rather than re-sending the entire state blob.

---

### G. Intentional `exhaustive-deps` disables in focus/headings effects

**Examples**

- Docs/Kit use `// eslint-disable-next-line react-hooks/exhaustive-deps` around effects that install handlers.

**Why it’s a smell**

- Makes bugs likely when callbacks/context objects change.
- Usually indicates missing stable wrappers (`useCallback`) or missing deps.

**Fix direction**

- Make the setter functions stable (or wrap them) and include deps.
- If disabling is absolutely necessary, add a comment explaining why the referenced functions are stable by contract.

---

## 2) Refactor strategy

### Principle 1: Make subscriptions explicit and narrow

- **Bad:** `const kit = useKit() as Kit;` (re-renders on too much)
- **Good:** `const designs = useKitDesigns(); const tags = useKitTags(); ...` and add additional narrow hooks where needed.

### Principle 2: One source of truth per state domain

- **UI-ephemeral:** pointer positions, drag state → `useRef` / local state.
- **App state (per kit/design/type):** selection, hover, panel visibility, layout, transactions → XState state and/or Yjs store.
- **Domain data:** kits, types, designs, qualities, files → KitStore Yjs state with narrow hooks.

### Principle 3: Route-level lazy loading

- Only load the app module when the route is active.
- Only load the MDX file when the docs page is active.

---

## 3) File-by-file plan

### 3.1 `Sketchpad.tsx` (core store & hooks)

**Tasks**

1. **Remove eager app module loading**
   - Replace `SketchpadStore._loadModules()` with route-level `React.lazy` imports.
   - Keep only truly global, lightweight registrations in Sketchpad entry.
2. **Replace `./*.tsx` config scanning**
   - Introduce `Design.config.ts`, `Kit.config.ts`, `Type.config.ts`, etc.
   - `loadAppConfigs()` should glob only `./*.config.ts` (eager is ok because configs are tiny).
3. **Move Docs panel visibility into docs app state**
   - Delete `docsPanelVisibilityState` singleton and wire docs panel visibility through XState docs state + triadic hook.
4. **Fix misleading hook APIs**
   - `useType(deep?: boolean)` / `useQuality(deep?: boolean)` currently expose a `deep` param but don’t use it. Either implement deep or remove the param.
5. **Add guardrails**
   - Add an ESLint policy / custom lint check: disallow `useKit()` without selector in UI components.
   - Disallow new `exhaustive-deps` disables without an explicit comment tag.

**Acceptance criteria**

- Initial route load does not import every app module.
- Docs panel toggles work without global singleton state.

---

### 3.2 `Docs.tsx` (Docs app)

**Tasks**

1. **Make MDX loading lazy**
   - Replace `import.meta.glob(..., { eager: true })` with non-eager glob.
   - Build the docs registry from file paths (keys) without importing every page.
   - When a page is actually visited, dynamically import that page.
2. **Headings state**
   - Replace the timeout-based heading extraction with either:
     - a remark/rehype plugin that emits headings metadata during MDX compile, or
     - an IntersectionObserver-based runtime scanner that updates a scoped store, not a global singleton.
3. **Focus handler effects**
   - Remove `exhaustive-deps` disables by making `setOnFocusItem` stable and including it in deps.
4. **Immutability**
   - Fix any command code that mutates arrays from current state snapshots (always create new arrays).

**Acceptance criteria**

- Visiting `/docs` does not load all MDX page modules in JS.
- Heading/focus navigation works after navigating between docs pages without refresh.

---

### 3.3 `Design.tsx` (Design app)

**Tasks**

1. **Fix initialization**
   - Change `hasInitialized` logic to re-init when `(kitGuid, designGuid)` changes.
2. **Remove derived node state**
   - Convert `nodes` into:
     - `const baseNodes = useMemo(...)` (projection)
     - local state only for live drag deltas (or store in app state)
   - Prefer storing authoritative piece positions in the kit/design store and projecting into ReactFlow nodes.
3. **Fix focus effect dependencies**
   - Include `focusContext` in deps, or use a stable ref pattern.
4. **Remove window module cache pattern**
   - Avoid `window.__KIT_APP_MODULE_CACHE__` and rely on standard ESM caching.
   - If circular deps are the real reason, extract `KitSection` into a dependency-light module and import that instead.

**Acceptance criteria**

- Switching between designs in the same session produces correct INIT state.
- Dragging pieces never “snaps back” due to baseNodes resync.

---

### 3.4 `Kit.tsx` (Kit app)

**Tasks**

1. **Fix initialization**
   - Re-init XState sync when `kitGuid` changes (keyed ref).
2. **Stop syncing the entire Yjs state blob into XState**
   - Replace `KIT.SYNC` with targeted sync events per field (panelVisibility, selection, hover, transaction).
   - Subscribe only to the specific Yjs fields that affect each event.
3. **Replace `useKit()` overfetch**
   - Use narrow kit hooks per table/diagram needs (designs/types/files/folders/tags/etc).
   - Add missing hooks in `Sketchpad.tsx` if required.
4. **Diagram simulation**
   - Move force simulation off the critical render path:
     - Option A: Web Worker for simulation ticks.
     - Option B: incremental simulation across frames (`requestAnimationFrame`) with cancellation on dependency change.
   - Persist layout results so re-rendering doesn’t rerun 120 ticks unnecessarily.
5. **Remove `exhaustive-deps` disables**
   - Fix focus handler installation effect dependencies.

**Acceptance criteria**

- Editing an unrelated kit field does not rerender the entire Kit app view.
- Diagram layout does not block the main thread on every filter change.

---

### 3.5 `Type.tsx` / `Quality.tsx`

**Tasks**

1. **Fix initialization**
   - Same keyed-init fix as Design/Kit.
2. **Remove global module cache**
   - Remove `window.__KIT_APP_MODULE_CACHE__` pattern and rely on standard dynamic import.
3. **Audit subscriptions**
   - Ensure Type/Quality components subscribe only to what they use (already closer to this via `useKitFiles/useKitTags/...`).

**Acceptance criteria**

- Navigating between types/qualities re-inits correctly.
- No reliance on window globals for module caching.

---

### 3.6 `Home.tsx` / `Feedback.tsx`

**Tasks**

1. **Remove duplicate handlers**
   - Pick **one** mechanism per event (prefer `registerEventHandler`).
   - Remove redundant `registerRuntimeAction` entries once confirmed unused.
2. **Keep triadic hooks as the public API**
   - Make sure UI uses `[value, setValue, canSet]` hooks rather than directly poking machine context.

**Acceptance criteria**

- Each event updates state exactly once.
- No duplicated “same update” code paths.

---

## 4) Execution roadmap (safe, incremental)

### Phase 1 — correctness & stability (low risk, high value)

- [ ] Fix `hasInitialized` patterns (Design/Kit/Type/Quality).
- [ ] Fix effect dependencies and remove `exhaustive-deps` disables where possible (Docs/Kit/Design).
- [ ] Fix any state snapshot mutation (Docs commands).

### Phase 2 — subscription narrowing (medium risk)

- [ ] Replace `useKit()` calls with narrow hooks.
- [ ] Add missing narrow hooks (authors, folders, ports, concepts, etc).
- [ ] Refactor Yjs → XState sync to field-level events.

### Phase 3 — lazy loading (medium/high value)

- [ ] Replace `SketchpadStore._loadModules()` with route-level lazy imports.
- [ ] Replace app config loading with `*.config.ts` modules.
- [ ] Replace Docs eager MDX glob with lazy per-page loading.

### Phase 4 — cleanup & guardrails

- [ ] Remove `window.__KIT_APP_MODULE_CACHE__` patterns.
- [ ] Add lint checks for overfetch patterns and forbidden disables.

---

## 5) Verification checklist (no new tests required)

Run existing Playwright seeds and a small manual perf check:

- [ ] `playwright/seed.spec.ts` (root)
- [ ] `playwright/kit/seed.spec.ts`
- [ ] `playwright/kit/design/seed.spec.ts`
- [ ] `playwright/kit/type/seed.spec.ts`
- [ ] `playwright/kit/quality/seed.spec.ts`
- [ ] `playwright/docs/seed.spec.ts`

Manual checks:

- [ ] Navigate: Home → Kit → Design → Type → Quality → Docs and back (no reload).
- [ ] Ensure INIT runs correctly when switching entities.
- [ ] Use React DevTools Profiler: verify that unrelated updates don’t rerender whole pages.
- [ ] Verify initial JS chunks: Docs pages and non-visited apps are not loaded.

---

## 6) Concrete “first PR” (suggested)

**PR 1: correctness**

1. Keyed init refs (Design/Kit/Type/Quality).
2. Fix Design focus effect deps (include `focusContext`).
3. Remove state mutation in Docs commands.
4. Remove / justify `exhaustive-deps` disables in Docs focus handler installation.

**PR 2: overfetch**

1. Replace `useKit()` in Kit toolbar and diagram with narrow hooks.
2. Add missing `useKitX` hooks in `Sketchpad.tsx`.

**PR 3: lazy loading**

1. Replace `_loadModules` eager imports.
2. Convert docs MDX loader to lazy.

---

## Appendix: recommended helper utilities

### A. `useScopedInitKey(ref, key, init)`

A tiny utility hook to reduce copy/paste bugs:

- Tracks the last initialized key.
- Re-initializes when key changes.

### B. “Narrow hooks only” conventions

- Prefer `useKitTypes/useKitDesigns/useKitFiles/...` everywhere.
- If a new UI needs a new field, add a new narrow hook instead of reaching for `useKit()`.

# Changes

## Changes

## Log

## Summary
# Summary
