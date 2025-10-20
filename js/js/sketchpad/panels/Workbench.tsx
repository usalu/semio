// #region Header

// Workbench.tsx

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
import { useParams } from "react-router";

import { ScrollArea } from "../../elements/aggregation/ScrollArea";
import { Tree, TreeSection } from "../../elements/aggregation/Tree";
import { TreeStateProvider } from "../../elements/aggregation/TreeStateProvider";
import { usePanelSections } from "../Navbar";
import { ResizablePanelProps } from "../Sketchpad";
import { DesignScopeProvider, KitScopeProvider, TypeScopeProvider, useActiveInteraction, useIsMobile, useNavigation } from "../store";

interface WorkbenchProps extends ResizablePanelProps {}

const ScopedContent: FC<{ children: ReactNode }> = ({ children }) => {
  const params = useParams();
  const navigation = useNavigation();
  const match = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types)\/([^/?]+))?/);
  const kit = params.kit ?? match?.[1];
  const design = params.design ?? (match?.[2] === "designs" ? match?.[3] : undefined);
  const type = params.type ?? (match?.[2] === "types" ? match?.[3] : undefined);
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
  const sections = usePanelSections("workbench");
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

  if (!visible) return null;

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
              <TreeStateProvider>
                <Tree>
                  {sortedSections.map((section) => {
                    const content = typeof section.content === "function" ? section.content() : section.content;
                    return (
                      <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions} onPointerEnter={section.onPointerEnter} onPointerLeave={section.onPointerLeave}>
                        {content}
                      </TreeSection>
                    );
                  })}
                </Tree>
              </TreeStateProvider>
            )}
          </ScopedContent>
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Workbench;
