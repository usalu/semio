// #region Header

// Editor.tsx

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
import { FC, memo, ReactNode, useEffect, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";

import { ReactFlowInstance, ReactFlowProvider } from "@xyflow/react";
import { TreeContent, TreeItem } from "../../../elements/aggregation/Tree";
import { Design, findConnectionsInDesign, guid, ICON_WIDTH, Kit, Type } from "../../../semio";
import { Canvas, HorizontalWindows, useCanvasContext } from "../../Canvas";
import { useDesign, useKit } from "../../kits/store";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useDragDrop } from "../../Sketchpad";
import { EditorType, ToolType, useEditorPanelVisibility, useEditorType, useSketchpad, useSketchpadCommands } from "../../store";
import { useKitEditorCommands } from "../kit/store";
import Diagram from "./canvas/Diagram";
import DesignScene from "./canvas/Scene";
import { ConnectionsSection, DesignSection, PiecesSection, PortSection } from "./panels/Details";
import { KitSection } from "../kit/panels/Details";
import { DesignAvatar, TypeAvatar } from "./panels/Workbench";
import { DesignEditorFullscreenWindow, useDesignEditor, useDesignEditorCommands, useDesignEditorFullscreen, useDesignEditorSelection } from "./store";
import { ToolsToggleGroup } from "./Tools";

export interface EditorProps {}

const CanvasWithSync: FC<{ fullscreenWindow: DesignEditorFullscreenWindow; children: ReactNode }> = memo(({ fullscreenWindow, children }) => {
  const { setFullscreenWindow } = useCanvasContext();

  useEffect(() => {
    switch (fullscreenWindow) {
      case DesignEditorFullscreenWindow.Diagram:
        setFullscreenWindow(DesignEditorFullscreenWindow.Diagram);
        break;
      case DesignEditorFullscreenWindow.Accessl:
        setFullscreenWindow(DesignEditorFullscreenWindow.Accessl);
        break;
      default:
        setFullscreenWindow(null);
    }
  }, [fullscreenWindow, setFullscreenWindow]);

  return <>{children}</>;
});

CanvasWithSync.displayName = "CanvasWithSync";

const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => <Diagram reactFlowInstanceRef={reactFlowInstanceRef} />);
DiagramWindow.displayName = "DiagramWindow";

const SceneWindow = memo(() => <DesignScene />);
SceneWindow.displayName = "SceneWindow";

