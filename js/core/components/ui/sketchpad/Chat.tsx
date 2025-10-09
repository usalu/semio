import { FC, useState } from "react";
import { useTranslation } from "react-i18next";
import { ScrollArea } from "../ScrollArea";
import { Textarea } from "../Textarea";
import { Tree, TreeSection } from "../Tree";
import { usePanelSections } from "./Navbar";
import { ResizablePanelProps } from "./Sketchpad";

interface ChatProps extends ResizablePanelProps {}

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
  const { t } = useTranslation();
  if (!visible) return null;
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  const sections = usePanelSections("chat");

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
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            {sortedSections.map((section) => (
              <TreeSection key={section.id} label={section.label} defaultOpen={section.defaultOpen} actions={section.actions}>
                {section.content}
              </TreeSection>
            ))}
          </Tree>
        </div>
        <div className="p-4 border-t">
          <Textarea placeholder={t("chat.placeholder")} />
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Chat;
