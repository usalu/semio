# 📚 Docs

## [👤semio🏪assets](semiorepo://bundle/semio/assets)

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

## [👤semio🖱️desktop](semiorepo://bundle/semio/desktop)

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

## [👤semio🌐docs](semiorepo://bundle/semio/docs)

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

## [👤semio📚engine](semiorepo://bundle/semio/engine)

## Files

- `engine.py` - Main engine module with Kit parsing, validation, transformation, dev-mode startup flag, and stdio MCP startup flag
- `engine.test.py` - Unit tests for engine functionality
- `generate-schemas.ts` - Generates GraphQL, JSON, and SQL schemas from TypeScript definitions
- `sqliteschema.ts` - SQLite schema generation utilities

## [👤semio📚js](semiorepo://bundle/semio/js)

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
- NEVER add semantic values and ALWAYS use hardcoded values in `theme.css`. NEVER use `theme.css` outside of `global.css`.
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

- `js/semio/sketchpad/elements.tsx` defines `TransactionProvider` and `useTransaction()`.
- `js/semio/sketchpad/elements.tsx` `Geometry` treats `color` as the base (non-interactive) color and uses selection/hover theme colors for the rendered material/edges when `selected`/`hovered` are true.
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
- **MIDDLE**: HUD, Stats (grouped, transparent)
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

- `semio/js/sketchpad/elements.tsx` provides `TransactionProvider` and `useTransaction()` for UI-scoped transactions.
- Sketchpad elements (`Input`, `Textarea`, `Select`, `Slider`, `Stepper`, `Combobox`, ...) use `useTransaction()` internally and do not accept a `transaction` prop.
- Apps wrap their UI subtree with `TransactionProvider` using the appropriate transaction hook so all descendant elements participate in undo/redo consistently.

### Sketchpad selection + hover visuals

- `semio/js/sketchpad/elements.tsx` `Geometry` renders selection/hover colors even when a base `color` is provided (it is treated as the non-interactive default).
- Hover and selection state for Home/Kit/Design/Type/Quality/Docs/Feedback is stored in the Sketchpad state machine; UI rows and diagram nodes dispatch hover events and visuals read from machine state.
- `semio/js/sketchpad/elements.tsx` `Table` exposes row hover callbacks so apps can forward row enter/leave events into their state machine commands.
- `semio/js/sketchpad/Design.tsx` diagram nodes use `ring-*` (not `ring-inset`) so hover/selection rings remain visible with `AvatarFallback` backgrounds.

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
- `layout` naming is reserved for window layout configs (GoldenLayout) and the `Layout` component in `semio/js/sketchpad/elements.tsx`.

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

All size constants are defined in `semio/js/globals.css` and derived from `--spacing`:

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

## Preflight

```bash
cd semio/js && npm run preflight
```

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

## [👤semio📚js🗃️sketchpad](semiorepo://folder/semio/js/sketchpad)

## elements.tsx

`Table` supports row-level hover callbacks for app hover state dispatch.

## Home.tsx

Home app hover state is stored in the Sketchpad state machine and updated via hover commands for table rows.

## Kit.tsx

Kit app hover state covers all artifact kinds and is updated via table and diagram hover dispatch.

## Sketchpad.tsx

Home command hooks forward hover events, including clear, into the Sketchpad state machine.

## [👤semio📚net](semiorepo://bundle/semio/net)

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

## [👤semio🏪assets💻iconsts](semiorepo://file/semio/assets/icons.ts)

Re-exports Lucide React icons with domain-specific semantic aliases.

## [👤semio🏪assets💻iconsts🔖exports](semiorepo://section/Exports)

Re-exports of Lucide React icons with semantic aliases for the UI.

## [👤semio🏪assets💻indexts](semiorepo://file/semio/assets/index.ts)

Barrel export for all asset modules including icons, fonts, models and images.

## [👤semio🏪assets💻indexts🛠️buildlookup](semiorepo://definition/semio/assets/index.ts/buildLookup)

Builds guid and name lookup maps from an item array

## [👤semio🏪assets💻indexts🪨typelookup](semiorepo://definition/semio/assets/index.ts/typeLookup)

Type lookup maps by guid and name

## [👤semio🏪assets💻indexts🪨designlookup](semiorepo://definition/semio/assets/index.ts/designLookup)

Design lookup maps by guid and name

## [👤semio🏪assets💻indexts🪨portlookup](semiorepo://definition/semio/assets/index.ts/portLookup)

Port lookup maps by guid and name

## [👤semio🏪assets💻indexts🪨nakagincapsuletowerdesign](semiorepo://definition/semio/assets/index.ts/nakaginCapsuleTowerDesign)

Nakagin Capsule Tower root design reference

## [👤semio🏪assets💻indexts🪨nakagincapsuletowerflatdesign](semiorepo://definition/semio/assets/index.ts/nakaginCapsuleTowerFlatDesign)

Nakagin Capsule Tower Flat variant design reference

## [👤semio🏪assets🛅logo💻logots](semiorepo://file/semio/assets/logo/logo.ts)

Generates animated SVG logo from static SVG input with keyframe sequences.

## [👤semio🏪assets🛅logo💻logots🔖types](semiorepo://section/Types)

Type definitions for SVG transform, group, and keyframe data structures.

## [👤semio🏪assets🛅logo💻logots🔖logogeneration](semiorepo://section/Logo%20Generation)

Functions for parsing SVG files and generating animated SVG logos.

## [👤semio🏪assets🛅logo💻logots🔖parsesvg](semiorepo://section/Parse%20SVG)

Parses an SVG file and returns keyframe data with group transforms and paths.

## [👤semio🏪assets🛅logo💻logots🔖generatekeyframesequence](semiorepo://section/Generate%20Keyframe%20Sequence)

Generates a palindromic keyframe sequence with triple repetition per frame.

## [👤semio🏪assets🛅logo💻logots🔖createanimatedsvg](semiorepo://section/Create%20Animated%20SVG)

Creates an animated SVG file with SMIL animations from keyframe data.

## [👤semio🏪assets🛅logo💻logots✂️transformdata](semiorepo://definition/semio/assets/logo/logo.ts/TransformData)

Type definitions for SVG transform, group, and keyframe data structures.

## [👤semio🏪assets🛅logo💻logots🛠️transformtomatrix](semiorepo://definition/semio/assets/logo/logo.ts/transformToMatrix)

Functions for parsing SVG files and generating animated SVG logos.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻fileemptyregiontsx](semiorepo://file/semio/assets/repo/some/folder/file_empty_region.tsx)

An empty region TypeScript file for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixabletsx](semiorepo://file/semio/assets/repo/some/folder/file_fixable.tsx)

A fixable TypeScript file for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixableexpectedtsx](semiorepo://file/semio/assets/repo/some/folder/file_fixable_expected.tsx)

A fixable TypeScript file for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedcs](semiorepo://file/semio/assets/repo/some/folder/file_fixed.cs)

A fixed CSharp class for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedcs🛠️fixedclass](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.cs/FixedClass)

/ <summary>Represents a fixed value container.</summary>
/ <remarks>
/ [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedcs🔖classes🛠️fixedclass](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.cs/Classes/FixedClass)
/ </remarks>

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo](semiorepo://file/semio/assets/repo/some/folder/file_fixed.go)

A fixed Go module for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo🔖package](semiorepo://section/Package)

Package declaration for fixed module.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo🔖functions](semiorepo://section/Functions)

Utility functions for fixed values.
FixedValue returns a constant integer.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedgo🛠️fixedvalue](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.go/FixedValue)

FixedValue returns a constant integer.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedpy](semiorepo://file/semio/assets/repo/some/folder/file_fixed.py)

A fixed Python module for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedpy🔖functions](semiorepo://section/Functions)

Utility functions for fixed values.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx](semiorepo://file/semio/assets/repo/some/folder/file_fixed.tsx)

A fixed TypeScript component for testing.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx🔖types](semiorepo://section/Types)

Type definitions for the fixed component.
Properties of a fixed component.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx🔖components](semiorepo://section/Components)

Rendering components for fixed types.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx✂️fixedtype](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.tsx/FixedType)

Properties of a fixed component.

## [👤semio🏪assets🗃️repo🗃️some🗃️folder💻filefixedtsx✂️fixedkind](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.tsx/FixedKind)

Kind alternatives for fixed types.

## [👤semio🖱️desktop💻forgeenvdts](semiorepo://file/semio/desktop/forge.env.d.ts)

Type declarations for Electron Forge environment variables.

## [👤semio🖱️desktop💻forgeenvdts🔖electronfuses](semiorepo://section/Electron%20Fuses)

Type declarations for Electron Forge fuse options.

## [👤semio🖱️desktop💻maints](semiorepo://file/semio/desktop/main.ts)

Entry point for the Electron main process managing windows and lifecycle.

## [👤semio🖱️desktop💻maints🔖mainprocess](semiorepo://section/Main%20Process)

Electron main process that creates the browser window and registers IPC handlers.

## [👤semio🖱️desktop💻maints🛠️createwindow](semiorepo://definition/semio/desktop/main.ts/createWindow)

Creates the main Electron browser window with preload and vite integration.

## [👤semio🖱️desktop💻preloadts](semiorepo://file/semio/desktop/preload.ts)

Electron preload script exposing safe APIs to the renderer process.

## [👤semio🖱️desktop💻preloadts🔖preload](semiorepo://section/Preload)

Electron preload script exposing window controls and OS APIs to the renderer.

## [👤semio🖱️desktop💻renderertsx](semiorepo://file/semio/desktop/renderer.tsx)

Entry point for the Electron renderer process mounting the React app.

## [👤semio🖱️desktop💻renderertsx🔖renderer](semiorepo://section/Renderer)

Electron renderer process that mounts the Sketchpad React app with window controls.

## [👤semio🖱️desktop💻renderertsx🛠️invokewindowcontrol](semiorepo://definition/semio/desktop/renderer.tsx/invokeWindowControl)

Invokes a window control action via the preload bridge.

## [👤semio🖱️desktop💻renderertsx🪨windowevents](semiorepo://definition/semio/desktop/renderer.tsx/windowEvents)

Window event handlers for minimize, maximize and close actions.

## [👤semio🖱️desktop💻renderertsx🪨os](semiorepo://definition/semio/desktop/renderer.tsx/os)

OS bridge for retrieving the current user identity.

## [👤semio🖱️desktop💻renderertsx🛠️app](semiorepo://definition/semio/desktop/renderer.tsx/App)

Root React component that loads the user identity and renders the sketchpad.

## [👤semio🌐docs💻indextsx](semiorepo://file/semio/docs/index.tsx)

Entry point for the documentation site React app.

## [👤semio🌐docs💻indextsx🔖entrypoint](semiorepo://section/Entrypoint)

Docs entrypoint that mounts the Sketchpad React component with StrictMode.

## [👤semio📚engine💻buildts](semiorepo://file/semio/engine/build.ts)

Build script for the semio engine Python package.

## [👤semio📚engine💻buildts🪨cwd](semiorepo://definition/semio/engine/build.ts/cwd)

Engine build working directory.

## [👤semio📚engine💻buildts🪨args](semiorepo://definition/semio/engine/build.ts/args)

PyInstaller CLI arguments for bundling the engine binary.

## [👤semio📚engine💻generateschemasts](semiorepo://file/semio/engine/generate-schemas.ts)

Generates JSON schemas from the engine's Python models.

## [👤semio📚engine💻postbuildts](semiorepo://file/semio/engine/post-build.ts)

Post-build script for engine artifact processing and packaging.

## [👤semio📚engine💻postbuildts🪨cwd](semiorepo://definition/semio/engine/post-build.ts/cwd)

Post-build working directory.

## [👤semio📚engine💻postbuildts🪨exepath](semiorepo://definition/semio/engine/post-build.ts/exePath)

Path to the PyInstaller-produced engine executable.

## [👤semio📚engine💻postbuildts🪨internalpath](semiorepo://definition/semio/engine/post-build.ts/internalPath)

Path to the PyInstaller internal dependencies folder.

## [👤semio📚engine💻postbuildts🪨grasshopperbinpath](semiorepo://definition/semio/engine/post-build.ts/grasshopperBinPath)

Grasshopper plugin binary output directory.

## [👤semio📚engine💻postbuildts🪨grasshopperexepath](semiorepo://definition/semio/engine/post-build.ts/grasshopperExePath)

Target path for the engine executable in the Grasshopper bin folder.

## [👤semio📚engine💻postbuildts🪨grasshopperinternalpath](semiorepo://definition/semio/engine/post-build.ts/grasshopperInternalPath)

Target path for the internal dependencies in the Grasshopper bin folder.

## [👤semio📚engine💻sqliteschemats](semiorepo://file/semio/engine/sqliteschema.ts)

Exports the SQLite schema definition for the engine database.

## [👤semio📚engine💻sqliteschemats🪨dbpath](semiorepo://definition/semio/engine/sqliteschema.ts/dbPath)

Path to the debug SQLite database.

## [👤semio📚engine💻sqliteschemats🪨outputpath](semiorepo://definition/semio/engine/sqliteschema.ts/outputPath)

Path to the exported SQL schema file.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs](semiorepo://file/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs)

Main Grasshopper plugin providing domain components for Rhino.

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️goo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Goo)

/ Generic Grasshopper data wrapper for semio entity types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️goo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/Goo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️param](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Param)

/ Generic Grasshopper parameter for semio entity types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️param](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/Param)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️enumgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EnumGoo)

/ Generic Grasshopper data wrapper for enum values.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️enumgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EnumGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️enumparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EnumParam)

/ Generic Grasshopper parameter for enum values.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️enumparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EnumParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️passthroughcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/PassthroughComponent)

/ Abstract Grasshopper component that passes input through transformation.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️passthroughcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/PassthroughComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️idgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/IdGoo)

/ Generic Grasshopper data wrapper for entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️idgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/IdGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️idparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/IdParam)

/ Generic Grasshopper parameter for entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️idparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/IdParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️idcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/IdComponent)

/ Abstract Grasshopper component for constructing entity IDs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️idcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/IdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️diffgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DiffGoo)

/ Generic Grasshopper data wrapper for entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️diffgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/DiffGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️diffparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DiffParam)

/ Generic Grasshopper parameter for entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️diffparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/DiffParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️diffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DiffComponent)

/ Abstract Grasshopper component for constructing entity diffs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️diffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/DiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️serializecomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/SerializeComponent)

/ Abstract Grasshopper component for serializing entities to JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️serializecomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/SerializeComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️deserializecomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DeserializeComponent)

/ Abstract Grasshopper component for deserializing entities from JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️deserializecomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/DeserializeComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️serializediffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/SerializeDiffComponent)

/ Abstract Grasshopper component for serializing diffs to JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️serializediffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/SerializeDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️deserializediffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DeserializeDiffComponent)

/ Abstract Grasshopper component for deserializing diffs from JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️deserializediffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/DeserializeDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️serializeidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/SerializeIdComponent)

/ Abstract Grasshopper component for serializing entity IDs to JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️serializeidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/SerializeIdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️deserializeidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/DeserializeIdComponent)

/ Abstract Grasshopper component for deserializing entity IDs from JSON.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️deserializeidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/DeserializeIdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitygoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityGoo)

