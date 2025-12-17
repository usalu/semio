---
slug: STATE-WRITES
summary: Update state writes handling
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.885Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

# Plan

Got it, we focus on **writes only**, don’t touch/introduce a new read layer, don’t use factories, and make sure:

- `value` comes from the **existing read hooks / store reads**
- `set` + `canSet` come from the **sketchpad state machine + command system**
- **only one exported hook per thing**, e.g. `useFlatPiecePlaneXAxisY()` returning `[state, setState, canSetState]`
- UI knows nothing about commands or machine – it just calls the hook.

Here’s a concrete refactor plan.

---

## 1. Clarify the current write pipeline

Right now:

- **Design app commands:**
  - Defined in `Design.tsx` (`designAppCommands` map).
  - Registered into `DesignStore`’s `commandRegistry` in its constructor.
  - Executed via `DesignStore.execute(commandId, origin, ...args)` which:
    - Wraps `executeCommand`.
    - Builds a `DesignAppCommandContext` (kit snapshot, design snapshot, designApp state).
    - Runs the command.
    - Applies `diff`/`kitDiff` to Yjs via `change(...)` and `kitStore.change(...)`.
    - Records edits / transactions.

- **UI write hooks for design app (`Design.tsx`):**
  - `useDesignAppCommands()` returns an object with methods like:
    - `deleteSelected(origin)` → `store.execute("semio.designApp.deleteSelected", origin)`
    - `setActiveTool(origin, tool)` → `store.execute("semio.designApp.setActiveTool", origin, tool)`
    - etc.

  - Components use these directly.

- **XState machine (`machines.ts`) already has:**
  - Events like `"DESIGN.SET_DIAGRAM_SCALE"`, `"DESIGN.DELETE_SELECTED"`, `"DESIGN.SELECT_PIECE"`, etc.
  - Actions that update **local app state** (`designApps[key]` in machine context).
  - But _most commands that actually mutate kit/design data_ are still called directly from React via the Yjs store, not via the machine.

Goal: flip this so that **all writes** go:

> UI hook → `actor.send(...)` → machine action → `DesignStore.execute(...)` → commands → Yjs.

Reads stay as they are.

---

## 2. Define the triadic hook pattern

For every “thing” we care about (diagram scale, panel visibility, selection, a piece plane component), the exported hook should have the shape:

```ts
export function useSomething(): [ValueType, (value: ValueType) => void, boolean] {
  // 1. Read value via existing mechanism (store/read hooks)
  // 2. Get XState actor & canSet via selectors
  // 3. Build a setter that sends an event to the machine
}
```

**No factories**, just manually written hooks.

Concretely, the pattern:

1. **Value**:
   - Comes from existing read hooks / store access:
     - e.g. `useDesignDiagramScale()` you already have, or `useFlatPiecePlane()` and then `.xAxis.y`.

   - Or a small wrapper around current read helpers (but no new generic read infrastructure in this plan).

2. **canSet**:
   - Comes from **XState selectors** via `useSelector(actor, selector)` (in `xstate-hooks.ts`).
   - Encodes app-level rules: is this design editable? is this mode active? etc.

3. **set**:
   - Uses `useActor()` (or your existing `useSketchpadActor` helper).
   - Gets IDs from scopes: `useKitScope`, `useDesignScope`, `usePieceScope`.
   - Sends a machine event with `{ type: ..., kitGuid, designGuid, pieceGuid, value }`.

The machine action for that event then calls the right `execute(...)` on the store.

---

## 3. Wire the state machine → command system properly

### 3.1. For design app writes

For any design app write (e.g. delete selection, move piece, change plane component, change diagram scale):

1. **Add / use an event in `machines.ts`**, if not already there, e.g.:

   ```ts
   type SketchpadEvent =
     | { type: "DESIGN.SET_DIAGRAM_SCALE"; kitGuid: Guid; designGuid: Guid; scale: number }
     | { type: "DESIGN.UPDATE_PIECE_PLANE_XAXIS_Y"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid; value: number }
     | ...;
   ```

