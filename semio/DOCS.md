# 📚 Docs

## [👤semio🏪assets](semiorepo://p/u/semio/b/a/assets)

## Badges

Each badge is created with [shields.io](https://shields.io) with style `flat-square` and semio colors.

1. Copy the `*.shields` file of an existing badge 📄
1. Open and download the `*.svg` file ⬇

## Fonts

1. Search font on [fontsource.org](https://fontsource.org) 🔍
1. Hit `Download` and extract zip file 📂
1. Use kebaberized font name as folder name and remove everything else (such as version numbers) ➖
1. Merge all types in one folder (`ttf`, `webfonts`, …) - they won't collide due to different extensions 🗃️️
1. Remove all parts that repeat everywhere (such as common name prefix, single weighted fonts, …) 💯

## Icons

1. Open [favicongenerator.net](https://www.favicongenerator.io) 🔍
1. Select `Circle` as `Background Shape` ⏺
1. Select `Anta` as `Font Family` 📃
1. Enter the `Code` that you find in the [dictionary](https://github.com/usalu/semio/tree/main/meta/dictionary.csv)
1. Adjust the `Font Size` to the largest so that the space to the side is the same as the thickness of the stroke 🖊
1. Toggle `Enable SVG` on 🔳
1. Hit `Generate Favicon` and download the zip file to `assets/icons/temp/NAME.zip` where `NAME` is the lowercase name and verb of the icon 📂
1. Repeat the process for all icons 🔁
1. Run `build icons` in the debugger of vscode 🔨

## Kits

`assets/index.ts` is the shared entry point for `semio/assets`. It re-exports the icon layer plus the Metabolism kit fixtures and helper constants. The kit fixtures are available as `MetabolismKit`, `MetabolismKitDiff`, `MetabolismKitDiffed`, `MetabolismKitDiffInverted`, `InvalidKit`, and `InvalidKitValidation`, while each kit entity list is exposed through `MetabolismKitTypes`, `MetabolismKitDesigns`, `MetabolismKitPorts`, `MetabolismKitQualities`, `MetabolismKitFiles`, `MetabolismKitFolders`, `MetabolismKitAuthors`, `MetabolismKitTags`, `MetabolismKitConcepts`, `MetabolismKitAttributes`, and the dedicated `MetabolismKitNakaginCapsuleTowerDesigns`.

Lookup tables `MetabolismKitTypesByGuid`, `MetabolismKitTypesByName`, `MetabolismKitDesignsByGuid`, `MetabolismKitDesignsByName`, `MetabolismKitPortsByGuid`, and `MetabolismKitPortsByName` provide direct access to every type, design, and port without filtering.

## [👤semio🖱️desktop](semiorepo://p/u/semio/b/u/desktop)

<details>
<summary><strong>📚 Resources:</strong></summary>

- [Electron](https://www.npmjs.com/package/electron) - `npm`
  - [Docs](https://www.electronjs.org/docs) - `official`
  - [API](https://www.electronjs.org/docs/latest/api/app) - `reference`
  - [Issues](https://github.com/electron/electron/issues) - `github`
- [Electron Forge](https://www.npmjs.com/package/electron-forge) - `npm`
  - [Docs](https://www.electronforge.io/docs) - `official`
  - [Issues](https://github.com/jclab/electron-forge/issues) - `github`

</details>

## [👤semio📚engine](semiorepo://p/u/semio/b/l/engine)

## Files

- `engine.py` - Main engine module with Kit parsing, validation, transformation, dev-mode startup flag, and stdio MCP startup flag
- `engine.test.py` - Unit tests for engine functionality
- `generate-schemas.ts` - Generates GraphQL, JSON, and SQL schemas from TypeScript definitions
- `sqliteschema.ts` - SQLite schema generation utilities

## [👤semio📚js](semiorepo://p/u/semio/b/l/js)

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

- `assets/semio/invalid.kit.semio.json` - Invalid kit with all validation constraint breachs
- `assets/semio/validation.semio.json` - Expected output (sorted by constraintId, then entityGuid)

##### Updating Metabolism Assets

```bash
npx tsx scripts/update-metabolism.tsx
```

This script consolidates all Metabolism asset generation:

- Regenerates `metabolism.zip` with updated SQL schema and copies to all public folders
- Generates diff files (`metabolism.kit.diff.semio.json`, `metabolism.kit.diff.inverted.semio.json`, `metabolism.kit.diffed.semio.json`)
- Generates `validation.semio.json` from `invalid.kit.semio.json`

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

## [👤semio📚net](semiorepo://p/u/semio/b/l/net)

## Semio.cs

Core library containing all model definitions, validation, serialization, and the Meta class for reflection-based metadata.

## Semio.Grasshopper.cs

Grasshopper plugin providing components for constructing, deconstructing, and modifying semio models.

### Architecture

The plugin uses a component hierarchy with base classes that provide default behavior:

- **`ModelComponent<TParam, TGoo, TModel>`**: Base class for model components with virtual methods for customization
- **`IdComponent`**, **`DiffComponent`**: Specialized base classes for Id and Diff model types
- **`SerializeComponent`**, **`DeserializeComponent`**: Base classes for serialization components

### Component Structure

Each model type has a set of classes:

- **`*Goo`**: Grasshopper wrapper for the model type with cast methods
- **`*Param`**: Grasshopper parameter definition
- **`*Component`**: Main model component for construct/deconstruct/modify
- **`Serialize*Component`**: JSON serialization component
- **`Deserialize*Component`**: JSON deserialization component

### Hardcoded Parameters

Components use virtual methods to define their inputs/outputs:

- `RegisterModelInputParams(pManager)`: Define input parameters
- `RegisterModelOutputParams(pManager)`: Define output parameters
- `GetModelData(DA, model)`: Read input data into model
- `SetModelData(DA, model)`: Write model data to outputs

Components can override these to hardcode their parameter structure, ensuring stable input/output definitions across schema changes.

## [👤semio🌐docs](semiorepo://p/u/semio/b/w/docs)

<details>
<summary><strong>📚 Resources:</strong></summary>

- [Markdown](https://www.markdownguide.org) - `guide`
- [MDX](https://mdxjs.com/docs) - `official`
- [Astro](https://astro.build/docs) - `official`
- [Starlight](https://starlight.astro.build) - `official`

</details>

<details>
<summary><strong>📺 Channels:</strong></summary>

- [Coding in Public](https://www.youtube.com/@CodinginPublic) - `astro`

</details>

## [👤semio🖱️sketchpad](semiorepo://p/u/semio/b/u/sketchpad)

Detailed app/runtime guidance remains in [`./sketchpad/README.md`](./sketchpad/README.md). This bundle now owns the Sketchpad React runtime exported as `@semio/sketchpad`, while `@semio/js` keeps only domain logic.

## [👤semio📚3dm🛅semiorhino💻semiorhino](semiorepo://p/u/semio/b/l/3dm/fd/req/Semio.Rhino/f/Semio.Rhino.cs)

Rhino 8 plugin hosting a WebView2 panel for importing semio kits and models.

## [👤semio📚3dm🛅semiorhinotests💻tests](semiorepo://p/u/semio/b/l/3dm/fd/req/Semio.Rhino.Tests/f/Tests.cs)

Unit tests for the Semio.Rhino bridge registry and layer service.

## [👤semio📚3dm🛅ui🗃️src💻rhinopanel](semiorepo://p/u/semio/b/l/3dm/fd/req/ui/fd/org/src/f/RhinoPanel.tsx)

Main panel component rendering the semio kit/type/model tree view for Rhino.

## [👤semio📚3dm🛅ui🗃️src💻bridge](semiorepo://p/u/semio/b/l/3dm/fd/req/ui/fd/org/src/f/bridge.ts)

Bridge client for JSON-RPC style communication with the native Rhino C# host.

## [👤semio📚3dm🛅ui🗃️src💻bridge🔖webviewglobal](semiorepo://p/u/semio/b/l/3dm/fd/req/ui/fd/org/src/f/bridge.ts/s/WebViewGlobal)

Global type augmentation for the WebView2 chrome.webview API.

## [👤semio📚3dm🛅ui🗃️src💻index](semiorepo://p/u/semio/b/l/3dm/fd/req/ui/fd/org/src/f/index.tsx)

Entry point for the semio 3dm React UI embedded in Rhino WebView2.

## [👤semio🏪assets💻icons](semiorepo://p/u/semio/b/a/assets/f/icons.ts)

Re-exports Lucide React icons with domain-specific semantic aliases.

## [👤semio🏪assets💻icons🔖exports](semiorepo://p/u/semio/b/a/assets/f/icons.ts/s/Exports)

Re-exports of Lucide React icons with semantic aliases for the UI.

## [👤semio🏪assets💻index](semiorepo://p/u/semio/b/a/assets/f/index.ts)

Barrel export for all asset modules including icons, fonts, models and images.

## [👤semio🏪assets🛅logo💻logo](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts)

Generates animated SVG logo from static SVG input with keyframe sequences.

## [👤semio🏪assets🛅logo💻logo🔖parsesvg](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Parse%20SVG)

Parses an SVG file and returns keyframe data with group transforms and paths.

## [👤semio🏪assets🛅logo💻logo🔖generatekeyframesequence](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Generate%20Keyframe%20Sequence)

Generates a palindromic keyframe sequence with triple repetition per frame.

## [👤semio🏪assets🛅logo💻logo🔖createanimatedsvg](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Create%20Animated%20SVG)

Creates an animated SVG file with SMIL animations from keyframe data.

## [👤semio🏪assets🛅logo💻logo🛠️parsesvgfile](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/d/i/parseSVGFile)

Parses an SVG file and returns keyframe data with group transforms and paths.

## [👤semio🏪assets🛅logo💻logo🛠️generatekeyframesequence](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/d/i/generateKeyframeSequence)

Generates a palindromic keyframe sequence with triple repetition per frame.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻fileemptyregion](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_empty_region.tsx)

An empty region TypeScript file for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixable](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixable.tsx)

A fixable TypeScript file for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixableexpected](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixable_expected.tsx)

A fixable TypeScript file for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.cs)

A fixed CSharp class for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🛠️fixedclass](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.cs/d/i/FixedClass)

/ <summary>Represents a fixed value container.</summary>
/ <remarks>
/ [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖classes🛠️fixedclass](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.cs/s/Classes/d/i/FixedClass)
/ </remarks>

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go)

A fixed Go module for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖package](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go/s/Package)

Package declaration for fixed module.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖functions](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go/s/Functions)

Utility functions for fixed values.
FixedValue returns a constant integer.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🛠️fixedvalue](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.go/d/i/FixedValue)

FixedValue returns a constant integer.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.py)

A fixed Python module for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖functions](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.py/s/Functions)

Utility functions for fixed values.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.tsx)

A fixed TypeScript component for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖types](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.tsx/s/Types)

Type definitions for the fixed component.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixed🔖components](semiorepo://p/u/semio/b/a/assets/fd/org/repo/fd/org/some/fd/org/folder/f/file_fixed.tsx/s/Components)

Rendering components for fixed types.

## [👤semio🖱️desktop💻forgeenvd](semiorepo://p/u/semio/b/u/desktop/f/forge.env.d.ts)

Type declarations for Electron Forge environment variables.

## [👤semio🖱️desktop💻forgeenvd🔖electronfuses](semiorepo://p/u/semio/b/u/desktop/f/forge.env.d.ts/s/Electron%20Fuses)

Type declarations for Electron Forge fuse options.

## [👤semio🖱️desktop💻main](semiorepo://p/u/semio/b/u/desktop/f/main.ts)

Entry point for the Electron main process managing windows and lifecycle.

## [👤semio🖱️desktop💻main🔖mainprocess](semiorepo://p/u/semio/b/u/desktop/f/main.ts/s/Main%20Process)

Electron main process that creates the browser window and registers IPC handlers.

## [👤semio🖱️desktop💻preload](semiorepo://p/u/semio/b/u/desktop/f/preload.ts)

Electron preload script exposing safe APIs to the renderer process.

## [👤semio🖱️desktop💻preload🔖preload](semiorepo://p/u/semio/b/u/desktop/f/preload.ts/s/Preload)

Electron preload script exposing window controls and OS APIs to the renderer.

## [👤semio🖱️desktop💻renderer](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx)

Entry point for the Electron renderer process mounting the React app.

## [👤semio🖱️desktop💻renderer🔖renderer](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer)

Electron renderer process that mounts the Sketchpad React app with window controls.

## [👤semio📚engine💻build](semiorepo://p/u/semio/b/l/engine/f/build.ts)

Build script for the semio engine Python package.

## [👤semio📚engine💻engine🔖mcp](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp)

Call start_working_in_local_kit(path) first; then use start_working_in_design/start_working_in_type to scope further.

## [👤semio📚engine💻generateschemas](semiorepo://p/u/semio/b/l/engine/f/generate-schemas.ts)

Generates JSON schemas from the engine's Python models.

## [👤semio📚engine💻postbuild](semiorepo://p/u/semio/b/l/engine/f/post-build.ts)

Post-build script for engine artifact processing and packaging.

## [👤semio📚engine💻sqliteschema](semiorepo://p/u/semio/b/l/engine/f/sqliteschema.ts)

Exports the SQLite schema definition for the engine database.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs)

Main Grasshopper plugin providing domain components for Rhino.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖importedrhinoobjectresolution](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/ImportedRhinoObjectResolution)

/ <summary>
/ Resolves a single imported Rhino model object by metadata identifier.
/
/ Specs:
/ Tries native object IDs first, then deterministic fallback IDs ("rhino-object-{index}") used by import metadata.
/ Returns null when no matching source model object can be found.
/ </summary>

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️goo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/Goo)

/ Generic Grasshopper data wrapper for semio entity types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️goo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/Goo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️param](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/Param)

/ Generic Grasshopper parameter for semio entity types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️param](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/Param)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️enumgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EnumGoo)

/ Generic Grasshopper data wrapper for enum values.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️enumgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EnumGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️enumparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EnumParam)

/ Generic Grasshopper parameter for enum values.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️enumparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EnumParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️passthroughcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/PassthroughComponent)

/ Abstract Grasshopper component that passes input through transformation.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️passthroughcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/PassthroughComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️idgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/IdGoo)

/ Generic Grasshopper data wrapper for entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️idgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/IdGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️idparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/IdParam)

/ Generic Grasshopper parameter for entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️idparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/IdParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️idcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/IdComponent)

/ Abstract Grasshopper component for constructing entity IDs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️idcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/IdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️diffgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DiffGoo)

/ Generic Grasshopper data wrapper for entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️diffgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/DiffGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️diffparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DiffParam)

/ Generic Grasshopper parameter for entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️diffparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/DiffParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️diffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DiffComponent)

/ Abstract Grasshopper component for constructing entity diffs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️diffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/DiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️changegoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/ChangeGoo)

/ Generic Grasshopper data wrapper for semio change types.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️changeparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/ChangeParam)

/ Generic Grasshopper parameter for semio change types.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️changecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/ChangeComponent)

/ Abstract Grasshopper component for constructing entity changes.

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️applydiffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/ApplyDiffComponent)

/ Abstract Grasshopper component for applying an entity diff to an entity.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️applydiffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/ApplyDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️serializecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/SerializeComponent)

/ Abstract Grasshopper component for serializing entities to JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️serializecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/SerializeComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️deserializecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DeserializeComponent)

/ Abstract Grasshopper component for deserializing entities from JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️deserializecomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/DeserializeComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️serializediffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/SerializeDiffComponent)

/ Abstract Grasshopper component for serializing diffs to JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️serializediffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/SerializeDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️deserializediffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DeserializeDiffComponent)

/ Abstract Grasshopper component for deserializing diffs from JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️deserializediffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/DeserializeDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️serializeidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/SerializeIdComponent)

/ Abstract Grasshopper component for serializing entity IDs to JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️serializeidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/SerializeIdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️deserializeidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/DeserializeIdComponent)

/ Abstract Grasshopper component for deserializing entity IDs from JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️deserializeidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/DeserializeIdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitygoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityGoo)

/ Generic Grasshopper data wrapper with built-in entity validation.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entitygoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityParam)

/ Generic Grasshopper parameter with entity validation support.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entityparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitycomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityComponent)

/ Abstract Grasshopper component for constructing validated entities.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entitycomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityidgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityIdGoo)

/ Generic Grasshopper data wrapper for validated entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entityidgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityIdGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityidparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityIdParam)

/ Generic Grasshopper parameter for validated entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entityidparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityIdParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entityidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityIdComponent)

/ Abstract Grasshopper component for constructing validated entity IDs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entityidcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityIdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitydiffgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityDiffGoo)

/ Generic Grasshopper data wrapper for validated entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entitydiffgoo](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityDiffGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitydiffparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityDiffParam)

/ Generic Grasshopper parameter for validated entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entitydiffparam](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityDiffParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshopper🛠️entitydiffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/d/i/EntityDiffComponent)

/ Abstract Grasshopper component for constructing validated entity diffs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshopper🔖bases🛠️entitydiffcomponent](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/Semio.Grasshopper.cs/s/Bases/d/i/EntityDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻buildvaluelists](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build-value-lists.ts)

Generates Grasshopper value list presets from domain data.

## [👤semio📚gh🛅semiograsshopper💻build](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/f/build.ts)

Build script for the Grasshopper plugin assembly.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻build](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/build.ts)

Build script for Yak package distribution of the Grasshopper plugin.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻login](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/login.ts)

Authenticates with the Yak package server for plugin publishing.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publish](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/publish.ts)

Publishes the Grasshopper plugin package to the Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpush](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/test-push.ts)

Tests the Yak package push workflow for the Grasshopper plugin.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearch](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/test-search.ts)

Tests Yak package search functionality for the Grasshopper plugin.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearch🔖script](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/test-search.ts/s/Script)

Test script for searching the Yak package manager test server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyank](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/unyank.ts)