/ Generic Grasshopper data wrapper with built-in entity validation.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitygoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityParam)

/ Generic Grasshopper parameter with entity validation support.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitycomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityComponent)

/ Abstract Grasshopper component for constructing validated entities.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitycomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityidgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityIdGoo)

/ Generic Grasshopper data wrapper for validated entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityidgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityIdGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityidparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityIdParam)

/ Generic Grasshopper parameter for validated entity ID types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityidparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityIdParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entityidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityIdComponent)

/ Abstract Grasshopper component for constructing validated entity IDs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityidcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityIdComponent)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitydiffgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityDiffGoo)

/ Generic Grasshopper data wrapper for validated entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitydiffgoo](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityDiffGoo)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitydiffparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityDiffParam)

/ Generic Grasshopper parameter for validated entity diff types.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitydiffparam](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityDiffParam)

## [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🛠️entitydiffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/EntityDiffComponent)

/ Abstract Grasshopper component for constructing validated entity diffs.
/ [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitydiffcomponent](semiorepo://definition/semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs/Bases/EntityDiffComponent)

## [👤semio📚gh🛅semiograsshopper💻buildvalueliststs](semiorepo://file/semio/gh/Semio.Grasshopper/build-value-lists.ts)

Generates Grasshopper value list presets from domain data.

## [👤semio📚gh🛅semiograsshopper💻buildvalueliststs🪨builddir](semiorepo://definition/semio/gh/Semio.Grasshopper/build-value-lists.ts/buildDir)

Build output directory for generated value list files.

## [👤semio📚gh🛅semiograsshopper💻buildvalueliststs🛠️convertcsvtovaluelist](semiorepo://definition/semio/gh/Semio.Grasshopper/build-value-lists.ts/convertCsvToValueList)

Converts a CSV file into a Grasshopper value list text format.

## [👤semio📚gh🛅semiograsshopper💻buildts](semiorepo://file/semio/gh/Semio.Grasshopper/build.ts)

Build script for the Grasshopper plugin assembly.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨cwd](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/cwd)

Grasshopper build working directory.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨msbuild](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/msbuild)

MSBuild executable path for Visual Studio 2022.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨yakdistfolder](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/yakDistFolder)

Yak distribution output folder path.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨binfolder](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/binFolder)

Debug build output folder containing compiled binaries.

## [👤semio📚gh🛅semiograsshopper💻buildts🪨files](semiorepo://definition/semio/gh/Semio.Grasshopper/build.ts/files)

List of all files in the build output folder.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts](semiorepo://file/semio/gh/Semio.Grasshopper/yak/build.ts)

Build script for Yak package distribution of the Grasshopper plugin.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🪨cwd](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/build.ts/cwd)

Yak build working directory.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🪨distdir](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/build.ts/distDir)

Distribution directory for the Yak package output.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻buildts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/build.ts/yak)

Yak CLI executable path for Rhino 8.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻logints](semiorepo://file/semio/gh/Semio.Grasshopper/yak/login.ts)

Authenticates with the Yak package server for plugin publishing.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻logints🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/login.ts/yak)

Yak CLI executable path for Rhino 8.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts](semiorepo://file/semio/gh/Semio.Grasshopper/yak/publish.ts)

Publishes the Grasshopper plugin package to the Yak server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨cwd](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/cwd)

Distribution directory containing the built Yak package.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨manifestcontent](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/manifestContent)

Manifest content read from the distribution folder.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨versionmatch](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/versionMatch)

Version regex match result from the manifest.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨version](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/version)

Extracted version string from the manifest.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨buildname](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/buildName)

Yak package filename following the naming convention.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻publishts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/publish.ts/yak)

Yak CLI executable path for Rhino 8.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts](semiorepo://file/semio/gh/Semio.Grasshopper/yak/test-push.ts)

Tests the Yak package push workflow for the Grasshopper plugin.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/test-push.ts/yak)

Yak CLI executable path for Rhino 8.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testpushts🪨packagefile](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/test-push.ts/packageFile)

Yak package filename from CLI argument or default.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearchts](semiorepo://file/semio/gh/Semio.Grasshopper/yak/test-search.ts)

Tests Yak package search functionality for the Grasshopper plugin.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearchts🔖script](semiorepo://section/Script)

Test script for searching the Yak package manager test server.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻testsearchts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/test-search.ts/yak)

Path to the Yak package manager executable.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts](semiorepo://file/semio/gh/Semio.Grasshopper/yak/unyank.ts)

Restores a previously yanked version of the Grasshopper Yak package.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/unyank.ts/yak)

Yak CLI executable path for Rhino 7.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻unyankts🪨version](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/unyank.ts/version)

Semio package version from CLI argument or default.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yankts](semiorepo://file/semio/gh/Semio.Grasshopper/yak/yank.ts)

Yanks a specific version of the Grasshopper Yak package from the registry.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yankts🪨yak](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/yank.ts/yak)

Yak CLI executable path for Rhino 7.

## [👤semio📚gh🛅semiograsshopper🗃️yak💻yankts🪨version](semiorepo://definition/semio/gh/Semio.Grasshopper/yak/yank.ts/version)

Semio package version from CLI argument or default.

## [👤semio📚go💻kitsqlitego](semiorepo://file/semio/go/kit_sqlite.go)

SQLite-backed persistence layer for kit import and export operations.

## [👤semio📚go💻kitsqlitego🛠️kitfromsqlite](semiorepo://definition/semio/go/kit_sqlite.go/KitFromSqlite)

KitFromSqlite reads a Kit from a SQLite database file

## [👤semio📚go💻kitsqlitego🛠️loadtypes](semiorepo://definition/semio/go/kit_sqlite.go/loadTypes)

loadTypes loads all types belonging to a kit from the database

## [👤semio📚go💻kitsqlitego🛠️loaddesigns](semiorepo://definition/semio/go/kit_sqlite.go/loadDesigns)

loadDesigns loads all designs belonging to a kit from the database

## [👤semio📚go💻kitsqlitego🛠️loadpieces](semiorepo://definition/semio/go/kit_sqlite.go/loadPieces)

loadPieces loads all pieces belonging to a design from the database

## [👤semio📚go💻kitsqlitego🛠️loadconnections](semiorepo://definition/semio/go/kit_sqlite.go/loadConnections)

loadConnections loads all connections belonging to a design from the database

## [👤semio📚go💻kitsqlitego🛠️loadconnectors](semiorepo://definition/semio/go/kit_sqlite.go/loadConnectors)

loadConnectors loads all connectors belonging to a type from the database

## [👤semio📚go💻kitsqlitego🛠️kittosqlite](semiorepo://definition/semio/go/kit_sqlite.go/KitToSqlite)

KitToSqlite writes a Kit to a SQLite database file

## [👤semio📚go💻kitsqlitego🛠️kitfromzip](semiorepo://definition/semio/go/kit_sqlite.go/KitFromZip)

KitFromZip extracts a Kit and its files from a zip archive

## [👤semio📚go💻kitsqlitego🛠️kittozip](semiorepo://definition/semio/go/kit_sqlite.go/KitToZip)

KitToZip packages a Kit and its files into a zip archive

## [👤semio📚go💻semiogo](semiorepo://file/semio/go/semio.go)

Core domain library in Go implementing the semio data model and operations.

## [👤semio📚go💻semiogo🔖utils](semiorepo://section/Utils)

Guid generates a new random 128-bit hex-encoded unique identifier.

## [👤semio📚go💻semiogo🔖entityids](semiorepo://section/Entity%20IDs)

AttributeId identifies an attribute entity by GUID.

## [👤semio📚go💻semiogo🔖weakentities](semiorepo://section/Weak%20Entities)

Coord represents a 2D coordinate with U and V components.

## [👤semio📚go💻semiogo🔖attribute](semiorepo://section/Attribute)

Attribute represents a key-value metadata entry with optional definition.

## [👤semio📚go💻semiogo🔖location](semiorepo://section/Location)

Location represents a geographic location with longitude, latitude and optional altitude.

## [👤semio📚go💻semiogo🔖author](semiorepo://section/Author)

Author represents a named authorship entity with optional email.

## [👤semio📚go💻semiogo🔖file](semiorepo://section/File)

File represents a file reference entity with name, remote URL and metadata.

## [👤semio📚go💻semiogo🔖folder](semiorepo://section/Folder)

Folder represents a folder hierarchy entity with name and parent reference.

## [👤semio📚go💻semiogo🔖benchmark](semiorepo://section/Benchmark)

Benchmark represents a named metric threshold with min and max bounds.

## [👤semio📚go💻semiogo🔖quality](semiorepo://section/Quality)

QualityKind is a bitfield enum for quality scope classification.

## [👤semio📚go💻semiogo🔖port](semiorepo://section/Port)

Port represents a named connector port with compatible port references.

## [👤semio📚go💻semiogo🔖prop](semiorepo://section/Prop)

Prop represents a quality property value with optional unit.

## [👤semio📚go💻semiogo🔖tag](semiorepo://section/Tag)

Tag represents a named classification tag with optional description and icon.

## [👤semio📚go💻semiogo🔖concept](semiorepo://section/Concept)

Concept represents a named categorization concept with optional description.

## [👤semio📚go💻semiogo🔖model](semiorepo://section/Model)

Model represents a 3D model reference associated with a file and tags.

## [👤semio📚go💻semiogo🔖connector](semiorepo://section/Connector)

Connector represents a spatial connection point on a type with position and direction.

## [👤semio📚go💻semiogo🔖type](semiorepo://section/Type)

Type represents a component type with models, connectors and hierarchical inheritance.

## [👤semio📚go💻semiogo🔖layer](semiorepo://section/Layer)

Layer represents a named layer with visibility, lock and color properties.

## [👤semio📚go💻semiogo🔖piece](semiorepo://section/Piece)

Piece represents a placed component instance within a design.

## [👤semio📚go💻semiogo🔖group](semiorepo://section/Group)

Group represents a named collection of pieces within a design.

## [👤semio📚go💻semiogo🔖side](semiorepo://section/Side)

Side represents one end of a connection referencing a piece and optional connector.

## [👤semio📚go💻semiogo🔖connection](semiorepo://section/Connection)

Connection represents a spatial relationship between two pieces with transform parameters.

## [👤semio📚go💻semiogo🔖stat](semiorepo://section/Stat)

Stat represents a statistical quality measurement with min and max bounds.

## [👤semio📚go💻semiogo🔖design](semiorepo://section/Design)

Design represents an assembly of pieces, connections, layers and groups.

## [👤semio📚go💻semiogo🔖kit](semiorepo://section/Kit)

Kit represents the root container for all domain entities.

## [👤semio📚go💻semiogo🔖serialization](semiorepo://section/Serialization)

SerializeKit marshals a kit to indented JSON bytes.

## [👤semio📚go💻semiogo🔖helpers](semiorepo://section/Helpers)

FindTypeInKit returns a pointer to the type with the given GUID or nil.

## [👤semio📚go💻semiogo🔖factories](semiorepo://section/Factories)

NewKit creates a new kit with the given name and a generated GUID.

## [👤semio📚go💻semiogo🔖kitoperations](semiorepo://section/Kit%20Operations)

AreKitsEqual compares two kits for structural equality.

## [👤semio📚go💻semiogo🔖kitdiffhelpers](semiorepo://section/Kit%20Diff%20Helpers)

AddTypeToKit creates a diff that adds a single type to a kit.

## [👤semio📚go💻semiogo🔖validation](semiorepo://section/Validation)

SemioEntityKind enumerates the kinds of semio domain entities.

## [👤semio📚go💻semiogo🔖validationserialization](semiorepo://section/Validation%20Serialization)

ProblemSerialized is the JSON-serializable representation of a validation problem.

## [👤semio📚go💻semiogo🪨assetspath](semiorepo://definition/semio/go/semio.go/AssetsPath)

AssetsPath holds the data fields for a AssetsPath record.

## [👤semio📚go💻semiogo🛠️guid](semiorepo://definition/semio/go/semio.go/Guid)

Guid generates a new random 128-bit hex-encoded unique identifier.

## [👤semio📚go💻semiogo🛠️normalize](semiorepo://definition/semio/go/semio.go/Normalize)

Normalize converts a string to lowercase trimmed form.

## [👤semio📚go💻semiogo🛠️round](semiorepo://definition/semio/go/semio.go/Round)

Round rounds a float64 to the specified number of decimal places.

## [👤semio📚go💻semiogo🛠️deepequal](semiorepo://definition/semio/go/semio.go/DeepEqual)

DeepEqual compares two values for deep equality via JSON serialization.

## [👤semio📚go💻semiogo✂️attributeid](semiorepo://definition/semio/go/semio.go/AttributeId)

AttributeId identifies an attribute entity by GUID.

## [👤semio📚go💻semiogo✂️locationid](semiorepo://definition/semio/go/semio.go/LocationId)

LocationId identifies a location entity by GUID.

## [👤semio📚go💻semiogo✂️authorid](semiorepo://definition/semio/go/semio.go/AuthorId)

AuthorId identifies an author entity by GUID.

## [👤semio📚go💻semiogo✂️fileid](semiorepo://definition/semio/go/semio.go/FileId)

FileId identifies a file entity by GUID.

## [👤semio📚go💻semiogo✂️folderid](semiorepo://definition/semio/go/semio.go/FolderId)

FolderId identifies a folder entity by GUID.

## [👤semio📚go💻semiogo✂️benchmarkid](semiorepo://definition/semio/go/semio.go/BenchmarkId)

BenchmarkId identifies a benchmark entity by GUID.

## [👤semio📚go💻semiogo✂️qualityid](semiorepo://definition/semio/go/semio.go/QualityId)

QualityId identifies a quality entity by GUID.

## [👤semio📚go💻semiogo✂️portid](semiorepo://definition/semio/go/semio.go/PortId)

PortId identifies a port entity by GUID.

## [👤semio📚go💻semiogo✂️propid](semiorepo://definition/semio/go/semio.go/PropId)

PropId identifies a prop entity by GUID.

## [👤semio📚go💻semiogo✂️tagid](semiorepo://definition/semio/go/semio.go/TagId)

TagId identifies a tag entity by GUID.

## [👤semio📚go💻semiogo✂️conceptid](semiorepo://definition/semio/go/semio.go/ConceptId)

ConceptId identifies a concept entity by GUID.

## [👤semio📚go💻semiogo✂️modelid](semiorepo://definition/semio/go/semio.go/ModelId)

ModelId identifies a model entity by GUID.

## [👤semio📚go💻semiogo✂️connectorid](semiorepo://definition/semio/go/semio.go/ConnectorId)

ConnectorId identifies a connector entity by GUID.

## [👤semio📚go💻semiogo✂️typeid](semiorepo://definition/semio/go/semio.go/TypeId)

TypeId identifies a type entity by GUID.

## [👤semio📚go💻semiogo✂️layerid](semiorepo://definition/semio/go/semio.go/LayerId)

LayerId identifies a layer entity by GUID.

## [👤semio📚go💻semiogo✂️pieceid](semiorepo://definition/semio/go/semio.go/PieceId)

PieceId identifies a piece entity by GUID.

