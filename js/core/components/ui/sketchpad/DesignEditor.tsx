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
import { Info, MessageCircle, Terminal, Wrench } from "lucide-react";
import { FC, useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { useHotkeys } from "react-hotkeys-hook";

import { ReactFlowProvider } from "@xyflow/react";
import { DesignId, TypeId } from "../../../semio";
import { useDesignEditorCommands, useDesignEditorFullscreen } from "../../../store";
import Navbar, { useNavbar } from "../Navbar";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";
import Chat from "./Chat";
import Console from "./Console";
import Details from "./Details";
import Diagram from "./Diagram";
import Model from "./Model";
import Workbench, { DesignAvatar, TypeAvatar } from "./Workbench";

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

const DesignEditor: FC<DesignEditorProps> = () => {
  const { setNavbarToolbar } = useNavbar();
  const fullscreenPanel = useDesignEditorFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, addPiece, execute } = useDesignEditorCommands();

  // Panel visibility and sizing state
  const [visiblePanels, setVisiblePanels] = useState<VisiblePanels>({
    workbench: false,
    details: false,
    console: false,
    chat: false,
  });

  const [workbenchWidth, setWorkbenchWidth] = useState(230);
  const [detailsWidth, setDetailsWidth] = useState(230);
  const [chatWidth, setChatWidth] = useState(230);
  const [consoleHeight, setConsoleHeight] = useState(200);

  // Drag and drop state
  const [activeDraggedTypeId, setActiveDraggedTypeId] = useState<TypeId | null>(null);
  const [activeDraggedDesignId, setActiveDraggedDesignId] = useState<DesignId | null>(null);

  // Hotkeys for common actions
  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  useHotkeys("mod+j", (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("workbench");
  });
  useHotkeys("mod+k", (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("console");
  });
  useHotkeys("mod+l", (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("details");
  });
  useHotkeys(["mod+[", "mod+semicolon", "mod+ö"], (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("chat");
  });

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

    if (over && over.id === "diagram-drop-zone") {
      const ICON_WIDTH = 64;
      const centerX = Math.round(delta.x / ICON_WIDTH);
      const centerY = Math.round(-delta.y / ICON_WIDTH);

      if (activeDraggedTypeId) {
        const piece = {
          id_: `piece-${Date.now()}`,
          type: activeDraggedTypeId,
          center: { x: centerX, y: centerY },
        };
        addPiece(piece).catch(() => {});
      } else if (activeDraggedDesignId) {
        const piece = {
          id_: `design-${Date.now()}`,
          design: activeDraggedDesignId,
          center: { x: centerX, y: centerY },
        };
        addPiece(piece).catch(() => {});
      }
    }

    setActiveDraggedTypeId(null);
    setActiveDraggedDesignId(null);
  };

  const togglePanel = (panel: keyof VisiblePanels) => {
    setVisiblePanels((prev) => {
      const newState = { ...prev };
      if (panel === "chat" && !prev.chat) {
        newState.details = false;
      }
      if (panel === "details" && !prev.details) {
        newState.chat = false;
      }
      newState[panel] = !prev[panel];
      return newState;
    });
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

  const designEditorToolbar = (
    <ToggleGroup
      type="multiple"
      value={Object.entries(visiblePanels)
        .filter(([_, isVisible]) => isVisible)
        .map(([key]) => key)}
      onValueChange={(values) => {
        Object.keys(visiblePanels).forEach((key) => {
          const isCurrentlyVisible = visiblePanels[key as keyof VisiblePanels];
          const shouldBeVisible = values.includes(key);
          if (isCurrentlyVisible !== shouldBeVisible) {
            togglePanel(key as keyof VisiblePanels);
          }
        });
      }}
    >
      <ToggleGroupItem value="workbench" tooltip="Workbench" hotkey="⌘J">
        <Wrench />
      </ToggleGroupItem>
      <ToggleGroupItem value="console" tooltip="Console" hotkey="⌘K">
        <Terminal />
      </ToggleGroupItem>
      <ToggleGroupItem value="details" tooltip="Details" hotkey="⌘L">
        <Info />
      </ToggleGroupItem>
      <ToggleGroupItem value="chat" tooltip="Chat" hotkey="⌘[">
        <MessageCircle />
      </ToggleGroupItem>
    </ToggleGroup>
  );

  useEffect(() => {
    setNavbarToolbar(designEditorToolbar);
    return () => setNavbarToolbar(null);
  }, [visiblePanels, setNavbarToolbar]);

  const rightPanelVisible = visiblePanels.details || visiblePanels.chat;

  return (
    <DndContext onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
      <div className="h-screen flex flex-col overflow-hidden bg-background">
        <Navbar />
        <div className="flex-1 flex overflow-hidden relative">
          {visiblePanels.workbench && <Workbench visible={visiblePanels.workbench} onWidthChange={setWorkbenchWidth} width={workbenchWidth} />}
          <ReactFlowProvider>
            <div className="flex-1 flex flex-col">
              <div className="flex-1 flex">
                <div className="flex-1 relative">
                  <Diagram />
                </div>
                <div className="flex-1 relative">
                  <Model />
                </div>
              </div>
              {visiblePanels.console && (
                <div className="h-48 border-t border-border">
                  <Console />
                </div>
              )}
            </div>
          </ReactFlowProvider>
          {rightPanelVisible && (
            <div className="flex">
              {visiblePanels.details && <Details visible={visiblePanels.details} onWidthChange={setDetailsWidth} width={detailsWidth} />}
              {visiblePanels.chat && <Chat visible={visiblePanels.chat} onWidthChange={setChatWidth} width={chatWidth} />}
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
      </div>
    </DndContext>
  );
};

export default DesignEditor;
