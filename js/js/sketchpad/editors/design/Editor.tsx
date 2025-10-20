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

// Architecture:
// This editor demonstrates the generalized panel section system:
// - Registers "Types" and "Designs" sections in the workbench panel
// - Registers context-sensitive sections in the details panel based on selection
// - Registers editor-specific settings in the settings panel
//
// The panel system allows ANY component (including nested editors) to be mounted as a section.
// Example of nesting a design editor as a section:
//   addSection("workbench", {
//     id: "nested-design-editor",
//     label: "Nested Design",
//     order: 10,
//     defaultOpen: false,
//     content: () => (
//       <DesignScopeProvider guid={someDesignGuid}>
//         <Editor />
//       </DesignScopeProvider>
//     )
//   });

// #endregion

import { DragEndEvent } from "@dnd-kit/core";
import { Plus } from "lucide-react";
import { FC, useEffect, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";

import { ReactFlowInstance, ReactFlowProvider } from "@xyflow/react";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../../../elements/aggregation/Resizable";
import { TreeContent, TreeItem } from "../../../elements/aggregation/Tree";
import { Design, findConnectionsInDesign, guid, ICON_WIDTH, Kit, Type } from "../../../semio";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useDragDrop } from "../../Sketchpad";
import { EditorType, useDesign, useEditorPanelVisibility, useEditorType, useKit, useKitEditorCommands, useSketchpad, useSketchpadCommands } from "../../store";
import Diagram from "./canvas/Diagram";
import DesignScene from "./canvas/Scene";
import { ConnectionsSection, DesignSection, PiecesSection, PortSection } from "./panels/Details";
import { DesignAvatar, TypeAvatar } from "./panels/Workbench";
import { DesignEditorFullscreenWindow, useDesignEditorCommands, useDesignEditorFullscreen, useDesignEditorSelection } from "./store";
import { ToolsToggleGroup } from "./Toolbar";

export interface EditorProps {}