const Editor: FC<EditorProps> = () => {
  const { t } = useTranslation();
  const fullscreenWindow = useDesignEditorFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleAccesslFullscreen, addPiece, startTransaction, finalizeTransaction, togglePanel, setActiveTool, hoverTypes, hoverDesigns, clearHover } =
    useDesignEditorCommands();
  const editor = useDesignEditor((s) => s);
  const activeTool = editor?.activeTool ?? ToolType.SELECTION_NORMAL;

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
  const { navigateToType, navigateToDesign, navigateToKit } = useSketchpadCommands();

  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  const editorType = useEditorType();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (activeTool === ToolType.SELECTION_NORMAL) {
        if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
          setActiveTool(ToolType.SELECTION_ADDITIVE);
        } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
          setActiveTool(ToolType.SELECTION_SUBTRACTIVE);
        }
      }
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (activeTool === ToolType.SELECTION_ADDITIVE && !e.shiftKey) {
        setActiveTool(ToolType.SELECTION_NORMAL);
      } else if (activeTool === ToolType.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
        setActiveTool(ToolType.SELECTION_NORMAL);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  // Add/remove details panel sections based on selection
  useEffect(() => {
    if (editorType !== "design") return;

    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.port !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    removeSection("details", "design-details");
    removeSection("details", "design-port");
    removeSection("details", "design-pieces");
    removeSection("details", "design-connections");
    removeSection("details", "design-mixed");
    removeSection("details", "design-kit");

    if (!hasSelection) {
      addSection("details", {
        id: "design-details",
        label: t("design.title"),
        order: 50,
        defaultOpen: true,
        content: () => <DesignSection />,
      });
    } else if (hasPortSelected) {
      const portPieceId = selection.port!.piece;
      const portId = selection.port!.port;
      addSection("details", {
        id: "design-port",
        label: t("port.title"),
        order: 0,
        defaultOpen: true,
        content: () => <PortSection pieceGuid={portPieceId} portGuid={portId} />,
      });
      addSection("details", {
        id: "design-details",
        label: t("design.title"),
        order: 50,
        defaultOpen: true,
        content: () => <DesignSection />,
      });
    } else {
      if (hasPieces) {
        addSection("details", {
          id: "design-pieces",
          label: selection.pieces!.length === 1 ? t("piece.piece") : t("pieces.multipleTitle"),
          order: 0,
          defaultOpen: true,
          content: () => <PiecesSection />,
        });
      }
      if (hasConnections) {
        const connGuids = selection.connections!;
        const conns = findConnectionsInDesign(design!, connGuids);
        addSection("details", {
          id: "design-connections",
          label: conns.length === 1 ? t("connection.title") : t("connections.multipleTitle"),
          order: 10,
          defaultOpen: true,
          content: () => <ConnectionsSection connections={conns} />,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: "design-mixed",
          label: t("selection.multipleTitle"),
          order: 20,
          defaultOpen: true,
          content: () => (
            <TreeItem>
              <TreeContent>
                <p className="text-sm text-muted-foreground">{t("design.selectOnlyPiecesOrConnections")}</p>
              </TreeContent>
            </TreeItem>
          ),
        });
      }
      addSection("details", {
        id: "design-details",
        label: t("design.title"),
        order: 50,
        defaultOpen: true,
        content: () => <DesignSection />,
      });
    }

    addSection("details", {
      id: "design-kit",
      label: t("kit.title"),
      order: 100,
      defaultOpen: true,
      content: () => <KitSection />,
    });

    return () => {
      removeSection("details", "design-details");
      removeSection("details", "design-port");
      removeSection("details", "design-pieces");
      removeSection("details", "design-connections");
      removeSection("details", "design-mixed");
      removeSection("details", "design-kit");
    };
  }, [selection, addSection, removeSection, editorType, t, design]);

  const TypesWorkbenchContent: FC = () => {
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
              onDoubleClick={(event) => {
                if ((event.target as HTMLElement).closest('[data-slot="action"]')) {
                  return;
                }
                event.preventDefault();
                event.stopPropagation();
                if (!kit?.guid) {
                  return;
                }
                navigateToKit(kit.guid, `kind=types&name=${encodeURIComponent(name)}`);
              }}
              actions={[
                {
                  icon: <Plus size={12} />,
                  onClick: () => handleCreateVariant(name),
                  title: t("common.addVariant"),
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
              onDoubleClick={(event) => {
                if ((event.target as HTMLElement).closest('[data-slot="action"]')) {
                  return;
                }
                event.preventDefault();
                event.stopPropagation();
                if (!kit?.guid) {
                  return;
                }
                navigateToKit(kit.guid, `kind=designs&name=${encodeURIComponent(name)}`);
              }}
              actions={[
                {
                  icon: <Plus size={12} />,
                  onClick: () => handleCreateVariant(name),
                  title: t("common.addVariant"),
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
    if (editorType !== "design") return;

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
    if (editorType !== "design") return;
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
      label: t("kitEditor.types"),
      order: 0,
      defaultOpen: true,
      content: () => <TypesWorkbenchContent />,
      actions: [
        {
          icon: <Plus size={12} />,
          onClick: handleCreateType,
          title: t("common.addType"),
        },
      ],
      onPointerEnter: () => {
        if (!kit.types || kit.types.length === 0) return;
        hoverTypes(kit.types.map((type) => type.guid));
      },
      onPointerLeave: () => clearHover(),
      onDoubleClick: () => {
        if (!kit?.guid) return;
        navigateToKit(kit.guid, "kind=types");
      },
    });

    addSection("workbench", {
      id: "design-designs",
      label: t("kitEditor.designs"),
      order: 1,
      defaultOpen: true,
      content: () => <DesignsWorkbenchContent />,
      actions: [
        {
          icon: <Plus size={12} />,
          onClick: handleCreateDesign,
          title: t("common.addDesign"),
        },
      ],
      onPointerEnter: () => {
        if (!kit.designs || kit.designs.length === 0) return;
        hoverDesigns(kit.designs.map((design) => design.guid));
      },
      onPointerLeave: () => clearHover(),
      onDoubleClick: () => {
        if (!kit?.guid) return;
        navigateToKit(kit.guid, "kind=designs");
      },
    });
    return () => {
      removeSection("workbench", "design-types");
      removeSection("workbench", "design-designs");
    };
  }, [editorType, kit.types, kit.designs]);

  // Add settings section
  useEffect(() => {
    addSection("settings", {
      id: "design-editor-settings",
      label: t("design.editorTitle"),
      order: 100,
      defaultOpen: true,
      content: () => (
        <>
          <TreeItem>
            <TreeContent>
              <div className="flex flex-col gap-1">
                <label>
                  {t("design.snappiness")}: {editorSettings.design?.snappiness}
                </label>
                <input type="range" min="0" max="20" value={editorSettings.design?.snappiness || 10} className="w-full" readOnly />
              </div>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              {t("design.gridSize")}: {editorSettings.design?.gridSize || 24}px
            </TreeContent>
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
      <Canvas>
        <CanvasWithSync fullscreenWindow={fullscreenWindow}>
          <HorizontalWindows
            windows={[
              {
                id: DesignEditorFullscreenWindow.Diagram,
                children: <DiagramWindow reactFlowInstanceRef={reactFlowInstanceRef} />,
                defaultSize: 50,
                onDoubleClick: toggleDiagramFullscreen,
              },
              {
                id: DesignEditorFullscreenWindow.Accessl,
                children: <SceneWindow />,
                defaultSize: 50,
              },
            ]}
          />
        </CanvasWithSync>
      </Canvas>
    </ReactFlowProvider>
  );
};

export default Editor;
