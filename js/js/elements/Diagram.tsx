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

import { ReactFlow, Background, Controls, MiniMap, Edge, Node, NodeTypes, EdgeTypes, useNodesState, useEdgesState, ReactFlowInstance, BackgroundVariant, Panel, ReactFlowProvider } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import * as dagre from "dagre";
import { FC, ReactNode, useEffect, useMemo, RefObject, useCallback } from "react";

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
export function calculateDiagramLayout(
  nodes: Node[],
  edges: Edge[],
  options: DiagramLayoutOptions = {}
): { nodes: Node[]; edges: Edge[] } {
  const {
    direction = "TB",
    nodeWidth = 48,
    nodeHeight = 48,
    rankSep = 80,
    nodeSep = 50,
  } = options;

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
  /** Initial nodes */
  initialNodes: Node[];
  /** Initial edges */
  initialEdges: Edge[];
  /** Callback when nodes change */
  onNodesChange?: (nodes: Node[]) => void;
  /** Callback when edges change */
  onEdgesChange?: (edges: Edge[]) => void;
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
  /** Callback when an edge is clicked */
  onEdgeClick?: (event: React.MouseEvent, edge: Edge) => void;
  /** Callback when an edge is mouse enter */
  onEdgeMouseEnter?: (event: React.MouseEvent, edge: Edge) => void;
  /** Callback when an edge is mouse leave */
  onEdgeMouseLeave?: (event: React.MouseEvent, edge: Edge) => void;
  /** Ref to access ReactFlow instance */
  reactFlowInstanceRef?: RefObject<ReactFlowInstance | null>;
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
  /** Delete key code */
  deleteKeyCode?: string | string[];
  /** Enable pan on drag */
  panOnDrag?: boolean;
  /** Enable selection on drag */
  selectionOnDrag?: boolean;
  /** Enable zoom on scroll */
  zoomOnScroll?: boolean;
  /** Enable zoom on pinch */
  zoomOnPinch?: boolean;
  /** Enable zoom on double click */
  zoomOnDoubleClick?: boolean;
}

/**
 * Generalized Diagram component built on ReactFlow
 * Provides common diagram functionality with customizable node and edge types
 * Note: This component is wrapped in ReactFlowProvider automatically
 */
const DiagramInner: FC<DiagramProps> = ({
  nodeTypes,
  edgeTypes,
  initialNodes,
  initialEdges,
  onNodesChange: onNodesChangeProp,
  onEdgesChange: onEdgesChangeProp,
  onConnect,
  onNodeClick,
  onNodeDoubleClick,
  onNodeMouseEnter,
  onNodeMouseLeave,
  onEdgeClick,
  onEdgeMouseEnter,
  onEdgeMouseLeave,
  reactFlowInstanceRef,
  showBackground = false,
  backgroundVariant = BackgroundVariant.Dots,
  showControls = true,
  showMinimap = false,
  panels,
  className = "",
  fitView = true,
  minZoom = 0.1,
  maxZoom = 4,
  connectionMode = "strict",
  deleteKeyCode = "Delete",
  panOnDrag = true,
  selectionOnDrag = false,
  zoomOnScroll = true,
  zoomOnPinch = true,
  zoomOnDoubleClick = false,
}) => {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // Callback to capture ReactFlow instance from onInit
  const handleInit = useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        reactFlowInstanceRef.current = instance;
      }
    },
    [reactFlowInstanceRef]
  );

  // Update nodes and edges when initial values change
  useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
  }, [initialNodes, initialEdges, setNodes, setEdges]);

  // Forward changes to parent if callbacks provided
  useEffect(() => {
    if (onNodesChangeProp) {
      onNodesChangeProp(nodes);
    }
  }, [nodes, onNodesChangeProp]);

  useEffect(() => {
    if (onEdgesChangeProp) {
      onEdgesChangeProp(edges);
    }
  }, [edges, onEdgesChangeProp]);

  return (
    <div className={`relative w-full h-full ${className}`}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onInit={handleInit}
        onNodeClick={onNodeClick}
        onNodeDoubleClick={onNodeDoubleClick}
        onNodeMouseEnter={onNodeMouseEnter}
        onNodeMouseLeave={onNodeMouseLeave}
        onEdgeClick={onEdgeClick}
        onEdgeMouseEnter={onEdgeMouseEnter}
        onEdgeMouseLeave={onEdgeMouseLeave}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
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
        proOptions={{ hideAttribution: true }}
        className="bg-background"
      >
        {showBackground && (
          <Background
            variant={backgroundVariant}
            gap={12}
            size={1}
            color="var(--border)"
          />
        )}
        {showControls && <Controls className="border border-border" showZoom={false} showInteractive={false} />}
        {showMinimap && (
          <MiniMap
            className="border border-border"
            maskColor="var(--accent)"
            bgColor="var(--background)"
            nodeStrokeWidth={3}
            zoomable
            pannable
          />
        )}
        {panels}
      </ReactFlow>
    </div>
  );
};

/**
 * Diagram component with ReactFlowProvider wrapper
 */
export const Diagram: FC<DiagramProps> = (props) => {
  return (
    <ReactFlowProvider>
      <DiagramInner {...props} />
    </ReactFlowProvider>
  );
};

/**
 * Helper hook to create a diagram with automatic layout
 */
export function useDiagramLayout(
  initialNodes: Node[],
  initialEdges: Edge[],
  layoutOptions?: DiagramLayoutOptions
): { nodes: Node[]; edges: Edge[] } {
  return useMemo(() => {
    if (initialNodes.length === 0) {
      return { nodes: [], edges: [] };
    }
    return calculateDiagramLayout(initialNodes, initialEdges, layoutOptions);
  }, [initialNodes, initialEdges, layoutOptions]);
}
