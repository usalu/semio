# Previously

The Kit app was extended to a multi-window layout (KIT-MULTI-WINDOW) with table and diagram views. The current diagram implementation uses ReactFlow with a static row-based layout:

```typescript
// Current layout in buildKitDiagramData (Kit.tsx:5204-5327)
for (const kind of kindGroups) {
  let x = 0;
  for (const item of items) {
    const position = { x, y: globalY };
    // Nodes positioned in grid: x += 180, y += 100 per kind group
  }
}
```

Key characteristics:

- Nodes are rectangular boxes with icons and text labels
- Static positioning (x/y grid layout by kind)
- No physics simulation or interactivity
- ReactFlow handles pan/zoom/selection

# Plan

## 1. Dependencies

Add d3-force to `js/semio/package.json`:

```json
"d3-force": "^3.0.0",
"@types/d3-force": "^3.0.10"
```

## 2. d3-force + ReactFlow Integration Strategy

### 2.1 Integration Pattern

d3-force provides simulation, ReactFlow provides rendering. The pattern:

```
┌─────────────────────────────────────────────────────────────┐
│                       KitDiagram                            │
│  ┌──────────────────┐    ┌─────────────────────────────┐   │
│  │  d3-force        │    │  ReactFlow                  │   │
│  │  ───────────────│    │  ─────────────────────────  │   │
│  │  forceSimulation │───>│  nodes (position updates)   │   │
│  │  forceManyBody   │    │  edges                      │   │
│  │  forceLink       │    │  nodeTypes                  │   │
│  │  forceCenter     │    │  pan/zoom/selection         │   │
│  │  forceCollide    │    │                             │   │
│  └──────────────────┘    └─────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Simulation Lifecycle

1. **Initialize**: Create `forceSimulation` with nodes when kit changes
2. **Tick**: On each tick, update ReactFlow node positions via `setNodes()`
3. **Drag**: When user drags node, update simulation's node.fx/fy (fixed position)
4. **Reheat**: When settings change, call `simulation.alpha(1).restart()`

### 2.3 d3-force API

```typescript
import {
  forceSimulation, // Main simulation
  forceManyBody, // Node repulsion (chargeStrength)
  forceLink, // Edge spring force (linkDistance)
  forceCenter, // Pull to center (centerStrength)
  forceCollide, // Prevent overlap (collideRadius)
  SimulationNodeDatum,
  SimulationLinkDatum,
} from "d3-force";

// Node type for simulation
interface ForceNode extends SimulationNodeDatum {
  id: string;
  x?: number;
  y?: number;
  fx?: number | null; // Fixed x (when dragging)
  fy?: number | null; // Fixed y (when dragging)
  kind: DiagramNodeKind;
  // ... other KitDiagramNode fields
}

// Link type for simulation
interface ForceLink extends SimulationLinkDatum<ForceNode> {
  source: string | ForceNode;
  target: string | ForceNode;
  relationship: "part-of" | "reference";
}
```

## 3. State Changes

### 3.1 KitAppState Extension

In `js/semio/sketchpad/Kit.tsx` (line ~259):

```typescript
export interface DiagramForceSettings {
  chargeStrength: number; // Node repulsion (-500 to 0), default -150
  linkDistance: number; // Edge length (50 to 300), default 100
  collideRadius: number; // Collision radius (10 to 100), default 30
  centerStrength: number; // Center pull (0 to 1), default 0.05
}

