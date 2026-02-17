# Move Workbench to Main Container

## Goal
Move the Workbench panel from the left side panel into the main canvas container area, making it appear as a window/tab in the canvas alongside other content like tables, diagrams, and scenes.

## Current Behavior
- Workbench is a LEFT side panel tab
- It appears in the collapsible left panel (sometimes with Tools)
- It's toggled via navbar dropdown toggles
- Position defined in `panelKindConfigs` as `PanelPosition.LEFT`
- Shows hierarchical tree of kit contents (types, designs, folders)

## Desired Behavior
- Workbench should appear as a window in the main canvas area
- It should be managed within the window layout system (like Table, Scene, Diagram)
- Users should navigate to it via routes or open it as a canvas window
- It should NOT appear as a side panel toggle/tab
- It becomes a first-class window that can be resized, moved, and positioned

## Changes Required

### 1. Update Panel Position Mapping
**File**: [semio/js/sketchpad/shared.ts](semio/js/sketchpad/shared.ts)

Change the position for Workbench in `panelKindConfigs`:

```typescript
// Current:
[PanelKind.WORKBENCH]: {
  icon: FolderIcon,
  position: PanelPosition.LEFT,  // ← Remove from left panel
  group: "left",
  isGroupable: true,
  hotkey: "ctrl+b",
},

// Change to: REMOVE THIS ENTRY or set to a new position
// Option A: Remove entirely from panelKindConfigs
// Option B: Create a new WindowKind and integrate into window system
```

### 2. Create Window Kind Definition
**File**: [semio/js/sketchpad/shared.ts](semio/js/sketchpad/shared.ts)

Extend `WindowKind` enum to include Workbench:

```typescript
export enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
  WORKBENCH = "workbench",  // ← Add
}
```

### 3. Update App Configs
**File**: Each app file (Home.tsx, Kit.tsx, Type.tsx, Design.tsx, etc.)

Remove Workbench from `getPanels()` returns:

```typescript
// Current (e.g., in Type.tsx):
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.WORKBENCH, "..."),  // ← REMOVE
  createPanelDefinition(PanelKind.TOOLS, "..."),
  createPanelDefinition(PanelKind.TOOLBAR, "..."),
  createPanelDefinition(PanelKind.DETAILS, "..."),
  // ...
],

// Change to:
getPanels: (): PanelDefinition[] => [
  createPanelDefinition(PanelKind.TOOLS, "..."),     // Keep if needed
  createPanelDefinition(PanelKind.TOOLBAR, "..."),
  createPanelDefinition(PanelKind.DETAILS, "..."),
  // Workbench removed - now handled as a window
],
```

### 4. Add Window Kind Config
**File**: Each app file or shared window registry

Add Workbench to window definitions:

```typescript
getWindows: (): WindowKindDefinition[] => [
  {
    kind: WindowKind.TABLE,
    icon: TableIcon,
    label: "semio.sketchpad.window.table",
    canCreate: true,
  },
  {
    kind: WindowKind.WORKBENCH,  // ← Add
    icon: FolderIcon,
    label: "semio.sketchpad.window.workbench",
    canCreate: true,
    singleton: true,  // Only one workbench window at a time
    defaultSize: { width: 300, height: "100%" },  // Default dimensions
  },
],
```

### 5. Create Workbench Window Component
**File**: [semio/js/sketchpad/Workbench.tsx](semio/js/sketchpad/Workbench.tsx) (new file or extract from existing)

