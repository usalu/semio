# Ticket

## Todos

# Previously

# Plan

Right now you kind of have **two state systems in parallel**:

- A big Yjs-backed `SketchpadStore` + `HomeStore`/`KitAppStore`/`DesignAppStore`/`TypeAppStore` etc. (commands like `compose.designApp.selectAll` that directly mutate Yjs).
- A new unified **`sketchpadMachine`** that already knows about `DesignAppState`, `TypeAppState`, etc. and is created in `createSketchpadActor` / `SketchpadScopeProvider`.

Plus some ad-hoc glue like `DesignAppSyncComponent` that “mirrors” Yjs → XState via `DESIGN.INIT` and `DESIGN.SYNC` events.

The plan below is about turning this into a **clean architecture**:

- **Yjs = collaborative data + shared app state (where needed).**
- **XState = single source of behavior & UI orchestration**, with a clear, uniform integration point to Yjs.
- **React = a read-only view** over the XState actor(s).

---

## 0. Target architecture (what we’re aiming for)

**Conceptual picture:**

```text
          ┌──────────────────────────┐
          │        React UI         │
          │  useSelector(actor, …)  │
          └──────────▲──────────────┘
                     │
                     │ XState events + state
                     │
          ┌──────────────────────────┐
          │    sketchpadMachine      │
          │  + per-kit / per-app    │
          │      child actors       │
          └──────────▲──────────────┘
                     │
                     │ Y_UPDATE / actions
                     │
          ┌──────────────────────────┐
          │      Yjs (yDoc, maps)    │
          └──────────────────────────┘
```

Key constraints:

- Only **one root actor**: `createSketchpadActor({ yDoc, ySketchpad, id })` (which you already have wired into `SketchpadScopeProvider`).
- This actor:
  - Subscribes to **Yjs** via a single `yjsSync` invoked service.
  - Spawns **child actors** for kits/types/designs/qualities/tutorial if needed.
  - Exposes a **pure event API** (`{ type: "DESIGN.HOVER_PIECE", … }`, etc.).

- **Legacy stores** (`SketchpadStore`, `DesignAppStore`, `KitAppStore`, …) shrink down to “Yjs façade” or are removed.

---

## 1. Phase 1 – Make XState the canonical UI state for the root

You already have selectors like `selectTheme`, `selectLayout`, `selectIsFullscreen`, etc., that read the root context from Yjs.

### 1.1. Tighten the root machine contract

In `sketchpadMachine`:

- Make sure context is the **single place** root UI state lives:

```ts
interface SketchpadContext {
 yDoc: Y.Doc;
 ySketchpad: Y.Map<any>;

 // Pure in-memory UI state
 theme: Theme;
 language: string;
 layout: Layout;
 mode: Mode;
 expertise: Expertise;
 isFullscreen: boolean;
 panelSizes: PanelSizes;
 navigation: string;
 navigationHistory: string[];
 navigationHistoryIndex: number;

 // Per-app maps
 homeApp: HomeAppState;
 kitApps: Record<Guid, KitAppState>;
 typeApps: Record<string, TypeAppState>;
 designApps: Record<string, DesignAppState>;
 qualityApps: Record<string, QualityAppState>;
}
```

(You mostly have this already in `SketchpadContext`, just ensure **root UI fields are actually kept there**, not only in Yjs.)

- For every root “command” you currently execute on `SketchpadStore` (e.g. `compose.sketchpad.setTheme`), add a **typed event**:

```ts
type SketchpadEvent = { type: "SET_THEME"; theme: Theme } | { type: "SET_LAYOUT"; layout: Layout } | { type: "SET_EXPERTISE"; expertise: Expertise } | { type: "NAVIGATE"; path: string } | { type: "NAVIGATE_BACK" } | { type: "NAVIGATE_FORWARD" };
// ...
```

…and implement inline actions that both:

1. Update `context.*`.
2. Persist to Yjs (`ySketchpad.set("theme", …)` etc.), like you already do in actions such as `navigateImpl`.

### 1.2. Flip your React hooks to XState

Right now hooks like `useTheme`, `useLayout`, `useMode`, etc. read from `useSketchpadStore` / Yjs.

Create hooks that read directly from the actor:

```ts
import { useSketchpadActor } from "./Sketchpad";
import { useSelector } from "@xstate/react";

export function useXTheme(): Theme {
 const actor = useSketchpadActor();
 return useSelector(actor, (s) => s.context.theme);
}
```

Then gradually migrate existing hooks to call these, e.g.:

```ts
export function useTheme(): Theme {
 // temporary bridge
 return useXTheme();
}
```

So the **UI is now coupled to the state machine**, not to Yjs / `SketchpadStore`.

---

## 2. Phase 2 – Normalize per-app state into the machine

Right now each app has:

- A Yjs-backed store (`HomeStore`, `KitAppStore`, `DesignAppStore`, `TypeAppStore`).
- Commands like `compose.designApp.hoverPiece`, `compose.kitApp.selectDesign`, etc., that directly mutate Yjs and then you mirror that into XState in a somewhat bespoke way (e.g. `DesignAppSyncComponent`).

You already have _parallel_ lean `DesignAppState`, `TypeAppState`, etc. inside `machines.ts` with default creators like `createDefaultDesignAppState()`, `createDefaultTypeAppState()`.

### 2.1. Treat those as the **canonical** app states

For each app, define machine events:

```ts
type DesignAppEvent =
 | { type: "DESIGN.INIT"; kitGuid: Guid; designGuid: Guid; state: DesignAppState }
 | { type: "DESIGN.SYNC"; kitGuid: Guid; designGuid: Guid; state: DesignAppState }
 | { type: "DESIGN.HOVER_PIECE"; kitGuid: Guid; designGuid: Guid; piece: Guid }
 | { type: "DESIGN.CLEAR_HOVER"; kitGuid: Guid; designGuid: Guid }
 | { type: "DESIGN.SELECT_ALL"; kitGuid: Guid; designGuid: Guid }
 | { type: "DESIGN.DELETE_SELECTED"; kitGuid: Guid; designGuid: Guid };
// ...
```

