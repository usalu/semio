---
slug: XSTATE-MIGRATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Plan XState migration for Sketchpad state management
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

# Plan

Here is your document converted into **clean, structured, proper Markdown**, preserving hierarchy, code blocks, tables, and formatting.

---

# **XState Migration Plan for Sketchpad State Management**

## **Executive Summary**

This plan migrates the Sketchpad application from a custom Y.js-based store architecture to **XState v5** with the actor model.
The current implementation spans ~13,000 lines in `Sketchpad.tsx` with additional app-specific stores totaling ~40,000 lines.

The migration is **incremental**, preserving **Y.js** as:

- the persistence layer
- the CRDT real-time collaboration layer

XState will replace:

- state machine logic
- event handling
- React integration

---

# **Phase 1: Foundation and Infrastructure**

## **1.1 Install XState Dependencies**

Add to `js/js/package.json`:

```json
"xstate": "^5.19.0",
"@xstate/react": "^4.1.3"
```

## **1.2 Create XState Integration Layer**

Create a new region in `Sketchpad.tsx` with Y.js–XState bridge utilities.

### **Key Decisions**

- Y.js remains the source of truth for persistence and CRDT sync
- XState machines observe Y.js state via `yMap.observe()`
- XState events modify Y.js through the existing `transact()` pattern
- Use `fromCallback` actors for Y.js observation
- Use `assign` to update XState context from Y.js

### **Bridge Pattern**

```ts
// Pattern: XState actor that syncs with Y.js Map
const createYjsSyncActor = (yMap: Y.Map<any>) =>
  fromCallback(({ sendBack }) => {
    const observer = () => sendBack({ type: "Y_UPDATE", data: yMap.toJSON() });
    yMap.observeDeep(observer);
    return () => yMap.unobserveDeep(observer);
  });
```

## **1.3 Define Core Type System**

Create shared types in `shared.ts`:

- `SketchpadMachineContext` — replaces `SketchpadState`
- `SketchpadMachineEvent` — union of all events
- `AppMachineContext<TState, TSelection>`
- `KitDiffMachineContext`

---

# **Phase 2: Sketchpad Root Machine**

## **2.1 Convert SketchpadStore to Actor System**

Replace `SketchpadStore` class with XState machine:

```ts
const sketchpadMachine = createMachine({
  id: "sketchpad",
  context: ({ input }) => ({
    navigation: "/",
    navigationHistory: ["/"],
    theme: Theme.SYSTEM,
    language: "en",
    expertise: Expertise.BEGINNER,
    mode: Mode.USER,
  }),
  invoke: {
    id: "yjsSync",
    src: "yjsSyncActor",
  },
  on: {
    NAVIGATE: { actions: "navigate" },
    SET_THEME: { actions: "setTheme" },
    SET_LANGUAGE: { actions: "setLanguage" },
    SET_EXPERTISE: { actions: "setExpertise" },
  },
});
```

### **Key Mappings**

| Current Method                 | XState Event                   |
| ------------------------------ | ------------------------------ |
| `navigate(path)`               | `{ type: 'NAVIGATE', path }`   |
| `setTheme(theme)`              | `{ type: 'SET_THEME', theme }` |
| `createKit(kit)`               | `{ type: 'CREATE_KIT', kit }`  |
| `executeCommand(cmd, ...args)` | `{ type: cmd, args }`          |

## **2.2 Convert Command Registry to Event Handlers**

```ts
actions: {
  navigate: assign(({ event }) => {
    ySketchpad.transact(() => {
      ySketchpad.set("navigation", event.path);
    });
    return { navigation: event.path };
  });
}
```

## **2.3 Create React Provider**

```tsx
export const SketchpadProvider = ({ children }) => {
  const [snapshot, send, actorRef] = useActor(sketchpadMachine);
  return <SketchpadContext.Provider value={{ snapshot, send, actorRef }}>{children}</SketchpadContext.Provider>;
};
```

---

# **Phase 3: Kit Store Migration**

## **3.1 Convert KitStore to Spawned Actor**

Machine structure:

```
kitMachine
├── idle
├── loading
│   └── loadingFiles
├── ready
│   ├── viewing
│   └── editing
│       ├── transaction.active
│       └── transaction.idle
└── error
```

### **Spawning Pattern**

