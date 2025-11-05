// #region Header

// App.tsx

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
// The panel system allows ANY component (including nested apps) to be mounted as a section.
// Example of nesting a design app as a section:
//   addSection("workbench", {
//     id: "nested-design-app",
//     label: "Nested Design",
//     order: 10,
//     defaultOpen: false,
//     content: () => (
//       <DesignScopeProvider guid={someDesignGuid}>
//         <App />
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
import { Design, findConnectionsInDesign, generateUniqueName, guid, ICON_WIDTH, Kit, Type } from "../../../semio";
import { Canvas, HorizontalWindows, useCanvasContext } from "../../Canvas";
import { useDesign, useKit } from "../../kits/store";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useDragDrop } from "../../Sketchpad";
import { ToolType, useAppPanelVisibility, useAppType, useSketchpad, useSketchpadCommands } from "../../store";
import { KitSection } from "../kit/panels/Details";
import { useKitAppCommands } from "../kit/store";
import Diagram from "./canvas/Diagram";
import DesignScene from "./canvas/Scene";
import { DesignAppFooter } from "./Footer";
import { ConnectionsSection, DesignSection, PiecesSection, PortSection } from "./panels/Details";
import { DesignAppFullscreenWindow, useDesignApp, useDesignAppCommands, useDesignAppFullscreen, useDesignAppSelection } from "./store";
import { ToolsToggleGroup } from "./Tools";

export interface AppProps {}

