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
import { FC, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { useHotkeys } from "react-hotkeys-hook";

import { ReactFlowInstance, ReactFlowProvider } from "@xyflow/react";
import { DesignId, ICON_WIDTH, TypeId } from "../../../semio";
import { DesignEditorFullscreenPanel, useDesign, useDesignEditorCommands, useDesignEditorFullscreen, useDesignEditorSelection, useKit, useSketchpad } from "../../../store";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../Resizable";
import { TreeItem } from "../Tree";
import { ConnectionsSection, DesignSection, PiecesSection, PortSection } from "./Details";
import Diagram from "./Diagram";
import Model from "./Model";
import { useAddPanelSection, useRemovePanelSection } from "./Navbar";
import { DesignAvatar, TypeAvatar } from "./Workbench";

export interface DesignEditorProps {}

const DesignEditor: FC<DesignEditorProps> = () => {
  const fullscreenPanel = useDesignEditorFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleModelFullscreen, addPiece } = useDesignEditorCommands();

  const selection = useDesignEditorSelection();
  const design = useDesign();
  const kit = useKit();
  const editorSettings = useSketchpad((s) => s.editorSettings);

  const [activeDraggedTypeId, setActiveDraggedTypeId] = useState<TypeId | null>(null);
  const [activeDraggedDesignId, setActiveDraggedDesignId] = useState<DesignId | null>(null);

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  // Add/remove details panel sections based on selection
  useEffect(() => {
    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.port !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    // Remove all details sections first
    removeSection("details", "design");
    removeSection("details", "port");
    removeSection("details", "pieces");
    removeSection("details", "connections");
    removeSection("details", "mixed");

    // Add appropriate sections based on selection
    if (!hasSelection) {
      addSection("details", {
        id: "design",
        label: "Design",
        order: 0,
        defaultOpen: true,
        content: <DesignSection />,
      });
    } else if (hasPortSelected) {
      addSection("details", {
        id: "port",
        label: "Port",
        order: 1,
        defaultOpen: true,
        content: <PortSection pieceId={selection.port!.piece} portId={selection.port!.port} />,
      });
    } else {
      if (hasPieces) {
        addSection("details", {
          id: "pieces",
          label: selection.pieces!.length === 1 ? "Piece" : `Pieces (${selection.pieces!.length})`,
          order: 2,
          defaultOpen: true,
          content: <PiecesSection />,
        });
      }
      if (hasConnections) {
        addSection("details", {
          id: "connections",
          label: selection.connections!.length === 1 ? "Connection" : `Connections (${selection.connections!.length})`,
          order: 3,
          defaultOpen: true,
          content: <ConnectionsSection connections={selection.connections!} />,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: "mixed",
          label: "Mixed Selection",
          order: 4,
          defaultOpen: true,
          content: (
            <TreeItem>
              <p className="text-sm text-muted-foreground">Select only pieces or only connections to edit details.</p>
            </TreeItem>
          ),
        });
      }
    }
  }, [selection, addSection, removeSection]);

  // Add workbench sections
  useEffect(() => {
    const typesByName = (kit.types || []).reduce(
      (acc, type) => {
        if (!acc[type.name]) acc[type.name] = [];
        acc[type.name].push(type);
        return acc;
      },
      {} as Record<string, any[]>,
    );

    const designsByName = (kit.designs || []).reduce(
      (acc, design) => {
        if (!acc[design.name]) acc[design.name] = [];
        acc[design.name].push(design);
        return acc;
      },
      {} as Record<string, any[]>,
    );

    addSection("workbench", {
      id: "types",
      label: "Types",
      order: 0,
      defaultOpen: true,
      content: (
        <>
          {Object.entries(typesByName).map(([name, variants]) => (
            <TreeItem key={name} label={name} defaultOpen={false}>
              <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                {variants.map((type: any) => (
                  <TypeAvatar key={`${type.name}-${type.variant}`} typeId={type} showHoverCard={true} />
                ))}
              </div>
            </TreeItem>
          ))}
        </>
      ),
    });

    addSection("workbench", {
      id: "designs",
      label: "Designs",
      order: 1,
      defaultOpen: true,
      content: (
        <>
          {Object.entries(designsByName).map(([name, designs]) => (
            <TreeItem key={name} label={name} defaultOpen={false}>
              <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                {designs.map((design: any) => (
                  <DesignAvatar key={`${design.name}-${design.variant}-${design.view}`} designId={design} showHoverCard={true} />
                ))}
              </div>
            </TreeItem>
          ))}
        </>
      ),
    });

    return () => {
      removeSection("workbench", "types");
      removeSection("workbench", "designs");
    };
  }, [kit, addSection, removeSection]);

  // Add settings section
  useEffect(() => {
    addSection("settings", {
      id: "design-editor-settings",
      label: "Design Editor",
      order: 100,
      defaultOpen: true,
      content: (
        <>
          <TreeItem>
            <div className="flex flex-col gap-2">
              <label>Snappiness: {editorSettings.design?.snappiness}</label>
              <input type="range" min="0" max="20" value={editorSettings.design?.snappiness || 10} className="w-full" readOnly />
            </div>
          </TreeItem>
          <TreeItem>Grid Size: {editorSettings.design?.gridSize || 24}px</TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("settings", "design-editor-settings");
    };
  }, [editorSettings, addSection, removeSection]);

  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event;
    const id = active.id as string;

    if (id.startsWith("type-")) {
      const parts = id.replace("type-", "").split("-");
      const name = parts[0];
      const variant = parts[1] || undefined;
      setActiveDraggedTypeId({ name, variant });
    } else if (id.startsWith("design-")) {
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