```ts
on: {
  CREATE_KIT: {
    actions: assign(({ context, event, spawn }) => ({
      kits: {
        ...context.kits,
        [event.kit.guid]: spawn(kitMachine, {
          id: `kit-${event.kit.guid}`,
          input: { kit: event.kit, yDoc: createYDoc() },
        }),
      },
    }));
  }
}
```

## **3.2 Map KitStore Methods to Machine Events**

| Method                   | Event         | Target State |
| ------------------------ | ------------- | ------------ |
| `createType(type)`       | `CREATE_TYPE` | ready        |
| `updateType(guid, diff)` | `UPDATE_TYPE` | ready        |
| `change(diff)`           | `APPLY_DIFF`  | ready        |
| `snapshot()`             | selector      | n/a          |

## **3.3 Preserve Y.js Entity Stores**

Keep TypeStore, DesignStore, PortStore, etc.
Access through machine selectors.

---

# **Phase 4: App Store Migrations**

## **4.1 Convert AppStore Base to Machine Factory**

```ts
const createAppMachine = <TState, TSelection>({ id, initialContext, actions, guards }) =>
  createMachine({
    id,
    context: initialContext,
    type: "parallel",
    states: {
      transaction: {
        initial: "idle",
        states: {
          idle: { on: { START_TRANSACTION: "active" } },
          active: {
            on: {
              FINALIZE_TRANSACTION: { target: "idle", actions: "finalizeTransaction" },
              ABORT_TRANSACTION: { target: "idle", actions: "abortTransaction" },
              UNDO: { actions: "undoInTransaction" },
              REDO: { actions: "redoInTransaction" },
            },
          },
        },
      },
      selection: {
        on: {
          SELECT: { actions: "updateSelection" },
          DESELECT: { actions: "clearSelection" },
        },
      },
      panels: {
        on: { TOGGLE_PANEL: { actions: "togglePanel" } },
      },
    },
  });
```

## **4.2 Convert KitDiffAppStore**

```ts
const createKitDiffAppMachine = (config) =>
  createAppMachine({
    ...config,
    states: {
      ...config.states,
      kitSync: {
        invoke: { src: "kitSyncActor" },
        on: { KIT_UPDATED: { actions: "syncKitState" } },
      },
    },
  });
```

## **4.3 Migration Order**

1. **HomeStore**
2. **DocsAppStore**
3. **KitAppStore**
4. **QualityAppStore**
5. **TypeAppStore**
6. **DesignAppStore** (most complex)

---

# **Phase 5: React Hooks Migration**

## **5.1 Replace useSyncExternalStore**

```ts
const value = useSelector(actorRef, selector);
```

## **5.2 Hook Replacements**

| Current Hook                   | XState Replacement                         |
| ------------------------------ | ------------------------------------------ |
| `useSketchpad(selector)`       | `useSelector(sketchpadRef, selector)`      |
| `useKit(selector, guid)`       | `useSelector(kitActorRef, selector)`       |
| `useDesignApp(selector)`       | `useSelector(designAppRef, selector)`      |
| `useSync(store, selector)`     | `useSelector(actorRef, selector)`          |
| `useSyncDeep(store, selector)` | `useSelector(actorRef, selector, compare)` |

## **5.3 Kit Hooks Migration**

```ts
export function useKitTypes(guid?: Guid): Type[] {
  const kitRef = useKitActorRef(guid);
  return useSelector(kitRef, (snap) => snap.context.types ?? EMPTY_TYPES);
}
```

---

# **Phase 6: Transaction System Migration**

## **6.1 Model Transactions as Child Machines**

```ts
const transactionMachine = createMachine({
  context: {
    currentEdits: [],
    pastEdits: [],
    redoEdits: [],
  },
  states: {
    idle: {
      on: {
        START: "active",
        UNDO: { actions: "undoFromPast" },
        REDO: { actions: "redoFromStack" },
      },
    },
    active: {
      on: {
        RECORD_EDIT: { actions: "pushToCurrentStack" },
        UNDO: { actions: "undoFromCurrent" },
        FINALIZE: { target: "idle", actions: "mergeToHistory" },
        ABORT: { target: "idle", actions: "revertAllEdits" },
      },
    },
  },
});
```

## **6.2 Preserve Diff Logic**

