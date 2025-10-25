# Window Component Generalization

This document describes the generalization work completed for window components to ensure a uniform experience across all editors.

## Completed Work

### 1. Base Scene Component Enhancement (`js/js/elements/windows/Scene.tsx`)

#### Added `<Model>` Component
A new generalized `<Model>` component was added to provide consistent selection, hover, and color handling for 3D objects:

**Features:**
- Automatic selection state styling (`selected` prop)
- Automatic hover state styling (`hovered` prop)
- Consistent color theming using CSS variables:
  - `--foreground` for normal state
  - `--active-base` for selected state
  - `--hover-base` for hovered state
- Automatic edge rendering with `showEdges` prop
- Event handlers: `onClick`, `onPointerEnter`, `onPointerLeave`
- Custom material properties: `color`, `emissiveColor`, `emissiveIntensity`
- UserData support for storing metadata

**Usage Example:**
```tsx
<Model
  selected={isSelected}
  hovered={isHovered}
  onClick={handleClick}
  onPointerEnter={handlePointerEnter}
  onPointerLeave={handlePointerLeave}
  color={materialColor}
  emissiveIntensity={0.45}
  showEdges
  edgeColor={foregroundColor}
  userData={{ pieceId: piece.guid }}
>
  {/* Custom mesh content or uses default box geometry */}
</Model>
```

### 2. Base Diagram Component Enhancement (`js/js/elements/windows/Diagram.tsx`)

#### Enhanced with Controlled/Uncontrolled Modes
The base Diagram component now supports both controlled and uncontrolled modes, making it flexible for simple and complex use cases:

**New Features:**
- **Controlled Mode**: Pass `nodes` and `edges` props with `onNodesChangeReactFlow` and `onEdgesChangeReactFlow`
- **Uncontrolled Mode**: Pass `initialNodes` and `initialEdges` props (existing behavior)
- Additional drag callbacks: `onNodeDragStart`, `onNodeDrag`, `onNodeDragStop`
- Pane interaction: `onPaneClick`, `onMoveEnd`
- Advanced ReactFlow props: `connectionLineComponent`, `elementsSelectable`, `nodesFocusable`, `edgesFocusable`, `nodesDraggable`, `miniMapNodeComponent`
- Support for `panOnDrag` as boolean or number array

**Controlled Mode Example:**
```tsx
<Diagram
  nodeTypes={nodeTypes}
  edgeTypes={edgeTypes}
  nodes={controlledNodes}
  edges={controlledEdges}
  onNodesChangeReactFlow={handleNodesChange}
  onEdgesChangeReactFlow={handleEdgesChange}
  onNodeDrag={handleNodeDrag}
  elementsSelectable={false}
  nodesDraggable={true}
/>
```

### 3. Design Editor Scene Update (`js/js/sketchpad/editors/design/canvas/Scene.tsx`)

#### Migrated to Use `<Model>` Component
The design editor Scene now uses the base `<Model>` component instead of raw Three.js mesh code:

**Before:**
```tsx
<mesh onClick={onSelect} onPointerEnter={() => hoverPiece(piece.guid)} onPointerLeave={() => clearHover()}>
  <boxGeometry args={[1, 1, 1]} />
  <meshStandardMaterial color={materialColor} emissive={emissiveColor} emissiveIntensity={0.45} />
  <Edges scale={1.001} color={foregroundColor} />
</mesh>
```

**After:**
```tsx
<Model
  selected={isSelected}
  hovered={isHovered}
  onClick={onSelect}
  onPointerEnter={() => hoverPiece(piece.guid)}
  onPointerLeave={() => clearHover()}
  color={materialColor}
  emissiveColor={emissiveColor}
  emissiveIntensity={0.45}
  showEdges
  edgeColor={foregroundColor}
/>
```

**Benefits:**
- Consistent color handling across all editors
- Automatic selection/hover state management
- Cleaner, more declarative code
- Easier to maintain and extend

### 4. Editor Component Verification

#### Quality Editor (`js/js/sketchpad/editors/quality/canvas/Diagram.tsx`)
✅ **Already using base Diagram component**
- Imports `BaseDiagram` from `../../../../elements/windows/Diagram`
- Only imports types from `@xyflow/react`
- Properly generalized

#### Type Editor (`js/js/sketchpad/editors/type/canvas/Scene.tsx`)
✅ **Already using base Scene component**
- Imports `SceneComponent` from `../../../../elements/windows/Scene`
- Uses base component properly

## Remaining Work

### Design Editor Diagram (`js/js/sketchpad/editors/design/canvas/Diagram.tsx`)

**Status:** ⚠️ Still using `ReactFlow` directly (1943 lines)

This file is highly complex with custom logic for:
- Helper lines for snapping
- Port-to-port connection snapping
- Equal distance guides
- Complex drag-and-drop handling
- Cluster and expand menus
- Viewport management
- Custom node and edge components with diff visualization

**Migration Path:**
The design editor Diagram would benefit from using the base Diagram component, but requires careful refactoring due to its complexity:

1. **Extract Custom Node/Edge Components**: Move `PieceNodeComponent`, `DesignNodeComponent`, and `ConnectionEdgeComponent` to remain custom
2. **Use Controlled Mode**: Leverage the new controlled mode in base Diagram
3. **Keep Custom Overlays**: Helper lines, cluster menu, and expand menu can remain as overlay components
4. **Preserve Event Handlers**: All the complex drag logic can be passed through the new callback props

**Recommendation:** This migration should be done incrementally:
1. First, ensure all props needed by design editor are available in base Diagram (✅ Done)
2. Test the enhanced base Diagram with a simpler use case (✅ Quality editor already uses it)
3. Create a migration branch and carefully move design editor logic piece by piece
4. Extensive testing required due to complexity

## Benefits of Generalization

### For Developers
- **Consistent API**: All diagrams and scenes use the same base components
- **Less Code Duplication**: Common functionality is centralized
- **Easier Debugging**: Issues can be fixed in one place
- **Better Type Safety**: Shared interfaces and props

### For Users
- **Uniform Experience**: Selection, hover, and interaction work the same way everywhere
- **Consistent Styling**: Colors and visual feedback follow the same patterns
- **Predictable Behavior**: Same actions produce same results across editors

## Design Principles Applied

1. **Composition over Inheritance**: Base components accept children and render props
2. **Progressive Enhancement**: Start simple, add complexity only when needed
3. **Controlled/Uncontrolled Patterns**: Support both modes for flexibility
4. **Separation of Concerns**: Visual components separated from business logic
5. **Theme Integration**: All colors use CSS variables for consistent theming

## Future Enhancements

1. **Table Component**: Consider adding similar generalization for table components
2. **Port Component**: Extract port rendering to a generalized component
3. **Connection Line Component**: Standardize connection line rendering
4. **Animation System**: Add consistent animation patterns to base components
5. **Accessibility**: Ensure all base components meet accessibility standards
