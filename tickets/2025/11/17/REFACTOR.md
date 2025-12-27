---
slug: REFACTOR
summary: Migration from REFACTOR.md
prompt: Migration from REFACTOR.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.669Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Sketchpad Apps Refactoring Proposal

## Executive Summary

This document analyzes the current architecture of the Sketchpad app system and proposes a comprehensive refactoring to address inconsistencies, improve maintainability, and enforce the Open-Closed Principle more rigorously.

## Current State Analysis

### App Inventory

| App         | Store Base Class           | Lines | Store Registration                  | Config Export | Regions                                                             |
| ----------- | -------------------------- | ----- | ----------------------------------- | ------------- | ------------------------------------------------------------------- |
| **design**  | `KitDiffAppStore`          | 6,751 | ✅ `registerDesignAppStoreFactory`  | ✅            | Store, Commands, Panels, Tools, Canvas, Footer, Config              |
| **type**    | `KitDiffAppStore`          | 3,184 | ✅ `registerTypeAppStoreFactory`    | ✅            | Store, Commands, Panels, Tools, App, Footer, Config                 |
| **kit**     | `KitDiffAppStore`          | 4,299 | ✅ `registerKitAppStoreFactory`     | ✅            | Store, Commands, Navbar, Canvas, Panels, Tools, Footer, App, Config |
| **quality** | `KitDiffAppStore`          | 1,781 | ✅ `registerQualityAppStoreFactory` | ✅            | Commands, Store, App, Config                                        |
| **home**    | Custom `HomeStore`         | 1,652 | ✅ `registerHomeStoreFactory`       | ✅            | Store, Commands, Navbar, Canvas, Panels, Tools, Footer, App, Config |
| **docs**    | Placeholder `DocsAppStore` | 1,500 | ❌ None                             | ✅            | MDX Loader, MDX Provider, Registry, Store, Commands                 |

### Key Inconsistencies Identified

#### 1. **Store Architecture Inconsistencies**

**Problem:** Apps use different store base classes without clear rationale:

- **design, type, kit, quality**: Extend `KitDiffAppStore` ✅
- **home**: Custom `HomeStore` (does NOT extend `AppStore`) ❌
- **docs**: Placeholder `DocsAppStore` (no functionality) ❌

**Impact:**

- `HomeStore` reimplements functionality from `AppStore` (transaction management, undo/redo, selection)
- No transaction support for home app
- Inconsistent command patterns
- `DocsAppStore` is essentially empty, yet the app works

**Root Cause:** `HomeStore` predates the `AppStore` abstraction and was never migrated.

#### 2. **Store Registration Inconsistencies**

**Problem:** Inconsistent factory registration patterns:

```typescript
// Design, Type, Quality: Inline registration
registerDesignAppStoreFactory((parent, yMap, transact, id, state) => new DesignAppStore(parent, yMap, transact, id, state));

// Kit, Type: Separate initialization function
export function initializeKitAppStore() {
  registerKitAppStoreFactory((parent, yMap, transact, id, state) => new KitAppStore(parent, yMap, transact, id, state));
}
if (typeof window !== "undefined") {
  setTimeout(() => initializeKitAppStore(), 0);
}

// Home: Direct registration
registerHomeStoreFactory((parent, yMap, transact) => new HomeStore(parent, yMap, transact));

// Docs: No registration at all
```

**Impact:**

- Confusing initialization order
- Timing issues with circular dependencies
- Different behavior in SSR vs. browser contexts

#### 3. **Region Organization Inconsistencies**

**Problem:** Apps organize regions differently:

| App         | Region Order                                                                          |
| ----------- | ------------------------------------------------------------------------------------- |
| **design**  | Header, Commands, Store, Imports → Footer, Tools, Panels, Canvas, Config              |
| **type**    | Header, Imports, Store, Commands → Panels, Tools, App, Footer, Config                 |
| **kit**     | Header, Imports, Store, Commands → Navbar, Canvas, Panels, Tools, Footer, App, Config |
| **quality** | Header, Imports, Types, Functions, Commands, Store → App, Config                      |
| **home**    | Header, Imports, Store, Commands → Navbar, Canvas, Panels, Tools, Footer, App, Config |
| **docs**    | Header, Imports, MDX Loader, MDX Provider, Registry, Store, Commands → (mixed)        |

**Impact:**