Restores a previously yanked version of the Grasshopper Yak package.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yank](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper/fd/org/yak/f/yank.ts)

Yanks a specific version of the Grasshopper Yak package from the registry.

## [👤semio📚go💻kitsqlite](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go)

SQLite-backed persistence layer for kit import and export operations.

## [👤semio📚go💻kitsqlite🛠️kitfromsqlite](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitFromSqlite)

KitFromSqlite reads a Kit from a SQLite database file

## [👤semio📚go💻kitsqlite🛠️loadtypes](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadTypes)

loadTypes loads all types belonging to a kit from the database

## [👤semio📚go💻kitsqlite🛠️loaddesigns](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadDesigns)

loadDesigns loads all designs belonging to a kit from the database

## [👤semio📚go💻kitsqlite🛠️loadpieces](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadPieces)

loadPieces loads all pieces belonging to a design from the database

## [👤semio📚go💻kitsqlite🛠️loadconnections](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadConnections)

loadConnections loads all connections belonging to a design from the database

## [👤semio📚go💻kitsqlite🛠️loadconnectors](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/loadConnectors)

loadConnectors loads all connectors belonging to a type from the database

## [👤semio📚go💻kitsqlite🛠️kittosqlite](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitToSqlite)

KitToSqlite writes a Kit to a SQLite database file

## [👤semio📚go💻kitsqlite🛠️kitfromzip](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitFromZip)

KitFromZip extracts a Kit and its files from a zip archive

## [👤semio📚go💻kitsqlite🛠️buildfilepath](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/buildFilePath)

buildFilePath constructs the file path from the folder hierarchy and file name

## [👤semio📚go💻kitsqlite🛠️buildfolderpath](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/buildFolderPath)

buildFolderPath constructs the folder path from the folder hierarchy

## [👤semio📚go💻kitsqlite🛠️blobencode](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/blobEncode)

blobEncode encodes bytes to a data URI string with the mime type inferred from filename.
Falls back to "application/octet-stream" when the extension is unknown.

## [👤semio📚go💻kitsqlite🛠️mimefromfilename](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/mimeFromFilename)

mimeFromFilename returns the mime type for a given filename based on its extension.
Returns "application/octet-stream" when the extension is unknown.

## [👤semio📚go💻kitsqlite🛠️blobdecode](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/blobDecode)

blobDecode decodes a data URI string to bytes.
Supports "data:<mime>;base64,<data>" format as well as raw base64 for backwards compatibility.

## [👤semio📚go💻kitsqlite🛠️kittozip](semiorepo://p/u/semio/b/l/go/f/kit_sqlite.go/d/i/KitToZip)

KitToZip packages a Kit and its files into a zip archive

## [👤semio📚go💻semio](semiorepo://p/u/semio/b/l/go/f/semio.go)

Core domain library in Go implementing the semio data model and operations.

## [👤semio📚go💻semio🔖utils](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Utils)

Guid generates a new random 128-bit hex-encoded unique identifier.
ptrString returns a pointer to the given string value.

## [👤semio📚go💻semio🔖entityids](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs)

AttributeId identifies an attribute entity by GUID.

## [👤semio📚go💻semio🔖weakentities](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities)

Coord represents a 2D coordinate with U and V components.

## [👤semio📚go💻semio🔖attribute](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Attribute)

Attribute represents a key-value metadata entry with optional definition.

## [👤semio📚go💻semio🔖location](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Location)

Location represents a geographic location with longitude, latitude and optional altitude.

## [👤semio📚go💻semio🔖author](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Author)

Author represents a named authorship entity with optional email.

## [👤semio📚go💻semio🔖file](semiorepo://p/u/semio/b/l/go/f/semio.go/s/File)

File represents a file reference entity with name, remote URL and metadata.

## [👤semio📚go💻semio🔖folder](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Folder)

Folder represents a folder hierarchy entity with name and parent reference.

## [👤semio📚go💻semio🔖benchmark](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Benchmark)

Benchmark represents a named metric threshold with min and max bounds.

## [👤semio📚go💻semio🔖quality](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Quality)

QualityKind is a bitfield enum for quality scope classification.

## [👤semio📚go💻semio🔖port](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Port)

Port represents a named connector port with compatible port references.

## [👤semio📚go💻semio🔖prop](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Prop)

Prop represents a quality property value with optional unit.

## [👤semio📚go💻semio🔖tag](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Tag)

Tag represents a named classification tag with optional description and icon.

## [👤semio📚go💻semio🔖concept](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Concept)

Concept represents a named categorization concept with optional description.

## [👤semio📚go💻semio🔖model](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Model)

Model represents a 3D model reference associated with a file and tags.

## [👤semio📚go💻semio🔖connector](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Connector)

Connector represents a spatial connection point on a type with position and direction.

## [👤semio📚go💻semio🔖type](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Type)

Type represents a component type with models, connectors and hierarchical inheritance.

## [👤semio📚go💻semio🔖layer](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Layer)

Layer represents a named layer with visibility, lock and color properties.

## [👤semio📚go💻semio🔖piece](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Piece)

Piece represents a placed component instance within a design.

## [👤semio📚go💻semio🔖group](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Group)

Group represents a named collection of pieces within a design.

## [👤semio📚go💻semio🔖side](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Side)

Side represents one end of a connection referencing a piece and optional connector.

## [👤semio📚go💻semio🔖connection](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Connection)

Connection represents a spatial relationship between two pieces with transform parameters.

## [👤semio📚go💻semio🔖stat](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Stat)

Stat represents a statistical quality measurement with min and max bounds.

## [👤semio📚go💻semio🔖design](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Design)

Design represents an assembly of pieces, connections, layers and groups.

## [👤semio📚go💻semio🔖kit](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Kit)

Kit represents the root container for all domain entities.

## [👤semio📚go💻semio🔖serialization](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Serialization)

SerializeKit marshals a kit to indented JSON bytes.

## [👤semio📚go💻semio🔖helpers](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Helpers)

FindTypeInKit returns a pointer to the type with the given GUID or nil.

## [👤semio📚go💻semio🔖factories](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Factories)

NewKit creates a new kit with the given name and a generated GUID.

## [👤semio📚go💻semio🔖kitoperations](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations)

AreKitsEqual compares two kits for structural equality.

## [👤semio📚go💻semio🔖kitchangehelpers](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers)

AddTypeToKit creates a change that adds a single type to a kit.

## [👤semio📚go💻semio🔖validation](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Validation)

SemioEntityKind enumerates the kinds of semio domain entities.

## [👤semio📚go💻semio🔖validationserialization](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Validation%20Serialization)

ProblemSerialized is the JSON-serializable representation of a validation problem.

## [👤semio📚go💻semio🔖flattendesign](semiorepo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design)

planeToMatrix holds the data fields for a planeToMatrix record.

## [👤semio📚go💻semio🔖exportdesignmodel](semiorepo://p/u/semio/b/l/go/f/semio.go/s/ExportDesignModel)

ExportModelFormats maps supported export format extensions.

## [👤semio📚go💻semio🔖exportdesignmodelhelpers](semiorepo://p/u/semio/b/l/go/f/semio.go/s/ExportDesignModel/Helpers)

exportMeshData holds extracted or generated mesh geometry for a single type.

## [👤semio📚go💻semio🪨assetspath](semiorepo://p/u/semio/b/l/go/f/semio.go/d/c/AssetsPath)

AssetsPath holds the data fields for a AssetsPath record.

## [👤semio📚go💻semio🛠️ptrstring](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ptrString)

Guid generates a new random 128-bit hex-encoded unique identifier.
ptrString returns a pointer to the given string value.

## [👤semio📚go💻semio🛠️ptrfloat64](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ptrFloat64)

ptrFloat64 holds the data fields for a ptrFloat64 record.

## [👤semio📚go💻semio🛠️floatequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/floatEqual)

floatEqual holds the data fields for a floatEqual record.

## [👤semio📚go💻semio🛠️optfloatequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/optFloatEqual)

optFloatEqual holds the data fields for a optFloatEqual record.

## [👤semio📚go💻semio🛠️optboolequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/optBoolEqual)

optBoolEqual holds the data fields for a optBoolEqual record.

## [👤semio📚go💻semio🛠️optstringequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/optStringEqual)

optStringEqual holds the data fields for a optStringEqual record.

## [👤semio📚go💻semio🛠️arelocationidsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areLocationIdsEqual)

areLocationIdsEqual holds the data fields for a areLocationIdsEqual record.

## [👤semio📚go💻semio🛠️aretypeidsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTypeIdsEqual)

areTypeIdsEqual holds the data fields for a areTypeIdsEqual record.

## [👤semio📚go💻semio🛠️aredesignidsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areDesignIdsEqual)

areDesignIdsEqual holds the data fields for a areDesignIdsEqual record.

## [👤semio📚go💻semio🛠️areportidsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePortIdsEqual)

arePortIdsEqual holds the data fields for a arePortIdsEqual record.

## [👤semio📚go💻semio🛠️arelayeridsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areLayerIdsEqual)

areLayerIdsEqual holds the data fields for a areLayerIdsEqual record.

## [👤semio📚go💻semio🛠️normalizeoptint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/normalizeOptInt)

normalizeOptInt holds the data fields for a normalizeOptInt record.

## [👤semio📚go💻semio🛠️areauthoridsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areAuthorIdsEqual)

areAuthorIdsEqual holds the data fields for a areAuthorIdsEqual record.

## [👤semio📚go💻semio🛠️areconceptidsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConceptIdsEqual)

areConceptIdsEqual holds the data fields for a areConceptIdsEqual record.

## [👤semio📚go💻semio🛠️areportidslicesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePortIdSlicesEqual)

arePortIdSlicesEqual holds the data fields for a arePortIdSlicesEqual record.

## [👤semio📚go💻semio🛠️areattributesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areAttributesEqual)

areAttributesEqual holds the data fields for a areAttributesEqual record.

## [👤semio📚go💻semio🛠️arepropsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePropsEqual)

arePropsEqual holds the data fields for a arePropsEqual record.

## [👤semio📚go💻semio🛠️guid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Guid)

Guid holds the data fields for a Guid record.

## [👤semio📚go💻semio🛠️normalize](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Normalize)

Normalize converts a string to lowercase trimmed form.

## [👤semio📚go💻semio🛠️round](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Round)

Round rounds a float64 to the specified number of decimal places.

## [👤semio📚go💻semio🛠️deepequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DeepEqual)

DeepEqual compares two values for deep equality via JSON serialization.

## [👤semio📚go💻semio✂️attributeid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AttributeId)

AttributeId identifies an attribute entity by GUID.

## [👤semio📚go💻semio✂️locationid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LocationId)

LocationId identifies a location entity by GUID.

## [👤semio📚go💻semio✂️authorid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AuthorId)

AuthorId identifies an author entity by GUID.

## [👤semio📚go💻semio✂️fileid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FileId)

FileId identifies a file entity by GUID.

## [👤semio📚go💻semio✂️folderid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FolderId)

FolderId identifies a folder entity by GUID.

## [👤semio📚go💻semio✂️benchmarkid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/BenchmarkId)

BenchmarkId identifies a benchmark entity by GUID.

## [👤semio📚go💻semio✂️qualityid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/QualityId)

QualityId identifies a quality entity by GUID.

## [👤semio📚go💻semio✂️portid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PortId)

PortId identifies a port entity by GUID.

## [👤semio📚go💻semio✂️propid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PropId)

PropId identifies a prop entity by GUID.

## [👤semio📚go💻semio✂️tagid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TagId)

TagId identifies a tag entity by GUID.

## [👤semio📚go💻semio✂️conceptid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConceptId)

ConceptId identifies a concept entity by GUID.

## [👤semio📚go💻semio✂️modelid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ModelId)

ModelId identifies a model entity by GUID.

## [👤semio📚go💻semio✂️connectorid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectorId)

ConnectorId identifies a connector entity by GUID.

## [👤semio📚go💻semio✂️typeid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TypeId)

TypeId identifies a type entity by GUID.

## [👤semio📚go💻semio✂️layerid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LayerId)

LayerId identifies a layer entity by GUID.

## [👤semio📚go💻semio✂️pieceid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PieceId)

PieceId identifies a piece entity by GUID.

## [👤semio📚go💻semio✂️groupid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/GroupId)

GroupId identifies a group entity by GUID.

## [👤semio📚go💻semio✂️sideid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/SideId)

SideId identifies a connection side by piece, design piece and connector references.

## [👤semio📚go💻semio✂️connectionid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectionId)

ConnectionId identifies a connection entity by GUID.

## [👤semio📚go💻semio✂️statid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/StatId)

StatId identifies a stat entity by GUID.

## [👤semio📚go💻semio✂️designid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/DesignId)

DesignId identifies a design entity by GUID.

## [👤semio📚go💻semio✂️kitid](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/KitId)

KitId identifies a kit entity by GUID.

## [👤semio📚go💻semio✂️coord](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Coord)

Coord represents a 2D coordinate with U and V components.

## [👤semio📚go💻semio✂️vec](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Vec)

Vec represents a 2D vector with U and V components.

## [👤semio📚go💻semio✂️point](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Point)

Point represents a 3D point with X, Y and Z components.

## [👤semio📚go💻semio✂️vector](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Vector)

Vector represents a 3D vector with X, Y and Z components.

## [👤semio📚go💻semio✂️plane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Plane)

Plane represents a 3D plane defined by origin, X-axis and Y-axis.

## [👤semio📚go💻semio✂️camera](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Camera)

Camera represents a 3D camera with position, forward and up vectors.

## [👤semio📚go💻semio✂️attribute](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Attribute)

Attribute represents a key-value metadata entry with optional definition.

## [👤semio📚go💻semio✂️attributediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AttributeDiff)

AttributeDiff represents changes to an attribute entity.

## [👤semio📚go💻semio✂️attributesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AttributesDiff)

AttributesDiff represents a collection of attribute additions, removals and updates.

## [👤semio📚go💻semio✂️location](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Location)

Location represents a geographic location with longitude, latitude and optional altitude.

## [👤semio📚go💻semio✂️locationdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LocationDiff)

LocationDiff represents changes to a location entity.

## [👤semio📚go💻semio✂️author](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Author)

Author represents a named authorship entity with optional email.

## [👤semio📚go💻semio✂️authordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AuthorDiff)

AuthorDiff represents changes to an author entity.

## [👤semio📚go💻semio✂️authorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AuthorsDiff)

AuthorsDiff represents a collection of author additions, removals and updates.

## [👤semio📚go💻semio✂️file](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/File)

File represents a file reference entity with name, remote URL and metadata.

## [👤semio📚go💻semio✂️filediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FileDiff)

FileDiff represents changes to a file entity.

## [👤semio📚go💻semio✂️filesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FilesDiff)

FilesDiff represents a collection of file additions, removals and updates.

## [👤semio📚go💻semio✂️folder](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Folder)

Folder represents a folder hierarchy entity with name and parent reference.

## [👤semio📚go💻semio✂️folderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FolderDiff)

FolderDiff represents changes to a folder entity.

## [👤semio📚go💻semio✂️foldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FoldersDiff)

FoldersDiff represents a collection of folder additions, removals and updates.

## [👤semio📚go💻semio✂️benchmark](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Benchmark)

Benchmark represents a named metric threshold with min and max bounds.

## [👤semio📚go💻semio✂️benchmarkdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/BenchmarkDiff)

BenchmarkDiff represents changes to a benchmark entity.

## [👤semio📚go💻semio✂️benchmarksdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/BenchmarksDiff)

BenchmarksDiff represents a collection of benchmark additions, removals and updates.

## [👤semio📚go💻semio✂️qualitykind](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/QualityKind)

QualityKind is a bitfield enum for quality scope classification.

## [👤semio📚go💻semio✂️quality](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Quality)

Quality represents a measurable property with formula, units and benchmarks.

## [👤semio📚go💻semio✂️qualitydiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/QualityDiff)

QualityDiff represents changes to a quality entity.

## [👤semio📚go💻semio✂️qualitiesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/QualitiesDiff)

QualitiesDiff represents a collection of quality additions, removals and updates.

## [👤semio📚go💻semio✂️port](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Port)

Port represents a named connector port with compatible port references.

## [👤semio📚go💻semio✂️portdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PortDiff)

PortDiff represents changes to a port entity.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semio✂️portsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PortsDiff)

PortsDiff represents a collection of port additions, removals and updates.

## [👤semio📚go💻semio✂️prop](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Prop)

Prop represents a quality property value with optional unit.

## [👤semio📚go💻semio✂️propdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PropDiff)

PropDiff represents changes to a prop entity.

## [👤semio📚go💻semio✂️propsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PropsDiff)

PropsDiff represents a collection of prop additions, removals and updates.

## [👤semio📚go💻semio✂️tag](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Tag)

Tag represents a named classification tag with optional description and icon.

## [👤semio📚go💻semio✂️tagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TagDiff)

TagDiff represents changes to a tag entity.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semio✂️tagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TagsDiff)

TagsDiff represents a collection of tag additions, removals and updates.

## [👤semio📚go💻semio✂️concept](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Concept)

Concept represents a named categorization concept with optional description.

## [👤semio📚go💻semio✂️conceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConceptDiff)

ConceptDiff represents changes to a concept entity.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semio✂️conceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConceptsDiff)

ConceptsDiff represents a collection of concept additions, removals and updates.

## [👤semio📚go💻semio✂️model](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Model)

Model represents a 3D model reference associated with a file and tags.

## [👤semio📚go💻semio✂️modeldiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ModelDiff)

ModelDiff represents changes to a model entity.

## [👤semio📚go💻semio✂️modelsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ModelsDiff)

ModelsDiff represents a collection of model additions, removals and updates.

## [👤semio📚go💻semio✂️connector](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Connector)

Connector represents a spatial connection point on a type with position and direction.

## [👤semio📚go💻semio✂️pointdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PointDiff)

PointDiff represents changes to a 3D point.

## [👤semio📚go💻semio✂️vectordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/VectorDiff)

VectorDiff represents changes to a 3D vector.