And keep all `DesignAppState` in `context.designApps[key]` (where key = `${kitGuid}:${designGuid}`), with helper functions already present in `machines.ts` (you had that idea in the context type).

### 2.2. Move the “command logic” into XState

Most of your commands in `Design.tsx` return a `diff` + optional `kitDiff`.

Example:

```ts
"compose.designApp.selectAll": ctx => ({
  diff: {
    selection: {
      pieces: { added: allPieceGuids },
      connections: { added: allConnectionGuids },
    },
  },
});
```

The plan:

- Keep the **domain mutations** (`kitDiff` applying to Yjs Kit data) inside your current **KitStore/DesignStore** for now – that’s fine, that’s domain-level.
- Move the **UI-only diff** (`DesignAppDiff`) handling into actions on `sketchpadMachine`:

```ts
actions: {
  designApplyDiff: assign(({ context, event }) => {
    if (event.type !== "DESIGN.APPLY_DIFF") return {};
    const key = `${event.kitGuid}:${event.designGuid}`;
    const current = context.designApps[key] ?? createDefaultDesignAppState();
    const next = {
      ...current,
      // merge selection / hover / etc.
    };
    return { designApps: { ...context.designApps, [key]: next } };
  }),
}
```

Then wire command handling like:

```ts
on: {
  "DESIGN.SELECT_ALL": {
    actions: ["applySelectAllDomain", "designApplyDiff"],
  },
}
```

Where:

- `applySelectAllDomain` calls the **existing Yjs-based command** and returns the `DesignAppDiff` + `KitDiff`.
- `designApplyDiff` uses that `diff` to mutate the machine’s `context.designApps[...]`.

That can be factored into a helper that bridges current command system with the state machine, so you don’t rewrite everything at once.

### 2.3. Replace `DesignAppSyncComponent` style ad-hoc syncing

Today, `DesignAppSyncComponent` does:

1. On mount: read Yjs `DesignAppStore.snapshot()` and send `DESIGN.INIT` to the actor.
2. Subscribe with `useSyncDeep` and send `DESIGN.SYNC` on every change.

Once `sketchpadMachine` owns `DesignAppState`:

- **Remove** that sync component.
- Instead, have a single `yjsSync` invoked service on the root machine (you already mention it in the comment) that watches relevant Yjs maps (design app Y map if you still keep some bits there) and emits `Y_UPDATE` or app-specific events.

That way **all Yjs → XState syncing is centralized** inside the machine, not spread across React components.

---

## 3. Phase 3 – Redesign the public “commands” API around XState events

Right now most of your UI helpers call `store.execute("compose.sketchpad.*", ...)` etc.

Introduce a clean, typed XState-centric command layer:

```ts
export function useSketchpadCommands() {
 const actor = useSketchpadActor();
 const navigate = useNavigate();

 return {
  setTheme: (origin: string, theme: Theme) => actor.send({ type: "SET_THEME", theme }),
  navigateToKit: (kit: Guid, search?: string) => {
   const path = `/kits/${kit}${search ? `?${search}` : ""}`;
   actor.send({ type: "NAVIGATE", path });
   navigate(path);
  },
  // ...
 };
}
```

Then gradually:

- Replace `store.execute("compose.sketchpad.X", ...)` calls with `actor.send(...)`.
- Provide a **transition adapter** so legacy code can still call `store.execute("compose.*")`, but internally it just sends the right events.

Eventually you can remove the string-based command registry on `AppStore` and keep pure TS event types instead.

---

## 4. Phase 4 – Thin or remove the legacy Yjs stores

Once React is reading from XState and XState is the only thing mutating UI state, the remaining job of the Yjs stores can be:

- Domain queries: `kitStore.design(guid)`, graph flattening, etc.
- Domain mutations: applying `KitDiff`, `DesignDiff`, etc. to Yjs structures. (Which XState actions can call.)

You can then:

1. **Stop using** `useSync`, `useSyncDeep`, `useSyncField`, etc. from React. All those hooks become internal helpers for the machine or are deleted.

2. Replace app-store hooks (`useDesignAppStore`, `useKitAppStore`, etc.) with **pure XState selectors**:

   ```ts
   export function useDesignAppState(kitGuid: Guid, designGuid: Guid) {
    const actor = useSketchpadActor();
    const key = `${kitGuid}:${designGuid}`;
    return useSelector(actor, (s) => s.context.designApps[key] ?? createDefaultDesignAppState());
   }
   ```

3. Eventually collapse the legacy store classes into a **domain service layer** (no internal cache, no command registry, just Yjs operations).

---

## 5. What you gain (why this is worth the pain)

- **Single event log**: every meaningful change is an XState event → easy to inspect, log, replay, test.
- **Deterministic UI state**: no more ad-hoc Yjs observers in React land; everything flows through the machine.
- **Local vs collaborative state separated**:
  - Yjs: kits, designs, presence, any state that must sync between peers.
  - XState: routing, panel visibility, “is modal open”, local selection if desired.

- **Gradual migration**: you don’t have to rewrite everything:
  - Start at the root (`SET_THEME`, navigation).
  - Then move one app at a time (Home → Kit → Design → Type).

---

If you’d like, next step could be: pick **one app** (e.g. Design) and I can sketch the concrete refactor diff: new event types for the machine, how to adapt `DesignAppCommandResult`, and how to rewrite one of the existing React hooks (`useDesignAppSelection`) to use XState instead of `DesignAppStore`.

Here’s a concrete, Design-app-specific plan that follows the architecture we talked about, using the code you actually have.

---

## 0. Where Design app is today (so we know what we’re changing)

Right now Design app state lives in three places:

1. **Yjs-backed `DesignAppStore`**
   - Holds `selection`, `hover`, `fullscreenWindow`, `panelVisibility`, `camera`, `diagramCenter`, `diagramScale`, `focusedPieceGuid`, model tags, presence, transactions, etc.
   - Exposes `execute("compose.designApp.*")` which:
     - Builds a `DesignAppCommandContext`.
     - Runs a command from `designAppCommands`.
     - Applies `result.diff` to its own Yjs state.
     - Applies `result.kitDiff` to the `KitStore` Yjs state.
     - Records edits/transactions.