```typescript
export const WorkbenchWindow: FC<{ windowId: string; kitGuid?: string }> = ({ 
  windowId, 
  kitGuid 
}) => {
  const kit = useKit(kitGuid);
  const workbenchSections = usePanelSections("workbench");
  
  // Move content from current Workbench panel here
  return (
    <Window 
      id={windowId}
      showControls={true}
      onClose={() => {
        // Handle window close
        appCommands.closeWindow(windowId);
      }}
    >
      <div className="h-full flex flex-col">
        {/* Workbench tree content that was previously in left panel */}
        <Scrollable className="flex-1">
          <Tree>
            {workbenchSections.map((section) => (
              <PanelTabSectionItem 
                key={section.id} 
                section={section} 
                defaultOpen={section.defaultOpen}
              />
            ))}
          </Tree>
        </Scrollable>
      </div>
    </Window>
  );
};

// For apps that need multiple workbenches (e.g., comparing kits)
export const MultiWorkbenchWindow: FC<{ windowId: string }> = ({ windowId }) => {
  const kits = useKits();
  const [selectedKitIndex, setSelectedKitIndex] = useState(0);
  
  return (
    <Window id={windowId} showControls={true}>
      <div className="h-full flex flex-col">
        {/* Kit selector */}
        <div className="border-b p-single">
          <Select
            value={kits[selectedKitIndex]?.guid}
            onValueChange={(guid) => {
              const index = kits.findIndex(k => k.guid === guid);
              if (index >= 0) setSelectedKitIndex(index);
            }}
          >
            {kits.map(kit => (
              <SelectItem key={kit.guid} value={kit.guid}>
                {kit.name}
              </SelectItem>
            ))}
          </Select>
        </div>
        
        {/* Workbench content for selected kit */}
        <Scrollable className="flex-1">
          <WorkbenchContent kitGuid={kits[selectedKitIndex]?.guid} />
        </Scrollable>
      </div>
    </Window>
  );
};
```

### 6. Register Window Renderer
**File**: [semio/js/sketchpad/Sketchpad.tsx](semio/js/sketchpad/Sketchpad.tsx)

Add to window rendering logic:

```typescript
const renderWindow = (windowConfig: AppWindowConfig) => {
  switch (windowConfig.kind) {
    case WindowKind.TABLE:
      return <TableWindow {...windowConfig} />;
    case WindowKind.SCENE:
      return <SceneWindow {...windowConfig} />;
    case WindowKind.DIAGRAM:
      return <DiagramWindow {...windowConfig} />;
    case WindowKind.WORKBENCH:  // ← Add
      return (
        <WorkbenchWindow 
          windowId={windowConfig.id}
          kitGuid={windowConfig.kitGuid}  // Pass kit context
        />
      );
    default:
      return <div>Unknown window</div>;
  }
};
```

### 7. Update Navigation/Access
**File**: Navbar or Footer components

Add button/menu item to open Workbench as a window:

```typescript
// In Navbar actions or Footer items:
{
  id: "semio.sketchpad.navbar.workbench",
  icon: <FolderIcon />,
  onClick: () => {
    // Open workbench window in canvas
    appCommands.openWindow("semio.sketchpad.navbar.workbench", {
      kind: WindowKind.WORKBENCH,
      id: "workbench-main",
      kitGuid: currentKit?.guid,  // Context-aware
      // ... window config
    });
  }
}
```

### 8. Handle Existing Panel Content
**File**: Current panel section providers

The content currently rendered in Workbench panel sections needs to be:
- Extracted from panel section hooks (usePanelSections)
- Moved to standalone components
- Integrated into the new Window component

Example migration:
```typescript
// OLD: Panel section in LayoutWrapper
const workbenchSections = usePanelSections("workbench");
// Rendered in left side panel

// NEW: Direct component render in canvas window
<WorkbenchWindow>
  <Tree>
    <TreeSection label="Types">
      {/* Type tree items */}
    </TreeSection>
    <TreeSection label="Designs">
      {/* Design tree items */}
    </TreeSection>
    <TreeSection label="Folders">
      {/* Folder tree items */}
    </TreeSection>
  </Tree>
</WorkbenchWindow>
```

### 9. Handle Context-Specific Workbenches
**File**: App-specific implementations

Different apps may need different workbench behavior:

#### Home App
```typescript
// Home app might show all kits
<WorkbenchWindow>
  <Tree>
    {kits.map(kit => (
      <TreeSection key={kit.guid} label={kit.name}>
        <TreeItem label="Types" onClick={() => navigate(`/kits/${kit.guid}/types`)} />
        <TreeItem label="Designs" onClick={() => navigate(`/kits/${kit.guid}/designs`)} />
      </TreeSection>
    ))}
  </Tree>
</WorkbenchWindow>
```

