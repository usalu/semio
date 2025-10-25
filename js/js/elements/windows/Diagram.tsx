// #region Header

// Diagram.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { BackgroundVariant, Edge, EdgeTypes, MiniMap, Node, NodeTypes, ReactFlow, ReactFlowInstance, ReactFlowProvider, useEdgesState, useNodesState } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import * as dagre from "dagre";
import { FC, ReactNode, RefObject, useCallback, useEffect, useMemo } from "react";

export {
  BaseEdge,
  ConnectionMode,
  Handle,
  Position,
  useReactFlow,
  ViewportPortal,
  type ConnectionLineComponentProps,
  type Edge,
  type EdgeProps,
  type EdgeTypes,
  type MiniMapNodeProps,
  type Node,
  type NodeProps,
  type NodeTypes,
  type Connection as ReactFlowConnection,
  type ReactFlowInstance,
} from "@xyflow/react";

/**
 * Layout direction for dagre
 */
export type DiagramLayoutDirection = "TB" | "BT" | "LR" | "RL";

/**
 * Props for diagram layout calculation
 */
export interface DiagramLayoutOptions {
  direction?: DiagramLayoutDirection;
  nodeWidth?: number;
  nodeHeight?: number;
  rankSep?: number;
  nodeSep?: number;
}

/**
 * Calculate layout for nodes and edges using dagre
 */
export function calculateDiagramLayout(nodes: Node[], edges: Edge[], options: DiagramLayoutOptions = {}): { nodes: Node[]; edges: Edge[] } {
  const { direction = "TB", nodeWidth = 48, nodeHeight = 48, rankSep = 80, nodeSep = 50 } = options;

  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({ rankdir: direction, ranksep: rankSep, nodesep: nodeSep });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  const layoutedNodes = nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    return {
      ...node,
      position: {
        x: nodeWithPosition.x - nodeWidth / 2,
        y: nodeWithPosition.y - nodeHeight / 2,
      },
    };
  });

  return { nodes: layoutedNodes, edges };
}

/**
 * Props for the generalized Diagram component
 */
export interface DiagramProps {
  /** Node types registry */
  nodeTypes: NodeTypes;
  /** Edge types registry */
  edgeTypes?: EdgeTypes;
  /** Initial nodes (for uncontrolled mode) */
  initialNodes?: Node[];
  /** Initial edges (for uncontrolled mode) */
  initialEdges?: Edge[];
  /** Controlled nodes (for controlled mode) */
  nodes?: Node[];
  /** Controlled edges (for controlled mode) */
  edges?: Edge[];
  /** Callback when nodes change (uncontrolled mode) */
  onNodesChange?: (nodes: Node[]) => void;
  /** Callback when edges change (uncontrolled mode) */
  onEdgesChange?: (edges: Edge[]) => void;
  /** Callback for ReactFlow's onNodesChange (controlled mode) */
  onNodesChangeReactFlow?: (changes: any[]) => void;
  /** Callback for ReactFlow's onEdgesChange (controlled mode) */
  onEdgesChangeReactFlow?: (changes: any[]) => void;
  /** Callback when a connection is made */
  onConnect?: (connection: any) => void;
  /** Callback when a node is clicked */
  onNodeClick?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when a node is double-clicked */
  onNodeDoubleClick?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when a node is mouse enter */
  onNodeMouseEnter?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when a node is mouse leave */
  onNodeMouseLeave?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when a node drag starts */
  onNodeDragStart?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when a node is being dragged */
  onNodeDrag?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when a node drag stops */
  onNodeDragStop?: (event: React.MouseEvent, node: Node) => void;
  /** Callback when an edge is clicked */
  onEdgeClick?: (event: React.MouseEvent, edge: Edge) => void;
  /** Callback when an edge is mouse enter */
  onEdgeMouseEnter?: (event: React.MouseEvent, edge: Edge) => void;
  /** Callback when an edge is mouse leave */
  onEdgeMouseLeave?: (event: React.MouseEvent, edge: Edge) => void;
  /** Callback when pane is clicked */
  onPaneClick?: (event: React.MouseEvent) => void;
  /** Callback when pane is double-clicked */
  onPaneDoubleClick?: (event: React.MouseEvent) => void;
  /** Callback when viewport moves */
  onMoveEnd?: () => void;
  /** Ref to access ReactFlow instance */
  reactFlowInstanceRef?: RefObject<ReactFlowInstance | null>;
  /** Ref to forward to the wrapper div (for drag-and-drop zones) */
  wrapperRef?: React.RefObject<HTMLDivElement> | ((node: HTMLDivElement | null) => void);
  /** Show background */
  showBackground?: boolean;
  /** Background variant */
  backgroundVariant?: BackgroundVariant;
  /** Show controls */
  showControls?: boolean;
  /** Show minimap */
  showMinimap?: boolean;
  /** Additional panels to render */
  panels?: ReactNode;
  /** Custom className */
  className?: string;
  /** Fit view on mount */
  fitView?: boolean;
  /** Min zoom level */
  minZoom?: number;
  /** Max zoom level */
  maxZoom?: number;
  /** Default zoom level */
  defaultZoom?: number;
  /** Connection mode */
  connectionMode?: "strict" | "loose";
  /** Connection line component */
  connectionLineComponent?: any;
  /** Delete key code */
  deleteKeyCode?: string | string[];
  /** Enable pan on drag */
  panOnDrag?: boolean | number[];
  /** Enable selection on drag */
  selectionOnDrag?: boolean;
  /** Enable zoom on scroll */
  zoomOnScroll?: boolean;
  /** Enable zoom on pinch */
  zoomOnPinch?: boolean;
  /** Enable zoom on double click */
  zoomOnDoubleClick?: boolean;
  /** Are elements selectable */
  elementsSelectable?: boolean;
  /** Are nodes focusable */
  nodesFocusable?: boolean;
  /** Are edges focusable */
  edgesFocusable?: boolean;
  /** Are nodes draggable */
  nodesDraggable?: boolean;
  /** MiniMap node component */
  miniMapNodeComponent?: any;
}

