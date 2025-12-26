---
slug: DIAGRAM-GENERALIZATION
summary: Generalize diagram component across apps
prompt: >-
  Implement generalized diagram component and migrate Design/Kit/Quality; ensure
  only elements.tsx imports @xyflow/react; ensure sketchpad tests pass.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-18T15:09:08.033Z"
iterations:
  - prompt: >-
      Implement generalized diagram component and migrate Design/Kit/Quality;
      ensure only elements.tsx imports @xyflow/react; ensure sketchpad tests
      pass.
    date:
      started: "2025-12-18T15:32:19.612Z"
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    files:
      updated:
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 111
              removed: 121
        - js/js/sketchpad/Design.tsx:
            lines:
              added: 10
              removed: 5
        - js/js/sketchpad/Kit.tsx:
            lines:
              added: 2
              removed: 6
        - js/js/sketchpad/Quality.tsx:
            lines:
              added: 0
              removed: 0
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 50
              removed: 3
        - README.md:
            lines:
              added: 11
              removed: 8
        - AGENTS.md:
            lines:
              added: 14
              removed: 4
        - log/tickets/2025/12/18/DIAGRAM-GENERALIZATION.md:
            lines:
              added: 124
              removed: 0
      created: []
      removed: []
    lines:
      added: 322
      removed: 147
---

# Previously

The sketchpad currently uses multiple diagram implementations across apps.

`@xyflow/react` is imported directly by multiple app files (Design/Kit/Quality/Sketchpad) and also by `elements.tsx`.

# Plan

- Define the generalized diagram contract (Semio-first).
  - Diagram coords are unit-based and independent from pixels.
  - 1 diagram unit equals the diameter of circular nodes.
  - All interaction callbacks export semio coordinates (u/v) and semio identifiers.
  - Nodes support circle+icon and square+label.
  - Handles are edge dots positioned by t in [0..1) where t=0/1 is 12 o'clock and increases clockwise.
- Implement the generalized diagram renderer in `js/js/sketchpad/elements.tsx`.
  - Keep `@xyflow/react` imports in this file only.
  - Add unit conversion helpers (diagram units <-> internal flow coords).
  - Add an imperative API ref (screen->diagram position, get nodes, fit view) without exposing `ReactFlowInstance`.
  - Provide overlay rendering support for diagram-attached UI (viewport overlay/portal).
- Remove all `@xyflow/react` imports outside `elements.tsx`.
  - `Design.tsx`: replace direct reactflow use with the generalized diagram API and semio callbacks.
  - `Kit.tsx`: replace `<ReactFlow>` usage with `<Diagram>` and convert node+edge types to generalized node spec.
  - `Quality.tsx`: replace `ReactFlowInstance` usage with the generalized diagram API ref; remove remaining reactflow types.
  - `Sketchpad.tsx`: remove `ReactFlowProvider` import and rely on `elements.tsx` exports.
- Migrate each app.
  - `Design.tsx` migration
    - Replace node drag updates with semio `onNodeDrag` events in u/v.
    - Replace handle definitions with `t`-based handles (connector `t` mapping).
    - Replace reactflow portals with diagram overlay support from `elements.tsx`.
    - Preserve helper lines, clustering overlays, and presence cursor rendering.
  - `Kit.tsx` migration
    - Keep d3-force simulation but run in diagram units.
    - Preserve drag pinning behavior using semio `onNodeDragStart/onNodeDrag/onNodeDragEnd` callbacks.
    - Move floating-edge boundary intersection math to `elements.tsx`.
  - `Quality.tsx` migration
    - Update drag/drop placement to use `screenToDiagramPosition` and unit-based hit testing.
    - Keep layout but express sizes as units (internal pixel mapping done in `elements.tsx`).
- Update developer documentation.
  - `README.md`
    - Products: diagram behavior described in user terms (units, nodes, handles, interactions).
    - Components: diagram mechanism described for junior devs.
  - `AGENTS.md`
    - SRS UI/UX: diagram coordinate system + interaction requirements.
    - Codebase: sketchpad diagram architecture and file boundaries (`elements.tsx` sole reactflow importer).
- Acceptance criteria.
  - No file except `js/js/sketchpad/elements.tsx` imports `@xyflow/react`.
  - All diagram callbacks emit semio coords (u/v) and semio identifiers.
  - Circle and square nodes render correctly with `t`-based handles.
  - Design/Kit/Quality diagrams retain existing interaction features (drag, select, hover, connect, overlays).

