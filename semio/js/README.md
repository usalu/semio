# Summary

Domain-only semio JavaScript core with kit schemas, import/export, geometry, graph, and validation logic shared across apps and tooling.

### Policies

- NEVER use inline styling. Use tailwindcss (v4). v4 uses `semio-elements/ui/theme.css` for theming and not `{theme:{…}}` in `tailwindconfig`.
- ALWAYS use colors defined in `@theme inline {…}` from `semio-elements/ui/globals.css`. NEVER use direct colors such as light, gray, …, dark, primary, secondary, tertiary outside of `semio-elements/ui/globals.css` and ALWAYS use semantic colors instead such as active, disabled, hover, …
- Borders use semantic kinds via Tailwind color tokens: `border-element` (hover color) and `border-window` (normal border color).
- GoldenLayout window chrome uses the window background token to match window content surfaces.
- GoldenLayout stack frames use inset strokes so window borders remain continuous on all four sides.
- ALWAYS add tooltips (normal and extensive) to all ui elements.
- ALWAYS load icons via the semantic icon layer in `semio/assets` and NEVER import icons directly from external libraries (lucide, heroicons, .). Only reexport placeholder assets from those libraries inside `semio/assets` and consume them through its semantic exports.

### Styling

- The ui consists of a three horizontal strips: navbar, canvas and footer. A canvas consists of windows. On top of the canvas are panels which can toggled on and off.
- Navbar panel toggles always order panels as Details, Chat, then Settings for every app.

# Docs

The Sketchpad runtime, app configs, i18n, pages, and browser-facing tests now live in `semio/sketchpad`. `semio/js` is limited to reusable domain logic from `semio.ts` plus its domain tests.

## 📁js/semio/

Shared react components. The main component is Sketchpad. Sketchpad is used in three different szenarios:

1. As guest mode (readonly) in a statically generated pages.
2. As user mode in the browser (nextjs).
3. As user mode in a desktop app (electron).
   Sketchpad has a local store in yjs which syncs with indexeddb and the backend provider.

**Policies:**

- Domain logic is ALWAYS in semio.ts and whenever an operation is not ui bound, it should be implemented there.
- **State Management Architecture**: XState is the SINGLE SOURCE OF TRUTH for all UI state. Yjs is ONLY used for collaborative Kit data (types, designs, etc.) via `KitStore`. All other app stores (Design, Type, Quality, Docs, Home, Feedback) use `PlainAppStore` or `PlainKitDiffAppStore` base classes which do NOT use Yjs. React components read state via `useSelector(actor, ...)` and send events via `actor.send({type: ...})`. NO Yjs in React components.
  - `machines.ts` - Unified XState machine with all app state
  - `xstate-hooks.ts` - Clean React hooks using XState selectors
  - State is ALWAYS accessed over hooks. Mutation ALWAYS is via actor events. NEVER use useState for app state.
- **Keyed Initialization Pattern**: App initialization hooks (e.g., `useDesignAppInitialize`, `useTypeAppInitialize`, `useKitAppYjsToXStateSync`) use keyed refs to track initialization scope. Instead of boolean `hasInitialized`, use `initializedKeyRef = useRef<string | null>(null)` with composite keys like `${kitGuid}:${designGuid}` to properly reinitialize when route scope changes.
- **Event Handler Registration**: ONLY use `registerEventHandler` for XState event handling. The legacy `registerRuntimeAction` mechanism exists but MUST NOT be duplicated with `registerEventHandler`. Each event should have exactly ONE registration.
- **Granular Hook Architecture**: All app state hooks follow the `[value, setter, canSet]` tuple pattern:
  - **Pattern**: `const [value, setValue, canSetValue] = useAppValue();`
  - **Types**: `HookResult<T>` for read-write hooks, `HookNoSetResult<T>` for read-only hooks
  - **Field<T> Type**: Alternative object-based pattern with always-defined `set` (no-op when disabled):
    ```typescript
    interface Field<T> {
     value: T;
     canSet: boolean;
     set: (next: T) => void;
    }
    const field = useDesignAppSelectionField();
    field.set(newSelection); // Safe - no-op if canSet is false
    ```
  - **ActionField Type**: For action-only hooks without value:
    ```typescript
    port ActionField {
      canExecute: boolean;
      execute: () => void;
    }
    const action = useXStateAction(canEvent, event);
    action.execute(); // Safe - no-op if canExecute is false
    ```
  - **Adapters**: Use `fieldToHookResult(field)` and `hookResultToField(result)` for interop
  - **No Parameters**: Hooks use scope providers (`useKitScope()`, `useDesignScope()`, `useTypeScope()`, `usePieceScope()`, `useConnectionScope()`, `useQualityScope()`) to get context
  - **canSet**: Boolean indicating if the action is available (scope exists and controller is valid). Use this to disable UI elements when action is unavailable.
  - **Examples**:
    - `const [selection, setSelection, canSetSelection] = useDesignAppSelection();`
    - `const field = useDesignAppSelectionField();` // Field<T> pattern
    - `const [camera, setCamera, canSetCamera] = useTypeAppCamera();`
    - `const [isHovered, _, canReadHover] = useKitAppIsTypeHovered();` (inside TypeScopeProvider)
    - `const [loadingKits, _, canReadLoadingKits] = useHomeLoadingKits();` (read-only)
    - `const [theme, setTheme, canSetTheme] = useTheme();` (global settings)
    - `const [language, setLanguage, canSetLanguage] = useLanguage();` (global settings)
    - `const [expertise, setExpertise, canSetExpertise] = useExpertise();` (global settings)
    - `const [mode, setMode, canSetMode] = useMode();` (global settings)
    - `const [device, setDevice, canSetDevice] = useDevice();` (global settings)
  - **Scope Providers**: Wrap components in appropriate scope providers to enable hooks:
    - `<KitScopeProvider guid={kitGuid}>` - For kit context
    - `<DesignScopeProvider guid={designGuid}>` - For design context
    - `<TypeScopeProvider guid={typeGuid}>` - For type context
    - `<PieceScopeProvider guid={pieceGuid}>` - For piece context
    - `<ConnectionScopeProvider guid={connectionGuid}>` - For connection context
    - `<QualityScopeProvider guid={qualityGuid}>` - For quality context
- **Targeted Hooks**: Components MUST use targeted hooks for kit data access. Use the following hooks from `Sketchpad.tsx`:
  - `useKitTypes(guid?)` - returns types array
  - `useKitFiles(guid?)` - returns files array
  - `useKitDesigns(guid?)` - returns designs array
  - `useKitQualities(guid?)` - returns qualities array
  - `useKitAuthors(guid?)` - returns authors array
  - `useKitFolders(guid?)` - returns folders array
  - `useKitPorts(guid?)` - returns ports array
  - `useKitTags(guid?)` - returns tags array
  - `useKitConcepts(guid?)` - returns concepts array
  - `useKitName(guid?)` - returns kit name
  - `useKitDescription(guid?)` - returns kit description
  - `useTypeFromKit(typeGuid, kitGuid?)` - returns specific type
  - `useDesignFromKit(designGuid, kitGuid?)` - returns specific design
- **Stable Selectors**: When using `useSyncExternalStore` (via `useKit`, `useSyncField`, etc.), selectors MUST be stable references. Inline functions like `(k) => k.types ?? []` are recreated each render, causing the `getSnapshot` callback to be recreated and triggering infinite re-render loops. Use one of:
  - Module-level constant functions: `const selectTypes = (k) => k.types ?? EMPTY_TYPES;`
  - `useCallback` with proper dependencies for dynamic selectors
  - Stable fallback constants: `const EMPTY_TYPES: Type[] = [];` instead of inline `[]`
- **Deep vs Shallow Subscriptions**: AVOID `deep=true` unless you need to react to nested property changes within array items. Use `deep=false` (default) for add/remove/replace operations.
- **Stabilizing useMemo Dependencies**: When hooks return object/array references that change on each render, extract primitive values before passing to `useMemo`. Use refs to track previous values and `useEffect` for side effects that should only run when data actually changes:

  ```typescript
  const type = useType();
  const typeGuid = type?.guid;  // Extract primitive
  const typeModels = type?.models;  // Reference will change but content is stable
  const prevModelGuidRef = useRef<string | null>(null);

  const { modelGuid } = useMemo(() => { /* compute */ }, [typeModels, ...]);

  useEffect(() => {
    if (modelGuid !== prevModelGuidRef.current) {
      prevModelGuidRef.current = modelGuid;
      console.log("Model changed:", modelGuid);
    }
  }, [modelGuid]);
  ```

- **Performance Logging**: Use `enablePerformanceLogging(true)` to enable performance logging that tracks overfetching. Check console for `[PERF] Rapid re-render` warnings indicating components re-rendering too frequently.
- **Granular Piece Metadata System**: The piece metadata system uses DerivedStore for efficient caching of computed piece data:
  - **`usePiecesMetadataMap()`**: Returns a cached `Map<string, PieceMetadata>` for all pieces in the current design. Uses DerivedStore to cache the full piecesMetadata computation. Only recomputes when pieces or connections change.
  - **`usePieceMetadata(pieceId?)`**: Returns metadata for a specific piece, extracting from the cached Map.
  - **`useFlatPiecePlane(id?)`**: Returns the flattened plane for a piece.
  - **`useFlatPieceCenter(id?)`**: Returns the flattened center for a piece.
  - **`useIsConnectedPiece(id?)`**: Returns whether a piece has a parent connection.
  - **`usePieceDepth(id?)`**: Returns the depth of a piece in the connection hierarchy.
  - **`useFixedPieceId(id?)`**: Returns the fixed piece ID (root of the connected component).
  - **`useParentPieceId(id?)`**: Returns the parent piece ID if connected.
- **YPath and DerivedStore**: For fine-grained subscriptions beyond field-level:
  - **YPath**: Navigate Y.js structures with `[yPathMapKey("pieces"), yPathArrayItemById(pieceGuid, "guid")]`
  - **usePath(store, path, selector)**: Subscribe to a specific path in a Y.js store
  - **useDerived(derivedStore, key, deps, compute, selector)**: Subscribe to a computed value that depends on base paths
  - **DerivedStore**: Each `KitStore` and `DesignStore` has a `derived` property for caching computed values
- Kit concepts live in `KitStore` as `ConceptStore` entries backed by the `yConcepts` Y.Array; snapshots return full `Concept` objects (name, description, icon, attributes) and persistence rebuilds them from `yDoc.getArray("concepts")` with legacy guid fallback.
- Commands ALWAYS have an origin. ALWAYS add the id of the ui element as origin when calling commands.
- There is a transaction mechanism for kits. Every app transaction is an extended kit transaction. The undo redo manager is on app level and stores the diff of the transaction along with the app state. This way undo redo works even when the kit changes because only the diff is stored. The inverted diff is stored along with the diff to enable relative undo redo.
- NEVER use direct strings or `useTranslation` for displaying text. ALWAYS assign an `id` the ui element and use i18n keys which match the id.
- The code runs in different environments (different browsers, electron, mobile/desktop/tablet). Platform-specific functionality MUST be generalized and provided as props to Sketchpad. NEVER hardcode platform-specific behavior or APIs directly in components.
- Model tag selection is implemented via `TypeAppFooter` and `DesignAppFooter` components showing clickable tag names, the `selectBestModel(models, selectedTagGuids)` function to find the best matching model, and `selectedModelTags` state tracked per type (in Design app: `Record<Guid, string[]>` mapping type guids to selected tag guids).
- `SUPPORTED_3D_EXTENSIONS` constant in `semio.ts` lists all supported 3D formats. Use `validateModelFile(filename)` to check if a file extension is supported.