2. **Add actions in the machine** that:
   - _First_: update local app state, if it is app state (e.g. diagram scale stored in machine context for view logic).
   - _Then_: call the design app store command for the actual kit/design mutation **if it should touch kit data**.

   For example, for a new plane-component write:

   ```ts
   const actions = {
     designUpdatePiecePlaneXAxisY: ({ context, event }) => {
       if (event.type !== "DESIGN.UPDATE_PIECE_PLANE_XAXIS_Y") return;

       const store = context.sketchpadStore.designApp({ kit: event.kitGuid, design: event.designGuid }) as DesignStore;

       store.execute(
         "semio.designApp.updatePieces", // existing command
         "semio.sketchpad.useFlatPiecePlaneXAxisY", // origin
         [{ id: event.pieceGuid, diff: { plane: { xAxis: { y: event.value } } } }],
       );
     },
   };
   ```

   For existing things like `"DESIGN.DELETE_SELECTED"`:
   - Instead of just clearing selection in the machine (what it does now), extend the action to:

     ```ts
     const store = context.sketchpadStore.designApp(...);
     store.execute("semio.designApp.deleteSelected", "semio.sketchpad.designDeleteSelected");
     ```

   - And _also_ clear selection in the machine context if you want local copy to stay in sync.

3. **Keep kit/design data out of the machine context**:
   - The machine only knows **which kit/design/piece** is targeted and what UI state should be (selection, hover, panel open).
   - All heavy lifting on data happens in commands, not in XState.

### 3.2. For other apps (home/kit/type/quality)

Same idea:

- Add events like `"KIT.RENAME_TYPE"`, `"TYPE.SET_PORT_LABEL"`, etc.
- Actions locate the right app controller / kit store and call `execute(...)` with the right command IDs.
- Local state (panels, selection, hover) is updated in machine context.

---

## 4. Replace `useDesignAppCommands` with triadic hooks

Right now, in `Design.tsx`:

- `useDesignAppCommands()` returns a big object of functions that directly call `store.execute("semio.designApp.*", origin, ...)`.

Plan:

1. **Stop exporting** `useDesignAppCommands` to UI components.

2. For each command that the UI needs, introduce a dedicated triadic hook that matches the pattern:
   - Example: diagram scale

     ```ts
     export function useDesignDiagramScaleTriad(): [number | undefined, (scale: number) => void, boolean] {
       // read
       const scale = useDesignDiagramScale(); // existing read hook

       // write / canSet
       const actor = useActor();
       const { kitGuid, designGuid } = useDesignScope(); // assuming scope hooks

       const canSet = useSelector(actor, (state) => {
         // e.g. check if this design is editable, based on machine's local state
         const app = selectDesignAppState(state, kitGuid, designGuid);
         return !app.isReadOnly;
       });

       const setScale = (value: number) => {
         if (!canSet) return;
         actor.send({
           type: "DESIGN.SET_DIAGRAM_SCALE",
           kitGuid,
           designGuid,
           scale: value,
         });
       };

       return [scale, setScale, canSet];
     }
     ```

     The `"DESIGN.SET_DIAGRAM_SCALE"` action then must call the appropriate **store execute** command, e.g. `"semio.designApp.setDiagramScale"` or a more generic `"updateDesignAppState"` if you have it.

3. **Example: delete selected pieces**

   ```ts
   export function useDesignDeleteSelectedTriad(): [boolean, () => void, boolean] {
     const actor = useActor();
     const { kitGuid, designGuid } = useDesignScope();

     const canDelete = useSelector(actor, (state) => {
       const app = selectDesignAppState(state, kitGuid, designGuid);
       const sel = app.selection;
       const hasSelection = !!(sel?.pieces?.length || sel?.connections?.length);
       return hasSelection && !app.isReadOnly;
     });

     const deleteSelected = () => {
       if (!canDelete) return;
       actor.send({ type: "DESIGN.DELETE_SELECTED", kitGuid, designGuid });
     };

     // state for this triad could just be “is there anything to delete?”
     return [canDelete, deleteSelected, canDelete];
   }
   ```

   In the machine, `"DESIGN.DELETE_SELECTED"` action calls `store.execute("semio.designApp.deleteSelected", origin)`.

4. Over time, **replace all uses** of `useDesignAppCommands()` in `Design.tsx` UI components with the appropriate triadic hooks:
   - `const [_, deleteSelected, canDelete] = useDesignDeleteSelectedTriad();`
   - `const [scale, setScale, canSetScale] = useDesignDiagramScaleTriad();`
   - etc.

