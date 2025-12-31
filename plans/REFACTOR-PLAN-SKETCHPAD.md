# Sketchpad Refactor Plan

> **Scope:** Code-only refactoring analysis. No file structure modifications proposed.
>
> **Generated:** Analysis of all sketchpad modules from top to bottom.

---

## Completion Status

| Item | Status | Notes |
|------|--------|-------|
| Generic ArrayDiff types | ✅ Complete | Shared in shared.ts |
| Event handler consolidation | ✅ Complete | All apps migrated to registerEventHandler |
| Legacy runtime action removal | ✅ Complete | Removed registerRuntimeAction, executeRuntimeAction |
| Transaction handler factory | ✅ Complete | createKeyedTransactionHandlers in shared.ts |
| Selector factory pattern | ✅ Complete | createAppPropertySelector in shared.ts |
| Y.js primitive store generic | ⬜ Deferred | Complex due to composite stores |
| Registry unification | ⬜ Deferred | Low impact |
| Type-safe events | ⬜ Deferred | High complexity |

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [File Statistics](#file-statistics)
3. [Pattern Analysis](#pattern-analysis)
4. [Refactoring Opportunities by File](#refactoring-opportunities-by-file)
5. [Cross-Cutting Concerns](#cross-cutting-concerns)
6. [Priority Matrix](#priority-matrix)

---

## Executive Summary

The sketchpad codebase consists of 10 major files totaling approximately **41,000+ lines of code**. The architecture follows a plugin-based app system with XState for UI state management and Y.js for collaborative kit data. While the patterns are consistent, significant opportunities exist for reducing duplication, improving type safety, and extracting reusable abstractions.

### Key Findings

| Category | Issue Count | Impact |
|----------|-------------|--------|
| Code Duplication | 15+ patterns | High |
| Type Safety | 8 areas | Medium |
| Abstraction Gaps | 12 patterns | Medium |
| Consistency Issues | 20+ instances | Low |

---

## File Statistics

| File | Lines | Primary Responsibility |
|------|-------|----------------------|
| `Sketchpad.tsx` | 15,951 | Main orchestrator, stores, providers, hooks |
| `Design.tsx` | 8,284 | Design app (pieces, connections) |
| `Kit.tsx` | 6,671 | Kit app (types, designs, qualities) |
| `Type.tsx` | 3,455 | Type app (connectors, models) |
| `Quality.tsx` | 1,931 | Quality app (formulas, benchmarks) |
| `Home.tsx` | 1,732 | Home app (kit management) |
| `shared.ts` | 1,724 | Types, enums, registries, utilities |
| `Docs.tsx` | 1,346 | Documentation app (MDX) |
| `Tutorials.tsx` | 1,035 | Tutorial system |
| `elements.tsx` | 5,906 | UI components, layout system |
| `Feedback.tsx` | 542 | Feedback form app |

**Total:** ~48,577 lines

---

## Pattern Analysis

### 1. State Management Patterns

#### 1.1 Duplicated XState Event Handler Registration

**Location:** Every app file (Home.tsx, Kit.tsx, Type.tsx, Design.tsx, Quality.tsx, Feedback.tsx)

**Pattern:**
```typescript
// Repeated in every app
if (typeof window !== "undefined") {
  registerAppPlugin(appPlugin);
  
  registerEventHandler("APP.TOGGLE_PANEL", {
    action: (context: any, event: any) => ({
      appApp: {
        ...context.appApp,
        panelVisibility: {
          ...context.appApp.panelVisibility,
          [event.panel]: !context.appApp.panelVisibility[event.panel],
        },
      },
    }),
  });
  // ... more handlers
}
```

**Issue:** Each app independently implements nearly identical event handlers for:
- `TOGGLE_PANEL`
- `SET_PANEL_VISIBILITY`
- `SET_HOVER`
- `CLEAR_HOVER`
- `SET_SELECTION`
- `CLEAR_SELECTION`
- `SET_WINDOW_LAYOUT`

**Refactor Opportunity:**
- Create a generic `createAppEventHandlers<TState>(namespace, config)` factory
- Use TypeScript generics to maintain type safety
- Reduce ~50-100 lines per app file

#### 1.2 Runtime Action vs Event Handler Duality

**Location:** `shared.ts`, `Type.tsx`, `Design.tsx`

**Pattern:** Two parallel systems exist:
- `registerRuntimeAction(name, handler)` - older pattern
- `registerEventHandler(eventType, config)` - newer pattern

**Issue:** Type.tsx uses `registerRuntimeAction` extensively while Home.tsx and Feedback.tsx use `registerEventHandler`. This creates inconsistency and confusion.

**Files Using RuntimeAction:**
- Type.tsx (30+ handlers)
- Design.tsx (25+ handlers)
- Kit.tsx (20+ handlers)

**Files Using EventHandler:**
- Home.tsx (8 handlers)
- Feedback.tsx (6 handlers)

**Refactor Opportunity:**
- Migrate all `registerRuntimeAction` calls to `registerEventHandler`
- Remove legacy `registerRuntimeAction` mechanism
- Consolidate into single event dispatch pattern

#### 1.3 Transaction State Management Duplication

**Location:** Type.tsx, Design.tsx, Kit.tsx

**Pattern:** Each app duplicates transaction management handlers:
```typescript
registerRuntimeAction("typeTransactionStart", (context, event) => { ... });
registerRuntimeAction("typeTransactionCommit", (context, event) => { ... });
registerRuntimeAction("typeTransactionAbort", (context, event) => { ... });
registerRuntimeAction("typeTransactionUndo", (context, event) => { ... });
registerRuntimeAction("typeTransactionRedo", (context, event) => { ... });
registerRuntimeAction("typeTransactionRecordEdit", (context, event) => { ... });
```

**Issue:** ~60 lines duplicated across 3 files (180+ lines total)

**Refactor Opportunity:**
- Extract `createTransactionHandlers(namespace, appKey)` factory
- Parameterize by app namespace and state key

---

### 2. Hook Patterns

#### 2.1 Triadic Hook Boilerplate

**Location:** All app files

**Pattern:** Each hook follows the same structure:
```typescript
export function useTypeAppSelection(): HookResult<TypeAppSelection> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeSelectionSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_TYPE_SELECTION;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_SELECTION" as const, kitGuid, typeGuid, selection: {} }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (selection: TypeAppSelection) => {
      actor.send({ type: "TYPE.SET_SELECTION", kitGuid, typeGuid, selection });
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}
```

**Issue:** This 15-20 line pattern repeats 40+ times across all apps.

**Refactor Opportunity:**
- Create `createTriadicHook<T>(config)` factory:
```typescript
const useTypeAppSelection = createTriadicHook({
  namespace: "TYPE",
  action: "SET_SELECTION",
  scopeKeys: ["kit", "type"],
  createSelector: createTypeSelectionSelector,
  defaultValue: EMPTY_TYPE_SELECTION,
  payloadKey: "selection",
});
```
- Reduce per-hook code from 20 lines to 6 lines

#### 2.2 Selector Factory Pattern Repetition

**Location:** Sketchpad.tsx, Type.tsx, Design.tsx, Kit.tsx

**Pattern:**
```typescript
export const createTypeSelectionSelector = (kitGuid: Guid, typeGuid: Guid) => (snapshot: any) => {
  const key = `${kitGuid}:${typeGuid}`;
  return snapshot.context.typeApps?.[key]?.selection;
};

export const createTypePanelVisibilitySelector = (kitGuid: Guid, typeGuid: Guid) => (snapshot: any) => {
  const key = `${kitGuid}:${typeGuid}`;
  return snapshot.context.typeApps?.[key]?.panelVisibility;
};
// ... 10+ more per app
```

**Issue:** Each property requires a separate selector factory with identical structure.

**Refactor Opportunity:**
- Create `createAppPropertySelectorFactory<T>(appKey, propertyPath)`:
```typescript
const createTypeSelector = createAppPropertySelectorFactory("typeApps");
export const createTypeSelectionSelector = createTypeSelector("selection");
export const createTypePanelVisibilitySelector = createTypeSelector("panelVisibility");
```

---

### 3. Store Architecture Patterns

#### 3.1 Y.js Primitive Store Duplication

**Location:** Sketchpad.tsx (lines 1400-2000)

**Pattern:** Separate store classes for each primitive type:
```typescript
class YCoordStore extends Store<Coord> { ... }
class YVecStore extends Store<Vec> { ... }
class YPointStore extends Store<Point> { ... }
class YVectorStore extends Store<Vector> { ... }
class YPlaneStore extends Store<Plane> { ... }
class YCameraStore extends Store<Camera> { ... }
class YLocationStore extends Store<Location> { ... }
class AuthorStore extends Store<Author> { ... }
```

**Issue:** Each class follows identical structure:
1. Constructor with Y.Map initialization
2. Getters for each field
3. `buildSnapshot()` method
4. `hash()` method

**Refactor Opportunity:**
- Create generic `YPrimitiveStore<T>` with schema definition:
```typescript
const YCoordStore = createYPrimitiveStore<Coord>({
  fields: { u: 'number', v: 'number' },
  defaults: { u: 0, v: 0 },
});
```

#### 3.2 App Store Key Pattern

**Location:** Type.tsx, Design.tsx, Kit.tsx, Quality.tsx

**Pattern:** All keyed app stores use same key computation:
```typescript
const key = `${kitGuid}:${typeGuid}`;
const app = context.typeApps[key] || createDefaultTypeAppState();
```

**Issue:** Key computation repeated in every handler.

**Refactor Opportunity:**
- Extract `getAppKey(scopes: string[])` utility
- Extract `getOrCreateAppState(context, appKey, key, defaultFactory)`

---

### 4. Selection/Diff System Patterns

#### 4.1 Selection Type Definitions

**Location:** Each app file

**Pattern:** Every app defines parallel selection structures:
```typescript
// Kit.tsx
interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  // ...
}
interface KitAppSelectionTypesDiff {
  added?: Guid[];
  removed?: Guid[];
}
interface KitAppSelectionDiff {
  types?: KitAppSelectionTypesDiff;
  // ...
}

// Type.tsx  
interface TypeAppSelection {
  connectors?: Guid[];
  models?: Guid[];
}
interface TypeAppSelectionPortsDiff {
  added?: Guid[];
  removed?: Guid[];
}
// ...

// Design.tsx
interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  connector?: Guid;
}
// ...
```

**Issue:** Identical diff structure (`added?`, `removed?`) repeated for each property.

**Refactor Opportunity:**
- Create generic types:
```typescript
type ArrayDiff<T> = { added?: T[]; removed?: T[] };
type SelectionDiff<T extends Record<string, any[]>> = {
  [K in keyof T]?: ArrayDiff<T[K][number]>;
};
```

#### 4.2 Inverse Selection Diff Functions

**Location:** Kit.tsx, Type.tsx, Design.tsx, Quality.tsx

**Pattern:** Each app implements identical inverse logic:
```typescript
export const inverseKitAppSelectionDiff = (selection, diff) => {
  const inverseDiff = {};
  if (diff.types) {
    inverseDiff.types = {};
    if (diff.types.added) inverseDiff.types.removed = diff.types.added;
    if (diff.types.removed) inverseDiff.types.added = diff.types.removed;
  }
  // ... repeat for each property
};
```

**Issue:** Same pattern repeated for each selection property in each app.

**Refactor Opportunity:**
- Create generic `inverseArrayDiff<T>(diff)`:
```typescript
const inverseArrayDiff = <T>(diff: ArrayDiff<T>): ArrayDiff<T> => ({
  added: diff.removed,
  removed: diff.added,
});

const inverseSelectionDiff = <T extends Record<string, ArrayDiff<any>>>(
  diff: T
): T => Object.fromEntries(
  Object.entries(diff).map(([k, v]) => [k, inverseArrayDiff(v)])
) as T;
```

---

### 5. UI Component Patterns

#### 5.1 Panel Section Registration

**Location:** All app files

**Pattern:**
```typescript
useLayoutEffect(() => {
  if (appType !== "design") return;
  
  addSection("toolbar", {
    id: "semio.sketchpad.app.design.toolbar.tools",
    specificity: 20,
    order: 0,
    content: <ToolsToolbar />,
  });
  
  return () => {
    removeSection("toolbar", "semio.sketchpad.app.design.toolbar.tools");
  };
}, [appType, addSection, removeSection]);
```

**Issue:** Effect boilerplate repeated for each section in each app.

**Refactor Opportunity:**
- Create `useAppSection(appType, panelKind, config)` hook:
```typescript
useAppSection("design", "toolbar", {
  id: "semio.sketchpad.app.design.toolbar.tools",
  specificity: 20,
  order: 0,
  content: <ToolsToolbar />,
});
```

#### 5.2 Footer Item Registration

**Location:** All app files

**Pattern:** Similar to panel sections but for footer:
```typescript
useEffect(() => {
  addFooterItem({
    id: "tutorial-controls",
    content: <TutorialControlsContent />,
    className: "aspect-auto",
    order: 100,
  });
  return () => removeFooterItem("tutorial-controls");
}, [addFooterItem, removeFooterItem]);
```

**Refactor Opportunity:**
- Create `useFooterItem(config)` hook that handles cleanup automatically

---

### 6. Consistency Issues

#### 6.1 Empty Constant Naming

**Location:** Throughout codebase

**Pattern:** Inconsistent naming for empty/default constants:
```typescript
// Type.tsx
const EMPTY_TYPE_SELECTION: TypeAppSelection = {};
const EMPTY_PANEL_VISIBILITY: PanelVisibility = { ... };
const EMPTY_OTHERS: TypeAppPresenceOther[] = [];
const EMPTY_MODEL_TAG_ARRAY: string[] = [];

// Kit.tsx  
const emptyKitAppSelection: KitAppSelection = {};  // lowercase

// Design.tsx
// No empty constants defined inline
```

**Issue:** Mix of `SCREAMING_CASE` and `camelCase` for constants.

**Refactor Opportunity:**
- Standardize on `SCREAMING_CASE` for immutable constants
- Extract common empty values to shared.ts

#### 6.2 Scope Provider Patterns

**Location:** Various app files

**Pattern:** Different apps use different scope access patterns:
```typescript
// Type.tsx
const kitScope = useKitScope();
const typeScope = useTypeScope();
const kitGuid = kitScope?.guid ?? "";

// Home.tsx
// No scope providers used

// Design.tsx
const kitScope = useKitScope();
const designScope = useDesignScope();
```

**Issue:** Null coalescing to empty string may hide bugs.

**Refactor Opportunity:**
- Create `useRequiredScope(scope)` that throws if undefined in dev mode
- Create `useOptionalScope(scope)` that returns typed undefined

#### 6.3 Import Organization

**Location:** All files

**Pattern:** Imports follow inconsistent ordering:
```typescript
// Some files: React first
import React, { FC, memo, useCallback } from "react";
import { useSelector } from "@xstate/react";

// Other files: Third-party first
import { DragEndEvent } from "@dnd-kit/core";
import { useSelector } from "@xstate/react";
import React, { FC } from "react";
```

**Refactor Opportunity:**
- Standardize import order:
  1. React
  2. Third-party libraries (alphabetical)
  3. Internal shared (`../semio`, `./shared`)
  4. Internal app (`./Sketchpad`, `./elements`)
  5. Type imports last

---

### 7. Type Safety Improvements

#### 7.1 Any Type Usage

**Location:** Event handlers, runtime actions

**Pattern:**
```typescript
registerEventHandler("HOME.TOGGLE_PANEL", {
  action: (context: any, event: any) => ({ ... }),
});

registerRuntimeAction("typeInit", (context: any, event: any) => { ... });
```

**Issue:** Loss of type safety in critical state management code.

**Refactor Opportunity:**
- Define typed event interfaces per app:
```typescript
type HomeEvent = 
  | { type: "HOME.TOGGLE_PANEL"; panel: PanelKind }
  | { type: "HOME.SET_SORT"; column: HomeSortColumn; direction: HomeSortDirection }
  | ...;

registerEventHandler<HomeEvent, SketchpadContext>("HOME.TOGGLE_PANEL", { ... });
```

#### 7.2 Generic Constraint Gaps

**Location:** shared.ts, Store classes

**Pattern:**
```typescript
class Store<TState> {
  // No constraint on TState
}

type HookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean];
// T can be any type
```

**Refactor Opportunity:**
- Add constraints: `TState extends object`
- Use branded types for GUIDs: `type KitGuid = Guid & { __brand: 'kit' }`

---

### 8. Performance Patterns

#### 8.1 Selector Memoization

**Location:** All hook files

**Pattern:**
```typescript
const selector = useMemo(
  () => createTypeSelectionSelector(kitGuid, typeGuid),
  [kitGuid, typeGuid]
);
```

**Issue:** New selector created on every scope change, even if values are same.

**Refactor Opportunity:**
- Cache selectors at module level:
```typescript
const selectorCache = new Map<string, Selector>();
const getCachedSelector = (key: string, factory: () => Selector) => {
  if (!selectorCache.has(key)) {
    selectorCache.set(key, factory());
  }
  return selectorCache.get(key)!;
};
```

#### 8.2 Object Creation in Renders

**Location:** Throughout UI components

**Pattern:**
```typescript
<ToggleGroup
  items={[
    { value: Theme.SYSTEM, id: "...", icon: <MonitorIcon /> },
    { value: Theme.LIGHT, id: "...", icon: <SunIcon /> },
    { value: Theme.DARK, id: "...", icon: <MoonIcon /> },
  ]}
/>
```

**Issue:** Array created on every render.

**Refactor Opportunity:**
- Extract to useMemo or module-level constants
- For icon components, use lazy factories

---

## Refactoring Opportunities by File

### Sketchpad.tsx (Highest Priority)

| Line Range | Issue | Refactor |
|------------|-------|----------|
| 1400-2000 | Y.js primitive store duplication | Generic `YPrimitiveStore<T>` |
| 250-700 | Store base class complexity | Extract interfaces, reduce inheritance |
| 820-1000 | Plain store variant duplication | Composition over inheritance |
| 1000-1400 | File provider implementations | Extract to separate module conceptually |

### Type.tsx

| Line Range | Issue | Refactor |
|------------|-------|----------|
| 140-350 | 30+ runtime action handlers | Event handler consolidation |
| 350-500 | Triadic hook boilerplate | Hook factory extraction |
| Selection types | Parallel diff structures | Generic diff types |

### Design.tsx

| Line Range | Issue | Refactor |
|------------|-------|----------|
| 1-400 | Selection/diff type definitions | Shared generic types |
| Similar to Type.tsx | Runtime action handlers | Same consolidation |

### Kit.tsx

| Line Range | Issue | Refactor |
|------------|-------|----------|
| 130-400 | Selection diff types (9 interfaces) | Generic ArrayDiff |
| 400-500 | KitStore class | Extract base patterns |
| inverse function | Manual property iteration | Generic inverse utility |

### Home.tsx

| Line Range | Issue | Refactor |
|------------|-------|----------|
| 50-170 | Well-structured event handlers | Reference pattern |
| Panel content | Duplicated settings UI | Extract `<SettingsContent />` shared |

### shared.ts

| Line Range | Issue | Refactor |
|------------|-------|----------|
| 1500-1724 | Multiple registry patterns | Unified registry factory |
| 1300-1500 | DerivedStore complexity | Simplify API |
| Type definitions | Spread across regions | Consolidate by domain |

---

## Cross-Cutting Concerns

### 1. Registry System Unification

**Current State:** 6 separate registries:
- App plugin registry
- Runtime action registry  
- Event handler registry
- Store factory registries (4 separate)
- App hooks registry
- Guard registry

**Recommendation:** Create unified registry factory:
```typescript
const createRegistry = <T>() => ({
  register: (key: string, value: T) => void,
  unregister: (key: string) => void,
  get: (key: string) => T | undefined,
  getAll: () => Map<string, T>,
});
```

### 2. Command System Consistency

**Current State:** Commands use origin strings manually:
```typescript
executeCommand("semio.designApp.deleteSelected", "semio.sketchpad.app.design.toolbar.delete", ...args);
```

**Recommendation:** Type-safe command system:
```typescript
type Command<TArgs extends any[], TResult> = {
  id: string;
  execute: (origin: string, ...args: TArgs) => TResult;
};

const deleteSelectedCommand: Command<[], void> = {
  id: "semio.designApp.deleteSelected",
  execute: (origin) => { ... },
};
```

### 3. Panel System Standardization

**Current State:** Each app manually defines panels with varying structures.

**Recommendation:** Declarative panel configuration:
```typescript
const designAppPanels: PanelConfig[] = [
  { kind: PanelKind.TOOLBAR, sections: [...] },
  { kind: PanelKind.DETAILS, sections: [...] },
];
```

---

## Priority Matrix

### High Priority (Immediate Impact)

| Item | Files Affected | Lines Saved | Complexity |
|------|----------------|-------------|------------|
| Generic ArrayDiff types | 4 | ~200 | Low |
| Event handler consolidation | 6 | ~400 | Medium |
| Triadic hook factory | 4 | ~600 | Medium |
| Transaction handler factory | 3 | ~180 | Low |

### Medium Priority (Maintainability)

| Item | Files Affected | Lines Saved | Complexity |
|------|----------------|-------------|------------|
| Y.js primitive store generic | 1 | ~400 | Medium |
| Selector caching | 4 | ~100 | Low |
| Registry unification | 1 | ~200 | High |
| Type-safe events | 6 | ~0 | High |

### Low Priority (Polish)

| Item | Files Affected | Lines Saved | Complexity |
|------|----------------|-------------|------------|
| Constant naming | All | ~0 | Low |
| Import ordering | All | ~0 | Low |
| Empty value extraction | 4 | ~50 | Low |

---

## Summary

The sketchpad codebase is well-organized with consistent patterns but contains significant opportunities for abstraction and deduplication. The highest-impact refactors are:

1. **Create generic diff/selection types** - Eliminates ~200 lines of parallel interface definitions
2. **Consolidate event handlers** - Migrates 80+ handlers to unified pattern
3. **Extract hook factories** - Reduces 40+ hooks from 20 lines each to 6 lines
4. **Unify transaction management** - Eliminates 180+ lines of duplicated handlers

Total potential reduction: **~1,500-2,000 lines** while improving type safety and maintainability.
