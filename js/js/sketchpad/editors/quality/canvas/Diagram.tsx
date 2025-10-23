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

import { useDroppable } from "@dnd-kit/core";
import { ReactFlow, Background, Controls, Edge, Connection as FlowConnection, Node, NodeTypes, addEdge, useEdgesState, useNodesState, ReactFlowInstance, useReactFlow } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import dagre from "dagre";
import { FC, useCallback, useEffect, useMemo, RefObject } from "react";
import { useQualityEditor, useQualityEditorCommands } from "../store";

const nodeTypes: NodeTypes = {
  function: ({ data }: any) => (
    <div className="border border-foreground bg-panel p-2 min-w-[calc(var(--spacing)*20)] text-center">
      <div className="text-xs font-bold">{data.label}</div>
    </div>
  ),
  quality: ({ data }: any) => (
    <div className="border border-foreground bg-panel p-2 min-w-[calc(var(--spacing)*20)] text-center">
      <div className="text-xs text-foreground">{data.label}</div>
    </div>
  ),
  variable: ({ data }: any) => (
    <div className="border border-foreground bg-panel p-2 min-w-[calc(var(--spacing)*20)] text-center">
      <div className="text-xs text-foreground">{data.label}</div>
    </div>
  ),
  unit: ({ data }: any) => (
    <div className="border border-foreground bg-panel p-2 min-w-[calc(var(--spacing)*20)] text-center">
      <div className="text-xs text-foreground">{data.label}</div>
    </div>
  ),
  value: ({ data }: any) => (
    <div className="border border-foreground bg-panel p-2 min-w-[calc(var(--spacing)*20)] text-center">
      <div className="text-xs text-foreground">{data.label}</div>
    </div>
  ),
};

const getLayoutedElements = (nodes: Node[], edges: Edge[], direction = "TB") => {
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({ rankdir: direction });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: 160, height: 40 });
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
        x: nodeWithPosition.x - 80,
        y: nodeWithPosition.y - 20,
      },
    };
  });

  return { nodes: layoutedNodes, edges };
};

interface DiagramProps {
  reactFlowInstanceRef: RefObject<ReactFlowInstance | null>;
}

const Diagram: FC<DiagramProps> = ({ reactFlowInstanceRef }) => {
  const formulaNodes = useQualityEditor((s) => s.formulaNodes) as any[];
  const { selectFormulaNode, hoverFormulaNode, clearHover, connectNodes } = useQualityEditorCommands();
  const reactFlowInstance = useReactFlow();
  const { setNodeRef: setDroppableRef } = useDroppable({ id: "quality-diagram-drop-zone" });

  useEffect(() => {
    reactFlowInstanceRef.current = reactFlowInstance;
  }, [reactFlowInstance, reactFlowInstanceRef]);

  const { nodes: initialNodes, edges: initialEdges } = useMemo(() => {
    const nodes: Node[] = formulaNodes.map((node) => ({
      id: node.id,
      type: node.type,
      position: { x: node.x ?? 0, y: node.y ?? 0 },
      data: { label: node.name },
    }));

    const edges: Edge[] = [];
    formulaNodes.forEach((node) => {
      if (node.children) {
        node.children.forEach((childId: string) => {
          edges.push({
            id: `${node.id}-${childId}`,
            source: node.id,
            target: childId,
          });
        });
      }
    });

    return getLayoutedElements(nodes, edges, "TB");
  }, [formulaNodes]);

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (connection: FlowConnection) => {
      if (connection.source && connection.target) {
        connectNodes?.(connection.source, connection.target);
        setEdges((eds) => addEdge(connection, eds));
      }
    },
    [connectNodes, setEdges]
  );

  return (
    <div ref={setDroppableRef} className="h-full w-full bg-base">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        nodeTypes={nodeTypes}
        fitView
        onNodeClick={(_, node) => selectFormulaNode(node.id)}
        onNodeMouseEnter={(_, node) => hoverFormulaNode(node.id)}
        onNodeMouseLeave={() => clearHover()}
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
};

export default Diagram;