2. **`sketchpadMachine` context**
   - Has `designApps: Record<string, DesignAppState>` with `designInit`/`designSync` actions to update it from Yjs.
   - Already has some Design events & actions: `DESIGN.SET_ACTIVE_TOOL`, `DESIGN.SET_FULLSCREEN`, `DESIGN.TOGGLE_PANEL`, `DESIGN.SET_SELECTION`, `DESIGN.CLEAR_SELECTION`, `DESIGN.SET_HOVER`, `DESIGN.CLEAR_HOVER`, `DESIGN.FOCUS_PIECE`, etc.

3. **React hooks reading from Yjs**
   - `useDesignAppSelection`, `useDesignAppActiveTool`, `useDesignAppHover`, etc. all use `useDesignAppStore` + `useSyncField` / `useSyncDeep` to read from `DesignAppStore` directly.

Plus a bridge:

- **`DesignAppSyncComponent` / `useDesignAppYjsToXStateSyncInternal`**:
  On mount: takes the Yjs snapshot and sends `DESIGN.INIT`.
  Then: uses `useSyncDeep` to mirror any Yjs changes back into XState with `DESIGN.SYNC`.

That’s the “adhoc integration” you mentioned.

The goal: **flip this** so that:

- XState is the **canonical Design app state**.
- Yjs holds only what’s truly collaborative / persisted.
- React views only talk to XState.

---

## 1. Decide what DesignAppState belongs where

Go through `DesignAppState` and classify each field:

From `DesignAppStore.buildSnapshot()` you have:

- **UI & interaction / local-ish:**
  - `fullscreenWindow`
  - `panelVisibility`
  - `activeTool`
  - `selection`
  - `hover`
  - `focusedPieceGuid`
  - `windowLayout`

- **View-related but possibly collaborative:**
  - `camera`
  - `diagramCenter`
  - `diagramScale`

- **Collaborative/infra:**
  - `presence`
  - `others`
  - `currentTransactionStackLength`
  - `selectedModelTags`

Plan:

1. Make **XState** the **source of truth for all UI-ish fields**:
   - `fullscreenWindow`, `panelVisibility`, `activeTool`, `selection`, `hover`, `focusedPieceGuid`, `windowLayout`.

2. Decide per field if it should still be persisted to Yjs:
   - E.g. `selection` & `hover`: probably best as **local per client** → **don’t persist to Yjs anymore**.
   - `camera`, `diagramCenter`, `diagramScale`: depends whether you want “shared view”. If not, treat as local too.

3. Keep **transactions & kit diffs** primarily in the Kit/Design domain layer; XState only needs light flags (e.g. `isTransactionActive`, `currentTransactionStackLength`).

Write this down in a small table in a doc so you have a checklist when implementing.

---

## 2. Expand the sketchpad machine’s Design events & actions

You already have `DESIGN.INIT`, `DESIGN.SYNC`, `DESIGN.SET_ACTIVE_TOOL`, `DESIGN.SET_FULLSCREEN`, `DESIGN.TOGGLE_PANEL`, `DESIGN.SET_SELECTION`, `DESIGN.CLEAR_SELECTION`, `DESIGN.SET_HOVER`, `DESIGN.CLEAR_HOVER`, `DESIGN.FOCUS_PIECE`, etc. implemented as context updates.

### 2.1. Make DesignAppState in machines.ts complete

In `machines.ts`, ensure `DesignAppState` there matches what you need (not the Yjs version; the slim XState version). Something like:

```ts
export interface DesignAppState {
 panelVisibility: PanelVisibility;
 selection?: DesignAppSelection;
 hover?: DesignAppHover;
 focusedPiece?: Guid;
 selectedModelTags: Record<Guid, string[]>;
 diagramCenter?: { x: number; y: number };
 diagramScale?: number;
 camera?: Camera;
 activeTool: ToolKind;
 fullscreenWindow: DesignAppFullscreenWindow;
 windowLayout?: any;
}
```

And confirm `createDefaultDesignAppState()` sets sane defaults. This is already partially there, just verify & clean it up.

### 2.2. Design key helper

