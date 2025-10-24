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
import { Connection as FlowConnection, Node, Edge, NodeTypes, ReactFlowInstance } from "@xyflow/react";
import { FC, useCallback, useMemo, RefObject } from "react";
import { useQualityEditor, useQualityEditorCommands } from "../store";
import { formulaFunctions } from "../functions";
import { DiagramNode, PlaceholderDiagramNode } from "@semio/js/elements/display/DiagramNode";
import { Diagram as BaseDiagram, calculateDiagramLayout } from "@semio/js/elements/Diagram";

// Function node with circular design using DiagramNode
const FunctionNode: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const initials = data.label.substring(0, 2).toUpperCase();
  
  return (
    <DiagramNode
      content={initials}
      selected={selected}
      showTopHandle
      showBottomHandle
    />
  );
};

// Quality node with circular design
const QualityNode: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const initials = data.label
    .split(".")
    .map((part: string) => part[0])
    .join("")
    .substring(0, 2)
    .toUpperCase();
    
  return (
    <DiagramNode
      content={initials}
      selected={selected}
      showTopHandle
      showBottomHandle
    />
  );
};

// Variable node
const VariableNode: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const varName = data.label.startsWith("$") ? data.label.substring(1) : data.label;
  const initials = varName.substring(0, 2).toUpperCase();
  
  return (
    <DiagramNode
      content={initials}
      selected={selected}
      showTopHandle
      showBottomHandle
    />
  );
};

// Value node (units, literals)
const ValueNode: FC<{ data: any; selected?: boolean }> = ({ data, selected }) => {
  const display = data.label.length > 4 ? data.label.substring(0, 4) : data.label;
  
  return (
    <DiagramNode
      content={display}
      selected={selected}
      showTopHandle
    />
  );
};

// Placeholder node for empty formula or drop targets
const PlaceholderNode: FC<{ data: any }> = ({ data }) => {
  return <PlaceholderDiagramNode label={data.label || "+ Drop"} />;
};

const nodeTypes: NodeTypes = {
  function: FunctionNode,
  quality: QualityNode,
  variable: VariableNode,
  unit: ValueNode,
  value: ValueNode,
  placeholder: PlaceholderNode,
};

interface QualityDiagramProps {
  reactFlowInstanceRef: RefObject<ReactFlowInstance | null>;
}

const QualityDiagram: FC<QualityDiagramProps> = ({ reactFlowInstanceRef }) => {
  const formulaNodes = useQualityEditor((s) => s.formulaNodes) as any[];
  const { selectFormulaNode, hoverFormulaNode, clearHover, connectNodes } = useQualityEditorCommands();
  const { setNodeRef: setDroppableRef } = useDroppable({ id: "quality-diagram-drop-zone" });

  const { nodes: initialNodes, edges: initialEdges } = useMemo(() => {
    // If no formula nodes exist, show a root placeholder
    if (!formulaNodes || formulaNodes.length === 0) {
      const placeholderNode: Node = {
        id: "root-placeholder",
        type: "placeholder",
        position: { x: 0, y: 0 },
        data: { label: "+ Start formula" },
      };
      return { nodes: [placeholderNode], edges: [] };
    }

    const nodes: Node[] = [];
    const edges: Edge[] = [];
    const placeholderNodes: Node[] = [];
    const placeholderEdges: Edge[] = [];

    // Create nodes for existing formula nodes
    formulaNodes.forEach((node) => {
      nodes.push({
        id: node.id,
        type: node.type,
        position: { x: node.x ?? 0, y: node.y ?? 0 },
        data: { label: node.name },
      });

      // Create edges for children
      if (node.children) {
        node.children.forEach((childId: string) => {
          edges.push({
            id: `${node.id}-${childId}`,
            source: node.id,
            target: childId,
          });
        });
      }

      // Add placeholder nodes for missing operands of function nodes
      if (node.type === "function") {
        const fn = formulaFunctions[node.name];
        const arity = fn?.arity;
        const currentChildCount = node.children?.length || 0;
        
        if (arity === "variadic" || (typeof arity === "number" && currentChildCount < arity)) {
          // Add placeholders for missing operands
          const maxPlaceholders = arity === "variadic" ? 1 : (arity - currentChildCount);
          
          for (let i = 0; i < maxPlaceholders; i++) {
            const placeholderId = `${node.id}-placeholder-${currentChildCount + i}`;
            placeholderNodes.push({
              id: placeholderId,
              type: "placeholder",
              position: { x: 0, y: 0 },
              data: { 
                label: `+ ${i + 1}`,
                parentId: node.id,
                operandIndex: currentChildCount + i
              },
            });

            // Add dashed edge from parent to placeholder
            placeholderEdges.push({
              id: `${node.id}-${placeholderId}`,
              source: node.id,
              target: placeholderId,
              style: { strokeDasharray: "5 5", opacity: 0.5 },
              animated: false,
            });
          }
        }
      }
    });

    // Combine all nodes and edges, then layout
    const allNodes = [...nodes, ...placeholderNodes];
    const allEdges = [...edges, ...placeholderEdges];

    return calculateDiagramLayout(allNodes, allEdges, {
      direction: "TB",
      nodeWidth: 48,
      nodeHeight: 48,
      rankSep: 80,
      nodeSep: 50,
    });
  }, [formulaNodes]);

  const handleConnect = useCallback(
    (connection: FlowConnection) => {
      if (connection.source && connection.target) {
        connectNodes?.(connection.source, connection.target);
      }
    },
    [connectNodes]
  );

  return (
    <div ref={setDroppableRef} className="h-full w-full bg-base">
      <BaseDiagram
        nodeTypes={nodeTypes}
        initialNodes={initialNodes}
        initialEdges={initialEdges}
        onConnect={handleConnect}
        onNodeClick={(_, node) => selectFormulaNode(node.id)}
        onNodeMouseEnter={(_, node) => hoverFormulaNode(node.id)}
        onNodeMouseLeave={() => clearHover()}
        reactFlowInstanceRef={reactFlowInstanceRef}
        showControls
        fitView
      />
    </div>
  );
};

export default QualityDiagram;