## [👤semio📚go💻semio✂️connectordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectorDiff)

ConnectorDiff represents changes to a connector entity.

## [👤semio📚go💻semio✂️connectorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectorsDiff)

ConnectorsDiff represents a collection of connector additions, removals and updates.

## [👤semio📚go💻semio🛠️type](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/Type)

Type represents a component type with models, connectors and hierarchical inheritance.

## [👤semio📚go💻semio✂️typediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TypeDiff)

TypeDiff represents changes to a type entity.

## [👤semio📚go💻semio🛠️unmarshaljson](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semio🛠️hasfield](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semio✂️typesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TypesDiff)

TypesDiff represents a collection of type additions, removals and updates.

## [👤semio📚go💻semio✂️layer](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Layer)

Layer represents a named layer with visibility, lock and color properties.

## [👤semio📚go💻semio✂️layerdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LayerDiff)

LayerDiff represents changes to a layer entity.

## [👤semio📚go💻semio✂️layersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LayersDiff)

LayersDiff represents a collection of layer additions, removals and updates.

## [👤semio📚go💻semio✂️piece](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Piece)

Piece represents a placed component instance within a design.

## [👤semio📚go💻semio✂️coorddiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/CoordDiff)

CoordDiff represents changes to a 2D coordinate.

## [👤semio📚go💻semio✂️planediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PlaneDiff)

PlaneDiff represents changes to a 3D plane.

## [👤semio📚go💻semio✂️piecediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PieceDiff)

PieceDiff represents changes to a piece entity.

## [👤semio📚go💻semio✂️piecesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PiecesDiff)

PiecesDiff represents a collection of piece additions, removals and updates.

## [👤semio📚go💻semio✂️group](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Group)

Group represents a named collection of pieces within a design.

## [👤semio📚go💻semio✂️groupdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/GroupDiff)

GroupDiff represents changes to a group entity.

## [👤semio📚go💻semio✂️groupsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/GroupsDiff)

GroupsDiff represents a collection of group additions, removals and updates.

## [👤semio📚go💻semio✂️side](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Side)

Side represents one end of a connection referencing a piece and optional connector.

## [👤semio📚go💻semio✂️sidediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/SideDiff)

SideDiff represents changes to a connection side.

## [👤semio📚go💻semio✂️connection](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Connection)

Connection represents a spatial relationship between two pieces with transform parameters.

## [👤semio📚go💻semio✂️connectiondiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectionDiff)

ConnectionDiff represents changes to a connection entity.

## [👤semio📚go💻semio✂️connectionsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectionsDiff)

ConnectionsDiff represents a collection of connection additions, removals and updates.

## [👤semio📚go💻semio✂️stat](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Stat)

Stat represents a statistical quality measurement with min and max bounds.

## [👤semio📚go💻semio✂️statdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/StatDiff)

StatDiff represents changes to a stat entity.

## [👤semio📚go💻semio✂️statsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/StatsDiff)

StatsDiff represents a collection of stat additions, removals and updates.

## [👤semio📚go💻semio✂️design](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Design)

Design represents an assembly of pieces, connections, layers and groups.

## [👤semio📚go💻semio✂️cameradiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/CameraDiff)

CameraDiff represents changes to a camera view.

## [👤semio📚go💻semio✂️designdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/DesignDiff)

DesignDiff represents changes to a design entity.

## [👤semio📚go💻semio✂️designsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/DesignsDiff)

DesignsDiff represents a collection of design additions, removals and updates.

## [👤semio📚go💻semio✂️kit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Kit)

Kit represents the root container for all domain entities.

## [👤semio📚go💻semio✂️kitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/KitDiff)

KitDiff represents changes to a kit entity.

## [👤semio📚go💻semio✂️kitsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/KitsDiff)

KitsDiff represents a collection of kit additions, removals and updates.

## [👤semio📚go💻semio✂️change](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Change)

Change represents a reversible entity change with forward and backward diffs.

## [👤semio📚go💻semio✂️attributechange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AttributeChange)

AttributeChange holds the data fields for a AttributeChange record.

## [👤semio📚go💻semio✂️locationchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LocationChange)

LocationChange holds the data fields for a LocationChange record.

## [👤semio📚go💻semio✂️authorchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/AuthorChange)

AuthorChange holds the data fields for a AuthorChange record.

## [👤semio📚go💻semio✂️filechange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FileChange)

FileChange holds the data fields for a FileChange record.

## [👤semio📚go💻semio✂️folderchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/FolderChange)

FolderChange holds the data fields for a FolderChange record.

## [👤semio📚go💻semio✂️benchmarkchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/BenchmarkChange)

BenchmarkChange holds the data fields for a BenchmarkChange record.

## [👤semio📚go💻semio✂️qualitychange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/QualityChange)

QualityChange holds the data fields for a QualityChange record.

## [👤semio📚go💻semio✂️portchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PortChange)

PortChange holds the data fields for a PortChange record.

## [👤semio📚go💻semio✂️propchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PropChange)

PropChange holds the data fields for a PropChange record.

## [👤semio📚go💻semio✂️tagchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TagChange)

TagChange holds the data fields for a TagChange record.

## [👤semio📚go💻semio✂️conceptchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConceptChange)

ConceptChange holds the data fields for a ConceptChange record.

## [👤semio📚go💻semio✂️modelchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ModelChange)

ModelChange holds the data fields for a ModelChange record.

## [👤semio📚go💻semio✂️connectorchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectorChange)

ConnectorChange holds the data fields for a ConnectorChange record.

## [👤semio📚go💻semio✂️typechange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/TypeChange)

TypeChange holds the data fields for a TypeChange record.

## [👤semio📚go💻semio✂️layerchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/LayerChange)

LayerChange holds the data fields for a LayerChange record.

## [👤semio📚go💻semio✂️piecechange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/PieceChange)

PieceChange holds the data fields for a PieceChange record.

## [👤semio📚go💻semio✂️groupchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/GroupChange)

GroupChange holds the data fields for a GroupChange record.

## [👤semio📚go💻semio✂️sidechange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/SideChange)

SideChange holds the data fields for a SideChange record.

## [👤semio📚go💻semio✂️connectionchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ConnectionChange)

ConnectionChange holds the data fields for a ConnectionChange record.

## [👤semio📚go💻semio✂️statchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/StatChange)

StatChange holds the data fields for a StatChange record.

## [👤semio📚go💻semio✂️designchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/DesignChange)

DesignChange holds the data fields for a DesignChange record.

## [👤semio📚go💻semio✂️kitchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/KitChange)

KitChange holds the data fields for a KitChange record.

## [👤semio📚go💻semio🛠️getdesignchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/GetDesignChange)

GetDesignChange holds the data fields for a GetDesignChange record.

## [👤semio📚go💻semio🛠️getkitchange](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/GetKitChange)

GetKitChange holds the data fields for a GetKitChange record.

## [👤semio📚go💻semio🛠️serializekit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/SerializeKit)

SerializeKit marshals a kit to indented JSON bytes.

## [👤semio📚go💻semio🛠️deserializekit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DeserializeKit)

DeserializeKit unmarshals JSON bytes into a kit.

## [👤semio📚go💻semio🛠️serializekitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/SerializeKitDiff)

SerializeKitDiff marshals a kit diff to indented JSON bytes.

## [👤semio📚go💻semio🛠️deserializekitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DeserializeKitDiff)

DeserializeKitDiff unmarshals JSON bytes into a kit diff.

## [👤semio📚go💻semio🛠️findtypeinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindTypeInKit)

FindTypeInKit returns a pointer to the type with the given GUID or nil.

## [👤semio📚go💻semio🛠️finddesigninkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindDesignInKit)

FindDesignInKit returns a pointer to the design with the given GUID or nil.

## [👤semio📚go💻semio🛠️findpieceindesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindPieceInDesign)

FindPieceInDesign returns a pointer to the piece with the given GUID or nil.

## [👤semio📚go💻semio🛠️findconnectionindesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindConnectionInDesign)

FindConnectionInDesign returns a pointer to the connection with the given GUID or nil.

## [👤semio📚go💻semio🛠️findconnectorintype](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindConnectorInType)

FindConnectorInType returns a pointer to the connector with the given GUID or nil.

## [👤semio📚go💻semio🛠️findfileinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindFileInKit)

FindFileInKit returns a pointer to the file with the given GUID or nil.

## [👤semio📚go💻semio🛠️findfolderinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindFolderInKit)

FindFolderInKit returns a pointer to the folder with the given GUID or nil.

## [👤semio📚go💻semio🛠️findqualityinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindQualityInKit)

FindQualityInKit returns a pointer to the quality with the given GUID or nil.

## [👤semio📚go💻semio🛠️findportinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindPortInKit)

FindPortInKit returns a pointer to the port with the given GUID or nil.

## [👤semio📚go💻semio🛠️findtaginkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindTagInKit)

FindTagInKit returns a pointer to the tag with the given GUID or nil.

## [👤semio📚go💻semio🛠️findconceptinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindConceptInKit)

FindConceptInKit returns a pointer to the concept with the given GUID or nil.

## [👤semio📚go💻semio🛠️findauthorinkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FindAuthorInKit)

FindAuthorInKit returns a pointer to the author with the given GUID or nil.

## [👤semio📚go💻semio🛠️sumqualityindesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/SumQualityInDesign)

For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.

## [👤semio📚go💻semio🛠️newkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewKit)

NewKit creates a new kit with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newtype](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewType)

NewType creates a new type with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newdesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewDesign)

NewDesign creates a new design with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newpiece](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewPiece)

NewPiece creates a new piece with a generated GUID.

## [👤semio📚go💻semio🛠️newconnection](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewConnection)

NewConnection creates a new connection between two pieces by their GUIDs.

## [👤semio📚go💻semio🛠️newconnector](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewConnector)

NewConnector creates a new connector with position, direction and parameter t.

## [👤semio📚go💻semio🛠️newfile](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewFile)

NewFile creates a new file with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newfolder](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewFolder)

NewFolder creates a new folder with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newquality](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewQuality)

NewQuality creates a new quality with the given key, name and a generated GUID.

## [👤semio📚go💻semio🛠️newport](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewPort)

NewPort creates a new port with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newtag](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewTag)

NewTag creates a new tag with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newconcept](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewConcept)

NewConcept creates a new concept with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️newauthor](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/NewAuthor)

NewAuthor creates a new author with the given name and a generated GUID.

## [👤semio📚go💻semio🛠️arekitsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AreKitsEqual)

AreKitsEqual compares two kits for structural equality.

## [👤semio📚go💻semio🛠️arekitdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AreKitDiffsEqual)

AreKitDiffsEqual compares two kit diffs for structural equality.

## [👤semio📚go💻semio🛠️aretypesdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTypesDiffsEqual)

areTypesDiffsEqual holds the data fields for a areTypesDiffsEqual record.

## [👤semio📚go💻semio🛠️aredesignsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areDesignsDiffsEqual)

areDesignsDiffsEqual holds the data fields for a areDesignsDiffsEqual record.

## [👤semio📚go💻semio🛠️aretagsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTagsDiffsEqual)

areTagsDiffsEqual holds the data fields for a areTagsDiffsEqual record.

## [👤semio📚go💻semio🛠️areconceptsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConceptsDiffsEqual)

areConceptsDiffsEqual holds the data fields for a areConceptsDiffsEqual record.

## [👤semio📚go💻semio🛠️areportsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePortsDiffsEqual)

arePortsDiffsEqual holds the data fields for a arePortsDiffsEqual record.

## [👤semio📚go💻semio🛠️arefilesdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFilesDiffsEqual)

areFilesDiffsEqual holds the data fields for a areFilesDiffsEqual record.

## [👤semio📚go💻semio🛠️arefoldersdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFoldersDiffsEqual)

areFoldersDiffsEqual holds the data fields for a areFoldersDiffsEqual record.

## [👤semio📚go💻semio🛠️areauthorsdiffsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areAuthorsDiffsEqual)

areAuthorsDiffsEqual holds the data fields for a areAuthorsDiffsEqual record.

## [👤semio📚go💻semio🛠️getkitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/GetKitDiff)

GetKitDiff computes the diff between a before and after kit state.

## [👤semio📚go💻semio🛠️gettypesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTypesDiff)

getTypesDiff holds the data fields for a getTypesDiff record.

## [👤semio📚go💻semio🛠️gettypediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTypeDiff)

getTypeDiff holds the data fields for a getTypeDiff record.

## [👤semio📚go💻semio🛠️istypediffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isTypeDiffEmpty)

isTypeDiffEmpty holds the data fields for a isTypeDiffEmpty record.

## [👤semio📚go💻semio🛠️getdesignsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getDesignsDiff)

getDesignsDiff holds the data fields for a getDesignsDiff record.

## [👤semio📚go💻semio🛠️getdesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getDesignDiff)

getDesignDiff holds the data fields for a getDesignDiff record.

## [👤semio📚go💻semio🛠️isdesigndiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isDesignDiffEmpty)

isDesignDiffEmpty holds the data fields for a isDesignDiffEmpty record.

## [👤semio📚go💻semio🛠️gettagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTagsDiff)

getTagsDiff holds the data fields for a getTagsDiff record.

## [👤semio📚go💻semio🛠️gettagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getTagDiff)

getTagDiff holds the data fields for a getTagDiff record.

## [👤semio📚go💻semio🛠️istagdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isTagDiffEmpty)

isTagDiffEmpty holds the data fields for a isTagDiffEmpty record.

## [👤semio📚go💻semio🛠️getconceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConceptsDiff)

getConceptsDiff holds the data fields for a getConceptsDiff record.

## [👤semio📚go💻semio🛠️getconceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConceptDiff)

getConceptDiff holds the data fields for a getConceptDiff record.

## [👤semio📚go💻semio🛠️isconceptdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isConceptDiffEmpty)

isConceptDiffEmpty holds the data fields for a isConceptDiffEmpty record.

## [👤semio📚go💻semio🛠️getportsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPortsDiff)

getPortsDiff holds the data fields for a getPortsDiff record.

## [👤semio📚go💻semio🛠️getportdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPortDiff)

getPortDiff holds the data fields for a getPortDiff record.

## [👤semio📚go💻semio🛠️isportdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isPortDiffEmpty)

isPortDiffEmpty holds the data fields for a isPortDiffEmpty record.

## [👤semio📚go💻semio🛠️getfilesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFilesDiff)

getFilesDiff holds the data fields for a getFilesDiff record.

## [👤semio📚go💻semio🛠️getfilediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFileDiff)

getFileDiff holds the data fields for a getFileDiff record.

## [👤semio📚go💻semio🛠️isfilediffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isFileDiffEmpty)

isFileDiffEmpty holds the data fields for a isFileDiffEmpty record.

## [👤semio📚go💻semio🛠️getfoldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFoldersDiff)

getFoldersDiff holds the data fields for a getFoldersDiff record.

## [👤semio📚go💻semio🛠️getfolderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getFolderDiff)

getFolderDiff holds the data fields for a getFolderDiff record.

## [👤semio📚go💻semio🛠️isfolderdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isFolderDiffEmpty)

isFolderDiffEmpty holds the data fields for a isFolderDiffEmpty record.

## [👤semio📚go💻semio🛠️getauthorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getAuthorsDiff)

getAuthorsDiff holds the data fields for a getAuthorsDiff record.

## [👤semio📚go💻semio🛠️getauthordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getAuthorDiff)

getAuthorDiff holds the data fields for a getAuthorDiff record.

## [👤semio📚go💻semio🛠️isauthordiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isAuthorDiffEmpty)

isAuthorDiffEmpty holds the data fields for a isAuthorDiffEmpty record.

## [👤semio📚go💻semio🛠️inversekitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/InverseKitDiff)

InverseKitDiff computes the reverse diff that undoes an applied diff.

## [👤semio📚go💻semio🛠️inversetypesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTypesDiff)

inverseTypesDiff holds the data fields for a inverseTypesDiff record.

## [👤semio📚go💻semio🛠️inversetypediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTypeDiff)

inverseTypeDiff holds the data fields for a inverseTypeDiff record.

## [👤semio📚go💻semio🛠️inversedesignsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseDesignsDiff)

inverseDesignsDiff holds the data fields for a inverseDesignsDiff record.

## [👤semio📚go💻semio🛠️inversedesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseDesignDiff)

inverseDesignDiff holds the data fields for a inverseDesignDiff record.

## [👤semio📚go💻semio🛠️inversetagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTagsDiff)

inverseTagsDiff holds the data fields for a inverseTagsDiff record.

## [👤semio📚go💻semio🛠️inversetagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseTagDiff)

inverseTagDiff holds the data fields for a inverseTagDiff record.

## [👤semio📚go💻semio🛠️inverseconceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConceptsDiff)

inverseConceptsDiff holds the data fields for a inverseConceptsDiff record.

## [👤semio📚go💻semio🛠️inverseconceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConceptDiff)

inverseConceptDiff holds the data fields for a inverseConceptDiff record.

## [👤semio📚go💻semio🛠️inverseportsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePortsDiff)

inversePortsDiff holds the data fields for a inversePortsDiff record.

## [👤semio📚go💻semio🛠️inverseportdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePortDiff)

inversePortDiff holds the data fields for a inversePortDiff record.

## [👤semio📚go💻semio🛠️inversefilesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFilesDiff)

inverseFilesDiff holds the data fields for a inverseFilesDiff record.

## [👤semio📚go💻semio🛠️inversefilediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFileDiff)

inverseFileDiff holds the data fields for a inverseFileDiff record.

## [👤semio📚go💻semio🛠️inversefoldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFoldersDiff)

inverseFoldersDiff holds the data fields for a inverseFoldersDiff record.

## [👤semio📚go💻semio🛠️inversefolderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseFolderDiff)

inverseFolderDiff holds the data fields for a inverseFolderDiff record.

## [👤semio📚go💻semio🛠️inverseauthorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseAuthorsDiff)

inverseAuthorsDiff holds the data fields for a inverseAuthorsDiff record.

