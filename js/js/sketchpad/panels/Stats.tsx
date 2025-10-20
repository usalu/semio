// #region Header

// Stats.tsx

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

import { FC, useState } from "react";
import { ScrollArea } from "../../elements/aggregation/ScrollArea";
import { Tree, TreeSection } from "../../elements/aggregation/Tree";
import { TreeStateProvider } from "../../elements/aggregation/TreeStateProvider";
import { usePanelSections } from "../Navbar";
import { ResizablePanelProps } from "../Sketchpad";
import { useActiveInteraction, useIsMobile } from "../store";

interface StatsProps extends ResizablePanelProps {}

const Stats: FC<StatsProps> = ({ visible, onWidthChange, width }) => {
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();
  const sections = usePanelSections("stats");

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
            <div className="p-4 text-center text-muted-foreground">No stats sections available</div>
          ) : (
            <TreeStateProvider>
              <Tree>
                {sortedSections.map((section) => {
                  const content = typeof section.content === "function" ? section.content() : section.content;
                  return (
                    <TreeSection
                      key={section.id}
                      label={section.label}
                      defaultOpen={section.defaultOpen}
                      actions={section.actions}
                      onPointerEnter={section.onPointerEnter}
                      onPointerLeave={section.onPointerLeave}
                    >
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

export default Stats;
