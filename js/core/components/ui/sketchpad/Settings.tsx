import { TreeItem, TreeSection } from "../Tree";

import { FingerprintIcon, Laptop, MonitorIcon, MoonIcon, SunIcon } from "lucide-react";
import { FC, useState } from "react";
import { Theme, useEditorType, useLayout, useSketchpad, useSketchpadCommands, useTheme } from "../../../store";
import { ScrollArea } from "../ScrollArea";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";
import { Tree } from "../Tree";
import { usePanelSections } from "./PanelSectionContext";
import { ResizablePanelProps } from "./Sketchpad";

interface SettingsProps extends ResizablePanelProps {}

const Settings: FC<SettingsProps> = ({ visible, onWidthChange, width }) => {
  const theme = useTheme();
  const layout = useLayout();

  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const editorType = useEditorType();
  const editorSettings = useSketchpad((s) => s.editorSettings);
  const { setTheme, setLayout, updateEditorSettings } = useSketchpadCommands();

  // Get editor-specific sections from context
  const sections = usePanelSections("settings");

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

  // Sort sections by order
  const sortedSections = sections.sort((a, b) => (a.order || 0) - (b.order || 0));

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
              <TreeItem label="Theme">
                <ToggleGroup type="single" value={theme} onValueChange={(value) => setTheme(value as Theme)}>
                  <ToggleGroupItem value="system">
                    <Laptop />
                  </ToggleGroupItem>
                  <ToggleGroupItem value="light">
                    <SunIcon />
                  </ToggleGroupItem>
                  <ToggleGroupItem value="dark">
                    <MoonIcon />
                  </ToggleGroupItem>
                </ToggleGroup>
              </TreeItem>
              <TreeItem label="Layout">
                <ToggleGroup type="single" value={layout} onValueChange={(value) => setLayout(value as Layout)}>
                  <ToggleGroupItem value="normal">
                    <MonitorIcon />
                  </ToggleGroupItem>
                  <ToggleGroupItem value="touch">
                    <FingerprintIcon />
                  </ToggleGroupItem>
                </ToggleGroup>
              </TreeItem>
            </TreeSection>

            {/* Editor-specific sections from context */}
            {sortedSections.map((section) => (
              <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                {section.content}
              </TreeSection>
            ))}
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Settings;