## [👤semio📚go💻semiogo✂️groupid](semiorepo://definition/semio/go/semio.go/GroupId)

GroupId identifies a group entity by GUID.

## [👤semio📚go💻semiogo✂️sideid](semiorepo://definition/semio/go/semio.go/SideId)

SideId identifies a connection side by piece, design piece and connector references.

## [👤semio📚go💻semiogo✂️connectionid](semiorepo://definition/semio/go/semio.go/ConnectionId)

ConnectionId identifies a connection entity by GUID.

## [👤semio📚go💻semiogo✂️statid](semiorepo://definition/semio/go/semio.go/StatId)

StatId identifies a stat entity by GUID.

## [👤semio📚go💻semiogo✂️designid](semiorepo://definition/semio/go/semio.go/DesignId)

DesignId identifies a design entity by GUID.

## [👤semio📚go💻semiogo✂️kitid](semiorepo://definition/semio/go/semio.go/KitId)

KitId identifies a kit entity by GUID.

## [👤semio📚go💻semiogo✂️coord](semiorepo://definition/semio/go/semio.go/Coord)

Coord represents a 2D coordinate with U and V components.

## [👤semio📚go💻semiogo✂️vec](semiorepo://definition/semio/go/semio.go/Vec)

Vec represents a 2D vector with U and V components.

## [👤semio📚go💻semiogo✂️point](semiorepo://definition/semio/go/semio.go/Point)

Point represents a 3D point with X, Y and Z components.

## [👤semio📚go💻semiogo✂️vector](semiorepo://definition/semio/go/semio.go/Vector)

Vector represents a 3D vector with X, Y and Z components.

## [👤semio📚go💻semiogo✂️plane](semiorepo://definition/semio/go/semio.go/Plane)

Plane represents a 3D plane defined by origin, X-axis and Y-axis.

## [👤semio📚go💻semiogo✂️camera](semiorepo://definition/semio/go/semio.go/Camera)

Camera represents a 3D camera with position, forward and up vectors.

## [👤semio📚go💻semiogo✂️attribute](semiorepo://definition/semio/go/semio.go/Attribute)

Attribute represents a key-value metadata entry with optional definition.

## [👤semio📚go💻semiogo✂️attributediff](semiorepo://definition/semio/go/semio.go/AttributeDiff)

AttributeDiff represents changes to an attribute entity.

## [👤semio📚go💻semiogo✂️attributesdiff](semiorepo://definition/semio/go/semio.go/AttributesDiff)

AttributesDiff represents a collection of attribute additions, removals and updates.

## [👤semio📚go💻semiogo✂️location](semiorepo://definition/semio/go/semio.go/Location)

Location represents a geographic location with longitude, latitude and optional altitude.

## [👤semio📚go💻semiogo✂️locationdiff](semiorepo://definition/semio/go/semio.go/LocationDiff)

LocationDiff represents changes to a location entity.

## [👤semio📚go💻semiogo✂️author](semiorepo://definition/semio/go/semio.go/Author)

Author represents a named authorship entity with optional email.

## [👤semio📚go💻semiogo✂️authordiff](semiorepo://definition/semio/go/semio.go/AuthorDiff)

AuthorDiff represents changes to an author entity.

## [👤semio📚go💻semiogo✂️authorsdiff](semiorepo://definition/semio/go/semio.go/AuthorsDiff)

AuthorsDiff represents a collection of author additions, removals and updates.

## [👤semio📚go💻semiogo✂️file](semiorepo://definition/semio/go/semio.go/File)

File represents a file reference entity with name, remote URL and metadata.

## [👤semio📚go💻semiogo✂️filediff](semiorepo://definition/semio/go/semio.go/FileDiff)

FileDiff represents changes to a file entity.

## [👤semio📚go💻semiogo✂️filesdiff](semiorepo://definition/semio/go/semio.go/FilesDiff)

FilesDiff represents a collection of file additions, removals and updates.

## [👤semio📚go💻semiogo✂️folder](semiorepo://definition/semio/go/semio.go/Folder)

Folder represents a folder hierarchy entity with name and parent reference.

## [👤semio📚go💻semiogo✂️folderdiff](semiorepo://definition/semio/go/semio.go/FolderDiff)

FolderDiff represents changes to a folder entity.

## [👤semio📚go💻semiogo✂️foldersdiff](semiorepo://definition/semio/go/semio.go/FoldersDiff)

FoldersDiff represents a collection of folder additions, removals and updates.

## [👤semio📚go💻semiogo✂️benchmark](semiorepo://definition/semio/go/semio.go/Benchmark)

Benchmark represents a named metric threshold with min and max bounds.

## [👤semio📚go💻semiogo✂️benchmarkdiff](semiorepo://definition/semio/go/semio.go/BenchmarkDiff)

BenchmarkDiff represents changes to a benchmark entity.

## [👤semio📚go💻semiogo✂️benchmarksdiff](semiorepo://definition/semio/go/semio.go/BenchmarksDiff)

BenchmarksDiff represents a collection of benchmark additions, removals and updates.

## [👤semio📚go💻semiogo✂️qualitykind](semiorepo://definition/semio/go/semio.go/QualityKind)

QualityKind is a bitfield enum for quality scope classification.

## [👤semio📚go💻semiogo✂️quality](semiorepo://definition/semio/go/semio.go/Quality)

Quality represents a measurable property with formula, units and benchmarks.

## [👤semio📚go💻semiogo✂️qualitydiff](semiorepo://definition/semio/go/semio.go/QualityDiff)

QualityDiff represents changes to a quality entity.

## [👤semio📚go💻semiogo✂️qualitiesdiff](semiorepo://definition/semio/go/semio.go/QualitiesDiff)

QualitiesDiff represents a collection of quality additions, removals and updates.

## [👤semio📚go💻semiogo✂️port](semiorepo://definition/semio/go/semio.go/Port)

Port represents a named connector port with compatible port references.

## [👤semio📚go💻semiogo✂️portdiff](semiorepo://definition/semio/go/semio.go/PortDiff)