#### Kit App
```typescript
// Kit app focuses on current kit structure
<WorkbenchWindow kitGuid={currentKitGuid}>
  <Tree>
    <TreeSection label="Types">
      {types.map(type => (
        <TreeItem 
          key={type.guid}
          label={type.name}
          onClick={() => navigate(`/kits/${kitGuid}/types/${type.guid}`)}
        />
      ))}
    </TreeSection>
    <TreeSection label="Designs">
      {/* Design items */}
    </TreeSection>
  </Tree>
</WorkbenchWindow>
```

#### Type/Design Apps
```typescript
// Type/Design apps show workbench with drag-drop functionality
<WorkbenchWindow>
  <Tree>
    <TreeSection label="Available Types">
      {types.map(type => (
        <DraggableTreeItem
          key={type.guid}
          data={{ type: "type", typeGuid: type.guid }}
          label={type.name}
        />
      ))}
    </TreeSection>
  </Tree>
</WorkbenchWindow>
```

### 10. Update Panel Visibility Interface (Optional)
**File**: [semio/js/sketchpad/shared.ts](semio/js/sketchpad/shared.ts)

Consider removing workbench from PanelVisibility if it's no longer a panel:

```typescript
export interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  hudPanel?: boolean;
  // workbench?: boolean;  // ← Remove or deprecate
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  chat?: boolean;
  settings?: boolean;
  params?: boolean;
  console?: boolean;
}
```

## Implementation Steps

1. **Remove from panels**:
   - Remove WORKBENCH from `panelKindConfigs`
   - Remove from all app `getPanels()` configurations
   
2. **Add as window**:
   - Add WORKBENCH to `WindowKind` enum
   - Create WorkbenchWindow component
   - Register in window rendering switch
   
3. **Migrate content**:
   - Extract workbench section content
   - Create standalone WorkbenchContent component
   - Update panel section providers
   
4. **Update navigation**:
   - Add button to open workbench window
   - Consider default layout with workbench window open
   - Add keyboard shortcut (Ctrl+B) to toggle workbench window
   
5. **Handle context**:
   - Pass kit context to workbench window
   - Handle app-specific workbench variations
   - Maintain drag-drop functionality from workbench to canvas
   
6. **Update layout defaults**:
   - Include workbench window in default layouts
   - Consider pinned/docked behavior option
   - Allow multiple workbench windows if needed
   
7. **Test**:
   - Verify Workbench no longer appears in left panel
   - Verify it opens as canvas window
   - Test window management (resize, close, maximize)
   - Test drag-drop from workbench to diagram/scene
   - Test persistence in layout state

## Testing Checklist

- [ ] Workbench button removed from left panel toggle/tabs
- [ ] Workbench opens as window in canvas (not side panel)
- [ ] Workbench window can be resized, moved, closed
- [ ] Workbench window shows correct kit content
- [ ] Drag-drop from workbench to diagram still works
- [ ] Drag-drop from workbench to scene still works
- [ ] Workbench content updates when kit changes
- [ ] Window state persists in layout
- [ ] All apps (Home, Kit, Type, Design) work correctly
- [ ] Left panel still shows Tools (if configured)
- [ ] Keyboard shortcut (Ctrl+B) toggles workbench window
- [ ] Multiple workbenches can be opened (if needed)
- [ ] No errors in console
- [ ] Existing tests updated to reflect new behavior

## Special Considerations

### Drag-and-Drop Interaction
Workbench is heavily used for drag-drop operations:
- **From Workbench**: Dragging types/designs to diagram/scene
- **To Canvas**: Dropping creates new pieces

Ensure window positioning doesn't interfere:
```typescript
// Consider default layout with workbench on the left
defaultLayout: {
  root: {
    type: "row",
    content: [
      // Workbench window on left
      {
        type: "component",
        componentName: "workbench",
        width: 25,  // 25% width
      },
      // Other windows on right
      {
        type: "stack",
        content: [/* diagram, scene, etc */],
        width: 75,
      }
    ]
  }
}
```

### Context Awareness
Workbench content should reflect current context:
- **Home**: All kits
- **Kit**: Current kit structure
- **Type**: Types available for editing
- **Design**: Types/designs available for use