- Difficult to navigate codebase
- Hard to find corresponding sections across apps
- Violates "TOOLFRIENDLY over intuitive" principle

#### 4. **Import Statement Positioning**

**Problem:** Imports appear in different positions:

- **design, kit**: Store region BEFORE Imports region
- **type, quality, home, docs**: Imports region BEFORE Store region

**Why it matters:** The "Store region before Imports" pattern in design/kit includes type imports needed for the Store interfaces, breaking the clean separation.

#### 5. **Y.js Type Declarations**

**Problem:** Inconsistent placement of Y.js type aliases:

```typescript
// Design, Kit, Type, Quality: In Store region
type YDesignAppVal = string | number | boolean | ...;
type YDesignApp = Y.Map<YDesignAppVal>;
type YDesignApps = Y.Map<Y.Map<YDesignApp>>;

// Home: In Store region (correct)
// Docs: Missing entirely
```

**Impact:** Should these be in a separate "Types" region? Or always in Store?

#### 6. **Tool System Inconsistencies**

**Problem:** Only some apps have tool systems:

| App         | Has Tools? | Tool Implementation           |
| ----------- | ---------- | ----------------------------- |
| **design**  | ✅         | `*Tool.tsx` files auto-loaded |
| **type**    | ✅         | `*Tool.tsx` files auto-loaded |
| **quality** | ✅         | `*Tool.tsx` files auto-loaded |
| **kit**     | ❌         | Empty Tools region            |
| **home**    | ❌         | Empty Tools region            |
| **docs**    | ❌         | No Tools region               |

**Impact:** Unclear when an app should have tools vs. not.

#### 7. **Lazy Loading Inconsistencies**

**Problem:** Some apps lazy-load cross-app dependencies, others don't:

```typescript
// Design, Type: Lazy load KitSection from kit app
const KitSectionLazy = React.lazy(async () => {
  const module = await import("../kit/App");
  return { default: module.KitSection };
});

// Quality: No lazy loading
// Kit: N/A (other apps depend on it)
// Home: No cross-app dependencies
// Docs: No cross-app dependencies
```

**Impact:** Potential circular dependency issues when apps reference each other.

#### 8. **Panel System Inconsistencies**

**Problem:** Different apps register panels differently:

- **All apps**: Define panels in `config.getPanels(t)`
- **Some apps**: Also register panel sections programmatically via `useAddPanelSection`
- **Inconsistent panel keys**: Some use strings, some use enums

#### 9. **Scope Provider Patterns**

**Problem:** Inconsistent context provider patterns:

```typescript
// Design: Uses useDesignScope() from App.tsx
// Type: Uses useTypeScope() from App.tsx
// Quality: Uses useQualityScope() from App.tsx
// Kit: Uses useKitScope() from App.tsx
// Home: No scope (singleton app)
// Docs: No scope (singleton app)
```

But scope providers are defined inconsistently:

- Some apps: Provider in App.tsx (DesignScopeProvider, TypeScopeProvider)
- Some apps: Provider in app file (QualityAppScopeProvider, TypeAppScopeProvider)

#### 10. **Command System Inconsistencies**

**Problem:** Command definitions vary:

- **Design, Type, Kit, Quality**: Commands defined in dedicated region with full type safety
- **Home**: Minimal command system (selection only)
- **Docs**: Placeholder commands (not functional)

#### 11. **File Size Disparities**

**Problem:** Massive variation in app file sizes:

- **design**: 6,751 lines (largest)
- **kit**: 4,299 lines
- **type**: 3,184 lines
- **quality**: 1,781 lines
- **home**: 1,652 lines
- **docs**: 1,500 lines

**Impact:**

- Design app is too large (4x larger than docs)
- Violates "NEVER create new files" rule but makes navigation difficult
- Suggests design app needs internal reorganization

#### 12. **Missing Features**

**Problem:** Some apps lack features that others have:

| Feature      | design | type | kit | quality | home | docs |
| ------------ | ------ | ---- | --- | ------- | ---- | ---- |
| Undo/Redo    | ✅     | ✅   | ✅  | ✅      | ❌   | ❌   |
| Transactions | ✅     | ✅   | ✅  | ✅      | ❌   | ❌   |
| KitDiff      | ✅     | ✅   | ✅  | ✅      | ❌   | ❌   |
| Selection    | ✅     | ✅   | ✅  | ✅      | ✅   | ✅   |
| Fullscreen   | ✅     | ✅   | ✅  | ✅      | ❌   | ❌   |
| Tools        | ✅     | ✅   | ❌  | ✅      | ❌   | ❌   |
| Hover State  | ✅     | ✅   | ✅  | ✅      | ❌   | ❌   |
| Camera       | ✅     | ✅   | ❌  | ❌      | ❌   | ❌   |