PortDiff represents changes to a port entity.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semiogo✂️portsdiff](semiorepo://definition/semio/go/semio.go/PortsDiff)

PortsDiff represents a collection of port additions, removals and updates.

## [👤semio📚go💻semiogo✂️prop](semiorepo://definition/semio/go/semio.go/Prop)

Prop represents a quality property value with optional unit.

## [👤semio📚go💻semiogo✂️propdiff](semiorepo://definition/semio/go/semio.go/PropDiff)

PropDiff represents changes to a prop entity.

## [👤semio📚go💻semiogo✂️propsdiff](semiorepo://definition/semio/go/semio.go/PropsDiff)

PropsDiff represents a collection of prop additions, removals and updates.

## [👤semio📚go💻semiogo✂️tag](semiorepo://definition/semio/go/semio.go/Tag)

Tag represents a named classification tag with optional description and icon.

## [👤semio📚go💻semiogo✂️tagdiff](semiorepo://definition/semio/go/semio.go/TagDiff)

TagDiff represents changes to a tag entity.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semiogo✂️tagsdiff](semiorepo://definition/semio/go/semio.go/TagsDiff)

TagsDiff represents a collection of tag additions, removals and updates.

## [👤semio📚go💻semiogo✂️concept](semiorepo://definition/semio/go/semio.go/Concept)

Concept represents a named categorization concept with optional description.

## [👤semio📚go💻semiogo✂️conceptdiff](semiorepo://definition/semio/go/semio.go/ConceptDiff)

ConceptDiff represents changes to a concept entity.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semiogo✂️conceptsdiff](semiorepo://definition/semio/go/semio.go/ConceptsDiff)

ConceptsDiff represents a collection of concept additions, removals and updates.

## [👤semio📚go💻semiogo✂️model](semiorepo://definition/semio/go/semio.go/Model)

Model represents a 3D model reference associated with a file and tags.

## [👤semio📚go💻semiogo✂️modeldiff](semiorepo://definition/semio/go/semio.go/ModelDiff)

ModelDiff represents changes to a model entity.

## [👤semio📚go💻semiogo✂️modelsdiff](semiorepo://definition/semio/go/semio.go/ModelsDiff)

ModelsDiff represents a collection of model additions, removals and updates.

## [👤semio📚go💻semiogo✂️connector](semiorepo://definition/semio/go/semio.go/Connector)

Connector represents a spatial connection point on a type with position and direction.

## [👤semio📚go💻semiogo✂️pointdiff](semiorepo://definition/semio/go/semio.go/PointDiff)

PointDiff represents changes to a 3D point.

## [👤semio📚go💻semiogo✂️vectordiff](semiorepo://definition/semio/go/semio.go/VectorDiff)

VectorDiff represents changes to a 3D vector.

## [👤semio📚go💻semiogo✂️connectordiff](semiorepo://definition/semio/go/semio.go/ConnectorDiff)

ConnectorDiff represents changes to a connector entity.

## [👤semio📚go💻semiogo✂️connectorsdiff](semiorepo://definition/semio/go/semio.go/ConnectorsDiff)

ConnectorsDiff represents a collection of connector additions, removals and updates.

## [👤semio📚go💻semiogo🛠️type](semiorepo://definition/semio/go/semio.go/Type)

Type represents a component type with models, connectors and hierarchical inheritance.

## [👤semio📚go💻semiogo✂️typediff](semiorepo://definition/semio/go/semio.go/TypeDiff)

TypeDiff represents changes to a type entity.

## [👤semio📚go💻semiogo🛠️unmarshaljson](semiorepo://definition/semio/go/semio.go/UnmarshalJSON)

UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.

## [👤semio📚go💻semiogo🛠️hasfield](semiorepo://definition/semio/go/semio.go/HasField)

HasField returns whether a JSON field was present in the unmarshaled data.

## [👤semio📚go💻semiogo✂️typesdiff](semiorepo://definition/semio/go/semio.go/TypesDiff)

TypesDiff represents a collection of type additions, removals and updates.

## [👤semio📚go💻semiogo✂️layer](semiorepo://definition/semio/go/semio.go/Layer)

Layer represents a named layer with visibility, lock and color properties.

## [👤semio📚go💻semiogo✂️layerdiff](semiorepo://definition/semio/go/semio.go/LayerDiff)

LayerDiff represents changes to a layer entity.

## [👤semio📚go💻semiogo✂️layersdiff](semiorepo://definition/semio/go/semio.go/LayersDiff)

LayersDiff represents a collection of layer additions, removals and updates.

## [👤semio📚go💻semiogo✂️piece](semiorepo://definition/semio/go/semio.go/Piece)

Piece represents a placed component instance within a design.

## [👤semio📚go💻semiogo✂️coorddiff](semiorepo://definition/semio/go/semio.go/CoordDiff)

CoordDiff represents changes to a 2D coordinate.

## [👤semio📚go💻semiogo✂️planediff](semiorepo://definition/semio/go/semio.go/PlaneDiff)

PlaneDiff represents changes to a 3D plane.

## [👤semio📚go💻semiogo✂️piecediff](semiorepo://definition/semio/go/semio.go/PieceDiff)

PieceDiff represents changes to a piece entity.

## [👤semio📚go💻semiogo✂️piecesdiff](semiorepo://definition/semio/go/semio.go/PiecesDiff)

PiecesDiff represents a collection of piece additions, removals and updates.

## [👤semio📚go💻semiogo✂️group](semiorepo://definition/semio/go/semio.go/Group)

Group represents a named collection of pieces within a design.

## [👤semio📚go💻semiogo✂️groupdiff](semiorepo://definition/semio/go/semio.go/GroupDiff)

GroupDiff represents changes to a group entity.

## [👤semio📚go💻semiogo✂️groupsdiff](semiorepo://definition/semio/go/semio.go/GroupsDiff)

GroupsDiff represents a collection of group additions, removals and updates.

## [👤semio📚go💻semiogo✂️side](semiorepo://definition/semio/go/semio.go/Side)

Side represents one end of a connection referencing a piece and optional connector.

## [👤semio📚go💻semiogo✂️sidediff](semiorepo://definition/semio/go/semio.go/SideDiff)

SideDiff represents changes to a connection side.

## [👤semio📚go💻semiogo✂️connection](semiorepo://definition/semio/go/semio.go/Connection)

Connection represents a spatial relationship between two pieces with transform parameters.

## [👤semio📚go💻semiogo✂️connectiondiff](semiorepo://definition/semio/go/semio.go/ConnectionDiff)

ConnectionDiff represents changes to a connection entity.

## [👤semio📚go💻semiogo✂️connectionsdiff](semiorepo://definition/semio/go/semio.go/ConnectionsDiff)

ConnectionsDiff represents a collection of connection additions, removals and updates.

## [👤semio📚go💻semiogo✂️stat](semiorepo://definition/semio/go/semio.go/Stat)

Stat represents a statistical quality measurement with min and max bounds.

## [👤semio📚go💻semiogo✂️statdiff](semiorepo://definition/semio/go/semio.go/StatDiff)

StatDiff represents changes to a stat entity.

## [👤semio📚go💻semiogo✂️statsdiff](semiorepo://definition/semio/go/semio.go/StatsDiff)

StatsDiff represents a collection of stat additions, removals and updates.

## [👤semio📚go💻semiogo✂️design](semiorepo://definition/semio/go/semio.go/Design)

Design represents an assembly of pieces, connections, layers and groups.

## [👤semio📚go💻semiogo✂️cameradiff](semiorepo://definition/semio/go/semio.go/CameraDiff)

CameraDiff represents changes to a camera view.

## [👤semio📚go💻semiogo✂️designdiff](semiorepo://definition/semio/go/semio.go/DesignDiff)

DesignDiff represents changes to a design entity.

## [👤semio📚go💻semiogo✂️designsdiff](semiorepo://definition/semio/go/semio.go/DesignsDiff)

DesignsDiff represents a collection of design additions, removals and updates.

## [👤semio📚go💻semiogo✂️kit](semiorepo://definition/semio/go/semio.go/Kit)

Kit represents the root container for all domain entities.

## [👤semio📚go💻semiogo✂️kitdiff](semiorepo://definition/semio/go/semio.go/KitDiff)

KitDiff represents changes to a kit entity.

## [👤semio📚go💻semiogo✂️kitsdiff](semiorepo://definition/semio/go/semio.go/KitsDiff)

KitsDiff represents a collection of kit additions, removals and updates.

## [👤semio📚go💻semiogo🛠️serializekit](semiorepo://definition/semio/go/semio.go/SerializeKit)

SerializeKit marshals a kit to indented JSON bytes.

## [👤semio📚go💻semiogo🛠️deserializekit](semiorepo://definition/semio/go/semio.go/DeserializeKit)

DeserializeKit unmarshals JSON bytes into a kit.

## [👤semio📚go💻semiogo🛠️serializekitdiff](semiorepo://definition/semio/go/semio.go/SerializeKitDiff)

SerializeKitDiff marshals a kit diff to indented JSON bytes.

## [👤semio📚go💻semiogo🛠️deserializekitdiff](semiorepo://definition/semio/go/semio.go/DeserializeKitDiff)

DeserializeKitDiff unmarshals JSON bytes into a kit diff.

## [👤semio📚go💻semiogo🛠️findtypeinkit](semiorepo://definition/semio/go/semio.go/FindTypeInKit)

FindTypeInKit returns a pointer to the type with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️finddesigninkit](semiorepo://definition/semio/go/semio.go/FindDesignInKit)

FindDesignInKit returns a pointer to the design with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findpieceindesign](semiorepo://definition/semio/go/semio.go/FindPieceInDesign)

FindPieceInDesign returns a pointer to the piece with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findconnectionindesign](semiorepo://definition/semio/go/semio.go/FindConnectionInDesign)

FindConnectionInDesign returns a pointer to the connection with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findconnectorintype](semiorepo://definition/semio/go/semio.go/FindConnectorInType)

FindConnectorInType returns a pointer to the connector with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findfileinkit](semiorepo://definition/semio/go/semio.go/FindFileInKit)

FindFileInKit returns a pointer to the file with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findfolderinkit](semiorepo://definition/semio/go/semio.go/FindFolderInKit)

FindFolderInKit returns a pointer to the folder with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findqualityinkit](semiorepo://definition/semio/go/semio.go/FindQualityInKit)

FindQualityInKit returns a pointer to the quality with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findportinkit](semiorepo://definition/semio/go/semio.go/FindPortInKit)

FindPortInKit returns a pointer to the port with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findtaginkit](semiorepo://definition/semio/go/semio.go/FindTagInKit)

FindTagInKit returns a pointer to the tag with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findconceptinkit](semiorepo://definition/semio/go/semio.go/FindConceptInKit)

FindConceptInKit returns a pointer to the concept with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️findauthorinkit](semiorepo://definition/semio/go/semio.go/FindAuthorInKit)

FindAuthorInKit returns a pointer to the author with the given GUID or nil.

## [👤semio📚go💻semiogo🛠️newkit](semiorepo://definition/semio/go/semio.go/NewKit)

NewKit creates a new kit with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newtype](semiorepo://definition/semio/go/semio.go/NewType)

NewType creates a new type with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newdesign](semiorepo://definition/semio/go/semio.go/NewDesign)

NewDesign creates a new design with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newpiece](semiorepo://definition/semio/go/semio.go/NewPiece)

NewPiece creates a new piece with a generated GUID.

## [👤semio📚go💻semiogo🛠️newconnection](semiorepo://definition/semio/go/semio.go/NewConnection)

NewConnection creates a new connection between two pieces by their GUIDs.

## [👤semio📚go💻semiogo🛠️newconnector](semiorepo://definition/semio/go/semio.go/NewConnector)

NewConnector creates a new connector with position, direction and parameter t.

## [👤semio📚go💻semiogo🛠️newfile](semiorepo://definition/semio/go/semio.go/NewFile)

NewFile creates a new file with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newfolder](semiorepo://definition/semio/go/semio.go/NewFolder)

NewFolder creates a new folder with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newquality](semiorepo://definition/semio/go/semio.go/NewQuality)

NewQuality creates a new quality with the given key, name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newport](semiorepo://definition/semio/go/semio.go/NewPort)

NewPort creates a new port with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newtag](semiorepo://definition/semio/go/semio.go/NewTag)

NewTag creates a new tag with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newconcept](semiorepo://definition/semio/go/semio.go/NewConcept)

NewConcept creates a new concept with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️newauthor](semiorepo://definition/semio/go/semio.go/NewAuthor)

NewAuthor creates a new author with the given name and a generated GUID.

## [👤semio📚go💻semiogo🛠️arekitsequal](semiorepo://definition/semio/go/semio.go/AreKitsEqual)

AreKitsEqual compares two kits for structural equality.

## [👤semio📚go💻semiogo🛠️arekitdiffsequal](semiorepo://definition/semio/go/semio.go/AreKitDiffsEqual)

AreKitDiffsEqual compares two kit diffs for structural equality.

## [👤semio📚go💻semiogo🛠️getkitdiff](semiorepo://definition/semio/go/semio.go/GetKitDiff)

GetKitDiff computes the diff between a before and after kit state.

## [👤semio📚go💻semiogo🛠️inversekitdiff](semiorepo://definition/semio/go/semio.go/InverseKitDiff)

InverseKitDiff computes the reverse diff that undoes an applied diff.

## [👤semio📚go💻semiogo🛠️applykitdiff](semiorepo://definition/semio/go/semio.go/ApplyKitDiff)

ApplyKitDiff applies a diff to a base kit producing the updated kit.

## [👤semio📚go💻semiogo🛠️filterdesignswithoutparent](semiorepo://definition/semio/go/semio.go/FilterDesignsWithoutParent)

FilterDesignsWithoutParent returns only root-level designs with no parent.

## [👤semio📚go💻semiogo🛠️addtypetokit](semiorepo://definition/semio/go/semio.go/AddTypeToKit)

AddTypeToKit creates a diff that adds a single type to a kit.

## [👤semio📚go💻semiogo🛠️removetypefromkit](semiorepo://definition/semio/go/semio.go/RemoveTypeFromKit)

RemoveTypeFromKit creates a diff that removes a type by GUID.

## [👤semio📚go💻semiogo🛠️adddesigntokit](semiorepo://definition/semio/go/semio.go/AddDesignToKit)

AddDesignToKit creates a diff that adds a single design to a kit.

## [👤semio📚go💻semiogo🛠️removedesignfromkit](semiorepo://definition/semio/go/semio.go/RemoveDesignFromKit)

RemoveDesignFromKit creates a diff that removes a design by GUID.

## [👤semio📚go💻semiogo🛠️addfiletokit](semiorepo://definition/semio/go/semio.go/AddFileToKit)

AddFileToKit creates a diff that adds a single file to a kit.

## [👤semio📚go💻semiogo🛠️removefilefromkit](semiorepo://definition/semio/go/semio.go/RemoveFileFromKit)

RemoveFileFromKit creates a diff that removes a file by GUID.

## [👤semio📚go💻semiogo🛠️addporttokit](semiorepo://definition/semio/go/semio.go/AddPortToKit)

AddPortToKit creates a diff that adds a single port to a kit.

## [👤semio📚go💻semiogo🛠️removeportfromkit](semiorepo://definition/semio/go/semio.go/RemovePortFromKit)

RemovePortFromKit creates a diff that removes a port by GUID.

## [👤semio📚go💻semiogo🛠️addtagtokit](semiorepo://definition/semio/go/semio.go/AddTagToKit)

AddTagToKit creates a diff that adds a single tag to a kit.

## [👤semio📚go💻semiogo🛠️removetagfromkit](semiorepo://definition/semio/go/semio.go/RemoveTagFromKit)

RemoveTagFromKit creates a diff that removes a tag by GUID.

## [👤semio📚go💻semiogo🛠️addconcepttokit](semiorepo://definition/semio/go/semio.go/AddConceptToKit)

AddConceptToKit creates a diff that adds a single concept to a kit.

## [👤semio📚go💻semiogo🛠️removeconceptfromkit](semiorepo://definition/semio/go/semio.go/RemoveConceptFromKit)

RemoveConceptFromKit creates a diff that removes a concept by GUID.

## [👤semio📚go💻semiogo✂️semioentitykind](semiorepo://definition/semio/go/semio.go/SemioEntityKind)

SemioEntityKind enumerates the kinds of semio domain entities.

## [👤semio📚go💻semiogo✂️severity](semiorepo://definition/semio/go/semio.go/Severity)

Severity enumerates validation problem severity levels.

## [👤semio📚go💻semiogo✂️domainlocation](semiorepo://definition/semio/go/semio.go/DomainLocation)

DomainLocation identifies the entity and field where a validation problem occurs.

## [👤semio📚go💻semiogo✂️fix](semiorepo://definition/semio/go/semio.go/Fix)

Fix represents a suggested correction for a validation problem.

## [👤semio📚go💻semiogo✂️problem](semiorepo://definition/semio/go/semio.go/Problem)

Problem represents a single validation constraint breach.

## [👤semio📚go💻semiogo✂️validationresult](semiorepo://definition/semio/go/semio.go/ValidationResult)

ValidationResult contains all problems found during kit validation.

## [👤semio📚go💻semiogo✂️validationcontext](semiorepo://definition/semio/go/semio.go/ValidationContext)

ValidationContext provides indexed access to kit entities for constraint evaluation.

## [👤semio📚go💻semiogo✂️constraint](semiorepo://definition/semio/go/semio.go/Constraint)

Constraint is a function that evaluates a validation rule against a kit context.

## [👤semio📚go💻semiogo🛠️guiduniquenessconstraint](semiorepo://definition/semio/go/semio.go/GuidUniquenessConstraint)

GuidUniquenessConstraint checks that all entity GUIDs are unique within a kit.

## [👤semio📚go💻semiogo🛠️typenameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/TypeNameUniquenessConstraint)

TypeNameUniquenessConstraint checks that sibling type names are unique.

## [👤semio📚go💻semiogo🛠️designnameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/DesignNameUniquenessConstraint)

DesignNameUniquenessConstraint checks that sibling design names are unique.

## [👤semio📚go💻semiogo🛠️piecenameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/PieceNameUniquenessConstraint)

PieceNameUniquenessConstraint checks that piece names are unique within each design.

## [👤semio📚go💻semiogo🛠️qualitynameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/QualityNameUniquenessConstraint)

QualityNameUniquenessConstraint checks that quality names are unique within a kit.

## [👤semio📚go💻semiogo🛠️portnameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/PortNameUniquenessConstraint)

PortNameUniquenessConstraint checks that port names are unique within a kit.

## [👤semio📚go💻semiogo🛠️filenameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/FileNameUniquenessConstraint)

FileNameUniquenessConstraint checks that file names are unique within a kit.

## [👤semio📚go💻semiogo🛠️foldernameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/FolderNameUniquenessConstraint)

FolderNameUniquenessConstraint checks that sibling folder names are unique.

## [👤semio📚go💻semiogo🛠️connectornameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/ConnectorNameUniquenessConstraint)

ConnectorNameUniquenessConstraint checks that connector names are unique within each type.

## [👤semio📚go💻semiogo🛠️modelnameuniquenessconstraint](semiorepo://definition/semio/go/semio.go/ModelNameUniquenessConstraint)

ModelNameUniquenessConstraint checks that model names are unique within each type.

## [👤semio📚go💻semiogo🛠️layerpathuniquenessconstraint](semiorepo://definition/semio/go/semio.go/LayerPathUniquenessConstraint)

LayerPathUniquenessConstraint checks that layer paths are unique within each design.

## [👤semio📚go💻semiogo🪨defaultconstraints](semiorepo://definition/semio/go/semio.go/DefaultConstraints)

DefaultConstraints lists all built-in validation constraints.

## [👤semio📚go💻semiogo🛠️validatekit](semiorepo://definition/semio/go/semio.go/ValidateKit)

ValidateKit validates a kit using the default set of constraints.

## [👤semio📚go💻semiogo🛠️validatekitwithconstraints](semiorepo://definition/semio/go/semio.go/ValidateKitWithConstraints)

ValidateKitWithConstraints validates a kit using the provided constraints.

## [👤semio📚go💻semiogo🛠️haserrors](semiorepo://definition/semio/go/semio.go/HasErrors)

HasErrors returns true if the validation result contains any error-severity problems.

## [👤semio📚go💻semiogo✂️problemserialized](semiorepo://definition/semio/go/semio.go/ProblemSerialized)

ProblemSerialized is the JSON-serializable representation of a validation problem.

## [👤semio📚go💻semiogo✂️validationresultserialized](semiorepo://definition/semio/go/semio.go/ValidationResultSerialized)

ValidationResultSerialized is the JSON-serializable representation of a validation result.

## [👤semio📚go💻semiogo🛠️tovalidationresult](semiorepo://definition/semio/go/semio.go/ToValidationResult)

ToValidationResult converts a validation result to its serializable form.

## [👤semio📚go💻semiogo🛠️arevalidationresultsequal](semiorepo://definition/semio/go/semio.go/AreValidationResultsEqual)

AreValidationResultsEqual compares two serialized validation results for equality.

## [👤semio📚go💻semiogo🛠️flattendesign](semiorepo://definition/semio/go/semio.go/FlattenDesign)

FlattenDesign computes absolute planes and centers for all pieces in a design.

## [👤semio📚go💻semiogo🛠️applydesigndiff](semiorepo://definition/semio/go/semio.go/ApplyDesignDiff)

ApplyDesignDiff applies a design diff to a base design.

## [👤semio📚js💻devts](semiorepo://file/semio/js/dev.ts)

Development server entry point for the JavaScript workspace.

## [👤semio📚js💻devts🔖dev](semiorepo://section/Dev)

Spawns parallel sketchpad and storybook dev servers.

## [👤semio📚js💻devts🪨iswindows](semiorepo://definition/semio/js/dev.ts/isWindows)

Whether the current platform is Windows.

## [👤semio📚js💻devts🪨npmcmd](semiorepo://definition/semio/js/dev.ts/npmCmd)

Platform-specific npm command name.

## [👤semio📚js💻devts🪨vite](semiorepo://definition/semio/js/dev.ts/vite)

Spawned sketchpad dev server process.

## [👤semio📚js💻devts🪨storybook](semiorepo://definition/semio/js/dev.ts/storybook)

Spawned storybook dev server process.

## [👤semio📚js💻globaldts](semiorepo://file/semio/js/global.d.ts)

Global type declarations for the JavaScript workspace.

## [👤semio📚js💻i18nts](semiorepo://file/semio/js/i18n.ts)

Internationalization setup and translation utilities for the UI.

## [👤semio📚js💻i18nts🔖i18n](semiorepo://section/I18n)

Initializes i18next with language detection, React bindings and expertise-aware label hooks.

## [👤semio📚js💻i18nts🪨getexpertisefunction](semiorepo://definition/semio/js/i18n.ts/getExpertiseFunction)

Mutable holder for the active expertise provider function.

## [👤semio📚js💻indexts](semiorepo://file/semio/js/index.ts)

Barrel export for the core JavaScript workspace modules.

## [👤semio📚js💻indexts🔖exports](semiorepo://section/Exports)

Public API surface re-exporting sketchpad components, semio domain, and shared configs.

## [👤semio📚js💻semiots](semiorepo://file/semio/js/semio.ts)

Core domain model types, schemas and utilities for the semio platform.

## [👤semio📚js💻sitetsx](semiorepo://file/semio/js/site.tsx)

Landing page and marketing site React component.

## [👤semio📚js💻sitetsx🔖entrypoint](semiorepo://section/Entrypoint)

Site entrypoint that mounts the Sketchpad React component into the DOM.

## [👤semio📚js🗃️sketchpad💻designtsx](semiorepo://file/semio/js/sketchpad/Design.tsx)

Design app providing diagram and scene windows for editing designs.

## [👤semio📚js🗃️sketchpad💻docstsx](semiorepo://file/semio/js/sketchpad/Docs.tsx)

Documentation viewer app with workbench and detail panels.

## [👤semio📚js🗃️sketchpad💻feedbacktsx](semiorepo://file/semio/js/sketchpad/Feedback.tsx)

Feedback collection app with rating hooks and submission forms.

## [👤semio📚js🗃️sketchpad💻hometsx](semiorepo://file/semio/js/sketchpad/Home.tsx)

Home screen app showing recent projects and getting started content.

## [👤semio📚js🗃️sketchpad💻kittsx](semiorepo://file/semio/js/sketchpad/Kit.tsx)

Kit editor app for managing types, designs and qualities.

## [👤semio📚js🗃️sketchpad💻qualitytsx](semiorepo://file/semio/js/sketchpad/Quality.tsx)

Quality inspection app for viewing and editing quality attributes.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx](semiorepo://file/semio/js/sketchpad/Sketchpad.tsx)

Main sketchpad container managing app tabs, panels and window layout.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖imports](semiorepo://section/Imports)

External and internal module imports.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖utilities](semiorepo://section/Utilities)

Utility functions used across sketchpad components.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖store](semiorepo://section/Store)

Reactive stores backed by Yjs for collaborative state management.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖store](semiorepo://section/Store)

Reactive stores backed by Yjs for collaborative state management.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖plainappstorenoyjs](semiorepo://section/Plain%20App%20Store%20(No%20YJS))

Non-YJS application stores using plain in-memory state with transaction support.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖fileprovider](semiorepo://section/File%20Provider)

In-memory file storage provider for temporary or test scenarios.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖memoryfileprovider](semiorepo://section/Memory%20File%20Provider)

In-memory file storage provider for temporary or test scenarios.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖localfileproviderindexeddb](semiorepo://section/Local%20File%20Provider%20(IndexedDB))

Browser-local file storage provider backed by IndexedDB.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖remotefileprovider](semiorepo://section/Remote%20File%20Provider)

Remote file storage provider backed by a REST API.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖compositefileprovider](semiorepo://section/Composite%20File%20Provider)

Composite file storage provider that delegates to multiple underlying providers.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖kits](semiorepo://section/Kits)

Yjs-backed attribute store for kit metadata.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖coord](semiorepo://section/Coord)

Yjs-backed coordinate store managing u/v values.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖vec](semiorepo://section/Vec)

Yjs-backed 3D vector component store managing x/y/z values.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖point](semiorepo://section/Point)

Yjs-backed 3D point store managing x/y/z coordinates.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖vector](semiorepo://section/Vector)

Yjs-backed 3D direction vector store managing x/y/z components.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖plane](semiorepo://section/Plane)

Yjs-backed 3D plane store managing origin point and direction vectors.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖camera](semiorepo://section/Camera)

Yjs-backed camera store managing view target and perspective planes.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖location](semiorepo://section/Location)

Yjs-backed location store managing geographical and licensing metadata.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖author](semiorepo://section/Author)

Yjs-backed author store managing author identity and attributes.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖file](semiorepo://section/File)

Yjs-backed file store managing file metadata and content references.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖folder](semiorepo://section/Folder)

Yjs-backed folder store managing folder hierarchy and file references.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖benchmark](semiorepo://section/Benchmark)

Yjs-backed benchmark store managing performance measurement data.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖quality](semiorepo://section/Quality)

Yjs-backed quality store managing quality criteria definitions.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖prop](semiorepo://section/Prop)

Yjs-backed prop store managing design property definitions.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖model](semiorepo://section/Model)

Yjs-backed model store managing 3D model representations.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖connector](semiorepo://section/Connector)

Yjs-backed connector store managing type connectors and their ports.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖type](semiorepo://section/Type)

Yjs-backed type store managing architectural type definitions and connectors.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖layer](semiorepo://section/Layer)

Yjs-backed layer store managing visibility layers in designs.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖piece](semiorepo://section/Piece)

Yjs-backed piece store managing design piece instances and their transforms.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖group](semiorepo://section/Group)

Yjs-backed group store managing piece grouping within designs.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖side](semiorepo://section/Side)

Side store managing connection endpoints for pieces.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖connection](semiorepo://section/Connection)

Yjs-backed connection store managing piece-to-piece connections.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖stat](semiorepo://section/Stat)

Yjs-backed stat store managing statistical measurement data.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖design](semiorepo://section/Design)

Yjs-backed design store managing complete design layouts with pieces and connections.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖ypathapi](semiorepo://section/YPath%20API)

Path-based observation and subscription API for deep design Yjs map access.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖kit](semiorepo://section/Kit)

Yjs-backed kit store managing the complete kit data structure.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖ypathapi](semiorepo://section/YPath%20API)

Path-based observation and subscription API for deep kit Yjs map access.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖targetedkithooks](semiorepo://section/Targeted%20Kit%20Hooks)

React hooks for accessing specific kit data through scope providers.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖commands](semiorepo://section/Commands)

Kit command definitions for import, export, and sync operations.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖machine](semiorepo://section/Machine)

Type definitions for app state, machine input, and context structures.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖types](semiorepo://section/Types)

Type definitions for app state, machine input, and context structures.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖appstatetypes](semiorepo://section/App%20State%20Types)

State shape interfaces for all application views: home, kit, design, type, quality.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖helpers](semiorepo://section/Helpers)

Helper functions for path migration, default state creation, and store initialization.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖sketchpadmachine](semiorepo://section/Sketchpad%20Machine)

XState state machine definition for the sketchpad application lifecycle.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖sketchpadselectors](semiorepo://section/Sketchpad%20Selectors)

Selector functions for extracting state from the sketchpad machine context.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖factory](semiorepo://section/Factory)

Factory function to instantiate the sketchpad actor.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖legacytypeexports](semiorepo://section/Legacy%20Type%20Exports)

Legacy type exports for backward compatibility with existing consumers.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖actortypes](semiorepo://section/Actor%20Types)

Type aliases for the sketchpad XState actor references and snapshots.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖apps](semiorepo://section/Apps)

App-specific hooks for design, type, kit, and sketchpad views.
Design app hooks for piece and connection selection, hover, and diff state.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖design](semiorepo://section/Design)

Design app hooks for piece and connection selection, hover, and diff state.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖sketchpad](semiorepo://section/Sketchpad)

Core reactive observation, synchronization hooks, and sketchpad store implementation.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖xstatehooks](semiorepo://section/XState%20Hooks)

React hooks for accessing XState sketchpad actor state and sending events.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖commands](semiorepo://section/Commands)

Exported sketchpad command map for theme, language, mode, device, and navigation.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖appsregistry](semiorepo://section/Apps%20Registry)

Dynamic app panel loader for registering app-specific panels.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖navbar](semiorepo://section/Navbar)

Focus-based navigation context provider for navbar breadcrumbs and search.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖sidepaneltabs](semiorepo://section/SidePanel%20Tabs)

Context provider managing side panel and HUD panel tab registration.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖origin](semiorepo://section/Origin)

Context provider for tracking the origin URL of the sketchpad instance.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖footeritems](semiorepo://section/Footer%20Items)

Context provider for dynamically registering footer bar items.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖globalfooteritems](semiorepo://section/Global%20Footer%20Items)

Global footer items component that registers persistent footer entries.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖conceptfilter](semiorepo://section/ConceptFilter)

Filter component for narrowing results by architectural concepts.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖toolgroup](semiorepo://section/ToolGroup)

Toolbar group component for switching between tool modes.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖dragdrop](semiorepo://section/DragDrop)

Context provider for drag-and-drop type placement interactions.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖hotkeys](semiorepo://section/Hotkeys)

Keyboard shortcut hook with configurable hotkey overrides.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖canvas](semiorepo://section/Canvas)

Canvas layout components for window management and multi-pane rendering.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖approuter](semiorepo://section/App%20Router)

React Router integration with scope providers and route-based app switching.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🔖sketchpadcomponents](semiorepo://section/Sketchpad%20Components)

Top-level sketchpad React components for rendering the complete application.

## [👤semio📚js🗃️sketchpad💻sketchpadtsx🛠️gettoolbargroupicon](semiorepo://definition/semio/js/sketchpad/Sketchpad.tsx/getToolbarGroupIcon)

Utility functions used across sketchpad components.

## [👤semio📚js🗃️sketchpad💻tutorialstsx](semiorepo://file/semio/js/sketchpad/Tutorials.tsx)

Interactive tutorial system with step-by-step guided workflows.

## [👤semio📚js🗃️sketchpad💻typetsx](semiorepo://file/semio/js/sketchpad/Type.tsx)

Type editor app for defining and editing type properties and ports.

## [👤semio📚js🗃️sketchpad🗃️apps💻indexts](semiorepo://file/semio/js/sketchpad/apps/index.ts)

Barrel export for all sketchpad app components.

## [👤semio📚js🗃️sketchpad🗃️apps💻indexts🔖exports](semiorepo://section/Exports)

Re-exports of app plugin utilities and types from the shared module.

## [👤semio📚js🗃️sketchpad💻elementstsx](semiorepo://file/semio/js/sketchpad/elements.tsx)

Shared UI elements and primitive components for sketchpad apps.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖imports](semiorepo://section/Imports)

External library and internal module imports used across all sections.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖sectionspecificity](semiorepo://section/Section%20Specificity)

Enum defining priority levels for section content ownership.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖interactioncontext](semiorepo://section/Interaction%20Context)

React context for tracking active UI interactions.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖levelcontext](semiorepo://section/Level%20Context)

React context for UI depth level tracking.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖element](semiorepo://section/Element)

Core element types, transaction context, and level-based CSS class helpers.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖command](semiorepo://section/Command)

Command palette UI built on cmdk primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖footer](semiorepo://section/Footer)

Status bar component at the bottom of the layout.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖layout](semiorepo://section/Layout)

Top-level layout orchestrating navbar, panels, canvas, and footer.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖popover](semiorepo://section/Popover)

Floating popover component built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖tooltip](semiorepo://section/Tooltip)

Tooltip components with expertise-level adaptive content.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖basecomponents](semiorepo://section/Base%20Components)

Foundational internal components like Label.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖displaycomponents](semiorepo://section/Display%20Components)

Read-only display wrappers for tooltips and callouts.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖aside](semiorepo://section/Aside)

Callout boxes for notes, tips, cautions, and dangers.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖avatar](semiorepo://section/Avatar)

User avatar components with image, fallback, drag, and table variants.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖card](semiorepo://section/Card)

Card container and grid layout for content blocks.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖spinner](semiorepo://section/Spinner)

Animated loading spinner in small, medium, or large sizes.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖loadingrow](semiorepo://section/LoadingRow)

Skeleton loading row with pulsing icon and name.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖diagramnode](semiorepo://section/DiagramNode)

Individual diagram node element with selection and hover states.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖hovercard](semiorepo://section/HoverCard)

Hover-triggered card built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖icons](semiorepo://section/Icons)

Cursor icon component for collaborative pointer display.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖section](semiorepo://section/Section)

Collapsible section container with heading and specificity.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖steps](semiorepo://section/Steps)

Ordered step list container for tutorial or wizard flows.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖inputcomponents](semiorepo://section/Input%20Components)

Compact action button group with dropdown support.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖actiongroup](semiorepo://section/ActionGroup)

Compact action button group with dropdown support.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖combobox](semiorepo://section/Combobox)

Searchable dropdown with popover options list.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖input](semiorepo://section/Input)

Text input field with label, validation, and clear support.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖select](semiorepo://section/Select)

Dropdown select built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖slider](semiorepo://section/Slider)

Range slider built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖stepper](semiorepo://section/Stepper)

Numeric stepper with increment/decrement and drag adjustment.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖textarea](semiorepo://section/Textarea)

Multi-line text input with label and validation.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖toggle](semiorepo://section/Toggle)

Toggle button with pressed/unpressed states.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖togglegroup](semiorepo://section/ToggleGroup)

Group of mutually exclusive or multi-select toggles.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents](semiorepo://section/Aggregation%20Components)

Collapsible accordion built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖accordion](semiorepo://section/Accordion)

Collapsible accordion built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖collapsible](semiorepo://section/Collapsible)

Collapsible section built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖dialog](semiorepo://section/Dialog)

Modal dialog built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖resizable](semiorepo://section/Resizable)

Resizable panel layout built on react-resizable-panels.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖scrollable](semiorepo://section/Scrollable)

Custom scrollable area built on Radix ScrollArea.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖band](semiorepo://section/Band)

Horizontal band of navigation items with labels and icons.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖strip](semiorepo://section/Strip)

Vertical strip of icon items for compact navigation.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖navbar](semiorepo://section/Navbar)

Top navigation bar with icon items.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖tabs](semiorepo://section/Tabs)

Tab container built on Radix primitives.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖tree](semiorepo://section/Tree)

Hierarchical tree view with sections, items, and file trees.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖navigationcomponents](semiorepo://section/Navigation%20Components)

Breadcrumb trail for hierarchical page navigation.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖breadcrumb](semiorepo://section/Breadcrumb)

Breadcrumb trail for hierarchical page navigation.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖pagenavigation](semiorepo://section/PageNavigation)

Previous/next page navigation links.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖panelcomponents](semiorepo://section/Panel%20Components)

Resizable dockable panel with sections and collapse support.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖panel](semiorepo://section/Panel)

Resizable dockable panel with sections and collapse support.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖panelgroup](semiorepo://section/PanelGroup)

Flex container grouping multiple panels together.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖leftpanel](semiorepo://section/LeftPanel)

Left-docked panel variant with right resize handle.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖rightpanel](semiorepo://section/RightPanel)

Right-docked panel variant with left resize handle.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖middlepanel](semiorepo://section/MiddlePanel)

Center panel variant without resize handles.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖bottompanel](semiorepo://section/BottomPanel)

Bottom-docked panel variant with top resize handle.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖sidepanel](semiorepo://section/SidePanel)

Collapsible side panel with tabbed content.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖hudpanel](semiorepo://section/HudPanel)

Floating heads-up display panel with tabs.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖windowcomponents](semiorepo://section/Window%20Components)

Draggable, resizable floating window with dashed border.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖window](semiorepo://section/Window)

Draggable, resizable floating window with dashed border.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖page](semiorepo://section/Page)

Full-page content wrapper with frontmatter and footer.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖diagram](semiorepo://section/Diagram)

Interactive node-edge diagram built on ReactFlow and D3 force.

## [👤semio📚js🗃️sketchpad💻elementstsx🔖table](semiorepo://section/Table)

Sortable, hierarchical data table with drag-drop support.

## [👤semio📚js🗃️sketchpad💻kitselectionhelperts](semiorepo://file/semio/js/sketchpad/kitSelectionHelper.ts)

Geometry and selection utilities for kit diagram interactions.

## [👤semio📚js🗃️sketchpad💻portcolorts](semiorepo://file/semio/js/sketchpad/portColor.ts)

Color mapping utilities for port visualization in diagrams.

## [👤semio📚js🗃️sketchpad💻portcolorts🔖portcolor](semiorepo://section/Port%20Color)

Assigns deterministic HSL color tones to ports based on compatibility groups.

## [👤semio📚js🗃️sketchpad💻portcolorts🪨defaultportguid](semiorepo://definition/semio/js/sketchpad/portColor.ts/DEFAULT_PORT_GUID)

Sentinel GUID for ports without an assigned identity.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️normalizeguid](semiorepo://definition/semio/js/sketchpad/portColor.ts/normalizeGuid)

Trims and normalizes a GUID string, returning undefined for empty values.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️normalizeportref](semiorepo://definition/semio/js/sketchpad/portColor.ts/normalizePortRef)

Extracts a GUID from a string or object with a guid property.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️hashstring](semiorepo://definition/semio/js/sketchpad/portColor.ts/hashString)

Produces a deterministic non-negative integer hash from a string.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️gettoneforkey](semiorepo://definition/semio/js/sketchpad/portColor.ts/getToneForKey)

Generates an HSL color tone from a port group key.

## [👤semio📚js🗃️sketchpad💻portcolorts🛠️createportgroupmap](semiorepo://definition/semio/js/sketchpad/portColor.ts/createPortGroupMap)

Builds a union-find map grouping compatible ports by root GUID.

## [👤semio📚js🗃️sketchpad💻sharedts](semiorepo://file/semio/js/sketchpad/shared.ts)

Shared state management types, hooks and store factories for sketchpad.

## [👤semio📚js💻viteenvdts](semiorepo://file/semio/js/vite-env.d.ts)

Vite client type declarations for the JavaScript workspace.

## [👤semio📚js💻viteenvdts🔖declarations](semiorepo://section/Declarations)

Ambient module declarations for non-standard import types.

## [👤semio🛂jsonschema💻buildts](semiorepo://file/semio/jsonschema/build.ts)

Build script for generating and exporting JSON Schema definitions.

## [👤semio🛂jsonschema💻buildts🪨inputfilepath](semiorepo://definition/semio/jsonschema/build.ts/inputFilePath)

Input JSON Schema file path.

## [👤semio🛂jsonschema💻buildts🪨outputfilepath](semiorepo://definition/semio/jsonschema/build.ts/outputFilePath)

Output file path for the unescaped JSON Schema.

## [👤semio🛂jsonschema💻buildts🪨jsoncontent](semiorepo://definition/semio/jsonschema/build.ts/jsonContent)

Raw JSON content read from the input schema file.

## [👤semio🛂jsonschema💻buildts🪨unescapedcontent](semiorepo://definition/semio/jsonschema/build.ts/unescapedContent)

Unescaped JSON content with backslash sequences resolved.

## [👤semio📚net🛅semio💻semiocs](semiorepo://file/semio/net/Semio/Semio.cs)

Core .NET library implementing the semio domain model and serialization.

## [👤semio📚net🛅semio💻semiocs🛠️symbol](semiorepo://definition/semio/net/Semio/Semio.cs/Symbol)

/ <summary>Abstract base for all expression tree nodes.</summary>
/ <remarks>
/ [👤semio📚net🛅semio💻semiocs🔖utility🔖expressions🛠️symbol](semiorepo://definition/semio/net/Semio/Semio.cs/Utility/Expressions/Symbol)
/ </remarks>

## [👤semio📚net🛅semio💻semiocs🛠️entity](semiorepo://definition/semio/net/Semio/Semio.cs/Entity)

/ Abstract generic base class providing equality, hashing, cloning, and validation.
/ [👤semio📚net🛅semio💻semiocs🔖entitying🛠️entity](semiorepo://definition/semio/net/Semio/Semio.cs/Entitying/Entity)

## [👤semio📚net🛅semio💻semiocs🛠️entityvalidator](semiorepo://definition/semio/net/Semio/Semio.cs/EntityValidator)

/ FluentValidation validator base for Entity subclasses.
/ [👤semio📚net🛅semio💻semiocs🔖entitying🛠️entityvalidator](semiorepo://definition/semio/net/Semio/Semio.cs/Entitying/EntityValidator)

## [👤semio📚net🛅semio💻buildts](semiorepo://file/semio/net/Semio/build.ts)

Build script for the Semio .NET library assembly.

## [👤semio📚net🛅semio💻buildts🪨msbuild](semiorepo://definition/semio/net/Semio/build.ts/msbuild)

MSBuild executable path for Visual Studio 2022.

## [👤semio🌐play💻indextsx](semiorepo://file/semio/play/index.tsx)

Entry point for the playground React app for interactive experimentation.

## [👤semio🌐play💻indextsx🔖entrypoint](semiorepo://section/Entrypoint)

Play application entrypoint registering sketchpad apps and rendering the root.

## [👤semio📚py💻semiopy🔖imports](semiorepo://section/Imports)

Standard library, third-party and framework imports.

## [👤semio📚py💻semiopy🔖typehints](semiorepo://section/Type%20Hints)

Custom type hint aliases used throughout the module.

## [👤semio📚py💻semiopy🔖constants](semiorepo://section/Constants)

Global constants for limits, paths, encodings and configuration.

## [👤semio📚py💻semiopy🔖utility](semiorepo://section/Utility)

General-purpose utility functions for encoding, formatting and transformation.

## [👤semio📚py💻semiopy🔖logging](semiorepo://section/Logging)

Module-level logger configuration.

## [👤semio📚py💻semiopy🔖exceptions](semiorepo://section/Exceptions)

Custom exception hierarchy for server, client and specification errors.

## [👤semio📚py💻semiopy🔖modeling](semiorepo://section/Modeling)

Abstract base classes for models, fields, ids, inputs, outputs and entities.

## [👤semio📚py💻semiopy🔖primitives](semiorepo://section/Primitives)

Abstract base classes for models, fields, ids, inputs, outputs and entities.

## [👤semio📚py💻semiopy🔖graphql](semiorepo://section/Graphql)

GraphQL node base classes for pydantic, sqlalchemy and relay integration.

## [👤semio📚py💻semiopy🔖domain](semiorepo://section/Domain)

Attribute entity with key-value pairs and definitions.

## [👤semio📚py💻semiopy🔖attribute](semiorepo://section/Attribute)

Attribute entity with key-value pairs and definitions.

## [👤semio📚py💻semiopy🔖tag](semiorepo://section/Tag)

Tag entity for categorizing and labeling kit elements.

## [👤semio📚py💻semiopy🔖concept](semiorepo://section/Concept)

Concept entity for semantic grouping of design elements.

## [👤semio📚py💻semiopy🔖coord](semiorepo://section/Coord)

Coordinate primitive for three-dimensional values.

## [👤semio📚py💻semiopy🔖point](semiorepo://section/Point)

Point primitive representing a position in 3D space.

## [👤semio📚py💻semiopy🔖vector](semiorepo://section/Vector)

Vector primitive representing a direction in 3D space.

## [👤semio📚py💻semiopy🔖plane](semiorepo://section/Plane)

Plane primitive representing an oriented coordinate frame in 3D space.

## [👤semio📚py💻semiopy🔖location](semiorepo://section/Location)

Location entity for geographic coordinates with longitude, latitude and altitude.

## [👤semio📚py💻semiopy🔖author](semiorepo://section/Author)

Author entity for tracking contributor identity and rank.

## [👤semio📚py💻semiopy🔖artifactauthor](semiorepo://section/ArtifactAuthor)

Artifact-author association entity linking artifacts to authors by email.

## [👤semio📚py💻semiopy🔖file](semiorepo://section/File)

File entity for managing binary assets with metadata and hashing.

## [👤semio📚py💻semiopy🔖folder](semiorepo://section/Folder)

Folder entity for hierarchical organization of kit content.

## [👤semio📚py💻semiopy🔖benchmark](semiorepo://section/Benchmark)

Benchmark entity for defining performance metrics with min-max bounds.

## [👤semio📚py💻semiopy🔖quality](semiorepo://section/Quality)

Quality entity for defining measurable properties with units and constraints.

## [👤semio📚py💻semiopy🔖prop](semiorepo://section/Prop)

Prop entity for key-value property pairs with units.

## [👤semio📚py💻semiopy🔖model](semiorepo://section/Model)

Model entity for 3D geometry representations linked to files.

## [👤semio📚py💻semiopy🔖port](semiorepo://section/Port)

Port entity for defining connection interfaces on types.

## [👤semio📚py💻semiopy🔖connector](semiorepo://section/Connector)

Compatible port entity for specifying allowed port pairings on connectors.

## [👤semio📚py💻semiopy🔖compatibleport](semiorepo://section/CompatiblePort)

Compatible port entity for specifying allowed port pairings on connectors.

## [👤semio📚py💻semiopy🔖type](semiorepo://section/Type)

Type entity for defining reusable parametric building blocks.

## [👤semio📚py💻semiopy🔖layer](semiorepo://section/Layer)

Layer entity for organizing design elements into visibility groups.

## [👤semio📚py💻semiopy🔖piece](semiorepo://section/Piece)

Piece entity for placed instances of types within a design.

## [👤semio📚py💻semiopy🔖group](semiorepo://section/Group)

Group entity for named collections of pieces in a design.

## [👤semio📚py💻semiopy🔖side](semiorepo://section/Side)

Side primitive for identifying a specific connector on a specific piece.

## [👤semio📚py💻semiopy🔖connection](semiorepo://section/Connection)

Connection entity for linking two pieces through their connectors.

## [👤semio📚py💻semiopy🔖stat](semiorepo://section/Stat)

Stat entity for recording computed statistics with bounds.

## [👤semio📚py💻semiopy🔖design](semiorepo://section/Design)

Design entity for composing pieces and connections into assemblies.

## [👤semio📚py💻semiopy🔖kit](semiorepo://section/Kit)

Kit entity for packaging types, designs, qualities and metadata.

## [👤semio📚py💻semiopy🔖designfamilyhelpers](semiorepo://section/Design%20Family%20Helpers)

Helper functions for querying design hierarchies and families.

## [👤semio📚py💻semiopy🔖typefamilyhelpers](semiorepo://section/Type%20Family%20Helpers)

Helper functions for querying type hierarchies and families.

## [👤semio📚py💻semiopy🔖movedgraphenenodes](semiorepo://section/Moved%20Graphene%20Nodes)

Graphene node definitions moved here due to forward-reference resolution order.

## [👤semio📚py💻semiopy🔖validation](semiorepo://section/Validation)

Validation logic for checking kit constraints and uniqueness rules.

## [👤semio📚py💻semiopy🔖dictbasedvalidation](semiorepo://section/Dict-based%20Validation)

Dictionary-based validation functions for kit data integrity.

## [👤semio📚py💻semiopy🔖graphoperations](semiorepo://section/Graph%20Operations)

Graph construction and traversal for piece connectivity analysis.

## [👤semio📚py💻semiopy🔖flattendesign](semiorepo://section/FlattenDesign)

Design flattening to resolve nested sub-designs into a single coordinate space.

## [👤semio📚py💻semiopy🔖kitdiffoperations](semiorepo://section/Kit%20Diff%20Operations)

Diffing and patching operations for comparing and merging kit versions.

## [👤semio📚py💻semiopy🔖kitimportexport](semiorepo://section/Kit%20Import/Export)

Import and export utilities for kit serialization and deserialization.

## [👤semio📚py💻semiopy🔖spatialmath](semiorepo://section/Spatial%20Math)

Spatial math utilities for vector normalization and plane computation.

## [👤semio📚rs💻semiors🔖utilityfunctions](semiorepo://section/Utility%20Functions)

/ <summary>Guid holds the data fields for a Guid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️guid](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/Guid)
/ </remarks>

## [👤semio📚rs💻semiors🔖finderfunctions](semiorepo://section/Finder%20Functions)

/ <summary>find_type_in_kit holds the data fields for a find_type_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findtypeinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_type_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🔖serialization](semiorepo://section/Serialization)

/ <summary>serialize_kit holds the data fields for a serialize_kit record.</summary>
/ [👤semio📚rs💻semiors🔖serialization🛠️serializekit](semiorepo://definition/semio/rs/semio.rs/Serialization/serialize_kit)

## [👤semio📚rs💻semiors🔖hasguidtrait](semiorepo://section/HasGuid%20Trait)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖hasguidtrait🛠️hasguid](semiorepo://definition/semio/rs/semio.rs/HasGuid%20Trait/HasGuid)
/ </remarks>

## [👤semio📚rs💻semiors🔖applydiff](semiorepo://section/ApplyDiff)

/ <summary>apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applycollectiondiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_collection_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🔖flattendesign](semiorepo://section/FlattenDesign)

/ <summary>FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
/ [👤semio📚rs💻semiors🔖flattendesign🛠️flattenedpiece](semiorepo://definition/semio/rs/semio.rs/FlattenDesign/FlattenedPiece)

## [👤semio📚rs💻semiors🪨semioerror](semiorepo://definition/semio/rs/semio.rs/SemioError)

/ <summary>SemioError holds the data fields for a SemioError record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖errortypes🛠️semioerror](semiorepo://definition/semio/rs/semio.rs/Error%20Types/SemioError)
/ </remarks>

## [👤semio📚rs💻semiors✂️result](semiorepo://definition/semio/rs/semio.rs/Result)

/ <summary>Result holds the data fields for a Result record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖errortypes🛠️result](semiorepo://definition/semio/rs/semio.rs/Error%20Types/Result)
/ </remarks>

## [👤semio📚rs💻semiors✂️guid](semiorepo://definition/semio/rs/semio.rs/Guid)

/ <summary>Guid holds the data fields for a Guid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️guid](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/Guid)
/ </remarks>

## [👤semio📚rs💻semiors🛠️guid](semiorepo://definition/semio/rs/semio.rs/guid)

/ <summary>guid holds the data fields for a guid record.</summary>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️guid](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/guid)

## [👤semio📚rs💻semiors🛠️normalize](semiorepo://definition/semio/rs/semio.rs/normalize)

/ <summary>normalize holds the data fields for a normalize record.</summary>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️normalize](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/normalize)

## [👤semio📚rs💻semiors🛠️round](semiorepo://definition/semio/rs/semio.rs/round)

/ <summary>round holds the data fields for a round record.</summary>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️round](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/round)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️jaccard](semiorepo://definition/semio/rs/semio.rs/jaccard)

/ <summary>jaccard holds the data fields for a jaccard record.</summary>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️jaccard](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/jaccard)