## [👤semio📚go💻semio🛠️inverseauthordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseAuthorDiff)

inverseAuthorDiff performs the inverseAuthorDiff operation.

## [👤semio📚go💻semio🛠️inverseconnectorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConnectorsDiff)

inverseConnectorsDiff holds the data fields for a inverseConnectorsDiff record.

## [👤semio📚go💻semio🛠️inverseconnectordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConnectorDiff)

inverseConnectorDiff holds the data fields for a inverseConnectorDiff record.

## [👤semio📚go💻semio🛠️inversemodelsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseModelsDiff)

inverseModelsDiff holds the data fields for a inverseModelsDiff record.

## [👤semio📚go💻semio🛠️inversemodeldiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseModelDiff)

inverseModelDiff holds the data fields for a inverseModelDiff record.

## [👤semio📚go💻semio🛠️inversepiecesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePiecesDiff)

inversePiecesDiff holds the data fields for a inversePiecesDiff record.

## [👤semio📚go💻semio🛠️inversepiecediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePieceDiff)

inversePieceDiff holds the data fields for a inversePieceDiff record.

## [👤semio📚go💻semio🛠️inverseconnectionsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConnectionsDiff)

inverseConnectionsDiff holds the data fields for a inverseConnectionsDiff record.

## [👤semio📚go💻semio🛠️inverseconnectiondiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseConnectionDiff)

inverseConnectionDiff holds the data fields for a inverseConnectionDiff record.

## [👤semio📚go💻semio🛠️inversesidediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseSideDiff)

inverseSideDiff holds the data fields for a inverseSideDiff record.

## [👤semio📚go💻semio🛠️inverseattributediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseAttributeDiff)

inverseAttributesDiff holds the data fields for a inverseAttributesDiff record.

## [👤semio📚go💻semio🛠️inverseattributesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseAttributesDiff)

inverseAttributesDiff performs the inverseAttributesDiff operation.

## [👤semio📚go💻semio🛠️inversepropsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePropsDiff)

inversePropsDiff holds the data fields for a inversePropsDiff record.

## [👤semio📚go💻semio🛠️inversepropdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inversePropDiff)

inversePropDiff holds the data fields for a inversePropDiff record.

## [👤semio📚go💻semio🛠️inversestatsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseStatsDiff)

inverseStatsDiff holds the data fields for a inverseStatsDiff record.

## [👤semio📚go💻semio🛠️inversestatdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseStatDiff)

inverseStatDiff holds the data fields for a inverseStatDiff record.

## [👤semio📚go💻semio🛠️inverselayersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseLayersDiff)

inverseLayersDiff holds the data fields for a inverseLayersDiff record.

## [👤semio📚go💻semio🛠️inverselayerdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseLayerDiff)

inverseLayerDiff performs the inverseLayerDiff operation.

## [👤semio📚go💻semio🛠️inversegroupsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseGroupsDiff)

inverseGroupsDiff holds the data fields for a inverseGroupsDiff record.

## [👤semio📚go💻semio🛠️inversegroupdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/inverseGroupDiff)

inverseGroupDiff holds the data fields for a inverseGroupDiff record.

## [👤semio📚go💻semio🛠️normalizestr](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/normalizeStr)

normalizeStr holds the data fields for a normalizeStr record.

## [👤semio📚go💻semio🛠️normalizeint64](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/normalizeInt64)

normalizeInt64 holds the data fields for a normalizeInt64 record.

## [👤semio📚go💻semio🛠️arefolderidsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFolderIdsEqual)

areFolderIdsEqual holds the data fields for a areFolderIdsEqual record.

## [👤semio📚go💻semio🛠️getattributediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getAttributeDiff)

getAttributesDiff holds the data fields for a getAttributesDiff record.

## [👤semio📚go💻semio🛠️isattributediffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isAttributeDiffEmpty)

isAttributeDiffEmpty performs the isAttributeDiffEmpty operation.

## [👤semio📚go💻semio🛠️getattributesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getAttributesDiff)

getAttributesDiff holds the data fields for a getAttributesDiff record.

## [👤semio📚go💻semio🛠️isattributesdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isAttributesDiffEmpty)

isAttributesDiffEmpty holds the data fields for a isAttributesDiffEmpty record.

## [👤semio📚go💻semio🛠️getpropsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPropsDiff)

getPropsDiff holds the data fields for a getPropsDiff record.

## [👤semio📚go💻semio🛠️getpropdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPropDiff)

getPropDiff holds the data fields for a getPropDiff record.

## [👤semio📚go💻semio🛠️ispropdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isPropDiffEmpty)

isPropDiffEmpty holds the data fields for a isPropDiffEmpty record.

## [👤semio📚go💻semio🛠️getstatsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getStatsDiff)

getStatsDiff holds the data fields for a getStatsDiff record.

## [👤semio📚go💻semio🛠️getstatdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getStatDiff)

getStatDiff holds the data fields for a getStatDiff record.

## [👤semio📚go💻semio🛠️isstatdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isStatDiffEmpty)

isStatDiffEmpty holds the data fields for a isStatDiffEmpty record.

## [👤semio📚go💻semio🛠️getlayersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getLayersDiff)

getLayersDiff holds the data fields for a getLayersDiff record.

## [👤semio📚go💻semio🛠️getlayerdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getLayerDiff)

getLayerDiff holds the data fields for a getLayerDiff record.

## [👤semio📚go💻semio🛠️islayerdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isLayerDiffEmpty)

isLayerDiffEmpty holds the data fields for a isLayerDiffEmpty record.

## [👤semio📚go💻semio🛠️getgroupsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getGroupsDiff)

getGroupsDiff holds the data fields for a getGroupsDiff record.

## [👤semio📚go💻semio🛠️getgroupdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getGroupDiff)

getGroupDiff holds the data fields for a getGroupDiff record.

## [👤semio📚go💻semio🛠️isgroupdiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isGroupDiffEmpty)

isGroupDiffEmpty holds the data fields for a isGroupDiffEmpty record.

## [👤semio📚go💻semio🛠️applyattributediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyAttributeDiff)

applyAttributesDiff holds the data fields for a applyAttributesDiff record.

## [👤semio📚go💻semio🛠️applyattributesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyAttributesDiff)

applyAttributesDiff holds the data fields for a applyAttributesDiff record.

## [👤semio📚go💻semio🛠️applypropsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPropsDiff)

applyPropsDiff holds the data fields for a applyPropsDiff record.

## [👤semio📚go💻semio🛠️applypropdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPropDiff)

applyPropDiff holds the data fields for a applyPropDiff record.

## [👤semio📚go💻semio🛠️applystatsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyStatsDiff)

applyStatsDiff holds the data fields for a applyStatsDiff record.

## [👤semio📚go💻semio🛠️applystatdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyStatDiff)

applyStatDiff holds the data fields for a applyStatDiff record.

## [👤semio📚go💻semio🛠️applylayersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyLayersDiff)

applyLayersDiff holds the data fields for a applyLayersDiff record.

## [👤semio📚go💻semio🛠️applylayerdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyLayerDiff)

applyLayerDiff holds the data fields for a applyLayerDiff record.

## [👤semio📚go💻semio🛠️applygroupsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyGroupsDiff)

applyGroupsDiff holds the data fields for a applyGroupsDiff record.

## [👤semio📚go💻semio🛠️applygroupdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyGroupDiff)

applyGroupDiff holds the data fields for a applyGroupDiff record.

## [👤semio📚go💻semio🛠️getconnectorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConnectorsDiff)

getConnectorsDiff holds the data fields for a getConnectorsDiff record.

## [👤semio📚go💻semio🛠️getconnectordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConnectorDiff)

getConnectorDiff holds the data fields for a getConnectorDiff record.

## [👤semio📚go💻semio🛠️isconnectordiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isConnectorDiffEmpty)

isConnectorDiffEmpty holds the data fields for a isConnectorDiffEmpty record.

## [👤semio📚go💻semio🛠️getmodeldiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getModelDiff)

getModelsDiff holds the data fields for a getModelsDiff record.

## [👤semio📚go💻semio🛠️getmodelsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getModelsDiff)

getModelsDiff holds the data fields for a getModelsDiff record.

## [👤semio📚go💻semio🛠️getpiecesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPiecesDiff)

getPiecesDiff holds the data fields for a getPiecesDiff record.

## [👤semio📚go💻semio🛠️getpiecediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getPieceDiff)

getPieceDiff holds the data fields for a getPieceDiff record.

## [👤semio📚go💻semio🛠️areplanesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePlanesEqual)

arePlanesEqual holds the data fields for a arePlanesEqual record.

## [👤semio📚go💻semio🛠️ispiecediffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isPieceDiffEmpty)

isPieceDiffEmpty holds the data fields for a isPieceDiffEmpty record.

## [👤semio📚go💻semio🛠️getconnectionsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConnectionsDiff)

getConnectionsDiff holds the data fields for a getConnectionsDiff record.

## [👤semio📚go💻semio🛠️getsidediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getSideDiff)

getSideDiff holds the data fields for a getSideDiff record.

## [👤semio📚go💻semio🛠️getconnectiondiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConnectionDiff)

getConnectionDiff holds the data fields for a getConnectionDiff record.

## [👤semio📚go💻semio🛠️isconnectiondiffempty](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/isConnectionDiffEmpty)

isConnectionDiffEmpty holds the data fields for a isConnectionDiffEmpty record.

## [👤semio📚go💻semio🛠️aretypesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTypesEqual)

areTypesEqual holds the data fields for a areTypesEqual record.

## [👤semio📚go💻semio🛠️areconnectorsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConnectorsEqual)

areConnectorsEqual holds the data fields for a areConnectorsEqual record.

## [👤semio📚go💻semio🛠️aremodelsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areModelsEqual)

areModelsEqual holds the data fields for a areModelsEqual record.

## [👤semio📚go💻semio🛠️aredesignsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areDesignsEqual)

areDesignsEqual holds the data fields for a areDesignsEqual record.

## [👤semio📚go💻semio🛠️arepiecesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePiecesEqual)

arePiecesEqual holds the data fields for a arePiecesEqual record.

## [👤semio📚go💻semio🛠️areconnectionsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConnectionsEqual)

areConnectionsEqual holds the data fields for a areConnectionsEqual record.

## [👤semio📚go💻semio🛠️aretagsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areTagsEqual)

areTagsEqual holds the data fields for a areTagsEqual record.

## [👤semio📚go💻semio🛠️areconceptsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areConceptsEqual)

areConceptsEqual holds the data fields for a areConceptsEqual record.

## [👤semio📚go💻semio🛠️areportsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/arePortsEqual)

arePortsEqual holds the data fields for a arePortsEqual record.

## [👤semio📚go💻semio🛠️arefilesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFilesEqual)

areFilesEqual holds the data fields for a areFilesEqual record.

## [👤semio📚go💻semio🛠️arefoldersequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areFoldersEqual)

areFoldersEqual holds the data fields for a areFoldersEqual record.

## [👤semio📚go💻semio🛠️areauthorsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areAuthorsEqual)

areAuthorsEqual holds the data fields for a areAuthorsEqual record.

## [👤semio📚go💻semio🛠️arecoordsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areCoordsEqual)

areCoordsEqual holds the data fields for a areCoordsEqual record.

## [👤semio📚go💻semio🛠️aresidesequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areSidesEqual)

areSidesEqual holds the data fields for a areSidesEqual record.

## [👤semio📚go💻semio🛠️arestatsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areStatsEqual)

areStatsEqual holds the data fields for a areStatsEqual record.

## [👤semio📚go💻semio🛠️arelayersequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areLayersEqual)

areLayersEqual holds the data fields for a areLayersEqual record.

## [👤semio📚go💻semio🛠️aregroupsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/areGroupsEqual)

areGroupsEqual holds the data fields for a areGroupsEqual record.

## [👤semio📚go💻semio🛠️applykitdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ApplyKitDiff)

ApplyKitDiff applies a diff to a base kit producing the updated kit.

## [👤semio📚go💻semio🛠️applytypesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTypesDiff)

applyTypesDiff holds the data fields for a applyTypesDiff record.

## [👤semio📚go💻semio🛠️applytypediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTypeDiff)

applyTypeDiff holds the data fields for a applyTypeDiff record.

## [👤semio📚go💻semio🛠️applyconnectorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectorsDiff)

applyConnectorsDiff holds the data fields for a applyConnectorsDiff record.

## [👤semio📚go💻semio🛠️applyconnectordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectorDiff)

applyConnectorDiff holds the data fields for a applyConnectorDiff record.

## [👤semio📚go💻semio🛠️applymodelsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyModelsDiff)

applyModelsDiff holds the data fields for a applyModelsDiff record.

## [👤semio📚go💻semio🛠️applymodeldiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyModelDiff)

applyModelDiff holds the data fields for a applyModelDiff record.

## [👤semio📚go💻semio🛠️applydesignsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyDesignsDiff)

applyDesignsDiff holds the data fields for a applyDesignsDiff record.

## [👤semio📚go💻semio🛠️applydesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyDesignDiff)

applyDesignDiff holds the data fields for a applyDesignDiff record.

## [👤semio📚go💻semio🛠️applypiecesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPiecesDiff)

applyPiecesDiff holds the data fields for a applyPiecesDiff record.

## [👤semio📚go💻semio🛠️applypiecediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPieceDiff)

applyPieceDiff holds the data fields for a applyPieceDiff record.

## [👤semio📚go💻semio🛠️applyconnectionsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectionsDiff)

applyConnectionsDiff holds the data fields for a applyConnectionsDiff record.

## [👤semio📚go💻semio🛠️applyconnectiondiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConnectionDiff)

applyConnectionDiff holds the data fields for a applyConnectionDiff record.

## [👤semio📚go💻semio🛠️applysidediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applySideDiff)

applySideDiff holds the data fields for a applySideDiff record.

## [👤semio📚go💻semio🛠️applytagsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTagsDiff)

applyTagsDiff holds the data fields for a applyTagsDiff record.

## [👤semio📚go💻semio🛠️applytagdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyTagDiff)

applyTagDiff holds the data fields for a applyTagDiff record.

## [👤semio📚go💻semio🛠️applyconceptsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConceptsDiff)

applyConceptsDiff holds the data fields for a applyConceptsDiff record.

## [👤semio📚go💻semio🛠️applyconceptdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyConceptDiff)

applyConceptDiff holds the data fields for a applyConceptDiff record.

## [👤semio📚go💻semio🛠️applyportsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPortsDiff)

applyPortsDiff holds the data fields for a applyPortsDiff record.

## [👤semio📚go💻semio🛠️applyportdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyPortDiff)

applyPortDiff holds the data fields for a applyPortDiff record.

## [👤semio📚go💻semio🛠️applyfilesdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFilesDiff)

applyFilesDiff performs the applyFilesDiff operation.

## [👤semio📚go💻semio🛠️applyfilediff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFileDiff)

applyFileDiff holds the data fields for a applyFileDiff record.

## [👤semio📚go💻semio🛠️applyfoldersdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFoldersDiff)

applyFoldersDiff holds the data fields for a applyFoldersDiff record.

## [👤semio📚go💻semio🛠️applyfolderdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyFolderDiff)

applyFolderDiff holds the data fields for a applyFolderDiff record.

## [👤semio📚go💻semio🛠️applyauthorsdiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyAuthorsDiff)

applyAuthorsDiff holds the data fields for a applyAuthorsDiff record.

## [👤semio📚go💻semio🛠️applyauthordiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyAuthorDiff)

applyAuthorDiff holds the data fields for a applyAuthorDiff record.

## [👤semio📚go💻semio🛠️filterdesignswithoutparent](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FilterDesignsWithoutParent)

FilterDesignsWithoutParent returns only root-level designs with no parent.

## [👤semio📚go💻semio🛠️addtypetokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddTypeToKit)

AddTypeToKit creates a change that adds a single type to a kit.

## [👤semio📚go💻semio🛠️removetypefromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveTypeFromKit)

RemoveTypeFromKit creates a change that removes a type by GUID.

## [👤semio📚go💻semio🛠️adddesigntokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddDesignToKit)

AddDesignToKit creates a change that adds a single design to a kit.

## [👤semio📚go💻semio🛠️removedesignfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveDesignFromKit)

RemoveDesignFromKit creates a change that removes a design by GUID.

## [👤semio📚go💻semio🛠️addfiletokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddFileToKit)

AddFileToKit creates a change that adds a single file to a kit.

## [👤semio📚go💻semio🛠️removefilefromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveFileFromKit)

RemoveFileFromKit creates a change that removes a file by GUID.

## [👤semio📚go💻semio🛠️addporttokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddPortToKit)

AddPortToKit creates a change that adds a single port to a kit.

## [👤semio📚go💻semio🛠️removeportfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemovePortFromKit)

RemovePortFromKit creates a change that removes a port by GUID.

## [👤semio📚go💻semio🛠️addtagtokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddTagToKit)

AddTagToKit creates a change that adds a single tag to a kit.

## [👤semio📚go💻semio🛠️removetagfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveTagFromKit)

RemoveTagFromKit creates a change that removes a tag by GUID.

## [👤semio📚go💻semio🛠️addconcepttokit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AddConceptToKit)

AddConceptToKit creates a change that adds a single concept to a kit.

## [👤semio📚go💻semio🛠️removeconceptfromkit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/RemoveConceptFromKit)

RemoveConceptFromKit creates a change that removes a concept by GUID.

## [👤semio📚go💻semio✂️semioentitykind](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/SemioEntityKind)

SemioEntityKind enumerates the kinds of semio domain entities.

## [👤semio📚go💻semio✂️severity](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Severity)

Severity enumerates validation problem severity levels.

## [👤semio📚go💻semio✂️domainlocation](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/DomainLocation)

DomainLocation identifies the entity and field where a validation problem occurs.

## [👤semio📚go💻semio✂️fix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Fix)

Fix represents a suggested correction for a validation problem.

## [👤semio📚go💻semio✂️problem](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Problem)