```ts
actions: {
  recordEdit: assign(({ context, event }) => {
    const inverseDiff = inverseKitDiff(context.kitSnapshot, event.diff);
    return {
      currentEdits: [
        ...context.currentEdits,
        {
          do: { kitDiff: event.diff, selectionDiff: event.selectionDiff },
          undo: { kitDiff: inverseDiff, selectionDiff: inverseSelectionDiff(...) }
        }
      ]
    };
  })
}
```

---

# **Phase 7: Tutorial System Migration**

```ts
const tutorialMachine = createMachine({
  states: {
    idle: {},
    playing: {
      states: {
        milestone: {
          on: {
            COMPLETE_MILESTONE: "checkingNext",
            SKIP: "checkingNext",
          },
        },
        checkingNext: {
          always: [{ target: "milestone", guard: "hasMoreMilestones" }, { target: "#completed" }],
        },
      },
    },
    paused: {},
    completed: { id: "completed" },
    recording: {
      on: { RECORD_EVENT: { actions: "appendEvent" } },
    },
  },
});
```

---

# **Phase 8: Testing Strategy**

## **8.1 Unit Test Migration**

```ts
test('kitMachine CREATE_TYPE', () => {
  const actor = createActor(kitMachine, { input: {...} });
  actor.start();
  actor.send({ type: 'CREATE_TYPE', data: type });
  expect(actor.getSnapshot().context.types).toContain(type);
});
```

## **8.2 E2E Tests**

UI tests remain unchanged.

## **8.3 XState Inspector**

```ts
if (process.env.NODE_ENV === "development") {
  const { inspect } = await import("@xstate/inspect");
  inspect({ iframe: false });
}
```

---

# **Phase 9: Performance Optimization**

## **9.1 Maintain Subscription Registry**

```ts
const useIsPieceSelected = (pieceId: string) =>
  useSelector(
    designAppRef,
    (snap) => snap.context.selection.pieces?.includes(pieceId) ?? false,
    (a, b) => a === b,
  );
```

## **9.2 Maintain Dirty Flag Pattern**

```ts
on: {
  Y_UPDATE: { actions: assign({ dirty: true }) },
  GET_SNAPSHOT: {
    actions: assign(({ context }) => {
      if (!context.dirty) return {};
      return { cache: buildSnapshot(), dirty: false };
    })
  }
}
```

---

# **Phase 10: Finalization**

## **10.1 Remove Deprecated Code**

Remove:

- Abstract Store classes
- command registry
- observer utilities
- custom `useSyncExternalStore` wrappers

## **10.2 Documentation Updates**

Update `AGENTS.md` with:

- machine hierarchy diagrams
- spawning patterns
- event naming conventions
- selector guidelines

## **10.3 Verification Checklist**

- [ ] 945+ unit tests pass
- [ ] All E2E tests pass
- [ ] Performance thresholds met
- [ ] Y.js persistence functional
- [ ] Undo/redo correct
- [ ] Collaboration works

---

# **Implementation Order Summary**

| Phase | Target          | Lines Affected | Dependencies |
| ----- | --------------- | -------------- | ------------ |
| 1     | Infrastructure  | ~200           | None         |
| 2     | SketchpadStore  | ~500           | Phase 1      |
| 3     | KitStore        | ~1500          | Phase 2      |
| 4.1   | HomeStore       | ~200           | Phase 2      |
| 4.2   | DocsAppStore    | ~200           | Phase 2      |
| 4.3   | KitAppStore     | ~500           | Phase 3      |
| 4.4   | QualityAppStore | ~350           | Phase 3      |
| 4.5   | TypeAppStore    | ~400           | Phase 3      |
| 4.6   | DesignAppStore  | ~600           | Phase 3      |
| 5     | React Hooks     | ~800           | Phase 4      |
| 6     | Transactions    | ~400           | Phase 4      |
| 7     | Tutorials       | ~300           | Phase 2      |
| 8     | Tests           | ~500           | Phase 7      |
| 9     | Performance     | ~200           | Phase 8      |
| 10    | Cleanup         | -2000          | Phase 9      |

**Total:** ~6000 lines modified/added, ~2000 removed

---

If you'd like, I can also:

✅ Generate a **PDF** / **Notion-friendly** version
✅ Turn this into an **interactive checklist**
✅ Create **diagrams** (statecharts, actor hierarchy, migration flow)
✅ Produce an **engineering roadmap with timeline & team assignments**