The former `Canvas`, `Navbar`, `Footer`, `Panel`, and `store` modules now live inside `js/semio/sketchpad/Sketchpad.tsx`. Keep the region order intact when modifying this file so downstream imports continue to work.

### Architecture - Open-Closed Principle

The codebase follows the Open-Closed Principle (OCP): closed for modification, open for extension. Adding new features ONLY requires adding new files/folders, NEVER modifying existing ones.

### Sketchpad App Plugin Architecture

The sketchpad uses a plugin-based architecture for apps. Each app (Home, Kit, Type, Design, Quality, Docs) registers itself via the `AppPlugin` system, enabling open/closed extensibility.

#### Plugin Structure

Each app plugin provides:

- **id**: Unique identifier (e.g., "home", "kit", "type", "design")
- **namespace**: Event prefix (e.g., "HOME", "KIT", "TYPE", "DESIGN")
- **machine**: XState machine contributions (actions, guards, eventHandlers, selectors)
- **createDefaultState**: Factory for initial app state
- **registerStores**: Optional store factory registration

##### File Layout

```
js/semio/sketchpad/
  shared.ts          # AppPlugin port, registry functions
  apps/
    index.ts         # Single import point for all app plugins
  Home.tsx           # Home app + homeAppPlugin
  Kit.tsx            # Kit app + kitAppPlugin
  Type.tsx           # Type app + typeAppPlugin
  Design.tsx         # Design app + designAppPlugin
  Quality.tsx        # Quality app + qualityAppPlugin
  Docs.tsx           # Docs app + docsAppPlugin
  Feedback.tsx       # Feedback app + feedbackAppPlugin
  Sketchpad.tsx      # Main orchestrator, XState machine
```

##### Plugin Registration

Apps register plugins as a side-effect on module import:

```typescript
const myAppPlugin: AppPlugin = {
  id: "myapp",
  namespace: "MYAPP",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: () => ({ ... }),
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(myAppPlugin);
}
```

##### Dynamic Event Dispatch

The sketchpad machine uses **dynamic event dispatch** via `dispatchAppEvent` action with **wildcard event handling**. Navigation states use `"*"` wildcard to accept ANY event, which is then dispatched to registered handlers.

**Architecture:**

```
┌─────────────────────────────────────────────────────────────┐
│ Sketchpad.tsx (App-Agnostic)                                │
│                                                             │
│  sketchpadMachine:                                          │
│    on: {                                                    │
│      // Explicit handlers for global events                 │
│      SET_THEME, SET_LANGUAGE, NAVIGATE, ...                │
│      // Wildcard at ROOT level catches all app events       │
│      "*": { actions: "dispatchAppEvent" }                  │
│    }                                                        │
│    states:                                                  │
│      navigation: { home: {}, kit: {}, design: {}, ... }    │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ shared.ts (Event Registry)                                  │
│                                                             │
│  registerEventHandler("HOME.TOGGLE_PANEL", handler)        │
│  registerEventHandler("KIT.SET_FILTER", handler)           │
│  executeEventHandler(context, event) → context updates     │
└─────────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────┬──────────────┬──────────────┬───────────────┐
│  Home.tsx    │   Kit.tsx    │  Design.tsx  │   Type.tsx    │
│              │              │              │               │
│ registerEvent│ registerEvent│ registerEvent│ registerEvent │
│ Handler(...) │ Handler(...) │ Handler(...) │ Handler(...) │
└──────────────┴──────────────┴──────────────┴───────────────┘
```

**Event Handler Registration:**

```typescript
import { registerEventHandler } from "./shared";

// Register handler for a specific event type
registerEventHandler("MYAPP.TOGGLE_PANEL", {
 guard: (context, event) => context.myApp !== undefined, // optional
 action: (context, event) => ({
  myApp: {
   ...context.myApp,
   panelVisibility: { ...context.myApp.panelVisibility, [event.panel]: !context.myApp.panelVisibility[event.panel] },
  },
 }),
});
```

**Key Functions:**

- **`registerEventHandler(eventType, config)`**: Registers a handler for a specific event type (e.g., "HOME.TOGGLE_PANEL")
- **`executeEventHandler(context, event)`**: Looks up and executes the handler for the event type
- **`dispatchAppEvent` action**: The sketchpad machine action that dispatches events dynamically
- **Fallback**: If no handler is registered via `registerEventHandler`, falls back to legacy `registerRuntimeAction` handlers

##### App Hooks Registry

Apps register hooks via the registry in `shared.ts` to enable cross-app communication without direct imports:

- **`registerDesignAppHooks(hooks)`**: Design.tsx registers its hooks (selection, hover, commands, etc.)
- **`registerKitAppHooks(hooks)`**: Kit.tsx registers its hooks (commands)
- **`registerDocsRegistry(registry)`**: Docs.tsx registers the docsRegistry
- **`getDesignAppHooks()`**: Returns registered design hooks (fallback defaults if not registered)
- **`getKitAppHooks()`**: Returns registered kit hooks (fallback defaults if not registered)
- **`getDocsRegistry()`**: Returns registered docs registry (null if not registered)

This pattern ensures:

- Sketchpad.tsx has no app-specific caches or hook getters
- elements.tsx has no imports from app modules
- Apps are self-contained and register their hooks on module load

**Benefits:**

- **Open/Closed Principle**: Adding a new app requires NO changes to `Sketchpad.tsx`
- **Self-contained apps**: Each app file registers its own event handlers
- **Wildcard handling**: Navigation states accept any event via `"*"` pattern
- **Guards in handlers**: Guards can be defined in the handler config, not in the machine
- **Gradual migration**: Existing `registerRuntimeAction` handlers continue to work
- **Single machine**: Only one `createMachine` call - `uiMachine` has been removed

##### Hook Pattern (Triadic)

All hooks follow the triadic pattern: `[value, setValue, canSetValue]`

- **UI components**: Only use triadic hooks, never access stores directly
- **Hooks**: Read from stores via subscriptions, write via `actor.send()` XState events
- **State machine**: Only writer API, accepts contributions from plugins
- **Stores/commands**: Implementation details behind machine actions

Example:

```typescript
export function useMyAppSelection(): HookResult<MySelection> {
  const actor = useSketchpadActor();
  const canSetEvent = useMemo(() => ({ type: "MYAPP.SET_SELECTION" as const, ... }), [...]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setSelection = useMemo(() => {
    if (!canSet) return undefined;
    return (value: MySelection) => actor.send({ type: "MYAPP.SET_SELECTION", ... });
  }, [actor, canSet, ...]);
  return conditionalHookResult(canSet, selection, setSelection);
}
```

##### Adding a New App

1. Create app file with types, state, hooks, and UI components
2. Define `AppPlugin` with namespace and machine contributions
3. Register plugin: `registerAppPlugin(myAppPlugin)`
4. Import app module in `apps/index.ts`
5. No edits to `Sketchpad.tsx` required (open/closed principle)

####### App Structure Standards

All apps in `js/semio/sketchpad/*App.tsx` (Design.tsx, Home.tsx, Kit.tsx, Quality.tsx, Type.tsx, Docs.tsx) MUST follow this structure:

1. **Region Order:** Header → Imports → Types → Store → Commands → Components → App → Config
2. **Store Base Class:** MUST extend either `AppStore` or `KitDiffAppStore` (no custom base classes)
3. **Store Registration:** MUST use inline registration pattern (no wrapper functions)
4. **Component Regions:** MUST nest under Components region (Navbar, Canvas, Panels, Tools, Footer)
5. **Tools:** MUST have Tools region if app has multiple interaction modes
6. **Scope Providers:** MUST be defined in app file (not App.tsx)
7. **Commands:** MUST define all commands in Commands region

See `REFACTOR.md` for detailed rationale and migration guide.

####### Adding a New App

To add a new app:

1. Create a file in `js/semio/sketchpad/{AppName}.tsx`.
2. Add a single file that:
   - exports the default React component,
   - declares and exports `config: AppConfig`,
   - wires any local state, commands, or helpers needed by the app.
3. Keep optional helpers (pages, panels, tools) alongside the file and import them from the same module.

The app registry auto-discovers app files via `import.meta.glob('./*.tsx')`.

Example section inside the app file:

```typescript
import { FC } from "react";
import { AppConfig } from "../registry";

const App: FC = () => {
 // ...
};

export const config: AppConfig = {
 id: "myapp",
 component: App,
 routeSegments: [{ path: "my/:id", paramName: "id" }],
 getPanels: (t) => [{ key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" }],
 matchesPath: (pathParts) => pathParts[0] === "my",
 order: 50,
};

export default App;
```

##### Sketchpad Apps

###### Home App (Home.tsx)

Landing page for kit management. Extends `AppStore` (no kit modifications).

**State (`HomeState`):**

- `panelVisibility` - Panel toggle states
- `selection` - Selected kit GUIDs
- `sortColumn` / `sortDirection` - Sorting preferences
- `loadingKits` - Kits currently being loaded

**Events:**

- `HOME.TOGGLE_PANEL` - Toggle panel visibility
- `HOME.SET_PANEL_VISIBILITY` - Set all panel states
- `HOME.SELECT_KIT` / `HOME.DESELECT_KIT` - Kit selection
- `HOME.SET_SORT` - Change sorting

**Hooks:**

- `useHomeApp()` - Full home app state
- `useHomeSelection()` - Selected kits
- `useHomeLoadingKits()` - Loading state
- `useHomePanelVisibility()` - Panel visibility

###### Kit App (Kit.tsx)

Kit artifact management with multi-window layout. Extends `KitDiffAppStore` (modifies kit data).

**Window Kinds (`KitAppWindowKind`):**

- `Table` - Tabular view of kit artifacts (types, designs, qualities, etc.)
- `Diagram` - Force-directed graph visualization of artifacts and relationships

**Diagram Relationships:**

- **Part-of**: Parent-child relationships (type/design parent, folder containment)
- **Reference**: Usage relationships (e.g., type referenced by design via pieces)

**State (`KitAppState`):**

- `panelVisibility` - Panel toggle states
- `selection` - Selected artifacts (types, designs, qualities, ports, tags, concepts, files, folders, authors)
- `hover` - Hovered artifact
- `filterSearch` - Search filter string
- `expandedRows` - Expanded table rows
- `sortColumn` / `sortDirection` - Sorting preferences
- `windowLayout` - Multi-window layout configuration

**Selection Types:** Types, designs, qualities, ports, tags, concepts, files, folders, authors

**Events:**

- `KIT.TOGGLE_PANEL` - Toggle panel visibility
- `KIT.SELECT_TYPE` / `KIT.DESELECT_TYPE` - Type selection
- `KIT.SELECT_DESIGN` / `KIT.DESELECT_DESIGN` - Design selection
- `KIT.SET_HOVER` - Set hover state
- `KIT.SET_FILTER_SEARCH` - Update search filter
- `KIT.SET_EXPANDED_ROWS` - Expand/collapse rows
- `KIT.CREATE_TYPE` / `KIT.CREATE_DESIGN` / `KIT.CREATE_QUALITY` - Create artifacts