## [👤semio📚rs💻semiors🛠️deepequal](semiorepo://definition/semio/rs/semio.rs/deep_equal)

/ <summary>deep_equal holds the data fields for a deep_equal record.</summary>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️deepequal](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/deep_equal)

## [👤semio📚rs💻semiors🛠️generateuniquename](semiorepo://definition/semio/rs/semio.rs/generate_unique_name)

/ <summary>generate_unique_name performs the generate_unique_name operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖utilityfunctions🛠️generateuniquename](semiorepo://definition/semio/rs/semio.rs/Utility%20Functions/generate_unique_name)
/ </remarks>

## [👤semio📚rs💻semiors🛠️attribute](semiorepo://definition/semio/rs/semio.rs/Attribute)

/ <summary>Attribute holds the data fields for a Attribute record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypesattribute🛠️attribute](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Attribute/Attribute)

## [👤semio📚rs💻semiors🛠️attributeid](semiorepo://definition/semio/rs/semio.rs/AttributeId)

/ <summary>AttributeId holds the data fields for a AttributeId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesattribute🛠️attributeid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Attribute/AttributeId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️coord](semiorepo://definition/semio/rs/semio.rs/Coord)

/ <summary>Coord holds the data fields for a Coord record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypescoord🛠️coord](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Coord/Coord)
/ </remarks>

