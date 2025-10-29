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

// #endregion

import { DragEndEvent } from "@dnd-kit/core";
import { ReactFlowInstance } from "@xyflow/react";
import { FC, memo, ReactNode, useEffect, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { guid, Quality } from "../../../semio";
import { Canvas, useCanvasContext, VerticalWindows } from "../../Canvas";
import { useQuality } from "../../kits/store";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { useAppType } from "../../store";
import Diagram from "./canvas/Diagram";
import Formula from "./canvas/Formula";
import { QualityDetails } from "./panels/Details";
import { QualityWorkbench, QualityWorkbenchQualities } from "./panels/Workbench";
import { FormulaNode, QualityAppFullscreenWindow, useQualityApp, useQualityAppCommands } from "./store";

export interface AppProps {}

const CanvasWithSync: FC<{ fullscreenWindow: QualityAppFullscreenWindow; children: ReactNode }> = memo(({ fullscreenWindow, children }) => {
  const { setFullscreenWindow } = useCanvasContext();

  useEffect(() => {
    switch (fullscreenWindow) {
      case QualityAppFullscreenWindow.Formula:
        setFullscreenWindow(QualityAppFullscreenWindow.Formula);
        break;
      case QualityAppFullscreenWindow.Diagram:
        setFullscreenWindow(QualityAppFullscreenWindow.Diagram);
        break;
      default:
        setFullscreenWindow(null);
    }
  }, [fullscreenWindow, setFullscreenWindow]);

  return <>{children}</>;
});

CanvasWithSync.displayName = "CanvasWithSync";

const FormulaWindow = memo(() => <Formula />);
FormulaWindow.displayName = "FormulaWindow";

const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => <Diagram reactFlowInstanceRef={reactFlowInstanceRef} />);
DiagramWindow.displayName = "DiagramWindow";

const App: FC<AppProps> = () => {
  const { t } = useTranslation();
  const fullscreenWindow = useQualityApp((s) => s.fullscreenWindow) as QualityAppFullscreenWindow;
  const { undo, redo, toggleFormulaFullscreen, toggleDiagramFullscreen, deselectAll, togglePanel, addFormulaNode, connectNodes, startTransaction, finalizeTransaction } = useQualityAppCommands();
  const quality = useQuality() as Quality | undefined;
  const appType = useAppType();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  useEffect(() => {
    if (appType !== "quality") return;

    addSection("details", {
      id: "quality-details",
      label: t("semio.sketchpad.app.quality.title"),
      order: 0,
      defaultOpen: true,
      content: () => <QualityDetails />,
    });

    return () => {
      removeSection("details", "quality-details");
    };
  }, [appType, addSection, removeSection, t]);

  useEffect(() => {
    if (appType !== "quality") return;

    addSection("workbench", {
      id: "quality-functions",
      label: t("semio.sketchpad.app.quality.functions"),
      order: 0,
      defaultOpen: true,
      content: () => <QualityWorkbench />,
    });

    addSection("workbench", {
      id: "quality-qualities",
      label: t("semio.sketchpad.app.quality.qualities"),
      order: 1,
      defaultOpen: true,
      content: () => <QualityWorkbenchQualities />,
    });

    return () => {
      removeSection("workbench", "quality-functions");
      removeSection("workbench", "quality-qualities");
    };
  }, [appType, addSection, removeSection, t]);

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over, delta } = event;

    if (over && over.id === "quality-diagram-drop-zone" && reactFlowInstanceRef.current) {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return;
      }

      const { x, y } = reactFlowInstanceRef.current.screenToFlowPosition({
        x: event.activatorEvent.clientX + delta.x,
        y: event.activatorEvent.clientY + delta.y,
      });

      const dragData = active.data.current as any;

      if (dragData) {
        startTransaction();

        // Check if we're dropping on a placeholder node
        const targetNode = reactFlowInstanceRef.current.getNodes().find((n) => {
          const nodeBounds = {
            left: n.position.x,
            right: n.position.x + 48, // node width (12 * 4px = 48px)
            top: n.position.y,
            bottom: n.position.y + 48, // node height
          };
          return x >= nodeBounds.left && x <= nodeBounds.right && y >= nodeBounds.top && y <= nodeBounds.bottom;
        });

        const isPlaceholder = targetNode?.type === "placeholder";
        const parentId = isPlaceholder ? (targetNode?.data as any)?.parentId : undefined;
        const operandIndex = isPlaceholder ? (targetNode?.data as any)?.operandIndex : undefined;

        let node: FormulaNode;

        // Handle quality avatar drops
        if (dragData.quality) {
          node = {
            id: guid(),
            type: "quality",
            name: dragData.quality.key,
            x: isPlaceholder ? 0 : x,
            y: isPlaceholder ? 0 : y,
          };
        }
        // Handle function/variable/unit/value drops
        else if (dragData.type && dragData.name) {
          node = {
            id: guid(),
            type: dragData.type,
            name: dragData.name,
            x: isPlaceholder ? 0 : x,
            y: isPlaceholder ? 0 : y,
          };
        } else {
          finalizeTransaction();
          return;
        }

        addFormulaNode(node);

        // If dropping on placeholder, connect to parent
        if (isPlaceholder && parentId) {
          connectNodes(parentId, node.id);
        }

        finalizeTransaction();
      }
    }
  };

  useEffect(() => {
    const listener = (e: Event) => {
      const customEvent = e as CustomEvent<DragEndEvent>;
      handleDragEnd(customEvent.detail);
    };
    window.addEventListener("quality-drag-end", listener);
    return () => window.removeEventListener("quality-drag-end", listener);
  }, [handleDragEnd]);

  return (
    <Canvas>
      <CanvasWithSync fullscreenWindow={fullscreenWindow}>
        <VerticalWindows
          windows={[
            {
              id: QualityAppFullscreenWindow.Formula,
              children: <FormulaWindow />,
              defaultSize: 20,
              onDoubleClick: toggleFormulaFullscreen,
            },
            {
              id: QualityAppFullscreenWindow.Diagram,
              children: <DiagramWindow reactFlowInstanceRef={reactFlowInstanceRef} />,
              defaultSize: 80,
              onDoubleClick: toggleDiagramFullscreen,
            },
          ]}
        />
      </CanvasWithSync>
    </Canvas>
  );
};

export default App;
