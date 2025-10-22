// #region Header

// Panel.tsx

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

import { FC, ReactNode, useState } from "react";
import { ScrollArea } from "../elements/aggregation/ScrollArea";
import { Tree, TreeSection } from "../elements/aggregation/Tree";
import { TreeStateProvider } from "../elements/aggregation/TreeStateProvider";
import { PanelKey, usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";
import { useActiveInteraction, useIsMobile } from "./store";

type ResizeSide = "left" | "right";

interface PanelProps extends ResizablePanelProps {
  panelId: PanelKey;
  resizeSide?: ResizeSide;
  zIndex?: 20 | 30;
  showBackground?: boolean;
  minWidth?: number;
  maxWidth?: number;
  scopeWrapper?: FC<{ children: ReactNode }>;
  emptyMessage?: string;
  additionalSections?: ReactNode;
  footer?: ReactNode;
  hideActiveInteractionOpacity?: (activeInteraction: string | null) => boolean;
}

const Panel: FC<PanelProps> = ({
  panelId,
  visible,
  onWidthChange,
  width,
  resizeSide = "right",
  zIndex = 20,
  showBackground = true,
  minWidth = 150,
  maxWidth = 500,
  scopeWrapper: ScopeWrapper,
  emptyMessage,
  additionalSections,
  footer,
  hideActiveInteractionOpacity,
}) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();
  const sections = usePanelSections(panelId);

  if (!visible) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const deltaX = e.clientX - startX;
      const newWidth = resizeSide === "right" ? startWidth + deltaX : startWidth - deltaX;
      if (newWidth >= minWidth && newWidth <= maxWidth) {
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

  const borderClass = resizeSide === "left" ? (isResizing || isResizeHovered ? "border-l-accent" : "border-l") : isResizing || isResizeHovered ? "border-r-accent" : "border-r";

  const shouldHideOpacity = hideActiveInteractionOpacity ? hideActiveInteractionOpacity(activeInteraction ?? null) : activeInteraction && !activeInteraction.startsWith(`${panelId}-`);

  const containerClass = `h-full z-${zIndex} text-foreground border min-w-0 overflow-hidden ${showBackground ? "bg-panel" : ""} ${borderClass}`;

  const hasContent = sortedSections.length > 0 || additionalSections;

  const content = (
    <TreeStateProvider>
      <Tree className="min-w-0 overflow-hidden">
        {additionalSections}
        {sortedSections.map((section) => (
          <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions} onPointerEnter={section.onPointerEnter} onPointerLeave={section.onPointerLeave} onDoubleClick={section.onDoubleClick}>
            {typeof section.content === "function" ? section.content() : section.content}
          </TreeSection>
        ))}
        {!hasContent && emptyMessage && <div className="p-4 text-center text-muted-foreground">{emptyMessage}</div>}
      </Tree>
    </TreeStateProvider>
  );

  const wrappedContent = ScopeWrapper ? <ScopeWrapper>{content}</ScopeWrapper> : content;

  return (
    <div className={containerClass} style={{ width: `${width}px`, opacity: shouldHideOpacity ? 0.1 : 1, transition: "opacity 150ms" }}>
      <ScrollArea className="h-full">
        <div className={`${isMobile ? "p-2" : "p-1"} overflow-hidden min-w-0`}>{wrappedContent}</div>
        {footer}
      </ScrollArea>
      <div
        className={`absolute top-0 bottom-0 ${resizeSide === "left" ? "left-0" : "right-0"} w-1 cursor-ew-resize`}
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  );
};

export default Panel;