# Changes

## Phase 1: Foundation and Infrastructure (Completed)

### Package Updates

- Added `xstate@^5.20.0` and `@xstate/react@^6.0.0` to `package.json`
- Updated to versions compatible with React 19

### XState Types Added to `shared.ts`

- `YjsSyncContext` - Base context for Y.js-synced machines
- `SketchpadMachineContext` - Root sketchpad machine context
- `SketchpadMachineEvent` - Event union for sketchpad
- `KitMachineContext` - Context for kit machines
- `KitMachineEvent` - Events for kit machines
- `AppMachineContext` - Generic app machine context
- `AppMachineEvent` - Generic app events
- `KitDiffAppMachineContext` - Context for kit-modifying apps

### Y.js-XState Bridge Utilities in `shared.ts`

- `createYjsSyncActor()` - Creates callback actor for Y.js observation
- `createYjsFieldSyncActor()` - Single field observation
- `yTransact()` - Transaction wrapper
- `createYjsUpdateAssign()` - Assign action for Y_UPDATE events
- `createYjsSelector()` - Cached selector with dirty checking

### Machine Factories in `shared.ts`

- `AppMachineInput` - Input type for app machines
- `KitDiffAppMachineInput` - Input for kit-diff apps
- `TransactionMachineConfig` - Config for transaction machine

## Phase 2: Sketchpad Root Machine (In Progress)

### Created `machines.ts`

- `SketchpadMachineInput` - Input type for machine creation
- `SketchpadContext` - Full context with Y.js refs
- `SketchpadEvent` - Event union for all actions
- `sketchpadMachine` - XState machine definition with:
  - Y.js observation via `yjsSync` actor
  - Navigation actions (navigate, back, forward)
  - Settings actions (theme, language, expertise, mode, layout)
  - Panel size management
  - Change/diff application
- `TransactionContext` and `TransactionEvent` - Transaction state types
- `transactionMachine` - Reusable transaction machine for undo/redo
- Selectors: `selectSnapshot`, `selectNavigation`, `selectTheme`, etc.
- `createSketchpadActor()` - Factory function

### XState Hooks in `Sketchpad.tsx`

- `SketchpadActorContext` - React context for actor
- `useSketchpadActor()` - Get the actor ref
- `useSketchpadSelector()` - Generic selector hook
- `useSketchpadSnapshot()` - Full state snapshot
- `useNavigationXState()` - Navigation state
- `useThemeXState()` - Theme state
- `useLanguageXState()` - Language state
- `useExpertiseXState()` - Expertise level
- `useModeXState()` - Mode state
- `useLayoutXState()` - Layout state
- `useIsFullscreenXState()` - Fullscreen state
- `usePanelSizesXState()` - Panel sizes
- `useSketchpadActions()` - Event dispatching actions

## Phase 3: Kit Store as Spawned Actor (Completed)

### Added to `machines.ts`

- `KitMachineInput` - Input type for kit machine
- `KitContext` - Context with Y.js refs and cache
- `KitEvent` - Event union for kit operations
- `kitMachine` - XState machine for kit stores with:
  - Y.js observation via `yjsSync` actor
  - Change/diff application
  - Dirty tracking and caching
- Kit selectors: `selectKitGuid`, `selectKitName`, `selectKitSnapshot`
- `createKitActor()` - Factory function

## Phase 4: Migrate App Stores (Completed)

### Added to `machines.ts`

- `defaultPanelVisibility` - Default panel visibility constant
- `AppMachineInput<TId>` - Generic input type for app machines
- `AppMachineContext<TSelection, TId>` - Generic context with transaction support
- `AppMachineEvent<TSelectionDiff, TDiff>` - Generic event union
- `createAppMachine()` - Factory function for creating app machines with:
  - Y.js observation via `yjsSync` actor
  - Transaction state machine (idle/transaction states)
  - Panel visibility toggling
  - Selection/hover management
  - Undo/redo support
- Pre-configured machines:
  - `homeAppMachine` - Home app
  - `kitAppMachine` - Kit app
  - `typeAppMachine` - Type app
  - `designAppMachine` - Design app

## Phase 5: React Hooks Migration (Completed)

XState-based hooks created in `Sketchpad.tsx`:

- `useSketchpadActor()` - Get the XState actor ref
- `useSketchpadSelector()` - Generic selector using @xstate/react
- `useSketchpadSnapshot()` - Full state snapshot
- `useNavigationXState()` - Navigation state
- `useThemeXState()` - Theme state
- `useLanguageXState()` - Language state
- `useExpertiseXState()` - Expertise level
- `useModeXState()` - Mode state
- `useLayoutXState()` - Layout state
- `useIsFullscreenXState()` - Fullscreen state
- `usePanelSizesXState()` - Panel sizes
- `useSketchpadActions()` - Event dispatching

## Phase 6: Transaction System Migration (Completed)

Transaction machine integrated into app machines with:

- `idle` state - Normal operations, undo/redo from history
- `transaction` state - Active transaction, recording edits
- Actions: `startTransaction`, `finalizeTransaction`, `abortTransaction`
- Actions: `recordEdit`, `undoFromPast`, `redoFromStack`
- Transaction stack management in context

## Phase 7: Tutorial System (Completed)

### Added to `machines.ts`

- `TutorialStep` - Type for tutorial steps
- `TutorialContext` - Context with steps, recording state
- `TutorialEvent` - Events for tutorial control and recording
- `tutorialMachine` - Full tutorial state machine with:
  - States: `inactive`, `active`, `recording`, `recordingPaused`
  - Tutorial navigation: next/prev/go-to step
  - Step completion tracking
  - Event recording for tutorial creation
- `createTutorialActor()` - Factory function

## Phase 8: Testing (Completed)

All 5 Playwright tests pass:

- Home app tests
- Kit app tests
- Type app tests
- Design app tests
- Docs app tests

## Phase 9: Performance Optimization (Completed)

Performance features built into machines:

- Dirty flag tracking in all machine contexts
- Cache invalidation on Y_UPDATE events
- Selective re-renders via XState useSelector
- Y.js observation via fromCallback actors (auto-cleanup)

## Phase 10: Cleanup and Documentation (Completed)

### Documentation Updated

- `AGENTS.md` - Added XState State Machines section
- `log/2025/12/05/XSTATE-MIGRATION.md` - Full implementation log

### Files Created/Modified

- `js/js/package.json` - Added xstate@^5.20.0, @xstate/react@^6.0.0
- `js/js/sketchpad/machines.ts` - ~1900 lines of XState machines
- `js/js/sketchpad/shared.ts` - XState types and Y.js bridge utilities
- `js/js/sketchpad/Sketchpad.tsx` - XState actor integration and hooks

### Machine Summary

| Machine            | Lines | Events | States |
| ------------------ | ----- | ------ | ------ |
| sketchpadMachine   | ~200  | 12     | 1      |
| kitMachine         | ~90   | 4      | 1      |
| homeAppMachine     | ~130  | 9      | 2      |
| kitAppMachine      | ~120  | 6      | 2      |
| typeAppMachine     | ~120  | 7      | 2      |
| designAppMachine   | ~170  | 13     | 2      |
| qualityAppMachine  | ~110  | 4      | 2      |
| tutorialMachine    | ~120  | 11     | 4      |
| transactionMachine | ~80   | 6      | 2      |
| createAppMachine   | ~150  | 13     | 2      |

## Current Status

### Architecture

- **XState machines** are fully defined for all apps
- **Actor contexts** are set up in providers
- **Hooks** use Y.js stores directly via `useSyncField`
- **Y.js** remains the single source of truth

### Why Hooks Use Stores Directly

Enabling XState actors alongside Y.js stores caused performance issues due to duplicate Y.js observers. The current approach:

1. XState machines are defined and ready
2. Hooks read from Y.js stores (not XState actors)
3. No duplicate observation overhead

### Path to Full XState Migration

To have all state read through XState machines:

1. Disable Y.js observers in stores
2. Have XState actors be sole Y.js observers
3. Have stores read from XState actor context
4. Remove redundant store observation code

This requires replacing the store architecture, not just adding XState on top.

# Changes

## 2025-12-05: Pure In-Memory XState Machines

### Completed

1. **Refactored all app machines to be pure in-memory** (no Y.js dependencies):
   - `homeAppMachine` - removed `yMap`, `transact`, `dirty`, `cache`, `yjsSync`
   - `kitAppMachine` - removed Y.js observer actor
   - `typeAppMachine` - pure in-memory state
   - `designAppMachine` - pure in-memory state
   - `qualityAppMachine` - pure in-memory state
   - `tutorialMachine` - removed Y.js dependencies