/**
 * Generalized Diagram component built on ReactFlow
 * Provides common diagram functionality with customizable node and edge types
 * Note: This component is wrapped in ReactFlowProvider automatically
 */
const DiagramInner: FC<DiagramProps> = ({
  nodeTypes,
  edgeTypes,
  initialNodes = [],
  initialEdges = [],
  nodes: controlledNodes,
  edges: controlledEdges,
  onNodesChange: onNodesChangeProp,
  onEdgesChange: onEdgesChangeProp,
  onNodesChangeReactFlow,
  onEdgesChangeReactFlow,
  onConnect,
  onNodeClick,
  onNodeDoubleClick,
  onNodeMouseEnter,
  onNodeMouseLeave,
  onNodeDragStart,
  onNodeDrag,
  onNodeDragStop,
  onEdgeClick,
  onEdgeMouseEnter,
  onEdgeMouseLeave,
  onPaneClick,
  onPaneDoubleClick,
  onMoveEnd,
  reactFlowInstanceRef,
  wrapperRef,
  showMinimap = false,
  panels,
  className = "",
  fitView = true,
  minZoom = 0.1,
  maxZoom = 12,
  connectionMode = "loose",
  connectionLineComponent,
  deleteKeyCode = "Delete",
  panOnDrag = [0],
  selectionOnDrag = false,
  zoomOnScroll = true,
  zoomOnPinch = true,
  zoomOnDoubleClick = false,
  elementsSelectable = false,
  nodesFocusable = false,
  edgesFocusable = false,
  nodesDraggable = true,
  miniMapNodeComponent,
}) => {
  const isControlled = controlledNodes !== undefined && controlledEdges !== undefined;

  const [internalNodes, setInternalNodes, onInternalNodesChange] = useNodesState(initialNodes);
  const [internalEdges, setInternalEdges, onInternalEdgesChange] = useEdgesState(initialEdges);

  const finalNodes = isControlled ? controlledNodes : internalNodes;
  const finalEdges = isControlled ? controlledEdges : internalEdges;
  const finalOnNodesChange = isControlled ? onNodesChangeReactFlow : onInternalNodesChange;
  const finalOnEdgesChange = isControlled ? onEdgesChangeReactFlow : onInternalEdgesChange;

  // Callback to capture ReactFlow instance from onInit
  const handleInit = useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        reactFlowInstanceRef.current = instance;
      }
    },
    [reactFlowInstanceRef],
  );

  // Update nodes and edges when initial values change (uncontrolled mode only)
  useEffect(() => {
    if (!isControlled) {
      setInternalNodes(initialNodes);
      setInternalEdges(initialEdges);
    }
  }, [initialNodes, initialEdges, isControlled, setInternalNodes, setInternalEdges]);

  // Forward changes to parent if callbacks provided (uncontrolled mode)
  useEffect(() => {
    if (!isControlled && onNodesChangeProp) {
      onNodesChangeProp(internalNodes);
    }
  }, [internalNodes, onNodesChangeProp, isControlled]);

  useEffect(() => {
    if (!isControlled && onEdgesChangeProp) {
      onEdgesChangeProp(internalEdges);
    }
  }, [internalEdges, onEdgesChangeProp, isControlled]);

  return (
    <div ref={wrapperRef} className={`relative w-full h-full ${className}`}>
      <ReactFlow
        nodes={finalNodes}
        edges={finalEdges}
        onNodesChange={finalOnNodesChange}
        onEdgesChange={finalOnEdgesChange}
        onConnect={onConnect}
        onInit={handleInit}
        onNodeClick={onNodeClick}
        onNodeDoubleClick={onNodeDoubleClick}
        onNodeMouseEnter={onNodeMouseEnter}
        onNodeMouseLeave={onNodeMouseLeave}
        onNodeDragStart={onNodeDragStart}
        onNodeDrag={onNodeDrag}
        onNodeDragStop={onNodeDragStop}
        onEdgeClick={onEdgeClick}
        onEdgeMouseEnter={onEdgeMouseEnter}
        onEdgeMouseLeave={onEdgeMouseLeave}
        onPaneClick={onPaneClick}
        onDoubleClick={onPaneDoubleClick}
        onMoveEnd={onMoveEnd}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionLineComponent={connectionLineComponent}
        fitView={fitView}
        minZoom={minZoom}
        maxZoom={maxZoom}
        connectionMode={connectionMode as any}
        deleteKeyCode={deleteKeyCode}
        panOnDrag={panOnDrag}
        selectionOnDrag={selectionOnDrag}
        zoomOnScroll={zoomOnScroll}
        zoomOnPinch={zoomOnPinch}
        zoomOnDoubleClick={zoomOnDoubleClick}
        elementsSelectable={elementsSelectable}
        nodesFocusable={nodesFocusable}
        edgesFocusable={edgesFocusable}
        nodesDraggable={nodesDraggable}
        proOptions={{ hideAttribution: true }}
        className="bg-background"
      >
        {showMinimap && <MiniMap className="border border-border" maskColor="var(--accent)" bgColor="var(--background)" nodeStrokeWidth={3} zoomable pannable nodeComponent={miniMapNodeComponent} />}
        {panels}
      </ReactFlow>
    </div>
  );
};

