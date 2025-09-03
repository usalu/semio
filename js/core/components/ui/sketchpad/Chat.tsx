import { FC, useState } from "react";
import { ScrollArea } from "../ScrollArea";
import { Textarea } from "../Textarea";
import { Tree, TreeItem, TreeSection } from "../Tree";
import { ResizablePanelProps } from "./DesignEditor";

interface ChatProps extends ResizablePanelProps {}

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null;
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

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

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? "border-l-primary" : "border-l"}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            <TreeSection label="Conversation History" defaultOpen={true}>
              <TreeSection label="Design Session #1">
                <TreeItem label="How can I add a new piece?" />
                <TreeItem label="Can you help with connections?" />
              </TreeSection>
              <TreeSection label="Design Session #2">
                <TreeItem label="What are the available types?" />
              </TreeSection>
            </TreeSection>
            <TreeSection label="Quick Actions" defaultOpen={true}>
              <TreeItem label="Add random piece" />
              <TreeItem label="Connect all pieces" />
              <TreeItem label="Generate layout suggestions" />
            </TreeSection>
            <TreeSection label="Templates">
              <TreeSection label="Common Questions" defaultOpen={true}>
                <TreeItem label="How do I create a connection?" />
                <TreeItem label="How do I delete a piece?" />
                <TreeItem label="How do I change piece properties?" />
              </TreeSection>
              <TreeSection label="Advanced Workflows">
                <TreeItem label="Batch operations" />
                <TreeItem label="Complex layouts" />
                <TreeItem label="Export/Import" />
              </TreeSection>
            </TreeSection>
          </Tree>
        </div>
        <div className="p-4 border-t">
          <Textarea placeholder="Ask a question about the design..." />
        </div>
      </ScrollArea>
      <div className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Chat;