## [👤semio📚rs💻semiors🛠️vector](semiorepo://definition/semio/rs/semio.rs/Vector)

/ <summary>Vector holds the data fields for a Vector record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesvector🛠️vector](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Vector/Vector)
/ </remarks>

## [👤semio📚rs💻semiors🛠️plane](semiorepo://definition/semio/rs/semio.rs/Plane)

/ <summary>Plane holds the data fields for a Plane record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypesplane🛠️plane](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Plane/Plane)

## [👤semio📚rs💻semiors🛠️camera](semiorepo://definition/semio/rs/semio.rs/Camera)

/ <summary>Camera holds the data fields for a Camera record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypescamera🛠️camera](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Camera/Camera)
/ </remarks>

## [👤semio📚rs💻semiors🛠️locationid](semiorepo://definition/semio/rs/semio.rs/LocationId)

/ <summary>LocationId holds the data fields for a LocationId record.</summary>
/ <summary>LocationId holds the data fields for a LocationId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️locationid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/LocationId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️location](semiorepo://definition/semio/rs/semio.rs/Location)

/ <summary>Location holds the data fields for a Location record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️location](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/Location)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️authorid](semiorepo://definition/semio/rs/semio.rs/AuthorId)

/ <summary>AuthorId holds the data fields for a AuthorId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️authorid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/AuthorId)

