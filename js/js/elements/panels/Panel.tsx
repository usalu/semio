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
import { useTranslation } from "react-i18next";
import { ScrollArea } from "../aggregation/ScrollArea";
import { Tree, TreeSection } from "../aggregation/Tree";
import { TreeStateProvider } from "../aggregation/TreeStateProvider";

export type ResizeSide = "left" | "right" | "top" | "bottom";

export interface PanelSection {
  id: string;
  content: ReactNode | (() => ReactNode);
  defaultOpen?: boolean;
  order?: number;
  translationParams?: Record<string, unknown>;
  actions?: Array<{
    icon: ReactNode;
    onClick: () => void;
    title?: string;
    id?: string;
  }>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: () => void;
}

export interface PanelProps {
  visible?: boolean;
  onSizeChange?: (size: number) => void;
  size?: number;
  resizeSide?: ResizeSide;
  zIndex?: 10 | 20 | 30 | 40;
  showBackground?: boolean;
  minSize?: number;
  maxSize?: number;
  sections?: PanelSection[];
  emptyMessage?: string;
  additionalContent?: ReactNode;
  footer?: ReactNode;
  className?: string;
  opacity?: number;
}

const Panel: FC<PanelProps> = ({
  visible = true,
  onSizeChange,
  size = 250,
  resizeSide = "right",
  zIndex = 20,
  showBackground = true,
  minSize = 150,
  maxSize = 500,
  sections = [],
  emptyMessage,
  additionalContent,
  footer,
  className = "",
  opacity = 1,
}) => {
  const { t } = useTranslation();
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  if (!visible) return null;
  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    const startPos = resizeSide === "top" || resizeSide === "bottom" ? e.clientY : e.clientX;
    const startSize = size;
    const handleMouseMove = (e: MouseEvent) => {
      const currentPos = resizeSide === "top" || resizeSide === "bottom" ? e.clientY : e.clientX;
      const delta = currentPos - startPos;
      let newSize: number;
      if (resizeSide === "right" || resizeSide === "bottom") {
        newSize = startSize + delta;
      } else {
        newSize = startSize - delta;
      }
      if (newSize >= minSize && newSize <= maxSize) {
        onSizeChange?.(newSize);
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
  const sortedSections = [...sections].sort((a, b) => (a.order || 0) - (b.order || 0));
  const borderClass =
    resizeSide === "left"
      ? isResizing || isResizeHovered
        ? "border-l-accent"
        : "border-l"
      : resizeSide === "right"
        ? isResizing || isResizeHovered
          ? "border-r-accent"
          : "border-r"
        : resizeSide === "top"
          ? isResizing || isResizeHovered
            ? "border-t-accent"
            : "border-t"
          : isResizing || isResizeHovered
            ? "border-b-accent"
            : "border-b";
  const containerClass = `h-full z-${zIndex} text-foreground border min-w-0 overflow-hidden ${showBackground ? "bg-panel" : ""} ${borderClass} ${className}`;
  const hasContent = sortedSections.length > 0 || additionalContent;
  const isHorizontal = resizeSide === "left" || resizeSide === "right";
  const sizeStyle = isHorizontal ? { width: `${size}px` } : { height: `${size}px` };
  const resizeHandleClass = isHorizontal ? `absolute top-0 bottom-0 ${resizeSide === "left" ? "left-0" : "right-0"} w-1 cursor-ew-resize` : `absolute left-0 right-0 ${resizeSide === "top" ? "top-0" : "bottom-0"} h-1 cursor-ns-resize`;
  return (
    <div className={containerClass} style={{ ...sizeStyle, opacity, transition: "opacity 150ms" }}>
      <ScrollArea className="h-full">
        <div className={`${className || "p-1"} overflow-hidden min-w-0`}>
          <TreeStateProvider>
            <Tree className="min-w-0 overflow-hidden">
              {additionalContent}
              {sortedSections.map((section) => {
                const Content = typeof section.content === "function" ? section.content : null;
                const sectionLabel = t(section.id, section.translationParams ?? {});
                return (
                  <TreeSection
                    key={section.id}
                    label={sectionLabel}
                    id={section.id}
                    defaultOpen={section.defaultOpen}
                    actions={section.actions}
                    onPointerEnter={section.onPointerEnter}
                    onPointerLeave={section.onPointerLeave}
                    onDoubleClick={section.onDoubleClick}
                  >
                    {Content ? <Content /> : section.content}
                  </TreeSection>
                );
              })}
              {!hasContent && emptyMessage && <div className="p-4 text-center text-muted-foreground">{emptyMessage}</div>}
            </Tree>
          </TreeStateProvider>
        </div>
        {footer}
      </ScrollArea>
      {onSizeChange && <div className={resizeHandleClass} onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />}
    </div>
  );
};

export default Panel;