You already use `const key = \`${kitGuid}:${designGuid}`;` in actions. Wrap that in a helper:

```ts
const designKey = (kitGuid: Guid, designGuid: Guid) => `${kitGuid}:${designGuid}`;
```

So all actions reuse the same key logic.

### 2.3. Extend Design actions to cover full state

Add / verify actions for:

- `DESIGN.SET_CAMERA`
- `DESIGN.SET_DIAGRAM_CENTER`
- `DESIGN.SET_DIAGRAM_SCALE`
- `DESIGN.SET_WINDOW_LAYOUT`
- `DESIGN.SET_SELECTED_MODEL_TAGS` (or ADD/REMOVE tag events as you prefer)

Following the existing pattern:

```ts
designSetCamera: assign(({ context, event }) => {
  if (event.type !== "DESIGN.SET_CAMERA") return {};
  const key = designKey(event.kitGuid, event.designGuid);
  const app = context.designApps[key] || createDefaultDesignAppState();
  return { designApps: { ...context.designApps, [key]: { ...app, camera: event.camera } } };
}),
```

Do the same for diagram center / scale / layout / selected model tags.

This ensures **all design UI state mutations are representable in the state machine**.

---

## 3. Introduce a “Design command” action in XState

Today, Design commands are run via `DesignAppStore.execute("compose.designApp.*")`.

We want:

- A single XState event family:
  - `DESIGN.EXECUTE_COMMAND` (or more specific events).

- Actions that:
  1. Call your existing command implementation (so no domain rewrite yet).
  2. Use the result’s `DesignAppDiff` to update `context.designApps[...]`.
  3. Use the `kitDiff` to mutate Yjs via existing `applyDiff` or Kit store.

### 3.1. Extract a pure “command runner” function

In `Design.tsx`, near `designAppCommands`, create a helper that **does NOT know about React or hooks**:

```ts
export function runDesignAppCommand(sketchpadStore: SketchpadStore, kitGuid: Guid, designGuid: Guid, command: string, origin: string, ...args: any[]): DesignAppCommandResult {
 const designAppStore = sketchpadStore.designApp(kitGuid, designGuid) as DesignAppStore;
 // Reuse its executeCommand, but we’ll intercept the result
 return designAppStore.execute<DesignAppCommandResult>(command, origin, ...args);
}
```

(If you prefer to avoid calling `execute` directly for architectural reasons, you can also call `designAppCommands[command]` with a crafted `DesignAppCommandContext` instead, but reusing `execute` is the smallest diff for now.)

### 3.2. Add a generic DESIGN.EXECUTE_CMD event

In `machines.ts`:

```ts
type SketchpadEvent = { type: "DESIGN.EXECUTE_CMD"; kitGuid: Guid; designGuid: Guid; command: string; origin: string; args: any[] };
// ... existing events ...
```

Add an action:

```ts
designExecuteCmd: ({ context, event }) => {
  if (event.type !== "DESIGN.EXECUTE_CMD") return;

  const { yDoc } = context;
  const sketchpadStore = /* a reference or injected service to SketchpadStore */;

  const result = runDesignAppCommand(
    sketchpadStore,
    event.kitGuid,
    event.designGuid,
    event.command,
    event.origin,
    ...event.args,
  );

  // 1) Apply DesignApp UI diff to machine context
  if (result.diff) {
    // Use a small helper applyDesignAppDiff(context, key, result.diff) that merges
    // selection/hover/focusedPiece/... into context.designApps[key]
  }

  // 2) kitDiff is already applied inside DesignAppStore.execute(), so for now
  //    we may not need to do anything here. If you later move kitDiff out of the store,
  //    this is where you’d call applyDiff(yDoc, ySketchpad, result.kitDiff);
},
```

You might not want to call `runDesignAppCommand` from inside the machine because that couples the machine to the `SketchpadStore` class. A cleaner variant:

- Treat Design commands as **domain services** living outside the machine.
- Have UI code call a `useDesignAppCommands()` hook which:
  - `actor.send({ type: "DESIGN.EXECUTE_CMD", ... })`
  - AND separately calls the SketchpadStore command for domain changes.

- Then wire `designExecuteCmd` to **only** apply the `DesignAppDiff` (UI state), not the `kitDiff` (domain, already handled).

Either route is fine; the plan is:

> **Short term**: reuse existing commands to mutate Yjs + rely on `DesignAppDiff` to keep XState context in sync.

---

## 4. Migrate React hooks from Yjs → XState (for Design app only)

Right now:

```ts
export function useDesignAppSelection(id?: DesignAppId): DesignAppSelection {
 const store = useDesignAppStore(identitySelector, id);
 if (!store) return EMPTY_SELECTION;
 return useSyncField(store as DesignAppStore, "selection", selectSelection);
}
```

Plan:

### 4.1. Add Design selectors in machines.ts

Similar to Type selectors you already have, add:

```ts
export const createDesignAppSelector = (kitGuid: Guid, designGuid: Guid) => {
 const key = `${kitGuid}:${designGuid}`;
 return (state: { context: SketchpadContext }) => {
  const app = state.context.designApps[key];
  return app ?? createDefaultDesignAppState();
 };
};

export const createDesignSelectionSelector = (kitGuid: Guid, designGuid: Guid) => {
 const key = `${kitGuid}:${designGuid}`;
 return (state: { context: SketchpadContext }) => state.context.designApps[key]?.selection ?? {};
};

export const createDesignActiveToolSelector = (kitGuid: Guid, designGuid: Guid) => {
 const key = `${kitGuid}:${designGuid}`;
 return (state: { context: SketchpadContext }) => state.context.designApps[key]?.activeTool ?? ToolKind.SELECTION_NORMAL;
};

// same for hover, fullscreenWindow, panelVisibility, camera, etc.
```

### 4.2. Wire new hooks via `useSketchpadActor` + selectors

Create new, XState-based versions of the hooks (either in `Design.tsx` or a separate hooks file):

```ts
import { useSelector } from "@xstate/react";
import { useSketchpadActor } from "./Sketchpad";
import { createDesignSelectionSelector, createDesignActiveToolSelector } from "./machines";

function resolveDesignIds(id?: DesignAppId) {
 const kitScope = useKitScope();
 const designScope = useDesignScope();
 return {
  kitGuid: kitScope?.guid ?? id?.kit,
  designGuid: designScope?.guid ?? id?.design,
 };
}

export function useDesignAppSelectionXState(id?: DesignAppId): DesignAppSelection {
 const actor = useSketchpadActor();
 const { kitGuid, designGuid } = resolveDesignIds(id);
 return useSelector(actor, createDesignSelectionSelector(kitGuid!, designGuid!));
}

export function useDesignAppActiveToolXState(id?: DesignAppId): ToolKind {
 const actor = useSketchpadActor();
 const { kitGuid, designGuid } = resolveDesignIds(id);
 return useSelector(actor, createDesignActiveToolSelector(kitGuid!, designGuid!));
}
```

Then **flip** the old hooks to delegate to the new ones:

```ts
export function useDesignAppSelection(id?: DesignAppId): DesignAppSelection {
 return useDesignAppSelectionXState(id);
}