export interface KitAppState {
  // ... existing fields
  diagramForce: DiagramForceSettings;
}
```

### 3.2 KitAppDiff Extension

```typescript
export interface KitAppDiff {
  // ... existing fields
  diagramForce?: Partial<DiagramForceSettings>;
}
```

### 3.3 Default State

In `js/semio/sketchpad/Sketchpad.tsx` (createDefaultKitAppState ~8119):

```typescript
export function createDefaultKitAppState(): KitAppState {
  return {
    // ... existing fields
    diagramForce: {
      chargeStrength: -150,
      linkDistance: 100,
      collideRadius: 30,
      centerStrength: 0.05,
    },
  };
}
```

## 4. XState Actions

Add to `js/semio/sketchpad/Kit.tsx` (after line ~1030):

```typescript
registerRuntimeAction("kitSetDiagramForce", (context: any, event: any) => {
  if (event.type !== "KIT.SET_DIAGRAM_FORCE") return {};
  const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
  return {
    kitApps: {
      ...context.kitApps,
      [event.kitGuid]: {
        ...app,
        diagramForce: { ...app.diagramForce, ...event.settings },
      },
    },
  };
});
```

## 5. Hook for Force Settings

Add after existing hooks (~line 1100):

```typescript
export function useKitAppDiagramForce(): HookResult<DiagramForceSettings> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const state = useKitApp(identitySelector);
  const force = (state as KitAppState).diagramForce ?? defaultDiagramForce;

  const setForce = useMemo(() => {
    return (settings: Partial<DiagramForceSettings>) => {
      actor.send({
        type: "KIT.SET_DIAGRAM_FORCE",
        kitGuid,
        settings,
      });
    };
  }, [actor, kitGuid]);

  return [force, setForce, true];
}
```

## 6. Node Design Changes

### 6.1 Circular Node Component

Replace `KitArtifactNode` (line ~5069-5193):

```typescript
const KitArtifactNode: FC<NodeProps<Node<KitDiagramNode>>> = ({ data, selected }) => {
  // ... existing hooks

  return (
    <div
      className={cn(
        // Circle: 40x40px with centered icon
        "size-[40px] rounded-full flex items-center justify-center cursor-pointer",
        "border-2 transition-colors",
        isSelected
          ? "bg-active-base border-foreground"
          : "bg-base border-foreground/20 hover:bg-hover-base"
      )}
      onClick={handleClick}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Handles positioned around circle */}
      <Handle type="target" position={Position.Top} className="!bg-foreground/50 !w-2 !h-2 !top-0" />
      <Handle type="source" position={Position.Bottom} className="!bg-foreground/50 !w-2 !h-2 !bottom-0" />
      <Handle type="target" position={Position.Left} className="!bg-foreground/50 !w-2 !h-2 !left-0" />
      <Handle type="source" position={Position.Right} className="!bg-foreground/50 !w-2 !h-2 !right-0" />

      {/* Centered icon */}
      {iconContent}
    </div>
  );
};
```

### 6.2 Node Name Display

Show name in tooltip on hover instead of inline:

```typescript
<Tooltip>
  <TooltipTrigger asChild>
    <div className="...">
      {iconContent}
    </div>
  </TooltipTrigger>
  <TooltipContent>
    <span>{data.name || data.guid.substring(0, 8)}</span>
  </TooltipContent>