**Hooks:**

- `useKitApp()` - Full kit app state
- `useKitAppSelection()` - Current selection
- `useKitAppHover()` - Hover state
- `useKitAppFilterSearch()` - Filter string
- `useKitAppWindowLayout()` - Window layout configuration

###### Type App (Type.tsx)

Type editing (connectors, models). Extends `KitDiffAppStore`.

**State (`TypeAppState`):**

- `panelVisibility` - Panel toggle states
- `activeTool` - Current tool (selection, etc.)
- `selection` - Selected connectors/models
- `hover` - Hovered connector/model
- `camera` - 3D camera state
- `focusedConnectorGuid` - Connector being edited
- `selectedModelGuid` - Active model
- `selectedModelTags` - Tags for model selection
- `fullscreenWindow` - Fullscreen mode
- `windowLayout` - Window arrangement

**Events:**

- `TYPE.TOGGLE_PANEL` - Toggle panel visibility
- `TYPE.SET_TOOL` - Change active tool
- `TYPE.SELECT_CONNECTOR` / `TYPE.DESELECT_CONNECTOR` - Connector selection
- `TYPE.SELECT_MODEL` / `TYPE.DESELECT_MODEL` - Model selection
- `TYPE.SET_HOVER` - Set hover state
- `TYPE.SET_CAMERA` - Update camera
- `TYPE.SET_SELECTED_MODEL_TAGS` - Model tag selection

**Hooks:**

- `useTypeApp()` - Full type app state
- `useTypeAppSelection()` - Current selection
- `useTypeAppHover()` - Hover state
- `useTypeAppCamera()` - Camera state
- `useTypeAppActiveTool()` - Active tool

###### Design App (Design.tsx)

Design editing (pieces, connections). Extends `KitDiffAppStore`.

**State (`DesignAppState`):**

- `panelVisibility` - Panel toggle states
- `activeTool` - Current tool (selection, connection, etc.)
- `selection` - Selected pieces/connections/connector
- `hover` - Hovered pieces/connections/connectors/types/designs
- `camera` - 3D camera state
- `diagramCenter` / `diagramScale` - 2D diagram view
- `focusedPieceGuid` - Piece being edited
- `selectedModelTags` - Model tags per type (`Record<Guid, string[]>`)
- `fullscreenWindow` - Fullscreen mode
- `windowLayout` - Window arrangement

**Selection Types:** Pieces, connections, connector (single connector selection for connection)

**Events:**

- `DESIGN.TOGGLE_PANEL` - Toggle panel visibility
- `DESIGN.SET_TOOL` - Change active tool
- `DESIGN.SELECT_PIECE` / `DESIGN.DESELECT_PIECE` - Piece selection
- `DESIGN.SELECT_CONNECTION` / `DESIGN.DESELECT_CONNECTION` - Connection selection
- `DESIGN.SET_HOVER` - Set hover state
- `DESIGN.SET_CAMERA` - Update 3D camera
- `DESIGN.SET_DIAGRAM_CENTER` / `DESIGN.SET_DIAGRAM_SCALE` - 2D diagram view
- `DESIGN.DELETE_SELECTED` - Delete selected elements
- `DESIGN.SET_SELECTED_MODEL_TAGS` - Model tag selection per type

**Commands:**

- `semio.designApp.selectAll` - Select all pieces and connections
- `semio.designApp.deselectAll` - Clear selection
- `semio.designApp.deleteSelected` - Delete selected elements

**Hooks:**

- `useDesignApp()` - Full design app state
- `useDesignAppSelection()` - Current selection
- `useDesignAppHover()` - Hover state
- `useDesignAppCamera()` - 3D camera
- `useDesignAppActiveTool()` - Active tool
- `useDesignAppDiagramCenter()` / `useDesignAppDiagramScale()` - Diagram view

**HoverIntentContext:**

Design app uses `HoverIntentContext` to manage hover/pan/drag state via refs instead of module-level variables:

- `hoverClearTimeoutRef` - Timeout for clearing hover state
- `currentHoveredPieceGuidRef` - Currently hovered piece GUID
- `isPanningRef` - Whether user is panning the canvas
- `isDraggingNodeRef` - Whether user is dragging a node

Access via `useHoverIntent()` hook within `HoverIntentProvider` scope.

**Derived State Providers:**

- `TransactionPiecesProvider` - Provides `changedPieces` Set and `statusMap` Map for pieces affected by current transaction
- `HoverPiecesProvider` - Provides `transitivelyHoveredPieces` and `transitivelyHoveredTypes` for hover highlighting

Both use `useSyncExternalStore` with structural equality helpers (`areSetsEqual`, `areMapsEqual`) instead of JSON.stringify diffing.

###### Quality App (Quality.tsx)

Quality/benchmark editing with formula visualization. Extends `KitDiffAppStore`.

**State (`QualityAppState`):**

- `panelVisibility` - Panel toggle states
- `activeTool` - Current tool
- `selection` - Selected formula nodes
- `hover` - Hovered formula node
- `formulaNodes` - Parsed formula tree
- `fullscreenWindow` - Fullscreen mode
- `windowLayout` - Window arrangement

**Formula Functions:** Numeric (Add, Subtract, Multiply, Divide, ...), Branching (If, Switch, ...), Data (Min, Max, Avg, ...), Text, Comparison

**Events:**

- `QUALITY.TOGGLE_PANEL` - Toggle panel visibility
- `QUALITY.SET_TOOL` - Change active tool
- `QUALITY.SELECT_FORMULA_NODE` / `QUALITY.DESELECT_FORMULA_NODE` - Node selection
- `QUALITY.SET_HOVER` - Set hover state

**Hooks:**

- `useQualityApp()` - Full quality app state
- `useQualityAppSelection()` - Current selection
- `useQualityAppHover()` - Hover state

###### Docs App (Docs.tsx)

In-app documentation viewer with MDX support.

**Features:**

- MDX file loading from `./pages/**/*.mdx`
- Section-based navigation
- Heading extraction for table of contents
- Tab components for content organization

**MDX Loading:**

- `loadMDXFile(path)` - Load single MDX file
- `getAllMDXFiles()` - List all MDX files
- `getMDXFilesBySection(section)` - Files in a section
- `getAllSections()` - All available sections

**Heading State:**

- `useHeadings()` - Subscribe to heading updates
- `headingsState.registerHeading(id, level, text)` - Register heading
- `headingsState.setActiveHeading(id)` - Set active heading

###### Feedback App (Feedback.tsx)

Bug report and feature idea submission form. State managed via XState triadic hooks.

**Route:** `/feedback`

**State (`FeedbackAppState` in Sketchpad.tsx):**

- `panelVisibility` - Panel toggle states
- `formData` - Form data (kind, title, description, app, name, email)
- `isSubmitting` - Form submission in progress
- `isSubmitted` - Form successfully submitted
- `error` - Error message if submission failed

**Form Kinds (`FeedbackKind`):**

- `bug` - Bug report (requires app selection)
- `idea` - Feature idea

**Triadic Hooks:**

- `useFeedbackFormData()` - `[formData, setFormData, canSet]`
- `useFeedbackIsSubmitting()` - `[isSubmitting, setIsSubmitting, canSet]`
- `useFeedbackIsSubmitted()` - `[isSubmitted, setIsSubmitted, canSet]`
- `useFeedbackError()` - `[error, setError, canSet]`
- `useFeedbackReset()` - `[reset, canReset]`

**Events:**

- `FEEDBACK.TOGGLE_PANEL` - Toggle panel visibility
- `FEEDBACK.SET_FORM_DATA` - Update form fields
- `FEEDBACK.RESET_FORM` - Reset form to initial state
- `FEEDBACK.SET_SUBMITTING` - Set submitting state
- `FEEDBACK.SET_SUBMITTED` - Set submitted state
- `FEEDBACK.SET_ERROR` - Set error message

**Global Footer Action:**

The feedback icon appears in every app's footer via `GlobalFooterItems` component in Sketchpad.tsx, providing universal access to the feedback form.

####### Adding a New Tool

To add a new tool to an app:

1. Create a `*Tool.tsx` file directly inside `js/semio/sketchpad/`.
2. Export a `Tool<AppState>` object with a unique `id` and `render` implementation.

Each app loads sibling `*Tool.tsx` modules via `import.meta.glob('./*Tool.tsx', { eager: true })`, so simply dropping the file in place registers it.

Example:

```typescript
export const MyTool: Tool<MyAppState> = {
  id: ToolKind.MY_TOOL,
  label: "My Tool",
  icon: <Icon />,
  render: (context) => ({ scene: <></>, diagram: null, table: null }),
};
```

####### Adding Panel Sections

Panel sections are dynamically added in the app's `useEffect`:

```typescript
useEffect(() => {
  removeSection("details", "my-section");
  addSection("details", {
    id: "my-section",
    label: t("mySection"),
    content: () => <MyComponent />,
    order: 1,
  });
  return () => removeSection("details", "my-section");
}, [appType, addSection, removeSection]);
```

Policies:

1. When a section id is conditional (for example `"properties"` vs `"multipleTitle"`), always `removeSection` for all possible ids before adding the currently active one.
2. Always `removeSection` for every id you `addSection` (including conditional variants) in the effect cleanup.
3. If the section content uses scope-bound hooks (`useKit()`, `useDesign()`, `useType()`), wrap `content` with the corresponding `*ScopeProvider` when registering the section.

####### Tutorials

The tutorial system is consolidated in `js/semio/sketchpad/Tutorials.tsx` and is split into regions for types, store, commands, built-in tutorials, and UI components. `TutorialStore` wraps a Y.js map and keeps playback, milestone ordering, and recording state (`TutorialPlaybackState`, `TutorialRecordingState`). Always create the store with the app transaction handler so tutorial mutations participate in undo/redo.

Wrap consumers in `TutorialProvider` and use the helper hooks (`useTutorialStore`, `useActiveTutorial`, `useTutorialProgress`, `useTutorialCommandInterceptor`, etc.) instead of accessing the store directly. `TutorialControls`, `RecordingControls`, `RecordButton`, and `TutorialOverlay` are the canonical UI integrations for playback, recording, highlighting, and capture.

Tutorial commands are consolidated in `Tutorials.tsx` under the `tutorialCommands` and `devCommands` objects for the `semio.tutorial.*` and `semio.recording.*` namespaces. Bundle reusable walkthroughs or recordings as data objects (for example `helloTutorial`, `sketchpadTour`) and register them with `addTutorial`.

All tutorial-related code (types, store, commands, UI components, and built-in tutorials) is now in a single file using regions for organization instead of being spread across multiple files in a separate folder.

####### Footer

`FooterItemProvider` wraps `Sketchpad` so apps can register footer entries with `useAddFooterItem` and remove them via `useRemoveFooterItem`; the provider keeps items ordered by the optional `order` field.

Register items inside effects and always call the remove helper in the cleanup; default contributions now live inside each app's `App.tsx`, next to the `config` export.

Providing an `id` shows the translated `DescriptionTooltipContent`, and the base footer auto-hides in fullscreen until the cursor nears the bottom edge, so interactive elements must tolerate that visibility change.

The shared `Footer` component has a fixed `h-medium` height.

##### Styling