/**
 * Diagram component with ReactFlowProvider wrapper
 */
const Diagram: FC<DiagramProps> = (props) => {
  return (
    <ReactFlowProvider>
      <DiagramInner {...props} />
    </ReactFlowProvider>
  );
};

export default Diagram;

/**
 * Helper hook to create a diagram with automatic layout
 */
export function useDiagramLayout(initialNodes: Node[], initialEdges: Edge[], layoutOptions?: DiagramLayoutOptions): { nodes: Node[]; edges: Edge[] } {
  return useMemo(() => {
    if (initialNodes.length === 0) {
      return { nodes: [], edges: [] };
    }
    return calculateDiagramLayout(initialNodes, initialEdges, layoutOptions);
  }, [initialNodes, initialEdges, layoutOptions]);
}

interface DiagramSkeletonProps {
  nodeCount?: number;
  edgeCount?: number;
  className?: string;
}

export const DiagramSkeleton: FC<DiagramSkeletonProps> = ({ nodeCount = 5, edgeCount = 4, className = "" }) => {
  const skeletonNodes: Node[] = useMemo(
    () =>
      Array.from({ length: nodeCount }).map((_, i) => ({
        id: `skeleton-node-${i}`,
        type: "default",
        position: { x: (i % 3) * 150 + 50, y: Math.floor(i / 3) * 150 + 50 },
        data: { label: "" },
        draggable: false,
      })),
    [nodeCount],
  );
  const skeletonEdges: Edge[] = useMemo(
    () =>
      Array.from({ length: edgeCount }).map((_, i) => ({
        id: `skeleton-edge-${i}`,
        source: `skeleton-node-${i}`,
        target: `skeleton-node-${Math.min(i + 1, nodeCount - 1)}`,
        animated: false,
      })),
    [edgeCount, nodeCount],
  );
  return (
    <div className={`relative w-full h-full ${className}`}>
      <ReactFlow
        nodes={skeletonNodes}
        edges={skeletonEdges}
        nodeTypes={{}}
        edgeTypes={{}}
        nodesDraggable={false}
        nodesConnectable={false}
        elementsSelectable={false}
        panOnDrag={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        proOptions={{ hideAttribution: true }}
        className="bg-background animate-pulse opacity-50"
      ></ReactFlow>
    </div>
  );
};