```typescript
// Context-aware workbench content
const WorkbenchWindow: FC = () => {
  const appType = useAppType();
  const currentKit = useCurrentKit();
  
  switch (appType) {
    case "home":
      return <AllKitsWorkbench />;
    case "kit":
      return <KitStructureWorkbench kitGuid={currentKit?.guid} />;
    case "type":
      return <TypesWorkbench kitGuid={currentKit?.guid} />;
    case "design":
      return <DesignsWorkbench kitGuid={currentKit?.guid} />;
    default:
      return <DefaultWorkbench />;
  }
};
```

### Performance Considerations
Workbench can contain large trees:
- Implement virtualization for long lists
- Lazy load folder contents
- Cache tree state

```typescript
// Virtualized tree for performance
import { FixedSizeList } from "react-window";

<FixedSizeList
  height={600}
  itemCount={items.length}
  itemSize={30}
>
  {({ index, style }) => (
    <div style={style}>
      <TreeItem {...items[index]} />
    </div>
  )}
</FixedSizeList>
```

## Alternative Approaches

### Option A: Floating/Docked Workbench
Keep workbench-like behavior but as a special window:
- Can be docked to left edge (like VS Code explorer)
- Can be undocked and moved freely
- Resizable but maintains tree structure

```typescript
interface WorkbenchWindowProps {
  docked?: boolean;
  dockSide?: "left" | "right";
  onDockChange?: (docked: boolean) => void;
}
```

### Option B: Hybrid Panel-Window
Allow workbench in both modes:
- Default: Left panel mode
- User can "pop out" to window mode
- User preference saved

```typescript
const WorkbenchContainer = () => {
  const [mode, setMode] = useLocalStorage("workbench-mode", "panel");
  
  if (mode === "panel") {
    return <WorkbenchPanel />;  // Traditional left panel
  } else {
    return <WorkbenchWindow />;  // Canvas window
  }
};
```

### Option C: Split Workbench
Different workbench types:
- **Explorer**: File tree, stays in panel
- **Workspace**: Active items, moves to canvas
- Separation of concerns

### Option D: Sidebar System
Create a new sidebar concept:
- Not a panel, not a window
- Fixed-width column on left/right
- Can contain workbench, outline, search
- Similar to VS Code Activity Bar

## Edge Cases to Handle

1. **Empty Kit**: Workbench with no content
2. **Large Kits**: Performance with 1000+ items
3. **Multiple Windows**: Multiple workbenches for different kits
4. **Split View**: Workbench + diagram side-by-side
5. **Mobile/Tablet**: Workbench behavior on small screens
6. **Search/Filter**: Finding items in large workbenches

## Migration Path

### Phase 1: Create Window Component (Non-Breaking)
- Add WORKBENCH to WindowKind
- Create WorkbenchWindow component
- Register renderer
- Don't remove from panels yet
- Allow opening as window via button

### Phase 2: Default to Window (Breaking)
- Remove from panelKindConfigs
- Remove from getPanels()
- Update default layouts
- Migration guide for users

### Phase 3: Cleanup (Breaking)
- Remove workbench from PanelVisibility
- Clean up old panel-related code
- Update documentation

## References

- Current panel system: [PANEL_TOGGLE_ARCHITECTURE.md](PANEL_TOGGLE_ARCHITECTURE.md)
- Window system: [semio/js/sketchpad/Sketchpad.tsx](semio/js/sketchpad/Sketchpad.tsx) (Canvas section)
- Panel configs: [semio/js/sketchpad/shared.ts](semio/js/sketchpad/shared.ts) (panelKindConfigs)
- Workbench sections: Used in Type.tsx, Design.tsx, Kit.tsx
- Drag-drop: [semio/js/sketchpad/Sketchpad.tsx](semio/js/sketchpad/Sketchpad.tsx) (DndContext)

## Notes

- Workbench is more complex than Settings/Chat due to drag-drop
- Consider keeping TOOLS in left panel (smaller, less intrusive)
- Workbench as window enables multiple workbenches (multi-kit workflows)
- Default layouts should include workbench for good UX
- Consider "pin" behavior: workbench always visible on left
- Tree state (expanded/collapsed) should persist per window
- Search/filter in workbench becomes more important as a window