Problem represents a single validation constraint breach.

## [👤semio📚go💻semio✂️validationresult](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ValidationResult)

ValidationResult contains all problems found during kit validation.

## [👤semio📚go💻semio✂️validationcontext](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ValidationContext)

ValidationContext provides indexed access to kit entities for constraint evaluation.

## [👤semio📚go💻semio✂️constraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/Constraint)

Constraint is a function that evaluates a validation rule against a kit context.

## [👤semio📚go💻semio🛠️buildvalidationcontext](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/buildValidationContext)

buildValidationContext holds the data fields for a buildValidationContext record.

## [👤semio📚go💻semio🛠️generateuniquename](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/generateUniqueName)

generateUniqueName holds the data fields for a generateUniqueName record.

## [👤semio📚go💻semio🛠️makefix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/makeFix)

makeFix holds the data fields for a makeFix record.

## [👤semio📚go💻semio🛠️guiduniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/GuidUniquenessConstraint)

GuidUniquenessConstraint checks that all entity GUIDs are unique within a kit.

## [👤semio📚go💻semio🛠️updateguideverywhere](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/updateGuidEverywhere)

updateGuidEverywhere holds the data fields for a updateGuidEverywhere record.

## [👤semio📚go💻semio🛠️typenameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/TypeNameUniquenessConstraint)

TypeNameUniquenessConstraint checks that sibling type names are unique.

## [👤semio📚go💻semio🛠️designnameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DesignNameUniquenessConstraint)

DesignNameUniquenessConstraint checks that sibling design names are unique.

## [👤semio📚go💻semio🛠️piecenameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/PieceNameUniquenessConstraint)

PieceNameUniquenessConstraint checks that piece names are unique within each design.

## [👤semio📚go💻semio🛠️qualitynameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/QualityNameUniquenessConstraint)

QualityNameUniquenessConstraint checks that quality names are unique within a kit.

## [👤semio📚go💻semio🛠️portnameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/PortNameUniquenessConstraint)

PortNameUniquenessConstraint checks that port names are unique within a kit.

## [👤semio📚go💻semio🛠️filenameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FileNameUniquenessConstraint)

FileNameUniquenessConstraint checks that file names are unique within a kit.

## [👤semio📚go💻semio🛠️foldernameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FolderNameUniquenessConstraint)

FolderNameUniquenessConstraint checks that sibling folder names are unique.

## [👤semio📚go💻semio🛠️connectornameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ConnectorNameUniquenessConstraint)

ConnectorNameUniquenessConstraint checks that connector names are unique within each type.

## [👤semio📚go💻semio🛠️modelnameuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ModelNameUniquenessConstraint)

ModelNameUniquenessConstraint checks that model names are unique within each type.

## [👤semio📚go💻semio🛠️layerpathuniquenessconstraint](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/LayerPathUniquenessConstraint)

LayerPathUniquenessConstraint checks that layer paths are unique within each design.

## [👤semio📚go💻semio🪨defaultconstraints](semiorepo://p/u/semio/b/l/go/f/semio.go/d/c/DefaultConstraints)

DefaultConstraints lists all built-in validation constraints.

## [👤semio📚go💻semio🛠️validatekit](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ValidateKit)

ValidateKit validates a kit using the default set of constraints.

## [👤semio📚go💻semio🛠️validatekitwithconstraints](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ValidateKitWithConstraints)

ValidateKitWithConstraints validates a kit using the provided constraints.

## [👤semio📚go💻semio🛠️haserrors](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/HasErrors)

HasErrors returns true if the validation result contains any error-severity problems.

## [👤semio📚go💻semio✂️problemserialized](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ProblemSerialized)

ProblemSerialized is the JSON-serializable representation of a validation problem.

## [👤semio📚go💻semio✂️validationresultserialized](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/ValidationResultSerialized)

ValidationResultSerialized is the JSON-serializable representation of a validation result.

## [👤semio📚go💻semio🛠️tovalidationresult](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ToValidationResult)

ToValidationResult converts a validation result to its serializable form.

## [👤semio📚go💻semio🛠️arevalidationresultsequal](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/AreValidationResultsEqual)

AreValidationResultsEqual compares two serialized validation results for equality.

## [👤semio📚go💻semio🛠️planetomatrix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/planeToMatrix)

planeToMatrix holds the data fields for a planeToMatrix record.

## [👤semio📚go💻semio🛠️matrixtoplane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/matrixToPlane)

matrixToPlane holds the data fields for a matrixToPlane record.

## [👤semio📚go💻semio🛠️cross](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/cross)

cross holds the data fields for a cross record.

## [👤semio📚go💻semio🛠️normalize](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/normalize)

normalize holds the data fields for a normalize record.

## [👤semio📚go💻semio🛠️dot](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/dot)

dot holds the data fields for a dot record.

## [👤semio📚go💻semio🛠️veclength](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/vecLength)

vecLength holds the data fields for a vecLength record.

## [👤semio📚go💻semio🛠️degtorad](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/degToRad)

degToRad holds the data fields for a degToRad record.

## [👤semio📚go💻semio🛠️roundfloat](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/roundFloat)

roundFloat holds the data fields for a roundFloat record.

## [👤semio📚go💻semio🛠️roundplane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/roundPlane)

roundPlane holds the data fields for a roundPlane record.

## [👤semio📚go💻semio🛠️makerotationaxis](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/makeRotationAxis)

makeRotationAxis holds the data fields for a makeRotationAxis record.

## [👤semio📚go💻semio🛠️maketranslation](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/makeTranslation)

makeTranslation holds the data fields for a makeTranslation record.

## [👤semio📚go💻semio🛠️quaternionfromaxisangle](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/quaternionFromAxisAngle)

quaternionFromAxisAngle holds the data fields for a quaternionFromAxisAngle record.

## [👤semio📚go💻semio🛠️quaternionfromunitvectors](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/quaternionFromUnitVectors)

quaternionFromUnitVectors holds the data fields for a quaternionFromUnitVectors record.

## [👤semio📚go💻semio🛠️quaterniontomatrix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/quaternionToMatrix)

quaternionToMatrix holds the data fields for a quaternionToMatrix record.

## [👤semio📚go💻semio🛠️multiplymatrices](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/multiplyMatrices)

multiplyMatrices holds the data fields for a multiplyMatrices record.

## [👤semio📚go💻semio🛠️applymatrix4tovec3](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/applyMatrix4ToVec3)

applyMatrix4ToVec3 holds the data fields for a applyMatrix4ToVec3 record.

## [👤semio📚go💻semio🛠️computechildplane](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/computeChildPlane)

computeChildPlane holds the data fields for a computeChildPlane record.

## [👤semio📚go💻semio✂️piecenode](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/pieceNode)

pieceNode holds the data fields for a pieceNode record.

## [👤semio📚go💻semio🛠️getconnector](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/getConnector)

getConnector holds the data fields for a getConnector record.

## [👤semio📚go💻semio🛠️flattendesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/FlattenDesign)

FlattenDesign computes absolute planes and centers for all pieces in a design.

## [👤semio📚go💻semio🛠️planesequalapprox](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/planesEqualApprox)

planesEqualApprox holds the data fields for a planesEqualApprox record.

## [👤semio📚go💻semio🛠️applydesigndiff](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ApplyDesignDiff)

ApplyDesignDiff applies a design diff to a base design.

## [👤semio📚go💻semio🛠️dragpiecesindesign](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/DragPiecesInDesign)

DragPiecesInDesign computes a DesignDiff that offsets selected piece centers and adjusts orphan connections.

## [👤semio📚go💻semio🪨exportmodelformats](semiorepo://p/u/semio/b/l/go/f/semio.go/d/c/ExportModelFormats)

ExportModelFormats maps supported export format extensions.

## [👤semio📚go💻semio✂️exportmeshdata](semiorepo://p/u/semio/b/l/go/f/semio.go/d/f/exportMeshData)

exportMeshData holds extracted or generated mesh geometry for a single type.

## [👤semio📚go💻semio🛠️exportplanetogltfmatrix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/exportPlaneToGltfMatrix)

exportPlaneToGltfMatrix converts a Plane to a column-major 4x4 matrix for glTF.

## [👤semio📚go💻semio🛠️exportdensetogltfmatrix](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/exportDenseToGltfMatrix)

exportDenseToGltfMatrix converts a gonum mat.Dense (row-major) to column-major glTF matrix.

## [👤semio📚go💻semio🛠️exportcreateboxmesh](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/exportCreateBoxMesh)

exportCreateBoxMesh generates a unit box placeholder mesh (1x1x1 centered at origin).

## [👤semio📚go💻semio🛠️exportdecodeblobtobytes](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/exportDecodeBlobToBytes)

exportDecodeBlobToBytes strips a data URI prefix and base64 decodes the blob content.

## [👤semio📚go💻semio🛠️exportparseglbmesh](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/exportParseGLBMesh)

exportParseGLBMesh parses a GLB binary file and extracts the first mesh's geometry data.

## [👤semio📚go💻semio🛠️exportfindmodelforkind](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/exportFindModelForKind)

exportFindModelForKind finds the best matching model for a type given tag filters.

## [👤semio📚go💻semio🛠️exportdesignmodel](semiorepo://p/u/semio/b/l/go/f/semio.go/d/i/ExportDesignModel)

ExportDesignModel exports the 3D model of a design to GLB or glTF format.

## [👤semio📚js💻index](semiorepo://p/u/semio/b/l/js/f/index.ts)

Barrel export for the domain-only JavaScript workspace modules.

## [👤semio📚js💻index🔖exports](semiorepo://p/u/semio/b/l/js/f/index.ts/s/Exports)

Public API surface re-exporting only semio domain logic and types.

## [👤semio📚js💻semio](semiorepo://p/u/semio/b/l/js/f/semio.ts)

Core domain model types, schemas and utilities for the semio platform.

## [👤semio📚js💻semio🔖attribute](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/Attribute)

DateProperty holds the data fields for a DateProperty record.

## [👤semio📚js💻semio🔖guidupdatehelper](semiorepo://p/u/semio/b/l/js/f/semio.ts/s/GUID%20update%20helper)

updateGuidEverywhere holds the data fields for a updateGuidEverywhere record.

## [👤semio📚js💻semio🛠️seededrandom](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/SeededRandom)

SeededRandom holds the data fields for a SeededRandom record.

## [👤semio📚js💻semio🛠️dateproperty](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/DateProperty)

DateProperty holds the data fields for a DateProperty record.

## [👤semio📚js💻semio🛠️getattributesdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/getAttributesDiff)

getAttributesDiff holds the data fields for a getAttributesDiff record.

## [👤semio📚js💻semio🛠️roundplane](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/roundPlane)

roundPlane holds the data fields for a roundPlane record.

## [👤semio📚js💻semio🛠️getbenchmarksdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/getBenchmarksDiff)

getBenchmarksDiff holds the data fields for a getBenchmarksDiff record.

## [👤semio📚js💻semio🛠️inversebenchmarksdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/inverseBenchmarksDiff)

inverseBenchmarksDiff holds the data fields for a inverseBenchmarksDiff record.

## [👤semio📚js💻semio🛠️mergebenchmarksdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/mergeBenchmarksDiff)

mergeBenchmarksDiff holds the data fields for a mergeBenchmarksDiff record.

## [👤semio📚js💻semio🛠️applybenchmarksdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/applyBenchmarksDiff)

applyBenchmarksDiff holds the data fields for a applyBenchmarksDiff record.

## [👤semio📚js💻semio🛠️getpropsdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/getPropsDiff)

getPropsDiff holds the data fields for a getPropsDiff record.

## [👤semio📚js💻semio🛠️inversepropsdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/inversePropsDiff)

inversePropsDiff holds the data fields for a inversePropsDiff record.

## [👤semio📚js💻semio🛠️mergepropsdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/mergePropsDiff)

mergePropsDiff holds the data fields for a mergePropsDiff record.

## [👤semio📚js💻semio🛠️applypropsdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/applyPropsDiff)

applyPropsDiff holds the data fields for a applyPropsDiff record.

## [👤semio📚js💻semio🛠️getconnectorsdiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/getConnectorsDiff)

getConnectorsDiff holds the data fields for a getConnectorsDiff record.

## [👤semio📚js💻semio🛠️computechildplane](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/computeChildPlane)

computeChildPlane holds the data fields for a computeChildPlane record.

## [👤semio📚js💻semio✂️entityidtype](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/f/EntityIdType)

EntityIdType holds the data fields for a EntityIdType record.

## [👤semio📚js💻semio✂️collectiondiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/f/CollectionDiff)

CollectionDiff holds the data fields for a CollectionDiff record.

## [👤semio📚js💻semio🪨getcollectiondiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/getCollectionDiff)

getCollectionDiff holds the data fields for a getCollectionDiff record.

## [👤semio📚js💻semio🪨inversecollectiondiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/inverseCollectionDiff)

inverseCollectionDiff holds the data fields for a inverseCollectionDiff record.

## [👤semio📚js💻semio🪨applycollectiondiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/applyCollectionDiff)

applyCollectionDiff holds the data fields for a applyCollectionDiff record.

## [👤semio📚js💻semio🪨mergecollectiondiff](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/mergeCollectionDiff)

mergeCollectionDiff holds the data fields for a mergeCollectionDiff record.

## [👤semio📚js💻semio🛠️getcolorfortext](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/getColorForText)

getColorForText holds the data fields for a getColorForText record.

## [👤semio📚js💻semio🪨cachedsqljs](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/cachedSqlJs)

cachedSqlJs holds the data fields for a cachedSqlJs record.

## [👤semio📚js💻semio🪨getsqljs](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/getSqlJs)

getSqlJs holds the data fields for a getSqlJs record.

## [👤semio📚js💻semio🛠️buildfolderpath](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/buildFolderPath)

buildFolderPath builds a slash-separated folder path from root to the given folder guid.
Uses proper mime type inferred from file extension.

## [👤semio📚js💻semio🛠️buildfilepath](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/buildFilePath)

buildFilePath builds the full path of a kit file including its folder hierarchy.
Uses proper mime type inferred from file extension.

## [👤semio📚js💻semio🪨sqlitetokit](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/sqliteToKit)

sqliteToKit holds the data fields for a sqliteToKit record.

## [👤semio📚js💻semio🪨toarray](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/toArray)

toArray holds the data fields for a toArray record.

## [👤semio📚js💻semio🪨kittosqlite](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/c/kitToSqlite)

kitToSqlite holds the data fields for a kitToSqlite record.

## [👤semio📚js💻semio🛠️updateguideverywhere](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/updateGuidEverywhere)

updateGuidEverywhere holds the data fields for a updateGuidEverywhere record.

## [👤semio📚js💻semio🛠️getprimitivedesignfromcontext](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/getPrimitiveDesignFromContext)

getPrimitiveDesignFromContext holds the data fields for a getPrimitiveDesignFromContext record.

## [👤semio📚js💻semio🛠️isguid](semiorepo://p/u/semio/b/l/js/f/semio.ts/d/i/isGuid)

isGuid holds the data fields for a isGuid record.

## [👤semio🛂jsonschema💻build](semiorepo://p/u/semio/b/s/jsonschema/f/build.ts)

Build script for generating and exporting JSON Schema definitions.

## [👤semio📚net🛅semio💻semio](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs)

Core .NET library implementing the semio domain model and serialization.

## [👤semio📚net🛅semio💻semio🛠️symbol](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/Symbol)

/ <summary>Abstract base for all expression tree nodes.</summary>
/ <remarks>
/ [👤semio📚net🛅semio💻semio🔖utility🔖expressions🛠️symbol](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/s/Utility/s/Expressions/d/i/Symbol)
/ </remarks>

## [👤semio📚net🛅semio💻semio🛠️entity](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/Entity)

/ Abstract generic base class providing equality, hashing, cloning, and validation.
/ [👤semio📚net🛅semio💻semio🔖entitying🛠️entity](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/s/Entitying/d/i/Entity)

## [👤semio📚net🛅semio💻semio🛠️entityvalidator](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/EntityValidator)

/ FluentValidation validator base for Entity subclasses.
/ [👤semio📚net🛅semio💻semio🔖entitying🛠️entityvalidator](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/s/Entitying/d/i/EntityValidator)

## [👤semio📚net🛅semio💻semio🛠️change](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/Semio.cs/d/i/Change)

/ <summary>Change holds the data fields for a Change record.</summary>

## [👤semio📚net🛅semio💻build](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/build.ts)

Build script for the Semio .NET library assembly.

## [👤semio📚py💻semio🔖imports](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Imports)

Standard library, third-party and framework imports.

## [👤semio📚py💻semio🔖typehints](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Type%20Hints)

Custom type hint aliases used throughout the module.

## [👤semio📚py💻semio🔖constants](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Constants)

Global constants for limits, paths, encodings and configuration.

## [👤semio📚py💻semio🔖utility](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Utility)

General-purpose utility functions for encoding, formatting and transformation.

## [👤semio📚py💻semio🔖logging](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Logging)

Module-level logger configuration.

## [👤semio📚py💻semio🔖exceptions](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Exceptions)

Custom exception hierarchy for server, client and specification errors.

## [👤semio📚py💻semio🔖modeling](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Modeling)

Abstract base classes for models, fields, ids, inputs, outputs and entities.

## [👤semio📚py💻semio🔖primitives](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Primitives)

Abstract base classes for models, fields, ids, inputs, outputs and entities.

## [👤semio📚py💻semio🔖graphql](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Graphql)

GraphQL node base classes for pydantic, sqlalchemy and relay integration.

## [👤semio📚py💻semio🔖domain](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Domain)

Attribute entity with key-value pairs and definitions.

## [👤semio📚py💻semio🔖attribute](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Attribute)

Attribute entity with key-value pairs and definitions.

## [👤semio📚py💻semio🔖tag](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Tag)

Tag entity for categorizing and labeling kit elements.

## [👤semio📚py💻semio🔖concept](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Concept)

Concept entity for semantic grouping of design elements.

## [👤semio📚py💻semio🔖coord](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Coord)