- NEVER use colors and spacing directly. ALWAYS use semantic variables from `global.css`. Only `global.css` uses colors and pixels directly.
- NEVER add semantic values and ALWAYS use hardcoded values in `semio-elements/ui/theme.css`. NEVER use `semio-elements/ui/theme.css` outside of `global.css`.
- ALWAYS use the standardized unit-based sizing system defined in globals.css (derived from `--spacing`):
  - Single: 1 unit - spacing between elements and between icon and element (e.g. `gap-1`)
  - Tiny: 3 units - icon size in actions, action text size (e.g. `h-tiny`, `w-tiny`, `text-tiny`)
  - Small: 5 units - actions, avatars, Strip items (e.g. `h-small`, `w-small`)
  - Medium: 7 units - buttons, toggles, inputs, sliders, steppers, Footer, table rows, Strip (e.g. `h-medium`, `w-medium`)
  - Large: 9 units - Band, Navbar (e.g. `h-large`, `w-large`)
  - Huge: 11 units - height of navigation buttons at bottom of docs pages (e.g. `h-11`)
  - Mega: 13 units - width of toggles with actions (toggles with dropdown or action buttons) (e.g. `w-mega`)
  - Giga: 15 units - reserved for future use (e.g. `w-giga`)
- Table body cells MUST NOT add vertical padding; `Table` centers cell content and uses `px-single py-0` so `h-medium` rows stay fixed even when rendering `h-medium` controls.

##### Store Architecture

This document describes the generalized store hierarchy for the semio application.

#### Overview

The store architecture consists of three levels of abstraction:

1. **Store** - Base class for any component with data
2. **AppStore** - Base class for apps with transaction support and undo/redo
3. **KitDiffAppStore** - Base class for apps that modify kits and track both app-specific and kit diffs

#### Store Hierarchy

```
Store<TState>
  ↓ extends
AppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult>
  ↓ extends
KitDiffAppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult>
```

#### 1. Store (Base Class)

The `Store` class is the foundation for all components that hold data.

##### Responsibilities

- State management with snapshot caching
- Observable pattern (onChanged, onChangedDeep)
- Access to parent SketchpadStore
- Y.js integration via yMap

##### Abstract Methods

- `hash(state: TState): string` - Generate a hash for cache invalidation
- `buildSnapshot(): TState` - Build the current state snapshot

##### Usage

Use this for simple components that only need state management without editing capabilities (e.g., HomeStore).

#### 2. AppStore (extends Store)

The `AppStore` adds transaction support with undo/redo functionality for any app.

##### Responsibilities

- Transaction management (start, finalize, abort)
- Undo/redo with two stacks:
  - **Current transaction stack**: Edits in the active transaction (merged on finalize)
  - **Past transactions stack**: Finalized transactions
- Selection management with diff-based updates
- Panel visibility and fullscreen management

##### Transaction Model

Every app supports transactions:

1. **Start Transaction**: `startTransaction()`
   - Activates transaction mode
   - New edits go to current transaction stack

2. **During Transaction**: `executeCommand(...)`
   - Each command creates an edit with `do` and `undo` steps
   - Edits accumulate in current transaction stack
   - Undo/redo work within the current transaction

3. **Finalize Transaction**: `finalizeTransaction()`
   - Merges all edits in current transaction into one edit
   - Moves merged edit to past transactions stack
   - Clears redo stack

4. **Abort Transaction**: `abortTransaction()`
   - Undoes all edits in current transaction
   - Clears current transaction stack

##### UI Transaction Context (Sketchpad elements)

Sketchpad UI elements resolve transactions via React context (not props):

- `semio-elements/ui/elements.tsx` defines `TransactionProvider` and `useTransaction()`.
- `semio-elements/ui/elements.tsx` `Geometry` treats `color` as the base (non-interactive) color and uses selection/hover theme colors for the rendered material/edges when `selected`/`hovered` are true.
- `js/semio/sketchpad/Design.tsx` diagram piece nodes use non-inset rings (`ring-*`, not `ring-inset`) so rings remain visible on `Avatar` nodes with full-size `AvatarFallback` backgrounds.
- Elements such as `Input`, `Textarea`, `Select`, `Slider`, `Stepper`, `Combobox`, and `ActionDropdown` call `useTransaction()` internally and do not accept a `transaction` prop.
- Apps are responsible for scoping transactions by wrapping their UI subtree with `TransactionProvider` using the appropriate transaction hook (per-app or kit-level), so all descendant elements participate consistently.

##### Hooks and Helpers

- **`useSync` / `useSyncDeep`** (from `js/semio/sketchpad/Sketchpad.tsx`) wrap `useSyncExternalStore` against a store's `onChanged` / `onChangedDeep` events. Pass a selector (defaults to `identitySelector`) to scope renders to the slice you need.
- **`useSyncField` / `useSyncFields`** subscribe to Y.js-backed store fields with optional `comparator?: (a: TSelected, b: TSelected) => boolean` parameter for custom equality checks instead of JSON.stringify. Use for Set/Map values or other complex types.
- **`createObserver`** bridges a Y.js map or array into the store by registering either shallow or deep observers; always dispose the returned cleanup in `useEffect` finalizers.
- **`RemoteProviders`** bundles the `yProvider` and `fileProvider` factories needed when constructing `SketchpadStore` so persistence and external file access stay aligned.

##### Edit Structure

```typescript
interface AppEdit<TSelectionDiff> {
 do: AppStep<TSelectionDiff>;
 undo: AppStep<TSelectionDiff>;
}

interface AppStep<TSelectionDiff> {
 selectionDiff?: TSelectionDiff;
}
```

Each edit stores:

- **do**: Forward diff to apply the change
- **undo**: Inverse diff to revert the change

#### Abstract Methods (in addition to Store)

- `applySelectionDiff(selectionDiff: TSelectionDiff): void` - Apply selection changes to Y.js
- `inverseSelectionDiff(selection, diff): TSelectionDiff` - Calculate inverse diff for undo
- `getSelection()` - Get current selection state

##### Undo/Redo Behavior

**Within Transaction:**

- Undo: Pops from current transaction stack, stores in temp variable
- Redo: Pushes temp variable back to current transaction stack

**Outside Transaction:**

- Undo: Moves edit from past transactions stack to redo stack
- Redo: Moves edit from redo stack back to past transactions stack

##### Usage

Use this for apps that don't modify kits (e.g., HomeStore for managing the home screen).

#### 3. KitDiffAppStore (extends AppStore)

The `KitDiffAppStore` extends AppStore for apps that modify kits (designs, types).

##### Additional Responsibilities

- Tracks kit diffs alongside app-specific diffs
- Applies kit changes through KitStore
- Records both app and kit changes in edits

##### Edit Structure

```typescript
interface KitDiffAppEdit<TSelectionDiff> {
 do: KitDiffAppStep<TSelectionDiff>;
 undo: KitDiffAppStep<TSelectionDiff>;
}

interface KitDiffAppStep<TSelectionDiff> {
 kitDiff?: KitDiff;
 selectionDiff?: TSelectionDiff;
}
```

Each edit stores:

- **do.kitDiff**: Forward kit diff to apply changes
- **do.selectionDiff**: Forward selection diff
- **undo.kitDiff**: Inverse kit diff to revert changes
- **undo.selectionDiff**: Inverse selection diff

##### Undo/Redo Behavior

Extends AppStore undo/redo to also:

- Apply/revert kit diffs through `kit().change(kitDiff)`
- Handle both kit and selection changes atomically

##### Abstract Methods

- `kit(): KitStore` - Get the associated kit store

##### Usage

Use this for apps that modify kits:

- **DesignAppStore** - Edit designs (pieces, connections)
- **TypeAppStore** - Edit types (connectors, models)
- **KitAppStore** - Edit kits (types, designs, qualities, files, authors)

#### Concrete Implementations

##### DesignAppStore

Edits design content:

- Selection: pieces, connections, connectors
- Kit diffs: piece changes, connection changes
- Transaction support for complex multi-step operations

##### TypeAppStore

Edits type definitions:

- Selection: connectors, models
- Kit diffs: connector changes, model changes
- Transaction support for type modifications

##### KitAppStore

Edits kit metadata:

- Selection: types, designs, qualities, files, authors
- Kit diffs: add/remove artifacts
- Transaction support for kit-level operations

##### HomeStore

Manages home screen (extends AppStore, not KitDiffAppStore):