---

## Proposed Refactoring

### Phase 1: Store Architecture Standardization

#### 1.1. Migrate HomeStore to AppStore

**Goal:** Make `HomeStore` extend `AppStore<...>` for consistency.

**Changes:**

```typescript
// OLD: js/js/sketchpad/apps/home/App.tsx
export class HomeStore {
  // Custom implementation
}

// NEW: js/js/sketchpad/apps/home/App.tsx
export class HomeStore extends AppStore<HomeState, HomeDiff, HomeSelectionDiff, HomeEdit, HomeCommandContext, HomeCommandResult> {
  // Inherit transaction, undo/redo, selection from AppStore
}

export interface HomeEdit extends AppEdit<HomeSelectionDiff> {}
```

**Benefits:**

- Transaction support for home app
- Consistent undo/redo behavior
- Unified command pattern
- Selection diff-based updates

**Risks:**

- May require Y.js migration for home state
- Could affect existing home app behavior

#### 1.2. Decide on DocsAppStore

**Options:**

**Option A: Full AppStore Migration** (Recommended)

```typescript
export class DocsAppStore extends AppStore<DocsState, DocsDiff, DocsSelectionDiff, DocsEdit, DocsCommandContext, DocsCommandResult> {
  // Full implementation
}
```

**Option B: Keep Minimal** (If docs truly needs no state persistence)

```typescript
// Remove DocsAppStore entirely
// Use local React state only
```

**Recommendation:** Option A if docs should persist state (current page, progress), Option B if purely navigation-focused.

#### 1.3. Standardize Store Base Classes

**Rule:** All app stores MUST extend one of:

1. **`AppStore`** - For apps without kit modification (home, docs)
2. **`KitDiffAppStore`** - For apps that modify kits (design, type, kit, quality)

**Exception:** None. No custom store base classes allowed.

### Phase 2: Region Organization Standardization

#### 2.1. Define Canonical Region Order

**Mandatory region order for ALL apps:**

```typescript
// #region Header
//   License, copyright, etc.
// #endregion

// #region Imports
//   All imports, including React, libraries, and internal
// #endregion

// #region Types
//   Y.js type aliases (YAppVal, YApp, YApps)
//   Enums
//   Interface definitions (State, Selection, Diff, Edit, Context, Result, etc.)
// #endregion

// #region Store
//   Store class definition
//   Store registration
//   Hooks (useStore, useState, useCommands)
//   Scope provider (if applicable)
// #endregion

// #region Commands
//   Command implementations
//   Helper functions for commands
// #endregion

// #region Components
//   // #region Navbar
//   // #endregion
//
//   // #region Canvas
//     // #region Windows
//       // #region Scene
//       // #endregion
//       // #region Diagram
//       // #endregion
//       // #region Table
//       // #endregion
//     // #endregion Windows
//   // #endregion Canvas
//
//   // #region Panels
//     // #region Left
//     // #endregion
//     // #region Right
//       // #region Details
//       // #endregion
//       // #region Chat
//       // #endregion
//       // #region Settings
//       // #endregion
//     // #endregion Right
//     // #region Bottom
//     // #endregion
//   // #endregion Panels
//
//   // #region Tools
//   // #endregion
//
//   // #region Footer
//   // #endregion
// #endregion Components

// #region App
//   Main App component
// #endregion

// #region Config
//   export const config: AppConfig
// #endregion
```

**Benefits:**

- Predictable navigation across all apps
- Easy to find corresponding sections
- Tool-friendly structure
- Clear hierarchy

#### 2.2. Apply to All Apps

**Action Items:**

1. **Reorder design/App.tsx** (currently: Store before Imports)
2. **Reorder kit/App.tsx** (currently: Store before Imports)
3. **Add missing regions** where empty (e.g., Tools in kit/home)
4. **Nest component regions** under Components parent region
5. **Extract Types** from Store regions into separate Types region

### Phase 3: Store Registration Standardization

#### 3.1. Unified Registration Pattern

**Rule:** All stores MUST use the same registration pattern:

```typescript
// #region Store

// ... Store class definition ...

// Immediate inline registration (no function wrapper)
if (typeof window !== "undefined") {
  register[App]StoreFactory((parent, yMap, transact, id?, state?) =>
    new [App]Store(parent, yMap, transact, id, state)
  );
}

// #endregion Store
```

**No more:**

- ❌ `export function initialize[App]Store()` wrapper functions
- ❌ `setTimeout(() => initialize...(), 0)` delayed initialization
- ❌ Separate registration outside the Store region

**Benefits:**

- Consistent initialization timing
- No circular dependency workarounds needed
- Clear where registration happens

### Phase 4: Tool System Standardization

#### 4.1. Define Tool System Rules

**Rule:** An app MUST have a Tools region if ANY of the following are true:

1. App has multiple ways to interact with canvas/diagram
2. App has mode-switching behavior (select, add, edit, etc.)
3. App uses tool-specific cursors or UI

**Current assessment:**

| App         | Needs Tools? | Why?                                               |
| ----------- | ------------ | -------------------------------------------------- |
| **design**  | ✅ Yes       | Multiple interaction modes (select, connect, etc.) |
| **type**    | ✅ Yes       | Connector placement, model selection               |
| **kit**     | ✅ Yes       | Artifact creation/editing modes                    |
| **quality** | ✅ Yes       | Formula editing modes                              |
| **home**    | ❌ No        | Single interaction mode (select kits)              |
| **docs**    | ❌ No        | Read-only navigation                               |

**Action Items:**

1. **Add Tools region to kit app** with tool definitions
2. **Keep empty Tools region in home app** (for future extensibility)
3. **Document tool auto-loading pattern** in all apps that use it

#### 4.2. Standardize Tool File Naming

**Rule:** Tool files MUST follow the pattern `*Tool.tsx`:

```
apps/
  design/
    App.tsx
    SelectTool.tsx
    ConnectTool.tsx
    PanTool.tsx
  type/
    App.tsx
    SelectTool.tsx
    PlacePortTool.tsx
```

**Auto-loading pattern (consistent across all apps):**

```typescript
// In App.tsx Tools region
const toolModules = import.meta.glob<{ default: Tool<AppState> }>("./*Tool.tsx", {
  eager: true,
});

const tools = Object.values(toolModules).map((m) => m.default);
```

### Phase 5: Lazy Loading Standardization

#### 5.1. Centralized Lazy Load Module Cache

**Problem:** Currently each app maintains its own module cache:

```typescript
// design/App.tsx
let kitAppModuleCache: any = null;
if (typeof window !== "undefined" && (window as any).__KIT_APP_MODULE_CACHE__) {
  kitAppModuleCache = (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache;
}
```

**Solution:** Centralize in App.tsx:

```typescript
// js/js/sketchpad/App.tsx

// #region Module Cache
const MODULE_CACHE = {
  design: null as any,
  type: null as any,
  kit: null as any,
  quality: null as any,
  home: null as any,
  docs: null as any,
};

if (typeof window !== "undefined") {
  (window as any).__SKETCHPAD_MODULE_CACHE__ = MODULE_CACHE;
}

export function getLazyModule(appName: keyof typeof MODULE_CACHE) {
  if (!MODULE_CACHE[appName]) {
    throw new Error(`Module ${appName} not loaded yet`);
  }
  return MODULE_CACHE[appName];
}

export function setLazyModule(appName: keyof typeof MODULE_CACHE, module: any) {
  MODULE_CACHE[appName] = module;
  if (typeof window !== "undefined") {
    (window as any).__SKETCHPAD_MODULE_CACHE__ = MODULE_CACHE;
  }
}
// #endregion Module Cache
```

**Then in apps:**

```typescript
// design/App.tsx
const KitSectionLazy = React.lazy(async () => {
  const module = await import("../kit/App");
  setLazyModule("kit", module);
  return { default: module.KitSection };
});
```

### Phase 6: Scope Provider Standardization

#### 6.1. Unified Scope Provider Pattern

**Rule:** All scope providers MUST be defined in the app file, not App.tsx:

```typescript
// apps/[app]/App.tsx

// #region Store

// ... Store definition ...

// Scope provider (if app supports multiple instances)
const [App]ScopeContext = createContext<{ guid: string } | undefined>(undefined);

export const [App]ScopeProvider: FC<{ guid: string; children: ReactNode }> =
  ({ guid, children }) => {
    const value = useMemo(() => ({ guid }), [guid]);
    return <[App]ScopeContext.Provider value={value}>{children}</[App]ScopeContext.Provider>;
  };

export const use[App]Scope = () => useContext([App]ScopeContext);

// #endregion Store
```