const Editor: FC<EditorProps> = () => {
  const fullscreenWindow = useDesignEditorFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleAccesslFullscreen, addPiece, startTransaction, finalizeTransaction, togglePanel } = useDesignEditorCommands();

  const selection = useDesignEditorSelection();
  const design = useDesign() as Design | undefined;
  const kit = useKit() as Kit;
  const editorSettings = useSketchpad((s) => s.editorSettings) as any;
  const panelVisibility = useEditorPanelVisibility();
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const kitEditorCommands = useKitEditorCommands();
  const { navigateToType, navigateToDesign } = useSketchpadCommands();

  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  const editorType = useEditorType();

  // Add/remove details panel sections based on selection
  useEffect(() => {
    if (editorType !== EditorType.DESIGN) return;

    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.port !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    removeSection("details", "design-details");
    removeSection("details", "design-port");
    removeSection("details", "design-pieces");
    removeSection("details", "design-connections");
    removeSection("details", "design-mixed");

    if (!hasSelection) {
      addSection("details", {
        id: "design-details",
        label: "Design",
        order: 0,
        defaultOpen: true,
        content: () => <DesignSection />,
      });
    } else if (hasPortSelected) {
      const portPieceId = selection.port!.piece;
      const portId = selection.port!.port;
      addSection("details", {
        id: "design-port",
        label: "Port",
        order: 1,
        defaultOpen: true,
        content: () => <PortSection pieceGuid={portPieceId} portGuid={portId} />,
      });
    } else {
      if (hasPieces) {
        addSection("details", {
          id: "design-pieces",
          label: selection.pieces!.length === 1 ? "Piece" : `Pieces (${selection.pieces!.length})`,
          order: 2,
          defaultOpen: true,
          content: () => <PiecesSection />,
        });
      }
      if (hasConnections) {
        const connGuids = selection.connections!;
        const conns = findConnectionsInDesign(design!, connGuids);
        addSection("details", {
          id: "design-connections",
          label: conns.length === 1 ? "Connection" : `Connections (${conns.length})`,
          order: 3,
          defaultOpen: true,
          content: () => <ConnectionsSection connections={conns} />,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: "design-mixed",
          label: "Mixed Selection",
          order: 4,
          defaultOpen: true,
          content: () => (
            <TreeItem>
              <TreeContent>
                <p className="text-sm text-muted-foreground">Select only pieces or only connections to edit details.</p>
              </TreeContent>
            </TreeItem>
          ),
        });
      }
    }

    return () => {
      removeSection("details", "design-details");
      removeSection("details", "design-port");
      removeSection("details", "design-pieces");
      removeSection("details", "design-connections");
      removeSection("details", "design-mixed");
    };
  }, [selection, addSection, removeSection, editorType]);

  const TypesWorkbenchContent: FC = () => {
    const { hoverTypes, clearHover } = useDesignEditorCommands();
    const typesByName = (kit.types || []).reduce((acc: Record<string, Type[]>, type: Type) => {
      if (!acc[type.name]) acc[type.name] = [];
      acc[type.name].push(type);
      return acc;
    }, {});

    const handleCreateVariant = (name: string) => {
      const existingTypes = typesByName[name] || [];
      const variantNumber = existingTypes.length + 1;
      const newType: Type = {
        guid: guid(),
        name,
        variant: `Variant ${variantNumber}`,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitEditorCommands.addType(newType);
      navigateToType(kit.guid, newType.guid);
    };

    return (
      <>
        {Object.entries(typesByName).map(([name, variants]) => (
          <div key={name} onPointerEnter={() => hoverTypes(variants.map((v) => v.guid))} onPointerLeave={() => clearHover()}>
            <TreeItem
              label={name}
              defaultOpen={true}
              actions={[
                {
                  icon: <Plus size={12} />,
                  onClick: () => handleCreateVariant(name),
                  title: "Add variant",
                },
              ]}
            >
              <TreeContent>
                <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                  {variants.map((type: Type) => (
                    <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                  ))}
                </div>
              </TreeContent>
            </TreeItem>
          </div>
        ))}
      </>
    );
  };

  const DesignsWorkbenchContent: FC = () => {
    const { hoverDesigns, clearHover } = useDesignEditorCommands();
    const designsByName = (kit.designs || []).reduce((acc: Record<string, Design[]>, design: Design) => {
      if (!acc[design.name]) acc[design.name] = [];
      acc[design.name].push(design);
      return acc;
    }, {});

    const handleCreateVariant = (name: string) => {
      const existingDesigns = designsByName[name] || [];
      const variantNumber = existingDesigns.length + 1;
      const newDesign: Design = {
        guid: guid(),
        name,
        variant: `Variant ${variantNumber}`,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitEditorCommands.addDesign(newDesign);
      navigateToDesign(kit.guid, newDesign.guid);
    };

    return (
      <>
        {Object.entries(designsByName).map(([name, designs]) => (
          <div key={name} onPointerEnter={() => hoverDesigns(designs.map((d) => d.guid))} onPointerLeave={() => clearHover()}>
            <TreeItem
              label={name}
              defaultOpen={true}
              actions={[
                {
                  icon: <Plus size={12} />,
                  onClick: () => handleCreateVariant(name),
                  title: "Add variant",
                },
              ]}
            >
              <TreeContent>
                <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                  {designs.map((d: Design) => (
                    <DesignAvatar key={`${d.name}-${d.variant}-${d.view}`} design={d} showHoverCard={true} isActive={design?.guid === d.guid} />
                  ))}
                </div>
              </TreeContent>
            </TreeItem>
          </div>
        ))}
      </>
    );
  };

  // Add toolbar tools
  useEffect(() => {
    if (editorType !== EditorType.DESIGN) return;

    addSection("toolbar", {
      id: "design-tools",
      label: "Tools",
      order: 0,
      content: () => <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "design-tools");
    };
  }, [editorType, addSection, removeSection]);

  useEffect(() => {
    if (editorType !== EditorType.DESIGN) return;

    console.log("[ORIGIN] Design Editor adding workbench sections", { kit, editorType });

    const handleCreateType = () => {
      const existingTypes = kit.types || [];
      const typeNumber = existingTypes.length + 1;
      const newType: Type = {
        guid: guid(),
        name: `Type ${typeNumber}`,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitEditorCommands.addType(newType);
      navigateToType(kit.guid, newType.guid);
    };

    const handleCreateDesign = () => {
      const existingDesigns = kit.designs || [];
      const designNumber = existingDesigns.length + 1;
      const newDesign: Design = {
        guid: guid(),
        name: `Design ${designNumber}`,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitEditorCommands.addDesign(newDesign);
      navigateToDesign(kit.guid, newDesign.guid);
    };

    addSection("workbench", {
      id: "design-types",
      label: "Types",
      order: 0,
      defaultOpen: true,
      content: () => <TypesWorkbenchContent />,
      actions: [
        {
          icon: <Plus size={12} />,
          onClick: handleCreateType,
          title: "Add type",
        },
      ],
    });

    addSection("workbench", {
      id: "design-designs",
      label: "Designs",
      order: 1,
      defaultOpen: true,
      content: () => <DesignsWorkbenchContent />,
      actions: [
        {
          icon: <Plus size={12} />,
          onClick: handleCreateDesign,
          title: "Add design",
        },
      ],
    });

    console.log("[ORIGIN] Design Editor workbench sections added");

    return () => {
      console.log("[ORIGIN] Design Editor removing workbench sections");
      removeSection("workbench", "design-types");
      removeSection("workbench", "design-designs");
    };
  }, [editorType, kit, addSection, removeSection]);

  // Add settings section
  useEffect(() => {
    addSection("settings", {
      id: "design-editor-settings",
      label: "Design Editor",
      order: 100,
      defaultOpen: true,
      content: () => (
        <>
          <TreeItem>
            <TreeContent>
              <div className="flex flex-col gap-1">
                <label>Snappiness: {editorSettings.design?.snappiness}</label>
                <input type="range" min="0" max="20" value={editorSettings.design?.snappiness || 10} className="w-full" readOnly />
              </div>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>Grid Size: {editorSettings.design?.gridSize || 24}px</TreeContent>
          </TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("settings", "design-editor-settings");
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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

      if (activeDraggedType) {
        startTransaction();
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          id_: pieceGuid,
          type: activeDraggedType.guid,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece);
        finalizeTransaction();
      } else if (activeDraggedDesign) {
        startTransaction();
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          id_: pieceGuid,
          design: activeDraggedDesign.guid,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece);
        finalizeTransaction();
      }
    }

    setActiveDraggedType(null);
    setActiveDraggedDesign(null);
  };

  useEffect(() => {
    const listener = (e: Event) => {
      const customEvent = e as CustomEvent<DragEndEvent>;
      handleDragEnd(customEvent.detail);
    };
    window.addEventListener("design-drag-end", listener);
    return () => window.removeEventListener("design-drag-end", listener);
  }, [handleDragEnd]);

  return (
    <ReactFlowProvider>
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel defaultSize={fullscreenWindow === DesignEditorFullscreenWindow.Diagram ? 100 : 50} className={`${fullscreenWindow === DesignEditorFullscreenWindow.Accessl ? "hidden" : "block"}`} onDoubleClick={toggleDiagramFullscreen}>
          <Diagram reactFlowInstanceRef={reactFlowInstanceRef} />
        </ResizablePanel>
        <ResizableHandle className={`border-r ${fullscreenWindow !== DesignEditorFullscreenWindow.None ? "hidden" : "block"}`} />
        <ResizablePanel defaultSize={fullscreenWindow === DesignEditorFullscreenWindow.Accessl ? 100 : 50} className={`${fullscreenWindow === DesignEditorFullscreenWindow.Diagram ? "hidden" : "block"}`}>
          <DesignScene />
        </ResizablePanel>
      </ResizablePanelGroup>
    </ReactFlowProvider>
  );
};

export default Editor;