- Selection: kits
- No kit diffs (doesn't modify kit content)
- Sorting and filtering state

#### Command Pattern

All apps use a command pattern:

```typescript
interface CommandContext {
  // Current state
}

interface CommandResult {
  diff?: TDiff;      // App-specific diff
  kitDiff?: KitDiff; // Kit diff (only for KitDiffAppStore)
}

executeCommand<T>(command: string, ...args): Promise<T>
```

##### Command Execution Flow

1. Look up command in registry
2. Build context with current state
3. Execute command function
4. Apply diffs (app diff + kit diff)
5. Record edit for undo/redo
6. Return result

#### Best Practices

1. **Always use transactions** for multi-step operations
2. **Keep edits atomic** - each edit should be independently undoable
3. **Calculate inverse diffs correctly** - critical for undo
4. **Don't nest transactions** - finish one before starting another
5. **Clear redo stack on new edits** - standard undo/redo behavior
6. **Use selection diffs** for all selection changes

#### Files

- `js/semio/sketchpad/Sketchpad.tsx` - Base Store, AppStore, KitDiffAppStore, SketchpadStore, KitStore
- `js/semio/sketchpad/Design.tsx` - DesignAppStore and design app state
- `js/semio/sketchpad/Type.tsx` - TypeAppStore and type toolchain
- `js/semio/sketchpad/Quality.tsx` - QualityAppStore and quality workflows
- `js/semio/sketchpad/Kit.tsx` - KitAppStore and kit command wiring
- `js/semio/sketchpad/Home.tsx` - HomeStore and home experience
- `js/semio/sketchpad/Docs.tsx` - DocsAppStore and documentation app
- `js/semio/sketchpad/Tutorials.tsx` - Tutorial system (consolidated)
- `js/semio/sketchpad/shared.ts` - Shared types and utilities

#### Kit app artifact creation

- `js/semio/sketchpad/Kit.tsx` create actions for `ports`, `tags`, `concepts`, and `folders` set the active `kind` filter and selection to the newly created entity.
- Default names are resolved via i18n labels: `semio.sketchpad.app.port.defaultName`, `semio.sketchpad.app.tag.defaultName`, `semio.sketchpad.app.concept.defaultName`.

#### XState State Machines

The application uses XState v5 for all Sketchpad UI state. Y.js is reserved for collaborative Kit data.

#### Architecture

- **XState actor** is the source of truth for Sketchpad UI state (`SketchpadState` + app slices).
- **Local persistence**: Sketchpad UI state is written to `localStorage` at `semio.sketchpad.state.<id>`.
- **Y.js** is used only for Kit data (per-kit `KitStore` documents, optionally connected via `RemoteProviders.yProvider`).
- **React hooks** read via `@xstate/react` `useSelector` and write via `actor.send({ type: ... })`.

#### Machine Files

**`Sketchpad.tsx`** contains the main machines:

##### sketchpadMachine

Unified state machine combining data management and hierarchical navigation:

**Root Structure (parallel):**

- Sketchpad UI state lives in the machine context (`SketchpadState` + app slices)
- `navigation` parallel state with hierarchical sub-states

**Navigation States:**

- `home` → `kit` → `design`/`type`/`quality`/`docs`
- State transitions via `KIT.INIT`, `DESIGN.INIT`, `TYPE.INIT` events

**State-Scoped Events:**

App-specific events are only available in their respective navigation states:

- **home**: `HOME.TOGGLE_PANEL`, `HOME.SET_HOVER`, `HOME.SELECT_KIT`, etc.
- **kit**: `KIT.SYNC`, `KIT.TOGGLE_PANEL`, `KIT.SET_FILTER`, `KIT.SELECT_TYPE`, etc.
- **design**: `DESIGN.SYNC`, `DESIGN.SET_HOVER`, `DESIGN.SELECT_PIECE`, `DESIGN.DELETE_SELECTED`, etc.
- **type**: `TYPE.SYNC`, `TYPE.SET_HOVER`, `TYPE.SELECT_CONNECTOR`, `TYPE.HOVER_MODEL`, etc.
- **quality**: `QUALITY.TOGGLE_PANEL`, `QUALITY.TOGGLE_BENCHMARK`

**Global Events (always available):**

- Navigation: `NAVIGATE`, `NAVIGATE_BACK`, `NAVIGATE_FORWARD`
- Settings: `SET_THEME`, `SET_LANGUAGE`, `SET_EXPERTISE`, `SET_MODE`, `SET_DEVICE`
- Background operations: `BACKGROUND.START`, `BACKGROUND.COMPLETE`, `BACKGROUND.FAIL`
- Tutorial: `TUTORIAL.START`, `TUTORIAL.END`, `TUTORIAL.NEXT_STEP`, etc.
- Sketchpad state updates: `CHANGE`

**Per-App Transaction Events (scoped to navigation state):**

Transaction management is per-app, not global. Each app (Design, Type, Kit) has its own transaction state embedded in its app state port.

- **design**: `DESIGN.TRANSACTION.START`, `DESIGN.TRANSACTION.COMMIT`, `DESIGN.TRANSACTION.ABORT`, `DESIGN.TRANSACTION.UNDO`, `DESIGN.TRANSACTION.REDO`, `DESIGN.TRANSACTION.RECORD_EDIT`
- **type**: `TYPE.TRANSACTION.START`, `TYPE.TRANSACTION.COMMIT`, `TYPE.TRANSACTION.ABORT`, `TYPE.TRANSACTION.UNDO`, `TYPE.TRANSACTION.REDO`, `TYPE.TRANSACTION.RECORD_EDIT`
- **kit**: `KIT.TRANSACTION.START`, `KIT.TRANSACTION.COMMIT`, `KIT.TRANSACTION.ABORT`, `KIT.TRANSACTION.UNDO`, `KIT.TRANSACTION.REDO`, `KIT.TRANSACTION.RECORD_EDIT`

**Navigation State Selectors:**

```typescript
import { selectNavigationState, selectIsInDesign, selectIsInType } from "./Sketchpad";

// Check current navigation state
const navState = useSelector(actor, selectNavigationState); // "home" | "kit" | "design" | "type" | "quality" | "docs"
const isInDesign = useSelector(actor, selectIsInDesign); // boolean
```

**Constraint Enforcement:**

- `DESIGN.DELETE_SELECTED` requires `hasDesignSelection` guard AND being in design state
- App-specific events are silently ignored when not in the correct navigation state
- This prevents invalid state transitions (e.g., selecting a piece when not in design view)

##### uiMachine (legacy)

Separate hierarchical UI state machine (kept for reference, functionality merged into sketchpadMachine):

- `interaction` region: Idle → Hovered → Selected → ContextMenu substates
- `tool` region: Active tool state (Design/Type apps)
- `drag` region: Drag-and-drop state (Design app)
- `modal` region: Command palette and search overlays

#### XState Hooks

**`Sketchpad.tsx`** provides XState-based hooks:

- `useSketchpadActor()` - Get the XState actor ref
- `useSketchpadSelector()` - Generic selector using @xstate/react
- `useSketchpadSnapshot()` - Full state snapshot
- `useSketchpadActions()` - Event dispatching functions
- App-specific hooks: `useThemeXState()`, `useModeXState()`, etc.

#### Y.js-XState Bridge

**`shared.ts`** contains bridge utilities:

- `createYjsSyncActor()` - Creates callback actor for Y.js observation
- `createYjsFieldSyncActor()` - Single field observation
- `yTransact()` - Transaction wrapper
- `createYjsUpdateAssign()` - Assign action for Y_UPDATE events
- `createYjsSelector()` - Cached selector with dirty checking

#### State ownership

- Sketchpad UI state (navigation/settings/panel sizes and per-app UI slices) is owned by `sketchpadMachine` context and exposed through XState selectors.
- Kit data is owned by per-kit Y.js documents (`KitStore`) and accessed via kit-level stores/hooks.

#### Transaction State Management

Transaction state is embedded in each app's state port via `AppTransactionState`:

```typescript
interface AppTransactionState<TEdit = any> {
 isTransactionActive: boolean;
 currentTransactionStack: TEdit[]; // Edits in current active transaction
 pastTransactionStack: TEdit[]; // Finalized transactions (for undo)
 redoStack: TEdit[]; // Undone transactions (for redo)
}
```

**Transaction Flow:**

1. **Start**: `APP.TRANSACTION.START` activates transaction mode, clears redo stack
2. **Record Edit**: `APP.TRANSACTION.RECORD_EDIT` pushes edit to current stack
3. **Commit**: `APP.TRANSACTION.COMMIT` merges current stack into one edit, moves to past stack
4. **Abort**: `APP.TRANSACTION.ABORT` discards current stack, deactivates transaction mode
5. **Undo**: `APP.TRANSACTION.UNDO` pops from current (if active) or past stack
6. **Redo**: `APP.TRANSACTION.REDO` moves edit from redo back to past stack

**Background Operations:**

Long-running async operations (kit import, file upload) are tracked via `backgroundOperations`:

```typescript
backgroundOperations: Record<
 string,
 {
  type: string;
  status: "pending" | "running" | "completed" | "failed";
  error?: string;
 }
>;
```

These continue even when navigating away from the originating app.

#### Command System

All state mutations are executed through commands. Commands provide a consistent port for operations and enable undo/redo, logging, and origin tracking.

#### Command Registry

Each store maintains a `commandRegistry` that maps command strings to handler functions. Commands are registered using `registerCommand` and unregistered using `unregisterCommand`.

#### Command Execution

Commands are executed via `executeCommand(command: string, ...args: any[])`:

1. **Origin Extraction**: If the first argument is a string starting with `semio.sketchpad.`, it's treated as the origin (UI element ID). Otherwise, origin is undefined.
2. **Command Lookup**: The command registry is searched for the handler.
3. **Context Building**: A command context is built with current state snapshot.
4. **Handler Execution**: The handler receives context and remaining arguments.
5. **Diff Application**: Result diffs are applied to the store.
6. **Edit Recording**: For AppStore/KitDiffAppStore, edits are recorded for undo/redo.

#### Command Naming Convention

Commands follow the pattern `semio.{scope}.{action}`:

- `semio.sketchpad.*` - Sketchpad-level commands
- `semio.kitApp.*` - Kit app commands
- `semio.designApp.*` - Design app commands
- `semio.typeApp.*` - Type app commands
- `semio.home.*` - Home app commands

Special commands:

- `semio.{app}.startTransaction` - Start a transaction
- `semio.{app}.finalizeTransaction` - Finalize current transaction
- `semio.{app}.abortTransaction` - Abort current transaction
- `semio.{app}.undo` - Undo last edit
- `semio.{app}.redo` - Redo last undone edit

#### Command Origin

Every command execution should include an origin string identifying the UI element that triggered it. Origins follow the pattern `semio.sketchpad.{path}` matching the element's `id` prop. This enables:

- Debugging and logging
- Tutorial recording
- Analytics tracking

### Diff System

The diff system tracks changes to models for undo/redo, synchronization, and persistence.

#### Diff Types

Every model has an associated `Diff` type that represents partial changes:

- **ModelDiff**: Partial update to a single model instance
- **ModelsDiff**: Collection diffs with `removed`, `updated`, and `added` arrays

#### Diff Operations

Each model type supports four diff operations:

1. **`getDiff(before, after): Diff`** - Calculate diff between two states
2. **`inverseDiff(original, appliedDiff): Diff`** - Calculate inverse diff for undo
3. **`mergeDiff(diff1, diff2): Diff`** - Merge two diffs (later takes precedence)
4. **`applyDiff(base, diff): Model`** - Apply diff to base state

#### Diff Status

Diffs track status using `DiffStatus` enum:

- `Unchanged` - No change
- `Added` - Newly added item
- `Removed` - Deleted item
- `Modified` - Updated item

#### Collection Diffs

Collection diffs (`*sDiff`) track changes to arrays/lists:

```typescript
interface CollectionDiff<T> {
 removed?: TId[]; // IDs of removed items
 updated?: { id: TId; diff: TDiff }[]; // Updated items with their diffs
 added?: T[]; // Newly added items
}
```

#### Inverse Diffs

Inverse diffs enable undo by reversing operations:

- `removed` → `added` (restore removed items)
- `added` → `removed` (remove added items)
- `updated` → inverse of the update diff

### Routing & App Registration

Apps are registered via the `AppRegistry` which auto-discovers apps using `import.meta.glob('./*/App.tsx')`.

#### AppConfig

Each app exports a `config: AppConfig`:

```typescript
interface AppConfig {
 id: string; // Unique app identifier
 component: ComponentType; // React component
 routeSegments: RouteSegment[]; // Route path segments
 getPanels: (t: TFunction) => PanelDefinition[]; // Panel definitions
 matchesPath: (pathParts: string[]) => boolean; // Path matcher
 order?: number; // Display order
}
```

#### Route Segments

Route segments define the app's URL structure:

```typescript
interface RouteSegment {
 path: string; // React Router path pattern
 paramName?: string; // Parameter name (e.g., "id")
 scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>; // Scope wrapper
}
```

#### Path Matching

Apps can match paths using `matchesPath(pathParts: string[])`. The registry searches apps in order and returns the first match.

#### Scope Providers

Scope providers wrap app components to provide context (e.g., kit/design/type GUIDs) via React Router params.

### Hotkeys

Hotkeys are configurable keyboard shortcuts stored in the SketchpadStore with user overrides.

#### Hotkey Paths

Hotkey paths follow the pattern `semio.sketchpad.{element.path}` matching UI element IDs. This enables:

- Automatic tooltip display
- Settings UI integration
- Tutorial highlighting

#### Hotkey Values

Hotkeys use the format from `react-hotkeys-hook`:

- `mod+k` - Meta/Ctrl + K
- `shift+alt+d` - Shift + Alt + D
- `escape` - Escape key

#### Hotkey Overrides

Users can override default hotkeys via `hotkeyOverrides` in SketchpadStore. Overrides take precedence over defaults.

#### Hotkey Hooks

- `useHotkey(path, callback, deps)` - Register hotkey handler (from `js/semio/sketchpad/Sketchpad.tsx`)
- `useSetHotkey()` - Set hotkey override
- `useResetHotkey()` - Reset hotkey to default
- `useResetAllHotkeys()` - Reset all overrides

### Core Types (shared.ts)

The `shared.ts` module exports all core types, enums, and ports used across the Sketchpad.

#### Hook Result Types

All hooks follow the triadic pattern returning `[value, setter, canSet]`:

```typescript
type HookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean];
type HookNoSetResult<T> = readonly [T, undefined, boolean];
```

**Helper Functions:**

- `readonlyHookResult(value)` - Create read-only result
- `writableHookResult(value, setter, canSet?)` - Create writable result
- `conditionalHookResult(canSet, value, setter)` - Create conditional result

#### Field<T> Type

Alternative object-based pattern with always-defined `set` function (no-op when disabled):

```typescript
interface Field<T> {
 value: T;
 canSet: boolean;
 set: (next: T) => void;
}

interface ActionField {
 canExecute: boolean;
 execute: () => void;
}
```

**Helper Functions:**

- `createField(value, setter, canSet)` - Create writable field
- `createReadonlyField(value)` - Create read-only field
- `createAction(execute, canExecute)` - Create action field
- `fieldToHookResult(field)` - Convert Field to HookResult
- `hookResultToField(result)` - Convert HookResult to Field

**XState Helpers (Sketchpad.tsx):**

- `useXStateField(value, canEvent, createEvent)` - Create Field from XState selector
- `useXStateFieldWithScope(value, canEvent, createEvent, hasScope)` - With wildcard fallback
- `useXStateAction(canEvent, event)` - Create ActionField from XState event

**App-Level Helper Pattern (Design.tsx):**

```typescript
interface UseDesignAppFieldOptions<T, TEvent> {
 selector: (s: DesignAppState) => T;
 fallback: T;
 canEventType: TEvent["type"];
 createCanEvent: (kitGuid: Guid, designGuid: Guid) => TEvent;
 createSendEvent: (kitGuid: Guid, designGuid: Guid, value: T) => TEvent;
 useWildcardFallback?: boolean;
}

function useDesignAppField<T, TEvent>(options: UseDesignAppFieldOptions<T, TEvent>): Field<T>;
```

#### Core Enums

```typescript
enum Theme {
 SYSTEM = "system",
 LIGHT = "light",
 DARK = "dark",
}
enum Expertise {
 BEGINNER = "beginner",
 NORMAL = "normal",
 EXPERT = "expert",
}
enum Mode {
 USER = "user",
 DEV = "dev",
}
enum StoreStatus {
 IDLE = "idle",
 LOADING = "loading",
 ERROR = "error",
 READY = "ready",
}
enum ToolKind {
 SELECTION_NORMAL,
 SELECTION_ADDITIVE,
 SELECTION_SUBTRACTIVE,
 LASSO_RECTANGULAR,
 LASSO_FREEFORM,
 CONNECTOR,
}
enum WindowKind {
 TABLE = "table",
 SCENE = "scene",
 DIAGRAM = "diagram",
 CUSTOM = "custom",
}
enum PanelPosition {
 LEFT = "left",
 RIGHT = "right",
 MIDDLE = "middle",
 BOTTOM = "bottom",
}
enum PanelKind {
 WORKBENCH,
 TOOLS,
 TOOLBAR,
 HUD,
 STATS,
 DETAILS,
 CHAT,
 SETTINGS,
 PARAMS,
}
```

#### Panel System

Panels are configured via `PanelKind` with predefined positions and behaviors:

```typescript
interface PanelKindConfig {
 icon: ComponentType<{ size?: number }>;
 position: PanelPosition;
 group?: string;
 isTransparent?: boolean;
 isGroupable?: boolean;
 hotkey?: string;
}

interface PanelVisibility {
 toolbar?: boolean;
 workbench?: boolean;
 tools?: boolean;
 hud?: boolean;
 stats?: boolean;
 details?: boolean;
 chat?: boolean;
 settings?: boolean;
 params?: boolean;
}

interface PanelSection {
 id: string;
 content: ReactNode | (() => ReactNode);
 specificity?: number;
 defaultOpen?: boolean;
 order?: number;
 actions?: Array<{ id: string; icon: ReactNode; onClick: () => void }>;
}
```

**Panel Positioning:**

- **LEFT**: Workbench, Tools (grouped)
- **RIGHT**: Details, Chat, Settings (grouped)
- **MIDDLE**: Stats (transparent)
- **BOTTOM**: Toolbar

#### Tool System

Tools define interaction modes within apps:

```typescript
interface Tool<TState = any> {
 id: ToolKind | string;
 icon?: ReactNode;
 render: (context: ToolRenderContext<TState>) => { scene?: ReactNode; diagram?: ReactNode | null; table?: ReactNode | null };
}

interface ToolMode {
 id: string;
 icon?: ReactNode;
 label?: string;
 tooltipId?: string;
}

interface ToolDefinition {
 id: string;
 defaultMode: ToolKind | string;
 modes: ToolMode[];
}
```

#### App IDs

Each app has a typed ID structure:

```typescript
interface KitAppId {
 kit: Guid;
}
interface TypeAppId {
 kit: Guid;
 type: Guid;
}
interface DesignAppId {
 kit: Guid;
 design: Guid;
}
interface QualityAppId {
 kit: Guid;
 quality: Guid;
}
```

### YPath and DerivedStore

YPath provides granular subscriptions to nested Y.js structures. DerivedStore caches computed values.

#### YPath

Navigate Y.js structures with path segments:

```typescript
type YPathSegment = { kind: "mapKey"; key: string } | { kind: "arrayIndex"; index: number } | { kind: "arrayItemById"; id: string; idKey: string };

type YPath = YPathSegment[];
```

**Path Helpers:**

- `yPathMapKey(key)` - Access a Y.Map key
- `yPathArrayIndex(index)` - Access a Y.Array index
- `yPathArrayItemById(id, idKey?)` - Find array item by ID

**Usage:**

```typescript
const path = [yPathMapKey("pieces"), yPathArrayItemById(pieceGuid, "guid")];
const value = getValueAtPath(yMap, path);
```

#### DerivedStore

Caches computed values that depend on Y.js paths:

```typescript
class DerivedNode<T> {
 snapshot(): T;
 subscribe(cb: () => void): Disposable;
 dispose(): void;
}

class DerivedStore {
 getOrCreate<T>(key: string, deps: BaseDependency[], compute: () => T): DerivedNode<T>;
 get<T>(key: string): DerivedNode<T> | undefined;
 delete(key: string): boolean;
 clear(): void;
}
```

**Usage:**

```typescript
const piecesMetadataNode = derivedStore.getOrCreate("piecesMetadata", [{ store: designStore, path: [yPathMapKey("pieces")] }], () => computePiecesMetadata(designStore.snapshot()));
```

### App Plugin Registry

Apps register plugins that contribute event handlers, guards, and state factories.

#### AppPlugin Port

```typescript
interface AppPlugin {
 id: string; // e.g., "home", "kit", "design"
 namespace: string; // e.g., "HOME", "KIT", "DESIGN"
 machine: AppMachineContribution;
 registerStores?: () => void;
 onRegister?: () => void;
}

interface AppMachineContribution {
 actions?: Record<string, (context: any, event: any) => any>;
 guards?: Record<string, (context: any, event: any) => boolean>;
 eventHandlers?: Record<string, { guard?: string; actions?: string | string[] }>;
 selectors?: Record<string, (context: any, ...args: any[]) => any>;
 createDefaultState?: () => any;
}
```

#### Registration Functions

- `registerAppPlugin(plugin)` - Register an app plugin
- `getAppPlugins()` - Get all registered plugins
- `getAppPlugin(id)` - Get plugin by ID
- `hasAppPlugin(id)` - Check if plugin exists
- `composePluginContributions()` - Merge all plugin contributions

#### Event Handler Registry

Dynamic event dispatch for app-specific events:

```typescript
interface EventHandlerConfig<TContext = any, TEvent = any> {
 guard?: (context: TContext, event: TEvent) => boolean;
 action: (context: TContext, event: TEvent) => Partial<TContext>;
}
```

**Registration:**

```typescript
registerEventHandler("HOME.TOGGLE_PANEL", {
 action: (context, event) => ({
  homeApp: {
   ...context.homeApp,
   panelVisibility: { ...context.homeApp.panelVisibility, [event.panel]: !context.homeApp.panelVisibility[event.panel] },
  },
 }),
});
```

**Functions:**

- `registerEventHandler(eventType, config)` - Register handler
- `unregisterEventHandler(eventType)` - Remove handler
- `executeEventHandler(context, event)` - Execute handler
- `getEventTypesForNamespace(namespace)` - List events for namespace
- `getRegisteredNamespaces()` - List all namespaces

#### Guard Registry

Named guards for conditional event handling:

- `registerGuard(name, guard)` - Register guard
- `unregisterGuard(name)` - Remove guard
- `getGuard(name)` - Get guard function
- `executeGuard(name, context, event)` - Execute guard

### Store Factory Registry

Apps register store factories to avoid circular dependencies:

```typescript
registerDesignAppStoreFactory(factory);
registerKitAppStoreFactory(factory);
registerTypeAppStoreFactory(factory);
registerQualityAppStoreFactory(factory);

getDesignAppStoreFactory();
getKitAppStoreFactory();
getTypeAppStoreFactory();
getQualityAppStoreFactory();
```

### File Providers

File providers abstract file storage for kits, supporting multiple backends.

#### FileProvider Port

```typescript
interface FileProvider {
 upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
 download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
 delete: (kitId: string, fileId: string, path: string) => Promise<void>;
 getUrl: (kitId: string, fileId: string, path: string) => string;
}
```

#### Provider Types

1. **MemoryFileProvider**: In-memory storage using Map (temporary kits)
2. **LocalFileProvider**: IndexedDB storage (browser persistence)
3. **RemoteFileProvider**: HTTP-based storage (server backend)
4. **CompositeFileProvider**: Combines multiple providers with fallback order

#### File Operations

File operations are handled automatically when kit diffs include file changes:

- **Added files**: Uploaded via provider, `remoteUrl` updated in kit
- **Removed files**: Deleted via provider
- **Updated files**: Re-uploaded if blob changed

### Y.js Integration

Y.js provides CRDT-based state synchronization and persistence.

#### Y.js Types

Stores use Y.js types for reactive state:

- `Y.Map` - Key-value maps (state objects)
- `Y.Array` - Arrays (lists, selections)
- `Y.Text` - Text (rarely used)

#### Persistence

- **IndexeddbPersistence**: Local browser persistence for kits
- **YProvider**: Remote synchronization (WebSocket, HTTP)

#### Observers

Y.js observers bridge Y.js changes to store updates:

- **Shallow observers**: Watch top-level map keys
- **Deep observers**: Watch nested changes

Use `createObserver` helper and dispose in `useEffect` cleanup.

#### Transactions

Y.js transactions batch operations:

- All Y.js mutations happen within transactions
- Store `transact` function wraps Y.js transactions
- Origin strings propagate to Y.js for debugging

### Coordinate System

semio uses a left-handed coordinate system that differs from Three.js.

#### semio Coordinate System

- **X-axis**: Right (thumb points right)
- **Y-axis**: Forward (index finger forward)
- **Z-axis**: Up (middle finger up)

#### Three.js Coordinate System

- **X-axis**: Right
- **Y-axis**: Up
- **Z-axis**: Backward (negative)

#### Conversion Functions

- `toThreeRotation()` - Matrix4 for semio → Three.js rotation
- `toSemioRotation()` - Matrix4 for Three.js → semio rotation
- `toThreeQuaternion()` - Quaternion for semio → Three.js
- `toSemioQuaternion()` - Quaternion for Three.js → Semio
- `vectorToThree(v)` - Convert Point/Vector to THREE.Vector3

### Expertise & Tooltips

The UI adapts to user expertise level, showing different tooltip content.

#### Expertise Levels

```typescript
enum Expertise {
 BEGINNER = "beginner", // Full tooltips with tutorials
 NORMAL = "normal", // Standard tooltips
 EXPERT = "expert", // No tooltips
}
```

#### Tooltip Content

Tooltips automatically adapt based on expertise:

- **BEGINNER**: Shows `.beginner` i18n key, tutorials, manuals, hotkeys
- **NORMAL**: Shows standard `.label` i18n key, manuals, hotkeys
- **EXPERT**: No tooltips shown

#### i18n Keys for Tooltips

Each UI element with an `id` prop automatically gets tooltip content from i18n:

- `{id}.label` - Standard label
- `{id}.beginner` - Beginner-friendly description
- `{id}.manual` - Manual page path
- `{id}.tutorial` - Tutorial path
- `{id}.hotkey` - Hotkey display string

#### Tooltip Components

- `<Tooltip>` - Base tooltip wrapper
- `<DescriptionTooltipContent>` - Automatic content from element ID
- `<EnhancedTooltipContent>` - Manual configuration

### Windows

Windows are the primary content areas within the canvas.

#### Window Kind

A window kind is an app-defined content surface identified by a stable id.

#### Window Layout

Window layouts are persisted per app as a JSON string (`windowLayout`).

#### Active Window

The canvas tracks the active window id for focus-sensitive UI.

#### Window Chrome

Window chrome includes action controls for open-in-new-window, maximize/minimize, and close.

### Validation

#### Overview

semio includes a **domain-pure validation system** built entirely in `semio.ts` with **zero JSON dependencies**. All validation logic works with `Kit` objects and produces `KitDiff`-based fixes.

#### Architecture

##### Layer 1: Domain Logic (`semio.ts`)

- **100% JSON-agnostic** - No JSON paths, parsing, or serialization logic
- **Pure functions** - All validation is deterministic and side-effect free
- **Diff-based fixes** - Every fix is a `KitDiff` that can be applied, inverted, and merged
- **Reusable everywhere** - Works in Sketchpad UI, CLI, backend, VS Code, and any other platform

##### Layer 2: Platform Integrations

Each platform provides its own thin wrapper:

- **VS Code Extension** (`js/vscode`) - JSON linter with Quick Fixes
- **Sketchpad UI** - In-app validation panel
- **CLI** - Command-line validation tool
- **Backend** - API validation endpoint

#### Validation Types

##### Core Types

```typescript
type SemioEntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Model" | "Layer" | "Group" | "Stat";
type Severity = "error" | "warning";

interface SemioDomainLocation {
 entityKind: SemioEntityKind;
 entityGuid?: Guid;
 field?: string;
}

interface Fix {
 title: string;
 diff: KitDiff;
}

interface Problem {
 constraintId: string;
 severity: Severity;
 message: string;
 location: SemioDomainLocation;
 relatedGuids?: Guid[];
 fixes: Fix[];
}

interface ValidationResult {
 problems: Problem[];
}
```

##### Validation Context

```typescript
interface ValidationContext {
 kit: Kit;
 typesByGuid: Map<Guid, Type>;
 designsByGuid: Map<Guid, Design>;
 piecesByGuid: Map<Guid, { designGuid: Guid; piece: Piece }>;
 connectorsByTypeGuid: Map<Guid, Connector[]>;
 modelsByTypeGuid: Map<Guid, Model[]>;
}
```

#### Validation Constraints

All validation constraints follow the pattern:

```typescript
type Constraint = (ctx: ValidationContext) => Problem[];
```

##### Default Constraints

#### 1. GUID Uniqueness (`guid-unique`)

**Severity:** Error

All GUIDs must be unique across the entire kit, including:

- Kit
- Types
- Designs
- Pieces
- Connections
- Stats
- Qualities
- Ports
- Files
- Folders

**Fix:** Regenerates a new GUID and updates all references throughout the kit.

#### 2. Type Name Uniqueness (`type-name-unique`)

**Severity:** Error

Types with the same parent must have unique names.

**Fix:** Renames the type with a unique suffix (e.g., "Wall 2", "Wall 3").

#### 3. Design Name Uniqueness (`design-name-unique`)

**Severity:** Error

Designs with the same parent must have unique names.

**Fix:** Renames the design with a unique suffix.

#### 4. Piece Name Uniqueness (`piece-name-unique`)

**Severity:** Error

Pieces within a design must have unique names.

**Fix:** Renames the piece with a unique suffix.

#### 5. Quality Name Uniqueness (`quality-name-unique`)

**Severity:** Error

All qualities within a kit must have unique names.

**Fix:** Renames the quality with a unique suffix.

#### 6. Port Name Uniqueness (`port-name-unique`)

**Severity:** Error

All ports within a kit must have unique names.

**Fix:** Renames the port with a unique suffix.

#### 7. File Name Uniqueness (`file-name-unique`)

**Severity:** Error

All files within a kit must have unique names.

**Fix:** Renames the file with a unique suffix.

#### 8. Folder Name Uniqueness (`folder-name-unique`)

**Severity:** Error

Folders with the same parent must have unique names.

**Fix:** Renames the folder with a unique suffix.

#### 9. Connector Name Uniqueness (`connector-name-unique`)

**Severity:** Error

Connectors within a type must have unique names.

**Fix:** Renames the connector with a unique suffix.

#### 10. Model Name Uniqueness (`model-name-unique`)

**Severity:** Error

Models within a type must have unique names.

**Fix:** Renames the model with a unique suffix.

#### 11. Layer Path Uniqueness (`layer-path-unique`)

**Severity:** Error

Layer paths within a design must be unique.

**Fix:** Renames the layer path with a unique suffix.

#### Uniqueness Requirements Summary

| Entity     | Scope                  | Field | Constraint ID         |
| ---------- | ---------------------- | ----- | --------------------- |
| Kit        | Global                 | guid  | guid-unique           |
| Type       | Siblings (same parent) | name  | type-name-unique      |
| Type       | Global                 | guid  | guid-unique           |
| Design     | Siblings (same parent) | name  | design-name-unique    |
| Design     | Global                 | guid  | guid-unique           |
| Piece      | Within design          | name  | piece-name-unique     |
| Piece      | Global                 | guid  | guid-unique           |
| Connection | Global                 | guid  | guid-unique           |
| Connector  | Within type            | name  | connector-name-unique |
| Model      | Within type            | name  | model-name-unique     |
| Quality    | Global                 | name  | quality-name-unique   |
| Quality    | Global                 | guid  | guid-unique           |
| Port       | Global                 | name  | port-name-unique      |
| Port       | Global                 | guid  | guid-unique           |
| File       | Global                 | name  | file-name-unique      |
| File       | Global                 | guid  | guid-unique           |
| Folder     | Siblings (same parent) | name  | folder-name-unique    |
| Folder     | Global                 | guid  | guid-unique           |
| Layer      | Within design          | path  | layer-path-unique     |
| Stat       | Global                 | guid  | guid-unique           |

#### Usage

##### In Domain Code

```typescript
const result = validateSemioKit(kit);
if (hasSemioErrors(result)) {
 console.error("Validation errors found:", result.problems);
}
```

##### Applying Fixes

```typescript
const problem = result.problems[0];
const fix = problem.fixes[0];
const fixedKit = applyKitDiff(kit, fix.diff);
```

##### Custom Validation

```typescript
const customConstraint: Constraint = (ctx) => {
 const problems: Problem[] = [];
 // Custom validation logic
 return problems;
};

const result = validateSemioKit(kit, {
 constraints: [...defaultConstraints, customConstraint],
});
```

###### Creating New Constraints

1. Define the constraint function following `Constraint` signature
2. Use `semioMakeFix` helper to generate `KitDiff`-based fixes
3. Add to `defaultConstraints` array
4. Document in this section

Example:

```typescript
export const semioCustomConstraint: Constraint = (ctx) => {
 const problems: Problem[] = [];
 // Validation logic
 // Use semioMakeFix to create fixes
 return problems;
};
```

#### Cross-Platform Connectorable Validation

All implementations (TypeScript, Python, C#) produce **identical** validation output for cross-platform compatibility. Problems include fixes with `KitDiff` structures.

##### Format

```json
{
  "problems": [
    {
      "constraintId": "type-name-unique",
      "severity": "error",
      "message": "Duplicate type name \"...\" among siblings.",
      "entityKind": "Type",
      "entityGuid": "...",
      "fixes": [
        {
          "title": "Rename \"...\"",
          "diff": { "types": { "updated": [...] } }
        }
      ]
    }
  ]
}
```

##### Implementation

- **TypeScript**: `toValidationResult()`, `serializeValidationResult()`, `areValidationResultsEqual()`
- **Python**: `ValidationResult.toDict()`, `ValidationResult.serialize()`, `areValidationResultsEqual()`
- **C#**: `SemioValidator.ValidateKit()`, `ValidationResult.Serialize()`, `ValidationResult.AreEqual()` (fix comparison pending)

##### Test Data

- `assets/semio/kit_invalid.json` - Invalid kit with all validation constraint breachs
- `assets/semio/validation.json` - Expected output (sorted by constraintId, then entityGuid)

##### Updating Metabolism Assets

```bash
npx tsx scripts/update-metabolism.tsx
```

This script consolidates all Metabolism asset generation:

- Regenerates `metabolism.zip` with updated SQL schema and copies to all public folders
- Generates diff files (`diff_kit_metabolism.json`, `diff_kit_metabolism_inverted.json`, `kit_metabolism_diffed.json`)
- Generates `validation.json` from `kit_invalid.json`

##### Validation Constraints

| Constraint ID           | Description                                  |
| ----------------------- | -------------------------------------------- |
| `guid-unique`           | All GUIDs must be unique across the kit      |
| `type-name-unique`      | Type names must be unique among siblings     |
| `design-name-unique`    | Design names must be unique among siblings   |
| `piece-name-unique`     | Piece names must be unique within a design   |
| `connector-name-unique` | Connector names must be unique within a type |
| `model-name-unique`     | Model names must be unique within a type     |
| `quality-name-unique`   | Quality names must be unique                 |
| `port-name-unique`      | Port names must be unique                    |
| `file-name-unique`      | File names must be unique                    |
| `folder-name-unique`    | Folder names must be unique among siblings   |
| `layer-path-unique`     | Layer paths must be unique within a design   |

##### Fix Comparison Notes

- New GUIDs in `guid-unique` fixes can differ between implementations
- Fix diffs are normalized (GUIDs replaced with `<GUID>`) before comparison
- C# fix generation is pending; comparison skips fix diff for now

## Borders

UI borders are semantic kinds so the same component can be restyled without rewriting class lists:

- `border-element` is the default UI element border kind and matches the hover color.
- `border-window` is the window border kind and matches the normal border color.

GoldenLayout stack windows render the window frame as an inset 1px stroke (overlay frame) so all four edges stay visible even when containers clip their contents.

## Background levels

- The base canvas, windows, panels, and temporary surfaces use different background levels.
- In a multi-window layout, exactly one window is active; its surface uses an active background tint and embedded table views inherit that surface background.

Use them with normal Tailwind border utilities like `border`, `border-t`, `border-l`, and `divide-*`.

## Validation System

semio includes a **domain-pure validation system** built entirely in `semio.ts` with zero JSON dependencies. All validation logic works with `Kit` objects and produces `KitDiff`-based fixes.

**11 Validation Constraints:**

1. **GUID Uniqueness** - All GUIDs must be unique globally
2. **Type Name Uniqueness** - Sibling types must have unique names
3. **Design Name Uniqueness** - Sibling designs must have unique names
4. **Piece Name Uniqueness** - Pieces in a design must have unique names
5. **Quality Name Uniqueness** - All qualities must have unique names
6. **Port Name Uniqueness** - All ports must have unique names
7. **File Name Uniqueness** - All files must have unique names
8. **Folder Name Uniqueness** - Sibling folders must have unique names
9. **Connector Name Uniqueness** - Connectors in a type must have unique names
10. **Model Name Uniqueness** - Models in a type must have unique names
11. **Layer Path Uniqueness** - Layers in a design must have unique paths

**Platform Integrations:**

- **VS Code Extension** - Kit validation with Quick Fixes, breach diagnostics with auto-refresh on save
- **Sketchpad UI** - In-app validation panel (planned)
- **CLI** - Command-line validation tool (planned)
- **Backend** - API validation endpoint (planned)

**Usage:**

```typescript
import { validateSemioKit, applyKitDiff } from "semio/js";

const result = validateSemioKit(kit);
if (result.problems.length > 0) {
 const fix = result.problems[0].fixes[0];
 const fixedKit = applyKitDiff(kit, fix.diff);
}
```

See [`AGENTS.md`](AGENTS.md#validation) for complete technical documentation.

## Sketchpad

### Sketchpad transactions

- `semio-elements/ui/elements.tsx` provides `TransactionProvider` and `useTransaction()` for UI-scoped transactions.
- Sketchpad elements (`Input`, `Textarea`, `Select`, `Slider`, `Stepper`, `Combobox`, ...) use `useTransaction()` internally and do not accept a `transaction` prop.
- Apps wrap their UI subtree with `TransactionProvider` using the appropriate transaction hook so all descendant elements participate in undo/redo consistently.

### Sketchpad selection + hover visuals

- `semio-elements/ui/elements.tsx` `Geometry` renders selection/hover colors even when a base `color` is provided (it is treated as the non-interactive default).
- Hover and selection state for Home/Kit/Design/Type/Quality/Docs/Feedback is stored in the Sketchpad state machine; UI rows and diagram nodes dispatch hover events and visuals read from machine state.
- `semio-elements/ui/elements.tsx` `Table` exposes row hover callbacks so apps can forward row enter/leave events into their state machine commands.
- `semio/js/sketchpad/Design.tsx` diagram nodes use `ring-*` (not `ring-inset`) so hover/selection rings remain visible with `AvatarFallback` backgrounds.
- Selection composition is unified across apps through shared semantics: `replace`, `additive`, `subtractive`, `intersect`.
- Shared selection composition applies stable ordering (`previous` order first, then first-seen additive entries) and dedupes ids at composition boundaries.
- Selection modifier resolution is unified across apps: `Shift => additive`, `Alt/Ctrl/Meta => subtractive`, combined modifiers => `intersect`.

### Sketchpad windows

- Window spacing uses the shared unit sizing system: a single unit gap between windows and a single unit margin between windows and the canvas edge.
- `semio/js/sketchpad/Sketchpad.tsx` `Canvas` applies `p-single` (1 unit) and window containers (`HorizontalWindows` / `VerticalWindows`) apply `gap-single` (1 unit).
- GoldenLayout window gaps use splitters sized to 1 unit and window borders are applied to the stack container via an inset 1px stroke; `Window` uses `kind="layout"` inside GoldenLayout to avoid nested borders.
- Window chrome controls are rendered as Action UI elements and forwarded to the underlying layout system when needed.
- Window surfaces paint the only filled background surface; surrounding UI and overlays remain transparent and rely on borders/blur.
- Each app registers its supported window kinds and provides a default layout; per-app `windowLayout` is persisted as a JSON string.

### Kit app artifact creation

- `semio/js/sketchpad/Kit.tsx` create actions for `ports`, `tags`, `concepts`, and `folders` set the `kind` filter and selection to the newly created entity so the details panel opens immediately.
- Default names are resolved via i18n labels: `semio.sketchpad.app.port.defaultName`, `semio.sketchpad.app.tag.defaultName`, `semio.sketchpad.app.concept.defaultName`.
- `semio/js/sketchpad/Kit.tsx` details panel sections are registered dynamically; remove all possible section ids (including conditional variants) before adding the active one and mirror removals in the effect cleanup.
- `semio/js/sketchpad/Kit.tsx` details panel section content that uses `useKit()` is wrapped in `KitScopeProvider` so read-only detail sections can resolve kit data consistently.
- `semio/js/sketchpad/Kit.tsx` read-only artifact detail fields reuse the same `id` values as the corresponding app details (Type/Design) so i18n/tooltips stay centralized.

### Sketchpad state

- `semio/js/sketchpad/Sketchpad.tsx` exposes a single `sketchpadMachine` actor that owns all Sketchpad UI state (`SketchpadState` + app slices like Home/Kit/Design/Type/Quality/Tutorial).
- Y.js is reserved for Kit data synchronization (per-kit `KitStore` documents).
- Sketchpad UI state is persisted locally via `localStorage` key `semio.sketchpad.state.<id>` (no Y.js dependency for settings/navigation/panel sizes).
- Global interaction mode is stored as `SketchpadState.device` and controlled via `useDevice()` / `SET_DEVICE` with i18n IDs `semio.sketchpad.settings.device.*`.
- `layout` naming is reserved for window layout configs (GoldenLayout) and the `Layout` component in `semio-elements/ui/elements.tsx`.

KitStore keeps kit concepts in the `yConcepts` array as `ConceptStore` entries so snapshots expose full concept data (name, description, icon, attributes) and persistence rehydrates from that array instead of guid placeholders.

## UI System

The UI uses a standardized unit-based sizing system for consistent spacing and sizing across all components. The system is derived from the `--spacing` CSS variable and uses Tailwind utility classes.

### UI Levels

The UI consists of five hierarchical levels, each with its own z-index and background styling:

1. **Base** - The foundation layer for the main canvas and background
2. **Window** - Container level for windows within the canvas
3. **Panel** - Floating panels and sidebars
4. **Overlay** - Transparent layer (only affects z-index, not background)
5. **Temporary** - Tooltips, dropdowns, modals, and transient UI

Each level (except overlay) has a background surface. Components use the `LevelProvider` context and `useLevel()` hook to access the current level, with helper functions for consistent styling:

The global Sketchpad shell is wrapped in `LevelProvider level="base"` so top-level chrome (Navbar/Footer) always resolves `bg-base`.

Panels are rendered under `LevelProvider level="panel"` so panel chrome and content consistently resolves `bg-panel`.

- `getLevelBgClass(level)` - Returns background class (e.g., `bg-base`, `bg-window`, `bg-panel`, `bg-temporary`)
- `getLevelHoverClass(level)` - Returns hover background class (e.g., `hover:bg-hover-base`)
- `getLevelZClass(level)` - Returns z-index class (e.g., `z-base`, `z-window`, `z-panel`, `z-overlay`, `z-temporary`)
- `getLevelActiveHoverClass(level)` - Returns active state hover class (e.g., `data-[state=active]:bg-hover-base`)

### Size Constants

All size constants are defined in `semio-elements/ui/globals.css` and derived from `--spacing`:

- **Single**: 1 unit (e.g. `gap-1`) - spacing between elements and between icon and element
- **Tiny**: 3 units (e.g. `h-tiny`, `w-tiny`, `text-tiny`) - icon size in actions, action text size
- **Small**: 5 units (e.g. `h-small`, `w-small`) - actions, avatars, Strip items
- **Medium**: 7 units (e.g. `h-medium`, `w-medium`) - buttons, toggles, inputs, sliders, steppers, Footer, table rows, Strip
- **Large**: 9 units (e.g. `h-large`, `w-large`) - Band, Navbar
- **Huge**: 11 units (e.g. `h-11`) - height of navigation buttons at bottom of docs pages
- **Mega**: 13 units (e.g. `w-mega`) - width of toggles with actions (toggles with dropdown or action buttons)
- **Giga**: 15 units (e.g. `w-giga`) - reserved for future use

### Guidelines

- All spacing between elements uses Tailwind unit classes (e.g. `gap-1`, `p-1`, `m-1`)
- Icons within actions use `h-tiny w-tiny` and action labels use `text-tiny`
- Interactive elements (buttons, toggles, inputs) use `h-medium`
- Toggles with actions (dropdown or action button) use `h-medium w-mega`
- Bands and navbars use `h-large`
- Strips, footers, and table rows use `h-medium`
- Table body cells use `px-single py-0` and center their content so `h-medium` row height stays fixed even when cells contain `h-medium` controls (toggles/inputs)
- Table headers use `h-large`
- Large navigation elements use `h-11` (huge)
- Navbar panel toggles list panels in the fixed order Details, Chat, Settings across all apps

The unit system automatically adapts based on the `--spacing` mode (compact vs touch).

## Platform Compatibility

The code runs in different environments (different browsers, electron, mobile/desktop/tablet). Platform-specific functionality MUST be generalized and provided as props to Sketchpad. NEVER hardcode platform-specific behavior or APIs directly in bundles.

## Resources

<details>
<summary><strong>📚 Resources:</strong></summary>

- [JavaScript](https://developer.mozilla.org/docs/Web/JavaScript) - `mdn`
- [Node](https://nodejs.org/en/learn/getting-started/introduction-to-nodejs) - `intro`
- [TypeScript](https://www.npmjs.com/package/typescript) - `npm`
  - [Docs](https://www.typescriptlang.org/docs) - `official`
  - [Issues](https://github.com/microsoft/TypeScript/issues) - `github`
- [Wasm](https://developer.mozilla.org/docs/WebAssembly) - `mdn`
  - [C/C++](https://developer.mozilla.org/docs/WebAssembly/Guides/C_to_Wasm) - `guide`
  - [Rust](https://developer.mozilla.org/docs/WebAssembly/Rust_to_Wasm) - `guide`

</details>

## Workspaces

- `semio/js` - Shared TypeScript/React codebase (Sketchpad UI, components, libs).
- `semio/desktop` - Electron desktop shell.
- `js/sketchpad` - Standalone Sketchpad web app shell.
- `semio/docs` - Documentation site.
- `semio/play` - Playground.
- `semio-repo/vscode` - VS Code extension.

## Channels

<details>
<summary><strong>📺 Channels:</strong></summary>

- [WebDevSimplified](https://www.youtube.com/@WebDevSimplified) - `beginner`
- [Jack Herrington](https://www.youtube.com/@jherr) - `react`
- [The Net Ninja](https://www.youtube.com/@NetNinja) - `everything`
- [Fireship](https://www.youtube.com/@Fireship) - `quaffable`
- [Theo - t3.gg](https://www.youtube.com/@t3dotgg) - `opinionated`
- [The Primeagen](https://www.youtube.com/@ThePrimeagen) - `entertainment`

</details>

# 💯Requirements
