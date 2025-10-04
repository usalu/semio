import { FC, useState } from "react";
import { ScrollArea } from "../ScrollArea";
import { Tree, TreeSection } from "../Tree";
import { usePanelSections } from "./PanelSectionContext";
import { ResizablePanelProps } from "./Sketchpad";

interface ConsoleProps extends ResizablePanelProps {
  height: number;
  onHeightChange?: (height: number) => void;
}

const Console: FC<ConsoleProps> = ({ visible, onHeightChange, height }) => {
  if (!visible) return null;
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  // Get sections from context (provided by editor)
  const sections = usePanelSections('console');

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startY = e.clientY;
    const startHeight = height;

    const handleMouseMove = (e: MouseEvent) => {
      const newHeight = startHeight - (e.clientY - startY);
      if (newHeight >= 100 && newHeight <= 600) {
        onHeightChange?.(newHeight);
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
      className={`absolute left-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? "border-t-primary" : "border-t"}`}
      style={{ height: `${height}px` }}
    >
      <div className="absolute top-0 left-0 right-0 h-1 cursor-ns-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            {/* Render sections from context */}
            {sortedSections.map((section) => (
              <TreeSection
                key={section.id}
                label={section.label}
                defaultOpen={section.defaultOpen}
                actions={section.actions}
              >
                {section.content}
              </TreeSection>
            ))}
          </Tree>
        </div>
      </ScrollArea>
    </div>
  );
};

export default Console;