## [👤semio📚rs💻semiors🛠️author](semiorepo://definition/semio/rs/semio.rs/Author)

/ <summary>Author holds the data fields for a Author record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️author](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/Author)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️folderid](semiorepo://definition/semio/rs/semio.rs/FolderId)

/ <summary>FolderId holds the data fields for a FolderId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️folderid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/FolderId)

## [👤semio📚rs💻semiors🛠️folder](semiorepo://definition/semio/rs/semio.rs/Folder)

/ <summary>Folder holds the data fields for a Folder record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️folder](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/Folder)
/ </remarks>

## [👤semio📚rs💻semiors🛠️fileid](semiorepo://definition/semio/rs/semio.rs/FileId)

/ <summary>FileId holds the data fields for a FileId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️fileid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/FileId)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️file](semiorepo://definition/semio/rs/semio.rs/File)

/ <summary>File holds the data fields for a File record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslocationauthorfilefolder🛠️file](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Location,%20Author,%20File,%20Folder/File)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️qualityid](semiorepo://definition/semio/rs/semio.rs/QualityId)

/ <summary>QualityId holds the data fields for a QualityId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️qualityid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/QualityId)
/ </remarks>

## [👤semio📚rs💻semiors🪨qualitykind](semiorepo://definition/semio/rs/semio.rs/QualityKind)

/ <summary>QualityKind holds the data fields for a QualityKind record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️qualitykind](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/QualityKind)

## [👤semio📚rs💻semiors🛠️quality](semiorepo://definition/semio/rs/semio.rs/Quality)

/ <summary>Quality holds the data fields for a Quality record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️quality](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/Quality)
/ </remarks>

## [👤semio📚rs💻semiors🛠️portid](semiorepo://definition/semio/rs/semio.rs/PortId)

/ <summary>PortId holds the data fields for a PortId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️portid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/PortId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️port](semiorepo://definition/semio/rs/semio.rs/Port)

/ <summary>Port holds the data fields for a Port record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️port](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/Port)
/ </remarks>

## [👤semio📚rs💻semiors🛠️tagid](semiorepo://definition/semio/rs/semio.rs/TagId)

/ <summary>TagId holds the data fields for a TagId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️tagid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/TagId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️tag](semiorepo://definition/semio/rs/semio.rs/Tag)

/ <summary>Tag holds the data fields for a Tag record.</summary>
/ <summary>Tag holds the data fields for a Tag record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️tag](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/Tag)
/ </remarks>

## [👤semio📚rs💻semiors🛠️conceptid](semiorepo://definition/semio/rs/semio.rs/ConceptId)

/ <summary>ConceptId holds the data fields for a ConceptId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️conceptid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/ConceptId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️concept](semiorepo://definition/semio/rs/semio.rs/Concept)

/ <summary>Concept holds the data fields for a Concept record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesqualityporttagconcept🛠️concept](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Quality,%20Port,%20Tag,%20Concept/Concept)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️propid](semiorepo://definition/semio/rs/semio.rs/PropId)

/ <summary>PropId holds the data fields for a PropId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️propid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Prop,%20Model,%20Connector/PropId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️prop](semiorepo://definition/semio/rs/semio.rs/Prop)

/ <summary>Prop holds the data fields for a Prop record.</summary>
/ <summary>Prop holds the data fields for a Prop record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️prop](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Prop,%20Model,%20Connector/Prop)
/ </remarks>

## [👤semio📚rs💻semiors🛠️modelid](semiorepo://definition/semio/rs/semio.rs/ModelId)

/ <summary>ModelId holds the data fields for a ModelId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️modelid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Prop,%20Model,%20Connector/ModelId)

## [👤semio📚rs💻semiors🛠️model](semiorepo://definition/semio/rs/semio.rs/Model)

/ <summary>Model holds the data fields for a Model record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️model](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Prop,%20Model,%20Connector/Model)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️connectorid](semiorepo://definition/semio/rs/semio.rs/ConnectorId)

/ <summary>ConnectorId holds the data fields for a ConnectorId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️connectorid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Prop,%20Model,%20Connector/ConnectorId)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️connector](semiorepo://definition/semio/rs/semio.rs/Connector)

/ <summary>Connector holds the data fields for a Connector record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypespropmodelconnector🛠️connector](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Prop,%20Model,%20Connector/Connector)
/ </remarks>

## [👤semio📚rs💻semiors🛠️typeid](semiorepo://definition/semio/rs/semio.rs/TypeId)

/ <summary>TypeId holds the data fields for a TypeId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypestype🛠️typeid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Type/TypeId)

## [👤semio📚rs💻semiors🛠️type](semiorepo://definition/semio/rs/semio.rs/Type)

/ <summary>Type holds the data fields for a Type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypestype🛠️type](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Type/Type)
/ </remarks>

## [👤semio📚rs💻semiors🛠️layerid](semiorepo://definition/semio/rs/semio.rs/LayerId)

/ <summary>LayerId holds the data fields for a LayerId record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layerid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/LayerId)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️layer](semiorepo://definition/semio/rs/semio.rs/Layer)

/ <summary>Layer holds the data fields for a Layer record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️layer](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/Layer)
/ </remarks>

## [👤semio📚rs💻semiors🛠️pieceid](semiorepo://definition/semio/rs/semio.rs/PieceId)

/ <summary>PieceId holds the data fields for a PieceId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️pieceid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/PieceId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️designid](semiorepo://definition/semio/rs/semio.rs/DesignId)

/ <summary>DesignId holds the data fields for a DesignId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️designid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/DesignId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️piece](semiorepo://definition/semio/rs/semio.rs/Piece)

/ <summary>Piece holds the data fields for a Piece record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️piece](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/Piece)
/ </remarks>

## [👤semio📚rs💻semiors🛠️groupid](semiorepo://definition/semio/rs/semio.rs/GroupId)

/ <summary>GroupId holds the data fields for a GroupId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️groupid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/GroupId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️group](semiorepo://definition/semio/rs/semio.rs/Group)

/ <summary>Group holds the data fields for a Group record.</summary>
/ <summary>Group holds the data fields for a Group record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️group](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/Group)
/ </remarks>

## [👤semio📚rs💻semiors🛠️side](semiorepo://definition/semio/rs/semio.rs/Side)

/ <summary>Side holds the data fields for a Side record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️side](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/Side)
/ </remarks>

## [👤semio📚rs💻semiors🛠️connectionid](semiorepo://definition/semio/rs/semio.rs/ConnectionId)

/ <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
/ <summary>ConnectionId holds the data fields for a ConnectionId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connectionid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/ConnectionId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️connection](semiorepo://definition/semio/rs/semio.rs/Connection)

/ <summary>Connection holds the data fields for a Connection record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️connection](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/Connection)
/ </remarks>

## [👤semio📚rs💻semiors🛠️statid](semiorepo://definition/semio/rs/semio.rs/StatId)

/ <summary>StatId holds the data fields for a StatId record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️statid](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/StatId)
/ </remarks>

## [👤semio📚rs💻semiors🛠️stat](semiorepo://definition/semio/rs/semio.rs/Stat)

/ <summary>Stat holds the data fields for a Stat record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypeslayerpiecegroupsideconnectionstat🛠️stat](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Layer,%20Piece,%20Group,%20Side,%20Connection,%20Stat/Stat)
/ </remarks>
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️design](semiorepo://definition/semio/rs/semio.rs/Design)

/ <summary>Design holds the data fields for a Design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖modeltypesdesign🛠️design](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Design/Design)
/ </remarks>

## [👤semio📚rs💻semiors🛠️kit](semiorepo://definition/semio/rs/semio.rs/Kit)

/ <summary>Kit holds the data fields for a Kit record.</summary>
/ [👤semio📚rs💻semiors🔖modeltypeskit🛠️kit](semiorepo://definition/semio/rs/semio.rs/Model%20Types%20-%20Kit/Kit)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️findtypeinkit](semiorepo://definition/semio/rs/semio.rs/find_type_in_kit)

/ <summary>find_type_in_kit holds the data fields for a find_type_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findtypeinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_type_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findtypeinkitmut](semiorepo://definition/semio/rs/semio.rs/find_type_in_kit_mut)

/ <summary>find_type_in_kit_mut holds the data fields for a find_type_in_kit_mut record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findtypeinkitmut](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_type_in_kit_mut)
/ </remarks>

## [👤semio📚rs💻semiors🛠️finddesigninkit](semiorepo://definition/semio/rs/semio.rs/find_design_in_kit)

/ <summary>find_design_in_kit holds the data fields for a find_design_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️finddesigninkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_design_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️finddesigninkitmut](semiorepo://definition/semio/rs/semio.rs/find_design_in_kit_mut)

/ <summary>find_design_in_kit_mut holds the data fields for a find_design_in_kit_mut record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️finddesigninkitmut](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_design_in_kit_mut)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findpieceindesign](semiorepo://definition/semio/rs/semio.rs/find_piece_in_design)

/ <summary>find_piece_in_design holds the data fields for a find_piece_in_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findpieceindesign](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_piece_in_design)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findpieceindesignmut](semiorepo://definition/semio/rs/semio.rs/find_piece_in_design_mut)

/ <summary>find_piece_in_design_mut performs the find_piece_in_design_mut operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findpieceindesignmut](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_piece_in_design_mut)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findconnectionindesign](semiorepo://definition/semio/rs/semio.rs/find_connection_in_design)

/ <summary>find_connection_in_design holds the data fields for a find_connection_in_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findconnectionindesign](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_connection_in_design)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findconnectorintype](semiorepo://definition/semio/rs/semio.rs/find_connector_in_type)

/ <summary>find_connector_in_type holds the data fields for a find_connector_in_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findconnectorintype](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_connector_in_type)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findmodelintype](semiorepo://definition/semio/rs/semio.rs/find_model_in_type)

/ <summary>find_model_in_type holds the data fields for a find_model_in_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findmodelintype](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_model_in_type)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findfileinkit](semiorepo://definition/semio/rs/semio.rs/find_file_in_kit)

/ <summary>find_file_in_kit performs the find_file_in_kit operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findfileinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_file_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findfolderinkit](semiorepo://definition/semio/rs/semio.rs/find_folder_in_kit)

/ <summary>find_folder_in_kit holds the data fields for a find_folder_in_kit record.</summary>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findfolderinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_folder_in_kit)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️findauthorinkit](semiorepo://definition/semio/rs/semio.rs/find_author_in_kit)

/ <summary>find_author_in_kit performs the find_author_in_kit operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findauthorinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_author_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findtaginkit](semiorepo://definition/semio/rs/semio.rs/find_tag_in_kit)

/ <summary>find_tag_in_kit holds the data fields for a find_tag_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findtaginkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_tag_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findconceptinkit](semiorepo://definition/semio/rs/semio.rs/find_concept_in_kit)

/ <summary>find_concept_in_kit holds the data fields for a find_concept_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findconceptinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_concept_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findqualityinkit](semiorepo://definition/semio/rs/semio.rs/find_quality_in_kit)

/ <summary>find_quality_in_kit holds the data fields for a find_quality_in_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findqualityinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_quality_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findinterfaceinkit](semiorepo://definition/semio/rs/semio.rs/find_interface_in_kit)

/ <summary>find_interface_in_kit performs the find_interface_in_kit operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findinterfaceinkit](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_interface_in_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findlayerindesign](semiorepo://definition/semio/rs/semio.rs/find_layer_in_design)

/ <summary>find_layer_in_design holds the data fields for a find_layer_in_design record.</summary>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findlayerindesign](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_layer_in_design)

## [👤semio📚rs💻semiors🛠️findgroupindesign](semiorepo://definition/semio/rs/semio.rs/find_group_in_design)

/ <summary>find_group_in_design holds the data fields for a find_group_in_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findgroupindesign](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_group_in_design)
/ </remarks>

## [👤semio📚rs💻semiors🛠️findstatindesign](semiorepo://definition/semio/rs/semio.rs/find_stat_in_design)