Coordinate primitive for three-dimensional values.

## [👤semio📚py💻semio🔖point](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Point)

Point primitive representing a position in 3D space.

## [👤semio📚py💻semio🔖vector](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Vector)

Vector primitive representing a direction in 3D space.

## [👤semio📚py💻semio🔖plane](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Plane)

Plane primitive representing an oriented coordinate frame in 3D space.

## [👤semio📚py💻semio🔖location](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Location)

Location entity for geographic coordinates with longitude, latitude and altitude.

## [👤semio📚py💻semio🔖author](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Author)

Author entity for tracking contributor identity and rank.

## [👤semio📚py💻semio🔖artifactauthor](semiorepo://p/u/semio/b/l/py/f/semio.py/s/ArtifactAuthor)

Artifact-author association entity linking artifacts to authors by email.

## [👤semio📚py💻semio🔖file](semiorepo://p/u/semio/b/l/py/f/semio.py/s/File)

File entity for managing binary assets with metadata and hashing.

## [👤semio📚py💻semio🔖folder](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Folder)

Folder entity for hierarchical organization of kit content.

## [👤semio📚py💻semio🔖benchmark](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Benchmark)

Benchmark entity for defining performance metrics with min-max bounds.

## [👤semio📚py💻semio🔖quality](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Quality)

Quality entity for defining measurable properties with units and constraints.

## [👤semio📚py💻semio🔖prop](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Prop)

Prop entity for key-value property pairs with units.

## [👤semio📚py💻semio🔖model](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Model)

Model entity for 3D geometry representations linked to files.

## [👤semio📚py💻semio🔖port](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Port)

Port entity for defining connection interfaces on types.

## [👤semio📚py💻semio🔖connector](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Connector)

Compatible port entity for specifying allowed port pairings on connectors.

## [👤semio📚py💻semio🔖compatibleport](semiorepo://p/u/semio/b/l/py/f/semio.py/s/CompatiblePort)

Compatible port entity for specifying allowed port pairings on connectors.

## [👤semio📚py💻semio🔖type](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Type)

Type entity for defining reusable parametric building blocks.

## [👤semio📚py💻semio🔖layer](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Layer)

Layer entity for organizing design elements into visibility groups.

## [👤semio📚py💻semio🔖piece](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Piece)

Piece entity for placed instances of types within a design.

## [👤semio📚py💻semio🔖group](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Group)

Group entity for named collections of pieces in a design.

## [👤semio📚py💻semio🔖side](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Side)

Side primitive for identifying a specific connector on a specific piece.

## [👤semio📚py💻semio🔖connection](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Connection)

Connection entity for linking two pieces through their connectors.

## [👤semio📚py💻semio🔖stat](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Stat)

Stat entity for recording computed statistics with bounds.

## [👤semio📚py💻semio🔖design](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Design)

Design entity for composing pieces and connections into assemblies.

## [👤semio📚py💻semio🔖kit](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Kit)

Kit entity for packaging types, designs, qualities and metadata.

## [👤semio📚py💻semio🔖designfamilyhelpers](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Design%20Family%20Helpers)

Helper functions for querying design hierarchies and families.

## [👤semio📚py💻semio🔖typefamilyhelpers](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Type%20Family%20Helpers)

Helper functions for querying type hierarchies and families.

## [👤semio📚py💻semio🔖kitqueryhelpers](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Kit%20Query%20Helpers)

Helper functions for querying entities in kits.

## [👤semio📚py💻semio🔖movedgraphenenodes](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Moved%20Graphene%20Nodes)

Graphene node definitions moved here due to forward-reference resolution order.

## [👤semio📚py💻semio🔖validation](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Validation)

Validation logic for checking kit constraints and uniqueness rules.

## [👤semio📚py💻semio🔖dictbasedvalidation](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Dict-based%20Validation)

Dictionary-based validation functions for kit data integrity.

## [👤semio📚py💻semio🔖graphoperations](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Graph%20Operations)

Graph construction and traversal for piece connectivity analysis.

## [👤semio📚py💻semio🔖flattendesign](semiorepo://p/u/semio/b/l/py/f/semio.py/s/FlattenDesign)

Design flattening to resolve nested sub-designs into a single coordinate space.

## [👤semio📚py💻semio🔖kitoperations](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Kit%20Operations)

Dict-based pure functions for kit operations exposed via MCP.

## [👤semio📚py💻semio🔖clustering](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Clustering)

Functions for clustering and expanding design pieces.

## [👤semio📚py💻semio🔖kitqueryhelpersdict](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Kit%20Query%20Helpers%20Dict)

Dict-based kit query helper functions.

## [👤semio📚py💻semio🔖kitdiffoperations](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Kit%20Diff%20Operations)

Diffing and patching operations for comparing and merging kit versions.

## [👤semio📚py💻semio🔖kitimportexport](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Kit%20Import/Export)

Import and export utilities for kit serialization and deserialization.

## [👤semio📚py💻semio🔖spatialmath](semiorepo://p/u/semio/b/l/py/f/semio.py/s/Spatial%20Math)

Spatial math utilities for vector normalization and plane computation.

## [👤semio📚rs💻semio🔖utilityfunctions](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions)

/ <summary>Guid holds the data fields for a Guid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️guid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/Guid)
/ </remarks>

## [👤semio📚rs💻semio🔖finderfunctions](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions)

/ <summary>find_type_in_kit holds the data fields for a find_type_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findtypeinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_type_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🔖serialization](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization)

/ <summary>serialize_kit holds the data fields for a serialize_kit record.</summary>
/ [👤semio📚rs💻semio🔖serialization🛠️serializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_kit)

## [👤semio📚rs💻semio🔖hasguidtrait](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🔖applydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff)

/ <summary>apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applycollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_collection_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🔖kitchangehelpers](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers)

/ Computes a CollectionDiff between two optional collections of guid-identified items.
/ Uses a caller-provided `compute_diff` function for entity-level diffs.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️getguidcollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_guid_collection_diff)

## [👤semio📚rs💻semio🔖flattendesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign)

/ <summary>FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
/ [👤semio📚rs💻semio🔖flattendesign🛠️flattenedpiece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/FlattenedPiece)

## [👤semio📚rs💻semio🔖kitmodelexport](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export)

/ <summary>Supported 3D model export formats (extension, description).</summary>
/ [👤semio📚rs💻semio🔖kitmodelexport🛠️exportmodelformats](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/EXPORT_MODEL_FORMATS)

## [👤semio📚rs💻semio🪨semioerror](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/c/SemioError)

/ <summary>SemioError holds the data fields for a SemioError record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖errortypes🛠️semioerror](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Error%20Types/d/i/SemioError)
/ </remarks>

## [👤semio📚rs💻semio✂️result](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/Result)

/ <summary>Result holds the data fields for a Result record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖errortypes🛠️result](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Error%20Types/d/i/Result)
/ </remarks>

## [👤semio📚rs💻semio✂️guid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/Guid)

/ <summary>Guid holds the data fields for a Guid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️guid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/Guid)
/ </remarks>

## [👤semio📚rs💻semio🛠️guid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/guid)

/ <summary>guid holds the data fields for a guid record.</summary>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️guid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/guid)

## [👤semio📚rs💻semio🛠️normalize](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/normalize)

/ <summary>normalize holds the data fields for a normalize record.</summary>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️normalize](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/normalize)

## [👤semio📚rs💻semio🛠️round](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/round)

/ <summary>round holds the data fields for a round record.</summary>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️round](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/round)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️jaccard](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/jaccard)

/ <summary>jaccard holds the data fields for a jaccard record.</summary>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️jaccard](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/jaccard)

## [👤semio📚rs💻semio🛠️deepequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deep_equal)

/ <summary>deep_equal holds the data fields for a deep_equal record.</summary>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️deepequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/deep_equal)

## [👤semio📚rs💻semio🛠️generateuniquename](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/generate_unique_name)

/ <summary>generate_unique_name performs the generate_unique_name operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖utilityfunctions🛠️generateuniquename](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Utility%20Functions/d/i/generate_unique_name)
/ </remarks>

## [👤semio📚rs💻semio🛠️attribute](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Attribute)

/ <summary>Attribute holds the data fields for a Attribute record.</summary>
/ [👤semio📚rs💻semio🔖modeltypesattribute🛠️attribute](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Attribute/d/i/Attribute)

## [👤semio📚rs💻semio🛠️attributeid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AttributeId)

/ <summary>AttributeId holds the data fields for a AttributeId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesattribute🛠️attributeid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Attribute/d/i/AttributeId)
/ </remarks>

## [👤semio📚rs💻semio🛠️coord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Coord)

/ <summary>Coord holds the data fields for a Coord record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypescoord🛠️coord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord/d/i/Coord)
/ </remarks>

## [👤semio📚rs💻semio🛠️coord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Coord)

/ <summary>Coord holds the data fields for a Coord record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypescoord🛠️coord](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Coord/d/i/Coord)
/ </remarks>

## [👤semio📚rs💻semio🛠️vector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Vector)

/ <summary>Vector holds the data fields for a Vector record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesvector🛠️vector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Vector/d/i/Vector)
/ </remarks>

## [👤semio📚rs💻semio🛠️vector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Vector)

/ <summary>Vector holds the data fields for a Vector record.</summary>
/ [👤semio📚rs💻semio🔖modeltypesvector🛠️vector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Vector/d/i/Vector)

## [👤semio📚rs💻semio🛠️plane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Plane)

/ <summary>Plane holds the data fields for a Plane record.</summary>
/ [👤semio📚rs💻semio🔖modeltypesplane🛠️plane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane/d/i/Plane)

## [👤semio📚rs💻semio🛠️default](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Default)

/ <summary>Default holds the data fields for a Default record.</summary>
/ [👤semio📚rs💻semio🔖modeltypesplane🛠️default](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane/d/i/Default)

## [👤semio📚rs💻semio🛠️plane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Plane)

/ <summary>Plane holds the data fields for a Plane record.</summary>
/ [👤semio📚rs💻semio🔖modeltypesplane🛠️plane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Plane/d/i/Plane)

## [👤semio📚rs💻semio🛠️camera](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Camera)

/ <summary>Camera holds the data fields for a Camera record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypescamera🛠️camera](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Camera/d/i/Camera)
/ </remarks>

## [👤semio📚rs💻semio🛠️default](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Default)

/ <summary>Default holds the data fields for a Default record.</summary>
/ [👤semio📚rs💻semio🔖modeltypescamera🛠️default](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Camera/d/i/Default)

## [👤semio📚rs💻semio🛠️locationid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/LocationId)

/ <summary>LocationId holds the data fields for a LocationId record.</summary>
/ <summary>LocationId holds the data fields for a LocationId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️locationid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/LocationId)
/ </remarks>

## [👤semio📚rs💻semio🛠️location](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Location)

/ <summary>Location holds the data fields for a Location record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️location](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/Location)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️authorid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AuthorId)

/ <summary>AuthorId holds the data fields for a AuthorId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️authorid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/AuthorId)

## [👤semio📚rs💻semio🛠️author](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Author)

/ <summary>Author holds the data fields for a Author record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️author](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/Author)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️folderid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FolderId)

/ <summary>FolderId holds the data fields for a FolderId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️folderid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/FolderId)

## [👤semio📚rs💻semio🛠️folder](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Folder)

/ <summary>Folder holds the data fields for a Folder record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️folder](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/Folder)
/ </remarks>

## [👤semio📚rs💻semio🛠️fileid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FileId)

/ <summary>FileId holds the data fields for a FileId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️fileid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/FileId)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️file](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/File)

/ <summary>File holds the data fields for a File record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslocationauthorfilefolder🛠️file](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/d/i/File)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️qualityid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/QualityId)

/ <summary>QualityId holds the data fields for a QualityId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️qualityid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/QualityId)
/ </remarks>

## [👤semio📚rs💻semio🪨qualitykind](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/c/QualityKind)

/ <summary>QualityKind holds the data fields for a QualityKind record.</summary>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️qualitykind](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/QualityKind)

## [👤semio📚rs💻semio🛠️quality](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Quality)

/ <summary>Quality holds the data fields for a Quality record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️quality](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Quality)
/ </remarks>

## [👤semio📚rs💻semio🛠️portid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PortId)

/ <summary>PortId holds the data fields for a PortId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️portid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/PortId)
/ </remarks>

## [👤semio📚rs💻semio🛠️port](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Port)

/ <summary>Port holds the data fields for a Port record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️port](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Port)
/ </remarks>

## [👤semio📚rs💻semio🛠️tagid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TagId)

/ <summary>TagId holds the data fields for a TagId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️tagid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/TagId)
/ </remarks>

## [👤semio📚rs💻semio🛠️tag](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Tag)

/ <summary>Tag holds the data fields for a Tag record.</summary>
/ <summary>Tag holds the data fields for a Tag record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️tag](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Tag)
/ </remarks>

## [👤semio📚rs💻semio🛠️conceptid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConceptId)

/ <summary>ConceptId holds the data fields for a ConceptId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️conceptid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/ConceptId)
/ </remarks>

## [👤semio📚rs💻semio🛠️concept](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Concept)

/ <summary>Concept holds the data fields for a Concept record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesqualityporttagconcept🛠️concept](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/d/i/Concept)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️propid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PropId)

/ <summary>PropId holds the data fields for a PropId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️propid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/PropId)
/ </remarks>

## [👤semio📚rs💻semio🛠️prop](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Prop)

/ <summary>Prop holds the data fields for a Prop record.</summary>
/ <summary>Prop holds the data fields for a Prop record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️prop](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/Prop)
/ </remarks>

## [👤semio📚rs💻semio🛠️modelid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ModelId)

/ <summary>ModelId holds the data fields for a ModelId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️modelid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/ModelId)

## [👤semio📚rs💻semio🛠️model](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Model)

/ <summary>Model holds the data fields for a Model record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️model](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/Model)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️connectorid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectorId)

/ <summary>ConnectorId holds the data fields for a ConnectorId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️connectorid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/ConnectorId)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️connector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Connector)

/ <summary>Connector holds the data fields for a Connector record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypespropmodelconnector🛠️connector](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Prop,%20Model,%20Connector/d/i/Connector)
/ </remarks>

## [👤semio📚rs💻semio🛠️typeid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TypeId)

/ <summary>TypeId holds the data fields for a TypeId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypestype🛠️typeid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Type/d/i/TypeId)

## [👤semio📚rs💻semio🛠️type](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Type)

/ <summary>Type holds the data fields for a Type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypestype🛠️type](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Type/d/i/Type)
/ </remarks>

## [👤semio📚rs💻semio🛠️layerid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/LayerId)

/ <summary>LayerId holds the data fields for a LayerId record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layerid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/LayerId)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️layer](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Layer)

/ <summary>Layer holds the data fields for a Layer record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layer](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Layer)
/ </remarks>

## [👤semio📚rs💻semio🛠️pieceid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PieceId)

/ <summary>PieceId holds the data fields for a PieceId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️pieceid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/PieceId)
/ </remarks>

## [👤semio📚rs💻semio🛠️designid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DesignId)

/ <summary>DesignId holds the data fields for a DesignId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️designid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/DesignId)
/ </remarks>

## [👤semio📚rs💻semio🛠️piece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Piece)

/ <summary>Piece holds the data fields for a Piece record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️piece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Piece)
/ </remarks>

## [👤semio📚rs💻semio🛠️groupid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/GroupId)

/ <summary>GroupId holds the data fields for a GroupId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️groupid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/GroupId)
/ </remarks>

## [👤semio📚rs💻semio🛠️group](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Group)

/ <summary>Group holds the data fields for a Group record.</summary>
/ <summary>Group holds the data fields for a Group record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️group](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Group)
/ </remarks>

## [👤semio📚rs💻semio🛠️side](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Side)

/ <summary>Side holds the data fields for a Side record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️side](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Side)
/ </remarks>

## [👤semio📚rs💻semio🛠️connectionid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectionId)

/ <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
/ <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connectionid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/ConnectionId)
/ </remarks>

## [👤semio📚rs💻semio🛠️connection](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Connection)

/ <summary>Connection holds the data fields for a Connection record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connection](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Connection)
/ </remarks>

## [👤semio📚rs💻semio🛠️statid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/StatId)

/ <summary>StatId holds the data fields for a StatId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️statid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/StatId)
/ </remarks>

## [👤semio📚rs💻semio🛠️stat](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Stat)

/ <summary>Stat holds the data fields for a Stat record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypeslayerpiecegroupsideconnectionstat🛠️stat](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/d/i/Stat)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️design](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Design)

/ <summary>Design holds the data fields for a Design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖modeltypesdesign🛠️design](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Design/d/i/Design)
/ </remarks>

## [👤semio📚rs💻semio🛠️kit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Kit)

/ <summary>Kit holds the data fields for a Kit record.</summary>
/ [👤semio📚rs💻semio🔖modeltypeskit🛠️kit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Model%20Types%20-%20Kit/d/i/Kit)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️findtypeinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_type_in_kit)

/ <summary>find_type_in_kit holds the data fields for a find_type_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findtypeinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_type_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findtypeinkitmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_type_in_kit_mut)

/ <summary>find_type_in_kit_mut holds the data fields for a find_type_in_kit_mut record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findtypeinkitmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_type_in_kit_mut)
/ </remarks>

## [👤semio📚rs💻semio🛠️finddesigninkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_design_in_kit)

/ <summary>find_design_in_kit holds the data fields for a find_design_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️finddesigninkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_design_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️finddesigninkitmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_design_in_kit_mut)

/ <summary>find_design_in_kit_mut holds the data fields for a find_design_in_kit_mut record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️finddesigninkitmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_design_in_kit_mut)
/ </remarks>

## [👤semio📚rs💻semio🛠️findpieceindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_piece_in_design)

/ <summary>find_piece_in_design holds the data fields for a find_piece_in_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findpieceindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_piece_in_design)
/ </remarks>