export function useDesignAppActiveTool(id?: DesignAppId): ToolKind {
 return useDesignAppActiveToolXState(id);
}
```

Repeat for:

- `useDesignAppFullscreen`
- `useDesignAppHover`
- `useDesignAppCamera`
- `useDesignAppDiagramCenter`
- `useDesignAppDiagramScale`
- `useDesignAppFocusedPieceGuid`
- `useDesignAppSelectedModelTags`
- `useDesignAppPanelVisibility`

Once that’s done, **React is reading only from XState** for Design app UI state.

---

## 5. Replace Yjs-based Design commands with XState events

Find all places where you call Design commands via the store, e.g.:

```ts
store.execute("compose.designApp.hoverPiece", origin, pieceGuid);
store.change({ selection: { pieces: { added: [guid] } } });
// etc
```

Replace them with:

1. Public **command hook** that wraps XState events:

   ```ts
   export function useDesignAppCommands(kitGuid: Guid, designGuid: Guid) {
    const actor = useSketchpadActor();

    return {
     setActiveTool: (origin: string, tool: ToolKind) => actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool }),
     hoverPiece: (origin: string, pieceGuid: Guid) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { pieces: [pieceGuid] } }),
     clearHover: (origin: string) => actor.send({ type: "DESIGN.CLEAR_HOVER", kitGuid, designGuid }),
     setSelection: (origin: string, selection: DesignAppSelection) => actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection }),
     // For domain edits:
     executeDomainCommand: (origin: string, command: string, ...args: any[]) => {
      // either actor.send({ type: "DESIGN.EXECUTE_CMD", ... }) and/or call runDesignAppCommand
     },
    };
   }
   ```

2. In components (canvas, toolbars, etc.), **stop using `DesignAppStore`** and use this hook instead.

This is where you actually start **removing the “parallel state” feel in day-to-day code**: everything goes through `actor.send`.

---

## 6. Remove `DesignAppSyncComponent` and tighten Yjs usage

Once:

- All Design UI reads are from XState.
- All writes go through `actor.send` and the machine updates `context.designApps[...]`.

Then:

1. **Stop mirroring Yjs → XState** for Design:
   Remove or no-op `DesignAppSyncComponent` & `useDesignAppYjsToXStateSyncInternal`.
   - At this point, the machine’s Design app context is responsible for the state; Yjs is only used for underlying kit/design data and any fields you decided to persist.

2. For any fields you still want to persist in Yjs (e.g. `camera` if shared):
   - Update the corresponding XState actions to write into Yjs inside a `yDoc.transact(...)`, the same way root actions like `setLayout` do.

   Example:

   ```ts
   designSetCamera: ({ context, event }) => {
     if (event.type !== "DESIGN.SET_CAMERA") return;
     const { yDoc } = context;
     const yDesignApp = /* locate Y.Map for this Design app */;
     yDoc.transact(() => {
       yDesignApp.set("camera", event.camera);
     });
   },
   ```

   (You already have the plumbing in `SketchpadStore` to get `yDesignApps`; you can expose helpers or pass a service into the machine when constructing it.)

3. For purely local fields (`hover`, `selection`, etc.) don’t write them to Yjs anymore; just keep them in XState.

---

## 7. Gradually thin / remove `DesignAppStore`

After the above, `DesignAppStore` should only be needed for:

- Running Design **domain commands** that affect kits/designs (until you move those to a pure service).
- Handling Yjs transactions / undo/redo.

Next steps:

1. Extract the **transaction logic** (`startTransaction`, `finalizeTransaction`, `abortTransaction`, `undo`, `redo`, transaction stacks) into a standalone domain service that works on `kitDiff` + Yjs.
2. Let XState track only lightweight flags:
   - `isTransactionActive`
   - `currentTransactionStackLength`

3. Refactor `TransactionPiecesProvider` and related hooks to read from that new transaction service / Yjs layer directly, not from `DesignAppStore`.
4. Once no React code calls `useDesignAppStore` anymore, you can:
   - Stop initializing `DesignAppStore` (remove `initializeDesignAppStore()` registration).
   - Eventually delete the `DesignAppStore` class, leaving only Yjs helpers + domain services.

---

## 8. Suggested implementation order (so it’s not chaotic)

To keep it sane:

1. **Week 1 – Model + selectors**
   - Finalize `DesignAppState` + `createDefaultDesignAppState` in `machines.ts`.
   - Implement Design selectors (`createDesignAppSelector`, `createDesignSelectionSelector`, etc.).
   - Add missing Design actions (`DESIGN.SET_CAMERA`, etc.).

2. **Week 2 – Read path migration**
   - Implement XState-based Design hooks (`useDesignAppSelectionXState`, `useDesignAppActiveToolXState`, …).
   - Flip existing hooks to delegate to the XState ones.
   - Fix any type issues, ensure UI still works.

3. **Week 3 – Write path migration**
   - Introduce `useDesignAppCommands` that only sends XState events.
   - Replace `DesignAppStore` calls in Design canvas/toolbars with these commands.
   - Add any extra XState design actions you need for behaviors previously done with `store.change(...)`.

4. **Week 4 – Remove Yjs→XState mirroring**
   - Remove `DesignAppSyncComponent` / `useDesignAppYjsToXStateSyncInternal`.
   - Persist only the fields you want in Yjs from XState actions.

5. **Week 5+ – Transactions & cleanup**
   - Extract transaction logic to a service.
   - Rewrite `TransactionPiecesProvider` & diff hooks to not rely on `DesignAppStore`.
   - Remove `DesignAppStore` factory registration and the class once unused.

---

Cool, let’s do the Type app next. I’ll mirror the Design plan, but tailored to what you actually have in `Type.tsx` and `machines.ts`.

---

## 0. Where the Type app is _today_

From your code:

- **Yjs-backed `TypeAppStore`** in `Type.tsx`
  - Holds `fullscreenWindow`, `panelVisibility`, `activeTool`, `selection`, `hover`, `presence`, `others`, `camera`, `focusedConnectorGuid`, `selectedModelGuid`, `selectedModelTags`, `windowLayout`, etc.
  - Initializes defaults in the constructor (toolbar always visible, clearing corrupt `windowLayout`, etc.).
  - Runs commands (`compose.typeApp.*`) via `registerCommand` and applies `TypeDiff` + `TypeAppDiff`.

- **React hooks** are all Yjs-based:
  - `useTypeApp`, `useTypeAppSelection`, `useTypeAppPanelVisibility`, `useTypeAppOthers`, `useTypeAppCamera`, `useTypeAppFocusedConnectorGuid`, `useTypeAppHover`, `useTypeAppSelectedModelGuid`, `useTypeAppSelectedModelTags`, etc.
  - These use `useTypeAppStore` + `useSyncDeep` / `useSyncField`.

- **Command hook**:
  - `useTypeAppCommands` calls `store.execute("compose.typeApp.*")` or `store.change(...)` directly.

In `machines.ts` you already have:

- A **simplified `TypeAppState`** (panelVisibility, selection, hover, focusedConnector, selectedModelTags, camera).
- `context.typeApps: Record<string, TypeAppState>` keyed by `${kitGuid}:${typeGuid}`.
- Events: `TYPE.INIT`, `TYPE.SYNC`, `TYPE.TOGGLE_PANEL`, `TYPE.FOCUS_CONNECTOR`, `TYPE.SELECT_MODEL_TAG`, `TYPE.DESELECT_MODEL_TAG`, `TYPE.SET_CAMERA`.
- Actions: `typeInit`, `typeSync`, `typeTogglePanel`, `typeFocusPort`, `typeSelectModelTag`, `typeDeselectModelTag`, `typeSetCamera`.

But **nothing in `Type.tsx` actually dispatches `TYPE.*` events yet**, and your hooks still talk straight to the Yjs store.

Goal for Type app, same as Design:

- **XState is the canonical UI state** for Type app.
- **Yjs stores are only for collaborative/persisted domain data** and transactions.
- React components read the Type UI state entirely via the XState actor.

---

## 1. Align the `TypeAppState` model

Right now you effectively have two TypeAppState definitions:

- In `Type.tsx`: richer, includes presence, activeTool, fullscreenWindow, selected model, tags as strings, windowLayout, etc.
- In `machines.ts`: smaller, tags as `Guid[]`, no activeTool/fullscreenWindow/presence/selectedModelGuid/windowLayout.

First step: decide the **single canonical shape** of `TypeAppState` you want the machine to hold.

### 1.1. Proposed unified `TypeAppState` (machine version)

Take the richer one from `Type.tsx`, but maybe trim presence if you prefer a separate awareness layer:

```ts
// machines.ts
export interface TypeAppState {
 fullscreenWindow: TypeAppFullscreenWindow;
 panelVisibility: PanelVisibility;
 activeTool: ToolKind;