- Migration inventory (what exactly must move).
  - `Design.tsx`
    - ReactFlow imports to remove.
      - `ConnectionLineComponentProps`, `Edge`, `EdgeProps`, `EdgeTypes`, `MiniMapNodeProps`, `Node`, `NodeProps`, `NodeTypes`, `Connection as RFConnection`.
      - `applyNodeChanges`, `BaseEdge`, `Handle`, `Position`, `ReactFlowInstance`, `ReactFlowProvider`, `useReactFlow`, `ViewportPortal`.
    - ReactFlow-dependent components.
      - Node renderers.
        - `PieceNodeComponent` and `DesignNodeComponent` (reactflow `NodeProps` usage).
        - `DesignNodeInner` renders connectors via `ConnectorHandle` and uses reactflow `Handle` ids for connector lookup.
      - Edge renderers.
        - `ConnectionEdgeComponent` / `ConnectionEdgeInner` / `ConnectionEdgeFallback` (reactflow `EdgeProps`, `BaseEdge`).
        - `ConnectionConnectionLine` (reactflow `ConnectionLineComponentProps`).
      - Overlay rendering.
        - `ClusterMenu`, `ExpandMenu`, `PresenceDiagram` use `ViewportPortal`.
        - `HelperLines` uses `useReactFlow().getViewport()`.
    - Imperative instance usage to remove.
      - `reactFlowInstanceRef.current.setViewport(...)` (restore/persist viewport).
      - `reactFlowInstanceRef.current.getViewport()` (persist center/scale).
      - `reactFlowInstanceRef.current.setNodes(...)` (escape abort refresh).
      - `reactFlowInstanceRef.current.getInternalNode(...)` and `internals.handleBounds` (drag-time connector proximity and connect resolution).
    - DOM coupling to reactflow class names to remove.
      - `querySelector(.react-flow__nodes/.react-flow__edges/.react-flow__viewport)` for pointerEvents and transform parsing.
    - Coordinate conversion currently embedded in app.
      - `ICON_WIDTH` maps between semio u/v and reactflow x/y.
      - Drop placement derives u/v by parsing viewport transform.
    - App-owned logic that stays in `Design.tsx`.
      - Selection logic (`onNodeClick`, `onEdgeClick`, `onPaneClick`).
      - Navigation (`onNodeDoubleClick` routes to type/design).
      - Hover throttling and delayed clear.
      - Snap + helper line computation (but should move to diagram units).
      - semio diff/transaction updates (`addPiece`, `updatePieces`, `addConnection`, `updateConnections`, etc).
  - `Kit.tsx`
    - ReactFlow imports to remove.
      - `ConnectionLineComponentProps`, `Edge`, `EdgeProps`, `Node`, `NodeProps`.
      - `Background`, `BaseEdge`, `getBezierPath`, `Handle`, `Position`, `ReactFlow`, `ReactFlowProvider`, `useInternalNode`, `useReactFlow`.
    - ReactFlow-dependent components.
      - `KitArtifactNode` (reactflow `NodeProps` + `Handle` placement).
      - `FloatingEdge` (uses `useInternalNode` + `BaseEdge` + bezier path).
      - `FloatingConnectionLine` (uses bezier path).
    - Data builders that are reactflow-shaped.
      - `buildKitDiagramData` currently returns `Node<KitDiagramNode>[]` and `Edge[]`.
    - ReactFlow viewport control.
      - `useReactFlow().fitView(...)` on simulation end.
    - App-owned logic that stays in `Kit.tsx`.
      - Artifact filtering/visibility derived from table state.
      - d3-force simulation orchestration (nodes/links, alpha target, pin/unpin on drag).
      - Selection/hover dispatch via sketchpad actor.
  - `Quality.tsx`
    - ReactFlow imports to remove.
      - `Connection`, `Edge`, `Node`, `NodeTypes`, `ReactFlowInstance`.
    - ReactFlow-dependent config.
      - `nodeTypes: NodeTypes` mapping.
      - Dagre layout currently uses pixel sizes (`nodeWidth: 48`, `nodeHeight: 48`).
    - Imperative instance usage to remove.
      - `reactFlowInstanceRef.current.screenToFlowPosition(...)` for drop placement.
      - `reactFlowInstanceRef.current.getNodes()` + pixel bounds checks (48x48) for placeholder hit testing.
    - App-owned logic that stays in `Quality.tsx`.
      - Formula node graph build (including placeholder nodes/edges).
      - Connect semantics (`connectNodes(parentId, nodeId)`).
      - Workbench drag sources and transaction boundaries.

# Changes

- Created ticket and collected current diagram usage and `@xyflow/react` import sites.
