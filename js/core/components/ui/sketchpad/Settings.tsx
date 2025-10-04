import { TreeItem, TreeSection } from "../Tree";

import { FC, useState } from "react";
import { EditorType, useEditorType, useSketchpad, useSketchpadCommands } from "../../../store";
import { ScrollArea } from "../ScrollArea";
import { Tree } from "../Tree";
import { ResizablePanelProps } from "./Sketchpad";

interface SettingsProps extends ResizablePanelProps {}

const Settings: FC<SettingsProps> = ({ visible, onWidthChange, width }) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const editorType = useEditorType();
  const editorSettings = useSketchpad((s) => s.editorSettings);
  const { updateEditorSettings } = useSketchpadCommands();

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX);
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth);
      }
    };

    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border min-w-0 overflow-hidden
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1 overflow-hidden min-w-0">
          <Tree className="min-w-0 overflow-hidden">
            {/* General Settings - always shown */}
            <TreeSection label="General" defaultOpen={true}>
              <TreeItem>Theme: System</TreeItem>
              <TreeItem>Layout: Normal</TreeItem>
            </TreeSection>

            {/* Design Editor Settings */}
            {editorType === EditorType.DESIGN && editorSettings.design && (
              <TreeSection label="Design Editor" defaultOpen={true}>
                <TreeItem>
                  <div className="flex flex-col gap-2">
                    <label>Snappiness: {editorSettings.design.snappiness}</label>
                    <input
                      type="range"
                      min="0"
                      max="20"
                      value={editorSettings.design.snappiness}
                      onChange={(e) => updateEditorSettings('design', { snappiness: Number(e.target.value) })}
                      className="w-full"
                    />
                  </div>
                </TreeItem>
                <TreeItem>Grid Size: {editorSettings.design.gridSize}px</TreeItem>
              </TreeSection>
            )}

            {/* Type Editor Settings */}
            {editorType === EditorType.TYPE && (
              <TreeSection label="Type Editor" defaultOpen={true}>
                <TreeItem>Type-specific settings here</TreeItem>
              </TreeSection>
            )}

            {/* Kit Editor Settings */}
            {editorType === EditorType.KIT && (
              <TreeSection label="Kit Editor" defaultOpen={true}>
                <TreeItem>Kit-specific settings here</TreeItem>
              </TreeSection>
            )}
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Settings;