/ <summary>find_stat_in_design holds the data fields for a find_stat_in_design record.</summary>
/ [👤semio📚rs💻semiors🔖finderfunctions🛠️findstatindesign](semiorepo://definition/semio/rs/semio.rs/Finder%20Functions/find_stat_in_design)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️serializekit](semiorepo://definition/semio/rs/semio.rs/serialize_kit)

/ <summary>serialize_kit holds the data fields for a serialize_kit record.</summary>
/ [👤semio📚rs💻semiors🔖serialization🛠️serializekit](semiorepo://definition/semio/rs/semio.rs/Serialization/serialize_kit)

## [👤semio📚rs💻semiors🛠️deserializekit](semiorepo://definition/semio/rs/semio.rs/deserialize_kit)

/ <summary>deserialize_kit holds the data fields for a deserialize_kit record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️deserializekit](semiorepo://definition/semio/rs/semio.rs/Serialization/deserialize_kit)
/ </remarks>

## [👤semio📚rs💻semiors🛠️serializedesign](semiorepo://definition/semio/rs/semio.rs/serialize_design)

/ <summary>serialize_design performs the serialize_design operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️serializedesign](semiorepo://definition/semio/rs/semio.rs/Serialization/serialize_design)
/ </remarks>

## [👤semio📚rs💻semiors🛠️deserializedesign](semiorepo://definition/semio/rs/semio.rs/deserialize_design)

/ <summary>deserialize_design holds the data fields for a deserialize_design record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️deserializedesign](semiorepo://definition/semio/rs/semio.rs/Serialization/deserialize_design)
/ </remarks>

## [👤semio📚rs💻semiors🛠️serializetype](semiorepo://definition/semio/rs/semio.rs/serialize_type)

/ <summary>serialize_type holds the data fields for a serialize_type record.</summary>
/ <summary>serialize_type performs the serialize_type operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️serializetype](semiorepo://definition/semio/rs/semio.rs/Serialization/serialize_type)
/ </remarks>

## [👤semio📚rs💻semiors🛠️deserializetype](semiorepo://definition/semio/rs/semio.rs/deserialize_type)

/ <summary>deserialize_type holds the data fields for a deserialize_type record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️deserializetype](semiorepo://definition/semio/rs/semio.rs/Serialization/deserialize_type)
/ </remarks>

## [👤semio📚rs💻semiors🛠️arekitsequal](semiorepo://definition/semio/rs/semio.rs/are_kits_equal)

/ <summary>are_kits_equal performs the are_kits_equal operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️arekitsequal](semiorepo://definition/semio/rs/semio.rs/Serialization/are_kits_equal)
/ </remarks>

## [👤semio📚rs💻semiors🛠️aredesignsequal](semiorepo://definition/semio/rs/semio.rs/are_designs_equal)

/ <summary>are_designs_equal holds the data fields for a are_designs_equal record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️aredesignsequal](semiorepo://definition/semio/rs/semio.rs/Serialization/are_designs_equal)
/ </remarks>

## [👤semio📚rs💻semiors🛠️aretypesequal](semiorepo://definition/semio/rs/semio.rs/are_types_equal)

/ <summary>are_types_equal holds the data fields for a are_types_equal record.</summary>
/ [👤semio📚rs💻semiors🔖serialization🛠️aretypesequal](semiorepo://definition/semio/rs/semio.rs/Serialization/are_types_equal)

## [👤semio📚rs💻semiors🪨supportedmodelextensions](semiorepo://definition/semio/rs/semio.rs/SUPPORTED_MODEL_EXTENSIONS)

/ <summary>SUPPORTED_MODEL_EXTENSIONS holds the data fields for a SUPPORTED_MODEL_EXTENSIONS record.</summary>
/ [👤semio📚rs💻semiors🔖serialization🛠️supportedmodelextensions](semiorepo://definition/semio/rs/semio.rs/Serialization/SUPPORTED_MODEL_EXTENSIONS)

## [👤semio📚rs💻semiors🛠️issupportedmodelextension](semiorepo://definition/semio/rs/semio.rs/is_supported_model_extension)

/ <summary>is_supported_model_extension performs the is_supported_model_extension operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖serialization🛠️issupportedmodelextension](semiorepo://definition/semio/rs/semio.rs/Serialization/is_supported_model_extension)
/ </remarks>

## [👤semio📚rs💻semiors🛠️removeditem](semiorepo://definition/semio/rs/semio.rs/RemovedItem)

/ <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
/ <summary>RemovedItem holds the data fields for a RemovedItem record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️removeditem](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/RemovedItem)
/ </remarks>

## [👤semio📚rs💻semiors🛠️diffupdate](semiorepo://definition/semio/rs/semio.rs/DiffUpdate)

/ <summary>DiffUpdate holds the data fields for a DiffUpdate record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️diffupdate](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/DiffUpdate)
/ </remarks>

## [👤semio📚rs💻semiors🛠️collectiondiff](semiorepo://definition/semio/rs/semio.rs/CollectionDiff)

/ <summary>CollectionDiff holds the data fields for a CollectionDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️collectiondiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/CollectionDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️attributediff](semiorepo://definition/semio/rs/semio.rs/AttributeDiff)

/ <summary>AttributeDiff holds the data fields for a AttributeDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️attributediff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/AttributeDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️propdiff](semiorepo://definition/semio/rs/semio.rs/PropDiff)

/ <summary>PropDiff holds the data fields for a PropDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️propdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/PropDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️connectordiff](semiorepo://definition/semio/rs/semio.rs/ConnectorDiff)

/ <summary>ConnectorDiff holds the data fields for a ConnectorDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️connectordiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/ConnectorDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️modeldiff](semiorepo://definition/semio/rs/semio.rs/ModelDiff)

/ <summary>ModelDiff holds the data fields for a ModelDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️modeldiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/ModelDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️typediff](semiorepo://definition/semio/rs/semio.rs/TypeDiff)

/ <summary>TypeDiff holds the data fields for a TypeDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️typediff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/TypeDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️sidediff](semiorepo://definition/semio/rs/semio.rs/SideDiff)

/ <summary>SideDiff holds the data fields for a SideDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️sidediff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/SideDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️connectiondiff](semiorepo://definition/semio/rs/semio.rs/ConnectionDiff)

/ <summary>ConnectionDiff holds the data fields for a ConnectionDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️connectiondiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/ConnectionDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️piecediff](semiorepo://definition/semio/rs/semio.rs/PieceDiff)

/ <summary>PieceDiff holds the data fields for a PieceDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️piecediff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/PieceDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️layerdiff](semiorepo://definition/semio/rs/semio.rs/LayerDiff)

/ <summary>LayerDiff holds the data fields for a LayerDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️layerdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/LayerDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️groupdiff](semiorepo://definition/semio/rs/semio.rs/GroupDiff)

/ <summary>GroupDiff holds the data fields for a GroupDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️groupdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/GroupDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️statdiff](semiorepo://definition/semio/rs/semio.rs/StatDiff)

/ <summary>StatDiff holds the data fields for a StatDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️statdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/StatDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️designdiff](semiorepo://definition/semio/rs/semio.rs/DesignDiff)

/ <summary>DesignDiff holds the data fields for a DesignDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️designdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/DesignDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️tagdiff](semiorepo://definition/semio/rs/semio.rs/TagDiff)

/ <summary>TagDiff holds the data fields for a TagDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️tagdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/TagDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️conceptdiff](semiorepo://definition/semio/rs/semio.rs/ConceptDiff)

/ <summary>ConceptDiff holds the data fields for a ConceptDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️conceptdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/ConceptDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️portdiff](semiorepo://definition/semio/rs/semio.rs/PortDiff)

/ <summary>PortDiff holds the data fields for a PortDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️portdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/PortDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️qualitydiff](semiorepo://definition/semio/rs/semio.rs/QualityDiff)

/ <summary>QualityDiff holds the data fields for a QualityDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️qualitydiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/QualityDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️filediff](semiorepo://definition/semio/rs/semio.rs/FileDiff)

/ <summary>FileDiff holds the data fields for a FileDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️filediff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/FileDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️folderdiff](semiorepo://definition/semio/rs/semio.rs/FolderDiff)

/ <summary>FolderDiff holds the data fields for a FolderDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️folderdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/FolderDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️authordiff](semiorepo://definition/semio/rs/semio.rs/AuthorDiff)

/ <summary>AuthorDiff holds the data fields for a AuthorDiff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖difftypes🛠️authordiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/AuthorDiff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️kitdiff](semiorepo://definition/semio/rs/semio.rs/KitDiff)

/ <summary>KitDiff holds the data fields for a KitDiff record.</summary>
/ [👤semio📚rs💻semiors🔖difftypes🛠️kitdiff](semiorepo://definition/semio/rs/semio.rs/Diff%20Types/KitDiff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors✂️hasguid](semiorepo://definition/semio/rs/semio.rs/HasGuid)

/ <summary>HasGuid holds the data fields for a HasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖hasguidtrait🛠️hasguid](semiorepo://definition/semio/rs/semio.rs/HasGuid%20Trait/HasGuid)
/ </remarks>

## [👤semio📚rs💻semiors✂️diffhasguid](semiorepo://definition/semio/rs/semio.rs/DiffHasGuid)

/ <summary>DiffHasGuid holds the data fields for a DiffHasGuid record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖hasguidtrait🛠️diffhasguid](semiorepo://definition/semio/rs/semio.rs/HasGuid%20Trait/DiffHasGuid)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applycollectiondiff](semiorepo://definition/semio/rs/semio.rs/apply_collection_diff)

/ <summary>apply_collection_diff holds the data fields for a apply_collection_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applycollectiondiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_collection_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyattributediff](semiorepo://definition/semio/rs/semio.rs/apply_attribute_diff)

/ <summary>apply_attribute_diff performs the apply_attribute_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyattributediff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_attribute_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applypropdiff](semiorepo://definition/semio/rs/semio.rs/apply_prop_diff)

/ <summary>apply_prop_diff holds the data fields for a apply_prop_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applypropdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_prop_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyconnectordiff](semiorepo://definition/semio/rs/semio.rs/apply_connector_diff)

/ <summary>apply_connector_diff holds the data fields for a apply_connector_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyconnectordiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_connector_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applymodeldiff](semiorepo://definition/semio/rs/semio.rs/apply_model_diff)

/ <summary>apply_model_diff performs the apply_model_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applymodeldiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_model_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applytypediff](semiorepo://definition/semio/rs/semio.rs/apply_type_diff)

/ <summary>apply_type_diff holds the data fields for a apply_type_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applytypediff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_type_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applylayerdiff](semiorepo://definition/semio/rs/semio.rs/apply_layer_diff)

/ <summary>apply_layer_diff holds the data fields for a apply_layer_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applylayerdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_layer_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️applygroupdiff](semiorepo://definition/semio/rs/semio.rs/apply_group_diff)

/ <summary>apply_group_diff holds the data fields for a apply_group_diff record.</summary>
/ <summary>apply_group_diff performs the apply_group_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applygroupdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_group_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applystatdiff](semiorepo://definition/semio/rs/semio.rs/apply_stat_diff)

/ <summary>apply_stat_diff holds the data fields for a apply_stat_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applystatdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_stat_diff)

## [👤semio📚rs💻semiors🛠️applypiecediff](semiorepo://definition/semio/rs/semio.rs/apply_piece_diff)

/ <summary>apply_piece_diff holds the data fields for a apply_piece_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applypiecediff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_piece_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyconnectiondiff](semiorepo://definition/semio/rs/semio.rs/apply_connection_diff)

/ <summary>apply_connection_diff holds the data fields for a apply_connection_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyconnectiondiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_connection_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applydesigndiff](semiorepo://definition/semio/rs/semio.rs/apply_design_diff)

/ <summary>apply_design_diff performs the apply_design_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applydesigndiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_design_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applytagdiff](semiorepo://definition/semio/rs/semio.rs/apply_tag_diff)

/ <summary>apply_tag_diff holds the data fields for a apply_tag_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applytagdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_tag_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyconceptdiff](semiorepo://definition/semio/rs/semio.rs/apply_concept_diff)

/ <summary>apply_concept_diff holds the data fields for a apply_concept_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyconceptdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_concept_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyinterfacediff](semiorepo://definition/semio/rs/semio.rs/apply_interface_diff)

/ <summary>apply_interface_diff holds the data fields for a apply_interface_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyinterfacediff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_interface_diff)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyqualitydiff](semiorepo://definition/semio/rs/semio.rs/apply_quality_diff)

/ <summary>apply_quality_diff holds the data fields for a apply_quality_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyqualitydiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_quality_diff)

## [👤semio📚rs💻semiors🛠️applyfilediff](semiorepo://definition/semio/rs/semio.rs/apply_file_diff)

/ <summary>apply_file_diff holds the data fields for a apply_file_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyfilediff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_file_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyfolderdiff](semiorepo://definition/semio/rs/semio.rs/apply_folder_diff)

/ <summary>apply_folder_diff performs the apply_folder_diff operation.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyfolderdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_folder_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️applyauthordiff](semiorepo://definition/semio/rs/semio.rs/apply_author_diff)

/ <summary>apply_author_diff holds the data fields for a apply_author_diff record.</summary>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applyauthordiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_author_diff)

## [👤semio📚rs💻semiors🛠️applykitdiff](semiorepo://definition/semio/rs/semio.rs/apply_kit_diff)

/ <summary>apply_kit_diff holds the data fields for a apply_kit_diff record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖applydiff🛠️applykitdiff](semiorepo://definition/semio/rs/semio.rs/ApplyDiff/apply_kit_diff)
/ </remarks>

## [👤semio📚rs💻semiors🛠️flattenedpiece](semiorepo://definition/semio/rs/semio.rs/FlattenedPiece)

/ <summary>FlattenedPiece holds the data fields for a FlattenedPiece record.</summary>
/ [👤semio📚rs💻semiors🔖flattendesign🛠️flattenedpiece](semiorepo://definition/semio/rs/semio.rs/FlattenDesign/FlattenedPiece)

## [👤semio📚rs💻semiors🛠️flattendesign](semiorepo://definition/semio/rs/semio.rs/flatten_design)

/ <summary>flatten_design holds the data fields for a flatten_design record.</summary>
/ [👤semio📚rs💻semiors🔖flattendesign🛠️flattendesign](semiorepo://definition/semio/rs/semio.rs/FlattenDesign/flatten_design)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️validationproblem](semiorepo://definition/semio/rs/semio.rs/ValidationProblem)

/ <summary>ValidationProblem holds the data fields for a ValidationProblem record.</summary>
/ <remarks>
/ </remarks>
/ <remarks>
/ [👤semio📚rs💻semiors🔖validationtypes🛠️validationproblem](semiorepo://definition/semio/rs/semio.rs/Validation%20Types/ValidationProblem)
/ </remarks>

## [👤semio📚rs💻semiors🛠️validationfix](semiorepo://definition/semio/rs/semio.rs/ValidationFix)

/ <summary>ValidationFix holds the data fields for a ValidationFix record.</summary>
/ <remarks>
/ [👤semio📚rs💻semiors🔖validationtypes🛠️validationfix](semiorepo://definition/semio/rs/semio.rs/Validation%20Types/ValidationFix)
/ </remarks>

## [👤semio📚rs💻semiors🛠️validationresult](semiorepo://definition/semio/rs/semio.rs/ValidationResult)

/ <summary>ValidationResult holds the data fields for a ValidationResult record.</summary>
/ [👤semio📚rs💻semiors🔖validationtypes🛠️validationresult](semiorepo://definition/semio/rs/semio.rs/Validation%20Types/ValidationResult)
/ <remarks>
/ </remarks>

## [👤semio📚rs💻semiors🛠️validatekit](semiorepo://definition/semio/rs/semio.rs/validate_kit)

/ <summary>validate_kit holds the data fields for a validate_kit record.</summary>
/ [👤semio📚rs💻semiors🔖validationtypes🛠️validatekit](semiorepo://definition/semio/rs/semio.rs/Validation%20Types/validate_kit)

## [👤semio📚rs💻semiors🛠️sqlite](semiorepo://definition/semio/rs/semio.rs/sqlite)

/ <summary>sqlite holds the data fields for a sqlite record.</summary>
/ [👤semio📚rs💻semiors🔖sqliteimport🔖export🛠️sqlite](semiorepo://definition/semio/rs/semio.rs/SQLite%20Import/Export/sqlite)

## [👤semio📚rs💻semiors🛠️ziproundtrip](semiorepo://definition/semio/rs/semio.rs/zip_roundtrip)

/ <summary>zip_roundtrip holds the data fields for a zip_roundtrip record.</summary>
/ [👤semio📚rs💻semiors🔖zipimport🔖export🛠️ziproundtrip](semiorepo://definition/semio/rs/semio.rs/Zip%20Import/Export/zip_roundtrip)

## [👤semio📚rs💻semiors🛠️wasm](semiorepo://definition/semio/rs/semio.rs/wasm)

/ <summary>wasm holds the data fields for a wasm record.</summary>
/ [👤semio📚rs💻semiors🔖wasmbindings🛠️wasm](semiorepo://definition/semio/rs/semio.rs/WASM%20Bindings/wasm)

## [👤semio🖱️sketchpad💻indextsx](semiorepo://file/semio/sketchpad/index.tsx)

Entry point for the standalone sketchpad web application.

## [👤semio🖱️sketchpad💻indextsx🔖entrypoint](semiorepo://section/Entrypoint)

Sketchpad application entrypoint registering apps and rendering the root.