const CanvasWithSync: FC<{ fullscreenWindow: DesignAppFullscreenWindow; children: ReactNode }> = memo(({ fullscreenWindow, children }) => {
  const { setFullscreenWindow } = useCanvasContext();

  useEffect(() => {
    switch (fullscreenWindow) {
      case DesignAppFullscreenWindow.Diagram:
        setFullscreenWindow(DesignAppFullscreenWindow.Diagram);
        break;
      case DesignAppFullscreenWindow.Accessl:
        setFullscreenWindow(DesignAppFullscreenWindow.Accessl);
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

const App: FC<AppProps> = () => {
  const { t } = useTranslation();
  const fullscreenWindow = useDesignAppFullscreen();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleAccesslFullscreen, addPiece, startTransaction, finalizeTransaction, togglePanel, setActiveTool, hoverTypes, hoverDesigns, clearHover } =
    useDesignAppCommands();
  const app = useDesignApp((s) => s);
  const activeTool = app?.activeTool ?? ToolType.SELECTION_NORMAL;

  const selection = useDesignAppSelection();
  const design = useDesign() as Design | undefined;
  const kit = useKit() as Kit;
  const appSettings = useSketchpad((s) => s.settings?.apps) as any;
  const panelVisibility = useAppPanelVisibility();
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const kitAppCommands = useKitAppCommands();
  const { navigateToType, navigateToDesign, navigateToKit } = useSketchpadCommands();

  useHotkeys("ctrl+a", () => selectAll());
  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("delete", () => deleteSelected());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  const appType = useAppType();

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
    if (appType !== "design") return;

    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.port !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    const pieceSingleId = "semio.sketchpad.app.design.panel.details.section.piece.title";
    const pieceMultipleId = "semio.sketchpad.app.design.panel.details.section.piece.multipleTitle";
    const connectionSingleId = "semio.sketchpad.app.design.panel.details.section.connection.title";
    const connectionMultipleId = "semio.sketchpad.app.design.panel.details.section.connection.multipleTitle";
    const selectionMultipleId = "semio.sketchpad.app.design.panel.details.section.selection.multipleTitle";

    removeSection("details", "semio.sketchpad.app.design.title");
    removeSection("details", "semio.sketchpad.app.type.port.title");
    removeSection("details", pieceSingleId);
    removeSection("details", pieceMultipleId);
    removeSection("details", connectionSingleId);
    removeSection("details", connectionMultipleId);
    removeSection("details", selectionMultipleId);
    removeSection("details", "semio.sketchpad.app.kit.title");

    if (!hasSelection) {
      addSection("details", {
        id: "semio.sketchpad.app.design.title",
        order: 50,
        content: () => <DesignSection />,
      });
    } else if (hasPortSelected) {
      const portPieceId = selection.port!.piece;
      const portId = selection.port!.port;
      addSection("details", {
        id: "semio.sketchpad.app.type.port.title",
        order: 0,
        content: () => <PortSection pieceGuid={portPieceId} portGuid={portId} />,
      });
      addSection("details", {
        id: "semio.sketchpad.app.design.title",
        order: 50,
        content: () => <DesignSection />,
      });
    } else {
      if (hasPieces) {
        const piecesCount = selection.pieces!.length;
        const piecesSectionId = piecesCount === 1 ? pieceSingleId : pieceMultipleId;
        addSection("details", {
          id: piecesSectionId,
          translationParams: piecesCount === 1 ? undefined : { count: piecesCount },
          order: 0,
          content: () => <PiecesSection />,
        });
      }
      if (hasConnections) {
        const connGuids = selection.connections!;
        const conns = findConnectionsInDesign(design!, connGuids);
        const connectionsSectionId = conns.length === 1 ? connectionSingleId : connectionMultipleId;
        addSection("details", {
          id: connectionsSectionId,
          translationParams: conns.length === 1 ? undefined : { count: conns.length },
          order: 10,
          content: () => <ConnectionsSection connections={conns} />,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: selectionMultipleId,
          order: 20,
          content: () => (
            <TreeItem>
              <TreeContent>
                <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.design.selectOnlyPiecesOrConnections")}</p>
              </TreeContent>
            </TreeItem>
          ),
        });
      }
      addSection("details", {
        id: "semio.sketchpad.app.design.title",
        order: 50,
        content: () => <DesignSection />,
      });
    }

    addSection("details", {
      id: "semio.sketchpad.app.kit.title",
      order: 100,
      content: () => <KitSection />,
    });

    return () => {
      removeSection("details", "semio.sketchpad.app.design.title");
      removeSection("details", "semio.sketchpad.app.type.port.title");
      removeSection("details", pieceSingleId);
      removeSection("details", pieceMultipleId);
      removeSection("details", connectionSingleId);
      removeSection("details", connectionMultipleId);
      removeSection("details", selectionMultipleId);
      removeSection("details", "semio.sketchpad.app.kit.title");
    };
  }, [selection, addSection, removeSection, appType, t, design]);

  const TypesWorkbenchContent: FC = () => {
    const typesByName = (kit.types || []).reduce((acc: Record<string, Type[]>, type: Type) => {
      if (!acc[type.name]) acc[type.name] = [];
      acc[type.name].push(type);
      return acc;
    }, {});

    const handleCreateChild = (parentType: Type) => {
      const existingChildren = kit.types?.filter((t) => t.parent === parentType.guid) || [];
      const uniqueName = generateUniqueName(
        parentType.name,
        existingChildren.map((t) => t.name),
      );
      const newType: Type = {
        guid: guid(),
        name: uniqueName,
        parent: parentType.guid,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitAppCommands.addType("semio.sketchpad.app.design.panel.workbench.types.createChild", newType);
      navigateToType(kit.guid, newType.guid);
    };

    const renderTypeTree = (types: Type[]): ReactNode[] => {
      return types.map((type) => {
        const children = kit.types?.filter((t) => t.parent === type.guid) || [];
        return (
          <div key={type.guid} onPointerEnter={() => hoverTypes("semio.sketchpad.app.design.panel.workbench.types.hover", [type.guid])} onPointerLeave={() => clearHover("semio.sketchpad.app.design.panel.workbench.types.leave")}>
            <TreeItem
              label={type.name}
              onDoubleClick={(event) => {
                if ((event.target as HTMLElement).closest('[data-slot="action"]')) {
                  return;
                }
                event.preventDefault();
                event.stopPropagation();
                navigateToType(kit.guid, type.guid);
              }}
              actions={[
                {
                  icon: <Plus size={12} />,
                  onClick: () => handleCreateChild(type),
                  id: "semio.sketchpad.common.addChild",
                },
              ]}
            >
              {children.length > 0 && renderTypeTree(children)}
            </TreeItem>
          </div>
        );
      });
    };

    const rootTypes = kit.types?.filter((t) => !t.parent) || [];

    return <>{renderTypeTree(rootTypes)}</>;
  };

  const DesignsWorkbenchContent: FC = () => {
    const handleCreateChild = (parentDesign: Design) => {
      const existingChildren = kit.designs?.filter((d) => d.parent === parentDesign.guid) || [];
      const uniqueName = generateUniqueName(
        parentDesign.name,
        existingChildren.map((d) => d.name),
      );
      const newDesign: Design = {
        guid: guid(),
        name: uniqueName,
        parent: parentDesign.guid,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitAppCommands.addDesign("semio.sketchpad.app.design.panel.workbench.designs.createChild", newDesign);
      navigateToDesign(kit.guid, newDesign.guid);
    };

    const renderDesignTree = (designs: Design[]): ReactNode[] => {
      return designs.map((d) => {
        const children = kit.designs?.filter((child) => child.parent === d.guid) || [];
        return (
          <div key={d.guid} onPointerEnter={() => hoverDesigns("semio.sketchpad.app.design.panel.workbench.designs.hover", [d.guid])} onPointerLeave={() => clearHover("semio.sketchpad.app.design.panel.workbench.designs.leave")}>
            <TreeItem
              label={d.name}
              onDoubleClick={(event) => {
                if ((event.target as HTMLElement).closest('[data-slot="action"]')) {
                  return;
                }
                event.preventDefault();
                event.stopPropagation();
                navigateToDesign(kit.guid, d.guid);
              }}
              actions={[
                {
                  icon: <Plus size={12} />,
                  onClick: () => handleCreateChild(d),
                  id: "semio.sketchpad.common.addChild",
                },
              ]}
            >
              {children.length > 0 && renderDesignTree(children)}
            </TreeItem>
          </div>
        );
      });
    };

    const rootDesigns = kit.designs?.filter((d) => !d.parent) || [];

    return <>{renderDesignTree(rootDesigns)}</>;
  };

  // Add toolbar tools
  useEffect(() => {
    if (appType !== "design") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.design.tools",
      order: 0,
      content: <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.design.tools");
    };
  }, [appType, addSection, removeSection]);

  useEffect(() => {
    if (appType !== "design") return;
    const handleCreateType = () => {
      const existingTypes = kit.types || [];
      const typeNumber = existingTypes.length + 1;
      const newType: Type = {
        guid: guid(),
        name: `Type ${typeNumber}`,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      kitAppCommands.addType("semio.sketchpad.app.design.panel.workbench.types.create", newType);
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
      kitAppCommands.addDesign("semio.sketchpad.app.design.panel.workbench.designs.create", newDesign);
      navigateToDesign(kit.guid, newDesign.guid);
    };

    addSection("workbench", {
      id: "semio.sketchpad.app.kit.types",
      order: 0,
      content: () => <TypesWorkbenchContent />,
      actions: [
        {
          icon: <Plus size={12} />,
          onClick: handleCreateType,
          id: "semio.sketchpad.common.addType",
        },
      ],
      onPointerEnter: () => {
        if (!kit.types || kit.types.length === 0) return;
        hoverTypes(
          "semio.sketchpad.app.design.panel.workbench.typesSection.hover",
          kit.types.map((type) => type.guid),
        );
      },
      onPointerLeave: () => clearHover("semio.sketchpad.app.design.panel.workbench.typesSection.leave"),
      onDoubleClick: () => {
        if (!kit?.guid) return;
        navigateToKit(kit.guid, "kind=types");
      },
    });

    addSection("workbench", {
      id: "semio.sketchpad.app.kit.designs",
      order: 1,
      content: () => <DesignsWorkbenchContent />,
      actions: [
        {
          icon: <Plus size={12} />,
          onClick: handleCreateDesign,
          id: "semio.sketchpad.common.addDesign",
        },
      ],
      onPointerEnter: () => {
        if (!kit.designs || kit.designs.length === 0) return;
        hoverDesigns(
          "semio.sketchpad.app.design.panel.workbench.designsSection.hover",
          kit.designs.map((design) => design.guid),
        );
      },
      onPointerLeave: () => clearHover("semio.sketchpad.app.design.panel.workbench.designsSection.leave"),
      onDoubleClick: () => {
        if (!kit?.guid) return;
        navigateToKit(kit.guid, "kind=designs");
      },
    });
    return () => {
      removeSection("workbench", "semio.sketchpad.app.kit.types");
      removeSection("workbench", "semio.sketchpad.app.kit.designs");
    };
  }, [appType, kit.types, kit.designs]);

  // Add settings section
  useEffect(() => {
    addSection("settings", {
      id: "semio.sketchpad.app.design.appTitle",
      order: 100,
      content: () => (
        <>
          <TreeItem>
            <TreeContent>
              <div className="flex flex-col gap-1">
                <label>
                  {t("semio.sketchpad.app.design.proximityConnectDistance")}: {appSettings.design?.proximityConnectDistance}
                </label>
                <input type="range" min="0" max="20" value={appSettings.design?.proximityConnectDistance || 10} className="w-full" readOnly />
              </div>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              {t("semio.sketchpad.app.design.gridSize")}: {appSettings.design?.gridSize || 24}px
            </TreeContent>
          </TreeItem>
        </>
      ),
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.design.appTitle");
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
        startTransaction("semio.sketchpad.app.design.dragEnd.type");
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          id_: pieceGuid,
          type: activeDraggedType.guid,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece("semio.sketchpad.app.design.dragEnd.type", piece);
        finalizeTransaction("semio.sketchpad.app.design.dragEnd.type");
      } else if (activeDraggedDesign) {
        startTransaction("semio.sketchpad.app.design.dragEnd.design");
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          id_: pieceGuid,
          design: activeDraggedDesign.guid,
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece("semio.sketchpad.app.design.dragEnd.design", piece);
        finalizeTransaction("semio.sketchpad.app.design.dragEnd.design");
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
                id: DesignAppFullscreenWindow.Diagram,
                children: <DiagramWindow reactFlowInstanceRef={reactFlowInstanceRef} />,
                defaultSize: 50,
                onDoubleClick: toggleDiagramFullscreen,
              },
              {
                id: DesignAppFullscreenWindow.Accessl,
                children: <SceneWindow />,
                defaultSize: 50,
              },
            ]}
          />
        </CanvasWithSync>
      </Canvas>
      <DesignAppFooter />
    </ReactFlowProvider>
  );
};

export default App;
