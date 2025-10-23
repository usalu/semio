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

// #endregion

import { DragEndEvent } from "@dnd-kit/core";
import { FC, memo, ReactNode, useEffect, useRef } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { ReactFlowInstance, ReactFlowProvider } from "@xyflow/react";
import { guid, Quality } from "../../../semio";
import { Canvas, useCanvasContext, VerticalWindows } from "../../Canvas";
import { useQuality } from "../../kits/store";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { EditorType, useEditorType } from "../../store";
import Diagram from "./canvas/Diagram";
import Formula from "./canvas/Formula";
import { QualityDetails } from "./panels/Details";
import { QualityWorkbench } from "./panels/Workbench";
import { FormulaNode, QualityEditorFullscreenWindow, useQualityEditor, useQualityEditorCommands } from "./store";

export interface EditorProps {}

const CanvasWithSync: FC<{ fullscreenWindow: QualityEditorFullscreenWindow; children: ReactNode }> = memo(({ fullscreenWindow, children }) => {
  const { setFullscreenWindow } = useCanvasContext();

  useEffect(() => {
    switch (fullscreenWindow) {
      case QualityEditorFullscreenWindow.Formula:
        setFullscreenWindow(QualityEditorFullscreenWindow.Formula);
        break;
      case QualityEditorFullscreenWindow.Diagram:
        setFullscreenWindow(QualityEditorFullscreenWindow.Diagram);
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

const Editor: FC<EditorProps> = () => {
  const { t } = useTranslation();
  const fullscreenWindow = useQualityEditor((s) => s.fullscreenWindow) as QualityEditorFullscreenWindow;
  const { undo, redo, toggleFormulaFullscreen, toggleDiagramFullscreen, deselectAll, togglePanel, addFormulaNode, startTransaction, finalizeTransaction } = useQualityEditorCommands();
  const quality = useQuality() as Quality | undefined;
  const editorType = useEditorType();

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  useHotkeys("ctrl+d", () => deselectAll());
  useHotkeys("ctrl+z", () => undo());
  useHotkeys("ctrl+y", () => redo());
  useHotkeys("ctrl+shift+z", () => redo());

  useEffect(() => {
    if (editorType !== EditorType.QUALITY) return;

    addSection("details", {
      id: "quality-details",
      label: t("quality.title"),
      order: 0,
      defaultOpen: true,
      content: () => <QualityDetails />,
    });

    return () => {
      removeSection("details", "quality-details");
    };
  }, [editorType, addSection, removeSection, t]);

  useEffect(() => {
    if (editorType !== EditorType.QUALITY) return;

    addSection("workbench", {
      id: "quality-functions",
      label: t("quality.functions"),
      order: 0,
      defaultOpen: true,
      content: () => <QualityWorkbench />,
    });

    return () => {
      removeSection("workbench", "quality-functions");
    };
  }, [editorType, addSection, removeSection, t]);

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

      const dragData = active.data.current as { name: string; type: "function" | "quality" | "variable" | "unit" | "value" };
      if (dragData) {
        startTransaction();
        const node: FormulaNode = {
          id: guid(),
          type: dragData.type,
          name: dragData.name,
          x,
          y,
        };
        addFormulaNode(node);
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
    <ReactFlowProvider>
      <Canvas>
        <CanvasWithSync fullscreenWindow={fullscreenWindow}>
          <VerticalWindows
            windows={[
              {
                id: QualityEditorFullscreenWindow.Formula,
                children: <FormulaWindow />,
                defaultSize: 20,
                onDoubleClick: toggleFormulaFullscreen,
              },
              {
                id: QualityEditorFullscreenWindow.Diagram,
                children: <DiagramWindow reactFlowInstanceRef={reactFlowInstanceRef} />,
                defaultSize: 80,
                onDoubleClick: toggleDiagramFullscreen,
              },
            ]}
          />
        </CanvasWithSync>
      </Canvas>
    </ReactFlowProvider>
  );
};

export default Editor;
