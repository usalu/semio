// #region Header

// Workbench.tsx

// Generalized panel component that displays dynamic sections registered by editors.
// This panel serves as a mountable container where different editors can inject their own content.
//
// Architecture:
// - Editors use `useAddPanelSection` to register sections dynamically
// - Each section has an id, label, order, defaultOpen state, and content (ReactNode or render function)
// - Sections are automatically sorted by order and rendered as TreeSections
// - The panel system is used by: Workbench, Details, Settings, and Chat panels
//
// Example usage in an editor:
//   const addSection = useAddPanelSection();
//   useEffect(() => {
//     addSection("workbench", {
//       id: "my-section",
//       label: "My Section",
//       order: 0,
//       defaultOpen: true,
//       content: () => <MyComponent />
//     });
//     return () => removeSection("workbench", "my-section");
//   }, []);

// #endregion

import { FC, ReactNode, useState } from "react";
import { useParams } from "react-router";

import { ScrollArea } from "../../elements/aggregation/ScrollArea";
import { Tree, TreeSection } from "../../elements/aggregation/Tree";
import { usePanelSections } from "../Navbar";
import { ResizablePanelProps } from "../Sketchpad";
import { DesignScopeProvider, KitScopeProvider, TypeScopeProvider, useActiveInteraction, useIsMobile } from "../store";

interface WorkbenchProps extends ResizablePanelProps {}

const ScopedContent: FC<{ children: ReactNode }> = ({ children }) => {
  const { kit, design, type } = useParams();
  if (design && kit) {
    return (
      <KitScopeProvider guid={kit}>
        <DesignScopeProvider guid={design}>{children}</DesignScopeProvider>
      </KitScopeProvider>
    );
  }
  if (type && kit) {
    return (
      <KitScopeProvider guid={kit}>
        <TypeScopeProvider guid={type}>{children}</TypeScopeProvider>
      </KitScopeProvider>
    );
  }
  if (kit) {
    return <KitScopeProvider guid={kit}>{children}</KitScopeProvider>;
  }
  return <>{children}</>;
};

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width }) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();

  console.log("[ORIGIN] Workbench render, activeInteraction:", activeInteraction);

  const sections = usePanelSections("workbench");

  console.log("[ORIGIN] Workbench sections:", sections, "isMobile:", isMobile, "visible:", visible);

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

  console.log(
    "[ORIGIN] Workbench rendering, sortedSections:",
    sortedSections.map((s) => ({ id: s.id, label: s.label })),
  );

  return (
    <div
      className={`h-full z-20 bg-panel text-foreground border
                ${isResizing || isResizeHovered ? "border-r-accent" : "border-r"}`}
      style={{ width: `${width}px`, opacity: activeInteraction ? 0.1 : 1, transition: "opacity 150ms" }}
    >
      <ScrollArea className="h-full">
        <div className={isMobile ? "p-2" : "p-1"}>
          <ScopedContent>
            {sortedSections.length === 0 ? (
              <div className="p-4 text-center text-muted-foreground">No workbench sections available</div>
            ) : (
              <Tree>
                {sortedSections.map((section) => {
                  console.log("[ORIGIN] Rendering TreeSection:", section.id, section.label);
                  const content = typeof section.content === "function" ? section.content() : section.content;
                  console.log("[ORIGIN] TreeSection content:", section.id, content);
                  return (
                    <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                      {content}
                    </TreeSection>
                  );
                })}
              </Tree>
            )}
          </ScopedContent>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Workbench;