## [👤semio📚rs💻semio🛠️findpieceindesignmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_piece_in_design_mut)

/ <summary>find_piece_in_design_mut performs the find_piece_in_design_mut operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findpieceindesignmut](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_piece_in_design_mut)
/ </remarks>

## [👤semio📚rs💻semio🛠️findconnectionindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_connection_in_design)

/ <summary>find_connection_in_design holds the data fields for a find_connection_in_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findconnectionindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_connection_in_design)
/ </remarks>

## [👤semio📚rs💻semio🛠️findconnectorintype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_connector_in_type)

/ <summary>find_connector_in_type holds the data fields for a find_connector_in_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findconnectorintype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_connector_in_type)
/ </remarks>

## [👤semio📚rs💻semio🛠️findmodelintype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_model_in_type)

/ <summary>find_model_in_type holds the data fields for a find_model_in_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findmodelintype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_model_in_type)
/ </remarks>

## [👤semio📚rs💻semio🛠️findfileinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_file_in_kit)

/ <summary>find_file_in_kit performs the find_file_in_kit operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findfileinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_file_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findfolderinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_folder_in_kit)

/ <summary>find_folder_in_kit holds the data fields for a find_folder_in_kit record.</summary>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findfolderinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_folder_in_kit)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️findauthorinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_author_in_kit)

/ <summary>find_author_in_kit performs the find_author_in_kit operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findauthorinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_author_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findtaginkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_tag_in_kit)

/ <summary>find_tag_in_kit holds the data fields for a find_tag_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findtaginkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_tag_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findconceptinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_concept_in_kit)

/ <summary>find_concept_in_kit holds the data fields for a find_concept_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findconceptinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_concept_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findqualityinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_quality_in_kit)

/ <summary>find_quality_in_kit holds the data fields for a find_quality_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findqualityinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_quality_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findinterfaceinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_interface_in_kit)

/ <summary>find_interface_in_kit performs the find_interface_in_kit operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findinterfaceinkit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_interface_in_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️findlayerindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_layer_in_design)

/ <summary>find_layer_in_design holds the data fields for a find_layer_in_design record.</summary>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findlayerindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_layer_in_design)

## [👤semio📚rs💻semio🛠️findgroupindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_group_in_design)

/ <summary>find_group_in_design holds the data fields for a find_group_in_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findgroupindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_group_in_design)
/ </remarks>

## [👤semio📚rs💻semio🛠️findstatindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/find_stat_in_design)

/ <summary>find_stat_in_design holds the data fields for a find_stat_in_design record.</summary>
/ [👤semio📚rs💻semio🔖finderfunctions🛠️findstatindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Finder%20Functions/d/i/find_stat_in_design)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️sumqualityindesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/sum_quality_in_design)

/ For each piece, it checks piece-level props first, then falls back to type-level props.

## [👤semio📚rs💻semio🛠️serializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/serialize_kit)

/ <summary>serialize_kit holds the data fields for a serialize_kit record.</summary>
/ [👤semio📚rs💻semio🔖serialization🛠️serializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_kit)

## [👤semio📚rs💻semio🛠️deserializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deserialize_kit)

/ <summary>deserialize_kit holds the data fields for a deserialize_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️deserializekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/deserialize_kit)
/ </remarks>

## [👤semio📚rs💻semio🛠️serializedesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/serialize_design)

/ <summary>serialize_design performs the serialize_design operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️serializedesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_design)
/ </remarks>

## [👤semio📚rs💻semio🛠️deserializedesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deserialize_design)

/ <summary>deserialize_design holds the data fields for a deserialize_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️deserializedesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/deserialize_design)
/ </remarks>

## [👤semio📚rs💻semio🛠️serializetype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/serialize_type)

/ <summary>serialize_type holds the data fields for a serialize_type record.</summary>
/ <summary>serialize_type performs the serialize_type operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️serializetype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/serialize_type)
/ </remarks>

## [👤semio📚rs💻semio🛠️deserializetype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/deserialize_type)

/ <summary>deserialize_type holds the data fields for a deserialize_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️deserializetype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/deserialize_type)
/ </remarks>

## [👤semio📚rs💻semio🛠️arekitsequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/are_kits_equal)

/ <summary>are_kits_equal performs the are_kits_equal operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️arekitsequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/are_kits_equal)
/ </remarks>

## [👤semio📚rs💻semio🛠️aredesignsequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/are_designs_equal)

/ <summary>are_designs_equal holds the data fields for a are_designs_equal record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️aredesignsequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/are_designs_equal)
/ </remarks>

## [👤semio📚rs💻semio🛠️aretypesequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/are_types_equal)

/ <summary>are_types_equal holds the data fields for a are_types_equal record.</summary>
/ [👤semio📚rs💻semio🔖serialization🛠️aretypesequal](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/are_types_equal)

## [👤semio📚rs💻semio🪨supportedmodelextensions](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/c/SUPPORTED_MODEL_EXTENSIONS)

/ <summary>SUPPORTED_MODEL_EXTENSIONS holds the data fields for a SUPPORTED_MODEL_EXTENSIONS record.</summary>
/ [👤semio📚rs💻semio🔖serialization🛠️supportedmodelextensions](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/SUPPORTED_MODEL_EXTENSIONS)

## [👤semio📚rs💻semio🛠️issupportedmodelextension](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/is_supported_model_extension)

/ <summary>is_supported_model_extension performs the is_supported_model_extension operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖serialization🛠️issupportedmodelextension](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Serialization/d/i/is_supported_model_extension)
/ </remarks>

## [👤semio📚rs💻semio🛠️removeditem](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/RemovedItem)

/ <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
/ <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️removeditem](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/RemovedItem)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffupdate](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffUpdate)

/ <summary>DiffUpdate holds the data fields for a DiffUpdate record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️diffupdate](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DiffUpdate)
/ </remarks>

## [👤semio📚rs💻semio🛠️collectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/CollectionDiff)

/ <summary>CollectionDiff holds the data fields for a CollectionDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️collectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/CollectionDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️attributediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AttributeDiff)

/ <summary>AttributeDiff holds the data fields for a AttributeDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️attributediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AttributeDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️propdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PropDiff)

/ <summary>PropDiff holds the data fields for a PropDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️propdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PropDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️connectordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectorDiff)

/ <summary>ConnectorDiff holds the data fields for a ConnectorDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️connectordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectorDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️modeldiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ModelDiff)

/ <summary>ModelDiff holds the data fields for a ModelDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️modeldiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ModelDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️typediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TypeDiff)

/ <summary>TypeDiff holds the data fields for a TypeDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️typediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TypeDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️sidediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/SideDiff)

/ <summary>SideDiff holds the data fields for a SideDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️sidediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/SideDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️connectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConnectionDiff)

/ <summary>ConnectionDiff holds the data fields for a ConnectionDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️connectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectionDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️piecediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PieceDiff)

/ <summary>PieceDiff holds the data fields for a PieceDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️piecediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PieceDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️layerdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/LayerDiff)

/ <summary>LayerDiff holds the data fields for a LayerDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️layerdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/LayerDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️groupdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/GroupDiff)

/ <summary>GroupDiff holds the data fields for a GroupDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️groupdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/GroupDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️statdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/StatDiff)

/ <summary>StatDiff holds the data fields for a StatDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️statdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/StatDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️designdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DesignDiff)

/ <summary>DesignDiff holds the data fields for a DesignDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️designdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DesignDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️tagdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/TagDiff)

/ <summary>TagDiff holds the data fields for a TagDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️tagdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TagDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️conceptdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ConceptDiff)

/ <summary>ConceptDiff holds the data fields for a ConceptDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️conceptdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConceptDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️portdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/PortDiff)

/ <summary>PortDiff holds the data fields for a PortDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️portdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PortDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️qualitydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/QualityDiff)

/ <summary>QualityDiff holds the data fields for a QualityDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️qualitydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/QualityDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️filediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FileDiff)

/ <summary>FileDiff holds the data fields for a FileDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️filediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FileDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️folderdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FolderDiff)

/ <summary>FolderDiff holds the data fields for a FolderDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️folderdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FolderDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️authordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/AuthorDiff)

/ <summary>AuthorDiff holds the data fields for a AuthorDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️authordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AuthorDiff)
/ </remarks>

## [👤semio📚rs💻semio🛠️kitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/KitDiff)

/ <summary>KitDiff holds the data fields for a KitDiff record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️kitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/KitDiff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️change](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/Change)

/ <summary>Change holds the data fields for a Change record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️change](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/Change)
/ </remarks>

## [👤semio📚rs💻semio✂️attributechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/AttributeChange)

/ <summary>AttributeChange holds the data fields for a AttributeChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️attributechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AttributeChange)
/ </remarks>

## [👤semio📚rs💻semio✂️authorchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/AuthorChange)

/ <summary>AuthorChange holds the data fields for a AuthorChange record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️authorchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/AuthorChange)

## [👤semio📚rs💻semio✂️filechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/FileChange)

/ <summary>FileChange holds the data fields for a FileChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️filechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FileChange)
/ </remarks>

## [👤semio📚rs💻semio✂️folderchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/FolderChange)

/ <summary>FolderChange holds the data fields for a FolderChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️folderchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/FolderChange)
/ </remarks>

## [👤semio📚rs💻semio✂️qualitychange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/QualityChange)

/ <summary>QualityChange holds the data fields for a QualityChange record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️qualitychange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/QualityChange)

## [👤semio📚rs💻semio✂️portchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/PortChange)

/ <summary>PortChange holds the data fields for a PortChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️portchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PortChange)
/ </remarks>

## [👤semio📚rs💻semio✂️propchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/PropChange)

/ <summary>PropChange holds the data fields for a PropChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️propchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PropChange)
/ </remarks>

## [👤semio📚rs💻semio✂️tagchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/TagChange)

/ <summary>TagChange holds the data fields for a TagChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️tagchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TagChange)
/ </remarks>

## [👤semio📚rs💻semio✂️conceptchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/ConceptChange)

/ <summary>ConceptChange holds the data fields for a ConceptChange record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️conceptchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConceptChange)

## [👤semio📚rs💻semio✂️modelchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/ModelChange)

/ <summary>ModelChange holds the data fields for a ModelChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️modelchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ModelChange)
/ </remarks>

## [👤semio📚rs💻semio✂️connectorchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/ConnectorChange)

/ <summary>ConnectorChange holds the data fields for a ConnectorChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️connectorchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectorChange)
/ </remarks>

## [👤semio📚rs💻semio✂️typechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/TypeChange)

/ <summary>TypeChange holds the data fields for a TypeChange record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️typechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/TypeChange)

## [👤semio📚rs💻semio✂️layerchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/LayerChange)

/ <summary>LayerChange holds the data fields for a LayerChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️layerchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/LayerChange)
/ </remarks>

## [👤semio📚rs💻semio✂️piecechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/PieceChange)

/ <summary>PieceChange holds the data fields for a PieceChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️piecechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/PieceChange)
/ </remarks>

## [👤semio📚rs💻semio✂️groupchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/GroupChange)

/ <summary>GroupChange holds the data fields for a GroupChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️groupchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/GroupChange)
/ </remarks>

## [👤semio📚rs💻semio✂️sidechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/SideChange)

/ <summary>SideChange holds the data fields for a SideChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️sidechange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/SideChange)
/ </remarks>

## [👤semio📚rs💻semio✂️connectionchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/ConnectionChange)

/ <summary>ConnectionChange holds the data fields for a ConnectionChange record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️connectionchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/ConnectionChange)

## [👤semio📚rs💻semio✂️statchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/StatChange)

/ <summary>StatChange holds the data fields for a StatChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️statchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/StatChange)
/ </remarks>

## [👤semio📚rs💻semio✂️designchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/DesignChange)

/ <summary>DesignChange holds the data fields for a DesignChange record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖difftypes🛠️designchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/DesignChange)
/ </remarks>

## [👤semio📚rs💻semio✂️kitchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/KitChange)

/ <summary>KitChange holds the data fields for a KitChange record.</summary>
/ [👤semio📚rs💻semio🔖difftypes🛠️kitchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Diff%20Types/d/i/KitChange)

## [👤semio📚rs💻semio✂️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️hasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/HasGuid)
/ </remarks>

## [👤semio📚rs💻semio✂️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/f/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖hasguidtrait🛠️diffhasguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/HasGuid%20Trait/d/i/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semio🛠️applycollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_collection_diff)

/ <summary>apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applycollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_collection_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️applyattributediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_attribute_diff)

/ <summary>apply_attribute_diff performs the apply_attribute_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyattributediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_attribute_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applypropdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_prop_diff)

/ <summary>apply_prop_diff holds the data fields for a apply_prop_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applypropdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_prop_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applyconnectordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_connector_diff)

/ <summary>apply_connector_diff holds the data fields for a apply_connector_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyconnectordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_connector_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applymodeldiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_model_diff)

/ <summary>apply_model_diff performs the apply_model_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applymodeldiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_model_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applytypediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_type_diff)

/ <summary>apply_type_diff holds the data fields for a apply_type_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applytypediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_type_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applylayerdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_layer_diff)

/ <summary>apply_layer_diff holds the data fields for a apply_layer_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applylayerdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_layer_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️applygroupdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_group_diff)

/ <summary>apply_group_diff holds the data fields for a apply_group_diff record.</summary>
/ <summary>apply_group_diff performs the apply_group_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applygroupdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_group_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applystatdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_stat_diff)

/ <summary>apply_stat_diff holds the data fields for a apply_stat_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applystatdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_stat_diff)

## [👤semio📚rs💻semio🛠️applypiecediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_piece_diff)

/ <summary>apply_piece_diff holds the data fields for a apply_piece_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applypiecediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_piece_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applyconnectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_connection_diff)

/ <summary>apply_connection_diff holds the data fields for a apply_connection_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyconnectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_connection_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applydesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_design_diff)

/ <summary>apply_design_diff performs the apply_design_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applydesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_design_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applytagdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_tag_diff)

/ <summary>apply_tag_diff holds the data fields for a apply_tag_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applytagdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_tag_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️applyconceptdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_concept_diff)

/ <summary>apply_concept_diff holds the data fields for a apply_concept_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyconceptdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_concept_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️applyinterfacediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_interface_diff)

/ <summary>apply_interface_diff holds the data fields for a apply_interface_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyinterfacediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_interface_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️applyqualitydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_quality_diff)

/ <summary>apply_quality_diff holds the data fields for a apply_quality_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyqualitydiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_quality_diff)

## [👤semio📚rs💻semio🛠️applyfilediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_file_diff)

/ <summary>apply_file_diff holds the data fields for a apply_file_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyfilediff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_file_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applyfolderdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_folder_diff)

/ <summary>apply_folder_diff performs the apply_folder_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyfolderdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_folder_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️applyauthordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_author_diff)

/ <summary>apply_author_diff holds the data fields for a apply_author_diff record.</summary>
/ [👤semio📚rs💻semio🔖applydiff🛠️applyauthordiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_author_diff)

## [👤semio📚rs💻semio🛠️applykitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_kit_diff)

/ <summary>apply_kit_diff holds the data fields for a apply_kit_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖applydiff🛠️applykitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/ApplyDiff/d/i/apply_kit_diff)
/ </remarks>

## [👤semio📚rs💻semio🛠️getguidcollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_guid_collection_diff)

/ Computes a CollectionDiff between two optional collections of guid-identified items.
/ Uses a caller-provided `compute_diff` function for entity-level diffs.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️getguidcollectiondiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_guid_collection_diff)

## [👤semio📚rs💻semio🛠️getkitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_kit_diff)

/ Computes the KitDiff that transforms `before` into `after`.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️getkitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_kit_diff)

## [👤semio📚rs💻semio🛠️getdesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_design_diff)

/ Computes the DesignDiff that transforms `before` into `after`.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️getdesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_design_diff)

## [👤semio📚rs💻semio🛠️inversekitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/inverse_kit_diff)

/ Computes the inverse of a KitDiff given the original Kit state.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️inversekitdiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/inverse_kit_diff)

## [👤semio📚rs💻semio🛠️inversedesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/inverse_design_diff)

/ Computes the inverse of a DesignDiff given the original Design state.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️inversedesigndiff](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/inverse_design_diff)

## [👤semio📚rs💻semio🛠️getkitchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_kit_change)

/ Computes a reversible KitChange from two kit states.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️getkitchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_kit_change)

## [👤semio📚rs💻semio🛠️getdesignchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_design_change)

/ Computes a reversible DesignChange from two design states.
/ [👤semio📚rs💻semio🔖kitchangehelpers🛠️getdesignchange](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Change%20Helpers/d/i/get_design_change)

## [👤semio📚rs💻semio🛠️flattenedpiece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/FlattenedPiece)

/ <summary>FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
/ [👤semio📚rs💻semio🔖flattendesign🛠️flattenedpiece](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/FlattenedPiece)

## [👤semio📚rs💻semio🛠️flattendesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/flatten_design)

/ <summary>flatten_design holds the data fields for a flatten_design record.</summary>
/ [👤semio📚rs💻semio🔖flattendesign🛠️flattendesign](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/flatten_design)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️planesequalapprox](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/planes_equal_approx)

/ <summary>planes_equal_approx holds the data fields for a planes_equal_approx record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️planesequalapprox](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/planes_equal_approx)
/ </remarks>

## [👤semio📚rs💻semio🛠️computeconnectionmatrixfast](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/compute_connection_matrix_fast)

/ <summary>compute_connection_matrix_fast holds the data fields for a compute_connection_matrix_fast record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️computeconnectionmatrixfast](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/compute_connection_matrix_fast)
/ </remarks>

## [👤semio📚rs💻semio🛠️computechildplanematrix](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/compute_child_plane_matrix)

/ <summary>compute_child_plane_matrix holds the data fields for a compute_child_plane_matrix record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️computechildplanematrix](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/compute_child_plane_matrix)
/ </remarks>

