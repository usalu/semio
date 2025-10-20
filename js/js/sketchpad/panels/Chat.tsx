// #region Header

// Chat.tsx

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
import { useTranslation } from "react-i18next";
import { ScrollArea } from "../../elements/aggregation/ScrollArea";
import { Tree, TreeSection } from "../../elements/aggregation/Tree";
import { TreeStateProvider } from "../../elements/aggregation/TreeStateProvider";
import { Textarea } from "../../elements/input/Textarea";
import { usePanelSections } from "../Navbar";
import { ResizablePanelProps } from "../Sketchpad";
import { useActiveInteraction, useIsMobile } from "../store";

interface ChatProps extends ResizablePanelProps {}

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const isMobile = useIsMobile();
  const activeInteraction = useActiveInteraction();

  const sections = usePanelSections("chat");

  if (!visible) return null;

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
      className={`h-full z-20 bg-panel text-foreground border
                ${isResizing || isResizeHovered ? "border-l-accent" : "border-l"}`}
      style={{ width: `${width}px`, opacity: activeInteraction && !activeInteraction.startsWith("chat-") ? 0.1 : 1, transition: "opacity 150ms" }}
    >
      <ScrollArea className="h-full">
        <div className={isMobile ? "p-2" : "p-1"}>
          <TreeStateProvider>
            <Tree>
              {sortedSections.map((section) => (
                <TreeSection
                  key={section.id}
                  label={section.label}
                  defaultOpen={section.defaultOpen}
                  actions={section.actions}
                  onPointerEnter={section.onPointerEnter}
                  onPointerLeave={section.onPointerLeave}
                >
                  {typeof section.content === "function" ? section.content() : section.content}
                </TreeSection>
              ))}
            </Tree>
          </TreeStateProvider>
        </div>
        <div className={`${isMobile ? "p-2" : "p-1"} border-t`}>
          <Textarea placeholder={t("chat.placeholder")} />
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Chat;
