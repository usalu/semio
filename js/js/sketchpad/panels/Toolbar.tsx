// #region Header

// Toolbar.tsx

// Horizontal toolbar panel component positioned at the bottom of the editor.
// Displays dynamic tool sections registered by editors in a horizontal layout.
//
// Architecture:
// - Positioned at bottom, between left and right panels
// - On mobile: full width (no side panels)
// - On desktop: adjusts width based on visible side panels
// - Editors use `useAddPanelSection` to register tool sections dynamically
// - Sections are displayed horizontally with separators
//
// Example usage in an editor:
//   const addSection = useAddPanelSection();
//   useEffect(() => {
//     addSection("toolbar", {
//       id: "my-tools",
//       label: "My Tools",
//       order: 0,
//       content: () => <MyToolButtons />
//     });
//     return () => removeSection("toolbar", "my-tools");
//   }, []);

// #endregion

import { FC, useState } from "react";
import { usePanelSections } from "../Navbar";
import { useActiveInteraction, useIsMobile } from "../store";

interface ToolbarProps {
  visible: boolean;
  onHeightChange?: (height: number) => void;
  height: number;
  leftOffset?: number;
  rightOffset?: number;
}

const Toolbar: FC<ToolbarProps> = ({ visible, onHeightChange, height, leftOffset = 0, rightOffset = 0 }) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();

  const sections = usePanelSections("toolbar");

  if (!visible) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startY = e.clientY;
    const startHeight = height;

    const handleMouseMove = (e: MouseEvent) => {
      const newHeight = startHeight - (e.clientY - startY);
      if (newHeight >= 40 && newHeight <= 200) {
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

  const sortedSections = sections.sort((a, b) => (a.order || 0) - (b.order || 0));

  return (
    <div
      className={`z-20 bg-panel text-foreground border border-t
                ${isResizing || isResizeHovered ? "border-t-primary" : ""}`}
      style={{
        height: `${height}px`,
        marginLeft: `${leftOffset}px`,
        marginRight: `${rightOffset}px`,
        opacity: activeInteraction ? 0.1 : 1,
        transition: "opacity 150ms",
      }}
    >
      <div className="h-full flex items-center overflow-x-auto overflow-y-hidden">
        <div className={`flex items-center gap-2 h-full ${isMobile ? "px-3" : "px-2"}`}>
          {sortedSections.length === 0 ? (
            <div className="text-muted-foreground text-xs py-2">No tools</div>
          ) : (
            sortedSections.map((section, index) => {
              const content = typeof section.content === "function" ? section.content() : section.content;
              return (
                <div key={section.id} className="flex items-center h-full">
                  {index > 0 && <div className="w-px h-6 bg-border mx-1" />}
                  <div className="flex items-center gap-1">{content}</div>
                </div>
              );
            })
          )}
        </div>
      </div>
      <div className="absolute top-0 left-0 right-0 h-1 cursor-ns-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Toolbar;