## [👤semio📚rs💻semio🛠️quattomatrix4](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/quat_to_matrix4)

/ <summary>quat_to_matrix4 holds the data fields for a quat_to_matrix4 record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️quattomatrix4](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/quat_to_matrix4)
/ </remarks>

## [👤semio📚rs💻semio🛠️maketranslation](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/make_translation)

/ <summary>make_translation holds the data fields for a make_translation record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️maketranslation](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/make_translation)
/ </remarks>

## [👤semio📚rs💻semio🛠️applymatrix4tovec3](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/apply_matrix4_to_vec3)

/ <summary>apply_matrix4_to_vec3 holds the data fields for a apply_matrix4_to_vec3 record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️applymatrix4tovec3](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/apply_matrix4_to_vec3)
/ </remarks>

## [👤semio📚rs💻semio🛠️getconnectorforsidefast](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_connector_for_side_fast)

/ <summary>get_connector_for_side_fast holds the data fields for a get_connector_for_side_fast record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️getconnectorforsidefast](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/get_connector_for_side_fast)
/ </remarks>

## [👤semio📚rs💻semio🛠️getconnectorfromtype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/get_connector_from_type)

/ <summary>get_connector_from_type holds the data fields for a get_connector_from_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️getconnectorfromtype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/get_connector_from_type)
/ </remarks>

## [👤semio📚rs💻semio🛠️connectortoplane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/connector_to_plane)

/ <summary>connector_to_plane holds the data fields for a connector_to_plane record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖flattendesign🛠️connectortoplane](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/FlattenDesign/d/i/connector_to_plane)
/ </remarks>

## [👤semio📚rs💻semio🪨exportmodelformats](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/c/EXPORT_MODEL_FORMATS)

/ <summary>Supported 3D model export formats (extension, description).</summary>
/ [👤semio📚rs💻semio🔖kitmodelexport🛠️exportmodelformats](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/EXPORT_MODEL_FORMATS)

## [👤semio📚rs💻semio🛠️matrix4togltfcolumnmajor](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/matrix4_to_gltf_column_major)

/ <summary>Converts a nalgebra Matrix4 to glTF column-major array of 16 f64.</summary>
/ [👤semio📚rs💻semio🔖kitmodelexport🛠️matrix4togltfcolumnmajor](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/matrix4_to_gltf_column_major)

## [👤semio📚rs💻semio🛠️selectmodelfortype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/select_model_for_type)

/ <summary>Selects the best model for a type given desired tag guids.</summary>
/ [👤semio📚rs💻semio🔖kitmodelexport🛠️selectmodelfortype](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Kit%20Model%20Export/d/i/select_model_for_type)

## [👤semio📚rs💻semio🛠️validationproblem](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ValidationProblem)

/ <summary>ValidationProblem holds the data fields for a ValidationProblem record.</summary>
/ <remarks>
/ </remarks>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️validationproblem](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/ValidationProblem)
/ </remarks>

## [👤semio📚rs💻semio🛠️validationfix](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ValidationFix)

/ <summary>ValidationFix holds the data fields for a ValidationFix record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️validationfix](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/ValidationFix)
/ </remarks>

## [👤semio📚rs💻semio🛠️validationresult](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/ValidationResult)

/ <summary>ValidationResult holds the data fields for a ValidationResult record.</summary>
/ [👤semio📚rs💻semio🔖validationtypes🛠️validationresult](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/ValidationResult)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semio🛠️validatekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/validate_kit)

/ <summary>validate_kit holds the data fields for a validate_kit record.</summary>
/ [👤semio📚rs💻semio🔖validationtypes🛠️validatekit](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/validate_kit)

## [👤semio📚rs💻semio🛠️checkguiduniquenessconstraint](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_guid_uniqueness_constraint)

/ <summary>check_guid_uniqueness_constraint holds the data fields for a check_guid_uniqueness_constraint record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkguiduniquenessconstraint](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_guid_uniqueness_constraint)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_guid)

/ <summary>check_guid holds the data fields for a check_guid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkguid](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_guid)
/ </remarks>

## [👤semio📚rs💻semio🛠️checktypenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_type_name_uniqueness)

/ <summary>check_type_name_uniqueness holds the data fields for a check_type_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checktypenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_type_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkdesignnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_design_name_uniqueness)

/ <summary>check_design_name_uniqueness holds the data fields for a check_design_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkdesignnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_design_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkpiecenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_piece_name_uniqueness)

/ <summary>check_piece_name_uniqueness holds the data fields for a check_piece_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkpiecenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_piece_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkconnectionnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_connection_name_uniqueness)

/ <summary>check_connection_name_uniqueness holds the data fields for a check_connection_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkconnectionnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_connection_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkconnectornameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_connector_name_uniqueness)

/ <summary>check_connector_name_uniqueness holds the data fields for a check_connector_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkconnectornameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_connector_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkmodelnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_model_name_uniqueness)

/ <summary>check_model_name_uniqueness holds the data fields for a check_model_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkmodelnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_model_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checklayerpathuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_layer_path_uniqueness)

/ <summary>check_layer_path_uniqueness holds the data fields for a check_layer_path_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checklayerpathuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_layer_path_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkqualitynameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_quality_name_uniqueness)

/ <summary>check_quality_name_uniqueness holds the data fields for a check_quality_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkqualitynameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_quality_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkportnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_port_name_uniqueness)

/ <summary>check_port_name_uniqueness holds the data fields for a check_port_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkportnameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_port_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkfilenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_file_name_uniqueness)

/ <summary>check_file_name_uniqueness holds the data fields for a check_file_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkfilenameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_file_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️checkfoldernameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/check_folder_name_uniqueness)

/ <summary>check_folder_name_uniqueness holds the data fields for a check_folder_name_uniqueness record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖validationtypes🛠️checkfoldernameuniqueness](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Validation%20Types/d/i/check_folder_name_uniqueness)
/ </remarks>

## [👤semio📚rs💻semio🛠️sqlite](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/sqlite)

/ <summary>sqlite holds the data fields for a sqlite record.</summary>
/ [👤semio📚rs💻semio🔖sqliteimport🔖export🛠️sqlite](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/SQLite%20Import/Export/d/i/sqlite)

## [👤semio📚rs💻semio🛠️mimefromfilename](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/mime_from_filename)

/ <summary>zip_roundtrip holds the data fields for a zip_roundtrip record.</summary>
/ [👤semio📚rs💻semio🔖zipimport🔖export🛠️ziproundtrip](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Zip%20Import/Export/d/i/zip_roundtrip)

## [👤semio📚rs💻semio🛠️wasm](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/wasm)

/ <summary>wasm holds the data fields for a wasm record.</summary>
/ [👤semio📚rs💻semio🔖wasmbindings🛠️wasm](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/WASM%20Bindings/d/i/wasm)

## [👤semio📚rs💻semio🛠️tests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/d/i/tests)

/ <summary>tests holds the data fields for a tests record.</summary>
/ <remarks>
/ [👤semio📚rs💻semio🔖tests🛠️tests](semiorepo://p/u/semio/b/l/rs/f/semio.rs/s/Tests/d/i/tests)
/ </remarks>

## [👤semio🌐docs💻index](semiorepo://p/u/semio/b/w/docs/f/index.tsx)

Entry point for the documentation site React app.

## [👤semio🌐docs💻index🔖entrypoint](semiorepo://p/u/semio/b/w/docs/f/index.tsx/s/Entrypoint)

Docs entrypoint that mounts the Sketchpad React component with StrictMode.

## [👤semio🌐play💻index](semiorepo://p/u/semio/b/w/play/f/index.tsx)

Entry point for the playground React app for interactive experimentation.

## [👤semio🌐play💻index🔖entrypoint](semiorepo://p/u/semio/b/w/play/f/index.tsx/s/Entrypoint)

Play application entrypoint registering sketchpad apps and rendering the root.

## [👤semio🖱️sketchpad💻globald](semiorepo://p/u/semio/b/u/sketchpad/f/global.d.ts)

Global type declarations for the JavaScript workspace.

## [👤semio🖱️sketchpad💻i18n](semiorepo://p/u/semio/b/u/sketchpad/f/i18n.ts)

Internationalization setup and translation utilities for the UI.

## [👤semio🖱️sketchpad💻i18n🔖i18n](semiorepo://p/u/semio/b/u/sketchpad/f/i18n.ts/s/I18n)

Initializes i18next with language detection, React bindings and expertise-aware label hooks.

## [👤semio🖱️sketchpad💻index](semiorepo://p/u/semio/b/u/sketchpad/f/index.ts)

Public bundle exports for the semio sketchpad runtime and app configs.

## [👤semio🖱️sketchpad💻index🔖exports](semiorepo://p/u/semio/b/u/sketchpad/f/index.ts/s/Exports)

Public API surface for sketchpad runtime, shared helpers, and app configs.

## [👤semio🖱️sketchpad💻index](semiorepo://p/u/semio/b/u/sketchpad/f/index.tsx)

Entry point for the standalone sketchpad web application.

## [👤semio🖱️sketchpad💻index🔖entrypoint](semiorepo://p/u/semio/b/u/sketchpad/f/index.tsx/s/Entrypoint)

Sketchpad application entrypoint registering apps and rendering the root.

## [👤semio🖱️sketchpad🗃️sketchpad💻design](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Design.tsx)

Design app providing diagram and scene windows for editing designs.

## [👤semio🖱️sketchpad🗃️sketchpad💻design🪨emptyedgesarray](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Design.tsx/d/c/EMPTY_EDGES_ARRAY)


## [👤semio🖱️sketchpad🗃️sketchpad💻docs](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Docs.tsx)

js/semio/sketchpad/Docs.tsx

## [👤semio🖱️sketchpad🗃️sketchpad💻feedback](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Feedback.tsx)

js/semio/sketchpad/Feedback.tsx

## [👤semio🖱️sketchpad🗃️sketchpad💻home](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Home.tsx)

js/semio/sketchpad/Home.tsx

## [👤semio🖱️sketchpad🗃️sketchpad💻kit](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Kit.tsx)

Kit editor app for managing types, designs and qualities.

## [👤semio🖱️sketchpad🗃️sketchpad💻quality](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Quality.tsx)

js/semio/sketchpad/Quality.tsx

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx)

Main sketchpad container managing app tabs, panels and window layout.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖imports](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Imports)

External and internal module imports.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖store](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Store)

Reactive stores backed by Yjs for collaborative state management.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖plainappstorenoyjs](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Plain%20App%20Store%20(No%20YJS))

Non-YJS application stores using plain in-memory state with transaction support.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖fileprovider](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/File%20Provider)

In-memory file storage provider for temporary or test scenarios.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖memoryfileprovider](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Memory%20File%20Provider)

In-memory file storage provider for temporary or test scenarios.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖localfileproviderindexeddb](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Local%20File%20Provider%20(IndexedDB))

Browser-local file storage provider backed by IndexedDB.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖remotefileprovider](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Remote%20File%20Provider)

Remote file storage provider backed by a REST API.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖compositefileprovider](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Composite%20File%20Provider)

Composite file storage provider that delegates to multiple underlying providers.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖kits](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Kits)

Yjs-backed attribute store for kit metadata.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖coord](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Coord)

Yjs-backed coordinate store managing u/v values.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖vec](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Vec)

Yjs-backed 3D vector component store managing x/y/z values.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖point](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Point)

Yjs-backed 3D point store managing x/y/z coordinates.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖vector](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Vector)

Yjs-backed 3D direction vector store managing x/y/z components.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖plane](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Plane)

Yjs-backed 3D plane store managing origin point and direction vectors.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖camera](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Camera)

Yjs-backed camera store managing view target and perspective planes.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖location](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Location)

Yjs-backed location store managing geographical and licensing metadata.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖author](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Author)

Yjs-backed author store managing author identity and attributes.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖file](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/File)

Yjs-backed file store managing file metadata and content references.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖folder](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Folder)

Yjs-backed folder store managing folder hierarchy and file references.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖benchmark](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Benchmark)

Yjs-backed benchmark store managing performance measurement data.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖quality](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Quality)

Yjs-backed quality store managing quality criteria definitions.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖prop](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Prop)

Yjs-backed prop store managing design property definitions.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖model](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Model)

Yjs-backed model store managing 3D model representations.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖connector](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Connector)

Yjs-backed connector store managing type connectors and their ports.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖type](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Type)

Yjs-backed type store managing architectural type definitions and connectors.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖layer](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Layer)

Yjs-backed layer store managing visibility layers in designs.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖piece](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Piece)

Yjs-backed piece store managing design piece instances and their transforms.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖group](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Group)

Yjs-backed group store managing piece grouping within designs.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖side](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Side)

Side store managing connection endpoints for pieces.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖connection](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Connection)

Yjs-backed connection store managing piece-to-piece connections.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖stat](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Stat)

Yjs-backed stat store managing statistical measurement data.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖design](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Design)

Yjs-backed design store managing complete design layouts with pieces and connections.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖ypathapi](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/YPath%20API)

Path-based observation and subscription API for deep design Yjs map access.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖kit](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Kit)

Yjs-backed kit store managing the complete kit data structure.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖ypathapi](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/YPath%20API)

Path-based observation and subscription API for deep kit Yjs map access.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖targetedkithooks](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Targeted%20Kit%20Hooks)

React hooks for accessing specific kit data through scope providers.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖commands](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Commands)

Kit command definitions for import, export, and sync operations.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖machine](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Machine)

Type definitions for app state, machine input, and context structures.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖types](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Types)

Type definitions for app state, machine input, and context structures.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖appstatetypes](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/App%20State%20Types)

State shape interfaces for all application views: home, kit, design, type, quality.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖helpers](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Helpers)

Helper functions for path migration, default state creation, and store initialization.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖sketchpadmachine](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Sketchpad%20Machine)

XState state machine definition for the sketchpad application lifecycle.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖sketchpadselectors](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Sketchpad%20Selectors)

Selector functions for extracting state from the sketchpad machine context.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖factory](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Factory)

Factory function to instantiate the sketchpad actor.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖legacytypeexports](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Legacy%20Type%20Exports)

Legacy type exports for backward compatibility with existing consumers.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖actortypes](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Actor%20Types)

Type aliases for the sketchpad XState actor references and snapshots.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖apps](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Apps)

App-specific hooks for design, type, kit, and sketchpad views.
Design app hooks for piece and connection selection, hover, and diff state.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖design](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Design)

Design app hooks for piece and connection selection, hover, and diff state.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖sketchpad](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Sketchpad)

Core reactive observation, synchronization hooks, and sketchpad store implementation.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖xstatehooks](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/XState%20Hooks)

React hooks for accessing XState sketchpad actor state and sending events.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖commands](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Commands)

Exported sketchpad command map for theme, language, mode, device, and navigation.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖appsregistry](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Apps%20Registry)

Dynamic app panel loader for registering app-specific panels.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖navbar](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Navbar)

Focus-based navigation context provider for navbar breadcrumbs and search.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖sidepaneltabs](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/SidePanel%20Tabs)

Context provider managing side panel and HUD panel tab registration.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖origin](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Origin)

Context provider for tracking the origin URL of the sketchpad instance.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖footeritems](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Footer%20Items)

Context provider for dynamically registering footer bar items.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖globalfooteritems](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Global%20Footer%20Items)

Global footer items component that registers persistent footer entries.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖conceptfilter](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/ConceptFilter)

Filter component for narrowing results by architectural concepts.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖toolgroup](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/ToolGroup)

Toolbar group component for switching between tool modes.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖dragdrop](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/DragDrop)

Context provider for drag-and-drop type placement interactions.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖hotkeys](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Hotkeys)

Keyboard shortcut hook with configurable hotkey overrides.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖canvas](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Canvas)

Canvas layout components for window management and multi-pane rendering.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖approuter](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/App%20Router)

React Router integration with scope providers and route-based app switching.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🔖sketchpadcomponents](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/s/Sketchpad%20Components)

Top-level sketchpad React components for rendering the complete application.

## [👤semio🖱️sketchpad🗃️sketchpad💻sketchpad🪨selectuiisinkit](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Sketchpad.tsx/d/c/selectUiIsInKit)

selectUiIsInKit holds the data fields for a selectUiIsInKit record.

## [👤semio🖱️sketchpad🗃️sketchpad💻tutorials](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Tutorials.tsx)

js/semio/sketchpad/Tutorials.tsx

## [👤semio🖱️sketchpad🗃️sketchpad💻type](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/Type.tsx)

js/semio/sketchpad/Type.tsx

## [👤semio🖱️sketchpad🗃️sketchpad🗃️apps💻index](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/fd/org/apps/f/index.ts)

js/semio/sketchpad/apps/index.ts

## [👤semio🖱️sketchpad🗃️sketchpad💻kitselectionhelper](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/kitSelectionHelper.ts)

Geometry and selection utilities for kit diagram interactions.

## [👤semio🖱️sketchpad🗃️sketchpad💻kitselectionhelpers](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/kitSelectionHelpers.ts)

js/semio/sketchpad/kitSelectionHelpers.ts

## [👤semio🖱️sketchpad🗃️sketchpad💻portcolor🔖portcolor](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/portColor.ts/s/Port%20Color)

Assigns deterministic HSL color tones to ports based on compatibility groups.

## [👤semio🖱️sketchpad🗃️sketchpad💻shared](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/shared.ts)

js/semio/sketchpad/shared.ts

## [👤semio🖱️sketchpad🗃️sketchpad💻shared🔖focus](semiorepo://p/u/semio/b/u/sketchpad/fd/org/sketchpad/f/shared.ts/s/Focus)

FocusItem is re-exported from elements.tsx as UIFindItem.

## [👤semio🖱️sketchpad💻viteenvd](semiorepo://p/u/semio/b/u/sketchpad/f/vite-env.d.ts)

Vite client type declarations for the JavaScript workspace.

## [👤semio🖱️sketchpad💻viteenvd🔖declarations](semiorepo://p/u/semio/b/u/sketchpad/f/vite-env.d.ts/s/Declarations)

Ambient module declarations for non-standard import types.