</Tooltip>
```

## 7. KitDiagram Component Rewrite

Replace `KitDiagram` (line ~5331-5365):

```typescript
const KitDiagram: FC<KitDiagramProps> = () => {
  const kit = useKit() as Kit | undefined;
  const kitCommands = useKitAppCommands();
  const [forceSettings] = useKitAppDiagramForce();
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const simulationRef = useRef<d3.Simulation<ForceNode, ForceLink> | null>(null);

  // Convert kit to force nodes/links
  const { forceNodes, forceLinks } = useMemo(() => {
    if (!kit) return { forceNodes: [], forceLinks: [] };
    return buildForceGraphData(kit);
  }, [kit]);

  // ReactFlow state
  const [nodes, setNodes] = useNodesState([]);
  const [edges, setEdges] = useEdgesState([]);

  // Initialize simulation
  useEffect(() => {
    if (forceNodes.length === 0) return;

    const simulation = forceSimulation<ForceNode>(forceNodes)
      .force("charge", forceManyBody().strength(forceSettings.chargeStrength))
      .force(
        "link",
        forceLink<ForceNode, ForceLink>(forceLinks)
          .id((d) => d.id)
          .distance(forceSettings.linkDistance),
      )
      .force("center", forceCenter(0, 0).strength(forceSettings.centerStrength))
      .force("collide", forceCollide<ForceNode>(forceSettings.collideRadius));

    simulation.on("tick", () => {
      setNodes((nodes) =>
        nodes.map((node) => {
          const simNode = simulation.nodes().find((n) => n.id === node.id);
          if (simNode) {
            return { ...node, position: { x: simNode.x ?? 0, y: simNode.y ?? 0 } };
          }
          return node;
        }),
      );
    });

    simulationRef.current = simulation;

    return () => {
      simulation.stop();
    };
  }, [forceNodes, forceLinks]);

  // Update forces when settings change
  useEffect(() => {
    const sim = simulationRef.current;
    if (!sim) return;

    sim.force("charge", forceManyBody().strength(forceSettings.chargeStrength));
    sim.force(
      "link",
      forceLink<ForceNode, ForceLink>(forceLinks)
        .id((d) => d.id)
        .distance(forceSettings.linkDistance),
    );
    sim.force("center", forceCenter(0, 0).strength(forceSettings.centerStrength));
    sim.force("collide", forceCollide<ForceNode>(forceSettings.collideRadius));
    sim.alpha(0.3).restart();
  }, [forceSettings, forceLinks]);

  // Node drag handlers
  const onNodeDragStart = useCallback((event: any, node: Node) => {
    const sim = simulationRef.current;
    if (sim) {
      sim.alphaTarget(0.3).restart();
      const simNode = sim.nodes().find((n) => n.id === node.id);
      if (simNode) {
        simNode.fx = node.position.x;
        simNode.fy = node.position.y;
      }
    }
  }, []);

  const onNodeDrag = useCallback((event: any, node: Node) => {
    const sim = simulationRef.current;
    if (sim) {
      const simNode = sim.nodes().find((n) => n.id === node.id);
      if (simNode) {
        simNode.fx = node.position.x;
        simNode.fy = node.position.y;
      }
    }
  }, []);

  const onNodeDragStop = useCallback((event: any, node: Node) => {
    const sim = simulationRef.current;
    if (sim) {
      sim.alphaTarget(0);
      const simNode = sim.nodes().find((n) => n.id === node.id);
      if (simNode) {
        simNode.fx = null;
        simNode.fy = null;
      }
    }
  }, []);

  // ... render ReactFlow with nodes, edges, handlers
};
```

## 8. Settings UI

Add to `KitSettingsContent` (line ~6177):

```typescript
const KitSettingsContent: FC = () => {
  // ... existing hooks
  const [diagramForce, setDiagramForce] = useKitAppDiagramForce();

  return (
    <>
      {/* ... existing settings */}

      {/* Force Diagram Settings */}
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.kitApp.settings.diagram.chargeStrength"
            showLabel
            min={-500}
            max={0}
            value={[diagramForce.chargeStrength]}
            onValueChange={(values) => setDiagramForce({ chargeStrength: values[0] })}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.kitApp.settings.diagram.linkDistance"
            showLabel
            min={50}
            max={300}
            value={[diagramForce.linkDistance]}
            onValueChange={(values) => setDiagramForce({ linkDistance: values[0] })}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.kitApp.settings.diagram.collideRadius"
            showLabel
            min={10}
            max={100}
            value={[diagramForce.collideRadius]}
            onValueChange={(values) => setDiagramForce({ collideRadius: values[0] })}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.kitApp.settings.diagram.centerStrength"
            showLabel
            min={0}
            max={1}
            step={0.01}
            value={[diagramForce.centerStrength]}
            onValueChange={(values) => setDiagramForce({ centerStrength: values[0] })}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};
```

## 9. i18n Labels

Add to `js/semio/sketchpad/locales/en.json`:

```json
"semio.kitApp.settings.diagram.chargeStrength": "Charge Strength",
"semio.kitApp.settings.diagram.chargeStrength.description": "Node repulsion force (-500 strong repulsion, 0 none)",
"semio.kitApp.settings.diagram.linkDistance": "Link Distance",
"semio.kitApp.settings.diagram.linkDistance.description": "Target distance between connected nodes",
"semio.kitApp.settings.diagram.collideRadius": "Collide Radius",
"semio.kitApp.settings.diagram.collideRadius.description": "Minimum distance between nodes",
"semio.kitApp.settings.diagram.centerStrength": "Center Strength",
"semio.kitApp.settings.diagram.centerStrength.description": "Force pulling nodes towards center"
```

Add corresponding entries to `de.json`.

## 10. File Changes Summary

| File                                 | Changes                                                                                                                                                                                        |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `js/semio/package.json`              | Add d3-force, @types/d3-force                                                                                                                                                                  |
| `js/semio/sketchpad/Kit.tsx`         | KitAppState, DiagramForceSettings interface, registerRuntimeAction, useKitAppDiagramForce hook, KitArtifactNode circular redesign, KitDiagram d3-force integration, KitSettingsContent sliders |
| `js/semio/sketchpad/Sketchpad.tsx`   | createDefaultKitAppState add diagramForce defaults                                                                                                                                             |
| `js/semio/sketchpad/locales/en.json` | Add diagram force labels                                                                                                                                                                       |
| `js/semio/sketchpad/locales/de.json` | Add diagram force labels                                                                                                                                                                       |
| `AGENTS.md`                          | Document diagram force settings                                                                                                                                                                |

## 11. Implementation Order

1. Add dependencies (`npm install d3-force @types/d3-force`)
2. Define types (DiagramForceSettings, ForceNode, ForceLink)
3. Extend KitAppState and KitAppDiff
4. Update createDefaultKitAppState
5. Add XState action (kitSetDiagramForce)
6. Add hook (useKitAppDiagramForce)
7. Rewrite buildKitDiagramData → buildForceGraphData
8. Update KitArtifactNode to circular
9. Rewrite KitDiagram with simulation
10. Add settings UI
11. Add i18n labels
12. Update AGENTS.md

# Changes
