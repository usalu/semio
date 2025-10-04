// #region Header

// DesignEditor.tsx

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

import { DndContext, DragEndEvent, DragOverlay, DragStartEvent } from "@dnd-kit/core";
import { FC, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { useHotkeys } from "react-hotkeys-hook";

import { ReactFlowInstance, ReactFlowProvider } from "@xyflow/react";
import { DesignId, ICON_WIDTH, TypeId } from "../../../semio";
import { DesignEditorFullscreenPanel, EditorType, useDesignEditorCommands, useDesignEditorFullscreen, useSketchpad } from "../../../store";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../Resizable";
import Chat from "./Chat";
import Console from "./Console";
import Details from "./Details";
import Diagram from "./Diagram";
import Model from "./Model";
import Settings from "./Settings";
import Workbench, { DesignAvatar, TypeAvatar } from "./Workbench";

export interface DesignEditorProps {}

const DesignEditor: FC<DesignEditorProps> = () => {
  const fullscreenPanel = useDesignEditorFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleModelFullscreen, addPiece, execute } = useDesignEditorCommands();

  const visiblePanels = useSketchpad((s) => s.panelVisibility[EditorType.DESIGN]) || {};

  const [workbenchWidth, setWorkbenchWidth] = useState(230);
  const [detailsWidth, setDetailsWidth] = useState(230);
  const [chatWidth, setChatWidth] = useState(230);
  const [consoleHeight, setConsoleHeight] = useState(200);

  const [activeDraggedTypeId, setActiveDraggedTypeId] = useState<TypeId | null>(null);
  const [activeDraggedDesignId, setActiveDraggedDesignId] = useState<DesignId | null>(null);

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event;
    const id = active.id as string;

    if (id.startsWith("type-")) {
      // Extract type information from draggable ID
      const parts = id.replace("type-", "").split("-");
      const name = parts[0];
      const variant = parts[1] || undefined;
      setActiveDraggedTypeId({ name, variant });
    } else if (id.startsWith("design-")) {
      // Extract design information from draggable ID
      const designName = id.replace("design-", "");
      setActiveDraggedDesignId({ name: designName, variant: "", view: "" });
    }
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over, delta } = event;

    if (over && over.id === "diagram-drop-zone" && reactFlowInstanceRef.current) {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return;
      }

      const { x, y } = reactFlowInstanceRef.current.screenToFlowPosition({
        x: event.activatorEvent.clientX + delta.x,
        y: event.activatorEvent.clientY + delta.y,
      });

      if (activeDraggedTypeId) {
        const piece = {
          id_: `piece-${Date.now()}`,
          type: activeDraggedTypeId,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece).catch(() => {});
      } else if (activeDraggedDesignId) {
        const piece = {
          id_: `design-${Date.now()}`,
          design: activeDraggedDesignId,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece).catch(() => {});
      }
    }

    setActiveDraggedTypeId(null);
    setActiveDraggedDesignId(null);
  };

  return (
    <DndContext onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
      <div className="flex-1 flex overflow-hidden relative">
        <ReactFlowProvider>
          <ResizablePanelGroup direction="horizontal">
            <ResizablePanel defaultSize={fullscreenPanel === DesignEditorFullscreenPanel.Diagram ? 100 : 50} className={`${fullscreenPanel === DesignEditorFullscreenPanel.Model ? "hidden" : "block"}`} onDoubleClick={toggleDiagramFullscreen}>
              <Diagram reactFlowInstanceRef={reactFlowInstanceRef} />
            </ResizablePanel>
            <ResizableHandle className={`border-r ${fullscreenPanel !== DesignEditorFullscreenPanel.None ? "hidden" : "block"}`} />
            <ResizablePanel defaultSize={fullscreenPanel === DesignEditorFullscreenPanel.Model ? 100 : 50} className={`${fullscreenPanel === DesignEditorFullscreenPanel.Diagram ? "hidden" : "block"}`} onDoubleClick={toggleModelFullscreen}>
              <Model />
            </ResizablePanel>
          </ResizablePanelGroup>
        </ReactFlowProvider>
        {visiblePanels.workbench && <Workbench visible={visiblePanels.workbench} onWidthChange={setWorkbenchWidth} width={workbenchWidth} />}
        {visiblePanels.console && <Console visible={visiblePanels.console} onHeightChange={setConsoleHeight} height={consoleHeight} />}
        {(visiblePanels.details || visiblePanels.chat || visiblePanels.settings) && (
          <div className="flex">
            {visiblePanels.details && <Details visible={visiblePanels.details} onWidthChange={setDetailsWidth} width={detailsWidth} />}
            {visiblePanels.chat && <Chat visible={visiblePanels.chat} onWidthChange={setChatWidth} width={chatWidth} />}
            {visiblePanels.settings && <Settings visible={visiblePanels.settings} onWidthChange={setDetailsWidth} width={detailsWidth} />}
          </div>
        )}
      </div>
      {createPortal(
        <DragOverlay>
          {activeDraggedTypeId && <TypeAvatar typeId={activeDraggedTypeId} />}
          {activeDraggedDesignId && <DesignAvatar designId={activeDraggedDesignId} />}
        </DragOverlay>,
        document.body,
      )}
    </DndContext>
  );
};

export default DesignEditor;
