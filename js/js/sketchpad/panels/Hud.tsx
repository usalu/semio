import { FC, useState } from "react";
import { ScrollArea } from "../../elements/aggregation/ScrollArea";
import { Tree, TreeSection } from "../../elements/aggregation/Tree";
import { TreeStateProvider } from "../../elements/aggregation/TreeStateProvider";
import { usePanelSections } from "../Navbar";
import { ResizablePanelProps } from "../Sketchpad";
import { useActiveInteraction, useIsMobile } from "../store";

interface HudProps extends ResizablePanelProps {}

const Hud: FC<HudProps> = ({ visible, onWidthChange, width }) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();
  const sections = usePanelSections("hud");

  if (!visible) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth + (e.clientX - startX);
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
      className={`h-full z-30 text-foreground border
                ${isResizing || isResizeHovered ? "border-r-accent" : "border-r"}`}
      style={{ width: `${width}px`, opacity: activeInteraction ? 0.1 : 1, transition: "opacity 150ms" }}
    >
      <ScrollArea className="h-full">
        <div className={isMobile ? "p-2" : "p-1"}>
          {sortedSections.length === 0 ? (
            <div className="p-4 text-center text-muted-foreground">No hud sections available</div>
          ) : (
            <TreeStateProvider>
              <Tree>
                {sortedSections.map((section) => {
                  const content = typeof section.content === "function" ? section.content() : section.content;
                  return (
                    <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                      {content}
                    </TreeSection>
                  );
                })}
              </Tree>
            </TreeStateProvider>
          )}
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Hud;
