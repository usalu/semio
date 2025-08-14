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
import { FC, useState } from "react";
import { createPortal } from "react-dom";
import { useHotkeys } from "react-hotkeys-hook";

import { DesignId, TypeId } from "@semio/js";
import { ReactFlowProvider } from "@xyflow/react";
import { DesignScopeProvider, useCommands, useDesign, useDesignEditorStoreFullscreenPanel, useDesignId, useKit } from "../../store";
import Chat from "./Chat";
import Console from "./Console";
import Details from "./Details";
import Diagram from "./Diagram";
import Model from "./Model";
import Navbar from "./Navbar";
import Workbench, { TypeAvatar } from "./Workbench";

export interface DesignEditorProps {}

export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

interface VisiblePanels {
  workbench: boolean;
  details: boolean;
  console: boolean;
  chat: boolean;
}

interface DesignAvatarProps {
  designId: DesignId;
  showHoverCard?: boolean;
}

export const DesignAvatar: FC<DesignAvatarProps> = ({ designId, showHoverCard = false }) => {
  const displayName = designId.name || "Untitled";
  return (
    <div className="flex items-center gap-2 p-2 rounded border cursor-pointer hover:bg-gray-100">
      <div className="w-8 h-8 bg-blue-100 rounded flex items-center justify-center text-xs font-medium">{displayName.substring(0, 2).toUpperCase()}</div>
      <span className="text-sm">{displayName}</span>
    </div>
  );
};

const DesignEditor: FC<DesignEditorProps> = () => {
  const kit = useKit();
  const design = useDesign();
  const designId = useDesignId();
  const fullscreenPanel = useDesignEditorStoreFullscreenPanel();
  const commands = useCommands();

  // Panel visibility and sizing state
  const [visiblePanels, setVisiblePanels] = useState<VisiblePanels>({
    workbench: true,
    details: true,
    console: false,
    chat: false,
  });

  const [workbenchWidth, setWorkbenchWidth] = useState(320);
  const [detailsWidth, setDetailsWidth] = useState(320);
  const [chatWidth, setChatWidth] = useState(320);
  const [consoleHeight, setConsoleHeight] = useState(200);

  // Drag and drop state
  const [activeDraggedTypeId, setActiveDraggedTypeId] = useState<TypeId | null>(null);
  const [activeDraggedDesignId, setActiveDraggedDesignId] = useState<DesignId | null>(null);

  // Hotkeys for common actions
  useHotkeys("ctrl+a", () => commands.selectAll());
  useHotkeys("ctrl+d", () => commands.deselectAll());
  useHotkeys("delete", () => commands.deleteSelected());
  useHotkeys("ctrl+z", () => commands.undo());
  useHotkeys("ctrl+y", () => commands.redo());
  useHotkeys("ctrl+shift+z", () => commands.redo());

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
    const { active, over } = event;

    if (over && over.id === "diagram-drop-zone") {
      if (activeDraggedTypeId) {
        // TODO: Add piece to diagram using commands
        console.log("Adding type to diagram:", activeDraggedTypeId);
      } else if (activeDraggedDesignId) {
        // TODO: Add design to diagram using commands
        console.log("Adding design to diagram:", activeDraggedDesignId);
      }
    }

    setActiveDraggedTypeId(null);
    setActiveDraggedDesignId(null);
  };

  const togglePanel = (panel: keyof VisiblePanels) => {
    setVisiblePanels((prev) => ({
      ...prev,
      [panel]: !prev[panel],
    }));
  };

  // Check for fullscreen mode
  if (fullscreenPanel && fullscreenPanel !== "none") {
    return (
      <div className="h-screen w-screen bg-background">
        {fullscreenPanel === "diagram" && <Diagram />}
        {fullscreenPanel === "model" && <Model />}
      </div>
    );
  }

  if (!kit || !design || !designId) {
    return <div className="flex items-center justify-center h-screen">Loading...</div>;
  }

  const rightPanelVisible = visiblePanels.details || visiblePanels.chat;

  return (
    <DndContext onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
      <div className="h-screen flex flex-col overflow-hidden bg-background">
        <Navbar
          toolbarContent={
            <div className="flex items-center gap-2">
              <button onClick={() => togglePanel("workbench")} className={`px-3 py-1 rounded text-sm ${visiblePanels.workbench ? "bg-primary text-primary-foreground" : "bg-muted"}`}>
                Workbench
              </button>
              <button onClick={() => togglePanel("console")} className={`px-3 py-1 rounded text-sm ${visiblePanels.console ? "bg-primary text-primary-foreground" : "bg-muted"}`}>
                Console
              </button>
              <button onClick={() => togglePanel("details")} className={`px-3 py-1 rounded text-sm ${visiblePanels.details ? "bg-primary text-primary-foreground" : "bg-muted"}`}>
                Details
              </button>
              <button onClick={() => togglePanel("chat")} className={`px-3 py-1 rounded text-sm ${visiblePanels.chat ? "bg-primary text-primary-foreground" : "bg-muted"}`}>
                Chat
              </button>
              <button onClick={() => commands.toggleDiagramFullscreen()} className="px-3 py-1 rounded text-sm bg-muted">
                Fullscreen
              </button>
            </div>
          }
        />
        <div className="flex-1 flex overflow-hidden relative">
          <ReactFlowProvider>
            <DesignScopeProvider id={designId}>
              <div className="flex-1 flex flex-col">
                <div className="flex-1 flex">
                  <div className="flex-1 relative">
                    <Diagram />
                  </div>
                  <div className="flex-1 relative">
                    <Model />
                  </div>
                </div>
              </div>
            </DesignScopeProvider>
          </ReactFlowProvider>
        </div>
        <Workbench visible={visiblePanels.workbench} onWidthChange={setWorkbenchWidth} width={workbenchWidth} />
        <Details visible={visiblePanels.details} onWidthChange={setDetailsWidth} width={detailsWidth} />
        <Console />
        <Chat visible={visiblePanels.chat} onWidthChange={setChatWidth} width={chatWidth} />
        {createPortal(
          <DragOverlay>
            {activeDraggedTypeId && <TypeAvatar typeId={activeDraggedTypeId} />}
            {activeDraggedDesignId && <DesignAvatar designId={activeDraggedDesignId} />}
          </DragOverlay>,
          document.body,
        )}
      </div>
    </DndContext>
  );
};

export default DesignEditor;
