// #region Header

// Resizable.tsx

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
import { GripVerticalIcon } from "lucide-react";
import * as React from "react";
import * as ResizablePrimitive from "react-resizable-panels";

import { cn } from "@semio/js/lib/utils";

function ResizablePanelGroup({ className, ...props }: React.ComponentProps<typeof ResizablePrimitive.PanelGroup>) {
  return <ResizablePrimitive.PanelGroup data-slot="resizable-panel-group" className={cn("flex h-full w-full data-[panel-group-direction=vertical]:flex-col", className)} {...props} />;
}

function ResizablePanel({ ...props }: React.ComponentProps<typeof ResizablePrimitive.Panel>) {
  return <ResizablePrimitive.Panel data-slot="resizable-panel" {...props} />;
}

function ResizableHandle({
  withHandle,
  className,
  onMouseDown: externalOnMouseDown,
  onMouseEnter: externalOnMouseEnter,
  onMouseLeave: externalOnMouseLeave,
  ...props
}: React.ComponentProps<typeof ResizablePrimitive.PanelResizeHandle> & {
  withHandle?: boolean;
}) {
  const [isHovered, setIsHovered] = React.useState(false);
  const [isDragging, setIsDragging] = React.useState(false);

  const handleMouseDown: React.MouseEventHandler<HTMLDivElement> = (e) => {
    setIsDragging(true);
    externalOnMouseDown?.(e as any);

    const handleMouseUp = () => {
      setIsDragging(false);
      document.removeEventListener("mouseup", handleMouseUp, true);
    };

    document.addEventListener("mouseup", handleMouseUp, true);
  };

  const handleMouseEnter: React.MouseEventHandler<HTMLDivElement> = (e) => {
    setIsHovered(true);
    externalOnMouseEnter?.(e as any);
  };

  const handleMouseLeave: React.MouseEventHandler<HTMLDivElement> = (e) => {
    if (!isDragging) {
      setIsHovered(false);
    }
    externalOnMouseLeave?.(e as any);
  };

  return (
    <ResizablePrimitive.PanelResizeHandle
      data-slot="resizable-handle"
      className={cn(
        "relative flex w-px items-center justify-center",
        "border-r", // Base border
        // Conditional hover/drag styling from the wrapper
        isDragging || isHovered ? "bg-primary border-primary" : "hover:border-primary",
        "before:absolute before:inset-y-0 before:-left-2 before:w-4 before:cursor-ew-resize", // Wider hit area
        "focus-visible:ring-ring focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none", // Changed outline-hidden to outline-none
        "after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2",
        "data-[panel-group-direction=vertical]:h-px data-[panel-group-direction=vertical]:w-full",
        "data-[panel-group-direction=vertical]:border-r-0 data-[panel-group-direction=vertical]:border-t", // Adjust border for vertical
        isDragging || isHovered
          ? "data-[panel-group-direction=vertical]:bg-primary data-[panel-group-direction=vertical]:border-primary" // Vertical hover/drag
          : "data-[panel-group-direction=vertical]:hover:border-primary", // Vertical hover
        "data-[panel-group-direction=vertical]:after:left-0 data-[panel-group-direction=vertical]:after:h-1 data-[panel-group-direction=vertical]:after:w-full data-[panel-group-direction=vertical]:after:-translate-y-1/2 data-[panel-group-direction=vertical]:after:translate-x-0",
        "data-[panel-group-direction=vertical]:before:inset-x-0 data-[panel-group-direction=vertical]:before:-top-2 data-[panel-group-direction=vertical]:before:h-4 data-[panel-group-direction=vertical]:before:w-full data-[panel-group-direction=vertical]:before:cursor-ns-resize",
        "[&[data-panel-group-direction=vertical]>div]:rotate-90",
        className,
      )}
      onMouseDown={handleMouseDown as any}
      onMouseEnter={handleMouseEnter as any}
      onMouseLeave={handleMouseLeave as any}
      {...props}
    >
      {withHandle && (
        <div
          className={cn(
            "z-10 flex h-4 w-3 items-center justify-center border bg-background",
            isDragging || isHovered ? "border-primary bg-primary" : "hover:border-primary", // Apply hover/drag to handle itself
          )}
        >
          <GripVerticalIcon className="size-2.5" />
        </div>
      )}
    </ResizablePrimitive.PanelResizeHandle>
  );
}

export { ResizableHandle, ResizablePanel, ResizablePanelGroup };