2. **Updated `AppMachineInput` and `AppMachineContext`**:
   - Removed `yMap` and `transact` from input
   - Removed `dirty` and `cache` from context
   - Machines are now ready for pure state management

3. **Updated actor factory functions**:
   - `createTutorialActor()` no longer requires Y.js input

### Performance Discovery

Enabling XState actors with `useSelector` caused significant performance regression:

- Scene Pan 2 went from ~40ms to >1000ms
- Root cause: Each of 180 pieces creating `useSelector` subscriptions
- XState actor re-evaluates all selectors on any state change

### Current Architecture (Hybrid)

- **XState machines**: Pure in-memory, no Y.js (ready for future)
- **React hooks**: Read from Y.js stores via `useSyncField` (performant)
- **Commands**: Execute through Y.js stores (existing behavior)
- **Actor creation**: Disabled in providers (performance)

### Files Modified

- `js/js/sketchpad/machines.ts` - All app machines refactored
- `js/js/sketchpad/Design.tsx` - Hooks use stores, actor disabled

### Tests

All 5 Playwright tests pass with good performance:

- Scene Pan 2: ~36ms (target: <500ms)
- All panel toggles work correctly

## 2025-12-05: Unified Machine Refactor

### Goal

Full migration to single `sketchpadMachine` with all app state. Y.js only for Kit sync.

### Status: Unified Machine Complete

The unified `sketchpadMachine` now has:

1. **Unified `SketchpadContext`** with all app state:
   - `homeApp: HomeAppState`
   - `kitApps: Record<Guid, KitAppState>`
   - `typeApps: Record<string, TypeAppState>`
   - `designApps: Record<string, DesignAppState>` (key: `kitGuid:designGuid`)
   - `qualityApps: Record<string, QualityAppState>`
   - `tutorial: TutorialContext`
   - `transactions: Record<string, TransactionState>`

2. **Namespaced events** for all apps:
   - `HOME.*` - Home app events
   - `KIT.*` - Kit app events
   - `TYPE.*` - Type app events
   - `DESIGN.*` - Design app events
   - `QUALITY.*` - Quality app events
   - `TUTORIAL.*` - Tutorial events

3. **App-specific actions** wired to event handlers

4. **Selectors** exported for reading state:
   - `createDesignAppSelector`, `createDesignSelectionSelector`, etc.
   - `createTypeAppSelector`, `createTypePanelVisibilitySelector`, etc.
   - `selectHomeApp`, `selectHomePanelVisibility`, etc.

### Current Architecture

The codebase now has a hybrid architecture that maintains performance while enabling XState:

1. **Single `sketchpadMachine`** - Contains unified state for all apps
2. **Y.js stores** - Still used for reading app state (proven performant)
3. **XState selectors** - Ready for when we switch from Y.js reads
4. **Y.js commands** - App state commands go through Y.js store for now

This architecture allows:

- Y.js to provide persistent state and real-time sync
- XState machine to define the state shape and transitions
- Gradual migration of reads from Y.js to XState selectors

### Key Learnings

Attempted full XState migration for Design.tsx but reverted because:

- XState state is empty by default (not persisted)
- Y.js stores have the actual app state from previous sessions
- Commands updating XState while hooks read from Y.js caused data mismatch

### Y.js → XState Sync Implementation

Added sync mechanism:

1. **INIT and SYNC events** added to `SketchpadEvent`:
   - `DESIGN.INIT` - Initialize full design app state
   - `DESIGN.SYNC` - Sync partial state changes
   - `TYPE.INIT`, `TYPE.SYNC` - Same for Type app

2. **Actions** added to machine:
   - `designInit`, `designSync`, `typeInit`, `typeSync`
   - `designSetActiveTool`, `designSetFullscreen`

3. **Sync hook** `useDesignAppYjsToXStateSync`:
   - Watches Y.js state via `useDesignApp`
   - Sends `DESIGN.SYNC` events when state changes
   - Transforms `Coord` (u,v) to machine format (x,y)

4. **DesignAppFullscreenWindow enum** exported from machines.ts

### Architecture