 selection?: TypeAppSelection;
 hover?: TypeAppHover;

 // Could keep presence here or move to a separate 'awareness' slice
 presence?: TypeAppPresence;
 others: TypeAppPresenceOther[];

 camera?: Camera;
 focusedConnectorGuid?: Guid;
 selectedModelGuid?: Guid;
 selectedModelTags?: string[];

 windowLayout?: any;
}
```

Then update `createDefaultTypeAppState()` to match this shape (with toolbar visible, sensible defaults for `activeTool`, etc., same semantics as your `TypeAppStore` constructor).

### 1.2. Triage fields: local vs collaborative

Go over each field:

- **Local-ish / per-client UI:**
  - `fullscreenWindow`
  - `panelVisibility` (maybe – could be local)
  - `activeTool`
  - `selection`
  - `hover`
  - `windowLayout` (Type comment says one scene window; probably per-user)

- **Possibly shared:**
  - `camera` (do you want shared camera in Type app?)
  - `selectedModelGuid` / `selectedModelTags`

- **Collab infra:**
  - `presence`, `others`

Decision:

- Everything you want to be **local UI** should exist _only_ in `TypeAppState` (machine context) and not in Yjs anymore.
- Anything you want synchronized between peers should still be written/read from Yjs, but via XState actions, not React + store hooks.

Write this decision down once so you can refer back during implementation.

---

## 2. Expand Type-related events & actions in the machine

You already have a basic set of Type events (`TYPE.INIT`, `TYPE.SYNC`, `TYPE.TOGGLE_PANEL`, `TYPE.FOCUS_CONNECTOR`, `TYPE.SELECT_MODEL_TAG`, `TYPE.DESELECT_MODEL_TAG`, `TYPE.SET_CAMERA`) and the corresponding actions.

Now extend this to cover all the pieces you actually use in the Type UI.

### 2.1. Add missing event types

In `SketchpadEvent` (machines.ts) add:

```ts
// Type app events (scoped by kitGuid:typeGuid)
| { type: "TYPE.SET_ACTIVE_TOOL"; kitGuid: Guid; typeGuid: Guid; tool: ToolKind }
| { type: "TYPE.SET_SELECTION"; kitGuid: Guid; typeGuid: Guid; selection: TypeAppSelection }
| { type: "TYPE.CLEAR_SELECTION"; kitGuid: Guid; typeGuid: Guid }
| { type: "TYPE.SET_HOVER"; kitGuid: Guid; typeGuid: Guid; hover: TypeAppHover }
| { type: "TYPE.CLEAR_HOVER"; kitGuid: Guid; typeGuid: Guid }
| { type: "TYPE.SET_FULLSCREEN"; kitGuid: Guid; typeGuid: Guid; window: TypeAppFullscreenWindow }
| { type: "TYPE.SET_SELECTED_MODEL"; kitGuid: Guid; typeGuid: Guid; modelGuid?: Guid }
| { type: "TYPE.SET_SELECTED_MODEL_TAGS"; kitGuid: Guid; typeGuid: Guid; tags: string[] }
| { type: "TYPE.SET_WINDOW_LAYOUT"; kitGuid: Guid; typeGuid: Guid; layout: any }
```

You can refine names later, but idea is: every UI mutation you currently do with `store.change(...)` becomes an explicit event.

### 2.2. Implement the actions

Following the pattern you already have for `typeTogglePanel`, add:

```ts
const typeKey = (kitGuid: Guid, typeGuid: Guid) => `${kitGuid}:${typeGuid}`;

typeSetActiveTool: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_ACTIVE_TOOL") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, activeTool: event.tool } } };
}),

typeSetSelection: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_SELECTION") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, selection: event.selection } } };
}),

typeClearSelection: assign(({ context, event }) => {
  if (event.type !== "TYPE.CLEAR_SELECTION") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, selection: {} } } };
}),

typeSetHover: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_HOVER") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, hover: event.hover } } };
}),

typeClearHover: assign(({ context, event }) => {
  if (event.type !== "TYPE.CLEAR_HOVER") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, hover: {} } } };
}),

typeSetFullscreen: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_FULLSCREEN") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, fullscreenWindow: event.window } } };
}),

