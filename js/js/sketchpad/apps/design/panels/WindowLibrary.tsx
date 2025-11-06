// #region Header

// WindowLibrary.tsx

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

import { useDraggable } from "@dnd-kit/core";
import { Box, Eye, Grid3x3, Network, Table2 } from "lucide-react";
import { FC } from "react";
import { TreeContent, TreeItem, TreeSection } from "../../../../elements/aggregation/Tree";

interface WindowTemplate {
  id: string;
  label: string;
  icon: React.ReactNode;
  windowTypeId: string;
  componentProps?: any;
}

const windowTemplates: WindowTemplate[] = [
  {
    id: "scene-perspective",
    label: "Perspective View",
    icon: <Box size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "perspective" },
  },
  {
    id: "scene-top",
    label: "Top View",
    icon: <Eye size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "top" },
  },
  {
    id: "scene-bottom",
    label: "Bottom View",
    icon: <Eye size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "bottom" },
  },
  {
    id: "scene-left",
    label: "Left View",
    icon: <Eye size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "left" },
  },
  {
    id: "scene-right",
    label: "Right View",
    icon: <Eye size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "right" },
  },
  {
    id: "diagram-full",
    label: "Full Diagram",
    icon: <Network size={16} />,
    windowTypeId: "diagram",
    componentProps: { graphType: "full" },
  },
  {
    id: "diagram-subgraph",
    label: "Subgraph",
    icon: <Grid3x3 size={16} />,
    windowTypeId: "diagram",
    componentProps: { graphType: "subgraph" },
  },
  {
    id: "table-pieces",
    label: "Pieces Table",
    icon: <Table2 size={16} />,
    windowTypeId: "table",
    componentProps: { dataType: "pieces" },
  },
  {
    id: "table-connections",
    label: "Connections Table",
    icon: <Table2 size={16} />,
    windowTypeId: "table",
    componentProps: { dataType: "connections" },
  },
];

interface DraggableWindowItemProps {
  template: WindowTemplate;
}

const DraggableWindowItem: FC<DraggableWindowItemProps> = ({ template }) => {
  const { attributes, listeners, setNodeRef, isDragging } = useDraggable({
    id: template.id,
    data: {
      type: "window-template",
      windowTypeId: template.windowTypeId,
      componentProps: template.componentProps,
    },
  });

  return (
    <div ref={setNodeRef} {...listeners} {...attributes} className={`cursor-grab active:cursor-grabbing ${isDragging ? "opacity-50" : ""}`}>
      <TreeItem>
        <TreeContent>
          <div className="flex items-center gap-2">
            {template.icon}
            <span className="text-sm">{template.label}</span>
          </div>
        </TreeContent>
      </TreeItem>
    </div>
  );
};

export const WindowLibrary: FC = () => {
  const sceneTemplates = windowTemplates.filter((t) => t.windowTypeId === "scene");
  const diagramTemplates = windowTemplates.filter((t) => t.windowTypeId === "diagram");
  const tableTemplates = windowTemplates.filter((t) => t.windowTypeId === "table");

  return (
    <div>
      <TreeSection id="semio.sketchpad.app.design.windowLibrary.scene" defaultOpen={true}>
        {sceneTemplates.map((template) => (
          <DraggableWindowItem key={template.id} template={template} />
        ))}
      </TreeSection>
      <TreeSection id="semio.sketchpad.app.design.windowLibrary.diagram" defaultOpen={true}>
        {diagramTemplates.map((template) => (
          <DraggableWindowItem key={template.id} template={template} />
        ))}
      </TreeSection>
      <TreeSection id="semio.sketchpad.app.design.windowLibrary.table" defaultOpen={false}>
        {tableTemplates.map((template) => (
          <DraggableWindowItem key={template.id} template={template} />
        ))}
      </TreeSection>
    </div>
  );
};