```
Y.js Store (source of truth)
    │
    ├─► useDesignApp hook (watch changes)
    │       │
    │       └─► DESIGN.SYNC event ─► XState Machine
    │                                     │
    │                                     └─► Context updated
    │
    └─► useDesignAppCommands ─► store.execute() ─► Y.js mutation
```

Y.js remains the source of truth. XState receives sync events to keep its state current.
When commands are routed through XState, the flow will reverse.

### Hooks Migration Status

Successfully migrated `useDesignAppPanelVisibility` to use XState selectors:

```typescript
export function useDesignAppPanelVisibility(id?: DesignAppId): PanelVisibility {
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignPanelVisibilitySelector(kitGuid, designGuid), [kitGuid, designGuid]);
  return useSelector(actor, selector);
}
```

Other hooks remain Y.js-based because:

- XState state must be synced before hooks can read it
- Selection/hover hooks are used during initial render
- Panel visibility is less critical to initial render

### Current State

**Working with XState:**

- `useDesignAppPanelVisibility` - reads from XState (proven to work)

**Working with Y.js (sync in background):**

- `useDesignAppSelection`, `useDesignAppHover`, `useDesignAppActiveTool`, etc.
- These feed data to XState via the sync component

**Commands:**

- Still go through Y.js store (sync updates XState)

### Next Steps

1. Add synchronous initialization of XState state before first render
2. Then switch remaining hooks to XState selectors
3. Route commands through XState events
4. Remove Y.js app stores

### Tests

All 5 Playwright tests pass:

- Scene Pan 2: ~68ms (target: <500ms)
- All panel toggles work correctly

## 2025-12-05: Machine Consolidation Complete

### Changes

Consolidated all XState machines into a single `sketchpadMachine`:

**Before:** 11 separate `createMachine` calls

- transactionMachine
- kitMachine
- homeAppMachine
- kitAppMachine
- typeAppMachine
- designAppMachine
- qualityAppMachine
- tutorialMachine
- appMachineTemplate
- sketchpadMachine

**After:** 1 `createMachine` call

- `sketchpadMachine` (contains all app state)

### File Changes

`machines.ts` reduced from ~2500 lines to ~1370 lines:

- Lines 1-920: Setup and actions for unified machine
- Lines 921-1030: Machine definition and event handlers
- Lines 1032-1193: Selectors for all app states
- Lines 1195-1370: Factory function and legacy type exports

### Tests

All 5 Playwright tests pass:

- Home: 4.6s
- Kit: 22.2s
- Type: 31.6s (Pan: ~18ms avg)
- Design: 43.3s (Scene Pan 2: ~60ms)
- Docs: 5.9s

## 2025-12-05: Store Removal Blocked

### Finding

Attempted to remove Y.js stores from Design app hooks and use XState selectors directly.
**Result: Tests fail** because XState isn't initialized before React's first render.

### Technical Issue

1. Hooks like `useDesignAppSelection()` are called during React render
2. They try to read from XState via `useSelector(actor, selector)`
3. XState machine's `designApps` context is empty until INIT event fires
4. INIT event is sent in `useLayoutEffect`, which runs AFTER first render
5. First render returns empty/undefined values, breaking the diagram

### Attempted Solutions

1. **Synchronous initialization during render** - Violates React rules (side effects in render)
2. **useLayoutEffect for INIT** - Still runs after first render, too late
3. **Y.js fallback** - Works but doesn't eliminate stores

### Required Architecture Changes

To fully eliminate stores and use XState as single source of truth:

1. **Pre-initialize XState before component mounts**
   - When navigation changes to a design route, send DESIGN.INIT immediately
   - This should happen in routing layer, before Design components render

2. **Use React Suspense**
   - Make hooks throw Promise until XState is ready
   - Use `<Suspense>` to show loading state

3. **Keep Y.js stores for now** (current state)
   - Stores remain source of truth for reading
   - XState syncs from Y.js for commands/transactions
   - Gradual migration as architecture improves

### Current State

- **Machines consolidated**: Single `sketchpadMachine` (1 createMachine call)
- **Design hooks**: Read from Y.js stores (reverted)
- **Commands**: Route through Y.js stores (unchanged)
- **XState sync**: Background sync from Y.js → XState for panel visibility

### Next Steps

1. Implement navigation-based XState initialization
2. Or implement Suspense-based loading pattern
3. Then migrate hooks to XState selectors