typeSetSelectedModel: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_SELECTED_MODEL") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelGuid: event.modelGuid } } };
}),

typeSetSelectedModelTags: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_SELECTED_MODEL_TAGS") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: event.tags } } };
}),

typeSetWindowLayout: assign(({ context, event }) => {
  if (event.type !== "TYPE.SET_WINDOW_LAYOUT") return {};
  const key = typeKey(event.kitGuid, event.typeGuid);
  const app = context.typeApps[key] || createDefaultTypeAppState();
  return { typeApps: { ...context.typeApps, [key]: { ...app, windowLayout: event.layout } } };
}),
```

This gives you a **complete vocabulary** to express Type UI changes as events.

---

## 3. Add XState selectors for Type app

Imitate what you did for Design:

```ts
// machines.ts
export const createTypeAppSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key] ?? createDefaultTypeAppState();
};

export const createTypeSelectionSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.selection ?? {};
};

export const createTypePanelVisibilitySelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.panelVisibility ?? DEFAULT_PANEL_VISIBILITY;
};

export const createTypeHoverSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.hover;

export const createTypeCameraSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.camera;

export const createTypeActiveToolSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.activeTool ?? ToolKind.SELECTION_NORMAL;

export const createTypeSelectedModelGuidSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.selectedModelGuid;

export const createTypeSelectedModelTagsSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) =>
    state.context.typeApps[key]?.selectedModelTags ?? [];
};
```

You can add presence-related selectors similarly if you keep presence in this state.

---

## 4. XState-based Type hooks, then flip the old ones

Now build XState-powered hooks in `Type.tsx` (or a new `Type.xstate.ts` file):

```ts
import { useSelector } from "@xstate/react";
import { useSketchpadActor } from "./Sketchpad";
import {
 createTypeAppSelector,
 createTypeSelectionSelector,
 createTypePanelVisibilitySelector,
 createTypeHoverSelector,
 createTypeCameraSelector,
 createTypeActiveToolSelector,
 createTypeSelectedModelGuidSelector,
 createTypeSelectedModelTagsSelector,
} from "./machines";

function resolveTypeIds(id?: TypeAppId) {
 const kitScope = useKitScope();
 const typeScope = useTypeScope();
 return {
  kitGuid: kitScope?.guid ?? id?.kit,
  typeGuid: typeScope?.guid ?? id?.type,
 };
}

export function useTypeAppXState<T = TypeAppState>(selector?: (state: TypeAppState) => T, id?: TypeAppId): T | TypeAppState | null {
 const actor = useSketchpadActor();
 const { kitGuid, typeGuid } = resolveTypeIds(id);
 if (!kitGuid || !typeGuid) return null;
 const baseSelector = createTypeAppSelector(kitGuid, typeGuid);
 return useSelector(actor, (s) => (selector ? selector(baseSelector(s)) : baseSelector(s))) as T;
}

export function useTypeAppSelectionXState(id?: TypeAppId): TypeAppSelection {
 const actor = useSketchpadActor();
 const { kitGuid, typeGuid } = resolveTypeIds(id);
 if (!kitGuid || !typeGuid) return {};
 return useSelector(actor, createTypeSelectionSelector(kitGuid, typeGuid));
}

// repeat for panel visibility, hover, camera, focusedConnectorGuid, activeTool, selectedModelGuid, tags
```

Then **flip your existing hooks** to use these instead of Yjs:

```ts
// Before:
export function useTypeAppSelection(id?: TypeAppId): TypeAppSelection {
 const store = useTypeAppStore(identitySelector, id);
 if (!store) return EMPTY_TYPE_SELECTION;
 return useSyncField<TypeAppState, TypeAppSelection>(store as TypeAppStore, "selection", selectTypeAppSelection);
}

