import { FC, useState } from "react";

import { ScrollArea } from "@semio/js/components/ui/ScrollArea";
import { Tree, TreeItem, TreeSection } from "@semio/js/components/ui/Tree";
import { usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";

interface DetailsProps extends ResizablePanelProps {}

const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null;
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const sections = usePanelSections("details");

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
            {sortedSections.map((section) => (
              <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                {section.content}
              </TreeSection>
            ))}

            {sortedSections.length === 0 && (
              <TreeSection label="No Selection" defaultOpen={true}>
                <TreeItem>
                  <p className="text-sm text-muted-foreground">Select an item to view details</p>
                </TreeItem>
              </TreeSection>
            )}
          </Tree>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Details;