---

## 5. Piece-level example: `useFlatPiecePlaneXAxisY`

For the concrete example you gave:

```ts
const [flatPiecePlaneXAxisY, setFlatPiecePlaneXAxisY, canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY();
```

Implementation sketch:

1. **Read**:
   - Use the existing read path; for example, if you have:

     ```ts
     const plane = useFlatPiecePlane(); // current hook
     const value = plane?.xAxis.y ?? 0;
     ```

   - Or if you already have a dedicated `useFlatPiecePlaneXAxisY` read hook, just call that.

2. **Write / canSet**:

   ```ts
   export function useFlatPiecePlaneXAxisY(): [number, (value: number) => void, boolean] {
     // READ: existing mechanism
     const plane = useFlatPiecePlane(); // uses current metadata/flattening logic
     const value = plane?.xAxis.y ?? 0;

     // IDs from scopes
     const { kitGuid } = useKitScope();
     const { designGuid } = useDesignScope();
     const { pieceGuid } = usePieceScope();

     const actor = useActor();

     const canSet = useSelector(actor, (state) => {
       const app = selectDesignAppState(state, kitGuid, designGuid);
       // example logic: editable design, correct tool, etc.
       return app.activeTool === ToolKind.Move && !app.isReadOnly;
     });

     const setValue = (next: number) => {
       if (!canSet) return;
       actor.send({
         type: "DESIGN.UPDATE_PIECE_PLANE_XAXIS_Y",
         kitGuid,
         designGuid,
         pieceGuid,
         value: next,
       });
     };

     return [value, setValue, canSet];
   }
   ```

   Then in `machines.ts`:
   - Add the `"DESIGN.UPDATE_PIECE_PLANE_XAXIS_Y"` event type.
   - Add an action that calls `DesignStore.execute("semio.designApp.updatePieces", origin, [{ id, diff }])` as shown earlier.

No second hook, no factory, no new read system.

---

## 6. Keep scopes as the only way to pass IDs

Your requirements include:

- “All hooks have no parameters and context is passed purely over scopes.”

So:

- For design-level hooks: use `useDesignScope()` to get `{ kitGuid, designGuid }`.
- For piece-level hooks: add `usePieceScope()` that also exposes `pieceGuid`.
- For other domains (type, connection, quality) do the same.

Every triadic hook:

- Asserts that the required scope(s) are present (or returns a safe default + `canSet = false`).
- Never receives IDs as parameters.

---

## 7. Summary of the write-side refactor

1. **Leave all read mechanisms as they are** (even if overfetching for now).
2. **Move all writes** from `useDesignAppCommands` / direct store calls in components into:
   - XState events (`DESIGN.*`, `TYPE.*`, `KIT.*`, etc.).
   - Machine actions that call `*.execute("semio.*", origin, ...)` on the appropriate store/controller.

3. **Export only triadic hooks** for UI:
   - `useDesignDiagramScale() → [scale, setScale, canSetScale]`
   - `useDesignSelection() → [selection, setSelection, canSetSelection]`
   - `useFlatPiecePlaneXAxisY() → [value, setValue, canSetValue]`
   - etc.

4. Inside each triadic hook:
   - **read** via existing read hook(s) / store selectors.
   - **set** by sending an event to the sketchpad machine.
   - **canSet** read from the machine via `useSelector`.

Once this is done, the UI is completely decoupled from both Yjs and the command system; it only knows about simple, local-looking React hooks, while the real work is done by the combination of XState (writes) and your existing command infrastructure (mutating the kit/design data).

# Changes

## Implemented

### 1. Added Scope-Based Triadic Hooks in `xstate-hooks.ts`

Added lazy import pattern to avoid circular dependencies:

```ts
let _scopeImports: { ... } | null = null;
function getScopeImports() {
  if (!_scopeImports) {
    const sketchpad = require("./Sketchpad");
    _scopeImports = { useKitScope, useDesignScope, useTypeScope, usePieceScope, useConnectionScope, useQualityScope };
  }
  return _scopeImports;
}
```

Implemented triadic hooks for each app following the pattern `[value, setValue, canSetValue]`:

**Design App Hooks:**

- `useDesignAppSelection()` - Selection state (pieces, connections, ports)
- `useDesignAppHover()` - Hover state
- `useDesignAppDiagramScale()` - Diagram zoom scale
- `useDesignAppDiagramCenter()` - Diagram pan center
- `useDesignAppCamera()` - 3D camera state
- `useDesignAppActiveTool()` - Current tool selection
- `useDesignAppFullscreenWindow()` - Fullscreen window state
- `useDesignAppFocusedPiece()` - Currently focused piece
- `useDesignAppPanelVisibility()` - Panel open/closed state
- `useDesignAppSelectedModelTags()` - Selected model tags per type

**Type App Hooks:**

- `useTypeAppSelection()` - Selection state (ports, models)
- `useTypeAppHover()` - Hover state
- `useTypeAppCamera()` - 3D camera state
- `useTypeAppActiveTool()` - Current tool selection
- `useTypeAppFocusedPort()` - Currently focused port
- `useTypeAppPanelVisibility()` - Panel open/closed state
- `useTypeAppFullscreenWindow()` - Fullscreen window state
- `useTypeAppSelectedModelTags()` - Selected model tags

**Kit App Hooks:**

- `useKitAppSelection()` - Selection state
- `useKitAppHover()` - Hover state
- `useKitAppPanelVisibility()` - Panel open/closed state
- `useKitAppFilterSearch()` - Filter search text
- `useKitAppExpandedRows()` - Expanded row GUIDs

**Home App Hooks:**

- `useHomeAppSelection()` - Kit selection state
- `useHomeAppHover()` - Hover state
- `useHomeAppPanelVisibility()` - Panel open/closed state
- `useHomeAppSortColumn()` - Sort column key
- `useHomeAppSortDirection()` - Sort direction (asc/desc)

### 2. Added New Events to `machines.ts`

Added to `SketchpadEvent` type:

- `HOME.SET_PANEL_VISIBILITY` - Set home panel visibility
- `KIT.SET_PANEL_VISIBILITY` - Set kit panel visibility
- `KIT.SET_EXPANDED_ROWS` - Set expanded rows in kit table
- `TYPE.SET_PANEL_VISIBILITY` - Set type panel visibility
- `TYPE.SET_FULLSCREEN_WINDOW` - Set type fullscreen window
- `DESIGN.SET_PANEL_VISIBILITY` - Set design panel visibility

### 3. Added New Actions to `machines.ts`

Implemented action handlers:

- `homeSetPanelVisibility` - Updates `homeApp.panels` Map
- `kitSetPanelVisibility` - Updates `kitApps[key].panels` Map
- `kitSetExpandedRows` - Updates `kitApps[key].expandedRows` Set
- `typeSetPanelVisibility` - Updates `typeApps[key].panels` Map
- `typeSetFullscreenWindow` - Updates `typeApps[key].fullscreenWindow`
- `designSetPanelVisibility` - Updates `designApps[key].panels` Map

### 4. Wired Events to Actions

Added event handlers in the machine's `on` block:

```ts
"HOME.SET_PANEL_VISIBILITY": { actions: "homeSetPanelVisibility" }
"KIT.SET_PANEL_VISIBILITY": { actions: "kitSetPanelVisibility" }
"KIT.SET_EXPANDED_ROWS": { actions: "kitSetExpandedRows" }
"TYPE.SET_PANEL_VISIBILITY": { actions: "typeSetPanelVisibility" }
"TYPE.SET_FULLSCREEN_WINDOW": { actions: "typeSetFullscreenWindow" }
"DESIGN.SET_PANEL_VISIBILITY": { actions: "designSetPanelVisibility" }
```

## Pending

- **Piece-level hooks**: Implement `useFlatPiecePlaneXAxisY` which requires:
  - `DESIGN.UPDATE_PIECE_PLANE_XAXIS_Y` event
  - Action that calls `DesignStore.execute("semio.designApp.updatePieces", ...)`
  - Uses `usePieceScope()` for piece context
- **Replace UI usages**: Gradually replace `useDesignAppCommands()` calls with triadic hooks
- **Quality app hooks**: Add triadic hooks for quality app once needed

```

```
