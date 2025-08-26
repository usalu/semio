import { FC, useState } from "react";
import { useDesign } from "../../store";
import { ResizablePanelProps } from "./DesignEditor";

interface DiagramProps extends ResizablePanelProps {}

const Diagram: FC<DiagramProps> = ({ visible, onWidthChange, width }) => {
  const design = useDesign();
  const [isResizeHovered, setIsResizeHovered] = useState(false);
  const [isResizing, setIsResizing] = useState(false);

  if (!visible) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);

    const startX = e.clientX;
    const startWidth = width;

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth + (e.clientX - startX);
      if (newWidth >= 150 && newWidth <= 800) {
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
      className={`absolute top-4 left-4 bottom-4 z-10 bg-background-level-1 text-foreground border
                ${isResizing || isResizeHovered ? "border-r-primary" : "border-r"}`}
      style={{ width: `${width}px` }}
    >
      <div className="h-full w-full flex items-center justify-center text-muted-foreground">
        <div className="text-center">
          <h3 className="text-lg font-semibold mb-2">Diagram View</h3>
          <p className="text-sm">Design: {design.name}</p>
          <p className="text-sm">Pieces: {design.pieces?.length || 0}</p>
          <p className="text-sm">Connections: {design.connections?.length || 0}</p>
          <p className="text-xs mt-4 opacity-50">Simplified view - diagram visualization will be restored later</p>
        </div>
      </div>

      <div className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize" onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />
    </div>
  );
};

export default Diagram;