// After:
export function useTypeAppSelection(id?: TypeAppId): TypeAppSelection {
 return useTypeAppSelectionXState(id);
}
```

Same for:

- `useTypeApp`
- `useTypeAppPanelVisibility`
- `useTypeAppOthers` (if you keep presence in TypeAppState, or add a separate presence selector)
- `useTypeAppCamera`
- `useTypeAppFocusedConnectorGuid`
- `useTypeAppHover`
- `useTypeAppSelectedModelGuid`
- `useTypeAppSelectedModelTags`

At this point, **all Type UI reads go through XState**, not the Yjs store.

---

## 5. Rebuild `useTypeAppCommands` on top of XState events

Right now `useTypeAppCommands` does a mix of:

- `store.execute("compose.typeApp.*")` (commands defined in `commands` object).
- `store.change({ ... })` for local UI bits (panel visibility, camera, activeTool, selection, hover, selectedModelGuid).

We want:

- A **pure XState command API** that sends events.
- Optional bridging to existing commands for domain effects (TypeDiff etc.).

### 5.1. A purely XState-based command hook

```ts
export function useTypeAppCommands(id?: TypeAppId) {
 const actor = useSketchpadActor();
 const { kitGuid, typeGuid } = resolveTypeIds(id);
 const noOp = () => {};

 if (!kitGuid || !typeGuid) {
  return {
   startTransaction: noOp,
   finalizeTransaction: noOp,
   abortTransaction: noOp,
   undo: noOp,
   redo: noOp,
   togglePanel: noOp,
   setCamera: noOp,
   focusPort: noOp,
   clearFocus: noOp,
   setActiveTool: noOp,
   selectConnector: noOp,
   deselectConnector: noOp,
   selectAll: noOp,
   deselectAll: noOp,
   setHover: noOp,
   clearHover: noOp,
   setSelectedModel: noOp,
   addModelTag: noOp,
   removeModelTag: noOp,
   clearModelTags: noOp,
   setModelTags: noOp,
   execute: noOp,
  };
 }

 return {
  // TODO: wire transactions separately (see below)
  startTransaction: noOp,
  finalizeTransaction: noOp,
  abortTransaction: noOp,
  undo: noOp,
  redo: noOp,

  togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
   actor.send({ type: "TYPE.TOGGLE_PANEL", kitGuid, typeGuid, panel: panelKey });
  },

  setCamera: (origin: string, camera: Camera) => {
   actor.send({ type: "TYPE.SET_CAMERA", kitGuid, typeGuid, camera });
  },

  focusPort: (origin: string, connectorGuid: Guid) => {
   actor.send({ type: "TYPE.FOCUS_CONNECTOR", kitGuid, typeGuid, connectorGuid });
  },

  clearFocus: (origin: string) => {
   actor.send({ type: "TYPE.FOCUS_CONNECTOR", kitGuid, typeGuid, connectorGuid: undefined });
  },

  setActiveTool: (origin: string, tool: ToolKind) => {
   actor.send({ type: "TYPE.SET_ACTIVE_TOOL", kitGuid, typeGuid, tool });
  },

  selectConnector: (origin: string, connectorId: Guid) => {
   // Compose the new selection based on current context, or later via a TYPE.SELECT_CONNECTOR event
   // For now, get current selection via a selector and send TYPE.SET_SELECTION
  },

  // etc...
 };
}
```

In the **first interaction**, you can:

- Keep complex selection logic (`selectConnector`, `deselectConnector`, `selectAll`, `deselectAll`) inside the store (commands), and simply send a generic `TYPE.SYNC` when the store changes.
- Or, better, mirror what you plan for Design: add a `TYPE.EXECUTE_CMD` event that takes command name + args and uses the returned `TypeAppDiff` to update the XState context.

### 5.2. Bridge existing commands (optional transitional step)

Add to `SketchpadEvent`:

```ts
| { type: "TYPE.EXECUTE_CMD"; kitGuid: Guid; typeGuid: Guid; command: string; origin: string; args: any[] }
```

Add an action:

```ts
typeExecuteCmd: ({ context, event }) => {
  if (event.type !== "TYPE.EXECUTE_CMD") return;

  // You can either:
  // A) Call the existing TypeAppStore command runner and grab its diff
  // B) Or call pure functions that produce TypeDiff + TypeAppDiff from a context

  // Option A (transitional):
  const sketchpadStore = /* resolve from a service or closure when you build the machine */;
  const typeAppStore = sketchpadStore.typeApp(event.kitGuid, event.typeGuid);
  const result = typeAppStore.execute<TypeAppCommandResult>(
    event.command,
    event.origin,
    ...event.args,
  );

  // Apply TypeAppDiff to XState context (you’ll need a helper):
  // applyTypeAppDiff(context, key, result.diff)

  // TypeDiff is already applied by the store.
},
```

That’s essentially the same pattern as for Design: **UI diff goes into XState**, **domain diff goes into Yjs**.

Then in `useTypeAppCommands.execute` you just:

```ts
execute: (origin: string, command: string, ...args: any[]) =>
  actor.send({ type: "TYPE.EXECUTE_CMD", kitGuid, typeGuid, command, origin, args }),
```

…instead of calling `store.execute` directly from React.

---

## 6. Centralize Yjs persistence for Type app

Once the UI is reading from the machine and commands send events, you can start trimming Yjs responsibilities:

1. **Stop using `useSyncField`/`useSyncDeep` in Type hooks** (we already replaced them).

2. For any Type fields you still want to persist, update the relevant XState actions:
   - `typeSetCamera` → write to Yjs TypeApp yMap.
   - `typeSetWindowLayout` → same, with your “clear corrupt layout” logic moved from the store constructor into the action.
   - `typeSelectModelTag` / `typeDeselectModelTag` → optionally also write tags to Yjs.

3. Most purely local fields (`hover`, `selection`, `activeTool`, `fullscreenWindow`) can just live in XState, and nothing else.

You can share helpers with `TypeAppStore` code (e.g. a function that validates and sanitizes `windowLayout`) rather than duplicating.

---

## 7. Thin or remove `TypeAppStore`

After the previous steps, `TypeAppStore` is mostly needed for:

- **Domain commands** that generate `TypeDiff` (mutating the type graph).
- **Transaction / undo/redo** infrastructure around those diffs.

This is similar to Design:

1. Extract a **Type-domain service layer**:
   - Pure functions that take Yjs type data + a command and produce `TypeDiff` (and possibly `TypeAppDiff`).
   - Or existing command handlers reused outside of the store.

2. Move transaction state into a generic transaction service or into XState’s `transactions` map (which you already have in `SketchpadContext`).
3. Update `useTypeAppCommands` to call that domain service for mutating types, but let XState handle all UI.

Once nothing outside uses `useTypeAppStore` / `TypeAppStore` directly, you can:

- Remove `registerTypeAppStoreFactory`.
- Delete the `TypeAppStore` class entirely.

---

## 8. Suggested implementation order for Type app

To keep it manageable (and in sync with your Design work):

1. **Step 1 – Model + events**
   - Align `TypeAppState` definition in `machines.ts` with the one in `Type.tsx`.
   - Add missing `TYPE.*` events and actions for activeTool, selection, hover, fullscreen, selected model, tags, layout.

2. **Step 2 – Selectors + hooks (read path)**
   - Implement `createTypeAppSelector` and derived selectors.
   - Add XState-based hooks (`useTypeAppXState`, `useTypeAppSelectionXState`, etc.).
   - Flip existing `useTypeApp*` hooks to delegate to these.

3. **Step 3 – Commands (write path)**
   - Introduce `useTypeAppCommands` that sends `TYPE.*` events instead of `store.change` for UI.
   - Optionally introduce `TYPE.EXECUTE_CMD` and route existing commands through it.

4. **Step 4 – Yjs persistence**
   - Decide which Type fields still need Yjs.
   - Move persistence into relevant XState actions (`typeSetCamera`, `typeSetWindowLayout`, tag updates).
   - Stop persisting purely local UI fields in Yjs.

5. **Step 5 – Transactions + cleanup**
   - Extract transaction logic for Type edits into a shared transaction service.
   - Redirect any undo/redo UI in Type app to that service / root machine.
   - Delete `TypeAppStore` once nothing references it.

---

# Changes

- Created this log entry to hold the end-to-end transition description and link back to the existing migration context.

## Changes

## Log

## Summary

# Summary

Document full xstate transition process