**Action Items:**

1. Move `DesignScopeProvider`, `TypeScopeProvider`, etc. from App.tsx to respective app files
2. Remove scope exports from App.tsx (breaking change, update imports)
3. Keep `useDesignScope()`, `useTypeScope()` as re-exports in App.tsx for convenience

### Phase 7: Command System Standardization

#### 7.1. Mandatory Command Structure

**Rule:** ALL apps (even home and docs) MUST define commands in Commands region:

```typescript
// #region Commands

export const [app]Commands = {
  "semio.[app].command": async (
    context: [App]CommandContext,
    ...args: any[]
  ): Promise<[App]CommandResult> => {
    // Implementation
    return { diff: { ... } };
  },
};

// #endregion Commands
```

**Even for simple apps:**

```typescript
// home/App.tsx
export const homeCommands = {
  "semio.home.selectKit": async (context: HomeCommandContext, kitId: Guid): Promise<HomeCommandResult> => {
    return {
      diff: {
        selection: {
          added: [kitId],
        },
      },
    };
  },
};
```

### Phase 8: Panel System Standardization

#### 8.1. Unified Panel Registration

**Current state:** Panels defined in `config.getPanels()` but sections added programmatically.

**Proposed:** Keep both patterns but make explicit:

```typescript
// #region Config

export const config: AppConfig = {
  // ...
  getPanels: (t) => [
    // Static panel definitions (chrome, visibility toggles)
    {
      key: "details",
      icon: Info,
      tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.details.show" },
      position: PanelPosition.RIGHT,
      group: "right",
      isGroupable: true
    },
  ],
};

// #endregion Config

// #region Components
// #region Panels
// #region Right
// #region Details

const DetailsSection: FC = () => {
  const { addSection, removeSection } = useAddPanelSection();

  useEffect(() => {
    // Dynamic section registration (content)
    addSection("details", {
      id: "kit-details",
      label: "Kit Details",
      content: () => <KitDetailsComponent />,
      order: 1,
    });
    return () => removeSection("details", "kit-details");
  }, []);

  return null; // Section registration only
};

// #endregion Details
// #endregion Right
// #endregion Panels
// #endregion Components
```

**Rule:**

- **Panel definition** (chrome): `config.getPanels()`
- **Section registration** (content): `useAddPanelSection()` in useEffect

### Phase 9: File Size Management

#### 9.1. Internal Subregions for Large Apps

**Problem:** Design app is 6,751 lines, violating navigability.

**Solution:** Use more granular subregions:

```typescript
// #region Components

// #region Navbar
// #endregion Navbar

// #region Canvas

  // #region Windows

    // #region Scene

      // #region Scene Components
        // #region PieceComponent
        const PieceComponent: FC<...> = ...
        // #endregion PieceComponent

        // #region ConnectionComponent
        const ConnectionComponent: FC<...> = ...
        // #endregion ConnectionComponent
      // #endregion Scene Components

    // #endregion Scene

  // #endregion Windows

// #endregion Canvas

// #endregion Components
```

**Benefits:**

- Still single file (adheres to "NEVER create new files")
- Better navigation via nested regions
- Clear component hierarchy

### Phase 10: Documentation & Enforcement

#### 10.1. Update AGENTS.md

Add new section to AGENTS.md:

```markdown
### App Structure Standards

All apps in `js/js/sketchpad/apps/*/App.tsx` MUST follow this structure:

1. **Region Order:** Header → Imports → Types → Store → Commands → Components → App → Config
2. **Store Base Class:** MUST extend either `AppStore` or `KitDiffAppStore` (no custom base classes)
3. **Store Registration:** MUST use inline registration pattern (no wrapper functions)
4. **Component Regions:** MUST nest under Components region (Navbar, Canvas, Panels, Tools, Footer)
5. **Tools:** MUST have Tools region if app has multiple interaction modes
6. **Scope Providers:** MUST be defined in app file (not App.tsx)
7. **Commands:** MUST define all commands in Commands region

See `REFACTOR.md` for detailed rationale and migration guide.
```

#### 10.2. Create Migration Checklist

**Per-app checklist:**

```markdown
## [App Name] Migration Checklist

- [ ] Store extends correct base class (AppStore or KitDiffAppStore)
- [ ] Store registration uses inline pattern
- [ ] Regions in correct order (Header, Imports, Types, Store, Commands, Components, App, Config)
- [ ] Component regions nested under Components parent
- [ ] Tools region present (even if empty)
- [ ] Scope provider in app file (if applicable)
- [ ] Commands defined in Commands region
- [ ] Panel sections registered in useEffect
- [ ] No direct Y.js manipulation in components (use store)
- [ ] Lazy module loading uses centralized cache (if applicable)
```

---

## Migration Strategy

### Phased Approach

**Phase 1: Foundation (Week 1)**

1. Centralize module cache in App.tsx
2. Standardize store registration pattern
3. Update AGENTS.md with new standards

**Phase 2: Store Architecture (Week 2)**

1. Migrate HomeStore to AppStore
2. Decide on DocsAppStore approach
3. Test undo/redo in all apps

**Phase 3: Region Reorganization (Week 2-3)**

1. Reorder regions in all apps
2. Add missing empty regions
3. Nest component regions

**Phase 4: Tool & Panel Systems (Week 3)**

1. Add Tools region to kit app
2. Standardize tool file naming
3. Clarify panel registration pattern

**Phase 5: Cleanup & Documentation (Week 4)**

1. Remove deprecated patterns
2. Update migration checklist
3. Validate all apps against standards

### Risk Mitigation

**High Risk Changes:**

1. **HomeStore migration** - Could break existing functionality
   - **Mitigation:** Feature flags, incremental rollout
2. **Region reordering** - Large diffs, merge conflicts
   - **Mitigation:** One app at a time, coordinate with team

**Low Risk Changes:**

1. **Registration pattern** - Internal only
2. **Region organization** - No functional changes
3. **Documentation updates** - Zero risk

---

## Breaking Changes

### For Internal Development

1. **Import paths change** for scope providers:

   ```typescript
   // OLD
   import { useDesignScope } from "../../App";

   // NEW
   import { useDesignScope } from "../design/App";
   ```

2. **Store initialization** may have different timing:
   - Commands executed before full initialization will fail differently
   - Mitigation: Better error messages

### For External Consumers

None. All changes are internal to sketchpad apps.

---

## Open Questions

1. **Should DocsAppStore extend AppStore?**
   - Depends on whether docs needs state persistence
   - Current minimal implementation suggests no
   - But future features (progress tracking, bookmarks) suggest yes

2. **Should we split design/App.tsx into multiple files?**
   - Current: 6,751 lines in one file
   - AGENTS.md says "NEVER create new files"
   - But navigation is difficult
   - **Proposal:** Use more granular subregions instead

3. **Should we create a base test suite for app conformance?**
   - Could validate:
     - Store extends correct base class
     - All required regions present
     - Config exports correctly
   - Would catch deviations early

4. **Should Tools be optional or mandatory?**
   - Current proposal: Always include region (even if empty)
   - Alternative: Only include if app has tools
   - **Recommendation:** Always include for consistency

5. **Should we version the app structure?**
   - e.g., `@version 2.0` comment in header
   - Would make it clear which apps follow new standards
   - Could help with gradual migration

---

## Success Metrics

### Quantitative

1. **Region consistency:** 100% of apps follow canonical region order
2. **Store inheritance:** 100% of apps extend AppStore or KitDiffAppStore
3. **Registration pattern:** 100% of apps use inline registration
4. **Code navigation:** <5 seconds to find corresponding section across apps

### Qualitative

1. **Developer experience:** New developers can understand app structure within 30 minutes
2. **Maintenance:** Adding new features follows clear patterns
3. **Refactoring:** Cross-app changes are straightforward
4. **Documentation:** AGENTS.md accurately describes all apps

---

## Alternatives Considered

### Alternative 1: Separate Files Per Region

**Approach:**

```
apps/
  design/
    App.tsx           # Exports only
    store.ts          # Store class
    commands.ts       # Commands
    components/       # All components
      Navbar.tsx
      Canvas.tsx
      Panels.tsx
    config.ts         # Config
```

**Pros:**

- Smaller files, easier to navigate
- Clear separation of concerns
- Better for code splitting

**Cons:**

- Violates "NEVER create new files" rule in AGENTS.md
- More files to manage
- Import boilerplate increases

**Decision:** Rejected due to AGENTS.md constraint.

### Alternative 2: Keep Current Structure

**Approach:** Document inconsistencies but don't refactor.

**Pros:**

- No migration risk
- No breaking changes
- Apps work today

**Cons:**

- Technical debt accumulates
- New developers confused
- Maintenance burden increases
- Violates Open-Closed Principle

**Decision:** Rejected. Debt must be addressed.

### Alternative 3: Gradual Convergence

**Approach:** Update standards but let apps converge gradually.

**Pros:**

- Lower risk
- No forced timeline
- Teams self-organize

**Cons:**

- May never fully converge
- Inconsistency persists
- Unclear ownership

**Decision:** Partial adoption. Use phased approach but with clear timeline.

---

## Conclusion

This refactoring proposal addresses systematic inconsistencies in the Sketchpad app architecture. By standardizing store base classes, region organization, registration patterns, and component structure, we can:

1. **Improve maintainability** - Clear patterns, easy navigation
2. **Reduce technical debt** - Eliminate custom store implementations
3. **Enhance developer experience** - Predictable structure across apps
4. **Enable future features** - Transactions, undo/redo for all apps
5. **Enforce architecture** - Open-Closed Principle via documentation

**Recommendation:** Proceed with phased migration starting with foundation (Phase 1) and store architecture (Phase 2), then evaluate before continuing.

**Timeline:** 4 weeks for complete migration, with checkpoints after each phase.

**Owner:** Core team (with review from app maintainers).

---

## Appendix A: Per-App Detailed Analysis

### A.1 Design App (6,751 lines)

**Current Problems:**

- Store region BEFORE Imports region (reversed)
- Extremely large file (needs internal subregions)
- Commands defined AFTER Store (inconsistent with others)

**Migration Tasks:**

1. Move Imports region to top
2. Extract Types from Store into separate region
3. Add more granular subregions within Components
4. Reorder Commands region to standard position

**Estimated Effort:** 8 hours (large file)

### A.2 Type App (3,184 lines)

**Current Problems:**

- Lazy loading pattern needs centralization
- Scope provider in app file (correct but inconsistent with design)

**Migration Tasks:**

1. Use centralized module cache
2. Add Types region
3. Verify region order

**Estimated Effort:** 2 hours

### A.3 Kit App (4,299 lines)

**Current Problems:**

- Store region BEFORE Imports region (reversed)
- Empty Tools region (should have tools?)
- Initialization function wrapper (should be inline)

**Migration Tasks:**

1. Move Imports region to top
2. Inline store registration
3. Add tool definitions or document why none needed
4. Add Types region

**Estimated Effort:** 4 hours

### A.4 Quality App (1,781 lines)

**Current Problems:**

- Regions not in standard order (Commands before Store)
- Missing some component subregions

**Migration Tasks:**

1. Reorder regions to standard
2. Add Types region
3. Add missing component subregions

**Estimated Effort:** 2 hours

### A.5 Home App (1,652 lines)

**Current Problems:**

- HomeStore doesn't extend AppStore (major architectural issue)
- No transaction support
- Manual undo/redo implementation (if any)

**Migration Tasks:**

1. Migrate HomeStore to extend AppStore
2. Add Y.js persistence for home state
3. Add transaction support
4. Test undo/redo
5. Add Types region
6. Define HomeEdit interface

**Estimated Effort:** 12 hours (significant refactor)

### A.6 Docs App (1,500 lines)

**Current Problems:**

- DocsAppStore is placeholder (no functionality)
- Mixed region organization
- Unclear if state persistence needed

**Migration Tasks:**

1. Decide: Full AppStore migration vs. remove store entirely
2. If keeping: Implement full store with state persistence
3. If removing: Use local React state only
4. Standardize region order
5. Add Types region

**Estimated Effort:** 6 hours (decision + implementation)

---

## Appendix B: Code Examples

### B.1 Before/After: HomeStore Migration

**Before:**

```typescript
export class HomeStore {
  public readonly guid: string;
  public readonly parent: SketchpadStore;
  public readonly yMap: Y.Map<any>;
  protected readonly commandRegistry: Map<string, Function> = new Map();
  protected readonly transact: (fn: () => void) => void;
  protected cache?: HomeState;

  // Manual cache invalidation
  hash(state: HomeState): string { ... }

  // No transaction support
  // No undo/redo
  // Manual selection management
}
```

**After:**

```typescript
export interface HomeEdit extends AppEdit<HomeSelectionDiff> {}

export class HomeStore extends AppStore<
  HomeState,
  HomeDiff,
  HomeSelectionDiff,
  HomeEdit,
  HomeCommandContext,
  HomeCommandResult
> {
  // Inherits:
  // - hash() from Store
  // - transaction management from AppStore
  // - undo/redo stacks from AppStore
  // - selection diff application from AppStore

  protected buildSnapshot(): HomeState { ... }

  protected applySelectionDiff(diff: HomeSelectionDiff): void { ... }

  protected inverseSelectionDiff(
    selection: HomeSelection,
    diff: HomeSelectionDiff
  ): HomeSelectionDiff { ... }

  protected getSelection(): HomeSelection { ... }
}
```

### B.2 Before/After: Region Organization

**Before (design/App.tsx):**

```typescript
// #region Header
// ...
// #endregion

// #region Commands
// (empty, defined later)
// #endregion

// #region Store
import * as Y from "yjs";
import { ... } from "../../App";
// ... types and store class
// #endregion

// #region Imports
import { DragEndEvent, ... } from "@dnd-kit/core";
import { BarChart3, ... } from "lucide-react";
// ... more imports
// #endregion

// ... rest of file

// #region Commands
// (actual commands here)
// #endregion
```

**After (design/App.tsx):**

```typescript
// #region Header
// ...
// #endregion

// #region Imports
import * as Y from "yjs";
import { DragEndEvent, ... } from "@dnd-kit/core";
import { BarChart3, ... } from "lucide-react";
import { ... } from "../../App";
import { ... } from "../../elements";
import { ... } from "../../../semio";
// #endregion Imports

// #region Types
type YDesignAppVal = string | number | boolean | ...;
type YDesignApp = Y.Map<YDesignAppVal>;
type YDesignApps = Y.Map<Y.Map<YDesignApp>>;

export interface DesignAppSelection { ... }
export interface DesignAppSelectionDiff { ... }
export interface DesignAppDiff { ... }
export interface DesignAppEdit extends KitDiffAppEdit<...> {}
export interface DesignAppState { ... }
export interface DesignAppCommandContext extends KitCommandContext { ... }
export interface DesignAppCommandResult { ... }
// #endregion Types

// #region Store
export class DesignAppStore extends KitDiffAppStore<...> { ... }

if (typeof window !== "undefined") {
  registerDesignAppStoreFactory((parent, yMap, transact, id, state) =>
    new DesignAppStore(parent, yMap, transact, id, state)
  );
}

export function useDesignAppStore<T>(...): T | DesignAppStore | null { ... }
export function useDesignApp<T>(...): T | DesignAppState | null { ... }
// ... other hooks
// #endregion Store

// #region Commands
export const designCommands = {
  "semio.designApp.selectPiece": async (context, pieceId) => { ... },
  // ... more commands
};
// #endregion Commands

// #region Components
// ... all components
// #endregion Components

// #region App
const App: FC = () => { ... };
export default App;
// #endregion App

// #region Config
export const config: AppConfig = { ... };
// #endregion Config
```

### B.3 Before/After: Store Registration

**Before (multiple patterns):**

```typescript
// Design app - inline
registerDesignAppStoreFactory((parent, yMap, transact, id, state) => new DesignAppStore(parent, yMap, transact, id, state));

// Kit app - wrapper function
export function initializeKitAppStore() {
  registerKitAppStoreFactory((parent, yMap, transact, id, state) => new KitAppStore(parent, yMap, transact, id, state));
}
if (typeof window !== "undefined") {
  setTimeout(() => initializeKitAppStore(), 0);
}

// Home app - direct
registerHomeStoreFactory((parent, yMap, transact) => new HomeStore(parent, yMap, transact));
```

**After (unified pattern):**

```typescript
// ALL apps use this pattern in Store region

// #region Store

export class [App]Store extends [Base]Store<...> {
  // ... implementation
}

// Immediate inline registration (browser only)
if (typeof window !== "undefined") {
  register[App]StoreFactory((parent, yMap, transact, id?, state?) =>
    new [App]Store(parent, yMap, transact, id, state)
  );
}

// Hooks
export function use[App]Store<T>(...) { ... }
export function use[App]<T>(...) { ... }
// ... more hooks

// #endregion Store
```

---

**End of Document**
